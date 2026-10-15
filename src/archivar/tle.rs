use super::astrometry::{
    JD_J2000, frame_bias_fk5_to_icrs, mat3_mul, mat3_transpose, mat3_vec, mean_obliquity_iau80,
    nutation_iau80, nutation_matrix, precession_iau76,
};
use std::f64::consts::PI;

pub const RADIUS_EARTH_KM: f64 = 6378.135;
pub const XKE: f64 = 0.07436691613317342;
pub const J2: f64 = 0.001082616;
pub const J3: f64 = -0.00000253881;
pub const J4: f64 = -0.00000165597;
pub const J3OJ2: f64 = J3 / J2;

const X2O3: f64 = 2.0 / 3.0;
const TWOPI: f64 = 2.0 * PI;
const TEMP4: f64 = 1.5e-12;

#[derive(Clone, Debug)]
pub struct Tle {
    pub satnum: String,
    pub epoch_jd: f64,
    pub ndot: f64,
    pub nddot: f64,
    pub bstar: f64,
    pub inclo: f64,
    pub nodeo: f64,
    pub ecco: f64,
    pub argpo: f64,
    pub mo: f64,
    pub no_kozai: f64,
}

fn slice(s: &[u8], a: usize, b: usize) -> Option<&str> {
    std::str::from_utf8(s.get(a..b)?).ok()
}

fn num(t: &str) -> Option<f64> {
    let t = t.trim();
    let (sign, rest) = if let Some(r) = t.strip_prefix("-") {
        ("-", r)
    } else if let Some(r) = t.strip_prefix("+") {
        ("", r)
    } else {
        ("", t)
    };
    let body = if rest.starts_with(".") {
        format!("0{rest}")
    } else {
        rest.to_string()
    };
    format!("{sign}{body}").parse().ok()
}

fn implied(mant: &str, exp: &str) -> Option<f64> {
    let sign = if mant.starts_with("-") { -1.0 } else { 1.0 };
    let m: f64 = format!("0.{}", &mant[1..]).parse().ok()?;
    let e: f64 = exp.trim().parse().ok()?;
    Some(sign * m * 10f64.powf(e))
}

pub fn parse(line1: &str, line2: &str) -> Option<Tle> {
    let s1 = line1.as_bytes();
    let s2 = line2.as_bytes();
    if s1.len() < 69 || s2.len() < 69 {
        return None;
    }
    if !line1.starts_with("1 ") || !line2.starts_with("2 ") {
        return None;
    }
    let satnum = slice(s1, 2, 7)?.trim().to_string();
    if satnum != slice(s2, 2, 7)?.trim() {
        return None;
    }
    let two = slice(s1, 18, 20)?.trim().parse::<i32>().ok()?;
    let year = if two < 57 { two + 2000 } else { two + 1900 };
    let epoch_days = slice(s1, 20, 32)?.trim().parse::<f64>().ok()?;
    let ndot = num(slice(s1, 33, 43)?)?;
    let nddot = implied(slice(s1, 44, 50)?, slice(s1, 50, 52)?)?;
    let bstar = implied(slice(s1, 53, 59)?, slice(s1, 59, 61)?)?;
    let inclo = slice(s2, 8, 16)?.trim().parse::<f64>().ok()?.to_radians();
    let nodeo = slice(s2, 17, 25)?.trim().parse::<f64>().ok()?.to_radians();
    let ecc_str = format!("0.{}", slice(s2, 26, 33)?.trim());
    let ecco = ecc_str.parse::<f64>().ok()?;
    let argpo = slice(s2, 34, 42)?.trim().parse::<f64>().ok()?.to_radians();
    let mo = slice(s2, 43, 51)?.trim().parse::<f64>().ok()?.to_radians();
    let no_revday = slice(s2, 52, 63)?.trim().parse::<f64>().ok()?;
    let no_kozai = no_revday / (1440.0 / TWOPI);
    let days = super::lsk::days_from_civil(i64::from(year), 1, 1)? as f64;
    let epoch_jd = days + 2440587.5 + (epoch_days - 1.0);
    let measured = (0.0..1.0).contains(&ecco) && no_kozai > 0.0 && epoch_days > 0.0;
    if !measured {
        return None;
    }
    Some(Tle {
        satnum,
        epoch_jd,
        ndot,
        nddot,
        bstar,
        inclo,
        nodeo,
        ecco,
        argpo,
        mo,
        no_kozai,
    })
}

