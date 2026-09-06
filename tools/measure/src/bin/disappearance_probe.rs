use omegaflow::archivar::{embedded_lsk, LeapSeconds};
use omegaflow::json::{parse_json, JsonVal};
use omegaflow::jwst::mjd_to_unix;
use omegaflow_measure::nadel_gate::sep_arcsec;
use std::collections::HashMap;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const UA: &str = "omegaflow-disappearance-probe/1.0";
const FINK_CONE: &str = "https://api.lsst.fink-portal.org/api/v1/conesearch";
const FINK_FP: &str = "https://api.lsst.fink-portal.org/api/v1/fp";
const DEFAULT_REGISTER: &str = "phi/reports/scan_coverage.φ";
const FP_BAND: &str = "r:band";
const FP_MJD: &str = "r:midpointMjdTai";
const FP_FLUX: &str = "r:scienceFlux";
const FP_RA: &str = "r:ra";
const FP_DEC: &str = "r:dec";

const VANSIG: f64 = 3.0;
const MAD_SIGMA_K: f64 = 1.4826;
const N_MIN: usize = 6;
const SPLIT_DEFAULT: f64 = 0.5;
const RADIUS_DEFAULT_ARCSEC: f64 = 10.0;
const HTTP_RETRY: usize = 3;
const RATE_BACKOFF_MS: u64 = 3000;

fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Word {
    Vanishing,
    Stable,
    Absent,
}

impl Word {
    fn label(self) -> &'static str {
        match self {
            Word::Vanishing => "vanishing",
            Word::Stable => "stable",
            Word::Absent => "absent",
        }
    }
}

struct Window {
    start_tdb: f64,
    end_tdb: f64,
    n: usize,
    median: f64,
    mad: f64,
    sigma: f64,
    threshold_low: f64,
}

struct Finding {
    word: Word,
    n_total: usize,
    n_ref: usize,
    n_test: usize,
    note: Option<String>,
    ref_win: Option<Window>,
    test_win: Option<Window>,
    drop_z: Option<f64>,
}

fn median_of_sorted(v: &[f64]) -> Option<f64> {
    let n = v.len();
    if n == 0 {
        return None;
    }
    if n % 2 == 1 {
        Some(v[n / 2])
    } else {
        Some(0.5 * (v[n / 2 - 1] + v[n / 2]))
    }
}

fn robust_stats(v: &[f64]) -> Option<(f64, f64, f64)> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    let median = median_of_sorted(&s)?;
    let mut dev: Vec<f64> = s.iter().map(|x| (x - median).abs()).collect();
    dev.sort_by(f64::total_cmp);
    let mad = median_of_sorted(&dev)?;
    let sigma = MAD_SIGMA_K * mad;
    if !median.is_finite() || !mad.is_finite() || !sigma.is_finite() {
        return None;
    }
    Some((median, mad, sigma))
}

fn window_of(rows: &[(f64, f64)], median: f64, mad: f64, sigma: f64) -> Option<Window> {
    let first = rows.first()?;
    let last = rows.last()?;
    Some(Window {
        start_tdb: first.0,
        end_tdb: last.0,
        n: rows.len(),
        median,
        mad,
        sigma,
        threshold_low: median - VANSIG * sigma,
    })
}

