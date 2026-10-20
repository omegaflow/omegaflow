use omegaflow::archivar::asset_path::{compiled, raw};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::lsk::days_from_civil;
use omegaflow::soho_eit::{parse_bin, write_bin};

const NETLOC: &str = "sun.stanford.edu";
const DEFAULT_OUT: &str = "soho_eit_195.bin";
const FITS_BZERO_ABSENT: f64 = 0.0;
const BASE: &str = "http://sun.stanford.edu/synop/EIT/";
const CR_ANCHOR: i64 = 1908;
const CR_ANCHOR_DAY: i64 = 9607;
const CARRINGTON_DAYS: f64 = 27.27638;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn carrington_day(cr: i64) -> i64 {
    CR_ANCHOR_DAY + (((cr - CR_ANCHOR) as f64) * CARRINGTON_DAYS).round() as i64
}

fn parse_tobs(s: &str) -> Option<i64> {
    let date = s.split('_').next()?;
    let mut parts = date.split('.');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    days_from_civil(year, month, day)
}

fn parse_index(html: &str) -> Vec<(i64, String)> {
    let mut out: Vec<(i64, String)> = Vec::new();
    let mut rest = html;
    while let Some(i) = rest.find("195A/CR") {
        rest = &rest[i + 5..];
        let digits: String = rest
            .chars()
            .skip(2)
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if digits.is_empty() {
            continue;
        }
        let after = &rest[2 + digits.len()..];
        let Some(stem) = after.strip_prefix("_195A_") else {
            continue;
        };
        let Some(suffix) = stem.chars().next() else {
            continue;
        };
        if !stem.starts_with(&format!("{suffix}.fits")) || !matches!(suffix, 'a' | 'c') {
            continue;
        }
        let Ok(cr) = digits.parse::<i64>() else {
            continue;
        };
        if out.iter().any(|(c, _)| *c == cr) {
            continue;
        }
        out.push((cr, format!("195A/CR{cr}_195A_{suffix}.fits")));
    }
    out.sort_by_key(|(cr, _)| *cr);
    out
}

fn map_mean(bytes: &[u8]) -> Option<f64> {
    let (img, _) = FitsImage::parse(bytes, 0)?;
    let blank = FitsHeader::parse(bytes, 0).and_then(|(h, _)| {
        let b = h.int("BLANK")?;
        let scale = h.f64("BSCALE").unwrap_or(1.0);
        let zero = h.f64("BZERO").unwrap_or(FITS_BZERO_ABSENT);
        Some(b as f64 * scale + zero)
    });
    let (w, h) = (img.dims[0], img.dims[1]);
    if w == 0 || h == 0 {
        return None;
    }
    let mut sum = 0.0f64;
    let mut n = 0usize;
    for y in 0..h {
        for x in 0..w {
            let Some(v) = img.value_f64(bytes, [x, y, 0]) else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            if let Some(b) = blank {
                if v == b {
                    continue;
                }
            }
            sum += v;
            n += 1;
        }
    }
    if n < 4 {
        return None;
    }
    Some(sum / n as f64)
}

