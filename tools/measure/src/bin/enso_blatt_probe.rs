use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::extract::geo_series_parse_bin;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_ERSSTV5, GeoRec};
use omegaflow::archivar::omni_hro::{COMP_IMF_BZ_GSM, parse_bin};
use omegaflow::archivar::usgs_comcat::{COMP_RATE, parse_bin as parse_comcat_bin};
use omegaflow::lsk::{LeapSeconds, days_from_civil};
use omegaflow::te::{
    BLATT_N_FLOOR, BlattPairSpec, conditional_embedded_te_phase, current_commit_sha,
    surrogate_max_phase_n, surrogate_stats_phase_n, transfer_entropy_lag, write_blatt_pair,
};
use std::collections::HashMap;

const OMNI_HRO_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/cdaweb.gsfc.nasa.gov/omni_hro_1min.bin";
const ERSSTV5_CDN: &str = "https://github.com/omegaflow/sources/releases/download/coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin";
const TAO_WND_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/data.pmel.noaa.gov/tao_wnd_zonal.csv";
const USGS_COMCAT_CDN: &str = "https://github.com/omegaflow/sources/releases/download/earthquake.usgs.gov/usgs_comcat_m45.bin";
const QBO_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/cpc.ncep.noaa.gov/qbo_30hpa.csv";
const D20_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/data.pmel.noaa.gov/d20_thermocline.csv";
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 100;
const MAX_LAG_MONTHS: usize = 12;
const MONTH_S: f64 = 2_592_000.0;
const MIN_PAIRED: usize = 30;
const CAL_MONTHS: usize = 12;
const CLIMATOLOGY_FLOOR: usize = 10;
const J2000_UNIX_OFFSET: f64 = 946_728_000.0;
const SECS_PER_DAY: f64 = 86_400.0;

const PC_N: usize = 256;
const PC_PLANTED_LAG: usize = 3;
const PC_COUPLING: f64 = 1.5;
const PC_AR: f64 = 0.5;
const PC_DRIVER_NOISE: f64 = 0.3;
const PC_TARGET_NOISE: f64 = 0.1;

const CH_WND: usize = 0;
const CH_QUAKE: usize = 1;
const CH_BZ: usize = 2;
const CH_SST: usize = 3;
const CH_QBO: usize = 4;
const CH_D20: usize = 5;
const CH_NAMES: [&str; 6] = ["Wnd", "Quake", "Bz", "SST", "QBO", "D20"];
const CH_MEDIA: [&str; 6] = ["atmos", "litho", "helios", "ocean", "atmos", "ocean"];

fn load_local_or_fetch(name: &str, url: &str) -> Option<Vec<u8>> {
    let cache = omegaflow::archivar::cache_root()
        .join(name)
        .to_string_lossy()
        .into_owned();
    if let Ok(b) = std::fs::read(&cache) {
        if !b.is_empty() {
            return Some(b);
        }
    }
    fetch_raw_bytes(url)
}

fn iso_to_tdb(lsk: &LeapSeconds, iso: &str) -> Option<f64> {
    let date = iso.get(0..10)?;
    let (y, rest) = date.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    let year: i64 = y.parse().ok()?;
    let month: i64 = m.parse().ok()?;
    let day: i64 = d.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let unix = days as f64 * SECS_PER_DAY;
    let leap = lsk.leap_at(unix)?;
    Some(unix + lsk.delta_t_a + leap - J2000_UNIX_OFFSET)
}

fn load_bz() -> Option<Vec<(f64, f64)>> {
    let bytes = load_local_or_fetch("omni_hro_1min.bin", OMNI_HRO_CDN)?;
    let recs = parse_bin(&bytes)?;
    let mut out: Vec<(f64, f64)> = recs
        .into_iter()
        .filter(|&(_, _, comp)| comp == COMP_IMF_BZ_GSM)
        .map(|(t, v, _)| (t, v))
        .collect();
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    if out.is_empty() { None } else { Some(out) }
}

fn load_wind() -> Option<Vec<(f64, f64)>> {
    let bytes = load_local_or_fetch("tao_wnd_zonal.csv", TAO_WND_CDN)?;
    let text = String::from_utf8_lossy(&bytes);
    let lsk = embedded_lsk()?;
    let mut out: Vec<(f64, f64)> = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5 {
            continue;
        }
        let Some(t) = iso_to_tdb(&lsk, cols[0].trim()) else {
            continue;
        };
        let Some(w) = cols[4].trim().parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        out.push((t, w));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    if out.is_empty() { None } else { Some(out) }
}

