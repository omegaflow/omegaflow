use super::*;

pub fn load_sources() -> Vec<SourceConfig> {
    let content = match std::fs::read_to_string("phi/sources.φ") {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    parse_sources(&content)
}

pub fn parse_sources(content: &str) -> Vec<SourceConfig> {
    let mut sources = Vec::new();

    let mut cur_ttl: u64 = 0;
    let mut cur_url = String::new();
    let mut cur_format = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new();
    let mut cur_headers: Vec<(String, String)> = Vec::new();
    let mut cur_target: Option<String> = None;
    let mut cur_catalog: Option<String> = None;
    let mut cur_max_freq: Option<f64> = None;
    let mut cur_min_freq: Option<f64> = None;
    let mut cur_body: Option<String> = None;
    let mut cur_post_body: Option<String> = None;

    let mut cur_stations_url: Option<String> = None;
    let mut cur_stations_path = String::from("stations");
    let mut cur_stations_lat = String::from("lat");
    let mut cur_stations_lon = String::from("lng");
    let mut cur_stations_id = String::from("id");
    let mut cur_hapi_fill: HashMap<String, f64> = HashMap::new();
    let mut cur_flux_from_mag: Option<String> = None;
    let mut cur_abs_mag_from: Option<String> = None;
    let mut cur_catalog_epoch: Option<f64> = None;
    let mut cur_repeat_ra_bins: u32 = 0;
    let mut cur_fanout_cap: u32 = 0;
    let mut cur_stations_flatten = String::new();
    let mut cur_stations_filter: Option<(String, String)> = None;
    let mut cur_fanout_delay: u64 = 0;
    let mut cur_frame: Option<Frame> = None;
    let mut active = false;

    macro_rules! flush {
        () => {
            if active && cur_ttl > 0 && !cur_url.is_empty() {
                if cur_format == "kernel_text" || cur_frame.is_some() {
                    if cur_flux_from_mag.is_some() && cur_abs_mag_from.is_some() {
                        eprintln!(
                            "source refused: flux_from_mag + abs_mag_from conflict at {}",
                            cur_url
                        );
                    } else {
                        sources.push(SourceConfig {
                            ttl: cur_ttl,
                            url: std::mem::take(&mut cur_url),
                            frame: match &cur_frame {
                                Some(f) => f.clone(),
                                None => Frame::Manifest,
                            },
                            format: std::mem::take(&mut cur_format),
                            extracts: std::mem::take(&mut cur_extracts),
                            headers: std::mem::take(&mut cur_headers),
                            post_body: cur_post_body.clone(),
                            target: cur_target.clone(),
                            catalog: cur_catalog.clone(),
                            max_freq: cur_max_freq,
                            min_freq: cur_min_freq,
                            body: cur_body.clone(),
                            stations_url: cur_stations_url.clone(),
                            stations_path: std::mem::take(&mut cur_stations_path),
                            stations_lat: std::mem::take(&mut cur_stations_lat),
                            stations_lon: std::mem::take(&mut cur_stations_lon),
                            stations_id: std::mem::take(&mut cur_stations_id),
                            hapi_fill: std::mem::take(&mut cur_hapi_fill),
                            flux_from_mag: cur_flux_from_mag.clone(),
                            abs_mag_from: cur_abs_mag_from.clone(),
                            catalog_epoch: cur_catalog_epoch,
                            repeat_ra_bins: cur_repeat_ra_bins,
                            fanout_cap: cur_fanout_cap,
                            stations_flatten: std::mem::take(&mut cur_stations_flatten),
                            stations_filter: cur_stations_filter.take(),
                            fanout_delay: cur_fanout_delay,
                        });
                    }
                }
            }
        };
    }

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "url" if parts.len() >= 2 => {
                flush!();
                cur_url = parts[1].to_string();
                cur_format.clear();
                cur_extracts.clear();
                cur_headers.clear();
                cur_ttl = 0;
                cur_target = None;
                cur_catalog = None;
                cur_max_freq = None;
                cur_min_freq = None;
                cur_body = None;
                cur_post_body = None;
                cur_stations_url = None;
                cur_stations_path = String::from("stations");
                cur_stations_lat = String::from("lat");
                cur_stations_lon = String::from("lng");
                cur_stations_id = String::from("id");
                cur_hapi_fill.clear();
                cur_flux_from_mag = None;
                cur_abs_mag_from = None;
                cur_catalog_epoch = None;
                cur_repeat_ra_bins = 0;
                cur_fanout_cap = 0;
                cur_stations_flatten = String::new();
                cur_stations_filter = None;
                cur_fanout_delay = 0;
                cur_frame = None;
                active = true;
            }
            "ttl" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<u64>() {
                    cur_ttl = v;
                } else {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!("ttl non-numeric: {}", line),
                    );
                }
            }
            "at" if parts.len() >= 2 => {
                let body = parts[1].to_string();
                cur_body = Some(body.clone());
                cur_frame = Some(Frame::Barycenter {
                    body_name: body,
                    scale: 1.0,
                });
            }
            "on" if parts.len() >= 4 => {
                let body = parts[1].to_string();
                cur_body = Some(body.clone());
                let lat: f64 = match parts[2].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("on lat non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                let lon: f64 = match parts[3].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("on lon non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                let alt: f64 = match parts.get(4) {
                    Some(s) => match s.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            report_anomaly(
                                "Invalid Syntax",
                                &cur_url,
                                &format!("on alt non-numeric: {}", line),
                            );
                            continue;
                        }
                    },
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("on without alt refused — declare alt: {}", line),
                        );
                        continue;
                    }
                };
                cur_frame = Some(Frame::Surface {
                    body_name: body,
                    lat,
                    lon,
                    alt,
                });
            }
            "on" => {
                report_anomaly(
                    "Invalid Syntax",
                    &cur_url,
                    &format!("on needs <body> <lat> <lon> [alt]: {}", line),
                );
            }
            "map" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Map {
                    arr_path: parts[1].to_string(),
                    lat_key: String::new(),
                    lon_key: String::new(),
                    alt_key: String::new(),
                    epoch_key: String::new(),
                    val_key: String::new(),
                    alt_scale: 1.0,
                    vel_key: String::new(),
                    vel_scale: 1.0,
                    trk_key: String::new(),
                    vr_key: String::new(),
                    fields: Vec::new(),
                    lat_sign: None,
                    lon_sign: None,
                    epoch_scale: 1.0,
                    tau_key: String::new(),
                    mag_type_key: String::new(),
                });
            }
            "cmap" if parts.len() >= 2 => {
                cur_extracts.push(Extract::CelestialMap {
                    arr_path: parts[1].to_string(),
                    ra_key: String::new(),
                    dec_key: String::new(),
                    dist_key: String::new(),
                    dist_scale: 1.0,
                    plx_key: String::new(),
                    z_key: String::new(),
                    pmra_key: String::new(),
                    pmdec_key: String::new(),
                    rv_key: String::new(),
                    rv_scale: 1.0,
                    epoch_key: String::new(),
                    fields: Vec::new(),
                    tau_key: String::new(),
                });
            }
            "profile" if parts.len() >= 2 => {
                cur_extracts.push(Extract::ProfileMap {
                    arr_path: parts[1].to_string(),
                    lat_key: String::new(),
                    lon_key: String::new(),
                    epoch_key: String::new(),
                    pressure_var: String::new(),
                    pressure_scale: 1.0,
                    fields: Vec::new(),
                });
            }
            "rows" => {
                cur_extracts.push(Extract::Rows {
                    last_line: false,
                    fields: Vec::new(),
                    tau_key: String::new(),
                });
            }
            "first" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let filter = match parse_where(&parts) {
                    Ok(f) => f,
                    Err(()) => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                cur_extracts.push(Extract::First(fc, filter));
            }
            "last" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let filter = match parse_where(&parts) {
                    Ok(f) => f,
                    Err(()) => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                cur_extracts.push(Extract::Last(fc, filter));
            }
            "count" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Count(FieldConfig {
                    key: parts[1].to_string(),
                    name: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        parts[1].to_string()
                    },
                    kernel: 0,
                    force: 0,
                    tau: 0.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    fold: None,
                }));
            }
            "lastrow" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                cur_extracts.push(Extract::LastRow(fc));
            }
            "lastobj" if parts.len() >= 5 => {
                cur_extracts.push(Extract::LastObj(
                    parts[1].to_string(),
                    parts[2].to_string(),
                    parts[3].to_string(),
                    parts[4].to_string(),
                ));
            }
            "lastline" if parts.len() >= 2 => {
                cur_extracts.push(Extract::LastLine(parts[1].to_string()));
            }
            "objlast" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                cur_extracts.push(Extract::ObjLast(fc));
            }
            "geojson" if parts.len() >= 6 => {
                let mag_key = parts[1].to_string();
                let min_mag: f64 = match parts[2].parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut outputs = Vec::new();
                for i in 3..parts.len().min(5) {
                    outputs.push(parts[i].to_string());
                }
                let tau: f64 = match parts.get(5).and_then(|s| s.parse().ok()) {
                    Some(v) if v > 0.0 => v,
                    _ => continue,
                };
                let absorption: f64 = match parts.get(6).and_then(|s| s.parse().ok()) {
                    Some(v) => v,
                    _ => continue,
                };
                let advection: f64 = match parts.get(7).and_then(|s| s.parse().ok()) {
                    Some(v) => v,
                    _ => continue,
                };
                cur_extracts.push(Extract::GeojsonEvents {
                    mag_key,
                    min_mag,
                    outputs,
                    tau,
                    absorption,
                    advection,
                    mag_type_key: String::new(),
                });
            }
            "path" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                cur_extracts.push(Extract::Path(fc));
            }
            "deep" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                cur_extracts.push(Extract::Deep(fc));
            }
            "regex" if parts.len() >= 9 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                cur_extracts.push(Extract::Regex(fc));
            }
            "flatten" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Flatten {
                    arr_path: parts[1].to_string(),
                    geom_path: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    },
                    epoch_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    fields: Vec::new(),
                });
            }
            "cmrpolygon" if parts.len() >= 2 => {
                cur_extracts.push(Extract::CmrPolygon {
                    arr_path: parts[1].to_string(),
                    fields: Vec::new(),
                    epoch_key: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    },
                    alt_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    val_key: if parts.len() >= 5 {
                        parts[4].to_string()
                    } else {
                        String::new()
                    },
                });
            }
            "celestialpolygon" if parts.len() >= 2 => {
                let radius: f64 = match parts.get(2).and_then(|s| s.parse().ok()) {
                    Some(v) => v,
                    None => {
                        eprintln!(
                            "celestialpolygon radius parse returned void: {:?}",
                            parts.get(2)
                        );
                        continue;
                    }
                };
                cur_extracts.push(Extract::CelestialPolygon {
                    arr_path: parts[1].to_string(),
                    radius,
                    fields: Vec::new(),
                    epoch_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    val_key: if parts.len() >= 5 {
                        parts[4].to_string()
                    } else {
                        String::new()
                    },
                });
            }
            "keplermap" if parts.len() >= 2 => {
                cur_extracts.push(Extract::KeplerMap {
                    arr_path: parts[1].to_string(),
                    a_key: if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    },
                    e_key: if parts.len() >= 4 {
                        parts[3].to_string()
                    } else {
                        String::new()
                    },
                    i_key: if parts.len() >= 5 {
                        parts[4].to_string()
                    } else {
                        String::new()
                    },
                    om_key: String::new(),
                    w_key: String::new(),
                    ma_key: String::new(),
                    epoch_key: String::new(),
                    q_key: String::new(),
                    tp_key: String::new(),
                    fields: Vec::new(),
                });
            }
            "hapi" if parts.len() >= 2 => {
                let mut params = Vec::new();
                for s in &parts[1..] {
                    if let Some((k, v)) = s.split_once('=') {
                        params.push((k.to_string(), v.to_string()));
                    }
                }
                cur_extracts.push(Extract::Hapi(params));
            }
            "hapi_fill" if parts.len() >= 2 => {
                for s in &parts[1..] {
                    if let Some((k, v)) = s.split_once('=') {
                        if let Ok(fv) = v.parse::<f64>() {
                            cur_hapi_fill.insert(k.to_string(), fv);
                        }
                    }
                }
            }
            "alerce" if parts.len() >= 2 => {
                cur_extracts.push(Extract::Alerce(parts[1].to_string()));
            }
            "xmlcount" if parts.len() >= 3 => {
                cur_extracts.push(Extract::XmlCount(
                    parts[1].to_string(),
                    parts[2].to_string(),
                ));
            }
            "ephemeris" | "vectors" => {
                eprintln!(
                    "{} refused: Horizons-text extract is superseded by format ephemeris_binary + body channels (value would be a fabricated range)",
                    parts[0]
                );
            }
            "field" if parts.len() == 6 => {
                let f = match force_id_of(parts[2]) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown force \"{}\": {}", parts[2], line),
                        );
                        continue;
                    }
                };
                report_physics_mismatch(f, parts[3], parts[1], &cur_url);
                let tau: f64 = match parts[4].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let k = match kernel_id_of(parts[5]) {
                    Some(k) => k,
                    None => match kernel_id_for_force(f) {
                        Some(k) => k,
                        None => continue,
                    },
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[1].to_string(),
                    kernel: k,
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[3].to_string(),
                    fold: None,
                };
                if let Some(ext) = cur_extracts.last_mut() {
                    let fields: Option<&mut Vec<FieldConfig>> = match ext {
                        Extract::Map { fields, .. } => Some(fields),
                        Extract::CelestialMap { fields, .. } => Some(fields),
                        Extract::Rows { fields, .. } => Some(fields),
                        Extract::Flatten { fields, .. } => Some(fields),
                        Extract::CmrPolygon { fields, .. } => Some(fields),
                        Extract::CelestialPolygon { fields, .. } => Some(fields),
                        Extract::KeplerMap { fields, .. } => Some(fields),
                        Extract::ProfileMap { .. } => {
                            eprintln!(
                                "field refused at {}: 5/6-token field inside a profile block is an orphan — the 9-token form carries the pressure arm",
                                parts[1]
                            );
                            continue;
                        }
                        _ => None,
                    };
                    if let Some(flds) = fields {
                        flds.push(fc);
                    } else {
                        cur_extracts.push(Extract::Field(fc.clone()));
                    }
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "field" if parts.len() == 5 => {
                let f = match force_id_of(parts[2]) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown force \"{}\": {}", parts[2], line),
                        );
                        continue;
                    }
                };
                report_physics_mismatch(f, parts[3], parts[1], &cur_url);
                let tau: f64 = match parts[4].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[1].to_string(),
                    kernel: match kernel_id_for_force(f) {
                        Some(k) => k,
                        None => continue,
                    },
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[3].to_string(),
                    fold: None,
                };
                if let Some(ext) = cur_extracts.last_mut() {
                    let fields: Option<&mut Vec<FieldConfig>> = match ext {
                        Extract::Map { fields, .. } => Some(fields),
                        Extract::CelestialMap { fields, .. } => Some(fields),
                        Extract::Rows { fields, .. } => Some(fields),
                        Extract::Flatten { fields, .. } => Some(fields),
                        Extract::CmrPolygon { fields, .. } => Some(fields),
                        Extract::CelestialPolygon { fields, .. } => Some(fields),
                        Extract::KeplerMap { fields, .. } => Some(fields),
                        Extract::ProfileMap { .. } => {
                            eprintln!(
                                "field refused at {}: 5/6-token field inside a profile block is an orphan — the 9-token form carries the pressure arm",
                                parts[1]
                            );
                            continue;
                        }
                        _ => None,
                    };
                    if let Some(flds) = fields {
                        flds.push(fc);
                    } else {
                        cur_extracts.push(Extract::Field(fc.clone()));
                    }
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "field" if parts.len() == 3 => {
                eprintln!(
                    "field refused at {}: 3-token field carries no tau (τ-Gate)",
                    parts[1]
                );
            }
            "field_in" if parts.len() >= 3 => {
                eprintln!(
                    "field_in refused at {}: legacy directive — the --gold port migrates field_in to field (τ-Gate)",
                    parts[1]
                );
            }
            "field" if parts.len() >= 9 => {
                if parts.len() >= 10 && parts[9] == "where" {
                    eprintln!(
                        "where refused at {}: the row filter lives on first/last, not on field",
                        parts[1]
                    );
                    continue;
                }
                let k = match kernel_id_of(parts[3]) {
                    Some(k) => k,
                    None => continue,
                };
                let f = match force_id_of(parts[4]) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown force \"{}\": {}", parts[4], line),
                        );
                        continue;
                    }
                };
                report_physics_mismatch(f, parts[5], parts[1], &cur_url);
                let tau: f64 = match parts[6].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => {
                        eprintln!(
                            "field refused at {}: tau absent or not positive (τ-Gate)",
                            parts[1]
                        );
                        continue;
                    }
                };
                let absorption: f64 = match parts[7].parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let advection: f64 = match parts[8].parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    kernel: k,
                    force: f,

                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    fold: None,
                };
                if let Some(Extract::Map { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::CelestialMap { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::Rows { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::Flatten { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::CmrPolygon { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::CelestialPolygon { fields, .. }) =
                    cur_extracts.last_mut()
                {
                    fields.push(fc);
                } else if let Some(Extract::KeplerMap { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else if let Some(Extract::ProfileMap { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "field" => {
                report_anomaly(
                    "Invalid Syntax",
                    &cur_url,
                    &format!("field arity {}: {}", parts.len(), line),
                );
            }
            "lat" if parts.len() >= 2 => match cur_extracts.last_mut() {
                Some(Extract::Map { lat_key, .. }) | Some(Extract::ProfileMap { lat_key, .. }) => {
                    *lat_key = parts[1].to_string();
                }
                _ => {}
            },
            "lon" if parts.len() >= 2 => match cur_extracts.last_mut() {
                Some(Extract::Map { lon_key, .. }) | Some(Extract::ProfileMap { lon_key, .. }) => {
                    *lon_key = parts[1].to_string();
                }
                _ => {}
            },
            "lat_sign" if parts.len() >= 2 => {
                if let Some(Extract::Map { lat_sign, .. }) = cur_extracts.last_mut() {
                    *lat_sign = Some(parts[1].to_string());
                }
            }
            "lon_sign" if parts.len() >= 2 => {
                if let Some(Extract::Map { lon_sign, .. }) = cur_extracts.last_mut() {
                    *lon_sign = Some(parts[1].to_string());
                }
            }
            "epoch_scale" if parts.len() >= 2 => {
                if let Ok(s) = parts[1].parse::<f64>() {
                    if let Some(Extract::Map { epoch_scale, .. }) = cur_extracts.last_mut() {
                        *epoch_scale = s;
                    }
                }
            }
            "alt" if parts.len() >= 2 => {
                let scale = match parts.get(2) {
                    None => 1.0,
                    Some(&"m") => 1.0,
                    Some(&"km") => 1000.0,
                    Some(&"ft") => 0.3048,
                    Some(&"cm") => 0.01,
                    Some(&"mm") => 0.001,
                    Some(&"-m") => -1.0,
                    Some(&"-km") => -1000.0,
                    Some(&"decibar") => 1.0,
                    Some(_) => continue,
                };
                if let Some(Extract::Map {
                    alt_key, alt_scale, ..
                }) = cur_extracts.last_mut()
                {
                    *alt_key = parts[1].to_string();
                    *alt_scale = scale;
                } else if let Some(Extract::ProfileMap {
                    pressure_var,
                    pressure_scale,
                    ..
                }) = cur_extracts.last_mut()
                {
                    *pressure_var = parts[1].to_string();
                    *pressure_scale = scale;
                }
            }
            "epoch" if parts.len() >= 2 => match cur_extracts.last_mut() {
                Some(Extract::Map { epoch_key, .. })
                | Some(Extract::KeplerMap { epoch_key, .. })
                | Some(Extract::ProfileMap { epoch_key, .. }) => {
                    *epoch_key = parts[1].to_string();
                }
                _ => {}
            },
            "pressure" if parts.len() >= 2 => {
                if let Some(Extract::ProfileMap {
                    pressure_var,
                    pressure_scale,
                    ..
                }) = cur_extracts.last_mut()
                {
                    *pressure_var = parts[1].to_string();
                    if parts.len() >= 3 {
                        if let Ok(s) = parts[2].parse::<f64>() {
                            *pressure_scale = s;
                        }
                    }
                }
            }
            "a" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { a_key, .. }) = cur_extracts.last_mut() {
                    *a_key = parts[1].to_string();
                }
            }
            "e" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { e_key, .. }) = cur_extracts.last_mut() {
                    *e_key = parts[1].to_string();
                }
            }
            "i" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { i_key, .. }) = cur_extracts.last_mut() {
                    *i_key = parts[1].to_string();
                }
            }
            "om" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { om_key, .. }) = cur_extracts.last_mut() {
                    *om_key = parts[1].to_string();
                }
            }
            "w" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { w_key, .. }) = cur_extracts.last_mut() {
                    *w_key = parts[1].to_string();
                }
            }
            "ma" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { ma_key, .. }) = cur_extracts.last_mut() {
                    *ma_key = parts[1].to_string();
                }
            }
            "qr" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { q_key, .. }) = cur_extracts.last_mut() {
                    *q_key = parts[1].to_string();
                }
            }
            "tp" if parts.len() >= 2 => {
                if let Some(Extract::KeplerMap { tp_key, .. }) = cur_extracts.last_mut() {
                    *tp_key = parts[1].to_string();
                }
            }
            "vel" if parts.len() >= 2 => {
                if let Some(Extract::Map {
                    vel_key, vel_scale, ..
                }) = cur_extracts.last_mut()
                {
                    if parts.len() >= 3 {
                        match convert_to_si(1.0, parts[2]) {
                            Some(scale) if scale > 0.0 => {
                                *vel_scale = scale;
                                *vel_key = parts[1].to_string();
                            }
                            _ => {
                                eprintln!(
                                    "vel refused: unit \"{}\" unconverted — SI absent (pending curation)",
                                    parts[2]
                                );
                            }
                        }
                    } else {
                        *vel_key = parts[1].to_string();
                    }
                }
            }
            "tau_key" if parts.len() >= 2 => {
                let target = match cur_extracts.last_mut() {
                    Some(Extract::Map { tau_key, .. })
                    | Some(Extract::CelestialMap { tau_key, .. })
                    | Some(Extract::Rows { tau_key, .. }) => Some(tau_key),
                    _ => None,
                };
                if let Some(tk) = target {
                    *tk = parts[1].to_string();
                }
            }
            "mag_type_key" if parts.len() >= 2 => {
                let target = match cur_extracts.last_mut() {
                    Some(Extract::Map { mag_type_key, .. })
                    | Some(Extract::GeojsonEvents { mag_type_key, .. }) => Some(mag_type_key),
                    _ => None,
                };
                if let Some(mt) = target {
                    *mt = parts[1].to_string();
                }
            }
            "fold" if parts.len() == 7 => {
                let op = match parts[1] {
                    "mean" => 1u8,
                    "diff" => 2,
                    "sum" => 3,
                    other => {
                        eprintln!("fold refused: op \"{}\" unknown (mean|diff|sum)", other);
                        continue;
                    }
                };
                let f = match force_id_of(parts[4]) {
                    Some(f) => f,
                    None => continue,
                };
                let tau: f64 = match parts[6].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let k = match kernel_id_for_force(f) {
                    Some(k) => k,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[2].to_string(),
                    name: format!("fold_{}_{}_{}", parts[1], parts[2], parts[3]),
                    kernel: k,
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[5].to_string(),
                    fold: Some((op, parts[3].to_string())),
                };
                let holder = match cur_extracts.last_mut() {
                    Some(Extract::Map { fields, .. })
                    | Some(Extract::CelestialMap { fields, .. })
                    | Some(Extract::Flatten { fields, .. })
                    | Some(Extract::Rows { fields, .. }) => Some(fields),
                    _ => None,
                };
                match holder {
                    Some(flds) => flds.push(fc),
                    None => {
                        eprintln!(
                            "fold refused: no map/cmap/flatten/rows holder at {} {}",
                            parts[2], parts[3]
                        );
                    }
                }
            }
            "trk" if parts.len() >= 2 => {
                if let Some(Extract::Map { trk_key, .. }) = cur_extracts.last_mut() {
                    *trk_key = parts[1].to_string();
                }
            }
            "vr" if parts.len() >= 2 => {
                if let Some(Extract::Map { vr_key, .. }) = cur_extracts.last_mut() {
                    *vr_key = parts[1].to_string();
                }
            }
            "val" if parts.len() >= 2 => {
                if let Some(Extract::Map { val_key, .. }) = cur_extracts.last_mut() {
                    *val_key = parts[1].to_string();
                }
            }
            "ra" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { ra_key, .. }) = cur_extracts.last_mut() {
                    *ra_key = parts[1].to_string();
                }
            }
            "dec" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { dec_key, .. }) = cur_extracts.last_mut() {
                    *dec_key = parts[1].to_string();
                }
            }
            "z" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { z_key, .. }) = cur_extracts.last_mut() {
                    *z_key = parts[1].to_string();
                }
            }
            "plx" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { plx_key, .. }) = cur_extracts.last_mut() {
                    *plx_key = parts[1].to_string();
                }
            }
            "pmra" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { pmra_key, .. }) = cur_extracts.last_mut() {
                    *pmra_key = parts[1].to_string();
                }
            }
            "pmdec" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { pmdec_key, .. }) = cur_extracts.last_mut() {
                    *pmdec_key = parts[1].to_string();
                }
            }
            "radvel" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { rv_key, .. }) = cur_extracts.last_mut() {
                    *rv_key = parts[1].to_string();
                }
            }
            "dist" if parts.len() >= 2 => {
                if let Some(Extract::CelestialMap { dist_key, .. }) = cur_extracts.last_mut() {
                    *dist_key = parts[1].to_string();
                }
            }
            "dist_scale" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    if let Some(Extract::CelestialMap { dist_scale, .. }) = cur_extracts.last_mut()
                    {
                        *dist_scale = v;
                    }
                }
            }
            "tname" if parts.len() >= 2 => {}
            "tra" if parts.len() >= 2 => {}
            "tdec" if parts.len() >= 2 => {}
            "tdist" if parts.len() >= 2 => {}
            "tdist_scale" if parts.len() >= 2 => {}
            "ta" if parts.len() >= 2 => {}
            "te" if parts.len() >= 2 => {}
            "ti" if parts.len() >= 2 => {}
            "tw" if parts.len() >= 2 => {}
            "ttranmid" if parts.len() >= 2 => {}
            "tperiod" if parts.len() >= 2 => {}
            "trp" if parts.len() >= 2 => {}
            "trs" if parts.len() >= 2 => {}
            "format" if parts.len() >= 2 => cur_format = parts[1..].join(" "),
            "body" if parts.len() >= 2 => {
                cur_body = Some(parts[1].to_string());
            }
            "force" if parts.len() >= 2 => {
                eprintln!(
                "force directive refused at {}: force is a field token, not a standalone directive",
                parts[1]
            );
            }
            "header" if parts.len() >= 3 => {
                cur_headers.push((parts[1].to_string(), parts[2].to_string()));
            }
            "post_body" if parts.len() >= 2 => cur_post_body = Some(parts[1].to_string()),
            "target" if parts.len() >= 2 => cur_target = Some(parts[1].to_string()),
            "catalog" if parts.len() >= 2 => cur_catalog = Some(parts[1].to_string()),
            "max_freq" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    cur_max_freq = Some(v);
                }
            }
            "min_freq" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    cur_min_freq = Some(v);
                }
            }
            "stations" if parts.len() >= 2 => cur_stations_url = Some(parts[1].to_string()),
            "stations_path" if parts.len() >= 2 => cur_stations_path = parts[1].to_string(),
            "stations_lat" if parts.len() >= 2 => cur_stations_lat = parts[1].to_string(),
            "stations_lon" if parts.len() >= 2 => cur_stations_lon = parts[1].to_string(),
            "stations_id" if parts.len() >= 2 => cur_stations_id = parts[1].to_string(),
            "stations_flatten" if parts.len() >= 2 => cur_stations_flatten = parts[1].to_string(),
            "stations_filter" if parts.len() >= 3 => {
                cur_stations_filter = Some((parts[1].to_string(), parts[2].to_string()));
            }
            "fanout_delay" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<u64>() {
                    cur_fanout_delay = v;
                }
            }
            "fanout" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<u32>() {
                    cur_fanout_cap = v;
                }
            }
            "flux_from_mag" if parts.len() >= 2 => cur_flux_from_mag = Some(parts[1].to_string()),
            "abs_mag_from" if parts.len() >= 2 => cur_abs_mag_from = Some(parts[1].to_string()),
            "catalog_epoch" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    cur_catalog_epoch = Some(v);
                }
            }
            "repeat" if parts.len() >= 2 => {
                if parts[1] == "ra" && parts.len() >= 5 {
                    if let Ok(v) = parts[4].parse::<u32>() {
                        cur_repeat_ra_bins = v;
                    }
                } else if let Ok(v) = parts[1].parse::<u32>() {
                    cur_repeat_ra_bins = v;
                }
            }
            _ => {}
        }
    }
    flush!();
    sources
}

