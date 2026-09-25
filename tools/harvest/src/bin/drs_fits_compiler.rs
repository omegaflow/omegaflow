use omegaflow::archivar::drs_fits::{parse_series, write_bin};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::drs_series;

const CDN_TAG: &str = "heasarc.gsfc.nasa.gov";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("drs_fits_compiler: --out <path> absent — the output path is never silent");
            std::process::exit(1);
        }
    };
    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
    let bytes = match (input, url) {
        (Some(path), _) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("drs_fits_compiler: read {path}: {e} — the granule stays unread");
                std::process::exit(1);
            }
        },
        (None, Some(route)) => match fetch_raw_bytes(&route) {
            Some(b) => b,
            None => {
                eprintln!("drs_fits_compiler: fetch void ({route})");
                std::process::exit(1);
            }
        },
        (None, None) => {
            eprintln!("drs_fits_compiler: --input <file> or --url <https> absent — refused");
            std::process::exit(1);
        }
    };
    let rows = match drs_series(&bytes) {
        Some(v) => v,
        None => {
            eprintln!(
                "drs_fits_compiler: the granule carries no SCI_SCIENCE_1Hz table with ESA00001/ESA00002 (UTC) and DST11077-079 / DST11083-085 (LTP1/LTP2 force) — the bin stays unwritten"
            );
            std::process::exit(1);
        }
    };
    let mut records: Vec<[f64; 3]> = Vec::with_capacity(rows.len());
    let mut epoch: Option<f64> = None;
    let mut skipped = 0usize;
    for (t, dg) in rows {
        if t.is_finite() && dg.iter().all(|v| v.is_finite()) {
            if epoch.is_none() {
                epoch = Some(t);
            }
            records.push(dg);
        } else {
            skipped += 1;
        }
    }
    let Some(epoch) = epoch.filter(|e| *e > 0.0) else {
        eprintln!(
            "drs_fits_compiler: no finite differential acceleration — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    };
    let bin = write_bin(&records, epoch);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("drs_fits_compiler: write {out} void");
        std::process::exit(1);
    }
    match parse_series(&bin) {
        Some(parsed) => {
            let mut gmax = 0.0f64;
            for r in &records {
                let m = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
                if m > gmax {
                    gmax = m;
                }
            }
            eprintln!(
                "drs_fits: {} rows, {} skipped, |dg| bis {gmax:.3e} m/s^2, t0 {epoch:.3e} s unix, {} B -> {out} (roundtrip: {} series points)",
                records.len(),
                skipped,
                bin.len(),
                parsed.len()
            );
        }
        None => {
            eprintln!(
                "drs_fits_compiler: {out}: roundtrip parse void — the asset stays unverified"
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_TAG, &out) {
        eprintln!("drs_fits_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}
