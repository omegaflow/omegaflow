use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::goes::{self, COMP_XRSA, COMP_XRSB};
use omegaflow::archivar::omni2::{self, COMP_BZ, COMP_N1800};
use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};

const GOES_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_xrs.bin";
const OMNI2_1H_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie_1h.bin";
const GOES13_DIR: &str = "data/ncei.noaa.gov/goes15_2013";
const GOES14_DIR: &str = "data/ncei.noaa.gov/goes15";
const GOES15_DIR: &str = "data/ncei.noaa.gov/goes15_2015";
const AIA_DIR: &str = "data/jsoc.stanford.edu";
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const HOUR: f64 = 3600.0;
const LAGS_HOUR: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const MIN_N: usize = 30;
const N_SURR: usize = 10;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const TT_UNIX: f64 = 32.184;
const WINDOW_LO: f64 = 1356998400.0;
const WINDOW_HI: f64 = 1451606400.0;
const FLARE_THRESH: f64 = 5e-6;
const REFRACTORY_HOURS: usize = 12;
const DEFAULT_WINDOW_HOURS: usize = 24;
const DEFAULT_MIN_EVENTS: usize = MIN_N;

const AIA_BANDS: [(u32, &str); 7] = [
    (5, "304A"),
    (1, "131A"),
    (2, "171A"),
    (3, "193A"),
    (4, "211A"),
    (6, "335A"),
    (0, "94A"),
];
const AIA_MAGIC: [u8; 4] = *b"AIA1";


fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load_bin(kind: &str, url: &str, path: Option<String>) -> Option<Vec<u8>> {
    if let Some(p) = path {
        match std::fs::read(&p) {
            Ok(bytes) => return Some(bytes),
            Err(_) => {
                eprintln!("{kind}: {} reads void — the channel stays unmeasured", p);
                return None;
            }
        }
    }
    match fetch_raw_bytes(url, 3600) {
        Some(bytes) => Some(bytes),
        None => {
            eprintln!("{kind}: {url} carries no asset — the channel stays unmeasured (0 honored)");
            None
        }
    }
}

