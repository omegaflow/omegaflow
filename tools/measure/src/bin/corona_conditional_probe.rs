use omegaflow::hdf5::{Endian, Hdf5File, decode_f32, decode_f64};
use omegaflow::te::{
    TeEstimator, TeNull, TeStats2Params, TeStatsParams, TeSurrogateParams,
    conditional_te_stats_lagged, conditional_te_stats_lagged_2, conditional_te_stats_lagged_n,
    conditional_te_surrogates_n, transfer_entropy_conditional_2,
    transfer_entropy_conditional_binned_n, transfer_entropy_conditional_h,
    transfer_entropy_ksg_conditional_n,
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
const KSG_K: usize = 4;

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

fn arg_values(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
                i += 1;
            }
        }
        i += 1;
    }
    out
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

enum ConfSpec {
    Fixed(Vec<usize>),
    AllBands(Vec<usize>),
}

fn ncond_for(spec: &ConfSpec) -> usize {
    match spec {
        ConfSpec::Fixed(v) => v.len(),
        ConfSpec::AllBands(extra) => 5 + extra.len(),
    }
}

fn pair_confounders(spec: &ConfSpec, li: usize) -> Option<Vec<usize>> {
    match spec {
        ConfSpec::Fixed(v) => {
            if v.iter().any(|&ci| ci == li || ci == li + 1) {
                return None;
            }
            Some(v.clone())
        }
        ConfSpec::AllBands(extra) => {
            let mut out: Vec<usize> = (0..7).filter(|&b| b != li && b != li + 1).collect();
            out.extend(extra.iter().copied());
            Some(out)
        }
    }
}

fn resolve_confounders(args: &[String]) -> Option<(ConfSpec, String)> {
    let mut names: Vec<String> = arg_values(args, "--confound");
    let confound2 = arg_value(args, "--confound2");
    if names.is_empty() {
        names.push("goes".to_string());
    }
    if let Some(c2) = confound2 {
        if names.len() < 2 {
            names.push(c2);
        } else {
            eprintln!("--confound2 {} superseded by the --confound list", c2);
        }
    }
    let all_bands = names.iter().any(|n| n == "all");
    let mut fixed: Vec<usize> = Vec::new();
    let mut extra: Vec<usize> = Vec::new();
    let mut shown: Vec<String> = Vec::new();
    for name in &names {
        if name == "all" {
            continue;
        }
        let idx = match confound_idx(name) {
            Some(i) => i,
            None => {
                eprintln!("--confound {} carries no band", name);
                return None;
            }
        };
        if all_bands {
            if idx >= LADDER.len() && !extra.contains(&idx) {
                extra.push(idx);
                shown.push(name.clone());
            }
        } else if !fixed.contains(&idx) {
            fixed.push(idx);
            shown.push(name.clone());
        }
    }
    let label = if all_bands {
        if shown.is_empty() {
            "all bands (5 per pair)".to_string()
        } else {
            format!("all bands (5 per pair) + {}", shown.join(" + "))
        }
    } else {
        shown.join(" + ")
    };
    if all_bands {
        Some((ConfSpec::AllBands(extra), label))
    } else {
        Some((ConfSpec::Fixed(fixed), label))
    }
}

fn bin_edges(v: &[f32]) -> Option<(f32, f32)> {
    let mut mn = f32::INFINITY;
    let mut mx = f32::NEG_INFINITY;
    for &x in v {
        if !x.is_finite() {
            return None;
        }
        mn = mn.min(x);
        mx = mx.max(x);
    }
    if mx <= mn {
        return None;
    }
    Some((mn, mx))
}

fn bin_index(v: f32, mn: f32, range: f32, bins: usize) -> usize {
    let mut b = (((v - mn) / range) * bins as f32) as usize;
    if b >= bins {
        b = bins - 1;
    }
    b
}

fn joint_key(indices: &[usize], bins: usize) -> u64 {
    let mut k = 0u64;
    let mut r = 1u64;
    for &i in indices {
        k += (i as u64) * r;
        r *= bins as u64;
    }
    k
}

