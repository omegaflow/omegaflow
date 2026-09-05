use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag_h};

const MAGIC: [u8; 4] = *b"EVL1";
const DT: f64 = 10.0;
const WINDOW: usize = 120;
const THRESH_FACTOR: f64 = 1.3;
const REFRACTORY: usize = 180;

const LINES: [(u32, &str, f64); 11] = [
    (23, "584A", 4.16),
    (11, "304A", 4.70),
    (36, "977A", 4.84),
    (38, "1032A", 5.47),
    (1, "131A", 5.57),
    (3, "171A", 5.81),
    (6, "195A", 6.13),
    (8, "211A", 6.27),
    (10, "284A", 6.30),
    (12, "335A", 6.43),
    (0, "94A", 6.81),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_eve_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{} reads void", path);
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        eprintln!("{} carries no EVL1 contract", path);
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
    factor: f64,
) -> (f64, usize, usize) {
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut sum = 0.0;
    let mut pos = 0usize;
    let mut tot = 0usize;
    for ev in events {
        let cool_use: Vec<f32>;
        let cool = if shuffle {
            cool_use = phase_randomized_surrogate(&ev.lines[ci], &mut rng);
            &cool_use
        } else {
            &ev.lines[ci]
        };
        let hot = &ev.lines[hi];
        let (Some(f), Some(r)) = (
            transfer_entropy_lag_h(hot, cool, lag, factor),
            transfer_entropy_lag_h(cool, hot, lag, factor),
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
    (sum / tot.max(1) as f64, pos, tot)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = arg_value(&args, "--eve") else {
        eprintln!("--eve <eve_lines.bin> absent");
        return;
    };
    let factor = arg_value(&args, "--h")
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|f| *f > 0.0)
        .unwrap_or(1.0);
    let n_surr = arg_value(&args, "--surr")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|n| *n >= 1)
        .unwrap_or(10);
    let records = read_eve_lines(&path);
    let mut series: Vec<(f64, f64)> = Vec::new();
    for (idx, _name, _logt) in LINES {
        let s: Vec<(f64, f64)> = records
            .iter()
            .filter(|(_, _, i)| *i == idx)
            .map(|&(t, v, _)| (t, v))
            .collect();
        if s.is_empty() {
            eprintln!("Linie {} absent im Bin", idx);
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
    for (idx, _name, _logt) in LINES {
        let s: Vec<(f64, f64)> = records
            .iter()
            .filter(|(_, _, i)| *i == idx)
            .map(|&(t, v, _)| (t, v))
            .collect();
        grid.push(bin_median(&s, t0, bins));
    }
    let dio: Vec<(f64, f64)> = records
        .iter()
        .filter(|(_, _, i)| *i == 100)
        .map(|&(t, v, _)| (t, v))
        .collect();
    let trig = bin_median(&dio, t0, bins);
    let n = grid[0].len();
    let mut trig_vals: Vec<f32> = trig.iter().filter_map(|&o| o).collect();
    let Some(bg) = median_f32(&trig_vals) else {
        eprintln!("quad diode carries no cells");
        return;
    };
    trig_vals.clear();
    let threshold = bg * THRESH_FACTOR as f32;

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
        let mut lines = vec![Vec::new(); LINES.len()];
        for k in lo..hi {
            let mut all = true;
            for (li, g) in grid.iter().enumerate() {
                match g[k] {
                    Some(v) => lines[li].push(v),
                    None => {
                        all = false;
                        break;
                    }
                }
            }
            if !all {
                for l in lines.iter_mut() {
                    l.clear();
                }
                break;
            }
        }
        if lines[0].len() >= 100 {
            events.push(Event { lines });
        }
        i = j.max(i + REFRACTORY);
    }

    println!(
        "{} events (quad diode 0,1–7 nm > {:.1}× median = {:.2e}), window ±20 min, 10-s cells",
        events.len(),
        THRESH_FACTOR,
        threshold
    );
    println!();
    println!(
        "The ladder (cool → hot): 584 → 304 → 977 → 1032 → 131 → 171 → 195 → 211 → 284 → 335 → 94 Å"
    );
    println!(
        "D(lag) = TE(cool→hot) − TE(hot→cool), positive = flux up the ladder. * = over the phase null."
    );
    println!();
    let mut real: Vec<Vec<f64>> = vec![vec![f64::NAN; 13]; LINES.len() - 1];
    let mut fam_pool: Vec<f64> = Vec::new();
    for p in 0..LINES.len() - 1 {
        for lag in 0..=12 {
            let (d, _, _) = stack_pair(&events, p, p + 1, lag, false, 0, factor);
            real[p][lag] = d;
            for s in 1..=n_surr {
                let (d_null, _, _) = stack_pair(
                    &events,
                    p,
                    p + 1,
                    lag,
                    true,
                    s as u64 * 0x9E37_79B9_7F4A_7C15,
                    factor,
                );
                fam_pool.push(d_null);
            }
        }
    }
    let fam = fam_pool.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    for p in 0..LINES.len() - 1 {
        let pair = format!("{}→{}", LINES[p].1, LINES[p + 1].1);
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
            .map(|(l, d)| format!("peak at lag {} ({} s) = {:.2e}", l, l * 10, d))
            .unwrap_or_default();
        println!(
            "{:>12} | lag:    0       2       4       6       8      10      12 | {}",
            pair, peak_s
        );
        println!(
            "{:>12} |      {:>6} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
            "", cells[0], cells[2], cells[4], cells[6], cells[8], cells[10], cells[12]
        );
    }
    println!();
    println!(
        "fam = {:.4e} — the strongest surrogate D of the whole round ({} pairs × 13 lags × {} surrogates, h × {}).",
        fam,
        LINES.len() - 1,
        n_surr,
        factor
    );
    println!("lag in 10-s cells (0..120 s); * = D over the full-round family bound fam.");
}
