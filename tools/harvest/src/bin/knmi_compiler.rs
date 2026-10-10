use omegaflow::archivar::JsonVal;
use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::fetch_raw_bytes_headers;
use omegaflow::archivar::http_code;
use omegaflow::archivar::jpath_val;
use omegaflow::archivar::json_num;
use omegaflow::archivar::jstr;
use omegaflow::archivar::load_env;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::parse_json;
use omegaflow::archivar::render_headers;
use omegaflow::archivar::secret_resolves_void;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::netcdf::NetcdfFile;
use std::collections::HashMap;

const NETLOC: &str = "api.dataplatform.knmi.nl";
const BASE: &str = "https://api.dataplatform.knmi.nl/open-data/v1";
const EDR_BASE: &str = "https://api.dataplatform.knmi.nl/edr/v1";
const COMPILER: &str = "tools/harvest/src/bin/knmi_compiler.rs";
const DEFAULT_DATASET: &str = "10-minute-in-situ-meteorological-observations";
const DEFAULT_VERSION: &str = "1.0";

const MAGIC: [u8; 4] = *b"KNMI";
const REC_BYTES: usize = 26 * 8;
const NFIELDS: usize = 4;

const SECS_PER_DAY: f64 = 86400.0;
const CADENCE_S: f64 = 600.0;
const TTL_S: f64 = 86400.0;
const ABSENT_FILL: f64 = f64::MIN;

const KERNEL_GAUSSIAN_INVERSE_SQUARE: u8 = 1;
const KERNEL_EXPONENTIAL_DECAY: u8 = 4;
const KERNEL_PATCH_LEVY: u8 = 5;

const FORCE_THERMAL: u8 = 5;
const FORCE_DIFFUSION: u8 = 6;
const FORCE_ADVECTIVE: u8 = 7;

const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];

struct FieldSpec {
    key: &'static str,
    var: &'static str,
    kernel: u8,
    kernel_token: &'static str,
    force: u8,
    force_token: &'static str,
    unit: &'static str,
    parse: fn(f64) -> Option<f64>,
}

const FIELDS: [FieldSpec; NFIELDS] = [
    FieldSpec {
        key: "knmi_air_temperature",
        var: "ta",
        kernel: KERNEL_EXPONENTIAL_DECAY,
        kernel_token: "exponential-decay",
        force: FORCE_THERMAL,
        force_token: "thermal",
        unit: "K",
        parse: celsius_to_kelvin,
    },
    FieldSpec {
        key: "knmi_wind_speed",
        var: "ff",
        kernel: KERNEL_PATCH_LEVY,
        kernel_token: "patch-levy",
        force: FORCE_ADVECTIVE,
        force_token: "advective",
        unit: "m/s",
        parse: wind_speed_m_s,
    },
    FieldSpec {
        key: "knmi_wind_gust",
        var: "fx",
        kernel: KERNEL_PATCH_LEVY,
        kernel_token: "patch-levy",
        force: FORCE_ADVECTIVE,
        force_token: "advective",
        unit: "m/s",
        parse: wind_speed_m_s,
    },
    FieldSpec {
        key: "knmi_precipitation_intensity",
        var: "rg",
        kernel: KERNEL_GAUSSIAN_INVERSE_SQUARE,
        kernel_token: "gaussian-inverse-square",
        force: FORCE_DIFFUSION,
        force_token: "diffusion",
        unit: "kg/(m^2 s)",
        parse: mm_per_hour_to_si,
    },
];

fn celsius_to_kelvin(raw: f64) -> Option<f64> {
    let k = raw + 273.15;
    k.is_finite()
        .then_some(k)
        .filter(|k| (150.0..=350.0).contains(k))
}

fn wind_speed_m_s(raw: f64) -> Option<f64> {
    raw.is_finite()
        .then_some(raw)
        .filter(|v| (0.0..=120.0).contains(v))
}

fn mm_per_hour_to_si(raw: f64) -> Option<f64> {
    let flux = raw / 3600.0;
    flux.is_finite()
        .then_some(flux)
        .filter(|v| (0.0..=0.1).contains(v))
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_or(args: &[String], name: &str, fallback: &str) -> String {
    match arg_value(args, name) {
        Some(v) => v,
        None => fallback.to_string(),
    }
}

fn usage() -> &'static str {
    "usage: knmi_compiler --body <name> --out <dir> [--dataset <name>] [--version <ver>] [--input <path>] [--name <slug>] [--ephemeris <path|url>] [--edr] [--edr-collection <id>] [--edr-dump <dir>] [--datetime <iso|start/end>] [--emit-field-names] [--ci-mode]"
}

fn emit_field_names() {
    for f in &FIELDS {
        println!(
            "field {} {} {} {} {} {} 0.0 0.0",
            f.key, f.key, f.kernel_token, f.force_token, f.unit, CADENCE_S as u64
        );
    }
}

fn is_hdf5(bytes: &[u8]) -> bool {
    bytes.starts_with(&HDF5_MAGIC)
}

