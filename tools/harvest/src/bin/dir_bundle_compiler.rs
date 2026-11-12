use omegaflow::archivar::cdn::require_ci;
use omegaflow::cdn::upload_release;
use std::io::Write;
use std::process::Command;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_bytes(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .args(["-sSfL", "--max-time", "3600", url])
        .output()
        .map_err(|e| format!("curl {url}: {e}"))?;
    if !out.status.success() {
        return Err(format!("curl {url}: exit {}", out.status));
    }
    Ok(out.stdout)
}

fn curl_text(url: &str) -> Result<String, String> {
    let bytes = curl_bytes(url)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn links(listing: &str, needle: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in listing.split("href=\"").skip(1) {
        let Some(name) = part.split('"').next() else {
            continue;
        };
        if name.is_empty() || name.starts_with('?') || name.starts_with('/') || name.contains("..") {
            continue;
        }
        if !name.contains(needle) {
            continue;
        }
        out.push(name.to_string());
    }
    out.sort();
    out.dedup();
    out
}

fn run(args: &[String]) -> Result<(), String> {
    let base = arg_value(args, "--base").ok_or_else(|| "--base <listing url> absent".to_string())?;
    let needle = arg_value(args, "--name").ok_or_else(|| "--name <name fragment> absent".to_string())?;
    let out = arg_value(args, "--out").ok_or_else(|| "--out <asset> absent".to_string())?;
    let tag = arg_value(args, "--tag").ok_or_else(|| "--tag <netloc> absent".to_string())?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let listing = curl_text(&base)?;
    let names = links(&listing, &needle);
    if names.is_empty() {
        return Err(format!("{base}: no entry carries {needle}"));
    }
    let mut buf: Vec<u8> = Vec::new();
    for name in &names {
        let url = format!("{}{}", base, name);
        let bytes = curl_bytes(&url)?;
        if bytes.is_empty() {
            return Err(format!("{url}: empty"));
        }
        buf.extend_from_slice(&bytes);
    }
    let mut file = std::fs::File::create(&out).map_err(|e| format!("create {out}: {e}"))?;
    file.write_all(&buf).map_err(|e| format!("write {out}: {e}"))?;
    eprintln!("{base}: {} files, {} bytes -> {out}", names.len(), buf.len());
    if ci_mode && !upload_release(&tag, &out) {
        return Err(format!("{out}: upload returned void"));
    }
    Ok(())
}

fn main() {
    require_ci();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: dir_bundle_compiler --base <listing url> --name <fragment> --tag <netloc> --out <asset> [--ci-mode]");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("dir_bundle_compiler: {msg}");
        std::process::exit(1);
    }
}
