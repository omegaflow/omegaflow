use omegaflow::archivar::curl_base;
use omegaflow::cdn::upload_release;
use std::fs::{self, File};
use std::io::Read;
use std::process::Stdio;

const VIZIER_NETLOC: &str = "vizier.cfa.harvard.edu";
const NAIF_NETLOC: &str = "naif.jpl.nasa.gov";
const OUT_ROOT: &str = "data";
const FETCH_TTL_S: u64 = 86400;
const TSV_TABLES: [&str; 6] = [
    "uranu_j", "ariel_j", "umbri_j", "titan_j", "obero_j", "miran_j",
];
const VIZIER_PREFIX: &str = "https://vizier.cfa.harvard.edu/viz-bin/asu-tsv?-source=J/A/A/582/A8/";
const VIZIER_SUFFIX: &str = "&-out.max=100000";
const SPK_URL: &str =
    "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/satellites/a_old_versions/ura111.bsp";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn tsv_url(table: &str) -> String {
    format!("{VIZIER_PREFIX}{table}{VIZIER_SUFFIX}")
}

fn curl_file(url: &str, path: &str) -> bool {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {path} returned void: {e}");
            return false;
        }
    };
    let mut cmd = curl_base(FETCH_TTL_S, 0);
    cmd.arg(url);
    cmd.stdout(Stdio::from(file));
    match cmd.status() {
        Ok(s) if s.success() => true,
        Ok(s) => {
            let code = match s.code() {
                Some(c) => c.to_string(),
                None => "signal".to_string(),
            };
            eprintln!("fetch {url}: curl exited {code}");
            let _ = fs::remove_file(path);
            false
        }
        Err(e) => {
            eprintln!("fetch {url}: curl absent: {e}");
            let _ = fs::remove_file(path);
            false
        }
    }
}

fn tsv_holds(path: &str) -> bool {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return false,
    };
    if bytes.len() < 64 {
        return false;
    }
    if !bytes.starts_with(b"#") {
        return false;
    }
    let head_end = bytes.len().min(4096);
    if !bytes[..head_end].contains(&b'\t') {
        return false;
    }
    true
}

fn spk_holds(path: &str) -> bool {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };
    if meta.len() < (1 << 20) {
        return false;
    }
    let mut head = [0u8; 8];
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    if file.read_exact(&mut head).is_err() {
        return false;
    }
    &head[..7] == b"DAF/SPK"
}

fn store_asset(url: &str, root: &str, netloc: &str, name: &str, spk: bool) -> Option<String> {
    let path = format!("{root}/{netloc}/{name}");
    let part = format!("{path}.part");
    if !curl_file(url, &part) {
        return None;
    }
    let holds = if spk {
        spk_holds(&part)
    } else {
        tsv_holds(&part)
    };
    if !holds {
        let _ = fs::remove_file(&part);
        return None;
    }
    if fs::rename(&part, &path).is_err() {
        let _ = fs::remove_file(&part);
        return None;
    }
    let len = match fs::metadata(&path) {
        Ok(m) => m.len(),
        Err(_) => 0,
    };
    eprintln!("manifest {}: origin verbatim, {} bytes", path, len);
    Some(path)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let root = match arg_value(&args, "--out") {
        Some(r) => r,
        None => OUT_ROOT.to_string(),
    };
    let mut staged: Vec<(String, &str)> = Vec::new();
    for table in TSV_TABLES {
        let name = format!("camargo_{table}.tsv");
        if let Some(path) = store_asset(&tsv_url(table), &root, VIZIER_NETLOC, &name, false) {
            staged.push((path, VIZIER_NETLOC));
        }
    }
    if let Some(path) = store_asset(SPK_URL, &root, NAIF_NETLOC, "ura111.bsp", true) {
        staged.push((path, NAIF_NETLOC));
    }
    if staged.len() < 7 {
        eprintln!("manifestor: {} of 7 assets present", staged.len());
        std::process::exit(1);
    }
    if !ci_mode {
        eprintln!(
            "manifestor: {} assets staged, no upload without --ci-mode",
            staged.len()
        );
        return;
    }
    for (path, tag) in &staged {
        eprintln!("upload {path} -> cdn tag {tag}");
        if !upload_release(tag, path) {
            eprintln!("upload {path} returned void");
            std::process::exit(1);
        }
    }
}
