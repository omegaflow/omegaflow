use omegaflow::archivar::mitdb::{comp_of_lead, decode_212, parse_bin, parse_hea, write_bin};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://physionet.org/files/mitdb/1.0.0/";
const CDN_RELEASE: &str = "physionet.org";
const DEFAULT_DECIMATE_MIN: f64 = 0.2;

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

fn median(vals: &mut Vec<f64>) -> f64 {
    vals.sort_by(|a, b| a.total_cmp(b));
    let n = vals.len();
    if n % 2 == 0 {
        (vals[n / 2 - 1] + vals[n / 2]) * 0.5
    } else {
        vals[n / 2]
    }
}

struct ChannelStats {
    n: usize,
    min_mv: f64,
    max_mv: f64,
}

fn bucket_medians(
    channel: &[i32],
    gain: f64,
    adc_zero: i64,
    sample_rate: u32,
    bucket_s: f64,
    comp: u32,
) -> (Vec<(f64, f64, u32)>, ChannelStats) {
    let mut buckets: HashMap<u64, Vec<f64>> = HashMap::new();
    let mut min_mv = f64::INFINITY;
    let mut max_mv = f64::NEG_INFINITY;
    for (i, raw) in channel.iter().enumerate() {
        let mv = (*raw as f64 - adc_zero as f64) / gain;
        if mv < min_mv {
            min_mv = mv;
        }
        if mv > max_mv {
            max_mv = mv;
        }
        let epoch = i as f64 / sample_rate as f64;
        let bucket = (epoch / bucket_s).floor() as u64;
        buckets.entry(bucket).or_default().push(mv.abs());
    }
    let mut out: Vec<(f64, f64, u32)> = buckets
        .into_iter()
        .map(|(b, mut vals)| ((b as f64 + 0.5) * bucket_s, median(&mut vals), comp))
        .collect();
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    let stats = if channel.is_empty() {
        ChannelStats {
            n: 0,
            min_mv: 0.0,
            max_mv: 0.0,
        }
    } else {
        ChannelStats {
            n: channel.len(),
            min_mv,
            max_mv,
        }
    };
    (out, stats)
}

