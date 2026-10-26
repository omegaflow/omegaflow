use crate::mathematikerin::channel::{CHANNEL_CAP, TriState};
use crate::pck::PckBody;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub const JUMP_GRID: f64 = 268435456.0;
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

pub fn enclosure_presences(
    presence: &HashMap<String, PresenceSample>,
    slot: &std::sync::Arc<std::sync::RwLock<PresenceState>>,
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
    pub slot: &'a std::sync::Arc<std::sync::RwLock<PresenceState>>,
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
            let alt = (*alt)?;
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
    residual >= Φ * JUMP_GRID
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

pub fn load_ephemeris_cache(
    fetch_tx: &mpsc::Sender<FetchResult>,
    source_idx: usize,
    src: &SourceConfig,
    tmp_path: &str,
    now: f64,
    lsk: &LeapSeconds,
    presences: &[PresenceSample],
) {
    if let ExtractResult::WithEphemeris(_, eph) = extract(src, tmp_path, now, lsk) {
        let Some(body) = src.body.as_deref() else {
            return;
        };
        let mut one: HashMap<String, BodyEphemeris> = HashMap::new();
        one.insert(body.to_string(), *eph);
        let admitted = {
            let Some(entry) = one.get(body) else {
                return;
            };
            let Some(t_r) = body_record_epoch(entry) else {
                return;
            };
            let Some(props) = entry.props.as_ref() else {
                return;
            };
            match body_barycenter_position(body, t_r, &one) {
                Some(pos) => body_in_enclosure(presences, props, pos, t_r, now),
                None => false,
            }
        };
        if !admitted {
            return;
        }
        let Some(eph) = one.remove(body) else {
            return;
        };
        let _ = fetch_tx.send(FetchResult {
            source_idx,
            channels: Vec::new(),
            eph_update: Some((body.to_string(), eph)),
            asteroid_samples: Vec::new(),
            star_samples: Vec::new(),
            curves: None,
            spectral: None,
            fetch_ok: true,
            sample_ttl_override: None,
        });
    }
}

pub fn download_ephemeris_batch(items: &[(usize, SourceConfig, String)]) {
    if items.is_empty() {
        return;
    }
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
    let mut cmd = curl_base(RetryPolicy::Transient, TRANSFER_BOUND_S, 8);
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
        for (i, part, tmp_path) in &pending {
            if std::fs::rename(part, tmp_path).is_err() {
                let _ = std::fs::remove_file(part);
            } else {
                write_cdn_stamp(tmp_path, &items[*i].1.url);
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

#[derive(Clone, Copy)]
pub struct DiodeState {
    pub force_ref: [f32; 9],
    pub expose_offset: f32,
    pub em_color: [f32; 4],
}

#[derive(Clone, Copy)]
pub struct PresenceState {
    pub p: [f64; 3],
    pub v: [f64; 3],
    pub grid_step: f64,
    pub range: f64,
    pub t_thrust: f64,
    pub band: Option<(f64, f64)>,
}

impl PresenceState {
    pub fn rest() -> Self {
        PresenceState {
            p: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
            grid_step: 2147483648.0,
            range: 1280.0 * 2147483648.0 * 2.0,
            t_thrust: 0.0,
            band: None,
        }
    }
}
pub fn emit_curves(
    cset: &CurveSet,
    center: [f64; 3],
    t: f64,
    pad: f64,
    records: &mut Vec<SampleRecord>,
) {
    let Some(kernel) = kernel_id_for_force(0) else {
        return;
    };
    for star in &cset.stars {
        if !star.plx_mas.is_finite() || star.plx_mas <= 0.0 || star.samples.len() < 2 {
            continue;
        }
        let d_m = (1000.0 / star.plx_mas) * PARSEC_M;
        let ra = star.ra_deg.to_radians();
        let dec = star.dec_deg.to_radians();
        let (sa, ca) = ra.sin_cos();
        let (sd, cd) = dec.sin_cos();
        let p = [cd * ca * d_m, cd * sa * d_m, sd * d_m];
        let rel = [p[0] - center[0], p[1] - center[1], p[2] - center[2]];
        let dist = (rel[0] * rel[0] + rel[1] * rel[1] + rel[2] * rel[2]).sqrt();
        if dist > pad {
            continue;
        }
        let idx = star.samples.partition_point(|s| s.0 <= t);
        if idx == 0 || idx >= star.samples.len() {
            continue;
        }
        let (t_a, f_a) = star.samples[idx - 1];
        let (t_b, f_b) = star.samples[idx];
        let (t_s, f_s) = if (t - t_a).abs() <= (t_b - t).abs() {
            (t_a, f_a)
        } else {
            (t_b, f_b)
        };
        records.push((
            p[0],
            p[1],
            p[2],
            f_s as f64,
            t_s,
            star.cadence,
            star.cadence,
            0.0,
            kernel as f64,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            star.freq,
            star.bin_width,
            0.0,
            0.0,
        ));
    }
}
#[derive(Clone, Copy)]
pub struct PresenceFrame {
    pub n: u16,
    pub schema_hash: u32,
    pub omega: [f32; CHANNEL_CAP],
    pub aperture: [f32; CHANNEL_CAP],
    pub state: [TriState; CHANNEL_CAP],
    pub partials_hz: [f32; CHANNEL_CAP],
    pub delay_rad: [f32; CHANNEL_CAP],
    pub pan_ms: Option<f32>,
    pub tilt_ms: Option<f32>,
    pub tau_ticks: u64,
}
