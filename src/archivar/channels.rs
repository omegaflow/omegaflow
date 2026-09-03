use super::*;

pub struct StationEntry {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
}

pub fn parse_station_entries(j: &JsonVal, src: &SourceConfig) -> Vec<StationEntry> {
    let arr = match jpath_val(j, &src.stations_path) {
        Some(JsonVal::Arr(a)) => a.iter().collect::<Vec<_>>(),
        _ => return Vec::new(),
    };
    let mut stations: Vec<StationEntry> = Vec::new();
    let mut push_entry = |id_ref: &JsonVal, lat: f64, lon: f64| {
        let id = match id_ref {
            JsonVal::Str(s) => s.clone(),
            JsonVal::Num(n) => n.to_string(),
            _ => return,
        };
        stations.push(StationEntry { id, lat, lon });
    };
    let filter_ok = |v: &JsonVal| -> bool {
        match &src.stations_filter {
            Some((k, want)) => match jpath_val(v, k) {
                Some(JsonVal::Str(s)) => s == want,
                _ => false,
            },
            None => true,
        }
    };
    for v in arr {
        let lat = match jpath(v, &src.stations_lat) {
            Some(l) => l,
            None => continue,
        };
        let lon = match jpath(v, &src.stations_lon) {
            Some(l) => l,
            None => continue,
        };
        if src.stations_flatten.is_empty() {
            if filter_ok(v) {
                match jpath_val(v, &src.stations_id) {
                    Some(id_ref) => push_entry(id_ref, lat, lon),
                    None => {}
                }
            }
        } else if let Some(JsonVal::Arr(elems)) = jpath_val(v, &src.stations_flatten) {
            for e in elems {
                if !filter_ok(e) {
                    continue;
                }
                match jpath_val(e, &src.stations_id) {
                    Some(id_ref) => push_entry(id_ref, lat, lon),
                    None => push_entry(v, lat, lon),
                }
            }
        }
    }
    stations
}

pub fn parse_stations_xml(body: &str) -> Vec<StationEntry> {
    let mut out = Vec::new();
    for obs in body.split("<Observatory>").skip(1) {
        let tag = |name: &str| -> Option<&str> {
            let open = format!("<{}>", name);
            let start = obs.find(&open)? + open.len();
            let end = obs[start..].find(&format!("</{}>", name))? + start;
            Some(&obs[start..end])
        };
        let code = match tag("Code") {
            Some(c) => c.trim().to_lowercase(),
            None => continue,
        };
        let lat = match tag("Latitude").and_then(|s| s.trim().parse::<f64>().ok()) {
            Some(v) => v,
            None => continue,
        };
        let lon = match tag("Longitude").and_then(|s| s.trim().parse::<f64>().ok()) {
            Some(v) => v,
            None => continue,
        };
        out.push(StationEntry { id: code, lat, lon });
    }
    out
}

