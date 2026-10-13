use std::borrow::Cow;
use std::sync::OnceLock;

pub const AACGM14_2010_TEXT: &str = include_str!("aacgm_coeffs-14-2010.asc");
pub const AACGM14_2015_TEXT: &str = include_str!("aacgm_coeffs-14-2015.asc");
pub const AACGM14_2020_TEXT: &str = include_str!("aacgm_coeffs-14-2020.asc");
pub const AACGM14_2025_TEXT: &str = include_str!("aacgm_coeffs-14-2025.asc");

const RE_KM: f64 = 6371.2;
const MAXALT_KM: f64 = 2000.0;
const SHORDER: usize = 10;
const AACGM_KMAX: usize = (SHORDER + 1) * (SHORDER + 1);
const NCOORD: usize = 3;
const POLYORD: usize = 5;
const NFLAG: usize = 2;
const WGS84_A_KM: f64 = 6378.1370;
const WGS84_F: f64 = 1.0 / 298.257223563;

#[derive(Clone, Debug)]
pub struct AacgmV2 {
    epoch: u16,
    values: Vec<f64>,
}

impl AacgmV2 {
    pub fn parse(text: &str, epoch: u16) -> Option<AacgmV2> {
        let values: Vec<f64> = text
            .split_whitespace()
            .map(|t| t.parse::<f64>().ok())
            .collect::<Option<Vec<f64>>>()?;
        if values.len() != NFLAG * POLYORD * NCOORD * AACGM_KMAX {
            return None;
        }
        Some(AacgmV2 { epoch, values })
    }

    pub fn epoch(&self) -> u16 {
        self.epoch
    }

    pub fn interpolate(lower: &AacgmV2, upper: &AacgmV2, fyear: f64) -> Option<AacgmV2> {
        if lower.values.len() != upper.values.len() || !fyear.is_finite() {
            return None;
        }
        let fraction = (fyear - f64::from(lower.epoch)) / 5.0;
        if !(0.0..=1.0).contains(&fraction) {
            return None;
        }
        let values = lower
            .values
            .iter()
            .zip(upper.values.iter())
            .map(|(&lo, &hi)| lo + fraction * (hi - lo))
            .collect();
        Some(AacgmV2 {
            epoch: lower.epoch,
            values,
        })
    }

    fn coef(&self, f: usize, l: usize, a: usize, t: usize) -> f64 {
        self.values[((f * POLYORD + l) * NCOORD + a) * AACGM_KMAX + t]
    }

    pub fn geodetic_to_aacgm(
        &self,
        geodetic_lat_deg: f64,
        lon_deg: f64,
        altitude_km: f64,
    ) -> Option<(f64, f64)> {
        if !geodetic_lat_deg.is_finite() || !lon_deg.is_finite() || !altitude_km.is_finite() {
            return None;
        }
        if !(-90.0..=90.0).contains(&geodetic_lat_deg) {
            return None;
        }
        let (r_re, geocentric_colat_rad, geocentric_lon_rad) =
            geodetic_to_geocentric(geodetic_lat_deg, lon_deg, altitude_km);
        let geocentric_lat_deg = 90.0 - geocentric_colat_rad.to_degrees();
        let geocentric_lon_deg = geocentric_lon_rad.to_degrees();
        let geocentric_alt_km = (r_re - 1.0) * RE_KM;
        if geocentric_alt_km > MAXALT_KM {
            return None;
        }
        self.evaluate(geocentric_lat_deg, geocentric_lon_deg, geocentric_alt_km)
    }

    fn evaluate(
        &self,
        geocentric_lat_deg: f64,
        geocentric_lon_deg: f64,
        geocentric_alt_km: f64,
    ) -> Option<(f64, f64)> {
        let alt_var = geocentric_alt_km / MAXALT_KM;
        let alt_var_sq = alt_var * alt_var;
        let alt_var_cu = alt_var * alt_var_sq;
        let alt_var_qu = alt_var * alt_var_cu;

        let mut cint = [[0.0f64; NCOORD]; AACGM_KMAX];
        for (k, row) in cint.iter_mut().enumerate() {
            for (a, cell) in row.iter_mut().enumerate() {
                *cell = self.coef(0, 0, a, k)
                    + self.coef(0, 1, a, k) * alt_var
                    + self.coef(0, 2, a, k) * alt_var_sq
                    + self.coef(0, 3, a, k) * alt_var_cu
                    + self.coef(0, 4, a, k) * alt_var_qu;
            }
        }

        let colat_input = (90.0 - geocentric_lat_deg).to_radians();
        let lon_input = geocentric_lon_deg.to_radians();
        let mut ylmval = [0.0f64; AACGM_KMAX];
        real_spherical_harmonics(colat_input, lon_input, SHORDER, &mut ylmval);

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for l in 0..=SHORDER {
            for m in -(l as i64)..=(l as i64) {
                let k = (l * (l + 1)) as i64 + m;
                let k = k as usize;
                x += cint[k][0] * ylmval[k];
                y += cint[k][1] * ylmval[k];
                z += cint[k][2] * ylmval[k];
            }
        }

        let fac = x * x + y * y;
        if fac > 1.0 {
            return None;
        }
        let ztmp = (1.0 - fac).sqrt();
        let z = if z < 0.0 { -ztmp } else { ztmp };
        let colat_temp = z.clamp(-1.0, 1.0).acos();
        let lon_temp = if x.abs() < 1e-8 && y.abs() < 1e-8 {
            0.0
        } else {
            y.atan2(x)
        };
        Some((90.0 - colat_temp.to_degrees(), lon_temp.to_degrees()))
    }
}

