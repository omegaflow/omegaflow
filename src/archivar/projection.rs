use super::las::LasCrs;

const UTM_K0: f64 = 0.9996;
const UTM_FALSE_EASTING: f64 = 500000.0;
const UTM_FALSE_NORTHING_SOUTH: f64 = 10000000.0;
const EPSG_WGS84: u16 = 4326;

const EPSG_LAMBERT93: u16 = 2154;
const EPSG_LV95: u16 = 2056;
const EPSG_RD_NEW: u16 = 28992;

const GRS80_A: f64 = 6_378_137.0;
const GRS80_F_INV: f64 = 298.257_222_101;
const BESSEL_A: f64 = 6_377_397.155;
const BESSEL_F_INV: f64 = 299.152_812_8;

const LAMBERT93_LAT_0: f64 = 46.5;
const LAMBERT93_LON_0: f64 = 3.0;
const LAMBERT93_LAT_1: f64 = 49.0;
const LAMBERT93_LAT_2: f64 = 44.0;
const LAMBERT93_X_0: f64 = 700_000.0;
const LAMBERT93_Y_0: f64 = 6_600_000.0;

const LV95_LAT_0: f64 = 46.952_405_555_555_6;
const LV95_LON_0: f64 = 7.439_583_333_333_33;
const LV95_K_0: f64 = 1.0;
const LV95_X_0: f64 = 2_600_000.0;
const LV95_Y_0: f64 = 1_200_000.0;

const RD_LAT_0: f64 = 52.156_160_555_555_6;
const RD_LON_0: f64 = 5.387_638_888_888_89;
const RD_K_0: f64 = 0.999_907_9;
const RD_X_0: f64 = 155_000.0;
const RD_Y_0: f64 = 463_000.0;

pub const LAMBERT93_TO_WGS84: [f64; 7] = [0.0; 7];
pub const LV95_TO_WGS84: [f64; 7] = [674.374, 15.056, 405.346, 0.0, 0.0, 0.0, 0.0];
pub const RD_TO_WGS84: [f64; 7] = [565.4171, 50.3319, 465.5524, 1.9342, -1.6677, 9.1019, 4.0725];

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

fn lambert93_inverse(easting: f64, northing: f64) -> Option<(f64, f64)> {
    if !easting.is_finite() || !northing.is_finite() {
        return None;
    }
    let f = 1.0 / GRS80_F_INV;
    let e2 = f * (2.0 - f);
    let e = e2.sqrt();
    let m = |phi: f64| phi.cos() / (1.0 - e2 * phi.sin().powi(2)).sqrt();
    let t = |phi: f64| {
        (std::f64::consts::FRAC_PI_4 - phi / 2.0).tan()
            / ((1.0 - e * phi.sin()) / (1.0 + e * phi.sin())).powf(e / 2.0)
    };
    let p0 = LAMBERT93_LAT_0.to_radians();
    let p1 = LAMBERT93_LAT_1.to_radians();
    let p2 = LAMBERT93_LAT_2.to_radians();
    let n = (m(p1).ln() - m(p2).ln()) / (t(p1).ln() - t(p2).ln());
    if !n.is_finite() || n == 0.0 {
        return None;
    }
    let big_f = m(p1) / (n * t(p1).powf(n));
    let rho0 = GRS80_A * big_f * t(p0).powf(n);
    let x = easting - LAMBERT93_X_0;
    let y = northing - LAMBERT93_Y_0;
    let rho = n.signum() * (x * x + (rho0 - y) * (rho0 - y)).sqrt();
    let theta = x.atan2(rho0 - y);
    let t0 = (rho / (GRS80_A * big_f)).powf(1.0 / n);
    let mut phi = std::f64::consts::FRAC_PI_2 - 2.0 * t0.atan();
    for _ in 0..15 {
        let s = phi.sin();
        let ratio = ((1.0 - e * s) / (1.0 + e * s)).powf(e / 2.0);
        let next = std::f64::consts::FRAC_PI_2 - 2.0 * (t0 * ratio).atan();
        if (next - phi).abs() < 1e-14 {
            phi = next;
            break;
        }
        phi = next;
    }
    let lon = LAMBERT93_LON_0.to_radians() + theta / n;
    if phi.is_finite() && lon.is_finite() {
        Some((phi.to_degrees(), lon.to_degrees()))
    } else {
        None
    }
}

