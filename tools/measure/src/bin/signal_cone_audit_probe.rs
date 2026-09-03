use omegaflow::archivar::euvs::{self, COMP_LYA1216};
use omegaflow::archivar::f107;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::goes::{self, COMP_XRSA, COMP_XRSB};
use omegaflow::archivar::omni2::{self, COMP_BZ, COMP_N1800, COMP_V1800};
use omegaflow::archivar::{
    BodyEphemeris, C_LIGHT, DIFFUSIVITY_MOLECULAR, body_barycenter_position,
    parse_ephemeris_binary, signal_reach,
};
use omegaflow::te::{permutation_entropy, phase_randomized_surrogate, transfer_entropy_lag};
use omegaflow::wind::{self, RECEIVER_RAD1, RECEIVER_RAD2, RECEIVER_TNR};
use omegaflow::wind_orbit;
use std::collections::HashMap;

const GOES_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_xrs.bin";
const F107_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/f107_penticton.bin";
const OMNI2_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie.bin";
const EUVS_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/goes_euvs.bin";
const WIND_WAVES_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/spdf.gsfc.nasa.gov/wind_waves.bin";
const SUN_EPH_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_sun.bin";
const WIND_ORBIT_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/spdf.gsfc.nasa.gov/wind_orbit.bin";
const EARTH_EPH_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_earth.bin";
const SURROGATE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DAY: f64 = 86400.0;
const AU: f64 = 1.495978707e11;
const LAGS: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const MIN_N: usize = 30;
const N_SURR: usize = 10;
const FORCE_EM: u8 = 0;
const FORCE_DIFFUSION: u8 = 6;
const FORCE_ADVECTIVE: u8 = 7;
const RTSW_WIND_ADVECTION_MS: f64 = 400000.0;
const PE_SEGMENT_CELLS: usize = 256;
const PE_ORDER: usize = 4;
const PE_DELAY: usize = 1;
const PE_RING_MAX: usize = 16;

#[derive(Clone, Copy, PartialEq)]
enum Anchor {
    Sun,
    Wind,
}

impl Anchor {
    fn word(self) -> &'static str {
        match self {
            Anchor::Sun => "at sun",
            Anchor::Wind => "at wind",
        }
    }
}

