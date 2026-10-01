use omegaflow::archivar::cache_root;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::omni2::{COMP_BZ, COMP_N1800, COMP_V1800, parse_bin};
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
const DEFAULT_STATION: &str = "ABK";
const DEFAULT_HOUR_START: &str = "2024-01-01";
const DEFAULT_HOUR_END: &str = "2024-12-31";
const SEASONS: usize = 4;

#[derive(Clone, Copy)]
enum ResampleMode {
    Driver,
    Condition,
}

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

fn align_channels(channels: &[&[Option<f32>]]) -> Vec<Vec<f32>> {
    let n = match channels.iter().map(|c| c.len()).min() {
        Some(n) => n,
        None => return Vec::new(),
    };
    let mut out: Vec<Vec<f32>> = vec![Vec::new(); channels.len()];
    for i in 0..n {
        if !channels.iter().all(|c| c[i].is_some()) {
            continue;
        }
        for (ci, c) in channels.iter().enumerate() {
            if let Some(v) = c[i] {
                out[ci].push(v);
            }
        }
    }
    out
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

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn uniform01(state: &mut u64) -> f64 {
    ((splitmix64(state) >> 11) as f64) / ((1u64 << 53) as f64)
}

fn gauss(state: &mut u64) -> f32 {
    loop {
        let u1 = uniform01(state) * 2.0 - 1.0;
        let u2 = uniform01(state) * 2.0 - 1.0;
        let s = u1 * u1 + u2 * u2;
        if s >= 1.0 || s <= 0.0 {
            continue;
        }
        return (u1 * (-2.0 * s.ln() / s).sqrt()) as f32;
    }
}

fn rng_for(seed: u64, replicate: usize) -> u64 {
    let mut s = seed ^ (replicate as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    splitmix64(&mut s);
    splitmix64(&mut s)
}

fn block_permutation(n: usize, block: usize, state: &mut u64) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..n).collect();
    if n == 0 {
        return perm;
    }
    if block <= 1 {
        for i in (1..n).rev() {
            let j = (splitmix64(state) as usize) % (i + 1);
            perm.swap(i, j);
        }
        return perm;
    }
    let mut blocks: Vec<Vec<usize>> = Vec::new();
    let mut i = 0usize;
    while i < n {
        let end = (i + block).min(n);
        blocks.push((i..end).collect());
        i = end;
    }
    let nb = blocks.len();
    let mut order: Vec<usize> = (0..nb).collect();
    for k in (1..nb).rev() {
        let j = (splitmix64(state) as usize) % (k + 1);
        order.swap(k, j);
    }
    let mut out = Vec::with_capacity(n);
    for &oi in &order {
        out.extend_from_slice(&blocks[oi]);
    }
    out
}

fn bootstrap_indices(n: usize, block: usize, seasons: usize, state: &mut u64) -> Vec<usize> {
    let mut out = Vec::with_capacity(n);
    if n == 0 {
        return out;
    }
    let b = block.min(n);
    let seasons = seasons.min(n);
    while out.len() < n {
        let k = out.len();
        let season = ((k * seasons) / n).min(seasons - 1);
        let s0 = (season * n) / seasons;
        let s1 = ((season + 1) * n) / seasons;
        if s1 <= s0 {
            break;
        }
        let span = s1 - s0;
        let start = s0 + (splitmix64(state) as usize) % span;
        for j in 0..b {
            if out.len() >= n {
                break;
            }
            let step = (start - s0 + j) % span;
            out.push(s0 + step);
        }
    }
    out
}

