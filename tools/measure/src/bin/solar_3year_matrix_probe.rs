use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::euvs::{self, COMP_LYA1216};
use omegaflow::archivar::f107;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::goes::{self, COMP_XRSA, COMP_XRSB};
use omegaflow::archivar::omni2::{self, COMP_BZ, COMP_N1800};
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};

const GOES_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_xrs.bin";
const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const OMNI2_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie.bin";
const OMNI2_1H_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie_1h.bin";
const EUVS_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_euvs.bin";
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DAY: f64 = 86400.0;
const HOUR: f64 = 3600.0;
const LAGS_DAY: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const LAGS_HOUR: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const MIN_N: usize = 30;
const N_SURR: usize = 10;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const TT_UNIX: f64 = 32.184;

const WINDOW_LO: f64 = 1356998400.0;
const WINDOW_HI: f64 = 1451606400.0;

fn year_bounds(year: u32) -> (f64, f64) {
    match year {
        2013 => (1356998400.0, 1388534400.0),
        2014 => (1388534400.0, 1420070400.0),
        2015 => (1420070400.0, 1451606400.0),
        _ => (WINDOW_LO, WINDOW_HI),
    }
}

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

fn pair_cells(a: &[Option<f32>], b: &[Option<f32>]) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (ca, cb) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (ca, cb) {
            xs.push(*x);
            ys.push(*y);
        }
    }
    (xs, ys)
}

fn surrogate_te_values(to: &[f32], from: &[f32], lag: usize, seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut vals = Vec::new();
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_lag(to, &ys, lag) {
            vals.push(te);
        }
    }
    vals
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
    series: Vec<(f64, f64)>,
    kind: &'static str,
}

