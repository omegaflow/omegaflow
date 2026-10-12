use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mariner_m10_edr;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "planetarydata.jpl.nasa.gov";
const SHARD_PREFIX: &str = "mariner_m10_jpl_edr";
const BASE: &str = "https://planetarydata.jpl.nasa.gov/img/data/mr10-m-iss-2-edr-pv0.x/";

const VOLUMES: [(&str, [u32; 4]); 3] = [
    ("mvm_0007", [24, 25, 26, 27]),
    ("mvm_0008", [28, 29, 30, 31]),
    ("mvm_0009", [32, 33, 34, 35]),
];

const REC: usize = 968;
const LINES: usize = 700;
const PIXELS: usize = 832;
const IMAGE_BYTES: usize = 726 * REC;
const EPOCH_BYTES: usize = 8;

const SHARD_BUDGET: usize = 1 << 30;
const SHARD_LIMIT: usize = 1 << 31;

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
    let days = omegaflow::lsk::days_from_civil(year, 1, 1)? + (doy - 1);
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

fn parse_href_names(html: &str, prefix: &str) -> Vec<String> {
    let needle = "href=\"";
    let mut names = Vec::new();
    let mut rest = html;
    while let Some(i) = rest.find(needle) {
        let after = &rest[i + needle.len()..];
        let Some(end) = after.find('"') else {
            break;
        };
        let name = &after[..end];
        if name.starts_with(prefix) {
            names.push(name.to_string());
        }
        rest = &after[end + 1..];
    }
    names.sort();
    names.dedup();
    names
}

fn index_urls(text: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        urls.push(line.to_string());
    }
    urls.sort();
    urls.dedup();
    urls
}

fn enumerate(base: &str) -> Result<Vec<String>, String> {
    let mut urls = Vec::new();
    for (volume, mves) in VOLUMES {
        for mve in mves {
            let dir = format!("{base}/{volume}/mve_{mve:03}/images/");
            let html = fetch_raw_bytes(&dir)
                .and_then(|b| String::from_utf8(b).ok())
                .ok_or_else(|| format!("{dir}: listing void"))?;
            let prefix = format!("mve_{mve:03}.");
            let names = parse_href_names(&html, &prefix);
            if names.is_empty() {
                return Err(format!("{dir}: no {prefix}* image hrefs"));
            }
            for name in names {
                urls.push(format!("{dir}{name}"));
            }
        }
    }
    Ok(urls)
}

fn packed_size(count: usize, pixel_bytes: usize) -> usize {
    mariner_m10_edr::HEADER_BYTES + count * EPOCH_BYTES + pixel_bytes
}

fn shard_ranges(image_pixel_bytes: &[usize], budget: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut lo = 0usize;
    while lo < image_pixel_bytes.len() {
        let mut hi = lo + 1;
        let mut data = image_pixel_bytes[lo];
        while hi < image_pixel_bytes.len()
            && packed_size(hi + 1 - lo, data + image_pixel_bytes[hi]) <= budget
        {
            data += image_pixel_bytes[hi];
            hi += 1;
        }
        ranges.push((lo, hi));
        lo = hi;
    }
    ranges
}

fn shard_name(prefix: &str, ord: usize) -> String {
    format!("{prefix}_s{ord}.bin")
}

