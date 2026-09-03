use omegaflow::cdn::upload_release;

const ERDDAP_URL: &str = "https://data.pmel.noaa.gov/pmel/erddap/tabledap/pmelTaoDyIso.csv?time,longitude,latitude,station,ISO_6,QI_5006&latitude>=-2&latitude<=2&longitude>=200&longitude<=280&time>={d_start}&time<={d_end}";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "d20_thermocline.csv".to_string());
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let d_end = now - 7 * 86400;
    let d_start = d_end - 120 * 86400;
    let fmt = |u: i64| {
        let days = u / 86400;
        let (y, m, d) = civil_from_days(days);
        format!("{:04}-{:02}-{:02}T00:00:00Z", y, m, d)
    };
    let url = ERDDAP_URL
        .replace("{d_start}", &fmt(d_start))
        .replace("{d_end}", &fmt(d_end));
    let body = match omegaflow::archivar::fetch_raw_bytes(&url, 60) {
        Some(b) => String::from_utf8_lossy(&b).into_owned(),
        None => {
            eprintln!("d20 fetch from {} returned void", url);
            std::process::exit(1);
        }
    };
    let mut rows: Vec<(String, String, f64, f64, f64)> = Vec::new();
    let mut line_n = 0;
    for line in body.lines() {
        line_n += 1;
        if line_n <= 2 {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 6 {
            continue;
        }
        let iso: f64 = match cols[4].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !iso.is_finite() || iso.abs() >= 1.0e20 {
            continue;
        }
        if iso <= 0.0 || iso > 500.0 {
            continue;
        }
        let qi: f64 = match cols[5].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if qi > 2.5 {
            continue;
        }
        let lon: f64 = match cols[1].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let lat: f64 = match cols[2].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        rows.push((
            cols[0].trim().to_string(),
            cols[3].trim().to_string(),
            lon,
            lat,
            iso,
        ));
    }
    if rows.len() < 200 {
        eprintln!(
            "d20 harvest carries only {} rows — the strip is incomplete, no file written",
            rows.len()
        );
        std::process::exit(1);
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut csv = String::from("time,station,lon,lat,iso_6\n");
    for (t, st, lon, lat, iso) in &rows {
        csv.push_str(&format!("{},{},{},{},{}\n", t, st, lon, lat, iso));
    }
    if let Err(e) = std::fs::write(&out, &csv) {
        eprintln!("write {} returned void: {}", out, e);
        std::process::exit(1);
    }
    eprintln!("d20 harvested {} station-days → {}", rows.len(), out);
    if ci_mode && !upload_release("data.pmel.noaa.gov", &out) {
        eprintln!("upload_release for {} returned void", out);
        std::process::exit(1);
    }
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}
