use super::parcorr::{CiTest, ParCorr};
use super::pc::pc_stable_skeleton;
use super::te::{
    LaggedCond, TeEstimator, TeNull, TeSurrogateParams, conditional_te_surrogates_n,
    transfer_entropy_conditional_binned_n,
};
use std::collections::BTreeSet;

pub struct TeBinnedCi {
    pub bins: usize,
    pub n_surr: usize,
    pub seed: u64,
}

impl TeBinnedCi {
    fn prepared(
        x: &[f64],
        y: &[f64],
        conds: &[&[f64]],
    ) -> Option<(Vec<f32>, Vec<f32>, Vec<Vec<f32>>)> {
        let n = x.len();
        if n < 8 || y.len() != n || conds.iter().any(|c| c.len() != n) {
            return None;
        }
        let xf: Vec<f32> = x.iter().map(|&v| v as f32).collect();
        let yf: Vec<f32> = y.iter().map(|&v| v as f32).collect();
        let cf: Vec<Vec<f32>> = conds
            .iter()
            .map(|c| c.iter().map(|&v| v as f32).collect())
            .collect();
        Some((xf, yf, cf))
    }

    fn zero_lag(conds: &[Vec<f32>]) -> Vec<LaggedCond<'_>> {
        conds
            .iter()
            .map(|s| LaggedCond {
                series: s.as_slice(),
                lag: 0,
            })
            .collect()
    }

    fn observed(
        &self,
        x: &[f64],
        y: &[f64],
        conds: &[&[f64]],
    ) -> Option<(f64, Vec<f32>, Vec<f32>, Vec<Vec<f32>>)> {
        let (xf, yf, cf) = Self::prepared(x, y, conds)?;
        let lc = Self::zero_lag(&cf);
        let te = transfer_entropy_conditional_binned_n(&xf, &yf, &lc, 1, self.bins)?;
        Some((te, xf, yf, cf))
    }

    pub fn te(&self, x: &[f64], y: &[f64], conds: &[&[f64]]) -> Option<f64> {
        self.observed(x, y, conds).map(|(te, _, _, _)| te)
    }
}

impl CiTest for TeBinnedCi {
    fn p(&self, x: &[f64], y: &[f64], conds: &[&[f64]]) -> Option<f64> {
        let (te, xf, yf, cf) = self.observed(x, y, conds)?;
        let lc = Self::zero_lag(&cf);
        let surr = conditional_te_surrogates_n(
            &xf,
            &yf,
            &lc,
            TeSurrogateParams {
                lag: 1,
                max_lag: 1,
                bins: self.bins,
                seed: self.seed,
                n_surr: self.n_surr,
                null: TeNull::Phase,
                block: 0,
                est: TeEstimator::Binned,
                k: 0,
            },
        )?;
        let b = surr.len();
        let rank = 1 + surr.iter().filter(|&&s| s >= te).count();
        Some(rank as f64 / (b + 1) as f64)
    }
}

pub struct PanelView {
    series: Vec<Vec<f32>>,
    n: usize,
    max_lag: usize,
}

impl PanelView {
    pub fn new(series: Vec<Vec<f32>>, max_lag: usize) -> Option<PanelView> {
        if series.is_empty() {
            return None;
        }
        let n = series[0].len();
        if n == 0 || series.iter().any(|s| s.len() != n) {
            return None;
        }
        Some(PanelView { series, n, max_lag })
    }

    pub fn max_lag(&self) -> usize {
        self.max_lag
    }

    pub fn aligned(&self, nodes: &[(usize, usize)]) -> Option<Vec<Vec<f64>>> {
        if nodes.is_empty() {
            return Some(Vec::new());
        }
        if nodes.iter().any(|&(var, _)| var >= self.series.len()) {
            return None;
        }
        let start = nodes.iter().map(|&(_, delay)| delay).max()?;
        if start >= self.n {
            return None;
        }
        let mut out: Vec<Vec<f64>> = Vec::with_capacity(nodes.len());
        for &(var, delay) in nodes {
            let mut col = Vec::with_capacity(self.n - start);
            for t in start..self.n {
                col.push(self.series[var][t - delay] as f64);
            }
            out.push(col);
        }
        Some(out)
    }

    pub fn acf(series: &[f32], max_lag: usize) -> Vec<f64> {
        let n = series.len();
        if n < 2 {
            return Vec::new();
        }
        let mean = series.iter().map(|&v| v as f64).sum::<f64>() / n as f64;
        let denom = series
            .iter()
            .map(|&v| {
                let d = v as f64 - mean;
                d * d
            })
            .sum::<f64>();
        if !(denom > 0.0) {
            return Vec::new();
        }
        let mut out = Vec::new();
        for lag in 1..=max_lag {
            if lag >= n {
                break;
            }
            let num = (lag..n)
                .map(|t| (series[t] as f64 - mean) * (series[t - lag] as f64 - mean))
                .sum::<f64>();
            out.push(num / denom);
        }
        out
    }
}

