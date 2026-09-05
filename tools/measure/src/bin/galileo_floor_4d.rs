use std::collections::{BTreeMap, BTreeSet, HashMap};

use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};
use omegaflow::spectral::civil_from_days;

const DAY_S: f64 = 86400.0;
const LOCK_HZ: f64 = 1.0e3;
const FLOOR: i64 = -2560;
const STRONG_MIN: i64 = -1750;
const LOUD_HZ: f64 = 1.0;
const AU_M: f64 = 1.495978707e11;
const PSD_LOG_MIN: f64 = -5.0;
const PSD_NBIN: usize = 64;
const PSD_BINW: f64 = 0.1;
const MIN_RUN: usize = 256;
const MAX_CHUNK: usize = 65536;
const MIN_BIN_N: usize = 20;

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn angle(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let na = norm(a);
    let nb = norm(b);
    if na > 0.0 && nb > 0.0 {
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

fn median_usize(vals: &[usize]) -> usize {
    if vals.is_empty() {
        return 0;
    }
    let mut s = vals.to_vec();
    s.sort_unstable();
    s[s.len() / 2]
}

fn fmt_opt(v: Option<usize>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "-".to_string(),
    }
}

fn fmt_level(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:+.2}"),
        None => "-".to_string(),
    }
}

fn load(name: &str, eph: &mut HashMap<String, BodyEphemeris>) -> bool {
    std::fs::read(format!("data/ephemeris_{name}.bin"))
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

fn month_key(tdb: f64) -> i64 {
    let (y, m, _) = date_of(tdb);
    y * 12 + m
}

fn fmt_date(tdb: f64) -> String {
    let (y, m, d) = date_of(tdb);
    format!("{y:04}-{m:02}-{d:02}")
}

fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    if n < 8 || n != y.len() {
        return None;
    }
    let mut xi: Vec<usize> = (0..n).collect();
    let mut yi: Vec<usize> = (0..n).collect();
    xi.sort_by(|a, b| x[*a].total_cmp(&x[*b]));
    yi.sort_by(|a, b| y[*a].total_cmp(&y[*b]));
    let rx = rank(&xi, x);
    let ry = rank(&yi, y);
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

fn rank(ord: &[usize], v: &[f64]) -> Vec<f64> {
    let n = v.len();
    let mut r = vec![0.0f64; n];
    let mut i = 0usize;
    while i < n {
        let mut j = i + 1;
        while j < n && v[ord[j]] == v[ord[i]] {
            j += 1;
        }
        let avg = ((i + j - 1) as f64) / 2.0;
        for k in ord[i..j].iter() {
            r[*k] = avg;
        }
        i = j;
    }
    r
}

#[derive(Clone, Copy)]
struct Cell {
    mode: i64,
    day: i64,
    st: i64,
    floor: bool,
    n: usize,
    rms: f64,
    t0: f64,
}

fn fft(re: &mut [f64], im: &mut [f64]) {
    let n = re.len();
    let mut j = 0usize;
    for i in 0..n {
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
        let mut m = n >> 1;
        while m >= 1 && j >= m {
            j -= m;
            m >>= 1;
        }
        j += m;
    }
    let mut len = 2usize;
    while len <= n {
        let ang = -2.0 * std::f64::consts::PI / (len as f64);
        let wr = ang.cos();
        let wi = ang.sin();
        let mut i = 0usize;
        while i < n {
            let mut cr = 1.0f64;
            let mut ci = 0.0f64;
            for k in 0..len / 2 {
                let ur = re[i + k];
                let ui = im[i + k];
                let vr = re[i + k + len / 2] * cr - im[i + k + len / 2] * ci;
                let vi = re[i + k + len / 2] * ci + im[i + k + len / 2] * cr;
                re[i + k] = ur + vr;
                im[i + k] = ui + vi;
                re[i + k + len / 2] = ur - vr;
                im[i + k + len / 2] = ui - vi;
                let nc = cr * wr - ci * wi;
                ci = cr * wi + ci * wr;
                cr = nc;
            }
            i += len;
        }
        len <<= 1;
    }
}

fn pow2_floor(n: usize) -> usize {
    let mut p = 1usize;
    while p <= n / 2 {
        p <<= 1;
    }
    p
}

fn detrend(x: &[f64], linear: bool) -> Vec<f64> {
    let n = x.len();
    let mut y = x.to_vec();
    if n < 3 || !linear {
        if n >= 3 {
            let m = x.iter().sum::<f64>() / n as f64;
            for v in y.iter_mut() {
                *v -= m;
            }
        }
        return y;
    }
    let nf = n as f64;
    let mt = (nf - 1.0) / 2.0;
    let mx = x.iter().sum::<f64>() / nf;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for (k, v) in x.iter().enumerate() {
        let t = k as f64 - mt;
        sxx += t * t;
        sxy += t * (*v - mx);
    }
    let b = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    for (k, v) in y.iter_mut().enumerate() {
        *v = *v - (mx + b * (k as f64 - mt));
    }
    y
}

struct PsdBins {
    sum_lf: [f64; PSD_NBIN],
    sum_lp: [f64; PSD_NBIN],
    cnt: [usize; PSD_NBIN],
}

impl PsdBins {
    fn new() -> Self {
        PsdBins {
            sum_lf: [0.0; PSD_NBIN],
            sum_lp: [0.0; PSD_NBIN],
            cnt: [0; PSD_NBIN],
        }
    }
    fn record(&mut self, f: f64, p: f64) {
        let lf = f.log10();
        let lp = p.log10();
        if !(lp.is_finite() && lf >= PSD_LOG_MIN) {
            return;
        }
        let idx = (((lf - PSD_LOG_MIN) / PSD_BINW) as usize).min(PSD_NBIN - 1);
        self.sum_lf[idx] += lf;
        self.sum_lp[idx] += lp;
        self.cnt[idx] += 1;
    }
    fn fit(&self) -> (Option<(f64, f64, f64, f64)>, usize, f64, f64) {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for i in 0..PSD_NBIN {
            if self.cnt[i] >= MIN_BIN_N {
                let x = self.sum_lf[i] / self.cnt[i] as f64;
                let y = self.sum_lp[i] / self.cnt[i] as f64;
                xs.push(x);
                ys.push(y);
                lo = lo.min(x);
                hi = hi.max(x);
            }
        }
        let n = xs.len();
        if n < 3 {
            return (None, n, f64::NAN, f64::NAN);
        }
        let nf = n as f64;
        let sx = xs.iter().sum::<f64>();
        let sy = ys.iter().sum::<f64>();
        let sxx = xs.iter().map(|v| v * v).sum::<f64>();
        let sxy = xs.iter().zip(ys.iter()).map(|(a, b)| a * b).sum::<f64>();
        let syy = ys.iter().map(|v| v * v).sum::<f64>();
        let denom = nf * sxx - sx * sx;
        if denom.abs() <= 0.0 {
            return (None, n, f64::NAN, f64::NAN);
        }
        let slope = (nf * sxy - sx * sy) / denom;
        let icpt = (sy - slope * sx) / nf;
        let sse = (syy - slope * sxy - icpt * sy).max(0.0);
        let r2 = 1.0 - sse / (syy - sy * sy / nf).max(1e-300);
        let se = if n > 2 {
            ((sse / (nf - 2.0)).max(0.0) / (sxx - sx * sx / nf).max(1e-300)).sqrt()
        } else {
            f64::NAN
        };
        (
            Some((slope, icpt, r2, se)),
            n,
            10f64.powf(lo),
            10f64.powf(hi),
        )
    }
    fn level_at(&self, f: f64) -> Option<f64> {
        let lf = f.log10();
        if lf < PSD_LOG_MIN {
            return None;
        }
        let idx = (((lf - PSD_LOG_MIN) / PSD_BINW) as usize).min(PSD_NBIN - 1);
        if self.cnt[idx] >= MIN_BIN_N {
            Some(self.sum_lp[idx] / self.cnt[idx] as f64)
        } else {
            None
        }
    }
}

fn add_run_psd(run: &[f64], linear: bool, acc: &mut PsdBins) -> usize {
    let n = run.len();
    let mut n_used = 0usize;
    let mut off = 0usize;
    while off < n {
        let rem = n - off;
        if rem < MIN_RUN {
            break;
        }
        let c = pow2_floor(rem.min(MAX_CHUNK));
        let x = &run[off..off + c];
        let y = detrend(x, linear);
        let mut re = y;
        let mut im = vec![0.0f64; c];
        fft(&mut re, &mut im);
        for j in 1..c / 2 {
            let f = j as f64 / c as f64;
            let p = 2.0 * (re[j] * re[j] + im[j] * im[j]) / c as f64;
            if p > 0.0 {
                acc.record(f, p);
            }
        }
        n_used += c;
        off += c;
    }
    n_used
}

fn runs_of(data: &[(f64, f64)]) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = Vec::new();
    let mut cur: Vec<f64> = Vec::new();
    for (k, &(t, r)) in data.iter().enumerate() {
        if !cur.is_empty() {
            let dt = t - data[k - 1].0;
            if !(dt > 0.5 && dt <= 1.6) {
                out.push(std::mem::take(&mut cur));
            }
        }
        cur.push(r);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn color_line(data: &[(f64, f64)], out: &mut Vec<String>, station: i64, floor: bool, wname: &str) {
    let runs = runs_of(data);
    let lens: Vec<usize> = runs.iter().map(|r| r.len()).collect();
    let n_run = lens.len();
    let n_tot: usize = lens.iter().sum();
    let n_use: usize = lens.iter().filter(|l| **l >= MIN_RUN).sum();
    let tag = format!(
        "st{station} {} {wname}",
        if floor { "floor" } else { "strong" }
    );
    if n_use < 2 * MIN_RUN {
        out.push(format!(
            "  {tag}: {n_tot} samples, {n_run} runs (max len {}), usable ({MIN_RUN}+) {n_use} — below the spectral floor",
            fmt_opt(lens.iter().max().copied())
        ));
        return;
    }
    let mut ad = PsdBins::new();
    let mut am = PsdBins::new();
    for r in &runs {
        if r.len() < MIN_RUN {
            continue;
        }
        add_run_psd(r, true, &mut ad);
        add_run_psd(r, false, &mut am);
    }
    let (fd, nd, lo_d, hi_d) = ad.fit();
    let (fm, nm, _, _) = am.fit();
    let lvl01 = fmt_level(ad.level_at(0.01));
    let lvl1 = fmt_level(ad.level_at(0.1));
    match (fd, fm) {
        (Some((sd, _, rd, se_d)), Some((sm, _, _, _))) => {
            let med = median_usize(&lens);
            let mx = fmt_opt(lens.iter().max().copied());
            out.push(format!(
                "  {tag}: samples {n_tot} runs {n_run} (len med {med} max {mx}) | detrend slope {sd:+.3} ± {se_d:.3} (r2 {rd:.2}, bins {nd}, band {lo_d:.5}..{hi_d:.3} Hz) | mean-subtract slope {sm:+.3} (bins {nm}) | log10 PSD @0.01 Hz {lvl01} @0.1 Hz {lvl1}"
            ));
        }
        _ => out.push(format!(
            "  {tag}: {n_tot} samples, {n_run} runs — PSD bins below the count floor"
        )),
    }
}

fn cell_line(c: &Cell, geo: &BTreeMap<i64, (f64, f64, f64, [f64; 3])>) -> String {
    let g = match geo.get(&c.day) {
        Some(&(r, ep, al, p)) => format!(
            " r {r:.3} AU  eps {ep:6.1}  alpha {al:6.1}  ICRS x {:.3} y {:.3} z {:.3} AU",
            p[0], p[1], p[2]
        ),
        None => " r --  eps --  alpha --".to_string(),
    };
    format!(
        "{} mode {} st{} {} n {:<7} RMS {:10.4} Hz  {}",
        fmt_date(c.t0),
        c.mode,
        c.st,
        if c.floor { "floor " } else { "strong" },
        c.n,
        c.rms,
        g
    )
}

fn envelope_rows(
    rows: &[Cell],
    mode: i64,
    geo: &BTreeMap<i64, (f64, f64, f64, [f64; 3])>,
) -> Vec<String> {
    let mrows: Vec<&Cell> = rows.iter().filter(|c| c.mode == mode).collect();
    let mut sec = Vec::new();
    sec.push(String::new());
    sec.push(format!("== mode {mode} form envelopes =="));

    sec.push(
        "  r-bin (AU): floor cells/days/med-RMS/loud | strong cells/days/med-RMS/loud".to_string(),
    );
    for lo in 0..14 {
        let a = lo as f64 * 0.5;
        let b = a + 0.5;
        let mut line = format!("  r {a:.1}..{b:.1}: ");
        for floor in [true, false] {
            let name = if floor { "floor" } else { "strong" };
            let sub: Vec<&&Cell> = mrows
                .iter()
                .filter(|c| {
                    c.floor == floor
                        && geo
                            .get(&c.day)
                            .map(|g| g.0 >= a && g.0 < b)
                            .unwrap_or(false)
                })
                .collect();
            if sub.is_empty() {
                line.push_str(&format!("{name} - | "));
                continue;
            }
            let days: BTreeSet<i64> = sub.iter().map(|c| c.day).collect();
            let rms_v: Vec<f64> = sub.iter().map(|c| c.rms).collect();
            let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
            line.push_str(&format!(
                "{name} {}c/{}d/{:.3}Hz/{loud}l | ",
                sub.len(),
                days.len(),
                median(&rms_v).unwrap_or(f64::NAN)
            ));
        }
        sec.push(line);
    }

    sec.push(
        "  eps-bin (deg): floor cells/days/med-RMS/loud | strong cells/days/med-RMS/loud"
            .to_string(),
    );
    for (elo, ehi, ename) in [
        (0.0, 30.0, "CONJ 0-30"),
        (30.0, 90.0, "NEAR 30-90"),
        (90.0, 150.0, "FAR 90-150"),
        (150.0, 180.0, "OPP 150-180"),
    ] {
        let mut line = format!("  {ename}: ");
        for floor in [true, false] {
            let name = if floor { "floor" } else { "strong" };
            let sub: Vec<&&Cell> = mrows
                .iter()
                .filter(|c| {
                    c.floor == floor
                        && geo
                            .get(&c.day)
                            .map(|g| g.1 >= elo && g.1 < ehi)
                            .unwrap_or(false)
                })
                .collect();
            if sub.is_empty() {
                line.push_str(&format!("{name} - | "));
                continue;
            }
            let days: BTreeSet<i64> = sub.iter().map(|c| c.day).collect();
            let rms_v: Vec<f64> = sub.iter().map(|c| c.rms).collect();
            let loud = sub.iter().filter(|c| c.rms >= LOUD_HZ).count();
            line.push_str(&format!(
                "{name} {}c/{}d/{:.3}Hz/{loud}l | ",
                sub.len(),
                days.len(),
                median(&rms_v).unwrap_or(f64::NAN)
            ));
        }
        sec.push(line);
    }

    sec.push("  floor (r AU x eps-bin) joint: cell count and median cell RMS".to_string());
    for (elo, ehi, ename) in [
        (0.0, 30.0, "CONJ"),
        (30.0, 90.0, "NEAR"),
        (90.0, 150.0, "FAR"),
        (150.0, 180.0, "OPP"),
    ] {
        let mut line = format!("  {ename}: ");
        for lo in 0..13 {
            let a = lo as f64;
            let b = a + 1.0;
            let sub: Vec<&&Cell> = mrows
                .iter()
                .filter(|c| {
                    c.floor
                        && geo
                            .get(&c.day)
                            .map(|g| g.0 >= a && g.0 < b && g.1 >= elo && g.1 < ehi)
                            .unwrap_or(false)
                })
                .collect();
            if sub.is_empty() {
                line.push_str(&format!("{a:.0}-{b:.0}: - | "));
            } else {
                let rms_v: Vec<f64> = sub.iter().map(|c| c.rms).collect();
                line.push_str(&format!(
                    "{a:.0}-{b:.0}: {}c/{:.3}Hz | ",
                    sub.len(),
                    median(&rms_v).unwrap_or(f64::NAN)
                ));
            }
        }
        sec.push(line);
    }

    sec.push("  year-month: floor cells/med-RMS/loud | strong cells/med-RMS/loud".to_string());
    struct MonthBin {
        rms: Vec<f64>,
        days: BTreeSet<i64>,
        loud: usize,
    }
    let mut bym: BTreeMap<(i64, bool), MonthBin> = BTreeMap::new();
    for c in &mrows {
        let mk = month_key(c.t0);
        let bin = bym.entry((mk, c.floor)).or_insert_with(|| MonthBin {
            rms: Vec::new(),
            days: BTreeSet::new(),
            loud: 0,
        });
        bin.rms.push(c.rms);
        bin.days.insert(c.day);
        if c.rms >= LOUD_HZ {
            bin.loud += 1;
        }
    }
    for ((mk, floor), bin) in &bym {
        let (y, m) = (mk / 12, mk % 12);
        let name = if *floor { "floor" } else { "strong" };
        sec.push(format!(
            "  {y:04}-{m:02} {name}: {}c/{}d/med {:.4} Hz/{}l",
            bin.rms.len(),
            bin.days.len(),
            median(&bin.rms).unwrap_or(f64::NAN),
            bin.loud
        ));
    }
    sec
}

fn main() {
    let path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "/tmp/opencode/galileo_floor_4d_report.txt".to_string(),
    };
    let mut eph: HashMap<String, BodyEphemeris> = HashMap::new();
    for b in ["galileo_daily", "earth"] {
        if !load(b, &mut eph) {
            eprintln!("galileo: {b} ephemeris bin void");
        }
    }
    let geom_ok = eph.contains_key("galileo_daily") && eph.contains_key("earth");
    let Ok(bytes) = std::fs::read("data/galileo_resid.bin") else {
        eprintln!("galileo: resid bin void");
        return;
    };
    let Some(recs) = omegaflow::atdf::parse_resid_bin(&bytes) else {
        eprintln!("galileo: resid bin parse void");
        return;
    };
    drop(bytes);

    let mut cell: BTreeMap<(i64, i64, i64, u8), (f64, f64, usize, f64)> = BTreeMap::new();
    let mut color: [Vec<(f64, f64)>; 12] = Default::default();
    let st_idx = |s: i64| -> Option<usize> {
        match s {
            14 => Some(0),
            43 => Some(1),
            63 => Some(2),
            _ => None,
        }
    };
    for r in &recs {
        let mode = r[3] as i64;
        if mode != 1 && mode != 2 {
            continue;
        }
        let resid = r[1];
        if !resid.is_finite() || resid.abs() > LOCK_HZ {
            continue;
        }
        let s = r[7] as i64;
        if s == 0 {
            continue;
        }
        let day = (r[0] / DAY_S).floor() as i64;
        let st = r[2] as i64;
        let cls: u8 = if s == FLOOR {
            0
        } else if s >= STRONG_MIN {
            1
        } else {
            continue;
        };
        let e = cell
            .entry((mode, day, st, cls))
            .or_insert_with(|| (0.0, 0.0, 0, r[0]));
        e.0 += resid;
        e.1 += resid * resid;
        e.2 += 1;
        if r[0] < e.3 {
            e.3 = r[0];
        }
        if mode == 1 {
            if let Some(si) = st_idx(st) {
                let mk = month_key(r[0]);
                let win = if (1995 * 12 + 11..=1995 * 12 + 12).contains(&mk) {
                    0usize
                } else if (1996 * 12 + 6..=1997 * 12 + 2).contains(&mk) {
                    1usize
                } else {
                    continue;
                };
                color[si * 4 + cls as usize * 2 + win].push((r[0], resid));
            }
        }
    }
    drop(recs);

    let mut rows: Vec<Cell> = Vec::new();
    for (&(mode, day, st, cls), &(sum, sum2, n, t0)) in &cell {
        if n == 0 {
            continue;
        }
        let m = sum / n as f64;
        let v = (sum2 / n as f64 - m * m).max(0.0);
        rows.push(Cell {
            mode,
            day,
            st,
            floor: cls == 0,
            n,
            rms: v.sqrt(),
            t0,
        });
    }
    rows.sort_by_key(|c| (c.mode, c.day, c.st));

    let mut geo: BTreeMap<i64, (f64, f64, f64, [f64; 3])> = BTreeMap::new();
    if geom_ok {
        let days: BTreeSet<i64> = rows.iter().map(|c| c.day).collect();
        for &d in &days {
            let t = d as f64 * DAY_S;
            if let (Some(p), Some(e)) = (
                body_barycenter_position("galileo_daily", t, &eph),
                body_barycenter_position("earth", t, &eph),
            ) {
                let rp = norm(p) / AU_M;
                if let (Some(ep), Some(al)) = (angle(sub([0.0; 3], e), sub(p, e)), angle(e, p)) {
                    let pp = [p[0] / AU_M, p[1] / AU_M, p[2] / AU_M];
                    geo.insert(d, (rp, ep, al, pp));
                }
            }
        }
    }

    let mut out: Vec<String> = Vec::new();
    let mut push_print = |s: String| {
        println!("{s}");
        out.push(s);
    };
    push_print("galileo floor 4D form + color probe".to_string());
    push_print(format!(
        "binding: modes 1 and 2; lock (|resid| > {LOCK_HZ:.0} Hz) excluded before noise; strength 0 separated, never classed"
    ));
    push_print(format!(
        "classes: floor = strength == {FLOOR} (AGC clamp); strong = strength >= {STRONG_MIN}"
    ));
    push_print(
        "geometry at the TDB day start from galileo_daily / earth barycentric ICRS positions (AU = 1.495978707e11 m); r = heliocentric |p|; eps = elongation at the Earth; alpha = angle at the Sun; loud = cell RMS >= 1 Hz".to_string(),
    );
    push_print(
        "color: mode-1 resid at ~1 s cadence per (station, class, window); contiguous runs, per-chunk DFT, log10 PSD pooled in 0.1-decade bins (>= 20 counts/bin); slope = LSQ log10 PSD vs log10 f over the covered band; detrend = linear, mean-subtract = mean only".to_string(),
    );
    push_print(
        "window late = 1996-06 .. 1997-02 (the loud floor era); window quiet = 1995-11 .. 1995-12"
            .to_string(),
    );

    for mode in [1i64, 2] {
        out.push(String::new());
        out.push(format!("== mode {mode} occurrence =="));
        for floor in [true, false] {
            let srows: Vec<&Cell> = rows
                .iter()
                .filter(|c| c.mode == mode && c.floor == floor)
                .collect();
            let n_samp: usize = srows.iter().map(|c| c.n).sum();
            let days: BTreeSet<i64> = srows.iter().map(|c| c.day).collect();
            let loud = srows.iter().filter(|c| c.rms >= LOUD_HZ).count();
            let (lo_d, hi_d) = match (
                srows.iter().map(|c| c.t0).min_by(f64::total_cmp),
                srows.iter().map(|c| c.t0).max_by(f64::total_cmp),
            ) {
                (Some(a), Some(b)) => (fmt_date(a), fmt_date(b)),
                _ => ("-".to_string(), "-".to_string()),
            };
            let rms_v: Vec<f64> = srows.iter().map(|c| c.rms).collect();
            let name = if floor { "floor" } else { "strong" };
            let line = format!(
                "  {name}: {n_samp} samples, {} (day,station) cells, {} distinct days, {lo_d} .. {hi_d}, loud {loud} cells, median cell RMS {:.3} Hz",
                srows.len(),
                days.len(),
                median(&rms_v).unwrap_or(f64::NAN)
            );
            println!("{line}");
            out.push(line);
        }
    }

    for mode in [1i64, 2] {
        let sec = envelope_rows(&rows, mode, &geo);
        for s in &sec {
            println!("{s}");
        }
        out.extend(sec);
    }

    out.push(String::new());
    out.push("== loud floor cells (RMS >= 1 Hz), chronological ==".to_string());
    for c in rows.iter().filter(|c| c.floor && c.rms >= LOUD_HZ) {
        out.push(cell_line(c, &geo));
    }
    out.push(String::new());
    out.push("== loud strong cells (RMS >= 1 Hz), chronological ==".to_string());
    for c in rows.iter().filter(|c| !c.floor && c.rms >= LOUD_HZ) {
        out.push(cell_line(c, &geo));
    }
    out.push(String::new());
    out.push("== all floor cells, chronological ==".to_string());
    for c in rows.iter().filter(|c| c.floor) {
        out.push(cell_line(c, &geo));
    }

    out.push(String::new());
    out.push(
        "== rank correlation of log10 cell RMS vs geometry/time (mode 1, per class) ==".to_string(),
    );
    for floor in [true, false] {
        let srows: Vec<&Cell> = rows
            .iter()
            .filter(|c| c.mode == 1 && c.floor == floor)
            .collect();
        let name = if floor { "floor" } else { "strong" };
        let mut lr = Vec::new();
        let mut vr = Vec::new();
        for c in &srows {
            let logr = c.rms.log10();
            if logr.is_finite() {
                lr.push(logr);
                if let Some(&(rr, _, _, _)) = geo.get(&c.day) {
                    vr.push(rr);
                }
            }
        }
        let days_v: Vec<f64> = srows.iter().map(|c| c.day as f64).collect();
        let rhor = if lr.len() == vr.len() {
            spearman(&vr, &lr)
        } else {
            None
        };
        let rhot = spearman(&days_v, &lr);
        let mut ex = Vec::new();
        let mut ey = Vec::new();
        for c in &srows {
            if let Some(&(_, ep, _, _)) = geo.get(&c.day) {
                ex.push(ep);
                ey.push(c.rms.log10());
            }
        }
        out.push(format!(
            "  mode1 {name}: n cells {} | rho(log10RMS vs heliocentric r) {} (n {}) | rho(log10RMS vs elongation eps) {} (n {}) | rho(log10RMS vs tdb day) {}",
            srows.len(),
            fmt_level(rhor),
            vr.len(),
            fmt_level(spearman(&ex, &ey)),
            ex.len(),
            fmt_level(rhot)
        ));
    }

    out.push(String::new());
    out.push("== color: mode-1 resid PSD slope per (station, class, window) ==".to_string());
    for si in 0..3 {
        for floor in [true, false] {
            for (wname, wi) in [("quiet", 0usize), ("late", 1usize)] {
                let data = color[si * 4 + floor as usize * 2 + wi].clone();
                color_line(&data, &mut out, [14, 43, 63][si], floor, wname);
            }
        }
    }
    let c0 = out
        .iter()
        .position(|x| x.starts_with("== color"))
        .unwrap_or(out.len());
    for s in &out[c0..] {
        println!("{s}");
    }

    let _ = std::fs::write(&path, out.join("\n") + "\n");
    eprintln!("galileo: floor 4D report written to {path}");
}
