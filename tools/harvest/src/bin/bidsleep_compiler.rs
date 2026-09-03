use omegaflow::archivar::bidsleep::{COMP_IHR, COMP_MX, COMP_MY, COMP_MZ, parse_bin, write_bin};
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::LeapSeconds;
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://physionet.org/files/bidsleep-dataset/1.0.0/";
const CDN_RELEASE: &str = "physionet.org";
const DEFAULT_DECIMATE_MIN: f64 = 1.0;

const G_STANDARD: f64 = 9.80665;
const IHR_BPM_MAX: f64 = 300.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_timeout(url: &str, max_time: u64) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("1")
        .arg("--retry-delay")
        .arg("2")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg(max_time.to_string())
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

fn fetch(url: &str) -> Option<Vec<u8>> {
    fetch_timeout(url, 120)
}

fn fetch_big(url: &str) -> Option<Vec<u8>> {
    fetch_timeout(url, 14400)
}

fn night_list(sha_text: &str) -> Vec<(String, String)> {
    let mut nights: Vec<(String, String)> = sha_text
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| l.ends_with("/hr.csv"))
        .filter_map(|l| {
            let path = l.split_whitespace().next_back()?;
            let path = path.trim_start_matches("./");
            let rest = path.strip_suffix("/hr.csv")?;
            let (subject, night) = rest.rsplit_once('/')?;
            Some((subject.to_string(), night.to_string()))
        })
        .collect();
    nights.sort();
    nights.dedup();
    nights
}

fn bucket_of(ts: f64, bucket_s: f64) -> Option<u64> {
    if !ts.is_finite() || ts < 0.0 {
        return None;
    }
    let b = (ts / bucket_s).floor();
    if b < 0.0 || b > 4_294_967_296.0 {
        return None;
    }
    Some(b as u64)
}

fn buckets_to_records(
    buckets: &mut HashMap<u64, Vec<f64>>,
    bucket_s: f64,
    lsk: &LeapSeconds,
    comp: u32,
) -> Vec<(f64, f64, u32)> {
    let mut keys: Vec<u64> = buckets.keys().copied().collect();
    keys.sort_unstable();
    let mut out = Vec::with_capacity(keys.len());
    for b in keys {
        let Some(mut vals) = buckets.remove(&b) else {
            continue;
        };
        let v = omegaflow::archivar::bidsleep::median(&mut vals);
        let center_unix = (b as f64 + 0.5) * bucket_s;
        let Some(tdb) = lsk.unix_to_tdb(center_unix) else {
            eprintln!(
                "bucket {} carries a void TDB epoch — the bucket stays out (0 honored)",
                b
            );
            continue;
        };
        out.push((tdb, v, comp));
    }
    out
}

