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
    std::path::PathBuf::from("state")
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
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let exe_candidate = dir.join(rel);
        if exe_candidate.exists() {
            return exe_candidate;
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

pub fn sense_membrane(buf: &Buffer, ctx: MembraneCtx<'_>, records: &mut Vec<SampleRecord>) {
    query_hash(&buf.cache, ctx, records);
    for sh in &buf.spectral {
        let Some(p) = sh.motion.at(ctx.t2, sh.epoch, ctx.eph) else {
            continue;
        };
        let Some(p2) = sh.motion.at(ctx.t2 + 1e-3, sh.epoch, ctx.eph) else {
            continue;
        };
        let ddx = p[0] - ctx.center[0];
        let ddy = p[1] - ctx.center[1];
        let ddz = p[2] - ctx.center[2];
        if ddx * ddx + ddy * ddy + ddz * ddz > ctx.pad * ctx.pad {
            continue;
        }
        let vx = (p2[0] - p[0]) / 1e-3;
        let vy = (p2[1] - p[1]) / 1e-3;
        let vz = (p2[2] - p[2]) / 1e-3;
        let ebv = sightline_ebv(sh, buf);
        let Some(ci) = crate::spectral::sed_to_bp_rp(&sh.bins, ebv) else {
            continue;
        };
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
                slot_or_pad(sh.absorption),
                slot_or_pad(sh.advection),
                vx,
                vy,
                vz,
                if sh.redshift > 0.0 { sh.redshift } else { 0.0 },
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                ci,
                freq,
                bin_width,
                0.0,
                presence_flags(
                    None,
                    sh.absorption,
                    sh.advection,
                    if sh.redshift > 0.0 {
                        sh.redshift
                    } else {
                        SLOT_ABSENT
                    },
                ),
            ));
        }
    }
}

fn sightline_ebv(sh: &SpectralHash, buf: &Buffer) -> Option<f64> {
    let Motion::Spherical { rec } = &sh.motion else {
        return None;
    };
    let map = buf.bayestar.as_ref()?;
    let theta = (90.0 - rec.dec_deg).to_radians();
    let phi = rec.ra_deg.rem_euclid(360.0).to_radians();
    let idx = crate::bayestar::leaf_record(&map.query, theta, phi)? as usize;
    let bf = map.best_fit.get(idx)?;
    if !rec.plx_mas.is_finite() || rec.plx_mas <= 0.0 {
        return None;
    }
    let d_pc = 1000.0 / rec.plx_mas;
    let mu = crate::bayestar::mu_of_r_pc(d_pc)?;
    crate::bayestar::ebv_at(bf, mu)
}

pub struct SurfaceMotionParams<'a> {
    pub body_name: &'a str,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub speed: f64,
    pub track: f64,
    pub vrate: f64,
    pub t: f64,
    pub eph: &'a HashMap<String, BodyEphemeris>,
}

pub fn surface_motion(p: SurfaceMotionParams<'_>) -> Option<Motion> {
    let SurfaceMotionParams {
        body_name,
        lat,
        lon,
        alt,
        speed,
        track,
        vrate,
        t,
        eph,
    } = p;
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
            (Some(s), Some(h)) if s > 0.0 => surface_motion(SurfaceMotionParams {
                body_name,
                lat: *lat,
                lon: *lon,
                alt: *alt,
                speed: s,
                track: h,
                vrate: 0.0,
                t,
                eph,
            }),
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

pub fn medium_reach(
    force_type: u8,
    age: f64,
    medium: Option<&crate::media::MediumParams>,
    advection: f64,
) -> Option<f64> {
    if age == 0.0 {
        return Some(0.0);
    }
    match force_type {
        2 => Some(medium?.sound_speed_m_s * age),
        3 => Some(medium?.p_wave_m_s * age),
        4 => Some(medium?.s_wave_m_s * age),
        5 => Some((2.0 * medium?.thermal_diffusivity_m2_s * age).sqrt()),
        6 => Some((2.0 * medium?.molecular_diffusivity_m2_s * age).sqrt()),
        7 => slot_measured(advection).then_some(advection * age),
        0 | 1 | 8 => Some(C_LIGHT * age),
        _ => None,
    }
}

pub fn granule_span_seconds(e: &BodyEphemeris) -> Option<f64> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in &e.granules {
        if !(g.t0_jd.is_finite() && g.dt_jd.is_finite() && g.dt_jd > 0.0) {
            continue;
        }
        lo = lo.min(g.t0_jd - g.dt_jd);
        hi = hi.max(g.t0_jd + g.dt_jd);
    }
    let span = (hi - lo) * 86400.0;
    (span.is_finite() && span > 0.0).then_some(span)
}

