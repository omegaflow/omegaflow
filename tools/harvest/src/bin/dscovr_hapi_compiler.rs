use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "cdaweb.gsfc.nasa.gov";
const BASE: &str = "https://cdaweb.gsfc.nasa.gov/hapi/data";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "dscovr_hapi_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn header_names(body: &str) -> Vec<String> {
    let mut meta = String::new();
    for line in body.lines() {
        let text = line.trim();
        if !text.starts_with('#') {
            break;
        }
        meta.push_str(text.trim_start_matches('#'));
        meta.push('\n');
    }
    if let Some(JsonVal::Obj(root)) = parse_json(&meta)
        && let Some(JsonVal::Arr(parameters)) = root.get("parameters")
    {
        let mut names = Vec::new();
        for parameter in parameters {
            let JsonVal::Obj(fields) = parameter else {
                continue;
            };
            let Some(JsonVal::Str(name)) = fields.get("name") else {
                continue;
            };
            let width = match fields.get("size") {
                Some(JsonVal::Arr(size)) => match size.first() {
                    Some(JsonVal::Num(n)) if *n >= 1.0 => *n as usize,
                    _ => 1,
                },
                _ => 1,
            };
            for k in 0..width {
                if width == 1 {
                    names.push(name.clone());
                } else {
                    names.push(format!("{name}.{}", k + 1));
                }
            }
        }
        if !names.is_empty() {
            return names;
        }
    }
    let mut names = Vec::new();
    for line in body.lines() {
        let text = line.trim();
        if !text.starts_with('#') {
            break;
        }
        let inner = text.trim_start_matches('#').trim();
        if let Some(rest) = inner.strip_prefix("\"name\":") {
            let name = rest
                .trim()
                .trim_end_matches(',')
                .trim()
                .trim_matches('"')
                .to_string();
            if !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

fn split_row(line: &str) -> Vec<&str> {
    line.split(',')
        .map(|c| c.trim().trim_matches('"'))
        .collect()
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let start = arg_value(args, "--start").ok_or("--start <iso> required")?;
    let stop = arg_value(args, "--stop").ok_or("--stop <iso> required")?;
    let value_col: usize = match arg_value(args, "--value-column") {
        Some(v) => v
            .parse()
            .map_err(|_| "--value-column is not an integer".to_string())?,
        None => 2,
    };
    if value_col == 0 {
        return Err("--value-column is 1-based and must be at least 1".to_string());
    }
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/dscovr_h1_fc.txt"),
    };

    let url = format!(
        "{BASE}?id=DSCOVR_H1_FC&time.min={start}&time.max={stop}&format=csv&include=header"
    );
    let body = curl(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let names = header_names(&body);
    let column = match names.get(value_col) {
        Some(c) => c.clone(),
        None => return Err(format!("HAPI header has no column {value_col}")),
    };

    let mut rows: Vec<(String, f64)> = Vec::new();
    for line in body.lines() {
        let text = line.trim();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        let cells = split_row(text);
        if cells.len() <= value_col {
            continue;
        }
        let stamp = cells[0];
        if stamp.is_empty() {
            continue;
        }
        let Ok(value) = cells[value_col].parse::<f64>() else {
            continue;
        };
        if !value.is_finite() {
            continue;
        }
        rows.push((stamp.to_string(), value));
    }
    if rows.is_empty() {
        return Err(format!(
            "{url} carries no measured {column} row for {start}..{stop}"
        ));
    }

    let mut text = format!("# DSCOVR H1 FC {start}..{stop}\n");
    text.push_str(&format!("# column {column}\n"));
    for (stamp, value) in &rows {
        text.push_str(&format!("{stamp} {value}\n"));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "dscovr_hapi_compiler: {} {column} rows ({} B) written to {out}",
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
        eprintln!("dscovr_hapi_compiler: {msg}");
        std::process::exit(2);
    }
}