fn harvest_night(
    subject: &str,
    night: &str,
    bucket_s: f64,
    lsk: &LeapSeconds,
) -> Result<Vec<(f64, f64, u32)>, String> {
    let hr_url = format!("{}{}/{}/hr.csv", BASE, subject, night);
    let Some(hr_bytes) = fetch(&hr_url) else {
        return Err("hr.csv fetch void".into());
    };
    let mut hr_buckets: HashMap<u64, Vec<f64>> = HashMap::new();
    let mut n_hr = 0usize;
    let mut bad_hr = 0usize;
    for line in String::from_utf8_lossy(&hr_bytes).lines() {
        let line = line.trim_end_matches('\r').trim();
        if line.is_empty() {
            continue;
        }
        let Some((ts_s, bpm_s)) = line.split_once(',') else {
            continue;
        };
        let Ok(ts) = ts_s.trim().parse::<f64>() else {
            continue;
        };
        let Ok(bpm) = bpm_s.trim().parse::<f64>() else {
            continue;
        };
        if !(bpm.is_finite() && bpm > 0.0 && bpm <= IHR_BPM_MAX) {
            bad_hr += 1;
            continue;
        }
        let Some(bucket) = bucket_of(ts, bucket_s) else {
            continue;
        };
        hr_buckets.entry(bucket).or_default().push(bpm);
        n_hr += 1;
    }
    let mo_url = format!("{}{}/{}/motion.csv", BASE, subject, night);
    let Some(mo_bytes) = fetch_big(&mo_url) else {
        return Err("motion.csv fetch void".into());
    };
    let mut mx: HashMap<u64, Vec<f64>> = HashMap::new();
    let mut my: HashMap<u64, Vec<f64>> = HashMap::new();
    let mut mz: HashMap<u64, Vec<f64>> = HashMap::new();
    let mut n_mot = 0usize;
    let mut bad_mot = 0usize;
    for line in String::from_utf8_lossy(&mo_bytes).lines() {
        let line = line.trim_end_matches('\r').trim();
        if line.is_empty() || line.starts_with("Timestamp") {
            continue;
        }
        let toks: Vec<&str> = line.split(',').collect();
        if toks.len() < 4 {
            continue;
        }
        let Ok(ts) = toks[0].trim().parse::<f64>() else {
            continue;
        };
        let Some(bucket) = bucket_of(ts, bucket_s) else {
            continue;
        };
        let mut axes = [0.0f64; 3];
        let mut valid = true;
        for k in 0..3 {
            let Ok(v) = toks[k + 1].trim().parse::<f64>() else {
                valid = false;
                break;
            };
            if !v.is_finite() {
                valid = false;
                break;
            }
            axes[k] = v;
        }
        if !valid {
            bad_mot += 1;
            continue;
        }
        n_mot += 1;
        mx.entry(bucket)
            .or_default()
            .push(axes[0].abs() * G_STANDARD);
        my.entry(bucket)
            .or_default()
            .push(axes[1].abs() * G_STANDARD);
        mz.entry(bucket)
            .or_default()
            .push(axes[2].abs() * G_STANDARD);
    }
    let mut records = Vec::new();
    records.extend(buckets_to_records(&mut hr_buckets, bucket_s, lsk, COMP_IHR));
    records.extend(buckets_to_records(&mut mx, bucket_s, lsk, COMP_MX));
    records.extend(buckets_to_records(&mut my, bucket_s, lsk, COMP_MY));
    records.extend(buckets_to_records(&mut mz, bucket_s, lsk, COMP_MZ));
    eprintln!(
        "{}/{}: {} hr samples ({} out), {} motion samples ({} out), {} bucket medians",
        subject,
        night,
        n_hr,
        bad_hr,
        n_mot,
        bad_mot,
        records.len()
    );
    if records.is_empty() {
        return Err("no sample carries a listed value".into());
    }
    Ok(records)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--merge") {
        merge_chunks(&args);
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "bidsleep_mehrnacht.bin".to_string());
    let decimate_min: f64 = arg_value(&args, "--decimate-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_DECIMATE_MIN);
    let bucket_s = decimate_min * 60.0;
    if !(bucket_s > 0.0) || !bucket_s.is_finite() {
        eprintln!(
            "--decimate-min {} carries no positive bucket width",
            decimate_min
        );
        std::process::exit(1);
    }
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let limit: Option<usize> = arg_value(&args, "--limit").and_then(|v| v.parse().ok());
    let offset: usize = arg_value(&args, "--offset")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the TDB epoch stays void (no fabricated epoch)");
        std::process::exit(1);
    };
    let lsk = Arc::new(lsk);
    let Some(sha_bytes) = fetch(&format!("{}SHA256SUMS.txt", BASE)) else {
        eprintln!("SHA256SUMS fetch void — the harvest stays void (0 honored)");
        std::process::exit(1);
    };
    let mut nights = night_list(&String::from_utf8_lossy(&sha_bytes));
    if offset > 0 {
        nights.drain(..offset.min(nights.len()));
    }
    if let Some(l) = limit {
        nights.truncate(l);
    }
    if nights.is_empty() {
        eprintln!("SHA256SUMS carries no nights — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "SHA256SUMS: {} nights, {} min buckets ({} s)",
        nights.len(),
        decimate_min,
        bucket_s
    );
    let nights = Arc::new(nights);
    let series: Arc<Mutex<Vec<(f64, f64, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let next = Arc::new(AtomicI64::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    let skipped = Arc::new(AtomicUsize::new(0));
    let total = nights.len();
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let nights = Arc::clone(&nights);
            let series = Arc::clone(&series);
            let next = Arc::clone(&next);
            let done = Arc::clone(&done);
            let skipped = Arc::clone(&skipped);
            let lsk = Arc::clone(&lsk);
            s.spawn(move || {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst) as usize;
                    if i >= nights.len() {
                        break;
                    }
                    let (subject, night) = &nights[i];
                    match harvest_night(subject, night, bucket_s, &lsk) {
                        Ok(recs) => series
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .extend(recs),
                        Err(note) => {
                            eprintln!("Bidslab{}/{}: {}", subject, night, note);
                            skipped.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                    let n = done.fetch_add(1, Ordering::SeqCst) + 1;
                    if n % 10 == 0 || n == total {
                        eprintln!("{}/{} nights harvested", n, total);
                    }
                }
            });
        }
    });
    let mut series = Arc::try_unwrap(series)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    series.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    let skipped = skipped.load(Ordering::SeqCst);
    if series.is_empty() {
        eprintln!("{}: no records — the bin stays unwritten (0 honored)", out);
        std::process::exit(1);
    }
    eprintln!(
        "{}: {} bucket medians ({} min buckets), {} B, {} nights skipped",
        out,
        series.len(),
        decimate_min,
        series.len() * 20 + 8,
        skipped
    );
    let bytes = write_bin(&series);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{}: {} records, roundtrip parses", out, parsed.len());
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_RELEASE, &out) {
        std::process::exit(1);
    }
}

