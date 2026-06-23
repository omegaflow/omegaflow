use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Clone)]
enum Extract {
    Field(String, String),
    First(String, String),
    Last(String, String),
    Count(String, String),
    LastRow(String, String),
    Vector(String, String, String),
    LastObj(String, String, String, String),
    Geojson { max_dist: f64, mag_key: String, min_mag: f64, outputs: Vec<String> },
}

struct SourceConfig {
    on_earth: bool,
    ttl: u64,
    url: String,
    extracts: Vec<Extract>,
}

fn load_sources() -> Vec<SourceConfig> {
    let mut sources = Vec::new();
    let content = std::fs::read_to_string("is/sources.is").unwrap_or_default();
    let mut cur_on_earth = false;
    let mut cur_ttl: u64 = 300;
    let mut cur_url = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new();
    let mut active = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "source" => {
                if active { sources.push(SourceConfig { on_earth: cur_on_earth, ttl: cur_ttl, url: cur_url.clone(), extracts: cur_extracts.clone() }); }
                cur_on_earth = parts.iter().any(|&p| p == "on_earth");
                cur_ttl = 300; cur_url.clear(); cur_extracts.clear(); active = true;
            }
            "url" => cur_url = parts[1..].join(" "),
            "ttl" => cur_ttl = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(300),
            "field" => cur_extracts.push(Extract::Field(parts[1].to_string(), parts[2].to_string())),
            "first" => cur_extracts.push(Extract::First(parts[1].to_string(), parts[2].to_string())),
            "last" => cur_extracts.push(Extract::Last(parts[1].to_string(), parts[2].to_string())),
            "count" => cur_extracts.push(Extract::Count(parts[1].to_string(), parts[2].to_string())),
            "last_row" => cur_extracts.push(Extract::LastRow(parts[1].to_string(), parts[2].to_string())),
            "vector" => cur_extracts.push(Extract::Vector(parts[1].to_string(), parts[2].to_string(), parts[3].to_string())),
            "last_obj" => cur_extracts.push(Extract::LastObj(parts[1].to_string(), parts[2].to_string(), parts[3].to_string(), parts[4].to_string())),
            "geojson" => cur_extracts.push(Extract::Geojson {
                max_dist: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(500000.0),
                mag_key: parts.get(3).unwrap_or(&"mag").to_string(),
                min_mag: parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                outputs: parts[5..].iter().map(|s| s.to_string()).collect(),
            }),
            _ => {}
        }
    }
    if active { sources.push(SourceConfig { on_earth: cur_on_earth, ttl: cur_ttl, url: cur_url, extracts: cur_extracts }); }
    eprintln!("loaded {} sources", sources.len());
    sources
}

fn fetch(url: &str) -> Option<String> {
    let output = Command::new("curl").arg("-s").arg("-m").arg("8").arg("--connect-timeout").arg("4").arg(url).output().ok()?;
    if output.status.success() { Some(String::from_utf8_lossy(&output.stdout).to_string()) } else { None }
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
        if let Ok(v) = trimmed[..end].parse::<f64>() { last_val = Some(v); }
        search = &search[pos + pat.len()..];
    }
    last_val
}

fn jarr_count(json: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{}\":", key);
    let start = json.find(&pat)? + pat.len();
    let rest = &json[start..];
    let as_ = rest.find('[')?;
    let ae = rest[as_..].find(']')?;
    Some(rest[as_+1..ae].split(',').filter(|p| !p.trim().is_empty()).count() as f64)
}

fn j2d_last_row(json: &str, col: &str) -> Option<f64> {
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
}

fn text_vector(text: &str) -> Option<(f64, f64, f64)> {
    let mut last = None;
    for line in text.lines() {
        let lx = line.find("X ="); let ly = line.find("Y ="); let lz = line.find("Z =");
        if let (Some(xp), Some(yp), Some(zp)) = (lx, ly, lz) {
            let xs = &line[xp+3..yp].trim();
            let ys = &line[yp+3..zp].trim();
            let zs = &line[zp+3..].trim();
            if let (Ok(xv), Ok(yv), Ok(zv)) = (xs.parse::<f64>(), ys.parse::<f64>(), zs.parse::<f64>()) {
                last = Some((xv, yv, zv));
            }
        }
    }
    last
}

