use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{self, MAGIC_GOEV};
use omegaflow::mathematikerin::te::transfer_entropy_binned;
use std::collections::{BTreeMap, BTreeSet, HashMap};

const HOUR: f64 = 3_600.0;
const MAX_PAIR_KM: f64 = 1_000.0;
const SURROGATES: usize = 500;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAGS: [i64; 7] = [-3, -2, -1, 0, 1, 2, 3];
const TE_LAGS: [usize; 4] = [0, 1, 2, 3];
const TE_BINS: usize = 4;
const MIN_SEGMENT_H: usize = 96;
const MAX_GAP_H: i64 = 3;

struct Station {
    code: &'static str,
    lat: f64,
    lon: f64,
}

const STATIONS: [Station; 6] = [
    Station {
        code: "FRD",
        lat: 38.205,
        lon: -77.373,
    },
    Station {
        code: "BOU",
        lat: 40.137,
        lon: -105.237,
    },
    Station {
        code: "NEW",
        lat: 48.265,
        lon: -117.122,
    },
    Station {
        code: "TUC",
        lat: 32.174,
        lon: -110.733,
    },
    Station {
        code: "BSL",
        lat: 30.350,
        lon: -89.635,
    },
    Station {
        code: "FRN",
        lat: 37.091,
        lon: -119.718,
    },
];

const N_ST: usize = STATIONS.len();

#[derive(Clone, Copy)]
struct Event {
    unix: f64,
    hour: i64,
    lat: f64,
    lon: f64,
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn iso_day(day: i64) -> String {
    let (y, m, d) = civil_from_days(day);
    format!("{y:04}-{m:02}-{d:02}")
}

fn iso_hour(hour: i64) -> String {
    let day = (hour as f64 * HOUR / 86_400.0).floor() as i64;
    let hh = hour - day * 24;
    format!("{}T{hh:02}:00Z", iso_day(day))
}

fn haversine_km(a_lat: f64, a_lon: f64, b_lat: f64, b_lon: f64) -> f64 {
    let r = 6_371.0;
    let p1 = a_lat.to_radians();
    let p2 = b_lat.to_radians();
    let dp = (b_lat - a_lat).to_radians();
    let dl = (b_lon - a_lon).to_radians();
    let h = (dp / 2.0).sin().powi(2) + p1.cos() * p2.cos() * (dl / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin().min(std::f64::consts::PI)
}

fn pearson(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < 3 || x.len() != y.len() {
        return None;
    }
    let nf = n as f64;
    let mx = x.iter().sum::<f64>() / nf;
    let my = y.iter().sum::<f64>() / nf;
    let mut sxy = 0.0f64;
    let mut sxx = 0.0f64;
    let mut syy = 0.0f64;
    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx <= f64::MIN_POSITIVE || syy <= f64::MIN_POSITIVE {
        return None;
    }
    Some(sxy / (sxx.sqrt() * syy.sqrt()))
}

fn ratio(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    })
}

