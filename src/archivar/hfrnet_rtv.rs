use super::*;

pub const HFRNET_REC_BYTES: usize = 60;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HfrSample {
    pub t: f64,
    pub lat: f64,
    pub lon: f64,
    pub u: Option<f64>,
    pub v: Option<f64>,
}

fn csv_f64(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("nan") {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn wrap_lon(lon: f64) -> f64 {
    if lon > 180.0 { lon - 360.0 } else { lon }
}

pub fn parse_csv(text: &str, lsk: &LeapSeconds) -> Option<Vec<HfrSample>> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let head = line.split(',').next().unwrap_or("");
        if head == "time" || head == "UTC" {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5 {
            continue;
        }
        let Some(t) = parse_iso_tdb(cols[0], lsk) else {
            continue;
        };
        let (Some(lat), Some(lon)) = (csv_f64(cols[1]), csv_f64(cols[2])) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) {
            continue;
        }
        let u = csv_f64(cols[3]);
        let v = csv_f64(cols[4]);
        if u.is_none() && v.is_none() {
            continue;
        }
        out.push(HfrSample {
            t,
            lat,
            lon: wrap_lon(lon),
            u,
            v,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn to_geo_rows(samples: &[HfrSample]) -> Vec<crate::geo::GeoRec> {
    let mut out = Vec::with_capacity(samples.len() * 2);
    for s in samples {
        for (val, comp) in [(s.u, crate::geo::COMP_HFR_U), (s.v, crate::geo::COMP_HFR_V)] {
            let Some(val) = val else { continue };
            out.push(crate::geo::GeoRec {
                t: s.t,
                lat: s.lat,
                lon: s.lon,
                alt: 0.0,
                freq: crate::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::spectral::SPECTRAL_NO_BAND,
                val,
                comp,
                station: 0,
            });
        }
    }
    out
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        crate::geo::COMP_HFR_U => Some("hfrnet_rtv_eastward_m_s"),
        crate::geo::COMP_HFR_V => Some("hfrnet_rtv_northward_m_s"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_CSV: &str = "time,latitude,longitude,water_u,water_v\nUTC,degrees_north,degrees_east,m s-1,m s-1\n2026-09-21T05:00:00Z,32.5874,242.75774,0.05,-0.25\n2026-09-21T05:00:00Z,32.7672,242.54951,NaN,NaN\n2026-09-21T05:00:00Z,33.3066,242.34131,0.08,NaN\n";

    #[test]
    fn parse_csv_carries_measured_components_and_drops_absent() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        let rows = parse_csv(MEASURED_CSV, &lsk).expect("the measured CSV parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].u, Some(0.05));
        assert_eq!(rows[0].v, Some(-0.25));
        assert_eq!(rows[0].lat, 32.5874);
        assert!((rows[0].lon - (-117.24226)).abs() < 1e-9);
        assert_eq!(rows[1].u, Some(0.08));
        assert_eq!(rows[1].v, None);
        let expected = crate::lsk::days_from_civil(2026, 9, 21).unwrap() as f64 * 86400.0
            + 5.0 * 3600.0;
        let expected = lsk.unix_to_tdb(expected).unwrap();
        assert!((rows[0].t - expected).abs() < 1e-6);
    }

    #[test]
    fn parse_csv_rejects_header_only_and_void() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        assert!(
            parse_csv("time,latitude,longitude,water_u,water_v\nUTC,d,e,m s-1,m s-1\n", &lsk)
                .is_none()
        );
        assert!(parse_csv("", &lsk).is_none());
    }

    #[test]
    fn to_geo_rows_emits_one_record_per_present_component() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        let rows = parse_csv(MEASURED_CSV, &lsk).unwrap();
        let geo = to_geo_rows(&rows);
        assert_eq!(geo.len(), 3);
        assert_eq!(geo[0].comp, crate::geo::COMP_HFR_U);
        assert_eq!(geo[1].comp, crate::geo::COMP_HFR_V);
        assert_eq!(geo[2].comp, crate::geo::COMP_HFR_U);
        assert_eq!(geo[1].val, -0.25);
        assert_eq!(geo[0].lat, 32.5874);
    }

    #[test]
    fn component_names_name_the_measured_components() {
        assert_eq!(
            component_name(crate::geo::COMP_HFR_U),
            Some("hfrnet_rtv_eastward_m_s")
        );
        assert_eq!(
            component_name(crate::geo::COMP_HFR_V),
            Some("hfrnet_rtv_northward_m_s")
        );
        assert_eq!(component_name(99), None);
    }
}
