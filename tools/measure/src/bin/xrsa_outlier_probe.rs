use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};

const AIA_MAGIC: [u8; 4] = *b"AIA1";
const DT: f64 = 24.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const TT_UNIX: f64 = 32.184;
const WINDOW_LO: f64 = 1356998400.0;
const WINDOW_HI: f64 = 1451606400.0;
const YEAR_UNIX: [f64; 3] = [1356998400.0, 1388534400.0, 1420070400.0];
const YEAR_NUMBER: [u32; 3] = [2013, 2014, 2015];
const AIA_DIR: &str = "data/jsoc.stanford.edu";
const GOES13_DIR: &str = "data/ncei.noaa.gov/goes15_2013";
const GOES14_DIR: &str = "data/ncei.noaa.gov/goes15";
const GOES15_DIR: &str = "data/ncei.noaa.gov/goes15_2015";
const FLARE_THRESH: f64 = 5e-6;
const REFRACTORY: usize = 75;
const WINDOW_CELLS: usize = 100;
const FILL: f64 = -9999.0;
const ONSET_SIGMA: f64 = 10.0;
const FRACTIONS: [f64; 3] = [0.25, 0.5, 0.75];

const AIA_BANDS: [(u32, &str, f64); 7] = [
    (5, "304A", 30.4e-9),
    (1, "131A", 13.1e-9),
    (2, "171A", 17.1e-9),
    (3, "193A", 19.3e-9),
    (4, "211A", 21.1e-9),
    (6, "335A", 33.5e-9),
    (0, "94A", 9.4e-9),
];

const CONES: [(usize, &str); 3] = [(7, "XRSA"), (6, "94A"), (5, "335A")];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_aia_lines(path: &str) -> Vec<(f64, f64, u32)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{} reads void", path);
        return Vec::new();
    };
    if bytes.len() < 8 || bytes[0..4] != AIA_MAGIC {
        eprintln!("{} carries no AIA1 contract", path);
        return Vec::new();
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let o = 8 + i * 20;
        let Some(t) = bytes
            .get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(v) = bytes
            .get(o + 8..o + 16)
            .and_then(|b| b.try_into().ok())
            .map(f64::from_le_bytes)
        else {
            continue;
        };
        let Some(idx) = bytes
            .get(o + 16..o + 20)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
        else {
            continue;
        };
        out.push((t, v, idx));
    }
    out
}

fn goes_xrs_both(path: &str) -> Option<(Vec<(f64, f64)>, Vec<(f64, f64)>)> {
    let bytes = std::fs::read(path).ok()?;
    let file = Hdf5File::parse(&bytes).ok()?;
    let (Ok(t_raw), Ok(a_raw), Ok(af_raw), Ok(b_raw), Ok(bf_raw)) = (
        file.read_dataset("time"),
        file.read_dataset("a_flux"),
        file.read_dataset("a_flags"),
        file.read_dataset("b_flux"),
        file.read_dataset("b_flags"),
    ) else {
        return None;
    };
    let n = t_raw.len() / 8;
    if a_raw.len() != n * 4
        || af_raw.len() != n * 2
        || b_raw.len() != n * 4
        || bf_raw.len() != n * 2
    {
        return None;
    }
    let mut a_out: Vec<(f64, f64)> = Vec::with_capacity(n);
    let mut b_out: Vec<(f64, f64)> = Vec::with_capacity(n);
    for i in 0..n {
        let Some(t) = decode_f64(&t_raw, i * 8, Endian::Le) else {
            continue;
        };
        let af = u16::from_le_bytes([af_raw[i * 2], af_raw[i * 2 + 1]]);
        let bf = u16::from_le_bytes([bf_raw[i * 2], bf_raw[i * 2 + 1]]);
        if af == 0 {
            if let Some(v) = decode_f32(&a_raw, i * 4, Endian::Le) {
                let v = v as f64;
                if v.is_finite() && v != FILL && v > 0.0 {
                    a_out.push((t, v));
                }
            }
        }
        if bf == 0 {
            if let Some(v) = decode_f32(&b_raw, i * 4, Endian::Le) {
                let v = v as f64;
                if v.is_finite() && v != FILL && v > 0.0 {
                    b_out.push((t, v));
                }
            }
        }
    }
    a_out.sort_by(|x, y| x.0.total_cmp(&y.0));
    b_out.sort_by(|x, y| x.0.total_cmp(&y.0));
    Some((a_out, b_out))
}

fn ymd_name(unix: f64) -> String {
    let days = (unix / 86400.0).floor() as i64;
    let (y, m, d) = civil_from_days(days);
    format!("xr_{:04}{:02}{:02}.nc", y, m, d)
}

