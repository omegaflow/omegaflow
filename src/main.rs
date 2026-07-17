#![allow(mixed_script_confusables)]
use std::collections::HashMap;
use std::io::{Read, Write, Cursor};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
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
    let lat_r = lat * std::f64::consts::PI / 180.0;
    let lon_r = lon * std::f64::consts::PI / 180.0;
    const WGS84_F: f64 = 1.0 / 298.257223563;
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let sin_lat = lat_r.sin();
    let n = EARTH_RADIUS / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    let x_ecef = (n + alt) * lat_r.cos() * lon_r.cos();
    let y_ecef = (n + alt) * lat_r.cos() * lon_r.sin();
    let z_ecef = (n * (1.0 - e2) + alt) * lat_r.sin();
    let jd = tdb_to_jd(tdb_secs);
    let t = (jd - J2000_EPOCH) / 36525.0;
    let gmst = 280.46061837 + 360.98564736629 * (jd - J2000_EPOCH) + 0.000387933 * t * t - t * t * t / 38710000.0;
    let gmst_rad = (gmst % 360.0) * std::f64::consts::PI / 180.0;
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
    let gmst = 280.46061837 + 360.98564736629 * (jd - J2000_EPOCH) + 0.000387933 * t * t - t * t * t / 38710000.0;
    let gmst_rad = (gmst % 360.0) * std::f64::consts::PI / 180.0;
    let x_ecef = x_eci * gmst_rad.cos() - y_eci * gmst_rad.sin();
    let y_ecef = x_eci * gmst_rad.sin() + y_eci * gmst_rad.cos();
    let z_ecef = z_eci;
    let lon = y_ecef.atan2(x_ecef).to_degrees();
    let lat = (z_ecef / EARTH_RADIUS).atan().to_degrees();
    (lat, lon)
}

#[derive(Clone, Debug)]
pub enum JsonVal {
    Null, Bool(bool), Num(f64), Str(String), Arr(Vec<JsonVal>), Obj(HashMap<String, JsonVal>),
}

pub fn parse_json(s: &str) -> Option<JsonVal> {
    let mut p = JsonParser { chars: s.as_bytes(), pos: 0 };
    p.skip_ws(); p.parse_value()
}

struct JsonParser<'a> { chars: &'a [u8], pos: usize }

impl<'a> JsonParser<'a> {
    fn skip_ws(&mut self) { while self.pos < self.chars.len() && (self.chars[self.pos] as char).is_whitespace() { self.pos += 1; } }
    fn parse_value(&mut self) -> Option<JsonVal> {
        self.skip_ws();
        if self.pos >= self.chars.len() { return None; }
        match self.chars[self.pos] {
            b'{' => self.parse_obj(), b'[' => self.parse_arr(), b'"' => self.parse_str().map(JsonVal::Str),
            b't' => { self.pos += 4; Some(JsonVal::Bool(true)) }, b'f' => { self.pos += 5; Some(JsonVal::Bool(false)) },
            b'n' => { self.pos += 4; Some(JsonVal::Null) }, _ => self.parse_num(),
        }
    }
    fn parse_obj(&mut self) -> Option<JsonVal> {
        self.pos += 1; self.skip_ws();
        let mut map = HashMap::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b'}' { self.pos += 1; return Some(JsonVal::Obj(map)); }
        loop {
            self.skip_ws(); let key = self.parse_str()?; self.skip_ws();
            if self.pos >= self.chars.len() || self.chars[self.pos] != b':' { return None; }
            self.pos += 1; let val = self.parse_value()?; map.insert(key, val); self.skip_ws();
            if self.pos >= self.chars.len() { return None; }
            match self.chars[self.pos] { b',' => { self.pos += 1; } b'}' => { self.pos += 1; break; } _ => return None, }
        }
        Some(JsonVal::Obj(map))
    }
    fn parse_arr(&mut self) -> Option<JsonVal> {
        self.pos += 1; self.skip_ws();
        let mut arr = Vec::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b']' { self.pos += 1; return Some(JsonVal::Arr(arr)); }
        loop {
            let val = self.parse_value()?; arr.push(val); self.skip_ws();
            if self.pos >= self.chars.len() { return None; }
            match self.chars[self.pos] { b',' => { self.pos += 1; } b']' => { self.pos += 1; break; } _ => return None, }
        }
        Some(JsonVal::Arr(arr))
    }
    fn parse_str(&mut self) -> Option<String> {
        if self.pos >= self.chars.len() || self.chars[self.pos] != b'"' { return None; }
        self.pos += 1; let mut s = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == b'\\' && self.pos + 1 < self.chars.len() {
                self.pos += 1;
                match self.chars[self.pos] { b'"' => s.push('"'), b'\\' => s.push('\\'), b'/' => s.push('/'), b'n' => s.push('\n'), b't' => s.push('\t'), _ => {} }
                self.pos += 1;
            } else if c == b'"' { self.pos += 1; return Some(s); } else { s.push(c as char); self.pos += 1; }
        }
        None
    }
    fn parse_num(&mut self) -> Option<JsonVal> {
        let start = self.pos;
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c.is_ascii_digit() || c == b'-' || c == b'+' || c == b'.' || c == b'e' || c == b'E' { self.pos += 1; } else { break; }
        }
        let s = std::str::from_utf8(&self.chars[start..self.pos]).ok()?;
        s.parse::<f64>().ok().map(JsonVal::Num)
    }
}

fn jnum(json: &JsonVal, key: &str) -> Option<f64> {
    if let JsonVal::Obj(map) = json {
        if let Some(JsonVal::Num(n)) = map.get(key) { return Some(*n); }
        if let Some(JsonVal::Str(s)) = map.get(key) { return s.parse().ok(); }
    }
    None
}

