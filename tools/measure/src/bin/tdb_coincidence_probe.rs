use omegaflow::archivar::C_LIGHT;
use omegaflow::archivar::{
    body_barycenter_position, cache_root, embedded_lsk, fetch_raw_bytes, parse_ephemeris_binary,
    BodyEphemeris, LeapSeconds,
};
use omegaflow::jwst::mjd_to_unix;
use omegaflow::kepler::AU_M;
use std::collections::HashMap;

const SUN_EPH_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_sun.bin";
const EARTH_EPH_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_earth.bin";

const JD_UNIX_OFFSET: f64 = 2440587.5;
const DAY_S: f64 = 86400.0;
const EARTH_RADIUS_M: f64 = 6.378137e6;
const SIDEREAL_YEAR_DAYS: f64 = 365.25;

#[derive(Clone, Copy, PartialEq)]
enum TimeKind {
    Mjd,
    Jd,
}

#[derive(Clone, Copy, PartialEq)]
enum TimeScale {
    Utc,
    Tai,
}

struct Event {
    ra_deg: f64,
    dec_deg: f64,
    day: f64,
    kind: TimeKind,
    scale: TimeScale,
}

struct Folding {
    arrival_tdb: f64,
    roemer_s: f64,
    emitted_tdb: f64,
}

enum FoldOutcome {
    Folding(Folding),
    TimeVoid,
    EphemerisVoid,
}

struct PairAnalysis {
    naive_s: Option<f64>,
    fold_a: Option<Folding>,
    fold_b: Option<Folding>,
    corrected_s: Option<f64>,
    pending_reason: String,
}

enum VerdictWord {
    Simultaneous,
    Separated,
    Pending,
}

fn kind_token(kind: TimeKind) -> &'static str {
    match kind {
        TimeKind::Mjd => "mjd",
        TimeKind::Jd => "jd",
    }
}

fn scale_token(scale: TimeScale) -> &'static str {
    match scale {
        TimeScale::Utc => "utc",
        TimeScale::Tai => "tai",
    }
}

fn parse_kind(token: &str) -> Option<TimeKind> {
    match token {
        "mjd" => Some(TimeKind::Mjd),
        "jd" => Some(TimeKind::Jd),
        _ => None,
    }
}

fn parse_scale(token: &str) -> Option<TimeScale> {
    match token {
        "utc" => Some(TimeScale::Utc),
        "tai" => Some(TimeScale::Tai),
        _ => None,
    }
}

fn sightline_unit(ra_deg: f64, dec_deg: f64) -> [f64; 3] {
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let cd = dec.cos();
    [cd * ra.cos(), cd * ra.sin(), dec.sin()]
}

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn to_tdb_arrival(day: f64, kind: TimeKind, scale: TimeScale, lsk: &LeapSeconds) -> Option<f64> {
    if !day.is_finite() {
        return None;
    }
    let unix = match kind {
        TimeKind::Mjd => mjd_to_unix(day),
        TimeKind::Jd => (day - JD_UNIX_OFFSET) * DAY_S,
    };
    match scale {
        TimeScale::Utc => lsk.unix_to_tdb(unix),
        TimeScale::Tai => {
            let tai_minus_utc = lsk.leap_at(unix)?;
            lsk.unix_to_tdb(unix - tai_minus_utc)
        }
    }
}

fn roemer_term_s(
    ra_deg: f64,
    dec_deg: f64,
    observer: [f64; 3],
    reference: [f64; 3],
) -> Option<f64> {
    if !ra_deg.is_finite() || !dec_deg.is_finite() {
        return None;
    }
    let n = sightline_unit(ra_deg, dec_deg);
    let tau = vec_dot(n, vec_sub(observer, reference)) / C_LIGHT;
    if tau.is_finite() {
        Some(tau)
    } else {
        None
    }
}

fn emitted_fold(
    arrival_tdb: f64,
    ra_deg: f64,
    dec_deg: f64,
    observer: [f64; 3],
    reference: [f64; 3],
) -> Option<Folding> {
    let roemer_s = roemer_term_s(ra_deg, dec_deg, observer, reference)?;
    if !arrival_tdb.is_finite() {
        return None;
    }
    Some(Folding {
        arrival_tdb,
        roemer_s,
        emitted_tdb: arrival_tdb + roemer_s,
    })
}

