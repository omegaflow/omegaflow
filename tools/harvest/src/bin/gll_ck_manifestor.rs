use omegaflow::archivar::{curl_base, fetch_raw};
use omegaflow::cdn::upload_release;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Read;
use std::process::{Command, Stdio};

const NAIF_NETLOC: &str = "naif.jpl.nasa.gov";
const OUT_ROOT: &str = "data";
const FETCH_TTL_S: u64 = 86400;
const RTR_INDEX: &str =
    "https://naif.jpl.nasa.gov/pub/naif/GLL/kernels/ck/prime_mission/unvalidated/rtr/";
const SCLK_URL: &str = "https://naif.jpl.nasa.gov/pub/naif/GLL/kernels/sclk/mk00062a.tsc";
const SCLK_NAME: &str = "mk00062a.tsc";
const SCLK_MARKER: &str = "SCLK01_COEFFICIENTS_77";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn index_names(html: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut rest: &str = html;
    while let Some(h) = rest.find("href=\"") {
        rest = &rest[h + 6..];
        let Some(e) = rest.find('"') else { break };
        let name = &rest[..e];
        if name.ends_with("_rtr.bc") {
            names.push(name.to_string());
        }
        rest = &rest[e + 1..];
    }
    names.sort();
    names.dedup();
    names
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

fn ck_holds(path: &str) -> bool {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };
    if meta.len() < 96 {
        return false;
    }
    let mut head = [0u8; 96];
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    if file.read_exact(&mut head).is_err() {
        return false;
    }
    &head[88..96] == b"BIG-IEEE"
}

fn tsc_holds(path: &str) -> bool {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return false,
    };
    if bytes.is_empty() {
        return false;
    }
    String::from_utf8_lossy(&bytes).contains(SCLK_MARKER)
}

fn cdn_asset_names() -> HashSet<String> {
    let out = Command::new("gh")
        .arg("release")
        .arg("view")
        .arg(NAIF_NETLOC)
        .arg("--repo")
        .arg("omegaflow/sources")
        .arg("--json")
        .arg("assets")
        .arg("--jq")
        .arg(".assets[].name")
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        _ => HashSet::new(),
    }
}

fn store_asset(url: &str, root: &str, name: &str) -> Option<String> {
    let path = format!("{root}/{NAIF_NETLOC}/{name}");
    let part = format!("{path}.part");
    if !curl_file(url, &part) {
        return None;
    }
    let holds = if name.ends_with(".bc") {
        ck_holds(&part)
    } else {
        tsc_holds(&part)
    };
    if !holds {
        let _ = fs::remove_file(&part);
        return None;
    }
    if fs::rename(&part, &path).is_err() {
        let _ = fs::remove_file(&part);
        return None;
    }
    let len = fs::metadata(&path).ok().map(|m| m.len());
    let len_note = match len {
        Some(n) => n.to_string(),
        None => "unknown".to_string(),
    };
    eprintln!("manifest {name}: origin verbatim, {len_note} bytes");
    Some(path)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let root = match arg_value(&args, "--out") {
        Some(r) => r,
        None => OUT_ROOT.to_string(),
    };
    let Some(html) = fetch_raw(RTR_INDEX, None, &[], FETCH_TTL_S) else {
        eprintln!("rtr index fetch void: {RTR_INDEX}");
        std::process::exit(1);
    };
    let names = index_names(&html);
    if names.is_empty() {
        eprintln!("rtr index carries no _rtr.bc product — nothing manifestiert (0 honored)");
        std::process::exit(1);
    }
    eprintln!("rtr index: {} rotor CK products", names.len());
    let present = if ci_mode {
        cdn_asset_names()
    } else {
        HashSet::new()
    };
    let mut staged: Vec<String> = Vec::new();
    for name in &names {
        if present.contains(name) {
            eprintln!("{name}: already on the CDN — fetch skipped");
            continue;
        }
        let url = format!("{RTR_INDEX}{name}");
        if let Some(path) = store_asset(&url, &root, name) {
            staged.push(path);
        }
    }
    if present.contains(SCLK_NAME) {
        eprintln!("{SCLK_NAME}: already on the CDN — fetch skipped");
    } else if let Some(path) = store_asset(SCLK_URL, &root, SCLK_NAME) {
        staged.push(path);
    }
    if staged.is_empty() {
        eprintln!(
            "manifestor: all {} products already rest on the CDN",
            names.len()
        );
        return;
    }
    if !ci_mode {
        eprintln!(
            "manifestor: {} assets staged, no upload without --ci-mode",
            staged.len()
        );
        return;
    }
    for path in &staged {
        eprintln!("upload {path} -> cdn tag {NAIF_NETLOC}");
        if !upload_release(NAIF_NETLOC, path) {
            eprintln!("upload {path} returned void");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_names_collects_only_rotor_ck() {
        let html = "<a href=\"ck90341a_rtr.bc\">x</a> \
                    <a href=\"ck89361a_rtr.xc\">x</a> \
                    <a href=\"?C=N;O=D\">x</a>";
        assert_eq!(index_names(html), vec!["ck90341a_rtr.bc".to_string()]);
    }

    #[test]
    fn ck_holds_requires_daf_big_ieee() {
        let dir = std::env::temp_dir();
        let path = dir.join("gll_ck_manifestor_ok.bc");
        let mut bytes = vec![0u8; 96];
        bytes[88..96].copy_from_slice(b"BIG-IEEE");
        std::fs::write(&path, &bytes).unwrap();
        assert!(ck_holds(path.to_str().unwrap()));

        let bad = dir.join("gll_ck_manifestor_bad.bc");
        std::fs::write(&bad, b"not a daf").unwrap();
        assert!(!ck_holds(bad.to_str().unwrap()));

        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&bad);
    }

    #[test]
    fn tsc_holds_requires_sclk_marker() {
        let dir = std::env::temp_dir();
        let path = dir.join("gll_ck_manifestor_ok.tsc");
        std::fs::write(&path, "KERNEL SCLK01_COEFFICIENTS_77 = (\n").unwrap();
        assert!(tsc_holds(path.to_str().unwrap()));

        let bad = dir.join("gll_ck_manifestor_bad.tsc");
        std::fs::write(&bad, "no marker here").unwrap();
        assert!(!tsc_holds(bad.to_str().unwrap()));

        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&bad);
    }
}
