use super::*;

pub fn ci_probe_render(
    template: &str,
    anchor: (f64, f64),
    env: &HashMap<String, String>,
) -> Option<String> {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .ok()?;
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days);
    let (yy, ym, yd) = days_to_ymd(days - 1);
    let (wy, wm, wd) = days_to_ymd(days - 7);
    let hour = (secs % 86400) / 3600;
    let minute = (secs % 3600) / 60;
    let now_iso = format!("{}-{:02}-{:02}T{:02}:{:02}:00", ty, tm, td, hour, minute);
    let hour_ago_iso = {
        let dt = secs.saturating_sub(3600);
        let (h_y, h_m, h_d) = days_to_ymd(dt / 86400);
        let h_h = (dt % 86400) / 3600;
        let h_min = (dt % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", h_y, h_m, h_d, h_h, h_min)
    };
    let half = 0.5f64;
    let jd_now = 2440587.5 + secs as f64 / 86400.0;
    let url = template
        .replace("{today}", &format!("{}-{:02}-{:02}", ty, tm, td))
        .replace("{yesterday}", &format!("{}-{:02}-{:02}", yy, ym, yd))
        .replace("{week_ago}", &format!("{}-{:02}-{:02}", wy, wm, wd))
        .replace("{now}", &now_iso)
        .replace("{hour_ago}", &hour_ago_iso)
        .replace("{year}", &ty.to_string())
        .replace("{jd_now}", &format!("{:.6}", jd_now))
        .replace("{jd_start}", &format!("{:.6}", jd_now - 1.0))
        .replace("{jd_end}", &format!("{:.6}", jd_now))
        .replace("{lat}", &format!("{:.6}", anchor.0))
        .replace("{lon}", &format!("{:.6}", anchor.1))
        .replace("{lat_int}", &format!("{:.0}", anchor.0))
        .replace("{lon_int}", &format!("{:.0}", anchor.1))
        .replace("{lat_min}", &format!("{:.6}", anchor.0 - half))
        .replace("{lat_max}", &format!("{:.6}", anchor.0 + half))
        .replace("{lon_min}", &format!("{:.6}", anchor.1 - half))
        .replace("{lon_max}", &format!("{:.6}", anchor.1 + half));
    Some(resolve_secret(&url, env))
}



pub fn render_headers(
    headers: &[(String, String)],
    env: &HashMap<String, String>,
) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(k, v)| (k.clone(), resolve_secret(v, env)))
        .collect()
}