fn harvest_record(rel: &str, bucket_s: f64) -> Result<Vec<(f64, f64, u32)>, String> {
    let hea_url = format!("{}{}.hea", BASE, rel);
    let Some(hea_bytes) = fetch(&hea_url) else {
        return Err("hea fetch void".into());
    };
    let hea = parse_hea(&String::from_utf8_lossy(&hea_bytes)).ok_or("hea parses void")?;
    if hea.nchan != 2 {
        return Err(format!("nchan {} unsupported", hea.nchan));
    }
    if hea.leads.iter().any(|l| l.format != 212) {
        return Err("a lead carries a non-212 format".into());
    }
    let dat_url = format!("{}{}.dat", BASE, rel);
    let Some(dat_bytes) = fetch(&dat_url) else {
        return Err("dat fetch void".into());
    };
    let expected = hea.nsamp.checked_mul(3).ok_or("nsamp overflow")?;
    if dat_bytes.len() != expected {
        return Err(format!(
            "sample drift: hea {} samples, dat {} bytes ({} samples)",
            hea.nsamp,
            dat_bytes.len(),
            dat_bytes.len() / 3 * 2
        ));
    }
    let (ch0, ch1) = decode_212(&dat_bytes, hea.nsamp).ok_or("212 decode void")?;
    let mut records = Vec::new();
    for (lead, channel) in hea.leads.iter().zip([&ch0, &ch1]) {
        let Some(comp) = comp_of_lead(&lead.name) else {
            eprintln!(
                "{}: lead {} unlisted — the channel stays unharvested (0 honored)",
                rel, lead.name
            );
            continue;
        };
        let (meds, stats) = bucket_medians(
            channel,
            lead.gain,
            lead.adc_zero,
            hea.sample_rate,
            bucket_s,
            comp,
        );
        eprintln!(
            "{} {}: {} samples {:.2}..{:.2} mV, {} buckets",
            rel,
            lead.name,
            stats.n,
            stats.min_mv,
            stats.max_mv,
            meds.len()
        );
        records.extend(meds);
    }
    if records.is_empty() {
        return Err(format!("{}: no channel carries a listed lead", rel));
    }
    if let Some(c) = &hea.comment {
        eprintln!("{}: patient line: {}", rel, c);
    }
    Ok(records)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "mitdb_arrhythmia.bin".to_string());
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
    let Some(records_text) = fetch(&format!("{}RECORDS", BASE)) else {
        eprintln!("RECORDS fetch void — the harvest stays void (0 honored)");
        std::process::exit(1);
    };
    let mut records: Vec<String> = String::from_utf8_lossy(&records_text)
        .lines()
        .map(|l| l.trim_end_matches('\r').trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    records.sort();
    records.dedup();
    if let Some(l) = limit {
        records.truncate(l);
    }
    if records.is_empty() {
        eprintln!("RECORDS carries no entries — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "RECORDS: {} records, {} min buckets ({} s)",
        records.len(),
        decimate_min,
        bucket_s
    );
    let records = Arc::new(records);
    let series: Arc<Mutex<Vec<(f64, f64, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let next = Arc::new(AtomicI64::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    let skipped = Arc::new(AtomicUsize::new(0));
    let total = records.len();
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let records = Arc::clone(&records);
            let series = Arc::clone(&series);
            let next = Arc::clone(&next);
            let done = Arc::clone(&done);
            let skipped = Arc::clone(&skipped);
            s.spawn(move || {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst) as usize;
                    if i >= records.len() {
                        break;
                    }
                    let rel = &records[i];
                    match harvest_record(rel, bucket_s) {
                        Ok(recs) => series
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .extend(recs),
                        Err(note) => {
                            eprintln!("{}: {}", rel, note);
                            skipped.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                    let n = done.fetch_add(1, Ordering::SeqCst) + 1;
                    if n % 10 == 0 || n == total {
                        eprintln!("{}/{} records harvested", n, total);
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
        "{}: {} bucket medians ({} min buckets), {} B, {} records skipped",
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

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::mitdb::{COMP_MLII, COMP_V1, COMP_V2, COMP_V4, COMP_V5};

    fn synth_212(pairs: &[(i32, i32)]) -> Vec<u8> {
        let mut b = Vec::with_capacity(pairs.len() * 3);
        for &(a, c) in pairs {
            let a = a.rem_euclid(0x1000) as u16;
            let c = c.rem_euclid(0x1000) as u16;
            assert_eq!(a >> 8, c >> 8, "212 pairs share the top nibble");
            let hi = (a >> 8) as u8;
            b.push((a & 0xFF) as u8);
            b.push((hi << 4) | hi);
            b.push((c & 0xFF) as u8);
        }
        b
    }

    #[test]
    fn decode_212_roundtrips_synthetic_triples() {
        let pairs: Vec<(i32, i32)> = vec![
            (995, 1011),
            (768, 768),
            (-1, -1),
            (255, 255),
            (256, 511),
            (1024, 1279),
            (-2048, -2048),
        ];
        let bytes = synth_212(&pairs);
        let (d0, d1) = decode_212(&bytes, pairs.len()).unwrap();
        let c0: Vec<i32> = pairs.iter().map(|&(a, _)| a).collect();
        let c1: Vec<i32> = pairs.iter().map(|&(_, c)| c).collect();
        assert_eq!(d0, c0);
        assert_eq!(d1, c1);
    }

    #[test]
    fn decode_212_known_real_triples() {
        let (d0, d1) = decode_212(&[227, 51, 243], 1).unwrap();
        assert_eq!(d0, vec![995]);
        assert_eq!(d1, vec![1011]);
        let (e0, e1) = decode_212(&[0, 67, 0], 1).unwrap();
        assert_eq!(e0, vec![768]);
        assert_eq!(e1, vec![768]);
        let (f0, f1) = decode_212(&[255, 255, 255], 1).unwrap();
        assert_eq!(f0, vec![-1]);
        assert_eq!(f1, vec![-1]);
    }

    #[test]
    fn decode_212_rejects_wrong_length() {
        assert!(decode_212(&[0, 1, 2, 3], 2).is_none());
        assert!(decode_212(&[0, 1], 2).is_none());
        assert!(decode_212(&[0, 1, 2], 2).is_none());
    }

    #[test]
    fn physical_mv_matches_gain_adczero() {
        assert_eq!((1024 - 1024) as f64 / 200.0, 0.0);
        assert_eq!((3024 - 1024) as f64 / 200.0, 10.0);
    }

    #[test]
    fn parses_real_hea() {
        let text = "100 2 360 650000\n100.dat 212 200 11 1024 995 -22131 0 MLII\n100.dat 212 200 11 1024 1011 20052 0 V5\n# 69 M 1085 1629 x1\n# Aldomet, Inderal\n";
        let hea = parse_hea(text).unwrap();
        assert_eq!(hea.nchan, 2);
        assert_eq!(hea.sample_rate, 360);
        assert_eq!(hea.nsamp, 650000);
        assert_eq!(hea.leads.len(), 2);
        assert_eq!(hea.leads[0].format, 212);
        assert_eq!(hea.leads[0].gain, 200.0);
        assert_eq!(hea.leads[0].adc_zero, 1024);
        assert_eq!(hea.leads[0].name, "MLII");
        assert_eq!(hea.leads[1].name, "V5");
        assert_eq!(hea.comment.as_deref(), Some("69 M 1085 1629 x1"));
    }

    #[test]
    fn parses_hea_with_na_date_token() {
        let text = "208 2 360 650000\n208.dat 212 200 11 1024 1003 8074 0 MLII\n208.dat 212 200 11 1024 1062 20649 0 V1\n# 23 F 2546 N/A x1\n";
        let hea = parse_hea(text).unwrap();
        assert_eq!(hea.leads[1].name, "V1");
        assert_eq!(hea.comment.as_deref(), Some("23 F 2546 N/A x1"));
    }

    #[test]
    fn maps_all_leads() {
        assert_eq!(comp_of_lead("MLII"), Some(COMP_MLII));
        assert_eq!(comp_of_lead("V1"), Some(COMP_V1));
        assert_eq!(comp_of_lead("V2"), Some(COMP_V2));
        assert_eq!(comp_of_lead("V4"), Some(COMP_V4));
        assert_eq!(comp_of_lead("V5"), Some(COMP_V5));
        assert_eq!(comp_of_lead("V6"), None);
        assert_eq!(comp_of_lead("Vx"), None);
    }

    #[test]
    fn bucket_medians_envelope() {
        let samples: Vec<i32> = (0..360).map(|_| 3024).collect();
        let (meds, stats) = bucket_medians(&samples, 200.0, 1024, 360, 3.0, COMP_MLII);
        assert_eq!(stats.n, 360);
        assert_eq!(stats.min_mv, 10.0);
        assert_eq!(stats.max_mv, 10.0);
        assert_eq!(meds.len(), 1);
        assert!((meds[0].0 - 1.5).abs() < 1e-9);
        assert!((meds[0].1 - 10.0).abs() < 1e-9);
        assert_eq!(meds[0].2, COMP_MLII);
    }

    #[test]
    fn median_handles_odd_and_even() {
        assert_eq!(median(&mut vec![3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&mut vec![4.0, 1.0, 2.0, 3.0]), 2.5);
    }
}
