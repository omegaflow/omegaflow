use omegaflow::archivar::curl_base;
use omegaflow::cdn::upload_release;
use std::fs::{self, File};
use std::process::Stdio;

const GEOAZUR_NETLOC: &str = "www.geoazur.fr";
const BASE_URL: &str = "https://www.geoazur.fr/astrogeo/observations/base/podb/DATABASE/";
const OUT_ROOT: &str = "data";
const FETCH_TTL_S: u64 = 86400;

const FILES: [(&str, &str); 23] = [
    ("apdb_format.txt", "format.txt"),
    ("apdb_hilton_type.instr", "HILTON/type.instr"),
    ("apdb_neptune_ccd_flgs", "URSS/NEPT/CCD/flgs.ccd.ftp"),
    ("apdb_neptune_jpl_nptsobs", "JPL/APDB/nptsobs.txt"),
    ("apdb_neptune_photo_nik", "URSS/NEPT/PHOTO/nik.pht.ftp"),
    ("apdb_neptune_transit_besa", "HILTON/NEPTUNE/BESA"),
    ("apdb_neptune_transit_camb", "HILTON/NEPTUNE/CAMB"),
    ("apdb_neptune_transit_cape", "HILTON/NEPTUNE/CAPE"),
    (
        "apdb_neptune_transit_golo",
        "URSS/NEPT/TRANSIT/golo.trn.ftp",
    ),
    ("apdb_neptune_transit_gren", "HILTON/NEPTUNE/GREN"),
    ("apdb_neptune_transit_gtok", "HILTON/NEPTUNE/GTOK"),
    ("apdb_neptune_transit_nice", "HILTON/NEPTUNE/NICE"),
    ("apdb_neptune_transit_nik", "URSS/NEPT/TRANSIT/nik.trn.ftp"),
    ("apdb_neptune_transit_pari", "HILTON/NEPTUNE/PARI"),
    ("apdb_neptune_transit_radc", "HILTON/NEPTUNE/RADC"),
    ("apdb_neptune_transit_stra", "HILTON/NEPTUNE/STRA"),
    ("apdb_neptune_transit_tky", "URSS/NEPT/TRANSIT/tky.trn.ftp"),
    ("apdb_neptune_transit_toul", "HILTON/NEPTUNE/TOUL"),
    ("apdb_neptune_transit_uccl", "HILTON/NEPTUNE/UCCL"),
    ("apdb_neptune_transit_usno", "HILTON/NEPTUNE/USNO"),
    (
        "apdb_neptune_transit_usno_urss",
        "URSS/NEPT/TRANSIT/usno.trn.ftp",
    ),
    ("apdb_obslist.opt", "OBSLIST.OPT"),
    ("apdb_ref_opt.txt", "Ref_opt.txt"),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn apdb_holds(path: &str) -> bool {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return false,
    };
    if bytes.is_empty() {
        return false;
    }
    let head_end = bytes.len().min(256);
    if bytes[..head_end].windows(5).any(|w| w == b"<html") {
        return false;
    }
    bytes.iter().any(|b| *b == b'\n')
}

fn store_asset(url: &str, root: &str, name: &str) -> Option<String> {
    let path = format!("{root}/{GEOAZUR_NETLOC}/{name}");
    let part = format!("{path}.part");
    if !curl_file(url, &part) {
        return None;
    }
    if !apdb_holds(&part) {
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
    eprintln!("manifest {path}: origin verbatim, {len} bytes");
    Some(path)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let root = match arg_value(&args, "--out") {
        Some(r) => r,
        None => OUT_ROOT.to_string(),
    };
    let mut staged: Vec<String> = Vec::new();
    for (name, remote) in FILES {
        let url = format!("{BASE_URL}{remote}");
        if let Some(path) = store_asset(&url, &root, name) {
            staged.push(path);
        }
    }
    if staged.len() < FILES.len() {
        eprintln!(
            "manifestor: {} of {} assets present",
            staged.len(),
            FILES.len()
        );
        std::process::exit(1);
    }
    if !ci_mode {
        eprintln!(
            "manifestor: {} assets staged, no upload without --ci-mode",
            staged.len()
        );
        return;
    }
    for path in &staged {
        eprintln!("upload {path} -> cdn tag {GEOAZUR_NETLOC}");
        if !upload_release(GEOAZUR_NETLOC, path) {
            eprintln!("upload {path} returned void");
            std::process::exit(1);
        }
    }
}
