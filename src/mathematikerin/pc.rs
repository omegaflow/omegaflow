use super::parcorr::CiTest;
use std::collections::BTreeSet;

const MAX_NEIGHBOURS: usize = 12;

pub fn pc_stable_skeleton(
    series: &[Vec<f64>],
    test: &dyn CiTest,
    alpha: f64,
    max_cond: usize,
) -> Vec<(usize, usize)> {
    let d = series.len();
    if d < 2 {
        return Vec::new();
    }
    let n = series[0].len();
    if series.iter().any(|c| c.len() != n) {
        return Vec::new();
    }

    let mut adj: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); d];
    for i in 0..d {
        for j in 0..d {
            if i != j {
                adj[i].insert(j);
            }
        }
    }

    for order in 0..=max_cond {
        let snapshot = adj.clone();
        let mut removed_any = false;
        for i in 0..d {
            for j in (i + 1)..d {
                if !adj[i].contains(&j) {
                    continue;
                }
                let neighbours: Vec<usize> =
                    snapshot[i].iter().copied().filter(|&k| k != j).collect();
                if neighbours.len() < order || neighbours.len() > MAX_NEIGHBOURS {
                    continue;
                }
                let mut combo = Combinations::new(neighbours.len(), order);
                while let Some(choice) = combo.next() {
                    let conds: Vec<&[f64]> = choice
                        .iter()
                        .map(|&k| series[neighbours[k]].as_slice())
                        .collect();
                    if let Some(p) = test.p(&series[i], &series[j], &conds) {
                        if p > alpha {
                            adj[i].remove(&j);
                            adj[j].remove(&i);
                            removed_any = true;
                            break;
                        }
                    }
                }
            }
        }
        if !removed_any {
            break;
        }
    }

    let mut edges = Vec::new();
    for i in 0..d {
        for &j in adj[i].iter() {
            if i < j {
                edges.push((i, j));
            }
        }
    }
    edges.sort();
    edges
}

struct Combinations {
    n: usize,
    k: usize,
    indices: Vec<usize>,
    first: bool,
    done: bool,
}

impl Combinations {
    fn new(n: usize, k: usize) -> Self {
        if k > n {
            Combinations {
                n,
                k,
                indices: Vec::new(),
                first: false,
                done: true,
            }
        } else {
            Combinations {
                n,
                k,
                indices: (0..k).collect(),
                first: true,
                done: false,
            }
        }
    }

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        if self.first {
            self.first = false;
            return Some(self.indices.clone());
        }
        if self.k == 0 {
            self.done = true;
            return None;
        }
        let mut i = self.k;
        while i > 0 {
            i -= 1;
            if self.indices[i] != i + self.n - self.k {
                self.indices[i] += 1;
                for j in (i + 1)..self.k {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                return Some(self.indices.clone());
            }
        }
        self.done = true;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathematikerin::parcorr::ParCorr;

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

        fn noise(&mut self) -> f64 {
            0.3 * self.normal()
        }
    }

    #[test]
    fn chain_skeleton() {
        let mut rng = Rng::new(7);
        let n = 1000;
        let mut x0 = vec![0.0; n];
        let mut x1 = vec![0.0; n];
        let mut x2 = vec![0.0; n];
        for t in 0..n {
            x0[t] = rng.noise();
            x1[t] = 0.8 * x0[t] + rng.noise();
            x2[t] = 0.8 * x1[t] + rng.noise();
        }
        let edges = pc_stable_skeleton(&[x0, x1, x2], &ParCorr, 0.05, 3);
        assert!(edges.contains(&(0, 1)), "edges {:?}", edges);
        assert!(edges.contains(&(1, 2)), "edges {:?}", edges);
        assert!(!edges.contains(&(0, 2)), "edges {:?}", edges);
    }

    #[test]
    fn confounder_skeleton() {
        let mut rng = Rng::new(11);
        let n = 1000;
        let mut z = vec![0.0; n];
        let mut a = vec![0.0; n];
        let mut b = vec![0.0; n];
        for t in 0..n {
            z[t] = rng.noise();
            a[t] = 0.8 * z[t] + rng.noise();
            b[t] = 0.8 * z[t] + rng.noise();
        }
        let edges = pc_stable_skeleton(&[z, a, b], &ParCorr, 0.05, 3);
        assert!(edges.contains(&(0, 1)), "edges {:?}", edges);
        assert!(edges.contains(&(0, 2)), "edges {:?}", edges);
    }

    #[test]
    fn independent_skeleton() {
        let mut rng = Rng::new(23);
        let n = 1000;
        let mut x0 = vec![0.0; n];
        let mut x1 = vec![0.0; n];
        let mut x2 = vec![0.0; n];
        for t in 0..n {
            x0[t] = rng.noise();
            x1[t] = rng.noise();
            x2[t] = rng.noise();
        }
        let edges = pc_stable_skeleton(&[x0, x1, x2], &ParCorr, 0.05, 3);
        assert!(edges.is_empty(), "edges {:?}", edges);
    }

    #[test]
    fn fewer_than_two_variables_is_empty() {
        let empty: Vec<Vec<f64>> = Vec::new();
        assert!(pc_stable_skeleton(&empty, &ParCorr, 0.05, 3).is_empty());
        let one = vec![vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6]];
        assert!(pc_stable_skeleton(&one, &ParCorr, 0.05, 3).is_empty());
    }

    #[test]
    fn combinations_are_lexicographic() {
        let mut combo = Combinations::new(4, 2);
        let mut out = Vec::new();
        while let Some(c) = combo.next() {
            out.push(c);
        }
        assert_eq!(
            out,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
            ]
        );
        let mut zero = Combinations::new(3, 0);
        assert_eq!(zero.next(), Some(Vec::new()));
        assert_eq!(zero.next(), None);
    }
}
