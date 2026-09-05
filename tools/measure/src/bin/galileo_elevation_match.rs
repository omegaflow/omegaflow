use std::collections::{BTreeMap, HashMap};

use omegaflow::archivar::{
    body_barycenter_position, body_fixed_to_icrs, icrs_to_body_surface,
    parse_ephemeris_binary, BodyEphemeris,
};
use omegaflow::atdf::parse_resid_bin;
use omegaflow::odp::{dsn_station, EARTH};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const MIN_CELL: usize = 30;
const DEFAULT_GAP_S: f64 = 600.0;
const SUB_GAP_S: f64 = 120.0;
const CHUNK_N: usize = 60;
const FLOOR_CAP: f64 = -2560.0;
const PLATEAU_MIN: f64 = -1900.0;
const EDGE_S: f64 = 120.0;
const STATIONS: [i64; 3] = [14, 43, 63];

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}


fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
    }
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0}"),
    }
}

fn classify(s: f64) -> u8 {
    if s == 0.0 {
        return 0;
    }
    if s <= FLOOR_CAP {
        return 1;
    }
    if s >= PLATEAU_MIN {
        return 2;
    }
    0
}

#[derive(Clone, Copy)]
struct Sample {
    t: f64,
    resid: f64,
    el: Option<f64>,
}

struct OpenChunk {
    label: u8,
    samples: Vec<Sample>,
}

#[derive(Clone, Copy)]
struct ChunkRec {
    label: u8,
    n: usize,
    dev2: f64,
    el_mean: Option<f64>,
    t0: f64,
    t1: f64,
}

#[derive(Clone, Copy, Default)]
struct Acc {
    n: usize,
    dev2: f64,
}

impl Acc {
    fn rms(&self) -> Option<f64> {
        if self.n == 0 {
            return None;
        }
        Some((self.dev2 / self.n as f64).sqrt())
    }
}

struct BandCells {
    cells: BTreeMap<i64, [Acc; 2]>,
    band_w: f64,
}

impl BandCells {
    fn add(&mut self, label: u8, el: f64, dev2: f64) {
        let b = (el / self.band_w).floor() as i64;
        let cell = self.cells.entry(b).or_insert([Acc::default(), Acc::default()]);
        let acc = &mut cell[(label - 1) as usize];
        acc.n += 1;
        acc.dev2 += dev2;
    }
}

struct PassOpen {
    t0: f64,
    t_last: f64,
    floor_all: usize,
    plateau_all: usize,
    cur: Option<OpenChunk>,
    chunks: Vec<ChunkRec>,
    floor_els: Vec<f64>,
    plateau_els: Vec<f64>,
    cells: BandCells,
}

impl PassOpen {
    fn start(t: f64, band_w: f64) -> PassOpen {
        PassOpen {
            t0: t,
            t_last: t,
            floor_all: 0,
            plateau_all: 0,
            cur: None,
            chunks: Vec::new(),
            floor_els: Vec::new(),
            plateau_els: Vec::new(),
            cells: BandCells { cells: BTreeMap::new(), band_w },
        }
    }
}

struct PassStat {
    mode: i64,
    station: i64,
    t0: f64,
    floor_all: usize,
    plateau_all: usize,
    f: Acc,
    p: Acc,
    fi: Acc,
    pi: Acc,
    chunks: Vec<ChunkRec>,
    floor_els: Vec<f64>,
    plateau_els: Vec<f64>,
    cells: BTreeMap<i64, [Acc; 2]>,
}

fn interior(rec: &ChunkRec, lo: f64, hi: f64) -> bool {
    rec.t0 >= lo && rec.t1 <= hi
}

fn flush_chunk(open: &mut PassOpen, label: u8, samples: &[Sample]) {
    if samples.len() < MIN_CELL {
        return;
    }
    let mut mean = 0.0f64;
    for s in samples {
        mean += s.resid;
    }
    mean /= samples.len() as f64;
    let mut dev2 = 0.0f64;
    let mut el_sum = 0.0f64;
    let mut el_n = 0usize;
    for s in samples {
        let d = s.resid - mean;
        dev2 += d * d;
        if let Some(el) = s.el {
            el_sum += el;
            el_n += 1;
            open.cells.add(label, el, d * d);
            if label == 1 {
                open.floor_els.push(el);
            } else {
                open.plateau_els.push(el);
            }
        }
    }
    let t0 = samples[0].t;
    let t1 = samples[samples.len() - 1].t;
    open.chunks.push(ChunkRec {
        label,
        n: samples.len(),
        dev2,
        el_mean: if el_n > 0 { Some(el_sum / el_n as f64) } else { None },
        t0,
        t1,
    });
}

