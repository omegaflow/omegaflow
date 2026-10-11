use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const API: &str = "https://web-api.tp.entsoe.eu/api";
const TAG: &str = "transparency.entsoe.eu";
const KEY_NAME: &str = "ENTSOE_API_TOKEN";
const MAGIC: &[u8; 4] = b"ESOE";
const DOCUMENT_TYPE: &str = "A78";
const BUSINESS_PLANNED: u8 = 1;
const BUSINESS_FORCED: u8 = 2;
const BUSINESS_OTHER: u8 = 3;

#[derive(Clone, Debug, PartialEq)]
struct EsRecord {
    start_unix: f64,
    end_unix: f64,
    business_type_code: u8,
    domain: String,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn find_repo_root() -> Option<PathBuf> {
    if let Ok(env_root) = env::var("OMEGAFLOW_REPO") {
        if !env_root.is_empty() {
            let root = PathBuf::from(env_root);
            if root.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(root);
            }
            return None;
        }
    }
    let mut starts: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = env::current_dir() {
        starts.push(cwd);
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            starts.push(dir.to_path_buf());
        }
    }
    for start in starts {
        let mut dir = start;
        loop {
            if dir.join("phi").join("sources.\u{3c6}").is_file() {
                return Some(dir);
            }
            match dir.parent() {
                Some(parent) => dir = parent.to_path_buf(),
                None => break,
            }
        }
    }
    None
}

fn read_secret(name: &str) -> Option<String> {
    let root = find_repo_root()?;
    let text = std::fs::read_to_string(root.join(".secrets.local")).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq) = line.find('=') {
            if line[..eq].trim() == name {
                let value = line[eq + 1..].trim().trim_matches(['"', '\'']);
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .args([
            "-sSL",
            "--retry",
            "3",
            "--max-time",
            "120",
            "--connect-timeout",
            "15",
            "--write-out",
            "\n%{http_code}",
            url,
        ])
        .output()
        .map_err(|e| format!("curl invocation void: {e}"))?;
    if !out.status.success() {
        return Err(format!("curl status {:?}", out.status.code()));
    }
    let raw = out.stdout;
    let Some(split) = raw.iter().rposition(|b| *b == b'\n') else {
        return Err("response carries no status line".to_string());
    };
    let code = String::from_utf8_lossy(&raw[split + 1..])
        .trim()
        .to_string();
    let body = &raw[..split];
    if !code.starts_with('2') {
        let head = String::from_utf8_lossy(&body[..body.len().min(200)]);
        let head = head.replace('\n', " ").replace('\r', " ");
        return Err(format!("HTTP {code}: {head}"));
    }
    if body.is_empty() {
        return Err("response carries no bytes".to_string());
    }
    Ok(body.to_vec())
}

fn tag_text(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let mut search = 0usize;
    while let Some(rel) = xml[search..].find(&open) {
        let at = search + rel;
        let after = &xml[at + open.len()..];
        match after.chars().next() {
            Some('>') | Some(' ') | Some('\t') | Some('\n') | Some('\r') => {}
            _ => {
                search = at + 1;
                continue;
            }
        }
        let gt = xml[at..].find('>')?;
        let text_start = at + gt + 1;
        let lt = xml[text_start..].find('<')?;
        let text = xml[text_start..text_start + lt].trim();
        if text.is_empty() {
            return None;
        }
        return Some(text.to_string());
    }
    None
}

fn time_series_blocks(xml: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut search = 0usize;
    while let Some(rel) = xml[search..].find("<TimeSeries") {
        let start = search + rel;
        let after = &xml[start + "<TimeSeries".len()..];
        match after.chars().next() {
            Some('>') | Some(' ') | Some('\t') | Some('\n') | Some('\r') | Some('/') => {}
            _ => {
                search = start + 1;
                continue;
            }
        }
        let Some(rel_end) = xml[start..].find("</TimeSeries>") else {
            break;
        };
        let end = start + rel_end + "</TimeSeries>".len();
        out.push(&xml[start..end]);
        search = end;
    }
    out
}

fn parse_date(date: &str) -> Option<(i64, i64, i64)> {
    let mut parts = date.trim().split('-');
    let year = parts.next()?.parse::<i64>().ok()?;
    let month = parts.next()?.parse::<i64>().ok()?;
    let day = parts.next()?.parse::<i64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some((year, month, day))
}

fn parse_time(time: &str) -> Option<(i64, i64, i64)> {
    let t = time.trim().trim_end_matches('Z');
    let mut parts = t.split(':');
    let hour = parts.next()?.parse::<i64>().ok()?;
    let minute = parts.next()?.parse::<i64>().ok()?;
    let second = match parts.next() {
        Some(s) => s.parse::<i64>().ok()?,
        None => 0,
    };
    if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) || !(0..=59).contains(&second) {
        return None;
    }
    Some((hour, minute, second))
}

