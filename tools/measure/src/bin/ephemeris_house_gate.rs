use omegaflow::archivar::{
    BodyEphemeris, J2000_EPOCH, NAIF_LSK_EMBEDDED, body_barycenter_position, parse_ephemeris_binary,
};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use std::collections::HashMap;

const DAY: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const PERIGEE_UNIX_HMS_S: f64 = 11.0 * HOUR + 45.0 * 60.0 + 12.0;

const DEFAULT_DE: &str = "data/ssd.jpl.nasa.gov-de/ephemeris_de441_earth.bin";
const DEFAULT_INPOP: &str = "data/ftp.imcce.fr/ephemeris_inpop_earth.bin";
const DEFAULT_EPM: &str = "data/ftp.iaaras.ru/ephemeris_epm_earth.bin";
const DEFAULT_REGISTER: &str = "data/flyby2/house-gate-2026-09-28.json";

fn arg_str(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_or(args: &[String], flag: &str, default: &str) -> String {
    match arg_str(args, flag) {
        Some(v) => v,
        None => default.to_string(),
    }
}

fn parse_ymd(s: &str) -> Option<(i64, i64, i64)> {
    let p: Vec<&str> = s.split('-').collect();
    if p.len() != 3 {
        return None;
    }
    Some((p[0].parse().ok()?, p[1].parse().ok()?, p[2].parse().ok()?))
}

fn parse_hms(s: &str) -> Option<f64> {
    let p: Vec<&str> = s.split(':').collect();
    if p.len() != 3 {
        return None;
    }
    let h: f64 = p[0].parse().ok()?;
    let m: f64 = p[1].parse().ok()?;
    let sec: f64 = p[2].parse().ok()?;
    if !(0.0..24.0).contains(&h) || !(0.0..60.0).contains(&m) || !(0.0..61.0).contains(&sec) {
        return None;
    }
    Some(h * HOUR + m * 60.0 + sec)
}

fn load_house(word: &str, path: &str, body: &str) -> Option<HashMap<String, BodyEphemeris>> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            println!("{word}: bin absent ({path}) — the line stays pending");
            return None;
        }
    };
    let e = match parse_ephemeris_binary(&bytes) {
        Some(e) => e,
        None => {
            println!("{word}: bin reads but parses void ({path})");
            return None;
        }
    };
    let mut map = HashMap::new();
    map.insert(body.to_string(), e);
    Some(map)
}

fn coverage_seconds(e: &BodyEphemeris) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &e.granules {
        lo = lo.min((g.t0_jd - g.dt_jd - J2000_EPOCH) * DAY);
        hi = hi.max((g.t0_jd + g.dt_jd - J2000_EPOCH) * DAY);
    }
    if lo.is_finite() && hi.is_finite() && hi > lo {
        Some((lo, hi))
    } else {
        None
    }
}

fn compute_window(
    houses: &[&Option<HashMap<String, BodyEphemeris>>],
    nominal: Option<(f64, f64)>,
) -> Option<(f64, f64)> {
    let (lo, hi) = nominal?;
    let mut lo = lo;
    let mut hi = hi;
    for house in houses {
        for e in house.iter().flat_map(|m| m.values()) {
            match coverage_seconds(e) {
                Some((clo, chi)) => {
                    lo = lo.max(clo);
                    hi = hi.min(chi);
                }
                None => return None,
            }
        }
    }
    if hi > lo { Some((lo, hi)) } else { None }
}

fn max_dev_km(
    a: &HashMap<String, BodyEphemeris>,
    b: &HashMap<String, BodyEphemeris>,
    body: &str,
    t_lo: f64,
    t_hi: f64,
    step: f64,
) -> (Option<f64>, usize) {
    let n = ((t_hi - t_lo) / step).floor() as usize + 1;
    let mut max = 0.0f64;
    let mut void = 0usize;
    for i in 0..n {
        let t = t_lo + i as f64 * step;
        let pa = match body_barycenter_position(body, t, a) {
            Some(p) => p,
            None => {
                void += 1;
                continue;
            }
        };
        let pb = match body_barycenter_position(body, t, b) {
            Some(p) => p,
            None => {
                void += 1;
                continue;
            }
        };
        let dx = pa[0] - pb[0];
        let dy = pa[1] - pb[1];
        let dz = pa[2] - pb[2];
        let d = (dx * dx + dy * dy + dz * dz).sqrt() / 1000.0;
        if d.is_finite() {
            if d > max {
                max = d;
            }
        } else {
            void += 1;
        }
    }
    let max = if void > 0 { None } else { Some(max) };
    (max, void)
}

