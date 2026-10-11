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
    if mf.is_finite() {
        Some(mf)
    } else {
        None
    }
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
    if delay.is_finite() {
        Some(delay)
    } else {
        None
    }
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
        assert!(low > zhd_m + zwd_m, "low elevation {low} above the zenith sum");
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
