use omegaflow::archivar::callisto::{self, BAND_COUNT};
use omegaflow::archivar::omni2;
use omegaflow::te::{gaussian, silverman};
use std::collections::BTreeMap;

const J2000_UNIX_OFFSET: f64 = 946_728_000.0;
const DAY: f64 = 86_400.0;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_N_SURR: usize = 500;
const BZ_FILL: f64 = 900.0;
const V_FILL: f64 = 2_000.0;
const MIN_HOURS: usize = 12;

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn day_from_iso(s: &str) -> Option<i64> {
    let mut p = s.split('-');
    let y: i64 = p.next()?.parse().ok()?;
    let m: i64 = p.next()?.parse().ok()?;
    let d: i64 = p.next()?.parse().ok()?;
    Some(days_from_civil(y, m, d))
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn iso_from_day(day: i64) -> String {
    let (y, m, d) = civil_from_days(day);
    format!("{y:04}-{m:02}-{d:02}")
}

fn year_of(day: i64) -> i64 {
    civil_from_days(day).0
}

fn read_u32(b: &[u8], o: usize) -> Option<u32> {
    b.get(o..o + 4)
        .and_then(|s| s.try_into().ok())
        .map(u32::from_le_bytes)
}

fn read_f64(b: &[u8], o: usize) -> Option<f64> {
    b.get(o..o + 8)
        .and_then(|s| s.try_into().ok())
        .map(f64::from_le_bytes)
}

fn median(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    })
}

fn read_ground(path: &str) -> Option<BTreeMap<i64, f64>> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let mut it = line.split_whitespace();
        let t: f64 = it.next()?.parse().ok()?;
        let v: f64 = it.next()?.parse().ok()?;
        if !t.is_finite() || !v.is_finite() {
            continue;
        }
        out.insert((t / DAY).floor() as i64, v);
    }
    Some(out)
}

fn read_omni(path: &str) -> Option<Vec<(f64, f64, u32)>> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.len() < 8 || &bytes[0..4] != b"OMN1" {
        return None;
    }
    let n = read_u32(&bytes, 4)? as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 20;
        let t = read_f64(&bytes, o)?;
        let v = read_f64(&bytes, o + 8)?;
        let c = read_u32(&bytes, o + 16)?;
        out.push((t, v, c));
    }
    Some(out)
}

fn read_sun(path: &str) -> Option<Vec<callisto::BandRecord>> {
    let bytes = std::fs::read(path).ok()?;
    callisto::parse_bin(&bytes)
}

fn sun_days(
    records: &[callisto::BandRecord],
    lo: i64,
    hi: i64,
) -> BTreeMap<i64, [f64; BAND_COUNT]> {
    let mut out: BTreeMap<i64, [f64; BAND_COUNT]> = BTreeMap::new();
    for r in records {
        if !r.t_unix.is_finite() {
            continue;
        }
        let day = (r.t_unix / DAY).floor() as i64;
        if day < lo || day > hi {
            continue;
        }
        let slot = out.entry(day).or_insert([f64::NEG_INFINITY; BAND_COUNT]);
        for (b, v) in r.bands.iter().enumerate() {
            if let Some(v) = v {
                let a = v.abs();
                if a > slot[b] {
                    slot[b] = a;
                }
            }
        }
    }
    out
}

fn daily_omni(
    recs: &[(f64, f64, u32)],
    lo: i64,
    hi: i64,
) -> (BTreeMap<i64, f64>, BTreeMap<i64, f64>) {
    let mut bz: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    let mut v: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for &(t, val, comp) in recs {
        if !t.is_finite() || !val.is_finite() {
            continue;
        }
        let unix = t + J2000_UNIX_OFFSET;
        let day = (unix / DAY).floor() as i64;
        if day < lo || day > hi {
            continue;
        }
        if comp == omni2::COMP_BZ && val.abs() < BZ_FILL {
            bz.entry(day).or_default().push(val);
        } else if comp == omni2::COMP_V1800 && val > 0.0 && val < V_FILL {
            v.entry(day).or_default().push(val);
        }
    }
    let mut bzmin = BTreeMap::new();
    for (day, vals) in bz {
        if vals.len() >= MIN_HOURS {
            bzmin.insert(day, vals.iter().copied().fold(f64::INFINITY, f64::min));
        }
    }
    let mut vmean = BTreeMap::new();
    for (day, vals) in v {
        if vals.len() >= MIN_HOURS {
            vmean.insert(day, vals.iter().sum::<f64>() / vals.len() as f64);
        }
    }
    (bzmin, vmean)
}

