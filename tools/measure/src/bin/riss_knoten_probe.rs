use std::collections::HashMap;

use omegaflow::archivar::{
    angular_distance_deg, body_barycenter_position, embedded_lsk, parse_ephemeris_binary,
    parse_json, BodyEphemeris, JsonVal, J2000_EPOCH,
};
use omegaflow::dastcom::{parse_record, state_at, AsteroidRec, RECORD_STRIDE};
use omegaflow::weberin::{add_sun, classify, separation_m, Agreement, BODY_NUMBER, WEBERIN_TOL_M};
use omegaflow_measure::weberin::borrowed_sense::{simbad_otype_known, FINK_LSST_CLASS_ABSENT};
use omegaflow_measure::weberin::nadel_gate::{
    borrowed_gate, GateWord, WiseRead, AGN_WEDGE_W1_W2, WISE_AGN_CITE, WISE_RADIUS_ARCSEC,
};

const DEFAULT_STATION: &str = "ABK";
const FILL_NT: f64 = 99999.0;
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const DEFAULT_RADIUS_DEG: f64 = 5.0;
const ENCOUNTER_MIN_LIMIT_S: f64 = 3.0 * MINUTE;
const GROUND_SIGMA_WINDOW_S: f64 = 30.0 * MINUTE;
const SWARM_SIGMA_WINDOW_S: f64 = 60.0;
const SIGMA_K: f64 = 2.0;

#[derive(Clone, Copy, PartialEq, Debug)]
enum LedgerState {
    Zwirn,
    Riss,
    Absent,
}

impl LedgerState {
    fn word(&self) -> &'static str {
        match self {
            LedgerState::Zwirn => "zwirn",
            LedgerState::Riss => "riss",
            LedgerState::Absent => "absent",
        }
    }
}

fn ledger_line(
    pair: &str,
    a: (&str, &str),
    b: (&str, &str),
    state: LedgerState,
    values: &str,
) -> String {
    format!(
        "riss knot {pair} state {} lines {}({}) + {}({}) values {values}",
        state.word(),
        a.0,
        a.1,
        b.0,
        b.1
    )
}

fn body_ledger(name: &str, state: LedgerState, values: &str) -> String {
    ledger_line(
        &format!("body-{name}"),
        ("spk-ephemeris", "spk-granules"),
        ("dastcom-keplerian", "dastcom-elements"),
        state,
        values,
    )
}

fn station_ledger(code: &str, state: LedgerState, values: &str) -> String {
    ledger_line(
        &format!("station-{code}"),
        ("intermagnet-ground", "intermagnet-xyzf-best-avail"),
        ("swarm-overflight", "swarm-maga_lr-1b-scalar-f"),
        state,
        values,
    )
}

fn borrowed_ledger(
    subject: &str,
    simbad: Option<&str>,
    wise: Option<WiseRead>,
    broker_class: Option<i64>,
) -> String {
    let simbad_known = simbad_otype_known(simbad);
    let (line, prov) = if simbad_known {
        ("simbad-window", "simbad-otype-crossmatch")
    } else if wise.is_some() {
        ("allwise-window", "allwise-w1-w2-wedge")
    } else {
        ("independent-window", "simbad-otype|allwise-w1-w2")
    };
    let (_, gate, _) = borrowed_gate(simbad_known, wise, broker_class);
    let state = match gate {
        GateWord::Zwirn => LedgerState::Zwirn,
        GateWord::Riss => LedgerState::Riss,
        _ => LedgerState::Absent,
    };
    let broker_val = match broker_class {
        Some(c) if c != FINK_LSST_CLASS_ABSENT => format!("broker class {c}"),
        _ => "broker class absent".to_string(),
    };
    let window_val = match simbad {
        Some(o) if simbad_otype_known(Some(o)) => format!("simbad otype {o}"),
        _ => match wise {
            Some(WiseRead::Agn) => {
                format!("allwise w1-w2 >= {AGN_WEDGE_W1_W2:.1} (agn wedge, {WISE_AGN_CITE})")
            }
            Some(WiseRead::FieldSource) => {
                format!("allwise w1-w2 < {AGN_WEDGE_W1_W2:.1} (field source)")
            }
            Some(WiseRead::NoSource) => {
                format!("allwise no source within {WISE_RADIUS_ARCSEC:.0} arcsec (0 honored)")
            }
            Some(WiseRead::Pending) | None => "independent window absent".to_string(),
        },
    };
    let tail = match gate {
        GateWord::Zwirn => " the two independent voices agree (zwirn)",
        GateWord::Riss => " the two independent voices refuse to converge — the riss stays visible (never smoothed)",
        _ => "",
    };
    let values = format!("{broker_val}; {window_val}{tail}");
    ledger_line(
        &format!("borrowed-{subject}"),
        ("broker-classifier", "fink-lsst-main_label_classifier"),
        (line, prov),
        state,
        &values,
    )
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else {
        s.split_once(' ')?
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * MINUTE + ss as f64)
}

