use omegaflow::archivar::C_LIGHT;

const AU: f64 = 1.495_978_707e11;

struct Osc {
    name: &'static str,
    d: f64,
    dt: f64,
    val: f64,
    ttl: f64,
    extent: f64,
}

fn kernel0(d2: f64, extent: f64, exposure: f64) -> Option<f64> {
    let e = extent.max(exposure);
    if !(e.is_finite() && e > 0.0) {
        return None;
    }
    let v = 1.0 / (d2 + e * e);
    if v.is_finite() { Some(v) } else { None }
}

fn val_eff(val: f64, dt: f64, d: f64, ttl: f64) -> Option<f64> {
    if !(val.is_finite() && dt.is_finite() && d.is_finite() && ttl.is_finite() && ttl > 0.0) {
        return None;
    }
    let pre_w = val * (-dt / ttl).exp();
    let corr = dt.min(d / C_LIGHT);
    let v = if corr > ttl * 1e-4 {
        pre_w * (corr / ttl).exp()
    } else {
        pre_w
    };
    if v.is_finite() { Some(v) } else { None }
}

fn ds2_and_weight(dt: f64, d: f64, exposure: f64) -> Option<(f64, f64)> {
    if !(dt.is_finite() && d.is_finite() && exposure.is_finite() && exposure > 0.0) {
        return None;
    }
    let dt_c = dt * C_LIGHT;
    let ds2 = dt_c * dt_c - d * d;
    if !ds2.is_finite() {
        return None;
    }
    if ds2 < 0.0 {
        Some((ds2, 0.0))
    } else {
        Some((ds2, exposure / (exposure + ds2)))
    }
}

fn omega(fixture: &[Osc], exposure: f64, with_ds2: bool) -> f64 {
    let mut sum = 0.0;
    for o in fixture {
        let Some(ve) = val_eff(o.val, o.dt, o.d, o.ttl) else {
            continue;
        };
        let d2 = o.d * o.d;
        let Some(sk) = kernel0(d2, o.extent, exposure) else {
            continue;
        };
        let c = ve * sk;
        sum += if with_ds2 {
            match ds2_and_weight(o.dt, o.d, exposure) {
                Some((_, w)) => c * w,
                None => continue,
            }
        } else {
            c
        };
    }
    sum
}

fn spacelike_count(fixture: &[Osc], exposure: f64) -> usize {
    fixture
        .iter()
        .filter(|o| matches!(ds2_and_weight(o.dt, o.d, exposure), Some((ds2, _)) if ds2 < 0.0))
        .count()
}

fn sun(dt: f64) -> Osc {
    Osc {
        name: "sun",
        d: AU,
        dt,
        val: 1.0,
        ttl: 3600.0,
        extent: 0.0,
    }
}

fn local(dt: f64) -> Osc {
    Osc {
        name: "local-sensor",
        d: 1.0,
        dt,
        val: 1.0,
        ttl: 60.0,
        extent: 0.0,
    }
}

fn half_max_after_lightlike(d: f64, exposure: f64) -> Option<f64> {
    if !(d.is_finite() && d > 0.0 && exposure.is_finite() && exposure > 0.0) {
        return None;
    }
    let dt_c_at_half = (d * d + exposure).sqrt();
    if !dt_c_at_half.is_finite() {
        return None;
    }
    Some((dt_c_at_half - d) / C_LIGHT)
}

