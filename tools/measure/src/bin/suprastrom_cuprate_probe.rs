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

    let n_cit = bin.citations.len();
    let n_points: usize = bin.citations.iter().map(|c| c.points.len()).sum();
    let with_series = bin.citations.iter().filter(|c| c.points.len() >= 2).count();

    println!("suprastrom_cuprate_probe — the Kuprat electric channel (NIST SRD 62)");
    println!(
        "  Supercurrent (electric): {} citations, {} penetration-depth points",
        n_cit, n_points
    );
    println!(
        "  series (>=2 T-points per source): {} — {}",
        with_series,
        if with_series == 0 {
            "no statement — every source carries a single point"
        } else {
            "per-source λ(T) series present"
        }
    );

    let mut widest: Option<(String, usize)> = None;
    for c in &bin.citations {
        let rho: Vec<f64> = c
            .points
            .iter()
            .filter_map(|p| lambda_inv2_m2(p.lambda_m))
            .collect();
        if rho.is_empty() {
            println!(
                "    {}: {} points, all non-physical λ (0 honored)",
                c.id,
                c.points.len()
            );
            continue;
        }
        let rho_max = rho.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let rho_min = rho.iter().cloned().fold(f64::INFINITY, f64::min);
        println!(
            "    {}: {} points, ρ_s ∝ λ⁻² range [{:.3e}, {:.3e}] m⁻²",
            c.id,
            c.points.len(),
            rho_min,
            rho_max
        );
        if let Some((_, wid_n)) = &widest {
            if c.points.len() > *wid_n {
                widest = Some((c.id.clone(), c.points.len()));
            }
        } else {
            widest = Some((c.id.clone(), c.points.len()));
        }
    }

    let verdict = if n_cit == 0 {
        "no statement — the electric channel carries no harvest".to_string()
    } else if with_series == 0 {
        "no statement — no source carries a temperature series; single points only, no ρ_s(T) curve"
            .to_string()
    } else {
        match &widest {
            Some((id, n)) => format!(
                "the electric channel is measured — {} sources, {} as the longest λ(T) series ({} points); ρ_s ∝ λ⁻² per source",
                n_cit, id, n
            ),
            None => "no statement".to_string(),
        }
    };
    println!("  verdict: {}", verdict);
}
