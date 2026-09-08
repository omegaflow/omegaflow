use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::te::{surrogate_stats_phase, transfer_entropy_lag};

const MAGIC: [u8; 4] = *b"AIA1";
const DT: f64 = 24.0;
const WINDOW: usize = 100;
const REFRACTORY: usize = 75;
const FLARE_THRESH: f64 = 5e-6;
const FILL: f64 = -9999.0;
const LAGS: [usize; 3] = [0, 4, 8];
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

const LADDER: [(u32, &str, f64); 7] = [
    (5, "304A", 4.70),
    (1, "131A", 5.57),
    (2, "171A", 5.81),
    (3, "193A", 6.15),
    (4, "211A", 6.27),
    (6, "335A", 6.43),
    (0, "94A", 6.81),
];

const CHANNELS: [(usize, &str); 2] = [(0, "304"), (1, "131")];
const CONFOUNDERS: [(usize, &str); 6] = [
    (2, "171"),
    (3, "193"),
    (4, "211"),
    (5, "335"),
    (6, "94"),
    (7, "goes"),
];

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

fn pearson(a: &[f32], b: &[f32]) -> Option<f64> {
    let n = a.len().min(b.len());
    if n < 8 {
        return None;
    }
    let mut sa = 0.0f64;
    let mut sb = 0.0f64;
    for i in 0..n {
        sa += a[i] as f64;
        sb += b[i] as f64;
    }
    let ma = sa / n as f64;
    let mb = sb / n as f64;
    let mut cov = 0.0f64;
    let mut va = 0.0f64;
    let mut vb = 0.0f64;
    for i in 0..n {
        let da = a[i] as f64 - ma;
        let db = b[i] as f64 - mb;
        cov += da * db;
        va += da * da;
        vb += db * db;
    }
    if va <= 0.0 || vb <= 0.0 {
        return None;
    }
    Some(cov / (va * vb).sqrt())
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
        let t0 = records
            .iter()
            .map(|&(t, _, _)| t)
            .fold(f64::INFINITY, f64::min);
        let t1 = records
            .iter()
            .map(|&(t, _, _)| t)
            .fold(f64::NEG_INFINITY, f64::max);
        let bins = ((t1 - t0) / DT).floor() as usize;
        let mut grid: Vec<Vec<Option<f32>>> = Vec::new();
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
        "stack: {} events; cross-information matrix C <-> Y, C in {{171,193,211,335,94,goes}}, Y in {{304,131}}",
        events.len()
    );
    println!(
        "TE(C->Y) measured as the driver C informing the target Y beyond its own past; null = mean+2sd over 10 phase-randomized surrogates of the driver."
    );
    println!();

    let n_ch = CHANNELS.len();
    let n_co = CONFOUNDERS.len();
    let mut cy_sum = vec![vec![vec![0.0f64; LAGS.len()]; n_co]; n_ch];
    let mut yc_sum = vec![vec![vec![0.0f64; LAGS.len()]; n_co]; n_ch];
    let mut cnt = vec![vec![vec![0usize; LAGS.len()]; n_co]; n_ch];
    let mut cy_arrow = vec![vec![vec![0usize; LAGS.len()]; n_co]; n_ch];
    let mut yc_arrow = vec![vec![vec![0usize; LAGS.len()]; n_co]; n_ch];
    let mut corr_sum = vec![vec![0.0f64; n_co]; n_ch];
    let mut corr_cnt = vec![vec![0usize; n_co]; n_ch];

    for (ei, ev) in events.iter().enumerate() {
        if ei % 50 == 0 {
            println!("progress {} / {}", ei, events.len());
        }
        for (yi, _yn) in CHANNELS.iter().enumerate() {
            let y = &ev.lines[CHANNELS[yi].0];
            for (ci, _cn) in CONFOUNDERS.iter().enumerate() {
                let c = &ev.lines[CONFOUNDERS[ci].0];
                if let Some(r) = pearson(y, c) {
                    corr_sum[yi][ci] += r;
                    corr_cnt[yi][ci] += 1;
                }
                for (lagi, &lag) in LAGS.iter().enumerate() {
                    let seed = SEED
                        ^ (ei as u64).wrapping_mul(0x9E37_79B9)
                        ^ (ci as u64).wrapping_mul(0x85EB_CA6B)
                        ^ (yi as u64).wrapping_mul(0xC2B2_AE35)
                        ^ (lag as u64).wrapping_mul(0x27D4_EB2F);
                    let (Some(fwd), Some(rev)) = (
                        transfer_entropy_lag(y, c, lag),
                        transfer_entropy_lag(c, y, lag),
                    ) else {
                        continue;
                    };
                    let (Some(thr_fwd), Some(thr_rev)) = (
                        surrogate_stats_phase(y, c, lag, seed).map(|t| t.2),
                        surrogate_stats_phase(c, y, lag, seed).map(|t| t.2),
                    ) else {
                        continue;
                    };
                    cy_sum[yi][ci][lagi] += fwd;
                    yc_sum[yi][ci][lagi] += rev;
                    cnt[yi][ci][lagi] += 1;
                    if fwd > thr_fwd {
                        cy_arrow[yi][ci][lagi] += 1;
                    }
                    if rev > thr_rev {
                        yc_arrow[yi][ci][lagi] += 1;
                    }
                }
            }
        }
    }

    println!();
    println!("C -> Y   | lag | mean TE(C->Y) | mean TE(Y->C) | C->Y/ev | Y->C/ev | r(C,Y)");
    for (ci, (_, cn)) in CONFOUNDERS.iter().enumerate() {
        for (yi, (_, yn)) in CHANNELS.iter().enumerate() {
            let label = format!("{}->{}", cn, yn);
            let r = if corr_cnt[yi][ci] > 0 {
                corr_sum[yi][ci] / corr_cnt[yi][ci] as f64
            } else {
                f64::NAN
            };
            for (lagi, &lag) in LAGS.iter().enumerate() {
                if cnt[yi][ci][lagi] == 0 {
                    println!("{:>7} | {:>3}s | absent", label, lag * 24);
                    continue;
                }
                let n = cnt[yi][ci][lagi];
                let mf = cy_sum[yi][ci][lagi] / n as f64;
                let mr = yc_sum[yi][ci][lagi] / n as f64;
                println!(
                    "{:>7} | {:>3}s | {:>12.3e} | {:>12.3e} | {:>3}/{:<4} | {:>3}/{:<4} | {:+.3}",
                    label,
                    lag * 24,
                    mf,
                    mr,
                    cy_arrow[yi][ci][lagi],
                    n,
                    yc_arrow[yi][ci][lagi],
                    n,
                    r
                );
            }
        }
    }
    println!();
    println!(
        "mean TE(C->Y) > mean TE(Y->C) with C->Y/ev high = the confounder leads the cool channel; a confounder that both leads the channels and collapses 304->131 on conditioning carries the shared driver."
    );
}
