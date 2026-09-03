use omegaflow::cdn::upload_asset;

const QBO_URL: &str = "https://www.cpc.ncep.noaa.gov/data/indices/qbo.u30.index";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "qbo_30hpa.csv".to_string());
    let body = match omegaflow::archivar::fetch_raw_bytes(QBO_URL, 30) {
        Some(b) => String::from_utf8_lossy(&b).into_owned(),
        None => {
            eprintln!("qbo fetch from {} returned void", QBO_URL);
            std::process::exit(1);
        }
    };
    let mut rows: Vec<(i64, u32, f64)> = Vec::new();
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty()
            || t.starts_with("30 mb")
            || t.starts_with("YEAR")
            || t.starts_with("ORIGINAL")
            || t.starts_with("PREDICTED")
            || t.starts_with('*')
        {
            continue;
        }
        let cols: Vec<&str> = t.split_whitespace().collect();
        if cols.len() < 13 {
            continue;
        }
        let year: i64 = match cols[0].parse() {
            Ok(y) if y >= 1979 => y,
            _ => continue,
        };
        for (m, c) in cols.iter().enumerate().take(12).skip(1) {
            let val: f64 = match c.trim().parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            if !val.is_finite() || val.abs() >= 900.0 {
                continue;
            }
            rows.push((year, m as u32, val));
        }
    }
    if rows.len() < 12 * 10 {
        eprintln!(
            "qbo table carries only {} cells — the harvest is incomplete, no file written",
            rows.len()
        );
        std::process::exit(1);
    }
    let mut csv = String::from("year,month,day,qbo_30hpa_ms\n");
    for (y, m, v) in &rows {
        csv.push_str(&format!("{},{},{},{}\n", y, m, 1, v));
    }
    if let Err(e) = std::fs::write(&out, &csv) {
        eprintln!("write {} returned void: {}", out, e);
        std::process::exit(1);
    }
    eprintln!("qbo transposed {} cells → {}", rows.len(), out);
    if ci_mode && !upload_asset(&out) {
        eprintln!("upload_asset for {} returned void", out);
        std::process::exit(1);
    }
}
