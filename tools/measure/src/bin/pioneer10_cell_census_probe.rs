use std::collections::{BTreeMap, BTreeSet};

use omegaflow::atdf::parse_bin as parse_pasf;

const DAY_S: f64 = 86400.0;
const BAND_LO: f64 = 0.044;
const BAND_HI: f64 = 0.058;
const STEP: f64 = 0.00002;
const STATIONS: [i64; 3] = [14, 43, 63];
const GAP_RUN_S: f64 = 600.0;
const MIN_RUN: usize = 4;
const MIN_N: usize = 200;
const FLAG_TOL: f64 = 0.00005;

#[derive(Clone, Copy)]
enum Field {
    Resid,
    Fsky,
    Ref,
}

impl Field {
    fn name(self) -> &'static str {
        match self {
            Field::Resid => "resid",
            Field::Fsky => "fsky",
            Field::Ref => "ref",
        }
    }

    fn present(self, r: &[f64; 14]) -> bool {
        match self {
            Field::Resid => r[8].is_finite(),
            Field::Fsky => r[1].is_finite() && r[1] > 0.0,
            Field::Ref => r[2].is_finite(),
        }
    }

    fn value(self, r: &Rec) -> f64 {
        match self {
            Field::Resid => r.resid,
            Field::Fsky => r.fsky,
            Field::Ref => r.refv,
        }
    }
}

struct Rec {
    t: f64,
    sampler: f64,
    station: i64,
    mode: i64,
    resid: f64,
    fsky: f64,
    refv: f64,
    year: Option<i64>,
    file_id: i64,
}

fn paper_value(st: i64) -> f64 {
    match st {
        14 => 0.04575,
        43 => 0.05155,
        63 => 0.04735,
        _ => f64::NAN,
    }
}

type GridPoint = (f64, f64, Option<f64>);

fn ls_fit(times: &[f64], vals: &[f64], vsum: f64, ss_tot: f64, fref: f64) -> (f64, Option<f64>) {
    let m = times.len() as f64;
    let mut s = 0.0;
    let mut c = 0.0;
    for &t in times {
        let ph = std::f64::consts::TAU * fref * t;
        s += ph.sin();
        c += ph.cos();
    }
    s /= m;
    c /= m;
    let mut ss = 0.0;
    let mut cc = 0.0;
    let mut sc = 0.0;
    let mut sy = 0.0;
    let mut cy = 0.0;
    for (i, &t) in times.iter().enumerate() {
        let ph = std::f64::consts::TAU * fref * t;
        let ds = ph.sin() - s;
        let dc = ph.cos() - c;
        let dv = vals[i] - vsum;
        ss += ds * ds;
        cc += dc * dc;
        sc += ds * dc;
        sy += ds * dv;
        cy += dc * dv;
    }
    let det = ss * cc - sc * sc;
    if det.abs() <= 1e-300 {
        return (0.0, None);
    }
    let a = (sy * cc - cy * sc) / det;
    let b = (cy * ss - sy * sc) / det;
    let power = (a * a + b * b) * m / 2.0;
    if ss_tot > 0.0 {
        let z = (a * sy + b * cy) / ss_tot;
        if (0.0..=1.0).contains(&z) {
            return (power, Some(z));
        }
    }
    (power, None)
}

fn ls_power_at(times: &[f64], vals: &[f64], vsum: f64, ss_tot: f64, fref: f64) -> f64 {
    ls_fit(times, vals, vsum, ss_tot, fref).0
}

fn ls_grid(times: &[f64], vals: &[f64], flo: f64, fhi: f64, step: f64) -> Vec<GridPoint> {
    let mut grid: Vec<GridPoint> = Vec::new();
    if times.is_empty() {
        return grid;
    }
    let m = times.len() as f64;
    let vsum = vals.iter().sum::<f64>() / m;
    let ss_tot: f64 = vals.iter().map(|v| (v - vsum) * (v - vsum)).sum();
    let mut f = flo;
    while f <= fhi {
        let (pow, norm) = ls_fit(times, vals, vsum, ss_tot, f);
        grid.push((f, pow, norm));
        f += step;
    }
    grid
}

fn peak_of(grid: &[GridPoint]) -> GridPoint {
    if grid.is_empty() {
        return (f64::NAN, 0.0, None);
    }
    let mut best = grid[0];
    for g in grid {
        if g.1 > best.1 {
            best = *g;
        }
    }
    best
}