fn quantile(sorted: &[f64], p: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let pos = p * (sorted.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    let f = pos - lo as f64;
    Some(sorted[lo] * (1.0 - f) + sorted[hi] * f)
}

fn load_events(path: &str, lsk: &omegaflow::archivar::lsk::LeapSeconds) -> Option<Vec<Event>> {
    let bytes = std::fs::read(path).ok()?;
    let recs = geo::parse_bin(MAGIC_GOEV, &bytes)?;
    let mut groups: HashMap<(u64, u64, u64), (f64, f64, f64)> = HashMap::new();
    for r in &recs {
        let key = (r.t.to_bits(), r.lat.to_bits(), r.lon.to_bits());
        groups.entry(key).or_insert((r.t, r.lat, r.lon));
    }
    let mut out = Vec::with_capacity(groups.len());
    for (t, lat, lon) in groups.into_values() {
        let Some(unix) = lsk.tdb_to_unix(t) else {
            continue;
        };
        out.push(Event {
            unix,
            hour: (unix / HOUR).floor() as i64,
            lat,
            lon,
        });
    }
    out.sort_by(|a, b| a.unix.total_cmp(&b.unix));
    Some(out)
}

fn assign_counties(events: &[Event]) -> (HashMap<(u64, u64), usize>, [usize; N_ST], [f64; N_ST]) {
    let mut counties: BTreeSet<(u64, u64)> = BTreeSet::new();
    for e in events {
        counties.insert((e.lat.to_bits(), e.lon.to_bits()));
    }
    let mut map = HashMap::new();
    let mut n_counties = [0usize; N_ST];
    let mut max_dist = [0.0f64; N_ST];
    for key in counties {
        let lat = f64::from_bits(key.0);
        let lon = f64::from_bits(key.1);
        let mut best: Option<(usize, f64)> = None;
        for (i, s) in STATIONS.iter().enumerate() {
            let d = haversine_km(lat, lon, s.lat, s.lon);
            if best.map_or(true, |(_, bd)| d < bd) {
                best = Some((i, d));
            }
        }
        if let Some((i, d)) = best {
            if d <= MAX_PAIR_KM {
                map.insert(key, i);
                n_counties[i] += 1;
                max_dist[i] = max_dist[i].max(d);
            }
        }
    }
    (map, n_counties, max_dist)
}

fn parse_iaga_minutes(text: &str) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    let mut started = false;
    for line in text.lines() {
        let t = line.trim();
        if !started {
            if t.starts_with("DATE") {
                started = true;
            }
            continue;
        }
        let c: Vec<&str> = t.split_whitespace().collect();
        if c.len() < 5 {
            continue;
        }
        let mut dp = c[0].split('-');
        let (Some(y), Some(mo), Some(da)) = (dp.next(), dp.next(), dp.next()) else {
            continue;
        };
        let (Ok(y), Ok(mo), Ok(da)) = (y.parse::<i64>(), mo.parse::<i64>(), da.parse::<i64>())
        else {
            continue;
        };
        let Some(days) = omegaflow::archivar::lsk::days_from_civil(y, mo, da) else {
            continue;
        };
        let tt: Vec<&str> = c[1].split(':').collect();
        if tt.len() < 2 {
            continue;
        }
        let (Ok(hh), Ok(mm), Some(Ok(ss))) = (
            tt[0].parse::<i64>(),
            tt[1].parse::<i64>(),
            tt.get(2).map(|v| v.parse::<f64>()),
        ) else {
            continue;
        };
        let Some(x) = c[3].parse::<f64>().ok() else {
            continue;
        };
        let Some(y) = c[4].parse::<f64>().ok() else {
            continue;
        };
        if !x.is_finite() || !y.is_finite() || x.abs() > 90_000.0 || y.abs() > 90_000.0 {
            continue;
        }
        let unix = days as f64 * 86_400.0 + hh as f64 * 3_600.0 + mm as f64 * 60.0 + ss;
        out.push((unix, x, y));
    }
    out
}

fn load_station(dir: &str, code: &str) -> BTreeMap<i64, f64> {
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.starts_with(&format!("{code}_")) && s.ends_with(".iaga"))
                .unwrap_or(false)
        })
        .collect();
    files.sort();
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let rows = parse_iaga_minutes(&text);
        let mut prev: Option<(f64, f64)> = None;
        for (unix, x, y) in rows {
            let h = (x * x + y * y).sqrt();
            if let Some((pt, ph)) = prev {
                let dt = unix - pt;
                if (30.0..=180.0).contains(&dt) {
                    let dbdt = (h - ph).abs() / dt * 60.0;
                    let hour = (unix / HOUR).floor() as i64;
                    let slot = out.entry(hour).or_insert(0.0);
                    if dbdt > *slot {
                        *slot = dbdt;
                    }
                }
            }
            prev = Some((unix, h));
        }
    }
    out
}

fn outage_maps(
    events: &[Event],
    assign: &HashMap<(u64, u64), usize>,
    n_counties: &[usize; N_ST],
) -> Vec<BTreeMap<i64, f64>> {
    let mut out: Vec<BTreeMap<i64, f64>> = vec![BTreeMap::new(); N_ST];
    for e in events {
        let Some(&s) = assign.get(&(e.lat.to_bits(), e.lon.to_bits())) else {
            continue;
        };
        let rate = 1_000.0 / n_counties[s] as f64;
        *out[s].entry(e.hour).or_insert(0.0) += rate;
    }
    out
}

struct Hours {
    base: i64,
    len: usize,
    hour: Vec<Vec<i64>>,
    b: Vec<Vec<f64>>,
}

