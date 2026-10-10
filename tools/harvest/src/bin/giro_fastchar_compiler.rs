use omegaflow::archivar::giro_fastchar::{parse_bin, parse_text, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::hapi_csv::parse_iso_seconds;

const BASE: &str = "https://lgdc.uml.edu/fastchar/getbest";

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

fn fmt_fastchar(unix: i64) -> String {
    let days = unix.div_euclid(86400);
    let sod = unix.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}.{:02}.{:02}T{:02}:{:02}:{:02}",
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
    let Some(station) = arg_value(&args, "--station") else {
        eprintln!("giro_fastchar_compiler needs --station <URSI> (e.g. JR055)");
        std::process::exit(2);
    };
    let char_name = match arg_value(&args, "--char") {
        Some(name) => name,
        None => "foF2".to_string(),
    };
    let (start, stop) = match (arg_value(&args, "--start"), arg_value(&args, "--stop")) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("giro_fastchar_compiler needs --start <isotime> and --stop <isotime>");
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
        None => format!("giro_fastchar_{station}.bin"),
    };
    let from_date = fmt_fastchar(start_unix as i64).replace(':', "%3A");
    let to_date = fmt_fastchar(stop_unix as i64).replace(':', "%3A");
    let url = format!(
        "{BASE}?ursiCode={station}&charName={char_name}&DMUF=3000&fromDate={from_date}&toDate={to_date}"
    );
    let Some(body) = omegaflow::archivar::fetch_raw(&url, None, &[]) else {
        eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    if let Some(line) = body
        .lines()
        .find(|l| l.trim_start().starts_with("# STATUS: ERROR"))
    {
        eprintln!("getbest response: {}", line.trim());
        std::process::exit(2);
    }
    let records = parse_text(&body);
    eprintln!(
        "GIRO {station} {char_name} {start} .. {stop}: {} records",
        records.len()
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
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
    if ci_mode && !upload_release("lgdc.uml.edu", &out) {
        std::process::exit(1);
    }
}