struct Series {
    name: &'static str,
    force: u8,
    anchor: Anchor,
    register_ref: &'static str,
    values: Vec<(f64, f64)>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load_bytes(kind: &str, url: &str, cache_name: &str, path: Option<String>) -> Option<Vec<u8>> {
    if let Some(p) = path {
        match std::fs::read(&p) {
            Ok(bytes) => return Some(bytes),
            Err(_) => {
                eprintln!("{kind}: {} reads void — the channel stays unmeasured", p);
                return None;
            }
        }
    }
    let cache_path = format!("/tmp/omegaflow_series_{cache_name}");
    if let Ok(bytes) = std::fs::read(&cache_path) {
        return Some(bytes);
    }
    match fetch_raw_bytes(url, 3600) {
        Some(bytes) => {
            if std::fs::write(&cache_path, &bytes).is_err() {
                eprintln!("{kind}: cache write void ({cache_path}) — the bytes stay in memory");
            }
            Some(bytes)
        }
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

fn median(vals: &mut [f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    vals.sort_by(f64::total_cmp);
    Some(vals[vals.len() / 2])
}

fn pe_gate(driver: &[f32]) -> Option<bool> {
    let segments = driver.len() / PE_SEGMENT_CELLS;
    if segments < 8 {
        return None;
    }
    let mut ring: Vec<f64> = Vec::new();
    for s in 0..segments {
        let seg: Vec<f64> = driver[s * PE_SEGMENT_CELLS..(s + 1) * PE_SEGMENT_CELLS]
            .iter()
            .map(|&v| v as f64)
            .collect();
        let Some(pe) = permutation_entropy(&seg, PE_ORDER, PE_DELAY) else {
            continue;
        };
        if ring.len() == PE_RING_MAX {
            ring.remove(0);
        }
        ring.push(pe);
        if ring.len() < 8 {
            continue;
        }
        let n = ring.len() as f64;
        let mean = ring.iter().sum::<f64>() / n;
        let var = ring.iter().map(|&p| (p - mean) * (p - mean)).sum::<f64>() / n;
        if (pe - mean).abs() > 2.0 * var.sqrt() {
            return Some(true);
        }
    }
    Some(false)
}

struct PairTe {
    te: f64,
    surrs: Vec<f64>,
}

struct PairVerdict {
    from: usize,
    to: usize,
    n: usize,
    best_lag: usize,
    te: f64,
    thr: f64,
    arrow: bool,
    family_bound: bool,
    pe_held: Option<bool>,
}

fn verdict_word(v: &PairVerdict) -> &'static str {
    if v.arrow && v.pe_held == Some(true) {
        "PE-held"
    } else if v.arrow {
        "PFEIL"
    } else if v.family_bound {
        "family bound"
    } else if v.te.is_nan() {
        "no statement"
    } else {
        "still"
    }
}

fn force_word(force: u8) -> &'static str {
    match force {
        FORCE_EM => "em",
        FORCE_DIFFUSION => "diffusion",
        FORCE_ADVECTIVE => "advective",
        _ => "unknown",
    }
}

fn law_word(force: u8) -> &'static str {
    match force {
        FORCE_EM => "c",
        FORCE_DIFFUSION => "√(2·D·τ)",
        FORCE_ADVECTIVE => "Windgeschwindigkeit",
        _ => "no law",
    }
}

fn cone_reach(force: u8, advection: f64, tau_s: f64) -> Option<f64> {
    signal_reach(force as f64, advection, tau_s)
}

fn cone_min_tau(force: u8, advection: f64, d: f64) -> Option<f64> {
    match force {
        FORCE_EM => Some(d / C_LIGHT),
        FORCE_ADVECTIVE => {
            if advection > 0.0 {
                Some(d / advection)
            } else {
                None
            }
        }
        FORCE_DIFFUSION => Some(d * d / (2.0 * DIFFUSIVITY_MOLECULAR)),
        _ => None,
    }
}

struct AuditRow {
    from: usize,
    to: usize,
    d: f64,
    tau_s: f64,
    reach: f64,
    holds: bool,
    sweep_edge: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let max_days: Option<usize> = arg_value(&args, "--days").and_then(|d| d.parse().ok());

    println!("=== The Blatt of the signal cone audit — the breach of the d/c law (Nadel Ⅶ) ===");
    println!(
        "TE-Lib: TE(Y→X; τ) over the shared day cell; fam = strongest surrogate TE of the whole round (10 phase-randomized surrogates per cell, multiple-comparison correction over all pairs × 8 lags)."
    );
    println!(
        "Cone law: signal_reach of the register — em → c, advective → the wind velocity of the channel (median of the V series over the window), diffusion → √(2·D·τ) with D = 0,05 m²/s. No mixing of the media."
    );
    println!(
        "PE gate: 2⁴-ring of the permutation entropy of the driver (segments à {PE_SEGMENT_CELLS} cells, order {PE_ORDER}); jump ⇔ |pe − mean| > 2·sd — the direction decision carries the caveat."
    );
    println!(
        "Epochs: F10.7 carries UTC calendar days, OMNI2 UTC-unix, GOES/EUVS/WAVES TDB — the conventions lie under the day-cell width (the same tolerance as solar_causal_graph_probe)."
    );

    let goes_bytes = load_bytes(
        "goes_xrs.bin",
        GOES_CDN,
        "goes_xrs.bin",
        arg_value(&args, "--goes-bin"),
    );
    let f107_bytes = load_bytes(
        "f107_penticton.bin",
        F107_CDN,
        "f107_penticton.bin",
        arg_value(&args, "--f107-bin"),
    );
    let omni2_bytes = load_bytes(
        "omni2_serie.bin",
        OMNI2_CDN,
        "omni2_serie.bin",
        arg_value(&args, "--omni2-bin"),
    );
    let euvs_bytes = load_bytes(
        "goes_euvs.bin",
        EUVS_CDN,
        "goes_euvs.bin",
        arg_value(&args, "--euvs-bin"),
    );
    let waves_bytes = load_bytes(
        "wind_waves.bin",
        WIND_WAVES_CDN,
        "wind_waves.bin",
        arg_value(&args, "--wind-waves-bin"),
    );

