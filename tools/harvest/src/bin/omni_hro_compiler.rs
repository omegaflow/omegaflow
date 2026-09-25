use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::omni_hro::{
    COMP_IMF_BX_GSE, COMP_IMF_BY_GSM, COMP_IMF_BZ_GSM, COMP_IMF_F, COMP_SW_DENSITY,
    COMP_SW_FLOW_SPEED, COMP_SW_PRESSURE, COMP_SW_TEMP, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE: &str = "https://cdaweb.gsfc.nasa.gov/hapi";
const DATASET: &str = "OMNI_HRO_1MIN";
const PARAMS: &str = "F,BX_GSE,BY_GSM,BZ_GSM,flow_speed,proton_density,T,Pressure";

struct Want {
    name: &'static str,
    comp: u32,
    range: f64,
    positive: bool,
}

const WANTED: [Want; 8] = [
    Want {
        name: "F",
        comp: COMP_IMF_F,
        range: 1000.0,
        positive: false,
    },
    Want {
        name: "BX_GSE",
        comp: COMP_IMF_BX_GSE,
        range: 1000.0,
        positive: false,
    },
    Want {
        name: "BY_GSM",
        comp: COMP_IMF_BY_GSM,
        range: 1000.0,
        positive: false,
    },
    Want {
        name: "BZ_GSM",
        comp: COMP_IMF_BZ_GSM,
        range: 1000.0,
        positive: false,
    },
    Want {
        name: "flow_speed",
        comp: COMP_SW_FLOW_SPEED,
        range: 5000.0,
        positive: true,
    },
    Want {
        name: "proton_density",
        comp: COMP_SW_DENSITY,
        range: 1000.0,
        positive: true,
    },
    Want {
        name: "T",
        comp: COMP_SW_TEMP,
        range: 1.0e8,
        positive: true,
    },
    Want {
        name: "Pressure",
        comp: COMP_SW_PRESSURE,
        range: 1000.0,
        positive: true,
    },
];

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

fn parse_iso(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    let month: i64 = s.get(5..7)?.parse().ok()?;
    let day: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let sec: i64 = s.get(17..19)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + sec as f64)
}

fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
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

fn keep_field(v: f64, fill: f64, range: f64) -> bool {
    v.is_finite() && v != fill && v.abs() <= range
}

fn keep_positive(v: f64, fill: f64, range: f64) -> bool {
    v.is_finite() && v != fill && v > 0.0 && v <= range
}

