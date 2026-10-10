use omegaflow::archivar::json::{JsonVal, parse_json};

const BASE: &str = "https://ssd-api.jpl.nasa.gov/sb_radar.api";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn positional(args: &[String]) -> Option<String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--params" {
            i += 2;
            continue;
        }
        if !args[i].starts_with('-') {
            return Some(args[i].clone());
        }
        i += 1;
    }
    None
}

fn scalar_text(v: &JsonVal) -> Option<String> {
    match v {
        JsonVal::Str(s) => Some(s.clone()),
        JsonVal::Num(n) => Some(format!("{n}")),
        JsonVal::Bool(b) => Some(b.to_string()),
        JsonVal::Null => None,
        JsonVal::Arr(_) | JsonVal::Obj(_) => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: sb_radar_compiler <query> | --params <k=v&k=v> [--inspect]");
        eprintln!("  --params <k=v&k=v> the URL query string appended to {BASE}");
        eprintln!("  --inspect          print the response field names and stop");
        eprintln!("  the physical mapping (radar delay/doppler -> the 26 x f64 wire) is pending;");
        eprintln!("  this compiler prints field=value rows only and writes no wire record");
        std::process::exit(2);
    }
    let inspect = args.iter().any(|a| a == "--inspect");
    let params = match arg_value(&args, "--params") {
        Some(p) if !p.is_empty() => p,
        _ => match positional(&args) {
            Some(p) => p,
            None => {
                eprintln!("sb_radar_compiler needs a positional query or --params <k=v&k=v>");
                std::process::exit(2);
            }
        },
    };
    let url = format!("{BASE}?{params}");
    let Some(body) = omegaflow::archivar::fetch_raw(&url, None, &[]) else {
        eprintln!("{url} fetch void — no row is printed (0 honored)");
        std::process::exit(1);
    };
    let Some(JsonVal::Obj(root)) = parse_json(&body) else {
        eprintln!("{url} carries no JSON object — no row is printed");
        std::process::exit(1);
    };
    let fields: Vec<String> = match root.get("fields") {
        Some(JsonVal::Arr(arr)) => arr
            .iter()
            .filter_map(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => {
            eprintln!("{url} carries no fields array — no row is printed");
            std::process::exit(1);
        }
    };
    if inspect {
        for field in &fields {
            println!("{field}");
        }
        std::process::exit(0);
    }
    let data = match root.get("data") {
        Some(JsonVal::Arr(arr)) => arr,
        _ => {
            eprintln!("{url} carries no data array — no row is printed");
            std::process::exit(1);
        }
    };
    let mut rows = 0usize;
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        let mut parts: Vec<String> = Vec::new();
        for (i, field) in fields.iter().enumerate() {
            if let Some(cell) = cells.get(i).and_then(scalar_text) {
                parts.push(format!("{field}={cell}"));
            }
        }
        println!("{}", parts.join(" "));
        rows += 1;
    }
    println!("sb_radar_compiler: {rows} rows, {} fields", fields.len());
}
