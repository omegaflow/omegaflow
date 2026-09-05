use omegaflow::archivar::C_LIGHT;
use omegaflow::te::{
    ols_residual, phase_randomized_surrogate, surrogate_stats_phase, transfer_entropy_lag,
    transfer_entropy_lag_h,
};
use std::process::Command;

const LSST_LAMBDA_NM: [(&str, f64); 6] = [
    ("u", 380.0),
    ("g", 500.0),
    ("r", 620.0),
    ("i", 740.0),
    ("z", 880.0),
    ("y", 1000.0),
];

const LSS1_MAGIC: [u8; 4] = *b"LSS1";
const LSS1_HEADER_BYTES: usize = 8;

const COINCIDENCE_S: f64 = 1800.0;
const N_MIN: usize = 24;
const N_COINC_MIN: usize = 12;
const FAP_GATE: f64 = 0.01;

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAG_MAX: usize = 2;
const N_SURR: usize = 10;
const MIN_SAMPLES: usize = 8;
const H_FACTOR: f64 = 2.0;

const C_MAG: f64 = -2.5 / std::f64::consts::LN_10;

const UA: &str = "omegaflow-nadel-v-witness/1.0";
const VSX_BASE: &str = "https://vizier.cfa.harvard.edu/viz-bin/asu-tsv";

struct VsxRow {
    oid: String,
    name: String,
    otype: String,
    period_d: Option<f64>,
}

fn vsx_fetch_text(ra_deg: f64, dec_deg: f64, radius_arcsec: f64, out_max: usize) -> Option<String> {
    let r_deg = radius_arcsec / 3600.0;
    let url = format!(
        "{VSX_BASE}?-source=B/vsx&-out.max={out_max}&-out=OID,Name,RAJ2000,DEJ2000,Type,Period&-c.ra={ra_deg:.6}&-c.dec={dec_deg:.6}&-c.r={r_deg:.8}"
    );
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("30")
        .arg("-A")
        .arg(UA)
        .arg(&url);
    let out = cmd.output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn vsx_cone(ra_deg: f64, dec_deg: f64, radius_arcsec: f64) -> Option<Vec<VsxRow>> {
    let text = vsx_fetch_text(ra_deg, dec_deg, radius_arcsec, 8)?;
    let mut rows: Vec<VsxRow> = Vec::new();
    let mut cols: Option<Vec<String>> = None;
    for line in text.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if line.starts_with('-') {
            continue;
        }
        let toks: Vec<&str> = line.split('\t').collect();
        if cols.is_none() {
            if toks.iter().any(|t| t.trim() == "OID") {
                cols = Some(toks.iter().map(|t| t.trim().to_string()).collect());
            }
            continue;
        }
        let c = cols.as_ref().unwrap();
        let at = |name: &str| -> Option<usize> { c.iter().position(|k| k == name) };
        let (Some(io), Some(i_name), Some(i_type)) = (at("OID"), at("Name"), at("Type")) else {
            continue;
        };
        if io >= toks.len() || i_name >= toks.len() || i_type >= toks.len() {
            continue;
        }
        if toks[io].trim().parse::<i64>().is_err() {
            continue;
        }
        let oid = toks[io].trim().to_string();
        let name = toks[i_name].trim().to_string();
        let otype = toks[i_type].trim().to_string();
        let period_d = at("Period")
            .filter(|&p| p < toks.len())
            .and_then(|p| toks[p].trim().parse::<f64>().ok())
            .filter(|v| *v > 0.0);
        rows.push(VsxRow {
            oid,
            name,
            otype,
            period_d,
        });
    }
    Some(rows)
}

// The external witness classes (VSX/GCVS variability types) that are known
// chromatic periodics: pulsators and eclipsing/rotational variables. The
// broker's own classifier column never enters this set.
fn vsx_type_known_chromatic(t: &str) -> bool {
    let t = t.trim();
    if t.is_empty() {
        return false;
    }
    if t.starts_with("RR")
        || t.starts_with("DSCT")
        || t.starts_with("DCEP")
        || t.starts_with("SXPHE")
        || t.starts_with("SXARI")
        || t == "ROT"
    {
        return true;
    }
    t == "E" || t.starts_with("EA") || t.starts_with("EB") || t.starts_with("EW") || t == "AR"
}

struct LsstCurve {
    ra_deg: f64,
    dec_deg: f64,
    freq: f64,
    samples: Vec<(f64, f32)>,
}

struct Group {
    ra: f64,
    dec: f64,
    curves: Vec<LsstCurve>,
}

fn lsst_band_of(freq: f64) -> Option<&'static str> {
    let lam_nm = C_LIGHT / freq * 1e9;
    let mut best: Option<(&str, f64)> = None;
    for (name, central) in LSST_LAMBDA_NM {
        let d = (lam_nm / central - 1.0).abs();
        if d < 0.15 && best.map(|(_, bd)| d < bd).unwrap_or(true) {
            best = Some((name, d));
        }
    }
    best.map(|(name, _)| name)
}

