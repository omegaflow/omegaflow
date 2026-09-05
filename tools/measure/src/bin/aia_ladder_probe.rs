use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};

const MAGIC: [u8; 4] = *b"AIA1";
const DT: f64 = 24.0;
const WINDOW: usize = 100;
const N_SURR: usize = 10;
const THRESH_FACTOR: f64 = 1.3;
const REFRACTORY: usize = 75;
const FLARE_THRESH: f64 = 5e-6;
const FILL: f64 = -9999.0;

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

fn median_f32(v: &[f32]) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    let m = if s.len() % 2 == 0 {
        (s[s.len() / 2 - 1] + s[s.len() / 2]) * 0.5
    } else {
        s[s.len() / 2]
    };
    Some(m)
}

struct Event {
    lines: Vec<Vec<f32>>,
}

fn stack_pair(
    events: &[Event],
    ci: usize,
    hi: usize,
    lag: usize,
    shuffle: bool,
    seed: u64,
) -> (f64, usize, usize) {
    let n_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(16)
        .min(events.len().max(1));
    let chunk_len = events.len().div_ceil(n_threads).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = events
            .chunks(chunk_len)
            .enumerate()
            .map(|(ci_chunk, chunk)| {
                let chunk_seed = seed.wrapping_add(ci_chunk as u64 * 0xD1B5_4A32_D192_ED03);
                scope.spawn(move || {
                    let mut sum = 0.0;
                    let mut pos = 0usize;
                    let mut tot = 0usize;
                    for (idx, ev) in chunk.iter().enumerate() {
                        let cool_use: Vec<f32>;
                        let cool = if shuffle {
                            let mut rng =
                                chunk_seed.wrapping_add(idx as u64 * 0x9E37_79B9_7F4A_7C15);
                            cool_use = phase_randomized_surrogate(&ev.lines[ci], &mut rng);
                            &cool_use
                        } else {
                            &ev.lines[ci]
                        };
                        let hot = &ev.lines[hi];
                        let (Some(f), Some(r)) = (
                            transfer_entropy_lag(cool, hot, lag),
                            transfer_entropy_lag(hot, cool, lag),
                        ) else {
                            continue;
                        };
                        let d = f - r;
                        let scale = (f.abs() + r.abs()).max(1e-12);
                        sum += d / scale;
                        tot += 1;
                        if d > 0.0 {
                            pos += 1;
                        }
                    }
                    (sum, pos, tot)
                })
            })
            .collect();
        let mut sum = 0.0;
        let mut pos = 0usize;
        let mut tot = 0usize;
        for handle in handles {
            if let Ok((s, p, t)) = handle.join() {
                sum += s;
                pos += p;
                tot += t;
            }
        }
        (sum / tot.max(1) as f64, pos, tot)
    })
}

