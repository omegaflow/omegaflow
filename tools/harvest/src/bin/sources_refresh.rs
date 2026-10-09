use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::spectral::civil_from_days;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const USER_AGENT: &str = "omegaflow-catalogs-bot/1.0";

const DEFAULT_SPEC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/sources_refresh.spec.json"
);

fn spec_list(spec: &JsonVal, key: &str) -> Vec<(String, String)> {
    let JsonVal::Obj(root) = spec else {
        return Vec::new();
    };
    let Some(JsonVal::Arr(items)) = root.get(key) else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|item| {
            let JsonVal::Obj(map) = item else {
                return None;
            };
            let name = match map.get("name") {
                Some(JsonVal::Str(s)) => s.clone(),
                _ => return None,
            };
            let command = match map.get("command") {
                Some(JsonVal::Str(s)) => s.clone(),
                _ => return None,
            };
            Some((name, command))
        })
        .collect()
}

const NDBC_BUOYS: &[u32] = &[
    41001, 42001, 42002, 42036, 42040, 42055, 44009, 44013, 44014, 44025, 46001, 46002, 46005,
    46006, 46012, 46013, 46022, 46025, 46026, 46027, 46029, 46047, 46053, 46054, 46059, 46069,
    46086, 51000, 51001, 51002, 51004,
];

const SWPC_URLS: &[(&str, &str)] = &[
    (
        "swpc_ace",
        "https://services.swpc.noaa.gov/products/ace-swepam-1-day.json",
    ),
    (
        "swpc_dscovr",
        "https://services.swpc.noaa.gov/products/dscovr-1-day.json",
    ),
    (
        "swpc_mag_1m",
        "https://services.swpc.noaa.gov/products/solar-wind/mag-1-day.json",
    ),
    (
        "swpc_plasma_1m",
        "https://services.swpc.noaa.gov/products/solar-wind/plasma-1-day.json",
    ),
];

fn fetch_bytes(url: &str) -> (u32, Option<Vec<u8>>) {
    let output = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-m")
        .arg("180")
        .arg("--retry")
        .arg("2")
        .arg("-A")
        .arg(USER_AGENT)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output();
    let out = match output {
        Ok(o) => o,
        Err(e) => {
            eprintln!("  {url}: curl reads void — {e}");
            return (0, None);
        }
    };
    let stdout = out.stdout;
    let Some(idx) = stdout.iter().rposition(|&b| b == b'\n') else {
        return (0, None);
    };
    let code: u32 = match String::from_utf8_lossy(&stdout[idx + 1..]).trim().parse() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("  {url}: http code unreadable — {e}");
            return (0, None);
        }
    };
    (code, Some(stdout[..idx].to_vec()))
}

fn save_if_changed(out_dir: &str, name: &str, bytes: &[u8]) -> bool {
    let dir = std::path::Path::new(out_dir);
    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("  {name}: {out_dir} stays unformed — {e}");
        return false;
    }
    let path = dir.join(format!("{name}.json"));
    if let Ok(old) = std::fs::read(&path)
        && sha256_hex(&old) == sha256_hex(bytes)
    {
        eprintln!("  {name}: unchanged");
        return false;
    }
    match std::fs::write(&path, bytes) {
        Ok(()) => {
            eprintln!("  {name}: updated ({}B)", bytes.len());
            true
        }
        Err(e) => {
            eprintln!("  {name}: write void — {e}");
            false
        }
    }
}

fn json_text_envelope(text: &str) -> Vec<u8> {
    let mut json = String::with_capacity(text.len() + 16);
    json.push_str("{\"text\": \"");
    for c in text.chars() {
        match c {
            '"' => json.push_str("\\\""),
            '\\' => json.push_str("\\\\"),
            '\n' => json.push_str("\\n"),
            '\r' => json.push_str("\\r"),
            '\t' => json.push_str("\\t"),
            c if (c as u32) < 0x20 => json.push_str(&format!("\\u{:04x}", c as u32)),
            c => json.push(c),
        }
    }
    json.push_str("\"}");
    json.into_bytes()
}

fn url_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'.' | b'-' | b'~' | b'/' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn utc_date(offset_days: i64) -> Option<String> {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let days = secs.div_euclid(86400) + offset_days;
    let (y, m, d) = civil_from_days(days)?;
    Some(format!("{y:04}-{m:02}-{d:02}"))
}