pub fn fanout_fetch(
    src: &SourceConfig,
    stations_url_tmpl: &str,
    x: f64,
    y: f64,
    z: f64,
    presence: Option<(f64, f64, f64)>,
    now: f64,
    r: f64,
    eph: &HashMap<String, BodyEphemeris>,
    env: &HashMap<String, String>,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let mut channels = Vec::new();
    let body_name = frame_body_name(&src.frame);
    let (ux, uy, uz) = presence.unwrap_or((x, y, z));
    let stations_url = match render_url(stations_url_tmpl, ux, uy, uz, now, r, &body_name, eph, lsk)
    {
        Some(u) => resolve_secret(&u, env),
        None => return channels,
    };
    let headers = render_headers(&src.headers, env);
    let raw = match fetch_raw(&stations_url, None, &headers, src.ttl) {
        Some(v) => v,
        None => return channels,
    };
    let mut stations = match parse_json(&raw) {
        Some(j) => parse_station_entries(&j, src),
        None => parse_stations_xml(&raw),
    };
    let sort_center = match presence {
        Some((px, py, pz)) => icrs_to_body_surface(px, py, pz, now, &body_name, eph),
        None => None,
    }
    .or_else(|| {
        if let Frame::Surface { lat, lon, .. } = src.frame {
            Some((lat, lon))
        } else {
            None
        }
    });
    if let Some((clat, clon)) = sort_center {
        stations.sort_by(|a, b| {
            angular_distance_deg(a.lat, a.lon, clat, clon)
                .partial_cmp(&angular_distance_deg(b.lat, b.lon, clat, clon))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    let cap = src.fanout_cap as usize;
    let base_url = match render_url(&src.url, ux, uy, uz, now, r, &body_name, eph, lsk) {
        Some(u) => u,
        None => return channels,
    };
    let body = render_source_body(src, ux, uy, uz, now, r, eph, lsk);
    let window = 3usize;
    let chunks: Vec<&StationEntry> = stations.iter().take(cap).collect();
    for (wi, chunk) in chunks.chunks(window).enumerate() {
        if wi > 0 && src.fanout_delay > 0 {
            thread::sleep(std::time::Duration::from_secs(src.fanout_delay));
        }
        thread::scope(|s| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|st| {
                    let base_url = base_url.clone();
                    let body = body.clone();
                    let headers = headers.clone();
                    let body_name = body_name.clone();
                    let st = *st;
                    s.spawn(move || -> Vec<(Channel, FieldConfig)> {
                        let url = resolve_secret(&base_url.replace("{station}", &st.id), env);
                        let post = body.as_deref().map(|b| b.replace("{station}", &st.id));
                        let raw = match fetch_raw(&url, post.as_deref(), &headers, src.ttl) {
                            Some(v) => v,
                            None => {
                                eprintln!("station {}: fetch void — retry in ttl/Φ", st.id);
                                return Vec::new();
                            }
                        };
                        let mut out = Vec::new();
                        if let ExtractResult::Measurements(mut cs) = extract(src, &raw, now, lsk) {
                            if cs.is_empty() {
                                eprintln!("station {}: extract returned no measurements", st.id);
                            }
                            for (mut ch, fc) in cs.drain(..) {
                                ch.position = Position::Surface {
                                    body_name: body_name.clone(),
                                    lat: st.lat,
                                    lon: st.lon,
                                    alt: 0.0,
                                };
                                out.push((ch, fc));
                            }
                        }
                        out
                    })
                })
                .collect();
            for h in handles {
                if let Ok(v) = h.join() {
                    channels.extend(v);
                }
            }
        });
    }
    channels
}

pub fn build_netcdf_channels(
    src: &SourceConfig,
    bytes: &[u8],
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let bytes = if bytes.starts_with(&[0x1f, 0x8b]) {
        match gunzip(bytes) {
            Some(b) => b,
            None => return Vec::new(),
        }
    } else {
        bytes.to_vec()
    };
    let nc = match NetcdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("netcdf {}: {:?}", src.url, note);
            return Vec::new();
        }
    };
    let mut channels = Vec::new();
    for ext in &src.extracts {
        let Extract::ProfileMap {
            lat_key,
            lon_key,
            epoch_key,
            pressure_var,
            pressure_scale,
            fields,
            ..
        } = ext
        else {
            continue;
        };
        let Some(lat_v) = nc.values_f64(&bytes, lat_key) else {
            continue;
        };
        let Some(lon_v) = nc.values_f64(&bytes, lon_key) else {
            continue;
        };
        let Some(juld_v) = nc.values_f64(&bytes, epoch_key) else {
            continue;
        };
        let Some(pres_v) = nc.values_f32(&bytes, pressure_var) else {
            continue;
        };
        let n_prof = lat_v.len().min(lon_v.len()).min(juld_v.len());
        let n_levels = match nc.var(pressure_var).and_then(|v| nc.var_shape(v).ok()) {
            Some(shape) => match shape.get(1) {
                Some(&n) => n as usize,
                None => continue,
            },
            None => continue,
        };
        if n_levels == 0 || pres_v.len() < n_prof * n_levels {
            continue;
        }
        let pres_fill = nc
            .var(pressure_var)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        let lat_fill = nc
            .var(lat_key)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        let lon_fill = nc
            .var(lon_key)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        let juld_fill = nc
            .var(epoch_key)
            .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
            .and_then(|a| nc.attr_num(a));
        for p in 0..n_prof {
            let lat = lat_v[p];
            let lon = lon_v[p];
            let juld = juld_v[p];
            if !lat.is_finite()
                || !lon.is_finite()
                || !juld.is_finite()
                || lat_fill.map_or(false, |f| lat == f)
                || lon_fill.map_or(false, |f| lon == f)
                || juld_fill.map_or(false, |f| juld == f)
            {
                continue;
            }
            let unix = (juld - 7305.0) * 86400.0;
            let Some(epoch) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            for fc in fields {
                let Some(vals) = nc.values_f32(&bytes, &fc.key) else {
                    continue;
                };
                if vals.len() < n_prof * n_levels {
                    continue;
                }
                let fill = nc
                    .var(&fc.key)
                    .and_then(|v| v.attrs.iter().find(|a| a.name == "_FillValue"))
                    .and_then(|a| nc.attr_num(a));
                for k in 0..n_levels {
                    let pres = pres_v[p * n_levels + k];
                    let val = vals[p * n_levels + k];
                    if !val.is_finite()
                        || !pres.is_finite()
                        || fill.map_or(false, |f| (val as f64) == f)
                        || pres_fill.map_or(false, |f| (pres as f64) == f)
                    {
                        continue;
                    }
                    let position = Position::Surface {
                        body_name: frame_body_name(&src.frame),
                        lat,
                        lon,
                        alt: -(pres as f64) * pressure_scale,
                    };
                    channels.push((
                        Channel {
                            z: 0.0,
                            freq: 0.0,
                            bin_width: 0.0,
                            epoch,
                            position,
                            name: fc.name.clone(),
                            value: val as f64,
                        },
                        fc.clone(),
                    ));
                }
            }
        }
    }
    channels
}

