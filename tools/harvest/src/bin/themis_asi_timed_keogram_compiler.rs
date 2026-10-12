use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::themis_asi;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::lsk::days_from_civil;
use omegaflow::spectral::civil_from_days;

const NETLOC: &str = "data.phys.ucalgary.ca";
const BASE: &str = "https://data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream0";
const COMPILER: &str = "tools/harvest/src/bin/themis_asi_timed_keogram_compiler.rs";
const FORMAT: &str = "themis_asi_timed_keogram";
const TTL: u64 = 604800;
const SUFFIX: &str = "_full.pgm.gz";

const FRAME_W: usize = 256;
const FRAME_H: usize = 256;
const FRAME_BYTES: usize = FRAME_W * FRAME_H * 2;

const BAND_COL0: usize = 120;
const BAND_COL1: usize = 135;
const BAND_COLS: usize = BAND_COL1 - BAND_COL0 + 1;

const MAGIC: &[u8; 4] = b"TKDG";
const DAY_S: f64 = 86400.0;
const HOUR_S: f64 = 3600.0;
const PGM_MAXVAL: u32 = 65535;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Date {
    year: i64,
    month: i64,
    day: i64,
    days: i64,
}

struct Frame {
    epoch: f64,
    profile: Vec<u16>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn station_url(label: &str, d: Date) -> String {
    format!("{BASE}/{:04}/{:02}/{:02}/{label}/", d.year, d.month, d.day)
}

fn file_url(hour_url: &str, name: &str) -> String {
    format!("{hour_url}{name}")
}

fn origin_template() -> &'static str {
    "https://data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream0/<YYYY>/<MM>/<DD>/<label>/ut<HH>/<YYYYMMDD>_<HHMM>_<label>_full.pgm.gz"
}

fn href_names(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(i) = rest.find("href=\"") {
        let after = &rest[i + 6..];
        match after.find('"') {
            Some(j) => {
                out.push(after[..j].to_string());
                rest = &after[j + 1..];
            }
            None => break,
        }
    }
    out
}

fn hour_dirs(html: &str, start_hh: Option<u32>, stop_hh: Option<u32>) -> Vec<String> {
    let mut out = Vec::new();
    for href in href_names(html) {
        let Some(name) = href.strip_suffix('/') else {
            continue;
        };
        if name.len() != 4 || !name.starts_with("ut") {
            continue;
        }
        let Ok(hh) = name[2..].parse::<u32>() else {
            continue;
        };
        if let Some(s) = start_hh {
            if hh < s {
                continue;
            }
        }
        if let Some(e) = stop_hh {
            if hh > e {
                continue;
            }
        }
        out.push(name.to_string());
    }
    out.sort();
    out.dedup();
    out
}

