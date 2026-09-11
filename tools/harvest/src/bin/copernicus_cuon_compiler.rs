use omegaflow::archivar::copernicus::{parse_cdm_obs, retrieve};
use omegaflow::archivar::geo::{parse_bin, write_bin, MAGIC_CDM};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;

const DATASET: &str = "insitu-comprehensive-upper-air-observation-network";
const NETLOC: &str = "cds.climate.copernicus.eu";
const EXTRA_INPUTS: &str = "\"version\":\"1_1_0\",";
const VARIABLES: &[&str] = &[
    "air_dewpoint",
    "air_temperature",
    "dew_point_depression",
    "eastward_wind_speed",
    "geopotential_height",
    "northward_wind_speed",
    "relative_humidity",
    "specific_humidity",
    "wind_from_direction",
    "wind_speed",
];

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn parse_area(text: &str) -> Option<[f64; 4]> {
    let parts: Vec<f64> = text
        .split(',')
        .filter_map(|p| p.trim().parse::<f64>().ok())
        .collect();
    match parts.as_slice() {
        [north, west, south, east] => Some([*north, *west, *south, *east]),
        _ => None,
    }
}

fn build_inputs(year: &str, month: Option<&str>, day: Option<&str>, area: [f64; 4]) -> String {
    let vars = VARIABLES
        .iter()
        .map(|v| format!("\"{v}\""))
        .collect::<Vec<_>>()
        .join(",");
    let mut body = format!("{{\"inputs\":{{{EXTRA_INPUTS}\"year\":[\"{year}\"],");
    if let Some(m) = month {
        body.push_str(&format!("\"month\":[\"{m}\"],"));
    }
    if let Some(d) = day {
        body.push_str(&format!("\"day\":[\"{d}\"],"));
    }
    body.push_str(&format!(
        "\"area\":[{},{},{},{}],\"variable\":[{vars}],\"data_format\":\"csv\"}}}}",
        area[0], area[1], area[2], area[3]
    ));
    body
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let year = match arg_value(&args, "--year") {
        Some(v) => v,
        None => {
            eprintln!("--year (e.g. 2020) required");
            std::process::exit(1);
        }
    };
    let month = arg_value(&args, "--month");
    let day = arg_value(&args, "--day");
    let area = match arg_value(&args, "--area").as_deref().map(parse_area) {
        Some(Some(a)) => a,
        None => [90.0, -180.0, -90.0, 180.0],
        Some(None) => {
            eprintln!("--area reads void — the order is north,west,south,east");
            std::process::exit(1);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let lsk = match arg_value(&args, "--lsk")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| parse_lsk(&t))
    {
        Some(l) => l,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void");
            std::process::exit(1);
        }
    };
    let token = match secret("CDS_API_KEY") {
        Some(t) => t,
        None => {
            eprintln!("CDS_API_KEY absent (.secrets.local) — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let inputs = build_inputs(&year, month.as_deref(), day.as_deref(), area);
    let csv = match retrieve(DATASET, &inputs, &token, 3600) {
        Some(b) => b,
        None => {
            eprintln!("{DATASET}: retrieval void — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&csv);
    let records = parse_cdm_obs(&text, &lsk);
    if records.is_empty() {
        eprintln!("{DATASET}: no measured records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(MAGIC_CDM, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_CDM, &bytes) {
        Some(parsed) => eprintln!(
            "{DATASET}: {} geo records written, roundtrip parses",
            parsed.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