fn fold_event(ev: &Event, lsk: &LeapSeconds, eph: &HashMap<String, BodyEphemeris>) -> FoldOutcome {
    let Some(arrival_tdb) = to_tdb_arrival(ev.day, ev.kind, ev.scale, lsk) else {
        return FoldOutcome::TimeVoid;
    };
    let Some(earth) = body_barycenter_position("earth", arrival_tdb, eph) else {
        return FoldOutcome::EphemerisVoid;
    };
    let Some(sun) = body_barycenter_position("sun", arrival_tdb, eph) else {
        return FoldOutcome::EphemerisVoid;
    };
    match emitted_fold(arrival_tdb, ev.ra_deg, ev.dec_deg, earth, sun) {
        Some(f) => FoldOutcome::Folding(f),
        None => FoldOutcome::EphemerisVoid,
    }
}

fn naive_clock_s(a: &Event, b: &Event) -> Option<f64> {
    if a.kind == b.kind && a.scale == b.scale {
        Some((a.day - b.day) * DAY_S)
    } else {
        None
    }
}

fn naive_absent_reason(a: &Event, b: &Event) -> String {
    if a.kind != b.kind {
        format!(
            "raw day numbers differ ({} vs {}) — a unit error, not a measurement",
            kind_token(a.kind),
            kind_token(b.kind)
        )
    } else {
        format!(
            "raw clock scales differ ({} vs {}) — an unshared TAI-UTC offset would masquerade as sky time",
            scale_token(a.scale),
            scale_token(b.scale)
        )
    }
}

fn fold_pending_reason(label: &str, outcome: &FoldOutcome) -> String {
    match outcome {
        FoldOutcome::Folding(_) => String::new(),
        FoldOutcome::TimeVoid => format!(
            "{label} maps to no TDB arrival (the naif0012.tls leap table is void at its epoch)"
        ),
        FoldOutcome::EphemerisVoid => format!(
            "{label} carries no Rømer term (the solar-system ephemeris does not cover its epoch or a coordinate is void)"
        ),
    }
}

fn analyze(
    a: &Event,
    b: &Event,
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
) -> PairAnalysis {
    let naive_s = naive_clock_s(a, b);
    let fa = fold_event(a, lsk, eph);
    let fb = fold_event(b, lsk, eph);
    let fold_a = match &fa {
        FoldOutcome::Folding(f) => Some(Folding {
            arrival_tdb: f.arrival_tdb,
            roemer_s: f.roemer_s,
            emitted_tdb: f.emitted_tdb,
        }),
        _ => None,
    };
    let fold_b = match &fb {
        FoldOutcome::Folding(f) => Some(Folding {
            arrival_tdb: f.arrival_tdb,
            roemer_s: f.roemer_s,
            emitted_tdb: f.emitted_tdb,
        }),
        _ => None,
    };
    let corrected_s = match (&fold_a, &fold_b) {
        (Some(x), Some(y)) => Some(x.emitted_tdb - y.emitted_tdb),
        _ => None,
    };
    let mut reasons: Vec<String> = Vec::new();
    if fold_a.is_none() {
        reasons.push(fold_pending_reason("event A", &fa));
    }
    if fold_b.is_none() {
        reasons.push(fold_pending_reason("event B", &fb));
    }
    let pending_reason = if corrected_s.is_some() {
        String::new()
    } else {
        reasons.join("; ")
    };
    PairAnalysis {
        naive_s,
        fold_a,
        fold_b,
        corrected_s,
        pending_reason,
    }
}

fn decide(corrected_s: Option<f64>, window_s: Option<f64>) -> VerdictWord {
    match corrected_s {
        None => VerdictWord::Pending,
        Some(diff) => match window_s {
            None => VerdictWord::Pending,
            Some(window) => {
                if diff.abs() <= window {
                    VerdictWord::Simultaneous
                } else {
                    VerdictWord::Separated
                }
            }
        },
    }
}

