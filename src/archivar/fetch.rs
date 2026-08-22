use super::*;

pub const FETCH_BUDGET: usize = 1 << 3;

pub const FETCH_VOID_CAP: u32 = 1 << 2;

pub const FETCH_DURATION_RING: usize = 1 << 4;

pub fn fetch_raw(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<String> {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    if let Some(b) = body {
        cmd.arg("-X").arg("POST");
        cmd.arg("-d").arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "\r\x1b[Kfetch returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}


pub fn curl_base(ttl: u64, parallel_max: u8) -> Command {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg("5")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2");
    if parallel_max > 0 {
        cmd.arg("--parallel")
            .arg("--parallel-max")
            .arg(parallel_max.to_string());
    }
    cmd.arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    cmd
}


pub fn fetch_raw_bytes(url: &str, ttl: u64) -> Option<Vec<u8>> {
    let mut cmd = curl_base(ttl, 0);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "\r\x1b[Kfetch_bytes returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}


pub fn fetch_raw_probe(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg("1")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("1")
        .arg("-m")
        .arg("10")
        .arg("--connect-timeout")
        .arg("5");
    if let Some(b) = body {
        cmd.arg("-X")
            .arg("POST")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        None
    }
}


pub fn fetch_raw_bytes_post(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<Vec<u8>> {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("--retry")
        .arg("5")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string())
        .arg("-X")
        .arg("POST");
    if let Some(b) = body {
        cmd.arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "fetch_bytes_post returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}


pub type Origin = u32;


#[derive(Clone)]
pub struct OriginState {
    pub fetched: f64,
    pub started: f64,
    pub prev_epoch: f64,
    pub prev_abs: [f64; 3],
    pub prev_motion: Option<Motion>,
    pub resid_ema: f64,
    pub has_prev: bool,
    pub failures: u32,
    pub in_flight: bool,
}


pub fn origin_stale(
    origins: &HashMap<Origin, OriginState>,
    origin: Origin,
    ttl: u64,
    now: f64,
    jump_epoch: Option<f64>,
) -> bool {
    match origins.get(&origin) {
        Some(o) => {
            let backoff = (ttl as f64 / Φ) * (2f64).powi(o.failures.min(FETCH_VOID_CAP) as i32);
            let jumped = match jump_epoch {
                Some(j) => o.fetched < j,
                None => false,
            };
            !o.in_flight && (now - o.fetched >= backoff || jumped)
        }
        None => true,
    }
}


pub fn begin_fetch(origins: &mut HashMap<Origin, OriginState>, origin: Origin, now: f64) {
    let st = origins.entry(origin).or_insert(OriginState {
        fetched: now,
        started: now,
        prev_epoch: now,
        prev_abs: [0.0, 0.0, 0.0],
        prev_motion: None,
        resid_ema: 0.0,
        has_prev: false,
        failures: 0,
        in_flight: false,
    });
    st.started = now;
    st.in_flight = true;
}


pub fn settle_fetch(st: &mut OriginState, ok: bool, now: f64) {
    st.fetched = now;
    st.in_flight = false;
    if ok {
        st.failures = 0;
    } else {
        st.failures = (st.failures + 1).min(FETCH_VOID_CAP);
    }
}


pub fn record_fetch_duration(
    ring: &mut [f64; FETCH_DURATION_RING],
    len: &mut usize,
    idx: &mut usize,
    d: f64,
) {
    ring[*idx] = d;
    *idx = (*idx + 1) % FETCH_DURATION_RING;
    *len = (*len + 1).min(FETCH_DURATION_RING);
}


pub fn median_fetch_duration(ring: &[f64; FETCH_DURATION_RING], len: usize) -> Option<f64> {
    if len == 0 {
        return None;
    }
    let mut vals: Vec<f64> = ring[..len].to_vec();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = len / 2;
    if len % 2 == 1 {
        Some(vals[mid])
    } else {
        Some((vals[mid - 1] + vals[mid]) / 2.0)
    }
}


pub fn presence_gate(
    presences: &[(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)],
    pos: (f64, f64, f64),
    reach: f64,
    body_radius: f64,
    v_anchor: Option<[f64; 3]>,
    median_fetch: Option<f64>,
) -> bool {
    presences
        .iter()
        .any(|&(_, px, py, pz, _range, vx, vy, vz, _thrust, grid_step)| {
            let limit = reach + body_radius.max(Φ * grid_step);
            let dx = pos.0 - px;
            let dy = pos.1 - py;
            let dz = pos.2 - pz;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            if dist <= limit {
                return true;
            }
            if vx * vx + vy * vy + vz * vz == 0.0 {
                return false;
            }
            let [ax, ay, az] = match v_anchor {
                Some(a) => a,
                None => return false,
            };
            let median = match median_fetch {
                Some(m) => m,
                None => return false,
            };
            let rel = [vx - ax, vy - ay, vz - az];
            let closing = (rel[0] * dx + rel[1] * dy + rel[2] * dz) / dist;
            if closing <= 0.0 {
                return false;
            }
            (dist - limit) / closing < Φ * median
        })
}


pub fn json_has_content(v: &JsonVal) -> bool {
    match v {
        JsonVal::Arr(arr) => !arr.is_empty() || arr.iter().any(json_has_content),
        JsonVal::Obj(map) => map.values().any(json_has_content),
        JsonVal::Null | JsonVal::Bool(_) | JsonVal::Str(_) | JsonVal::Num(_) => false,
    }
}


pub fn diagnose_no_samples(src: &SourceConfig, body: &str) -> String {
    let parsed = parse_json(body);
    match parsed {
        None => {
            let trimmed = body.trim();
            if trimmed.is_empty() {
                "empty-response (empty body)".to_string()
            } else {
                "data-present (non-JSON body: HTML/XML/text)".to_string()
            }
        }
        Some(j) => {
            let mut arr_has_rows = false;
            let mut key_found = false;
            for ext in &src.extracts {
                match ext {
                    Extract::Map {
                        arr_path,
                        lat_key,
                        lon_key,
                        fields,
                        ..
                    } => {
                        if let Some(JsonVal::Arr(arr)) = jpath_val(&j, arr_path) {
                            if !arr.is_empty() {
                                arr_has_rows = true;
                            }
                        }
                        for fk in [lat_key.as_str(), lon_key.as_str()] {
                            if jpath_val(&j, fk).is_some() {
                                key_found = true;
                            }
                        }
                        for fc in fields {
                            if jpath_val(&j, &fc.key).is_some() {
                                key_found = true;
                            }
                        }
                    }
                    Extract::CelestialMap {
                        arr_path, fields, ..
                    }
                    | Extract::Flatten {
                        arr_path, fields, ..
                    }
                    | Extract::CmrPolygon {
                        arr_path, fields, ..
                    }
                    | Extract::CelestialPolygon {
                        arr_path, fields, ..
                    }
                    | Extract::KeplerMap {
                        arr_path, fields, ..
                    }
                    | Extract::ProfileMap {
                        arr_path, fields, ..
                    } => {
                        if let Some(JsonVal::Arr(arr)) = jpath_val(&j, arr_path) {
                            if !arr.is_empty() {
                                arr_has_rows = true;
                            }
                        }
                        for fc in fields {
                            if jpath_val(&j, &fc.key).is_some() {
                                key_found = true;
                            }
                        }
                    }
                    Extract::Rows { .. } | Extract::GeojsonEvents { .. } | Extract::Hapi(_) => {
                        if json_has_content(&j) {
                            arr_has_rows = true;
                        }
                    }
                    Extract::Field(FieldConfig { key, .. })
                    | Extract::First(FieldConfig { key, .. }, _)
                    | Extract::Last(FieldConfig { key, .. }, _)
                    | Extract::Count(FieldConfig { key, .. })
                    | Extract::Path(FieldConfig { key, .. })
                    | Extract::Deep(FieldConfig { key, .. })
                    | Extract::LastRow(FieldConfig { key, .. })
                    | Extract::ObjLast(FieldConfig { key, .. })
                    | Extract::Regex(FieldConfig { key, .. }) => {
                        if jpath_val(&j, key).is_some()
                            || jpath_val(
                                &j,
                                if let Some((p, _)) = key.rsplit_once('.') {
                                    p
                                } else {
                                    key
                                },
                            )
                            .is_some()
                        {
                            key_found = true;
                        }
                    }
                    Extract::LastObj(_, _, _, _)
                    | Extract::LastLine(_)
                    | Extract::XmlCount(_, _) => {
                        if json_has_content(&j) {
                            key_found = true;
                        }
                    }
                    Extract::Alerce(_) => {
                        if json_has_content(&j) {
                            key_found = true;
                        }
                    }
                }
            }
            if arr_has_rows {
                "data-present (container array has rows but extract yielded nothing)".to_string()
            } else if key_found {
                "data-present (keys exist but no rows extracted)".to_string()
            } else if json_has_content(&j) {
                "data-present (JSON has content but declared keys absent)".to_string()
            } else {
                "empty-response (JSON parsed but all containers empty)".to_string()
            }
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum VoidClass {
    Key,
    Drift,
    Quiet,
    Kaputt,
}


impl VoidClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoidClass::Key => "key-void",
            VoidClass::Drift => "drift-void",
            VoidClass::Quiet => "ruhig-void",
            VoidClass::Kaputt => "kaputt",
        }
    }
}


pub struct VoidFinding {
    pub url: String,
    pub class: VoidClass,
    pub detail: String,
}


pub fn civil_date(unix: u64) -> (i64, u32, u32) {
    let days = (unix / 86400) as i64;
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}


pub fn date_str(unix: u64) -> String {
    let (y, m, d) = civil_date(unix);
    format!("{:04}-{:02}-{:02}", y, m, d)
}


pub fn hour_str(unix: u64) -> String {
    format!(
        "{}T{:02}:{:02}:{:02}Z",
        date_str(unix),
        (unix / 3600) % 24,
        (unix / 60) % 60,
        unix % 60
    )
}


pub fn live_markers() -> Vec<(String, String)> {
    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, m, d) = civil_date(unix);
    let jd = unix as f64 / 86400.0 + 2440587.5;
    vec![
        ("{today}".into(), date_str(unix)),
        ("{yesterday}".into(), date_str(unix - 86400)),
        ("{tomorrow}".into(), date_str(unix + 86400)),
        ("{now}".into(), hour_str(unix)),
        ("{year}".into(), format!("{:04}", y)),
        ("{month}".into(), format!("{:02}", m)),
        ("{day}".into(), format!("{:02}", d)),
        ("{lat}".into(), "29.5".into()),
        ("{lon}".into(), "-95.0".into()),
        ("{ra}".into(), "0.0".into()),
        ("{dec}".into(), "0.0".into()),
        ("{target}".into(), "Ceres".into()),
        ("{week_ago}".into(), date_str(unix - 7 * 86400)),
        ("{hour_ago}".into(), hour_str(unix - 3600)),
        ("{body}".into(), "ISS".into()),
        ("{lon_min}".into(), "-95.0".into()),
        ("{lon_max}".into(), "-94.0".into()),
        ("{lat_min}".into(), "29.0".into()),
        ("{lat_max}".into(), "30.0".into()),
        ("{grid}".into(), "29.5,-95.0|29.6,-95.0".into()),
        ("{nearest_station}".into(), "8518750".into()),
        ("{jd_now}".into(), format!("{:.2}", jd)),
        ("{jd_start}".into(), format!("{:.2}", jd - 1.0)),
        ("{jd_end}".into(), format!("{:.2}", jd)),
    ]
}


pub fn unresolved_key(template: &str, env: &HashMap<String, String>) -> Option<String> {
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            return None;
        };
        let key = &rest[..end];
        let upper = key.to_uppercase();
        match env.get(key).or_else(|| env.get(&upper)) {
            Some(v) if !v.is_empty() => {}
            _ => return Some(key.to_string()),
        }
        rest = &rest[end + 1..];
    }
    None
}


pub fn live_sweep(
    env: &HashMap<String, String>,
    now: f64,
    lsk: &LeapSeconds,
    limit: usize,
) -> (usize, Vec<VoidFinding>) {
    let srcs = load_sources();
    let markers = live_markers();
    let mut ok = 0usize;
    let mut findings: Vec<VoidFinding> = Vec::new();
    let mut budget = limit;
    for s in srcs.iter() {
        if s.url.starts_with("https://github.com/omegaflow/sources") {
            continue;
        }
        if s.fanout_cap > 0 || s.format == "csv_zip" || s.format == "kernel_text" {
            continue;
        }
        if matches!(
            s.format.as_str(),
            "ephemeris_binary"
                | "orbit_bin"
                | "catalog_dastcom"
                | "netcdf"
                | "finals"
                | "ionex"
                | "alerce"
                | "catalog_tycho"
                | "spectral"
                | "lightcurve"
                | "rpw_efield"
                | "goes_xrs"
                | "wind_waves"
                | "gong_modes"
        ) {
            continue;
        }
        if budget == 0 {
            break;
        }
        budget -= 1;
        let mut url = s.url.clone();
        for (k, v) in &markers {
            url = url.replace(k, v);
        }
        if let Some(key) = unresolved_key(&url, env) {
            findings.push(VoidFinding {
                url: s.url.clone(),
                class: VoidClass::Key,
                detail: format!("marker {{{}}} absent in .secrets.local", key),
            });
            continue;
        }
        let mut header_void = None;
        for (_, v) in &s.headers {
            if let Some(key) = unresolved_key(v, env) {
                header_void = Some(key);
                break;
            }
        }
        if let Some(key) = header_void {
            findings.push(VoidFinding {
                url: s.url.clone(),
                class: VoidClass::Key,
                detail: format!("header marker {{{}}} absent in .secrets.local", key),
            });
            continue;
        }
        let url = resolve_secret(&url, env);
        let headers = render_headers(&s.headers, env);
        let body = match fetch_one(&url, None, &headers, s.ttl, Some(now)) {
            Some(b) => b,
            None => {
                findings.push(VoidFinding {
                    url: s.url.clone(),
                    class: VoidClass::Kaputt,
                    detail: "fetch void".into(),
                });
                continue;
            }
        };
        match extract(s, &body, now, lsk) {
            ExtractResult::Measurements(v) | ExtractResult::WithEphemeris(v, _) => {
                if v.is_empty() {
                    let diag = diagnose_no_samples(s, &body);
                    let class = if diag.contains("all containers empty")
                        || diag.contains("no rows extracted")
                    {
                        VoidClass::Quiet
                    } else {
                        VoidClass::Drift
                    };
                    findings.push(VoidFinding {
                        url: s.url.clone(),
                        class,
                        detail: diag,
                    });
                } else {
                    ok += 1;
                }
            }
        }
    }
    (ok, findings)
}


pub struct FetchResult {
    pub source_idx: usize,
    pub channels: Vec<(Channel, FieldConfig)>,
    pub eph_update: Option<(String, BodyEphemeris)>,
    pub asteroid_samples: Vec<Sample>,
    pub star_samples: Vec<Sample>,
    pub curves: Option<Arc<CurveSet>>,
    pub spectral: Option<SpectralHash>,
    pub fetch_ok: bool,
}


pub fn rfc1123_to_unix(s: &str) -> Option<u64> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }
    let day: u32 = parts[1].parse().ok()?;
    let month = match parts[2] {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return None,
    };
    let year: i64 = parts[3].parse().ok()?;
    let mut hms = parts[4].split(':');
    let hh: u64 = hms.next()?.parse().ok()?;
    let mm: u64 = hms.next()?.parse().ok()?;
    let ss: u64 = hms.next()?.parse().ok()?;
    let days = ymd_to_days(year, month, day)?;
    Some(days * 86400 + hh * 3600 + mm * 60 + ss)
}


pub fn cdn_fresh(cdn_url: &str, ttl: u64) -> bool {
    const CI_REFRESH_S: u64 = 300;
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).ceil() as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-I")
        .arg("-L")
        .arg("-m")
        .arg(connect_t.to_string())
        .arg(cdn_url);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => return false,
    };
    if !output.status.success() {
        return false;
    }
    let head = String::from_utf8_lossy(&output.stdout).to_string();
    let lm = match extract_header(&head, "last-modified") {
        Some(v) => v,
        None => return false,
    };
    let asset_ts = match rfc1123_to_unix(&lm) {
        Some(t) => t,
        None => return false,
    };
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return false,
    };
    now.saturating_sub(asset_ts) < ttl.max(CI_REFRESH_S)
}


