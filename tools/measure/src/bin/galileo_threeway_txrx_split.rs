use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, Read};

use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::atdf::{
    extract, field_of, full_year, strip_markers, tracking_record, IDFORM, LOGICAL_RECORD,
    TKFORM, Tracking, XPFORM,
};
use omegaflow::lsk::{days_from_civil, LeapSeconds};

const BASE: &str = "https://pds-ppi.igpp.ucla.edu/annex/";

const TARGETS: &[(&str, &str)] = &[
    ("GO-SUN-RSS-1-TDF-V1.0", "5327328A.TDF"),
    ("GO-SUN-RSS-1-TDF-V1.0", "5337339A.TDF"),
    ("GO-SUN-RSS-1-TDF-V1.0", "5340341A.TDF"),
    ("GO-JG-RSS-1-TDF-V1.0", "6177179A.TDF"),
];

const IDENTITY_ITEMS: &[(u32, &str)] = &[
    (9, "NET_ID"),
    (10, "STATION"),
    (11, "DOWNLINK_BAND"),
    (26, "XMTR_ON0"),
    (28, "SOURCE_DESIG"),
    (64, "UPLINK_BAND"),
    (69, "DOPPLER_CHANNEL"),
    (70, "FREQ_STD"),
    (71, "DOPPLER_RCVR_REF"),
    (92, "RCVR_NUMBER"),
    (94, "AMP_NUMBER"),
    (95, "AMP_TYPE"),
    (96, "XMTR_POWER_IND"),
];

fn tdb_of(tr: &Tracking, lsk: &LeapSeconds) -> Option<f64> {
    if tr.day <= 0 || tr.day > 366 {
        return None;
    }
    let year = full_year(tr.year);
    let days = days_from_civil(year, 1, 1)? + tr.day - 1;
    let unix = days as f64 * 86400.0
        + tr.hour as f64 * 3600.0
        + tr.minute as f64 * 60.0
        + tr.second as f64;
    lsk.unix_to_tdb(unix)
}

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / 86400.0;
    (jd - 2440587.5).round() as i64
}

