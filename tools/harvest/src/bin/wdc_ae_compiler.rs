use omegaflow::cdn::upload_release;
use omegaflow::hapi_csv::parse_iso_seconds;
use omegaflow::wdc_ae::{WdcRecord, parse_bin, parse_text, write_bin};

const BASE: &str = "https://wdc.kugi.kyoto-u.ac.jp/hapi/data";
const CHUNK_S: i64 = 365 * 86400;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn fmt_iso(unix: i64) -> String {
    let days = unix.div_euclid(86400);
    let sod = unix.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y,
        m,
        d,
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let datasets: Vec<String> = match arg_value(&args, "--dataset") {
        Some(d) if d == "all" => vec!["min_ae".into(), "min_asysym".into(), "hour_dst".into()],
        Some(d) => {
            if !matches!(d.as_str(), "min_ae" | "min_asysym" | "hour_dst") {
                eprintln!("unknown --dataset {d} (min_ae|min_asysym|hour_dst|all)");
                std::process::exit(2);
            }
            vec![d]
        }
        None => vec!["min_ae".into(), "min_asysym".into(), "hour_dst".into()],
    };
    let dataset = if datasets.len() == 1 {
        datasets[0].as_str()
    } else {
        "all"
    };
    let (start, stop) = match (arg_value(&args, "--start"), arg_value(&args, "--stop")) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("wdc_ae_compiler needs --start <isotime> and --stop <isotime>");
            std::process::exit(2);
        }
    };
    let (Some(start_unix), Some(stop_unix)) = (parse_iso_seconds(&start), parse_iso_seconds(&stop))
    else {
        eprintln!("--start/--stop must be ISO8601 (YYYY-MM-DDTHH:MM:SSZ)");
        std::process::exit(2);
    };
    if stop_unix <= start_unix {
        eprintln!("--stop must lie after --start");
        std::process::exit(2);
    }
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            if datasets.len() == 1 {
                format!("wdc_ae_{dataset}.bin")
            } else {
                "wdc_ae.bin".to_string()
            }
        }
    };

    let mut records: Vec<WdcRecord> = Vec::new();
    let end = stop_unix as i64;
    for dataset in &datasets {
        let mut cursor = start_unix as i64;
        while cursor < end {
            let chunk_stop = (cursor + CHUNK_S).min(end);
            let url = format!(
                "{BASE}?id={dataset}&start={}&stop={}&format=csv",
                fmt_iso(cursor),
                fmt_iso(chunk_stop)
            );
            let Some(body) = omegaflow::archivar::fetch_raw(&url, None, &[]) else {
                eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            };
            let chunk = parse_text(&body, dataset);
            eprintln!(
                "{dataset} {} .. {}: {} records",
                fmt_iso(cursor),
                fmt_iso(chunk_stop),
                chunk.len()
            );
            records.extend(chunk);
            cursor = chunk_stop;
        }
    }
    records.sort_by(|a, b| a.t_unix.total_cmp(&b.t_unix));
    eprintln!(
        "WDC {dataset} {} .. {}: {} records",
        start,
        stop,
        records.len()
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("wdc.kugi.kyoto-u.ac.jp", &out) {
        std::process::exit(1);
    }
}
