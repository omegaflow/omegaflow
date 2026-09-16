#[derive(Clone, Debug)]
pub struct NdkEvent {
    pub name: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hyp_lat: f64,
    pub hyp_lon: f64,
    pub hyp_depth_km: f64,
    pub exponent: i32,
    pub m_rr: f64,
    pub m_tt: f64,
    pub m_pp: f64,
    pub m_rt: f64,
    pub m_rp: f64,
    pub m_tp: f64,
    pub centroid_lat: f64,
    pub centroid_lon: f64,
    pub centroid_depth_km: f64,
    pub m0: f64,
    pub strike: f64,
    pub dip: f64,
    pub rake: f64,
    pub strike2: f64,
    pub dip2: f64,
    pub rake2: f64,
}

impl NdkEvent {
    pub fn scalar_moment_dyne_cm(&self) -> f64 {
        self.m0 * 10f64.powi(self.exponent)
    }

    pub fn mw(&self) -> Option<f64> {
        let m0_nm = self.scalar_moment_dyne_cm() * 1e-7;
        if !m0_nm.is_finite() || m0_nm <= 0.0 {
            return None;
        }
        Some((2.0 / 3.0) * (m0_nm.log10() - 9.1))
    }
}

fn ndk_num(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn is_hypocenter_line(line: &str) -> bool {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 6 {
        return false;
    }
    let d = t[1].as_bytes();
    d.len() == 10 && d[4] == b'/' && d[7] == b'/'
}

fn parse_hypocenter(line: &str) -> Option<(i32, u32, u32, f64, f64, f64)> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 6 {
        return None;
    }
    let parts: Vec<&str> = t[1].split('/').collect();
    if parts.len() != 3 {
        return None;
    }
    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let day = parts[2].parse::<u32>().ok()?;
    let lat = ndk_num(t[3])?;
    let lon = ndk_num(t[4])?;
    let depth = ndk_num(t[5])?;
    Some((year, month, day, lat, lon, depth))
}

fn parse_centroid(line: &str) -> Option<(f64, f64, f64)> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 9 {
        return None;
    }
    let lat = ndk_num(t[3])?;
    let lon = ndk_num(t[5])?;
    let depth = ndk_num(t[7])?;
    Some((lat, lon, depth))
}

fn parse_tensor(line: &str) -> Option<(i32, [f64; 6])> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 13 {
        return None;
    }
    let exponent = t[0].parse::<i32>().ok()?;
    let m = [
        ndk_num(t[1])?,
        ndk_num(t[3])?,
        ndk_num(t[5])?,
        ndk_num(t[7])?,
        ndk_num(t[9])?,
        ndk_num(t[11])?,
    ];
    Some((exponent, m))
}

fn parse_v10(line: &str) -> Option<(f64, f64, f64, f64, f64, f64, f64)> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 17 {
        return None;
    }
    let m0 = ndk_num(t[10])?;
    let strike = ndk_num(t[11])?;
    let dip = ndk_num(t[12])?;
    let rake = ndk_num(t[13])?;
    let strike2 = ndk_num(t[14])?;
    let dip2 = ndk_num(t[15])?;
    let rake2 = ndk_num(t[16])?;
    Some((m0, strike, dip, rake, strike2, dip2, rake2))
}

pub fn parse_ndk(text: &str) -> Vec<NdkEvent> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        if !is_hypocenter_line(lines[i]) {
            i += 1;
            continue;
        }
        if i + 4 >= lines.len() {
            break;
        }
        let Some((year, month, day, lat, lon, depth)) = parse_hypocenter(lines[i]) else {
            i += 1;
            continue;
        };
        let name = lines[i + 1]
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();
        let Some((c_lat, c_lon, c_depth)) = parse_centroid(lines[i + 2]) else {
            i += 5;
            continue;
        };
        let Some((exponent, m)) = parse_tensor(lines[i + 3]) else {
            i += 5;
            continue;
        };
        let Some((m0, strike, dip, rake, strike2, dip2, rake2)) = parse_v10(lines[i + 4]) else {
            i += 5;
            continue;
        };
        out.push(NdkEvent {
            name,
            year,
            month,
            day,
            hyp_lat: lat,
            hyp_lon: lon,
            hyp_depth_km: depth,
            exponent,
            m_rr: m[0],
            m_tt: m[1],
            m_pp: m[2],
            m_rt: m[3],
            m_rp: m[4],
            m_tp: m[5],
            centroid_lat: c_lat,
            centroid_lon: c_lon,
            centroid_depth_km: c_depth,
            m0,
            strike,
            dip,
            rake,
            strike2,
            dip2,
            rake2,
        });
        i += 5;
    }
    out
}

pub fn fetch_events(url: &str, ttl: u64) -> Option<Vec<NdkEvent>> {
    let name = url.rsplit('/').next().unwrap_or("catalog");
    let path = super::content_cache(&format!("omegaflow_ndk_{name}"));
    if super::cache_fresh(&path, ttl) {
        let text = std::fs::read_to_string(&path).ok()?;
        return Some(parse_ndk(&text));
    }
    let bytes = super::fetch_raw_bytes(url, ttl)?;
    if std::fs::write(&path, &bytes).is_err() {
        eprintln!("cache {path}: write void — refetch next cycle");
    }
    Some(parse_ndk(&String::from_utf8_lossy(&bytes)))
}

