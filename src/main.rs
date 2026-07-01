#![allow(mixed_script_confusables)]
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const Φ: f64 = 1.618033988749895;
const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const J2000_EPOCH: f64 = 2451545.0;
const UNIX_JD_EPOCH: f64 = 2440587.5;
const EARTH_R_MIN: f64 = 6.0e6;
const EARTH_R_MAX: f64 = 7.5e6;

fn j2000_to_unix(t: f64) -> u64 {
    ((t + J2000_EPOCH - UNIX_JD_EPOCH) * 86400.0) as u64
}

fn unix_to_j2000(secs: u64) -> f64 {
    (secs as f64 / 86400.0) + UNIX_JD_EPOCH - J2000_EPOCH
}

#[derive(Clone)]
enum Extract {
    Field(String, String),
    First(String, String),
    Last(String, String),
    Count(String, String),
    LastRow(String, String),
    Vector(String, String, String),
    LastObj(String, String, String, String),
    GeojsonEvents { mag_key: String, min_mag: f64, outputs: Vec<String> },
    Path(String, String),
    Sum(String, String),
    Map(String, Vec<(String, String)>),
}

struct SourceConfig {
    ttl: u64,
    res: u8,
    url: String,
    lat: Option<f64>,
    lon: Option<f64>,
    format: String,
    extracts: Vec<Extract>,
    headers: Vec<(String, String)>,
}

struct Archive {
    sources: Vec<SourceConfig>,
    index_html: Vec<u8>,
    constants_js: Vec<u8>,
    data_cache: Mutex<HashMap<String, (u64, HashMap<String, (f64, f64)>)>>,
    active_positions: Mutex<HashMap<String, u64>>,
}

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
    let mut d = total_days as u32;
    let mut y = 1970u32;
    loop {
        let yd = if is_leap(y) { 366 } else { 365 };
        if d < yd { break; }
        d -= yd; y += 1;
    }
    let months: [u32; 12] = if is_leap(y) { [31,29,31,30,31,30,31,31,30,31,30,31] } else { [31,28,31,30,31,30,31,31,30,31,30,31] };
    let mut m = 0u32;
    while d >= months[m as usize] { d -= months[m as usize]; m += 1; }
    (y, m + 1, d + 1)
}

fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let a = WGS84_A; let f = WGS84_F; let b = a*(1.0-f);
    let e2 = f*(2.0-f); let ep2 = (a*a-b*b)/(b*b);
    let p = (x*x+y*y).sqrt();
    let theta = (z*a/(p*b)).atan2(1.0);
    let lat = (z+ep2*b*theta.sin().powi(3)).atan2(p-e2*a*theta.cos().powi(3));
    let lon = y.atan2(x);
    let n = a/(1.0-e2*lat.sin().powi(2)).sqrt();
    let alt = p/lat.cos()-n;
    (lat.to_degrees(), lon.to_degrees(), alt)
}

fn pos_key(lat: f64, lon: f64, res: u8) -> String {
    let lat_str = format!("{:.*}", res as usize, lat);
    let lon_str = format!("{:.*}", res as usize, lon);
    format!("{}_{}", lat_str, lon_str)
}

fn parse_pos(pos: &str) -> Option<(f64, f64)> {
    let parts: Vec<&str> = pos.split('_').collect();
    if parts.len() == 2 {
        if let (Ok(lat), Ok(lon)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            return Some((lat, lon));
        }
    }
    None
}