fn bin_mean(series: &[(f64, f64)], t0: f64, n: usize, cell: f64) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        if !(t >= t0 && t < t0 + cell * n as f64) {
            continue;
        }
        let idx = ((t - t0) / cell).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += v;
        counts[i] += 1;
    }
    (0..n)
        .map(|i| {
            if counts[i] > 0 {
                Some((sums[i] / counts[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect()
}

fn fold_max_into(best: &mut [Option<f32>], series: &[(f64, f64)], t0: f64, cell: f64) {
    for &(t, v) in series {
        let idx = ((t - t0) / cell).floor();
        if idx < 0.0 || idx >= best.len() as f64 {
            continue;
        }
        let i = idx as usize;
        let vf = v as f32;
        if best[i].map_or(true, |b| vf > b) {
            best[i] = Some(vf);
        }
    }
}

fn mean_plus_2sigma(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64;
    Some(m + 2.0 * var.sqrt())
}

struct Actor {
    name: &'static str,
    kind: &'static str,
    series: Vec<(f64, f64)>,
}

fn read_aia_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{} reads void", path);
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != AIA_MAGIC {
        eprintln!("{} carries no AIA1 contract", path);
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 20;
        let Some(t) = bytes
            .get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(v) = bytes
            .get(o + 8..o + 16)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(idx) = bytes
            .get(o + 16..o + 20)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
        else {
            continue;
        };
        out.push((t, v, idx));
    }
    out
}

fn load_aia(dir: &str, year: u32) -> Vec<(f64, f64, u32)> {
    let mut all = Vec::new();
    if year == 2014 {
        for m in 1..=12u32 {
            let p = format!("{}/aia2014_{:02}.bin", dir, m);
            all.extend(read_aia_lines(&p));
        }
    } else {
        let p = format!("{}/aia{}_fullyear.bin", dir, year);
        all.extend(read_aia_lines(&p));
    }
    all
}

fn goes_b_flux(path: &str) -> Vec<(f64, f64)> {
    let Ok(bytes) = std::fs::read(path) else {
        return Vec::new();
    };
    let Ok(file) = Hdf5File::parse(&bytes) else {
        return Vec::new();
    };
    let (Ok(t_raw), Ok(v_raw), Ok(f_raw)) = (
        file.read_dataset("time"),
        file.read_dataset("b_flux"),
        file.read_dataset("b_flags"),
    ) else {
        return Vec::new();
    };
    let n = t_raw.len() / 8;
    if v_raw.len() != n * 4 || f_raw.len() != n * 2 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let Some(t) = decode_f64(&t_raw, i * 8, Endian::Le) else {
            continue;
        };
        let flag = u16::from_le_bytes([f_raw[i * 2], f_raw[i * 2 + 1]]);
        if flag != 0 {
            continue;
        }
        let Some(v) = decode_f32(&v_raw, i * 4, Endian::Le) else {
            continue;
        };
        let v = v as f64;
        if !v.is_finite() || v == -9999.0 || v <= 0.0 {
            continue;
        }
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn fold_b_flux_dir(best: &mut [Option<f32>], dir: &str) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        eprintln!("{} reads void — that year's flare record stays unmeasured", dir);
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("xr_") && name.ends_with(".nc") {
            let raw = goes_b_flux(&entry.path().to_string_lossy());
            fold_max_into(best, &raw, WINDOW_LO, HOUR);
        }
    }
}

fn cut_events(trig: &[Option<f32>], threshold: f32, refractory: usize) -> Vec<usize> {
    let n = trig.len();
    let mut peaks = Vec::new();
    let mut i = 0usize;
    while i < n {
        let Some(tv) = trig[i] else {
            i += 1;
            continue;
        };
        if tv < threshold {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut peak = i;
        while j < n && j - i < refractory {
            if let Some(v) = trig[j] {
                if v < threshold && j > i + 3 {
                    break;
                }
                if let Some(pv) = trig[peak] {
                    if v > pv {
                        peak = j;
                    }
                }
            }
            j += 1;
        }
        peaks.push(peak);
        i = j.max(i + refractory);
    }
    peaks
}

fn year_of(t: f64) -> u32 {
    if t < 1388534400.0 {
        2013
    } else if t < 1420070400.0 {
        2014
    } else {
        2015
    }
}

struct WindowPair {
    x: Vec<f32>,
    y: Vec<f32>,
}

fn window_pairs(
    cells: &[Vec<Option<f32>>],
    events: &[usize],
    from: usize,
    to: usize,
    half: usize,
    min_cells: usize,
) -> Vec<WindowPair> {
    let n_cells = cells[0].len();
    let target = &cells[to];
    let driver = &cells[from];
    let mut out = Vec::new();
    for &peak in events {
        let lo = peak.saturating_sub(half);
        let hi = (peak + half + 1).min(n_cells);
        if hi - lo < min_cells {
            continue;
        }
        let mut x = Vec::with_capacity(hi - lo);
        let mut y = Vec::with_capacity(hi - lo);
        for k in lo..hi {
            if let (Some(xv), Some(yv)) = (target[k], driver[k]) {
                x.push(xv);
                y.push(yv);
            }
        }
        if x.len() >= min_cells {
            out.push(WindowPair { x, y });
        }
    }
    out
}

fn stack_pass(
    windows: &[WindowPair],
    lag: usize,
    shuffle: bool,
    seed: u64,
) -> (f64, usize, usize, f64) {
    let mut sum = 0.0;
    let mut pos = 0usize;
    let mut tot = 0usize;
    let mut cell_sum = 0.0;
    for (idx, w) in windows.iter().enumerate() {
        let (f, r) = if shuffle {
            let mut rng = seed.wrapping_add(idx as u64 * 0x9E37_79B9_7F4A_7C15);
            let y_sur = phase_randomized_surrogate(&w.y, &mut rng);
            let (Some(f), Some(r)) = (
                transfer_entropy_lag(&w.x, &y_sur, lag),
                transfer_entropy_lag(&y_sur, &w.x, lag),
            ) else {
                continue;
            };
            (f, r)
        } else {
            let (Some(f), Some(r)) = (
                transfer_entropy_lag(&w.x, &w.y, lag),
                transfer_entropy_lag(&w.y, &w.x, lag),
            ) else {
                continue;
            };
            (f, r)
        };
        let d = f - r;
        let scale = f.abs() + r.abs();
        let term = if scale > 0.0 { d / scale } else { 0.0 };
        sum += term;
        if d > 0.0 {
            pos += 1;
        }
        tot += 1;
        cell_sum += w.x.len() as f64;
    }
    (sum, pos, tot, cell_sum)
}

struct Row {
    from: usize,
    to: usize,
    n_ev: usize,
    cells_mean: f64,
    best_lag: usize,
    d: f64,
    thr: f64,
    pos: usize,
    surr_max: f64,
}

fn row_for(
    cells: &[Vec<Option<f32>>],
    events: &[usize],
    from: usize,
    to: usize,
    half: usize,
    min_cells: usize,
) -> Row {
    let windows = window_pairs(cells, events, from, to, half, min_cells);
    let mut surr_max = f64::NEG_INFINITY;
    let mut best: Option<(usize, f64)> = None;
    let mut best_thr = f64::NAN;
    let mut best_pos = 0usize;
    let mut best_tot = 0usize;
    let mut best_cells = f64::NAN;
    for &lag in LAGS_HOUR.iter() {
        let (sum, pos, tot, cell_sum) = stack_pass(&windows, lag, false, 0);
        let d = if tot > 0 {
            sum / tot as f64
        } else {
            f64::NAN
        };
        let mut surr_vals: Vec<f64> = Vec::new();
        for s in 1..=N_SURR {
            let seed = SURROGATE_SEED
                ^ (from as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                ^ (to as u64).wrapping_mul(0x517C_C1B7_2722_0A95)
                ^ (lag as u64).wrapping_mul(0xD1B5_4A32_D192_ED03)
                ^ (s as u64).wrapping_mul(0x0FEB_11D1_B2D1_9C93);
            let (ss, _, stot, _) = stack_pass(&windows, lag, true, seed);
            if stot > 0 {
                surr_vals.push(ss / stot as f64);
            }
        }
        for &v in &surr_vals {
            if v > surr_max {
                surr_max = v;
            }
        }
        let thr = mean_plus_2sigma(&surr_vals).unwrap_or(f64::NAN);
        if d.is_finite() {
            if best.map_or(true, |(_, b)| d > b) {
                best = Some((lag, d));
                best_thr = thr;
                best_pos = pos;
                best_tot = tot;
                best_cells = cell_sum / tot as f64;
            }
        }
    }
    match best {
        Some((best_lag, d)) => Row {
            from,
            to,
            n_ev: best_tot,
            cells_mean: best_cells,
            best_lag,
            d,
            thr: best_thr,
            pos: best_pos,
            surr_max,
        },
        None => Row {
            from,
            to,
            n_ev: 0,
            cells_mean: f64::NAN,
            best_lag: 0,
            d: f64::NAN,
            thr: f64::NAN,
            pos: 0,
            surr_max,
        },
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let window_hours = match arg_value(&args, "--window-hours") {
        Some(w) => match w.parse::<usize>() {
            Ok(x) => x,
            Err(_) => DEFAULT_WINDOW_HOURS,
        },
        None => DEFAULT_WINDOW_HOURS,
    };
    let min_events = match arg_value(&args, "--min-events") {
        Some(m) => match m.parse::<usize>() {
            Ok(x) => x,
            Err(_) => DEFAULT_MIN_EVENTS,
        },
        None => DEFAULT_MIN_EVENTS,
    };
    let aia_dir = match arg_value(&args, "--aia-dir") {
        Some(d) => d,
        None => AIA_DIR.to_string(),
    };
    let goes_dirs = [
        match arg_value(&args, "--bflux-2013-dir") {
            Some(d) => d,
            None => GOES13_DIR.to_string(),
        },
        match arg_value(&args, "--bflux-2014-dir") {
            Some(d) => d,
            None => GOES14_DIR.to_string(),
        },
        match arg_value(&args, "--bflux-2015-dir") {
            Some(d) => d,
            None => GOES15_DIR.to_string(),
        },
    ];

    let window_cells = 2 * window_hours + 1;
    let min_cells = MIN_N.min(window_cells);
    let n_cells = ((WINDOW_HI - WINDOW_LO) / HOUR).floor() as usize;

    println!(
        "=== Hourly event-wise all-actor solar TE matrix over 2013-2015 ==="
    );
    println!(
        "Window (unix {:.0}..{:.0}): {} hourly cells over 2013 + 2014 + 2015.",
        WINDOW_LO, WINDOW_HI, n_cells
    );
    println!(
        "Actors (11): GOES XRSA/XRSB (goes_xrs.bin, TDB -> unix), OMNI Bz/Density (omni2_serie_1h.bin, unix as-is) — kind L1 — and 7 AIA EUV bands (J2000 TT + {:.0} - {:.1} -> unix) — kind aia; XRS kind historical. F10.7 and Lyman-alpha are daily-born and excluded (no hourly resolution).",
        J2000_UNIX_OFFSET, TT_UNIX
    );
    println!(
        "Flare events: GOES soft X-ray b_flux (2-s xr_*.nc per day, hourly-max cells) over the C1.0 threshold {:.0e} W/m², refractory {} h.",
        FLARE_THRESH, REFRACTORY_HOURS
    );
    println!(
        "Event window: peak hour ±{} h ({} hourly cells); an event feeds a pair when >= {} of its window cells carry both actors (MIN_N = {} at the default ±{} h window).",
        window_hours, window_cells, min_cells, MIN_N, window_hours
    );
    println!("Direction 'A -> B' reads 'A drives B': per-event D = (TE(A->B) - TE(B->A)) / (|TE(A->B)| + |TE(B->A)|); the pair stacks the mean of per-event D over events; pos counts events with D > 0.");
    println!(
        "Verdicts: ARROW (stacked D > fam) | family bound (stacked D > own mean+2sigma over {} surrogates, <= fam) | still | no-statement ({} < {} events, the MIN_N event gate).",
        N_SURR, "events", min_events
    );
    println!(
        "fam = strongest surrogate stacked D over all directed pairs x {} lags x {} surrogates; cross-block fam over pairs whose actors differ in kind (excludes XRSA<->XRSB and intra-AIA).",
        LAGS_HOUR.len(),
        N_SURR
    );
    println!();

    let goes_bytes = load_bin("goes_xrs.bin", GOES_CDN, arg_value(&args, "--goes-bin"));
    let omni2_1h_bytes = load_bin(
        "omni2_serie_1h.bin",
        OMNI2_1H_CDN,
        arg_value(&args, "--omni2-1h-bin"),
    );
    let Some(lsk) = embedded_lsk() else {
        eprintln!("embedded LSK absent — the TDB axes stay unconverted");
        return;
    };

    let goes_records = match &goes_bytes {
        Some(b) => match goes::parse_bin(b) {
            Some(r) => r,
            None => Vec::new(),
        },
        None => Vec::new(),
    };
    let omni_records = match &omni2_1h_bytes {
        Some(b) => match omni2::parse_bin(b) {
            Some(r) => r,
            None => Vec::new(),
        },
        None => Vec::new(),
    };

    let mut actors: Vec<Actor> = Vec::new();
    actors.push(Actor {
        name: "XRSA",
        kind: "historical",
        series: goes_records
            .iter()
            .filter(|(_, _, c)| *c == COMP_XRSA)
            .filter_map(|&(t, v, _)| lsk.tdb_to_unix(t).map(|u| (u, v)))
            .filter(|&(t, _)| t >= WINDOW_LO && t < WINDOW_HI)
            .collect(),
    });
    actors.push(Actor {
        name: "XRSB",
        kind: "historical",
        series: goes_records
            .iter()
            .filter(|(_, _, c)| *c == COMP_XRSB)
            .filter_map(|&(t, v, _)| lsk.tdb_to_unix(t).map(|u| (u, v)))
            .filter(|&(t, _)| t >= WINDOW_LO && t < WINDOW_HI)
            .collect(),
    });
    for (name, comp) in [("Bz", COMP_BZ), ("Density", COMP_N1800)] {
        actors.push(Actor {
            name,
            kind: "L1",
            series: omni_records
                .iter()
                .filter(|(_, _, c)| *c == comp)
                .map(|&(t, v, _)| (t, v))
                .filter(|&(t, _)| t >= WINDOW_LO && t < WINDOW_HI)
                .collect(),
        });
    }
    let mut aia_all: Vec<(f64, f64, u32)> = Vec::new();
    for year in [2013u32, 2014, 2015] {
        aia_all.extend(load_aia(&aia_dir, year));
    }
    for &(bidx, bname) in AIA_BANDS.iter() {
        actors.push(Actor {
            name: bname,
            kind: "aia",
            series: aia_all
                .iter()
                .filter(|(_, _, i)| *i == bidx)
                .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET - TT_UNIX, v))
                .filter(|&(t, _)| t >= WINDOW_LO && t < WINDOW_HI)
                .collect(),
        });
    }

    println!("=== Actor board (kind | n samples over the window) ===");
    for a in &actors {
        match (a.series.first(), a.series.last()) {
            (Some(&(t0v, _)), Some(&(t1v, _))) => println!(
                "{:<9} | {:<10} | n {:<10} | {:.0} d .. {:.0} d",
                a.name,
                a.kind,
                a.series.len(),
                t0v / 86400.0,
                t1v / 86400.0
            ),
            _ => println!(
                "{:<9} | {:<10} | n 0 — absent over the window",
                a.name, a.kind
            ),
        }
    }

    let cells: Vec<Vec<Option<f32>>> = actors
        .iter()
        .map(|a| bin_mean(&a.series, WINDOW_LO, n_cells, HOUR))
        .collect();

    let mut trig_cells: Vec<Option<f32>> = vec![None; n_cells];
    for dir in goes_dirs.iter() {
        fold_b_flux_dir(&mut trig_cells, dir);
    }
    let events = cut_events(&trig_cells, FLARE_THRESH as f32, REFRACTORY_HOURS);
    let mut counts: [usize; 3] = [0, 0, 0];
    for &peak in &events {
        let t = WINDOW_LO + peak as f64 * HOUR;
        match year_of(t) {
            2013 => counts[0] += 1,
            2014 => counts[1] += 1,
            _ => counts[2] += 1,
        }
    }
    println!();
    println!(
        "=== Flare events (GOES b_flux hourly-max > {:.0e} W/m²) ===",
        FLARE_THRESH
    );
    println!("2013: {} events", counts[0]);
    println!("2014: {} events", counts[1]);
    println!("2015: {} events", counts[2]);
    println!("total: {} events", events.len());
    if events.is_empty() {
        println!();
        println!("The event stack stays empty — no flare cell over the threshold (0 honored).");
        return;
    }

    println!();
    let n_actors = actors.len();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..n_actors {
        for j in 0..n_actors {
            if i != j {
                pairs.push((i, j));
            }
        }
    }
    let n_threads = std::thread::available_parallelism()
        .map(|v| v.get())
        .unwrap_or(4)
        .min(16)
        .min(pairs.len());
    let chunk = pairs.len().div_ceil(n_threads);
    let mut rows: Vec<Row> = std::thread::scope(|scope| {
        let handles: Vec<_> = pairs
            .chunks(chunk)
            .map(|chunk_pairs| {
                let cells_ref = &cells;
                let events_ref = &events;
                scope.spawn(move || {
                    chunk_pairs
                        .iter()
                        .map(|&(fi, ti)| row_for(cells_ref, events_ref, fi, ti, window_hours, min_cells))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        let mut out = Vec::new();
        for h in handles {
            match h.join() {
                Ok(v) => out.extend(v),
                Err(_) => {}
            }
        }
        out
    });
    rows.sort_by_key(|r| (r.from, r.to));

    let mut fam = f64::NEG_INFINITY;
    let mut cross_fam = f64::NEG_INFINITY;
    for r in &rows {
        if r.surr_max > fam {
            fam = r.surr_max;
        }
        if actors[r.from].kind != actors[r.to].kind && r.surr_max > cross_fam {
            cross_fam = r.surr_max;
        }
    }
    println!();
    println!(
        "fam (full matrix) = {:.4e} over {} directed pairs x {} lags x {} surrogates.",
        fam,
        pairs.len(),
        LAGS_HOUR.len(),
        N_SURR
    );
    println!(
        "cross-block fam (kinds differ) = {:.4e} — the guard for the cross-block channel DAG.",
        cross_fam
    );
    println!();
    println!(
        "=== The {} x {} event-wise matrix (stacked per-event D, best lag in h) ===",
        n_actors, n_actors
    );
    for r in &rows {
        let word = if r.n_ev < min_events || !r.d.is_finite() {
            "no-statement"
        } else if fam.is_finite() && r.d > fam {
            "ARROW"
        } else if r.thr.is_finite() && r.d > r.thr {
            "family bound"
        } else {
            "still"
        };
        let cross = if actors[r.from].kind != actors[r.to].kind {
            if cross_fam.is_finite() {
                let cw = if r.d > cross_fam { "cb-arrow" } else { "cb-still" };
                format!(" [{}]", cw)
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        println!(
            "{:>8} -> {:<8} | n_ev {:>4} | {:.1} cells/ev | lag {:>2} h | D {:>10.4e} | thr {:>10.4e} | pos {:>4}/{:>4} | {}{}",
            actors[r.from].name,
            actors[r.to].name,
            r.n_ev,
            r.cells_mean,
            r.best_lag,
            r.d,
            r.thr,
            r.pos,
            r.n_ev,
            word,
            cross
        );
    }
    println!();
    let arrows: Vec<&Row> = rows.iter().filter(|r| r.d > fam && r.n_ev >= min_events).collect();
    if arrows.is_empty() {
        println!("No event-wise stacked D clears the full-round family bound fam — silence is a finding (0 honored).");
    } else {
        for r in arrows {
            println!(
                "{} -> {} (lag {} h, D {:.4e} > fam {:.4e}, {} events)",
                actors[r.from].name,
                actors[r.to].name,
                r.best_lag,
                r.d,
                fam,
                r.n_ev
            );
        }
    }
}