fn report_pair(
    index: usize,
    a: &Event,
    b: &Event,
    window_s: Option<f64>,
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
) {
    let an = analyze(a, b, lsk, eph);
    let window_text = match window_s {
        Some(w) => format!("{w:.3} s"),
        None => "none (not declared)".to_string(),
    };
    println!("TDB-Rømer coincidence probe: pair {index} — coincidence window {window_text}");
    println!(
        "  event A: ra {:.5}° dec {:.5}° at {}{} {:.9}",
        a.ra_deg,
        a.dec_deg,
        kind_token(a.kind),
        scale_token(a.scale),
        a.day
    );
    println!(
        "  event B: ra {:.5}° dec {:.5}° at {}{} {:.9}",
        b.ra_deg,
        b.dec_deg,
        kind_token(b.kind),
        scale_token(b.scale),
        b.day
    );
    for (label, fold) in [("A", &an.fold_a), ("B", &an.fold_b)] {
        match fold {
            Some(f) => println!(
                "  Rømer fold {label}: arrival TDB {:.6} s (J2000-relative) + {:.6} s light-time → emitted {:.6} s",
                f.arrival_tdb, f.roemer_s, f.emitted_tdb
            ),
            None => println!("  Rømer fold {label}: not computable"),
        }
    }
    match an.naive_s {
        Some(s) => println!("  naive raw-clock difference A-B = {s:.6} s"),
        None => println!(
            "  naive raw-clock difference A-B = absent ({})",
            naive_absent_reason(a, b)
        ),
    }
    match an.corrected_s {
        Some(s) => println!("  TDB+Rømer emitted difference A-B = {s:.6} s"),
        None => println!(
            "  TDB+Rømer emitted difference A-B = not computable ({})",
            an.pending_reason
        ),
    }
    if let (Some(c), Some(n)) = (an.corrected_s, an.naive_s) {
        println!(
            "  honest sharpening (corrected minus naive) = {:.6} s",
            c - n
        );
    }
    match decide(an.corrected_s, window_s) {
        VerdictWord::Simultaneous => println!(
            "  verdict: simultaneous — |emitted difference| ≤ window (same clock to the source frame)"
        ),
        VerdictWord::Separated => match (window_s, an.corrected_s) {
            (Some(w), Some(diff)) => println!(
                "  verdict: separated — measured emitted difference {diff:.6} s exceeds the {w:.3} s window"
            ),
            _ => println!("  verdict: separated"),
        },
        VerdictWord::Pending => {
            if an.pending_reason.is_empty() {
                println!(
                    "  verdict: pending — no coincidence window declared; the emitted difference above is the measurement"
                );
            } else {
                println!("  verdict: pending — {}", an.pending_reason);
            }
        }
    }
}

fn print_term_budget() {
    let single_s = AU_M / C_LIGHT;
    let cross_s = 2.0 * single_s;
    let v_earth_mps = 2.0 * std::f64::consts::PI * AU_M / (SIDEREAL_YEAR_DAYS * DAY_S);
    let weekly_s = v_earth_mps * 7.0 * DAY_S / C_LIGHT;
    let diurnal_ms = EARTH_RADIUS_M / C_LIGHT * 1000.0;
    println!(
        "TDB-Rømer coincidence probe: term budget (measured from constants) — single-sightline Rømer seasonal amplitude {single_s:.1} s (±8 min); opposite-direction pair 2 AU/c {cross_s:.1} s; Earth line-of-sight drift {weekly_s:.1} s/week; observer = geocenter (the diurnal station swing ≤ {diurnal_ms:.1} ms stays a named bound)"
    );
}

fn ephemeris_cache_bytes(name: &str, url: &str) -> Option<Vec<u8>> {
    let path = cache_root().join(format!("lsst_roemer_{name}.bin"));
    if let Ok(b) = std::fs::read(&path) {
        return Some(b);
    }
    let b = fetch_raw_bytes(url, 3600)?;
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&path, &b).is_err() {
        println!(
            "TDB-Rømer coincidence probe: the ephemeris cache was not written ({})",
            path.display()
        );
    }
    Some(b)
}

fn load_ephemeris_map() -> Option<HashMap<String, BodyEphemeris>> {
    let sun_b = ephemeris_cache_bytes("sun", SUN_EPH_CDN)?;
    let earth_b = ephemeris_cache_bytes("earth", EARTH_EPH_CDN)?;
    let mut map = HashMap::new();
    map.insert("sun".to_string(), parse_ephemeris_binary(&sun_b)?);
    map.insert("earth".to_string(), parse_ephemeris_binary(&earth_b)?);
    Some(map)
}

