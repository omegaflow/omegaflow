use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::lsk::days_from_civil;
use omegaflow::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};

const MPCOBS_RECORD_STRIDE: usize = 50;
const UNNUMBERED: u32 = 0;

fn base62_digit(c: u8) -> Option<u32> {
    match c {
        b'0'..=b'9' => Some((c - b'0') as u32),
        b'A'..=b'Z' => Some((c - b'A' + 10) as u32),
        b'a'..=b'z' => Some((c - b'a' + 36) as u32),
        _ => None,
    }
}

fn decode_packed_number(field: &str) -> Option<u32> {
    let b = field.as_bytes();
    if field.trim().is_empty() {
        return None;
    }
    if b[0].is_ascii_digit() {
        return field.trim().parse().ok();
    }
    let hi = base62_digit(b[0])?;
    let lo: u32 = std::str::from_utf8(&b[1..]).ok()?.parse().ok()?;
    Some(hi * 10000 + lo)
}

fn ymd_to_jd(year: i64, month: i64, day_frac: f64) -> Option<f64> {
    let day = day_frac.floor();
    let days = days_from_civil(year, month, day as i64)?;
    Some(days as f64 + (day_frac - day) + 2440587.5)
}

fn parse_date(field: &str) -> Option<f64> {
    let parts: Vec<&str> = field.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    let year: i64 = parts[0].parse().ok()?;
    let month: i64 = parts[1].parse().ok()?;
    let day: f64 = parts[2].parse().ok()?;
    if !(1..=12).contains(&month) || !(1.0..32.0).contains(&day) {
        return None;
    }
    ymd_to_jd(year, month, day)
}

fn record_bytes(line: &str) -> Option<Vec<u8>> {
    let b = line.as_bytes();
    if b.len() < 80 {
        return None;
    }
    let number = match decode_packed_number(&line[0..5]) {
        Some(n) => n,
        None => UNNUMBERED,
    };
    let designation = &line[5..12];
    let epoch_jd = parse_date(&line[15..31])?;
    let ra_deg = sexagesimal_ra_to_deg(&line[32..44])?;
    let dec_deg = sexagesimal_dec_to_deg(&line[44..56])?;
    let (mag, mag_present) = match line[65..70].trim().parse::<f32>() {
        Ok(m) => (m, 1u8),
        Err(_) => (0.0, 0u8),
    };
    let band = line.as_bytes()[70];
    let band = if band.is_ascii_alphabetic() { band } else { 0 };
    let mut obs_code = [0x20u8; 3];
    obs_code.copy_from_slice(&line.as_bytes()[77..80]);
    let mut reference = [0x20u8; 6];
    reference.copy_from_slice(&line.as_bytes()[71..77]);
    let mut designation_bytes = [0x20u8; 7];
    designation_bytes.copy_from_slice(designation.as_bytes());

    let mut out = Vec::with_capacity(MPCOBS_RECORD_STRIDE);
    out.extend_from_slice(&epoch_jd.to_le_bytes());
    out.extend_from_slice(&ra_deg.to_le_bytes());
    out.extend_from_slice(&dec_deg.to_le_bytes());
    out.extend_from_slice(&mag.to_le_bytes());
    out.push(mag_present);
    out.extend_from_slice(&number.to_le_bytes());
    out.push(band);
    out.extend_from_slice(&obs_code);
    out.extend_from_slice(&reference);
    out.extend_from_slice(&designation_bytes);
    Some(out)
}

