#![allow(mixed_script_confusables)]

use omegaflow::archivar::{parse_ephemeris_binary, BodyEphemeris, J2000_EPOCH};

const DAY_S: f64 = 86400.0;

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.6}"),
        None => "-".to_string(),
    }
}

fn granule_span(granules: &[omegaflow::archivar::ChebyshevGranule]) -> (f64, f64) {
    let mut lo = f64::MAX;
    let mut hi = f64::MIN;
    for g in granules {
        let a = g.t0_jd - g.dt_jd;
        let b = g.t0_jd + g.dt_jd;
        if a < lo {
            lo = a;
        }
        if b > hi {
            hi = b;
        }
    }
    (lo, hi)
}

fn cadence_median(granules: &[omegaflow::archivar::ChebyshevGranule]) -> Option<f64> {
    if granules.len() < 2 {
        return None;
    }
    let mut t0: Vec<f64> = granules.iter().map(|g| g.t0_jd).collect();
    t0.sort_by(f64::total_cmp);
    let mut gaps: Vec<f64> = t0.windows(2).map(|w| w[1] - w[0]).collect();
    gaps.sort_by(f64::total_cmp);
    Some(gaps[gaps.len() / 2])
}

fn matrix_span(matrices: &[(f64, [f64; 9])]) -> (f64, f64) {
    let mut lo = f64::MAX;
    let mut hi = f64::MIN;
    for (t, _) in matrices {
        if *t < lo {
            lo = *t;
        }
        if *t > hi {
            hi = *t;
        }
    }
    (lo, hi)
}

fn report(path: &str, eph: &BodyEphemeris, size: u64) {
    let (lo, hi) = granule_span(&eph.granules);
    let years = (hi - lo) / 365.25;
    let cad = cadence_median(&eph.granules);
    let (mlo, mhi) = matrix_span(&eph.rotation_matrices);
    let props = match &eph.props {
        Some(p) => format!(
            "α0 {:.4} δ0 {:.4} w0 {:.4} radius {:.0} m flattening {} gm {} j2 {} j4 {} ω_g {}",
            p.α0_deg,
            p.δ0_deg,
            p.w0_deg,
            p.radius_m,
            fmt_opt(p.flattening),
            fmt_opt(p.gm),
            fmt_opt(p.j2),
            fmt_opt(p.j4),
            match p.omega_g {
                Some((v, s)) => format!("({v:.6},{s:.6})"),
                None => "-".to_string(),
            },
        ),
        None => "props absent".to_string(),
    };
    let granules = eph.granules.len();
    let cadence = fmt_opt(cad);
    let matrices = eph.rotation_matrices.len();
    let nutation = eph
        .props
        .as_ref()
        .and_then(|p| p.nutation.as_ref())
        .map(|n| n.len())
        .map_or_else(|| "-".to_string(), |n| n.to_string());
    println!(
        "structure {path}: {size} B · {granules} granules · span jd [{lo:.3}..{hi:.3}] = {years:.0} yr · cadence {cadence} d · {matrices} matrices span jd [{mlo:.3}..{mhi:.3}] · nutation {nutation} · {props}",
    );
    let anchor = J2000_EPOCH + (1503273600.0 + 18.0 * 3600.0 + 26.0 * 60.0 + 40.0) / DAY_S;
    let covers_anchor = lo <= anchor && anchor <= hi;
    println!("  anchor 2017-08-21 jd {anchor:.3} covered: {covers_anchor}");
}

fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("structure: bin paths absent — pass one or more ephemeris_*.bin paths");
        return;
    }
    for path in &paths {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("structure {path}: reads void — {e}");
                continue;
            }
        };
        let size = bytes.len() as u64;
        let eph = match parse_ephemeris_binary(&bytes) {
            Some(e) => e,
            None => {
                eprintln!("structure {path}: {size} B carry no CF-86 BodyEphemeris contract");
                continue;
            }
        };
        report(path, &eph, size);
    }
}