fn jpath(json: &JsonVal, path: &str) -> Option<f64> {
    let mut current = json;
    for part in path.split('.') {
        if let Ok(idx) = part.parse::<usize>() {
            if let JsonVal::Arr(arr) = current { current = arr.get(idx)?; } else { return None; }
        } else {
            if let JsonVal::Obj(map) = current { current = map.get(part)?; } else { return None; }
        }
    }
    if let JsonVal::Num(n) = current { Some(*n) } else if let JsonVal::Str(s) = current { s.parse().ok() } else { None }
}

fn jdeep_find_num(json: &JsonVal, key: &str) -> Option<f64> {
    match json {
        JsonVal::Obj(map) => {
            if let Some(v) = map.get(key) {
                if let JsonVal::Num(n) = v { return Some(*n); }
                if let JsonVal::Str(s) = v { return s.parse().ok(); }
            }
            for v in map.values() { if let Some(n) = jdeep_find_num(v, key) { return Some(n); } }
            None
        }
        JsonVal::Arr(arr) => {
            for v in arr { if let Some(n) = jdeep_find_num(v, key) { return Some(n); } }
            None
        }
        _ => None,
    }
}

fn jarr_count(json: &JsonVal, key: &str) -> Option<f64> {
    if let JsonVal::Obj(map) = json { if let Some(JsonVal::Arr(arr)) = map.get(key) { return Some(arr.len() as f64); } }
    None
}

fn jarr_first(json: &JsonVal, key: &str) -> Option<f64> {
    if let JsonVal::Obj(map) = json {
        if let Some(JsonVal::Arr(arr)) = map.get(key) {
            if arr.is_empty() { return None; }
            if let Some(JsonVal::Num(n)) = arr.first() { return Some(*n); }
            if let Some(JsonVal::Str(s)) = arr.first() { return s.parse().ok(); }
        }
    }
    None
}

fn jarr_last(json: &JsonVal, key: &str) -> Option<f64> {
    if let JsonVal::Obj(map) = json {
        if let Some(JsonVal::Arr(arr)) = map.get(key) {
            if arr.is_empty() { return None; }
            if let Some(JsonVal::Num(n)) = arr.last() { return Some(*n); }
            if let Some(JsonVal::Str(s)) = arr.last() { return s.parse().ok(); }
        }
    }
    None
}

fn jsum(json: &JsonVal, key: &str) -> Option<f64> {
    if let JsonVal::Obj(map) = json {
        if let Some(JsonVal::Arr(arr)) = map.get(key) {
            let mut sum = 0.0;
            for v in arr {
                if let JsonVal::Num(n) = v { sum += *n; }
                else if let JsonVal::Str(s) = v { sum += s.parse::<f64>().unwrap_or(0.0); }
            }
            return Some(sum);
        }
    }
    None
}

fn jarr_avg(json: &JsonVal, path: &str) -> Option<f64> {
    let mut current = json;
    for part in path.split('.') {
        if let Ok(idx) = part.parse::<usize>() {
            if let JsonVal::Arr(arr) = current { current = arr.get(idx)?; } else { return None; }
        } else {
            if let JsonVal::Obj(map) = current { current = map.get(part)?; } else { return None; }
        }
    }
    if let JsonVal::Arr(arr) = current {
        let mut sum = 0.0; let mut count = 0.0;
        for v in arr {
            if let JsonVal::Num(n) = v { sum += *n; count += 1.0; }
            else if let JsonVal::Str(s) = v { if let Ok(n) = s.parse::<f64>() { sum += n; count += 1.0; } }
        }
        if count > 0.0 { return Some(sum / count); }
    }
    None
}

#[derive(Clone)]
enum Extract {
    Field(String, String), First(String, String), Last(String, String), Count(String, String),
    LastRow(String, String), Vector(String, String, String), LastObj(String, String, String, String),
    GeojsonEvents { mag_key: String, min_mag: f64, outputs: Vec<String> }, Path(String, String),
    Sum(String, String), Map(String, Vec<(String, String)>), Deep(String, String), DeepArr(String, String), Avg(String, String),
}

struct SourceConfig { ttl: u64, res: i32, url: String, lat: Option<f64>, lon: Option<f64>, format: String, extracts: Vec<Extract>, headers: Vec<(String, String)> }


struct Archive { sources: Vec<SourceConfig>, index_html: Vec<u8>, constants_js: Vec<u8>, data_cache: Mutex<HashMap<String, (f64, HashMap<String, (f64, f64, f64, f64, f64)>)>>, active_positions: Mutex<HashMap<String, f64>> }
struct WsFrame { opcode: u8, payload: Vec<u8> }

