use std::collections::HashMap;
use std::sync::Arc;

use omegaflow::archivar::{
    MembraneCtx, SampleRecord, build_spatial_hash, build_star_samples, embedded_lsk, query_hash,
};
use omegaflow::sha256::sha256_hex;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name).and_then(|w| w.parse::<f64>().ok())
}

fn flatten(records: &[SampleRecord]) -> Vec<f64> {
    let mut out = Vec::with_capacity(records.len() * 26);
    for r in records {
        out.extend_from_slice(&[
            r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9, r.10, r.11, r.12, r.13, r.14, r.15,
            r.16, r.17, r.18, r.19, r.20, r.21, r.22, r.23, r.24, r.25,
        ]);
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let stars_path = match arg_value(&args, "--stars") {
        Some(p) => p,
        None => "data/ssd.jpl.nasa.gov/dr3_stars.bin".to_string(),
    };
    let epoch = match arg_f64(&args, "--epoch") {
        Some(e) => e,
        None => 2000.0,
    };
    let now = match arg_f64(&args, "--now") {
        Some(t) => t,
        None => match embedded_lsk().and_then(|l| l.system_now_tdb()) {
            Some(t) => t,
            None => 8.443618e8,
        },
    };
    let bytes = std::fs::read(&stars_path).expect("stars read");
    let samples = build_star_samples(&bytes, Some(epoch));
    let hash = build_spatial_hash(samples.into_iter().map(Arc::new).collect(), 1.0);
    let floor = [1e-40f64; 9];
    let mut records: Vec<SampleRecord> = Vec::new();
    query_hash(
        &hash,
        MembraneCtx {
            center: [0.0, 0.0, 0.0],
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
    let flat = flatten(&records);
    let bytes_le: Vec<u8> = flat.iter().flat_map(|v| v.to_le_bytes()).collect();
    if let Some(dump) = arg_value(&args, "--dump") {
        std::fs::write(&dump, &bytes_le).expect("dump write");
    }
    println!(
        "NATIVE | records {} | values {} | t2 {now:.17e} | sha256 {}",
        records.len(),
        flat.len(),
        sha256_hex(&bytes_le)
    );
}
