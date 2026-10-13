use std::sync::OnceLock;

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
            for a in 0..NCOORD {
                row[a] = self.coef(0, 0, a, k)
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

static AACGM14_2025: OnceLock<Option<AacgmV2>> = OnceLock::new();

fn aacgm14_2025() -> Option<&'static AacgmV2> {
    AACGM14_2025
        .get_or_init(|| AacgmV2::parse(AACGM14_2025_TEXT, 2025))
        .as_ref()
}

pub fn aacgm_v2_latitude(
    date_year: f64,
    geodetic_lat_deg: f64,
    lon_deg: f64,
    altitude_km: f64,
) -> Option<f64> {
    if !date_year.is_finite() || !(2025.0..2030.0).contains(&date_year) {
        return None;
    }
    let model = aacgm14_2025()?;
    let (lat, _lon) = model.geodetic_to_aacgm(geodetic_lat_deg, lon_deg, altitude_km)?;
    Some(lat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_committed_epoch_2025_file_loads_with_the_full_coefficient_count() {
        let model = AacgmV2::parse(AACGM14_2025_TEXT, 2025).expect("the committed file parses");
        assert_eq!(model.epoch(), 2025);
        assert_eq!(model.values.len(), NFLAG * POLYORD * NCOORD * AACGM_KMAX);
    }

    #[test]
    fn a_date_outside_the_pinned_epoch_returns_none() {
        assert!(aacgm_v2_latitude(2024.9, 45.5, -23.5, 0.0).is_none());
        assert!(aacgm_v2_latitude(2030.0, 45.5, -23.5, 0.0).is_none());
        assert!(aacgm_v2_latitude(f64::NAN, 45.5, -23.5, 0.0).is_none());
    }

    #[test]
    fn a_geodetic_latitude_outside_its_range_returns_none() {
        assert!(aacgm_v2_latitude(2025.0, 95.0, 0.0, 0.0).is_none());
    }
}
