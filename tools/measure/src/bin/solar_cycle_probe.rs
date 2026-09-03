use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, embedded_lsk, fetch_raw_bytes, parse_ephemeris_binary,
};
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};
use std::collections::HashMap;

const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const JUPITER_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_jupiter.bin";
const SATURN_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_saturn.bin";
const GM_J: f64 = 1.26686534e17;
const GM_S: f64 = 3.7931187e16;
const DAY: f64 = 86400.0;
const STRIDE_DAYS: i64 = 30;
const LAG_MAX: usize = 24;
const N_SURR: usize = 10;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_f107(path: &str) -> Vec<(i64, f64)> {
    let Ok(bytes) = std::fs::read(path) else {
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != *b"F107" {
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 16;
        let d = i64::from_le_bytes(bytes[o..o + 8].try_into().unwrap());
        let v = f64::from_le_bytes(bytes[o + 8..o + 16].try_into().unwrap());
        out.push((d, v));
    }
    out
}

fn load_ephemerides(args: &[String]) -> HashMap<String, BodyEphemeris> {
    let mut eph = HashMap::new();
    for (name, url, arg) in [
        ("jupiter", JUPITER_CDN, "--jupiter-bin"),
        ("saturn", SATURN_CDN, "--saturn-bin"),
    ] {
        let bytes = match arg_value(args, arg) {
            Some(p) => std::fs::read(&p).ok(),
            None => fetch_raw_bytes(url, 3600),
        };
        if let Some(b) = bytes {
            if let Some(e) = parse_ephemeris_binary(&b) {
                eph.insert(name.to_string(), e);
            }
        }
    }
    eph
}

fn tidal_at(tdb: f64, eph: &HashMap<String, BodyEphemeris>) -> Option<f64> {
    let rj = body_barycenter_position("jupiter", tdb, eph)?;
    let rs = body_barycenter_position("saturn", tdb, eph)?;
    let dj = (rj[0] * rj[0] + rj[1] * rj[1] + rj[2] * rj[2]).sqrt();
    let ds = (rs[0] * rs[0] + rs[1] * rs[1] + rs[2] * rs[2]).sqrt();
    if dj <= 0.0 || ds <= 0.0 {
        return None;
    }
    Some(GM_J / (dj * dj * dj) + GM_S / (ds * ds * ds))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let f107_path = arg_value(&args, "--f107-bin")
        .unwrap_or_else(|| "/tmp/opencode/f107_penticton.bin".to_string());
    let mut f107 = read_f107(&f107_path);
    if f107.is_empty() {
        if let Some(bytes) = fetch_raw_bytes(F107_CDN, 3600) {
            if bytes.len() >= 8 && bytes[0..4] == *b"F107" {
                let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
                f107 = (0..n)
                    .filter_map(|i| {
                        let o = 8 + i * 16;
                        let d = i64::from_le_bytes(bytes[o..o + 8].try_into().ok()?);
                        let v = f64::from_le_bytes(bytes[o + 8..o + 16].try_into().ok()?);
                        Some((d, v))
                    })
                    .collect();
            }
        }
    }
    if f107.is_empty() {
        eprintln!("{}: F107 absent or carries no F107 contract", f107_path);
        return;
    }
    let eph = load_ephemerides(&args);
    if eph.len() < 2 {
        eprintln!("ephemerides absent (jupiter + saturn)");
        return;
    }
    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("the time base is absent");
            return;
        }
    };

    let d0 = f107.first().map(|&(d, _)| d).unwrap();
    let d1 = f107.last().map(|&(d, _)| d).unwrap();
    let grid: Vec<i64> = (d0..=d1).step_by(STRIDE_DAYS as usize).collect();

    let mut f107_bin = vec![Vec::<f64>::new(); grid.len()];
    for &(d, v) in &f107 {
        let idx = (d - d0) / STRIDE_DAYS;
        if idx >= 0 && (idx as usize) < grid.len() {
            f107_bin[idx as usize].push(v);
        }
    }
    let mut f_series: Vec<f32> = Vec::new();
    let mut t_series: Vec<f32> = Vec::new();
    for (i, &d) in grid.iter().enumerate() {
        if f107_bin[i].is_empty() {
            continue;
        }
        let fm = f107_bin[i].iter().sum::<f64>() / f107_bin[i].len() as f64;
        let unix = d as f64 * DAY + 43200.0;
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let Some(tv) = tidal_at(tdb, &eph) else {
            continue;
        };
        f_series.push(fm as f32);
        t_series.push(tv as f32);
    }
    println!(
        "monthly grid: {} cells over {} years ({}–{})",
        f_series.len(),
        (d1 - d0) as f64 / 365.25,
        d0,
        d1
    );

    let peaks = cycle_peaks(&f_series);
    println!("F10.7 cycle peaks (indices): {:?}", peaks);
    if peaks.len() >= 2 {
        let spacings: Vec<f64> = peaks
            .windows(2)
            .map(|w| (w[1] - w[0]) as f64 * STRIDE_DAYS as f64 / 365.25)
            .collect();
        println!(
            "peak spacings in years: {:?} — mean {:.2} a vs Jupiter 11,86 a",
            spacings
                .iter()
                .map(|s| format!("{:.1}", s))
                .collect::<Vec<_>>(),
            spacings.iter().sum::<f64>() / spacings.len() as f64
        );
    }

    println!();
    println!(
        "TE lag sweep (tides ↔ F10.7, monthly, lag 0..{} months):",
        LAG_MAX
    );
    println!(
        "{:>4} | {:>11} | {:>11} | {:>11}",
        "lag", "TE(T→F)", "TE(F→T)", "fam-so-far"
    );
    let mut fam = f64::NEG_INFINITY;
    let mut best: Option<(usize, f64)> = None;
    for lag in 0..=LAG_MAX {
        let seed = SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let tf = transfer_entropy_lag(&f_series, &t_series, lag).unwrap_or(f64::NAN);
        let ft = transfer_entropy_lag(&t_series, &f_series, lag).unwrap_or(f64::NAN);
        let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut vals = Vec::new();
        for _ in 0..N_SURR {
            let ys = phase_randomized_surrogate(&t_series, &mut rng);
            if let Some(v) = transfer_entropy_lag(&f_series, &ys, lag) {
                vals.push(v);
                if v > fam {
                    fam = v;
                }
            }
        }
        if tf > best.map_or(f64::NEG_INFINITY, |(_, b)| b) {
            best = Some((lag, tf));
        }
        println!(
            "{:>4} | {:>11.4e} | {:>11.4e} | {:>11.4e}",
            lag, tf, ft, fam
        );
    }
    println!();
    if let Some((l, v)) = best {
        let word = if v > fam { "ARROW (over fam)" } else { "still" };
        println!(
            "Peak TE(Tidal→F10.7) at lag {} months = {:.3} a, TE {:.4e} vs fam {:.4e} — {}",
            l,
            l as f64 / 12.0,
            v,
            fam,
            word
        );
    }
    println!(
        "fam = strongest surrogate TE of the round (multiple-comparison correction). 7 cycles are statistically thin — a still result is the honest answer here (0 honored)."
    );
}

fn cycle_peaks(f: &[f32]) -> Vec<usize> {
    let w = 36usize;
    let mut smooth = vec![0.0f64; f.len()];
    for i in 0..f.len() {
        let lo = i.saturating_sub(w);
        let hi = (i + w + 1).min(f.len());
        let mut s = 0.0;
        for j in lo..hi {
            s += f[j] as f64;
        }
        smooth[i] = s / (hi - lo) as f64;
    }
    let mut peaks = Vec::new();
    let mut last = -1000isize;
    for i in 1..f.len() - 1 {
        if smooth[i] > smooth[i - 1] && smooth[i] >= smooth[i + 1] && i as isize - last >= 96 {
            peaks.push(i);
            last = i as isize;
        }
    }
    peaks
}
