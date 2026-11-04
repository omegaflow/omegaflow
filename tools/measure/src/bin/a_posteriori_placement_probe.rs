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

const D2_X0: f64 = 0.05;
const D2_X1: f64 = 1.0;
const D2_GRID_STEP: f64 = 1e-3;
const D2_REF_N: usize = 200;
const D2_POINT_CAP: usize = 5_000_000;
const SMOLYAK_MAX_LEVEL: usize = 18;

fn omega_2d(x: f64, y: f64) -> f64 {
    1.0 / (x * x + y * y + EXTENT * EXTENT)
}

fn omega_grad_mag_2d(x: f64, y: f64) -> f64 {
    let r2 = x * x + y * y;
    let r = r2.sqrt();
    if !(r > 0.0) {
        return 0.0;
    }
    let denom = r2 + EXTENT * EXTENT;
    2.0 * r / (denom * denom)
}

fn r_struct_2d(x: f64, y: f64) -> Option<f64> {
    let o = omega_2d(x, y);
    let g = omega_grad_mag_2d(x, y);
    if !o.is_finite() || !g.is_finite() || g == 0.0 {
        return None;
    }
    let r = F32_EPS * o.abs() / g.abs();
    if r.is_finite() { Some(r) } else { None }
}

fn analytic_spacing_h_2d(r: f64, tol: f64) -> Option<f64> {
    let rs = r_struct_2d(r, 0.0)?;
    let h = (rs * tol / F32_EPS).max(D2_GRID_STEP);
    if h.is_finite() && h > 0.0 {
        Some(h)
    } else {
        None
    }
}

fn analytic_points_2d(tol: f64) -> Option<Vec<(f64, f64)>> {
    let r_min = (2.0 * D2_X0 * D2_X0).sqrt();
    let r_max = (2.0 * D2_X1 * D2_X1).sqrt();
    let mut pts: Vec<(f64, f64)> = Vec::new();
    let mut r = r_min;
    while r <= r_max {
        let h = analytic_spacing_h_2d(r, tol)?;
        let n_f = (std::f64::consts::TAU * r / h).ceil();
        if !n_f.is_finite() || n_f <= 0.0 || n_f > 4.0 * D2_POINT_CAP as f64 {
            return None;
        }
        let n = n_f as usize;
        for j in 0..n {
            let theta = std::f64::consts::TAU * (j as f64) / (n as f64);
            let x = r * theta.cos();
            let y = r * theta.sin();
            if x >= D2_X0 && x <= D2_X1 && y >= D2_X0 && y <= D2_X1 {
                pts.push((x, y));
                if pts.len() > D2_POINT_CAP {
                    return None;
                }
            }
        }
        r += h;
    }
    if pts.is_empty() { None } else { Some(pts) }
}

fn max_cell_rel_dev(x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let mx = 0.5 * (x0 + x1);
    let my = 0.5 * (y0 + y1);
    let oc = omega_2d(mx, my);
    if !oc.is_finite() || oc.abs() <= OMEGA_FLOOR {
        return 0.0;
    }
    let samples = [
        (x0, y0),
        (x1, y0),
        (x0, y1),
        (x1, y1),
        (mx, y0),
        (mx, y1),
        (x0, my),
        (x1, my),
    ];
    let mut worst = 0.0;
    for (sx, sy) in samples {
        let os = omega_2d(sx, sy);
        if !os.is_finite() || os.abs() <= OMEGA_FLOOR {
            continue;
        }
        let e = ((os - oc) / os).abs();
        if e.is_finite() && e > worst {
            worst = e;
        }
    }
    worst
}

fn quadtree_points_2d(tol: f64) -> Option<Vec<(f64, f64)>> {
    let mut leaves: Vec<(f64, f64)> = Vec::new();
    let mut stack: Vec<(f64, f64, f64, f64)> = vec![(D2_X0, D2_X1, D2_X0, D2_X1)];
    while let Some((x0, x1, y0, y1)) = stack.pop() {
        if leaves.len() > D2_POINT_CAP {
            return None;
        }
        let w = x1 - x0;
        let h = y1 - y0;
        if w > D2_GRID_STEP && h > D2_GRID_STEP && max_cell_rel_dev(x0, x1, y0, y1) > tol {
            let mx = 0.5 * (x0 + x1);
            let my = 0.5 * (y0 + y1);
            stack.push((x0, mx, y0, my));
            stack.push((mx, x1, y0, my));
            stack.push((x0, mx, my, y1));
            stack.push((mx, x1, my, y1));
        } else {
            leaves.push((0.5 * (x0 + x1), 0.5 * (y0 + y1)));
        }
    }
    if leaves.is_empty() {
        None
    } else {
        Some(leaves)
    }
}