fn pack_and_verify(images: &[(f64, Vec<u16>)], out: &str) -> Result<Vec<u8>, String> {
    let bin =
        mariner_m10_edr::write_bin(images).ok_or_else(|| format!("{out}: bin does not encode"))?;
    let Some(parsed) = mariner_m10_edr::parse_bin(&bin) else {
        return Err(format!("{out}: roundtrip parse void"));
    };
    let expected: usize = images.iter().map(|(_, v)| v.len()).sum();
    if parsed.len() != expected {
        return Err(format!(
            "{out}: roundtrip carries {} value(s) instead of {expected}",
            parsed.len()
        ));
    }
    if let Some(parent) = std::path::Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    Ok(bin)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out-bin") {
        Some(v) => v,
        None => "mariner_m10_jpl_edr.bin".to_string(),
    };
    let base = match arg_value(args, "--base") {
        Some(v) => v,
        None => BASE.to_string(),
    };
    let urls = match arg_value(args, "--index") {
        Some(path) => {
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("read {path} returned void: {e}"))?;
            index_urls(&text)
        }
        None => enumerate(base.trim_end_matches('/').to_string().as_str())?,
    };
    if urls.is_empty() {
        return Err("no image urls — the bin stays unwritten (0 honored)".to_string());
    }

    let mut images = Vec::with_capacity(urls.len());
    for url in &urls {
        let data = fetch_raw_bytes(url).ok_or_else(|| format!("{url}: fetch void"))?;
        if data.len() != IMAGE_BYTES {
            return Err(format!(
                "{url}: {} B instead of {IMAGE_BYTES} — the bin stays unwritten (0 honored)",
                data.len()
            ));
        }
        let Some(epoch) = label_epoch(&data[0..360]) else {
            return Err(format!(
                "{url}: EBCDIC label carries no epoch — the bin stays unwritten (0 honored)"
            ));
        };
        images.push((epoch, column_means(&data)));
    }

    let pixel_per_image = PIXELS * mariner_m10_edr::PIXEL_BYTES;
    let ranges = shard_ranges(&vec![pixel_per_image; images.len()], SHARD_BUDGET);
    if ranges.len() <= 1 {
        let bin = pack_and_verify(&images, &out)?;
        eprintln!(
            "{out}: {} image(s) x {PIXELS} cols ({} B), format mariner_m10_edr, sha256 {}",
            images.len(),
            bin.len(),
            sha256_hex(&bin)
        );
        if ci_mode && !upload_release(NETLOC, &out) {
            return Err(format!("{out}: CDN upload returned void"));
        }
        return Ok(());
    }

    let mut shards: Vec<(String, String)> = Vec::new();
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let name = shard_name(SHARD_PREFIX, ord);
        let path = format!("data/{NETLOC}/{name}");
        let bin = pack_and_verify(&images[lo..hi], &path)?;
        if bin.len() > SHARD_LIMIT {
            return Err(format!(
                "{path}: {}-byte shard exceeds the {SHARD_LIMIT}-byte CDN asset limit — the bin stays unwritten (0 honored)",
                bin.len()
            ));
        }
        eprintln!(
            "{path}: {} image(s) packed ({} B), roundtrip holds",
            hi - lo,
            bin.len()
        );
        shards.push((name, path));
    }
    for (name, _) in &shards {
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format mariner_m10_edr");
        println!("at mars");
        println!();
    }
    if ci_mode {
        for (_, path) in &shards {
            if !upload_release(NETLOC, path) {
                return Err(format!("{path}: CDN upload returned void"));
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: mariner_m10_jpl_edr_compiler [--index <file>] [--base <listing-url>] [--out-bin <name>] [--ci-mode]\n\
             \n\
             Enumerates the Mariner 10 ISS EDR image directories (mve_024..035/images/) under\n\
             {BASE} and packs each 726x968 EDR file into the mariner_m10_edr bin format.\n\
             --index <file> reads one image URL per line (a measured manifest); otherwise the\n\
             plain HTML directory listings are enumerated live. Shards when the payload exceeds\n\
             {SHARD_BUDGET} B."
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("mariner_m10_jpl_edr_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_href_names_dedupes_sorts_and_keeps_the_prefix() {
        let html = "<a href=\"mve_024.010\">x</a><a href=\"mve_024.002\">y</a>\
                    <a href=\"mve_024.002\">z</a>\
                    <a href=\"/data/mr10-m-iss-2-edr-pv0.x/mvm_0007/mve_024/\">p</a>";
        assert_eq!(
            parse_href_names(html, "mve_024."),
            vec!["mve_024.002".to_string(), "mve_024.010".to_string()]
        );
        assert!(parse_href_names(html, "mve_025.").is_empty());
    }

    #[test]
    fn index_urls_skips_comments_and_blank_lines() {
        let text = "# a comment\nhttps://a/mve_024.001\n\nhttps://a/mve_024.002\n";
        assert_eq!(
            index_urls(text),
            vec![
                "https://a/mve_024.001".to_string(),
                "https://a/mve_024.002".to_string()
            ]
        );
    }

    #[test]
    fn shard_ranges_holds_one_shard_below_budget_and_splits_above() {
        let per = PIXELS * mariner_m10_edr::PIXEL_BYTES;
        assert!(shard_ranges(&[], SHARD_BUDGET).is_empty());
        assert_eq!(shard_ranges(&[per; 4], SHARD_BUDGET), vec![(0, 4)]);
        assert_eq!(
            shard_ranges(&[per; 3], 2 * per),
            vec![(0, 1), (1, 2), (2, 3)]
        );
        assert_eq!(shard_name(SHARD_PREFIX, 0), "mariner_m10_jpl_edr_s0.bin");
    }

    #[test]
    fn packed_size_carries_the_header_and_one_epoch_per_image() {
        let per = PIXELS * mariner_m10_edr::PIXEL_BYTES;
        assert_eq!(
            packed_size(2, 2 * per),
            mariner_m10_edr::HEADER_BYTES + 2 * EPOCH_BYTES + 2 * per
        );
    }

    #[test]
    fn column_means_averages_each_column_over_all_lines() {
        let mut image = vec![0u8; IMAGE_BYTES];
        for r in 0..LINES {
            let base = REC * (r + 1);
            image[base] = 10;
            image[base + 1] = 4;
        }
        let means = column_means(&image);
        assert_eq!(means.len(), PIXELS);
        assert_eq!(means[0], 10);
        assert_eq!(means[1], 4);
        assert_eq!(means[2], 0);
    }
}
