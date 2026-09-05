use std::collections::BTreeMap;

use omegaflow::atdf::parse_resid_bin;
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const GAP_S: f64 = 600.0;
const SUB_GAP_S: f64 = 120.0;
const FLOOR: f64 = -2560.0;
const PLATEAU_MIN: f64 = -1900.0;
const MIN_N: usize = 30;
const MAX_W: usize = 60;
const MODE: i64 = 1;
const STATIONS: [i64; 2] = [43, 63];

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    match civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("t {tdb_s:.0}"),
    }
}

fn median(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some(s[s.len() / 2])
}

fn fmt_o(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.3}"),
        _ => "-".to_string(),
    }
}

#[derive(Clone, Copy)]
struct Rec {
    t: f64,
    resid: f64,
    s: f64,
}

impl Rec {
    fn tracked(self) -> bool {
        self.resid.abs() <= LOCK_HZ && self.s != 0.0
    }
}

fn window_rms(pass: &[Rec], stream: &[usize], a: usize, b: usize) -> Option<f64> {
    if b <= a {
        return None;
    }
    let mut mean = 0.0f64;
    for k in a..b {
        mean += pass[stream[k]].resid;
    }
    mean /= (b - a) as f64;
    let mut m2 = 0.0f64;
    for k in a..b {
        let d = pass[stream[k]].resid - mean;
        m2 += d * d;
    }
    Some((m2 / (b - a) as f64).sqrt())
}

fn window_plat_frac(pass: &[Rec], stream: &[usize], a: usize, b: usize) -> f64 {
    if b <= a {
        return 0.0;
    }
    let mut p = 0usize;
    for k in a..b {
        if pass[stream[k]].s >= PLATEAU_MIN {
            p += 1;
        }
    }
    p as f64 / (b - a) as f64
}

struct Onset {
    t: f64,
    date: String,
    pre_n: usize,
    post_n: usize,
    pre_rms: f64,
    post_rms: f64,
    pre_plat_frac: f64,
}

struct DualAgg {
    n_dual: usize,
    floor_runs: usize,
    at_pass_start: usize,
    arcstart: usize,
    pre_short: usize,
    usable: usize,
    onsets: Vec<Onset>,
    profile: Vec<(i64, f64)>,
}

struct PassAgg {
    n_floor_runs: usize,
    n_plateau_runs: usize,
    onsets: Vec<Onset>,
    at_pass_start: usize,
    arcstart: usize,
    pre_short: usize,
    usable: usize,
    profile: Vec<(i64, f64)>,
}

fn analyze_pass(pass: &[Rec]) -> PassAgg {
    let mut agg = PassAgg {
        n_floor_runs: 0,
        n_plateau_runs: 0,
        onsets: Vec::new(),
        at_pass_start: 0,
        arcstart: 0,
        pre_short: 0,
        usable: 0,
        profile: Vec::new(),
    };
    let mut stream: Vec<usize> = Vec::new();
    for (i, r) in pass.iter().enumerate() {
        if r.tracked() {
            stream.push(i);
        }
    }
    if stream.is_empty() {
        return agg;
    }
    let mut arcs: Vec<(usize, usize)> = Vec::new();
    let mut lo = 0usize;
    for k in 1..=stream.len() {
        let brk = if k == stream.len() {
            true
        } else {
            let a = stream[k - 1];
            let b = stream[k];
            b > a + 1 || pass[b].t - pass[a].t > SUB_GAP_S
        };
        if brk {
            if k > lo {
                arcs.push((lo, k));
            }
            lo = k;
        }
    }
    for &(loa, hia) in &arcs {
        let mut q = loa;
        while q < hia {
            let plat = pass[stream[q]].s >= PLATEAU_MIN;
            let mut r = q + 1;
            while r < hia && (pass[stream[r]].s >= PLATEAU_MIN) == plat {
                r += 1;
            }
            if plat && r - q >= MIN_N {
                agg.n_plateau_runs += 1;
            }
            q = r;
        }
    }
    for &(loa, hia) in &arcs {
        let mut q = loa;
        while q < hia {
            let fl = pass[stream[q]].s <= FLOOR;
            let mut r = q + 1;
            while r < hia && (pass[stream[r]].s <= FLOOR) == fl {
                r += 1;
            }
            if fl {
                let flen = r - q;
                if flen >= MIN_N {
                    agg.n_floor_runs += 1;
                    if q == loa {
                        agg.arcstart += 1;
                        if loa == 0 && q == 0 {
                            agg.at_pass_start += 1;
                        }
                    } else {
                        let mut k = q;
                        while k > loa && pass[stream[k - 1]].s > FLOOR {
                            k -= 1;
                        }
                        let pre_len = q - k;
                        if pre_len < MIN_N {
                            agg.pre_short += 1;
                        } else {
                            agg.usable += 1;
                            let pre_w = pre_len.min(MAX_W);
                            let post_w = flen.min(MAX_W);
                            let pre_rms = window_rms(pass, &stream, q - pre_w, q);
                            let post_rms = window_rms(pass, &stream, q, q + post_w);
                            if let (Some(a), Some(b)) = (pre_rms, post_rms) {
                                agg.onsets.push(Onset {
                                    t: pass[stream[q]].t,
                                    date: jd_date(pass[stream[q]].t),
                                    pre_n: pre_w,
                                    post_n: post_w,
                                    pre_rms: a,
                                    post_rms: b,
                                    pre_plat_frac: window_plat_frac(pass, &stream, q - pre_w, q),
                                });
                            }
                            for off in [-90i64, -60, -30, 0, 30, 60, 90] {
                                let wl = q as i64 + off;
                                if wl < loa as i64 {
                                    continue;
                                }
                                let wla = wl as usize;
                                let wlb = wla + MIN_N;
                                if wlb > hia {
                                    continue;
                                }
                                if let Some(rv) = window_rms(pass, &stream, wla, wlb) {
                                    agg.profile.push((off, rv));
                                }
                            }
                        }
                    }
                }
            }
            q = r;
        }
    }
    agg
}