fn safe_token(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

fn slug_of(dataset: &str, version: &str) -> String {
    format!("{dataset}_{version}")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn latest_filename(text: &str) -> Option<String> {
    let json = parse_json(text)?;
    jstr(&json, "files.0.filename")
}

fn download_url(text: &str) -> Option<String> {
    let json = parse_json(text)?;
    jstr(&json, "temporaryDownloadUrl")
}

fn epoch_seconds(units: &str) -> Option<(f64, f64)> {
    let since = units.find("since")?;
    let factor = if units.starts_with("days") {
        86400.0
    } else if units.starts_with("hours") {
        3600.0
    } else if units.starts_with("minutes") {
        60.0
    } else if units.starts_with("seconds") {
        1.0
    } else {
        return None;
    };
    let rest = units[since + 5..].trim();
    let date = rest.split(|c: char| c == 'T' || c == ' ').next()?;
    let mut it = date.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    let base = days_from_civil(y, m, d)? as f64 * SECS_PER_DAY;
    Some((factor, base))
}

fn attr_text(nc: &NetcdfFile, name: &str, key: &str) -> Option<String> {
    nc.var(name)?
        .attrs
        .iter()
        .find(|a| a.name == key)
        .and_then(|a| nc.attr_text(a))
}

fn var_fill(nc: &NetcdfFile, name: &str) -> Option<f64> {
    nc.var(name)?
        .attrs
        .iter()
        .find(|a| a.name == "_FillValue")
        .and_then(|a| nc.attr_num(a))
}

fn record(pos: [f64; 3], val: f64, tdb: f64, kernel: u8, force: u8) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = CADENCE_S;
    r[7] = 0.0;
    r[8] = kernel as f64;
    r[9] = force as f64;
    r[25] = 1.0;
    r
}

fn write_bin(records: &[[f64; 26]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn read_bin(data: &[u8]) -> Option<usize> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * REC_BYTES {
        return None;
    }
    Some(count)
}

fn compile(
    lat: &[f64],
    lon: &[f64],
    height: Option<&[f64]>,
    values: &[Option<Vec<f64>>],
    fills: &[Option<f64>],
    tdb: f64,
    eph: &HashMap<String, BodyEphemeris>,
    body: &str,
) -> (Vec<Vec<[f64; 26]>>, usize, usize) {
    let mut per_field: Vec<Vec<[f64; 26]>> = (0..NFIELDS).map(|_| Vec::new()).collect();
    let mut frame_void = 0usize;
    let mut fill_void = 0usize;
    let stations = lat.len().min(lon.len());
    for j in 0..stations {
        let Some(&latj) = lat.get(j) else {
            continue;
        };
        let Some(&lonj) = lon.get(j) else {
            continue;
        };
        let alt = match height.and_then(|h| h.get(j).copied()) {
            Some(a) => a,
            None => {
                frame_void += 1;
                continue;
            }
        };
        let Some(pos) = body_fixed_to_icrs(body, latj, lonj, alt, tdb, eph) else {
            frame_void += 1;
            continue;
        };
        for i in 0..NFIELDS {
            let Some(vals) = values.get(i).and_then(Option::as_ref) else {
                continue;
            };
            let Some(&raw) = vals.get(j) else {
                continue;
            };
            if let Some(fill) = fills.get(i).and_then(|f| *f) {
                if raw == fill {
                    fill_void += 1;
                    continue;
                }
            }
            if let Some(v) = (FIELDS[i].parse)(raw) {
                per_field[i].push(record(pos, v, tdb, FIELDS[i].kernel, FIELDS[i].force));
            }
        }
    }
    (per_field, frame_void, fill_void)
}

fn obj_field<'a>(v: &'a JsonVal, key: &str) -> Option<&'a JsonVal> {
    match v {
        JsonVal::Obj(map) => map.get(key),
        _ => None,
    }
}

fn arr_items<'a>(v: &'a JsonVal) -> Option<&'a Vec<JsonVal>> {
    match v {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn json_unit(param: &JsonVal) -> String {
    let unit = obj_field(param, "unit");
    let symbol = unit.and_then(|u| jstr(u, "symbol"));
    let label = unit.and_then(|u| jstr(u, "label"));
    match unit {
        Some(JsonVal::Str(s)) => s.clone(),
        Some(JsonVal::Num(n)) => format!("{n}"),
        _ => match symbol.or(label) {
            Some(s) => s,
            None => String::new(),
        },
    }
}

fn iso_to_unix(text: &str) -> Option<f64> {
    let s = text.trim().trim_end_matches('Z');
    let (date, clock) = s.split_once('T')?;
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    let mut t = clock.split(':');
    let h: i64 = t.next()?.parse().ok()?;
    let mi: i64 = t.next()?.parse().ok()?;
    let se: f64 = match t.next() {
        Some(x) => x.parse().ok()?,
        None => 0.0,
    };
    let base = days_from_civil(y, mo, da)? as f64 * SECS_PER_DAY;
    Some(base + h as f64 * 3600.0 + mi as f64 * 60.0 + se)
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
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn unix_to_iso(unix: f64) -> String {
    let days = (unix / SECS_PER_DAY).floor() as i64;
    let rem = unix - days as f64 * SECS_PER_DAY;
    let (y, m, d) = civil_from_days(days);
    let h = (rem / 3600.0).floor() as i64;
    let mi = ((rem - h as f64 * 3600.0) / 60.0).floor() as i64;
    let se = (rem - h as f64 * 3600.0 - mi as f64 * 60.0).floor() as i64;
    format!("{y:04}-{m:02}-{d:02}T{h:02}:{mi:02}:{se:02}Z")
}

fn default_datetime() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as f64)
        .ok();
    match now {
        Some(now) => format!("{}/{}", unix_to_iso(now - SECS_PER_DAY), unix_to_iso(now)),
        None => String::new(),
    }
}

