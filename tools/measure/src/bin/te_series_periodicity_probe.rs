use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::omni2::{
    parse_bin, COMP_AE, COMP_BX, COMP_BY, COMP_BZ, COMP_DST, COMP_N1800, COMP_SYMH, COMP_V1800,
};
use omegaflow::lsk::days_from_civil;

const OMNI2_1H_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie_1h.bin";
const OMNI2_INDICES_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_indices.bin";
const HOUR: f64 = 3600.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const CARRINGTON_DAY: f64 = 27.2753;
const PERIODIC_FAP_GATE: f64 = 1e-3;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn year_epoch(y: i64) -> Option<f64> {
    days_from_civil(y, 1, 1).map(|d| d as f64 * 86400.0)
}

fn load_solar_wind() -> Option<Vec<(f64, f64, u32)>> {
    let cache = omegaflow::archivar::cache_root()
        .join("omni2_serie_1h.bin")
        .to_string_lossy()
        .into_owned();
    let bytes = match std::fs::read(&cache) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("omni2_serie_1h.bin absent locally — fetching the CDN asset");
            fetch_raw_bytes(OMNI2_1H_CDN, 3600)?
        }
    };
    parse_bin(&bytes)
}

fn load_indices() -> Option<Vec<(f64, f64, u32)>> {
    let cache = omegaflow::archivar::cache_root()
        .join("omni2_indices.bin")
        .to_string_lossy()
        .into_owned();
    let bytes = match std::fs::read(&cache) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("omni2_indices.bin absent locally — fetching the CDN asset");
            fetch_raw_bytes(OMNI2_INDICES_CDN, 3600)?
        }
    };
    parse_bin(&bytes)
}

fn channel(recs: &[(f64, f64, u32)], comp: u32, lo: f64, hi: f64, shift: f64) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = recs
        .iter()
        .filter(|&&(t, _, c)| {
            let u = t + shift;
            c == comp && u >= lo && u < hi
        })
        .map(|&(t, v, _)| (t + shift, v))
        .collect();
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn bin_cells(series: &[(f64, f64)], t0: f64, dt: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / dt).floor();
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

fn lomb_scan(t: &[f64], r: &[f32], fmin: f64) -> Option<(f64, f64, f64, u32)> {
    let n = t.len();
    if n < 8 {
        return None;
    }
    let x: Vec<f64> = r.iter().map(|&v| v as f64).collect();
    let mean = x.iter().sum::<f64>() / n as f64;
    let var = x.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n as f64;
    if var <= 0.0 {
        return None;
    }
    let span = t[n - 1] - t[0];
    if span <= 0.0 {
        return None;
    }
    let sample = span / n as f64;
    if sample <= 0.0 {
        return None;
    }
    let fmax = 1.0 / (2.0 * sample);
    if fmax < fmin {
        return None;
    }
    let mut best_z = 0.0f64;
    let mut best_f = fmin;
    let mut nf = 0u32;
    let mut f = fmin;
    while f <= fmax {
        let w = 2.0 * std::f64::consts::PI * f;
        let mut c2 = 0.0f64;
        let mut s2 = 0.0f64;
        for &ti in t {
            c2 += (2.0 * w * ti).cos();
            s2 += (2.0 * w * ti).sin();
        }
        let tau = 0.5 * s2.atan2(c2) / w;
        let mut num_c = 0.0f64;
        let mut num_s = 0.0f64;
        let mut den_c = 0.0f64;
        let mut den_s = 0.0f64;
        for (i, &ti) in t.iter().enumerate() {
            let xv = x[i] - mean;
            let ph = w * (ti - tau);
            num_c += xv * ph.cos();
            num_s += xv * ph.sin();
            den_c += ph.cos() * ph.cos();
            den_s += ph.sin() * ph.sin();
        }
        if den_c > 1e-12 && den_s > 1e-12 {
            let z = 0.5 * (num_c * num_c / den_c + num_s * num_s / den_s) / var;
            if z > best_z {
                best_z = z;
                best_f = f;
            }
        }
        nf += 1;
        f *= 1.05;
    }
    if nf == 0 {
        return None;
    }
    let n_indep = nf as f64;
    let fap = 1.0 - (1.0 - (-best_z).exp()).powf(n_indep);
    Some((best_f, best_z, fap, nf))
}