fn jobj_last_match(json: &str, filter_key: &str, filter_val: &str, extract_key: &str) -> Option<f64> {
    let fv_quoted = format!("\"{}\":\"{}\"", filter_key, filter_val);
    let ek_pat = format!("\"{}\":", extract_key);
    let mut last_val = None;
    let mut search_start = 0;
    while let Some(fv_pos) = json[search_start..].find(&fv_quoted) {
        let abs_pos = search_start + fv_pos;
        let chunk_start = json[..abs_pos].rfind('{').unwrap_or(0);
        let chunk_end = json[abs_pos..].find('}').map(|e| abs_pos + e).unwrap_or(json.len());
        let chunk = &json[chunk_start..chunk_end];
        if let Some(ek_pos) = chunk.find(&ek_pat) {
            let rest = &chunk[ek_pos + ek_pat.len()..];
            let trimmed = rest.trim_start();
            let end = trimmed.find(|c: char| c == ',' || c == '}').unwrap_or(trimmed.len());
            if let Ok(v) = trimmed[..end].parse::<f64>() { last_val = Some(v); }
        }
        search_start = abs_pos + fv_quoted.len();
    }
    last_val
}

fn is_obj(out: &mut Vec<u8>, fields: &[(&str, f64)]) {
    out.push(fields.len() as u8);
    for (name, _) in fields { out.push(name.len() as u8); out.extend_from_slice(name.as_bytes()); out.push(0u8); }
    for (_, val) in fields { out.extend_from_slice(&val.to_le_bytes()); }
    out.extend_from_slice(&0u32.to_le_bytes());
}

struct Archive {
    sources: Vec<SourceConfig>,
    idx: Vec<u8>,
    dat: Vec<u8>,
    index_html: Vec<u8>,
    world_js: Vec<u8>,
    cache: Mutex<HashMap<String, (u64, String)>>,
}

fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let a = 6378137.0_f64; let f = 1.0/298.257223563; let b = a*(1.0-f);
    let e2 = f*(2.0-f); let ep2 = (a*a-b*b)/(b*b);
    let p = (x*x+y*y).sqrt();
    let theta = (z*a/(p*b)).atan2(1.0);
    let lat = (z+ep2*b*theta.sin().powi(3)).atan2(p-e2*a*theta.cos().powi(3));
    let lon = y.atan2(x);
    let n = a/(1.0-e2*lat.sin().powi(2)).sqrt();
    let alt = p/lat.cos()-n;
    (lat.to_degrees(), lon.to_degrees(), alt)
}

fn render_url(template: &str, lat: f64, lon: f64) -> String {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let days = secs / 86400;
    let today_days = days + 40587;
    let today = format!("{}-{:02}-{:02}", 1970 + (today_days / 146097) * 400 + ((today_days % 146097) / 36524) * 100 + (((today_days % 146097) % 36524) / 1461) * 4 + ((((today_days % 146097) % 36524) % 1461) / 365), 0, 0);
    let tomorrow_days = days + 40588;
    let tomorrow = format!("{}-{:02}-{:02}", 1970 + (tomorrow_days / 146097) * 400 + ((tomorrow_days % 146097) / 36524) * 100 + (((tomorrow_days % 146097) % 36524) / 1461) * 4 + ((((tomorrow_days % 146097) % 36524) % 1461) / 365), 0, 0);
    template
        .replace("{lat}", &format!("{:.4}", lat)).replace("{lon}", &format!("{:.4}", lon))
        .replace("{lat_min}", &format!("{:.2}", lat-0.5)).replace("{lat_max}", &format!("{:.2}", lat+0.5))
        .replace("{lon_min}", &format!("{:.2}", lon-0.5)).replace("{lon_max}", &format!("{:.2}", lon+0.5))
        .replace("{today}", &today).replace("{tomorrow}", &tomorrow)
}