fn peak_interp(grid: &[GridPoint]) -> Option<f64> {
    let (fmax, _, _) = peak_of(grid);
    let k = grid.iter().position(|g| g.0 == fmax)?;
    if k == 0 || k + 1 >= grid.len() {
        return None;
    }
    let step = grid[k].0 - grid[k - 1].0;
    let pm = grid[k - 1].1;
    let p0 = grid[k].1;
    let pp = grid[k + 1].1;
    let denom = pm - 2.0 * p0 + pp;
    if denom.abs() < 1e-300 {
        return None;
    }
    let d = 0.5 * (pm - pp) / denom;
    if d.abs() > 1.0 {
        return None;
    }
    Some(fmax + d * step)
}

fn detrend_runs(ts: &[f64], vs: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut dts: Vec<f64> = Vec::new();
    let mut dvs: Vec<f64> = Vec::new();
    let mut lo = 0usize;
    while lo < ts.len() {
        let mut hi = lo + 1;
        while hi < ts.len() && ts[hi] - ts[hi - 1] <= GAP_RUN_S {
            hi += 1;
        }
        if hi - lo >= MIN_RUN {
            let n = (hi - lo) as f64;
            let mx = ts[lo..hi].iter().sum::<f64>() / n;
            let my = vs[lo..hi].iter().sum::<f64>() / n;
            let mut num = 0.0;
            let mut den = 0.0;
            for k in lo..hi {
                num += (ts[k] - mx) * (vs[k] - my);
                den += (ts[k] - mx) * (ts[k] - mx);
            }
            let slope = if den.abs() > 1e-300 { num / den } else { 0.0 };
            for k in lo..hi {
                dts.push(ts[k]);
                dvs.push(vs[k] - (slope * (ts[k] - mx) + my));
            }
        }
        lo = hi;
    }
    (dts, dvs)
}

fn year_of(tdb: f64) -> Option<i64> {
    let jd = 2451545.0 + tdb / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y as i64)
}

fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(f64::total_cmp);
    let n = xs.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        0.5 * (xs[n / 2 - 1] + xs[n / 2])
    }
}

struct CellPeak {
    peak: f64,
    n_det: usize,
    floor: f64,
    ratio: Option<f64>,
    fap: Option<f64>,
}

fn peak_of_cell(mut seq: Vec<(f64, f64)>) -> Option<CellPeak> {
    if seq.len() < MIN_N {
        return None;
    }
    seq.sort_by(|a, b| a.0.total_cmp(&b.0));
    let ts: Vec<f64> = seq.iter().map(|x| x.0).collect();
    let vs: Vec<f64> = seq.iter().map(|x| x.1).collect();
    let (dts, dvs) = detrend_runs(&ts, &vs);
    if dts.len() < MIN_N {
        return None;
    }
    let grid = ls_grid(&dts, &dvs, BAND_LO, BAND_HI, STEP);
    let (fp, fpow, peak_norm) = peak_of(&grid);
    let peak = match peak_interp(&grid) {
        Some(p) => p,
        None => fp,
    };
    let floor = median(grid.iter().map(|g| g.1).collect());
    let ratio = if floor > 0.0 && fpow.is_finite() {
        Some(fpow / floor)
    } else {
        None
    };
    let fap = peak_norm.and_then(|z| {
        let n = dts.len();
        if n <= 3 {
            return None;
        }
        let p1 = (1.0 - z).powf((n as f64 - 3.0) / 2.0);
        Some(1.0 - (1.0 - p1).powf(grid.len() as f64))
    });
    Some(CellPeak {
        peak,
        n_det: dts.len(),
        floor,
        ratio,
        fap,
    })
}

struct CrossRank {
    resid_dom: f64,
    fsky_at_resid: f64,
    fsky_rank: usize,
    fsky_n: usize,
    fsky_dom: f64,
    resid_at_fsky: f64,
    resid_rank: usize,
    resid_n: usize,
}