fn load_goes_days(day0: i64, dirs: &[String]) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut a_all: Vec<(f64, f64)> = Vec::new();
    let mut b_all: Vec<(f64, f64)> = Vec::new();
    for d in day0 - 1..=day0 + 1 {
        let name = ymd_name(d as f64 * 86400.0);
        for dir in dirs {
            let p = format!("{}/{}", dir, name);
            if let Some((a, b)) = goes_xrs_both(&p) {
                a_all.extend(a);
                b_all.extend(b);
                break;
            }
        }
    }
    a_all.sort_by(|x, y| x.0.total_cmp(&y.0));
    b_all.sort_by(|x, y| x.0.total_cmp(&y.0));
    (a_all, b_all)
}

fn bin_median(series: &[(f64, f64)], t0: f64, bins: usize) -> Vec<Option<f32>> {
    let mut acc: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(t, v) in series {
        let idx = ((t - t0) / DT).floor();
        if idx < 0.0 || idx >= bins as f64 {
            continue;
        }
        acc[idx as usize].push(v);
    }
    acc.into_iter()
        .map(|mut v| {
            if v.is_empty() {
                return None;
            }
            v.sort_by(|a, b| a.total_cmp(b));
            let m = if v.len() % 2 == 0 {
                (v[v.len() / 2 - 1] + v[v.len() / 2]) * 0.5
            } else {
                v[v.len() / 2]
            };
            Some(m as f32)
        })
        .collect()
}

fn place(grid: &mut [Option<f32>], sub: &[Option<f32>], offset: usize) {
    for (i, cell) in sub.iter().enumerate() {
        if let Some(v) = cell {
            grid[offset + i] = Some(*v);
        }
    }
}

fn cut_events(trig: &[Option<f32>], threshold: f32, refractory: usize) -> Vec<usize> {
    let n = trig.len();
    let mut peaks = Vec::new();
    let mut i = 0usize;
    while i < n {
        let Some(tv) = trig[i] else {
            i += 1;
            continue;
        };
        if tv < threshold {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut peak = i;
        while j < n && j - i < refractory {
            if let Some(v) = trig[j] {
                if v < threshold && j > i + 3 {
                    break;
                }
                if let Some(pv) = trig[peak] {
                    if v > pv {
                        peak = j;
                    }
                }
            }
            j += 1;
        }
        peaks.push(peak);
        i = j.max(i + refractory);
    }
    peaks
}

fn year_of(t: f64) -> usize {
    if t < YEAR_UNIX[1] {
        0
    } else if t < YEAR_UNIX[2] {
        1
    } else {
        2
    }
}

fn median_sd(vals: &[f32]) -> (Option<f64>, Option<f64>) {
    let n = vals.len();
    if n == 0 {
        return (None, None);
    }
    let mut sorted: Vec<f64> = vals.iter().map(|&v| v as f64).collect();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let med = if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) * 0.5
    } else {
        sorted[n / 2]
    };
    let mean = sorted.iter().sum::<f64>() / n as f64;
    let var = sorted.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n as f64;
    (Some(med), Some(var.sqrt()))
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s: Vec<f64> = vals.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    let n = s.len();
    Some(if n % 2 == 0 {
        (s[n / 2 - 1] + s[n / 2]) * 0.5
    } else {
        s[n / 2]
    })
}

fn mean_sd(vals: &[f64]) -> (f64, f64) {
    let n = vals.len() as f64;
    if n == 0.0 {
        return (0.0, 0.0);
    }
    let mean = vals.iter().sum::<f64>() / n;
    let var = vals.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n;
    (mean, var.sqrt())
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (y + if m <= 2 { 1 } else { 0 }, m as u32, d as u32)
}

fn date_string(unix: f64) -> String {
    let days = (unix / 86400.0).floor() as i64;
    let secs = unix - days as f64 * 86400.0;
    let (y, m, d) = civil_from_days(days);
    let h = (secs / 3600.0).floor() as u32;
    let mi = ((secs - h as f64 * 3600.0) / 60.0).floor() as u32;
    let s = secs - h as f64 * 3600.0 - mi as f64 * 60.0;
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:05.2}", y, m, d, h, mi, s)
}

fn window_series(grid: &[Option<f32>], lo: usize, hi: usize) -> Vec<Option<f64>> {
    (lo..hi).map(|k| grid[k].map(|v| v as f64)).collect()
}