pub fn build_finals_channels(
    src: &SourceConfig,
    text: &str,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let mut channels = Vec::new();
    let mut fields: Vec<&FieldConfig> = Vec::new();
    for ext in &src.extracts {
        if let Extract::Field(fc) = ext {
            fields.push(fc);
        }
    }
    if fields.is_empty() {
        return channels;
    }
    for line in text.lines().rev() {
        if line.len() < 58 {
            continue;
        }
        let col = |a: usize, b: usize| -> Option<f64> {
            line.get(a..b).unwrap_or("").trim().parse::<f64>().ok()
        };
        let (Some(mjd), Some(pmx), Some(pmy), Some(ut1)) =
            (col(7, 15), col(18, 27), col(38, 47), col(60, 70))
        else {
            continue;
        };
        let unix = (mjd - 40587.0) * 86400.0;
        let Some(epoch) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let position = Position::Source;
        for fc in &fields {
            let value = match fc.key.as_str() {
                "ut1_utc" => ut1,
                "pmx" => pmx,
                "pmy" => pmy,
                _ => continue,
            };
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    position: position.clone(),
                    name: fc.name.clone(),
                    value,
                },
                (*fc).clone(),
            ));
        }
        break;
    }
    channels
}

pub fn build_ionex_channels(
    src: &SourceConfig,
    text: &str,
    now: f64,
    lsk: &LeapSeconds,
) -> Vec<(Channel, FieldConfig)> {
    let mut channels = Vec::new();
    let mut tec_field: Option<&FieldConfig> = None;
    for ext in &src.extracts {
        if let Extract::Field(fc) = ext {
            if fc.key == "tec" {
                tec_field = Some(fc);
            }
        }
    }
    let Some(fc) = tec_field else {
        return channels;
    };
    let mut exponent: Option<f64> = None;
    for line in text.lines() {
        if line.ends_with("EXPONENT") {
            exponent = line
                .split_whitespace()
                .next()
                .and_then(|t| t.parse::<f64>().ok());
            break;
        }
        if line.contains("START OF TEC MAP") {
            break;
        }
    }
    let Some(exp) = exponent else {
        return channels;
    };
    let mut lines = text.lines().peekable();
    let mut best: Option<(f64, f64, Vec<(f64, f64, f64)>)> = None;
    while let Some(l) = lines.next() {
        if !l.trim_end().ends_with("START OF TEC MAP") {
            continue;
        }
        let Some(ep_line) = lines.next() else { break };
        let t: Vec<f64> = ep_line
            .split_whitespace()
            .filter_map(|t| t.parse::<f64>().ok())
            .collect();
        if t.len() < 6 {
            continue;
        }
        let (y, mo, d) = (t[0] as i64, t[1] as u32, t[2] as u32);
        let Some(days) = ymd_to_days(y, mo, d) else {
            continue;
        };
        let unix = days as f64 * 86400.0 + t[3] * 3600.0 + t[4] * 60.0 + t[5];
        let Some(epoch) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        if epoch > now {
            continue;
        }
        let Some(hdr) = lines.next() else { break };
        let getp = |a: usize, b: usize| -> Option<f64> {
            hdr.get(a..b).unwrap_or("").trim().parse::<f64>().ok()
        };
        let (Some(lon1), Some(lon2), Some(dlon), Some(h)) =
            (getp(8, 14), getp(14, 20), getp(20, 26), getp(26, 32))
        else {
            continue;
        };
        if dlon == 0.0 {
            continue;
        }
        let nlon = ((lon2 - lon1) / dlon).round() as i64 + 1;
        let mut pts: Vec<(f64, f64, f64)> = Vec::new();
        loop {
            let Some(cur) = lines.next() else { break };
            if cur.trim_end().ends_with("END OF TEC MAP") {
                break;
            }
            let Some(lat) = cur.get(2..8).and_then(|s| s.trim().parse::<f64>().ok()) else {
                break;
            };
            let mut remaining = nlon as usize;
            let mut row: Option<&str> = Some(cur);
            loop {
                let Some(r) = row else { break };
                let take = remaining.min(16);
                for k in 0..take {
                    let idx = nlon as usize - remaining + k;
                    if let Some(v) = r
                        .get(32 + 5 * k..32 + 5 * (k + 1))
                        .and_then(|s| s.trim().parse::<f64>().ok())
                    {
                        let tec = v * 10f64.powf(exp);
                        if tec >= 0.0 {
                            pts.push((lat, lon1 + idx as f64 * dlon, tec));
                        }
                    }
                }
                remaining -= take;
                if remaining == 0 {
                    break;
                }
                row = lines.next();
            }
        }
        if best.as_ref().map_or(true, |(be, _, _)| epoch > *be) {
            best = Some((epoch, h * 1000.0, pts));
        }
    }
    if let Some((epoch, alt, pts)) = best {
        let body = frame_body_name(&src.frame);
        for (lat, lon, tec) in pts {
            channels.push((
                Channel {
                    z: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    epoch,
                    position: Position::Surface {
                        body_name: body.clone(),
                        lat,
                        lon,
                        alt,
                    },
                    name: fc.name.clone(),
                    value: tec,
                },
                fc.clone(),
            ));
        }
    }
    channels
}

