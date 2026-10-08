use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_CSES_HPM_FG2_X, COMP_CSES_HPM_FG2_Y, COMP_CSES_HPM_FG2_Z, GeoRec, MAGIC_CSES_HPM,
    verify_bin, write_bin,
};
use omegaflow::archivar::hdf5::Hdf5File;
use omegaflow::archivar::rinex::{ecef_to_geodetic, receiver_ellipsoid_for_compiler};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::SPECTRAL_NO_BAND;

const NETLOC: &str = "scidb.cn";
const FILETREE_URL: &str = "https://www.scidb.cn/api/sdb-filetree-service/getAllUrl?dataSetId=30660a0fa4f04312b49689c3365fb474&type=personal&version=V1&global=en";
const DEFAULT_FILE_ID: &str = "6398427cbae2f1393c118b52";
const DOWNLOAD: &str = "https://download.scidb.cn/download?fileId=";
const DAY_S: f64 = 86400.0;
const BIN: &str = "cses_hpm_compiler";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn download_url(file_id: &str) -> String {
    format!("{DOWNLOAD}{file_id}")
}

fn utc_field_to_unix(v: i64) -> Option<f64> {
    if v <= 0 {
        return None;
    }
    let secs = v / 1000;
    let millis = (v % 1000) as f64;
    if secs < 10_000_000_000_000 {
        return None;
    }
    let yyyy = secs / 10_000_000_000;
    let mm = (secs / 100_000_000) % 100;
    let dd = (secs / 1_000_000) % 100;
    let hh = (secs / 10_000) % 100;
    let mi = (secs / 100) % 100;
    let ss = secs % 100;
    if !(1..=12).contains(&mm)
        || !(1..=31).contains(&dd)
        || !(0..=23).contains(&hh)
        || !(0..=59).contains(&mi)
        || !(0..=60).contains(&ss)
    {
        return None;
    }
    let days = days_from_civil(yyyy, mm, dd)?;
    Some(days as f64 * DAY_S + (hh * 3600 + mi * 60 + ss) as f64 + millis / 1000.0)
}

fn parse_utc_text(s: &str) -> Option<f64> {
    let s = s.trim_matches(|c: char| c == '\0' || c.is_whitespace());
    let (head, frac) = match s.split_once('.') {
        Some((h, f)) => (h, f),
        None => (s, ""),
    };
    let secs: i64 = head.parse().ok()?;
    let mut frac: String = frac.chars().filter(|c| c.is_ascii_digit()).collect();
    while frac.len() < 3 {
        frac.push('0');
    }
    frac.truncate(3);
    let millis: i64 = frac.parse().ok()?;
    utc_field_to_unix(secs * 1000 + millis)
}

fn read_utc_text(file: &Hdf5File, name: &str) -> Result<Vec<Option<f64>>, String> {
    let (_, ds, _) = file
        .dataset(name)
        .map_err(|n| format!("{name} absent: {n:?}"))?;
    let raw = file
        .read_dataset(name)
        .map_err(|n| format!("{name} unread: {n:?}"))?;
    let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    if count == 0 || raw.is_empty() {
        return Err(format!("{name} carries no element"));
    }
    let stride = raw.len() / count;
    if stride == 0 {
        return Err(format!("{name} carries no byte per element"));
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let chunk = &raw[i * stride..(i + 1) * stride];
        let text = String::from_utf8_lossy(chunk);
        out.push(parse_utc_text(&text));
    }
    Ok(out)
}

fn median_positive(mut deltas: Vec<f64>) -> f64 {
    if deltas.is_empty() {
        return SPECTRAL_NO_BAND;
    }
    deltas.sort_by(f64::total_cmp);
    deltas[deltas.len() / 2]
}

fn push_rec(
    out: &mut Vec<GeoRec>,
    t: f64,
    lat: f64,
    lon: f64,
    alt_m: f64,
    cadence: f64,
    comp: u32,
    val: f64,
) {
    out.push(GeoRec {
        t,
        lat,
        lon,
        alt: alt_m,
        freq: SPECTRAL_NO_BAND,
        bin_width: cadence,
        val,
        comp,
        station: 0,
    });
}

