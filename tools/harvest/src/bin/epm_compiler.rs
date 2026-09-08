use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

use omegaflow::archivar::motion::parse_ephemeris_binary;
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::upload_release;
use omegaflow::ephemeris::{extract_granules, write_binary, GRANULE_DAYS};
use omegaflow::fk::FkFile;
use omegaflow::pck::{self, PckBody};

const CDN_TAG: &str = "ftp.iaaras.ru";
const EPM2021_BSP: &str = "https://ftp.iaaras.ru/pub/epm/EPM2021/SPICE/epm2021.bsp";

fn fetch_bsp(url: &str) -> Option<PathBuf> {
    let name = url.rsplit('/').next().unwrap_or("epm2021.bsp");
    let path = PathBuf::from(name);
    let status = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("5400")
        .arg("-o")
        .arg(&path)
        .arg(url)
        .status();
    match status {
        Ok(s) if s.success() => {
            let landed = match std::fs::metadata(&path) {
                Ok(m) => m.len(),
                Err(_) => 0,
            };
            if landed > 0 {
                Some(path)
            } else {
                let _ = std::fs::remove_file(&path);
                eprintln!("epm: {} landed 0 bytes — the download stays rejected", url);
                None
            }
        }
        _ => {
            let _ = std::fs::remove_file(&path);
            eprintln!("epm: download of {} returned void", url);
            None
        }
    }
}

fn resolve_inputs(paths: &[String]) -> Vec<PathBuf> {
    let mut bsps = Vec::new();
    for p in paths {
        if p.contains("://") {
            match fetch_bsp(p) {
                Some(b) => bsps.push(b),
                None => eprintln!("epm: no position kernel resolved from {}", p),
            }
        } else {
            bsps.push(PathBuf::from(p));
        }
    }
    bsps.sort();
    bsps
}

fn resolved_bodies(bsps: &[PathBuf]) -> BTreeMap<i32, String> {
    let table = omegaflow::ephemeris::body_table();
    let mut by_id: BTreeMap<i32, String> = BTreeMap::new();
    for p in bsps {
        match SpkFile::open(p) {
            Ok(spk) => {
                for seg in spk.segments() {
                    if let Some(b) = table.get(&seg.target) {
                        by_id.entry(seg.target).or_insert_with(|| b.name.clone());
                    }
                }
            }
            Err(e) => eprintln!("epm: open {}: {}", p.display(), e),
        }
    }
    by_id
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: epm_compiler [<epm2021.bsp|url>]... [--ci-mode]");
        eprintln!("  downloads https://ftp.iaaras.ru/pub/epm/EPM2021/SPICE/epm2021.bsp when no path is given");
        eprintln!("  emits ephemeris_epm_<body>.bin in the current directory");
        eprintln!(
            "  --ci-mode uploads each asset to the {} CDN release",
            CDN_TAG
        );
        std::process::exit(1);
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let inputs: Vec<String> = args.iter().filter(|a| *a != "--ci-mode").cloned().collect();
    let mut bsps = resolve_inputs(&inputs);
    if bsps.is_empty() {
        eprintln!(
            "epm: no position kernel resolved from the given inputs — fetching {}",
            EPM2021_BSP
        );
        match fetch_bsp(EPM2021_BSP) {
            Some(b) => bsps.push(b),
            None => {
                eprintln!("epm: the EPM2021 bsp download returned void — the build stays closed");
                std::process::exit(1);
            }
        }
    }
    let mut spk_files = Vec::new();
    for p in &bsps {
        match SpkFile::open(p) {
            Ok(s) => spk_files.push(s),
            Err(e) => {
                eprintln!("epm: open {}: {}", p.display(), e);
                std::process::exit(1);
            }
        }
    }
    eprintln!("epm: {} position kernel(s) opened", spk_files.len());
    let table = omegaflow::ephemeris::body_table();
    for spk in &spk_files {
        for seg in spk.segments() {
            let name = table
                .get(&seg.target)
                .map(|b| b.name.as_str())
                .unwrap_or("?");
            eprintln!(
                "  segment target {} ({}) center {} frame {} type {} et [{:.3}, {:.3}] name {:?}",
                seg.target,
                name,
                seg.center,
                seg.frame,
                seg.data_type,
                seg.start_et,
                seg.end_et,
                seg.name
            );
        }
    }
    let pck_bodies: std::collections::HashMap<i32, PckBody> = pck::parse(None, None);
    let woven = omegaflow::weberin::EPM_LINE_BODIES;
    let mut written = 0usize;
    let mut uploaded = 0usize;
    for (target, name) in resolved_bodies(&bsps) {
        if !woven.contains(&name.as_str()) {
            eprintln!(
                "epm: {} (target {}) covered but not compiled — the weave scope is the planet/moon set",
                name, target
            );
            continue;
        }
        let wgccre = match pck_bodies.get(&target) {
            Some(w) => w.clone(),
            None => PckBody::minimal(target),
        };
        let fk = FkFile::parse("");
        let mut granules = Vec::new();
        let mut rotations = Vec::new();
        let mut nutation = Vec::new();
        for spk in &spk_files {
            let has_coverage = spk.segments().iter().any(|s| s.target == target);
            if !has_coverage {
                continue;
            }
            let (g, r, n) =
                extract_granules(spk, &spk_files, target, &wgccre, &[], &fk, GRANULE_DAYS);
            granules.extend(g);
            rotations.extend(r);
            nutation.extend(n);
        }
        granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        rotations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        nutation.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        if granules.is_empty() {
            eprintln!(
                "epm: {} (target {}) carries no type-2/type-20 granule — skipped",
                name, target
            );
            continue;
        }
        let path = format!("ephemeris_epm_{}.bin", name);
        if !write_binary(
            &path, &name, &granules, &rotations, &nutation, &wgccre, None,
        ) {
            eprintln!("epm: write {} returned void", path);
            continue;
        }
        match std::fs::read(&path) {
            Ok(b) if parse_ephemeris_binary(&b).is_some() => {}
            Ok(_) => {
                eprintln!(
                    "epm: {} does not parse back to a BodyEphemeris — the file stays rejected",
                    path
                );
                continue;
            }
            Err(e) => {
                eprintln!("epm: read {}: {}", path, e);
                continue;
            }
        }
        written += 1;
        if ci_mode && upload_release(CDN_TAG, &path) {
            uploaded += 1;
        }
    }
    eprintln!(
        "epm: {} body line(s) compiled into ephemeris_epm_<body>.bin, {} uploaded to the {} release",
        written, uploaded, CDN_TAG
    );
    if ci_mode && uploaded != written {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_bodies_stays_empty_when_no_kernel_opens() {
        let paths = vec![PathBuf::from("/nonexistent/epm2021.bsp")];
        let map = resolved_bodies(&paths);
        assert!(map.is_empty());
    }

    #[test]
    fn resolve_inputs_keeps_local_kernel_paths_without_a_fetch() {
        let paths = vec![
            "/data/epm2021.bsp".to_string(),
            "/data/epm2021.bsp".to_string(),
        ];
        let bsps = resolve_inputs(&paths);
        assert_eq!(bsps.len(), 2);
        assert_eq!(bsps[0], PathBuf::from("/data/epm2021.bsp"));
    }
}
