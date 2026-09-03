use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::suprastrom::{lambda_inv2_m2, parse_suprastrom_bin};

const SRD62_SUPRASTROM_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/srd62_suprastrom.bin";

fn main() {
    let bytes = match fetch_raw_bytes(SRD62_SUPRASTROM_CDN, 3600) {
        Some(b) => b,
        None => {
            eprintln!(
                "suprastrom_cuprate_probe: {} carries no asset (0 honored)",
                SRD62_SUPRASTROM_CDN
            );
            std::process::exit(1);
        }
    };
    let bin = match parse_suprastrom_bin(&bytes) {
        Some(b) => b,
        None => {
            eprintln!("suprastrom_cuprate_probe: the CDN suprastrom bin parses void");
            std::process::exit(1);
        }
    };

    let n_series = bin.series.len();
    let n_points: usize = bin.series.iter().map(|s| s.points.len()).sum();
    let n_sources: usize = {
        let mut seen = std::collections::BTreeSet::new();
        for s in &bin.series {
            seen.insert(s.id.clone());
        }
        seen.len()
    };
    let with_series = bin.series.iter().filter(|s| s.points.len() >= 2).count();

    println!("suprastrom_cuprate_probe — the Kuprat electric channel (NIST SRD 62)");
    println!(
        "  Supercurrent (electric): {} sources, {} series, {} penetration-depth points",
        n_sources, n_series, n_points
    );
    println!(
        "  temperature series (>=2 points per source+condition): {} — {}",
        with_series,
        if with_series == 0 {
            "no statement — no series carries two points"
        } else {
            "per-source λ(T) series present"
        }
    );

    let mut widest: Option<(String, String, usize)> = None;
    for s in &bin.series {
        let rho: Vec<f64> = s
            .points
            .iter()
            .filter_map(|p| lambda_inv2_m2(p.lambda_m))
            .collect();
        if rho.is_empty() {
            println!(
                "    {} [{}]: {} points, all non-physical λ (0 honored)",
                s.id,
                s.label,
                s.points.len()
            );
            continue;
        }
        let rho_max = rho.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let rho_min = rho.iter().cloned().fold(f64::INFINITY, f64::min);
        println!(
            "    {} [{}]: {} points, ρ_s ∝ λ⁻² range [{:.3e}, {:.3e}] m⁻²",
            s.id,
            s.label,
            s.points.len(),
            rho_min,
            rho_max
        );
        if let Some((_, _, wid_n)) = &widest {
            if s.points.len() > *wid_n {
                widest = Some((s.id.clone(), s.label.clone(), s.points.len()));
            }
        } else {
            widest = Some((s.id.clone(), s.label.clone(), s.points.len()));
        }
    }

    let verdict = if n_series == 0 {
        "no statement — the electric channel carries no harvest".to_string()
    } else if with_series == 0 {
        "no statement — no series carries a temperature curve; single points only, no ρ_s(T)"
            .to_string()
    } else {
        match &widest {
            Some((id, label, n)) => format!(
                "the electric channel is measured — {} sources; {} [{}] as the longest λ(T) series ({} points); ρ_s ∝ λ⁻² per series",
                n_sources, id, label, n
            ),
            None => "no statement".to_string(),
        }
    };
    println!("  verdict: {}", verdict);
}