fn classify(samples: &[(f64, f64)], split: f64) -> Finding {
    let mut s = samples.to_vec();
    s.sort_by(|a, b| a.0.total_cmp(&b.0));
    let n = s.len();
    if n < 2 {
        return Finding {
            word: Word::Absent,
            n_total: n,
            n_ref: 0,
            n_test: 0,
            note: Some(format!(
                "{n} detection row(s) — fewer than the two disjoint windows the baseline needs"
            )),
            ref_win: None,
            test_win: None,
            drop_z: None,
        };
    }
    let frac = if split.is_finite() && split > 0.0 && split < 1.0 {
        split
    } else {
        SPLIT_DEFAULT
    };
    let mut n_ref = ((n as f64) * frac).floor() as usize;
    if n_ref < 1 {
        n_ref = 1;
    }
    if n_ref >= n {
        n_ref = n - 1;
    }
    let n_test = n - n_ref;
    if n_ref < N_MIN || n_test < N_MIN {
        return Finding {
            word: Word::Absent,
            n_total: n,
            n_ref,
            n_test,
            note: Some(format!(
                "{n} detection row(s) split {n_ref} + {n_test} — a disjoint window below the N_MIN {N_MIN} row floor cannot read a baseline"
            )),
            ref_win: None,
            test_win: None,
            drop_z: None,
        };
    }
    let ref_rows = &s[..n_ref];
    let test_rows = &s[n_ref..];
    let ref_flux: Vec<f64> = ref_rows.iter().map(|&(_, f)| f).collect();
    let test_flux: Vec<f64> = test_rows.iter().map(|&(_, f)| f).collect();
    let Some((median, mad, sigma)) = robust_stats(&ref_flux) else {
        return Finding {
            word: Word::Absent,
            n_total: n,
            n_ref,
            n_test,
            note: Some("the reference window carries no finite flux".to_string()),
            ref_win: None,
            test_win: None,
            drop_z: None,
        };
    };
    let Some(ref_win) = window_of(ref_rows, median, mad, sigma) else {
        return Finding {
            word: Word::Absent,
            n_total: n,
            n_ref,
            n_test,
            note: Some("the reference window carries no time span".to_string()),
            ref_win: None,
            test_win: None,
            drop_z: None,
        };
    };
    let mut test_sorted = test_flux.clone();
    test_sorted.sort_by(f64::total_cmp);
    let Some(test_median) = median_of_sorted(&test_sorted) else {
        return Finding {
            word: Word::Absent,
            n_total: n,
            n_ref,
            n_test,
            note: Some("the test window carries no finite flux".to_string()),
            ref_win: Some(ref_win),
            test_win: None,
            drop_z: None,
        };
    };
    let Some(test_win) = window_of(test_rows, test_median, f64::NAN, f64::NAN) else {
        return Finding {
            word: Word::Absent,
            n_total: n,
            n_ref,
            n_test,
            note: Some("the test window carries no time span".to_string()),
            ref_win: Some(ref_win),
            test_win: None,
            drop_z: None,
        };
    };
    let drop_z = if sigma > 0.0 {
        Some((median - test_median) / sigma)
    } else {
        None
    };
    let word = if test_median < ref_win.threshold_low {
        Word::Vanishing
    } else {
        Word::Stable
    };
    Finding {
        word,
        n_total: n,
        n_ref,
        n_test,
        note: None,
        ref_win: Some(ref_win),
        test_win: Some(test_win),
        drop_z,
    }
}

