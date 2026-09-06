use std::collections::BTreeMap;
use std::fs;

use omegaflow::atdf::parse_resid_bin;
use omegaflow::odf::parse_p11r_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const MIN_CELL: usize = 30;
const ERA0_C: (i64, i64, i64) = (1995, 11, 23);
const ERA1_C: (i64, i64, i64) = (1997, 2, 28);
const TRIO: [i64; 3] = [14, 43, 63];
const SEG_GAP_S: f64 = 300.0;
const W0_C: (i64, i64, i64) = (1995, 11, 26);
const W1_C: (i64, i64, i64) = (1995, 12, 6);
const B0_C: (i64, i64, i64) = (1995, 11, 27);
const B1_C: (i64, i64, i64) = (1995, 12, 3);

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn day_of_tdb(tdb: f64) -> i64 {
    (2451545.0 + tdb / DAY_S - 2440587.5).round() as i64
}

fn civil_str(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day {day}"),
    }
}

fn utc_day_tod(tdb: f64) -> (i64, f64) {
    let jd = 2451545.0 + tdb / DAY_S;
    let du = jd - 2440587.5;
    let day = du.floor() as i64;
    let tod = (du - day as f64) * DAY_S;
    (day, tod)
}

fn dt_str(tdb: f64) -> String {
    let (day, tod) = utc_day_tod(tdb);
    let hh = (tod / 3600.0).floor().max(0.0) as i64;
    let mm = ((tod - hh as f64 * 3600.0) / 60.0).floor().max(0.0) as i64;
    format!("{} {:02}:{:02} UTC", civil_str(day), hh, mm)
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn rms_vals(vals: &[f64], m: f64) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    let v = vals.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / vals.len() as f64;
    v.sqrt()
}

fn fmt_opt(v: Option<f64>, w: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:+.w$}", w = w),
        Some(x) => format!("{x}"),
        None => "-".repeat(w + 1),
    }
}

struct CellAgg {
    n: usize,
    sum: f64,
    sum2: f64,
    tdbs: Vec<f64>,
    vals: Vec<f64>,
    refs: Vec<f64>,
}

impl CellAgg {
    fn new() -> CellAgg {
        CellAgg {
            n: 0,
            sum: 0.0,
            sum2: 0.0,
            tdbs: Vec::new(),
            vals: Vec::new(),
            refs: Vec::new(),
        }
    }
    fn push(&mut self, tdb: f64, resid: f64, ref_hz: f64) {
        self.n += 1;
        self.sum += resid;
        self.sum2 += resid * resid;
        self.tdbs.push(tdb);
        self.vals.push(resid);
        self.refs.push(ref_hz);
    }
}

struct Row {
    day: i64,
    n: usize,
    mean: f64,
    med: f64,
    rms: f64,
    ref_med: f64,
    ref_lo: f64,
    ref_hi: f64,
}

struct Seg {
    t0: f64,
    t1: f64,
    vals: Vec<f64>,
}

fn rec(out: &mut Vec<String>, s: String) {
    out.push(s);
}

fn push_seg(out: &mut Vec<String>, st: i64, seg: &Seg, tag: &str) {
    let med = median(&seg.vals);
    let m = seg.vals.iter().sum::<f64>() / seg.vals.len() as f64;
    let n = seg.vals.len();
    rec(
        out,
        format!(
            "  st{st} {tag} {} -> {} ({} min, n {}) med {:+.4} mean {:+.4} rms {:.4}",
            dt_str(seg.t0),
            dt_str(seg.t1),
            ((seg.t1 - seg.t0) / 60.0).round() as i64,
            n,
            med.map_or(f64::NAN, |v| v),
            m,
            rms_vals(&seg.vals, m)
        ),
    );
}