#[cfg(feature = "browser_relay")]
pub fn parse_path(s: &str) -> String {
    let Some(fl) = s.lines().next() else {
        return "/".to_string();
    };
    let p: Vec<&str> = fl.split_whitespace().collect();
    if p.len() >= 2 {
        p[1].to_string()
    } else {
        "/".to_string()
    }
}

pub fn parse_iso_tdb(s: &str, lsk: &LeapSeconds) -> Option<f64> {
    let s = s.trim();
    let (date, time) = if let Some((d, t)) = s.split_once('T') {
        (d, t)
    } else if let Some((d, t)) = s.split_once(' ') {
        (d, t)
    } else {
        return None;
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let t = match time
        .split(|c: char| c == '.' || c == 'Z' || c == 'z')
        .next()
    {
        Some(t) => t,
        None => return None,
    };
    let mut tp = t.split(':');
    let hh: u32 = tp.next()?.parse().ok()?;
    let mm: u32 = match tp.next() {
        Some(s) => s,
        None => "0",
    }
    .parse()
    .ok()?;
    let ss: u32 = match tp.next() {
        Some(s) => s,
        None => "0",
    }
    .parse()
    .ok()?;
    let days = ymd_to_days(y, m, d)? as i64;
    let unix = days * 86400 + (hh as i64) * 3600 + (mm as i64) * 60 + ss as i64;
    lsk.unix_to_tdb(unix as f64)
}

pub fn parse_field_config(parts: &[&str]) -> Option<(u8, u8, f64, f64, f64)> {
    let kernel = match kernel_id_of(parts[3]) {
        Some(k) => k,
        None => return None,
    };
    let force = match force_id_of(parts[4]) {
        Some(f) => f,
        None => return None,
    };
    let tau: f64 = match parts[6].parse() {
        Ok(v) if v > 0.0 => v,
        _ => return None,
    };
    let absorption: f64 = match parts[7].parse() {
        Ok(v) => v,
        Err(_) => return None,
    };
    let advection: f64 = match parts[8].parse() {
        Ok(v) => v,
        Err(_) => return None,
    };
    Some((kernel, force, tau, absorption, advection))
}

pub fn parse_where(parts: &[&str]) -> Result<Option<(String, String)>, ()> {
    if parts.len() < 10 || parts[9] != "where" {
        return Ok(None);
    }
    if parts.len() != 12 {
        eprintln!(
            "where refused at {}: the filter clause carries exactly `where <key> <value>`",
            parts.get(1).copied().unwrap_or("?")
        );
        return Err(());
    }
    Ok(Some((parts[10].to_string(), parts[11].to_string())))
}

pub fn load_sources_from(content: &str) -> Vec<SourceConfig> {
    parse_sources(content)
}

pub fn load_all_sources(dir: &str) -> Vec<SourceConfig> {
    let mut sources = Vec::new();
    let dir_path = std::path::Path::new(dir);
    if let Ok(entries) = std::fs::read_dir(dir_path) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                let is_fetch_only = p.file_name().is_some_and(|n| {
                    let n = n.to_string_lossy();
                    n == "research" || n == "port"
                });
                if is_fetch_only {
                    continue;
                }
                let path_str = p.to_string_lossy().to_string();
                sources.extend(load_all_sources(&path_str));
            } else if p.extension().is_some_and(|x| x == "φ") {
                if let Ok(content) = std::fs::read_to_string(&p) {
                    sources.extend(load_sources_from(&content));
                }
            }
        }
    }
    sources
}