fn obj_str<'a>(m: &'a HashMap<String, JsonVal>, key: &str) -> Option<&'a str> {
    match m.get(key) {
        Some(JsonVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn obj_f64(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match m.get(key) {
        Some(JsonVal::Num(n)) if n.is_finite() => Some(*n),
        _ => None,
    }
}

struct FpCensus {
    fp_rows: usize,
    det_rows: usize,
    per_band: Vec<(String, usize, usize)>,
}

struct FpSet {
    rows: Vec<(String, f64, f64)>,
    coord: Option<(f64, f64)>,
    census: FpCensus,
}

fn parse_fp(body: &[u8]) -> Option<FpSet> {
    let text = std::str::from_utf8(body).ok()?;
    let rows = match parse_json(text) {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let mut det: Vec<(String, f64, f64)> = Vec::new();
    let mut coord: Option<(f64, f64)> = None;
    let mut fp_per: HashMap<String, usize> = HashMap::new();
    let mut det_per: HashMap<String, usize> = HashMap::new();
    let mut fp_rows = 0usize;
    for r in &rows {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        let (Some(band), Some(mjd)) = (obj_str(m, FP_BAND), obj_f64(m, FP_MJD)) else {
            continue;
        };
        if coord.is_none() {
            if let (Some(ra), Some(dec)) = (obj_f64(m, FP_RA), obj_f64(m, FP_DEC)) {
                coord = Some((ra, dec));
            }
        }
        fp_rows += 1;
        *fp_per.entry(band.to_string()).or_insert(0) += 1;
        match obj_f64(m, FP_FLUX) {
            Some(f) if f > 0.0 => {
                det.push((band.to_string(), mjd, f));
                *det_per.entry(band.to_string()).or_insert(0) += 1;
            }
            _ => {}
        }
    }
    let mut per_band: Vec<(String, usize, usize)> = fp_per
        .into_iter()
        .map(|(b, nf)| {
            let nd = match det_per.get(&b).copied() {
                Some(n) => n,
                None => 0,
            };
            (b, nf, nd)
        })
        .collect();
    per_band.sort();
    let det_rows = det.len();
    Some(FpSet {
        rows: det,
        coord,
        census: FpCensus {
            fp_rows,
            det_rows,
            per_band,
        },
    })
}

fn fink_mjd_tai_to_tdb(mjd_tai: f64, lsk: &LeapSeconds) -> Option<f64> {
    let as_utc_unix = mjd_to_unix(mjd_tai);
    let tai_minus_utc = lsk.leap_at(as_utc_unix)?;
    lsk.unix_to_tdb(as_utc_unix - tai_minus_utc)
}

fn curl_post_json(url: &str, body: &str) -> Option<(String, Vec<u8>)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(body)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}");
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn curl_post_rate_aware(url: &str, body: &str, who: &str) -> Option<(String, Vec<u8>)> {
    for attempt in 0..HTTP_RETRY {
        let Some(resp) = curl_post_json(url, body) else {
            return None;
        };
        if resp.0 != "429" {
            return Some(resp);
        }
        let backoff = RATE_BACKOFF_MS * (attempt as u64 + 1);
        println!(
            "{who}: HTTP 429 — the endpoint asks for a slower pace; {backoff} ms before the next try (try {})",
            attempt + 1
        );
        sleep_ms(backoff);
    }
    println!(
        "{who}: HTTP 429 held across {HTTP_RETRY} backed-off tries — the rate limit stands, the query stays pending"
    );
    None
}

fn fp_fetch(id: &str) -> Option<(String, Vec<u8>)> {
    let payload = format!("{{\"diaObjectId\": \"{id}\"}}");
    curl_post_rate_aware(FINK_FP, &payload, &format!("Fink/LSST FP {id}"))
}

struct ConeHit {
    id: String,
    ra: f64,
    dec: f64,
    sep_as: f64,
}

fn extract_object_id(body: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(body).ok()?;
    let needle = "\"r:diaObjectId\":";
    let pos = text.find(needle)?;
    let mut digits = String::new();
    for c in text[pos + needle.len()..].chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            break;
        }
    }
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

fn parse_cone_hits(body: &[u8]) -> Option<Vec<ConeHit>> {
    let text = std::str::from_utf8(body).ok()?;
    let rows = match parse_json(text) {
        Some(JsonVal::Arr(a)) => a,
        _ => return None,
    };
    let mut ids: Vec<String> = Vec::new();
    let mut probe = text;
    let needle = "\"r:diaObjectId\":";
    while let Some(rel) = probe.find(needle) {
        let mut digits = String::new();
        for c in probe[rel + needle.len()..].chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else {
                break;
            }
        }
        if digits.is_empty() {
            break;
        }
        let consumed = digits.len();
        ids.push(digits);
        probe = &probe[rel + needle.len() + consumed..];
    }
    if ids.len() != rows.len() {
        return None;
    }
    let mut out: Vec<ConeHit> = Vec::new();
    for (idx, r) in rows.iter().enumerate() {
        let JsonVal::Obj(m) = r else {
            continue;
        };
        let (Some(ra), Some(dec)) = (obj_f64(m, FP_RA), obj_f64(m, FP_DEC)) else {
            continue;
        };
        out.push(ConeHit {
            id: ids[idx].clone(),
            ra,
            dec,
            sep_as: 0.0,
        });
    }
    Some(out)
}

fn cone_nearest(ra: f64, dec: f64, radius_as: f64) -> (Option<ConeHit>, usize, Option<String>) {
    let payload = format!(
        "{{\"ra\": {ra}, \"dec\": {dec}, \"radius\": {radius_as}, \"columns\": \"r:diaObjectId,r:ra,r:dec\"}}"
    );
    let who = format!("Fink/LSST cone ({ra}, {dec}, {radius_as} arcsec)");
    let Some((code, body)) = curl_post_rate_aware(FINK_CONE, &payload, &who) else {
        return (
            None,
            0,
            Some(format!(
                "{who}: the cone query did not answer (measured stall)"
            )),
        );
    };
    if code != "200" {
        return (
            None,
            0,
            Some(format!("{who}: the cone answered HTTP {code}")),
        );
    }
    let Some(mut hits) = parse_cone_hits(&body) else {
        return (
            None,
            0,
            Some(format!(
                "{who}: the cone body is not a row array — the parser stays pending"
            )),
        );
    };
    for h in &mut hits {
        h.sep_as = sep_arcsec(ra, dec, h.ra, h.dec);
    }
    hits.sort_by(|a, b| a.sep_as.total_cmp(&b.sep_as));
    let total = hits.len();
    let nearest = hits.into_iter().next();
    (nearest, total, None)
}

