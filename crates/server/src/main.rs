use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

struct Archive {
    idx: Vec<u8>,
    dat: Vec<u8>,
    index_html: Vec<u8>,
    world_js: Vec<u8>,
}

fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let a = 6378137.0_f64;
    let f = 1.0 / 298.257223563;
    let b = a * (1.0 - f);
    let e2 = f * (2.0 - f);
    let ep2 = (a * a - b * b) / (b * b);
    let p = (x * x + y * y).sqrt();
    let theta = (z * a / (p * b)).atan2(1.0);
    let lat = (z + ep2 * b * theta.sin().powi(3)).atan2(p - e2 * a * theta.cos().powi(3));
    let lon = y.atan2(x);
    let n = a / (1.0 - e2 * lat.sin().powi(2)).sqrt();
    let alt = p / lat.cos() - n;
    (lat.to_degrees(), lon.to_degrees(), alt)
}

fn http_get(host: &str, path: &str) -> Option<String> {
    let mut stream = TcpStream::connect((host, 80)).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let req = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
    stream.write_all(req.as_bytes()).ok()?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    let s = String::from_utf8_lossy(&buf);
    if let Some(pos) = s.find("\r\n\r\n") { Some(s[pos + 4..].to_string()) } else { None }
}

fn jnum(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = json[start..].trim_start();
    let end = rest.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(rest.len());
    rest[..end].parse().ok()
}

fn jarr_first(json: &str, key: &str) -> Option<Vec<f64>> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = &json[start..];
    let as_ = rest.find('[')?;
    let ae = rest[as_..].find(']')?;
    let inner = &rest[as_ + 1..ae];
    let mut vals = Vec::new();
    for p in inner.split(',') { if let Ok(v) = p.trim().parse() { vals.push(v); } }
    if vals.is_empty() { None } else { Some(vals) }
}

