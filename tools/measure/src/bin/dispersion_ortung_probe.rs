use omegaflow::archivar::{
    body_barycenter_position, embedded_lsk, parse_ephemeris_binary, BodyEphemeris, C_LIGHT,
};
use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use std::collections::HashMap;

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
const SUN_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_sun.bin";
const EARTH_EPH: &str = "data/ssd.jpl.nasa.gov/ephemeris_earth.bin";
const FLARE_THRESH: f64 = 5e-6;
const REFRACTORY: usize = 75;
const WINDOW_CELLS: usize = 100;
const FILL: f64 = -9999.0;
const ONSET_SIGMA: f64 = 10.0;
const AU_M: f64 = 1.495978707e11;
const LIGHT_TIME_EPS: f64 = 1e-9;

const AIA_BANDS: [(u32, &str, f64); 7] = [
    (5, "304A", 30.4e-9),
    (1, "131A", 13.1e-9),
    (2, "171A", 17.1e-9),
    (3, "193A", 19.3e-9),
    (4, "211A", 21.1e-9),
    (6, "335A", 33.5e-9),
    (0, "94A", 9.4e-9),
];

const CONES: [(usize, &str, f64); 3] = [
    (7, "XRSA", 0.225e-9),
    (6, "94A", 9.4e-9),
    (5, "335A", 33.5e-9),
];

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

fn rise_cell(grid: &[Option<f32>], peak: usize) -> Option<usize> {
    let lo = peak.saturating_sub(WINDOW_CELLS);
    let hi = (peak + WINDOW_CELLS + 1).min(grid.len());
    let mut mn = f32::INFINITY;
    let mut mx = f32::NEG_INFINITY;
    for k in lo..hi {
        if let Some(v) = grid[k] {
            mn = mn.min(v);
            mx = mx.max(v);
        }
    }
    if !(mn < mx) {
        return None;
    }
    let mid = mn + 0.5 * (mx - mn);
    let mut below = false;
    for k in lo..hi {
        if let Some(v) = grid[k] {
            if v < mid {
                below = true;
            } else if below {
                return Some(k);
            }
        }
    }
    None
}

fn peak_cell_of(grid: &[Option<f32>], peak: usize) -> Option<usize> {
    let lo = peak.saturating_sub(WINDOW_CELLS);
    let hi = (peak + WINDOW_CELLS + 1).min(grid.len());
    let mut best: Option<(usize, f32)> = None;
    for k in lo..hi {
        if let Some(v) = grid[k] {
            if best.map_or(true, |(_, bv)| v > bv) {
                best = Some((k, v));
            }
        }
    }
    best.map(|(k, _)| k)
}