struct RegisterCone {
    ra: f64,
    dec: f64,
    radius_as: f64,
    instrument: String,
    date: String,
    outcome: String,
}

fn parse_register_cone(line: &str) -> Option<RegisterCone> {
    let mut t = line.split_whitespace();
    if t.next()? != "cone" {
        return None;
    }
    let mut ra = None;
    let mut dec = None;
    let mut radius_as = None;
    let mut instrument = None;
    let mut date = None;
    loop {
        let key = t.next()?;
        match key {
            "ra" => ra = Some(t.next()?.parse().ok()?),
            "dec" => dec = Some(t.next()?.parse().ok()?),
            "radius" => radius_as = Some(t.next()?.parse().ok()?),
            "instrument" => instrument = Some(t.next()?.to_string()),
            "date" => date = Some(t.next()?.to_string()),
            "outcome" => break,
            _ => return None,
        }
    }
    let outcome: Vec<&str> = t.collect();
    Some(RegisterCone {
        ra: ra?,
        dec: dec?,
        radius_as: radius_as?,
        instrument: instrument?,
        date: date?,
        outcome: outcome.join(" "),
    })
}

fn read_register(path: &str) -> Vec<RegisterCone> {
    let Ok(body) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    body.lines().filter_map(parse_register_cone).collect()
}

fn coverage_at(register_path: &str, ra: f64, dec: f64) {
    let cones = read_register(register_path);
    if cones.is_empty() {
        println!(
            "coverage register {register_path}: no structured cone line read — the footprint consultation stays pending"
        );
        return;
    }
    let mut nearest: Option<(f64, &RegisterCone)> = None;
    for c in &cones {
        let sep = sep_arcsec(ra, dec, c.ra, c.dec);
        if nearest.map(|(d, _)| sep < d).unwrap_or(true) {
            nearest = Some((sep, c));
        }
    }
    let Some((sep, c)) = nearest else {
        return;
    };
    let inside = sep <= c.radius_as;
    println!(
        "coverage register {register_path}: {} cone line(s); ra {ra:.4} dec {dec:.4} {} the nearest measured footprint ra {:.4} dec {:.4} radius {:.0} arcsec instrument {} date {} outcome {} — separation {sep:.1} arcsec",
        cones.len(),
        if inside {
            "sits inside"
        } else {
            "sits outside"
        },
        c.ra,
        c.dec,
        c.radius_as,
        c.instrument,
        c.date,
        c.outcome
    );
}

struct ObjectOutcome {
    vanishing: usize,
    stable: usize,
    absent: usize,
    pending: bool,
}

fn fmt_tdb(tdb: f64) -> String {
    format!("{tdb:.1} s since J2000")
}

fn report_finding(band: &str, f: &Finding) {
    match f.word {
        Word::Absent => {
            let note = f.note.as_deref().unwrap_or("no window read");
            println!(
                "  band {band}: verdict absent — {note} ({n_total} detection row(s); windows {n_ref} + {n_test})",
                n_total = f.n_total,
                n_ref = f.n_ref,
                n_test = f.n_test
            );
        }
        Word::Vanishing | Word::Stable => {
            let rw = f
                .ref_win
                .as_ref()
                .expect("a classified band carries its reference window");
            let tw = f
                .test_win
                .as_ref()
                .expect("a classified band carries its test window");
            println!(
                "  band {band}: verdict {} — the late half {}",
                f.word.label(),
                match f.word {
                    Word::Vanishing => format!(
                        "lies {VANSIG} robust sigma or more below the reference median (a darkening event)"
                    ),
                    _ => "does not cross the lower threshold — steady over the disjoint windows"
                        .to_string(),
                }
            );
            println!(
                "    reference (upstream) window TDB {} .. {} (n {}, {:.1} d span): median {:.3}, MAD {:.3}, robust sigma {MAD_SIGMA_K} x MAD = {:.3}",
                fmt_tdb(rw.start_tdb),
                fmt_tdb(rw.end_tdb),
                rw.n,
                (rw.end_tdb - rw.start_tdb) / 86400.0,
                rw.median,
                rw.mad,
                rw.sigma
            );
            println!(
                "    lower threshold median - VANSIG {VANSIG} x sigma = {:.3} (mirrors DIP_SIG 3.0)",
                rw.threshold_low
            );
            println!(
                "    test (late) window TDB {} .. {} (n {}, {:.1} d span): median {:.3}",
                fmt_tdb(tw.start_tdb),
                fmt_tdb(tw.end_tdb),
                tw.n,
                (tw.end_tdb - tw.start_tdb) / 86400.0,
                tw.median
            );
            match f.drop_z {
                Some(z) => println!("    drop z = (median_ref - median_test) / sigma = {z:.2}"),
                None => println!(
                    "    drop z: the reference window carries zero scatter (MAD 0) — a lower late median is not attributable to noise; the drop is read on the median line"
                ),
            }
        }
    }
}

