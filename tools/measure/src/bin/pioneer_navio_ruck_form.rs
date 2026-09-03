use std::collections::BTreeMap;

use omegaflow::archivar::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const GATE_SD: f64 = 4.0;
const COARSE_MIN_DAYS: usize = 3;
const FINE_MIN_DAYS: usize = 10;

fn jd_date(tdb: f64) -> String {
    let jd = tdb / DAY_S + 2451545.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb:.0} s"),
    }
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}

fn is_continuous(a: f64, b: f64) -> bool {
    (b - a) <= 1.5 * DAY_S
}

fn read_daily(path: &str) -> Option<Vec<(f64, f64)>> {
    let d = std::fs::read(path).ok()?;
    if d.len() < 8 || &d[0..4] != b"PNDM" {
        return None;
    }
    let cnt = u32::from_le_bytes(d[4..8].try_into().ok()?) as usize;
    if d.len() != 8 + cnt * 32 {
        return None;
    }
    let mut out = Vec::new();
    for i in 0..cnt {
        let o = 8 + i * 32;
        let t = f64::from_le_bytes(d[o..o + 8].try_into().ok()?);
        let med = f64::from_le_bytes(d[o + 8..o + 16].try_into().ok()?);
        out.push((t, med));
    }
    Some(out)
}

#[derive(Clone, Copy)]
struct Form {
    jump: f64,
    pre: f64,
    post: f64,
    rise_days: usize,
    returns: bool,
    returns_measured: bool,
    sign_flips: usize,
    seg_len: usize,
}

fn measure_form(daily: &[(f64, f64)], k: usize) -> Option<Form> {
    let n = daily.len();

    let mut lo = k;
    while lo > 0 && is_continuous(daily[lo - 1].0, daily[lo].0) {
        lo -= 1;
    }
    let mut hi = k;
    while hi + 1 < n && is_continuous(daily[hi].0, daily[hi + 1].0) {
        hi += 1;
    }
    let seg = &daily[lo..=hi];
    let seg_len = seg.len();
    let kk = k - lo;

    if seg_len < COARSE_MIN_DAYS || kk == 0 || kk + 1 >= seg_len {
        return None;
    }

    let pre: Vec<f64> = seg[..kk].iter().map(|x| x.1).collect();
    let post: Vec<f64> = seg[kk + 1..].iter().map(|x| x.1).collect();
    if pre.is_empty() || post.is_empty() {
        return None;
    }
    let pre_med = median(&pre);
    let post_med = median(&post);
    let jump = post_med - pre_med;
    let rise_days = 1 + post.len().min(pre.len());

    let mut sign_flips = 0usize;
    for i in 1..seg.len() {
        if seg[i].1 - seg[i - 1].1 == 0.0 {
            continue;
        }
        let dp = if i >= 2 {
            seg[i - 1].1 - seg[i - 2].1
        } else {
            0.0
        };
        if i >= 2 && (seg[i].1 - seg[i - 1].1) * dp < 0.0 {
            sign_flips += 1;
        }
    }

    let returns_measured = seg_len >= FINE_MIN_DAYS;
    let returns = if returns_measured {
        let post_tail: Vec<f64> = post.iter().cloned().collect();
        let tail_med = median(&post_tail);
        (tail_med - pre_med).abs() < 0.5 * jump.abs()
    } else {
        false
    };

    Some(Form {
        jump,
        pre: pre_med,
        post: post_med,
        rise_days,
        returns,
        returns_measured,
        sign_flips,
        seg_len,
    })
}

fn run(name: &str) {
    let path = format!("data/{name}_navio_subkhz_daily.bin");
    if !std::path::Path::new(&path).exists() {
        let path = format!("data/{name}_navio_daily.bin");
        let Some(daily) = read_daily(&path) else {
            eprintln!("{name}: daily bin void/parse void ({path})");
            return;
        };
        scan(name, &daily);
        return;
    }
    let Some(daily) = read_daily(&path) else {
        eprintln!("{name}: daily bin void/parse void ({path})");
        return;
    };
    eprintln!("{name}: scanning the sub-kHz negative-fuzzy daily-median basis ({path})");
    scan(name, &daily);
}