fn parse_event_tokens(tokens: &[&str], offset: usize) -> Option<Event> {
    let ra_deg = tokens.get(offset)?.parse().ok()?;
    let dec_deg = tokens.get(offset + 1)?.parse().ok()?;
    let day = tokens.get(offset + 2)?.parse().ok()?;
    let kind = parse_kind(tokens.get(offset + 3)?)?;
    let scale = parse_scale(tokens.get(offset + 4)?)?;
    Some(Event {
        ra_deg,
        dec_deg,
        day,
        kind,
        scale,
    })
}

fn run_file(
    path: &str,
    window_s: Option<f64>,
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
) {
    let Ok(text) = std::fs::read_to_string(path) else {
        println!("TDB-Rømer coincidence probe: the pair file {path} does not read — pending");
        return;
    };
    let mut index = 0usize;
    for (line_no, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        let mut line_window = window_s;
        if tokens.len() == 11 {
            line_window = tokens[10].parse::<f64>().ok().filter(|w| w.is_finite());
        }
        index += 1;
        let Some(a) = parse_event_tokens(&tokens, 0) else {
            println!(
                "TDB-Rømer coincidence probe: pair file line {} — event A absent (pending); expected 10 or 11 tokens, saw {}",
                line_no + 1,
                tokens.len()
            );
            continue;
        };
        let Some(b) = parse_event_tokens(&tokens, 5) else {
            println!(
                "TDB-Rømer coincidence probe: pair file line {} — event B absent (pending)",
                line_no + 1
            );
            continue;
        };
        report_pair(index, &a, &b, line_window, lsk, eph);
    }
    if index == 0 {
        println!("TDB-Rømer coincidence probe: the pair file {path} carries no pair (0 honored)");
    }
}

fn arg_f64(args: &[String], key: &str) -> Option<f64> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1)?.parse().ok()
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn arg_time_kind(args: &[String], key: &str) -> Option<TimeKind> {
    parse_kind(&arg_token(args, key)?)
}

fn arg_time_scale(args: &[String], key: &str) -> Option<TimeScale> {
    parse_scale(&arg_token(args, key)?)
}