fn emit(s: &mut TcpStream, st: &str, ct: &str, b: &[u8]) { let _=s.write_all(format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",st,ct,b.len()).as_bytes()); let _=s.write_all(b); }
fn emit_void(s: &mut TcpStream) { let _=s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"); }
fn extract_header(s: &str, n: &str) -> Option<String> { for l in s.lines() { if let Some(c) = l.find(':') { if l[..c].trim().eq_ignore_ascii_case(n) { return Some(l[c+1..].trim().to_string()); } } } None }

fn fetch_with_headers(url: &str, headers: &[(String, String)], ttl: u64) -> Option<String> {
    let connect_t = ((ttl as f64) / (Φ * Φ * Φ)).max(1.0) as u64;
    let max_t = ((ttl as f64) / (Φ * Φ)).max(1.0) as u64;
    let mut cmd = Command::new("curl");
    cmd.arg("-s").arg("-k").arg("-m").arg(max_t.to_string()).arg("--connect-timeout").arg(connect_t.to_string());
    for (k, v) in headers { cmd.arg("-H").arg(format!("{}: {}", k, v)); }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() { Some(String::from_utf8_lossy(&output.stdout).to_string()) } else { None }
}

fn format_dormant_snapshot(c: &HashMap<String,(u32,u32)>) -> String {
    let mut o=String::new(); let mut k: Vec<&String>=c.keys().collect(); k.sort();
    for key in k { let (d,s)=c[key]; if d==0&&s==0 { o.push_str(&format!("dormant {}\n",key)); } else { o.push_str(&format!("dormant {} {} {}\n",key,d,s)); } }
    o
}

fn handle_observer(stream: TcpStream, dormant: Arc<Mutex<HashMap<String,(u32,u32)>>>, dormant_state: Arc<Mutex<String>>, archive: Arc<Archive>) {
    let mut s = stream; s.set_nodelay(true).ok();
    let signal = match read_signal(&mut s) { Some(r) => r, None => return };
    if signal.to_lowercase().contains("upgrade: websocket") { handle_pulse(s, &signal, dormant, dormant_state, archive); }
    else {
        let mut cur = signal;
        loop {
            let path = parse_path(&cur);
            if path == "/scan" {
                s.set_read_timeout(Some(std::time::Duration::from_millis(500))).ok();
                let mut body_buf = Vec::new();
                let header_end = cur.find("\r\n\r\n").map(|i| i + 4).unwrap_or(cur.len());
                body_buf.extend_from_slice(&cur.as_bytes()[header_end..]);
                let mut tmp = [0u8; 4096];
                loop {
                    match s.read(&mut tmp) {
                        Ok(0) => break,
                        Ok(n) => body_buf.extend_from_slice(&tmp[..n]),
                        Err(_) => break,
                    }
                }
                let body_str = String::from_utf8_lossy(&body_buf);
                let paths: Vec<&str> = body_str.trim().split('\n').filter(|x| !x.is_empty()).collect();
                {
                    let mut c = dormant.lock().unwrap_or_else(|e| e.into_inner());
                    for p in &paths { c.entry(p.to_string()).or_insert((1, 0)); }
                    rewrite_dormant(&c);
                    *dormant_state.lock().unwrap_or_else(|e| e.into_inner()) = format_dormant_snapshot(&c);
                }
                println!("SCAN: {} pathogens quarantined", paths.len());
                emit(&mut s, "200 OK", "text/plain", b"ok");
            } else if path.starts_with("/crash") {
                let body_start = cur.find("\r\n\r\n").map(|i| &cur[i+4..]).unwrap_or("");
                let body = body_start.trim();
                let log = format!("[{}] DISTRESS: {}\n", SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(), body);
                println!("{}", log.trim());
                let _ = std::fs::OpenOptions::new().create(true).append(true).open("crash.log").and_then(|mut f| f.write_all(log.as_bytes()));
                if body.starts_with("ACTUATOR_FAIL") {
                    let detail = body.find(':').map(|i| body[i+1..].trim()).unwrap_or("");
                    let actuator_path = detail.split_whitespace().next().unwrap_or("");
                    if !actuator_path.is_empty() {
                        let mut c = dormant.lock().unwrap_or_else(|e| e.into_inner());
                        c.entry(actuator_path.to_string()).or_insert((0,0)).0 += 1;
                        rewrite_dormant(&c);
                        *dormant_state.lock().unwrap_or_else(|e| e.into_inner()) = format_dormant_snapshot(&c);
                    }
                }
                emit(&mut s, "200 OK", "text/plain", b"ok");
            } else {
                match path.as_str() {
                    "/" => emit(&mut s, "200 OK", "text/html", &archive.index_html),
                    "/dormant" => { let b = dormant_state.lock().unwrap_or_else(|e| e.into_inner()).clone(); emit(&mut s, "200 OK", "text/plain", b.as_bytes()); }
                    "/time" => { let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64(); emit(&mut s, "200 OK", "text/plain", t.to_string().as_bytes()); }
                    "/constants.js" => emit(&mut s, "200 OK", "application/javascript", &archive.constants_js),
                    "/scan.html" => {
                        let html = std::fs::read("immune/scan.html").unwrap_or_else(|_| b"<h1>scan.html not found</h1>".to_vec());
                        emit(&mut s, "200 OK", "text/html", &html);
                    }
                    "/definitions.js" => {
                        let js = std::fs::read("immune/definitions.js").unwrap_or_else(|_| b"".to_vec());
                        emit(&mut s, "200 OK", "application/javascript", &js);
                    }
                    _ => { emit_void(&mut s); break; }
                }
            }
            match read_signal(&mut s) { Some(r) => cur = r, None => break }
        }
    }
}

fn handle_pulse(mut stream: TcpStream, signal: &str, dormant: Arc<Mutex<HashMap<String,(u32,u32)>>>, dormant_state: Arc<Mutex<String>>, archive: Arc<Archive>) {
    let key = match extract_header(signal,"Sec-WebSocket-Key") { Some(k)=>k, None=>return };
    let encoded = base64_encode(&sha1(&format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").into_bytes()));
    if stream.write_all(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", encoded).as_bytes()).is_err() { return; }
    let _=stream.set_nodelay(true);
    let mut last_stimulus: Vec<String> = Vec::new();
    let mut last_coord: (f64, f64, f64) = (f64::NAN, f64::NAN, f64::NAN);
    let mut last_key: String = String::new();
    let mut last_lat: f64 = 0.0;
    let mut last_lon: f64 = 0.0;

    while let Some(frame) = read_ws_frame_raw(&mut stream) {
        if frame.opcode==0x8 { break; }
        if frame.opcode==0x2 {
            if frame.payload.len()<12 { continue; }
            let id=u32::from_le_bytes(frame.payload[0..4].try_into().unwrap_or([0u8;4]));
            let input_count=u32::from_le_bytes(frame.payload[4..8].try_into().unwrap_or([0u8;4])) as usize;
            let mut off=8;

            {
                let t_frame = f64::from_le_bytes(frame.payload[8..16].try_into().unwrap_or([0u8;8]));
                let now = j2000_to_unix(t_frame);
                let mut cache = archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                for _ in 0..input_count {
                    if off+41 > frame.payload.len() { break; }
                    off+=8;
                    let x=f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;
                    let y=f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;
                    let z=f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;
                    let value=f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;
                    let name_len=frame.payload[off] as usize; off+=1;
                    let name=String::from_utf8_lossy(&frame.payload[off..off+name_len]).to_string();
                    off+=name_len;

                    if (x,y,z) != last_coord {
                        last_coord=(x,y,z);
                        let r = (x*x + y*y + z*z).sqrt();
                        if r > EARTH_R_MIN && r < EARTH_R_MAX {
                            let (lat, lon, _) = ecef_to_geodetic(x, y, z);
                            last_lat = lat;
                            last_lon = lon;
                            last_key = format!("local_{}", pos_key(lat, lon, 7));
                        } else { last_key.clear(); }
                    }
                    if !last_key.is_empty() {
                        cache.entry(last_key.clone()).or_insert_with(||(now,HashMap::new())).1.insert(name,(value,t_frame));
                    }
                }
            }

            if off+4 > frame.payload.len() { continue; }
            let query_count=u32::from_le_bytes(frame.payload[off..off+4].try_into().unwrap_or([0u8;4])) as usize;
            off+=4;

            let mut out=Vec::with_capacity(1024);
            out.extend_from_slice(&[0xCF, 0x86]); out.push(6u8);
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(query_count as u32).to_le_bytes());

            {
                let cache=archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                let mut active=archive.active_positions.lock().unwrap_or_else(|e| e.into_inner());

                for _ in 0..query_count {
                    if off+32 > frame.payload.len() { break; }
                    let q_t = f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;
                    let x=f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;
                    let y=f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;
                    let z=f64::from_le_bytes(frame.payload[off..off+8].try_into().unwrap_or([0u8;8])); off+=8;

                    if (x,y,z) != last_coord {
                        last_coord=(x,y,z);
                        let r = (x*x + y*y + z*z).sqrt();
                        if r > EARTH_R_MIN && r < EARTH_R_MAX {
                            let (lat, lon, _) = ecef_to_geodetic(x, y, z);
                            last_lat = lat;
                            last_lon = lon;
                        }
                    }


                    let q_unix = j2000_to_unix(q_t);
                    active.insert(format!("{}_{}", last_lat, last_lon), q_unix);

                    let obj_pos=out.len();
                    out.extend_from_slice(&0u32.to_le_bytes());

                    let mut merged_values: HashMap<String, (f64, f64)> = HashMap::new();

                    let local_key = format!("local_{}", pos_key(last_lat, last_lon, 7));
                    if let Some((_, values)) = cache.get(&local_key) {
                        for (k, v) in values { merged_values.insert(k.clone(), *v); }
                    }

                    for (i, src) in archive.sources.iter().enumerate() {
                        if src.url.starts_with("nostr://") { continue; }
                        let src_key = if let (Some(sl), Some(slo)) = (src.lat, src.lon) {
                            format!("{}_{}", i, pos_key(sl, slo, src.res))
                        } else if src.url.contains("{lat}") || src.url.contains("{lon}") {
                            format!("{}_{}", i, pos_key(last_lat, last_lon, src.res))
                        } else {
                            format!("{}_{}", i, src.url.split('?').next().unwrap_or(&src.url))
                        };
                        if let Some((_, values)) = cache.get(&src_key) {
                            for (k, v) in values { merged_values.insert(k.clone(), *v); }
                        }
                    }

                    if !merged_values.is_empty() {
                        let fields: Vec<(&str,f64,f64)> = merged_values.iter().map(|(k,v)|(k.as_str(),v.0,v.1)).collect();
                        φ_obj(&mut out,&fields);
                    }

                    let obj_count=((out.len()-obj_pos-4)>0) as u32;
                    out[obj_pos..obj_pos+4].copy_from_slice(&obj_count.to_le_bytes());
                }
            }

            if off < frame.payload.len() {
                let signal_count = frame.payload[off] as usize; off += 1;
                let mut new_resonant = Vec::new();
                let mut new_pulse = Vec::new();
                for _ in 0..signal_count {
                    if off + 2 > frame.payload.len() { break; }
                    let sig_type = frame.payload[off]; off += 1;
                    let path_len = frame.payload[off] as usize; off += 1;
                    if off + path_len > frame.payload.len() { break; }
                    let path = String::from_utf8_lossy(&frame.payload[off..off+path_len]).to_string();
                    off += path_len;
                    match sig_type {
                        1 => { new_resonant.push(path); }
                        0 => { new_pulse.push(path); }
                        _ => {}
                    }
                }
                if !new_resonant.is_empty() {
                    let mut c = dormant.lock().unwrap_or_else(|e| e.into_inner());
                    for p in &new_resonant { c.entry(p.clone()).or_insert((0,0)).1 += 1; }
                    rewrite_dormant(&c); *dormant_state.lock().unwrap_or_else(|e| e.into_inner()) = format_dormant_snapshot(&c);
                    last_stimulus.clear();
                }
                if !new_pulse.is_empty() {
                    last_stimulus = new_pulse;
                }
            }

            write_ws_binary(&mut stream,&out);
        }
    }
    if last_stimulus.len()==1 { let mut c=dormant.lock().unwrap_or_else(|e| e.into_inner()); c.entry(last_stimulus[0].clone()).or_insert((0,0)).0+=1; rewrite_dormant(&c); *dormant_state.lock().unwrap_or_else(|e| e.into_inner())=format_dormant_snapshot(&c); }
}

fn is_leap(y: u32) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }

fn φ_obj(out: &mut Vec<u8>, fields: &[(&str, f64, f64)]) {
    out.push(fields.len() as u8);
    for (name, _, _) in fields { out.push(name.len() as u8); out.extend_from_slice(name.as_bytes()); out.push(0u8); }
    for (_, val, t) in fields { out.extend_from_slice(&val.to_le_bytes()); out.extend_from_slice(&t.to_le_bytes()); }
    out.extend_from_slice(&0u32.to_le_bytes());
}

fn j2d_last_row(json: &str, col: &str) -> Option<f64> {
    if json.contains("[[") {
        let hs = json.find("[[")? + 1;
        let he = json[hs..].find("]")? + hs;
        let headers: Vec<&str> = json[hs+1..he].split(',').map(|s| s.trim().trim_matches('"')).collect();
        let col_idx = headers.iter().position(|h| *h == col)?;
        let trimmed = json.trim_end();
        let wo = &trimmed[..trimmed.len()-1];
        let lrs = wo.rfind("[")?;
        let lre = wo[lrs..].find("]")? + lrs;
        let vals: Vec<&str> = wo[lrs+1..lre].split(',').collect();
        vals.get(col_idx)?.trim().trim_matches('"').parse().ok()
    } else { text_last_col(json, col) }
}

fn jarr_count(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = &json[start..];
    let as_ = rest.find('[')?;
    let ae = rest[as_..].find(']')?;
    Some(rest[as_+1..ae].split(',').filter(|p| !p.trim().is_empty()).count() as f64)
}

fn jarr_first(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = &json[start..];
    let as_ = rest.find('[')?;
    let ae = rest[as_..].find(']')?;
    rest[as_+1..ae].split(',').next().and_then(|p| p.trim().parse().ok())
}

fn jarr_last(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let mut last_val = None;
    let mut search = json;
    while let Some(pos) = search.find(&pat) {
        let rest = &search[pos + pat.len()..];
        let trimmed = rest.trim_start();
        let end = trimmed.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(trimmed.len());
        if let Ok(v) = trimmed[..end].trim_matches('"').parse::<f64>() { last_val = Some(v); }
        search = &search[pos + pat.len()..];
    }
    last_val
}

fn jnum(json: &str, key: &str) -> Option<f64> {
    if key.contains('.') { return jpath(json, key); }
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = json[start..].trim_start();
    if rest.starts_with('[') {
        let close = rest.find(']')?;
        let inner = &rest[1..close];
        let nums: Vec<&str> = inner.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        return nums.last().and_then(|s| s.parse().ok());
    }
    let end = rest.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(rest.len());
    rest[..end].trim_matches('"').parse().ok()
}

fn jobj_last_match(json: &str, filter_key: &str, filter_val: &str, extract_key: &str) -> Option<f64> {
    let fv_quoted = format!("\"{}\":", filter_key);
    let fv_val = format!("\"{}\"", filter_val);
    let ek_pat = format!("\"{}\":", extract_key);
    let mut last_val = None;
    let mut search_start = 0;
    while let Some(fv_pos) = json[search_start..].find(&fv_quoted) {
        let abs_pos = search_start + fv_pos;
        let after_key = &json[abs_pos + fv_quoted.len()..];
        let after_trimmed = after_key.trim_start();
        if !after_trimmed.starts_with(&fv_val) {
            search_start = abs_pos + fv_quoted.len();
            continue;
        }
        let chunk_start = json[..abs_pos].rfind('{').unwrap_or(0);
        let chunk_end = json[abs_pos..].find('}').map(|e| abs_pos + e).unwrap_or(json.len());
        let chunk = &json[chunk_start..chunk_end];
        if let Some(ek_pos) = chunk.find(&ek_pat) {
            let rest = &chunk[ek_pos + ek_pat.len()..];
            let trimmed = rest.trim_start();
            let end = trimmed.find(|c: char| c == ',' || c == '}').unwrap_or(trimmed.len());
            if let Ok(v) = trimmed[..end].trim_matches('"').parse::<f64>() { last_val = Some(v); }
        }
        search_start = abs_pos + fv_quoted.len();
    }
    last_val
}

fn jpath(json: &str, path: &str) -> Option<f64> {
    let mut current = json;
    for part in path.split('.') {
        let part = part.trim();
        if part.is_empty() { continue; }
        if let Ok(idx) = part.parse::<usize>() {
            let trimmed = current.trim_start();
            if !trimmed.starts_with('[') { return None; }
            let mut depth: i32 = 0;
            let mut in_string = false;
            let mut escape = false;
            let mut elements: Vec<&str> = Vec::new();
            let mut start = 1usize;
            for (i, c) in trimmed.char_indices() {
                if i == 0 { continue; }
                if escape { escape = false; continue; }
                if c == '\\' && in_string { escape = true; continue; }
                if c == '"' { in_string = !in_string; continue; }
                if in_string { continue; }
                match c {
                    '[' | '{' => depth += 1,
                    ']' | '}' => {
                        if depth == 0 {
                            if start < i { elements.push(&trimmed[start..i]); }
                            break;
                        }
                        depth -= 1;
                    }
                    ',' if depth == 0 => {
                        elements.push(&trimmed[start..i]);
                        start = i + 1;
                    }
                    _ => {}
                }
            }
            let elem = elements.get(idx)?;
            current = elem.trim();
        } else {
            let pat = format!("\"{}\"", part);
            let key_pos = current.find(&pat)?;
            let after = &current[key_pos + pat.len()..];
            let trimmed = after.trim_start();
            let colon_pos = trimmed.find(':')?;
            let value_start = &trimmed[colon_pos + 1..];
            let value_trimmed = value_start.trim_start();
            if value_trimmed.starts_with('{') {
                let mut depth = 0; let mut end = 0;
                let mut in_string = false;
                let mut escape = false;
                for (i, c) in value_trimmed.char_indices() {
                    if escape { escape = false; continue; }
                    if c == '\\' && in_string { escape = true; continue; }
                    if c == '"' { in_string = !in_string; continue; }
                    if in_string { continue; }
                    if c == '{' { depth += 1; }
                    else if c == '}' { depth -= 1; if depth == 0 { end = i + 1; break; } }
                }
                current = &value_trimmed[..end];
            } else if value_trimmed.starts_with('[') {
                let mut depth = 0; let mut end = 0;
                let mut in_string = false;
                let mut escape = false;
                for (i, c) in value_trimmed.char_indices() {
                    if escape { escape = false; continue; }
                    if c == '\\' && in_string { escape = true; continue; }
                    if c == '"' { in_string = !in_string; continue; }
                    if in_string { continue; }
                    if c == '[' { depth += 1; }
                    else if c == ']' { depth -= 1; if depth == 0 { end = i + 1; break; } }
                }
                current = &value_trimmed[..end];
            } else {
                let end = value_trimmed.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(value_trimmed.len());
                let val_str = value_trimmed[..end].trim_matches('"');
                if let Ok(v) = val_str.parse::<f64>() { return Some(v); }
                current = val_str;
            }
        }
    }
    None
}

fn match_brace(s: &str) -> usize {
    if !s.starts_with('{') { return 0; }
    let mut depth = 0;
    let mut in_string = false;
    let mut escape = false;
    for (i, c) in s.char_indices() {
        if escape { escape = false; continue; }
        if c == '\\' && in_string { escape = true; continue; }
        if c == '"' { in_string = !in_string; continue; }
        if in_string { continue; }
        if c == '{' { depth += 1; }
        else if c == '}' { depth -= 1; if depth == 0 { return i + 1; } }
    }
    0
}

fn jpath_raw<'a>(json: &'a str, path: &str) -> Option<&'a str> {
    let mut current = json;
    for part in path.split('.') {
        let part = part.trim();
        if part.is_empty() { continue; }
        if let Ok(idx) = part.parse::<usize>() {
            let trimmed = current.trim_start();
            if !trimmed.starts_with('[') { return None; }
            let mut depth: i32 = 0;
            let mut in_string = false;
            let mut escape = false;
            let mut elements: Vec<&str> = Vec::new();
            let mut start = 1usize;
            for (i, c) in trimmed.char_indices() {
                if i == 0 { continue; }
                if escape { escape = false; continue; }
                if c == '\\' && in_string { escape = true; continue; }
                if c == '"' { in_string = !in_string; continue; }
                if in_string { continue; }
                match c {
                    '[' | '{' => depth += 1,
                    ']' | '}' => {
                        if depth == 0 {
                            if start < i { elements.push(&trimmed[start..i]); }
                            break;
                        }
                        depth -= 1;
                    }
                    ',' if depth == 0 => {
                        elements.push(&trimmed[start..i]);
                        start = i + 1;
                    }
                    _ => {}
                }
            }
            let elem = elements.get(idx)?;
            current = elem.trim();
        } else {
            let pat = format!("\"{}\"", part);
            let key_pos = current.find(&pat)?;
            let after = &current[key_pos + pat.len()..];
            let trimmed = after.trim_start();
            let colon_pos = trimmed.find(':')?;
            let value_start = &trimmed[colon_pos + 1..];
            let value_trimmed = value_start.trim_start();
            if value_trimmed.starts_with('{') {
                let end = match_brace(value_trimmed);
                if end == 0 { return None; }
                current = &value_trimmed[..end];
            } else if value_trimmed.starts_with('[') {
                let mut depth = 0; let mut end = 0;
                let mut in_string = false;
                let mut escape = false;
                for (i, c) in value_trimmed.char_indices() {
                    if escape { escape = false; continue; }
                    if c == '\\' && in_string { escape = true; continue; }
                    if c == '"' { in_string = !in_string; continue; }
                    if in_string { continue; }
                    if c == '[' { depth += 1; }
                    else if c == ']' { depth -= 1; if depth == 0 { end = i + 1; break; } }
                }
                current = &value_trimmed[..end];
            } else {
                let end = value_trimmed.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(value_trimmed.len());
                current = &value_trimmed[..end];
            }
        }
    }
    Some(current)
}

fn jsum(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let mut sum = 0.0;
    let mut found = false;
    let mut search = json;
    while let Some(pos) = search.find(&pat) {
        let rest = &search[pos + pat.len()..];
        let trimmed = rest.trim_start();
        let end = trimmed.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(trimmed.len());
        if let Ok(v) = trimmed[..end].parse::<f64>() { sum += v; found = true; }
        search = &search[pos + pat.len()..];
    }
    if found { Some(sum) } else { None }
}

fn load_env() {
    if let Ok(content) = std::fs::read_to_string(".env") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some(eq) = line.find('=') {
                let key = line[..eq].trim();
                let val = line[eq+1..].trim();
                if std::env::var(key).is_err() { unsafe { std::env::set_var(key, val); } }
            }
        }
    }
}

