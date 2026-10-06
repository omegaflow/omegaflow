use crate::mathematikerin::least_squares::solve_normal_equations;

pub struct Witness<'a> {
    pub name: &'a str,
    pub series: &'a [f64],
}

pub struct Residual {
    pub series: Vec<f64>,
    pub witnesses_used: Vec<String>,
    pub n: usize,
    pub lag: usize,
}

pub fn residual_against_witnesses(
    target: &[f64],
    witnesses: &[Witness<'_>],
    lags: &[usize],
) -> Option<Residual> {
    let mut best: Option<(f64, Residual)> = None;
    for &lag in lags {
        if lag >= target.len() {
            continue;
        }
        let used: Vec<usize> = witnesses
            .iter()
            .enumerate()
            .filter(|(_, w)| w.series.len() == target.len())
            .map(|(i, _)| i)
            .collect();
        if used.is_empty() {
            continue;
        }
        let n = target.len() - lag;
        let params = 1 + used.len();
        let mut ata = vec![vec![0.0f64; params]; params];
        let mut atx = vec![0.0f64; params];
        for t in lag..target.len() {
            let mut row = Vec::with_capacity(params);
            row.push(1.0);
            for &wi in &used {
                row.push(witnesses[wi].series[t - lag]);
            }
            for a in 0..params {
                for b in 0..params {
                    ata[a][b] += row[a] * row[b];
                }
                atx[a] += row[a] * target[t];
            }
        }
        let Some((coeffs, _, _)) = solve_normal_equations(&ata, &atx, &atx, &atx) else {
            continue;
        };
        let mut series = Vec::with_capacity(n);
        for t in lag..target.len() {
            let mut prediction = coeffs[0];
            for (k, &wi) in used.iter().enumerate() {
                prediction += coeffs[k + 1] * witnesses[wi].series[t - lag];
            }
            series.push(target[t] - prediction);
        }
        let variance = residual_variance(&series);
        let residual = Residual {
            series,
            witnesses_used: used
                .iter()
                .map(|&i| witnesses[i].name.to_string())
                .collect(),
            n,
            lag,
        };
        match &best {
            Some((v, _)) if *v <= variance => {}
            _ => best = Some((variance, residual)),
        }
    }
    best.map(|(_, residual)| residual)
}

fn residual_variance(series: &[f64]) -> f64 {
    let n = series.len();
    if n == 0 {
        return f64::INFINITY;
    }
    let mean = series.iter().sum::<f64>() / n as f64;
    series.iter().map(|r| (r - mean) * (r - mean)).sum::<f64>() / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u64);

    impl Lcg {
        fn next_unit(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }

        fn next_noise(&mut self) -> f64 {
            self.next_unit() * 2.0 - 1.0
        }
    }

    fn variance(series: &[f64]) -> f64 {
        let n = series.len();
        let mean = series.iter().sum::<f64>() / n as f64;
        series.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n as f64
    }

    #[test]
    fn residual_removes_the_witness_prediction() {
        let n = 256;
        let mut w0 = Vec::with_capacity(n);
        let mut w1 = Vec::with_capacity(n);
        for t in 0..n {
            let x = t as f64;
            w0.push((x * 0.11).sin());
            w1.push((x * 0.07).cos() + 0.2 * (x * 0.31).sin());
        }
        let mut rng = Lcg(0x9E37_79B9_7F4A_7C15);
        let target: Vec<f64> = (0..n)
            .map(|t| 0.5 * w0[t] + 0.3 * w1[t] + 0.01 * rng.next_noise())
            .collect();

        let witnesses = [
            Witness {
                name: "w0",
                series: &w0,
            },
            Witness {
                name: "w1",
                series: &w1,
            },
        ];
        let r = residual_against_witnesses(&target, &witnesses, &[0]).expect("residual");
        assert_eq!(r.n, target.len());
        assert_eq!(r.series.len(), target.len());
        assert_eq!(r.witnesses_used.len(), 2);
        assert_eq!(r.lag, 0);
        assert!(variance(&r.series) < 0.1 * variance(&target));

        let empty: [Witness<'_>; 0] = [];
        assert!(residual_against_witnesses(&target, &empty, &[0]).is_none());

        let kept_nothing = [Witness {
            name: "empty",
            series: &[],
        }];
        assert!(residual_against_witnesses(&target, &kept_nothing, &[0]).is_none());

        let short = &target[..4];
        assert!(residual_against_witnesses(short, &witnesses, &[10]).is_none());
    }
}
