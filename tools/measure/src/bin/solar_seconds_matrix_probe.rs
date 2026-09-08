use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::scalar_te_gpu::ScalarTeGpu;
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};
use std::sync::Mutex;

const AIA_MAGIC: [u8; 4] = *b"AIA1";
const DT: f64 = 24.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const TT_UNIX: f64 = 32.184;
const WINDOW_LO: f64 = 1356998400.0;
const WINDOW_HI: f64 = 1451606400.0;
const YEAR_UNIX: [f64; 3] = [1356998400.0, 1388534400.0, 1420070400.0];
const YEAR_NUMBER: [u32; 3] = [2013, 2014, 2015];
const AIA_DIR: &str = "data/jsoc.stanford.edu";
const GOES13_DIR: &str = "data/ncei.noaa.gov/goes15_2013";
const GOES14_DIR: &str = "data/ncei.noaa.gov/goes15";
const GOES15_DIR: &str = "data/ncei.noaa.gov/goes15_2015";
const FLARE_THRESH: f64 = 5e-6;
const REFRACTORY: usize = 75;
const WINDOW_CELLS_BASE: usize = 100;
const MIN_CELLS: usize = 100;
const MIN_EVENTS_FLOOR: usize = 30;
const N_SURR: usize = 10;
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAG_MAX: usize = 12;
const FILL: f64 = -9999.0;