pub fn alerce_objects(json: &JsonVal) -> Vec<(String, f64, f64)> {
    let mut out = Vec::new();
    let JsonVal::Obj(root) = json else {
        return out;
    };
    let Some(JsonVal::Arr(items)) = root.get("items") else {
        return out;
    };
    for it in items {
        let JsonVal::Obj(o) = it else { continue };
        let (Some(JsonVal::Str(oid)), Some(ra), Some(dec)) = (
            o.get("oid"),
            o.get("meanra").and_then(scalar_of),
            o.get("meandec").and_then(scalar_of),
        ) else {
            continue;
        };
        if ra.is_finite() && dec.is_finite() {
            out.push((oid.clone(), ra, dec));
        }
    }
    out
}

pub fn alerce_detection_rows(json: &JsonVal) -> Vec<(f64, f64, f64, f64, f64)> {
    let mut out = Vec::new();
    let JsonVal::Arr(rows) = json else {
        return out;
    };
    for r in rows {
        let JsonVal::Obj(o) = r else { continue };
        let (Some(ra), Some(dec), Some(mjd), Some(magpsf), Some(magap)) = (
            o.get("ra").and_then(scalar_of),
            o.get("dec").and_then(scalar_of),
            o.get("mjd").and_then(scalar_of),
            o.get("magpsf").and_then(scalar_of),
            o.get("magap").and_then(scalar_of),
        ) else {
            continue;
        };
        if ra.is_finite() && dec.is_finite() && mjd.is_finite() {
            out.push((ra, dec, mjd, magpsf, magap));
        }
    }
    out
}

