use omegaflow::archivar::bpa_gic;
use omegaflow::archivar::geo::{self, MAGIC_GIC, MAGIC_GOEV};
use omegaflow::archivar::intermagnet;
use std::collections::{BTreeMap, BTreeSet, HashMap};

const J2000_UNIX_OFFSET: f64 = 946_728_000.0;
const DAY_S: f64 = 86_400.0;
const MAX_PAIR_KM: f64 = 300.0;
const SURROGATES: usize = 500;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAGS: [i64; 3] = [0, 1, 2];

const GOEV_DURATION: u32 = 1;
const GOEV_MAX_CUSTOMERS: u32 = 2;
const GOEV_CAUSE: u32 = 3;

struct Station {
    name: &'static str,
    lat: f64,
    lon: f64,
}

const STATIONS: [Station; 11] = [
    Station {
        name: "eugene_or",
        lat: 44.0521,
        lon: -123.0868,
    },
    Station {
        name: "spokane_wa",
        lat: 47.6588,
        lon: -117.4260,
    },
    Station {
        name: "the_dalles_or",
        lat: 45.5946,
        lon: -121.1787,
    },
    Station {
        name: "lewiston_id",
        lat: 46.4166,
        lon: -117.0177,
    },
    Station {
        name: "kalispell_mt",
        lat: 48.1916,
        lon: -114.3162,
    },
    Station {
        name: "monroe_wa",
        lat: 47.8554,
        lon: -121.9707,
    },
    Station {
        name: "wilsonville_or",
        lat: 45.2998,
        lon: -122.7737,
    },
    Station {
        name: "vancouver_wa",
        lat: 45.6387,
        lon: -122.6615,
    },
    Station {
        name: "ellensburg_wa",
        lat: 46.9965,
        lon: -120.5478,
    },
    Station {
        name: "bothell_wa",
        lat: 47.7623,
        lon: -122.2054,
    },
    Station {
        name: "moses_lake_wa",
        lat: 47.1301,
        lon: -119.2781,
    },
];

#[derive(Clone, Copy, Default)]
struct Agg {
    events: f64,
    customers: f64,
    duration: f64,
}

impl Agg {
    fn add(&mut self, e: &Event) {
        self.events += 1.0;
        if let Some(v) = e.customers {
            self.customers += v;
        }
        if let Some(v) = e.duration {
            self.duration += v;
        }
    }
}

