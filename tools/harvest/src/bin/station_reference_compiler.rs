use std::path::PathBuf;

struct Row {
    station: String,
    x_m: f64,
    y_m: f64,
    z_m: f64,
    vx_m_per_year: f64,
    vy_m_per_year: f64,
    vz_m_per_year: f64,
    epoch_year: f64,
    reference: String,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_epoch(text: &str) -> Option<f64> {
    for line in text.lines() {
        let Some(index) = line.find("epoch") else {
            continue;
        };
        let rest = &line[index + "epoch".len()..];
        let token = rest
            .trim_start_matches([' ', ':'])
            .split(|c: char| !(c.is_ascii_digit() || c == '.'))
            .next()?;
        let token = token.trim_end_matches('.');
        if let Ok(value) = token.parse::<f64>()
            && token.contains('.')
        {
            return Some(value);
        }
    }
    None
}

fn parse_jpl(text: &str, reference: &str, epoch_year: f64) -> (Vec<Row>, usize) {
    let mut rows = Vec::new();
    let mut skipped = 0usize;
    for line in text.lines() {
        let trimmed = line.trim();
        let mut parts = trimmed.split_whitespace();
        let Some(raw_number) = parts.next() else {
            continue;
        };
        let digits: String = raw_number
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if digits.is_empty() {
            continue;
        }
        if raw_number.ends_with('*') {
            skipped += 1;
            continue;
        }
        let _diameter = parts.next();
        let _kind = parts.next();
        let mut numbers = [0.0f64; 6];
        let mut complete = true;
        for slot in &mut numbers {
            match parts.next().and_then(|p| p.parse::<f64>().ok()) {
                Some(value) if value.is_finite() => *slot = value,
                _ => {
                    complete = false;
                    break;
                }
            }
        }
        if !complete {
            skipped += 1;
            continue;
        }
        rows.push(Row {
            station: format!("DSS-{digits}"),
            x_m: numbers[0],
            y_m: numbers[1],
            z_m: numbers[2],
            vx_m_per_year: numbers[3],
            vy_m_per_year: numbers[4],
            vz_m_per_year: numbers[5],
            epoch_year,
            reference: reference.to_string(),
        });
    }
    (rows, skipped)
}

fn parse_extra(path: &PathBuf) -> Option<Vec<Row>> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split('\t');
        let station = parts.next()?.trim().to_string();
        let numbers: Vec<f64> = (0..7)
            .map(|_| parts.next().and_then(|p| p.trim().parse::<f64>().ok()))
            .collect::<Option<Vec<f64>>>()?;
        let reference = parts.next().unwrap_or("").trim().to_string();
        rows.push(Row {
            station,
            x_m: numbers[0],
            y_m: numbers[1],
            z_m: numbers[2],
            vx_m_per_year: numbers[3],
            vy_m_per_year: numbers[4],
            vz_m_per_year: numbers[5],
            epoch_year: numbers[6],
            reference,
        });
    }
    Some(rows)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(jpl_path) = arg_value(&args, "--jpl").map(PathBuf::from) else {
        eprintln!(
            "station_reference: --jpl <stations_19jul2013.txt> absent — the call names no table"
        );
        std::process::exit(2);
    };
    let Some(body) = arg_value(&args, "--body") else {
        eprintln!("station_reference: --body <name> absent — the station frame needs its body");
        std::process::exit(2);
    };
    let extra_path = arg_value(&args, "--extra").map(PathBuf::from);
    let out_path = arg_value(&args, "--out").map(PathBuf::from);

    let Ok(text) = std::fs::read_to_string(&jpl_path) else {
        eprintln!("station_reference: {} is not readable", jpl_path.display());
        std::process::exit(1);
    };
    let Some(epoch_year) = parse_epoch(&text) else {
        eprintln!(
            "station_reference: {} carries no reference epoch — the table stays unread",
            jpl_path.display()
        );
        std::process::exit(1);
    };
    let reference = jpl_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();
    let (mut rows, skipped) = parse_jpl(&text, &reference, epoch_year);
    if let Some(path) = &extra_path {
        match parse_extra(path) {
            Some(mut extra) => rows.append(&mut extra),
            None => {
                eprintln!(
                    "station_reference: {} is not readable — the extra stations stay absent",
                    path.display()
                );
                std::process::exit(1);
            }
        }
    }

    let mut out = String::from(
        "# station\tbody\tx_m\ty_m\tz_m\tvx_m_per_year\tvy_m_per_year\tvz_m_per_year\tepoch_year\treference\n",
    );
    for row in &rows {
        out.push_str(&format!(
            "{}\t{}\t{:.4}\t{:.4}\t{:.4}\t{:.6}\t{:.6}\t{:.6}\t{:.4}\t{}\n",
            row.station,
            body,
            row.x_m,
            row.y_m,
            row.z_m,
            row.vx_m_per_year,
            row.vy_m_per_year,
            row.vz_m_per_year,
            row.epoch_year,
            row.reference
        ));
    }
    match out_path {
        Some(path) => {
            if let Err(err) = std::fs::write(&path, out) {
                eprintln!(
                    "station_reference: {} carries no write ({err})",
                    path.display()
                );
                std::process::exit(1);
            }
            eprintln!(
                "station_reference: {} station(s), {skipped} old position(s) skipped, epoch {epoch_year:.4} — {}",
                rows.len(),
                path.display()
            );
        }
        None => {
            print!("{out}");
            eprintln!(
                "station_reference: {} station(s), {skipped} old position(s) skipped, epoch {epoch_year:.4}",
                rows.len()
            );
        }
    }
}
