use super::*;

pub struct StderrRadiator {
    pub last_line: String,
    pub interactive: bool,
}

impl Radiator for StderrRadiator {
    fn accept(&mut self, field: Arc<Buffer>) {
        let mut body_samples = 0usize;
        let mut api_samples = 0usize;
        let mut sensor_samples = 0usize;
        let mut body_src: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut api_src: std::collections::HashSet<u32> = std::collections::HashSet::new();
        for cell in field
            .cache
            .cells
            .values()
            .chain(std::iter::once(&field.cache.unbounded))
        {
            for sample in cell {
                match sample.source {
                    SampleSource::Sensor => sensor_samples += 1,
                    SampleSource::Source(idx) => {
                        api_samples += 1;
                        api_src.insert(idx);
                    }
                    SampleSource::Ephemeris => {
                        body_samples += 1;
                        body_src.insert(sample.name.split('.').next().unwrap_or("").to_string());
                    }
                }
            }
        }
        let line = format!(
            "omegaflow v{} | φ v8 | body: {} sources, {} samples | api: {} sources, {} samples | sensor: {} samples",
            env!("CARGO_PKG_VERSION"),
            body_src.len(),
            body_samples,
            api_src.len(),
            api_samples,
            sensor_samples,
        );
        let prev_len = self.last_line.chars().count();
        if self.interactive {
            let pad = " ".repeat(prev_len.saturating_sub(line.chars().count()));
            eprint!("\r{}{}", line, pad);
        } else if line != self.last_line {
            eprintln!("{}", line);
        }
        self.last_line = line;
    }
}

pub struct Archive {
    pub sources: Vec<SourceConfig>,
    pub body_ephemerides: Arc<HashMap<String, BodyEphemeris>>,
    pub field: Arc<Buffer>,
    pub presence: HashMap<String, (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>,
    pub jump_epoch: Option<f64>,
    pub declared_body: Option<DeclaredBody>,
    pub origins: HashMap<Origin, OriginState>,
    pub pck_bodies: HashMap<i32, PckBody>,
    pub time: Arc<Mutex<Option<LeapSeconds>>>,
    pub asteroid_samples: Vec<Sample>,
    pub star_samples: Vec<Sample>,
    pub curves: Option<Arc<CurveSet>>,
    pub spectral: Vec<SpectralHash>,
    pub pending_channels: Vec<(Channel, FieldConfig, u32)>,
    pub fetch_durations: [f64; FETCH_DURATION_RING],
    pub fetch_duration_len: usize,
    pub fetch_duration_idx: usize,
}

pub fn anchor_uses(sources: &[SourceConfig]) -> std::collections::HashMap<String, usize> {
    let mut uses = std::collections::HashMap::new();
    for s in sources {
        if s.format == "ephemeris_binary" || s.format == "kernel_text" || s.format == "orbit_bin" {
            continue;
        }
        match &s.frame {
            Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => {
                *uses.entry(body_name.clone()).or_insert(0) += 1;
            }
            Frame::Manifest => {}
        }
    }
    uses
}

pub fn spawn_ephemeris_bootstrap(
    sources: &[SourceConfig],
    guard: &std::sync::Arc<std::sync::atomic::AtomicBool>,
    fetch_tx: mpsc::Sender<FetchResult>,
    time: std::sync::Arc<std::sync::Mutex<Option<LeapSeconds>>>,
) {
    if guard.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return;
    }
    let anchor_uses = anchor_uses(sources);
    let anchor_order = |s: &SourceConfig| {
        std::cmp::Reverse(
            anchor_uses
                .get(s.body.as_deref().unwrap_or(""))
                .copied()
                .unwrap_or(0),
        )
    };
    let mut fresh_items: Vec<(usize, SourceConfig, String)> = Vec::new();
    let mut anchor_items: Vec<(usize, SourceConfig, String)> = Vec::new();
    let mut rest_items: Vec<(usize, SourceConfig, String)> = Vec::new();
    for (i, s) in sources.iter().enumerate() {
        if s.format != "ephemeris_binary" && s.format != "orbit_bin" {
            continue;
        }
        let Some(body) = &s.body else {
            continue;
        };
        let tmp_path = format!("/tmp/omegaflow_eph_{}.bin", body);
        if cache_fresh(&tmp_path, s.ttl) {
            fresh_items.push((i, s.clone(), tmp_path));
        } else if anchor_uses.contains_key(body) {
            anchor_items.push((i, s.clone(), tmp_path));
        } else {
            rest_items.push((i, s.clone(), tmp_path));
        }
    }
    fresh_items.sort_by_key(|(_, s, _)| anchor_order(s));
    anchor_items.sort_by_key(|(_, s, _)| anchor_order(s));
    if fresh_items.is_empty() && anchor_items.is_empty() && rest_items.is_empty() {
        guard.store(false, std::sync::atomic::Ordering::SeqCst);
        return;
    }
    let guard = guard.clone();
    thread::spawn(move || {
        let lsk = match leap_seconds(&time) {
            Some(l) => l,
            None => {
                guard.store(false, std::sync::atomic::Ordering::SeqCst);
                return;
            }
        };
        let now = lsk.system_now_tdb().unwrap_or(0.0);
        for (i, s, p) in fresh_items {
            load_ephemeris_cache(&fetch_tx, i, &s, &p, now, &lsk);
        }
        let mut stale = anchor_items;
        stale.extend(rest_items);
        download_ephemeris_batch(&stale);
        for (i, s, p) in stale {
            load_ephemeris_cache(&fetch_tx, i, &s, &p, now, &lsk);
        }
        guard.store(false, std::sync::atomic::Ordering::SeqCst);
    });
}

fn load_ephemeris_cache(
    fetch_tx: &mpsc::Sender<FetchResult>,
    source_idx: usize,
    src: &SourceConfig,
    tmp_path: &str,
    now: f64,
    lsk: &LeapSeconds,
) {
    if let ExtractResult::WithEphemeris(_, eph) = extract(src, tmp_path, now, lsk) {
        let _ = fetch_tx.send(FetchResult {
            source_idx,
            channels: Vec::new(),
            eph_update: src.body.clone().map(|b| (b, eph)),
            asteroid_samples: Vec::new(),
            star_samples: Vec::new(),
            curves: None,
            spectral: None,
            fetch_ok: true,
        });
    }
}

