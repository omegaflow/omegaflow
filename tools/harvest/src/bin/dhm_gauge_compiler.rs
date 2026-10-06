use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, jstr, parse_json};
use omegaflow::lsk::days_from_civil;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "dhm.gov.np";
const BASE: &str = "https://dhm.gov.np";
const PAGE_PATH: &str = "/hydrology/hms-Single";
const SERIES_PATH: &str = "/site/getRiverWatchBySeriesId_Single";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn jar_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .ok();
    let name = match stamp {
        Some(nanos) => format!("dhm_gauge_{}_{}.jar", std::process::id(), nanos),
        None => format!("dhm_gauge_{}.jar", std::process::id()),
    };
    std::env::temp_dir().join(name)
}

fn curl(jar: &Path, url: &str, body: Option<&str>, headers: &[String]) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSfL")
        .arg("-m")
        .arg("120")
        .arg("-c")
        .arg(jar)
        .arg("-b")
        .arg(jar);
    if let Some(b) = body {
        cmd.arg("-d").arg(b);
    }
    for h in headers {
        cmd.arg("-H").arg(h);
    }
    let out = cmd.arg(url).output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "dhm_gauge_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn hidden_value(html: &str, name: &str) -> Option<String> {
    let needle = format!("name=\"{name}\"");
    let at = html.find(&needle)? + needle.len();
    let rest = &html[at..];
    let vpos = rest.find("value=\"")? + "value=\"".len();
    let rest = &rest[vpos..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn js_string(body: &str, marker: &str) -> Option<String> {
    let start = body.find(marker)? + marker.len();
    let rest = &body[start..];
    let bytes = rest.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == b'\'' {
            return Some(rest[..i].to_string());
        }
        i += 1;
    }
    None
}

fn decode_js_escapes(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'x' if i + 3 < bytes.len() => {
                    let hex = std::str::from_utf8(&bytes[i + 2..i + 4]).ok();
                    if let Some(v) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                        out.push(v);
                        i += 4;
                        continue;
                    }
                    out.push(bytes[i + 1]);
                    i += 2;
                }
                b'u' if i + 5 < bytes.len() => {
                    let hex = std::str::from_utf8(&bytes[i + 2..i + 6]).ok();
                    let cp = hex
                        .and_then(|h| u32::from_str_radix(h, 16).ok())
                        .and_then(char::from_u32);
                    if let Some(c) = cp {
                        let mut buf = [0u8; 4];
                        out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                        i += 6;
                        continue;
                    }
                    out.push(bytes[i + 1]);
                    i += 2;
                }
                b'n' => {
                    out.push(b'\n');
                    i += 2;
                }
                b'r' => {
                    out.push(b'\r');
                    i += 2;
                }
                b't' => {
                    out.push(b'\t');
                    i += 2;
                }
                b'\\' => {
                    out.push(b'\\');
                    i += 2;
                }
                other => {
                    out.push(other);
                    i += 2;
                }
            }
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn split_offset(rest: &str) -> (&str, &str) {
    let bytes = rest.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (b == b'+' || b == b'-') {
            return (&rest[..i], &rest[i..]);
        }
    }
    (rest, "")
}

fn parse_offset(off: &str) -> Option<i64> {
    if off.is_empty() || off.eq_ignore_ascii_case("z") {
        return Some(0);
    }
    let (sign, body) = match off.as_bytes().first().copied()? {
        b'+' => (1i64, &off[1..]),
        b'-' => (-1i64, &off[1..]),
        _ => return None,
    };
    let digits: String = body.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 2 {
        return None;
    }
    let hh: i64 = digits.get(..2)?.parse().ok()?;
    let mm: i64 = if digits.len() >= 4 {
        digits.get(2..4)?.parse().ok()?
    } else {
        0
    };
    Some(sign * (hh * 3600 + mm * 60))
}