fn refresh_ndbc(out_dir: &str) -> bool {
    let mut changed = false;
    for &buoy_id in NDBC_BUOYS {
        let url = format!("https://www.ndbc.noaa.gov/data/realtime2/{buoy_id}.txt");
        let (status, body) = fetch_bytes(&url);
        match body {
            Some(bytes) if status == 200 && !bytes.is_empty() => {
                let text = String::from_utf8_lossy(&bytes);
                changed |= save_if_changed(
                    out_dir,
                    &format!("ndbc_{buoy_id}"),
                    &json_text_envelope(&text),
                );
            }
            _ => eprintln!("  NDBC {buoy_id}: HTTP {status} or empty"),
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    changed
}

fn refresh_horizons(out_dir: &str, bodies: &[(String, String)]) -> bool {
    let (Some(today), Some(tomorrow)) = (utc_date(0), utc_date(1)) else {
        eprintln!("  horizons: the UTC date reads void — the VECTORS URLs stay unformed");
        return false;
    };
    let mut changed = false;
    for (name, command) in bodies {
        let url = format!(
            "https://ssd.jpl.nasa.gov/api/horizons.api?format=json\
             &COMMAND=%27{cmd}%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27\
             &EPHEM_TYPE=%27VECTORS%27&CENTER=%27500@0%27\
             &START_TIME=%27{start}%27&STOP_TIME=%27{stop}%27&STEP_SIZE=%271%20d%27",
            cmd = url_quote(command),
            start = url_quote(&today),
            stop = url_quote(&tomorrow),
        );
        let (status, body) = fetch_bytes(&url);
        match body {
            Some(bytes) if status == 200 && !bytes.is_empty() => {
                changed |= save_if_changed(out_dir, &format!("horizons_{name}"), &bytes);
            }
            _ => eprintln!("  {name}: HTTP {status} or empty"),
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    changed
}

fn json_key_envelope(key: &str, raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len() + key.len() + 6);
    out.extend_from_slice(b"{\"");
    out.extend_from_slice(key.as_bytes());
    out.extend_from_slice(b"\": ");
    out.extend_from_slice(raw);
    out.push(b'}');
    out
}

fn refresh_orbits(out_dir: &str, bodies: &[(String, String)]) -> bool {
    let Some(today) = utc_date(0) else {
        eprintln!("  orbits: the UTC date reads void — the VECTORS URLs stay unformed");
        return false;
    };
    let year = &today[..4];
    let start = format!("{year}-01-01");
    let end = format!("{year}-12-31");
    let mut changed = false;
    for (name, command) in bodies {
        let url = format!(
            "https://ssd.jpl.nasa.gov/api/horizons.api?format=json\
             &COMMAND=%27{cmd}%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27\
             &EPHEM_TYPE=%27VECTORS%27&CENTER=%27500@0%27\
             &START_TIME=%27{start}%27&STOP_TIME=%27{stop}%27&STEP_SIZE=%271%20d%27",
            cmd = url_quote(command),
            start = url_quote(&start),
            stop = url_quote(&end),
        );
        let (status, body) = fetch_bytes(&url);
        match body {
            Some(bytes) if status == 200 && !bytes.is_empty() => {
                changed |= save_if_changed(out_dir, &format!("orbit_{name}"), &bytes);
            }
            _ => eprintln!("  {name}: HTTP {status} or empty"),
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    changed
}

fn refresh_observer(out_dir: &str, bodies: &[(String, String)]) -> bool {
    let Some(today) = utc_date(0) else {
        eprintln!("  observer: the UTC date reads void — the OBSERVER URLs stay unformed");
        return false;
    };
    let mut changed = false;
    for (name, command) in bodies {
        let url = format!(
            "https://ssd.jpl.nasa.gov/api/horizons.api?format=text\
             &COMMAND=%27{cmd}%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27\
             &EPHEM_TYPE=%27OBSERVER%27&CENTER=%27500@399%27\
             &START_TIME=%27{start}%27&STOP_TIME=%27{stop}%27&STEP_SIZE=%271%20d%27\
             &QUANTITIES=%271,2,3,4,20%27",
            cmd = url_quote(command),
            start = url_quote(&today),
            stop = url_quote(&today),
        );
        let (status, body) = fetch_bytes(&url);
        let size = body.as_ref().map_or(0, Vec::len);
        match body {
            Some(bytes) if status == 200 && bytes.len() > 200 => {
                let text = String::from_utf8_lossy(&bytes);
                changed |= save_if_changed(
                    out_dir,
                    &format!("observer_{name}"),
                    &json_text_envelope(&text),
                );
            }
            _ => eprintln!("  {name}: HTTP {status} or size {size}"),
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    changed
}

fn refresh_mass(out_dir: &str, bodies: &[(String, String)]) -> bool {
    let mut changed = false;
    for (name, command) in bodies {
        let url = format!(
            "https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND={command}&OBJ_DATA=YES"
        );
        let (status, body) = fetch_bytes(&url);
        match body {
            Some(bytes) if status == 200 && !bytes.is_empty() => {
                changed |= save_if_changed(out_dir, &format!("mass_{name}"), &bytes);
            }
            _ => eprintln!("  {name}: HTTP {status} or empty"),
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    changed
}

fn refresh_elements(out_dir: &str, bodies: &[(String, String)]) -> bool {
    let (Some(today), Some(tomorrow)) = (utc_date(0), utc_date(1)) else {
        eprintln!("  elements: the UTC date reads void — the ELEMENTS URLs stay unformed");
        return false;
    };
    let mut changed = false;
    for (name, command) in bodies {
        let url = format!(
            "https://ssd.jpl.nasa.gov/api/horizons.api?format=text\
             &COMMAND=%27{cmd}%27&OBJ_DATA=%27YES%27&MAKE_EPHEM=%27YES%27\
             &EPHEM_TYPE=%27ELEMENTS%27&CENTER=%27@0%27\
             &START_TIME=%27{start}%27&STOP_TIME=%27{stop}%27&STEP_SIZE=%271+d%27\
             &REF_PLANE=%27ECLIPTIC%27",
            cmd = url_quote(command),
            start = url_quote(&today),
            stop = url_quote(&tomorrow),
        );
        let (status, body) = fetch_bytes(&url);
        match body {
            Some(bytes) if status == 200 && !bytes.is_empty() => {
                let text = String::from_utf8_lossy(&bytes);
                changed |= save_if_changed(
                    out_dir,
                    &format!("elements_{name}"),
                    &json_text_envelope(&text),
                );
            }
            _ => eprintln!("  {name}: HTTP {status} or empty"),
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    changed
}

fn refresh_jpl(out_dir: &str) -> bool {
    let mut changed = false;
    let (status, body) =
        fetch_bytes("https://ssd-api.jpl.nasa.gov/fireball.api?limit=100&sort=date");
    match body {
        Some(bytes) if status == 200 && !bytes.is_empty() => {
            changed |= save_if_changed(
                out_dir,
                "jpl_fireball",
                &json_key_envelope("fireball", &bytes),
            );
        }
        _ => eprintln!("  jpl_fireball: HTTP {status} or empty"),
    }
    let (status, body) =
        fetch_bytes("https://ssd-api.jpl.nasa.gov/cad.api?dist-max=10LD&limit=200");
    match body {
        Some(bytes) if status == 200 && !bytes.is_empty() => {
            changed |= save_if_changed(out_dir, "jpl_cad", &json_key_envelope("cad", &bytes));
        }
        _ => eprintln!("  jpl_cad: HTTP {status} or empty"),
    }
    changed
}

fn refresh_swpc(out_dir: &str) -> bool {
    let mut changed = false;
    for (name, url) in SWPC_URLS {
        let (status, body) = fetch_bytes(url);
        match body {
            Some(bytes) if status == 200 && !bytes.is_empty() => {
                changed |= save_if_changed(out_dir, name, &bytes);
            }
            _ => eprintln!("  {name}: HTTP {status} or empty"),
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    changed
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_dir = "data".to_string();
    let mut spec_path = DEFAULT_SPEC.to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => match args.get(i + 1) {
                Some(p) => {
                    out_dir = p.clone();
                    i += 1;
                }
                None => {
                    eprintln!("--out needs a directory");
                    std::process::exit(2);
                }
            },
            "--spec" => match args.get(i + 1) {
                Some(p) => {
                    spec_path = p.clone();
                    i += 1;
                }
                None => {
                    eprintln!("--spec needs a file");
                    std::process::exit(2);
                }
            },
            _ => {}
        }
        i += 1;
    }
    let Some(spec) = std::fs::read_to_string(&spec_path)
        .ok()
        .and_then(|text| parse_json(&text))
    else {
        eprintln!("spec {spec_path} reads void — the body lists stay unformed");
        std::process::exit(2);
    };
    let horizons = spec_list(&spec, "horizons");
    let orbits = spec_list(&spec, "orbits");
    let observer = spec_list(&spec, "observer");
    let mass = spec_list(&spec, "mass");
    let elements = spec_list(&spec, "elements");

    let mut any = false;
    eprintln!("[horizons]");
    any |= refresh_horizons(&out_dir, &horizons);
    eprintln!("[orbits]");
    any |= refresh_orbits(&out_dir, &orbits);
    eprintln!("[observer]");
    any |= refresh_observer(&out_dir, &observer);
    eprintln!("[mass]");
    any |= refresh_mass(&out_dir, &mass);
    eprintln!("[elements]");
    any |= refresh_elements(&out_dir, &elements);
    eprintln!("[jpl]");
    any |= refresh_jpl(&out_dir);
    eprintln!("[ndbc]");
    any |= refresh_ndbc(&out_dir);
    eprintln!("[swpc]");
    any |= refresh_swpc(&out_dir);
    eprintln!("done, changed: {any}");
}
