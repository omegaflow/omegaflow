use omegaflow::callisto::{BAND_COUNT, component_name, parse_bin as parse_callisto};
use omegaflow::archivar::radio_burst::parse_bin as parse_swpc;
use omegaflow::lsk::days_from_civil;

const DETECT_Z: f64 = 5.0;
const BAND_MIN: usize = 3;
const COVER_HALF_S: f64 = 5.0 * 60.0;
const MATCH_HALF_S: f64 = 10.0 * 60.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn median(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    if n % 2 == 0 {
        Some((v[n / 2 - 1] + v[n / 2]) * 0.5)
    } else {
        Some(v[n / 2])
    }
}

fn iso(t: f64) -> String {
    let secs = t as i64;
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
}

fn parse_iso(s: &str) -> Option<f64> {
    let bytes = s.as_bytes();
    if bytes.len() < 19 {
        return None;
    }
    let y: i64 = s.get(0..4)?.parse().ok()?;
    let mo: i64 = s.get(5..7)?.parse().ok()?;
    let d: i64 = s.get(8..10)?.parse().ok()?;
    let h: i64 = s.get(11..13)?.parse().ok()?;
    let mi: i64 = s.get(14..16)?.parse().ok()?;
    let sec: f64 = s.get(17..19)?.parse().ok()?;
    let days = days_from_civil(y, mo, d)?;
    Some((days * 86_400 + h * 3600 + mi * 60) as f64 + sec)
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let callisto_path = match arg_value(&args, "--callisto") {
        Some(p) => p,
        None => {
            eprintln!("callisto_swpc_validation needs --callisto <bin>");
            std::process::exit(2);
        }
    };
    let swpc_path = match arg_value(&args, "--swpc") {
        Some(p) => p,
        None => {
            eprintln!("callisto_swpc_validation needs --swpc <bin>");
            std::process::exit(2);
        }
    };
    let Some(callisto_bytes) = std::fs::read(&callisto_path).ok() else {
        eprintln!("{callisto_path}: read void");
        std::process::exit(1);
    };
    let z_gate: f64 = arg_value(&args, "--z")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DETECT_Z);
    let band_min: usize = arg_value(&args, "--band-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(BAND_MIN);
    let Some(swpc_bytes) = std::fs::read(&swpc_path).ok() else {
        eprintln!("{swpc_path}: read void");
        std::process::exit(1);
    };
    let Some(records) = parse_callisto(&callisto_bytes) else {
        eprintln!("{callisto_path}: CALLISTO bin reads void");
        std::process::exit(1);
    };
    let Some(events) = parse_swpc(&swpc_bytes) else {
        eprintln!("{swpc_path}: SWPC bin reads void");
        std::process::exit(1);
    };
    if records.is_empty() || events.is_empty() {
        eprintln!("no records — the validation stays unmeasured");
        std::process::exit(1);
    }
    let first_event = events.iter().map(|e| e.t_unix).fold(f64::INFINITY, f64::min);
    let last_event = events
        .iter()
        .map(|e| e.t_unix)
        .fold(f64::NEG_INFINITY, f64::max);
    let lo = first_event - 86_400.0;
    let hi = last_event + 86_400.0;
    let window: Vec<_> = records
        .iter()
        .filter(|r| r.t_unix >= lo && r.t_unix <= hi)
        .collect();
    if window.is_empty() {
        eprintln!("the SWPC window carries no CALLISTO minutes — unmeasured");
        std::process::exit(1);
    }
    if let (Some(a), Some(b)) = (
        arg_value(&args, "--dump-start"),
        arg_value(&args, "--dump-end"),
    ) {
        let (Some(a), Some(b)) = (parse_iso(&a), parse_iso(&b)) else {
            eprintln!("--dump-start/--dump-end carry no ISO time");
            std::process::exit(2);
        };
        for r in &window {
            if r.t_unix < a || r.t_unix > b {
                continue;
            }
            print!("{}", iso(r.t_unix));
            for b in 0..BAND_COUNT {
                match r.bands[b] {
                    Some(v) => print!(" {:>8.2}", v),
                    None => print!(" {:>8}", "-"),
                }
            }
            println!();
        }
        return;
    }
    println!(
        "CALLISTO window {} .. {} ({} minutes); SWPC {} events {} .. {}",
        iso(window[0].t_unix),
        iso(window[window.len() - 1].t_unix),
        window.len(),
        events.len(),
        iso(first_event),
        iso(last_event)
    );

    let mut med = [0.0f64; BAND_COUNT];
    let mut mad = [1.0f64; BAND_COUNT];
    for b in 0..BAND_COUNT {
        let vals: Vec<f64> = window.iter().filter_map(|r| r.bands[b]).collect();
        let Some(m) = median(vals.clone()) else {
            continue;
        };
        med[b] = m;
        let dev: Vec<f64> = vals.iter().map(|v| (v - m).abs()).collect();
        let Some(d) = median(dev) else {
            continue;
        };
        mad[b] = if d > 0.0 { d } else { 1.0 };
    }
    println!("band                              median      MAD   n");
    for b in 0..BAND_COUNT {
        let n = window.iter().filter(|r| r.bands[b].is_some()).count();
        println!(
            "{:<32} {:>9.3} {:>7.3} {:>6}",
            component_name(b as u32).unwrap_or("?"),
            med[b],
            mad[b],
            n
        );
    }

    let mut active: Vec<(f64, f64)> = Vec::new();
    let mut current: Option<(f64, f64)> = None;
    for r in &window {
        let mut peak = 0.0f64;
        let mut n_bands = 0usize;
        for b in 0..BAND_COUNT {
            if let Some(v) = r.bands[b] {
                let z = (v - med[b]).abs() / (1.4826 * mad[b]);
                if z > peak {
                    peak = z;
                }
                if z > z_gate {
                    n_bands += 1;
                }
            }
        }
        if peak > z_gate && n_bands >= band_min {
            match current {
                Some((start, last)) if r.t_unix - last <= 120.0 => current = Some((start, r.t_unix)),
                Some((start, last)) => {
                    active.push((start, last));
                    current = Some((r.t_unix, r.t_unix));
                }
                None => current = Some((r.t_unix, r.t_unix)),
            }
        }
    }
    if let Some(span) = current {
        active.push(span);
    }
    println!(
        "detector z > {z_gate:.1} (1.4826*MAD) in >= {band_min} bands, >= 2 active minutes within 2 min: {} events",
        active.len()
    );

    let mut covered = 0usize;
    let mut hits = 0usize;
    let mut misses = 0usize;
    let mut unobserved = 0usize;
    for e in &events {
        let has_coverage = window
            .iter()
            .any(|r| (r.t_unix - e.t_unix).abs() <= COVER_HALF_S);
        if !has_coverage {
            unobserved += 1;
            println!(
                "UNOBSERVED {} {} {} (no CALLISTO minute within +/-5 min)",
                iso(e.t_unix),
                e.type_code.map(|x| x as i64).unwrap_or(-1),
                e.freq_low_mhz.map(|x| x as i64).unwrap_or(-1)
            );
            continue;
        }
        covered += 1;
        let hit = active
            .iter()
            .any(|(s, l)| e.t_unix >= *s - MATCH_HALF_S && e.t_unix <= *l + MATCH_HALF_S);
        if hit {
            hits += 1;
            println!("HIT       {} type {:?}", iso(e.t_unix), e.type_code);
        } else {
            misses += 1;
            println!("MISS      {} type {:?}", iso(e.t_unix), e.type_code);
        }
    }
    let mut alarms = 0usize;
    for (s, _l) in &active {
        let matched = events
            .iter()
            .any(|e| *s >= e.t_unix - MATCH_HALF_S && *s <= e.t_unix + MATCH_HALF_S);
        if !matched {
            alarms += 1;
            println!("ALARM     {} (no SWPC event within +/-10 min)", iso(*s));
        }
    }
    println!(
        "validation: {} covered, {} unobserved, {} hit, {} miss, {} unmatched detections",
        covered, unobserved, hits, misses, alarms
    );
    if covered > 0 {
        println!(
            "hit rate on covered events: {:.3} ({} / {})",
            hits as f64 / covered as f64,
            hits,
            covered
        );
    }
}
