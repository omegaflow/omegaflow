use omegaflow::archivar::{allwise, uws};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "irsa.ipac.caltech.edu";
const DEFAULT_ASYNC_BASE: &str = "https://irsa.ipac.caltech.edu/TAP/async";
const SRC_QUERY: &str = "SELECT TOP 5000 ra,dec,w1mpro,w2mpro,w3mpro,w4mpro,w3snr,w4snr FROM allsky_4band_p3as_psd WHERE w1mpro IS NOT NULL";
const DEFAULT_OUT: &str = "data/irsa.ipac.caltech.edu/allwise_psd.bin";

const MAGIC_ALWS: [u8; 4] = *b"ALWS";
const HEADER_BYTES: usize = 8;
const REC_BYTES: usize = 65;
const MASK_W1: u8 = 1 << 0;
const MASK_W2: u8 = 1 << 1;
const MASK_W3: u8 = 1 << 2;
const MASK_W4: u8 = 1 << 3;
const MASK_W3SNR: u8 = 1 << 4;
const MASK_W4SNR: u8 = 1 << 5;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn f64_at(data: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(data.get(off..off + 8)?.try_into().ok()?))
}

fn encode_rec(r: &allwise::AllwisePsd) -> [u8; REC_BYTES] {
    let mut rec = [0u8; REC_BYTES];
    rec[0..8].copy_from_slice(&r.ra.to_le_bytes());
    rec[8..16].copy_from_slice(&r.dec.to_le_bytes());
    let mut mask = 0u8;
    let cells: [(Option<f64>, u8, usize); 6] = [
        (r.w1mpro, MASK_W1, 16),
        (r.w2mpro, MASK_W2, 24),
        (r.w3mpro, MASK_W3, 32),
        (r.w4mpro, MASK_W4, 40),
        (r.w3snr, MASK_W3SNR, 48),
        (r.w4snr, MASK_W4SNR, 56),
    ];
    for (value, bit, off) in cells {
        match value {
            Some(v) => {
                mask |= bit;
                rec[off..off + 8].copy_from_slice(&v.to_le_bytes());
            }
            None => {
                rec[off..off + 8].copy_from_slice(&0.0f64.to_le_bytes());
            }
        }
    }
    rec[64] = mask;
    rec
}

fn decode_rec(data: &[u8]) -> Option<allwise::AllwisePsd> {
    if data.len() < REC_BYTES {
        return None;
    }
    let ra = f64_at(data, 0)?;
    let dec = f64_at(data, 8)?;
    if !ra.is_finite() || !dec.is_finite() {
        return None;
    }
    let mask = data[64];
    let cell = |off: usize, bit: u8| -> Option<f64> {
        if mask & bit == 0 {
            return None;
        }
        f64_at(data, off).filter(|v| v.is_finite())
    };
    Some(allwise::AllwisePsd {
        ra,
        dec,
        w1mpro: cell(16, MASK_W1),
        w2mpro: cell(24, MASK_W2),
        w3mpro: cell(32, MASK_W3),
        w4mpro: cell(40, MASK_W4),
        w3snr: cell(48, MASK_W3SNR),
        w4snr: cell(56, MASK_W4SNR),
    })
}

fn write_bin(records: &[allwise::AllwisePsd]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC_ALWS);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(&encode_rec(r));
    }
    out
}

fn read_bin(data: &[u8]) -> Option<Vec<allwise::AllwisePsd>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC_ALWS {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        out.push(decode_rec(data.get(base..base + REC_BYTES)?)?);
    }
    Some(out)
}