pub fn fetch_one(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
    now: Option<f64>,
) -> Option<String> {
    let manifest = cdn_manifest_map();
    let asset_name = |u: &str| -> String {
        manifest
            .get(u)
            .cloned()
            .unwrap_or_else(|| source_name_from_url(u))
    };
    if !url.starts_with("https://github.com/omegaflow/sources") {
        if let Some(netloc) = extract_netloc(url) {
            let name = asset_name(url);
            if !name.is_empty() {
                let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                if now.map_or(false, |n| cache_fresh_at(&cache_path, ttl, n)) {
                    if let Some(cached) = std::fs::read_to_string(&cache_path).ok() {
                        return Some(cached);
                    }
                }
                let cdn_url = format!("{}/{}/{}.json", crate::cdn::CDN_BASE, netloc, name);
                if cdn_fresh(&cdn_url, ttl) {
                    if let Some(cdn_body) = fetch_raw(&cdn_url, None, &[], ttl) {
                        if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        match std::fs::write(&cache_path, cdn_body.as_bytes()) {
                            Ok(()) => {
                                if let Some(n) = now {
                                    write_epoch_stamp(&cache_path, n);
                                }
                            }
                            Err(_) => {
                                eprintln!("cache {}: write void — refetch next cycle", cache_path)
                            }
                        }
                        return Some(cdn_body);
                    }
                }
            }
        }
    }
    let live = fetch_raw(url, body, headers, ttl);
    if let Some(ref r) = live {
        if let Some(netloc) = extract_netloc(url) {
            let name = asset_name(url);
            if !name.is_empty() {
                let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                match std::fs::write(&cache_path, r.as_bytes()) {
                    Ok(()) => {
                        if let Some(n) = now {
                            write_epoch_stamp(&cache_path, n);
                        }
                    }
                    Err(_) => {
                        eprintln!("cache {}: write void — refetch next cycle", cache_path)
                    }
                }
            }
        }
    }
    live
}


