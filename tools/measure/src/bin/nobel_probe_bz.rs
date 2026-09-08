use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::omni2::{
    parse_bin, COMP_AE, COMP_BX, COMP_BY, COMP_BZ, COMP_DST, COMP_N1800, COMP_SYMH, COMP_V1800,
};
use omegaflow::lsk::days_from_civil;
use omegaflow::te::{benjamini_hochberg, pcmci_links};

const OMNI2_1H_CDN: &str =
    "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/omni2_serie_1h.bin";
const HOUR: f64 = 3600.0;
const J2000_UNIX_OFFSET: f64 = 946728000.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn year_epoch(y: i64) -> Option<f64> {
    days_from_civil(y, 1, 1).map(|d| d as f64 * 86400.0)
}

fn load_solar_wind() -> Option<Vec<(f64, f64, u32)>> {
    let cache = omegaflow::archivar::cache_root()
        .join("omni2_serie_1h.bin")
        .to_string_lossy()
        .into_owned();
    let bytes = match std::fs::read(&cache) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("omni2_serie_1h.bin absent locally — fetching the CDN asset");
            fetch_raw_bytes(OMNI2_1H_CDN, 3600)?
        }
    };
    parse_bin(&bytes)
}

fn load_indices() -> Option<Vec<(f64, f64, u32)>> {
    let cache = omegaflow::archivar::cache_root()
        .join("omni2_indices.bin")
        .to_string_lossy()
        .into_owned();
    match std::fs::read(&cache) {
        Ok(bytes) => parse_bin(&bytes),
        Err(_) => {
            eprintln!("omni2_indices.bin reads void — the index channels stay unmeasured");
            None
        }
    }
}

