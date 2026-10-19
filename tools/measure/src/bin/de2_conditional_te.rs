use std::collections::{BTreeMap, HashMap};
use std::env;

use omegaflow::archivar::hapi_csv::parse_iso_seconds;
use omegaflow::archivar::{
    BodyEphemeris, body_barycenter_position, body_fixed_vector_to_icrs, parse_ephemeris_binary,
};
use omegaflow::te::{
    LaggedCond, TeNull, TeStatsParams, conditional_te_stats_lagged_n,
    transfer_entropy_conditional_binned_n,
};
use omegaflow::wdc_ae::parse_bin;

const SEED: u64 = 0x5eed_de2;
const FILL_ABS: f64 = 1.0e30;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    arg_value(args, name)
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn usable(v: f64) -> bool {
    v.is_finite() && v.abs() < FILL_ABS
}

struct Acc {
    sum: f64,
    n: usize,
}

impl Acc {
    fn empty() -> Self {
        Acc { sum: 0.0, n: 0 }
    }

    fn add(&mut self, v: f64) {
        self.sum += v;
        self.n += 1;
    }

    fn mean(&self) -> Option<f64> {
        (self.n > 0).then(|| self.sum / self.n as f64)
    }
}

fn bin_of(t: f64, bin_s: f64) -> i64 {
    (t / bin_s).floor() as i64
}

fn read_kp_csv(path: &str, bin_s: f64) -> BTreeMap<i64, f64> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("de2_conditional_te: Kp csv unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let mut acc: BTreeMap<i64, Acc> = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() < 2 {
            continue;
        }
        let Some(t) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let Ok(v) = cells[1].parse::<f64>() else {
            continue;
        };
        if usable(v) {
            acc.entry(bin_of(t, bin_s))
                .or_insert(Acc::empty())
                .add(v);
        }
    }
    acc.into_iter()
        .filter_map(|(k, a)| a.mean().map(|m| (k, m)))
        .collect()
}

fn read_orbit_csv(path: &str) -> Vec<(f64, f64, f64, f64, f64)> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("de2_conditional_te: orbit csv unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let mut rows: Vec<(f64, f64, f64, f64, f64)> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() < 3 {
            continue;
        }
        let t = parse_iso_seconds(cells[0]).or_else(|| cells[0].parse::<f64>().ok());
        let (Some(t), Ok(alt), Ok(lt)) = (t, cells[1].parse::<f64>(), cells[2].parse::<f64>())
        else {
            continue;
        };
        let lat = cells.get(3).and_then(|c| c.parse::<f64>().ok());
        let lon = cells.get(4).and_then(|c| c.parse::<f64>().ok());
        let (lat, lon) = match (lat, lon) {
            (Some(a), Some(b)) if a.is_finite() && b.is_finite() => (a, b),
            _ => (f64::NAN, f64::NAN),
        };
        if t.is_finite() && usable(alt) && alt > 0.0 && lt.is_finite() && (0.0..=24.0).contains(&lt)
        {
            rows.push((t, alt, lt, lat, lon));
        }
    }
    rows.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    rows
}

fn interp_orbit(rows: &[(f64, f64, f64, f64, f64)], t: f64) -> Option<(f64, f64, f64, f64)> {
    if rows.is_empty() {
        return None;
    }
    let i = rows.partition_point(|r| r.0 <= t);
    if i == 0 || i == rows.len() {
        return None;
    }
    let (t0, alt0, lt0, lat0, lon0) = rows[i - 1];
    let (t1, alt1, lt1, lat1, lon1) = rows[i];
    let span = t1 - t0;
    if !(span.is_finite() && span > 0.0) {
        return Some((alt0, lt0, lat0, lon0));
    }
    if span > 600.0 {
        return None;
    }
    let f = ((t - t0) / span).clamp(0.0, 1.0);
    let dlt = ((lt1 - lt0 + 12.0).rem_euclid(24.0)) - 12.0;
    let lt = (lt0 + f * dlt).rem_euclid(24.0);
    Some((
        alt0 + f * (alt1 - alt0),
        lt,
        lat0 + f * (lat1 - lat0),
        lon0 + f * (lon1 - lon0),
    ))
}

