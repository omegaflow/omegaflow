use super::*;

pub const NAIF_LSK_TTL_SECS: u64 = 86400;

pub const NAIF_LSK_EMBEDDED: &str = include_str!("kernels/naif0012.tls");

pub fn embedded_lsk() -> Option<LeapSeconds> {
    crate::lsk::parse(NAIF_LSK_EMBEDDED)
}

pub fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from(".")
}

pub fn state_asset(rel: &str) -> std::path::PathBuf {
    let state_candidate = state_dir().join(rel);
    if state_candidate.exists() {
        return state_candidate;
    }
    resolve_asset(rel)
}

pub fn resolve_asset(rel: &str) -> std::path::PathBuf {
    let cwd_candidate = std::path::PathBuf::from(rel);
    if cwd_candidate.exists() {
        return cwd_candidate;
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let exe_candidate = dir.join(rel);
            if exe_candidate.exists() {
                return exe_candidate;
            }
        }
    }
    if !rel.starts_with('.') {
        eprintln!("asset {} absent (CWD {:?})", rel, std::env::current_dir());
    }
    cwd_candidate
}

pub const SURFACE_MOTION_DT: f64 = 0.01;

pub const MAX_SAMPLES: usize = 1 << 22;

pub struct TemporalRing {
    pub static_in: usize,
    pub temporal_in: usize,
    pub temporal_kept: usize,
    pub temporal_dropped: usize,
}

pub fn is_static(sample: &Sample) -> bool {
    matches!(sample.source, SampleSource::Ephemeris)
}

