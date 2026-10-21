//! Gaia DR4 epoch-astrometry dry run.
//!
//! Reads the ESA DR4 prerelease VOTable (12 sources, June 2026) and fits a
//! linear proper-motion model to the along-scan / cross-scan centroid
//! positions. This is the method that will be applied to the 512 ICRF anchors
//! when the official DR4 lands (2026-12-02).
//!
//! Usage:
//!   gaia_dr4_probe --xml <GAIA_DR4_PRERELEASE_EPOCH_ASTROMETRY_RAW.xml>
//!
//! The fit is deliberately simple: slope of centroid position vs time, with
//! the residual RMS printed. No attitude, calibration, or along/cross-to-RA/Dec
//! rotation is applied. Those belong to the full DR4 pass, not this dry run.

use omegaflow::archivar::{JsonVal, jnum, votable_to_json};
use std::collections::BTreeMap;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn field<'a>(row: &'a JsonVal, name: &str) -> Option<&'a JsonVal> {
    match row {
        JsonVal::Obj(map) => map.get(name),
        _ => None,
    }
}

fn nums(value: Option<&JsonVal>) -> Vec<f64> {
    match value {
        Some(JsonVal::Arr(items)) => items
            .iter()
            .filter_map(|v| match v {
                JsonVal::Num(n) if n.is_finite() => Some(*n),
                _ => None,
            })
            .collect(),
        Some(JsonVal::Num(n)) if n.is_finite() => vec![*n],
        Some(JsonVal::Str(s)) => s
            .split_whitespace()
            .filter_map(|x| x.parse::<f64>().ok())
            .filter(|x| x.is_finite())
            .collect(),
        _ => Vec::new(),
    }
}

fn bools(value: Option<&JsonVal>) -> Vec<bool> {
    match value {
        Some(JsonVal::Arr(items)) => items
            .iter()
            .filter_map(|v| match v {
                JsonVal::Bool(b) => Some(*b),
                JsonVal::Num(n) => Some(*n != 0.0),
                _ => None,
            })
            .collect(),
        Some(JsonVal::Bool(b)) => vec![*b],
        _ => Vec::new(),
    }
}

fn solve3(mut a: [[f64; 3]; 3], mut b: [f64; 3]) -> Option<[f64; 3]> {
    for col in 0..3 {
        let mut pivot = col;
        for row in col + 1..3 {
            if a[row][col].abs() > a[pivot][col].abs() {
                pivot = row;
            }
        }
        if a[pivot][col].abs() < 1e-18 {
            return None;
        }
        a.swap(col, pivot);
        b.swap(col, pivot);
        let d = a[col][col];
        for k in col..3 {
            a[col][k] /= d;
        }
        b[col] /= d;
        for row in 0..3 {
            if row == col {
                continue;
            }
            let f = a[row][col];
            for k in col..3 {
                a[row][k] -= f * a[col][k];
            }
            b[row] -= f * b[col];
        }
    }
    Some(b)
}

fn fit_pm(ts: &[f64], al: &[f64], pa: &[f64]) -> Option<(f64, f64, f64)> {
    let n = ts.len().min(al.len()).min(pa.len());
    if n < 3 {
        return None;
    }
    let mt = ts[..n].iter().sum::<f64>() / n as f64;
    let mut ata = [[0.0f64; 3]; 3];
    let mut atb = [0.0f64; 3];
    for i in 0..n {
        let dt = ts[i] - mt;
        let p = pa[i].to_radians();
        let row = [dt * p.sin(), dt * p.cos(), 1.0];
        for r in 0..3 {
            atb[r] += row[r] * al[i];
            for c in 0..3 {
                ata[r][c] += row[r] * row[c];
            }
        }
    }
    let [pm_ra, pm_dec, c] = solve3(ata, atb)?;
    let rms = (0..n)
        .map(|i| {
            let dt = ts[i] - mt;
            let p = pa[i].to_radians();
            let model = pm_ra * dt * p.sin() + pm_dec * dt * p.cos() + c;
            let r = al[i] - model;
            r * r
        })
        .sum::<f64>()
        / n as f64;
    Some((pm_ra, pm_dec, rms.sqrt()))
}

fn solve4(mut a: [[f64; 4]; 4], mut b: [f64; 4]) -> Option<[f64; 4]> {
    for col in 0..4 {
        let mut pivot = col;
        for row in col + 1..4 {
            if a[row][col].abs() > a[pivot][col].abs() {
                pivot = row;
            }
        }
        if a[pivot][col].abs() < 1e-18 {
            return None;
        }
        a.swap(col, pivot);
        b.swap(col, pivot);
        let d = a[col][col];
        for k in col..4 {
            a[col][k] /= d;
        }
        b[col] /= d;
        for row in 0..4 {
            if row == col {
                continue;
            }
            let f = a[row][col];
            for k in col..4 {
                a[row][k] -= f * a[col][k];
            }
            b[row] -= f * b[col];
        }
    }
    Some(b)
}

