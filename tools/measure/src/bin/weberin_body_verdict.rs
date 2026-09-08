use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use omegaflow::archivar::{
    embedded_lsk, extract, fetch_raw_bytes, load_sources, parse_ephemeris_binary, system_now,
    BodyEphemeris, ExtractResult, LeapSeconds, SourceConfig, J2000_EPOCH,
};
use omegaflow::cdn::{CDN_BASE, CDN_RELEASE};
use omegaflow::dastcom::{
    parse_comet_record, parse_record, AsteroidRec, CometRec, COMET_RECORD_BYTES, RECORD_STRIDE,
};
use omegaflow::weberin::{
    BodyOutcome, Weberin, WeberinFeed, BODY_COMET, BODY_NUMBER, INPOP_LINE_BODIES,
    PLANET_WEBERIN_TOL_M, WEBERIN_TOL_M,
};

const BIN_TTL_S: u64 = 604800;

fn ensure_bin(path: &str, netloc: &str, asset: &str, ttl: u64) -> Option<Vec<u8>> {
    if let Ok(bytes) = std::fs::read(path) {
        return Some(bytes);
    }
    if !path.starts_with("data/") {
        return None;
    }
    let url = format!("{}/{}/{}", CDN_BASE, netloc, asset);
    let bytes = fetch_raw_bytes(&url, ttl)?;
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(path, &bytes).is_err() {
        return None;
    }
    Some(bytes)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() {
    println!(
        "usage: weberin_body_verdict [--eph-dir <data-root>] [--dastcom <dastcom_asteroids.bin>] [--dcom5 <dcom5_comets.bin>] [--epoch <jd>] [--tol <m>]"
    );
}

fn cdn_parts(url: &str) -> Option<(String, String)> {
    let rest = url.strip_prefix(CDN_BASE)?.strip_prefix('/')?;
    let (netloc, asset) = rest.split_once('/')?;
    Some((netloc.to_string(), asset.to_string()))
}

fn verdict_line(name: &str, outcome: &BodyOutcome) -> String {
    match outcome {
        BodyOutcome::Placed { sep_m } => format!("weberin {name} state placed sep {sep_m:e}"),
        BodyOutcome::Absent { line } => {
            format!(
                "weberin {name} state absent sep absent missing {}",
                line.word()
            )
        }
        BodyOutcome::Riss { sep_m, knot } => format!(
            "weberin {name} state riss sep {sep_m:e} knot {}+{}",
            knot[0].word(),
            knot[1].word()
        ),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let eph_dir = match arg_value(&args, "--eph-dir") {
        Some(d) => d,
        None => "data".to_string(),
    };
    let dastcom_path = match arg_value(&args, "--dastcom") {
        Some(d) => d,
        None => "data/ssd.jpl.nasa.gov/dastcom_asteroids.bin".to_string(),
    };
    let dcom5_path = match arg_value(&args, "--dcom5") {
        Some(d) => d,
        None => "data/ssd.jpl.nasa.gov/dcom5_comets.bin".to_string(),
    };
    let tol_m = match arg_value(&args, "--tol").and_then(|w| w.parse::<f64>().ok()) {
        Some(t) if t.is_finite() && t > 0.0 => t,
        Some(_) => {
            eprintln!("weberin: --tol not a finite positive value — the weave stays closed");
            return;
        }
        None => WEBERIN_TOL_M,
    };
    let jd = match arg_value(&args, "--epoch").and_then(|w| w.parse::<f64>().ok()) {
        Some(j) if j.is_finite() && j > 0.0 => j,
        Some(_) => {
            eprintln!("weberin: --epoch not a finite JD — the weave stays closed");
            return;
        }
        None => {
            let time: Arc<Mutex<Option<LeapSeconds>>> = Arc::new(Mutex::new(embedded_lsk()));
            match system_now(&time) {
                Some(tdb) => tdb / 86400.0 + J2000_EPOCH,
                None => {
                    eprintln!("weberin: the system TDB epoch reads void (naif0012 leap table) — give --epoch <jd>");
                    return;
                }
            }
        }
    };
    let tdb = (jd - J2000_EPOCH) * 86400.0;

    let dastcom_bytes = match ensure_bin(
        &dastcom_path,
        CDN_RELEASE,
        "dastcom_asteroids.bin",
        BIN_TTL_S,
    ) {
        Some(b) => b,
        None => {
            eprintln!("weberin: {dastcom_path} bin void — absent on disk and the CDN fetch returned non-200 — the second body line stays unread");
            return;
        }
    };
    let recs: Vec<AsteroidRec> = dastcom_bytes
        .chunks_exact(RECORD_STRIDE)
        .filter_map(parse_record)
        .collect();
    if recs.is_empty() {
        eprintln!(
            "weberin: {dastcom_path} carries no {}-byte asteroid record",
            RECORD_STRIDE
        );
        return;
    }

    let comets: Vec<CometRec> = match ensure_bin(
        &dcom5_path,
        CDN_RELEASE,
        "dcom5_comets.bin",
        BIN_TTL_S,
    ) {
        Some(b) => b
            .chunks_exact(COMET_RECORD_BYTES)
            .filter_map(parse_comet_record)
            .collect(),
        None => {
            println!(
                    "weberin: {dcom5_path} bin void — absent on disk and the CDN fetch returned non-200 — the comet second line stays unread"
                );
            Vec::new()
        }
    };
    if comets.is_empty() {
        println!(
            "weberin: {dcom5_path} carries no {}-byte comet record",
            COMET_RECORD_BYTES
        );
    }

    println!("=== weberin — the second body line (dastcom/MPC Keplerian elements, INPOP SPK planets/moon) against the JPL SPK ephemeris points ===");

    let sources = load_sources();
    let mut bodies: Vec<(String, SourceConfig)> = sources
        .into_iter()
        .filter(|s| s.format == "ephemeris_binary" || s.format == "orbit_bin")
        .filter_map(|s| s.body.clone().map(|b| (b, s)))
        .collect();
    bodies.sort_by(|a, b| a.0.cmp(&b.0));
    if bodies.is_empty() {
        println!("weberin: phi/sources.φ carries no ephemeris_binary/orbit_bin body — the body chain is void");
        return;
    }
    println!("dastcom {dastcom_path}: {} numbered-asteroid record(s) read | dcom5 {dcom5_path}: {} comet record(s) read | weave epoch jd {jd:.5} (tdb {tdb:.3} s past J2000) | tolerance {tol_m:.3e} m (ephemeris-vs-kepler line) + {PLANET_WEBERIN_TOL_M:.3e} m (de-vs-inpop line) | {} registered body worldline(s) from phi/sources.φ | the body set is the union of the registered SPK/orbit bodies, the {}-body dastcom table and the {}-comet dcom5 map", recs.len(), comets.len(), bodies.len(), BODY_NUMBER.len(), BODY_COMET.len());

    let Some(lsk) = embedded_lsk() else {
        println!(
            "weberin: the embedded leap-second table reads void — the body line cannot be judged"
        );
        return;
    };
    let mut sun_map: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut opened = 0usize;
    for (name, src) in &bodies {
        let Some((netloc, asset)) = cdn_parts(&src.url) else {
            println!(
                "weberin {name}: register url {} is not a CDN release path — the body line is not read",
                src.url
            );
            continue;
        };
        let path = format!("{eph_dir}/{netloc}/{asset}");
        if ensure_bin(&path, &netloc, &asset, BIN_TTL_S).is_none() {
            println!("weberin {name} bin void {path} — absent on disk and the CDN fetch returned non-200");
            continue;
        }
        match extract(src, &path, tdb, &lsk) {
            ExtractResult::WithEphemeris(_, body_eph) => {
                if name == "sun" {
                    sun_map.insert(name.clone(), body_eph.clone());
                }
                eph.insert(name.clone(), body_eph);
                opened += 1;
            }
            _ => println!("weberin {name}: {path} reads but does not parse to a BodyEphemeris"),
        }
    }
    if sun_map.is_empty() {
        println!("weberin: the sun reference is void — the heliocentric dastcom line cannot fold to the barycentric frame");
        return;
    }

    const INPOP_NETLOC: &str = "ftp.imcce.fr";
    let mut inpop_map: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut opened_inpop = 0usize;
    for name in INPOP_LINE_BODIES {
        let asset = format!("ephemeris_inpop_{}.bin", name);
        let path = format!("{eph_dir}/{INPOP_NETLOC}/{asset}");
        let Some(bytes) = ensure_bin(&path, INPOP_NETLOC, &asset, BIN_TTL_S) else {
            println!("weberin {name} inpop bin void {path} — absent on disk and the CDN fetch returned non-200 — the INPOP line stays unread");
            continue;
        };
        match parse_ephemeris_binary(&bytes) {
            Some(e) => {
                inpop_map.insert((*name).to_string(), e);
                opened_inpop += 1;
            }
            None => println!("weberin {name}: {path} reads but does not parse to a BodyEphemeris"),
        }
    }

    let mut w = Weberin::new();
    w.feed(WeberinFeed {
        eph: Arc::new(eph),
        sun: Arc::new(sun_map),
        eph_inpop: Arc::new(inpop_map),
        recs,
        comets,
    });
    w.weave(tdb, tol_m);
    if !w.woven {
        eprintln!("weberin: the weave did not run — no body line is judged");
        return;
    }

    let mut placed = 0usize;
    let mut absent = 0usize;
    let mut riss = 0usize;
    for v in &w.verdicts {
        match &v.outcome {
            BodyOutcome::Placed { .. } => placed += 1,
            BodyOutcome::Absent { .. } => absent += 1,
            BodyOutcome::Riss { .. } => riss += 1,
        }
        println!("{}", verdict_line(&v.name, &v.outcome));
    }
    println!(
        "weberin tally: {opened}/{} registered body bin(s) opened | {opened_inpop}/{} INPOP body bin(s) opened | {} body line(s) judged | placed {placed} | absent {absent} | riss {riss}",
        bodies.len(),
        INPOP_LINE_BODIES.len(),
        w.verdicts.len(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::weberin::BodyLine;

    #[test]
    fn placed_line_carries_the_measured_separation() {
        let o = BodyOutcome::Placed { sep_m: 1.5e4 };
        assert_eq!(
            verdict_line("ceres", &o),
            "weberin ceres state placed sep 1.5e4"
        );
    }

    #[test]
    fn absent_line_names_the_missing_line() {
        let o = BodyOutcome::Absent {
            line: BodyLine::Dastcom,
        };
        assert_eq!(
            verdict_line("vesta", &o),
            "weberin vesta state absent sep absent missing dastcom-keplerian"
        );
        let o = BodyOutcome::Absent {
            line: BodyLine::Spk,
        };
        assert_eq!(
            verdict_line("pluto", &o),
            "weberin pluto state absent sep absent missing spk-ephemeris"
        );
    }

    #[test]
    fn riss_line_names_both_refusing_threads() {
        let o = BodyOutcome::Riss {
            sep_m: 2.3e9,
            knot: [BodyLine::Spk, BodyLine::Dastcom],
        };
        assert_eq!(
            verdict_line("apophis", &o),
            "weberin apophis state riss sep 2.3e9 knot spk-ephemeris+dastcom-keplerian"
        );
    }

    #[test]
    fn absent_inpop_line_names_the_inpop_ephemeris() {
        let o = BodyOutcome::Absent {
            line: BodyLine::Inpop,
        };
        assert_eq!(
            verdict_line("uranus", &o),
            "weberin uranus state absent sep absent missing inpop-ephemeris"
        );
    }

    #[test]
    fn riss_line_names_the_inpop_knot() {
        let o = BodyOutcome::Riss {
            sep_m: 1.6e6,
            knot: [BodyLine::Spk, BodyLine::Inpop],
        };
        assert_eq!(
            verdict_line("neptune", &o),
            "weberin neptune state riss sep 1.6e6 knot spk-ephemeris+inpop-ephemeris"
        );
    }
}
