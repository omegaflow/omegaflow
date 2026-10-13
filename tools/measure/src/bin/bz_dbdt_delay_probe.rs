use omegaflow::archivar::cache_root;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::omni2::{COMP_BZ, parse_bin as parse_omni2};
use omegaflow::intermagnet::parse_bin as parse_dbdt_bin;
use std::collections::HashMap;

const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86400.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const OMNI2_BIN: &str = "omni2_serie_1h.bin";
const OMNI2_CDN_BASE: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov";
const IMAG_CDN_BASE: &str =
    "https://github.com/omegaflow/sources/releases/download/imag-data.bgs.ac.uk";
const STATION_WORD: &str = "ABK";
const START_WORD: &str = "2024-01-01";
const END_WORD: &str = "2024-12-31";
const DEFAULT_MAX_LAG_MIN: usize = 180;
const MIN_PAIRS: usize = 8;
const SHIFT_STEP_MIN: usize = 60;
const ALPHA: f64 = 0.05;

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
        if let Some(recs) = parse_omni2(&bytes) {
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
    match parse_omni2(&bytes) {
        Some(recs) => recs,
        None => {
            eprintln!("{path} parses void after fetch — the top series stays unmeasured");
            Vec::new()
        }
    }
}

fn load_station_dbdt_1m(station: &str) -> Option<Vec<(f64, f64)>> {
    let asset = format!("{}_dbdt_1m.bin", station.to_lowercase());
    let candidates = [
        std::path::PathBuf::from(disk_cache(&asset)),
        cache_root().join(&asset),
        std::path::PathBuf::from(&asset),
    ];
    for path in &candidates {
        if let Ok(bytes) = std::fs::read(path) {
            if let Some(recs) = parse_dbdt_bin(&bytes) {
                println!("{} dB/dt 1m loaded from {}", station, path.display());
                return Some(recs.into_iter().map(|(t, v, _)| (t, v)).collect());
            }
            eprintln!("{} reads {} but parses void", station, path.display());
        }
    }
    let url = format!("{IMAG_CDN_BASE}/{asset}");
    let bytes = fetch_raw_bytes(&url)?;
    let recs = parse_dbdt_bin(&bytes)?;
    let cache = std::path::PathBuf::from(disk_cache(&asset));
    if let Some(parent) = cache.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&cache, &bytes);
    Some(recs.into_iter().map(|(t, v, _)| (t, v)).collect())
}

fn shared_window(
    a: &[(f64, f64)],
    b: &[(f64, f64)],
    lo_clamp: Option<f64>,
    hi_clamp: Option<f64>,
) -> Option<(f64, f64)> {
    let a0 = a.first()?.0;
    let a1 = a.last()?.0;
    let b0 = b.first()?.0;
    let b1 = b.last()?.0;
    let lo = a0.max(b0).max(lo_clamp.unwrap_or(f64::NEG_INFINITY));
    let hi = a1.min(b1).min(hi_clamp.unwrap_or(f64::INFINITY));
    if hi <= lo { None } else { Some((lo, hi)) }
}

fn hour_step_minutes(bz: &[(f64, f64)], t0: f64, n: usize) -> Vec<Option<f32>> {
    let mut acc: HashMap<i64, (f64, u32)> = HashMap::new();
    for &(t, v) in bz {
        let key = (t / HOUR).floor() as i64;
        let entry = acc.entry(key).or_insert((0.0, 0));
        entry.0 += v;
        entry.1 += 1;
    }
    (0..n)
        .map(|i| {
            let t = t0 + i as f64 * MINUTE;
            let key = (t / HOUR).floor() as i64;
            acc.get(&key).map(|&(s, c)| (s / c as f64) as f32)
        })
        .collect()
}

fn minute_cells(series: &[(f64, f64)], t0: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / MINUTE).floor();
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

