use super::*;
use crate::force::{QuantityKind, quantity_kind_id, quantity_kind_of};
use crate::mathematikerin::channel::{
    ChannelDescriptor, Conserved, FluxKind, Medium, PdeType, QuantityRole, Regime, TransportOp,
    channel_ref_of_descriptor, conserved_for_quantity, conserved_name, descriptor_from_axes,
    unit_token,
};

fn split_directive(line: &str) -> Vec<&str> {
    let bytes = line.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    let len = bytes.len();
    while i < len {
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if bytes[i] == b'"' {
            i += 1;
            let start = i;
            while i < len && bytes[i] != b'"' {
                i += 1;
            }
            tokens.push(&line[start..i]);
            if i < len {
                i += 1;
            }
        } else {
            let start = i;
            while i < len && !bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            tokens.push(&line[start..i]);
        }
    }
    tokens
}

fn push_field(cur_extracts: &mut Vec<Extract>, fc: FieldConfig) {
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
    } else if let Some(Extract::CelestialPolygon { fields, .. }) = cur_extracts.last_mut() {
        fields.push(fc);
    } else if let Some(Extract::KeplerMap { fields, .. }) = cur_extracts.last_mut() {
        fields.push(fc);
    } else if let Some(Extract::ProfileMap { fields, .. }) = cur_extracts.last_mut() {
        fields.push(fc);
    } else if let Some(Extract::EpnCore { fields, .. }) = cur_extracts.last_mut() {
        fields.push(fc);
    } else {
        cur_extracts.push(Extract::Field(fc));
    }
}

