use omegaflow::cdn::upload_release;
use omegaflow::hapi_csv::parse_iso_seconds;
use omegaflow::substorm::{SubstormOnset, parse_bin, parse_csv, write_bin};

const BASE: &str = "https://supermag.jhuapl.edu/lib/services/";
const CHUNK_S: i64 = 25 * 365 * 86400;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_to_unix(s: &str) -> Option<i64> {
    if s.len() >= 19 {
        parse_iso_seconds(s).map(|v| v as i64)
    } else if s.len() == 10 {
        parse_iso_seconds(&format!("{s}T00:00:00Z")).map(|v| v as i64)
    } else {
        None
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

fn fmt_date(unix: i64) -> String {
    let days = unix.div_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let list = match arg_value(&args, "--list") {
        Some(l) => l,
        None => {
            eprintln!("substorm_compiler needs --list <newell|forsyth|liou|frey|ohtani>");
            std::process::exit(2);
        }
    };
    if !matches!(
        list.as_str(),
        "newell" | "forsyth" | "liou" | "frey" | "ohtani"
    ) {
        eprintln!("unknown --list {list} (newell|forsyth|liou|frey|ohtani)");
        std::process::exit(2);
    }
    let (start, stop) = match (arg_value(&args, "--start"), arg_value(&args, "--stop")) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!("substorm_compiler needs --start <isotime> and --stop <isotime>");
            std::process::exit(2);
        }
    };
    let (Some(start_unix), Some(stop_unix)) = (iso_to_unix(&start), iso_to_unix(&stop)) else {
        eprintln!("--start/--stop must be ISO8601 (YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ)");
        std::process::exit(2);
    };
    if stop_unix <= start_unix {
        eprintln!("--stop must lie after --start");
        std::process::exit(2);
    }
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => format!("substorm_{list}.bin"),
    };

    let mut records: Vec<SubstormOnset> = Vec::new();
    let end = stop_unix;
    let mut cursor = start_unix;
    while cursor < end {
        let chunk_stop = (cursor + CHUNK_S).min(end);
        let url = format!(
            "{BASE}?service=substorms&downloadtype=substorm_list&fmt=csv&start={}&end={}&list={list}",
            fmt_date(cursor),
            fmt_date(chunk_stop)
        );
        let Some(body) = omegaflow::archivar::fetch_raw(&url, None, &[]) else {
            eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
            std::process::exit(1);
        };
        let chunk = parse_csv(&body);
        eprintln!(
            "{list} {} .. {}: {} onsets",
            fmt_date(cursor),
            fmt_date(chunk_stop),
            chunk.len()
        );
        records.extend(chunk);
        cursor = chunk_stop;
    }
    records.sort_by(|a, b| a.t_unix.total_cmp(&b.t_unix));
    eprintln!(
        "substorm {list} {start} .. {stop}: {} onsets",
        records.len()
    );
    if records.is_empty() {
        eprintln!("no onsets — the bin stays unwritten (0 honored)");
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
                "{out}: {} onsets, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("supermag.jhuapl.edu", &out) {
        std::process::exit(1);
    }
}