fn civil(day: i64) -> String {
    match omegaflow::spectral::civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn fmt_utc(tdb: f64) -> String {
    let unix = tdb + 10957.5 * 86400.0;
    let rem = unix.rem_euclid(86400.0);
    let h = (rem / 3600.0) as i64;
    let m = ((rem % 3600.0) / 60.0) as i64;
    format!("{h:02}:{m:02}")
}

fn is_threeway(mode: i64) -> bool {
    mode == 3 || mode == 4
}

fn in_trio(station: i64) -> bool {
    station == 14 || station == 43 || station == 63
}

struct Grp {
    n: usize,
    floor: usize,
    xmtr_on: usize,
    xmtr_off: usize,
    vals: BTreeMap<u32, BTreeMap<i64, usize>>,
    pow_nz: usize,
    pow_lo: Option<i64>,
    pow_hi: Option<i64>,
    freq_nz: usize,
    freq_lo: Option<i64>,
    freq_hi: Option<i64>,
}

impl Grp {
    fn new() -> Grp {
        Grp {
            n: 0,
            floor: 0,
            xmtr_on: 0,
            xmtr_off: 0,
            vals: BTreeMap::new(),
            pow_nz: 0,
            pow_lo: None,
            pow_hi: None,
            freq_nz: 0,
            freq_lo: None,
            freq_hi: None,
        }
    }
}

fn fmt_vals(map: &BTreeMap<i64, usize>) -> String {
    if map.is_empty() {
        return "empty".to_string();
    }
    let mut parts: Vec<String> = Vec::new();
    for (v, c) in map {
        parts.push(format!("{v}:{c}"));
    }
    parts.join(" ")
}

fn opt_range(lo: Option<i64>, hi: Option<i64>) -> String {
    match (lo, hi) {
        (Some(a), Some(b)) => format!("{a}..{b}"),
        _ => "0".to_string(),
    }
}

fn record_values(rec: &[u8]) -> BTreeMap<u32, i64> {
    let mut out = BTreeMap::new();
    for (item, _) in IDENTITY_ITEMS {
        if let Some(fld) = field_of(TKFORM, *item) {
            out.insert(*item, extract(rec, fld));
        }
    }
    out
}

struct CellStat {
    n: f64,
    sum: f64,
    sumsq: f64,
}

impl CellStat {
    fn new() -> CellStat {
        CellStat {
            n: 0.0,
            sum: 0.0,
            sumsq: 0.0,
        }
    }
    fn push(&mut self, v: f64) {
        self.n += 1.0;
        self.sum += v;
        self.sumsq += v * v;
    }
    fn rms(&self) -> Option<f64> {
        if self.n <= 0.0 {
            return None;
        }
        let mean = self.sum / self.n;
        let var = (self.sumsq / self.n - mean * mean).max(0.0);
        Some(var.sqrt())
    }
}

fn main() {
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut target_days: BTreeSet<i64> = BTreeSet::new();
    for (volume, name) in TARGETS {
        let url = format!("{BASE}{volume}/TDF/{name}");
        let cache_path = format!("tmp/galileo_tdf_cache_{name}");
        let bytes = match std::fs::read(&cache_path) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("fetching {url}");
                match fetch_raw_bytes(&url, 3600) {
                    Some(b) => {
                        let _ = std::fs::write(&cache_path, &b);
                        b
                    }
                    None => {
                        eprintln!("{name}: fetch void ({url})");
                        continue;
                    }
                }
            }
        };
        let Some(stripped) = strip_markers(&bytes) else {
            eprintln!("{name}: marker strip void");
            continue;
        };
        let nlog = stripped.len() / LOGICAL_RECORD;
        if nlog < 3 {
            eprintln!("{name}: {nlog} logical records — too short");
            continue;
        }
        let rec0 = &stripped[0..LOGICAL_RECORD];
        let rec1 = &stripped[LOGICAL_RECORD..2 * LOGICAL_RECORD];
        let fyear = full_year(extract(rec0, field_of(IDFORM, 3).unwrap()));
        let fday = extract(rec0, field_of(IDFORM, 4).unwrap());
        let fhour = extract(rec0, field_of(IDFORM, 5).unwrap());
        let sc = extract(rec1, field_of(XPFORM, 9).unwrap());
        let xpon_hp = extract(rec1, field_of(XPFORM, 17).unwrap());
        let xpon_lp = extract(rec1, field_of(XPFORM, 18).unwrap());
        let xpon = xpon_hp as f64 * 1e4 + xpon_lp as f64 / 1e3;

        let mut groups: BTreeMap<(i64, i64, i64), Grp> = BTreeMap::new();
        let mut station_mode: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        let mut runs: BTreeMap<(i64, i64), (usize, usize, f64)> = BTreeMap::new();
        let mut n_dop = 0usize;
        let mut n_three = 0usize;
        let mut seq: Vec<(f64, i64, i64, i64)> = Vec::new();
        for i in 2..nlog {
            let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
            let tr = tracking_record(rec);
            if tr.day == 0 || !(tr.data_type == 1 || tr.data_type == 2) {
                continue;
            }
            let sampler = tr.sampler_time as f64 / 100.0;
            if sampler <= 0.0 {
                continue;
            }
            let Some(tdb) = tdb_of(&tr, &lsk) else {
                continue;
            };
            let resid = tr.doppler_resid as f64 / 1000.0;
            if !resid.is_finite() {
                continue;
            }
            n_dop += 1;
            let day = unix_day(tdb);
            let station = tr.station;
            let mode = tr.ground_mode;
            *station_mode.entry((station, mode)).or_insert(0) += 1;
            if is_threeway(mode) {
                n_three += 1;
                target_days.insert(day);
                let r = runs.entry((day, station)).or_insert((0, 0, f64::NEG_INFINITY));
                r.0 += 1;
                if tdb - r.2 > 600.0 {
                    r.1 += 1;
                }
                r.2 = tdb;
            }
            let g = groups.entry((day, station, mode)).or_insert_with(Grp::new);
            g.n += 1;
            if tr.signal_strength == -2560 {
                g.floor += 1;
            }
            let fv = record_values(rec);
            for (item, v) in &fv {
                if *item == 10 {
                    continue;
                }
                let m = g.vals.entry(*item).or_insert_with(BTreeMap::new);
                *m.entry(*v).or_insert(0) += 1;
            }
            let on = fv[&26];
            if on == 0 {
                g.xmtr_on += 1;
            } else {
                g.xmtr_off += 1;
            }
            if (mode == 2 || is_threeway(mode)) && in_trio(station) {
                seq.push((tdb, station, mode, on));
            }
            let pow = fv[&98];
            if pow != 0 {
                g.pow_nz += 1;
                g.pow_lo = Some(g.pow_lo.map_or(pow, |p| p.min(pow)));
                g.pow_hi = Some(g.pow_hi.map_or(pow, |p| p.max(pow)));
            }
            let freq = fv[&116];
            if freq != 0 {
                g.freq_nz += 1;
                g.freq_lo = Some(g.freq_lo.map_or(freq, |p| p.min(freq)));
                g.freq_hi = Some(g.freq_hi.map_or(freq, |p| p.max(freq)));
            }
        }

        println!("FILE {name} url {url} logical {nlog}");
        println!(
            "  header: year {fyear} day {fday} hour {fhour} sc {sc} xponder {xpon:.3e} Hz"
        );
        println!("  doppler records {n_dop}, three-way records {n_three}");
        let mut sm: Vec<_> = station_mode.into_iter().collect();
        sm.sort();
        println!("  station-mode census:");
        for ((station, mode), c) in sm {
            println!("    station {station} mode {mode}: {c}");
        }

        let mut tw_pairs: BTreeSet<(i64, i64, i64)> = BTreeSet::new();
        for ((day, station, mode), g) in groups.iter() {
            if !is_threeway(*mode) {
                continue;
            }
            println!(
                "  M3GROUP {name} date {} day {} station {} mode {} n {} floor {} xmtr_on0 {} xmtr_off1 {}",
                civil(*day),
                *day,
                *station,
                mode,
                g.n,
                g.floor,
                g.xmtr_on,
                g.xmtr_off
            );
            let mut station_like: Vec<String> = Vec::new();
            for (item, label) in IDENTITY_ITEMS {
                if *item == 10 {
                    continue;
                }
                if let Some(vmap) = g.vals.get(item) {
                    println!("    {label} item {item}: {}", fmt_vals(vmap));
                    for v in vmap.keys() {
                        if (11..=99).contains(v) && *v != *station {
                            station_like.push(format!("{label}={v}"));
                        }
                    }
                }
            }
            println!(
                "    xmtr_power nonzero {}/{} range {}; xmtr_freq nonzero {}/{} range {}",
                g.pow_nz,
                g.n,
                opt_range(g.pow_lo, g.pow_hi),
                g.freq_nz,
                g.n,
                opt_range(g.freq_lo, g.freq_hi),
            );
            if station_like.is_empty() {
                println!("    station-like second value: none");
            } else {
                println!("    station-like second value: {}", station_like.join(" "));
            }
            if groups.contains_key(&(*day, *station, 2)) {
                tw_pairs.insert((*day, *station, *mode));
            }
        }

        for (day, station, mode3) in tw_pairs {
            let g2 = groups.get(&(day, station, 2)).unwrap();
            let g3 = groups.get(&(day, station, mode3)).unwrap();
            println!(
                "  TRANSITION {name} date {} day {} station {} mode2_n {} threeway_n {}",
                civil(day),
                day,
                station,
                g2.n,
                g3.n
            );
            for (item, label) in IDENTITY_ITEMS {
                if *item == 10 {
                    continue;
                }
                let v2 = g2.vals.get(item);
                let v3 = g3.vals.get(item);
                let set2 = v2.map(|m| m.keys().copied().collect::<Vec<i64>>());
                let set3 = v3.map(|m| m.keys().copied().collect::<Vec<i64>>());
                let same = match (&set2, &set3) {
                    (Some(a), Some(b)) => a == b,
                    (None, None) => true,
                    _ => false,
                };
                if !same {
                    let s2 = match v2 {
                        Some(m) => fmt_vals(m),
                        None => "empty".to_string(),
                    };
                    let s3 = match v3 {
                        Some(m) => fmt_vals(m),
                        None => "empty".to_string(),
                    };
                    println!("    diff {label}: mode2 [{s2}] vs threeway [{s3}]");
                }
            }
        }

        let mut rk: Vec<_> = runs.into_iter().collect();
        rk.sort_by(|a, b| a.0.cmp(&b.0));
        for ((day, station), (n, nruns, _)) in rk {
            println!(
                "  M3RUN {name} date {} day {} station {} samples {} runs {}",
                civil(day),
                day,
                station,
                n,
                nruns
            );
        }

        let mut blocks: Vec<(i64, i64, i64, f64, f64, usize, usize)> = Vec::new();
        for station in [14i64, 43, 63] {
            let mut st_rec: Vec<&(f64, i64, i64, i64)> =
                seq.iter().filter(|r| r.1 == station).collect();
            st_rec.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut bi: Option<(i64, i64, i64, f64, f64, usize, usize)> = None;
            for r in st_rec {
                let day = unix_day(r.0);
                let extend = match &bi {
                    Some(b) => b.0 == r.2 && b.1 == day && r.0 - b.4 <= 600.0,
                    None => false,
                };
                if extend {
                    let b = bi.as_mut().unwrap();
                    b.4 = r.0;
                    b.5 += 1;
                    if r.3 != 0 {
                        b.6 += 1;
                    }
                } else {
                    if let Some(b) = bi.take() {
                        blocks.push(b);
                    }
                    bi = Some((
                        r.2,
                        day,
                        station,
                        r.0,
                        r.0,
                        1,
                        if r.3 != 0 { 1 } else { 0 },
                    ));
                }
            }
            if let Some(b) = bi.take() {
                blocks.push(b);
            }
        }
        let mut bl: Vec<&(i64, i64, i64, f64, f64, usize, usize)> = blocks.iter().collect();
        bl.sort_by(|a, b| {
            a.1.cmp(&b.1)
                .then(a.2.cmp(&b.2))
                .then(a.0.cmp(&b.0))
                .then_with(|| a.3.total_cmp(&b.3))
        });
        for b in &bl {
            println!(
                "  BLOCK {name} day {} date {} station {} mode {} n {} xmtr_off {} start {} end {}",
                b.1,
                civil(b.1),
                b.2,
                b.0,
                b.5,
                b.6,
                fmt_utc(b.3),
                fmt_utc(b.4)
            );
        }
        for b in bl.iter().filter(|b| is_threeway(b.0)) {
            let cands: Vec<&&(i64, i64, i64, f64, f64, usize, usize)> = bl
                .iter()
                .filter(|a| a.0 == 2 && a.1 == b.1 && a.2 != b.2 && a.3 <= b.4 && a.4 >= b.3)
                .collect();
            if cands.is_empty() {
                println!(
                    "  TXOVERLAP {name} day {} date {} threeway station {} — no two-way overlap at another station in this file",
                    b.1,
                    civil(b.1),
                    b.2
                );
            } else {
                for a in cands {
                    println!(
                        "  TXOVERLAP {name} day {} date {} threeway station {} overlaps two-way station {} (candidate uplink, inference)",
                        b.1,
                        civil(b.1),
                        b.2,
                        a.2
                    );
                }
            }
        }
    }

    println!("\nfloor cells over the fetched-day set (local resid asset):");
    floor_report(&target_days);
}

