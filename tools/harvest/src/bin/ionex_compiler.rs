use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::ionex::{parse_gim, parse_gim_bin, write_gim_bin};
use std::process::Command;

const NETLOC: &str = "cddis.nasa.gov";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSLf")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("240");
    if let Some(token) = secret("EARTHDATA_EDL_TOKEN") {
        cmd.arg("-H").arg(format!("Authorization: Bearer {token}"));
    }
    let out = cmd.arg(url).output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn is_gzip(bytes: &[u8]) -> bool {
    bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b
}

fn raw_of_source(url: &str) -> Option<Vec<u8>> {
    if secret("EARTHDATA_EDL_TOKEN").is_none() {
        eprintln!(
            "EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token (0 honored)"
        );
        return None;
    }
    fetch(url)
}

fn run(out_path: &str, raw: Vec<u8>, label: &str, ci: bool) -> Result<(), String> {
    let bytes = if is_gzip(&raw) {
        match gunzip(&raw) {
            Some(b) => b,
            None => return Err(format!("{label}: gunzip returned void")),
        }
    } else {
        raw
    };
    let text = String::from_utf8_lossy(&bytes);
    let grids = parse_gim(&text, -1.0);
    if grids.is_empty() {
        return Err(format!(
            "{label}: carries no TEC map — the bin stays unwritten (0 honored)"
        ));
    }
    let cells: usize = grids.iter().map(|g| g.cells.len()).sum();
    let mut epochs: Vec<f64> = grids.iter().map(|g| g.epoch_unix).collect();
    epochs.sort_by(f64::total_cmp);
    eprintln!(
        "{label}: {} TEC maps, {} cells — epochs {}..{}",
        grids.len(),
        cells,
        epochs[0],
        epochs[epochs.len() - 1]
    );
    let bin = write_gim_bin(&grids);
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(out_path, &bin).map_err(|e| format!("{out_path}: {e}"))?;
    match parse_gim_bin(&bin) {
        Some(parsed) if parsed.len() == grids.len() => {
            eprintln!(
                "{out_path}: {} TEC maps (tecu), {} B, roundtrip parses",
                parsed.len(),
                bin.len()
            );
        }
        Some(parsed) => {
            return Err(format!(
                "{out_path}: {} maps roundtrip, {} written — the bin stays unverified",
                parsed.len(),
                grids.len()
            ));
        }
        None => {
            return Err(format!(
                "{out_path}: roundtrip parse void — the bin stays unverified"
            ));
        }
    }
    if ci && !upload_release(NETLOC, out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(out_path) = arg_value(&args, "--out") else {
        eprintln!("ionex_compiler: --out <asset.bin> absent — the output path is never silent");
        std::process::exit(1);
    };
    let ci = args.iter().any(|a| a == "--ci-mode");
    let (raw, label) = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => (b, path),
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => match arg_value(&args, "--url") {
            Some(url) => match raw_of_source(&url) {
                Some(b) => (b, url),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!(
                    "ionex_compiler: --input <file.INX.gz> or --url <https://…/IGS0OPSRAP_….INX.gz> required — refused"
                );
                std::process::exit(1);
            }
        },
    };
    if let Err(msg) = run(&out_path, raw, &label, ci) {
        eprintln!("ionex_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_gzip_reads_the_magic() {
        assert!(is_gzip(&[0x1f, 0x8b, 0x08, 0x00]));
        assert!(!is_gzip(b"not gzip"));
        assert!(!is_gzip(&[]));
        assert!(!is_gzip(&[0x1f]));
    }
}