pub fn download_ephemeris_batch(items: &[(usize, SourceConfig, String)]) {
    if items.is_empty() {
        return;
    }
    let ttl = items[0].1.ttl;
    let parts: Vec<String> = items
        .iter()
        .map(|(_, _, p)| format!("{}.part", p))
        .collect();
    let mut pending: Vec<(usize, String, String)> = Vec::new();
    for (i, (_, s, tmp_path)) in items.iter().enumerate() {
        if let Ok(data) = std::fs::read(&parts[i]) {
            let valid = if s.format == "orbit_bin" {
                crate::wind_orbit::parse_bin(&data).is_some()
            } else {
                parse_ephemeris_binary(&data).is_some()
            };
            if valid {
                let _ = std::fs::rename(&parts[i], tmp_path);
                continue;
            }
        }
        let _ = std::fs::remove_file(&parts[i]);
        pending.push((i, parts[i].clone(), tmp_path.clone()));
    }
    if pending.is_empty() {
        return;
    }
    let mut cmd = curl_base(ttl, 8);
    for (i, part, _) in &pending {
        cmd.arg("-o").arg(part).arg(&items[*i].1.url);
    }
    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => {
            eprintln!("ephemeris batch ({} files): curl void", pending.len());
            for (_, part, _) in &pending {
                let _ = std::fs::remove_file(part);
            }
            return;
        }
    };
    if output.status.success() {
        for (_, part, tmp_path) in &pending {
            if std::fs::rename(part, tmp_path).is_err() {
                let _ = std::fs::remove_file(part);
            }
        }
    } else {
        eprintln!(
            "ephemeris batch ({} files): curl returned {}: {}",
            pending.len(),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        for (_, part, _) in &pending {
            let _ = std::fs::remove_file(part);
        }
    }
}

pub struct RefusalLedger {
    pub path: std::path::PathBuf,
    pub seen: std::collections::HashSet<String>,
}

impl RefusalLedger {
    pub fn new(path: &str) -> RefusalLedger {
        let mut seen = std::collections::HashSet::new();
        if let Ok(content) = std::fs::read_to_string(path) {
            for line in content.lines() {
                let mut parts = line.splitn(4, ' ');
                if parts.next() == Some("refused") {
                    let _unix = parts.next();
                    let class = parts.next().unwrap_or("");
                    let url = parts.next().unwrap_or("");
                    if !class.is_empty() && !url.is_empty() {
                        seen.insert(format!("{}|{}", class, url));
                    }
                }
            }
        }
        RefusalLedger {
            path: std::path::PathBuf::from(path),
            seen,
        }
    }

    pub fn register(&mut self, url: &str, class: &str) {
        let key = format!("{}|{}", class, url);
        if !self.seen.insert(key) {
            return;
        }
        let unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let line = format!("refused {} {} {}\n", unix, class, url);
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            use std::io::Write;
            let _ = f.write_all(line.as_bytes());
        }
    }
}