fn base64_encode(input: &[u8]) -> String {
    const T: &[u8;64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = String::new();
    for c in input.chunks(3) {
        let b = [c[0], *c.get(1).unwrap_or(&0), *c.get(2).unwrap_or(&0)];
        r.push(T[(b[0]>>2) as usize] as char);
        r.push(T[(((b[0]&0x03)<<4)|(b[1]>>4)) as usize] as char);
        r.push(if c.len()>1 { T[(((b[1]&0x0f)<<2)|(b[2]>>6)) as usize] as char } else { '=' });
        r.push(if c.len()>2 { T[(b[2]&0x3f) as usize] as char } else { '=' });
    }
    r
}

fn days_to_ymd(total_days: u64) -> (u32, u32, u32) {
    let mut d = total_days as u32; let mut y = 1970u32;
    loop { let yd = if is_leap(y) { 366 } else { 365 }; if d < yd { break; } d -= yd; y += 1; }
    let months: [u32; 12] = if is_leap(y) { [31,29,31,30,31,30,31,31,30,31,30,31] } else { [31,28,31,30,31,30,31,31,30,31,30,31] };
    let mut m = 0u32; while d >= months[m as usize] { d -= months[m as usize]; m += 1; }
    (y, m + 1, d + 1)
}

fn pos_key(x: f64, y: f64, z: f64, res: i32, t: f64) -> String {
    let (lat, lon) = icrs_to_geodetic(x, y, z, t);
    let r = if res < 0 { 0 } else { res as usize };
    format!("{}_{}", format!("{:.*}", r, lat), format!("{:.*}", r, lon))
}

fn parse_pos(pos: &str) -> Option<(f64, f64, f64)> {
    let parts: Vec<&str> = pos.split('_').collect();
    if parts.len() == 3 { 
        if let (Ok(x), Ok(y), Ok(z)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>(), parts[2].parse::<f64>()) { 
            return Some((x, y, z)); 
        } 
    }
    None
}

fn emit(s: &mut TcpStream, st: &str, ct: &str, b: &[u8]) { let _=s.write_all(format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: keep-alive\r\n\r\n",st,ct,b.len()).as_bytes()); let _=s.write_all(b); }
fn emit_void(s: &mut TcpStream) { let _=s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"); }
fn extract_header(s: &str, n: &str) -> Option<String> { for l in s.lines() { if let Some(c) = l.find(':') { if l[..c].trim().eq_ignore_ascii_case(n) { return Some(l[c+1..].trim().to_string()); } } } None }

fn fetch_with_headers(url: &str, headers: &[(String, String)], ttl: u64) -> Option<String> {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).max(1.0) as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).max(1.0) as u64;
    let mut cmd = Command::new("curl"); cmd.arg("-s").arg("-k").arg("-m").arg(max_t.to_string()).arg("--connect-timeout").arg(connect_t.to_string());
    for (k, v) in headers { cmd.arg("-H").arg(format!("{}: {}", k, v)); }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() { Some(String::from_utf8_lossy(&output.stdout).to_string()) } else { None }
}

fn handle_ingress(stream: TcpStream, archive: Arc<Archive>) {
    let mut s = stream; s.set_nodelay(true).ok();
    let signal = match read_signal(&mut s) { Some(r) => r, None => return };
    if signal.to_lowercase().contains("upgrade: websocket") { handle_pulse(s, &signal, archive); }
    else {
        let mut cur = signal;
        loop {
            let path = parse_path(&cur);
            if path.starts_with("/crash") {
                let body_start = cur.find("\r\n\r\n").map(|i| &cur[i+4..]).unwrap_or("");
                let log = format!("[{}] ASYNC_LOG: {}\n", SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(), body_start.trim());
                println!("{}", log.trim());
                let _ = std::fs::OpenOptions::new().create(true).append(true).open("crash.log").and_then(|mut f| f.write_all(log.as_bytes()));
                emit(&mut s, "200 OK", "text/plain", b"ok");
            } else {
                match path.as_str() {
                    "/" => emit(&mut s, "200 OK", "text/html", &archive.index_html),
                    "/time" => { 
                        let unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64();
                        let tdb = unix - UNIX_J2000_OFFSET;
                        emit(&mut s, "200 OK", "text/plain", tdb.to_string().as_bytes()); 
                    }
                    "/constants.js" => emit(&mut s, "200 OK", "application/javascript", &archive.constants_js),
                    _ => { emit_void(&mut s); break; }
                }
            }
            match read_signal(&mut s) { Some(r) => cur = r, None => break }
        }
    }
}

fn handle_pulse(mut stream: TcpStream, signal: &str, archive: Arc<Archive>) {
    let key = match extract_header(signal,"Sec-WebSocket-Key") { Some(k)=>k, None=>return };
    let encoded = base64_encode(&sha1(&format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").into_bytes()));
    if stream.write_all(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", encoded).as_bytes()).is_err() { return; }
    let _=stream.set_nodelay(true);
    let mut last_coord: (f64, f64, f64) = (f64::NAN, f64::NAN, f64::NAN);

    while let Some(frame) = read_ws_frame_raw(&mut stream) {
        if frame.opcode==0x8 { break; }
        if frame.opcode==0x2 {
            if frame.payload.len()<12 { continue; }

            let mut cursor = Cursor::new(&frame.payload);
            let mut buf4 = [0u8; 4];

            if cursor.read_exact(&mut buf4).is_err() { continue; } let id = u32::from_le_bytes(buf4);
            if cursor.read_exact(&mut buf4).is_err() { continue; } let input_count = u32::from_le_bytes(buf4) as usize;

            {
                let mut cache = archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                let mut t_frame = 0.0;
                for i in 0..input_count {
                    let mut t_buf = [0u8; 8];
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let _t = f64::from_le_bytes(t_buf);
                    if i == 0 { t_frame = _t; }
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let x = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let y = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let z = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let value = f64::from_le_bytes(t_buf);

                    let mut name_len_buf = [0u8; 1];
                    if cursor.read_exact(&mut name_len_buf).is_err() { break; }
                    let name_len = name_len_buf[0] as usize;
                    let mut name_bytes = vec![0u8; name_len];
                    if cursor.read_exact(&mut name_bytes).is_err() { break; }
                    let name = String::from_utf8_lossy(&name_bytes).to_string();

                    let local_key = format!("local_{}", pos_key(last_coord.0, last_coord.1, last_coord.2, 7, t_frame));

                    cache.entry(local_key).or_insert_with(|| (t_frame, HashMap::<String, (f64, f64, f64, f64, f64)>::new())).1.insert(name, (value, t_frame, x, y, z));
                }
            }

            if cursor.read_exact(&mut buf4).is_err() { continue; }
            let query_count = u32::from_le_bytes(buf4) as usize;

            let mut out=Vec::with_capacity(1024);
            out.extend_from_slice(&[0xCF, 0x86]); out.push(1u8);
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(query_count as u32).to_le_bytes());

            {
                let cache=archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                let mut active=archive.active_positions.lock().unwrap_or_else(|e| e.into_inner());

                for _ in 0..query_count {
                    let mut t_buf = [0u8; 8];
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let q_t = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let x = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let y = f64::from_le_bytes(t_buf);
                    if cursor.read_exact(&mut t_buf).is_err() { break; } let z = f64::from_le_bytes(t_buf);

                    if (x,y,z) != last_coord {
                        last_coord=(x,y,z);
                    }

                    active.insert(format!("{}_{}_{}", x, y, z), q_t);

                    let obj_pos=out.len();
                    out.extend_from_slice(&0u32.to_le_bytes());
                    let mut merged_values: HashMap<String, (f64, f64, f64, f64, f64)> = HashMap::new();

                    let local_key = format!("local_{}", pos_key(x, y, z, 7, q_t));
                    if let Some((_, values)) = cache.get(&local_key) { for (k, v) in values { merged_values.insert(k.clone(), (v.0, v.1, v.2, v.3, v.4)); } }

                    for (i, src) in archive.sources.iter().enumerate() {
                        if src.url.starts_with("nostr://") { continue; }
                        
                        let is_global = src.lat.is_none() && src.lon.is_none() && !src.url.contains("{lat}") && !src.url.contains("{lon}");
                        let (src_x, src_y, src_z) = if is_global {
                            (0.0, 0.0, 0.0)
                        } else if let (Some(lat), Some(lon)) = (src.lat, src.lon) {
                            geodetic_to_icrs(lat, lon, 0.0, q_t)
                        } else {
                            (x, y, z)
                        };

                        if !is_global && pos_key(src_x, src_y, src_z, src.res, q_t) != pos_key(x, y, z, src.res, q_t) { continue; }

                        let src_key = if is_global {
                            format!("global_{}", i)
                        } else {
                            format!("{}_{}", i, pos_key(src_x, src_y, src_z, src.res, q_t))
                        };
                        
                        if let Some((_, values)) = cache.get(&src_key) { for (k, v) in values { merged_values.insert(k.clone(), (v.0, v.1, v.2, v.3, v.4)); } }
                    }

                    if !merged_values.is_empty() {

                        let fields: Vec<(&str,f64,f64,f64,f64,f64)> = merged_values.iter().map(|(k,v)|(k.as_str(),v.0,v.1,v.2,v.3,v.4)).collect();
                        φ_obj(&mut out,&fields);
                    }

                    let obj_count=((out.len()-obj_pos-4)>0) as u32;
                    out[obj_pos..obj_pos+4].copy_from_slice(&obj_count.to_le_bytes());
                }
            }

            write_ws_binary(&mut stream,&out);
        }
    }
}

fn is_leap(y: u32) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }
fn φ_obj(out: &mut Vec<u8>, fields: &[(&str, f64, f64, f64, f64, f64)]) {
    let valid: Vec<&(&str,f64,f64,f64,f64,f64)> = fields.iter().filter(|(n,_,_,_,_,_)| !n.is_empty() && n.len() <= 255).collect();
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
    out.extend_from_slice(&0u32.to_le_bytes());
}

fn j2d_last_row(json: &JsonVal, col: &str) -> Option<f64> {
    if let JsonVal::Arr(arr) = json {
        if arr.len() < 2 { return None; }
        if let JsonVal::Arr(headers) = &arr[0] {
            let col_idx = headers.iter().position(|h| { if let JsonVal::Str(s) = h { s.eq_ignore_ascii_case(col) || s.starts_with(col) } else { false } })?;
            if let Some(JsonVal::Arr(last_row)) = arr.last() {
                if let Some(v) = last_row.get(col_idx) {
                    if let JsonVal::Num(n) = v { return Some(*n); }
                    if let JsonVal::Str(s) = v { return s.parse().ok(); }
                }
            }
        }
    }
    None
}

fn text_last_col(data: &str, col: &str) -> Option<f64> {
    let mut header_idx: Option<usize> = None;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        let stripped = trimmed.strip_prefix('#').unwrap_or(trimmed).trim();
        let cols = split_data_line(stripped);
        if header_idx.is_none() { if let Some(idx) = cols.iter().position(|c| c.eq_ignore_ascii_case(col) || c.starts_with(col)) { header_idx = Some(idx); } continue; }
    }
    let idx = header_idx?;
    for line in data.lines().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) { continue; }
        let cols = split_data_line(trimmed);
        if let Some(v) = cols.get(idx) { if let Ok(f) = v.trim_matches('"').parse::<f64>() { return Some(f); } }
    }
    None
}