fn occupied_joint_cells(
    x: &[f32],
    y: &[f32],
    conds: &[&[f32]],
    lag: usize,
    bins: usize,
) -> Option<(usize, usize)> {
    let n = x.len();
    if n < 8 || bins < 2 || y.len() < n {
        return None;
    }
    for c in conds {
        if c.len() < n {
            return None;
        }
    }
    let shift = if lag == 0 { 1usize } else { lag };
    let m = n.checked_sub(shift)?;
    if m < 8 {
        return None;
    }
    let (mn_x, mx_x) = bin_edges(x)?;
    let range_x = mx_x - mn_x;
    let (mn_y, mx_y) = bin_edges(y)?;
    let range_y = mx_y - mn_y;
    let mut cond_edges: Vec<(f32, f32)> = Vec::with_capacity(conds.len());
    for c in conds {
        let (mn, mx) = bin_edges(c)?;
        cond_edges.push((mn, mx - mn));
    }
    let bx: Vec<usize> = x
        .iter()
        .map(|&v| bin_index(v, mn_x, range_x, bins))
        .collect();
    let by: Vec<usize> = y
        .iter()
        .map(|&v| bin_index(v, mn_y, range_y, bins))
        .collect();
    let bcond: Vec<Vec<usize>> = conds
        .iter()
        .zip(&cond_edges)
        .map(|(c, &(mn, range))| c.iter().map(|&v| bin_index(v, mn, range, bins)).collect())
        .collect();
    let mut keybuf: Vec<usize> = Vec::with_capacity(conds.len() + 3);
    let mut seen: std::collections::HashSet<u64> = std::collections::HashSet::new();
    for s in 0..m {
        keybuf.clear();
        keybuf.push(bx[s + shift]);
        keybuf.push(bx[s]);
        keybuf.push(by[s]);
        for b in &bcond {
            keybuf.push(b[s]);
        }
        seen.insert(joint_key(&keybuf, bins));
    }
    Some((seen.len(), m))
}

