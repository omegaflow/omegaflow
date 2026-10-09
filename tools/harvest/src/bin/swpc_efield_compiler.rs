use omegaflow::archivar::embedded_lsk;
use omegaflow::cdn::upload_release;
use omegaflow::hapi_csv::parse_iso_seconds;
use omegaflow::swpc_efield::{COMP_EX, COMP_EY, parse_bin, parse_frame, write_bin};

const BASE: &str = "https://services.swpc.noaa.gov/json/lists/rgeojson/US-Canada-1D/";
const SUFFIX: &str = "-15-Efield-US-Canada.json";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn index_frames(html: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut rest = html;
    while let Some(pos) = rest.find("href=\"") {
        let tail = &rest[pos + 6..];
        let end = tail.find('"').unwrap_or(tail.len());
        let name = &tail[..end];
        if name.ends_with(SUFFIX) && !out.iter().any(|n| n == name) {
            out.push(name.to_string());
        }
        rest = &tail[end..];
    }
    out
}

fn filename_unix(name: &str) -> Option<f64> {
    let b = name.as_bytes();
    if b.len() < 15 || b[8] != b'T' {
        return None;
    }
    let d = name.get(0..8)?;
    let t = name.get(9..15)?;
    let iso = format!(
        "{}-{}-{}T{}:{}:{}Z",
        d.get(0..4)?,
        d.get(4..6)?,
        d.get(6..8)?,
        t.get(0..2)?,
        t.get(2..4)?,
        t.get(4..6)?
    );
    parse_iso_seconds(&iso)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let (start, stop) = match (arg_value(&args, "--start"), arg_value(&args, "--stop")) {
        (Some(s), Some(e)) => (s, e),
        _ => {
            eprintln!(
                "swpc_efield_compiler needs --start <iso> and --stop <iso> (YYYY-MM-DDTHH:MM:SSZ)"
            );
            std::process::exit(2);
        }
    };
    let (Some(start_unix), Some(stop_unix)) = (parse_iso_seconds(&start), parse_iso_seconds(&stop))
    else {
        eprintln!("--start/--stop carry no ISO8601 time (YYYY-MM-DDTHH:MM:SSZ)");
        std::process::exit(2);
    };
    if stop_unix <= start_unix {
        eprintln!("--stop {stop} lies before --start {start}");
        std::process::exit(2);
    }
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("swpc_efield_compiler needs --out <path>");
            std::process::exit(2);
        }
    };

    let Some(lsk) = embedded_lsk() else {
        eprintln!("embedded leap-second table reads void — the epoch stays unbuilt");
        std::process::exit(1);
    };
    let Some(index) = omegaflow::archivar::fetch_raw(BASE, None, &[]) else {
        eprintln!("{BASE}: index fetch void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let index_text = &index;

    let mut rows: Vec<(f64, f64, u32)> = Vec::new();
    let mut frames = 0usize;
    for name in index_frames(index_text) {
        let Some(t_unix) = filename_unix(&name) else {
            eprintln!("{name}: filename carries no frame stamp");
            continue;
        };
        if t_unix < start_unix || t_unix > stop_unix {
            continue;
        }
        let Some(tdb) = lsk.unix_to_tdb(t_unix) else {
            eprintln!("{name}: unix {t_unix} lies outside the leap-second table");
            continue;
        };
        let url = format!("{BASE}{name}");
        let Some(body) = omegaflow::archivar::fetch_raw(&url, None, &[]) else {
            eprintln!("{name}: fetch void — the frame stays unharvested");
            continue;
        };
        let Some((ex, ey)) = parse_frame(body.as_bytes()) else {
            eprintln!("{name}: frame carries no measurable field (0 honored)");
            continue;
        };
        if let Some(v) = ex {
            rows.push((tdb, v, COMP_EX));
        }
        if let Some(v) = ey {
            rows.push((tdb, v, COMP_EY));
        }
        frames += 1;
        eprintln!(
            "{name}: Ex {} / Ey {}",
            ex.map_or(String::from("-"), |v| v.to_string()),
            ey.map_or(String::from("-"), |v| v.to_string())
        );
    }
    rows.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    eprintln!(
        "SWPC E-field {start} .. {stop}: {frames} frames, {} rows",
        rows.len()
    );
    if rows.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&rows);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} rows, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("services.swpc.noaa.gov", &out) {
        std::process::exit(1);
    }
}