fn text_vector(text: &str) -> Option<(f64, f64, f64)> {
    let unescaped = text.replace("\\n", "\n"); let mut last = None;
    for line in unescaped.lines() {
        let lx = line.find("X ="); let ly = line.find("Y ="); let lz = line.find("Z =");
        if let (Some(xp), Some(yp), Some(zp)) = (lx, ly, lz) {
            let xs = &line[xp+3..yp].trim(); let ys = &line[yp+3..zp].trim();
            let zs = &line[zp+3..].split_whitespace().next().unwrap_or("").trim();
            if let (Ok(xv), Ok(yv), Ok(zv)) = (xs.parse::<f64>(), ys.parse::<f64>(), zs.parse::<f64>()) { last = Some((xv, yv, zv)); }
        }
    }
    last
}

fn load_env() { if let Ok(content) = std::fs::read_to_string(".env") { for line in content.lines() { let line = line.trim(); if line.is_empty() || line.starts_with('#') { continue; } if let Some(eq) = line.find('=') { let key = line[..eq].trim(); let val = line[eq+1..].trim(); if std::env::var(key).is_err() { unsafe { std::env::set_var(key, val); } } } } } }

fn load_sources() -> Vec<SourceConfig> {
    let mut sources = Vec::new(); let content = std::fs::read_to_string("phi/sources.φ").unwrap_or_default();
    let mut cur_ttl: u64 = 0; let mut cur_res: i32 = 0; let mut cur_url = String::new();
    let mut cur_lat: Option<f64> = None; let mut cur_lon: Option<f64> = None; let mut cur_lat_str = String::new();
    let mut cur_format = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new(); let mut cur_headers: Vec<(String, String)> = Vec::new(); let mut active = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("```") { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }
        match parts[0] {
            "source" => {
                if active { 
                    let is_dynamic = cur_lat.is_none() && cur_lon.is_none() && (cur_url.contains("{lat}") || cur_url.contains("{lon}"));
                    let is_global = cur_lat.is_none() && cur_lon.is_none() && !is_dynamic;

                    if is_dynamic && cur_res == 0 {
                    } else {
                        let (final_lat, final_lon, final_res) = if is_dynamic {
                            (None, None, cur_res)
                        } else if is_global {
                            (Some(0.0), Some(0.0), -8)
                        } else {
                            let mut final_res = cur_res;
                            if final_res == 0 {
                                let decimals = cur_lat_str.split('.').last().unwrap_or("").len() as i32;
                                final_res = decimals; 
                            }
                            (cur_lat, cur_lon, final_res)
                        };
                        sources.push(SourceConfig { ttl: cur_ttl, res: final_res, url: cur_url.clone(), lat: final_lat, lon: final_lon, format: cur_format.clone(), extracts: cur_extracts.clone(), headers: cur_headers.clone() }); 
                    }
                }
                cur_ttl = 0; cur_res = 0; cur_url.clear(); cur_lat = None; cur_lon = None; cur_lat_str.clear(); cur_format.clear(); cur_extracts.clear(); cur_headers.clear(); active = true;
            }
            "url" => cur_url = line[4..].trim().to_string(),
            "ttl" => cur_ttl = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            "res" => cur_res = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            "format" => cur_format = parts.get(1).unwrap_or(&"json").to_string(),
            "lat" => { 
                cur_lat_str = parts.get(1).unwrap_or(&"").to_string();
                cur_lat = cur_lat_str.parse().ok();
            },
            "lon" => cur_lon = parts.get(1).and_then(|s| s.parse().ok()),
            "header" => { let rest = line[7..].trim(); if let Some(sp) = rest.find(' ') { cur_headers.push((rest[..sp].to_string(), rest[sp+1..].trim_matches('"').to_string())); } },
            "field" => { if parts.len()>=3 { cur_extracts.push(Extract::Field(parts[1].to_string(), parts[2].to_string())); } }
            "first" => { if parts.len()>=3 { cur_extracts.push(Extract::First(parts[1].to_string(), parts[2].to_string())); } }
            "last" => { if parts.len()>=3 { cur_extracts.push(Extract::Last(parts[1].to_string(), parts[2].to_string())); } }
            "count" => { if parts.len()>=3 { cur_extracts.push(Extract::Count(parts[1].to_string(), parts[2].to_string())); } }
            "sum" => { if parts.len()>=3 { cur_extracts.push(Extract::Sum(parts[1].to_string(), parts[2].to_string())); } }
            "last_row" => { if parts.len()>=3 { cur_extracts.push(Extract::LastRow(parts[1].to_string(), parts[2].to_string())); } }
            "vector" => { if parts.len()>=4 { cur_extracts.push(Extract::Vector(parts[1].to_string(), parts[2].to_string(), parts[3].to_string())); } }
            "path" => { if parts.len()>=3 { cur_extracts.push(Extract::Path(parts[1].to_string(), parts[2].to_string())); } }
            "deep" => { if parts.len()>=3 { cur_extracts.push(Extract::Deep(parts[1].to_string(), parts[2].to_string())); } }
            "deep_arr" => { if parts.len()>=3 { cur_extracts.push(Extract::DeepArr(parts[1].to_string(), parts[2].to_string())); } }
            "avg" => { if parts.len()>=3 { cur_extracts.push(Extract::Avg(parts[1].to_string(), parts[2].to_string())); } }
            "last_obj" => { let quoted = parse_quoted_args(&line[9..]); if quoted.len() >= 4 { cur_extracts.push(Extract::LastObj(quoted[0].clone(), quoted[1].clone(), quoted[2].clone(), quoted[3].clone())); } }
            "geojson" => { if parts.len() >= 5 && parts[1] == "events" { cur_extracts.push(Extract::GeojsonEvents { mag_key: parts.get(2).unwrap_or(&"mag").to_string(), min_mag: parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0), outputs: parts[4..].iter().map(|s| s.to_string()).collect() }); } }
            "map" => { if parts.len() >= 2 { cur_extracts.push(Extract::Map(parts[1].to_string(), Vec::new())); } }
            "field_in" => { if let Some(Extract::Map(_, fields)) = cur_extracts.last_mut() { if parts.len() >= 3 { fields.push((parts[1].to_string(), parts[2].to_string())); } } },
            _ => {}
        }
    }
    if active { 
        let is_dynamic = cur_lat.is_none() && cur_lon.is_none() && (cur_url.contains("{lat}") || cur_url.contains("{lon}"));
        let is_global = cur_lat.is_none() && cur_lon.is_none() && !is_dynamic;

        if !(is_dynamic && cur_res == 0) {
            let (final_lat, final_lon, final_res) = if is_dynamic {
                (None, None, cur_res)
            } else if is_global {
                (Some(0.0), Some(0.0), -8)
            } else {
                let mut final_res = cur_res;
                if final_res == 0 {
                    let decimals = cur_lat_str.split('.').last().unwrap_or("").len() as i32;
                    final_res = decimals;
                }
                (cur_lat, cur_lon, final_res)
            };
            sources.push(SourceConfig { ttl: cur_ttl, res: final_res, url: cur_url, lat: final_lat, lon: final_lon, format: cur_format, extracts: cur_extracts, headers: cur_headers }); 
        }
    }
    sources
}