fn floor_report(target_days: &BTreeSet<i64>) {
    let path = "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin";
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("{path} absent (0 honored)");
            return;
        }
    };
    let mut reader = BufReader::new(file);
    let mut magic = [0u8; 4];
    if reader.read_exact(&mut magic).is_err() || &magic != b"GASR" {
        eprintln!("{path} magic void");
        return;
    }
    let mut count_buf = [0u8; 4];
    if reader.read_exact(&mut count_buf).is_err() {
        eprintln!("{path} count void");
        return;
    }
    let count = u32::from_le_bytes(count_buf) as usize;
    let mut target_cell: BTreeMap<(i64, i64, i64), CellStat> = BTreeMap::new();
    let mut era_m3: BTreeMap<(i64, i64, i64), CellStat> = BTreeMap::new();
    let mut rec = [0u8; 64];
    let mut parsed = 0usize;
    loop {
        match reader.read_exact(&mut rec) {
            Ok(()) => {}
            Err(_) => break,
        }
        parsed += 1;
        let mut r = [0.0f64; 8];
        for k in 0..8 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&rec[k * 8..k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        let resid = r[1];
        let station = r[2] as i64;
        let mode = r[3] as i64;
        let strength = r[7] as i64;
        if !resid.is_finite() || resid.abs() > 1000.0 || strength != -2560 {
            continue;
        }
        if !in_trio(station) {
            continue;
        }
        let day = unix_day(r[0]);
        if is_threeway(mode) {
            era_m3.entry((mode, station, day)).or_insert_with(CellStat::new).push(resid);
        }
        if target_days.contains(&day) {
            target_cell
                .entry((mode, station, day))
                .or_insert_with(CellStat::new)
                .push(resid);
        }
    }
    println!(
        "  resid records parsed {parsed} of header count {count} — target-day floor cells (mode station date day n rms class):"
    );
    let mut keys: Vec<_> = target_cell.into_iter().collect();
    keys.sort_by(|a, b| a.0.cmp(&b.0));
    for ((mode, station, day), s) in keys {
        let n = s.n as usize;
        match s.rms() {
            Some(rms) => println!(
                "    mode {mode} station {station} date {} day {} n {n} rms {rms:.4} Hz {}",
                civil(day),
                day,
                if rms >= 1.0 { "LOUD" } else { "quiet" }
            ),
            None => println!("    mode {mode} station {station} date {} day {} n {n} rms void", civil(day), day),
        }
    }

    let mut series: BTreeMap<(i64, i64), Vec<(i64, f64, usize)>> = BTreeMap::new();
    for ((mode, station, day), s) in era_m3.iter() {
        let n = s.n as usize;
        if n < 30 {
            continue;
        }
        if let Some(rms) = s.rms() {
            series
                .entry((*station, *mode))
                .or_insert_with(Vec::new)
                .push((*day, rms, n));
        }
    }
    println!(
        "  whole-era three-way floor cells (mode 3/4, trio stations), robust days n>=30 per (station, mode):"
    );
    for ((station, mode), rows) in series.iter_mut() {
        rows.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.total_cmp(&b.1)));
        let loud = rows.iter().filter(|(_, rms, _)| *rms >= 1.0).count();
        println!(
            "    station {station} mode {mode}: robust days {}, loud days {}",
            rows.len(),
            loud
        );
        for (day, rms, n) in rows.iter() {
            println!(
                "      day {} date {} n {n} rms {rms:.4} Hz {}",
                day,
                civil(*day),
                if *rms >= 1.0 { "LOUD" } else { "quiet" }
            );
        }
    }
}