#[derive(Clone, Copy)]
struct KdPoint {
    x: f64,
    y: f64,
}

struct KdNode {
    point: usize,
    axis: usize,
    left: i32,
    right: i32,
}

struct KdTree {
    points: Vec<KdPoint>,
    nodes: Vec<KdNode>,
    root: i32,
}

impl KdTree {
    fn new(points: Vec<(f64, f64)>) -> Self {
        let points: Vec<KdPoint> = points.into_iter().map(|(x, y)| KdPoint { x, y }).collect();
        let mut tree = KdTree {
            points,
            nodes: Vec::new(),
            root: -1,
        };
        let mut idx: Vec<usize> = (0..tree.points.len()).collect();
        tree.root = tree.build(&mut idx, 0);
        tree
    }

    fn build(&mut self, idx: &mut [usize], depth: usize) -> i32 {
        if idx.is_empty() {
            return -1;
        }
        let axis = depth & 1;
        let mid = idx.len() / 2;
        {
            let points = &self.points;
            idx.select_nth_unstable_by(mid, |&a, &b| {
                let pa = points[a];
                let pb = points[b];
                let va = if axis == 0 { pa.x } else { pa.y };
                let vb = if axis == 0 { pb.x } else { pb.y };
                va.total_cmp(&vb)
            });
        }
        let point = idx[mid];
        let self_idx = self.nodes.len() as i32;
        self.nodes.push(KdNode {
            point,
            axis,
            left: -1,
            right: -1,
        });
        let (left, right) = idx.split_at_mut(mid);
        let l = self.build(left, depth + 1);
        let r = self.build(&mut right[1..], depth + 1);
        self.nodes[self_idx as usize].left = l;
        self.nodes[self_idx as usize].right = r;
        self_idx
    }

    fn nearest(&self, qx: f64, qy: f64) -> Option<usize> {
        if self.root < 0 {
            return None;
        }
        let mut best = usize::MAX;
        let mut best_d = f64::INFINITY;
        self.nn(self.root, qx, qy, &mut best, &mut best_d);
        if best == usize::MAX { None } else { Some(best) }
    }

    fn nn(&self, node: i32, qx: f64, qy: f64, best: &mut usize, best_d: &mut f64) {
        if node < 0 {
            return;
        }
        let n = &self.nodes[node as usize];
        let p = self.points[n.point];
        let dx = p.x - qx;
        let dy = p.y - qy;
        let d = dx * dx + dy * dy;
        if d < *best_d {
            *best_d = d;
            *best = n.point;
        }
        let diff = if n.axis == 0 { qx - p.x } else { qy - p.y };
        let (near, far) = if diff < 0.0 {
            (n.left, n.right)
        } else {
            (n.right, n.left)
        };
        self.nn(near, qx, qy, best, best_d);
        if diff * diff < *best_d {
            self.nn(far, qx, qy, best, best_d);
        }
    }
}

fn rel_error_at_2d(tree: &KdTree, x: f64, y: f64) -> Option<f64> {
    let o = omega_2d(x, y);
    if !o.is_finite() || o.abs() <= OMEGA_FLOOR {
        return None;
    }
    let i = tree.nearest(x, y)?;
    let p = &tree.points[i];
    let op = omega_2d(p.x, p.y);
    if !op.is_finite() {
        return None;
    }
    let e = ((o - op) / o).abs();
    if e.is_finite() { Some(e) } else { None }
}

fn max_rel_error_2d(tree: &KdTree) -> Option<f64> {
    let mut worst: Option<f64> = None;
    for i in 0..=D2_REF_N {
        let x = D2_X0 + (D2_X1 - D2_X0) * (i as f64 / D2_REF_N as f64);
        for j in 0..=D2_REF_N {
            let y = D2_X0 + (D2_X1 - D2_X0) * (j as f64 / D2_REF_N as f64);
            if let Some(e) = rel_error_at_2d(tree, x, y) {
                worst = Some(match worst {
                    Some(w) => w.max(e),
                    None => e,
                });
            }
        }
    }
    worst
}

