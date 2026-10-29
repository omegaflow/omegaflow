use omegaflow::archivar::geoid::GeoidModel;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cap = 360usize;
    let mut points: Vec<(f64, f64)> = Vec::new();
    let mut models: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--model" => {
                if let Some(p) = args.get(i + 1) {
                    models.push(p.clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--point" => {
                if let Some(v) = args.get(i + 1)
                    && let Some((lat, lon)) = v.split_once(',')
                    && let (Ok(lat), Ok(lon)) =
                        (lat.trim().parse::<f64>(), lon.trim().parse::<f64>())
                {
                    points.push((lat, lon));
                }
                i += 2;
            }
            "--cap" => {
                if let Some(v) = args.get(i + 1).and_then(|v| v.parse::<usize>().ok()) {
                    cap = v;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    if models.is_empty() {
        eprintln!("usage: geoid_probe --model <file.gfc> [--model ...] [--cap <degree>]");
        return;
    }
    let stations: Vec<(f64, f64, String)> = if points.is_empty() {
        vec![
            (52.52, 13.40, "Berlin".to_string()),
            (21.30, -157.86, "Honolulu".to_string()),
            (-33.93, 18.42, "Kapstadt".to_string()),
        ]
    } else {
        points
            .iter()
            .map(|(lat, lon)| (*lat, *lon, format!("{lat:.2},{lon:.2}")))
            .collect()
    };
    for path in &models {
        let Ok(text) = std::fs::read_to_string(path) else {
            println!("# {path}: unreadable");
            continue;
        };
        let Some(model) = GeoidModel::parse(&text, cap) else {
            println!("# {path}: no radius or no coefficients");
            continue;
        };
        for (lat, lon, name) in &stations {
            match model.geoid_m(*lat, *lon) {
                Some(n) => println!("{path}\t{name}\t{lat:.2}\t{lon:.2}\t{n:.3}"),
                None => println!("{path}\t{name}\t{lat:.2}\t{lon:.2}\tabsent"),
            }
        }
        println!("# {path}: R {:.1} m, cap {cap}", model.radius_m);
    }
}