fn parse_lss1(bytes: &[u8]) -> Option<Vec<LsstCurve>> {
    if bytes.len() < LSS1_HEADER_BYTES || bytes[0..4] != LSS1_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = LSS1_HEADER_BYTES;
    let mut curves = Vec::with_capacity(count);
    for _ in 0..count {
        let f64_at = |o: &mut usize| -> Option<f64> {
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            Some(v)
        };
        let ra_deg = f64_at(&mut off)?;
        let dec_deg = f64_at(&mut off)?;
        let freq = f64_at(&mut off)?;
        let n_samples = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
        off += 4;
        if !ra_deg.is_finite() || !dec_deg.is_finite() || !freq.is_finite() {
            return None;
        }
        let mut samples = Vec::with_capacity(n_samples);
        for _ in 0..n_samples {
            let t = f64_at(&mut off)?;
            let f = f32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            if !t.is_finite() || !f.is_finite() {
                return None;
            }
            samples.push((t, f));
        }
        curves.push(LsstCurve {
            ra_deg,
            dec_deg,
            freq,
            samples,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(curves)
}

fn median_f32(v: &[f32]) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = s.len();
    Some(if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    })
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let var = v.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / n;
    (m, var.sqrt())
}

fn rms_f32(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    Some((v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt())
}

fn sd_f32(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    Some((v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt())
}

fn residual_series(curve: &LsstCurve) -> Option<Vec<(f64, f32)>> {
    let flux: Vec<f32> = curve.samples.iter().map(|&(_, f)| f).collect();
    let med = median_f32(&flux)?;
    let res: Vec<f64> = if med > 0.0 {
        flux.iter().map(|&f| (f / med - 1.0) as f64).collect()
    } else {
        let (_, sd) = mean_sd(&flux.iter().map(|&f| f as f64).collect::<Vec<f64>>());
        if sd <= 0.0 {
            return None;
        }
        flux.iter()
            .map(|&f| ((f as f64) - med as f64) / sd)
            .collect()
    };
    Some(
        curve
            .samples
            .iter()
            .zip(&res)
            .map(|(&(t, _), &r)| (t, r as f32))
            .collect(),
    )
}

fn join_series(a: &[(f64, f32)], b: &[(f64, f32)]) -> Vec<(f64, f32, f32)> {
    let mut pairs: Vec<(f64, f32, f32)> = Vec::new();
    let mut used: Vec<bool> = vec![false; b.len()];
    for &(ta, fa) in a {
        let mut best: Option<(usize, f64)> = None;
        for (j, &(tb, _)) in b.iter().enumerate() {
            if used[j] {
                continue;
            }
            let dt = (ta - tb).abs();
            if dt <= COINCIDENCE_S && best.map(|(_, d)| dt < d).unwrap_or(true) {
                best = Some((j, dt));
            }
        }
        if let Some((j, _)) = best {
            used[j] = true;
            pairs.push((ta, fa, b[j].1));
        }
    }
    pairs.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(std::cmp::Ordering::Equal));
    pairs
}

fn spearman(a: &[f32], b: &[f32]) -> Option<f64> {
    let mut idx: Vec<usize> = (0..a.len()).collect();
    idx.sort_by(|&x, &y| a[x].partial_cmp(&a[y]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank_a: Vec<f64> = vec![0.0; a.len()];
    for (r, &i) in idx.iter().enumerate() {
        rank_a[i] = r as f64;
    }
    idx.sort_by(|&x, &y| b[x].partial_cmp(&b[y]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank_b: Vec<f64> = vec![0.0; b.len()];
    for (r, &i) in idx.iter().enumerate() {
        rank_b[i] = r as f64;
    }
    let n = a.len() as f64;
    let ma = rank_a.iter().sum::<f64>() / n;
    let mb = rank_b.iter().sum::<f64>() / n;
    let cov = rank_a
        .iter()
        .zip(&rank_b)
        .map(|(&x, &y)| (x - ma) * (y - mb))
        .sum::<f64>();
    let va = rank_a.iter().map(|&x| (x - ma) * (x - ma)).sum::<f64>();
    let vb = rank_b.iter().map(|&x| (x - mb) * (x - mb)).sum::<f64>();
    if va <= 0.0 || vb <= 0.0 {
        return None;
    }
    Some(cov / (va * vb).sqrt())
}

// The periodogram power in the standard normalization (variance-normalized,
// both quadrature terms, the τ shift that makes the cosine/sine basis
// orthogonal). The statistic is amplitude-invariant: a uniform rescale of the
// series moves the data and the variance together, so the FAP measures the
// significance of the periodicity, not the photometric scale. The form it
// replaces carried x² inside the denominator, making the power scale ~ n/σ² —
// quiet fractional photometry (σ ~ 0.02) read FAP 0 on every row and the
// periodic/aperiodic cell never opened (measured, committed a4b1dcc).
fn lomb_scargle_fap(t: &[f64], r: &[f32]) -> (f64, f64) {
    let n = t.len();
    if n < 8 {
        return (0.0, 1.0);
    }
    let x: Vec<f64> = r.iter().map(|&v| v as f64).collect();
    let mean = x.iter().sum::<f64>() / n as f64;
    let var = x.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n as f64;
    if var <= 0.0 {
        return (0.0, 1.0);
    }
    let tmin = t[0];
    let tmax = t[n - 1];
    let span = (tmax - tmin).max(1.0);
    let fmin = 1.0 / span;
    let fmax = 1.0 / (2.0 * (span / n as f64).max(1.0));
    let mut best_z = 0.0f64;
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
            }
        }
        nf += 1;
        f *= 1.05;
    }
    let n_indep = nf.max(1) as f64;
    let fap = 1.0 - (1.0 - (-best_z).exp()).powf(n_indep);
    (best_z, fap)
}

fn deepest_dip(res: &[(f64, f32)]) -> (f64, f64) {
    let vals: Vec<f64> = res.iter().map(|&(_, r)| r as f64).collect();
    let (_, sd) = mean_sd(&vals);
    let mut min = res[0].1 as f64;
    for &(_, r) in res {
        if (r as f64) < min {
            min = r as f64;
        }
    }
    (min, min / sd.max(1e-300))
}

fn surrogate_stats_phase_h(
    x: &[f32],
    y: &[f32],
    lag: usize,
    factor: f64,
    seed: u64,
) -> Option<(f64, f64, f64)> {
    let mut vals: Vec<f64> = Vec::with_capacity(N_SURR);
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(y, &mut rng);
        if let Some(te) = transfer_entropy_lag_h(x, &ys, lag, factor) {
            vals.push(te);
        }
    }
    if vals.len() < 2 {
        return None;
    }
    let n = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / n;
    Some((mean, var.sqrt(), mean + 2.0 * var.sqrt()))
}

fn fmt_te(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.3e}"),
        None => "n/a (n-floor)".to_string(),
    }
}

