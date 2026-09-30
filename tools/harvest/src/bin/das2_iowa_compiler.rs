use omegaflow::archivar::cache_root;
use omegaflow::archivar::hapi_csv::{
    HapiParam, csv_header_fields, parse_bin, parse_info, series_from_csv, write_bin,
};
use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{LeapSeconds, days_from_civil, parse as parse_lsk};
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://planet.physics.uiowa.edu/das/das2Server/hapi";
const DEFAULT_ID: &str = "Cassini/MAG/VectorKSO";

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
        .arg("2")
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

fn day_field(root: &JsonVal, key: &str) -> Option<i64> {
    let JsonVal::Obj(o) = root else {
        return None;
    };
    let JsonVal::Str(s) = o.get(key)? else {
        return None;
    };
    parse_days(s.get(0..10)?)
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

fn median(vals: &mut [f64]) -> f64 {
    vals.sort_by(|a, b| a.total_cmp(b));
    let n = vals.len();
    if n % 2 == 0 {
        (vals[n / 2 - 1] + vals[n / 2]) * 0.5
    } else {
        vals[n / 2]
    }
}

fn window_records(
    id: &str,
    start_day: i64,
    end_day: i64,
    params: &[HapiParam],
    lsk: &LeapSeconds,
    out: &Mutex<Vec<(f64, f64, u32)>>,
    rows_out: &Mutex<usize>,
) {
    let url = format!(
        "{}/data?id={}&time.min={}T00:00:00Z&time.max={}T23:59:59Z&format=csv",
        BASE,
        id,
        date_of(start_day),
        date_of(end_day)
    );
    let Some(text) = fetch(&url) else {
        eprintln!(
            "window {}-{}: fetch void — the window stays unharvested",
            date_of(start_day),
            date_of(end_day)
        );
        return;
    };
    if csv_header_fields(&text).is_some_and(|fields| fields.len() != params.len()) {
        eprintln!(
            "window {}-{}: CSV header width differs from info table width — columns read positionally",
            date_of(start_day),
            date_of(end_day)
        );
    }
    let records = series_from_csv(&text, params, lsk);
    let rows = records.len();
    let mut guard = match out.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.extend(records);
    drop(guard);
    let mut r = match rows_out.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    *r += rows;
    drop(r);
    eprintln!(
        "window {}-{}: {} records",
        date_of(start_day),
        date_of(end_day),
        rows
    );
}

fn slug_of(id: &str) -> String {
    id.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let id = match arg_value(&args, "--id") {
        Some(v) => v,
        None => DEFAULT_ID.to_string(),
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => cache_root()
            .join(format!("das2_iowa_{}.bin", slug_of(&id)))
            .to_string_lossy()
            .into_owned(),
    };
    let decimate_s: f64 = match arg_value(&args, "--decimate-s") {
        Some(v) => match v.parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("--decimate-s {v} carries no number");
                std::process::exit(2);
            }
        },
        None => 0.0,
    };
    if !decimate_s.is_finite() || decimate_s < 0.0 {
        eprintln!("--decimate-s {decimate_s} carries no non-negative bucket width");
        std::process::exit(1);
    }
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    let lsk = match arg_value(&args, "--lsk") {
        Some(p) => match std::fs::read_to_string(&p).ok().and_then(|t| parse_lsk(&t)) {
            Some(l) => l,
            None => {
                eprintln!("--lsk {p} parses void — the leap-second table stays unread");
                std::process::exit(1);
            }
        },
        None => match embedded_lsk() {
            Some(l) => l,
            None => {
                eprintln!("embedded leap-second table void — the TDB conversion stays absent");
                std::process::exit(1);
            }
        },
    };
    let info_text = match fetch(&format!("{BASE}/info?id={id}")) {
        Some(t) => t,
        None => {
            eprintln!("info {id} returns void — the parameter table stays unread");
            std::process::exit(1);
        }
    };
    let params = match parse_info(&info_text) {
        Some(p) => p,
        None => {
            eprintln!("info {id} carries no parameter table — the channels stay unread");
            std::process::exit(1);
        }
    };
    let info = parse_json(&info_text);
    let start_day = arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
        .or_else(|| info.as_ref().and_then(|j| day_field(j, "sampleStartDate")))
        .or_else(|| info.as_ref().and_then(|j| day_field(j, "startDate")));
    let end_day = arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
        .or_else(|| info.as_ref().and_then(|j| day_field(j, "sampleStopDate")))
        .or_else(|| info.as_ref().and_then(|j| day_field(j, "stopDate")));
    let (Some(start_day), Some(end_day)) = (start_day, end_day) else {
        eprintln!("{id} carries no window dates — name --window-start and --window-end");
        std::process::exit(1);
    };
    if start_day > end_day {
        eprintln!(
            "window start {} lies past stop {} — the bin stays unwritten",
            date_of(start_day),
            date_of(end_day)
        );
        std::process::exit(1);
    }
    let windows = match month_windows(start_day, end_day) {
        Some(w) => w,
        None => {
            eprintln!("month windows void — the calendar stays unread");
            std::process::exit(1);
        }
    };
    if windows.is_empty() {
        eprintln!(
            "{id} carries no month window for {}..{}",
            date_of(start_day),
            date_of(end_day)
        );
        std::process::exit(1);
    }
    eprintln!(
        "{id}: {} channels, {} month windows {}..{}, decimate {} s",
        params.len().saturating_sub(1),
        windows.len(),
        date_of(start_day),
        date_of(end_day),
        decimate_s
    );
    let records: Arc<Mutex<Vec<(f64, f64, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let window_rows: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let next = Arc::new(AtomicUsize::new(0));
    let windows = Arc::new(windows);
    let params = Arc::new(params);
    let lsk = Arc::new(lsk);
    let mut workers = Vec::new();
    for _ in 0..jobs {
        let records = Arc::clone(&records);
        let window_rows = Arc::clone(&window_rows);
        let next = Arc::clone(&next);
        let windows = Arc::clone(&windows);
        let params = Arc::clone(&params);
        let lsk = Arc::clone(&lsk);
        let id = id.clone();
        workers.push(std::thread::spawn(move || {
            loop {
                let idx = next.fetch_add(1, Ordering::SeqCst);
                if idx >= windows.len() {
                    break;
                }
                let (ws, we) = windows[idx];
                window_records(&id, ws, we, &params, &lsk, &records, &window_rows);
            }
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let Some(collected) = Arc::try_unwrap(records)
        .ok()
        .and_then(|m| m.into_inner().ok())
    else {
        eprintln!("records stay shared — the series stays unread");
        std::process::exit(1);
    };
    let Some(rows) = Arc::try_unwrap(window_rows)
        .ok()
        .and_then(|m| m.into_inner().ok())
    else {
        eprintln!("window_rows stays shared — the row count stays unread");
        std::process::exit(1);
    };
    let nchan = params.len().saturating_sub(1) as u32;
    let mut final_records: Vec<(f64, f64, u32)> = if decimate_s > 0.0 {
        let mut buckets: HashMap<(u32, i64), Vec<f64>> = HashMap::new();
        for (t, val, comp) in collected {
            let bucket = (t / decimate_s).floor() as i64;
            buckets.entry((comp, bucket)).or_default().push(val);
        }
        let mut out = Vec::with_capacity(buckets.len());
        for ((comp, bucket), mut vals) in buckets {
            out.push(((bucket as f64 + 0.5) * decimate_s, median(&mut vals), comp));
        }
        out
    } else {
        collected
    };
    final_records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    if final_records.is_empty() {
        eprintln!("{id}: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "{id}: {} rows, {} records, epoch TDB via LSK",
        rows,
        final_records.len()
    );
    let bytes = write_bin(nchan, &final_records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{out}: {} records, roundtrip parses", parsed.len());
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("planet.physics.uiowa.edu", &out) {
        std::process::exit(1);
    }
}
