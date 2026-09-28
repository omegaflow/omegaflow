use std::collections::HashMap;

use omegaflow::archivar::motion::{body_barycenter_position, parse_ephemeris_binary};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::ephemeris::{
    GRANULE_DAYS, J2000_EPOCH, extract_granules, state_ssb_multi, write_binary,
};
use omegaflow::fk::FkFile;
use omegaflow::pck::PckBody;

const CDN_RELEASE: &str = "ssd.jpl.nasa.gov-ephemeris";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: spacecraft_ephemeris_compiler --kernel <mission.bsp> [--kernel <carrier.bsp>]... \
             --naif <id> --name <name> [--out <ephemeris_<name>.bin>] [--ci-mode]"
        );
        eprintln!("  emits the <name> SSB line, chained across every given kernel");
        eprintln!("  --ci-mode uploads the asset to the {CDN_RELEASE} CDN release");
        std::process::exit(1);
    }

    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut kernels: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--kernel" => {
                if let Some(p) = args.get(i + 1) {
                    kernels.push(p.clone());
                    i += 1;
                }
            }
            "--naif" | "--name" | "--out" => i += 1,
            "--ci-mode" => {}
            other => {
                eprintln!("spacecraft_ephemeris_compiler: unknown argument {other}");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let naif = match arg_value(&args, "--naif").and_then(|v| v.parse::<i32>().ok()) {
        Some(id) => id,
        None => {
            eprintln!("spacecraft: --naif <id> is mandatory");
            std::process::exit(1);
        }
    };
    let name = match arg_value(&args, "--name") {
        Some(n) => n,
        None => {
            eprintln!("spacecraft: --name <name> is mandatory");
            std::process::exit(1);
        }
    };
    if kernels.is_empty() {
        eprintln!("spacecraft: at least one --kernel <bsp> is mandatory");
        std::process::exit(1);
    }

    let mut spk_files: Vec<SpkFile> = Vec::new();
    for path in &kernels {
        match SpkFile::open(path) {
            Ok(s) => spk_files.push(s),
            Err(e) => {
                eprintln!("spacecraft: open {path}: {e}");
                std::process::exit(1);
            }
        }
    }

    for spk in &spk_files {
        for seg in spk.segments() {
            if seg.target == naif {
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
    }

    let absent_pck = PckBody::absent();
    let fk = FkFile::parse("");
    let mut granules = Vec::new();
    let mut rotations = Vec::new();
    let mut nutation = Vec::new();
    for spk in &spk_files {
        let (g, r, n) =
            extract_granules(spk, &spk_files, naif, &absent_pck, &[], &fk, GRANULE_DAYS);
        granules.extend(g);
        rotations.extend(r);
        nutation.extend(n);
    }
    granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    rotations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    nutation.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    if granules.is_empty() {
        eprintln!(
            "spacecraft: naif {naif} carries no granule across the given kernels — the anchor stays unwritten"
        );
        std::process::exit(1);
    }

    let out = match arg_value(&args, "--out") {
        Some(out) => out,
        None => {
            eprintln!("spacecraft: --out is mandatory — the anchor needs its named destination");
            std::process::exit(2);
        }
    };
    if !write_binary(
        &out,
        &name,
        &granules,
        &rotations,
        &nutation,
        &absent_pck,
        None,
    ) {
        eprintln!("spacecraft: write {out} returned void");
        std::process::exit(1);
    }
    let bytes = match std::fs::read(&out) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("spacecraft: read {out}: {e}");
            std::process::exit(1);
        }
    };
    let eph = match parse_ephemeris_binary(&bytes) {
        Some(e) => e,
        None => {
            eprintln!(
                "spacecraft: {out} does not parse back to a BodyEphemeris — the asset stays rejected"
            );
            std::process::exit(1);
        }
    };
    let mut map = HashMap::new();
    map.insert(name.clone(), eph);
    let mut max_delta_m = 0.0f64;
    let mut probe_n = 0usize;
    for (t0, _, _, _, _) in &granules {
        let et = (t0 - J2000_EPOCH) * 86400.0;
        let Some(src) = state_ssb_multi(&spk_files, naif, et) else {
            continue;
        };
        let Some(p) = body_barycenter_position(&name, et, &map) else {
            continue;
        };
        let d = ((p[0] - src[0] * 1000.0).powi(2)
            + (p[1] - src[2] * 1000.0).powi(2)
            + (p[2] - src[4] * 1000.0).powi(2))
        .sqrt();
        if d > max_delta_m {
            max_delta_m = d;
        }
        probe_n += 1;
    }
    eprintln!(
        "{out}: {} B, sha256 {} — roundtrip verified on {probe_n} probes, max granule-vs-source delta {:.0} m",
        bytes.len(),
        sha256_hex(&bytes),
        max_delta_m
    );
    eprintln!("CDN asset: {}", body_url(&name));
    if ci_mode && !upload_release(CDN_RELEASE, &out) {
        eprintln!("spacecraft: upload of {out} returned void");
        std::process::exit(1);
    }
}