fn geodetic_to_geocentric(lat_deg: f64, lon_deg: f64, alt_km: f64) -> (f64, f64, f64) {
    let a = WGS84_A_KM;
    let b = a * (1.0 - WGS84_F);
    let a2 = a * a;
    let b2 = b * b;
    let theta = (90.0 - lat_deg).to_radians();
    let st = theta.sin();
    let ct = theta.cos();
    let one = a2 * st * st;
    let two = b2 * ct * ct;
    let three = one + two;
    let rho = three.sqrt();
    let r = (alt_km * (alt_km + 2.0 * rho) + (a2 * one + b2 * two) / three).sqrt();
    let cd = (alt_km + rho) / r;
    let sd = (a2 - b2) / rho * ct * st / r;
    let r_re = r / RE_KM;
    let colat_rad = (ct * cd - st * sd).clamp(-1.0, 1.0).acos();
    (r_re, colat_rad, lon_deg.to_radians())
}

fn real_spherical_harmonics(colat: f64, lon: f64, order: usize, ylmval: &mut [f64]) {
    let cos_theta = colat.cos();
    let sin_theta = colat.sin();
    let cos_lon = lon.cos();
    let sin_lon = lon.sin();

    let q_fac_x = -sin_theta * cos_lon;
    let q_fac_y = -sin_theta * sin_lon;

    ylmval[0] = 1.0;
    ylmval[2] = cos_theta;
    for l in 2..=order {
        let ia = (l - 2) * (l - 1);
        let ib = (l - 1) * l;
        let ic = l * (l + 1);
        ylmval[ic] =
            (cos_theta * (2 * l - 1) as f64 * ylmval[ib] - (l - 1) as f64 * ylmval[ia]) / l as f64;
    }

    let mut qx = q_fac_x;
    let mut qy = q_fac_y;
    ylmval[3] = qx;
    ylmval[1] = -qy;
    for l in 2..=order {
        let d1 = (2 * l - 1) as f64;
        let z2x = d1 * q_fac_x;
        let z2y = d1 * q_fac_y;
        let z1x = z2x * qx - z2y * qy;
        let z1y = z2x * qy + z2y * qx;
        qx = z1x;
        qy = z1y;
        ylmval[l * (l + 2)] = qx;
        ylmval[l * l] = -qy;
    }

    for l in 2..=order {
        let l2 = l * l;
        let tl = 2 * l;
        let ia = l2 - 1;
        let ib = l2 - tl + 1;
        let ic = l2 + tl - 1;
        let id = l2 + 1;
        let fac = (tl - 1) as f64;
        ylmval[ic] = fac * cos_theta * ylmval[ia];
        ylmval[id] = fac * cos_theta * ylmval[ib];
    }

    if order >= 2 {
        for m in 1..=(order - 2) {
            for l in (m + 2)..=order {
                let ca = (2 * l - 1) as f64 / (l - m) as f64;
                let cb = (l + m - 1) as f64 / (l - m) as f64;
                let l2 = l * l;
                let mut ic = l2 + l + m;
                let mut ib = l2 - l + m;
                let mut ia = l2 - l - l - l + 2 + m;
                ylmval[ic] = ca * cos_theta * ylmval[ib] - cb * ylmval[ia];
                ic -= 2 * m;
                ib -= 2 * m;
                ia -= 2 * m;
                ylmval[ic] = ca * cos_theta * ylmval[ib] - cb * ylmval[ia];
            }
        }
    }

    let mut fact = vec![1.0f64; 2 * order + 2];
    for k in 2..=(2 * order + 1) {
        fact[k] = k as f64 * fact[k - 1];
    }
    let mut ffff = vec![0.0f64; (order + 1) * (order + 1)];
    for l in 0..=order {
        for m in 0..=l {
            let k = l * (l + 1) + m;
            ffff[k] = ((2 * l + 1) as f64 / (4.0 * std::f64::consts::PI) * fact[l - m]
                / fact[l + m])
                .sqrt();
            ylmval[k] *= ffff[k];
        }
        for m in 1..=l {
            let k = l * (l + 1) - m;
            let kk = l * (l + 1) + m;
            let sign = if m % 2 == 1 { -1.0 } else { 1.0 };
            ylmval[k] *= ffff[kk] * sign;
        }
    }
}

