use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "lgdc.uml.edu";
const URL_BASE: &str = "https://lgdc.uml.edu/fastchar/getbest";
const CHARS: &str = "foF2,hmF2";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "giro_fastchar_lastday_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
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

fn utc_stamp(unix: i64) -> String {
    let (y, mo, d) = civil_from_days(unix.div_euclid(86400));
    let rem = unix.rem_euclid(86400);
    format!(
        "{y:04}/{mo:02}/{d:02}+{:02}:{:02}:{:02}",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

fn parse_iso_unix(s: &str) -> Option<i64> {
    let text = s.trim();
    let text = match text.strip_suffix('Z') {
        Some(t) => t,
        None => text,
    };
    let (date, clock) = text.split_once('T')?;
    let mut d = date.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let mut t = clock.split(':');
    let hour: i64 = t.next()?.parse().ok()?;
    let minute: i64 = t.next()?.parse().ok()?;
    let second: i64 = if let Some(sec) = t.next() {
        sec.split('.').next()?.parse().ok()?
    } else {
        0
    };
    Some(days * 86400 + hour * 3600 + minute * 60 + second)
}

struct Row {
    unix: i64,
    fof2: f64,
    hmf2: f64,
    cs: i64,
}

fn rows(body: &str) -> Result<Vec<Row>, String> {
    let mut header_seen = false;
    let mut out = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("# STATUS:") {
            return Err(format!("payload reports STATUS:{}", rest));
        }
        if line.starts_with('#') {
            if line.starts_with("# Time") {
                let cols = line.split_whitespace().collect::<Vec<&str>>();
                let wanted = cols.contains(&"foF2") && cols.contains(&"hmF2");
                if !wanted {
                    return Err(format!("payload table header carries no foF2/hmF2: {line}"));
                }
                header_seen = true;
            }
            continue;
        }
        if !header_seen {
            continue;
        }
        let f = line.split_whitespace().collect::<Vec<&str>>();
        if f.len() < 6 {
            continue;
        }
        let Some(unix) = parse_iso_unix(f[0]) else {
            continue;
        };
        let Ok(cs) = f[1].parse::<i64>() else {
            continue;
        };
        let (Ok(fof2), Ok(hmf2)) = (f[2].parse::<f64>(), f[4].parse::<f64>()) else {
            continue;
        };
        if !fof2.is_finite() || !hmf2.is_finite() {
            continue;
        }
        out.push(Row {
            unix,
            fof2,
            hmf2,
            cs,
        });
    }
    if !header_seen {
        return Err("payload carries no '# Time' table header".to_string());
    }
    out.sort_by_key(|r| r.unix);
    Ok(out)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = match arg_value(args, "--station") {
        Some(v) => v,
        None => "RO041".to_string(),
    };
    let hours: i64 = match arg_value(args, "--hours") {
        Some(v) => v
            .parse()
            .map_err(|_| format!("--hours {v} is not an integer"))?,
        None => 24,
    };
    if hours <= 0 {
        return Err(format!("--hours {hours} is not positive"));
    }
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/fastchar_{station}.txt"),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system clock returned {e}"))?
        .as_secs() as i64;
    let from = utc_stamp(now - hours * 3600);
    let to = utc_stamp(now);
    let url =
        format!("{URL_BASE}?ursiCode={station}&charName={CHARS}&DMUF=&fromDate={from}&toDate={to}");

    let body = curl(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Err(format!("{url}: empty body"));
    }
    if trimmed.starts_with('<') {
        return Err(format!("{url}: body is HTML, not a measurement table"));
    }
    let rows = rows(&body).map_err(|e| format!("{url}: {e}"))?;
    if rows.is_empty() {
        return Err(format!(
            "{url}: no timestamp carries a finite foF2 and hmF2 — 0 honored"
        ));
    }

    let mut text = format!(
        "# giro fastchar | unix fof2_mhz hmf2_km cs | foF2 F2 layer critical frequency, hmF2 peak height F2-layer, cs autoscaling confidence score (999 manual, -1 unknown) | station {station} | source {url}\n"
    );
    for r in &rows {
        text.push_str(&format!("{} {} {} {}\n", r.unix, r.fof2, r.hmf2, r.cs));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "giro_fastchar_lastday_compiler: {} rows written to {out} ({} B)",
        rows.len(),
        text.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("giro_fastchar_lastday_compiler: {msg}");
        std::process::exit(2);
    }
}
