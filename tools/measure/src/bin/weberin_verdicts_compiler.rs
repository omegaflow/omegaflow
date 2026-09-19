use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use omegaflow::archivar::{
    BodyEphemeris, ExtractResult, J2000_EPOCH, LeapSeconds, SourceConfig, VerdictLine, VerdictWord,
    embedded_lsk, encode_weberin_verdicts, extract, fetch_raw_bytes, load_sources,
    parse_ephemeris_binary, system_now,
};
use omegaflow::cdn::{CDN_BASE, CDN_RELEASE, upload_asset};
use omegaflow::dastcom::{
    AsteroidRec, COMET_RECORD_BYTES, CometRec, RECORD_STRIDE, parse_comet_record, parse_record,
};
use omegaflow::weberin::{
    BodyOutcome, EPM_LINE_BODIES, INPOP_LINE_BODIES, ThreeWayVerdict, TriadFold, Weberin,
    WeberinFeed,
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
        "usage: weberin_verdicts_compiler [--eph-dir <data-root>] [--dastcom <dastcom_asteroids.bin>] [--dcom5 <dcom5_comets.bin>] [--epoch <jd>] [--tol <m>] [--out <path>] [--ci-mode]"
    );
}

fn cdn_parts(url: &str) -> Option<(String, String)> {
    let rest = url.strip_prefix(CDN_BASE)?.strip_prefix('/')?;
    let (netloc, asset) = rest.split_once('/')?;
    Some((netloc.to_string(), asset.to_string()))
}

fn body_verdict_line(name: &str, outcome: &BodyOutcome, weave_epoch: f64) -> VerdictLine {
    match outcome {
        BodyOutcome::Placed { sep_m } => VerdictLine {
            name: name.to_string(),
            word: VerdictWord::Placed,
            knot: [None, None],
            sep_m: Some(*sep_m),
            weave_epoch,
        },
        BodyOutcome::Absent { line } => VerdictLine {
            name: name.to_string(),
            word: VerdictWord::Absent,
            knot: [Some(*line), None],
            sep_m: None,
            weave_epoch,
        },
        BodyOutcome::Riss { sep_m, knot } => VerdictLine {
            name: name.to_string(),
            word: VerdictWord::Riss,
            knot: [Some(knot[0]), Some(knot[1])],
            sep_m: Some(*sep_m),
            weave_epoch,
        },
    }
}

