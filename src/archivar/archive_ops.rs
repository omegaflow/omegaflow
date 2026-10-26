use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatSource {
    Serial,
    Ble,
    Fit,
    None,
}

pub fn select_beat_source(
    serial_present: bool,
    ble_mac: Option<&str>,
    fit_dir: Option<&str>,
) -> BeatSource {
    if serial_present {
        BeatSource::Serial
    } else if ble_mac.is_some_and(|m| !m.trim().is_empty()) {
        BeatSource::Ble
    } else if fit_dir.is_some_and(|d| !d.trim().is_empty()) {
        BeatSource::Fit
    } else {
        BeatSource::None
    }
}

pub fn jwst_spectrum_motion(rec: Arc<StarRec>, anchor: Option<&Motion>) -> Option<Motion> {
    if rec.plx_mas.is_finite() && rec.plx_mas > 0.0 {
        Some(Motion::Spherical { rec })
    } else {
        anchor.cloned()
    }
}

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
            .chain(field.cache.star_cells.values())
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

pub fn enclosure_presences(
    presence: &HashMap<String, PresenceSample>,
    slot: &std::sync::Arc<std::sync::RwLock<crate::mathematikerin::PresenceState>>,
    now: f64,
) -> Vec<PresenceSample> {
    if !presence.is_empty() {
        return presence.values().cloned().collect();
    }
    let state = match slot.read() {
        Ok(guard) => *guard,
        Err(_) => return Vec::new(),
    };
    vec![(
        now,
        state.p[0],
        state.p[1],
        state.p[2],
        state.range,
        state.v[0],
        state.v[1],
        state.v[2],
        state.t_thrust,
        state.grid_step,
    )]
}

pub fn source_name(url: &str) -> &str {
    match url.rsplit('/').next() {
        Some(name) if !name.is_empty() => name,
        _ => url,
    }
}

pub struct PresenceGateCtx<'a> {
    pub slot: &'a std::sync::Arc<std::sync::RwLock<crate::mathematikerin::PresenceState>>,
    pub body_ephemerides: &'a HashMap<String, BodyEphemeris>,
    pub now: f64,
    pub median_fetch: Option<f64>,
    pub refusal_ledger: &'a std::sync::Mutex<RefusalLedger>,
}

pub fn raw_presence_gate(
    i: usize,
    sources: &[SourceConfig],
    presence: &HashMap<String, PresenceSample>,
    ctx: PresenceGateCtx<'_>,
) -> Option<(f64, f64, f64)> {
    let presences: Vec<PresenceSample> = enclosure_presences(presence, ctx.slot, ctx.now);
    let mut fields: Vec<FieldConfig> = Vec::new();
    for ext in &sources[i].extracts {
        fields.extend(extract_fields(ext));
    }
    let Some(r) = dispatch_reach(&fields, sources[i].ttl as f64) else {
        if fields.is_empty() {
            eprintln!(
                "source {} [{}]: carries no field lines — refused, retry in ttl/Φ",
                i, sources[i].format
            );
            if let Ok(mut ledger) = ctx.refusal_ledger.lock() {
                ledger.register(&sources[i].url, "gate-no-field-lines");
            }
        } else {
            eprintln!(
                "source {} [{}]: no field carries a propagation law — refused, retry in ttl/Φ",
                i, sources[i].format
            );
            if let Ok(mut ledger) = ctx.refusal_ledger.lock() {
                ledger.register(&sources[i].url, "gate-no-propagation");
            }
        }
        return None;
    };
    let pos = match &sources[i].frame() {
        Frame::Surface {
            lat,
            lon,
            alt,
            body_name,
        } => {
            let Some(alt) = *alt else {
                return None;
            };
            let p = body_fixed_to_icrs(body_name, *lat, *lon, alt, ctx.now, ctx.body_ephemerides)?;
            (p[0], p[1], p[2])
        }
        Frame::Barycenter { body_name, scale } => {
            let bp = body_barycenter_position(body_name, ctx.now, ctx.body_ephemerides)?;
            (bp[0] * scale, bp[1] * scale, bp[2] * scale)
        }
        Frame::Manifest => return None,
    };
    let anchor_body = frame_body_name(&sources[i].frame());
    let body_props = ctx
        .body_ephemerides
        .get(anchor_body.as_str())
        .and_then(|e| e.props.as_ref());
    let body_radius = body_props?.radius_m;
    let v_anchor = anchor_velocity(&sources[i].frame(), ctx.now, ctx.body_ephemerides);
    if presence_gate(&presences, pos, r, body_radius, v_anchor, ctx.median_fetch) {
        Some(pos)
    } else {
        None
    }
}