struct Row {
    fwd: bool,
    lag: usize,
    te: Option<f64>,
    thr: Option<f64>,
}

struct Outcome {
    linear_carried: bool,
    nonlinear_carried: bool,
    carried: bool,
    h_robust: bool,
    fam_obj: Option<f64>,
    best_te: Option<f64>,
}

fn linear_r2(color: &[f32], cond: &[f32]) -> Option<f64> {
    let n = color.len();
    if n < 2 {
        return None;
    }
    let r = ols_residual(color, cond)?;
    let m0 = color.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
    let rss0 = color.iter().map(|&x| (x as f64 - m0).powi(2)).sum::<f64>();
    let rss1 = r.iter().map(|&x| (x as f64).powi(2)).sum::<f64>();
    if rss0 <= 0.0 {
        return None;
    }
    Some(1.0 - rss1 / rss0)
}

fn linear_coupling_null(
    color: &[f32],
    brightness: &[f32],
    seed: u64,
) -> (Option<f64>, Option<f64>) {
    let real = linear_r2(color, brightness);
    let mut vals: Vec<f64> = Vec::with_capacity(N_SURR);
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(brightness, &mut rng);
        if let Some(v) = linear_r2(color, &ys) {
            vals.push(v);
        }
    }
    if vals.len() < 2 {
        return (real, None);
    }
    let m = vals.len() as f64;
    let mean = vals.iter().sum::<f64>() / m;
    let var = vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / m;
    (real, Some(mean + 2.0 * var.sqrt()))
}