fn weave(payload: &[u8], archive: &Archive) -> Vec<u8> {
    if payload.len() < 33 { return Vec::new(); }
    let t = f64::from_le_bytes(payload[0..8].try_into().unwrap_or([0u8;8]));
    let x = f64::from_le_bytes(payload[8..16].try_into().unwrap_or([0u8;8]));
    let y = f64::from_le_bytes(payload[16..24].try_into().unwrap_or([0u8;8]));
    let z = f64::from_le_bytes(payload[24..32].try_into().unwrap_or([0u8;8]));

    let mut out = Vec::new();
    out.extend_from_slice(b"IS"); out.push(2u8);
    let mut obj_count: u32 = 0;
    let obj_count_pos = out.len();
    out.extend_from_slice(&0u32.to_le_bytes());

    let r = (x*x+y*y+z*z).sqrt();
    let on_earth = r > 6.3e6 && r < 6.5e6;
    let (lat, lon) = if on_earth { let (la,lo,_)=ecef_to_geodetic(x,y,z); (Some(la),Some(lo)) } else { (None,None) };

    for src in &archive.sources {
        if src.on_earth && !on_earth { continue; }
        let url = if lat.is_some() { render_url(&src.url, lat.unwrap(), lon.unwrap()) } else { src.url.clone() };
        let now_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let cache_key = if lat.is_some() { format!("{}_{:.4}_{:.4}", src.url.split('?').next().unwrap_or(&src.url), lat.unwrap(), lon.unwrap()) } else { src.url.split('?').next().unwrap_or(&src.url).to_string() };
        let body = {
            let cache = archive.cache.lock().unwrap();
            if let Some((ts, data)) = cache.get(&cache_key) {
                if now_secs.saturating_sub(*ts) < src.ttl { data.clone() } else { String::new() }
            } else { String::new() }
        };
        let body = if body.is_empty() {
            let fetched = match fetch(&url) { Some(b) => b, None => continue };
            let mut cache = archive.cache.lock().unwrap();
            cache.insert(cache_key, (now_secs, fetched.clone()));
            fetched
        } else { body };

        for ext in &src.extracts {
            match ext {
                Extract::Field(k, n) => { if let Some(v) = jnum(&body, k) { is_obj(&mut out, &[(n, v)]); obj_count += 1; } }
                Extract::First(k, n) => { if let Some(v) = jarr_first(&body, k) { is_obj(&mut out, &[(n, v)]); obj_count += 1; } }
                Extract::Last(k, n) => { if let Some(v) = jarr_last(&body, k) { is_obj(&mut out, &[(n, v)]); obj_count += 1; } }
                Extract::Count(k, n) => { if let Some(v) = jarr_count(&body, k) { is_obj(&mut out, &[(n, v)]); obj_count += 1; } }
                Extract::LastRow(k, n) => { if let Some(v) = j2d_last_row(&body, k) { is_obj(&mut out, &[(n, v)]); obj_count += 1; } }
                Extract::Vector(nx, ny, nz) => {
                    if let Some((vx, vy, vz)) = text_vector(&body) {
                        is_obj(&mut out, &[(nx, vx), (ny, vy), (nz, vz)]); obj_count += 1;
                    }
                }
                Extract::LastObj(fk, fv, ek, n) => {
                    if let Some(v) = jobj_last_match(&body, fk, fv, ek) {
                        is_obj(&mut out, &[(n, v)]); obj_count += 1;
                    }
                }
                Extract::Geojson { max_dist, mag_key, min_mag, outputs } => {
                    if outputs.len() < 3 || lat.is_none() { continue; }
                    let (lv, ln) = (lat.unwrap(), lon.unwrap());
                    let mut search = &body[..]; let mut found = false;
                    while let Some(cs) = search.find("\"coordinates\":[") {
                        let csi = cs + "\"coordinates\":[".len();
                        let cei = match search[csi..].find(']') { Some(e) => csi+e, None => break };
                        let parts: Vec<&str> = search[csi..cei].split(',').collect();
                        if parts.len() >= 3 {
                            let (elo,ela,ed) = (parts[0].trim().parse().unwrap_or(0.0), parts[1].trim().parse().unwrap_or(0.0), parts[2].trim().parse().unwrap_or(0.0));
                            let dlat = (ela-lv).to_radians(); let dlon = (elo-ln).to_radians();
                            let h = dlat.sin()*dlat.sin() + lv.to_radians().cos()*ela.to_radians().cos()*dlon.sin()*dlon.sin();
                            let dist = 6371000.0*2.0*h.sqrt().atan2((1.0-h).sqrt());
                            if dist < *max_dist {
                                let ac = &search[cei..];
                                if let Some(ms) = ac.find(&format!("\"{}\":", mag_key)) {
                                    let rest = &ac[ms+mag_key.len()+3..];
                                    let vend = rest.find(|c: char| c==','||c=='}').unwrap_or(rest.len());
                                    let mag: f64 = rest[..vend].trim().parse().unwrap_or(0.0);
                                    if mag >= *min_mag {
                                        is_obj(&mut out, &[(&outputs[0], mag), (&outputs[1], ed), (&outputs[2], dist)]);
                                        obj_count += 1; found = true;
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

    if archive.idx.len() >= 16 {
        let entry_count = u32::from_le_bytes(archive.idx[0..4].try_into().unwrap_or([0u8;4])) as usize;
        let field_count = u32::from_le_bytes(archive.idx[4..8].try_into().unwrap_or([0u8;4])) as usize;
        let mut o = 8; let mut field_names = Vec::new();
        for _ in 0..field_count {
            if o >= archive.idx.len() { break; }
            let nl = archive.idx[o] as usize; o += 1;
            if o+nl > archive.idx.len() { break; }
            field_names.push(std::str::from_utf8(&archive.idx[o..o+nl]).unwrap_or("").to_string());
            o += nl;
        }
        if o+4 <= archive.idx.len() {
            let rec_size = u32::from_le_bytes(archive.idx[o..o+4].try_into().unwrap_or([0u8;4])) as usize; o += 4;
            let idx_start = o; let entry_size = 40; let dat_len = archive.dat.len();
            let mut left = 0; let mut right = entry_count; let mut i = 0;
            while left < right {
                let mid = left + (right-left)/2; let base = idx_start + mid*entry_size;
                if base+entry_size > archive.idx.len() { break; }
                let idx_t = f64::from_le_bytes(archive.idx[base..base+8].try_into().unwrap_or([0u8;8]));
                if t < idx_t { right = mid; } else { left = mid+1; }
                i = mid;
            }
            let mut j = i;
            while j < entry_count {
                let base = idx_start + j*entry_size;
                if base+entry_size > archive.idx.len() { break; }
                let idx_t = f64::from_le_bytes(archive.idx[base..base+8].try_into().unwrap_or([0u8;8]));
                let idx_x = f64::from_le_bytes(archive.idx[base+8..base+16].try_into().unwrap_or([0u8;8]));
                let idx_y = f64::from_le_bytes(archive.idx[base+16..base+24].try_into().unwrap_or([0u8;8]));
                let idx_z = f64::from_le_bytes(archive.idx[base+24..base+32].try_into().unwrap_or([0u8;8]));
                let offset = u64::from_le_bytes(archive.idx[base+32..base+40].try_into().unwrap_or([0u8;8])) as usize;
                let dist2 = (idx_x-x).powi(2)+(idx_y-y).powi(2)+(idx_z-z).powi(2);
                if (idx_t-t).abs() < 1e6 && dist2.sqrt() < 1e9 && offset+rec_size <= dat_len {
                    let p = offset;
                    out.push(field_count as u8);
                    for name in &field_names { out.push(name.len() as u8); out.extend_from_slice(name.as_bytes()); out.push(0u8); }
                    for fi in 0..field_count {
                        let vo = p+32+fi*8;
                        if vo+8 <= archive.dat.len() { out.extend_from_slice(&archive.dat[vo..vo+8]); } else { out.extend_from_slice(&0.0f64.to_le_bytes()); }
                    }
                    out.extend_from_slice(&1u32.to_le_bytes()); out.push(4u8);
                    out.push(1); out.extend_from_slice(b"t"); out.push(0u8);
                    out.push(1); out.extend_from_slice(b"x"); out.push(0u8);
                    out.push(1); out.extend_from_slice(b"y"); out.push(0u8);
                    out.push(1); out.extend_from_slice(b"z"); out.push(0u8);
                    let rt = f64::from_le_bytes(archive.dat[p..p+8].try_into().unwrap_or([0u8;8]));
                    let rx = f64::from_le_bytes(archive.dat[p+8..p+16].try_into().unwrap_or([0u8;8]));
                    let ry = f64::from_le_bytes(archive.dat[p+16..p+24].try_into().unwrap_or([0u8;8]));
                    let rz = f64::from_le_bytes(archive.dat[p+24..p+32].try_into().unwrap_or([0u8;8]));
                    out.extend_from_slice(&rt.to_le_bytes()); out.extend_from_slice(&rx.to_le_bytes());
                    out.extend_from_slice(&ry.to_le_bytes()); out.extend_from_slice(&rz.to_le_bytes());
                    obj_count += 1; break;
                }
                j += 1;
            }
        }
    }

    out[obj_count_pos..obj_count_pos+4].copy_from_slice(&obj_count.to_le_bytes());
    out
}

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

fn extract_header(s: &str, n: &str) -> Option<String> {
    for l in s.lines() { if let Some(c) = l.find(':') { if l[..c].trim().eq_ignore_ascii_case(n) { return Some(l[c+1..].trim().to_string()); } } }
    None
}

fn extract_json_value(msg: &str, key: &str) -> Option<String> {
    let p = format!("\"{}\":\"", key);
    let s = msg.find(&p)? + p.len();
    let e = msg[s..].find('"')? + s;
    Some(msg[s..e].to_string())
}

fn handle_observer(stream: TcpStream, immunity: Arc<Mutex<HashMap<String,(u32,u32)>>>, immunity_str: Arc<Mutex<String>>, archive: Arc<Archive>) {
    let mut s = stream; s.set_nodelay(true).ok();
    let signal = match read_signal(&mut s) { Some(r) => r, None => return };
    if signal.to_lowercase().contains("upgrade: websocket") { handle_pulse(s, &signal, immunity, immunity_str, archive); }
    else {
        let mut cur = signal;
        loop {
            match parse_path(&cur).as_str() {
                "/" => emit(&mut s, "200 OK", "text/html", &archive.index_html),
                "/immunity" => { let b = immunity_str.lock().unwrap().clone(); emit(&mut s, "200 OK", "text/plain", b.as_bytes()); }
                "/time" => { let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64(); emit(&mut s, "200 OK", "text/plain", t.to_string().as_bytes()); }
                "/world.js" => emit(&mut s, "200 OK", "application/javascript", &archive.world_js),
                _ => { emit_void(&mut s); break; }
            }
            match read_signal(&mut s) { Some(r) => cur = r, None => break }
        }
    }
}

struct WsFrame { opcode: u8, payload: Vec<u8> }

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

fn write_ws_binary(stream: &mut TcpStream, data: &[u8]) {
    let mut h=[0u8;10]; h[0]=0x82;
    if data.len()<=125 { h[1]=data.len() as u8; let _=stream.write_all(&h[..2]); }
    else if data.len()<=65535 { h[1]=126; let e=(data.len() as u16).to_be_bytes(); h[2]=e[0]; h[3]=e[1]; let _=stream.write_all(&h[..4]); }
    else { h[1]=127; let e=(data.len() as u64).to_be_bytes(); h[2..10].copy_from_slice(&e); let _=stream.write_all(&h); }
    let _=stream.write_all(data);
}

fn handle_pulse(mut stream: TcpStream, signal: &str, immunity: Arc<Mutex<HashMap<String,(u32,u32)>>>, immunity_str: Arc<Mutex<String>>, archive: Arc<Archive>) {
    let key = match extract_header(signal,"Sec-WebSocket-Key") { Some(k)=>k, None=>return };
    let encoded = base64_encode(&sha1(&format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").into_bytes()));
    if stream.write_all(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", encoded).as_bytes()).is_err() { return; }
    let _=stream.set_nodelay(true);
    let mut last_poke: Vec<String> = Vec::new();
    while let Some(frame) = read_ws_frame_raw(&mut stream) {
        if frame.opcode==0x8 { break; }
        if frame.opcode==0x2 {
            if frame.payload.len()<37 { continue; }
            let id=u32::from_le_bytes(frame.payload[33..37].try_into().unwrap_or([0u8;4]));
            let resp=weave(&frame.payload[0..33], &archive);
            let mut out=Vec::with_capacity(resp.len()+4); out.extend_from_slice(&resp); out.extend_from_slice(&id.to_le_bytes());
            write_ws_binary(&mut stream, &out);
        } else if frame.opcode==0x1 {
            let msg=String::from_utf8_lossy(&frame.payload);
            if let Some(survived)=extract_json_value(&msg,"survived") {
                let mut c=immunity.lock().unwrap();
                for p in survived.split('|') { c.entry(p.to_string()).or_insert((0,0)).1+=1; }
                rewrite_immunity(&c); *immunity_str.lock().unwrap()=format_immunity_snapshot(&c); last_poke.clear();
            } else if let Some(poke)=extract_json_value(&msg,"poke") { last_poke=poke.split('|').map(|s|s.to_string()).collect(); }
        }
    }
    if last_poke.len()==1 { let mut c=immunity.lock().unwrap(); c.entry(last_poke[0].clone()).or_insert((0,0)).0+=1; rewrite_immunity(&c); *immunity_str.lock().unwrap()=format_immunity_snapshot(&c); }
}

fn format_immunity_snapshot(c: &HashMap<String,(u32,u32)>) -> String {
    let mut o=String::new(); let mut k: Vec<&String>=c.keys().collect(); k.sort();
    for key in k { let (d,s)=c[key]; if d==0&&s==0 { o.push_str(&format!("immunity {}\n",key)); } else { o.push_str(&format!("immunity {} {} {}\n",key,d,s)); } }
    o
}

fn load_immunity() -> HashMap<String,(u32,u32)> {
    let mut c=HashMap::new();
    if let Ok(content)=std::fs::read_to_string("is/immunity.is") {
        for line in content.lines() { let p: Vec<&str>=line.split_whitespace().collect(); if p.len()>=2&&p[0]=="immunity" { c.insert(p[1].to_string(),(if p.len()>=3{p[2].parse().unwrap_or(0)}else{0}, if p.len()>=4{p[3].parse().unwrap_or(0)}else{0})); } }
    }
    c
}

fn rewrite_immunity(c: &HashMap<String,(u32,u32)>) {
    let mut o=String::new(); let mut k: Vec<&String>=c.keys().collect(); k.sort();
    for key in k { let (d,s)=c[key]; if d==0&&s==0 { o.push_str(&format!("immunity {}\n",key)); } else { o.push_str(&format!("immunity {} {} {}\n",key,d,s)); } }
    let _=std::fs::write("is/immunity.is",o);
}

fn parse_path(s: &str) -> String { let fl=s.lines().next().unwrap_or(""); let p: Vec<&str>=fl.split_whitespace().collect(); if p.len()>=2 { p[1].to_string() } else { "/".to_string() } }
fn emit_void(s: &mut TcpStream) { let _=s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"); }
fn emit(s: &mut TcpStream, st: &str, ct: &str, b: &[u8]) { let _=s.write_all(format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",st,ct,b.len()).as_bytes()); let _=s.write_all(b); }
fn read_signal(s: &mut TcpStream) -> Option<String> {
    let mut buf=[0u8;8192]; let mut acc=Vec::new();
    loop { match s.read(&mut buf) { Ok(0)=>return None, Ok(n)=>{ acc.extend_from_slice(&buf[..n]); if acc.windows(4).any(|w|w==b"\r\n\r\n") { return Some(String::from_utf8_lossy(&acc).to_string()); } if acc.len()>65536 { return None; } } Err(_)=>return None } }
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

fn main() {
    let port: u16 = std::env::var("PORT").ok().and_then(|s|s.parse().ok()).unwrap_or(8080);
    let archive = Arc::new(Archive {
        sources: load_sources(),
        idx: std::fs::read("is/measured.idx").unwrap_or_default(),
        dat: std::fs::read("is/measured.dat").unwrap_or_default(),
        index_html: std::fs::read("crates/server/static/index.html").unwrap_or_default(),
        world_js: std::fs::read("crates/server/static/world.js").unwrap_or_default(),
        cache: Mutex::new(HashMap::new()),
    });
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let immunity = Arc::new(Mutex::new(load_immunity()));
    let immunity_str = Arc::new(Mutex::new(format_immunity_snapshot(&immunity.lock().unwrap())));
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let im = Arc::clone(&immunity); let is = Arc::clone(&immunity_str); let ar = Arc::clone(&archive);
            thread::spawn(move || handle_observer(stream, im, is, ar));
        }
    }
}
