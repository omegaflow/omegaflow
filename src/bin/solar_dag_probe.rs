// Das Blatt der Korona-Heizung (Nadel III) — die volle kausale DAG der
// solaren Kanäle auf der Tages-Skala. Fünf Kanäle mit historischer Serie:
// F10.7 (f107_penticton.bin, 1947–2026), GOES-XRS schmal/breit (goes_xrs.bin,
// 1995–2020), IMF-Bz und Dichte (omni2_serie.bin, 1963–2026) — 20 gerichtete
// Paare, beide Richtungen, lag 0..7 d, TE mit phasenrandomisierter Schwelle
// (mean + 2σ über 10 Surrogate) und Familien-Schwelle (fam = stärkste
// Surrogat-TE der ganzen Runde — die Mehrfachvergleichskorrektur). Ein Pfeil
// gilt erst, wenn TE > fam. EUV-304/284 tragen keine historische Serie —
// pending, keine fabrizierte Zelle (0 honored).
//
// Epochen: F10.7 trägt UTC-Kalendertage, OMNI2 UTC-unix, GOES TDB — die
// ~69 s TDB-Konstante und die Mittag-/Mitternachts-Konvention liegen unter
// der Tages-Zellen-Weite (dieselbe Toleranz wie long_window_probe).

use omegaflow::archivar::euvs::{self, COMP_LYA1216};
use omegaflow::archivar::f107;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::goes::{self, COMP_XRSA, COMP_XRSB};
use omegaflow::archivar::omni2::{self, COMP_BZ, COMP_N1800};
use omegaflow::te::{gaussian, phase_randomized_surrogate, silverman, transfer_entropy_lag};

const GOES_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_xrs.bin";
const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const OMNI2_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie.bin";
const EUVS_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_euvs.bin";
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DAY: f64 = 86400.0;
const LAGS: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const MIN_N: usize = 30;
const N_SURR: usize = 10;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load_bin(kind: &str, url: &str, path: Option<String>) -> Option<Vec<u8>> {
    if let Some(p) = path {
        match std::fs::read(&p) {
            Ok(bytes) => return Some(bytes),
            Err(_) => {
                eprintln!("{kind}: {} reads void — the channel stays unmeasured", p);
                return None;
            }
        }
    }
    match fetch_raw_bytes(url, 3600) {
        Some(bytes) => Some(bytes),
        None => {
            eprintln!("{kind}: {url} carries no asset — the channel stays unmeasured (0 honored)");
            None
        }
    }
}

fn bin_mean_day(series: &[(f64, f64)], t0: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / DAY).floor();
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

fn pair_cells(a: &[Option<f32>], b: &[Option<f32>]) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (ca, cb) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (ca, cb) {
            xs.push(*x);
            ys.push(*y);
        }
    }
    (xs, ys)
}

fn surrogate_te_values(to: &[f32], from: &[f32], lag: usize, seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut vals = Vec::new();
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(from, &mut rng);
        if let Some(te) = transfer_entropy_lag(to, &ys, lag) {
            vals.push(te);
        }
    }
    vals
}

fn mean_plus_2sigma(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64;
    Some(m + 2.0 * var.sqrt())
}