pub fn load_sources() -> Vec<SourceConfig> {
    let content = match std::fs::read_to_string("phi/sources.φ") {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    refuse_shard_overlaps(parse_sources(&content))
}

pub fn parse_sources(content: &str) -> Vec<SourceConfig> {
    let mut sources = Vec::new();

    let mut cur_ttl: u64 = 0;
    let mut cur_url = String::new();
    let mut cur_format = String::new();
    let mut cur_extracts: Vec<Extract> = Vec::new();
    let mut cur_channels: Vec<ChannelDescriptor> = Vec::new();
    let mut cur_headers: Vec<(String, String)> = Vec::new();
    let mut cur_target: Option<String> = None;
    let mut cur_catalog: Option<String> = None;
    let mut cur_range: Option<RangeAxis> = None;
    let mut cur_max_freq: Option<f64> = None;
    let mut cur_min_freq: Option<f64> = None;
    let mut cur_body: Option<String> = None;
    let mut cur_post_body: Option<String> = None;
    let mut cur_origin: Option<String> = None;
    let mut cur_terms: Option<String> = None;
    let mut cur_rights_identifier: Option<String> = None;
    let mut cur_rights_scheme: Option<String> = None;
    let mut cur_rights_uri: Option<String> = None;

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
    let mut cur_fanout_center: Option<QueryCenter> = None;
    let mut cur_stations_flatten = String::new();
    let mut cur_stations_filter: Option<(String, String)> = None;
    let mut cur_station_code: Option<String> = None;
    let mut cur_cgm_lat: Option<f64> = None;
    let mut cur_cgm_source: Option<String> = None;
    let mut cur_geomag_lat: Option<f64> = None;
    let mut cur_weberin_role: Option<WeberinRole> = None;
    let mut cur_fanout_delay: u64 = 0;
    let mut cur_frame: Option<Frame> = None;
    let mut cur_sha256: Option<String> = None;
    let mut cur_window: Option<(f64, f64)> = None;
    let mut cur_live_only = false;
    let mut active = false;

    macro_rules! flush {
        () => {
            if active && cur_ttl > 0 && !cur_url.is_empty() {
                if cur_format == "kernel_text"
                    || cur_format == "reference"
                    || cur_frame.is_some()
                    || !cur_extracts.is_empty()
                    || !cur_channels.is_empty()
                {
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
                            channels: std::mem::take(&mut cur_channels),
                            headers: std::mem::take(&mut cur_headers),
                            post_body: cur_post_body.clone(),
                            target: cur_target.clone(),
                            origin: cur_origin.clone(),
                            terms: cur_terms.clone(),
                            rights_identifier: cur_rights_identifier.clone(),
                            rights_scheme: cur_rights_scheme.clone(),
                            rights_uri: cur_rights_uri.clone(),
                            catalog: cur_catalog.clone(),
                            range: cur_range,
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
                            fanout_center: cur_fanout_center,
                            stations_flatten: std::mem::take(&mut cur_stations_flatten),
                            station_code: cur_station_code.clone(),
                            stations_filter: cur_stations_filter.take(),
                            fanout_delay: cur_fanout_delay,
                            sha256: cur_sha256.clone(),
                            window: cur_window,
                            live_only: cur_live_only,
                            cgm_lat: cur_cgm_lat,
                            cgm_source: cur_cgm_source.clone(),
                            geomag_lat: cur_geomag_lat,
                            weberin_role: cur_weberin_role,
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
        let parts: Vec<&str> = split_directive(line);
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "url" if parts.len() >= 2 => {
                flush!();
                cur_url = parts[1].to_string();
                cur_format.clear();
                cur_extracts.clear();
                cur_channels.clear();
                cur_headers.clear();
                cur_ttl = 0;
                cur_target = None;
                cur_catalog = None;
                cur_range = None;
                cur_max_freq = None;
                cur_min_freq = None;
                cur_body = None;
                cur_post_body = None;
                cur_origin = None;
                cur_terms = None;
                cur_rights_identifier = None;
                cur_rights_scheme = None;
                cur_rights_uri = None;
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
                cur_fanout_center = None;
                cur_stations_flatten = String::new();
                cur_stations_filter = None;
                cur_station_code = None;
                cur_cgm_lat = None;
                cur_cgm_source = None;
                cur_geomag_lat = None;
                cur_weberin_role = None;
                cur_fanout_delay = 0;
                cur_frame = None;
                cur_sha256 = None;
                cur_window = None;
                cur_live_only = false;
                active = true;
            }
            "origin" => {
                let rest = line.strip_prefix("origin").unwrap_or("").trim();
                cur_origin = if rest.is_empty() {
                    None
                } else {
                    Some(rest.to_string())
                };
            }
            "terms" => {
                let rest = line.strip_prefix("terms").unwrap_or("").trim();
                cur_terms = if rest.is_empty() {
                    None
                } else {
                    Some(rest.to_string())
                };
            }
            "rights" => {
                cur_rights_identifier = parts.get(1).map(|s| s.to_string());
                cur_rights_scheme = parts.get(2).map(|s| s.to_string());
                cur_rights_uri = parts.get(3).map(|s| s.to_string());
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
            "no-cadence" => {
                cur_ttl = NO_CADENCE;
            }
            "live" => {
                cur_live_only = true;
            }
            "weberin" if parts.len() >= 2 => match WeberinRole::parse(parts[1]) {
                Some(role) => cur_weberin_role = Some(role),
                None => report_anomaly(
                    "Invalid Syntax",
                    &cur_url,
                    &format!("unknown weberin role \"{}\": {}", parts[1], line),
                ),
            },
            "weberin" => {
                report_anomaly(
                    "Invalid Syntax",
                    &cur_url,
                    &format!("weberin needs <role>: {}", line),
                );
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
                    dist_scale: None,
                    plx_key: String::new(),
                    z_key: String::new(),
                    pmra_key: String::new(),
                    pmdec_key: String::new(),
                    rv_key: String::new(),
                    rv_scale: None,
                    epoch_key: String::new(),
                    epoch_mjd: false,
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
            "epncore" if parts.len() >= 2 => {
                cur_extracts.push(Extract::EpnCore {
                    arr_path: parts[1].to_string(),
                    body_key: String::new(),
                    lon_min_key: String::new(),
                    lon_max_key: String::new(),
                    lat_min_key: String::new(),
                    lat_max_key: String::new(),
                    alt_min_key: String::new(),
                    alt_max_key: String::new(),
                    s_region_key: String::new(),
                    epoch_key: String::new(),
                    epoch_mjd: false,
                    val_key: String::new(),
                    fields: Vec::new(),
                });
            }
            "volume" if parts.len() >= 3 => {
                cur_extracts.push(Extract::Volume {
                    value_key: parts[2].to_string(),
                    lat_key: String::new(),
                    lon_key: String::new(),
                    depth_key: String::new(),
                    depth_scale: 1.0,
                    name: parts[1].to_string(),
                    frame_body: cur_frame
                        .as_ref()
                        .map(frame_body_name)
                        .filter(|s| !s.is_empty()),
                });
            }
            "rows" => {
                cur_extracts.push(Extract::Rows {
                    last_line: false,
                    lat_key: String::new(),
                    lon_key: String::new(),
                    fields: Vec::new(),
                    tau_key: String::new(),
                    epoch_cols: Vec::new(),
                    gates: Vec::new(),
                    bin_s: 0,
                    name_prefix: String::new(),
                });
            }
            "prefix" if parts.len() == 2 => {
                let holder = match cur_extracts.last_mut() {
                    Some(Extract::Rows { name_prefix, .. }) => Some(name_prefix),
                    _ => None,
                };
                match holder {
                    Some(p) => *p = parts[1].to_string(),
                    None => {
                        eprintln!("prefix refused at {}: no rows holder", parts[1]);
                    }
                }
            }
            "bin" if parts.len() == 2 => {
                let Some(secs) = parts[1].parse::<u64>().ok() else {
                    continue;
                };
                let holder = match cur_extracts.last_mut() {
                    Some(Extract::Rows { bin_s, .. }) => Some(bin_s),
                    _ => None,
                };
                match holder {
                    Some(bs) => *bs = secs,
                    None => {
                        eprintln!("bin refused at {}: no rows holder", secs);
                    }
                }
            }
            "epoch" if parts.len() == 6 => {
                let holder = match cur_extracts.last_mut() {
                    Some(Extract::Rows { epoch_cols, .. }) => Some(epoch_cols),
                    _ => None,
                };
                match holder {
                    Some(cols) => {
                        *cols = parts[1..6].iter().map(|s| s.to_string()).collect();
                    }
                    None => {
                        eprintln!(
                            "epoch refused at {}: no rows holder",
                            parts.get(1).copied().unwrap_or("?")
                        );
                    }
                }
            }
            "gate" if parts.len() == 4 => {
                let (Some(min), Some(max)) =
                    (parts[2].parse::<f64>().ok(), parts[3].parse::<f64>().ok())
                else {
                    eprintln!(
                        "gate refused at {}: min/max non-numeric",
                        parts.get(1).copied().unwrap_or("?")
                    );
                    continue;
                };
                let holder = match cur_extracts.last_mut() {
                    Some(Extract::Rows { gates, .. }) => Some(gates),
                    _ => None,
                };
                match holder {
                    Some(gs) => gs.push((parts[1].to_string(), min, max)),
                    None => {
                        eprintln!(
                            "gate refused at {}: no rows holder",
                            parts.get(1).copied().unwrap_or("?")
                        );
                    }
                }
            }
            "first" if parts.len() >= 7 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let filter = match parse_where(&parts) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
                };
                cur_extracts.push(Extract::First(fc, filter));
            }
            "last" if parts.len() >= 7 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let filter = match parse_where(&parts) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
                    band_id: None,
                    kernel: 0,
                    force: 0,
                    tau: 0.0,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: String::new(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
                }));
            }
            "lastrow" if parts.len() >= 7 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
            "objlast" if parts.len() >= 7 => {
                let (k, f, tau, absorption, advection) = match parse_field_config(&parts) {
                    Some(v) => v,
                    None => continue,
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
                for p in &parts[3..parts.len().min(5)] {
                    outputs.push(p.to_string());
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
            "quakeml" if parts.len() >= 6 => {
                let tau: f64 = match parts[3].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => continue,
                };
                let absorption: f64 = match parts[4].parse() {
                    Ok(v) => v,
                    _ => continue,
                };
                let advection: f64 = match parts[5].parse() {
                    Ok(v) => v,
                    _ => continue,
                };
                cur_extracts.push(Extract::QuakeMlEvents {
                    outputs: vec![parts[1].to_string(), parts[2].to_string()],
                    tau,
                    absorption,
                    advection,
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
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
                    if let Some((k, v)) = s.split_once('=')
                        && let Ok(fv) = v.parse::<f64>()
                    {
                        cur_hapi_fill.insert(k.to_string(), fv);
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
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[3].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
                        Extract::EpnCore { fields, .. } => Some(fields),
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
                    band_id: None,
                    kernel: match kernel_id_for_force(f) {
                        Some(k) => k,
                        None => continue,
                    },
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit: parts[3].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
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
                        Extract::EpnCore { fields, .. } => Some(fields),
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
            "channel" if parts.len() >= 3 => {
                let unit = match unit_token(parts[2]) {
                    Some(u) => u,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown channel unit \"{}\": {}", parts[2], line),
                        );
                        continue;
                    }
                };
                match ChannelDescriptor::parse_spec(parts[1], unit) {
                    Ok(d) => cur_channels.push(d),
                    Err(reason) => {
                        report_anomaly("Invalid Syntax", &cur_url, &format!("{}: {}", reason, line))
                    }
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
            "quantity" if parts.len() >= 9 => {
                let k = match kernel_id_of(parts[3]) {
                    Some(k) => k,
                    None => continue,
                };
                let kind = match quantity_kind_of(parts[4]) {
                    Some(k) => k,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("unknown quantity kind \"{}\": {}", parts[4], line),
                        );
                        continue;
                    }
                };
                if !matches!(kind, QuantityKind::Geometry | QuantityKind::SourceParameter)
                    && !allowed_units_for_quantity(quantity_kind_id(kind))
                        .contains(&normalize_unit(parts[5]).as_str())
                {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!(
                            "unknown quantity unit \"{}\" for kind \"{}\": {}",
                            parts[5], parts[4], line
                        ),
                    );
                    continue;
                }
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
                let mut freq = crate::spectral::SPECTRAL_NO_BAND;
                let mut bin_width = crate::spectral::SPECTRAL_NO_BAND;
                let mut band_declared = false;
                if parts.len() > 9 {
                    if parts[9] != "band" {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "quantity {} carries an unknown trailing token \"{}\": {}",
                                parts[1], parts[9], line
                            ),
                        );
                        continue;
                    }
                    if parts.len() < 13 || parts[11] != "pivot" {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "quantity {} band clause malformed (band <id> pivot <lambda><unit> [edges <lo>-<hi><unit>]): {}",
                                parts[1], line
                            ),
                        );
                        continue;
                    }
                    let lam_m = match parse_wavelength_m(parts[12]) {
                        Some(m) => m,
                        None => {
                            report_anomaly(
                                "Invalid Syntax",
                                &cur_url,
                                &format!(
                                    "quantity {} pivot wavelength \"{}\" unmeasurable: {}",
                                    parts[1], parts[12], line
                                ),
                            );
                            continue;
                        }
                    };
                    freq = C_LIGHT / lam_m;
                    band_declared = true;
                    if parts.len() > 13 {
                        if parts[13] != "edges" || parts.len() < 15 {
                            report_anomaly(
                                "Invalid Syntax",
                                &cur_url,
                                &format!(
                                    "quantity {} edges clause malformed (edges <lo>-<hi><unit>): {}",
                                    parts[1], line
                                ),
                            );
                            continue;
                        }
                        match parse_wavelength_range_m(parts[14]) {
                            Some((lo_m, hi_m)) => {
                                bin_width = (C_LIGHT / lo_m - C_LIGHT / hi_m).abs();
                            }
                            None => {
                                report_anomaly(
                                    "Invalid Syntax",
                                    &cur_url,
                                    &format!(
                                        "quantity {} edges \"{}\" unmeasurable: {}",
                                        parts[1], parts[14], line
                                    ),
                                );
                                continue;
                            }
                        }
                    }
                }
                if kind == QuantityKind::Scale
                    && normalize_unit(parts[5]) == "nmgy"
                    && !band_declared
                {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!(
                            "photometric flux {} without band: declare band <id> pivot <lambda> [edges <lo>-<hi>]",
                            parts[1]
                        ),
                    );
                    continue;
                }
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: if band_declared {
                        Some(parts[10].to_string())
                    } else {
                        None
                    },
                    kernel: k,
                    force: CHANNEL_REF_QUANTITY,
                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq,
                    bin_width,
                    fold: None,
                    aperture: Aperture::None,
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
                } else if let Some(Extract::EpnCore { fields, .. }) = cur_extracts.last_mut() {
                    fields.push(fc);
                } else {
                    cur_extracts.push(Extract::Field(fc.clone()));
                }
            }
            "field"
                if parts.len() >= 12
                    && QuantityRole::parse(parts[3]).is_some()
                    && Conserved::parse(parts[4]).is_some() =>
            {
                let (i, declared_regime) = match Regime::parse(parts[9]) {
                    Some(r) => (10, Some(r)),
                    None => (9, None),
                };
                if parts.len() < i + 3 {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!(
                            "field {} carries no kernel/unit/tau after the boundary (a regime token shifts the kernel by one)",
                            parts[1]
                        ),
                    );
                    continue;
                }
                let k = match kernel_id_of(parts[i]) {
                    Some(k) => k,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} carries \"{}\" where a kernel (or a regime token) is expected",
                                parts[1], parts[i]
                            ),
                        );
                        continue;
                    }
                };
                let unit = match unit_token(parts[i + 1]) {
                    Some(u) => u,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} carries an unknown unit \"{}\": {}",
                                parts[1],
                                parts[i + 1],
                                line
                            ),
                        );
                        continue;
                    }
                };
                let tau: f64 = match parts[i + 2].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => {
                        eprintln!(
                            "field refused at {}: tau absent or not positive (τ-Gate)",
                            parts[1]
                        );
                        continue;
                    }
                };
                let absorption: f64 = match parts.get(i + 3) {
                    Some(s) => match s.parse() {
                        Ok(v) => v,
                        Err(_) => continue,
                    },
                    None => 0.0,
                };
                let advection: f64 = match parts.get(i + 4) {
                    Some(s) => match s.parse() {
                        Ok(v) => v,
                        Err(_) => continue,
                    },
                    None => 0.0,
                };
                let desc = match descriptor_from_axes(
                    parts[3], parts[4], parts[5], parts[6], parts[7], parts[8], unit,
                ) {
                    Ok(d) => d,
                    Err(reason) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("field {}: {}", parts[1], reason),
                        );
                        continue;
                    }
                };
                if let Some(declared) = declared_regime
                    && declared != desc.regime
                {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!(
                            "field {} declares regime {} but the descriptor carries {} — a contradiction, not a default",
                            parts[1],
                            declared.name(),
                            desc.regime.name()
                        ),
                    );
                    continue;
                }
                let f = match channel_ref_of_descriptor(&desc) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} descriptor ({}/{}/{}/{}/{}/{}) matches no channel — unresolved",
                                parts[1],
                                parts[3],
                                parts[4],
                                parts[5],
                                parts[6],
                                parts[7],
                                parts[8]
                            ),
                        );
                        continue;
                    }
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[i + 1].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
                };
                push_field(&mut cur_extracts, fc);
            }
            "field"
                if parts.len() >= 10
                    && TransportOp::parse(parts[3]).is_some()
                    && PdeType::parse(parts[4]).is_some()
                    && Medium::parse(parts[5]).is_some() =>
            {
                let op = match TransportOp::parse(parts[3]) {
                    Some(op) => op,
                    None => continue,
                };
                let conserved = match op {
                    TransportOp::Flux(FluxKind::Fick) => "mass",
                    TransportOp::Flux(FluxKind::Fourier) => "energy",
                    TransportOp::Flux(FluxKind::Ohm) => "charge",
                    TransportOp::Flux(FluxKind::NewtonViscous) => "momentum",
                    TransportOp::Advective
                    | TransportOp::Wave
                    | TransportOp::Maxwell
                    | TransportOp::Poisson => match conserved_for_quantity(parts[2]) {
                        Some(c) => conserved_name(c),
                        None => {
                            report_anomaly(
                                "Invalid Syntax",
                                &cur_url,
                                &format!(
                                    "field {}: quantity \"{}\" resolves to no conserved quantity for operator \"{}\" — pending, never a default",
                                    parts[1], parts[2], parts[3]
                                ),
                            );
                            continue;
                        }
                    },
                };
                let (role, role_idx) = match interaction_or_role(&parts, 6) {
                    Ok(resolved) => resolved,
                    Err(reason) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("field {}: {}", parts[1], reason),
                        );
                        continue;
                    }
                };
                let unit_idx = role_idx + 1;
                let tau_idx = role_idx + 2;
                if parts.len() < tau_idx + 2 {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!(
                            "field {} carries no unit/tau/kernel after the role",
                            parts[1]
                        ),
                    );
                    continue;
                }
                let k = match kernel_id_of(parts[parts.len() - 1]) {
                    Some(k) => k,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} carries \"{}\" where a kernel is expected",
                                parts[1],
                                parts[parts.len() - 1]
                            ),
                        );
                        continue;
                    }
                };
                let unit = match unit_token(parts[unit_idx]) {
                    Some(u) => u,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} carries an unknown unit \"{}\": {}",
                                parts[1], parts[unit_idx], line
                            ),
                        );
                        continue;
                    }
                };
                let tau: f64 = match parts[tau_idx].parse() {
                    Ok(v) if v > 0.0 => v,
                    _ => {
                        eprintln!(
                            "field refused at {}: tau absent or not positive (τ-Gate)",
                            parts[1]
                        );
                        continue;
                    }
                };
                let tail = &parts[tau_idx + 1..];
                let (absorption, advection) = match tail.len() {
                    1 => (0.0, 0.0),
                    2 => match tail[0].parse() {
                        Ok(v) => (v, 0.0),
                        Err(_) => continue,
                    },
                    3 => match (tail[0].parse(), tail[1].parse()) {
                        (Ok(a), Ok(b)) => (a, b),
                        _ => continue,
                    },
                    _ => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} carries {} trailing tokens after tau where [abs] [adv] <kernel> is expected",
                                parts[1],
                                tail.len()
                            ),
                        );
                        continue;
                    }
                };
                let desc = match descriptor_from_axes(
                    role, conserved, parts[3], parts[4], parts[5], "none", unit,
                ) {
                    Ok(d) => d,
                    Err(reason) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("field {}: {}", parts[1], reason),
                        );
                        continue;
                    }
                };
                let f = match channel_ref_of_descriptor(&desc) {
                    Some(f) => f,
                    None => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} descriptor ({}/{}/{}/{}/{}/{}) matches no channel — unresolved",
                                parts[1], role, conserved, parts[3], parts[4], parts[5], unit
                            ),
                        );
                        continue;
                    }
                };
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption,
                    advection,
                    unit: parts[unit_idx].to_string(),
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: None,
                    aperture: Aperture::None,
                };
                push_field(&mut cur_extracts, fc);
            }
            "field" if parts.len() >= 7 => {
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
                let mut freq = crate::spectral::SPECTRAL_NO_BAND;
                let mut bin_width = crate::spectral::SPECTRAL_NO_BAND;
                let aperture_idx = parts.iter().position(|p| p.starts_with("aperture:"));
                let declared_aperture = match aperture_idx {
                    Some(i) => {
                        match Aperture::of_class(parts[i].strip_prefix("aperture:").unwrap_or("")) {
                            Some(a) => Some(a),
                            None => {
                                report_anomaly(
                                    "Invalid Syntax",
                                    &cur_url,
                                    &format!("unknown aperture class \"{}\": {}", parts[i], line),
                                );
                                continue;
                            }
                        }
                    }
                    None => None,
                };
                let block_has_z = matches!(
                    cur_extracts.last(),
                    Some(Extract::CelestialMap { z_key, .. }) if !z_key.is_empty()
                );
                let aperture_mandatory = block_has_z && f == 0;
                let aperture = match (declared_aperture, aperture_mandatory) {
                    (Some(a), _) => a,
                    (None, false) => Aperture::None,
                    (None, true) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!(
                                "field {} carries no aperture token in a z-block (mandatory)",
                                parts[1]
                            ),
                        );
                        continue;
                    }
                };
                let extras: Vec<f64> = parts
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i >= 9 && Some(*i) != aperture_idx)
                    .filter_map(|(_, s)| s.parse::<f64>().ok())
                    .filter(|v| v.is_finite() && *v > 0.0)
                    .collect();
                if let Some(v) = extras.first() {
                    freq = *v;
                }
                if let Some(v) = extras.get(1) {
                    bin_width = *v;
                }
                let fc = FieldConfig {
                    key: parts[1].to_string(),
                    name: parts[2].to_string(),
                    band_id: None,
                    kernel: k,
                    force: f,

                    tau,
                    absorption,
                    advection,
                    unit: parts[5].to_string(),
                    freq,
                    bin_width,
                    fold: None,
                    aperture,
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
                } else if let Some(Extract::EpnCore { fields, .. }) = cur_extracts.last_mut() {
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
                Some(Extract::Map { lat_key, .. })
                | Some(Extract::ProfileMap { lat_key, .. })
                | Some(Extract::Rows { lat_key, .. })
                | Some(Extract::Volume { lat_key, .. }) => {
                    *lat_key = parts[1].to_string();
                }
                _ => {}
            },
            "lon" if parts.len() >= 2 => match cur_extracts.last_mut() {
                Some(Extract::Map { lon_key, .. })
                | Some(Extract::ProfileMap { lon_key, .. })
                | Some(Extract::Rows { lon_key, .. })
                | Some(Extract::Volume { lon_key, .. }) => {
                    *lon_key = parts[1].to_string();
                }
                _ => {}
            },
            "depth" if parts.len() >= 2 => {
                if let Some(Extract::Volume {
                    depth_key,
                    depth_scale,
                    ..
                }) = cur_extracts.last_mut()
                {
                    *depth_key = parts[1].to_string();
                    if parts.len() >= 3
                        && let Ok(s) = parts[2].parse::<f64>()
                    {
                        *depth_scale = s;
                    }
                }
            }
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
                if let Ok(s) = parts[1].parse::<f64>()
                    && let Some(Extract::Map { epoch_scale, .. }) = cur_extracts.last_mut()
                {
                    *epoch_scale = s;
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
                Some(Extract::Rows { epoch_cols, .. }) => {
                    *epoch_cols = vec![parts[1].to_string()];
                }
                Some(Extract::CelestialMap {
                    epoch_key,
                    epoch_mjd,
                    ..
                }) => {
                    *epoch_key = parts[1].to_string();
                    if parts.len() >= 3 && parts[2] == "mjd" {
                        *epoch_mjd = true;
                    }
                }
                Some(Extract::EpnCore {
                    epoch_key,
                    epoch_mjd,
                    ..
                }) => {
                    *epoch_key = parts[1].to_string();
                    if parts.len() >= 3 && parts[2] == "mjd" {
                        *epoch_mjd = true;
                    }
                }
                _ => {}
            },
            "target_name" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { body_key, .. }) = cur_extracts.last_mut() {
                    *body_key = parts[1].to_string();
                }
            }
            "lonmin" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { lon_min_key, .. }) = cur_extracts.last_mut() {
                    *lon_min_key = parts[1].to_string();
                }
            }
            "lonmax" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { lon_max_key, .. }) = cur_extracts.last_mut() {
                    *lon_max_key = parts[1].to_string();
                }
            }
            "latmin" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { lat_min_key, .. }) = cur_extracts.last_mut() {
                    *lat_min_key = parts[1].to_string();
                }
            }
            "latmax" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { lat_max_key, .. }) = cur_extracts.last_mut() {
                    *lat_max_key = parts[1].to_string();
                }
            }
            "altmin" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { alt_min_key, .. }) = cur_extracts.last_mut() {
                    *alt_min_key = parts[1].to_string();
                }
            }
            "altmax" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { alt_max_key, .. }) = cur_extracts.last_mut() {
                    *alt_max_key = parts[1].to_string();
                }
            }
            "region" if parts.len() >= 2 => {
                if let Some(Extract::EpnCore { s_region_key, .. }) = cur_extracts.last_mut() {
                    *s_region_key = parts[1].to_string();
                }
            }
            "pressure" if parts.len() >= 2 => {
                if let Some(Extract::ProfileMap {
                    pressure_var,
                    pressure_scale,
                    ..
                }) = cur_extracts.last_mut()
                {
                    *pressure_var = parts[1].to_string();
                    if parts.len() >= 3
                        && let Ok(s) = parts[2].parse::<f64>()
                    {
                        *pressure_scale = s;
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
            "fold" if parts.len() == 8 || parts.len() == 7 => {
                let op = match parts[1] {
                    "mean" => 1u8,
                    "diff" => 2,
                    "sum" => 3,
                    "sin_deg" => 4,
                    "cos_deg" => 5,
                    other => {
                        eprintln!(
                            "fold refused: op \"{}\" unknown (mean|diff|sum|sin_deg|cos_deg)",
                            other
                        );
                        continue;
                    }
                };
                let (k, f, tau, unit) = if parts.len() == 8 {
                    let k = match kernel_id_of(parts[4]) {
                        Some(k) => k,
                        None => continue,
                    };
                    let f = match force_id_of(parts[5]) {
                        Some(f) => f,
                        None => continue,
                    };
                    let tau: f64 = match parts[7].parse() {
                        Ok(v) if v > 0.0 => v,
                        _ => continue,
                    };
                    (k, f, tau, parts[6].to_string())
                } else {
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
                    (k, f, tau, parts[5].to_string())
                };
                let fc = FieldConfig {
                    key: parts[2].to_string(),
                    name: format!("fold_{}_{}_{}", parts[1], parts[2], parts[3]),
                    band_id: None,
                    kernel: k,
                    force: f,
                    tau,
                    absorption: 0.0,
                    advection: 0.0,
                    unit,
                    freq: crate::spectral::SPECTRAL_NO_BAND,
                    bin_width: crate::spectral::SPECTRAL_NO_BAND,
                    fold: Some((op, parts[3].to_string())),
                    aperture: Aperture::None,
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
                } else if let Some(Extract::EpnCore { val_key, .. }) = cur_extracts.last_mut() {
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
                if let Ok(v) = parts[1].parse::<f64>()
                    && let Some(Extract::CelestialMap { dist_scale, .. }) = cur_extracts.last_mut()
                {
                    *dist_scale = Some(v);
                }
            }
            "rv_scale" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>()
                    && let Some(Extract::CelestialMap { rv_scale, .. }) = cur_extracts.last_mut()
                {
                    *rv_scale = Some(v);
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
            "sha256" if parts.len() >= 2 => cur_sha256 = Some(parts[1].to_string()),
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
            "range" if parts.len() == 3 => {
                let start = parts[1].parse::<f64>();
                let step = parts[2].parse::<f64>();
                match (start, step) {
                    (Ok(s), Ok(d)) if s.is_finite() && s >= 0.0 && d.is_finite() && d > 0.0 => {
                        cur_range = Some(RangeAxis {
                            start_m: s,
                            step_m: d,
                        });
                    }
                    _ => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("range void: {}", line),
                        );
                    }
                }
            }
            "range" => {
                report_anomaly(
                    "Invalid Syntax",
                    &cur_url,
                    &format!("range arity {}: {}", parts.len(), line),
                );
            }
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
            "station" if parts.len() >= 2 => cur_station_code = Some(parts[1].to_string()),
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
            "fanout_center" if parts.len() >= 2 => {
                cur_fanout_center = match parts[1] {
                    "receiver" => Some(QueryCenter::Receiver),
                    "anchor" => Some(QueryCenter::Anchor),
                    other => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("fanout_center unknown arm: {}", other),
                        );
                        None
                    }
                };
            }
            "flux_from_mag" if parts.len() >= 2 => cur_flux_from_mag = Some(parts[1].to_string()),
            "abs_mag_from" if parts.len() >= 2 => cur_abs_mag_from = Some(parts[1].to_string()),
            "catalog_epoch" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    cur_catalog_epoch = Some(v);
                }
            }
            "cgm_lat" if parts.len() >= 2 => {
                let v: f64 = match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("cgm_lat non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                cur_cgm_lat = Some(v);
            }
            "cgm_source" if parts.len() >= 2 => {
                cur_cgm_source = Some(parts[1].to_string());
            }
            "geomag_lat" if parts.len() >= 2 => {
                let v: f64 = match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("geomag_lat non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                cur_geomag_lat = Some(v);
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
            "window" if parts.len() >= 3 => {
                let from: f64 = match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("window from non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                let until: f64 = match parts[2].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        report_anomaly(
                            "Invalid Syntax",
                            &cur_url,
                            &format!("window until non-numeric: {}", line),
                        );
                        continue;
                    }
                };
                if from <= until {
                    cur_window = Some((from, until));
                } else {
                    report_anomaly(
                        "Invalid Syntax",
                        &cur_url,
                        &format!("window from > until: {}", line),
                    );
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
    } else if let Some((d, t)) = s.split_once('/') {
        (d, t)
    } else {
        (s, "0")
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let t = time.split(['.', 'Z', 'z']).next()?;
    let mut tp = t.split(':');
    let hh: u32 = tp.next()?.parse().ok()?;
    let mm: u32 = tp.next().unwrap_or("0").parse().ok()?;
    let ss: u32 = tp.next().unwrap_or("0").parse().ok()?;
    let days = ymd_to_days(y, m, d)? as i64;
    let unix = days * 86400 + (hh as i64) * 3600 + (mm as i64) * 60 + ss as i64;
    lsk.unix_to_tdb(unix as f64)
}

pub fn parse_field_config(parts: &[&str]) -> Option<(u8, u8, f64, f64, f64)> {
    let kernel = kernel_id_of(parts[3])?;
    let force = match force_id_of(parts[4]) {
        Some(f) => f,
        None => match quantity_kind_of(parts[4]) {
            Some(QuantityKind::Geometry | QuantityKind::SourceParameter) => CHANNEL_REF_QUANTITY,
            _ => return None,
        },
    };
    let tau: f64 = match parts[6].parse() {
        Ok(v) if v > 0.0 => v,
        _ => return None,
    };
    let absorption: f64 = if parts.len() > 7 {
        match parts[7].parse() {
            Ok(v) => v,
            Err(_) => return None,
        }
    } else {
        SLOT_ABSENT
    };
    let advection: f64 = if parts.len() > 8 {
        match parts[8].parse() {
            Ok(v) => v,
            Err(_) => return None,
        }
    } else {
        SLOT_ABSENT
    };
    Some((kernel, force, tau, absorption, advection))
}

pub struct WhereRefused;

pub fn parse_where(parts: &[&str]) -> Result<Option<(String, String)>, WhereRefused> {
    if parts.len() < 10 || parts[9] != "where" {
        return Ok(None);
    }
    if parts.len() != 12 {
        eprintln!(
            "where refused at {}: the filter clause carries exactly `where <key> <value>`",
            parts.get(1).copied().unwrap_or("?")
        );
        return Err(WhereRefused);
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
            } else if p.extension().is_some_and(|x| x == "φ")
                && let Ok(content) = std::fs::read_to_string(&p)
            {
                sources.extend(load_sources_from(&content));
            }
        }
    }
    sources
}

