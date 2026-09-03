use super::*;

pub fn angular_distance_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r1 = lat1.to_radians();
    let r2 = lat2.to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = ((r2 - r1) * 0.5).sin().powi(2) + r1.cos() * r2.cos() * (dlon * 0.5).sin().powi(2);
    (2.0 * a.sqrt().asin()).to_degrees()
}

pub fn port_field_synth(
    directive: &str,
    force: &str,
    key: &str,
    name: &str,
    tau: Option<f64>,
) -> Option<String> {
    let (kernel, f) = default_kernel_for(force)?;
    let tau = tau.filter(|t| t.is_finite() && *t > 0.0)?;
    Some(format!(
        "{} {} {} {} {} 1 {} 0.0 0.0\n",
        directive, key, name, kernel, f, tau
    ))
}

pub fn port_block(block: &str) -> String {
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
                head.push(t.to_string());
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
            "map" | "cmap" | "rows" => {
                let arg = parts.get(1).copied().unwrap_or(".");
                map_line = Some(format!("{} {}", parts[0], arg));
            }
            "lat_key" if parts.len() >= 2 => lat_key = Some(parts[1].to_string()),
            "lon_key" if parts.len() >= 2 => lon_key = Some(parts[1].to_string()),
            "alt_key" if parts.len() >= 2 => alt_key = Some(parts[1].to_string()),
            "epoch_key" if parts.len() >= 2 => epoch_key = Some(parts[1].to_string()),
            "field" | "field_in" | "first" | "last" | "count" | "path" | "deep" | "last_row"
            | "last_line" | "last_obj" | "geojson" | "regex" => {
                raw_extracts.push(t.to_string());
            }
            _ => {}
        }
    }

    let celestial = matches!(
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
        out.push_str("on earth 0 0\n");
    } else if let (Some(lat), Some(lon)) = (lat, lon) {
        match alt {
            Some(a) => out.push_str(&format!("on earth {} {} {}\n", lat, lon, a)),
            None => out.push_str(&format!("on earth {} {}\n", lat, lon)),
        }
    }
    if let Some(m) = &map_line {
        if celestial {
            let arg = m.splitn(2, ' ').nth(1).unwrap_or(".");
            out.push_str("cmap ");
            out.push_str(arg);
            out.push('\n');
        } else {
            out.push_str(m);
            out.push('\n');
        }
    }
    if celestial {
        if let Some(k) = &lat_key {
            out.push_str("ra ");
            out.push_str(k);
            out.push('\n');
        }
        if let Some(k) = &lon_key {
            out.push_str("dec ");
            out.push_str(k);
            out.push('\n');
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
    for r in &raw_extracts {
        let parts: Vec<&str> = r.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let s = match parts[0] {
            "field" | "field_in" if parts.len() >= 3 => {
                port_field_synth("field", &force, parts[1], parts[2], None)
            }
            "first" | "last" | "count" | "path" | "deep" if parts.len() >= 3 => {
                port_field_synth(parts[0], &force, parts[1], parts[2], None)
            }
            "last_row" if parts.len() >= 3 => {
                port_field_synth("lastrow", &force, parts[1], parts[2], None)
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
                let name = parts[parts.len() - 1];
                let pat = parts[1..parts.len() - 1].join(" ");
                port_field_synth("regex", &force, &pat, name, None)
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
) {
    *total += 1;
    let conv = port_block(block);
    if !parse_sources(&conv).is_empty() {
        *parsed += 1;
        converted.push_str(&conv);
        converted.push('\n');
    }
}

pub fn port_mode(input: &str, output: &str) -> i32 {
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
    let mut in_source = false;
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("source ") {
            if !block.is_empty() {
                flush_port_block(&block, &mut converted, &mut total, &mut parsed);
                block = String::new();
            }
            in_source = true;
            block.push_str(line);
            block.push('\n');
            continue;
        }
        if t.starts_with("url ") && !in_source {
            if !block.is_empty() {
                flush_port_block(&block, &mut converted, &mut total, &mut parsed);
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
        flush_port_block(&block, &mut converted, &mut total, &mut parsed);
    }
    if std::fs::write(output, &converted).is_err() {
        eprintln!("--port: output unwritable: {}", output);
        return 1;
    }
    eprintln!(
        "--port: {} blocks converted, {} parse in the current parser → {}",
        total, parsed, output
    );
    0
}

pub fn probe_one(
    src: &SourceConfig,
    now: f64,
    lsk_ref: &LeapSeconds,
    void_eph: &HashMap<String, BodyEphemeris>,
    env: &HashMap<String, String>,
    fetchone: bool,
    precise: bool,
    lat: f64,
    lon: f64,
) -> (bool, String) {
    let url = match render_url(&src.url, 0.0, 0.0, 0.0, now, 0.0, "", void_eph, lsk_ref) {
        Some(u) => u,
        None => return (false, "# declined: time absent\n".to_string()),
    }
    .replace("{lat}", &format!("{:.6}", lat))
    .replace("{lon}", &format!("{:.6}", lon));
    let mut url = url;
    for (k, v) in live_markers() {
        url = url.replace(&k, &v);
    }
    let url = resolve_secret(&url, env);
    let url = url.replace("ZZ", "Z").replace("  ", " ");
    let headers = render_headers(&src.headers, env);
    let raw = if fetchone {
        fetch_one(&url, None, &headers, src.ttl, Some(now))
    } else {
        fetch_raw_probe(&url, None, &headers)
    };
    let parsed = raw.as_ref().and_then(|r| parse_json(r));
    let auto_ttl = raw.as_ref().and_then(|r| probe_ttl(r));
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
        walk_json_probe(&p, "", &mut fields, &mut coords, &mut map_path, &mut budget);
        if map_path.is_none() && !coords.is_empty() {
            map_path = Some(".".to_string());
        }
        let precision_lines = measure_precision(&p);
        if !precision_lines.is_empty() {
            block.push_str(&precision_lines);
        }
        if let Some(ref mp) = map_path {
            if !coords.is_empty() {
                let container = if coords.contains("ra ") || coords.contains("dec ") {
                    "cmap"
                } else {
                    "map"
                };
                block.push_str(&format!("{} {}\n", container, mp));
            }
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
    if precise && raw.is_some() {
        block.push_str(&bruteforce_precision(&url, &src.url, ttl));
    }
    let verdict = match &raw {
        Some(r) => {
            let declared_ok = match extract(src, r, now, lsk_ref) {
                ExtractResult::Measurements(v) | ExtractResult::WithEphemeris(v, _) => {
                    if v.is_empty() {
                        None
                    } else {
                        Some(v.len())
                    }
                }
            };
            match declared_ok {
                Some(n) => Ok(n),
                None => match parse_sources(&block).first() {
                    Some(candidate) => match extract(candidate, r, now, lsk_ref) {
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
    let mut lines: Vec<String> = vec![
        format!(
            "# recheck-live {} — mechanical re-verification sweep over phi/sources.φ (live sources)",
            date_str(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            )
        ),
        "# Classes: key-void (key marker without .secrets.local) | drift-void (API drift — curation duty) | quiet-void (empty = truth) | broken (fetch void)".into(),
    ];
    for f in findings.iter() {
        let line = format!("recheck {} {} — {}", f.url, f.class.as_str(), f.detail);
        println!("{}", line);
        lines.push(line);
    }
    if findings.is_empty() {
        lines.push(format!(
            "recheck-live {}: 0 findings — all {} tested sources harvested samples",
            date_str(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            ),
            ok
        ));
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
    if lsk.is_none() {
        if let Some(text) = fetch_one(
            "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls",
            None,
            &[],
            NAIF_LSK_TTL_SECS,
            machine_now_tdb(),
        ) {
            lsk = crate::lsk::parse(&text);
        }
    }
    let time_pair: Option<(f64, LeapSeconds)> = match lsk {
        Some(l) => match l.system_now_tdb() {
            Some(t) => Some((t, l)),
            None => None,
        },
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
            let workers = 8.min(non_kernel.len().max(1));
            std::thread::scope(|scope| {
                for _ in 0..workers {
                    scope.spawn(|| loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if i >= non_kernel.len() {
                            break;
                        }
                        let (ok, text) = probe_one(
                            non_kernel[i],
                            now,
                            lsk_ref,
                            &void_eph,
                            env,
                            fetchone,
                            precise,
                            lat,
                            lon,
                        );
                        if ok {
                            accepted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            out_lock.lock().unwrap().push_str(&text);
                        } else {
                            declined.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            dead_lock.lock().unwrap().push_str(&text);
                        }
                        std::thread::sleep(std::time::Duration::from_millis(300));
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
        if let (Some(b), Some(base)) = (&body, &baseline) {
            if b != base {
                effective_dp = dp;
            }
        }
    }
    format!("# template_precision {}dp\n", effective_dp)
}

pub fn probe_ttl(body: &str) -> Option<u64> {
    let val = parse_json(body)?;
    match val {
        JsonVal::Arr(ref arr) if arr.len() >= 2 => {
            let t0 = find_timestamp(&arr[0]);
            let t1 = find_timestamp(&arr[1]);
            match (t0, t1) {
                (Some(a), Some(b)) if (a - b).abs() >= 1.0 => Some((a - b).abs() as u64),
                _ => None,
            }
        }
        JsonVal::Obj(ref map) => {
            for (k, v) in map {
                if is_time_key(k) {
                    if json_num(v).is_some() {
                        return Some(60);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

pub fn find_timestamp(val: &JsonVal) -> Option<f64> {
    if let JsonVal::Obj(map) = val {
        for (k, v) in map {
            if is_time_key(k) {
                if let Some(n) = json_num(v) {
                    return Some(n);
                }
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
    Some(format!(
        "field {} {} {} {} {} {} 0.0 0.0\n",
        key, key, kid, force, unit, tau
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
        if force != "DROP" {
            if let Some(line) = draft_field_line(col, &force, &unit, tau) {
                out.push_str(&line);
            }
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

pub fn probe_classify(key: &str) -> (&str, &str, f64) {
    let kl = key.to_lowercase();
    if kl.contains("temp") || kl.contains("atmp") || kl.contains("wtmp") || kl.contains("dewp") {
        ("thermal", "C", 3600.0)
    } else if kl.contains("pres") || kl.contains("baro") {
        ("advective", "hPa", 60.0)
    } else if kl.contains("spd")
        || kl.contains("gust")
        || (kl.contains("wind") && !kl.contains("dir"))
    {
        ("advective", "m/s", 60.0)
    } else if kl.contains("dir") || kl.contains("heading") {
        ("advective", "deg", 60.0)
    } else if kl.contains("wave") || kl.contains("wvht") || kl.contains("swell") {
        ("acoustic", "m", 10.0)
    } else if kl.contains("depth") {
        ("seismic-body", "km", 10.0)
    } else if kl.contains("flux") {
        ("em", "W/m2", 3600.0)
    } else if kl == "bx"
        || kl == "by"
        || kl == "bz"
        || kl == "bt"
        || kl == "dst"
        || kl.contains("mag_")
        || kl.contains("_b_")
    {
        ("em", "nT", 60.0)
    } else if kl.contains("hum") || kl.contains("rh") || kl == "rel_hum" {
        ("diffusion", "%", 86400.0)
    } else if kl.contains("rain") || kl.contains("prcp") {
        ("acoustic", "mm", 60.0)
    } else if kl.contains("vis") {
        ("em", "km", 60.0)
    } else if kl.contains("co2")
        || kl.contains("ch4")
        || kl.contains("o3")
        || kl.contains("no2")
        || kl.contains("so2")
    {
        ("diffusion", "ppm", 86400.0)
    } else if kl.contains("vel") || kl.contains("vlct") {
        ("advective", "km/s", 60.0)
    } else if kl.contains("freq") || kl.ends_with("_hz") {
        ("em", "Hz", 60.0)
    } else if kl.contains("dens") {
        ("diffusion", "p/cm3", 3600.0)
    } else if kl.contains("conc") || kl.contains("salinity") {
        ("diffusion", "PSU", 86400.0)
    } else if kl == "db" || kl.ends_with("_db") {
        ("acoustic", "dB", 60.0)
    } else if kl.contains("discharge") {
        ("advective", "m3/s", 60.0)
    } else if kl == "v" || kl == "s" {
        ("gravity", "m", 3600.0)
    } else if kl.contains("footprint") {
        ("em", "km", 60.0)
    } else if kl.contains("volt") || kl.contains("efield") || kl.contains("potential") {
        ("electric", "V", 60.0)
    } else if kl.contains("current") && !kl.contains("ocean") {
        ("electric", "A", 60.0)
    } else if kl.contains("conduct") {
        ("electric", "S/m", 3600.0)
    } else if kl.contains("sample") || kl.contains("sort") || kl.contains("order") {
        ("DROP", "", 0.0)
    } else if kl == "mag" || kl.contains("magnitude") {
        ("seismic-body", "M", 3600.0)
    } else if kl.contains("bbox") {
        ("DROP", "", 0.0)
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
                            let old_exact = old_key.map_or(false, |ok_key| {
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
                        if force != "DROP" {
                            if let Some(line) = draft_field_line(&path, &force, &unit, tau) {
                                out.push_str(&line);
                            }
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
                let in_registry = force_id_of(&force)
                    .map(|fid| allowed_units_for_force(fid).contains(&unit_lc.as_str()))
                    .unwrap_or(false);
                if !in_registry {
                    out.push_str(&format!("# unit {} not in force registry — review\n", unit));
                }
                if let Some(line) = draft_field_line(prefix, &force, &unit, tau) {
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
                    let in_registry = force_id_of(&force)
                        .map(|fid| allowed_units_for_force(fid).contains(&unit_lc.as_str()))
                        .unwrap_or(false);
                    if !in_registry {
                        out.push_str(&format!("# unit {} not in force registry — review\n", unit));
                    }
                    if let Some(line) = draft_field_line(prefix, &force, &unit, tau) {
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
            if let Some(features) = map.get("features") {
                if let JsonVal::Arr(features_arr) = features {
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
    if let ExtractResult::Measurements(channels) = extract(src, raw, now, lsk) {
        if channels.is_empty() {
            report_anomaly("Empty Data", &src.url, "extract returned no measurements");
        }
    }
}

pub fn ci_mode(dir: &str) -> i32 {
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
    if lsk.is_none() {
        if let Some(text) = fetch_one(
            "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls",
            None,
            &[],
            NAIF_LSK_TTL_SECS,
            machine_now_tdb(),
        ) {
            lsk = crate::lsk::parse(&text);
        }
    }
    let now_tdb: Option<f64> = lsk.as_ref().and_then(|l| l.system_now_tdb());
    let total = sources.len();
    let mut reachable = 0usize;
    let mut dead = 0usize;
    let mut pending = 0usize;
    let mut mirrored = 0u32;
    let mut fresh = 0u32;
    let mut host_void: HashSet<String> = HashSet::new();
    for src in &sources {
        if src.url.starts_with("https://github.com/omegaflow/sources")
            || src.format == "ephemeris_binary"
            || src.format == "catalog_dastcom"
            || src.format == "csv_zip"
            || src.format == "kernel_text"
        {
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
            );
            probe_fanout(
                src,
                &headers,
                &env,
                &mut reachable,
                &mut dead,
                &mut mirrored,
                &mut pending,
                &mut host_void,
            );
            continue;
        }
        if url_has_template(&src.url) {
            probe_template(
                src,
                &headers,
                &env,
                &mut reachable,
                &mut dead,
                &mut mirrored,
            );
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
        let name = manifest
            .get(&src.url)
            .cloned()
            .unwrap_or_else(|| source_name_from_url(&src.url));
        let cache_path = match (&netloc, &name) {
            (Some(nl), nm) if !nm.is_empty() => {
                Some(format!("/tmp/archivar_cache/{}/{}.json", nl, nm))
            }
            _ => None,
        };
        if let Some(cp) = &cache_path {
            if cache_fresh(cp, src.ttl) {
                fresh += 1;
                continue;
            }
        }
        let raw = match fetch_raw(&src.url, None, &headers, src.ttl) {
            Some(r) => r,
            None => {
                eprintln!("ci-mode: fetch returned void for {}", src.url);
                report_anomaly("API Unreachable", &src.url, "fetch returned void");
                dead += 1;
                continue;
            }
        };
        if parse_json(&raw).is_some() {
            reachable += 1;
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
            if mirror_enabled {
                if let Some(netloc) = extract_netloc(&src.url) {
                    let name = manifest
                        .get(&src.url)
                        .cloned()
                        .unwrap_or_else(|| source_name_from_url(&src.url));
                    let tmp_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
                    if std::fs::write(&tmp_path, &raw).is_ok()
                        && crate::cdn::upload_release(netloc, &tmp_path)
                    {
                        mirrored += 1;
                    }
                }
            }
        } else {
            eprintln!("ci-mode: {} JSON parse void", src.url);
            report_anomaly("Malformed Data", &src.url, "JSON parse void");
            dead += 1;
        }
    }
    eprintln!(
        "ci-mode: {}/{} reachable, {} dead, {} pending (secret void), {} mirrored to CDN, {} fresh (local TTL), mirror={}",
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
            let already_open = Command::new("gh")
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
                .map(|o| String::from_utf8_lossy(&o.stdout).contains(&title))
                .unwrap_or(false);
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
    0
}
pub fn fanout_stations_secret_void(src: &SourceConfig, env: &HashMap<String, String>) -> bool {
    src.url.contains('{')
        && src
            .stations_url
            .as_deref()
            .map(|u| secret_resolves_void(u, env))
            .unwrap_or(false)
}

pub fn mirror_stations(
    src: &SourceConfig,
    headers: &[(String, String)],
    mirrored: &mut u32,
    reachable: &mut usize,
    dead: &mut usize,
    host_void: &mut HashSet<String>,
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
    let cache_path = format!("/tmp/archivar_cache/{}/{}.json", netloc, name);
    if cache_fresh(&cache_path, src.ttl) {
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
            } else {
                eprintln!("ci-mode: stations {} JSON parse void", stations_url);
                *dead += 1;
            }
        }
        None => {
            eprintln!("ci-mode: stations fetch void {}", stations_url);
            host_void.insert(netloc.to_string());
            *dead += 1;
        }
    }
}

pub fn probe_fanout(
    src: &SourceConfig,
    headers: &[(String, String)],
    env: &HashMap<String, String>,
    reachable: &mut usize,
    dead: &mut usize,
    mirrored: &mut u32,
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
    let Some(netloc) = extract_netloc(&src.url) else {
        return;
    };
    let tag = format!("{}-template", netloc);
    let name = source_name_from_url(&src.url);
    let cache_path = format!("/tmp/archivar_cache/{}/{}.json", tag, name);
    if cache_fresh(&cache_path, src.ttl) {
        *reachable += 1;
        return;
    }
    match fetch_raw(&probe_url, None, headers, src.ttl) {
        Some(body) => {
            if parse_json(&body).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: fanout probe {} JSON ok", probe_url);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::write(&cache_path, &body).is_ok()
                    && crate::cdn::upload_release(&tag, &cache_path)
                {
                    *mirrored += 1;
                }
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
    mirrored: &mut u32,
) {
    let anchor = frame_anchor(&src.frame);
    let probe_url = match ci_probe_render(&src.url, anchor, env) {
        Some(u) => u,
        None => return,
    };
    if secret_resolves_void(&probe_url, env) {
        return;
    }
    let Some(netloc) = extract_netloc(&src.url) else {
        return;
    };
    let tag = format!("{}-template", netloc);
    let name = source_name_from_url(&src.url);
    let cache_path = format!("/tmp/archivar_cache/{}/{}.json", tag, name);
    if cache_fresh(&cache_path, src.ttl) {
        *reachable += 1;
        return;
    }
    match fetch_raw(&probe_url, None, headers, src.ttl) {
        Some(body) => {
            if parse_json(&body).is_some() {
                *reachable += 1;
                eprintln!("ci-mode: template probe {} JSON ok", probe_url);
                if let Some(parent) = std::path::Path::new(&cache_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::write(&cache_path, &body).is_ok()
                    && crate::cdn::upload_release(&tag, &cache_path)
                {
                    *mirrored += 1;
                }
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
    let workers = 8.min(total.max(1));
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
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
                if let Some(body) = raw {
                    if let Some(parsed) = parse_json(&body) {
                        let tap_flat = tap_to_json(&parsed);
                        let effective = tap_flat.as_ref().unwrap_or(&parsed);
                        let mut fields = String::new();
                        let mut coords = String::new();
                        let mut map_path: Option<String> = None;
                        let mut budget = 48usize;
                        walk_json_probe(
                            effective,
                            "",
                            &mut fields,
                            &mut coords,
                            &mut map_path,
                            &mut budget,
                        );
                        let ttl = probe_ttl(&body);
                        let (frame, reason) = derive_frame(effective, &coords);
                        if !frame.is_empty() {
                            if let Some(rk) = route_key(&urls[i]) {
                                learned_lock
                                    .lock()
                                    .unwrap()
                                    .entry(rk)
                                    .or_insert_with(|| frame.trim_end().to_string());
                            }
                        }
                        let mut block = format!("url {}\n", urls[i]);
                        if let Some(t) = ttl {
                            block.push_str(&format!("ttl {}\n", t));
                        }
                        if tap_flat.is_some() {
                            block.push_str("format tap\n");
                        }
                        block.push_str(&frame);
                        if let Some(ref mp) = map_path {
                            if !coords.is_empty() {
                                let container = if coords.contains("ra ") || coords.contains("dec ")
                                {
                                    "cmap"
                                } else {
                                    "map"
                                };
                                block.push_str(&format!("{} {}\n", container, mp));
                            }
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
                }
                std::thread::sleep(std::time::Duration::from_millis(300));
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
                                .trim_end_matches(|ch| ch == ',' || ch == '|' || ch == ';');
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
        let context = context_map.get(url).cloned().unwrap_or_default();
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
            if let Some(rest) = t.strip_prefix("url ") {
                if let Some(nl) = extract_netloc(rest.trim()) {
                    if seen.insert(format!("+{}", nl)) {
                        delta.push((4, "-".to_string(), nl.to_string()));
                    }
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/dead_sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                if let Some(nl) = extract_netloc(rest.trim()) {
                    if seen.insert(format!("-{}", nl)) {
                        delta.push((-4, "-".to_string(), nl.to_string()));
                    }
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/blocked_sources.φ") {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                if let Some(nl) = extract_netloc(rest.trim()) {
                    if seen.insert(format!("b{}", nl)) {
                        delta.push((-2, "b".to_string(), nl.to_string()));
                    }
                }
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
        "# gate-delta — netloc weights, self-learning from sources.φ (+) + dead_sources.φ (−) + blocked_sources.φ (b)\n",
    );
    for (w, f, tag) in &delta {
        d.push_str(&format!("{} {} {}\n", w, f, tag));
    }
    if std::fs::write("phi/pipeline/library_gate_delta.φ", d).is_err() {
        eprintln!("write phi/pipeline/library_gate_delta.φ: the register does not remember");
    }
    let library = std::fs::read_to_string("phi/pipeline/library.φ").unwrap_or_default();
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

pub fn url_probe_mode(
    path: &str,
    env: &HashMap<String, String>,
    fetchone: bool,
    jina: bool,
) -> i32 {
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
            u.split_whitespace().next().unwrap_or("").to_string()
        })
        .filter(|u| u.starts_with("http"))
        .collect();
    let total = urls.len();
    let live = std::sync::atomic::AtomicUsize::new(0);
    let void = std::sync::atomic::AtomicUsize::new(0);
    let live_lock = std::sync::Mutex::new(String::new());
    let void_lock = std::sync::Mutex::new(String::new());
    let jina_lock = std::sync::Mutex::new(Vec::<String>::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    let workers = 8.min(total.max(1));
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
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
                        if jina && kind != "json" {
                            jina_lock.lock().unwrap().push(urls[i].clone());
                        }
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
    let mut jina_report = 0usize;
    if jina {
        let candidates = jina_lock.into_inner().unwrap();
        let jina_out = std::sync::Mutex::new(String::new());
        let jn = std::sync::atomic::AtomicUsize::new(0);
        let n_cand = candidates.len();
        let jina_key = env.get("JINA_API_KEY").cloned().unwrap_or_default();
        let jina_headers: Vec<(String, String)> = if jina_key.is_empty() {
            Vec::new()
        } else {
            vec![("Authorization".to_string(), format!("Bearer {}", jina_key))]
        };
        let j_workers = 4.min(n_cand.max(1));
        let j_pacing = if jina_key.is_empty() { 4000u64 } else { 500u64 };
        std::thread::scope(|scope| {
            for _ in 0..j_workers {
                scope.spawn(|| loop {
                    let i = jn.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if i >= n_cand {
                        break;
                    }
                    let wrapped = format!("https://r.jina.ai/{}", candidates[i]);
                    let body = fetch_raw_probe(&wrapped, None, &jina_headers);
                    if let Some(b) = body {
                        if parse_json(&b).is_some() {
                            jina_out
                                .lock()
                                .unwrap()
                                .push_str(&format!("jina-json | {}\n", candidates[i]));
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(j_pacing));
                });
            }
        });
        let out = jina_out.into_inner().unwrap();
        jina_report = out.lines().count();
        std::fs::write("phi/pipeline/probe_jina.txt", out).ok();
    }
    eprintln!(
        "--urls: {} checked, {} live, {} void, {} jina-json → phi/pipeline/probe_live.txt + probe_url_void.txt + probe_jina.txt",
        total, live, void, jina_report
    );
    0
}