fn fit_pm4(ts: &[f64], al: &[f64], pa: &[f64], pf: &[f64]) -> Option<(f64, f64, f64, f64)> {
    let n = ts.len().min(al.len()).min(pa.len()).min(pf.len());
    if n < 4 {
        return None;
    }
    let mt = ts[..n].iter().sum::<f64>() / n as f64;
    let mut ata = [[0.0f64; 4]; 4];
    let mut atb = [0.0f64; 4];
    for i in 0..n {
        let dt = ts[i] - mt;
        let p = pa[i].to_radians();
        let row = [dt * p.sin(), dt * p.cos(), pf[i], 1.0];
        for r in 0..4 {
            atb[r] += row[r] * al[i];
            for c in 0..4 {
                ata[r][c] += row[r] * row[c];
            }
        }
    }
    let [pm_ra, pm_dec, plx, c] = solve4(ata, atb)?;
    let rms = (0..n)
        .map(|i| {
            let dt = ts[i] - mt;
            let p = pa[i].to_radians();
            let model = pm_ra * dt * p.sin() + pm_dec * dt * p.cos() + plx * pf[i] + c;
            let r = al[i] - model;
            r * r
        })
        .sum::<f64>()
        / n as f64;
    Some((pm_ra, pm_dec, plx, rms.sqrt()))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = arg_value(&args, "--xml") else {
        eprintln!("usage: gaia_dr4_probe --xml <GAIA_DR4_PRERELEASE_EPOCH_ASTROMETRY_RAW.xml>");
        std::process::exit(2);
    };
    let body = match std::fs::read_to_string(&path) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("{path}: {e}");
            std::process::exit(1);
        }
    };
    let Some(JsonVal::Arr(rows)) = votable_to_json(&body) else {
        eprintln!("{path}: VOTable did not parse into rows");
        std::process::exit(1);
    };
    println!("file {path}");
    println!("rows {}", rows.len());

    // Group by (ra0, dec0) — exact enough for 12 distinct sources and immune to
    // the f64 rounding of the 64-bit source_id in the generic VOTable reader.
    let mut groups: BTreeMap<(i64, i64), Vec<(f64, f64, f64, f64)>> = BTreeMap::new();
    let mut ids: BTreeMap<(i64, i64), f64> = BTreeMap::new();
    for row in &rows {
        let ra0 = jnum(row, "ra0");
        let dec0 = jnum(row, "dec0");
        let (Some(ra0), Some(dec0)) = (ra0, dec0) else {
            continue;
        };
        let key = ((ra0 * 1e6).round() as i64, (dec0 * 1e6).round() as i64);
        if let Some(id) = jnum(row, "source_id") {
            ids.entry(key).or_insert(id);
        }
        let times = nums(field(row, "obs_time_tcb"));
        let al = nums(field(row, "centroid_pos_al"));
        let pa = nums(field(row, "scan_pos_angle"));
        let pf = jnum(row, "parallax_factor_al").unwrap_or(0.0);
        let used = bools(field(row, "used_by_agis_al"));
        let entry = groups.entry(key).or_default();
        for i in 0..times.len().min(al.len()).min(pa.len()) {
            if !used.is_empty() && i < used.len() && !used[i] {
                continue;
            }
            // ns since the VOTable time origin; the origin cancels in a slope.
            let years = times[i] / 1e9 / 86400.0 / 365.25;
            entry.push((years, al[i], pa[i], pf));
        }
    }

    println!("sources {}", groups.len());
    for (key, epochs) in &groups {
        let ts: Vec<f64> = epochs.iter().map(|(t, _, _, _)| *t).collect();
        let al: Vec<f64> = epochs.iter().map(|(_, a, _, _)| *a).collect();
        let pa: Vec<f64> = epochs.iter().map(|(_, _, p, _)| *p).collect();
        let pf: Vec<f64> = epochs.iter().map(|(_, _, _, f)| *f).collect();
        let span = ts
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), t| {
                (lo.min(*t), hi.max(*t))
            });
        let use_plx = pf.iter().any(|p| p.abs() > 1e-9);
        let (pm_ra, pm_dec, plx, rms) = if use_plx {
            match fit_pm4(&ts, &al, &pa, &pf) {
                Some((a, d, p, r)) => (a, d, Some(p), r),
                None => (f64::NAN, f64::NAN, None, f64::NAN),
            }
        } else {
            match fit_pm(&ts, &al, &pa) {
                Some((a, d, r)) => (a, d, None, r),
                None => (f64::NAN, f64::NAN, None, f64::NAN),
            }
        };
        println!(
            "source_id {:.0} ra0 {:.6} dec0 {:.6} | epochs {} | span {:.2} a | pm_ra {:+.4} mas/a | pm_dec {:+.4} mas/a | plx {} | rms {:.3} mas",
            ids.get(key).copied().unwrap_or(f64::NAN),
            key.0 as f64 / 1e6,
            key.1 as f64 / 1e6,
            epochs.len(),
            span.1 - span.0,
            pm_ra,
            pm_dec,
            plx.map(|p| format!("{p:+.4} mas")).unwrap_or_else(|| "n/a".into()),
            rms
        );
    }
}