fn main() {
    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut per: BTreeMap<i64, Vec<Rec>> = BTreeMap::new();
    let mut n_mode1 = 0usize;
    let mut n_lock = 0usize;
    for r in &recs {
        if r[3] as i64 != MODE {
            continue;
        }
        n_mode1 += 1;
        if r[1].abs() > LOCK_HZ {
            n_lock += 1;
        }
        if STATIONS.contains(&(r[2] as i64)) {
            per.entry(r[2] as i64).or_default().push(Rec {
                t: r[0],
                resid: r[1],
                s: r[7],
            });
        }
    }
    drop(recs);
    for v in per.values_mut() {
        v.sort_by(|a, b| a.t.total_cmp(&b.t));
    }

    let mut out: Vec<String> = Vec::new();
    out.push("galileo floor-onset lead probe — is the within-pass floor-vs-plateau noise covariance directional (floor -> noise) or simultaneous?".to_string());
    out.push("binding: pass = contiguous tracking arc per station, boundary = time gap between consecutive samples > 600 s (mode 1); tracked sample = non-lock |resid| <= 1000 Hz and strength != 0; arc = consecutive tracked samples with no lock/null sample between and a time gap <= 120 s; floor = strength <= -2560 (AGC clamp), plateau = strength >= -1900; dual pass = an arc has a plateau run and a floor run, each >= 30 tracked samples; floor onset = the first tracked sample of a floor run that has an in-arc non-floor tracked sample immediately before it; noise = RMS of resid about the window mean; pre window = up to 60 tracked samples of the maximal non-floor run immediately before the onset (>= 30 required); post window = up to 60 tracked samples from the onset inside the floor run (>= 30 required); usable onset = inside a dual pass, floor run >= 30, adjacent pre run >= 30".to_string());
    out.push(format!("stations {:?}, mode {MODE}, lock transitions excluded ({n_lock} of {n_mode1} mode-1 samples)", STATIONS));
    out.push(String::new());

    out.push("station  samples  lock  passes  dual  floor_runs_ge30  pass_start  arcstart_noPre  preShort  usable  med_pre  med_post  med_delta  frac_postGTpre".to_string());
    let mut pooled_pre: Vec<f64> = Vec::new();
    let mut pooled_post: Vec<f64> = Vec::new();
    let mut pooled_delta: Vec<f64> = Vec::new();
    let mut pooled_profile: Vec<(i64, f64)> = Vec::new();

    for station in STATIONS {
        let Some(list) = per.get(&station) else {
            continue;
        };
        let n_lock_st: usize = list.iter().filter(|r| r.resid.abs() > LOCK_HZ).count();
        let mut passes: Vec<(usize, usize)> = Vec::new();
        let mut b0 = 0usize;
        for i in 1..=list.len() {
            let brk = i == list.len() || list[i].t - list[i - 1].t > GAP_S;
            if brk {
                if i > b0 {
                    passes.push((b0, i));
                }
                b0 = i;
            }
        }
        let mut agg = DualAgg {
            n_dual: 0,
            floor_runs: 0,
            at_pass_start: 0,
            arcstart: 0,
            pre_short: 0,
            usable: 0,
            onsets: Vec::new(),
            profile: Vec::new(),
        };
        for &(a, b) in &passes {
            let pa = analyze_pass(&list[a..b]);
            if pa.n_floor_runs > 0 && pa.n_plateau_runs > 0 {
                agg.n_dual += 1;
                agg.floor_runs += pa.n_floor_runs;
                agg.at_pass_start += pa.at_pass_start;
                agg.arcstart += pa.arcstart;
                agg.pre_short += pa.pre_short;
                agg.usable += pa.usable;
                agg.onsets.extend(pa.onsets);
                agg.profile.extend(pa.profile);
            }
        }
        let pre: Vec<f64> = agg.onsets.iter().map(|o| o.pre_rms).collect();
        let post: Vec<f64> = agg.onsets.iter().map(|o| o.post_rms).collect();
        let delta: Vec<f64> = agg.onsets.iter().map(|o| o.post_rms - o.pre_rms).collect();
        let gt = agg.onsets.iter().filter(|o| o.post_rms > o.pre_rms).count();
        out.push(format!(
            "  {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            station,
            list.len(),
            n_lock_st,
            passes.len(),
            agg.n_dual,
            agg.floor_runs,
            agg.at_pass_start,
            agg.arcstart,
            agg.pre_short,
            agg.usable,
            fmt_o(median(&pre)),
            fmt_o(median(&post)),
            fmt_o(median(&delta)),
            if agg.onsets.is_empty() {
                "-".to_string()
            } else {
                format!("{:.3}", gt as f64 / agg.onsets.len() as f64)
            }
        ));
        pooled_pre.extend(pre);
        pooled_post.extend(post);
        pooled_delta.extend(delta);
        pooled_profile.extend(agg.profile);

        if agg.onsets.is_empty() {
            out.push(format!("  station {station}: no usable floor onset inside a dual pass — the measured limit; {} floor runs reach >= 30 samples, of which {} begin at the pass's first tracked sample and {} at an arc start (no adjacent pre-onset window)", agg.floor_runs, agg.at_pass_start, agg.arcstart));
        } else {
            out.push(format!("  station {station}: usable onset detail (date t pre_n post_n pre_rms post_rms delta pre_plateau_frac)"));
            for o in &agg.onsets {
                out.push(format!(
                    "    {} {:.0} {} {} {:.3} {:.3} {:.3} {:.3}",
                    o.date,
                    o.t,
                    o.pre_n,
                    o.post_n,
                    o.pre_rms,
                    o.post_rms,
                    o.post_rms - o.pre_rms,
                    o.pre_plat_frac
                ));
            }
        }
    }
    out.push(String::new());

    out.push("pooled noise profile vs onset (window = 30 tracked samples starting at the offset; median RMS across usable onsets)".to_string());
    out.push("  offset  n  med_rms".to_string());
    for off in [-90i64, -60, -30, 0, 30, 60, 90] {
        let vals: Vec<f64> = pooled_profile
            .iter()
            .filter(|(o, _)| *o == off)
            .map(|(_, v)| *v)
            .collect();
        out.push(format!(
            "  {:>5}  {}  {}",
            off,
            vals.len(),
            fmt_o(median(&vals))
        ));
    }
    out.push(String::new());

    out.push("pooled pre vs post (both stations)".to_string());
    let gt = pooled_delta.iter().filter(|d| **d > 0.0).count();
    let mut ratio_v: Vec<f64> = Vec::new();
    for (a, b) in pooled_pre.iter().zip(&pooled_post) {
        if *a > 0.0 {
            ratio_v.push(b / a);
        }
    }
    out.push(format!(
        "  usable onsets {} | med_pre {} med_post {} | med_delta {} mean_delta {} | post>pre {} frac {:.3} | med_ratio {}",
        pooled_delta.len(),
        fmt_o(median(&pooled_pre)),
        fmt_o(median(&pooled_post)),
        fmt_o(median(&pooled_delta)),
        fmt_o(if pooled_delta.is_empty() { None } else { Some(pooled_delta.iter().sum::<f64>() / pooled_delta.len() as f64) }),
        gt,
        if pooled_delta.is_empty() { 0.0 } else { gt as f64 / pooled_delta.len() as f64 },
        fmt_o(median(&ratio_v))
    ));

    let body = out.join("\n") + "\n";
    println!("{body}");
}