fn cross_rank(seq_resid: &[(f64, f64)], seq_fsky: &[(f64, f64)]) -> Option<CrossRank> {
    if seq_resid.len() < MIN_N || seq_fsky.len() < MIN_N {
        return None;
    }
    let mut sr: Vec<(f64, f64)> = seq_resid
        .iter()
        .copied()
        .filter(|(t, v)| t.is_finite() && v.is_finite())
        .collect();
    sr.sort_by(|a, b| a.0.total_cmp(&b.0));
    let rt: Vec<f64> = sr.iter().map(|x| x.0).collect();
    let rv: Vec<f64> = sr.iter().map(|x| x.1).collect();
    let (rdt, rdv) = detrend_runs(&rt, &rv);
    if rdt.len() < MIN_N {
        return None;
    }
    let mut sf: Vec<(f64, f64)> = seq_fsky
        .iter()
        .copied()
        .filter(|(t, v)| t.is_finite() && v.is_finite())
        .collect();
    sf.sort_by(|a, b| a.0.total_cmp(&b.0));
    let ft: Vec<f64> = sf.iter().map(|x| x.0).collect();
    let fv: Vec<f64> = sf.iter().map(|x| x.1).collect();
    let (fdt, fdv) = detrend_runs(&ft, &fv);
    if fdt.len() < MIN_N {
        return None;
    }
    let rgrid = ls_grid(&rdt, &rdv, BAND_LO, BAND_HI, STEP);
    let fgrid = ls_grid(&fdt, &fdv, BAND_LO, BAND_HI, STEP);
    let resid_dom = match peak_interp(&rgrid) {
        Some(p) => p,
        None => peak_of(&rgrid).0,
    };
    let fsky_dom = match peak_interp(&fgrid) {
        Some(p) => p,
        None => peak_of(&fgrid).0,
    };
    let rm = rdv.len() as f64;
    let rsum = rdv.iter().sum::<f64>() / rm;
    let rss: f64 = rdv.iter().map(|v| (v - rsum) * (v - rsum)).sum();
    let fm = fdv.len() as f64;
    let fsum = fdv.iter().sum::<f64>() / fm;
    let fss: f64 = fdv.iter().map(|v| (v - fsum) * (v - fsum)).sum();
    let fsky_at_resid = ls_power_at(&fdt, &fdv, fsum, fss, resid_dom);
    let fsky_rank = 1 + fgrid.iter().filter(|g| g.1 > fsky_at_resid).count();
    let fsky_n = fgrid.len();
    let resid_at_fsky = ls_power_at(&rdt, &rdv, rsum, rss, fsky_dom);
    let resid_rank = 1 + rgrid.iter().filter(|g| g.1 > resid_at_fsky).count();
    let resid_n = rgrid.len();
    Some(CrossRank {
        resid_dom,
        fsky_at_resid,
        fsky_rank,
        fsky_n,
        fsky_dom,
        resid_at_fsky,
        resid_rank,
        resid_n,
    })
}

fn xrank_line(label: &str, xr: Option<CrossRank>) -> String {
    match xr {
        Some(c) => format!(
            "xrank {label}: resid-dom {:.3} -> fsky rank {}/{} pow {:.3e} | fsky-dom {:.3} -> resid rank {}/{} pow {:.3e}",
            c.resid_dom * 1e3,
            c.fsky_rank,
            c.fsky_n,
            c.fsky_at_resid,
            c.fsky_dom * 1e3,
            c.resid_rank,
            c.resid_n,
            c.resid_at_fsky,
        ),
        None => format!("xrank {label}: —"),
    }
}

fn fmt_peak(st: i64, peak: f64) -> String {
    let hit = (peak - paper_value(st)).abs() <= FLAG_TOL;
    if hit {
        format!(
            "{:.3} mHz  <<< MATCH st{st} {:.3} mHz",
            peak * 1e3,
            paper_value(st) * 1e3
        )
    } else {
        format!("{:.3} mHz", peak * 1e3)
    }
}

fn fmt_ratio(ratio: Option<f64>) -> String {
    match ratio {
        Some(r) => format!("{r:.3}"),
        None => "—".to_string(),
    }
}

fn fmt_fap(fap: Option<f64>) -> String {
    match fap {
        Some(p) => format!("FAP {p:.2e}"),
        None => "FAP —".to_string(),
    }
}

