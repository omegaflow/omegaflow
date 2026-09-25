use omegaflow::cdn::upload_release;

const ERDDAP_URL: &str = "https://data.pmel.noaa.gov/pmel/erddap/tabledap/pmelTaoDyW.csv?time,longitude,latitude,station,WU_422,QWS_5401&latitude>=-2&latitude<=2&longitude>=200&longitude<=280&time>={d_start}&time<={d_end}";
const COASTWATCH_URL: &str = "https://coastwatch.pfeg.noaa.gov/erddap/tabledap/pmelTaoDyW.csv?time%2Clongitude%2Clatitude%2Cstation%2CWU_422%2CQWS_5401&latitude%3E=-2&latitude%3C=2&longitude%3E=200&longitude%3C=280&time%3E={d_start}&time%3C={d_end}";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "tao_wnd_zonal.csv".to_string(),
    };
    let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => {
            eprintln!("system clock before UNIX_EPOCH — no tao wind window computable");
            std::process::exit(1);
        }
    };
    let d_end = now - 7 * 86400;
    let d_start = d_end - 120 * 86400;
    let fmt = |u: i64| {
        let days = u / 86400;
        let (y, m, d) = civil_from_days(days);
        format!("{:04}-{:02}-{:02}T00:00:00Z", y, m, d)
    };
    let body = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {} returned void: {}", path, e);
                std::process::exit(1);
            }
        },
        None => {
            let url = ERDDAP_URL
                .replace("{d_start}", &fmt(d_start))
                .replace("{d_end}", &fmt(d_end));
            let cw = COASTWATCH_URL
                .replace("{d_start}", &fmt(d_start))
                .replace("{d_end}", &fmt(d_end));
            match omegaflow::archivar::fetch_raw(&url, None, &[]) {
                Some(b) => b,
                None => {
                    eprintln!("tao wind direct fetch void — falling back to the CoastWatch mirror");
                    match omegaflow::archivar::fetch_raw(&cw, None, &[]) {
                        Some(b) => b,
                        None => {
                            eprintln!("tao wind fetch from {} returned void", cw);
                            std::process::exit(1);
                        }
                    }
                }
            }
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
        let wu: f64 = match cols[4].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !wu.is_finite() || wu.abs() >= 1.0e20 {
            continue;
        }
        if wu.abs() > 50.0 {
            continue;
        }
        let qws: f64 = match cols[5].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if qws > 2.5 {
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
            wu,
        ));
    }
    if rows.len() < 200 {
        eprintln!(
            "tao wind harvest carries only {} rows — the strip is incomplete, no file written",
            rows.len()
        );
        std::process::exit(1);
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut csv = String::from("time,station,lon,lat,wu_422\n");
    for (t, st, lon, lat, wu) in &rows {
        csv.push_str(&format!("{},{},{},{},{}\n", t, st, lon, lat, wu));
    }
    if let Err(e) = std::fs::write(&out, &csv) {
        eprintln!("write {} returned void: {}", out, e);
        std::process::exit(1);
    }
    eprintln!("tao wind harvested {} station-days → {}", rows.len(), out);
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