fn main() {
    println!("=== Minkowski ds² delta probe — the ω-loop with and without the ds² factor ===");
    println!(
        "ω-loop (measured site): FIELD_WGSL presence_probe (src/mathematikerin/shaders.rs:329-348), the sum omegas[f] += c.x (line 347)."
    );
    println!(
        "Per-oscillator factors mirrored: temporal decay pre.w = m.w·exp(-temporal/ttl) (line 339), retardation corr = min(temporal, d/v) (val_eff_at, lines 111-119), kernel 0 = 1/(d2 + e²), e = max(extent, exposure) (line 46)."
    );
    println!(
        "ds² factor (spec, docs/specs/minkowski-field-permeability.md lines 63-69): dt_c = |Δt|·c; ds² = dt_c² − d²; ds² < 0 → weight 0; else weight = exposure/(exposure + ds²)."
    );
    println!(
        "Layer: the factor is an eval-time weight of the WGSL sum; the Archivar cone gate (query_hash/signal_reach, src/archivar/spatial.rs:382-409) is a per-medium inclusion gate that does not hold t_presence — see the session report."
    );
    println!(
        "Fixture: em oscillators, val = 1.0, extent = 0.0 (e = exposure); exposure is the named field-energy parameter (vp.surface.w = max(|ω|, 2⁻⁶⁴))."
    );
    println!();

    let scenarios: Vec<(&'static str, &'static str, Vec<Osc>)> = vec![
        (
            "empty",
            "no oscillator — the degenerate case: the sum is the honest zero, never a default",
            vec![],
        ),
        (
            "fresh-fetch sun",
            "d = 1 AU, dt = 0 s (epoch = fetch time — the spec's own solar example, lines 81-87)",
            vec![sun(0.0)],
        ),
        (
            "lightlike sun",
            "d = 1 AU, dt = d/c = 499.0 s (emission-corrected epoch, ds² = 0)",
            vec![sun(AU / C_LIGHT)],
        ),
        (
            "half-light sun",
            "d = 1 AU, dt = d/(2c) (spacelike — inside the horizon, no signal can have arrived)",
            vec![sun(AU / (2.0 * C_LIGHT))],
        ),
        (
            "deep-timelike sun",
            "d = 1 AU, dt = 2·d/c (timelike — the event is older than its light time)",
            vec![sun(2.0 * AU / C_LIGHT)],
        ),
        (
            "fresh-fetch local sensor",
            "d = 1 m, dt = 0 s (browser sensor, epoch = read time)",
            vec![local(0.0)],
        ),
        (
            "lightlike local sensor",
            "d = 1 m, dt = d/c = 3.34 ns",
            vec![local(1.0 / C_LIGHT)],
        ),
        (
            "mixed field",
            "fresh-fetch sun + lightlike local sensor — the field with both cosmically stale and cone-fitting records",
            vec![sun(0.0), local(1.0 / C_LIGHT)],
        ),
    ];

    let exposures: [f64; 3] = [1.0, 1e-3, 1e-6];

    for (scenario, what, fixture) in &scenarios {
        println!("== {scenario} ==");
        println!("   {what}");
        for &exposure in &exposures {
            let omega_plain = omega(fixture, exposure, false);
            let omega_ds2 = omega(fixture, exposure, true);
            let delta = omega_ds2 - omega_plain;
            let delta_rel = if omega_plain != 0.0 {
                Some(delta / omega_plain)
            } else {
                None
            };
            let rel_word = match delta_rel {
                Some(r) => format!("{:.3} %", r * 100.0),
                None => "absent (ω_plain = 0 — the honest zero, never a default)".to_string(),
            };
            println!(
                "   exposure {:>8.1e} | ω_plain {:>12.6e} | ω_ds2 {:>12.6e} | Δ {:>12.6e} | Δ_rel {:>24} | spacelike {}/{}",
                exposure,
                omega_plain,
                omega_ds2,
                delta,
                rel_word,
                spacelike_count(fixture, exposure),
                fixture.len(),
            );
        }
        for o in fixture {
            match ds2_and_weight(o.dt, o.d, 1e-3) {
                Some((ds2, w)) => println!(
                    "   {}: ds² = {:>12.6e} m² | weight = {:>12.6e} (exposure 1e-3)",
                    o.name, ds2, w
                ),
                None => println!("   {}: ds²/weight absent", o.name),
            }
        }
        println!();
    }

    println!("=== The corridor — the factor's live regime (sun, d = 1 AU) ===");
    println!(
        "weight(ds²): 1.0 at ds² = 0 (lightlike); 0.0 for ds² < 0 (spacelike, a cliff at the cone); exposure/(exposure + ds²) for ds² > 0."
    );
    for &exposure in &exposures {
        let analytic = exposure / (2.0 * AU * C_LIGHT);
        match half_max_after_lightlike(AU, exposure) {
            Some(w) => println!(
                "   exposure {:<8.1e}: analytic half-max width exposure/(2·d·c) = {:.3e} s; f64 width at d = 1 AU = {:.3e} s (the window is below f64 resolution — d² + exposure == d² in f64, absorbed)",
                exposure, analytic, w
            ),
            None => println!("   exposure {:<8.1e}: corridor absent", exposure),
        }
    }
    println!();

    println!("=== Verdict ===");
    let mut zeroed = 0usize;
    let mut kept = 0usize;
    for (_, _, fixture) in &scenarios {
        if fixture.is_empty() {
            continue;
        }
        let omega_plain = omega(fixture, 1e-3, false);
        let omega_ds2 = omega(fixture, 1e-3, true);
        if omega_plain != 0.0 && omega_ds2 <= omega_plain.abs() * 1e-3 {
            zeroed += 1;
        } else if omega_plain != 0.0 && (omega_ds2 - omega_plain).abs() <= omega_plain.abs() * 1e-9
        {
            kept += 1;
        }
    }
    println!(
        "Measured at exposure 1e-3: {} of 7 non-empty scenarios carry ω_ds2 ≤ ω_plain·1e-3 (the factor zeroes the field — fresh-fetch, half-light and deep-timelike sun, fresh-fetch local sensor); {} scenarios carry ω_ds2 ≈ ω_plain (the lightlike fits, and the mixed field whose cone-fitting record dominates).",
        zeroed, kept
    );
    println!(
        "Δ at cosmic scale: −100 % wherever the epoch is not on the light cone — dt = 0 (fetch time) gives ds² = −d² < 0 → weight 0; dt = 2·d/c (deep timelike) gives weight = exposure/(exposure + 3·d²) ≈ 0. The mixed field shows the shape: the cosmically stale record is zeroed (spacelike 1/2), the cone-fitting record carries the field."
    );
    println!(
        "The factor's live regime is the light cone itself: a window of analytic width exposure/(2·d·c) ≈ 1.1e-23 s at d = 1 AU (exposure 1e-3) — below f64 resolution."
    );
    println!(
        "The empty field carries ω = 0, Δ = 0 — the degenerate case is the honest zero, never a default."
    );
}
