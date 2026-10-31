pub const VMF3_SLANT_DELAY_CONVENTION: &str = "slant troposphere delay = mf_h(el)*zhd + mf_w(el)*zwd; mf(el) = (1 + a/(1 + b/(1 + c))) / (sin el + a/(sin el + b/(sin el + c))), el = pi/2 - zd; mf(pi/2) = 1 so the zenith delay flows at zenith; a is discrete (VMF3 site-wise/grid product), b and c are the empirical VMF3 degree-12 spherical-harmonic model (TU Wien vmf3.m, Landskron & Boehm 2018); a non-finite or non-positive coefficient, a non-finite or negative zenith delay, or an elevation outside (0, pi/2] -> absent (None)";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vmf3Coefficients {
    pub a_h: Option<f64>,
    pub b_h: Option<f64>,
    pub c_h: Option<f64>,
    pub a_w: Option<f64>,
    pub b_w: Option<f64>,
    pub c_w: Option<f64>,
}

fn coefficient_of(value: Option<f64>) -> Option<f64> {
    match value {
        Some(v) if v.is_finite() && v > 0.0 => Some(v),
        _ => None,
    }
}

fn mapping_factor(elevation_rad: f64, a: f64, b: f64, c: f64) -> Option<f64> {
    let sin_el = elevation_rad.sin();
    let numerator = 1.0 + a / (1.0 + b / (1.0 + c));
    let denominator = sin_el + a / (sin_el + b / (sin_el + c));
    let mf = numerator / denominator;
    if mf.is_finite() { Some(mf) } else { None }
}

pub fn slant_delay_m(
    elevation_rad: f64,
    zhd_m: f64,
    zwd_m: f64,
    coefficients: Vmf3Coefficients,
) -> Option<f64> {
    let Vmf3Coefficients {
        a_h,
        b_h,
        c_h,
        a_w,
        b_w,
        c_w,
    } = coefficients;
    if !elevation_rad.is_finite()
        || elevation_rad <= 0.0
        || elevation_rad > std::f64::consts::FRAC_PI_2
    {
        return None;
    }
    if !zhd_m.is_finite() || zhd_m < 0.0 || !zwd_m.is_finite() || zwd_m < 0.0 {
        return None;
    }
    let (a_h, b_h, c_h) = (
        coefficient_of(a_h)?,
        coefficient_of(b_h)?,
        coefficient_of(c_h)?,
    );
    let (a_w, b_w, c_w) = (
        coefficient_of(a_w)?,
        coefficient_of(b_w)?,
        coefficient_of(c_w)?,
    );
    let mfh = mapping_factor(elevation_rad, a_h, b_h, c_h)?;
    let mfw = mapping_factor(elevation_rad, a_w, b_w, c_w)?;
    let delay = mfh * zhd_m + mfw * zwd_m;
    if delay.is_finite() { Some(delay) } else { None }
}

pub const SAASTAMOINEN_DRY_CONVENTION: &str = "dry zenith delay = 0.0022768 * pressure_hpa / (1 - 0.00266 cos(2 latitude) - 0.00028 height_km) in metres, the Saastamoinen hydrostatic term without the humidity part: the coefficient is the refractivity integral at 45 degrees, and the two corrections carry the latitude flattening and the station height; a non-finite or non-positive pressure, a non-finite height outside -1 to 100 km, or a latitude outside -90 to 90 degrees -> absent (None)";

pub fn dry_zenith_delay_m(pressure_hpa: f64, height_m: f64, latitude_deg: f64) -> Option<f64> {
    if !pressure_hpa.is_finite() || pressure_hpa <= 0.0 {
        return None;
    }
    if !height_m.is_finite() || !(-1000.0..=100_000.0).contains(&height_m) {
        return None;
    }
    if !latitude_deg.is_finite() || !(-90.0..=90.0).contains(&latitude_deg) {
        return None;
    }
    let height_km = height_m / 1000.0;
    let denominator = 1.0 - 0.00266 * (2.0 * latitude_deg.to_radians()).cos() - 0.00028 * height_km;
    if !denominator.is_finite() || denominator <= 0.0 {
        return None;
    }
    let delay = 0.0022768 * pressure_hpa / denominator;
    if delay.is_finite() { Some(delay) } else { None }
}

