use omegaflow::archivar::mariner_m10_edr;
use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, fetch_raw_bytes, parse_ephemeris_binary,
};
use omegaflow::cdn::body_url;
use std::collections::BTreeMap;
use std::collections::HashMap;

const EDR_URL: &str =
    "https://github.com/omegaflow/sources/releases/download/ser.im-ldi.com/mariner_m10_edr.bin";
const J2000_UNIX_OFFSET: f64 = 946728000.0;

const R_VENUS_M: f64 = 6_051_800.0;
const R_MERCURY_M: f64 = 2_439_700.0;

const FOV_NARROW_DEG: f64 = 0.42;
const FOV_WIDE_DEG: f64 = 0.98;

const VENUS_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_venus.bin";
const MERCURY_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_mercury.bin";
const MERCURY_EPH_ALT: &str = "data/ftp.imcce.fr/ephemeris_inpop_mercury.bin";
const M10_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_mariner10.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn eph_key(path: &str) -> String {
    let base = path.rsplit('/').next().unwrap_or(path);
    base.trim_start_matches("ephemeris_")
        .trim_end_matches(".bin")
        .to_string()
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (y + if m <= 2 { 1 } else { 0 }, m as u32, d as u32)
}

fn date_of(unix: f64) -> String {
    let days = (unix / 86400.0).floor() as i64;
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn angular_diameter_deg(radius_m: f64, range_m: f64) -> f64 {
    let ratio = (radius_m / range_m).clamp(-1.0, 1.0);
    2.0 * ratio.asin().to_degrees()
}

fn coverage_jd(e: &BodyEphemeris) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &e.granules {
        lo = lo.min(g.t0_jd - g.dt_jd);
        hi = hi.max(g.t0_jd + g.dt_jd);
    }
    if e.granules.is_empty() {
        None
    } else {
        Some((lo, hi))
    }
}

fn parse_file(path: &str) -> Option<BodyEphemeris> {
    match std::fs::read(path) {
        Ok(bytes) => match parse_ephemeris_binary(&bytes) {
            Some(e) => Some(e),
            None => {
                eprintln!("{path} carries no ephemeris contract");
                None
            }
        },
        Err(_) => {
            eprintln!("{path} reads void");
            None
        }
    }
}