fn finish_pass(pass: &mut PassOpen) -> PassStat {
    if let Some(cur) = pass.cur.take() {
        flush_chunk(pass, cur.label, &cur.samples);
    }
    let lo = pass.t0 + EDGE_S;
    let hi = pass.t_last - EDGE_S;
    let mut f = Acc::default();
    let mut p = Acc::default();
    let mut fi = Acc::default();
    let mut pi = Acc::default();
    for c in &pass.chunks {
        if c.label == 1 {
            f.n += c.n;
            f.dev2 += c.dev2;
            if interior(c, lo, hi) {
                fi.n += c.n;
                fi.dev2 += c.dev2;
            }
        } else {
            p.n += c.n;
            p.dev2 += c.dev2;
            if interior(c, lo, hi) {
                pi.n += c.n;
                pi.dev2 += c.dev2;
            }
        }
    }
    PassStat {
        mode: 0,
        station: 0,
        t0: pass.t0,
        floor_all: pass.floor_all,
        plateau_all: pass.plateau_all,
        f,
        p,
        fi,
        pi,
        chunks: std::mem::take(&mut pass.chunks),
        floor_els: std::mem::take(&mut pass.floor_els),
        plateau_els: std::mem::take(&mut pass.plateau_els),
        cells: std::mem::take(&mut pass.cells.cells),
    }
}

struct KeyState {
    prev: Option<f64>,
    pass: Option<PassOpen>,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState { prev: None, pass: None }
    }
}

fn elevation_at(t: f64, station: i64, eph: &HashMap<String, BodyEphemeris>) -> Option<f64> {
    let (lat_deg, lon_deg, _alt) = dsn_station(station)?;
    let p = body_barycenter_position("galileo_daily", t, eph)?;
    let e = body_barycenter_position(EARTH, t, eph)?;
    let v = sub(p, e);
    let r = norm(v);
    if r <= 0.0 || !r.is_finite() {
        return None;
    }
    let dec = (v[2] / r).clamp(-1.0, 1.0).asin();
    let ra = v[1].atan2(v[0]);
    let jd = t / 86400.0 + 2451545.0;
    let gmst = (280.46061837 + 360.98564736629 * (jd - 2451545.0)).rem_euclid(360.0);
    let lst = (gmst + lon_deg).rem_euclid(360.0).to_radians();
    let ha = lst - ra;
    let phi = lat_deg.to_radians();
    let sin_el = phi.sin() * dec.sin() + phi.cos() * dec.cos() * ha.cos();
    Some(sin_el.clamp(-1.0, 1.0).asin().to_degrees())
}




fn histogram(recs: &[[f64; 8]], eph: &HashMap<String, BodyEphemeris>, station: i64, t0: f64, t1: f64) {
    let hours = ((t1 - t0) / 3600.0).round() as usize;
    let mut counts = vec![0usize; hours];
    for r in recs {
        if r[3] as i64 != 1 || r[2] as i64 != station {
            continue;
        }
        let t = r[0];
        if t < t0 || t > t1 {
            continue;
        }
        if r[1].abs() > LOCK_HZ {
            continue;
        }
        if classify(r[7]) == 0 {
            continue;
        }
        let h = ((t - t0) / 3600.0).floor() as usize;
        if h < hours {
            counts[h] += 1;
        }
    }
    let el_row: Vec<String> = (0..hours)
        .map(|h| {
            let t = t0 + (h as f64 + 0.5) * 3600.0;
            match elevation_at(t, station, eph) {
                Some(el) => format!("{el:+.0}"),
                None => "na".to_string(),
            }
        })
        .collect();
    let ct_row: Vec<String> = counts.iter().map(|c| format!("{c}")).collect();
    println!(
        "hist st{station} {:.0}..{:.0}: el/h  {}",
        t0 / 86400.0,
        t1 / 86400.0,
        el_row.join(" ")
    );
    println!("                counts/h  {}", ct_row.join(" "));
}

