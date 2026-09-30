use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds4_fits::{band_means, parse_image};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-geosciences.wustl.edu";
const DATA_ROUTE: &str =
    "https://pds-geosciences.wustl.edu/Lunar/urn-nasa-pds-chang_e_microwave_processed/data/";
const ASSET_PREFIX: &str = "pds4_fits_";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn hrefs(text: &str) -> Vec<String> {
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        match low[start..].find('"') {
            Some(end) => {
                out.push(text[start..start + end].to_string());
                from = start + end;
            }
            None => break,
        }
    }
    out
}

fn fetch_or_read(spec: &str) -> Option<Vec<u8>> {
    if spec.starts_with("http://") || spec.starts_with("https://") {
        fetch_raw_bytes(spec)
    } else {
        std::fs::read(spec).ok()
    }
}

fn asset_name(spec: &str) -> String {
    let base = spec.rsplit('/').next().unwrap_or(spec);
    let last = base.split_once('?').map(|(h, _)| h).unwrap_or(base);
    let stem = last.split('.').next().unwrap_or(last);
    format!("{ASSET_PREFIX}{}.fits", stem.to_ascii_lowercase())
}

fn print_register_lines(asset: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format pds4_fits");
    println!("ttl 604800");
    println!();
}

fn compile_entry(spec: &str, out_dir: Option<&str>, ci_mode: bool) -> Option<String> {
    let Some(bytes) = fetch_or_read(spec) else {
        eprintln!("fits fetch/read void ({spec})");
        return None;
    };
    let Some(raster) = parse_image(&bytes) else {
        eprintln!("{spec}: no IMAGE HDU — the FITS stays untouched (0 honored)");
        return None;
    };
    let Some(means) = band_means(&raster) else {
        eprintln!("{spec}: no band with a finite pixel — the FITS stays unwritten (0 honored)");
        return None;
    };
    let asset = asset_name(spec);
    let out_path = match out_dir {
        Some(dir) => format!("{}/{asset}", dir.trim_end_matches('/')),
        None => format!("data/{NETLOC}/pds4_fits/{asset}"),
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bytes).is_err() {
        eprintln!("write {out_path} returned void");
        return None;
    }
    let written = match std::fs::read(&out_path) {
        Ok(w) => w,
        Err(_) => {
            eprintln!("{out_path}: read-back returned void — the FITS stays unverified");
            return None;
        }
    };
    let digest = sha256_hex(&bytes);
    if sha256_hex(&written) != digest {
        eprintln!("{asset}: read-back sha256 differs — the FITS stays unverified (0 honored)");
        return None;
    }
    eprintln!(
        "{out_path}: {} band(s), {} line(s), {} sample(s), {} B, sha256 {}, {} band mean(s), sha256 read-back holds",
        raster.bands,
        raster.lines,
        raster.samples,
        bytes.len(),
        digest,
        means.len(),
    );
    eprintln!("{asset}: band names [{}]", raster.band_names.join(", "));
    print_register_lines(&asset);
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        return None;
    }
    Some(asset)
}

fn collect_fits(dir_url: &str, out: &mut Vec<String>) {
    let Some(bytes) = fetch_raw_bytes(dir_url) else {
        eprintln!("{dir_url}: listing fetch void");
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{dir_url}: listing not utf8");
        return;
    };
    let base = dir_url.trim_end_matches('/');
    let mut names: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.to_ascii_lowercase().ends_with(".fits"))
        .map(|h| {
            h.trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .collect();
    names.sort();
    names.dedup();
    for name in names {
        out.push(format!("{base}/{name}"));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_arg = arg_value(&args, "--out");
    let mut specs: Vec<String> = Vec::new();
    match arg_value(&args, "--url").or_else(|| arg_value(&args, "--input")) {
        Some(spec) => specs.push(spec),
        None => collect_fits(DATA_ROUTE, &mut specs),
    }
    if specs.is_empty() {
        eprintln!("no .fits found — nothing written (0 honored)");
        std::process::exit(1);
    }
    let mut written = 0usize;
    for spec in &specs {
        if compile_entry(spec, out_arg.as_deref(), ci_mode).is_some() {
            written += 1;
        }
    }
    if written == 0 {
        eprintln!("no FITS manifested — nothing written (0 honored)");
        std::process::exit(1);
    }
    eprintln!("{written} FITS manifested");
}