fn run_fp_body(
    label: &str,
    hint: Option<(f64, f64)>,
    body: &[u8],
    source: &str,
    register_path: &str,
    split: f64,
) -> ObjectOutcome {
    let Some(set) = parse_fp(body) else {
        println!(
            "Fink/LSST FP {label}: the forced-photometry sample is not a JSON array — parser pending on the real schema ({source})"
        );
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: true,
        };
    };
    let coord = hint.or(set.coord);
    let (ra, dec) = match coord {
        Some((r, d)) => (r, d),
        None => {
            println!(
                "Fink/LSST FP {label}: no FP row carries ra/dec — the object stays unplaced (pending)"
            );
            return ObjectOutcome {
                vanishing: 0,
                stable: 0,
                absent: 0,
                pending: true,
            };
        }
    };
    println!(
        "Fink/LSST FP {label} ({source}): {fp} forced-photometry row(s) at ra {ra:.4} dec {dec:.4}; {det} row(s) with scienceFlux > 0 become measurement rows; the rest stay absent (0 honored, never a fabricated 0.0)",
        fp = set.census.fp_rows,
        det = set.census.det_rows
    );
    if !set.census.per_band.is_empty() {
        let parts: Vec<String> = set
            .census
            .per_band
            .iter()
            .map(|(b, nf, nd)| format!("{b}: {nd} detection(s) of {nf} FP row(s)"))
            .collect();
        println!("  per-band census — {}", parts.join(" | "));
    }
    let Some(lsk) = embedded_lsk() else {
        println!(
            "Fink/LSST FP {label}: the embedded naif0012.tls leap table is absent — the time layer stays pending"
        );
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: true,
        };
    };
    println!(
        "Fink/LSST FP {label}: time layer — r:midpointMjdTai (MJD/TAI) mapped mjd -> unix -> TDB (naif0012.tls) before the disjoint windows are cut; the Rømer light-travel term (seasonal +-8 min) shifts the arrival axis without re-ordering a brightness series and stays out of this flux-residual cut"
    );
    let mut per_band: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    let mut tdb_skipped = 0usize;
    for (band, mjd, flux) in &set.rows {
        match fink_mjd_tai_to_tdb(*mjd, &lsk) {
            Some(tdb) => {
                per_band
                    .entry(band.clone())
                    .or_insert_with(Vec::new)
                    .push((tdb, *flux));
            }
            None => tdb_skipped += 1,
        }
    }
    if tdb_skipped > 0 {
        println!(
            "Fink/LSST FP {label}: {tdb_skipped} measurement row(s) carry no TDB mapping (the leap table is void at their epoch — absent)"
        );
    }
    let mut bands: Vec<String> = per_band.keys().cloned().collect();
    bands.sort();
    if bands.is_empty() {
        println!("Fink/LSST FP {label}: no measurement row maps onto the TDB time layer (absent)");
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: false,
        };
    }
    coverage_at(register_path, ra, dec);
    println!(
        "\nFink/LSST FP {label}: disappearance verdict per band (disjoint windows, reference = upstream half, split {split}, N_MIN {N_MIN} per window, VANSIG {VANSIG} robust sigma)"
    );
    let mut vanishing = 0usize;
    let mut stable = 0usize;
    let mut absent = 0usize;
    for band in &bands {
        let samples = per_band.get(band).expect("band key from the map");
        let f = classify(samples, split);
        match f.word {
            Word::Vanishing => vanishing += 1,
            Word::Stable => stable += 1,
            Word::Absent => absent += 1,
        }
        report_finding(band, &f);
    }
    ObjectOutcome {
        vanishing,
        stable,
        absent,
        pending: false,
    }
}