fn lv95_inverse(easting: f64, northing: f64) -> Option<(f64, f64)> {
    if !easting.is_finite() || !northing.is_finite() {
        return None;
    }
    let f = 1.0 / BESSEL_F_INV;
    let e2 = f * (2.0 - f);
    let e = e2.sqrt();
    let one_es = 1.0 - e2;
    let rone_es = 1.0 / one_es;
    let hlf_e = 0.5 * e;
    let phi0 = LV95_LAT_0.to_radians();
    let cp = phi0.cos();
    let c = (1.0 + e2 * cp.powi(4) * rone_es).sqrt();
    let sinp0 = phi0.sin() / c;
    let phip0 = sinp0.asin();
    let cosp0 = phip0.cos();
    let sp = phi0.sin() * e;
    let k = (std::f64::consts::FRAC_PI_4 + phip0 / 2.0).tan().ln()
        - c * ((std::f64::consts::FRAC_PI_4 + phi0 / 2.0).tan().ln()
            - hlf_e * ((1.0 + sp) / (1.0 - sp)).ln());
    let kr = LV95_K_0 * one_es.sqrt() / (1.0 - sp * sp);
    let x = (easting - LV95_X_0) / BESSEL_A;
    let y = (northing - LV95_Y_0) / BESSEL_A;
    let phipp = 2.0 * ((y / kr).exp().atan() - std::f64::consts::FRAC_PI_4);
    let lampp = x / kr;
    let cp2 = phipp.cos();
    let phip = (cosp0 * phipp.sin() + sinp0 * cp2 * lampp.cos()).asin();
    let lamp = (cp2 * lampp.sin() / phip.cos()).asin();
    let con = (k - (std::f64::consts::FRAC_PI_4 + phip / 2.0).tan().ln()) / c;
    let mut phi = phip;
    for _ in 0..6 {
        let esp = e * phi.sin();
        let delp = (con + (std::f64::consts::FRAC_PI_4 + phi / 2.0).tan().ln()
            - hlf_e * ((1.0 + esp) / (1.0 - esp)).ln())
            * (1.0 - esp * esp)
            * phi.cos()
            * rone_es;
        phi -= delp;
        if delp.abs() < 1e-10 {
            break;
        }
    }
    let lon = LV95_LON_0.to_radians() + lamp / c;
    if phi.is_finite() && lon.is_finite() {
        Some((phi.to_degrees(), lon.to_degrees()))
    } else {
        None
    }
}

struct Gauss {
    c: f64,
    k: f64,
    e: f64,
    rc: f64,
    chi: f64,
}

fn gauss_ini(e: f64, phi0: f64) -> Option<Gauss> {
    let es = e * e;
    let sphi = phi0.sin();
    let cphi = phi0.cos().powi(2);
    let rc = (1.0 - es).sqrt() / (1.0 - es * sphi * sphi);
    let c = (1.0 + es * cphi * cphi / (1.0 - es)).sqrt();
    if c == 0.0 {
        return None;
    }
    let chi = (sphi / c).asin();
    let ratexp = 0.5 * c * e;
    let srat_val = ((1.0 - e * sphi) / (1.0 + e * sphi)).powf(ratexp);
    if srat_val == 0.0 {
        return None;
    }
    let k = if 0.5 * phi0 + std::f64::consts::FRAC_PI_4 < 1e-10 {
        1.0 / srat_val
    } else {
        (0.5 * chi + std::f64::consts::FRAC_PI_4).tan()
            / ((0.5 * phi0 + std::f64::consts::FRAC_PI_4).tan().powf(c) * srat_val)
    };
    Some(Gauss { c, k, e, rc, chi })
}

fn inv_gauss(g: &Gauss, phi_s: f64, lam_s: f64) -> (f64, f64) {
    let elam = lam_s / g.c;
    let num = ((0.5 * phi_s + std::f64::consts::FRAC_PI_4).tan() / g.k).powf(1.0 / g.c);
    let mut phi_s = phi_s;
    let mut phi = 0.0;
    for _ in 0..20 {
        let srat_val = ((1.0 - g.e * phi_s.sin()) / (1.0 + g.e * phi_s.sin())).powf(-0.5 * g.e);
        phi = 2.0 * (num * srat_val).atan() - std::f64::consts::FRAC_PI_2;
        if (phi - phi_s).abs() < 1e-14 {
            break;
        }
        phi_s = phi;
    }
    (phi, elam)
}