pub fn main_flow() {
    let env = Arc::new(load_env());
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && args[1] == "--verify" {
            let dir = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--verify: directory argument absent");
                    std::process::exit(1);
                }
            };
            ANOMALY_COLLECT.with(|c| c.set(true));
            std::process::exit(ci_mode(dir));
        }
        if args.len() > 1 && args[1] == "--port" {
            let input = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--port: input file argument absent");
                    std::process::exit(1);
                }
            };
            let output = match args.get(3) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--port: output file argument absent");
                    std::process::exit(1);
                }
            };
            std::process::exit(port_mode(input, output));
        }
        if args.len() > 1 && args[1] == "--learn-gate" {
            std::process::exit(gate_learn_mode());
        }
        if args.len() > 1 && args[1] == "--draft-context" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--draft-context: file argument absent");
                    std::process::exit(1);
                }
            };
            std::process::exit(draft_context_mode(path));
        }
        if args.len() > 1 && args[1] == "--draft" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--draft: file argument absent");
                    std::process::exit(1);
                }
            };
            let fetchone = args.iter().any(|a| a == "--fetchone");
            std::process::exit(draft_url_mode(path, &env, fetchone));
        }
        if args.len() > 1 && args[1] == "--urls" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--urls: file argument absent");
                    std::process::exit(1);
                }
            };
            let fetchone = args.iter().any(|a| a == "--fetchone");
            let jina = args.iter().any(|a| a == "--jina");
            std::process::exit(url_probe_mode(path, &env, fetchone, jina));
        }
        if args.len() > 1 && args[1] == "--probe" {
            let path = match args.get(2) {
                Some(s) => s.as_str(),
                None => {
                    eprintln!("--probe: file argument absent");
                    std::process::exit(1);
                }
            };
            let precise = args.iter().any(|a| a == "--precise");
            let fetchone = args.iter().any(|a| a == "--fetchone");
            let mut lat = 0.0;
            let mut lon = 0.0;
            let mut i = 2;
            while i < args.len() {
                if args[i] == "--lat" {
                    if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        lat = v;
                        i += 1;
                    }
                } else if args[i] == "--lon" {
                    if let Some(v) = args.get(i + 1).and_then(|s| s.parse::<f64>().ok()) {
                        lon = v;
                        i += 1;
                    }
                }
                i += 1;
            }
            std::process::exit(probe_mode(path, precise, lat, lon, &env, fetchone));
        }
        if args.len() > 1 && args[1] == "--reverify" {
            std::process::exit(reverify_mode(&env));
        }
    }
    let declared_body: Option<DeclaredBody> = std::env::args().skip(1).find_map(|a| {
        let rest = a.strip_prefix("#body=")?;
        let parts: Vec<&str> = rest.split(',').collect();
        if parts.len() < 3 {
            return None;
        }
        let lat: f64 = parts[1].parse().ok()?;
        let lon: f64 = parts[2].parse().ok()?;
        let alt: Option<f64> = parts.get(3).and_then(|s| s.parse().ok());
        Some(DeclaredBody {
            body_name: parts[0].to_string(),
            lat,
            lon,
            alt,
        })
    });
    if declared_body.is_none() {
        eprintln!(
            "native body undeclared — station samples refused (declare via #body=<body>,<lat>,<lon>,<alt>)"
        );
    }
    let loaded = load_sources();
    let refusal_ledger = Arc::new(Mutex::new(RefusalLedger::new(
        "phi/pipeline/refusal_ledger.φ",
    )));
    let (sensor_tx, sensor_rx) = mpsc::channel::<Vec<(String, f64, f64)>>();
    let consent = Arc::new(AtomicBool::new(false));
    eprintln!("record consent: silent until the operator speaks (browser Y/N relay pending)");
    let serial_tx = sensor_tx.clone();
    thread::spawn(move || serial_ingress(serial_tx));
    let battery_tx = sensor_tx.clone();
    thread::spawn(move || battery_ingress(battery_tx));
    #[cfg(feature = "browser_relay")]
    let port: u16 = match std::env::var("PORT").ok().and_then(|s| s.parse().ok()) {
        Some(p) => p,
        None => crate::relay::PORT_CONST,
    };
    let (fetch_tx, fetch_rx) = mpsc::channel::<FetchResult>();
    #[cfg(feature = "browser_relay")]
    let (sample_tx, sample_rx) = mpsc::channel::<Vec<Sample>>();
    #[cfg(not(feature = "browser_relay"))]
    let sample_rx = mpsc::channel::<Vec<Sample>>().1;
    let (presence_tx, presence_rx) =
        mpsc::channel::<(String, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)>();
    let body_ephemerides = Arc::new(HashMap::new());
    #[cfg(feature = "browser_relay")]
    let index_html = match std::fs::read(resolve_asset("static/index.html")) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("static/index.html absent — serving 0 bytes");
            Vec::new()
        }
    };
    #[cfg(feature = "browser_relay")]
    let constants_js = match std::fs::read(resolve_asset("static/constants.js")) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("static/constants.js absent — browser protocol empty");
            Vec::new()
        }
    };
    let time: Arc<Mutex<Option<LeapSeconds>>> = Arc::new(Mutex::new(embedded_lsk()));
    let (solar_tx, solar_rx) = mpsc::channel::<crate::machines::SolarCell>();
    let (machine_tx, machine_rx) = mpsc::channel::<(
        crate::archivar::Frame,
        Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
    )>();
    let solar_sources = loaded.clone();
    let solar_time = time.clone();
    thread::spawn(move || crate::machines::solar_harvest(solar_tx, solar_sources, solar_time));
    let mut archive = Archive {
        sources: loaded,
        body_ephemerides: body_ephemerides.clone(),
        field: Arc::new(build_buffer(
            Vec::new(),
            1.0,
            body_ephemerides.clone(),
            None,
            Vec::new(),
        )),
        presence: HashMap::new(),
        jump_epoch: None,
        declared_body,
        origins: HashMap::new(),
        pck_bodies: HashMap::new(),
        time: time.clone(),
        asteroid_samples: Vec::new(),
        star_samples: Vec::new(),
        curves: None,
        spectral: Vec::new(),
        pending_channels: Vec::new(),
        fetch_durations: [0.0; FETCH_DURATION_RING],
        fetch_duration_len: 0,
        fetch_duration_idx: 0,
    };
    #[cfg(feature = "browser_relay")]
    let body_names: Arc<Vec<String>> = {
        let mut names: Vec<String> = archive
            .sources
            .iter()
            .filter(|s| s.format != "kernel_text")
            .filter_map(|s| s.body.clone())
            .collect();
        names.sort();
        names.dedup();
        Arc::new(names)
    };
    let mut radiators: Vec<Box<dyn Radiator>> = Vec::new();
    #[cfg(feature = "browser_relay")]
    let presence_relay_tx = presence_tx.clone();
    let hidden = std::env::var("OMEGAFLOW_HIDDEN").is_ok();
    let presence_slot: std::sync::Arc<std::sync::RwLock<crate::mathematikerin::PresenceState>> =
        std::sync::Arc::new(std::sync::RwLock::new(
            crate::mathematikerin::PresenceState::rest(),
        ));
    let diode: std::sync::Arc<std::sync::RwLock<crate::mathematikerin::DiodeState>> =
        std::sync::Arc::new(std::sync::RwLock::new(crate::mathematikerin::DiodeState {
            force_ref: [0.0; 9],
            expose_offset: crate::mathematikerin::EXPOSE_OFFSET_BASE,
        }));
    let (acoustic_tx, acoustic_rx) = mpsc::channel::<crate::mathematikerin::PresenceFrame>();
    let (seismic_tx, seismic_rx) = mpsc::channel::<crate::mathematikerin::PresenceFrame>();
    if !hidden {
        let mut kinetic: Vec<Box<dyn crate::mathematikerin::KineticRadiator>> = Vec::new();
        if let Ok(port) = std::env::var("OMEGAFLOW_SERIAL_OUT") {
            kinetic.push(Box::new(crate::mathematikerin::SeismicOscillator::new(
                &port,
            )));
        }
        thread::spawn(move || {
            while let Ok(frame) = seismic_rx.recv() {
                for s in kinetic.iter_mut() {
                    s.vibrate(&frame);
                }
            }
        });
    }
    let boot_pt = time
        .lock()
        .ok()
        .and_then(|l| l.as_ref().and_then(|l| l.system_now_tdb()));
    if let Some(pt) = boot_pt {
        let rest = crate::mathematikerin::PresenceState::rest();
        if let Ok(mut slot) = presence_slot.write() {
            *slot = rest;
        }
        let _ = presence_tx.send((
            "browser".to_string(),
            pt,
            0.0,
            0.0,
            0.0,
            rest.range,
            0.0,
            0.0,
            0.0,
            0.0,
            rest.grid_step,
        ));
    }
    let em_shutdown = if std::env::var("OMEGAFLOW_HEADLESS").is_ok() {
        std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false))
    } else {
        let em = crate::mathematikerin::LoopRadiator::new(
            time.clone(),
            consent.clone(),
            acoustic_tx,
            seismic_tx,
            solar_rx,
            machine_rx,
            presence_slot.clone(),
            diode.clone(),
        );
        let em_shutdown = em.shutdown_flag();
        radiators.push(Box::new(em));
        em_shutdown
    };
    #[cfg(feature = "browser_relay")]
    {
        if !hidden {
            let sr = crate::relay::TcpRadiator::new(
                port,
                body_names.clone(),
                archive.field.clone(),
                index_html.clone(),
                constants_js.clone(),
                sample_tx.clone(),
                presence_relay_tx,
                time.clone(),
                consent.clone(),
                diode.clone(),
            );
            radiators.push(Box::new(sr));
        }
    }
    if !hidden {
        let _acoustic = crate::mathematikerin::AcousticOscillator::new(acoustic_rx);
    }
    radiators.push(Box::new(StderrRadiator {
        last_line: String::new(),
        interactive: std::io::stderr().is_terminal(),
    }));
    let cadence = 1.0;
    let mut gm_text: Option<String> = None;
    let mut pck_text: Option<String> = None;
    let bootstrap_running: std::sync::Arc<std::sync::atomic::AtomicBool> =
        std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    spawn_ephemeris_bootstrap(
        &archive.sources,
        &bootstrap_running,
        fetch_tx.clone(),
        archive.time.clone(),
    );
    let mut last_bootstrap: f64 = 0.0;
    let mut tick: u64 = 0;
    loop {
        tick += 1;
        if em_shutdown.load(Ordering::SeqCst) {
            eprintln!("the window closed — the ω-loop ends");
            break;
        }
        while let Ok((name, pt, px, py, pz, pr, vx, vy, vz, tt, gs)) = presence_rx.try_recv() {
            if name == "browser" {
                if let Some(&(_, opx, opy, opz, ..)) = archive.presence.get("browser") {
                    let dx = px - opx;
                    let dy = py - opy;
                    let dz = pz - opz;
                    let dp = (dx * dx + dy * dy + dz * dz).sqrt();
                    if dp >= 4.0 * crate::mathematikerin::JUMP_GRID
                        && vx * vx + vy * vy + vz * vz == 0.0
                    {
                        archive.jump_epoch = Some(pt);
                    }
                }
                if let Ok(mut slot) = presence_slot.write() {
                    slot.p = [px, py, pz];
                    slot.v = [vx, vy, vz];
                    if gs.is_finite() && gs > 0.0 {
                        slot.grid_step = gs;
                    }
                    slot.range = pr;
                    slot.t_thrust = tt;
                }
            }
            archive
                .presence
                .insert(name, (pt, px, py, pz, pr, vx, vy, vz, tt, gs));
        }
        let now = match system_now(&archive.time) {
            Some(t) => t,
            None => {
                thread::sleep(std::time::Duration::from_secs_f64(cadence));
                continue;
            }
        };
        for i in 0..archive.sources.len() {
            if archive.sources[i].format != "kernel_text" {
                continue;
            }
            let Some(kernel_body) = archive.sources[i].body.clone() else {
                continue;
            };
            let url = archive.sources[i].url.clone();
            let ttl = archive.sources[i].ttl;
            let cache_path = format!("/tmp/omegaflow_kernel_{}.txt", kernel_body);
            if !cache_fresh(&cache_path, ttl) {
                let Some(text) = fetch_one(&url, None, &[], ttl, Some(now)) else {
                    continue;
                };
                if std::fs::write(&cache_path, text.as_bytes()).is_err() {
                    continue;
                }
            }
            let Ok(text) = std::fs::read_to_string(&cache_path) else {
                continue;
            };
            match kernel_body.as_str() {
                "gm_de440" => gm_text = Some(text),
                "pck00010" | "geophysical" => {
                    pck_text = Some(match pck_text {
                        Some(prev) => prev + "\n" + &text,
                        None => text,
                    });
                }
                "naif0012" => {
                    if let Some(l) = crate::lsk::parse(&text) {
                        if let Ok(mut guard) = archive.time.lock() {
                            *guard = Some(l);
                        }
                    }
                }
                _ => {}
            }
            archive.pck_bodies = crate::pck::parse(gm_text.as_deref(), pck_text.as_deref());
        }
        let lsk = match leap_seconds(&archive.time) {
            Some(l) => l,
            None => {
                eprintln!(
                    "the time base is absent — the process refuses to fabricate a dead field"
                );
                thread::sleep(std::time::Duration::from_secs_f64(cadence));
                continue;
            }
        };
        let wall_entered = match lsk.system_now_tdb() {
            Some(t) => t,
            None => {
                thread::sleep(std::time::Duration::from_secs_f64(cadence));
                continue;
            }
        };
        if tick & 63 == 0 {
            let mut missing_ttl: Option<f64> = None;
            for s in &archive.sources {
                if s.format != "ephemeris_binary" && s.format != "orbit_bin" {
                    continue;
                }
                let Some(body) = &s.body else {
                    continue;
                };
                let tmp_path = format!("/tmp/omegaflow_eph_{}.bin", body);
                if cache_fresh(&tmp_path, s.ttl)
                    || archive
                        .body_ephemerides
                        .get(body)
                        .map(|e| e.props.is_some() || e.orbit.is_some())
                        .unwrap_or(false)
                {
                    continue;
                }
                missing_ttl = Some(s.ttl as f64);
                break;
            }
            if let Some(ttl) = missing_ttl {
                if now - last_bootstrap >= ttl / (Φ * Φ) {
                    last_bootstrap = now;
                    spawn_ephemeris_bootstrap(
                        &archive.sources,
                        &bootstrap_running,
                        fetch_tx.clone(),
                        archive.time.clone(),
                    );
                }
            }
        }
        let mut fetched_samples: Vec<Sample> = Vec::new();
        {
            let mut still_pending: Vec<(Channel, FieldConfig, u32)> = Vec::new();
            for (channel, sensor, idx) in archive.pending_channels.drain(..) {
                let src = &archive.sources[idx as usize];
                match anchor(
                    &channel,
                    &sensor,
                    src.ttl as f64,
                    Some(idx),
                    Some(&src.frame),
                    None,
                    &archive.body_ephemerides,
                ) {
                    Some(sample) => fetched_samples.push(sample),
                    None => still_pending.push((channel, sensor, idx)),
                }
            }
            if !still_pending.is_empty() {
                eprintln!(
                    "{} channels waiting for body ephemerides — anchored on arrival",
                    still_pending.len()
                );
            }
            archive.pending_channels = still_pending;
        }
        let mut dropped_channels: Vec<(Channel, FieldConfig, u32)> = Vec::new();
        while let Ok(res) = fetch_rx.try_recv() {
            let st = archive
                .origins
                .entry(res.source_idx as u32)
                .or_insert(OriginState {
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
            let fetch_duration = now - st.started;
            settle_fetch(st, res.fetch_ok, now);
            if fetch_duration.is_finite() && fetch_duration > 0.0 {
                record_fetch_duration(
                    &mut archive.fetch_durations,
                    &mut archive.fetch_duration_len,
                    &mut archive.fetch_duration_idx,
                    fetch_duration,
                );
            }
            if let Some((name, eph)) = res.eph_update {
                let mut eph_map = (*archive.body_ephemerides).clone();
                eph_map.insert(name, eph);
                archive.body_ephemerides = Arc::new(eph_map);
            }
            if !res.asteroid_samples.is_empty() {
                archive.asteroid_samples = res.asteroid_samples;
            }
            if !res.star_samples.is_empty() {
                archive.star_samples = res.star_samples;
            }
            if let Some(curves) = res.curves {
                archive.curves = Some(curves);
            }
            if let Some(hash) = res.spectral {
                archive.spectral.retain(|h| h.name != hash.name);
                archive.spectral.push(hash);
            }
            let src = &archive.sources[res.source_idx];
            let _ = machine_tx.send((src.frame.clone(), res.channels.clone()));
            for (channel, sensor) in &res.channels {
                let track_origin = matches!(channel.position, Position::Source)
                    || matches!(channel.position, Position::StateVector { track: true, .. });
                if let Some(sample) = anchor(
                    channel,
                    sensor,
                    src.ttl as f64,
                    Some(res.source_idx as u32),
                    Some(&src.frame),
                    if track_origin {
                        archive.origins.get_mut(&(res.source_idx as u32))
                    } else {
                        None
                    },
                    &archive.body_ephemerides,
                ) {
                    fetched_samples.push(sample);
                } else {
                    let eph_missing = match &channel.position {
                        Position::Surface { body_name, .. }
                        | Position::SurfaceFlow { body_name, .. }
                        | Position::Barycenter { body_name, .. } => archive
                            .body_ephemerides
                            .get(body_name.as_str())
                            .and_then(|e| e.props.as_ref())
                            .is_none(),
                        Position::Source => match &src.frame {
                            Frame::Surface { body_name, .. }
                            | Frame::Barycenter { body_name, .. } => archive
                                .body_ephemerides
                                .get(body_name.as_str())
                                .and_then(|e| e.props.as_ref())
                                .is_none(),
                            Frame::Manifest => false,
                        },
                        Position::StateVector { .. } => false,
                    };
                    if eph_missing {
                        dropped_channels.push((
                            channel.clone(),
                            sensor.clone(),
                            res.source_idx as u32,
                        ));
                    }
                }
            }
        }
        archive.pending_channels.extend(dropped_channels);
        while let Ok(samples) = sample_rx.try_recv() {
            fetched_samples.extend(samples);
        }
        while let Ok(samples) = sensor_rx.try_recv() {
            if !consent.load(Ordering::SeqCst) {
                continue;
            }
            let Some(declared_body) = archive.declared_body.clone() else {
                continue;
            };
            for (name, value, tau) in samples {
                let Some(bs) = sensor_config(&name) else {
                    continue;
                };
                let effective_tau = if tau > 0.0 {
                    tau
                } else {
                    continue;
                };
                if !value.is_finite() {
                    continue;
                }
                let fc = FieldConfig {
                    key: bs.key.clone(),
                    name: bs.key.clone(),
                    kernel: bs.kernel,
                    force: bs.force,
                    tau: effective_tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                };
                let channel = Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch: now,
                    position: Position::Surface {
                        body_name: declared_body.body_name.clone(),
                        lat: declared_body.lat,
                        lon: declared_body.lon,
                        alt: match declared_body.alt {
                            Some(a) => a,
                            None => {
                                eprintln!(
                                    "sensor alt undeclared — samples refused (declare #body=<body>,<lat>,<lon>,<alt>)"
                                );
                                continue;
                            }
                        },
                    },
                    name: fc.name.clone(),
                    value,
                };
                if let Some(sample) = anchor(
                    &channel,
                    &fc,
                    bs.ttl,
                    None,
                    None,
                    None,
                    &archive.body_ephemerides,
                ) {
                    fetched_samples.push(sample);
                }
            }
        }
        let median_fetch =
            median_fetch_duration(&archive.fetch_durations, archive.fetch_duration_len);
        for i in 0..archive.sources.len() {
            let origin = i as u32;
            if !origin_stale(
                &archive.origins,
                origin,
                archive.sources[i].ttl,
                now,
                archive.jump_epoch,
            ) {
                continue;
            }
            if archive.sources[i].format == "kernel_text" {
                continue;
            }
            if archive.origins.values().filter(|o| o.in_flight).count() >= FETCH_BUDGET {
                break;
            }
            if archive.sources[i].format == "ephemeris_binary"
                || archive.sources[i].format == "orbit_bin"
            {
                let src_idx = i;
                let src_clone = archive.sources[i].clone();
                let tmp_path = match &src_clone.body {
                    Some(b) => format!("/tmp/omegaflow_eph_{}.bin", b),
                    None => continue,
                };
                if cache_fresh(&tmp_path, src_clone.ttl) {
                    begin_fetch(&mut archive.origins, i as u32, now);
                    let ftx = fetch_tx.clone();
                    let lsk_c = lsk.clone();
                    let now_c = now;
                    let tmp_path_c = tmp_path.clone();
                    thread::spawn(move || {
                        if let ExtractResult::WithEphemeris(_, eph) =
                            extract(&src_clone, &tmp_path_c, now_c, &lsk_c)
                        {
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: src_clone.body.clone().map(|b| (b, eph)),
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                        } else {
                            eprintln!(
                                "ephemeris {}: extract void — cache dropped, refetch next bootstrap",
                                src_clone.body.as_deref().unwrap_or("?")
                            );
                            let _ = std::fs::remove_file(&tmp_path_c);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                        }
                    });
                }
                continue;
            }
            if archive.sources[i].format == "catalog_dastcom" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("catalog").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    let mut fetched = true;
                    if !cache_fresh(&tmp_path, src_ttl) {
                        fetched = match fetch_raw_bytes(&url, src_ttl) {
                            Some(bytes) => std::fs::write(&tmp_path, &bytes).is_ok(),
                            None => false,
                        };
                    }
                    if !fetched {
                        eprintln!(
                            "catalog {}: fetch void — the catalog stays absent, retry in ttl/Φ·2ⁿ",
                            url
                        );
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: false,
                        });
                        return;
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("catalog {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let samples = build_asteroid_samples(&bytes, src_ttl);
                    eprintln!("\r\x1b[Kcatalog_dastcom: {} samples", samples.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: samples,
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "netcdf" {
                let src_clone = archive.sources[i].clone();
                let pos = match &src_clone.frame {
                    Frame::Surface {
                        lat,
                        lon,
                        alt,
                        body_name,
                    } => body_fixed_to_icrs(
                        body_name,
                        *lat,
                        *lon,
                        *alt,
                        now,
                        &archive.body_ephemerides,
                    )
                    .map(|p| (p[0], p[1], p[2])),
                    Frame::Barycenter { body_name, scale } => {
                        body_barycenter_position(body_name, now, &archive.body_ephemerides)
                            .map(|p| (p[0] * scale, p[1] * scale, p[2] * scale))
                    }
                    Frame::Manifest => None,
                };
                let url = match pos {
                    Some((x, y, z)) => render_source_url(
                        &src_clone,
                        x,
                        y,
                        z,
                        now,
                        0.0,
                        &archive.body_ephemerides,
                        &env,
                        &lsk,
                    ),
                    None => render_source_url(
                        &src_clone,
                        0.0,
                        0.0,
                        0.0,
                        now,
                        0.0,
                        &archive.body_ephemerides,
                        &env,
                        &lsk,
                    ),
                };
                let Some(url) = url else {
                    continue;
                };
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                let lsk_c = lsk.clone();
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("netcdf").to_string();
                    let tmp_path = format!("/tmp/omegaflow_netcdf_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("netcdf {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("netcdf {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("netcdf {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let channels = build_netcdf_channels(&src_clone, &bytes, &lsk_c);
                    eprintln!("\r\x1b[Knetcdf {}: {} samples", name, channels.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "finals" || archive.sources[i].format == "ionex" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                let lsk_c = lsk.clone();
                let now_c = now;
                let is_ionex = archive.sources[i].format == "ionex";
                thread::spawn(move || {
                    let bytes = match fetch_raw_bytes(&url, src_ttl) {
                        Some(b) => b,
                        None => {
                            eprintln!("finals {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: false,
                            });
                            return;
                        }
                    };
                    let text = String::from_utf8_lossy(&bytes).into_owned();
                    let channels = if is_ionex {
                        build_ionex_channels(&src_clone, &text, now_c, &lsk_c)
                    } else {
                        build_finals_channels(&src_clone, &text, &lsk_c)
                    };
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "alerce" {
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let cap = src_clone.fanout_cap.max(1) as usize;
                let delay = src_clone.fanout_delay;
                thread::spawn(move || {
                    let channels = build_alerce_channels(&src_clone, cap, delay);
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "catalog_tycho" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let src_ttl = src_clone.ttl;
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("stars").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("catalog_tycho {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("catalog_tycho {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("catalog_tycho {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let star_samples = build_star_samples(&bytes);
                    eprintln!("\r\x1b[Kcatalog_tycho: {} stars", star_samples.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples,
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "spectral" {
                let src = archive.sources[i].clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let url = src.url.clone();
                    let name = url.rsplit('/').next().unwrap_or("spectra").to_string();
                    let tmp_path = format!("/tmp/omegaflow_spectral_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("spectral {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("spectral {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("spectral {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let (epoch, bins) = match crate::spectral::parse_spectral_bin(&bytes) {
                        Some(x) => x,
                        None => {
                            eprintln!(
                                "spectral {}: bin reads void — {} B carry no spectra.bin contract",
                                url,
                                bytes.len()
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let motion = match &src.frame {
                        Frame::Surface {
                            body_name,
                            lat,
                            lon,
                            alt,
                        } => Motion::Surface {
                            body_name: body_name.clone(),
                            lat: *lat,
                            lon: *lon,
                            alt: *alt,
                        },
                        Frame::Barycenter { body_name, scale } => Motion::Barycenter {
                            body_name: body_name.clone(),
                            scale: *scale,
                        },
                        Frame::Manifest => {
                            eprintln!(
                                "spectral {}: frameless — the block declares no position",
                                url
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let field = match src.extracts.first() {
                        Some(Extract::Field(fc)) => fc.clone(),
                        _ => {
                            eprintln!(
                                "spectral {}: field undeclared — the block carries no field line",
                                url
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let hash = SpectralHash {
                        name: field.name.clone(),
                        motion,
                        epoch,
                        ttl: src_ttl as f64,
                        tau: field.tau,
                        kernel_id: field.kernel as f64,
                        force_type: field.force as f64,
                        absorption: field.absorption,
                        advection: field.advection,
                        bins,
                    };
                    eprintln!(
                        "\r\x1b[Kspectral {}: {} bins, epoch_tdb {}",
                        field.name,
                        hash.bins.len(),
                        hash.epoch
                    );
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: Some(hash),
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "jwst_spectra" {
                let src = archive.sources[i].clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let url = src.url.clone();
                    let name = url.rsplit('/').next().unwrap_or("jwst_spectra").to_string();
                    let tmp_path = format!("/tmp/omegaflow_jwst_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("jwst_spectra {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("jwst_spectra {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("jwst_spectra {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let specs = match crate::jwst::parse_jwst_bin(&bytes) {
                        Some(s) => s,
                        None => {
                            eprintln!(
                                "jwst_spectra {}: bin reads void — {} B carry no JWS1 contract",
                                url,
                                bytes.len()
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let field = match src.extracts.first() {
                        Some(Extract::Field(fc)) => fc.clone(),
                        _ => {
                            eprintln!(
                                "jwst_spectra {}: field undeclared — the block carries no field line",
                                url
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    if specs.is_empty() {
                        eprintln!("jwst_spectra {}: bin carries no records", url);
                        let _ = ftx.send(empty(true));
                        return;
                    }
                    let total = specs.len();
                    let mut named_skips = 0usize;
                    for spec in specs {
                        if spec.plx_mas <= 0.0 {
                            named_skips += 1;
                            continue;
                        }
                        let rec = Arc::new(StarRec {
                            ra_deg: spec.ra_deg,
                            dec_deg: spec.dec_deg,
                            pm_ra_masyr: 0.0,
                            pm_de_masyr: 0.0,
                            plx_mas: spec.plx_mas,
                            flux: 0.0,
                            mag: 0.0,
                            tau: 0.0,
                            color_index: 0.0,
                            rv_m_s: 0.0,
                        });
                        let hash_name = format!("jwst_spectra.flux.{}.{}", spec.host, spec.obs_id);
                        let hash = SpectralHash {
                            name: hash_name,
                            motion: Motion::Spherical { rec },
                            epoch: spec.epoch_tdb,
                            ttl: src_ttl as f64,
                            tau: field.tau,
                            kernel_id: field.kernel as f64,
                            force_type: field.force as f64,
                            absorption: field.absorption,
                            advection: field.advection,
                            bins: spec.bins,
                        };
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: Some(hash),
                            fetch_ok: true,
                        });
                    }
                    eprintln!(
                        "\r\x1b[Kjwst_spectra {}: {} records sent, {} without parallax named",
                        url,
                        total.saturating_sub(named_skips),
                        named_skips
                    );
                    if named_skips == total {
                        let _ = ftx.send(empty(true));
                    }
                });
                continue;
            }
            if archive.sources[i].format == "lightcurve" {
                let url = archive.sources[i].url.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = archive.sources[i].ttl;
                thread::spawn(move || {
                    let name = url.rsplit('/').next().unwrap_or("curves").to_string();
                    let tmp_path = format!("/tmp/omegaflow_catalog_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("lightcurve {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("lightcurve {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("lightcurve {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let curves = build_curve_set(&bytes);
                    eprintln!("\r\x1b[Klightcurve: {} stars", curves.stars.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: Some(Arc::new(curves)),
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if matches!(
                archive.sources[i].format.as_str(),
                "rpw_efield" | "goes_xrs" | "omni2_serie" | "mitdb" | "circor" | "ltmm"
            ) {
                let url = archive.sources[i].url.clone();
                let src = archive.sources[i].clone();
                let fmt = archive.sources[i].format.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let name = url.rsplit('/').next().unwrap_or("series").to_string();
                    let tmp_path = format!("/tmp/omegaflow_series_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("{} {}: fetch void — retry in ttl/Φ·2ⁿ", fmt, url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("{} {}: write void — retry in ttl/Φ", fmt, url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("{} {}: read void — retry in ttl/Φ", fmt, url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let records = match series_parse_bin(&fmt, &bytes) {
                        Some(r) => r,
                        None => {
                            eprintln!(
                                "{} {}: bin reads void — {} B carry no {} contract",
                                fmt,
                                url,
                                bytes.len(),
                                fmt
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let fields: Vec<FieldConfig> = src
                        .extracts
                        .iter()
                        .filter_map(|e| match e {
                            Extract::Field(fc) => Some(fc.clone()),
                            _ => None,
                        })
                        .collect();
                    if fields.is_empty() {
                        eprintln!(
                            "{} {}: field undeclared — the block carries no field line",
                            fmt, url
                        );
                        let _ = ftx.send(empty(true));
                        return;
                    }
                    let mut channels = Vec::with_capacity(records.len());
                    for (t, val, comp) in records {
                        let Some(name) = series_component_name(&fmt, comp) else {
                            continue;
                        };
                        let Some(fc) = fields.iter().find(|fc| fc.name == name) else {
                            continue;
                        };
                        channels.push((
                            Channel {
                                z: 0.0,
                                freq: 0.0,
                                bin_width: 0.0,
                                epoch: t,
                                position: Position::Source,
                                name: fc.name.clone(),
                                value: val,
                            },
                            fc.clone(),
                        ));
                    }
                    eprintln!("\r\x1b[K{} {}: {} oscillators", fmt, url, channels.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "wind_waves" {
                let url = archive.sources[i].url.clone();
                let src = archive.sources[i].clone();
                let fmt = archive.sources[i].format.clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let name = url.rsplit('/').next().unwrap_or("wind_waves").to_string();
                    let tmp_path = format!("/tmp/omegaflow_series_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("{} {}: fetch void — retry in ttl/Φ·2ⁿ", fmt, url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("{} {}: write void — retry in ttl/Φ", fmt, url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("{} {}: read void — retry in ttl/Φ", fmt, url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let records = match crate::wind::parse_bin(&bytes) {
                        Some(r) => r,
                        None => {
                            eprintln!(
                                "{} {}: bin reads void — {} B carry no WAV1 contract",
                                fmt,
                                url,
                                bytes.len()
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let fields: Vec<FieldConfig> = src
                        .extracts
                        .iter()
                        .filter_map(|e| match e {
                            Extract::Field(fc) => Some(fc.clone()),
                            _ => None,
                        })
                        .collect();
                    if fields.is_empty() {
                        eprintln!(
                            "{} {}: field undeclared — the block carries no field line",
                            fmt, url
                        );
                        let _ = ftx.send(empty(true));
                        return;
                    }
                    let mut channels = Vec::with_capacity(records.len());
                    for (t, freq, binw, val, recv) in records {
                        let name = format!(
                            "wind_waves_{}",
                            crate::wind::receiver_name(recv).to_lowercase()
                        );
                        let Some(fc) = fields.iter().find(|fc| fc.name == name) else {
                            continue;
                        };
                        channels.push((
                            Channel {
                                z: 0.0,
                                freq,
                                bin_width: binw,
                                epoch: t,
                                position: Position::Source,
                                name: fc.name.clone(),
                                value: val,
                            },
                            fc.clone(),
                        ));
                    }
                    eprintln!("\r\x1b[K{} {}: {} oscillators", fmt, url, channels.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "gong_modes" {
                let url = archive.sources[i].url.clone();
                let src = archive.sources[i].clone();
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_idx = i;
                let src_ttl = src.ttl;
                thread::spawn(move || {
                    let empty = |fetch_ok: bool| FetchResult {
                        source_idx: src_idx,
                        channels: Vec::new(),
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok,
                    };
                    let name = url.rsplit('/').next().unwrap_or("gong").to_string();
                    let tmp_path = format!("/tmp/omegaflow_gong_{}", name);
                    if !cache_fresh(&tmp_path, src_ttl) {
                        let bytes = match fetch_raw_bytes(&url, src_ttl) {
                            Some(b) => b,
                            None => {
                                eprintln!("gong {}: fetch void — retry in ttl/Φ·2ⁿ", url);
                                let _ = ftx.send(empty(false));
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("gong {}: write void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    }
                    let bytes = match std::fs::read(&tmp_path) {
                        Ok(b) => b,
                        Err(_) => {
                            eprintln!("gong {}: read void — retry in ttl/Φ", url);
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let modes = match crate::gong::parse_bin(&bytes) {
                        Some(m) => m,
                        None => {
                            eprintln!(
                                "gong {}: bin reads void — {} B carry no gong_modes.bin contract",
                                url,
                                bytes.len()
                            );
                            let _ = ftx.send(empty(true));
                            return;
                        }
                    };
                    let fields: Vec<FieldConfig> = src
                        .extracts
                        .iter()
                        .filter_map(|e| match e {
                            Extract::Field(fc) => Some(fc.clone()),
                            _ => None,
                        })
                        .collect();
                    let Some(fc) = fields.first().cloned() else {
                        eprintln!(
                            "gong {}: field undeclared — the block carries no field line",
                            url
                        );
                        let _ = ftx.send(empty(true));
                        return;
                    };
                    let mut channels = Vec::with_capacity(modes.len());
                    for (_, _, t, rms) in modes {
                        channels.push((
                            Channel {
                                z: 0.0,
                                freq: 0.0,
                                bin_width: 0.0,
                                epoch: t,
                                position: Position::Source,
                                name: fc.name.clone(),
                                value: rms,
                            },
                            fc.clone(),
                        ));
                    }
                    eprintln!("\r\x1b[Kgong {}: {} modes", url, channels.len());
                    let _ = ftx.send(FetchResult {
                        source_idx: src_idx,
                        channels,
                        eph_update: None,
                        asteroid_samples: Vec::new(),
                        star_samples: Vec::new(),
                        curves: None,
                        spectral: None,
                        fetch_ok: true,
                    });
                });
                continue;
            }
            if archive.sources[i].format == "csv_zip" {
                begin_fetch(&mut archive.origins, i as u32, now);
                let ftx = fetch_tx.clone();
                let src_clone = archive.sources[i].clone();
                let src_idx = i;
                let eph_arc = archive.body_ephemerides.clone();
                let e = env.clone();
                let lsk_c = lsk.clone();
                thread::spawn(move || {
                    let url = match render_source_url(
                        &src_clone, 0.0, 0.0, 0.0, now, 0.0, &eph_arc, &e, &lsk_c,
                    ) {
                        Some(u) => u,
                        None => {
                            eprintln!("csv_zip {}: url render void — retry in ttl/Φ", src_idx);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    };
                    let tmp_path = format!("/tmp/omegaflow_csv_{}.zip", src_idx);
                    if !cache_fresh(&tmp_path, src_clone.ttl) {
                        let headers = render_headers(&src_clone.headers, &e);
                        let bytes = match fetch_raw_bytes_post(&url, None, &headers, src_clone.ttl)
                        {
                            Some(b) => b,
                            None => {
                                eprintln!("csv_zip {}: fetch void — retry in ttl/Φ·2ⁿ", src_idx);
                                let _ = ftx.send(FetchResult {
                                    source_idx: src_idx,
                                    channels: Vec::new(),
                                    eph_update: None,
                                    asteroid_samples: Vec::new(),
                                    star_samples: Vec::new(),
                                    curves: None,
                                    spectral: None,
                                    fetch_ok: false,
                                });
                                return;
                            }
                        };
                        if std::fs::write(&tmp_path, &bytes).is_err() {
                            eprintln!("csv_zip {}: write void — retry in ttl/Φ", src_idx);
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: None,
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            return;
                        }
                    }
                    if let ExtractResult::Measurements(channels) =
                        extract(&src_clone, &tmp_path, now, &lsk_c)
                    {
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels,
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    } else {
                        eprintln!("csv_zip {}: extract void — retry in ttl/Φ", src_idx);
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    }
                });
                continue;
            }
            let mut fields: Vec<FieldConfig> = Vec::new();
            for ext in &archive.sources[i].extracts {
                fields.extend(extract_fields(ext));
            }
            let Some(r) = dispatch_reach(&fields, archive.sources[i].ttl as f64) else {
                if fields.is_empty() {
                    eprintln!(
                        "source {}: carries no field lines — refused, retry in ttl/Φ",
                        i
                    );
                    if let Ok(mut ledger) = refusal_ledger.lock() {
                        ledger.register(&archive.sources[i].url, "gate-no-field-lines");
                    }
                } else {
                    eprintln!(
                        "source {}: no field carries a propagation law — refused, retry in ttl/Φ",
                        i
                    );
                    if let Ok(mut ledger) = refusal_ledger.lock() {
                        ledger.register(&archive.sources[i].url, "gate-no-propagation");
                    }
                }
                continue;
            };
            let pos = match &archive.sources[i].frame {
                Frame::Surface {
                    lat,
                    lon,
                    alt,
                    body_name,
                } => {
                    if let Some(p) = body_fixed_to_icrs(
                        body_name,
                        *lat,
                        *lon,
                        *alt,
                        now,
                        &archive.body_ephemerides,
                    ) {
                        (p[0], p[1], p[2])
                    } else {
                        continue;
                    }
                }
                Frame::Barycenter { body_name, scale } => {
                    if let Some(bp) =
                        body_barycenter_position(body_name, now, &archive.body_ephemerides)
                    {
                        (bp[0] * scale, bp[1] * scale, bp[2] * scale)
                    } else {
                        continue;
                    }
                }
                Frame::Manifest => continue,
            };
            let presences: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> =
                archive.presence.values().cloned().collect();
            let anchor_body = frame_body_name(&archive.sources[i].frame);
            let body_props = archive
                .body_ephemerides
                .get(anchor_body.as_str())
                .and_then(|e| e.props.as_ref());
            let body_radius = match body_props {
                Some(p) => p.radius_m,
                None => 0.0,
            };
            let v_anchor =
                anchor_velocity(&archive.sources[i].frame, now, &archive.body_ephemerides);
            if !presence_gate(&presences, pos, r, body_radius, v_anchor, median_fetch) {
                continue;
            }
            begin_fetch(&mut archive.origins, i as u32, now);
            let ftx = fetch_tx.clone();
            let src_clone = archive.sources[i].clone();
            let eph_arc = archive.body_ephemerides.clone();
            let e = env.clone();
            let src_idx = i;
            let lsk_c = lsk.clone();
            let rl = refusal_ledger.clone();
            let presence_center = presences.first().map(|p| (p.2, p.3, p.4));
            thread::spawn(move || {
                if src_clone.fanout_cap > 0 {
                    if let Some(ref su) = src_clone.stations_url {
                        let channels = fanout_fetch(
                            &src_clone,
                            su,
                            pos.0,
                            pos.1,
                            pos.2,
                            presence_center,
                            now,
                            r,
                            &eph_arc,
                            &e,
                            &lsk_c,
                        );
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels,
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    } else {
                        eprintln!("fanout {}: stations_url absent — retry in ttl/Φ", src_idx);
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                    }
                    return;
                }
                let url = match render_source_url(
                    &src_clone, pos.0, pos.1, pos.2, now, r, &eph_arc, &e, &lsk_c,
                ) {
                    Some(u) => u,
                    None => {
                        eprintln!("source {}: url render void — retry in ttl/Φ", src_idx);
                        if let Ok(mut ledger) = rl.lock() {
                            ledger.register(&src_clone.url, "url-render-void");
                        }
                        let _ = ftx.send(FetchResult {
                            source_idx: src_idx,
                            channels: Vec::new(),
                            eph_update: None,
                            asteroid_samples: Vec::new(),
                            star_samples: Vec::new(),
                            curves: None,
                            spectral: None,
                            fetch_ok: true,
                        });
                        return;
                    }
                };
                let body =
                    render_source_body(&src_clone, pos.0, pos.1, pos.2, now, r, &eph_arc, &lsk_c);
                let headers = render_headers(&src_clone.headers, &e);
                let raw = fetch_one(&url, body.as_deref(), &headers, src_clone.ttl, Some(now));
                let fetch_ok = raw.is_some();
                let channels = match raw {
                    Some(ref r) => match extract(&src_clone, r, now, &lsk_c) {
                        ExtractResult::Measurements(v) => {
                            if v.is_empty() {
                                eprintln!("source {}: extract returned no measurements", src_idx);
                                if let Ok(mut ledger) = rl.lock() {
                                    ledger.register(&src_clone.url, "extract-void");
                                }
                            }
                            v
                        }
                        ExtractResult::WithEphemeris(v, eph) => {
                            let _ = ftx.send(FetchResult {
                                source_idx: src_idx,
                                channels: Vec::new(),
                                eph_update: src_clone.body.clone().map(|b| (b, eph)),
                                asteroid_samples: Vec::new(),
                                star_samples: Vec::new(),
                                curves: None,
                                spectral: None,
                                fetch_ok: true,
                            });
                            v
                        }
                    },
                    None => {
                        eprintln!("source {}: fetch void — retry in ttl/Φ·2ⁿ", src_idx);
                        if let Ok(mut ledger) = rl.lock() {
                            ledger.register(&src_clone.url, "fetch-void");
                        }
                        Vec::new()
                    }
                };
                let _ = ftx.send(FetchResult {
                    source_idx: src_idx,
                    channels,
                    eph_update: None,
                    asteroid_samples: Vec::new(),
                    star_samples: Vec::new(),

                    curves: None,
                    spectral: None,
                    fetch_ok,
                });
            });
        }
        {
            let old = archive.field.clone();
            let retained_estimate: usize = old.cache.cells.values().map(|v| v.len()).sum::<usize>()
                + old.cache.unbounded.len();
            let mut all: Vec<Sample> = Vec::with_capacity(
                fetched_samples.len()
                    + retained_estimate
                    + archive.body_ephemerides.len() * 2
                    + archive.asteroid_samples.len(),
            );
            all.append(&mut fetched_samples);
            for v in old
                .cache
                .cells
                .values()
                .chain(std::iter::once(&old.cache.unbounded))
            {
                for s in v {
                    if matches!(s.source, SampleSource::Ephemeris) {
                        continue;
                    }
                    if (now - s.epoch).abs() <= s.ttl * 64.0 {
                        all.push(s.clone());
                    }
                }
            }
            for (name, eph) in archive.body_ephemerides.iter() {
                if let Some(props) = &eph.props {
                    if props.radius_m > 0.0 {
                        let Some(body_ttl) = archive
                            .sources
                            .iter()
                            .find(|s| s.body.as_deref() == Some(name.as_str()))
                            .map(|s| s.ttl as f64)
                        else {
                            continue;
                        };
                        let frame = Frame::Barycenter {
                            body_name: name.clone(),
                            scale: 1.0,
                        };
                        for (channel, sensor) in body_channels(name, props, now) {
                            if let Some(mut sample) = anchor(
                                &channel,
                                &sensor,
                                body_ttl,
                                Some(archive.sources.len() as u32),
                                Some(&frame),
                                None,
                                &archive.body_ephemerides,
                            ) {
                                sample.source = SampleSource::Ephemeris;
                                all.push(sample);
                            }
                        }
                    }
                }
            }
            all.extend(archive.asteroid_samples.iter().cloned());
            all.extend(archive.star_samples.iter().cloned());
            if all.len() > MAX_SAMPLES {
                let ring = epoch_zero_ring(&mut all, MAX_SAMPLES);
                eprintln!(
                    "epoch0 ring: total in {}, epoch0 in {}, epoch0 kept {}, epoch0 dropped {} (cap {})",
                    ring.total_in,
                    ring.epoch0_in,
                    ring.epoch0_kept,
                    ring.epoch0_dropped,
                    MAX_SAMPLES
                );
            }
            archive.field = Arc::new(build_buffer(
                all,
                cadence,
                archive.body_ephemerides.clone(),
                archive.curves.clone(),
                archive.spectral.clone(),
            ));
        }
        let f = archive.field.clone();
        for r in &mut radiators {
            r.accept(f.clone());
        }
        let elapsed = match lsk.system_now_tdb() {
            Some(t) => t - wall_entered,
            None => cadence,
        };
        if elapsed < cadence {
            thread::sleep(std::time::Duration::from_secs_f64(cadence - elapsed));
        }
    }
}