pub fn dc_moment_tensor(strike_deg: f64, dip_deg: f64, rake_deg: f64) -> [f64; 6] {
    let s = strike_deg.to_radians();
    let d = dip_deg.to_radians();
    let l = rake_deg.to_radians();
    let (sin_s, cos_s) = s.sin_cos();
    let (sin_d, cos_d) = d.sin_cos();
    let (sin_l, cos_l) = l.sin_cos();
    let sin_2d = 2.0 * sin_d * cos_d;
    let cos_2d = cos_d * cos_d - sin_d * sin_d;
    let sin_2s = 2.0 * sin_s * cos_s;
    let cos_2s = cos_s * cos_s - sin_s * sin_s;
    [
        sin_2d * sin_l,
        -(sin_d * cos_l * sin_2s + sin_2d * sin_l * sin_s * sin_s),
        sin_d * cos_l * sin_2s - sin_2d * sin_l * cos_s * cos_s,
        -(cos_d * cos_l * cos_s + cos_2d * sin_l * sin_s),
        cos_d * cos_l * sin_s - cos_2d * sin_l * cos_s,
        -(sin_d * cos_l * cos_2s + 0.5 * sin_2d * sin_l * sin_2s),
    ]
}

pub fn rp(m: &[f64; 6], r: &[f64; 3]) -> f64 {
    let (rr, rt, rp) = (r[0], r[1], r[2]);
    m[0] * rr * rr
        + m[1] * rt * rt
        + m[2] * rp * rp
        + 2.0 * m[3] * rr * rt
        + 2.0 * m[4] * rr * rp
        + 2.0 * m[5] * rt * rp
}

