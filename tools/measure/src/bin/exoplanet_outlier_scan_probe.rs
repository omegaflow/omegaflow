use std::process::Command;

const UA: &str = "omegaflow-measure-exoplanet-outlier-scan/1.0";
const TAP_SYNC: &str = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync";
const SCAN_DATE: &str = "2026-09-05";
const NAME_HEADER: &str = "pl_name";
const MIN_AXES: usize = 4;
const TOP_OUTLIERS: usize = 30;
const CORR_N_FLOOR: usize = 60;
const TAP_QUERY: &str = "SELECT pl_name,pl_rade,pl_masse,pl_orbper,pl_orbeccen,pl_eqt,pl_dens,st_mass,st_teff,st_met,sy_dist FROM ps WHERE default_flag=1 AND pl_name IS NOT NULL ORDER BY pl_name";
const NUM_COLS: [&str; 10] = [
    "pl_rade",
    "pl_masse",
    "pl_orbper",
    "pl_orbeccen",
    "pl_eqt",
    "pl_dens",
    "st_mass",
    "st_teff",
    "st_met",
    "sy_dist",
];

struct Axis {
    col: usize,
    name: &'static str,
    unit: &'static str,
    log: bool,
}

const AXES: [Axis; 7] = [
    Axis { col: 0, name: "radius", unit: "R_E", log: true },
    Axis { col: 1, name: "mass", unit: "M_E", log: true },
    Axis { col: 2, name: "period", unit: "d", log: true },
    Axis { col: 5, name: "density", unit: "g/cm3", log: true },
    Axis { col: 4, name: "teq", unit: "K", log: true },
    Axis { col: 3, name: "ecc", unit: "", log: false },
    Axis { col: 7, name: "host_teff", unit: "K", log: true },
];

struct Planet {
    name: String,
    vals: [Option<f64>; 10],
}

struct AxisRead {
    name: &'static str,
    unit: &'static str,
    col: usize,
    log: bool,
    n: usize,
    sorted: Vec<f64>,
    center: f64,
    sigma: f64,
    zero_pile: usize,
}

struct Corr {
    a: &'static str,
    b: &'static str,
    rho: f64,
    n: usize,
}

struct OutlierRow {
    index: usize,
    score: f64,
    zs: Vec<(usize, f64)>,
}

struct StructureBullet {
    name: &'static str,
    width: f64,
    below: usize,
    above: usize,
    log: bool,
}

fn fetch_csv() -> Result<String, String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("300")
        .arg("-A")
        .arg(UA)
        .arg("-G")
        .arg(TAP_SYNC)
        .arg("--data-urlencode")
        .arg(format!("query={TAP_QUERY}"))
        .arg("--data-urlencode")
        .arg("format=csv")
        .arg("-w")
        .arg("\n%{http_code}");
    let out = cmd.output().map_err(|e| format!("curl: {e}"))?;
    let stdout = out.stdout;
    if stdout.is_empty() {
        return Err("curl without bytes".to_string());
    }
    let idx = stdout
        .iter()
        .rposition(|&b| b == b'\n')
        .ok_or_else(|| "curl reply without newline".to_string())?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..]).trim().to_string();
    let body = String::from_utf8_lossy(&stdout[..idx]).to_string();
    if code != "200" {
        return Err(format!("TAP HTTP {code}"));
    }
    Ok(body)
}

fn split_csv(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else {
                    quoted = false;
                }
            } else {
                current.push(c);
            }
        } else if c == '"' {
            quoted = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    fields.push(current);
    fields
}

fn parse_cell(cell: &str) -> Option<f64> {
    let trimmed = cell.trim();
    if trimmed.is_empty() {
        return None;
    }
    let value = match trimmed.parse::<f64>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    if value.is_finite() {
        Some(value)
    } else {
        None
    }
}

