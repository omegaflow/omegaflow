use std::env;
use std::path::Path;

use omegaflow::te::betti0_silverman;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_series(path: &Path) -> Option<Vec<f64>> {
    let body = std::fs::read_to_string(path).ok()?;
    Some(
        body.lines()
            .filter_map(|l| {
                let l = l.trim();
                if l.is_empty() || l.starts_with('#') {
                    return None;
                }
                l.parse::<f64>().ok()
            })
            .collect(),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(series_path) = arg_value(&args, "--series") else {
        eprintln!("usage: betti0_silverman_probe --series <path> [--dim <n>]");
        std::process::exit(2);
    };
    let dim = match arg_value(&args, "--dim") {
        Some(v) => match v.parse::<usize>() {
            Ok(n) if n >= 2 => n,
            _ => {
                eprintln!("--dim must be an integer >= 2");
                std::process::exit(2);
            }
        },
        None => 3,
    };
    let Some(series) = read_series(Path::new(&series_path)) else {
        eprintln!("series unreadable: {series_path}");
        std::process::exit(2);
    };

    let v = betti0_silverman(&series, dim);
    match v.tau {
        Some(t) => println!("tau {t}"),
        None => println!("tau absent"),
    }
    match v.threshold {
        Some(h) => println!("threshold {h:.6e}"),
        None => println!("threshold absent"),
    }
    println!("points {}", v.n_points);
    println!("components {}", v.components);
}