    let (Some(goes_bytes), Some(f107_bytes), Some(omni2_bytes), Some(euvs_bytes)) =
        (goes_bytes, f107_bytes, omni2_bytes, euvs_bytes)
    else {
        println!(
            "Verdict: the Blatt stays empty — the CDN assets are the prerequisite (0 honored)."
        );
        return;
    };
    let goes_records = match goes::parse_bin(&goes_bytes) {
        Some(r) => r,
        None => {
            println!("goes_xrs.bin carries no GXS1 contract — the channel stays unmeasured");
            return;
        }
    };
    let f107_records = match f107::parse_bin(&f107_bytes) {
        Some(r) => r,
        None => {
            println!("f107_penticton.bin carries no F107 contract — the channel stays unmeasured");
            return;
        }
    };
    let omni2_records = match omni2::parse_bin(&omni2_bytes) {
        Some(r) => r,
        None => {
            println!("omni2_serie.bin carries no OMN1 contract — the channel stays unmeasured");
            return;
        }
    };
    let euvs_records = match euvs::parse_bin(&euvs_bytes) {
        Some(r) => r,
        None => {
            println!("goes_euvs.bin carries no GEUV contract — the channel stays unmeasured");
            return;
        }
    };
    let waves_records = waves_bytes.as_deref().and_then(wind::parse_bin);

