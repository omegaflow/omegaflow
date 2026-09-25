use omegaflow::cdn::upload_release;
use std::io::Write;

const URL: &str = "https://transmission.bpa.gov/business/operations/gic/gic.txt";
const NETLOC: &str = "transmission.bpa.gov";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_time(s: &str) -> Option<String> {
    let (date, time) = s.trim().split_once(' ')?;
    let mut d = date.split('/');
    let m: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let y: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    if !(1..=12).contains(&m)
        || !(1..=31).contains(&day)
        || !(0..=23).contains(&hh)
        || !(0..=59).contains(&mm)
    {
        return None;
    }
    Some(format!("{y:04}-{m:02}-{day:02}T{hh:02}:{mm:02}:00"))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "bpa_gic.csv".to_string(),
    };

    let text = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("read {} returned void: {}", path, e);
                std::process::exit(1);
            }
        },
        None => match omegaflow::archivar::fetch_raw(URL, None, &[]) {
            Some(t) => t,
            None => {
                eprintln!("fetch {} returned void", URL);
                std::process::exit(1);
            }
        },
    };

    let mut stations: Vec<String> = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() >= 2 && cols[0].trim() == "Date/Time" {
            stations = cols[1..].iter().map(|c| c.trim().to_string()).collect();
            break;
        }
    }
    if stations.is_empty() {
        eprintln!("bpa-gic: no Date/Time header — the stations stay unread");
        std::process::exit(1);
    }

    let mut f = match std::fs::File::create(&out) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {} returned void: {}", out, e);
            std::process::exit(1);
        }
    };
    if f.write_all(b"time\tstation\tvalue\n").is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    let mut records: u64 = 0;
    let mut line = String::new();
    let mut in_data = false;
    for l in text.lines() {
        if !in_data {
            let cols: Vec<&str> = l.split('\t').collect();
            if cols.len() >= 2 && cols[0].trim() == "Date/Time" {
                in_data = true;
            }
            continue;
        }
        let cols: Vec<&str> = l.split('\t').collect();
        if cols.len() < 2 {
            continue;
        }
        let Some(t) = iso_time(cols[0]) else {
            continue;
        };
        for (i, station) in stations.iter().enumerate() {
            let Some(raw_val) = cols.get(i + 1) else {
                break;
            };
            let Some(v) = raw_val.trim().parse::<f64>().ok() else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            line.clear();
            line.push_str(&format!("{}\t{}\t{}\n", t, station, v));
            if f.write_all(line.as_bytes()).is_err() {
                eprintln!("write {} returned void", out);
                std::process::exit(1);
            }
            records += 1;
        }
    }
    if f.flush().is_err() {
        eprintln!("flush {} returned void", out);
        std::process::exit(1);
    }
    if records == 0 {
        eprintln!("bpa-gic: no measured Ampere sample — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!("bpa-gic harvested {} station-samples -> {}", records, out);
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("upload_release for {} returned void", out);
        std::process::exit(1);
    }
}
