use super::*;

pub const ARCSEC_TO_RAD: f64 = std::f64::consts::PI / (180.0 * 3600.0);

pub const JD_B1950: f64 = 2433282.42345905;
pub const JD_J2000: f64 = 2451545.0;
pub const BESSEL_1900: f64 = 2415020.31352;
pub const TROPICAL_YEAR: f64 = 365.242198781;

pub const EARTH_ROT_RAD_S: f64 = 7.2921150e-5;

fn rot_x(a: f64) -> [f64; 9] {
    let (s, c) = a.sin_cos();
    [1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c]
}

fn rot_y(a: f64) -> [f64; 9] {
    let (s, c) = a.sin_cos();
    [c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c]
}

fn rot_z(a: f64) -> [f64; 9] {
    let (s, c) = a.sin_cos();
    [c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0]
}

pub fn mat3_mul(a: &[f64; 9], b: &[f64; 9]) -> [f64; 9] {
    let mut o = [0.0f64; 9];
    for r in 0..3 {
        for c in 0..3 {
            o[r * 3 + c] = a[r * 3] * b[c] + a[r * 3 + 1] * b[3 + c] + a[r * 3 + 2] * b[6 + c];
        }
    }
    o
}

pub fn mat3_vec(m: &[f64; 9], v: [f64; 3]) -> [f64; 3] {
    [
        m[0] * v[0] + m[1] * v[1] + m[2] * v[2],
        m[3] * v[0] + m[4] * v[1] + m[5] * v[2],
        m[6] * v[0] + m[7] * v[1] + m[8] * v[2],
    ]
}

pub fn mat3_transpose(m: &[f64; 9]) -> [f64; 9] {
    [m[0], m[3], m[6], m[1], m[4], m[7], m[2], m[5], m[8]]
}

pub fn vec3_norm_u(v: [f64; 3]) -> Option<[f64; 3]> {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if n.is_finite() && n > 0.0 {
        Some([v[0] / n, v[1] / n, v[2] / n])
    } else {
        None
    }
}

pub fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

