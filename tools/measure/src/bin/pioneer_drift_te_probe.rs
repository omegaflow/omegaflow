use omegaflow::atdf::parse_bin;
use omegaflow::te::topological_te_phase;

const JD_UNIX_EPOCH: f64 = 2440587.5;
const DAY: f64 = 86400.0;
const J2000_EPOCH: f64 = 2451545.0;
const STATIONS: [i64; 3] = [14, 43, 63];
const MIN_SAMPLES_PER_MONTH: usize = 3;
const MIN_MONTHS: usize = 24;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn month_of(r: &[f64; 14]) -> Option<i64> {
    let jd = J2000_EPOCH + r[0] / DAY;
    let days = (jd - JD_UNIX_EPOCH).floor() as i64;
    let (y, m, _) = omegaflow::spectral::civil_from_days(days)?;
    Some(y as i64 * 12 + m as i64)
}

fn monthly_medians(
    records: &[[f64; 14]],
    station: i64,
    slot: usize,
    want_sampler: Option<f64>,
    want_mode: Option<f64>,
) -> Vec<(i64, f64)> {
    let mut groups: std::collections::HashMap<i64, Vec<f64>> = std::collections::HashMap::new();
    for r in records {
        if r[6] as i64 != station {
            continue;
        }
        if let Some(s) = want_sampler {
            if (r[3] - s).abs() > 0.1 {
                continue;
            }
        }
        if let Some(md) = want_mode {
            if r[13] as i64 != md as i64 {
                continue;
            }
        }
        let v = r[slot];
        if !v.is_finite() {
            continue;
        }
        let Some(mo) = month_of(r) else {
            continue;
        };
        groups.entry(mo).or_default().push(v);
    }
    let mut out: Vec<(i64, f64)> = Vec::new();
    for (mo, mut vals) in groups {
        if vals.len() < MIN_SAMPLES_PER_MONTH {
            continue;
        }
        vals.sort_by(f64::total_cmp);
        out.push((mo, vals[vals.len() / 2]));
    }
    out.sort_by_key(|x| x.0);
    out
}

fn lin_fit(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let sx: f64 = xs.iter().sum();
    let sy: f64 = ys.iter().sum();
    let sxx: f64 = xs.iter().map(|x| x * x).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let denom = n * sxx - sx * sx;
    if denom.abs() < 1e-300 {
        return (0.0, 0.0);
    }
    let a = (n * sxy - sx * sy) / denom;
    let b = (sy - a * sx) / n;
    (a, b)
}

fn span_text(first: i64, last: i64) -> String {
    format!(
        "{:04}-{:02}..{:04}-{:02}",
        first / 12,
        first % 12 + 1,
        last / 12,
        last % 12 + 1
    )
}

