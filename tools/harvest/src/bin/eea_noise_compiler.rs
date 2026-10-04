use omegaflow::archivar::eea_noise::parse_series;
use omegaflow::cdn::upload_release;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "eea.europa.eu";
const BASE: &str = "https://noise.discomap.eea.europa.eu/arcgis/rest/services/noiseStoryMap/Noise_exposure_2025/MapServer/76/query";
const ASSET: &str = "eea_noise_2025.json";

const FIELDS: [&str; 15] = [
    "SNLD55",
    "SNLD65",
    "SNLD75",
    "SNLN50",
    "SNLN60",
    "SNLN70",
    "NLD5559",
    "NLD6064",
    "NLD6569",
    "NLD7074",
    "NLN5054",
    "NLN5559",
    "NLN6064",
    "NLN6569",
    "N_INHAB",
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch() -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg("-G")
        .arg(BASE)
        .arg("--data-urlencode")
        .arg("where=1=1")
        .arg("--data-urlencode")
        .arg(format!("outFields={}", FIELDS.join(",")))
        .arg("--data-urlencode")
        .arg("returnGeometry=false")
        .arg("--data-urlencode")
        .arg("resultRecordCount=2000")
        .arg("--data-urlencode")
        .arg("f=json")
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "eea_noise_compiler: fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join(ASSET)
            .to_string_lossy()
            .into_owned(),
    };

    let Some(bytes) = fetch() else {
        eprintln!("eea_noise_compiler: fetch void — no asset (0 honored)");
        std::process::exit(1);
    };
    match parse_series(&bytes) {
        Some(rows) if !rows.is_empty() => {
            eprintln!("eea_noise_compiler: {} record(s), {} B", rows.len(), bytes.len())
        }
        Some(_) => {
            eprintln!(
                "eea_noise_compiler: no declared counter survived the plausibility gate — no asset (0 honored)"
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("eea_noise_compiler: {} parses void — no asset", ASSET);
            std::process::exit(1);
        }
    }

    if let Some(parent) = Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("eea_noise_compiler: write {out} returned void");
        std::process::exit(1);
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("eea_noise_compiler: CDN upload returned void");
        std::process::exit(1);
    }
}
