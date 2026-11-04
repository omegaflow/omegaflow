use omegaflow::archivar::gwosc_events::{GwEvent, parse_bin, write_bin};
use omegaflow::archivar::json::{JsonVal, jpath_val, parse_json};
use std::process::Command;

const BASE: &str = "https://gwosc.org/api/v2/catalogs";
const DEFAULT_CATALOG: &str = "GWTC-5.0";
const MAX_PAGES: usize = 64;
const FAR: &str = "far";
const SNR: &str = "network_matched_filter_snr";
const DISTANCE: &str = "luminosity_distance";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "fetch http {} for {url}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn obj(v: &JsonVal) -> Option<&std::collections::HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

fn arr(v: &JsonVal) -> Option<&Vec<JsonVal>> {
    match v {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn str_of(v: &JsonVal, key: &str) -> Option<String> {
    match obj(v)?.get(key)? {
        JsonVal::Str(s) => Some(s.clone()),
        _ => None,
    }
}

fn num_of(v: &JsonVal, key: &str) -> Option<f64> {
    match obj(v)?.get(key)? {
        JsonVal::Num(n) if n.is_finite() => Some(*n),
        _ => None,
    }
}

fn bool_of(v: &JsonVal, key: &str) -> Option<bool> {
    match obj(v)?.get(key)? {
        JsonVal::Bool(b) => Some(*b),
        _ => None,
    }
}

struct CatalogEvent {
    short_name: String,
    gps: f64,
}

fn catalog_page(catalog: &str, page: usize) -> Option<(Vec<CatalogEvent>, bool)> {
    let url = format!("{BASE}/{catalog}/events?page={page}");
    let text = fetch(&url)?;
    let json = parse_json(&text)?;
    let results = arr(jpath_val(&json, "results")?)?;
    let mut out = Vec::with_capacity(results.len());
    for r in results {
        let Some(short_name) = str_of(r, "shortName") else {
            continue;
        };
        let Some(gps) = num_of(r, "gps") else {
            continue;
        };
        out.push(CatalogEvent { short_name, gps });
    }
    let more = jpath_val(&json, "next")
        .map(|v| !matches!(v, JsonVal::Null))
        .unwrap_or(false);
    Some((out, more))
}

fn parameters_of(short_name: &str) -> Option<(Option<f64>, Option<f64>, Option<f64>)> {
    let url = format!("https://gwosc.org/api/v2/event-versions/{short_name}/parameters");
    let text = fetch(&url)?;
    let json = parse_json(&text)?;
    let results = arr(jpath_val(&json, "results")?)?;
    let chosen = results
        .iter()
        .find(|r| bool_of(r, "is_preferred") == Some(true))
        .or_else(|| results.first())?;
    let params = arr(obj(chosen)?.get("parameters")?)?;
    let mut far = None;
    let mut snr = None;
    let mut distance = None;
    for p in params {
        let Some(name) = str_of(p, "name") else {
            continue;
        };
        let Some(best) = num_of(p, "best") else {
            continue;
        };
        match name.as_str() {
            FAR => far = Some(best),
            SNR => snr = Some(best),
            DISTANCE => distance = Some(best),
            _ => {}
        }
    }
    Some((far, snr, distance))
}

fn collect(catalog: &str, limit: Option<usize>) -> Option<Vec<GwEvent>> {
    let mut events = Vec::new();
    let mut page = 1usize;
    let mut more = true;
    while more {
        if page > MAX_PAGES {
            eprintln!("catalog {catalog}: page {page} exceeds the {MAX_PAGES} page bound");
            return None;
        }
        let (rows, next) = catalog_page(catalog, page)?;
        let empty = rows.is_empty();
        events.extend(rows);
        more = next && !empty;
        page += 1;
        if let Some(n) = limit {
            if events.len() >= n {
                events.truncate(n);
                break;
            }
        }
    }
    let mut out = Vec::with_capacity(events.len());
    for e in events {
        let (far, snr, distance) = match parameters_of(&e.short_name) {
            Some(p) => p,
            None => {
                eprintln!(
                    "event {}: parameters unreadable — it stays out of the bin",
                    e.short_name
                );
                continue;
            }
        };
        if far.is_none() && snr.is_none() && distance.is_none() {
            continue;
        }
        out.push(GwEvent {
            gps: e.gps,
            far,
            snr,
            distance,
        });
    }
    Some(out)
}

fn selftest() {
    let catalog_json = r#"{"next":"https://gwosc.org/api/v2/catalogs/GWTC-5.0/events?page=2","results":[{"name":"GW150914","shortName":"GW150914-v1","gps":1126259462.4},{"name":"bad","gps":null}]}"#;
    let json = parse_json(catalog_json).expect("catalog json parses");
    let results = arr(jpath_val(&json, "results").unwrap()).unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(str_of(&results[0], "shortName").unwrap(), "GW150914-v1");
    assert_eq!(num_of(&results[0], "gps").unwrap(), 1126259462.4);
    assert!(num_of(&results[1], "gps").is_none());
    assert!(jpath_val(&json, "next").is_some());

    let param_json = r#"{"results":[{"pipeline":"gstlal","is_preferred":false,"parameters":[{"name":"far","best":9.0}]},{"pipeline":"MBTA","is_preferred":true,"parameters":[{"name":"far","best":1e-7},{"name":"network_matched_filter_snr","best":24.4},{"name":"luminosity_distance","best":410.0},{"name":"p_astro","best":0.99}]}]}"#;
    let json = parse_json(param_json).expect("parameters json parses");
    let results = arr(jpath_val(&json, "results").unwrap()).unwrap();
    let chosen = results
        .iter()
        .find(|r| bool_of(r, "is_preferred") == Some(true))
        .expect("preferred result exists");
    assert_eq!(str_of(chosen, "pipeline").unwrap(), "MBTA");
    let params = arr(obj(chosen).unwrap().get("parameters").unwrap()).unwrap();
    let best = |name: &str| {
        params.iter().find_map(|p| {
            (str_of(p, "name").as_deref() == Some(name)).then(|| num_of(p, "best")).flatten()
        })
    };
    assert_eq!(best(FAR), Some(1e-7));
    assert_eq!(best(SNR), Some(24.4));
    assert_eq!(best(DISTANCE), Some(410.0));

    let events = vec![
        GwEvent {
            gps: 1126259462.4,
            far: Some(1e-7),
            snr: Some(24.4),
            distance: Some(410.0),
        },
        GwEvent {
            gps: 1421382272.1,
            far: None,
            snr: Some(8.7),
            distance: None,
        },
    ];
    let bytes = write_bin(&events);
    assert_eq!(parse_bin(&bytes).unwrap(), events);

    println!("gwosc_events_compiler selftest: 2 events, absence preserved");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    omegaflow::archivar::cdn::require_ci();
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join("gwosc_events.bin")
            .to_string_lossy()
            .into_owned(),
    };
    let catalog = arg_value(&args, "--catalog").unwrap_or_else(|| DEFAULT_CATALOG.to_string());
    let limit = arg_value(&args, "--limit").and_then(|v| v.parse::<usize>().ok());
    if limit == Some(0) {
        eprintln!("--limit 0 carries no event");
        std::process::exit(2);
    }
    let Some(events) = collect(&catalog, limit) else {
        eprintln!("catalog {catalog}: the event list stays unreadable — the bin stays unwritten");
        std::process::exit(1);
    };
    if events.is_empty() {
        eprintln!("catalog {catalog}: no event carries a parameter — the bin stays unwritten");
        std::process::exit(1);
    }
    let with_far = events.iter().filter(|e| e.far.is_some()).count();
    let with_snr = events.iter().filter(|e| e.snr.is_some()).count();
    let with_distance = events.iter().filter(|e| e.distance.is_some()).count();
    std::fs::write(&out, write_bin(&events)).unwrap_or_else(|e| {
        eprintln!("{out}: {e}");
        std::process::exit(1);
    });
    eprintln!(
        "{out}: {} events, {with_far} with far, {with_snr} with snr, {with_distance} with distance",
        events.len()
    );
}
