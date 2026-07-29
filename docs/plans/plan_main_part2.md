=== Replace lines 1685-1699 ===
old:
            let mut out = Vec::with_capacity(11 + records.len() * 56);
            out.extend_from_slice(&[0xCF, 0x86]);
            out.push(1u8);
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(records.len() as u32).to_le_bytes());
            for &(val, x, y, z, r, epoch, ttl) in &records {
                out.extend_from_slice(&x.to_le_bytes());
                out.extend_from_slice(&y.to_le_bytes());
                out.extend_from_slice(&z.to_le_bytes());
                out.extend_from_slice(&val.to_le_bytes());
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&epoch.to_le_bytes());
                out.extend_from_slice(&ttl.to_le_bytes());
            }
            write_ws_binary(&mut stream, &out);
new:
            let mut out = Vec::with_capacity(11 + records.len() * 72);
            out.extend_from_slice(&[0xCF, 0x86]);
            out.push(1u8);
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&(records.len() as u32).to_le_bytes());
            for &(x, y, z, val, extent, epoch, ttl, tau, force_type) in &records {
                out.extend_from_slice(&x.to_le_bytes());
                out.extend_from_slice(&y.to_le_bytes());
                out.extend_from_slice(&z.to_le_bytes());
                out.extend_from_slice(&val.to_le_bytes());
                out.extend_from_slice(&extent.to_le_bytes());
                out.extend_from_slice(&epoch.to_le_bytes());
                out.extend_from_slice(&ttl.to_le_bytes());
                out.extend_from_slice(&tau.to_le_bytes());
                out.extend_from_slice(&force_type.to_le_bytes());
            }
            write_ws_binary(&mut stream, &out);

=== Replace lines 474-482 ===
old:
fn presence_gate(presences: &[(f64, f64, f64, f64, f64)], pos: (f64, f64, f64), r: f64) -> bool {
    presences.iter().any(|&(_, x, y, z, extent)| {
        let reach = r * Φ + extent;
        let dx = x - pos.0;
        let dy = y - pos.1;
        let dz = z - pos.2;
        dx * dx + dy * dy + dz * dz <= reach * reach
    })
}
new:
fn presence_gate(presences: &[(f64, f64, f64, f64, f64)], pos: (f64, f64, f64), extent: f64) -> bool {
    presences.iter().any(|&(_, x, y, z, range)| {
        let reach = extent * Φ + range;
        let dx = x - pos.0;
        let dy = y - pos.1;
        let dz = z - pos.2;
        dx * dx + dy * dy + dz * dz <= reach * reach
    })
}

=== Delete lines 484-486 ===

=== Replace lines 1197-1207 ===
old:
struct SourceConfig {
    ttl: u64,
    url: String,
    frame: Frame,
    res: i32,
    format: String,
    extracts: Vec<Extract>,
    headers: Vec<(String, String)>,
    pos_fields: Option<(String, String, Option<String>, f64)>,
    ap: Option<f64>,
}
new:
struct SourceConfig {
    ttl: u64,
    url: String,
    frame: Frame,
    force: String,
    tau: Option<f64>,
    tau_key: Option<String>,
    format: String,
    extracts: Vec<Extract>,
    headers: Vec<(String, String)>,
    pos_fields: Option<(String, String, Option<String>, f64)>,
}

=== After line 1187 (after last Extract variant), insert ===
fn force_constants(force: &str) -> Option<(f64, f64, bool, u8)> {
    match force {
        "em"               => Some((C_LIGHT,        (1.0 / C_LIGHT) * 1024.0,                            false, 0)),
        "gravity"          => Some((C_LIGHT,        (AU / C_LIGHT) / 1024.0,                            false, 1)),
        "acoustic"         => Some((V_SOUND_288,    1.0 / V_SOUND_288,                                  false, 2)),
        "seismic-body"     => Some((V_P_GRANITE,   1.0 / V_P_GRANITE,                                  false, 3)),
        "seismic-surface"  => Some((V_S_GRANITE,   1.0 / V_S_GRANITE,                                  false, 4)),
        "thermal"          => Some((ALPHA_AIR,     1.0 / ALPHA_AIR,                                    true,  5)),
        "diffusion"        => Some((D_AIR,         1.0 / D_AIR,                                        true,  6)),
        "advection"        => Some((V_WIND_REF,    1.0 / V_WIND_REF,                                   false, 7)),
        _ => None,
    }
}

fn compute_extent(force: &str, tau: f64) -> f64 {
    let (v_or_d, _, is_diff, _) = match force_constants(force) {
        Some(fc) => fc,
        None => return 0.0,
    };
    if is_diff { (2.0 * v_or_d * tau).sqrt() } else { v_or_d * tau }
}