fn load_dormant() -> HashMap<String,(u32,u32)> {
    let mut c=HashMap::new();
    if let Ok(content)=std::fs::read_to_string("immune/dormant") {
        for line in content.lines() { let p: Vec<&str>=line.split_whitespace().collect(); if p.len()>=2&&p[0]=="dormant" { c.insert(p[1].to_string(),(if p.len()>=3{p[2].parse().unwrap_or(0)}else{0}, if p.len()>=4{p[3].parse().unwrap_or(0)}else{0})); } }
    }
    c
}

fn load_sources() -> Vec<SourceConfig> {
    let mut sources = Vec::new();
    let content = std::fs::read_to_string("φ/sources.φ").unwrap_or_default();
    let mut cur_ttl: u64 = 0;
    let mut cur_res: u8 = 2;
    let mut cur_url = String::new();
    let mut cur_lat: Option<f64> = None;
    let mut cur_lon: Option<f64> = None;
    let mut cur_format = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new();
    let mut cur_headers: Vec<(String, String)> = Vec::new();
    let mut active = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("```") { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }
        match parts[0] {
            "source" => {
                if active { sources.push(SourceConfig { ttl: cur_ttl, res: cur_res, url: cur_url.clone(), lat: cur_lat, lon: cur_lon, format: cur_format.clone(), extracts: cur_extracts.clone(), headers: cur_headers.clone() }); }
                cur_ttl = 0; cur_res = 2; cur_url.clear(); cur_lat = None; cur_lon = None; cur_format.clear(); cur_extracts.clear(); cur_headers.clear(); active = true;
            }
            "url" => cur_url = line[4..].trim().to_string(),
            "ttl" => cur_ttl = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
            "res" => cur_res = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(2),
            "format" => cur_format = parts.get(1).unwrap_or(&"json").to_string(),
            "lat" => cur_lat = parts.get(1).and_then(|s| s.parse().ok()),
            "lon" => cur_lon = parts.get(1).and_then(|s| s.parse().ok()),
            "header" => {
                let rest = line[7..].trim();
                if let Some(sp) = rest.find(' ') { cur_headers.push((rest[..sp].to_string(), rest[sp+1..].trim_matches('"').to_string())); }
            },
            "field" => { if parts.len()>=3 { cur_extracts.push(Extract::Field(parts[1].to_string(), parts[2].to_string())); } }
            "first" => { if parts.len()>=3 { cur_extracts.push(Extract::First(parts[1].to_string(), parts[2].to_string())); } }
            "last" => { if parts.len()>=3 { cur_extracts.push(Extract::Last(parts[1].to_string(), parts[2].to_string())); } }
            "count" => { if parts.len()>=3 { cur_extracts.push(Extract::Count(parts[1].to_string(), parts[2].to_string())); } }
            "sum" => { if parts.len()>=3 { cur_extracts.push(Extract::Sum(parts[1].to_string(), parts[2].to_string())); } }
            "last_row" => { if parts.len()>=3 { cur_extracts.push(Extract::LastRow(parts[1].to_string(), parts[2].to_string())); } }
            "vector" => { if parts.len()>=4 { cur_extracts.push(Extract::Vector(parts[1].to_string(), parts[2].to_string(), parts[3].to_string())); } }
            "path" => { if parts.len()>=3 { cur_extracts.push(Extract::Path(parts[1].to_string(), parts[2].to_string())); } }
            "last_obj" => {
                let quoted = parse_quoted_args(&line[9..]);
                if quoted.len() >= 4 { cur_extracts.push(Extract::LastObj(quoted[0].clone(), quoted[1].clone(), quoted[2].clone(), quoted[3].clone())); }
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
                    cur_extracts.push(Extract::Map(parts[1].to_string(), Vec::new()));
                }
            }
            "field_in" => {
                if let Some(Extract::Map(_, fields)) = cur_extracts.last_mut() {
                    if parts.len() >= 3 { fields.push((parts[1].to_string(), parts[2].to_string())); }
                }
            },
            _ => {}
        }
    }
    if active { sources.push(SourceConfig { ttl: cur_ttl, res: cur_res, url: cur_url, lat: cur_lat, lon: cur_lon, format: cur_format, extracts: cur_extracts, headers: cur_headers }); }
    sources
}