fn cell_line(st: i64, label: &str, n_raw: usize, peak: Option<CellPeak>) -> String {
    match peak {
        Some(c) => format!(
            "{label:<16} n={n_raw:<7} (detrend {}) peak {:<38} floor {:.3e} x {} {}",
            c.n_det,
            fmt_peak(st, c.peak),
            c.floor,
            fmt_ratio(c.ratio),
            fmt_fap(c.fap),
        ),
        None => {
            if n_raw < MIN_N {
                format!("{label:<16} n={n_raw:<7} — absent (n < {MIN_N})")
            } else {
                format!("{label:<16} n={n_raw:<7} — absent (detrend < {MIN_N})")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let field = match args.iter().position(|a| a == "--field") {
        Some(i) => match args.get(i + 1).map(String::as_str) {
            Some("fsky") => Field::Fsky,
            Some("ref") => Field::Ref,
            _ => Field::Resid,
        },
        None => Field::Resid,
    };

    let pasf_bytes = match std::fs::read("data/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin") {
        Ok(b) => b,
        Err(_) => {
            eprintln!("PASF void — empty (0 honored)");
            return;
        }
    };
    let Some(pasf) = parse_pasf(&pasf_bytes) else {
        eprintln!("PASF parse void");
        return;
    };

    let recs: Vec<Rec> = pasf
        .iter()
        .filter(|r| field.present(r))
        .map(|r| Rec {
            t: r[0],
            sampler: r[3],
            station: r[6] as i64,
            mode: r[13] as i64,
            resid: r[8],
            fsky: r[1],
            refv: r[2],
            year: year_of(r[0]),
            file_id: r[12] as i64,
        })
        .collect();

    let n_dumps = recs
        .iter()
        .map(|r| r.file_id)
        .collect::<BTreeSet<i64>>()
        .len();
    eprintln!(
        "PASF: {} records, {} finite-{}, {} distinct dumps",
        pasf.len(),
        recs.len(),
        field.name(),
        n_dumps
    );

    let modes: [(&str, fn(i64) -> bool); 5] = [
        ("mode1", |m| m == 1),
        ("mode2", |m| m == 2),
        ("mode3", |m| m == 3),
        ("mode1+3", |m| m == 1 || m == 3),
        ("mode1+2+3", |m| m == 1 || m == 2 || m == 3),
    ];
    let classes: [(&str, fn(f64) -> bool); 5] = [
        ("s1.000", |s| s == 1.0),
        ("sub2", |s| s < 2.0),
        ("lt10", |s| s < 10.0),
        ("10-30", |s| s >= 10.0 && s < 30.0),
        ("60", |s| s >= 30.0),
    ];

    println!(
        "=== CELL CENSUS (ground_mode x sampler-class) — peak in mHz over 44–58 mHz field={} ===",
        field.name()
    );
    for st in STATIONS {
        println!("station {st} (paper {:.3} mHz):", paper_value(st) * 1e3);
        for (mname, mpred) in modes {
            for (cname, cpred) in classes {
                let label = format!("{mname} {cname}");
                let seq: Vec<(f64, f64)> = recs
                    .iter()
                    .filter(|r| r.station == st && mpred(r.mode) && cpred(r.sampler))
                    .map(|r| (r.t, field.value(r)))
                    .collect();
                let n_raw = seq.len();
                let peak = peak_of_cell(seq);
                println!("  {}", cell_line(st, &label, n_raw, peak));
                let seq_resid: Vec<(f64, f64)> = recs
                    .iter()
                    .filter(|r| r.station == st && mpred(r.mode) && cpred(r.sampler))
                    .map(|r| (r.t, r.resid))
                    .collect();
                let seq_fsky: Vec<(f64, f64)> = recs
                    .iter()
                    .filter(|r| r.station == st && mpred(r.mode) && cpred(r.sampler))
                    .map(|r| (r.t, r.fsky))
                    .collect();
                println!(
                    "  {}",
                    xrank_line(&label, cross_rank(&seq_resid, &seq_fsky))
                );
            }
        }
    }

    println!("\n=== STRICT-1.000-s CLASS COUNTS (confound e) ===");
    for st in STATIONS {
        let total = recs
            .iter()
            .filter(|r| r.station == st && r.sampler == 1.0)
            .count();
        let by_mode: BTreeMap<i64, usize> = recs
            .iter()
            .filter(|r| r.station == st && r.sampler == 1.0)
            .fold(BTreeMap::new(), |mut acc, r| {
                *acc.entry(r.mode).or_insert(0) += 1;
                acc
            });
        println!(
            "  station {st}: strict-1.000-s total n={total}, by ground_mode {:?}",
            by_mode
        );
    }

    println!("\n=== ERA — per year (mode3, lt10 = paper baseline) ===");
    let mut years: Vec<i64> = recs.iter().filter_map(|r| r.year).collect();
    years.sort_unstable();
    years.dedup();
    for st in STATIONS {
        println!("  station {st}:");
        for y in &years {
            let seq: Vec<(f64, f64)> = recs
                .iter()
                .filter(|r| {
                    r.station == st && r.mode == 3 && r.sampler < 10.0 && r.year == Some(*y)
                })
                .map(|r| (r.t, field.value(r)))
                .collect();
            let n_raw = seq.len();
            let peak = peak_of_cell(seq);
            println!("    {y}: {}", cell_line(st, "mode3 lt10", n_raw, peak));
        }
    }

    println!("\n=== ERA — per dump (mode3, lt10 = paper baseline; confound f) ===");
    let mut dumps: Vec<i64> = recs.iter().map(|r| r.file_id).collect();
    dumps.sort_unstable();
    dumps.dedup();
    for st in STATIONS {
        println!("  station {st}:");
        for d in &dumps {
            let seq: Vec<(f64, f64)> = recs
                .iter()
                .filter(|r| r.station == st && r.mode == 3 && r.sampler < 10.0 && r.file_id == *d)
                .map(|r| (r.t, field.value(r)))
                .collect();
            let n_raw = seq.len();
            let yrange = seq.iter().filter_map(|(t, _)| year_of(*t)).fold(
                None,
                |acc: Option<(i64, i64)>, y| match acc {
                    None => Some((y, y)),
                    Some((lo, hi)) => Some((lo.min(y), hi.max(y))),
                },
            );
            let peak = peak_of_cell(seq);
            let yr = match yrange {
                Some((lo, hi)) => format!("{lo}–{hi}"),
                None => "void".to_string(),
            };
            println!(
                "    dump {d} ({yr}): {}",
                cell_line(st, "mode3 lt10", n_raw, peak)
            );
        }
    }

    println!("\n=== (mode x class x year) CROSS — only cells with n >= {MIN_N} ===");
    for st in STATIONS {
        println!("  station {st} (paper {:.3} mHz):", paper_value(st) * 1e3);
        for (mname, mpred) in modes {
            for (cname, cpred) in classes {
                for y in &years {
                    let seq: Vec<(f64, f64)> = recs
                        .iter()
                        .filter(|r| {
                            r.station == st
                                && mpred(r.mode)
                                && cpred(r.sampler)
                                && r.year == Some(*y)
                        })
                        .map(|r| (r.t, field.value(r)))
                        .collect();
                    let n_raw = seq.len();
                    if n_raw < MIN_N {
                        continue;
                    }
                    let peak = peak_of_cell(seq);
                    let Some(c) = peak else { continue };
                    let label = format!("{mname} {cname} {y}");
                    println!("    {}", cell_line(st, &label, n_raw, Some(c)));
                    let seq_resid: Vec<(f64, f64)> = recs
                        .iter()
                        .filter(|r| {
                            r.station == st
                                && mpred(r.mode)
                                && cpred(r.sampler)
                                && r.year == Some(*y)
                        })
                        .map(|r| (r.t, r.resid))
                        .collect();
                    let seq_fsky: Vec<(f64, f64)> = recs
                        .iter()
                        .filter(|r| {
                            r.station == st
                                && mpred(r.mode)
                                && cpred(r.sampler)
                                && r.year == Some(*y)
                        })
                        .map(|r| (r.t, r.fsky))
                        .collect();
                    println!(
                        "    {}",
                        xrank_line(&label, cross_rank(&seq_resid, &seq_fsky))
                    );
                }
            }
        }
    }

    println!("\n=== REFERENCE r[2] LS — st63 lt10 1992 (fsky-fine cells) ===");
    for (mname, mpred) in modes {
        let seq_resid: Vec<(f64, f64)> = recs
            .iter()
            .filter(|r| {
                r.station == 63
                    && r.sampler < 10.0
                    && r.year == Some(1992)
                    && mpred(r.mode)
                    && r.resid.is_finite()
            })
            .map(|r| (r.t, r.resid))
            .collect();
        let seq_fsky: Vec<(f64, f64)> = recs
            .iter()
            .filter(|r| {
                r.station == 63
                    && r.sampler < 10.0
                    && r.year == Some(1992)
                    && mpred(r.mode)
                    && r.fsky.is_finite()
                    && r.fsky > 0.0
            })
            .map(|r| (r.t, r.fsky))
            .collect();
        let seq_ref: Vec<(f64, f64)> = recs
            .iter()
            .filter(|r| {
                r.station == 63
                    && r.sampler < 10.0
                    && r.year == Some(1992)
                    && mpred(r.mode)
                    && r.refv.is_finite()
            })
            .map(|r| (r.t, r.refv))
            .collect();
        for (fname, seq) in [("resid", seq_resid), ("fsky", seq_fsky), ("ref", seq_ref)] {
            let n_raw = seq.len();
            let peak = peak_of_cell(seq);
            println!("    {mname} {fname} {}", cell_line(63, fname, n_raw, peak));
        }
    }
}