fn main() {
    let mut report = "/tmp/opencode/galileo_ruck_zeugen_report.txt".to_string();
    for a in std::env::args().skip(1) {
        if let Some(r) = a.strip_prefix("--report=") {
            report = r.to_string();
        }
    }
    let path = "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin";
    let Ok(bytes) = fs::read(path) else {
        eprintln!("{path}: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("{path}: resid bin parse void");
        return;
    };
    drop(bytes);

    let era0 = days_from_civil(ERA0_C.0, ERA0_C.1, ERA0_C.2);
    let era1 = days_from_civil(ERA1_C.0, ERA1_C.1, ERA1_C.2);
    let w0 = days_from_civil(W0_C.0, W0_C.1, W0_C.2);
    let w1 = days_from_civil(W1_C.0, W1_C.1, W1_C.2);
    let b0 = days_from_civil(B0_C.0, B0_C.1, B0_C.2);
    let b1 = days_from_civil(B1_C.0, B1_C.1, B1_C.2);

    let mut cells: BTreeMap<(i64, i64), CellAgg> = BTreeMap::new();
    let mut win: BTreeMap<(i64, i64), CellAgg> = BTreeMap::new();
    let mut n_rec = 0usize;
    let mut n_lock = 0usize;
    let mut n_floor_m1_trio = 0usize;
    for r in &recs {
        n_rec += 1;
        let mode = r[3] as i64;
        let st = r[2] as i64;
        if mode != 1 || !TRIO.contains(&st) {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() {
            continue;
        }
        if resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        if r[7] as i64 != FLOOR {
            continue;
        }
        let day = day_of_tdb(r[0]);
        let ref_hz = r[5];
        n_floor_m1_trio += 1;
        if day >= era0 && day <= era1 {
            cells.entry((day, st)).or_insert_with(CellAgg::new).push(r[0], resid, ref_hz);
        }
        if day >= w0 && day <= w1 {
            win.entry((day, st)).or_insert_with(CellAgg::new).push(r[0], resid, ref_hz);
        }
    }
    drop(recs);

    let mut out: Vec<String> = Vec::new();
    rec(&mut out, format!("galileo ruck zeugen probe — {path}"));
    rec(
        &mut out,
        format!(
            "question: where does the mode-1 quiet-basis ruck (1995-11-30/12-01, resid-level -0.80..-0.82 Hz) sit — in the received/reference track or in the resid (model) — and which witnesses (pioneer, sub-day structure) carry it"
        ),
    );
    rec(
        &mut out,
        format!(
            "resid bin: {n_rec} records; mode-1 trio-station floor samples (strength {FLOOR}, |resid| <= {LOCK_HZ:.0} Hz) over the era: {n_floor_m1_trio}; lock-excluded mode-1 trio samples {n_lock}"
        ),
    );
    rec(
        &mut out,
        format!(
            "GASR field 5 (doppler reference frequency, Hz) is read per sample and its daily median/spread printed next to the resid daily level; skyfreq GASF coverage is measured separately (see below)"
        ),
    );

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== A: mode-1 quiet robust day cells over the era — resid level and reference-frequency level ==".to_string(),
    );
    for st in TRIO {
        let mut rows: Vec<Row> = Vec::new();
        for ((day, s), c) in &cells {
            if *s != st {
                continue;
            }
            let mval = c.sum / c.n as f64;
            let v = (c.sum2 / c.n as f64 - mval * mval).max(0.0);
            let rms = v.sqrt();
            let med = median(&c.vals);
            let rmed = median(&c.refs);
            let rmin = c.refs.iter().cloned().fold(f64::INFINITY, f64::min);
            let rmax = c.refs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            if c.n >= MIN_CELL && rms < LOUD_HZ {
                rows.push(Row {
                    day: *day,
                    n: c.n,
                    mean: mval,
                    med: med.expect("median present"),
                    rms,
                    ref_med: rmed.expect("ref median present"),
                    ref_lo: rmin,
                    ref_hi: rmax,
                });
            }
        }
        rows.sort_by_key(|r| r.day);
        let quiet = rows.len();
        rec(&mut out, String::new());
        rec(
            &mut out,
            format!("M1 st{st}: {quiet} quiet robust days (n >= {MIN_CELL}, rms < {LOUD_HZ} Hz) — date | n | resid-med | resid-mean | resid-rms | ref-med Hz | ref min..max Hz"),
        );
        for r in &rows {
            rec(
                &mut out,
                format!(
                    "{:10} | n {:<7} | med {} | mean {} | rms {:.4} | ref {:.0} | {:.0}..{:.0}",
                    civil_str(r.day),
                    r.n,
                    fmt_opt(Some(r.med), 3),
                    fmt_opt(Some(r.mean), 3),
                    r.rms,
                    r.ref_med,
                    r.ref_lo,
                    r.ref_hi
                ),
            );
        }
        let mut resid_adj: Vec<(f64, i64, i64)> = Vec::new();
        let mut ref_adj: Vec<(f64, i64, i64)> = Vec::new();
        for i in 0..rows.len().saturating_sub(1) {
            let a = &rows[i];
            let c = &rows[i + 1];
            if c.day - a.day > 3 {
                continue;
            }
            resid_adj.push(((a.med - c.med).abs(), a.day, c.day));
            ref_adj.push(((a.ref_med - c.ref_med).abs(), a.day, c.day));
        }
        if !resid_adj.is_empty() {
            let mut rd: Vec<f64> = resid_adj.iter().map(|x| x.0).collect();
            rd.sort_by(f64::total_cmp);
            let mut rf: Vec<f64> = ref_adj.iter().map(|x| x.0).collect();
            rf.sort_by(f64::total_cmp);
            let mxr = resid_adj.iter().max_by(|a, b| a.0.total_cmp(&b.0)).expect("non-empty");
            let mxf = ref_adj.iter().max_by(|a, b| a.0.total_cmp(&b.0)).expect("non-empty");
            rec(
                &mut out,
                format!(
                    "  adjacent quiet-day steps (<=3 d apart): n {} | resid|med| median {:.4} max {:.4} Hz ({} -> {}) | ref|med| median {:.0} max {:.0} Hz ({} -> {})",
                    rd.len(),
                    rd[rd.len() / 2],
                    mxr.0,
                    civil_str(mxr.1),
                    civil_str(mxr.2),
                    rf[rf.len() / 2],
                    mxf.0,
                    civil_str(mxf.1),
                    civil_str(mxf.2)
                ),
            );
        }
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== B: the boundary cells 1995-11-27..12-03 — resid level vs reference level on the adjacent quiet days ==".to_string(),
    );
    for st in TRIO {
        rec(&mut out, format!("  --- M1 st{st} ---"));
        for ((day, s), c) in &cells {
            if *s != st || *day < b0 || *day > b1 {
                continue;
            }
            let mval = c.sum / c.n as f64;
            let v = (c.sum2 / c.n as f64 - mval * mval).max(0.0);
            let rms = v.sqrt();
            let med = median(&c.vals);
            let rmed = median(&c.refs);
            let rmin = c.refs.iter().cloned().fold(f64::INFINITY, f64::min);
            let rmax = c.refs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let cls = if c.n < MIN_CELL {
                format!("thin n {}", c.n)
            } else if rms >= LOUD_HZ {
                format!("loud rms {rms:.3}")
            } else {
                format!("quiet")
            };
            rec(
                &mut out,
                format!(
                    "  {} st{st} n {:<7} resid-med {} resid-mean {} resid-rms {:.4} | ref-med {:.0} (span {:.0}) | {cls}",
                    civil_str(*day),
                    c.n,
                    fmt_opt(med, 3),
                    fmt_opt(Some(mval), 3),
                    rms,
                    rmed.expect("ref median present"),
                    rmax - rmin,
                ),
            );
        }
    }


    rec(
        &mut out,
        "  ruck boundary pair (last plateau quiet cell -> first low quiet cell), resid and ref deltas:".to_string(),
    );
    let pair_days: [(i64, (i64, i64, i64), (i64, i64, i64)); 3] = [
        (14, (1995, 11, 30), (1995, 12, 1)),
        (43, (1995, 11, 30), (1995, 12, 2)),
        (63, (1995, 11, 29), (1995, 12, 1)),
    ];
    for (st, hi_c, lo_c) in pair_days {
        let hi = days_from_civil(hi_c.0, hi_c.1, hi_c.2);
        let lo = days_from_civil(lo_c.0, lo_c.1, lo_c.2);
        let (m_hi, r_hi, m_lo, r_lo) = match (cells.get(&(hi, st)), cells.get(&(lo, st))) {
            (Some(a), Some(b)) => {
                let med = |c: &CellAgg| median(&c.vals).expect("median present");
                let rmed = |c: &CellAgg| median(&c.refs).expect("ref median present");
                (med(a), rmed(a), med(b), rmed(b))
            }
            _ => continue,
        };
        let resid_step = m_lo - m_hi;
        let ref_step = r_lo - r_hi;
        rec(
            &mut out,
            format!(
                "  M1 st{st} {} (resid {m_hi:+.3} Hz, ref {r_hi:.0}) -> {} (resid {m_lo:+.3} Hz, ref {r_lo:.0}) | resid step {resid_step:+.3} Hz | ref step {ref_step:+.0} Hz",
                civil_str(hi),
                civil_str(lo),
            ),
        );
    }

    rec(&mut out, String::new());
    rec(&mut out, String::new());
    rec(
        &mut out,
        "== C: sub-day structure across the transition (mode-1 floor samples, in-track segments gap > 300 s) ==".to_string(),
    );
    for st in TRIO {
        rec(&mut out, format!("  --- M1 st{st} segments {} .. {} ---", civil_str(b0), civil_str(b1)));
        let mut seg: Option<Seg> = None;
        let flush = |out: &mut Vec<String>, st: i64, s: &mut Option<Seg>, tag: &str| {
            if let Some(sg) = s.take() {
                push_seg(out, st, &sg, tag);
            }
        };
        for ((day, s), c) in &win {
            if *s != st || *day < b0 || *day > b1 {
                continue;
            }
            let mut idx: Vec<usize> = (0..c.n).collect();
            idx.sort_by(|a, b| c.tdbs[*a].total_cmp(&c.tdbs[*b]));
            for &i in &idx {
                let t = c.tdbs[i];
                match seg.as_mut() {
                    Some(sg) if t - sg.t1 <= SEG_GAP_S => {
                        sg.t1 = t;
                        sg.vals.push(c.vals[i]);
                    }
                    _ => {
                        flush(&mut out, st, &mut seg, "seg");
                        seg = Some(Seg {
                            t0: t,
                            t1: t,
                            vals: vec![c.vals[i]],
                        });
                    }
                }
            }
            flush(&mut out, st, &mut seg, "seg");
        }
        flush(&mut out, st, &mut seg, "seg");
    }
    rec(
        &mut out,
        "  per-day-segment above is a rough pass view; hourly medians for the two boundary days follow".to_string(),
    );
    for st in TRIO {
        rec(&mut out, format!("  --- M1 st{st} hourly median resid, true-UTC 1995-11-30 and 1995-12-01 ---"));
        for dd in [days_from_civil(1995, 11, 30), days_from_civil(1995, 12, 1)] {
            let mut hrs: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
            for ((_, s), c) in &win {
                if *s != st {
                    continue;
                }
                for (i, &t) in c.tdbs.iter().enumerate() {
                    let (ud, tod) = utc_day_tod(t);
                    if ud == dd {
                        hrs.entry((tod / 3600.0) as i64).or_default().push(c.vals[i]);
                    }
                }
            }
            let mut line = format!("  {}:", civil_str(dd));
            for (h, v) in &hrs {
                if v.len() >= 15 {
                    line.push_str(&format!(" h{h:02} n{} {:+.3}", v.len(), median(v).expect("med")));
                } else {
                    line.push_str(&format!(" h{h:02} n{}", v.len()));
                }
            }
            rec(&mut out, line);
        }
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== D: skyfreq (GASF) coverage of the transition window (read with the 14-field GASF parse) ==".to_string(),
    );
    if let Ok(sb) = fs::read("data/pds-ppi.igpp.ucla.edu/galileo_skyfreq.bin") {
        if sb.len() >= 8 && &sb[0..4] == b"GASF" {
            let cnt = u32::from_le_bytes(sb[4..8].try_into().ok().expect("len")) as usize;
            if sb.len() == 8 + cnt * 112 {
                let mut daymin = i64::MAX;
                let mut daymax = i64::MIN;
                let mut stset: BTreeMap<i64, usize> = BTreeMap::new();
                for i in 0..cnt {
                    let base = 8 + i * 112;
                    let mut buf = [0u8; 8];
                    buf.copy_from_slice(&sb[base..base + 8]);
                    let t = f64::from_le_bytes(buf);
                    let d = day_of_tdb(t);
                    daymin = daymin.min(d);
                    daymax = daymax.max(d);
                    let mut sb2 = [0u8; 8];
                    sb2.copy_from_slice(&sb[base + 6 * 8..base + 7 * 8]);
                    let stv = f64::from_le_bytes(sb2) as i64;
                    *stset.entry(stv).or_default() += 1;
                }
                rec(
                    &mut out,
                    format!(
                        "galileo_skyfreq.bin: {cnt} GASF samples, span {} .. {}, stations {:?}",
                        civil_str(daymin),
                        civil_str(daymax),
                        stset
                    ),
                );
                if daymin <= days_from_civil(1995, 12, 1) && daymax >= days_from_civil(1995, 11, 30) {
                    rec(&mut out, "  the transition days carry skyfreq samples".to_string());
                } else {
                    rec(
                        &mut out,
                        "  the transition days 1995-11-30/12-01 carry no skyfreq samples (0 honored)".to_string(),
                    );
                }
            } else {
                rec(&mut out, "galileo_skyfreq.bin: GASF length mismatch (0 honored)".to_string());
            }
        } else {
            rec(&mut out, "galileo_skyfreq.bin: no GASF magic (0 honored)".to_string());
        }
    } else {
        rec(&mut out, "galileo_skyfreq.bin: void (0 honored)".to_string());
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== E: pioneer residuum coverage of the boundary window (per day and station) ==".to_string(),
    );
    let pwin0 = days_from_civil(1995, 11, 15);
    let pwin1 = days_from_civil(1995, 12, 20);
    for (name, ppath) in [
        ("pioneer10", "data/spdf.gsfc.nasa.gov/pioneer10_navio_residuum.bin"),
        ("pioneer11", "data/spdf.gsfc.nasa.gov/pioneer11_navio_residuum.bin"),
    ] {
        rec(&mut out, format!("  --- {name} ---"));
        let Ok(pb) = fs::read(ppath) else {
            rec(&mut out, format!("  {ppath}: void (0 honored)"));
            continue;
        };
        let Some(precs) = parse_p11r_bin(&pb) else {
            rec(&mut out, format!("  {ppath}: parse void (0 honored)"));
            continue;
        };
        let mut pday: BTreeMap<i64, BTreeMap<i64, Vec<f64>>> = BTreeMap::new();
        let mut stmode: BTreeMap<(i64, i64, i64), usize> = BTreeMap::new();
        for p in &precs {
            let day = day_of_tdb(p[0]);
            if day < pwin0 || day > pwin1 {
                continue;
            }
            let rx = p[5] as i64;
            if !TRIO.contains(&rx) {
                continue;
            }
            let resid = p[1];
            if !resid.is_finite() {
                continue;
            }
            pday.entry(day).or_default().entry(rx).or_default().push(resid);
            let mode = p[7] as i64;
            *stmode.entry((day, rx, mode)).or_default() += 1;
        }
        let mut rowc = 0usize;
        for (day, m) in &pday {
            let mut cells: Vec<String> = Vec::new();
            for (rx, v) in m {
                let med = median(v).expect("median present");
                let rm = rms_vals(v, v.iter().sum::<f64>() / v.len() as f64);
                let modes: Vec<String> = stmode
                    .iter()
                    .filter(|((d, r, _), _)| *d == *day && *r == *rx)
                    .map(|((_, _, mo), n)| format!("m{mo} n{n}"))
                    .collect();
                cells.push(format!(
                    "st{rx} n{:<3} resid-med {med:+.3} rms {rm:.3} [{}]",
                    v.len(),
                    modes.join("+")
                ));
            }
            rec(
                &mut out,
                format!("  {name} {}: {}",
                    civil_str(*day),
                    cells.join(" | ")
                ),
            );
            rowc += 1;
        }
        if rowc == 0 {
            rec(&mut out, format!("  {name}: no trio-station samples in window (0 honored)"));
        }
        let nd30 = pday.get(&days_from_civil(1995, 11, 30)).map_or(0usize, |m| m.len());
        rec(
            &mut out,
            format!(
                "  {name}: 1995-11-30 trio cells: {nd30} (0 honored when absent); resid field is the per-epoch fixed-effects residual of the navio compiler (pass offsets removed) — a day-level step is removed with the offset, this axis carries no level witness"
            ),
        );
    }

    rec(&mut out, String::new());
    rec(
        &mut out,
        "== verdict numbers ==".to_string(),
    );
    for st in TRIO {
        let mut plat: Vec<f64> = Vec::new();
        let mut post: Vec<f64> = Vec::new();
        for ((day, s), c) in &cells {
            if *s != st || *day > days_from_civil(1995, 12, 31) {
                continue;
            }
            let mval = c.sum / c.n as f64;
            let rms = (c.sum2 / c.n as f64 - mval * mval).max(0.0).sqrt();
            if c.n < MIN_CELL || rms >= LOUD_HZ {
                continue;
            }
            if let Some(m) = median(&c.vals) {
                if *day <= days_from_civil(1995, 11, 30) {
                    plat.push(m);
                } else {
                    post.push(m);
                }
            }
        }
        let pm = median(&plat).map_or(f64::NAN, |v| v);
        let pom = median(&post).map_or(f64::NAN, |v| v);
        let diff = pom - pm;
        rec(
            &mut out,
            format!(
                "M1 st{st}: plateau (1995-11-23..30 quiet cells) n {} med {pm:+.3} Hz | early-post (1995-12 quiet cells) n {} med {pom:+.3} Hz | difference {diff:+.3} Hz",
                plat.len(),
                post.len(),
            ),
        );
    }

    let _ = fs::write(&report, out.join("\n") + "\n");
    println!("report written to {report}");
}