pub fn render_url(
    template: &str,
    x: f64,
    y: f64,
    z: f64,
    tdb_secs: f64,
    extent: f64,
    body_name: &str,
    eph: &HashMap<String, BodyEphemeris>,
    lsk: &LeapSeconds,
) -> Option<String> {
    let unix = match lsk.tdb_to_unix(tdb_secs) {
        Some(u) => u,
        None => return None,
    };
    let secs = unix as u64;
    let days = secs / 86400;
    let (ty, tm, td) = days_to_ymd(days);
    let yday = {
        let cum = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let leap = (ty % 4 == 0 && ty % 100 != 0) || ty % 400 == 0;
        let base = if tm > 0 { cum[(tm - 1) as usize] } else { 0 };
        base + td + if leap && tm > 2 { 1 } else { 0 }
    };
    let year2 = ty % 100;
    let today = format!("{}-{:02}-{:02}", ty, tm, td);
    let (yy, ym, yd) = days_to_ymd(days - 1);
    let yesterday = format!("{}-{:02}-{:02}", yy, ym, yd);
    let (tmy, tmm, tmd) = days_to_ymd(days + 1);
    let tomorrow = format!("{}-{:02}-{:02}", tmy, tmm, tmd);
    let today_yyyymmdd = format!("{}_{:02}_{:02}", ty, tm, td);
    let today_nodashes = format!("{}{:02}{:02}", ty, tm, td);
    let yesterday_nodashes = format!("{}{:02}{:02}", yy, ym, yd);
    let tomorrow_nodashes = format!("{}{:02}{:02}", tmy, tmm, tmd);
    let hour_ago = {
        let dt = secs.saturating_sub(3600);
        let (h_y, h_m, h_d) = days_to_ymd(dt / 86400);
        let h_h = (dt % 86400) / 3600;
        let h_min = (dt % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", h_y, h_m, h_d, h_h, h_min)
    };
    let now_iso = {
        let n_h = (secs % 86400) / 3600;
        let n_min = (secs % 3600) / 60;
        format!("{}-{:02}-{:02}T{:02}:{:02}:00", ty, tm, td, n_h, n_min)
    };
    let now_minus_1 = {
        let dt = secs.saturating_sub(60);
        let (n1_y, n1_m, n1_d) = days_to_ymd(dt / 86400);
        let n1_h = (dt % 86400) / 3600;
        let n1_min = (dt % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            n1_y, n1_m, n1_d, n1_h, n1_min
        )
    };
    let now_minus_2 = {
        let dt = secs.saturating_sub(120);
        let (n2_y, n2_m, n2_d) = days_to_ymd(dt / 86400);
        let n2_h = (dt % 86400) / 3600;
        let n2_min = (dt % 3600) / 60;
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:00",
            n2_y, n2_m, n2_d, n2_h, n2_min
        )
    };
    let week_ago = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}-{:02}-{:02}", w_y, w_m, w_d)
    };
    let week_ago_nodashes = {
        let dt = secs.saturating_sub(604800);
        let (w_y, w_m, w_d) = days_to_ymd(dt / 86400);
        format!("{}{:02}{:02}", w_y, w_m, w_d)
    };
    let q_hour = (secs % 86400) / 3600;
    let q_minute = (secs % 3600) / 60;
    let unix_now = secs.to_string();
    let unix_now_plus_3600 = (secs + 3600).to_string();
    let jd_now = format!("{:.6}", tdb_to_jd(tdb_secs));
    let jd_start = format!("{:.6}", tdb_to_jd(tdb_secs - 86400.0));

    let mut url = template
        .replace("{x}", &format!("{}", x))
        .replace("{y}", &format!("{}", y))
        .replace("{z}", &format!("{}", z))
        .replace("{jd_now}", &jd_now)
        .replace("{jd_start}", &jd_start)
        .replace("{jd_end}", &jd_now)
        .replace("{today}", &today)
        .replace("{yesterday}", &yesterday)
        .replace("{tomorrow}", &tomorrow)
        .replace("{today_yyyymmdd}", &today_yyyymmdd)
        .replace("{today_ymd}", &today_yyyymmdd)
        .replace("{today_nodashes}", &today_nodashes)
        .replace("{yesterday_nodashes}", &yesterday_nodashes)
        .replace("{tomorrow_nodashes}", &tomorrow_nodashes)
        .replace("{t_start}", &yesterday)
        .replace("{t_end}", &today)
        .replace("{now}", &now_iso)
        .replace("{now_minus_1}", &now_minus_1)
        .replace("{now_minus_2}", &now_minus_2)
        .replace("{week_ago}", &week_ago)
        .replace("{week_ago_nodashes}", &week_ago_nodashes)
        .replace(
            "{today_plus_365}",
            &format!("{}-{:02}-{:02}", ty + 1, tm, td),
        )
        .replace("{hour_ago}", &hour_ago)
        .replace("{year}", &ty.to_string())
        .replace("{year2}", &format!("{:02}", year2))
        .replace("{month}", &tm.to_string())
        .replace("{day}", &td.to_string())
        .replace("{yday}", &format!("{:03}", yday))
        .replace("{hour}", &format!("{:02}", q_hour))
        .replace("{minute}", &format!("{:02}", q_minute))
        .replace("{unix_now}", &unix_now)
        .replace("{unix_now_plus_3600}", &unix_now_plus_3600);

    if let Some((lat, lon)) = icrs_to_body_surface(x, y, z, tdb_secs, body_name, eph) {
        let radius_m = match eph.get(body_name).and_then(|e| e.props.as_ref()) {
            Some(p) => p.radius_m,
            None => 0.0,
        };
        let lat_str = format!("{:.6}", lat);
        let lon_str = format!("{:.6}", lon);
        url = url
            .replace("{lat}", &lat_str)
            .replace("{lon}", &lon_str)
            .replace("{lat_int}", &format!("{:.0}", lat))
            .replace("{lon_int}", &format!("{:.0}", lon));
        if radius_m > 0.0 {
            let m_per_deg =
                std::f64::consts::PI * radius_m / 180.0 * lat.to_radians().cos().max(0.0);
            if m_per_deg > 0.0 {
                let half_deg = extent / m_per_deg;
                let res = 6usize;
                url = url
                    .replace("{lat_min}", &format!("{:.*}", res, lat - half_deg))
                    .replace("{lat_max}", &format!("{:.*}", res, lat + half_deg))
                    .replace("{lon_min}", &format!("{:.*}", res, lon - half_deg))
                    .replace("{lon_max}", &format!("{:.*}", res, lon + half_deg));
                let step = half_deg * 0.5;
                let mut grid = Vec::with_capacity(16);
                let mut gla = Vec::with_capacity(4);
                let mut glo = Vec::with_capacity(4);
                for i in 0..4 {
                    for j in 0..4 {
                        grid.push(format!(
                            "{:.*},{:.*}",
                            res,
                            lat + (i as f64 - 1.5) * step,
                            res,
                            lon + (j as f64 - 1.5) * step
                        ));
                    }
                    gla.push(format!("{:.*}", res, lat + (i as f64 - 1.5) * step));
                    glo.push(format!("{:.*}", res, lon + (i as f64 - 1.5) * step));
                }
                url = url
                    .replace("{grid}", &grid.join("|"))
                    .replace("{grid_lat}", &gla.join(","))
                    .replace("{grid_lon}", &glo.join(","));
            }
        }
    }

    Some(url)
}