fn parse_path(s: &str) -> String { let fl=s.lines().next().unwrap_or(""); let p: Vec<&str>=fl.split_whitespace().collect(); if p.len()>=2 { p[1].to_string() } else { "/".to_string() } }

fn parse_quoted_args(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) { chars.next(); }
        if chars.peek().is_none() { break; }
        if *chars.peek().unwrap() == '"' {
            chars.next();
            let mut val = String::new();
            while let Some(&c) = chars.peek() { if c == '"' { chars.next(); break; } val.push(c); chars.next(); }
            result.push(val);
        } else {
            let mut val = String::new();
            while let Some(&c) = chars.peek() { if c.is_whitespace() { break; } val.push(c); chars.next(); }
            result.push(val);
        }
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

fn render_url(template: &str, lat: f64, lon: f64, query_t: f64) -> String {
    let secs = j2000_to_unix(query_t);
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days);
    let today = format!("{}-{:02}-{:02}", ty, tm, td);
    let (yy, ym, yd) = days_to_ymd(days - 1);
    let yesterday = format!("{}-{:02}-{:02}", yy, ym, yd);
    let (tmy, tmm, tmd) = days_to_ymd(days + 1);
    let tomorrow = format!("{}-{:02}-{:02}", tmy, tmm, tmd);
    let today_yyyymmdd = format!("{}_{:02}_{:02}", ty, tm, td);
    let hour_ago = {
        let dt = secs.saturating_sub(3600);
        let (h_y, h_m, h_d) = days_to_ymd(dt / 86400);
        let h_h = (dt % 86400) / 3600;
        let h_min = (dt % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", h_y, h_m, h_d, h_h, h_min)
    };
    let q_hour = (secs % 86400) / 3600;
    let q_minute = (secs % 3600) / 60;
    let unix_now = secs.to_string();
    let unix_now_plus_3600 = (secs + 3600).to_string();
    template
        .replace("{lat}", &format!("{}", lat))
        .replace("{lon}", &format!("{}", lon))
        .replace("{lat_min}", &format!("{}", lat - (1.0 / Φ)))
        .replace("{lat_max}", &format!("{}", lat + (1.0 / Φ)))
        .replace("{lon_min}", &format!("{}", lon - (1.0 / Φ)))
        .replace("{lon_max}", &format!("{}", lon + (1.0 / Φ)))
        .replace("{today}", &today)
        .replace("{yesterday}", &yesterday)
        .replace("{tomorrow}", &tomorrow)
        .replace("{today_yyyymmdd}", &today_yyyymmdd)
        .replace("{today_ymd}", &today_yyyymmdd)
        .replace("{t_start}", &yesterday)
        .replace("{t_end}", &today)
        .replace("{today_plus_365}", &format!("{}-{:02}-{:02}", ty+1, tm, td))
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
        .replace("{nasa_key}", &std::env::var("NASA_KEY").unwrap_or_else(|_| "DEMO_KEY".to_string()))
}