fn parse_iso_unix(s: &str) -> Option<i64> {
    let text = s.trim();
    let (date, clock) = match text.split_once('T') {
        Some(pair) => pair,
        None => text.split_once(' ')?,
    };
    let mut d = date.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let (hms, off) = split_offset(clock);
    let mut t = hms.split(':');
    let hour: i64 = t.next()?.parse().ok()?;
    let minute: i64 = t.next()?.parse().ok()?;
    let seconds_text = t.next()?.split('.').next()?;
    let second: i64 = seconds_text.parse().ok()?;
    let offset = parse_offset(off)?;
    Some(days * 86400 + hour * 3600 + minute * 60 + second - offset)
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn today_iso() -> Option<String> {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let (y, m, d) = civil_from_days(secs.div_euclid(86400));
    Some(format!("{y:04}-{m:02}-{d:02}"))
}

fn parse_series(payload: &JsonVal) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let rows = match payload {
        JsonVal::Arr(rows) => rows,
        _ => return out,
    };
    for row in rows {
        let JsonVal::Obj(map) = row else { continue };
        let Some(value) = map.get("value").and_then(|val| match val {
            JsonVal::Num(n) => Some(*n),
            JsonVal::Str(s) => s.parse::<f64>().ok(),
            _ => None,
        }) else {
            continue;
        };
        let Some(datetime) = map.get("datetime").and_then(|d| match d {
            JsonVal::Str(s) => Some(s.as_str()),
            _ => None,
        }) else {
            continue;
        };
        let Some(t) = parse_iso_unix(datetime) else {
            continue;
        };
        if value.is_finite() {
            out.push((t as f64, value));
        }
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = arg_value(args, "--station").ok_or("--station <id> required")?;
    let period = match arg_value(args, "--period") {
        Some(v) => v
            .parse::<u32>()
            .map_err(|_| format!("--period '{v}' is not 1..4"))?,
        None => 1,
    };
    if !(1..=4).contains(&period) {
        return Err(format!("--period {period} outside 1..4"));
    }
    let date = match arg_value(args, "--date") {
        Some(v) => v,
        None => today_iso().ok_or("system clock carries no date")?,
    };
    let page_url = format!("{BASE}{PAGE_PATH}/{station}");
    let series_url = format!("{BASE}{SERIES_PATH}");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/dhm_{station}_stage.txt"),
    };

    let jar = jar_path();
    let html = curl(&jar, &page_url, None, &[]).ok_or_else(|| format!("{page_url}: fetch void"))?;
    let csrf = hidden_value(&html, "csrf_test_name")
        .ok_or_else(|| format!("{page_url}: no csrf_test_name witness"))?;

    let page_meta = js_string(&html, "var river = '")
        .map(|s| decode_js_escapes(&s))
        .and_then(|s| parse_json(&s))
        .ok_or_else(|| format!("{page_url}: no var river metadata"))?;
    let series_id = match &page_meta {
        JsonVal::Obj(map) => map.get("series_id").and_then(|v| match v {
            JsonVal::Num(n) => Some(*n as i64),
            JsonVal::Str(s) => s.parse::<i64>().ok(),
            _ => None,
        }),
        _ => None,
    }
    .ok_or_else(|| format!("{page_url}: series_id absent"))?;
    let name =
        jstr(&page_meta, "name").ok_or_else(|| format!("{page_url}: station name absent"))?;

    let body = format!("csrf_test_name={csrf}&seriesid={series_id}&period={period}&date={date}");
    let headers = vec![
        "X-Requested-With: XMLHttpRequest".to_string(),
        format!("Referer: {page_url}"),
    ];
    let response = curl(&jar, &series_url, Some(&body), &headers)
        .ok_or_else(|| format!("{series_url}: POST void"))?;
    let _ = std::fs::remove_file(&jar);

    let root = parse_json(&response).ok_or_else(|| format!("{series_url}: response parse void"))?;
    let chart =
        jstr(&root, "data.chart").ok_or_else(|| format!("{series_url}: data.chart absent"))?;
    let inner = js_string(&chart, "var river = '")
        .ok_or_else(|| format!("{series_url}: chart carries no var river"))?;
    let payload = parse_json(&decode_js_escapes(&inner))
        .ok_or_else(|| format!("{series_url}: variable river parse void"))?;
    let mut series = parse_series(&payload);
    series.sort_by(|a, b| a.0.total_cmp(&b.0));

    if series.is_empty() {
        println!(
            "dhm_gauge_compiler: station {station} series {series_id} carries no measured stage row for {date} period {period} — the asset stays unwritten (0 honored)"
        );
        return Ok(());
    }

    let mut text = format!(
        "# dhm.gov.np river gauge stage | station {station} {name} | series {series_id} | value water level m | period {period} | date {date} | source {page_url}\n"
    );
    for (t, v) in &series {
        text.push_str(&format!("{t} {v}\n"));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "dhm_gauge_compiler: {} stage rows written to {out} ({series_url})",
        series.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("dhm_gauge_compiler: {msg}");
        std::process::exit(2);
    }
}