pub fn render_source_url(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    tdb: f64,
    r: f64,
    eph: &HashMap<String, BodyEphemeris>,
    env: &HashMap<String, String>,
    lsk: &LeapSeconds,
) -> Option<String> {
    let mut url = match render_url(
        &src.url,
        x,
        y,
        z,
        tdb,
        r,
        &frame_body_name(&src.frame),
        eph,
        lsk,
    ) {
        Some(u) => u,
        None => return None,
    };
    if let Some(ref t) = src.target {
        url = url.replace("{target}", t);
    }
    if let Some(ref c) = src.catalog {
        url = url.replace("{catalog}", c);
    }
    if let Some(f) = src.max_freq {
        url = url.replace("{max_freq}", &f.to_string());
    }
    if let Some(f) = src.min_freq {
        url = url.replace("{min_freq}", &f.to_string());
    }
    if src.repeat_ra_bins > 0 {
        let ra_deg = f64::atan2(y, x).to_degrees();
        let ra_norm = ((ra_deg % 360.0) + 360.0) % 360.0;
        let bin = ((ra_norm / 360.0) * (src.repeat_ra_bins as f64)) as u32;
        let bin_str = format!("{:02}", bin);
        url = url
            .replace("{repeat_bin}", &bin_str)
            .replace("{bin}", &bin_str);
    }
    if url.contains("{nearest_station}") {
        if let Some(ref st_url) = src.stations_url {
            let stations = if let Some(body) = fetch_one(st_url, None, &[], 86400, Some(tdb)) {
                if let Some(j) = parse_json(&body) {
                    let arr = jpath_val(&j, &src.stations_path).and_then(|v| {
                        if let JsonVal::Arr(a) = v {
                            Some(a)
                        } else {
                            None
                        }
                    });
                    if let Some(arr) = arr {
                        let entries: Vec<StationEntry> = arr
                            .iter()
                            .filter_map(|s| {
                                let id = match jpath_val(s, &src.stations_id)? {
                                    JsonVal::Str(st) => st.clone(),
                                    JsonVal::Num(n) => n.to_string(),
                                    _ => return None,
                                };
                                let lat = scalar_of(jpath_val(s, &src.stations_lat)?)?;
                                let lon = scalar_of(jpath_val(s, &src.stations_lon)?)?;
                                Some(StationEntry { id, lat, lon })
                            })
                            .collect();
                        Arc::new(entries)
                    } else {
                        Arc::new(Vec::new())
                    }
                } else {
                    Arc::new(Vec::new())
                }
            } else {
                Arc::new(Vec::new())
            };
            if !stations.is_empty() {
                let (lat, lon) =
                    match icrs_to_body_surface(x, y, z, tdb, &frame_body_name(&src.frame), eph) {
                        Some(ll) => ll,
                        None => return Some(url),
                    };
                let mut best = 0usize;
                let mut best_d = f64::MAX;
                for (i, st) in stations.iter().enumerate() {
                    let d2 = (st.lat - lat).powi(2) + (st.lon - lon).powi(2);
                    if d2 < best_d {
                        best_d = d2;
                        best = i;
                    }
                }
                url = url.replace("{nearest_station}", &stations[best].id);
            }
        }
    }
    Some(resolve_secret(&url, env))
}



pub fn render_source_body(
    src: &SourceConfig,
    x: f64,
    y: f64,
    z: f64,
    tdb: f64,
    r: f64,
    eph: &HashMap<String, BodyEphemeris>,
    lsk: &LeapSeconds,
) -> Option<String> {
    let tmpl = src.post_body.as_ref()?;
    let mut body = match render_url(
        tmpl,
        x,
        y,
        z,
        tdb,
        r,
        &frame_body_name(&src.frame),
        eph,
        lsk,
    ) {
        Some(b) => b,
        None => return None,
    };
    if let Some(ref t) = src.target {
        body = body.replace("{target}", t);
    }
    if let Some(ref c) = src.catalog {
        body = body.replace("{catalog}", c);
    }
    if let Some(f) = src.max_freq {
        body = body.replace("{max_freq}", &f.to_string());
    }
    if let Some(f) = src.min_freq {
        body = body.replace("{min_freq}", &f.to_string());
    }
    Some(body)
}
