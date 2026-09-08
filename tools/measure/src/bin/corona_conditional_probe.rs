use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::te::{
    conditional_te_stats_lagged, conditional_te_stats_lagged_2, transfer_entropy_conditional,
    transfer_entropy_conditional_2,
};

const MAGIC: [u8; 4] = *b"AIA1";
const DT: f64 = 24.0;
const WINDOW: usize = 100;
const N_SURR: usize = 10;
const REFRACTORY: usize = 75;
const FLARE_THRESH: f64 = 5e-6;
const FILL: f64 = -9999.0;
const LAGS: [usize; 3] = [0, 4, 8];
const C_IDX: usize = 7;

const LADDER: [(u32, &str, f64); 7] = [
    (5, "304A", 4.70),
    (1, "131A", 5.57),
    (2, "171A", 5.81),
    (3, "193A", 6.15),
    (4, "211A", 6.27),
    (6, "335A", 6.43),
    (0, "94A", 6.81),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn confound_idx(name: &str) -> Option<usize> {
    if name == "goes" {
        return Some(C_IDX);
    }
    let n = name.trim_end_matches(['A', 'a']);
    LADDER
        .iter()
        .position(|(_, band, _)| band.trim_end_matches('A') == n)
}

fn read_aia_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{} reads void", path);
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
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
        if !v.is_finite() || v == FILL || v <= 0.0 {
            continue;
        }
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
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

struct Event {
    lines: Vec<Vec<f32>>,
}

fn cut_events(trig: &[Option<f32>], threshold: f32, grid: &[Vec<Option<f32>>]) -> Vec<Event> {
    let n = trig.len();
    let n_series = grid.len();
    let mut events: Vec<Event> = Vec::new();
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
        while j < n && j - i < REFRACTORY {
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
        let lo = peak.saturating_sub(WINDOW);
        let hi = (peak + WINDOW).min(n);
        let mut best: Vec<Vec<f32>> = Vec::new();
        let mut run: Vec<Vec<f32>> = vec![Vec::new(); n_series];
        for k in lo..hi {
            let complete = grid.iter().all(|g| g[k].is_some());
            if complete {
                for (li, g) in grid.iter().enumerate() {
                    run[li].push(g[k].unwrap());
                }
                continue;
            }
            if run[0].len() > best.first().map_or(0, Vec::len) {
                best = run;
            }
            run = vec![Vec::new(); n_series];
        }
        if run[0].len() > best.first().map_or(0, Vec::len) {
            best = run;
        }
        if best.first().map_or(0, Vec::len) >= 100 {
            events.push(Event { lines: best });
        }
        i = j.max(i + REFRACTORY);
    }
    events
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let lsk = omegaflow::archivar::embedded_lsk();

    let mut positions: Vec<usize> = Vec::new();
    let mut idx = 0usize;
    while idx < args.len() {
        if args[idx] == "--year" {
            positions.push(idx);
            idx += 1;
        } else {
            idx += 1;
        }
    }
    if positions.is_empty() {
        eprintln!("--year <aia_lines.bin> <goes-dir> absent (repeat per year)");
        return;
    }
    let max_lag: usize = match arg_value(&args, "--max-lag").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => 8,
    };
    let confound_name = match arg_value(&args, "--confound") {
        Some(n) => n,
        None => "goes".to_string(),
    };
    let c_idx = match confound_idx(&confound_name) {
        Some(i) => i,
        None => {
            eprintln!("--confound {} carries no band", confound_name);
            return;
        }
    };
    let confound2_name = arg_value(&args, "--confound2");
    let c2_idx = match confound2_name.as_deref() {
        Some(name) => match confound_idx(name) {
            Some(i) => Some(i),
            None => {
                eprintln!("--confound2 {} carries no band", name);
                return;
            }
        },
        None => None,
    };

    let mut all_events: Vec<Event> = Vec::new();
    for (yi, &pos) in positions.iter().enumerate() {
        let path = match args.get(pos + 1) {
            Some(p) => p.clone(),
            None => {
                eprintln!("--year {}: bin path absent", yi);
                return;
            }
        };
        let goes_dir = match args.get(pos + 2) {
            Some(d) => d.clone(),
            None => {
                eprintln!("--year {}: goes-dir absent", yi);
                return;
            }
        };
        let records = read_aia_lines(&path);
        if records.is_empty() {
            eprintln!("year {}: {} reads void", yi, path);
            return;
        }
        let mut series: Vec<(f64, f64)> = Vec::new();
        for (bidx, _name, _logt) in LADDER {
            let s: Vec<(f64, f64)> = records
                .iter()
                .filter(|(_, _, i)| *i == bidx)
                .map(|&(t, v, _)| (t, v))
                .collect();
            if s.is_empty() {
                eprintln!("year {} band {} absent in the bin", yi, bidx);
                return;
            }
            series.extend(s);
        }
        let t0 = series.iter().map(|&(t, _)| t).fold(f64::INFINITY, f64::min);
        let t1 = series
            .iter()
            .map(|&(t, _)| t)
            .fold(f64::NEG_INFINITY, f64::max);
        let bins = ((t1 - t0) / DT).floor() as usize;
        let mut grid: Vec<Vec<Option<f32>>> = Vec::new();
        for (bidx, _name, _logt) in LADDER {
            let s: Vec<(f64, f64)> = records
                .iter()
                .filter(|(_, _, i)| *i == bidx)
                .map(|&(t, v, _)| (t, v))
                .collect();
            grid.push(bin_median(&s, t0, bins));
        }
        let mut b_flux: Vec<(f64, f64)> = Vec::new();
        let Ok(entries) = std::fs::read_dir(&goes_dir) else {
            eprintln!("{} reads void", goes_dir);
            return;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("xr_") && name.ends_with(".nc") {
                let raw = goes_b_flux(&entry.path().to_string_lossy());
                if let Some(lsk) = &lsk {
                    b_flux.extend(
                        raw.into_iter()
                            .filter_map(|(t, v)| lsk.unix_to_tdb(t).map(|t2| (t2, v))),
                    );
                } else {
                    b_flux.extend(raw);
                }
            }
        }
        b_flux.sort_by(|a, b| a.0.total_cmp(&b.0));
        let confound = bin_median(&b_flux, t0, bins);
        grid.push(confound.clone());
        let ev = cut_events(&confound, FLARE_THRESH as f32, &grid);
        let before = all_events.len();
        all_events.extend(ev);
        println!(
            "year {}: {} events (GOES b_flux > {:.0e} W/m2), window +/-40 min, 24-s cells",
            yi,
            all_events.len() - before,
            FLARE_THRESH
        );
    }
    let events = all_events;
    println!(
        "stack: {} events; confounder C = {}; null max_lag {}",
        events.len(),
        confound_name,
        max_lag
    );
    if let Some(name) = confound2_name.as_deref() {
        println!("second confounder C2 = {}", name);
    }
    println!("conditional directional excess D|C = TE(cool->hot|C) - TE(hot->cool|C), lagged null");
    println!();

    let n_pairs = LADDER.len() - 1;
    let mut d_sum = vec![vec![0.0f64; LAGS.len()]; n_pairs];
    let mut d_cnt = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut fwd_arrow = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut rev_arrow = vec![vec![0usize; LAGS.len()]; n_pairs];

    for (ei, ev) in events.iter().enumerate() {
        if ei % 50 == 0 {
            println!("progress {} / {}", ei, events.len());
        }
        for li in 0..n_pairs {
            if li == c_idx || li + 1 == c_idx {
                continue;
            }
            if let Some(ci2) = c2_idx {
                if li == ci2 || li + 1 == ci2 {
                    continue;
                }
            }
            let cool = &ev.lines[li];
            let hot = &ev.lines[li + 1];
            let c = &ev.lines[c_idx];
            for (lagi, &lag) in LAGS.iter().enumerate() {
                let seed =
                    0x9E37_79B9_7F4A_7C15 ^ (li as u64 * 0x9E37_79B9) ^ (lag as u64 * 0x85EB_CA6B);
                let (te_fwd, te_rev, thr_fwd, thr_rev) = match c2_idx {
                    Some(ci2) => {
                        let c2 = &ev.lines[ci2];
                        (
                            transfer_entropy_conditional_2(hot, cool, c, c2, lag),
                            transfer_entropy_conditional_2(cool, hot, c, c2, lag),
                            conditional_te_stats_lagged_2(
                                hot, cool, c, c2, lag, max_lag, seed, N_SURR,
                            )
                            .map(|t| t.2),
                            conditional_te_stats_lagged_2(
                                cool, hot, c, c2, lag, max_lag, seed, N_SURR,
                            )
                            .map(|t| t.2),
                        )
                    }
                    None => (
                        transfer_entropy_conditional(hot, cool, c, lag),
                        transfer_entropy_conditional(cool, hot, c, lag),
                        conditional_te_stats_lagged(hot, cool, c, lag, max_lag, seed, N_SURR)
                            .map(|t| t.2),
                        conditional_te_stats_lagged(cool, hot, c, lag, max_lag, seed, N_SURR)
                            .map(|t| t.2),
                    ),
                };
                let (Some(te_fwd), Some(te_rev)) = (te_fwd, te_rev) else {
                    continue;
                };
                d_sum[li][lagi] += te_fwd - te_rev;
                d_cnt[li][lagi] += 1;
                if let Some(thr) = thr_fwd {
                    if te_fwd > thr {
                        fwd_arrow[li][lagi] += 1;
                    }
                }
                if let Some(thr) = thr_rev {
                    if te_rev > thr {
                        rev_arrow[li][lagi] += 1;
                    }
                }
            }
        }
    }

    println!();
    println!("pair      | lag |   mean D|C | fwd/events | rev/events");
    for li in 0..n_pairs {
        let pair = format!("{}->{}", LADDER[li].1, LADDER[li + 1].1);
        if li == c_idx || li + 1 == c_idx {
            println!("{:>9} | skipped (confounder is a member)", pair);
            continue;
        }
        if let Some(ci2) = c2_idx {
            if li == ci2 || li + 1 == ci2 {
                println!("{:>9} | skipped (confounder2 is a member)", pair);
                continue;
            }
        }
        for (lagi, &lag) in LAGS.iter().enumerate() {
            if d_cnt[li][lagi] == 0 {
                println!("{:>9} | {:>3}s | absent", pair, lag * 24);
                continue;
            }
            let mean = d_sum[li][lagi] / d_cnt[li][lagi] as f64;
            println!(
                "{:>9} | {:>3}s | {:>10.2e} | {:>3}/{:<4} | {:>3}/{:<4}",
                pair,
                lag * 24,
                mean,
                fwd_arrow[li][lagi],
                d_cnt[li][lagi],
                rev_arrow[li][lagi],
                d_cnt[li][lagi]
            );
        }
    }
    println!();
    println!(
        "fwd = cool->hot conditional arrow (TE > lagged null), rev = hot->cool. mean D|C > 0 = direction survives conditioning on the shared X-ray envelope."
    );
}
