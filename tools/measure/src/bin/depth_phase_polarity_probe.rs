use omegaflow::ak135::{
    free_surface_pp, free_surface_sp, p_p_rayparam, p_travel, surface_incidence_deg,
    surface_p_velocity, surface_s_velocity,
};

const PILOT_DEPTH_KM: f64 = 231.0;
const PILOT_DELTA_DEG: f64 = 30.7;
const DEG_TO_KM: f64 = 111.1949;

fn main() {
    let (Some(alpha), Some(beta)) = (surface_p_velocity(), surface_s_velocity()) else {
        eprintln!(
            "the ak135 surface velocities stay unread — no free-surface derivation (0 honored)"
        );
        return;
    };
    println!("=== depth-phase polarity — the free-surface reflection coefficient re-derived at deep geometry ===");
    println!(
        "surface layer of the ak135 model the code already carries: vp = {alpha:.2} km/s, vs = {beta:.2} km/s"
    );
    println!(
        "R_pp(p) and R_sp(p) are the P->P and SV->P reflection ratios at the free surface; the"
    );
    println!(
        "recorded pP/sP sign = sign(source radiation ratio) * sign(R) — the free-surface factor is sign(R)"
    );
    println!();

    println!("R(p) across the incidence band (the sign is the polarity factor):");
    println!("{:>8}  {:>8}  {:>8}", "i_surf", "R_pp", "R_sp");
    for deg in [0.0f64, 10.0, 20.0, 30.0, 45.0, 60.0] {
        let p = deg.to_radians().sin() / alpha;
        let fmt = |o: Option<f64>| match o {
            Some(v) => format!("{v:+.3}"),
            None => "absent".to_string(),
        };
        println!(
            "{:>8.1}  {:>8}  {:>8}",
            deg,
            fmt(free_surface_pp(p)),
            fmt(free_surface_sp(p))
        );
    }
    println!();

    match find_pp_zero() {
        Some(z) => println!(
            "R_pp crosses zero at incidence ~{z:.1} deg — below that angle the free surface inverts pP, above it pP keeps the source sign"
        ),
        None => println!(
            "R_pp carries no zero crossing in the P incidence band — polarity stays inverted"
        ),
    }

    let p_direct = rayparam_of_p(PILOT_DELTA_DEG);
    let i_pilot = p_direct.and_then(surface_incidence_deg);
    let (Some(p_p), Some(i_s)) = (p_direct, i_pilot) else {
        eprintln!("the direct-P ray parameter at {PILOT_DELTA_DEG} deg stays unread — no incidence estimate");
        return;
    };
    let r_pp_pilot = free_surface_pp(p_p);
    let r_sp_pilot = free_surface_sp(p_p);
    println!(
        "pilot event us10003re5 (catalog depth {PILOT_DEPTH_KM:.0} km): the pP downleg is a teleseismic P at ~{PILOT_DELTA_DEG} deg,"
    );
    println!(
        "  surface incidence ~{i_s:.1} deg -> R_pp = {}, R_sp = {}",
        r_pp_pilot
            .map(|v| format!("{v:+.2}"))
            .unwrap_or("absent".into()),
        r_sp_pilot
            .map(|v| format!("{v:+.2}"))
            .unwrap_or("absent".into()),
    );
    println!();
    println!("the derivation:");
    println!(
        "  the free surface reflects pP inverted (R_pp negative) across the whole steep band — the pilot sits at ~{i_s:.1} deg"
    );
    println!("  the S->P conversion inverts sP (R_sp ~ -1) across the same band");
    println!(
        "  so the measured sP (negative at all six stations) is carried by the free surface alone"
    );
    println!(
        "  but the measured pP MIX (+ at four, - at two) is not: the free-surface factor is the same sign at every station"
    );
    println!(
        "  the pP mix must come from the source radiation ratio (upgoing vs downgoing P take-off sign),"
    );
    println!(
        "  which is pending without a focal mechanism — the shallow-source pP-inverted rule is refined, not confirmed"
    );
    println!();
    let rp = p_p_rayparam(PILOT_DELTA_DEG, PILOT_DEPTH_KM);
    println!(
        "note: the pP ray parameter near {PILOT_DELTA_DEG} deg is multi-valued ({} — the pP travel-time curve is non-monotonic there,",
        rp.map(|v| format!("{v:.4} s/km")).unwrap_or("absent".into())
    );
    println!(
        "  a triplication in the deep-source pP branch) — so a single per-station pP ray parameter is not a clean value;"
    );
    println!(
        "  the incidence estimate above uses the smooth direct P, which bounds the same steep band"
    );
}

fn rayparam_of_p(delta_deg: f64) -> Option<f64> {
    let d = 0.5;
    let t_lo = p_travel(delta_deg - d)?;
    let t_hi = p_travel(delta_deg + d)?;
    Some((t_hi - t_lo) / (2.0 * d) / DEG_TO_KM)
}

fn find_pp_zero() -> Option<f64> {
    let alpha = surface_p_velocity()?;
    let mut prev: Option<(f64, f64)> = None;
    let mut p = 0.0;
    while p < 1.0 / alpha {
        let r = free_surface_pp(p)?;
        if let Some((pp, rr)) = prev {
            if rr < 0.0 && r > 0.0 {
                let frac = rr / (rr - r);
                let p_zero = pp + frac * (p - pp);
                return Some((p_zero * alpha).asin().to_degrees());
            }
        }
        prev = Some((p, r));
        p += 0.0005;
    }
    None
}
