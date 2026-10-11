use omegaflow::archivar::fetch_raw;
use omegaflow::hapi_csv::parse_iso_seconds;
use omegaflow::substorm::{SubstormOnset, parse_csv};

const BASE: &str = "https://supermag.jhuapl.edu/lib/services/";
const CHUNK_S: i64 = 25 * 365 * 86400;
const LISTS: [&str; 5] = ["newell", "forsyth", "liou", "frey", "ohtani"];

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
        }
        i += 1;
    }
    out
}

fn iso_to_unix(s: &str) -> Option<i64> {
    if s.len() >= 19 {
        parse_iso_seconds(s).map(|v| v as i64)
    } else if s.len() == 10 {
        parse_iso_seconds(&format!("{s}T00:00:00Z")).map(|v| v as i64)
    } else {
        None
    }
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn ymdhms_to_unix(y: i64, m: i64, d: i64, h: i64, mi: i64, s: i64) -> f64 {
    (days_from_civil(y, m, d) * 86400 + h * 3600 + mi * 60 + s) as f64
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
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn fmt_date(unix: i64) -> String {
    let days = unix.div_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn nearest(sorted: &[f64], x: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    match sorted.binary_search_by(|p| p.total_cmp(&x)) {
        Ok(_) => Some(0.0),
        Err(i) => {
            let lo = if i > 0 { Some(x - sorted[i - 1]) } else { None };
            let hi = if i < sorted.len() {
                Some(sorted[i] - x)
            } else {
                None
            };
            match (lo, hi) {
                (Some(l), Some(h)) => Some(l.min(h)),
                (Some(l), None) => Some(l),
                (None, Some(h)) => Some(h),
                (None, None) => None,
            }
        }
    }
}

fn median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    if n % 2 == 1 {
        Some(values[n / 2])
    } else {
        Some((values[n / 2 - 1] + values[n / 2]) / 2.0)
    }
}

fn match_onsets(a: &[f64], b: &[f64], tol: f64) -> (usize, usize, Option<f64>) {
    let mut dts: Vec<f64> = Vec::new();
    let mut matched_a = 0usize;
    for &x in a {
        if let Some(dt) = nearest(b, x) {
            if dt <= tol {
                matched_a += 1;
                dts.push(dt);
            }
        }
    }
    let mut matched_b = 0usize;
    for &x in b {
        if let Some(dt) = nearest(a, x) {
            if dt <= tol {
                matched_b += 1;
                dts.push(dt);
            }
        }
    }
    (matched_a, matched_b, median(&mut dts))
}

fn ratio(num: usize, den: usize) -> String {
    if den == 0 {
        "n/a".to_string()
    } else {
        format!("{:.6}", num as f64 / den as f64)
    }
}

fn secs(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.3}"),
        None => "n/a".to_string(),
    }
}