fn parse_datetime(date: &str, time: &str) -> Option<f64> {
    let (year, month, day) = parse_date(date)?;
    let (hour, minute, second) = parse_time(time)?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64)
}

fn parse_series(block: &str) -> Option<EsRecord> {
    let business_type_code = match tag_text(block, "businessType").as_deref() {
        Some("A53") => BUSINESS_PLANNED,
        Some("A54") => BUSINESS_FORCED,
        _ => BUSINESS_OTHER,
    };
    let domain = tag_text(block, "biddingZone_Domain.mRID")?;
    let start_date = tag_text(block, "start_DateAndOrTime.date")?;
    let start_time = tag_text(block, "start_DateAndOrTime.time")?;
    let end_date = tag_text(block, "end_DateAndOrTime.date")?;
    let end_time = tag_text(block, "end_DateAndOrTime.time")?;
    let start_unix = parse_datetime(&start_date, &start_time)?;
    let end_unix = parse_datetime(&end_date, &end_time)?;
    Some(EsRecord {
        start_unix,
        end_unix,
        business_type_code,
        domain,
    })
}

fn parse_document(xml: &str) -> Vec<EsRecord> {
    let mut out = Vec::new();
    for block in time_series_blocks(xml) {
        if let Some(record) = parse_series(block) {
            out.push(record);
        }
    }
    out
}

fn compact_period(date: &str) -> Option<String> {
    let (year, month, day) = parse_date(date)?;
    Some(format!("{year:04}{month:02}{day:02}0000"))
}

