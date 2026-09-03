use omegaflow::archivar::movement_monitoring::{
    COMP_AP, COMP_ML, COMP_PITCH, COMP_ROLL, COMP_V, COMP_YAW, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://physionet.org/files/ltmm/1.0.0/";
const CDN_RELEASE: &str = "physionet.org";
const DEFAULT_DECIMATE_MIN: f64 = 1.0;

const G_STANDARD: f64 = 9.80665;
const DEG_TO_RAD: f64 = PI / 180.0;

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
        .arg("2")
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
    fetch_timeout(url, 1800)
}

struct LtmmSignal {
    format: u16,
    gain: f64,
    baseline: f64,
    unit: String,
    name: String,
}

struct HeaInfo {
    nchan: usize,
    sample_rate: u32,
    nsamp: usize,
    signals: Vec<LtmmSignal>,
    comment: Option<String>,
}

fn parse_gain_field(s: &str) -> Option<(f64, f64, String)> {
    let (num, tail) = s.split_once('(')?;
    let gain: f64 = num.parse().ok()?;
    if !gain.is_finite() || gain <= 0.0 {
        return None;
    }
    let (base, rest) = tail.split_once(')')?;
    let baseline: f64 = base.parse().ok()?;
    if !baseline.is_finite() {
        return None;
    }
    let unit = rest.strip_prefix('/')?.to_string();
    if unit.is_empty() {
        return None;
    }
    Some((gain, baseline, unit))
}

fn parse_hea(text: &str) -> Option<HeaInfo> {
    let mut lines = text.lines().map(|l| l.trim_end_matches('\r'));
    let tokens: Vec<&str> = lines.next()?.split_whitespace().collect();
    if tokens.len() < 4 {
        return None;
    }
    let nchan: usize = tokens[1].parse().ok()?;
    let sample_rate: u32 = tokens[2].parse().ok()?;
    let nsamp: usize = tokens[3].parse().ok()?;
    if nchan == 0 || nsamp == 0 || sample_rate == 0 {
        return None;
    }
    let mut signals = Vec::new();
    let mut comment = None;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(c) = line.strip_prefix('#') {
            if comment.is_none() {
                comment = Some(c.trim().to_string());
            }
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 3 {
            continue;
        }
        let format: u16 = toks[1].parse().ok()?;
        let (gain, baseline, unit) = parse_gain_field(toks[2])?;
        let name = toks.last()?.to_string();
        signals.push(LtmmSignal {
            format,
            gain,
            baseline,
            unit,
            name,
        });
    }
    if signals.len() != nchan {
        return None;
    }
    Some(HeaInfo {
        nchan,
        sample_rate,
        nsamp,
        signals,
        comment,
    })
}

fn unit_scale(unit: &str) -> Option<f64> {
    match unit {
        "g" => Some(G_STANDARD),
        "degrees/s" => Some(DEG_TO_RAD),
        _ => None,
    }
}