fn passcheck(recs: &[[f64; 8]], eph: &HashMap<String, BodyEphemeris>) {
    histogram(recs, eph, 43, -94478400.0, -94305600.0);
    histogram(recs, eph, 63, -127742400.0, -127656000.0);

    let windows = [
        (43i64, -94478400.0f64, -94305600.0f64, "st43 1997-01-03..05"),
        (63i64, -127742400.0f64, -127656000.0f64, "st63 1995-12-15..16"),
    ];
    for (station, t0, t1, tag) in windows {
        let mut els: Vec<f64> = Vec::new();
        let mut ts: Vec<f64> = Vec::new();
        let mut n = 0usize;
        let mut n_lock = 0usize;
        let mut floor_n = 0usize;
        let mut plat_n = 0usize;
        for r in recs {
            if r[3] as i64 != 1 || r[2] as i64 != station {
                continue;
            }
            let t = r[0];
            if t < t0 || t > t1 {
                continue;
            }
            n += 1;
            if r[1].abs() > LOCK_HZ {
                n_lock += 1;
                continue;
            }
            let lbl = classify(r[7]);
            if lbl == 0 {
                continue;
            }
            if lbl == 1 {
                floor_n += 1;
            } else {
                plat_n += 1;
            }
            if let Some(el) = elevation_at(t, station, eph) {
                els.push(el);
                ts.push(t);
            }
        }
        let mut sorted = els.clone();
        sorted.sort_by(f64::total_cmp);
        let pick = |fr: f64| -> String {
            if sorted.is_empty() {
                return "-".to_string();
            }
            let i = (fr * sorted.len() as f64) as usize;
            format!("{:.1}", sorted[i.min(sorted.len() - 1)])
        };
        let above5 = els.iter().filter(|e| **e > 5.0).count();
        let above0 = els.iter().filter(|e| **e > 0.0).count();
        println!(
            "passcheck {tag}: {n} samples ({n_lock} lock), floor {floor_n} plateau {plat_n}; classified-with-el {}",
            els.len()
        );
        println!(
            "  elevation p1/p10/p50/p90/p99 {}/{}/{}/{}/{}; above0 {above0} above5 {above5}",
            pick(0.01),
            pick(0.1),
            pick(0.5),
            pick(0.9),
            pick(0.99)
        );
        if !ts.is_empty() {
            let tm = |t: f64| -> f64 { 2451545.0 + t / 86400.0 };
            let lo = ts.iter().cloned().fold(f64::INFINITY, f64::min);
            let hi = ts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            println!("  sample t span jd {:.2} .. {:.2}", tm(lo), tm(hi));
        }
    }
}