fn pair_dev(
    a: &Option<HashMap<String, BodyEphemeris>>,
    b: &Option<HashMap<String, BodyEphemeris>>,
    body: &str,
    window: Option<(f64, f64)>,
    step: f64,
) -> (Option<f64>, usize) {
    match (a, b, window) {
        (Some(ma), Some(mb), Some((lo, hi))) => max_dev_km(ma, mb, body, lo, hi, step),
        _ => (None, 0),
    }
}

fn diff_vec_km(
    a: &HashMap<String, BodyEphemeris>,
    b: &HashMap<String, BodyEphemeris>,
    body: &str,
    t: f64,
) -> Option<[f64; 3]> {
    let pa = body_barycenter_position(body, t, a)?;
    let pb = body_barycenter_position(body, t, b)?;
    Some([
        (pa[0] - pb[0]) / 1000.0,
        (pa[1] - pb[1]) / 1000.0,
        (pa[2] - pb[2]) / 1000.0,
    ])
}

fn num_label(v: Option<f64>) -> String {
    match v {
        None => "pending".to_string(),
        Some(x) => format!("{x}"),
    }
}

fn num_json(v: Option<f64>) -> String {
    match v {
        None => "\"pending\"".to_string(),
        Some(x) => format!("{x}"),
    }
}

