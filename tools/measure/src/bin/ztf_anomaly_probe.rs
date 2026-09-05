use omegaflow::te::{phase_randomized_surrogate, surrogate_stats_phase, transfer_entropy_lag};
use omegaflow::ztf::{ZtfCurve, parse_ztf_bin};

const COINCIDENCE_S: f64 = 1800.0;
const N_MIN: usize = 24;
const N_COINC_MIN: usize = 12;
const DIP_SIG: f64 = 3.0;
const ACHROMATIC_RATIO: f64 = 2.0;
const FAP_GATE: f64 = 0.01;
const SEED: u64 = 0x5eed_2026;

fn band_of(freq: f64) -> Option<&'static str> {
    if freq > 5.8e14 {
        Some("g")
    } else if freq > 4.2e14 {
        Some("r")
    } else {
        Some("i")
    }
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

fn residual_series(curve: &ZtfCurve) -> Option<Vec<(f64, f32)>> {
    let flux: Vec<f32> = curve.samples.iter().map(|&(_, f)| f).collect();
    let med = median_f32(&flux)?;
    if med <= 0.0 {
        return None;
    }
    Some(
        curve
            .samples
            .iter()
            .map(|&(t, f)| (t, (f / med - 1.0) as f32))
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
// quiet fractional photometry (σ ~ 0.02) read FAP 0 on every row (measured in
// the LSST round, committed a4b1dcc; the same one path served this probe).
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

fn arrow_te(g: &[f32], r: &[f32], fam: &mut f64) -> Option<(f64, f64, String)> {
    let mut best_te = f64::NEG_INFINITY;
    let mut best_thr = 0.0f64;
    let mut best_desc = String::new();
    for (name, x, y) in [("g→r", g, r), ("r→g", r, g)] {
        for lag in 0..=1usize {
            let Some(te) = transfer_entropy_lag(x, y, lag) else {
                continue;
            };
            let Some((_, _, thr)) = surrogate_stats_phase(x, y, lag, SEED) else {
                continue;
            };
            let mut rng = SEED.wrapping_add(0x9e37_79b9_7f4a_7c15);
            for _ in 0..10 {
                let ys = phase_randomized_surrogate(y, &mut rng);
                if let Some(s) = transfer_entropy_lag(x, &ys, lag) {
                    if s > *fam {
                        *fam = s;
                    }
                }
            }
            if te > best_te {
                best_te = te;
                best_thr = thr;
                best_desc = format!("{name} @ lag {lag}");
            }
        }
    }
    if best_te.is_finite() {
        Some((best_te, best_thr, best_desc))
    } else {
        None
    }
}

struct StarGroup {
    ra: f64,
    dec: f64,
    curves: Vec<ZtfCurve>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/ztf_scan/ztf_lightcurves.bin".to_string());
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "ztf_anomaly_probe: read {}: {} — the scan waits for the asset (0 honored)",
                path, e
            );
            return;
        }
    };
    let Some(curves) = parse_ztf_bin(&bytes) else {
        eprintln!(
            "ztf_anomaly_probe: {} carries no ZTF1 record — the scan stays void",
            path
        );
        return;
    };
    let mut groups: Vec<StarGroup> = Vec::new();
    for c in curves {
        let key = groups
            .iter_mut()
            .find(|g| (g.ra - c.ra_deg).abs() < 1e-3 && (g.dec - c.dec_deg).abs() < 1e-3);
        match key {
            Some(g) => g.curves.push(c),
            None => groups.push(StarGroup {
                ra: c.ra_deg,
                dec: c.dec_deg,
                curves: vec![c],
            }),
        }
    }

    let mut fam = f64::NEG_INFINITY;
    let mut scanned = 0usize;
    let mut kandidaten = 0usize;
    let mut single_band = 0usize;

    println!(
        "\n=== Front-I scanner: achromatic + non-periodic shadow cut in the real ZTF light curves ==="
    );
    for g in &groups {
        let bands: Vec<(&str, &ZtfCurve)> = g
            .curves
            .iter()
            .filter_map(|c| band_of(c.freq).map(|b| (b, c)))
            .collect();
        if bands.len() < 2 {
            single_band += 1;
            println!(
                "  ra {:9.4} dec {:9.4}: absent — one band carries no achromatic cut (0 honored)",
                g.ra, g.dec
            );
            continue;
        }
        let gr = bands
            .iter()
            .find(|(b, _)| *b == "g")
            .and_then(|(_, c)| residual_series(c));
        let rr = bands
            .iter()
            .find(|(b, _)| *b == "r")
            .and_then(|(_, c)| residual_series(c));
        let (Some(gs), Some(rs)) = (gr, rr) else {
            println!(
                "  ra {:9.4} dec {:9.4}: absent — one band-residual series carries no positive model (0 honored)",
                g.ra, g.dec
            );
            continue;
        };
        if gs.len() < N_MIN || rs.len() < N_MIN {
            println!(
                "  ra {:9.4} dec {:9.4}: absent — only {}/{} samples, too few for the arrow (0 honored)",
                g.ra,
                g.dec,
                gs.len(),
                rs.len()
            );
            continue;
        }
        let joint = join_series(&gs, &rs);
        if joint.len() < N_COINC_MIN {
            println!(
                "  ra {:9.4} dec {:9.4}: absent — {} coincident visits (|Δt| ≤ 1800 s), too few (0 honored)",
                g.ra,
                g.dec,
                joint.len()
            );
            continue;
        }
        scanned += 1;
        let gv: Vec<f32> = joint.iter().map(|&(_, gv, _)| gv).collect();
        let rv: Vec<f32> = joint.iter().map(|&(_, _, rv)| rv).collect();
        let tj: Vec<f64> = joint.iter().map(|&(t, _, _)| t).collect();
        let rho = spearman(&gv, &rv);
        let (dip_g, sig_g) = deepest_dip(&gs);
        let (dip_r, sig_r) = deepest_dip(&rs);
        let ratio = if dip_r.abs() > 1e-9 {
            dip_g.abs() / dip_r.abs()
        } else {
            f64::INFINITY
        };
        let dip_times_dig = sig_g.min(sig_r) >= DIP_SIG && dip_g < 0.0 && dip_r < 0.0;
        let achromatisch =
            dip_times_dig && (1.0 / ACHROMATIC_RATIO..=ACHROMATIC_RATIO).contains(&ratio);
        let (_, fap) = lomb_scargle_fap(&tj, &gv);
        let nicht_periodisch = fap >= FAP_GATE;
        let Some((te, thr, desc)) = arrow_te(&gv, &rv, &mut fam) else {
            println!(
                "  ra {:9.4} dec {:9.4}: still — the arrow does not measure (0 honored)",
                g.ra, g.dec
            );
            continue;
        };
        let rho_word = match rho {
            Some(r) => format!("rho {r:.2}"),
            None => "rho absent".to_string(),
        };
        let word = if te > fam {
            "CANDIDATE (fam-bearing)"
        } else if te > thr && achromatisch && nicht_periodisch {
            "CANDIDATE"
        } else {
            "still"
        };
        if word.starts_with("CANDIDATE") {
            kandidaten += 1;
        }
        println!(
            "  ra {:9.4} dec {:9.4} {word}: TE {te:.3e} (thr {thr:.3e}) {desc} | {rho_word} | dip g {dip_g:.3} ({sig_g:.1}σ) r {dip_r:.3} ({sig_r:.1}σ) ratio {ratio:.2} | FAP {fap:.2e} {}",
            g.ra,
            g.dec,
            if achromatisch {
                "achromatic"
            } else {
                "chromatic"
            },
        );
    }
    println!("\nfam (multiple comparison over the catalog) = {fam:.3e}");
    println!(
        "Verdict: {kandidaten} candidate(s) of {scanned} scanned multiband stars; {single_band} single-band stars without achromatic cell (absent)"
    );
    println!(
        "Quantitative limit: no shadow cut above fam across {scanned} real light curves (0 honored) — or the line above names it"
    );
}