fn assemble_fg2(
    utc: &[Option<f64>],
    xyz: &[f64],
    fg2: &[f32],
    ellipsoid: Option<(f64, f64)>,
) -> Result<Vec<GeoRec>, String> {
    let np = utc.len();
    if fg2.len() < np * 3 {
        return Err(format!("FG2 carries {} values, {np} x 3 needed", fg2.len()));
    }
    if xyz.len() < np * 3 {
        return Err(format!(
            "SatPos carries {} values, {np} x 3 needed",
            xyz.len()
        ));
    }
    let deltas: Vec<f64> = utc
        .windows(2)
        .filter_map(|w| match (w[0], w[1]) {
            (Some(a), Some(b)) => {
                let d = b - a;
                if d.is_finite() && d > 0.0 {
                    Some(d)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();
    let cadence = median_positive(deltas);
    let comps = [
        COMP_CSES_HPM_FG2_X,
        COMP_CSES_HPM_FG2_Y,
        COMP_CSES_HPM_FG2_Z,
    ];
    let mut out = Vec::with_capacity(np * 3);
    for p in 0..np {
        let Some(t) = utc[p] else { continue };
        let Some((a, e2)) = ellipsoid else { continue };
        let (la, lo, al) = match ecef_to_geodetic(xyz[p * 3], xyz[p * 3 + 1], xyz[p * 3 + 2], a, e2)
        {
            Some((la, lo, al))
                if la.is_finite()
                    && (-90.0..=90.0).contains(&la)
                    && lo.is_finite()
                    && al.is_finite()
                    && al > 0.0 =>
            {
                (la, lo, al)
            }
            _ => continue,
        };
        for c in 0..3 {
            let v = fg2[p * 3 + c] as f64;
            if v.is_finite() {
                push_rec(&mut out, t, la, lo, al, cadence, comps[c], v);
            }
        }
    }
    Ok(out)
}

fn parse_hpm(bytes: &[u8], ellipsoid: Option<(f64, f64)>) -> Result<Vec<GeoRec>, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("HDF5 parse: {n:?}"))?;
    let fg2 = file
        .read_f32_dataset("FG2")
        .map_err(|n| format!("FG2 unread: {n:?}"))?;
    let utc = read_utc_text(&file, "UTCTime")?;
    let sat_pos = file
        .read_f32_dataset("SatPos")
        .map_err(|n| format!("SatPos unread: {n:?}"))?;
    let xyz: Vec<f64> = sat_pos.iter().map(|v| *v as f64).collect();
    assemble_fg2(&utc, &xyz, &fg2, ellipsoid)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("cses_hpm_compiler: --out <path> absent — the output path is never silent");
            std::process::exit(1);
        }
    };

    let bytes = if let Some(path) = arg_value(&args, "--input") {
        match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("cses_hpm_compiler: read {path}: {e} — the file stays unread");
                std::process::exit(1);
            }
        }
    } else {
        let url = match arg_value(&args, "--url") {
            Some(u) => u,
            None => match arg_value(&args, "--file-id") {
                Some(id) => download_url(&id),
                None => download_url(DEFAULT_FILE_ID),
            },
        };
        match fetch_raw_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("cses_hpm_compiler: fetch void ({url}); filetree {FILETREE_URL}");
                std::process::exit(1);
            }
        }
    };

    let mut records = match parse_hpm(&bytes, receiver_ellipsoid_for_compiler(BIN)) {
        Ok(r) => r,
        Err(msg) => {
            eprintln!("cses_hpm_compiler: {msg} — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    if records.is_empty() {
        eprintln!(
            "cses_hpm_compiler: {} B carry no placeable HPM sample — the bin stays unwritten (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }

    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bin = write_bin(MAGIC_CSES_HPM, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("cses_hpm_compiler: write {out} void");
        std::process::exit(1);
    }
    match verify_bin(MAGIC_CSES_HPM, &bin) {
        Some(n) => eprintln!(
            "cses_hpm: {} records, {} B -> {out} (verified {})",
            records.len(),
            bin.len(),
            n
        ),
        None => {
            eprintln!("cses_hpm_compiler: {out}: verify void — the asset stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("cses_hpm_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_text_reads_millisecond_stamp() {
        let t = parse_utc_text("20180811214154.006").expect("stamp parses");
        let expect = days_from_civil(2018, 8, 11).expect("civil day") as f64 * DAY_S
            + 21.0 * 3600.0
            + 41.0 * 60.0
            + 54.0
            + 0.006;
        assert!((t - expect).abs() < 1e-6);
    }

    #[test]
    fn assemble_fg2_packs_three_components() {
        let epoch = days_from_civil(2018, 8, 11).expect("civil day") as f64 * DAY_S + 21.0 * 3600.0;
        let utc = [Some(epoch), Some(epoch + 1.0 / 60.0)];
        let xyz = [
            164443.0, 2390484.0, -6447972.0, 167864.0, 2396892.0, -6445496.0,
        ];
        let fg2 = [1.0f32, 2.0, 3.0, -4.0, 5.0, 6.0];
        let recs = assemble_fg2(&utc, &xyz, &fg2, Some((6_378_137.0, 6.694_379_990_14e-3)))
            .expect("assembles");
        assert_eq!(recs.len(), 6);
        assert_eq!(recs[0].comp, COMP_CSES_HPM_FG2_X);
        assert_eq!(recs[1].comp, COMP_CSES_HPM_FG2_Y);
        assert_eq!(recs[2].comp, COMP_CSES_HPM_FG2_Z);
        assert!((recs[0].bin_width - 1.0 / 60.0).abs() < 1e-9);
        assert!((recs[0].alt - 519_000.0).abs() < 20_000.0);
    }

    #[test]
    fn hpm_bin_roundtrips_through_write_verify_parse() {
        let recs = vec![GeoRec {
            t: 1_533_974_514.0,
            lat: -64.74,
            lon: 81.15,
            alt: 517_500.0,
            freq: SPECTRAL_NO_BAND,
            bin_width: 1.0 / 60.0,
            val: -9581.369,
            comp: COMP_CSES_HPM_FG2_X,
            station: 0,
        }];
        let bin = write_bin(MAGIC_CSES_HPM, &recs);
        assert_eq!(verify_bin(MAGIC_CSES_HPM, &bin), Some(1));
        let parsed = omegaflow::archivar::geo::parse_bin(MAGIC_CSES_HPM, &bin).expect("parses");
        assert_eq!(parsed.len(), 1);
        assert!((parsed[0].val + 9581.369).abs() < 1e-9);
        assert_eq!(parsed[0].comp, COMP_CSES_HPM_FG2_X);
        assert!(verify_bin(omegaflow::archivar::geo::MAGIC_CSES_SCM, &bin).is_none());
    }
}