fn min_max(series: &[Option<f64>]) -> (Option<f64>, Option<f64>) {
    let mut mn = f64::INFINITY;
    let mut mx = f64::NEG_INFINITY;
    for v in series.iter().flatten() {
        mn = mn.min(*v);
        mx = mx.max(*v);
    }
    if mn == f64::INFINITY {
        (None, None)
    } else {
        (Some(mn), Some(mx))
    }
}

fn rise_at_fraction(series: &[Option<f64>], mn: f64, mx: f64, frac: f64) -> Option<usize> {
    let level = mn + frac * (mx - mn);
    let mut below = false;
    for (i, v) in series.iter().enumerate() {
        if let Some(x) = v {
            if *x < level {
                below = true;
            } else if below {
                return Some(i);
            }
        }
    }
    None
}

struct EventTiming {
    xrsb_peak_unix: f64,
    xrsb_peak_val: f64,
    rise_unix: [[Option<f64>; 3]; 3],
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let aia_dir = arg_value(&args, "--aia-dir").unwrap_or(AIA_DIR.to_string());
    let goes_dirs = [
        arg_value(&args, "--goes-dir-2013").unwrap_or(GOES13_DIR.to_string()),
        arg_value(&args, "--goes-dir-2014").unwrap_or(GOES14_DIR.to_string()),
        arg_value(&args, "--goes-dir-2015").unwrap_or(GOES15_DIR.to_string()),
    ];

    let year_cells = ((YEAR_UNIX[1] - YEAR_UNIX[0]) / DT) as usize;
    let n_cells = 3 * year_cells;

    println!("=== XRSA outlier probe: three channels (XRSA/94A/335A), rise profile at 25/50/75% of the excursion, conditioned on the XRSB peak ===");
    println!(
        "grid: {} x 24-s cells over {} .. {} (unix); window +-40 min; the 94A candidate inventory (year median + {:.0} sigma, refractory {} cells)",
        n_cells, WINDOW_LO, WINDOW_HI, ONSET_SIGMA, REFRACTORY
    );

    let mut aia_grids: Vec<Vec<Option<f32>>> = (0..7).map(|_| vec![None; n_cells]).collect();
    for (yi, &year_num) in YEAR_NUMBER.iter().enumerate() {
        let mut records: Vec<(f64, f64, u32)> = Vec::new();
        if year_num == 2014 {
            for m in 1..=12u32 {
                let p = format!("{}/aia2014_{:02}.bin", aia_dir, m);
                records.extend(read_aia_lines(&p));
            }
        } else {
            let p = format!("{}/aia{}_fullyear.bin", aia_dir, year_num);
            records = read_aia_lines(&p);
        }
        for (bi, &(bidx, _, _)) in AIA_BANDS.iter().enumerate() {
            let series: Vec<(f64, f64)> = records
                .iter()
                .filter(|r| r.2 == bidx)
                .map(|r| (r.0 + J2000_UNIX_OFFSET - TT_UNIX, r.1))
                .collect();
            let yr_grid = bin_median(&series, YEAR_UNIX[yi], year_cells);
            place(&mut aia_grids[bi], &yr_grid, yi * year_cells);
        }
        eprintln!("year {}: AIA binned", year_num);
    }
    let grids = aia_grids;