fn load_ephem(path: &str) -> (String, HashMap<String, BodyEphemeris>) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("de2_conditional_te: ephemeris unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let e = match parse_ephemeris_binary(&bytes) {
        Some(e) => e,
        None => {
            eprintln!("de2_conditional_te: ephemeris parses void {path}");
            std::process::exit(2);
        }
    };
    let name = match std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
    {
        Some(s) => s.strip_prefix("ephemeris_").unwrap_or(s).to_string(),
        None => {
            eprintln!("de2_conditional_te: ephemeris path names no body: {path}");
            std::process::exit(2);
        }
    };
    if name.is_empty() {
        eprintln!("de2_conditional_te: ephemeris path names no body: {path}");
        std::process::exit(2);
    }
    let mut map = HashMap::new();
    map.insert(name.clone(), e);
    (name, map)
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn moon_potential(
    t_unix: f64,
    radial_km: f64,
    lat_deg: f64,
    lon_deg: f64,
    moon_name: &str,
    earth_name: &str,
    moon: &HashMap<String, BodyEphemeris>,
    earth: &HashMap<String, BodyEphemeris>,
) -> Option<f64> {
    if !(radial_km.is_finite() && radial_km > 0.0 && lat_deg.is_finite() && lon_deg.is_finite()) {
        return None;
    }
    let debug = std::env::var("DE2_MOON_DEBUG").is_ok();
    macro_rules! step {
        ($e:expr, $label:expr) => {
            match $e {
                Some(v) => v,
                None => {
                    if debug {
                        eprintln!("de2_conditional_te: moon step `{}` void", $label);
                    }
                    return None;
                }
            }
        };
    }
    let lsk = step!(omegaflow::archivar::embedded_lsk(), "lsk");
    let tdb = step!(lsk.unix_to_tdb(t_unix), "tdb");
    let r = radial_km * 1000.0;
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    let xyz = [
        r * lat.cos() * lon.cos(),
        r * lat.cos() * lon.sin(),
        r * lat.sin(),
    ];
    let earth_bary = step!(
        body_barycenter_position(earth_name, tdb, earth),
        "earth barycenter"
    );
    let p_bary = step!(
        body_fixed_vector_to_icrs(earth_name, xyz, tdb, earth),
        "earth fixed->icrs"
    );
    let r_vec = [
        p_bary[0] - earth_bary[0],
        p_bary[1] - earth_bary[1],
        p_bary[2] - earth_bary[2],
    ];
    let moon_bary = step!(
        body_barycenter_position(moon_name, tdb, moon),
        "moon barycenter"
    );
    let m_vec = [
        moon_bary[0] - earth_bary[0],
        moon_bary[1] - earth_bary[1],
        moon_bary[2] - earth_bary[2],
    ];
    let rn = norm(r_vec);
    let mn = norm(m_vec);
    let dot = r_vec[0] * m_vec[0] + r_vec[1] * m_vec[1] + r_vec[2] * m_vec[2];
    if !(rn > 0.0 && mn > 0.0 && dot.is_finite()) {
        return None;
    }
    let c = (dot / (rn * mn)).clamp(-1.0, 1.0);
    Some((3.0 * c * c - 1.0) / 2.0)
}

