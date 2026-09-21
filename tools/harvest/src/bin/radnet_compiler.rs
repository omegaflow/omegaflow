use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::LeapSeconds;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::json::{JsonVal, parse_json};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::process::Command;

const EF_ROOT: &str = "https://data.epa.gov/efservice";
const AGOL_URL: &str = "https://services.arcgis.com/XG15cJAlne2vxtgt/arcgis/rest/services/EPA_Radiation_Air_Monitors/FeatureServer/0/query?where=1%3D1&outFields=name,city,state,State_Abbr,type,url&returnGeometry=true&outSR=4326&f=json";
const CDN_TAG: &str = "data.epa.gov";
const PAGE_SIZE: usize = 1000;
const MAX_PAGES: usize = 4096;
const RISS_LINE_CAP: usize = 20;
const MAGIC: [u8; 4] = *b"RDNT";
const REC_BYTES: usize = 26 * 8;
const FORCE_EM: f64 = 0.0;
const KERNEL_INVERSE_SQUARE: f64 = 0.0;
const TTL_S: f64 = 604800.0;
const TAU_S: f64 = 86400.0;
const SECS_PER_DAY: f64 = 86400.0;
const EARTH: &str = "earth";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_body(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "radnet fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn fetch_table(root: &str, table: &str, page_size: usize) -> Result<(Vec<JsonVal>, usize), String> {
    let mut rows = Vec::new();
    let mut page_void = 0usize;
    let mut start = 0usize;
    loop {
        if start / page_size >= MAX_PAGES {
            page_void += 1;
            break;
        }
        let url = format!("{root}/{table}/rows/{start}:{}/json", start + page_size - 1);
        let Some(body) = curl_body(&url) else {
            if rows.is_empty() {
                return Err(format!("{table} page {start} stays unread ({url})"));
            }
            page_void += 1;
            break;
        };
        let Some(parsed) = parse_json(&body) else {
            if rows.is_empty() {
                return Err(format!("{table} page {start} carries no json ({url})"));
            }
            page_void += 1;
            break;
        };
        let JsonVal::Arr(page) = parsed else {
            if rows.is_empty() {
                return Err(format!("{table} page {start} is not an array ({url})"));
            }
            page_void += 1;
            break;
        };
        if page.is_empty() {
            break;
        }
        let gained = page.len();
        rows.extend(page);
        if gained < page_size {
            break;
        }
        start += page_size;
    }
    Ok((rows, page_void))
}

fn row_obj(r: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match r {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

fn map_str(m: &HashMap<String, JsonVal>, key: &str) -> Option<String> {
    match m.get(key)? {
        JsonVal::Str(s) => Some(s.clone()),
        JsonVal::Num(n) if n.is_finite() => Some(format!("{n}")),
        _ => None,
    }
}

fn map_f64(m: &HashMap<String, JsonVal>, key: &str) -> Option<f64> {
    match m.get(key)? {
        JsonVal::Num(n) => Some(*n),
        _ => None,
    }
}

fn map_key(m: &HashMap<String, JsonVal>, key: &str) -> Option<String> {
    match m.get(key)? {
        JsonVal::Str(s) => {
            let t = s.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        }
        JsonVal::Num(n) if n.is_finite() => Some(format!("{n}")),
        _ => None,
    }
}

struct LocationRow {
    state_abbr: Option<String>,
    city_name: Option<String>,
    station: Option<String>,
}

struct SampleRow {
    loc_num: Option<String>,
}

struct AnalysisRow {
    samp_num: Option<String>,
}

struct ResultRow {
    ana_num: String,
    result_in_si: Option<f64>,
    result_date: Option<String>,
}

struct AgolFeature {
    name: String,
    city: String,
    state_abbr: String,
    lon: f64,
    lat: f64,
}

struct Layer {
    features: Vec<AgolFeature>,
    by_name: HashMap<String, Vec<usize>>,
    by_pair: HashMap<String, Vec<usize>>,
}

fn normalize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut gap = false;
    for c in s.trim().chars() {
        if c.is_alphanumeric() {
            out.extend(c.to_lowercase());
            gap = false;
        } else if !gap && !out.is_empty() {
            out.push(' ');
            gap = true;
        }
    }
    while out.ends_with(' ') {
        out.pop();
    }
    out
}

fn norm_of(s: &Option<String>) -> String {
    match s {
        Some(v) => normalize(v),
        None => String::new(),
    }
}

fn raw(s: &Option<String>) -> &str {
    match s {
        Some(v) => v.as_str(),
        None => "",
    }
}

fn layer_from(features: Vec<AgolFeature>) -> Layer {
    let mut by_name: HashMap<String, Vec<usize>> = HashMap::new();
    let mut by_pair: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, f) in features.iter().enumerate() {
        by_name
            .entry(normalize(&f.name))
            .or_insert_with(Vec::new)
            .push(i);
        let pair = format!("{} {}", normalize(&f.state_abbr), normalize(&f.city));
        by_pair.entry(pair).or_insert_with(Vec::new).push(i);
    }
    Layer {
        features,
        by_name,
        by_pair,
    }
}

