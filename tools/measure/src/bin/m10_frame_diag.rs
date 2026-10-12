use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, fetch_raw_bytes, parse_ephemeris_binary,
};
use omegaflow::cdn::body_url;

const J2000_EPOCH: f64 = 2451545.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_bytes(label: &str, bytes: &[u8]) -> Option<BodyEphemeris> {
    match parse_ephemeris_binary(bytes) {
        Some(e) => Some(e),
        None => {
            eprintln!("{label}: carries no ephemeris contract ({} B)", bytes.len());
            None
        }
    }
}

fn load(name: &str, explicit: Option<&str>) -> Option<(BodyEphemeris, String)> {
    if let Some(path) = explicit {
        match std::fs::read(path) {
            Ok(b) => {
                return parse_bytes(path, &b).map(|e| (e, path.to_string()));
            }
            Err(_) => eprintln!("{name}: explicit {path} reads void"),
        }
    }
    let local = format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin");
    if let Ok(b) = std::fs::read(&local) {
        if let Some(e) = parse_bytes(&local, &b) {
            return Some((e, local));
        }
    }
    let url = body_url(name);
    let bytes = match fetch_raw_bytes(&url) {
        Some(b) => b,
        None => {
            eprintln!("{name}: {url} fetch void — {name} stays absent");
            return None;
        }
    };
    if let Some(parent) = std::path::Path::new(&local).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let _ = std::fs::write(&local, &bytes);
    parse_bytes(&url, &bytes).map(|e| (e, format!("{url} -> {local}")))
}

fn koord(v: [f64; 3]) -> String {
    format!(
        "[{:>14.1}, {:>14.1}, {:>14.1}]",
        v[0] / 1000.0,
        v[1] / 1000.0,
        v[2] / 1000.0
    )
}