fn corr_at_lag(a: &[Option<f32>], b: &[Option<f32>], lag: usize) -> Option<(f64, usize)> {
    if lag >= a.len() {
        return None;
    }
    let mut n = 0u64;
    let mut mx = 0.0f64;
    let mut my = 0.0f64;
    let mut cxy = 0.0f64;
    let mut mxx = 0.0f64;
    let mut myy = 0.0f64;
    for i in 0..a.len() - lag {
        let (Some(x), Some(y)) = (a[i], b[i + lag]) else {
            continue;
        };
        let x = x as f64;
        let y = y as f64;
        n += 1;
        let dx = x - mx;
        mx += dx / n as f64;
        let dy = y - my;
        my += dy / n as f64;
        cxy += dx * (y - my);
        mxx += dx * (x - mx);
        myy += dy * (y - my);
    }
    if n < MIN_PAIRS as u64 {
        return None;
    }
    let denom = (mxx * myy).sqrt();
    if !(denom > 0.0) || !denom.is_finite() {
        return None;
    }
    let r = cxy / denom;
    if !r.is_finite() {
        return None;
    }
    Some((r, n as usize))
}

struct Moments {
    n: u64,
    sx: f64,
    sy: f64,
    sxx: f64,
    syy: f64,
    sxy: f64,
}

impl Moments {
    fn zero() -> Self {
        Moments {
            n: 0,
            sx: 0.0,
            sy: 0.0,
            sxx: 0.0,
            syy: 0.0,
            sxy: 0.0,
        }
    }

    fn push(&mut self, x: f64, y: f64) {
        self.n += 1;
        self.sx += x;
        self.sy += y;
        self.sxx += x * x;
        self.syy += y * y;
        self.sxy += x * y;
    }

    fn minus(&self, o: &Moments) -> Moments {
        Moments {
            n: self.n - o.n,
            sx: self.sx - o.sx,
            sy: self.sy - o.sy,
            sxx: self.sxx - o.sxx,
            syy: self.syy - o.syy,
            sxy: self.sxy - o.sxy,
        }
    }

    fn corr(&self) -> Option<f64> {
        if self.n < MIN_PAIRS as u64 {
            return None;
        }
        let n = self.n as f64;
        let cxy = self.sxy - self.sx * self.sy / n;
        let mxx = self.sxx - self.sx * self.sx / n;
        let myy = self.syy - self.sy * self.sy / n;
        let denom = (mxx * myy).sqrt();
        if !(denom > 0.0) || !denom.is_finite() {
            return None;
        }
        let r = cxy / denom;
        if r.is_finite() { Some(r) } else { None }
    }
}

fn corr_block_se(
    a: &[Option<f32>],
    b: &[Option<f32>],
    lag: usize,
    block: usize,
) -> Option<(f64, f64, usize)> {
    if lag >= a.len() || block == 0 {
        return None;
    }
    let mut total = Moments::zero();
    let mut blocks: Vec<Moments> = Vec::new();
    let mut m = 0usize;
    for i in 0..a.len() - lag {
        let (Some(x), Some(y)) = (a[i], b[i + lag]) else {
            continue;
        };
        let (x, y) = (x as f64, y as f64);
        let bi = m / block;
        if bi >= blocks.len() {
            blocks.push(Moments::zero());
        }
        blocks[bi].push(x, y);
        total.push(x, y);
        m += 1;
    }
    if m < MIN_PAIRS || blocks.len() < 2 {
        return None;
    }
    let r = total.corr()?;
    let mut mean = 0.0f64;
    let mut kept = 0usize;
    let mut deltas: Vec<f64> = Vec::with_capacity(blocks.len());
    for blk in &blocks {
        if let Some(rj) = total.minus(blk).corr() {
            deltas.push(rj);
            mean += rj;
            kept += 1;
        }
    }
    if kept < 2 {
        return None;
    }
    mean /= kept as f64;
    let mut sum_dev = 0.0f64;
    for rj in &deltas {
        let d = rj - mean;
        sum_dev += d * d;
    }
    let se = (((kept - 1) as f64 / kept as f64) * sum_dev).sqrt();
    if se.is_finite() {
        Some((r, se, m))
    } else {
        None
    }
}

fn driver_block_length(driver: &[Option<f32>], cap: usize) -> Option<usize> {
    let mut lag = 1usize;
    while lag <= cap {
        match corr_at_lag(driver, driver, lag) {
            Some((r, _)) if r <= 0.0 => return Some(lag),
            Some(_) => lag += 1,
            None => return None,
        }
    }
    None
}