pub struct Satrec {
    pub bstar: f64,
    pub ecco: f64,
    pub argpo: f64,
    pub inclo: f64,
    pub mo: f64,
    pub no_unkozai: f64,
    pub nodeo: f64,
    pub isimp: bool,
    cc1: f64,
    cc4: f64,
    cc5: f64,
    d2: f64,
    d3: f64,
    d4: f64,
    delmo: f64,
    eta: f64,
    argpdot: f64,
    omgcof: f64,
    sinmao: f64,
    t2cof: f64,
    t3cof: f64,
    t4cof: f64,
    t5cof: f64,
    x1mth2: f64,
    x7thm1: f64,
    mdot: f64,
    nodedot: f64,
    xlcof: f64,
    xmcof: f64,
    nodecf: f64,
    con41: f64,
    aycof: f64,
}

pub fn sgp4_init(tle: &Tle) -> Option<Satrec> {
    let ecco = tle.ecco;
    let inclo = tle.inclo;
    let no_kozai = tle.no_kozai;
    if no_kozai.is_nan() || no_kozai <= 0.0 {
        return None;
    }
    let omeosq = 1.0 - ecco * ecco;
    if omeosq < 0.0 {
        return None;
    }
    let rteosq = omeosq.sqrt();
    let cosio = inclo.cos();
    let cosio2 = cosio * cosio;
    let sinio = inclo.sin();

    let ak = (XKE / no_kozai).powf(X2O3);
    let d1 = 0.75 * J2 * (3.0 * cosio2 - 1.0) / (rteosq * omeosq);
    let mut del = d1 / (ak * ak);
    let adel = ak * (1.0 - del * del - del * (1.0 / 3.0 + 134.0 * del * del / 81.0));
    del = d1 / (adel * adel);
    let no_unkozai = no_kozai / (1.0 + del);
    if TWOPI / no_unkozai >= 225.0 {
        return None;
    }
    let ao = (XKE / no_unkozai).powf(X2O3);
    let po = ao * omeosq;
    let con42 = 1.0 - 5.0 * cosio2;
    let con41 = -con42 - cosio2 - cosio2;
    let posq = po * po;
    let rp = ao * (1.0 - ecco);

    let ss = 78.0 / RADIUS_EARTH_KM + 1.0;
    let qzms2t = {
        let q = (120.0 - 78.0) / RADIUS_EARTH_KM;
        q * q * q * q
    };

    let isimp = rp < 220.0 / RADIUS_EARTH_KM + 1.0;
    let mut sfour = ss;
    let mut qzms24 = qzms2t;
    let perige = (rp - 1.0) * RADIUS_EARTH_KM;
    if perige < 156.0 {
        sfour = perige - 78.0;
        if perige < 98.0 {
            sfour = 20.0;
        }
        let q = (120.0 - sfour) / RADIUS_EARTH_KM;
        qzms24 = q * q * q * q;
        sfour = sfour / RADIUS_EARTH_KM + 1.0;
    }

    let pinvsq = 1.0 / posq;
    let tsi = 1.0 / (ao - sfour);
    let eta = ao * ecco * tsi;
    let etasq = eta * eta;
    let eeta = ecco * eta;
    let psisq = (1.0 - etasq).abs();
    let coef = qzms24 * tsi.powi(4);
    let coef1 = coef / psisq.powf(3.5);
    let cc2 = coef1
        * no_unkozai
        * (ao * (1.0 + 1.5 * etasq + eeta * (4.0 + etasq))
            + 0.375 * J2 * tsi / psisq * con41 * (8.0 + 3.0 * etasq * (8.0 + etasq)));
    let cc1 = tle.bstar * cc2;
    let cc3 = if ecco > 1.0e-4 {
        -2.0 * coef * tsi * J3OJ2 * no_unkozai * sinio / ecco
    } else {
        0.0
    };
    let x1mth2 = 1.0 - cosio2;
    let cc4 = 2.0
        * no_unkozai
        * coef1
        * ao
        * omeosq
        * (eta * (2.0 + 0.5 * etasq) + ecco * (0.5 + 2.0 * etasq)
            - J2 * tsi / (ao * psisq)
                * (-3.0 * con41 * (1.0 - 2.0 * eeta + etasq * (1.5 - 0.5 * eeta))
                    + 0.75
                        * x1mth2
                        * (2.0 * etasq - eeta * (1.0 + etasq))
                        * (2.0 * tle.argpo).cos()));
    let cc5 = 2.0 * coef1 * ao * omeosq * (1.0 + 2.75 * (etasq + eeta) + eeta * etasq);
    let cosio4 = cosio2 * cosio2;
    let temp1 = 1.5 * J2 * pinvsq * no_unkozai;
    let temp2 = 0.5 * temp1 * J2 * pinvsq;
    let temp3 = -0.46875 * J4 * pinvsq * pinvsq * no_unkozai;
    let mdot = no_unkozai
        + 0.5 * temp1 * rteosq * con41
        + 0.0625 * temp2 * rteosq * (13.0 - 78.0 * cosio2 + 137.0 * cosio4);
    let argpdot = -0.5 * temp1 * con42
        + 0.0625 * temp2 * (7.0 - 114.0 * cosio2 + 395.0 * cosio4)
        + temp3 * (3.0 - 36.0 * cosio2 + 49.0 * cosio4);
    let xhdot1 = -temp1 * cosio;
    let nodedot =
        xhdot1 + (0.5 * temp2 * (4.0 - 19.0 * cosio2) + 2.0 * temp3 * (3.0 - 7.0 * cosio2)) * cosio;
    let omgcof = tle.bstar * cc3 * tle.argpo.cos();
    let xmcof = if ecco > 1.0e-4 {
        -X2O3 * coef * tle.bstar / eeta
    } else {
        0.0
    };
    let nodecf = 3.5 * omeosq * xhdot1 * cc1;
    let t2cof = 1.5 * cc1;
    let xlcof = if (cosio + 1.0).abs() > TEMP4 {
        -0.25 * J3OJ2 * sinio * (3.0 + 5.0 * cosio) / (1.0 + cosio)
    } else {
        -0.25 * J3OJ2 * sinio * (3.0 + 5.0 * cosio) / TEMP4
    };
    let aycof = -0.5 * J3OJ2 * sinio;
    let delmo = {
        let d = 1.0 + eta * tle.mo.cos();
        d * d * d
    };
    let sinmao = tle.mo.sin();
    let x7thm1 = 7.0 * cosio2 - 1.0;

    let (d2, d3, d4, t3cof, t4cof, t5cof) = if isimp {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    } else {
        let cc1sq = cc1 * cc1;
        let d2 = 4.0 * ao * tsi * cc1sq;
        let temp = d2 * tsi * cc1 / 3.0;
        let d3 = (17.0 * ao + sfour) * temp;
        let d4 = 0.5 * temp * ao * tsi * (221.0 * ao + 31.0 * sfour) * cc1;
        let t3cof = d2 + 2.0 * cc1sq;
        let t4cof = 0.25 * (3.0 * d3 + cc1 * (12.0 * d2 + 10.0 * cc1sq));
        let t5cof =
            0.2 * (3.0 * d4 + 12.0 * cc1 * d3 + 6.0 * d2 * d2 + 15.0 * cc1sq * (2.0 * d2 + cc1sq));
        (d2, d3, d4, t3cof, t4cof, t5cof)
    };

    Some(Satrec {
        bstar: tle.bstar,
        ecco,
        argpo: tle.argpo,
        inclo,
        mo: tle.mo,
        no_unkozai,
        nodeo: tle.nodeo,
        isimp,
        cc1,
        cc4,
        cc5,
        d2,
        d3,
        d4,
        delmo,
        eta,
        argpdot,
        omgcof,
        sinmao,
        t2cof,
        t3cof,
        t4cof,
        t5cof,
        x1mth2,
        x7thm1,
        mdot,
        nodedot,
        xlcof,
        xmcof,
        nodecf,
        con41,
        aycof,
    })
}