fn register_json(
    body: &str,
    epoch: &str,
    window_days: f64,
    step_hours: f64,
    perigee_tdb: Option<f64>,
    de_inpop: Option<f64>,
    de_epm: Option<f64>,
    inpop_epm: Option<f64>,
) -> String {
    let span = match (de_inpop, de_epm, inpop_epm) {
        (Some(a), Some(b), Some(c)) => Some(a.max(b).max(c)),
        _ => None,
    };
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"gate\": \"ephemeris_house_gate\",\n");
    s.push_str("  \"body\": \""); // body line below keeps the field order stable
    s.push_str(body);
    s.push_str("\",\n");
    s.push_str(&format!("  \"epoch\": \"{epoch}\",\n"));
    s.push_str(&format!("  \"perigee_tdb\": {},\n", num_json(perigee_tdb)));
    s.push_str(&format!("  \"window_days\": {},\n", window_days));
    s.push_str(&format!("  \"step_hours\": {},\n", step_hours));
    s.push_str(&format!(
        "  \"earth_delta_km\": {{\"de_inpop\": {}, \"de_epm\": {}, \"inpop_epm\": {}}},\n",
        num_json(de_inpop),
        num_json(de_epm),
        num_json(inpop_epm),
    ));
    s.push_str(&format!("  \"three_house_span_km\": {}\n", num_json(span)));
    s.push_str("}\n");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let de_path = arg_or(&args, "--de", DEFAULT_DE);
    let inpop_path = arg_or(&args, "--inpop", DEFAULT_INPOP);
    let epm_path = arg_or(&args, "--epm", DEFAULT_EPM);
    let register_path = arg_or(&args, "--register", DEFAULT_REGISTER);
    let body = arg_or(&args, "--body", "earth");

    let window_days: f64 = match arg_str(&args, "--window-days") {
        Some(s) => match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            Ok(_) => {
                eprintln!("ephemeris_house_gate: --window-days {s} fails the plausibility gate");
                std::process::exit(2);
            }
            Err(_) => {
                eprintln!("ephemeris_house_gate: --window-days {s} carries no f64");
                std::process::exit(2);
            }
        },
        None => 2.0,
    };
    let step_hours: f64 = match arg_str(&args, "--step-hours") {
        Some(s) => match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            Ok(_) => {
                eprintln!("ephemeris_house_gate: --step-hours {s} fails the plausibility gate");
                std::process::exit(2);
            }
            Err(_) => {
                eprintln!("ephemeris_house_gate: --step-hours {s} carries no f64");
                std::process::exit(2);
            }
        },
        None => 1.0,
    };

    let (epoch_y, epoch_m, epoch_d) = match arg_str(&args, "--epoch-ymd") {
        Some(s) => match parse_ymd(&s) {
            Some(v) => v,
            None => {
                eprintln!("ephemeris_house_gate: --epoch-ymd {s} carries no YYYY-MM-DD");
                std::process::exit(2);
            }
        },
        None => (2026, 9, 28),
    };
    let epoch_label = format!("{epoch_y:04}-{epoch_m:02}-{epoch_d:02}");
    let (epoch_hms_s, epoch_hms_note) = match arg_str(&args, "--epoch-hms") {
        Some(s) => match parse_hms(&s) {
            Some(v) => (v, String::new()),
            None => {
                eprintln!("ephemeris_house_gate: --epoch-hms {s} carries no HH:MM:SS");
                std::process::exit(2);
            }
        },
        None => (
            PERIGEE_UNIX_HMS_S,
            " (default JUICE perigee; pass --epoch-hms for another epoch)".to_string(),
        ),
    };
    let lsk = parse_lsk(NAIF_LSK_EMBEDDED);
    let perigee_unix =
        days_from_civil(epoch_y, epoch_m, epoch_d).map(|d| d as f64 * DAY + epoch_hms_s);
    let perigee_tdb = match (&lsk, perigee_unix) {
        (Some(l), Some(u)) => l.unix_to_tdb(u),
        _ => None,
    };

    let de = load_house("de", &de_path, &body);
    let inpop = load_house("inpop", &inpop_path, &body);
    let epm = load_house("epm", &epm_path, &body);

    let step = step_hours * HOUR;
    let nominal = perigee_tdb.map(|tp| (tp - window_days * DAY, tp + window_days * DAY));
    let window = compute_window(&[&de, &inpop, &epm], nominal);

    let (de_inpop, void_de_inpop) = pair_dev(&de, &inpop, &body, window, step);
    let (de_epm, void_de_epm) = pair_dev(&de, &epm, &body, window, step);
    let (inpop_epm, void_inpop_epm) = pair_dev(&inpop, &epm, &body, window, step);

    println!(
        "ephemeris_house_gate — {body} barycenter across DE / INPOP / EPM, epoch {epoch_label}"
    );
    println!("epoch time: {epoch_hms_s} s after 00:00 UTC{epoch_hms_note}");
    match perigee_tdb {
        Some(t) => println!("perigee tdb: {t}"),
        None => println!("perigee tdb: pending — the leap table carries no value"),
    }
    match window {
        Some((lo, hi)) => println!(
            "window: ±{window_days} d nominal, common arc {lo}..{hi} tdb s, step {step_hours} h"
        ),
        None => println!("window: pending — the three lines carry no common arc"),
    }
    println!("Δ DE vs INPOP ({body}): {} km", num_label(de_inpop));
    println!("Δ DE vs EPM ({body}): {} km", num_label(de_epm));
    println!("Δ INPOP vs EPM ({body}): {} km", num_label(inpop_epm));
    for (word, void) in [
        ("Δ DE vs INPOP", void_de_inpop),
        ("Δ DE vs EPM", void_de_epm),
        ("Δ INPOP vs EPM", void_inpop_epm),
    ] {
        if void > 0 {
            println!("{word}: {void} void samples — the max stays pending");
        }
    }

    if let Some(t) = perigee_tdb {
        for (w, a, b) in [
            ("INPOP-EPM", &inpop, &epm),
            ("DE-EPM", &de, &epm),
            ("DE-INPOP", &de, &inpop),
        ] {
            if let (Some(ma), Some(mb)) = (a, b) {
                if let Some(v) = diff_vec_km(ma, mb, &body, t) {
                    println!(
                        "vec {w} ({body}) at perigee: {:.4} {:.4} {:.4} km",
                        v[0], v[1], v[2]
                    );
                }
            }
        }
    }

    let json = register_json(
        &body,
        &epoch_label,
        window_days,
        step_hours,
        perigee_tdb,
        de_inpop,
        de_epm,
        inpop_epm,
    );
    if let Some(parent) = std::path::Path::new(&register_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    match std::fs::write(&register_path, &json) {
        Ok(_) => println!("register: {register_path}"),
        Err(e) => {
            eprintln!("register write void ({register_path}): {e}");
            std::process::exit(2);
        }
    }
}
