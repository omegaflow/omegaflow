pub fn gammln(xx: f64) -> f64 {
    let cof = [
        76.18009172947146,
        -86.50532032941677,
        24.01409824083091,
        -1.231739572450155,
        0.1208650973866179e-2,
        -0.5395239384953e-5,
    ];
    let x = xx;
    let mut y = xx;
    let mut tmp = x + 5.5;
    tmp -= (x + 0.5) * tmp.ln();
    let mut ser = 1.000000000190015;
    for j in 0..6 {
        y += 1.0;
        ser += cof[j] / y;
    }
    -tmp + (2.5066282746310005 * ser / x).ln()
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    const MAXIT: usize = 400;
    const EPS: f64 = 1.0e-14;
    const FPMIN: f64 = 1.0e-300;
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < FPMIN {
        d = FPMIN;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=MAXIT {
        let m2 = 2 * m;
        let mut aa = m as f64 * (b - m as f64) * x / ((qam + m2 as f64) * (a + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        h *= d * c;
        let m2 = 2 * m + 1;
        aa = -(a + m as f64) * (qab + m as f64) * x / ((a + m2 as f64) * (qap + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < EPS {
            break;
        }
    }
    h
}

fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let bt = (gammln(a + b) - gammln(a) - gammln(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

pub fn t_two_p(t: f64, df: f64) -> f64 {
    betai(df / 2.0, 0.5, df / (df + t * t))
}

pub fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let s: f64 = xs.iter().sum();
    Some(s / xs.len() as f64)
}

pub fn sample_sd(xs: &[f64]) -> Option<f64> {
    if xs.len() < 2 {
        return None;
    }
    let m = mean(xs)?;
    let var = xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() - 1) as f64;
    if var.is_finite() {
        Some(var.sqrt())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{mean, sample_sd, t_two_p};

    #[test]
    fn mean_and_sample_sd_read_the_series() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0]).unwrap(), 2.5);
        let sd = sample_sd(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]).unwrap();
        let expect = (32.0f64 / 7.0).sqrt();
        assert!((sd - expect).abs() < 1e-9, "sample sd {sd} vs {expect}");
        assert!(sample_sd(&[1.0]).is_none(), "one point carries no sd");
        assert!(mean(&[]).is_none(), "no points carry no mean");
    }

    fn close(p: f64, expect: f64, tol: f64) {
        assert!(
            (p - expect).abs() < tol,
            "two-sided p {p} deviates from {expect} beyond {tol}"
        );
    }

    #[test]
    fn t_two_p_reference_values() {
        let crit: &[(f64, f64)] = &[
            (12.706, 1.0),
            (4.303, 2.0),
            (2.571, 5.0),
            (2.447, 6.0),
            (2.042, 30.0),
            (2.021, 40.0),
        ];
        for (t, df) in crit {
            close(t_two_p(*t, *df), 0.05, 1.5e-3);
        }
        close(t_two_p(1.943, 6.0), 0.10, 1.5e-3);
    }
}
