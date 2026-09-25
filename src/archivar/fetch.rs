use super::*;

fn append_ca(cmd: &mut Command) {
    let path = match std::env::var("OMEGAFLOW_CA_BUNDLE") {
        Ok(p) if !p.is_empty() && std::path::Path::new(&p).is_file() => p,
        _ => return,
    };
    cmd.arg("--cacert").arg(path);
}

pub const FETCH_BUDGET: usize = 1 << 3;

pub const FETCH_VOID_CAP: u32 = 1 << 2;

pub const FETCH_DURATION_RING: usize = 1 << 4;

pub const CONNECT_BOUND_S: u64 = 1 << 5;

#[derive(Clone, Copy)]
pub enum RetryPolicy {
    Transient,
    All,
}

pub fn ttl_transfer_bound(ttl: u64) -> u64 {
    ((ttl as f64) / (Φ * Φ)).ceil() as u64
}

fn append_retry(cmd: &mut Command, retry: RetryPolicy, attempts: u64) {
    cmd.arg("--retry")
        .arg(attempts.to_string())
        .arg("--retry-delay")
        .arg("2");
    match retry {
        RetryPolicy::Transient => {
            cmd.arg("--retry-connrefused");
        }
        RetryPolicy::All => {
            cmd.arg("--retry-all-errors");
        }
    }
}