pub struct MciParams {
    pub max_lag: usize,
    pub max_cond: usize,
    pub alpha_pc: f64,
    pub alpha_mci: f64,
    pub bins: usize,
    pub n_surr: usize,
    pub seed: u64,
}

pub struct WindowLink {
    pub driver: usize,
    pub target: usize,
    pub lag: usize,
    pub te: f64,
    pub p_value: f64,
    pub fdr_pass: bool,
}

pub struct PendingEdge {
    pub driver: usize,
    pub target: usize,
    pub lag: usize,
    pub n: usize,
    pub n_min: usize,
    pub dim: usize,
}

pub struct WindowGraph {
    pub links: Vec<WindowLink>,
    pub pending: Vec<PendingEdge>,
}

fn link_seed(base: u64, i: usize, j: usize, tau: usize) -> u64 {
    base ^ (i as u64).wrapping_mul(0x9E37_79B9)
        ^ (j as u64).wrapping_mul(0x85EB_CA6B)
        ^ (tau as u64).wrapping_mul(0xC2B2_AE3D)
}

fn benjamini_yekutieli_pass(p_values: &[f64], level: f64) -> Vec<bool> {
    let m = p_values.len();
    if m == 0 || !(level > 0.0 && level <= 1.0) {
        return vec![false; m];
    }
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|&a, &b| p_values[a].total_cmp(&p_values[b]));
    let h_m: f64 = (1..=m).map(|k| 1.0 / k as f64).sum();
    let mf = m as f64;
    let mut cutoff: Option<f64> = None;
    for (rank0, &oi) in order.iter().enumerate() {
        let k = (rank0 + 1) as f64;
        let p = p_values[oi];
        if p <= k / mf * level / h_m {
            cutoff = Some(p);
        }
    }
    match cutoff {
        Some(c) => p_values.iter().map(|&p| p <= c).collect(),
        None => vec![false; m],
    }
}

pub fn mci_window_graph(series: &[Vec<f32>], p: MciParams) -> WindowGraph {
    let d = series.len();
    if d < 2 {
        return WindowGraph {
            links: Vec::new(),
            pending: Vec::new(),
        };
    }
    let stride = p.max_lag + 1;

    let panel = match PanelView::new(series.to_vec(), p.max_lag) {
        Some(v) => v,
        None => {
            return WindowGraph {
                links: Vec::new(),
                pending: Vec::new(),
            };
        }
    };

    let all_nodes: Vec<(usize, usize)> = (0..d)
        .flat_map(|var| (0..=p.max_lag).map(move |delay| (var, delay)))
        .collect();
    let columns = match panel.aligned(&all_nodes) {
        Some(c) => c,
        None => {
            return WindowGraph {
                links: Vec::new(),
                pending: Vec::new(),
            };
        }
    };

    let edges = pc_stable_skeleton(&columns, &ParCorr, p.alpha_pc, p.max_cond);

    let mut parents: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); d];
    for &(a, b) in &edges {
        for &(x, y) in &[(a, b), (b, a)] {
            if y % stride == 0 && x % stride != 0 {
                parents[y / stride].insert(x);
            }
        }
    }

    let mut links: Vec<WindowLink> = Vec::new();
    let mut pending: Vec<PendingEdge> = Vec::new();
    for i in 0..d {
        for tau in 1..=p.max_lag {
            let candidate = i * stride + tau;
            for j in 0..d {
                if j == i || !parents[j].contains(&candidate) {
                    continue;
                }

                let mut cond_nodes: BTreeSet<usize> = BTreeSet::new();
                for &pnode in &parents[j] {
                    if pnode != candidate {
                        cond_nodes.insert(pnode);
                    }
                }
                for &pnode in &parents[i] {
                    cond_nodes.insert(pnode);
                }
                cond_nodes.retain(|&pnode| {
                    let v = pnode / stride;
                    v != i && v != j
                });

                let mut req: Vec<(usize, usize)> = Vec::with_capacity(2 + cond_nodes.len());
                req.push((i, tau - 1));
                req.push((j, 0));
                for &pnode in &cond_nodes {
                    req.push((pnode / stride, pnode % stride));
                }
                let cols = match panel.aligned(&req) {
                    Some(c) => c,
                    None => continue,
                };
                let driver_col = cols[0].as_slice();
                let target_col = cols[1].as_slice();
                let cond_cols: Vec<&[f64]> = cols[2..].iter().map(|c| c.as_slice()).collect();

                let dim = cond_cols.len();
                let n_min = p.bins.pow((2 + dim) as u32);
                if target_col.len() < n_min {
                    pending.push(PendingEdge {
                        driver: i,
                        target: j,
                        lag: tau,
                        n: target_col.len(),
                        n_min,
                        dim,
                    });
                    continue;
                }

                let ci = TeBinnedCi {
                    bins: p.bins,
                    n_surr: p.n_surr,
                    seed: link_seed(p.seed, i, j, tau),
                };
                let p_value = match ci.p(target_col, driver_col, &cond_cols) {
                    Some(v) => v,
                    None => continue,
                };
                let te = match ci.te(target_col, driver_col, &cond_cols) {
                    Some(v) => v,
                    None => continue,
                };

                links.push(WindowLink {
                    driver: i,
                    target: j,
                    lag: tau,
                    te,
                    p_value,
                    fdr_pass: false,
                });
            }
        }
    }

    links.sort_by(|a, b| {
        a.driver
            .cmp(&b.driver)
            .then(a.target.cmp(&b.target))
            .then(a.lag.cmp(&b.lag))
    });

    let p_values: Vec<f64> = links.iter().map(|l| l.p_value).collect();
    let passes = benjamini_yekutieli_pass(&p_values, p.alpha_mci);
    for (link, pass) in links.iter_mut().zip(passes) {
        link.fdr_pass = pass;
    }
    WindowGraph { links, pending }
}