fn triad_verdict_line(t: &ThreeWayVerdict, weave_epoch: f64) -> VerdictLine {
    match t.fold {
        TriadFold::United => VerdictLine {
            name: t.name.clone(),
            word: VerdictWord::Placed,
            knot: [None, None],
            sep_m: None,
            weave_epoch,
        },
        TriadFold::Shared { line } => VerdictLine {
            name: t.name.clone(),
            word: VerdictWord::Placed,
            knot: [Some(line), None],
            sep_m: None,
            weave_epoch,
        },
        TriadFold::Outlier { knot, .. } => VerdictLine {
            name: t.name.clone(),
            word: VerdictWord::Riss,
            knot: [Some(knot[0]), Some(knot[1])],
            sep_m: None,
            weave_epoch,
        },
        TriadFold::Severed => VerdictLine {
            name: t.name.clone(),
            word: VerdictWord::Absent,
            knot: [None, None],
            sep_m: None,
            weave_epoch,
        },
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
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
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => "data/weberin_verdicts.bin".to_string(),
    };
    let tol_m = match arg_value(&args, "--tol").and_then(|w| w.parse::<f64>().ok()) {
        Some(t) if t.is_finite() && t > 0.0 => t,
        Some(_) => {
            eprintln!(
                "weberin-verdicts: --tol not a finite positive value — the weave stays closed"
            );
            return;
        }
        None => omegaflow::weberin::WEBERIN_TOL_M,
    };
    let jd = match arg_value(&args, "--epoch").and_then(|w| w.parse::<f64>().ok()) {
        Some(j) if j.is_finite() && j > 0.0 => j,
        Some(_) => {
            eprintln!("weberin-verdicts: --epoch not a finite JD — the weave stays closed");
            return;
        }
        None => {
            let time: Arc<Mutex<Option<LeapSeconds>>> = Arc::new(Mutex::new(embedded_lsk()));
            match system_now(&time) {
                Some(tdb) => tdb / 86400.0 + J2000_EPOCH,
                None => {
                    eprintln!(
                        "weberin-verdicts: the system TDB epoch reads void (naif0012 leap table) — give --epoch <jd>"
                    );
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
            eprintln!(
                "weberin-verdicts: {dastcom_path} bin void — absent on disk and the CDN fetch returned non-200 — the second body line stays unread"
            );
            return;
        }
    };
    let recs: Vec<AsteroidRec> = dastcom_bytes
        .chunks_exact(RECORD_STRIDE)
        .filter_map(parse_record)
        .collect();
    if recs.is_empty() {
        eprintln!(
            "weberin-verdicts: {dastcom_path} carries no {}-byte asteroid record",
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
                "weberin-verdicts: {dcom5_path} bin void — absent on disk and the CDN fetch returned non-200 — the comet second line stays unread"
            );
            Vec::new()
        }
    };

    let sources = load_sources();
    let mut bodies: Vec<(String, SourceConfig)> = sources
        .into_iter()
        .filter(|s| s.format == "ephemeris_binary" || s.format == "orbit_bin")
        .filter_map(|s| s.body.clone().map(|b| (b, s)))
        .collect();
    bodies.sort_by(|a, b| a.0.cmp(&b.0));
    if bodies.is_empty() {
        println!(
            "weberin-verdicts: phi/sources.φ carries no ephemeris_binary/orbit_bin body — the body chain is void"
        );
        return;
    }

    let Some(lsk) = embedded_lsk() else {
        println!(
            "weberin-verdicts: the embedded leap-second table reads void — the body line cannot be judged"
        );
        return;
    };
    let mut sun_map: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut opened = 0usize;
    for (name, src) in &bodies {
        let Some((netloc, asset)) = cdn_parts(&src.url) else {
            println!(
                "weberin-verdicts {name}: register url {} is not a CDN release path — the body line is not read",
                src.url
            );
            continue;
        };
        let path = format!("{eph_dir}/{netloc}/{asset}");
        if ensure_bin(&path, &netloc, &asset, BIN_TTL_S).is_none() {
            println!(
                "weberin-verdicts {name} bin void {path} — absent on disk and the CDN fetch returned non-200"
            );
            continue;
        }
        match extract(src, &path, tdb, &lsk) {
            ExtractResult::WithEphemeris(_, body_eph) => {
                if name == "sun" {
                    sun_map.insert(name.clone(), (*body_eph).clone());
                }
                eph.insert(name.clone(), *body_eph);
                opened += 1;
            }
            _ => println!(
                "weberin-verdicts {name}: {path} reads but does not parse to a BodyEphemeris"
            ),
        }
    }
    if sun_map.is_empty() {
        println!(
            "weberin-verdicts: the sun reference is void — the heliocentric dastcom line cannot fold to the barycentric frame"
        );
        return;
    }

    const INPOP_NETLOC: &str = "ftp.imcce.fr";
    let mut inpop_map: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut opened_inpop = 0usize;
    for name in INPOP_LINE_BODIES {
        let asset = format!("ephemeris_inpop_{}.bin", name);
        let path = format!("{eph_dir}/{INPOP_NETLOC}/{asset}");
        let Some(bytes) = ensure_bin(&path, INPOP_NETLOC, &asset, BIN_TTL_S) else {
            println!(
                "weberin-verdicts {name} inpop bin void {path} — absent on disk and the CDN fetch returned non-200 — the INPOP line stays unread"
            );
            continue;
        };
        match parse_ephemeris_binary(&bytes) {
            Some(e) => {
                inpop_map.insert((*name).to_string(), e);
                opened_inpop += 1;
            }
            None => println!(
                "weberin-verdicts {name}: {path} reads but does not parse to a BodyEphemeris"
            ),
        }
    }

    const EPM_NETLOC: &str = "ftp.iaaras.ru";
    let mut epm_map: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut opened_epm = 0usize;
    for name in EPM_LINE_BODIES {
        let asset = format!("ephemeris_epm_{}.bin", name);
        let path = format!("{eph_dir}/{EPM_NETLOC}/{asset}");
        let Some(bytes) = ensure_bin(&path, EPM_NETLOC, &asset, BIN_TTL_S) else {
            println!(
                "weberin-verdicts {name} epm bin void {path} — absent on disk and the CDN fetch returned non-200 — the EPM line stays unread"
            );
            continue;
        };
        match parse_ephemeris_binary(&bytes) {
            Some(e) => {
                epm_map.insert((*name).to_string(), e);
                opened_epm += 1;
            }
            None => println!(
                "weberin-verdicts {name}: {path} reads but does not parse to a BodyEphemeris"
            ),
        }
    }

    let rec_count = recs.len();
    let comet_count = comets.len();
    let mut w = Weberin::new();
    w.feed(WeberinFeed {
        eph: Arc::new(eph),
        sun: Arc::new(sun_map),
        eph_inpop: Arc::new(inpop_map),
        eph_epm: Arc::new(epm_map),
        recs,
        comets,
        mpc_recs: Vec::new(),
    });
    w.weave(tdb, tol_m);
    if !w.woven {
        eprintln!("weberin-verdicts: the weave did not run — no body line is judged");
        return;
    }

    let mut lines: Vec<VerdictLine> = Vec::new();
    for v in &w.verdicts {
        lines.push(body_verdict_line(&v.name, &v.outcome, tdb));
    }
    for v in &w.mpc_verdicts {
        lines.push(body_verdict_line(&v.name, &v.outcome, tdb));
    }
    for t in &w.triads {
        lines.push(triad_verdict_line(t, tdb));
    }

    if lines.is_empty() {
        println!(
            "weberin-verdicts: no body line judged — {opened}/{} registered SPK/orbit bin(s), {opened_inpop}/{} INPOP bin(s), {opened_epm}/{} EPM bin(s), {rec_count} asteroid record(s), {comet_count} comet record(s) read; no verdict bin written",
            bodies.len(),
            INPOP_LINE_BODIES.len(),
            EPM_LINE_BODIES.len()
        );
        return;
    }

    let riss = lines.iter().filter(|l| l.word == VerdictWord::Riss).count();
    let absent = lines
        .iter()
        .filter(|l| l.word == VerdictWord::Absent)
        .count();
    let placed = lines
        .iter()
        .filter(|l| l.word == VerdictWord::Placed)
        .count();
    let bytes = encode_weberin_verdicts(&lines);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!(
            "weberin-verdicts: {out} did not take the {} byte(s) — the verdict bin is not on disk",
            bytes.len()
        );
        return;
    }
    println!(
        "weberin_verdicts: {} lines ({riss} riss, {absent} absent, {placed} placed) -> {out}",
        lines.len()
    );
    if ci_mode && !upload_asset(&out) {
        eprintln!(
            "weberin_verdicts: {out} did not reach the CDN release {CDN_RELEASE} — the verdict bin stands local, the manifest is pending"
        );
    }
}