fn shard_range_of(src: &SourceConfig) -> Option<(f64, f64)> {
    let filename = src.url.rsplit('/').next()?;
    let (name, lo, hi) = odf::parse_podf_shard_name(filename)?;
    if name != src.format.as_str() {
        return None;
    }
    Some((lo, hi))
}

pub fn refuse_shard_overlaps(sources: Vec<SourceConfig>) -> Vec<SourceConfig> {
    let mut accepted: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    let mut kept = Vec::with_capacity(sources.len());
    for src in sources {
        match shard_range_of(&src) {
            None => kept.push(src),
            Some((lo, hi)) => {
                if lo.partial_cmp(&hi) != Some(std::cmp::Ordering::Less) {
                    eprintln!(
                        "source refused: shard TDB range [{}, {}) is not half-open at {}",
                        lo, hi, src.url
                    );
                    continue;
                }
                let ranges = accepted.entry(src.format.clone()).or_default();
                let overlaps = ranges.iter().any(|&(alo, ahi)| lo < ahi && alo < hi);
                if overlaps {
                    eprintln!(
                        "source refused: shard TDB range [{}, {}) overlaps an accepted range at {} — never merged",
                        lo, hi, src.url
                    );
                    continue;
                }
                ranges.push((lo, hi));
                kept.push(src);
            }
        }
    }
    kept
}

