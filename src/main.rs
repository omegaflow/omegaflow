#![allow(mixed_script_confusables)]
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const Φ: f64 = 1.618033988749895;
const AU: f64 = 1.495978707e11;
const EARTH_RADIUS: f64 = 6378137.0;
const EARTH_ECC: f64 = 0.0167086;
const ECLIPTIC_OBLIQUITY: f64 = 0.409092804;
const J2000_EPOCH: f64 = 2451545.0;
const UNIX_J2000_OFFSET: f64 = 946728000.0;

fn compute_gmst(tdb_secs: f64) -> f64 {
    let jd = tdb_secs / 86400.0 + J2000_EPOCH;
    let t = (jd - J2000_EPOCH) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - J2000_EPOCH) + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    (gmst % 360.0) * std::f64::consts::PI / 180.0
}

fn geodetic_to_ecef(lat: f64, lon: f64, alt: f64) -> (f64, f64, f64) {
    let lat_r = lat * std::f64::consts::PI / 180.0;
    let lon_r = lon * std::f64::consts::PI / 180.0;
    const WGS84_F: f64 = 1.0 / 298.257223563;
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let sin_lat = lat_r.sin();
    let n = EARTH_RADIUS / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    (
        (n + alt) * lat_r.cos() * lon_r.cos(),
        (n + alt) * lat_r.cos() * lon_r.sin(),
        (n * (1.0 - e2) + alt) * sin_lat,
    )
}

fn tdb_to_jd(tdb_secs: f64) -> f64 {
    tdb_secs / 86400.0 + J2000_EPOCH
}

fn earth_position_icrs(tdb_secs: f64) -> (f64, f64, f64) {
    let jd = tdb_to_jd(tdb_secs);
    let t = (jd - J2000_EPOCH) / 36525.0;
    let m = 6.239996 + 0.017201969 * t * 36525.0;
    let e = EARTH_ECC;
    let mut e_anom = m;
    for _ in 0..5 {
        e_anom = e_anom - (e_anom - e * e_anom.sin() - m) / (1.0 - e * e_anom.cos());
    }
    let x_orb = AU * (e_anom.cos() - e);
    let y_orb = AU * (1.0 - e * e).sqrt() * e_anom.sin();
    let omega: f64 = -0.113;
    let x_ecl = x_orb * omega.cos() - y_orb * omega.sin();
    let y_ecl = x_orb * omega.sin() + y_orb * omega.cos();
    let x_icrs = x_ecl;
    let y_icrs = y_ecl * ECLIPTIC_OBLIQUITY.cos();
    let z_icrs = y_ecl * ECLIPTIC_OBLIQUITY.sin();
    (x_icrs, y_icrs, z_icrs)
}

fn geodetic_to_icrs(lat: f64, lon: f64, alt: f64, tdb_secs: f64) -> (f64, f64, f64) {
    let (x_ecef, y_ecef, z_ecef) = geodetic_to_ecef(lat, lon, alt);
    let gmst_rad = compute_gmst(tdb_secs);
    let x_eci = x_ecef * gmst_rad.cos() + y_ecef * gmst_rad.sin();
    let y_eci = -x_ecef * gmst_rad.sin() + y_ecef * gmst_rad.cos();
    let z_eci = z_ecef;
    let x_ecl = x_eci;
    let y_ecl = y_eci * ECLIPTIC_OBLIQUITY.cos() + z_eci * ECLIPTIC_OBLIQUITY.sin();
    let z_ecl = -y_eci * ECLIPTIC_OBLIQUITY.sin() + z_eci * ECLIPTIC_OBLIQUITY.cos();
    let (ex, ey, ez) = earth_position_icrs(tdb_secs);
    (x_ecl + ex, y_ecl + ey, z_ecl + ez)
}

fn icrs_to_geodetic(x: f64, y: f64, z: f64, tdb_secs: f64) -> (f64, f64) {
    let (ex, ey, ez) = earth_position_icrs(tdb_secs);
    let x_ecl = x - ex;
    let y_ecl = y - ey;
    let z_ecl = z - ez;
    let x_eci = x_ecl;
    let y_eci = y_ecl * ECLIPTIC_OBLIQUITY.cos() - z_ecl * ECLIPTIC_OBLIQUITY.sin();
    let z_eci = y_ecl * ECLIPTIC_OBLIQUITY.sin() + z_ecl * ECLIPTIC_OBLIQUITY.cos();
    let jd = tdb_to_jd(tdb_secs);
    let t = (jd - J2000_EPOCH) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - J2000_EPOCH) + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    let gmst_rad = (gmst % 360.0) * std::f64::consts::PI / 180.0;
    let x_ecef = x_eci * gmst_rad.cos() - y_eci * gmst_rad.sin();
    let y_ecef = x_eci * gmst_rad.sin() + y_eci * gmst_rad.cos();
    let z_ecef = z_eci;
    let lon = y_ecef.atan2(x_ecef).to_degrees();
    let lat = z_ecef
        .atan2((x_ecef * x_ecef + y_ecef * y_ecef).sqrt())
        .to_degrees();
    (lat, lon)
}

#[derive(Clone, Debug)]
pub enum JsonVal {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<JsonVal>),
    Obj(HashMap<String, JsonVal>),
}

pub fn parse_json(s: &str) -> Option<JsonVal> {
    let mut p = JsonParser {
        chars: s.as_bytes(),
        pos: 0,
    };
    p.skip_ws();
    p.parse_value()
}