fn circle_intersection(d_lo: &[f64], d_hi: &[f64]) -> (f64, f64) {
    let inter_lo = d_lo.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let inter_hi = d_hi.iter().cloned().fold(f64::INFINITY, f64::min);
    (inter_lo, inter_hi)
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

fn light_time_emission(
    t_arr_tdb: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<(f64, [f64; 3], [f64; 3], f64)> {
    let earth = body_barycenter_position("earth", t_arr_tdb, eph)?;
    let mut t_e = t_arr_tdb;
    for _ in 0..16 {
        let sun = body_barycenter_position("sun", t_e, eph)?;
        let d = ((sun[0] - earth[0]).powi(2)
            + (sun[1] - earth[1]).powi(2)
            + (sun[2] - earth[2]).powi(2))
        .sqrt();
        let t_next = t_arr_tdb - d / C_LIGHT;
        if (t_next - t_e).abs() < LIGHT_TIME_EPS {
            return Some((t_e, sun, earth, d));
        }
        t_e = t_next;
    }
    None
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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let aia_dir = arg_value(&args, "--aia-dir").unwrap_or(AIA_DIR.to_string());
    let goes_dirs = [
        arg_value(&args, "--goes-dir-2013").unwrap_or(GOES13_DIR.to_string()),
        arg_value(&args, "--goes-dir-2014").unwrap_or(GOES14_DIR.to_string()),
        arg_value(&args, "--goes-dir-2015").unwrap_or(GOES15_DIR.to_string()),
    ];
    let sun_eph_path = arg_value(&args, "--sun-eph").unwrap_or(SUN_EPH.to_string());
    let earth_eph_path = arg_value(&args, "--earth-eph").unwrap_or(EARTH_EPH.to_string());
    let wanted_epoch: Option<f64> = arg_value(&args, "--epoch").and_then(|s| s.parse().ok());
    let lsk = embedded_lsk();

    let year_cells = ((YEAR_UNIX[1] - YEAR_UNIX[0]) / DT) as usize;
    let n_cells = 3 * year_cells;

    println!("=== Dispersion localization test: one event, three cones, the Sun as calibration reference ===");
    println!(
        "Grid: {} x 24-s cells over {} .. {} (unix), 3 years.",
        n_cells, WINDOW_LO, WINDOW_HI
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
        if records.is_empty() {
            eprintln!("year {} AIA corpus reads void", year_num);
        }
        eprintln!("year {}: {} AIA records loaded", year_num, records.len());
        for (bi, &(bidx, _, _)) in AIA_BANDS.iter().enumerate() {
            let series: Vec<(f64, f64)> = records
                .iter()
                .filter(|r| r.2 == bidx)
                .map(|r| (r.0 + J2000_UNIX_OFFSET - TT_UNIX, r.1))
                .collect();
            let yr_grid = bin_median(&series, YEAR_UNIX[yi], year_cells);
            place(&mut aia_grids[bi], &yr_grid, yi * year_cells);
        }
        eprintln!("year {}: 7 AIA bands binned", year_num);
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
    println!(
        "94A candidate cut + year baseline (year median + {:.0} sigma); cone onset = the rising-edge crossing: the first cell at >= the midpoint of the cone's own window excursion (min + 0.5*(max-min)), preceded by a cell below that midpoint:",
        ONSET_SIGMA
    );
    println!(
        "  94A  | 2013 {:.4e} | 2014 {:.4e} | 2015 {:.4e}",
        aia_thr[6][0], aia_thr[6][1], aia_thr[6][2]
    );
    println!(
        "  335A | 2013 {:.4e} | 2014 {:.4e} | 2015 {:.4e}",
        aia_thr[5][0], aia_thr[5][1], aia_thr[5][2]
    );

    let mut candidates: Vec<usize> = Vec::new();
    for yi in 0..3 {
        let seg = &grids[6][yi * year_cells..(yi + 1) * year_cells];
        for p in cut_events(seg, aia_thr[6][yi] as f32, REFRACTORY) {
            candidates.push(yi * year_cells + p);
        }
    }
    println!(
        "event candidates: {} (94A 24-s median above the year onset threshold, refractory {} cells)",
        candidates.len(),
        REFRACTORY
    );

    let mut ranked = candidates.clone();
    ranked.sort_by(|&a, &b| {
        let sa = grids[6][a].unwrap() as f64 / aia_thr[6][year_of(WINDOW_LO + a as f64 * DT)];
        let sb = grids[6][b].unwrap() as f64 / aia_thr[6][year_of(WINDOW_LO + b as f64 * DT)];
        sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
    });

    let scan_list: Vec<usize> = match wanted_epoch {
        Some(epoch) => {
            let want_cell = ((epoch - WINDOW_LO) / DT).floor() as isize;
            candidates
                .iter()
                .copied()
                .filter(|&p| (p as isize - want_cell).unsigned_abs() <= WINDOW_CELLS as usize)
                .collect()
        }
        None => ranked.clone(),
    };

    let mut chosen: Option<(usize, usize, Vec<Option<f32>>, Vec<Option<f32>>)> = None;
    let mut scanned = 0usize;
    for &pk94 in &scan_list {
        if scanned >= 64 {
            break;
        }
        if rise_cell(&grids[6], pk94).is_none() || rise_cell(&grids[5], pk94).is_none() {
            eprintln!(
                "candidate at {}: no clean rise in 94A or 335A",
                date_string(WINDOW_LO + pk94 as f64 * DT)
            );
            continue;
        }
        scanned += 1;
        let day0 = ((WINDOW_LO + pk94 as f64 * DT) / 86400.0).floor() as i64;
        let (a_series, b_series) = load_goes_days(day0, &goes_dirs);
        if b_series.is_empty() {
            eprintln!(
                "candidate at {}: the day carries no GOES b_flux — no evidence",
                date_string(WINDOW_LO + pk94 as f64 * DT)
            );
            continue;
        }
        let a_slice = bin_median(&a_series, WINDOW_LO, n_cells);
        let b_slice = bin_median(&b_series, WINDOW_LO, n_cells);
        let lo = pk94.saturating_sub(WINDOW_CELLS);
        let hi = (pk94 + WINDOW_CELLS + 1).min(n_cells);
        let xrsa_rise = rise_cell(&a_slice, pk94);
        let mut xpk: Option<usize> = None;
        for k in lo..hi {
            if let Some(v) = b_slice[k] {
                if v >= FLARE_THRESH as f32 && xpk.map_or(true, |p| b_slice[p].unwrap() < v) {
                    xpk = Some(k);
                }
            }
        }
        let Some(xpk) = xpk else {
            eprintln!(
                "candidate at {}: XRSB stays under {:.0e} W/m2 in the window — no evidence",
                date_string(WINDOW_LO + pk94 as f64 * DT),
                FLARE_THRESH
            );
            continue;
        };
        if xrsa_rise.is_none() {
            eprintln!(
                "candidate at {}: XRSA carries no clean rise in the window",
                date_string(WINDOW_LO + pk94 as f64 * DT)
            );
            continue;
        }
        chosen = Some((pk94, xpk, a_slice, b_slice));
        break;
    }

    let Some((pk94, xpk, a_slice, b_slice)) = chosen else {
        println!();
        println!(
            "no candidate carries all three cones and the XRSB evidence within {} scanned — the localization stays unmeasured (0 honored)",
            scanned
        );
        return;
    };

    let rank = ranked
        .iter()
        .position(|&p| p == pk94)
        .map(|i| i + 1)
        .expect("the chosen event is one of the ranked candidates");
    let t_arr_unix = WINDOW_LO + xpk as f64 * DT;
    let xrsb_peak_val = b_slice[xpk].unwrap();
    println!();
    println!(
        "chosen event: rank {} by 94A peak — evidence: XRSB 24-s median peak {:.3e} W/m2 > {:.0e} at unix {:.0} ({}, TDB {} via LSK)",
        rank,
        xrsb_peak_val,
        FLARE_THRESH,
        t_arr_unix,
        date_string(t_arr_unix),
        lsk.as_ref()
            .and_then(|l| l.unix_to_tdb(t_arr_unix))
            .map(|t| format!("{:.3}", t))
            .unwrap_or("void".to_string())
    );

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    match std::fs::read(&sun_eph_path) {
        Ok(bytes) => match parse_ephemeris_binary(&bytes) {
            Some(e) => {
                eph.insert("sun".to_string(), e);
            }
            None => eprintln!("{} carries no ephemeris contract", sun_eph_path),
        },
        Err(_) => eprintln!("{} reads void", sun_eph_path),
    }
    match std::fs::read(&earth_eph_path) {
        Ok(bytes) => match parse_ephemeris_binary(&bytes) {
            Some(e) => {
                eph.insert("earth".to_string(), e);
            }
            None => eprintln!("{} carries no ephemeris contract", earth_eph_path),
        },
        Err(_) => eprintln!("{} reads void", earth_eph_path),
    }

    let Some(lsk) = lsk.as_ref() else {
        eprintln!("the embedded leap-second table reads void — TDB conversion stays unmeasured");
        return;
    };
    let Some(t_arr_tdb) = lsk.unix_to_tdb(t_arr_unix) else {
        eprintln!("the event epoch carries no TDB reading — the light-time basis stays unmeasured");
        return;
    };
    let Some((t_e, r_sun, r_earth, d_eph)) = light_time_emission(t_arr_tdb, &eph) else {
        eprintln!("the Sun or Earth ephemeris carries no reading at the event epoch — the calibration origin stays unmeasured (0 honored)");
        return;
    };

    let tau_lt = d_eph / C_LIGHT;
    let delta_d = C_LIGHT * DT;
    println!();
    println!("=== Light-time basis (ephemerides) ===");
    println!(
        "emission epoch t_E = t_arr - tau_lt = {:.3} s TDB (iterated to < {:.0e} s)",
        t_e, LIGHT_TIME_EPS
    );
    println!(
        "tau_lt = d/c = {:.3} s; d_eph = |r_sun(t_E) - r_earth(t_arr)| = {:.6e} m = {:.6} AU",
        tau_lt,
        d_eph,
        d_eph / AU_M
    );
    println!(
        "r_sun(t_E)   = [{:.3e}, {:.3e}, {:.3e}] m (ICRS, Barycenter)",
        r_sun[0], r_sun[1], r_sun[2]
    );
    println!(
        "r_earth(t_arr) = [{:.3e}, {:.3e}, {:.3e}] m (ICRS, Barycenter; the GOES-15 geostationary offset 42164 km = 0.14 light-s lies under the cell, named)",
        r_earth[0], r_earth[1], r_earth[2]
    );
    println!(
        "error-circle radius delta_d = c * dtau_min = {:.3e} m (dtau_min = {} s cell); scale delta_d/tau_lt = {:.2e} m/s ({:.1} % of c)",
        delta_d,
        DT,
        delta_d / tau_lt,
        delta_d / tau_lt / C_LIGHT * 100.0
    );

    println!();
    println!("=== Three cones: measured arrival latencies -> distances via the band speed (v = c, band-flat) ===");
    println!("cone    | lambda   | freq_hz    | rise unix       | rise tdb       | dtau_i = t_on - t_E (s) | d_i = c*dtau_i (m)     | eps_i = d_i - d_eph (s | m)");
    let mut d_lo: Vec<f64> = Vec::new();
    let mut d_hi: Vec<f64> = Vec::new();
    let mut d_center: Vec<f64> = Vec::new();
    let mut latencies: Vec<f64> = Vec::new();
    for &(gi, name, lam) in &CONES {
        let grid: &[Option<f32>] = if gi == 7 { &a_slice } else { &grids[gi] };
        match rise_cell(grid, pk94) {
            Some(k) => {
                let t_on_unix = WINDOW_LO + k as f64 * DT;
                let Some(t_on_tdb) = lsk.unix_to_tdb(t_on_unix) else {
                    println!(
                        "{:<7} | {:.2e} | {:.3e} | carries no TDB reading — cone void",
                        name,
                        lam,
                        C_LIGHT / lam
                    );
                    continue;
                };
                let dtau = t_on_tdb - t_e;
                let d_i = C_LIGHT * dtau;
                let eps_s = dtau - tau_lt;
                let eps_m = d_i - d_eph;
                d_lo.push(d_i - delta_d);
                d_hi.push(d_i + delta_d);
                d_center.push(d_i);
                latencies.push(dtau);
                let rise = peak_cell_of(grid, pk94).map(|k| WINDOW_LO + k as f64 * DT - t_on_unix);
                println!(
                    "{:<7} | {:.2e} | {:.3e} | {:.0} | {:.3} | {:>22.1} | {:>22.3e} | {:>8.1} | {:>10.3e}{}",
                    name,
                    lam,
                    C_LIGHT / lam,
                    t_on_unix,
                    t_on_tdb,
                    dtau,
                    d_i,
                    eps_s,
                    eps_m,
                    rise.map(|r| format!("  (peak {} s after onset)", r as i64)).unwrap_or(String::new())
                );
            }
            None => {
                println!(
                    "{:<7} | {:.2e} | {:.3e} | no clean rise in the window — cone void",
                    name,
                    lam,
                    C_LIGHT / lam
                );
            }
        }
    }

    println!();
    println!("=== Error circle: the intersection of the three circles ===");
    if d_lo.len() != CONES.len() {
        println!(
            "the intersection stays unmeasured — cones void: {} of {} carry an onset",
            d_lo.len(),
            CONES.len()
        );
        return;
    }
    let (inter_lo, inter_hi) = circle_intersection(&d_lo, &d_hi);
    let circle_center = d_center.iter().sum::<f64>() / d_center.len() as f64;
    if inter_lo <= inter_hi {
        println!(
            "intersection [ {:.6e}, {:.6e} ] m = [ {:.6}, {:.6} ] AU (non-empty)",
            inter_lo,
            inter_hi,
            inter_lo / AU_M,
            inter_hi / AU_M
        );
    } else {
        println!(
            "intersection empty — the three circles carry no common annulus; the gap {:.6e} m is the measured disagreement",
            inter_lo - inter_hi
        );
    }
    println!("d_eph = {:.6e} m = {:.6} AU", d_eph, d_eph / AU_M);
    let hit = inter_lo <= d_eph && d_eph <= inter_hi;
    let offset_m = d_eph - circle_center;
    println!();
    println!("=== Verdict ===");
    if hit {
        println!(
            "the localization hits the calibration origin: d_eph lies inside the error circle [ {:.6e}, {:.6e} ] m",
            inter_lo, inter_hi
        );
    } else {
        println!(
            "the localization misses the calibration origin: offset = {:.6e} m = {:.1} s of light time (d_eph - mean of the three d_i, {:.6e} m); the per-cone eps_i columns above carry the measured numbers",
            offset_m,
            offset_m / C_LIGHT,
            circle_center
        );
    }
    let (f_lo, f_hi) = CONES
        .iter()
        .map(|&(_, _, lam)| C_LIGHT / lam)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), f| {
            (lo.min(f), hi.max(f))
        });
    println!(
        "cone frequency basis: {:.3e} Hz .. {:.3e} Hz (span {:.2} decades); error-circle scale dtau*c/tau_lt = {:.2e} m/s",
        f_lo,
        f_hi,
        (f_hi / f_lo).log10(),
        delta_d / tau_lt
    );
    println!(
        "measured latencies per cone: {}",
        latencies
            .iter()
            .map(|v| format!("{:.1} s", v))
            .collect::<Vec<String>>()
            .join(", ")
    );
}

#[cfg(test)]
mod tests {
    use super::{circle_intersection, cut_events, rise_cell, year_of};

    #[test]
    fn rise_is_the_midpoint_crossing_after_a_trough() {
        let grid: Vec<Option<f32>> = vec![
            None,
            Some(2.0),
            Some(3.0),
            Some(9.0),
            Some(10.0),
            Some(12.0),
            Some(13.0),
            Some(2.0),
        ];
        assert_eq!(rise_cell(&grid, 4), Some(3));
    }

    #[test]
    fn a_local_bump_on_an_elevated_floor_is_a_rise() {
        let grid: Vec<Option<f32>> = vec![
            Some(10.0),
            Some(9.0),
            Some(11.0),
            Some(12.0),
            Some(13.0),
            Some(12.0),
            Some(10.0),
            Some(11.0),
        ];
        assert_eq!(rise_cell(&grid, 4), Some(2));
    }

    #[test]
    fn a_flat_window_carries_no_rise() {
        let grid: Vec<Option<f32>> = vec![Some(5.0); 8];
        assert_eq!(rise_cell(&grid, 4), None);
    }

    #[test]
    fn rise_absent_without_data() {
        let grid: Vec<Option<f32>> = vec![None, None, None];
        assert_eq!(rise_cell(&grid, 1), None);
    }

    #[test]
    fn events_cut_on_the_threshold_and_refractory() {
        let grid: Vec<Option<f32>> = vec![
            None,
            None,
            Some(6e-6),
            Some(8e-6),
            Some(3e-6),
            Some(1e-6),
            Some(1e-6),
            Some(1e-6),
            Some(1e-6),
            Some(9e-6),
            Some(7e-6),
            Some(2e-6),
        ];
        let peaks = cut_events(&grid, 5e-6, 6);
        assert_eq!(peaks, vec![3, 9]);
    }

    #[test]
    fn year_boundaries_are_unix() {
        assert_eq!(year_of(1356998400.0), 0);
        assert_eq!(year_of(1388534400.0), 1);
        assert_eq!(year_of(1420070400.0), 2);
    }

    #[test]
    fn three_overlapping_circles_share_an_annulus() {
        let (lo, hi) = circle_intersection(&[100.0, 110.0, 120.0], &[140.0, 150.0, 160.0]);
        assert_eq!(lo, 120.0);
        assert_eq!(hi, 140.0);
    }

    #[test]
    fn disjoint_circles_carry_an_empty_intersection() {
        let (lo, hi) = circle_intersection(&[100.0, 150.0, 120.0], &[110.0, 160.0, 130.0]);
        assert!(lo > hi);
    }
}
