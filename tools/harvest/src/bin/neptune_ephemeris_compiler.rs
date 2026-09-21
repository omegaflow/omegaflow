use std::process::Command;

use omegaflow::archivar::motion::parse_ephemeris_binary;
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::upload_release;
use omegaflow::ephemeris::{GRANULE_DAYS, extract_granules, pck_id_of, write_binary};
use omegaflow::fk::FkFile;
use omegaflow::pck::{self, PckBody};

const IAU_PCK_10: &str = "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/pck00010.tpc";
const IAU_PCK_11: &str = "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/pck00011.tpc";
const NETLOC: &str = "ssd.jpl.nasa.gov-neptune";
const LABEL: &str = "de440";
const NEPTUNE_BARYCENTER: i32 = 8;
const BODY_NAME: &str = "neptune";

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
                None => eprintln!("neptune: pck fetch of {} returned void", url),
            }
        }
    } else {
        for p in local {
            match std::fs::read_to_string(p) {
                Ok(t) => {
                    text.push_str(&t);
                    text.push('\n');
                }
                Err(e) => eprintln!("neptune: pck read {}: {}", p, e),
            }
        }
    }
    if text.is_empty() { None } else { Some(text) }
}

fn has_barycenter(spk: &SpkFile) -> bool {
    spk.segments()
        .iter()
        .any(|s| s.target == NEPTUNE_BARYCENTER)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: neptune_ephemeris_compiler <de440s.bsp> [--pck <body.tpc>]... [--ci-mode]"
        );
        eprintln!(
            "  emits data/{NETLOC}/ephemeris_{LABEL}_{BODY_NAME}.bin (the DE440 Neptune barycenter line)"
        );
        eprintln!("  --pck passes a NAIF body PCK text; absent, pck00010+pck00011 are fetched");
        eprintln!("  --ci-mode uploads the asset to the {NETLOC} CDN release");
        std::process::exit(1);
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut pck_local: Vec<String> = Vec::new();
    let mut bsp_path: Option<String> = None;
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
        if a != "--ci-mode" && bsp_path.is_none() {
            bsp_path = Some(a.clone());
        }
    }
    let bsp_path = match bsp_path {
        Some(p) => p,
        None => {
            eprintln!("neptune: no position kernel given");
            std::process::exit(1);
        }
    };
    let spk = match SpkFile::open(&bsp_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("neptune: open {}: {}", bsp_path, e);
            std::process::exit(1);
        }
    };
    eprintln!("neptune: {} opened", bsp_path);
    let table = omegaflow::ephemeris::body_table();
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
    if !has_barycenter(&spk) {
        eprintln!(
            "neptune: the kernel carries no target-{NEPTUNE_BARYCENTER} segment — the run stays closed"
        );
        std::process::exit(1);
    }
    let pck_bodies: std::collections::HashMap<i32, PckBody> =
        pck::parse(None, body_pck_text(&pck_local).as_deref());
    let wgccre = match pck_bodies.get(&pck_id_of(NEPTUNE_BARYCENTER)) {
        Some(w) => w.clone(),
        None => PckBody::minimal(NEPTUNE_BARYCENTER),
    };
    let fk = FkFile::parse("");
    let kernels = [spk];
    let (mut granules, mut rotations, mut nutation) = extract_granules(
        &kernels[0],
        &kernels,
        NEPTUNE_BARYCENTER,
        &wgccre,
        &[],
        &fk,
        GRANULE_DAYS,
    );
    granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    rotations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    nutation.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    if granules.is_empty() {
        eprintln!(
            "neptune: target {NEPTUNE_BARYCENTER} carries no granule — no SSB chain — the run stays closed"
        );
        std::process::exit(1);
    }
    let out_dir = format!("data/{NETLOC}");
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("neptune: create {}: {}", out_dir, e);
        std::process::exit(1);
    }
    let path = format!("{out_dir}/ephemeris_{LABEL}_{BODY_NAME}.bin");
    if !write_binary(
        &path, BODY_NAME, &granules, &rotations, &nutation, &wgccre, None,
    ) {
        eprintln!("neptune: write {} returned void", path);
        std::process::exit(1);
    }
    match std::fs::read(&path) {
        Ok(b) if parse_ephemeris_binary(&b).is_some() => {}
        Ok(_) => {
            eprintln!(
                "neptune: {} does not parse back to a BodyEphemeris — the file stays rejected",
                path
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("neptune: read {}: {}", path, e);
            std::process::exit(1);
        }
    }
    eprintln!(
        "neptune: {} (target {NEPTUNE_BARYCENTER}) — {} granules — {}",
        BODY_NAME,
        granules.len(),
        path
    );
    if ci_mode && !upload_release(NETLOC, &path) {
        eprintln!("neptune: upload {} returned void", path);
        std::process::exit(1);
    }
}