pub fn cache_fresh(path: &str, ttl: u64) -> bool {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };
    let modified = match meta.modified() {
        Ok(m) => m,
        Err(_) => return false,
    };
    match std::time::SystemTime::now().duration_since(modified) {
        Ok(age) => age.as_secs() < ttl,
        Err(_) => false,
    }
}


pub fn cache_fresh_at(path: &str, ttl: u64, t_presence: f64) -> bool {
    match read_epoch_stamp(path) {
        Some(epoch) => (t_presence - epoch).abs() < ttl as f64,
        None => false,
    }
}


pub fn read_epoch_stamp(path: &str) -> Option<f64> {
    let stamp_path = format!("{}.epoch", path);
    std::fs::read_to_string(stamp_path)
        .ok()
        .and_then(|t| t.trim().parse::<f64>().ok())
}


pub fn write_epoch_stamp(path: &str, epoch: f64) {
    let stamp_path = format!("{}.epoch", path);
    if std::fs::write(&stamp_path, epoch.to_string()).is_err() {
        eprintln!(
            "cache {}: epoch stamp write void — refetch next cycle",
            path
        );
    }
}


pub fn machine_now_tdb() -> Option<f64> {
    embedded_lsk().and_then(|l| l.system_now_tdb())
}


pub fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}


