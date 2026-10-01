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
const STATION_WORD: &str = "ABK";
const HOUR_START_WORD: &str = "2024-01-01";
const HOUR_END_WORD: &str = "2024-12-31";

const FAMILY_K: usize = 6;

const MONTH_HOUR_BUCKETS: usize = 24 * 12;

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


fn round_seed(round: u64, station: &str, year: i64) -> u64 {
    let mut s = 0xCBF2_9CE4_8422_2325u64 ^ round;
    for b in station.bytes() {
        s ^= b as u64;
        s = s.wrapping_mul(0x0000_0100_0000_01B3);
    }
    s ^= (year as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    splitmix64(&mut s);
    splitmix64(&mut s)
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn year_of_unix(t: f64) -> i64 {
    civil_from_days((t / DAY).floor() as i64).0
}


fn month_hour_bucket(t: f64) -> Option<usize> {
    let (_, m, _) = civil_from_days((t / DAY).floor() as i64);
    if !(1..=12).contains(&m) {
        return None;
    }
    let hour = (t.rem_euclid(DAY) / HOUR).floor() as i64;
    if !(0..24).contains(&hour) {
        return None;
    }
    Some(((m - 1) * 24 + hour) as usize)
}

fn phase_data(times: &[f64]) -> (Vec<Vec<usize>>, Vec<Option<usize>>) {
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); MONTH_HOUR_BUCKETS];
    let mut pos: Vec<Option<usize>> = Vec::with_capacity(times.len());
    for (i, &t) in times.iter().enumerate() {
        let b = month_hour_bucket(t);
        if let Some(id) = b {
            buckets[id].push(i);
        }
        pos.push(b);
    }
    (buckets, pos)
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


fn bootstrap_indices_seasonal(
    n: usize,
    block: usize,
    state: &mut u64,
    buckets: &[Vec<usize>],
    pos_bucket: &[Option<usize>],
) -> Vec<usize> {
    let mut out = Vec::with_capacity(n);
    if n == 0 {
        return out;
    }
    let b = block.min(n);
    while out.len() < n {
        let k = out.len();
        let Some(id) = pos_bucket.get(k).copied().flatten() else {
            return Vec::new();
        };
        let cands = match buckets.get(id) {
            Some(cands) => cands,
            None => return Vec::new(),
        };
        if cands.is_empty() {
            return Vec::new();
        }
        let start = cands[(splitmix64(state) as usize) % cands.len()];
        for j in 0..b {
            if out.len() >= n {
                break;
            }
            out.push((start + j) % n);
        }
    }
    out
}


fn inv_norm(p: f64) -> f64 {
    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_690e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239e0,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838e0,
        -2.549_732_539_343_734e0,
        4.374_664_141_464_968e0,
        2.938_163_982_698_783e0,
    ];
    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996e0,
        3.754_408_661_907_416e0,
    ];
    const P_LOW: f64 = 0.024_25;
    const P_HIGH: f64 = 1.0 - P_LOW;
    if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= P_HIGH {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    }
}


