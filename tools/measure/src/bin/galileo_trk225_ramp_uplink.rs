use std::collections::BTreeMap;
use std::fs;

use omegaflow::archivar::embedded_lsk;
use omegaflow::atdf::{extract, field_of, full_year, strip_markers, LOGICAL_RECORD, TKFORM};
use omegaflow::lsk::{days_from_civil, LeapSeconds};

const CACHES: &[&str] = &[
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5327328A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5337339A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5340341A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_6177179A.TDF",
];

fn tdb_of(year: i64, day: i64, h: i64, m: i64, s: i64, lsk: &LeapSeconds) -> Option<f64> {
    if day <= 0 || day > 366 {
        return None;
    }
    let days = days_from_civil(year, 1, 1)? + day - 1;
    let unix = days as f64 * 86400.0 + h as f64 * 3600.0 + m as f64 * 60.0 + s as f64;
    lsk.unix_to_tdb(unix)
}

fn civil_day(tdb: f64) -> String {
    let unix = tdb + 10957.5 * 86400.0;
    let days = (unix / 86400.0).floor() as i64;
    match omegaflow::spectral::civil_from_days(days) {
        Some((y, mo, d)) => format!("{y:04}-{mo:02}-{d:02}"),
        None => format!("day {days}"),
    }
}

fn fmt_utc(tdb: f64) -> String {
    let unix = tdb + 10957.5 * 86400.0;
    let rem = unix.rem_euclid(86400.0);
    let h = (rem / 3600.0) as i64;
    let m = ((rem % 3600.0) / 60.0) as i64;
    format!("{h:02}:{m:02}")
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_trk225_ramp_uplink_report.txt".to_string(),
    };
    let Some(lsk) = embedded_lsk() else {
        println!("naif0012 table void");
        return;
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo TRK-2-25 ramp records as the 3-way transmitter source (atdf2ascii route)".to_string());
    for cache in CACHES {
        let Ok(bytes) = fs::read(cache) else {
            out.push(format!("{cache}: read void"));
            continue;
        };
        let Some(stripped) = strip_markers(&bytes) else {
            out.push(format!("{cache}: marker strip void"));
            continue;
        };
        let nlog = stripped.len() / LOGICAL_RECORD;
        if nlog < 3 {
            out.push(format!("{cache}: {nlog} logical records"));
            continue;
        }
        let mut ramp_rec: Vec<(i64, i64, f64)> = Vec::new();
        let mut ramp_on: Vec<(i64, i64, f64)> = Vec::new();
        let mut tw_rec: Vec<(i64, i64, f64)> = Vec::new();
        let mut ramp_gm: BTreeMap<i64, usize> = BTreeMap::new();
        let mut ramp_xmtr_on: BTreeMap<i64, usize> = BTreeMap::new();
        let mut ramp_power_nz = 0usize;
        let mut ramp_freq_nz = 0usize;
        let mut ramp_ctrl: BTreeMap<i64, usize> = BTreeMap::new();
        for i in 2..nlog {
            let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
            let day = extract(rec, field_of(TKFORM, 4).unwrap());
            if day == 0 {
                continue;
            }
            let data_type = extract(rec, field_of(TKFORM, 12).unwrap());
            let gm = extract(rec, field_of(TKFORM, 13).unwrap());
            let station = extract(rec, field_of(TKFORM, 10).unwrap());
            let year = full_year(extract(rec, field_of(TKFORM, 3).unwrap()));
            let h = extract(rec, field_of(TKFORM, 5).unwrap());
            let m = extract(rec, field_of(TKFORM, 6).unwrap());
            let s = extract(rec, field_of(TKFORM, 7).unwrap());
            let Some(tdb) = tdb_of(year, day, h, m, s, &lsk) else {
                continue;
            };
            if data_type == 6 {
                *ramp_gm.entry(gm).or_insert(0) += 1;
                let xon = extract(rec, field_of(TKFORM, 26).unwrap());
                *ramp_xmtr_on.entry(xon).or_insert(0) += 1;
                let ctrl = extract(rec, field_of(TKFORM, 111).unwrap());
                *ramp_ctrl.entry(ctrl).or_insert(0) += 1;
                if extract(rec, field_of(TKFORM, 98).unwrap()) != 0 {
                    ramp_power_nz += 1;
                }
                if extract(rec, field_of(TKFORM, 116).unwrap()) != 0 {
                    ramp_freq_nz += 1;
                }
                ramp_rec.push((station, gm, tdb));
                if xon == 0 {
                    ramp_on.push((station, gm, tdb));
                }
            }
            if (data_type == 1 || data_type == 2) && (gm == 3 || gm == 4) {
                tw_rec.push((station, gm, tdb));
            }
        }
        out.push(format!(
            "{cache}: logical {nlog} ramp records {} three-way doppler {}",
            ramp_rec.len(),
            tw_rec.len()
        ));
        out.push(format!(
            "  ramp ground_mode {ramp_gm:?} | ramp XMTR_ON0 {ramp_xmtr_on:?} | ramp RAMP_CTRL {ramp_ctrl:?} | XMTR_POWER nonzero {ramp_power_nz} | XMTR_FREQ nonzero {ramp_freq_nz}"
        ));
        let mut st_ramp: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for (st, _, tdb) in &ramp_rec {
            st_ramp.entry(*st).or_default().push(*tdb);
        }
        for (st, ts) in &st_ramp {
            let lo = ts.iter().cloned().fold(f64::INFINITY, f64::min);
            let hi = ts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            out.push(format!(
                "  ramp station {st} n {} date {} {}..{} UTC",
                ts.len(),
                civil_day(lo),
                fmt_utc(lo),
                fmt_utc(hi)
            ));
        }
        let mut tw_st: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
        for (st, _, tdb) in &tw_rec {
            tw_st.entry(*st).or_default().push(*tdb);
        }
        for (st, ts) in &tw_st {
            let lo = ts.iter().cloned().fold(f64::INFINITY, f64::min);
            let hi = ts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            out.push(format!(
                "  three-way doppler station {st} n {} date {} {}..{} UTC",
                ts.len(),
                civil_day(lo),
                fmt_utc(lo),
                fmt_utc(hi)
            ));
        }
        if ramp_rec.is_empty() || tw_rec.is_empty() {
            out.push("  no ramp or no three-way -> transmitter-source route void in this file".to_string());
            continue;
        }
        out.push(format!("  ramp records with XMTR_ON0 == 0 (transmitter on): {}", ramp_on.len()));
        let ramp_src = if ramp_on.is_empty() { &ramp_rec } else { &ramp_on };
        let mut cov: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        let mut n_covered = 0usize;
        for (st_tw, _, tdb_tw) in &tw_rec {
            let mut covered = false;
            for (st_ramp, _, tdb_ramp) in ramp_src {
                if *st_ramp != *st_tw && (tdb_ramp - tdb_tw).abs() <= 300.0 {
                    *cov.entry((*st_tw, *st_ramp)).or_insert(0) += 1;
                    covered = true;
                }
            }
            if covered {
                n_covered += 1;
            }
        }
        out.push(format!(
            "  three-way samples with a different-station transmitter-on ramp within |dt| <= 300 s: {n_covered}/{}",
            tw_rec.len()
        ));
        if cov.is_empty() {
            out.push("    (no receiver, transmitter) pairs (0 honored)".to_string());
        } else {
            let mut kv: Vec<((i64, i64), usize)> = cov.into_iter().collect();
            kv.sort();
            for ((recv, xmtr), c) in kv {
                out.push(format!("    (three-way receiver st{recv}, ramp station st{xmtr}): {c} three-way samples"));
            }
        }
    }
    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