pub fn load_env() -> HashMap<String, String> {
    let mut env: HashMap<String, String> = std::env::vars().collect();
    for name in &[".env", ".secrets.local"] {
        if let Ok(content) = std::fs::read_to_string(resolve_asset(name)) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some(eq) = line.find('=') {
                    let key = line[..eq].trim().to_string();
                    let val = line[eq + 1..].trim().to_string();
                    if !env.contains_key(&key) {
                        env.insert(key, val);
                    }
                }
            }
        }
    }
    env
}


pub fn resolve_secret(url: &str, env: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(url.len());
    let mut rest = url;
    while let Some(start) = rest.find('{') {
        result.push_str(&rest[..start]);
        rest = &rest[start + 1..];
        if let Some(end) = rest.find('}') {
            let key = &rest[..end];
            let upper = key.to_uppercase();
            match env.get(key).or_else(|| env.get(&upper)) {
                Some(val) => result.push_str(val),
                None => eprintln!("env marker {{{}}} absent — substituting void", key),
            }
            rest = &rest[end + 1..];
        } else {
            result.push('{');
        }
    }
    result.push_str(rest);
    result
}


pub fn secret_resolves_void(template: &str, env: &HashMap<String, String>) -> bool {
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            return false;
        };
        let key = &rest[..end];
        let upper = key.to_uppercase();
        match env.get(key).or_else(|| env.get(&upper)) {
            Some(v) if !v.is_empty() => {}
            _ => return true,
        }
        rest = &rest[end + 1..];
    }
    false
}


