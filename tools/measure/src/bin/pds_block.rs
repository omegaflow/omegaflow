use std::fs;
use std::path::Path;

use omegaflow::archivar::sha256::sha256_hex;

fn or_blank(value: Option<String>) -> String {
    match value {
        Some(v) => v,
        None => String::new(),
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn slug(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

fn quoted(value: &str) -> String {
    if value.contains(char::is_whitespace) {
        format!("\"{value}\"")
    } else {
        value.to_string()
    }
}

fn label_field(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix(key)
            && let Some(value) = rest.trim_start().strip_prefix('=')
        {
            return Some(value.trim().trim_matches('"').trim().to_string());
        }
    }
    None
}

struct Column {
    name: String,
    unit: Option<String>,
}

fn columns(text: &str) -> Vec<Column> {
    let mut out = Vec::new();
    let mut current: Option<Column> = None;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("OBJECT                      = COLUMN") {
            current = Some(Column {
                name: String::new(),
                unit: None,
            });
            continue;
        }
        if t.starts_with("END_OBJECT                  = COLUMN")
            && let Some(column) = current.take()
        {
            out.push(column);
            continue;
        }
        if let Some(column) = current.as_mut() {
            if let Some(rest) = t.strip_prefix("NAME                      = ") {
                column.name = rest.trim().trim_matches('"').to_string();
            } else if let Some(rest) = t.strip_prefix("UNIT                      = ") {
                column.unit = Some(rest.trim().trim_matches('"').to_string());
            }
        }
    }
    out
}

fn measured_step(tab: &str) -> Option<f64> {
    let mut times: Vec<f64> = Vec::new();
    for line in tab.lines().take(400) {
        let stamp = line.trim().get(0..24)?;
        let millis = iso_millis(stamp)?;
        times.push(millis);
    }
    let mut steps: Vec<f64> = Vec::new();
    for pair in times.windows(2) {
        let d = (pair[1] - pair[0]) / 1000.0;
        if d > 0.0 {
            steps.push(d);
        }
    }
    if steps.is_empty() {
        return None;
    }
    steps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    Some(steps[steps.len() / 2])
}

fn iso_millis(stamp: &str) -> Option<f64> {
    let bytes: Vec<char> = stamp.chars().collect();
    if bytes.len() < 19 {
        return None;
    }
    let num = |a: usize, b: usize| -> Option<i64> {
        bytes
            .get(a..b)?
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .ok()
    };
    let year = num(0, 4)?;
    let month = num(5, 7)?;
    let day = num(8, 10)?;
    let hour = num(11, 13)?;
    let minute = num(14, 16)?;
    let second = num(17, 19)?;
    let frac = if bytes.len() > 20 && bytes[19] == '.' {
        let digits: String = bytes[20..]
            .iter()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        digits
            .parse::<f64>()
            .ok()
            .map(|v| v / 10f64.powi(digits.len() as i32))
    } else {
        None
    };
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    let mut unix = days as f64 * 86400.0 + (hour * 3600 + minute * 60 + second) as f64;
    if let Some(v) = frac {
        unix += v;
    }
    Some(unix * 1000.0)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let label_path = match arg_value(&args, "--label") {
        Some(v) => v,
        None => {
            println!(
                "usage: pds_block --label <file.lbl> [--tab <file.tab>] [--dataset <slug>] [--origin <url>] [--anchor <token>]"
            );
            return;
        }
    };
    let dataset = match arg_value(&args, "--dataset") {
        Some(v) => v,
        None => "vega2_mischa".to_string(),
    };
    let anchor = match arg_value(&args, "--anchor") {
        Some(v) => v,
        None => "halley".to_string(),
    };
    let label = match fs::read_to_string(&label_path) {
        Ok(t) => t,
        Err(e) => {
            println!("pds_block: {label_path}: {e}");
            return;
        }
    };
    let columns = columns(&label);
    let rows = or_blank(label_field(&label, "ROWS"));
    let start = or_blank(label_field(&label, "START_TIME"));
    let stop = or_blank(label_field(&label, "STOP_TIME"));
    let description = or_blank(label_field(&label, "DESCRIPTION"));

    let tab_path = arg_value(&args, "--tab");
    let mut step = String::from("pending");
    let mut sha = String::from("pending");
    if let Some(path) = &tab_path {
        if let Ok(tab) = fs::read_to_string(path) {
            if let Some(s) = measured_step(&tab) {
                step = format!("{s}");
            }
        }
        if let Ok(bytes) = fs::read(path.as_str()) {
            sha = sha256_hex(&bytes);
        }
    }

    println!("# generated from {}", Path::new(&label_path).display());
    println!("# label rows {rows}, start {start}, stop {stop}");
    if !description.is_empty() {
        println!("# label description: {description}");
    }
    println!("origin {}", or_blank(arg_value(&args, "--origin")));
    println!("sha256 {sha}");
    println!("coverage {start}..{stop}");
    println!("interval {step}");
    println!("anchor {anchor}");
    for column in &columns {
        if column.name == "UT" || column.name == "FLAG" {
            continue;
        }
        let unit = match &column.unit {
            Some(u) => u.clone(),
            None => continue,
        };
        let name = format!("{dataset}_{}", slug(&column.name));
        println!("field {} {name} {unit}", quoted(&column.name));
    }
}
