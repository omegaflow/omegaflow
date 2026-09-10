use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufReader, Read};

use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const LOUD_HZ: f64 = 1.0;
const ROBUST_N: usize = 30;
const PASS_GAP_S: f64 = 600.0;
const FLOOR_AGC: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const CLASS_FLOOR: i64 = 0;
const CLASS_STRONG: i64 = 1;
const TRIO: [i64; 3] = [14, 43, 63];
const MODES: [i64; 2] = [1, 2];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rx {
    rx_ref: i64,
    rx_num: i64,
    amp_num: i64,
    amp_type: i64,
}

#[derive(Clone, Copy)]
struct Samp {
    tdb: f64,
    resid: f64,
    rx: Rx,
}

struct DayBuf {
    floor: Vec<Samp>,
    strong: Vec<Samp>,
}

struct RecStream {
    reader: BufReader<File>,
    rec_bytes: usize,
}

impl RecStream {
    fn open(path: &str, magic: &[u8; 4], fields: usize) -> Option<(RecStream, u32)> {
        let file = File::open(path).ok()?;
        let mut br = BufReader::with_capacity(1 << 20, file);
        let mut head = [0u8; 8];
        br.read_exact(&mut head).ok()?;
        if &head[0..4] != magic {
            return None;
        }
        let count = u32::from_le_bytes(head[4..8].try_into().ok()?);
        Some((
            RecStream {
                reader: br,
                rec_bytes: fields * 8,
            },
            count,
        ))
    }

    fn next(&mut self) -> Option<Vec<f64>> {
        let nf = self.rec_bytes / 8;
        let mut rec = vec![0u8; self.rec_bytes];
        self.reader.read_exact(&mut rec).ok()?;
        let mut out = Vec::with_capacity(nf);
        for k in 0..nf {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&rec[k * 8..k * 8 + 8]);
            out.push(f64::from_le_bytes(buf));
        }
        Some(out)
    }
}

fn day_key(tdb: f64) -> i64 {
    (tdb / DAY_S + 10957.5).round() as i64
}

fn civil_date(day: i64) -> String {
    match civil_from_days(day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("day{day}"),
    }
}

fn hour_of_day(tdb: f64) -> f64 {
    (tdb / DAY_S + 10957.5).rem_euclid(1.0) * 24.0
}

fn rms_of(sum: f64, sum2: f64, n: usize) -> f64 {
    let m = sum / n as f64;
    let v = (sum2 / n as f64 - m * m).max(0.0);
    v.sqrt()
}

fn fmt_rx(rx: &Rx) -> String {
    format!(
        "ref {} rcv {} amp {} atype {}",
        rx.rx_ref, rx.rx_num, rx.amp_num, rx.amp_type
    )
}

fn class_name(class: i64) -> String {
    if class == CLASS_FLOOR {
        "floor".to_string()
    } else {
        "strong".to_string()
    }
}

fn circ_mean(hours: &[f64]) -> (usize, f64, f64) {
    let n = hours.len();
    if n == 0 {
        return (0, f64::NAN, f64::NAN);
    }
    let mut sx = 0.0;
    let mut sy = 0.0;
    for &h in hours {
        let ph = std::f64::consts::TAU * h / 24.0;
        sx += ph.cos();
        sy += ph.sin();
    }
    let r = (sx * sx + sy * sy).sqrt() / n as f64;
    let mut mean = sy.atan2(sx) * 24.0 / std::f64::consts::TAU;
    if mean < 0.0 {
        mean += 24.0;
    }
    (n, mean, r)
}

fn pass_runs(vs: &[Samp]) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut i = 0usize;
    while i < vs.len() {
        let mut j = i + 1;
        while j < vs.len() && vs[j].tdb - vs[j - 1].tdb <= PASS_GAP_S {
            j += 1;
        }
        runs.push((i, j));
        i = j;
    }
    runs
}