fn parse_rows(text: &str) -> Result<(Vec<Planet>, usize), String> {
    let mut lines = text.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => return Err("TAP reply without header line".to_string()),
    };
    let header_cells = split_csv(header);
    let name_pos = match header_cells.iter().position(|c| c.trim() == NAME_HEADER) {
        Some(p) => p,
        None => return Err("TAP header without pl_name column".to_string()),
    };
    let mut col_pos = [0usize; 10];
    for (i, name) in NUM_COLS.iter().enumerate() {
        let pos = match header_cells.iter().position(|c| c.trim() == *name) {
            Some(p) => p,
            None => return Err(format!("TAP header without {name} column")),
        };
        col_pos[i] = pos;
    }
    let mut planets: Vec<Planet> = Vec::new();
    let mut skipped = 0usize;
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let cells = split_csv(trimmed);
        let name_cell = match cells.get(name_pos) {
            Some(c) => c.trim(),
            None => {
                skipped += 1;
                continue;
            }
        };
        if name_cell.is_empty() {
            skipped += 1;
            continue;
        }
        let mut vals = [None; 10];
        for (i, pos) in col_pos.iter().enumerate() {
            let cell = match cells.get(*pos) {
                Some(c) => c,
                None => continue,
            };
            vals[i] = parse_cell(cell);
        }
        planets.push(Planet {
            name: name_cell.to_string(),
            vals,
        });
    }
    Ok((planets, skipped))
}

fn transform(planet: &Planet, col: usize, log: bool) -> Option<f64> {
    let raw = planet.vals[col]?;
    if log {
        if raw > 0.0 {
            Some(raw.log10())
        } else {
            None
        }
    } else {
        Some(raw)
    }
}

fn median_of_sorted(sorted: &[f64]) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let n = sorted.len();
    if n % 2 == 1 {
        Some(sorted[n / 2])
    } else {
        Some(0.5 * (sorted[n / 2 - 1] + sorted[n / 2]))
    }
}