fn koord6(v: [f64; 6]) -> String {
    koord([v[0], v[2], v[4]])
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn neg3(v: [f64; 3]) -> [f64; 3] {
    [-v[0], -v[1], -v[2]]
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn cosine(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let (na, nb) = (norm(a), norm(b));
    if na > 0.0 && nb > 0.0 {
        Some((a[0] * b[0] + a[1] * b[1] + a[2] * b[2]) / (na * nb))
    } else {
        None
    }
}

fn tdb_of_jd(jd: f64) -> f64 {
    (jd - J2000_EPOCH) * 86400.0
}

const EPH_M10: &str = "data/ssd.jpl.nasa.gov/ephemeris_mariner10.bin";
const EPH_MERCURY: &str = "data/ssd.jpl.nasa.gov/ephemeris_mercury.bin";
const EPH_BARY1: &str = "data/ssd.jpl.nasa.gov/ephemeris_earth.bin";
const EPH_BARY2: &str = "data/ssd.jpl.nasa.gov/ephemeris_sun.bin";

fn eph_name(path: &str) -> String {
    match path.rsplit('/').next() {
        Some(base) => base
            .trim_start_matches("ephemeris_")
            .trim_end_matches(".bin")
            .to_string(),
        None => path.to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let names = [
        eph_name(EPH_M10),
        eph_name(EPH_MERCURY),
        eph_name(EPH_BARY1),
        eph_name(EPH_BARY2),
    ];

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut src: HashMap<String, String> = HashMap::new();
    println!("=== m10_frame_diag — centre identification ===");
    for name in &names {
        let explicit = arg_value(&args, &format!("--{name}-eph"));
        match load(name, explicit.as_deref()) {
            Some((e, path)) => {
                println!("{name:>9}: {} granules | source {}", e.granules.len(), path);
                src.insert(name.to_string(), path);
                eph.insert(name.to_string(), e);
            }
            None => println!("{name:>9}: absent"),
        }
    }

    println!();
    println!("--- granule coverage / first granule (t0_jd, dt_jd) ---");
    for name in &names {
        let Some(e) = eph.get(name) else { continue };
        if e.granules.is_empty() {
            println!("{name:>9}: 0 granules");
            continue;
        }
        let g0 = &e.granules[0];
        let gl = e.granules.last().unwrap();
        let lo = e
            .granules
            .iter()
            .map(|g| g.t0_jd - g.dt_jd)
            .fold(f64::INFINITY, f64::min);
        let hi = e
            .granules
            .iter()
            .map(|g| g.t0_jd + g.dt_jd)
            .fold(f64::NEG_INFINITY, f64::max);
        println!(
            "{name:>9}: t0 first {:.6} last {:.6} | dt first {:.6} | coverage JD {:.4}..{:.4}",
            g0.t0_jd, gl.t0_jd, g0.dt_jd, lo, hi
        );
    }

    println!();
    println!("--- raw vectors at M1 (km, J2000) + centre tests ---");
    for jd in [2442135.305_f64, 2442136.366, 2442137.370] {
        let t = tdb_of_jd(jd);
        let sc = body_barycenter_position(&names[0], t, &eph);
        let me = body_barycenter_position(&names[1], t, &eph);
        let sun = body_barycenter_position(&names[3], t, &eph);
        let mag = |v: Option<[f64; 3]>| match v {
            Some(v) => format!("{:.1} km", norm(v) / 1000.0),
            None => "absent".into(),
        };
        println!("JD {jd:.4}");
        println!(
            "   sc  {} |sc| = {}",
            sc.map(koord).unwrap_or("absent".into()),
            mag(sc)
        );
        println!(
            "   me  {} |me| = {}",
            me.map(koord).unwrap_or("absent".into()),
            mag(me)
        );
        println!(
            "   sun {} |sun| = {}",
            sun.map(koord).unwrap_or("absent".into()),
            mag(sun)
        );
        let d = |a: Option<[f64; 3]>, b: Option<[f64; 3]>| match (a, b) {
            (Some(a), Some(b)) => format!("{:.1}", norm(sub(a, b)) / 1000.0),
            _ => "absent".into(),
        };
        println!(
            "   |sc-me| = {} |sc-sun| = {} |me-sun| = {} km",
            d(sc, me),
            d(sc, sun),
            d(me, sun)
        );
        if let (Some(sc), Some(me), Some(sun)) = (sc, me, sun) {
            let a = sub(sc, sun);
            let b = sub(me, sun);
            match cosine(a, b) {
                Some(c) => println!(
                    "   cos( (sc-sun) , (me-sun) ) = {:.6}  (angle {:.2} deg)",
                    c,
                    c.clamp(-1.0, 1.0).acos().to_degrees()
                ),
                None => println!("   cos: a or b is zero"),
            }
            println!(
                "   |sc-me| vs |sc|+|me| : {:.1} vs {:.1} km",
                norm(sub(sc, me)) / 1000.0,
                (norm(sc) + norm(me)) / 1000.0
            );
            let cos_sc_me = match cosine(sc, me) {
                Some(c) => format!("{c:.6}"),
                None => "absent".into(),
            };
            println!(
                "   cos(sc, me) = {} | sign-corrected |(-sc)-me| = {:.1} km",
                cos_sc_me,
                norm(sub(neg3(sc), me)) / 1000.0
            );
        }
    }

    println!();
    println!("--- M1 closest-approach scan JD 2442135.0..2442138.0 (step 0.001 d) ---");
    let mut raw_best: Option<(f64, f64)> = None;
    let mut cor_best: Option<(f64, f64)> = None;
    let mut jd = 2442135.0_f64;
    while jd <= 2442138.0 {
        let t = tdb_of_jd(jd);
        if let (Some(sc), Some(me)) = (
            body_barycenter_position(&names[0], t, &eph),
            body_barycenter_position(&names[1], t, &eph),
        ) {
            let raw = norm(sub(sc, me)) / 1000.0;
            let cor = norm(sub(neg3(sc), me)) / 1000.0;
            if raw_best.is_none_or(|b| raw < b.0) {
                raw_best = Some((raw, jd));
            }
            if cor_best.is_none_or(|b| cor < b.0) {
                cor_best = Some((cor, jd));
            }
        }
        jd += 0.001;
    }
    match raw_best {
        Some((km, at)) => println!("raw        min |sc-me|   = {km:.1} km at JD {at:.4}"),
        None => println!("raw        min |sc-me|   = absent (no common frame reading)"),
    }
    match cor_best {
        Some((km, at)) => println!("sign-fixed min |(-sc)-me| = {km:.1} km at JD {at:.4}"),
        None => println!("sign-fixed min |(-sc)-me| = absent (no common frame reading)"),
    }
    println!("(stated M1 closest approach: JD 2442137.366; historical ~3e3 km)");

    let Some(kernel) = arg_value(&args, "--kernel") else {
        println!();
        println!("(no --kernel given; SPK cross-check skipped)");
        return;
    };
    let spk = match SpkFile::open(&kernel) {
        Ok(s) => s,
        Err(e) => {
            println!();
            println!("--kernel {kernel}: SpkFile::open reads void: {e:?}");
            return;
        }
    };
    println!();
    println!("--- SPK ground truth from {kernel} (km, native frame) ---");
    for jd in [2442135.305_f64, 2442136.366, 2442137.370] {
        let t = tdb_of_jd(jd);
        let m10_wrt_sun = spk.state(-76, 10, t).ok();
        let sun_wrt_ssb = spk.state(10, 0, t).ok();
        let mer_wrt_ssb = spk.state(1, 0, t).ok();
        let m10_wrt_mer = spk.state(-76, 1, t).ok();
        let true_m10 = match (m10_wrt_sun, sun_wrt_ssb) {
            (Some(a), Some(b)) => Some([a[0] + b[0], a[2] + b[2], a[4] + b[4]]),
            _ => None,
        };
        let sc = body_barycenter_position(&names[0], t, &eph);
        let true_mag = match true_m10 {
            Some(v) => format!("{:.1} km", norm(v) / 1000.0),
            None => "absent".into(),
        };
        println!(
            "JD {jd:.4}  true M10-SSB {} | true |M10| {}",
            true_m10.map(koord).unwrap_or("absent".into()),
            true_mag
        );
        println!(
            "            Mercury-SSB {} | M10-Mercury {}",
            mer_wrt_ssb.map(koord6).unwrap_or("absent".into()),
            m10_wrt_mer.map(koord6).unwrap_or("absent".into())
        );
        if let (Some(true_m10), Some(sc)) = (true_m10, sc) {
            let c = cosine(sc, true_m10);
            println!(
                "            bin sc {} | cos(bin, true M10-SSB) = {}",
                koord(sc),
                c.map(|c| format!("{c:.4}")).unwrap_or("n/a".into())
            );
        }
    }
}
