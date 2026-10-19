use omegaflow::archivar::{
    BodyEphemeris, J2000_EPOCH, NAIF_LSK_EMBEDDED, body_barycenter_position, parse_ephemeris_binary,
};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use omegaflow::sha256::sha256_hex;
use std::collections::HashMap;

const SEALED_SHA256: &str = "aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a";
const RENEWED_SHA256: &str = "eee376effcb4def668a61d47ab7ea2e6f7b0b7cc3f884ea6634997349d4389b5";
const PERIGEE_UTC: &str = "2026-09-28T11:45:12Z";

const SEALED_ASSET: &str = "ephemeris_juice_official.bin";
const DEFAULT_SOURCES: &str = "phi/sources.φ";

const DAY: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const PERIGEE_UNIX_HMS_S: f64 = 11.0 * HOUR + 45.0 * 60.0 + 12.0;

const DEFAULT_SEALED: &str = "sources/ssd.jpl.nasa.gov/ephemeris_juice.bin";
const DEFAULT_RENEWED: &str = "sources/ssd.jpl.nasa.gov/ephemeris_juice_renewed.bin";
const DEFAULT_RECON: &str = "sources/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin";
const DEFAULT_EARTH: &str = "sources/ssd.jpl.nasa.gov/ephemeris_earth.bin";
const DEFAULT_DE441_JUICE: &str = DEFAULT_SEALED;
const DEFAULT_DE441_EARTH: &str = "sources/ssd.jpl.nasa.gov-de/ephemeris_de441_earth.bin";
const DEFAULT_DE442_JUICE: &str = DEFAULT_SEALED;
const DEFAULT_DE442_EARTH: &str = "sources/ssd.jpl.nasa.gov-de/ephemeris_de442_earth.bin";
const DEFAULT_REGISTER: &str = "sources/flyby2/gate-juice-2026-09-28.json";

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

fn file_sha(path: &str) -> Option<String> {
    std::fs::read(path).ok().map(|b| sha256_hex(&b))
}

fn load_line(word: &str, juice_path: &str, earth_path: &str) -> Option<Line> {
    let juice_bytes = match std::fs::read(juice_path) {
        Ok(b) => b,
        Err(_) => {
            println!("{word}: juice bin absent ({juice_path}) — the line stays pending");
            return None;
        }
    };
    let earth_bytes = match std::fs::read(earth_path) {
        Ok(b) => b,
        Err(_) => {
            println!("{word}: earth bin absent ({earth_path}) — the line stays pending");
            return None;
        }
    };
    let juice = match parse_ephemeris_binary(&juice_bytes) {
        Some(e) => e,
        None => {
            println!("{word}: juice bin reads but parses void ({juice_path})");
            return None;
        }
    };
    let earth = match parse_ephemeris_binary(&earth_bytes) {
        Some(e) => e,
        None => {
            println!("{word}: earth bin reads but parses void ({earth_path})");
            return None;
        }
    };
    let mut map = HashMap::new();
    map.insert(body_key(juice_path)?, juice);
    map.insert(body_key(earth_path)?, earth);
    Some(Line {
        bodies: map,
        spacecraft: body_key(juice_path)?,
        receiver: body_key(earth_path)?,
    })
}

struct Line {
    bodies: HashMap<String, BodyEphemeris>,
    spacecraft: String,
    receiver: String,
}

fn body_key(path: &str) -> Option<String> {
    let stem = std::path::Path::new(path).file_stem()?.to_str()?;
    Some(stem.strip_prefix("ephemeris_").unwrap_or(stem).to_string())
}

fn geocentric(line: &Line, tdb: f64) -> Option<[f64; 3]> {
    let j = body_barycenter_position(&line.spacecraft, tdb, &line.bodies)?;
    let e = body_barycenter_position(&line.receiver, tdb, &line.bodies)?;
    Some([j[0] - e[0], j[1] - e[1], j[2] - e[2]])
}

fn coverage_seconds(map: &HashMap<String, BodyEphemeris>) -> Option<(f64, f64)> {
    let mut lo = f64::NEG_INFINITY;
    let mut hi = f64::INFINITY;
    let mut any = false;
    for e in map.values() {
        let mut blo = f64::INFINITY;
        let mut bhi = f64::NEG_INFINITY;
        for g in &e.granules {
            blo = blo.min((g.t0_jd - g.dt_jd - J2000_EPOCH) * DAY);
            bhi = bhi.max((g.t0_jd + g.dt_jd - J2000_EPOCH) * DAY);
        }
        if blo.is_finite() && bhi.is_finite() {
            lo = lo.max(blo);
            hi = hi.min(bhi);
            any = true;
        }
    }
    if any && lo.is_finite() && hi.is_finite() && hi > lo {
        Some((lo, hi))
    } else {
        None
    }
}

