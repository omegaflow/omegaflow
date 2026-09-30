use omegaflow::archivar::extract::geo_series_parse_bin;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_ERSSTV5, GeoRec};
use omegaflow::archivar::omni_hro::{COMP_IMF_BZ_GSM, parse_bin};
use omegaflow::te::{
    BLATT_N_FLOOR, BlattPairSpec, current_commit_sha, surrogate_max_phase_n,
    surrogate_stats_phase_n, transfer_entropy_lag, write_blatt_pair,
};
use std::collections::HashMap;

const OMNI_HRO_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/cdaweb.gsfc.nasa.gov/omni_hro_1min.bin";
const ERSSTV5_CDN: &str = "https://github.com/omegaflow/sources/releases/download/coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin";
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 100;
const MAX_LAG_MONTHS: usize = 12;
const MONTH_S: f64 = 2_592_000.0;
const MIN_PAIRED: usize = 30;

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

fn bin_bz_monthly(bz: &[(f64, f64)], months: &[f64]) -> Vec<Option<f64>> {
    let mut sums = vec![0.0f64; months.len()];
    let mut counts = vec![0u32; months.len()];
    let mut mi = 0usize;
    for &(t, v) in bz {
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
                        "{from:>3} → {to:<3} | best lag {lag:>2} months | TE {te:.4e} | threshold {thr:.4e} (surrogate mean {mean:.3e}, σ {sd:.3e}) | fam {fam_s} | excess {:+.4e} | {verdict}",
                        te - thr
                    )
                }
                None => format!(
                    "{from:>3} → {to:<3} | best lag {lag:>2} months | TE {te:.4e} | threshold absent (surrogates < 2)"
                ),
            }
        }
        None => format!("{from:>3} → {to:<3} | TE absent (n < 8) — the pair stays unmeasured"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    println!("=== ENSO Blatt probe — the directional driver of the NINO3.4 SST anomaly ===");
    println!(
        "Pair: Bz (omni_hro_imf_bz_gsm_nt, OMNI_HRO_1MIN, hourly) × SST (ersstv5_nino34_ssta, ERSSTv5, monthly)."
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
    println!("Time base: TDB seconds since J2000; TE is shift-invariant.");

    let Some(bz) = load_bz() else {
        eprintln!("Bz series carries no records — the run stays unmeasured");
        return;
    };
    let Some(sst) = load_sst() else {
        eprintln!("SST series carries no records — the run stays unmeasured");
        return;
    };

    println!();
    println!("=== channel board ===");
    window_report("Bz", &bz);
    window_report("SST", &sst);

    let months: Vec<f64> = sst.iter().map(|&(t, _)| t).collect();
    let sst_vals: Vec<f64> = sst.iter().map(|&(_, v)| v).collect();
    let bz_month = bin_bz_monthly(&bz, &months);

    let mut sst_paired: Vec<f32> = Vec::new();
    let mut bz_paired: Vec<f32> = Vec::new();
    for i in 0..months.len() {
        if let Some(b) = bz_month[i] {
            if b.is_finite() {
                sst_paired.push(sst_vals[i] as f32);
                bz_paired.push(b as f32);
            }
        }
    }
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
        "=== Lag sweep ({} months, monthly grid) ===",
        MAX_LAG_MONTHS
    );
    println!("{:>4} | {:>14} | {:>14}", "lag", "TE(Bz→SST)", "TE(SST→Bz)");
    for &lag in &lags {
        let b2s = transfer_entropy_lag(&sst_paired, &bz_paired, lag);
        let s2b = transfer_entropy_lag(&bz_paired, &sst_paired, lag);
        println!("{lag:>4} | {} | {}", fmt_opt(b2s), fmt_opt(s2b));
    }

    let mut fam: Option<f64> = None;
    for &lag in &lags {
        for (x, y) in [(&sst_paired, &bz_paired), (&bz_paired, &sst_paired)] {
            if let Some(m) = surrogate_max_phase_n(x, y, lag, SURROGATE_SEED, N_SURR) {
                fam = Some(fam.map_or(m, |f| f.max(m)));
            }
        }
    }

    println!();
    match fam {
        Some(f) => println!(
            "=== Family bound (round max over Bz↔SST pair × lags, {N_SURR} surrogates) === fam = {f:.4e}"
        ),
        None => println!("=== Family bound absent (surrogates < 2) ==="),
    }
    println!("=== Verdict (per-lag threshold = mean + 2σ; arrow iff TE > fam) ===");
    let b2s_line = direction_verdict("Bz", "SST", &sst_paired, &bz_paired, &lags, fam);
    let s2b_line = direction_verdict("SST", "Bz", &bz_paired, &sst_paired, &lags, fam);
    println!("{b2s_line}");
    println!("{s2b_line}");

    println!();
    println!("=== THE BLATT ===");
    println!("Title: the directional driver of the NINO3.4 SST anomaly.");
    println!(
        "Pair: TE(Bz → SST) vs TE(SST → Bz), monthly grid, lag sweep 0–{MAX_LAG_MONTHS} months."
    );
    println!("Urteil: {b2s_line} / {s2b_line}");
    println!(
        "Schwelle: per-lag mean + 2σ over phase-randomized surrogates (f64 FFT, {N_SURR}) plus the round-max family bound fam (§3.2 rule)."
    );
    println!(
        "Fenster: n = {} months | SST 1854-01-01…2026-08-01 (ERSSTv5, cut NINO3.4) | Bz OMNI_HRO_1MIN hourly → monthly | φ/sources.φ omni_hro_imf_bz_gsm_nt × ersstv5_nino34_ssta.",
        sst_paired.len()
    );

    println!();
    println!("=== Missing register (named, not concealed) ===");
    println!(
        "Multiple-comparison correction: fam = round max over the Bz↔SST pair × lags (built, §3.2 rule)."
    );
    println!(
        "Wind channel pending: `tao_wnd_zonal.csv` is a 120-day live window (compiler fetches d_end-120d…d_end-7d); the 1977+ record needs a compiler extension (Mountain)."
    );
    println!(
        "LAIC channel pending: the comcat catalog asset is absent (compiler duty, Mountain/Mycelium)."
    );
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