fn sanity_geometry(eph: &HashMap<String, BodyEphemeris>) {
    let au = 1.495978707e11;
    let start = -94392000.0;
    let (lat0, lon0, alt0) = dsn_station(43).unwrap_or((0.0, 0.0, 0.0));
    let st0 = body_fixed_to_icrs(EARTH, lat0, lon0, alt0, start, eph).unwrap_or([f64::NAN; 3]);
    let e0 = body_barycenter_position(EARTH, start, eph).unwrap_or([f64::NAN; 3]);
    let p0 = body_barycenter_position("galileo_daily", start, eph).unwrap_or([f64::NAN; 3]);
    let off = sub(st0, e0);
    let r_pe = norm(sub(p0, e0)) / au;
    let r_st = norm(off);
    let mut latr = f64::NAN;
    let mut lonr = f64::NAN;
    if let Some((la, lo)) = icrs_to_body_surface(st0[0], st0[1], st0[2], start, EARTH, eph) {
        latr = la;
        lonr = lo;
    }
    println!("sanity frame: start tdb {start:.0} (1997-01-04); DSS43 geodetic ({lat0}, {lon0}, {alt0:.0} m)");
    println!("sanity station offset |st-earth| = {r_st:.0} m (expect ~6378200); recovered body-fixed lat/lon {latr:.3} {lonr:.3} (DSS43 = -35.401 148.982)");
    println!("sanity probe-earth dist {r_pe:.3} AU; earth-barycenter |e| {:.3e} m", norm(e0));
    for (tag, tq) in [
        ("1995-12-15 12:00", -127706400.0f64),
        ("1997-01-04 12:00", -94348800.0f64),
        ("1997-01-04 04:00", -94377600.0f64),
    ] {
        let (Some(pe), Some(ee)) = (
            body_barycenter_position("galileo_daily", tq, eph),
            body_barycenter_position(EARTH, tq, eph),
        ) else {
            continue;
        };
        let v = sub(pe, ee);
        let r = norm(v);
        let dec = (v[2] / r).clamp(-1.0, 1.0).asin().to_degrees();
        let ra = v[1].atan2(v[0]).to_degrees();
        let ra = if ra < 0.0 { ra + 360.0 } else { ra };
        let jd = 2451545.0 + tq / 86400.0;
        let gmst = (280.46061837 + 360.98564736629 * (jd - 2451545.0)).rem_euclid(360.0);
        println!(
            "sanity {tag} tdb {tq:.0}: probe ra {ra:.2} dec {dec:.2} dist {:.3} AU; gmst {gmst:.2}",
            r / 1.495978707e11
        );
        for station in [14i64, 43, 63] {
            let (lat, lon, _alt) = dsn_station(station).unwrap_or((0.0, 0.0, 0.0));
            let lst = (gmst + lon).rem_euclid(360.0);
            let mut ha = (lst - ra).rem_euclid(360.0);
            if ha > 180.0 {
                ha -= 360.0;
            }
            let sinel = lat.to_radians().sin() * dec.to_radians().sin()
                + lat.to_radians().cos() * dec.to_radians().cos() * ha.to_radians().cos();
            let el = sinel.clamp(-1.0, 1.0).asin().to_degrees();
            println!(
                "   st{station} lat {lat:.1} lon {lon:.1}: lst {lst:.2} ha {ha:.2} textbook-el {el:.2}"
            );
        }
    }

    for station in [14i64, 43, 63] {
        let (lat, lon, alt) = dsn_station(station).unwrap_or((0.0, 0.0, 0.0));
        let mut els: Vec<f64> = Vec::new();
        for k in 0..24 {
            let t = start + (k as f64) * 3600.0;
            let st = body_fixed_to_icrs(EARTH, lat, lon, alt, t, eph);
            let e = body_barycenter_position(EARTH, t, eph);
            let p = body_barycenter_position("galileo_daily", t, eph);
            if let (Some(st), Some(e), Some(p)) = (st, e, p) {
                let up = sub(st, e);
                let los = sub(p, st);
                let nu = norm(up);
                let nl = norm(los);
                if nu > 0.0 && nl > 0.0 && nu.is_finite() && nl.is_finite() {
                    els.push((dot(los, up) / (nu * nl)).clamp(-1.0, 1.0).asin().to_degrees());
                }
            }
        }
        let mut sorted = els.clone();
        sorted.sort_by(f64::total_cmp);
        let pick = |frac: f64| -> String {
            if sorted.is_empty() {
                return "-".to_string();
            }
            let i = ((sorted.len() as f64) * frac) as usize;
            let i = i.min(sorted.len() - 1);
            format!("{:.1}", sorted[i])
        };
        let row: Vec<String> = els
            .iter()
            .enumerate()
            .filter(|(k, _)| k % 3 == 0)
            .map(|(_, v)| format!("{v:.1}"))
            .collect();
        println!(
            "sanity st{station} elevation sweep h0..h21 (3 h step): {}",
            row.join(" ")
        );
        println!("  min {} p10 {} p50 {} p90 {} max {}", pick(0.0), pick(0.1), pick(0.5), pick(0.9), pick(1.0));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let nums: Vec<f64> = args.iter().filter_map(|a| a.parse::<f64>().ok()).collect();
    let gap_s = nums.first().copied().unwrap_or(DEFAULT_GAP_S);
    let band_w = nums.get(1).copied().unwrap_or(5.0);

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load_eph(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
            return;
        }
    }

    if args.iter().any(|a| a == "sanity") {
        sanity_geometry(&eph);
        return;
    }

    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    if args.iter().any(|a| a == "passcheck") {
        passcheck(&recs, &eph);
        return;
    }
    drop(bytes);

    let mut passmap: BTreeMap<(i64, i64), Vec<PassStat>> = BTreeMap::new();
    let mut state: BTreeMap<(i64, i64), KeyState> = BTreeMap::new();
    let mut mode_samples: BTreeMap<i64, usize> = BTreeMap::new();
    let mut mode_lock: BTreeMap<i64, usize> = BTreeMap::new();
    let mut el_missing: BTreeMap<i64, usize> = BTreeMap::new();

    for r in &recs {
        let mode = r[3] as i64;
        if mode != 1 && mode != 2 {
            continue;
        }
        let station = r[2] as i64;
        if !STATIONS.contains(&station) {
            continue;
        }
        *mode_samples.entry(mode).or_insert(0) += 1;
        let t = r[0];
        let resid = r[1];
        let key = (mode, station);
        if resid.abs() > LOCK_HZ {
            *mode_lock.entry(mode).or_insert(0) += 1;
        }
        let ks = state.entry(key).or_insert_with(KeyState::new);
        let boundary = match ks.prev {
            None => true,
            Some(prev) => t - prev > gap_s,
        };
        if boundary {
            if let Some(mut pass) = ks.pass.take() {
                let mut stat = finish_pass(&mut pass);
                stat.mode = mode;
                stat.station = station;
                passmap.entry(key).or_default().push(stat);
            }
        }
        let pass = ks.pass.get_or_insert_with(|| PassOpen::start(t, band_w));
        pass.t_last = t;
        if resid.abs() <= LOCK_HZ {
            let s = r[7];
            let lbl = classify(s);
            if lbl == 0 {
                if let Some(cur) = pass.cur.take() {
                    flush_chunk(pass, cur.label, &cur.samples);
                }
            } else {
                match lbl {
                    1 => pass.floor_all += 1,
                    2 => pass.plateau_all += 1,
                    _ => {}
                }
                let el = if mode == 1 { elevation_at(t, station, &eph) } else { None };
                if el.is_none() && mode == 1 {
                    *el_missing.entry(mode).or_insert(0) += 1;
                }
                let need_new = match &pass.cur {
                    None => true,
                    Some(c) => {
                        c.label != lbl
                            || t - c.samples[c.samples.len() - 1].t > SUB_GAP_S
                            || c.samples.len() >= CHUNK_N
                    }
                };
                if need_new {
                    if let Some(cur) = pass.cur.take() {
                        flush_chunk(pass, cur.label, &cur.samples);
                    }
                    pass.cur = Some(OpenChunk {
                        label: lbl,
                        samples: vec![Sample { t, resid, el }],
                    });
                } else if let Some(c) = pass.cur.as_mut() {
                    c.samples.push(Sample { t, resid, el });
                }
            }
        }
        ks.prev = Some(t);
    }
    for ((mode, station), ks) in &mut state {
        if let Some(mut pass) = ks.pass.take() {
            let mut stat = finish_pass(&mut pass);
            stat.mode = *mode;
            stat.station = *station;
            passmap.entry((*mode, *station)).or_default().push(stat);
        }
    }
    drop(state);

    let mut out: Vec<String> = Vec::new();
    out.push("galileo in-pass elevation match — floor vs plateau noise at matched probe elevation".to_string());
    out.push("binding: pass/sub-arc/state construction identical to galileo_pass_strength_ramp (gap > gap_s pass boundary; |resid| > 1000 Hz lock excluded; strength floor <= -2560, plateau >= -1900, between or 0 = transition/pad; sub-arc = contiguous same-state run split on state change, > 120 s gap, or 60 samples; chunk noise = resid RMS about the chunk mean; chunk >= 30 samples enters the pool)".to_string());
    out.push("elevation proxy: spherical-astronomy topocentric elevation of the probe above the station horizon; probe topocentric direction = galileo_daily barycenter minus earth barycenter (ICRS RA/Dec; station parallax negligible at ~6 AU, probe-earth dist 1-6 AU over the era); station geodetic position via dsn_station (DSS 14 35.4268333N -116.8900000E, DSS 43 -35.4014889N 148.9816167E, DSS 63 40.4312500N -4.2487778E); local sidereal time = GMST (IAU 1982: 280.46061837 + 360.98564736629 deg/day from J2000; tdb~UT1 to ~1 min, equinox-of-date vs ICRS RA <= ~0.5 deg) + east longitude; elevation = asin(sin lat sin dec + cos lat cos dec cos HA), horizon = 0 deg; validated by pass gating (file samples occupy exactly the positive-elevation hours, e.g. DSS43 1997-01-04 peak ~+71 deg at 04:00 UTC)".to_string());
    out.push("matched-elevation comparison: (A) per dual pass, floor and plateau sub-arcs whose chunk-mean elevations both fall within a tolerance window of width 2T (best common window per pass, min(state sample n) maximised, both >= 30); (B) per pass per elevation band of width band_w, sample-level floor vs plateau noise within the same band".to_string());
    out.push(format!(
        "pass gap threshold: {gap_s:.0} s; band width W {band_w:.0} deg; stations 14/43/63; modes 1 (elevation computed) and 2 (structure only)"
    ));
    out.push(String::new());

    out.push("overview".to_string());
    for mode in [1i64, 2] {
        out.push(format!(
            "  mode {mode}: {} samples at 14/43/63, {} lock transitions",
            mode_samples.get(&mode).copied().unwrap_or(0),
            mode_lock.get(&mode).copied().unwrap_or(0)
        ));
    }
    out.push(String::new());

    out.push("pass structure per station per mode (dual_full = floor pool >= 30 and plateau pool >= 30)".to_string());
    out.push("  st mode passes floor_pres plateau_pres dual_full dual_int floor_only plateau_only neither".to_string());
    let mut keys: Vec<(i64, i64)> = passmap.keys().copied().collect();
    keys.sort();
    for (mode, station) in &keys {
        let list = &passmap[&(*mode, *station)];
        let mut fp = 0usize;
        let mut pp = 0usize;
        let mut dual_f = 0usize;
        let mut dual_i = 0usize;
        let mut f_only = 0usize;
        let mut p_only = 0usize;
        let mut neither = 0usize;
        for ps in list {
            let hf = ps.floor_all >= MIN_CELL;
            let hp = ps.plateau_all >= MIN_CELL;
            let hfd = ps.f.n >= MIN_CELL && ps.p.n >= MIN_CELL;
            let hid = ps.fi.n >= MIN_CELL && ps.pi.n >= MIN_CELL;
            if hf {
                fp += 1;
            }
            if hp {
                pp += 1;
            }
            if hfd {
                dual_f += 1;
            }
            if hid {
                dual_i += 1;
            }
            if hf && hp {
            } else if hf {
                f_only += 1;
            } else if hp {
                p_only += 1;
            } else {
                neither += 1;
            }
        }
        out.push(format!(
            "  {station} {mode} {} {} {} {} {} {} {} {}",
            list.len(),
            fp,
            pp,
            dual_f,
            dual_i,
            f_only,
            p_only,
            neither
        ));
    }
    out.push(String::new());

    out.push("anchor replication (unrestricted within-pass paired noise), mode 1: floor vs plateau of the same pass".to_string());
    out.push("  st n_dual med_floor med_plateau med_diff mean_diff floor>plat floor<plat med_ratio".to_string());
    for station in STATIONS {
        let key = (1, station);
        let mut dv: Vec<f64> = Vec::new();
        let mut rv: Vec<f64> = Vec::new();
        let mut fv: Vec<f64> = Vec::new();
        let mut pv: Vec<f64> = Vec::new();
        if let Some(list) = passmap.get(&key) {
            for ps in list {
                if ps.f.n < MIN_CELL || ps.p.n < MIN_CELL {
                    continue;
                }
                let fr = ps.f.rms().unwrap_or(f64::NAN);
                let pr = ps.p.rms().unwrap_or(f64::NAN);
                if !fr.is_finite() || !pr.is_finite() {
                    continue;
                }
                dv.push(fr - pr);
                rv.push(fr / pr);
                fv.push(fr);
                pv.push(pr);
            }
        }
        let above = dv.iter().filter(|d| **d > 0.0).count();
        let below = dv.iter().filter(|d| **d < 0.0).count();
        let mean_d = if dv.is_empty() {
            None
        } else {
            Some(dv.iter().sum::<f64>() / dv.len() as f64)
        };
        out.push(format!(
            "  {station} {} {} {} {} {} {} {} {}",
            dv.len(),
            fmt_o(median(&fv)),
            fmt_o(median(&pv)),
            fmt_o(median(&dv)),
            fmt_o(mean_d),
            above,
            below,
            fmt_o(median(&rv))
        ));
    }
    out.push(String::new());

    out.push("anchor replication interior (120 s edge excluded), mode 1".to_string());
    out.push("  st n_dual med_floor med_plateau med_diff mean_diff floor>plat floor<plat med_ratio".to_string());
    for station in STATIONS {
        let key = (1, station);
        let mut dv: Vec<f64> = Vec::new();
        let mut rv: Vec<f64> = Vec::new();
        let mut fv: Vec<f64> = Vec::new();
        let mut pv: Vec<f64> = Vec::new();
        if let Some(list) = passmap.get(&key) {
            for ps in list {
                if ps.fi.n < MIN_CELL || ps.pi.n < MIN_CELL {
                    continue;
                }
                let fr = ps.fi.rms().unwrap_or(f64::NAN);
                let pr = ps.pi.rms().unwrap_or(f64::NAN);
                if !fr.is_finite() || !pr.is_finite() {
                    continue;
                }
                dv.push(fr - pr);
                rv.push(fr / pr);
                fv.push(fr);
                pv.push(pr);
            }
        }
        let above = dv.iter().filter(|d| **d > 0.0).count();
        let below = dv.iter().filter(|d| **d < 0.0).count();
        let mean_d = if dv.is_empty() {
            None
        } else {
            Some(dv.iter().sum::<f64>() / dv.len() as f64)
        };
        out.push(format!(
            "  {station} {} {} {} {} {} {} {} {}",
            dv.len(),
            fmt_o(median(&fv)),
            fmt_o(median(&pv)),
            fmt_o(median(&dv)),
            fmt_o(mean_d),
            above,
            below,
            fmt_o(median(&rv))
        ));
    }
    out.push(String::new());

    out.push("mode 1 elevation geometry of the states within dual passes".to_string());
    out.push("  st n_dual med_floor_el med_plateau_el med_el_delta passes_floor_lower frac_floor_el_lower_5".to_string());
    for station in STATIONS {
        let key = (1, station);
        let mut fd: Vec<f64> = Vec::new();
        let mut pv: Vec<f64> = Vec::new();
        let mut dd: Vec<f64> = Vec::new();
        let mut lower = 0usize;
        let mut lower5 = 0usize;
        if let Some(list) = passmap.get(&key) {
            for ps in list {
                if ps.f.n < MIN_CELL || ps.p.n < MIN_CELL {
                    continue;
                }
                let fe = median(&ps.floor_els);
                let pe = median(&ps.plateau_els);
                let (Some(fe), Some(pe)) = (fe, pe) else {
                    continue;
                };
                fd.push(fe);
                pv.push(pe);
                dd.push(fe - pe);
                if fe < pe {
                    lower += 1;
                    if pe - fe <= 5.0 {
                        lower5 += 1;
                    }
                }
            }
        }
        out.push(format!(
            "  {station} {} {} {} {} {} {}",
            dd.len(),
            fmt_o(median(&fd)),
            fmt_o(median(&pv)),
            fmt_o(median(&dd)),
            lower,
            lower5
        ));
    }
    out.push(String::new());

    for tol in [3.0f64, 5.0, 8.0] {
        out.push(format!(
            "matched-elevation paired test A (same pass, common elevation window width 2T = {:.0} deg, both state pools >= 30 samples)",
            tol * 2.0
        ));
        out.push("  st n_pass_match n_pass_nomatch med_floor med_plateau med_diff mean_diff floor>plat floor<plat med_ratio".to_string());
        for station in STATIONS {
            let key = (1, station);
            let mut dv: Vec<f64> = Vec::new();
            let mut rv: Vec<f64> = Vec::new();
            let mut fv: Vec<f64> = Vec::new();
            let mut pv: Vec<f64> = Vec::new();
            let mut n_no = 0usize;
            if let Some(list) = passmap.get(&key) {
                for ps in list {
                    if ps.f.n < MIN_CELL || ps.p.n < MIN_CELL {
                        continue;
                    }
                    match pass_match(ps, tol) {
                        Some((fr, pr, _fc, _pc, _cen)) => {
                            dv.push(fr - pr);
                            rv.push(fr / pr);
                            fv.push(fr);
                            pv.push(pr);
                        }
                        None => n_no += 1,
                    }
                }
            }
            let above = dv.iter().filter(|d| **d > 0.0).count();
            let below = dv.iter().filter(|d| **d < 0.0).count();
            let mean_d = if dv.is_empty() {
                None
            } else {
                Some(dv.iter().sum::<f64>() / dv.len() as f64)
            };
            out.push(format!(
                "  {station} {} {} {} {} {} {} {} {} {}",
                dv.len(),
                n_no,
                fmt_o(median(&fv)),
                fmt_o(median(&pv)),
                fmt_o(median(&dv)),
                fmt_o(mean_d),
                above,
                below,
                fmt_o(median(&rv))
            ));
        }
        out.push(String::new());
    }

    out.push(format!(
        "matched-elevation paired test B (same pass AND same elevation band of width {band_w:.0} deg, both state pools >= 30 samples)"
    ));
    out.push("  st n_pairs med_floor med_plateau med_diff floor>plat floor<plat med_ratio".to_string());
    for station in STATIONS {
        let key = (1, station);
        let mut dv: Vec<f64> = Vec::new();
        let mut rv: Vec<f64> = Vec::new();
        let mut fv: Vec<f64> = Vec::new();
        let mut pv: Vec<f64> = Vec::new();
        if let Some(list) = passmap.get(&key) {
            for ps in list {
                for (_, cell) in &ps.cells {
                    let fc = cell[0];
                    let pc = cell[1];
                    if fc.n < MIN_CELL || pc.n < MIN_CELL {
                        continue;
                    }
                    let (Some(fr), Some(pr)) = (fc.rms(), pc.rms()) else {
                        continue;
                    };
                    if !fr.is_finite() || !pr.is_finite() {
                        continue;
                    }
                    dv.push(fr - pr);
                    rv.push(fr / pr);
                    fv.push(fr);
                    pv.push(pr);
                }
            }
        }
        let above = dv.iter().filter(|d| **d > 0.0).count();
        let below = dv.iter().filter(|d| **d < 0.0).count();
        out.push(format!(
            "  {station} {} {} {} {} {} {} {}",
            dv.len(),
            fmt_o(median(&fv)),
            fmt_o(median(&pv)),
            fmt_o(median(&dv)),
            above,
            below,
            fmt_o(median(&rv))
        ));
    }
    out.push(String::new());

    out.push("per-pass matched pairs (test A, T = 5.0), mode 1: date el_center floor_rms plat_rms diff floor_n plat_n".to_string());
    for station in [43i64, 63, 14] {
        let key = (1, station);
        if let Some(list) = passmap.get(&key) {
            for ps in list {
                if ps.f.n < MIN_CELL || ps.p.n < MIN_CELL {
                    continue;
                }
                if let Some((fr, pr, fc, pc, cen)) = pass_match(ps, 5.0) {
                    out.push(format!(
                        "  st{station} {} {:.1} {:.3} {:.3} {:.3} {} {}",
                        jd_date(ps.t0),
                        cen,
                        fr,
                        pr,
                        fr - pr,
                        fc,
                        pc
                    ));
                }
            }
        }
    }
    out.push(String::new());

    out.push("elevation availability (mode 1 classified samples)".to_string());
    out.push(format!(
        "  samples without probe/station ephemeris elevation: {}",
        el_missing.get(&1).copied().unwrap_or(0)
    ));
    for station in [43i64, 63, 14] {
        let key = (1, station);
        let mut ylo: BTreeMap<i64, (usize, usize, f64)> = BTreeMap::new();
        let mut all_s: Vec<f64> = Vec::new();
        if let Some(list) = passmap.get(&key) {
            for ps in list {
                let y = (2451545.0 + ps.t0 / DAY_S) as i64;
                let year = ((y - 2440587) as f64 / 365.2425).floor() as i64 + 1970;
                for c in &ps.chunks {
                    if let Some(el) = c.el_mean {
                        let e = ylo.entry(year).or_insert((0, 0, 0.0));
                        e.0 += 1;
                        if el < 5.0 {
                            e.1 += 1;
                        }
                        e.2 += el;
                        all_s.push(el);
                    }
                }
            }
        }
        let mut parts: Vec<String> = Vec::new();
        for (year, (n, nlow, esum)) in &ylo {
            let med = if *n > 0 { esum / *n as f64 } else { f64::NAN };
            parts.push(format!(
                "{year}: {n} chunks, {:.0}% <5 deg, mean el {med:.1}",
                100.0 * *nlow as f64 / *n as f64
            ));
        }
        out.push(format!("  st{station} chunk-el by year: {}", parts.join(" | ")));
    }
    out.push(String::new());

    let body = out.join("\n") + "\n";
    println!("{body}");
}