pub fn url_has_template(url: &str) -> bool {
    let mut rest = url;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            return false;
        };
        if rest[..end].chars().any(|c| c.is_ascii_lowercase()) {
            return true;
        }
        rest = &rest[end + 1..];
    }
    false
}


pub fn url_is_fanout(url: &str) -> bool {
    url.contains("{station}") || url.contains("{nearest_station}")
}


pub fn frame_anchor(frame: &Frame) -> (f64, f64) {
    match frame {
        Frame::Surface { lat, lon, .. } => (*lat, *lon),
        _ => (0.0, 0.0),
    }
}


pub fn extract_netloc(url: &str) -> Option<&str> {
    let after = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let netloc = after.split('/').next()?;
    Some(if let Some(s) = netloc.strip_prefix("www.") {
        s
    } else {
        netloc
    })
}


pub fn route_segments(url: &str) -> Option<(String, Vec<String>)> {
    let after = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let (netloc, rest) = match after.split_once('/') {
        Some((n, r)) => (n, r),
        None => (after, ""),
    };
    let host = netloc.strip_prefix("www.").unwrap_or(netloc);
    let path = rest.split(|c| c == '?' || c == '#').next().unwrap_or("");
    let mut segs: Vec<String> = Vec::new();
    for s in path.split('/') {
        if s.is_empty() {
            continue;
        }
        let seg = if s.starts_with('{') && s.ends_with('}') {
            "*".to_string()
        } else {
            s.to_string()
        };
        segs.push(seg);
    }
    Some((host.to_string(), segs))
}