pub fn ray_direction(takeoff_deg: f64, azimuth_deg: f64, upgoing: bool) -> [f64; 3] {
    let i = takeoff_deg.to_radians();
    let az = azimuth_deg.to_radians();
    let r_up = if upgoing { i.cos() } else { -i.cos() };
    let r_south = -i.sin() * az.cos();
    let r_east = i.sin() * az.sin();
    [r_up, r_south, r_east]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE_EVENT: &str = "MLI 1976/01/01 01:29:39.6 -28.61 -177.64 59.0 6.2 0.0 KERMADEC ISLANDS REGION\n\
M010176A B: 0 0 0 S: 0 0 0 M: 12 30 135 CMT: 1 BOXHD: 9.4\n\
CENTROID: 13.8 0.2 -29.25 0.02 -176.96 0.01 47.8 0.6 FREE O-00000000000000\n\
26 7.680 0.090 0.090 0.060 -7.770 0.070 1.390 0.160 4.520 0.160 -3.260 0.060\n\
V10 8.940 75 283 1.260 2 19 -10.190 15 110 9.560 202 30 93 18 60 88";

    const SAMPLE: &str = "MLI 1976/01/01 01:29:39.6 -28.61 -177.64 59.0 6.2 0.0 KERMADEC ISLANDS REGION\n\
M010176A B: 0 0 0 S: 0 0 0 M: 12 30 135 CMT: 1 BOXHD: 9.4\n\
CENTROID: 13.8 0.2 -29.25 0.02 -176.96 0.01 47.8 0.6 FREE O-00000000000000\n\
26 7.680 0.090 0.090 0.060 -7.770 0.070 1.390 0.160 4.520 0.160 -3.260 0.060\n\
V10 8.940 75 283 1.260 2 19 -10.190 15 110 9.560 202 30 93 18 60 88\n\
MLI 1976/01/05 02:31:36.3 -13.29 -74.90 95.0 6.0 0.0 PERU\n\
C010576A B: 6 14 45 S: 0 0 0 M: 5 8 135 CMT: 1 BOXHD: 1.6\n\
CENTROID: 8.4 0.4 -13.42 0.07 -75.14 0.06 85.4 3.2 FREE O-00000000000000\n\
24 -1.780 0.210 -0.590 0.280 2.370 0.280 -1.280 0.150 1.970 0.150 -2.900 0.220\n\
V10 4.970 19 238 -2.350 14 143 -2.620 66 20 3.790 350 28 -60 137 66 -105";

    #[test]
    fn parses_a_single_five_line_event() {
        let events = parse_ndk(ONE_EVENT);
        assert_eq!(events.len(), 1);
        let e = &events[0];
        assert_eq!(e.name, "M010176A");
        assert_eq!((e.year, e.month, e.day), (1976, 1, 1));
        assert!((e.hyp_lat - -28.61).abs() < 1e-9);
        assert!((e.hyp_lon - -177.64).abs() < 1e-9);
        assert!((e.hyp_depth_km - 59.0).abs() < 1e-9);
    }

    #[test]
    fn parses_two_measured_events() {
        let events = parse_ndk(SAMPLE);
        assert_eq!(events.len(), 2);
        let e0 = &events[0];
        assert_eq!(e0.name, "M010176A");
        assert_eq!((e0.year, e0.month, e0.day), (1976, 1, 1));
        assert!((e0.hyp_lat - -28.61).abs() < 1e-9);
        assert!((e0.hyp_lon - -177.64).abs() < 1e-9);
        assert!((e0.hyp_depth_km - 59.0).abs() < 1e-9);
        assert_eq!(e0.exponent, 26);
        assert!((e0.m_rr - 7.680).abs() < 1e-9);
        assert!((e0.m_tt - 0.090).abs() < 1e-9);
        assert!((e0.m_pp - -7.770).abs() < 1e-9);
        assert!((e0.m_rt - 1.390).abs() < 1e-9);
        assert!((e0.m_rp - 4.520).abs() < 1e-9);
        assert!((e0.m_tp - -3.260).abs() < 1e-9);
        assert!((e0.centroid_lat - -29.25).abs() < 1e-9);
        assert!((e0.centroid_lon - -176.96).abs() < 1e-9);
        assert!((e0.centroid_depth_km - 47.8).abs() < 1e-9);
        assert!((e0.m0 - 9.560).abs() < 1e-9);
        assert!((e0.strike - 202.0).abs() < 1e-9);
        assert!((e0.dip - 30.0).abs() < 1e-9);
        assert!((e0.rake - 93.0).abs() < 1e-9);
        assert!((e0.strike2 - 18.0).abs() < 1e-9);
        assert!((e0.dip2 - 60.0).abs() < 1e-9);
        assert!((e0.rake2 - 88.0).abs() < 1e-9);

        let e1 = &events[1];
        assert_eq!(e1.name, "C010576A");
        assert_eq!(e1.exponent, 24);
        assert!((e1.m0 - 3.790).abs() < 1e-9);
        assert!((e1.strike - 350.0).abs() < 1e-9);
        assert!((e1.dip - 28.0).abs() < 1e-9);
        assert!((e1.rake - -60.0).abs() < 1e-9);
        assert!((e1.centroid_depth_km - 85.4).abs() < 1e-9);
    }

    #[test]
    fn the_fixture_reparses_identically() {
        let first = parse_ndk(SAMPLE);
        let second = parse_ndk(SAMPLE);
        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!((a.year, a.month, a.day), (b.year, b.month, b.day));
            assert_eq!(a.exponent, b.exponent);
            assert_eq!(a.m0, b.m0);
            assert_eq!(a.m_rr, b.m_rr);
            assert_eq!(a.m_tp, b.m_tp);
        }
    }

    #[test]
    fn scalar_moment_and_mw_derive_from_m0_and_exponent() {
        let events = parse_ndk(SAMPLE);
        let e0 = &events[0];
        let m0 = e0.scalar_moment_dyne_cm();
        assert!((m0 - 9.560e26).abs() / 9.560e26 < 1e-6);
        assert!((e0.mw().unwrap() - 7.254).abs() < 0.05);
    }

    #[test]
    fn the_double_couple_tensor_is_trace_free() {
        for (s, d, r) in [
            (0.0, 45.0, 90.0),
            (202.0, 30.0, 93.0),
            (350.0, 28.0, -60.0),
            (10.0, 80.0, 20.0),
        ] {
            let m = dc_moment_tensor(s, d, r);
            assert!(
                (m[0] + m[1] + m[2]).abs() < 1e-12,
                "trace must vanish for {s}/{d}/{r}"
            );
        }
    }

    #[test]
    fn the_thrust_radiation_pattern_is_vertical_compressional() {
        let m = dc_moment_tensor(0.0, 45.0, 90.0);
        assert!((rp(&m, &[1.0, 0.0, 0.0]) - 1.0).abs() < 1e-12);
        assert!((rp(&m, &[-1.0, 0.0, 0.0]) - 1.0).abs() < 1e-12);
        assert!((rp(&m, &[0.0, 0.0, 1.0]) - -1.0).abs() < 1e-12);
        assert!(rp(&m, &[0.0, 1.0, 0.0]).abs() < 1e-12);
    }

    #[test]
    fn the_dc_from_strike_dip_rake_matches_the_ndk_off_diagonals() {
        let events = parse_ndk(SAMPLE);
        let e0 = &events[0];
        let m = dc_moment_tensor(e0.strike, e0.dip, e0.rake);
        for (dc, ndk_v) in [(m[3], e0.m_rt), (m[4], e0.m_rp), (m[5], e0.m_tp)] {
            let scaled = dc * e0.m0;
            assert!(
                scaled.signum() == ndk_v.signum(),
                "off-diagonal sign must match: {scaled} vs {ndk_v}"
            );
            assert!(
                (scaled - ndk_v).abs() < 0.30 * e0.m0,
                "off-diagonal {scaled} vs {ndk_v} (CLVD carries the rest)"
            );
        }
    }
}
