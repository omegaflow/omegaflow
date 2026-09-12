use omegaflow::ak135;
use omegaflow_measure::iasp91;

fn main() {
    println!(
        "=== head-wave gate 410/660 — the P triplication from the external iasp91 reference ==="
    );
    println!(
        "external reference: iasp91 (Kennett & Engdahl 1991, Geophys. J. Int. 105, 429-465), embedded mantle table ({} nodes)",
        iasp91::iasp91_nodes().len()
    );
    println!("internal reference: ak135 (Kennett, Engdahl & Buland 1995), src/archivar/ak135.rs");
    println!();
    println!("the triplication is the fold of Delta(p): Delta decreases with p in the smooth");
    println!("gradient, but increases with p across the discontinuity's bottoming branch, so one");
    println!("epicentral distance carries three P arrivals inside the bracket.");
    println!();

    let trips = iasp91::triplications();
    for t in &trips {
        println!(
            "iasp91 {} km discontinuity: triplication bracket Delta {:.2}..{:.2} deg,",
            t.depth_km, t.delta_lo_deg, t.delta_hi_deg
        );
        println!(
            "  slowness bracket {:.3}..{:.3} s/deg",
            t.slowness_lo_s_deg, t.slowness_hi_s_deg
        );
    }
    println!();

    let t410 = trips.iter().find(|t| (t.depth_km - 410.0).abs() < 1e-6);
    let t660 = trips.iter().find(|t| (t.depth_km - 660.0).abs() < 1e-6);

    println!("fold witness — the non-monotonic Delta(p) across the 410/660 slowness gaps:");
    println!("{:>8}  {:>8}  {:>8}", "p s/deg", "Delta deg", "T s");
    for &(p_deg, delta_deg, t) in iasp91::delta_sweep()
        .iter()
        .filter(|&&(p, d, _)| {
            let near_410 = t410
                .map(|t| p >= t.slowness_lo_s_deg - 0.4 && p <= t.slowness_hi_s_deg + 0.4)
                .unwrap_or(false);
            let near_660 = t660
                .map(|t| p >= t.slowness_lo_s_deg - 0.4 && p <= t.slowness_hi_s_deg + 0.4)
                .unwrap_or(false);
            (near_410 || near_660) && d < 45.0
        })
        .step_by(400)
    {
        println!("{p_deg:>8.3}  {delta_deg:>8.2}  {t:>8.2}");
    }
    println!();

    println!(
        "external/internal agreement — iasp91 vs ak135 first-arrival P (the depth reference):"
    );
    println!(
        "{:>8}  {:>8}  {:>8}  {:>8}",
        "Delta", "iasp91", "ak135", "diff s"
    );
    for d in [30.0, 45.0, 60.0, 75.0, 90.0] {
        match (iasp91::p_travel(d), ak135::p_travel(d)) {
            (Some(ti), Some(ta)) => {
                println!("{d:>8.1}  {ti:>8.2}  {ta:>8.2}  {:>+8.2}", ti - ta);
            }
            _ => println!("{d:>8.1}  absent in one model"),
        }
    }
    println!();

    println!("wiring status: pending");
    println!(
        "  the depth-phase probes (depth_phase_field/fleet/polarity) emit per-station delta_deg"
    );
    println!("  and the ak135-model pP/sP lag, but no measured ray parameter (slowness) from the");
    println!(
        "  observed P arrival, and no machine-readable delta+p handoff. The triplication bracket"
    );
    println!(
        "  is a property of the direct-P curve, so a station inside the bracket is the gate target"
    );
    println!("  once a probe emits the measured P slowness at its delta. Until then the gate self-measures");
    println!("  the model triplication (above) and the wiring stays named, not wired.");
}