fn parse_path(s: &str) -> String { let fl=s.lines().next().unwrap_or(""); let p: Vec<&str>=fl.split_whitespace().collect(); if p.len()>=2 { p[1].to_string() } else { "/".to_string() } }
fn parse_quoted_args(s: &str) -> Vec<String> {
    let mut result = Vec::new(); let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) { chars.next(); }
        if chars.peek().is_none() { break; }
        if *chars.peek().unwrap() == '"' {
            chars.next(); let mut val = String::new();
            while let Some(&c) = chars.peek() { if c == '"' { chars.next(); break; } val.push(c); chars.next(); }
            result.push(val);
        } else { let mut val = String::new(); while let Some(&c) = chars.peek() { if c.is_whitespace() { break; } val.push(c); chars.next(); } result.push(val); }
    }
    result
}

fn read_signal(s: &mut TcpStream) -> Option<String> {
    let mut buf=[0u8;8192]; let mut acc=Vec::new();
    loop { match s.read(&mut buf) { Ok(0)=>return None, Ok(n)=>{ acc.extend_from_slice(&buf[..n]); if acc.windows(4).any(|w|w==b"\r\n\r\n") { return Some(String::from_utf8_lossy(&acc).to_string()); } if acc.len()>65536 { return None; } } Err(_)=>return None } }
}