pub fn height_scale_m(temperature_k: f64) -> Option<f64> {
    if !temperature_k.is_finite() || temperature_k <= 0.0 {
        return None;
    }
    let scale = 287.05 * temperature_k / 9.80665;
    if scale.is_finite() { Some(scale) } else { None }
}

pub const SAASTAMOINEN_SIMPLE_DRY_CONVENTION: &str = "dry zenith delay = 0.0022768 * pressure_hpa in metres, the plain hydrostatic term without the latitude and station-height corrections; this is the form the Rosetta flyby package carries in TROPO_DRY, and the corrected form differs from it by the cos(2 latitude) and height terms; a non-finite or non-positive pressure -> absent (None)";

pub fn dry_zenith_delay_simple_m(pressure_hpa: f64) -> Option<f64> {
    if !pressure_hpa.is_finite() || pressure_hpa <= 0.0 {
        return None;
    }
    let delay = 0.0022768 * pressure_hpa;
    if delay.is_finite() { Some(delay) } else { None }
}

pub fn vapour_pressure_hpa(temperature_k: f64, humidity_percent: f64) -> Option<f64> {
    if !temperature_k.is_finite() || temperature_k <= 0.0 {
        return None;
    }
    if !humidity_percent.is_finite() || !(0.0..=100.0).contains(&humidity_percent) {
        return None;
    }
    let celsius = temperature_k - 273.15;
    let saturation = 6.112 * (17.67 * celsius / (celsius + 243.5)).exp();
    let partial = saturation * humidity_percent / 100.0;
    if partial.is_finite() {
        Some(partial)
    } else {
        None
    }
}

