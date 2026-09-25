use super::*;

pub fn angular_distance_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r1 = lat1.to_radians();
    let r2 = lat2.to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = ((r2 - r1) * 0.5).sin().powi(2) + r1.cos() * r2.cos() * (dlon * 0.5).sin().powi(2);
    (2.0 * a.sqrt().asin()).to_degrees()
}

pub struct PortMeasure {
    pub hapi: bool,
    pub unit: Option<String>,
    pub tau: Option<f64>,
}

pub fn port_field_synth(
    directive: &str,
    force: &str,
    key: &str,
    name: &str,
    unit: Option<&str>,
    tau: Option<f64>,
) -> Option<String> {
    let (kernel, f) = default_kernel_for(force)?;
    let unit = unit?;
    let tau = match tau {
        Some(t) if t.is_finite() && t > 0.0 => t,
        _ => return None,
    };
    Some(format!(
        "{} {} {} {} {} {} {} 0.0 0.0\n",
        directive, key, name, kernel, f, unit, tau
    ))
}

fn port_non_oscillator(name: &str) -> bool {
    let kl = name.to_lowercase();
    is_drop_key(name)
        || kl.ends_with("_id")
        || kl.ends_with("_name")
        || kl.ends_with("_title")
        || kl.ends_with("_doi")
        || kl.ends_with("_url")
        || kl.ends_with("_author")
        || kl.ends_with("_description")
        || kl.ends_with("_date")
        || kl.ends_with("_rate")
        || kl.ends_with("_total")
        || kl.ends_with("_station")
}

fn hapi_field_synth(
    directive: &str,
    force: &str,
    key: &str,
    name: &str,
    measure: &PortMeasure,
    fallback_tau: f64,
) -> Option<String> {
    let unit = match measure.unit.as_deref() {
        Some(u) if !u.contains(char::is_whitespace) => u,
        Some(u) => {
            return Some(format!(
                "# pending {} {} — unit not representable (multi-token {}), review\n",
                directive, name, u
            ));
        }
        None => match unit_from_name_suffix(name) {
            Some(u) => u,
            None => {
                return Some(format!(
                    "# pending {} {} — unit absent, review\n",
                    directive, name
                ));
            }
        },
    };
    let tau = match measure.tau {
        Some(t) if t.is_finite() && t > 0.0 => t,
        _ => {
            if fallback_tau.is_finite() && fallback_tau > 0.0 {
                fallback_tau
            } else {
                return Some(format!(
                    "# pending {} {} — cadence absent, review\n",
                    directive, name
                ));
            }
        }
    };
    port_field_synth(directive, force, key, name, Some(unit), Some(tau))
}

fn field_or_review(
    directive: &str,
    force: &str,
    key: &str,
    name: &str,
    measure: &PortMeasure,
) -> Option<String> {
    let (classified, cunit, ctau) = probe_classify(name);
    if classified == "DROP" || (classified == "UNCERTAIN" && port_non_oscillator(name)) {
        return Some(format!(
            "# declined {} {} — not an oscillator (no physical force)\n",
            directive, name
        ));
    }
    if measure.hapi {
        return match classified {
            "UNCERTAIN" => Some(format!(
                "# pending {} {} — force undetermined, review\n",
                directive, name
            )),
            other => hapi_field_synth(directive, other, key, name, measure, ctau),
        };
    }
    let block_force = if default_kernel_for(force).is_some() {
        Some(force)
    } else {
        None
    };
    let absent_line = Some(format!(
        "# pending {} {} — unit or cadence absent, review\n",
        directive, name
    ));
    match block_force {
        None => match classified {
            "UNCERTAIN" => Some(format!(
                "# pending {} {} — force undetermined, review\n",
                directive, name
            )),
            other => port_field_synth(directive, other, key, name, Some(cunit), Some(ctau))
                .or(absent_line),
        },
        Some(f) => port_field_synth(directive, f, key, name, None, None).or(absent_line),
    }
}

pub fn port_block(block: &str) -> String {
    let mechanical = PortMeasure {
        hapi: false,
        unit: None,
        tau: None,
    };
    port_block_measured(block, &mechanical)
}