fn scan_peak_z(t: &[f64], x: &[f64], mean: f64, var: f64, f: f64) -> f64 {
    let w = 2.0 * std::f64::consts::PI * f;
    let mut c2 = 0.0f64;
    let mut s2 = 0.0f64;
    for &ti in t {
        c2 += (2.0 * w * ti).cos();
        s2 += (2.0 * w * ti).sin();
    }
    let tau = 0.5 * s2.atan2(c2) / w;
    let mut num_c = 0.0f64;
    let mut num_s = 0.0f64;
    let mut den_c = 0.0f64;
    let mut den_s = 0.0f64;
    for (i, &ti) in t.iter().enumerate() {
        let xv = x[i] - mean;
        let ph = w * (ti - tau);
        num_c += xv * ph.cos();
        num_s += xv * ph.sin();
        den_c += ph.cos() * ph.cos();
        den_s += ph.sin() * ph.sin();
    }
    if den_c > 1e-12 && den_s > 1e-12 {
        0.5 * (num_c * num_c / den_c + num_s * num_s / den_s) / var
    } else {
        0.0
    }
}

fn present_pairs(cells: &[Option<f32>], t0: f64) -> (Vec<f64>, Vec<f32>) {
    let mut t = Vec::new();
    let mut v = Vec::new();
    for (i, c) in cells.iter().enumerate() {
        if let Some(x) = c {
            t.push(t0 + i as f64 * HOUR);
            v.push(*x);
        }
    }
    (t, v)
}