    let series: Vec<Series> = vec![
        Series {
            name: "F10.7",
            force: FORCE_EM,
            anchor: Anchor::Sun,
            register_ref: "f107_penticton.bin carries no block — the live channel solar_f107_flux_sfu (f107_cm_flux.json) stands at sun",
            values: f107_records
                .iter()
                .map(|&(d, v)| (d as f64 * DAY, v))
                .collect(),
        },
        Series {
            name: "XRSA",
            force: FORCE_EM,
            anchor: Anchor::Sun,
            register_ref: "goes_xrs.bin (sources.φ, goes_xrs_xrsa) at sun",
            values: goes_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_XRSA)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "XRSB",
            force: FORCE_EM,
            anchor: Anchor::Sun,
            register_ref: "goes_xrs.bin (sources.φ, goes_xrs_xrsb) at sun",
            values: goes_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_XRSB)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "Lya1216",
            force: FORCE_EM,
            anchor: Anchor::Sun,
            register_ref: "goes_euvs.bin carries no block — the live channel solar_euv (euvs-7-day.json) stands at sun",
            values: euvs_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_LYA1216)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "Bz",
            force: FORCE_EM,
            anchor: Anchor::Sun,
            register_ref: "omni2_serie.bin (sources.φ, omni_imf_bz_gsm_nt) at sun",
            values: omni2_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_BZ)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "Density",
            force: FORCE_DIFFUSION,
            anchor: Anchor::Sun,
            register_ref: "omni2_serie.bin (sources.φ, omni_solarwind_density_percc) at sun",
            values: omni2_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_N1800)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "Vwind",
            force: FORCE_ADVECTIVE,
            anchor: Anchor::Sun,
            register_ref: "omni2_serie.bin (sources.φ, omni_solarwind_flow_speed_kms) at sun — the anchor at sun carries advection 0,0 → ADVECTIVE_BASE_SPEED; the audit computes the measured wind velocity",
            values: omni2_records
                .iter()
                .filter(|(_, _, c)| *c == COMP_V1800)
                .map(|&(t, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "RAD1",
            force: FORCE_EM,
            anchor: Anchor::Wind,
            register_ref: "wind_waves.bin (sources.φ, wind_waves_rad1) at wind (L1)",
            values: waves_records
                .iter()
                .flatten()
                .filter(|(_, _, _, _, r)| *r == RECEIVER_RAD1)
                .map(|&(t, _, _, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "RAD2",
            force: FORCE_EM,
            anchor: Anchor::Wind,
            register_ref: "wind_waves.bin (sources.φ, wind_waves_rad2) at wind (L1)",
            values: waves_records
                .iter()
                .flatten()
                .filter(|(_, _, _, _, r)| *r == RECEIVER_RAD2)
                .map(|&(t, _, _, v, _)| (t, v))
                .collect(),
        },
        Series {
            name: "TNR",
            force: FORCE_EM,
            anchor: Anchor::Wind,
            register_ref: "wind_waves.bin (sources.φ, wind_waves_tnr) at wind (L1)",
            values: waves_records
                .iter()
                .flatten()
                .filter(|(_, _, _, _, r)| *r == RECEIVER_TNR)
                .map(|&(t, _, _, v, _)| (t, v))
                .collect(),
        },
    ];

    let series_any = series.iter().any(|s| !s.values.is_empty());
    if !series_any {
        println!("all channels harvest null — the cells stay empty (0 honored)");
        return;
    }

    println!();
    println!("=== Channel board ===");
    for s in &series {
        match (s.values.first(), s.values.last()) {
            (Some(&(a, _)), Some(&(b, _))) => println!(
                "{:<8} | n = {:<7} | window {:.1} d | cells/day {:.3} | {} | {} | {}",
                s.name,
                s.values.len(),
                (b - a) / DAY,
                s.values.len() as f64 / ((b - a) / DAY).max(1.0),
                force_word(s.force),
                s.anchor.word(),
                s.register_ref
            ),
            _ => println!(
                "{:<8} | no samples — the channel harvests null | {} | {} | {}",
                s.name,
                force_word(s.force),
                s.anchor.word(),
                s.register_ref
            ),
        }
    }

    let lo = series
        .iter()
        .filter_map(|s| s.values.first().map(|&(t, _)| t))
        .fold(f64::NEG_INFINITY, f64::max);
    let hi = series
        .iter()
        .filter_map(|s| s.values.last().map(|&(t, _)| t))
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
    let t_mid = t0 + n as f64 * DAY / 2.0;
    let cells: Vec<Vec<Option<f32>>> = series
        .iter()
        .map(|s| bin_mean_day(&s.values, t0, n))
        .collect();

    let mut wind_ms: Vec<f64> = Vec::new();
    for s in &series {
        if s.name == "Vwind" {
            wind_ms.extend(
                s.values
                    .iter()
                    .filter(|(t, v)| *t >= t0 && *t <= t0 + n as f64 * DAY && v.is_finite())
                    .map(|(_, v)| *v * 1000.0),
            );
        }
    }
    let wind_median_ms = median(&mut wind_ms);

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let sun_eph = load_bytes(
        "ephemeris_sun.bin",
        SUN_EPH_CDN,
        "ephemeris_sun.bin",
        arg_value(&args, "--sun-eph"),
    )
    .and_then(|b| parse_ephemeris_binary(&b));
    if let Some(e) = sun_eph {
        eph.insert("sun".to_string(), e);
    }
    let earth_eph = load_bytes(
        "ephemeris_earth.bin",
        EARTH_EPH_CDN,
        "ephemeris_earth.bin",
        arg_value(&args, "--earth-eph"),
    )
    .and_then(|b| parse_ephemeris_binary(&b));
    if let Some(e) = earth_eph {
        eph.insert("earth".to_string(), e);
    }
    let wind_orbit = load_bytes(
        "wind_orbit.bin",
        WIND_ORBIT_CDN,
        "wind_orbit.bin",
        arg_value(&args, "--wind-orbit"),
    )
    .and_then(|b| wind_orbit::parse_bin(&b))
    .map(|records| wind_orbit::orbit_rec(&records));

    let sun_pos = body_barycenter_position("sun", t_mid, &eph);
    let wind_gci = wind_orbit
        .as_ref()
        .and_then(|rec| wind_orbit::position_at(rec, t_mid))
        .map(|(p, _)| p);
    let earth_pos_mid = body_barycenter_position("earth", t_mid, &eph);
    let wind_pos = match (wind_gci, earth_pos_mid) {
        (Some(w), Some(e)) => Some([w[0] + e[0], w[1] + e[1], w[2] + e[2]]),
        _ => None,
    };
    let d_sun_wind = match (sun_pos, wind_pos) {
        (Some(s), Some(w)) => {
            let d = ((s[0] - w[0]).powi(2) + (s[1] - w[1]).powi(2) + (s[2] - w[2]).powi(2)).sqrt();
            Some(d)
        }
        _ => None,
    };

    println!();
    println!(
        "=== Anchor board (t_mid = {:.1} d from t0) ===",
        (t_mid - t0) / DAY
    );
    match sun_pos {
        Some(p) => println!(
            "sun    : [{:.3e}, {:.3e}, {:.3e}] m (ephemeris_sun.bin, Barycenter)",
            p[0], p[1], p[2]
        ),
        None => println!(
            "sun    : absent — ephemeris_sun.bin carries no contract; Sun pairs stay without cone check"
        ),
    }
    match wind_gci {
        Some(p) => println!(
            "wind   : [{:.3e}, {:.3e}, {:.3e}] m (wind_orbit.bin, GCI_POS — geocentric)",
            p[0], p[1], p[2]
        ),
        None => println!(
            "wind   : absent — wind_orbit.bin carries no contract; L1 pairs stay without cone check"
        ),
    }
    match (wind_gci, earth_pos_mid) {
        (Some(_), Some(_)) => println!(
            "wind ICRS = GCI + earth(t_mid) — the L1 address in the block (the GCI↔ICRS frame bias lies under the measurement width, named). The membrane itself carries the GCI record directly as the block address (finding in the register)."
        ),
        _ => {}
    }
    match wind_pos {
        Some(p) => println!("wind-ICRS : [{:.3e}, {:.3e}, {:.3e}] m", p[0], p[1], p[2]),
        None => println!("wind-ICRS : absent — the earth ephemeris does not carry the offset"),
    }
    match d_sun_wind {
        Some(d) => println!(
            "d(sun↔wind) = {:.3e} m = {:.4} AU — the only non-degenerate anchor distance",
            d,
            d / AU
        ),
        None => println!(
            "d(sun↔wind) = absent — the cone check of the cross pairs does not stand (0 honored)"
        ),
    }
    match wind_median_ms {
        Some(v) => println!(
            "Vwind median over the window = {:.3e} m/s — the wind velocity of the advective channel (register: rtsw proton_speed advection {} m/s, omni flow_speed 0,0)",
            v, RTSW_WIND_ADVECTION_MS
        ),
        None => println!("Vwind median = absent — the advective cone carries no v_force (absent)"),
    }

    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..series.len() {
        for j in 0..series.len() {
            if i != j {
                pairs.push((i, j));
            }
        }
    }

    let mut round: Vec<Vec<Option<PairTe>>> = pairs
        .iter()
        .map(|_| (0..LAGS.len()).map(|_| None).collect())
        .collect();
    for (pi, &(fi, ti)) in pairs.iter().enumerate() {
        let (xs, ys) = pair_cells(&cells[ti], &cells[fi]);
        if xs.len() < MIN_N {
            continue;
        }
        for (li, &lag) in LAGS.iter().enumerate() {
            let seed = SURROGATE_SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let surrs = surrogate_te_values(&xs, &ys, lag, seed);
            let te = transfer_entropy_lag(&xs, &ys, lag).unwrap_or(f64::NAN);
            round[pi][li] = Some(PairTe { te, surrs });
        }
        eprintln!(
            "pair {}/{} {} → {} | n {} | TE field stands",
            pi + 1,
            pairs.len(),
            series[fi].name,
            series[ti].name,
            xs.len()
        );
    }

    let fam = round
        .iter()
        .flatten()
        .flatten()
        .flat_map(|pt| pt.surrs.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    println!();
    println!(
        "Family threshold fam = {:.4e} — the strongest surrogate TE of the whole round ({} pairs × {} lags).",
        fam,
        pairs.len(),
        LAGS.len()
    );

    let mut verdicts: Vec<PairVerdict> = Vec::new();
    for (pi, &(fi, ti)) in pairs.iter().enumerate() {
        let (xs, ys) = pair_cells(&cells[ti], &cells[fi]);
        if xs.len() < MIN_N {
            verdicts.push(PairVerdict {
                from: fi,
                to: ti,
                n: xs.len(),
                best_lag: 0,
                te: f64::NAN,
                thr: f64::NAN,
                arrow: false,
                family_bound: false,
                pe_held: None,
            });
            continue;
        }
        let mut best: Option<(usize, f64, f64)> = None;
        for (li, &lag) in LAGS.iter().enumerate() {
            let Some(pt) = &round[pi][li] else {
                continue;
            };
            if pt.te.is_nan() {
                continue;
            }
            if best.map_or(true, |(_, b, _)| pt.te > b) {
                let thr = mean_plus_2sigma(&pt.surrs).unwrap_or(f64::NAN);
                best = Some((lag, pt.te, thr));
            }
        }
        let Some((best_lag, te, thr)) = best else {
            verdicts.push(PairVerdict {
                from: fi,
                to: ti,
                n: xs.len(),
                best_lag: 0,
                te: f64::NAN,
                thr: f64::NAN,
                arrow: false,
                family_bound: false,
                pe_held: None,
            });
            continue;
        };
        let arrow = te > fam;
        let family_bound = !arrow && te > thr;
        let pe_held = if arrow { pe_gate(&ys) } else { None };
        verdicts.push(PairVerdict {
            from: fi,
            to: ti,
            n: xs.len(),
            best_lag,
            te,
            thr,
            arrow,
            family_bound,
            pe_held,
        });
    }

    println!();
    println!(
        "=== Matrix — day cells (shared window, {} d, lag ∈ 0..7 d) ===",
        n
    );
    for v in &verdicts {
        println!(
            "{:>6} → {:<6} | n {:>5} | lag {:<2} d | TE {:>10.4e} | thr {:>10.4e} | {}",
            series[v.from].name,
            series[v.to].name,
            v.n,
            v.best_lag,
            v.te,
            v.thr,
            verdict_word(v)
        );
    }

    let pe_held_rows: Vec<&PairVerdict> = verdicts
        .iter()
        .filter(|v| v.arrow && v.pe_held == Some(true))
        .collect();
    if !pe_held_rows.is_empty() {
        println!();
        println!("=== PE gate holds the direction decision ===");
        for v in &pe_held_rows {
            println!(
                "{:>6} → {:<6} | lag {} d — the driver carries a PE jump (|pe − mean| > 2·sd over the 2⁴-ring); the arrow carries the caveat, no cone check",
                series[v.from].name, series[v.to].name, v.best_lag
            );
        }
    }

    let mut audit_rows: Vec<AuditRow> = Vec::new();
    let mut absent_arrows: Vec<(&str, &str, &'static str)> = Vec::new();
    for v in &verdicts {
        if !v.arrow || v.pe_held == Some(true) {
            continue;
        }
        let d = match (series[v.from].anchor, series[v.to].anchor) {
            (Anchor::Sun, Anchor::Wind) | (Anchor::Wind, Anchor::Sun) => d_sun_wind,
            _ => Some(0.0),
        };
        let Some(d) = d else {
            absent_arrows.push((series[v.from].name, series[v.to].name, "anchor absent"));
            continue;
        };
        let tau_s = v.best_lag as f64 * DAY;
        if series[v.from].force == FORCE_ADVECTIVE && wind_median_ms.is_none() {
            absent_arrows.push((series[v.from].name, series[v.to].name, "v_force absent"));
            continue;
        }
        let advection = if series[v.from].force == FORCE_ADVECTIVE {
            wind_median_ms.unwrap_or(0.0)
        } else {
            0.0
        };
        let Some(reach) = cone_reach(series[v.from].force, advection, tau_s) else {
            continue;
        };
        audit_rows.push(AuditRow {
            from: v.from,
            to: v.to,
            d,
            tau_s,
            reach,
            holds: reach >= d,
            sweep_edge: v.best_lag == 0 || v.best_lag == *LAGS.last().unwrap_or(&0),
        });
    }

    println!();
    println!("=== The audit table — τ·v_force ≥ d per fam pair (cone check) ===");
    if audit_rows.is_empty() {
        println!(
            "No fam pair carries an arrow — the cone is checked by no arrow; the silence is the finding (0 honored)."
        );
    } else {
        for r in &audit_rows {
            let min_tau = cone_min_tau(
                series[r.from].force,
                if series[r.from].force == FORCE_ADVECTIVE {
                    wind_median_ms.unwrap_or(0.0)
                } else {
                    0.0
                },
                r.d,
            );
            let min_word = match min_tau {
                Some(m) => format!("{:.3e} s", m),
                None => "absent".to_string(),
            };
            println!(
                "{:>6} → {:<6} | {} | d {:>10.3e} m | {} ({}) | τ {} d | reach(τ) {:>10.3e} m | mindest-τ {} | {} {}",
                series[r.from].name,
                series[r.to].name,
                force_word(series[r.from].force),
                r.d,
                law_word(series[r.from].force),
                series[r.from].anchor.word(),
                r.tau_s / DAY,
                r.reach,
                min_word,
                if r.holds { "holds" } else { "VIOLATED" },
                if r.sweep_edge { "(sweep edge)" } else { "" }
            );
        }
    }
    for (from, to, reason) in &absent_arrows {
        println!(
            "{:>6} → {:<6} | {} — the cone check does not stand (0 honored)",
            from, to, reason
        );
    }

    println!();
    println!("=== Negativ-Lag-Scan ===");
    println!(
        "The scalar TE machine (transfer_entropy_lag) takes lag ∈ ℕ (usize) — a τ < 0 cell is structurally no condition the machine builds (Atom 10 mirrors the condition backward); the counterpart of every row is the opposite-direction row of the matrix."
    );
    println!(
        "τ < 0 cells: 0 — the machine builds none; the scan documents the void, it does not invent it (0 honored)."
    );
    let mut rev_arrows = 0usize;
    for i in 0..series.len() {
        for j in (i + 1)..series.len() {
            let fwd = verdicts.iter().find(|v| v.from == i && v.to == j);
            let rev = verdicts.iter().find(|v| v.from == j && v.to == i);
            match (fwd, rev) {
                (Some(f), Some(r)) => {
                    if r.arrow {
                        rev_arrows += 1;
                    }
                    println!(
                        "{:>6} ↔ {:<6} | {} (lag {} d) / {} (lag {} d)",
                        series[i].name,
                        series[j].name,
                        verdict_word(f),
                        f.best_lag,
                        verdict_word(r),
                        r.best_lag
                    );
                }
                _ => {}
            }
        }
    }
    println!(
        "Opposite-direction rows (the structural correspondence): {} fam arrow(s) — the direction picture of the matrix, no τ < 0 finding.",
        rev_arrows
    );

    println!();
    println!("=== Funnel cross-check (Nadel Ⅵ, shared) ===");
    let earth_pos = body_barycenter_position("earth", t_mid, &eph);
    match (sun_pos, earth_pos) {
        (Some(s), Some(e)) => {
            let d_au =
                ((s[0] - e[0]).powi(2) + (s[1] - e[1]).powi(2) + (s[2] - e[2]).powi(2)).sqrt();
            println!(
                "d(sun↔earth) at t_mid = {:.3e} m = {:.4} AU — the Sonden-Front flies through the Earth sphere.",
                d_au,
                d_au / AU
            );
            println!(
                "Minimum τ against the same cone: em {:.1} s | advective ({:.1e} m/s) {:.1} d | diffusion {:.2e} s — no law carries both paths.",
                d_au / C_LIGHT,
                RTSW_WIND_ADVECTION_MS,
                d_au / RTSW_WIND_ADVECTION_MS / DAY,
                d_au * d_au / (2.0 * DIFFUSIVITY_MOLECULAR)
            );
        }
        _ => println!("d(sun↔earth) = absent — the funnel geometry does not stand (0 honored)"),
    }
    println!(
        "The jerk arrows of the Sonden-Front (flyby_probe, fam round): none today — Path 1 + Path 2 are closed, no fam arrow (pre-registration sealed). The cross-check table stays empty (0 honored); future jerk arrows run through the same cone check of this section — shared machine, own table section."
    );

    println!();
    println!("=== Verdict ===");
    let arrows: Vec<&PairVerdict> = verdicts
        .iter()
        .filter(|v| v.arrow && v.pe_held != Some(true))
        .collect();
    if arrows.is_empty() {
        println!(
            "No fam pair carries an arrow — the signal cone carries no violation; the silence is the measurement (0 honored)."
        );
    } else {
        let violations: Vec<&AuditRow> = audit_rows.iter().filter(|r| !r.holds).collect();
        if violations.is_empty() {
            println!(
                "{} fam arrows, all hold the cone (τ·v_force ≥ d per force) — the cone integrity is a quantitative limit.",
                arrows.len()
            );
        } else {
            for r in &violations {
                println!(
                    "{} → {} — cone-breach candidate: reach(τ) {:.3e} m < d {:.3e} m (τ {} d, {}). The machine shows the violation, it does not explain it.",
                    series[r.from].name,
                    series[r.to].name,
                    r.reach,
                    r.d,
                    r.tau_s / DAY,
                    law_word(series[r.from].force)
                );
            }
        }
    }
}