fn read_ws_frame_raw(stream: &mut TcpStream) -> Option<WsFrame> {
    let mut header = [0u8;2]; stream.read_exact(&mut header).ok()?;
    let opcode = header[0]&0x0f; let masked = (header[1]&0x80)!=0;
    let mut plen = (header[1]&0x7f) as usize;
    if plen==126 { let mut e=[0u8;2]; stream.read_exact(&mut e).ok()?; plen=u16::from_be_bytes(e) as usize; }
    else if plen==127 { let mut e=[0u8;8]; stream.read_exact(&mut e).ok()?; plen=u64::from_be_bytes(e) as usize; }
    let mut mk=[0u8;4]; if masked { stream.read_exact(&mut mk).ok()?; }
    let mut payload=vec![0u8;plen]; stream.read_exact(&mut payload).ok()?;
    if masked { for i in 0..payload.len() { payload[i]^=mk[i%4]; } }
    Some(WsFrame{opcode,payload})
}

fn render_url(template: &str, x: f64, y: f64, z: f64, tdb_secs: f64) -> String {
    let unix = tdb_secs + UNIX_J2000_OFFSET;
    let secs = unix as u64;
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days); let today = format!("{}-{:02}-{:02}", ty, tm, td);
    let (yy, ym, yd) = days_to_ymd(days - 1); let yesterday = format!("{}-{:02}-{:02}", yy, ym, yd);
    let (tmy, tmm, tmd) = days_to_ymd(days + 1); let tomorrow = format!("{}-{:02}-{:02}", tmy, tmm, tmd);
    let today_yyyymmdd = format!("{}_{:02}_{:02}", ty, tm, td);
    let today_nodashes = format!("{}{:02}{:02}", ty, tm, td);
    let hour_ago = { let dt = secs.saturating_sub(3600); let (h_y, h_m, h_d) = days_to_ymd(dt / 86400); let h_h = (dt % 86400) / 3600; let h_min = (dt % 3600) / 60; format!("{}-{:02}-{:02}T{:02}:{:02}:00", h_y, h_m, h_d, h_h, h_min) };
    let now_iso = { let n_h = (secs % 86400) / 3600; let n_min = (secs % 3600) / 60; format!("{}-{:02}-{:02}T{:02}:{:02}:00", ty, tm, td, n_h, n_min) };
    let week_ago = { let dt = secs.saturating_sub(604800); let (w_y, w_m, w_d) = days_to_ymd(dt / 86400); format!("{}-{:02}-{:02}", w_y, w_m, w_d) };
    let week_ago_nodashes = { let dt = secs.saturating_sub(604800); let (w_y, w_m, w_d) = days_to_ymd(dt / 86400); format!("{}{:02}{:02}", w_y, w_m, w_d) };
    let q_hour = (secs % 86400) / 3600; let q_minute = (secs % 3600) / 60;
    let unix_now = secs.to_string(); let unix_now_plus_3600 = (secs + 3600).to_string();
    
    let (lat, lon) = icrs_to_geodetic(x, y, z, tdb_secs);
    
    template
        .replace("{x}", &format!("{}", x)).replace("{y}", &format!("{}", y)).replace("{z}", &format!("{}", z))
        .replace("{lat}", &format!("{}", lat)).replace("{lon}", &format!("{}", lon))
        .replace("{lat_min}", &format!("{}", lat - (1.0 / Φ))).replace("{lat_max}", &format!("{}", lat + (1.0 / Φ)))
        .replace("{lon_min}", &format!("{}", lon - (1.0 / Φ))).replace("{lon_max}", &format!("{}", lon + (1.0 / Φ)))
        .replace("{today}", &today).replace("{yesterday}", &yesterday).replace("{tomorrow}", &tomorrow)
        .replace("{today_yyyymmdd}", &today_yyyymmdd).replace("{today_ymd}", &today_yyyymmdd).replace("{today_nodashes}", &today_nodashes)
        .replace("{t_start}", &yesterday).replace("{t_end}", &today)
        .replace("{now}", &now_iso).replace("{week_ago}", &week_ago).replace("{week_ago_nodashes}", &week_ago_nodashes)
        .replace("{today_plus_365}", &format!("{}-{:02}-{:02}", ty+1, tm, td))
        .replace("{lat_int}", &format!("{}", lat as i32)).replace("{lon_int}", &format!("{}", lon as i32))
        .replace("{hour_ago}", &hour_ago).replace("{year}", &ty.to_string()).replace("{month}", &tm.to_string()).replace("{day}", &td.to_string())
        .replace("{hour}", &format!("{:02}", q_hour)).replace("{minute}", &format!("{:02}", q_minute))
        .replace("{unix_now}", &unix_now).replace("{unix_now_plus_3600}", &unix_now_plus_3600)
        .replace("{nasa_key}", &std::env::var("NASA_KEY").unwrap_or_else(|_| "DEMO_KEY".to_string()))
}