fn probe_color_brightness(
    ra: f64,
    dec: f64,
    pair: &str,
    n: usize,
    color: &[f32],
    brightness: &[f32],
    seed: u64,
    fam_pool: &mut Vec<f64>,
    lin_pool: &mut Vec<f64>,
) -> Outcome {
    println!();
    println!(
        "  color-brightness coupling (negativ-fuzzy two stages), ra {:9.4} dec {:9.4}, {pair}",
        ra, dec
    );
    println!(
        "    color series n={} RMS {:.3e} | brightness ({} band residual) n={} RMS {:.3e}",
        n,
        rms_f32(color).unwrap_or(f64::NAN),
        pair.split('/').next().unwrap_or(""),
        brightness.len(),
        rms_f32(brightness).unwrap_or(f64::NAN)
    );

    let sdw = sd_f32(brightness);
    let mut resid = color.to_vec();
    let sdw_const = sdw.map_or(false, |s| s <= 1e-30);
    if sdw_const {
        println!(
            "    stage-1: brightness witness is constant — nothing to regress, coupling stays pending (0 honored)"
        );
    } else {
        match ols_residual(color, brightness) {
            Some(r) => {
                let m0 = color.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
                let rss0 = color.iter().map(|&x| (x as f64 - m0).powi(2)).sum::<f64>();
                let rss1 = r.iter().map(|&x| (x as f64).powi(2)).sum::<f64>();
                let v = if rss0 > 0.0 { 1.0 - rss1 / rss0 } else { 0.0 };
                println!(
                    "    stage-1 extraction (ols_residual: color on brightness): removed {v:.4} of the color variance (linear law) — resid RMS {:.3e}",
                    rms_f32(&r).unwrap_or(f64::NAN)
                );
                resid = r;
            }
            None => {
                println!(
                    "    stage-1 extraction: the linear fit is degenerate (0 variance of the conditioning) — the color residue stays the color series"
                );
            }
        }
    }

    let (lin_r2, lin_null) = linear_coupling_null(color, brightness, seed);
    if let Some(null) = lin_null {
        lin_pool.push(null);
    }
    let linear_carried = match (lin_r2, lin_null) {
        (Some(r2), Some(null)) => {
            println!(
                "    stage-1 linear coupling test: color-on-brightness r2 {r2:.4} vs the no-coupling null (mean+2sigma of {N_SURR} phase-randomized brightness surrogates) {null:.4} — {}",
                if r2 > null {
                    "the linear color-brightness law is measured"
                } else {
                    "r2 inside the no-coupling null"
                }
            );
            r2 > null
        }
        _ => {
            println!(
                "    stage-1 linear coupling test: the r2 null does not resolve (0 variance or <2 surrogate draws) — the linear channel stays pending"
            );
            false
        }
    };

    let mut rows: Vec<Row> = Vec::new();
    let mut surr_seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    for lag in 0..=LAG_MAX {
        if n < MIN_SAMPLES + lag {
            continue;
        }
        for fwd in [true, false] {
            let (te, thr) = if fwd {
                let t = transfer_entropy_lag(&resid, brightness, lag);
                let s = surrogate_stats_phase(&resid, brightness, lag, surr_seed)
                    .map(|(_, _, thr)| thr);
                surr_seed = surr_seed.wrapping_add(0x517C_C1B7_2722_0A95);
                (t, s)
            } else {
                let t = transfer_entropy_lag(brightness, &resid, lag);
                let s = surrogate_stats_phase(brightness, &resid, lag, surr_seed)
                    .map(|(_, _, thr)| thr);
                surr_seed = surr_seed.wrapping_add(0x517C_C1B7_2722_0A95);
                (t, s)
            };
            if let Some(thr) = thr {
                fam_pool.push(thr);
            }
            rows.push(Row { fwd, lag, te, thr });
        }
    }

    let mut fam_obj: Option<f64> = None;
    for r in &rows {
        if let Some(t) = r.thr {
            fam_obj = Some(match fam_obj {
                Some(a) => a.max(t),
                None => t,
            });
        }
    }

    println!(
        "    stage-2 negative test on the color residue (both directions, lags 0..={LAG_MAX}; phase-randomized surrogate mean+2sigma per test):"
    );
    println!(
        "      fam (strongest per-test null of this run) = {}",
        fam_obj.map_or("n/a".to_string(), |f| format!("{f:.3e}"))
    );

    let mut nonlinear_carried = false;
    let mut best_te: Option<f64> = None;
    let mut best_desc = String::new();
    let mut best_idx: Option<usize> = None;
    for (i, r) in rows.iter().enumerate() {
        let label = if r.fwd {
            "brightness -> color-resid"
        } else {
            "color-resid -> brightness"
        };
        let word = match (r.te, r.thr, fam_obj) {
            (Some(te), Some(thr), Some(fam)) => {
                if te > fam.max(thr) {
                    nonlinear_carried = true;
                    "carried".to_string()
                } else if te > thr {
                    "above own null, below fam".to_string()
                } else {
                    "still (TE ~ 0)".to_string()
                }
            }
            _ => "n/a".to_string(),
        };
        if let Some(te) = r.te {
            if best_te.map_or(true, |b| te > b) {
                best_te = Some(te);
                best_desc = format!("{label} @ lag {}", r.lag);
                best_idx = Some(i);
            }
        }
        println!(
            "      {label:<26} lag {} te {}  thr {}  | {word}",
            r.lag,
            fmt_te(r.te),
            fmt_te(r.thr)
        );
    }

    let mut h_robust = false;
    if let Some(bi) = best_idx {
        let b = &rows[bi];
        let h_seed = seed.wrapping_add(0x2722_0A95_517C_C1B7);
        let (hte, hthr) = if b.fwd {
            let t = transfer_entropy_lag_h(&resid, brightness, b.lag, H_FACTOR);
            let s = surrogate_stats_phase_h(&resid, brightness, b.lag, H_FACTOR, h_seed);
            (t, s)
        } else {
            let t = transfer_entropy_lag_h(brightness, &resid, b.lag, H_FACTOR);
            let s = surrogate_stats_phase_h(brightness, &resid, b.lag, H_FACTOR, h_seed);
            (t, s)
        };
        let h_word = match (hte, hthr) {
            (Some(te), Some((_, _, thr))) if te > thr => {
                h_robust = true;
                "above null".to_string()
            }
            (Some(_), Some(_)) => "still".to_string(),
            _ => "n/a".to_string(),
        };
        println!(
            "      KDE-h sensitivity ({best_desc}, bandwidth x{H_FACTOR}): te {}  thr {}  | {h_word}",
            fmt_te(hte),
            fmt_te(hthr.map(|(_, _, t)| t))
        );
    }

    let carried = linear_carried || nonlinear_carried;
    Outcome {
        linear_carried,
        nonlinear_carried,
        carried,
        h_robust,
        fam_obj,
        best_te,
    }
}

