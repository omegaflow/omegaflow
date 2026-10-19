use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://cdaweb.gsfc.nasa.gov/hapi";
const ID: &str = "DE2_UA16S_ALL";
const PARAMS: &str = "NACS_N2density,WATS_NVEneutralWind,WATS_NVUneutralWind,RPA_NIionDensity,RPA_IVEionDrift,RPA_IVNionDrift";
const WINDOW_START: &str = "1981-08-06";
const WINDOW_END: &str = "1983-02-19";
const OUT_DEFAULT: &str = "sources/cdaweb.gsfc.nasa.gov/de2_ua16s_all.csv";
const DEFAULT_JOBS: usize = 8;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("5")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("5")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn date_of(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y}-{m:02}-{d:02}")
}

fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
}

fn month_windows(start: i64, end: i64) -> Option<Vec<(i64, i64)>> {
    let (sy, sm, _) = civil_from_days(start);
    let (ey, em, _) = civil_from_days(end);
    let mut out = Vec::new();
    let (mut y, mut m) = (sy, sm);
    loop {
        let first = days_from_civil(y, m, 1)?;
        let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
        let next = days_from_civil(ny, nm, 1)?;
        let last = next - 1;
        let ws = first.max(start);
        let we = last.min(end);
        if ws <= we {
            out.push((ws, we));
        }
        if y == ey && m == em {
            break;
        }
        y = ny;
        m = nm;
    }
    Some(out)
}

fn window_url(start_day: i64, end_day: i64) -> String {
    format!(
        "{BASE}/data?id={ID}&time.min={}T00:00:00Z&time.max={}T23:59:59Z&parameters={PARAMS}&format=csv",
        date_of(start_day),
        date_of(end_day)
    )
}

fn concat_csv(windows: &[String]) -> Option<String> {
    let mut out = String::new();
    let mut header_written = false;
    let mut rows = 0usize;
    for text in windows {
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.starts_with('#') {
                if !header_written {
                    out.push_str(line.trim_end());
                    out.push('\n');
                }
                continue;
            }
            header_written = true;
            out.push_str(line.trim_end());
            out.push('\n');
            rows += 1;
        }
    }
    if rows == 0 { None } else { Some(out) }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => OUT_DEFAULT.to_string(),
    };
    let jobs: usize = match arg_value(&args, "--jobs").and_then(|v| v.parse().ok()) {
        Some(v) if v > 0 => v,
        _ => DEFAULT_JOBS,
    };
    let start = arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
        .or_else(|| parse_days(WINDOW_START));
    let end = arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
        .or_else(|| parse_days(WINDOW_END));
    let (Some(start), Some(end)) = (start, end) else {
        eprintln!("window dates void — the calendar stays unread");
        std::process::exit(1);
    };
    if start > end {
        eprintln!(
            "window start {} lies past stop {}",
            date_of(start),
            date_of(end)
        );
        std::process::exit(1);
    }
    let Some(windows) = month_windows(start, end) else {
        eprintln!("month windows void — the calendar stays unread");
        std::process::exit(1);
    };
    if windows.is_empty() {
        eprintln!(
            "{ID} carries no month window for {}..{}",
            date_of(start),
            date_of(end)
        );
        std::process::exit(1);
    }
    eprintln!(
        "{ID}: {} month windows {}..{}",
        windows.len(),
        date_of(start),
        date_of(end)
    );
    let windows = Arc::new(windows);
    let results: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(vec![None; windows.len()]));
    let next = Arc::new(AtomicUsize::new(0));
    let mut workers = Vec::new();
    for _ in 0..jobs {
        let windows = Arc::clone(&windows);
        let results = Arc::clone(&results);
        let next = Arc::clone(&next);
        workers.push(std::thread::spawn(move || {
            loop {
                let idx = next.fetch_add(1, Ordering::SeqCst);
                if idx >= windows.len() {
                    break;
                }
                let (ws, we) = windows[idx];
                let text = fetch(&window_url(ws, we));
                let mut guard = match results.lock() {
                    Ok(g) => g,
                    Err(poisoned) => poisoned.into_inner(),
                };
                guard[idx] = text;
            }
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let collected = match Arc::try_unwrap(results) {
        Ok(m) => match m.into_inner() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("window results stay shared — the rows stay unread");
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("window results stay shared — the rows stay unread");
            std::process::exit(1);
        }
    };
    let mut bodies: Vec<String> = Vec::with_capacity(collected.len());
    for (i, body) in collected.into_iter().enumerate() {
        match body {
            Some(text) => bodies.push(text),
            None => {
                let (ws, we) = windows[i];
                eprintln!(
                    "window {}-{}: fetch void — the asset stays unwritten",
                    date_of(ws),
                    date_of(we)
                );
                std::process::exit(1);
            }
        }
    }
    let Some(csv) = concat_csv(&bodies) else {
        eprintln!(
            "{ID}: no data rows across {} windows — the asset stays unwritten (0 honored)",
            bodies.len()
        );
        std::process::exit(1);
    };
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, csv.as_bytes()).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let rows = csv
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .count();
    eprintln!("{out}: {rows} data rows");
    if ci_mode && !upload_release("cdaweb.gsfc.nasa.gov", &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "# HAPI 1.1.0\n# parameters: time, B_mag\n# units: UTC, nT\n";

    #[test]
    fn concatenates_windows_into_one_header_and_all_data_rows() {
        let a = format!("{HEADER}2020-01-01T00:00:00.000Z, 1.0\n2020-01-01T00:01:00.000Z, 2.0\n");
        let b = format!("{HEADER}2020-02-01T00:00:00.000Z, 3.0\n");
        let out = concat_csv(&[a, b]).expect("two windows concatenate");
        let header = out
            .lines()
            .filter(|l| l.trim_start().starts_with("# parameters:"))
            .count();
        let units = out
            .lines()
            .filter(|l| l.trim_start().starts_with("# units:"))
            .count();
        let data = out
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && !t.starts_with('#')
            })
            .count();
        assert_eq!(header, 1);
        assert_eq!(units, 1);
        assert_eq!(data, 3);
    }

    #[test]
    fn window_without_data_rows_stays_void() {
        assert!(concat_csv(&[HEADER.to_string()]).is_none());
    }
}
