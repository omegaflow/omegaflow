use super::las::LasCrs;

const UTM_K0: f64 = 0.9996;
const UTM_FALSE_EASTING: f64 = 500000.0;
const UTM_FALSE_NORTHING_SOUTH: f64 = 10000000.0;
const EPSG_WGS84: u16 = 4326;

pub fn utm_inverse(
    epsg: u16,
    easting: f64,
    northing: f64,
    a: f64,
    f_inv: f64,
) -> Option<(f64, f64)> {
    let (zone, south) = if (32601..=32660).contains(&epsg) {
        ((epsg - 32600) as f64, false)
    } else if (32701..=32760).contains(&epsg) {
        ((epsg - 32700) as f64, true)
    } else {
        return None;
    };
    if !easting.is_finite() || !northing.is_finite() {
        return None;
    }
    let f = 1.0 / f_inv;
    let e2 = f * (2.0 - f);
    let ep2 = e2 / (1.0 - e2);
    if !e2.is_finite() || !ep2.is_finite() {
        return None;
    }
    let false_northing = if south { UTM_FALSE_NORTHING_SOUTH } else { 0.0 };
    let m = (northing - false_northing) / UTM_K0;
    if !m.is_finite() {
        return None;
    }
    let root = (1.0 - e2).sqrt();
    if !root.is_finite() {
        return None;
    }
    let e1 = (1.0 - root) / (1.0 + root);
    if !e1.is_finite() {
        return None;
    }
    let mu = m / (a * (1.0 - e2 / 4.0 - 3.0 * e2 * e2 / 64.0 - 5.0 * e2 * e2 * e2 / 256.0));
    if !mu.is_finite() {
        return None;
    }
    let phi1 = mu
        + (3.0 * e1 / 2.0 - 27.0 * e1 * e1 * e1 / 32.0) * (2.0 * mu).sin()
        + (21.0 * e1 * e1 / 16.0 - 55.0 * e1 * e1 * e1 * e1 / 32.0) * (4.0 * mu).sin()
        + (151.0 * e1 * e1 * e1 / 96.0) * (6.0 * mu).sin()
        + (1097.0 * e1 * e1 * e1 * e1 / 512.0) * (8.0 * mu).sin();
    if !phi1.is_finite() {
        return None;
    }
    let sin_phi1 = phi1.sin();
    let cos_phi1 = phi1.cos();
    let tan_phi1 = phi1.tan();
    let c1 = ep2 * cos_phi1 * cos_phi1;
    let t1 = tan_phi1 * tan_phi1;
    let denom = (1.0 - e2 * sin_phi1 * sin_phi1).sqrt();
    if !denom.is_finite() || denom == 0.0 {
        return None;
    }
    let n1 = a / denom;
    let r1 = a * (1.0 - e2) / (denom * denom * denom);
    if !n1.is_finite() || !r1.is_finite() {
        return None;
    }
    let d = (easting - UTM_FALSE_EASTING) / (n1 * UTM_K0);
    if !d.is_finite() {
        return None;
    }
    let lat = phi1
        - (n1 * tan_phi1 / r1)
            * (d * d / 2.0
                - (5.0 + 3.0 * t1 + 10.0 * c1 - 4.0 * c1 * c1 - 9.0 * ep2) * d * d * d * d / 24.0
                + (61.0 + 90.0 * t1 + 298.0 * c1 + 45.0 * t1 * t1 - 252.0 * ep2 - 3.0 * c1 * c1)
                    * d
                    * d
                    * d
                    * d
                    * d
                    * d
                    / 720.0);
    if !lat.is_finite() {
        return None;
    }
    let lon = (zone - 1.0) * 6.0 - 180.0
        + 3.0
        + (d - (1.0 + 2.0 * t1 + c1) * d * d * d / 6.0
            + (5.0 - 2.0 * c1 + 28.0 * t1 - 3.0 * c1 * c1 + 8.0 * ep2 + 24.0 * t1 * t1)
                * d
                * d
                * d
                * d
                * d
                / 120.0)
            / cos_phi1;
    if !lon.is_finite() {
        return None;
    }
    Some((lat.to_degrees(), lon.to_degrees()))
}