pub fn fetch_raw_with(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
    retry: RetryPolicy,
    transfer_bound_s: u64,
) -> Option<String> {
    if url.starts_with("s3://") {
        if body.is_some() {
            return None;
        }
        return super::range::fetch_s3_whole(url, ttl)
            .map(|b| String::from_utf8_lossy(&b).into_owned());
    }
    let connect_t = CONNECT_BOUND_S;
    let mut cmd = Command::new("curl");
    cmd.arg("-s").arg("-S").arg("-f").arg("-L").arg("-g");
    append_retry(&mut cmd, retry, 3);
    cmd.arg("-m")
        .arg(transfer_bound_s.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    if let Some(b) = body {
        cmd.arg("-X").arg("POST");
        cmd.arg("-d").arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    append_ca(&mut cmd);
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

pub fn fetch_raw(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<String> {
    fetch_raw_with(
        url,
        body,
        headers,
        ttl,
        RetryPolicy::Transient,
        ttl_transfer_bound(ttl),
    )
}

pub fn curl_base(retry: RetryPolicy, transfer_bound_s: u64, parallel_max: u8) -> Command {
    let connect_t = CONNECT_BOUND_S;
    let mut cmd = Command::new("curl");
    cmd.arg("-s").arg("-S").arg("-f").arg("-L").arg("-g");
    append_retry(&mut cmd, retry, 5);
    if parallel_max > 0 {
        cmd.arg("--parallel")
            .arg("--parallel-max")
            .arg(parallel_max.to_string());
    }
    cmd.arg("-m")
        .arg(transfer_bound_s.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    append_ca(&mut cmd);
    cmd
}

pub fn fetch_raw_bytes_with(
    url: &str,
    ttl: u64,
    retry: RetryPolicy,
    transfer_bound_s: u64,
) -> Option<Vec<u8>> {
    if url.starts_with("s3://") {
        return super::range::fetch_s3_whole(url, ttl);
    }
    let mut cmd = curl_base(retry, transfer_bound_s, 0);
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

pub fn fetch_raw_bytes(url: &str, ttl: u64) -> Option<Vec<u8>> {
    fetch_raw_bytes_with(url, ttl, RetryPolicy::Transient, ttl_transfer_bound(ttl))
}

pub fn fetch_raw_bytes_headers_with(
    url: &str,
    headers: &[(String, String)],
    ttl: u64,
    retry: RetryPolicy,
    transfer_bound_s: u64,
) -> Option<Vec<u8>> {
    if url.starts_with("s3://") {
        return super::range::fetch_s3_whole(url, ttl);
    }
    let mut cmd = curl_base(retry, transfer_bound_s, 0);
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
            "\r\x1b[Kfetch_bytes_headers returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

pub fn fetch_raw_bytes_headers(
    url: &str,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<Vec<u8>> {
    fetch_raw_bytes_headers_with(
        url,
        headers,
        ttl,
        RetryPolicy::Transient,
        ttl_transfer_bound(ttl),
    )
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
    append_ca(&mut cmd);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        None
    }
}

pub fn fetch_raw_bytes_post_with(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    retry: RetryPolicy,
    transfer_bound_s: u64,
) -> Option<Vec<u8>> {
    let connect_t = CONNECT_BOUND_S;
    let mut cmd = Command::new("curl");
    cmd.arg("-s").arg("-S").arg("-f").arg("-L");
    append_retry(&mut cmd, retry, 5);
    cmd.arg("-m")
        .arg(transfer_bound_s.to_string())
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
    append_ca(&mut cmd);
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

pub fn fetch_raw_bytes_post(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
) -> Option<Vec<u8>> {
    fetch_raw_bytes_post_with(
        url,
        body,
        headers,
        RetryPolicy::Transient,
        ttl_transfer_bound(ttl),
    )
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

pub fn stale_after(fetched: f64, failures: u32, ttl: u64, now: f64) -> bool {
    now - fetched >= (ttl as f64 / Φ) * (2f64).powi(failures.min(FETCH_VOID_CAP) as i32)
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
            let jumped = match jump_epoch {
                Some(j) => o.fetched < j,
                None => false,
            };
            !o.in_flight && (stale_after(o.fetched, o.failures, ttl, now) || jumped)
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

pub fn load_origin_clock(path: &str) -> HashMap<String, (f64, u32)> {
    let mut clock = HashMap::new();
    let Ok(content) = std::fs::read_to_string(path) else {
        return clock;
    };
    let mut lines = content.lines();
    match lines.next() {
        Some("origin_clock 1") => {}
        _ => return clock,
    }
    for line in lines {
        let mut parts = line.splitn(3, ' ');
        let Some(fetched) = parts.next().and_then(|p| p.parse::<f64>().ok()) else {
            continue;
        };
        let Some(failures) = parts.next().and_then(|p| p.parse::<u32>().ok()) else {
            continue;
        };
        let Some(url) = parts.next() else {
            continue;
        };
        clock.insert(url.to_string(), (fetched, failures));
    }
    clock
}

pub fn save_origin_clock(path: &str, clock: &HashMap<String, (f64, u32)>) {
    let mut out = String::from("origin_clock 1\n");
    for (url, (fetched, failures)) in clock {
        out.push_str(&format!("{fetched} {failures} {url}\n"));
    }
    if std::fs::write(path, out).is_err() {
        eprintln!(
            "origin clock {}: write void — the clock starts absent next run",
            path
        );
    }
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

type PresenceSample = (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);

pub fn presence_gate(
    presences: &[PresenceSample],
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

pub struct EnclosureField<'a> {
    pub config: &'a FieldConfig,
    pub body_props: Option<&'a BodyProperties>,
    pub body_radius: Option<f64>,
}

pub struct AnchorEnvelope {
    pub vmax: f64,
    pub amax: f64,
    pub pad: f64,
    pub ttl: f64,
}

pub fn record_in_enclosure(
    presences: &[PresenceSample],
    p_r: Option<[f64; 3]>,
    t_r: f64,
    now: f64,
    field: EnclosureField<'_>,
    env: AnchorEnvelope,
) -> bool {
    let fc = field.config;
    let body_props = field.body_props;
    let body_radius = field.body_radius;
    let anchor_vmax = env.vmax;
    let anchor_amax = env.amax;
    let pad = env.pad;
    let effective_ttl = env.ttl;
    let Some(p_r) = p_r else {
        return true;
    };
    let age = (now - t_r).abs();
    if age > effective_ttl * 64.0 {
        return false;
    }
    let Some(reach_signal) =
        signal_reach(fc.force as f64, fc.advection, age, fc.freq, fc.bin_width)
    else {
        return true;
    };
    let extent = kernel_extent(fc.force, fc.kernel, body_props, fc.tau);
    let rho = anchor_vmax * age + 0.5 * anchor_amax * age * age + pad;
    presences
        .iter()
        .any(|&(_, px, py, pz, _range, vx, vy, vz, _thrust, grid_step)| {
            let v_abs = (vx * vx + vy * vy + vz * vz).sqrt();
            let body_term = match body_radius {
                Some(r) => r.max(Φ * grid_step),
                None => Φ * grid_step,
            };
            let limit = reach_signal + extent + rho + v_abs * age + body_term;
            let dx = p_r[0] - px;
            let dy = p_r[1] - py;
            let dz = p_r[2] - pz;
            (dx * dx + dy * dy + dz * dz).sqrt() <= limit
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
                        let first = match jpath_val(&j, arr_path) {
                            Some(JsonVal::Arr(arr)) => {
                                if !arr.is_empty() {
                                    arr_has_rows = true;
                                }
                                arr.first()
                            }
                            Some(obj @ JsonVal::Obj(_)) => Some(obj),
                            _ => None,
                        };
                        if let Some(row) = first {
                            for fk in [lat_key.as_str(), lon_key.as_str()] {
                                if jpath_val(row, fk).is_some() {
                                    key_found = true;
                                }
                            }
                            for fc in fields {
                                if jpath_val(row, &fc.key).is_some() {
                                    key_found = true;
                                }
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
                        if let Some(JsonVal::Arr(arr)) = jpath_val(&j, arr_path)
                            && !arr.is_empty()
                        {
                            arr_has_rows = true;
                        }
                        for fc in fields {
                            if jpath_val(&j, &fc.key).is_some() {
                                key_found = true;
                            }
                        }
                    }
                    Extract::Rows { .. }
                    | Extract::GeojsonEvents { .. }
                    | Extract::QuakeMlEvents { .. }
                    | Extract::Hapi(_) => {
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
    Refused,
    Broken,
}

impl VoidClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoidClass::Key => "key-void",
            VoidClass::Drift => "drift-void",
            VoidClass::Quiet => "quiet-void",
            VoidClass::Refused => "refused",
            VoidClass::Broken => "broken",
        }
    }
}

pub struct VoidFinding {
    pub url: String,
    pub class: VoidClass,
    pub detail: String,
}

pub fn void_class(diag: &str) -> VoidClass {
    if diag.contains("all containers empty") || diag.contains("empty body") {
        VoidClass::Quiet
    } else {
        VoidClass::Drift
    }
}

pub fn carries_coord_marker(url: &str) -> bool {
    COORD_MARKERS.iter().any(|m| url.contains(m))
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

pub fn month_abbr(m: u32) -> &'static str {
    [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ][(m - 1) as usize]
}

fn minute_iso(unix: u64) -> String {
    let (y, m, d) = civil_date(unix);
    let h = (unix / 3600) % 24;
    let min = (unix / 60) % 60;
    format!("{}-{:02}-{:02}T{:02}:{:02}:00", y, m, d, h, min)
}

fn day_of_year(y: i64, m: u32, d: u32) -> u32 {
    let cum = [0u32, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let base = if m > 0 { cum[(m - 1) as usize] } else { 0 };
    base + d + if leap && m > 2 { 1 } else { 0 }
}

pub fn time_markers(unix: u64) -> Vec<(&'static str, String)> {
    let (y, m, d) = civil_date(unix);
    let (yy, ym, yd) = civil_date(unix.saturating_sub(86400));
    let (tmy, tmm, tmd) = civil_date(unix.saturating_add(86400));
    let (wy, wm, wd) = civil_date(unix.saturating_sub(7 * 86400));
    let prev_y = civil_date(unix.saturating_sub(366 * 86400)).0;
    let yday = day_of_year(y, m, d);
    vec![
        ("{today}", date_str(unix)),
        ("{yesterday}", date_str(unix.saturating_sub(86400))),
        ("{tomorrow}", date_str(unix.saturating_add(86400))),
        ("{prev_year}", format!("{:04}", prev_y)),
        ("{week_ago}", format!("{}-{:02}-{:02}", wy, wm, wd)),
        ("{week_ago_nodashes}", format!("{}{:02}{:02}", wy, wm, wd)),
        ("{now}", minute_iso(unix)),
        ("{now_minus_1}", minute_iso(unix.saturating_sub(60))),
        ("{now_minus_2}", minute_iso(unix.saturating_sub(120))),
        ("{hour_ago}", minute_iso(unix.saturating_sub(3600))),
        ("{year}", y.to_string()),
        ("{year2}", format!("{:02}", y % 100)),
        ("{month}", m.to_string()),
        (
            "{prev_Mon}",
            month_abbr(if m == 1 { 12 } else { m - 1 }).to_string(),
        ),
        ("{day}", d.to_string()),
        ("{yday}", format!("{:03}", yday)),
        ("{hour}", format!("{:02}", (unix / 3600) % 24)),
        ("{minute}", format!("{:02}", (unix / 60) % 60)),
        ("{unix_now}", unix.to_string()),
        (
            "{unix_now_plus_3600}",
            (unix.saturating_add(3600)).to_string(),
        ),
        ("{today_yyyymmdd}", format!("{}_{:02}_{:02}", y, m, d)),
        ("{today_ymd}", format!("{}_{:02}_{:02}", y, m, d)),
        ("{today_nodashes}", format!("{}{:02}{:02}", y, m, d)),
        ("{yesterday_nodashes}", format!("{}{:02}{:02}", yy, ym, yd)),
        (
            "{tomorrow_nodashes}",
            format!("{}{:02}{:02}", tmy, tmm, tmd),
        ),
        ("{t_start}", date_str(unix.saturating_sub(86400))),
        ("{t_end}", date_str(unix)),
        ("{today_plus_365}", format!("{}-{:02}-{:02}", y + 1, m, d)),
    ]
}

pub fn live_markers() -> Vec<(String, String)> {
    let unix = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => return Vec::new(),
    };
    let jd = unix as f64 / 86400.0 + 2440587.5;
    let mut out: Vec<(String, String)> = time_markers(unix)
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    out.push(("{jd_now}".into(), format!("{:.6}", jd)));
    out.push(("{jd_start}".into(), format!("{:.6}", jd - 1.0)));
    out.push(("{jd_end}".into(), format!("{:.6}", jd)));
    out
}

pub fn unresolved_key(template: &str, env: &HashMap<String, String>) -> Option<String> {
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let end = rest.find('}')?;
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
        if s.fanout_cap > 0
            || s.format == "csv_zip"
            || s.format == "igra_zip"
            || s.format == "kernel_text"
        {
            continue;
        }
        if matches!(
            s.format.as_str(),
            "ephemeris_binary"
                | "orbit_bin"
                | "catalog_dastcom"
                | "netcdf"
                | "opendap"
                | "finals"
                | "ionex"
                | "rinex"
                | "cors_rinex"
                | "alerce"
                | "catalog_tycho"
                | "spectral"
                | "xp_spectra"
                | "jwst_spectra"
                | "bl_narrowband"
                | "lightcurve"
                | "rpw_efield"
                | "goes_xrs"
                | "wind_waves"
                | "gong_modes"
                | "spk"
                | "reference"
                | "tar_gz_yaml"
                | "flac"
        ) {
            continue;
        }
        if carries_coord_marker(&s.url) {
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
                let (class, detail) = match http_code(&url, &headers) {
                    Some(code) => (
                        VoidClass::Refused,
                        format!(
                            "host answers http {} but body unusable (fetch void) — refused, not dead",
                            code
                        ),
                    ),
                    None => (
                        VoidClass::Broken,
                        "host unreachable (dns/connect/timeout) — dead candidate".into(),
                    ),
                };
                findings.push(VoidFinding {
                    url: s.url.clone(),
                    class,
                    detail,
                });
                continue;
            }
        };
        match extract(s, &body, now, lsk) {
            ExtractResult::Measurements(v) | ExtractResult::WithEphemeris(v, _) => {
                if v.is_empty() {
                    let diag = diagnose_no_samples(s, &body);
                    let class = void_class(&diag);
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
    pub sample_ttl_override: Option<f64>,
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

pub fn http_code(url: &str, headers: &[(String, String)]) -> Option<u16> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}")
        .arg("-L")
        .arg("-g")
        .arg("-m")
        .arg("20")
        .arg("--connect-timeout")
        .arg("10");
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    append_ca(&mut cmd);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout);
    s.trim().parse::<u16>().ok().filter(|c| *c > 0)
}

fn cdn_last_modified(url: &str) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-I")
        .arg("-L")
        .arg("-m")
        .arg(CONNECT_BOUND_S.to_string());
    append_ca(&mut cmd);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let head = String::from_utf8_lossy(&output.stdout).to_string();
    extract_header(&head, "last-modified")
}

pub fn cdn_last_modified_age(url: &str) -> Option<u64> {
    let lm = cdn_last_modified(url)?;
    let asset_ts = rfc1123_to_unix(&lm)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    Some(now.as_secs().saturating_sub(asset_ts))
}

pub fn cdn_fresh_age(cdn_url: &str, ttl: u64) -> Option<u64> {
    cdn_last_modified_age(cdn_url).filter(|age| *age < ttl)
}

pub fn cdn_fresh(cdn_url: &str, ttl: u64) -> bool {
    cdn_fresh_age(cdn_url, ttl).is_some()
}

pub fn url_has_fixed_window(template: &str) -> bool {
    let b = template.as_bytes();
    let mut i = 0;
    while i + 10 <= b.len() {
        if b[i].is_ascii_digit()
            && b[i + 1].is_ascii_digit()
            && b[i + 2].is_ascii_digit()
            && b[i + 3].is_ascii_digit()
            && b[i + 4] == b'-'
            && b[i + 5].is_ascii_digit()
            && b[i + 6].is_ascii_digit()
            && b[i + 7] == b'-'
            && b[i + 8].is_ascii_digit()
            && b[i + 9].is_ascii_digit()
        {
            return true;
        }
        i += 1;
    }
    false
}

pub fn fetch_one_with_age(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
    now: Option<f64>,
    live_only: bool,
) -> Option<(String, Option<u64>)> {
    if live_only {
        return fetch_raw(url, body, headers, ttl).map(|l| (l, None));
    }
    let manifest = cdn_manifest_map();
    let asset_name = |u: &str| -> String {
        match manifest.get(u) {
            Some(name) => name.clone(),
            None => source_name_from_url(u),
        }
    };
    if !url.starts_with("https://github.com/omegaflow/sources")
        && let Some(netloc) = extract_netloc(url)
    {
        let name = asset_name(url);
        if !name.is_empty() {
            let cache_path = cache_path_for(netloc, &name);
            if now.is_some_and(|n| cache_fresh_at(&cache_path, ttl, n))
                && let Ok(cached) = std::fs::read_to_string(&cache_path)
            {
                return Some((cached, None));
            }
            let cdn_url = format!("{}/{}/{}.json", crate::cdn::cdn_base(), netloc, name);
            if let Some(age) = cdn_fresh_age(&cdn_url, ttl)
                && let Some(cdn_body) = fetch_raw(&cdn_url, None, &[], ttl)
            {
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
                return Some((cdn_body, Some(age)));
            }
        }
    }
    let live = fetch_raw(url, body, headers, ttl);
    if live.is_none()
        && !url.starts_with("https://github.com/omegaflow/sources")
        && let Some(netloc) = extract_netloc(url)
    {
        let name = asset_name(url);
        if !name.is_empty() {
            let cdn_url = format!("{}/{}/{}.json", crate::cdn::cdn_base(), netloc, name);
            if let Some(cdn_body) = fetch_raw(&cdn_url, None, &[], ttl) {
                return Some((cdn_body, cdn_last_modified_age(&cdn_url)));
            }
        }
    }
    if let Some(ref r) = live
        && let Some(netloc) = extract_netloc(url)
    {
        let name = asset_name(url);
        if !name.is_empty() {
            let cache_path = cache_path_for(netloc, &name);
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
    live.map(|l| (l, None))
}

pub fn fetch_one(
    url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
    ttl: u64,
    now: Option<f64>,
) -> Option<String> {
    fetch_one_with_age(url, body, headers, ttl, now, false).map(|(b, _)| b)
}

pub fn cache_root() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir).join("archivar_cache");
    }
    std::path::PathBuf::from("cache")
}

pub fn cache_path_for(netloc: &str, name: &str) -> String {
    cache_root()
        .join(netloc)
        .join(format!("{name}.json"))
        .to_string_lossy()
        .into_owned()
}

pub fn content_cache(name: &str) -> String {
    let root = cache_root();
    if let Some(parent) = root.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    root.join(name).to_string_lossy().into_owned()
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

fn cdn_stamp_path(path: &str) -> String {
    format!("{path}.cdn")
}

pub fn write_cdn_stamp(path: &str, url: &str) {
    let Some(lm) = cdn_last_modified(url) else {
        return;
    };
    if std::fs::write(cdn_stamp_path(path), lm).is_err() {
        eprintln!("cache {}: cdn stamp write void — recheck next cycle", path);
    }
}

pub fn cache_fresh_cdn(path: &str, ttl: u64, url: &str) -> bool {
    if !cache_fresh(path, ttl) {
        return false;
    }
    if !url.contains("/releases/download/") {
        return true;
    }
    let Some(current) = cdn_last_modified(url) else {
        return true;
    };
    match std::fs::read_to_string(cdn_stamp_path(path)) {
        Ok(stamp) => stamp.trim() == current.trim(),
        Err(_) => false,
    }
}

pub fn machine_now_tdb() -> Option<f64> {
    embedded_lsk().and_then(|l| l.system_now_tdb())
}

pub fn is_leap(y: u32) -> bool {
    (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400)
}

pub fn load_env() -> HashMap<String, String> {
    let mut env: HashMap<String, String> = std::env::vars().collect();
    for name in &[".env", ".secrets.local"] {
        if let Ok(content) = std::fs::read_to_string(state_asset(name)) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some(eq) = line.find('=') {
                    let key = line[..eq].trim().to_string();
                    let val = line[eq + 1..].trim().to_string();
                    env.entry(key).or_insert(val);
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

pub fn frame_anchor(frame: &Frame) -> Option<(f64, f64)> {
    match frame {
        Frame::Surface { lat, lon, .. } => Some((*lat, *lon)),
        _ => None,
    }
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

#[cfg(test)]
mod cache_root_tests {
    use super::*;

    #[test]
    fn cache_root_resolves_and_serves() {
        let state = "/tmp/opencode/omegaflow_cache_root_state";
        let state_root = std::path::PathBuf::from(state).join("archivar_cache");

        unsafe {
            std::env::set_var("OMEGAFLOW_STATE", state);
        }
        assert_eq!(
            cache_root(),
            state_root,
            "the cache root honors OMEGAFLOW_STATE"
        );

        unsafe {
            std::env::remove_var("OMEGAFLOW_STATE");
            std::env::set_var("HOME", "/home/probe");
        }
        assert_eq!(
            cache_root(),
            std::path::PathBuf::from("cache"),
            "without OMEGAFLOW_STATE the cache root lives in the folder"
        );

        unsafe {
            std::env::set_var("OMEGAFLOW_STATE", state);
        }
        let path = cache_path_for("example.com", "sample_source");
        assert_eq!(
            path,
            state_root
                .join("example.com/sample_source.json")
                .to_string_lossy(),
            "the path joins netloc and name under the state root"
        );
        let parent = std::path::Path::new(&path).parent().unwrap().to_path_buf();
        std::fs::create_dir_all(&parent).unwrap();
        let body = "{\"measured\":true}";
        std::fs::write(&path, body).unwrap();
        let stamp_path = format!("{path}.epoch");
        std::fs::write(&stamp_path, "8.0e8").unwrap();
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            body,
            "the written cache reads back intact"
        );
        assert!(
            cache_fresh_at(&path, 3600, 8.0e8 + 100.0),
            "the freshly written cache serves within its ttl"
        );
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(&stamp_path);
        unsafe {
            std::env::remove_var("OMEGAFLOW_STATE");
        }
    }
}

#[cfg(test)]
mod cdn_cache_tests {
    use super::*;

    #[test]
    fn cache_fresh_cdn_falls_back_to_mtime_for_non_cdn_urls() {
        let path = std::env::temp_dir().join("omegaflow_cdn_fallback_test.bin");
        let path = path.to_str().unwrap();
        let _ = std::fs::remove_file(path);
        assert!(
            !cache_fresh_cdn(path, 3600, "https://example.com/plain.bin"),
            "an absent cache is not fresh"
        );
        std::fs::write(path, b"bytes").unwrap();
        assert!(
            cache_fresh_cdn(path, 3600, "https://example.com/plain.bin"),
            "a freshly written non-cdn cache serves within ttl"
        );
        assert!(
            !cache_fresh_cdn(path, 0, "https://example.com/plain.bin"),
            "a zero ttl closes the gate"
        );
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(test)]
mod ca_bundle_tests {
    use super::*;

    #[test]
    fn append_ca_reads_the_configured_bundle() {
        let path = std::env::temp_dir().join("omegaflow_ca_bundle_test.pem");
        let path = path.to_str().unwrap();
        std::fs::write(path, b"-----BEGIN CERTIFICATE-----\n").unwrap();
        unsafe {
            std::env::set_var("OMEGAFLOW_CA_BUNDLE", path);
        }
        let mut cmd = Command::new("curl");
        append_ca(&mut cmd);
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert_eq!(args, vec!["--cacert".to_string(), path.to_string()]);
        unsafe {
            std::env::set_var("OMEGAFLOW_CA_BUNDLE", "/nonexistent/bundle.pem");
        }
        let mut cmd = Command::new("curl");
        append_ca(&mut cmd);
        assert_eq!(cmd.get_args().count(), 0);
        unsafe {
            std::env::remove_var("OMEGAFLOW_CA_BUNDLE");
        }
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(test)]
mod marker_tests {
    use super::*;

    #[test]
    fn month_abbr_names_all_twelve_months() {
        assert_eq!(month_abbr(1), "Jan");
        assert_eq!(month_abbr(8), "Aug");
        assert_eq!(month_abbr(12), "Dec");
    }

    #[test]
    fn prev_mon_marker_is_the_previous_calendar_month() {
        let markers = live_markers();
        let cur: u32 = markers
            .iter()
            .find(|(k, _)| k == "{month}")
            .map(|(_, v)| v.parse().unwrap())
            .unwrap();
        let prev = markers
            .iter()
            .find(|(k, _)| k == "{prev_Mon}")
            .map(|(_, v)| v.as_str())
            .unwrap();
        assert_eq!(prev, month_abbr(if cur == 1 { 12 } else { cur - 1 }));
    }

    #[test]
    fn dart_sources_are_registered() {
        let srcs = crate::archivar::load_sources();
        assert!(
            srcs.iter()
                .any(|s| s.url == "https://www.ndbc.noaa.gov/data/realtime2/21414.dart"),
            "live dart source absent"
        );
        assert!(
            srcs.iter()
                .any(|s| s.url == "https://www.ndbc.noaa.gov/data/dart/{prev_Mon}/21414.txt"),
            "archive dart source absent"
        );
    }
}