fn load_quake() -> Option<Vec<(f64, f64)>> {
    let bytes = load_local_or_fetch("usgs_comcat_m45.bin", USGS_COMCAT_CDN)?;
    let recs = parse_comcat_bin(&bytes)?;
    let lsk = embedded_lsk()?;
    let mut out: Vec<(f64, f64)> = Vec::new();
    for (t_unix, v, comp) in recs {
        if comp != COMP_RATE || !v.is_finite() {
            continue;
        }
        let Some(t) = lsk.unix_to_tdb(t_unix) else {
            continue;
        };
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    if out.is_empty() { None } else { Some(out) }
}

fn monthly_sst(records: &[GeoRec]) -> Vec<(f64, f64)> {
    let mut map: HashMap<u64, (f64, Vec<f64>)> = HashMap::new();
    for r in records {
        if r.comp != COMP_ERSSTV5 || !r.val.is_finite() {
            continue;
        }
        let entry = map
            .entry(r.t.to_bits())
            .or_insert_with(|| (r.t, Vec::new()));
        entry.1.push(r.val);
    }
    let mut months: Vec<(f64, f64)> = map
        .into_iter()
        .map(|(_, (t, vals))| {
            let n = vals.len() as f64;
            (t, vals.iter().sum::<f64>() / n)
        })
        .collect();
    months.sort_by(|a, b| a.0.total_cmp(&b.0));
    months
}

fn load_sst() -> Option<Vec<(f64, f64)>> {
    let bytes = load_local_or_fetch("ersstv5_nino34.bin", ERSSTV5_CDN)?;
    let recs = geo_series_parse_bin("ersstv5_nino34", &bytes)?;
    let months = monthly_sst(&recs);
    if months.is_empty() {
        None
    } else {
        Some(months)
    }
}

fn load_qbo() -> Option<Vec<(f64, f64)>> {
    let bytes = load_local_or_fetch("qbo_30hpa.csv", QBO_CDN)?;
    let text = String::from_utf8_lossy(&bytes);
    let lsk = embedded_lsk()?;
    let mut out: Vec<(f64, f64)> = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 4 {
            continue;
        }
        let year: i64 = match cols[0].trim().parse() {
            Ok(y) => y,
            Err(_) => continue,
        };
        let month: i64 = match cols[1].trim().parse() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let date = format!("{year:04}-{month:02}-01");
        let Some(t) = iso_to_tdb(&lsk, &date) else {
            continue;
        };
        let Some(v) = cols[3].trim().parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    if out.is_empty() { None } else { Some(out) }
}

fn load_d20() -> Option<Vec<(f64, f64)>> {
    let bytes = load_local_or_fetch("d20_thermocline.csv", D20_CDN)?;
    let text = String::from_utf8_lossy(&bytes);
    let lsk = embedded_lsk()?;
    let mut out: Vec<(f64, f64)> = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5 {
            continue;
        }
        let Some(t) = iso_to_tdb(&lsk, cols[0].trim()) else {
            continue;
        };
        let Some(v) = cols[4].trim().parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        out.push((t, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    if out.is_empty() { None } else { Some(out) }
}

fn bin_monthly(series: &[(f64, f64)], months: &[f64]) -> Vec<Option<f64>> {
    let mut sums = vec![0.0f64; months.len()];
    let mut counts = vec![0u32; months.len()];
    let mut mi = 0usize;
    for &(t, v) in series {
        while mi + 1 < months.len() && t >= months[mi + 1] {
            mi += 1;
        }
        if t < months[mi] {
            continue;
        }
        let hi = months.get(mi + 1).copied().unwrap_or(months[mi] + MONTH_S);
        if t >= hi {
            continue;
        }
        sums[mi] += v;
        counts[mi] += 1;
    }
    (0..months.len())
        .map(|i| {
            if counts[i] > 0 {
                Some(sums[i] / counts[i] as f64)
            } else {
                None
            }
        })
        .collect()
}

fn deseasonalize_monthly(series: &[Option<f64>]) -> Vec<Option<f64>> {
    let mut sums = [0.0f64; CAL_MONTHS];
    let mut sumsq = [0.0f64; CAL_MONTHS];
    let mut counts = [0u32; CAL_MONTHS];
    for (i, v) in series.iter().enumerate() {
        if let Some(x) = v {
            if x.is_finite() {
                sums[i % CAL_MONTHS] += x;
                sumsq[i % CAL_MONTHS] += x * x;
                counts[i % CAL_MONTHS] += 1;
            }
        }
    }
    let mut means = [0.0f64; CAL_MONTHS];
    let mut sds = [0.0f64; CAL_MONTHS];
    for (m, mean) in means.iter_mut().enumerate() {
        if counts[m] >= CLIMATOLOGY_FLOOR as u32 {
            let n = counts[m] as f64;
            *mean = sums[m] / n;
            let var = (sumsq[m] / n - *mean * *mean).max(0.0);
            sds[m] = var.sqrt();
        } else if counts[m] > 0 {
            println!(
                "deseasonalize: calendar month {:02} carries n = {} measured values < floor {CLIMATOLOGY_FLOOR} — its values stay unchanged (climatology not removed)",
                m + 1,
                counts[m]
            );
        }
    }
    series
        .iter()
        .enumerate()
        .map(|(i, v)| match v {
            Some(x) if counts[i % CAL_MONTHS] >= CLIMATOLOGY_FLOOR as u32 => {
                let m = i % CAL_MONTHS;
                let centered = *x - means[m];
                if sds[m].is_finite() && sds[m] > 0.0 {
                    Some(centered / sds[m])
                } else {
                    Some(centered)
                }
            }
            other => *other,
        })
        .collect()
}

fn paired_series(a: &[Option<f64>], b: &[Option<f64>]) -> (Vec<f32>, Vec<f32>) {
    let mut av = Vec::new();
    let mut bv = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (x, y) {
            if x.is_finite() && y.is_finite() {
                av.push(*x as f32);
                bv.push(*y as f32);
            }
        }
    }
    (av, bv)
}

fn triple_series(
    a: &[Option<f64>],
    b: &[Option<f64>],
    c: &[Option<f64>],
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut av = Vec::new();
    let mut bv = Vec::new();
    let mut cv = Vec::new();
    for ((x, y), z) in a.iter().zip(b.iter()).zip(c.iter()) {
        if let (Some(x), Some(y), Some(z)) = (x, y, z) {
            if x.is_finite() && y.is_finite() && z.is_finite() {
                av.push(*x as f32);
                bv.push(*y as f32);
                cv.push(*z as f32);
            }
        }
    }
    (av, bv, cv)
}

fn channel_active(c: &Option<Vec<Option<f64>>>) -> bool {
    c.as_ref()
        .is_some_and(|m| m.iter().filter(|v| v.is_some()).count() >= MIN_PAIRED)
}

fn channel_pending_reason(c: &Option<Vec<Option<f64>>>, label: &str) -> String {
    match c {
        None => format!("{label} asset absent (loads null)"),
        Some(m) => {
            let n = m.iter().filter(|v| v.is_some()).count();
            format!("{label} n = {n} months < floor {MIN_PAIRED}")
        }
    }
}

fn window_report(name: &str, s: &[(f64, f64)]) {
    match (s.first(), s.last()) {
        (Some(&(a, _)), Some(&(b, _))) => {
            let days = (b - a) / 86400.0;
            println!(
                "{name:<14} | n = {:<6} | window {:.2} d | cadence {:.2} s",
                s.len(),
                days,
                if s.len() > 1 {
                    (b - a) / (s.len() as f64 - 1.0)
                } else {
                    0.0
                }
            );
        }
        _ => println!("{name:<14} | no samples — the channel harvests null"),
    }
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:>14.4e}"),
        None => "      absent".to_string(),
    }
}