fn is_obj(out: &mut Vec<u8>, fields: &[(&str, f64)]) {
    out.push(fields.len() as u8);
    for (name, _) in fields {
        out.push(name.len() as u8);
        out.extend_from_slice(name.as_bytes());
        out.push(0u8);
    }
    for (_, val) in fields { out.extend_from_slice(&val.to_le_bytes()); }
    out.extend_from_slice(&0u32.to_le_bytes());
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in input.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        result.push(TABLE[(b[0] >> 2) as usize] as char);
        result.push(TABLE[(((b[0] & 0x03) << 4) | (b[1] >> 4)) as usize] as char);
        if chunk.len() > 1 {
            result.push(TABLE[(((b[1] & 0x0f) << 2) | (b[2] >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(TABLE[(b[2] & 0x3f) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

fn extract_header(signal: &str, name: &str) -> Option<String> {
    for line in signal.lines() {
        if let Some(colon) = line.find(':') {
            if line[..colon].trim().eq_ignore_ascii_case(name) {
                return Some(line[colon + 1..].trim().to_string());
            }
        }
    }
    None
}

fn extract_json_value(msg: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", key);
    let start = msg.find(&pattern)? + pattern.len();
    let end = msg[start..].find('"')? + start;
    Some(msg[start..end].to_string())
}

fn handle_observer(stream: TcpStream, immunity: Arc<Mutex<HashMap<String, (u32, u32)>>>, immunity_str: Arc<Mutex<String>>, archive: Arc<Archive>) {
    let mut stream_ref = stream;
    stream_ref.set_nodelay(true).ok();
    let signal = match read_signal(&mut stream_ref) {
        Some(r) => r,
        None => return,
    };
    if signal.to_lowercase().contains("upgrade: websocket") {
        handle_pulse(stream_ref, &signal, immunity, immunity_str, archive);
    } else {
        let mut cur_signal = signal;
        loop {
            let path = parse_path(&cur_signal);
            match path.as_str() {
                "/" => emit(&mut stream_ref, "200 OK", "text/html", &archive.index_html),
                "/immunity" => {
                    let body = immunity_str.lock().unwrap().clone();
                    emit(&mut stream_ref, "200 OK", "text/plain", body.as_bytes());
                }
                "/time" => {
                    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64();
                    emit(&mut stream_ref, "200 OK", "text/plain", t.to_string().as_bytes());
                }
                "/world.js" => emit(&mut stream_ref, "200 OK", "application/javascript", &archive.world_js),
                _ => { emit_void(&mut stream_ref); break; }
            }
            match read_signal(&mut stream_ref) { Some(r) => cur_signal = r, None => break }
        }
    }
}

fn weave(payload: &[u8], archive: &Archive) -> Vec<u8> {
    if payload.len() < 33 { return Vec::new(); }
    let t = f64::from_le_bytes(payload[0..8].try_into().unwrap_or([0u8; 8]));
    let x = f64::from_le_bytes(payload[8..16].try_into().unwrap_or([0u8; 8]));
    let y = f64::from_le_bytes(payload[16..24].try_into().unwrap_or([0u8; 8]));
    let z = f64::from_le_bytes(payload[24..32].try_into().unwrap_or([0u8; 8]));

    let mut out = Vec::new();
    out.extend_from_slice(b"IS");
    out.push(2u8);
    let mut obj_count: u32 = 0;
    let obj_count_pos = out.len();
    out.extend_from_slice(&0u32.to_le_bytes());

    let r2 = x * x + y * y + z * z;
    let r = r2.sqrt();
    let on_earth = r > 6.3e6 && r < 6.5e6;

    if on_earth {
        let (lat, lon, _alt) = ecef_to_geodetic(x, y, z);
        let lat_s = format!("{:.4}", lat);
        let lon_s = format!("{:.4}", lon);

        if let Some(body) = http_get("api.open-meteo.com",
            &format!("/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,surface_pressure&elevation=yes", lat_s, lon_s))
        {
            if let Some(temp) = jnum(&body, "temperature_2m") {
                is_obj(&mut out, &[("atmosphere_temp", temp)]); obj_count += 1;
            }
            if let Some(wind) = jnum(&body, "wind_speed_10m") {
                is_obj(&mut out, &[("atmosphere_wind", wind)]); obj_count += 1;
            }
            if let Some(pres) = jnum(&body, "surface_pressure") {
                is_obj(&mut out, &[("atmosphere_pressure", pres)]); obj_count += 1;
            }
            if let Some(elev) = jnum(&body, "elevation") {
                is_obj(&mut out, &[("topography_elevation", elev)]); obj_count += 1;
            }
        }

        if let Some(body) = http_get("api.open-meteo.com",
            &format!("/v1/forecast?latitude={}&longitude={}&hourly=soil_temperature_0cm,soil_moisture_0_to_1cm", lat_s, lon_s))
        {
            if let Some(arr) = jarr_first(&body, "soil_temperature_0cm") {
                if let Some(&v) = arr.first() {
                    is_obj(&mut out, &[("biosphere_soil_temp", v)]); obj_count += 1;
                }
            }
            if let Some(arr) = jarr_first(&body, "soil_moisture_0_to_1cm") {
                if let Some(&v) = arr.first() {
                    is_obj(&mut out, &[("biosphere_soil_moisture", v)]); obj_count += 1;
                }
            }
        }

        if let Some(body) = http_get("earthquake.usgs.gov",
            "/earthquakes/feed/v1.0/summary/all_hour.geojson")
        {
            let mut search = &body[..];
            let mut found = false;
            while let Some(coords_start) = search.find("\"coordinates\":[") {
                let cs = coords_start + "\"coordinates\":[".len();
                let ce = match search[cs..].find(']') { Some(e) => cs + e, None => break };
                let inner = &search[cs..ce];
                let parts: Vec<&str> = inner.split(',').collect();
                if parts.len() >= 3 {
                    let eq_lon: f64 = parts[0].trim().parse().unwrap_or(0.0);
                    let eq_lat: f64 = parts[1].trim().parse().unwrap_or(0.0);
                    let eq_dep: f64 = parts[2].trim().parse().unwrap_or(0.0);
                    let dlat = (eq_lat - lat).to_radians();
                    let dlon = (eq_lon - lon).to_radians();
                    let a_ = dlat.sin() * dlat.sin() + lat.to_radians().cos() * eq_lat.to_radians().cos() * dlon.sin() * dlon.sin();
                    let dist = 6371000.0 * 2.0 * a_.sqrt().atan2((1.0 - a_).sqrt());
                    if dist < 500000.0 {
                        let mag_search = &search[ce..];
                        if let Some(ms) = mag_search.find("\"mag\":") {
                            let ms_end = ms + 5;
                            let rest = &mag_search[ms_end..];
                            let vend = rest.find(|c: char| c == ',' || c == '}').unwrap_or(rest.len());
                            let mag: f64 = rest[..vend].trim().parse().unwrap_or(0.0);
                            if mag > 2.0 {
                                is_obj(&mut out, &[
                                    ("seismic_magnitude", mag),
                                    ("seismic_depth", eq_dep),
                                    ("seismic_distance", dist),
                                ]); obj_count += 1;
                                found = true;
                            }
                        }
                    }
                }
                if found { break; }
                search = &search[ce..];
            }
        }
    }

    if let Some(body) = http_get("services.swpc.noaa.gov",
        "/json/products/summary/solar-wind-mag-1-day.json")
    {
        if let Some(bz) = jnum(&body, "bz") {
            is_obj(&mut out, &[("magnetism_bz", bz)]); obj_count += 1;
        }
        if let Some(bt) = jnum(&body, "bt") {
            is_obj(&mut out, &[("magnetism_bt", bt)]); obj_count += 1;
        }
    }

    if archive.idx.len() >= 16 {
        let entry_count = u32::from_le_bytes(archive.idx[0..4].try_into().unwrap_or([0u8; 4])) as usize;
        let field_count = u32::from_le_bytes(archive.idx[4..8].try_into().unwrap_or([0u8; 4])) as usize;
        let mut o = 8;
        let mut field_names = Vec::new();
        for _ in 0..field_count {
            if o >= archive.idx.len() { break; }
            let nl = archive.idx[o] as usize; o += 1;
            if o + nl > archive.idx.len() { break; }
            let name = std::str::from_utf8(&archive.idx[o..o+nl]).unwrap_or("").to_string();
            o += nl;
            field_names.push(name);
        }
        if o + 4 <= archive.idx.len() {
            let rec_size = u32::from_le_bytes(archive.idx[o..o+4].try_into().unwrap_or([0u8; 4])) as usize;
            o += 4;
            let idx_data_start = o;
            let entry_size = 40usize;
            let dat_len = archive.dat.len();

            let mut left = 0;
            let mut right = entry_count;
            let mut i = 0;
            while left < right {
                let mid_idx = left + (right - left) / 2;
                let base = idx_data_start + mid_idx * entry_size;
                if base + entry_size > archive.idx.len() { break; }
                let idx_t = f64::from_le_bytes(archive.idx[base..base + 8].try_into().unwrap_or([0u8; 8]));
                if t < idx_t { right = mid_idx; } else { left = mid_idx + 1; }
                i = mid_idx;
            }

            let mut j = i;
            while j < entry_count {
                let base = idx_data_start + j * entry_size;
                if base + entry_size > archive.idx.len() { break; }
                let idx_t = f64::from_le_bytes(archive.idx[base..base + 8].try_into().unwrap_or([0u8; 8]));
                let idx_x = f64::from_le_bytes(archive.idx[base + 8..base + 16].try_into().unwrap_or([0u8; 8]));
                let idx_y = f64::from_le_bytes(archive.idx[base + 16..base + 24].try_into().unwrap_or([0u8; 8]));
                let idx_z = f64::from_le_bytes(archive.idx[base + 24..base + 32].try_into().unwrap_or([0u8; 8]));
                let offset = u64::from_le_bytes(archive.idx[base + 32..base + 40].try_into().unwrap_or([0u8; 8])) as usize;
                let dist2 = (idx_x - x).powi(2) + (idx_y - y).powi(2) + (idx_z - z).powi(2);
                if (idx_t - t).abs() < 1e6 && dist2.sqrt() < 1e9 && offset + rec_size <= dat_len {
                    let p = offset;
                    out.push(field_count as u8);
                    for name in &field_names { out.push(name.len() as u8); out.extend_from_slice(name.as_bytes()); out.push(0u8); }
                    for fi in 0..field_count {
                        let val_off = p + 32 + fi * 8;
                        if val_off + 8 <= archive.dat.len() { out.extend_from_slice(&archive.dat[val_off..val_off+8]); }
                        else { out.extend_from_slice(&0.0f64.to_le_bytes()); }
                    }
                    out.extend_from_slice(&1u32.to_le_bytes());
                    out.push(4u8);
                    out.push(1); out.extend_from_slice(b"t"); out.push(0u8);
                    out.push(1); out.extend_from_slice(b"x"); out.push(0u8);
                    out.push(1); out.extend_from_slice(b"y"); out.push(0u8);
                    out.push(1); out.extend_from_slice(b"z"); out.push(0u8);
                    let rec_t = f64::from_le_bytes(archive.dat[p..p+8].try_into().unwrap_or([0u8; 8]));
                    let rec_x = f64::from_le_bytes(archive.dat[p+8..p+16].try_into().unwrap_or([0u8; 8]));
                    let rec_y = f64::from_le_bytes(archive.dat[p+16..p+24].try_into().unwrap_or([0u8; 8]));
                    let rec_z = f64::from_le_bytes(archive.dat[p+24..p+32].try_into().unwrap_or([0u8; 8]));
                    out.extend_from_slice(&rec_t.to_le_bytes());
                    out.extend_from_slice(&rec_x.to_le_bytes());
                    out.extend_from_slice(&rec_y.to_le_bytes());
                    out.extend_from_slice(&rec_z.to_le_bytes());
                    obj_count += 1;
                    break;
                }
                j += 1;
            }
        }
    }

    out[obj_count_pos..obj_count_pos + 4].copy_from_slice(&obj_count.to_le_bytes());
    out
}

fn handle_pulse(mut stream: TcpStream, signal: &str, immunity: Arc<Mutex<HashMap<String, (u32, u32)>>>, immunity_str: Arc<Mutex<String>>, archive: Arc<Archive>) {
    let key = match extract_header(signal, "Sec-WebSocket-Key") {
        Some(k) => k,
        None => return,
    };
    let combined = format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let hashed = sha1(combined.as_bytes());
    let encoded = base64_encode(&hashed);
    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",
        encoded
    );
    if stream.write_all(response.as_bytes()).is_err() { return; }
    let _ = stream.set_nodelay(true);
    let mut last_poke: Vec<String> = Vec::new();
    while let Some(frame) = read_ws_frame_raw(&mut stream) {
        if frame.opcode == 0x8 { break; }
        if frame.opcode == 0x2 {
            if frame.payload.len() < 37 { continue; }
            let id = u32::from_le_bytes(frame.payload[33..37].try_into().unwrap_or([0u8; 4]));
            let response = weave(&frame.payload[0..33], &archive);
            let mut out_with_id = Vec::with_capacity(response.len() + 4);
            out_with_id.extend_from_slice(&response);
            out_with_id.extend_from_slice(&id.to_le_bytes());
            write_ws_binary(&mut stream, &out_with_id);
        } else if frame.opcode == 0x1 {
            let msg = String::from_utf8_lossy(&frame.payload);
            if let Some(survived) = extract_json_value(&msg, "survived") {
                let mut counts = immunity.lock().unwrap();
                for path in survived.split('|') {
                    let entry = counts.entry(path.to_string()).or_insert((0, 0));
                    entry.1 += 1;
                }
                rewrite_immunity(&counts);
                *immunity_str.lock().unwrap() = format_immunity_snapshot(&counts);
                last_poke.clear();
            } else if let Some(poke) = extract_json_value(&msg, "poke") {
                last_poke = poke.split('|').map(|s| s.to_string()).collect();
            }
        }
    }
    if last_poke.len() == 1 {
        let mut counts = immunity.lock().unwrap();
        let entry = counts.entry(last_poke[0].clone()).or_insert((0, 0));
        entry.0 += 1;
        rewrite_immunity(&counts);
        *immunity_str.lock().unwrap() = format_immunity_snapshot(&counts);
    }
}

struct WsFrame {
    opcode: u8,
    payload: Vec<u8>,
}

fn read_ws_frame_raw(stream: &mut TcpStream) -> Option<WsFrame> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).ok()?;
    let opcode = header[0] & 0x0f;
    let masked = (header[1] & 0x80) != 0;
    let mut payload_len = (header[1] & 0x7f) as usize;
    if payload_len == 126 {
        let mut ext = [0u8; 2];
        stream.read_exact(&mut ext).ok()?;
        payload_len = u16::from_be_bytes(ext) as usize;
    } else if payload_len == 127 {
        let mut ext = [0u8; 8];
        stream.read_exact(&mut ext).ok()?;
        payload_len = u64::from_be_bytes(ext) as usize;
    }
    let mut mask_key = [0u8; 4];
    if masked {
        stream.read_exact(&mut mask_key).ok()?;
    }
    let mut payload = vec![0u8; payload_len];
    stream.read_exact(&mut payload).ok()?;
    if masked {
        for i in 0..payload.len() {
            payload[i] ^= mask_key[i % 4];
        }
    }
    Some(WsFrame { opcode, payload })
}

fn write_ws_binary(stream: &mut TcpStream, data: &[u8]) {
    let mut header = [0u8; 10];
    header[0] = 0x82;
    if data.len() <= 125 {
        header[1] = data.len() as u8;
        let _ = stream.write_all(&header[..2]);
    } else if data.len() <= 65535 {
        header[1] = 126;
        let ext = (data.len() as u16).to_be_bytes();
        header[2] = ext[0];
        header[3] = ext[1];
        let _ = stream.write_all(&header[..4]);
    } else {
        header[1] = 127;
        let ext = (data.len() as u64).to_be_bytes();
        header[2..10].copy_from_slice(&ext);
        let _ = stream.write_all(&header[..10]);
    }
    let _ = stream.write_all(data);
}

fn format_immunity_snapshot(counts: &HashMap<String, (u32, u32)>) -> String {
    let mut out = String::new();
    let mut keys: Vec<&String> = counts.keys().collect();
    keys.sort();
    for key in keys {
        let (deaths, survived) = counts[key];
        if deaths == 0 && survived == 0 {
            out.push_str(&format!("immunity {}\n", key));
        } else {
            out.push_str(&format!("immunity {} {} {}\n", key, deaths, survived));
        }
    }
    out
}

fn load_immunity() -> HashMap<String, (u32, u32)> {
    let mut counts = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("is/immunity.is") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "immunity" {
                let d = if parts.len() >= 3 { parts[2].parse().unwrap_or(0) } else { 0 };
                let s = if parts.len() >= 4 { parts[3].parse().unwrap_or(0) } else { 0 };
                counts.insert(parts[1].to_string(), (d, s));
            }
        }
    }
    counts
}

fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let archive = Arc::new(Archive {
        idx: std::fs::read("is/measured.idx").unwrap_or_default(),
        dat: std::fs::read("is/measured.dat").unwrap_or_default(),
        index_html: std::fs::read("crates/server/static/index.html").unwrap_or_default(),
        world_js: std::fs::read("crates/server/static/world.js").unwrap_or_default(),
    });
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let immunity = Arc::new(Mutex::new(load_immunity()));
    let immunity_str = Arc::new(Mutex::new(format_immunity_snapshot(&immunity.lock().unwrap())));
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let immunity = Arc::clone(&immunity);
            let immunity_str = Arc::clone(&immunity_str);
            let archive = Arc::clone(&archive);
            thread::spawn(move || handle_observer(stream, immunity, immunity_str, archive));
        }
    }
}

