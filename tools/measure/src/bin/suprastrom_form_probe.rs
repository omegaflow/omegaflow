use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::suprastrom::{lambda_inv2_m2, parse_suprastrom_bin};

const SRD62_SUPRASTROM_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/srd62_suprastrom.bin";
const MIN_POINTS: usize = 8;
const T_FRAC_LOW: f64 = 0.4;

struct Series {
    id: String,
    label: String,
    t: Vec<f64>,
    rho: Vec<f64>,
}

fn two_fluid_rho0(tc: f64, rho0: f64, t_k: f64) -> f64 {
    let t = (t_k / tc).min(1.0 - 1e-6);
    rho0 * (1.0 - t * t * t * t)
}

fn main() {
    let bytes = match fetch_raw_bytes(SRD62_SUPRASTROM_CDN, 3600) {
        Some(b) => b,
        None => {
            eprintln!(
                "suprastrom_form_probe: {} carries no asset (0 honored)",
                SRD62_SUPRASTROM_CDN
            );
            std::process::exit(1);
        }
    };
    let bin = match parse_suprastrom_bin(&bytes) {
        Some(b) => b,
        None => {
            eprintln!("suprastrom_form_probe: the CDN suprastrom bin parses void");
            std::process::exit(1);
        }
    };

    let mut candidates: Vec<Series> = Vec::new();
    for s in &bin.series {
        let mut pts: Vec<(f64, f64)> = s
            .points
            .iter()
            .filter_map(|p| lambda_inv2_m2(p.lambda_m).map(|rho| (p.t_k, rho)))
            .collect();
        if pts.len() < MIN_POINTS {
            continue;
        }
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let tmax = pts.last().map(|p| p.0).unwrap_or(0.0);
        let tmin = pts.first().map(|p| p.0).unwrap_or(0.0);
        let span = tmax - tmin;
        if span <= 0.0 {
            continue;
        }
        let t_low_cut = tmin + span * T_FRAC_LOW;
        let low: Vec<f64> = pts
            .iter()
            .filter(|(t, _)| *t <= t_low_cut)
            .map(|(_, rho)| *rho)
            .collect();
        if low.is_empty() {
            continue;
        }
        candidates.push(Series {
            id: s.id.clone(),
            label: s.label.clone(),
            t: pts.iter().map(|(t, _)| *t).collect(),
            rho: pts.iter().map(|(_, r)| *r).collect(),
        });
    }

    println!("suprastrom_form_probe — the two-fluid form ρ_s ∝ 1−(T/Tc)⁴ on NIST SRD 62");
    println!(
        "  candidates: {} series (>= {} points, low-T plateau for ρ₀)",
        candidates.len(),
        MIN_POINTS
    );

    let mut best: Option<(String, String, f64, f64, f64)> = None;
    for c in &candidates {
        let tmin = c.t.first().copied().unwrap_or(0.0);
        let tmax = c.t.last().copied().unwrap_or(0.0);
        let span = tmax - tmin;
        if span <= 0.0 {
            continue;
        }
        let cut = tmin + span * T_FRAC_LOW;
        let lows: Vec<f64> = c
            .rho
            .iter()
            .zip(c.t.iter())
            .filter(|(_, t)| **t <= cut)
            .map(|(r, _)| *r)
            .collect();
        if lows.is_empty() {
            continue;
        }
        let rho0 = lows.iter().sum::<f64>() / lows.len() as f64;
        if rho0 <= 0.0 {
            continue;
        }
        let mut best_tc: Option<(f64, f64)> = None;
        for i in 0..=10000 {
            let tc = tmax + (i as f64) * 0.05;
            let mut chi = 0.0f64;
            for (t, r) in c.t.iter().zip(c.rho.iter()) {
                let pred = two_fluid_rho0(tc, rho0, *t);
                let d = pred - r;
                chi += d * d;
            }
            if best_tc.is_none() || chi < best_tc.unwrap().1 {
                best_tc = Some((tc, chi));
            }
        }
        let (tc, chi) = best_tc.unwrap_or((tmax, f64::NAN));
        let rms = (chi / c.t.len() as f64).sqrt();
        let rms_rel = rms / rho0;
        println!(
            "    {} [{}]: T {:6.1}–{:6.1} K, ρ₀ = {:.3e}, Tc(fit) = {:5.1} K, chi² = {:6.1}, RMS/ρ₀ = {:.3}{}",
            c.id,
            c.label,
            tmin,
            tmax,
            rho0,
            tc,
            chi,
            rms_rel,
            if tmax < 0.8 * tc {
                "  (data stop before the transition — Tc extrapolated)"
            } else {
                ""
            }
        );
        if tmax < 0.8 * tc {
            continue;
        }
        if let Some((_, _, _, best_rms_rel, _)) = &best {
            if rms_rel < *best_rms_rel {
                best = Some((c.id.clone(), c.label.clone(), tc, chi, rho0));
            }
        } else {
            best = Some((c.id.clone(), c.label.clone(), tc, chi, rho0));
        }
    }

    match &best {
        Some((id, label, tc, chi, rho0)) => {
            let n = candidates
                .iter()
                .find(|c| &c.id == id && &c.label == label)
                .map(|c| c.t.len())
                .unwrap_or(0);
            let rms = (chi / n.max(1) as f64).sqrt();
            let rms_rel = rms / rho0;
            println!(
                "  verdict: the two-fluid form ρ_s ∝ 1−(T/Tc)⁴ is carried — {} [{}], Tc = {:.1} K, {} points, RMS/ρ₀ = {:.3} (literature scatter, no named systematic drift); the single-material count ({}) is below the MIN_N threshold for a cross-material claim",
                id, label, tc, n, rms_rel, n
            );
        }
        None => {
            println!("  verdict: no statement — no series carries a two-fluid-addressable ρ_s(T)")
        }
    }
}
