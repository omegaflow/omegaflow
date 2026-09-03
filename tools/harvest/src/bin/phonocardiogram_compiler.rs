use omegaflow::archivar::phonocardiogram::{
    COMP_AV, COMP_MV, COMP_PHC, COMP_PV, COMP_TV, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://physionet.org/files/circor-heart-sound/1.0.3/";
const CDN_RELEASE: &str = "physionet.org";
const DEFAULT_DECIMATE_MIN: f64 = 0.05;

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

struct HeaSignal {
    bits: u16,
    nchan: u16,
    sigtype: String,
}

struct HeaInfo {
    sample_rate: u32,
    nsamp: usize,
    signals: Vec<HeaSignal>,
    comment: Option<String>,
}

fn parse_hea(text: &str) -> Option<HeaInfo> {
    let mut lines = text.lines().map(|l| l.trim_end_matches('\r'));
    let tokens: Vec<&str> = lines.next()?.split_whitespace().collect();
    if tokens.len() < 4 {
        return None;
    }
    let sample_rate: u32 = tokens[2].parse().ok()?;
    if sample_rate == 0 {
        return None;
    }
    let nsamp: usize = tokens[3].parse().ok()?;
    let mut signals = Vec::new();
    let mut comment = None;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(c) = line.strip_prefix('#') {
            comment = Some(c.trim().to_string());
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() < 3 {
            continue;
        }
        let bits = toks[1].split('+').next()?.parse::<u16>().ok()?;
        let nchan: u16 = toks[2].parse().ok()?;
        let sigtype = toks.last()?.to_string();
        signals.push(HeaSignal {
            bits,
            nchan,
            sigtype,
        });
    }
    if signals.is_empty() {
        return None;
    }
    Some(HeaInfo {
        sample_rate,
        nsamp,
        signals,
        comment,
    })
}

struct WavInfo {
    data_off: usize,
    data_len: usize,
    nchan: usize,
    sample_rate: u32,
}

fn parse_wav(bytes: &[u8]) -> Option<WavInfo> {
    if bytes.len() < 44 || bytes[0..4] != *b"RIFF" || bytes[8..12] != *b"WAVE" {
        return None;
    }
    let mut off = 12usize;
    let mut fmt: Option<(u16, u16, u32, u16)> = None;
    let mut data: Option<(usize, usize)> = None;
    while off + 8 <= bytes.len() {
        let id = &bytes[off..off + 4];
        let size = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().ok()?) as usize;
        if id == *b"fmt " {
            if off + 8 + 16 > bytes.len() {
                return None;
            }
            let tag = u16::from_le_bytes(bytes[off + 8..off + 10].try_into().ok()?);
            let nchan = u16::from_le_bytes(bytes[off + 10..off + 12].try_into().ok()?);
            let rate = u32::from_le_bytes(bytes[off + 12..off + 16].try_into().ok()?);
            let bits = u16::from_le_bytes(bytes[off + 22..off + 24].try_into().ok()?);
            fmt = Some((tag, nchan, rate, bits));
        } else if id == *b"data" {
            let body = off + 8;
            if body + size > bytes.len() {
                return None;
            }
            data = Some((body, size));
        }
        off += 8 + size + (size % 2);
    }
    let (tag, nchan, rate, bits) = fmt?;
    let (data_off, data_len) = data?;
    if tag != 1 || bits != 16 || nchan == 0 || rate == 0 || data_len % 2 != 0 {
        return None;
    }
    Some(WavInfo {
        data_off,
        data_len,
        nchan: nchan as usize,
        sample_rate: rate,
    })
}