fn usage() {
    println!(
        "TDB-Rømer coincidence probe — Funke 5: simultaneity on the honest clock.\n\
         One-shot pair:\n  \
         cargo run -p omegaflow-measure --bin tdb_coincidence_probe -- \\\n    \
         --ra1 <deg> --dec1 <deg> --t1 <mjd|jd> \\\n    \
         --ra2 <deg> --dec2 <deg> --t2 <mjd|jd> [--kind1 mjd|jd] [--scale1 utc|tai] \\\n    \
         [--kind2 mjd|jd] [--scale2 utc|tai] [--window-s <seconds>]\n\
         Pair file:\n  \
         cargo run -p omegaflow-measure --bin tdb_coincidence_probe -- --file <path> [--window-s <seconds>]\n\
         File line: ra1 dec1 t1 kind1 scale1 ra2 dec2 t2 kind2 scale2 [window_s]; '#' comments."
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage();
        return;
    }
    let Some(lsk) = embedded_lsk() else {
        println!(
            "TDB-Rømer coincidence probe: the embedded naif0012.tls leap table is absent — the time layer stays pending"
        );
        return;
    };
    print_term_budget();
    let eph = match load_ephemeris_map() {
        Some(map) => map,
        None => {
            println!(
                "TDB-Rømer coincidence probe: the solar-system ephemeris for the Rømer term is not reachable — every Rømer fold stays pending (named); naive raw-clock differences remain measurable"
            );
            HashMap::new()
        }
    };
    let window_s = arg_f64(&args, "--window-s");
    if let Some(path) = arg_token(&args, "--file") {
        run_file(&path, window_s, &lsk, &eph);
        return;
    }
    let ra1 = arg_f64(&args, "--ra1");
    let dec1 = arg_f64(&args, "--dec1");
    let t1 = arg_f64(&args, "--t1");
    let ra2 = arg_f64(&args, "--ra2");
    let dec2 = arg_f64(&args, "--dec2");
    let t2 = arg_f64(&args, "--t2");
    match (ra1, dec1, t1, ra2, dec2, t2) {
        (Some(ra1), Some(dec1), Some(t1), Some(ra2), Some(dec2), Some(t2)) => {
            let kind1 = arg_time_kind(&args, "--kind1").unwrap_or(TimeKind::Mjd);
            let scale1 = arg_time_scale(&args, "--scale1").unwrap_or(TimeScale::Utc);
            let kind2 = arg_time_kind(&args, "--kind2").unwrap_or(TimeKind::Mjd);
            let scale2 = arg_time_scale(&args, "--scale2").unwrap_or(TimeScale::Utc);
            let a = Event {
                ra_deg: ra1,
                dec_deg: dec1,
                day: t1,
                kind: kind1,
                scale: scale1,
            };
            let b = Event {
                ra_deg: ra2,
                dec_deg: dec2,
                day: t2,
                kind: kind2,
                scale: scale2,
            };
            report_pair(1, &a, &b, window_s, &lsk, &eph);
        }
        _ => {
            println!(
                "TDB-Rømer coincidence probe: the event pair is incomplete (a coordinate or a time is absent) — pending"
            );
            usage();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn earth_sun_geometry() -> ([f64; 3], [f64; 3]) {
        ([AU_M, 0.0, 0.0], [0.0, 0.0, 0.0])
    }

    #[test]
    fn opposite_directions_equal_emitted_fold_to_simultaneous() {
        let (earth, sun) = earth_sun_geometry();
        let single = AU_M / C_LIGHT;
        let emitted = 1000.0;
        let arrival_a = emitted - single;
        let arrival_b = emitted + single;
        let fold_a = emitted_fold(arrival_a, 0.0, 0.0, earth, sun).unwrap();
        let fold_b = emitted_fold(arrival_b, 180.0, 0.0, earth, sun).unwrap();
        assert!(
            (fold_a.roemer_s - single).abs() < 1e-9,
            "the ra 0 sightline carries +1 AU/c light-time, was {}",
            fold_a.roemer_s
        );
        assert!(
            (fold_b.roemer_s + single).abs() < 1e-9,
            "the ra 180 sightline carries -1 AU/c light-time, was {}",
            fold_b.roemer_s
        );
        let naive_s = arrival_a - arrival_b;
        assert!(
            (naive_s + 2.0 * single).abs() < 1e-9,
            "naive clock difference is -2 AU/c, was {naive_s}"
        );
        let corrected_s = fold_a.emitted_tdb - fold_b.emitted_tdb;
        assert!(
            corrected_s.abs() < 1e-9,
            "corrected emitted difference is ~0, was {corrected_s}"
        );
        assert!(
            matches!(
                decide(Some(corrected_s), Some(60.0)),
                VerdictWord::Simultaneous
            ),
            "equal emitted times sit inside a 60 s window"
        );
    }

    #[test]
    fn naive_to_corrected_sharpening_is_the_roemer_cross_term() {
        let (earth, sun) = earth_sun_geometry();
        let single = AU_M / C_LIGHT;
        let emitted = 1000.0;
        let a = Event {
            ra_deg: 0.0,
            dec_deg: 0.0,
            day: 60000.0 + (emitted - single) / DAY_S,
            kind: TimeKind::Mjd,
            scale: TimeScale::Utc,
        };
        let b = Event {
            ra_deg: 180.0,
            dec_deg: 0.0,
            day: 60000.0 + (emitted + single) / DAY_S,
            kind: TimeKind::Mjd,
            scale: TimeScale::Utc,
        };
        let naive_s = naive_clock_s(&a, &b).unwrap();
        let fold_a = emitted_fold(emitted - single, a.ra_deg, a.dec_deg, earth, sun).unwrap();
        let fold_b = emitted_fold(emitted + single, b.ra_deg, b.dec_deg, earth, sun).unwrap();
        let corrected_s = fold_a.emitted_tdb - fold_b.emitted_tdb;
        let sharpening_s = corrected_s - naive_s;
        assert!(
            (naive_s + 2.0 * single).abs() < 1e-6,
            "naive MJD difference is -2 AU/c ≈ -998 s, was {naive_s}"
        );
        assert!(
            corrected_s.abs() < 1e-9,
            "corrected emitted difference is ~0, was {corrected_s}"
        );
        assert!(
            (sharpening_s - 2.0 * single).abs() < 1e-6,
            "the honest sharpening is the full 2 AU/c cross term ≈ +998 s, was {sharpening_s}"
        );
    }

    #[test]
    fn separated_pair_and_window_boundary() {
        assert!(matches!(
            decide(Some(600.0), Some(300.0)),
            VerdictWord::Separated
        ));
        assert!(matches!(
            decide(Some(300.0), Some(300.0)),
            VerdictWord::Simultaneous
        ));
        assert!(matches!(
            decide(Some(-300.0), Some(300.0)),
            VerdictWord::Simultaneous
        ));
    }

    #[test]
    fn tai_and_utc_raw_clocks_fold_to_the_same_tdb_instant() {
        let lsk = embedded_lsk().expect("the embedded naif0012.tls parses");
        let unix_utc = 1_785_000_000.0;
        let leap = lsk.leap_at(unix_utc).expect("2026 carries a leap offset");
        let mjd_utc = unix_utc / DAY_S + 40587.0;
        let mjd_tai = mjd_utc + leap / DAY_S;
        let tdb_utc = to_tdb_arrival(mjd_utc, TimeKind::Mjd, TimeScale::Utc, &lsk).unwrap();
        let tdb_tai = to_tdb_arrival(mjd_tai, TimeKind::Mjd, TimeScale::Tai, &lsk).unwrap();
        assert!(
            (tdb_utc - tdb_tai).abs() < 1e-6,
            "the same physical instant folds identically from utc and tai raw clocks, was {tdb_utc} vs {tdb_tai}"
        );
        assert!(
            to_tdb_arrival(30000.0, TimeKind::Mjd, TimeScale::Utc, &lsk).is_none(),
            "a pre-1972 epoch leaves the leap table void — no fabricated TDB"
        );
    }

    #[test]
    fn absent_geometry_stays_pending() {
        let lsk = embedded_lsk().expect("the embedded naif0012.tls parses");
        let empty: HashMap<String, BodyEphemeris> = HashMap::new();
        let a = Event {
            ra_deg: 0.0,
            dec_deg: 0.0,
            day: 61246.0,
            kind: TimeKind::Mjd,
            scale: TimeScale::Utc,
        };
        let b = Event {
            ra_deg: 180.0,
            dec_deg: 0.0,
            day: 61246.0,
            kind: TimeKind::Mjd,
            scale: TimeScale::Utc,
        };
        assert!(matches!(
            fold_event(&a, &lsk, &empty),
            FoldOutcome::EphemerisVoid
        ));
        let an = analyze(&a, &b, &lsk, &empty);
        assert!(an.corrected_s.is_none());
        assert!(!an.pending_reason.is_empty());
        assert!(matches!(
            decide(an.corrected_s, Some(60.0)),
            VerdictWord::Pending
        ));
    }

    #[test]
    fn absent_time_maps_to_pending() {
        let lsk = embedded_lsk().expect("the embedded naif0012.tls parses");
        let empty: HashMap<String, BodyEphemeris> = HashMap::new();
        let a = Event {
            ra_deg: 0.0,
            dec_deg: 0.0,
            day: 30000.0,
            kind: TimeKind::Mjd,
            scale: TimeScale::Utc,
        };
        let b = Event {
            ra_deg: 180.0,
            dec_deg: 0.0,
            day: 61246.0,
            kind: TimeKind::Mjd,
            scale: TimeScale::Utc,
        };
        assert!(matches!(
            fold_event(&a, &lsk, &empty),
            FoldOutcome::TimeVoid
        ));
        let an = analyze(&a, &b, &lsk, &empty);
        assert!(an.naive_s.is_some());
        assert!(an.corrected_s.is_none());
        assert!(an.pending_reason.contains("event A"));
    }

    #[test]
    fn mismatched_raw_clocks_leave_naive_absent() {
        let a = Event {
            ra_deg: 0.0,
            dec_deg: 0.0,
            day: 61246.0,
            kind: TimeKind::Mjd,
            scale: TimeScale::Utc,
        };
        let b = Event {
            ra_deg: 180.0,
            dec_deg: 0.0,
            day: 2460000.5,
            kind: TimeKind::Jd,
            scale: TimeScale::Utc,
        };
        assert!(naive_clock_s(&a, &b).is_none());
        let c = Event {
            ra_deg: 180.0,
            dec_deg: 0.0,
            day: 61246.0,
            kind: TimeKind::Mjd,
            scale: TimeScale::Tai,
        };
        assert!(naive_clock_s(&a, &c).is_none());
    }
}