fn max_dev_km(a: &Line, b: &Line, t_lo: f64, t_hi: f64, step: f64) -> (Option<f64>, usize, usize) {
    let n = ((t_hi - t_lo) / step).floor() as usize + 1;
    let mut max = 0.0f64;
    let mut void = 0usize;
    for i in 0..n {
        let t = t_lo + i as f64 * step;
        let pa = match geocentric(a, t) {
            Some(p) => p,
            None => {
                void += 1;
                continue;
            }
        };
        let pb = match geocentric(b, t) {
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
    (max, n, void)
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

fn word_label(v: Option<bool>) -> String {
    match v {
        None => "pending".to_string(),
        Some(true) => "stands".to_string(),
        Some(false) => "falsified".to_string(),
    }
}

fn sha_label(v: Option<&str>) -> String {
    match v {
        None => "pending".to_string(),
        Some(h) => h.to_string(),
    }
}

fn seal_word(measured: Option<&str>, expected: Option<&str>) -> &'static str {
    match (measured, expected) {
        (None, _) => "pending",
        (Some(_), None) => "measured",
        (Some(m), Some(e)) if m == e => "placed",
        (Some(_), Some(_)) => "riss",
    }
}

fn register_pin(content: &str, asset: &str) -> Option<String> {
    omegaflow::archivar::parse_sources(content)
        .into_iter()
        .find(|s| s.url.ends_with(asset))
        .and_then(|s| s.sha256)
}

fn anchor_word(pin: Option<&str>, constant: &str) -> &'static str {
    match pin {
        None => "pending",
        Some(p) if p == constant => "placed",
        Some(_) => "riss",
    }
}

fn pending_or_word(v: Option<bool>) -> String {
    match v {
        None => "\"pending\"".to_string(),
        Some(true) => "\"stands\"".to_string(),
        Some(false) => "\"falsified\"".to_string(),
    }
}

fn sha_json(v: Option<&str>) -> String {
    match v {
        None => "\"pending\"".to_string(),
        Some(h) => format!("\"{h}\""),
    }
}

fn f64_json(v: f64) -> String {
    format!("{v}")
}

#[allow(clippy::too_many_arguments)]
fn register_json(
    window_days: f64,
    step_hours: f64,
    sealed_sha: Option<&str>,
    sealed_pin: Option<&str>,
    anchor: &str,
    renewed_sha: Option<&str>,
    recon_sha: Option<&str>,
    delta_sealed: Option<f64>,
    delta_renewed: Option<f64>,
    edition_rift: Option<f64>,
    sigma_recon: Option<f64>,
    threshold: Option<f64>,
    verdict_sealed: Option<bool>,
    verdict_renewed: Option<bool>,
    riss: bool,
) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"gate\": \"flyby_ephemeris_gate\",\n");
    s.push_str("  \"flyby\": \"juice\",\n");
    s.push_str(&format!("  \"perigee_utc\": \"{PERIGEE_UTC}\",\n"));
    s.push_str(&format!("  \"window_days\": {},\n", f64_json(window_days)));
    s.push_str(&format!("  \"step_hours\": {},\n", f64_json(step_hours)));
    s.push_str(&format!(
        "  \"trajectory\": {{\"sealed_sha256\": \"{SEALED_SHA256}\", \"sealed_measured\": {}, \"renewed_sha256\": \"{RENEWED_SHA256}\", \"renewed_measured\": {}, \"recon_sha256\": {}}},\n",
        sha_json(sealed_sha),
        sha_json(renewed_sha),
        sha_json(recon_sha),
    ));
    s.push_str(&format!(
        "  \"anchor\": {{\"sealed\": \"{anchor}\", \"sealed_register_sha256\": {}}},\n",
        sha_json(sealed_pin),
    ));
    s.push_str(&format!(
        "  \"delta_km\": {{\"sealed\": {}, \"renewed\": {}}},\n",
        num_json(delta_sealed),
        num_json(delta_renewed),
    ));
    s.push_str(&format!(
        "  \"edition_rift_km\": {},\n",
        num_json(edition_rift)
    ));
    s.push_str(&format!(
        "  \"sigma_recon_km\": {},\n",
        num_json(sigma_recon)
    ));
    s.push_str(&format!("  \"threshold_km\": {},\n", num_json(threshold)));
    s.push_str(&format!(
        "  \"verdict\": {{\"sealed\": {}, \"renewed\": {}}},\n",
        pending_or_word(verdict_sealed),
        pending_or_word(verdict_renewed),
    ));
    s.push_str(&format!("  \"riss\": {riss}\n"));
    s.push_str("}\n");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sealed_path = arg_or(&args, "--sealed", DEFAULT_SEALED);
    let renewed_path = arg_or(&args, "--renewed", DEFAULT_RENEWED);
    let recon_path = arg_or(&args, "--recon", DEFAULT_RECON);
    let earth_path = arg_or(&args, "--earth", DEFAULT_EARTH);
    let de441_juice = arg_or(&args, "--de441-juice", DEFAULT_DE441_JUICE);
    let de441_earth = arg_or(&args, "--de441-earth", DEFAULT_DE441_EARTH);
    let de442_juice = arg_or(&args, "--de442-juice", DEFAULT_DE442_JUICE);
    let de442_earth = arg_or(&args, "--de442-earth", DEFAULT_DE442_EARTH);
    let register_path = arg_or(&args, "--register", DEFAULT_REGISTER);
    let sources_path = arg_or(&args, "--sources", DEFAULT_SOURCES);

    let sealed_pin = std::fs::read_to_string(&sources_path)
        .ok()
        .and_then(|content| register_pin(&content, SEALED_ASSET));
    let anchor = anchor_word(sealed_pin.as_deref(), SEALED_SHA256);
    let seal_expected = sealed_pin.as_deref().unwrap_or(SEALED_SHA256);

    let window_days: f64 = match arg_str(&args, "--window-days") {
        None => 21.0,
        Some(s) => match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                eprintln!("flyby_ephemeris_gate: --window-days {s} carries no positive finite f64");
                std::process::exit(2);
            }
        },
    };
    let step_hours: f64 = match arg_str(&args, "--step-hours") {
        None => 1.0,
        Some(s) => match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                eprintln!("flyby_ephemeris_gate: --step-hours {s} carries no positive finite f64");
                std::process::exit(2);
            }
        },
    };
    let sigma_recon: Option<f64> = match arg_str(&args, "--sigma-recon") {
        None => None,
        Some(s) => match s.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => Some(v),
            Ok(v) => {
                eprintln!(
                    "flyby_ephemeris_gate: --sigma-recon {v} fails the plausibility gate — sigma_recon pending"
                );
                None
            }
            Err(_) => {
                eprintln!("flyby_ephemeris_gate: --sigma-recon {s} carries no f64");
                std::process::exit(2);
            }
        },
    };

    let lsk = parse_lsk(NAIF_LSK_EMBEDDED);
    let perigee_unix = days_from_civil(2026, 9, 28).map(|d| d as f64 * DAY + PERIGEE_UNIX_HMS_S);
    let perigee_tdb = match (&lsk, perigee_unix) {
        (Some(l), Some(u)) => l.unix_to_tdb(u),
        _ => None,
    };
    let step = step_hours * HOUR;
    let nominal_window = perigee_tdb.map(|tp| (tp - window_days * DAY, tp + window_days * DAY));

    let sealed = load_line("sealed", &sealed_path, &earth_path);
    let renewed = load_line("renewed", &renewed_path, &earth_path);
    let recon = load_line("recon", &recon_path, &earth_path);
    let de441 = load_line("de441", &de441_juice, &de441_earth);
    let de442 = load_line("de442", &de442_juice, &de442_earth);

    let window_tdb = match (
        nominal_window,
        sealed.as_ref().and_then(|l| coverage_seconds(&l.bodies)),
    ) {
        (Some((lo, hi)), Some((slo, shi))) => {
            let wlo = lo.max(slo);
            let whi = hi.min(shi);
            if whi > wlo { Some((wlo, whi)) } else { None }
        }
        _ => None,
    };
    let sealed_sha = file_sha(&sealed_path);
    let renewed_sha = file_sha(&renewed_path);
    let recon_sha = file_sha(&recon_path);

    let dev_sealed = match (window_tdb, &recon, &sealed) {
        (Some((lo, hi)), Some(r), Some(s)) => Some(max_dev_km(r, s, lo, hi, step)),
        _ => None,
    };
    let dev_renewed = match (window_tdb, &recon, &renewed) {
        (Some((lo, hi)), Some(r), Some(s)) => Some(max_dev_km(r, s, lo, hi, step)),
        _ => None,
    };
    let dev_rift = match (window_tdb, &de441, &de442) {
        (Some((lo, hi)), Some(a), Some(b)) => Some(max_dev_km(a, b, lo, hi, step)),
        _ => None,
    };

    let delta_sealed = dev_sealed.and_then(|(m, _, _)| m);
    let delta_renewed = dev_renewed.and_then(|(m, _, _)| m);
    let edition_rift = dev_rift.and_then(|(m, _, _)| m);

    let threshold = match (edition_rift, sigma_recon) {
        (Some(d), Some(s)) => Some(d + 3.0 * s),
        _ => None,
    };
    let verdict_sealed = match (delta_sealed, threshold) {
        (Some(d), Some(th)) => Some(d <= th),
        _ => None,
    };
    let verdict_renewed = match (delta_renewed, threshold) {
        (Some(d), Some(th)) => Some(d <= th),
        _ => None,
    };
    let riss = anchor == "riss"
        || match (verdict_sealed, verdict_renewed) {
            (Some(a), Some(b)) => a != b,
            _ => false,
        };

    println!("flyby_ephemeris_gate — JUICE Earth flyby {PERIGEE_UTC}");
    match (perigee_unix, perigee_tdb) {
        (Some(u), Some(t)) => println!("perigee: unix {u} s, tdb {t} s"),
        _ => println!("perigee: pending — the civil date or the leap table carries no value"),
    }
    match window_tdb {
        Some((lo, hi)) => {
            let n_steps = ((hi - lo) / step).floor() as usize + 1;
            println!(
                "window: ±{window_days} d nominal, sealed arc {lo}..{hi} tdb s, step {step_hours} h, {n_steps} samples"
            )
        }
        None => {
            println!("window: pending — the sealed arc carries no coverage to bound the window")
        }
    }
    println!(
        "anchor sealed: constant {SEALED_SHA256} · register {} → {anchor}",
        sha_label(sealed_pin.as_deref())
    );
    println!(
        "trajectory sealed: sha256 {} — seal {} → {}",
        sha_label(sealed_sha.as_deref()),
        seal_expected,
        seal_word(sealed_sha.as_deref(), Some(seal_expected))
    );
    println!(
        "trajectory renewed: sha256 {} — seal {} → {}",
        sha_label(renewed_sha.as_deref()),
        RENEWED_SHA256,
        seal_word(renewed_sha.as_deref(), Some(RENEWED_SHA256))
    );
    println!(
        "trajectory recon: sha256 {} — no seal",
        sha_label(recon_sha.as_deref())
    );
    println!("Δ sealed (recon vs sealed): {} km", num_label(delta_sealed));
    println!(
        "Δ renewed (recon vs renewed): {} km",
        num_label(delta_renewed)
    );
    println!(
        "δ edition rift (DE441 vs DE442): {} km",
        num_label(edition_rift)
    );
    for (word, void) in [
        ("Δ sealed", dev_sealed.map(|(_, _, v)| v)),
        ("Δ renewed", dev_renewed.map(|(_, _, v)| v)),
        ("δ edition rift", dev_rift.map(|(_, _, v)| v)),
    ] {
        match void {
            Some(v) if v > 0 => println!("{word}: {v} void samples — the max stays pending"),
            _ => {}
        }
    }
    println!("σ_recon: {} km", num_label(sigma_recon));
    println!("threshold δ + 3σ: {} km", num_label(threshold));
    println!("verdict sealed: {}", word_label(verdict_sealed));
    println!("verdict renewed: {}", word_label(verdict_renewed));
    println!("riss: {riss}");

    let json = register_json(
        window_days,
        step_hours,
        sealed_sha.as_deref(),
        sealed_pin.as_deref(),
        anchor,
        renewed_sha.as_deref(),
        recon_sha.as_deref(),
        delta_sealed,
        delta_renewed,
        edition_rift,
        sigma_recon,
        threshold,
        verdict_sealed,
        verdict_renewed,
        riss,
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

#[cfg(test)]
mod tests {
    use super::*;

    const PIN: &str = "aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a";

    fn snippet(sha: &str) -> String {
        format!(
            "url https://example.com/ssd.jpl.nasa.gov-ephemeris/ephemeris_juice_official.bin\n\
             terms PD-USGov https://ssd.jpl.nasa.gov/\n\
             format ephemeris_binary\n\
             sha256 {sha}\n\
             at juice\n\
             no-cadence\n"
        )
    }

    #[test]
    fn register_pin_reads_the_pinned_post() {
        assert_eq!(
            register_pin(&snippet(PIN), SEALED_ASSET),
            Some(PIN.to_string())
        );
    }

    #[test]
    fn register_pin_ignores_other_assets() {
        let other = "url https://example.com/ephemeris_juice.bin\nterms PD-USGov https://ssd.jpl.nasa.gov/\nformat ephemeris_binary\nsha256 1111111111111111111111111111111111111111111111111111111111111111\nat juice\nno-cadence\n";
        assert_eq!(register_pin(other, SEALED_ASSET), None);
    }

    #[test]
    fn anchor_is_pending_without_a_pin_and_riss_on_disagreement() {
        assert_eq!(anchor_word(None, PIN), "pending");
        assert_eq!(anchor_word(Some(PIN), PIN), "placed");
        assert_eq!(anchor_word(Some("0000"), PIN), "riss");
    }
}