#[derive(Clone, Copy)]
struct Event {
    day: i64,
    lat: f64,
    lon: f64,
    customers: Option<f64>,
    duration: Option<f64>,
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
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn iso_day(day: i64) -> String {
    let (y, m, d) = civil_from_days(day);
    format!("{y:04}-{m:02}-{d:02}")
}

fn unix_day(t_unix: f64) -> i64 {
    (t_unix / DAY_S).floor() as i64
}

fn tdb_day(t_tdb: f64) -> i64 {
    ((t_tdb + J2000_UNIX_OFFSET) / DAY_S).floor() as i64
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

fn bump_max(map: &mut BTreeMap<i64, f64>, day: i64, value: f64) {
    match map.entry(day) {
        std::collections::btree_map::Entry::Vacant(slot) => {
            slot.insert(value);
        }
        std::collections::btree_map::Entry::Occupied(mut slot) => {
            if value > *slot.get() {
                slot.insert(value);
            }
        }
    }
}

fn pearson(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < 3 || x.len() != y.len() {
        return None;
    }
    let nf = n as f64;
    let mx = x.iter().sum::<f64>() / nf;
    let my = y.iter().sum::<f64>() / nf;
    let mut sxy = f64::default();
    let mut sxx = f64::default();
    let mut syy = f64::default();
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

struct OutageStats {
    records: usize,
    duration_recs: usize,
    customers_recs: usize,
    cause_recs: usize,
    unique_keys: usize,
}

fn load_outages(path: &str) -> Option<(Vec<Event>, OutageStats)> {
    let bytes = std::fs::read(path).ok()?;
    let recs = geo::parse_bin(MAGIC_GOEV, &bytes)?;
    let mut stats = OutageStats {
        records: recs.len(),
        duration_recs: 0,
        customers_recs: 0,
        cause_recs: 0,
        unique_keys: 0,
    };
    let mut groups: HashMap<(u64, u64, u64), (f64, f64, f64, Option<f64>, Option<f64>)> =
        HashMap::new();
    for r in &recs {
        let key = (r.t.to_bits(), r.lat.to_bits(), r.lon.to_bits());
        let slot = groups.entry(key).or_insert((r.t, r.lat, r.lon, None, None));
        match r.comp {
            GOEV_DURATION => {
                stats.duration_recs += 1;
                slot.3 = Some(r.val);
            }
            GOEV_MAX_CUSTOMERS => {
                stats.customers_recs += 1;
                slot.4 = Some(r.val);
            }
            GOEV_CAUSE => stats.cause_recs += 1,
            _ => {}
        }
    }
    stats.unique_keys = groups.len();
    let mut out: Vec<Event> = groups
        .into_values()
        .map(|(t, lat, lon, customers, duration)| Event {
            day: tdb_day(t),
            lat,
            lon,
            customers,
            duration,
        })
        .collect();
    out.sort_by_key(|e| e.day);
    Some((out, stats))
}

fn bpa_daily(bytes: &[u8]) -> Option<Vec<BTreeMap<i64, f64>>> {
    let recs = bpa_gic::parse_bin(bytes)?;
    let mut out: Vec<BTreeMap<i64, f64>> = vec![BTreeMap::new(); bpa_gic::CHANNELS];
    for r in &recs {
        let day = unix_day(r.t_unix);
        for (i, v) in r.values().iter().enumerate() {
            let Some(v) = v else { continue };
            if !v.is_finite() {
                continue;
            }
            bump_max(&mut out[i], day, v.abs());
        }
    }
    Some(out)
}

fn load_geo_daily(path: &str, magic: [u8; 4]) -> Option<BTreeMap<i64, f64>> {
    let bytes = std::fs::read(path).ok()?;
    let recs = geo::parse_bin(magic, &bytes)?;
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for r in &recs {
        if !r.val.is_finite() {
            continue;
        }
        let day = tdb_day(r.t);
        bump_max(&mut out, day, r.val.abs());
    }
    Some(out)
}

fn load_dbdt_im(path: &str) -> Option<BTreeMap<i64, f64>> {
    let bytes = std::fs::read(path).ok()?;
    let recs = intermagnet::parse_bin(&bytes)?;
    let mut out: BTreeMap<i64, f64> = BTreeMap::new();
    for (t, v, _) in &recs {
        if !v.is_finite() {
            continue;
        }
        let day = unix_day(*t);
        bump_max(&mut out, day, v.abs());
    }
    Some(out)
}

#[derive(Clone, Copy)]
enum Metric {
    Events,
    LnCustomers,
    LnDuration,
}

impl Metric {
    fn name(self) -> &'static str {
        match self {
            Metric::Events => "events/day",
            Metric::LnCustomers => "ln(1+customers)",
            Metric::LnDuration => "ln(1+duration_h)",
        }
    }
    fn of(self, a: Agg) -> f64 {
        match self {
            Metric::Events => a.events,
            Metric::LnCustomers => (1.0 + a.customers).ln(),
            Metric::LnDuration => (1.0 + a.duration).ln(),
        }
    }
}

const METRICS: [Metric; 3] = [Metric::Events, Metric::LnCustomers, Metric::LnDuration];

struct Cell {
    metric: Metric,
    lag: i64,
    r: f64,
    n: usize,
}

fn cells(pred: &BTreeMap<i64, f64>, out: &BTreeMap<i64, Agg>) -> Vec<Cell> {
    let mut v = Vec::new();
    let days: Vec<i64> = pred.keys().copied().collect();
    for metric in METRICS {
        for lag in LAGS {
            let mut xs = Vec::new();
            let mut ys = Vec::new();
            for &d in &days {
                let Some(&x) = pred.get(&d) else { continue };
                let y = match out.get(&(d + lag)) {
                    Some(a) => *a,
                    None => Agg::default(),
                };
                xs.push(x);
                ys.push(metric.of(y));
            }
            if let Some(r) = pearson(&xs, &ys) {
                v.push(Cell {
                    metric,
                    lag,
                    r,
                    n: xs.len(),
                });
            }
        }
    }
    v
}

fn best_cell(cs: &[Cell]) -> Option<&Cell> {
    cs.iter().max_by(|a, b| a.r.abs().total_cmp(&b.r.abs()))
}

fn block_shift_null(
    pred: &BTreeMap<i64, f64>,
    out: &BTreeMap<i64, Agg>,
    observed: f64,
) -> Option<(f64, f64, usize)> {
    let days: Vec<i64> = pred.keys().copied().collect();
    let mut year_map: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (i, &d) in days.iter().enumerate() {
        let (y, _, _) = civil_from_days(d);
        year_map.entry(y).or_default().push(i);
    }
    let year_idx: Vec<Vec<usize>> = year_map.into_values().filter(|idx| idx.len() > 1).collect();
    if year_idx.is_empty() {
        return None;
    }
    let vals: Vec<f64> = days.iter().map(|d| pred[d]).collect();
    let mut rng = Rng(SEED);
    let mut null = Vec::with_capacity(SURROGATES);
    for _ in 0..SURROGATES {
        let mut shifted = vals.clone();
        for idx in &year_idx {
            let shift = rng.below(idx.len());
            for (k, &i) in idx.iter().enumerate() {
                shifted[i] = vals[idx[(k + shift) % idx.len()]];
            }
        }
        let shifted_map: BTreeMap<i64, f64> =
            days.iter().copied().zip(shifted.iter().copied()).collect();
        let cs = cells(&shifted_map, out);
        let mx = cs.iter().map(|c| c.r.abs()).fold(f64::default(), f64::max);
        null.push(mx);
    }
    null.sort_by(|a, b| a.total_cmp(b));
    let q95 = null[((null.len() as f64) * 0.95) as usize];
    let exceed = null.iter().filter(|v| **v >= observed).count();
    let p = (exceed as f64 + 1.0) / (null.len() as f64 + 1.0);
    Some((q95, p, null.len()))
}

fn report_family(label: &str, pred: &BTreeMap<i64, f64>, out: &BTreeMap<i64, Agg>) {
    let cs = cells(pred, out);
    if cs.is_empty() {
        println!("{label}: no paired days (family stays void)");
        return;
    }
    println!("{label}: {} cells (3 metrics x lags 0-2 d)", cs.len());
    for c in &cs {
        println!(
            "  r = {:+.4}  lag {} d  {}  n = {}",
            c.r,
            c.lag,
            c.metric.name(),
            c.n
        );
    }
    let Some(best) = best_cell(&cs) else { return };
    let observed = best.r.abs();
    println!(
        "{label}: observed max |r| = {:.4} at lag {} d, {} (n = {})",
        observed,
        best.lag,
        best.metric.name(),
        best.n
    );
    if let Some((q95, p, reps)) = block_shift_null(pred, out, observed) {
        println!(
            "{label}: block-shift null 95% = {q95:.4}, p = {p:.4} ({reps} surrogates, circular within each year)"
        );
        println!(
            "{label}: verdict = {}",
            if observed > q95 {
                "traegt (observed max clears the null)"
            } else {
                "traegt nicht (observed max sits inside the null)"
            }
        );
    } else {
        println!("{label}: null stays void (fewer than 2 days in every year block)");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let outage_path =
        arg_after(&args, "--outage").unwrap_or("/tmp/opencode/outage_kante/grid_outage_events.bin");
    let bpa_path = arg_after(&args, "--bpa").unwrap_or("/tmp/opencode/outage_kante/bpa_gic.bin");
    let abk_path =
        arg_after(&args, "--abk").unwrap_or("/tmp/opencode/outage_kante/abk_dbdt_1h.bin");
    let sod_path =
        arg_after(&args, "--sod").unwrap_or("/tmp/opencode/outage_kante/sod_dbdt_1h.bin");
    let nur_path =
        arg_after(&args, "--nur").unwrap_or("/tmp/opencode/outage_kante/fmi_image_mag_nur.bin");
    let fmi_path = arg_after(&args, "--fmi").unwrap_or("/tmp/opencode/outage_kante/fmi_gic.bin");

    let Some((events, stats)) = load_outages(outage_path) else {
        eprintln!("{outage_path}: grid_outage_event bin reads void — the edge stays unmeasured");
        std::process::exit(1);
    };
    if events.is_empty() {
        eprintln!("{outage_path}: 0 events — the edge stays unmeasured");
        std::process::exit(1);
    }
    println!(
        "outage records: {} ({} duration_h, {} max_customers, {} cause_class) -> {} unique (start,county) keys",
        stats.records,
        stats.duration_recs,
        stats.customers_recs,
        stats.cause_recs,
        stats.unique_keys
    );
    let ev_min = events.iter().map(|e| e.day).min().unwrap();
    let ev_max = events.iter().map(|e| e.day).max().unwrap();
    let mut counties: BTreeSet<(u64, u64)> = BTreeSet::new();
    for e in &events {
        counties.insert((e.lat.to_bits(), e.lon.to_bits()));
    }
    println!(
        "outage: {} county-events, {} counties, {} .. {}",
        events.len(),
        counties.len(),
        iso_day(ev_min),
        iso_day(ev_max)
    );

    let mut county_station: HashMap<(u64, u64), (usize, f64)> = HashMap::new();
    for &key in &counties {
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
                county_station.insert(key, (i, d));
            }
        }
    }
    let paired_events: Vec<&Event> = events
        .iter()
        .filter(|e| county_station.contains_key(&(e.lat.to_bits(), e.lon.to_bits())))
        .collect();
    let paired_counties = county_station.len();
    println!(
        "pair: {} of {} counties within {:.0} km of a BPA station, {} of {} events",
        paired_counties,
        counties.len(),
        MAX_PAIR_KM,
        paired_events.len(),
        events.len()
    );
    let mut per_station_counties = [0usize; 11];
    let mut per_station_events = [0usize; 11];
    let mut max_dist = f64::default();
    for &(i, d) in county_station.values() {
        per_station_counties[i] += 1;
        max_dist = max_dist.max(d);
    }
    for e in &paired_events {
        let (i, _) = county_station[&(e.lat.to_bits(), e.lon.to_bits())];
        per_station_events[i] += 1;
    }
    println!("pair max distance: {max_dist:.1} km");

    let mut national: BTreeMap<i64, Agg> = BTreeMap::new();
    for e in &events {
        national.entry(e.day).or_default().add(e);
    }
    let mut regional: BTreeMap<i64, Agg> = BTreeMap::new();
    let mut per_station_out: Vec<BTreeMap<i64, Agg>> = vec![BTreeMap::new(); 11];
    for e in &paired_events {
        let (i, _) = county_station[&(e.lat.to_bits(), e.lon.to_bits())];
        regional.entry(e.day).or_default().add(e);
        per_station_out[i].entry(e.day).or_default().add(e);
    }

    let Some(bpa_bytes) = std::fs::read(bpa_path).ok() else {
        eprintln!("{bpa_path}: read void — the BPA edge stays unmeasured");
        std::process::exit(1);
    };
    let Some(bpa) = bpa_daily(&bpa_bytes) else {
        eprintln!("{bpa_path}: BPA bin reads void — the edge stays unmeasured");
        std::process::exit(1);
    };
    let mut bpa_days = BTreeSet::new();
    for s in &bpa {
        bpa_days.extend(s.keys().copied());
    }
    let bpa_min = *bpa_days.iter().next().unwrap_or(&ev_min);
    let bpa_max = *bpa_days.iter().next_back().unwrap_or(&ev_max);
    println!(
        "bpa: {} stations, {} distinct days, {} .. {}",
        bpa.len(),
        bpa_days.len(),
        iso_day(bpa_min),
        iso_day(bpa_max)
    );

    let stations_used: Vec<usize> = (0..STATIONS.len())
        .filter(|&i| per_station_counties[i] > 0)
        .collect();
    let mut regional_pred: BTreeMap<i64, f64> = BTreeMap::new();
    for &i in &stations_used {
        for (&d, &v) in &bpa[i] {
            bump_max(&mut regional_pred, d, v);
        }
    }
    let common_min = ev_min.max(bpa_min);
    let common_max = ev_max.min(bpa_max);
    regional_pred.retain(|d, _| *d >= common_min && *d <= common_max);

    println!("station pairing table (<= {:.0} km):", MAX_PAIR_KM);
    println!("  station | counties | events | best |r| | lag | metric");
    for (i, s) in STATIONS.iter().enumerate() {
        if per_station_counties[i] == 0 {
            println!(
                "  {:<14} | {:>8} | {:>6} | (no paired county)",
                s.name, per_station_counties[i], per_station_events[i]
            );
            continue;
        }
        let pred: BTreeMap<i64, f64> = bpa[i]
            .iter()
            .filter(|(d, _)| **d >= common_min && **d <= common_max)
            .map(|(&d, &v)| (d, v))
            .collect();
        let cs = cells(&pred, &per_station_out[i]);
        match best_cell(&cs) {
            Some(b) => println!(
                "  {:<14} | {:>8} | {:>6} | {:>8.4} | {:>3} | {}",
                s.name,
                per_station_counties[i],
                per_station_events[i],
                b.r.abs(),
                b.lag,
                b.metric.name()
            ),
            None => println!(
                "  {:<14} | {:>8} | {:>6} | (no paired day)",
                s.name, per_station_counties[i], per_station_events[i]
            ),
        }
    }

    let mut common_days = 0usize;
    for &d in regional_pred.keys() {
        if national.contains_key(&d) || (d >= ev_min && d <= ev_max) {
            common_days += 1;
        }
    }
    let paired_days_in_window = regional_pred
        .keys()
        .filter(|d| **d >= common_min && **d <= common_max)
        .count();
    println!(
        "window BPA/outage: {} .. {}, {} days with a regional GIC sample",
        iso_day(common_min),
        iso_day(common_max),
        paired_days_in_window
    );
    if paired_days_in_window == 0 {
        println!(
            "BPA riss: the released bpa_gic.bin is a rolling {} - {} snapshot; it carries no day of the outage window, so the county pairing cannot be measured in time",
            iso_day(bpa_min),
            iso_day(bpa_max)
        );
    }
    let _ = common_days;

    println!();
    report_family("BPA paired -> regional outage", &regional_pred, &regional);
    println!();

    let proxies: [(&str, Option<BTreeMap<i64, f64>>); 4] = [
        ("ABK dB/dt", load_dbdt_im(abk_path)),
        ("SOD dB/dt", load_dbdt_im(sod_path)),
        ("NUR dB/dt", load_geo_daily(nur_path, geo::MAGIC_IMAGE)),
        ("FMI GIC", load_geo_daily(fmi_path, MAGIC_GIC)),
    ];
    for (label, series) in proxies {
        let Some(series) = series else {
            println!("{label}: read void — soil sensitivity stays void");
            continue;
        };
        if series.is_empty() {
            println!("{label}: 0 days — soil sensitivity stays void");
            continue;
        }
        let smin = *series.keys().next().unwrap();
        let smax = *series.keys().next_back().unwrap();
        let overlap_min = ev_min.max(smin);
        let overlap_max = ev_max.min(smax);
        let pred: BTreeMap<i64, f64> = series
            .iter()
            .filter(|(d, _)| **d >= overlap_min && **d <= overlap_max)
            .map(|(&d, &v)| (d, v))
            .collect();
        println!(
            "{label}: {} days, {} .. {}, overlap {} .. {} ({} days); non-spatial proxy (no county pairing)",
            series.len(),
            iso_day(smin),
            iso_day(smax),
            iso_day(overlap_min),
            iso_day(overlap_max),
            pred.len()
        );
        report_family(&format!("{label} -> national outage"), &pred, &national);
        println!();
    }
}