fn direction_verdict(
    from: &str,
    to: &str,
    to_s: &[f32],
    from_s: &[f32],
    lags: &[usize],
    fam: Option<f64>,
) -> String {
    let mut best: Option<(usize, f64)> = None;
    for &lag in lags {
        if let Some(te) = transfer_entropy_lag(to_s, from_s, lag) {
            if best.is_none_or(|(_, b)| te > b) {
                best = Some((lag, te));
            }
        }
    }
    match best {
        Some((lag, te)) => {
            match surrogate_stats_phase_n(to_s, from_s, lag, SURROGATE_SEED, N_SURR) {
                Some((mean, sd, thr)) => {
                    let verdict = match fam {
                        Some(f) if te > f => "arrow",
                        _ if te > thr => "family bound",
                        _ => "silent",
                    };
                    let fam_s = match fam {
                        Some(f) => format!("{f:.4e}"),
                        None => "absent".to_string(),
                    };
                    format!(
                        "{from:>5} → {to:<5} | best lag {lag:>2} months | TE {te:.4e} | threshold {thr:.4e} (surrogate mean {mean:.3e}, σ {sd:.3e}) | fam {fam_s} | excess {:+.4e} | {verdict}",
                        te - thr
                    )
                }
                None => format!(
                    "{from:>5} → {to:<5} | best lag {lag:>2} months | TE {te:.4e} | threshold absent (surrogates < 2)"
                ),
            }
        }
        None => format!("{from:>5} → {to:<5} | TE absent (n < 8) — the pair stays unmeasured"),
    }
}

