use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Clone)]
enum Extract {
    Field(String, String),      // json_key → out_name
    First(String, String),      // first element of json array
    Count(String, String),      // count items in json array
    Geojson {                   // nearby geojson search
        max_dist: f64,
        mag_key: String,
        min_mag: f64,
        outputs: Vec<String>,
    },
}

struct SourceConfig {
    _name: String,
    on_earth: bool,
    url: String,                // full URL template
    extracts: Vec<Extract>,
}

fn load_sources() -> Vec<SourceConfig> {
    let mut sources = Vec::new();
    let content = std::fs::read_to_string("is/sources.is").unwrap_or_default();
    let mut cur_name = String::new();
    let mut cur_on_earth = false;
    let mut cur_url = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new();
    let mut active = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts[0] == "source" {
            if active {
                sources.push(SourceConfig {
                    _name: cur_name.clone(),
                    on_earth: cur_on_earth,
                    url: cur_url.clone(),
                    extracts: cur_extracts.clone(),
                });
            }
            cur_name = parts.get(1).unwrap_or(&"").to_string();
            cur_on_earth = parts.iter().any(|&p| p == "on_earth");
            cur_url.clear();
            cur_extracts.clear();
            active = true;
        } else if parts[0] == "url" {
            cur_url = parts[1..].join(" ");
        } else if parts[0] == "field" {
            cur_extracts.push(Extract::Field(
                parts.get(1).unwrap_or(&"").to_string(),
                parts.get(2).unwrap_or(&"").to_string(),
            ));
        } else if parts[0] == "first" {
            cur_extracts.push(Extract::First(
                parts.get(1).unwrap_or(&"").to_string(),
                parts.get(2).unwrap_or(&"").to_string(),
            ));
        } else if parts[0] == "count" {
            cur_extracts.push(Extract::Count(
                parts.get(1).unwrap_or(&"").to_string(),
                parts.get(2).unwrap_or(&"").to_string(),
            ));
        } else if parts[0] == "geojson" {
            cur_extracts.push(Extract::Geojson {
                max_dist: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(500000.0),
                mag_key: parts.get(3).unwrap_or(&"mag").to_string(),
                min_mag: parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                outputs: parts[5..].iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    if active {
        sources.push(SourceConfig {
            _name: cur_name,
            on_earth: cur_on_earth,
            url: cur_url,
            extracts: cur_extracts,
        });
    }
    eprintln!("loaded {} sources from is/sources.is", sources.len());
    sources
}


fn fetch(url: &str) -> Option<String> {
    let output = Command::new("curl")
        .arg("-s")
        .arg("-m")
        .arg("8")
        .arg("--connect-timeout")
        .arg("4")
        .arg(url)
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}


fn jnum(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = json[start..].trim_start();
    let end = rest.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(rest.len());
    rest[..end].parse().ok()
}

fn jarr_first(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = &json[start..];
    let as_ = rest.find('[')?;
    let ae = rest[as_..].find(']')?;
    let inner = &rest[as_ + 1..ae];
    inner.split(',').next().and_then(|p| p.trim().parse().ok())
}

fn jarr_count(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = &json[start..];
    let as_ = rest.find('[')?;
    let ae = rest[as_..].find(']')?;
    let inner = &rest[as_ + 1..ae];
    let count = inner.split(',').filter(|p| !p.trim().is_empty()).count();
    Some(count as f64)
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


struct Archive {
    sources: Vec<SourceConfig>,
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

fn render_url(template: &str, lat: f64, lon: f64) -> String {
    template
        .replace("{lat}", &format!("{:.4}", lat))
        .replace("{lon}", &format!("{:.4}", lon))
        .replace("{lat_min}", &format!("{:.2}", lat - 0.5))
        .replace("{lat_max}", &format!("{:.2}", lat + 0.5))
        .replace("{lon_min}", &format!("{:.2}", lon - 0.5))
        .replace("{lon_max}", &format!("{:.2}", lon + 0.5))
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

    let (lat, lon) = if on_earth {
        let (lat, lon, _) = ecef_to_geodetic(x, y, z);
        (Some(lat), Some(lon))
    } else {
        (None, None)
    };

    for src in &archive.sources {
        if src.on_earth && !on_earth { continue; }

        let url = if lat.is_some() {
            render_url(&src.url, lat.unwrap(), lon.unwrap())
        } else {
            src.url.clone()
        };

        let body = match fetch(&url) {
            Some(b) => b,
            None => continue,
        };

        for ext in &src.extracts {
            match ext {
                Extract::Field(json_key, out_name) => {
                    if let Some(v) = jnum(&body, json_key) {
                        is_obj(&mut out, &[(out_name, v)]);
                        obj_count += 1;
                    }
                }
                Extract::First(json_key, out_name) => {
                    if let Some(v) = jarr_first(&body, json_key) {
                        is_obj(&mut out, &[(out_name, v)]);
                        obj_count += 1;
                    }
                }
                Extract::Count(json_key, out_name) => {
                    if let Some(v) = jarr_count(&body, json_key) {
                        is_obj(&mut out, &[(out_name, v)]);
                        obj_count += 1;
                    }
                }
                Extract::Geojson { max_dist, mag_key, min_mag, outputs } => {
                    if outputs.len() < 3 || lat.is_none() { continue; }
                    let lat_v = lat.unwrap();
                    let lon_v = lon.unwrap();
                    let mut search = &body[..];
                    let mut found = false;
                    while let Some(cs) = search.find("\"coordinates\":[") {
                        let csi = cs + "\"coordinates\":[".len();
                        let cei = match search[csi..].find(']') { Some(e) => csi + e, None => break };
                        let inner = &search[csi..cei];
                        let parts: Vec<&str> = inner.split(',').collect();
                        if parts.len() >= 3 {
                            let eq_lon: f64 = parts[0].trim().parse().unwrap_or(0.0);
                            let eq_lat: f64 = parts[1].trim().parse().unwrap_or(0.0);
                            let eq_dep: f64 = parts[2].trim().parse().unwrap_or(0.0);
                            let dlat = (eq_lat - lat_v).to_radians();
                            let dlon = (eq_lon - lon_v).to_radians();
                            let h = dlat.sin() * dlat.sin()
                                + lat_v.to_radians().cos() * eq_lat.to_radians().cos()
                                * dlon.sin() * dlon.sin();
                            let dist = 6371000.0 * 2.0 * h.sqrt().atan2((1.0 - h).sqrt());
                            if dist < *max_dist {
                                let after_coords = &search[cei..];
                                if let Some(ms) = after_coords.find(&format!("\"{}\":", mag_key)) {
                                    let rest = &after_coords[ms + mag_key.len() + 3..];
                                    let vend = rest.find(|c: char| c == ',' || c == '}').unwrap_or(rest.len());
                                    let mag: f64 = rest[..vend].trim().parse().unwrap_or(0.0);
                                    if mag >= *min_mag {
                                        is_obj(&mut out, &[
                                            (&outputs[0], mag),
                                            (&outputs[1], eq_dep),
                                            (&outputs[2], dist),
                                        ]);
                                        obj_count += 1;
                                        found = true;
                                    }
                                }
                            }
                        }
                        if found { break; }
                        search = &search[cei..];
                    }
                }
            }
        }
    }

    // ─── Local archive (.idx/.dat) ───
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


fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in input.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        result.push(TABLE[(b[0] >> 2) as usize] as char);
        result.push(TABLE[(((b[0] & 0x03) << 4) | (b[1] >> 4)) as usize] as char);
        if chunk.len() > 1 { result.push(TABLE[(((b[1] & 0x0f) << 2) | (b[2] >> 6)) as usize] as char); }
        else { result.push('='); }
        if chunk.len() > 2 { result.push(TABLE[(b[2] & 0x3f) as usize] as char); }
        else { result.push('='); }
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
    let signal = match read_signal(&mut stream_ref) { Some(r) => r, None => return };
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

struct WsFrame { opcode: u8, payload: Vec<u8> }

fn read_ws_frame_raw(stream: &mut TcpStream) -> Option<WsFrame> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).ok()?;
    let opcode = header[0] & 0x0f;
    let masked = (header[1] & 0x80) != 0;
    let mut payload_len = (header[1] & 0x7f) as usize;
    if payload_len == 126 {
        let mut ext = [0u8; 2]; stream.read_exact(&mut ext).ok()?;
        payload_len = u16::from_be_bytes(ext) as usize;
    } else if payload_len == 127 {
        let mut ext = [0u8; 8]; stream.read_exact(&mut ext).ok()?;
        payload_len = u64::from_be_bytes(ext) as usize;
    }
    let mut mask_key = [0u8; 4];
    if masked { stream.read_exact(&mut mask_key).ok()?; }
    let mut payload = vec![0u8; payload_len];
    stream.read_exact(&mut payload).ok()?;
    if masked { for i in 0..payload.len() { payload[i] ^= mask_key[i % 4]; } }
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
        header[2] = ext[0]; header[3] = ext[1];
        let _ = stream.write_all(&header[..4]);
    } else {
        header[1] = 127;
        let ext = (data.len() as u64).to_be_bytes();
        header[2..10].copy_from_slice(&ext);
        let _ = stream.write_all(&header[..10]);
    }
    let _ = stream.write_all(data);
}

fn handle_pulse(mut stream: TcpStream, signal: &str, immunity: Arc<Mutex<HashMap<String, (u32, u32)>>>, immunity_str: Arc<Mutex<String>>, archive: Arc<Archive>) {
    let key = match extract_header(signal, "Sec-WebSocket-Key") { Some(k) => k, None => return };
    let combined = format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let encoded = base64_encode(&sha1(combined.as_bytes()));
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

fn format_immunity_snapshot(counts: &HashMap<String, (u32, u32)>) -> String {
    let mut out = String::new();
    let mut keys: Vec<&String> = counts.keys().collect();
    keys.sort();
    for key in keys {
        let (d, s) = counts[key];
        if d == 0 && s == 0 { out.push_str(&format!("immunity {}\n", key)); }
        else { out.push_str(&format!("immunity {} {} {}\n", key, d, s)); }
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

fn rewrite_immunity(counts: &HashMap<String, (u32, u32)>) {
    let mut out = String::new();
    let mut keys: Vec<&String> = counts.keys().collect();
    keys.sort();
    for key in keys {
        let (d, s) = counts[key];
        if d == 0 && s == 0 { out.push_str(&format!("immunity {}\n", key)); }
        else { out.push_str(&format!("immunity {} {} {}\n", key, d, s)); }
    }
    let _ = std::fs::write("is/immunity.is", out);
}

fn parse_path(signal: &str) -> String {
    let first_line = signal.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() >= 2 { parts[1].to_string() } else { "/".to_string() }
}

fn emit_void(stream: &mut TcpStream) {
    let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
}

fn emit(stream: &mut TcpStream, status: &str, content_type: &str, body: &[u8]) {
    let header = format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n", status, content_type, body.len());
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
        for i in 0..16 { w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]); }
        for i in 16..80 { w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1); }
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
        h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b); h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d); h[4] = h[4].wrapping_add(e);
    }
    let mut result = [0u8; 20];
    for i in 0..5 { result[i*4..i*4+4].copy_from_slice(&h[i].to_be_bytes()); }
    result
}

fn main() {
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let archive = Arc::new(Archive {
        sources: load_sources(),
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