fn load_named(name: &str, explicit: Option<&str>, candidates: &[&str]) -> Option<BodyEphemeris> {
    if let Some(path) = explicit {
        match parse_file(path) {
            Some(e) => {
                eprintln!("{name}: ephemeris {path} (explicit)");
                return Some(e);
            }
            None => eprintln!("{name}: explicit {path} carries no reading"),
        }
    }
    for cand in candidates {
        if std::path::Path::new(cand).exists() {
            if let Some(e) = parse_file(cand) {
                eprintln!("{name}: ephemeris {cand}");
                return Some(e);
            }
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
    let e = match parse_ephemeris_binary(&bytes) {
        Some(e) => e,
        None => {
            eprintln!("{name}: {url} carries no ephemeris contract");
            return None;
        }
    };
    if let Some(dest) = candidates.first() {
        if let Some(parent) = std::path::Path::new(dest).parent() {
            std::fs::create_dir_all(parent).ok();
        }
        if std::fs::write(dest, &bytes).is_ok() {
            eprintln!("{name}: ephemeris {url} -> {dest}");
        } else {
            eprintln!("{name}: {dest} write void — the reading stays in memory");
        }
    }
    Some(e)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: m10_expected_diameter [--in <mariner_m10_edr.bin>] [--m10-eph <ephemeris_mariner10.bin>]"
        );
        std::process::exit(2);
    }
    let in_path = arg_value(&args, "--in");
    let m10_explicit = arg_value(&args, "--m10-eph");
    let venus_key = eph_key(VENUS_EPH);
    let mercury_key = eph_key(MERCURY_EPH);
    let m10_key = eph_key(M10_EPH);

    let edr_bytes = match &in_path {
        Some(path) => match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!(
                    "m10_expected_diameter: {path} reads void: {e} — the EDR corpus stays unmeasured"
                );
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(EDR_URL) {
            Some(b) => b,
            None => {
                eprintln!(
                    "m10_expected_diameter: {EDR_URL} fetch void — the EDR corpus stays unmeasured"
                );
                std::process::exit(1);
            }
        },
    };
    let series = match mariner_m10_edr::parse_bin(&edr_bytes) {
        Some(s) => s,
        None => {
            eprintln!(
                "m10_expected_diameter: the bin carries no mariner_m10_edr contract — 0 honored"
            );
            std::process::exit(1);
        }
    };

    let mut epochs: Vec<f64> = Vec::new();
    for (epoch, _, _) in &series {
        if !epochs.iter().any(|e| e == epoch) {
            epochs.push(*epoch);
        }
    }
    epochs.sort_by(f64::total_cmp);

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    if let Some(e) = load_named(&venus_key, None, &[VENUS_EPH]) {
        eph.insert(venus_key.clone(), e);
    }
    if let Some(e) = load_named(&mercury_key, None, &[MERCURY_EPH, MERCURY_EPH_ALT]) {
        eph.insert(mercury_key.clone(), e);
    }
    if let Some(e) = load_named(&m10_key, m10_explicit.as_deref(), &[M10_EPH]) {
        eph.insert(m10_key.clone(), e);
    }

    for (name, e) in &eph {
        match coverage_jd(e) {
            Some((lo, hi)) => {
                eprintln!("{name}: {} granules, JD {lo:.3}..{hi:.3}", e.granules.len())
            }
            None => eprintln!("{name}: 0 granules — the ephemeris carries no position"),
        }
    }
    if let (Some(first), Some(last)) = (epochs.first(), epochs.last()) {
        eprintln!(
            "EDR epochs: {first:.0} ({}) .. {last:.0} ({}), {} unique",
            date_of(*first),
            date_of(*last),
            epochs.len()
        );
    }

    if !eph.contains_key(&m10_key) {
        eprintln!(
            "m10_expected_diameter: no mariner10 ephemeris reading — the range stays unmeasured"
        );
        return;
    }
    if !eph.contains_key(&venus_key) && !eph.contains_key(&mercury_key) {
        eprintln!(
            "m10_expected_diameter: neither venus nor mercury carries an ephemeris reading — the angular diameter stays unmeasured"
        );
        return;
    }

    println!(
        "=== M10 EDR angular diameter vs camera FOV (narrow {FOV_NARROW_DEG} deg, wide {FOV_WIDE_DEG} deg) ==="
    );
    println!(
        "unique frame epochs {} | positions meters (SPK km x 1000 in the kernels) | range printed in km",
        epochs.len()
    );

    let mut groups: BTreeMap<(String, String), (f64, f64, usize)> = BTreeMap::new();
    let mut measured = 0usize;
    let mut no_m10 = 0usize;
    for &epoch in &epochs {
        let date = date_of(epoch);
        let tdb = epoch - J2000_UNIX_OFFSET;
        let Some(r_s) = body_barycenter_position(&m10_key, tdb, &eph) else {
            no_m10 += 1;
            println!(
                "{epoch:>12.0} | {date} | mariner10 ephemeris carries no reading — epoch unmeasured"
            );
            continue;
        };
        let r_v = body_barycenter_position(&venus_key, tdb, &eph);
        let r_m = body_barycenter_position(&mercury_key, tdb, &eph);

        let mut candidates: Vec<(&str, f64, f64)> = Vec::new();
        if let Some(r) = r_v {
            let range = distance(r, r_s);
            if range > 0.0 {
                candidates.push((
                    venus_key.as_str(),
                    range,
                    angular_diameter_deg(R_VENUS_M, range),
                ));
            }
        }
        if let Some(r) = r_m {
            let range = distance(r, r_s);
            if range > 0.0 {
                candidates.push((
                    mercury_key.as_str(),
                    range,
                    angular_diameter_deg(R_MERCURY_M, range),
                ));
            }
        }
        if candidates.is_empty() {
            println!("{epoch:>12.0} | {date} | no venus/mercury reading — epoch unmeasured");
            continue;
        }
        let Some((body, range_m, ang)) = candidates.iter().min_by(|a, b| a.1.total_cmp(&b.1))
        else {
            continue;
        };
        let body = *body;
        let range_m = *range_m;
        let ang = *ang;

        let verdict = if ang < FOV_WIDE_DEG {
            "limb possible (fits 0.98)"
        } else {
            "no limb (ang_diam >= FOV)"
        };
        let narrow = if ang < FOV_NARROW_DEG {
            "fits 0.42"
        } else {
            "exceeds 0.42"
        };
        println!(
            "{epoch:>12.0} | {date} | {body:<7} | range_km {:>14.3} | ang_diam {ang:>12.6} deg | {verdict} | narrow {narrow}",
            range_m / 1000.0
        );

        let key = (date, body.to_string());
        let e = groups.entry(key).or_insert((ang, ang, 0));
        e.0 = e.0.min(ang);
        e.1 = e.1.max(ang);
        e.2 += 1;
        measured += 1;
    }

    println!();
    println!("=== per encounter (nearest body, date) — min/max angular diameter ===");
    println!(
        "{:<12} {:<8} {:>5} {:>14} {:>14}",
        "date", "body", "n", "min_deg", "max_deg"
    );
    for ((date, body), (mn, mx, n)) in &groups {
        println!("{date:<12} {body:<8} {n:>5} {mn:>14.6} {mx:>14.6}");
    }
    println!();
    println!(
        "epochs total {} | measured {} | mariner10-no-reading {}",
        epochs.len(),
        measured,
        no_m10
    );
}