fn read_aia_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != AIA_MAGIC {
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

fn load_aia(year: u32) -> Vec<(f64, f64, u32)> {
    let mut all = Vec::new();
    if year == 2014 {
        for m in 1..=12 {
            let p = format!("data/jsoc.stanford.edu/aia2014_{:02}.bin", m);
            all.extend(read_aia_lines(&p));
        }
    } else {
        let p = format!("data/jsoc.stanford.edu/aia{}_fullyear.bin", year);
        all.extend(read_aia_lines(&p));
    }
    all
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let grain = match arg_value(&args, "--grain") {
        Some(g) => g,
        None => "daily".to_string(),
    };
    let cell = match grain.as_str() {
        "hourly" => HOUR,
        _ => DAY,
    };
    let (window_lo, window_hi) = match arg_value(&args, "--year") {
        Some(y) => match y.parse::<u32>() {
            Ok(yr) => year_bounds(yr),
            Err(_) => (WINDOW_LO, WINDOW_HI),
        },
        None => (WINDOW_LO, WINDOW_HI),
    };
    let n_cells = ((window_hi - window_lo) / cell).floor() as usize;
    let lags: &[usize] = if grain == "hourly" {
        &LAGS_HOUR
    } else {
        &LAGS_DAY
    };
    let lag_unit = if grain == "hourly" { "h" } else { "d" };

    println!(
        "=== Solar 3-year all-actor TE matrix (Nadel III) — grain {} ===",
        grain
    );
    println!(
        "Window: (unix {:.0}..{:.0}), {} cells.",
        window_lo, window_hi, n_cells
    );
    println!("Direction: 'A -> B' reads 'A drives B' (TE computed with B's cells as target, A as driver).");
    println!(
        "fam = max surrogate TE over all directed pairs x {} lags x {} surrogates.",
        lags.len(),
        N_SURR
    );

    let goes_bytes = load_bin("goes_xrs.bin", GOES_CDN, arg_value(&args, "--goes-bin"));
    let f107_bytes = load_bin(
        "f107_penticton.bin",
        F107_CDN,
        arg_value(&args, "--f107-bin"),
    );
    let omni2_bytes = load_bin(
        "omni2_serie.bin",
        OMNI2_CDN,
        arg_value(&args, "--omni2-bin"),
    );
    let omni2_1h_bytes = if grain == "hourly" {
        load_bin(
            "omni2_serie_1h.bin",
            OMNI2_1H_CDN,
            arg_value(&args, "--omni2-1h-bin"),
        )
    } else {
        None
    };
    let euvs_bytes = load_bin("goes_euvs.bin", EUVS_CDN, arg_value(&args, "--euvs-bin"));

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
    let f107_records = match &f107_bytes {
        Some(b) => match f107::parse_bin(b) {
            Some(r) => r,
            None => Vec::new(),
        },
        None => Vec::new(),
    };
    let omni2_records = match &omni2_bytes {
        Some(b) => match omni2::parse_bin(b) {
            Some(r) => r,
            None => Vec::new(),
        },
        None => Vec::new(),
    };
    let omni2_1h_records = match &omni2_1h_bytes {
        Some(b) => match omni2::parse_bin(b) {
            Some(r) => r,
            None => Vec::new(),
        },
        None => Vec::new(),
    };
    let euvs_records = match &euvs_bytes {
        Some(b) => match euvs::parse_bin(b) {
            Some(r) => r,
            None => Vec::new(),
        },
        None => Vec::new(),
    };

    let mut actors: Vec<Actor> = Vec::new();

    if grain != "hourly" {
        actors.push(Actor {
            name: "F10.7",
            series: f107_records
                .iter()
                .map(|&(d, v)| (d as f64 * DAY, v))
                .filter(|&(t, _)| t >= window_lo && t < window_hi)
                .collect(),
            kind: "historical",
        });
        actors.push(Actor {
            name: "Lya1216",
            series: euvs_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_LYA1216)
                .filter_map(|&(t, v, _)| lsk.tdb_to_unix(t).map(|u| (u, v)))
                .filter(|&(t, _)| t >= window_lo && t < window_hi)
                .collect(),
            kind: "historical",
        });
    }

    actors.push(Actor {
        name: "XRSA",
        series: goes_records
            .iter()
            .filter(|(_, _, c)| *c == COMP_XRSA)
            .filter_map(|&(t, v, _)| lsk.tdb_to_unix(t).map(|u| (u, v)))
            .filter(|&(t, _)| t >= window_lo && t < window_hi)
            .collect(),
        kind: "historical",
    });
    actors.push(Actor {
        name: "XRSB",
        series: goes_records
            .iter()
            .filter(|(_, _, c)| *c == COMP_XRSB)
            .filter_map(|&(t, v, _)| lsk.tdb_to_unix(t).map(|u| (u, v)))
            .filter(|&(t, _)| t >= window_lo && t < window_hi)
            .collect(),
        kind: "historical",
    });
    let omni_series = if grain == "hourly" {
        &omni2_1h_records
    } else {
        &omni2_records
    };
    for (name, comp, kind) in [("Bz", COMP_BZ, "L1"), ("Density", COMP_N1800, "L1")] {
        actors.push(Actor {
            name,
            series: omni_series
                .iter()
                .filter(|(_, _, c)| *c == comp)
                .map(|&(t, v, _)| (t, v))
                .filter(|&(t, _)| t >= window_lo && t < window_hi)
                .collect(),
            kind,
        });
    }

    let mut aia_all: Vec<(f64, f64, u32)> = Vec::new();
    let single_year: Option<u32> = arg_value(&args, "--year")
        .and_then(|y| y.parse::<u32>().ok())
        .filter(|y| matches!(y, 2013 | 2014 | 2015));
    match single_year {
        Some(year) => aia_all.extend(load_aia(year)),
        None => {
            for year in [2013u32, 2014, 2015] {
                aia_all.extend(load_aia(year));
            }
        }
    }
    for &(bidx, bname) in AIA_BANDS.iter() {
        actors.push(Actor {
            name: bname,
            series: aia_all
                .iter()
                .filter(|(_, _, i)| *i == bidx)
                .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET - TT_UNIX, v))
                .filter(|&(t, _)| t >= window_lo && t < window_hi)
                .collect(),
            kind: "aia",
        });
    }

    println!();
    println!("=== Channel board (grain {}) ===", grain);
    for a in &actors {
        match (a.series.first(), a.series.last()) {
            (Some(&(t0v, _)), Some(&(t1v, _))) => println!(
                "{:<9} | {:<10} | n {:<10} | {:.0} d .. {:.0} d",
                a.name,
                a.kind,
                a.series.len(),
                t0v / DAY,
                t1v / DAY
            ),
            _ => println!(
                "{:<9} | {:<10} | n 0 — absent over the window",
                a.name, a.kind
            ),
        }
    }

    let cells: Vec<Vec<Option<f32>>> = actors
        .iter()
        .map(|a| bin_mean(&a.series, window_lo, n_cells, cell))
        .collect();

    let n_actors = actors.len();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..n_actors {
        for j in 0..n_actors {
            if i != j {
                pairs.push((i, j));
            }
        }
    }

    struct Row {
        fi: usize,
        ti: usize,
        n: usize,
        best_lag: usize,
        te: f64,
        thr: f64,
        surr_max: f64,
    }
    fn row_for(cells: &[Vec<Option<f32>>], fi: usize, ti: usize, lags: &[usize]) -> Option<Row> {
        let (xs, ys) = pair_cells(&cells[ti], &cells[fi]);
        if xs.len() < MIN_N {
            return None;
        }
        let mut best: Option<(usize, f64)> = None;
        let mut best_thr = f64::NAN;
        let mut surr_max = f64::NEG_INFINITY;
        for &lag in lags {
            let seed = SURROGATE_SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let surr = surrogate_te_values(&xs, &ys, lag, seed);
            for &v in &surr {
                if v > surr_max {
                    surr_max = v;
                }
            }
            if let Some(te) = transfer_entropy_lag(&xs, &ys, lag) {
                if best.map_or(true, |(_, b)| te > b) {
                    best = Some((lag, te));
                    best_thr = mean_plus_2sigma(&surr).unwrap_or(f64::NAN);
                }
            }
        }
        let (best_lag, te) = best?;
        Some(Row {
            fi,
            ti,
            n: xs.len(),
            best_lag,
            te,
            thr: best_thr,
            surr_max,
        })
    }

    let n_threads = std::thread::available_parallelism()
        .map(|v| v.get())
        .unwrap_or(4)
        .min(16)
        .min(pairs.len());
    let chunk = pairs.len().div_ceil(n_threads);
    let rows: Vec<Option<Row>> = std::thread::scope(|scope| {
        let handles: Vec<_> = pairs
            .chunks(chunk)
            .map(|chunk_pairs| {
                let cells_ref = &cells;
                let lags_ref = lags;
                scope.spawn(move || {
                    chunk_pairs
                        .iter()
                        .map(|&(fi, ti)| row_for(cells_ref, fi, ti, lags_ref))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| match h.join() {
                Ok(v) => v,
                Err(_) => Vec::new(),
            })
            .collect()
    });

    let mut fam = f64::NEG_INFINITY;
    let mut cross_fam = f64::NEG_INFINITY;
    for row in rows.iter().flatten() {
        if row.surr_max > fam {
            fam = row.surr_max;
        }
        if actors[row.fi].kind != actors[row.ti].kind && row.surr_max > cross_fam {
            cross_fam = row.surr_max;
        }
    }

    println!();
    println!(
        "fam (full matrix) = {:.4e} over {} directed pairs x {} lags.",
        fam,
        pairs.len(),
        lags.len()
    );
    println!(
        "cross-block fam (historical/AIA + L1) = {:.4e} — the guard for the cross-block channel DAG.",
        cross_fam
    );
    println!();
    println!(
        "=== The {} x {} matrix (grain {}) ===",
        n_actors, n_actors, grain
    );
    for row in rows.iter().flatten() {
        let word = if row.te > fam {
            "ARROW"
        } else if row.te > row.thr {
            "family bound"
        } else {
            "still"
        };
        let cross = if actors[row.fi].kind != actors[row.ti].kind {
            let cw = if row.te > cross_fam {
                "cb-arrow"
            } else {
                "cb-still"
            };
            format!(" [{}]", cw)
        } else {
            String::new()
        };
        println!(
            "{:>9} -> {:<9} | n {:>5} | lag {:<2} {} | TE {:>10.4e} | thr {:>10.4e} | {}{}",
            actors[row.fi].name,
            actors[row.ti].name,
            row.n,
            row.best_lag,
            lag_unit,
            row.te,
            row.thr,
            word,
            cross
        );
    }
}