pub fn port_block_measured(block: &str, measure: &PortMeasure) -> String {
    let mut head: Vec<String> = Vec::new();
    let mut force = String::new();
    let mut ttl: u64 = 0;
    let mut frame_line: Option<String> = None;
    let mut lat: Option<f64> = None;
    let mut lon: Option<f64> = None;
    let mut alt: Option<f64> = None;
    let mut map_line: Option<String> = None;
    let mut lat_key: Option<String> = None;
    let mut lon_key: Option<String> = None;
    let mut ra_key: Option<String> = None;
    let mut dec_key: Option<String> = None;
    let mut plx_key: Option<String> = None;
    let mut z_key: Option<String> = None;
    let mut dist_key: Option<String> = None;
    let mut dist_scale: Option<String> = None;
    let mut pmra_key: Option<String> = None;
    let mut pmdec_key: Option<String> = None;
    let mut radvel_key: Option<String> = None;
    let mut rv_scale: Option<String> = None;
    let mut tau_key: Option<String> = None;
    let mut vel_key: Option<String> = None;
    let mut trk_key: Option<String> = None;
    let mut vr_key: Option<String> = None;
    let mut alt_key: Option<String> = None;
    let mut epoch_key: Option<String> = None;
    let mut post_body: Option<String> = None;
    let mut method_post = false;
    let mut body_target: Option<String> = None;
    let mut raw_extracts: Vec<String> = Vec::new();
    for line in block.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = t.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "url" | "format" | "header" | "target" | "catalog" | "flux_from_mag"
            | "abs_mag_from" | "catalog_epoch" | "max_freq" | "min_freq" => {
                head.push(t.to_string());
            }
            "ttl" => {
                if let Ok(v) = parts[1].parse::<u64>() {
                    ttl = v;
                }
            }
            "on" | "at" => frame_line = Some(t.to_string()),
            "lat" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    lat = Some(v);
                }
            }
            "lon" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    lon = Some(v);
                }
            }
            "alt" if parts.len() >= 2 => {
                if let Ok(v) = parts[1].parse::<f64>() {
                    alt = Some(v);
                }
            }
            "force" if parts.len() >= 2 => force = parts[1].to_string(),
            "method" if parts.len() >= 2 => {
                method_post = parts[1].eq_ignore_ascii_case("post");
            }
            "body" if parts.len() >= 2 => {
                if parts[1].starts_with('{') || method_post {
                    post_body = Some(parts[1].to_string());
                } else {
                    body_target = Some(parts[1].to_string());
                }
            }
            "map" | "cmap" | "rows" | "flatten" => {
                let arg = parts.get(1).copied().unwrap_or(".");
                map_line = Some(format!("{} {}", parts[0], arg));
            }
            "lat_key" if parts.len() >= 2 => lat_key = Some(parts[1].to_string()),
            "lon_key" if parts.len() >= 2 => lon_key = Some(parts[1].to_string()),
            "ra_key" if parts.len() >= 2 => ra_key = Some(parts[1].to_string()),
            "dec_key" if parts.len() >= 2 => dec_key = Some(parts[1].to_string()),
            "plx_key" if parts.len() >= 2 => plx_key = Some(parts[1].to_string()),
            "z_key" if parts.len() >= 2 => z_key = Some(parts[1].to_string()),
            "dist_key" if parts.len() >= 2 => dist_key = Some(parts[1].to_string()),
            "dist_scale" if parts.len() >= 2 => dist_scale = Some(parts[1].to_string()),
            "pmra_key" if parts.len() >= 2 => pmra_key = Some(parts[1].to_string()),
            "pmdec_key" if parts.len() >= 2 => pmdec_key = Some(parts[1].to_string()),
            "radvel_key" if parts.len() >= 2 => radvel_key = Some(parts[1].to_string()),
            "rv_scale" if parts.len() >= 2 => rv_scale = Some(parts[1].to_string()),
            "tau_key" if parts.len() >= 2 => tau_key = Some(parts[1].to_string()),
            "vel_key" if parts.len() >= 2 => vel_key = Some(parts[1].to_string()),
            "trk_key" if parts.len() >= 2 => trk_key = Some(parts[1].to_string()),
            "vr_key" if parts.len() >= 2 => vr_key = Some(parts[1].to_string()),
            "alt_key" if parts.len() >= 2 => alt_key = Some(parts[1].to_string()),
            "epoch_key" if parts.len() >= 2 => epoch_key = Some(parts[1].to_string()),
            "field" | "field_in" | "first" | "last" | "count" | "path" | "deep" | "last_row"
            | "last_line" | "last_obj" | "geojson" | "regex" => {
                raw_extracts.push(t.to_string());
            }
            _ => {}
        }
    }

    let celestial = (ra_key.is_some() && dec_key.is_some())
        || matches!(
            (lat_key.as_deref(), lon_key.as_deref()),
            (Some(k1), Some(k2))
                if (k1.eq_ignore_ascii_case("ra") || k1.eq_ignore_ascii_case("s_ra"))
                    && (k2.eq_ignore_ascii_case("dec") || k2.eq_ignore_ascii_case("s_dec"))
        );
    let named_keys = lat_key
        .as_deref()
        .is_some_and(|k| k.parse::<f64>().is_err())
        && lon_key
            .as_deref()
            .is_some_and(|k| k.parse::<f64>().is_err());

    let mut out = String::new();
    for h in head.iter().filter(|h| h.starts_with("url ")) {
        out.push_str(h);
        out.push('\n');
    }
    if ttl > 0 {
        out.push_str(&format!("ttl {}\n", ttl));
    }
    for h in head.iter().filter(|h| !h.starts_with("url ")) {
        out.push_str(h);
        out.push('\n');
    }
    if let Some(b) = &post_body {
        out.push_str("post_body ");
        out.push_str(b);
        out.push('\n');
    }
    if let Some(b) = &body_target {
        out.push_str("body ");
        out.push_str(b);
        out.push('\n');
    }
    if let Some(f) = &frame_line {
        out.push_str(f);
        out.push('\n');
    } else if celestial && map_line.is_some() {
        out.push_str("at sun\n");
    } else if named_keys && map_line.is_some() {
        out.push_str("on earth 0 0 0\n");
    } else if let (Some(lat), Some(lon)) = (lat, lon) {
        match alt {
            Some(a) => out.push_str(&format!("on earth {} {} {}\n", lat, lon, a)),
            None => eprintln!(
                "port: block refused 'on' — lat/lon without alt (declare alt); the alt-less frame is not representable"
            ),
        }
    }
    if let Some(m) = &map_line {
        if celestial {
            let arg = m.split_once(' ').map(|x| x.1).unwrap_or(".");
            out.push_str("cmap ");
            out.push_str(arg);
            out.push('\n');
        } else {
            out.push_str(m);
            out.push('\n');
        }
    }
    if celestial {
        if let Some(k) = ra_key.as_deref().or(lat_key.as_deref()) {
            out.push_str("ra ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = dec_key.as_deref().or(lon_key.as_deref()) {
            out.push_str("dec ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &plx_key {
            out.push_str("plx ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &z_key {
            out.push_str("z ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &pmra_key {
            out.push_str("pmra ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &pmdec_key {
            out.push_str("pmdec ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &radvel_key {
            out.push_str("radvel ");
            out.push_str(k);
            out.push('\n');
            if let Some(s) = &rv_scale {
                out.push_str("rv_scale ");
                out.push_str(s);
                out.push('\n');
            }
        }
        if let Some(k) = &dist_key {
            if let Some(plx) = k.strip_prefix("1000/") {
                if plx_key.is_none() {
                    out.push_str("plx ");
                    out.push_str(plx);
                    out.push('\n');
                }
            } else if let Some(s) = &dist_scale {
                out.push_str("dist ");
                out.push_str(k);
                out.push('\n');
                out.push_str("dist_scale ");
                out.push_str(s);
                out.push('\n');
            }
        }
    } else {
        if let Some(k) = &lat_key {
            out.push_str("lat ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &lon_key {
            out.push_str("lon ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &alt_key {
            out.push_str("alt ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &epoch_key {
            out.push_str("epoch ");
            out.push_str(k);
            out.push('\n');
        }
    }
    if let Some(k) = &tau_key {
        out.push_str("tau_key ");
        out.push_str(k);
        out.push('\n');
    }
    if let Some(k) = &vel_key {
        out.push_str("vel ");
        out.push_str(k);
        out.push('\n');
    }
    if let Some(k) = &trk_key {
        out.push_str("trk ");
        out.push_str(k);
        out.push('\n');
    }
    if let Some(k) = &vr_key {
        out.push_str("vr ");
        out.push_str(k);
        out.push('\n');
    }
    for r in &raw_extracts {
        let parts: Vec<&str> = r.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let s = match parts[0] {
            "field" | "field_in" if parts.len() >= 3 => {
                field_or_review("field", &force, parts[1], parts[2], measure)
            }
            "count" if parts.len() >= 3 => Some(format!("count {} {}", parts[1], parts[2])),
            "first" | "last" | "path" | "deep" if parts.len() >= 3 => {
                field_or_review(parts[0], &force, parts[1], parts[2], measure)
            }
            "last_row" if parts.len() >= 3 => {
                field_or_review("lastrow", &force, parts[1], parts[2], measure)
            }
            "last_line" if parts.len() >= 2 => Some(format!("lastline {}", parts[1])),
            "last_obj" if parts.len() >= 5 => {
                let name = parts[parts.len() - 1];
                let key = parts[parts.len() - 2];
                let parent = parts[1];
                let m = parts[2..parts.len() - 2].join(" ");
                Some(format!("lastobj {} {} {} {}", parent, m, key, name))
            }
            "geojson" if parts.len() >= 5 => None,
            "regex" if parts.len() >= 3 => {
                field_or_review("regex", &force, parts[1], parts[2], measure)
            }
            _ => None,
        };
        if let Some(s) = s {
            out.push_str(s.trim_end());
            out.push('\n');
        }
    }
    out
}

pub fn flush_port_block(
    block: &str,
    converted: &mut String,
    total: &mut usize,
    parsed: &mut usize,
    pending: &mut usize,
    declined: &mut usize,
    measure: &PortMeasure,
) {
    *total += 1;
    let conv = port_block_measured(block, measure);
    if conv.contains("# pending ") {
        *pending += 1;
        converted.push_str(&conv);
        converted.push('\n');
        return;
    }
    let srcs = parse_sources(&conv);
    if !srcs.is_empty() {
        *parsed += 1;
        converted.push_str(&conv);
        converted.push('\n');
        return;
    }
    if conv.contains("# declined ") {
        *declined += 1;
        converted.push_str(&conv);
        converted.push('\n');
        return;
    }
    let synthesized_extract = conv.lines().any(|l| {
        let p: Vec<&str> = l.split_whitespace().collect();
        matches!(
            p.first().copied(),
            Some("field" | "path" | "deep" | "first" | "last" | "lastrow" | "count" | "regex")
        )
    });
    if synthesized_extract {
        *pending += 1;
        converted.push_str(&conv);
        converted.push_str("# pending block — refused at parse (ttl/frame gate), review\n");
        converted.push('\n');
    }
}

fn port_measure_for(block: &str, env: &HashMap<String, String>) -> PortMeasure {
    let url = block.lines().find_map(|l| {
        let t = l.trim_start();
        let rest = t.strip_prefix("url ")?;
        let u = rest.split_whitespace().next().unwrap_or("");
        if u.is_empty() {
            None
        } else {
            Some(u.to_string())
        }
    });
    match url {
        Some(u) if u.contains("/hapi/") => hapi_live_measure(&u, env),
        _ => PortMeasure {
            hapi: false,
            unit: None,
            tau: None,
        },
    }
}

fn hapi_live_measure(url: &str, env: &HashMap<String, String>) -> PortMeasure {
    let void = PortMeasure {
        hapi: true,
        unit: None,
        tau: None,
    };
    let Some(info_url) = hapi_cadence_url(url) else {
        return void;
    };
    let info_url = resolve_secret(&info_url, env);
    let Some(body) = fetch_raw_probe(&info_url, None, &[]) else {
        return void;
    };
    let Some(parsed) = parse_json(&body) else {
        return void;
    };
    let Some(meta) = hapi_meta_params(&parsed) else {
        return void;
    };
    let tau = find_cadence_seconds(&parsed).map(|s| s as f64);
    let order = hapi_request_order(url, &meta);
    let mut units: Vec<&str> = order
        .iter()
        .filter_map(|req| meta.iter().find(|m| m.name == *req))
        .filter(|m| !is_time_key(&m.name))
        .filter_map(|m| m.unit.as_deref())
        .filter(|u| !u.trim().is_empty())
        .collect();
    units.sort_unstable();
    units.dedup();
    let unit = match units.as_slice() {
        [u] => Some(u.to_string()),
        _ => None,
    };
    PortMeasure {
        hapi: true,
        unit,
        tau,
    }
}

pub fn port_mode(input: &str, output: &str, env: &HashMap<String, String>) -> i32 {
    let content = match std::fs::read_to_string(input) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("--port: input unreadable: {}", input);
            return 1;
        }
    };
    let mut converted =
        String::from("# port conversion (source grammar → canonical grammar, mechanical)\n");
    let mut block = String::new();
    let mut total = 0usize;
    let mut parsed = 0usize;
    let mut pending = 0usize;
    let mut declined = 0usize;
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("url ") {
            if !block.is_empty() {
                flush_port_block(
                    &block,
                    &mut converted,
                    &mut total,
                    &mut parsed,
                    &mut pending,
                    &mut declined,
                    &port_measure_for(&block, env),
                );
                block = String::new();
            }
            block.push_str(line);
            block.push('\n');
            continue;
        }
        block.push_str(line);
        block.push('\n');
    }
    if !block.is_empty() {
        flush_port_block(
            &block,
            &mut converted,
            &mut total,
            &mut parsed,
            &mut pending,
            &mut declined,
            &port_measure_for(&block, env),
        );
    }
    if std::fs::write(output, &converted).is_err() {
        eprintln!("--port: output unwritable: {}", output);
        return 1;
    }
    eprintln!(
        "--port: {} blocks converted, {} parse in the current parser, {} pending review, {} declined → {}",
        total, parsed, pending, declined, output
    );
    0
}

pub struct ProbeParams<'a> {
    pub now: f64,
    pub lsk_ref: &'a LeapSeconds,
    pub void_eph: &'a HashMap<String, BodyEphemeris>,
    pub env: &'a HashMap<String, String>,
    pub fetchone: bool,
    pub precise: bool,
    pub lat: f64,
    pub lon: f64,
}

pub fn probe_one(src: &SourceConfig, params: ProbeParams<'_>) -> (bool, String) {
    let url = match render_url(
        &src.url,
        "",
        RenderCtx {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tdb: params.now,
            r: 0.0,
            eph: params.void_eph,
            lsk: params.lsk_ref,
        },
    ) {
        Some(u) => u,
        None => return (false, "# declined: time absent\n".to_string()),
    }
    .replace("{lat}", &format!("{:.6}", params.lat))
    .replace("{lon}", &format!("{:.6}", params.lon));
    let mut url = url;
    for (k, v) in live_markers() {
        url = url.replace(&k, &v);
    }
    let url = resolve_secret(&url, params.env);
    let headers = render_headers(&src.headers, params.env);
    let raw = if params.fetchone {
        fetch_one(&url, None, &headers, src.ttl, Some(params.now))
    } else {
        fetch_raw_probe(&url, None, &headers)
    };
    let parsed = raw.as_ref().and_then(|r| parse_json(r));
    let auto_ttl = raw.as_ref().and_then(|r| derive_ttl(&url, r, params.env));
    let mut block = String::new();
    block.push_str(&format!("url {}\n", src.url));
    let ttl = match auto_ttl {
        Some(t) => t,
        None => src.ttl,
    };
    block.push_str(&format!("ttl {}\n", ttl));
    match &src.frame {
        Frame::Surface {
            body_name,
            lat,
            lon,
            alt,
        } => {
            block.push_str(&format!("on {} {:?} {:?} {:?}\n", body_name, lat, lon, alt));
        }
        Frame::Barycenter { body_name, scale } if *scale == 1.0 => {
            block.push_str(&format!("at {}\n", body_name));
        }
        Frame::Barycenter { body_name, scale } => {
            block.push_str(&format!("at {} {}\n", body_name, scale));
        }
        Frame::Manifest => {}
    }
    if let Some(p) = parsed {
        let mut fields = String::new();
        let mut coords = String::new();
        let mut map_path: Option<String> = None;
        let mut budget = 48usize;
        if !hapi_draft_fields(&url, &p, params.env, &mut fields) {
            walk_json_probe(&p, "", &mut fields, &mut coords, &mut map_path, &mut budget);
        }
        if map_path.is_none() && !coords.is_empty() {
            map_path = Some(".".to_string());
        }
        let precision_lines = measure_precision(&p);
        if !precision_lines.is_empty() {
            block.push_str(&precision_lines);
        }
        if let Some(ref mp) = map_path
            && !coords.is_empty()
        {
            let container = if coords.contains("ra ") || coords.contains("dec ") {
                "cmap"
            } else {
                "map"
            };
            block.push_str(&format!("{} {}\n", container, mp));
        }
        if !coords.is_empty() {
            block.push_str(&coords);
        }
        if !fields.is_empty() {
            block.push_str(&fields);
        }
    } else if let Some(ref r) = raw {
        if let Some(csv) = probe_csv(r) {
            block.push_str("format free text\n");
            block.push_str(&csv);
        }
    } else {
        block.push_str("# fetch returned void\n");
    }
    if params.precise && raw.is_some() {
        block.push_str(&bruteforce_precision(&url, &src.url, ttl));
    }
    let verdict = match &raw {
        Some(r) => {
            let declared_ok = match extract(src, r, params.now, params.lsk_ref) {
                ExtractResult::Measurements(v) | ExtractResult::WithEphemeris(v, _) => {
                    if v.is_empty() { None } else { Some(v.len()) }
                }
            };
            match declared_ok {
                Some(n) => Ok(n),
                None => match parse_sources(&block).first() {
                    Some(candidate) => match extract(candidate, r, params.now, params.lsk_ref) {
                        ExtractResult::Measurements(v) | ExtractResult::WithEphemeris(v, _) => {
                            if v.is_empty() {
                                Err(diagnose_no_samples(candidate, r))
                            } else {
                                Ok(v.len())
                            }
                        }
                    },
                    None => Err("block refused at parse (frame/ttl/field gate)".into()),
                },
            }
        }
        None => Err("fetch returned void".into()),
    };
    match verdict {
        Ok(n) => {
            let mut b = format!("# verified {} samples\n", n);
            b.push_str(&block);
            b.push('\n');
            (true, b)
        }
        Err(why) => {
            let mut b = format!("# declined: {}\n", why);
            b.push_str(&block);
            b.push('\n');
            (false, b)
        }
    }
}

pub fn reverify_mode(env: &HashMap<String, String>) -> i32 {
    let Some(lsk) = embedded_lsk() else {
        eprintln!("reverify: the time base is absent — no sweep without a clock");
        return 1;
    };
    let Some(now) = lsk.system_now_tdb() else {
        eprintln!("reverify: TDB absent — no sweep without a clock");
        return 1;
    };
    let (ok, findings) = live_sweep(env, now, &lsk, 600);
    eprintln!(
        "\n=== REVERIFY: {} ok, {} void (of {} tested) ===",
        ok,
        findings.len(),
        ok + findings.len()
    );
    let clock = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs());
    let dash = clock.map(|u| format!("{} — ", date_str(u)));
    let mut lines: Vec<String> = vec![
        match &dash {
            Some(d) => format!(
                "# recheck-live {}mechanical re-verification sweep over phi/sources.φ (live sources)",
                d
            ),
            None => "# recheck-live mechanical re-verification sweep over phi/sources.φ (live sources)"
                .into(),
        },
        "# Classes: key-void (key marker without .secrets.local) | drift-void (API drift — curation duty) | quiet-void (empty = truth) | refused (host answers but refuses body — alive, not dead) | broken (fetch void — host unreachable, dead candidate)".into(),
    ];
    for f in findings.iter() {
        let line = format!("recheck {} {} — {}", f.url, f.class.as_str(), f.detail);
        println!("{}", line);
        lines.push(line);
    }
    if findings.is_empty() {
        let colon = clock.map(|u| format!("{}: ", date_str(u)));
        lines.push(match &colon {
            Some(c) => format!(
                "recheck-live {}0 findings — all {} tested sources harvested samples",
                c, ok
            ),
            None => format!(
                "recheck-live 0 findings — all {} tested sources harvested samples",
                ok
            ),
        });
    }
    match std::fs::write("phi/pipeline/stage/recheck_live.φ", lines.join("\n") + "\n") {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("reverify: write phi/pipeline/stage/recheck_live.φ: {}", e);
            1
        }
    }
}

pub fn probe_mode(
    path: &str,
    precise: bool,
    lat: f64,
    lon: f64,
    env: &HashMap<String, String>,
    fetchone: bool,
) -> i32 {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("probe: read {}: {}", path, e);
            return 1;
        }
    };
    let sources = load_sources_from(&content);
    eprintln!(
        "probe: {} source blocks loaded from {}",
        sources.len(),
        path
    );
    let mut lsk: Option<LeapSeconds> = None;
    for src in sources.iter().filter(|s| s.format == "kernel_text") {
        if src.body.as_deref() != Some("naif0012") {
            continue;
        }
        if let Some(text) = fetch_one(&src.url, None, &[], src.ttl, machine_now_tdb()) {
            lsk = crate::lsk::parse(&text);
        }
    }
    if lsk.is_none()
        && let Some(text) = fetch_one(
            "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls",
            None,
            &[],
            NAIF_LSK_TTL_SECS,
            machine_now_tdb(),
        )
    {
        lsk = crate::lsk::parse(&text);
    }
    let time_pair: Option<(f64, LeapSeconds)> = match lsk {
        Some(l) => l.system_now_tdb().map(|t| (t, l)),
        None => None,
    };
    let void_eph: HashMap<String, BodyEphemeris> = HashMap::new();
    let accepted = std::sync::atomic::AtomicUsize::new(0);
    let declined = std::sync::atomic::AtomicUsize::new(0);
    let out_lock = std::sync::Mutex::new(String::new());
    let dead_lock = std::sync::Mutex::new(String::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    let non_kernel: Vec<&SourceConfig> = sources
        .iter()
        .filter(|s| s.format != "kernel_text")
        .collect();
    match &time_pair {
        Some((now, lsk_ref)) => {
            let now = *now;
            let workers = 8.min(non_kernel.len());
            std::thread::scope(|scope| {
                for _ in 0..workers {
                    scope.spawn(|| {
                        loop {
                            let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            if i >= non_kernel.len() {
                                break;
                            }
                            let (ok, text) = probe_one(
                                non_kernel[i],
                                ProbeParams {
                                    now,
                                    lsk_ref,
                                    void_eph: &void_eph,
                                    env,
                                    fetchone,
                                    precise,
                                    lat,
                                    lon,
                                },
                            );
                            if ok {
                                accepted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                out_lock.lock().unwrap().push_str(&text);
                            } else {
                                declined.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                dead_lock.lock().unwrap().push_str(&text);
                            }
                            std::thread::sleep(std::time::Duration::from_millis(300));
                        }
                    });
                }
            });
        }
        None => {
            for _ in &non_kernel {
                declined.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                dead_lock
                    .lock()
                    .unwrap()
                    .push_str("# declined: time absent\n");
            }
        }
    }
    let accepted = accepted.load(std::sync::atomic::Ordering::Relaxed);
    let declined = declined.load(std::sync::atomic::Ordering::Relaxed);
    let out = out_lock.into_inner().unwrap();
    let dead = dead_lock.into_inner().unwrap();
    std::fs::create_dir_all("phi/pipeline").ok();
    if std::fs::write("phi/pipeline/probe_survivors.φ", &out).is_err() {
        eprintln!("write phi/pipeline/probe_survivors.φ: the register does not remember");
    }
    let per_input = format!("{}.survivors.φ", path);
    if std::fs::write(&per_input, &out).is_err() {
        eprintln!("write {}: the register does not remember", per_input);
    }
    if std::fs::write("phi/pipeline/probe_void.txt", &dead).is_err() {
        eprintln!("write phi/pipeline/probe_void.txt: the register does not remember");
    }
    eprintln!(
        "probe: wrote phi/pipeline/probe_survivors.φ ({} verified) and phi/pipeline/probe_void.txt ({} declined)",
        accepted, declined
    );
    0
}

pub fn extract_all_template_values(
    substituted_url: &str,
    template_url: &str,
) -> HashMap<String, String> {
    let mut values = HashMap::new();
    let bytes = template_url.as_bytes();
    let mut markers: Vec<(usize, usize, &str)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'}' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
                if let Ok(marker) = std::str::from_utf8(&bytes[start..i]) {
                    markers.push((start, i, marker));
                }
            }
        } else {
            i += 1;
        }
    }
    let mut parts: Vec<&str> = Vec::new();
    let mut prev_end = 0;
    for &(start, end, _) in &markers {
        parts.push(&template_url[prev_end..start]);
        prev_end = end;
    }
    parts.push(&template_url[prev_end..]);
    let sub = substituted_url;
    let mut pos = 0;
    for idx in 0..markers.len() {
        let part = parts[idx];
        if !part.is_empty() {
            match sub[pos..].find(part) {
                Some(offset) => pos += offset + part.len(),
                None => break,
            }
        }
        let marker = markers[idx].2;
        let next_part = parts[idx + 1];
        let val_end = if next_part.is_empty() {
            let next_const = parts[idx + 2..].iter().find(|p| !p.is_empty());
            match next_const {
                Some(nc) => match sub[pos..].find(nc) {
                    Some(p) => pos + p,
                    None => sub.len(),
                },
                None => sub.len(),
            }
        } else {
            match sub[pos..].find(next_part) {
                Some(p) => pos + p,
                None => sub.len(),
            }
        };
        let val_str = &sub[pos..val_end];
        values.insert(marker.to_string(), val_str.to_string());
        pos = val_end;
    }
    values
}