fn main() {
    let path = "data/pioneer10_skyfreq.bin";
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("pioneer10: skyfreq bin void ({path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("pioneer10: skyfreq bin parse void");
        return;
    };

    let mut classes: Vec<(String, usize, Option<f64>, Option<f64>)> = Vec::new();
    classes.push(("ref_all".to_string(), 2, None, None));
    classes.push(("resid_1s3w".to_string(), 8, Some(1.0), Some(3.0)));
    classes.push(("resid_60s2w".to_string(), 8, Some(60.0), Some(2.0)));

    println!(
        "Pioneer drift TE: {} PASF records — monthly median per station, ≥ {} samples/month, TE dim 3 order 3",
        records.len(),
        MIN_SAMPLES_PER_MONTH
    );
    println!();
    println!("coverage per station (sampler class × mode):");
    println!(
        "{:>4} | {:>10} | {:>12}",
        "Sta", "Sampler s", "Samples/Mode"
    );
    for st in STATIONS {
        let mut counts: std::collections::HashMap<(u64, i64), usize> =
            std::collections::HashMap::new();
        for r in &records {
            if r[6] as i64 != st {
                continue;
            }
            let key = (r[3].to_bits(), r[13] as i64);
            *counts.entry(key).or_default() += 1;
        }
        let mut rows: Vec<((u64, i64), usize)> = counts.into_iter().collect();
        rows.sort_by(|a, b| a.0.0.cmp(&b.0.0).then(a.0.1.cmp(&b.0.1)));
        for ((s, m), n) in rows {
            println!(
                "{:>4} | {:>10.1} | {:>6} Samples, Mode {m}",
                st,
                f64::from_bits(s),
                n
            );
        }
    }
    println!();

    let mut series: std::collections::HashMap<(i64, usize), Vec<(i64, f64)>> =
        std::collections::HashMap::new();
    for st in STATIONS {
        for (ci, (_name, slot, ws, wm)) in classes.iter().enumerate() {
            series.insert((st, ci), monthly_medians(&records, st, *slot, *ws, *wm));
        }
    }

    for (ci, (name, _slot, _ws, _wm)) in classes.iter().enumerate() {
        println!("Class {name}: monthly-median series per station — n months, span, linear drift:");
        println!(
            "{:>4} | {:>5} | {:>22} | {:>14} | {:>14}",
            "Sta", "n", "span", "drift", "se"
        );
        for st in STATIONS {
            let m = &series[&(st, ci)];
            if m.is_empty() {
                println!("{st:>4} | no samples in the class (0 honored)");
                continue;
            }
            let xs: Vec<f64> = m.iter().map(|x| x.0 as f64).collect();
            let ys: Vec<f64> = m.iter().map(|x| x.1).collect();
            let (a2, b2) = lin_fit(&xs, &ys);
            let n = xs.len() as f64;
            let xm = xs.iter().sum::<f64>() / n;
            let sxx: f64 = xs.iter().map(|x| (x - xm).powi(2)).sum();
            let mut resid = 0.0f64;
            for (x, y) in xs.iter().zip(&ys) {
                let r = y - a2 * x - b2;
                resid += r * r;
            }
            let se = if n > 2.0 && sxx > 0.0 {
                (resid / ((n - 2.0) * sxx)).sqrt()
            } else {
                f64::NAN
            };
            println!(
                "{:>4} | {:>5} | {:>22} | {:>10.4} mHz/mo | {:>10.4}",
                st,
                m.len(),
                span_text(m[0].0, m[m.len() - 1].0),
                1e3 * a2,
                1e3 * se
            );
        }
        println!();
        for (i, a) in STATIONS.iter().enumerate() {
            for b in &STATIONS[i + 1..] {
                let ma = &series[&(*a, ci)];
                let mb = &series[&(*b, ci)];
                if ma.is_empty() || mb.is_empty() {
                    continue;
                }
                let va: std::collections::HashMap<i64, f64> = ma.iter().copied().collect();
                let vb: std::collections::HashMap<i64, f64> = mb.iter().copied().collect();
                let mut xs: Vec<f64> = Vec::new();
                let mut ys: Vec<f64> = Vec::new();
                for (mo, v) in &va {
                    if let Some(w) = vb.get(mo) {
                        xs.push(*v);
                        ys.push(*w);
                    }
                }
                if xs.len() < MIN_MONTHS {
                    println!(
                        "Sta {a}↔{b}: only {} shared months (< {MIN_MONTHS}) — still (0 honored)",
                        xs.len()
                    );
                    continue;
                }
                let xf: Vec<f32> = xs.iter().map(|v| *v as f32).collect();
                let yf: Vec<f32> = ys.iter().map(|v| *v as f32).collect();
                let ab = topological_te_phase(&xf, &yf, 3, 3, SEED ^ (*a as u64));
                let ba = topological_te_phase(&yf, &xf, 3, 3, SEED ^ (*b as u64));
                println!("Sta {a}↔{b}: {} shared months", xs.len());
                let pairs = [(format!("TE({a}→{b})"), &ab), (format!("TE({b}→{a})"), &ba)];
                for (label, v) in pairs {
                    match v {
                        Some(t) => {
                            let word = if t.te > t.threshold {
                                "ARROW (over thr)"
                            } else {
                                "still"
                            };
                            println!(
                                "  {label}: te {:.4e} vs thr {:.4e} ({} Surrogate, τ_x {} τ_y {}) — {}",
                                t.te, t.threshold, t.surrogates_used, t.tau_x, t.tau_y, word
                            );
                        }
                        None => println!(
                            "  {label}: no MI-τ — the phase space carries no coupling (still, 0 honored)"
                        ),
                    }
                }
            }
        }
        println!();
    }
    println!(
        "fam/fhr: TE over the surrogate threshold (mean+2σ, phase-randomized). A shared driver (calibration cadence) showed itself as symmetric coupling; a one-way coupling would be the station chain itself. n = monthly medians, no interpolation."
    );
}