fn hapi_header() -> Option<([f64; 8], i64, i64)> {
    let text = fetch(&format!("{BASE}/info?id={DATASET}"))?;
    let root = parse_json(&text)?;
    let JsonVal::Obj(root) = root else {
        return None;
    };
    let JsonVal::Arr(params) = root.get("parameters")? else {
        return None;
    };
    let mut fills = [f64::NAN; 8];
    let mut found = [false; 8];
    for p in params {
        let JsonVal::Obj(entry) = p else {
            continue;
        };
        let Some(JsonVal::Str(name)) = entry.get("name") else {
            continue;
        };
        let Some(idx) = WANTED.iter().position(|w| w.name == name.as_str()) else {
            continue;
        };
        let fill = match entry.get("fill") {
            Some(JsonVal::Num(n)) if n.is_finite() => *n,
            Some(JsonVal::Str(s)) => match s.parse::<f64>() {
                Ok(v) if v.is_finite() => v,
                _ => {
                    eprintln!("header {name} carries no numeric fill — channel stays unread");
                    return None;
                }
            },
            _ => {
                eprintln!("header {name} carries no numeric fill — channel stays unread");
                return None;
            }
        };
        fills[idx] = fill;
        found[idx] = true;
    }
    if !found.iter().all(|&b| b) {
        eprintln!("header omits a wanted parameter — the channel map stays unread");
        return None;
    }
    let start = match root.get("startDate")? {
        JsonVal::Str(s) => parse_days(s.get(0..10)?)?,
        _ => return None,
    };
    let stop = match root.get("stopDate")? {
        JsonVal::Str(s) => parse_days(s.get(0..10)?)?,
        _ => return None,
    };
    Some((fills, start, stop))
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

fn harvest_window(
    start_day: i64,
    end_day: i64,
    decimate_s: f64,
    fills: &[f64; 8],
    buckets: &Mutex<HashMap<(u32, i64), Vec<f64>>>,
    rows_out: &Mutex<usize>,
    fills_out: &Mutex<usize>,
) {
    let url = format!(
        "{}/data?id={}&time.min={}T00:00:00Z&time.max={}T23:59:59Z&parameters={}&format=csv",
        BASE,
        DATASET,
        date_of(start_day),
        date_of(end_day),
        PARAMS
    );
    let Some(text) = fetch(&url) else {
        eprintln!(
            "window {}-{}: fetch void — the window stays unharvested",
            date_of(start_day),
            date_of(end_day)
        );
        return;
    };
    let mut rows = 0usize;
    let mut fill_rows = 0usize;
    let mut guard = match buckets.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    for line in text.lines() {
        if line.is_empty() || !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 9 {
            continue;
        }
        let Some(t) = parse_iso(parts[0]) else {
            continue;
        };
        rows += 1;
        let bucket = (t / decimate_s).floor() as i64;
        let mut taken = 0usize;
        for (i, w) in WANTED.iter().enumerate() {
            let Ok(v) = parts[i + 1].parse::<f64>() else {
                continue;
            };
            let keep = if w.positive {
                keep_positive(v, fills[i], w.range)
            } else {
                keep_field(v, fills[i], w.range)
            };
            if keep {
                guard.entry((w.comp, bucket)).or_default().push(v);
                taken += 1;
            }
        }
        if taken == 0 {
            fill_rows += 1;
        }
    }
    drop(guard);
    let mut r = match rows_out.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    *r += rows;
    drop(r);
    let mut f = match fills_out.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    *f += fill_rows;
    drop(f);
    eprintln!(
        "window {}-{}: {} rows, {} fill-skipped",
        date_of(start_day),
        date_of(end_day),
        rows,
        fill_rows
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join("omni_hro_1min.bin")
            .to_string_lossy()
            .into_owned(),
    };
    let decimate_min: f64 = arg_value(&args, "--decimate-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(60.0);
    let decimate_s = decimate_min * 60.0;
    if !(decimate_s > 0.0) || !decimate_s.is_finite() {
        eprintln!(
            "--decimate-min {} carries no positive bucket width",
            decimate_min
        );
        std::process::exit(1);
    }
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    let (fills, info_start, info_stop) = match hapi_header() {
        Some(h) => h,
        None => {
            eprintln!(
                "HAPI info {} parses void — no window and no fill table, the bin stays unwritten",
                DATASET
            );
            std::process::exit(1);
        }
    };
    let start_day = arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
        .unwrap_or(info_start);
    let end_day = arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
        .unwrap_or(info_stop);
    if start_day > end_day {
        eprintln!(
            "window start {} lies past stop {} — the bin stays unwritten",
            date_of(start_day),
            date_of(end_day)
        );
        std::process::exit(1);
    }
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let windows = match month_windows(start_day, end_day) {
        Some(w) => w,
        None => {
            eprintln!("month windows void — the calendar stays unread");
            std::process::exit(1);
        }
    };
    if windows.is_empty() {
        eprintln!(
            "{} carries no month window for {}..{}",
            DATASET,
            date_of(start_day),
            date_of(end_day)
        );
        std::process::exit(1);
    }
    eprintln!(
        "{}: {} month windows {}..{}, {} channels, decimate {} min",
        DATASET,
        windows.len(),
        date_of(start_day),
        date_of(end_day),
        WANTED.len(),
        decimate_min
    );
    let buckets: Arc<Mutex<HashMap<(u32, i64), Vec<f64>>>> = Arc::new(Mutex::new(HashMap::new()));
    let window_rows: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let window_fills: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let next = Arc::new(AtomicUsize::new(0));
    let windows = Arc::new(windows);
    let mut workers = Vec::new();
    for _ in 0..jobs {
        let buckets = Arc::clone(&buckets);
        let window_rows = Arc::clone(&window_rows);
        let window_fills = Arc::clone(&window_fills);
        let next = Arc::clone(&next);
        let windows = Arc::clone(&windows);
        workers.push(std::thread::spawn(move || {
            loop {
                let idx = next.fetch_add(1, Ordering::SeqCst);
                if idx >= windows.len() {
                    break;
                }
                let (ws, we) = windows[idx];
                harvest_window(
                    ws,
                    we,
                    decimate_s,
                    &fills,
                    &buckets,
                    &window_rows,
                    &window_fills,
                );
            }
        }));
    }
    for w in workers {
        let _ = w.join();
    }
    let Some(rows) = Arc::try_unwrap(window_rows)
        .ok()
        .and_then(|m| m.into_inner().ok())
    else {
        eprintln!("window_rows stays shared — the row count stays unread");
        std::process::exit(1);
    };
    let Some(fill_rows) = Arc::try_unwrap(window_fills)
        .ok()
        .and_then(|m| m.into_inner().ok())
    else {
        eprintln!("window_fills stays shared — the fill count stays unread");
        std::process::exit(1);
    };
    let Some(buckets_guard) = Arc::try_unwrap(buckets)
        .ok()
        .and_then(|m| m.into_inner().ok())
    else {
        eprintln!("buckets stay shared — the bucket map stays unread");
        std::process::exit(1);
    };
    let mut raw: Vec<(f64, f64, u32)> = Vec::new();
    let mut pre_lsk_skip = 0usize;
    for ((comp, bucket), vals) in buckets_guard {
        if vals.is_empty() {
            continue;
        }
        let t_unix = (bucket as f64 + 0.5) * decimate_s;
        let Some(t) = lsk.unix_to_tdb(t_unix) else {
            pre_lsk_skip += 1;
            continue;
        };
        let mut vals = vals;
        raw.push((t, median(&mut vals), comp));
    }
    raw.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    if raw.is_empty() {
        eprintln!(
            "{}: no records — the bin stays unwritten (0 honored)",
            DATASET
        );
        std::process::exit(1);
    }
    eprintln!(
        "{}: {} rows, {} fill-skipped rows, {} records, {} buckets pre-1972 stay unharvested (leap table void, 0 honored) — epoch TDB via LSK",
        DATASET,
        rows,
        fill_rows,
        raw.len(),
        pre_lsk_skip
    );
    let bytes = write_bin(&raw);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{}: {} records, roundtrip parses", out, parsed.len());
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("cdaweb.gsfc.nasa.gov", &out) {
        std::process::exit(1);
    }
}