fn param_field(name: &str, unit: &str) -> Option<usize> {
    let n = name.to_ascii_lowercase();
    let u = unit.to_ascii_lowercase();
    let kelvin = u.contains("kelvin") || u == "k";
    let celsius = u.contains("celsius") || u.contains("degc");
    let speed = u.contains("meter per second") || u.contains("m s-1") || u.contains("m/s");
    let rain = u.contains("millimeter per hour") || u.contains("mm/h");
    if n == "ta" && (kelvin || celsius) {
        Some(0)
    } else if n == "ff" && speed {
        Some(1)
    } else if n == "fx" && speed {
        Some(2)
    } else if n == "rg" && rain {
        Some(3)
    } else if n.contains("temp") && (kelvin || celsius) {
        Some(0)
    } else if n.contains("gust") && speed {
        Some(2)
    } else if n.contains("wind") && speed {
        Some(1)
    } else if (n.contains("precip") || n.contains("regen") || n.contains("neerslag")) && rain {
        Some(3)
    } else {
        None
    }
}

fn convert_raw(field: usize, unit: &str, raw: f64) -> Option<f64> {
    if !raw.is_finite() {
        return None;
    }
    let u = unit.to_ascii_lowercase();
    match field {
        0 => {
            if u.contains("celsius") || u.contains("degc") || u == "c" || u == "°c" {
                Some(raw)
            } else if u.contains("kelvin") || u == "k" {
                Some(raw - 273.15)
            } else {
                None
            }
        }
        1 | 2 => {
            if u.contains("km") {
                Some(raw / 3.6)
            } else if u.contains("knot") || u == "kt" {
                Some(raw * 0.514444)
            } else if u.contains("meter per second") || u.contains("m/s") || u.contains("m s-1") {
                Some(raw)
            } else {
                None
            }
        }
        3 => {
            if u.contains("millimeter per hour") || u.contains("mm/h") {
                Some(raw)
            } else if u.contains("kg") {
                Some(raw * 3600.0)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn edr_headers(env: &HashMap<String, String>) -> Vec<(String, String)> {
    render_headers(
        &[
            ("Authorization".to_string(), "{KNMI_API_KEY}".to_string()),
            (
                "Accept".to_string(),
                "application/prs.coverage+json".to_string(),
            ),
        ],
        env,
    )
}

fn fetch_edr(url: &str, headers: &[(String, String)]) -> Result<Vec<u8>, String> {
    match fetch_raw_bytes_headers(url, headers) {
        Some(b) => Ok(b),
        None => match http_code(url, headers) {
            Some(c) => Err(format!("{url}: measured HTTP {c}")),
            None => Err(format!(
                "{url}: fetch returned void — no HTTP status measured"
            )),
        },
    }
}

fn dump_edr(dir: &Option<String>, name: &str, bytes: &[u8]) {
    if let Some(d) = dir {
        let _ = std::fs::create_dir_all(d);
        let _ = std::fs::write(format!("{d}/{name}"), bytes);
    }
}

fn edr_collection_ids(text: &str) -> Vec<String> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let Some(items) = jpath_val(&json, "collections").and_then(arr_items) else {
        return Vec::new();
    };
    items.iter().filter_map(|c| jstr(c, "id")).collect()
}

fn edr_parameters(meta: &JsonVal) -> Vec<(String, String)> {
    match jpath_val(meta, "parameter_names").and_then(arr_items) {
        Some(items) => items
            .iter()
            .filter_map(|p| {
                let name = jstr(p, "id").or_else(|| jstr(p, "name"))?;
                let unit = json_unit(p);
                Some((name, unit))
            })
            .collect(),
        None => match jpath_val(meta, "parameter_names") {
            Some(JsonVal::Obj(map)) => map.iter().map(|(k, v)| (k.clone(), json_unit(v))).collect(),
            _ => Vec::new(),
        },
    }
}

fn location_points(text: &str) -> Vec<(String, f64, f64, Option<f64>)> {
    let Some(json) = parse_json(text) else {
        return Vec::new();
    };
    let Some(features) = jpath_val(&json, "features").and_then(arr_items) else {
        return Vec::new();
    };
    features
        .iter()
        .filter_map(|f| {
            let id = jstr(f, "id")
                .or_else(|| jstr(f, "properties.station_id"))
                .or_else(|| jstr(f, "properties.name"))?;
            let coords = jpath_val(f, "geometry.coordinates").and_then(arr_items)?;
            let lon = coords.first().and_then(json_num)?;
            let lat = coords.get(1).and_then(json_num)?;
            let height = coords
                .get(2)
                .and_then(json_num)
                .or_else(|| {
                    jpath_val(f, "properties.height_above_mean_sea_level").and_then(json_num)
                })
                .or_else(|| jpath_val(f, "properties.height").and_then(json_num))
                .or_else(|| jpath_val(f, "properties.elevation").and_then(json_num));
            Some((id, lon, lat, height))
        })
        .collect()
}

fn time_axis_unix(coverage: &JsonVal) -> Vec<f64> {
    let Some(axis_t) = jpath_val(coverage, "domain.axes.t") else {
        return Vec::new();
    };
    if let Some(vals) = obj_field(axis_t, "values").and_then(arr_items) {
        let direct: Vec<f64> = vals.iter().filter_map(json_num).collect();
        if direct.len() == vals.len() {
            if let Some(tmp) = temporal_coordinates(coverage) {
                if direct
                    .iter()
                    .all(|i| i.fract() == 0.0 && *i >= 0.0 && (*i as usize) < tmp.len())
                {
                    return direct
                        .iter()
                        .filter_map(|i| tmp.get(*i as usize).copied())
                        .collect();
                }
            }
            return direct;
        }
        let mapped: Vec<f64> = vals
            .iter()
            .filter_map(|v| match v {
                JsonVal::Str(s) => iso_to_unix(s),
                _ => None,
            })
            .collect();
        if mapped.len() == vals.len() {
            return mapped;
        }
    }
    let start = obj_field(axis_t, "start").and_then(json_num);
    let stop = obj_field(axis_t, "stop").and_then(json_num);
    let num = obj_field(axis_t, "num").and_then(json_num);
    match (start, stop, num) {
        (Some(a), Some(b), Some(n)) if n >= 1.0 => {
            let steps = n.max(1.0) as usize;
            if steps == 1 {
                vec![a]
            } else {
                (0..steps)
                    .map(|i| a + (b - a) * i as f64 / (steps - 1) as f64)
                    .collect()
            }
        }
        _ => Vec::new(),
    }
}

fn temporal_coordinates(coverage: &JsonVal) -> Option<Vec<f64>> {
    let refs = jpath_val(coverage, "domain.referencing").and_then(arr_items)?;
    for r in refs {
        let temporal_marker = match jstr(r, "coordinatesType") {
            Some(t) => t.contains("temporal"),
            None => false,
        };
        let temporal_system = match jstr(r, "system.type") {
            Some(t) => t.contains("Temporal"),
            None => false,
        };
        if !temporal_marker && !temporal_system {
            continue;
        }
        let coords = obj_field(r, "coordinates").and_then(arr_items)?;
        let unix: Vec<f64> = coords
            .iter()
            .filter_map(|c| match c {
                JsonVal::Str(s) => iso_to_unix(s),
                _ => None,
            })
            .collect();
        if unix.len() == coords.len() && !unix.is_empty() {
            return Some(unix);
        }
    }
    None
}

fn range_axis_names(range: &JsonVal) -> Vec<String> {
    match obj_field(range, "axisNames").and_then(arr_items) {
        Some(a) => a
            .iter()
            .filter_map(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        None => Vec::new(),
    }
}

fn range_values(range: &JsonVal) -> Vec<Option<f64>> {
    match obj_field(range, "values").and_then(arr_items) {
        Some(a) => a
            .iter()
            .map(|v| match v {
                JsonVal::Num(n) if n.is_finite() => Some(*n),
                JsonVal::Str(s) => s.parse::<f64>().ok().filter(|x| x.is_finite()),
                _ => None,
            })
            .collect(),
        None => Vec::new(),
    }
}

fn write_assets(
    out_dir: &str,
    slug: &str,
    body: &str,
    origin: Option<&str>,
    per_field: &[Vec<[f64; 26]>],
    ci_mode: bool,
) -> Result<(), String> {
    std::fs::create_dir_all(out_dir).map_err(|e| format!("create {out_dir} returned void: {e}"))?;
    for (i, recs) in per_field.iter().enumerate() {
        if recs.is_empty() {
            eprintln!(
                "knmi: {} carries no measured station — skipped",
                FIELDS[i].key
            );
            continue;
        }
        let name = format!("knmi_{slug}_{}.bin", FIELDS[i].key);
        let path = format!("{out_dir}/{name}");
        let bin = write_bin(recs);
        std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned void: {e}"))?;
        let roundtrip = read_bin(&bin)
            .ok_or_else(|| format!("{path}: roundtrip parse void — the asset stays unverified"))?;
        eprintln!(
            "knmi: {} records, {} B -> {path} (roundtrip {roundtrip})",
            recs.len(),
            bin.len()
        );
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format knmi");
        if let Some(src) = origin {
            println!("origin {src}");
        }
        println!("compiler {COMPILER}");
        println!("at {body}");
        println!("ttl {}", TTL_S as u64);
        println!(
            "field {} {} {} {} {} {} 0.0 0.0",
            FIELDS[i].key,
            FIELDS[i].key,
            FIELDS[i].kernel_token,
            FIELDS[i].force_token,
            FIELDS[i].unit,
            CADENCE_S as u64
        );
        println!("sha256 {}", sha256_hex(&bin));
        if ci_mode && !upload_release(NETLOC, &path) {
            return Err(format!("{path}: CDN upload returned void"));
        }
    }
    Ok(())
}

fn run_edr(
    args: &[String],
    env: &HashMap<String, String>,
    body: &str,
    out_dir: &str,
    ci_mode: bool,
    dataset: &str,
) -> Result<(), String> {
    let headers = edr_headers(env);
    let dump_dir = arg_value(args, "--edr-dump");
    let collections_url = format!("{EDR_BASE}/collections");
    let cols_bytes = match fetch_edr(&collections_url, &headers) {
        Ok(b) => b,
        Err(reason) => {
            println!(
                "pending — the EDR API list is unread ({reason}); the Open Data product is NetCDF-4/HDF5, the EDR CoverageJSON path stays the next bounded step"
            );
            return Ok(());
        }
    };
    dump_edr(&dump_dir, "collections.json", &cols_bytes);
    let ids = edr_collection_ids(&String::from_utf8_lossy(&cols_bytes));
    if ids.is_empty() {
        return Err(format!(
            "{collections_url}: measured no collection id — the EDR catalog stays unread"
        ));
    }
    eprintln!("knmi: EDR collections measured: {}", ids.join(", "));
    let collection = arg_or(args, "--edr-collection", dataset);
    if !ids.iter().any(|i| i == &collection) {
        println!(
            "pending — EDR collection '{collection}' absent from the catalog; measured: {}",
            ids.join(", ")
        );
        return Ok(());
    }

    let meta_url = format!("{EDR_BASE}/collections/{collection}");
    let meta_bytes = fetch_edr(&meta_url, &headers)?;
    dump_edr(&dump_dir, "collection.json", &meta_bytes);
    let meta = parse_json(&String::from_utf8_lossy(&meta_bytes))
        .ok_or_else(|| format!("{meta_url}: the collection metadata stays unread"))?;
    let params = edr_parameters(&meta);
    if params.is_empty() {
        return Err(format!(
            "{meta_url}: the collection metadata carries no parameter_names — the parameter binding stays open"
        ));
    }
    for (name, unit) in &params {
        eprintln!("knmi: EDR parameter '{name}' unit '{unit}'");
    }

    let loc_url = format!("{EDR_BASE}/collections/{collection}/locations");
    let loc_bytes = fetch_edr(&loc_url, &headers)?;
    dump_edr(&dump_dir, "locations.json", &loc_bytes);
    let points = location_points(&String::from_utf8_lossy(&loc_bytes));
    if points.is_empty() {
        return Err(format!(
            "{loc_url}: measured no location geometry — the station binding stays open"
        ));
    }
    eprintln!("knmi: EDR locations measured: {}", points.len());
    if dump_dir.is_some() {
        if let Some(json) = parse_json(&String::from_utf8_lossy(&loc_bytes)) {
            if let Some(first) = jpath_val(&json, "features.0") {
                eprintln!("knmi: first location feature: {first:?}");
            }
        }
    }

    let binding: Vec<(usize, String, String)> = params
        .iter()
        .filter_map(|(name, unit)| param_field(name, unit).map(|f| (f, name.clone(), unit.clone())))
        .collect();
    if binding.is_empty() {
        println!(
            "pending — no EDR parameter matched a known SI field; measured parameters: {}",
            params
                .iter()
                .map(|(n, u)| format!("{n} ({u})"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Ok(());
    }
    eprintln!(
        "knmi: EDR binding: {}",
        binding
            .iter()
            .map(|(f, n, _)| format!("{n} -> {}", FIELDS[*f].key))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let datetime = match arg_value(args, "--datetime") {
        Some(d) => d,
        None => default_datetime(),
    };
    let stations: Vec<(String, f64, f64, f64)> = points
        .iter()
        .filter_map(|(id, lon, lat, height)| height.map(|h| (id.clone(), *lon, *lat, h)))
        .collect();
    if stations.is_empty() {
        if let (Some(dir), Some((id, _, _, _))) = (&dump_dir, points.first()) {
            let url = format!("{EDR_BASE}/collections/{collection}/locations/{id}");
            if let Ok(b) = fetch_edr(&url, &headers) {
                dump_edr(&dump_dir, &format!("coverage-{id}.json"), &b);
                if let Some(cov) = parse_json(&String::from_utf8_lossy(&b)) {
                    eprintln!(
                        "knmi: sample coverage domain: {:?}",
                        jpath_val(&cov, "domain")
                    );
                    eprintln!(
                        "knmi: sample coverage range keys: {:?}",
                        jpath_val(&cov, "ranges").and_then(arr_items).map(Vec::len)
                    );
                }
            }
            eprintln!("knmi: dumped EDR responses under {dir}");
        }
        println!(
            "pending — no EDR location carries a measured station height; the ICRS frame is never fabricated at altitude 0.0 (measured {} locations without height)",
            points.len()
        );
        return Ok(());
    }

    let mut lats: Vec<f64> = Vec::with_capacity(stations.len());
    let mut lons: Vec<f64> = Vec::with_capacity(stations.len());
    let mut heights: Vec<f64> = Vec::with_capacity(stations.len());
    let mut field_values: Vec<Vec<Option<f64>>> = (0..NFIELDS).map(|_| Vec::new()).collect();
    let mut unix_sum = 0.0f64;
    let mut unix_count = 0usize;

    let param_names = binding
        .iter()
        .map(|(_, name, _)| name.as_str())
        .collect::<Vec<_>>()
        .join(",");
    for (id, lon, lat, height) in &stations {
        let url = format!(
            "{EDR_BASE}/collections/{collection}/position?coords=POINT({lon}%20{lat})&datetime={datetime}&parameter-name={param_names}"
        );
        lats.push(*lat);
        lons.push(*lon);
        heights.push(*height);
        for v in field_values.iter_mut() {
            v.push(None);
        }
        let slot = lats.len() - 1;
        let cov_bytes = match fetch_edr(&url, &headers) {
            Ok(b) => b,
            Err(reason) => {
                eprintln!("knmi: location '{id}' unread ({reason}) — skipped");
                continue;
            }
        };
        if slot == 0 {
            dump_edr(&dump_dir, &format!("coverage-{id}.json"), &cov_bytes);
        }
        let Some(cov) = parse_json(&String::from_utf8_lossy(&cov_bytes)) else {
            eprintln!("knmi: location '{id}' carries no parseable CoverageJSON — skipped");
            continue;
        };
        let root = match jpath_val(&cov, "coverages.0") {
            Some(c) => c,
            None => &cov,
        };
        let times = time_axis_unix(root);
        let last_t = times.len().saturating_sub(1);
        if let Some(t) = times.last() {
            unix_sum += *t;
            unix_count += 1;
        }
        for (field, name, unit) in &binding {
            let Some(range) = jpath_val(root, "ranges").and_then(|r| obj_field(r, name)) else {
                continue;
            };
            let vals = range_values(range);
            if vals.is_empty() {
                continue;
            }
            let idx = if range_axis_names(range).iter().any(|a| a == "t") {
                if times.is_empty() {
                    continue;
                }
                last_t
            } else {
                0
            };
            if let Some(raw) = vals.get(idx).copied().flatten() {
                if let Some(converted) = convert_raw(*field, unit, raw) {
                    field_values[*field][slot] = Some(converted);
                }
            }
        }
    }

    if unix_count == 0 {
        return Err(format!(
            "{collection}: measured no temporal axis — the observation epoch stays open"
        ));
    }
    let unix = unix_sum / unix_count as f64;
    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no date→TDB step".to_string())?;
    let tdb = lsk
        .unix_to_tdb(unix)
        .ok_or_else(|| format!("unix {unix} lies outside the leap table — no TDB step"))?;

    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(src) if src.starts_with("http") => {
            fetch_raw_bytes(&src).ok_or_else(|| format!("body ephemeris fetch void ({src})"))?
        }
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("body ephemeris read {path}: {e}"))?
        }
        None => fetch_raw_bytes(&body_url(body))
            .ok_or_else(|| "body ephemeris fetch void (CDN) — no ICRS frame".to_string())?,
    };
    let body_eph = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the body ephemeris binary stays unread — no ICRS frame".to_string())?;
    let eph = HashMap::from([(body.to_string(), body_eph)]);

    let values: Vec<Option<Vec<f64>>> = field_values
        .iter()
        .map(|field| {
            let aligned: Vec<f64> = field
                .iter()
                .map(|o| match o {
                    Some(v) => *v,
                    None => ABSENT_FILL,
                })
                .collect();
            Some(aligned)
        })
        .collect();
    let fills: Vec<Option<f64>> = (0..NFIELDS).map(|_| Some(ABSENT_FILL)).collect();
    let (per_field, frame_void, fill_void) = compile(
        &lats,
        &lons,
        Some(&heights),
        &values,
        &fills,
        tdb,
        &eph,
        body,
    );
    let total: usize = per_field.iter().map(Vec::len).sum();
    if total == 0 {
        return Err(format!(
            "no record left the EDR harvest — {} locations read; frame_void {}, fill_void {}",
            lats.len(),
            frame_void,
            fill_void
        ));
    }
    let slug = arg_or(args, "--name", &slug_of(dataset, DEFAULT_VERSION));
    let origin = format!("{EDR_BASE}/collections/{collection}/position");
    eprintln!(
        "knmi: EDR {slug} ({} locations, tdb {tdb}); frame_void {}, fill_void {}",
        lats.len(),
        frame_void,
        fill_void
    );
    write_assets(out_dir, &slug, body, Some(&origin), &per_field, ci_mode)
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--emit-field-names") {
        emit_field_names();
        return Ok(());
    }
    if args.is_empty() || args.iter().any(|a| a == "--help") {
        println!("{}", usage());
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let force_edr = args.iter().any(|a| a == "--edr");
    let dataset = arg_or(args, "--dataset", DEFAULT_DATASET);
    let version = arg_or(args, "--version", DEFAULT_VERSION);
    let input = arg_value(args, "--input");

    let env = load_env();
    if input.is_none() && secret_resolves_void("{KNMI_API_KEY}", &env) {
        println!("pending — KNMI_API_KEY absent");
        return Ok(());
    }

    let body = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;
    let out_dir = arg_value(args, "--out").ok_or_else(|| {
        "--out <dir> is required — the asset is never written to a guessed path".to_string()
    })?;

    if force_edr {
        return run_edr(args, &env, &body, &out_dir, ci_mode, &dataset);
    }

    let (bytes, origin) = if let Some(path) = input {
        let raw = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        (raw, None)
    } else {
        if !safe_token(&dataset) || !safe_token(&version) {
            return Err(format!(
                "dataset '{dataset}' or version '{version}' carries a token outside the API path alphabet — refused"
            ));
        }
        let headers = render_headers(
            &[("Authorization".to_string(), "{KNMI_API_KEY}".to_string())],
            &env,
        );
        let list_url = format!(
            "{BASE}/datasets/{dataset}/versions/{version}/files?maxKeys=1&orderBy=created&sorting=desc"
        );
        let list_body = match fetch_raw_bytes_headers(&list_url, &headers) {
            Some(b) => b,
            None => {
                eprintln!(
                    "knmi: the Open Data list returned void — routing to the EDR CoverageJSON path"
                );
                return run_edr(args, &env, &body, &out_dir, ci_mode, &dataset);
            }
        };
        let filename = latest_filename(&String::from_utf8_lossy(&list_body)).ok_or_else(|| {
            format!(
                "{list_url}: the response carries no files[0].filename — the dataset stays unread"
            )
        })?;
        let url_url = format!("{BASE}/datasets/{dataset}/versions/{version}/files/{filename}/url");
        let url_body = fetch_raw_bytes_headers(&url_url, &headers).ok_or_else(|| {
            format!("{url_url}: fetch returned void — the download URL stays unread")
        })?;
        let download = download_url(&String::from_utf8_lossy(&url_body)).ok_or_else(|| {
            format!(
                "{url_url}: the response carries no temporaryDownloadUrl — the file stays unread"
            )
        })?;
        let raw = fetch_raw_bytes(&download).ok_or_else(|| {
            format!("{filename}: fetch returned void — the observation stays unread (0 honored)")
        })?;
        (
            raw,
            Some(format!(
                "{BASE}/datasets/{dataset}/versions/{version}/files"
            )),
        )
    };

    if is_hdf5(&bytes) {
        eprintln!("knmi: the KDP product is NetCDF-4/HDF5 — routing to the EDR CoverageJSON path");
        return run_edr(args, &env, &body, &out_dir, ci_mode, &dataset);
    }
    let nc = NetcdfFile::parse(&bytes)
        .map_err(|note| format!("the NetCDF container stays unread: {note:?}"))?;

    let time = nc
        .values_numeric(&bytes, "time")
        .ok_or_else(|| "the time variable stays unread — no observation epoch".to_string())?;
    let t_raw = *time
        .first()
        .ok_or_else(|| "the time variable carries no value — no observation epoch".to_string())?;
    let units = attr_text(&nc, "time", "units")
        .ok_or_else(|| "the time variable carries no units content".to_string())?;
    let (factor, base) =
        epoch_seconds(&units).ok_or_else(|| format!("the time units '{units}' carry no epoch"))?;
    let unix = base + t_raw * factor;
    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no date→TDB step".to_string())?;
    let tdb = lsk
        .unix_to_tdb(unix)
        .ok_or_else(|| format!("unix {unix} lies outside the leap table — no TDB step"))?;

    let lat = nc
        .values_numeric(&bytes, "lat")
        .ok_or_else(|| "the lat variable stays unread — no station position".to_string())?;
    let lon = nc
        .values_numeric(&bytes, "lon")
        .ok_or_else(|| "the lon variable stays unread — no station position".to_string())?;
    let height = nc.values_numeric(&bytes, "height");

    let values: Vec<Option<Vec<f64>>> = FIELDS
        .iter()
        .map(|f| nc.values_numeric(&bytes, f.var))
        .collect();
    let fills: Vec<Option<f64>> = FIELDS.iter().map(|f| var_fill(&nc, f.var)).collect();

    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(src) if src.starts_with("http") => {
            fetch_raw_bytes(&src).ok_or_else(|| format!("body ephemeris fetch void ({src})"))?
        }
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("body ephemeris read {path}: {e}"))?
        }
        None => fetch_raw_bytes(&body_url(&body))
            .ok_or_else(|| "body ephemeris fetch void (CDN) — no ICRS frame".to_string())?,
    };
    let body_eph = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the body ephemeris binary stays unread — no ICRS frame".to_string())?;
    let eph = HashMap::from([(body.clone(), body_eph)]);

    let (per_field, frame_void, fill_void) = compile(
        &lat,
        &lon,
        height.as_deref(),
        &values,
        &fills,
        tdb,
        &eph,
        &body,
    );
    let total: usize = per_field.iter().map(Vec::len).sum();
    if total == 0 {
        return Err(format!(
            "no record left the harvest — {} stations read; frame_void {}, fill_void {}",
            lat.len(),
            frame_void,
            fill_void
        ));
    }

    let slug = arg_or(args, "--name", &slug_of(&dataset, &version));
    eprintln!(
        "knmi: {slug} ({} stations, tdb {tdb}); frame_void {}, fill_void {}",
        lat.len(),
        frame_void,
        fill_void
    );
    write_assets(
        &out_dir,
        &slug,
        &body,
        origin.as_deref(),
        &per_field,
        ci_mode,
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("knmi_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_filename_reads_the_first_file() {
        let text =
            r#"{"files":[{"filename":"KMDS__OPER_P___10M_OBS_L2_202401010000.nc","size":1}]}"#;
        assert_eq!(
            latest_filename(text).as_deref(),
            Some("KMDS__OPER_P___10M_OBS_L2_202401010000.nc")
        );
        assert_eq!(latest_filename("{}"), None);
    }

    #[test]
    fn download_url_reads_the_temporary_link() {
        let text = r#"{"temporaryDownloadUrl":"https://example.invalid/x.nc","filename":"x.nc"}"#;
        assert_eq!(
            download_url(text).as_deref(),
            Some("https://example.invalid/x.nc")
        );
        assert_eq!(download_url("{}"), None);
    }

    #[test]
    fn epoch_seconds_reads_the_1950_base() {
        let (factor, base) = epoch_seconds("seconds since 1950-01-01 00:00:00").expect("units");
        assert_eq!(factor, 1.0);
        assert_eq!(
            base,
            days_from_civil(1950, 1, 1).expect("1950") as f64 * SECS_PER_DAY
        );
    }

    #[test]
    fn conversions_honor_zero_and_drop_absurd() {
        assert_eq!(celsius_to_kelvin(0.0), Some(273.15));
        assert_eq!(celsius_to_kelvin(-9999.0), None);
        assert_eq!(wind_speed_m_s(0.0), Some(0.0));
        assert_eq!(wind_speed_m_s(500.0), None);
        assert_eq!(mm_per_hour_to_si(0.0), Some(0.0));
        assert_eq!(mm_per_hour_to_si(3600.0), Some(1.0));
        assert_eq!(mm_per_hour_to_si(-1.0), None);
    }

    #[test]
    fn iso_roundtrips_through_unix() {
        let u = iso_to_unix("2024-01-01T00:00:00Z").expect("iso");
        assert_eq!(
            u,
            days_from_civil(2024, 1, 1).expect("2024") as f64 * SECS_PER_DAY
        );
        assert_eq!(iso_to_unix("no-date"), None);
    }

    #[test]
    fn convert_raw_reaches_the_classic_source_unit() {
        assert_eq!(convert_raw(0, "degrees celsius", 10.0), Some(10.0));
        assert_eq!(convert_raw(0, "K", 283.15), Some(10.0));
        assert_eq!(convert_raw(1, "meter per second", 5.0), Some(5.0));
        assert_eq!(convert_raw(1, "km/h", 36.0), Some(10.0));
        assert_eq!(convert_raw(3, "millimeter per hour", 3.6), Some(3.6));
        assert_eq!(convert_raw(3, "kg m-2 s-1", 0.001), Some(3.6));
        assert_eq!(convert_raw(0, "foot", 1.0), None);
    }

    #[test]
    fn edr_pipeline_reaches_si_once() {
        let celsius = convert_raw(0, "degrees celsius", 10.0).expect("celsius");
        assert_eq!(celsius_to_kelvin(celsius), Some(283.15));
        let mm_h = convert_raw(3, "millimeter per hour", 3.6).expect("mm/h");
        assert_eq!(mm_per_hour_to_si(mm_h), Some(0.001));
    }

    #[test]
    fn param_field_binds_names_not_guesses() {
        assert_eq!(param_field("ta", "degrees celsius"), Some(0));
        assert_eq!(param_field("fx", "meter per second"), Some(2));
        assert_eq!(param_field("ff", "meter per second"), Some(1));
        assert_eq!(param_field("rg", "millimeter per hour"), Some(3));
        assert_eq!(param_field("rh", "percent"), None);
        assert_eq!(param_field("pg", "millimeter per hour"), None);
        assert_eq!(param_field("hc", "foot"), None);
        assert_eq!(param_field("relative_humidity", "percent"), None);
    }

    #[test]
    fn edr_collection_ids_read_the_catalog() {
        let text = r#"{"collections":[{"id":"a"},{"id":"b"}]}"#;
        assert_eq!(
            edr_collection_ids(text),
            vec!["a".to_string(), "b".to_string()]
        );
        assert!(edr_collection_ids("{}").is_empty());
    }

    #[test]
    fn location_points_read_the_geojson() {
        let text = r#"{"features":[{"id":"06215","geometry":{"coordinates":[4.92,52.35,-0.2]}}]}"#;
        assert_eq!(
            location_points(text),
            vec![("06215".to_string(), 4.92, 52.35, Some(-0.2))]
        );
        let no_height = r#"{"features":[{"id":"x","geometry":{"coordinates":[1.0,2.0]}}]}"#;
        assert_eq!(
            location_points(no_height),
            vec![("x".to_string(), 1.0, 2.0, None)]
        );
    }

    #[test]
    fn time_axis_reads_referencing_strings() {
        let text = r#"{"domain":{"axes":{"t":{"values":[0,1]}},"referencing":[{"coordinatesType":"temporal","coordinates":["2024-01-01T00:00:00Z","2024-01-01T00:10:00Z"]}]}}"#;
        let json = parse_json(text).expect("json");
        let times = time_axis_unix(&json);
        assert_eq!(times.len(), 2);
        assert!(times[1] > times[0]);
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record(
            [1.0, 2.0, 3.0],
            0.5,
            8.0e8,
            KERNEL_EXPONENTIAL_DECAY,
            FORCE_THERMAL,
        );
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 2.0);
        assert_eq!(r[2], 3.0);
        assert_eq!(r[3], 0.5);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], CADENCE_S);
        assert_eq!(r[8], KERNEL_EXPONENTIAL_DECAY as f64);
        assert_eq!(r[9], FORCE_THERMAL as f64);
        assert_eq!(r[25], 1.0);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
    }

    #[test]
    fn bin_roundtrip_counts_records() {
        let records = vec![
            record(
                [1.0, 2.0, 3.0],
                0.4,
                8.0e8,
                KERNEL_PATCH_LEVY,
                FORCE_ADVECTIVE,
            ),
            record(
                [4.0, 5.0, 6.0],
                0.7,
                8.0e8 + 1.0,
                KERNEL_PATCH_LEVY,
                FORCE_ADVECTIVE,
            ),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(read_bin(&bytes), Some(2));
        assert!(read_bin(b"X").is_none());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