fn te_uncond(xp: &[f32], xf: &[f32], y: &[f32]) -> Option<f64> {
    let n = xp.len();
    if n < 8 || xf.len() < n || y.len() < n {
        return None;
    }
    let mut both = Vec::with_capacity(2 * n);
    both.extend_from_slice(xp);
    both.extend_from_slice(xf);
    let hx = silverman(&both)?;
    let hy = silverman(y)?;
    let mut te = 0.0;
    for t in 0..n {
        let xpt = xp[t] as f64;
        let xft = xf[t] as f64;
        let yt = y[t] as f64;
        let mut s3 = 0.0;
        let mut s1 = 0.0;
        let mut s2xy = 0.0;
        let mut s2x = 0.0;
        for s in 0..n {
            let kx = gaussian(xpt - xp[s] as f64, hx);
            let kxf = gaussian(xft - xf[s] as f64, hx);
            let ky = gaussian(yt - y[s] as f64, hy);
            s3 += kxf * kx * ky;
            s1 += kx;
            s2xy += kx * ky;
            s2x += kxf * kx;
        }
        te += ((s3 * s1) / (s2xy * s2x).max(1e-300)).ln();
    }
    Some(te / n as f64)
}

fn te_cond(xp: &[f32], xf: &[f32], y: &[f32], c: &[f32]) -> Option<f64> {
    let n = xp.len();
    if n < 8 || xf.len() < n || y.len() < n || c.len() < n {
        return None;
    }
    let mut both = Vec::with_capacity(2 * n);
    both.extend_from_slice(xp);
    both.extend_from_slice(xf);
    let hx = silverman(&both)?;
    let hy = silverman(y)?;
    let hc = silverman(c)?;
    let mut te = 0.0;
    for t in 0..n {
        let xpt = xp[t] as f64;
        let xft = xf[t] as f64;
        let yt = y[t] as f64;
        let ct = c[t] as f64;
        let mut s5 = 0.0;
        let mut s3 = 0.0;
        let mut s4a = 0.0;
        let mut s4b = 0.0;
        for s in 0..n {
            let kx = gaussian(xpt - xp[s] as f64, hx);
            let kxf = gaussian(xft - xf[s] as f64, hx);
            let ky = gaussian(yt - y[s] as f64, hy);
            let kc = gaussian(ct - c[s] as f64, hc);
            s5 += kxf * kx * ky * kc;
            s3 += kx * kc;
            s4a += kx * ky * kc;
            s4b += kxf * kx * kc;
        }
        te += ((s5 * s3) / (s4a * s4b).max(1e-300)).ln();
    }
    Some(te / n as f64)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn shift_within_year(values: &[f32], years: &[i64], rng: &mut Rng) -> Vec<f32> {
    let mut out = values.to_vec();
    let mut groups: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (i, &y) in years.iter().enumerate() {
        groups.entry(y).or_default().push(i);
    }
    for idx in groups.values() {
        if idx.len() < 2 {
            continue;
        }
        let shift = rng.below(idx.len());
        if shift == 0 {
            continue;
        }
        for (k, &i) in idx.iter().enumerate() {
            out[i] = values[idx[(k + shift) % idx.len()]];
        }
    }
    out
}

fn q95(null: &[f64]) -> Option<f64> {
    if null.is_empty() {
        return None;
    }
    let mut v = null.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    Some(v[(v.len() as f64 * 0.95) as usize])
}

fn p_value(obs: f64, null: &[f64]) -> f64 {
    if null.is_empty() {
        return 1.0;
    }
    let exceed = null.iter().filter(|v| **v >= obs).count();
    (exceed as f64 + 1.0) / (null.len() as f64 + 1.0)
}

fn mean(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        None
    } else {
        Some(v.iter().sum::<f64>() / v.len() as f64)
    }
}