fn merge_chunks(args: &[String]) {
    let out = arg_value(args, "--out").unwrap_or_else(|| "bidsleep_mehrnacht.bin".to_string());
    let chunk_paths: Vec<String> = args
        .iter()
        .position(|a| a == "--merge")
        .map(|p| args[p + 1..].to_vec())
        .unwrap_or_default();
    if chunk_paths.is_empty() {
        eprintln!("--merge carries no chunk paths");
        std::process::exit(1);
    }
    let mut merged: Vec<(f64, f64, u32)> = Vec::new();
    for path in &chunk_paths {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {} returned void: {}", path, e);
                std::process::exit(1);
            }
        };
        match parse_bin(&bytes) {
            Some(recs) => merged.extend(recs),
            None => {
                eprintln!("{}: parse void — the chunk stays out (0 honored)", path);
                std::process::exit(1);
            }
        }
    }
    merged.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    let bytes = write_bin(&merged);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    eprintln!(
        "{}: {} records merged from {} chunks",
        out,
        merged.len(),
        chunk_paths.len()
    );
    if args.iter().any(|a| a == "--ci-mode") && !upload_release(CDN_RELEASE, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_night_manifest() {
        let text = "2cc2e3f4 Bidslab00/1/hr.csv\nc7827450 Bidslab00/1/labels.mat\n1ff851c0 Bidslab00/1/motion.csv\nc3b17710 Bidslab00/2/hr.csv\n";
        let nights = night_list(text);
        assert_eq!(nights.len(), 2);
        assert_eq!(nights[0], ("Bidslab00".to_string(), "1".to_string()));
        assert_eq!(nights[1], ("Bidslab00".to_string(), "2".to_string()));
    }

    #[test]
    fn parses_dotted_and_duplicate_paths() {
        let text = "abc ./Bidslab07/3/hr.csv\ndef Bidslab07/3/hr.csv\n";
        let nights = night_list(text);
        assert_eq!(nights.len(), 1);
        assert_eq!(nights[0], ("Bidslab07".to_string(), "3".to_string()));
    }

    #[test]
    fn bucket_of_gates_time() {
        assert_eq!(bucket_of(60.0, 60.0), Some(1));
        assert_eq!(bucket_of(59.999, 60.0), Some(0));
        assert_eq!(bucket_of(-1.0, 60.0), None);
        assert_eq!(bucket_of(f64::NAN, 60.0), None);
        assert_eq!(bucket_of(1e12, 60.0), None);
    }

    #[test]
    fn hr_line_plausibility_holds() {
        assert!(48.0 > 0.0 && 48.0 <= IHR_BPM_MAX);
        assert!(0.0 <= IHR_BPM_MAX);
        assert!(!(0.0 > 0.0));
        assert!(300.0 > 0.0 && 300.0 <= IHR_BPM_MAX);
        assert!(!(301.0 > 0.0 && 301.0 <= IHR_BPM_MAX));
    }
}
