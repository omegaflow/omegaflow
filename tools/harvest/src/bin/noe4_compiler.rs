use std::collections::BTreeMap;
use std::process::Command;

use omegaflow::archivar::motion::parse_ephemeris_binary;
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::upload_release;
use omegaflow::ephemeris::{extract_granules, pck_id_of, write_binary, GRANULE_DAYS};
use omegaflow::fk::FkFile;
use omegaflow::pck::{self, PckBody};

const CDN_TAG: &str = "ftp.imcce.fr";
const NOE4_TPC_URL: &str = "https://ftp.imcce.fr/pub/ephem/satel/NOE/MARS/2020/NOE-4-2020.tpc";
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

fn noe4_gm_text(gm_local: Option<&str>) -> Option<String> {
    match gm_local {
        Some(p) => match std::fs::read_to_string(p) {
            Ok(t) => Some(t),
            Err(e) => {
                eprintln!("noe4: gm read {}: {}", p, e);
                None
            }
        },
        None => match fetch_text(NOE4_TPC_URL) {
            Some(t) => Some(t),
            None => {
                eprintln!("noe4: gm fetch of {} returned void", NOE4_TPC_URL);
                None
            }
        },
    }
}

fn body_pck_text() -> Option<String> {
    let mut text = String::new();
    for url in [IAU_PCK_10, IAU_PCK_11] {
        match fetch_text(url) {
            Some(t) => {
                text.push_str(&t);
                text.push('\n');
            }
            None => eprintln!("noe4: pck fetch of {} returned void", url),
        }
    }
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn resolved_bodies(spk: &SpkFile) -> BTreeMap<i32, String> {
    let table = omegaflow::ephemeris::body_table();
    let mut by_id: BTreeMap<i32, String> = BTreeMap::new();
    for seg in spk.segments() {
        if matches!(seg.target, 401 | 402) {
            if let Some(b) = table.get(&seg.target) {
                by_id.entry(seg.target).or_insert_with(|| b.name.clone());
            }
        }
    }
    by_id
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: noe4_compiler <NOE-4-2020.bsp> [--gm <NOE-4-2020.tpc>] [--chain <planetary.bsp>]... [--ci-mode]"
        );
        eprintln!(
            "  emits ephemeris_noe4_phobos.bin and ephemeris_noe4_deimos.bin in the current directory"
        );
        eprintln!("  --gm passes the NOE-4 PCK text (GM source); absent, it is fetched from IMCCE");
        eprintln!(
            "  --chain adds a planetary kernel carrying the Mars-barycenter-to-SSB state; NOE-4 carries the satellites only"
        );
        eprintln!(
            "  --ci-mode uploads each asset to the {} CDN release",
            CDN_TAG
        );
        std::process::exit(1);
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut gm_local: Option<String> = None;
    let mut chain_paths: Vec<String> = Vec::new();
    let mut noe4_path: Option<String> = None;
    let mut skip_next = false;
    for (i, a) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match a.as_str() {
            "--gm" => {
                if let Some(f) = args.get(i + 1) {
                    gm_local = Some(f.clone());
                    skip_next = true;
                }
            }
            "--chain" => {
                if let Some(f) = args.get(i + 1) {
                    chain_paths.push(f.clone());
                    skip_next = true;
                }
            }
            "--ci-mode" => {}
            other => {
                if noe4_path.is_none() {
                    noe4_path = Some(other.to_string());
                }
            }
        }
    }
    let noe4_path = match noe4_path {
        Some(p) => p,
        None => {
            eprintln!("noe4: no NOE-4 position kernel given");
            std::process::exit(1);
        }
    };
    let noe4_spk = match SpkFile::open(&noe4_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("noe4: open {}: {}", noe4_path, e);
            std::process::exit(1);
        }
    };
    eprintln!("noe4: {} opened", noe4_path);
    for seg in noe4_spk.segments() {
        eprintln!(
            "  segment target {} center {} frame {} type {} et [{:.3}, {:.3}] name {:?}",
            seg.target, seg.center, seg.frame, seg.data_type, seg.start_et, seg.end_et, seg.name
        );
    }
    let mut all_kernels = vec![noe4_spk];
    for cp in &chain_paths {
        match SpkFile::open(cp) {
            Ok(s) => all_kernels.push(s),
            Err(e) => {
                eprintln!("noe4: chain open {}: {}", cp, e);
                std::process::exit(1);
            }
        }
    }
    let pck_bodies: std::collections::HashMap<i32, PckBody> = pck::parse(
        noe4_gm_text(gm_local.as_deref()).as_deref(),
        body_pck_text().as_deref(),
    );
    let targets = resolved_bodies(&all_kernels[0]);
    if targets.is_empty() {
        eprintln!(
            "noe4: no Phobos (401) or Deimos (402) segment in the kernel — the run stays closed"
        );
        std::process::exit(1);
    }
    let mut written = 0usize;
    let mut uploaded = 0usize;
    for (target, name) in targets {
        let wgccre = match pck_bodies.get(&pck_id_of(target)) {
            Some(w) => w.clone(),
            None => PckBody::minimal(target),
        };
        let fk = FkFile::parse("");
        let noe4_segments_have = |t: i32| all_kernels[0].segments().iter().any(|s| s.target == t);
        if !noe4_segments_have(target) {
            continue;
        }
        let (g, r, n) = extract_granules(
            &all_kernels[0],
            &all_kernels,
            target,
            &wgccre,
            &[],
            &fk,
            GRANULE_DAYS,
        );
        let mut granules = g;
        let mut rotations = r;
        let mut nutation = n;
        granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        rotations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        nutation.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        if granules.is_empty() {
            eprintln!(
                "noe4: {} (target {}) carries no granule — no SSB chain (--chain planetary kernel absent) — skipped",
                name, target
            );
            continue;
        }
        let path = format!("ephemeris_noe4_{}.bin", name);
        if !write_binary(
            &path, &name, &granules, &rotations, &nutation, &wgccre, None,
        ) {
            eprintln!("noe4: write {} returned void", path);
            continue;
        }
        match std::fs::read(&path) {
            Ok(b) if parse_ephemeris_binary(&b).is_some() => {}
            Ok(_) => {
                eprintln!(
                    "noe4: {} does not parse back to a BodyEphemeris — the file stays rejected",
                    path
                );
                continue;
            }
            Err(e) => {
                eprintln!("noe4: read {}: {}", path, e);
                continue;
            }
        }
        eprintln!(
            "noe4: {} (target {}) — {} granules",
            name,
            target,
            granules.len()
        );
        written += 1;
        if ci_mode && upload_release(CDN_TAG, &path) {
            uploaded += 1;
        }
    }
    eprintln!(
        "noe4: {} body line(s) compiled into ephemeris_noe4_<body>.bin, {} uploaded to the {} release",
        written, uploaded, CDN_TAG
    );
    if written == 0 {
        eprintln!(
            "noe4: no body line compiled — a kernel with supported bodies was opened; the run stays closed"
        );
        std::process::exit(1);
    }
    if ci_mode && uploaded != written {
        std::process::exit(1);
    }
}
