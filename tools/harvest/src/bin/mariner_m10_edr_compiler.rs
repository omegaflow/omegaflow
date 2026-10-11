use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mariner_m10_edr;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::{gunzip, tar_members};
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "ser.im-ldi.com";
const FORMAT: &str = "mariner_m10_edr";
const URL: &str = "https://ser.im-ldi.com/M10/MVM/mvm_0001.tgz";

const REC: usize = 968;
const LINES: usize = 700;
const PIXELS: usize = 832;
const IMAGE_BYTES: usize = 726 * REC;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn ebcdic_char(b: u8) -> u8 {
    match b {
        0x40 => b' ',
        0x4B => b'.',
        0x4D => b'(',
        0x4E => b'+',
        0x50 => b'&',
        0x5B => b'$',
        0x5C => b'*',
        0x5D => b')',
        0x61 => b'/',
        0x6B => b',',
        0x6D => b'_',
        0x7A => b':',
        0x7E => b'=',
        0x81..=0x89 => b'a' + (b - 0x81),
        0x91..=0x99 => b'j' + (b - 0x91),
        0xA2..=0xA9 => b's' + (b - 0xA2),
        0xC1..=0xC9 => b'A' + (b - 0xC1),
        0xD1..=0xD9 => b'J' + (b - 0xD1),
        0xE2..=0xE9 => b'S' + (b - 0xE2),
        0xF0..=0xF9 => b'0' + (b - 0xF0),
        _ => b' ',
    }
}

fn ebcdic_text(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| ebcdic_char(b) as char).collect()
}

fn digits(s: &str, key: &str, n: usize) -> Option<i64> {
    let i = s.find(key)? + key.len();
    s.get(i..i + n).and_then(|v| v.trim().parse().ok())
}

fn label_epoch(label: &[u8]) -> Option<f64> {
    let s = ebcdic_text(label);
    let year = digits(&s, "YR=", 2)? + 1900;
    let doy = digits(&s, "DAY=", 3)?;
    let i = s.find("GMT=")? + 4;
    let hms = s.get(i..i + 8)?;
    let hour: i64 = hms.get(0..2)?.parse().ok()?;
    let minute: i64 = hms.get(3..5)?.parse().ok()?;
    let second: i64 = hms.get(6..8)?.parse().ok()?;
    if !(1..=366).contains(&doy) || hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let days = days_from_civil(year, 1, 1)? + (doy - 1);
    let unix = (days * 86400 + hour * 3600 + minute * 60 + second) as f64;
    if unix.is_finite() { Some(unix) } else { None }
}

fn column_means(image: &[u8]) -> Vec<u16> {
    let mut sums = vec![0u64; PIXELS];
    for r in 0..LINES {
        let base = REC * (r + 1);
        for (c, sum) in sums.iter_mut().enumerate() {
            *sum += image[base + c] as u64;
        }
    }
    sums.iter().map(|&s| (s / LINES as u64) as u16).collect()
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out-bin") {
        Some(v) => v,
        None => "mariner_m10_edr.bin".to_string(),
    };

    let gz = match arg_value(args, "--in") {
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => fetch_raw_bytes(URL).ok_or_else(|| format!("{URL}: fetch void"))?,
    };
    let tar = gunzip(&gz).ok_or_else(|| format!("{URL}: gzip stream does not decode"))?;
    let members =
        tar_members(&tar).ok_or_else(|| format!("{URL}: tar stream does not enumerate"))?;

    let mut images = Vec::new();
    for m in &members {
        if !m.name.contains("IMAGES/") || m.end.saturating_sub(m.start) != IMAGE_BYTES {
            continue;
        }
        let Some(data) = tar.get(m.start..m.end) else {
            continue;
        };
        let Some(epoch) = label_epoch(&data[0..360]) else {
            continue;
        };
        images.push((epoch, column_means(data)));
    }
    if images.is_empty() {
        return Err("no images parsed — the bin stays unwritten (0 honored)".to_string());
    }
    let bin =
        mariner_m10_edr::write_bin(&images).ok_or_else(|| format!("{out}: bin does not encode"))?;
    if mariner_m10_edr::parse_bin(&bin).is_none() {
        return Err(format!("{out}: roundtrip parse void"));
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    eprintln!(
        "{out}: {} images x {} cols ({} B), format {FORMAT}, sha256 {}",
        images.len(),
        PIXELS,
        bin.len(),
        sha256_hex(&bin)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: mariner_m10_edr_compiler [--in <mvm.tgz>] [--out-bin <name>] [--ci-mode]"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("mariner_m10_edr_compiler: {msg}");
        std::process::exit(1);
    }
}