fn cut_events(trig: &[Option<f32>], threshold: f32, grid: &[Vec<Option<f32>>]) -> Vec<Event> {
    let n = trig.len();
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
        let mut run: Vec<Vec<f32>> = vec![Vec::new(); LADDER.len()];
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
            run = vec![Vec::new(); LADDER.len()];
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
    let Some(path) = arg_value(&args, "--aia") else {
        eprintln!("--aia <aia_lines.bin> absent");
        return;
    };
    let records = read_aia_lines(&path);
    let mut series: Vec<(f64, f64)> = Vec::new();
    for (idx, _name, _logt) in LADDER {
        let s: Vec<(f64, f64)> = records
            .iter()
            .filter(|(_, _, i)| *i == idx)
            .map(|&(t, v, _)| (t, v))
            .collect();
        if s.is_empty() {
            eprintln!("band {} absent in the bin", idx);
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
    for (idx, _name, _logt) in LADDER {
        let s: Vec<(f64, f64)> = records
            .iter()
            .filter(|(_, _, i)| *i == idx)
            .map(|&(t, v, _)| (t, v))
            .collect();
        grid.push(bin_median(&s, t0, bins));
    }

    let goes_dir = arg_value(&args, "--goes-dir");
    let events = if let Some(dir) = goes_dir {
        let lsk = omegaflow::archivar::embedded_lsk();
        let mut b_flux: Vec<(f64, f64)> = Vec::new();
        let Ok(entries) = std::fs::read_dir(&dir) else {
            eprintln!("{} reads void", dir);
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
        let trig = bin_median(&b_flux, t0, bins);
        let events = cut_events(&trig, FLARE_THRESH as f32, &grid);
        println!(
            "{} events (GOES b_flux > {:.0e} W/m², C1.0), window ±40 min, 24-s cells",
            events.len(),
            FLARE_THRESH
        );
        events
    } else {
        let dio: Vec<(f64, f64)> = records
            .iter()
            .filter(|(_, _, i)| *i == 5)
            .map(|&(t, v, _)| (t, v))
            .collect();
        let trig = bin_median(&dio, t0, bins);
        let mut trig_vals: Vec<f32> = trig.iter().filter_map(|&o| o).collect();
        let Some(bg) = median_f32(&trig_vals) else {
            eprintln!("304-Å carries no cells");
            return;
        };
        trig_vals.clear();
        let threshold = bg * THRESH_FACTOR as f32;
        let events = cut_events(&trig, threshold, &grid);
        println!(
            "{} events (AIA-304 > {:.1}× median = {:.2e}), window ±40 min, 24-s cells",
            events.len(),
            THRESH_FACTOR,
            threshold
        );
        events
    };

    println!();
    println!("The AIA ladder (cool → hot): 304 → 131 → 171 → 193 → 211 → 335 → 94 Å");
    println!(
        "D(lag) = TE(cool→hot) − TE(hot→cool), positive = flux up the ladder. * = over the full-round family bound fam."
    );
    println!();
    let out_path = arg_value(&args, "--out");
    let done_pairs: Vec<String> = match &out_path {
        Some(p) => std::fs::read_to_string(p)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| {
                l.split(" | ")
                    .next()
                    .map(|f| f.trim().to_string())
                    .filter(|f| f.contains('→'))
            })
            .collect(),
        None => Vec::new(),
    };
    let n_pairs = LADDER.len() - 1;
    let mut real: Vec<Vec<f64>> = vec![vec![f64::NAN; 13]; n_pairs];
    let mut fam_pool: Vec<f64> = Vec::new();
    for p in 0..n_pairs {
        for lag in 0..=12 {
            let (d, _, _) = stack_pair(&events, p, p + 1, lag, false, 0);
            real[p][lag] = d;
            for s in 1..=N_SURR {
                let (d_null, _, _) = stack_pair(
                    &events,
                    p,
                    p + 1,
                    lag,
                    true,
                    s as u64 * 0x9E37_79B9_7F4A_7C15,
                );
                fam_pool.push(d_null);
            }
        }
    }
    let fam = fam_pool.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    for p in 0..n_pairs {
        let pair = format!("{}→{}", LADDER[p].1, LADDER[p + 1].1);
        if done_pairs.contains(&pair) {
            continue;
        }
        let mut cells: Vec<String> = Vec::new();
        let mut peak: Option<(usize, f64)> = None;
        for lag in 0..=12 {
            let d = real[p][lag];
            if d > peak.map_or(f64::NEG_INFINITY, |(_, b)| b) {
                peak = Some((lag, d));
            }
            let sig = if d > fam { "*" } else { " " };
            cells.push(format!("{:>8.2e}{}", d, sig));
        }
        let peak_s = peak
            .map(|(l, d)| format!("peak at lag {} ({} s) = {:.2e}", l, l * 24, d))
            .unwrap_or_default();
        let line_a = format!(
            "{:>12} | lag:    0       2       4       6       8      10      12 | {}",
            pair, peak_s
        );
        let line_b = format!(
            "{:>12} |      {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
            "", cells[0], cells[2], cells[4], cells[6], cells[8], cells[10], cells[12]
        );
        println!("{}", line_a);
        println!("{}", line_b);
        if let Some(path) = &out_path {
            let mut buf = String::new();
            if let Ok(existing) = std::fs::read_to_string(path) {
                buf.push_str(&existing);
            }
            buf.push_str(&line_a);
            buf.push('\n');
            buf.push_str(&line_b);
            buf.push('\n');
            let _ = std::fs::write(path, buf);
        }
    }
    if let Some(path) = &out_path {
        if let Ok(text) = std::fs::read_to_string(path) {
            for line in text.lines() {
                println!("{}", line);
            }
        }
    }
    println!();
    println!(
        "fam = {:.4e} — the strongest surrogate D of the whole round ({} pairs × 13 lags × {} surrogates).",
        fam,
        n_pairs,
        N_SURR
    );
    println!("lag in 24-s cells (0..288 s); * = D over the full-round family bound fam.");
}
