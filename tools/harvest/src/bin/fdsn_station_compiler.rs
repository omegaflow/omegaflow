use omegaflow::archivar::fetch::fetch_raw_bytes;
use std::env;
use std::fs;

const BASE_URL: &str =
    "https://service.earthscope.org/fdsnws/station/1/query?level=station&format=text";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn num_cell(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

struct StationEpoch {
    net: String,
    sta: String,
    lat: Option<f64>,
    lon: Option<f64>,
    elev: Option<f64>,
    site: String,
    start: String,
    end: Option<String>,
}

fn parse_rows(body: &str) -> (Vec<StationEpoch>, usize) {
    let mut rows = Vec::new();
    let mut skipped = 0usize;
    for line in body.lines() {
        let t = line.trim_end_matches('\r');
        let t = t.trim_end();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split('|').collect();
        if cols.len() < 8 {
            skipped += 1;
            continue;
        }
        let end = cols[7].trim();
        rows.push(StationEpoch {
            net: cols[0].trim().to_string(),
            sta: cols[1].trim().to_string(),
            lat: num_cell(cols[2]),
            lon: num_cell(cols[3]),
            elev: num_cell(cols[4]),
            site: cols[5].trim().to_string(),
            start: cols[6].trim().to_string(),
            end: if end.is_empty() {
                None
            } else {
                Some(end.to_string())
            },
        });
    }
    (rows, skipped)
}

fn write_records(rows: &[StationEpoch], path: Option<&str>) -> usize {
    let mut buf = String::new();
    buf.push_str("#net|station|lat|lon|elev_m|site|start|end\n");
    for r in rows {
        let lat = match r.lat {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let lon = match r.lon {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let elev = match r.elev {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let end = r.end.as_deref().unwrap_or("");
        buf.push_str(&format!(
            "{}|{}|{}|{}|{}|{}|{}|{}\n",
            r.net, r.sta, lat, lon, elev, r.site, r.start, end
        ));
    }
    if let Some(p) = path {
        if fs::write(p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    }
    rows.len()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out = arg_value(&args, "--out");
    let network = arg_value(&args, "--network");
    let mut url = BASE_URL.to_string();
    if let Some(net) = network {
        if !net.is_empty() {
            url.push_str("&network=");
            url.push_str(&net);
        }
    }
    let bytes = match fetch_raw_bytes(&url, 600) {
        Some(b) => b,
        None => {
            eprintln!(
                "fdsn: {} carried no body — the station list stays unread",
                url
            );
            std::process::exit(1);
        }
    };
    let body = String::from_utf8_lossy(&bytes).into_owned();
    let (rows, skipped) = parse_rows(&body);
    let n = write_records(&rows, out.as_deref());
    let mut open = 0usize;
    let mut stations = std::collections::BTreeSet::new();
    let mut nets = std::collections::BTreeSet::new();
    for r in &rows {
        if r.end.is_none() {
            open += 1;
        }
        stations.insert(format!("{}.{}", r.net, r.sta));
        nets.insert(r.net.clone());
    }
    eprintln!(
        "fdsn: {} station epochs ({} currently open, {} stations, {} networks, {} lines skipped) · {}",
        n,
        open,
        stations.len(),
        nets.len(),
        skipped,
        url
    );
    if n == 0 {
        eprintln!("fdsn: the service carried no station rows — nothing fabricated");
        std::process::exit(1);
    }
    for r in rows.iter().take(3) {
        let lat = match r.lat {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let lon = match r.lon {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let elev = match r.elev {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        eprintln!(
            "  {}|{}|{}|{}|{}|{}",
            r.net, r.sta, lat, lon, elev, r.site
        );
    }
}