impl Hours {
    fn build(bmaps: &[BTreeMap<i64, f64>], lo: i64, hi: i64) -> Self {
        let base = lo - 4;
        let len = (hi - base + 5) as usize;
        let mut hour = Vec::with_capacity(N_ST);
        let mut b = Vec::with_capacity(N_ST);
        for m in bmaps {
            let mut h = Vec::with_capacity(m.len());
            let mut v = Vec::with_capacity(m.len());
            for (&q, &x) in m {
                if q < lo || q > hi {
                    continue;
                }
                h.push(q);
                v.push(x);
            }
            hour.push(h);
            b.push(v);
        }
        Hours { base, len, hour, b }
    }
}

fn count_vec(omap: &BTreeMap<i64, f64>, base: i64, len: usize) -> Vec<f64> {
    let mut out = vec![0.0; len];
    for (&h, &v) in omap {
        let i = h - base;
        if i >= 0 && (i as usize) < len {
            out[i as usize] = v;
        }
    }
    out
}

fn panel_r(hours: &Hours, counts: &[Vec<f64>], lag: i64) -> Option<(f64, usize)> {
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut n = 0usize;
    for s in 0..N_ST {
        for (i, &q) in hours.hour[s].iter().enumerate() {
            let oh = q - lag - hours.base;
            if oh < 0 || oh as usize >= hours.len {
                continue;
            }
            let x = hours.b[s][i];
            let y = counts[s][oh as usize];
            sx += x;
            sy += y;
            sxy += x * y;
            sxx += x * x;
            syy += y * y;
            n += 1;
        }
    }
    if n < 3 {
        return None;
    }
    let nf = n as f64;
    let vx = sxx - sx * sx / nf;
    let vy = syy - sy * sy / nf;
    if vx <= f64::MIN_POSITIVE || vy <= f64::MIN_POSITIVE {
        return None;
    }
    Some(((sxy - sx * sy / nf) / (vx.sqrt() * vy.sqrt()), n))
}

fn panel_r_diurnal(hours: &Hours, counts: &[Vec<f64>], lag: i64) -> Option<(f64, usize)> {
    let mut mx = [[0.0f64; 24]; N_ST];
    let mut my = [[0.0f64; 24]; N_ST];
    let mut nx = [[0usize; 24]; N_ST];
    let mut ny = [[0usize; 24]; N_ST];
    for s in 0..N_ST {
        for (i, &q) in hours.hour[s].iter().enumerate() {
            let hod = q.rem_euclid(24) as usize;
            mx[s][hod] += hours.b[s][i];
            nx[s][hod] += 1;
            let idx = q - hours.base;
            if idx >= 0 && (idx as usize) < hours.len {
                my[s][hod] += counts[s][idx as usize];
                ny[s][hod] += 1;
            }
        }
    }
    for s in 0..N_ST {
        for h in 0..24 {
            if nx[s][h] > 0 {
                mx[s][h] /= nx[s][h] as f64;
            }
            if ny[s][h] > 0 {
                my[s][h] /= ny[s][h] as f64;
            }
        }
    }
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut n = 0usize;
    for s in 0..N_ST {
        for (i, &q) in hours.hour[s].iter().enumerate() {
            let oh = q - lag;
            let idx = oh - hours.base;
            if idx < 0 || idx as usize >= hours.len {
                continue;
            }
            let hx = q.rem_euclid(24) as usize;
            let hy = oh.rem_euclid(24) as usize;
            let x = hours.b[s][i] - mx[s][hx];
            let y = counts[s][idx as usize] - my[s][hy];
            sx += x;
            sy += y;
            sxy += x * y;
            sxx += x * x;
            syy += y * y;
            n += 1;
        }
    }
    if n < 3 {
        return None;
    }
    let nf = n as f64;
    let vx = sxx - sx * sx / nf;
    let vy = syy - sy * sy / nf;
    if vx <= f64::MIN_POSITIVE || vy <= f64::MIN_POSITIVE {
        return None;
    }
    Some(((sxy - sx * sy / nf) / (vx.sqrt() * vy.sqrt()), n))
}

fn best_panel(hours: &Hours, counts: &[Vec<f64>]) -> Option<(f64, i64, usize)> {
    let mut best: Option<(f64, i64, usize)> = None;
    for lag in LAGS {
        if let Some((r, n)) = panel_r(hours, counts, lag) {
            if best.map_or(true, |(br, _, _)| r.abs() > br.abs()) {
                best = Some((r, lag, n));
            }
        }
    }
    best
}