fn groups_of(curves: Vec<LsstCurve>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for c in curves {
        let key = groups
            .iter_mut()
            .find(|g| (g.ra - c.ra_deg).abs() < 1e-3 && (g.dec - c.dec_deg).abs() < 1e-3);
        match key {
            Some(g) => g.curves.push(c),
            None => groups.push(Group {
                ra: c.ra_deg,
                dec: c.dec_deg,
                curves: vec![c],
            }),
        }
    }
    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bin = args.get(1).cloned().unwrap_or_default();
    let map = args.get(2).cloned();
    if bin.is_empty() {
        eprintln!(
            "lsst_color_coupling_probe — home-game layer A behind the LSST anomaly scan\n\
             usage: lsst_color_coupling_probe <lsst_lightcurves.bin> [<lsst_cone_object_map.csv>]"
        );
        std::process::exit(2);
    }
    let bytes = match std::fs::read(&bin) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "lsst_color_coupling_probe: read {bin}: {e} — the layer waits for the LSS1 asset (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let Some(curves) = parse_lss1(&bytes) else {
        eprintln!(
            "lsst_color_coupling_probe: {bin} carries no LSS1 record — the layer stays void (0 honored)"
        );
        std::process::exit(1);
    };
    let groups = groups_of(curves);

    let mut map_rows: Vec<(f64, f64, String, i64, String)> = Vec::new();
    if let Some(map_path) = &map {
        if let Ok(body) = std::fs::read_to_string(map_path) {
            for (i, line) in body.lines().enumerate() {
                if i == 0 {
                    continue;
                }
                let f: Vec<&str> = line.split(',').collect();
                if f.len() != 6 {
                    continue;
                }
                if let (Ok(ra), Ok(dec), Ok(class)) = (
                    f[1].trim().parse::<f64>(),
                    f[2].trim().parse::<f64>(),
                    f[4].trim().parse::<i64>(),
                ) {
                    map_rows.push((ra, dec, f[0].to_string(), class, f[5].trim().to_string()));
                }
            }
        }
    }
    let cone_obj = |ra: f64, dec: f64| -> Option<&(f64, f64, String, i64, String)> {
        map_rows
            .iter()
            .find(|(mra, mdec, _, _, _)| (mra - ra).abs() < 1e-3 && (mdec - dec).abs() < 1e-3)
    };

    println!("=== home-game layer A: the natural-variability signature behind the LSST scan ===");
    println!(
        "witness question: does the periodic candidate's COLOR series couple to its BRIGHTNESS series? A natural periodic variable is chromatic (spots, eclipses, pulsation change the color with the light curve); a colorblind blinker's color does not couple."
    );
    println!(
        "machinery: ols_residual (color on brightness) then transfer_entropy_lag both directions vs surrogate_stats_phase mean+2sigma over lags 0..={LAG_MAX}, KDE-h x{H_FACTOR} sensitivity via transfer_entropy_lag_h; the negative-fuzzy designation 'not carried' never reads 'independent'."
    );
    println!(
        "population: the periodic multiband candidates of the cone scan ({N_MIN}+ samples per band, {N_COINC_MIN}+ coincident visits, FAP < {FAP_GATE})."
    );

    let mut scanned = 0usize;
    let mut single_band = 0usize;
    let mut periodic = 0usize;
    let mut evaluated = 0usize;
    let mut vsx_known = 0usize;
    let mut vsx_known_couple = 0usize;
    let mut vsx_pending = 0usize;
    let mut fam_pool: Vec<f64> = Vec::new();
    let mut lin_pool: Vec<f64> = Vec::new();
    let mut verdicts: Vec<(
        f64,
        f64,
        String,
        i64,
        String,
        String,
        bool,
        bool,
        bool,
        bool,
        bool,
        Option<f64>,
        Option<f64>,
    )> = Vec::new();

    for (gi, g) in groups.iter().enumerate() {
        let bands: Vec<(&str, &LsstCurve)> = g
            .curves
            .iter()
            .filter_map(|c| lsst_band_of(c.freq).map(|b| (b, c)))
            .collect();
        if bands.len() < 2 {
            single_band += 1;
            continue;
        }
        let mut pick: Vec<(&str, &LsstCurve)> = bands
            .iter()
            .filter(|(b, _)| *b == "g" || *b == "r" || *b == "i" || *b == "z")
            .map(|&(b, c)| (b, c))
            .collect();
        pick.sort_by(|a, b| {
            b.1.samples
                .len()
                .cmp(&a.1.samples.len())
                .then_with(|| a.0.cmp(b.0))
        });
        if pick.len() < 2 {
            single_band += 1;
            continue;
        }
        let (ba, ca) = pick[0];
        let (bb, cb) = pick[1];
        let Some(a_res) = residual_series(ca) else {
            continue;
        };
        let Some(b_res) = residual_series(cb) else {
            continue;
        };
        if a_res.len() < N_MIN || b_res.len() < N_MIN {
            continue;
        }
        let joint = join_series(&a_res, &b_res);
        if joint.len() < N_COINC_MIN {
            continue;
        }
        let a_j: Vec<f32> = joint.iter().map(|&(_, v, _)| v).collect();
        let b_j: Vec<f32> = joint.iter().map(|&(_, _, v)| v).collect();
        let tj: Vec<f64> = joint.iter().map(|&(t, _, _)| t).collect();
        let (dip_a, sig_a) = deepest_dip(&a_res);
        let (dip_b, sig_b) = deepest_dip(&b_res);
        let ratio = if dip_b.abs() > 1e-9 {
            dip_a.abs() / dip_b.abs()
        } else {
            f64::INFINITY
        };
        let sig_gate = sig_a.min(sig_b).abs() >= 3.0 && dip_a < 0.0 && dip_b < 0.0;
        let achromatisch = sig_gate && (0.5..=2.0).contains(&ratio);
        let (_, fap) = lomb_scargle_fap(&tj, &a_j);
        let is_periodic = fap < FAP_GATE;
        let rho = spearman(&a_j, &b_j);
        scanned += 1;

        let obj_id = cone_obj(g.ra, g.dec);
        let id_word = obj_id
            .map(|(_, _, id, class, simbad)| format!("{id} (class {class} {simbad})"))
            .unwrap_or_else(|| "no map row".to_string());

        if !is_periodic {
            println!(
                "ra {:9.4} dec {:9.4} {ba}/{bb} aperiodic dip (FAP {fap:.2e}) — the aperiodic cell belongs to the dip-candidate scan, not to the periodic home-game layer",
                g.ra, g.dec
            );
            continue;
        }
        periodic += 1;

        let med_a = match median_f32(&ca.samples.iter().map(|&(_, f)| f).collect::<Vec<f32>>()) {
            Some(m) => m,
            None => continue,
        };
        let med_b = match median_f32(&cb.samples.iter().map(|&(_, f)| f).collect::<Vec<f32>>()) {
            Some(m) => m,
            None => continue,
        };
        let mut color: Vec<f32> = Vec::with_capacity(joint.len());
        let mut brightness: Vec<f32> = Vec::with_capacity(joint.len());
        for &(_, ra_val, rb_val) in &joint {
            let fa = med_a as f64 * (1.0 + ra_val as f64);
            let fb = med_b as f64 * (1.0 + rb_val as f64);
            if fa > 0.0 && fb > 0.0 {
                color.push((C_MAG * (fa / fb).ln()) as f32);
                brightness.push(ra_val);
            }
        }
        let n = color.len();
        if n < MIN_SAMPLES {
            println!(
                "ra {:9.4} dec {:9.4} {ba}/{bb} periodic but only {n} positive-flux joint visits — below the te.rs n-floor (0 honored)",
                g.ra, g.dec
            );
            continue;
        }

        let frame_word = if achromatisch {
            "achromatic"
        } else {
            "chromatic"
        };
        println!();
        println!(
            "ra {:9.4} dec {:9.4} {ba}/{bb} periodic {id_word} | dip {ba} {dip_a:.3} ({sig_a:.1}σ) {bb} {dip_b:.3} ({sig_b:.1}σ) ratio {ratio:.2} {frame_word} | FAP {fap:.2e} {} | color {}-{} n {n}",
            g.ra,
            g.dec,
            match rho {
                Some(r) => format!("rho {r:.2}"),
                None => "rho absent".to_string(),
            },
            ba,
            bb
        );

        let mut vsx_chromatic = false;
        match vsx_cone(g.ra, g.dec, 3.0) {
            Some(m) if !m.is_empty() => {
                vsx_chromatic = vsx_type_known_chromatic(&m[0].otype);
                let pword = m[0]
                    .period_d
                    .map(|p| format!("P {p:.6} d"))
                    .unwrap_or_else(|| "P absent".to_string());
                println!(
                    "  external witness (VSX B/vsx, cone 3\"): {} (OID {} type {}) {}",
                    m[0].name, m[0].oid, m[0].otype, pword
                );
                if vsx_chromatic {
                    println!(
                        "  external witness class: a known chromatic periodic ({}), NOT the broker's classifier",
                        m[0].otype
                    );
                }
                vsx_known += 1;
                if m.len() > 1 {
                    println!(
                        "  external witness: {} further VSX cone match(es) within 3\"",
                        m.len() - 1
                    );
                }
            }
            Some(_) => {
                println!(
                    "  external witness (VSX): no VSX entry within 3\" — the object is not a catalogued variable"
                );
            }
            None => {
                vsx_pending += 1;
                println!(
                    "  external witness (VSX): the crossmatch did not answer (reachability pending)"
                );
            }
        }

        let seed = SEED
            .wrapping_add((gi as u64).wrapping_mul(0x517C_C1B7_2722_0A95))
            .wrapping_add(0x2722_0A95_517C_C1B7);
        let pair = format!("{ba}/{bb}");
        let out = probe_color_brightness(
            g.ra,
            g.dec,
            &pair,
            n,
            &color,
            &brightness,
            seed,
            &mut fam_pool,
            &mut lin_pool,
        );
        let word = if out.carried {
            let chan = if out.linear_carried && out.nonlinear_carried {
                "linear stage-1 and residue stage-2"
            } else if out.linear_carried {
                "linear stage-1"
            } else {
                "residue stage-2"
            };
            format!(
                "couples to the natural color-brightness signature ({chan}) — natural periodic variable"
            )
        } else {
            "not carried — the machine's candidate designation (colorblind periodic blinker), never 'independent'".to_string()
        };
        println!(
            "  VERDICT: {word}{}",
            if out.h_robust && out.nonlinear_carried {
                " — residue TE KDE-h x2 robust"
            } else {
                ""
            }
        );
        if vsx_chromatic {
            if out.carried {
                vsx_known_couple += 1;
                println!(
                    "  POSITIVE CONTROL (external chromatic witness): couples — the test finds the known chromatic periodic (control row passes)"
                );
            } else {
                println!(
                    "  POSITIVE CONTROL (external chromatic witness): NOT carried for a known chromatic periodic — the test measures nothing for this object (control row fails, named)"
                );
            }
        }
        evaluated += 1;
        verdicts.push((
            g.ra,
            g.dec,
            obj_id
                .map(|(_, _, id, _, _)| id.clone())
                .unwrap_or_default(),
            obj_id.map(|(_, _, _, c, _)| *c).unwrap_or(-1),
            obj_id.map(|(_, _, _, _, s)| s.clone()).unwrap_or_default(),
            pair,
            out.carried,
            out.linear_carried,
            out.nonlinear_carried,
            out.h_robust,
            achromatisch,
            out.best_te,
            out.fam_obj,
        ));
    }

    let fam_family = fam_pool.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let fam_lin_family = lin_pool.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    println!();
    println!("=== layer A family statistic over the periodic candidates ===");
    println!(
        "fam over the candidate family (the strongest per-test phase-surrogate mean+2sigma null over all {evaluated} evaluated candidate runs) = {}",
        if fam_pool.is_empty() {
            "n/a (no residue test resolved)".to_string()
        } else {
            format!("{fam_family:.3e}")
        }
    );
    println!(
        "stage-1 linear family null (the strongest r2 mean+2sigma no-coupling null across the runs) = {}",
        if lin_pool.is_empty() {
            "n/a (no run resolved)".to_string()
        } else {
            format!("{fam_lin_family:.3e}")
        }
    );
    let n_couple = verdicts.iter().filter(|v| v.6).count();
    let n_not = verdicts.iter().filter(|v| !v.6).count();
    for v in &verdicts {
        let (
            ra,
            dec,
            id,
            class,
            simbad,
            pair,
            carried,
            linear_carried,
            nonlinear_carried,
            h_robust,
            achr,
            best_te,
            fam_obj,
        ) = v;
        let who = if id.is_empty() {
            format!("ra {ra:.4} dec {dec:.4}")
        } else {
            format!("{id} (class {class} {simbad})")
        };
        let coupl_word = if *carried {
            let chan = if *linear_carried && *nonlinear_carried {
                "linear + residue"
            } else if *linear_carried {
                "linear stage-1"
            } else {
                "residue stage-2"
            };
            format!(
                "couples ({chan}){}",
                if *h_robust && *nonlinear_carried {
                    ", residue TE h x2 robust"
                } else {
                    ""
                }
            )
        } else {
            "not carried".to_string()
        };
        println!(
            "  {who} {pair} | {} periodic | strongest TE {:.3e} vs run fam {:.3e} | {coupl_word}",
            if *achr {
                "achromatic-dip"
            } else {
                "chromatic-dip"
            },
            best_te.unwrap_or(f64::NAN),
            fam_obj.unwrap_or(f64::NAN),
        );
    }
    println!();
    println!(
        "Verdict over the periodic cone population: {periodic} periodic multiband candidate(s) of {scanned} scanned multiband objects ({single_band} single-band absent); {n_couple} couple to the natural color-brightness signature (natural periodic variables); {n_not} not carried — colorblind periodic blinker candidate(s), never 'independent'."
    );
    println!(
        "external witness (VSX B/vsx catalog crossmatch, not the broker's classifier column): {vsx_known} periodic candidate(s) have a VSX entry, of which {vsx_known_couple} couple in the color-brightness test (positive-control rows); {vsx_pending} candidate(s) hit a non-answering crossmatch (pending). A VSX-confirmed chromatic periodic that does NOT couple is a failed positive-control row — the layer would measure nothing for it."
    );
    let achr_candidates: Vec<&(
        f64,
        f64,
        String,
        i64,
        String,
        String,
        bool,
        bool,
        bool,
        bool,
        bool,
        Option<f64>,
        Option<f64>,
    )> = verdicts.iter().filter(|v| v.10).collect();
    if !achr_candidates.is_empty() {
        println!(
            "of the {count} scan-achromatic periodic candidate(s) (the colorblind-blinker population the scan's dip ratio names), {couple} couple in the time-resolved layer (natural) and {notc} stay not carried:",
            count = achr_candidates.len(),
            couple = achr_candidates.iter().filter(|v| v.6).count(),
            notc = achr_candidates.iter().filter(|v| !v.6).count()
        );
        for v in &achr_candidates {
            if !v.6 {
                let (ra, dec, id, class, simbad, pair, _, _, _, _, _, _, _) = v;
                println!(
                    "    colorblind blinker candidate at ra {ra:.4} dec {dec:.4} {id} (class {class} {simbad}) {pair}"
                );
            }
        }
    }
    println!();
    println!(
        "Designation: 'not carried' is the negative-fuzzy candidate word. It is never 'independent' — the coupling of an unknown natural class that no provided signature names is not excluded (missing-witness limit). The residue stage-2 test alone cannot see a color series whose coupling to brightness is purely linear (the OLS stage removes it); the layer therefore weighs the stage-1 linear law against its own phase-randomized null (named, measured, not assumed)."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // A verbatim slice of the real VizieR B/vsx TSV answer (measured
    // 2026-09-05, cone around ra 245.8967 dec -26.5252) — the parser must read
    // the schema as the live server sends it.
    const VSX_SAMPLE: &str = "\
OID\tName\tRAJ2000\tDEJ2000\tType\tPeriod\n\
  \t \tdeg\tdeg\t \td\n\
--------\t------------------------------\t---------\t---------\t------------------------------\t-------------------\n\
 5871185\tGaia DR3 6045465884181860992  \t245.88692\t-26.52603\tE                             \t     3.697550000000\n\
 5871187\tGaia DR3 6045465884191750784  \t245.88702\t-26.52958\tS                             \t\n\
 5871384\tGaia DR3 6045465918551521536  \t245.89449\t-26.51797\tRRAB                           \t     0.478799000000\n";

    #[test]
    fn vsx_sample_parses_the_live_schema() {
        let mut rows = Vec::new();
        let mut cols: Option<Vec<String>> = None;
        for line in VSX_SAMPLE.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if line.starts_with('-') {
                continue;
            }
            let toks: Vec<&str> = line.split('\t').collect();
            if cols.is_none() {
                if toks.iter().any(|t| t.trim() == "OID") {
                    cols = Some(toks.iter().map(|t| t.trim().to_string()).collect());
                }
                continue;
            }
            let c = cols.as_ref().unwrap();
            let at = |name: &str| -> Option<usize> { c.iter().position(|k| k == name) };
            let (Some(io), Some(i_name), Some(i_type)) = (at("OID"), at("Name"), at("Type")) else {
                continue;
            };
            if toks[io].trim().parse::<i64>().is_err() {
                continue;
            }
            let oid = toks[io].trim().to_string();
            let name = toks[i_name].trim().to_string();
            let otype = toks[i_type].trim().to_string();
            let period_d = at("Period")
                .filter(|&p| p < toks.len())
                .and_then(|p| toks[p].trim().parse::<f64>().ok())
                .filter(|v| *v > 0.0);
            rows.push((oid, name, otype, period_d));
        }
        assert_eq!(rows.len(), 3, "the real VSX slice carries three rows");
        assert_eq!(rows[0].2, "E");
        assert!((rows[0].3.unwrap() - 3.69755).abs() < 1e-6);
        assert_eq!(rows[1].2, "S");
        assert!(
            rows[1].3.is_none(),
            "an empty Period cell is absent, never 0"
        );
        assert_eq!(rows[2].2, "RRAB");
    }

    #[test]
    fn vsx_external_chromatic_classes_are_external_only() {
        assert!(vsx_type_known_chromatic("RRAB"));
        assert!(vsx_type_known_chromatic("DSCT"));
        assert!(vsx_type_known_chromatic("EW"));
        assert!(vsx_type_known_chromatic("EA/EB"));
        assert!(vsx_type_known_chromatic("ROT"));
        assert!(!vsx_type_known_chromatic("S"));
        assert!(!vsx_type_known_chromatic(""));
        assert!(
            !vsx_type_known_chromatic("SN"),
            "a supernova is not a periodic witness"
        );
        // The broker classifier word is not a variability type and must never
        // be treated as the external witness.
        assert!(!vsx_type_known_chromatic("f:main_label_classifier"));
    }
}
