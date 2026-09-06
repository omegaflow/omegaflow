use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read};

use omegaflow::archivar::embedded_lsk;
use omegaflow::atdf::{extract, field_of, full_year, strip_markers, LOGICAL_RECORD, TKFORM};
use omegaflow::lsk::{days_from_civil, LeapSeconds};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR_AGC: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const RUN_GAP_S: f64 = 600.0;
const TRANS_HZ: f64 = 1.0e2;
const MERGE_S: f64 = 60.0;
const ADJ_S: f64 = 60.0;

const CACHES: &[&str] = &[
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5327328A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5337339A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5340341A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_6177179A.TDF",
];

#[derive(Clone, Copy)]
struct Anchor {
    mode: i64,
    station: i64,
    day_unix: i64,
    name: &'static str,
}

fn anchors() -> Vec<Anchor> {
    vec![
        Anchor { mode: 1, station: 14, day_unix: days_from_civil(1995, 11, 24).unwrap(), name: "M1 st14 1995-11-24 (25.85 Hz)" },
        Anchor { mode: 2, station: 14, day_unix: days_from_civil(1995, 11, 24).unwrap(), name: "M2 st14 1995-11-24 (31.77 Hz)" },
        Anchor { mode: 1, station: 63, day_unix: days_from_civil(1996, 6, 26).unwrap(), name: "M1 st63 1996-06-26 (20.64 Hz)" },
        Anchor { mode: 3, station: 43, day_unix: days_from_civil(1995, 12, 4).unwrap(), name: "M3 st43 1995-12-04 (10.52 Hz)" },
        Anchor { mode: 3, station: 14, day_unix: days_from_civil(1995, 12, 5).unwrap(), name: "M3 st14 1995-12-05 (52.92 Hz)" },
        Anchor { mode: 3, station: 63, day_unix: days_from_civil(1995, 11, 27).unwrap(), name: "M3 st63 1995-11-27 (186.5 Hz)" },
        Anchor { mode: 1, station: 43, day_unix: days_from_civil(1996, 11, 4).unwrap(), name: "M1 st43 1996-11-04 (23.1 Hz)" },
    ]
}

fn civil(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn fmt_utc(tdb: f64) -> String {
    let unix = tdb + 10957.5 * DAY_S;
    let rem = unix.rem_euclid(DAY_S);
    let h = (rem / 3600.0) as i64;
    let m = ((rem % 3600.0) / 60.0) as i64;
    let s = (rem % 60.0) as i64;
    format!("{h:02}:{m:02}:{s:02}")
}

fn unix_day(tdb: f64) -> i64 {
    let jd = 2451545.0 + tdb / DAY_S;
    (jd - 2440587.5).round() as i64
}

fn stats_of(v: &[(f64, f64)], lo: usize, hi: usize) -> (usize, f64, f64) {
    let n = hi - lo;
    if n == 0 {
        return (0, 0.0, 0.0);
    }
    let mut sum = 0.0;
    let mut sumsq = 0.0;
    for s in &v[lo..hi] {
        sum += s.1;
        sumsq += s.1 * s.1;
    }
    let mean = sum / n as f64;
    (n, mean, (sumsq / n as f64 - mean * mean).max(0.0).sqrt())
}

fn tdb_of(rec: &[u8], lsk: &LeapSeconds) -> Option<f64> {
    let year = full_year(extract(rec, field_of(TKFORM, 3).unwrap()));
    let day = extract(rec, field_of(TKFORM, 4).unwrap());
    if day <= 0 || day > 366 {
        return None;
    }
    let hour = extract(rec, field_of(TKFORM, 5).unwrap());
    let minute = extract(rec, field_of(TKFORM, 6).unwrap());
    let second = extract(rec, field_of(TKFORM, 7).unwrap());
    let days = days_from_civil(year, 1, 1)? + day - 1;
    let unix = days as f64 * DAY_S + hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64;
    lsk.unix_to_tdb(unix)
}

fn resid() -> Option<(Vec<Vec<(f64, f64, f64)>>, usize)> {
    let file = File::open("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin").ok()?;
    let mut reader = BufReader::new(file);
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic).ok()?;
    if &magic != b"GASR" {
        return None;
    }
    let mut count_buf = [0u8; 4];
    reader.read_exact(&mut count_buf).ok()?;
    let anc = anchors();
    let mut per: Vec<Vec<(f64, f64, f64)>> = vec![Vec::new(); anc.len()];
    let mut rec = [0u8; 64];
    let mut kept = 0usize;
    loop {
        if reader.read_exact(&mut rec).is_err() {
            break;
        }
        let mut r = [0.0f64; 8];
        for (k, slot) in r.iter_mut().enumerate() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&rec[k * 8..k * 8 + 8]);
            *slot = f64::from_le_bytes(buf);
        }
        let resid_v = r[1];
        if !resid_v.is_finite() {
            continue;
        }
        let station = r[2] as i64;
        let mode = r[3] as i64;
        let day = unix_day(r[0]);
        for (i, a) in anc.iter().enumerate() {
            if a.mode == mode && a.station == station && day == a.day_unix {
                per[i].push((r[0], resid_v, r[7]));
                kept += 1;
                break;
            }
        }
    }
    for v in per.iter_mut() {
        v.sort_by(|x, y| x.0.total_cmp(&y.0));
    }
    Some((per, kept))
}