fn rd_new_inverse(easting: f64, northing: f64) -> Option<(f64, f64)> {
    if !easting.is_finite() || !northing.is_finite() {
        return None;
    }
    let f = 1.0 / BESSEL_F_INV;
    let e2 = f * (2.0 - f);
    let e = e2.sqrt();
    let g = gauss_ini(e, RD_LAT_0.to_radians())?;
    let sinc0 = g.chi.sin();
    let cosc0 = g.chi.cos();
    let r2 = 2.0 * g.rc;
    let x = (easting - RD_X_0) / BESSEL_A / RD_K_0;
    let y = (northing - RD_Y_0) / BESSEL_A / RD_K_0;
    let rho = (x * x + y * y).sqrt();
    let (phi_s, lam_s) = if rho != 0.0 {
        let c = 2.0 * rho.atan2(r2);
        let sinc = c.sin();
        let cosc = c.cos();
        let phi_s = (cosc * sinc0 + y * sinc * cosc0 / rho).asin();
        let lam_s = (x * sinc).atan2(rho * cosc0 * cosc - y * sinc0 * sinc);
        (phi_s, lam_s)
    } else {
        (g.chi, 0.0)
    };
    let (phi, elam) = inv_gauss(&g, phi_s, lam_s);
    let lon = RD_LON_0.to_radians() + elam;
    if phi.is_finite() && lon.is_finite() {
        Some((phi.to_degrees(), lon.to_degrees()))
    } else {
        None
    }
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
        LasCrs::Epsg(EPSG_LAMBERT93) => {
            let (lat, lon) = lambert93_inverse(x, y)?;
            Some((lon, lat, z))
        }
        LasCrs::Epsg(EPSG_LV95) => {
            let (lat, lon) = lv95_inverse(x, y)?;
            Some((lon, lat, z))
        }
        LasCrs::Epsg(EPSG_RD_NEW) => {
            let (lat, lon) = rd_new_inverse(x, y)?;
            Some((lon, lat, z))
        }
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

    #[test]
    fn lambert93_origin_matches_reference() {
        let p = lambert93_inverse(700000.0, 6600000.0).unwrap();
        assert!(near(p.0, 46.5, 1e-6), "{:?}", p);
        assert!(near(p.1, 3.0, 1e-6), "{:?}", p);
    }

    #[test]
    fn lambert93_point_matches_reference() {
        let p = lambert93_inverse(650000.0, 6860000.0).unwrap();
        assert!(near(p.0, 48.8381101226, 1e-6), "{:?}", p);
        assert!(near(p.1, 2.3187905970, 1e-6), "{:?}", p);
    }

    #[test]
    fn lv95_origin_matches_reference() {
        let p = lv95_inverse(2600000.0, 1200000.0).unwrap();
        assert!(near(p.0, 46.9524055556, 1e-6), "{:?}", p);
        assert!(near(p.1, 7.4395833333, 1e-6), "{:?}", p);
    }

    #[test]
    fn lv95_point_matches_reference() {
        let p = lv95_inverse(2683000.0, 1247000.0).unwrap();
        assert!(near(p.0, 47.3699765253, 1e-6), "{:?}", p);
        assert!(near(p.1, 8.5386333992, 1e-6), "{:?}", p);
    }

    #[test]
    fn rd_new_origin_matches_reference() {
        let p = rd_new_inverse(155000.0, 463000.0).unwrap();
        assert!(near(p.0, 52.1561605556, 1e-6), "{:?}", p);
        assert!(near(p.1, 5.3876388889, 1e-6), "{:?}", p);
    }

    #[test]
    fn rd_new_point_matches_reference() {
        let p = rd_new_inverse(100000.0, 400000.0).unwrap();
        assert!(near(p.0, 51.5871380133, 1e-6), "{:?}", p);
        assert!(near(p.1, 4.5939184649, 1e-6), "{:?}", p);
    }

    #[test]
    fn national_grids_report_lon_lat_order() {
        let l = to_geodetic(
            &LasCrs::Epsg(EPSG_LAMBERT93),
            650000.0,
            6860000.0,
            12.0,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(l.0, 2.3187905970, 1e-6), "{:?}", l);
        assert!(near(l.1, 48.8381101226, 1e-6), "{:?}", l);
        assert!(near(l.2, 12.0, 1e-12));
        let s = to_geodetic(
            &LasCrs::Epsg(EPSG_LV95),
            2683000.0,
            1247000.0,
            5.0,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(s.0, 8.5386333992, 1e-6), "{:?}", s);
        assert!(near(s.1, 47.3699765253, 1e-6), "{:?}", s);
        let r = to_geodetic(
            &LasCrs::Epsg(EPSG_RD_NEW),
            100000.0,
            400000.0,
            3.0,
            EARTH_A,
            EARTH_F_INV,
        )
        .unwrap();
        assert!(near(r.0, 4.5939184649, 1e-6), "{:?}", r);
        assert!(near(r.1, 51.5871380133, 1e-6), "{:?}", r);
    }

    #[test]
    fn national_grids_non_finite_is_absent() {
        assert!(lambert93_inverse(f64::NAN, 0.0).is_none());
        assert!(lv95_inverse(0.0, f64::INFINITY).is_none());
        assert!(rd_new_inverse(f64::NAN, f64::NAN).is_none());
    }
}