pub fn to_geodetic(
    crs: &LasCrs,
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    f_inv: f64,
) -> Option<(f64, f64, f64)> {
    match crs {
        LasCrs::Epsg(EPSG_WGS84) => Some((x, y, z)),
        LasCrs::Epsg(code) => {
            let (lat, lon) = utm_inverse(*code, x, y, a, f_inv)?;
            Some((lon, lat, z))
        }
        LasCrs::Wkt(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EARTH_A: f64 = 6378137.0;
    const EARTH_F_INV: f64 = 298.257223563;

    fn near(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn north_zone33_inside_matches_reference() {
        let p = utm_inverse(
            32633,
            426128.6021191576,
            5372358.413914613,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(p.0, 48.5, 1e-6), "{:?}", p);
        assert!(near(p.1, 14.0, 1e-6), "{:?}", p);
    }

    #[test]
    fn north_zone33_second_matches_reference() {
        let p = utm_inverse(
            32633,
            607498.4933613017,
            5539708.756779527,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(p.0, 50.0, 1e-6), "{:?}", p);
        assert!(near(p.1, 16.5, 1e-6), "{:?}", p);
    }

    #[test]
    fn north_zone33_central_meridian_matches_reference() {
        let p = utm_inverse(32633, 500000.0, 4982950.400226552, EARTH_A, EARTH_F_INV).unwrap();
        assert!(near(p.0, 45.0, 1e-6), "{:?}", p);
        assert!(near(p.1, 15.0, 1e-6), "{:?}", p);
    }

    #[test]
    fn south_zone33_matches_reference() {
        let p = utm_inverse(
            32733,
            313152.77721466526,
            6346936.4957081545,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(p.0, -33.0, 1e-6), "{:?}", p);
        assert!(near(p.1, 13.0, 1e-6), "{:?}", p);
    }

    #[test]
    fn unsupported_epsg_is_absent() {
        assert!(utm_inverse(32661, 0.0, 0.0, EARTH_A, EARTH_F_INV).is_none());
        assert!(utm_inverse(32600, 0.0, 0.0, EARTH_A, EARTH_F_INV).is_none());
        assert!(utm_inverse(9999, 0.0, 0.0, EARTH_A, EARTH_F_INV).is_none());
    }

    #[test]
    fn non_finite_input_is_absent() {
        assert!(utm_inverse(32633, f64::NAN, 0.0, EARTH_A, EARTH_F_INV).is_none());
        assert!(utm_inverse(32633, 0.0, f64::INFINITY, EARTH_A, EARTH_F_INV).is_none());
    }

    #[test]
    fn wgs84_identity_passes_through() {
        let g = to_geodetic(
            &LasCrs::Epsg(EPSG_WGS84),
            9.5,
            48.25,
            120.0,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(g.0, 9.5, 1e-12));
        assert!(near(g.1, 48.25, 1e-12));
        assert!(near(g.2, 120.0, 1e-12));
    }

    #[test]
    fn utm_crs_reports_lon_lat_order() {
        let g = to_geodetic(
            &LasCrs::Epsg(32633),
            426128.6021191576,
            5372358.413914613,
            7.0,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(g.0, 14.0, 1e-6), "{:?}", g);
        assert!(near(g.1, 48.5, 1e-6), "{:?}", g);
        assert!(near(g.2, 7.0, 1e-12));
    }

    #[test]
    fn wkt_crs_is_absent() {
        assert!(
            to_geodetic(
                &LasCrs::Wkt("PROJCS[...]".to_string()),
                1.0,
                2.0,
                3.0,
                EARTH_A,
                EARTH_F_INV
            )
            .is_none()
        );
    }
}