pub fn wet_zenith_delay_m(temperature_k: f64, humidity_percent: f64) -> Option<f64> {
    let vapour = vapour_pressure_hpa(temperature_k, humidity_percent)?;
    if !temperature_k.is_finite() || temperature_k <= 0.0 {
        return None;
    }
    let delay = 0.002277 * (1255.0 / temperature_k + 0.05) * vapour;
    if delay.is_finite() { Some(delay) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aggo_coefficients() -> Vmf3Coefficients {
        Vmf3Coefficients {
            a_h: Some(0.00126424),
            b_h: Some(2.71285863109945e-3),
            c_h: Some(5.71481238161787e-2),
            a_w: Some(0.00043626),
            b_w: Some(1.36127467401223e-3),
            c_w: Some(3.95329695826997e-2),
        }
    }

    #[test]
    fn the_plain_term_matches_the_package_form() {
        let delay = dry_zenith_delay_simple_m(983.7).expect("a delay");
        assert!((delay - 2.239694).abs() < 1e-6);
        assert!(dry_zenith_delay_simple_m(0.0).is_none());
    }

    #[test]
    fn the_corrected_term_sits_above_the_plain_one_in_the_tropics() {
        let plain = dry_zenith_delay_simple_m(983.7).expect("a delay");
        let corrected = dry_zenith_delay_m(983.7, 252.2, -31.048).expect("a delay");
        assert!(corrected > plain);
        assert!((corrected - plain - 0.00295).abs() < 5e-5);
    }

    #[test]
    fn the_wet_delay_follows_vapour_pressure() {
        let vapour = vapour_pressure_hpa(293.35, 51.9).expect("a pressure");
        assert!((vapour - 12.28).abs() < 0.02);
        let wet = wet_zenith_delay_m(293.35, 51.9).expect("a delay");
        assert!((wet - 0.12102).abs() < 5e-4);
        assert!(wet_zenith_delay_m(293.35, 120.0).is_none());
    }

    #[test]
    fn the_dry_delay_falls_with_height_at_about_a_quarter_millimetre_per_metre() {
        let low = dry_zenith_delay_m(1013.25, 0.0, 45.0).expect("a delay");
        let high = dry_zenith_delay_m(975.0, 300.0, 45.0).expect("a delay");
        assert!((low - 2.3139).abs() < 1e-3);
        assert!(high < low);
        let gradient = (low - high) / 300.0;
        assert!((0.00020..0.00035).contains(&gradient));
    }

    #[test]
    fn the_southern_latitude_correction_is_small_and_negative() {
        let north = dry_zenith_delay_m(980.0, 300.0, 31.0).expect("a delay");
        let south = dry_zenith_delay_m(980.0, 300.0, -31.0).expect("a delay");
        assert!((north - south).abs() < 1e-12);
        assert!((north - 2.2310).abs() < 1e-3);
    }

    #[test]
    fn an_absent_pressure_is_absent_not_zero_delay() {
        assert!(dry_zenith_delay_m(0.0, 100.0, 0.0).is_none());
        assert!(dry_zenith_delay_m(f64::NAN, 100.0, 0.0).is_none());
        assert!(dry_zenith_delay_m(1013.0, -5000.0, 0.0).is_none());
        assert!(dry_zenith_delay_m(1013.0, 100.0, 91.0).is_none());
    }

    #[test]
    fn the_height_scale_follows_the_temperature() {
        let scale = height_scale_m(292.05).expect("a scale");
        assert!((scale - 8549.4).abs() < 1.0);
        assert!(height_scale_m(0.0).is_none());
    }

    #[test]
    fn zenith_elevation_flows_the_zenith_delay() {
        let zhd_m = 2.2940;
        let zwd_m = 0.1199;
        let delay = slant_delay_m(
            std::f64::consts::FRAC_PI_2,
            zhd_m,
            zwd_m,
            aggo_coefficients(),
        )
        .expect("the complete coefficient set carries a delay");
        assert!(
            (delay - (zhd_m + zwd_m)).abs() < 1.0e-9,
            "zenith delay {delay} vs zenith sum {}",
            zhd_m + zwd_m
        );
    }

    #[test]
    fn below_zenith_the_delay_exceeds_the_zenith_delay() {
        let zhd_m = 2.2940;
        let zwd_m = 0.1199;
        let low = slant_delay_m(0.1, zhd_m, zwd_m, aggo_coefficients()).expect("a finite delay");
        let high = slant_delay_m(1.5, zhd_m, zwd_m, aggo_coefficients()).expect("a finite delay");
        assert!(
            low > zhd_m + zwd_m,
            "low elevation {low} above the zenith sum"
        );
        assert!(
            low > high,
            "delay grows toward the horizon: low {low} vs high {high}"
        );
    }

    #[test]
    fn non_finite_elevation_stays_absent() {
        assert!(slant_delay_m(f64::NAN, 2.2940, 0.1199, aggo_coefficients()).is_none());
        assert!(slant_delay_m(f64::INFINITY, 2.2940, 0.1199, aggo_coefficients()).is_none());
        assert!(slant_delay_m(0.0, 2.2940, 0.1199, aggo_coefficients()).is_none());
        assert!(slant_delay_m(-0.5, 2.2940, 0.1199, aggo_coefficients()).is_none());
    }

    #[test]
    fn absent_or_fabricated_coefficient_stays_absent() {
        let mut absent = aggo_coefficients();
        absent.b_h = None;
        assert!(slant_delay_m(0.5, 2.2940, 0.1199, absent).is_none());

        let mut fabricated = aggo_coefficients();
        fabricated.c_w = Some(0.0);
        assert!(slant_delay_m(0.5, 2.2940, 0.1199, fabricated).is_none());

        let mut non_finite = aggo_coefficients();
        non_finite.a_h = Some(f64::NAN);
        assert!(slant_delay_m(0.5, 2.2940, 0.1199, non_finite).is_none());
    }
}
