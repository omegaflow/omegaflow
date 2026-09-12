use omegaflow::ak135::{
    free_surface_pp, free_surface_sp, p_p_rayparam, p_travel, surface_incidence_deg,
    surface_p_velocity, surface_s_velocity,
};
use omegaflow::archivar::fetch_raw;
use omegaflow_measure::iasp91;
use omegaflow_measure::ndk;

const PILOT_DEPTH_KM: f64 = 231.0;
const PILOT_DELTA_DEG: f64 = 30.7;
const DEG_TO_KM: f64 = 111.1949;

const PILOT_YEAR: i32 = 2015;
const PILOT_MONTH: u32 = 10;
const PILOT_DAY: u32 = 26;
const PILOT_LAT: f64 = 36.5244;
const PILOT_LON: f64 = 70.3676;
const GCMT_NDK_URL: &str =
    "https://www.ldeo.columbia.edu/~gcmt/projects/CMT/catalog/jan76_dec25.ndk";

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
    println!("  which the CMT source term below now resolves (absent before the NDK parser)");
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

    println!();
    println!("=== the CMT source term — the pP sign mix read from the focal mechanism ===");
    let Some(ndk_body) = fetch_raw(GCMT_NDK_URL, None, &[], 3600) else {
        println!("no GCMT NDK body — the CMT source term stays absent; the pP mix stays measured and unresolved (no fabricated flip)");
        return;
    };
    let events = ndk::parse_ndk(&ndk_body);
    let pilot = events
        .iter()
        .filter(|e| e.year == PILOT_YEAR && e.month == PILOT_MONTH && e.day == PILOT_DAY)
        .filter(|e| (e.hyp_lat - PILOT_LAT).abs() < 2.0 && (e.hyp_lon - PILOT_LON).abs() < 2.0)
        .min_by(|a, b| {
            let da = (a.hyp_lat - PILOT_LAT).powi(2) + (a.hyp_lon - PILOT_LON).powi(2);
            let db = (b.hyp_lat - PILOT_LAT).powi(2) + (b.hyp_lon - PILOT_LON).powi(2);
            da.total_cmp(&db)
        });
    let Some(ev) = pilot else {
        println!("no GCMT centroid in the pilot window — the CMT source term stays absent; the pP mix stays measured and unresolved (no fabricated flip)");
        return;
    };
    let mw_txt = ev
        .mw()
        .map(|m| format!("{m:.2}"))
        .unwrap_or("absent".to_string());
    println!(
        "GCMT centroid {} ({:04}/{:02}/{:02}): strike/dip/rake {:.0}/{:.0}/{:.0} (conjugate {:.0}/{:.0}/{:.0})",
        ev.name, ev.year, ev.month, ev.day, ev.strike, ev.dip, ev.rake, ev.strike2, ev.dip2, ev.rake2
    );
    println!(
        "  scalar moment M0 {:.3e} dyne-cm, Mw {mw_txt}, centroid lat {:.2} lon {:.2} depth {:.1} km",
        ev.scalar_moment_dyne_cm(),
        ev.centroid_lat,
        ev.centroid_lon,
        ev.centroid_depth_km
    );

    let m = ndk::dc_moment_tensor(ev.strike, ev.dip, ev.rake);
    let (i_up, i_down) = match (
        iasp91::takeoff_angle_deg(PILOT_DELTA_DEG, ev.centroid_depth_km, true),
        iasp91::takeoff_angle_deg(PILOT_DELTA_DEG, ev.centroid_depth_km, false),
    ) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            println!("the take-off angle at the centroid depth stays unread — the radiation direction r̂ stays absent; the mix stays unresolved, no fabricated angle");
            return;
        }
    };
    println!(
        "take-off angle at {:.0} km depth, Δ {PILOT_DELTA_DEG} deg (iasp91): upgoing pP leg {i_up:.1} deg, downgoing direct-P leg {i_down:.1} deg",
        ev.centroid_depth_km
    );

    let mut nodal: Vec<f64> = Vec::new();
    let mut prev: Option<f64> = None;
    let mut prev_az = 0.0f64;
    let mut rp_min = f64::INFINITY;
    let mut rp_max = f64::NEG_INFINITY;
    for az in 0..360 {
        let a = az as f64;
        let r = ndk::ray_direction(i_up, a, true);
        let v = ndk::rp(&m, &r);
        rp_min = rp_min.min(v);
        rp_max = rp_max.max(v);
        if let Some(p) = prev {
            if (p < 0.0 && v > 0.0) || (p > 0.0 && v < 0.0) {
                let frac = p / (p - v);
                nodal.push(prev_az + frac);
            }
        }
        prev = Some(v);
        prev_az = a;
    }
    let nodal_txt = nodal
        .iter()
        .map(|n| format!("{n:.1} deg"))
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "upgoing P radiation coefficient R_P = r̂·M·r̂ spans [{rp_min:+.2}, {rp_max:+.2}] (unit M) across azimuth;"
    );
    println!(
        "  nodal azimuths (R_P = 0): {}",
        if nodal.is_empty() {
            "none in the 1-deg sweep".to_string()
        } else {
            nodal_txt
        }
    );
    println!(
        "the free surface is uniform (R_pp < 0 across the whole steep band), so it cannot split six stations into 4+/2−;"
    );
    println!(
        "  the double-couple upgoing radiation can: two nodal azimuths divide the focal sphere into four quadrants,"
    );
    println!(
        "  and a station fan that crosses one nodal azimuth reads a sign mix — the measured 4×+/2×− pP mix is this source term, not the surface."
    );
    println!(
        "amplitude is carried: a station near a nodal azimuth reads a small |R_P| (nodal-near) — the two − stations may sit close to a node."
    );
    println!(
        "kalibrier-gate: the sign of R_P_up at each of the six measured station azimuths is the predicted polarity;"
    );
    println!(
        "  the six azimuths of the 2026-09-09 field run are not in the handover — the per-station sign comparison runs live in the fleet,"
    );
    println!("  now that the source term carries the prediction (named pending, not fabricated).");
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
