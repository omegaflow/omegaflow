use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const NETLOC: &str = "data.phys.ucalgary.ca";
const BASE: &str = "https://data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream2";
const COMPILER: &str = "tools/harvest/src/bin/themis_asi_keogram_compiler.rs";
const FORMAT: &str = "themis_asi_keogram";
const TTL: u64 = 604800;
const OUT_DIR: &str = "data/data.phys.ucalgary.ca";
const SUFFIX: &str = "_full-keogram.pgm.jpg";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Date {
    year: i64,
    month: i64,
    day: i64,
    days: i64,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
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

fn parse_date(s: &str) -> Option<Date> {
    if s.len() != 8 || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(4..6)?.parse().ok()?;
    let day: i64 = s.get(6..8)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let (cy, cm, cd) = civil_from_days(days)?;
    if cy as i64 != year || cm as i64 != month || cd as i64 != day {
        return None;
    }
    Some(Date {
        year,
        month,
        day,
        days,
    })
}

fn ymd(d: Date) -> String {
    format!("{:04}{:02}{:02}", d.year, d.month, d.day)
}

fn day_url(d: Date) -> String {
    format!("{BASE}/{:04}/{:02}/{:02}/", d.year, d.month, d.day)
}

fn keogram_url(label: &str, d: Date) -> String {
    format!(
        "{BASE}/{:04}/{:02}/{:02}/{label}/{}__{label}{SUFFIX}",
        d.year,
        d.month,
        d.day,
        ymd(d)
    )
}

fn resolve_label(index_html: &str, station: &str) -> Option<String> {
    let needle = format!("{station}_themis");
    let start = index_html.find(&needle)?;
    let rest = &index_html[start..];
    let end = rest.find('/')?;
    if end == 0 {
        return None;
    }
    Some(rest[..end].to_string())
}

fn label_for(station: &str, d: Date) -> Option<String> {
    if station.contains('_') {
        return Some(station.to_string());
    }
    let url = day_url(d);
    let bytes = fetch_raw_bytes(&url)?;
    let html = String::from_utf8_lossy(&bytes);
    let label = resolve_label(&html, station)?;
    eprintln!("{station}: {url} -> {label}");
    Some(label)
}

fn emit(name: &str, origin: &str, sha: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("at earth");
    println!("ttl {TTL}");
    println!("sha256 {sha}");
    println!();
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => OUT_DIR.to_string(),
    };

    let mut stations: Vec<String> = arg_values(args, "--station");
    if stations.is_empty() {
        return Err(
            "--station <label|code> is absent — the keogram stays unfetched (0 honored)"
                .to_string(),
        );
    }
    stations.sort();
    stations.dedup();

    let mut dates: Vec<Date> = Vec::new();
    for s in arg_values(args, "--date") {
        let d = parse_date(&s).ok_or_else(|| format!("--date {s} is not YYYYMMDD"))?;
        dates.push(d);
    }
    if dates.is_empty() {
        match (arg_value(args, "--start"), arg_value(args, "--stop")) {
            (Some(s), Some(e)) => {
                let start = parse_date(&s).ok_or_else(|| format!("--start {s} is not YYYYMMDD"))?;
                let stop = parse_date(&e).ok_or_else(|| format!("--stop {e} is not YYYYMMDD"))?;
                if stop.days < start.days {
                    return Err("--stop lies before --start".to_string());
                }
                let mut day = start.days;
                while day <= stop.days {
                    let (y, m, dd) =
                        civil_from_days(day).ok_or("civil_from_days returned void")?;
                    dates.push(Date {
                        year: y as i64,
                        month: m as i64,
                        day: dd as i64,
                        days: day,
                    });
                    day += 1;
                }
            }
            _ => {
                return Err(
                    "--date <YYYYMMDD> or --start/--stop <YYYYMMDD> is absent — the keogram stays unfetched (0 honored)"
                        .to_string(),
                )
            }
        }
    }
    dates.sort_by_key(|d| d.days);
    dates.dedup_by_key(|d| d.days);

    std::fs::create_dir_all(&out).map_err(|e| format!("create {out} void: {e}"))?;

    let mut written = 0usize;
    for d in &dates {
        for station in &stations {
            let Some(label) = label_for(station, *d) else {
                eprintln!(
                    "{station} {}: label not in the day index — night skipped (0 honored)",
                    ymd(*d)
                );
                continue;
            };
            let url = keogram_url(&label, *d);
            let Some(bytes) = fetch_raw_bytes(&url) else {
                eprintln!("{url}: fetch void — night skipped (0 honored)");
                continue;
            };
            if bytes.is_empty() {
                eprintln!("{url}: measured 0 bytes — night skipped (0 honored)");
                continue;
            }
            let name = format!("{}__{label}{SUFFIX}", ymd(*d));
            let path = format!("{out}/{name}");
            std::fs::write(&path, &bytes)
                .map_err(|e| format!("write {path} returned void: {e}"))?;
            let sha = sha256_hex(&bytes);
            eprintln!("{name}: {} B, sha256 {sha}", bytes.len());
            emit(&name, &url, &sha);
            if ci_mode && !upload_release(NETLOC, &path) {
                return Err(format!("{path}: CDN upload returned void"));
            }
            written += 1;
        }
    }

    if written == 0 {
        return Err("no keogram measured — the asset stays unwritten (0 honored)".to_string());
    }
    eprintln!("{written} keogram image(s) written under {out}");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: themis_asi_keogram_compiler --station <label|code>... (--date <YYYYMMDD>... | --start <YYYYMMDD> --stop <YYYYMMDD>) [--out <dir>] [--ci-mode]"
        );
        eprintln!(
            "  --station  archive platform label (e.g. atha_themis02) or bare station code (e.g. atha, resolved from the day index)"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("themis_asi_keogram_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_reads_yyyymmdd_and_refuses_impossible_days() {
        let d = parse_date("20261009").expect("a measured date parses");
        assert_eq!((d.year, d.month, d.day), (2026, 10, 9));
        assert_eq!(parse_date("20260230"), None);
        assert_eq!(parse_date("2026-10-09"), None);
        assert_eq!(parse_date("2026100"), None);
    }

    #[test]
    fn keogram_url_matches_the_measured_template() {
        let d = parse_date("20261009").expect("date parses");
        assert_eq!(
            keogram_url("atha_themis02", d),
            "https://data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream2/2026/10/09/atha_themis02/20261009__atha_themis02_full-keogram.pgm.jpg"
        );
        let h = parse_date("20220131").expect("date parses");
        assert_eq!(
            keogram_url("fsim_themis08", h),
            "https://data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream2/2022/01/31/fsim_themis08/20220131__fsim_themis08_full-keogram.pgm.jpg"
        );
    }

    #[test]
    fn resolve_label_finds_the_platform_directory_for_a_bare_code() {
        let html = r#"<a href="../">../</a><a href="atha_themis02/">atha_themis02/</a><a href="fsim_themis08/">fsim_themis08/</a>"#;
        assert_eq!(
            resolve_label(html, "atha").as_deref(),
            Some("atha_themis02")
        );
        assert_eq!(
            resolve_label(html, "fsim").as_deref(),
            Some("fsim_themis08")
        );
        assert_eq!(resolve_label(html, "zzzz"), None);
    }

    #[test]
    fn a_full_label_skips_the_index_lookup() {
        assert!(label_for("atha_themis02", parse_date("20261009").unwrap()).is_some());
    }
}