pub fn route_key(url: &str) -> Option<String> {
    let (host, segs) = route_segments(url)?;
    if segs.is_empty() {
        Some(host)
    } else {
        Some(format!("{}/{}", host, segs.join("/")))
    }
}


pub fn route_prefix_keys(url: &str) -> Vec<String> {
    let Some((host, segs)) = route_segments(url) else {
        return Vec::new();
    };
    let mut keys = vec![host.clone()];
    let mut acc = host;
    for s in segs {
        acc.push('/');
        acc.push_str(&s);
        keys.push(acc.clone());
    }
    keys.reverse();
    keys
}


pub fn source_name_from_url(url: &str) -> String {
    let s1 = match url.strip_prefix("https://") {
        Some(s) => s,
        None => url,
    };
    let s2 = match s1.strip_prefix("http://") {
        Some(s) => s,
        None => s1,
    };
    let without_scheme = match s2.strip_prefix("www.") {
        Some(s) => s,
        None => s2,
    };
    let after_domain: Vec<&str> = without_scheme.splitn(2, '/').collect();
    if after_domain.len() < 2 {
        return "index.json".to_string();
    }
    let path_and_query = after_domain[1];
    let cleaned = path_and_query
        .chars()
        .map(|c| match c {
            c if c.is_ascii_alphanumeric()
                || c == '-'
                || c == '.'
                || c == '_'
                || c == '{'
                || c == '}' =>
            {
                c
            }
            '/' | '?' | '&' | '=' => '-',
            _ => '_',
        })
        .collect::<String>();
    if cleaned.is_empty() {
        "index".to_string()
    } else {
        cleaned.trim_matches('-').to_string()
    }
}


pub fn cdn_manifest_for(urls: impl Iterator<Item = String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    for url in urls {
        if url.starts_with("https://github.com/omegaflow/sources") {
            continue;
        }
        let name = source_name_from_url(&url);
        let k = match seen.get_mut(&name) {
            Some(n) => {
                *n += 1;
                *n
            }
            None => {
                seen.insert(name, 1);
                continue;
            }
        };
        map.insert(url, format!("{}-{}", name, k));
    }
    map
}


pub fn cdn_manifest_map() -> &'static HashMap<String, String> {
    static MANIFEST: OnceLock<HashMap<String, String>> = OnceLock::new();
    MANIFEST.get_or_init(|| {
        if let Ok(content) = std::fs::read_to_string("phi/sources.φ") {
            let sources = load_sources_from(&content);
            cdn_manifest_for(sources.iter().map(|s| s.url.clone()))
        } else {
            HashMap::new()
        }
    })
}


pub fn json_has_key_ci(val: &JsonVal, target: &str) -> bool {
    match val {
        JsonVal::Obj(map) => {
            map.keys().any(|k| k.eq_ignore_ascii_case(target))
                || map.values().any(|v| json_has_key_ci(v, target))
        }
        JsonVal::Arr(arr) => arr.iter().any(|v| json_has_key_ci(v, target)),
        _ => false,
    }
}
