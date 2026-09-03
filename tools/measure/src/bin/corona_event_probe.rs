use omegaflow::hdf5::{Endian, Hdf5File, decode_f32, decode_f64};
use omegaflow::te::{gaussian, phase_randomized_surrogate, silverman};

const EPOCH_2000: f64 = 946728000.0;
const DT: f64 = 10.0;
const FILL: f64 = -9999.0;
const FLARE_THRESH: f64 = 5e-6;
const REFRACTORY: usize = 180;
const WINDOW: usize = 120;
const LAG_MAX: usize = 12;
const N_SURR: usize = 10;

fn series(
    path: &str,
    tname: &str,
    vname: &str,
    fname: &str,
    flag_bytes: usize,
    gc_name: Option<&str>,
    t_offset: f64,
) -> Vec<(f64, f64)> {
    let Ok(bytes) = std::fs::read(path) else {
        return Vec::new();
    };
    let Ok(file) = Hdf5File::parse(&bytes) else {
        return Vec::new();
    };
    let (Ok(t_raw), Ok(v_raw), Ok(f_raw)) = (
        file.read_dataset(tname),
        file.read_dataset(vname),
        file.read_dataset(fname),
    ) else {
        return Vec::new();
    };
    let gc_raw = gc_name.and_then(|g| file.read_dataset(g).ok());
    let n = t_raw.len() / 8;
    if v_raw.len() != n * 4 || f_raw.len() != n * flag_bytes {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let Some(t) = decode_f64(&t_raw, i * 8, Endian::Le) else {
            continue;
        };
        let flag: u16 = if flag_bytes == 2 {
            u16::from_le_bytes([f_raw[i * 2], f_raw[i * 2 + 1]])
        } else {
            f_raw[i] as u16
        };
        if flag != 0 {
            continue;
        }
        if let Some(g) = &gc_raw {
            if g[i] != 0 {
                continue;
            }
        }
        let Some(v) = decode_f32(&v_raw, i * 4, Endian::Le) else {
            continue;
        };
        let v = v as f64;
        if !v.is_finite() || v == FILL || v <= 0.0 {
            continue;
        }
        out.push((t + t_offset, v));
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

fn te_conditional(x: &[f32], y: &[f32], z: &[f32], lag: usize) -> Option<f64> {
    let n = x.len();
    if n < 8 {
        return None;
    }
    let (hx, hy, hz) = (silverman(x)?, silverman(y)?, silverman(z)?);
    let m = if lag == 0 { n - 1 } else { n - lag };
    if m < 8 {
        return None;
    }
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[if lag == 0 { t + 1 } else { t + lag }] as f64;
        let yt = y[t] as f64;
        let zt = z[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            let xf = x[if lag == 0 { s + 1 } else { s + lag }] as f64;
            k3 += gaussian(xt1 - xf, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - z[s] as f64, hz);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in 0..n {
            k1 += gaussian(xt - x[s] as f64, hx) * gaussian(zt - z[s] as f64, hz);
        }
        let p1 = k1 / n as f64;
        let mut k2xy = 0.0;
        for s in 0..n {
            k2xy += gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - z[s] as f64, hz);
        }
        let p2xy = k2xy / n as f64;
        let mut k2x = 0.0;
        for s in 0..m {
            let xf = x[if lag == 0 { s + 1 } else { s + lag }] as f64;
            k2x += gaussian(xt1 - xf, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(zt - z[s] as f64, hz);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

struct Event {
    a: Vec<f32>,
    b: Vec<f32>,
    e: Vec<f32>,
}

fn detect_events(a: &[Option<f32>], b: &[Option<f32>], e: &[Option<f32>]) -> Vec<Event> {
    let n = a.len();
    let mut events = Vec::new();
    let mut i = 0usize;
    while i < n {
        let Some(bv) = b[i] else {
            i += 1;
            continue;
        };
        if bv < FLARE_THRESH as f32 {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut peak = i;
        while j < n && j - i < REFRACTORY {
            if let Some(v) = b[j] {
                if v < FLARE_THRESH as f32 && j > i + 3 {
                    break;
                }
                if let Some(pv) = b[peak] {
                    if v > pv {
                        peak = j;
                    }
                }
            }
            j += 1;
        }
        let lo = peak.saturating_sub(WINDOW);
        let hi = (peak + WINDOW).min(n);
        let mut ev = Event {
            a: Vec::new(),
            b: Vec::new(),
            e: Vec::new(),
        };
        for k in lo..hi {
            if let (Some(av), Some(bv2), Some(ev2)) = (a[k], b[k], e[k]) {
                ev.a.push(av);
                ev.b.push(bv2);
                ev.e.push(ev2);
            }
        }
        if ev.a.len() >= 100 {
            events.push(ev);
        }
        i = j.max(i + REFRACTORY);
    }
    events
}

fn stack_direction(
    events: &[Event],
    lag: usize,
    shuffle: bool,
    rng_seed: u64,
) -> (f64, usize, usize) {
    let mut rng = rng_seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut sum = 0.0;
    let mut pos = 0usize;
    let mut tot = 0usize;
    for ev in events {
        let e_shuffled;
        let e_use: &[f32] = if shuffle {
            e_shuffled = phase_randomized_surrogate(&ev.e, &mut rng);
            &e_shuffled
        } else {
            &ev.e
        };
        let (Some(f), Some(r)) = (
            te_conditional(&ev.a, e_use, &ev.b, lag),
            te_conditional(e_use, &ev.a, &ev.b, lag),
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
    let dir = args
        .iter()
        .position(|a| a == "--dir")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let Some(dir) = dir else {
        eprintln!("--dir <ordner> absent");
        return;
    };
    let mut days = std::collections::BTreeSet::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with("xr_") && name.ends_with(".nc") {
                days.insert(name[3..11].to_string());
            }
        }
    }
    let mut events = Vec::new();
    for day in &days {
        let xr_path = format!("{}/xr_{}.nc", dir, day);
        let eu_path = format!("{}/eu_{}.nc", dir, day);
        let a_s = series(&xr_path, "time", "a_flux", "a_flags", 2, None, 0.0);
        let b_s = series(&xr_path, "time", "b_flux", "b_flags", 2, None, 0.0);
        let e_s = series(
            &eu_path,
            "time_chanE",
            "irr_1216_1nm",
            "irr_1216_flag",
            1,
            None,
            EPOCH_2000,
        );
        if a_s.is_empty() || b_s.is_empty() || e_s.is_empty() {
            continue;
        }
        let t0 = a_s
            .first()
            .map(|&(t, _)| t)
            .unwrap_or(0.0)
            .max(b_s.first().map(|&(t, _)| t).unwrap_or(0.0))
            .max(e_s.first().map(|&(t, _)| t).unwrap_or(0.0));
        let t1 = a_s
            .last()
            .map(|&(t, _)| t)
            .unwrap_or(0.0)
            .min(b_s.last().map(|&(t, _)| t).unwrap_or(0.0))
            .min(e_s.last().map(|&(t, _)| t).unwrap_or(0.0));
        let bins = ((t1 - t0) / DT).floor() as usize;
        let ab = bin_median(&a_s, t0, bins);
        let bb = bin_median(&b_s, t0, bins);
        let eb = bin_median(&e_s, t0, bins);
        events.extend(detect_events(&ab, &bb, &eb));
    }
    println!(
        "{} days | {} flare events (b_flux > 1e-6 W/m², window ±20 min, 10-s cells)",
        days.len(),
        events.len()
    );
    println!();
    println!(
        "{:>4} | {:>9} | {:>8} | {:>8} | {:>8}",
        "lag", "D(Lya→A)", "share>0", "n", "null-max"
    );
    for lag in 0..=LAG_MAX {
        let (d_real, pos, tot) = stack_direction(&events, lag, false, 0);
        let mut null_max = f64::NEG_INFINITY;
        for s in 1..=N_SURR {
            let (d_null, _, _) =
                stack_direction(&events, lag, true, s as u64 * 0x9E37_79B9_7F4A_7C15);
            if d_null > null_max {
                null_max = d_null;
            }
        }
        let sig = if d_real > null_max {
            "  <-- over null"
        } else {
            ""
        };
        println!(
            "{:>4} | {:>9.4e} | {:>7.1}% | {:>8} | {:>8.4e}{}",
            lag,
            d_real,
            pos as f64 / tot.max(1) as f64 * 100.0,
            tot,
            null_max,
            sig
        );
    }
    println!();
    println!(
        "lag in 10-s cells; lag 10 ≈ 100 s. D > 0 = chromosphere (Lyman-α) leads the hot X-ray; null-max = strongest D of the phase-randomized rounds (Lyman-α reordered per window)."
    );
}