fn scan(name: &str, daily: &[(f64, f64)]) {
    let ts: Vec<f64> = daily.iter().map(|x| x.0).collect();
    let vs: Vec<f64> = daily.iter().map(|x| x.1).collect();

    let mut ruck: Vec<f64> = Vec::new();
    for i in 1..ts.len() - 1 {
        if !is_continuous(ts[i - 1], ts[i]) || !is_continuous(ts[i], ts[i + 1]) {
            ruck.push(f64::NAN);
        } else {
            ruck.push((vs[i + 1] - vs[i - 1]) / (2.0 * DAY_S));
        }
    }
    let mut finite: Vec<f64> = ruck.iter().cloned().filter(|v| v.is_finite()).collect();
    if finite.len() < 20 {
        eprintln!("{name}: too few finite Ruck steps, stays silent");
        return;
    }
    finite.sort_by(f64::total_cmp);
    let med = finite[finite.len() / 2];
    let devs: Vec<f64> = finite.iter().map(|x| (x - med).abs()).collect();
    let mut sdevs = devs.clone();
    sdevs.sort_by(f64::total_cmp);
    let mad = sdevs[sdevs.len() / 2];
    let sd = 1.4826 * mad;
    let gate = GATE_SD * sd;

    let mut flagged: Vec<(usize, f64)> = Vec::new();
    for (i, &r) in ruck.iter().enumerate() {
        if r.is_finite() && (r - med).abs() > gate {
            flagged.push((i + 1, r));
        }
    }
    eprintln!(
        "{name}: {n} daily steps, Ruck median {med:.3e}, sd {sd:.3e}, gate {gate:.3e} ({GATE_SD}·sd) — {nflag} flagged",
        n = daily.len(),
        nflag = flagged.len()
    );

    let mut measured = 0usize;
    let mut coarse = 0usize;
    let mut fine = 0usize;
    let mut too_short = 0usize;
    let mut sign_flip_dist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut seg_dist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut returns_count = 0usize;
    let mut returns_measured_count = 0usize;

    let mut forms: Vec<(String, Form)> = Vec::new();
    for (k, _) in &flagged {
        let Some(f) = measure_form(&daily, *k) else {
            too_short += 1;
            continue;
        };
        measured += 1;
        if f.seg_len >= FINE_MIN_DAYS {
            fine += 1;
        } else {
            coarse += 1;
        }
        *seg_dist.entry(f.seg_len).or_insert(0) += 1;
        *sign_flip_dist.entry(f.sign_flips).or_insert(0) += 1;
        if f.returns_measured {
            returns_measured_count += 1;
            if f.returns {
                returns_count += 1;
            }
        }
        forms.push((jd_date(ts[*k]), f));
    }

    forms.sort_by(|a, b| b.1.jump.abs().total_cmp(&a.1.jump.abs()));

    eprintln!(
        "{name}: form measured on {measured} of {nflag} flags — {fine} fine (≥{FINE_MIN_DAYS}d), {coarse} coarse ({COARSE_MIN_DAYS}-{fine}...d), {too_short} below {COARSE_MIN_DAYS}-day continuous window (benannt, nicht bemalt, 0 honored)",
        fine = fine,
        coarse = coarse,
        nflag = flagged.len()
    );
    eprintln!("{name}: segment length dist (days->count): {:?}", seg_dist);
    eprintln!(
        "{name}: sign-flip (zitter) dist: {:?}; returns measured on {returns_measured_count}, returned to pre-baseline: {returns_count}",
        sign_flip_dist
    );

    eprintln!("{name}: top form profiles by |jump|:");
    for (date, f) in forms.iter().take(15) {
        let ret = if f.returns_measured {
            if f.returns {
                "returns"
            } else {
                "holds"
            }
        } else {
            "returns:n/a"
        };
        eprintln!(
            "  {date}: jump {j:.3e} Hz, pre {p:.3e} .. post {q:.3e} Hz, rise {r}d over {s}-day seg, {flips} sign-flips, {ret}",
            j = f.jump,
            p = f.pre,
            q = f.post,
            r = f.rise_days,
            s = f.seg_len,
            flips = f.sign_flips
        );
    }
}