fn iso_utc(unix: f64) -> String {
    let total = (unix.max(0.0) / DAY).floor() as i64;
    let day_secs = unix.max(0.0) - total as f64 * DAY;
    let z = total + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let hh = (day_secs / HOUR) as i64;
    let mm = ((day_secs - hh as f64 * HOUR) / MINUTE) as i64;
    let ss = (day_secs - hh as f64 * HOUR - mm as f64 * MINUTE) as i64;
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

#[derive(Clone)]
struct Row {
    t: f64,
    f: f64,
    lat: Option<f64>,
    lon: Option<f64>,
}

struct Encounter {
    epoch: f64,
    distance_deg: f64,
    ground_f: f64,
    swarm_f: f64,
    ground_sigma: f64,
    swarm_sigma: f64,
}

fn parse_vector_rows(text: &str) -> Vec<Row> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let JsonVal::Obj(map) = json else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(data)) = map.get("data") else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        if cells.len() < 2 {
            continue;
        }
        let Some(t) = (match &cells[0] {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let JsonVal::Arr(comps) = &cells[1] else {
            continue;
        };
        if comps.len() < 3 {
            continue;
        }
        let mut f = 0.0;
        let mut ok = true;
        for c in comps.iter().take(3) {
            match c {
                JsonVal::Num(n) if n.is_finite() && *n != FILL_NT => {
                    f += n * n;
                }
                _ => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let mag = f.sqrt();
        if mag.is_finite() && mag > 0.0 {
            rows.push(Row {
                t,
                f: mag,
                lat: None,
                lon: None,
            });
        }
    }
    rows.sort_by(|a, b| a.t.total_cmp(&b.t));
    rows
}

fn parse_scalar_rows(text: &str) -> Vec<Row> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let JsonVal::Obj(map) = json else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(data)) = map.get("data") else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        if cells.len() < 2 {
            continue;
        }
        let Some(t) = (match &cells[0] {
            JsonVal::Str(s) => iso_to_unix(s),
            _ => None,
        }) else {
            continue;
        };
        let nums: Vec<f64> = cells[1..]
            .iter()
            .filter_map(|c| match c {
                JsonVal::Num(n) if n.is_finite() => Some(*n),
                _ => None,
            })
            .collect();
        let row = match nums.len() {
            1 => Row {
                t,
                f: nums[0],
                lat: None,
                lon: None,
            },
            3 => Row {
                t,
                f: nums[2],
                lat: Some(nums[0]),
                lon: Some(nums[1]),
            },
            _ => continue,
        };
        if row.f.is_finite() && row.f != FILL_NT {
            rows.push(row);
        }
    }
    rows.sort_by(|a, b| a.t.total_cmp(&b.t));
    rows
}

fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

fn std(xs: &[f64]) -> Option<f64> {
    if xs.len() < 2 {
        return None;
    }
    let m = mean(xs)?;
    let v = xs.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / xs.len() as f64;
    Some(v.sqrt())
}

