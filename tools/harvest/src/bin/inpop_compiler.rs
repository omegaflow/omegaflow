use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

use omegaflow::archivar::motion::parse_ephemeris_binary;
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::upload_release;
use omegaflow::ephemeris::{extract_granules, pck_id_of, write_binary, GRANULE_DAYS};
use omegaflow::fk::FkFile;
use omegaflow::pck::{self, PckBody};

const CDN_TAG: &str = "ftp.imcce.fr";
const IAU_PCK_10: &str = "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/pck00010.tpc";
const IAU_PCK_11: &str = "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/pck00011.tpc";

fn fetch_text(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn body_pck_text(local: &[String]) -> Option<String> {
    let mut text = String::new();
    if local.is_empty() {
        for url in [IAU_PCK_10, IAU_PCK_11] {
            match fetch_text(url) {
                Some(t) => {
                    text.push_str(&t);
                    text.push('\n');
                }
                None => eprintln!("inpop: pck fetch of {} returned void", url),
            }
        }
    } else {
        for p in local {
            match std::fs::read_to_string(p) {
                Ok(t) => {
                    text.push_str(&t);
                    text.push('\n');
                }
                Err(e) => eprintln!("inpop: pck read {}: {}", p, e),
            }
        }
    }
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn resolve_inputs(paths: &[String]) -> (Vec<PathBuf>, Option<PathBuf>) {
    let mut bsps = Vec::new();
    let mut tpc = None;
    for p in paths {
        let path = PathBuf::from(p);
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if name.ends_with(".tar.gz") {
            let dir = PathBuf::from(format!("{}.d", path.display()));
            let _ = std::fs::create_dir_all(&dir);
            let status = Command::new("tar")
                .arg("-xzf")
                .arg(&path)
                .arg("-C")
                .arg(&dir)
                .status();
            match status {
                Ok(s) if s.success() => {}
                _ => {
                    eprintln!("inpop: tar extraction of {} returned void", path.display());
                    continue;
                }
            }
            let entries = match std::fs::read_dir(&dir) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("inpop: read {}: {}", dir.display(), e);
                    continue;
                }
            };
            for entry in entries.flatten() {
                let ep = entry.path();
                let n = match ep.file_name().and_then(|x| x.to_str()) {
                    Some(n) => n,
                    None => continue,
                };
                if n.ends_with(".bsp") && !n.contains("_time.bsp") {
                    bsps.push(ep);
                } else if n.ends_with(".tpc") && tpc.is_none() {
                    tpc = Some(ep);
                }
            }
        } else if name.ends_with(".bsp") {
            bsps.push(path);
        } else if name.ends_with(".tpc") {
            tpc = Some(path);
        }
    }
    bsps.sort();
    (bsps, tpc)
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
            Err(e) => eprintln!("inpop: open {}: {}", p.display(), e),
        }
    }
    by_id
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: inpop_compiler <bsp|spice.tar.gz>... [--pck <body.tpc>]... [--ci-mode]");
        eprintln!("  emits ephemeris_inpop_<body>.bin in the current directory");
        eprintln!("  --pck passes a NAIF body PCK text (POLE/RADII); absent, pck00010+pck00011 are fetched");
        eprintln!(
            "  --ci-mode uploads each asset to the {} CDN release",
            CDN_TAG
        );
        std::process::exit(1);
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut pck_local: Vec<String> = Vec::new();
    let mut rest: Vec<String> = Vec::new();
    let mut skip_next = false;
    for (i, a) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if a == "--pck" {
            if let Some(f) = args.get(i + 1) {
                pck_local.push(f.clone());
                skip_next = true;
            }
            continue;
        }
        if a != "--ci-mode" {
            rest.push(a.clone());
        }
    }
    let (bsps, tpc) = resolve_inputs(&rest);
    if bsps.is_empty() {
        eprintln!("inpop: no position kernel resolved from the given inputs");
        std::process::exit(1);
    }
    let mut spk_files = Vec::new();
    for p in &bsps {
        match SpkFile::open(p) {
            Ok(s) => spk_files.push(s),
            Err(e) => {
                eprintln!("inpop: open {}: {}", p.display(), e);
                std::process::exit(1);
            }
        }
    }
    eprintln!("inpop: {} position kernel(s) opened", spk_files.len());
    for spk in &spk_files {
        for seg in spk.segments() {
            eprintln!(
                "  segment target {} center {} frame {} type {} et [{:.3}, {:.3}] name {:?}",
                seg.target,
                seg.center,
                seg.frame,
                seg.data_type,
                seg.start_et,
                seg.end_et,
                seg.name
            );
        }
    }
    let pck_bodies: std::collections::HashMap<i32, PckBody> = pck::parse(
        tpc_text(&tpc).as_deref(),
        body_pck_text(&pck_local).as_deref(),
    );
    let woven = omegaflow::weberin::INPOP_LINE_BODIES;
    let in_scope = |name: &str| name == "sun" || woven.contains(&name);
    let mut written = 0usize;
    let mut uploaded = 0usize;
    for (target, name) in resolved_bodies(&bsps) {
        if !in_scope(&name) {
            eprintln!(
                "inpop: {} (target {}) covered but not compiled — the weave scope is the planet/moon set plus the sun",
                name, target
            );
            continue;
        }
        let wgccre = match pck_bodies.get(&pck_id_of(target)) {
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
                "inpop: {} (target {}) carries no type-2 granule — skipped",
                name, target
            );
            continue;
        }
        let path = format!("ephemeris_inpop_{}.bin", name);
        if !write_binary(
            &path, &name, &granules, &rotations, &nutation, &wgccre, None,
        ) {
            eprintln!("inpop: write {} returned void", path);
            continue;
        }
        match std::fs::read(&path) {
            Ok(b) if parse_ephemeris_binary(&b).is_some() => {}
            Ok(_) => {
                eprintln!(
                    "inpop: {} does not parse back to a BodyEphemeris — the file stays rejected",
                    path
                );
                continue;
            }
            Err(e) => {
                eprintln!("inpop: read {}: {}", path, e);
                continue;
            }
        }
        written += 1;
        if ci_mode && upload_release(CDN_TAG, &path) {
            uploaded += 1;
        }
    }
    eprintln!(
        "inpop: {} body line(s) compiled into ephemeris_inpop_<body>.bin, {} uploaded to the {} release",
        written, uploaded, CDN_TAG
    );
    if ci_mode && uploaded != written {
        std::process::exit(1);
    }
}

fn tpc_text(tpc: &Option<PathBuf>) -> Option<String> {
    let p = tpc.as_ref()?;
    match std::fs::read_to_string(p) {
        Ok(t) => Some(t),
        Err(e) => {
            eprintln!("inpop: read {}: {}", p.display(), e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_bodies_stays_empty_when_no_kernel_opens() {
        let paths = vec![PathBuf::from("/nonexistent/earth.bsp")];
        let map = resolved_bodies(&paths);
        assert!(map.is_empty());
    }
}