fn sha1(input: &[u8]) -> [u8;20] {
    let mut h:[u32;5]=[0x67452301,0xEFCDAB89,0x98BADCFE,0x10325476,0xC3D2E1F0];
    let bl=(input.len() as u64)*8; let mut m=input.to_vec(); m.push(0x80);
    while m.len()%64!=56 { m.push(0); } m.extend_from_slice(&bl.to_be_bytes());
    for chunk in m.chunks(64) {
        let mut w=[0u32;80];
        for i in 0..16 { w[i]=u32::from_be_bytes([chunk[i*4],chunk[i*4+1],chunk[i*4+2],chunk[i*4+3]]); }
        for i in 16..80 { w[i]=(w[i-3]^w[i-8]^w[i-14]^w[i-16]).rotate_left(1); }
        let (mut a,mut b,mut c,mut d,mut e)=(h[0],h[1],h[2],h[3],h[4]);
        for i in 0..80 {
            let (f,k)=match i { 0..=19=>((b&c)|((!b)&d),0x5A827999), 20..=39=>(b^c^d,0x6ED9EBA1), 40..=59=>((b&c)|(b&d)|(c&d),0x8F1BBCDC), _=>(b^c^d,0xCA62C1D6) };
            let temp=a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e=d; d=c; c=b.rotate_left(30); b=a; a=temp;
        }
        h[0]=h[0].wrapping_add(a); h[1]=h[1].wrapping_add(b); h[2]=h[2].wrapping_add(c); h[3]=h[3].wrapping_add(d); h[4]=h[4].wrapping_add(e);
    }
    let mut r=[0u8;20]; for i in 0..5 { r[i*4..i*4+4].copy_from_slice(&h[i].to_be_bytes()); } r
}

fn split_data_line(line: &str) -> Vec<&str> {
    if line.contains(';') { line.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect() }
    else if line.contains(',') && line.split(',').count() > 2 { line.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect() }
    else { line.split_whitespace().collect() }
}

fn write_ws_binary(stream: &mut TcpStream, data: &[u8]) {
    let mut h=[0u8;10]; h[0]=0x82;
    if data.len()<=125 { h[1]=data.len() as u8; let _=stream.write_all(&h[..2]); }
    else if data.len()<=65535 { h[1]=126; let e=(data.len() as u16).to_be_bytes(); h[2]=e[0]; h[3]=e[1]; let _=stream.write_all(&h[..4]); }
    else { h[1]=127; let e=(data.len() as u64).to_be_bytes(); h[2..10].copy_from_slice(&e); let _=stream.write_all(&h); }
    let _=stream.write_all(data);
}

