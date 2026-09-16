use omegaflow::atdf::parse_bin;

const DAY_S: f64 = 86400.0;
const BAND_LO: f64 = 0.044;
const BAND_HI: f64 = 0.058;
const MIR_LO: f64 = 0.044;
const MIR_HI: f64 = 0.056;
const STEP: f64 = 0.00005;
const GAP_RUN_S: f64 = 600.0;
const MIN_RUN: usize = 4;
const MIN_N: usize = 200;
const TOP_MEMBERS: usize = 5;

const STATIONS: [i64; 3] = [14, 43, 63];
const REF_FREQS: [(i64, f64, &str); 3] = [
    (14, 0.04575, "45.75"),
    (43, 0.05155, "51.55"),
    (63, 0.04735, "47.35"),
];
const DOMINANTS: [(i64, f64, &str); 3] = [
    (14, 0.05711, "57.11"),
    (43, 0.04440, "44.40"),
    (63, 0.05199, "51.99"),
];
const EARLIER_AMP: [(i64, &str); 3] = [
    (14, "160/153/161 Hz (weak->strong)"),
    (43, "104/102/82 Hz (weak->strong)"),
    (63, "57/50/65 Hz (weak->strong)"),
];

struct Rec {
    t: f64,
    sampler: f64,
    station: i64,
    resid: f64,
    strength: f64,
    year: Option<i64>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn year_of(tdb: f64) -> Option<i64> {
    let jd = 2451545.0 + tdb / DAY_S;
    let unix_day = (jd - 2440587.5).round() as i64;
    omegaflow::spectral::civil_from_days(unix_day).map(|(y, _, _)| y as i64)
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

fn ls_grid(times: &[f64], vals: &[f64], flo: f64, fhi: f64, step: f64) -> Vec<(f64, f64)> {
    let mut grid: Vec<(f64, f64)> = Vec::new();
    let mut f = flo;
    while f <= fhi + step * 0.5 {
        grid.push((f, 0.0));
        f += step;
    }
    if grid.is_empty() {
        return grid;
    }
    let m = times.len() as f64;
    let vsum = vals.iter().sum::<f64>() / m;
    for (fref, pow) in grid.iter_mut() {
        let mut s = 0.0;
        let mut c = 0.0;
        for &t in times {
            let ph = std::f64::consts::TAU * *fref * t;
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
            let ph = std::f64::consts::TAU * *fref * t;
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
        if det.abs() > 1e-300 {
            let a = (sy * cc - cy * sc) / det;
            let b = (cy * ss - sy * sc) / det;
            *pow = (a * a + b * b) * m / 2.0;
        }
    }
    grid
}

fn median_power(grid: &[(f64, f64)]) -> Option<f64> {
    if grid.is_empty() {
        return None;
    }
    let mut pows: Vec<f64> = grid.iter().map(|g| g.1).collect();
    pows.sort_by(f64::total_cmp);
    Some(pows[pows.len() / 2])
}

fn peak_of(grid: &[(f64, f64)]) -> Option<(usize, f64, f64, f64)> {
    if grid.is_empty() {
        return None;
    }
    let mut bi = 0;
    for (i, g) in grid.iter().enumerate() {
        if g.1 > grid[bi].1 {
            bi = i;
        }
    }
    let best = grid[bi];
    let floor = median_power(grid)?;
    if floor <= 0.0 {
        return None;
    }
    Some((bi, best.0, best.1, best.1 / floor))
}

fn peak_interp(grid: &[(f64, f64)], idx: usize) -> Option<f64> {
    if idx == 0 || idx + 1 >= grid.len() {
        return None;
    }
    let step = grid[idx].0 - grid[idx - 1].0;
    let pm = grid[idx - 1].1;
    let p0 = grid[idx].1;
    let pp = grid[idx + 1].1;
    let denom = pm - 2.0 * p0 + pp;
    if denom.abs() < 1e-300 {
        return None;
    }
    let d = 0.5 * (pm - pp) / denom;
    if d.abs() > 1.0 {
        return None;
    }
    Some(grid[idx].0 + d * step)
}

fn amp_of(power: f64, n: usize) -> Option<f64> {
    if n == 0 || !power.is_finite() || power < 0.0 {
        return None;
    }
    Some((2.0 * power / n as f64).sqrt())
}

fn nearest_power(grid: &[(f64, f64)], f: f64) -> Option<f64> {
    grid.iter()
        .min_by(|a, b| (a.0 - f).abs().total_cmp(&(b.0 - f).abs()))
        .map(|(_, p)| *p)
}

fn top_members(grid: &[(f64, f64)], floor: f64, k: usize) -> Vec<(f64, f64, f64)> {
    let mut local: Vec<(f64, f64, f64)> = Vec::new();
    for i in 1..grid.len().saturating_sub(1) {
        if grid[i].1 > grid[i - 1].1 && grid[i].1 >= grid[i + 1].1 {
            let f = peak_interp(grid, i).unwrap_or(grid[i].0);
            local.push((f, grid[i].1, grid[i].1 / floor));
        }
    }
    local.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut taken: Vec<(f64, f64, f64)> = Vec::new();
    for (f, p, r) in local {
        if taken.iter().all(|(tf, _, _)| (tf - f).abs() > 0.00015) {
            taken.push((f, p, r));
        }
        if taken.len() >= k {
            break;
        }
    }
    taken
}

fn year_cell(
    recs: &[Rec],
    station: i64,
    class: fn(f64) -> bool,
    year: Option<i64>,
) -> (Vec<f64>, Vec<f64>) {
    let mut ts: Vec<f64> = Vec::new();
    let mut vs: Vec<f64> = Vec::new();
    for r in recs {
        if r.station != station || !class(r.sampler) {
            continue;
        }
        if let Some(y) = year {
            if r.year != Some(y) {
                continue;
            }
        }
        ts.push(r.t);
        vs.push(r.resid);
    }
    let mut order: Vec<usize> = (0..ts.len()).collect();
    order.sort_by(|&a, &b| ts[a].total_cmp(&ts[b]));
    let ts: Vec<f64> = order.iter().map(|&i| ts[i]).collect();
    let vs: Vec<f64> = order.iter().map(|&i| vs[i]).collect();
    (ts, vs)
}

fn fmt_amp(a: Option<f64>) -> String {
    match a {
        Some(v) => format!("{v:.3e} Hz"),
        None => "absent".to_string(),
    }
}

fn cell_report(
    station: i64,
    class_name: &str,
    year: Option<i64>,
    ts: &[f64],
    vs: &[f64],
) -> String {
    let n_raw = ts.len();
    let (dts, dvs) = detrend_runs(ts, vs);
    if dts.len() < MIN_N {
        return format!(
            "  {:<8} {:<9} n={n_raw:<6} — absent (detrend n={} < {MIN_N})",
            year.map_or("all".to_string(), |y| y.to_string()),
            class_name,
            dts.len()
        );
    }
    let grid = ls_grid(&dts, &dvs, BAND_LO, BAND_HI, STEP);
    let Some(floor) = median_power(&grid) else {
        return format!(
            "  {:<8} {:<9} n={n_raw:<6} — absent (empty LS grid)",
            year.map_or("all".to_string(), |y| y.to_string()),
            class_name
        );
    };
    let Some((fi, fp, pp, ratio)) = peak_of(&grid) else {
        return format!(
            "  {:<8} {:<9} n={n_raw:<6} — absent (floor {floor:.3e})",
            year.map_or("all".to_string(), |y| y.to_string()),
            class_name
        );
    };
    let f_peak = peak_interp(&grid, fi).unwrap_or(fp);
    let a_peak = amp_of(pp, dts.len());
    let members = top_members(&grid, floor, TOP_MEMBERS);
    let mem_txt: Vec<String> = members
        .iter()
        .map(|(f, p, _)| format!("{:.3} mHz A={}", f * 1e3, fmt_amp(amp_of(*p, dts.len()))))
        .collect();
    let mirror: Vec<(f64, f64)> = grid
        .iter()
        .copied()
        .filter(|(f, _)| *f >= MIR_LO && *f <= MIR_HI)
        .collect();
    let mirror_txt = match peak_of(&mirror) {
        Some((_mi, mf, mp, mr)) => format!(
            "mirror 44-56 peak {:.3} mHz ({mr:.1}x) A={}",
            mf * 1e3,
            fmt_amp(amp_of(mp, dts.len()))
        ),
        None => "mirror 44-56 absent".to_string(),
    };

    let mut ref_txt: Vec<String> = Vec::new();
    for (st, rf, rname) in REF_FREQS {
        if st != station {
            continue;
        }
        let a = nearest_power(&grid, rf).and_then(|p| amp_of(p, dts.len()));
        ref_txt.push(format!("{rname} mHz {}", fmt_amp(a)));
    }
    for (st, rf, dname) in DOMINANTS {
        if st != station {
            continue;
        }
        let a = nearest_power(&grid, rf).and_then(|p| amp_of(p, dts.len()));
        ref_txt.push(format!("{dname} mHz {}", fmt_amp(a)));
    }
    let earlier = EARLIER_AMP
        .iter()
        .find(|(st, _)| *st == station)
        .map(|(_, v)| *v)
        .unwrap_or("absent");

    format!(
        "  {:<8} {:<9} n={n_raw:<6} detrend={:<6} peak {:.3} mHz ({ratio:.1}x) A={} | {mirror_txt} | earlier {earlier} | refs: {} | top-{TOP_MEMBERS}: {}",
        year.map_or("all".to_string(), |y| y.to_string()),
        class_name,
        dts.len(),
        f_peak * 1e3,
        fmt_amp(a_peak),
        ref_txt.join(" | "),
        if mem_txt.is_empty() {
            "none".to_string()
        } else {
            mem_txt.join(" | ")
        }
    )
}

fn detrend_aligned(ts: &[f64], vs: &[f64]) -> Vec<Option<f64>> {
    let mut out: Vec<Option<f64>> = vec![None; ts.len()];
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
                out[k] = Some(vs[k] - (slope * (ts[k] - mx) + my));
            }
        }
        lo = hi;
    }
    out
}