#[derive(Clone, Debug)]
enum Join {
    Coord(usize),
    RissStation(Vec<String>),
    RissWitness {
        station_name: String,
        pair_name: String,
    },
    Unjoined,
}

fn resolve_join(loc: &LocationRow, layer: &Layer) -> Join {
    let station_key = norm_of(&loc.station);
    let pair_key = format!(
        "{} {}",
        norm_of(&loc.state_abbr),
        norm_of(&loc.city_name)
    );
    let station_matches = if station_key.is_empty() {
        None
    } else {
        layer.by_name.get(&station_key)
    };
    let pair_matches = if pair_key.trim().is_empty() {
        None
    } else {
        layer.by_pair.get(&pair_key)
    };
    let pair_unique = match pair_matches {
        Some(v) if v.len() == 1 => Some(v[0]),
        _ => None,
    };
    match station_matches {
        Some(v) if v.len() == 1 => {
            let one = v[0];
            match pair_unique {
                Some(p) if p != one => Join::RissWitness {
                    station_name: layer.features[one].name.clone(),
                    pair_name: layer.features[p].name.clone(),
                },
                _ => Join::Coord(one),
            }
        }
        Some(v) => Join::RissStation(v.iter().map(|&i| layer.features[i].name.clone()).collect()),
        None => match pair_unique {
            Some(p) => Join::Coord(p),
            None => Join::Unjoined,
        },
    }
}

fn value_gate(v: Option<f64>) -> Option<f64> {
    match v {
        Some(v) if v.is_finite() && v > 0.0 => Some(v),
        _ => None,
    }
}

fn days_in_month(y: i64, m: i64) -> Option<i64> {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
            29
        } else {
            28
        }),
        _ => None,
    }
}

fn parse_result_date(s: Option<&str>) -> Option<(i64, i64, i64)> {
    let s = s?;
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    if !b[..4].iter().all(u8::is_ascii_digit)
        || !b[5..7].iter().all(u8::is_ascii_digit)
        || !b[8..10].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    let y = std::str::from_utf8(&b[..4]).ok()?.parse::<i64>().ok()?;
    let m = std::str::from_utf8(&b[5..7]).ok()?.parse::<i64>().ok()?;
    let d = std::str::from_utf8(&b[8..10]).ok()?.parse::<i64>().ok()?;
    let Some(dim) = days_in_month(y, m) else {
        return None;
    };
    if d < 1 || d > dim {
        return None;
    }
    Some((y, m, d))
}