pub fn build_alerce_channels(
    src: &SourceConfig,
    cap: usize,
    delay: u64,
) -> Vec<(Channel, FieldConfig)> {
    let channels = Vec::new();
    let Some(Extract::Alerce(detail)) = src
        .extracts
        .iter()
        .find(|e| matches!(e, Extract::Alerce(_)))
    else {
        return channels;
    };
    let Some(list_bytes) = fetch_raw_bytes(&src.url, src.ttl) else {
        return channels;
    };
    let Some(list_json) = parse_json(&String::from_utf8_lossy(&list_bytes)) else {
        return channels;
    };
    let objects = alerce_objects(&list_json);
    for (wi, (oid, _, _)) in objects.iter().take(cap).enumerate() {
        if wi > 0 && delay > 0 {
            thread::sleep(std::time::Duration::from_secs(delay));
        }
        let url = detail.replace("{oid}", oid);
        let Some(det_bytes) = fetch_raw_bytes(&url, src.ttl) else {
            continue;
        };
        let Some(det_json) = parse_json(&String::from_utf8_lossy(&det_bytes)) else {
            continue;
        };
        let detections = alerce_detection_rows(&det_json);
        if !detections.is_empty() {
            eprintln!(
                "alerce {}: {} detections without distance — dark until a distance channel exists (pending)",
                oid,
                detections.len()
            );
        }
    }
    channels
}