fn surrogate_stats(vals: &[f64]) -> Option<(f64, f64, f64)> {
    if vals.len() < 2 {
        return None;
    }
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let sd = var.sqrt();
    Some((mean, sd, mean + 2.0 * sd))
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
    let h_factor: f64 = match arg_value(&args, "--h").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => 1.0,
    };
    let bins: usize = match arg_value(&args, "--bins").and_then(|s| s.parse().ok()) {
        Some(v) if v >= 2 => v,
        Some(_) => {
            eprintln!("--bins needs a value >= 2");
            return;
        }
        None => 4,
    };
    let max_events: usize = match arg_value(&args, "--max-events").and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => 0,
    };
    let (estimator, estimator_explicit) = match arg_value(&args, "--estimator").as_deref() {
        None => ("binned", false),
        Some("binned") => ("binned", true),
        Some("ksg") => ("ksg", true),
        Some(other) => {
            eprintln!("--estimator {} carries no name (binned|ksg)", other);
            return;
        }
    };
    let use_ksg = estimator == "ksg";
    let (spec, label) = match resolve_confounders(&args) {
        Some(s) => s,
        None => return,
    };
    let ncond = ncond_for(&spec);
    if estimator_explicit && ncond <= 2 {
        eprintln!("--estimator applies only beyond 2 confounders; the kde path stays");
    }

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
        let bins_year = ((t1 - t0) / DT).floor() as usize;
        let mut grid: Vec<Vec<Option<f32>>> = Vec::new();
        for (bidx, _name, _logt) in LADDER {
            let s: Vec<(f64, f64)> = records
                .iter()
                .filter(|(_, _, i)| *i == bidx)
                .map(|&(t, v, _)| (t, v))
                .collect();
            grid.push(bin_median(&s, t0, bins_year));
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
        let confound = bin_median(&b_flux, t0, bins_year);
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
    let mut events = all_events;
    if max_events > 0 && events.len() > max_events {
        println!(
            "max-events {}: first {} of {} events processed",
            max_events,
            max_events,
            events.len()
        );
        events.truncate(max_events);
    }
    let path_name = if ncond <= 1 {
        "kde-1"
    } else if ncond == 2 {
        "kde-2"
    } else if use_ksg {
        "ksg-n"
    } else {
        "binned-n"
    };
    println!("stack: {} events; confounders {}", events.len(), label);
    if ncond > 2 {
        println!(
            "path {}; bins {}; null max_lag {}",
            path_name, bins, max_lag
        );
    } else {
        println!("path {}; null max_lag {}", path_name, max_lag);
    }
    println!("conditional directional excess D|C = TE(cool->hot|C) - TE(hot->cool|C), lagged null");
    println!();

    let n_pairs = LADDER.len() - 1;
    let mut d_sum = vec![vec![0.0f64; LAGS.len()]; n_pairs];
    let mut fwd_sum = vec![vec![0.0f64; LAGS.len()]; n_pairs];
    let mut rev_sum = vec![vec![0.0f64; LAGS.len()]; n_pairs];
    let mut d_cnt = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut fwd_arrow = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut rev_arrow = vec![vec![0usize; LAGS.len()]; n_pairs];
    let mut occ_sum = 0usize;
    let mut m_sum = 0usize;
    let mut cell_calls = 0usize;

    for (ei, ev) in events.iter().enumerate() {
        if ei % 50 == 0 {
            println!("progress {} / {}", ei, events.len());
        }
        for li in 0..n_pairs {
            let Some(confs) = pair_confounders(&spec, li) else {
                continue;
            };
            let cool = &ev.lines[li];
            let hot = &ev.lines[li + 1];
            for (lagi, &lag) in LAGS.iter().enumerate() {
                let seed =
                    0x9E37_79B9_7F4A_7C15 ^ (li as u64 * 0x9E37_79B9) ^ (lag as u64 * 0x85EB_CA6B);
                let (te_fwd, te_rev, thr_fwd, thr_rev) = match confs.len() {
                    1 => {
                        let c = &ev.lines[confs[0]];
                        (
                            transfer_entropy_conditional_h(hot, cool, c, lag, h_factor),
                            transfer_entropy_conditional_h(cool, hot, c, lag, h_factor),
                            conditional_te_stats_lagged(hot, cool, c, lag, max_lag, seed, N_SURR)
                                .map(|t| t.2),
                            conditional_te_stats_lagged(cool, hot, c, lag, max_lag, seed, N_SURR)
                                .map(|t| t.2),
                        )
                    }
                    2 => {
                        let c = &ev.lines[confs[0]];
                        let c2 = &ev.lines[confs[1]];
                        (
                            transfer_entropy_conditional_2(hot, cool, c, c2, lag),
                            transfer_entropy_conditional_2(cool, hot, c, c2, lag),
                            conditional_te_stats_lagged_2(
                                hot,
                                cool,
                                c,
                                c2,
                                TeStats2Params {
                                    lag,
                                    max_lag,
                                    seed,
                                    n_surr: N_SURR,
                                },
                            )
                            .map(|t| t.2),
                            conditional_te_stats_lagged_2(
                                cool,
                                hot,
                                c,
                                c2,
                                TeStats2Params {
                                    lag,
                                    max_lag,
                                    seed,
                                    n_surr: N_SURR,
                                },
                            )
                            .map(|t| t.2),
                        )
                    }
                    _ => {
                        let conds: Vec<&[f32]> =
                            confs.iter().map(|&ci| ev.lines[ci].as_slice()).collect();
                        if let Some((occ, mm)) = occupied_joint_cells(hot, cool, &conds, lag, bins)
                        {
                            occ_sum += occ;
                            m_sum += mm;
                            cell_calls += 1;
                        }
                        if use_ksg {
                            (
                                transfer_entropy_ksg_conditional_n(hot, cool, &conds, lag, KSG_K),
                                transfer_entropy_ksg_conditional_n(cool, hot, &conds, lag, KSG_K),
                                conditional_te_surrogates_n(
                                    hot,
                                    cool,
                                    &conds,
                                    TeSurrogateParams {
                                        lag,
                                        max_lag,
                                        bins,
                                        seed,
                                        n_surr: N_SURR,
                                        null: TeNull::Residual,
                                        block: 0,
                                        est: TeEstimator::Ksg,
                                        k: KSG_K,
                                    },
                                )
                                .and_then(|v| surrogate_stats(&v))
                                .map(|t| t.2),
                                conditional_te_surrogates_n(
                                    cool,
                                    hot,
                                    &conds,
                                    TeSurrogateParams {
                                        lag,
                                        max_lag,
                                        bins,
                                        seed,
                                        n_surr: N_SURR,
                                        null: TeNull::Residual,
                                        block: 0,
                                        est: TeEstimator::Ksg,
                                        k: KSG_K,
                                    },
                                )
                                .and_then(|v| surrogate_stats(&v))
                                .map(|t| t.2),
                            )
                        } else {
                            (
                                transfer_entropy_conditional_binned_n(hot, cool, &conds, lag, bins),
                                transfer_entropy_conditional_binned_n(cool, hot, &conds, lag, bins),
                                conditional_te_stats_lagged_n(
                                    hot,
                                    cool,
                                    &conds,
                                    TeStatsParams {
                                        lag,
                                        max_lag,
                                        bins,
                                        seed,
                                        n_surr: N_SURR,
                                        null: TeNull::Residual,
                                    },
                                )
                                .map(|t| t.2),
                                conditional_te_stats_lagged_n(
                                    cool,
                                    hot,
                                    &conds,
                                    TeStatsParams {
                                        lag,
                                        max_lag,
                                        bins,
                                        seed,
                                        n_surr: N_SURR,
                                        null: TeNull::Residual,
                                    },
                                )
                                .map(|t| t.2),
                            )
                        }
                    }
                };
                let (Some(te_fwd), Some(te_rev)) = (te_fwd, te_rev) else {
                    continue;
                };
                d_sum[li][lagi] += te_fwd - te_rev;
                fwd_sum[li][lagi] += te_fwd;
                rev_sum[li][lagi] += te_rev;
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
    println!("pair      | lag |   mean D|C | TE c->h|C | TE h->c|C | fwd/events | rev/events");
    for li in 0..n_pairs {
        let pair = format!("{}->{}", LADDER[li].1, LADDER[li + 1].1);
        if pair_confounders(&spec, li).is_none() {
            println!("{:>9} | skipped (confounder is a member)", pair);
            continue;
        }
        for (lagi, &lag) in LAGS.iter().enumerate() {
            if d_cnt[li][lagi] == 0 {
                println!("{:>9} | {:>3}s | absent", pair, lag * 24);
                continue;
            }
            let mean = d_sum[li][lagi] / d_cnt[li][lagi] as f64;
            let mf = fwd_sum[li][lagi] / d_cnt[li][lagi] as f64;
            let mr = rev_sum[li][lagi] / d_cnt[li][lagi] as f64;
            println!(
                "{:>9} | {:>3}s | {:>10.2e} | {:>10.2e} | {:>10.2e} | {:>3}/{:<4} | {:>3}/{:<4}",
                pair,
                lag * 24,
                mean,
                mf,
                mr,
                fwd_arrow[li][lagi],
                d_cnt[li][lagi],
                rev_arrow[li][lagi],
                d_cnt[li][lagi]
            );
        }
    }
    if ncond > 2 {
        let dims = 3 + ncond;
        let space = (bins as u128).pow(dims as u32);
        println!();
        println!(
            "sparsity: {} occupied joint cells over {} samples in {} calls; cell space {}^{} = {}",
            occ_sum, m_sum, cell_calls, bins, dims, space
        );
        if m_sum > 0 && occ_sum == m_sum {
            println!(
                "every sample lands in its own joint cell — the binned estimator degenerates at bins {} with {} confounders",
                bins, ncond
            );
        }
    }
    println!();
    println!(
        "fwd = cool->hot conditional arrow (TE > lagged null), rev = hot->cool. TE c->h|C = mean TE(cool->hot|C); mean D|C > 0 = direction survives conditioning on the shared envelope."
    );
}