const NUT80: [[f64; 9]; 106] = [
    [0.0, 0.0, 0.0, 0.0, 1.0, -171996.0, -174.2, 92025.0, 8.9],
    [0.0, 0.0, 0.0, 0.0, 2.0, 2062.0, 0.2, -895.0, 0.5],
    [-2.0, 0.0, 2.0, 0.0, 1.0, 46.0, 0.0, -24.0, 0.0],
    [2.0, 0.0, -2.0, 0.0, 0.0, 11.0, 0.0, 0.0, 0.0],
    [-2.0, 0.0, 2.0, 0.0, 2.0, -3.0, 0.0, 1.0, 0.0],
    [1.0, -1.0, 0.0, -1.0, 0.0, -3.0, 0.0, 0.0, 0.0],
    [0.0, -2.0, 2.0, -2.0, 1.0, -2.0, 0.0, 1.0, 0.0],
    [2.0, 0.0, -2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 2.0, -2.0, 2.0, -13187.0, -1.6, 5736.0, -3.1],
    [0.0, 1.0, 0.0, 0.0, 0.0, 1426.0, -3.4, 54.0, -0.1],
    [0.0, 1.0, 2.0, -2.0, 2.0, -517.0, 1.2, 224.0, -0.6],
    [0.0, -1.0, 2.0, -2.0, 2.0, 217.0, -0.5, -95.0, 0.3],
    [0.0, 0.0, 2.0, -2.0, 1.0, 129.0, 0.1, -70.0, 0.0],
    [2.0, 0.0, 0.0, -2.0, 0.0, 48.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 2.0, -2.0, 0.0, -22.0, 0.0, 0.0, 0.0],
    [0.0, 2.0, 0.0, 0.0, 0.0, 17.0, -0.1, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0, 1.0, -15.0, 0.0, 9.0, 0.0],
    [0.0, 2.0, 2.0, -2.0, 2.0, -16.0, 0.1, 7.0, 0.0],
    [0.0, -1.0, 0.0, 0.0, 1.0, -12.0, 0.0, 6.0, 0.0],
    [-2.0, 0.0, 0.0, 2.0, 1.0, -6.0, 0.0, 3.0, 0.0],
    [0.0, -1.0, 2.0, -2.0, 1.0, -5.0, 0.0, 3.0, 0.0],
    [2.0, 0.0, 0.0, -2.0, 1.0, 4.0, 0.0, -2.0, 0.0],
    [0.0, 1.0, 2.0, -2.0, 1.0, 4.0, 0.0, -2.0, 0.0],
    [1.0, 0.0, 0.0, -1.0, 0.0, -4.0, 0.0, 0.0, 0.0],
    [2.0, 1.0, 0.0, -2.0, 0.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, -2.0, 2.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, -2.0, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 2.0, -2.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 2.0, 0.0, 2.0, -2274.0, -0.2, 977.0, -0.5],
    [1.0, 0.0, 0.0, 0.0, 0.0, 712.0, 0.1, -7.0, 0.0],
    [0.0, 0.0, 2.0, 0.0, 1.0, -386.0, -0.4, 200.0, 0.0],
    [1.0, 0.0, 2.0, 0.0, 2.0, -301.0, 0.0, 129.0, -0.1],
    [1.0, 0.0, 0.0, -2.0, 0.0, -158.0, 0.0, -1.0, 0.0],
    [-1.0, 0.0, 2.0, 0.0, 2.0, 123.0, 0.0, -53.0, 0.0],
    [0.0, 0.0, 0.0, 2.0, 0.0, 63.0, 0.0, -2.0, 0.0],
    [1.0, 0.0, 0.0, 0.0, 1.0, 63.0, 0.1, -33.0, 0.0],
    [-1.0, 0.0, 0.0, 0.0, 1.0, -58.0, -0.1, 32.0, 0.0],
    [-1.0, 0.0, 2.0, 2.0, 2.0, -59.0, 0.0, 26.0, 0.0],
    [1.0, 0.0, 2.0, 0.0, 1.0, -51.0, 0.0, 27.0, 0.0],
    [0.0, 0.0, 2.0, 2.0, 2.0, -38.0, 0.0, 16.0, 0.0],
    [2.0, 0.0, 0.0, 0.0, 0.0, 29.0, 0.0, -1.0, 0.0],
    [1.0, 0.0, 2.0, -2.0, 2.0, 29.0, 0.0, -12.0, 0.0],
    [2.0, 0.0, 2.0, 0.0, 2.0, -31.0, 0.0, 13.0, 0.0],
    [0.0, 0.0, 2.0, 0.0, 0.0, 26.0, 0.0, -1.0, 0.0],
    [-1.0, 0.0, 2.0, 0.0, 1.0, 21.0, 0.0, -10.0, 0.0],
    [-1.0, 0.0, 0.0, 2.0, 1.0, 16.0, 0.0, -8.0, 0.0],
    [1.0, 0.0, 0.0, -2.0, 1.0, -13.0, 0.0, 7.0, 0.0],
    [-1.0, 0.0, 2.0, 2.0, 1.0, -10.0, 0.0, 5.0, 0.0],
    [1.0, 1.0, 0.0, -2.0, 0.0, -7.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 2.0, 0.0, 2.0, 7.0, 0.0, -3.0, 0.0],
    [0.0, -1.0, 2.0, 0.0, 2.0, -7.0, 0.0, 3.0, 0.0],
    [1.0, 0.0, 2.0, 2.0, 2.0, -8.0, 0.0, 3.0, 0.0],
    [1.0, 0.0, 0.0, 2.0, 0.0, 6.0, 0.0, 0.0, 0.0],
    [2.0, 0.0, 2.0, -2.0, 2.0, 6.0, 0.0, -3.0, 0.0],
    [0.0, 0.0, 0.0, 2.0, 1.0, -6.0, 0.0, 3.0, 0.0],
    [0.0, 0.0, 2.0, 2.0, 1.0, -7.0, 0.0, 3.0, 0.0],
    [1.0, 0.0, 2.0, -2.0, 1.0, 6.0, 0.0, -3.0, 0.0],
    [0.0, 0.0, 0.0, -2.0, 1.0, -5.0, 0.0, 3.0, 0.0],
    [1.0, -1.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0],
    [2.0, 0.0, 2.0, 0.0, 1.0, -5.0, 0.0, 3.0, 0.0],
    [0.0, 1.0, 0.0, -2.0, 0.0, -4.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, -2.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0, 0.0, -4.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 0.0, 0.0, 0.0, -3.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0],
    [1.0, -1.0, 2.0, 0.0, 2.0, -3.0, 0.0, 1.0, 0.0],
    [-1.0, -1.0, 2.0, 2.0, 2.0, -3.0, 0.0, 1.0, 0.0],
    [-2.0, 0.0, 0.0, 0.0, 1.0, -2.0, 0.0, 1.0, 0.0],
    [3.0, 0.0, 2.0, 0.0, 2.0, -3.0, 0.0, 1.0, 0.0],
    [0.0, -1.0, 2.0, 2.0, 2.0, -3.0, 0.0, 1.0, 0.0],
    [1.0, 1.0, 2.0, 0.0, 2.0, 2.0, 0.0, -1.0, 0.0],
    [-1.0, 0.0, 2.0, -2.0, 1.0, -2.0, 0.0, 1.0, 0.0],
    [2.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, -1.0, 0.0],
    [1.0, 0.0, 0.0, 0.0, 2.0, -2.0, 0.0, 1.0, 0.0],
    [3.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 2.0, 1.0, 2.0, 2.0, 0.0, -1.0, 0.0],
    [-1.0, 0.0, 0.0, 0.0, 2.0, 1.0, 0.0, -1.0, 0.0],
    [1.0, 0.0, 0.0, -4.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [-2.0, 0.0, 2.0, 2.0, 2.0, 1.0, 0.0, -1.0, 0.0],
    [-1.0, 0.0, 2.0, 4.0, 2.0, -2.0, 0.0, 1.0, 0.0],
    [2.0, 0.0, 0.0, -4.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 2.0, -2.0, 2.0, 1.0, 0.0, -1.0, 0.0],
    [1.0, 0.0, 2.0, 2.0, 1.0, -1.0, 0.0, 1.0, 0.0],
    [-2.0, 0.0, 2.0, 4.0, 2.0, -1.0, 0.0, 1.0, 0.0],
    [-1.0, 0.0, 4.0, 0.0, 2.0, 1.0, 0.0, 0.0, 0.0],
    [1.0, -1.0, 0.0, -2.0, 0.0, 1.0, 0.0, 0.0, 0.0],
    [2.0, 0.0, 2.0, -2.0, 1.0, 1.0, 0.0, -1.0, 0.0],
    [2.0, 0.0, 2.0, 2.0, 2.0, -1.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0, 2.0, 1.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 4.0, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0],
    [3.0, 0.0, 2.0, -2.0, 2.0, 1.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, 2.0, -2.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    [-1.0, -1.0, 0.0, 2.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, -2.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 2.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, -2.0, -2.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, -1.0, 2.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 0.0, -2.0, 1.0, -1.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, -2.0, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [2.0, 0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 2.0, 4.0, 2.0, -1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0],
];

fn anpm(a: f64) -> f64 {
    let d2pi = 2.0 * std::f64::consts::PI;
    let mut w = a % d2pi;
    if w < 0.0 {
        w += d2pi;
    }
    if w > std::f64::consts::PI {
        w - d2pi
    } else {
        w
    }
}

fn delaunay80(jd_tt: f64) -> [f64; 5] {
    let t = (jd_tt - JD_J2000) / 36525.0;
    let d2r = ARCSEC_TO_RAD;
    let d2pi = 2.0 * std::f64::consts::PI;
    let l = anpm(
        (485866.733 + (715922.633 + (31.310 + 0.064 * t) * t) * t) * d2r
            + (1325.0 * t).rem_euclid(1.0) * d2pi,
    );
    let lp = anpm(
        (1287099.804 + (1292581.224 + (-0.577 - 0.012 * t) * t) * t) * d2r
            + (99.0 * t).rem_euclid(1.0) * d2pi,
    );
    let f = anpm(
        (335778.877 + (295263.137 + (-13.257 + 0.011 * t) * t) * t) * d2r
            + (1342.0 * t).rem_euclid(1.0) * d2pi,
    );
    let d = anpm(
        (1072261.307 + (1105601.328 + (-6.891 + 0.019 * t) * t) * t) * d2r
            + (1236.0 * t).rem_euclid(1.0) * d2pi,
    );
    let om = anpm(
        (450160.280 + (-482890.539 + (7.455 + 0.008 * t) * t) * t) * d2r
            + (-5.0 * t).rem_euclid(1.0) * d2pi,
    );
    [d, lp, l, f, om]
}

pub fn nutation_iau80(jd_tt: f64) -> (f64, f64) {
    let t = (jd_tt - JD_J2000) / 36525.0;
    let [d, lp, l, f, om] = delaunay80(jd_tt);
    let mut dpsi = 0.0;
    let mut deps = 0.0;
    for x in NUT80.iter().rev() {
        let (nl, nlp, nf, nd, nom) = (x[0], x[1], x[2], x[3], x[4]);
        let arg = nl * l + nlp * lp + nf * f + nd * d + nom * om;
        let s = x[5] + x[6] * t;
        let c = x[7] + x[8] * t;
        if s != 0.0 {
            dpsi += s * arg.sin();
        }
        if c != 0.0 {
            deps += c * arg.cos();
        }
    }
    (dpsi * 1e-4 * ARCSEC_TO_RAD, deps * 1e-4 * ARCSEC_TO_RAD)
}

pub fn mean_obliquity_iau80(jd_tt: f64) -> f64 {
    let t = (jd_tt - JD_J2000) / 36525.0;
    (84381.448 + (-46.8150 + (-0.00059 + 0.001813 * t) * t) * t) * ARCSEC_TO_RAD
}

fn precession_angles_iau76(jd_start: f64, jd_end: f64) -> (f64, f64, f64) {
    let t0 = (jd_start - JD_J2000) / 36525.0;
    let t = (jd_end - jd_start) / 36525.0;
    let w = 2306.2181 + (1.39656 - 0.000139 * t0) * t0;
    let zeta = (w + ((0.30188 - 0.000344 * t0) + 0.017998 * t) * t) * t;
    let z = (w + ((1.09468 + 0.000066 * t0) + 0.018203 * t) * t) * t;
    let theta = ((2004.3109 + (-0.85330 - 0.000217 * t0) * t0)
        + ((-0.42665 - 0.000217 * t0) - 0.041833 * t) * t)
        * t;
    (zeta, z, theta)
}

fn precession_matrix_angles(zeta_as: f64, z_as: f64, theta_as: f64) -> [f64; 9] {
    let zeta = zeta_as * ARCSEC_TO_RAD;
    let z = z_as * ARCSEC_TO_RAD;
    let theta = theta_as * ARCSEC_TO_RAD;
    mat3_mul(&rot_z(z), &mat3_mul(&rot_y(-theta), &rot_z(zeta)))
}

pub fn precession_iau76(jd_start: f64, jd_end: f64) -> [f64; 9] {
    let (zeta, z, theta) = precession_angles_iau76(jd_start, jd_end);
    precession_matrix_angles(zeta, z, theta)
}

fn newcomb_angles_as(t0: f64, t: f64) -> (f64, f64, f64) {
    let zeta = (2304.250 + 1.396 * t0) * t + 0.302 * t * t + 0.018 * t * t * t;
    let z = (2304.250 + 1.396 * t0) * t + 1.093 * t * t + 0.018 * t * t * t;
    let theta = (2004.682 - 0.853 * t0) * t - 0.426 * t * t - 0.042 * t * t * t;
    (zeta, z, theta)
}

pub fn precession_newcomb_to_b1950(jd: f64) -> [f64; 9] {
    let bessel = 1900.0 + (jd - BESSEL_1900) / TROPICAL_YEAR;
    let t0 = (bessel - 1900.0) / 100.0;
    let t = (1950.0 - bessel) / 100.0;
    let (zeta, z, theta) = newcomb_angles_as(t0, t);
    precession_matrix_angles(zeta, z, theta)
}

pub fn fk4_b1950_to_fk5_j2000() -> [f64; 9] {
    [
        0.9999256782,
        -0.0111820611,
        -0.0048579477,
        0.0111820610,
        0.9999374784,
        -0.0000271765,
        0.0048579479,
        -0.0000271474,
        0.9999881997,
    ]
}

pub fn frame_bias_fk5_to_icrs() -> [f64; 9] {
    let xi0 = -0.016617 * ARCSEC_TO_RAD;
    let eta0 = -0.0068192 * ARCSEC_TO_RAD;
    let da0 = -0.0146 * ARCSEC_TO_RAD;
    mat3_mul(&rot_x(-eta0), &mat3_mul(&rot_y(xi0), &rot_z(da0)))
}

pub fn nutation_matrix(jd_tt: f64, forward: bool) -> [f64; 9] {
    let (dpsi, deps) = nutation_iau80(jd_tt);
    let eps = mean_obliquity_iau80(jd_tt);
    let m = mat3_mul(&rot_x(eps + deps), &mat3_mul(&rot_z(dpsi), &rot_x(-eps)));
    if forward {
        m
    } else {
        mat3_transpose(&m)
    }
}

pub fn aberration_apply(u: [f64; 3], v: [f64; 3], forward: bool) -> Option<[f64; 3]> {
    let beta = [v[0] / C_LIGHT, v[1] / C_LIGHT, v[2] / C_LIGHT];
    let num = if forward {
        [u[0] + beta[0], u[1] + beta[1], u[2] + beta[2]]
    } else {
        [u[0] - beta[0], u[1] - beta[1], u[2] - beta[2]]
    };
    vec3_norm_u(num)
}

pub fn parallax_geo_to_topo(u_geo: [f64; 3], obs_minus_geo: [f64; 3], d: f64) -> Option<[f64; 3]> {
    let v = [
        d * u_geo[0] - obs_minus_geo[0],
        d * u_geo[1] - obs_minus_geo[1],
        d * u_geo[2] - obs_minus_geo[2],
    ];
    vec3_norm_u(v)
}

pub fn parallax_topo_to_geo(u_topo: [f64; 3], obs_minus_geo: [f64; 3], d: f64) -> Option<[f64; 3]> {
    let v = [
        d * u_topo[0] + obs_minus_geo[0],
        d * u_topo[1] + obs_minus_geo[1],
        d * u_topo[2] + obs_minus_geo[2],
    ];
    vec3_norm_u(v)
}

pub fn diurnal_velocity_icrs(obs_minus_geo: [f64; 3]) -> [f64; 3] {
    let omega = [0.0, 0.0, EARTH_ROT_RAD_S];
    vec3_cross(omega, obs_minus_geo)
}

pub fn delta_t_espenak_meeus(year: f64) -> f64 {
    let y = year;
    if y < -500.0 {
        let u = (y - 1820.0) / 100.0;
        return -20.0 + 32.0 * u * u;
    }
    if y < 500.0 {
        let u = y / 100.0;
        return 10583.6 - 1014.41 * u + 33.78311 * u * u
            - 5.952053 * u * u * u
            - 0.1798452 * u.powi(4)
            + 0.022174192 * u.powi(5)
            + 0.0090316521 * u.powi(6);
    }
    if y < 1600.0 {
        let u = (y - 1000.0) / 100.0;
        return 1574.2 - 556.01 * u + 71.23472 * u * u + 0.319781 * u * u * u
            - 0.8503463 * u.powi(4)
            - 0.005050998 * u.powi(5)
            + 0.0083572073 * u.powi(6);
    }
    if y < 1700.0 {
        let t = y - 1600.0;
        return 120.0 - 0.9808 * t - 0.01532 * t * t + t * t * t / 7129.0;
    }
    if y < 1800.0 {
        let t = y - 1700.0;
        return 8.83 + 0.1603 * t - 0.0059285 * t * t + 0.00013336 * t * t * t
            - t.powi(4) / 1174000.0;
    }
    if y < 1860.0 {
        let t = y - 1800.0;
        return 13.72 - 0.332447 * t + 0.0068612 * t * t + 0.0041116 * t.powi(3)
            - 0.00037436 * t.powi(4)
            + 0.0000121272 * t.powi(5)
            - 0.0000001699 * t.powi(6)
            + 0.000000000875 * t.powi(7);
    }
    if y < 1900.0 {
        let t = y - 1860.0;
        return 7.62 + 0.5737 * t - 0.251754 * t * t + 0.01680668 * t.powi(3)
            - 0.0004473624 * t.powi(4)
            + t.powi(5) / 233174.0;
    }
    if y < 1920.0 {
        let t = y - 1900.0;
        return -2.79 + 1.494119 * t - 0.0598939 * t * t + 0.0061966 * t.powi(3)
            - 0.000197 * t.powi(4);
    }
    if y < 1941.0 {
        let t = y - 1920.0;
        return 21.20 + 0.84493 * t - 0.076100 * t * t + 0.0020936 * t.powi(3);
    }
    if y < 1961.0 {
        let t = y - 1950.0;
        return 29.07 + 0.407 * t - t * t / 233.0 + t.powi(3) / 2547.0;
    }
    if y < 1986.0 {
        let t = y - 1975.0;
        return 45.45 + 1.067 * t - t * t / 260.0 - t.powi(3) / 718.0;
    }
    if y < 2005.0 {
        let t = y - 2000.0;
        return 63.86 + 0.3345 * t - 0.060374 * t * t
            + 0.0017275 * t.powi(3)
            + 0.000651814 * t.powi(4)
            + 0.00002373599 * t.powi(5);
    }
    if y < 2050.0 {
        let t = y - 2000.0;
        return 62.92 + 0.32217 * t + 0.005589 * t * t;
    }
    if y < 2150.0 {
        let u = (y - 1820.0) / 100.0;
        return -20.0 + 32.0 * u * u - 0.5628 * (2150.0 - y);
    }
    let u = (y - 1820.0) / 100.0;
    -20.0 + 32.0 * u * u
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn nutation_iau80_reproduces_the_sofa_2006_vector() {
        let (dpsi, deps) = nutation_iau80(2453736.5);
        assert!(near(dpsi, -0.9643658353226563966e-5, 1e-15));
        assert!(near(deps, 0.4060051006879713322e-4, 1e-15));
    }

    #[test]
    fn nutation_iau80_reproduces_the_1987_explanatory_supplement() {
        let (dpsi, deps) = nutation_iau80(2446895.5);
        assert!(near(dpsi / ARCSEC_TO_RAD, -3.788, 0.001));
        assert!(near(deps / ARCSEC_TO_RAD, 9.443, 0.001));
    }

    #[test]
    fn obliquity_iau80_carries_84381_448_at_j2000() {
        assert!(near(
            mean_obliquity_iau80(JD_J2000) / ARCSEC_TO_RAD,
            84381.448,
            1e-9
        ));
    }

    #[test]
    fn precession_iau76_matches_lieske_half_century() {
        let (zeta, z, theta) = precession_angles_iau76(2451545.0, 2469807.5);
        assert!(near(zeta, 1153.1868, 1e-3));
        assert!(near(z, 1153.3850, 1e-3));
        assert!(near(theta, 1002.0436, 1e-3));
    }

    #[test]
    fn precession_newcomb_matches_b1900_to_b1950() {
        let (zeta, z, theta) = newcomb_angles_as(0.0, 0.5);
        assert!(near(zeta, 1152.203, 1e-2));
        assert!(near(z, 1152.401, 1e-2));
        assert!(near(theta, 1002.229, 1e-2));
    }

    #[test]
    fn aoki_matrix_is_an_orthonormal_rotation() {
        let m = fk4_b1950_to_fk5_j2000();
        let mt = mat3_transpose(&m);
        let id = mat3_mul(&m, &mt);
        for i in 0..3 {
            for j in 0..3 {
                let expect = if i == j { 1.0 } else { 0.0 };
                assert!(near(id[i * 3 + j], expect, 1e-7));
            }
        }
    }

    #[test]
    fn frame_bias_maps_pole_and_equinox() {
        let m = frame_bias_fk5_to_icrs();
        let pole = mat3_vec(&m, [0.0, 0.0, 1.0]);
        assert!(near(pole[0] / ARCSEC_TO_RAD, -0.016617, 1e-4));
        assert!(near(pole[1] / ARCSEC_TO_RAD, -0.0068192, 1e-4));
        let equinox = mat3_vec(&m, [1.0, 0.0, 0.0]);
        assert!(near(
            equinox[1].atan2(equinox[0]) / ARCSEC_TO_RAD,
            -0.0146,
            1e-4
        ));
    }

    #[test]
    fn aberration_removes_the_constant_of_aberration() {
        let u = [1.0_f64, 0.0, 0.0];
        let v = [0.0, 29_790.0, 0.0];
        let shifted = aberration_apply(u, v, true).unwrap();
        let angle = shifted[1].atan2(shifted[0]);
        assert!(near(angle / ARCSEC_TO_RAD, 20.49552, 0.01));
    }

    #[test]
    fn aberration_roundtrips() {
        let u = vec3_norm_u([0.4_f64, 0.7, 0.6]).unwrap();
        let v = [-29_785.0, 10_000.0, 3_000.0];
        let app = aberration_apply(u, v, true).unwrap();
        let back = aberration_apply(app, v, false).unwrap();
        for k in 0..3 {
            assert!(near(back[k], u[k], 1e-9));
        }
    }

    #[test]
    fn parallax_roundtrips() {
        let u_geo = vec3_norm_u([0.1, -0.4, 0.9]).unwrap();
        let off = [4_000_000.0, 2_000_000.0, 1_000_000.0];
        let d = 4.5e12;
        let u_topo = parallax_geo_to_topo(u_geo, off, d).unwrap();
        let back = parallax_topo_to_geo(u_topo, off, d).unwrap();
        for k in 0..3 {
            assert!(near(back[k], u_geo[k], 1e-9));
        }
    }

    #[test]
    fn full_rotation_chain_roundtrips() {
        let u_icrs = vec3_norm_u([0.5, 0.3, -0.8]).unwrap();
        let jd = 2400000.5 + 20000.0;
        let aoki = fk4_b1950_to_fk5_j2000();
        let bias = frame_bias_fk5_to_icrs();
        let prec = precession_newcomb_to_b1950(jd);
        let nut_fwd = nutation_matrix(jd, true);
        let nut_inv = nutation_matrix(jd, false);

        let u_j2000 = mat3_vec(&mat3_transpose(&bias), u_icrs);
        let u_b1950 = mat3_vec(&mat3_transpose(&aoki), u_j2000);
        let u_mean = mat3_vec(&mat3_transpose(&prec), u_b1950);
        let u_true = mat3_vec(&nut_fwd, u_mean);

        let back_mean = mat3_vec(&nut_inv, u_true);
        let back_b1950 = mat3_vec(&prec, back_mean);
        let back_j2000 = mat3_vec(&aoki, back_b1950);
        let back_icrs = mat3_vec(&bias, back_j2000);
        for k in 0..3 {
            assert!(near(back_icrs[k], u_icrs[k], 1e-6));
        }
    }

    #[test]
    fn delta_t_covers_historic_observing_epochs() {
        assert!(near(delta_t_espenak_meeus(1866.0), 5.1, 1.5));
        assert!(near(delta_t_espenak_meeus(1969.0), 39.2, 3.0));
        assert!(near(delta_t_espenak_meeus(1979.0), 49.6, 3.0));
    }

    #[test]
    fn newcomb_to_b1950_is_the_inverse_direction_of_aoki() {
        let nc = precession_newcomb_to_b1950(JD_J2000);
        let composite = mat3_mul(&fk4_b1950_to_fk5_j2000(), &nc);
        let id = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for k in 0..9 {
            assert!(near(composite[k], id[k], 1e-4));
        }
    }

    #[test]
    fn precession_iau76_direction_matches_the_aoki_matrix() {
        let p76 = precession_iau76(JD_B1950, JD_J2000);
        let aoki = fk4_b1950_to_fk5_j2000();
        for k in 0..9 {
            assert!(near(p76[k], aoki[k], 1e-3));
        }
    }
}
