use omegaflow::archivar::cache_root;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::omni2::{COMP_BZ, COMP_N1800, COMP_V1800, parse_bin};
use omegaflow::mathematikerin::wy_max_t::*;
use omegaflow::te::transfer_entropy_lag;
use std::time::{SystemTime, UNIX_EPOCH};

const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const MINUTE: f64 = 60.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const OMNI2_BIN: &str = "omni2_serie_1h.bin";
const OMNI2_CDN_BASE: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov";
const IMAG_CDN_BASE: &str =
    "https://github.com/omegaflow/sources/releases/download/imag-data.bgs.ac.uk";
const STATION_WORD: &str = "ABK";
const HOUR_START_WORD: &str = "2024-01-01";
const HOUR_END_WORD: &str = "2024-12-31";

fn now_unix() -> Option<f64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64())
}

fn iso_to_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else {
        s.split_once(' ')?
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let t = time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()?;
    let mut tp = t.split(':');
    let hh: i64 = tp.next()?.parse().ok()?;
    let mm: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: i64 = match tp.next() {
        Some(v) => v,
        None => "0",
    }
    .parse()
    .ok()?;
    let a = (14 - m) / 12;
    let yy = y + 4800 - a;
    let jdn =
        d + (153 * (m + 12 * a - 3) + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32045;
    Some((jdn - 2440588) as f64 * DAY + hh as f64 * HOUR + mm as f64 * MINUTE + ss as f64)
}

fn arg_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn args_after_all(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(i) = args.iter().position(|a| a == flag) {
        for a in &args[i + 1..] {
            if a.starts_with("--") {
                break;
            }
            out.push(a.clone());
        }
    }
    out
}

fn write_null_matrix(path: &str, nulls: &[Vec<f64>]) -> bool {
    let m = nulls.first().map_or(0, |row| row.len());
    let mut bytes = Vec::with_capacity(nulls.len() * m * 8);
    for row in nulls {
        for &v in row.iter() {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
    }
    std::fs::write(path, bytes).is_ok()
}

fn read_null_matrix(path: &str, m_expected: usize) -> Option<Vec<Vec<f64>>> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.len() % 8 != 0 || m_expected == 0 {
        return None;
    }
    let total = bytes.len() / 8;
    if total % m_expected != 0 {
        return None;
    }
    let b = total / m_expected;
    let mut out = Vec::with_capacity(b);
    for r in 0..b {
        let mut row = Vec::with_capacity(m_expected);
        for c in 0..m_expected {
            let i = (r * m_expected + c) * 8;
            let arr: [u8; 8] = bytes[i..i + 8].try_into().ok()?;
            row.push(f64::from_le_bytes(arr));
        }
        out.push(row);
    }
    Some(out)
}

fn disk_cache(name: &str) -> String {
    cache_root()
        .join("bz_retro")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn load_omni2(path: &str) -> Vec<(f64, f64, u32)> {
    if let Ok(bytes) = std::fs::read(path) {
        if let Some(recs) = parse_bin(&bytes) {
            return recs;
        }
        eprintln!("{path} parses void — fetching the CDN asset");
    } else {
        eprintln!("{path} reads void — fetching the CDN asset");
    }
    let name = match std::path::Path::new(path).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => {
            eprintln!("{path} carries no asset name — the CDN fetch stays pending");
            return Vec::new();
        }
    };
    let url = format!("{OMNI2_CDN_BASE}/{name}");
    let Some(bytes) = fetch_raw_bytes(&url) else {
        eprintln!("{url} fetch stays pending — the top series stays unmeasured");
        return Vec::new();
    };
    let _ = std::fs::write(path, &bytes);
    match parse_bin(&bytes) {
        Some(recs) => recs,
        None => {
            eprintln!("{path} parses void after fetch — the top series stays unmeasured");
            Vec::new()
        }
    }
}

fn load_cdn_station_dbdt(station: &str) -> Option<Vec<(f64, f64)>> {
    let asset = format!("{}_dbdt_1h.bin", station.to_lowercase());
    let local = disk_cache(&asset);
    let bytes = std::fs::read(&local).ok().or_else(|| {
        let url = format!("{IMAG_CDN_BASE}/{asset}");
        match fetch_raw_bytes(&url) {
            Some(b) => {
                let _ = std::fs::write(&local, &b);
                Some(b)
            }
            None => None,
        }
    })?;
    let recs = omegaflow::intermagnet::parse_bin(&bytes)?;
    Some(recs.into_iter().map(|(t, v, _)| (t, v)).collect())
}

fn bin_cells(series: &[(f64, f64)], t0: f64, dt: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / dt).floor();
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

fn align_channels(channels: &[&[Option<f32>]], times: &[f64]) -> (Vec<Vec<f32>>, Vec<f64>) {
    let n = match channels.iter().map(|c| c.len()).min() {
        Some(n) => n,
        None => return (Vec::new(), Vec::new()),
    };
    let n = n.min(times.len());
    let mut out: Vec<Vec<f32>> = vec![Vec::new(); channels.len()];
    let mut out_times: Vec<f64> = Vec::new();
    for i in 0..n {
        if !channels.iter().all(|c| c[i].is_some()) {
            continue;
        }
        for (ci, c) in channels.iter().enumerate() {
            if let Some(v) = c[i] {
                out[ci].push(v);
            }
        }
        out_times.push(times[i]);
    }
    (out, out_times)
}

fn shared_window(
    a: &[(f64, f64)],
    b: &[(f64, f64)],
    lo_clamp: Option<f64>,
    hi_clamp: Option<f64>,
) -> Option<(f64, f64)> {
    let (Some(a0), Some(a1)) = (a.first().map(|&(t, _)| t), a.last().map(|&(t, _)| t)) else {
        return None;
    };
    let (Some(b0), Some(b1)) = (b.first().map(|&(t, _)| t), b.last().map(|&(t, _)| t)) else {
        return None;
    };
    let lo = match lo_clamp {
        Some(c) => a0.max(b0).max(c),
        None => a0.max(b0),
    };
    let hi = match hi_clamp {
        Some(c) => a1.min(b1).min(c),
        None => a1.min(b1),
    };
    if hi <= lo { None } else { Some((lo, hi)) }
}

fn synthetic_channels(n: usize, a: f32, channels: usize, state: &mut u64) -> Vec<Vec<f32>> {
    let burn = 200usize;
    let mut x = vec![vec![0f32; burn + n]; channels];
    for step in 1..burn + n {
        for j in 0..channels {
            x[j][step] = a * x[j][step - 1] + gauss(state);
        }
    }
    x.into_iter().map(|c| c[burn..].to_vec()).collect()
}

fn print_observed(members: &[Member], observed: &[Option<f64>]) {
    for (m, te) in members.iter().zip(observed.iter()) {
        match te {
            Some(v) => println!("observed {} | lag {} | TE {}", m.label, m.lag, v),
            None => println!("observed {} | TE absent (n < 8)", m.label),
        }
    }
    let lag1_delta = members
        .iter()
        .enumerate()
        .filter_map(|(i, m)| {
            let a = observed[i]?;
            let b = transfer_entropy_lag(&m.target, &m.driver, 1)?;
            Some((a - b).abs())
        })
        .fold(0.0, f64::max);
    println!("lag-0/1 identity check (max |TE(lag0) - TE(lag1)|): {lag1_delta:.4e}");
}

fn report_family_partition(
    members: &[Member],
    observed: &[Option<f64>],
    nulls: &[Vec<f64>],
    alpha: f64,
) {
    let sigma = sigma_per_statistic(nulls);
    let null_means = null_means_per_statistic(nulls);
    for (m, s) in members.iter().zip(sigma.iter()) {
        match s {
            Some(s) => println!("sigma {} = {s:.4e}", m.label),
            None => println!("sigma {} absent — the null carries no spread", m.label),
        }
    }
    let stud_null = studentized_maxima(nulls, &null_means, &sigma);
    let mut finite: Vec<f64> = stud_null
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .collect();
    finite.sort_by(|a, b| a.total_cmp(b));
    let Some(q) = quantile(&finite, alpha) else {
        println!(
            "studentized max-T null distribution absent — no finite replicate carries a spread"
        );
        return;
    };
    println!(
        "studentized max-T null distribution: {} finite replicates | (1-alpha) quantile = {}",
        finite.len(),
        q
    );

    let obs_stud: Vec<Option<f64>> = observed
        .iter()
        .enumerate()
        .map(|(mi, o)| {
            let m = null_means.get(mi).copied().flatten()?;
            let s = sigma.get(mi).copied().flatten()?;
            Some(((*o)? - m) / s)
        })
        .collect();
    let obs_max = obs_stud
        .iter()
        .flatten()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    println!(
        "observed studentized family maximum = {} | family maximum clears (1-alpha) quantile: {}",
        obs_max,
        obs_max > q
    );

    let b = finite.len();
    println!(
        "{:<18} | {:>10} | {:>10} | {:>10} | {:>10} | verdict",
        "member", "TE_obs", "TE_bc", "T_stud", "p_adj"
    );
    for (mi, m) in members.iter().enumerate() {
        let Some(v) = observed[mi] else {
            println!(
                "{:<18} | {:>10} | {:>10} | {:>10} | {:>10} | TE absent",
                m.label, "pending", "pending", "pending", "pending"
            );
            continue;
        };
        let Some(vs) = obs_stud[mi] else {
            let bc = match null_means[mi] {
                Some(mu) => v - mu,
                None => f64::NAN,
            };
            println!(
                "{:<18} | {:>10.4e} | {:>10.4e} | {:>10} | {:>10} | sigma absent",
                m.label, v, bc, "pending", "pending"
            );
            continue;
        };
        let ge = finite.iter().filter(|&&x| x >= vs).count();
        let p = (ge as f64 + 1.0) / (b as f64 + 1.0);
        let verdict = if vs > q {
            "family-clearing"
        } else {
            "family bound"
        };
        match null_means[mi] {
            Some(mu) => {
                let bc = v - mu;
                println!(
                    "{:<18} | {:>10.4e} | {:>10.4e} | {:>10.4e} | {:>10.4e} | {}",
                    m.label, v, bc, vs, p, verdict
                );
            }
            None => println!(
                "{:<18} | {:>10.4e} | {:>10} | {:>10.4e} | {:>10.4e} | {}",
                m.label, v, "pending", vs, p, verdict
            ),
        }
    }
    println!(
        "BCa per-statistic CI: pending — the joint-stationary jackknife is not cheap; named, never dropped."
    );
}

fn run_family(
    members: &[Member],
    times: &[f64],
    perm_from: usize,
    perm_to: usize,
    block: usize,
    alpha: f64,
    seed: u64,
    threads: usize,
    mode: ResampleMode,
    out_null: Option<&str>,
) {
    let n = members[0].target.len();
    let count = perm_to.saturating_sub(perm_from);
    let mode_name = match mode {
        ResampleMode::Driver => "driver-bootstrap (seasonal block, month/hour matched)",
        ResampleMode::Condition => "condition-permutation (block)",
    };
    println!(
        "family: K = {} declared (index-hash dedup of {} candidate (pair,lag) statistics) | n = {} | B = {} | replicates [{}..{}) | resample = {} | block = {} | alpha = {} | seed = {}",
        FAMILY_K,
        FAMILY_K * 2,
        n,
        count,
        perm_from,
        perm_to,
        mode_name,
        block,
        alpha,
        seed
    );
    println!(
        "construction: rank-Gauss per channel before the TE; one resample of the driver dependence is shared across the K statistics per draw; each statistic is studentized and bias-corrected against its own null."
    );
    let observed = observed_family(members);
    print_observed(members, &observed);

    let (buckets, pos_bucket) = phase_data(times);
    let nulls = null_matrix(
        members,
        perm_from,
        perm_to,
        block,
        mode,
        seed,
        threads,
        &buckets,
        &pos_bucket,
    );
    if let Some(path) = out_null {
        if write_null_matrix(path, &nulls) {
            println!(
                "null matrix written: {path} | rows = {} | m = {} (NaN rows kept)",
                nulls.len(),
                members.len()
            );
        } else {
            println!("{path} writes void — the null matrix stays unwritten");
        }
    }
    report_family_partition(members, &observed, &nulls, alpha);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let selftest = args.iter().any(|a| a == "--selftest");
    let station = arg_after(&args, "--station")
        .unwrap_or(STATION_WORD)
        .to_string();
    let hour_start = arg_after(&args, "--hour-start")
        .unwrap_or(HOUR_START_WORD)
        .to_string();
    let hour_end = arg_after(&args, "--hour-end")
        .unwrap_or(HOUR_END_WORD)
        .to_string();
    let n_perm = arg_after(&args, "--n-perm")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(9999);
    let block = arg_after(&args, "--block")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(24);
    let alpha = arg_after(&args, "--alpha")
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| v.is_finite() && *v > 0.0 && *v < 1.0)
        .unwrap_or(0.05);
    let threads = arg_after(&args, "--threads")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&t| t > 0)
        .unwrap_or(4);
    let round = match arg_after(&args, "--round").and_then(|v| v.parse::<u64>().ok()) {
        Some(r) => r,
        None => 0,
    };
    let year_arg = arg_after(&args, "--year").and_then(|v| v.parse::<i64>().ok());
    let seed_override = arg_after(&args, "--seed").and_then(|v| v.parse::<u64>().ok());
    let mode = match arg_after(&args, "--resample").unwrap_or("driver") {
        "condition" => ResampleMode::Condition,
        "driver" => ResampleMode::Driver,
        other => {
            eprintln!("--resample {other} names no mode — driver|condition; driver stands");
            ResampleMode::Driver
        }
    };
    let gpd = args.iter().any(|a| a == "--gpd");
    let perm_from = match arg_after(&args, "--perm-from").and_then(|v| v.parse::<usize>().ok()) {
        Some(v) => v,
        None => 0,
    };
    let perm_to = arg_after(&args, "--perm-to")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(n_perm);
    let out_null = arg_after(&args, "--out-null").map(|s| s.to_string());
    let combine_paths = args_after_all(&args, "--combine");
    if combine_paths.is_empty() && perm_to <= perm_from {
        println!("replicate range [{perm_from}..{perm_to}) is empty — no measurement");
        return;
    }

    println!("=== Westfall-Young max-T permutation probe (Bz / L1 drivers -> dB/dt) ===");
    if let Some(now) = now_unix() {
        println!("system time: {now:.0} unix");
    }
    println!(
        "construction: rank-Gauss per channel; seasonal block bootstrap of the driver (block in steps, intended as multiples of 24 h); start indices matched on calendar month and UT hour; one resample shared across the K=6 statistics per draw; seed = hash(round, station, year); each statistic studentized and bias-corrected."
    );
    if gpd {
        println!(
            "GPD tail: pending — the empirical studentized max-T quantile carries the family bound; the estimator stays untouched."
        );
    }

    if selftest {
        let n = 240usize;
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        let ch = synthetic_channels(n, 0.5, 4, &mut state);
        let members = prepare_members(&ch);
        if members.len() != FAMILY_K {
            println!(
                "family K {} differs from declared {} — the selftest stays absent",
                members.len(),
                FAMILY_K
            );
            return;
        }
        let epoch = match iso_to_unix("2024-01-01T00:00:00Z") {
            Some(t) => t,
            None => {
                println!("selftest epoch absent — no run");
                return;
            }
        };
        let times: Vec<f64> = (0..n).map(|i| epoch + i as f64 * HOUR).collect();
        let year = 2024i64;
        let seed = match seed_override {
            Some(s) => s,
            None => round_seed(round, &station, year),
        };
        println!(
            "selftest: four independent AR(1) a=0.5 channels, n = {} (no data fetch)",
            n
        );
        run_family(
            &members,
            &times,
            0,
            n_perm.min(400),
            block,
            alpha,
            seed,
            threads,
            mode,
            out_null.as_deref(),
        );
        return;
    }

    let omni = load_omni2(&disk_cache(OMNI2_BIN));
    let omni_bz: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_BZ)
        .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET, v))
        .collect();
    let omni_speed: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_V1800)
        .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET, v))
        .collect();
    let omni_density: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_N1800)
        .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET, v))
        .collect();
    let Some(dbdt_1h) = load_cdn_station_dbdt(&station) else {
        println!("{station} hourly |dB/dt| absent — the CDN asset stays pending; no measurement");
        return;
    };
    println!(
        "omni2_serie_1h.bin: Bz {:<6} | Speed {:<6} | Density {:<6} | {} dB/dt {:<6}",
        omni_bz.len(),
        omni_speed.len(),
        omni_density.len(),
        station,
        dbdt_1h.len()
    );
    let h_start = iso_to_unix(&format!("{hour_start}T00:00:00Z"));
    let h_end = iso_to_unix(&format!("{hour_end}T00:00:00Z"));
    let Some((lo, hi)) = shared_window(&omni_bz, &dbdt_1h, h_start, h_end) else {
        println!("common hourly window absent — no hour shared by OMNI2 and {station} |dB/dt|");
        return;
    };
    let t0 = (lo / HOUR).floor() * HOUR;
    let hours = ((hi - t0) / HOUR).floor();
    if hours < 1.0 {
        println!("common hourly window spans no hour — the family stays unmeasured");
        return;
    }
    let n_cells = hours as usize;
    let times_all: Vec<f64> = (0..n_cells).map(|i| t0 + i as f64 * HOUR).collect();
    let bz = bin_cells(&omni_bz, t0, HOUR, n_cells);
    let speed = bin_cells(&omni_speed, t0, HOUR, n_cells);
    let density = bin_cells(&omni_density, t0, HOUR, n_cells);
    let dbdt = bin_cells(&dbdt_1h, t0, HOUR, n_cells);
    let (aligned, aligned_times) = align_channels(&[&dbdt, &bz, &speed, &density], &times_all);
    if aligned.len() < 4 || aligned[0].len() < 8 {
        println!("aligned hourly series absent — the family stays unmeasured");
        return;
    }
    println!(
        "window {} -> {} | {} hourly cells | {} common complete cases n = {}",
        hour_start,
        hour_end,
        n_cells,
        station,
        aligned[0].len()
    );
    let members = prepare_members(&aligned);
    if members.len() != FAMILY_K {
        println!(
            "family K {} differs from declared {} — the family stays absent",
            members.len(),
            FAMILY_K
        );
        return;
    }
    let Some(&t_first) = aligned_times.first() else {
        println!("aligned time axis absent — the family stays unmeasured");
        return;
    };
    let year = match year_arg {
        Some(y) => y,
        None => year_of_unix(t_first),
    };
    let seed = match seed_override {
        Some(s) => s,
        None => round_seed(round, &station, year),
    };
    println!(
        "seed derivation: round = {round} | station = {station} | year = {year} | seed = {seed}"
    );

    if !combine_paths.is_empty() {
        let mut pooled: Vec<Vec<f64>> = Vec::new();
        for path in &combine_paths {
            match read_null_matrix(path, members.len()) {
                Some(rows) => {
                    println!("pooled shard {path}: {} replicates", rows.len());
                    pooled.extend(rows);
                }
                None => {
                    println!(
                        "{path} reads void — the pooled null stays incomplete; no measurement"
                    );
                    return;
                }
            }
        }
        println!(
            "combine: pooled B = {} | m = {} | alpha = {} | seed = {} (shards carried the same seed and global replicate indices)",
            pooled.len(),
            members.len(),
            alpha,
            seed
        );
        let observed = observed_family(&members);
        print_observed(&members, &observed);
        report_family_partition(&members, &observed, &pooled, alpha);
        return;
    }

    run_family(
        &members,
        &aligned_times,
        perm_from,
        perm_to,
        block,
        alpha,
        seed,
        threads,
        mode,
        out_null.as_deref(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits_eq(a: &[Vec<f64>], b: &[Vec<f64>]) -> bool {
        a.len() == b.len()
            && a.iter().zip(b.iter()).all(|(ra, rb)| {
                ra.len() == rb.len()
                    && ra
                        .iter()
                        .zip(rb.iter())
                        .all(|(x, y)| x.to_bits() == y.to_bits())
            })
    }

    #[test]
    fn shard_split_pools_to_single_run() {
        let n = 240usize;
        let n_perm = 300usize;
        let block = 1usize;
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        let ch = synthetic_channels(n, 0.5, 4, &mut state);
        let members = prepare_members(&ch);
        assert_eq!(members.len(), FAMILY_K);
        let epoch = iso_to_unix("2024-01-01T00:00:00Z").expect("epoch");
        let times: Vec<f64> = (0..n).map(|i| epoch + i as f64 * HOUR).collect();
        let (buckets, pos_bucket) = phase_data(&times);
        let seed = 0xDEAD_BEEF_1234_5678u64;

        let full = null_matrix(
            &members,
            0,
            n_perm,
            block,
            ResampleMode::Driver,
            seed,
            4,
            &buckets,
            &pos_bucket,
        );
        assert_eq!(full.len(), n_perm);

        let bounds = [n_perm / 3, 2 * n_perm / 3];
        let mut shards: Vec<Vec<Vec<f64>>> = Vec::new();
        let mut lo = 0usize;
        for &hi in &bounds {
            shards.push(null_matrix(
                &members,
                lo,
                hi,
                block,
                ResampleMode::Driver,
                seed,
                4,
                &buckets,
                &pos_bucket,
            ));
            lo = hi;
        }
        shards.push(null_matrix(
            &members,
            lo,
            n_perm,
            block,
            ResampleMode::Driver,
            seed,
            4,
            &buckets,
            &pos_bucket,
        ));

        let dir = std::env::temp_dir();
        let mut pooled: Vec<Vec<f64>> = Vec::new();
        for (si, shard) in shards.iter().enumerate() {
            let path = dir.join(format!("wy_max_t_probe_shard_{si}.bin"));
            let p = path.to_string_lossy().into_owned();
            assert!(write_null_matrix(&p, shard));
            let back = read_null_matrix(&p, members.len()).expect("read");
            assert!(bits_eq(shard, &back));
            pooled.extend(back);
            let _ = std::fs::remove_file(&path);
        }
        assert!(
            bits_eq(&full, &pooled),
            "pooled shards equal the single run"
        );

        let sigma_full = sigma_per_statistic(&full);
        let means_full = null_means_per_statistic(&full);
        let sigma_pool = sigma_per_statistic(&pooled);
        let means_pool = null_means_per_statistic(&pooled);
        let max_full = studentized_maxima(&full, &means_full, &sigma_full);
        let max_pool = studentized_maxima(&pooled, &means_pool, &sigma_pool);
        assert!(
            max_full
                .iter()
                .zip(max_pool.iter())
                .all(|(a, b)| a.to_bits() == b.to_bits())
        );
        let mut f_full: Vec<f64> = max_full.iter().copied().filter(|v| v.is_finite()).collect();
        let mut f_pool: Vec<f64> = max_pool.iter().copied().filter(|v| v.is_finite()).collect();
        f_full.sort_by(|a, b| a.total_cmp(b));
        f_pool.sort_by(|a, b| a.total_cmp(b));
        assert!(bits_eq(&[f_full], &[f_pool]));
    }
}
