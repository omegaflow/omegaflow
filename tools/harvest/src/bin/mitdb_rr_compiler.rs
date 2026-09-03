use omegaflow::archivar::mitdb::parse_atr;
use omegaflow::cdn::upload_release;
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://physionet.org/files/mitdb/1.0.0/";
const CDN_RELEASE: &str = "physionet.org";
const MAGIC: [u8; 4] = *b"RRB1";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out_path = arg_value(&args, "--out").unwrap_or_else(|| "mitdb_rr.bin".to_string());
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let Some(records_text) = fetch(&format!("{}RECORDS", BASE)) else {
        eprintln!("RECORDS absent — the compiler stays still (0 honored)");
        return;
    };
    let mut records: Vec<String> = String::from_utf8_lossy(&records_text)
        .lines()
        .map(|l| l.trim_end_matches('\r').trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    records.sort();
    records.dedup();
    if records.is_empty() {
        eprintln!("RECORDS carries no entries (0 honored)");
        return;
    }

    let records = Arc::new(records);
    let all = Mutex::new(Vec::<(f64, f64, u32)>::new());
    let next = AtomicUsize::new(0);
    let skipped = AtomicI64::new(0);
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let records = Arc::clone(&records);
            let all = &all;
            let next = &next;
            let skipped = &skipped;
            s.spawn(move || {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    if i >= records.len() {
                        break;
                    }
                    let rec = &records[i];
                    let Some(atr_bytes) = fetch(&format!("{}{}.atr", BASE, rec)) else {
                        skipped.fetch_add(1, Ordering::SeqCst);
                        continue;
                    };
                    let Some(series) = parse_atr(&atr_bytes, 360) else {
                        skipped.fetch_add(1, Ordering::SeqCst);
                        continue;
                    };
                    let mut out = all.lock().unwrap_or_else(|e| e.into_inner());
                    for (epoch, rr) in series {
                        out.push((epoch, rr, (i + 1) as u32));
                    }
                }
            });
        }
    });

    let mut records_out = all.into_inner().unwrap_or_else(|e| e.into_inner());
    records_out.sort_by(|a, b| a.2.cmp(&b.2).then(a.0.total_cmp(&b.0)));
    let n_beats = records_out.len();
    let bytes = write_bin(&records_out);
    if let Err(e) = std::fs::write(&out_path, &bytes) {
        eprintln!("write {}: {}", out_path, e);
        return;
    }
    let n_recs = records.len();
    let mean_rr = if n_beats > 0 {
        records_out.iter().map(|(_, r, _)| r).sum::<f64>() / n_beats as f64
    } else {
        f64::NAN
    };
    eprintln!(
        "{}: {} records, {} beats (RR mean {:.3} s), {} skipped, {} bytes written",
        out_path,
        n_recs,
        n_beats,
        mean_rr,
        skipped.load(Ordering::SeqCst),
        bytes.len()
    );

    if args.iter().any(|a| a == "--ci-mode") {
        let ok = upload_release(CDN_RELEASE, &out_path);
        eprintln!(
            "CDN physionet.org: {}",
            if ok { "hochgeladen" } else { "upload void" }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::mitdb::parse_atr;

    #[test]
    fn parses_real_atr_shape() {
        let bytes = std::fs::read("/tmp/opencode/100.atr").unwrap();
        let series = parse_atr(&bytes, 360).unwrap();
        assert!(series.len() > 2000, "100.atr should carry >2000 beats");
        let mean_rr = series.iter().map(|(_, r)| r).sum::<f64>() / series.len() as f64;
        assert!(
            (mean_rr - 0.795).abs() < 0.05,
            "RR mean of record 100 should be ~0.795 s, got {}",
            mean_rr
        );
    }

    #[test]
    fn roundtrip_bin() {
        let recs = vec![(0.5, 0.795, 1u32), (1.295, 0.812, 1), (0.2, 0.9, 2)];
        let bytes = write_bin(&recs);
        assert_eq!(&bytes[0..4], b"RRB1");
        assert_eq!(bytes.len(), 8 + 3 * 20);
    }
}
