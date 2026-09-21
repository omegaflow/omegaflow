use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmoSample {
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

pub fn parse_csv(text: &str, lsk: &LeapSeconds) -> Option<Vec<EmoSample>> {
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
        if cols.len() < 6 {
            continue;
        }
        let Some(t) = parse_iso_tdb(cols[0], lsk) else {
            continue;
        };
        let (Some(lat), Some(lon)) = (csv_f64(cols[2]), csv_f64(cols[3])) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) {
            continue;
        }
        let u = csv_f64(cols[4]);
        let v = csv_f64(cols[5]);
        if u.is_none() && v.is_none() {
            continue;
        }
        out.push(EmoSample {
            t,
            lat,
            lon: wrap_lon(lon),
            u,
            v,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn to_geo_rows(samples: &[EmoSample]) -> Vec<crate::geo::GeoRec> {
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
        crate::geo::COMP_HFR_U => Some("emodnet_hfr_eastward_m_s"),
        crate::geo::COMP_HFR_V => Some("emodnet_hfr_northward_m_s"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_CSV: &str = "time,depth,latitude,longitude,EWCT,NSCT\nUTC,m,degrees_north,degrees_east,m s-1,m s-1\n2026-09-21T10:00:00Z,0.0,45.52685,13.375,NaN,NaN\n2026-09-21T10:00:00Z,0.0,45.580845,13.375,-0.106,-0.02\n2026-09-21T10:00:00Z,0.0,45.607845,13.45225,-0.146,NaN\n";

    #[test]
    fn parse_csv_carries_measured_components_and_drops_absent() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        let rows = parse_csv(MEASURED_CSV, &lsk).expect("the measured CSV parses");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].u, Some(-0.106));
        assert_eq!(rows[0].v, Some(-0.02));
        assert_eq!(rows[0].lat, 45.580845);
        assert_eq!(rows[0].lon, 13.375);
        assert_eq!(rows[1].u, Some(-0.146));
        assert_eq!(rows[1].v, None);
    }

    #[test]
    fn parse_csv_rejects_header_only_and_void() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        assert!(
            parse_csv(
                "time,depth,latitude,longitude,EWCT,NSCT\nUTC,m,d,e,m s-1,m s-1\n",
                &lsk
            )
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
        assert_eq!(geo[1].val, -0.02);
        assert_eq!(geo[0].lat, 45.580845);
    }

    #[test]
    fn component_names_name_the_measured_components() {
        assert_eq!(
            component_name(crate::geo::COMP_HFR_U),
            Some("emodnet_hfr_eastward_m_s")
        );
        assert_eq!(
            component_name(crate::geo::COMP_HFR_V),
            Some("emodnet_hfr_northward_m_s")
        );
        assert_eq!(component_name(99), None);
    }
}
