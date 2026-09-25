use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://atmos.nmsu.edu/PDS/data/";
const VOLUMES: [&str; 4] = ["VXRS_1101", "VXRS_1102", "VXRS_1103", "VXRS_1104"];
const ODF_DIR: &str = "/ODF/";
const UNIX_1950_OFFSET: f64 = 631152000.0;

fn files_of(volume: &str) -> Vec<String> {
    let dir = format!("{BASE}{volume}{ODF_DIR}");
    let Some(bytes) = fetch_raw_bytes(&dir) else {
        eprintln!("{volume}: odf dir listing fetch void ({dir})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{volume}: odf dir listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for token in text.split("href=\"") {
        let Some(end) = token.find('"') else {
            continue;
        };
        let name = &token[..end];
        if name.to_ascii_uppercase().ends_with(".ODF") {
            out.push(name.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

fn merge_bytes(
    label: &str,
    bytes: &[u8],
    lsk: &omegaflow::archivar::LeapSeconds,
    merged: &mut Vec<[f64; 9]>,
) {
    let Some(recs) = odf::parse_odf(bytes) else {
        eprintln!("{label}: parse void — {} B", bytes.len());
        return;
    };
    let mut kept = 0usize;
    let mut skipped = 0usize;
    for r in &recs {
        let doppler = (11..=14).contains(&r.data_type);
        if !r.valid || !doppler {
            skipped += 1;
            continue;
        }
        let unix = r.t_since_1950 - UNIX_1950_OFFSET;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            skipped += 1;
            continue;
        };
        merged.push([
            tdb,
            r.observable_hz,
            r.ref_hz,
            r.dss_rx as f64,
            r.dss_tx as f64,
            r.data_type as f64,
            r.downlink_band as f64,
            r.scid as f64,
            r.compression_s,
        ]);
        kept += 1;
    }
    eprintln!(
        "{label}: {} orbit records, {kept} kept ({skipped} discarded)",
        recs.len()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let inputs: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == "--input")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 9]> = Vec::new();
    if inputs.is_empty() {
        for volume in VOLUMES {
            let rels = files_of(volume);
            eprintln!("{volume}/ODF: {} files", rels.len());
            for rel in rels {
                let url = format!("{BASE}{volume}{ODF_DIR}{rel}");
                let Some(bytes) = fetch_raw_bytes(&url) else {
                    eprintln!("{volume}/{rel}: fetch void ({url})");
                    continue;
                };
                merge_bytes(&format!("{volume}/{rel}"), &bytes, &lsk, &mut merged);
            }
        }
    } else {
        for path in &inputs {
            match std::fs::read(path) {
                Ok(bytes) => merge_bytes(path, &bytes, &lsk, &mut merged),
                Err(e) => eprintln!("read {path} returned void: {e}"),
            }
        }
    }
    if merged.is_empty() {
        eprintln!(
            "no Venus Express VeRa ODF orbit samples — the series stays unwritten (0 honored)"
        );
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/atmos.nmsu.edu/vex_odf.bin";
    std::fs::create_dir_all("data/atmos.nmsu.edu").ok();
    let bin = odf::write_podf_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let mut stations: Vec<i64> = parsed.iter().map(|r| r[3] as i64).collect();
            stations.sort_unstable();
            stations.dedup();
            let mut dts: Vec<i64> = parsed.iter().map(|r| r[5] as i64).collect();
            dts.sort_unstable();
            dts.dedup();
            eprintln!(
                "{out}: {} orbit samples (tdb {}..{}), stations {stations:?}, data_type {dts:?}, {} B — roundtrip parses",
                parsed.len(),
                d0[0],
                d1[0],
                bin.len()
            );
        }
        None => eprintln!("{out}: roundtrip parse void — the series stays unverified"),
    }
    if ci_mode && !upload_release("atmos.nmsu.edu", out) {
        std::process::exit(1);
    }
}
