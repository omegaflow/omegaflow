use std::collections::HashMap;

use omegaflow::archivar::camera_angles::parse_camera_angles;
use omegaflow::archivar::{BodyEphemeris, body_barycenter_position, parse_ephemeris_binary};

const J2000_EPOCH: f64 = 2451545.0;
const DEFAULT_ANGLES: &str = "state/research/m10-camera-angles/camera_angles.txt";
const M10_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_mariner10.bin";
const MERCURY_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_mercury.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn km(v: [f64; 3]) -> [f64; 3] {
    [v[0] * 1e-3, v[1] * 1e-3, v[2] * 1e-3]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn unit(v: [f64; 3]) -> Option<[f64; 3]> {
    let n = norm(v);
    if n > 0.0 {
        Some([v[0] / n, v[1] / n, v[2] / n])
    } else {
        None
    }
}

fn ra_deg(u: [f64; 3]) -> f64 {
    u[1].atan2(u[0]).to_degrees().rem_euclid(360.0)
}

fn dec_deg(u: [f64; 3]) -> f64 {
    u[2].clamp(-1.0, 1.0).asin().to_degrees()
}

fn angsep_deg(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let (a1, d1) = (ra1.to_radians(), dec1.to_radians());
    let (a2, d2) = (ra2.to_radians(), dec2.to_radians());
    let c = d1.sin() * d2.sin() + d1.cos() * d2.cos() * (a1 - a2).cos();
    c.clamp(-1.0, 1.0).acos().to_degrees()
}

fn coverage_jd(e: &BodyEphemeris) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &e.granules {
        lo = lo.min(g.t0_jd - g.dt_jd);
        hi = hi.max(g.t0_jd + g.dt_jd);
    }
    if hi > lo { Some((lo, hi)) } else { None }
}

