use super::*;

#[derive(Clone, Copy)]
pub struct RenderCtx<'a> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub tdb: f64,
    pub r: f64,
    pub eph: &'a HashMap<String, BodyEphemeris>,
    pub lsk: &'a LeapSeconds,
}

pub fn ci_probe_render(
    template: &str,
    anchor: Option<(f64, f64)>,
    env: &HashMap<String, String>,
) -> Option<String> {
    const COORD_MARKERS: [&str; 8] = [
        "{lat}",
        "{lon}",
        "{lat_int}",
        "{lon_int}",
        "{lat_min}",
        "{lat_max}",
        "{lon_min}",
        "{lon_max}",
    ];
    let needs_anchor = COORD_MARKERS.iter().any(|m| template.contains(m));
    if needs_anchor && anchor.is_none() {
        return None;
    }
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
    let mut url = template
        .replace("{today}", &format!("{}-{:02}-{:02}", ty, tm, td))
        .replace("{yesterday}", &format!("{}-{:02}-{:02}", yy, ym, yd))
        .replace("{week_ago}", &format!("{}-{:02}-{:02}", wy, wm, wd))
        .replace("{now}", &now_iso)
        .replace("{hour_ago}", &hour_ago_iso)
        .replace("{year}", &ty.to_string())
        .replace("{jd_now}", &format!("{:.6}", jd_now))
        .replace("{jd_start}", &format!("{:.6}", jd_now - 1.0))
        .replace("{jd_end}", &format!("{:.6}", jd_now));
    if let Some((lat, lon)) = anchor {
        url = url
            .replace("{lat}", &format!("{:.6}", lat))
            .replace("{lon}", &format!("{:.6}", lon))
            .replace("{lat_int}", &format!("{:.0}", lat))
            .replace("{lon_int}", &format!("{:.0}", lon))
            .replace("{lat_min}", &format!("{:.6}", lat - half))
            .replace("{lat_max}", &format!("{:.6}", lat + half))
            .replace("{lon_min}", &format!("{:.6}", lon - half))
            .replace("{lon_max}", &format!("{:.6}", lon + half));
    }
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

pub fn render_url(template: &str, body_name: &str, ctx: RenderCtx<'_>) -> Option<String> {
    let RenderCtx {
        x,
        y,
        z,
        tdb,
        r,
        eph,
        lsk,
    } = ctx;
    let unix = lsk.tdb_to_unix(tdb)?;
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
    let jd_now = format!("{:.6}", tdb_to_jd(tdb));
    let jd_start = format!("{:.6}", tdb_to_jd(tdb - 86400.0));

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
        .replace("{prev_Mon}", month_abbr(if tm == 1 { 12 } else { tm - 1 }))
        .replace("{day}", &td.to_string())
        .replace("{yday}", &format!("{:03}", yday))
        .replace("{hour}", &format!("{:02}", q_hour))
        .replace("{minute}", &format!("{:02}", q_minute))
        .replace("{unix_now}", &unix_now)
        .replace("{unix_now_plus_3600}", &unix_now_plus_3600);

    if let Some((lat, lon)) = icrs_to_body_surface(x, y, z, tdb, body_name, eph) {
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
                let half_deg = r / m_per_deg;
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
    ctx: RenderCtx<'_>,
    env: &HashMap<String, String>,
) -> Option<String> {
    let RenderCtx {
        x, y, z, tdb, eph, ..
    } = ctx;
    let mut url = render_url(&src.url, &frame_body_name(&src.frame), ctx)?;
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
    if url.contains("{nearest_station}")
        && let Some(ref st_url) = src.stations_url
    {
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
    Some(resolve_secret(&url, env))
}

pub fn render_source_body(src: &SourceConfig, ctx: RenderCtx<'_>) -> Option<String> {
    let tmpl = src.post_body.as_ref()?;
    let mut body = render_url(tmpl, &frame_body_name(&src.frame), ctx)?;
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

#[cfg(test)]
mod prev_mon_tests {
    use super::*;

    #[test]
    fn render_url_prev_mon_is_the_previous_calendar_month() {
        let lsk = crate::archivar::LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let url = render_url(
            "https://example.com/{month}/{prev_Mon}",
            "earth",
            RenderCtx {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                tdb: 8.0e8,
                r: 1000.0,
                eph: &std::collections::HashMap::new(),
                lsk: &lsk,
            },
        )
        .unwrap();
        let parts: Vec<&str> = url
            .trim_start_matches("https://example.com/")
            .split('/')
            .collect();
        let month: u32 = parts[0].parse().unwrap();
        assert_eq!(
            parts[1],
            crate::archivar::month_abbr(if month == 1 { 12 } else { month - 1 })
        );
    }
}