pub fn sgp4(sat: &Satrec, tsince: f64) -> Option<([f64; 3], [f64; 3])> {
    let vkmpersec = RADIUS_EARTH_KM * XKE / 60.0;
    let t = tsince;
    let xmdf = sat.mo + sat.mdot * t;
    let argpdf = sat.argpo + sat.argpdot * t;
    let nodedf = sat.nodeo + sat.nodedot * t;
    let mut argpm = argpdf;
    let mut mm = xmdf;
    let t2 = t * t;
    let mut nodem = nodedf + sat.nodecf * t2;
    let mut tempa = 1.0 - sat.cc1 * t;
    let mut tempe = sat.bstar * sat.cc4 * t;
    let mut templ = sat.t2cof * t2;

    if !sat.isimp {
        let delomg = sat.omgcof * t;
        let delmtemp = 1.0 + sat.eta * xmdf.cos();
        let delm = sat.xmcof * (delmtemp * delmtemp * delmtemp - sat.delmo);
        let temp = delomg + delm;
        mm = xmdf + temp;
        argpm = argpdf - temp;
        let t3 = t2 * t;
        let t4 = t3 * t;
        tempa = tempa - sat.d2 * t2 - sat.d3 * t3 - sat.d4 * t4;
        tempe += sat.bstar * sat.cc5 * (mm.sin() - sat.sinmao);
        templ += sat.t3cof * t3 + t4 * (sat.t4cof + t * sat.t5cof);
    }

    let nm0 = sat.no_unkozai;
    if nm0.is_nan() || nm0 <= 0.0 {
        return None;
    }
    let mut em = sat.ecco - tempe;
    let am = (XKE / nm0).powf(X2O3) * tempa * tempa;
    let nm = XKE / am.powf(1.5);
    if em >= 1.0 || em < -0.001 {
        return None;
    }
    if em < 1.0e-6 {
        em = 1.0e-6;
    }
    mm += sat.no_unkozai * templ;
    let mut xlm = mm + argpm + nodem;
    nodem = nodem.rem_euclid(TWOPI);
    argpm = argpm.rem_euclid(TWOPI);
    xlm = xlm.rem_euclid(TWOPI);
    mm = (xlm - argpm - nodem).rem_euclid(TWOPI);

    let xincp = sat.inclo;
    let argpp = argpm;
    let nodep = nodem;
    let mp = mm;
    let sinip = sat.inclo.sin();
    let cosip = sat.inclo.cos();

    let axnl = em * argpp.cos();
    let temp = 1.0 / (am * (1.0 - em * em));
    let aynl = em * argpp.sin() + temp * sat.aycof;
    let xl = mp + argpp + nodep + temp * sat.xlcof * axnl;

    let u = (xl - nodep).rem_euclid(TWOPI);
    let mut eo1 = u;
    let mut tem5: f64 = 9999.9;
    let mut ktr = 1;
    let mut coseo1 = 0.0;
    let mut sineo1 = 0.0;
    while tem5.abs() >= 1.0e-12 && ktr <= 10 {
        sineo1 = eo1.sin();
        coseo1 = eo1.cos();
        tem5 = 1.0 - coseo1 * axnl - sineo1 * aynl;
        tem5 = (u - aynl * coseo1 + axnl * sineo1 - eo1) / tem5;
        if tem5.abs() >= 0.95 {
            tem5 = if tem5 > 0.0 { 0.95 } else { -0.95 };
        }
        eo1 += tem5;
        ktr += 1;
    }

    let ecose = axnl * coseo1 + aynl * sineo1;
    let esine = axnl * sineo1 - aynl * coseo1;
    let el2 = axnl * axnl + aynl * aynl;
    let pl = am * (1.0 - el2);
    if pl < 0.0 {
        return None;
    }
    let rl = am * (1.0 - ecose);
    let rdotl = am.sqrt() * esine / rl;
    let rvdotl = pl.sqrt() / rl;
    let betal = (1.0 - el2).sqrt();
    let temp = esine / (1.0 + betal);
    let sinu = am / rl * (sineo1 - aynl - axnl * temp);
    let cosu = am / rl * (coseo1 - axnl + aynl * temp);
    let mut su = sinu.atan2(cosu);
    let sin2u = (cosu + cosu) * sinu;
    let cos2u = 1.0 - 2.0 * sinu * sinu;
    let temp = 1.0 / pl;
    let temp1 = 0.5 * J2 * temp;
    let temp2 = temp1 * temp;

    let mrt = rl * (1.0 - 1.5 * temp2 * betal * sat.con41) + 0.5 * temp1 * sat.x1mth2 * cos2u;
    su -= 0.25 * temp2 * sat.x7thm1 * sin2u;
    let xnode = nodep + 1.5 * temp2 * cosip * sin2u;
    let xinc = xincp + 1.5 * temp2 * cosip * sinip * cos2u;
    let mvt = rdotl - nm * temp1 * sat.x1mth2 * sin2u / XKE;
    let rvdot = rvdotl + nm * temp1 * (sat.x1mth2 * cos2u + 1.5 * sat.con41) / XKE;

    let sinsu = su.sin();
    let cossu = su.cos();
    let snod = xnode.sin();
    let cnod = xnode.cos();
    let sini = xinc.sin();
    let cosi = xinc.cos();
    let xmx = -snod * cosi;
    let xmy = cnod * cosi;
    let ux = xmx * sinsu + cnod * cossu;
    let uy = xmy * sinsu + snod * cossu;
    let uz = sini * sinsu;
    let vx = xmx * cossu - cnod * sinsu;
    let vy = xmy * cossu - snod * sinsu;
    let vz = sini * cossu;

    if mrt < 1.0 {
        return None;
    }
    let mr = mrt * RADIUS_EARTH_KM;
    let r = [mr * ux, mr * uy, mr * uz];
    let v = [
        (mvt * ux + rvdot * vx) * vkmpersec,
        (mvt * uy + rvdot * vy) * vkmpersec,
        (mvt * uz + rvdot * vz) * vkmpersec,
    ];
    Some((r, v))
}