fn run_fp_target(
    id: &str,
    hint: Option<(f64, f64)>,
    save: Option<&str>,
    register_path: &str,
    split: f64,
) -> ObjectOutcome {
    let Some((code, body)) = fp_fetch(id) else {
        println!(
            "Fink/LSST FP {id}: the forced-photometry query did not answer (measured stall) — pending"
        );
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: true,
        };
    };
    if code != "200" {
        println!("Fink/LSST FP {id}: the forced photometry answered HTTP {code} — pending");
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: true,
        };
    }
    let raw_path = match save {
        Some(s) => s.to_string(),
        None => format!("tmp/fink_fp_{id}.json"),
    };
    if std::fs::write(&raw_path, &body).is_err() {
        println!("Fink/LSST FP {id}: the real sample was not saved ({raw_path})");
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: true,
        };
    }
    println!(
        "Fink/LSST /api/v1/fp {id}: HTTP {code}, {} bytes, real sample saved {raw_path}",
        body.len()
    );
    run_fp_body(id, hint, &body, &raw_path, register_path, split)
}

fn run_fp_position(
    ra: f64,
    dec: f64,
    radius_as: f64,
    save: Option<&str>,
    register_path: &str,
    split: f64,
) -> ObjectOutcome {
    println!(
        "\n=== Disappearance probe (Funke 1 — vanishing instead of appearing) at ra {ra:.4} dec {dec:.4}, search radius {radius_as} arcsec ==="
    );
    coverage_at(register_path, ra, dec);
    let (nearest, total, msg) = cone_nearest(ra, dec, radius_as);
    if let Some(m) = msg {
        println!("{m}");
    }
    let Some(hit) = nearest else {
        println!(
            "Fink/LSST: {total} object row(s) within {radius_as} arcsec of ra {ra:.4} dec {dec:.4} — the empty road has no source to fade; the fixed-position hunt stays void (0 honored)"
        );
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: false,
        };
    };
    println!(
        "Fink/LSST: {total} object row(s) within {radius_as} arcsec; the nearest diaObject {} sits at ra {:.4} dec {:.4}, separation {:.2} arcsec — its forced photometry is the measurement at the fixed position",
        hit.id, hit.ra, hit.dec, hit.sep_as
    );
    run_fp_target(&hit.id, Some((hit.ra, hit.dec)), save, register_path, split)
}

fn run_fp_scan(path: &str, register_path: &str, split: f64) -> ObjectOutcome {
    let body = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            println!(
                "Fink/LSST FP: no forced-photometry sample at {path} — the re-scan stays void (0 honored, pending)"
            );
            return ObjectOutcome {
                vanishing: 0,
                stable: 0,
                absent: 0,
                pending: true,
            };
        }
    };
    let Some(id) = extract_object_id(&body) else {
        println!(
            "Fink/LSST FP: the sample at {path} carries no r:diaObjectId — the re-scan stays void (absent)"
        );
        return ObjectOutcome {
            vanishing: 0,
            stable: 0,
            absent: 0,
            pending: false,
        };
    };
    println!(
        "\n=== Disappearance probe (Funke 1 — vanishing instead of appearing), offline re-scan of {path} (diaObject {id}) ==="
    );
    run_fp_body(&id, None, &body, path, register_path, split)
}

