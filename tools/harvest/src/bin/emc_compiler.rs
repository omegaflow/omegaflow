use omegaflow::cdn::upload_release;
use omegaflow::emc::{EmcSeries, encode_emc_bin, parse_idv_csv};
use std::process::Command;

const NETLOC: &str = "data.earthscope.org";
const DATA_URL: &str = "https://data.earthscope.org/archive/seismology/products/emc/data/";
const DATA_PATH: &str = "/archive/seismology/products/emc/data/";
const ASSET: &str = "emc_radial.bin";

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
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch {} http {}: {}",
            url,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn dir_hrefs(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(pos) = rest.find(DATA_PATH) {
        let after = &rest[pos + DATA_PATH.len()..];
        let name: String = after.chars().take_while(|c| *c != '/').collect();
        let name_len = name.len();
        if !name.is_empty()
            && !name.contains('.')
            && !name.contains("?type=")
            && !out.contains(&name)
        {
            out.push(name);
        }
        rest = &after[name_len..];
    }
    out
}

fn idv_files(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    let needle = "?type=file\">";
    while let Some(pos) = rest.find(needle) {
        let after = &rest[pos + needle.len()..];
        let name: String = after.chars().take_while(|c| *c != '<').collect();
        if name.ends_with("_IDV.csv") && !name.is_empty() && !out.iter().any(|n| n == &name) {
            out.push(name);
        }
        rest = after;
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut out_dir = String::from(".");
    if let Some(d) = arg_value(&args, "--out") {
        out_dir = d;
    }
    let model_filter = arg_value(&args, "--model");

    let Some(listing) = fetch(DATA_URL) else {
        eprintln!("emc_compiler: the data listing carries void");
        std::process::exit(1);
    };
    let models = dir_hrefs(&listing);
    eprintln!("emc_compiler: {} model directories listed", models.len());

    let mut series: Vec<EmcSeries> = Vec::new();
    for model in &models {
        if let Some(want) = &model_filter {
            if model != want {
                continue;
            }
        }
        let Some(dir_html) = fetch(&format!("{DATA_URL}{model}/")) else {
            continue;
        };
        let idv = idv_files(&dir_html);
        if idv.is_empty() {
            continue;
        }
        for file in &idv {
            let Some(text) = fetch(&format!("{DATA_URL}{model}/{file}")) else {
                continue;
            };
            let variant = file.trim_end_matches("_IDV.csv");
            match parse_idv_csv(&text, model, variant) {
                Some(s) => {
                    eprintln!(
                        "  {}/{}: {} columns, {} rows",
                        s.model,
                        s.variant,
                        s.columns.len(),
                        s.rows.len()
                    );
                    series.push(s);
                }
                None => {
                    eprintln!("  {model}/{variant}: the IDV csv carries no series (0 honored)");
                }
            }
        }
    }

    if series.is_empty() {
        eprintln!("emc_compiler: no series survived the crawl — the asset stays unwritten");
        std::process::exit(1);
    }
    let n_points: usize = series.iter().map(|s| s.rows.len()).sum();
    eprintln!(
        "emc_compiler: {} series, {} rows across {} models",
        n_points,
        series.len(),
        series
            .iter()
            .map(|s| s.model.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    );

    let out_path = format!("{out_dir}/{ASSET}");
    let bytes = encode_emc_bin(&series);
    if let Err(e) = std::fs::write(&out_path, &bytes) {
        eprintln!("emc_compiler: write {} returned {}", out_path, e);
        std::process::exit(1);
    }
    eprintln!("emc_compiler: wrote {} ({} bytes)", out_path, bytes.len());
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("emc_compiler: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_hrefs_extracts_model_names_only() {
        let html = format!(
            "<a href=\"https://host{DATA_PATH}IASP91/\">IASP91</a>\n<a href=\"https://host{DATA_PATH}EARS.lst?type=file\">EARS.lst</a>\n<a href=\"https://host{DATA_PATH}PREM/\">PREM</a>"
        );
        let dirs = dir_hrefs(&html);
        assert_eq!(dirs, vec!["IASP91", "PREM"]);
    }

    #[test]
    fn idv_files_picks_idv_csv_only() {
        let html = "<a href=\"https://host/x/IASP91.csv?type=file\">IASP91.csv</a>\n<a href=\"https://host/x/IASP91_IDV.csv?type=file\">IASP91_IDV.csv</a>\n<a href=\"https://host/x/IASP91.txt?type=file\">IASP91.txt</a>";
        assert_eq!(idv_files(html), vec!["IASP91_IDV.csv"]);
    }
}