fn read_mcpherron(path: &str) -> Vec<f64> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("substorm_compare: {path} read void");
            std::process::exit(1);
        }
    };
    let mut out: Vec<f64> = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().take(6).collect();
        if cols.len() < 6 {
            continue;
        }
        let mut v = [0i64; 6];
        let mut ok = true;
        for (i, c) in cols.iter().enumerate() {
            match c.parse::<i64>() {
                Ok(x) => v[i] = x,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            out.push(ymdhms_to_unix(v[0], v[1], v[2], v[3], v[4], v[5]));
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = arg_value(&args, "--mcpherron") else {
        eprintln!("substorm_compare needs --mcpherron <path>");
        std::process::exit(2);
    };
    let (Some(start), Some(stop)) = (arg_value(&args, "--start"), arg_value(&args, "--stop"))
    else {
        eprintln!("substorm_compare needs --start <iso> and --stop <iso>");
        std::process::exit(2);
    };
    let (Some(start_unix), Some(stop_unix)) = (iso_to_unix(&start), iso_to_unix(&stop)) else {
        eprintln!("--start/--stop must be ISO8601 (YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ)");
        std::process::exit(2);
    };
    if stop_unix <= start_unix {
        eprintln!("--stop must lie after --start");
        std::process::exit(2);
    }
    let tol_min: Vec<i64> = match arg_value(&args, "--tol-min") {
        Some(t) => match t.parse::<i64>() {
            Ok(v) if v > 0 => vec![v],
            _ => {
                eprintln!("--tol-min must be a positive integer");
                std::process::exit(2);
            }
        },
        None => vec![5, 15, 30],
    };
    let lists: Vec<String> = match arg_values(&args, "--list") {
        v if v.is_empty() => LISTS.iter().map(|s| s.to_string()).collect(),
        v => v,
    };
    for l in &lists {
        if !LISTS.contains(&l.as_str()) {
            eprintln!("unknown --list {l} (newell|forsyth|liou|frey|ohtani)");
            std::process::exit(2);
        }
    }

    let mcp_all = read_mcpherron(&path);
    let mcp: Vec<f64> = mcp_all
        .into_iter()
        .filter(|&t| t >= start_unix as f64 && t <= stop_unix as f64)
        .collect();
    println!(
        "substorm_compare window {} .. {} (unix {} .. {}), mcpherron {} onsets",
        fmt_date(start_unix),
        fmt_date(stop_unix),
        start_unix,
        stop_unix,
        mcp.len()
    );

    for list in &lists {
        let mut b: Vec<f64> = Vec::new();
        let mut cursor = start_unix;
        while cursor < stop_unix {
            let chunk_stop = (cursor + CHUNK_S).min(stop_unix);
            let url = format!(
                "{BASE}?service=substorms&downloadtype=substorm_list&fmt=csv&start={}&end={}&list={list}",
                fmt_date(cursor),
                fmt_date(chunk_stop)
            );
            let Some(body) = fetch_raw(&url, None, &[]) else {
                eprintln!("substorm_compare: {list} {url} fetch void");
                std::process::exit(1);
            };
            let chunk: Vec<SubstormOnset> = parse_csv(&body);
            for onset in &chunk {
                if onset.t_unix >= start_unix as f64 && onset.t_unix <= stop_unix as f64 {
                    b.push(onset.t_unix);
                }
            }
            cursor = chunk_stop;
        }
        b.sort_by(|a, b| a.total_cmp(b));
        for &tol_min in &tol_min {
            let tol = tol_min as f64 * 60.0;
            let (matched_a, matched_b, med) = match_onsets(&mcp, &b, tol);
            println!(
                "list={list} tol_min={tol_min} n_mcp={} n_list={} hit_mcp={} hit_list={} prec_list={} rec_mcp={} median_dt_s={}",
                mcp.len(),
                b.len(),
                matched_a,
                matched_b,
                ratio(matched_b, b.len()),
                ratio(matched_a, mcp.len()),
                secs(med)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_conversion_matches_the_measured_epoch() {
        assert_eq!(ymdhms_to_unix(1970, 1, 1, 0, 0, 0), 0.0);
        assert_eq!(ymdhms_to_unix(1966, 1, 1, 0, 0, 0), -126230400.0);
    }

    #[test]
    fn matching_counts_both_directions_and_medians_the_gaps() {
        let a = [0.0, 100.0, 1000.0];
        let b = [10.0, 90.0, 5000.0];
        let (matched_a, matched_b, med) = match_onsets(&a, &b, 60.0);
        assert_eq!(matched_a, 2);
        assert_eq!(matched_b, 2);
        assert_eq!(med, Some(10.0));
    }

    #[test]
    fn an_empty_side_leaves_every_count_and_the_median_absent() {
        let (matched_a, matched_b, med) = match_onsets(&[0.0], &[], 60.0);
        assert_eq!(matched_a, 0);
        assert_eq!(matched_b, 0);
        assert_eq!(med, None);
    }
}