fn pgm_files(html: &str) -> Vec<String> {
    let mut out: Vec<String> = href_names(html)
        .into_iter()
        .filter(|h| h.ends_with(SUFFIX))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn minute_token(name: &str) -> Option<u32> {
    let tok = name.get(9..13)?;
    if !tok.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    tok.parse().ok()
}

fn parse_hhmm(s: &str) -> Option<u32> {
    if s.len() != 4 || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
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

fn read_line(data: &[u8], pos: &mut usize, name: &str, index: usize) -> Result<String, String> {
    let start = *pos;
    let mut end = start;
    while end < data.len() && data[end] != b'\n' {
        end += 1;
    }
    if end >= data.len() {
        return Err(format!(
            "{name}: sub-frame {index} header line unterminated at byte {start}"
        ));
    }
    let line = &data[start..end];
    *pos = end + 1;
    String::from_utf8(line.to_vec())
        .map_err(|_| format!("{name}: sub-frame {index} header line is not UTF-8 at byte {start}"))
}

fn parse_utc(line: &str) -> Option<f64> {
    if !line.contains("Image request start") {
        return None;
    }
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks.len() < 3 || toks[toks.len() - 1] != "UTC" {
        return None;
    }
    let date = toks[toks.len() - 3];
    let time = toks[toks.len() - 2];
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: f64 = t.next()?.parse().ok()?;
    if !(1..=12).contains(&mo) || !(1..=31).contains(&da) {
        return None;
    }
    let days = days_from_civil(y, mo, da)?;
    Some(days as f64 * DAY_S + hh as f64 * HOUR_S + mm as f64 * 60.0 + ss)
}

fn profile_of(block: &[u8]) -> Vec<u16> {
    let mut profile = Vec::with_capacity(FRAME_H);
    for row in 0..FRAME_H {
        let mut sum: u32 = 0;
        for col in BAND_COL0..=BAND_COL1 {
            let i = (row * FRAME_W + col) * 2;
            let value = ((block[i] as u16) << 8) | block[i + 1] as u16;
            sum += value as u32;
        }
        profile.push(((sum + BAND_COLS as u32 / 2) / BAND_COLS as u32) as u16);
    }
    profile
}

fn parse_frames(data: &[u8], name: &str) -> Result<Vec<Frame>, String> {
    let mut frames = Vec::new();
    let mut pos = 0usize;
    let mut index = 0usize;
    while pos < data.len() {
        index += 1;
        if data.len() - pos < 2 || &data[pos..pos + 2] != b"P5" {
            return Err(format!(
                "{name}: sub-frame {index} does not begin with P5 at byte {pos}"
            ));
        }
        pos += 2;
        if pos < data.len() && data[pos] == b'\n' {
            pos += 1;
        }
        let mut dims: Option<(usize, usize)> = None;
        let mut maxval: Option<u32> = None;
        let mut stamp: Option<f64> = None;
        while maxval.is_none() {
            let line = read_line(data, &mut pos, name, index)?;
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                if let Some(t) = parse_utc(trimmed) {
                    stamp = Some(t);
                }
                continue;
            }
            if trimmed.is_empty() {
                continue;
            }
            if dims.is_none() {
                let mut it = trimmed.split_whitespace();
                let w: usize = it.next().and_then(|v| v.parse().ok()).ok_or_else(|| {
                    format!("{name}: sub-frame {index} dimensions line unreadable: {trimmed}")
                })?;
                let h: usize = it.next().and_then(|v| v.parse().ok()).ok_or_else(|| {
                    format!("{name}: sub-frame {index} dimensions line unreadable: {trimmed}")
                })?;
                dims = Some((w, h));
            } else {
                let v: u32 = trimmed.parse().map_err(|_| {
                    format!("{name}: sub-frame {index} maxval unreadable: {trimmed}")
                })?;
                maxval = Some(v);
            }
        }
        let (w, h) =
            dims.ok_or_else(|| format!("{name}: sub-frame {index} carries no dimensions"))?;
        let mv = maxval.ok_or_else(|| format!("{name}: sub-frame {index} carries no maxval"))?;
        if w != FRAME_W || h != FRAME_H || mv != PGM_MAXVAL {
            return Err(format!(
                "{name}: sub-frame {index} geometry {w}x{h} maxval {mv} differs from {FRAME_W}x{FRAME_H} maxval {PGM_MAXVAL}"
            ));
        }
        let epoch = stamp.ok_or_else(|| {
            format!("{name}: sub-frame {index} carries no `Image request start` UTC stamp")
        })?;
        if data.len() - pos < FRAME_BYTES {
            return Err(format!(
                "{name}: sub-frame {index} pixel block truncated: {} of {FRAME_BYTES} bytes",
                data.len() - pos
            ));
        }
        let profile = profile_of(&data[pos..pos + FRAME_BYTES]);
        pos += FRAME_BYTES;
        frames.push(Frame { epoch, profile });
    }
    Ok(frames)
}

fn write_tkdg(frames: &[Frame]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + frames.len() * (8 + FRAME_H * 2));
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    for f in frames {
        out.extend_from_slice(&f.epoch.to_le_bytes());
        for v in &f.profile {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn emit(name: &str, origin: &str, sha: &str, station: &themis_asi::AsiStation, alt_m: f64) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("station {}", station.code);
    println!("on earth {} {} {alt_m}", station.lat_deg, station.lon_deg);
    println!("ttl {TTL}");
    println!("sha256 {sha}");
    println!();
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let station = arg_value(args, "--station").ok_or(
        "--station <label|code> is absent — the timed keogram stays unfetched (0 honored)",
    )?;
    let date_str = arg_value(args, "--date")
        .ok_or("--date <YYYYMMDD> is absent — the timed keogram stays unfetched (0 honored)")?;
    let d = parse_date(&date_str).ok_or_else(|| format!("--date {date_str} is not YYYYMMDD"))?;

    let start = arg_value(args, "--start");
    let stop = arg_value(args, "--stop");
    if start.is_some() != stop.is_some() {
        return Err("--start and --stop stand together or not at all".to_string());
    }
    let (start_min, stop_min) = match (&start, &stop) {
        (Some(s), Some(e)) => {
            let sv = parse_hhmm(s).ok_or_else(|| format!("--start {s} is not HHMM"))?;
            let ev = parse_hhmm(e).ok_or_else(|| format!("--stop {e} is not HHMM"))?;
            if ev < sv {
                return Err("--stop lies before --start".to_string());
            }
            (Some(sv), Some(ev))
        }
        _ => (None, None),
    };
    let start_hh = start_min.map(|v| v / 100);
    let stop_hh = stop_min.map(|v| v / 100);

    let out = arg_value(args, "--out-bin")
        .ok_or_else(|| "--out-bin is absent — the asset stays unwritten (0 honored)".to_string())?;

    let label = label_for(&station, d).ok_or_else(|| {
        format!(
            "{station} {}: label not in the day index — night skipped (0 honored)",
            ymd(d)
        )
    })?;

    let code = label.split("_themis").next().unwrap_or(label.as_str());
    let st = themis_asi::station(code).ok_or_else(|| {
        format!("{station}: not in the ASI station table — night skipped (0 honored)")
    })?;
    let alt_m = st
        .alt_m
        .ok_or_else(|| format!("{station}: alt pending — night skipped (0 honored)"))?;

    let base = station_url(&label, d);
    let station_html = fetch_raw_bytes(&base)
        .ok_or_else(|| format!("{base}: station index fetch void (0 honored)"))?;
    let station_text = String::from_utf8_lossy(&station_html);
    let hours = hour_dirs(&station_text, start_hh, stop_hh);
    if hours.is_empty() {
        return Err(format!(
            "{base}: no ut<HH> directory in the requested window — night skipped (0 honored)"
        ));
    }

    let mut frames: Vec<Frame> = Vec::new();
    let mut fetched = 0usize;
    for ut in &hours {
        let hour_url = format!("{base}{ut}/");
        let Some(hour_html) = fetch_raw_bytes(&hour_url) else {
            eprintln!("{hour_url}: index fetch void — hour skipped (0 honored)");
            continue;
        };
        let hour_text = String::from_utf8_lossy(&hour_html);
        for name in pgm_files(&hour_text) {
            if let (Some(sv), Some(ev)) = (start_min, stop_min) {
                let Some(bit) = minute_token(&name) else {
                    continue;
                };
                if bit < sv || bit > ev {
                    continue;
                }
            }
            let url = file_url(&hour_url, &name);
            let Some(gz) = fetch_raw_bytes(&url) else {
                eprintln!("{url}: fetch void — minute skipped (0 honored)");
                continue;
            };
            let Some(pgm) = gunzip(&gz) else {
                eprintln!("{url}: gunzip void — minute skipped (0 honored)");
                continue;
            };
            match parse_frames(&pgm, &url) {
                Ok(mut f) => {
                    fetched += 1;
                    frames.append(&mut f);
                }
                Err(msg) => {
                    eprintln!("{url}: {msg} — minute skipped (0 honored)");
                }
            }
        }
    }

    if frames.is_empty() {
        return Err("no sub-frame measured — the asset stays unwritten (0 honored)".to_string());
    }
    frames.sort_by(|a, b| a.epoch.total_cmp(&b.epoch));

    let bytes = write_tkdg(&frames);
    let sha = sha256_hex(&bytes);
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    eprintln!(
        "{out}: {} frame(s) from {fetched} minute file(s), {} B, sha256 {sha}",
        frames.len(),
        bytes.len()
    );
    let asset = match out.rsplit('/').next() {
        Some(name) if !name.is_empty() => name,
        _ => out.as_str(),
    };
    emit(asset, origin_template(), &sha, st, alt_m);
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: themis_asi_timed_keogram_compiler --station <label|code> --date <YYYYMMDD> [--start <HHMM> --stop <HHMM>] [--out-bin <file.bin>] [--ci-mode]"
        );
        eprintln!(
            "  --station  archive platform label (e.g. atha_themis02) or bare station code (e.g. atha, resolved from the day index)"
        );
        eprintln!(
            "  --date     night YYYYMMDD; --start/--stop clamp the UT hour-minute window within the night"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("themis_asi_timed_keogram_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_reads_yyyymmdd_and_refuses_impossible_days() {
        let d = parse_date("20261001").expect("a measured date parses");
        assert_eq!((d.year, d.month, d.day), (2026, 10, 1));
        assert_eq!(parse_date("20260230"), None);
        assert_eq!(parse_date("2026-10-01"), None);
        assert_eq!(parse_date("2026100"), None);
    }

    #[test]
    fn station_url_matches_the_measured_template() {
        let d = parse_date("20261001").expect("date parses");
        assert_eq!(
            station_url("atha_themis02", d),
            "https://data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream0/2026/10/01/atha_themis02/"
        );
    }

    #[test]
    fn hour_dirs_read_the_index_and_clamp_the_window() {
        let html = r#"<a href="../">../</a><a href="ut02/">ut02/</a><a href="ut06/">ut06/</a><a href="ut07/">ut07/</a>"#;
        assert_eq!(hour_dirs(html, None, None), vec!["ut02", "ut06", "ut07"]);
        assert_eq!(hour_dirs(html, Some(6), Some(7)), vec!["ut06", "ut07"]);
        assert_eq!(hour_dirs(html, Some(3), Some(5)), Vec::<String>::new());
    }

    #[test]
    fn pgm_files_keep_only_the_full_pgm_gz() {
        let html = r#"<a href="../">../</a><a href="20261001_0610_atha_themis02_full.pgm.gz">x</a><a href="20261001_0610_atha_themis02_full.pgm">y</a>"#;
        assert_eq!(
            pgm_files(html),
            vec!["20261001_0610_atha_themis02_full.pgm.gz"]
        );
    }

    #[test]
    fn minute_token_reads_the_hhmm_field() {
        assert_eq!(
            minute_token("20261001_0610_atha_themis02_full.pgm.gz"),
            Some(610)
        );
        assert_eq!(minute_token("short"), None);
    }

    #[test]
    fn resolve_label_finds_the_platform_directory_for_a_bare_code() {
        let html = r#"<a href="../">../</a><a href="atha_themis02/">atha_themis02/</a><a href="inuv_themis17/">inuv_themis17/</a>"#;
        assert_eq!(
            resolve_label(html, "atha").as_deref(),
            Some("atha_themis02")
        );
        assert_eq!(
            resolve_label(html, "inuv").as_deref(),
            Some("inuv_themis17")
        );
        assert_eq!(resolve_label(html, "zzzz"), None);
    }

    #[test]
    fn the_request_start_line_yields_unix_utc() {
        let line = r#"#"Image request start" 2024-05-10 06:00:00.033644 UTC"#;
        let t = parse_utc(line).expect("a stamp");
        assert!((t - 1715320800.033644).abs() < 1e-6);
    }

    #[test]
    fn the_profile_is_the_band_mean_in_big_endian() {
        let mut block = vec![0u8; FRAME_BYTES];
        for col in BAND_COL0..=BAND_COL1 {
            let i = col * 2;
            block[i] = 0x03;
            block[i + 1] = 0xE8;
        }
        block[(FRAME_W + BAND_COL0) * 2 + 1] = 0xC8;
        let profile = profile_of(&block);
        assert_eq!(profile.len(), FRAME_H);
        assert_eq!(profile[0], 1000);
        assert_eq!(profile[1], 200);
        assert_eq!(profile[2], 0);
    }

    #[test]
    fn the_tkdg_header_carries_magic_and_count() {
        let frames = vec![Frame {
            epoch: 1715320800.0,
            profile: vec![7u16; FRAME_H],
        }];
        let bytes = write_tkdg(&frames);
        assert_eq!(&bytes[0..4], MAGIC);
        assert_eq!(
            u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            1
        );
        assert_eq!(bytes.len(), 8 + 8 + FRAME_H * 2);
    }
}
