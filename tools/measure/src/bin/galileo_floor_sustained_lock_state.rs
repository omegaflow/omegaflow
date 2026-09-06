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
const EP_TH_HZ: f64 = 10.0;
const EP_GAP_S: f64 = 30.0;
const SUSTAINED_S: f64 = 60.0;
const ADJ_S: f64 = 60.0;
const TIGHT_S: f64 = 2.0;

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
    lock0: i64,
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
                lock0: extract(rec, field_of(TKFORM, 25).unwrap()),
                slipped: extract(rec, field_of(TKFORM, 76).unwrap()),
            });
        }
    }
    outv.sort_by(|x, y| x.tdb.total_cmp(&y.tdb));
    outv
}

struct Episode {
    t0: f64,
    t1: f64,
    n: usize,
    peak: f64,
}

fn episodes_on(times: &[f64], resids: &[f64], gap_s: f64) -> Vec<Episode> {
    let mut out: Vec<Episode> = Vec::new();
    if times.is_empty() {
        return out;
    }
    let mut open: Option<(usize, usize, f64)> = None;
    for i in 0..times.len() {
        let loud = resids[i].abs() > EP_TH_HZ;
        if loud {
            match open {
                None => {
                    open = Some((i, i, resids[i].abs()));
                }
                Some((s, _, pk)) => {
                    open = Some((s, i, pk.max(resids[i].abs())));
                }
            }
        } else if let Some((s, e, pk)) = open {
            if times[i] - times[e] > gap_s {
                out.push(Episode { t0: times[s], t1: times[e], n: e - s + 1, peak: pk });
                open = None;
            }
        }
    }
    if let Some((s, e, pk)) = open {
        out.push(Episode { t0: times[s], t1: times[e], n: e - s + 1, peak: pk });
    }
    out
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_floor_sustained_lock_state_report.txt".to_string(),
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo loud-pass sustained-episode lock-state crossing (Klarstellung A)".to_string());
    out.push("binding: floor sample = strength == -2560 AND |resid| <= 1000 Hz (loud run = gap <= 600 s, n >= 30, run RMS about run mean >= 1 Hz)".to_string());
    out.push("episode = connected |resid| > 10 Hz floor samples of the loud run, merged across a gap <= 30 s (elevation canonical T 10 Hz / gap 30 s)".to_string());
    out.push("sustained episode = episode with span (last - first) >= 60 s (minutes class); transient = span < 60 s (the <2 s sub-class is reported separately)".to_string());
    out.push("flag crossing: raw-TDF caches of the anchor day, AGC-floor flagged records (strength == -2560, |resid| <= 1000): RCVR_LOCK0 (25) 0 = in-lock / 1 = out-of-lock, DOPPLER_GOOD0 (17) 0 = good, SLIPPED_CYCLE (76) > 0".to_string());
    out.push("tight window per episode = [t0 - 2 s, t1 + 2 s]; lock markers = |resid| > 1000 Hz records of the day within +-60 s of the episode".to_string());
    out.push("question: do the sustained (minutes-hours) loud episodes sit in out-of-lock-flagged stretches or in in-lock/good stretches with slips only at the edges?".to_string());

    let Some((per, kept)) = resid() else {
        println!("resid parse void");
        return;
    };
    out.push(format!("resid.bin records in the seven (mode, station, day) day cells: {kept}"));

    let anc = anchors();
    let mut sust_total = 0usize;
    let mut sust_out = 0usize;
    let mut sust_in = 0usize;
    let mut sust_void = 0usize;
    let mut sust_span_out = 0.0;
    let mut sust_span_in = 0.0;
    let mut sust_span_void = 0.0;
    let mut trans_total = 0usize;
    let mut short_total = 0usize;

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
        let flagged = load_flagged(a);
        let flag_present = !flagged.is_empty();
        if !flag_present {
            out.push("  raw-cache flag census for the anchor day: no flagged doppler records (0 honored, resid-alone measure applies)".to_string());
        }
        for (lo, hi) in &loud_runs {
            let (n_r, m_r, rms_r) = stats_of(&floor, *lo, *hi);
            let t0r = floor[*lo].0;
            let t1r = floor[*hi - 1].0;
            out.push(format!(
                "  loud run [{} .. {} UTC] n {n_r} mean {m_r:.3} Hz run RMS {rms_r:.4} Hz",
                fmt_utc(t0r),
                fmt_utc(t1r)
            ));
            let run_ts: Vec<f64> = floor[*lo..*hi].iter().map(|x| x.0).collect();
            let run_rs: Vec<f64> = floor[*lo..*hi].iter().map(|x| x.1).collect();
            let eps = episodes_on(&run_ts, &run_rs, EP_GAP_S);
            let single = eps.iter().filter(|e| e.n == 1).count();
            let short2 = eps.iter().filter(|e| e.t1 - e.t0 < 2.0).count();
            let sust: Vec<&Episode> = eps.iter().filter(|e| e.t1 - e.t0 >= SUSTAINED_S).collect();
            let sust_span: f64 = sust.iter().map(|e| e.t1 - e.t0).sum();
            out.push(format!(
                "  episodes (T {ep_th:.0} Hz, gap {ep_gap:.0} s): n {n_ep} (single-sample {n_single}, <2 s {n_short}, band transients {n_rest}) | sustained >= {sust_s:.0} s: n {n_sust} total span {span_sust:.0} s (loud-run {span_run:.0} s)",
                ep_th = EP_TH_HZ,
                ep_gap = EP_GAP_S,
                n_ep = eps.len(),
                n_single = single,
                n_short = short2,
                n_rest = eps.len() - short2,
                sust_s = SUSTAINED_S,
                n_sust = sust.len(),
                span_sust = sust_span,
                span_run = t1r - t0r
            ));
            trans_total += eps.len() - sust.len();
            short_total += short2;
            sust_total += sust.len();
            if !flag_present {
                sust_void += sust.len();
                sust_span_void += sust_span;
                out.push("  sustained episodes on this run carry no raw-cache flag census (resid-alone, flag class pending, 0 honored)".to_string());
                for e in &sust {
                    out.push(format!(
                        "    sustained episode [{} .. {} UTC] span {:.0} s n {} peak {:.1} Hz | no flags (0 honored)",
                        fmt_utc(e.t0),
                        fmt_utc(e.t1),
                        e.t1 - e.t0,
                        e.n,
                        e.peak
                    ));
                }
                continue;
            }
            let ff_run: Vec<&Flagged> = flagged
                .iter()
                .filter(|f| f.tdb >= t0r - TIGHT_S && f.tdb <= t1r + TIGHT_S)
                .filter(|f| f.strength == FLOOR_AGC && f.resid.abs() <= LOCK_HZ)
                .collect();
            let ff_out_run = ff_run.iter().filter(|f| f.lock0 != 0).count();
            out.push(format!(
                "  AGC-floor flagged records within the run (+-2 s): {} (lock-out {ff_out_run})",
                ff_run.len()
            ));
            let marker_near: Vec<&Flagged> = flagged
                .iter()
                .filter(|f| f.resid.abs() > LOCK_HZ && f.tdb >= t0r - ADJ_S && f.tdb <= t1r + ADJ_S)
                .collect();
            out.push(format!(
                "  lock markers |resid| > 1000 Hz within {adj:.0} s of the run: {n_mark}",
                adj = ADJ_S,
                n_mark = marker_near.len()
            ));
            for e in &eps {
                let span = e.t1 - e.t0;
                let sustained = span >= SUSTAINED_S;
                let win: Vec<&Flagged> = flagged
                    .iter()
                    .filter(|f| f.tdb >= e.t0 - TIGHT_S && f.tdb <= e.t1 + TIGHT_S)
                    .filter(|f| f.strength == FLOOR_AGC && f.resid.abs() <= LOCK_HZ)
                    .collect();
                let out_c = win.iter().filter(|f| f.lock0 != 0).count();
                let in_c = win.iter().filter(|f| f.lock0 == 0).count();
                let good_bad = win.iter().filter(|f| f.good0 != 0).count();
                let slip_c = win.iter().filter(|f| f.slipped > 0).count();
                let markers: Vec<&Flagged> = flagged
                    .iter()
                    .filter(|f| f.resid.abs() > LOCK_HZ && f.tdb >= e.t0 - ADJ_S && f.tdb <= e.t1 + ADJ_S)
                    .collect();
                let m_before = markers.iter().filter(|f| f.tdb < e.t0).count();
                let m_after = markers.iter().filter(|f| f.tdb > e.t1).count();
                let state = if win.is_empty() {
                    "flag-void".to_string()
                } else if out_c * 2 > win.len() {
                    "out-of-lock".to_string()
                } else if out_c == 0 {
                    "in-lock".to_string()
                } else {
                    "in-lock-lean".to_string()
                };
                if sustained {
                    match state.as_str() {
                        "out-of-lock" => {
                            sust_out += 1;
                            sust_span_out += span;
                        }
                        "flag-void" => {
                            sust_void += 1;
                            sust_span_void += span;
                        }
                        _ => {
                            sust_in += 1;
                            sust_span_in += span;
                        }
                    }
                }
                out.push(format!(
                    "  {kind} episode [{t0s} .. {t1s} UTC] span {span:.0} s n {n_ep} peak {peak:.1} Hz | tight-window AGC-floor flags {win_len} (out-of-lock {out_c}, in-lock {in_c}, good-bad {good_bad}, slipped {slip_c}) | lock markers +-{adj:.0} s: {m_total} ({m_before} before, {m_inside} inside, {m_after} after) -> {state}",
                    kind = if sustained { "SUSTAINED" } else { "transient" },
                    t0s = fmt_utc(e.t0),
                    t1s = fmt_utc(e.t1),
                    span = span,
                    n_ep = e.n,
                    peak = e.peak,
                    win_len = win.len(),
                    adj = ADJ_S,
                    m_total = markers.len(),
                    m_before = m_before,
                    m_after = m_after,
                    m_inside = markers.len() - m_before - m_after,
                    out_c = out_c,
                    in_c = in_c,
                    good_bad = good_bad,
                    slip_c = slip_c,
                    state = state
                ));
            }
        }
    }

    out.push("\n== sustained-episode verdict (all flag-covered loud runs) ==".to_string());
    out.push(format!(
        "sustained episodes (span >= {SUSTAINED_S:.0} s): n {sust_total} | out-of-lock-dominant {sust_out} (span {sust_span_out:.0} s), in-lock-dominant/in-lock-lean {sust_in} (span {sust_span_in:.0} s), flag-void (cache-free passes) {sust_void} (span {sust_span_void:.0} s)"
    ));
    out.push(format!(
        "transient episodes (span < {SUSTAINED_S:.0} s): n {trans_total} (of which < 2 s {short_total})"
    ));
    let verdict = if sust_out > 0 && sust_in == 0 && sust_out >= sust_total - sust_void {
        "sustained episodes of the flag-covered loud passes sit in out-of-lock-flagged stretches: sustained per-pass out-of-lock receive state, not locked-loop noise with edge slips alone"
    } else if sust_in > 0 && sust_out == 0 {
        "sustained episodes of the flag-covered loud passes sit in in-lock/good stretches: sustained received noise in a locked loop, slips at the edges"
    } else {
        "sustained episodes split between out-of-lock and in-lock stretches (mixed; per-pass state named in the episode table)"
    };
    out.push(format!("VERDICT (Klarstellung A): {verdict}"));
    out.push("0 honored: the two cache-free loud passes (M3 st63 1995-11-27, M1 st43 1996-11-04) carry no raw-cache flag census; their sustained episodes are counted flag-void, not classified.".to_string());

    let text = out.join("\n");
    println!("{text}");
    let _ = std::fs::write(&report_path, text);
}
