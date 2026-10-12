use std::collections::HashMap;

use omegaflow::archivar::camera_angles::{CameraAngleFrame, parse_camera_angles};
use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, fetch_raw_bytes, parse_ephemeris_binary,
};
use omegaflow::cdn::body_url;

const DEFAULT_ANGLES: &str = "state/research/m10-camera-angles/camera_angles.txt";
const J2000_EPOCH: f64 = 2451545.0;

const M10_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_mariner10.bin";
const MERCURY_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_mercury.bin";
const MERCURY_EPH_ALT: &str = "data/ftp.imcce.fr/ephemeris_inpop_mercury.bin";
const EARTH_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_earth.bin";

const CLUSTER_GAP_D: f64 = 10.0;
const IDENT_TOL_KM: f64 = 50.0;
const M1_CA_REF_JD: f64 = 2442137.37;

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

fn tdb_of_jd(jd: f64) -> f64 {
    (jd - J2000_EPOCH) * 86400.0
}

fn km(p: [f64; 3]) -> [f64; 3] {
    [p[0] / 1000.0, p[1] / 1000.0, p[2] / 1000.0]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn neg(v: [f64; 3]) -> [f64; 3] {
    [-v[0], -v[1], -v[2]]
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

struct Acc {
    sq: f64,
    n: usize,
}

impl Acc {
    fn new() -> Self {
        Acc { sq: 0.0, n: 0 }
    }

    fn push(&mut self, d: [f64; 3]) {
        self.sq += d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
        self.n += 1;
    }

    fn rms(&self) -> f64 {
        if self.n == 0 {
            f64::NAN
        } else {
            (self.sq / self.n as f64).sqrt()
        }
    }
}

const NAMES: [&str; 18] = [
    "a  sc - me",
    "b  me - sc",
    "c  sc (ssb)",
    "d  sc - earth",
    "e  sign(+,-,-) sc-me",
    "e  sign(-,+,-) sc-me",
    "e  sign(-,-,+) sc-me",
    "e  sign(-,+,+) sc-me",
    "e  sign(+,-,+) sc-me",
    "e  sign(+,+,-) sc-me",
    "e  sign(+,+,+) sc-me",
    "e  sign(-,-,-) sc-me",
    "f  -sc - me",
    "g  -sc + me",
    "h  sc + me",
    "i  -sc (ssb)",
    "j  -sc - earth",
    "k  -sc + earth",
];

const E_SIGNS: [[f64; 3]; 8] = [
    [1.0, -1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [1.0, -1.0, 1.0],
    [1.0, 1.0, -1.0],
    [1.0, 1.0, 1.0],
    [-1.0, -1.0, -1.0],
];

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

fn date_of_jd(jd: f64) -> String {
    let unix = (jd - 2440587.5) * 86400.0;
    let days = (unix / 86400.0).floor() as i64;
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn jd_of_utc(y: i64, mo: u32, d: u32, hh: u32, mm: u32) -> f64 {
    let a = (14 - mo as i64) / 12;
    let y2 = y + 4800 - a;
    let m2 = mo as i64 + 12 * a - 3;
    let jdn = d as i64 + (153 * m2 + 2) / 5 + 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 - 32045;
    jdn as f64 - 0.5 + (hh as f64 + mm as f64 / 60.0) / 24.0
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: m10_camera_angles_frame [--angles <path>] [--m10-eph <path>] [--mercury-eph <path>]"
        );
        std::process::exit(2);
    }
    let angles_path = match arg_value(&args, "--angles") {
        Some(p) => p,
        None => DEFAULT_ANGLES.to_string(),
    };
    let m10_explicit = arg_value(&args, "--m10-eph");
    let mercury_explicit = arg_value(&args, "--mercury-eph");

    let text = match std::fs::read_to_string(&angles_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("m10_camera_angles_frame: {angles_path} reads void: {e}");
            std::process::exit(1);
        }
    };
    let mut frames = parse_camera_angles(&text);
    if frames.is_empty() {
        eprintln!("m10_camera_angles_frame: no camera-angle records in {angles_path} — 0 honored");
        std::process::exit(1);
    }
    frames.sort_by(|a, b| a.jd.total_cmp(&b.jd));

    let m10_key = eph_key(M10_EPH);
    let mercury_key = eph_key(MERCURY_EPH);
    let earth_key = eph_key(EARTH_EPH);

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    if let Some(e) = load_named(&m10_key, m10_explicit.as_deref(), &[M10_EPH]) {
        eph.insert(m10_key.clone(), e);
    }
    if let Some(e) = load_named(
        &mercury_key,
        mercury_explicit.as_deref(),
        &[MERCURY_EPH, MERCURY_EPH_ALT],
    ) {
        eph.insert(mercury_key.clone(), e);
    }
    if let Some(e) = load_named(&earth_key, None, &[EARTH_EPH]) {
        eph.insert(earth_key.clone(), e);
    }

    for (name, e) in &eph {
        match coverage_jd(e) {
            Some((lo, hi)) => {
                eprintln!("{name}: {} granules, JD {lo:.3}..{hi:.3}", e.granules.len())
            }
            None => eprintln!("{name}: 0 granules — the ephemeris carries no position"),
        }
    }

    println!("=== M10 camera-angle SX SY SZ frame identification ===");
    println!("angles: {angles_path}");
    println!("records parsed: {}", frames.len());
    println!(
        "jd span: {:.6} ({}) .. {:.6} ({})",
        frames[0].jd,
        date_of_jd(frames[0].jd),
        frames[frames.len() - 1].jd,
        date_of_jd(frames[frames.len() - 1].jd)
    );

    let Some((m10_lo, m10_hi)) = eph.get(&m10_key).and_then(coverage_jd) else {
        println!();
        println!("M10 ephemeris absent — the S frame stays unmeasured");
        return;
    };
    println!("M10 ephemeris coverage: JD {m10_lo:.4}..{m10_hi:.4}");

    let m1: Vec<&CameraAngleFrame> = frames
        .iter()
        .filter(|f| f.jd >= m10_lo && f.jd <= m10_hi)
        .collect();
    println!(
        "M1 frames inside M10 coverage: {} (of {} total)",
        m1.len(),
        frames.len()
    );

    if m1.is_empty() {
        println!("no frame falls inside the M10 ephemeris coverage — the frame stays unmeasured");
        return;
    }

    println!();
    println!("--- frame-center diagnostic (vectors km, J2000/SSB via archivar) ---");
    for probe_jd in [
        2442135.305_f64,
        2442136.366,
        2442136.3778,
        2442137.606,
        2442141.7505,
    ] {
        let tdb = tdb_of_jd(probe_jd);
        let sc = body_barycenter_position(&m10_key, tdb, &eph).map(km);
        let me = body_barycenter_position(&mercury_key, tdb, &eph).map(km);
        let ea = body_barycenter_position(&earth_key, tdb, &eph).map(km);
        let f = |v: Option<[f64; 3]>| match v {
            Some(v) => format!("|r|={:>14.1} km", norm(v)),
            None => "reading absent".to_string(),
        };
        let sm = match (sc, me) {
            (Some(sc), Some(me)) => format!("|sc-me|={:>14.1} km", norm(sub(sc, me))),
            _ => "sc-me absent".to_string(),
        };
        let fm = match (sc, me) {
            (Some(sc), Some(me)) => format!("|f=-sc-me|={:>12.1} km", norm(sub(neg(sc), me))),
            _ => "f absent".to_string(),
        };
        let hm = match (sc, me) {
            (Some(sc), Some(me)) => format!("|h=sc+me|={:>12.1} km", norm(sub(sc, neg(me)))),
            _ => "h absent".to_string(),
        };
        println!(
            "JD {probe_jd:.4} | sc {} | me {} | earth {} | {} | {} | {}",
            f(sc),
            f(me),
            f(ea),
            sm,
            fm,
            hm
        );
    }

    let mut accs: Vec<Acc> = (0..NAMES.len()).map(|_| Acc::new()).collect();
    let mut s_mag_sum = 0.0;
    let mut s_mag_max = 0.0f64;
    let mut used = 0usize;

    for f in &m1 {
        let tdb = tdb_of_jd(f.jd);
        let Some(sc) = body_barycenter_position(&m10_key, tdb, &eph).map(km) else {
            continue;
        };
        let me = body_barycenter_position(&mercury_key, tdb, &eph).map(km);
        let earth = body_barycenter_position(&earth_key, tdb, &eph).map(km);
        let s = [f.sx, f.sy, f.sz];
        s_mag_sum += norm(s);
        s_mag_max = s_mag_max.max(norm(s));

        accs[2].push(sub(sc, s));

        if let Some(me) = me {
            let sc_me = sub(sc, me);
            accs[0].push(sub(sc_me, s));
            accs[1].push(sub(neg(sc_me), s));
            for (k, sg) in E_SIGNS.iter().enumerate() {
                let p = [sc_me[0] * sg[0], sc_me[1] * sg[1], sc_me[2] * sg[2]];
                accs[4 + k].push(sub(p, s));
            }
            accs[12].push(sub(sub(neg(sc), me), s));
            accs[13].push(sub(sub(neg(sc), neg(me)), s));
            accs[14].push(sub(sub(sc, neg(me)), s));
        }
        accs[15].push(sub(neg(sc), s));
        if let Some(earth) = earth {
            accs[3].push(sub(sub(sc, earth), s));
            accs[16].push(sub(sub(neg(sc), earth), s));
            accs[17].push(sub(sub(neg(sc), neg(earth)), s));
        }
        used += 1;
    }

    println!();
    println!(
        "S magnitude over M1: mean {:.1} km, max {:.1} km",
        s_mag_sum / used as f64,
        s_mag_max
    );
    println!("frames with M10 reading: {used}");
    println!();
    println!("{:<28} {:>14} {:>14}", "candidate", "RMS_3D_km", "frames");
    let mut best = 0usize;
    for (i, a) in accs.iter().enumerate() {
        if a.n == 0 {
            continue;
        }
        println!("{:<28} {:>14.3} {:>14}", NAMES[i], a.rms(), a.n);
        if accs[best].n == 0 || a.rms() < accs[best].rms() {
            best = i;
        }
    }

    println!();
    let best_rms = accs[best].rms();
    let mean_mag = s_mag_sum / used as f64;
    if best_rms.is_finite() && best_rms <= IDENT_TOL_KM {
        println!(
            "WINNER: {}  RMS {:.3} km (<= {:.1} km tolerance; {:.4}% of mean |S|)",
            NAMES[best],
            best_rms,
            IDENT_TOL_KM,
            100.0 * best_rms / mean_mag
        );
        println!("FRAME: IDENTIFIED — origin/frame = {}", NAMES[best]);
    } else {
        println!(
            "best candidate {} RMS {:.1} km ({:.2}% of mean |S|) — no candidate within {:.1} km",
            NAMES[best],
            best_rms,
            100.0 * best_rms / mean_mag,
            IDENT_TOL_KM
        );
        println!("FRAME: UNIDENTIFIED — the residual table stands, no origin matches");
    }

    let mut cos_sum = 0.0;
    let mut ratio_sum = 0.0;
    let mut cos_n = 0usize;
    for f in &m1 {
        let tdb = tdb_of_jd(f.jd);
        let (Some(sc), Some(me)) = (
            body_barycenter_position(&m10_key, tdb, &eph).map(km),
            body_barycenter_position(&mercury_key, tdb, &eph).map(km),
        ) else {
            continue;
        };
        let s = [f.sx, f.sy, f.sz];
        let cand = sub(neg(sc), me);
        let (cn, sn) = (norm(cand), norm(s));
        if cn > 0.0 && sn > 0.0 {
            cos_sum += (cand[0] * s[0] + cand[1] * s[1] + cand[2] * s[2]) / (cn * sn);
            ratio_sum += cn / sn;
            cos_n += 1;
        }
    }
    if cos_n > 0 {
        let mean_cos = cos_sum / cos_n as f64;
        println!(
            "leading-candidate relation to S: mean cos(angle) {mean_cos:.4} (~{:.1} deg), mean |cand|/|S| {:.4}",
            mean_cos.clamp(-1.0, 1.0).acos().to_degrees(),
            ratio_sum / cos_n as f64
        );
    }

    let mut clusters: Vec<Vec<&CameraAngleFrame>> = Vec::new();
    let mut cur: Vec<&CameraAngleFrame> = Vec::new();
    for f in &frames {
        if let Some(prev) = cur.last() {
            if f.jd - prev.jd > CLUSTER_GAP_D {
                clusters.push(std::mem::take(&mut cur));
            }
        }
        cur.push(f);
    }
    if !cur.is_empty() {
        clusters.push(cur);
    }

    println!();
    println!("=== per-encounter |S| minimum ===");
    println!(
        "{:<6} {:>6} {:>22} {:>14} {:>14} {:>12}",
        "enc", "frames", "jd range", "min|S|_km", "at jd", "fds"
    );
    for (i, cl) in clusters.iter().enumerate() {
        let jd_lo = cl[0].jd;
        let jd_hi = cl[cl.len() - 1].jd;
        let mut min_mag = f64::INFINITY;
        let mut min_jd = f64::NAN;
        let mut min_fds = String::new();
        for f in cl {
            let m = norm([f.sx, f.sy, f.sz]);
            if m < min_mag {
                min_mag = m;
                min_jd = f.jd;
                min_fds = f.fds.clone();
            }
        }
        println!(
            "{:<6} {:>6} {:>22} {:>14.3} {:>14.6} {:>12}",
            format!("M{}", i + 1),
            cl.len(),
            format!("{jd_lo:.3}..{jd_hi:.3}"),
            min_mag,
            min_jd,
            min_fds
        );
    }

    let mut ca_jd = f64::NAN;
    let mut ca_dist = f64::INFINITY;
    let mut fmin_jd = f64::NAN;
    let mut fmin_dist = f64::INFINITY;
    let step = 0.002;
    let mut jd = m10_lo;
    while jd <= m10_hi {
        let tdb = tdb_of_jd(jd);
        if let (Some(sc), Some(me)) = (
            body_barycenter_position(&m10_key, tdb, &eph),
            body_barycenter_position(&mercury_key, tdb, &eph),
        ) {
            let d = norm(sub(sc, me)) / 1000.0;
            if d < ca_dist {
                ca_dist = d;
                ca_jd = jd;
            }
            let dsigned = norm(sub(neg(sc), me)) / 1000.0;
            if dsigned < fmin_dist {
                fmin_dist = dsigned;
                fmin_jd = jd;
            }
        }
        jd += step;
    }

    let date_ca_jd = jd_of_utc(1974, 3, 29, 20, 47);
    let min_s_jd = clusters
        .first()
        .and_then(|cl| {
            cl.iter()
                .map(|f| (f.jd, norm([f.sx, f.sy, f.sz]), f.fds.clone()))
                .min_by(|a, b| a.1.total_cmp(&b.1))
        })
        .map(|(jd, _, fds)| (jd, fds));

    println!();
    println!("=== closest-approach check (M1) ===");
    if ca_jd.is_finite() {
        println!(
            "ephemeris min |sc-me| over M10 coverage: JD {ca_jd:.4}, {ca_dist:.1} km — NOT usable as CA (the compiled M10 and Mercury ephemerides are mutually inconsistent)"
        );
        println!("ephemeris min |(-sc)-me| (sign-corrected): JD {fmin_jd:.4}, {fmin_dist:.1} km");
    } else {
        println!("ephemeris closest approach stays unmeasured — no coincident M10/Mercury reading");
    }
    println!("date-derived M1 closest approach: 1974-03-29 20:47 UT = JD {date_ca_jd:.4}");
    println!("task-stated reference: JD {M1_CA_REF_JD:.4} (a 1.004 d slip vs the UT date above)");
    if let Some((jd, fds)) = min_s_jd {
        println!(
            "min |S| frame of M1: JD {jd:.4} FDS {fds} — offset {:+.4} d from the date-derived CA, {:+.4} d from the stated reference",
            jd - date_ca_jd,
            jd - M1_CA_REF_JD
        );
        if (jd - date_ca_jd).abs() < 0.05 {
            println!(
                "min |S| COINCIDES with the date-derived closest approach (1974-03-29 20:47 UT)"
            );
        } else {
            println!("min |S| does NOT coincide with the date-derived closest approach");
        }
    }
}