fn studentized_max(
    a: &[Option<f32>],
    b: &[Option<f32>],
    lmax: usize,
    block: usize,
) -> Option<(f64, Option<f64>)> {
    let mut max_r: Option<f64> = None;
    let mut max_t: Option<f64> = None;
    for lag in 0..=lmax {
        let Some((r, se, _)) = corr_block_se(a, b, lag, block) else {
            continue;
        };
        let ar = r.abs();
        if max_r.map_or(true, |v| ar > v) {
            max_r = Some(ar);
        }
        if se > 0.0 && se.is_finite() {
            let t = (r / se).abs();
            if t.is_finite() && max_t.map_or(true, |v| t > v) {
                max_t = Some(t);
            }
        }
    }
    max_r.map(|r| (r, max_t))
}

fn quantile(sorted: &[f64], alpha: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let rank = ((1.0 - alpha) * sorted.len() as f64) as usize;
    Some(sorted[rank.min(sorted.len() - 1)])
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let station = arg_after(&args, "--station")
        .unwrap_or(STATION_WORD)
        .to_string();
    let start = arg_after(&args, "--start").unwrap_or(START_WORD);
    let end = arg_after(&args, "--end").unwrap_or(END_WORD);
    let max_lag_min = arg_after(&args, "--max-lag-min")
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_MAX_LAG_MIN);
    let n_surr = match arg_after(&args, "--n-surr").and_then(|v| v.parse::<usize>().ok()) {
        Some(v) => v,
        None => 0,
    };

    println!("=== Bz -> dB/dt delay probe at the minute grain (1-min witness) ===");
    println!(
        "station = {station} | window = {start} .. {end} | max lag = {max_lag_min} min | method = minute-lag Pearson cross-correlation"
    );

    let omni = load_omni2(&disk_cache(OMNI2_BIN));
    let bz: Vec<(f64, f64)> = omni
        .iter()
        .filter(|(_, _, c)| *c == COMP_BZ)
        .map(|&(t, v, _)| (t + J2000_UNIX_OFFSET, v))
        .collect();
    let Some(dbdt_1m) = load_station_dbdt_1m(&station) else {
        println!("{station} minute |dB/dt| absent — the CDN asset stays pending; no measurement");
        return;
    };
    println!(
        "omni2_serie_1h.bin: Bz {} | {station} 1-min |dB/dt| {}",
        bz.len(),
        dbdt_1m.len()
    );
    if bz.is_empty() {
        println!("OMNI2 Bz series absent — no measurement");
        return;
    }

    let lo_clamp = iso_to_unix(&format!("{start}T00:00:00Z"));
    let hi_clamp = iso_to_unix(&format!("{end}T00:00:00Z"));
    let Some((lo, hi)) = shared_window(&bz, &dbdt_1m, lo_clamp, hi_clamp) else {
        println!(
            "common minute window absent — no minute shared by OMNI2 Bz and {station} |dB/dt|"
        );
        return;
    };
    let t0 = (lo / MINUTE).floor() * MINUTE;
    let n_min = ((hi - t0) / MINUTE).floor();
    if n_min < MIN_PAIRS as f64 {
        println!(
            "common minute window spans fewer than {MIN_PAIRS} minutes — Z absent; no measurement"
        );
        return;
    }
    let n = n_min as usize;
    let bz_min = hour_step_minutes(&bz, t0, n);
    let dbdt_min = minute_cells(&dbdt_1m, t0, n);
    let complete = bz_min
        .iter()
        .zip(dbdt_min.iter())
        .filter(|(a, b)| a.is_some() && b.is_some())
        .count();
    println!(
        "window {} .. {} | {} minute cells | {} cells carry both series",
        start, end, n, complete
    );
    if complete < MIN_PAIRS {
        println!("common complete minute cells fewer than {MIN_PAIRS} — Z absent; no measurement");
        return;
    }

    let lmax = max_lag_min.min(n.saturating_sub(1));
    println!("{:>7} | {:>10} | {:>8}", "lag_min", "r", "n");
    let mut best: Option<(usize, f64, usize)> = None;
    for lag in 0..=lmax {
        match corr_at_lag(&bz_min, &dbdt_min, lag) {
            Some((r, pairs)) => {
                println!("{lag:>7} | {r:>10.5} | {pairs:>8}");
                if best.map_or(true, |(_, br, _)| r.abs() > br.abs()) {
                    best = Some((lag, r, pairs));
                }
            }
            None => println!("{lag:>7} | {:>10} | {:>8}", "absent", "< 8"),
        }
    }

    let Some((z, r, pairs)) = best else {
        println!("no lag carries enough complete minute pairs — Z absent; no measurement");
        return;
    };
    println!("Z (Bz -> dB/dt peak at the minute grain) = {z} min | r = {r:.5} | n = {pairs}");
    println!(
        "Bz threshold = pending (named open slot — needs station X and the calibrated alpha; not set)"
    );

    if n_surr == 0 {
        println!(
            "significance: pending — the calibrated minute-null (hour-aligned circular shift, block-jackknife studentized max over lags) is built but not run; pass --n-surr N to calibrate."
        );
    } else {
        let cap = lmax.max(SHIFT_STEP_MIN);
        let Some(block) = driver_block_length(&bz_min, cap) else {
            println!(
                "significance: the driver's autocorrelation turns non-positive nowhere within {cap} min — the block length is absent; the studentized null stays pending"
            );
            return;
        };
        println!(
            "null block length measured from the Bz step-driver autocorrelation = {block} min (first non-positive lag; search cap {cap} min)"
        );
        let Some((obs_r, obs_t)) = studentized_max(&bz_min, &dbdt_min, lmax, block) else {
            println!(
                "significance: the observed statistic is absent — no lag carries enough complete pairs for a block SE"
            );
            return;
        };
        let Some(obs_z) = obs_t else {
            println!(
                "significance: observed max |T| absent — the block jackknife carries no finite SE; the studentized null stays pending"
            );
            return;
        };
        let max_hours = n / SHIFT_STEP_MIN;
        let lo_hours = (lmax / SHIFT_STEP_MIN) + 1;
        if max_hours <= lo_hours {
            println!(
                "significance: the shared window spans fewer than {lo_hours} whole hours — an hour-aligned circular shift is absent; the studentized null stays pending"
            );
            return;
        }
        let span_hours = max_hours - lo_hours;
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let mut ge_t = 0usize;
        let mut ge_r = 0usize;
        let mut finite = 0usize;
        let mut m_vals: Vec<f64> = Vec::with_capacity(n_surr);
        for _ in 0..n_surr {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let hours = lo_hours + (state >> 33) as usize % span_hours;
            let offset = hours * SHIFT_STEP_MIN;
            let shifted: Vec<Option<f32>> = (0..n).map(|i| bz_min[(i + offset) % n]).collect();
            if let Some((mr, mt)) = studentized_max(&shifted, &dbdt_min, lmax, block) {
                finite += 1;
                if mr >= obs_r {
                    ge_r += 1;
                }
                if let Some(mt) = mt {
                    if mt >= obs_z {
                        ge_t += 1;
                    }
                    m_vals.push(mt);
                }
            }
        }
        if finite == 0 || m_vals.is_empty() {
            println!(
                "significance: no surrogate carried a finite statistic — the calibrated null stays pending"
            );
            return;
        }
        let p_r = (ge_r as f64 + 1.0) / (finite as f64 + 1.0);
        let p_adj = (ge_t as f64 + 1.0) / (m_vals.len() as f64 + 1.0);
        m_vals.sort_by(|a, b| a.total_cmp(b));
        let Some(crit) = quantile(&m_vals, ALPHA) else {
            println!("significance: the studentized null carries no quantile — pending");
            return;
        };
        println!(
            "significance: hour-aligned circular-shift null over {finite} finite surrogates (shift step {} min) | observed max |r| = {obs_r:.5} | p_family(raw) = {p_r:.4}",
            SHIFT_STEP_MIN
        );
        println!(
            "studentized null: Z = max |T| = {obs_z:.4} (block-jackknife SE, block = {block} min) | p_adj = {p_adj:.4}"
        );
        println!(
            "calibrated critical value: M_(1-{ALPHA}) = {crit:.4} over {} studentized surrogates | the observed Z {} the {:.0}% bound",
            m_vals.len(),
            if obs_z >= crit {
                "exceeds"
            } else {
                "stays within"
            },
            (1.0 - ALPHA) * 100.0
        );
        println!(
            "significance note: the shift preserves the hourly-step boundaries (multiples of {} min) and the SEs come from the driver's measured block length; the prior arbitrary-minute circular-shift max-|r| null is replaced.",
            SHIFT_STEP_MIN
        );
    }
}