pub fn bruteforce_precision(substituted_url: &str, template_url: &str, ttl: u64) -> String {
    let spatial: &[&str] = &["{lat}", "{lon}", "{x}", "{y}", "{z}"];
    let has_spatial = spatial.iter().any(|v| template_url.contains(v));
    if !has_spatial {
        return String::new();
    }
    let all_values = extract_all_template_values(substituted_url, template_url);
    let baseline = fetch_raw(substituted_url, None, &[], ttl);
    let mut effective_dp: usize = 0;
    for dp in 0..=15 {
        let mut test_url = template_url.to_string();
        for (marker, value_str) in &all_values {
            let replacement: String = if spatial.contains(&marker.as_str()) {
                match marker.as_str() {
                    "{lat}" => format!("{:.lat_dp$}", 35.0, lat_dp = dp),
                    "{lon}" => format!("{:.lon_dp$}", 139.0, lon_dp = dp),
                    "{x}" => format!("{:.x_dp$}", 1.495978707e11, x_dp = dp),
                    "{y}" => format!("{:.y_dp$}", 0.0, y_dp = dp),
                    "{z}" => format!("{:.z_dp$}", 0.0, z_dp = dp),
                    _ => format!("{:.prec$}", 0.0, prec = dp),
                }
            } else {
                value_str.clone()
            };
            test_url = test_url.replace(marker, &replacement);
        }
        let body = fetch_raw(&test_url, None, &[], ttl);
        if let (Some(b), Some(base)) = (&body, &baseline)
            && b != base
        {
            effective_dp = dp;
        }
    }
    format!("# template_precision {}dp\n", effective_dp)
}

fn iso_sample_seconds(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 || !b[..19].iter().all(u8::is_ascii) {
        return None;
    }
    if b[4] != b'-'
        || b[7] != b'-'
        || (b[10] != b'T' && b[10] != b' ')
        || b[13] != b':'
        || b[16] != b':'
    {
        return None;
    }
    let year: i64 = s[0..4].parse().ok()?;
    let month: u32 = s[5..7].parse().ok()?;
    let day: u32 = s[8..10].parse().ok()?;
    let hour: u32 = s[11..13].parse().ok()?;
    let minute: u32 = s[14..16].parse().ok()?;
    let second: u32 = s[17..19].parse().ok()?;
    if month == 0 || month > 12 || day == 0 || day > 31 || hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let days = ymd_to_days(year, month, day)?;
    let mut secs =
        days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64;
    if b.len() > 19 && b[19] == b'.' {
        let mut end = 20;
        while end < b.len() && b[end].is_ascii_digit() {
            end += 1;
        }
        if end == 20 {
            return None;
        }
        let places = (end - 20) as i32;
        let fraction: f64 = s[20..end].parse().ok()?;
        secs += fraction / 10f64.powi(places);
    }
    Some(secs)
}

fn scan_iso_samples(body: &str) -> Vec<f64> {
    let b = body.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 19 <= b.len() {
        if b[i].is_ascii_digit()
            && b[i + 4] == b'-'
            && b[i + 7] == b'-'
            && (b[i + 10] == b'T' || b[i + 10] == b' ')
            && b[i + 13] == b':'
            && b[i + 16] == b':'
            && let Some(secs) = iso_sample_seconds(&body[i..])
        {
            out.push(secs);
            i += 19;
            continue;
        }
        i += 1;
    }
    out
}

fn to_ttl_seconds(secs: f64) -> Option<u64> {
    if secs.is_finite() && secs >= 1.0 {
        Some(secs.round() as u64)
    } else {
        None
    }
}

fn body_sample_ttl(body: &str) -> Option<u64> {
    let mut samples = scan_iso_samples(body);
    samples.sort_by(f64::total_cmp);
    samples.dedup();
    if samples.len() < 2 {
        return None;
    }
    let mut spacing = Vec::with_capacity(samples.len() - 1);
    for pair in samples.windows(2) {
        let gap = pair[1] - pair[0];
        if gap < 1.0 {
            return None;
        }
        spacing.push(gap);
    }
    spacing.sort_by(f64::total_cmp);
    let n = spacing.len();
    let median = if n % 2 == 1 {
        spacing[n / 2]
    } else {
        (spacing[n / 2 - 1] + spacing[n / 2]) * 0.5
    };
    to_ttl_seconds(median)
}

fn cadence_seconds(field: &str) -> Option<u64> {
    let t = field.trim();
    if let Ok(secs) = t.parse::<f64>() {
        return to_ttl_seconds(secs);
    }
    let rest = t.strip_prefix("PT")?;
    let cut = rest.find(|c: char| !c.is_ascii_digit() && c != '.')?;
    let (amount, unit) = rest.split_at(cut);
    let secs: f64 = amount.parse().ok()?;
    let mult = match unit {
        "S" => 1.0,
        "M" => 60.0,
        "H" => 3600.0,
        _ => return None,
    };
    to_ttl_seconds(secs * mult)
}

fn cadence_value_seconds(val: &JsonVal) -> Option<u64> {
    match val {
        JsonVal::Num(n) => to_ttl_seconds(*n),
        JsonVal::Str(s) => cadence_seconds(s),
        _ => None,
    }
}

fn find_cadence_seconds(val: &JsonVal) -> Option<u64> {
    match val {
        JsonVal::Obj(map) => {
            for (k, v) in map {
                if k.eq_ignore_ascii_case("cadence")
                    && let Some(secs) = cadence_value_seconds(v)
                {
                    return Some(secs);
                }
            }
            map.values().find_map(find_cadence_seconds)
        }
        JsonVal::Arr(arr) => arr.iter().find_map(find_cadence_seconds),
        _ => None,
    }
}

fn hapi_cadence_url(url: &str) -> Option<String> {
    let after = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let (netloc, route) = after.split_once('/')?;
    let (path, query) = match route.split_once('?') {
        Some((p, q)) => (p, q),
        None => (route, ""),
    };
    if !path.contains("hapi/") {
        return None;
    }
    let id = query
        .split('&')
        .find_map(|p| p.strip_prefix("id="))
        .filter(|v| !v.is_empty())?;
    let base = if url.starts_with("https://") {
        "https://"
    } else {
        "http://"
    };
    Some(format!("{}{}/hapi/info?id={}", base, netloc, id))
}

fn hapi_cadence_ttl(url: &str, env: &HashMap<String, String>) -> Option<u64> {
    let info_url = resolve_secret(&hapi_cadence_url(url)?, env);
    let body = fetch_raw_probe(&info_url, None, &[])?;
    let parsed = parse_json(&body)?;
    find_cadence_seconds(&parsed)
}

pub fn derive_ttl(url: &str, body: &str, env: &HashMap<String, String>) -> Option<u64> {
    if let Some(secs) = body_sample_ttl(body) {
        return Some(secs);
    }
    if url.contains("/hapi/") {
        return hapi_cadence_ttl(url, env);
    }
    None
}

struct HapiMetaParam {
    name: String,
    unit: Option<String>,
}