fn map_day(bytes: &[u8], cr: i64) -> i64 {
    let stamped = FitsHeader::parse(bytes, 0)
        .and_then(|(h, _)| h.str_unescaped("T_OBS"))
        .and_then(|s| parse_tobs(&s));
    match stamped {
        Some(day) => day,
        None => carrington_day(cr),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            let path = compiled(NETLOC, DEFAULT_OUT);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let cache_dir = match arg_value(&args, "--cache-dir") {
        Some(v) => v,
        None => raw(NETLOC, "cache/eit").to_string_lossy().into_owned(),
    };
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{cache_dir}: cache dir stays uncreatable");
        std::process::exit(1);
    }

    let Some(index) = fetch_raw_bytes(BASE) else {
        eprintln!("EIT index void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let html = String::from_utf8_lossy(&index);
    let mut maps = parse_index(&html);
    if maps.is_empty() {
        eprintln!("EIT index carries no 195A link — the series stays unwritten");
        std::process::exit(1);
    }
    if let Some(limit) = arg_value(&args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        maps.truncate(limit);
    }
    eprintln!("EIT 195A: {} synoptic charts", maps.len());

    let mut records: Vec<(i64, f64)> = Vec::new();
    let mut skipped = 0usize;
    for (cr, path) in &maps {
        let cache_path = format!("{}/{}.fits", cache_dir, cr);
        let bytes = match std::fs::read(&cache_path) {
            Ok(b) => Some(b),
            Err(_) => {
                let got = fetch_raw_bytes(&format!("{BASE}{path}"));
                if let Some(b) = &got {
                    let _ = std::fs::write(&cache_path, b);
                }
                got
            }
        };
        let Some(bytes) = bytes else {
            eprintln!("CR{cr}: fetch void");
            skipped += 1;
            continue;
        };
        let Some(mean) = map_mean(&bytes) else {
            eprintln!("CR{cr}: no finite pixels");
            skipped += 1;
            continue;
        };
        records.push((map_day(&bytes, *cr), mean));
    }
    records.sort_by_key(|(d, _)| *d);
    records.dedup_by_key(|(d, _)| *d);
    eprintln!(
        "EIT 195A mean: {} records, {} skipped (absent)",
        records.len(),
        skipped
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "{out}: {} records, roundtrip parses ({} B)",
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(key: &str, value: &str) -> String {
        let mut s = format!("{key:<8}= {value:>20}");
        while s.len() < 80 {
            s.push(' ');
        }
        s
    }

    fn fits_fixture(values: &[f32], w: usize, h: usize) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&card("SIMPLE", "T"));
        out.push_str(&card("BITPIX", "-32"));
        out.push_str(&card("NAXIS", "2"));
        out.push_str(&card("NAXIS1", &w.to_string()));
        out.push_str(&card("NAXIS2", &h.to_string()));
        out.push_str(&card("T_OBS", "'2001.04.23_02:20:25_TAI'"));
        out.push_str(&card("END", ""));
        while out.len() % 2880 != 0 {
            out.push(' ');
        }
        let mut bytes = out.into_bytes();
        for v in values {
            bytes.extend_from_slice(&v.to_be_bytes());
        }
        bytes
    }

    #[test]
    fn index_keeps_the_first_link_per_rotation() {
        let html = r#"
            <a href="195A/CR1911_195A_a.fits">FITS</a>
            <a href="195A/CR1912_195A_a.fits">FITS</a>
            <a href="195A/CR2055_195A_c.fits">FITS</a>
            <a href="171A/CR1911_171A_a.fits">FITS</a>
            <a href="195A/CR1911_195A_a.gif">GIF</a>
        "#;
        let maps = parse_index(html);
        assert_eq!(
            maps,
            vec![
                (1911, "195A/CR1911_195A_a.fits".to_string()),
                (1912, "195A/CR1912_195A_a.fits".to_string()),
                (2055, "195A/CR2055_195A_c.fits".to_string()),
            ]
        );
    }

    #[test]
    fn carrington_day_matches_the_jsoc_stamps() {
        assert_eq!(carrington_day(1908), days_from_civil(1996, 4, 21).unwrap());
        assert_eq!(carrington_day(1911), days_from_civil(1996, 7, 12).unwrap());
        assert_eq!(carrington_day(2107), days_from_civil(2011, 3, 2).unwrap());
    }

    #[test]
    fn map_mean_skips_nan_and_the_trailing_padding() {
        let values = [1.0f32, 2.0, f32::NAN, 4.0, 5.0, 6.0];
        let bytes = fits_fixture(&values, 3, 2);
        let mean = map_mean(&bytes).unwrap();
        assert!((mean - 3.6).abs() < 1e-6, "mean {mean}");
        assert_eq!(map_day(&bytes, 1975), days_from_civil(2001, 4, 23).unwrap());
    }

    #[test]
    fn map_mean_rejects_an_all_nan_map() {
        let bytes = fits_fixture(&[f32::NAN; 6], 3, 2);
        assert!(map_mean(&bytes).is_none());
    }
}
