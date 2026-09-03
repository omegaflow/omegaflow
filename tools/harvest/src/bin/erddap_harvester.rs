use omegaflow::json::{JsonVal, parse_json};
use std::env;
use std::fs;
use std::process::Command;

fn as_str(j: &JsonVal) -> String {
    match j {
        JsonVal::Str(s) => s.clone(),
        JsonVal::Num(n) => format!("{}", n),
        JsonVal::Bool(b) => format!("{}", b),
        _ => String::new(),
    }
}

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut root: Option<String> = None;
    let mut out: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let Some(root) = root else {
        eprintln!("--root absent");
        std::process::exit(1);
    };
    let url = format!("{}/erddap/info/index.json", root.trim_end_matches('/'));
    let Some(body) = curl(&url) else {
        eprintln!("erddap: {} returned void", url);
        std::process::exit(1);
    };
    let Some(parsed) = parse_json(&body) else {
        eprintln!("erddap: index.json not parseable");
        std::process::exit(1);
    };
    let JsonVal::Obj(top) = parsed else {
        eprintln!("erddap: top not object");
        std::process::exit(1);
    };
    let Some(JsonVal::Obj(table)) = top.get("table") else {
        eprintln!("erddap: table absent");
        std::process::exit(1);
    };
    let cols: Vec<String> = match table.get("columnNames") {
        Some(JsonVal::Arr(a)) => a.iter().map(as_str).collect(),
        _ => Vec::new(),
    };
    let rows: &Vec<JsonVal> = match table.get("rows") {
        Some(JsonVal::Arr(a)) => a,
        _ => {
            eprintln!("erddap: rows absent");
            std::process::exit(1);
        }
    };
    let title_idx = cols.iter().position(|c| c == "Title");
    let dsid_idx = cols.iter().position(|c| c == "Dataset ID");
    let (Some(ti), Some(di)) = (title_idx, dsid_idx) else {
        eprintln!("erddap: Title/Dataset ID absent, available: {:?}", cols);
        std::process::exit(1);
    };
    let mut buf = String::new();
    let mut total = 0usize;
    for row in rows {
        if let JsonVal::Arr(cells) = row {
            if cells.len() > di && cells.len() > ti {
                let dsid = as_str(&cells[di]);
                let title = as_str(&cells[ti]);
                if !dsid.is_empty() {
                    buf.push_str(&format!("{} | {}\n", dsid, title));
                    total += 1;
                }
            }
        }
    }
    if let Some(path) = out {
        let _ = fs::write(&path, &buf);
        eprintln!("erddap: {} records → {}", total, path);
    } else {
        eprintln!("erddap: {} records (--out absent)", total);
    }
}