fn q_at(sorted: &[f64], q: f64) -> f64 {
    let n = sorted.len();
    if n == 1 {
        return sorted[0];
    }
    let pos = q * (n - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    let frac = pos - lo as f64;
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

fn median_positive_gap(sorted: &[f64]) -> Option<f64> {
    let mut gaps: Vec<f64> = Vec::new();
    for pair in sorted.windows(2) {
        let gap = pair[1] - pair[0];
        if gap > 0.0 {
            gaps.push(gap);
        }
    }
    if gaps.is_empty() {
        return None;
    }
    gaps.sort_by(f64::total_cmp);
    median_of_sorted(&gaps)
}

fn axis_read(planets: &[Planet], ax: &Axis) -> Option<AxisRead> {
    let mut carried: Vec<f64> = planets
        .iter()
        .filter_map(|p| transform(p, ax.col, ax.log))
        .collect();
    if carried.is_empty() {
        return None;
    }
    carried.sort_by(f64::total_cmp);
    let center = median_of_sorted(&carried)?;
    let mut dev: Vec<f64> = carried.iter().map(|x| (x - center).abs()).collect();
    dev.sort_by(f64::total_cmp);
    let mad = median_of_sorted(&dev)?;
    let sigma = 1.4826 * mad;
    if !(sigma > 0.0) {
        return None;
    }
    let zero_pile = planets
        .iter()
        .filter(|p| p.vals[ax.col] == Some(0.0))
        .count();
    Some(AxisRead {
        name: ax.name,
        unit: ax.unit,
        col: ax.col,
        log: ax.log,
        n: carried.len(),
        sorted: carried,
        center,
        sigma,
        zero_pile,
    })
}

fn axis_z(read: &AxisRead, planet: &Planet) -> Option<f64> {
    let x = transform(planet, read.col, read.log)?;
    Some((x - read.center) / read.sigma)
}

fn raw_median(planets: &[Planet], col: usize) -> Option<f64> {
    let mut values: Vec<f64> = planets.iter().filter_map(|p| p.vals[col]).collect();
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    median_of_sorted(&values)
}

fn fmt_sig(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    let a = v.abs();
    if a >= 10000.0 || a < 1e-3 {
        return format!("{:.2e}", v);
    }
    if a >= 1000.0 {
        return format!("{:.1}", v);
    }
    let mut s = format!("{:.4}", v);
    while s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
}

fn pct(part: usize, whole: usize) -> f64 {
    100.0 * part as f64 / whole as f64
}

fn pair_values(planets: &[Planet], a: usize, b: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for p in planets {
        match (p.vals[a], p.vals[b]) {
            (Some(x), Some(y)) => out.push((x, y)),
            _ => {}
        }
    }
    out
}

fn ranks(vals: &[f64]) -> Vec<f64> {
    let n = vals.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| vals[i].total_cmp(&vals[j]));
    let mut out = vec![0.0; n];
    let mut i = 0usize;
    while i < n {
        let mut j = i;
        while j + 1 < n && vals[order[j + 1]] == vals[order[i]] {
            j += 1;
        }
        let avg = 1.0 + (i + j) as f64 / 2.0;
        for k in i..=j {
            out[order[k]] = avg;
        }
        i = j + 1;
    }
    out
}

fn rho_of_pairs(pairs: &[(f64, f64)]) -> Option<f64> {
    let n = pairs.len();
    if n < 2 {
        return None;
    }
    let xs: Vec<f64> = pairs.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pairs.iter().map(|p| p.1).collect();
    let rx = ranks(&xs);
    let ry = ranks(&ys);
    let mx = rx.iter().sum::<f64>() / n as f64;
    let my = ry.iter().sum::<f64>() / n as f64;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for k in 0..n {
        let dx = rx[k] - mx;
        let dy = ry[k] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx == 0.0 || syy == 0.0 {
        return None;
    }
    Some(sxy / (sxx.sqrt() * syy.sqrt()))
}

fn build_report(planets: &[Planet], skipped: usize) -> String {
    let total = planets.len();
    let reads: Vec<AxisRead> = AXES.iter().filter_map(|ax| axis_read(planets, ax)).collect();
    let mut report = String::new();
    report.push_str(
        "EXOPLANET OUTLIER SCAN — hypothesis-free structure scan of the archive exoplanet population\n",
    );
    report.push_str(&format!(
        "sample: NASA Exoplanet Archive TAP, ps composite, default_flag=1 (one row per planet, all detection channels, confirmed per archive ps)\n"
    ));
    report.push_str(&format!("query: {TAP_QUERY}\n"));
    report.push_str(&format!(
        "scan date: {SCAN_DATE} | planets parsed: {total} | rows skipped (nameless/empty): {skipped}\n"
    ));
    report.push_str(
        "empty CSV cells are read as absent (Option/None); no cell is filled with a fabricated value\n",
    );

    report.push_str("\nCOVERAGE (n carrying the column / total)\n");
    for (i, name) in NUM_COLS.iter().enumerate() {
        let carried = planets.iter().filter(|p| p.vals[i].is_some()).count();
        let med = raw_median(planets, i);
        let med_txt = match med {
            Some(m) => fmt_sig(m),
            None => "absent".to_string(),
        };
        report.push_str(&format!(
            "  {name:<12} n={carried:<5} ({:.1}%)  median={med_txt}\n",
            pct(carried, total)
        ));
    }

    report.push_str("\nOUTLIER SCAN — single-case exceptions against the main cloud\n");
    report.push_str(
        "method: per-axis robust z = (x - median)/(1.4826*MAD) on the axis-carrying population; radius, mass, period, density, teq, host_teff in log10; eccentricity on the linear axis because reported 0.0 is a boundary pile; planet score = rms of its per-axis z; planets carrying fewer than 4 of the 7 axes are not scored\n",
    );
    for read in &reads {
        let scale_txt = if read.log { "dex" } else { "" };
        let sigma_disp = fmt_sig(read.sigma);
        let tail4 = read
            .sorted
            .iter()
            .filter(|x| ((**x - read.center) / read.sigma).abs() > 4.0)
            .count();
        report.push_str(&format!(
            "  {} (n={}): center {} {} | robust sigma {} {} | |z|>4 carriers {}\n",
            read.name,
            read.n,
            fmt_sig(if read.log {
                10f64.powf(read.center)
            } else {
                read.center
            }),
            read.unit,
            sigma_disp,
            scale_txt,
            tail4
        ));
    }

    let mut scored: Vec<OutlierRow> = Vec::new();
    for (p_idx, p) in planets.iter().enumerate() {
        let mut zs: Vec<(usize, f64)> = Vec::new();
        for (r_idx, read) in reads.iter().enumerate() {
            if let Some(z) = axis_z(read, p) {
                zs.push((r_idx, z));
            }
        }
        if zs.len() >= MIN_AXES {
            let sum_sq: f64 = zs.iter().map(|&(_, z)| z * z).sum();
            let score = (sum_sq / zs.len() as f64).sqrt();
            scored.push(OutlierRow {
                index: p_idx,
                score,
                zs,
            });
        }
    }
    scored.sort_by(|a, b| b.score.total_cmp(&a.score));
    report.push_str(&format!(
        "\nscored population: {} planets carrying >= {} axes ({} total)\n",
        scored.len(),
        MIN_AXES,
        total
    ));
    report.push_str(&format!("TOP {} SINGLE-CASE OUTLIERS (rms robust z)\n", TOP_OUTLIERS));
    let shown = scored.len().min(TOP_OUTLIERS);
    for (rank, row) in scored.iter().take(shown).enumerate() {
        let p = &planets[row.index];
        let mut zs_sorted = row.zs.clone();
        zs_sorted.sort_by(|a, b| b.1.abs().total_cmp(&a.1.abs()));
        let mut axes_txt = Vec::new();
        for &(r_idx, z) in zs_sorted.iter().take(4) {
            let read = &reads[r_idx];
            let raw = match p.vals[read.col] {
                Some(v) => v,
                None => continue,
            };
            axes_txt.push(format!(
                "{} {} {} (z {:+.1})",
                read.name,
                fmt_sig(raw),
                read.unit.trim(),
                z
            ));
        }
        report.push_str(&format!(
            "{:>2}. {:<22} rms_z {:<6.2} carried {}/{} | {}\n",
            rank + 1,
            p.name,
            row.score,
            row.zs.len(),
            reads.len(),
            axes_txt.join(", ")
        ));
    }

    report.push_str("\nSTRUCTURES — data-driven reads per axis (measured interior bands)\n");
    let mut bullets: Vec<StructureBullet> = Vec::new();
    for read in &reads {
        let sorted = &read.sorted;
        let n = read.n;
        if n < 2 {
            continue;
        }
        let q05 = q_at(sorted, 0.05);
        let q95 = q_at(sorted, 0.95);
        let q10 = q_at(sorted, 0.10);
        let q90 = q_at(sorted, 0.90);
        let nat = |x: f64| -> f64 {
            if read.log {
                10f64.powf(x)
            } else {
                x
            }
        };
        let zp_pct = pct(read.zero_pile, read.n);
        let med_nat = nat(read.center);
        let band = {
            let mut best: Option<(usize, f64, f64)> = None;
            for i in 0..sorted.len() - 1 {
                let left = sorted[i];
                let right = sorted[i + 1];
                if left < q10 || right > q90 {
                    continue;
                }
                let width = right - left;
                match best {
                    None => best = Some((i, left, width)),
                    Some((_, _, bw)) if width > bw => best = Some((i, left, width)),
                    Some(_) => {}
                }
            }
            best
        };
        let tau = median_positive_gap(sorted);
        report.push_str(&format!(
            "  {}: q05..q95 {}..{} {} | median {} {} | robust sigma {} {} | exact-0 pile {} ({:.1}%)\n",
            read.name,
            fmt_sig(nat(q05)),
            fmt_sig(nat(q95)),
            read.unit,
            fmt_sig(med_nat),
            read.unit,
            fmt_sig(read.sigma),
            if read.log { "dex" } else { "" },
            read.zero_pile,
            zp_pct
        ));
        if let Some((i, left, width)) = band {
            let right = sorted[i + 1];
            let below = i + 1;
            let above = n - below;
            let bf = pct(below, n);
            let af = pct(above, n);
            let left_nat = nat(left);
            let right_nat = nat(right);
            let width_txt = if read.log {
                format!(
                    "{}x ({}..{} {})",
                    fmt_sig(10f64.powf(width)),
                    fmt_sig(left_nat),
                    fmt_sig(right_nat),
                    read.unit
                )
            } else {
                format!(
                    "{} ({}..{} {})",
                    fmt_sig(right - left),
                    fmt_sig(left_nat),
                    fmt_sig(right_nat),
                    read.unit
                )
            };
            let mut read_label = String::new();
            let mut is_void = false;
            if let Some(t) = tau {
                let ratio = width / t;
                let mut line = format!(
                    "    largest interior band: width {width_txt}, {:.0}x median neighbour spacing, split {below}/{above} ({:.1}%/{:.1}%)\n",
                    ratio, bf, af
                );
                if ratio >= 20.0 {
                    is_void = true;
                    let small = bf.min(af);
                    if small >= 10.0 {
                        read_label.push_str(" -> two interior clouds");
                    } else if small >= 3.0 {
                        read_label.push_str(" -> minor far-side component");
                    } else {
                        read_label.push_str(" -> sparse far side");
                    }
                    line = line.trim_end_matches('\n').to_string();
                    line.push_str(&read_label);
                    line.push('\n');
                }
                report.push_str(&line);
            } else {
                report.push_str(&format!(
                    "    largest interior band: width {width_txt}, split {below}/{above} ({:.1}%/{:.1}%)\n",
                    bf, af
                ));
            }
            if is_void {
                bullets.push(StructureBullet {
                    name: read.name,
                    width,
                    below,
                    above,
                    log: read.log,
                });
            }
        } else {
            report.push_str("    no interior band measurable on this axis\n");
        }
    }
    if bullets.is_empty() {
        report.push_str("  no interior band exceeds 20x the median neighbour spacing on any axis\n");
    } else {
        report.push_str("  interior bands exceeding 20x the median neighbour spacing:\n");
        for b in &bullets {
            let nat_width = if b.log {
                format!("{}x", fmt_sig(10f64.powf(b.width)))
            } else {
                fmt_sig(b.width)
            };
            report.push_str(&format!(
                "    {}: interior band width {}, split {} below / {} above\n",
                b.name, nat_width, b.below, b.above
            ));
        }
    }

    report.push_str("\nPAIRWISE CORRELATIONS (Spearman rank rho, pairwise complete case)\n");
    let mut corrs: Vec<Corr> = Vec::new();
    for a in 0..10 {
        for b in (a + 1)..10 {
            let pairs = pair_values(planets, a, b);
            if pairs.len() < CORR_N_FLOOR {
                continue;
            }
            let rho = match rho_of_pairs(&pairs) {
                Some(r) => r,
                None => continue,
            };
            corrs.push(Corr {
                a: NUM_COLS[a],
                b: NUM_COLS[b],
                rho,
                n: pairs.len(),
            });
        }
    }
    corrs.sort_by(|x, y| y.rho.abs().total_cmp(&x.rho.abs()));
    report.push_str(&format!(
        "  pairs with n >= {CORR_N_FLOOR}: {}\n",
        corrs.len()
    ));
    for c in &corrs {
        report.push_str(&format!(
            "  {:<12} x {:<12} rho {:+.3}   n {}\n",
            c.a, c.b, c.rho, c.n
        ));
    }

    let dcol = 5usize;
    let near_zero: Vec<&Corr> = corrs
        .iter()
        .filter(|c| c.n >= 800 && c.rho.abs() < 0.05)
        .collect();
    report.push_str("\nmeasured near-zero at n >= 800 (|rho| < 0.05):\n");
    if near_zero.is_empty() {
        report.push_str("  none measured\n");
    } else {
        for c in &near_zero {
            report.push_str(&format!("  {} x {} rho {:+.3} n {}\n", c.a, c.b, c.rho, c.n));
        }
    }

    let independent: Vec<&Corr> = corrs
        .iter()
        .filter(|c| {
            let a_is_dens = c.a == NUM_COLS[dcol];
            let b_is_dens = c.b == NUM_COLS[dcol];
            let a_is_rad = c.a == NUM_COLS[0];
            let b_is_rad = c.b == NUM_COLS[0];
            let a_is_mass = c.a == NUM_COLS[1];
            let b_is_mass = c.b == NUM_COLS[1];
            let a_is_dist = c.a == NUM_COLS[9];
            let b_is_dist = c.b == NUM_COLS[9];
            let density_link = (a_is_dens && (b_is_rad || b_is_mass))
                || (b_is_dens && (a_is_rad || a_is_mass));
            let dist_link = a_is_dist || b_is_dist;
            !density_link && !dist_link
        })
        .take(6)
        .collect();
    report.push_str("\nstrongest measured associations without a density/algebraic or distance/selection link:\n");
    for c in &independent {
        report.push_str(&format!("  {} x {} rho {:+.3} n {}\n", c.a, c.b, c.rho, c.n));
    }

    report.push_str("\nHONEST LIMITS\n");
    for (i, name) in NUM_COLS.iter().enumerate() {
        let carried = planets.iter().filter(|p| p.vals[i].is_some()).count();
        if pct(carried, total) < 50.0 {
            report.push_str(&format!(
                "  {name}: carries only {carried}/{total} planets ({:.1}%); reads on it rest on that subset\n",
                pct(carried, total)
            ));
        }
    }
    if let Some(ecc) = reads.iter().find(|r| r.name == "ecc") {
        report.push_str(&format!(
            "  eccentricity: {:.1}% of the carried sample reports exactly 0.0 (the circular boundary); the ecc scans apply to the carried subset\n",
            pct(ecc.zero_pile, ecc.n)
        ));
    }
    report.push_str(
        "  density is derived from mass and radius; its rho against those two is algebraic, not an independent measurement\n",
    );
    report.push_str(
        "  sy_dist pairs reflect the reachable volume of each detection channel; they are sample geometry, not a planet property\n",
    );
    report.push_str(
        "  the sample mixes detection channels as the archive reports them; the reads are of that mixed population\n",
    );
    report
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path = "tmp/exoplanet_outlier_scan_report.txt".to_string();
    let mut csv_path: Option<String> = None;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--csv" => {
                i += 1;
                csv_path = args.get(i).cloned();
            }
            "--out" => {
                i += 1;
                if let Some(v) = args.get(i).cloned() {
                    out_path = v;
                }
            }
            other => {
                eprintln!("exoplanet_outlier_scan_probe: unknown argument {other}");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    match run(&csv_path, &out_path) {
        Ok(()) => {}
        Err(msg) => {
            eprintln!("exoplanet_outlier_scan_probe: {msg}");
            std::process::exit(1);
        }
    }
}

fn run(csv_path: &Option<String>, out_path: &str) -> Result<(), String> {
    let text = match csv_path {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|e| format!("read {path}: {e}"))?,
        None => fetch_csv()?,
    };
    let (planets, skipped) = parse_rows(&text)?;
    if planets.is_empty() {
        return Err("no planet rows parsed from the reply".to_string());
    }
    let report = build_report(&planets, skipped);
    std::fs::write(out_path, &report).map_err(|e| format!("write {out_path}: {e}"))?;
    print!("{report}");
    Ok(())
}
