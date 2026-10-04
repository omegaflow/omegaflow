use omegaflow::archivar::wqp_result::parse_series;
use omegaflow::cdn::upload_release;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "waterqualitydata.us";
const BASE: &str = "https://www.waterqualitydata.us/data/Result/search";
const ASSET: &str = "wqp_result.csv.zip";
const STATE_DEFAULT: &str = "US:19";
const FROM_DEFAULT: &str = "01-01-2020";
const TO_DEFAULT: &str = "12-31-2020";

const CHARACTERISTICS: [&str; 10] = [
    "Temperature",
    "pH",
    "Dissolved oxygen",
    "Conductivity",
    "Nitrate-N",
    "Ammonia",
    "Orthophosphate",
    "Chloride",
    "Atrazine",
    "Metolachlor",
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(state: &str, from: &str, to: &str) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg("-G")
        .arg(BASE)
        .arg("--data-urlencode")
        .arg(format!("statecode={state}"))
        .arg("--data-urlencode")
        .arg(format!("startDateLo={from}"))
        .arg("--data-urlencode")
        .arg(format!("startDateHi={to}"));
    for characteristic in CHARACTERISTICS {
        cmd.arg("--data-urlencode")
            .arg(format!("characteristicName={characteristic}"));
    }
    cmd.arg("--data-urlencode")
        .arg("mimeType=csv")
        .arg("--data-urlencode")
        .arg("zip=yes");
    let out = cmd.output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "wqp_result_compiler: fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let state = match arg_value(&args, "--state") {
        Some(v) => v,
        None => STATE_DEFAULT.to_string(),
    };
    let from = match arg_value(&args, "--from") {
        Some(v) => v,
        None => FROM_DEFAULT.to_string(),
    };
    let to = match arg_value(&args, "--to") {
        Some(v) => v,
        None => TO_DEFAULT.to_string(),
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join(ASSET)
            .to_string_lossy()
            .into_owned(),
    };

    let Some(bytes) = fetch(&state, &from, &to) else {
        eprintln!("wqp_result_compiler: {state} {from}..{to}: fetch void — no asset (0 honored)");
        std::process::exit(1);
    };
    match parse_series(&bytes) {
        Some(rows) if !rows.is_empty() => eprintln!(
            "wqp_result_compiler: {state} {from}..{to}: {} record(s), {} B",
            rows.len(),
            bytes.len()
        ),
        Some(_) => {
            eprintln!(
                "wqp_result_compiler: {state} {from}..{to}: no declared component survived the plausibility gate — no asset (0 honored)"
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("wqp_result_compiler: {} parses void — no asset", ASSET);
            std::process::exit(1);
        }
    }

    if let Some(parent) = Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("wqp_result_compiler: write {out} returned void");
        std::process::exit(1);
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("wqp_result_compiler: CDN upload returned void");
        std::process::exit(1);
    }
}