fn comp_of(sigtype: &str) -> Option<u32> {
    match sigtype.to_uppercase().as_str() {
        "AV" => Some(COMP_AV),
        "MV" => Some(COMP_MV),
        "PV" => Some(COMP_PV),
        "TV" => Some(COMP_TV),
        "PHC" => Some(COMP_PHC),
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

fn bucket_medians(wav: &[u8], info: &WavInfo, bucket_s: f64) -> Vec<(f64, f64)> {
    let mut buckets: HashMap<u64, Vec<f64>> = HashMap::new();
    let nchan = info.nchan.max(1);
    let samples = (info.data_len / 2) / nchan;
    for i in 0..samples {
        let off = info.data_off + i * 2 * nchan;
        let raw = i16::from_le_bytes([wav[off], wav[off + 1]]);
        let epoch = i as f64 / info.sample_rate as f64;
        let bucket = (epoch / bucket_s).floor() as u64;
        buckets.entry(bucket).or_default().push((raw as f64).abs());
    }
    let mut out: Vec<(f64, f64)> = buckets
        .into_iter()
        .map(|(b, mut vals)| ((b as f64 + 0.5) * bucket_s, median(&mut vals)))
        .collect();
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn harvest_record(rel: &str, bucket_s: f64) -> Result<Vec<(f64, f64, u32)>, String> {
    let hea_url = format!("{}{}.hea", BASE, rel);
    let Some(hea_bytes) = fetch(&hea_url) else {
        return Err("hea fetch void".into());
    };
    let hea = parse_hea(&String::from_utf8_lossy(&hea_bytes)).ok_or("hea parses void")?;
    let signal = hea.signals.first().ok_or("hea carries no signal line")?;
    if signal.bits != 16 {
        return Err(format!("bits {} unsupported", signal.bits));
    }
    let comp = comp_of(&signal.sigtype).ok_or(format!("sigtype {} unlisted", signal.sigtype))?;
    let wav_url = format!("{}{}.wav", BASE, rel);
    let Some(wav_bytes) = fetch(&wav_url) else {
        return Err("wav fetch void".into());
    };
    let wav = parse_wav(&wav_bytes).ok_or("wav header parses void")?;
    if wav.nchan != signal.nchan as usize {
        return Err(format!(
            "channel drift: hea {} wav {}",
            signal.nchan, wav.nchan
        ));
    }
    if wav.sample_rate != hea.sample_rate {
        return Err(format!(
            "rate drift: hea {} wav {}",
            hea.sample_rate, wav.sample_rate
        ));
    }
    let wav_nsamp = (wav.data_len / 2) / wav.nchan;
    if wav_nsamp != hea.nsamp {
        return Err(format!("sample drift: hea {} wav {}", hea.nsamp, wav_nsamp));
    }
    if let Some(c) = &hea.comment {
        if !c.is_empty() {
            eprintln!("{}: .hea comment: {}", rel, c);
        }
    }
    Ok(bucket_medians(&wav_bytes, &wav, bucket_s)
        .into_iter()
        .map(|(t, v)| (t, v, comp))
        .collect())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "circor_pcg.bin".to_string());
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
                    if n % 200 == 0 || n == total {
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

    fn synth_wav(rate: u32, samples: &[i16]) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"RIFF");
        b.extend_from_slice(&((36 + samples.len() * 2) as u32).to_le_bytes());
        b.extend_from_slice(b"WAVE");
        b.extend_from_slice(b"fmt ");
        b.extend_from_slice(&16u32.to_le_bytes());
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&rate.to_le_bytes());
        b.extend_from_slice(&(rate * 2).to_le_bytes());
        b.extend_from_slice(&2u16.to_le_bytes());
        b.extend_from_slice(&16u16.to_le_bytes());
        b.extend_from_slice(b"data");
        b.extend_from_slice(&((samples.len() * 2) as u32).to_le_bytes());
        for s in samples {
            b.extend_from_slice(&s.to_le_bytes());
        }
        b
    }

    #[test]
    fn roundtrip() {
        let records = vec![
            (0.5, 287.0, COMP_AV),
            (3.0, 229.0, COMP_MV),
            (6.5, 401.0, COMP_PHC),
        ];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"CRCR\x00").is_none());
    }

    #[test]
    fn rejects_unknown_component() {
        let bytes = write_bin(&[(10.0, 1.0, 7)]);
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn parses_real_hea() {
        let text = "2530_AV 1 4000 94400\n2530_AV.wav 16+44 1 16 0 0 0 0 AV\n";
        let hea = parse_hea(text).unwrap();
        assert_eq!(hea.sample_rate, 4000);
        assert_eq!(hea.nsamp, 94400);
        assert_eq!(hea.signals.len(), 1);
        assert_eq!(hea.signals[0].bits, 16);
        assert_eq!(hea.signals[0].nchan, 1);
        assert_eq!(hea.signals[0].sigtype, "AV");
        assert!(hea.comment.is_none());
    }

    #[test]
    fn maps_all_auscultation_points() {
        assert_eq!(comp_of("AV"), Some(COMP_AV));
        assert_eq!(comp_of("MV"), Some(COMP_MV));
        assert_eq!(comp_of("PV"), Some(COMP_PV));
        assert_eq!(comp_of("TV"), Some(COMP_TV));
        assert_eq!(comp_of("Phc"), Some(COMP_PHC));
        assert_eq!(comp_of("XY"), None);
    }

    #[test]
    fn parses_synthetic_wav() {
        let wav = synth_wav(4000, &[1, -2, 3]);
        let info = parse_wav(&wav).unwrap();
        assert_eq!(info.sample_rate, 4000);
        assert_eq!(info.nchan, 1);
        assert_eq!(info.data_off, 44);
        assert_eq!(info.data_len, 6);
    }

    #[test]
    fn wav_sample_count_meets_hea() {
        let wav = synth_wav(4000, &[0; 94400]);
        let info = parse_wav(&wav).unwrap();
        let wav_nsamp = (info.data_len / 2) / info.nchan;
        assert_eq!(wav_nsamp, 94400);
    }

    #[test]
    fn bucket_medians_envelope() {
        let samples: Vec<i16> = (0..12000)
            .map(|i| if i < 6000 { 2000 } else { -2000 })
            .collect();
        let wav = synth_wav(4000, &samples);
        let info = parse_wav(&wav).unwrap();
        let meds = bucket_medians(&wav, &info, 3.0);
        assert_eq!(meds.len(), 1);
        assert!((meds[0].0 - 1.5).abs() < 1e-9);
        assert!((meds[0].1 - 2000.0).abs() < 1e-9);
    }

    #[test]
    fn median_handles_odd_and_even() {
        assert_eq!(median(&mut vec![3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&mut vec![4.0, 1.0, 2.0, 3.0]), 2.5);
    }
}
