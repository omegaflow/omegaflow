use std::process::Command;

use omegaflow::gaia_sso::GAIA_TAP_SYNC;

const ADQL: &str = "SELECT g.parallax, g.parallax_error FROM gaiadr3.vari_cepheid AS c JOIN gaiadr3.gaia_source AS g USING (source_id) WHERE g.parallax > 0 AND g.parallax_error > 0 AND g.parallax > 5 * g.parallax_error";

fn tap_csv(adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("180")
        .arg("-A")
        .arg("omegaflow-cepheid-parallax-weigh/1.0")
        .arg("-G")
        .arg(GAIA_TAP_SYNC)
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .output()
        .ok()?;
    String::from_utf8(out.stdout).ok()
}

fn parse_parallaxes(csv: &str) -> Vec<(f64, f64)> {
    let mut rows = Vec::new();
    for line in csv.lines().skip(1) {
        let mut cols = line.split(',');
        let pi = cols.next().and_then(|s| s.trim().parse::<f64>().ok());
        let sig = cols.next().and_then(|s| s.trim().parse::<f64>().ok());
        if let (Some(pi), Some(sig)) = (pi, sig) {
            if pi.is_finite() && sig.is_finite() && pi > 0.0 && sig > 0.0 {
                rows.push((pi, sig));
            }
        }
    }
    rows
}

fn weighted_mean(rows: &[(f64, f64)]) -> Option<(f64, f64)> {
    let mut num = 0.0;
    let mut den = 0.0;
    for (pi, sig) in rows {
        let w = 1.0 / (sig * sig);
        num += w * pi;
        den += w;
    }
    if den <= 0.0 || !den.is_finite() {
        return None;
    }
    let mean = num / den;
    let err = 1.0 / den.sqrt();
    if !mean.is_finite() || !err.is_finite() {
        return None;
    }
    Some((mean, err))
}

fn main() {
    let csv = match tap_csv(ADQL) {
        Some(c) => c,
        None => {
            eprintln!(
                "cepheid_parallax_weigh: the Gaia TAP query returned no bytes — the weighing stays unmeasured"
            );
            std::process::exit(1);
        }
    };
    let first_line = match csv.lines().next() {
        Some(l) => l,
        None => {
            eprintln!(
                "cepheid_parallax_weigh: the TAP answer is an empty body — the weighing stays unmeasured"
            );
            std::process::exit(1);
        }
    };
    if !first_line.contains("parallax") {
        eprintln!(
            "cepheid_parallax_weigh: the TAP answer is not a parallax CSV (header: {first_line}) — the weighing stays unmeasured"
        );
        std::process::exit(1);
    }
    let rows = parse_parallaxes(&csv);
    if rows.is_empty() {
        eprintln!(
            "cepheid_parallax_weigh: the CSV carries no positive finite parallax+error rows — the weighing stays unmeasured"
        );
        std::process::exit(1);
    }
    let Some((mean, err)) = weighted_mean(&rows) else {
        eprintln!(
            "cepheid_parallax_weigh: the parallax weights collapse to a non-finite mean — the weighing stays unmeasured"
        );
        std::process::exit(1);
    };
    let dist_pc = 1000.0 / mean;
    let mut meds: Vec<f64> = rows.iter().map(|r| r.0).collect();
    meds.sort_by(|a, b| a.total_cmp(b));
    let med = meds[meds.len() / 2];
    let min = meds[0];
    let max = meds[meds.len() - 1];
    let unweighted = meds.iter().sum::<f64>() / meds.len() as f64;
    println!(
        "cepheid_parallax_weigh: N={} | inverse-variance weighted mean parallax = {:.4} mas ± {:.4} mas (standard error) | 1/π = {:.1} pc",
        rows.len(), mean, err, dist_pc
    );
    println!(
        "cepheid_parallax_weigh: sample median parallax = {:.4} mas | min {:.4} | max {:.4} | unweighted mean {:.4} mas",
        med, min, max, unweighted
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_skips_header_and_rejects_nonpositive() {
        let csv = "parallax,parallax_error\n1.0,0.1\n0.0,0.2\n-3.0,0.1\n2.0,0.2\n";
        let rows = parse_parallaxes(csv);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], (1.0, 0.1));
        assert_eq!(rows[1], (2.0, 0.2));
    }

    #[test]
    fn weighted_mean_of_two_known_rows() {
        let rows = vec![(1.0, 0.1), (2.0, 0.1)];
        let (m, e) = weighted_mean(&rows).unwrap();
        assert!((m - 1.5).abs() < 1e-9);
        let expect_err = 1.0 / (2.0 * 100.0f64).sqrt();
        assert!((e - expect_err).abs() < 1e-12);
    }
}