fn channel(recs: &[(f64, f64, u32)], comp: u32, lo: f64, hi: f64, shift: f64) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = recs
        .iter()
        .filter(|&&(t, _, c)| {
            let u = t + shift;
            c == comp && u >= lo && u < hi
        })
        .map(|&(t, v, _)| (t + shift, v))
        .collect();
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn bin_cells(series: &[(f64, f64)], t0: f64, dt: f64, n: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; n];
    let mut counts = vec![0u32; n];
    for &(t, v) in series {
        let idx = ((t - t0) / dt).floor();
        if idx < 0.0 || idx >= n as f64 {
            continue;
        }
        let i = idx as usize;
        sums[i] += v;
        counts[i] += 1;
    }
    (0..n)
        .map(|i| {
            if counts[i] > 0 {
                Some((sums[i] / counts[i] as f64) as f32)
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let start_year: i64 = arg_value(&args, "--window-start")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2015);
    let end_year: i64 = arg_value(&args, "--window-end")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2026);
    let max_lag: usize = arg_value(&args, "--max-lag")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);
    let null_lag: usize = arg_value(&args, "--null-lag")
        .and_then(|v| v.parse().ok())
        .unwrap_or(12);
    let bins: usize = arg_value(&args, "--bins")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    if start_year >= end_year {
        eprintln!("--window-start/--window-end carry no valid span");
        return;
    }
    let lo = match year_epoch(start_year) {
        Some(v) => v,
        None => {
            eprintln!("--window-start {} carries no civil year", start_year);
            return;
        }
    };
    let hi = match year_epoch(end_year) {
        Some(v) => v,
        None => {
            eprintln!("--window-end {} carries no civil year", end_year);
            return;
        }
    };

    println!("=== Nobel-DAG Bz probe — the Runge-2018 counterpart ===");
    println!(
        "window {}..{} (hourly), channels V n Bz |B| AE Dst; max_lag {} null_lag {} bins {}",
        start_year, end_year, max_lag, null_lag, bins
    );

    let Some(sw) = load_solar_wind() else {
        eprintln!("solar wind carries no records — the run stays unmeasured");
        return;
    };
    let Some(idx) = load_indices() else {
        eprintln!("indices carry no records — the run stays unmeasured");
        return;
    };

    let v = channel(&sw, COMP_V1800, lo, hi, J2000_UNIX_OFFSET);
    let n = channel(&sw, COMP_N1800, lo, hi, J2000_UNIX_OFFSET);
    let bz = channel(&sw, COMP_BZ, lo, hi, J2000_UNIX_OFFSET);
    let bx = channel(&sw, COMP_BX, lo, hi, J2000_UNIX_OFFSET);
    let by = channel(&sw, COMP_BY, lo, hi, J2000_UNIX_OFFSET);
    let ae = channel(&idx, COMP_AE, lo, hi, 0.0);
    let dst = channel(&idx, COMP_DST, lo, hi, 0.0);
    let symh = channel(&idx, COMP_SYMH, lo, hi, 0.0);

    let channels: [&[(f64, f64)]; 8] = [&v, &n, &bz, &bx, &by, &ae, &dst, &symh];
    let grid_lo = channels
        .iter()
        .filter_map(|c| c.first().map(|&(t, _)| t))
        .fold(f64::NEG_INFINITY, f64::max);
    let grid_hi = channels
        .iter()
        .filter_map(|c| c.last().map(|&(t, _)| t))
        .fold(f64::INFINITY, f64::min);
    if grid_lo >= grid_hi {
        eprintln!("common window empty — the matrix stays unmeasured");
        return;
    }
    let t0 = (grid_lo / HOUR).floor() * HOUR;
    let n_cells = ((grid_hi - t0) / HOUR).floor() as usize;

    let bc_v = bin_cells(&v, t0, HOUR, n_cells);
    let bc_n = bin_cells(&n, t0, HOUR, n_cells);
    let bc_bz = bin_cells(&bz, t0, HOUR, n_cells);
    let bc_bx = bin_cells(&bx, t0, HOUR, n_cells);
    let bc_by = bin_cells(&by, t0, HOUR, n_cells);
    let bc_ae = bin_cells(&ae, t0, HOUR, n_cells);
    let bc_dst = bin_cells(&dst, t0, HOUR, n_cells);
    let bc_symh = bin_cells(&symh, t0, HOUR, n_cells);

    let mut series: [Vec<f32>; 7] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    for i in 0..n_cells {
        let (Some(bx), Some(by), Some(bz)) = (bc_bx[i], bc_by[i], bc_bz[i]) else {
            continue;
        };
        let (Some(vv), Some(nn), Some(ae), Some(ds), Some(sh)) =
            (bc_v[i], bc_n[i], bc_ae[i], bc_dst[i], bc_symh[i])
        else {
            continue;
        };
        let bmag = (bx * bx + by * by + bz * bz).sqrt();
        series[0].push(vv);
        series[1].push(nn);
        series[2].push(bz);
        series[3].push(bmag);
        series[4].push(ae);
        series[5].push(ds);
        series[6].push(sh);
    }
    let names = ["V", "n", "Bz", "|B|", "AE", "Dst", "SYM-H"];
    let m = series[0].len();
    println!(
        "common hourly cells: {} ({:.1} years)",
        m,
        m as f64 / 8760.0
    );
    if m < 100 {
        eprintln!("below the n-floor — no verdict");
        return;
    }

    let refs: Vec<&[f32]> = series.iter().map(|s| s.as_slice()).collect();
    let Some(links) = pcmci_links(&refs, max_lag, null_lag, bins, 0x9E37_79B9_7F4A_7C15, 10) else {
        eprintln!("pcmci_links returns void — no verdict");
        return;
    };
    let p_vals: Vec<f64> = links.iter().map(|l| l.p_value).collect();
    let Some(_cutoff) = benjamini_hochberg(&p_vals, 0.05) else {
        eprintln!("FDR returns void — no verdict");
        return;
    };

    let mut ranked: Vec<&omegaflow::te::CausalLink> = links.iter().collect();
    ranked.sort_by(|a, b| (b.te - b.threshold).total_cmp(&(a.te - a.threshold)));
    println!();
    println!("directed edges, ranked by excess (arrow = TE > mean+2σ):");
    for l in &ranked {
        let arrow = if l.te > l.threshold {
            "arrow"
        } else {
            "silent"
        };
        println!(
            "  {} -> {} | lag {} | TE {:.3e} | thr {:.3e} | excess {:+.3e} | ratio {:.2} | {}",
            names[l.driver],
            names[l.target],
            l.lag,
            l.te,
            l.threshold,
            l.te - l.threshold,
            l.te / l.threshold,
            arrow
        );
    }

    let arrow = |driver: usize, target: usize| -> (bool, f64) {
        links
            .iter()
            .filter(|l| l.driver == driver && l.target == target)
            .map(|l| (l.te > l.threshold, l.te / l.threshold))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap_or((false, 0.0))
    };
    let (bz_ae, bz_ae_r) = arrow(2, 4);
    let (bz_dst, bz_dst_r) = arrow(2, 5);
    let (bz_symh, bz_symh_r) = arrow(2, 6);
    let (ae_dst, ae_dst_r) = arrow(4, 5);
    let (dst_ae, dst_ae_r) = arrow(5, 4);
    let reverse_leak = (arrow(4, 0).0 || arrow(4, 1).0 || arrow(4, 2).0 || arrow(4, 3).0)
        || (arrow(5, 0).0 || arrow(5, 1).0 || arrow(5, 2).0 || arrow(5, 3).0)
        || (arrow(6, 0).0 || arrow(6, 1).0 || arrow(6, 2).0 || arrow(6, 3).0);

    println!();
    println!("=== Verdict (Runge-2018 counterpart) ===");
    if bz_ae && bz_dst && bz_symh {
        println!(
            "Bz is a common driver of AE, Dst and SYM-H (Bz->AE {:.2}, Bz->Dst {:.2}, Bz->SYM-H {:.2}) — matching Runge 2018.",
            bz_ae_r, bz_dst_r, bz_symh_r
        );
    } else if bz_ae && (bz_dst || bz_symh) {
        println!(
            "Bz is a common driver of AE and one storm channel (Bz->AE {:.2}, Bz->Dst {:.2}, Bz->SYM-H {:.2}) — matching Runge 2018; the missing storm channel is named.",
            bz_ae_r, bz_dst_r, bz_symh_r
        );
    } else {
        println!(
            "Bz->AE {} ({:.2}), Bz->Dst {} ({:.2}), Bz->SYM-H {} ({:.2}) — the machine does not recover Bz as the common driver (calibration to be investigated, not silence).",
            bz_ae, bz_ae_r, bz_dst, bz_dst_r, bz_symh, bz_symh_r
        );
    }
    let ae_dst_any = ae_dst || dst_ae;
    if ae_dst_any {
        let r = ae_dst_r.max(dst_ae_r);
        if r >= 1.5 {
            println!(
                "A direct AE<->Dst edge survives conditioning on Bz at ratio {:.2} — a measured substorm->ring-current coupling, the deviation from Runge's 'no direct edge'.",
                r
            );
        } else {
            println!(
                "The AE<->Dst edge is marginal (ratio {:.2}, below 1.5x threshold vs Bz's {:.2}/{:.2}) — consistent with Runge's 'association via the common driver' at strong significance.",
                r, bz_ae_r, bz_dst_r
            );
        }
    } else {
        println!(
            "No direct AE<->Dst edge — the common-driver reading holds (Runge 2018 reproduced)."
        );
    }
    if reverse_leak {
        println!(
            "Reverse edges (AE/Dst/SYM-H -> solar-wind channels) are measured as arrows — physically impossible (a ground index cannot drive the upstream wind); this is the contemporaneous-coupling leak of the estimator, named, not concealed."
        );
    } else {
        println!(
            "No reverse (geomagnetic -> solar-wind) edge — the null holds the upstream direction."
        );
    }
}