fn parse_path(signal: &str) -> String {
    let first_line = signal.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() >= 2 { parts[1].to_string() } else { "/".to_string() }
}

fn rewrite_immunity(counts: &HashMap<String, (u32, u32)>) {
    let mut out = String::new();
    let mut keys: Vec<&String> = counts.keys().collect();
    keys.sort();
    for key in keys {
        let (deaths, survived) = counts[key];
        if deaths == 0 && survived == 0 {
            out.push_str(&format!("immunity {}\n", key));
        } else {
            out.push_str(&format!("immunity {} {} {}\n", key, deaths, survived));
        }
    }
    let _ = std::fs::write("is/immunity.is", out);
}

fn emit_void(stream: &mut TcpStream) {
    let response = b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    let _ = stream.write_all(response);
}

fn emit(stream: &mut TcpStream, status: &str, content_type: &str, body: &[u8]) {
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
        status, content_type, body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
}

fn read_signal(stream: &mut TcpStream) -> Option<String> {
    let mut buf = [0u8; 8192];
    let mut accumulated = Vec::new();
    loop {
        match stream.read(&mut buf) {
            Ok(0) => return None,
            Ok(n) => {
                accumulated.extend_from_slice(&buf[..n]);
                if accumulated.windows(4).any(|w| w == b"\r\n\r\n") {
                    return Some(String::from_utf8_lossy(&accumulated).to_string());
                }
                if accumulated.len() > 65536 { return None; }
            }
            Err(_) => return None,
        }
    }
}

fn sha1(input: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let bit_len = (input.len() as u64) * 8;
    let mut msg = input.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 { msg.push(0); }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
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
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d; d = c; c = b.rotate_left(30); b = a; a = temp;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut result = [0u8; 20];
    for i in 0..5 {
        result[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    result
}