struct PositiveControl {
    planted_lag: usize,
    best_lag: usize,
    te: f64,
    threshold: f64,
    fam: f64,
    detected: bool,
}

fn lcg_uniform(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*state >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn gaussian_noise(state: &mut u64) -> f64 {
    let u1 = lcg_uniform(state).max(f64::MIN_POSITIVE);
    let u2 = lcg_uniform(state);
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

fn synthetic_coupled_pair(
    n: usize,
    planted_lag: usize,
    coupling: f64,
    ar: f64,
    driver_noise: f64,
    target_noise: f64,
    seed: u64,
) -> (Vec<f32>, Vec<f32>) {
    let mut state = seed | 1;
    let mut driver = vec![0.0f64; n];
    let mut target = vec![0.0f64; n];
    let mut d = 0.0f64;
    let mut x = 0.0f64;
    for t in 0..n {
        d = ar * d + driver_noise * gaussian_noise(&mut state);
        let lagged = if t >= planted_lag {
            driver[t - planted_lag]
        } else {
            0.0
        };
        x = ar * x + coupling * lagged + target_noise * gaussian_noise(&mut state);
        driver[t] = d;
        target[t] = x;
    }
    (
        driver.iter().map(|&v| v as f32).collect(),
        target.iter().map(|&v| v as f32).collect(),
    )
}

fn positive_control() -> Option<PositiveControl> {
    let (driver, target) = synthetic_coupled_pair(
        PC_N,
        PC_PLANTED_LAG,
        PC_COUPLING,
        PC_AR,
        PC_DRIVER_NOISE,
        PC_TARGET_NOISE,
        SURROGATE_SEED,
    );
    let lags: Vec<usize> = (0..=MAX_LAG_MONTHS).collect();
    let mut best: Option<(usize, f64)> = None;
    for &lag in &lags {
        if let Some(te) = transfer_entropy_lag(&target, &driver, lag) {
            if best.is_none_or(|(_, b)| te > b) {
                best = Some((lag, te));
            }
        }
    }
    let (best_lag, te) = best?;
    let (_, _, threshold) =
        surrogate_stats_phase_n(&target, &driver, best_lag, SURROGATE_SEED, N_SURR)?;
    let mut fam: Option<f64> = None;
    for &lag in &lags {
        for (x, y) in [(&target, &driver), (&driver, &target)] {
            if let Some(m) = surrogate_max_phase_n(x, y, lag, SURROGATE_SEED, N_SURR) {
                fam = Some(fam.map_or(m, |f| f.max(m)));
            }
        }
    }
    let fam = fam?;
    Some(PositiveControl {
        planted_lag: PC_PLANTED_LAG,
        best_lag,
        te,
        threshold,
        fam,
        detected: te > fam,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    println!("=== ENSO Blatt probe — the directional driver of the NINO3.4 SST anomaly ===");
    println!(
        "Channels (Council 2026-09-30 cut): 1·Wnd (tao_wnd_zonal, atmos) · 2·Quake (usgs_comcat_m45, litho) · 3·Bz (omni_hro_imf_bz_gsm_nt, helios) · 4·QBO (qbo_30hpa, atmos) · 5·D20 (d20_thermocline, ocean) — target SST (ersstv5_nino34_ssta, ocean)."
    );
    println!(
        "Cut (operator word 2026-09-29): NINO3.4 box lat −5…5, lon 190…240, 1854-01-01…2026-08-01."
    );
    println!(
        "Estimator: TE(Y→X; τ) = Σ_t ln[ p(x_{{t+τ}}, x_t, y_t) · p(x_t) / (p(x_t, y_t) · p(x_{{t+τ}}, x_t)) ] / m, KDE Silverman, lag in months."
    );
    println!(
        "Threshold: phase-randomized surrogates (f64 FFT, {N_SURR} realizations), mean + 2σ — the null-control record."
    );
    println!(
        "fam = round max over the measured directed pairs × lags (up to 30 = 6 × 5 ordered pairs over {{Wnd, Quake, Bz, SST, QBO, D20}}; the target is counted; a pending channel's pairs stay unmeasured)."
    );
    println!("Time base: TDB seconds since J2000; TE is shift-invariant.");

    let Some(bz) = load_bz() else {
        eprintln!("Bz series carries no records — the run stays unmeasured");
        return;
    };
    let Some(sst) = load_sst() else {
        eprintln!("SST series carries no records — the run stays unmeasured");
        return;
    };
    let wind = load_wind();
    let quake = load_quake();
    let qbo = load_qbo();
    let d20 = load_d20();

    println!();
    println!("=== channel board ===");
    if let Some(w) = &wind {
        window_report("Wnd", w);
    } else {
        println!("Wnd            | no samples — the channel harvests null (asset absent)");
    }
    if let Some(q) = &quake {
        window_report("Quake", q);
    } else {
        println!("Quake          | no samples — the channel harvests null (asset absent)");
    }
    if let Some(q) = &qbo {
        window_report("QBO", q);
    } else {
        println!("QBO            | no samples — the channel harvests null (asset absent)");
    }
    if let Some(d) = &d20 {
        window_report("D20", d);
    } else {
        println!("D20            | no samples — the channel harvests null (asset absent)");
    }
    window_report("Bz", &bz);
    window_report("SST", &sst);

    let months: Vec<f64> = sst.iter().map(|&(t, _)| t).collect();
    let sst_vals: Vec<f64> = sst.iter().map(|&(_, v)| v).collect();
    let bz_month = deseasonalize_monthly(&bin_monthly(&bz, &months));
    let wind_month = wind
        .as_ref()
        .map(|s| deseasonalize_monthly(&bin_monthly(s, &months)));
    let quake_month = quake
        .as_ref()
        .map(|s| deseasonalize_monthly(&bin_monthly(s, &months)));
    let qbo_month = qbo
        .as_ref()
        .map(|s| deseasonalize_monthly(&bin_monthly(s, &months)));
    let d20_month = d20
        .as_ref()
        .map(|s| deseasonalize_monthly(&bin_monthly(s, &months)));
    let sst_month: Vec<Option<f64>> = deseasonalize_monthly(
        &sst_vals
            .iter()
            .map(|v| Some(*v))
            .collect::<Vec<Option<f64>>>(),
    );

    let channels: [Option<Vec<Option<f64>>>; 6] = [
        wind_month,
        quake_month,
        Some(bz_month),
        Some(sst_month),
        qbo_month,
        d20_month,
    ];

    println!();
    println!("=== monthly channel board (aligned to the SST grid) ===");
    println!("{:<6} | {:<7} | {:>7} | state", "chan", "medium", "n_mon");
    for i in 0..6 {
        let n = channels[i]
            .as_ref()
            .map_or(0, |m| m.iter().filter(|v| v.is_some()).count());
        let state = if channel_active(&channels[i]) {
            "active".to_string()
        } else {
            channel_pending_reason(&channels[i], CH_NAMES[i])
        };
        println!(
            "{:<6} | {:<7} | {:>7} | {}",
            CH_NAMES[i], CH_MEDIA[i], n, state
        );
    }

    let (sst_paired, bz_paired) = match (channels[CH_SST].as_ref(), channels[CH_BZ].as_ref()) {
        (Some(s), Some(b)) => paired_series(s, b),
        _ => (Vec::new(), Vec::new()),
    };
    println!();
    println!(
        "common months (Bz and SST both measured): {}",
        sst_paired.len()
    );
    if sst_paired.len() < MIN_PAIRED {
        eprintln!("paired n below the statement floor ({MIN_PAIRED}) — no verdict");
        return;
    }

    let lags: Vec<usize> = (0..=MAX_LAG_MONTHS).collect();
    println!();
    println!(
        "=== Lag sweep ({} months, monthly grid, Bz ↔ SST) ===",
        MAX_LAG_MONTHS
    );
    println!("{:>4} | {:>14} | {:>14}", "lag", "TE(Bz→SST)", "TE(SST→Bz)");
    for &lag in &lags {
        let b2s = transfer_entropy_lag(&sst_paired, &bz_paired, lag);
        let s2b = transfer_entropy_lag(&bz_paired, &sst_paired, lag);
        println!("{lag:>4} | {} | {}", fmt_opt(b2s), fmt_opt(s2b));
    }

    let mut active_series: Vec<(usize, usize, Vec<f32>, Vec<f32>)> = Vec::new();
    for from in 0..6 {
        for to in 0..6 {
            if from == to {
                continue;
            }
            if !channel_active(&channels[from]) || !channel_active(&channels[to]) {
                continue;
            }
            let (Some(a), Some(b)) = (channels[from].as_ref(), channels[to].as_ref()) else {
                continue;
            };
            let (from_s, to_s) = paired_series(a, b);
            if to_s.len() < MIN_PAIRED {
                continue;
            }
            active_series.push((from, to, from_s, to_s));
        }
    }

    let mut fam: Option<f64> = None;
    for (_, _, from_s, to_s) in &active_series {
        for &lag in &lags {
            if let Some(m) = surrogate_max_phase_n(to_s, from_s, lag, SURROGATE_SEED, N_SURR) {
                fam = Some(fam.map_or(m, |f| f.max(m)));
            }
        }
    }

    println!();
    match fam {
        Some(f) => println!(
            "=== Family bound (round max over the measured directed pairs × lags, {N_SURR} surrogates) === fam = {f:.4e}"
        ),
        None => println!("=== Family bound absent (surrogates < 2) ==="),
    }

    println!();
    match positive_control() {
        Some(pc) => {
            let verdict = if pc.detected {
                format!("detected at lag {}", pc.best_lag)
            } else {
                "not detected".to_string()
            };
            println!(
                "positive control (planted coupling): {verdict} | planted lag {} | best lag {} | TE {:.4e} | per-lag threshold {:.4e} | fam {:.4e}",
                pc.planted_lag, pc.best_lag, pc.te, pc.threshold, pc.fam
            );
        }
        None => println!(
            "positive control (planted coupling): absent — the estimator returned null on the synthetic pair"
        ),
    }
    println!(
        "=== Directed-path census and verdict (up to 30 ordered pairs = 6 × 5 over Wnd, Quake, Bz, SST, QBO, D20; target counted; per-channel pending named; per-lag threshold = mean + 2σ; arrow iff TE > fam) ==="
    );
    for from in 0..6 {
        for to in 0..6 {
            if from == to {
                continue;
            }
            let (from_name, to_name) = (CH_NAMES[from], CH_NAMES[to]);
            let pair_pending = || {
                let reason = if !channel_active(&channels[from]) {
                    channel_pending_reason(&channels[from], from_name)
                } else if !channel_active(&channels[to]) {
                    channel_pending_reason(&channels[to], to_name)
                } else {
                    format!("paired n < floor {MIN_PAIRED}")
                };
                println!("{from_name:>5} → {to_name:<5} | pending ({reason})");
            };
            if channel_active(&channels[from]) && channel_active(&channels[to]) {
                let (Some(a), Some(b)) = (channels[from].as_ref(), channels[to].as_ref()) else {
                    pair_pending();
                    continue;
                };
                let (from_s, to_s) = paired_series(a, b);
                if to_s.len() >= MIN_PAIRED {
                    let line = direction_verdict(from_name, to_name, &to_s, &from_s, &lags, fam);
                    println!("{line}");
                    continue;
                }
            }
            pair_pending();
        }
    }

    println!();
    println!("=== Conditional arm cTE(Bz → SST | Wnd) ===");
    if channel_active(&channels[CH_WND]) {
        if let (Some(s), Some(b), Some(w)) = (
            channels[CH_SST].as_ref(),
            channels[CH_BZ].as_ref(),
            channels[CH_WND].as_ref(),
        ) {
            let (sst_c, bz_c, wnd_c) = triple_series(s, b, w);
            if sst_c.len() >= MIN_PAIRED {
                match conditional_embedded_te_phase(&sst_c, &bz_c, &wnd_c, 3, SURROGATE_SEED) {
                    Some(v) => println!(
                        "cTE(Bz → SST | Wnd) = {:.4e} | threshold {:.4e} (surrogate mean {:.3e}, σ {:.3e}, {} surrogates) | τ_x {} τ_y {} τ_z {} | {}",
                        v.te,
                        v.threshold,
                        v.surrogate_mean,
                        v.surrogate_sd,
                        v.surrogates_used,
                        v.tau_x,
                        v.tau_y,
                        v.tau_z,
                        if v.te > v.threshold {
                            "conditioned arrow"
                        } else {
                            "conditioned silent"
                        }
                    ),
                    None => println!(
                        "cTE(Bz → SST | Wnd) absent — the estimator returned null (MI lag or embedding void)"
                    ),
                }
            } else {
                println!(
                    "cTE(Bz → SST | Wnd) pending — triple-overlap n = {} < floor {MIN_PAIRED}",
                    sst_c.len()
                );
            }
        }
    } else {
        println!(
            "cTE(Bz → SST | Wnd) pending — {}",
            channel_pending_reason(&channels[CH_WND], "Wnd")
        );
    }

    println!();
    println!("=== Named confounds (mandatory) ===");
    println!(
        "Annual cycle removed: the monthly climatology (per calendar month-of-year, index mod {CAL_MONTHS} on the shared SST grid) is standardized (mean subtracted, sd divided) uniformly from all six channels (Wnd, Quake, Bz, SST, QBO, D20) after `bin_monthly` and before the `channels` array is built (council decision 2026-10-02; the sd division answers the GLM-5.3/Claude first-moment critique — a seasonally modulated variance is itself a common annual driver); grouping is by month-of-year, so year-boundary wraparound is handled. Named and pending: the surrogates must be built on the anomaly scale (or the climatology removal repeated inside every surrogate), else the null mismatch biases the p-values; a seasonal positive control (common annual driver + coupling) is the named fixture."
    );
    println!(
        "In-sample climatology: the seasonal mean is estimated from the same 1854-01…2026-08 span it is subtracted from; the leakage is small and named, never swallowed — each calendar month carries n ≥ floor {CLIMATOLOGY_FLOOR} measured values, so the self-weight of the subtracted mean is 1/n (large n on the full-span channels); a calendar month below the floor keeps its values unchanged and emits its named note."
    );
    println!(
        "Positive control untouched: the synthetic pair (`synthetic_coupled_pair`) is an AR(1) driver with a planted lagged coupling and carries no annual cycle, so `positive_control()` runs without deseasonalization — its detection stays the positive control for the estimator."
    );
    println!(
        "Deferred follow-up: a seasonal positive control — a common annual driver plus a planted coupling — as a fixture is pending; it would measure whether deseasonalization suppresses a genuinely seasonal coupling rather than only the common cycle."
    );
    println!(
        "Quake counting series: the Quake channel is a monthly event count (usgs_comcat_m45_rate, events per month), an autocorrelated counting series with declustering absent — its serial memory is counting nature, not a physical driver."
    );

    println!();
    println!("=== THE BLATT ===");
    println!("Title: the directional driver of the NINO3.4 SST anomaly.");
    println!(
        "Pair (written): TE(Bz → SST) vs TE(SST → Bz), monthly grid, lag sweep 0–{MAX_LAG_MONTHS} months."
    );
    println!(
        "Urteil: {b2s_line} / {s2b_line}",
        b2s_line = direction_verdict("Bz", "SST", &sst_paired, &bz_paired, &lags, fam),
        s2b_line = direction_verdict("SST", "Bz", &bz_paired, &sst_paired, &lags, fam)
    );
    println!(
        "Schwelle: per-lag mean + 2σ over phase-randomized surrogates (f64 FFT, {N_SURR}) plus the round-max family bound fam over the measured directed pairs (§3.2 rule)."
    );
    println!(
        "Fenster: n = {} months | SST 1854-01-01…2026-08-01 (ERSSTv5, cut NINO3.4) | Bz OMNI_HRO_1MIN hourly → monthly | φ/sources.φ omni_hro_imf_bz_gsm_nt × ersstv5_nino34_ssta.",
        sst_paired.len()
    );

    println!();
    println!("=== Missing register (named, not concealed) ===");
    println!(
        "Multiple-comparison correction: fam = round max over the measured directed pairs × lags; every pending pair is named in the census above (built, §3.2 rule)."
    );
    if channel_active(&channels[CH_WND]) {
        println!("Wnd channel measured.");
    } else {
        println!(
            "Wnd channel pending: {}. `tao_wnd_zonal.csv` carries the 1977-11-06+ zonal wind record (`tao_wnd_compiler.rs` SOURCE_START); the channel stays unmeasured when the asset is absent or the aligned monthly n is below the floor.",
            channel_pending_reason(&channels[CH_WND], "Wnd")
        );
    }
    if channel_active(&channels[CH_QUAKE]) {
        println!("Quake channel measured.");
    } else {
        println!(
            "Quake channel pending: {}. `usgs_comcat_m45.bin` carries the USGS comcat M4.5 monthly count (`usgs_comcat.rs`, `usgs_comcat_m45_rate`, epoch at month midpoint in UTC unix s → TDB); the channel stays unmeasured when the asset is absent or the aligned monthly n is below the floor.",
            channel_pending_reason(&channels[CH_QUAKE], "Quake")
        );
    }
    if channel_active(&channels[CH_QBO]) {
        println!("QBO channel measured.");
    } else {
        println!(
            "QBO channel pending: {}. `qbo_30hpa.csv` carries the CPC 30 mb zonal wind record (`qbo_compiler.rs`, `qbo_30hpa_ms`, year-month-day rows → TDB); the channel stays unmeasured when the asset is absent or the aligned monthly n is below the floor.",
            channel_pending_reason(&channels[CH_QBO], "QBO")
        );
    }
    if channel_active(&channels[CH_D20]) {
        println!("D20 channel measured.");
    } else {
        println!(
            "D20 channel pending: {}. `d20_thermocline.csv` carries the PMEL TAO 20°C isotherm depth station-days (`d20_compiler.rs`, `d20_thermocline_depth_m`, iso_6 column → TDB); the channel stays unmeasured when the asset is absent or the aligned monthly n is below the floor.",
            channel_pending_reason(&channels[CH_D20], "D20")
        );
    }
    println!("KDE bandwidth h sensitivity (Silverman factor sweep): pending.");
    println!(
        "The area mean over the ±5° box is equal-weight (cos-lat variation < 0.4% over the box)."
    );
    println!("The last SST month's tail bucket is a fixed 30-day window (calendar tail), named.");
    println!("Silent lines are findings. Exit 0.");

    if let Some(i) = args.iter().position(|a| a == "--write") {
        let path = match args.get(i + 1) {
            Some(p) => p.clone(),
            None => {
                eprintln!("enso_blatt_probe: --write carries no path");
                std::process::exit(2);
            }
        };
        if sst_paired.len() < BLATT_N_FLOOR {
            eprintln!(
                "enso_blatt_probe: paired n = {} < {} — the pair stays unwritten (0 honored)",
                sst_paired.len(),
                BLATT_N_FLOOR
            );
        } else {
            match current_commit_sha() {
                Some(sha) => {
                    let span_s = months.last().copied().unwrap_or(months[0]) - months[0];
                    if let Err(e) = write_blatt_pair(
                        &path,
                        &sst_paired,
                        &bz_paired,
                        &BlattPairSpec {
                            span_s,
                            cadence_s: MONTH_S,
                            seed: SURROGATE_SEED,
                            commit_sha: &sha,
                            n_surr: N_SURR,
                        },
                    ) {
                        eprintln!("enso_blatt_probe: pair write refused: {e}");
                        std::process::exit(2);
                    }
                    println!(
                        "pair written: {path} (xs = SST, ys = Bz, n = {})",
                        sst_paired.len()
                    );
                }
                None => eprintln!(
                    "enso_blatt_probe: commit-sha absent — the pair stays unwritten (0 honored)"
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_control_recovers_planted_coupling() {
        let pc = positive_control().expect("the synthetic pair carries a positive control");
        assert!(
            pc.detected,
            "the planted coupling is not detected: TE {} <= fam {}",
            pc.te, pc.fam
        );
        let span = pc.planted_lag.abs_diff(pc.best_lag);
        assert!(
            span <= 1,
            "the positive control recovers planted lag {} at best lag {} (TE {}, fam {})",
            pc.planted_lag,
            pc.best_lag,
            pc.te,
            pc.fam
        );
    }

    #[test]
    fn deseasonalize_removes_common_annual_cycle() {
        let years = 40usize;
        let n = years * CAL_MONTHS;
        let mut series: Vec<Option<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let m = i % CAL_MONTHS;
            let seasonal = 3.0 * (std::f64::consts::TAU * m as f64 / CAL_MONTHS as f64).sin();
            let noise = ((i as f64) * 0.37).sin() * 0.5;
            series.push(Some(seasonal + noise));
        }
        let out = deseasonalize_monthly(&series);
        assert_eq!(out.len(), n);
        for m in 0..CAL_MONTHS {
            let vals: Vec<f64> = (0..n)
                .filter(|&i| i % CAL_MONTHS == m)
                .filter_map(|i| out[i])
                .collect();
            assert_eq!(vals.len(), years);
            let mean = vals.iter().sum::<f64>() / vals.len() as f64;
            assert!(
                mean.abs() < 1e-12,
                "calendar month {m} residual mean {mean} is not ≈ 0"
            );
        }
    }
}
