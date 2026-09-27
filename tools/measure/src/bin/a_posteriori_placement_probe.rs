const F32_EPS: f64 = 1.19e-7;
const OMEGA_FLOOR: f64 = 1e-30;
const EXTENT: f64 = 0.05;
const X0: f64 = 0.05;
const X1: f64 = 4.0;
const X_TAIL: f64 = 1.0;
const GRID_STEP: f64 = 1e-4;
const REF_SAMPLES: usize = 4000;
const INTERVAL_SAMPLES: usize = 8;
const POINT_CAP: usize = 5_000_000;

fn omega(x: f64) -> f64 {
    1.0 / (x * x + EXTENT * EXTENT)
}

fn omega_grad(x: f64) -> f64 {
    let denom = x * x + EXTENT * EXTENT;
    -2.0 * x / (denom * denom)
}

fn r_struct(x: f64) -> Option<f64> {
    let o = omega(x);
    let g = omega_grad(x);
    if !o.is_finite() || !g.is_finite() || g == 0.0 {
        return None;
    }
    let r = F32_EPS * o.abs() / g.abs();
    if r.is_finite() { Some(r) } else { None }
}

fn analytic_points(tol: f64) -> Option<Vec<f64>> {
    let mut pts: Vec<f64> = Vec::new();
    let mut x = X0;
    while x <= X1 {
        pts.push(x);
        if pts.len() > POINT_CAP {
            return None;
        }
        let r = r_struct(x)?;
        let h = (r * tol / F32_EPS).max(GRID_STEP);
        if !(h > 0.0) {
            return None;
        }
        x += h;
    }
    Some(pts)
}

fn nearest(pts: &[f64], x: f64) -> Option<f64> {
    if pts.is_empty() {
        return None;
    }
    let idx = pts.partition_point(|p| *p < x);
    let left = if idx > 0 { Some(pts[idx - 1]) } else { None };
    let right = if idx < pts.len() {
        Some(pts[idx])
    } else {
        None
    };
    match (left, right) {
        (Some(l), Some(r)) => Some(if (x - l).abs() <= (r - x).abs() { l } else { r }),
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}

fn rel_error_at(pts: &[f64], x: f64) -> Option<f64> {
    let o = omega(x);
    if !o.is_finite() || o.abs() <= OMEGA_FLOOR {
        return None;
    }
    let p = nearest(pts, x)?;
    let op = omega(p);
    if !op.is_finite() {
        return None;
    }
    let e = ((o - op) / o).abs();
    if e.is_finite() { Some(e) } else { None }
}

fn max_rel_error(pts: &[f64], lo: f64, hi: f64) -> Option<f64> {
    let mut worst: Option<f64> = None;
    for k in 0..=REF_SAMPLES {
        let x = lo + (hi - lo) * (k as f64 / REF_SAMPLES as f64);
        if let Some(e) = rel_error_at(pts, x) {
            worst = Some(match worst {
                Some(w) => w.max(e),
                None => e,
            });
        }
    }
    worst
}

fn interval_rel_error(lo: f64, hi: f64, mid: f64) -> f64 {
    let om = omega(mid);
    if !om.is_finite() || om.abs() <= OMEGA_FLOOR {
        return 0.0;
    }
    let mut worst = 0.0;
    for k in 0..=INTERVAL_SAMPLES {
        let x = lo + (hi - lo) * (k as f64 / INTERVAL_SAMPLES as f64);
        if let Some(e) = rel_error_at(&[mid], x) {
            if e > worst {
                worst = e;
            }
        }
    }
    worst
}

fn adaptive_points(tol: f64) -> Vec<f64> {
    let mut leaves: Vec<f64> = Vec::new();
    let mut stack: Vec<(f64, f64)> = vec![(X0, X1)];
    while let Some((lo, hi)) = stack.pop() {
        if leaves.len() > POINT_CAP {
            break;
        }
        let mid = 0.5 * (lo + hi);
        if hi - lo > GRID_STEP && interval_rel_error(lo, hi, mid) > tol {
            stack.push((lo, mid));
            stack.push((mid, hi));
        } else {
            leaves.push(mid);
        }
    }
    leaves.sort_by(|a, b| a.total_cmp(b));
    leaves
}

fn main() {
    println!(
        "a-posteriori placement probe | 1D radial, kernel 0 (1/(d^2+e^2)), extent {EXTENT}, domain [{X0}, {X1}], gridStep {GRID_STEP}"
    );
    println!(
        "rule: R_struct = F32_EPS*|Omega|/|Omega'| (src/mathematikerin/tests.rs:2228); analytic h = max(gridStep, tol*|Omega|/|Omega'|)"
    );
    println!(
        "metric: piecewise-constant nearest-sample relative error, floor Omega <= {OMEGA_FLOOR}"
    );
    println!(
        "note: 1D radial fixture; 2D Voronoi/quadtree topology and constant factors are the named next step"
    );
    println!();
    println!(
        "{:>9} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "tol", "N_analytic", "N_adaptiv", "ratio", "e_tail_ana", "e_tail_ada"
    );
    println!(
        "(N_adaptiv is built to the analytic's own achieved tail error e_tail_ana — equal error, not equal tol)"
    );

    let tols = [1e-1_f64, 1e-2, 1e-3, 1e-4];
    let mut min_ratio = f64::INFINITY;
    let mut max_ratio = f64::NEG_INFINITY;
    let mut measured_rows = 0usize;

    for &tol in &tols {
        let Some(pa) = analytic_points(tol) else {
            println!("{tol:>9.1e}  analytic point cap reached, tol pending");
            continue;
        };
        let Some(ea_tail) = max_rel_error(&pa, X_TAIL, X1) else {
            println!("{tol:>9.1e}  analytic tail error undefined, tol pending");
            continue;
        };
        let pq = adaptive_points(ea_tail);
        let eq_tail = max_rel_error(&pq, X_TAIL, X1);
        if pa.is_empty() || pq.is_empty() {
            println!("{tol:>9.1e}  empty point set, tol pending");
            continue;
        }
        let ratio = pq.len() as f64 / pa.len() as f64;
        measured_rows += 1;
        if ratio < min_ratio {
            min_ratio = ratio;
        }
        if ratio > max_ratio {
            max_ratio = ratio;
        }
        println!(
            "{tol:>9.1e} {:>10} {:>10} {:>10.3} {:>10} {:>10}",
            pa.len(),
            pq.len(),
            ratio,
            fmt_opt(Some(ea_tail)),
            fmt_opt(eq_tail)
        );
    }

    println!();
    println!();
    if measured_rows > 0 {
        println!(
            "verdict: ratio N_adaptiv/N_analytic at equal achieved tail error, min {min_ratio:.3}, max {max_ratio:.3}"
        );
        if min_ratio >= 1.0 {
            println!(
                "a-posteriori never sinks below the analytic placement at equal error in this fixture"
            );
        } else if max_ratio > 1.0 {
            println!(
                "no systematic sink: the a-posteriori count straddles the analytic count at equal error — the deviations are the power-of-2 cell granularity of the hierarchical subdivision, not an order gain; the analytic O(Quellen) placement is not beaten"
            );
        } else {
            println!(
                "a-posteriori sinks below the analytic placement by a bounded constant factor at every measured error level"
            );
        }
    } else {
        println!("verdict: no tractable tolerance row, metric pending");
    }
}

fn fmt_opt(e: Option<f64>) -> String {
    match e {
        Some(v) => format!("{v:.3e}"),
        None => String::from("none"),
    }
}
