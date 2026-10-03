use crate::te::transfer_entropy_lag;

pub const FAMILY_K: usize = 6;

pub const MONTH_HOUR_BUCKETS: usize = 24 * 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResampleMode {
    Driver,
    Condition,
}

pub struct Member {
    pub label: String,
    pub pair_id: usize,
    pub lag: usize,
    pub target: Vec<f32>,
    pub driver: Vec<f32>,
}

impl Member {
    pub fn new(
        label: impl Into<String>,
        pair_id: usize,
        lag: usize,
        target: Vec<f32>,
        driver: Vec<f32>,
    ) -> Self {
        Self {
            label: label.into(),
            pair_id,
            lag,
            target,
            driver,
        }
    }
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

pub fn gauss(state: &mut u64) -> f32 {
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

pub fn round_seed(round: u64, station: &str, year: i64) -> u64 {
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

pub fn year_of_unix(t: f64) -> i64 {
    civil_from_days((t / 86_400.0).floor() as i64).0
}

fn month_hour_bucket(t: f64) -> Option<usize> {
    let (_, m, _) = civil_from_days((t / 86_400.0).floor() as i64);
    if !(1..=12).contains(&m) {
        return None;
    }
    let hour = (t.rem_euclid(86_400.0) / 3_600.0).floor() as i64;
    if !(0..24).contains(&hour) {
        return None;
    }
    Some(((m - 1) * 24 + hour) as usize)
}

pub struct PhaseIndex {
    pub buckets: Vec<Vec<usize>>,
    pub pos_bucket: Vec<Option<usize>>,
}

pub fn phase_data(times: &[f64]) -> PhaseIndex {
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); MONTH_HOUR_BUCKETS];
    let mut pos: Vec<Option<usize>> = Vec::with_capacity(times.len());
    for (i, &t) in times.iter().enumerate() {
        let b = month_hour_bucket(t);
        if let Some(id) = b {
            buckets[id].push(i);
        }
        pos.push(b);
    }
    PhaseIndex {
        buckets,
        pos_bucket: pos,
    }
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

pub fn bootstrap_indices_seasonal(
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
        1.383_577_518_672_69e2,
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

pub fn rank_gauss(v: &[f32]) -> Vec<f32> {
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

fn pair_lag_index_hash(pair_id: usize, lag: usize) -> u64 {
    let shift = if lag == 0 { 1 } else { lag };
    let mut s = (pair_id as u64)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(shift as u64);
    splitmix64(&mut s)
}

pub fn dedup_family(candidates: Vec<Member>) -> Vec<Member> {
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

pub fn family_candidates(transformed: &[Vec<f32>]) -> Vec<Member> {
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
            out.push(Member::new(
                label,
                pair_id,
                lag,
                (*target).clone(),
                (*driver).clone(),
            ));
        }
    }
    out
}

pub fn prepare_members(raw: &[Vec<f32>]) -> Vec<Member> {
    let transformed: Vec<Vec<f32>> = raw.iter().map(|c| rank_gauss(c)).collect();
    dedup_family(family_candidates(&transformed))
}

pub fn null_matrix(
    members: &[Member],
    perm: std::ops::Range<usize>,
    block: usize,
    mode: ResampleMode,
    seed: u64,
    threads: usize,
    phase: &PhaseIndex,
) -> Vec<Vec<f64>> {
    let m = members.len();
    let n = match members.first() {
        Some(member) => member.target.len(),
        None => return Vec::new(),
    };
    let count = perm.end.saturating_sub(perm.start);
    let mut nulls: Vec<Vec<f64>> = vec![vec![f64::NAN; m]; count];
    if count == 0 {
        return nulls;
    }
    let workers = threads.min(count);
    let chunk = count.div_ceil(workers);
    std::thread::scope(|s| {
        let mut handles = Vec::new();
        for (ti, slice) in nulls.chunks_mut(chunk).enumerate() {
            let start = perm.start + ti * chunk;
            handles.push(s.spawn(move || {
                let mut buf = vec![0f32; n];
                for (off, row) in slice.iter_mut().enumerate() {
                    let replicate = start + off;
                    let mut state = rng_for(seed, replicate);
                    let idx = match mode {
                        ResampleMode::Condition => block_permutation(n, block, &mut state),
                        ResampleMode::Driver => bootstrap_indices_seasonal(
                            n,
                            block,
                            &mut state,
                            &phase.buckets,
                            &phase.pos_bucket,
                        ),
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

pub fn sigma_per_statistic(nulls: &[Vec<f64>]) -> Vec<Option<f64>> {
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

pub fn null_means_per_statistic(nulls: &[Vec<f64>]) -> Vec<Option<f64>> {
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

pub fn studentized_maxima(
    nulls: &[Vec<f64>],
    means: &[Option<f64>],
    sigma: &[Option<f64>],
) -> Vec<f64> {
    nulls
        .iter()
        .map(|row| {
            let mut sup = f64::NEG_INFINITY;
            for (mi, &v) in row.iter().enumerate() {
                let (Some(m), Some(s)) = (
                    means.get(mi).copied().flatten(),
                    sigma.get(mi).copied().flatten(),
                ) else {
                    continue;
                };
                if v.is_finite() {
                    let t = (v - m) / s;
                    if t > sup {
                        sup = t;
                    }
                }
            }
            sup
        })
        .collect()
}

pub fn quantile(sorted: &[f64], alpha: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let b = sorted.len();
    let idx = ((1.0 - alpha) * b as f64).ceil() as usize;
    Some(sorted[idx.saturating_sub(1).min(b - 1)])
}

pub fn observed_family(members: &[Member]) -> Vec<Option<f64>> {
    members
        .iter()
        .map(|m| transfer_entropy_lag(&m.target, &m.driver, m.lag))
        .collect()
}