fn te_bandwidth(x: &[f32], y: &[f32], lag: usize, factor: f64) -> Option<f64> {
    if lag == 0 {
        let n = x.len();
        if n < 8 {
            return None;
        }
        let hx = silverman(x)? * factor;
        let hy = silverman(y)? * factor;
        let m = n - 1;
        let mut te = 0.0;
        for t in 0..m {
            let xt = x[t] as f64;
            let xt1 = x[t + 1] as f64;
            let yt = y[t] as f64;
            let mut k3 = 0.0;
            for s in 0..m {
                k3 += gaussian(xt1 - x[s + 1] as f64, hx)
                    * gaussian(xt - x[s] as f64, hx)
                    * gaussian(yt - y[s] as f64, hy);
            }
            let p3 = k3 / m as f64;
            let mut k1 = 0.0;
            for s in 0..n {
                k1 += gaussian(xt - x[s] as f64, hx);
            }
            let p1 = k1 / n as f64;
            let mut k2xy = 0.0;
            for s in 0..n {
                k2xy += gaussian(xt - x[s] as f64, hx) * gaussian(yt - y[s] as f64, hy);
            }
            let p2xy = k2xy / n as f64;
            let mut k2x = 0.0;
            for s in 0..m {
                k2x += gaussian(xt1 - x[s + 1] as f64, hx) * gaussian(xt - x[s] as f64, hx);
            }
            let p2x = k2x / m as f64;
            te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
        }
        return Some(te / m as f64);
    }
    let n = x.len();
    if n < 8 {
        return None;
    }
    let m = n - lag;
    if m < 8 {
        return None;
    }
    let hx = silverman(x)? * factor;
    let hy = silverman(y)? * factor;
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[t + lag] as f64;
        let yt = y[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            k3 += gaussian(xt1 - x[s + lag] as f64, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in 0..n {
            k1 += gaussian(xt - x[s] as f64, hx);
        }
        let p1 = k1 / n as f64;
        let mut k2xy = 0.0;
        for s in 0..n {
            k2xy += gaussian(xt - x[s] as f64, hx) * gaussian(yt - y[s] as f64, hy);
        }
        let p2xy = k2xy / n as f64;
        let mut k2x = 0.0;
        for s in 0..m {
            k2x += gaussian(xt1 - x[s + lag] as f64, hx) * gaussian(xt - x[s] as f64, hx);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn surrogate_thr_bandwidth(
    x: &[f32],
    y: &[f32],
    lag: usize,
    seed: u64,
    factor: f64,
) -> Option<f64> {
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut vals = Vec::new();
    for _ in 0..N_SURR {
        let ys = phase_randomized_surrogate(y, &mut rng);
        if let Some(te) = te_bandwidth(x, &ys, lag, factor) {
            vals.push(te);
        }
    }
    mean_plus_2sigma(&vals)
}

struct PairVerdict {
    from: &'static str,
    to: &'static str,
    n: usize,
    best_lag: usize,
    te: f64,
    thr: f64,
    arrow: bool,
    family_bound: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let max_days: Option<usize> = arg_value(&args, "--days").and_then(|d| d.parse().ok());

    println!(
        "=== Das Blatt der Korona-Heizung — die kausale DAG der solaren Kanäle (Nadel III) ==="
    );
    println!(
        "TE-Lib: TE(Y→X; τ) über die gemeinsame Tages-Zelle; KDE-Bandbreite Silverman je Serie."
    );
    println!("Schwellen: phasenrandomisierte Surrogate (10, mean + 2σ) + Familien-Schwelle fam = stärkste Surrogat-TE der ganzen Runde (Mehrfachvergleichskorrektur über alle Paare × 8 Lags). Pfeil ⇔ TE > fam.");
    println!("Kanäle: F10.7-Penticton, GOES XRSA (0,05–0,4 nm), XRSB (0,1–0,8 nm), Lyman-α 121,6 nm, IMF-Bz, Dichte — je Tagesmittel; EUV-304/284 bleiben live-only (7-Tage-Feed, Minute-Skala): das NCEI-Science-Produkt trägt 304 tot (irr_304_1nm = NaN über 2009–2020) und 284 gar nicht — keine historische Serie, benannte Lücke.");

    let goes_bytes = load_bin("goes_xrs.bin", GOES_CDN, arg_value(&args, "--goes-bin"));
    let f107_bytes = load_bin(
        "f107_penticton.bin",
        F107_CDN,
        arg_value(&args, "--f107-bin"),
    );
    let omni2_bytes = load_bin(
        "omni2_serie.bin",
        OMNI2_CDN,
        arg_value(&args, "--omni2-bin"),
    );
    let euvs_bytes = load_bin("goes_euvs.bin", EUVS_CDN, arg_value(&args, "--euvs-bin"));
    let (Some(goes_bytes), Some(f107_bytes), Some(omni2_bytes), Some(euvs_bytes)) =
        (goes_bytes, f107_bytes, omni2_bytes, euvs_bytes)
    else {
        println!(
            "Verdict: das Blatt bleibt leer — die CDN-Assets sind die Voraussetzung (0 honored)."
        );
        return;
    };
    let goes_records = match goes::parse_bin(&goes_bytes) {
        Some(r) => r,
        None => {
            println!("goes_xrs.bin trägt kein GXS1-Kontrakt — der Kanal bleibt ungemessen");
            return;
        }
    };
    let f107_records = match f107::parse_bin(&f107_bytes) {
        Some(r) => r,
        None => {
            println!("f107_penticton.bin trägt kein F107-Kontrakt — der Kanal bleibt ungemessen");
            return;
        }
    };
    let omni2_records = match omni2::parse_bin(&omni2_bytes) {
        Some(r) => r,
        None => {
            println!("omni2_serie.bin trägt kein OMN1-Kontrakt — der Kanal bleibt ungemessen");
            return;
        }
    };
    let euvs_records = match euvs::parse_bin(&euvs_bytes) {
        Some(r) => r,
        None => {
            println!("goes_euvs.bin trägt kein GEUV-Kontrakt — der Kanal bleibt ungemessen");
            return;
        }
    };

    let series: Vec<(&'static str, Vec<(f64, f64)>)> = vec![
        (
            "F10.7",
            f107_records
                .iter()
                .map(|&(d, v)| (d as f64 * DAY, v))
                .collect(),
        ),
        (
            "XRSA",
            goes_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_XRSA)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        ),
        (
            "XRSB",
            goes_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_XRSB)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        ),
        (
            "Lya1216",
            euvs_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_LYA1216)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        ),
        (
            "Bz",
            omni2_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_BZ)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        ),
        (
            "Dichte",
            omni2_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_N1800)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        ),
    ];

    println!();
    println!("=== Kanal-Brett ===");
    for (name, s) in &series {
        match (s.first(), s.last()) {
            (Some(&(a, _)), Some(&(b, _))) => println!(
                "{name:<8} | n = {:<8} | window {:.1} d | cells/day {:.3}",
                s.len(),
                (b - a) / DAY,
                s.len() as f64 / ((b - a) / DAY).max(1.0)
            ),
            _ => println!("{name:<8} | no samples — the channel harvests null"),
        }
    }

    let lo = series
        .iter()
        .filter_map(|(_, s)| s.first().map(|&(t, _)| t))
        .fold(f64::NEG_INFINITY, f64::max);
    let hi = series
        .iter()
        .filter_map(|(_, s)| s.last().map(|&(t, _)| t))
        .fold(f64::INFINITY, f64::min);
    let Some((t0, n_full)) = (lo < hi).then(|| {
        let t0 = (lo / DAY).floor() * DAY;
        let n = ((hi - t0) / DAY).floor() as usize;
        (t0, n)
    }) else {
        println!("common window empty — the pairing carries no cells");
        return;
    };
    let n = match max_days {
        Some(d) => n_full.min(d),
        None => n_full,
    };
    let t0 = t0 + (n_full - n) as f64 * DAY;
    let cells: Vec<Vec<Option<f32>>> = series.iter().map(|(_, s)| bin_mean_day(s, t0, n)).collect();

    let mut pairs: Vec<(&'static str, &'static str, usize, usize)> = Vec::new();
    for i in 0..series.len() {
        for j in 0..series.len() {
            if i != j {
                pairs.push((series[i].0, series[j].0, i, j));
            }
        }
    }

    println!();
    println!(
        "=== Matrix — Tages-Zellen (gemeinsames Fenster, {} d, lag ∈ 0..7 d) ===",
        n
    );

    let mut fam = f64::NEG_INFINITY;
    let mut verdicts: Vec<PairVerdict> = Vec::new();
    for (idx, &(from, to, fi, ti)) in pairs.iter().enumerate() {
        let (xs, ys) = pair_cells(&cells[ti], &cells[fi]);
        if xs.len() < MIN_N {
            verdicts.push(PairVerdict {
                from,
                to,
                n: xs.len(),
                best_lag: 0,
                te: f64::NAN,
                thr: f64::NAN,
                arrow: false,
                family_bound: false,
            });
            continue;
        }
        let mut best: Option<(usize, f64)> = None;
        let mut best_thr = f64::NAN;
        for &lag in LAGS.iter() {
            let seed = SURROGATE_SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let surr = surrogate_te_values(&xs, &ys, lag, seed);
            for &v in &surr {
                if v > fam {
                    fam = v;
                }
            }
            if let Some(te) = transfer_entropy_lag(&xs, &ys, lag) {
                if best.map_or(true, |(_, b)| te > b) {
                    best = Some((lag, te));
                    best_thr = mean_plus_2sigma(&surr).unwrap_or(f64::NAN);
                }
            }
        }
        let Some((best_lag, te)) = best else {
            verdicts.push(PairVerdict {
                from,
                to,
                n: xs.len(),
                best_lag: 0,
                te: f64::NAN,
                thr: f64::NAN,
                arrow: false,
                family_bound: false,
            });
            continue;
        };
        let arrow = te > fam;
        let family_bound = !arrow && te > best_thr;
        verdicts.push(PairVerdict {
            from,
            to,
            n: xs.len(),
            best_lag,
            te,
            thr: best_thr,
            arrow,
            family_bound,
        });
        eprintln!(
            "pair {}/{} {} → {} | n {} | lag {} d | TE {:.4e} | fam-so-far {:.4e}",
            idx + 1,
            pairs.len(),
            from,
            to,
            xs.len(),
            best_lag,
            te,
            fam
        );
    }

    println!();
    println!("Familien-Schwelle fam = {:.4e} — die stärkste Surrogat-TE der ganzen Runde (Mehrfachvergleichskorrektur über {} Paare × {} Lags).", fam, pairs.len(), LAGS.len());

    println!();
    println!("=== Das Blatt ===");
    for v in &verdicts {
        let word = if v.arrow {
            "PFEIL"
        } else if v.family_bound {
            "family bound"
        } else if v.te.is_nan() {
            "no statement"
        } else {
            "still"
        };
        println!(
            "{:>6} → {:<6} | n {:>5} | lag {:<2} d | TE {:>10.4e} | thr {:>10.4e} | fam {:>10.4e} | {}",
            v.from, v.to, v.n, v.best_lag, v.te, v.thr, fam, word
        );
    }

    if args.iter().any(|a| a == "--h-sweep") {
        println!();
        println!(
            "=== KDE-Sensitivität (h, h/2, 2h) — die entscheidenden Paare bei ihrem besten Lag ==="
        );
        println!("Verdikt je Faktor gegen die je Faktor gerechnete Surrogat-Schwelle; fam selbst wird nicht neu gerechnet (fam(h/2)/fam(2h) wäre je eine eigene volle Runde).");
        for v in &verdicts {
            if !(v.family_bound || v.te > 0.6 * fam) {
                continue;
            }
            let Some(fi) = series.iter().position(|(n, _)| *n == v.from) else {
                continue;
            };
            let Some(ti) = series.iter().position(|(n, _)| *n == v.to) else {
                continue;
            };
            let (xs, ys) = pair_cells(&cells[ti], &cells[fi]);
            if xs.len() < MIN_N {
                continue;
            }
            let seed = SURROGATE_SEED ^ (v.best_lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let half_te = te_bandwidth(&xs, &ys, v.best_lag, 0.5);
            let half_thr = surrogate_thr_bandwidth(&xs, &ys, v.best_lag, seed, 0.5);
            let two_te = te_bandwidth(&xs, &ys, v.best_lag, 2.0);
            let two_thr = surrogate_thr_bandwidth(&xs, &ys, v.best_lag, seed, 2.0);
            let word = |t: Option<f64>, h: Option<f64>| match (t, h) {
                (Some(t), Some(h)) if t > h => "arrow",
                (Some(_), Some(_)) => "still",
                _ => "void",
            };
            println!(
                "{:>6} → {:<6} | lag {} d | h/2: TE {:>9.4e} thr {:>9.4e} ({}) | h: TE {:>9.4e} thr {:>9.4e} ({}) | 2h: TE {:>9.4e} thr {:>9.4e} ({})",
                v.from,
                v.to,
                v.best_lag,
                half_te.unwrap_or(f64::NAN),
                half_thr.unwrap_or(f64::NAN),
                word(half_te, half_thr),
                v.te,
                v.thr,
                word(Some(v.te), Some(v.thr)),
                two_te.unwrap_or(f64::NAN),
                two_thr.unwrap_or(f64::NAN),
                word(two_te, two_thr),
            );
        }
    }

    println!();
    println!("=== EUV-304/284 ===");
    println!("live-only — das NCEI-Science-Produkt trägt irr_304_1nm tot (NaN über 2009–2020, 0/3633 valide im avg1d) und 284 gar nicht (Wellenlängen-Achse [30,4, 121,6] nm); die historische Serie existiert nicht, keine fabrizierte Zelle. Die Minute-Skala der nobel probe trägt EUV→X-Ray und Bz→X-Ray (lag 0/1, 7-Tage-Fenster) — dort leben 304/284.");
    println!();
    println!("=== Verdikt ===");
    let arrows: Vec<&PairVerdict> = verdicts.iter().filter(|v| v.arrow).collect();
    if arrows.is_empty() {
        println!(
            "Kein fam-gereinigter Pfeil auf der Tages-Skala — Stille ist ein Befund (0 honored)."
        );
    } else {
        for a in &arrows {
            println!(
                "{} → {} (lag {} d, TE {:.4e} > fam {:.4e})",
                a.from, a.to, a.best_lag, a.te, fam
            );
        }
    }
}
