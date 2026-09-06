use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, Read};

use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::lsk::days_from_civil;
use omegaflow::odf::bits;
use omegaflow::spectral::civil_from_days;

const BASE: &str = "https://pds-ppi.igpp.ucla.edu/annex/GO-J-RSS-1-ODF-V1.0/ODF/";
const UNIX_1950_OFFSET: f64 = 631152000.0;
const LOUD_HZ: f64 = 1.0;
const ROBUST_N: usize = 30;

const PK_FILE_LABEL: u32 = 101;
const PK_IDENTIFIER: u32 = 107;
const PK_ORBIT_HEADER: u32 = 109;
const PK_RAMP: u32 = 2030;
const PK_CLOCK: u32 = 2040;
const PK_SUMMARY: u32 = 105;

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / 86400.0;
    (jd - 2440587.5).round() as i64
}

fn civil(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn in_trio(station: i64) -> bool {
    station == 14 || station == 43 || station == 63
}

fn is_odf_threeway(dt: i64) -> bool {
    dt == 13 || dt == 14
}

fn is_resid_threeway(mode: i64) -> bool {
    mode == 3 || mode == 4
}

fn name_span(name: &str) -> Option<(i64, i64)> {
    let b = name.as_bytes();
    if b.len() < 7 {
        return None;
    }
    let d0 = (b[0] as char).to_digit(10)? as i64;
    let ds = (b[1] as char).to_digit(10)? as i64 * 100
        + (b[2] as char).to_digit(10)? as i64 * 10
        + (b[3] as char).to_digit(10)? as i64;
    let de = (b[4] as char).to_digit(10)? as i64 * 100
        + (b[5] as char).to_digit(10)? as i64 * 10
        + (b[6] as char).to_digit(10)? as i64;
    let year = 1990 + d0;
    let base = days_from_civil(year, 1, 1)?;
    Some((base + ds - 1, base + de - 1))
}

fn files_of() -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(BASE, 604800) else {
        eprintln!("odf dir listing fetch void ({BASE})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("odf dir listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for token in text.split("href=\"") {
        let Some(end) = token.find('"') else {
            continue;
        };
        let name = &token[..end];
        if name.ends_with(".ODF") {
            out.push(name.to_string());
        }
    }
    out.sort();
    out.dedup();
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

struct RawO {
    t: f64,
    rx: i64,
    tx: i64,
    dt: i64,
    fmt: i64,
    scid: i64,
}

fn parse_odf_file(name: &str, bytes: &[u8]) -> Vec<RawO> {
    if bytes.len() % 36 != 0 {
        eprintln!("{name}: {0} bytes not a multiple of 36 (0 honored)", bytes.len());
        return Vec::new();
    }
    let mut raw: Vec<RawO> = Vec::new();
    let mut in_orbit = false;
    for i in 0..bytes.len() / 36 {
        let mut words = [0u32; 9];
        for k in 0..9 {
            let base = i * 36 + k * 4;
            let mut w4 = [0u8; 4];
            w4.copy_from_slice(&bytes[base..base + 4]);
            words[k] = u32::from_be_bytes(w4);
        }
        match words[0] {
            PK_FILE_LABEL | PK_IDENTIFIER | PK_ORBIT_HEADER | PK_RAMP | PK_CLOCK | PK_SUMMARY => {
                in_orbit = words[0] == PK_ORBIT_HEADER;
            }
            _ => {
                if !in_orbit {
                    continue;
                }
                let fmt = bits(&words, 129, 131);
                let dt = if fmt == 2 {
                    bits(&words, 148, 153)
                } else {
                    bits(&words, 150, 155)
                };
                let scid = if fmt == 2 {
                    bits(&words, 168, 177)
                } else {
                    bits(&words, 160, 167)
                };
                let t_int = words[0] as f64;
                let t_ns = words[1] as f64 / 1.0e9;
                raw.push(RawO {
                    t: t_int + t_ns,
                    rx: bits(&words, 132, 138),
                    tx: bits(&words, 139, 145),
                    dt,
                    fmt,
                    scid,
                });
            }
        }
    }
    raw
}

struct Run {
    rx: i64,
    tx: i64,
    dt: i64,
    t0: f64,
    t1: f64,
    n: usize,
}

fn doppler_runs(raw: &[RawO], lsk: &omegaflow::lsk::LeapSeconds) -> Vec<Run> {
    let mut dopp: Vec<(f64, i64, i64, i64)> = Vec::new();
    for r in raw {
        if !(11..=14).contains(&r.dt) {
            continue;
        }
        if !in_trio(r.rx) || !(in_trio(r.tx) || r.tx == 0) {
            continue;
        }
        let unix = r.t - UNIX_1950_OFFSET;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        dopp.push((tdb, r.rx, r.tx, r.dt));
    }
    dopp.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out: Vec<Run> = Vec::new();
    for (t, rx, tx, dt) in dopp {
        let extend = match out.last() {
            Some(r) => r.rx == rx && r.dt == dt && t - r.t1 <= 7200.0,
            None => false,
        };
        if extend {
            let r = out.last_mut().unwrap();
            r.t1 = t;
            r.n += 1;
        } else {
            out.push(Run {
                rx,
                tx,
                dt,
                t0: t,
                t1: t,
                n: 1,
            });
        }
    }
    out
}

fn read_resid_cells() -> BTreeMap<(i64, i64, i64), CellStat> {
    let path = "data/galileo_resid.bin";
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("{path} absent (0 honored)");
            return BTreeMap::new();
        }
    };
    let mut reader = BufReader::new(file);
    let mut magic = [0u8; 4];
    if reader.read_exact(&mut magic).is_err() || &magic != b"GASR" {
        eprintln!("{path} magic void");
        return BTreeMap::new();
    }
    let mut count_buf = [0u8; 4];
    if reader.read_exact(&mut count_buf).is_err() {
        eprintln!("{path} count void");
        return BTreeMap::new();
    }
    let mut cells: BTreeMap<(i64, i64, i64), CellStat> = BTreeMap::new();
    let mut rec = [0u8; 64];
    loop {
        match reader.read_exact(&mut rec) {
            Ok(()) => {}
            Err(_) => break,
        }
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
        if !in_trio(station) || !is_resid_threeway(mode) {
            continue;
        }
        let day = unix_day(r[0]);
        cells.entry((mode, station, day)).or_insert_with(CellStat::new).push(resid);
    }
    cells
}

fn main() {
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void (0 honored)");
        return;
    };

    println!("== 1. mode-3 floor cells over the local resid asset (whole era) ==");
    let cells = read_resid_cells();
    let m3days: BTreeSet<i64> = cells.keys().map(|&(_, _, day)| day).collect();
    let mut robust_series: BTreeMap<(i64, i64), Vec<(i64, f64, usize)>> = BTreeMap::new();
    for ((mode, station, day), s) in cells.iter() {
        let n = s.n as usize;
        if n < ROBUST_N {
            continue;
        }
        if let Some(rms) = s.rms() {
            robust_series
                .entry((*station, *mode))
                .or_insert_with(Vec::new)
                .push((*day, rms, n));
        }
    }
    let mut total_robust = 0usize;
    let mut total_loud = 0usize;
    let mut keys: Vec<_> = robust_series.iter_mut().collect();
    keys.sort_by(|a, b| (a.0 .0, a.0 .1).cmp(&(b.0 .0, b.0 .1)));
    for ((station, mode), rows) in keys {
        rows.sort_by(|a, b| a.0.cmp(&b.0));
        let loud = rows.iter().filter(|(_, rms, _)| *rms >= LOUD_HZ).count();
        total_robust += rows.len();
        total_loud += loud;
        println!(
            "station {station} mode {mode}: robust days {} loud days {}",
            rows.len(),
            loud
        );
        for (day, rms, n) in rows.iter() {
            println!(
                "  day {} date {} n {n} rms {rms:.4} Hz {}",
                day,
                civil(*day),
                if *rms >= LOUD_HZ { "LOUD" } else { "quiet" }
            );
        }
    }
    println!(
        "mode-3 robust floor cells total {total_robust}, loud {total_loud}; distinct days with mode-3 floor samples {}",
        m3days.len()
    );

    println!("\n== 2. ODF volume files and window ==");
    let names = files_of();
    println!("GO-J-RSS-1-ODF-V1.0/ODF: {} files", names.len());
    let mut window_days: BTreeSet<i64> = BTreeSet::new();
    let mut first_day: Option<i64> = None;
    let mut last_day: Option<i64> = None;
    for name in &names {
        let Some((d0, d1)) = name_span(name) else {
            println!("  {name}: name not year-doy-doy");
            continue;
        };
        first_day = Some(first_day.map_or(d0, |a: i64| a.min(d0)));
        last_day = Some(last_day.map_or(d1, |a: i64| a.max(d1)));
        for d in d0..=d1 {
            window_days.insert(d);
        }
        println!("  {name}: {} .. {}", civil(d0), civil(d1));
    }
    match (first_day, last_day) {
        (Some(lo), Some(hi)) => println!(
            "ODF window {} .. {} ({} distinct days in file spans)",
            civil(lo),
            civil(hi),
            window_days.len()
        ),
        _ => println!("ODF window void (no files)"),
    }
    if let (Some(lo), Some(hi)) = (m3days.iter().min(), m3days.iter().max()) {
        println!(
            "mode-3 floor era (resid) {} .. {} — ODF window days inside mode-3 era: {}",
            civil(*lo),
            civil(*hi),
            window_days.intersection(&m3days).count()
        );
    }

    println!("\n== 3. ODF orbit record field census (transmitting station reachability) ==");
    let mut fetch_days: BTreeSet<i64> = m3days.intersection(&window_days).copied().collect();
    let m3_last = m3days.iter().max().copied();
    if let Some(last) = m3_last {
        for name in &names {
            if let Some((d0, d1)) = name_span(name) {
                if d0 <= last {
                    fetch_days.insert(d0);
                    fetch_days.insert(d1);
                }
            }
        }
    }
    println!(
        "fetch selection: {} candidate file days (mode-3 day overlap plus every ODF file starting within the mode-3 era)",
        fetch_days.len()
    );
    let extra: BTreeSet<String> = std::env::args().skip(1).collect();
    let mut fetched: Vec<String> = Vec::new();
    for name in &names {
        let Some((d0, d1)) = name_span(name) else {
            continue;
        };
        if !extra.contains(name) && !(d0..=d1).any(|d| fetch_days.contains(&d)) {
            continue;
        }
        let url = format!("{BASE}{name}");
        let cache_path = format!("/tmp/opencode/galileo_odf_cache_{name}");
        let bytes = match std::fs::read(&cache_path) {
            Ok(b) => b,
            Err(_) => match fetch_raw_bytes(&url, 604800) {
                Some(b) => {
                    let _ = std::fs::write(&cache_path, &b);
                    b
                }
                None => {
                    eprintln!("{name}: fetch void ({url})");
                    continue;
                }
            },
        };
        let raw = parse_odf_file(name, &bytes);
        let mut fmt_hist: BTreeMap<i64, usize> = BTreeMap::new();
        let mut dt_hist: BTreeMap<i64, usize> = BTreeMap::new();
        let mut scid_hist: BTreeMap<i64, usize> = BTreeMap::new();
        let mut rx_tx: BTreeMap<(i64, i64, i64), usize> = BTreeMap::new();
        for r in &raw {
            *fmt_hist.entry(r.fmt).or_insert(0) += 1;
            *dt_hist.entry(r.dt).or_insert(0) += 1;
            *scid_hist.entry(r.scid).or_insert(0) += 1;
            if (11..=14).contains(&r.dt) {
                *rx_tx.entry((r.rx, r.tx, r.dt)).or_insert(0) += 1;
            }
        }
        fetched.push(name.clone());
        println!(
            "  FILE {name}: {} orbit data words, fmt {:?}, scid {:?}",
            raw.len(),
            fmt_hist,
            scid_hist
        );
        let mut dk: Vec<_> = dt_hist.into_iter().collect();
        dk.sort();
        for (dt, c) in dk {
            println!("    data_type {dt}: {c}");
        }
        for ((rx, tx, dt), c) in rx_tx {
            println!("    rx {rx} tx {tx} data_type {dt}: {c}");
        }
    }
    println!("ODF files fetched: {}", fetched.len());

    println!("\n== 4. (receiving x transmitting) three-way runs and the floor split ==");
    let mut runs_all: Vec<Run> = Vec::new();
    for name in &fetched {
        let cache_path = format!("/tmp/opencode/galileo_odf_cache_{name}");
        let Ok(bytes) = std::fs::read(&cache_path) else {
            continue;
        };
        let raw = parse_odf_file(name, &bytes);
        let runs = doppler_runs(&raw, &lsk);
        println!("  ODF-RUNS {name}: {} doppler runs (rx tx dt n t0 t1)", runs.len());
        for r in &runs {
            println!(
                "    rx {} tx {} dt {} n {} {} .. {}",
                r.rx,
                r.tx,
                r.dt,
                r.n,
                civil(unix_day(r.t0)),
                civil(unix_day(r.t1))
            );
        }
        runs_all.extend(runs);
    }
    let three_runs: Vec<&Run> = runs_all.iter().filter(|r| is_odf_threeway(r.dt)).collect();
    println!("three-way ODF runs over fetched files: {}", three_runs.len());
    for r in &three_runs {
        println!(
            "    rx {} tx {} dt {} n {} {} .. {}",
            r.rx,
            r.tx,
            r.dt,
            r.n,
            civil(unix_day(r.t0)),
            civil(unix_day(r.t1))
        );
    }
    if three_runs.is_empty() {
        println!(
            "no ODF three-way orbit record in the fetched window — the (rx x tx) floor split is not measurable on it (0 honored)"
        );
        return;
    }
    let path = "data/galileo_resid.bin";
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
    let mut split: BTreeMap<(i64, i64, i64, i64, i64), CellStat> = BTreeMap::new();
    let mut unassigned: BTreeMap<(i64, i64, i64), CellStat> = BTreeMap::new();
    let mut rec = [0u8; 64];
    loop {
        match reader.read_exact(&mut rec) {
            Ok(()) => {}
            Err(_) => break,
        }
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
        if !in_trio(station) || !is_resid_threeway(mode) {
            continue;
        }
        let hit = three_runs.iter().find(|run| {
            run.rx == station && r[0] >= run.t0 - 60.0 && r[0] <= run.t1 + 60.0
        });
        match hit {
            Some(run) => {
                split
                    .entry((unix_day(r[0]), station, run.tx, mode, run.dt))
                    .or_insert_with(CellStat::new)
                    .push(resid);
            }
            None => {
                unassigned
                    .entry((unix_day(r[0]), station, mode))
                    .or_insert_with(CellStat::new)
                    .push(resid);
            }
        }
    }
    println!("(receiving x transmitting) robust floor cells over ODF-covered mode-3 samples (day rx tx mode dt n rms class):");
    let mut sk: Vec<_> = split.into_iter().collect();
    sk.sort_by(|a, b| a.0.cmp(&b.0));
    let mut tx_agg: BTreeMap<(i64, i64, i64, i64), (usize, usize)> = BTreeMap::new();
    for ((day, station, tx, mode, dt), s) in sk {
        let n = s.n as usize;
        if n < ROBUST_N {
            continue;
        }
        if let Some(rms) = s.rms() {
            let loud = rms >= LOUD_HZ;
            println!(
                "  day {} date {} rx {} tx {} mode {} dt {} n {n} rms {rms:.4} Hz {}",
                day,
                civil(day),
                station,
                tx,
                mode,
                dt,
                if loud { "LOUD" } else { "quiet" }
            );
            let e = tx_agg.entry((station, tx, mode, dt)).or_insert((0, 0));
            e.1 += n;
            if loud {
                e.0 += 1;
            }
        }
    }
    println!("robust loud cells and floor samples per (rx, tx, mode, dt):");
    for ((rx, tx, mode, dt), (loud_cells, n)) in tx_agg {
        println!("  rx {rx} tx {tx} mode {mode} dt {dt}: loud cells {loud_cells}, floor samples {n}");
    }
    println!(
        "mode-3 floor samples inside ODF-covered three-way windows but not matched to a run: {} (day, rx, mode) cells",
        unassigned.len()
    );
}