    let mut aia_thr: Vec<[f64; 3]> = Vec::with_capacity(7);
    for grid in grids.iter() {
        let mut by_year: [Vec<f32>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (k, cell) in grid.iter().enumerate() {
            if let Some(v) = cell {
                by_year[k / year_cells].push(*v);
            }
        }
        let mut yr_thr = [0.0f64; 3];
        for (yi, vals) in by_year.iter().enumerate() {
            let (Some(med), Some(sd)) = median_sd(vals) else {
                yr_thr[yi] = f64::NAN;
                continue;
            };
            yr_thr[yi] = med + ONSET_SIGMA * sd;
        }
        aia_thr.push(yr_thr);
    }

    let mut candidates: Vec<usize> = Vec::new();
    for yi in 0..3 {
        let seg = &grids[6][yi * year_cells..(yi + 1) * year_cells];
        for p in cut_events(seg, aia_thr[6][yi] as f32, REFRACTORY) {
            candidates.push(yi * year_cells + p);
        }
    }
    let mut ranked = candidates.clone();
    ranked.sort_by(|&a, &b| {
        let sa = grids[6][a].unwrap() as f64 / aia_thr[6][year_of(WINDOW_LO + a as f64 * DT)];
        let sb = grids[6][b].unwrap() as f64 / aia_thr[6][year_of(WINDOW_LO + b as f64 * DT)];
        sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
    });
    println!("event candidates: {} (94A)", candidates.len());

    let mut timings: Vec<EventTiming> = Vec::new();
    for &pk94 in &ranked {
        let lo = pk94.saturating_sub(WINDOW_CELLS);
        let hi = (pk94 + WINDOW_CELLS + 1).min(n_cells);
        let day0 = ((WINDOW_LO + pk94 as f64 * DT) / 86400.0).floor() as i64;
        let (a_series, b_series) = load_goes_days(day0, &goes_dirs);
        if b_series.is_empty() {
            continue;
        }
        let a_slice = bin_median(&a_series, WINDOW_LO, n_cells);
        let b_slice = bin_median(&b_series, WINDOW_LO, n_cells);

        let xrsa = window_series(&a_slice, lo, hi);
        let c94 = window_series(&grids[6], lo, hi);
        let c335 = window_series(&grids[5], lo, hi);
        let series: [&[Option<f64>]; 3] = [&xrsa, &c94, &c335];

        let mut rise_unix: [[Option<f64>; 3]; 3] = [[None; 3]; 3];
        let mut complete = true;
        for (ci, s) in series.iter().enumerate() {
            let (Some(mn), Some(mx)) = min_max(s) else {
                complete = false;
                break;
            };
            if !(mn < mx) {
                complete = false;
                break;
            }
            for (fi, &frac) in FRACTIONS.iter().enumerate() {
                let r = rise_at_fraction(s, mn, mx, frac);
                rise_unix[ci][fi] = r.map(|i| WINDOW_LO + (lo + i) as f64 * DT);
                if r.is_none() {
                    complete = false;
                }
            }
        }
        if !complete {
            continue;
        }

        let mut xpk: Option<usize> = None;
        for k in lo..hi {
            if let Some(v) = b_slice[k] {
                if v >= FLARE_THRESH as f32 && xpk.map_or(true, |p| b_slice[p].unwrap() < v) {
                    xpk = Some(k);
                }
            }
        }
        let Some(xpk) = xpk else {
            continue;
        };
        let xrsb_peak_unix = WINDOW_LO + xpk as f64 * DT;
        let xrsb_peak_val = b_slice[xpk].unwrap() as f64;
        timings.push(EventTiming {
            xrsb_peak_unix,
            xrsb_peak_val,
            rise_unix,
        });
    }

    if timings.is_empty() {
        println!();
        println!("no event carries a clean rise at all three fractions in all three channels — the outlier probe stays unmeasured (0 honored)");
        return;
    }

    println!();
    println!(
        "events with a clean rise at 25/50/75% in all three channels: {}",
        timings.len()
    );
    println!();
    println!("per event: leads of each channel's rise relative to the XRSB peak (negative = before the peak), seconds:");
    println!(
        "{:>19} | {:>9} | XRSA 25/50/75 | 94A 25/50/75 | 335A 25/50/75",
        "event", "XRSB peak"
    );
    for t in &timings {
        let lead = |ci: usize, fi: usize| match t.rise_unix[ci][fi] {
            Some(u) => format!("{:>6.0}", (u - t.xrsb_peak_unix) / DT * DT),
            None => "     -".to_string(),
        };
        println!(
            "{:>19} | {:>9.2e} | {:>12} | {:>12} | {:>12}",
            date_string(t.xrsb_peak_unix),
            t.xrsb_peak_val,
            format!("{} {} {}", lead(0, 0), lead(0, 1), lead(0, 2)),
            format!("{} {} {}", lead(1, 0), lead(1, 1), lead(1, 2)),
            format!("{} {} {}", lead(2, 0), lead(2, 1), lead(2, 2)),
        );
    }

    println!();
    println!("stacked: median lead per channel per fraction (seconds before the XRSB peak):");
    println!("channel | 25% median (sd) | 50% median (sd) | 75% median (sd)");
    let mut med: [[Option<f64>; 3]; 3] = [[None; 3]; 3];
    let mut sd: [[Option<f64>; 3]; 3] = [[None; 3]; 3];
    for ci in 0..3 {
        for fi in 0..3 {
            let vals: Vec<f64> = timings
                .iter()
                .filter_map(|t| t.rise_unix[ci][fi].map(|u| u - t.xrsb_peak_unix))
                .collect();
            med[ci][fi] = median(&vals);
            let (m, s) = mean_sd(&vals);
            sd[ci][fi] = if m.is_finite() { Some(s) } else { None };
        }
    }
    for (ci, (_, name)) in CONES.iter().enumerate() {
        let f = |fi: usize| match (med[ci][fi], sd[ci][fi]) {
            (Some(m), Some(s)) => format!("{:>7.0} ({:>5.0})", m, s),
            _ => "     -".to_string(),
        };
        println!("{:<7} | {:>16} | {:>16} | {:>16}", name, f(0), f(1), f(2));
    }

    println!();
    println!("the decisive comparison: the XRSA lead against 94A at the early fraction (25%) vs the midpoint (50%):");
    let xrsa_94_25: Vec<f64> = timings
        .iter()
        .filter_map(|t| match (t.rise_unix[0][0], t.rise_unix[1][0]) {
            (Some(a), Some(b)) => Some(a - b),
            _ => None,
        })
        .collect();
    let xrsa_94_50: Vec<f64> = timings
        .iter()
        .filter_map(|t| match (t.rise_unix[0][1], t.rise_unix[1][1]) {
            (Some(a), Some(b)) => Some(a - b),
            _ => None,
        })
        .collect();
    let xrsa_335_25: Vec<f64> = timings
        .iter()
        .filter_map(|t| match (t.rise_unix[0][0], t.rise_unix[2][0]) {
            (Some(a), Some(b)) => Some(a - b),
            _ => None,
        })
        .collect();
    let xrsa_335_50: Vec<f64> = timings
        .iter()
        .filter_map(|t| match (t.rise_unix[0][1], t.rise_unix[2][1]) {
            (Some(a), Some(b)) => Some(a - b),
            _ => None,
        })
        .collect();
    let (m25, s25) = (median(&xrsa_94_25), mean_sd(&xrsa_94_25).1);
    let (m50, s50) = (median(&xrsa_94_50), mean_sd(&xrsa_94_50).1);
    let (n25, s35_25) = (median(&xrsa_335_25), mean_sd(&xrsa_335_25).1);
    let (n50, s35_50) = (median(&xrsa_335_50), mean_sd(&xrsa_335_50).1);
    println!(
        "XRSA-94A  lead: 25% {:>7.0} s (sd {:>5.0}) | 50% {:>7.0} s (sd {:>5.0})",
        m25.unwrap_or(f64::NAN),
        s25,
        m50.unwrap_or(f64::NAN),
        s50
    );
    println!(
        "XRSA-335A lead: 25% {:>7.0} s (sd {:>5.0}) | 50% {:>7.0} s (sd {:>5.0})",
        n25.unwrap_or(f64::NAN),
        s35_25,
        n50.unwrap_or(f64::NAN),
        s35_50
    );

    let mut ordered = 0usize;
    let mut tot = 0usize;
    for t in &timings {
        if let (Some(a), Some(b), Some(c)) =
            (t.rise_unix[0][1], t.rise_unix[1][1], t.rise_unix[2][1])
        {
            tot += 1;
            if a <= b && b <= c {
                ordered += 1;
            }
        }
    }
    println!();
    println!(
        "ordering at 50%: XRSA <= 94A <= 335A in {} of {} events",
        ordered, tot
    );

    println!();
    println!("=== Verdict ===");
    let lead_25 = m25.map(|v| v.abs());
    let lead_50 = m50.map(|v| v.abs());
    match (lead_25, lead_50) {
        (Some(early_lead), Some(mid_lead)) if early_lead >= 0.6 * mid_lead && mid_lead > 24.0 => {
            println!(
                "quell-seitig (source-time-structure): the XRSA rise leads 94A already at the early fraction (25%: {:.0} s), not only at the midpoint (50%: {:.0} s) — the XRSA channel responds earlier in the flare, the -240 s outlier of the Ortungs-Test is the source speaking, not a channel-timing artifact",
                early_lead,
                mid_lead
            );
        }
        (Some(early_lead), Some(mid_lead)) if mid_lead > 24.0 && early_lead < 0.6 * mid_lead => {
            println!(
                "kanal-taktung (marker/shape): the XRSA lead appears only at the midpoint (50%: {:.0} s) while the early fraction is synchronized (25%: {:.0} s) — the midpoint marker is steeper for XRSA, the -240 s is the onset marker's shape, not an earlier source response",
                mid_lead,
                early_lead
            );
        }
        (Some(early_lead), Some(mid_lead)) => {
            println!(
                "no early lead: the XRSA and 94A rises are synchronized within the cell (25% lead {:.0} s, 50% lead {:.0} s) — the single-event -240 s does not repeat across the corpus; the outlier is event-local",
                early_lead,
                mid_lead
            );
        }
        _ => println!("the lead stays unmeasured — the median carries no value (0 honored)"),
    }
}
