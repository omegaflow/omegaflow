use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://pds-ppi.igpp.ucla.edu/data/mess-rs-raw/data-tnf/";
const NETLOC: &str = "pds-ppi.igpp.ucla.edu";
const PREFIX: &str = "messenger_tnf";
const DIR: &str = "data/pds-ppi.igpp.ucla.edu";

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

fn years() -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(BASE) else {
        eprintln!("year listing fetch void ({BASE})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("year listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.ends_with('/'))
        .map(|h| {
            h.trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .filter(|n| n.len() == 4 && n.chars().all(|c| c.is_ascii_digit()))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn files_of(year: &str) -> Vec<String> {
    let dir = format!("{BASE}{year}/");
    let Some(bytes) = fetch_raw_bytes(&dir) else {
        eprintln!("{year}: tnf listing fetch void ({dir})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{year}: tnf listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.to_ascii_lowercase().ends_with("_tnf.dat"))
        .map(|h| h.rsplit('/').next().unwrap_or("").to_string())
        .collect();
    out.sort();
    out.dedup();
    out
}

fn write_and_verify(records: &[[f64; 9]], out: &str) -> Vec<u8> {
    let bin = odf::write_podf_bin(records);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            eprintln!(
                "{out}: {} TNF samples (tdb {}..{}), {} B — roundtrip parses",
                parsed.len(),
                d0[0],
                d1[0],
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            std::process::exit(1);
        }
    }
    bin
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let year_only = args
        .iter()
        .position(|a| a == "--year")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 9]> = Vec::new();
    let yrs = match &year_only {
        Some(y) => vec![y.clone()],
        None => years(),
    };
    eprintln!("mess-rs-raw/data-tnf: {} years", yrs.len());
    for year in yrs {
        let files = files_of(&year);
        eprintln!("{year}: {} tnf files", files.len());
        let mut kept_year = 0usize;
        for rel in files {
            let url = format!("{BASE}{year}/{rel}");
            let Some(bytes) = fetch_raw_bytes(&url) else {
                eprintln!("{year}/{rel}: fetch void ({url})");
                continue;
            };
            let Some(recs) = odf::tnf_rows(&bytes, &lsk) else {
                eprintln!("{year}/{rel}: tnf scan void — {} B", bytes.len());
                continue;
            };
            kept_year += recs.len();
            merged.extend(recs);
        }
        eprintln!("{year}: {kept_year} samples merged");
    }
    if merged.is_empty() {
        eprintln!("no Messenger TNF samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let mut codes: std::collections::BTreeMap<i64, usize> = std::collections::BTreeMap::new();
    for r in &merged {
        *codes.entry(r[odf::TNF_ROW_FORMAT] as i64).or_insert(0) += 1;
    }
    eprintln!("messenger TNF rows by format code: {codes:?}");
    std::fs::create_dir_all(DIR).ok();

    let ranges = odf::podf_shard_ranges(merged.len(), odf::PODF_SHARD_BUDGET);
    if ranges.len() == 1 {
        let out = format!("{DIR}/{PREFIX}.bin");
        write_and_verify(&merged, &out);
        if ci_mode && !upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
        return;
    }

    let mut names: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    for (ord, &(lo, hi)) in ranges.iter().enumerate() {
        let t_lo = merged[lo][0];
        let t_hi = merged[hi - 1][0];
        let mut name = odf::podf_shard_name(PREFIX, t_lo, t_hi);
        if names.contains(&name) {
            name = odf::podf_shard_name_ord(PREFIX, t_lo, t_hi, ord);
        }
        names.push(name.clone());
        let path = format!("{DIR}/{name}");
        let bin = write_and_verify(&merged[lo..hi], &path);
        if bin.len() > odf::PODF_SHARD_LIMIT {
            eprintln!(
                "{path}: {}-byte shard exceeds the {}-byte CDN asset limit — the series stays unwritten (0 honored)",
                bin.len(),
                odf::PODF_SHARD_LIMIT
            );
            std::process::exit(1);
        }
        paths.push(path);
    }
    for name in &names {
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format {PREFIX}");
        println!("at earth");
        println!("ttl 604800");
        println!(
            "field ul_phase_cycles {PREFIX}_ul_phase_cycles inverse-square em cycle 604800 0.0 0.0"
        );
        println!();
    }
    if ci_mode {
        for path in &paths {
            if !upload_release(NETLOC, path) {
                std::process::exit(1);
            }
        }
    }
}