fn read_cond(path: &str, bin_s: f64, field: &str) -> BTreeMap<i64, f64> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("de2_conditional_te: conditioner bin unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("de2_conditional_te: {path} is not a wdc_ae bin (magic/length)");
        std::process::exit(2);
    };
    let mut acc: BTreeMap<i64, Acc> = BTreeMap::new();
    for r in records {
        let v = match field {
            "dst" => r.dst_nt,
            "symd" => r.sym_d_nt,
            _ => r.ae_nt,
        };
        if let Some(v) = v
            && usable(v)
            && r.t_unix.is_finite()
        {
            acc.entry(bin_of(r.t_unix, bin_s))
                .or_insert(Acc::empty())
                .add(v);
        }
    }
    acc.into_iter()
        .filter_map(|(k, a)| a.mean().map(|m| (k, m)))
        .collect()
}

fn read_de2(path: &str, bin_s: f64) -> (BTreeMap<i64, f64>, BTreeMap<i64, f64>) {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("de2_conditional_te: DE-2 CSV unread {path}: {e}");
            std::process::exit(2);
        }
    };
    let mut ion: BTreeMap<i64, Acc> = BTreeMap::new();
    let mut neutral: BTreeMap<i64, Acc> = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() != 7 {
            continue;
        }
        let Some(t) = parse_iso_seconds(cells[0]) else {
            continue;
        };
        let mut v = [0.0f64; 6];
        let mut ok = true;
        for (slot, cell) in v.iter_mut().zip(&cells[1..]) {
            match cell.parse::<f64>() {
                Ok(x) => *slot = x,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let (nve, ive) = (v[1], v[4]);
        if !usable(nve) || !usable(ive) {
            continue;
        }
        let key = bin_of(t, bin_s);
        neutral.entry(key).or_insert(Acc::empty()).add(nve);
        ion.entry(key).or_insert(Acc::empty()).add(ive);
    }
    let means = |m: BTreeMap<i64, Acc>| {
        m.into_iter()
            .filter_map(|(k, a)| a.mean().map(|x| (k, x)))
            .collect()
    };
    (means(ion), means(neutral))
}

fn demean(v: &mut [f32]) {
    let n = v.len() as f64;
    if n == 0.0 {
        return;
    }
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    for x in v {
        *x = (*x as f64 - mean) as f32;
    }
}

