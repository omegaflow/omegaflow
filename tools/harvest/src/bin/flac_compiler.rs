use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::flac::{decode, parse_series, write_series};
use omegaflow::archivar::units::ymd_to_days;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "storage.googleapis.com";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn name_of(source: &str) -> String {
    let mut base = source;
    if let Some((_, tail)) = source.rsplit_once('/') {
        base = tail;
    }
    match base.split_once('?') {
        Some((head, _)) => head,
        None => base,
    }
    .to_string()
}

fn epoch_of_filename(name: &str) -> Option<f64> {
    let stem = name.strip_suffix(".flac").unwrap_or(name);
    let bytes = stem.as_bytes();
    let mut i = 0usize;
    while i + 8 <= bytes.len() {
        let cand = &bytes[i..i + 8];
        if cand.iter().all(u8::is_ascii_digit) {
            let y = stem[i..i + 4].parse::<i64>().ok()?;
            let m = stem[i + 4..i + 6].parse::<u32>().ok()?;
            let d = stem[i + 6..i + 8].parse::<u32>().ok()?;
            let rest = &stem[i + 8..];
            let (h, mi, s) = match rest.strip_prefix('_') {
                Some(r) if r.len() >= 6 && r[..6].bytes().all(|b| b.is_ascii_digit()) => {
                    let h = r[0..2].parse::<u32>().ok()?;
                    let mi = r[2..4].parse::<u32>().ok()?;
                    let s = r[4..6].parse::<u32>().ok()?;
                    (h, mi, s)
                }
                _ => (0, 0, 0),
            };
            let days = ymd_to_days(y, m, d)? as f64;
            let unix = days * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + s as f64;
            return Some(unix);
        }
        i += 1;
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "data/storage.googleapis.com/nrs_audio_series.bin".to_string(),
    };
    let (bytes, source) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match fetch_raw_bytes(&url) {
                Some(b) => (b, url),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!("--input <file.flac> or --url <https> required");
                std::process::exit(1);
            }
        },
    };
    let epoch = match arg_value(&args, "--start-unix") {
        Some(s) => match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                eprintln!("--start-unix {s}: not a positive unix second");
                std::process::exit(1);
            }
        },
        None => match epoch_of_filename(&name_of(&source)) {
            Some(v) => v,
            None => {
                eprintln!(
                    "{source}: filename carries no YYYYMMDD_HHMMSS stamp and --start-unix is absent — the epoch stays unnamed"
                );
                std::process::exit(1);
            }
        },
    };
    let Some(decoded) = decode(&bytes) else {
        eprintln!("FLAC decode void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    if decoded.channels != 1 {
        eprintln!(
            "FLAC carries {} channels — the mono series form reads channel 0 only (0 honored)",
            decoded.channels
        );
    }
    let values: Vec<f64> = decoded.samples.iter().map(|&s| s as f64).collect();
    if values.is_empty() {
        eprintln!("FLAC decoded to zero samples — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let sample_rate = decoded.sample_rate as f64;
    let bin = write_series(&values, epoch, sample_rate);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_series(&bin) {
        Some(parsed) => {
            eprintln!(
                "flac: {source} -> {out}: {} samples @ {} Hz / {} bps, t0 {} s unix, roundtrip {} points",
                values.len(),
                decoded.sample_rate,
                decoded.bits_per_sample,
                epoch,
                parsed.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the asset stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("flac_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_reads_nrs_filename_stamp() {
        let v = epoch_of_filename("NRS01_20141016_154112.flac").unwrap();
        assert_eq!(v, 1_413_474_072.0);
        assert!(epoch_of_filename("no-stamp.flac").is_none());
        assert!(epoch_of_filename("recording.flac").is_none());
    }
}