pub fn body_anchor_samples(
    name: &str,
    eph: &HashMap<String, BodyEphemeris>,
    epoch: f64,
) -> Vec<Sample> {
    let mut out = Vec::new();
    let Some(be) = eph.get(name) else {
        return out;
    };
    let Some(props) = be.props.as_ref() else {
        eprintln!("body {name}: the ephemeris carries no props — no anchor");
        return out;
    };
    let radius = props.radius_m;
    if !(radius.is_finite() && radius > 0.0) {
        eprintln!("body {name}: props carry no finite radius — no anchor");
        return out;
    }
    let Some(ttl) = granule_span_seconds(be) else {
        eprintln!("body {name}: granules carry no finite span — no anchor");
        return out;
    };
    let motion = Motion::Barycenter {
        body_name: name.to_string(),
        scale: 1.0,
    };
    let Some((anchor_vmax, anchor_amax, anchor_p0)) = law_bounds(&motion, epoch, 0.0, eph) else {
        return out;
    };
    let mut push = |val: f64, tau: f64, freq: f64, bin_width: f64, suffix: &str| {
        out.push(Sample {
            source: SampleSource::Ephemeris,
            epoch,
            ttl,
            extent: radius,
            tau,
            kernel_id: 0.0,
            force_type: 1.0,
            absorption: absorption_for_force(1, 0.0),
            advection: 0.0,
            anchor_vmax,
            anchor_amax,
            anchor_p0,
            motion: motion.clone(),
            val,
            name: format!("{name}.{suffix}"),
            z_flux: SLOT_ABSENT,
            freq,
            bin_width,
            color_index: 0.0,
            phase: None,
        });
    };
    if let Some(gm) = props.gm.filter(|v| v.is_finite() && *v > 0.0) {
        push(gm, f64::INFINITY, 0.0, 0.0, "mass");
    } else {
        eprintln!("body {name}: props carry no finite gm — the mass channel stays absent");
    }
    if let Some((omega_g, sigma)) = props
        .omega_g
        .filter(|(w, s)| w.is_finite() && *w > 0.0 && s.is_finite())
    {
        push(omega_g, 1.0 / omega_g, omega_g, sigma, "omega_g");
    } else {
        eprintln!("body {name}: props carry no omega_g — the rotation channel stays absent");
    }
    out
}

pub fn all_body_anchor_samples(eph: &HashMap<String, BodyEphemeris>) -> Vec<Sample> {
    let mut names: Vec<&String> = eph.keys().collect();
    names.sort();
    let mut out = Vec::new();
    for name in names {
        let Some(be) = eph.get(name) else { continue };
        let Some(epoch) = body_record_epoch(be) else {
            continue;
        };
        out.extend(body_anchor_samples(name, eph, epoch));
    }
    out
}

pub fn signal_reach(
    force_type: f64,
    advection: f64,
    age: f64,
    freq: f64,
    bin_width: f64,
) -> Option<f64> {
    propagation_speed(force_type, advection, freq, bin_width).map(|v| v * age)
}

pub fn enclosure_rho(vmax: f64, amax: f64, dt: f64, pad: f64) -> f64 {
    vmax * dt + 0.5 * amax * dt * dt + pad
}

