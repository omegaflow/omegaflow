use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::drs_differential_acceleration;

const CDN_TAG: &str = "heasarc.gsfc.nasa.gov";
const MAGIC: [u8; 4] = *b"DRSF";
const HEADER_BYTES: usize = 8;
const REC_BYTES: usize = 24;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn write_bin(records: &[[f64; 3]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(&r[0].to_le_bytes());
        out.extend_from_slice(&r[1].to_le_bytes());
        out.extend_from_slice(&r[2].to_le_bytes());
    }
    out
}

fn read_bin(data: &[u8]) -> Option<Vec<[f64; 3]>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(data.get(o..o + 8)?.try_into().ok()?))
        };
        let gx = f64_at(base)?;
        let gy = f64_at(base + 8)?;
        let gz = f64_at(base + 16)?;
        if !gx.is_finite() || !gy.is_finite() || !gz.is_finite() {
            return None;
        }
        out.push([gx, gy, gz]);
    }
    Some(out)
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
        (None, Some(route)) => match fetch_raw_bytes(&route, 604800) {
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
    let dg = match drs_differential_acceleration(&bytes) {
        Some(v) => v,
        None => {
            eprintln!(
                "drs_fits_compiler: the granule carries no SCI_SCIENCE_1Hz differential acceleration — the bin stays unwritten"
            );
            std::process::exit(1);
        }
    };
    let mut records: Vec<[f64; 3]> = Vec::with_capacity(dg.len());
    let mut skipped = 0usize;
    for r in dg {
        if r.iter().all(|v| v.is_finite()) {
            records.push(r);
        } else {
            skipped += 1;
        }
    }
    if records.is_empty() {
        eprintln!(
            "drs_fits_compiler: no finite differential acceleration — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let bin = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("drs_fits_compiler: write {out} void");
        std::process::exit(1);
    }
    match read_bin(&bin) {
        Some(parsed) => {
            let mut gmax = 0.0f64;
            for r in &parsed {
                let m = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt();
                if m > gmax {
                    gmax = m;
                }
            }
            eprintln!(
                "drs_fits: {} rows, {} skipped, |dg| bis {gmax:.3e} m/s^2, {} B -> {out} (roundtrip parses)",
                parsed.len(),
                skipped,
                bin.len()
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