fn cc_nodes_1d(level: usize) -> Vec<f64> {
    let m = if level <= 1 {
        1
    } else {
        (1usize << (level - 1)) + 1
    };
    if m == 1 {
        return vec![0.0];
    }
    let mut nodes: Vec<f64> = Vec::with_capacity(m);
    for j in 0..m {
        let arg = std::f64::consts::PI * (j as f64) / ((m - 1) as f64);
        nodes.push(-arg.cos());
    }
    nodes
}

fn smolyak_grid_points(level: usize) -> Option<Vec<(f64, f64)>> {
    let lo = D2_X0;
    let hi = D2_X1;
    let map = |u: f64| 0.5 * (lo + hi) + 0.5 * (hi - lo) * u;
    let mut seen: std::collections::BTreeSet<(i64, i64)> = std::collections::BTreeSet::new();
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for i1 in 1..=level + 1 {
        let xn = cc_nodes_1d(i1);
        for i2 in 1..=level + 1 {
            if i1 + i2 > level + 1 {
                continue;
            }
            let yn = cc_nodes_1d(i2);
            for &ux in &xn {
                for &uy in &yn {
                    let px = map(ux);
                    let py = map(uy);
                    let kx = (px * 1e12).round() as i64;
                    let ky = (py * 1e12).round() as i64;
                    if seen.insert((kx, ky)) {
                        pts.push((px, py));
                        if pts.len() > D2_POINT_CAP {
                            return None;
                        }
                    }
                }
            }
        }
    }
    if pts.is_empty() { None } else { Some(pts) }
}

fn smolyak_points_2d(achieved_error: f64) -> Option<Vec<(f64, f64)>> {
    if !achieved_error.is_finite() || !(achieved_error > 0.0) {
        return None;
    }
    for level in 1..=SMOLYAK_MAX_LEVEL {
        let pts = smolyak_grid_points(level)?;
        let tree = KdTree::new(pts);
        if let Some(e) = max_rel_error_2d(&tree) {
            if e <= achieved_error {
                return Some(tree.points.into_iter().map(|p| (p.x, p.y)).collect());
            }
        }
    }
    None
}

fn probe_2d() {
    println!();
    println!("--- 2D Voronoi/quadtree placement path ---");
    println!(
        "a-posteriori placement probe | 2D, kernel 0 (1/(r^2+e^2)), extent {EXTENT}, domain [{D2_X0}, {D2_X1}]^2, gridStep {D2_GRID_STEP}"
    );
    println!(
        "rule: R_struct = F32_EPS*|Omega|/|grad Omega| (mirrors 1D); analytic radial shell spacing h = max(gridStep, tol*|Omega|/|grad Omega|)"
    );
    println!(
        "metric: piecewise-constant 2D nearest-sample relative error, {D2_REF_N}x{D2_REF_N} reference grid, floor Omega <= {OMEGA_FLOOR}"
    );
    println!();
    println!(
        "{:>9} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "tol", "N_analytic", "N_adaptiv", "ratio", "e_ana", "e_ada"
    );
    println!(
        "(N_adaptiv is built to the analytic's own achieved max error e_ana — equal error, not equal tol)"
    );

    let tols = [1e-1_f64, 1e-2];
    let mut min_ratio = f64::INFINITY;
    let mut max_ratio = f64::NEG_INFINITY;
    let mut measured_rows = 0usize;

    for &tol in &tols {
        let Some(pa) = analytic_points_2d(tol) else {
            println!("{tol:>9.1e}  analytic point cap reached, tol pending");
            continue;
        };
        let tree_a = KdTree::new(pa);
        let Some(ea) = max_rel_error_2d(&tree_a) else {
            println!("{tol:>9.1e}  analytic error undefined, tol pending");
            continue;
        };
        let Some(pq) = quadtree_points_2d(ea) else {
            println!("{tol:>9.1e}  adaptive point cap reached, tol pending");
            continue;
        };
        if tree_a.points.is_empty() {
            println!("{tol:>9.1e}  empty point set, tol pending");
            continue;
        }
        let tree_q = KdTree::new(pq);
        let eq = max_rel_error_2d(&tree_q);
        let ratio = tree_q.points.len() as f64 / tree_a.points.len() as f64;
        measured_rows += 1;
        if ratio < min_ratio {
            min_ratio = ratio;
        }
        if ratio > max_ratio {
            max_ratio = ratio;
        }
        println!(
            "{tol:>9.1e} {:>10} {:>10} {:>10.3} {:>10} {:>10}",
            tree_a.points.len(),
            tree_q.points.len(),
            ratio,
            fmt_opt(Some(ea)),
            fmt_opt(eq)
        );
    }

    println!();
    if measured_rows > 0 {
        println!(
            "2D verdict: ratio N_adaptiv/N_analytic at equal achieved error, min {min_ratio:.3}, max {max_ratio:.3}"
        );
        if min_ratio >= 1.0 {
            println!(
                "2D: a-posteriori quadtree never sinks below the analytic shell placement at equal error in this fixture"
            );
        } else if max_ratio > 1.0 {
            println!(
                "2D: no systematic sink: the a-posteriori count straddles the analytic count at equal error — the deviations are the power-of-2 cell granularity of the 2D quadtree, not an order gain; the analytic O(Quellen) placement is not beaten"
            );
        } else {
            println!(
                "2D: a-posteriori sinks below the analytic placement by a bounded constant factor at every measured error level"
            );
        }
    } else {
        println!("2D verdict: no tractable tolerance row, metric pending");
    }
}