fn rewrite_dormant(c: &HashMap<String,(u32,u32)>) {
    let _=std::fs::write("immune/dormant", format_dormant_snapshot(c));
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

fn text_last_col(data: &str, col: &str) -> Option<f64> {
    let mut header_idx: Option<usize> = None;
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        let stripped = trimmed.strip_prefix('#').unwrap_or(trimmed).trim();
        let cols = split_data_line(stripped);
        if header_idx.is_none() {
            if let Some(idx) = cols.iter().position(|c| c.eq_ignore_ascii_case(col) || c.starts_with(col)) { header_idx = Some(idx); }
            continue;
        }
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
    let unescaped = text.replace("\\n", "\n");
    let mut last = None;
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

fn write_ws_binary(stream: &mut TcpStream, data: &[u8]) {
    let mut h=[0u8;10]; h[0]=0x82;
    if data.len()<=125 { h[1]=data.len() as u8; let _=stream.write_all(&h[..2]); }
    else if data.len()<=65535 { h[1]=126; let e=(data.len() as u16).to_be_bytes(); h[2]=e[0]; h[3]=e[1]; let _=stream.write_all(&h[..4]); }
    else { h[1]=127; let e=(data.len() as u64).to_be_bytes(); h[2..10].copy_from_slice(&e); let _=stream.write_all(&h); }
    let _=stream.write_all(data);
}

fn warm_cache(archive: Arc<Archive>) {
    loop {
        let positions: Vec<(String, u64)> = archive.active_positions.lock().unwrap_or_else(|e| e.into_inner()).iter().map(|(k,v)| (k.clone(), *v)).collect();
        if positions.is_empty() {
            let min_ttl = archive.sources.iter().map(|s| s.ttl).min().unwrap_or(60);
            thread::sleep(std::time::Duration::from_secs((min_ttl as f64 / Φ) as u64));
            continue;
        }
        for (pos, query_t) in &positions {
            let (pos_lat, pos_lon) = match parse_pos(pos) { Some(c) => c, None => continue };
            let j2000_t = unix_to_j2000(*query_t);
            let needs: Vec<(usize, String, String, Vec<(String, String)>, u64)> = archive.sources.iter().enumerate()
                .filter(|(_, src)| !src.url.starts_with("nostr://"))
                .filter_map(|(i, src)| {
                    let (cache_key, render_lat, render_lon) = if let (Some(sl), Some(slo)) = (src.lat, src.lon) {
                        let key = format!("{}_{}", i, pos_key(sl, slo, src.res));
                        (key, sl, slo)
                    } else if src.url.contains("{lat}") || src.url.contains("{lon}") {
                        let key = format!("{}_{}", i, pos_key(pos_lat, pos_lon, src.res));
                        (key, pos_lat, pos_lon)
                    } else {
                        let key = format!("{}_{}", i, src.url.split('?').next().unwrap_or(&src.url));
                        (key, pos_lat, pos_lon)
                    };
                    let needs_fetch = {
                        let cache = archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                        match cache.get(&cache_key) {
                            Some((ts, _)) => query_t.saturating_sub(*ts) >= src.ttl,
                            None => true,
                        }
                    };
                    if needs_fetch {
                        let url = render_url(&src.url, render_lat, render_lon, j2000_t);
                        let headers_rendered: Vec<(String, String)> = src.headers.iter()
                            .map(|(k, v)| (k.clone(), render_url(v, render_lat, render_lon, j2000_t))).collect();
                        Some((i, cache_key, url, headers_rendered, src.ttl))
                    } else { None }
                }).collect();
            let n_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
            let chunk_size = (needs.len() + n_threads - 1) / n_threads.max(1);
            let results: Vec<(usize, String, Option<String>)> = thread::scope(|s| {
                let handles: Vec<_> = needs.chunks(chunk_size.max(1)).map(|chunk| {
                    s.spawn(|| {
                        chunk.iter().filter_map(|&(i, ref cache_key, ref url, ref headers, ref ttl)| {
                            let body = fetch_with_headers(url, headers, *ttl);
                            Some((i, cache_key.clone(), body))
                        }).collect::<Vec<_>>()
                    })
                }).collect();
                handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
            });
            for (src_idx, cache_key, body_opt) in results {
                if let Some(body) = body_opt {
                    let src = &archive.sources[src_idx];
                    let mut extracted: HashMap<String, f64> = HashMap::new();
                    for ext in &src.extracts {
                        match ext {
                            Extract::Field(k, n) => { if let Some(v) = jnum(&body, k) { extracted.insert(n.clone(), v); } }
                            Extract::First(k, n) => { if let Some(v) = jarr_first(&body, k) { extracted.insert(n.clone(), v); } }
                            Extract::Last(k, n) => { if let Some(v) = jarr_last(&body, k) { extracted.insert(n.clone(), v); } }
                            Extract::Count(k, n) => {
                let v = if src.format == "csv" || k == "lines" {
                    Some(body.lines().filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#')).count() as f64)
                } else {
                    jarr_count(&body, k)
                };
                if let Some(v) = v { extracted.insert(n.clone(), v); }
            }
                            Extract::Sum(k, n) => { if let Some(v) = jsum(&body, k) { extracted.insert(n.clone(), v); } }
                            Extract::LastRow(k, n) => { if let Some(v) = j2d_last_row(&body, k) { extracted.insert(n.clone(), v); } }
                            Extract::Path(k, n) => { if let Some(v) = jpath(&body, k) { extracted.insert(n.clone(), v); } }
                            Extract::Vector(nx, ny, nz) => {
                                if let Some((vx, vy, vz)) = text_vector(&body) {
                                    extracted.insert(nx.clone(), vx);
                                    extracted.insert(ny.clone(), vy);
                                    extracted.insert(nz.clone(), vz);
                                }
                            }
                            Extract::LastObj(fk, fv, ek, n) => {
                                if let Some(v) = jobj_last_match(&body, fk, fv, ek) { extracted.insert(n.clone(), v); }
                            }
                            Extract::Map(arr_path, fields) => {
                                if let Some(arr_str) = jpath_raw(&body, arr_path) {
                                    let mut idx = 0;
                                    let mut search = arr_str.trim_start();
                                    while search.starts_with('{') {
                                        let end = match_brace(search);
                                        if end == 0 { break; }
                                        let obj = &search[..end];
                                        for (fk, fn_) in fields {
                                            if let Some(v) = jnum(obj, fk) {
                                                extracted.insert(format!("{}_{}", fn_, idx), v);
                                            }
                                        }
                                        idx += 1;
                                        search = search[end..].trim_start();
                                        if search.starts_with(',') { search = search[1..].trim_start(); }
                                    }
                                }
                            }
                            Extract::GeojsonEvents { mag_key, min_mag, outputs } => {
                                if outputs.len() >= 2 {
                                    let mut cache = archive.data_cache.lock().unwrap_or_else(|e| e.into_inner());
                                    let mut search = &body[..];
                                    while let Some(cs) = search.find("\"coordinates\":[") {
                                        let csi = cs + "\"coordinates\":[".len();
                                        let cei = match search[csi..].find(']') { Some(e) => csi + e, None => break };
                                        let parts: Vec<&str> = search[csi..cei].split(',').collect();
                                        if parts.len() >= 3 {
                                            if let (Ok(elo), Ok(ela), Ok(ed)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>(), parts[2].trim().parse::<f64>()) {
                                                let ac = &search[cei..];
                                                if let Some(ms) = ac.find(&format!("\"{}\":", mag_key)) {
                                                    let rest = &ac[ms + mag_key.len() + 3..];
                                                    let vend = rest.find(|c: char| c == ',' || c == '}').unwrap_or(rest.len());
                                                    if let Ok(mag) = rest[..vend].trim().parse::<f64>() {
                                                        if mag >= *min_mag {
                                                            let ev_key = format!("geojson_{}", pos_key(ela, elo, 4));
                                                            let mut ev_vals: HashMap<String, (f64, f64)> = HashMap::new();
                                                            ev_vals.insert(outputs[0].clone(), (mag, j2000_t));
                                                            ev_vals.insert(outputs[1].clone(), (ed, j2000_t));
                                                            cache.insert(ev_key, (*query_t, ev_vals));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        search = &search[cei..];
                                    }
                                }
                            }
                        }
                    }
                    if !extracted.is_empty() {
                        let extracted_with_t: HashMap<String, (f64, f64)> = extracted.iter().map(|(k, v)| (k.clone(), (*v, j2000_t))).collect();
                        archive.data_cache.lock().unwrap_or_else(|e| e.into_inner()).insert(cache_key, (*query_t, extracted_with_t));
                    }
                }
            }
        }
        let min_ttl = archive.sources.iter().map(|s| s.ttl).min().unwrap_or(60);
        let max_ttl = archive.sources.iter().map(|s| s.ttl).max().unwrap_or(3600);
        let now_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let evict_thresh = now_secs.saturating_sub(max_ttl * 2);
        archive.data_cache.lock().unwrap_or_else(|e| e.into_inner()).retain(|_, (ts, _)| *ts > evict_thresh);
        thread::sleep(std::time::Duration::from_secs((min_ttl as f64 / (Φ * Φ)) as u64));
    }
}

fn main() {
    load_env();
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3571);
    let archive = Arc::new(Archive {
        sources: load_sources(),
        index_html: std::fs::read("static/index.html").unwrap_or_default(),
        constants_js: std::fs::read("static/constants.js").unwrap_or_default(),
        data_cache: Mutex::new(HashMap::new()),
        active_positions: Mutex::new(HashMap::new()),
    });
    {
        let ar = Arc::clone(&archive);
        thread::spawn(move || warm_cache(ar));
    }
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let dormant = Arc::new(Mutex::new(load_dormant()));
    let dormant_state = Arc::new(Mutex::new(format_dormant_snapshot(&dormant.lock().unwrap_or_else(|e| e.into_inner()))));
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let im = Arc::clone(&dormant); let ds = Arc::clone(&dormant_state); let ar = Arc::clone(&archive);
            thread::spawn(move || handle_observer(stream, im, ds, ar));
        }
    }
}