fn encounter(
    ground: &[Row],
    swarm: &[Row],
    st_lat: f64,
    st_lon: f64,
    radius_deg: f64,
) -> Option<Encounter> {
    let mut best: Option<(f64, usize)> = None;
    for (i, r) in swarm.iter().enumerate() {
        let (Some(la), Some(lo)) = (r.lat, r.lon) else {
            continue;
        };
        let d = angular_distance_deg(st_lat, st_lon, la, lo);
        if best.map_or(true, |(bd, _)| d < bd) {
            best = Some((d, i));
        }
    }
    let (dist, si) = best?;
    if !(dist <= radius_deg) {
        return None;
    }
    let enc_t = swarm[si].t;
    let swarm_f = swarm[si].f;
    let mut nearest: Option<(f64, usize)> = None;
    for (i, r) in ground.iter().enumerate() {
        let d = (r.t - enc_t).abs();
        if nearest.map_or(true, |(bd, _)| d < bd) {
            nearest = Some((d, i));
        }
    }
    let (gd, gi) = nearest?;
    if gd > ENCOUNTER_MIN_LIMIT_S {
        return None;
    }
    let ground_f = ground[gi].f;
    let g_win: Vec<f64> = ground
        .iter()
        .filter(|r| (r.t - enc_t).abs() <= GROUND_SIGMA_WINDOW_S)
        .map(|r| r.f)
        .collect();
    let s_win: Vec<f64> = swarm
        .iter()
        .filter(|r| (r.t - enc_t).abs() <= SWARM_SIGMA_WINDOW_S)
        .map(|r| r.f)
        .collect();
    let ground_sigma = std(&g_win).unwrap_or(f64::NAN);
    let swarm_sigma = std(&s_win).unwrap_or(f64::NAN);
    Some(Encounter {
        epoch: enc_t,
        distance_deg: dist,
        ground_f,
        swarm_f,
        ground_sigma,
        swarm_sigma,
    })
}

fn station_state(present: bool, dev: f64, tol: Option<f64>) -> LedgerState {
    if !present {
        return LedgerState::Absent;
    }
    match tol {
        Some(t) if t.is_finite() && t >= 0.0 => {
            if dev <= t {
                LedgerState::Zwirn
            } else {
                LedgerState::Riss
            }
        }
        _ => {
            if dev == 0.0 {
                LedgerState::Zwirn
            } else {
                LedgerState::Riss
            }
        }
    }
}

fn station_match_values(e: &Encounter, dev: f64, tol: Option<f64>) -> String {
    let base = format!(
        "ground {:.1} nT swarm {:.1} nT deviation {:.1} nT overflight {:.2} deg at {}",
        e.ground_f,
        e.swarm_f,
        dev,
        e.distance_deg,
        iso_utc(e.epoch)
    );
    match tol {
        Some(t) if t.is_finite() && t >= 0.0 => format!("{base} tolerance {t:.1} nT"),
        _ => format!("{base} tolerance absent (sigma windows unmeasured)"),
    }
}

fn station_ledger_for_match(code: &str, e: &Encounter, dev: f64, tol: Option<f64>) -> String {
    let values = station_match_values(e, dev, tol);
    station_ledger(code, station_state(true, dev, tol), &values)
}

fn read_body_spk(eph_dir: &str, name: &str) -> Option<BodyEphemeris> {
    let path = format!("{eph_dir}/ephemeris_{name}.bin");
    let bytes = std::fs::read(&path).ok()?;
    parse_ephemeris_binary(&bytes)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name).and_then(|w| w.parse::<f64>().ok())
}

fn epoch_from_args(args: &[String]) -> Option<f64> {
    match arg_value(args, "--epoch").and_then(|w| w.parse::<f64>().ok()) {
        Some(j) if j.is_finite() && j > 0.0 => Some(j),
        _ => embedded_lsk()
            .and_then(|l| l.system_now_tdb())
            .map(|tdb| tdb / 86400.0 + J2000_EPOCH),
    }
}

fn usage() {
    println!("usage: riss_knoten_probe [--grammar] [--help]");
    println!(
        "  body pair (per body):    --dastcom <bin> --eph-dir <dir> [--epoch <jd>] [--tol <m>]"
    );
    println!("  station pair:            --station <code> --lat <deg> --lon <deg> --ground <hapi.json> --swarm <hapi.json> [--radius <deg>] [--tolerance-nT <nt>]");
    println!("  borrowed-sense pair:     --dia <id> | (--ra <deg> --dec <deg>) [--broker-class <i64>] [--simbad <otype>] [--wise agn|field|nosource]");
    println!("  defaults: eph-dir data/ssd.jpl.nasa.gov, dastcom data/ssd.jpl.nasa.gov/dastcom_asteroids.bin, epoch = the system TDB, station {DEFAULT_STATION} (a missing input leaves that pair absent — never fabricated)");
}