fn iso(t: f64) -> String {
    let secs = t.floor() as i64;
    let days = secs.div_euclid(86400);
    let sod = secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let csv = match arg_value(&args, "--csv") {
        Some(v) => v,
        None => "sources/cdaweb.gsfc.nasa.gov/de2_ua16s_all.csv".to_string(),
    };
    let ae = match arg_value(&args, "--ae") {
        Some(v) => v,
        None => "data/wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin".to_string(),
    };
    let bin_s: f64 = parse_arg(&args, "--bin-seconds", 300.0);
    if !(bin_s.is_finite() && bin_s > 0.0) {
        eprintln!("de2_conditional_te: --bin-seconds must be positive");
        std::process::exit(2);
    }
    let cond_fields: Vec<String> = match arg_value(&args, "--cond") {
        Some(v) => v
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        None => vec!["ae".to_string()],
    };
    for field in &cond_fields {
        if !["ae", "dst", "symd", "kp", "lt", "alt", "lunar"].contains(&field.as_str()) {
            eprintln!("de2_conditional_te: --cond names no ae|dst|symd|kp|lt|alt|lunar");
            std::process::exit(2);
        }
    }
    let cond_arg: f64 = parse_arg(&args, "--cond-seconds", 0.0);
    let cond_default = if cond_fields.iter().any(|f| f == "kp") {
        10800.0
    } else {
        bin_s
    };
    let cond_s = if cond_arg == 0.0 { cond_default } else { cond_arg };
    if !(cond_s.is_finite() && cond_s >= bin_s) {
        eprintln!("de2_conditional_te: --cond-seconds must be finite and >= --bin-seconds");
        std::process::exit(2);
    }
    let max_lag: usize = parse_arg(&args, "--max-lag", 3);
    let bins: usize = parse_arg(&args, "--bins", 8);
    let n_surr: usize = parse_arg(&args, "--surr", 50);

    let (ion, neutral) = read_de2(&csv, bin_s);
    let mut index_bins: Vec<(String, BTreeMap<i64, f64>)> = Vec::new();
    for field in &cond_fields {
        if ["ae", "dst", "symd"].contains(&field.as_str()) {
            index_bins.push((field.clone(), read_cond(&ae, cond_s, field)));
        }
    }
    let kp_bins = if cond_fields.iter().any(|f| f == "kp") {
        let kp = match arg_value(&args, "--kp-csv") {
            Some(v) => v,
            None => {
                eprintln!("de2_conditional_te: --cond kp needs --kp-csv <time,kp,...>");
                std::process::exit(2);
            }
        };
        Some(read_kp_csv(&kp, cond_s))
    } else {
        None
    };
    let orbit = if cond_fields
        .iter()
        .any(|f| f == "lt" || f == "alt" || f == "lunar")
    {
        let path = match arg_value(&args, "--orbit-csv") {
            Some(v) => v,
            None => {
                eprintln!(
                    "de2_conditional_te: --cond lt|alt|moon needs --orbit-csv <time,radial_km,lt_hr,lat,lon>"
                );
                std::process::exit(2);
            }
        };
        Some(read_orbit_csv(&path))
    } else {
        None
    };
    let moon_eph = if cond_fields.iter().any(|f| f == "lunar") {
        let moon_bin = match arg_value(&args, "--moon-bin") {
            Some(v) => v,
            None => {
                eprintln!("de2_conditional_te: --cond lunar needs --moon-bin <ephemeris_moon.bin>");
                std::process::exit(2);
            }
        };
        let earth_bin = match arg_value(&args, "--earth-bin") {
            Some(v) => v,
            None => {
                eprintln!("de2_conditional_te: --cond lunar needs --earth-bin <ephemeris_earth.bin>");
                std::process::exit(2);
            }
        };
        let (moon_name, moon_map) = load_ephem(&moon_bin);
        let (earth_name, earth_map) = load_ephem(&earth_bin);
        Some((moon_name, moon_map, earth_name, earth_map))
    } else {
        None
    };

    let value_at = |field: &str, k: i64| -> Option<f64> {
        let coarse = bin_of(k as f64 * bin_s, cond_s);
        match field {
            "lt" | "alt" | "lunar" => {
                let rows = orbit.as_ref()?;
                let (radial, lt, lat, lon) = interp_orbit(rows, (k as f64 + 0.5) * bin_s)?;
                match field {
                    "lt" => Some(lt),
                    "alt" => Some(radial),
                    _ => {
                        let (moon_name, moon, earth_name, earth) = moon_eph.as_ref()?;
                        moon_potential(
                            (k as f64 + 0.5) * bin_s,
                            radial,
                            lat,
                            lon,
                            moon_name,
                            earth_name,
                            moon,
                            earth,
                        )
                    }
                }
            }
            "kp" => kp_bins.as_ref()?.get(&coarse).copied(),
            _ => index_bins
                .iter()
                .find(|(n, _)| n == field)
                .and_then(|(_, m)| m.get(&coarse).copied()),
        }
    };

    let mut keys: Vec<i64> = ion
        .keys()
        .filter(|k| {
            neutral.contains_key(*k)
                && cond_fields
                    .iter()
                    .all(|f| value_at(f.as_str(), **k).is_some())
        })
        .copied()
        .collect();
    keys.sort_unstable();
    if keys.len() < 32 {
        println!(
            "de2_conditional_te: only {} joint bins (bin={}s) -> not measurable, no fabrication",
            keys.len(),
            bin_s
        );
        return;
    }

    let mut nve: Vec<f32> = keys.iter().map(|k| neutral[k] as f32).collect();
    let mut ive: Vec<f32> = keys.iter().map(|k| ion[k] as f32).collect();
    demean(&mut nve);
    demean(&mut ive);

    let cond_label = cond_fields.join("+");
    let mut cond_series: Vec<Vec<f32>> = Vec::new();
    for field in &cond_fields {
        let mut series: Vec<f32> = keys
            .iter()
            .map(|k| {
                value_at(field.as_str(), *k).expect("keys were filtered on every conditioner")
                    as f32
            })
            .collect();
        demean(&mut series);
        cond_series.push(series);
    }

    let mut gaps: Vec<i64> = keys.windows(2).map(|w| w[1] - w[0]).collect();
    gaps.sort_unstable();
    let segments = 1 + gaps.iter().filter(|&&g| g > 2).count();
    let gap_median = gaps[gaps.len() / 2];
    let gap_max = gaps[gaps.len() - 1];
    let span = format!(
        "{} .. {}",
        iso(keys[0] as f64 * bin_s),
        iso((keys[keys.len() - 1] + 1) as f64 * bin_s)
    );

    println!(
        "de2_conditional_te: n={} bin={}s cond={} cond_bin={}s span={} segments={} gap_median={}s gap_max={}s bins={} surr={} null=Arx est=Binned seed={:#x}",
        keys.len(),
        bin_s,
        cond_label,
        cond_s,
        span,
        segments,
        gap_median as f64 * bin_s,
        gap_max as f64 * bin_s,
        bins,
        n_surr,
        SEED
    );

    let cond: Vec<LaggedCond> = cond_series
        .iter()
        .map(|series| LaggedCond { series, lag: 0 })
        .collect();
    let empty: [LaggedCond; 0] = [];
    let mut any_cond_pass = false;
    let mut max_cond: [(f64, f64); 2] = [(f64::NEG_INFINITY, f64::NEG_INFINITY); 2];
    let mut max_raw: [(f64, f64); 2] = [(f64::NEG_INFINITY, f64::NEG_INFINITY); 2];

    for lag in 1..=max_lag {
        for (dir, (label, driver, target)) in
            [("NVE->IVE", &nve, &ive), ("IVE->NVE", &ive, &nve)]
                .into_iter()
                .enumerate()
        {
            let obs_cond = transfer_entropy_conditional_binned_n(driver, target, &cond, lag, bins);
            let stats_cond = conditional_te_stats_lagged_n(
                driver,
                target,
                &cond,
                TeStatsParams {
                    lag,
                    max_lag,
                    bins,
                    seed: SEED,
                    n_surr,
                    null: TeNull::Arx,
                },
            );
            let obs_raw = transfer_entropy_conditional_binned_n(driver, target, &empty, lag, bins);
            let stats_raw = conditional_te_stats_lagged_n(
                driver,
                target,
                &empty,
                TeStatsParams {
                    lag,
                    max_lag,
                    bins,
                    seed: SEED,
                    n_surr,
                    null: TeNull::Arx,
                },
            );
            let fmt = |obs: Option<f64>, st: Option<(f64, f64, f64)>| -> (String, bool) {
                match (obs, st) {
                    (Some(o), Some((m, sd, thr))) => {
                        let pass = o > thr;
                        (
                            format!(
                                "te={o:+.6} null_mean={m:+.6} null_sd={sd:.6} thr={thr:+.6} pass={}",
                                if pass { "yes" } else { "no" }
                            ),
                            pass,
                        )
                    }
                    _ => ("te=absent".to_string(), false),
                }
            };
            let (cond_line, cond_pass) = fmt(obs_cond, stats_cond);
            any_cond_pass |= cond_pass;
            let (raw_line, _) = fmt(obs_raw, stats_raw);
            if let (Some(o), Some((_, _, thr))) = (obs_cond, stats_cond) {
                max_cond[dir].0 = max_cond[dir].0.max(thr);
                max_cond[dir].1 = max_cond[dir].1.max(o);
            }
            if let (Some(o), Some((_, _, thr))) = (obs_raw, stats_raw) {
                max_raw[dir].0 = max_raw[dir].0.max(thr);
                max_raw[dir].1 = max_raw[dir].1.max(o);
            }
            println!(
                "LAG {:>5}s {:>8} cond[{}]: {}",
                lag as f64 * bin_s,
                label,
                cond_label,
                cond_line
            );
            println!(
                "LAG {:>5}s {:>8} raw     : {}",
                lag as f64 * bin_s,
                label,
                raw_line
            );
        }
    }

    for (dir, label) in ["NVE->IVE", "IVE->NVE"].into_iter().enumerate() {
        let (thr, te) = max_cond[dir];
        let pass = te.is_finite() && te > thr;
        println!(
            "MAX-NULL {label} cond[{cond_label}]: te_max={te:+.6} thr_max={thr:+.6} pass={}",
            if pass { "yes" } else { "no" }
        );
        let (thr, te) = max_raw[dir];
        let pass = te.is_finite() && te > thr;
        println!(
            "MAX-NULL {label} raw     : te_max={te:+.6} thr_max={thr:+.6} pass={}",
            if pass { "yes" } else { "no" }
        );
    }
    println!(
        "de2_conditional_te: any conditioned lag above the ARX null: {}",
        if any_cond_pass { "yes" } else { "no" }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn orbit_fixture() -> std::path::PathBuf {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static N: AtomicUsize = AtomicUsize::new(0);
        let n = N.fetch_add(1, Ordering::SeqCst);
        let p = std::env::temp_dir().join(format!(
            "de2-orbit-{}-{n}.csv",
            std::process::id()
        ));
        std::fs::write(
            &p,
            "1981-08-15T00:00:00.000Z,7330.0,13.0,-46.0,197.0\n\
             1981-08-15T00:01:00.000Z,7340.0,13.1,-44.0,197.2\n",
        )
        .unwrap();
        p
    }

    #[test]
    fn orbit_csv_carries_lat_lon() {
        let p = orbit_fixture();
        let rows = read_orbit_csv(p.to_str().unwrap());
        assert_eq!(rows.len(), 2);
        assert!(rows[0].3.is_finite() && rows[0].4.is_finite());
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn orbit_interpolation_carries_lat_lon() {
        let p = orbit_fixture();
        let rows = read_orbit_csv(p.to_str().unwrap());
        let t = rows[0].0 + 30.0;
        let (radial, _lt, lat, lon) = interp_orbit(&rows, t).expect("interpolated");
        assert!((radial - 7335.0).abs() < 1e-6, "{radial}");
        assert!((lat - -45.0).abs() < 1e-6, "{lat}");
        assert!((lon - 197.1).abs() < 1e-6, "{lon}");
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn moon_potential_refuses_a_void_position() {
        let moon: HashMap<String, BodyEphemeris> = HashMap::new();
        let earth: HashMap<String, BodyEphemeris> = HashMap::new();
        assert!(
            moon_potential(0.0, 7000.0, f64::NAN, 0.0, "m", "e", &moon, &earth).is_none()
        );
        assert!(moon_potential(0.0, -1.0, 0.0, 0.0, "m", "e", &moon, &earth).is_none());
    }

    #[test]
    fn moon_potential_reads_the_real_ephemerides_when_named() {
        let (Ok(moon_bin), Ok(earth_bin)) = (
            std::env::var("DE2_MOON_BIN"),
            std::env::var("DE2_EARTH_BIN"),
        ) else {
            return;
        };
        let (moon_name, moon) = load_ephem(&moon_bin);
        let (earth_name, earth) = load_ephem(&earth_bin);
        let t = 364_000_000.0;
        let v = moon_potential(t, 7330.0, -46.0, 197.0, &moon_name, &earth_name, &moon, &earth);
        let Some(v) = v else {
            panic!("moon potential void with real ephemerides");
        };
        assert!(v.is_finite() && (-1.0..=1.0).contains(&v), "{v}");
    }
}
