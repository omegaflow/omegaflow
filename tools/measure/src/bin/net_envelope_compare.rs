use omegaflow::intermagnet;

fn read_envelope(path: &str) -> Vec<(f64, f64)> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("net_envelope_compare: {path} read void: {e}");
            std::process::exit(1);
        }
    };
    let mut series: Vec<(f64, f64)> = match intermagnet::parse_bin(&bytes) {
        Some(records) => records.into_iter().map(|(t, v, _)| (t, v)).collect(),
        None => {
            eprintln!("net_envelope_compare: {path} is not an IMDT envelope");
            std::process::exit(1);
        }
    };
    series.sort_by(|a, b| a.0.total_cmp(&b.0));
    series
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let idx = (((sorted.len() - 1) as f64) * q).round() as usize;
    sorted[idx]
}

fn median_abs_dev(sorted: &[f64]) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let med = quantile(sorted, 0.5);
    let mut dev: Vec<f64> = sorted.iter().map(|v| (v - med).abs()).collect();
    dev.sort_by(|a, b| a.total_cmp(b));
    quantile(&dev, 0.5)
}

fn compare_envelopes(a: &[(f64, f64)], b: &[(f64, f64)]) {
    let (mut ia, mut ib) = (0usize, 0usize);
    let mut shared = 0usize;
    let mut only_a = 0usize;
    let mut only_b = 0usize;
    let mut above = 0usize;
    let mut worst = f64::NEG_INFINITY;
    while ia < a.len() && ib < b.len() {
        let ta = a[ia].0 as i64;
        let tb = b[ib].0 as i64;
        if ta == tb {
            shared += 1;
            if b[ib].1 > a[ia].1 {
                above += 1;
                let gap = b[ib].1 - a[ia].1;
                if gap > worst {
                    worst = gap;
                }
            }
            ia += 1;
            ib += 1;
        } else if ta < tb {
            only_a += 1;
            ia += 1;
        } else {
            only_b += 1;
            ib += 1;
        }
    }
    only_a += a.len() - ia;
    only_b += b.len() - ib;
    let worst = if worst.is_finite() { worst } else { 0.0 };
    println!(
        "pair |smaller|={} |larger|={} shared={shared} only_in_smaller={only_a} only_in_larger={only_b} larger_above_smaller={above} worst_gap={worst:.4}",
        a.len(),
        b.len()
    );
}

fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.len() < 2 {
        eprintln!(
            "net_envelope_compare needs at least two envelope bins in growing station count: <bin_smaller_n> <bin_larger_n> [...]"
        );
        std::process::exit(2);
    }
    let series: Vec<Vec<(f64, f64)>> = paths.iter().map(|p| read_envelope(p)).collect();
    for (path, s) in paths.iter().zip(series.iter()) {
        let mut vals: Vec<f64> = s
            .iter()
            .map(|(_, v)| *v)
            .filter(|v| v.is_finite())
            .collect();
        vals.sort_by(|a, b| a.total_cmp(b));
        println!(
            "envelope {path} samples={} mad={:.4} p50={:.4} p90={:.4} p99={:.4}",
            s.len(),
            median_abs_dev(&vals),
            quantile(&vals, 0.5),
            quantile(&vals, 0.9),
            quantile(&vals, 0.99)
        );
    }
    for (i, w) in series.windows(2).enumerate() {
        println!("comparison {} -> {}", paths[i], paths[i + 1]);
        compare_envelopes(&w[0], &w[1]);
    }
}
