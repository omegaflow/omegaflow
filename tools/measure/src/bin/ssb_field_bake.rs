use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use omegaflow::archivar::{
    MembraneCtx, SampleRecord, build_asteroid_samples, build_spatial_hash, build_star_samples,
    embedded_lsk, enclosure_presences, parse_sources, query_hash,
};
use omegaflow::mathematikerin::PresenceState;
use omegaflow::sha256::sha256_hex;

const DEFAULT_DASTCOM_TTL: u64 = 86400;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name).and_then(|w| w.parse::<f64>().ok())
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn flatten(records: &[SampleRecord]) -> Vec<u8> {
    let mut out = Vec::with_capacity(records.len() * 26 * 8);
    for r in records {
        for v in [
            r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9, r.10, r.11, r.12, r.13, r.14, r.15,
            r.16, r.17, r.18, r.19, r.20, r.21, r.22, r.23, r.24, r.25,
        ] {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn usage() {
    println!(
        "usage: ssb_field_bake [--sources <file>] [--stars <bin>] [--dastcom <bin>] [--now <tdb>] [--out <file>] [--ci-mode] [--help]"
    );
    println!(
        "  the enclosure query at the resting presence (the SSB origin) over the measured hull — the records are baked as the field asset (26*f64 little-endian per record)"
    );
    println!(
        "  defaults: sources phi/sources.φ, stars data/ssd.jpl.nasa.gov/dr3_stars.bin, dastcom data/ssd.jpl.nasa.gov/dastcom_asteroids.bin, out out/ssb_field.bin, now = embedded-LSK system TDB"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if flag(&args, "--help") || flag(&args, "-h") {
        usage();
        return;
    }
    let sources_path = match arg_value(&args, "--sources") {
        Some(p) => p,
        None => "phi/sources.φ".to_string(),
    };
    let stars_path = match arg_value(&args, "--stars") {
        Some(p) => p,
        None => "data/ssd.jpl.nasa.gov/dr3_stars.bin".to_string(),
    };
    let dastcom_path = match arg_value(&args, "--dastcom") {
        Some(p) => p,
        None => "data/ssd.jpl.nasa.gov/dastcom_asteroids.bin".to_string(),
    };
    let out_path = match arg_value(&args, "--out") {
        Some(p) => p,
        None => "out/ssb_field.bin".to_string(),
    };
    let now = match arg_f64(&args, "--now") {
        Some(t) if t.is_finite() => t,
        Some(_) => {
            eprintln!("ssb_field_bake: --now carries no finite value");
            std::process::exit(2);
        }
        None => match embedded_lsk().and_then(|l| l.system_now_tdb()) {
            Some(t) => t,
            None => {
                eprintln!(
                    "ssb_field_bake: the embedded LSK yields no TDB now — pass --now explicitly"
                );
                std::process::exit(2);
            }
        },
    };

    let sources = match std::fs::read_to_string(&sources_path) {
        Ok(c) => parse_sources(&c),
        Err(_) => {
            eprintln!(
                "ssb_field_bake: {} read void — the frame stays unnamed",
                sources_path
            );
            Vec::new()
        }
    };

    let slot = Arc::new(RwLock::new(PresenceState::rest()));
    let presences = enclosure_presences(&HashMap::new(), &slot, now);
    let center = match presences.first() {
        Some(&(_, cx, cy, cz, ..)) => [cx, cy, cz],
        None => {
            eprintln!("ssb_field_bake: the resting presence slot yields no hull — no measurement");
            std::process::exit(2);
        }
    };

    let star_bytes = match std::fs::read(&stars_path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!(
                "ssb_field_bake: {} read void — the field stays absent, not zero",
                stars_path
            );
            std::process::exit(2);
        }
    };
    let star_epoch = sources
        .iter()
        .find(|s| s.format == "catalog_tycho")
        .and_then(|s| s.catalog_epoch);
    let mut star_samples = build_star_samples(&star_bytes, star_epoch);
    if star_samples.is_empty() {
        eprintln!(
            "ssb_field_bake: {} yields no star samples — the measurement is absent, not zero",
            stars_path
        );
        std::process::exit(2);
    }
    if flag(&args, "--include-asteroids") {
        let dastcom_ttl = match sources.iter().find(|s| s.format == "catalog_dastcom") {
            Some(s) => s.ttl,
            None => DEFAULT_DASTCOM_TTL,
        };
        if let Ok(bytes) = std::fs::read(&dastcom_path) {
            star_samples.extend(build_asteroid_samples(&bytes, dastcom_ttl));
        }
    }

    let floor = [1e-40f64; 9];
    let hash = build_spatial_hash(star_samples.into_iter().map(Arc::new).collect(), 1.0);
    let mut records: Vec<SampleRecord> = Vec::new();
    query_hash(
        &hash,
        MembraneCtx {
            center,
            t2: now,
            pad: 1.0,
            delta_t_cache: 0.0,
            floor: &floor,
            softening: 1.0,
            forward: [1.0, 0.0, 0.0],
            eph: &HashMap::new(),
        },
        &mut records,
    );

    let bytes = flatten(&records);
    let sha = sha256_hex(&bytes);
    if flag(&args, "--ci-mode") || arg_value(&args, "--out").is_some() {
        if let Some(dir) = std::path::Path::new(&out_path).parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        match std::fs::write(&out_path, &bytes) {
            Ok(()) => println!("WROTE {out_path} bytes {} sha256 {sha}", bytes.len()),
            Err(e) => {
                eprintln!("ssb_field_bake: {out_path} write failed: {e}");
                std::process::exit(2);
            }
        }
    }
    println!(
        "SSB_FIELD | center ({:.6e}, {:.6e}, {:.6e}) | records {} | values {} | t2 {now:.17e} | sha256 {sha}",
        center[0],
        center[1],
        center[2],
        records.len(),
        records.len() * 26
    );
}