fn usage() {
    eprintln!(
        "disappearance_probe — Funke 1 (vanishing instead of appearing): forced-photometry darkening hunt\n\
         the fixed-position light curve of an object is measured without an alert; the late half is judged\n\
         against the disjoint upstream reference half (median baseline, MAD scale, lower threshold\n\
         median - VANSIG 3.0 * 1.4826*MAD); a late half that lies below the threshold is a vanishing event\n\
         live forced photometry by Fink diaObjectId (anonymous /api/v1/fp), no token:\n\
         \x20 disappearance_probe --fink-fp <diaObjectId> [--save <raw.json>]\n\
         live forced photometry by position (Fink cone to the nearest diaObject, then /api/v1/fp):\n\
         \x20 disappearance_probe --pos ra,dec[,radius_arcsec] [--save <raw.json>]  (repeatable)\n\
         offline re-scan of a saved /api/v1/fp forced-photometry sample (the window rides with the verdict):\n\
         \x20 disappearance_probe --scan <fink_fp_<diaObjectId>.json>\n\
         options:\n\
         \x20 --split <fraction=0.5> — the count split between the upstream reference and the late test half\n\
         \x20 --register <path=phi/reports/scan_coverage.phi> — the coverage register, consulted as the honest \"where we have looked\""
    );
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    let mut i = 0;
    while i + 1 < args.len() {
        if args[i] == name {
            return Some(args[i + 1].clone());
        }
        i += 1;
    }
    None
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name)?.parse().ok()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 2 {
        usage();
        return;
    }
    let register_path = match arg_value(&args, "--register") {
        Some(v) => v,
        None => DEFAULT_REGISTER.to_string(),
    };
    let split = arg_f64(&args, "--split").unwrap_or(SPLIT_DEFAULT);
    let save = arg_value(&args, "--save");
    let mut targets: Vec<String> = Vec::new();
    let mut positions: Vec<(f64, f64, f64)> = Vec::new();
    let mut scans: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--fink-fp" => {
                if let Some(v) = args.get(i + 1) {
                    targets.push(v.clone());
                    i += 1;
                }
            }
            "--pos" => {
                if let Some(spec) = args.get(i + 1) {
                    let p: Vec<&str> = spec.split(',').collect();
                    if p.len() >= 2 {
                        if let (Some(ra), Some(dec)) = (
                            p[0].trim().parse::<f64>().ok(),
                            p[1].trim().parse::<f64>().ok(),
                        ) {
                            let radius = p
                                .get(2)
                                .and_then(|r| r.trim().parse::<f64>().ok())
                                .filter(|r| r.is_finite() && *r > 0.0)
                                .unwrap_or(RADIUS_DEFAULT_ARCSEC);
                            if ra.is_finite() && dec.is_finite() {
                                positions.push((ra, dec, radius));
                                i += 1;
                            }
                        }
                    }
                }
            }
            "--scan" => {
                if let Some(v) = args.get(i + 1) {
                    scans.push(v.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    if targets.is_empty() && positions.is_empty() && scans.is_empty() {
        usage();
        return;
    }
    let mut v = 0usize;
    let mut s = 0usize;
    let mut a = 0usize;
    let mut pend = 0usize;
    let mut n_obj = 0usize;
    for id in &targets {
        n_obj += 1;
        let o = run_fp_target(id, None, save.as_deref(), &register_path, split);
        v += o.vanishing;
        s += o.stable;
        a += o.absent;
        pend += o.pending as usize;
    }
    for (ra, dec, radius) in &positions {
        n_obj += 1;
        let o = run_fp_position(*ra, *dec, *radius, save.as_deref(), &register_path, split);
        v += o.vanishing;
        s += o.stable;
        a += o.absent;
        pend += o.pending as usize;
    }
    for path in &scans {
        n_obj += 1;
        let o = run_fp_scan(path, &register_path, split);
        v += o.vanishing;
        s += o.stable;
        a += o.absent;
        pend += o.pending as usize;
    }
    println!(
        "\nCampaign verdict: {n_obj} object(s) probed — {v} band(s) vanishing, {s} stable, {a} absent, {pend} band-layer(s) pending"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lsk() -> LeapSeconds {
        embedded_lsk().expect("the embedded naif0012.tls table parses in the test build")
    }

    fn offsets(n: usize, base: f64, amplitude: f64, t0: f64) -> Vec<(f64, f64)> {
        let mut out = Vec::with_capacity(n);
        for k in 0..n {
            let o = ((k % 5) as f64 - 2.0) * amplitude;
            out.push((t0 + k as f64, base + o));
        }
        out
    }

    #[test]
    fn known_baseline_with_a_late_decline_reads_vanishing() {
        let mut samples = offsets(24, 1000.0, 3.2, 0.0);
        samples.extend(offsets(24, 180.0, 3.2, 100.0));
        let f = classify(&samples, SPLIT_DEFAULT);
        assert_eq!(f.word, Word::Vanishing);
        assert_eq!(f.n_ref, 24);
        assert_eq!(f.n_test, 24);
        let rw = f.ref_win.as_ref().expect("vanishing carries its windows");
        assert!((rw.median - 1000.0).abs() < 1e-9);
        assert!(rw.sigma > 0.0, "the cycling offsets give a real MAD scale");
        let tw = f.test_win.as_ref().expect("vanishing carries its windows");
        assert!((tw.median - 180.0).abs() < 1e-9);
        let z = f.drop_z.expect("a positive sigma yields a drop z");
        assert!(
            z > VANSIG,
            "the late drop lies {z:.2} robust sigma below the reference median"
        );
        assert!(rw.threshold_low > tw.median);
    }

    #[test]
    fn a_flat_series_stays_stable() {
        let mut samples = offsets(24, 1000.0, 3.2, 0.0);
        samples.extend(offsets(24, 1000.0, 3.2, 100.0));
        let f = classify(&samples, SPLIT_DEFAULT);
        assert_eq!(f.word, Word::Stable);
        let rw = f.ref_win.as_ref().expect("stable carries its windows");
        let tw = f.test_win.as_ref().expect("stable carries its windows");
        assert!((rw.median - tw.median).abs() < 1e-9);
        let z = f.drop_z.expect("a positive sigma yields a drop z");
        assert!(
            z < VANSIG,
            "the flat late half lies {z:.2} sigma below — not enough for vanishing"
        );
    }

    #[test]
    fn a_series_too_short_for_two_disjoint_windows_is_absent() {
        let samples = offsets(8, 1000.0, 3.2, 0.0);
        let f = classify(&samples, SPLIT_DEFAULT);
        assert_eq!(f.word, Word::Absent);
        assert!(f.ref_win.is_none(), "no window rides an absent verdict");
        assert!(f.test_win.is_none(), "no window rides an absent verdict");
        assert!(
            f.note.as_deref().unwrap_or("").contains("N_MIN"),
            "the absent reason names the N_MIN floor, was {:?}",
            f.note
        );
    }

    #[test]
    fn a_handful_of_rows_never_reaches_a_classified_window() {
        let samples = vec![(0.0, 100.0), (1.0, 99.0), (2.0, 101.0)];
        let f = classify(&samples, SPLIT_DEFAULT);
        assert_eq!(f.word, Word::Absent);
        assert_eq!(f.note.is_some(), true);
    }

    #[test]
    fn fp_parser_keeps_positive_flux_rows_and_counts_the_absent() {
        let body = br#"[{"r:band":"g","r:dec":2.5208047696,"r:diaForcedSourceId":1,"r:diaObjectId":313998569858662581,"r:midpointMjdTai":61204.9817515752,"r:ra":148.8745712188,"r:scienceFlux":-320.1,"r:visit":1},{"r:band":"r","r:dec":2.5208047616,"r:diaForcedSourceId":2,"r:diaObjectId":313998569858662581,"r:midpointMjdTai":61205.9837394019,"r:ra":148.8745712297,"r:scienceFlux":0.0,"r:visit":2},{"r:band":"i","r:dec":2.5208,"r:diaForcedSourceId":3,"r:diaObjectId":313998569858662581,"r:midpointMjdTai":61206.0,"r:ra":148.8746,"r:scienceFlux":5100.0,"r:visit":3}]"#;
        let set = parse_fp(body).expect("the FP body parses");
        assert_eq!(set.census.fp_rows, 3);
        assert_eq!(
            set.rows.len(),
            1,
            "only the positive scienceFlux row is a measurement"
        );
        assert_eq!(set.rows[0].0, "i");
        assert!((set.rows[0].2 - 5100.0).abs() < 1e-9);
        let (ra, dec) = set.coord.expect("the rows carry ra/dec");
        assert!((ra - 148.8745712188).abs() < 1e-6);
        assert!((dec - 2.5208047696).abs() < 1e-6);
        assert_eq!(set.census.per_band.len(), 3);
        let i_entry = set
            .census
            .per_band
            .iter()
            .find(|(b, _, _)| b == "i")
            .expect("the i band row is in the census");
        assert_eq!(i_entry.1, 1, "one i FP row");
        assert_eq!(i_entry.2, 1, "its flux is a measurement");
    }

    #[test]
    fn fp_mjd_rows_fold_to_an_absolute_tdb() {
        let body = include_str!("lsst_fp_313998569858662581_6rows.json");
        let set = parse_fp(body.as_bytes()).expect("the real FP sample parses");
        assert!(set.rows.len() >= 6);
        let lsk = lsk();
        for (band, mjd, flux) in &set.rows {
            let tdb = fink_mjd_tai_to_tdb(*mjd, &lsk).expect("a 2026 epoch lies in the leap table");
            assert!(
                tdb.is_finite() && tdb > 0.0,
                "the fold lands on a positive TDB axis"
            );
            assert!(*flux > 0.0);
            assert!(!band.is_empty());
        }
    }

    #[test]
    fn register_cone_line_parses_into_the_footprint_fields() {
        let line = "cone ra 267.4955 dec -29.6079 radius 900 instrument fink-lsst date 2026-09-05 outcome void (Fink returned 0 object rows) raw 0 floor 0 natural 0 agn 0 food 0";
        let c = parse_register_cone(line).expect("the register cone line parses");
        assert!((c.ra - 267.4955).abs() < 1e-9);
        assert!((c.dec + 29.6079).abs() < 1e-9);
        assert_eq!(c.radius_as, 900.0);
        assert_eq!(c.instrument, "fink-lsst");
        assert_eq!(c.date, "2026-09-05");
        assert!(c.outcome.starts_with("void"));
    }
}