fn fetch_result(args: &[String], poll_secs: u64) -> Option<String> {
    match arg_value(args, "--job") {
        Some(job) => {
            eprintln!("uws job: {job}");
            match uws::uws_poll(&job, uws::UWS_POLL_STEP_S, poll_secs, &[]) {
                Some(phase) if phase == "COMPLETED" => {}
                Some(phase) => {
                    eprintln!("uws job phase {phase} — the query stays unharvested");
                    return None;
                }
                None => return None,
            }
            match uws::uws_result_bytes(&job, &[]) {
                Some(bytes) => String::from_utf8(bytes).ok(),
                None => {
                    eprintln!("uws result void at {job} — the query stays unharvested");
                    None
                }
            }
        }
        None => {
            let async_base = match arg_value(args, "--async-base") {
                Some(v) => v,
                None => DEFAULT_ASYNC_BASE.to_string(),
            };
            let query = match arg_value(args, "--query") {
                Some(v) => v,
                None => SRC_QUERY.to_string(),
            };
            let params = [
                ("REQUEST", "doQuery"),
                ("LANG", "ADQL"),
                ("FORMAT", "votable/td"),
                ("QUERY", query.as_str()),
            ];
            let job = match uws::uws_submit(&async_base, &params, &[]) {
                Some(j) => j,
                None => {
                    eprintln!("uws submit void at {async_base} — the query stays unharvested");
                    return None;
                }
            };
            eprintln!("uws job: {job}");
            if uws::uws_phase(&job, &[]).as_deref() == Some("PENDING")
                && !uws::uws_post_phase(&job, "RUN", &[])
            {
                eprintln!("uws run post void at {job}/phase — the job may stay PENDING");
            }
            let body = match uws::uws_poll(&job, uws::UWS_POLL_STEP_S, poll_secs, &[]) {
                Some(phase) if phase == "COMPLETED" => match uws::uws_result_bytes(&job, &[]) {
                    Some(bytes) => String::from_utf8(bytes).ok(),
                    None => {
                        eprintln!("uws result void at {job} — the query stays unharvested");
                        None
                    }
                },
                Some(phase) => {
                    eprintln!("uws job phase {phase} — the query stays unharvested");
                    None
                }
                None => None,
            };
            if !uws::uws_delete(&job, &[]) {
                eprintln!("uws delete void at {job} — the job expires on the service");
            }
            body
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent()
        && !parent.as_os_str().is_empty()
        && std::fs::create_dir_all(parent).is_err()
    {
        eprintln!("allwise_tap_compiler: create parent dir of {out_path} void");
        std::process::exit(1);
    }
    let poll_secs = match arg_value(&args, "--poll") {
        Some(s) => match s.parse::<u64>() {
            Ok(v) if v > 0 => v,
            _ => {
                eprintln!("allwise_tap_compiler: --poll {s} is no positive count — refused");
                std::process::exit(1);
            }
        },
        None => uws::UWS_POLL_BUDGET_S,
    };
    let body = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("allwise_tap_compiler: read {path}: {e}");
                std::process::exit(1);
            }
        },
        None => match fetch_result(&args, poll_secs) {
            Some(b) => b,
            None => std::process::exit(1),
        },
    };
    if body.is_empty() {
        eprintln!(
            "allwise_tap_compiler: the result body is void — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let sources = match allwise::parse_votable(&body) {
        Some(s) => s,
        None => {
            eprintln!(
                "allwise votable: {} B carry no measured row — the bin stays unwritten (0 honored)",
                body.len()
            );
            std::process::exit(1);
        }
    };
    let bin = write_bin(&sources);
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("allwise_tap_compiler: write {out_path} void");
        std::process::exit(1);
    }
    match read_bin(&bin) {
        Some(parsed) if parsed.len() == sources.len() => {
            eprintln!(
                "allwise psd: {} rows, {} B -> {out_path} (roundtrip parses)",
                parsed.len(),
                bin.len()
            );
        }
        _ => {
            eprintln!(
                "allwise_tap_compiler: {out_path}: roundtrip parse void — the asset stays unverified"
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("allwise_tap_compiler: upload {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<allwise::AllwisePsd> {
        vec![
            allwise::AllwisePsd {
                ra: 189.5907715,
                dec: -50.3575314,
                w1mpro: Some(13.615),
                w2mpro: Some(13.666),
                w3mpro: Some(12.826),
                w4mpro: Some(9.617),
                w3snr: Some(0.7),
                w4snr: Some(0.0),
            },
            allwise::AllwisePsd {
                ra: 1.5,
                dec: 2.5,
                w1mpro: Some(16.2),
                w2mpro: None,
                w3mpro: None,
                w4mpro: None,
                w3snr: None,
                w4snr: None,
            },
        ]
    }

    #[test]
    fn bin_roundtrip_carries_all_rows_and_absent_masks() {
        let recs = fixture();
        let bytes = write_bin(&recs);
        let parsed = read_bin(&bytes).expect("roundtrip parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra, recs[0].ra);
        assert_eq!(parsed[0].w4snr, Some(0.0));
        assert_eq!(parsed[1].w2mpro, None);
        assert_eq!(parsed[1].w1mpro, Some(16.2));
    }

    #[test]
    fn bin_rejects_bad_magic_and_truncation() {
        assert!(read_bin(b"XXXX").is_none());
        let bytes = write_bin(&fixture());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert!(read_bin(&bad).is_none());
    }

    #[test]
    fn bin_rejects_nonfinite_cell_behind_a_set_mask() {
        let bytes = write_bin(&fixture());
        let mut bad = bytes;
        let off = HEADER_BYTES + 24;
        bad[off..off + 8].copy_from_slice(&f64::INFINITY.to_le_bytes());
        assert!(read_bin(&bad).is_none());
    }
}
