use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_IERS_LOD, GeoRec, MAGIC_IERS_LOD, parse_bin, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::LeapSeconds;

const NETLOC: &str = "datacenter.iers.org";
const URL: &str = "https://datacenter.iers.org/data/latestVersion/EOP_20_C04_one_file_1962-now.txt";
const DEFAULT_OUT: &str = "iers_eop_c04_lod.bin";
const MJD_UNIX_EPOCH: f64 = 40587.0;
const LOD_COLUMN: usize = 12;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_eopc04(text: &str, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let mut records = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = trimmed.split_whitespace().collect();
        if cols.len() <= LOD_COLUMN {
            continue;
        }
        let Ok(mjd) = cols[4].parse::<f64>() else {
            continue;
        };
        if !mjd.is_finite() {
            continue;
        }
        let Ok(lod) = cols[LOD_COLUMN].parse::<f64>() else {
            continue;
        };
        if !lod.is_finite() {
            continue;
        }
        let unix = (mjd - MJD_UNIX_EPOCH) * 86400.0;
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        records.push(GeoRec {
            t,
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: lod,
            comp: COMP_IERS_LOD,
            station: 0,
        });
    }
    records
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let url = match arg_value(args, "--url") {
        Some(v) => v,
        None => URL.to_string(),
    };
    let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let text = String::from_utf8_lossy(&bytes);
    let records = parse_eopc04(&text, &lsk);
    if records.is_empty() {
        return Err(format!(
            "{url}: no measured LOD day left the harvest ({} B) — the bin stays unwritten (0 honored)",
            bytes.len()
        ));
    }
    let bin = write_bin(MAGIC_IERS_LOD, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(MAGIC_IERS_LOD, &bin) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} EOP C04 LOD days ({url}), roundtrip parses",
                parsed.len()
            );
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("iers_eop_c04_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lsk() -> LeapSeconds {
        LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(0.0, -1e12)],
        }
    }

    #[test]
    fn parses_the_measured_c04_lod_column() {
        let text = "# EOP (IERS) 20 C04 TIME SERIES\n\
# YR  MM  DD  HH       MJD        x(\")        y(\")  UT1-UTC(s)       dX(\")      dY(\")       xrt(\")      yrt(\")      LOD(s)\n\
1962   1   1   0  37665.00   -0.012700    0.213000   0.0326338    0.000000    0.000000    0.000000    0.000000   0.0017230\n\
1962   1   2   0  37666.00   -0.015900    0.214100   0.0320547    0.000000    0.000000    0.000000    0.000000   0.0016690\n";
        let records = parse_eopc04(text, &lsk());
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].val, 0.0017230);
        assert_eq!(records[1].val, 0.0016690);
        let expect = (37665.0 - MJD_UNIX_EPOCH) * 86400.0 + 32.184 - 946728000.0;
        assert!((records[0].t - expect).abs() < 1e-6);
    }

    #[test]
    fn header_lines_read_no_record() {
        let records = parse_eopc04("# a\n\n", &lsk());
        assert!(records.is_empty());
    }

    #[test]
    fn bin_roundtrip_carries_the_lod_record() {
        let records = vec![GeoRec {
            t: -4_607_323_167.816,
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 0.0017230,
            comp: COMP_IERS_LOD,
            station: 0,
        }];
        let bytes = write_bin(MAGIC_IERS_LOD, &records);
        let parsed = parse_bin(MAGIC_IERS_LOD, &bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].val, 0.0017230);
        assert_eq!(parsed[0].comp, COMP_IERS_LOD);
    }
}