fn pass_match(
    ps: &PassStat,
    tol: f64,
) -> Option<(f64, f64, usize, usize, f64)> {
    let mut fc: Vec<(f64, usize, f64)> = Vec::new();
    let mut pc: Vec<(f64, usize, f64)> = Vec::new();
    for c in &ps.chunks {
        let Some(el) = c.el_mean else {
            continue;
        };
        if c.label == 1 {
            fc.push((el, c.n, c.dev2));
        } else {
            pc.push((el, c.n, c.dev2));
        }
    }
    if fc.is_empty() || pc.is_empty() {
        return None;
    }
    let mut best: Option<(usize, usize, f64, f64, f64)> = None;
    let lo = fc
        .iter()
        .chain(pc.iter())
        .map(|(el, _, _)| *el)
        .fold(f64::INFINITY, f64::min);
    let hi = fc
        .iter()
        .chain(pc.iter())
        .map(|(el, _, _)| *el)
        .fold(f64::NEG_INFINITY, f64::max);
    let mut c = lo - tol;
    while c <= hi + tol {
        let mut fn_ = 0usize;
        let mut fdev2 = 0.0f64;
        let mut pn = 0usize;
        let mut pdev2 = 0.0f64;
        for (el, n, dev2) in &fc {
            if (el - c).abs() <= tol {
                fn_ += n;
                fdev2 += dev2;
            }
        }
        for (el, n, dev2) in &pc {
            if (el - c).abs() <= tol {
                pn += n;
                pdev2 += dev2;
            }
        }
        if fn_ >= MIN_CELL && pn >= MIN_CELL {
            let better = match best {
                None => true,
                Some((bfn, bpn, _, _, _)) => {
                    fn_.min(pn) > bfn.min(bpn)
                        || (fn_.min(pn) == bfn.min(bpn) && fn_ + pn > bfn + bpn)
                }
            };
            if better {
                best = Some((fn_, pn, fdev2, pdev2, c));
            }
        }
        c += 0.5;
    }
    let (fn_, pn, fdev2, pdev2, cen) = best?;
    let fr = (fdev2 / fn_ as f64).sqrt();
    let pr = (pdev2 / pn as f64).sqrt();
    Some((fr, pr, fn_, pn, cen))
}

fn load_eph(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    let p = format!("data/ephemeris_{name}.bin");
    std::fs::read(&p)
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| {
            eph.insert(name.to_string(), e);
        })
        .is_some()
}