pub fn anchor(
    channel: &Channel,
    sensor: &FieldConfig,
    source_ttl: f64,
    source_idx: Option<u32>,
    frame: Option<&Frame>,
    mut origin_state: Option<&mut OriginState>,
    eph: &HashMap<String, BodyEphemeris>,
) -> Option<Sample> {
    if sensor.tau <= 0.0 {
        return None;
    }
    let motion = match &channel.position {
        Position::StateVector { p, v, .. } => Motion::Linear { p: *p, v: *v },
        Position::Surface {
            body_name,
            lat,
            lon,
            alt,
        } => {
            if eph
                .get(body_name.as_str())
                .and_then(|e| e.props.as_ref())
                .is_none()
            {
                return None;
            }
            Motion::Surface {
                body_name: body_name.clone(),
                lat: *lat,
                lon: *lon,
                alt: *alt,
            }
        }
        Position::SurfaceFlow {
            body_name,
            lat,
            lon,
            alt,
            speed,
            track,
            vrate,
        } => {
            if eph
                .get(body_name.as_str())
                .and_then(|e| e.props.as_ref())
                .is_none()
            {
                return None;
            }
            let v = match vrate {
                Some(v) => *v,
                None => return None,
            };
            match surface_motion(
                body_name,
                *lat,
                *lon,
                *alt,
                *speed,
                *track,
                v,
                channel.epoch,
                eph,
            ) {
                Some(m) => m,
                None => return None,
            }
        }
        Position::Barycenter { body_name, scale } => {
            if eph
                .get(body_name.as_str())
                .and_then(|e| e.props.as_ref())
                .is_none()
            {
                return None;
            }
            Motion::Barycenter {
                body_name: body_name.clone(),
                scale: *scale,
            }
        }
        Position::Source => match frame {
            Some(f) => match frame_motion(f, None, None, channel.epoch, eph) {
                Some(m) => m,
                None => return None,
            },
            None => return None,
        },
    };
    let abs = match motion.at(channel.epoch, channel.epoch, eph) {
        Some(p) => p,
        None => return None,
    };
    if !abs[0].is_finite()
        || !abs[1].is_finite()
        || !abs[2].is_finite()
        || !channel.epoch.is_finite()
    {
        return None;
    }
    let mut resid_ema = 0.0;
    if let Some(ref mut st) = origin_state {
        if st.has_prev {
            let dt_raw = (channel.epoch - st.prev_epoch).abs();
            if dt_raw > 0.0 && source_ttl > 0.0 {
                let dt = dt_raw;
                if let Some(pm) = &st.prev_motion {
                    if let Some(pred) = pm.at(channel.epoch, st.prev_epoch, eph) {
                        let resid = ((pred[0] - abs[0]).powi(2)
                            + (pred[1] - abs[1]).powi(2)
                            + (pred[2] - abs[2]).powi(2))
                        .sqrt();
                        let alpha = 1.0 - (-dt / source_ttl).exp();
                        st.resid_ema += (resid / dt - st.resid_ema) * alpha;
                    }
                }
            }
        }
        resid_ema = st.resid_ema;
        st.prev_epoch = channel.epoch;
        st.prev_abs = abs;
        st.prev_motion = Some(motion.clone());
        st.has_prev = true;
    }
    let (anchor_vmax, anchor_amax, anchor_p0) =
        match law_bounds(&motion, channel.epoch, resid_ema, eph) {
            Some(b) => b,
            None => return None,
        };
    if !anchor_p0[0].is_finite()
        || !anchor_p0[1].is_finite()
        || !anchor_p0[2].is_finite()
        || !anchor_vmax.is_finite()
        || !anchor_amax.is_finite()
    {
        return None;
    }
    let body_props = motion
        .anchor_body()
        .and_then(|name| eph.get(name))
        .and_then(|e| e.props.as_ref());
    let extent = kernel_extent(sensor.force, sensor.kernel, body_props, sensor.tau);
    if !extent.is_finite() {
        return None;
    }
    Some(Sample {
        source: match source_idx {
            Some(idx) => SampleSource::Source(idx),
            None => SampleSource::Sensor,
        },
        epoch: channel.epoch,
        ttl: source_ttl,
        extent,
        tau: sensor.tau,
        kernel_id: sensor.kernel as f64,
        force_type: sensor.force as f64,
        absorption: sensor.absorption,
        advection: sensor.advection,
        anchor_vmax,
        anchor_amax,
        anchor_p0,
        motion: motion.clone(),
        val: match convert_to_si(channel.value, &sensor.unit) {
            Some(v) => v,
            None => {
                register_unconverted_unit(&sensor.unit, &channel.name);
                return None;
            }
        },
        name: channel.name.clone(),
        z: channel.z,
        freq: channel.freq,
        bin_width: channel.bin_width,
        color_index: 0.0,
        phase: None,
    })
}

pub fn body_channels(name: &str, props: &BodyProperties, now: f64) -> Vec<(Channel, FieldConfig)> {
    let mut out = Vec::new();
    if let Some(gm) = props.gm {
        out.push((
            Channel {
                z: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                name: format!("{}.mass", name),
                value: gm,
                position: Position::Source,
                epoch: now,
            },
            FieldConfig {
                key: format!("{}.mass", name),
                name: format!("{}.mass", name),
                kernel: 0,
                force: 1,
                tau: f64::INFINITY,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            },
        ));
    }
    if let Some((omega_g, sigma)) = props.omega_g {
        let tau = if omega_g > 0.0 { 1.0 / omega_g } else { 0.0 };
        out.push((
            Channel {
                z: 0.0,
                freq: omega_g,
                bin_width: sigma,
                name: format!("{}.omega_g", name),
                value: omega_g,
                position: Position::Source,
                epoch: now,
            },
            FieldConfig {
                key: format!("{}.omega_g", name),
                name: format!("{}.omega_g", name),
                kernel: 0,
                force: 1,
                tau,
                absorption: 0.0,
                advection: 0.0,
                unit: String::new(),
                fold: None,
            },
        ));
    }
    out
}