pub fn temporal_ring(
    static_catalog: &[Sample],
    temporal: &mut Vec<Sample>,
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

pub fn sense_membrane(
    buf: &Buffer,
    center: [f64; 3],
    t2: f64,
    pad: f64,
    delta_t_cache: f64,
    floor: &[f64; 9],
    softening: f64,
    forward: [f64; 3],
    records: &mut Vec<SampleRecord>,
    eph: &HashMap<String, BodyEphemeris>,
) {
    query_hash(
        &buf.cache,
        center,
        t2,
        pad,
        delta_t_cache,
        floor,
        softening,
        forward,
        records,
        eph,
    );
    for sh in &buf.spectral {
        let Some(p) = sh.motion.at(t2, sh.epoch, eph) else {
            continue;
        };
        let Some(p2) = sh.motion.at(t2 + 1e-3, sh.epoch, eph) else {
            continue;
        };
        let ddx = p[0] - center[0];
        let ddy = p[1] - center[1];
        let ddz = p[2] - center[2];
        if ddx * ddx + ddy * ddy + ddz * ddz > pad * pad {
            continue;
        }
        let vx = (p2[0] - p[0]) / 1e-3;
        let vy = (p2[1] - p[1]) / 1e-3;
        let vz = (p2[2] - p[2]) / 1e-3;
        for &(freq, bin_width, val) in &sh.bins {
            records.push((
                p[0],
                p[1],
                p[2],
                val,
                sh.epoch,
                sh.ttl,
                sh.tau,
                0.0,
                sh.kernel_id,
                sh.force_type,
                sh.absorption,
                sh.advection,
                vx,
                vy,
                vz,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                freq,
                bin_width,
                0.0,
                0.0,
            ));
        }
    }
}

pub fn surface_motion(
    body_name: &str,
    lat: f64,
    lon: f64,
    alt: f64,
    speed: f64,
    track: f64,
    vrate: f64,
    t: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Motion> {
    let p0 = body_fixed_to_icrs(body_name, lat, lon, alt, t, eph)?;
    let p1 = body_fixed_to_icrs(body_name, lat, lon, alt, t + 1.0, eph)?;
    let v_frame = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let latr = lat.to_radians();
    let lonr = lon.to_radians();
    let trk = track.to_radians();
    let v_e = speed * trk.sin();
    let v_n = speed * trk.cos();
    let v_ecef = [
        -v_e * lonr.sin() - v_n * latr.sin() * lonr.cos() + vrate * latr.cos() * lonr.cos(),
        v_e * lonr.cos() - v_n * latr.sin() * lonr.sin() + vrate * latr.cos() * lonr.sin(),
        v_n * latr.cos() + vrate * latr.sin(),
    ];
    let r = eph
        .get(body_name)
        .and_then(|e| e.props.as_ref())
        .map(|p| p.radius_m)?;
    let cl = latr.cos();
    let dt = SURFACE_MOTION_DT;
    let pp = body_fixed_to_icrs(
        body_name,
        lat + v_ecef[1] * dt / r,
        lon + v_ecef[0] * dt / (r * cl),
        alt + v_ecef[2] * dt,
        t,
        eph,
    )?;
    let v_rot = [
        (pp[0] - p0[0]) / dt,
        (pp[1] - p0[1]) / dt,
        (pp[2] - p0[2]) / dt,
    ];
    Some(Motion::Linear {
        p: p0,
        v: [
            v_frame[0] + v_rot[0],
            v_frame[1] + v_rot[1],
            v_frame[2] + v_rot[2],
        ],
    })
}

#[cfg(feature = "browser_relay")]
pub fn body_id_to_name(bodies: &[String], id: u32) -> Option<String> {
    if id == 0 {
        return None;
    }
    bodies.get((id - 1) as usize).cloned()
}

pub fn frame_motion(
    frame: &Frame,
    spd: Option<f64>,
    hdg: Option<f64>,
    t: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Motion> {
    match frame {
        Frame::Surface {
            body_name,
            lat,
            lon,
            alt,
            ..
        } => match (spd, hdg) {
            (Some(s), Some(h)) if s > 0.0 => {
                surface_motion(body_name, *lat, *lon, *alt, s, h, 0.0, t, eph)
            }
            _ => {
                if eph.get(body_name).and_then(|e| e.props.as_ref()).is_some() {
                    Some(Motion::Surface {
                        body_name: body_name.clone(),
                        lat: *lat,
                        lon: *lon,
                        alt: *alt,
                    })
                } else {
                    None
                }
            }
        },
        Frame::Barycenter {
            body_name, scale, ..
        } => {
            if eph.get(body_name).is_some() {
                Some(Motion::Barycenter {
                    body_name: body_name.clone(),
                    scale: *scale,
                })
            } else {
                None
            }
        }
        Frame::Manifest => None,
    }
}

pub fn leap_seconds(time: &Arc<Mutex<Option<LeapSeconds>>>) -> Option<LeapSeconds> {
    match time.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => None,
    }
}

pub fn system_now(time: &Arc<Mutex<Option<LeapSeconds>>>) -> Option<f64> {
    match leap_seconds(time) {
        Some(lsk) => lsk.system_now_tdb(),
        None => None,
    }
}

pub fn kernel_extent(
    force_type: u8,
    kernel_id: u8,
    body_props: Option<&BodyProperties>,
    tau: f64,
) -> f64 {
    if tau == 0.0 {
        return 0.0;
    }
    let p = match body_props {
        Some(p) => p,
        None => return 0.0,
    };
    if force_type == 1 {
        return p.radius_m;
    }
    let reach_time = tau;
    if kernel_id == 1 {
        return p.gaussian_inverse_square * reach_time;
    }
    if kernel_id == 2 {
        return p.gaussian_inverse * reach_time;
    }
    if kernel_id == 3 {
        return (2.0 * p.erfc * reach_time).sqrt();
    }
    if kernel_id == 4 {
        return p.exponential_decay;
    }
    if kernel_id == 5 {
        return p.patch_levy * reach_time;
    }
    0.0
}

pub const AUDIO_SPEED_AIR: f64 = 343.0;
pub const SEISMIC_BODY_SPEED: f64 = 6000.0;
pub const SEISMIC_SURFACE_SPEED: f64 = 3000.0;
pub const ADVECTIVE_BASE_SPEED: f64 = 1.0;
pub const DIFFUSIVITY_THERMAL: f64 = 0.3;

pub const DIFFUSIVITY_MOLECULAR: f64 = 0.05;

pub fn signal_reach(force_type: f64, advection: f64, age: f64) -> Option<f64> {
    match force_type as u8 {
        0 | 1 | 8 => Some(C_LIGHT * age),
        2 => Some(AUDIO_SPEED_AIR * age),
        3 => Some(SEISMIC_BODY_SPEED * age),
        4 => Some(SEISMIC_SURFACE_SPEED * age),
        7 => {
            if advection > 0.0 {
                Some(advection * age)
            } else {
                Some(ADVECTIVE_BASE_SPEED * age)
            }
        }
        5 => Some((2.0 * DIFFUSIVITY_THERMAL * age).sqrt()),
        6 => Some((2.0 * DIFFUSIVITY_MOLECULAR * age).sqrt()),
        _ => None,
    }
}

pub fn dispatch_reach(fields: &[FieldConfig], src_ttl: f64) -> Option<f64> {
    let mut reach: Option<f64> = None;
    for fc in fields {
        if let Some(rr) = signal_reach(fc.force as f64, fc.advection, src_ttl * 64.0) {
            reach = Some(reach.map_or(rr, |prev| prev.max(rr)));
        }
    }
    reach
}

pub fn anchor_velocity(
    frame: &Frame,
    now: f64,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<[f64; 3]> {
    match frame {
        Frame::Surface { body_name, .. } => body_barycenter_velocity(body_name, now, eph),
        Frame::Barycenter { body_name, scale } => body_barycenter_velocity(body_name, now, eph)
            .map(|[x, y, z]| [x * scale, y * scale, z * scale]),
        Frame::Manifest => None,
    }
}

pub fn propagation_speed(force_type: f64, advection: f64) -> Option<f64> {
    match force_type as u8 {
        0 | 1 | 8 => Some(C_LIGHT),
        2 => Some(AUDIO_SPEED_AIR),
        3 => Some(SEISMIC_BODY_SPEED),
        4 => Some(SEISMIC_SURFACE_SPEED),
        5 => Some(DIFFUSIVITY_THERMAL),
        6 => Some(DIFFUSIVITY_MOLECULAR),
        7 => {
            if advection > 0.0 {
                Some(advection)
            } else {
                Some(ADVECTIVE_BASE_SPEED)
            }
        }
        _ => None,
    }
}

pub fn wire_extent(extent: f64) -> f64 {
    if extent.is_finite() {
        extent
    } else {
        0.0
    }
}

pub fn sensor_config(name: &str) -> Option<BrowserSensor> {
    let kl = name.to_lowercase();
    let (force, kernel, ttl) = if kl.contains("temperature")
        || kl.contains("temp")
        || kl == "thermistor"
    {
        (5, 3, 60.0)
    } else if kl.contains("pressure") || kl.contains("baro") || kl == "pres" {
        (6, 3, 60.0)
    } else if kl.contains("humidity") || kl.contains("humid") || kl == "rh" || kl == "moisture" {
        (5, 3, 300.0)
    } else if kl.contains("wind") && kl.contains("speed") || kl == "windspeed" || kl == "anemometer"
    {
        (6, 3, 10.0)
    } else if (kl.contains("wind") && kl.contains("dir"))
        || kl == "winddirection"
        || kl == "winddir"
        || kl == "vane"
    {
        (6, 3, 10.0)
    } else if kl.contains("mic")
        || kl.contains("audio")
        || kl.contains("sound")
        || kl.contains("noise")
        || kl == "spl"
    {
        (2, 1, 0.01)
    } else if kl.contains("light")
        || kl.contains("lux")
        || kl.contains("lumin")
        || kl.contains("irradiance")
    {
        (0, 0, 10.0)
    } else if kl.contains("battery")
        && (kl.contains("level") || kl.contains("pct") || kl.contains("soc"))
    {
        (5, 3, 60.0)
    } else if kl.contains("battery") && (kl.contains("volt") || kl == "voltage") {
        (8, 5, 60.0)
    } else if kl.contains("battery") && kl.contains("current") {
        (8, 5, 10.0)
    } else if kl.contains("co2")
        || kl.contains("voc")
        || kl.contains("pm2")
        || kl.contains("pm10")
        || kl.contains("gas")
    {
        (5, 3, 300.0)
    } else if kl.contains("magnet") || kl.contains("compass") || kl.contains("b_field") {
        (0, 0, 10.0)
    } else if kl.contains("accelerometer") || kl.contains("acc") || kl.contains("vibration") {
        (3, 1, 1.0)
    } else if kl.contains("gyro") {
        (3, 1, 1.0)
    } else if kl.contains("gravity") {
        (1, 0, 10.0)
    } else if kl.contains("camera") || kl.contains("video") {
        (0, 0, 1.0 / 30.0)
    } else if kl.contains("battery") && kl.contains("charging") {
        (8, 5, 60.0)
    } else if kl.contains("gps") || kl.contains("gnss") {
        return None;
    } else if kl.starts_with("event.") {
        (0, 0, 10.0)
    } else {
        return None;
    };
    Some(BrowserSensor {
        key: name.into(),
        force,
        kernel,
        ttl,
    })
}