fn national_series(
    bmaps: &[BTreeMap<i64, f64>],
    events: &[Event],
    n_counties: usize,
    lo: i64,
    hi: i64,
) -> (BTreeMap<i64, f64>, BTreeMap<i64, f64>) {
    let mut sum: BTreeMap<i64, (f64, usize)> = BTreeMap::new();
    for m in bmaps {
        for (&h, &v) in m {
            let slot = sum.entry(h).or_insert((0.0, 0));
            slot.0 += v;
            slot.1 += 1;
        }
    }
    let b: BTreeMap<i64, f64> = sum
        .into_iter()
        .filter(|(h, _)| *h >= lo && *h <= hi)
        .map(|(h, (s, n))| (h, s / n as f64))
        .collect();
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for e in events {
        let rate = 1_000.0 / n_counties as f64;
        *out.entry(e.hour).or_insert(0.0) += rate;
    }
    for h in lo..=hi {
        out.entry(h).or_insert(0.0);
    }
    (b, out)
}

fn national_r(
    b: &BTreeMap<i64, f64>,
    o: &BTreeMap<i64, f64>,
    lo: i64,
    hi: i64,
    lag: i64,
) -> Option<(f64, usize)> {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (&q, &x) in b {
        let oh = q - lag;
        if oh < lo || oh > hi {
            continue;
        }
        xs.push(x);
        ys.push(o[&oh]);
    }
    pearson(&xs, &ys).map(|r| (r, xs.len()))
}

fn segments_hours(bmap: &BTreeMap<i64, f64>, lo: i64, hi: i64) -> Vec<(Vec<i64>, Vec<f32>)> {
    let mut out = Vec::new();
    let mut hs: Vec<i64> = Vec::new();
    let mut xs: Vec<f32> = Vec::new();
    let mut last: Option<i64> = None;
    let mut h = lo;
    while h <= hi {
        let b = bmap.get(&h).copied();
        if let Some(v) = b {
            if let Some(pl) = last {
                if h - pl > MAX_GAP_H {
                    if xs.len() >= MIN_SEGMENT_H {
                        out.push((std::mem::take(&mut hs), std::mem::take(&mut xs)));
                    } else {
                        hs.clear();
                        xs.clear();
                    }
                } else if h - pl > 1 {
                    if let Some(&pv) = xs.last() {
                        let gap = (h - pl) as usize;
                        for k in 1..gap {
                            let f = k as f32 / gap as f32;
                            hs.push(pl + k as i64);
                            xs.push(pv + (v as f32 - pv) * f);
                        }
                    }
                }
            }
            hs.push(h);
            xs.push(v as f32);
            last = Some(h);
        }
        h += 1;
    }
    if xs.len() >= MIN_SEGMENT_H {
        out.push((hs, xs));
    }
    out
}

fn te_for_segments(
    segs: &[Vec<(Vec<i64>, Vec<f32>)>],
    counts: &[Vec<f64>],
    base: i64,
    len: usize,
    lag: usize,
) -> Option<(f64, usize)> {
    let mut acc = 0.0f64;
    let mut weight = 0usize;
    for s in 0..N_ST {
        for (hs, x) in &segs[s] {
            let mut y: Vec<f32> = Vec::with_capacity(hs.len());
            for &h in hs {
                let i = h - base;
                let v = if i >= 0 && (i as usize) < len {
                    counts[s][i as usize]
                } else {
                    0.0
                };
                y.push(v as f32);
            }
            if let Some(te) = transfer_entropy_binned(&y, x, lag, TE_BINS) {
                acc += te * x.len() as f64;
                weight += x.len();
            }
        }
    }
    if weight == 0 {
        return None;
    }
    Some((acc / weight as f64, weight))
}