fn hapi_meta_params(parsed: &JsonVal) -> Option<Vec<HapiMetaParam>> {
    let JsonVal::Obj(root) = parsed else {
        return None;
    };
    let JsonVal::Arr(list) = root.get("parameters")? else {
        return None;
    };
    let mut out = Vec::new();
    for p in list {
        let JsonVal::Obj(pm) = p else {
            continue;
        };
        let Some(JsonVal::Str(name)) = pm.get("name") else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        let unit = match pm.get("units") {
            Some(JsonVal::Str(u)) if !u.trim().is_empty() => Some(u.clone()),
            _ => None,
        };
        out.push(HapiMetaParam {
            name: name.clone(),
            unit,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

fn hapi_meta_for(
    url: &str,
    parsed: &JsonVal,
    env: &HashMap<String, String>,
) -> Option<Vec<HapiMetaParam>> {
    hapi_meta_params(parsed).or_else(|| {
        let info_url = resolve_secret(&hapi_cadence_url(url)?, env);
        let info_body = fetch_raw_probe(&info_url, None, &[])?;
        let info_json = parse_json(&info_body)?;
        hapi_meta_params(&info_json)
    })
}

fn hapi_request_order(url: &str, meta: &[HapiMetaParam]) -> Vec<String> {
    let query = match url.split_once('?') {
        Some((_, q)) => q,
        None => "",
    };
    let requested: Option<Vec<String>> = query
        .split('&')
        .find_map(|kv| kv.strip_prefix("parameters="))
        .map(|list| {
            list.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        });
    match requested {
        Some(rs) if !rs.is_empty() => rs,
        _ => meta.iter().skip(1).map(|m| m.name.clone()).collect(),
    }
}

fn hapi_query_id(url: &str) -> Option<String> {
    let query = url.split_once('?').map(|(_, q)| q)?;
    query
        .split('&')
        .find_map(|kv| kv.strip_prefix("id="))
        .filter(|v| !v.is_empty())
        .map(str::to_string)
}

pub fn register_hapi_units_of(sources: &[SourceConfig]) -> HashMap<(String, String), String> {
    let mut out = HashMap::new();
    for src in sources {
        let Some(id) = hapi_query_id(&src.url) else {
            continue;
        };
        let mut short_to_long: Vec<(String, String)> = Vec::new();
        for ext in &src.extracts {
            if let Extract::Hapi(pairs) = ext {
                short_to_long.extend(pairs.iter().cloned());
            }
        }
        for (short, long) in short_to_long {
            for ext in &src.extracts {
                if let Extract::Field(fc) = ext
                    && fc.key == long
                {
                    out.insert((id.clone(), short.clone()), fc.unit.clone());
                }
            }
        }
    }
    out
}

fn register_hapi_units() -> HashMap<(String, String), String> {
    register_hapi_units_of(&load_sources())
}

pub fn hapi_draft_fields(
    url: &str,
    parsed: &JsonVal,
    env: &HashMap<String, String>,
    fields: &mut String,
) -> bool {
    if !url.contains("/hapi/") {
        return false;
    }
    let JsonVal::Obj(root) = parsed else {
        return false;
    };
    let Some(JsonVal::Arr(data)) = root.get("data") else {
        return false;
    };
    if !matches!(data.first(), Some(JsonVal::Arr(_))) {
        return false;
    }
    let Some(meta) = hapi_meta_for(url, parsed, env) else {
        fields.push_str("# pending hapi columns — HAPI parameter metadata absent, review\n");
        return true;
    };
    let width = match data.first() {
        Some(JsonVal::Arr(row)) => row.len(),
        _ => 1,
    };
    let ncols = width.saturating_sub(1);
    if ncols == 0 {
        fields.push_str("# pending hapi — data rows carry no value column\n");
        return true;
    }
    let order = hapi_request_order(url, &meta);
    if order.len() != ncols {
        for (i, req) in order.iter().enumerate() {
            fields.push_str(&format!(
                "# pending hapi column {} — {} alignment unverified (metadata width {} vs data width {}), review\n",
                i + 1,
                req,
                order.len(),
                ncols
            ));
        }
        return true;
    }
    let resolved: Vec<Option<&HapiMetaParam>> = order
        .iter()
        .map(|req| meta.iter().find(|m| m.name == *req))
        .collect();
    if resolved.iter().any(|c| c.is_none()) {
        for (i, req) in order.iter().enumerate() {
            fields.push_str(&format!(
                "# pending hapi column {} — parameter {} absent from metadata, review\n",
                i + 1,
                req
            ));
        }
        return true;
    }
    let mut hapi_line = String::from("hapi");
    for (short, col) in order.iter().zip(resolved.iter()) {
        match col {
            Some(col) => hapi_line.push_str(&format!(" {}={}", short, col.name)),
            None => return true,
        }
    }
    fields.push_str(&hapi_line);
    fields.push('\n');
    let register = register_hapi_units();
    for col in resolved.iter() {
        let Some(col) = col else {
            return true;
        };
        let (force, _, tau) = probe_classify(&col.name);
        match force {
            "UNCERTAIN" => {
                fields.push_str(&format!(
                    "# uncertain field {} — force/unit undetermined, review\n",
                    col.name
                ));
            }
            "DROP" => {}
            _ => match &col.unit {
                Some(unit) => {
                    let unit_norm = normalize_unit(unit);
                    let in_registry = match force_id_of(force) {
                        Some(fid) => allowed_units_for_force(fid).contains(&unit_norm.as_str()),
                        None => false,
                    };
                    if !in_registry {
                        let mut note = format!("# unit {} not in force registry", unit);
                        if let Some(reg) =
                            hapi_query_id(url).and_then(|id| register.get(&(id, col.name.clone())))
                        {
                            note.push_str(&format!(" — register carries {}", reg));
                        }
                        note.push_str(" — review\n");
                        fields.push_str(&note);
                    }
                    if let Some(line) = hapi_field_line(&col.name, force, unit, tau) {
                        fields.push_str(&line);
                    }
                }
                None => {
                    fields.push_str(&format!(
                        "# pending field {} — HAPI unit absent, review\n",
                        col.name
                    ));
                }
            },
        }
    }
    for (col_idx, col) in resolved.iter().enumerate() {
        let Some(col) = col else {
            continue;
        };
        let cell_idx = col_idx + 1;
        let mut first: Option<f64> = None;
        let mut finite: Option<f64> = None;
        for row in data.iter() {
            let JsonVal::Arr(r) = row else {
                continue;
            };
            let Some(cell) = r.get(cell_idx).and_then(json_num) else {
                continue;
            };
            if first.is_none() {
                first = Some(cell);
            }
            if cell.is_finite() {
                finite = Some(cell);
                break;
            }
        }
        match finite {
            Some(v) => {
                if first.is_some_and(|f| !f.is_finite()) {
                    fields.push_str(&format!(
                        "# {} = {} — first row server fill, first finite sample shown\n",
                        col.name, v
                    ));
                } else {
                    fields.push_str(&format!("# {} = {}\n", col.name, v));
                }
            }
            None => match first {
                Some(f) => fields.push_str(&format!(
                    "# {} = {} — window carries no finite sample\n",
                    col.name, f
                )),
                None => fields.push_str(&format!("# {} = no sample\n", col.name)),
            },
        }
    }
    true
}

pub fn find_timestamp(val: &JsonVal) -> Option<f64> {
    if let JsonVal::Obj(map) = val {
        for (k, v) in map {
            if is_time_key(k)
                && let Some(n) = json_num(v)
            {
                return Some(n);
            }
        }
    }
    None
}

pub fn is_coord_key(key: &str) -> bool {
    let kl = key.to_lowercase();
    kl == "latitude"
        || kl == "lat"
        || kl == "longitude"
        || kl == "lon"
        || kl == "lng"
        || kl == "altitude"
        || kl == "alt"
        || kl == "depth"
        || kl == "solar_lat"
        || kl == "solar_lon"
        || kl == "ra"
        || kl == "dec"
        || kl.contains("raj2000")
        || kl.contains("dej2000")
}

pub fn draft_field_line(key: &str, force: &str, unit: &str, tau: f64) -> Option<String> {
    let fid = force_id_of(force)?;
    let kid = kernel_id_for_force(fid)?;
    let kernel = kernel_name_of(kid)?;
    Some(format!(
        "field {} {} {} {} {} {} 0.0 0.0\n",
        key, key, kernel, force, unit, tau
    ))
}

fn kernel_name_of(id: u8) -> Option<&'static str> {
    match id {
        0 => Some("inverse-square"),
        1 => Some("gaussian-inverse-square"),
        2 => Some("gaussian-inverse"),
        3 => Some("erfc"),
        4 => Some("exponential-decay"),
        5 => Some("patch-levy"),
        6 => Some("inverse-linear"),
        _ => None,
    }
}

fn hapi_field_line(key: &str, force: &str, unit: &str, tau: f64) -> Option<String> {
    let fid = force_id_of(force)?;
    let kid = kernel_id_for_force(fid)?;
    let kernel = kernel_name_of(kid)?;
    Some(format!(
        "field {} {} {} {} {} {} 0.0 0.0\n",
        key, key, kernel, force, unit, tau
    ))
}

pub fn probe_csv(raw: &str) -> Option<String> {
    let first_header = raw.lines().find_map(|line| {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let stripped = (if let Some(s) = trimmed.strip_prefix('#') {
                s
            } else {
                trimmed
            })
            .trim();
            if !stripped.is_empty() {
                return Some(stripped);
            }
        }
        None
    })?;
    let cols: Vec<&str> = first_header.split_whitespace().collect();
    if cols.len() <= 5 {
        return None;
    }
    let mut out = String::new();
    for col in &cols[5..] {
        let lower = col.to_lowercase();
        if lower == "yy" || lower == "mm" || lower == "dd" || lower == "hh" || lower == "min" {
            continue;
        }
        if is_unit_name(&lower) {
            continue;
        }
        if is_drop_key(&lower) {
            continue;
        }
        out.push_str(&format!("# {}\n", col));
        let (force, unit, tau) = probe_classify(col);
        if force != "DROP"
            && let Some(line) = draft_field_line(col, force, unit, tau)
        {
            out.push_str(&line);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn probe_classify(key: &str) -> (&str, &str, f64) {
    if let Some(verdict) = intentional_core(key) {
        return verdict;
    }
    if let Some((force, unit, tau)) = register_field_map().get(key).copied() {
        return (force, unit, tau);
    }
    let (force, unit, tau) = probe_classify_raw(key);
    if force == "DROP" || force == "UNCERTAIN" {
        return (force, unit, tau);
    }
    let allowed = match force_id_of(force) {
        Some(fid) => allowed_units_for_force(fid).contains(&normalize_unit(unit).as_str()),
        None => false,
    };
    if allowed {
        (force, unit, tau)
    } else {
        ("UNCERTAIN", "", 0.0)
    }
}

fn intentional_core(key: &str) -> Option<(&'static str, &'static str, f64)> {
    let kl = key.to_lowercase();
    if kl == "sample" {
        return Some(("DROP", "", 0.0));
    }
    if kl == "reporting_network"
        || kl == "event_source"
        || kl == "orbit_class"
        || kl == "source_class"
        || kl == "spot_class"
        || kl == "hale_class"
        || kl.contains("uv_index")
        || kl.ends_with("_type")
    {
        return Some(("DROP", "", 0.0));
    }
    match key {
        "copernicus_air_pressure_at_sea_level"
        | "cosmic_ro_pressure_hpa"
        | "igra_air_pressure_hpa"
        | "noaa_gsod_slp_hpa"
        | "noaa_isd_slp_hpa"
        | "observations.seaLevelPressure"
        | "omni_solarwind_pressure_npa"
        | "pressure"
        | "properties.PRES"
        | "properties.PTDY"
        | "soles.0.pressure"
        | "sols.6.pressure"
        | "surface_partial_pressure_of_carbon_dioxide_in_sea_water"
        | "surface_pressure" => return Some(("acoustic", "hPa", 21600.0)),
        _ => {}
    }
    None
}

fn register_field_map() -> &'static HashMap<String, (&'static str, &'static str, f64)> {
    static MAP: OnceLock<HashMap<String, (&'static str, &'static str, f64)>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map: HashMap<String, (&'static str, &'static str, f64)> = HashMap::new();
        for line in include_str!("../../phi/sources.φ").lines() {
            let p: Vec<&str> = line.split_whitespace().collect();
            match p.first().copied() {
                Some("field" | "first" | "last" | "lastrow") if p.len() >= 7 => {
                    if let Ok(tau) = p[6].parse::<f64>()
                        && tau.is_finite()
                        && tau > 0.0
                    {
                        map.insert(p[1].to_string(), (p[4], p[5], tau));
                    }
                }
                _ => {}
            }
        }
        map
    })
}

fn probe_classify_raw(key: &str) -> (&str, &str, f64) {
    let kl = key.to_lowercase();
    if kl.contains("sample") || kl.contains("sort") || kl.contains("order") || kl.contains("bbox") {
        ("DROP", "", 0.0)
    } else if kl == "par"
        || kl == "plx"
        || kl.contains("parallax")
        || kl.contains("plx_value")
        || kl.contains("_plx_")
        || kl.contains("proper_motion")
    {
        ("DROP", "", 0.0)
    } else if kl == "bp_rp"
        || kl == "bp_minus_rp"
        || kl == "b_minus_v_color"
        || kl.contains("color_index")
    {
        ("DROP", "", 0.0)
    } else if kl.ends_with("_temp_f") {
        ("thermal", "f", 86400.0)
    } else if kl.ends_with("_wind_kt") || kl.ends_with("_speed_kt") {
        ("advective", "kt", 86400.0)
    } else if kl.ends_with("_mass_kt") {
        ("diffusion", "kt_mass", 86400.0)
    } else if kl.ends_with("_inch") {
        ("acoustic", "inch", 86400.0)
    } else if kl.ends_with("_miles") {
        ("em", "mile", 86400.0)
    } else if kl.ends_with("_dbar") {
        ("acoustic", "dbar", 86400.0)
    } else if kl.ends_with("_mpc") {
        ("gravity", "mpc", 604800.0)
    } else if kl.ends_with("_axis_au") {
        ("gravity", "au", 604800.0)
    } else if kl.ends_with("orbital_inclination_deg") {
        ("gravity", "deg", 604800.0)
    } else if kl.ends_with("_arcmin") {
        ("em", "arcmin", 604800.0)
    } else if kl == "lod" {
        ("gravity", "s", 86400.0)
    } else if kl == "dpsi" || kl == "deps" {
        ("gravity", "arcsec", 86400.0)
    } else if kl.contains("cloud_fraction") {
        ("diffusion", "1", 86400.0)
    } else if kl.contains("albedo") {
        ("em", "1", 604800.0)
    } else if kl.contains("eccentricity") {
        ("gravity", "1", 604800.0)
    } else if kl == "rho_cos_phi" || kl == "rho_sin_phi" {
        ("gravity", "1", 604800.0)
    } else if kl.contains("eop_")
        || kl.contains("ut1_utc")
        || kl.contains("polar_motion")
        || kl == "pmx"
        || kl == "pmy"
    {
        ("UNCERTAIN", "", 0.0)
    } else if kl.contains("phase_cycle")
        || kl.ends_with("polar_angle_cycles")
        || kl == "l1"
        || kl == "l2"
        || kl == "l5"
        || kl.ends_with("_cycle")
    {
        ("em", "cycle", 604800.0)
    } else if kl.ends_with("angle_a") || kl.ends_with("angle_b") {
        ("em", "rad", 604800.0)
    } else if kl.ends_with("x_core_m")
        || kl.ends_with("y_core_m")
        || kl.contains("las_x_")
        || kl.contains("las_y_")
        || kl.contains("las_z_")
    {
        ("em", "m", 604800.0)
    } else if kl.ends_with("shower_age") {
        ("em", "1", 604800.0)
    } else if kl.ends_with("_classification") {
        ("diffusion", "1", 604800.0)
    } else if kl.ends_with("_azimuth_deg") || kl.ends_with("_zenith_deg") {
        ("acoustic", "deg", 300.0)
    } else if kl.contains("teff") {
        ("thermal", "K", 604800.0)
    } else if kl.contains("brightness") {
        ("thermal", "K", 3600.0)
    } else if kl.contains("sst") {
        ("thermal", "K", 360.0)
    } else if kl.contains("thermal_speed") {
        ("thermal", "km/s", 3600.0)
    } else if kl.ends_with("_kelvin") || kl.ends_with("_k") {
        ("thermal", "K", 3600.0)
    } else if kl.contains("temp")
        || kl.contains("atmp")
        || kl.contains("wtmp")
        || kl.contains("dewp")
        || kl.contains("dew_point")
        || kl.contains("tmax")
        || kl.contains("tmin")
        || kl.contains("tavg")
        || kl.contains("t25")
        || kl.ends_with("max_c")
        || kl.ends_with("min_c")
    {
        ("thermal", "C", 3600.0)
    } else if kl.starts_with("cmb") {
        ("thermal", "K", 604800.0)
    } else if kl.contains("ptdy")
        || kl.contains("pres")
        || kl.contains("baro")
        || kl.contains("slp")
        || kl.contains("altim")
    {
        ("acoustic", "hPa", 21600.0)
    } else if kl.contains("flow_speed") || kl.contains("proton_speed") {
        ("advective", "km/s", 3600.0)
    } else if kl.contains("spd")
        || kl.contains("gust")
        || kl.contains("gst")
        || kl.contains("wdsp")
        || kl.contains("qbo")
        || (kl.contains("wind") && !kl.contains("dir"))
    {
        ("advective", "m/s", 60.0)
    } else if kl.contains("radiation") || kl.contains("irradiance") {
        ("em", "W/m2", 3600.0)
    } else if kl.contains("wave_dir") {
        ("acoustic", "deg", 60.0)
    } else if kl.contains("dir") || kl.contains("heading") {
        ("advective", "deg", 60.0)
    } else if kl.contains("eastward")
        || kl.contains("northward")
        || kl.contains("ucur")
        || kl.contains("vcur")
    {
        ("advective", "m/s", 3600.0)
    } else if kl.ends_with("u_cm_s") || kl.ends_with("v_cm_s") {
        ("advective", "cm/s", 3600.0)
    } else if kl.contains("dpd")
        || kl.contains("apd")
        || kl.contains("dominant_period")
        || kl.contains("avg_period")
    {
        ("acoustic", "s", 21600.0)
    } else if kl.contains("rms_amplitude") {
        ("acoustic", "pa", 300.0)
    } else if kl.contains("hydrophone") {
        ("acoustic", "count", 60.0)
    } else if kl.contains("gong") {
        ("acoustic", "m/s", 604800.0)
    } else if kl.contains("wave") || kl.contains("wvht") || kl.contains("swell") {
        ("acoustic", "m", 10.0)
    } else if kl.contains("tide")
        || kl.contains("gage_height")
        || kl.contains("_height")
        || kl.contains("ssh")
        || kl.contains("ssha")
        || kl.contains("_h_ph")
        || kl.contains("distance")
        || kl.contains("river_stage")
    {
        ("gravity", "m", 3600.0)
    } else if kl.contains("range_rate") {
        ("gravity", "m/s", 86400.0)
    } else if kl.contains("range_accl") {
        ("gravity", "m/s2", 86400.0)
    } else if kl.contains("transit_depth") {
        ("em", "ppt", 604800.0)
    } else if kl.contains("depth") {
        ("seismic-body", "km", 10.0)
    } else if kl.contains("flux") || kl.contains("radiance") {
        ("em", "W/m2", 3600.0)
    } else if kl.contains("radiative_power") {
        ("em", "W", 3600.0)
    } else if kl.contains("radiant_energy") {
        ("em", "J", 3600.0)
    } else if kl.contains("radiated_energy") {
        ("em", "e10j", 3600.0)
    } else if kl.contains("impact_energy") {
        ("em", "kt_tnt", 3600.0)
    } else if kl.contains("fluence") {
        ("em", "erg/cm2", 604800.0)
    } else if kl.ends_with("_ev") {
        ("em", "eV", 604800.0)
    } else if kl.contains("xrs") || kl.contains("xray") {
        ("em", "W/m2", 3600.0)
    } else if kl.contains("x_class") {
        ("em", "1e-4W/m2", 3600.0)
    } else if kl.contains("kp_a_") || kl.contains("kp_3h") {
        ("em", "1", 3600.0)
    } else if kl == "bx"
        || kl == "by"
        || kl == "bz"
        || kl == "bt"
        || kl == "dst"
        || kl.contains("_b_")
        || kl.ends_with("_nt")
        || kl.starts_with("bx_")
        || kl.starts_with("by_")
        || kl.starts_with("bz_")
        || kl.starts_with("bt_")
    {
        ("em", "nT", 60.0)
    } else if kl.ends_with("_ppb") {
        ("diffusion", "ppb", 86400.0)
    } else if kl.ends_with("_ppmv") {
        ("diffusion", "ppmv", 86400.0)
    } else if kl.ends_with("_ppm") {
        ("diffusion", "ppm", 86400.0)
    } else if kl.ends_with("_percent") || kl.ends_with("_pct") {
        ("diffusion", "%", 86400.0)
    } else if kl.ends_with("_ppt") {
        ("em", "ppt", 604800.0)
    } else if kl.ends_with("_umol_kg") || kl.ends_with("_umolkg") {
        ("diffusion", "micromole/kg", 604800.0)
    } else if kl.ends_with("_mg_m3") || kl.ends_with("_mgm3") {
        ("diffusion", "mg/m3", 604800.0)
    } else if kl.ends_with("_ug_m3") || kl.ends_with("_ugm3") {
        ("diffusion", "ug/m3", 3600.0)
    } else if kl.ends_with("_jm2") || kl.ends_with("_j_m2") {
        ("thermal", "j/m2", 604800.0)
    } else if kl.ends_with("_mhz") {
        ("em", "mhz", 3600.0)
    } else if kl.contains("hum") || kl.contains("rh") || kl == "rel_hum" {
        ("diffusion", "%", 86400.0)
    } else if kl.contains("cloud_cover") || kl.contains("leaf_wetness") {
        ("diffusion", "%", 300.0)
    } else if kl.contains("precipitable_water") {
        ("diffusion", "cm", 86400.0)
    } else if kl.contains("available_water") {
        ("diffusion", "mm", 86400.0)
    } else if kl.contains("rain") || kl.contains("prcp") || kl.contains("precip") {
        ("acoustic", "mm", 60.0)
    } else if kl.contains("vis") {
        ("em", "km", 60.0)
    } else if kl.contains("co2")
        || kl.contains("ch4")
        || kl.contains("o3")
        || kl.contains("no2")
        || kl.contains("so2")
        || kl.contains("h2s")
    {
        ("diffusion", "ppm", 86400.0)
    } else if kl.contains("pm25")
        || kl.contains("pm10")
        || kl.contains("pm2_5")
        || kl.contains("pm1_0")
        || kl.ends_with("_ugm3")
        || kl.ends_with("_ug_m3")
    {
        ("diffusion", "ug/m3", 3600.0)
    } else if kl.contains("vel")
        || kl.contains("vlct")
        || kl.contains("vpec")
        || kl.contains("_rv_")
        || kl.ends_with("cz_kms")
        || kl.ends_with("_rv_km_s")
    {
        ("advective", "km/s", 60.0)
    } else if kl.contains("speed") {
        ("advective", "km/s", 3600.0)
    } else if kl.contains("freq") || kl.ends_with("_hz") {
        ("em", "Hz", 60.0)
    } else if kl.contains("dens") {
        ("diffusion", "cm-3", 3600.0)
    } else if kl.contains("doxy") || kl.contains("nitrate") {
        ("diffusion", "micromole/kg", 604800.0)
    } else if kl.contains("chla") {
        ("diffusion", "mg/m3", 604800.0)
    } else if kl.contains("bbp") {
        ("diffusion", "m-1", 604800.0)
    } else if kl.contains("oxygen") {
        ("diffusion", "ml/l", 604800.0)
    } else if kl.contains("total_ozone") {
        ("diffusion", "DU", 86400.0)
    } else if kl.contains("turbidity") {
        ("diffusion", "ntu", 3600.0)
    } else if kl.contains("psal") {
        ("diffusion", "psu", 86400.0)
    } else if kl.ends_with("_mg_kg") {
        ("diffusion", "mg/kg", 604800.0)
    } else if kl.contains("conc") || kl.contains("salinity") {
        ("diffusion", "PSU", 86400.0)
    } else if kl.contains("streamflow") || kl.contains("river_flow") {
        ("advective", "cfs", 3600.0)
    } else if kl.contains("e_rms") {
        ("electric", "V/m", 3600.0)
    } else if kl.contains("rpw_e") || kl.contains("efield") {
        ("electric", "mV/m", 3600.0)
    } else if kl.contains("gic") {
        ("electric", "A", 3600.0)
    } else if kl == "db" || kl.ends_with("_db") {
        ("acoustic", "dB", 60.0)
    } else if kl.contains("discharge") {
        ("advective", "m3/s", 60.0)
    } else if kl.ends_with("_tec") || kl.contains("_tec_") {
        ("em", "tecu", 7200.0)
    } else if kl.contains("aod")
        || kl.contains("aerosol_optical")
        || kl.contains("cdod")
        || kl.contains("redshift")
    {
        ("em", "1", 86400.0)
    } else if kl.contains("extinction_ebv") {
        ("em", "mag", 604800.0)
    } else if kl.contains("extinction") {
        ("em", "1", 604800.0)
    } else if kl.contains("_dm_") || kl.contains("dispersion") || kl.ends_with("_dm_pccm3") {
        ("em", "pc/cm3", 604800.0)
    } else if kl.contains("separation") {
        ("em", "arcsec", 604800.0)
    } else if kl.ends_with("scatter_ms") {
        ("em", "ms", 604800.0)
    } else if kl.contains("period_s") {
        ("em", "s", 604800.0)
    } else if kl.contains("period_d") {
        ("em", "d", 604800.0)
    } else if kl.ends_with("snr") {
        ("em", "1", 604800.0)
    } else if kl.ends_with("_mjy") {
        ("em", "mJy", 604800.0)
    } else if kl.contains("neutron") {
        ("em", "%", 3600.0)
    } else if kl.ends_with("_cpm") {
        ("em", "cpm", 3600.0)
    } else if kl.contains("dose_rate") {
        ("em", "usv/h", 3600.0)
    } else if kl.contains("activity") {
        ("em", "bq/l", 3600.0)
    } else if kl.contains("refractivity") {
        ("em", "N-units", 604800.0)
    } else if kl.contains("logg") {
        ("gravity", "logg", 604800.0)
    } else if kl.contains("planet_mass") {
        ("gravity", "M_earth", 604800.0)
    } else if kl.contains("mass") {
        ("gravity", "M_sun", 604800.0)
    } else if kl.contains("planet_radius") {
        ("gravity", "R_earth", 604800.0)
    } else if kl.contains("igets_gravity") {
        ("gravity", "nm/s2", 86400.0)
    } else if kl.ends_with("_ms2") {
        ("gravity", "m/s2", 604800.0)
    } else if kl.contains("clock_bias") {
        ("em", "s", 86400.0)
    } else if kl.contains("storm_intensity") {
        ("advective", "knot", 300.0)
    } else if kl.contains("ccor") || kl.contains("las_intensity") {
        ("em", "1", 86400.0)
    } else if kl.contains("_c1_m")
        || kl.contains("_p1_m")
        || kl.contains("_c2_m")
        || kl.contains("_p2_m")
        || kl.contains("_c5_m")
    {
        ("em", "m", 86400.0)
    } else if kl.contains("dbhz") {
        ("em", "dbhz", 86400.0)
    } else if kl.ends_with("_count") || kl.contains("_count_") {
        ("em", "count", 604800.0)
    } else if kl.contains("occlt") {
        ("em", "1", 604800.0)
    } else if kl == "v" || kl == "s" {
        ("gravity", "m", 3600.0)
    } else if kl.contains("footprint") {
        ("em", "km", 60.0)
    } else if kl.contains("volt") || kl.contains("potential") {
        ("electric", "V", 60.0)
    } else if kl.contains("current") && !kl.contains("ocean") {
        ("electric", "A", 60.0)
    } else if kl.contains("conduct") {
        ("electric", "S/m", 3600.0)
    } else if kl.starts_with("fugin") {
        ("em", "k.m/s", 31536000.0)
    } else if kl.starts_with("gw2") {
        ("gravity", "1", 31536000.0)
    } else if kl.contains("sky1")
        || kl.contains("_dl3")
        || kl.starts_with("lhaaso")
        || kl.starts_with("hawc")
        || kl.starts_with("hess")
        || kl.starts_with("magic")
        || kl.starts_with("icecat")
        || kl.starts_with("antares")
    {
        ("em", "1", 31536000.0)
    } else if kl == "mag" || kl == "magnitude" || kl.contains("quake") || kl.ends_with("_mw") {
        ("seismic-body", "Mw", 3600.0)
    } else if kl.ends_with("mag")
        || kl.ends_with("mag1")
        || kl.ends_with("mag2")
        || kl.starts_with("mag_")
        || kl.contains("magpsf")
        || kl.contains("magap")
        || kl.contains("magnitude")
    {
        ("em", "mag", 604800.0)
    } else {
        ("UNCERTAIN", "", 0.0)
    }
}

pub fn walk_json_probe(
    val: &JsonVal,
    prefix: &str,
    out: &mut String,
    coords: &mut String,
    map_path: &mut Option<String>,
    budget: &mut usize,
) {
    match val {
        JsonVal::Obj(map) => {
            for (k, v) in map {
                let path = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                if is_coord_key(k) {
                    let unit = coord_unit(k);
                    let directive = coord_directive(k);
                    let exact = matches!(
                        k.to_lowercase().as_str(),
                        "lat"
                            | "latitude"
                            | "lon"
                            | "lng"
                            | "longitude"
                            | "alt"
                            | "altitude"
                            | "depth"
                    );
                    let line = format!("{} {} {}\n", directive, path, unit);
                    let marker = format!("{} ", directive);
                    let existing: Option<String> = coords
                        .lines()
                        .find(|l| l.starts_with(&marker))
                        .map(|l| l.to_string());
                    match existing {
                        None => coords.push_str(&line),
                        Some(old) => {
                            let old_key = old.split_whitespace().nth(1);
                            let old_exact = old_key.is_some_and(|ok_key| {
                                let tail = match ok_key.rfind('.') {
                                    Some(p) => &ok_key[p + 1..],
                                    None => ok_key,
                                };
                                matches!(
                                    tail.to_lowercase().as_str(),
                                    "lat"
                                        | "latitude"
                                        | "lon"
                                        | "lng"
                                        | "longitude"
                                        | "alt"
                                        | "altitude"
                                        | "depth"
                                )
                            });
                            if exact && !old_exact {
                                *coords = coords.replace(&format!("{}\n", old), &line);
                            }
                        }
                    }
                    if k == "depth" {
                        let (force, unit, tau) = probe_classify("depth");
                        if force != "DROP"
                            && let Some(line) = draft_field_line(&path, force, unit, tau)
                        {
                            out.push_str(&line);
                        }
                    }
                } else {
                    walk_json_probe(v, &path, out, coords, map_path, budget);
                }
            }
        }
        JsonVal::Arr(arr) => {
            if arr.is_empty() {
                return;
            }
            if prefix.ends_with(".coordinates") || prefix == "coordinates" {
                let lon_path = if prefix.is_empty() {
                    "coordinates.0".to_string()
                } else {
                    format!("{}.0", prefix)
                };
                let lat_path = if prefix.is_empty() {
                    "coordinates.1".to_string()
                } else {
                    format!("{}.1", prefix)
                };
                let alt_path = if prefix.is_empty() {
                    "coordinates.2".to_string()
                } else {
                    format!("{}.2", prefix)
                };
                let lon_line = format!("lon {} deg\n", lon_path);
                if !coords.contains(&lon_line) {
                    coords.push_str(&lon_line);
                }
                let lat_line = format!("lat {} deg\n", lat_path);
                if !coords.contains(&lat_line) {
                    coords.push_str(&lat_line);
                }
                let alt_line = format!("alt {} km\n", alt_path);
                if !coords.contains(&alt_line) {
                    coords.push_str(&alt_line);
                }
                return;
            }
            let first = &arr[0];
            if matches!(first, JsonVal::Obj(_)) {
                if map_path.is_none() {
                    *map_path = Some(if prefix.is_empty() {
                        ".".to_string()
                    } else {
                        prefix.to_string()
                    });
                    walk_json_probe(first, "", out, coords, map_path, budget);
                } else {
                    walk_json_probe(first, prefix, out, coords, map_path, budget);
                }
            } else {
                for (i, v) in arr.iter().enumerate() {
                    walk_json_probe(
                        v,
                        &format!("{}.{}", prefix, i),
                        out,
                        coords,
                        map_path,
                        budget,
                    );
                }
            }
        }
        JsonVal::Num(n) => {
            let key = match prefix.rfind('.') {
                Some(pos) => &prefix[pos + 1..],
                None => prefix,
            };
            if is_drop_key(key) || is_coord_key(key) {
                return;
            }
            if *budget == 0 {
                return;
            }
            *budget -= 1;
            out.push_str(&format!("# {} = {:?}\n", prefix, n));
            let (force, unit, tau) = probe_classify(key);
            if force == "UNCERTAIN" {
                out.push_str(&format!(
                    "# uncertain field {} — force/unit undetermined, review\n",
                    prefix
                ));
            } else if force != "DROP" {
                let unit_lc = unit.to_lowercase();
                let in_registry = match force_id_of(force) {
                    Some(fid) => allowed_units_for_force(fid).contains(&unit_lc.as_str()),
                    None => false,
                };
                if !in_registry {
                    out.push_str(&format!("# unit {} not in force registry — review\n", unit));
                }
                if let Some(line) = draft_field_line(prefix, force, unit, tau) {
                    out.push_str(&line);
                }
            }
        }
        JsonVal::Str(s) => {
            if let Ok(n) = s.parse::<f64>() {
                let key = match prefix.rfind('.') {
                    Some(pos) => &prefix[pos + 1..],
                    None => prefix,
                };
                if is_drop_key(key) || is_coord_key(key) {
                    return;
                }
                if *budget == 0 {
                    return;
                }
                *budget -= 1;
                out.push_str(&format!("# {} = {:?} (str)\n", prefix, n));
                let (force, unit, tau) = probe_classify(key);
                if force == "UNCERTAIN" {
                    out.push_str(&format!(
                        "# uncertain field {} — force/unit undetermined, review\n",
                        prefix
                    ));
                } else if force != "DROP" {
                    let unit_lc = unit.to_lowercase();
                    let in_registry = match force_id_of(force) {
                        Some(fid) => allowed_units_for_force(fid).contains(&unit_lc.as_str()),
                        None => false,
                    };
                    if !in_registry {
                        out.push_str(&format!("# unit {} not in force registry — review\n", unit));
                    }
                    if let Some(line) = draft_field_line(prefix, force, unit, tau) {
                        out.push_str(&line);
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn coord_unit(key: &str) -> &'static str {
    let kl = key.to_lowercase();
    if kl == "altitude" || kl == "alt" || kl.contains("depth") {
        "km"
    } else {
        "deg"
    }
}

pub fn coord_directive(key: &str) -> &'static str {
    let kl = key.to_lowercase();
    if kl == "altitude" || kl == "alt" || kl.contains("depth") {
        "alt"
    } else if kl == "ra" || kl.contains("raj2000") {
        "ra"
    } else if kl == "dec" || kl.contains("dej2000") {
        "dec"
    } else if kl.contains("lon") || kl == "lng" {
        "lon"
    } else {
        "lat"
    }
}

pub fn coord_precision(a: f64, b: f64) -> usize {
    let diff = (a - b).abs();
    if diff == 0.0 {
        return 15;
    }
    let mut p = 0;
    let mut d = diff;
    while d < 1.0 && p < 15 {
        d *= 10.0;
        p += 1;
    }
    p
}

pub fn measure_precision(val: &JsonVal) -> String {
    match val {
        JsonVal::Arr(arr) if arr.len() >= 2 => {
            let a = &arr[0];
            let b = &arr[1];
            let mut out = String::new();
            find_coord_precisions(a, b, "", &mut out);
            if !out.is_empty() {
                format!("# precision {}\n", out.trim())
            } else {
                String::new()
            }
        }
        JsonVal::Obj(map) => {
            if let Some(JsonVal::Arr(features_arr)) = map.get("features") {
                if features_arr.len() >= 2 {
                    let a = &features_arr[0];
                    let b = &features_arr[1];
                    let mut out = String::new();
                    find_coord_precisions(a, b, "", &mut out);
                    if !out.is_empty() {
                        format!("# precision {}\n", out.trim())
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

pub fn find_coord_precisions(a: &JsonVal, b: &JsonVal, prefix: &str, out: &mut String) {
    match (a, b) {
        (JsonVal::Obj(ma), JsonVal::Obj(mb)) => {
            for (k, va) in ma {
                if let Some(vb) = mb.get(k) {
                    let path = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    find_coord_precisions(va, vb, &path, out);
                }
            }
        }
        (JsonVal::Arr(aa), JsonVal::Arr(ab)) => {
            if aa.len() >= 2 && ab.len() >= 2 && prefix.ends_with("coordinates") {
                for i in 0..3.min(aa.len()).min(ab.len()) {
                    if let (JsonVal::Num(na), JsonVal::Num(nb)) = (&aa[i], &ab[i]) {
                        let p = coord_precision(*na, *nb);
                        let label = ["lon", "lat", "alt"][i.min(2)];
                        out.push_str(&format!("{}={}dp ", label, p));
                    }
                }
            }
        }
        (JsonVal::Num(na), JsonVal::Num(nb)) => {
            let p = coord_precision(*na, *nb);
            if p < 15 {
                let key = match prefix.rfind('.') {
                    Some(pos) => &prefix[pos + 1..],
                    None => prefix,
                };
                if is_drop_key(key) || is_coord_key(key) {
                    return;
                }
                out.push_str(&format!("{}={}dp ", prefix, p));
            }
        }
        (JsonVal::Str(sa), JsonVal::Str(sb)) => {
            if let (Ok(na), Ok(nb)) = (sa.parse::<f64>(), sb.parse::<f64>()) {
                let p = coord_precision(na, nb);
                if p < 15 {
                    let key = match prefix.rfind('.') {
                        Some(pos) => &prefix[pos + 1..],
                        None => prefix,
                    };
                    if is_drop_key(key) || is_coord_key(key) {
                        return;
                    }
                    out.push_str(&format!("{}={}dp ", prefix, p));
                }
            }
        }
        _ => {}
    }
}

pub fn check_empty_data(src: &SourceConfig, raw: &str, now: f64, lsk: &LeapSeconds) {
    if let ExtractResult::Measurements(channels) = extract(src, raw, now, lsk)
        && channels.is_empty()
    {
        report_anomaly("Empty Data", &src.url, "extract returned no measurements");
    }
}

fn clock_now_unix() -> Option<f64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64())
}

fn clock_stale(clock: &HashMap<String, (f64, u32)>, url: &str, ttl: u64, now: f64) -> bool {
    match clock.get(url) {
        Some((fetched, failures)) => stale_after(*fetched, *failures, ttl, now),
        None => true,
    }
}

fn clock_record(clock: &mut HashMap<String, (f64, u32)>, url: &str, ok: bool) {
    let Some(now) = clock_now_unix() else {
        return;
    };
    let prev = match clock.get(url) {
        Some((_, f)) => *f,
        None => 0,
    };
    let next = if ok {
        0
    } else {
        (prev + 1).min(FETCH_VOID_CAP)
    };
    clock.insert(url.to_string(), (now, next));
}

fn shard_bounds(len: usize, idx: usize, n: usize) -> (usize, usize) {
    let start = (len as u128 * idx as u128 / n as u128) as usize;
    let end = (len as u128 * (idx + 1) as u128 / n as u128) as usize;
    (start, end)
}

pub fn ci_mode(dir: &str, shard: Option<(usize, usize)>) -> i32 {
    let env = load_env();
    let sources = if dir == "phi" {
        match std::fs::read_to_string("phi/sources.φ") {
            Ok(content) => load_sources_from(&content),
            Err(e) => {
                eprintln!("ci-mode: read phi/sources.φ: {}", e);
                return 1;
            }
        }
    } else {
        load_all_sources(dir)
    };
    let mirror_enabled = dir == "phi";
    let mut lsk: Option<LeapSeconds> = None;
    for src in sources.iter().filter(|s| s.format == "kernel_text") {
        if src.body.as_deref() != Some("naif0012") {
            continue;
        }
        if let Some(text) = fetch_one(&src.url, None, &[], src.ttl, machine_now_tdb()) {
            lsk = crate::lsk::parse(&text);
        }
    }
    if lsk.is_none()
        && let Some(text) = fetch_one(
            "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls",
            None,
            &[],
            NAIF_LSK_TTL_SECS,
            machine_now_tdb(),
        )
    {
        lsk = crate::lsk::parse(&text);
    }
    let now_tdb: Option<f64> = lsk.as_ref().and_then(|l| l.system_now_tdb());
    let (shard_start, shard_end) = match shard {
        Some((idx, n)) => shard_bounds(sources.len(), idx, n),
        None => (0, sources.len()),
    };
    let shard_sources = &sources[shard_start..shard_end];
    let total = shard_sources.len();
    let mut reachable = 0usize;
    let mut dead = 0usize;
    let mut pending = 0usize;
    let mut mirrored = 0u32;
    let mut fresh = 0u32;
    let mut host_void: HashSet<String> = HashSet::new();
    let clock_path = content_cache("origin_clock.φ");
    let mut clock = load_origin_clock(&clock_path);
    for src in shard_sources {
        if src.url.starts_with("https://github.com/omegaflow/sources")
            || src.format == "ephemeris_binary"
            || src.format == "catalog_dastcom"
            || src.format == "csv_zip"
            || src.format == "igra_zip"
            || src.format == "kernel_text"
            || src.format == "opendap"
            || src.format == "tar_gz_yaml"
        {
            continue;
        }
        if src.format == "reference" {
            let bytes = match fetch_raw_bytes(&src.url, src.ttl) {
                Some(b) => b,
                None => {
                    eprintln!("ci-mode: {} reference fetch returned void", src.url);
                    report_anomaly("API Unreachable", &src.url, "reference fetch returned void");
                    dead += 1;
                    continue;
                }
            };
            if let Some(pin) = src.sha256.as_deref() {
                let measured = crate::archivar::sha256::sha256_hex(&bytes);
                if measured != pin {
                    eprintln!(
                        "ci-mode: {} reference sha256 drift (measured {}, registered {})",
                        src.url, measured, pin
                    );
                    report_anomaly(
                        "Sha256 Drift",
                        &src.url,
                        &format!("measured {measured}, registered {pin}"),
                    );
                    dead += 1;
                    continue;
                }
            }
            let Some(netloc) = extract_netloc(&src.url) else {
                dead += 1;
                continue;
            };
            let file_name = reference_name_from_url(&src.url);
            let tmp_path = format!("{}/{}", std::env::temp_dir().display(), file_name);
            let _ = crate::cdn::ensure_release(netloc);
            if std::fs::write(&tmp_path, &bytes).is_ok()
                && crate::cdn::upload_release(netloc, &tmp_path)
            {
                mirrored += 1;
                reachable += 1;
                eprintln!("ci-mode: {} reference ok ({} B)", src.url, bytes.len());
            } else {
                eprintln!("ci-mode: {} reference upload returned void", src.url);
                dead += 1;
            }
            let _ = std::fs::remove_file(&tmp_path);
            continue;
        }
        let headers = render_headers(&src.headers, &env);
        if headers.iter().any(|(_, v)| secret_resolves_void(v, &env)) {
            eprintln!("ci-mode: {} header secret void — pending", src.url);
            pending += 1;
            continue;
        }
        if url_is_fanout(&src.url) {
            if fanout_stations_secret_void(src, &env) {
                eprintln!("ci-mode: {} stations secret void — pending", src.url);
                pending += 1;
                continue;
            }
            mirror_stations(
                src,
                &headers,
                &mut mirrored,
                &mut reachable,
                &mut dead,
                &mut host_void,
                &mut clock,
            );
            probe_fanout(
                src,
                &headers,
                &env,
                &mut reachable,
                &mut dead,
                &mut pending,
                &mut host_void,
            );
            continue;
        }
        if url_has_template(&src.url) {
            probe_template(src, &headers, &env, &mut reachable, &mut dead);
            continue;
        }
        if secret_resolves_void(&src.url, &env) {
            eprintln!("ci-mode: {} secret void — pending", src.url);
            pending += 1;
            continue;
        }
        if src.url.contains('{') {
            let resolved = resolve_secret(&src.url, &env);
            match fetch_raw(&resolved, None, &headers, src.ttl) {
                Some(r) if parse_json(&r).is_some() => {
                    reachable += 1;
                    eprintln!("ci-mode: {} JSON ok (live-only, secret in URL)", src.url);
                }
                Some(_) => {
                    eprintln!("ci-mode: {} JSON parse void", src.url);
                    dead += 1;
                }
                None => {
                    eprintln!("ci-mode: fetch returned void for {}", src.url);
                    dead += 1;
                }
            }
            continue;
        }
        let netloc = extract_netloc(&src.url);
        let manifest = cdn_manifest_map();
        let name = match manifest.get(&src.url).cloned() {
            Some(n) => n,
            None => source_name_from_url(&src.url),
        };
        let cache_path = match (&netloc, &name) {
            (Some(nl), nm) if !nm.is_empty() => Some(cache_path_for(nl, nm)),
            _ => None,
        };
        let now = clock_now_unix();
        let cache_held = cache_path
            .as_ref()
            .is_some_and(|cp| std::path::Path::new(cp).is_file())
            && now.is_some_and(|n| !clock_stale(&clock, &src.url, src.ttl, n));
        if cache_held {
            fresh += 1;
            continue;
        }
        let raw = match fetch_raw(&src.url, None, &headers, src.ttl) {
            Some(r) => r,
            None => {
                eprintln!("ci-mode: fetch returned void for {}", src.url);
                report_anomaly("API Unreachable", &src.url, "fetch returned void");
                dead += 1;
                if cache_path.is_some() {
                    clock_record(&mut clock, &src.url, false);
                }
                continue;
            }
        };
        if parse_json(&raw).is_some() {
            reachable += 1;
            if cache_path.is_some() {
                clock_record(&mut clock, &src.url, true);
            }
            if let (Some(l), Some(now)) = (&lsk, now_tdb) {
                check_empty_data(src, &raw, now, l);
            }
            eprintln!("ci-mode: {} JSON ok", src.url);
            if let Some(cp) = &cache_path {
                if let Some(parent) = std::path::Path::new(cp).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(cp, &raw);
            }
            if mirror_enabled && let Some(netloc) = extract_netloc(&src.url) {
                let name = match manifest.get(&src.url).cloned() {
                    Some(n) => n,
                    None => source_name_from_url(&src.url),
                };
                let tmp_path = cache_path_for(netloc, &name);
                if std::fs::write(&tmp_path, &raw).is_ok()
                    && crate::cdn::upload_release(netloc, &tmp_path)
                {
                    mirrored += 1;
                }
            }
        } else {
            eprintln!("ci-mode: {} JSON parse void", src.url);
            report_anomaly("Malformed Data", &src.url, "JSON parse void");
            dead += 1;
            if cache_path.is_some() {
                clock_record(&mut clock, &src.url, false);
            }
        }
    }
    eprintln!(
        "ci-mode: {}/{} reachable, {} dead, {} pending (secret void), {} mirrored to CDN, {} fresh (ttl/Φ gate), mirror={}",
        reachable, total, dead, pending, mirrored, fresh, mirror_enabled
    );
    let anomalies = take_anomalies();
    if !anomalies.is_empty() {
        if std::env::var("GH_TOKEN").is_ok() {
            let date = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(d) => {
                    let (y, m, d) = days_to_ymd(d.as_secs() / 86400);
                    format!("{}-{:02}-{:02}", y, m, d)
                }
                Err(_) => "clock-unavailable".to_string(),
            };
            let title = format!("[Automated CI Report] Omegaflow Anomalies ({})", date);
            let body = anomaly_issue_body(&anomalies);
            let already_open = match Command::new("gh")
                .arg("issue")
                .arg("list")
                .arg("--repo")
                .arg("omegaflow/omegaflow")
                .arg("--state")
                .arg("open")
                .arg("--label")
                .arg("anomaly-report")
                .arg("--json")
                .arg("title")
                .arg("--jq")
                .arg(".[].title")
                .output()
            {
                Ok(o) => String::from_utf8_lossy(&o.stdout).contains(&title),
                Err(_) => false,
            };
            if already_open {
                eprintln!(
                    "ci-mode: anomaly issue already open ({}) — no new issue",
                    title
                );
            } else {
                match Command::new("gh")
                    .arg("issue")
                    .arg("create")
                    .arg("--repo")
                    .arg("omegaflow/omegaflow")
                    .arg("--title")
                    .arg(&title)
                    .arg("--label")
                    .arg("anomaly-report")
                    .arg("--body")
                    .arg(&body)
                    .output()
                {
                    Ok(o) if o.status.success() => {
                        eprintln!(
                            "ci-mode: anomaly issue created ({} anomalies)",
                            anomalies.len()
                        )
                    }
                    Ok(o) => eprintln!("ci-mode: gh issue create exited {:?}", o.status.code()),
                    Err(e) => eprintln!("ci-mode: gh issue create: {}", e),
                }
            }
        } else {
            eprintln!(
                "ci-mode: {} anomalies, GH_TOKEN absent — the report goes to the console (no issue register)",
                anomalies.len()
            );
            for a in &anomalies {
                eprintln!("anomaly: {} | {} | {}", a.category, a.url, a.details);
            }
        }
    }
    save_origin_clock(&clock_path, &clock);
    0
}
pub fn fanout_stations_secret_void(src: &SourceConfig, env: &HashMap<String, String>) -> bool {
    src.url.contains('{')
        && match src.stations_url.as_deref() {
            Some(u) => secret_resolves_void(u, env),
            None => false,
        }
}

pub fn mirror_stations(
    src: &SourceConfig,
    headers: &[(String, String)],
    mirrored: &mut u32,
    reachable: &mut usize,
    dead: &mut usize,
    host_void: &mut HashSet<String>,
    clock: &mut HashMap<String, (f64, u32)>,
) {
    let Some(stations_url) = &src.stations_url else {
        return;
    };
    if url_has_template(stations_url) {
        return;
    }
    let Some(netloc) = extract_netloc(stations_url) else {
        return;
    };
    if host_void.contains(netloc) {
        eprintln!("ci-mode: stations {} host void — pending", stations_url);
        return;
    }
    let name = source_name_from_url(stations_url);
    let cache_path = cache_path_for(netloc, &name);
    let now = clock_now_unix();
    let cache_held = std::path::Path::new(&cache_path).is_file()
        && now.is_some_and(|n| !clock_stale(clock, stations_url, src.ttl, n));
    if cache_held {
        return;
    }
    match fetch_raw(stations_url, None, headers, src.ttl) {
        Some(raw) => {
            if parse_json(&raw).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: stations {} JSON ok", stations_url);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::write(&cache_path, &raw).is_ok()
                    && crate::cdn::upload_release(netloc, &cache_path)
                {
                    *mirrored += 1;
                }
                clock_record(clock, stations_url, true);
            } else {
                eprintln!("ci-mode: stations {} JSON parse void", stations_url);
                *dead += 1;
                clock_record(clock, stations_url, false);
            }
        }
        None => {
            eprintln!("ci-mode: stations fetch void {}", stations_url);
            host_void.insert(netloc.to_string());
            *dead += 1;
            clock_record(clock, stations_url, false);
        }
    }
}

pub fn probe_fanout(
    src: &SourceConfig,
    headers: &[(String, String)],
    env: &HashMap<String, String>,
    reachable: &mut usize,
    dead: &mut usize,
    pending: &mut usize,
    host_void: &mut HashSet<String>,
) {
    let Some(stations_url) = &src.stations_url else {
        *pending += 1;
        return;
    };
    let stations_url = if url_has_template(stations_url) {
        match ci_probe_render(stations_url, frame_anchor(&src.frame), env) {
            Some(u) => u,
            None => {
                *pending += 1;
                return;
            }
        }
    } else {
        resolve_secret(stations_url, env)
    };
    let Some(netloc) = extract_netloc(&stations_url) else {
        return;
    };
    if host_void.contains(netloc) {
        eprintln!(
            "ci-mode: fanout stations {} host void — pending",
            stations_url
        );
        *pending += 1;
        return;
    }
    let raw = match fetch_raw(&stations_url, None, headers, 86400) {
        Some(r) => r,
        None => {
            eprintln!("ci-mode: fanout stations void {}", stations_url);
            host_void.insert(netloc.to_string());
            *dead += 1;
            return;
        }
    };
    let stations = match parse_json(&raw) {
        Some(j) => parse_station_entries(&j, src),
        None => parse_stations_xml(&raw),
    };
    let Some(first) = stations.first() else {
        eprintln!("ci-mode: fanout no stations {}", stations_url);
        *dead += 1;
        return;
    };
    let probe_url = resolve_secret(&src.url.replace("{station}", &first.id), env)
        .replace("{nearest_station}", &first.id);
    match fetch_raw(&probe_url, None, headers, src.ttl) {
        Some(body) => {
            if parse_json(&body).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: fanout probe {} JSON ok", probe_url);
            } else {
                eprintln!("ci-mode: fanout probe {} JSON parse void", probe_url);
                *dead += 1;
            }
        }
        None => {
            eprintln!("ci-mode: fanout probe void {}", probe_url);
            *dead += 1;
        }
    }
}

pub fn probe_template(
    src: &SourceConfig,
    headers: &[(String, String)],
    env: &HashMap<String, String>,
    reachable: &mut usize,
    dead: &mut usize,
) {
    let anchor = frame_anchor(&src.frame);
    let probe_url = match ci_probe_render(&src.url, anchor, env) {
        Some(u) => u,
        None => return,
    };
    if secret_resolves_void(&probe_url, env) {
        return;
    }
    match fetch_raw(&probe_url, None, headers, src.ttl) {
        Some(body) => {
            if parse_json(&body).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: template probe {} JSON ok", probe_url);
            } else {
                eprintln!("ci-mode: template probe {} JSON parse void", probe_url);
                *dead += 1;
            }
        }
        None => {
            eprintln!("ci-mode: template probe void {}", probe_url);
            *dead += 1;
        }
    }
}

pub fn draft_url_mode(path: &str, env: &HashMap<String, String>, fetchone: bool) -> i32 {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("--draft: read {}: {}", path, e);
            return 1;
        }
    };
    let urls: Vec<String> = content
        .lines()
        .map(|l| {
            let t = l.trim();
            let u = if t.starts_with("live ") || t.starts_with("candidate ") {
                t.split_whitespace()
                    .find(|w| w.starts_with("http"))
                    .unwrap_or("")
            } else {
                t
            };
            u.to_string()
        })
        .filter(|u| u.starts_with("http"))
        .collect();
    let total = urls.len();
    let out_lock = std::sync::Mutex::new(String::new());
    let learned_lock = std::sync::Mutex::new(HashMap::<String, String>::new());
    let drafted = std::sync::atomic::AtomicUsize::new(0);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let workers = 8.min(total);
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if i >= total {
                        break;
                    }
                    let url = resolve_secret(&urls[i], env);
                    let raw = if fetchone {
                        fetch_one(&url, None, &[], 3600, machine_now_tdb())
                    } else {
                        fetch_raw_probe(&url, None, &[])
                    };
                    if let Some(body) = raw
                        && let Some(parsed) = parse_json(&body)
                    {
                        let tap_flat = tap_to_json(&parsed);
                        let effective = match tap_flat.as_ref() {
                            Some(flat) => flat,
                            None => &parsed,
                        };
                        let mut fields = String::new();
                        let mut coords = String::new();
                        let mut map_path: Option<String> = None;
                        let mut budget = 48usize;
                        if !hapi_draft_fields(&url, effective, env, &mut fields) {
                            walk_json_probe(
                                effective,
                                "",
                                &mut fields,
                                &mut coords,
                                &mut map_path,
                                &mut budget,
                            );
                        }
                        let ttl = derive_ttl(&url, &body, env);
                        let (frame, reason) = derive_frame(effective, &coords);
                        if !frame.is_empty()
                            && let Some(rk) = route_key(&urls[i])
                        {
                            learned_lock
                                .lock()
                                .unwrap()
                                .entry(rk)
                                .or_insert_with(|| frame.trim_end().to_string());
                        }
                        let mut block = format!("url {}\n", urls[i]);
                        if let Some(t) = ttl {
                            block.push_str(&format!("ttl {}\n", t));
                        }
                        if tap_flat.is_some() {
                            block.push_str("format tap\n");
                        }
                        block.push_str(&frame);
                        if let Some(ref mp) = map_path
                            && !coords.is_empty()
                        {
                            let container = if coords.contains("ra ") || coords.contains("dec ") {
                                "cmap"
                            } else {
                                "map"
                            };
                            block.push_str(&format!("{} {}\n", container, mp));
                        }
                        if !coords.is_empty() {
                            block.push_str(&coords);
                        }
                        if !fields.is_empty() {
                            block.push_str(&fields);
                        }
                        let mut out = format!("# frame: {}\n", reason);
                        out.push_str(&block);
                        out.push('\n');
                        out_lock.lock().unwrap().push_str(&out);
                        drafted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
            });
        }
    });
    let drafted = drafted.load(std::sync::atomic::Ordering::Relaxed);
    std::fs::create_dir_all("phi/pipeline").ok();
    std::fs::write(
        "phi/pipeline/probe_drafts.φ",
        out_lock.into_inner().unwrap(),
    )
    .ok();
    let learned = learned_lock.into_inner().unwrap();
    eprintln!(
        "--draft: {} candidates, {} blocks drafted, {} frames learned → phi/pipeline/probe_drafts.φ + phi/pipeline/frame_learned.φ",
        total,
        drafted,
        learned.len()
    );
    learn_frames(&learned);
    0
}