struct Flagged {
    tdb: f64,
    resid: f64,
    strength: i64,
    good0: i64,
    tol0: i64,
    lock0: i64,
    xmtr_on0: i64,
    slipped: i64,
}

fn load_flagged(anchor: &Anchor) -> Vec<Flagged> {
    let Some(lsk) = embedded_lsk() else {
        return Vec::new();
    };
    let mut outv: Vec<Flagged> = Vec::new();
    for cache in CACHES {
        let Ok(bytes) = std::fs::read(cache) else {
            continue;
        };
        let Some(stripped) = strip_markers(&bytes) else {
            continue;
        };
        let nlog = stripped.len() / LOGICAL_RECORD;
        if nlog < 3 {
            continue;
        }
        for i in 2..nlog {
            let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
            let day = extract(rec, field_of(TKFORM, 4).unwrap());
            if day == 0 {
                continue;
            }
            let dtype = extract(rec, field_of(TKFORM, 12).unwrap());
            if !(dtype == 1 || dtype == 2) {
                continue;
            }
            let station = extract(rec, field_of(TKFORM, 10).unwrap());
            let mode = extract(rec, field_of(TKFORM, 13).unwrap());
            if station != anchor.station || mode != anchor.mode {
                continue;
            }
            let Some(tdb) = tdb_of(rec, &lsk) else {
                continue;
            };
            if unix_day(tdb) != anchor.day_unix {
                continue;
            }
            let resid = extract(rec, field_of(TKFORM, 60).unwrap()) as f64 / 1000.0;
            if !resid.is_finite() {
                continue;
            }
            outv.push(Flagged {
                tdb,
                resid,
                strength: extract(rec, field_of(TKFORM, 78).unwrap()),
                good0: extract(rec, field_of(TKFORM, 17).unwrap()),
                tol0: extract(rec, field_of(TKFORM, 18).unwrap()),
                lock0: extract(rec, field_of(TKFORM, 25).unwrap()),
                xmtr_on0: extract(rec, field_of(TKFORM, 26).unwrap()),
                slipped: extract(rec, field_of(TKFORM, 76).unwrap()),
            });
        }
    }
    outv.sort_by(|x, y| x.tdb.total_cmp(&y.tdb));
    outv
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_cycle_slip_discipline_report.txt".to_string(),
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo loud-pass transient lock/cycle-boundary discipline (direction CS)".to_string());
    out.push("binding: floor sample = strength == -2560 AND |resid| <= 1000 Hz; |resid| > 1000 Hz samples are the lock-cut markers (excluded from the floor set)".to_string());
    out.push("loud run = maximal floor-sample run (gap <= 600 s) with n >= 30 and run RMS about the run mean >= 1 Hz".to_string());
    out.push("transient event = cluster of floor samples with |resid| >= 100 Hz, merged when the gap between consecutive event samples <= 60 s".to_string());
    out.push("lock-adjacent event = event whose span is within 60 s of a |resid| > 1000 Hz sample or of a run edge (lock-loss / reacquire boundary)".to_string());
    out.push("flag crossing reads the raw TDF caches of the anchor day: DOPPLER_GOOD0 (17), DOPPLER_TOL0 (18), RCVR_LOCK0 (25), XMTR_ON0 (26), SLIPPED_CYCLE (76)".to_string());

    let Some((per, kept)) = resid() else {
        println!("resid parse void");
        return;
    };
    out.push(format!("resid.bin records in the seven (mode, station, day) day cells: {kept}"));

    let anc = anchors();
    for (i, a) in anc.iter().enumerate() {
        out.push(format!("\nANCHOR {} mode {} st{} ({})", a.name, a.mode, a.station, civil(a.day_unix)));
        let v = &per[i];
        if v.is_empty() {
            out.push("  no (mode, station, day) samples in resid.bin (0 honored)".to_string());
            continue;
        }
        let floor: Vec<(f64, f64)> = v
            .iter()
            .filter(|(_, r, s)| *s == FLOOR_AGC as f64 && r.abs() <= LOCK_HZ)
            .map(|(t, r, _)| (*t, *r))
            .collect();
        let (n_day, m_day, rms_day) = stats_of(&floor, 0, floor.len());
        out.push(format!(
            "  floor day cell: n {n_day} mean {m_day:.3} Hz RMS {rms_day:.4} Hz (reference reproduction)"
        ));
        if n_day < MIN_CELL {
            out.push("  day n < MIN_CELL (0 honored)".to_string());
            continue;
        }
        let run_segs: Vec<(usize, usize)> = {
            let mut segs: Vec<(usize, usize)> = Vec::new();
            let mut s = 0usize;
            for k in 1..floor.len() {
                if floor[k].0 - floor[k - 1].0 > RUN_GAP_S {
                    segs.push((s, k));
                    s = k;
                }
            }
            segs.push((s, floor.len()));
            segs
        };
        let loud_runs: Vec<(usize, usize)> = run_segs
            .iter()
            .filter(|(lo, hi)| hi - lo >= MIN_CELL && stats_of(&floor, *lo, *hi).2 >= LOUD_HZ)
            .copied()
            .collect();
        if loud_runs.is_empty() {
            out.push("  no loud floor run (n >= MIN_CELL and run RMS >= 1 Hz) in the day cell (0 honored)".to_string());
            continue;
        }
        for (lo, hi) in &loud_runs {
            let (n_r, m_r, rms_r) = stats_of(&floor, *lo, *hi);
            let t0 = floor[*lo].0;
            let t1 = floor[*hi - 1].0;
            out.push(format!(
                "  loud run [{} .. {} UTC] n {n_r} mean {m_r:.3} Hz run RMS {rms_r:.4} Hz",
                fmt_utc(t0),
                fmt_utc(t1)
            ));
            let marker_total = v.iter().filter(|(_, r, _)| r.abs() > LOCK_HZ).count();
            let markers_near: Vec<(f64, f64)> = v
                .iter()
                .filter(|(t, r, _)| r.abs() > LOCK_HZ && *t >= t0 - ADJ_S && *t <= t1 + ADJ_S)
                .map(|(t, r, _)| (*t, *r))
                .collect();
            let markers_before = markers_near.iter().filter(|(t, _)| *t < t0).count();
            let markers_after = markers_near.iter().filter(|(t, _)| *t > t1).count();
            out.push(format!(
                "    lock-cut markers |resid| > 1000 Hz in the day cell: {marker_total}; within {ADJ_S:.0} s of the run: {} ({markers_before} before run, {} inside, {markers_after} after)",
                markers_near.len(),
                markers_near.len() - markers_before - markers_after
            ));
            let trans_idx: Vec<usize> = (*lo..*hi).filter(|k| floor[*k].1.abs() >= TRANS_HZ).collect();
            let mut events: Vec<(usize, usize)> = Vec::new();
            if !trans_idx.is_empty() {
                let mut s = 0usize;
                for k in 1..trans_idx.len() {
                    if floor[trans_idx[k]].0 - floor[trans_idx[k - 1]].0 > MERGE_S {
                        events.push((trans_idx[s], trans_idx[k - 1]));
                        s = k;
                    }
                }
                events.push((trans_idx[s], trans_idx[trans_idx.len() - 1]));
            }
            out.push(format!(
                "    transient samples (|resid| >= {TRANS_HZ:.0} Hz in run): {} | merged transient events: {}",
                trans_idx.len(),
                events.len()
            ));
            let flagged_all = load_flagged(a);
            if flagged_all.is_empty() {
                out.push("    raw-cache flag census for the anchor day: no flagged doppler records (0 honored, resid-alone measure applies)".to_string());
            } else {
                let unique = |f: &dyn Fn(&Flagged) -> i64| -> Vec<(i64, usize)> {
                    let mut m: BTreeMap<i64, usize> = BTreeMap::new();
                    for x in &flagged_all {
                        *m.entry(f(x)).or_insert(0) += 1;
                    }
                    let mut v: Vec<(i64, usize)> = m.into_iter().collect();
                    v.sort();
                    v
                };
                let lock_hist = unique(&|x: &Flagged| x.lock0);
                let good_hist = unique(&|x: &Flagged| x.good0);
                let tol_hist = unique(&|x: &Flagged| x.tol0);
                let slip_hist = unique(&|x: &Flagged| x.slipped);
                out.push(format!(
                    "    raw-cache flag census (n {}): RCVR_LOCK0 {lock_hist:?} | DOPPLER_GOOD0 {good_hist:?} | DOPPLER_TOL0 {tol_hist:?} | SLIPPED_CYCLE {slip_hist:?}",
                    flagged_all.len()
                ));
                let small: Vec<&Flagged> = flagged_all.iter().filter(|x| x.resid.abs() <= 100.0).collect();
                let big: Vec<&Flagged> = flagged_all.iter().filter(|x| x.resid.abs() > 100.0 && x.resid.abs() <= LOCK_HZ).collect();
                let huge: Vec<&Flagged> = flagged_all.iter().filter(|x| x.resid.abs() > LOCK_HZ).collect();
                let sm_lock = small.iter().filter(|x| x.lock0 != 0).count();
                let bg_lock = big.iter().filter(|x| x.lock0 != 0).count();
                let hu_lock = huge.iter().filter(|x| x.lock0 != 0).count();
                let sm_good = small.iter().filter(|x| x.good0 != 0).count();
                let bg_good = big.iter().filter(|x| x.good0 != 0).count();
                let hu_good = huge.iter().filter(|x| x.good0 != 0).count();
                let sm_slip = small.iter().filter(|x| x.slipped != 0).count();
                let bg_slip = big.iter().filter(|x| x.slipped != 0).count();
                let hu_slip = huge.iter().filter(|x| x.slipped != 0).count();
                out.push(format!(
                    "    lock0!=0 by |resid| class: <=100 Hz {} of {}, 100..1000 Hz {} of {}, >1000 Hz {} of {} | good0!=0: {} of {}, {} of {}, {} of {} | slipped!=0: {} of {}, {} of {}, {} of {}",
                    sm_lock, small.len(), bg_lock, big.len(), hu_lock, huge.len(),
                    sm_good, small.len(), bg_good, big.len(), hu_good, huge.len(),
                    sm_slip, small.len(), bg_slip, big.len(), hu_slip, huge.len()
                ));
            }
            let flagged = flagged_all;
            let flag_cover: Vec<&Flagged> = flagged
                .iter()
                .filter(|f| f.tdb >= t0 - ADJ_S && f.tdb <= t1 + ADJ_S)
                .collect();
            out.push(format!(
                "    raw-cache flagged doppler records within {ADJ_S:.0} s of the run: {} (resid day-cell floor n {n_day} for coverage scale)",
                flag_cover.len()
            ));
            let run_lock = flag_cover.iter().filter(|f| f.lock0 != 0).count();
            let run_good = flag_cover.iter().filter(|f| f.good0 != 0).count();
            let run_slip = flag_cover.iter().filter(|f| f.slipped != 0).count();
            let floor_flag: Vec<&Flagged> = flag_cover
                .iter()
                .copied()
                .filter(|f| f.resid.abs() <= LOCK_HZ && f.strength == FLOOR_AGC)
                .collect();
            let floor_flag_lock = floor_flag.iter().filter(|f| f.lock0 != 0).count();
            out.push(format!(
                "    run-window flag context: lock-out {run_lock}, good-bad {run_good}, slipped {run_slip} of {} | AGC-floor flagged records within the run: {} (of which lock-out {floor_flag_lock})",
                flag_cover.len(),
                floor_flag.len()
            ));
            if events.is_empty() {
                out.push("    no transient events above the 100 Hz threshold (0 honored)".to_string());
            }
            for (e_lo, e_hi) in &events {
                let ev0 = floor[*e_lo].0;
                let ev1 = floor[*e_hi].0;
                let n_ev = e_hi - e_lo + 1;
                let mut max_abs = 0.0f64;
                for k in *e_lo..=*e_hi {
                    max_abs = max_abs.max(floor[k].1.abs());
                }
                let d_mark = markers_near
                    .iter()
                    .map(|(t, _)| (t - (ev0 + ev1) * 0.5).abs() - (ev1 - ev0) * 0.5)
                    .fold(f64::INFINITY, f64::min)
                    .max(0.0);
                let edge_adj = (ev0 - t0).abs() <= ADJ_S || (t1 - ev1).abs() <= ADJ_S;
                let lock_adj = d_mark <= ADJ_S || edge_adj;
                let flag_ev: Vec<&Flagged> = flag_cover
                    .iter()
                    .copied()
                    .filter(|f| f.tdb >= ev0 - 2.0 && f.tdb <= ev1 + 2.0)
                    .collect();
                let flag_ctx: Vec<&Flagged> = flag_cover
                    .iter()
                    .copied()
                    .filter(|f| f.tdb >= ev0 - ADJ_S && f.tdb <= ev1 + ADJ_S)
                    .collect();
                let n_lock_bad = flag_ev.iter().filter(|f| f.lock0 != 0).count();
                let n_good_bad = flag_ev.iter().filter(|f| f.good0 != 0).count();
                let n_tol_out = flag_ev.iter().filter(|f| f.tol0 != 0).count();
                let n_slipped = flag_ev.iter().filter(|f| f.slipped != 0).count();
                let flag_any = n_lock_bad + n_good_bad + n_tol_out + n_slipped;
                let resid_min = flag_ev.iter().map(|f| f.resid.abs()).fold(f64::INFINITY, f64::min);
                let resid_max = flag_ev.iter().map(|f| f.resid.abs()).fold(f64::NEG_INFINITY, f64::max);
                let xmtr_off = flag_ev.iter().filter(|f| f.xmtr_on0 != 0).count();
                let xmtr_on = flag_ev.iter().filter(|f| f.xmtr_on0 == 0).count();
                let strength_min = flag_ev.iter().map(|f| f.strength).min();
                let strength_max = flag_ev.iter().map(|f| f.strength).max();
                let ctx_lock = flag_ctx.iter().filter(|f| f.lock0 != 0).count();
                let ctx_good = flag_ctx.iter().filter(|f| f.good0 != 0).count();
                let ctx_slip = flag_ctx.iter().filter(|f| f.slipped != 0).count();
                let class = if flag_cover.is_empty() {
                    if lock_adj {
                        "lock-adjacent (resid alone, raw-cache coverage void)"
                    } else {
                        "mid-locked-run (resid alone, raw-cache coverage void)"
                    }
                } else if flag_any > 0 {
                    "receive-cycle-slip/lock-flag present"
                } else if lock_adj {
                    "lock-adjacent, flags nominal"
                } else {
                    "mid-locked-run, flags nominal (predict/ramp scale)"
                };
                let mut strength_txt = String::new();
                if let (Some(smin), Some(smax)) = (strength_min, strength_max) {
                    strength_txt = format!("strength {smin}..{smax}");
                }
                let mut resid_txt = String::new();
                if !flag_ev.is_empty() {
                    resid_txt = format!("flagged |resid| {resid_min:.1}..{resid_max:.1} Hz");
                }
                out.push(format!(
                    "      event [{} .. {} UTC] n {n_ev} max|resid| {max_abs:.1} Hz | lock-marker distance {d_mark:.0} s, run-start {:.0} s, run-end {:.0} s | tight +-2 s flags {} records (lock-out {n_lock_bad}, good-bad {n_good_bad}, tol-out {n_tol_out}, slipped {n_slipped}; {resid_txt}; {strength_txt}; xmtr off {xmtr_off} on {xmtr_on}) | +-60 s context {ctx_lock} lock-out, {ctx_good} good-bad, {ctx_slip} slipped -> {class}",
                    fmt_utc(ev0),
                    fmt_utc(ev1),
                    (ev0 - t0).abs(),
                    (t1 - ev1).abs(),
                    flag_ev.len()
                ));
            }
        }
    }

    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