fn fmt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.4e}"),
        None => "pending".to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ground_path =
        arg_after(&args, "--ground").unwrap_or("experiments/ableitungen/bz_retro/abk_dbdt_daily.tsv");
    let omni_path = arg_after(&args, "--omni").unwrap_or("/tmp/opencode/gic_te/omni2_serie_1h.bin");
    let sun_path = arg_after(&args, "--sun")
        .unwrap_or("/tmp/opencode/callisto_mro9/callisto_mro_multiyear.bin");
    let start = match arg_after(&args, "--start").and_then(day_from_iso) {
        Some(v) => v,
        None => days_from_civil(2018, 1, 1),
    };
    let end = match arg_after(&args, "--end").and_then(day_from_iso) {
        Some(v) => v,
        None => days_from_civil(2025, 12, 31),
    };
    let n_surr = arg_after(&args, "--n-surreps")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_N_SURR);
    let max_lag = arg_after(&args, "--lags")
        .and_then(|s| {
            s.split(',')
                .filter_map(|x| x.trim().parse::<i64>().ok())
                .max()
        })
        .unwrap_or(3)
        .max(0);

    let ground = match read_ground(ground_path) {
        Some(g) => g,
        None => {
            eprintln!("{ground_path}: reads void — no measurement");
            std::process::exit(1);
        }
    };
    let omni = match read_omni(omni_path) {
        Some(o) => o,
        None => {
            eprintln!("{omni_path}: reads void — no measurement");
            std::process::exit(1);
        }
    };
    let sun_recs = match read_sun(sun_path) {
        Some(s) => s,
        None => {
            eprintln!("{sun_path}: reads void — no measurement");
            std::process::exit(1);
        }
    };
    let (bzmin, vmean) = daily_omni(&omni, start, end);
    let sun_raw = sun_days(&sun_recs, start, end);

    let mut anchor: Vec<i64> = sun_raw
        .keys()
        .copied()
        .filter(|d| *d >= start + 1 && *d + max_lag <= end)
        .filter(|d| {
            ground.contains_key(&(*d - 1))
                && ground.contains_key(&(*d + max_lag))
                && bzmin.contains_key(d)
                && vmean.contains_key(d)
                && bzmin.contains_key(&(*d - 1))
                && bzmin.contains_key(&(*d + max_lag))
                && vmean.contains_key(&(*d - 1))
                && vmean.contains_key(&(*d + max_lag))
        })
        .collect();
    anchor.sort_unstable();
    let n = anchor.len();
    if n < 20 {
        eprintln!("only {n} anchor days — no measurement");
        std::process::exit(1);
    }

    let mut band_values: Vec<Vec<f32>> = vec![Vec::with_capacity(n); BAND_COUNT];
    for &d in &anchor {
        let row = &sun_raw[&d];
        for b in 0..BAND_COUNT {
            band_values[b].push(row[b] as f32);
        }
    }
    let mut band_med: [Option<f64>; BAND_COUNT] = [None; BAND_COUNT];
    for b in 0..BAND_COUNT {
        let mut vals: Vec<f64> = band_values[b]
            .iter()
            .filter(|v| v.is_finite())
            .map(|v| *v as f64)
            .collect();
        band_med[b] = median(&mut vals);
    }
    let mut sun_comp_opt: Vec<Option<f32>> = vec![None; n];
    for i in 0..n {
        let mut acc = 0.0f64;
        let mut ok = true;
        for b in 0..BAND_COUNT {
            let v = band_values[b][i] as f64;
            let Some(m) = band_med[b] else {
                ok = false;
                break;
            };
            if !v.is_finite() || !m.is_finite() || m <= 0.0 {
                ok = false;
                break;
            }
            let e = v / m;
            acc += e * e;
        }
        if ok {
            sun_comp_opt[i] = Some((acc / BAND_COUNT as f64) as f32);
        }
    }
    let good: Vec<usize> = (0..n).filter(|&i| sun_comp_opt[i].is_some()).collect();
    let anchor: Vec<i64> = good.iter().map(|&i| anchor[i]).collect();
    let n = anchor.len();
    if n < 20 {
        eprintln!("only {n} complete-anchor days — no measurement");
        std::process::exit(1);
    }
    let sun_comp: Vec<f32> = good.iter().filter_map(|&i| sun_comp_opt[i]).collect();
    let mut band_series: Vec<Vec<f32>> = vec![Vec::with_capacity(n); BAND_COUNT];
    for b in 0..BAND_COUNT {
        band_series[b] = good.iter().map(|&i| band_values[b][i]).collect();
    }
    let years: Vec<i64> = anchor.iter().map(|&d| year_of(d)).collect();
    let mut per_year: BTreeMap<i64, usize> = BTreeMap::new();
    for &y in &years {
        *per_year.entry(y).or_insert(0) += 1;
    }

    let bz: Vec<f32> = anchor.iter().map(|d| bzmin[d] as f32).collect();
    let v: Vec<f32> = anchor.iter().map(|d| vmean[d] as f32).collect();

    let xp: Vec<f32> = anchor.iter().map(|d| ground[&(d - 1)] as f32).collect();
    let mut xf_by_lag: Vec<Vec<f32>> = Vec::new();
    for lag in 0..=max_lag {
        xf_by_lag.push(anchor.iter().map(|d| ground[&(d + lag)] as f32).collect());
    }

    println!("=== GIC incremental chain — Sun -> L1 -> ground ===");
    println!(
        "window: {} .. {} | ground {} days | OMNI {} days (Bz {}, V {}) | MRO days {}",
        iso_from_day(start),
        iso_from_day(end),
        ground.keys().filter(|d| **d >= start && **d <= end).count(),
        bzmin.len(),
        bzmin.len(),
        vmean.len(),
        sun_raw.len()
    );
    let cadence = (anchor[anchor.len() - 1] - anchor[0]) as f64 / (anchor.len() - 1) as f64;
    println!(
        "anchor days n = {n} | mean spacing {cadence:.2} d (MRO observes one day every nine days) | per year {per_year:?}"
    );
    println!(
        "ground = daily max of 1-min dB/dt [nT/min] | Sun = 9-band normalized daily max |amplitude| energy | L1 = Bz daily min / Speed daily mean"
    );
    println!(
        "lags = 0..{max_lag} calendar days | null = circular shift within each calendar year, {n_surr} replicates, 95th percentile | seed = 0x{SEED:016X}"
    );
    println!("band medians (daily max |amplitude|, digits): {band_med:?}");
    let sun_min = sun_comp
        .iter()
        .map(|&x| x as f64)
        .fold(f64::INFINITY, f64::min);
    let sun_max = sun_comp
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, |a, b| a.max(b as f64));
    let mut sun_med_vals = sun_comp.iter().map(|&x| x as f64).collect::<Vec<_>>();
    let sun_med = match median(&mut sun_med_vals) {
        Some(v) => format!("{v:.3}"),
        None => "pending".to_string(),
    };
    println!("sun composite: min {sun_min:.3} q50 {sun_med} max {sun_max:.3}");

    let mut rng = Rng(SEED);
    let mut sun_null_draws: Vec<Vec<f32>> = Vec::with_capacity(n_surr);
    let mut bz_null_draws: Vec<Vec<f32>> = Vec::with_capacity(n_surr);
    let mut v_null_draws: Vec<Vec<f32>> = Vec::with_capacity(n_surr);
    for _ in 0..n_surr {
        sun_null_draws.push(shift_within_year(&sun_comp, &years, &mut rng));
        bz_null_draws.push(shift_within_year(&bz, &years, &mut rng));
        v_null_draws.push(shift_within_year(&v, &years, &mut rng));
    }

    println!("\n=== TE table: L1 = Bz daily minimum ===");
    println!(
        "lag | TE(Sun->G) | null95 | TE(Sun->G|L1) | null95 | TE(L1->G) | null95 | TE(L1->G|Sun) | null95 | delta | p | verdict"
    );
    for lag in 0..=max_lag {
        let xf = &xf_by_lag[lag as usize];
        let sun_te = te_uncond(&xp, xf, &sun_comp);
        let sun_cond = te_cond(&xp, xf, &sun_comp, &bz);
        let bz_te = te_uncond(&xp, xf, &bz);
        let bz_cond = te_cond(&xp, xf, &bz, &sun_comp);

        let mut sun_null_te = Vec::with_capacity(n_surr);
        let mut sun_null_cond = Vec::with_capacity(n_surr);
        let mut bz_null_te = Vec::with_capacity(n_surr);
        let mut bz_null_cond = Vec::with_capacity(n_surr);
        for b in 0..n_surr {
            if let Some(t) = te_uncond(&xp, xf, &sun_null_draws[b]) {
                sun_null_te.push(t);
            }
            if let Some(t) = te_cond(&xp, xf, &sun_null_draws[b], &bz) {
                sun_null_cond.push(t);
            }
            if let Some(t) = te_uncond(&xp, xf, &bz_null_draws[b]) {
                bz_null_te.push(t);
            }
            if let Some(t) = te_cond(&xp, xf, &bz_null_draws[b], &sun_comp) {
                bz_null_cond.push(t);
            }
        }
        let q_sun = q95(&sun_null_te);
        let q_sun_cond = q95(&sun_null_cond);
        let q_bz = q95(&bz_null_te);
        let q_bz_cond = q95(&bz_null_cond);
        let (delta, verdict) = match (bz_cond, q_bz_cond) {
            (Some(o), Some(q)) if o > q => (Some(o - q), "traegt"),
            (Some(o), Some(q)) => (Some(o - q), "traegt nicht"),
            _ => (None, "pending"),
        };
        println!(
            "{lag} d | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.4} | {}",
            fmt(sun_te),
            fmt(q_sun),
            fmt(sun_cond),
            fmt(q_sun_cond),
            fmt(bz_te),
            fmt(q_bz),
            fmt(bz_cond),
            fmt(q_bz_cond),
            fmt(delta),
            bz_cond.map(|o| p_value(o, &bz_null_cond)).unwrap_or(1.0),
            verdict
        );
        if let (Some(o), Some(m)) = (bz_cond, mean(&bz_null_cond)) {
            println!(
                "      null mean {m:.4e} | observed-minus-q95 {} | observed-minus-null-mean {:.4e}",
                fmt(delta),
                o - m
            );
        }
    }

    println!("\n=== TE table: L1 = Speed daily mean ===");
    println!(
        "lag | TE(Sun->G) | null95 | TE(Sun->G|L1) | null95 | TE(L1->G) | null95 | TE(L1->G|Sun) | null95 | delta | p | verdict"
    );
    for lag in 0..=max_lag {
        let xf = &xf_by_lag[lag as usize];
        let sun_te = te_uncond(&xp, xf, &sun_comp);
        let sun_cond = te_cond(&xp, xf, &sun_comp, &v);
        let v_te = te_uncond(&xp, xf, &v);
        let v_cond = te_cond(&xp, xf, &v, &sun_comp);

        let mut sun_null_te = Vec::with_capacity(n_surr);
        let mut sun_null_cond = Vec::with_capacity(n_surr);
        let mut v_null_te = Vec::with_capacity(n_surr);
        let mut v_null_cond = Vec::with_capacity(n_surr);
        for b in 0..n_surr {
            if let Some(t) = te_uncond(&xp, xf, &sun_null_draws[b]) {
                sun_null_te.push(t);
            }
            if let Some(t) = te_cond(&xp, xf, &sun_null_draws[b], &v) {
                sun_null_cond.push(t);
            }
            if let Some(t) = te_uncond(&xp, xf, &v_null_draws[b]) {
                v_null_te.push(t);
            }
            if let Some(t) = te_cond(&xp, xf, &v_null_draws[b], &sun_comp) {
                v_null_cond.push(t);
            }
        }
        let q_sun = q95(&sun_null_te);
        let q_sun_cond = q95(&sun_null_cond);
        let q_v = q95(&v_null_te);
        let q_v_cond = q95(&v_null_cond);
        let (delta, verdict) = match (v_cond, q_v_cond) {
            (Some(o), Some(q)) if o > q => (Some(o - q), "traegt"),
            (Some(o), Some(q)) => (Some(o - q), "traegt nicht"),
            _ => (None, "pending"),
        };
        println!(
            "{lag} d | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.4} | {}",
            fmt(sun_te),
            fmt(q_sun),
            fmt(sun_cond),
            fmt(q_sun_cond),
            fmt(v_te),
            fmt(q_v),
            fmt(v_cond),
            fmt(q_v_cond),
            fmt(delta),
            v_cond.map(|o| p_value(o, &v_null_cond)).unwrap_or(1.0),
            verdict
        );
        if let (Some(o), Some(m)) = (v_cond, mean(&v_null_cond)) {
            println!(
                "      null mean {m:.4e} | observed-minus-q95 {} | observed-minus-null-mean {:.4e}",
                fmt(delta),
                o - m
            );
        }
    }

    println!("\n=== Sun -> L1 (first edge of the chain) ===");
    println!("L1 | lag | TE(Sun->L1) | null95 | p | verdict");
    let mut rng_l1 = Rng(SEED ^ 0xA5A5_5A5A_1234_5678);
    for (name, map) in [("Bzmin", &bzmin), ("Vmean", &vmean)] {
        for lag in 0..=max_lag {
            let mut xp_l1 = Vec::with_capacity(n);
            let mut xf_l1 = Vec::with_capacity(n);
            let mut ys = Vec::with_capacity(n);
            let mut yrs = Vec::with_capacity(n);
            for (i, &d) in anchor.iter().enumerate() {
                let (Some(&p), Some(&f)) = (map.get(&(d - 1)), map.get(&(d + lag))) else {
                    continue;
                };
                xp_l1.push(p as f32);
                xf_l1.push(f as f32);
                ys.push(sun_comp[i]);
                yrs.push(years[i]);
            }
            let obs = te_uncond(&xp_l1, &xf_l1, &ys);
            let mut null = Vec::with_capacity(n_surr);
            for _ in 0..n_surr {
                let shifted = shift_within_year(&ys, &yrs, &mut rng_l1);
                if let Some(t) = te_uncond(&xp_l1, &xf_l1, &shifted) {
                    null.push(t);
                }
            }
            let q = q95(&null);
            let p = obs.map(|o| p_value(o, &null)).unwrap_or(1.0);
            let verdict = match (obs, q) {
                (Some(o), Some(q)) if o > q => "traegt",
                (Some(_), Some(_)) => "traegt nicht",
                _ => "pending",
            };
            println!(
                "{name} | {lag} d | {} | {} | {:.4} | {verdict}",
                fmt(obs),
                fmt(q),
                p
            );
        }
    }

    println!("\n=== per-band observed TE (descriptive; the family null below is the test) ===");
    println!("band | lag | TE(band->G) | TE(band->G|Bzmin)");
    let mut band_best = (f64::NEG_INFINITY, 0usize, 0i64);
    for b in 0..BAND_COUNT {
        for lag in 0..=max_lag {
            let xf = &xf_by_lag[lag as usize];
            let te = te_uncond(&xp, xf, &band_series[b]);
            let te_c = te_cond(&xp, xf, &band_series[b], &bz);
            if let Some(t) = te
                && t > band_best.0
            {
                band_best = (t, b, lag);
            }
            println!(
                "{} | {lag} d | {} | {}",
                callisto::component_name(b as u32).unwrap_or("?"),
                fmt(te),
                fmt(te_c)
            );
        }
    }

    println!("\n=== per-band family null (max over 9 bands x all lags) ===");
    let band_n_surr = n_surr.min(100);
    println!("band-family null replicates: {band_n_surr}");
    let mut band_null_max = Vec::with_capacity(band_n_surr);
    for _ in 0..band_n_surr {
        let mut mx = f64::NEG_INFINITY;
        for band in 0..BAND_COUNT {
            let shifted = shift_within_year(&band_series[band], &years, &mut rng);
            for lag in 0..=max_lag {
                if let Some(t) = te_uncond(&xp, &xf_by_lag[lag as usize], &shifted) {
                    mx = mx.max(t);
                }
            }
        }
        if mx.is_finite() {
            band_null_max.push(mx);
        }
    }
    if let Some(q) = q95(&band_null_max) {
        let verdict = if band_best.0 > q {
            "traegt"
        } else {
            "traegt nicht"
        };
        println!(
            "observed max TE(band->G) = {:.4e} at {} lag {} d | family null95 = {q:.4e} | p = {:.4} | {verdict}",
            band_best.0,
            callisto::component_name(band_best.1 as u32).unwrap_or("?"),
            band_best.2,
            p_value(band_best.0, &band_null_max)
        );
    } else {
        println!("family null pending — no finite surrogate maximum");
    }

    println!(
        "\nverdict: L1 carries incrementally over the Sun stage only where the TE(L1->G|Sun) column clears its own 95th-percentile null and delta > 0. A negative delta is a measured negative result, not a missing number. The Sun -> L1 table names whether the chain's first edge is live; if it is null, a positive incremental L1 term is L1's own edge, not a transmitted radio signal."
    );
}
