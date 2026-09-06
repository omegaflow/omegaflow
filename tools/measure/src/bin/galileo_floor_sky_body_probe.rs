use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;

use omegaflow::archivar::{BodyEphemeris, body_barycenter_position, parse_ephemeris_binary};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const LOUD_HZ: f64 = 1.0;
const ROBUST_N: usize = 30;
const AU_M: f64 = 1.495978707e11;
const R_J_M: f64 = 7.1492e7;
const MIN_RANK_N: usize = 8;
const TRIO: [i64; 3] = [14, 43, 63];

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn unit(v: [f64; 3]) -> Option<[f64; 3]> {
    let n = norm(v);
    if n > 0.0 && n.is_finite() {
        Some([v[0] / n, v[1] / n, v[2] / n])
    } else {
        None
    }
}
fn angle_deg(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let na = norm(a);
    let nb = norm(b);
    if na > 0.0 && nb > 0.0 && na.is_finite() && nb.is_finite() {
        Some((dot(a, b) / (na * nb)).clamp(-1.0, 1.0).acos().to_degrees())
    } else {
        None
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
fn lo_hi(vals: &[f64]) -> Option<(f64, f64)> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    Some((s[0], s[s.len() - 1]))
}
fn q_low(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let mut s = vals.to_vec();
    s.sort_by(f64::total_cmp);
    let idx = (((s.len() - 1) as f64) * 0.25).floor() as usize;
    Some(s[idx])
}
fn rms_of(sum: f64, sum2: f64, n: usize) -> f64 {
    if n == 0 {
        return f64::NAN;
    }
    let m = sum / n as f64;
    let v = (sum2 / n as f64 - m * m).max(0.0);
    v.sqrt()
}
fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < MIN_RANK_N || n != y.len() {
        return None;
    }
    let mut xi: Vec<usize> = (0..n).collect();
    let mut yi: Vec<usize> = (0..n).collect();
    xi.sort_by(|a, b| x[*a].total_cmp(&x[*b]));
    yi.sort_by(|a, b| y[*a].total_cmp(&y[*b]));
    let mut rx = vec![0.0f64; n];
    let mut ry = vec![0.0f64; n];
    let mut i = 0usize;
    while i < n {
        let mut j = i + 1;
        while j < n && x[xi[j]] == x[xi[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in xi[i..j].iter() {
            rx[*k] = avg;
        }
        i = j;
    }
    i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && y[yi[j]] == y[yi[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in yi[i..j].iter() {
            ry[*k] = avg;
        }
        i = j;
    }
    let mx = rx.iter().sum::<f64>() / n as f64;
    let my = ry.iter().sum::<f64>() / n as f64;
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for k in 0..n {
        let a = rx[k] - mx;
        let b = ry[k] - my;
        num += a * b;
        dx2 += a * a;
        dy2 += b * b;
    }
    if dx2 > 0.0 && dy2 > 0.0 {
        Some(num / (dx2 * dy2).sqrt())
    } else {
        None
    }
}
fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    std::fs::read(format!("data/ssd.jpl.nasa.gov/ephemeris_{name}.bin"))
        .ok()
        .and_then(|d| parse_ephemeris_binary(&d))
        .map(|e| eph.insert(name.to_string(), e))
        .is_some()
}
fn unix_day(tdb: f64) -> i64 {
    (tdb / DAY_S + 10957.5).round() as i64
}
fn date_of(tdb: f64) -> (i64, i64, i64) {
    match civil_from_days(unix_day(tdb)) {
        Some((y, m, d)) => (y as i64, m as i64, d as i64),
        None => (0, 0, 0),
    }
}
fn fmt_date(tdb: f64) -> String {
    let (y, m, d) = date_of(tdb);
    format!("{y:04}-{m:02}-{d:02}")
}
fn fmt_daycell(day: i64) -> String {
    fmt_date(day as f64 * DAY_S)
}
fn ym_of_daycell(day: i64) -> (i64, i64) {
    let (y, m, _) = date_of(day as f64 * DAY_S);
    (y, m)
}
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}
fn anchor_daycell(y: i64, m: i64, d: i64) -> i64 {
    days_from_civil(y, m, d) - 10958
}

#[derive(Clone, Copy)]
struct Cell {
    mode: i64,
    st: i64,
    day: i64,
    n: usize,
    rms: f64,
}

#[derive(Clone, Copy)]
struct DayGeo {
    r_e_au: Option<f64>,
    jrj: Option<f64>,
    ra: Option<f64>,
    dec: Option<f64>,
    u: Option<[f64; 3]>,
}

struct SkyStat {
    r: f64,
    cx: f64,
    cy: f64,
    cz: f64,
    mean_deg: f64,
    max_deg: f64,
}

fn sky_stat(us: &[[f64; 3]]) -> Option<SkyStat> {
    if us.is_empty() {
        return None;
    }
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sz = 0.0;
    for u in us {
        sx += u[0];
        sy += u[1];
        sz += u[2];
    }
    let n = us.len() as f64;
    let mx = sx / n;
    let my = sy / n;
    let mz = sz / n;
    let r = norm([mx, my, mz]);
    let Some(c) = unit([mx, my, mz]) else {
        return None;
    };
    let mut mean_d = 0.0;
    let mut max_d = 0.0;
    for u in us {
        let a = (dot(*u, c)).clamp(-1.0, 1.0).acos().to_degrees();
        mean_d += a;
        if a > max_d {
            max_d = a;
        }
    }
    Some(SkyStat {
        r,
        cx: c[0],
        cy: c[1],
        cz: c[2],
        mean_deg: mean_d / n,
        max_deg: max_d,
    })
}
fn ra_dec(u: [f64; 3]) -> (f64, f64) {
    let ra = {
        let r = u[1].atan2(u[0]).to_degrees();
        if r < 0.0 {
            r + 360.0
        } else {
            r
        }
    };
    let dec = u[2].clamp(-1.0, 1.0).asin().to_degrees();
    (ra, dec)
}
fn fmt_opt(v: Option<f64>, w: usize, prec: usize) -> String {
    match v {
        Some(x) => format!("{x:>w$.p$}", x = x, w = w, p = prec),
        None => format!("{:>w$}", "-", w = w),
    }
}
fn fmt_frac(n: usize, d: usize) -> f64 {
    if d == 0 {
        0.0
    } else {
        100.0 * n as f64 / d as f64
    }
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_floor_sky_body_report.txt".to_string(),
    };

    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth", "jupiter"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
        }
    }
    let Ok(bytes) = fs::read("data/pds-ppi.igpp.ucla.edu/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut cell: BTreeMap<(i64, i64, i64), (f64, f64, usize)> = BTreeMap::new();
    let mut n_total = 0usize;
    let mut n_lock = 0usize;
    for r in &recs {
        n_total += 1;
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            n_lock += 1;
            continue;
        }
        let mode = r[3] as i64;
        if mode < 1 || mode > 3 {
            continue;
        }
        let st = r[2] as i64;
        if !TRIO.contains(&st) {
            continue;
        }
        if r[7] as i64 != FLOOR {
            continue;
        }
        let day = (r[0] / DAY_S).floor() as i64;
        let e = cell.entry((mode, st, day)).or_insert((0.0, 0.0, 0));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
    }
    drop(recs);

    let mut rows: Vec<Cell> = Vec::new();
    for (&(mode, st, day), &(sum, sum2, n)) in &cell {
        rows.push(Cell {
            mode,
            st,
            day,
            n,
            rms: rms_of(sum, sum2, n),
        });
    }
    rows.sort_by_key(|c| (c.mode, c.st, c.day));

    let era_min = rows.iter().map(|c| c.day).min();
    let era_max = rows.iter().map(|c| c.day).max();
    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String| {
        println!("{s}");
        out.push(s);
    };

    push("galileo floor sky & body probe".to_string());
    push("target: the station-bound floor peak days (robust floor cells, n >= 30, RMS >= 1 Hz) per (mode, station) against sky line position and body distance".to_string());
    push("binding: floor = strength == -2560; cell = (mode, station, tdb day); lock (|resid| > 1000 Hz) and non-finite excluded; cell RMS about the cell mean; robust day = cell n >= 30; loud = cell RMS >= 1 Hz".to_string());
    push("geometry at the TDB day start from galileo_daily / earth / jupiter barycentric ICRS (m): u = unit(galileo - earth) sight line; rE AU = |galileo - earth|; J RJ = |galileo - jupiter| / 71492 km; RA/Dec of u ICRS".to_string());
    let era_min_s = match era_min {
        Some(d) => fmt_daycell(d),
        None => "-".to_string(),
    };
    let era_max_s = match era_max {
        Some(d) => fmt_daycell(d),
        None => "-".to_string(),
    };
    push(format!(
        "samples {n_total}; lock/non-finite excluded {n_lock}; floor trio cells {} over era {era_min_s} .. {era_max_s}",
        rows.len()
    ));

    let mut state: BTreeMap<(i64, i64, i64), u8> = BTreeMap::new();
    for c in &rows {
        let s = if c.n >= ROBUST_N {
            if c.rms >= LOUD_HZ {
                3u8
            } else {
                2u8
            }
        } else {
            1u8
        };
        state.insert((c.mode, c.st, c.day), s);
    }
    let mut n_thin = 0usize;
    let mut n_robust = 0usize;
    let mut n_loud = 0usize;
    for v in state.values() {
        match v {
            1 => n_thin += 1,
            2 => n_robust += 1,
            _ => {
                n_robust += 1;
                n_loud += 1;
            }
        }
    }
    push(format!(
        "floor trio cells thin (1..29 samples) {n_thin}; robust (>=30) {n_robust}; loud of robust {n_loud}"
    ));

    let (Some(d0), Some(d1)) = (era_min, era_max) else {
        push("no floor cells over the trio (0 honored)".to_string());
        let _ = fs::write(&report_path, out.join("\n") + "\n");
        return;
    };
    let mut geo_ok = 0usize;
    let mut geo_missing = 0usize;
    let mut day_geo: BTreeMap<i64, DayGeo> = BTreeMap::new();
    let mut j_missing = 0usize;
    let mut d = d0;
    while d <= d1 {
        let t = d as f64 * DAY_S;
        let gp = body_barycenter_position("galileo_daily", t, &eph);
        let ep = body_barycenter_position("earth", t, &eph);
        let jp = body_barycenter_position("jupiter", t, &eph);
        let mut g = DayGeo {
            r_e_au: None,
            jrj: None,
            ra: None,
            dec: None,
            u: None,
        };
        if let (Some(gp), Some(ep)) = (gp, ep) {
            let e_to_p = sub(gp, ep);
            let r_e = norm(e_to_p);
            if let Some(u) = unit(e_to_p) {
                let (ra, dec) = ra_dec(u);
                g.r_e_au = Some(r_e / AU_M);
                g.ra = Some(ra);
                g.dec = Some(dec);
                g.u = Some(u);
            }
        }
        if let (Some(gp), Some(jp)) = (gp, jp) {
            let jau = norm(sub(gp, jp)) / AU_M;
            g.jrj = Some(jau * AU_M / R_J_M);
        }
        if g.u.is_some() && g.jrj.is_some() {
            geo_ok += 1;
        } else {
            geo_missing += 1;
            if g.jrj.is_none() {
                j_missing += 1;
            }
        }
        day_geo.insert(d, g);
        d += 1;
    }
    push(format!(
        "era day geometry: {geo_ok} days full (galileo+earth+jupiter), {geo_missing} partial/absent (jupiter absent on {j_missing} days; 0 honored)"
    ));

    let modes = [1i64, 2, 3];
    let mut series_data: Vec<(i64, i64, Vec<(i64, f64)>)> = Vec::new();
    for m in modes {
        for st in TRIO {
            let mut srows: Vec<(i64, f64)> = rows
                .iter()
                .filter(|c| c.mode == m && c.st == st && c.n >= ROBUST_N)
                .map(|c| (c.day, c.rms))
                .collect();
            srows.sort_by_key(|x| x.0);
            series_data.push((m, st, srows));
        }
    }

    push(String::new());
    push("== n first per (mode, station): robust floor day series ==".to_string());
    for (m, st, srows) in &series_data {
        let loud = srows.iter().filter(|x| x.1 >= LOUD_HZ).count();
        let quiet = srows.len() - loud;
        push(format!(
            "mode {m} st{st}: robust days n {} (loud {loud}, quiet {quiet})",
            srows.len()
        ));
    }

    for (m, st, srows) in &series_data {
        let loud_days: Vec<i64> = srows.iter().filter(|x| x.1 >= LOUD_HZ).map(|x| x.0).collect();
        let quiet_days: Vec<i64> = srows.iter().filter(|x| x.1 < LOUD_HZ).map(|x| x.0).collect();
        push(String::new());
        push(format!("== mode {m} st{st}: loud vs quiet day sky + distance =="));
        let lus: Vec<[f64; 3]> = loud_days
            .iter()
            .filter_map(|d| day_geo.get(d).and_then(|g| g.u))
            .collect();
        let qus: Vec<[f64; 3]> = quiet_days
            .iter()
            .filter_map(|d| day_geo.get(d).and_then(|g| g.u))
            .collect();
        let all_u: Vec<[f64; 3]> = srows
            .iter()
            .filter_map(|x| day_geo.get(&x.0).and_then(|g| g.u))
            .collect();
        let ls = sky_stat(&lus);
        let qs = sky_stat(&qus);
        let as_ = sky_stat(&all_u);
        let fmt_sky = |n: usize, s: &Option<SkyStat>| -> String {
            match s {
                Some(x) => {
                    let (ra, dec) = ra_dec([x.cx, x.cy, x.cz]);
                    format!(
                        "n {:>2} R {:.3} centroid RA {:6.1} Dec {:6.1} mean-dist {:5.1} max-dist {:5.1} deg",
                        n, x.r, ra, dec, x.mean_deg, x.max_deg
                    )
                }
                None => "n 0 (absent)".to_string(),
            }
        };
        push(format!("  loud  sky: {}", fmt_sky(lus.len(), &ls)));
        push(format!("  quiet sky: {}", fmt_sky(qus.len(), &qs)));
        push(format!("  all   sky: {}", fmt_sky(all_u.len(), &as_)));
        let sep = match (&ls, &qs) {
            (Some(a), Some(b)) => angle_deg([a.cx, a.cy, a.cz], [b.cx, b.cy, b.cz]),
            _ => None,
        };
        push(format!(
            "  loud/quiet centroid separation: {} deg",
            fmt_opt(sep, 8, 1)
        ));
        let med_lo_hi = |days: &[i64], sel: fn(&DayGeo) -> Option<f64>| -> (usize, Option<f64>, Option<(f64, f64)>) {
            let vals: Vec<f64> = days
                .iter()
                .filter_map(|d| day_geo.get(d).and_then(|g| sel(g)))
                .collect();
            (vals.len(), median(&vals), lo_hi(&vals))
        };
        let (nl, lm, lr) = med_lo_hi(&loud_days, |g| g.r_e_au);
        let (nq, qm, qr) = med_lo_hi(&quiet_days, |g| g.r_e_au);
        let (nlj, ljm, ljr) = med_lo_hi(&loud_days, |g| g.jrj);
        let (nqj, qjm, qjr) = med_lo_hi(&quiet_days, |g| g.jrj);
        push(format!(
            "  rE AU: loud med {} [{} .. {}] n {} | quiet med {} [{} .. {}] n {}",
            fmt_opt(lm, 7, 3),
            fmt_opt(lr.map(|x| x.0), 7, 3),
            fmt_opt(lr.map(|x| x.1), 7, 3),
            nl,
            fmt_opt(qm, 7, 3),
            fmt_opt(qr.map(|x| x.0), 7, 3),
            fmt_opt(qr.map(|x| x.1), 7, 3),
            nq
        ));
        push(format!(
            "  J RJ:   loud med {} [{} .. {}] n {} | quiet med {} [{} .. {}] n {}",
            fmt_opt(ljm, 8, 1),
            fmt_opt(ljr.map(|x| x.0), 8, 1),
            fmt_opt(ljr.map(|x| x.1), 8, 1),
            nlj,
            fmt_opt(qjm, 8, 1),
            fmt_opt(qjr.map(|x| x.0), 8, 1),
            fmt_opt(qjr.map(|x| x.1), 8, 1),
            nqj
        ));
        let j_all: Vec<f64> = srows
            .iter()
            .filter_map(|x| day_geo.get(&x.0).and_then(|g| g.jrj))
            .collect();
        let thr = q_low(&j_all);
        let loud_j: Vec<f64> = loud_days
            .iter()
            .filter_map(|d| day_geo.get(d).and_then(|g| g.jrj))
            .collect();
        let quiet_j: Vec<f64> = quiet_days
            .iter()
            .filter_map(|d| day_geo.get(d).and_then(|g| g.jrj))
            .collect();
        let cnt_le = |v: &[f64], t: f64| v.iter().filter(|x| **x <= t).count();
        match thr {
            Some(t) => {
                let cl = cnt_le(&loud_j, t);
                let cq = cnt_le(&quiet_j, t);
                push(format!(
                    "  robust-day J quartile Q25 = {t:.1} RJ; loud <= Q25 {cl}/{nlj} ({:.0}%), quiet <= Q25 {cq}/{nqj} ({:.0}%)",
                    fmt_frac(cl, nlj),
                    fmt_frac(cq, nqj)
                ));
            }
            None => push("  robust-day J quartile absent (no geometry)".to_string()),
        }
        let mut lrms = Vec::new();
        let mut lj = Vec::new();
        let mut lre = Vec::new();
        let mut ldy = Vec::new();
        for (day, rms_v) in srows {
            if *rms_v > 0.0 && rms_v.is_finite() {
                lrms.push(rms_v.log10());
                ldy.push(*day as f64);
                if let Some(g) = day_geo.get(day) {
                    if let Some(jrj) = g.jrj {
                        lj.push(jrj);
                    }
                    if let Some(r_e) = g.r_e_au {
                        lre.push(r_e);
                    }
                }
            }
        }
        let mk = |x: &[f64], y: &[f64]| -> String {
            if x.len() == y.len() {
                match spearman(x, y) {
                    Some(v) => format!("{v:+.2}"),
                    None => "-".to_string(),
                }
            } else {
                "-".to_string()
            }
        };
        push(format!(
            "  spearman log10(dayRMS) vs day {} | vs J RJ {} | vs rE AU {}  (n {})",
            mk(&ldy, &lrms),
            mk(&lj, &lrms),
            mk(&lre, &lrms),
            lrms.len()
        ));
        let mut mhist: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        for d in &loud_days {
            *mhist.entry(ym_of_daycell(*d)).or_insert(0) += 1;
        }
        let mh: Vec<String> = mhist
            .iter()
            .map(|((y, m), n)| format!("{y:04}-{m:02}:{n}"))
            .collect();
        push(format!("  loud day months (era cells): {}", mh.join(" ")));
        push(format!("  loud day geometry rows ({} days):", loud_days.len()));
        for d in &loud_days {
            let g = day_geo.get(d);
            let (r_e_s, ra_s, dec_s, j_s) = match g {
                Some(gg) => (
                    fmt_opt(gg.r_e_au, 7, 3),
                    fmt_opt(gg.ra, 7, 1),
                    fmt_opt(gg.dec, 7, 1),
                    fmt_opt(gg.jrj, 9, 1),
                ),
                None => ("-".to_string(), "-".to_string(), "-".to_string(), "-".to_string()),
            };
            push(format!(
                "    {} rE {} AU RA {} Dec {} J {} RJ",
                fmt_daycell(*d),
                r_e_s,
                ra_s,
                dec_s,
                j_s
            ));
        }
        let mut steps_sky: Vec<f64> = Vec::new();
        let mut steps_re: Vec<f64> = Vec::new();
        let mut steps_j: Vec<f64> = Vec::new();
        let mut nflip = 0usize;
        for w in srows.windows(2) {
            let a = w[0];
            let b = w[1];
            if b.0 - a.0 != 1 {
                continue;
            }
            let la = a.1 >= LOUD_HZ;
            let lb = b.1 >= LOUD_HZ;
            if la == lb {
                continue;
            }
            nflip += 1;
            let ga = day_geo.get(&a.0);
            let gb = day_geo.get(&b.0);
            if let (Some(ga), Some(gb)) = (ga, gb) {
                if let (Some(ua), Some(ub)) = (ga.u, gb.u) {
                    if let Some(sa) = angle_deg(ua, ub) {
                        steps_sky.push(sa);
                    }
                }
                if let (Some(xa), Some(xb)) = (ga.r_e_au, gb.r_e_au) {
                    steps_re.push((xb - xa).abs());
                }
                if let (Some(xa), Some(xb)) = (ga.jrj, gb.jrj) {
                    steps_j.push((xb - xa).abs());
                }
            }
        }
        let max_of = |v: &[f64]| -> Option<f64> {
            v.iter().copied().fold(None, |a: Option<f64>, x| match a { Some(m) => Some(m.max(x)), None => Some(x) })
        };
        push(format!(
            "  flip count {nflip}; sky step deg med {} max {}; |d rE| AU med {} max {}; |d J| RJ med {} max {}",
            fmt_opt(median(&steps_sky), 6, 2),
            fmt_opt(max_of(&steps_sky), 6, 2),
            fmt_opt(median(&steps_re), 6, 4),
            fmt_opt(max_of(&steps_re), 6, 4),
            fmt_opt(median(&steps_j), 8, 1),
            fmt_opt(max_of(&steps_j), 8, 1)
        ));
    }

    push(String::new());
    push("== pooled flip step over all robust series ==".to_string());
    let mut all_sky: Vec<f64> = Vec::new();
    let mut all_re: Vec<f64> = Vec::new();
    let mut all_j: Vec<f64> = Vec::new();
    let mut all_count = 0usize;
    let mut lt1 = 0usize;
    let mut lt2 = 0usize;
    let mut lt5 = 0usize;
    for (_, _, srows) in &series_data {
        for w in srows.windows(2) {
            let a = w[0];
            let b = w[1];
            if b.0 - a.0 != 1 {
                continue;
            }
            if (a.1 >= LOUD_HZ) == (b.1 >= LOUD_HZ) {
                continue;
            }
            all_count += 1;
            let ga = day_geo.get(&a.0);
            let gb = day_geo.get(&b.0);
            if let (Some(ga), Some(gb)) = (ga, gb) {
                if let (Some(ua), Some(ub)) = (ga.u, gb.u) {
                    if let Some(sa) = angle_deg(ua, ub) {
                        all_sky.push(sa);
                        if sa < 1.0 {
                            lt1 += 1;
                        }
                        if sa < 2.0 {
                            lt2 += 1;
                        }
                        if sa < 5.0 {
                            lt5 += 1;
                        }
                    }
                }
                if let (Some(xa), Some(xb)) = (ga.r_e_au, gb.r_e_au) {
                    all_re.push((xb - xa).abs());
                }
                if let (Some(xa), Some(xb)) = (ga.jrj, gb.jrj) {
                    all_j.push((xb - xa).abs());
                }
            }
        }
    }
    let max_of = |v: &[f64]| -> Option<f64> {
        v.iter().copied().fold(None, |a: Option<f64>, x| match a { Some(m) => Some(m.max(x)), None => Some(x) })
    };
    push(format!(
        "flips total {all_count}; sky step deg med {} max {} (<1 deg {lt1}, <2 deg {lt2}, <5 deg {lt5}); |d rE| AU med {}; |d J| RJ med {}",
        fmt_opt(median(&all_sky), 6, 2),
        fmt_opt(max_of(&all_sky), 6, 2),
        fmt_opt(median(&all_re), 6, 4),
        fmt_opt(median(&all_j), 8, 1)
    ));

    push(String::new());
    push("== same-day station contrast (one geometry shared by all stations per day) ==".to_string());
    for m in modes {
        let mut all_loud = 0usize;
        let mut all_quiet = 0usize;
        let mut mixed_n = 0usize;
        let mut ex: Vec<String> = Vec::new();
        let days: BTreeSet<i64> = rows
            .iter()
            .filter(|c| c.mode == m && c.n >= ROBUST_N)
            .map(|c| c.day)
            .collect();
        for day in &days {
            let mut loud_st = 0usize;
            let mut quiet_st = 0usize;
            let mut present = 0usize;
            for st in TRIO {
                match state.get(&(m, st, *day)) {
                    Some(3) => {
                        loud_st += 1;
                        present += 1;
                    }
                    Some(2) => {
                        quiet_st += 1;
                        present += 1;
                    }
                    _ => {}
                }
            }
            if present < 2 {
                continue;
            }
            if loud_st > 0 && quiet_st > 0 {
                mixed_n += 1;
                if ex.len() < 6 {
                    let parts: Vec<String> = TRIO
                        .iter()
                        .filter_map(|st| match state.get(&(m, *st, *day)) {
                            Some(3) => Some(format!("st{st} LOUD")),
                            Some(2) => Some(format!("st{st} quiet")),
                            _ => None,
                        })
                        .collect();
                    ex.push(format!("    {} {}", fmt_daycell(*day), parts.join(" | ")));
                }
            } else if loud_st == present {
                all_loud += 1;
            } else {
                all_quiet += 1;
            }
        }
        push(format!(
            "mode {m}: shared robust days n {} — all loud {all_loud}, all quiet {all_quiet}, mixed loud+quiet at one geometry {mixed_n}",
            all_loud + all_quiet + mixed_n
        ));
        for l in &ex {
            push(l.clone());
        }
    }

    push(String::new());
    push("== jupiter-distance profile over the floor era: monthly minimum (perijove proxy), daily raster ==".to_string());
    let mut monthly_min: BTreeMap<(i64, i64), (i64, f64)> = BTreeMap::new();
    let mut dd = d0;
    while dd <= d1 {
        if let Some(g) = day_geo.get(&dd) {
            if let Some(jrj) = g.jrj {
                let ym = ym_of_daycell(dd);
                let upd = match monthly_min.get(&ym) {
                    Some((_, v)) => jrj < *v,
                    None => true,
                };
                if upd {
                    monthly_min.insert(ym, (dd, jrj));
                }
            }
        }
        dd += 1;
    }
    for ((y, m), (day, v)) in &monthly_min {
        push(format!(
            "  {y:04}-{m:02}: min J {v:9.1} RJ on {}",
            fmt_daycell(*day)
        ));
    }

    push(String::new());
    push("== 1996-06-20 .. 1996-07-06 window: per-day geometry and floor state (L robust loud, q robust quiet, . thin floor, blank absent) ==".to_string());
    let w0 = anchor_daycell(1996, 6, 20);
    let w1 = anchor_daycell(1996, 7, 6);
    let mut wd = w0;
    while wd <= w1 {
        let mut line = format!("  {} ", fmt_daycell(wd));
        if let Some(g) = day_geo.get(&wd) {
            line.push_str(&format!(
                "rE {} AU RA {} Dec {} J {} RJ",
                fmt_opt(g.r_e_au, 7, 3),
                fmt_opt(g.ra, 7, 1),
                fmt_opt(g.dec, 7, 1),
                fmt_opt(g.jrj, 9, 1)
            ));
        } else {
            line.push_str("geometry absent");
        }
        for (m, st, _) in &series_data {
            let ch = match state.get(&(*m, *st, wd)) {
                Some(3) => 'L',
                Some(2) => 'q',
                Some(1) => '.',
                _ => ' ',
            };
            line.push_str(&format!("  m{m}st{st}:{ch}"));
        }
        push(line);
        wd += 1;
    }

    push(String::new());
    push("== 1996-06 window jupiter-distance at half-day sampling (galileo_daily is a 1-day-raster Chebyshev fit; half-day values are fit interpolations) ==".to_string());
    let mut best: Option<(f64, f64)> = None;
    let mut sd = (w0 - 2) as f64;
    let s_end = (w1 + 2) as f64;
    while sd <= s_end {
        let t = sd * DAY_S;
        if let (Some(gp), Some(jp)) = (
            body_barycenter_position("galileo_daily", t, &eph),
            body_barycenter_position("jupiter", t, &eph),
        ) {
            let jrj = norm(sub(gp, jp)) / AU_M * AU_M / R_J_M;
            let is_min = match best {
                Some((_, v)) => jrj < v,
                None => true,
            };
            if is_min {
                best = Some((sd, jrj));
            }
        }
        sd += 0.5;
    }
    match best {
        Some((dayf, v)) => push(format!(
            "  window minimum J {v:.1} RJ at day {:.1} ({})",
            dayf,
            fmt_date(dayf * DAY_S)
        )),
        None => push("  window jupiter distance absent".to_string()),
    }

    push(String::new());
    push("== loud day union across series: unique dates, geometry, loud-series list ==".to_string());
    let mut by_day: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    for (m, st, srows) in &series_data {
        for (day, rms_v) in srows {
            if *rms_v >= LOUD_HZ {
                by_day.entry(*day).or_default().push((*m, *st));
            }
        }
    }
    let mut both_loud_quiet = 0usize;
    for (day, lst) in &by_day {
        let mut quiet_on_day = 0usize;
        for (m, st, _) in &series_data {
            if matches!(state.get(&(*m, *st, *day)), Some(2)) {
                quiet_on_day += 1;
            }
        }
        if quiet_on_day > 0 {
            both_loud_quiet += 1;
        }
        let who: Vec<String> = lst.iter().map(|(m, st)| format!("m{m}st{st}")).collect();
        let g = day_geo.get(day);
        let (r_e_s, j_s) = match g {
            Some(gg) => (fmt_opt(gg.r_e_au, 7, 3), fmt_opt(gg.jrj, 9, 1)),
            None => ("-".to_string(), "-".to_string()),
        };
        push(format!(
            "  {} rE {} AU J {} RJ | loud {}",
            fmt_daycell(*day),
            r_e_s,
            j_s,
            who.join(" ")
        ));
    }
    push(format!(
        "unique loud dates {}; of these with at least one quiet robust series on the same day: {both_loud_quiet}",
        by_day.len()
    ));

    let _ = fs::write(&report_path, out.join("\n") + "\n");
    eprintln!("galileo: floor sky/body report written to {report_path}");
}