pub fn dispatch_reach(fields: &[FieldConfig], src_ttl: f64) -> Option<f64> {
    let mut reach: Option<f64> = None;
    for fc in fields {
        if let Some(rr) = signal_reach(
            fc.force as f64,
            fc.advection,
            src_ttl * 64.0,
            fc.freq,
            fc.bin_width,
        ) {
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

pub fn flat_propagation_speed(force_type: f64, advection: f64) -> Option<f64> {
    match force_type as u8 {
        0 | 1 | 8 => Some(C_LIGHT),
        7 => slot_measured(advection).then_some(advection),
        _ => None,
    }
}

pub fn propagation_speed(
    force_type: f64,
    advection: f64,
    freq: f64,
    bin_width: f64,
) -> Option<f64> {
    if let Some(v) = crate::mathematikerin::dispersion::v_at(force_type as u8, freq, bin_width) {
        return Some(v);
    }
    flat_propagation_speed(force_type, advection)
}

pub fn wire_extent(extent: f64) -> f64 {
    if extent.is_finite() { extent } else { 0.0 }
}

pub fn sensor_config(name: &str) -> Option<BrowserSensor> {
    let kl = name.to_lowercase();
    let (force, kernel, ttl, unit, tau) = if kl.contains("temperature")
        || kl.contains("temp")
        || kl == "thermistor"
    {
        (5, 3, 60.0, "", Some(3600.0))
    } else if kl.contains("pressure") || kl.contains("baro") || kl == "pres" {
        (6, 3, 60.0, "", Some(3600.0))
    } else if kl.contains("humidity") || kl.contains("humid") || kl == "rh" || kl == "moisture" {
        (5, 3, 300.0, "", Some(3600.0))
    } else if (kl.contains("wind") && kl.contains("speed"))
        || kl == "windspeed"
        || kl == "anemometer"
        || (kl.contains("wind") && kl.contains("dir"))
        || kl == "winddirection"
        || kl == "winddir"
        || kl == "vane"
    {
        (6, 3, 10.0, "", Some(3600.0))
    } else if kl.contains("mic")
        || kl.contains("audio")
        || kl.contains("sound")
        || kl.contains("noise")
        || kl == "spl"
    {
        (2, 1, 0.01, "", None)
    } else if kl.contains("light")
        || kl.contains("lux")
        || kl.contains("lumin")
        || kl.contains("irradiance")
    {
        (0, 0, 10.0, "", None)
    } else if kl.contains("battery")
        && (kl.contains("level") || kl.contains("pct") || kl.contains("soc"))
    {
        (8, 0, 60.0, "%", Some(60.0))
    } else if kl.contains("battery") && (kl.contains("volt") || kl == "voltage") {
        (8, 0, 60.0, "v", Some(60.0))
    } else if kl.contains("battery") && kl.contains("current") {
        (8, 0, 10.0, "a", Some(10.0))
    } else if kl.contains("co2")
        || kl.contains("voc")
        || kl.contains("pm2")
        || kl.contains("pm10")
        || kl.contains("gas")
    {
        (5, 3, 300.0, "", None)
    } else if kl.contains("spo2") || kl.contains("oxygen") || kl.contains("sao2") || kl == "o2" {
        (6, 3, 10.0, "%", None)
    } else if kl.contains("magnet") || kl.contains("compass") || kl.contains("b_field") {
        (0, 0, 10.0, "", None)
    } else if kl.contains("accelerometer")
        || kl.contains("acc")
        || kl.contains("vibration")
        || kl.contains("gyro")
    {
        (3, 1, 1.0, "", None)
    } else if kl.contains("gravity") {
        (1, 0, 10.0, "", None)
    } else if kl.contains("camera") || kl.contains("video") {
        (0, 0, 1.0 / 30.0, "", None)
    } else if kl.contains("battery") && kl.contains("charging") {
        (8, 0, 60.0, "1", Some(60.0))
    } else if kl.contains("gps")
        || kl.contains("gnss")
        || kl.contains("speed")
        || kl.contains("velocity")
        || kl.contains("track")
        || kl.contains("heading")
        || kl.contains("course")
        || kl.contains("bearing")
    {
        if kl.contains("speed") || kl.contains("velocity") {
            (7, 1, 10.0, "m/s", None)
        } else if kl.contains("track")
            || kl.contains("heading")
            || kl.contains("course")
            || kl.contains("bearing")
        {
            (0, 0, 10.0, "deg", None)
        } else {
            return None;
        }
    } else if kl == "hr" || kl.contains("heart") || kl.contains("bpm") {
        (6, 3, 10.0, "bpm", None)
    } else if kl.contains("cadence") {
        (7, 1, 10.0, "1/min", None)
    } else if kl.starts_with("event.") {
        (0, 0, 10.0, "", None)
    } else {
        return None;
    };
    Some(BrowserSensor {
        key: name.into(),
        force,
        kernel,
        ttl,
        tau,
        unit: unit.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_declared_unit_names_what_the_sender_carries() {
        assert_eq!(sensor_config("battery.voltage").expect("sensor").unit, "v");
        assert_eq!(sensor_config("battery.current").expect("sensor").unit, "a");
        assert_eq!(sensor_config("battery.level").expect("sensor").unit, "%");
        assert_eq!(sensor_config("battery.charging").expect("sensor").unit, "1");
    }

    #[test]
    fn the_battery_family_carries_the_electric_force() {
        assert_eq!(sensor_config("battery.level").expect("sensor").force, 8);
        assert_eq!(sensor_config("battery.voltage").expect("sensor").force, 8);
        assert_eq!(sensor_config("battery.current").expect("sensor").force, 8);
        assert_eq!(sensor_config("battery.charging").expect("sensor").force, 8);
    }

    #[test]
    fn the_electric_force_carries_the_inverse_square_kernel() {
        assert_eq!(sensor_config("battery.level").expect("sensor").kernel, 0);
        assert_eq!(sensor_config("battery.voltage").expect("sensor").kernel, 0);
        assert_eq!(sensor_config("battery.current").expect("sensor").kernel, 0);
        assert_eq!(sensor_config("battery.charging").expect("sensor").kernel, 0);
    }

    #[test]
    fn the_meteo_series_carry_the_hourly_relaxation() {
        assert_eq!(
            sensor_config("temperature").expect("sensor").tau,
            Some(3600.0)
        );
        assert_eq!(sensor_config("pressure").expect("sensor").tau, Some(3600.0));
        assert_eq!(sensor_config("humidity").expect("sensor").tau, Some(3600.0));
        assert_eq!(
            sensor_config("wind.speed").expect("sensor").tau,
            Some(3600.0)
        );
        assert_eq!(
            sensor_config("wind.direction").expect("sensor").tau,
            Some(3600.0)
        );
    }

    #[test]
    fn the_battery_tau_lives_in_the_registry_not_the_wire() {
        assert_eq!(
            sensor_config("battery.level").expect("sensor").tau,
            Some(60.0)
        );
        assert_eq!(
            sensor_config("battery.voltage").expect("sensor").tau,
            Some(60.0)
        );
        assert_eq!(
            sensor_config("battery.current").expect("sensor").tau,
            Some(10.0)
        );
        assert_eq!(
            sensor_config("battery.charging").expect("sensor").tau,
            Some(60.0)
        );
        assert_eq!(
            sensor_config("temperature").expect("sensor").tau,
            Some(3600.0)
        );
    }

    #[test]
    fn the_undeclared_wire_unit_stays_absent() {
        assert_eq!(sensor_config("temperature").expect("sensor").unit, "");
        assert_eq!(sensor_config("wind.speed").expect("sensor").unit, "");
    }

    #[test]
    fn the_spo2_sensor_carries_the_diffusion_force_and_percent_unit() {
        let spo2 = sensor_config("spo2").expect("sensor");
        assert_eq!(spo2.unit, "%");
        assert_eq!(spo2.force, 6);
        assert_eq!(spo2.kernel, 3);
        assert_eq!(spo2.tau, None);
    }

    #[test]
    fn the_gnss_speed_carries_the_advective_force_and_metre_per_second() {
        let speed = sensor_config("gnss.speed").expect("sensor");
        assert_eq!(speed.unit, "m/s");
        assert_eq!(speed.force, 7);
        assert_eq!(speed.kernel, 1);
    }

    #[test]
    fn the_gnss_track_carries_the_em_force_and_degrees() {
        let track = sensor_config("gnss.track").expect("sensor");
        assert_eq!(track.unit, "deg");
        assert_eq!(track.force, 0);
        assert_eq!(track.kernel, 0);
    }

    #[test]
    fn the_gnss_position_and_quality_metrics_stay_absent() {
        assert!(sensor_config("gnss.lat").is_none());
        assert!(sensor_config("gnss.lon").is_none());
        assert!(sensor_config("gnss.alt").is_none());
        assert!(sensor_config("gnss.hdop").is_none());
        assert!(sensor_config("gnss.sats").is_none());
    }

    #[test]
    fn a_shelf_covered_band_answers_the_measured_v_through_the_wiring() {
        let v = propagation_speed(4.0, 0.0, 0.05, 0.0166667)
            .expect("the 0.05 Hz Rayleigh band carries a speed");
        assert!(
            (v - 2899.8).abs() < 1e-6,
            "the cone carries the measured 2899.8 m/s, not the flat 3000: {v}"
        );
        let reach = signal_reach(4.0, 0.0, 10.0, 0.05, 0.0166667)
            .expect("the Rayleigh band carries a reach");
        assert!(
            (reach - 2899.8 * 10.0).abs() < 1e-6,
            "the signal cone is v(f)·age: {reach}"
        );
    }

    #[test]
    fn an_uncovered_earth_band_carries_no_flat_fallback() {
        assert_eq!(
            propagation_speed(4.0, 0.0, 0.07, 0.0),
            None,
            "0.07 Hz lies above the Rayleigh rows — no earth speed is invented"
        );
        assert_eq!(
            propagation_speed(0.0, 0.0, 5.0e14, 0.0),
            Some(C_LIGHT),
            "an em band without a shelf row carries c"
        );
        assert_eq!(
            propagation_speed(2.0, 0.0, 440.0, 0.0),
            None,
            "acoustic carries no flat carrier without a body medium"
        );
        assert_eq!(
            propagation_speed(7.0, 12.5, 0.0, 0.0),
            Some(12.5),
            "the advective force carries its advection, not the shelf"
        );
        assert_eq!(
            propagation_speed(7.0, 0.0, 0.0, 0.0),
            Some(0.0),
            "a measured 0.0 advection carries speed 0, never the base"
        );
        assert_eq!(
            propagation_speed(7.0, SLOT_ABSENT, 0.0, 0.0),
            None,
            "absent advection has no propagation term"
        );
        assert_eq!(
            signal_reach(4.0, 0.0, 10.0, 0.07, 0.0),
            None,
            "an uncovered earth band carries no flat cone"
        );
    }

    #[test]
    fn the_f32_wgsl_selection_stays_within_tolerance_of_the_f64_wiring() {
        let flat = |ft: u8, advection: f32, advection_measured: bool| -> Option<f32> {
            match ft {
                0 | 1 | 8 => Some(C_LIGHT as f32),
                7 => {
                    if advection_measured {
                        Some(advection)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        };
        let fixtures: [(u8, f64, f64, f64, bool); 8] = [
            (4, 0.0, 0.05, 0.0166667, false),
            (4, 0.0, 0.005, 0.0016667, false),
            (4, 0.0, 0.07, 0.0, false),
            (0, 0.0, 9.861594e15, 0.0, false),
            (0, 0.0, 5.0e14, 0.0, false),
            (7, 12.5, 0.0, 0.0, true),
            (7, 0.0, 0.0, 0.0, true),
            (7, -1.0, 0.0, 0.0, false),
        ];
        for (ft, adv, freq, bw, present) in fixtures {
            let cpu = propagation_speed(ft as f64, adv, freq, bw);
            let gpu = crate::mathematikerin::dispersion::v_at_f32(ft, freq as f32, bw as f32)
                .or_else(|| flat(ft, adv as f32, present));
            match (cpu, gpu) {
                (Some(c), Some(g)) => {
                    if c == 0.0 {
                        assert_eq!(g, 0.0, "cpu/wgsl agreement at force {ft}: zero speed");
                    } else {
                        let rel = ((g as f64 - c) / c).abs();
                        assert!(
                            rel < 1e-4,
                            "cpu/wgsl parity: force {ft} at {freq} Hz — wgsl {g} cpu {c} rel {rel}"
                        );
                    }
                }
                (None, None) => {}
                (c, g) => panic!("cpu/wgsl disagreement: force {ft} — wgsl {g:?} cpu {c:?}"),
            }
        }
    }
}