fn comp_of(name: &str) -> Option<u32> {
    match name.to_uppercase().as_str() {
        "V-ACCELERATION" => Some(COMP_V),
        "ML-ACCELERATION" => Some(COMP_ML),
        "AP-ACCELERATION" => Some(COMP_AP),
        "YAW-VELOCITY" => Some(COMP_YAW),
        "PITCH-VELOCITY" => Some(COMP_PITCH),
        "ROLL-VELOCITY" => Some(COMP_ROLL),
        _ => None,
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
    min: f64,
    max: f64,
}

fn bucket_medians(
    data: &[u8],
    nchan: usize,
    channel: usize,
    gain: f64,
    baseline: f64,
    scale: f64,
    sample_rate: u32,
    bucket_s: f64,
) -> (Vec<(f64, f64)>, ChannelStats) {
    let mut buckets: HashMap<u64, Vec<f64>> = HashMap::new();
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let frames = data.len() / (nchan * 2);
    let mut n = 0usize;
    for i in 0..frames {
        let off = i * nchan * 2 + channel * 2;
        let raw = i16::from_le_bytes([data[off], data[off + 1]]);
        let phys = ((raw as f64 - baseline) / gain) * scale;
        if phys < min {
            min = phys;
        }
        if phys > max {
            max = phys;
        }
        n += 1;
        let epoch = i as f64 / sample_rate as f64;
        let bucket = (epoch / bucket_s).floor() as u64;
        buckets.entry(bucket).or_default().push(phys.abs());
    }
    let mut out: Vec<(f64, f64)> = buckets
        .into_iter()
        .map(|(b, mut vals)| ((b as f64 + 0.5) * bucket_s, median(&mut vals)))
        .collect();
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    let stats = if n == 0 {
        ChannelStats {
            n: 0,
            min: 0.0,
            max: 0.0,
        }
    } else {
        ChannelStats { n, min, max }
    };
    (out, stats)
}

fn harvest_record(rel: &str, bucket_s: f64) -> Result<Vec<(f64, f64, u32)>, String> {
    let hea_url = format!("{}{}.hea", BASE, rel);
    let Some(hea_bytes) = fetch(&hea_url) else {
        return Err("hea fetch void".into());
    };
    let hea = parse_hea(&String::from_utf8_lossy(&hea_bytes)).ok_or("hea parses void")?;
    if hea.nchan != 6 {
        return Err(format!("nchan {} unsupported", hea.nchan));
    }
    if hea.signals.iter().any(|s| s.format != 16) {
        return Err("a signal carries a non-16 format".into());
    }
    let dat_url = format!("{}{}.dat", BASE, rel);
    let Some(dat_bytes) = fetch_big(&dat_url) else {
        return Err("dat fetch void".into());
    };
    let expected = hea
        .nsamp
        .checked_mul(hea.nchan * 2)
        .ok_or("nsamp overflow")?;
    if dat_bytes.len() != expected {
        return Err(format!(
            "sample drift: hea {} samples, dat {} bytes ({} samples)",
            hea.nsamp,
            dat_bytes.len(),
            dat_bytes.len() / (hea.nchan * 2)
        ));
    }
    let mut records = Vec::new();
    for (i, sig) in hea.signals.iter().enumerate() {
        let Some(comp) = comp_of(&sig.name) else {
            eprintln!(
                "{}: signal {} unlisted — the channel stays unharvested (0 honored)",
                rel, sig.name
            );
            continue;
        };
        let Some(scale) = unit_scale(&sig.unit) else {
            eprintln!(
                "{}: unit {} unlisted — the channel stays unharvested (0 honored)",
                rel, sig.unit
            );
            continue;
        };
        let (meds, stats) = bucket_medians(
            &dat_bytes,
            hea.nchan,
            i,
            sig.gain,
            sig.baseline,
            scale,
            hea.sample_rate,
            bucket_s,
        );
        let unit = if sig.unit == "g" { "m/s²" } else { "rad/s" };
        eprintln!(
            "{} {}: {} samples {:.3}..{:.3} {}, {} buckets",
            rel,
            sig.name,
            stats.n,
            stats.min,
            stats.max,
            unit,
            meds.len()
        );
        records.extend(meds.into_iter().map(|(t, v)| (t, v, comp)));
    }
    if records.is_empty() {
        return Err(format!("{}: no channel carries a listed signal", rel));
    }
    if let Some(c) = &hea.comment {
        eprintln!("{}: patient line: {}", rel, c);
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
    let out = arg_value(&args, "--out").unwrap_or_else(|| "ltmm_movement.bin".to_string());
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
    if offset > 0 {
        records.drain(..offset.min(records.len()));
    }
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

fn merge_chunks(args: &[String]) {
    let out = arg_value(args, "--out").unwrap_or_else(|| "ltmm_movement.bin".to_string());
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

    fn synth_16(channels: usize, samples: &[Vec<i16>]) -> Vec<u8> {
        let n = samples[0].len();
        let mut b = Vec::with_capacity(n * channels * 2);
        for i in 0..n {
            for c in 0..channels {
                b.extend_from_slice(&samples[c][i].to_le_bytes());
            }
        }
        b
    }

    #[test]
    fn roundtrip() {
        let records = vec![
            (0.5, 9.7, COMP_V),
            (60.5, 12.1, COMP_ML),
            (120.5, 0.31, COMP_ROLL),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"LTMM\x00").is_none());
    }

    #[test]
    fn rejects_unknown_component() {
        let bytes = write_bin(&[(10.0, 1.0, 9)]);
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn parses_real_hea() {
        let text = "CO001 6 100 17141698\nCO001.dat 16 5594.5611(120)/g 0 0 -16 10729 0 v-acceleration\nCO001.dat 16 5637.2473(136)/g 0 0 96 18803 0 ml-acceleration\nCO001.dat 16 5517.1923(192)/g 0 0 -5603 -28221 0 ap-acceleration\nCO001.dat 16 145.8539(-1152)/degrees/s 0 0 -1124 3453 0 yaw-velocity\nCO001.dat 16 141.7616(-1261)/degrees/s 0 0 -883 -9724 0 pitch-velocity\nCO001.dat 16 141.5523(633)/degrees/s 0 0 634 -26911 0 roll-velocity\n#Age:75.17\n#Sex:F\n";
        let hea = parse_hea(text).unwrap();
        assert_eq!(hea.nchan, 6);
        assert_eq!(hea.sample_rate, 100);
        assert_eq!(hea.nsamp, 17141698);
        assert_eq!(hea.signals.len(), 6);
        assert_eq!(hea.signals[0].format, 16);
        assert_eq!(hea.signals[0].gain, 5594.5611);
        assert_eq!(hea.signals[0].baseline, 120.0);
        assert_eq!(hea.signals[0].unit, "g");
        assert_eq!(hea.signals[0].name, "v-acceleration");
        assert_eq!(hea.signals[5].baseline, 633.0);
        assert_eq!(hea.signals[5].unit, "degrees/s");
        assert_eq!(hea.signals[5].name, "roll-velocity");
        assert_eq!(hea.comment.as_deref(), Some("Age:75.17"));
    }

    #[test]
    fn parses_negative_gain_baseline() {
        let text = "FL001 6 100 24542514\nFL001.dat 16 16943.7565(-136)/g 0 0 2433 18132 0 v-acceleration\nFL001.dat 16 16735.6612(-232)/g 0 0 3890 -7444 0 ml-acceleration\nFL001.dat 16 16463.5366(72)/g 0 0 -16584 14478 0 ap-acceleration\nFL001.dat 16 145.4294(422)/degrees/s 0 0 -546 -5303 0 yaw-velocity\nFL001.dat 16 139.4163(223)/degrees/s 0 0 -4168 11422 0 pitch-velocity\nFL001.dat 16 82.4151(-809)/degrees/s 0 0 -1863 -20148 0 roll-velocity\n#Age:79.2\n#Sex:M\n";
        let hea = parse_hea(text).unwrap();
        assert_eq!(hea.nchan, 6);
        assert_eq!(hea.nsamp, 24542514);
        assert_eq!(hea.signals[0].baseline, -136.0);
        assert_eq!(hea.signals[2].baseline, 72.0);
        assert_eq!(hea.signals[5].baseline, -809.0);
        assert_eq!(hea.signals[5].unit, "degrees/s");
    }

    #[test]
    fn gain_offset_conversion_in_si() {
        let raw = 120i16;
        let ms2 = ((raw as f64 - 120.0) / 5594.5611) * G_STANDARD;
        assert!((ms2 - 0.0).abs() < 1e-9);
        let raw = 5594 + 120;
        let ms2 = ((raw as f64 - 120.0) / 5594.5611) * G_STANDARD;
        assert!((ms2 - G_STANDARD).abs() < 0.01);
        let raw = -1152 + 146;
        let rad = ((raw as f64 + 1152.0) / 145.8539) * DEG_TO_RAD;
        assert!((rad - DEG_TO_RAD).abs() < 0.001);
    }

    #[test]
    fn maps_all_signals() {
        assert_eq!(comp_of("v-acceleration"), Some(COMP_V));
        assert_eq!(comp_of("ml-acceleration"), Some(COMP_ML));
        assert_eq!(comp_of("ap-acceleration"), Some(COMP_AP));
        assert_eq!(comp_of("yaw-velocity"), Some(COMP_YAW));
        assert_eq!(comp_of("pitch-velocity"), Some(COMP_PITCH));
        assert_eq!(comp_of("roll-velocity"), Some(COMP_ROLL));
        assert_eq!(comp_of("unknown-channel"), None);
    }

    #[test]
    fn unit_scale_gates_units() {
        assert_eq!(unit_scale("g"), Some(G_STANDARD));
        assert_eq!(unit_scale("degrees/s"), Some(DEG_TO_RAD));
        assert_eq!(unit_scale("mV"), None);
        assert_eq!(unit_scale(""), None);
    }

    #[test]
    fn decode_16_interleaved_channels() {
        let ch0: Vec<i16> = (0..300).map(|i| i as i16).collect();
        let ch1: Vec<i16> = (0..300).map(|i| -i as i16).collect();
        let data = synth_16(2, &[ch0.clone(), ch1.clone()]);
        let frames = data.len() / (2 * 2);
        assert_eq!(frames, 300);
        for i in 0..frames {
            let a = i16::from_le_bytes([data[i * 4], data[i * 4 + 1]]);
            let b = i16::from_le_bytes([data[i * 4 + 2], data[i * 4 + 3]]);
            assert_eq!(a, i as i16);
            assert_eq!(b, -(i as i16));
        }
    }

    #[test]
    fn bucket_medians_rest_at_one_g() {
        let mut ch: Vec<i16> = Vec::new();
        for _ in 0..6000 {
            ch.push(120 + 5595);
        }
        let data = synth_16(1, &[ch]);
        let (meds, stats) = bucket_medians(&data, 1, 0, 5594.5611, 120.0, G_STANDARD, 100, 60.0);
        assert_eq!(stats.n, 6000);
        assert!((meds[0].1 - G_STANDARD).abs() < 0.01);
        assert_eq!(meds.len(), 1);
        assert!((meds[0].0 - 30.0).abs() < 1e-9);
    }

    #[test]
    fn median_handles_odd_and_even() {
        assert_eq!(median(&mut vec![3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&mut vec![4.0, 1.0, 2.0, 3.0]), 2.5);
    }
}