fn force_type_of(force: &str) -> f64 {
    force_constants(force).map(|(_, _, _, id)| id as f64).unwrap_or(0.0)
}

=== Replace lines 1608, 1621 ===
old:
                        let res = (aperture(0) / acc).log10().floor() as i32;
new:
                        let extent = 1.0;

old:
                            r: aperture(res),
new:
                            extent,
                            tau: 60.0,
                            force_type: 0.0,

=== Replace line 1617-1627 ===
old:
                        let sample = Sample {
                            origin: (u32::MAX, 0, 0),
                            epoch: now,
                            ttl: Φ * Φ * station.ema_interval,
                            r: aperture(res),
                            vmax,
                            amax,
                            p0f,
                            motion,
                            fields: station_fields,
                        };
new:
                        let sample = Sample {
                            origin: (u32::MAX, 0, 0),
                            epoch: now,
                            ttl: Φ * Φ * station.ema_interval,
                            extent,
                            tau: 60.0,
                            force_type: 0.0,
                            vmax,
                            amax,
                            p0f,
                            motion,
                            fields: station_fields,
                        };

=== Replace lines 3405-3425 ===
old:
    let sample_r = if let Some(ap_deg) = src.ap {
        let lat_rad = match &pend.position {
            PendingPosition::Geodetic { lat, .. } => *lat * std::f64::consts::PI / 180.0,
            PendingPosition::GeodeticFlow { lat, .. } => *lat * std::f64::consts::PI / 180.0,
            _ => 0.0,
        };
        111195.0 * ap_deg * lat_rad.cos()
    } else {
        aperture(src.res)
    };
    Some(Sample {
        origin,
        epoch: pend.epoch,
        ttl: src.ttl.max(1) as f64,
        r: sample_r,
        vmax,
        amax,
        p0f,
        motion,
        fields: pend.fields,
    })
new:
    let fc = force_constants(&src.force).unwrap();
    let (v_or_d, tau_default, is_diff, force_type) = fc;
    let tau = src.tau.unwrap_or(tau_default);
    let extent = if is_diff { (2.0 * v_or_d * tau).sqrt() } else { v_or_d * tau };
    Some(Sample {
        origin,
        epoch: pend.epoch,
        ttl: src.ttl.max(1) as f64,
        extent,
        tau,
        force_type: force_type as f64,
        vmax,
        amax,
        p0f,
        motion,
        fields: pend.fields,
    })

=== Replace lines 3428-3433 ===
old:
fn render_headers(src: &SourceConfig, x: f64, y: f64, z: f64, now: f64) -> Vec<(String, String)> {
    src.headers
        .iter()
        .map(|(k, v)| (k.clone(), render_url(v, x, y, z, now, src.res)))
        .collect()
}
new:
fn render_headers(src: &SourceConfig, x: f64, y: f64, z: f64, now: f64, extent: f64) -> Vec<(String, String)> {
    src.headers
        .iter()
        .map(|(k, v)| (k.clone(), render_url(v, x, y, z, now, extent)))
        .collect()
}