const AIA_BANDS: [(u32, &str); 7] = [
    (5, "304A"),
    (1, "131A"),
    (2, "171A"),
    (3, "193A"),
    (4, "211A"),
    (6, "335A"),
    (0, "94A"),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn load_year_aia(dir: &str, year: u32) -> Vec<(f64, f64, u32)> {
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

fn goes_xrs_day(path: &str, flux: &str, flags: &str) -> Vec<(f64, f64)> {
    let Ok(bytes) = std::fs::read(path) else {
        return Vec::new();
    };
    let Ok(file) = Hdf5File::parse(&bytes) else {
        return Vec::new();
    };
    let (Ok(t_raw), Ok(v_raw), Ok(f_raw)) = (
        file.read_dataset("time"),
        file.read_dataset(flux),
        file.read_dataset(flags),
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
        if !v.is_finite() || v == FILL || v <= 0.0 {
            continue;
        }
        out.push((t, v));
    }
    out
}

fn dir_xrs_series(dir: &str, flux: &str, flags: &str) -> Vec<(f64, f64)> {
    let mut series: Vec<(f64, f64)> = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        eprintln!("{} reads void", dir);
        return series;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("xr_") && name.ends_with(".nc") {
            series.extend(goes_xrs_day(&entry.path().to_string_lossy(), flux, flags));
        }
    }
    series.sort_by(|a, b| a.0.total_cmp(&b.0));
    series
}

fn bin_median(series: &[(f64, f64)], t0: f64, bins: usize) -> Vec<Option<f32>> {
    let mut acc: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(t, v) in series {
        let idx = ((t - t0) / DT).floor();
        if idx < 0.0 || idx >= bins as f64 {
            continue;
        }
        acc[idx as usize].push(v);
    }
    acc.into_iter()
        .map(|mut v| {
            if v.is_empty() {
                return None;
            }
            v.sort_by(|a, b| a.total_cmp(b));
            let m = if v.len() % 2 == 0 {
                (v[v.len() / 2 - 1] + v[v.len() / 2]) * 0.5
            } else {
                v[v.len() / 2]
            };
            Some(m as f32)
        })
        .collect()
}

fn place(grid: &mut [Option<f32>], sub: &[Option<f32>], offset: usize) {
    for (i, cell) in sub.iter().enumerate() {
        if let Some(v) = cell {
            grid[offset + i] = Some(*v);
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

fn year_of(t: f64) -> usize {
    if t < YEAR_UNIX[1] {
        0
    } else if t < YEAR_UNIX[2] {
        1
    } else {
        2
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
    gpu: Option<&Mutex<ScalarTeGpu>>,
) -> Row {
    let windows = window_pairs(cells, events, from, to, half, min_cells);
    let mut surrogates: Vec<Vec<Vec<f32>>> = Vec::with_capacity(windows.len());
    for (idx, w) in windows.iter().enumerate() {
        let mut per = Vec::with_capacity(N_SURR);
        for s in 1..=N_SURR {
            let base = SURROGATE_SEED
                ^ (from as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
                ^ (to as u64).wrapping_mul(0x517C_C1B7_2722_0A95)
                ^ (s as u64).wrapping_mul(0x0FEB_11D1_B2D1_9C93);
            let mut rng = base.wrapping_add(idx as u64 * 0x9E37_79B9_7F4A_7C15);
            per.push(phase_randomized_surrogate(&w.y, &mut rng));
        }
        surrogates.push(per);
    }
    let grids: Option<Vec<Vec<f32>>> = gpu.map(|g| {
        windows
            .iter()
            .zip(surrogates.iter())
            .map(|(w, surr)| g.lock().unwrap().run(&w.x, &w.y, surr))
            .collect()
    });
    let measure = |lag: usize, sel: Option<usize>| -> (f64, usize, usize, f64) {
        let mut sum = 0.0;
        let mut pos = 0usize;
        let mut tot = 0usize;
        let mut cell_sum = 0.0;
        for (idx, w) in windows.iter().enumerate() {
            let (f, r) = match &grids {
                Some(grids) => {
                    let k = match sel {
                        None => 0usize,
                        Some(s) => 1 + s,
                    };
                    let fv = grids[idx][((0 * 11 + k) * 13 + lag) * 2 + 1];
                    let rv = grids[idx][((1 * 11 + k) * 13 + lag) * 2 + 1];
                    if fv == 0.0 || rv == 0.0 {
                        continue;
                    }
                    let f = grids[idx][((0 * 11 + k) * 13 + lag) * 2] as f64;
                    let r = grids[idx][((1 * 11 + k) * 13 + lag) * 2] as f64;
                    (f, r)
                }
                None => {
                    let y = match sel {
                        None => &w.y,
                        Some(s) => &surrogates[idx][s],
                    };
                    let (Some(f), Some(r)) = (
                        transfer_entropy_lag(&w.x, y, lag),
                        transfer_entropy_lag(y, &w.x, lag),
                    ) else {
                        continue;
                    };
                    (f, r)
                }
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
    };
    let mut surr_max = f64::NEG_INFINITY;
    let mut best: Option<(usize, f64)> = None;
    let mut best_thr = f64::NAN;
    let mut best_pos = 0usize;
    let mut best_tot = 0usize;
    let mut best_cells = f64::NAN;
    for lag in 0..=LAG_MAX {
        let (sum, pos, tot, cell_sum) = measure(lag, None);
        let d = if tot > 0 { sum / tot as f64 } else { f64::NAN };
        let mut surr_vals: Vec<f64> = Vec::new();
        for s in 0..N_SURR {
            let (ss, _, stot, _) = measure(lag, Some(s));
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

fn block_rank(from_kind: &str, to_kind: &str) -> usize {
    match (from_kind, to_kind) {
        ("aia", "aia") => 0,
        ("aia", "xrs") => 1,
        ("xrs", "aia") => 2,
        _ => 3,
    }
}

fn block_label(from_kind: &str, to_kind: &str) -> &'static str {
    match (from_kind, to_kind) {
        ("aia", "aia") => "intra-AIA",
        ("aia", "xrs") => "AIA -> XRS",
        ("xrs", "aia") => "XRS -> AIA",
        _ => "XRS-internal",
    }
}

fn verdict_of(r: &Row, min_events: usize, fam: f64) -> &'static str {
    if r.n_ev < min_events || !r.d.is_finite() {
        "no-statement"
    } else if fam.is_finite() && r.d > fam {
        "ARROW"
    } else if r.thr.is_finite() && r.d > r.thr {
        "family bound"
    } else {
        "still"
    }
}

fn sig_s(v: f64) -> String {
    if v.is_finite() {
        format!("{:+.4e}", v)
    } else {
        "-".to_string()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let aia_dir = match arg_value(&args, "--aia-dir") {
        Some(d) => d,
        None => AIA_DIR.to_string(),
    };
    let goes_dirs = [
        match arg_value(&args, "--goes-dir-2013") {
            Some(d) => d,
            None => GOES13_DIR.to_string(),
        },
        match arg_value(&args, "--goes-dir-2014") {
            Some(d) => d,
            None => GOES14_DIR.to_string(),
        },
        match arg_value(&args, "--goes-dir-2015") {
            Some(d) => d,
            None => GOES15_DIR.to_string(),
        },
    ];
    let window_cells = match arg_value(&args, "--window") {
        Some(w) => match w.parse::<usize>() {
            Ok(x) => x,
            Err(_) => WINDOW_CELLS_BASE,
        },
        None => WINDOW_CELLS_BASE,
    };
    let min_events = match arg_value(&args, "--min-events") {
        Some(m) => match m.parse::<usize>() {
            Ok(x) => x,
            Err(_) => MIN_EVENTS_FLOOR,
        },
        None => MIN_EVENTS_FLOOR,
    };
    let min_cells = MIN_CELLS.min(2 * window_cells + 1);

    let year_cells = ((YEAR_UNIX[1] - YEAR_UNIX[0]) / DT) as usize;
    let n_cells = 3 * year_cells;

    println!("=== Solar 24-s all-actor flare-stacked TE matrix over 2013-2015 ===");
    println!(
        "Window (unix {:.0} .. {:.0}): {} x 24-s cells over the three years ({} cells per year).",
        WINDOW_LO, WINDOW_HI, n_cells, year_cells
    );
    println!(
        "Actors (9): 7 AIA EUV bands (J2000 TT count, unix = t + {:.0} - {:.1}) - kind aia; GOES XRSA (a_flux, 0.05-0.4 nm) and XRSB (b_flux, 0.1-0.8 nm) - kind xrs - from the 2-s xr_YYYYMMDD.nc files; their time dataset carries unix seconds (measured: xr_20130101.nc opens at 1356998400.011). F10.7, Lyman-alpha and OMNI Bz/Density live at daily/hourly cadence - not seconds-resolvable - excluded.",
        J2000_UNIX_OFFSET, TT_UNIX
    );
    println!(
        "Flare events: XRSB b_flux as 24-s medians over the C1.0 threshold {:.0e} W/m2, refractory {} cells (30 min).",
        FLARE_THRESH, REFRACTORY
    );
    println!(
        "Event window: peak cell +-{} cells (+-40 min); a pair draws an event when >= {} aligned window cells carry both actors.",
        window_cells, min_cells
    );
    println!(
        "Direction 'A -> B' reads 'A drives B': TE runs with B as target and A as driver; per-event D = (TE(A->B) - TE(B->A)) / (|TE(A->B)| + |TE(B->A)|); the pair stacks the mean of per-event D over its events; pos counts events with D > 0."
    );
    println!(
        "Verdicts: ARROW (D > fam) | family bound (D > the pair's own mean+2 sigma over {} surrogates at its best lag and <= fam) | still | no-statement (< {} events).",
        N_SURR, min_events
    );
    println!(
        "fam = strongest surrogate stacked D over the 72 directed pairs x {} lags x {} surrogates; lag in 24-s cells (0 .. {} s).",
        LAG_MAX + 1, N_SURR, LAG_MAX * 24
    );
    println!();

    let mut aia_grids: Vec<Vec<Option<f32>>> = (0..7).map(|_| vec![None; n_cells]).collect();
    for (yi, &year_num) in YEAR_NUMBER.iter().enumerate() {
        let records = load_year_aia(&aia_dir, year_num);
        if records.is_empty() {
            eprintln!("year {} AIA corpus reads void", year_num);
        }
        for (bi, &(bidx, _)) in AIA_BANDS.iter().enumerate() {
            let series: Vec<(f64, f64)> = records
                .iter()
                .filter(|r| r.2 == bidx)
                .map(|r| (r.0 + J2000_UNIX_OFFSET - TT_UNIX, r.1))
                .collect();
            if series.is_empty() {
                eprintln!("year {} band {} absent in the corpus", year_num, bidx);
                continue;
            }
            let yr_grid = bin_median(&series, YEAR_UNIX[yi], year_cells);
            place(&mut aia_grids[bi], &yr_grid, yi * year_cells);
        }
    }

    let mut xrsa_grid: Vec<Option<f32>> = vec![None; n_cells];
    let mut xrsb_grid: Vec<Option<f32>> = vec![None; n_cells];
    for (yi, dir) in goes_dirs.iter().enumerate() {
        let series = dir_xrs_series(dir, "b_flux", "b_flags");
        if series.is_empty() {
            eprintln!(
                "{} carries no b_flux - that year's flare record stays unmeasured",
                dir
            );
        } else {
            let yr_grid = bin_median(&series, YEAR_UNIX[yi], year_cells);
            place(&mut xrsb_grid, &yr_grid, yi * year_cells);
        }
    }
    for (yi, dir) in goes_dirs.iter().enumerate() {
        let series = dir_xrs_series(dir, "a_flux", "a_flags");
        if series.is_empty() {
            eprintln!(
                "{} carries no a_flux - that year's XRSA channel stays unmeasured",
                dir
            );
        } else {
            let yr_grid = bin_median(&series, YEAR_UNIX[yi], year_cells);
            place(&mut xrsa_grid, &yr_grid, yi * year_cells);
        }
    }

    let mut cells: Vec<Vec<Option<f32>>> = aia_grids;
    cells.push(xrsa_grid);
    cells.push(xrsb_grid);

    let mut names: Vec<&str> = AIA_BANDS.iter().map(|&(_, n)| n).collect();
    names.push("XRSA");
    names.push("XRSB");
    let mut kinds: Vec<&str> = vec!["aia"; 7];
    kinds.push("xrs");
    kinds.push("xrs");

    println!("=== Actor board (24-s cells over the window) ===");
    for (i, name) in names.iter().enumerate() {
        let filled = cells[i].iter().filter(|c| c.is_some()).count();
        let first = cells[i].iter().position(|c| c.is_some());
        let last = cells[i].iter().rposition(|c| c.is_some());
        match (first, last) {
            (Some(f), Some(l)) => println!(
                "{:<6} | {:<4} | {:>9} filled | unix {:.0} .. {:.0}",
                name,
                kinds[i],
                filled,
                WINDOW_LO + f as f64 * DT,
                WINDOW_LO + l as f64 * DT
            ),
            _ => println!("{:<6} | {:<4} | absent over the window", name, kinds[i]),
        }
    }

    let events = cut_events(&cells[8], FLARE_THRESH as f32, REFRACTORY);
    let mut counts: [usize; 3] = [0, 0, 0];
    for &peak in &events {
        let t = WINDOW_LO + peak as f64 * DT;
        counts[year_of(t)] += 1;
    }
    println!();
    println!(
        "=== Flare events (XRSB b_flux 24-s median > {:.0e} W/m2) ===",
        FLARE_THRESH
    );
    for (yi, &year_num) in YEAR_NUMBER.iter().enumerate() {
        println!("{}: {} events", year_num, counts[yi]);
    }
    println!("total: {} events", events.len());
    if events.is_empty() {
        println!();
        println!("The event stack stays empty - no flare cell over the threshold (0 honored).");
        return;
    }

    let n_actors = cells.len();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..n_actors {
        for j in 0..n_actors {
            if i != j {
                pairs.push((i, j));
            }
        }
    }
    let sel: Option<(usize, usize)> = match arg_value(&args, "--pairs") {
        Some(spec) => match spec.split_once(':') {
            Some((a, b)) => match (a.trim().parse::<usize>(), b.trim().parse::<usize>()) {
                (Ok(von), Ok(bis)) if von <= bis && bis < pairs.len() => Some((von, bis)),
                _ => {
                    eprintln!(
                        "--pairs von:bis needs 0 <= von <= bis < {} directed pairs",
                        pairs.len()
                    );
                    std::process::exit(2);
                }
            },
            None => {
                eprintln!("--pairs carries no von:bis");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let selected: Vec<(usize, usize)> = match sel {
        Some((von, bis)) => pairs[von..=bis].to_vec(),
        None => pairs.clone(),
    };
    let n_threads = std::thread::available_parallelism()
        .map(|v| v.get())
        .unwrap_or(4)
        .min(16)
        .min(selected.len());
    let chunk = selected.len().div_ceil(n_threads);
    let gpu = ScalarTeGpu::new(LAG_MAX + 1).map(Mutex::new);
    match &gpu {
        Some(_) => println!("Scalar TE path: WebGPU device present - the KDE runs on the GPU."),
        None => println!("Scalar TE path: no WebGPU device - the KDE stays on the CPU."),
    }
    let gpu_ref: Option<&Mutex<ScalarTeGpu>> = gpu.as_ref();
    let names_ref = &names;
    let mut rows: Vec<Row> = std::thread::scope(|scope| {
        let handles: Vec<_> = selected
            .chunks(chunk)
            .map(|chunk_pairs| {
                let cells_ref = &cells;
                let events_ref = &events;
                let names_c = names_ref;
                scope.spawn(move || {
                    let mut out = Vec::with_capacity(chunk_pairs.len());
                    for &(fi, ti) in chunk_pairs {
                        let r = row_for(
                            cells_ref,
                            events_ref,
                            fi,
                            ti,
                            window_cells,
                            min_cells,
                            gpu_ref,
                        );
                        eprintln!(
                            "row {:>6} -> {:<6} complete: {} events",
                            names_c[fi], names_c[ti], r.n_ev
                        );
                        out.push(r);
                    }
                    out
                })
            })
            .collect();
        let mut out = Vec::new();
        for h in handles {
            if let Ok(v) = h.join() {
                out.extend(v);
            }
        }
        out
    });
    rows.sort_by_key(|r| (block_rank(kinds[r.from], kinds[r.to]), r.from, r.to));

    if let Some((von, bis)) = sel {
        let mut sm = f64::NEG_INFINITY;
        for r in &rows {
            if r.surr_max.is_finite() && r.surr_max > sm {
                sm = r.surr_max;
            }
        }
        for r in &rows {
            let cell_s = if r.n_ev > 0 {
                format!("{:.1}", r.cells_mean)
            } else {
                "-".to_string()
            };
            let lag_s = if r.n_ev > 0 {
                format!("{}", r.best_lag)
            } else {
                "-".to_string()
            };
            println!(
                "ROW {} {} {} {} {} {} {} {}",
                r.from,
                r.to,
                r.n_ev,
                cell_s,
                lag_s,
                sig_s(r.d),
                sig_s(r.thr),
                r.pos
            );
        }
        println!("SURRM_MAX {}", sig_s(sm));
        eprintln!(
            "sonde complete: pairs {}..={} of {}",
            von,
            bis,
            pairs.len() - 1
        );
        return;
    }

    let mut fam = f64::NEG_INFINITY;
    for r in &rows {
        if r.surr_max.is_finite() && r.surr_max > fam {
            fam = r.surr_max;
        }
    }
    println!();
    if fam.is_finite() {
        println!(
            "fam = {:.4e} over {} directed pairs x {} lags x {} surrogates.",
            fam,
            pairs.len(),
            LAG_MAX + 1,
            N_SURR
        );
    } else {
        println!("fam stays undefined - no surrogate draw over the round (0 honored).");
    }
    println!();
    println!("=== The 9 x 9 event-wise matrix (stacked per-event D, best lag in 24-s cells) ===");
    let mut last_block: Option<usize> = None;
    for r in &rows {
        let block = block_rank(kinds[r.from], kinds[r.to]);
        if last_block != Some(block) {
            println!();
            println!("--- {} ---", block_label(kinds[r.from], kinds[r.to]));
            last_block = Some(block);
        }
        let cell_s = if r.n_ev > 0 {
            format!("{:.1}", r.cells_mean)
        } else {
            "-".to_string()
        };
        let lag_s = if r.n_ev > 0 {
            format!("{}", r.best_lag)
        } else {
            "-".to_string()
        };
        println!(
            "{:>8} -> {:<8} | n_ev {:>4} | {:>6} cells/ev | lag {:>2} | D {:>11} | thr {:>11} | pos {:>4}/{:>4} | {}",
            names[r.from],
            names[r.to],
            r.n_ev,
            cell_s,
            lag_s,
            sig_s(r.d),
            sig_s(r.thr),
            r.pos,
            r.n_ev,
            verdict_of(r, min_events, fam)
        );
    }
    println!();

    let mut fam_counts: [usize; 4] = [0, 0, 0, 0];
    let mut intra_counts: [usize; 4] = [0, 0, 0, 0];
    let mut cross_counts: [usize; 4] = [0, 0, 0, 0];
    let mut xrs_counts: [usize; 4] = [0, 0, 0, 0];
    for r in &rows {
        let v = match verdict_of(r, min_events, fam) {
            "ARROW" => 0,
            "family bound" => 1,
            "still" => 2,
            _ => 3,
        };
        fam_counts[v] += 1;
        let block = block_rank(kinds[r.from], kinds[r.to]);
        match block {
            0 => intra_counts[v] += 1,
            3 => xrs_counts[v] += 1,
            _ => cross_counts[v] += 1,
        }
    }
    let label = |c: [usize; 4]| {
        format!(
            "ARROW {} | family bound {} | still {} | no-statement {}",
            c[0], c[1], c[2], c[3]
        )
    };
    println!("=== Relationship blocks (fam is the full-matrix surrogate bound) ===");
    println!("intra-AIA (42 directed pairs): {}", label(intra_counts));
    println!("AIA-XRS (28 directed pairs): {}", label(cross_counts));
    println!("XRS-internal (2 directed pairs): {}", label(xrs_counts));
    println!("whole matrix (72 directed pairs): {}", label(fam_counts));
    println!();

    let arrows: Vec<&Row> = rows
        .iter()
        .filter(|r| r.d > fam && r.n_ev >= min_events)
        .collect();
    if arrows.is_empty() {
        println!(
            "No stacked D clears the full-round family bound fam - silence is a finding (0 honored)."
        );
    } else {
        for r in arrows {
            println!(
                "{} -> {} (lag {} cells = {} s, D {:.4e} > fam {:.4e}, {} events, pos {}/{})",
                names[r.from],
                names[r.to],
                r.best_lag,
                r.best_lag * 24,
                r.d,
                fam,
                r.n_ev,
                r.pos,
                r.n_ev
            );
        }
    }
    println!();
    println!(
        "Lag in 24-s cells (0..{} s); D > 0 means the first actor drives the second within the flare window; the cascade windows are GOES b_flux flare windows.",
        LAG_MAX * 24
    );
}