pub fn mci_window_links(series: &[Vec<f32>], p: MciParams) -> Vec<WindowLink> {
    mci_window_graph(series, p).links
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rng(u64);

    impl Rng {
        fn new(seed: u64) -> Self {
            Rng(seed)
        }

        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }

        fn uniform(&mut self) -> f64 {
            (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
        }

        fn normal(&mut self) -> f64 {
            let u1 = self.uniform().max(f64::MIN_POSITIVE);
            let u2 = self.uniform();
            (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
        }
    }

    #[test]
    fn aligned_drops_exactly_the_boundary_rows() {
        let s0: Vec<f32> = (0..10).map(|i| i as f32).collect();
        let s1: Vec<f32> = (0..10).map(|i| (10 + i) as f32).collect();
        let panel = PanelView::new(vec![s0.clone(), s1.clone()], 2).unwrap();
        assert_eq!(panel.max_lag(), 2);
        let out = panel.aligned(&[(0, 0), (1, 2)]).unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].len(), 8);
        assert_eq!(out[1].len(), 8);
        assert_eq!(out[0][0], s0[2] as f64);
        assert_eq!(out[1][0], s1[0] as f64);
        assert!(panel.aligned(&[(2, 0)]).is_none());
    }

    #[test]
    fn acf_of_white_noise_is_small_and_of_ar1_is_large() {
        let mut rng = Rng::new(5);
        let n = 2000;
        let white: Vec<f32> = (0..n).map(|_| (0.3 * rng.normal()) as f32).collect();
        let w = PanelView::acf(&white, 1);
        assert!(w[0].abs() < 0.2, "white acf {}", w[0]);

        let mut ar = vec![0.0_f32; n];
        let mut prev = 0.0_f64;
        for t in 0..n {
            prev = 0.9 * prev + 0.3 * rng.normal();
            ar[t] = prev as f32;
        }
        let a = PanelView::acf(&ar, 1);
        assert!(a[0] > 0.6, "ar acf {}", a[0]);
    }

    #[test]
    fn te_binned_ci_finds_the_direction() {
        let mut rng = Rng::new(3);
        let n = 1500;
        let mut driver = vec![0.0_f64; n];
        let mut prev = 0.0_f64;
        for t in 0..n {
            prev = 0.9 * prev + 0.3 * rng.normal();
            driver[t] = prev;
        }
        let mut target = vec![0.0_f64; n];
        for t in 0..n {
            target[t] = if t == 0 {
                0.3 * rng.normal()
            } else {
                0.7 * driver[t - 1] + 0.3 * rng.normal()
            };
        }

        let ci = TeBinnedCi {
            bins: 4,
            n_surr: 60,
            seed: 1,
        };
        let forward = ci.p(&target, &driver, &[]).unwrap();
        assert!(forward < 0.05, "forward p {}", forward);
        let reverse = ci.p(&driver, &target, &[]).unwrap();
        assert!(reverse > forward, "reverse {} forward {}", reverse, forward);
    }

    #[test]
    fn te_binned_ci_conditioning_returns_a_p_value() {
        let mut rng = Rng::new(9);
        let n = 400;
        let mut a = vec![0.0_f64; n];
        let mut b = vec![0.0_f64; n];
        let mut c = vec![0.0_f64; n];
        for t in 0..n {
            a[t] = rng.normal();
            b[t] = 0.5 * a[t] + 0.5 * rng.normal();
            c[t] = rng.normal();
        }
        let ci = TeBinnedCi {
            bins: 4,
            n_surr: 20,
            seed: 2,
        };
        let p = ci.p(&b, &a, &[&c]).unwrap();
        assert!((0.0..=1.0).contains(&p), "p {}", p);
        assert!(ci.p(&b, &a, &[&c[..n - 1]]).is_none());
    }

    #[test]
    fn mci_chain_keeps_direct_links_and_drops_the_mediator() {
        let mut rng = Rng::new(101);
        let n = 1200;
        let mut x0 = vec![0.0_f32; n];
        let mut x1 = vec![0.0_f32; n];
        let mut x2 = vec![0.0_f32; n];
        for t in 0..n {
            x0[t] = (0.5 * rng.normal()) as f32;
            x1[t] = if t == 0 {
                (0.5 * rng.normal()) as f32
            } else {
                (0.7 * x0[t - 1] as f64 + 0.5 * rng.normal()) as f32
            };
            x2[t] = if t == 0 {
                (0.5 * rng.normal()) as f32
            } else {
                (0.7 * x1[t - 1] as f64 + 0.5 * rng.normal()) as f32
            };
        }
        let series = vec![x0, x1, x2];
        let params = MciParams {
            max_lag: 2,
            max_cond: 2,
            alpha_pc: 0.2,
            alpha_mci: 0.05,
            bins: 4,
            n_surr: 30,
            seed: 7,
        };
        let links = mci_window_links(&series, params);
        let shape: Vec<(usize, usize, usize)> =
            links.iter().map(|l| (l.driver, l.target, l.lag)).collect();
        assert!(
            links
                .iter()
                .any(|l| l.driver == 0 && l.target == 1 && l.lag == 1),
            "missing 0->1 links {:?}",
            shape
        );
        assert!(
            links
                .iter()
                .any(|l| l.driver == 1 && l.target == 2 && l.lag == 1),
            "missing 1->2 links {:?}",
            shape
        );
        assert!(
            !links
                .iter()
                .any(|l| l.driver == 0 && l.target == 2 && l.lag == 1),
            "spurious 0->2 links {:?}",
            shape
        );
    }

    #[test]
    fn mci_confounder_produces_no_direct_link() {
        let mut rng = Rng::new(202);
        let n = 1200;
        let mut z = vec![0.0_f32; n];
        let mut a = vec![0.0_f32; n];
        let mut b = vec![0.0_f32; n];
        for t in 0..n {
            z[t] = (0.5 * rng.normal()) as f32;
            a[t] = if t == 0 {
                (0.5 * rng.normal()) as f32
            } else {
                (0.8 * z[t - 1] as f64 + 0.5 * rng.normal()) as f32
            };
            b[t] = if t == 0 {
                (0.5 * rng.normal()) as f32
            } else {
                (0.8 * z[t - 1] as f64 + 0.5 * rng.normal()) as f32
            };
        }
        let series = vec![z, a, b];
        let params = MciParams {
            max_lag: 2,
            max_cond: 2,
            alpha_pc: 0.2,
            alpha_mci: 0.05,
            bins: 4,
            n_surr: 30,
            seed: 11,
        };
        let links = mci_window_links(&series, params);
        let shape: Vec<(usize, usize, usize)> =
            links.iter().map(|l| (l.driver, l.target, l.lag)).collect();
        assert!(
            !links.iter().any(|l| l.driver == 1 && l.target == 2),
            "spurious a->b links {:?}",
            shape
        );
        assert!(
            links.iter().any(|l| l.driver == 0 && l.target == 1),
            "missing z->a links {:?}",
            shape
        );
        assert!(
            links.iter().any(|l| l.driver == 0 && l.target == 2),
            "missing z->b links {:?}",
            shape
        );
    }

    #[test]
    fn underpowered_window_edges_are_named_pending() {
        let n = 12usize;
        let x: Vec<f32> = (0..n).map(|i| (i as f32 * 0.7).sin()).collect();
        let y: Vec<f32> = x.iter().map(|v| v * 0.9).collect();
        let graph = mci_window_graph(
            &[x, y],
            MciParams {
                max_lag: 1,
                max_cond: 2,
                alpha_pc: 0.2,
                alpha_mci: 0.05,
                bins: 4,
                n_surr: 10,
                seed: 1,
            },
        );
        assert!(
            !graph.pending.is_empty(),
            "a batch below bins^(2+dim) is pending, not silently binned"
        );
        assert!(
            graph.pending.iter().all(|e| e.n < e.n_min),
            "a pending edge carries n < n_min"
        );
    }
}