pub fn draft_context_mode(path: &str) -> i32 {
    let drafts = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("--draft-context: read {}: {}", path, e);
            return 1;
        }
    };
    let mut context_map: HashMap<String, String> = HashMap::new();
    for dir in ["phi/pipeline/catalog", "phi/pipeline"] {
        if let Ok(rd) = std::fs::read_dir(dir) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                let relevant = (dir == "phi/pipeline/catalog" && name.ends_with(".φ"))
                    || (dir == "phi/pipeline"
                        && name.starts_with("weights_")
                        && name.ends_with(".txt"));
                if !relevant {
                    continue;
                }
                if let Ok(c) = std::fs::read_to_string(e.path()) {
                    for l in c.lines() {
                        let t = l.trim();
                        if let Some(pos) = t.find("http") {
                            let u = t[pos..]
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .trim_end_matches([',', '|', ';']);
                            if u.starts_with("http") {
                                context_map
                                    .entry(u.to_string())
                                    .or_insert_with(|| t.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    let registry = build_frame_registry();
    let mut reg = String::from(
        "# frame-registry — route (host/path, query stripped) → frame, self-learning from sources.φ + dead_sources.φ + blocked_sources.φ + frame_learned.φ\n",
    );
    let mut reg_keys: Vec<(&String, &String)> = registry.iter().collect();
    reg_keys.sort();
    for (nl, f) in reg_keys {
        reg.push_str(&format!("{} | {}\n", nl, f));
    }
    if std::fs::write("phi/pipeline/frame_registry.φ", reg).is_err() {
        eprintln!("write phi/pipeline/frame_registry.φ: the register does not remember");
    }
    let mut out = String::new();
    let mut celestial = 0usize;
    let mut terrestrial = 0usize;
    let mut pending = 0usize;
    for block in drafts.split("\n\n") {
        let b = block.trim();
        if b.is_empty() {
            continue;
        }
        let mut url = "";
        let mut is_pending = false;
        for l in b.lines() {
            if l.starts_with("url ") {
                url = l.trim_start_matches("url ").trim();
            }
            if l == "# frame: frame pending" {
                is_pending = true;
            }
        }
        if !is_pending || url.is_empty() {
            out.push_str(b);
            out.push_str("\n\n");
            continue;
        }
        let context = match context_map.get(url) {
            Some(c) => c.clone(),
            None => String::new(),
        };
        let (frame, reason) = draft_frame_guess(url, &context, &registry);
        if frame.is_empty() {
            pending += 1;
            out.push_str(b);
            out.push_str("\n\n");
            continue;
        }
        let mut lines: Vec<String> = Vec::new();
        for l in b.lines() {
            if l == "# frame: frame pending" {
                lines.push(format!("# frame: {}", reason));
                continue;
            }
            lines.push(l.to_string());
            if l.starts_with("ttl ") {
                lines.push(frame.trim_end().to_string());
            }
        }
        if frame.starts_with("at sun") {
            celestial += 1;
        } else {
            terrestrial += 1;
        }
        out.push_str(&lines.join("\n"));
        out.push_str("\n\n");
    }
    std::fs::create_dir_all("phi/pipeline").ok();
    if std::fs::write("phi/pipeline/probe_drafts_enriched.φ", out).is_err() {
        eprintln!("write phi/pipeline/probe_drafts_enriched.φ: the register does not remember");
    }
    eprintln!(
        "--draft-context: {} pending → {} celestial, {} terrestrial, {} stay pending → phi/pipeline/probe_drafts_enriched.φ",
        celestial + terrestrial + pending,
        celestial,
        terrestrial,
        pending
    );
    0
}

pub fn gate_learn_mode() -> i32 {
    let mut delta: Vec<(i32, String, String)> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Ok(content) = std::fs::read_to_string("phi/sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ")
                && let Some(nl) = extract_netloc(rest.trim())
                && seen.insert(format!("+{}", nl))
            {
                delta.push((4, "-".to_string(), nl.to_string()));
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/dead_sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ")
                && let Some(nl) = extract_netloc(rest.trim())
                && seen.insert(format!("-{}", nl))
            {
                delta.push((-4, "-".to_string(), nl.to_string()));
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/declined_sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ")
                && let Some(nl) = extract_netloc(rest.trim())
                && seen.insert(format!("-{}", nl))
            {
                delta.push((-4, "-".to_string(), nl.to_string()));
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/blocked_sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ")
                && let Some(nl) = extract_netloc(rest.trim())
                && seen.insert(format!("b{}", nl))
            {
                delta.push((-2, "b".to_string(), nl.to_string()));
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/witnesses.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ")
                && let Some(nl) = extract_netloc(rest.trim())
                && seen.insert(format!("w{}", nl))
            {
                delta.push((2, "w".to_string(), nl.to_string()));
            }
        }
    }
    let (mut pos, mut neg) = (0usize, 0usize);
    for (w, _, _) in &delta {
        if *w > 0 {
            pos += 1;
        } else {
            neg += 1;
        }
    }
    let mut d = String::from(
        "# gate-delta — netloc weights, self-learning from sources.φ (+) + dead_sources.φ (−) + blocked_sources.φ (b) + witnesses.φ (w)\n",
    );
    for (w, f, tag) in &delta {
        d.push_str(&format!("{} {} {}\n", w, f, tag));
    }
    if std::fs::write("phi/pipeline/library_gate_delta.φ", d).is_err() {
        eprintln!("write phi/pipeline/library_gate_delta.φ: the register does not remember");
    }
    let library = match std::fs::read_to_string("phi/pipeline/library.φ") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("learn-gate: read phi/pipeline/library.φ: {}", e);
            String::new()
        }
    };
    let delta_lines: Vec<String> = delta
        .iter()
        .map(|(w, f, tag)| format!("{} {} {}", w, f, tag))
        .collect();
    let mut seen2: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out = String::new();
    for line in library
        .lines()
        .chain(delta_lines.iter().map(|s| s.as_str()))
    {
        if line.starts_with('#') {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, char::is_whitespace).collect();
        if parts.len() < 3 {
            continue;
        }
        if seen2.insert(parts[2].to_string()) {
            out.push_str(line);
            out.push('\n');
        }
    }
    if std::fs::write("phi/pipeline/library.φ", out).is_err() {
        eprintln!("write phi/pipeline/library.φ: the register does not remember");
    }
    eprintln!(
        "--learn-gate: {} netloc-Gewichte ({} positiv, {} negativ) → library.φ + library_gate_delta.φ",
        delta.len(),
        pos,
        neg
    );
    0
}

pub fn url_probe_mode(path: &str, env: &HashMap<String, String>, fetchone: bool) -> i32 {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("--urls: read {}: {}", path, e);
            return 1;
        }
    };
    let urls: Vec<String> = content
        .lines()
        .map(|l| {
            let t = l.trim();
            let u = if t.starts_with("candidate ") {
                t.trim_start_matches("candidate ")
            } else {
                t
            };
            let word = u.split_whitespace().next().unwrap_or("");
            word.to_string()
        })
        .filter(|u| u.starts_with("http"))
        .collect();
    let total = urls.len();
    let live = std::sync::atomic::AtomicUsize::new(0);
    let void = std::sync::atomic::AtomicUsize::new(0);
    let live_lock = std::sync::Mutex::new(String::new());
    let void_lock = std::sync::Mutex::new(String::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    let workers = 8.min(total);
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if i >= total {
                        break;
                    }
                    let url = resolve_secret(&urls[i], env);
                    let raw = if fetchone {
                        fetch_one(&url, None, &[], 3600, machine_now_tdb())
                    } else {
                        fetch_raw_probe(&url, None, &[])
                    };
                    match raw {
                        Some(body) => {
                            let kind = if parse_json(&body).is_some() {
                                "json"
                            } else if body.trim_start().starts_with('<') {
                                "html"
                            } else {
                                "text"
                            };
                            live.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            live_lock
                                .lock()
                                .unwrap()
                                .push_str(&format!("live {} | {}\n", kind, urls[i]));
                        }
                        None => {
                            void.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            void_lock
                                .lock()
                                .unwrap()
                                .push_str(&format!("void {}\n", urls[i]));
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
            });
        }
    });
    let live = live.load(std::sync::atomic::Ordering::Relaxed);
    let void = void.load(std::sync::atomic::Ordering::Relaxed);
    std::fs::create_dir_all("phi/pipeline").ok();
    std::fs::write(
        "phi/pipeline/probe_live.txt",
        live_lock.into_inner().unwrap(),
    )
    .ok();
    std::fs::write(
        "phi/pipeline/probe_url_void.txt",
        void_lock.into_inner().unwrap(),
    )
    .ok();
    eprintln!(
        "--urls: {} checked, {} live, {} void → phi/pipeline/probe_live.txt + probe_url_void.txt",
        total, live, void
    );
    0
}

#[cfg(test)]
mod probe_classify_tests {
    use super::probe_classify;

    #[test]
    fn ndbc_buoy_fields_classify() {
        assert_eq!(probe_classify("gst_m_s"), ("advective", "m/s", 60.0));
        assert_eq!(probe_classify("dpd_s"), ("acoustic", "s", 21600.0));
        assert_eq!(probe_classify("apd_s"), ("acoustic", "s", 21600.0));
        assert_eq!(probe_classify("ptdy_hpa"), ("acoustic", "hPa", 21600.0));
        assert_eq!(probe_classify("tide_ft"), ("gravity", "m", 3600.0));
        assert_eq!(
            probe_classify("hydrosphere_ndbc_buoy_dominant_period"),
            ("acoustic", "s", 21600.0)
        );
    }

    #[test]
    fn argo_biogeochemical_fields_classify() {
        assert_eq!(
            probe_classify("argo_dac_bgc_doxy_umol_kg"),
            ("diffusion", "micromole/kg", 604800.0)
        );
        assert_eq!(
            probe_classify("argo_dac_bgc_nitrate_umol_kg"),
            ("diffusion", "micromole/kg", 604800.0)
        );
        assert_eq!(
            probe_classify("argo_dac_bgc_chla_mg_m3"),
            ("diffusion", "mg/m3", 604800.0)
        );
        assert_eq!(
            probe_classify("argo_dac_bgc_bbp700_m1"),
            ("diffusion", "m-1", 604800.0)
        );
    }

    #[test]
    fn space_weather_fields_classify() {
        assert_eq!(
            probe_classify("streamflow_cfs"),
            ("advective", "cfs", 3600.0)
        );
        assert_eq!(probe_classify("rpw_e_y"), ("electric", "mV/m", 3600.0));
        assert_eq!(
            probe_classify("solo_rpw_e_rms_vm"),
            ("electric", "V/m", 3600.0)
        );
        assert_eq!(probe_classify("rave_teff_k"), ("thermal", "K", 604800.0));
        assert_eq!(probe_classify("goes_abi_radiance"), ("em", "W/m2", 3600.0));
        assert_eq!(
            probe_classify("magnetosphere_total_field_nt"),
            ("em", "nT", 86400.0)
        );
        assert_eq!(
            probe_classify("intermagnet_xyz_x_nt"),
            ("em", "nT", 86400.0)
        );
        assert_eq!(
            probe_classify("hydrosphere_drifter_sst_k"),
            ("thermal", "K", 360.0)
        );
    }

    #[test]
    fn metrology_anchor_form_pending_or_mirrors_register() {
        assert_eq!(
            probe_classify("eop_iers_ut1_utc_s"),
            ("gravity", "s", 86400.0)
        );
        assert_eq!(
            probe_classify("eop_iers_polar_motion_x_arcsec"),
            ("gravity", "arcsec", 86400.0)
        );
        assert_eq!(probe_classify("ut1_utc"), ("gravity", "s", 86400.0));
        assert_eq!(probe_classify("pmx"), ("gravity", "arcsec", 86400.0));
        assert_eq!(probe_classify("ul_phase_cycles"), ("em", "cycle", 604800.0));
        assert_eq!(
            probe_classify("polar_angle_cycles"),
            ("em", "cycle", 604800.0)
        );
        assert_eq!(probe_classify("l1"), ("em", "cycle", 604800.0));
        assert_eq!(probe_classify("angle_a"), ("em", "rad", 604800.0));
        assert_eq!(probe_classify("las_x_icrs_m"), ("em", "m", 86400.0));
        assert_eq!(probe_classify("las_z_icrs_m"), ("em", "m", 86400.0));
        assert_eq!(
            probe_classify("kcdc_grande_x_core_m"),
            ("em", "m", 604800.0)
        );
        assert_eq!(
            probe_classify("kcdc_grande_shower_age"),
            ("em", "1", 604800.0)
        );
        assert_eq!(
            probe_classify("las_classification"),
            ("diffusion", "1", 86400.0)
        );
        assert_eq!(
            probe_classify("cors_rinex_l1_cycle"),
            ("em", "cycle", 604800.0)
        );
        assert_eq!(
            probe_classify("wds_separation_arcsec"),
            ("em", "arcsec", 604800.0)
        );
    }

    #[test]
    fn arrival_direction_is_acoustic_deg() {
        assert_eq!(
            probe_classify("bgr_infrasound_back_azimuth_deg"),
            ("acoustic", "deg", 300.0)
        );
        assert_eq!(
            probe_classify("kcdc_kascade_zenith_deg"),
            ("acoustic", "deg", 604800.0)
        );
        assert_eq!(
            probe_classify("kcdc_grande_azimuth_deg"),
            ("acoustic", "deg", 604800.0)
        );
    }

    #[test]
    fn solar_wind_flow_speed_is_advective() {
        assert_eq!(
            probe_classify("omni_solarwind_flow_speed_kms"),
            ("advective", "km/s", 86400.0)
        );
        assert_eq!(
            probe_classify("psp_solarwind_flow_speed_kms"),
            ("advective", "km/s", 3600.0)
        );
        assert_eq!(
            probe_classify("cme_plane_of_sky_speed_km_s"),
            ("advective", "km/s", 3600.0)
        );
    }

    #[test]
    fn registry_membership_gate_closes_unit_leaks() {
        assert_eq!(
            probe_classify("wod_oxygen_ml_l"),
            ("diffusion", "ml/l", 604800.0)
        );
        assert_eq!(
            probe_classify("cosmic_ro_refractivity_n_units"),
            ("em", "N-units", 604800.0)
        );
        assert_eq!(probe_classify("0.speed"), ("advective", "km/s", 360.0));
        assert_eq!(
            probe_classify("cryosat_magnetic_field_intensity_nt"),
            ("em", "nT", 4.0)
        );
        assert_eq!(
            probe_classify("cme_plane_of_sky_speed_km_s"),
            ("advective", "km/s", 3600.0)
        );
        assert_eq!(probe_classify("ptdy_hpa"), ("acoustic", "hPa", 21600.0));
    }

    #[test]
    fn infrasound_and_lidar_physical_components_classify() {
        assert_eq!(
            probe_classify("bgr_infrasound_back_azimuth_deg"),
            ("acoustic", "deg", 300.0)
        );
        assert_eq!(
            probe_classify("bgr_infrasound_rms_amplitude_pa"),
            ("acoustic", "pa", 300.0)
        );
        assert_eq!(probe_classify("las_intensity_dn"), ("em", "1", 86400.0));
    }

    #[test]
    fn stellar_and_seismic_magnitude_part_ways() {
        assert_eq!(probe_classify("comet_h_mag"), ("em", "mag", 604800.0));
        assert_eq!(probe_classify("wd_vmag"), ("em", "mag", 604800.0));
        assert_eq!(probe_classify("wds_primary_mag"), ("em", "mag", 604800.0));
        assert_eq!(
            probe_classify("alerce_ztf_detection_magpsf"),
            ("em", "mag", 604800.0)
        );
        assert_eq!(
            probe_classify("geosphere_earthquake_mag"),
            ("seismic-body", "Mw", 3600.0)
        );
    }

    #[test]
    fn sky_maps_and_surveys_classify() {
        assert_eq!(probe_classify("fugin_moment0"), ("em", "K.m/s", 31536000.0));
        assert_eq!(
            probe_classify("lhaaso_sky1"),
            ("em", "m-2.s-1.tev-1", 31536000.0)
        );
        assert_eq!(probe_classify("hess_dl3"), ("em", "tev", 31536000.0));
        assert_eq!(
            probe_classify("gw250207_115645"),
            ("gravity", "1", 31536000.0)
        );
    }

    #[test]
    fn dissolved_constituents_classify() {
        assert_eq!(
            probe_classify("woudc_total_ozone_column_du"),
            ("diffusion", "DU", 86400.0)
        );
        assert_eq!(
            probe_classify("dc_wq_turbidity_ntu"),
            ("diffusion", "ntu", 3600.0)
        );
        assert_eq!(
            probe_classify("ndbc_dart_21414_height_m"),
            ("gravity", "m", 3600.0)
        );
    }

    #[test]
    fn unclassified_stays_pending() {
        assert_eq!(probe_classify("sommerfeld_ratio"), ("UNCERTAIN", "", 0.0));
    }

    #[test]
    fn catalog_and_observable_fields_classify() {
        assert_eq!(
            probe_classify("bat_fluence_erg_cm2"),
            ("em", "erg/cm2", 604800.0)
        );
        assert_eq!(
            probe_classify("radnet_activity_bq_l"),
            ("em", "bq/l", 3600.0)
        );
        assert_eq!(
            probe_classify("kcdc_kascade_energy_ev"),
            ("em", "eV", 604800.0)
        );
        assert_eq!(probe_classify("mars_dust_cdod"), ("em", "1", 86400.0));
        assert_eq!(probe_classify("extinction_rv"), ("em", "1", 604800.0));
        assert_eq!(
            probe_classify("extinction_ebv_mag"),
            ("em", "mag", 604800.0)
        );
        assert_eq!(
            probe_classify("geosphere_fireball_radiated_energy_e10j"),
            ("em", "e10j", 3600.0)
        );
        assert_eq!(
            probe_classify("geosphere_fireball_impact_energy_kt"),
            ("em", "kt_tnt", 3600.0)
        );
        assert_eq!(
            probe_classify("champ_absolute_vertical_tec_tecu"),
            ("em", "TECU", 10.0)
        );
        assert_eq!(probe_classify("ned_redshift_z"), ("em", "1", 86400.0));
        assert_eq!(
            probe_classify("pulsar_dm_pccm3"),
            ("em", "pc/cm3", 604800.0)
        );
        assert_eq!(
            probe_classify("planet_mass"),
            ("gravity", "M_earth", 604800.0)
        );
        assert_eq!(
            probe_classify("cb_primary_mass"),
            ("gravity", "M_sun", 604800.0)
        );
        assert_eq!(probe_classify("corot_logg"), ("gravity", "logg", 604800.0));
    }

    #[test]
    fn water_and_soil_constituents_classify() {
        assert_eq!(
            probe_classify("aeronet_precipitable_water_cm"),
            ("diffusion", "cm", 86400.0)
        );
        assert_eq!(
            probe_classify("plant_available_water_mm"),
            ("diffusion", "mm", 86400.0)
        );
        assert_eq!(
            probe_classify("icesat2_atl03_h_ph_m"),
            ("gravity", "m", 3600.0)
        );
        assert_eq!(
            probe_classify("noaa_keo_psal_psu"),
            ("diffusion", "psu", 86400.0)
        );
    }

    #[test]
    fn residual_pending_names_classify() {
        assert_eq!(
            probe_classify("solar_flare_xray_intensity"),
            ("em", "W/m2", 3600.0)
        );
        assert_eq!(
            probe_classify("solar_flare_x_class_latest"),
            ("em", "1e-4W/m2", 3600.0)
        );
        assert_eq!(
            probe_classify("magnetosphere_kp_a_running"),
            ("em", "1", 3600.0)
        );
        assert_eq!(probe_classify("magnetosphere_kp_3h"), ("em", "1", 3600.0));
        assert_eq!(probe_classify("safecast_cpm"), ("em", "cpm", 3600.0));
        assert_eq!(
            probe_classify("oulu_neutron_corr_for_eff"),
            ("em", "%", 3600.0)
        );
        assert_eq!(
            probe_classify("meteo_rasuwa_cloud_cover"),
            ("diffusion", "%", 300.0)
        );
        assert_eq!(
            probe_classify("meteo_rasuwa_cloud_cover_high"),
            ("diffusion", "%", 300.0)
        );
        assert_eq!(
            probe_classify("meteo_rasuwa_dew_point_2m"),
            ("thermal", "C", 3600.0)
        );
        assert_eq!(
            probe_classify("meteo_rasuwa_diffuse_radiation"),
            ("em", "W/m2", 3600.0)
        );
        assert_eq!(
            probe_classify("meteo_rasuwa_direct_radiation"),
            ("em", "W/m2", 3600.0)
        );
        assert_eq!(
            probe_classify("meteo_rasuwa_direct_normal_irradiance"),
            ("em", "W/m2", 3600.0)
        );
        assert_eq!(
            probe_classify("meteo_rasuwa_leaf_wetness_probability"),
            ("diffusion", "%", 300.0)
        );
        assert_eq!(
            probe_classify("atmosphere_metar_wind_direction_deg"),
            ("advective", "deg", 60.0)
        );
        assert_eq!(
            probe_classify("hydrosphere_ndbc_buoy_mean_wave_dir"),
            ("acoustic", "deg", 60.0)
        );
        assert_eq!(
            probe_classify("hydrosphere_river_flow_cfs"),
            ("advective", "cfs", 3600.0)
        );
        assert_eq!(
            probe_classify("hydrosphere_river_stage_m"),
            ("gravity", "m", 3600.0)
        );
        assert_eq!(
            probe_classify("glm_bolide_radiant_energy_j"),
            ("em", "J", 3600.0)
        );
        assert_eq!(
            probe_classify("exosphere_ace_speed_kms"),
            ("advective", "km/s", 3600.0)
        );
        assert_eq!(
            probe_classify("exosphere_ace_dens_ncc"),
            ("diffusion", "cm-3", 3600.0)
        );
        assert_eq!(probe_classify("gps_sv_clock_bias_s"), ("em", "s", 86400.0));
        assert_eq!(
            probe_classify("hfradar_codar_2015_u_cm_s"),
            ("advective", "cm/s", 3600.0)
        );
        assert_eq!(
            probe_classify("hfradar_radial_velocity_USM_SGRV_cm_s"),
            ("advective", "cm/s", 3600.0)
        );
        assert_eq!(probe_classify("magnetar_period_s"), ("em", "s", 604800.0));
        assert_eq!(probe_classify("gcvs_period_d"), ("em", "d", 604800.0));
        assert_eq!(probe_classify("pulsar_period_s"), ("em", "s", 604800.0));
        assert_eq!(probe_classify("frb_scatter_ms"), ("em", "ms", 604800.0));
        assert_eq!(
            probe_classify("gracefo_kbr_range_rate_m_s"),
            ("gravity", "m/s", 86400.0)
        );
        assert_eq!(
            probe_classify("gracefo_kbr_range_accl_m_s2"),
            ("gravity", "m/s2", 86400.0)
        );
    }

    #[test]
    fn register_replay_reproduces_force_unit() {
        let register = include_str!("../../phi/sources.φ");
        let mut pairs: std::collections::BTreeMap<
            String,
            std::collections::BTreeSet<(String, String)>,
        > = std::collections::BTreeMap::new();
        for line in register.lines() {
            let p: Vec<&str> = line.split_whitespace().collect();
            match p.first().copied() {
                Some("field" | "first" | "last" | "lastrow") if p.len() >= 6 => {
                    pairs
                        .entry(p[1].to_string())
                        .or_default()
                        .insert((p[4].to_string(), p[5].to_string()));
                }
                _ => {}
            }
        }
        let allowlist: &[(&str, &str)] = &[
            (
                "copernicus_air_pressure_at_sea_level",
                "pressure unification → acoustic hPa; register acoustic Pa",
            ),
            (
                "cosmic_ro_pressure_hpa",
                "pressure unification → acoustic hPa; register em hPa",
            ),
            (
                "igra_air_pressure_hpa",
                "pressure unification → acoustic hPa; register advective hPa",
            ),
            (
                "noaa_gsod_slp_hpa",
                "pressure unification → acoustic hPa; register advective hPa",
            ),
            (
                "noaa_isd_slp_hpa",
                "pressure unification → acoustic hPa; register advective hPa",
            ),
            (
                "observations.seaLevelPressure",
                "pressure unification → acoustic hPa; register advective hPa",
            ),
            (
                "omni_solarwind_pressure_npa",
                "pressure unification → acoustic hPa; register advective nPa",
            ),
            (
                "pressure",
                "pressure riss (advective hPa/mb vs diffusion hPa) → acoustic hPa",
            ),
            (
                "properties.PRES",
                "pressure unification → acoustic hPa; register advective",
            ),
            (
                "properties.PTDY",
                "pressure unification → acoustic hPa; register advective",
            ),
            ("sample", "metadata DROP; register em/count"),
            (
                "soles.0.pressure",
                "pressure unification → acoustic hPa; register acoustic Pa",
            ),
            (
                "sols.6.pressure",
                "pressure unification → acoustic hPa; register acoustic Pa",
            ),
            (
                "surface_partial_pressure_of_carbon_dioxide_in_sea_water",
                "pressure unification → acoustic hPa; register diffusion uatm",
            ),
            (
                "surface_pressure",
                "pressure unification → acoustic hPa; register advective hPa",
            ),
        ];
        let allow_names: std::collections::HashSet<&str> =
            allowlist.iter().map(|(n, _)| *n).collect();
        let mut missing = 0usize;
        let mut pending = 0usize;
        for (name, rpairs) in &pairs {
            let (force, unit, _) = probe_classify(name);
            if force == "UNCERTAIN" {
                pending += 1;
            }
            let norm = crate::archivar::units::normalize_unit;
            let reproduced = rpairs
                .iter()
                .any(|(rf, ru)| force == rf.as_str() && norm(unit) == norm(ru.as_str()));
            if !reproduced && !allow_names.contains(name.as_str()) {
                missing += 1;
                eprintln!(
                    "replay mismatch {name}: classify {force}/{unit} vs register {:?}",
                    rpairs
                );
            }
        }
        assert_eq!(
            missing, 0,
            "register replay: {} names diverged and are not allowlisted; pending {}",
            missing, pending
        );
    }
}

#[cfg(test)]
mod shard_tests {
    use super::shard_bounds;

    #[test]
    fn shards_contiguous_and_covering() {
        let cases = [
            (0usize, 1usize),
            (1, 1),
            (5, 1),
            (521, 8),
            (10, 3),
            (7, 7),
            (3, 8),
            (0, 5),
        ];
        for (len, n) in cases {
            let mut expected = 0usize;
            for idx in 0..n {
                let (start, end) = shard_bounds(len, idx, n);
                assert_eq!(start, expected, "len {} n {} idx {} start", len, n, idx);
                assert!(start <= end, "len {} n {} idx {} start>end", len, n, idx);
                assert!(end <= len, "len {} n {} idx {} end>len", len, n, idx);
                expected = end;
            }
            assert_eq!(expected, len, "len {} n {} union must cover 0..len", len, n);
        }
    }
}