fn warm_cache(archive: Arc<Archive>) {
    loop {
        let positions: Vec<(String, f64)> = archive.active_positions.lock().unwrap_or_else(|e| e.into_inner()).iter().map(|(k,v)| (k.clone(), *v)).collect();
        if positions.is_empty() { let min_ttl = archive.sources.iter().map(|s| s.ttl).min().unwrap_or(60); thread::sleep(std::time::Duration::from_secs((min_ttl as f64 / Φ) as u64)); continue; }
        
        for (pos, query_t) in &positions {
            let (pos_x, pos_y, pos_z) = match parse_pos(pos) { Some(c) => c, None => continue };
            
            let needs: Vec<(usize, String, String, Vec<(String, String)>, u64)> = archive.sources.iter().enumerate()
                .filter(|(_, src)| !src.url.starts_with("nostr://"))
                .filter_map(|(i, src)| {
                    let is_global = src.lat.is_none() && src.lon.is_none() && !src.url.contains("{lat}") && !src.url.contains("{lon}");
                    let (src_x, src_y, src_z, render_x, render_y, render_z) = if is_global {
                        (0.0, 0.0, 0.0, pos_x, pos_y, pos_z)
                    } else if let (Some(lat), Some(lon)) = (src.lat, src.lon) {
                        let (x, y, z) = geodetic_to_icrs(lat, lon, 0.0, *query_t);
                        (x, y, z, pos_x, pos_y, pos_z)
                    } else {
                        (pos_x, pos_y, pos_z, pos_x, pos_y, pos_z)
                    };

                    if !is_global && pos_key(src_x, src_y, src_z, src.res, *query_t) != pos_key(pos_x, pos_y, pos_z, src.res, *query_t) { return None; }

                    let cache_key = if is_global {
                        format!("global_{}", i)
                    } else {
                        format!("{}_{}", i, pos_key(src_x, src_y, src_z, src.res, *query_t))
                    };
                    
                    let needs_fetch = { let cache = archive.data_cache.lock().unwrap_or_else(|e| e.into_inner()); match cache.get(&cache_key) { Some((ts, _)) => query_t - *ts >= src.ttl as f64, None => true } };
                    if needs_fetch { let url = render_url(&src.url, render_x, render_y, render_z, *query_t); let headers_rendered: Vec<(String, String)> = src.headers.iter().map(|(k, v)| (k.clone(), render_url(v, render_x, render_y, render_z, *query_t))).collect(); Some((i, cache_key, url, headers_rendered, src.ttl)) } else { None }
                }).collect();

            if needs.is_empty() { continue; }

            let results: Vec<(usize, String, Option<String>)> = thread::scope(|s| {
                let handles: Vec<_> = needs.iter().map(|&(i, ref cache_key, ref url, ref headers, ref ttl)| {
                    s.spawn(move || {
                        let body = fetch_with_headers(url, headers, *ttl);
                        (i, cache_key.clone(), body)
                    })
                }).collect();
                handles.into_iter().filter_map(|h| h.join().ok()).collect()
            });

            for (src_idx, cache_key, body_opt) in results {
                if let Some(body) = body_opt {
                    let src = &archive.sources[src_idx]; let mut extracted: HashMap<String, f64> = HashMap::new();
                    let parsed_json = if src.format == "json" || src.format.is_empty() { parse_json(&body) } else { None };

                    for ext in &src.extracts {
                        match ext {
                            Extract::Field(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jnum(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::First(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jarr_first(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::Last(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jarr_last(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::Count(k, n) => {
                                let v = if src.format == "csv" || k == "lines" { Some(body.lines().filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#')).count() as f64) } else { parsed_json.as_ref().and_then(|j| jarr_count(j, k)) };
                                if let Some(v) = v { extracted.insert(n.clone(), v); }
                            }
                            Extract::Sum(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jsum(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::LastRow(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = j2d_last_row(j, k) { extracted.insert(n.clone(), v); } } else { if let Some(v) = text_last_col(&body, k) { extracted.insert(n.clone(), v); } } }
                            Extract::Path(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jpath(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::Deep(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jdeep_find_num(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::DeepArr(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jpath(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::Avg(k, n) => { if let Some(ref j) = parsed_json { if let Some(v) = jarr_avg(j, k) { extracted.insert(n.clone(), v); } } }
                            Extract::Vector(nx, ny, nz) => { if let Some((vx, vy, vz)) = text_vector(&body) { extracted.insert(nx.clone(), vx); extracted.insert(ny.clone(), vy); extracted.insert(nz.clone(), vz); } }
                            Extract::LastObj(fk, fv, ek, n) => { if let Some(ref j) = parsed_json { if let JsonVal::Obj(map) = j { if let Some(JsonVal::Arr(arr)) = map.get(fk) { for v in arr.iter().rev() { if let JsonVal::Obj(o) = v { if let Some(JsonVal::Str(s)) = o.get(fv) { if s == fv { if let Some(val) = jnum(&JsonVal::Obj(o.clone()), ek) { extracted.insert(n.clone(), val); break; } } } } } } } } }
                            Extract::Map(arr_path, fields) => {
                                if let Some(ref j) = parsed_json {
                                    let mut current = j;
                                    let mut path_ok = true;
                                    for part in arr_path.split('.') {
                                        if let Ok(idx) = part.parse::<usize>() {
                                            if let JsonVal::Arr(arr) = current { current = match arr.get(idx) { Some(v) => v, None => { path_ok = false; break; } }; }
                                            else { path_ok = false; break; }
                                        } else {
                                            if let JsonVal::Obj(map) = current { current = match map.get(part) { Some(v) => v, None => { path_ok = false; break; } }; }
                                            else { path_ok = false; break; }
                                        }
                                    }
                                    if path_ok { if let JsonVal::Arr(arr) = current { for (idx, v) in arr.iter().enumerate() { if let JsonVal::Obj(o) = v { for (fk, fn_) in fields { if let Some(val) = jnum(&JsonVal::Obj(o.clone()), fk) { extracted.insert(format!("{}_{}", fn_, idx), val); } } } } } }
                                }
                            }
                            Extract::GeojsonEvents { mag_key, min_mag, outputs } => {
                                if outputs.len() >= 2 {
                                    let mut cache = archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                                    if let Some(ref j) = parsed_json { if let JsonVal::Arr(arr) = j { for v in arr.iter() { if let JsonVal::Obj(o) = v { let mut elo = 0.0; let mut ela = 0.0; let mut ed = 0.0; let mut mag = 0.0; let mut valid = false; if let Some(JsonVal::Obj(coords)) = o.get("geometry") { if let Some(JsonVal::Arr(c)) = coords.get("coordinates") { if c.len() >= 3 { if let JsonVal::Num(n) = c[0] { elo = n; } if let JsonVal::Num(n) = c[1] { ela = n; } if let JsonVal::Num(n) = c[2] { ed = n; } valid = true; } } } if valid { if let Some(m) = jnum(&JsonVal::Obj(o.clone()), mag_key) { mag = m; } if mag >= *min_mag { let (ev_x, ev_y, ev_z) = geodetic_to_icrs(ela, elo, 0.0, *query_t); let ev_key = format!("geojson_{}", pos_key(ev_x, ev_y, ev_z, 4, *query_t)); let mut ev_vals: HashMap<String, (f64, f64, f64, f64, f64)> = HashMap::new(); ev_vals.insert(outputs[0].clone(), (mag, *query_t, ev_x, ev_y, ev_z)); ev_vals.insert(outputs[1].clone(), (ed, *query_t, ev_x, ev_y, ev_z)); cache.insert(ev_key, (*query_t, ev_vals)); } } } } } }
                                }
                            }
                        }
                    }
                    if !extracted.is_empty() {
                        let extracted_with_t: HashMap<String, (f64, f64, f64, f64, f64)> = extracted.iter().map(|(k, v)| (k.clone(), (*v, *query_t, pos_x, pos_y, pos_z))).collect();
                        archive.data_cache.lock().unwrap_or_else(|e| e.into_inner()).insert(cache_key, (*query_t, extracted_with_t));
                    }
                }
            }
        }
        
        let min_ttl = archive.sources.iter().map(|s| s.ttl).min().unwrap_or(60);
        let max_ttl = archive.sources.iter().map(|s| s.ttl).max().unwrap_or(3600);
        let now_tdb = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64() - UNIX_J2000_OFFSET;
        let evict_thresh = now_tdb - max_ttl as f64 * 2.0;
        archive.data_cache.lock().unwrap_or_else(|e| e.into_inner()).retain(|_, (ts, _)| *ts > evict_thresh);
        thread::sleep(std::time::Duration::from_secs((min_ttl as f64 / (Φ * Φ)) as u64));
    }
}

fn main() {
    load_env();
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(1111);
    let archive = Arc::new(Archive {
        sources: load_sources(), 
        index_html: std::fs::read("static/index.html").unwrap_or_default(),
        constants_js: std::fs::read("static/constants.js").unwrap_or_default(),
        data_cache: Mutex::new(HashMap::new()), 
        active_positions: Mutex::new(HashMap::new()),
    });
    { let ar = Arc::clone(&archive); thread::spawn(move || warm_cache(ar)); }
    if let Ok(listener) = TcpListener::bind(format!("0.0.0.0:{}", port)) {
        for stream in listener.incoming() { 
            if let Ok(stream) = stream { 
                let ar = Arc::clone(&archive); 
                thread::spawn(move || handle_ingress(stream, ar)); 
            } 
        }
    }
}