fn report_series(name: &str, cells: &[Option<f32>], t0: f64) {
    let n_present = cells.iter().filter(|c| c.is_some()).count();
    if n_present < 128 {
        println!(
            "{name}: {n_present} hourly cells — below the n-floor, the series stays unmeasured"
        );
        return;
    }
    let (t_all, v_all) = present_pairs(cells, t0);
    let mut t = Vec::with_capacity(t_all.len() / 3);
    let mut v = Vec::with_capacity(t_all.len() / 3);
    for i in (0..t_all.len()).step_by(3) {
        t.push(t_all[i]);
        v.push(v_all[i]);
    }
    let span = t[t.len() - 1] - t[0];
    let span_d = span / 86400.0;
    let Some((f_win, z_win, fap, nf)) = lomb_scan(&t, &v, 10.0 / span) else {
        println!("{name}: no finite folding band — the series stays unmeasured");
        return;
    };
    let x: Vec<f64> = v.iter().map(|&s| s as f64).collect();
    let mean = x.iter().sum::<f64>() / x.len() as f64;
    let var = x.iter().map(|&s| (s - mean) * (s - mean)).sum::<f64>() / x.len() as f64;
    let mut f_best = f_win;
    let mut z_best = z_win;
    if var > 0.0 {
        let mut f = f_win / 1.01;
        let hi = f_win * 1.01;
        while f <= hi {
            let z = scan_peak_z(&t, &x, mean, var, f);
            if z > z_best {
                z_best = z;
                f_best = f;
            }
            f *= 1.001;
        }
    }
    let p_h = 1.0 / f_best / 3600.0;
    let p_d = p_h / 24.0;
    let cycles = span_d / p_d.max(1e-9);
    let periodic = fap < PERIODIC_FAP_GATE;
    println!(
        "{name}: {n_present} hourly cells, span {span_d:.0} d — folding-band dominant period {p_h:.1} h = {p_d:.2} d, Z {z_best:.2}, FAP {fap:.2e} over {nf} freqs, {cycles:.1} cycles in span"
    );
    if periodic {
        let rotation = ((p_d / CARRINGTON_DAY) - 1.0).abs() < 0.1;
        let diurnal = (p_d - 1.0).abs() < 0.02;
        if rotation {
            println!(
                "  carries a real periodic folding at K = {p_d:.2} d ({p_h:.1} h) — the measured K lies within 10 % of the Carrington rotation (27.28 d)"
            );
        } else if diurnal {
            println!(
                "  carries a real periodic folding at K = {p_d:.2} d ({p_h:.1} h) — the measured K is the diurnal"
            );
        } else {
            println!("  carries a real periodic folding at K = {p_d:.2} d ({p_h:.1} h)");
        }
    } else {
        println!("  no periodic folding above the gate (FAP {fap:.2e} >= {PERIODIC_FAP_GATE})");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let start_year: i64 = arg_value(&args, "--window-start")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2015);
    let end_year: i64 = arg_value(&args, "--window-end")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2026);
    if start_year >= end_year {
        eprintln!("--window-start/--window-end carry no valid span");
        return;
    }
    let lo = match year_epoch(start_year) {
        Some(v) => v,
        None => {
            eprintln!("--window-start {} carries no civil year", start_year);
            return;
        }
    };
    let hi = match year_epoch(end_year) {
        Some(v) => v,
        None => {
            eprintln!("--window-end {} carries no civil year", end_year);
            return;
        }
    };

    println!("=== TE-series periodicity probe — the cycle-length measurement ===");
    println!(
        "window {}..{} (hourly), channels V n Bz |B| AE Dst SYM-H; folding domain 6 h .. span/10 (>= 10 cycles), 3-h subsampling, Lomb-Scargle FAP gate {}",
        start_year, end_year, PERIODIC_FAP_GATE
    );

    let Some(sw) = load_solar_wind() else {
        eprintln!("solar wind carries no records — the run stays unmeasured");
        return;
    };
    let Some(idx) = load_indices() else {
        eprintln!("indices carry no records — the run stays unmeasured");
        return;
    };

    let v = channel(&sw, COMP_V1800, lo, hi, J2000_UNIX_OFFSET);
    let n = channel(&sw, COMP_N1800, lo, hi, J2000_UNIX_OFFSET);
    let bz = channel(&sw, COMP_BZ, lo, hi, J2000_UNIX_OFFSET);
    let bx = channel(&sw, COMP_BX, lo, hi, J2000_UNIX_OFFSET);
    let by = channel(&sw, COMP_BY, lo, hi, J2000_UNIX_OFFSET);
    let ae = channel(&idx, COMP_AE, lo, hi, 0.0);
    let dst = channel(&idx, COMP_DST, lo, hi, 0.0);
    let symh = channel(&idx, COMP_SYMH, lo, hi, 0.0);

    let channels: [&[(f64, f64)]; 8] = [&v, &n, &bz, &bx, &by, &ae, &dst, &symh];
    let grid_lo = channels
        .iter()
        .filter_map(|c| c.first().map(|&(t, _)| t))
        .fold(f64::NEG_INFINITY, f64::max);
    let grid_hi = channels
        .iter()
        .filter_map(|c| c.last().map(|&(t, _)| t))
        .fold(f64::INFINITY, f64::min);
    if grid_lo >= grid_hi {
        eprintln!("common window empty — the series stay unmeasured");
        return;
    }
    let t0 = (grid_lo / HOUR).floor() * HOUR;
    let n_cells = ((grid_hi - t0) / HOUR).floor() as usize;

    let bc_v = bin_cells(&v, t0, HOUR, n_cells);
    let bc_n = bin_cells(&n, t0, HOUR, n_cells);
    let bc_bz = bin_cells(&bz, t0, HOUR, n_cells);
    let bc_bx = bin_cells(&bx, t0, HOUR, n_cells);
    let bc_by = bin_cells(&by, t0, HOUR, n_cells);
    let bc_ae = bin_cells(&ae, t0, HOUR, n_cells);
    let bc_dst = bin_cells(&dst, t0, HOUR, n_cells);
    let bc_symh = bin_cells(&symh, t0, HOUR, n_cells);

    let bc_bmag: Vec<Option<f32>> = (0..n_cells)
        .map(|i| match (bc_bx[i], bc_by[i], bc_bz[i]) {
            (Some(x), Some(y), Some(z)) => Some((x * x + y * y + z * z).sqrt()),
            _ => None,
        })
        .collect();

    report_series("V", &bc_v, t0);
    report_series("n", &bc_n, t0);
    report_series("Bz", &bc_bz, t0);
    report_series("|B|", &bc_bmag, t0);
    report_series("AE", &bc_ae, t0);
    report_series("Dst", &bc_dst, t0);
    report_series("SYM-H", &bc_symh, t0);
}