pub fn parse_sink_names(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.split('\t').nth(1))
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .collect()
}
pub fn jump_residual_breached(
    p_new: [f64; 3],
    p_old: [f64; 3],
    v_old: [f64; 3],
    v_new: [f64; 3],
    dt: f64,
) -> bool {
    if !dt.is_finite() || dt <= 0.0 {
        return false;
    }
    let a = [
        p_old[0] + v_old[0] * dt,
        p_old[1] + v_old[1] * dt,
        p_old[2] + v_old[2] * dt,
    ];
    let b = [
        p_old[0] + v_new[0] * dt,
        p_old[1] + v_new[1] * dt,
        p_old[2] + v_new[2] * dt,
    ];
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    let residual = if dd > 0.0 {
        let t = (((p_new[0] - a[0]) * d[0] + (p_new[1] - a[1]) * d[1] + (p_new[2] - a[2]) * d[2])
            / dd)
            .clamp(0.0, 1.0);
        let q = [
            p_new[0] - (a[0] + t * d[0]),
            p_new[1] - (a[1] + t * d[1]),
            p_new[2] - (a[2] + t * d[2]),
        ];
        (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt()
    } else {
        let q = [p_new[0] - a[0], p_new[1] - a[1], p_new[2] - a[2]];
        (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt()
    };
    residual >= Φ * crate::mathematikerin::JUMP_GRID
}

pub struct Archive {
    pub sources: Vec<SourceConfig>,
    pub body_ephemerides: Arc<HashMap<String, BodyEphemeris>>,
    pub field: Arc<Buffer>,
    pub presence: HashMap<String, PresenceSample>,
    pub jump_epoch: Option<f64>,
    pub prev_presence_epoch: Option<f64>,
    pub declared_body: Option<DeclaredBody>,
    pub receiver_aperture: ReceiverAperture,
    pub origins: HashMap<Origin, OriginState>,
    pub pck_bodies: HashMap<i32, PckBody>,
    pub time: Arc<Mutex<Option<LeapSeconds>>>,
    pub asteroid_samples: Vec<Arc<Sample>>,
    pub star_samples: Vec<Arc<Sample>>,
    pub stations: Arc<Mutex<Vec<StationThread>>>,
    pub gestalt_surface_threads: Arc<Mutex<Vec<Motion>>>,
    pub curves: Option<Arc<CurveSet>>,
    pub spectral: Vec<SpectralHash>,
    pub volumes: Arc<Mutex<Vec<(String, crate::archivar::volume::Volume)>>>,
    pub bayestar: Arc<Mutex<Option<Arc<crate::archivar::bayestar::BayestarMap>>>>,
    pub pending_channels: Vec<(Channel, FieldConfig, u32)>,
    pub fetch_durations: [f64; FETCH_DURATION_RING],
    pub fetch_duration_len: usize,
    pub fetch_duration_idx: usize,
}

pub fn temporal_ring_shared(
    static_catalog: &[Arc<Sample>],
    temporal: &mut Vec<Arc<Sample>>,
    cap: usize,
) -> TemporalRing {
    let temporal_in = temporal.len();
    if temporal.len() > cap {
        temporal.sort_by(|a, b| b.epoch.total_cmp(&a.epoch));
        temporal.truncate(cap);
    }
    TemporalRing {
        static_in: static_catalog.len(),
        temporal_in,
        temporal_kept: temporal.len(),
        temporal_dropped: temporal_in.saturating_sub(temporal.len()),
    }
}

pub fn anchor_uses(sources: &[SourceConfig]) -> std::collections::HashMap<String, usize> {
    let mut uses = std::collections::HashMap::new();
    for s in sources {
        if matches!(
            s.format.as_str(),
            "ephemeris_binary"
                | "kernel_text"
                | "orbit_bin"
                | "ephemeris_inpop"
                | "ephemeris_epm"
                | "ephemeris_de440"
                | "ephemeris_de442"
                | "ephemeris_noe4"
        ) {
            continue;
        }
        match &s.frame() {
            Frame::Surface { body_name, .. } | Frame::Barycenter { body_name, .. } => {
                *uses.entry(body_name.clone()).or_insert(0) += 1;
            }
            Frame::Manifest => {}
        }
    }
    uses
}

pub fn ephemeris_edition_body(url: &str, format: &str) -> Option<String> {
    let edition = format.strip_prefix("ephemeris_")?;
    let name = source_name(url);
    let body = name
        .strip_prefix(&format!("ephemeris_{edition}_"))?
        .strip_suffix(".bin")?;
    if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    }
}

