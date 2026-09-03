use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::radio::{RADIO_BIN_WIDTH_HZ, RADIO_FREQ_HZ, RadioSource, write_bin};
use omegaflow::cdn::upload_asset;
use std::path::Path;

const NVSS_DEFAULT: &str = "/tmp/opencode/nvss.json";
const FIRST_DEFAULT: &str = "/tmp/opencode/first14.json";
const OUT_DEFAULT: &str = "/tmp/opencode/radio.bin";
const MJY_TO_WM2HZ: f64 = 1e-29;

fn val_of(el: &JsonVal, key: &str) -> Option<f64> {
    match el {
        JsonVal::Obj(map) => map.get(key).and_then(|v| match v {
            JsonVal::Num(n) if n.is_finite() => Some(*n),
            _ => None,
        }),
        _ => None,
    }
}

fn array_of(bytes: &[u8], path: &str) -> Option<Vec<JsonVal>> {
    let text = String::from_utf8_lossy(bytes);
    let j = parse_json(&text)?;
    match j {
        JsonVal::Arr(arr) => Some(arr),
        _ => {
            eprintln!("{path}: expected a JSON array at top level");
            None
        }
    }
}

fn flux_of(el: &JsonVal, keys: &[&str]) -> Option<f64> {
    for k in keys {
        if let Some(v) = val_of(el, k) {
            if v > 0.0 {
                return Some(v);
            }
        }
    }
    None
}

fn run(
    nvss_path: &str,
    first_path: &str,
    out_path: &str,
    ci: bool,
    nvss_only: bool,
) -> Result<(), String> {
    let mut sources: Vec<RadioSource> = Vec::new();

    if Path::new(nvss_path).exists() {
        let bytes = std::fs::read(nvss_path).map_err(|e| format!("{nvss_path}: {e}"))?;
        let arr = array_of(&bytes, nvss_path).ok_or("nvss: array parse void")?;
        let before = sources.len();
        for el in &arr {
            let (Some(ra), Some(dec)) = (val_of(el, "ra"), val_of(el, "dec")) else {
                continue;
            };
            let Some(flux_mjy) = flux_of(el, &["flux", "S1_4", "fpeak", "fint"]) else {
                continue;
            };
            sources.push(RadioSource {
                ra_deg: ra,
                dec_deg: dec,
                plx_mas: 0.0,
                freq: RADIO_FREQ_HZ,
                bin_width: RADIO_BIN_WIDTH_HZ,
                flux: flux_mjy * MJY_TO_WM2HZ,
            });
        }
        eprintln!("nvss: +{} sources", sources.len() - before);
    } else {
        eprintln!("nvss: {nvss_path} absent — skip");
    }

    if !nvss_only && Path::new(first_path).exists() {
        let bytes = std::fs::read(first_path).map_err(|e| format!("{first_path}: {e}"))?;
        let arr = if bytes.is_empty() {
            eprintln!("first14: {first_path} empty — skip");
            vec![]
        } else {
            array_of(&bytes, first_path).ok_or("first14: array parse void")?
        };
        let before = sources.len();
        for el in &arr {
            let (Some(ra), Some(dec)) = (val_of(el, "ra"), val_of(el, "dec")) else {
                continue;
            };
            let Some(flux_mjy) = flux_of(el, &["fpeak", "fint", "flux"]) else {
                continue;
            };
            sources.push(RadioSource {
                ra_deg: ra,
                dec_deg: dec,
                plx_mas: 0.0,
                freq: RADIO_FREQ_HZ,
                bin_width: RADIO_BIN_WIDTH_HZ,
                flux: flux_mjy * MJY_TO_WM2HZ,
            });
        }
        eprintln!("first14: +{} sources", sources.len() - before);
    } else if nvss_only {
        eprintln!("first14: --nvss-only — FIRST sources skipped");
    } else {
        eprintln!("first14: {first_path} absent — skip");
    }

    if sources.is_empty() {
        return Err(
            "no radio sources harvested — the output would be an empty bin (refused)".into(),
        );
    }

    let bytes = write_bin(&sources).ok_or("write_bin: non-finite value refused")?;
    std::fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    println!("radio.bin: {} sources, {} B", sources.len(), bytes.len());

    if ci {
        if !upload_asset(out_path) {
            return Err(format!("{out_path}: CDN upload returned void"));
        }
        println!("radio.bin: uploaded to the CDN");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut nvss = NVSS_DEFAULT.to_string();
    let mut first = FIRST_DEFAULT.to_string();
    let mut out = OUT_DEFAULT.to_string();
    let mut ci = false;
    let mut nvss_only = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--nvss" => {
                i += 1;
                nvss = args.get(i).cloned().unwrap_or(NVSS_DEFAULT.to_string());
            }
            "--first" => {
                i += 1;
                first = args.get(i).cloned().unwrap_or(FIRST_DEFAULT.to_string());
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(OUT_DEFAULT.to_string());
            }
            "--ci-mode" => ci = true,
            "--nvss-only" => nvss_only = true,
            other => {
                eprintln!("radio_compiler: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if let Err(msg) = run(&nvss, &first, &out, ci, nvss_only) {
        eprintln!("radio_compiler: {msg}");
        std::process::exit(1);
    }
}