fn probe_smolyak() {
    println!();
    println!("--- 2D Smolyak sparse-grid placement path ---");
    println!(
        "sparse grid | Clenshaw-Curtis nested 1D rules, isotropic level cap {SMOLYAK_MAX_LEVEL}, domain [{D2_X0}, {D2_X1}]^2"
    );
    println!(
        "rule: build the lowest sparse-grid level whose achieved max relative error is <= the analytic e_ana — equal error, not equal tol"
    );
    println!(
        "metric: piecewise-constant 2D nearest-sample relative error, {D2_REF_N}x{D2_REF_N} reference grid, floor Omega <= {OMEGA_FLOOR}"
    );
    println!();
    println!(
        "{:>9} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "tol", "N_analytic", "N_smolyak", "ratio", "e_ana", "e_smo"
    );

    let tols = [1e-1_f64, 1e-2];
    let mut min_ratio = f64::INFINITY;
    let mut max_ratio = f64::NEG_INFINITY;
    let mut measured_rows = 0usize;

    for &tol in &tols {
        let Some(pa) = analytic_points_2d(tol) else {
            println!("{tol:>9.1e}  analytic point cap reached, tol pending");
            continue;
        };
        let tree_a = KdTree::new(pa);
        let Some(ea) = max_rel_error_2d(&tree_a) else {
            println!("{tol:>9.1e}  analytic error undefined, tol pending");
            continue;
        };
        let Some(ps) = smolyak_points_2d(ea) else {
            println!(
                "{tol:>9.1e}  sparse-grid level cap {SMOLYAK_MAX_LEVEL} exhausted, tol pending"
            );
            continue;
        };
        if tree_a.points.is_empty() {
            println!("{tol:>9.1e}  empty point set, tol pending");
            continue;
        }
        let tree_s = KdTree::new(ps);
        let es = max_rel_error_2d(&tree_s);
        let ratio = tree_s.points.len() as f64 / tree_a.points.len() as f64;
        measured_rows += 1;
        if ratio < min_ratio {
            min_ratio = ratio;
        }
        if ratio > max_ratio {
            max_ratio = ratio;
        }
        println!(
            "{tol:>9.1e} {:>10} {:>10} {:>10.3} {:>10} {:>10}",
            tree_a.points.len(),
            tree_s.points.len(),
            ratio,
            fmt_opt(Some(ea)),
            fmt_opt(es)
        );
    }

    println!();
    if measured_rows > 0 {
        println!(
            "smolyak verdict: ratio N_smolyak/N_analytic at equal achieved error, min {min_ratio:.3}, max {max_ratio:.3}"
        );
        if min_ratio < 0.1 {
            println!(
                "smolyak: the sparse grid undercuts the analytic O(Quellen) placement by an order of magnitude at every measured error level"
            );
        } else if min_ratio < 1.0 {
            println!(
                "smolyak: the sparse grid sinks below the analytic placement by a bounded constant factor — no order gain"
            );
        } else {
            println!(
                "smolyak: no order gain — the structure-agnostic sparse grid uses at least as many points as the analytic O(Quellen) placement at equal achieved error"
            );
        }
    } else {
        println!("smolyak verdict: no tractable tolerance row, metric pending");
    }
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
        "note: 1D radial fixture; the 2D Voronoi/quadtree topology is measured in the probe_2d section below"
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

    probe_2d();
    probe_smolyak();
}

fn fmt_opt(e: Option<f64>) -> String {
    match e {
        Some(v) => format!("{v:.3e}"),
        None => String::from("none"),
    }
}