=== Replace line 2320 ===
old:
fn render_url(template: &str, x: f64, y: f64, z: f64, tdb_secs: f64, res: i32) -> String {
new:
fn render_url(template: &str, x: f64, y: f64, z: f64, tdb_secs: f64, extent: f64) -> String {

=== Replace lines 2382-2385 ===
old:
    let res_usize = res.max(0) as usize;
    let lat_str = format!("{:.*}", res_usize, lat);
    let lon_str = format!("{:.*}", res_usize, lon);
    let half_deg = aperture(res) / 111319.0;
new:
    let half_deg = extent / 111319.0;
    let res_usize = 6usize;
    let lat_str = format!("{:.6}", lat);
    let lon_str = format!("{:.6}", lon);

=== Replace lines 2300-2303 + 2306-2309 (match arms in load_sources) ===
old:
    let mut cur_res: i32 = 0;
    let mut cur_res_set = false;
    let mut cur_ap: Option<f64> = None;
new:
    let mut cur_force = String::new();
    let mut cur_tau: Option<f64> = None;
    let mut cur_tau_key: Option<String> = None;

old:
            "res" => {
                cur_res = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                cur_res_set = true;
            }
            "ap" => cur_ap = parts.get(1).and_then(|s| s.parse().ok()),
new:
            "force" => cur_force = parts.get(1).unwrap_or(&"").to_string(),
            "tau" => cur_tau = parts.get(1).and_then(|s| s.parse().ok()),
            "tau_key" => cur_tau_key = parts.get(1).map(|s| s.to_string()),

old:
                cur_res = 0;
                cur_res_set = false;
                cur_ap = None;
new:
                cur_force.clear();
                cur_tau = None;
                cur_tau_key = None;

=== Replace flush! macro in load_sources (lines 1816-1871) ===
old:
            if active {
                let mut res = cur_res;
                if !cur_res_set && cur_lat.is_some() {
                    res = match cur_lat_str.find('.') {
                        Some(dot) => (cur_lat_str.len() - dot - 1) as i32,
                        None => 0,
                    };
                }
                let has_data_position = cur_pos.is_some()
                    || cur_extracts.iter().any(|e| {
                        matches!(
                            e,
                            Extract::Map { .. }
                                | Extract::GeojsonEvents { .. }
                                | Extract::Ephemeris(_)
                                | Extract::CelestialMap { .. }
                                | Extract::Flatten { .. }
                                | Extract::Rows { .. }
                                | Extract::KeplerMap { .. }
                        )
                    });
                let frame = if let (Some(lat), Some(lon)) = (cur_lat, cur_lon) {
                    Some(Frame::Ground {
                        lat,
                        lon,
                        alt: cur_alt,
                    })
                } else if let Some(scale) = cur_terra {
                    Some(Frame::Terra { scale })
                } else if let Some(scale) = cur_mars {
                    Some(Frame::Mars { scale })
                } else if has_data_position {
                    Some(Frame::Data)
                } else if cur_url.contains("{lat}")
                    || cur_url.contains("{lon}")
                    || cur_url.contains("{x}")
                    || cur_url.contains("{y}")
                    || cur_url.contains("{z}")
                    || cur_url.contains("{grid")
                {
                    Some(Frame::Query)
                } else {
                    eprintln!("source refused (no reference frame): {}", cur_name);
                    None
                };
                if let Some(frame) = frame {
                    sources.push(SourceConfig {
                        ttl: cur_ttl,
                        url: cur_url.clone(),
                        frame,
                        res,
                        format: cur_format.clone(),
                        extracts: cur_extracts.clone(),
                        headers: cur_headers.clone(),
                        pos_fields: cur_pos.clone(),
                        ap: cur_ap,
                    });
                }
            }
new:
            if active {
                if cur_force.is_empty() {
                    eprintln!("source refused (no force): {}", cur_name);
                } else if force_constants(&cur_force).is_none() {
                    eprintln!("source refused (unknown force '{}'): {}", cur_force, cur_name);
                } else {
                    let has_data_position = cur_pos.is_some()
                        || cur_extracts.iter().any(|e| {
                            matches!(
                                e,
                                Extract::Map { .. }
                                    | Extract::GeojsonEvents { .. }
                                    | Extract::Ephemeris(_)
                                    | Extract::CelestialMap { .. }
                                    | Extract::Flatten { .. }
                                    | Extract::Rows { .. }
                                    | Extract::KeplerMap { .. }
                            )
                        });
                    let frame = if let (Some(lat), Some(lon)) = (cur_lat, cur_lon) {
                        Some(Frame::Ground {
                            lat,
                            lon,
                            alt: cur_alt,
                        })
                    } else if let Some(scale) = cur_terra {
                        Some(Frame::Terra { scale })
                    } else if let Some(scale) = cur_mars {
                        Some(Frame::Mars { scale })
                    } else if has_data_position {
                        Some(Frame::Data)
                    } else if cur_url.contains("{lat}")
                        || cur_url.contains("{lon}")
                        || cur_url.contains("{x}")
                        || cur_url.contains("{y}")
                        || cur_url.contains("{z}")
                        || cur_url.contains("{grid")
                    {
                        Some(Frame::Query)
                    } else {
                        eprintln!("source refused (no reference frame): {}", cur_name);
                        None
                    };
                    if let Some(frame) = frame {
                        sources.push(SourceConfig {
                            ttl: cur_ttl,
                            url: cur_url.clone(),
                            frame,
                            force: cur_force.clone(),
                            tau: cur_tau,
                            tau_key: cur_tau_key.clone(),
                            format: cur_format.clone(),
                            extracts: cur_extracts.clone(),
                            headers: cur_headers.clone(),
                            pos_fields: cur_pos.clone(),
                        });
                    }
                }
            }

=== Replace warm_cache extent + render_url calls (lines 3474-3475, all src.res references) ===
old:
                let r = aperture(src.res);
new:
                let fc = force_constants(&src.force).unwrap();
                let (v_or_d, tau_default, is_diff, _) = fc;
                let tau = src.tau.unwrap_or(tau_default);
                let r = if is_diff { (2.0 * v_or_d * tau).sqrt() } else { v_or_d * tau };

=== Every render_url call in warm_cache: change last arg from `src.res` to `r` (the just-computed extent) ===
Replace all:
    render_url(&src.url, ..., src.res)
with:
    render_url(&src.url, ..., r)

Replace all:
    render_headers(src, ..., now)
with:
    render_headers(src, ..., now, r)

=== Warm_cache region_quantize calls (lines 3573-3574, 3648-3649) ===
old:
                                let origin = (
                                    i as u32,
                                    region_quantize(lat, src.res),
                                    region_quantize(lon, src.res),
                                );
new:
                                let origin = (i as u32, 0, 0);

=== Second region_quantize block (same pattern, same replacement) ===

=== Warm_cache Data frame presence_gate call (line 3638) ===
Unchanged. `r` is now computed as extent via force_constants(), presence_gate uses it as extent parameter.

=== index.html lines 265-299 (WGSL fragment shader) ===

Replace line 265 (insert erfc before @fragment):
old:
@fragment fn fs(i: V) -> @location(0) vec4f {
new:
fn erfc(x: f32) -> f32 {
  let xa = abs(x);
  let t = 1.0 / (1.0 + 0.3275911 * xa);
  let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
  let y = poly * exp(-xa * xa);
  return select(y, 2.0 - y, x < 0.0);
}
@fragment fn fs(i: V) -> @location(0) vec4f {

=== Replace omega loop in fs (lines 276-288) ===
old:
    var omega = 0.0f;
    let phi = vp.right.xyz * ((i.u.x - 0.5) * w * scale) + vp.up.xyz * ((0.5 - i.u.y) * h * scale);
    for (var j = 0u; j < count; j = j + 1u) {
        let m = field[j];
        let mt = props[j];
        let d = m.xyz - phi;
        let d2 = dot(d, d);
        let a2 = mt.x * mt.x;
        if (d2 > a2) { continue; }
        let val_eff = m.w * exp2(-mt.y / max(mt.z, 1.0) * 1.4426950408889634);
        omega = omega + val_eff * max(0.0, 1.0 - d2 / max(a2, 1.0)) / (d2 + scale * scale);
    }
new:
    var omega = 0.0f;
    let phi = vp.right.xyz * ((i.u.x - 0.5) * w * scale) + vp.up.xyz * ((0.5 - i.u.y) * h * scale);
    let softening = vp.expose.y;
    let s2 = softening * softening;
    for (var j = 0u; j < count; j = j + 1u) {
        let m = field[j];
        let mt = props[j];
        let d = m.xyz - phi;
        let d2 = dot(d, d);
        let d_mag = sqrt(d2);
        let extent = mt.x;
        let force_type = u32(mt.w);
        var spatial: f32;
        if (force_type == 5u || force_type == 6u) {
            spatial = erfc(d_mag / (extent * 1.41421356237));
        } else if (force_type == 4u) {
            spatial = exp(-d2 / (2.0 * extent * extent)) / (d_mag + softening);
        } else if (force_type == 0u || force_type == 1u) {
            spatial = 1.0 / (d2 + s2);
        } else {
            var kernel: f32;
            if (force_type == 7u) {
                kernel = erfc(d_mag / (extent * 1.41421356237));
            } else {
                kernel = exp(-d2 / (2.0 * extent * extent));
            }
            spatial = kernel / (d2 + s2);
        }
        omega = omega + m.w * spatial;
    }
    let aw = abs(omega);
    let lvl = vp.expose.x;
    if (lvl <= 0.0 || aw < lvl * exp2(-16.0)) { return vec4f(vec3f(starBright), 1.0); }
    let t2 = clamp((log2(aw / lvl) + 16.0) / 32.0, 0.0, 1.0);
    let c1 = mix(vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.3, 0.8), clamp(t2 * 4.0, 0.0, 1.0));
    let c2 = mix(c1, vec3f(0.2, 0.8, 1.0), clamp((t2 - 0.25) * 4.0, 0.0, 1.0));
    let c3 = mix(c2, vec3f(1.0, 0.7, 0.1), clamp((t2 - 0.5) * 4.0, 0.0, 1.0));
    let c4 = mix(c3, vec3f(1.0, 1.0, 1.0), clamp((t2 - 0.75) * 4.0, 0.0, 1.0));
    let fc = c4 + starBright * 0.08;
    return vec4f(fc, 1.0);
}