fn load(path: &str) -> Option<BodyEphemeris> {
    std::fs::read(path)
        .ok()
        .and_then(|b| parse_ephemeris_binary(&b))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let angles_path = match arg_value(&args, "--angles") {
        Some(v) => v,
        None => DEFAULT_ANGLES.to_string(),
    };
    let m10_path = match arg_value(&args, "--m10-eph") {
        Some(v) => v,
        None => M10_EPH.to_string(),
    };
    let mercury_path = match arg_value(&args, "--mercury-eph") {
        Some(v) => v,
        None => MERCURY_EPH.to_string(),
    };

    let text = match std::fs::read_to_string(&angles_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("m10_attitude_check: {angles_path} reads void: {e}");
            std::process::exit(1);
        }
    };
    let frames = parse_camera_angles(&text);
    if frames.is_empty() {
        eprintln!("m10_attitude_check: no camera-angle records in {angles_path}");
        std::process::exit(1);
    }

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    match load(&m10_path) {
        Some(e) => {
            if let Some((lo, hi)) = coverage_jd(&e) {
                eprintln!(
                    "mariner10: {} granules, JD {lo:.3}..{hi:.3}",
                    e.granules.len()
                );
            }
            eph.insert("mariner10".into(), e);
        }
        None => {
            eprintln!("mariner10: {m10_path} reads void — the position stays unmeasured");
            std::process::exit(1);
        }
    }
    match load(&mercury_path) {
        Some(e) => {
            eph.insert("mercury".into(), e);
        }
        None => {
            eprintln!("mercury: {mercury_path} reads void — the target stays unmeasured");
            std::process::exit(1);
        }
    }

    let Some((m10_lo, m10_hi)) = eph.get("mariner10").and_then(coverage_jd) else {
        eprintln!("mariner10: no coverage");
        std::process::exit(1);
    };

    println!("=== M10 attitude check — SPK+Mercury vs camera_angles ===");
    println!("angles: {angles_path}");
    println!("mariner10 ephemeris coverage: JD {m10_lo:.4}..{m10_hi:.4}");

    let m1: Vec<_> = frames
        .iter()
        .filter(|f| f.jd >= m10_lo && f.jd <= m10_hi)
        .collect();
    println!(
        "M1 frames inside the M10 coverage: {} (of {} total)",
        m1.len(),
        frames.len()
    );
    if m1.is_empty() {
        std::process::exit(1);
    }

    let mut n = 0usize;
    let mut sum_s = 0.0f64;
    let mut max_s = 0.0f64;
    let mut max_s_jd = 0.0f64;
    let mut sum_sep = 0.0f64;
    let mut max_sep = 0.0f64;
    let mut max_sep_jd = 0.0f64;
    let mut sum_sep_planet = 0.0f64;
    let mut max_sep_planet = 0.0f64;
    let mut worst: Option<(String, f64, [f64; 3], [f64; 3], [f64; 3])> = None;

    for f in &m1 {
        let tdb = (f.jd - J2000_EPOCH) * 86400.0;
        let (Some(sc), Some(me)) = (
            body_barycenter_position("mariner10", tdb, &eph),
            body_barycenter_position("mercury", tdb, &eph),
        ) else {
            continue;
        };
        let sc = km(sc);
        let me = km(me);
        let vec = sub(sc, me);
        let s = [f.sx, f.sy, f.sz];
        let ds = norm(sub(vec, s));
        let Some(dir) = unit(sub(me, sc)) else {
            continue;
        };
        let ra = ra_deg(dir);
        let dec = dec_deg(dir);
        let sep = angsep_deg(ra, dec, f.c1, f.c2);

        n += 1;
        sum_s += ds;
        if ds > max_s {
            max_s = ds;
            max_s_jd = f.jd;
            worst = Some((f.fds.clone(), f.jd, vec, s, sc));
        }
        sum_sep += sep;
        if sep > max_sep {
            max_sep = sep;
            max_sep_jd = f.jd;
        }
        let sep_planet = angsep_deg(ra, dec, f.c1, f.c2);
        sum_sep_planet += sep_planet;
        max_sep_planet = max_sep_planet.max(sep_planet);

        if n <= 8 {
            println!(
                "FDS {} JD {:.5} |vec|={:.1} |S|={:.1} |vec-S|={:.1} km | RA_dir={:.4} Dec_dir={:.4} C1={:.4} C2={:.4} sep={:.4} deg",
                f.fds,
                f.jd,
                norm(vec),
                norm(s),
                ds,
                ra,
                dec,
                f.c1,
                f.c2,
                sep
            );
        }
    }

    if n == 0 {
        println!("no frame carried both readings — the check stays unmeasured");
        std::process::exit(1);
    }
    println!();
    println!("frames measured: {n}");
    println!(
        "|vec - SXSYSZ|: mean {:.1} km, max {:.1} km @ JD {:.5}",
        sum_s / n as f64,
        max_s,
        max_s_jd
    );
    println!(
        "angsep( M10->Mercury , C1C2 ): mean {:.4} deg, max {:.4} deg @ JD {:.5}",
        sum_sep / n as f64,
        max_sep,
        max_sep_jd
    );
    println!(
        "angsep( RA/Dec_planet , C1C2 ): mean {:.4} deg, max {:.4} deg",
        sum_sep_planet / n as f64,
        max_sep_planet
    );
    if let Some((fds, jd, vec, s, sc)) = worst {
        println!();
        println!("worst |vec-S| frame FDS {fds} JD {jd:.5}");
        println!(
            "  sc (M10 SSB)   [{:.1}, {:.1}, {:.1}] km",
            sc[0], sc[1], sc[2]
        );
        println!(
            "  vec (sc-me)    [{:.1}, {:.1}, {:.1}] km |vec| {:.1}",
            vec[0],
            vec[1],
            vec[2],
            norm(vec)
        );
        println!(
            "  SXSYSZ (file)  [{:.1}, {:.1}, {:.1}] km |S| {:.1}",
            s[0],
            s[1],
            s[2],
            norm(s)
        );
    }
}