fn wavelength_value_and_scale(token: &str) -> Option<(f64, f64)> {
    let (value, factor) = if let Some(v) = token.strip_suffix("angstrom") {
        (v, 1e-10)
    } else if let Some(v) = token.strip_suffix("nm") {
        (v, 1e-9)
    } else {
        let v = token.strip_suffix("um")?;
        (v, 1e-6)
    };
    let value: f64 = value.parse().ok()?;
    Some((value, factor))
}

fn parse_wavelength_m(token: &str) -> Option<f64> {
    let (value, factor) = wavelength_value_and_scale(token)?;
    let m = value * factor;
    if m.is_finite() && m > 0.0 {
        Some(m)
    } else {
        None
    }
}

fn parse_wavelength_range_m(token: &str) -> Option<(f64, f64)> {
    let (lo_str, hi_str) = token.split_once('-')?;
    let (hi_value, factor) = wavelength_value_and_scale(hi_str)?;
    let lo_value: f64 = lo_str.parse().ok()?;
    let lo_m = lo_value * factor;
    let hi_m = hi_value * factor;
    if lo_m.is_finite() && hi_m.is_finite() && lo_m > 0.0 && lo_m < hi_m {
        Some((lo_m, hi_m))
    } else {
        None
    }
}

fn interaction_or_role<'a>(parts: &[&'a str], idx: usize) -> Result<(&'a str, usize), String> {
    if QuantityRole::parse(parts[idx]).is_some() {
        return Ok((parts[idx], idx));
    }
    match parts[idx] {
        "gravity" | "em" => match parts.get(idx + 1) {
            Some(role) if QuantityRole::parse(*role).is_some() => Ok((*role, idx + 1)),
            Some(other) => Err(format!(
                "carries \"{}\" where a role (primary/derived/geometry/source-parameter) is expected after the interaction token",
                other
            )),
            None => Err(format!(
                "carries interaction \"{}\" with no role after it",
                parts[idx]
            )),
        },
        other => Err(format!(
            "carries \"{}\" where an interaction (gravity/em) or a role is expected",
            other
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shard(url: &str, format: &str) -> SourceConfig {
        SourceConfig {
            ttl: 604800,
            url: url.into(),
            origin: None,
            terms: None,
            rights_identifier: None,
            rights_scheme: None,
            rights_uri: None,
            frame: Frame::Manifest,
            format: format.into(),
            channels: Vec::new(),
            extracts: Vec::new(),
            headers: Vec::new(),
            post_body: None,
            target: None,
            catalog: None,
            range: None,
            max_freq: None,
            min_freq: None,
            body: None,
            stations_url: None,
            stations_path: String::new(),
            stations_lat: String::new(),
            stations_lon: String::new(),
            stations_id: String::new(),
            hapi_fill: HashMap::new(),
            flux_from_mag: None,
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            fanout_center: None,
            stations_flatten: String::new(),
            station_code: None,
            stations_filter: None,
            fanout_delay: 0,
            sha256: None,
            window: None,
            live_only: false,
            cgm_lat: None,
            cgm_source: None,
            geomag_lat: None,
            weberin_role: None,
        }
    }

    const URL_A: &str = "https://cdn.example/x/odyssey_odf_t700000000_800000000.bin";
    const URL_B: &str = "https://cdn.example/x/odyssey_odf_t800000000_900000000.bin";
    const URL_C: &str = "https://cdn.example/x/odyssey_odf_t900000000_1000000000.bin";

    #[test]
    fn a_descriptor_field_line_resolves_to_its_registered_channel() {
        let content = "url https://example.com/x\nttl 600\nat sun\nfield probe probe primary energy flux-fourier parabolic fluid none exponential-decay K 60.0 0.0 0.0\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        let field = sources[0]
            .extracts
            .iter()
            .find_map(|e| match e {
                Extract::Field(fc) => Some(fc),
                _ => None,
            })
            .expect("the descriptor field is admitted");
        assert_eq!(field.force, 5, "Fourier/energy/fluid resolves to thermal");
        assert_eq!(field.unit, "K");
    }

    #[test]
    fn a_descriptor_field_line_without_a_registered_channel_is_refused() {
        let content = "url https://example.com/x\nttl 600\nat sun\nfield probe probe primary momentum flux-newton-viscous mixed fluid none patch-levy m/s 60.0 0.0 0.0\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        assert!(
            sources[0]
                .extracts
                .iter()
                .all(|e| !matches!(e, Extract::Field(_))),
            "no registered channel → the field is refused, never a default"
        );
    }

    #[test]
    fn p10_2a_target_grammar_parses() {
        let content = "url https://example.com/x\nttl 600\nat sun\nfield bz_gsm bz maxwell elliptic vacuum em primary V/m 60.0 inverse-square\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        let field = sources[0]
            .extracts
            .iter()
            .find_map(|e| match e {
                Extract::Field(fc) => Some(fc),
                _ => None,
            })
            .expect("the P10.2a target line is admitted");
        assert_eq!(field.kernel, 0, "inverse-square is kernel 0");
        assert_eq!(
            field.force, 8,
            "maxwell/elliptic/vacuum resolves to electric"
        );
        assert_eq!(field.unit, "V/m");

        let invalid = "url https://example.com/x\nttl 600\nat sun\nfield bz_gsm bz bogus elliptic vacuum em primary V/m 60.0 inverse-square\n";
        let skipped = parse_sources(invalid);
        assert!(
            skipped[0]
                .extracts
                .iter()
                .all(|e| !matches!(e, Extract::Field(_))),
            "an unknown operator is skipped, never defaulted"
        );

        let unresolved = "url https://example.com/x\nttl 600\nat sun\nfield x xyzzy maxwell elliptic vacuum em primary V/m 60.0 inverse-square\n";
        let pending = parse_sources(unresolved);
        assert!(
            pending[0]
                .extracts
                .iter()
                .all(|e| !matches!(e, Extract::Field(_))),
            "a quantity that resolves to no conserved quantity is pending, never defaulted"
        );
    }

    #[test]
    fn p10_2a_interaction_absent_form_parses() {
        let content = "url https://example.com/x\nttl 600\nat sun\nfield bz_gsm bz maxwell elliptic vacuum primary V/m 60.0 inverse-square\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        let field = sources[0]
            .extracts
            .iter()
            .find_map(|e| match e {
                Extract::Field(fc) => Some(fc),
                _ => None,
            })
            .expect("the optional interaction may be absent — the line is admitted, never silently dropped");
        assert_eq!(
            field.force, 8,
            "the interaction-absent form resolves to the same channel as the interaction-present form"
        );
        assert_eq!(field.unit, "V/m");
    }

    #[test]
    fn p10_2a_bogus_interaction_refused() {
        let content = "url https://example.com/x\nttl 600\nat sun\nfield bz_gsm bz maxwell elliptic vacuum bogus primary V/m 60.0 inverse-square\n";
        let sources = parse_sources(content);
        assert!(
            sources[0]
                .extracts
                .iter()
                .all(|e| !matches!(e, Extract::Field(_))),
            "a token that is neither an interaction (gravity/em) nor a role is refused, never defaulted"
        );
    }

    #[test]
    fn volume_frame_body_declared_and_refused_when_absent() {
        let declared = "url https://cdn.example/x/AFRP.volume.bin\nformat volume\nat earth\nttl 604800\nvolume v v\n";
        let with = parse_sources(declared);
        let body = with[0].extracts.iter().find_map(|e| match e {
            Extract::Volume { frame_body, .. } => Some(frame_body.clone()),
            _ => None,
        });
        assert_eq!(body, Some(Some("earth".to_string())));

        let absent =
            "url https://cdn.example/x/x.volume.bin\nformat volume\nttl 604800\nvolume v v\n";
        let without = parse_sources(absent);
        let body2 = without[0].extracts.iter().find_map(|e| match e {
            Extract::Volume { frame_body, .. } => Some(frame_body.clone()),
            _ => None,
        });
        assert_eq!(body2, Some(None), "no `at` → absent, never a default body");
    }

    #[test]
    fn cgm_lat_directive_is_carried() {
        let declared =
            "url https://x/y.bin\nformat volume\nat earth\nttl 604800\nvolume v v\ncgm_lat 40.5\n";
        let with = parse_sources(declared);
        assert_eq!(with[0].cgm_lat, Some(40.5));
        assert_eq!(with[0].cgm_source, None);
    }

    #[test]
    fn cgm_source_directive_is_carried_and_orthogonal_to_cgm_lat() {
        let declared = "url https://x/y.bin\nformat volume\nat earth\nttl 604800\nvolume v v\ncgm_lat 11.23\ncgm_source bgs-quasi-dipole\n";
        let with = parse_sources(declared);
        assert_eq!(with[0].cgm_lat, Some(11.23));
        assert_eq!(with[0].cgm_source.as_deref(), Some("bgs-quasi-dipole"));
    }

    #[test]
    fn rights_triple_is_carried_and_absent_without_directive() {
        let declared = "url https://x/y.bin\nformat volume\nat earth\nttl 604800\nvolume v v\nrights CC-BY-4.0 SPDX https://spdx.org/licenses/CC-BY-4.0.html\n";
        let with = parse_sources(declared);
        assert_eq!(with[0].rights_identifier.as_deref(), Some("CC-BY-4.0"));
        assert_eq!(with[0].rights_scheme.as_deref(), Some("SPDX"));
        assert_eq!(
            with[0].rights_uri.as_deref(),
            Some("https://spdx.org/licenses/CC-BY-4.0.html")
        );

        let absent =
            parse_sources("url https://x/y.bin\nformat volume\nat earth\nttl 604800\nvolume v v\n");
        assert_eq!(absent[0].rights_identifier, None);
        assert_eq!(absent[0].rights_scheme, None);
        assert_eq!(absent[0].rights_uri, None);

        let partial = parse_sources(
            "url https://x/y.bin\nformat volume\nat earth\nttl 604800\nvolume v v\nrights NONE\n",
        );
        assert_eq!(partial[0].rights_identifier.as_deref(), Some("NONE"));
        assert_eq!(partial[0].rights_scheme, None);
        assert_eq!(partial[0].rights_uri, None);
    }

    #[test]
    fn shard_overlap_is_refused() {
        let kept = refuse_shard_overlaps(vec![
            shard(URL_A, "odyssey_odf"),
            shard(
                "https://cdn.example/x/odyssey_odf_t750000000_850000000.bin",
                "odyssey_odf",
            ),
        ]);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].url, URL_A);
    }

    #[test]
    fn shard_disjoint_is_accepted() {
        let kept = refuse_shard_overlaps(vec![
            shard(URL_A, "odyssey_odf"),
            shard(URL_C, "odyssey_odf"),
        ]);
        assert_eq!(kept.len(), 2);
    }

    #[test]
    fn no_cadence_keeps_a_static_source_active() {
        let content =
            "url https://example.com/static.bin\nformat ephemeris_binary\nat moon\nno-cadence\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].ttl, NO_CADENCE);
    }

    #[test]
    fn a_source_without_ttl_or_no_cadence_is_inactive() {
        let content = "url https://example.com/static.bin\nformat ephemeris_binary\nat moon\n";
        assert!(parse_sources(content).is_empty());
    }

    #[test]
    fn range_directive_records_start_and_step_metres() {
        let content =
            "url https://example.com/gate.bin\nformat gras_2c\nrange 300.0 0.15\nttl 604800\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        assert_eq!(
            sources[0].range,
            Some(RangeAxis {
                start_m: 300.0,
                step_m: 0.15,
            })
        );
    }

    #[test]
    fn range_directive_with_zero_or_negative_step_stays_unrecorded() {
        let zero = "url https://example.com/gate.bin\nformat gras_2c\nrange 300.0 0\nttl 604800\n";
        let negative =
            "url https://example.com/gate.bin\nformat gras_2c\nrange 300.0 -0.15\nttl 604800\n";
        assert_eq!(parse_sources(zero)[0].range, None);
        assert_eq!(parse_sources(negative)[0].range, None);
    }

    #[test]
    fn range_directive_with_void_arity_stays_unrecorded() {
        let content = "url https://example.com/gate.bin\nformat gras_2c\nrange 300.0\nttl 604800\n";
        assert_eq!(parse_sources(content)[0].range, None);
    }

    #[test]
    fn range_directive_resets_at_the_next_url() {
        let content = "url https://example.com/a.bin\nformat gras_2c\nrange 300.0 0.15\nat mars\n\
                       ttl 604800\nurl https://example.com/b.bin\nformat gras_2c\nat mars\n\
                       ttl 604800\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 2);
        assert!(sources[0].range.is_some());
        assert_eq!(sources[1].range, None);
    }

    #[test]
    fn shard_touching_boundaries_are_accepted() {
        let kept = refuse_shard_overlaps(vec![
            shard(URL_A, "odyssey_odf"),
            shard(URL_B, "odyssey_odf"),
        ]);
        assert_eq!(kept.len(), 2);
    }

    #[test]
    fn shard_identical_ranges_are_refused() {
        let kept = refuse_shard_overlaps(vec![
            shard(URL_A, "odyssey_odf"),
            shard(
                "https://cdn.example/x/odyssey_odf_t700000000_800000000_1.bin",
                "odyssey_odf",
            ),
        ]);
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn shard_not_half_open_is_refused() {
        let kept = refuse_shard_overlaps(vec![shard(
            "https://cdn.example/x/odyssey_odf_t800000000_800000000.bin",
            "odyssey_odf",
        )]);
        assert_eq!(kept.len(), 0);
    }

    #[test]
    fn shard_of_a_different_name_is_not_compared() {
        let kept = refuse_shard_overlaps(vec![
            shard(URL_A, "odyssey_odf"),
            shard(
                "https://cdn.example/x/mro_odf_t700000000_800000000.bin",
                "mro_odf",
            ),
        ]);
        assert_eq!(kept.len(), 2);
    }

    #[test]
    fn a_non_shard_url_does_not_trigger_the_gate() {
        let kept = refuse_shard_overlaps(vec![
            shard("https://cdn.example/x/mro_odf.bin", "mro_odf"),
            shard(URL_A, "odyssey_odf"),
        ]);
        assert_eq!(kept.len(), 2);
    }

    #[test]
    fn every_cdn_source_carries_its_origin_and_compiler_or_is_a_reference() {
        let content = std::fs::read_to_string("phi/sources.φ").expect("phi/sources.φ reads");
        let mut missing: Vec<String> = Vec::new();
        let mut url = String::new();
        let mut cdn = false;
        let mut origin = false;
        let mut compiler = false;
        let mut reference = false;
        for line in content.lines() {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("url") => {
                    if cdn && !(origin && (compiler || reference)) {
                        missing.push(url.clone());
                    }
                    url = words.next().map_or(String::new(), str::to_string);
                    cdn = url.contains("/releases/download/");
                    origin = false;
                    compiler = false;
                    reference = false;
                }
                Some("origin") => origin = true,
                Some("compiler") => compiler = true,
                Some("format") => reference = words.next() == Some("reference"),
                _ => {}
            }
        }
        if cdn && !(origin && (compiler || reference)) {
            missing.push(url);
        }
        assert!(
            missing.is_empty(),
            "CDN sources without origin/compiler: {missing:?}"
        );
    }

    #[test]
    fn field_without_absorption_advection_declares_absent_not_zero() {
        let short = split_directive("field x y inverse-square em nT 3600");
        let (_, _, tau, absorption, advection) = parse_field_config(&short).unwrap();
        assert_eq!(tau, 3600.0);
        assert_eq!(
            absorption, SLOT_ABSENT,
            "absent absorption must not read as 0.0"
        );
        assert_eq!(
            advection, SLOT_ABSENT,
            "absent advection must not read as 0.0"
        );

        let full = split_directive("field x y inverse-square em nT 3600 0.0 0.0");
        let (_, _, _, absorption, advection) = parse_field_config(&full).unwrap();
        assert_eq!(absorption, 0.0, "a declared 0.0 stays the measured value");
        assert_eq!(advection, 0.0, "a declared 0.0 stays the measured value");
    }

    #[test]
    fn presence_flags_carry_the_measured_bits_and_pad_the_slots() {
        assert_eq!(
            presence_flags(None, SLOT_ABSENT, SLOT_ABSENT, SLOT_ABSENT),
            0.0
        );
        assert_eq!(
            presence_flags(Some(0.0), SLOT_ABSENT, SLOT_ABSENT, SLOT_ABSENT),
            PRESENCE_FLAG_PHASE
        );
        assert_eq!(
            presence_flags(Some(1.5), 0.25, SLOT_ABSENT, SLOT_ABSENT),
            PRESENCE_FLAG_PHASE + PRESENCE_FLAG_ABSORPTION
        );
        assert_eq!(
            presence_flags(Some(1.5), 0.25, 3.0, SLOT_ABSENT),
            PRESENCE_FLAG_PHASE + PRESENCE_FLAG_ABSORPTION + PRESENCE_FLAG_ADVECTION
        );
        assert_eq!(slot_or_pad(SLOT_ABSENT), 0.0);
        assert_eq!(slot_or_pad(0.0), 0.0, "a measured zero stays the value");
        assert_eq!(slot_or_pad(0.25), 0.25);
        assert_eq!(slot_or_pad(f64::NAN), 0.0, "NaN never crosses the wire");
        assert!(!slot_measured(SLOT_ABSENT));
        assert!(slot_measured(0.0), "a measured zero is a measurement");
    }

    #[test]
    fn field_and_quantity_are_disjoint_taxonomies() {
        let field_of = |content: &str| -> Option<u8> {
            parse_sources(content).first().and_then(|s| {
                s.extracts.iter().find_map(|e| match e {
                    Extract::Field(fc) => Some(fc.force),
                    _ => None,
                })
            })
        };

        let valid = "url https://example.com/q.bin\nttl 604800\n\
                     quantity qkey qname inverse-square mass kg 3600 0.0 0.0\n";
        assert_eq!(field_of(valid), Some(CHANNEL_REF_QUANTITY));

        let force_as_kind = "url https://example.com/q.bin\nttl 604800\n\
                             quantity qkey qname inverse-square em kg 3600 0.0 0.0\n";
        assert_eq!(
            field_of(force_as_kind),
            None,
            "quantity with a force name in the kind slot must not become a field"
        );

        let kind_as_force = "url https://example.com/q.bin\nttl 604800\n\
                             field qkey qname inverse-square mass kg 3600 0.0 0.0\n";
        assert_eq!(
            field_of(kind_as_force),
            None,
            "field with a quantity kind in the force slot must not become a field"
        );
    }

    #[test]
    fn a_last_selector_migrates_a_geometry_value_out_of_the_force_taxonomy() {
        let last_of = |content: &str| -> Option<FieldConfig> {
            parse_sources(content).first().and_then(|s| {
                s.extracts.iter().find_map(|e| match e {
                    Extract::Last(fc, _) => Some(fc.clone()),
                    _ => None,
                })
            })
        };
        let geometry = "url https://example.com/g.bin\nttl 604800\n\
                        last items.value river_stage inverse-square geometry m 900.0 0.0 0.0\n";
        let fc = last_of(geometry).expect("a last selector with a geometry kind flows");
        assert_eq!(
            fc.force, CHANNEL_REF_QUANTITY,
            "a geometry value leaves Σω through the last arm, like the quantity arm"
        );
        assert_eq!(fc.unit, "m", "the geometry unit stays free");

        let source_parameter = "url https://example.com/g.bin\nttl 604800\n\
                                last mass planet_mass inverse-square source-parameter M_earth 60 0.0 0.0\n";
        assert_eq!(
            last_of(source_parameter).map(|f| f.force),
            Some(CHANNEL_REF_QUANTITY),
            "a source parameter leaves Σω through the last arm"
        );

        let still_a_force = "url https://example.com/g.bin\nttl 604800\n\
                             last data.v tide_height inverse-square gravity m 3600 0.0 0.0\n";
        assert_eq!(
            last_of(still_a_force).map(|f| f.force),
            Some(1),
            "a force token in the same slot keeps its channel"
        );
    }

    #[test]
    fn the_field_regime_token_is_resolved_or_refused_never_defaulted() {
        let force_of = |content: &str| -> Option<u8> {
            parse_sources(content).first().and_then(|s| {
                s.extracts.iter().find_map(|e| match e {
                    Extract::Field(fc) => Some(fc.force),
                    _ => None,
                })
            })
        };
        let bare = "url https://example.com/f.bin\nttl 604800\n\
                    field fkey fname primary energy maxwell mixed vacuum none inverse-square V/m 60 0.0 0.0\n";
        assert_eq!(
            force_of(bare),
            Some(0),
            "the regime is derived from the axes when no token is declared"
        );

        let declared = "url https://example.com/f.bin\nttl 604800\n\
                        field fkey fname primary energy maxwell mixed vacuum none radiating inverse-square V/m 60 0.0 0.0\n";
        assert_eq!(
            force_of(declared),
            Some(0),
            "a declared regime that matches the descriptor resolves"
        );

        let contradicted = "url https://example.com/f.bin\nttl 604800\n\
                            field fkey fname primary energy maxwell mixed vacuum none quasi-static inverse-square V/m 60 0.0 0.0\n";
        assert_eq!(
            force_of(contradicted),
            None,
            "a declared regime contradicting the descriptor is refused, never defaulted"
        );

        let electric = "url https://example.com/f.bin\nttl 604800\n\
                        field fkey fname primary energy maxwell elliptic vacuum none quasi-static inverse-square V/m 60 0.0 0.0\n";
        assert_eq!(
            force_of(electric),
            Some(8),
            "the quasi-static regime selects the electric channel"
        );
    }

    #[test]
    fn derived_magnetic_index_is_a_quantity_not_a_force_field() {
        let field_of = |content: &str| -> Option<FieldConfig> {
            parse_sources(content).first().and_then(|s| {
                s.extracts.iter().find_map(|e| match e {
                    Extract::Field(fc) => Some(fc.clone()),
                    _ => None,
                })
            })
        };
        let q = "url https://example.com/q.bin\nttl 604800\n\
                 quantity qkey qname inverse-square index nt 60 0.0 0.0\n";
        let fc = field_of(q).expect("a magnetic index quantity in nT flows as a quantity");
        assert_eq!(
            fc.force, CHANNEL_REF_QUANTITY,
            "a derived index stays outside Σω"
        );
    }

    #[test]
    fn a_geometry_and_source_parameter_quantity_flow_without_a_unit_gate() {
        let field_of = |content: &str| -> Option<FieldConfig> {
            parse_sources(content).first().and_then(|s| {
                s.extracts.iter().find_map(|e| match e {
                    Extract::Field(fc) => Some(fc.clone()),
                    _ => None,
                })
            })
        };
        let geometry = "url https://example.com/q.bin\nttl 604800\n\
                        quantity tide_ft tide_ft inverse-square geometry ft 3600 0.0 0.0\n";
        let fc = field_of(geometry).expect("a geometry quantity flows");
        assert_eq!(fc.force, CHANNEL_REF_QUANTITY);
        assert_eq!(
            fc.unit, "ft",
            "the geometry unit is free (length, angle, mass …)"
        );

        let source_parameter = "url https://example.com/q.bin\nttl 604800\n\
                                quantity planet_mass planet_mass inverse-square source-parameter M_earth 60 0.0 0.0\n";
        assert_eq!(
            field_of(source_parameter).map(|f| f.force),
            Some(CHANNEL_REF_QUANTITY),
            "a source parameter flows as a quantity, never a field"
        );
    }

    #[test]
    fn photometric_flux_without_band_is_refused_and_with_band_carries_the_band() {
        let field_of = |content: &str| -> Option<FieldConfig> {
            parse_sources(content).first().and_then(|s| {
                s.extracts.iter().find_map(|e| match e {
                    Extract::Field(fc) => Some(fc.clone()),
                    _ => None,
                })
            })
        };

        let bandless = "url https://example.com/q.bin\nttl 604800\n\
                        quantity flux_g g_flux inverse-square scale nmgy 31536000 0.0 0.0\n";
        assert!(
            field_of(bandless).is_none(),
            "a photometric scale nmgy quantity without a band reference is refused, never a 0.0 band"
        );

        let with_band = "url https://example.com/q.bin\nttl 604800\n\
                         quantity flux_g g_flux inverse-square scale nmgy 31536000 0.0 0.0 band DECam_g pivot 4808.49angstrom edges 3900-5600angstrom\n";
        let fc = field_of(with_band).expect("the band-declared photometric quantity flows");
        assert!(
            fc.freq > 0.0,
            "the pivot wavelength resolves to a positive frequency"
        );
        assert!(
            fc.bin_width > 0.0,
            "the edges resolve to a positive bandwidth"
        );
        assert_eq!(
            fc.band_id.as_deref(),
            Some("DECam_g"),
            "the band id persists on the parsed field"
        );
    }

    #[test]
    fn a_channel_directive_carries_the_descriptor() {
        let content = "url https://example.com/x\nttl 3600\n\
                       channel mass:primary:poisson:elliptic:vacuum:unspecified:none m/s^2\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].channels.len(), 1);
        let expected = crate::mathematikerin::channel::descriptor_for_force(
            "gravity",
            crate::mathematikerin::channel::Medium::Vacuum,
        )
        .expect("gravity descriptor");
        assert_eq!(sources[0].channels[0].hash(), expected.hash());
    }

    #[test]
    fn a_channel_directive_carries_the_geometry_extent() {
        let content = "url https://example.com/x\nttl 3600\n\
                       channel energy:primary:wave:hyperbolic:fluid:circle:dirichlet:2.0 Pa\n";
        let sources = parse_sources(content);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].channels.len(), 1);
        let d = &sources[0].channels[0];
        assert_eq!(d.extent, Some(2.0));
        assert!(d.mode_wavenumbers(3).is_some(), "geometry carries modes");
    }

    #[test]
    fn a_channel_directive_with_an_unknown_unit_is_refused() {
        let content = "url https://example.com/x\nttl 3600\n\
                       channel mass:primary:poisson:elliptic:vacuum:unspecified:none furlongs\n";
        let sources = parse_sources(content);
        assert!(
            sources
                .first()
                .map(|s| s.channels.is_empty())
                .unwrap_or(true)
        );
    }
}
