use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;

const URL: &str = "https://fermi.gsfc.nasa.gov/ssc/data/access/gbm/tgf/gbm_tgf_catalog_offline.csv";
const NETLOC: &str = "fermi.gsfc.nasa.gov";
const MIN_ROWS: usize = 1000;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn cells(line: &str) -> Vec<&str> {
    line.split(',').map(|c| c.trim()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "gbm_tgf_catalog_offline.csv".to_string(),
    };

    let raw = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {} returned void: {}", path, e);
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(URL) {
            Some(b) => b,
            None => {
                eprintln!("fetch {} returned void", URL);
                std::process::exit(1);
            }
        },
    };
    let text = String::from_utf8_lossy(&raw).into_owned();

    let Some(header_line) = text.lines().find(|l| !l.trim().is_empty()) else {
        eprintln!("gbm_tgf: the catalog carries no header line");
        std::process::exit(1);
    };
    let header = cells(header_line.trim_start_matches('#'));
    let col =
        |key: &str| -> Option<usize> { header.iter().position(|h| h.eq_ignore_ascii_case(key)) };
    let (Some(date_i), Some(utc_i), Some(lat_i), Some(lon_i)) =
        (col("Date"), col("UTC"), col("Lat"), col("Lon"))
    else {
        eprintln!(
            "gbm_tgf: header carries no Date/UTC/Lat/Lon — the catalog stays unwritten (0 honored)"
        );
        std::process::exit(1);
    };

    let mut csv = String::from("time,");
    csv.push_str(&header.join(","));
    csv.push('\n');
    let mut records: usize = 0;
    let mut skipped: usize = 0;
    for line in text.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols = cells(line);
        let (Some(date), Some(utc), Some(lat), Some(lon)) = (
            cols.get(date_i).copied(),
            cols.get(utc_i).copied(),
            cols.get(lat_i).copied(),
            cols.get(lon_i).copied(),
        ) else {
            skipped += 1;
            continue;
        };
        let lat = lat.trim_start_matches('+');
        let lon = lon.trim_start_matches('+');
        let Ok(lat_v) = lat.parse::<f64>() else {
            skipped += 1;
            continue;
        };
        let Ok(lon_v) = lon.parse::<f64>() else {
            skipped += 1;
            continue;
        };
        if !lat_v.is_finite() || !lon_v.is_finite() || !date.contains('-') || !utc.contains(':') {
            skipped += 1;
            continue;
        }
        csv.push_str(&format!("{date}T{utc},"));
        csv.push_str(&cols.join(","));
        csv.push('\n');
        records += 1;
    }

    if records < MIN_ROWS {
        eprintln!(
            "gbm_tgf: the reader carries only {} rows (expected >= {}) — the asset stays unwritten (0 honored)",
            records, MIN_ROWS
        );
        std::process::exit(1);
    }
    if let Err(e) = std::fs::write(&out, &csv) {
        eprintln!("write {} returned void: {}", out, e);
        std::process::exit(1);
    }
    eprintln!(
        "gbm_tgf: read {} rows with an ISO epoch ({skipped} skipped) -> {}",
        records, out
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("upload_release for {} returned void", out);
        std::process::exit(1);
    }
}
