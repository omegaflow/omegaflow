use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::extract::geo_series_parse_bin;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_ERSSTV5, GeoRec};
use omegaflow::archivar::omni_hro::{COMP_IMF_BZ_GSM, parse_bin};
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
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const N_SURR: usize = 100;
const MAX_LAG_MONTHS: usize = 12;
const MONTH_S: f64 = 2_592_000.0;
const MIN_PAIRED: usize = 30;
const J2000_UNIX_OFFSET: f64 = 946_728_000.0;
const SECS_PER_DAY: f64 = 86_400.0;

const CH_WND: usize = 0;
const CH_QUAKE: usize = 1;
const CH_BZ: usize = 2;
const CH_SST: usize = 3;
const CH_NAMES: [&str; 4] = ["Wnd", "Quake", "Bz", "SST"];
const CH_MEDIA: [&str; 4] = ["atmos", "litho", "helios", "ocean"];

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    println!("=== ENSO Blatt probe — the directional driver of the NINO3.4 SST anomaly ===");
    println!(
        "Channels (Council 2026-09-30 cut): 1·Wnd (tao_wnd_zonal, atmos) · 2·Quake (usgs_comcat_m45, litho) · 3·Bz (omni_hro_imf_bz_gsm_nt, helios) — target SST (ersstv5_nino34_ssta, ocean)."
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
        "fam = round max over the measured directed pairs × lags (up to 12 = 4 × 3 ordered pairs over {{Wnd, Quake, Bz, SST}}; the target is counted; a pending channel's pairs stay unmeasured)."
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
    let quake: Option<Vec<(f64, f64)>> = None;

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
    window_report("Bz", &bz);
    window_report("SST", &sst);

    let months: Vec<f64> = sst.iter().map(|&(t, _)| t).collect();
    let sst_vals: Vec<f64> = sst.iter().map(|&(_, v)| v).collect();
    let bz_month = bin_monthly(&bz, &months);
    let wind_month = wind.as_ref().map(|s| bin_monthly(s, &months));
    let quake_month = quake.as_ref().map(|s| bin_monthly(s, &months));
    let sst_month: Vec<Option<f64>> = sst_vals.iter().map(|v| Some(*v)).collect();

    let channels: [Option<Vec<Option<f64>>>; 4] =
        [wind_month, quake_month, Some(bz_month), Some(sst_month)];

    println!();
    println!("=== monthly channel board (aligned to the SST grid) ===");
    println!("{:<6} | {:<7} | {:>7} | state", "chan", "medium", "n_mon");
    for i in 0..4 {
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
    for from in 0..4 {
        for to in 0..4 {
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
    println!(
        "=== Directed-path census and verdict (up to 12 ordered pairs = 4 × 3 over Wnd, Quake, Bz, SST; target counted; per-channel pending named; per-lag threshold = mean + 2σ; arrow iff TE > fam) ==="
    );
    for from in 0..4 {
        for to in 0..4 {
            if from == to {
                continue;
            }
            let (from_name, to_name) = (CH_NAMES[from], CH_NAMES[to]);
            let pair_pending = |label: &str| {
                let reason = if !channel_active(&channels[from]) {
                    channel_pending_reason(&channels[from], label)
                } else if !channel_active(&channels[to]) {
                    channel_pending_reason(&channels[to], label)
                } else {
                    format!("paired n < floor {MIN_PAIRED}")
                };
                println!("{from_name:>5} → {to_name:<5} | pending ({reason})");
            };
            if channel_active(&channels[from]) && channel_active(&channels[to]) {
                let (Some(a), Some(b)) = (channels[from].as_ref(), channels[to].as_ref()) else {
                    pair_pending(from_name);
                    continue;
                };
                let (from_s, to_s) = paired_series(a, b);
                if to_s.len() >= MIN_PAIRED {
                    let line = direction_verdict(from_name, to_name, &to_s, &from_s, &lags, fam);
                    println!("{line}");
                    continue;
                }
            }
            pair_pending(from_name);
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
        "Annual cycle: the seasonal cycle (~12-month lag) drives Wnd, Bz and SST in common; fam's round max over lags 0–{MAX_LAG_MONTHS} includes the 12-month band, so a 12-month peak is the annual cycle, not a channel arrow."
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
            "Wnd channel pending: {}. `tao_wnd_zonal.csv` is a 120-day live window (compiler fetches d_end-120d…d_end-7d); the 1977+ record needs a compiler extension (Mountain).",
            channel_pending_reason(&channels[CH_WND], "Wnd")
        );
    }
    if channel_active(&channels[CH_QUAKE]) {
        println!("Quake channel measured.");
    } else {
        println!(
            "Quake channel pending: {}. The `usgs_comcat_m45.bin` parser/asset registration is Mountain's in-flight commit (the channel is read generically and named here until it lands).",
            channel_pending_reason(&channels[CH_QUAKE], "Quake")
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