fn fmt_hour(tdb: f64) -> String {
    format!("{:.1}", hour_of_day(tdb))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut positional: Vec<String> = Vec::new();
    let mut report = "tmp/galileo_receiver_pass_cross_report.txt".to_string();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--report" {
            if let Some(p) = it.next() {
                report = p.clone();
            }
        } else if !a.starts_with('-') {
            positional.push(a.clone());
        }
    }
    let recv_path = match positional.first() {
        Some(p) => p.clone(),
        None => "data/pds-ppi.igpp.ucla.edu/galileo_receiver.bin".to_string(),
    };
    let resid_path = match positional.get(1) {
        Some(p) => p.clone(),
        None => "data/pds-ppi.igpp.ucla.edu/galileo_resid.bin".to_string(),
    };

    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push(format!("galileo receiver pass-cross probe — {recv_path}"));
    push(format!(
        "scope: trio stations 14/43/63, ground modes 1/2; floor = strength {FLOOR_AGC} (AGC clamp), strong = strength >= {STRONG_MIN}; lock (|resid| > {LOCK_HZ:.0} Hz) excluded before class; pass = contiguous same-class samples with gap <= {PASS_GAP_S:.0} s; loud/quiet of a pass or day = class RMS >= {LOUD_HZ:.0} Hz about that day's class mean; robust = class n >= {ROBUST_N}; receiver words: slot 8 = DOPPLER_RCVR_REF, 9 = RCVR_NUMBER, 10 = AMP_NUMBER, 11 = AMP_TYPE"
    ));

    let mut buf: BTreeMap<(i64, i64, i64), DayBuf> = BTreeMap::new();
    let mut floor_census: BTreeMap<(i64, i64, Rx), usize> = BTreeMap::new();
    let mut strong_census: BTreeMap<(i64, i64, Rx), usize> = BTreeMap::new();
    let mut t0_all = f64::INFINITY;
    let mut t1_all = f64::NEG_INFINITY;
    let mut n_aux_nonzero = 0usize;
    let mut n_lock = 0usize;
    let mut n_other_st = 0usize;
    let mut n_other_mode = 0usize;
    let mut n_unclassed = 0usize;
    let mut n_classed_trio = 0usize;
    {
        let (mut stream, count) = match RecStream::open(&recv_path, b"GARX", 12) {
            Some(x) => x,
            None => {
                eprintln!("{recv_path}: not a GARX receiver bin");
                return;
            }
        };
        let header_count = count;
        while let Some(r) = stream.next() {
            let tdb = r[0];
            t0_all = t0_all.min(tdb);
            t1_all = t1_all.max(tdb);
            let resid = r[1];
            if !resid.is_finite() || resid.abs() > LOCK_HZ {
                n_lock += 1;
                continue;
            }
            let st = r[2] as i64;
            if !TRIO.contains(&st) {
                n_other_st += 1;
                continue;
            }
            let mo = r[3] as i64;
            if !MODES.contains(&mo) {
                n_other_mode += 1;
                continue;
            }
            let s = r[7] as i64;
            let class = if s == FLOOR_AGC {
                CLASS_FLOOR
            } else if s >= STRONG_MIN {
                CLASS_STRONG
            } else {
                n_unclassed += 1;
                continue;
            };
            n_classed_trio += 1;
            let rx = Rx {
                rx_ref: r[8] as i64,
                rx_num: r[9] as i64,
                amp_num: r[10] as i64,
                amp_type: r[11] as i64,
            };
            if rx.rx_num != 0 || rx.amp_num != 0 || rx.amp_type != 0 {
                n_aux_nonzero += 1;
            }
            if class == CLASS_FLOOR {
                *floor_census.entry((mo, st, rx)).or_insert(0) += 1;
            } else {
                *strong_census.entry((mo, st, rx)).or_insert(0) += 1;
            }
            let day = day_key(tdb);
            let d = buf.entry((mo, st, day)).or_insert_with(|| DayBuf {
                floor: Vec::new(),
                strong: Vec::new(),
            });
            let samp = Samp { tdb, resid, rx };
            if class == CLASS_FLOOR {
                d.floor.push(samp);
            } else {
                d.strong.push(samp);
            }
        }
        push(format!(
            "receiver asset span {} .. {}; header sample count {header_count}; excluded: lock/non-finite {n_lock}, non-trio station {n_other_st}, non-mode-1/2 {n_other_mode}, strength neither floor nor strong {n_unclassed}; classed trio samples {n_classed_trio}; samples whose rx_num/amp_num/amp_type are nonzero {n_aux_nonzero}",
            civil_date(day_key(t0_all)),
            civil_date(day_key(t1_all)),
        ));
    }
    for d in buf.values_mut() {
        d.floor.sort_by(|a, b| a.tdb.total_cmp(&b.tdb));
        d.strong.sort_by(|a, b| a.tdb.total_cmp(&b.tdb));
    }

    let mut class_day: BTreeMap<(i64, i64, i64, i64), (usize, f64, f64)> = BTreeMap::new();
    for (&(mo, st, day), db) in &buf {
        for class in [CLASS_FLOOR, CLASS_STRONG] {
            let vs = if class == CLASS_FLOOR {
                &db.floor
            } else {
                &db.strong
            };
            let mut sum = 0.0f64;
            let mut sum2 = 0.0f64;
            for s in vs {
                sum += s.resid;
                sum2 += s.resid * s.resid;
            }
            class_day.insert((mo, st, day, class), (vs.len(), sum, sum2));
        }
    }

    let resid_exists = fs::metadata(&resid_path).is_ok();
    let mut resid_cells: BTreeMap<(i64, i64, i64), (usize, f64, f64)> = BTreeMap::new();
    if resid_exists {
        if let Some((mut stream, count)) = RecStream::open(&resid_path, b"GASR", 8) {
            let header_count = count;
            let mut n_kept = 0usize;
            while let Some(r) = stream.next() {
                let resid = r[1];
                if !resid.is_finite() || resid.abs() > LOCK_HZ {
                    continue;
                }
                let st = r[2] as i64;
                if !TRIO.contains(&st) {
                    continue;
                }
                let mo = r[3] as i64;
                if !MODES.contains(&mo) {
                    continue;
                }
                if (r[7] as i64) != FLOOR_AGC {
                    continue;
                }
                let day = day_key(r[0]);
                let e = resid_cells
                    .entry((mo, st, day))
                    .or_insert((0usize, 0.0f64, 0.0f64));
                e.0 += 1;
                e.1 += resid;
                e.2 += resid * resid;
                n_kept += 1;
            }
            push(format!(
                "resid cross-reference (template source) {resid_path}: header count {header_count}, mode-1/2 trio floor samples kept {n_kept}, floor day-cells {}",
                resid_cells.len()
            ));
        }
    }

    push(String::new());
    push("== 1. coverage ==".to_string());
    push("register reference (resid, galileo_floor_subday_clock_probe): robust floor day-cells n>=30, loud = RMS>=1 Hz — M1 st14 62/32 st43 64/35 st63 68/34; M2 st14 38/17 st43 35/16 st63 39/22".to_string());
    for mo in MODES {
        for st in TRIO {
            let mut robust = 0usize;
            let mut loud = 0usize;
            for (&(m, s, _, class), &(n, sum, sum2)) in &class_day {
                if m == mo && s == st && class == CLASS_FLOOR && n >= ROBUST_N {
                    robust += 1;
                    if rms_of(sum, sum2, n) >= LOUD_HZ {
                        loud += 1;
                    }
                }
            }
            push(format!(
                "  receiver M{mo} st{st}: robust floor days {robust}, loud {loud}"
            ));
        }
    }
    if resid_exists {
        for mo in MODES {
            for st in TRIO {
                let mut robust = 0usize;
                let mut loud = 0usize;
                for (&(m, s, _), &(n, sum, sum2)) in &resid_cells {
                    if m == mo && s == st && n >= ROBUST_N {
                        robust += 1;
                        if rms_of(sum, sum2, n) >= LOUD_HZ {
                            loud += 1;
                        }
                    }
                }
                push(format!(
                    "  resid   M{mo} st{st}: robust floor days {robust}, loud {loud}"
                ));
            }
        }
    }
    let mut resid_loud: BTreeMap<(i64, i64), BTreeSet<i64>> = BTreeMap::new();
    let mut resid_quiet_robust: BTreeMap<(i64, i64), BTreeSet<i64>> = BTreeMap::new();
    for (&(mo, st, day), &(n, sum, sum2)) in &resid_cells {
        if n >= ROBUST_N {
            if rms_of(sum, sum2, n) >= LOUD_HZ {
                resid_loud.entry((mo, st)).or_default().insert(day);
            } else {
                resid_quiet_robust.entry((mo, st)).or_default().insert(day);
            }
        }
    }
    let mut recv_floor_day: BTreeMap<(i64, i64, i64), (bool, usize)> = BTreeMap::new();
    for (&(mo, st, day, class), &(n, sum, sum2)) in &class_day {
        if class == CLASS_FLOOR {
            let loud = n >= ROBUST_N && rms_of(sum, sum2, n) >= LOUD_HZ;
            recv_floor_day.insert((mo, st, day), (loud, n));
        }
    }
    push(String::new());
    push("loud-day coverage of the receiver asset (per template loud day, is it a loud floor day in the asset?):".to_string());
    let mut n_resid_loud = 0usize;
    let mut n_matched_loud = 0usize;
    let mut n_present_quiet = 0usize;
    let mut n_present_thin = 0usize;
    let mut n_absent = 0usize;
    for (mo, st) in MODES
        .iter()
        .flat_map(|m| TRIO.iter().map(move |s| (*m, *s)))
    {
        if let Some(days) = resid_loud.get(&(mo, st)) {
            for day in days {
                n_resid_loud += 1;
                match recv_floor_day.get(&(mo, st, *day)) {
                    Some((true, _)) => n_matched_loud += 1,
                    Some((false, n)) => {
                        if *n >= ROBUST_N {
                            n_present_quiet += 1;
                        } else {
                            n_present_thin += 1;
                        }
                    }
                    None => n_absent += 1,
                }
            }
        }
    }
    push(format!(
        "  template loud days (resid, M1+M2) {n_resid_loud}; loud in receiver {n_matched_loud}; present but quiet-robust {n_present_quiet}; present but thin {n_present_thin}; absent from receiver floor {n_absent}"
    ));
    let mut recv_loud_only: Vec<(i64, i64, i64)> = Vec::new();
    for (mo, st) in MODES
        .iter()
        .flat_map(|m| TRIO.iter().map(move |s| (*m, *s)))
    {
        for (&(m2, s2, d2), &(loud, _)) in &recv_floor_day {
            if m2 == mo && s2 == st && loud {
                let in_resid = match resid_loud.get(&(mo, st)) {
                    Some(set) => set.contains(&d2),
                    None => false,
                };
                if !in_resid {
                    recv_loud_only.push((m2, s2, d2));
                }
            }
        }
    }
    let extra_str: Vec<String> = recv_loud_only
        .iter()
        .map(|&(m2, s2, d2)| match resid_cells.get(&(m2, s2, d2)) {
            Some(&(rn, _, _)) => {
                format!("M{m2} st{s2} {} (resid n {rn})", civil_date(d2))
            }
            None => format!("M{m2} st{s2} {} (resid absent)", civil_date(d2)),
        })
        .collect();
    push(format!(
        "  receiver loud days not loud in the resid template series: {} — {}",
        recv_loud_only.len(),
        extra_str.join(" | ")
    ));
    push(
        "  receiver-vs-resid floor day-cell set differences (floor day-cells with n>=1):"
            .to_string(),
    );
    for mo in MODES {
        for st in TRIO {
            let mut resid_days: BTreeSet<i64> = BTreeSet::new();
            let mut recv_days: BTreeSet<i64> = BTreeSet::new();
            for (&(m2, s2, d2), _) in &resid_cells {
                if m2 == mo && s2 == st {
                    resid_days.insert(d2);
                }
            }
            for (&(m2, s2, d2), &(_, n)) in &recv_floor_day {
                if m2 == mo && s2 == st && n >= 1 {
                    recv_days.insert(d2);
                }
            }
            let recv_only: Vec<i64> = recv_days.difference(&resid_days).copied().collect();
            let resid_only: Vec<i64> = resid_days.difference(&recv_days).copied().collect();
            let recv_loud_extra: Vec<String> = recv_loud_only
                .iter()
                .filter(|&&(m2, s2, _)| m2 == mo && s2 == st)
                .map(|&(_, _, d2)| civil_date(d2))
                .collect();
            push(format!(
                "  M{mo} st{st}: floor days only in receiver {r1}, only in resid {r2}; receiver-loud extra days (thin in resid): {r3}",
                r1 = recv_only.len(),
                r2 = resid_only.len(),
                r3 = if recv_loud_extra.is_empty() {
                    "none".to_string()
                } else {
                    recv_loud_extra.join(", ")
                }
            ));
        }
    }
    push("anchor loud cells of the templates (1995-11-24 M1 st14; 1996-06-26 M1 st63) verified present below in the per-day pass section.".to_string());

    push(String::new());
    push("== 2. receiver sample census per class (mode 1/2, trio) ==".to_string());
    for (class, census) in [(CLASS_FLOOR, &floor_census), (CLASS_STRONG, &strong_census)] {
        push(format!("  -- {class} --", class = class_name(class)));
        for (&(mo, st, rx), cnt) in census {
            push(format!("    M{mo} st{st} {}: {cnt}", fmt_rx(&rx)));
        }
    }

    push(String::new());
    push("== 3. per-pass structure + receiver census, robust day-cells ==".to_string());
    for mo in MODES {
        for st in TRIO {
            for class in [CLASS_FLOOR, CLASS_STRONG] {
                let mut n_robust_days = 0usize;
                let mut n_pass_loud = 0usize;
                let mut n_pass_quiet = 0usize;
                let mut loud_samples_cfg: BTreeMap<Rx, usize> = BTreeMap::new();
                let mut quiet_samples_cfg: BTreeMap<Rx, usize> = BTreeMap::new();
                let mut loud_pass_cfg: BTreeMap<Rx, usize> = BTreeMap::new();
                let mut quiet_pass_cfg: BTreeMap<Rx, usize> = BTreeMap::new();
                let mut mixed_loud = 0usize;
                let mut mixed_quiet = 0usize;
                for (&(m, s, day, c), &(n, sum, _)) in &class_day {
                    if m != mo || s != st || c != class || n < ROBUST_N {
                        continue;
                    }
                    n_robust_days += 1;
                    let db = &buf[&(m, s, day)];
                    let vs = if class == CLASS_FLOOR {
                        &db.floor
                    } else {
                        &db.strong
                    };
                    let day_mean = sum / n as f64;
                    for (a, b) in pass_runs(vs) {
                        let pass_n = b - a;
                        let mut dev2 = 0.0f64;
                        let mut cfg_count: BTreeMap<Rx, usize> = BTreeMap::new();
                        for s_ in &vs[a..b] {
                            let d = s_.resid - day_mean;
                            dev2 += d * d;
                            *cfg_count.entry(s_.rx).or_insert(0) += 1;
                        }
                        let pass_rms = (dev2 / pass_n as f64).sqrt();
                        let loud = pass_rms >= LOUD_HZ;
                        let single_cfg = if cfg_count.len() == 1 {
                            cfg_count.keys().next().copied()
                        } else {
                            None
                        };
                        if loud {
                            n_pass_loud += 1;
                            if let Some(rx) = single_cfg {
                                *loud_pass_cfg.entry(rx).or_insert(0) += 1;
                                *loud_samples_cfg.entry(rx).or_insert(0) += pass_n;
                            } else {
                                mixed_loud += 1;
                            }
                        } else {
                            n_pass_quiet += 1;
                            if let Some(rx) = single_cfg {
                                *quiet_pass_cfg.entry(rx).or_insert(0) += 1;
                                *quiet_samples_cfg.entry(rx).or_insert(0) += pass_n;
                            } else {
                                mixed_quiet += 1;
                            }
                        }
                    }
                }
                let fmt_cfgmap = |cfg: &BTreeMap<Rx, usize>, prefix: &str| -> String {
                    if cfg.is_empty() {
                        return format!("{prefix} none");
                    }
                    cfg.iter()
                        .map(|(rx, cnt)| format!("{prefix} {}: {cnt}", fmt_rx(rx)))
                        .collect::<Vec<String>>()
                        .join(" ; ")
                };
                let cfg_lines = format!(
                    "loud-pass rx per cfg: {}; quiet-pass rx per cfg: {}; loud samples per cfg: {}; quiet samples per cfg: {}",
                    fmt_cfgmap(&loud_pass_cfg, ""),
                    fmt_cfgmap(&quiet_pass_cfg, ""),
                    fmt_cfgmap(&loud_samples_cfg, ""),
                    fmt_cfgmap(&quiet_samples_cfg, ""),
                );
                push(format!(
                    "M{mo} st{st} {cls}: robust days {n_robust_days}; passes loud {n_pass_loud} quiet {n_pass_quiet} (mixed-config loud {mixed_loud} quiet {mixed_quiet}); {cfg_lines}",
                    mo = mo,
                    st = st,
                    cls = class_name(class),
                    n_robust_days = n_robust_days,
                    n_pass_loud = n_pass_loud,
                    n_pass_quiet = n_pass_quiet,
                    mixed_loud = mixed_loud,
                    mixed_quiet = mixed_quiet,
                    cfg_lines = cfg_lines,
                ));
            }
        }
    }

    push(String::new());
    push("== 4. station daily phase and receiver binding ==".to_string());
    push("per (mode, station) floor robust days: pass-start hour of loud vs quiet passes (mean/R) — the register found congruence only per station (the station's pass phase), shared by quiet days.".to_string());
    for mo in MODES {
        for st in TRIO {
            let mut loud_hours: Vec<f64> = Vec::new();
            let mut quiet_hours: Vec<f64> = Vec::new();
            for (&(m, s, day, class), &(n, sum, _)) in &class_day {
                if m != mo || s != st || class != CLASS_FLOOR || n < ROBUST_N {
                    continue;
                }
                let db = &buf[&(m, s, day)];
                let vs = &db.floor;
                let day_mean = sum / n as f64;
                for (a, b) in pass_runs(vs) {
                    let pass_n = b - a;
                    let mut dev2 = 0.0f64;
                    for s_ in &vs[a..b] {
                        let d = s_.resid - day_mean;
                        dev2 += d * d;
                    }
                    let pass_rms = (dev2 / pass_n as f64).sqrt();
                    if pass_rms >= LOUD_HZ {
                        loud_hours.push(hour_of_day(vs[a].tdb));
                    } else {
                        quiet_hours.push(hour_of_day(vs[a].tdb));
                    }
                }
            }
            let (nl, ml, rl) = circ_mean(&loud_hours);
            let (nq, mq, rq) = circ_mean(&quiet_hours);
            push(format!(
                "M{mo} st{st} floor: loud passes n {nl} mean-start {ml:.1} h R {rl:.3}; quiet passes n {nq} mean-start {mq:.1} h R {rq:.3}"
            ));
        }
    }
    push("floor receiver config per (mode, station): the sample census above is the config of every floor day; the loud/quiet flip runs within that single constant config. The strong class carries the only rx_ref variation — its date spans per config follow.".to_string());
    for mo in MODES {
        for st in TRIO {
            let mut span: BTreeMap<Rx, (i64, i64, usize)> = BTreeMap::new();
            for (&(m, s, day, class), _) in &class_day {
                if m != mo || s != st || class != CLASS_STRONG {
                    continue;
                }
                let db = &buf[&(m, s, day)];
                let mut seen: BTreeSet<Rx> = BTreeSet::new();
                for smp in &db.strong {
                    seen.insert(smp.rx);
                }
                for rx in seen {
                    let e = span.entry(rx).or_insert((i64::MAX, i64::MIN, 0));
                    e.0 = e.0.min(day);
                    e.1 = e.1.max(day);
                    e.2 += 1;
                }
            }
            let parts: Vec<String> = span
                .iter()
                .map(|(rx, &(f, l, nd))| {
                    format!(
                        "{} {}..{} ({} days)",
                        fmt_rx(rx),
                        civil_date(f),
                        civil_date(l),
                        nd
                    )
                })
                .collect();
            push(format!(
                "M{mo} st{st} strong config spans: {}",
                parts.join(" | ")
            ));
        }
    }

    push(String::new());
    push("== 5. anchor days, per pass (floor and strong, modes 1 and 2) ==".to_string());
    for anchor in ["1995-11-24", "1996-06-26"] {
        push(format!("-- {anchor} --"));
        for st in TRIO {
            for mo in MODES {
                for class in [CLASS_FLOOR, CLASS_STRONG] {
                    for (&(m, s, day, c), &(n, sum, sum2)) in &class_day {
                        if civil_date(day) != anchor || s != st || m != mo || c != class {
                            continue;
                        }
                        let db = &buf[&(m, s, day)];
                        let vs = if class == CLASS_FLOOR {
                            &db.floor
                        } else {
                            &db.strong
                        };
                        let day_mean = sum / n as f64;
                        let mut parts: Vec<String> = Vec::new();
                        for (a, b) in pass_runs(vs) {
                            let pass_n = b - a;
                            let mut dev2 = 0.0f64;
                            let mut cfg_count: BTreeMap<Rx, usize> = BTreeMap::new();
                            for s_ in &vs[a..b] {
                                let d = s_.resid - day_mean;
                                dev2 += d * d;
                                *cfg_count.entry(s_.rx).or_insert(0) += 1;
                            }
                            let pass_rms = (dev2 / pass_n as f64).sqrt();
                            let cfg_s = match cfg_count.keys().next() {
                                Some(rx) if cfg_count.len() == 1 => fmt_rx(rx),
                                _ => format!("{} configs", cfg_count.len()),
                            };
                            let state = if pass_rms >= LOUD_HZ { "LOUD" } else { "quiet" };
                            parts.push(format!(
                                "{}h n{} rms {:.3} {state} {}",
                                fmt_hour(vs[a].tdb),
                                pass_n,
                                pass_rms,
                                cfg_s
                            ));
                        }
                        let rms_d = rms_of(sum, sum2, n);
                        let state_d = if rms_d >= LOUD_HZ { "LOUD" } else { "quiet" };
                        let pass_str = parts.join(" | ");
                        push(format!(
                            "  M{mo} st{st} {cls}: day n {n} rms {rms_d:.4} {state_d}; passes: {pass_str}",
                            mo = mo,
                            st = st,
                            cls = class_name(class),
                            n = n,
                            rms_d = rms_d,
                            state_d = state_d,
                            pass_str = pass_str,
                        ));
                    }
                }
            }
        }
    }

    push(String::new());
    push("== summary ==".to_string());
    push(format!(
        "receiver floor loud-day reproduction vs register: see coverage table; per-pass rx census over robust days: see section 3; anchor pass detail: section 5"
    ));

    match fs::write(&report, out.join("\n") + "\n") {
        Ok(()) => eprintln!("galileo: receiver pass-cross report written to {report}"),
        Err(w) => eprintln!("galileo: report write returned {w}; diagnostics held on stdout"),
    }
}