fn parse_date(s: &str) -> Option<(i32, u32, u32)> {
    let p: Vec<&str> = s.split('-').collect();
    if p.len() != 3 {
        return None;
    }
    let y: i32 = p[0].parse().ok()?;
    let m: u32 = p[1].parse().ok()?;
    let d: u32 = p[2].parse().ok()?;
    Some((y, m, d))
}

fn date_tdb(y: i32, m: u32, d: u32) -> Option<f64> {
    let days = omegaflow::archivar::lsk::days_from_civil(y as i64, m as i64, d as i64)?;
    Some((days as f64 + 2440587.5 - 2451545.0) * DAY_S)
}

fn zitter_vs_step(daily: &[(f64, f64)], center: usize) {
    let lo = center.saturating_sub(15);
    let hi = (center + 15).min(daily.len());
    if hi - lo < 10 {
        eprintln!("  too few days in the ±15 window for a form reading");
        return;
    }
    let win: Vec<f64> = daily[lo..hi].iter().map(|x| x.1).collect();
    let mut flips = 0usize;
    let mut prev: Option<f64> = None;
    for i in 1..win.len() {
        let dd = win[i] - win[i - 1];
        if dd == 0.0 {
            continue;
        }
        if let Some(p) = prev {
            if dd * p < 0.0 {
                flips += 1;
            }
        }
        prev = Some(dd);
    }
    let min = win.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = win.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let p2p = max - min;
    let pre = &win[..9.min(win.len())];
    let post = &win[10.min(win.len())..];
    let pm = median(pre);
    let qm = median(post);
    let step = qm - pm;
    eprintln!(
        "  form: {}-day window mean {:.0} Hz, peak-to-peak {:.0} Hz, sign-flips {flips}, pre→post median step {step:.0} Hz",
        win.len(),
        win.iter().sum::<f64>() / win.len() as f64,
        p2p
    );
    if step.abs() > 0.5 * p2p && flips <= 3 {
        eprintln!("  verdict: step dominates zitter -> RAMP/STEP form (transit-shaped candidate)");
    } else {
        eprintln!("  verdict: zitter ({flips} flips, p2p {p2p:.0}) dominates the step ({step:.0} Hz) -> ARTIFACT form (noise), not a ramp");
    }
}

fn measure_date(name: &str, date_str: &str) {
    let path = format!("data/{name}_navio_daily.bin");
    let Some(daily) = read_daily(&path) else {
        eprintln!("{name}: daily bin void/parse void ({path})");
        return;
    };
    let Some((y, m, d)) = parse_date(date_str) else {
        eprintln!("{name}: bad date {date_str}");
        return;
    };
    let Some(target) = date_tdb(y, m, d) else {
        eprintln!("{name}: date out of range");
        return;
    };
    let mut best = 0usize;
    let mut bd = f64::INFINITY;
    for (i, (t, _)) in daily.iter().enumerate() {
        let dd = (t - target).abs();
        if dd < bd {
            bd = dd;
            best = i;
        }
    }
    eprintln!(
        "{name}: nearest daily point to {date_str} is {date} (offset {off:.1} d) — form measurement (flag-independent):",
        date = jd_date(daily[best].0),
        off = bd / DAY_S
    );
    if let Some(f) = measure_form(&daily, best) {
        eprintln!(
            "  descriptor: jump {:.3e} Hz, pre {:.3e} .. post {:.3e} Hz, rise {}d over {}-day seg, {} sign-flips, returns {}",
            f.jump,
            f.pre,
            f.post,
            f.rise_days,
            f.seg_len,
            f.sign_flips,
            if f.returns_measured {
                if f.returns { "yes" } else { "no (holds)" }
            } else {
                "n/a"
            }
        );
    } else {
        eprintln!("  descriptor: not measurable at the flag position (edge of a segment)");
    }
    zitter_vs_step(&daily, best);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut target: Option<(&str, String)> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--date" && i + 1 < args.len() {
            target = Some(("pioneer11", args[i + 1].clone()));
            i += 2;
        } else {
            i += 1;
        }
    }
    match target {
        Some((name, date)) => measure_date(name, &date),
        None => {
            for name in ["pioneer10", "pioneer11"] {
                run(name);
            }
        }
    }
}
