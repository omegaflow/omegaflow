use omegaflow::archivar::agrav::{AgravRecord, parse_records, plausible_gravity, write_bin};
use omegaflow::archivar::json::{JsonVal, jstr, parse_json};
use omegaflow::archivar::{LeapSeconds, cache_root, embedded_lsk, parse_iso_tdb};
use omegaflow::cdn::upload_release;
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "api.sedoo.fr";
const BASE: &str = "https://api.sedoo.fr/get-agrav-rest";
const STATIONS_URL: &str = "https://api.sedoo.fr/get-agrav-rest/station/nearto?longitude=0&latitude=0&radiusKm=20000&institutionUuid=";
const ASSET: &str = "agrav.bin";
const USER_AGENT: &str = "omegaflow-agrav-compiler/1.0";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn http_get(url: &str) -> Option<(String, Vec<u8>)> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("120")
        .arg("-A")
        .arg(USER_AGENT)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn station_uuids(body: &[u8]) -> Option<Vec<String>> {
    let json = parse_json(std::str::from_utf8(body).ok()?)?;
    let JsonVal::Arr(items) = json else {
        return None;
    };
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for item in &items {
        if let Some(uuid) = jstr(item, "station.uuid")
            && seen.insert(uuid.clone())
        {
            out.push(uuid);
        }
    }
    Some(out)
}

fn points_url(uuid: &str) -> String {
    format!("{BASE}/point/byStationUuid?stationUuid={uuid}")
}

fn gravity_of(body: &[u8], lsk: &LeapSeconds, out: &mut Vec<AgravRecord>) -> Option<usize> {
    let json = parse_json(std::str::from_utf8(body).ok()?)?;
    let JsonVal::Arr(points) = json else {
        return None;
    };
    let mut kept = 0;
    for point in &points {
        let JsonVal::Obj(map) = point else {
            continue;
        };
        let Some(JsonVal::Arr(observations)) = map.get("observations") else {
            continue;
        };
        for observation in observations {
            let JsonVal::Obj(fields) = observation else {
                continue;
            };
            let Some(JsonVal::Num(gravity)) = fields.get("gravity") else {
                continue;
            };
            if !plausible_gravity(*gravity) {
                continue;
            }
            let Some(JsonVal::Str(reference)) = fields.get("referenceTime") else {
                continue;
            };
            let Some(epoch_tdb) = parse_iso_tdb(reference, lsk) else {
                continue;
            };
            out.push(AgravRecord {
                epoch_tdb,
                gravity: *gravity,
            });
            kept += 1;
        }
    }
    Some(kept)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("usage: agrav_compiler [--out <agrav.bin>] [--ci-mode]");
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => cache_root().join(ASSET).to_string_lossy().into_owned(),
    };

    let Some((code, station_body)) = http_get(STATIONS_URL) else {
        eprintln!("agrav: station/nearto answered void — the harvest stays pending");
        std::process::exit(1);
    };
    if code != "200" {
        eprintln!("agrav: station/nearto answered HTTP {code} — the harvest stays pending");
        std::process::exit(1);
    }
    let Some(uuids) = station_uuids(&station_body) else {
        eprintln!(
            "agrav: station/nearto did not carry the station list — the harvest stays pending"
        );
        std::process::exit(1);
    };
    if uuids.is_empty() {
        eprintln!(
            "agrav: station/nearto answered no station — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let Some(lsk) = embedded_lsk() else {
        eprintln!("agrav: the embedded leap seconds returned void — the harvest stays pending");
        std::process::exit(1);
    };

    let mut records: Vec<AgravRecord> = Vec::new();
    let mut answered = 0usize;
    for (i, uuid) in uuids.iter().enumerate() {
        let Some((code, body)) = http_get(&points_url(uuid)) else {
            continue;
        };
        if code != "200" {
            continue;
        }
        answered += 1;
        let _ = gravity_of(&body, &lsk, &mut records);
        if (i + 1) % 128 == 0 || i + 1 == uuids.len() {
            eprintln!(
                "agrav: {}/{} station(s), {} answered, {} gravity record(s)",
                i + 1,
                uuids.len(),
                answered,
                records.len()
            );
        }
    }
    if records.is_empty() {
        eprintln!(
            "agrav: no gravity observation survived the plausibility gate — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let Some(bytes) = write_bin(&records) else {
        eprintln!(
            "agrav: a held record is not finite or not serializable — the asset stays unwritten"
        );
        std::process::exit(1);
    };
    match parse_records(&bytes) {
        Some(parsed) if parsed == records => eprintln!(
            "agrav: {} gravity record(s) from {} station(s) — {} B, roundtrip reads back",
            records.len(),
            answered,
            bytes.len()
        ),
        _ => {
            eprintln!("agrav: the roundtrip does not read back — the asset stays unwritten");
            std::process::exit(1);
        }
    }
    if let Some(parent) = Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("agrav: write {out} returned void — the asset stays unwritten");
        std::process::exit(1);
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!(
            "agrav: {out} did not reach the CDN — the local asset stands, the manifest is pending"
        );
    }
}