static AACGM14_2010: OnceLock<Option<AacgmV2>> = OnceLock::new();
static AACGM14_2015: OnceLock<Option<AacgmV2>> = OnceLock::new();
static AACGM14_2020: OnceLock<Option<AacgmV2>> = OnceLock::new();
static AACGM14_2025: OnceLock<Option<AacgmV2>> = OnceLock::new();

fn epoch_model(epoch: u16) -> Option<&'static AacgmV2> {
    let (cell, text): (&OnceLock<Option<AacgmV2>>, &str) = match epoch {
        2010 => (&AACGM14_2010, AACGM14_2010_TEXT),
        2015 => (&AACGM14_2015, AACGM14_2015_TEXT),
        2020 => (&AACGM14_2020, AACGM14_2020_TEXT),
        2025 => (&AACGM14_2025, AACGM14_2025_TEXT),
        _ => return None,
    };
    cell.get_or_init(|| AacgmV2::parse(text, epoch)).as_ref()
}

fn model_for_year(date_year: f64) -> Option<Cow<'static, AacgmV2>> {
    if !(2010.0..2030.0).contains(&date_year) {
        return None;
    }
    let lower_epoch = (date_year / 5.0).floor() as u16 * 5;
    let lower = epoch_model(lower_epoch)?;
    match epoch_model(lower_epoch + 5) {
        Some(upper) => AacgmV2::interpolate(lower, upper, date_year).map(Cow::Owned),
        None => Some(Cow::Borrowed(lower)),
    }
}

pub fn aacgm_v2_latitude(
    date_year: f64,
    geodetic_lat_deg: f64,
    lon_deg: f64,
    altitude_km: f64,
) -> Option<f64> {
    let model = model_for_year(date_year)?;
    let (lat, _lon) = model.geodetic_to_aacgm(geodetic_lat_deg, lon_deg, altitude_km)?;
    Some(lat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_committed_epoch_file_loads_with_the_full_coefficient_count() {
        for (epoch, text) in [
            (2010u16, AACGM14_2010_TEXT),
            (2015, AACGM14_2015_TEXT),
            (2020, AACGM14_2020_TEXT),
            (2025, AACGM14_2025_TEXT),
        ] {
            let model = AacgmV2::parse(text, epoch).expect("the committed file parses");
            assert_eq!(model.epoch(), epoch);
            assert_eq!(model.values.len(), NFLAG * POLYORD * NCOORD * AACGM_KMAX);
        }
    }

    #[test]
    fn a_date_outside_the_published_epochs_returns_none() {
        assert!(aacgm_v2_latitude(2009.9, 45.5, -23.5, 0.0).is_none());
        assert!(aacgm_v2_latitude(2030.0, 45.5, -23.5, 0.0).is_none());
        assert!(aacgm_v2_latitude(f64::NAN, 45.5, -23.5, 0.0).is_none());
    }

    #[test]
    fn a_geodetic_latitude_outside_its_range_returns_none() {
        assert!(aacgm_v2_latitude(2025.0, 95.0, 0.0, 0.0).is_none());
    }

    #[test]
    fn the_bracketing_pair_blends_linearly_between_its_epochs() {
        let lower = epoch_model(2015).expect("2015 parses");
        let upper = epoch_model(2020).expect("2020 parses");
        let midpoint = model_for_year(2017.5).expect("2017.5 is inside the window");
        for (idx, (&lo, &hi)) in lower.values.iter().zip(upper.values.iter()).enumerate() {
            let expected = lo + 0.5 * (hi - lo);
            assert!(
                (midpoint.values[idx] - expected).abs() < 1e-12,
                "coefficient {idx} is not the epoch midpoint"
            );
        }
        let anchored = model_for_year(2015.0).expect("2015 is inside the window");
        assert_eq!(anchored.values, lower.values);
    }

    #[test]
    fn upstream_c_test_witnesses_reproduce_through_two_epoch_interpolation() {
        let fyear_2014 = 2014.0 + (80.0 + (3.0 + 11.0 / 60.0) / 24.0) / 365.0;
        let cases = [
            (fyear_2014, 45.5, -23.5, 1135.0, 48.1902, 57.7505),
            (2018.0, 60.0, 0.0, 300.0, 58.2199, 80.7290),
        ];
        for (fyear, lat_in, lon_in, alt_in, lat_expected, lon_expected) in cases {
            let model = model_for_year(fyear).expect("the bracketing epochs load");
            let (lat, lon) = model
                .geodetic_to_aacgm(lat_in, lon_in, alt_in)
                .expect("the witness coordinate converts");
            assert!(
                (lat - lat_expected).abs() < 1e-4,
                "latitude {lat} differs from {lat_expected} at fyear {fyear}"
            );
            assert!(
                (lon - lon_expected).abs() < 1e-4,
                "longitude {lon} differs from {lon_expected} at fyear {fyear}"
            );
        }
    }
}