fn compile_into(input: &str, buf: &mut Vec<u8>) -> (usize, usize) {
    let packed = match std::fs::read(input) {
        Ok(p) => p,
        Err(e) => panic!("read {}: {}", input, e),
    };
    let bytes = match gunzip(&packed) {
        Some(b) => b,
        None => panic!("gunzip {}", input),
    };
    let text = String::from_utf8_lossy(&bytes);
    let mut written = 0usize;
    let mut skipped = 0usize;
    for line in text.split('\n') {
        let line = line.trim_end_matches('\r');
        match record_bytes(line) {
            Some(rec) => {
                buf.extend_from_slice(&rec);
                written += 1;
            }
            None => {
                skipped += 1;
            }
        }
    }
    (written, skipped)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "usage: mpcobs_compiler --input <observations.txt.gz> [--input ...] --out <mpcobs.bin> [--ci-mode]"
        );
        std::process::exit(1);
    }
    let mut inputs: Vec<String> = Vec::new();
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                inputs.push(args[i + 1].clone());
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    if inputs.is_empty() {
        eprintln!("--input absent");
        std::process::exit(1);
    }
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let mut buf = Vec::new();
    let mut total_written = 0usize;
    let mut total_skipped = 0usize;
    for input in &inputs {
        let (written, skipped) = compile_into(input, &mut buf);
        total_written += written;
        total_skipped += skipped;
    }
    match std::fs::write(&out_path, &buf) {
        Ok(()) => {}
        Err(e) => panic!("write {}: {}", out_path, e),
    }
    eprintln!(
        "mpcobs: {} records, {} skipped, {} B -> {}",
        total_written,
        total_skipped,
        buf.len(),
        out_path
    );
    if ci_mode && !upload_release("minorplanetcenter.net", &out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_number_decodes_plain_and_base62() {
        assert_eq!(decode_packed_number("00001"), Some(1));
        assert_eq!(decode_packed_number("99999"), Some(99999));
        assert_eq!(decode_packed_number("A0000"), Some(100000));
        assert_eq!(decode_packed_number("Z9999"), Some(359999));
        assert_eq!(decode_packed_number("a0000"), Some(360000));
        assert_eq!(decode_packed_number("     "), None);
    }

    #[test]
    fn date_parses_to_jd() {
        let jd = parse_date("2000 04 06.31600").unwrap();
        assert!((jd - 2451640.816).abs() < 1e-3, "jd {}", jd);
        let jd2 = parse_date("1970 09 29.82890").unwrap();
        assert!(jd2 > 2440000.0 && jd2 < 2450000.0);
        assert!(parse_date("     ").is_none());
    }

    #[test]
    fn record_parses_numbered_ccd_line() {
        let line =
            "00001         C2000 04 06.31600 12 21 58.174+15 23 45.06          5.96Jli0331G91";
        let rec = record_bytes(line).unwrap();
        assert_eq!(rec.len(), MPCOBS_RECORD_STRIDE);
        let epoch = f64::from_le_bytes(rec[0..8].try_into().unwrap());
        assert!((epoch - 2451640.816).abs() < 1e-3);
        let ra = f64::from_le_bytes(rec[8..16].try_into().unwrap());
        assert!((ra - (12.0 + 21.0 / 60.0 + 58.174 / 3600.0) * 15.0).abs() < 1e-6);
        let dec = f64::from_le_bytes(rec[16..24].try_into().unwrap());
        assert!((dec - (15.0 + 23.0 / 60.0 + 45.06 / 3600.0)).abs() < 1e-6);
        let mag = f32::from_le_bytes(rec[24..28].try_into().unwrap());
        assert!((mag - 5.96).abs() < 1e-3);
        assert_eq!(rec[28], 1);
        let number = u32::from_le_bytes(rec[29..33].try_into().unwrap());
        assert_eq!(number, 1);
        assert_eq!(rec[33], b'J');
        assert_eq!(&rec[34..37], b"G91");
        assert_eq!(&rec[43..50], b"       ");
    }

    #[test]
    fn record_parses_old_unnumbered_line() {
        let line =
            "00232J70S01N* A1970 09 29.82890 22 13 16.75 -12 32 43.6 J70P00R  15.5   M4635095";
        let rec = record_bytes(line).unwrap();
        assert_eq!(rec.len(), MPCOBS_RECORD_STRIDE);
        let number = u32::from_le_bytes(rec[29..33].try_into().unwrap());
        assert_eq!(number, 232);
        assert_eq!(rec[33], 0);
        assert_eq!(&rec[43..50], b"J70S01N");
        assert_eq!(&rec[34..37], b"095");
        let mag = f32::from_le_bytes(rec[24..28].try_into().unwrap());
        assert!((mag - 15.5).abs() < 1e-3);
    }
}