fn write_bin(records: &[EsRecord]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for record in records {
        out.extend_from_slice(&record.start_unix.to_le_bytes());
        out.extend_from_slice(&record.end_unix.to_le_bytes());
        out.push(record.business_type_code);
        let bytes = record.domain.as_bytes();
        let len = bytes.len().min(u16::MAX as usize) as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&bytes[..len as usize]);
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<EsRecord>> {
    if bytes.len() < 8 || &bytes[0..4] != &MAGIC[..] {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = 8usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        if off + 17 > bytes.len() {
            return None;
        }
        let start_unix = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        off += 8;
        let end_unix = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        off += 8;
        let business_type_code = bytes[off];
        off += 1;
        let domain_len = u16::from_le_bytes(bytes[off..off + 2].try_into().ok()?) as usize;
        off += 2;
        if off + domain_len > bytes.len() {
            return None;
        }
        let domain = String::from_utf8(bytes[off..off + domain_len].to_vec()).ok()?;
        off += domain_len;
        out.push(EsRecord {
            start_unix,
            end_unix,
            business_type_code,
            domain,
        });
    }
    Some(out)
}

fn build_url(token: &str, domain: &str, period_start: &str, period_end: &str) -> String {
    format!(
        "{API}?securityToken={token}&documentType={DOCUMENT_TYPE}&in_Domain={domain}&periodStart={period_start}&periodEnd={period_end}"
    )
}

fn usage() -> String {
    "entsoe_compiler --domain <EIC> [--domain <EIC>...] --start YYYY-MM-DD --stop YYYY-MM-DD [--out-bin name] [--ci-mode]".to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let domains = arg_values(&args, "--domain");
    if domains.is_empty() {
        eprintln!("entsoe_compiler: --domain <EIC> required (in_Domain is never fabricated)");
        eprintln!("usage: {}", usage());
        std::process::exit(2);
    }

    let Some(start_arg) = arg_value(&args, "--start") else {
        eprintln!("entsoe_compiler: --start <YYYY-MM-DD> required");
        eprintln!("usage: {}", usage());
        std::process::exit(2);
    };
    let Some(stop_arg) = arg_value(&args, "--stop") else {
        eprintln!("entsoe_compiler: --stop <YYYY-MM-DD> required");
        eprintln!("usage: {}", usage());
        std::process::exit(2);
    };
    let Some(period_start) = compact_period(&start_arg) else {
        eprintln!("entsoe_compiler: --start {start_arg} is no YYYY-MM-DD date");
        std::process::exit(2);
    };
    let Some(period_end) = compact_period(&stop_arg) else {
        eprintln!("entsoe_compiler: --stop {stop_arg} is no YYYY-MM-DD date");
        std::process::exit(2);
    };

    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => "entsoe_a78.bin".to_string(),
    };

    let Some(token) = read_secret(KEY_NAME) else {
        eprintln!("{KEY_NAME} absent");
        std::process::exit(1);
    };

    let mut records: Vec<EsRecord> = Vec::new();
    for domain in &domains {
        let url = build_url(&token, domain, &period_start, &period_end);
        match fetch(&url) {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let series = parse_document(&text);
                eprintln!("{domain}: {} TimeSeries parsed", series.len());
                records.extend(series);
            }
            Err(msg) => {
                eprintln!("{domain}: fetch void — {msg}, the domain stays unharvested");
            }
        }
    }

    if records.is_empty() {
        eprintln!("entsoe_a78: no TimeSeries parsed — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.start_unix.total_cmp(&b.start_unix));

    let bytes = write_bin(&records);
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "{out_bin}: {} unavailability records, {} B, roundtrip parses",
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{out_bin}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(TAG, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "<Unavailability_MarketDocument>\
<unavailability_Time_Period.timeInterval><start>2024-01-01T00:00Z</start><end>2024-02-01T00:00Z</end></unavailability_Time_Period.timeInterval>\
<TimeSeries><mRID>1</mRID><businessType>A54</businessType>\
<biddingZone_Domain.mRID codingScheme=\"A01\">10YFI-1--------U</biddingZone_Domain.mRID>\
<start_DateAndOrTime.date>2024-01-05</start_DateAndOrTime.date>\
<start_DateAndOrTime.time>08:30:00Z</start_DateAndOrTime.time>\
<end_DateAndOrTime.date>2024-01-05</end_DateAndOrTime.date>\
<end_DateAndOrTime.time>12:00:00Z</end_DateAndOrTime.time>\
<quantity_Measure_Unit.name>MAW</quantity_Measure_Unit.name></TimeSeries>\
<TimeSeries><businessType>A53</businessType>\
<biddingZone_Domain.mRID codingScheme=\"A01\">10YFI-1--------U</biddingZone_Domain.mRID>\
<start_DateAndOrTime.date>2024-01-10</start_DateAndOrTime.date>\
<start_DateAndOrTime.time>00:00:00Z</start_DateAndOrTime.time>\
<end_DateAndOrTime.date>2024-01-10</end_DateAndOrTime.date>\
<end_DateAndOrTime.time>06:15:00Z</end_DateAndOrTime.time></TimeSeries>\
</Unavailability_MarketDocument>";

    #[test]
    fn parses_two_time_series_with_codes() {
        let records = parse_document(SAMPLE);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].business_type_code, 2);
        assert_eq!(records[1].business_type_code, 1);
        assert_eq!(records[0].domain, "10YFI-1--------U");
        let expected =
            days_from_civil(2024, 1, 5).unwrap() as f64 * 86400.0 + 8.0 * 3600.0 + 30.0 * 60.0;
        assert!((records[0].start_unix - expected).abs() < 1e-9);
    }

    #[test]
    fn a_missing_tag_yields_none_not_zero() {
        let block = "<TimeSeries><businessType>A54</businessType></TimeSeries>";
        assert!(parse_series(block).is_none());
        assert!(tag_text(block, "biddingZone_Domain.mRID").is_none());
    }

    #[test]
    fn bin_roundtrip_restores_the_records() {
        let records = parse_document(SAMPLE);
        let bytes = write_bin(&records);
        assert_eq!(&bytes[0..4], &MAGIC[..]);
        let parsed = parse_bin(&bytes).expect("roundtrip parses");
        assert_eq!(parsed, records);
    }

    #[test]
    fn a_foreign_magic_yields_none() {
        assert!(parse_bin(b"XXXX\x00\x00\x00\x00").is_none());
    }

    #[test]
    fn compact_period_is_yyyymmddhhmm() {
        assert_eq!(
            compact_period("2024-01-05").as_deref(),
            Some("202401050000")
        );
        assert!(compact_period("2024-13-05").is_none());
    }
}
