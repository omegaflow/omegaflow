use super::*;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
pub const PORT_CONST: u16 = 1618;
struct WsConfig {
    bodies: Arc<Vec<String>>,
    index_html: Vec<u8>,
    constants_js: Vec<u8>,
    field_rx: mpsc::Receiver<Arc<Buffer>>,
    osc_tx: mpsc::Sender<Vec<Oscillator>>,
    presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
    time: Arc<Mutex<Option<LeapSeconds>>>,
}
pub struct TcpRadiator {
    shutdown: Arc<AtomicBool>,
    field_tx: mpsc::Sender<Arc<Buffer>>,
    _thread: Option<thread::JoinHandle<()>>,
}

impl TcpRadiator {
    pub fn new(
        port: u16,
        bodies: Arc<Vec<String>>,
        initial_field: Arc<Buffer>,
        index_html: Vec<u8>,
        constants_js: Vec<u8>,
        osc_tx: mpsc::Sender<Vec<Oscillator>>,
        presence_tx: mpsc::Sender<(f64, f64, f64, f64, f64)>,
        time: Arc<Mutex<Option<LeapSeconds>>>,
    ) -> Self {
        let (field_tx, field_rx) = mpsc::channel::<Arc<Buffer>>();
        let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(l) => {
                eprintln!("serving on http://127.0.0.1:{}", port);
                l
            }
            Err(e) => {
                eprintln!("TCP bind to 127.0.0.1:{} returned {:?}", port, e.kind());
                std::process::exit(1);
            }
        };
        match listener.set_nonblocking(true) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("TCP set_nonblocking returned {:?}", e.kind());
                std::process::exit(1);
            }
        }
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();
        let handle = thread::spawn(move || {
            let mut field_txs: Vec<mpsc::SyncSender<Arc<Buffer>>> = Vec::new();
            let mut latest: Arc<Buffer> = initial_field;
            loop {
                if shutdown_clone.load(Ordering::Relaxed) {
                    break;
                }
                while let Ok((stream, _)) = listener.accept() {
                    let (ftx, frx) = mpsc::sync_channel::<Arc<Buffer>>(2);
                    let _ = ftx.try_send(latest.clone());
                    field_txs.push(ftx);
                    let cfg = WsConfig {
                        bodies: bodies.clone(),
                        index_html: index_html.clone(),
                        constants_js: constants_js.clone(),
                        field_rx: frx,
                        osc_tx: osc_tx.clone(),
                        presence_tx: presence_tx.clone(),
                        time: time.clone(),
                    };
                    thread::spawn(move || handle_ingress(stream, cfg));
                }
                match field_rx.recv_timeout(std::time::Duration::from_secs_f64(2f64.powi(-8))) {
                    Ok(field) => {
                        latest = field;
                        field_txs.retain(|tx| match tx.try_send(latest.clone()) {
                            Ok(_) => true,
                            Err(mpsc::TrySendError::Full(_)) => true,
                            Err(mpsc::TrySendError::Disconnected(_)) => false,
                        });
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });
        Self {
            shutdown,
            field_tx,
            _thread: Some(handle),
        }
    }
}

impl Radiator for TcpRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        let _ = self.field_tx.send(field);
    }
}