fn strength_tercile_report(station: i64, year: i64, recs: &[Rec]) -> Option<String> {
    let mut cell: Vec<(f64, f64, f64)> = recs
        .iter()
        .filter(|r| r.station == station && r.year == Some(year) && r.sampler < 10.0)
        .map(|r| (r.t, r.resid, r.strength))
        .collect();
    if cell.len() < MIN_N {
        return None;
    }
    cell.sort_by(|a, b| a.0.total_cmp(&b.0));
    let ts: Vec<f64> = cell.iter().map(|(t, _, _)| *t).collect();
    let vs: Vec<f64> = cell.iter().map(|(_, r, _)| *r).collect();
    let det = detrend_aligned(&ts, &vs);
    let mut binned: Vec<(f64, f64, f64)> = Vec::new();
    for (i, d) in det.iter().enumerate() {
        if let Some(v) = d {
            binned.push((cell[i].2, cell[i].0, *v));
        }
    }
    if binned.len() < MIN_N {
        return None;
    }
    binned.sort_by(|a, b| a.0.total_cmp(&b.0));
    let n = binned.len();
    let cuts = [0usize, n / 3, 2 * n / 3, n];
    let names = ["weak", "mid", "strong"];
    let mut amps: Vec<Option<f64>> = Vec::new();
    let mut parts: Vec<String> = Vec::new();
    for (k, name) in names.iter().enumerate() {
        let seg = &binned[cuts[k]..cuts[k + 1]];
        if seg.len() < MIN_N {
            amps.push(None);
            parts.push(format!("{name} absent (n={})", seg.len()));
            continue;
        }
        let sts: Vec<f64> = seg.iter().map(|(_, t, _)| *t).collect();
        let svs: Vec<f64> = seg.iter().map(|(_, _, v)| *v).collect();
        let grid = ls_grid(&sts, &svs, BAND_LO, BAND_HI, STEP);
        let a = peak_of(&grid).and_then(|(_, _, p, _)| amp_of(p, svs.len()));
        amps.push(a);
        parts.push(format!("{name} A={} (n={})", fmt_amp(a), seg.len()));
    }
    let ratio = match (amps[0], amps[2]) {
        (Some(w), Some(s)) if w > 0.0 && s > 0.0 => {
            let r = if w > s { w / s } else { s / w };
            format!("max/min={r:.3}")
        }
        _ => "constancy absent (a tercile carries no amplitude)".to_string(),
    };
    Some(format!(
        "  {year} strength terciles (sub-10-s, field 78 dBm): {} | {ratio}",
        parts.join(" | ")
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = arg_value(&args, "--atdf") else {
        eprintln!(
            "band_amplitude_probe needs --atdf <pioneer10_skyfreq.bin> — the census stays empty (0 honored)"
        );
        std::process::exit(2);
    };
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("{path} reads void — the census stays empty (0 honored)");
        return;
    };
    let Some(rows) = parse_bin(&bytes) else {
        eprintln!("{path} carries no PASF contract — the census stays empty (0 honored)");
        return;
    };
    let recs: Vec<Rec> = rows
        .iter()
        .filter(|r| r[8].is_finite() && r[0].is_finite() && r[3].is_finite())
        .map(|r| Rec {
            t: r[0],
            sampler: r[3],
            station: r[6] as i64,
            resid: r[8],
            strength: r[10],
            year: year_of(r[0]),
        })
        .collect();
    if recs.is_empty() {
        eprintln!("{path}: no finite-resid samples — the census stays empty (0 honored)");
        return;
    }

    println!("=== 160-Hz band-amplitude census (44-58 mHz) — the run's missing measurement ===");
    println!(
        "series : {path} (PASF; ATDF doppler_resid field r[8], TRK-2-25 Item 101 — the NOCC-corrected residual)"
    );
    println!(
        "method : per contiguous run (gap {GAP_RUN_S:.0} s, >= {MIN_RUN} samples) linear detrend; LS {:.0}-{:.0} mHz @ {:.2} mHz; floor = band median power; A = sqrt(2 P / n) Hz",
        BAND_LO * 1e3,
        BAND_HI * 1e3,
        STEP * 1e3
    );
    println!(
        "earlier: st14 160/153/161 Hz, st43 104/102/82 Hz, st63 57/50/65 Hz over weak->strong (probe-front-dark-matter.md) — the 160-Hz value pending this census"
    );
    println!(
        "mirror : 44-56 mHz subset carried by the same grid; top-{TOP_MEMBERS} members are local maxima, parabolic-interpolated"
    );
    println!();

    let mut years: Vec<i64> = recs.iter().filter_map(|r| r.year).collect();
    years.sort_unstable();
    years.dedup();

    for station in STATIONS {
        let earlier = EARLIER_AMP
            .iter()
            .find(|(st, _)| *st == station)
            .map(|(_, v)| *v)
            .unwrap_or("absent");
        println!("station {station} — earlier A = {earlier}:");
        let classes: [(&str, fn(f64) -> bool); 2] =
            [("strict-1", |s| s == 1.0), ("sub-10", |s| s < 10.0)];
        for (cname, cpred) in classes {
            for y in &years {
                let (ts, vs) = year_cell(&recs, station, cpred, Some(*y));
                println!("{}", cell_report(station, cname, Some(*y), &ts, &vs));
            }
        }
        if let Some(line) = strength_tercile_report(station, 1988, &recs) {
            println!("{line}");
        }
        println!();
    }

    println!("reference frequencies (earlier selected members):");
    for (st, rf, rname) in REF_FREQS {
        println!("  st{st}: {rname} mHz (grid bin {:.5} Hz)", rf);
    }
    println!("canonical dominants (§1, 1988 sub-10-s): st14 57.11 / st43 44.40 / st63 51.99 mHz");
    println!(
        "the 160-Hz re-measurement: the per-cell A above is the census value; the ratio A/A_ref closes the pending line when A is a dominant amplitude (peak column), not a fixed-frequency sample"
    );
}
