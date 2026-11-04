use omegaflow::archivar::geoid::{GeoidModel, StationValue, exact_radius, fixture};

struct ModelArg {
    label: String,
    path: String,
    cap: usize,
}

fn split_model(arg: &str) -> Option<ModelArg> {
    let (label, rest) = arg.split_once('=')?;
    let (path, cap) = rest.rsplit_once(':')?;
    Some(ModelArg {
        label: label.to_string(),
        path: path.to_string(),
        cap: cap.parse().ok()?,
    })
}

fn station_rows(rows: &[StationValue]) -> Vec<(String, f64, f64)> {
    let mut out: Vec<(String, f64, f64)> = Vec::new();
    for row in rows {
        if !out.iter().any(|(name, _, _)| name == &row.station) {
            out.push((row.station.clone(), row.lat_deg, row.lon_deg));
        }
    }
    out
}

fn cell<'a>(
    rows: &'a [StationValue],
    station: &str,
    source: &str,
    quantity: &str,
    cap: Option<usize>,
) -> Option<&'a StationValue> {
    rows.iter().find(|r| {
        r.station == station
            && r.source == source
            && r.quantity == quantity
            && cap.map(|c| c == r.cap).unwrap_or(true)
    })
}

fn delta(actual: Option<f64>, expected: Option<f64>) -> String {
    match (actual, expected) {
        (Some(a), Some(e)) => format!("{:+.4}", a - e),
        _ => "absent".to_string(),
    }
}

fn value(actual: Option<f64>) -> String {
    match actual {
        Some(a) => format!("{a:.4}"),
        None => "absent".to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut models: Vec<ModelArg> = Vec::new();
    let mut grids: Vec<(String, String)> = Vec::new();
    let mut bruns_source = String::from("ours EGM2008 bruns");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--model" => {
                if let Some(v) = args.get(i + 1)
                    && let Some(m) = split_model(v)
                {
                    models.push(m);
                }
                i += 2;
            }
            "--grid" => {
                if let Some(v) = args.get(i + 1)
                    && let Some((label, grid)) = v.split_once('=')
                {
                    grids.push((label.to_string(), grid.to_string()));
                }
                i += 2;
            }
            "--bruns" => {
                if let Some(v) = args.get(i + 1) {
                    bruns_source = v.clone();
                }
                i += 2;
            }
            _ => i += 1,
        }
    }
    if models.is_empty() {
        eprintln!(
            "usage: geoid_chain --model \"<quelle>=<pfad>:<grad>\" [--model ...] --grid \"<quelle>=<gitterquelle>\" [--bruns <tabellenquelle>]"
        );
        return;
    }
    let Some(rows) = fixture() else {
        println!("# the register table is not readable");
        return;
    };
    let stations = station_rows(&rows);
    for arg in &models {
        let Ok(text) = std::fs::read_to_string(&arg.path) else {
            println!("# {}: unreadable", arg.path);
            continue;
        };
        let Some(model) = GeoidModel::parse(&text, arg.cap) else {
            println!("# {}: no radius or no coefficients", arg.path);
            continue;
        };
        let Some((_, grid_label)) = grids.iter().find(|(label, _)| label == &arg.label) else {
            println!("# {}: no grid assigned", arg.label);
            continue;
        };
        println!("# {}  degree {}  grid {}", arg.label, arg.cap, grid_label);
        let mut w_at_grid: Vec<f64> = Vec::new();
        for (name, lat, lon) in &stations {
            if let Some(grid) = cell(&rows, name, grid_label, "n", None)
                && let Some(parts) =
                    model.potential_parts(exact_radius(*lat) + grid.value, *lat, *lon)
            {
                w_at_grid.push(parts.0);
            }
        }
        if w_at_grid.is_empty() {
            println!("#   no potential at the grid surface");
            continue;
        }
        let w0 = w_at_grid.iter().sum::<f64>() / w_at_grid.len() as f64;
        let spread = w_at_grid.iter().cloned().fold(f64::MIN, f64::max)
            - w_at_grid.iter().cloned().fold(f64::MAX, f64::min);
        println!(
            "#   W0 from the grid {w0:.4}  spread {spread:.4}  ({} points)",
            w_at_grid.len()
        );
        let service_label = arg.label.replace("ours ", "ICGEM ");
        for (name, lat, lon) in &stations {
            let potential = model
                .potential_parts(exact_radius(*lat), *lat, *lon)
                .map(|p| p.0);
            let closed = model.geoid_m(*lat, *lon);
            let bruns = model.geoid_m_bruns(*lat, *lon, w0);
            let gravity_only = model
                .potential_parts(exact_radius(*lat), *lat, *lon)
                .map(|p| p.1);
            let grid = cell(&rows, name, grid_label, "n", None);
            let table_w = cell(&rows, name, &arg.label, "w", Some(arg.cap));
            let table_n = cell(&rows, name, &arg.label, "n", Some(arg.cap));
            let table_bruns = cell(&rows, name, &bruns_source, "n", None);
            let table_service = cell(&rows, name, &service_label, "w", Some(arg.cap));
            let table_service_g = cell(&rows, name, &service_label, "g", Some(arg.cap));
            println!(
                "{name:<13} W {}  dTable {}  dService {} | G {}  dService {} | N closed {}  dTable {} | N bruns {}  dGrid {}  dTable {}",
                value(potential),
                delta(potential, table_w.map(|r| r.value)),
                delta(potential, table_service.map(|r| r.value)),
                value(gravity_only),
                delta(gravity_only, table_service_g.map(|r| r.value)),
                value(closed),
                delta(closed, table_n.map(|r| r.value)),
                value(bruns),
                delta(bruns, grid.map(|r| r.value)),
                delta(bruns, table_bruns.map(|r| r.value)),
            );
        }
    }
}
