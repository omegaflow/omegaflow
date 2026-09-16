use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::ifms_agc::{parse_ifms_agc, parse_series, write_series};
use omegaflow::cdn::upload_release;

const BASE: &str = "https://archives.esac.esa.int/psa/ftp/INTERNATIONAL-ROSETTA-MISSION/RSI/";
const ODF_DIR: &str = "DATA/LEVEL1A/CLOSED_LOOP/IFMS/";
const ODF_SUBDIRS: &[&str] = &["AG1", "AG2", "DP1", "DP2"];

fn hrefs(text: &str) -> Vec<String> {
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        let Some(end) = low[start..].find('"') else {
            break;
        };
        out.push(text[start..start + end].to_string());
        from = start + end;
    }
    out
}

fn name_of(href: &str) -> String {
    href.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string()
}

fn bundles() -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(BASE, 604800) else {
        eprintln!("bundle listing fetch void ({BASE})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("bundle listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.ends_with('/'))
        .map(|h| name_of(&h))
        .filter(|n| {
            let l = n.to_ascii_lowercase();
            l.starts_with("ro-") && l.contains("-rsi-")
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_of(bundle: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for sub in ODF_SUBDIRS {
        let dir = format!("{BASE}{bundle}/{ODF_DIR}{sub}/");
        let Some(bytes) = fetch_raw_bytes(&dir, 604800) else {
            eprintln!("{bundle}/{sub}: odf listing fetch void ({dir})");
            continue;
        };
        let Ok(text) = std::str::from_utf8(&bytes) else {
            eprintln!("{bundle}/{sub}: odf listing not utf8");
            continue;
        };
        let mut rels: Vec<String> = hrefs(text)
            .into_iter()
            .filter(|h| h.to_ascii_lowercase().ends_with(".raw"))
            .map(|h| format!("{sub}/{}", name_of(&h)))
            .collect();
        out.append(&mut rels);
    }
    out.sort();
    out.dedup();
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut merged: Vec<(f64, f64, f64)> = Vec::new();
    let bundles = bundles();
    eprintln!(
        "INTERNATIONAL-ROSETTA-MISSION/RSI: {} bundles",
        bundles.len()
    );
    for bundle in bundles {
        let mut kept_bundle = 0usize;
        for rel in files_of(&bundle) {
            let url = format!("{BASE}{bundle}/{ODF_DIR}{rel}");
            let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
                eprintln!("{bundle}/{rel}: fetch void ({url})");
                continue;
            };
            let Some(file) = parse_ifms_agc(&bytes) else {
                eprintln!("{bundle}/{rel}: parse void — {} B", bytes.len());
                continue;
            };
            for s in &file.samples {
                merged.push((s.unix_time, s.carrier_level_dbm, s.polar_angle_cycles));
            }
            kept_bundle += file.samples.len();
            eprintln!(
                "{bundle}/{rel}: {} AGC samples ({} header fields)",
                file.samples.len(),
                file.fields.len()
            );
        }
        eprintln!("{bundle}: {kept_bundle} samples merged");
    }
    if merged.is_empty() {
        eprintln!("no Rosetta RSI IFMS AGC samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a.0.total_cmp(&b.0));
    let out = "data/archives.esac.esa.int/rosetta_odf.bin";
    std::fs::create_dir_all("data/archives.esac.esa.int").ok();
    let bin = write_series(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    let (first, last) = match parse_series(&bin) {
        Some(parsed) => match (parsed.first(), parsed.last()) {
            (Some(a), Some(b)) => (a.0, b.0),
            _ => {
                eprintln!("{out}: roundtrip parse empty — the series stays unverified");
                return;
            }
        },
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            return;
        }
    };
    eprintln!(
        "{out}: {} AGC samples (unix {first}..{last}), {} B — roundtrip parses",
        merged.len(),
        bin.len()
    );
    if ci_mode && !upload_release("archives.esac.esa.int", out) {
        std::process::exit(1);
    }
}