struct Member {
    label: &'static str,
    target: Vec<f32>,
    driver: Vec<f32>,
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

fn observed_members(aligned: &[Vec<f32>]) -> Vec<Member> {
    let dbdt = &aligned[0];
    let bz = &aligned[1];
    let speed = &aligned[2];
    let density = &aligned[3];
    vec![
        Member {
            label: "Bz -> dB/dt",
            target: dbdt.clone(),
            driver: bz.clone(),
        },
        Member {
            label: "dB/dt -> Bz",
            target: bz.clone(),
            driver: dbdt.clone(),
        },
        Member {
            label: "Speed -> dB/dt",
            target: dbdt.clone(),
            driver: speed.clone(),
        },
        Member {
            label: "dB/dt -> Speed",
            target: speed.clone(),
            driver: dbdt.clone(),
        },
        Member {
            label: "Density -> dB/dt",
            target: dbdt.clone(),
            driver: density.clone(),
        },
        Member {
            label: "dB/dt -> Density",
            target: density.clone(),
            driver: dbdt.clone(),
        },
    ]
}

fn null_matrix(
    members: &[Member],
    n: usize,
    n_perm: usize,
    block: usize,
    mode: ResampleMode,
    seed: u64,
    threads: usize,
) -> Vec<Vec<f64>> {
    let m = members.len();
    let mut nulls: Vec<Vec<f64>> = vec![vec![f64::NAN; m]; n_perm];
    let workers = threads.min(n_perm);
    let chunk = (n_perm + workers - 1) / workers;
    std::thread::scope(|s| {
        let mut handles = Vec::new();
        for (ti, slice) in nulls.chunks_mut(chunk).enumerate() {
            let start = ti * chunk;
            handles.push(s.spawn(move || {
                let mut buf = vec![0f32; n];
                for (off, row) in slice.iter_mut().enumerate() {
                    let replicate = start + off;
                    let mut state = rng_for(seed, replicate);
                    let idx = match mode {
                        ResampleMode::Condition => block_permutation(n, block, &mut state),
                        ResampleMode::Driver => bootstrap_indices(n, block, SEASONS, &mut state),
                    };
                    for (mi, member) in members.iter().enumerate() {
                        for (k, &idx_k) in idx.iter().enumerate() {
                            buf[k] = member.driver[idx_k];
                        }
                        if let Some(te) = transfer_entropy_lag(&member.target, &buf, 0) {
                            row[mi] = te;
                        }
                    }
                }
            }));
        }
        for h in handles {
            let _ = h.join();
        }
    });
    nulls
}

fn sigma_per_statistic(nulls: &[Vec<f64>]) -> Vec<Option<f64>> {
    let m = match nulls.first() {
        Some(row) => row.len(),
        None => return Vec::new(),
    };
    (0..m)
        .map(|mi| {
            let vals: Vec<f64> = nulls
                .iter()
                .map(|row| row[mi])
                .filter(|v| v.is_finite())
                .collect();
            if vals.len() < 2 {
                return None;
            }
            let mean = vals.iter().sum::<f64>() / vals.len() as f64;
            let var = vals.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>()
                / (vals.len() as f64 - 1.0);
            let sd = var.sqrt();
            if sd.is_finite() && sd > 0.0 {
                Some(sd)
            } else {
                None
            }
        })
        .collect()
}

fn studentized_maxima(nulls: &[Vec<f64>], sigma: &[Option<f64>]) -> Vec<f64> {
    nulls
        .iter()
        .map(|row| {
            let mut sup = f64::NEG_INFINITY;
            for (mi, &v) in row.iter().enumerate() {
                if let Some(s) = sigma.get(mi).copied().flatten() {
                    if v.is_finite() {
                        let t = v / s;
                        if t > sup {
                            sup = t;
                        }
                    }
                }
            }
            sup
        })
        .collect()
}

fn quantile(sorted: &[f64], alpha: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let b = sorted.len();
    let idx = ((1.0 - alpha) * b as f64).ceil() as usize;
    Some(sorted[idx.saturating_sub(1).min(b - 1)])
}

fn run_family(
    members: &[Member],
    n_perm: usize,
    block: usize,
    alpha: f64,
    seed: u64,
    threads: usize,
    mode: ResampleMode,
) {
    let n = members[0].target.len();
    let mode_name = match mode {
        ResampleMode::Driver => "driver-bootstrap (seasonal block)",
        ResampleMode::Condition => "condition-permutation (block)",
    };
    println!(
        "family: {} distinct pair statistics | n = {} | n_perm = {} | resample = {} | block = {} | alpha = {}",
        members.len(),
        n,
        n_perm,
        mode_name,
        block,
        alpha
    );
    let mut observed: Vec<Option<f64>> = Vec::with_capacity(members.len());
    for m in members {
        observed.push(transfer_entropy_lag(&m.target, &m.driver, 0));
    }
    for (m, te) in members.iter().zip(observed.iter()) {
        match te {
            Some(v) => println!("observed {} | lag 0/1 h | TE {} | n {}", m.label, v, n),
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

    let nulls = null_matrix(members, n, n_perm, block, mode, seed, threads);
    let sigma = sigma_per_statistic(&nulls);
    for (m, s) in members.iter().zip(sigma.iter()) {
        match s {
            Some(s) => println!("sigma {} = {s:.4e}", m.label),
            None => println!("sigma {} absent — the null carries no spread", m.label),
        }
    }
    let stud_null = studentized_maxima(&nulls, &sigma);
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
            let s = sigma.get(mi).copied().flatten()?;
            Some((*o)? / s)
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
        "{:<18} | {:>10} | {:>10} | {:>12} | verdict",
        "member", "TE_obs", "T_stud", "p_adj"
    );
    for (mi, m) in members.iter().enumerate() {
        let Some(v) = observed[mi] else {
            println!(
                "{:<18} | {:>10} | {:>10} | {:>12} | TE absent",
                m.label, "pending", "pending", "pending"
            );
            continue;
        };
        let Some(vs) = obs_stud[mi] else {
            println!(
                "{:<18} | {:>10.4e} | {:>10} | {:>12} | sigma absent",
                m.label, v, "pending", "pending"
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
        println!(
            "{:<18} | {:>10.4e} | {:>10.4e} | {:>12.4e} | {}",
            m.label, v, vs, p, verdict
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let selftest = args.iter().any(|a| a == "--selftest");
    let station = arg_after(&args, "--station")
        .unwrap_or(DEFAULT_STATION)
        .to_string();
    let hour_start = arg_after(&args, "--hour-start")
        .unwrap_or(DEFAULT_HOUR_START)
        .to_string();
    let hour_end = arg_after(&args, "--hour-end")
        .unwrap_or(DEFAULT_HOUR_END)
        .to_string();
    let n_perm = arg_after(&args, "--n-perm")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(2000);
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
    let seed = arg_after(&args, "--seed")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0x9E37_79B9_7F4A_7C15);
    let mode = match arg_after(&args, "--resample").unwrap_or("driver") {
        "condition" => ResampleMode::Condition,
        "driver" => ResampleMode::Driver,
        other => {
            eprintln!("--resample {other} names no mode — driver|condition; driver stands");
            ResampleMode::Driver
        }
    };
    let gpd = args.iter().any(|a| a == "--gpd");

    println!("=== Westfall-Young max-T permutation probe (Bz / L1 drivers -> dB/dt) ===");
    if let Some(now) = now_unix() {
        println!("system time: {now:.0} unix");
    }
    println!(
        "construction: the six distinct (pair) TE statistics share one resample of the driver dependence per replicate; each statistic is divided by its own null spread before the maximum (studentized)."
    );
    if gpd {
        println!(
            "GPD tail: pending — the empirical studentized max-T quantile carries the family bound; the estimator stays untouched."
        );
    }

    if selftest {
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        let ch = synthetic_channels(240, 0.5, 4, &mut state);
        let members = observed_members(&ch);
        println!(
            "selftest: four independent AR(1) a=0.5 channels, n = {} (no data fetch)",
            members[0].target.len()
        );
        run_family(&members, n_perm.min(400), block, alpha, seed, threads, mode);
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
    let bz = bin_cells(&omni_bz, t0, HOUR, n_cells);
    let speed = bin_cells(&omni_speed, t0, HOUR, n_cells);
    let density = bin_cells(&omni_density, t0, HOUR, n_cells);
    let dbdt = bin_cells(&dbdt_1h, t0, HOUR, n_cells);
    let aligned = align_channels(&[&dbdt, &bz, &speed, &density]);
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
    let members = observed_members(&aligned);
    run_family(&members, n_perm, block, alpha, seed, threads, mode);
}