fn record(pos: [f64; 3], val: f64, tdb: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = 0.0;
    r[8] = KERNEL_INVERSE_SQUARE;
    r[9] = FORCE_EM;
    r[25] = 0.0;
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

struct Skips {
    no_analysis: usize,
    no_sample: usize,
    no_location: usize,
    unjoined: usize,
    riss_station: usize,
    riss_witness: usize,
    value: usize,
    date: usize,
    clock: usize,
    frame: usize,
}

impl Skips {
    fn new() -> Skips {
        Skips {
            no_analysis: 0,
            no_sample: 0,
            no_location: 0,
            unjoined: 0,
            riss_station: 0,
            riss_witness: 0,
            value: 0,
            date: 0,
            clock: 0,
            frame: 0,
        }
    }
}

struct TableSkips {
    result_nokey: usize,
    analysis_nokey: usize,
    analysis_dup: usize,
    sample_nokey: usize,
    sample_dup: usize,
    location_nokey: usize,
    location_dup: usize,
    agol_bad: usize,
    page_void: usize,
}

impl TableSkips {
    fn new() -> TableSkips {
        TableSkips {
            result_nokey: 0,
            analysis_nokey: 0,
            analysis_dup: 0,
            sample_nokey: 0,
            sample_dup: 0,
            location_nokey: 0,
            location_dup: 0,
            agol_bad: 0,
            page_void: 0,
        }
    }
}

fn parse_locations(rows: &[JsonVal], tsk: &mut TableSkips) -> HashMap<String, LocationRow> {
    let mut map = HashMap::new();
    for r in rows {
        let Some(m) = row_obj(r) else {
            tsk.location_nokey += 1;
            continue;
        };
        let Some(key) = map_key(m, "loc_num") else {
            tsk.location_nokey += 1;
            continue;
        };
        let row = LocationRow {
            state_abbr: map_str(m, "state_abbr"),
            city_name: map_str(m, "city_name"),
            station: map_str(m, "station"),
        };
        match map.entry(key) {
            Entry::Occupied(_) => tsk.location_dup += 1,
            Entry::Vacant(v) => {
                v.insert(row);
            }
        }
    }
    map
}

fn parse_samples(rows: &[JsonVal], tsk: &mut TableSkips) -> HashMap<String, SampleRow> {
    let mut map = HashMap::new();
    for r in rows {
        let Some(m) = row_obj(r) else {
            tsk.sample_nokey += 1;
            continue;
        };
        let Some(key) = map_key(m, "samp_num") else {
            tsk.sample_nokey += 1;
            continue;
        };
        let row = SampleRow {
            loc_num: map_key(m, "loc_num"),
        };
        match map.entry(key) {
            Entry::Occupied(_) => tsk.sample_dup += 1,
            Entry::Vacant(v) => {
                v.insert(row);
            }
        }
    }
    map
}

fn parse_analyses(rows: &[JsonVal], tsk: &mut TableSkips) -> HashMap<String, AnalysisRow> {
    let mut map = HashMap::new();
    for r in rows {
        let Some(m) = row_obj(r) else {
            tsk.analysis_nokey += 1;
            continue;
        };
        let Some(key) = map_key(m, "ana_num") else {
            tsk.analysis_nokey += 1;
            continue;
        };
        let row = AnalysisRow {
            samp_num: map_key(m, "samp_num"),
        };
        match map.entry(key) {
            Entry::Occupied(_) => tsk.analysis_dup += 1,
            Entry::Vacant(v) => {
                v.insert(row);
            }
        }
    }
    map
}

fn parse_results(rows: &[JsonVal], tsk: &mut TableSkips) -> Vec<ResultRow> {
    let mut out = Vec::new();
    for r in rows {
        let Some(m) = row_obj(r) else {
            tsk.result_nokey += 1;
            continue;
        };
        let Some(ana_num) = map_key(m, "ana_num") else {
            tsk.result_nokey += 1;
            continue;
        };
        out.push(ResultRow {
            ana_num,
            result_in_si: map_f64(m, "result_in_si"),
            result_date: map_str(m, "result_date"),
        });
    }
    out
}

fn agol_features(src: &str, tsk: &mut TableSkips) -> Result<Vec<AgolFeature>, String> {
    let body = if src.starts_with("http") {
        curl_body(src).ok_or_else(|| format!("agol fetch void ({src})"))?
    } else {
        std::fs::read_to_string(src).map_err(|e| format!("agol read {src}: {e}"))?
    };
    let parsed = parse_json(&body).ok_or_else(|| "agol body carries no json".to_string())?;
    let JsonVal::Obj(m) = &parsed else {
        return Err("agol body is not an object".into());
    };
    if matches!(m.get("exceededTransferLimit"), Some(JsonVal::Bool(true))) {
        return Err("the agol layer spans more features than one page returned".into());
    }
    let feats = match m.get("features") {
        Some(JsonVal::Arr(a)) => a,
        _ => return Err("agol body carries no features array".into()),
    };
    let mut out = Vec::new();
    for f in feats {
        let JsonVal::Obj(fo) = f else {
            tsk.agol_bad += 1;
            continue;
        };
        let Some(JsonVal::Obj(attrs)) = fo.get("attributes") else {
            tsk.agol_bad += 1;
            continue;
        };
        let (Some(name), Some(city), Some(state_abbr)) = (
            map_str(attrs, "name"),
            map_str(attrs, "city"),
            map_str(attrs, "State_Abbr"),
        ) else {
            tsk.agol_bad += 1;
            continue;
        };
        let (lon, lat) = match fo.get("geometry") {
            Some(JsonVal::Obj(g)) => (
                match g.get("x") {
                    Some(JsonVal::Num(v)) => *v,
                    _ => f64::NAN,
                },
                match g.get("y") {
                    Some(JsonVal::Num(v)) => *v,
                    _ => f64::NAN,
                },
            ),
            _ => (f64::NAN, f64::NAN),
        };
        if !lon.is_finite()
            || !lat.is_finite()
            || !(-90.0..=90.0).contains(&lat)
            || !(-180.0..=180.0).contains(&lon)
        {
            tsk.agol_bad += 1;
            continue;
        }
        out.push(AgolFeature {
            name,
            city,
            state_abbr,
            lon,
            lat,
        });
    }
    if out.is_empty() {
        return Err("the agol layer carries no feature with a finite WGS84 point".into());
    }
    Ok(out)
}

fn compile(
    results: &[ResultRow],
    analysis: &HashMap<String, AnalysisRow>,
    sample: &HashMap<String, SampleRow>,
    location: &HashMap<String, LocationRow>,
    layer: &Layer,
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
) -> (Vec<[f64; 26]>, Skips, Vec<String>) {
    let mut records = Vec::new();
    let mut skips = Skips::new();
    let mut witness_lines = Vec::new();
    let mut join_cache: HashMap<String, Join> = HashMap::new();
    for row in results {
        let Some(ana) = analysis.get(&row.ana_num) else {
            skips.no_analysis += 1;
            continue;
        };
        let Some(samp_key) = &ana.samp_num else {
            skips.no_sample += 1;
            continue;
        };
        let Some(samp) = sample.get(samp_key) else {
            skips.no_sample += 1;
            continue;
        };
        let Some(loc_key) = &samp.loc_num else {
            skips.no_location += 1;
            continue;
        };
        let Some(loc) = location.get(loc_key) else {
            skips.no_location += 1;
            continue;
        };
        let join = match join_cache.get(loc_key) {
            Some(j) => j.clone(),
            None => {
                let j = resolve_join(loc, layer);
                match &j {
                    Join::RissStation(names) => {
                        skips.riss_station += 1;
                        if witness_lines.len() < RISS_LINE_CAP {
                            witness_lines.push(format!(
                                "riss station '{}' matches {} agol names: {}",
                                raw(&loc.station),
                                names.len(),
                                names.join(", ")
                            ));
                        }
                    }
                    Join::RissWitness {
                        station_name,
                        pair_name,
                    } => {
                        skips.riss_witness += 1;
                        if witness_lines.len() < RISS_LINE_CAP {
                            witness_lines.push(format!(
                                "riss witnesses disagree: station '{}' -> '{}' vs city/state '{} {}' -> '{}'",
                                raw(&loc.station),
                                station_name,
                                raw(&loc.state_abbr),
                                raw(&loc.city_name),
                                pair_name
                            ));
                        }
                    }
                    Join::Unjoined => skips.unjoined += 1,
                    Join::Coord(_) => {}
                }
                join_cache.insert(loc_key.clone(), j.clone());
                j
            }
        };
        let (lon, lat) = match join {
            Join::Coord(i) => (layer.features[i].lon, layer.features[i].lat),
            _ => continue,
        };
        let Some(val) = value_gate(row.result_in_si) else {
            skips.value += 1;
            continue;
        };
        let Some((y, m, d)) = parse_result_date(row.result_date.as_deref()) else {
            skips.date += 1;
            continue;
        };
        let Some(days) = days_from_civil(y, m, d) else {
            skips.date += 1;
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(days as f64 * SECS_PER_DAY) else {
            skips.clock += 1;
            continue;
        };
        let Some(pos) = body_fixed_to_icrs(EARTH, lat, lon, 0.0, tdb, eph) else {
            skips.frame += 1;
            continue;
        };
        records.push(record(pos, val, tdb));
    }
    (records, skips, witness_lines)
}

fn run(args: &[String]) -> Result<(), String> {
    let usage = "usage: radnet_compiler --out <bin> [--root <efservice-root>] [--agol <url|json-file>] [--ephemeris <path|url>] [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let root = match arg_value(args, "--root") {
        Some(v) => v,
        None => EF_ROOT.to_string(),
    };
    let agol_src = match arg_value(args, "--agol") {
        Some(v) => v,
        None => AGOL_URL.to_string(),
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => return Err(usage.to_string()),
    };

    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;

    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(src) if src.starts_with("http") => fetch_raw_bytes(&src, 604800)
            .ok_or_else(|| format!("earth ephemeris fetch void ({src})"))?,
        Some(path) => std::fs::read(&path).map_err(|e| format!("earth ephemeris read {path}: {e}"))?,
        None => fetch_raw_bytes(&body_url(EARTH), 604800)
            .ok_or_else(|| "earth ephemeris fetch void (CDN)".to_string())?,
    };
    let earth = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the earth ephemeris binary stays unread — no ICRS frame".to_string())?;
    let eph = HashMap::from([(EARTH.to_string(), earth)]);

    let mut tsk = TableSkips::new();
    let features = agol_features(&agol_src, &mut tsk)?;
    let layer = layer_from(features);
    eprintln!("radnet: {} agol features read", layer.features.len());

    let (loc_rows, pv) = fetch_table(&root, "ERM_LOCATION", PAGE_SIZE)?;
    tsk.page_void += pv;
    let location = parse_locations(&loc_rows, &mut tsk);
    let (samp_rows, pv) = fetch_table(&root, "ERM_SAMPLE", PAGE_SIZE)?;
    tsk.page_void += pv;
    let sample = parse_samples(&samp_rows, &mut tsk);
    let (ana_rows, pv) = fetch_table(&root, "ERM_ANALYSIS", PAGE_SIZE)?;
    tsk.page_void += pv;
    let analysis = parse_analyses(&ana_rows, &mut tsk);
    let (res_rows, pv) = fetch_table(&root, "ERM_RESULT", PAGE_SIZE)?;
    tsk.page_void += pv;
    let results = parse_results(&res_rows, &mut tsk);

    let (records, skips, riss_lines) = compile(
        &results,
        &analysis,
        &sample,
        &location,
        &layer,
        &lsk,
        &eph,
    );
    if records.is_empty() {
        return Err(format!(
            "no record left the harvest — {} result rows read; skips: no_analysis {}, no_sample {}, no_location {}, unjoined {}, riss_station {}, riss_witness {}, value {}, date {}, clock {}, frame {}",
            results.len(),
            skips.no_analysis,
            skips.no_sample,
            skips.no_location,
            skips.unjoined,
            skips.riss_station,
            skips.riss_witness,
            skips.value,
            skips.date,
            skips.clock,
            skips.frame
        ));
    }
    let bin = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out}: {e}"))?;
    let roundtrip = read_bin(&bin)
        .ok_or_else(|| format!("{out}: roundtrip parse void — the asset stays unverified"))?;
    eprintln!(
        "radnet: {} records, {} B -> {out} (roundtrip: {roundtrip}); tables: result {} (nokey {}), analysis {} (nokey {}, dup {}), sample {} (nokey {}, dup {}), location {} (nokey {}, dup {}); agol {} (bad {}); page_void {}; skips: no_analysis {}, no_sample {}, no_location {}, unjoined {}, riss_station {}, riss_witness {}, value {}, date {}, clock {}, frame {}",
        records.len(),
        bin.len(),
        results.len(),
        tsk.result_nokey,
        analysis.len(),
        tsk.analysis_nokey,
        tsk.analysis_dup,
        sample.len(),
        tsk.sample_nokey,
        tsk.sample_dup,
        location.len(),
        tsk.location_nokey,
        tsk.location_dup,
        layer.features.len(),
        tsk.agol_bad,
        tsk.page_void,
        skips.no_analysis,
        skips.no_sample,
        skips.no_location,
        skips.unjoined,
        skips.riss_station,
        skips.riss_witness,
        skips.value,
        skips.date,
        skips.clock,
        skips.frame
    );
    for line in &riss_lines {
        eprintln!("radnet {line}");
    }
    if ci_mode && !upload_release(CDN_TAG, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("radnet_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feat(name: &str, city: &str, abbr: &str) -> AgolFeature {
        AgolFeature {
            name: name.into(),
            city: city.into(),
            state_abbr: abbr.into(),
            lon: -75.5,
            lat: 39.2,
        }
    }

    fn loc(station: &str, abbr: &str, city: &str) -> LocationRow {
        LocationRow {
            station: Some(station.into()),
            state_abbr: Some(abbr.into()),
            city_name: Some(city.into()),
        }
    }

    #[test]
    fn normalize_folds_case_whitespace_and_punctuation() {
        assert_eq!(normalize("Anniston, AL"), "anniston al");
        assert_eq!(normalize("ST. LOUIS"), "st louis");
        assert_eq!(normalize("  DENVER  "), "denver");
        assert_eq!(normalize("dover-air-force"), "dover air force");
        assert_eq!(normalize("D.C."), "d c");
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn join_matches_the_exact_station_key() {
        let layer = layer_from(vec![feat("Dover", "Dover", "DE"), feat("Wilmington", "Wilmington", "DE")]);
        assert!(matches!(
            resolve_join(&loc("DOVER", "DE", "DOVER"), &layer),
            Join::Coord(0)
        ));
        assert!(matches!(
            resolve_join(&loc(" wilmington ", "de", "wilmington"), &layer),
            Join::Coord(1)
        ));
    }

    #[test]
    fn join_falls_back_to_a_unique_city_state_pair() {
        let layer = layer_from(vec![feat("Site 31", "Dover", "DE")]);
        assert!(matches!(
            resolve_join(&loc("UNKNOWN STATION", "de", "dover"), &layer),
            Join::Coord(0)
        ));
    }

    #[test]
    fn join_refuses_a_nonunique_pair_without_station() {
        let layer = layer_from(vec![feat("Site A", "Dover", "DE"), feat("Site B", "Dover", "DE")]);
        assert!(matches!(
            resolve_join(&loc("X", "DE", "Dover"), &layer),
            Join::Unjoined
        ));
        assert!(matches!(
            resolve_join(&loc("", "", ""), &layer),
            Join::Unjoined
        ));
    }

    #[test]
    fn join_marks_multiple_station_matches_a_riss() {
        let layer = layer_from(vec![feat("Dover", "Dover", "DE"), feat("Dover", "Dover AFB", "DE")]);
        match resolve_join(&loc("Dover", "DE", "Dover"), &layer) {
            Join::RissStation(names) => assert_eq!(names.len(), 2),
            other => panic!("the join carries {other:?}, not RissStation"),
        }
    }

    #[test]
    fn join_marks_conflicting_witnesses_a_riss() {
        let layer = layer_from(vec![
            feat("Dover", "Dover AFB", "DE"),
            feat("Dover AFB", "Dover", "DE"),
        ]);
        match resolve_join(&loc("Dover", "DE", "Dover"), &layer) {
            Join::RissWitness {
                station_name,
                pair_name,
            } => {
                assert_eq!(station_name, "Dover");
                assert_eq!(pair_name, "Dover AFB");
            }
            other => panic!("the join carries {other:?}, not RissWitness"),
        }
    }

    #[test]
    fn station_wins_when_the_pair_is_no_witness() {
        let layer = layer_from(vec![feat("Dover", "Dover", "DE"), feat("Dover AFB", "Dover", "DE")]);
        assert!(matches!(
            resolve_join(&loc("Dover", "DE", "Dover"), &layer),
            Join::Coord(0)
        ));
    }

    #[test]
    fn value_gate_admits_only_finite_positive() {
        assert_eq!(value_gate(Some(1.5)), Some(1.5));
        assert_eq!(value_gate(Some(-0.5)), None);
        assert_eq!(value_gate(Some(0.0)), None);
        assert_eq!(value_gate(Some(f64::NAN)), None);
        assert_eq!(value_gate(Some(f64::INFINITY)), None);
        assert_eq!(value_gate(None), None);
    }

    #[test]
    fn result_date_parse_accepts_the_measured_shape() {
        assert_eq!(parse_result_date(Some("2015-06-15")), Some((2015, 6, 15)));
        assert_eq!(parse_result_date(Some("2024-02-29")), Some((2024, 2, 29)));
        assert_eq!(parse_result_date(Some("2023-02-29")), None);
        assert_eq!(parse_result_date(Some("2015-6-15")), None);
        assert_eq!(parse_result_date(Some("2015-06")), None);
        assert_eq!(parse_result_date(Some("15-06-15")), None);
        assert_eq!(parse_result_date(Some("2015-13-01")), None);
        assert_eq!(parse_result_date(Some("2015-06-31")), None);
        assert_eq!(parse_result_date(Some("2015-06-1x")), None);
        assert_eq!(parse_result_date(Some("")), None);
        assert_eq!(parse_result_date(None), None);
    }

    #[test]
    fn date_precision_spans_exactly_one_day_and_clock_refuses_pre_1972() {
        let lsk = embedded_lsk().expect("embedded lsk");
        let unix_a = days_from_civil(2015, 6, 15).expect("civil") as f64 * SECS_PER_DAY;
        let unix_b = days_from_civil(2015, 6, 16).expect("civil") as f64 * SECS_PER_DAY;
        assert_eq!(unix_b - unix_a, 86400.0);
        let tdb_a = lsk.unix_to_tdb(unix_a).expect("tdb");
        let tdb_b = lsk.unix_to_tdb(unix_b).expect("tdb");
        assert_eq!(tdb_b - tdb_a, 86400.0);
        let pre = days_from_civil(1971, 12, 31).expect("civil") as f64 * SECS_PER_DAY;
        assert!(lsk.unix_to_tdb(pre).is_none());
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record([1.0, 2.0, 3.0], 0.5, 8.0e8);
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 2.0);
        assert_eq!(r[2], 3.0);
        assert_eq!(r[3], 0.5);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], TAU_S);
        assert_eq!(r[7], 0.0);
        assert_eq!(r[8], KERNEL_INVERSE_SQUARE);
        assert_eq!(r[9], FORCE_EM);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
        assert_eq!(r[25], 0.0);
    }

    #[test]
    fn bin_roundtrip_counts_records() {
        let records = vec![
            record([1.0, 2.0, 3.0], 0.4, 8.0e8),
            record([4.0, 5.0, 6.0], 0.7, 8.0e8 + 1.0),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(read_bin(&bytes), Some(2));
        assert!(read_bin(b"X").is_none());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert!(read_bin(&bad).is_none());
    }

    #[test]
    fn compile_counts_every_skip_class() {
        let layer = layer_from(vec![feat("Dover", "Dover", "DE")]);
        let lsk = embedded_lsk().expect("embedded lsk");
        let mut analysis = HashMap::new();
        analysis.insert("11".into(), AnalysisRow { samp_num: Some("21".into()) });
        analysis.insert("12".into(), AnalysisRow { samp_num: None });
        analysis.insert("13".into(), AnalysisRow { samp_num: Some("99".into()) });
        analysis.insert("14".into(), AnalysisRow { samp_num: Some("22".into()) });
        analysis.insert("15".into(), AnalysisRow { samp_num: Some("23".into()) });
        let mut sample = HashMap::new();
        sample.insert("21".into(), SampleRow { loc_num: Some("31".into()) });
        sample.insert("22".into(), SampleRow { loc_num: None });
        sample.insert("23".into(), SampleRow { loc_num: Some("32".into()) });
        let mut location = HashMap::new();
        location.insert("31".into(), loc("Dover", "DE", "Dover"));
        location.insert("32".into(), loc("X", "MD", "X"));
        let results = vec![
            ResultRow { ana_num: "10".into(), result_in_si: Some(1.0), result_date: Some("2015-06-15".into()) },
            ResultRow { ana_num: "11".into(), result_in_si: Some(-1.0), result_date: Some("2015-06-15".into()) },
            ResultRow { ana_num: "11".into(), result_in_si: Some(1.0), result_date: Some("not-a-date".into()) },
            ResultRow { ana_num: "12".into(), result_in_si: Some(1.0), result_date: Some("2015-06-15".into()) },
            ResultRow { ana_num: "13".into(), result_in_si: Some(1.0), result_date: Some("2015-06-15".into()) },
            ResultRow { ana_num: "14".into(), result_in_si: Some(1.0), result_date: Some("2015-06-15".into()) },
            ResultRow { ana_num: "15".into(), result_in_si: Some(1.0), result_date: Some("2015-06-15".into()) },
            ResultRow { ana_num: "11".into(), result_in_si: Some(1.0), result_date: Some("1971-12-31".into()) },
            ResultRow { ana_num: "11".into(), result_in_si: Some(1.0), result_date: Some("2015-06-15".into()) },
        ];
        let (records, skips, lines) = compile(
            &results,
            &analysis,
            &sample,
            &location,
            &layer,
            &lsk,
            &HashMap::new(),
        );
        assert!(records.is_empty());
        assert!(lines.is_empty());
        assert_eq!(skips.no_analysis, 1);
        assert_eq!(skips.no_sample, 2);
        assert_eq!(skips.no_location, 1);
        assert_eq!(skips.unjoined, 1);
        assert_eq!(skips.riss_station, 0);
        assert_eq!(skips.riss_witness, 0);
        assert_eq!(skips.value, 1);
        assert_eq!(skips.date, 1);
        assert_eq!(skips.clock, 1);
        assert_eq!(skips.frame, 1);
    }

    #[test]
    fn agol_parse_reads_features_and_counts_bad_geometry() {
        let dir = std::env::temp_dir().join(format!("radnet_agol_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("tmp dir");
        let path = dir.join("agol.json");
        std::fs::write(
            &path,
            r#"{"features":[{"attributes":{"name":"Dover","city":"Dover","State_Abbr":"DE"},"geometry":{"x":-75.5,"y":39.2}},{"attributes":{"name":"Bad","city":"X","State_Abbr":"XX"},"geometry":{"x":"nope","y":0.0}}],"exceededTransferLimit":false}"#,
        )
        .expect("write");
        let mut tsk = TableSkips::new();
        let feats = agol_features(&path.to_string_lossy(), &mut tsk).expect("parse");
        assert_eq!(feats.len(), 1);
        assert_eq!(tsk.agol_bad, 1);
        assert_eq!(feats[0].name, "Dover");
        assert!((feats[0].lon + 75.5).abs() < 1e-12);
        assert!((feats[0].lat - 39.2).abs() < 1e-12);
    }

    #[test]
    fn agol_parse_refuses_a_transfer_limit_page() {
        let dir = std::env::temp_dir().join(format!("radnet_agol_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("tmp dir");
        let path = dir.join("agol_limit.json");
        std::fs::write(&path, r#"{"features":[],"exceededTransferLimit":true}"#).expect("write");
        let mut tsk = TableSkips::new();
        assert!(agol_features(&path.to_string_lossy(), &mut tsk).is_err());
    }
}
