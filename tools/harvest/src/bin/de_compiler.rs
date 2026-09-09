use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

use omegaflow::archivar::motion::parse_ephemeris_binary;
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::upload_release;
use omegaflow::ephemeris::{extract_granules, pck_id_of, write_binary, GRANULE_DAYS};
use omegaflow::fk::FkFile;
use omegaflow::pck::{self, PckBody};

const IAU_PCK_10: &str = "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/pck00010.tpc";
const IAU_PCK_11: &str = "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/pck00011.tpc";
const DE_BODIES: [&str; 3] = ["sun", "moon", "earth"];
const DEFAULT_NETLOC: &str = "ssd.jpl.nasa.gov";

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
                None => eprintln!("de: pck fetch of {} returned void", url),
            }
        }
    } else {
        for p in local {
            match std::fs::read_to_string(p) {
                Ok(t) => {
                    text.push_str(&t);
                    text.push('\n');
                }
                Err(e) => eprintln!("de: pck read {}: {}", p, e),
            }
        }
    }
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
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
            Err(e) => eprintln!("de: open {}: {}", p.display(), e),
        }
    }
    by_id
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: de_compiler <de.bsp>... --label <edition> [--netloc <netloc>] [--pck <body.tpc>]... [--ci-mode]");
        eprintln!("  emits ephemeris_<edition>_sun.bin, ephemeris_<edition>_moon.bin, ephemeris_<edition>_earth.bin in the current directory");
        eprintln!("  --label is the JPL DE edition word (the data lineage), e.g. de440");
        eprintln!("  --netloc is the CDN release tag, default ssd.jpl.nasa.gov");
        eprintln!("  --pck passes a NAIF body PCK text; absent, pck00010+pck00011 are fetched");
        eprintln!("  --ci-mode uploads each asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let label = match arg_value(&args, "--label") {
        Some(l) if !l.is_empty() => l,
        _ => {
            eprintln!("de: --label <edition> is required — the DE edition word names the lineage");
            std::process::exit(1);
        }
    };
    let netloc = match arg_value(&args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => DEFAULT_NETLOC.to_string(),
    };
    let mut pck_local: Vec<String> = Vec::new();
    let mut rest: Vec<String> = Vec::new();
    let mut skip_next = false;
    for (i, a) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if a == "--label" || a == "--netloc" {
            skip_next = true;
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
    let bsps: Vec<PathBuf> = rest.into_iter().map(PathBuf::from).collect();
    if bsps.is_empty() {
        eprintln!("de: no position kernel resolved from the given inputs");
        std::process::exit(1);
    }
    let mut spk_files = Vec::new();
    for p in &bsps {
        match SpkFile::open(p) {
            Ok(s) => spk_files.push(s),
            Err(e) => {
                eprintln!("de: open {}: {}", p.display(), e);
                std::process::exit(1);
            }
        }
    }
    eprintln!(
        "de {}: {} position kernel(s) opened",
        label,
        spk_files.len()
    );
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
    let pck_bodies: std::collections::HashMap<i32, PckBody> =
        pck::parse(None, body_pck_text(&pck_local).as_deref());
    let in_scope = |name: &str| DE_BODIES.contains(&name);
    let mut written = 0usize;
    let mut uploaded = 0usize;
    for (target, name) in resolved_bodies(&bsps) {
        if !in_scope(&name) {
            eprintln!(
                "de: {} (target {}) covered but not compiled — the scope is the sun/moon/earth line",
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
                "de: {} (target {}) carries no type-2/type-20 granule — skipped",
                name, target
            );
            continue;
        }
        let path = format!("ephemeris_{}_{}.bin", label, name);
        if !write_binary(
            &path, &name, &granules, &rotations, &nutation, &wgccre, None,
        ) {
            eprintln!("de: write {} returned void", path);
            continue;
        }
        match std::fs::read(&path) {
            Ok(b) if parse_ephemeris_binary(&b).is_some() => {}
            Ok(_) => {
                eprintln!(
                    "de: {} does not parse back to a BodyEphemeris — the file stays rejected",
                    path
                );
                continue;
            }
            Err(e) => {
                eprintln!("de: read {}: {}", path, e);
                continue;
            }
        }
        written += 1;
        if ci_mode && upload_release(&netloc, &path) {
            uploaded += 1;
        }
    }
    eprintln!(
        "de {}: {} body line(s) compiled into ephemeris_<edition>_<body>.bin, {} uploaded to the {} release",
        label, written, uploaded, netloc
    );
    if written == 0 {
        eprintln!(
            "de: no body line compiled — a kernel with supported bodies was opened; the run stays closed"
        );
        std::process::exit(1);
    }
    if ci_mode && uploaded != written {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_bodies_stays_empty_when_no_kernel_opens() {
        let paths = vec![PathBuf::from("/nonexistent/de440.bsp")];
        let map = resolved_bodies(&paths);
        assert!(map.is_empty());
    }
}