fn te_for(
    segs: &[Vec<(Vec<i64>, Vec<f32>)>],
    counts: &[Vec<f64>],
    base: i64,
    len: usize,
    lags: &[usize],
) -> Option<(f64, usize, usize)> {
    let mut best: Option<(f64, usize, usize)> = None;
    for &lag in lags {
        if let Some((te, weight)) = te_for_segments(segs, counts, base, len, lag) {
            if best.map_or(true, |(bt, _, _)| te > bt) {
                best = Some((te, lag, weight));
            }
        }
    }
    best
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let outage_path =
        arg_after(&args, "--outage").unwrap_or("/tmp/opencode/ecod_usgs/grid_outage_events.bin");
    let dir = arg_after(&args, "--dir").unwrap_or("/tmp/opencode/ecod_usgs/iaga");
    let draws = arg_after(&args, "--draws")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(SURROGATES);
    let Some(lsk) = embedded_lsk() else {
        eprintln!("embedded leap-second table stays unread");
        std::process::exit(1);
    };
    let Some(events) = load_events(outage_path, &lsk) else {
        eprintln!("{outage_path}: outage bin reads void");
        std::process::exit(1);
    };
    if events.is_empty() {
        eprintln!("{outage_path}: 0 unique events");
        std::process::exit(1);
    }
    let (assign, n_counties, max_dist) = assign_counties(&events);
    let paired: Vec<&Event> = events
        .iter()
        .filter(|e| assign.contains_key(&(e.lat.to_bits(), e.lon.to_bits())))
        .collect();
    let mut counties_total: BTreeSet<(u64, u64)> = BTreeSet::new();
    for e in &events {
        counties_total.insert((e.lat.to_bits(), e.lon.to_bits()));
    }
    println!(
        "ECOD: {} unique events, {} county centroids, {} .. {} .. {}",
        events.len(),
        counties_total.len(),
        iso_hour(events.first().unwrap().hour),
        iso_hour(events.last().unwrap().hour),
        events.len()
    );
    println!(
        "pair: {} of {} counties within {:.0} km, {} of {} events ({:.2} %), {} absent",
        assign.len(),
        counties_total.len(),
        MAX_PAIR_KM,
        paired.len(),
        events.len(),
        100.0 * paired.len() as f64 / events.len() as f64,
        events.len() - paired.len()
    );
    println!();
    println!("| station | lat | lon | counties | events | max pair km | hours |");
    println!("|---|---:|---:|---:|---:|---:|---:|");
    let mut per_station_events = [0usize; N_ST];
    for e in &paired {
        if let Some(&s) = assign.get(&(e.lat.to_bits(), e.lon.to_bits())) {
            per_station_events[s] += 1;
        }
    }
    let mut bmaps: Vec<BTreeMap<i64, f64>> = Vec::new();
    for (i, s) in STATIONS.iter().enumerate() {
        let b = load_station(dir, s.code);
        println!(
            "| {} | {:.3} | {:.3} | {} | {} | {:.1} | {} |",
            s.code,
            s.lat,
            s.lon,
            n_counties[i],
            per_station_events[i],
            max_dist[i],
            b.len()
        );
        bmaps.push(b);
    }
    if bmaps.iter().all(|m| m.is_empty()) {
        eprintln!("{dir}: no IAGA files parse, measurement stays open");
        std::process::exit(1);
    }
    let lo = events.first().unwrap().hour;
    let hi = events.last().unwrap().hour;
    let omaps = outage_maps(&events, &assign, &n_counties);
    let hours = Hours::build(&bmaps, lo, hi);
    let counts: Vec<Vec<f64>> = omaps
        .iter()
        .map(|m| count_vec(m, hours.base, hours.len))
        .collect();
    let segs: Vec<Vec<(Vec<i64>, Vec<f32>)>> =
        bmaps.iter().map(|m| segments_hours(m, lo, hi)).collect();

    println!();
    println!("## Paired panel, hourly, lag = dB/dt hour minus outage hour");
    println!();
    println!("| lag h | r | n |");
    println!("|---:|---:|---:|");
    let mut best: Option<(f64, i64, usize)> = None;
    for lag in LAGS {
        if let Some((r, n)) = panel_r(&hours, &counts, lag) {
            println!("| {lag:+} | {r:+.5} | {n} |");
            if best.map_or(true, |(br, _, _)| r.abs() > br.abs()) {
                best = Some((r, lag, n));
            }
        }
    }
    let Some((obs_r, obs_lag, obs_n)) = best else {
        eprintln!("panel stays empty");
        std::process::exit(1);
    };
    println!();
    println!(
        "observed max |r| = {:.5} at lag {:+} h, n = {}",
        obs_r.abs(),
        obs_lag,
        obs_n
    );
    if let Some((rd, nd)) = panel_r_diurnal(&hours, &counts, obs_lag) {
        println!("diurnal-adjusted r at the same lag = {rd:+.5}, n = {nd}");
    }

    let mut counties_vec: Vec<(u64, u64, usize)> =
        assign.iter().map(|(&k, &s)| (k.0, k.1, s)).collect();
    counties_vec.sort();
    let mut labels: Vec<usize> = counties_vec.iter().map(|v| v.2).collect();
    let county_keys: Vec<(u64, u64)> = counties_vec.iter().map(|v| (v.0, v.1)).collect();
    let event_by_county: HashMap<(u64, u64), Vec<&Event>> = {
        let mut m: HashMap<(u64, u64), Vec<&Event>> = HashMap::new();
        for e in &paired {
            m.entry((e.lat.to_bits(), e.lon.to_bits()))
                .or_default()
                .push(e);
        }
        m
    };
    let mut rng = Rng(SEED);
    let mut null = Vec::with_capacity(draws);
    for _ in 0..draws {
        for i in (1..labels.len()).rev() {
            let j = rng.below(i + 1);
            labels.swap(i, j);
        }
        let mut shuffled_assign: HashMap<(u64, u64), usize> =
            HashMap::with_capacity(county_keys.len());
        let mut shuffled_counts = [0usize; N_ST];
        for (k, &s) in county_keys.iter().zip(labels.iter()) {
            shuffled_assign.insert(*k, s);
            shuffled_counts[s] += 1;
        }
        let mut sh_events: Vec<Event> = Vec::with_capacity(paired.len());
        for &k in &county_keys {
            if let Some(evs) = event_by_county.get(&k) {
                for e in evs {
                    sh_events.push(**e);
                }
            }
        }
        let sh_omaps = outage_maps(&sh_events, &shuffled_assign, &shuffled_counts);
        let sh_counts: Vec<Vec<f64>> = sh_omaps
            .iter()
            .map(|m| count_vec(m, hours.base, hours.len))
            .collect();
        if let Some((r, _, _)) = best_panel(&hours, &sh_counts) {
            null.push(r.abs());
        }
    }
    null.sort_by(|a, b| a.total_cmp(b));
    let Some(q95) = quantile(&null, 0.95) else {
        eprintln!("station-shuffle null stays empty");
        std::process::exit(1);
    };
    let exceed = null.iter().filter(|&&v| v >= obs_r.abs()).count();
    let p = (exceed as f64 + 1.0) / (null.len() as f64 + 1.0);
    println!(
        "station-shuffle null: 95 % = {q95:.5}, p = {p:.4}, draws = {} (labels permuted among paired counties)",
        null.len()
    );

    let (nb, no) = national_series(&bmaps, &events, counties_total.len(), lo, hi);
    let mut nat_best: Option<(f64, i64, usize)> = None;
    for lag in LAGS {
        if let Some((r, n)) = national_r(&nb, &no, lo, hi, lag) {
            if nat_best.map_or(true, |(br, _, _)| r.abs() > br.abs()) {
                nat_best = Some((r, lag, n));
            }
        }
    }
    if let Some((nr, nlag, nn)) = nat_best {
        println!(
            "national reference (6-station mean dB/dt, all counties): max |r| = {:.5} at lag {:+} h, n = {}",
            nr.abs(),
            nlag,
            nn
        );
    }

    println!();
    println!("## Quiet and active hours (pooled over paired counties)");
    let all_b: Vec<f64> = bmaps
        .iter()
        .flat_map(|m| {
            m.iter()
                .filter(|(h, _)| **h >= lo && **h <= hi)
                .map(|(_, v)| *v)
        })
        .collect();
    let Some(med) = median(&mut all_b.clone()) else {
        eprintln!("dB/dt median stays empty");
        std::process::exit(1);
    };
    let mut sorted = all_b.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let (Some(q25), Some(q75)) = (quantile(&sorted, 0.25), quantile(&sorted, 0.75)) else {
        eprintln!("dB/dt quartiles stay empty");
        std::process::exit(1);
    };
    let mut quiet_rate = 0.0;
    let mut quiet_hours = 0usize;
    let mut active_rate = 0.0;
    let mut active_hours = 0usize;
    for s in 0..N_ST {
        for (i, &q) in hours.hour[s].iter().enumerate() {
            let oi = q - obs_lag - hours.base;
            if oi < 0 || oi as usize >= hours.len {
                continue;
            }
            let rate = counts[s][oi as usize];
            let b = hours.b[s][i];
            if b <= q25 {
                quiet_rate += rate;
                quiet_hours += 1;
            }
            if b >= q75 {
                active_rate += rate;
                active_hours += 1;
            }
        }
    }
    if quiet_hours == 0 || active_hours == 0 {
        eprintln!("quiet or active window stays empty");
        std::process::exit(1);
    }
    let qr = quiet_rate / quiet_hours as f64;
    let ar = active_rate / active_hours as f64;
    println!("median |dB/dt| = {med:.3} nT/min, q25 = {q25:.3}, q75 = {q75:.3}");
    match ratio(ar, qr) {
        Some(v) => println!(
            "quiet hours: {quiet_hours}, outage rate {qr:.6} per 1000 county-hours; active hours: {active_hours}, rate {ar:.6}; ratio active/quiet = {v:.3}"
        ),
        None => println!(
            "quiet hours: {quiet_hours}, outage rate {qr:.6} per 1000 county-hours; active hours: {active_hours}, rate {ar:.6}; ratio active/quiet absent"
        ),
    }

    println!();
    println!("## Transfer entropy, binned {TE_BINS} bins, dB/dt -> outage rate");
    println!();
    println!("| lead h | TE | samples |");
    println!("|---:|---:|---:|");
    for &lag in &TE_LAGS {
        if let Some((te, weight)) = te_for_segments(&segs, &counts, hours.base, hours.len, lag) {
            println!("| {lag} | {te:.6} | {weight} |");
        }
    }
    let te_obs = te_for(&segs, &counts, hours.base, hours.len, &TE_LAGS);
    let mut te_null = Vec::with_capacity(draws);
    let mut rng2 = Rng(SEED ^ 0xA5A5_5A5A_1234_5678);
    for _ in 0..draws {
        for i in (1..labels.len()).rev() {
            let j = rng2.below(i + 1);
            labels.swap(i, j);
        }
        let mut shuffled_assign: HashMap<(u64, u64), usize> =
            HashMap::with_capacity(county_keys.len());
        let mut shuffled_counts = [0usize; N_ST];
        for (k, &s) in county_keys.iter().zip(labels.iter()) {
            shuffled_assign.insert(*k, s);
            shuffled_counts[s] += 1;
        }
        let mut sh_events: Vec<Event> = Vec::with_capacity(paired.len());
        for &k in &county_keys {
            if let Some(evs) = event_by_county.get(&k) {
                for e in evs {
                    sh_events.push(**e);
                }
            }
        }
        let sh_omaps = outage_maps(&sh_events, &shuffled_assign, &shuffled_counts);
        let sh_counts: Vec<Vec<f64>> = sh_omaps
            .iter()
            .map(|m| count_vec(m, hours.base, hours.len))
            .collect();
        if let Some((te, _, _)) = te_for(&segs, &sh_counts, hours.base, hours.len, &TE_LAGS) {
            te_null.push(te);
        }
    }
    te_null.sort_by(|a, b| a.total_cmp(b));
    if let Some((te, telag, tew)) = te_obs {
        if let Some(tq95) = quantile(&te_null, 0.95) {
            let tex = te_null.iter().filter(|&&v| v >= te).count();
            let tep = (tex as f64 + 1.0) / (te_null.len() as f64 + 1.0);
            println!();
            println!(
                "observed max TE = {te:.6} at lead {telag} h, samples = {tew}; station-shuffle null 95 % = {tq95:.6}, p = {tep:.4}, draws = {}",
                te_null.len()
            );
        }
    }

    println!();
    let carries = obs_r.abs() > q95;
    let more_than_national = nat_best.map_or(false, |(nr, _, _)| obs_r.abs() > nr.abs());
    println!(
        "verdict: spatial pairing {}; versus national reference {}",
        if carries {
            "clears the station-shuffle null"
        } else {
            "sits inside the station-shuffle null"
        },
        if more_than_national {
            "higher max |r|"
        } else {
            "not higher max |r|"
        }
    );
}