struct JsonParser<'a> {
    chars: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn skip_ws(&mut self) {
        while self.pos < self.chars.len() && (self.chars[self.pos] as char).is_whitespace() {
            self.pos += 1;
        }
    }
    fn parse_value(&mut self) -> Option<JsonVal> {
        self.skip_ws();
        if self.pos >= self.chars.len() {
            return None;
        }
        match self.chars[self.pos] {
            b'{' => self.parse_obj(),
            b'[' => self.parse_arr(),
            b'"' => self.parse_str().map(JsonVal::Str),
            b't' => {
                self.pos += 4;
                Some(JsonVal::Bool(true))
            }
            b'f' => {
                self.pos += 5;
                Some(JsonVal::Bool(false))
            }
            b'n' => {
                self.pos += 4;
                Some(JsonVal::Null)
            }
            _ => self.parse_num(),
        }
    }
    fn parse_obj(&mut self) -> Option<JsonVal> {
        self.pos += 1;
        self.skip_ws();
        let mut map = HashMap::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b'}' {
            self.pos += 1;
            return Some(JsonVal::Obj(map));
        }
        loop {
            self.skip_ws();
            let key = self.parse_str()?;
            self.skip_ws();
            if self.pos >= self.chars.len() || self.chars[self.pos] != b':' {
                return None;
            }
            self.pos += 1;
            let val = self.parse_value()?;
            map.insert(key, val);
            self.skip_ws();
            if self.pos >= self.chars.len() {
                return None;
            }
            match self.chars[self.pos] {
                b',' => {
                    self.pos += 1;
                }
                b'}' => {
                    self.pos += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(JsonVal::Obj(map))
    }
    fn parse_arr(&mut self) -> Option<JsonVal> {
        self.pos += 1;
        self.skip_ws();
        let mut arr = Vec::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b']' {
            self.pos += 1;
            return Some(JsonVal::Arr(arr));
        }
        loop {
            let val = self.parse_value()?;
            arr.push(val);
            self.skip_ws();
            if self.pos >= self.chars.len() {
                return None;
            }
            match self.chars[self.pos] {
                b',' => {
                    self.pos += 1;
                }
                b']' => {
                    self.pos += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(JsonVal::Arr(arr))
    }
    fn parse_str(&mut self) -> Option<String> {
        if self.pos >= self.chars.len() || self.chars[self.pos] != b'"' {
            return None;
        }
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == b'\\' && self.pos + 1 < self.chars.len() {
                self.pos += 1;
                match self.chars[self.pos] {
                    b'"' => {
                        s.push('"');
                        self.pos += 1;
                    }
                    b'\\' => {
                        s.push('\\');
                        self.pos += 1;
                    }
                    b'/' => {
                        s.push('/');
                        self.pos += 1;
                    }
                    b'n' => {
                        s.push('\n');
                        self.pos += 1;
                    }
                    b't' => {
                        s.push('\t');
                        self.pos += 1;
                    }
                    b'r' => {
                        s.push('\r');
                        self.pos += 1;
                    }
                    b'u' => {
                        self.pos += 1;
                        if self.pos + 4 <= self.chars.len() {
                            if let Ok(hex) =
                                std::str::from_utf8(&self.chars[self.pos..self.pos + 4])
                            {
                                if let Ok(cp) = u32::from_str_radix(hex, 16) {
                                    if let Some(ch) = char::from_u32(cp) {
                                        s.push(ch);
                                    }
                                }
                            }
                            self.pos += 4;
                        }
                    }
                    _ => {
                        self.pos += 1;
                    }
                }
            } else if c == b'"' {
                self.pos += 1;
                return Some(s);
            } else {
                s.push(c as char);
                self.pos += 1;
            }
        }
        None
    }
    fn parse_num(&mut self) -> Option<JsonVal> {
        let start = self.pos;
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c.is_ascii_digit() || c == b'-' || c == b'+' || c == b'.' || c == b'e' || c == b'E' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let s = std::str::from_utf8(&self.chars[start..self.pos]).ok()?;
        s.parse::<f64>().ok().map(JsonVal::Num)
    }
}

fn scalar_of(v: &JsonVal) -> Option<f64> {
    match v {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn jpath_val<'a>(json: &'a JsonVal, path: &str) -> Option<&'a JsonVal> {
    let mut current = json;
    for part in path.split('.') {
        if let Ok(idx) = part.parse::<usize>() {
            if let JsonVal::Arr(arr) = current {
                current = arr.get(idx)?;
            } else {
                return None;
            }
        } else {
            if let JsonVal::Obj(map) = current {
                current = map.get(part)?;
            } else {
                return None;
            }
        }
    }
    Some(current)
}

fn jnum(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        return jpath_val(json, key).and_then(scalar_of);
    }
    match json {
        JsonVal::Obj(map) => map.get(key).and_then(scalar_of),
        _ => None,
    }
}

fn jpath(json: &JsonVal, path: &str) -> Option<f64> {
    jpath_val(json, path).and_then(scalar_of)
}

fn jdeep_find_num(json: &JsonVal, key: &str) -> Option<f64> {
    match json {
        JsonVal::Obj(map) => {
            if let Some(v) = map.get(key) {
                if let Some(n) = scalar_of(v) {
                    return Some(n);
                }
            }
            for v in map.values() {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        JsonVal::Arr(arr) => {
            for v in arr {
                if let Some(n) = jdeep_find_num(v, key) {
                    return Some(n);
                }
            }
            None
        }
        _ => None,
    }
}

fn jcount(json: &JsonVal, path: &str) -> Option<f64> {
    if path == "." || path.is_empty() {
        if let JsonVal::Arr(arr) = json {
            return Some(arr.len() as f64);
        }
        return None;
    }
    if path.contains('.') {
        let target = jpath_val(json, path)?;
        if let JsonVal::Arr(arr) = target {
            return Some(arr.len() as f64);
        }
        return None;
    }
    match json {
        JsonVal::Obj(map) => {
            if let Some(JsonVal::Arr(arr)) = map.get(path) {
                Some(arr.len() as f64)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn jfirst(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        let target_path = key.rsplit_once('.').map(|(p, _)| p).unwrap_or("");
        let final_key = key.rsplit_once('.').map(|(_, k)| k).unwrap_or(key);
        let parent = if target_path.is_empty() {
            json
        } else {
            jpath_val(json, target_path)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.first().and_then(|v| {
                if let JsonVal::Obj(o) = v {
                    o.get(final_key).and_then(scalar_of)
                } else {
                    scalar_of(v)
                }
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.first().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.first().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}

fn jlast(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        let target_path = key.rsplit_once('.').map(|(p, _)| p).unwrap_or("");
        let final_key = key.rsplit_once('.').map(|(_, k)| k).unwrap_or(key);
        let parent = if target_path.is_empty() {
            json
        } else {
            jpath_val(json, target_path)?
        };
        if let JsonVal::Arr(arr) = parent {
            return arr.last().and_then(|v| {
                if let JsonVal::Obj(o) = v {
                    o.get(final_key).and_then(scalar_of)
                } else {
                    scalar_of(v)
                }
            });
        }
        return None;
    }
    match json {
        JsonVal::Arr(arr) => arr.last().and_then(|v| match v {
            JsonVal::Obj(o) => o.get(key).and_then(scalar_of),
            other => scalar_of(other),
        }),
        JsonVal::Obj(map) => map.get(key).and_then(|v| {
            if let JsonVal::Arr(a) = v {
                a.last().and_then(scalar_of)
            } else {
                None
            }
        }),
        _ => None,
    }
}

fn extract_regex_val(body: &str, pat: &str) -> Option<f64> {
    let pat_start = pat.find('(')? + 1;
    let pat_end = pat.rfind(')')?;
    if pat_start >= pat_end {
        return None;
    }
    let inner_pat = &pat[pat_start..pat_end];
    let (prefix, suffix) = inner_pat.split_once("...").unwrap_or((inner_pat, ""));

    let p_start = body.find(prefix)?;
    let val_start = p_start + prefix.len();
    let remainder = &body[val_start..];
    let val_end = if suffix.is_empty() {
        remainder
            .find(|c: char| c.is_whitespace() || c == '<' || c == '"')
            .unwrap_or(remainder.len())
    } else {
        remainder.find(suffix).unwrap_or(remainder.len())
    };
    remainder[..val_end].trim().parse::<f64>().ok()
}

#[derive(Clone)]
enum Extract {
    Field(String, String),
    First(String, String),
    Last(String, String),
    Count(String, String),
    LastRow(String, String),
    LastObj(String, String, String, String),
    GeojsonEvents {
        mag_key: String,
        min_mag: f64,
        outputs: Vec<String>,
    },
    Path(String, String),
    Deep(String, String),
    Map {
        arr_path: String,
        lat_key: String,
        lon_key: String,
        alt_key: String,
        fields: Vec<(String, String)>,
    },
    Sum(String, String),
    Regex(String, String),
    XmlCount(String, String),
    Vector(String),
    Ephemeris(String),
}

struct SourceConfig {
    ttl: u64,
    url: String,
    lat: Option<f64>,
    lon: Option<f64>,
    res: i32,
    format: String,
    extracts: Vec<Extract>,
    headers: Vec<(String, String)>,
}

fn res_sq(res: i32) -> f64 {
    let res_m = EARTH_RADIUS * (std::f64::consts::PI / 180.0) / 10f64.powi(res);
    res_m * res_m
}

fn wgs84_key(lat: f64, lon: f64, res: i32) -> String {
    let r = res.max(0) as usize;
    format!(
        "WGS84_{}_{}",
        format!("{:.*}", r, lat),
        format!("{:.*}", r, lon)
    )
}

fn icrs_key(x: f64, y: f64, z: f64, t: f64) -> String {
    format!("ICRS_{}_{}_{}_{}", x as i64, y as i64, z as i64, t as i64)
}

struct Archive {
    sources: Vec<SourceConfig>,
    index_html: Vec<u8>,
    constants_js: Vec<u8>,
    gpu_worker_js: Vec<u8>,
    data_cache: Mutex<HashMap<String, (f64, HashMap<String, (f64, f64, f64, f64, f64)>)>>,
    active_positions: Mutex<HashMap<String, (f64, f64, f64, f64)>>,
    source_positions: Mutex<HashMap<usize, (f64, f64, f64, String)>>,
}
struct WsFrame {
    opcode: u8,
    payload: Vec<u8>,
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = String::new();
    for c in data.chunks(3) {
        let b = [c[0], *c.get(1).unwrap_or(&0), *c.get(2).unwrap_or(&0)];
        r.push(T[(b[0] >> 2) as usize] as char);
        r.push(T[(((b[0] & 0x03) << 4) | (b[1] >> 4)) as usize] as char);
        r.push(if c.len() > 1 {
            T[(((b[1] & 0x0f) << 2) | (b[2] >> 6)) as usize] as char
        } else {
            '='
        });
        r.push(if c.len() > 2 {
            T[(b[2] & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    r
}

fn days_to_ymd(total_days: u64) -> (u32, u32, u32) {
    let mut d = total_days as u32;
    let mut y = 1970u32;
    loop {
        let yd = if is_leap(y) { 366 } else { 365 };
        if d < yd {
            break;
        }
        d -= yd;
        y += 1;
    }
    let months: [u32; 12] = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0u32;
    while d >= months[m as usize] {
        d -= months[m as usize];
        m += 1;
    }
    (y, m + 1, d + 1)
}

fn emit(s: &mut TcpStream, st: &str, ct: &str, b: &[u8]) {
    let _=s.write_all(format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: keep-alive\r\n\r\n",st,ct,b.len()).as_bytes());
    let _ = s.write_all(b);
}
fn emit_void(s: &mut TcpStream) {
    let _ =
        s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
}
fn extract_header(s: &str, n: &str) -> Option<String> {
    for l in s.lines() {
        if let Some(c) = l.find(':') {
            if l[..c].trim().eq_ignore_ascii_case(n) {
                return Some(l[c + 1..].trim().to_string());
            }
        }
    }
    None
}

fn fetch_with_headers(url: &str, headers: &[(String, String)], ttl: u64) -> Option<String> {
    let connect_t = (((ttl as f64) / (Φ * Φ * Φ)).max(1.0) as u64).min(15);
    let max_t = (((ttl as f64) / (Φ * Φ)).max(1.0) as u64).min(30);
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-m")
        .arg(max_t.to_string())
        .arg("--connect-timeout")
        .arg(connect_t.to_string());
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

fn handle_ingress(stream: TcpStream, archive: Arc<Archive>) {
    let mut s = stream;
    s.set_nodelay(true).ok();
    let signal = match read_signal(&mut s) {
        Some(r) => r,
        None => return,
    };
    if signal.to_lowercase().contains("upgrade: websocket") {
        resonance(s, &signal, archive);
    } else {
        let mut cur = signal;
        loop {
            let path = parse_path(&cur);
            if path.starts_with("/crash") {
                let body_start = cur.find("\r\n\r\n").map(|i| &cur[i + 4..]).unwrap_or("");
                let log = format!(
                    "[{}] ASYNC_LOG: {}\n",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    body_start.trim()
                );
                println!("{}", log.trim());
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("crash.log")
                    .and_then(|mut f| f.write_all(log.as_bytes()));
                emit(&mut s, "200 OK", "text/plain", b"ok");
            } else {
                match path.as_str() {
                    "/" => emit(&mut s, "200 OK", "text/html", &archive.index_html),
                    "/time" => {
                        let unix = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs_f64();
                        let tdb = unix - UNIX_J2000_OFFSET;
                        emit(&mut s, "200 OK", "text/plain", tdb.to_string().as_bytes());
                    }
                    "/constants.js" => emit(
                        &mut s,
                        "200 OK",
                        "application/javascript",
                        &archive.constants_js,
                    ),
                    "/gpu.worker.js" => emit(
                        &mut s,
                        "200 OK",
                        "application/javascript",
                        &archive.gpu_worker_js,
                    ),
                    _ => {
                        emit_void(&mut s);
                        break;
                    }
                }
            }
            match read_signal(&mut s) {
                Some(r) => cur = r,
                None => break,
            }
        }
    }
}

fn resonance(mut stream: TcpStream, signal: &str, archive: Arc<Archive>) {
    let key = match extract_header(signal, "Sec-WebSocket-Key") {
        Some(k) => k,
        None => return,
    };
    let encoded = base64_encode(&sha1(
        &format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").into_bytes(),
    ));
    if stream.write_all(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", encoded).as_bytes()).is_err() { return; }
    let _ = stream.set_nodelay(true);
    while let Some(frame) = read_ws_frame_raw(&mut stream) {
        if frame.opcode == 0x8 {
            break;
        }
        if frame.opcode == 0x9 {
            let mut h = [0u8; 2];
            h[0] = 0x8A;
            h[1] = frame.payload.len() as u8;
            if stream.write_all(&h).is_err() {
                break;
            }
            if stream.write_all(&frame.payload).is_err() {
                break;
            }
            continue;
        }
        if frame.opcode == 0x2 {
            if frame.payload.len() < 12 {
                continue;
            }

            let mut cursor = Cursor::new(&frame.payload);
            let mut buf4 = [0u8; 4];

            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let id = u32::from_le_bytes(buf4);
            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let oscillator_count = u32::from_le_bytes(buf4) as usize;

            let mut source_oscillators: Vec<(String, f64)> = Vec::with_capacity(oscillator_count);
            {
                for _ in 0..oscillator_count {
                    let mut val_buf = [0u8; 8];
                    if cursor.read_exact(&mut val_buf).is_err() {
                        break;
                    }
                    let value = f64::from_le_bytes(val_buf);

                    let mut name_len_buf = [0u8; 1];
                    if cursor.read_exact(&mut name_len_buf).is_err() {
                        break;
                    }
                    let name_len = name_len_buf[0] as usize;
                    let mut name_bytes = vec![0u8; name_len];
                    if cursor.read_exact(&mut name_bytes).is_err() {
                        break;
                    }
                    let name = String::from_utf8_lossy(&name_bytes).to_string();

                    source_oscillators.push((name, value));
                }
            }

            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let query_count = u32::from_le_bytes(buf4) as usize;

            let mut out = Vec::with_capacity(1024);
            out.extend_from_slice(&[0xCF, 0x86]);
            out.push(1u8);
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(query_count as u32).to_le_bytes());

            {
                let mut cache = archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                let mut active = archive
                    .active_positions
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                let source_positions = archive
                    .source_positions
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());

                for _ in 0..query_count {
                    let mut t_buf = [0u8; 8];
                    if cursor.read_exact(&mut t_buf).is_err() {
                        break;
                    }
                    let presence_t = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() {
                        break;
                    }
                    let presence_x = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() {
                        break;
                    }
                    let presence_y = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() {
                        break;
                    }
                    let presence_z = f64::from_le_bytes(t_buf);
                    let mut src_lat = None;
                    let mut src_lon = None;
                    for (name, val) in &source_oscillators {
                        if name == "lat" {
                            src_lat = Some(*val);
                        }
                        if name == "lon" {
                            src_lon = Some(*val);
                        }
                    }
                    let browser_pos = if let (Some(lat), Some(lon)) = (src_lat, src_lon) {
                        Some(geodetic_to_icrs(lat, lon, 0.0, presence_t))
                    } else {
                        None
                    };
                    if let Some((source_x, source_y, source_z)) = browser_pos {
                        let browser_key = wgs84_key(src_lat.unwrap(), src_lon.unwrap(), 5);
                        for (name, value) in &source_oscillators {
                            cache
                                .entry(browser_key.clone())
                                .or_insert_with(|| {
                                    (
                                        presence_t,
                                        HashMap::<String, (f64, f64, f64, f64, f64)>::new(),
                                    )
                                })
                                .1
                                .insert(
                                    name.clone(),
                                    (*value, presence_t, source_x, source_y, source_z),
                                );
                        }
                    }
                    active.insert(
                        format!(
                            "{}_{}_{}",
                            presence_x as i64, presence_y as i64, presence_z as i64
                        ),
                        (presence_t, presence_x, presence_y, presence_z),
                    );
                    let obj_pos = out.len();
                    out.extend_from_slice(&0u32.to_le_bytes());
                    let mut merged_values: HashMap<String, (f64, f64, f64, f64, f64)> =
                        HashMap::new();
                    let mut merge_at = |x: f64, y: f64, z: f64, res: i32, key: String| {
                        let dx = x - presence_x;
                        let dy = y - presence_y;
                        let dz = z - presence_z;
                        if dx * dx + dy * dy + dz * dz > res_sq(res) {
                            return;
                        }
                        if let Some((_, values)) = cache.get(&key) {
                            for (k, v) in values {
                                merged_values.insert(k.clone(), (v.0, v.1, v.2, v.3, v.4));
                            }
                        }
                    };
                    for (i, src) in archive.sources.iter().enumerate() {
                        if let (Some(lat), Some(lon)) = (src.lat, src.lon) {
                            let (x, y, z) = geodetic_to_icrs(lat, lon, 0.0, presence_t);
                            merge_at(x, y, z, src.res, wgs84_key(lat, lon, src.res));
                        } else if let Some((x, y, z, key)) = source_positions.get(&i) {
                            merge_at(*x, *y, *z, src.res, key.clone());
                        }
                    }
                    if let (Some(lat), Some(lon), Some((x, y, z))) = (src_lat, src_lon, browser_pos)
                    {
                        merge_at(x, y, z, 5, wgs84_key(lat, lon, 5));
                    }
                    if !merged_values.is_empty() {
                        let fields: Vec<(&str, f64, f64, f64, f64, f64)> = merged_values
                            .iter()
                            .map(|(k, v)| (k.as_str(), v.0, v.1, v.2, v.3, v.4))
                            .collect();
                        φ_obj(&mut out, &fields);
                    }
                    let obj_count = ((out.len() - obj_pos - 4) > 0) as u32;
                    out[obj_pos..obj_pos + 4].copy_from_slice(&obj_count.to_le_bytes());
                }
            }

            out.extend_from_slice(&0u32.to_le_bytes());
            write_ws_binary(&mut stream, &out);
        }
    }
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
fn φ_obj(out: &mut Vec<u8>, fields: &[(&str, f64, f64, f64, f64, f64)]) {
    let mut valid: Vec<&(&str, f64, f64, f64, f64, f64)> = fields
        .iter()
        .filter(|(n, _, _, _, _, _)| !n.is_empty() && n.len() <= 255)
        .collect();
    if valid.len() > 255 {
        valid.truncate(255);
    }
    out.push(valid.len() as u8);
    for (name, val, t, x, y, z) in valid {
        out.push(name.len() as u8);
        out.extend_from_slice(name.as_bytes());
        out.push(0u8);
        out.extend_from_slice(&val.to_le_bytes());
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&x.to_le_bytes());
        out.extend_from_slice(&y.to_le_bytes());
        out.extend_from_slice(&z.to_le_bytes());
    }
}

fn j2d_last_row(json: &JsonVal, col: &str) -> Option<f64> {
    if let JsonVal::Arr(arr) = json {
        if arr.len() < 2 {
            return None;
        }
        if let JsonVal::Arr(headers) = &arr[0] {
            let col_idx = headers.iter().position(|h| {
                if let JsonVal::Str(s) = h {
                    s.eq_ignore_ascii_case(col) || s.starts_with(col)
                } else {
                    false
                }
            })?;
            if let Some(JsonVal::Arr(last_row)) = arr.last() {
                return last_row.get(col_idx).and_then(scalar_of);
            }
        }
    }
    None
}

fn text_last_col(data: &str, col: &str) -> Option<f64> {
    let mut header_idx: Option<usize> = None;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let stripped = trimmed.strip_prefix('#').unwrap_or(trimmed).trim();
        let cols = split_data_line(stripped);
        if header_idx.is_none() {
            if let Some(idx) = cols
                .iter()
                .position(|c| c.eq_ignore_ascii_case(col) || c.starts_with(col))
            {
                header_idx = Some(idx);
                break;
            }
            continue;
        }
    }
    let idx = header_idx?;
    for line in data.lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed
                .chars()
                .next()
                .map(|c| c.is_alphabetic())
                .unwrap_or(false)
        {
            continue;
        }
        let cols = split_data_line(trimmed);
        if let Some(v) = cols.get(idx) {
            if let Ok(f) = v.trim_matches('"').parse::<f64>() {
                return Some(f);
            }
        }
    }
    None
}

fn load_env() {
    if let Ok(content) = std::fs::read_to_string(".env") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(eq) = line.find('=') {
                let key = line[..eq].trim();
                let val = line[eq + 1..].trim();
                if std::env::var(key).is_err() {
                    unsafe {
                        std::env::set_var(key, val);
                    }
                }
            }
        }
    }
}

fn load_sources() -> Vec<SourceConfig> {
    let mut sources = Vec::new();
    let content = std::fs::read_to_string("phi/sources.φ").unwrap_or_default();
    let mut cur_ttl: u64 = 0;
    let mut cur_res: i32 = 0;
    let mut cur_url = String::new();
    let mut cur_lat: Option<f64> = None;
    let mut cur_lon: Option<f64> = None;
    let mut cur_lat_str = String::new();
    let mut cur_format = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new();
    let mut cur_headers: Vec<(String, String)> = Vec::new();
    let mut active = false;

    macro_rules! flush {
        () => {
            if active {
                let mut res = cur_res;
                if res == 0 && cur_lat.is_some() {
                    res = match cur_lat_str.find('.') {
                        Some(dot) => (cur_lat_str.len() - dot - 1) as i32,
                        None => 0,
                    };
                }
                sources.push(SourceConfig {
                    ttl: cur_ttl,
                    url: cur_url.clone(),
                    lat: cur_lat,
                    lon: cur_lon,
                    res,
                    format: cur_format.clone(),
                    extracts: cur_extracts.clone(),
                    headers: cur_headers.clone(),
                });
            }
        };
    }

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("```") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "source" => {
                flush!();
                cur_ttl = 0;
                cur_res = 0;
                cur_url.clear();
                cur_lat = None;
                cur_lon = None;
                cur_lat_str.clear();
                cur_format.clear();
                cur_extracts.clear();
                cur_headers.clear();
                active = true;
            }
            "url" => cur_url = line.get(4..).unwrap_or("").trim().to_string(),
            "ttl" => cur_ttl = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            "res" => cur_res = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            "format" => cur_format = parts.get(1).unwrap_or(&"json").to_string(),
            "lat" => {
                cur_lat_str = parts.get(1).unwrap_or(&"").to_string();
                cur_lat = cur_lat_str.parse().ok();
            }
            "lon" => cur_lon = parts.get(1).and_then(|s| s.parse().ok()),
            "header" => {
                let rest = line.get(7..).unwrap_or("").trim();
                if let Some(sp) = rest.find(' ') {
                    cur_headers.push((
                        rest[..sp].to_string(),
                        rest[sp + 1..].trim_matches('"').to_string(),
                    ));
                }
            }
            "field" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::Field(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "first" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::First(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "last" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::Last(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "count" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::Count(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "last_row" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::LastRow(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "path" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::Path(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "deep" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::Deep(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "sum" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::Sum(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "regex" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::Regex(parts[1].to_string(), parts[2].to_string()));
                }
            }
            "xml_count" => {
                if parts.len() >= 3 {
                    cur_extracts.push(Extract::XmlCount(
                        parts[1].to_string(),
                        parts[2].to_string(),
                    ));
                }
            }
            "vector" => {
                if parts.len() >= 2 {
                    cur_extracts.push(Extract::Vector(parts[1].to_string()));
                }
            }
            "ephemeris" => {
                if parts.len() >= 2 {
                    cur_extracts.push(Extract::Ephemeris(parts[1].to_string()));
                }
            }
            "last_obj" => {
                let quoted = parse_quoted_args(line.get(9..).unwrap_or(""));
                if quoted.len() >= 4 {
                    cur_extracts.push(Extract::LastObj(
                        quoted[0].clone(),
                        quoted[1].clone(),
                        quoted[2].clone(),
                        quoted[3].clone(),
                    ));
                }
            }
            "geojson" => {
                if parts.len() >= 5 && parts[1] == "events" {
                    cur_extracts.push(Extract::GeojsonEvents {
                        mag_key: parts.get(2).unwrap_or(&"mag").to_string(),
                        min_mag: parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                        outputs: parts[4..].iter().map(|s| s.to_string()).collect(),
                    });
                }
            }
            "map" => {
                if parts.len() >= 2 {
                    cur_extracts.push(Extract::Map {
                        arr_path: parts[1].to_string(),
                        lat_key: String::new(),
                        lon_key: String::new(),
                        alt_key: String::new(),
                        fields: Vec::new(),
                    });
                }
            }
            "lat_key" => {
                if let Some(Extract::Map { lat_key, .. }) = cur_extracts.last_mut() {
                    *lat_key = parts.get(1).unwrap_or(&"").to_string();
                }
            }
            "lon_key" => {
                if let Some(Extract::Map { lon_key, .. }) = cur_extracts.last_mut() {
                    *lon_key = parts.get(1).unwrap_or(&"").to_string();
                }
            }
            "alt_key" => {
                if let Some(Extract::Map { alt_key, .. }) = cur_extracts.last_mut() {
                    *alt_key = parts.get(1).unwrap_or(&"").to_string();
                }
            }
            "field_in" => {
                if let Some(Extract::Map { fields, .. }) = cur_extracts.last_mut() {
                    if parts.len() >= 3 {
                        fields.push((parts[1].to_string(), parts[2].to_string()));
                    }
                }
            }
            _ => {}
        }
    }
    flush!();
    sources
}

fn parse_path(s: &str) -> String {
    let fl = s.lines().next().unwrap_or("");
    let p: Vec<&str> = fl.split_whitespace().collect();
    if p.len() >= 2 {
        p[1].to_string()
    } else {
        "/".to_string()
    }
}
fn parse_quoted_args(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.next();
        }
        if chars.peek().is_none() {
            break;
        }
        if *chars.peek().unwrap() == '"' {
            chars.next();
            let mut val = String::new();
            while let Some(&c) = chars.peek() {
                if c == '"' {
                    chars.next();
                    break;
                }
                val.push(c);
                chars.next();
            }
            result.push(val);
        } else {
            let mut val = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    break;
                }
                val.push(c);
                chars.next();
            }
            result.push(val);
        }
    }
    result
}

fn read_signal(s: &mut TcpStream) -> Option<String> {
    let mut buf = [0u8; 8192];
    let mut acc = Vec::new();
    loop {
        match s.read(&mut buf) {
            Ok(0) => return None,
            Ok(n) => {
                acc.extend_from_slice(&buf[..n]);
                if acc.windows(4).any(|w| w == b"\r\n\r\n") {
                    return Some(String::from_utf8_lossy(&acc).to_string());
                }
                if acc.len() > 65536 {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }
}

fn read_ws_frame_raw(stream: &mut TcpStream) -> Option<WsFrame> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).ok()?;
    let opcode = header[0] & 0x0f;
    let masked = (header[1] & 0x80) != 0;
    let mut plen = (header[1] & 0x7f) as usize;
    if plen == 126 {
        let mut e = [0u8; 2];
        stream.read_exact(&mut e).ok()?;
        plen = u16::from_be_bytes(e) as usize;
    } else if plen == 127 {
        let mut e = [0u8; 8];
        stream.read_exact(&mut e).ok()?;
        plen = u64::from_be_bytes(e) as usize;
    }
    let mut mk = [0u8; 4];
    if masked {
        stream.read_exact(&mut mk).ok()?;
    }
    let mut payload = vec![0u8; plen];
    stream.read_exact(&mut payload).ok()?;
    if masked {
        for i in 0..payload.len() {
            payload[i] ^= mk[i % 4];
        }
    }
    Some(WsFrame { opcode, payload })
}

fn render_url(template: &str, x: f64, y: f64, z: f64, tdb_secs: f64, res: i32) -> String {
    let unix = tdb_secs + UNIX_J2000_OFFSET;
    let secs = unix as u64;
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days);
    let today = format!("{}-{:02}-{:02}", ty, tm, td);
    let (yy, ym, yd) = days_to_ymd(days - 1);
    let yesterday = format!("{}-{:02}-{:02}", yy, ym, yd);
    let (tmy, tmm, tmd) = days_to_ymd(days + 1);
    let tomorrow = format!("{}-{:02}-{:02}", tmy, tmm, tmd);
    let today_yyyymmdd = format!("{}_{:02}_{:02}", ty, tm, td);
    let today_nodashes = format!("{}{:02}{:02}", ty, tm, td);
    let yesterday_nodashes = format!("{}{:02}{:02}", yy, ym, yd);
    let tomorrow_nodashes = format!("{}{:02}{:02}", tmy, tmm, tmd);
    let hour_ago = {
        let dt = secs.saturating_sub(3600);
        let (h_y, h_m, h_d) = days_to_ymd(dt / 86400);
        let h_h = (dt % 86400) / 3600;
        let h_min = (dt % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", h_y, h_m, h_d, h_h, h_min)
    };
    let now_iso = {
        let n_h = (secs % 86400) / 3600;
        let n_min = (secs % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", ty, tm, td, n_h, n_min)
    };
    let week_ago = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}-{:02}-{:02}", w_y, w_m, w_d)
    };
    let week_ago_nodashes = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}{:02}{:02}", w_y, w_m, w_d)
    };
    let q_hour = (secs % 86400) / 3600;
    let q_minute = (secs % 3600) / 60;
    let unix_now = secs.to_string();
    let unix_now_plus_3600 = (secs + 3600).to_string();

    let (lat, lon) = icrs_to_geodetic(x, y, z, tdb_secs);
    let res_usize = res.max(0) as usize;
    let lat_str = format!("{:.*}", res_usize, lat);
    let lon_str = format!("{:.*}", res_usize, lon);
    let lat_min_str = format!("{:.*}", res_usize, lat - (1.0 / Φ));
    let lat_max_str = format!("{:.*}", res_usize, lat + (1.0 / Φ));
    let lon_min_str = format!("{:.*}", res_usize, lon - (1.0 / Φ));
    let lon_max_str = format!("{:.*}", res_usize, lon + (1.0 / Φ));

    template
        .replace("{x}", &format!("{}", x))
        .replace("{y}", &format!("{}", y))
        .replace("{z}", &format!("{}", z))
        .replace("{lat}", &lat_str)
        .replace("{lon}", &lon_str)
        .replace("{lat_min}", &lat_min_str)
        .replace("{lat_max}", &lat_max_str)
        .replace("{lon_min}", &lon_min_str)
        .replace("{lon_max}", &lon_max_str)
        .replace("{today}", &today)
        .replace("{yesterday}", &yesterday)
        .replace("{tomorrow}", &tomorrow)
        .replace("{today_yyyymmdd}", &today_yyyymmdd)
        .replace("{today_ymd}", &today_yyyymmdd)
        .replace("{today_nodashes}", &today_nodashes)
        .replace("{yesterday_nodashes}", &yesterday_nodashes)
        .replace("{tomorrow_nodashes}", &tomorrow_nodashes)
        .replace("{t_start}", &yesterday)
        .replace("{t_end}", &today)
        .replace("{now}", &now_iso)
        .replace("{week_ago}", &week_ago)
        .replace("{week_ago_nodashes}", &week_ago_nodashes)
        .replace(
            "{today_plus_365}",
            &format!("{}-{:02}-{:02}", ty + 1, tm, td),
        )
        .replace("{lat_int}", &format!("{}", lat as i32))
        .replace("{lon_int}", &format!("{}", lon as i32))
        .replace("{hour_ago}", &hour_ago)
        .replace("{year}", &ty.to_string())
        .replace("{month}", &tm.to_string())
        .replace("{day}", &td.to_string())
        .replace("{hour}", &format!("{:02}", q_hour))
        .replace("{minute}", &format!("{:02}", q_minute))
        .replace("{unix_now}", &unix_now)
        .replace("{unix_now_plus_3600}", &unix_now_plus_3600)
        .replace(
            "{nasa_key}",
            &std::env::var("NASA_KEY").unwrap_or_else(|_| "DEMO_KEY".to_string()),
        )
}

fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let bl = (data.len() as u64) * 8;
    let mut m = data.to_vec();
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&bl.to_be_bytes());
    for chunk in m.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut r = [0u8; 20];
    for i in 0..5 {
        r[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    r
}

fn split_data_line(line: &str) -> Vec<&str> {
    if line.contains(';') {
        line.split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else if line.contains(',') && line.split(',').count() > 2 {
        line.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        line.split_whitespace().collect()
    }
}

fn write_ws_binary(stream: &mut TcpStream, data: &[u8]) {
    let mut h = [0u8; 10];
    h[0] = 0x82;
    if data.len() <= 125 {
        h[1] = data.len() as u8;
        let _ = stream.write_all(&h[..2]);
    } else if data.len() <= 65535 {
        h[1] = 126;
        let e = (data.len() as u16).to_be_bytes();
        h[2] = e[0];
        h[3] = e[1];
        let _ = stream.write_all(&h[..4]);
    } else {
        h[1] = 127;
        let e = (data.len() as u64).to_be_bytes();
        h[2..10].copy_from_slice(&e);
        let _ = stream.write_all(&h);
    }
    let _ = stream.write_all(data);
}

static CURL_PERMITS: AtomicUsize = AtomicUsize::new(8);

struct CurlPermit;

impl CurlPermit {
    fn acquire() -> Self {
        loop {
            let cur = CURL_PERMITS.load(Ordering::Acquire);
            if cur > 0
                && CURL_PERMITS
                    .compare_exchange_weak(cur, cur - 1, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
            {
                return CurlPermit;
            }
            thread::yield_now();
        }
    }
}

impl Drop for CurlPermit {
    fn drop(&mut self) {
        CURL_PERMITS.fetch_add(1, Ordering::AcqRel);
    }
}

fn warm_cache(archive: Arc<Archive>) {
    loop {
        let positions: Vec<(f64, f64, f64, f64)> = archive
            .active_positions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .values()
            .cloned()
            .collect();
        if positions.is_empty() {
            let min_ttl = archive.sources.iter().map(|s| s.ttl).min().unwrap_or(60);
            thread::sleep(std::time::Duration::from_secs(
                (min_ttl as f64 / Φ).max(1.0) as u64,
            ));
            continue;
        }

        for (query_t, pos_x, pos_y, pos_z) in &positions {
            let needs: Vec<(usize, String, Vec<(String, String)>, u64)> = archive
                .sources
                .iter()
                .enumerate()
                .filter_map(|(i, src)| {
                    if let (Some(lat), Some(lon)) = (src.lat, src.lon) {
                        let (source_x, source_y, source_z) =
                            geodetic_to_icrs(lat, lon, 0.0, *query_t);
                        let dx = source_x - *pos_x;
                        let dy = source_y - *pos_y;
                        let dz = source_z - *pos_z;
                        if dx * dx + dy * dy + dz * dz > res_sq(src.res) {
                            return None;
                        }
                        let cache_key = wgs84_key(lat, lon, src.res);
                        let needs_fetch = {
                            let cache =
                                archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                            match cache.get(&cache_key) {
                                Some((ts, _)) => *query_t - *ts >= src.ttl as f64,
                                None => true,
                            }
                        };
                        if !needs_fetch {
                            return None;
                        }
                        let url =
                            render_url(&src.url, source_x, source_y, source_z, *query_t, src.res);
                        let headers_rendered: Vec<(String, String)> = src
                            .headers
                            .iter()
                            .map(|(k, v)| {
                                (
                                    k.clone(),
                                    render_url(v, source_x, source_y, source_z, *query_t, src.res),
                                )
                            })
                            .collect();
                        Some((i, url, headers_rendered, src.ttl))
                    } else {
                        let prev_pos = archive
                            .source_positions
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .get(&i)
                            .cloned();
                        if let Some((_, _, _, ref prev_key)) = prev_pos {
                            let fresh = {
                                let cache =
                                    archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                                match cache.get(prev_key) {
                                    Some((ts, _)) => *query_t - *ts < src.ttl as f64,
                                    None => false,
                                }
                            };
                            if fresh {
                                return None;
                            }
                        }
                        let url = render_url(&src.url, *pos_x, *pos_y, *pos_z, *query_t, src.res);
                        let headers_rendered: Vec<(String, String)> = src
                            .headers
                            .iter()
                            .map(|(k, v)| {
                                (
                                    k.clone(),
                                    render_url(v, *pos_x, *pos_y, *pos_z, *query_t, src.res),
                                )
                            })
                            .collect();
                        Some((i, url, headers_rendered, src.ttl))
                    }
                })
                .collect();

            if needs.is_empty() {
                continue;
            }

            let results: Vec<(usize, Option<String>)> = thread::scope(|s| {
                let handles: Vec<_> = needs
                    .iter()
                    .map(|&(i, ref url, ref headers, ref ttl)| {
                        s.spawn(move || {
                            let _permit = CurlPermit::acquire();
                            let body = fetch_with_headers(url, headers, *ttl);
                            (i, body)
                        })
                    })
                    .collect();
                handles.into_iter().filter_map(|h| h.join().ok()).collect()
            });

            for (src_idx, body_opt) in results {
                if let Some(body) = body_opt {
                    let src = &archive.sources[src_idx];
                    let mut extracted: HashMap<String, f64> = HashMap::new();
                    let mut eph_pos: Option<(f64, f64, f64)> = None;
                    let parsed_json = if src.format == "json" || src.format.is_empty() {
                        parse_json(&body)
                    } else {
                        None
                    };

                    for ext in &src.extracts {
                        match ext {
                            Extract::Field(k, n) => {
                                if let Some(ref j) = parsed_json {
                                    if let Some(v) = jnum(j, k) {
                                        extracted.insert(n.clone(), v);
                                    }
                                }
                            }
                            Extract::First(k, n) => {
                                if let Some(ref j) = parsed_json {
                                    if let Some(v) = jfirst(j, k) {
                                        extracted.insert(n.clone(), v);
                                    }
                                }
                            }
                            Extract::Last(k, n) => {
                                if let Some(ref j) = parsed_json {
                                    if let Some(v) = jlast(j, k) {
                                        extracted.insert(n.clone(), v);
                                    }
                                }
                            }
                            Extract::Count(k, n) => {
                                let v = if src.format == "csv" || k == "lines" {
                                    Some(
                                        body.lines()
                                            .filter(|l| {
                                                !l.trim().is_empty() && !l.trim().starts_with('#')
                                            })
                                            .count() as f64,
                                    )
                                } else {
                                    parsed_json.as_ref().and_then(|j| jcount(j, k))
                                };
                                if let Some(v) = v {
                                    extracted.insert(n.clone(), v);
                                }
                            }
                            Extract::LastRow(k, n) => {
                                if let Some(ref j) = parsed_json {
                                    if let Some(v) = j2d_last_row(j, k) {
                                        extracted.insert(n.clone(), v);
                                    }
                                } else {
                                    if let Some(v) = text_last_col(&body, k) {
                                        extracted.insert(n.clone(), v);
                                    }
                                }
                            }
                            Extract::Path(k, n) => {
                                if let Some(ref j) = parsed_json {
                                    if let Some(v) = jpath(j, k) {
                                        extracted.insert(n.clone(), v);
                                    }
                                }
                            }
                            Extract::Deep(k, n) => {
                                if let Some(ref j) = parsed_json {
                                    if let Some(v) = jdeep_find_num(j, k) {
                                        extracted.insert(n.clone(), v);
                                    }
                                }
                            }
                            Extract::Sum(path, n) => {
                                if let Some(ref j) = parsed_json {
                                    let target = if path == "." || path.is_empty() {
                                        Some(j)
                                    } else {
                                        jpath_val(j, path)
                                    };
                                    if let Some(JsonVal::Arr(arr)) = target {
                                        let sum: f64 =
                                            arr.iter().filter_map(|v| scalar_of(v)).sum();
                                        if sum.is_finite() {
                                            extracted.insert(n.clone(), sum);
                                        }
                                    }
                                }
                            }
                            Extract::Regex(pat, n) => {
                                if let Some(v) = extract_regex_val(&body, pat) {
                                    extracted.insert(n.clone(), v);
                                }
                            }
                            Extract::XmlCount(tag, n) => {
                                let count = body.matches(&format!("<{}>", tag)).count() as f64;
                                extracted.insert(n.clone(), count);
                            }
                            Extract::Vector(prefix) => {
                                if let Some(v) =
                                    extract_regex_val(&body, &format!("({}...)", prefix))
                                {
                                    extracted.insert(format!("{}_val", prefix), v);
                                }
                            }
                            Extract::Ephemeris(n) => {
                                let ht = if let Some(ref j) = parsed_json {
                                    if let JsonVal::Obj(m) = j {
                                        if let Some(JsonVal::Str(s)) = m.get("result") {
                                            s.clone()
                                        } else {
                                            body.clone()
                                        }
                                    } else {
                                        body.clone()
                                    }
                                } else {
                                    body.clone()
                                };
                                if let Some(soe) = ht.find("$$SOE") {
                                    let a = &ht[soe + 5..];
                                    let e = a.find("$$EOE").unwrap_or(a.len());
                                    let blk = &a[..e];
                                    let ph = |k: &str| -> Option<f64> {
                                        let p = blk.find(k)?;
                                        let r = blk[p + k.len()..].trim_start_matches(|c: char| {
                                            c == '=' || c == ' ' || c == '\t'
                                        });
                                        let end =
                                            r.find(|c: char| c.is_whitespace()).unwrap_or(r.len());
                                        r[..end].parse::<f64>().ok()
                                    };
                                    if let (Some(x), Some(y), Some(z), Some(rg)) =
                                        (ph("X"), ph("Y"), ph("Z"), ph("RG"))
                                    {
                                        eph_pos = Some((x * 1000.0, y * 1000.0, z * 1000.0));
                                        extracted.insert(n.clone(), rg * 1000.0);
                                    }
                                }
                            }
                            Extract::LastObj(fk, fv, ek, n) => {
                                if let Some(ref j) = parsed_json {
                                    if let JsonVal::Arr(arr) = j {
                                        for v in arr.iter().rev() {
                                            if let JsonVal::Obj(o) = v {
                                                if let Some(JsonVal::Str(s)) = o.get(fk) {
                                                    if s == fv {
                                                        if let Some(val) = jnum(v, ek) {
                                                            extracted.insert(n.clone(), val);
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Extract::Map {
                                arr_path,
                                lat_key,
                                lon_key,
                                alt_key,
                                fields,
                            } => {
                                if let Some(ref j) = parsed_json {
                                    let mut current = j;
                                    let mut path_ok = true;
                                    for part in arr_path.split('.') {
                                        if let Ok(idx) = part.parse::<usize>() {
                                            if let JsonVal::Arr(arr) = current {
                                                current = match arr.get(idx) {
                                                    Some(v) => v,
                                                    None => {
                                                        path_ok = false;
                                                        break;
                                                    }
                                                };
                                            } else {
                                                path_ok = false;
                                                break;
                                            }
                                        } else {
                                            if let JsonVal::Obj(map) = current {
                                                current = match map.get(part) {
                                                    Some(v) => v,
                                                    None => {
                                                        path_ok = false;
                                                        break;
                                                    }
                                                };
                                            } else {
                                                path_ok = false;
                                                break;
                                            }
                                        }
                                    }
                                    if path_ok {
                                        if let JsonVal::Arr(arr) = current {
                                            let mut cache = archive
                                                .data_cache
                                                .lock()
                                                .unwrap_or_else(|e| e.into_inner());
                                            for v in arr.iter() {
                                                let lat = jpath(v, lat_key);
                                                let lon = jpath(v, lon_key);
                                                let alt = if alt_key.is_empty() {
                                                    Some(0.0)
                                                } else {
                                                    jpath(v, alt_key)
                                                };
                                                if let (Some(la), Some(lo), Some(al)) =
                                                    (lat, lon, alt)
                                                {
                                                    let (ev_x, ev_y, ev_z) =
                                                        geodetic_to_icrs(la, lo, al, *query_t);
                                                    let ev_key = wgs84_key(la, lo, src.res);
                                                    let mut ev_vals: HashMap<
                                                        String,
                                                        (f64, f64, f64, f64, f64),
                                                    > = HashMap::new();
                                                    for (fk, fn_) in fields {
                                                        if let Some(val) = jpath(v, fk) {
                                                            ev_vals.insert(
                                                                fn_.clone(),
                                                                (val, *query_t, ev_x, ev_y, ev_z),
                                                            );
                                                        }
                                                    }
                                                    if !ev_vals.is_empty() {
                                                        cache.insert(ev_key, (*query_t, ev_vals));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Extract::GeojsonEvents {
                                mag_key,
                                min_mag,
                                outputs,
                            } => {
                                if outputs.len() >= 2 {
                                    if let Some(ref j) = parsed_json {
                                        if let JsonVal::Obj(root) = j {
                                            if let Some(JsonVal::Arr(features)) =
                                                root.get("features")
                                            {
                                                let mut cache = archive
                                                    .data_cache
                                                    .lock()
                                                    .unwrap_or_else(|e| e.into_inner());
                                                for feat in features {
                                                    if let JsonVal::Obj(f) = feat {
                                                        let mut elo = 0.0;
                                                        let mut ela = 0.0;
                                                        let mut ed = 0.0;
                                                        let mut mag = 0.0;
                                                        let mut valid = false;
                                                        if let Some(JsonVal::Obj(geom)) =
                                                            f.get("geometry")
                                                        {
                                                            if let Some(JsonVal::Arr(c)) =
                                                                geom.get("coordinates")
                                                            {
                                                                if c.len() >= 3 {
                                                                    if let JsonVal::Num(n) = c[0] {
                                                                        elo = n;
                                                                    }
                                                                    if let JsonVal::Num(n) = c[1] {
                                                                        ela = n;
                                                                    }
                                                                    if let JsonVal::Num(n) = c[2] {
                                                                        ed = n;
                                                                    }
                                                                    valid = true;
                                                                }
                                                            }
                                                        }
                                                        if valid {
                                                            if let Some(props) = f.get("properties")
                                                            {
                                                                if let Some(m) =
                                                                    jnum(props, mag_key)
                                                                {
                                                                    mag = m;
                                                                }
                                                            }
                                                            if mag >= *min_mag {
                                                                let (ev_x, ev_y, ev_z) =
                                                                    geodetic_to_icrs(
                                                                        ela, elo, 0.0, *query_t,
                                                                    );
                                                                let ev_key =
                                                                    wgs84_key(ela, elo, src.res);
                                                                let mut ev_vals: HashMap<
                                                                    String,
                                                                    (f64, f64, f64, f64, f64),
                                                                > = HashMap::new();
                                                                ev_vals.insert(
                                                                    outputs[0].clone(),
                                                                    (
                                                                        mag, *query_t, ev_x, ev_y,
                                                                        ev_z,
                                                                    ),
                                                                );
                                                                ev_vals.insert(
                                                                    outputs[1].clone(),
                                                                    (
                                                                        ed, *query_t, ev_x, ev_y,
                                                                        ev_z,
                                                                    ),
                                                                );
                                                                cache.insert(
                                                                    ev_key,
                                                                    (*query_t, ev_vals),
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if !extracted.is_empty() {
                        let (cache_key, fx, fy, fz) = if let Some((px, py, pz)) = eph_pos {
                            (icrs_key(px, py, pz, *query_t), px, py, pz)
                        } else if let (Some(la), Some(lo)) = (src.lat, src.lon) {
                            let (px, py, pz) = geodetic_to_icrs(la, lo, 0.0, *query_t);
                            (wgs84_key(la, lo, src.res), px, py, pz)
                        } else if let (Some(la), Some(lo)) =
                            (extracted.get("lat"), extracted.get("lon"))
                        {
                            let (px, py, pz) = geodetic_to_icrs(*la, *lo, 0.0, *query_t);
                            (wgs84_key(*la, *lo, src.res), px, py, pz)
                        } else {
                            (
                                icrs_key(*pos_x, *pos_y, *pos_z, *query_t),
                                *pos_x,
                                *pos_y,
                                *pos_z,
                            )
                        };
                        archive
                            .source_positions
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .insert(src_idx, (fx, fy, fz, cache_key.clone()));
                        let extracted_with_t: HashMap<String, (f64, f64, f64, f64, f64)> =
                            extracted
                                .iter()
                                .map(|(k, v)| (k.clone(), (*v, *query_t, fx, fy, fz)))
                                .collect();
                        archive
                            .data_cache
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .insert(cache_key, (*query_t, extracted_with_t));
                    }
                }
            }
        }

        let min_ttl = archive.sources.iter().map(|s| s.ttl).min().unwrap_or(60);
        let max_ttl = archive.sources.iter().map(|s| s.ttl).max().unwrap_or(3600);
        let now_tdb = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            - UNIX_J2000_OFFSET;
        let evict_thresh = now_tdb - max_ttl as f64 * 2.0;
        archive
            .data_cache
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|_, (ts, _)| *ts > evict_thresh);
        archive
            .active_positions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|_, (t, _, _, _)| now_tdb - *t < 300.0);
        thread::sleep(std::time::Duration::from_secs(
            (min_ttl as f64 / (Φ * Φ)).max(1.0) as u64,
        ));
    }
}

fn main() {
    load_env();
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1111);
    let archive = Arc::new(Archive {
        sources: load_sources(),
        index_html: std::fs::read("static/index.html").unwrap_or_default(),
        constants_js: std::fs::read("static/constants.js").unwrap_or_default(),
        gpu_worker_js: std::fs::read("static/gpu.worker.js").unwrap_or_default(),
        data_cache: Mutex::new(HashMap::new()),
        active_positions: Mutex::new(HashMap::new()),
        source_positions: Mutex::new(HashMap::new()),
    });
    {
        let ar = Arc::clone(&archive);
        thread::spawn(move || warm_cache(ar));
    }
    if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let ar = Arc::clone(&archive);
                thread::spawn(move || handle_ingress(stream, ar));
            }
        }
    }
}