fn rot_z(v: [f64; 3], a: f64) -> [f64; 3] {
    let (s, c) = a.sin_cos();
    [v[0] * c - v[1] * s, v[0] * s + v[1] * c, v[2]]
}

pub fn gmst_iau82(jd_ut1: f64, jd_tt: f64) -> Option<f64> {
    let era = crate::mathematikerin::receiver::earth_rotation_angle_rad(jd_ut1)?;
    let t = (jd_tt - JD_J2000) / 36525.0;
    let precession_arcsec = 0.014506 + 4612.156534 * t + 1.3915817 * t * t
        - 0.00000044 * t * t * t
        - 0.000029956 * t.powi(4)
        - 0.0000000368 * t.powi(5);
    Some(era + precession_arcsec * PI / 648000.0)
}

fn equation_of_equinoxes(jd_tt: f64) -> f64 {
    let (dpsi, _) = nutation_iau80(jd_tt);
    dpsi * mean_obliquity_iau80(jd_tt).cos()
}

pub fn teme_to_icrs(r: [f64; 3], v: [f64; 3], jd: f64) -> Option<([f64; 3], [f64; 3])> {
    let gmst = gmst_iau82(jd, jd)?;
    let gast = gmst + equation_of_equinoxes(jd);
    let r_tod = rot_z(rot_z(r, -gmst), gast);
    let v_tod = rot_z(rot_z(v, -gmst), gast);
    let p = mat3_transpose(&precession_iau76(JD_J2000, jd));
    let n = mat3_transpose(&nutation_matrix(jd, true));
    let m = mat3_mul(&frame_bias_fk5_to_icrs(), &mat3_mul(&p, &n));
    Some((mat3_vec(&m, r_tod), mat3_vec(&m, v_tod)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const L1_00005: &str = "1 00005U 58002B   00179.78495062  .00000023  00000-0  28098-4 0  4753";
    const L2_00005: &str = "2 00005  34.2682 348.7242 1859667 331.7664  19.3264 10.82419157413667";
    const L1_TIMED: &str = "1 26998U 01055B   26287.22414614  .00001152  00000+0  10753-3 0  9991";
    const L2_TIMED: &str = "2 26998  74.0694 355.2967 0000920 228.8676 131.2429 14.95511591348618";
    const L1_04632: &str = "1 04632U 70093B   04031.91070959 -.00000084  00000-0  10000-3 0  9955";
    const L2_04632: &str = "2 04632  11.4628 273.1101 1450506 207.6000 143.9350  1.20231981 44145";

    fn near(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    fn moment(v: [f64; 3]) -> f64 {
        (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
    }

    #[test]
    fn sgp4_reproduces_the_00005_verification_vectors() {
        let tle = parse(L1_00005, L2_00005).unwrap();
        let sat = sgp4_init(&tle).unwrap();
        let (r0, v0) = sgp4(&sat, 0.0).unwrap();
        let e_r0 = [7022.46529266, -1400.08296755, 0.03995155];
        let e_v0 = [1.893841015, 6.405893759, 4.534807250];
        for k in 0..3 {
            assert!(
                near(r0[k], e_r0[k], 1.0e-5),
                "r0[{k}] = {} want {}",
                r0[k],
                e_r0[k]
            );
            assert!(
                near(v0[k], e_v0[k], 1.0e-7),
                "v0[{k}] = {} want {}",
                v0[k],
                e_v0[k]
            );
        }
        let (r1, v1) = sgp4(&sat, 360.0).unwrap();
        let e_r1 = [-7154.03120202, -3783.17682504, -3536.19412294];
        let e_v1 = [4.741887409, -4.151817765, -2.093935425];
        for k in 0..3 {
            assert!(
                near(r1[k], e_r1[k], 1.0e-5),
                "r1[{k}] = {} want {}",
                r1[k],
                e_r1[k]
            );
            assert!(
                near(v1[k], e_v1[k], 1.0e-7),
                "v1[{k}] = {} want {}",
                v1[k],
                e_v1[k]
            );
        }
    }

    #[test]
    fn timed_tle_carries_a_600_km_orbit() {
        let tle = parse(L1_TIMED, L2_TIMED).unwrap();
        assert!((tle.inclo.to_degrees() - 74.0694).abs() < 1.0e-6);
        let sat = sgp4_init(&tle).unwrap();
        let (r, v) = sgp4(&sat, 0.0).unwrap();
        let alt = moment(r) - RADIUS_EARTH_KM;
        assert!((alt - 600.0).abs() < 80.0, "altitude {alt} km");
        assert!((moment(v) - 7.5).abs() < 0.2, "speed {} km/s", moment(v));
    }

    #[test]
    fn deep_space_element_sets_stay_a_gap() {
        let tle = parse(L1_04632, L2_04632).unwrap();
        assert!(sgp4_init(&tle).is_none());
    }

    #[test]
    fn teme_to_icrs_preserves_the_vector_lengths() {
        let r = [7000.0, -1200.0, 300.0];
        let v = [1.2, 6.3, 4.1];
        let (ri, vi) = teme_to_icrs(r, v, 2461000.5).unwrap();
        assert!(near(moment(r), moment(ri), 1.0e-6));
        assert!(near(moment(v), moment(vi), 1.0e-9));
    }
}