fn grammar() {
    println!("=== the riss-knoten ledger grammar (one line per pair, zwirn / riss / absent) ===");
    println!(
        "{}",
        body_ledger(
            "ceres",
            LedgerState::Zwirn,
            "sep 2.700e4 m tol 1.000e6 m jd 2461544.50000"
        )
    );
    println!(
        "{}",
        body_ledger(
            "apophis",
            LedgerState::Riss,
            "sep 2.300e9 m tol 1.000e6 m jd 2461544.50000"
        )
    );
    println!(
        "{}",
        body_ledger(
            "vesta",
            LedgerState::Absent,
            "missing dastcom-keplerian (record 4 not in the bin)"
        )
    );
    let placed_enc = Encounter {
        epoch: 1.76616e9,
        distance_deg: 0.02,
        ground_f: 53699.2,
        swarm_f: 53700.6,
        ground_sigma: 0.4,
        swarm_sigma: 0.6,
    };
    println!(
        "{}",
        station_ledger_for_match("ABK", &placed_enc, 1.4, Some(4.8))
    );
    let riss_enc = Encounter {
        epoch: 1.76616e9,
        distance_deg: 0.02,
        ground_f: 53699.2,
        swarm_f: 44510.0,
        ground_sigma: 0.4,
        swarm_sigma: 0.6,
    };
    println!(
        "{}",
        station_ledger_for_match("ABK", &riss_enc, 9189.2, Some(4.8))
    );
    println!("{}", station_ledger("ABK", LedgerState::Absent, "ground line unread (--ground cache json absent) — the station carries fewer than two independent matched lines"));
    println!(
        "{}",
        borrowed_ledger("ZTF26abcdefg", Some("SN"), None, Some(11))
    );
    println!(
        "{}",
        borrowed_ledger("ZTF26abcdefg", None, Some(WiseRead::FieldSource), Some(11))
    );
    println!("{}", borrowed_ledger("unregistered", None, None, None));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if flag(&args, "--help") || flag(&args, "-h") {
        usage();
        return;
    }
    if flag(&args, "--grammar") {
        grammar();
        return;
    }
    let mut n_pairs = 0usize;
    let mut n_zwirn = 0usize;
    let mut n_riss = 0usize;
    let mut n_absent = 0usize;

    let eph_dir = match arg_value(&args, "--eph-dir") {
        Some(d) => d,
        None => "data/ssd.jpl.nasa.gov".to_string(),
    };
    let dastcom_path = match arg_value(&args, "--dastcom") {
        Some(d) => d,
        None => format!("{eph_dir}/dastcom_asteroids.bin"),
    };
    let tol_m = match arg_f64(&args, "--tol") {
        Some(t) if t.is_finite() && t > 0.0 => t,
        _ => WEBERIN_TOL_M,
    };
    let jd = epoch_from_args(&args);

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let mut opened = 0usize;
    for (name, _) in BODY_NUMBER {
        if let Some(e) = read_body_spk(&eph_dir, name) {
            eph.insert((*name).to_string(), e);
            opened += 1;
        }
    }
    if let Some(sun) = read_body_spk(&eph_dir, "sun") {
        eph.insert("sun".to_string(), sun);
    }
    let mut recs: HashMap<u32, AsteroidRec> = HashMap::new();
    let mut dastcom_records = 0usize;
    match std::fs::read(&dastcom_path) {
        Ok(bytes) => {
            for r in bytes.chunks_exact(RECORD_STRIDE).filter_map(parse_record) {
                recs.insert(r.number, r);
                dastcom_records += 1;
            }
        }
        Err(_) => {}
    }
    let jd_word = match jd {
        Some(j) => format!("{j:.5}"),
        None => "unresolved".to_string(),
    };
    println!("=== the riss-knoten ledger — one line per pair of independent threads, each incompatibility named and kept visible (the Riss stays visible, never smoothed) ===");
    println!(
        "body pair: {opened}/{} SPK ephemeris bin(s) opened in {eph_dir} | {dastcom_records} dastcom record(s) in {dastcom_path} | weave epoch jd {jd_word} | tolerance {tol_m:.3e} m",
        BODY_NUMBER.len()
    );

    for (name, num) in BODY_NUMBER {
        n_pairs += 1;
        let path = format!("{eph_dir}/ephemeris_{name}.bin");
        let rec = recs.get(num).cloned();
        let (Some(jd_v), Some(tdb)) = (jd, jd.map(|j| (j - J2000_EPOCH) * 86400.0)) else {
            n_absent += 1;
            println!(
                "{}",
                body_ledger(
                    name,
                    LedgerState::Absent,
                    "weave epoch unresolved — no sep measurable"
                )
            );
            continue;
        };
        let spk_bary = body_barycenter_position(name, tdb, &eph);
        let helio = rec.as_ref().and_then(|r| state_at(r, jd_v)).map(|(p, _)| p);
        let sun_bary = body_barycenter_position("sun", tdb, &eph);
        let (state, values) = match spk_bary {
            None => (
                LedgerState::Absent,
                format!("spk-ephemeris value absent — unread at {path} or outside the granule coverage"),
            ),
            Some(spk) => match helio {
                None => (
                    LedgerState::Absent,
                    format!("dastcom-keplerian value absent — record {num} unread or not evaluable at jd {jd_v:.5}"),
                ),
                Some(helio) => match sun_bary {
                    None => (
                        LedgerState::Absent,
                        "sun spk reference absent — the heliocentric dastcom-keplerian line cannot fold"
                            .to_string(),
                    ),
                    Some(sun) => {
                        let sep = separation_m(spk, add_sun(helio, sun));
                        let state = match classify(sep, tol_m) {
                            Agreement::Placed { .. } => LedgerState::Zwirn,
                            Agreement::Riss { .. } => LedgerState::Riss,
                        };
                        (
                            state,
                            format!("sep {sep:.3e} m tol {tol_m:.3e} m jd {jd_v:.5}"),
                        )
                    }
                },
            },
        };
        match state {
            LedgerState::Zwirn => n_zwirn += 1,
            LedgerState::Riss => n_riss += 1,
            LedgerState::Absent => n_absent += 1,
        }
        println!("{}", body_ledger(name, state, &values));
    }

    n_pairs += 1;
    let station = match arg_value(&args, "--station") {
        Some(s) => s,
        None => DEFAULT_STATION.to_string(),
    };
    let point = match (arg_f64(&args, "--lat"), arg_f64(&args, "--lon")) {
        (Some(la), Some(lo)) if la.is_finite() && lo.is_finite() => Some((la, lo)),
        _ => None,
    };
    let line = match point {
        None => station_ledger(
            &station,
            LedgerState::Absent,
            "station point absent (--lat/--lon not given and no local register read) — the overflight geometry stays unmeasured",
        ),
        Some((lat, lon)) => {
            let radius = match arg_f64(&args, "--radius") {
                Some(r) if r.is_finite() && r > 0.0 => r,
                _ => DEFAULT_RADIUS_DEG,
            };
            let tol_arg = arg_f64(&args, "--tolerance-nT");
            let ground_rows: Vec<Row> = match arg_value(&args, "--ground") {
                Some(p) => match std::fs::read_to_string(&p) {
                    Ok(t) => parse_vector_rows(&t),
                    Err(_) => Vec::new(),
                },
                None => Vec::new(),
            };
            let swarm_rows: Vec<Row> = match arg_value(&args, "--swarm") {
                Some(p) => match std::fs::read_to_string(&p) {
                    Ok(t) => parse_scalar_rows(&t),
                    Err(_) => Vec::new(),
                },
                None => Vec::new(),
            };
            let ground_present = !ground_rows.is_empty();
            let swarm_present = !swarm_rows.is_empty();
            let swarm_geometry = swarm_rows.iter().any(|r| r.lat.is_some() && r.lon.is_some());
            if !ground_present {
                station_ledger(
                    &station,
                    LedgerState::Absent,
                    "ground line absent (the intermagnet xyzf cache json is unread) — fewer than two independent matched lines at the point",
                )
            } else if !swarm_present {
                station_ledger(
                    &station,
                    LedgerState::Absent,
                    "swarm line absent (the swarm scalar-f cache json is unread) — fewer than two independent matched lines at the point",
                )
            } else if !swarm_geometry {
                station_ledger(
                    &station,
                    LedgerState::Absent,
                    "swarm cache carries time+f only — the overflight geometry is not local (the station point match stays absent)",
                )
            } else {
                match encounter(&ground_rows, &swarm_rows, lat, lon, radius) {
                    None => station_ledger(
                        &station,
                        LedgerState::Absent,
                        &format!(
                            "no swarm overflight within {radius} deg of the point matched to a ground minute in the local window — fewer than two matched lines at the point"
                        ),
                    ),
                    Some(e) => {
                        let dev = (e.ground_f - e.swarm_f).abs();
                        let sigma_tol =
                            if e.ground_sigma.is_finite() && e.swarm_sigma.is_finite() {
                                Some(SIGMA_K
                                    * (e.ground_sigma * e.ground_sigma
                                        + e.swarm_sigma * e.swarm_sigma)
                                        .sqrt())
                            } else {
                                None
                            };
                        let tol = tol_arg.or(sigma_tol);
                        station_ledger_for_match(&station, &e, dev, tol)
                    }
                }
            }
        }
    };
    match &line {
        l if l.contains("state zwirn") => n_zwirn += 1,
        l if l.contains("state riss") => n_riss += 1,
        _ => n_absent += 1,
    }
    println!("{line}");

    n_pairs += 1;
    let dia = arg_value(&args, "--dia");
    let (ra, dec) = (arg_f64(&args, "--ra"), arg_f64(&args, "--dec"));
    let subject = match &dia {
        Some(id) => id.clone(),
        None => match (ra, dec) {
            (Some(r), Some(d)) => format!("ra-{r:.4}-dec-{d:.4}"),
            _ => "unregistered".to_string(),
        },
    };
    let simbad = arg_value(&args, "--simbad");
    let wise = match arg_value(&args, "--wise") {
        Some(w) if w == "agn" => Some(WiseRead::Agn),
        Some(w) if w == "field" => Some(WiseRead::FieldSource),
        Some(w) if w == "nosource" => Some(WiseRead::NoSource),
        _ => None,
    };
    let broker_class = arg_value(&args, "--broker-class").and_then(|w| w.parse::<i64>().ok());
    let bline = borrowed_ledger(&subject, simbad.as_deref(), wise, broker_class);
    match &bline {
        l if l.contains("state zwirn") => n_zwirn += 1,
        l if l.contains("state riss") => n_riss += 1,
        _ => n_absent += 1,
    }
    println!("{bline}");
    println!(
        "riss-knoten tally: {n_pairs} pair(s) — zwirn {n_zwirn} | riss {n_riss} | absent {n_absent} — every refusal stays visible, nothing smoothed"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ledger_line_names_pair_state_lines_and_provenance() {
        let line = ledger_line(
            "body-ceres",
            ("spk-ephemeris", "spk-granules"),
            ("dastcom-keplerian", "dastcom-elements"),
            LedgerState::Zwirn,
            "sep 2.700e4 m tol 1.000e6 m jd 2461544.50000",
        );
        assert_eq!(
            line,
            "riss knot body-ceres state zwirn lines spk-ephemeris(spk-granules) + dastcom-keplerian(dastcom-elements) values sep 2.700e4 m tol 1.000e6 m jd 2461544.50000"
        );
    }

    #[test]
    fn the_three_body_states_render_from_measured_parts() {
        let zwirn = body_ledger(
            "ceres",
            LedgerState::Zwirn,
            "sep 2.700e4 m tol 1.000e6 m jd 2461544.50000",
        );
        assert!(zwirn.contains("body-ceres state zwirn"));
        assert!(zwirn.contains("spk-ephemeris(spk-granules) + dastcom-keplerian(dastcom-elements)"));
        let riss = body_ledger("apophis", LedgerState::Riss, "sep 2.300e9 m");
        assert!(riss.contains("state riss"));
        assert!(riss.contains("sep 2.300e9 m"));
        let absent = body_ledger(
            "vesta",
            LedgerState::Absent,
            "missing dastcom-keplerian (record 4 not in the bin)",
        );
        assert!(absent.contains("state absent"));
        assert!(absent.contains("missing dastcom-keplerian"));
    }

    #[test]
    fn the_body_fold_zwirns_risses_and_stays_absent() {
        let sep = separation_m([0.0, 0.0, 0.0], add_sun([3.0, 4.0, 0.0], [-3.0, -4.0, 0.0]));
        assert!(sep < 1.0e-9);
        assert!(matches!(
            classify(sep, WEBERIN_TOL_M),
            Agreement::Placed { .. }
        ));
        assert!(matches!(
            classify(5.0e6, WEBERIN_TOL_M),
            Agreement::Riss { .. }
        ));
    }

    #[test]
    fn vector_rows_give_ground_magnitudes() {
        let text = r#"{"data":[["2026-09-05T12:00Z",[11127.7,2136.4,52490.6],53699.2],["2026-09-05T12:01Z",[11127.4,2136.9,52490.5],53699.1]]}"#;
        let rows = parse_vector_rows(text);
        assert_eq!(rows.len(), 2);
        let want = (11127.7f64.powi(2) + 2136.4f64.powi(2) + 52490.6f64.powi(2)).sqrt();
        assert!((rows[0].f - want).abs() < 1e-3);
    }

    #[test]
    fn scalar_rows_carry_geometry_when_present() {
        let text = r#"{"data":[["2026-09-05T12:00:00.000Z",9.344,-137.772,26639.44],["2026-09-05T12:00:01.000Z",9.409,-137.773,26653.7]]}"#;
        let rows = parse_scalar_rows(text);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].lat, Some(9.344));
        assert_eq!(rows[0].f, 26639.44);
    }

    #[test]
    fn fill_values_refuse_a_vector_row() {
        let text = r#"{"data":[["2026-09-05T12:00Z",[99999.0,2136.4,52490.6],53699.2]]}"#;
        assert!(parse_vector_rows(text).is_empty());
    }

    #[test]
    fn iso_round_trip() {
        let u = 1.76616e9;
        let s = iso_utc(u);
        assert_eq!(iso_to_unix(&s), Some(u));
    }

    #[test]
    fn two_present_station_lines_that_agree_are_zwirn() {
        assert_eq!(station_state(true, 1.4, Some(4.8)), LedgerState::Zwirn);
        assert_eq!(station_state(true, 4.8, Some(4.8)), LedgerState::Zwirn);
    }

    #[test]
    fn two_present_station_lines_that_refuse_are_riss() {
        assert_eq!(station_state(true, 9189.2, Some(4.8)), LedgerState::Riss);
    }

    #[test]
    fn a_missing_station_line_is_absent() {
        assert_eq!(station_state(false, 0.0, None), LedgerState::Absent);
    }

    #[test]
    fn the_station_ledger_names_both_lines_and_the_overflight() {
        let enc = Encounter {
            epoch: 1.76616e9,
            distance_deg: 0.02,
            ground_f: 53699.2,
            swarm_f: 53700.6,
            ground_sigma: 0.4,
            swarm_sigma: 0.6,
        };
        let line = station_ledger_for_match("ABK", &enc, 1.4, Some(4.8));
        assert!(line.starts_with("riss knot station-ABK state zwirn lines intermagnet-ground(intermagnet-xyzf-best-avail) + swarm-overflight(swarm-maga_lr-1b-scalar-f)"));
        assert!(line.contains("ground 53699.2 nT swarm 53700.6 nT deviation 1.4 nT"));
        assert!(line.contains("overflight 0.02 deg at"));
        assert!(line.ends_with("tolerance 4.8 nT"));
    }

    #[test]
    fn the_station_absent_line_names_the_missing_line() {
        let line = station_ledger("ABK", LedgerState::Absent, "ground line unread");
        assert!(line.contains("state absent"));
        assert!(line.contains("intermagnet-ground(intermagnet-xyzf-best-avail)"));
    }

    #[test]
    fn borrowed_sense_zwirn_names_broker_and_simbad_window() {
        let line = borrowed_ledger("ZTF26abcdefg", Some("SN"), None, Some(11));
        assert!(line.starts_with("riss knot borrowed-ZTF26abcdefg state zwirn"));
        assert!(line.contains("broker-classifier(fink-lsst-main_label_classifier) + simbad-window(simbad-otype-crossmatch)"));
        assert!(line.contains("broker class 11; simbad otype SN"));
        assert!(line.contains("agree (zwirn)"));
    }

    #[test]
    fn borrowed_sense_riss_names_the_allwise_field_window() {
        let line = borrowed_ledger("ZTF26abcdefg", None, Some(WiseRead::FieldSource), Some(11));
        assert!(line.starts_with("riss knot borrowed-ZTF26abcdefg state riss"));
        assert!(line.contains("allwise-window(allwise-w1-w2-wedge)"));
        assert!(line.contains("allwise w1-w2 < 0.8 (field source)"));
        assert!(line.contains("riss stays visible (never smoothed)"));
    }

    #[test]
    fn borrowed_sense_with_a_missing_broker_is_absent() {
        let line = borrowed_ledger("unregistered", None, None, None);
        assert!(line.contains("state absent"));
        assert!(line.contains("broker class absent; independent window absent"));
    }

    #[test]
    fn borrowed_sense_silent_broker_is_absent_not_a_contradiction() {
        let line = borrowed_ledger(
            "ZTF26abcdefg",
            None,
            Some(WiseRead::FieldSource),
            Some(FINK_LSST_CLASS_ABSENT),
        );
        assert!(line.contains("state absent"));
        assert!(line.contains("broker class absent"));
    }
}