impl Drop for TcpRadiator {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}
struct WsFrame {
    opcode: u8,
    payload: Vec<u8>,
}
fn base64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = String::new();
    for c in data.chunks(3) {
        r.push(T[(c[0] >> 2) as usize] as char);
        if c.len() == 1 {
            r.push(T[((c[0] & 0x03) << 4) as usize] as char);
            r.push('=');
            r.push('=');
        } else {
            r.push(T[(((c[0] & 0x03) << 4) | (c[1] >> 4)) as usize] as char);
            if c.len() == 2 {
                r.push(T[((c[1] & 0x0f) << 2) as usize] as char);
                r.push('=');
            } else {
                r.push(T[(((c[1] & 0x0f) << 2) | (c[2] >> 6)) as usize] as char);
                r.push(T[(c[2] & 0x3f) as usize] as char);
            }
        }
    }
    r
}
fn emit(s: &mut TcpStream, st: &str, ct: &str, b: &[u8]) {
    let _=s.write_all(format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store, no-cache, must-revalidate\r\nPragma: no-cache\r\nExpires: 0\r\nConnection: keep-alive\r\n\r\n",st,ct,b.len()).as_bytes());
    let _ = s.write_all(b);
}
fn emit_void(s: &mut TcpStream) {
    let _ =
        s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
}
fn handle_ingress(stream: TcpStream, cfg: WsConfig) {
    let mut s = stream;
    s.set_nodelay(true).ok();
    let signal = match read_signal(&mut s) {
        Some(r) => r,
        None => return,
    };
    if signal.to_lowercase().contains("upgrade: websocket") {
        resonance(s, &signal, cfg);
    } else {
        let mut last_field: Option<Arc<Buffer>> = None;
        let mut cur = signal;
        loop {
            let path = parse_path(&cur);
            match path.as_str() {
                "/" => {
                    let page = match std::fs::read(resolve_asset("static/index.html")) {
                        Ok(v) => v,
                        Err(_) => cfg.index_html.clone(),
                    };
                    emit(&mut s, "200 OK", "text/html", &page);
                }
                "/time" => match system_now(&cfg.time) {
                    Some(tdb) => {
                        emit(&mut s, "200 OK", "text/plain", tdb.to_string().as_bytes());
                    }
                    None => {
                        emit_void(&mut s);
                        break;
                    }
                },
                "/device" => {
                    let now = match system_now(&cfg.time) {
                        Some(t) => t,
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    };
                    let result = {
                        let buf = {
                            if let Ok(f) = cfg.field_rx.try_recv() {
                                last_field = Some(f);
                            }
                            last_field.clone().unwrap_or_else(|| {
                                Arc::new(build_buffer(
                                    Vec::new(),
                                    1.0,
                                    Arc::new(HashMap::new()),
                                    None,
                                    None,
                                ))
                            })
                        };
                        let eph_map = buf.eph.clone();
                        let mut device_sample: Option<Oscillator> = None;
                        for hash in buf.bodies.values().chain(std::iter::once(&buf.inertial)) {
                            for v in hash.cells.values().chain(std::iter::once(&hash.unbounded)) {
                                for osc in v {
                                    if matches!(osc.source, OscillatorSource::Device) {
                                        let newer = match &device_sample {
                                            Some(cur) => osc.epoch > cur.epoch,
                                            None => true,
                                        };
                                        if newer {
                                            device_sample = Some(osc.clone());
                                        }
                                    }
                                }
                            }
                        }
                        device_sample.and_then(|osc| {
                            let p0 = osc.motion.at(now, osc.epoch, &eph_map)?;
                            let p1 = osc.motion.at(now + 1.0, osc.epoch, &eph_map)?;
                            Some((p0, [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]]))
                        })
                    };
                    match result {
                        Some((p, v)) => emit(
                            &mut s,
                            "200 OK",
                            "text/plain",
                            format!("{} {} {} {} {} {}", p[0], p[1], p[2], v[0], v[1], v[2])
                                .as_bytes(),
                        ),
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    }
                }
                _ if path.starts_with("/jump/") => {
                    let body: &str = &path[6..];
                    let eph = {
                        if let Ok(f) = cfg.field_rx.try_recv() {
                            last_field = Some(f);
                        }
                        last_field
                            .as_ref()
                            .map(|b| b.eph.clone())
                            .unwrap_or_else(|| Arc::new(HashMap::new()))
                    };
                    let now = match system_now(&cfg.time) {
                        Some(t) => t,
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    };
                    match body_barycenter_position(body, now, &eph) {
                        Some([x, y, z]) => emit(
                            &mut s,
                            "200 OK",
                            "text/plain",
                            format!("{} {} {}", x, y, z).as_bytes(),
                        ),
                        None => {
                            emit_void(&mut s);
                            break;
                        }
                    }
                }
                "/field" => {
                    let buf = {
                        if let Ok(f) = cfg.field_rx.try_recv() {
                            last_field = Some(f);
                        }
                        last_field.clone().unwrap_or_else(|| {
                            Arc::new(build_buffer(
                                Vec::new(),
                                1.0,
                                Arc::new(HashMap::new()),
                                None,
                                None,
                            ))
                        })
                    };
                    let mut report = String::new();
                    let mut hashes: Vec<(&str, &SpatialHash)> =
                        buf.bodies.iter().map(|(k, v)| (k.as_str(), v)).collect();
                    hashes.push(("inertial", &buf.inertial));
                    for (fname, hash) in hashes {
                        let mut n = 0usize;
                        let mut field_names: std::collections::HashSet<&str> =
                            std::collections::HashSet::new();
                        let mut src_ids: std::collections::HashSet<u32> =
                            std::collections::HashSet::new();
                        for v in hash.cells.values().chain(std::iter::once(&hash.unbounded)) {
                            for osc in v {
                                n += 1;
                                field_names.insert(osc.name.as_str());
                                match osc.source {
                                    OscillatorSource::Api(id) => {
                                        src_ids.insert(id);
                                    }
                                    _ => {}
                                };
                            }
                        }
                        report.push_str(&format!(
                            "{} samples={} cells={} unbounded={} vmax={:.3e} epoch_min={:.1} origins={}\n",
                            fname,
                            n,
                            hash.cells.len(),
                            hash.unbounded.len(),
                            hash.vmax,
                            hash.epoch_min,
                            src_ids.len()
                        ));
                        let mut names: Vec<&str> = field_names.into_iter().collect();
                        names.sort();
                        report.push_str(&format!("{} fields: {}\n", fname, names.len()));
                        for nm in names {
                            report.push_str(&format!("  {}\n", nm));
                        }
                    }
                    report.push_str(&format!("ephemerides={}\n", buf.eph.len()));
                    emit(&mut s, "200 OK", "text/plain", report.as_bytes());
                }
                "/constants.js" => {
                    let mut page = match std::fs::read(resolve_asset("static/constants.js")) {
                        Ok(v) => v,
                        Err(_) => cfg.constants_js.clone(),
                    };
                    let mut extra = String::from("\nexport const BODY_REGISTRY = {");
                    for (i, name) in cfg.bodies.iter().enumerate() {
                        extra.push_str(&format!("{}:\"{}\",", i + 1, name));
                    }
                    extra.push_str("};\n");
                    page.extend_from_slice(extra.as_bytes());
                    emit(&mut s, "200 OK", "application/javascript", &page);
                }
                "/crash" => {
                    emit(&mut s, "200 OK", "text/plain", &[]);
                    break;
                }
                _ => {
                    emit_void(&mut s);
                    break;
                }
            }
            match read_signal(&mut s) {
                Some(r) => cur = r,
                None => break,
            }
        }
    }
}
fn resonance(mut stream: TcpStream, signal: &str, cfg: WsConfig) {
    let key = match extract_header(signal, "Sec-WebSocket-Key") {
        Some(k) => k,
        None => return,
    };
    let encoded = base64_encode(&sha1(
        &format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").into_bytes(),
    ));
    if stream.write_all(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", encoded).as_bytes()).is_err() { return; }
    let mut last_field_r: Option<Arc<Buffer>> = None;
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

            let mut browser: Vec<(String, f64, f64)> = Vec::with_capacity(oscillator_count);
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
                    let mut tau_buf = [0u8; 8];
                    let tau = if cursor.read_exact(&mut tau_buf).is_ok() {
                        f64::from_le_bytes(tau_buf)
                    } else {
                        0.0
                    };

                    browser.push((name, value, tau));
                }
            }

            if cursor.read_exact(&mut buf4).is_err() {
                continue;
            }
            let query_count = u32::from_le_bytes(buf4) as usize;
            let mut queries: Vec<(f64, f64, f64, f64)> = Vec::with_capacity(query_count);
            for _ in 0..query_count {
                let mut t_buf = [0u8; 8];
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qt = f64::from_le_bytes(t_buf);
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qx = f64::from_le_bytes(t_buf);
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qy = f64::from_le_bytes(t_buf);
                if cursor.read_exact(&mut t_buf).is_err() {
                    break;
                }
                let qz = f64::from_le_bytes(t_buf);
                queries.push((qt, qx, qy, qz));
            }
            let mut delta_t_cache = 0.0f64;
            {
                let mut pb8 = [0u8; 8];
                if cursor.read_exact(&mut pb8).is_ok() {
                    let px = f64::from_le_bytes(pb8);
                    if cursor.read_exact(&mut pb8).is_ok() {
                        let py = f64::from_le_bytes(pb8);
                        if cursor.read_exact(&mut pb8).is_ok() {
                            let pz = f64::from_le_bytes(pb8);
                            if cursor.read_exact(&mut pb8).is_ok() {
                                let pt = f64::from_le_bytes(pb8);
                                if cursor.read_exact(&mut pb8).is_ok() {
                                    let pr = f64::from_le_bytes(pb8);
                                    let _ = cfg.presence_tx.send((pt, px, py, pz, pr));
                                    if cursor.read_exact(&mut pb8).is_ok() {
                                        delta_t_cache = f64::from_le_bytes(pb8);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let field = {
                if let Ok(f) = cfg.field_rx.try_recv() {
                    last_field_r = Some(f);
                }
                last_field_r.clone().unwrap_or_else(|| {
                    Arc::new(build_buffer(
                        Vec::new(),
                        1.0,
                        Arc::new(HashMap::new()),
                        None,
                        None,
                    ))
                })
            };
            let eph_map = field.eph.clone();
            let now = match system_now(&cfg.time) {
                Some(t) => t,
                None => continue,
            };
            let (mut st_lat, mut st_lon, mut st_alt, mut st_body) = (None, None, None, None::<u32>);
            let mut field_values: Vec<(String, f64, f64)> = Vec::new();
            for (name, value, tau) in &browser {
                let tau = *tau;
                match name.as_str() {
                    "lat" => st_lat = Some(*value),
                    "lon" => st_lon = Some(*value),
                    "alt" => st_alt = Some(*value),
                    "body" => {
                        st_body = if *value > 0.0 && *value <= u32::MAX as f64 {
                            Some(*value as u32)
                        } else {
                            None
                        }
                    }
                    _ => {
                        field_values.push((name.clone(), *value, tau));
                    }
                }
            }
            if let (Some(lat), Some(lon), Some(alt)) = (st_lat, st_lon, st_alt) {
                let body_name = match st_body.and_then(|id| body_id_to_name(&cfg.bodies, id)) {
                    Some(n) => n,
                    None => String::new(),
                };
                if eph_map
                    .get(&body_name)
                    .and_then(|e| e.props.as_ref())
                    .is_some()
                {
                    let pos = Position::Surface {
                        body_name: body_name.clone(),
                        lat,
                        lon,
                        alt,
                    };
                    let mut channels: Vec<(Channel, FieldConfig, f64)> = Vec::new();
                    for (name, value, tau) in &field_values {
                        if let Some(bs) = sensor_config(name) {
                            let sensor_ttl = bs.ttl;
                            let effective_tau = if *tau > 0.0 { *tau } else { bs.ttl };
                            let fc = FieldConfig {
                                key: bs.key.clone(),
                                name: bs.key.clone(),
                                kernel: bs.kernel,
                                force: bs.force,
                                tau: effective_tau,
                                absorption: 0.0,
                                advection: 0.0,
                            };
                            if value.is_finite() {
                                channels.push((
                                    Channel {
                                        epoch: now,
                                        position: pos.clone(),
                                        name: fc.name.clone(),
                                        value: *value,
                                    },
                                    fc,
                                    sensor_ttl,
                                ));
                            }
                        }
                    }
                    let mut oscillators = Vec::new();
                    for (channel, sensor, sensor_ttl) in channels {
                        if let Some(osc) =
                            anchor(&channel, &sensor, sensor_ttl, None, None, None, &eph_map)
                        {
                            oscillators.push(osc);
                        }
                    }
                    if !oscillators.is_empty() {
                        let _ = cfg.osc_tx.send(oscillators);
                    }
                }
            } else if let Some(body_id) = st_body {
                let body_name = match body_id_to_name(&cfg.bodies, body_id) {
                    Some(n) => n,
                    None => String::new(),
                };
                let frame = Frame::Barycenter {
                    body_name: body_name.clone(),
                    scale: 1.0,
                };
                let pos = Position::Source;
                let mut channels: Vec<(Channel, FieldConfig, f64)> = Vec::new();
                for (name, value, tau) in &field_values {
                    if let Some(bs) = sensor_config(name) {
                        let sensor_ttl = bs.ttl;
                        let effective_tau = if *tau > 0.0 { *tau } else { bs.ttl };
                        let fc = FieldConfig {
                            key: bs.key.clone(),
                            name: bs.key.clone(),
                            kernel: bs.kernel,
                            force: bs.force,
                            tau: effective_tau,
                            absorption: 0.0,
                            advection: 0.0,
                        };
                        if value.is_finite() {
                            channels.push((
                                Channel {
                                    epoch: now,
                                    position: pos.clone(),
                                    name: fc.name.clone(),
                                    value: *value,
                                },
                                fc,
                                sensor_ttl,
                            ));
                        }
                    }
                }
                let mut oscillators = Vec::new();
                for (channel, sensor, sensor_ttl) in channels {
                    if let Some(osc) = anchor(
                        &channel,
                        &sensor,
                        sensor_ttl,
                        None,
                        Some(&frame),
                        None,
                        &eph_map,
                    ) {
                        oscillators.push(osc);
                    }
                }
                if !oscillators.is_empty() {
                    let _ = cfg.osc_tx.send(oscillators);
                }
            }
            let mut records: Vec<(
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
            )> = Vec::new();
            let response_epoch;
            if !queries.is_empty() {
                let (t0, x0, y0, z0) = queries[0];
                let mut extent = 0.0f64;
                for &(_, qx, qy, qz) in &queries[1..] {
                    let d = ((qx - x0).powi(2) + (qy - y0).powi(2) + (qz - z0).powi(2)).sqrt();
                    if d > extent {
                        extent = d;
                    }
                }
                let center = [x0, y0, z0];
                sense_buffer(
                    &field,
                    center,
                    t0,
                    extent,
                    delta_t_cache,
                    &mut records,
                    &eph_map,
                );
                response_epoch = t0;
            } else {
                response_epoch = now;
            }

            let mut out = Vec::with_capacity(19 + records.len() * 168);
            out.extend_from_slice(&[0xCF, 0x86]);
            out.push(6u8);
            out.extend_from_slice(&response_epoch.to_le_bytes());
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(records.len() as u32).to_le_bytes());
            for &(
                x,
                y,
                z,
                val,
                epoch,
                ttl,
                tau,
                extent,
                kernel_id,
                force_type,
                absorption,
                advection,
                vx,
                vy,
                vz,
                pole_x,
                pole_y,
                pole_z,
                j2,
                j4,
                r_eq,
            ) in &records
            {
                out.extend_from_slice(&x.to_le_bytes());
                out.extend_from_slice(&y.to_le_bytes());
                out.extend_from_slice(&z.to_le_bytes());
                out.extend_from_slice(&val.to_le_bytes());
                out.extend_from_slice(&epoch.to_le_bytes());
                out.extend_from_slice(&ttl.to_le_bytes());
                out.extend_from_slice(&tau.to_le_bytes());
                out.extend_from_slice(&extent.to_le_bytes());
                out.extend_from_slice(&kernel_id.to_le_bytes());
                out.extend_from_slice(&force_type.to_le_bytes());
                out.extend_from_slice(&absorption.to_le_bytes());
                out.extend_from_slice(&advection.to_le_bytes());
                out.extend_from_slice(&vx.to_le_bytes());
                out.extend_from_slice(&vy.to_le_bytes());
                out.extend_from_slice(&vz.to_le_bytes());
                out.extend_from_slice(&pole_x.to_le_bytes());
                out.extend_from_slice(&pole_y.to_le_bytes());
                out.extend_from_slice(&pole_z.to_le_bytes());
                out.extend_from_slice(&j2.to_le_bytes());
                out.extend_from_slice(&j4.to_le_bytes());
                out.extend_from_slice(&r_eq.to_le_bytes());
            }
            write_ws_binary(&mut stream, &out);
        }
    }
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
fn read_ws_frame_part(stream: &mut TcpStream) -> Option<(u8, bool, Vec<u8>)> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).ok()?;
    let fin = (header[0] & 0x80) != 0;
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
    if plen > 1 << 24 {
        return None;
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
    Some((opcode, fin, payload))
}
fn read_ws_frame_raw(stream: &mut TcpStream) -> Option<WsFrame> {
    let (opcode, mut fin, mut payload) = read_ws_frame_part(stream)?;
    while !fin {
        let (next_op, next_fin, mut next_payload) = read_ws_frame_part(stream)?;
        match next_op {
            0x0 => {
                if payload.len() + next_payload.len() > 1 << 24 {
                    return None;
                }
                payload.append(&mut next_payload);
                fin = next_fin;
            }
            0x8 => {
                return Some(WsFrame {
                    opcode: 0x8,
                    payload: next_payload,
                });
            }
            _ => {}
        }
    }
    Some(WsFrame { opcode, payload })
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