fn rank_gauss(v: &[f32]) -> Vec<f32> {
    let n = v.len();
    if n == 0 {
        return Vec::new();
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut out = vec![0f32; n];
    let mut i = 0usize;
    while i < n {
        let mut j = i;
        while j + 1 < n && v[order[j + 1]] == v[order[i]] {
            j += 1;
        }
        let avg = ((i + 1) as f64 + (j + 1) as f64) / 2.0;
        let p = (avg - 0.5) / n as f64;
        let z = inv_norm(p) as f32;
        for &o in &order[i..=j] {
            out[o] = z;
        }
        i = j + 1;
    }
    out
}

struct Member {
    label: &'static str,
    pair_id: usize,
    lag: usize,
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

fn family_candidates(transformed: &[Vec<f32>]) -> Vec<Member> {
    let dbdt = &transformed[0];
    let bz = &transformed[1];
    let speed = &transformed[2];
    let density = &transformed[3];
    let pairs: [(&'static str, &Vec<f32>, &Vec<f32>); FAMILY_K] = [
        ("Bz -> dB/dt", dbdt, bz),
        ("dB/dt -> Bz", bz, dbdt),
        ("Speed -> dB/dt", dbdt, speed),
        ("dB/dt -> Speed", speed, dbdt),
        ("Density -> dB/dt", dbdt, density),
        ("dB/dt -> Density", density, dbdt),
    ];
    let mut out = Vec::with_capacity(FAMILY_K * 2);
    for (pair_id, (label, target, driver)) in pairs.into_iter().enumerate() {
        for &lag in &[0usize, 1usize] {
            out.push(Member {
                label,
                pair_id,
                lag,
                target: (*target).clone(),
                driver: (*driver).clone(),
            });
        }
    }
    out
}

fn pair_lag_index_hash(pair_id: usize, lag: usize) -> u64 {
    let shift = if lag == 0 { 1 } else { lag };
    let mut s = (pair_id as u64)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(shift as u64);
    splitmix64(&mut s)
}


fn dedup_family(candidates: Vec<Member>) -> Vec<Member> {
    let mut seen: Vec<u64> = Vec::with_capacity(candidates.len());
    let mut out: Vec<Member> = Vec::with_capacity(FAMILY_K);
    for m in candidates {
        let h = pair_lag_index_hash(m.pair_id, m.lag);
        if seen.contains(&h) {
            continue;
        }
        seen.push(h);
        out.push(m);
    }
    out
}

fn prepare_members(raw: &[Vec<f32>]) -> Vec<Member> {
    let transformed: Vec<Vec<f32>> = raw.iter().map(|c| rank_gauss(c)).collect();
    dedup_family(family_candidates(&transformed))
}

fn null_matrix(
    members: &[Member],
    n_perm: usize,
    block: usize,
    mode: ResampleMode,
    seed: u64,
    threads: usize,
    buckets: &[Vec<usize>],
    pos_bucket: &[Option<usize>],
) -> Vec<Vec<f64>> {
    let m = members.len();
    let n = match members.first() {
        Some(member) => member.target.len(),
        None => return Vec::new(),
    };
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
                        ResampleMode::Driver => {
                            bootstrap_indices_seasonal(n, block, &mut state, buckets, pos_bucket)
                        }
                    };
                    if idx.len() != n {
                        continue;
                    }
                    for (mi, member) in members.iter().enumerate() {
                        for (k, &idx_k) in idx.iter().enumerate() {
                            buf[k] = member.driver[idx_k];
                        }
                        if let Some(te) = transfer_entropy_lag(&member.target, &buf, member.lag) {
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

fn null_means_per_statistic(nulls: &[Vec<f64>]) -> Vec<Option<f64>> {
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
            if vals.is_empty() {
                return None;
            }
            Some(vals.iter().sum::<f64>() / vals.len() as f64)
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
    times: &[f64],
    n_perm: usize,
    block: usize,
    alpha: f64,
    seed: u64,
    threads: usize,
    mode: ResampleMode,
) {
    let n = members[0].target.len();
    let mode_name = match mode {
        ResampleMode::Driver => "driver-bootstrap (seasonal block, month/hour matched)",
        ResampleMode::Condition => "condition-permutation (block)",
    };
    println!(
        "family: K = {} declared (index-hash dedup of {} candidate (pair,lag) statistics) | n = {} | B = {} | resample = {} | block = {} | alpha = {} | seed = {}",
        FAMILY_K,
        FAMILY_K * 2,
        n,
        n_perm,
        mode_name,
        block,
        alpha,
        seed
    );
    println!(
        "construction: rank-Gauss per channel before the TE; one resample of the driver dependence is shared across the K statistics per draw; each statistic is studentized and bias-corrected against its own null."
    );
    let mut observed: Vec<Option<f64>> = Vec::with_capacity(members.len());
    for m in members {
        observed.push(transfer_entropy_lag(&m.target, &m.driver, m.lag));
    }
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

    let (buckets, pos_bucket) = phase_data(times);
    let nulls = null_matrix(
        members,
        n_perm,
        block,
        mode,
        seed,
        threads,
        &buckets,
        &pos_bucket,
    );
    let sigma = sigma_per_statistic(&nulls);
    let null_means = null_means_per_statistic(&nulls);
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
            n_perm.min(400),
            block,
            alpha,
            seed,
            threads,
            mode,
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
    run_family(
        &members,
        &aligned_times,
        n_perm,
        block,
        alpha,
        seed,
        threads,
        mode,
    );
}
