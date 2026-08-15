use crate::kepler::{elements_to_icrs_state, solve_kepler_ecc, AU_M, GM_SUN_M3_S2};

pub const RECORD_STRIDE: usize = 92;

#[derive(Clone, Debug)]
pub struct AsteroidRec {
    pub number: u32,
    pub epoch_jd: f64,
    pub a_au: f64,
    pub e: f64,
    pub incl_deg: f64,
    pub node_deg: f64,
    pub peri_deg: f64,
    pub ma_deg: f64,
    pub h: f32,
    pub g: f32,
    pub albedo: f32,
    pub rot_period_h: f32,
    pub radius_km: f32,
    pub gm_km3_s2: f32,
    pub sptype: [u8; 5],
}

fn f32_at(buf: &[u8], off: usize) -> Option<f32> {
    Some(f32::from_le_bytes(buf.get(off..off + 4)?.try_into().ok()?))
}

fn f64_at(buf: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(buf.get(off..off + 8)?.try_into().ok()?))
}

pub fn parse_record(buf: &[u8]) -> Option<AsteroidRec> {
    if buf.len() < RECORD_STRIDE {
        return None;
    }
    let mut sptype = [0u8; 5];
    sptype.copy_from_slice(&buf[84..89]);
    Some(AsteroidRec {
        number: u32::from_le_bytes(buf[0..4].try_into().ok()?),
        epoch_jd: f64_at(buf, 4)?,
        a_au: f64_at(buf, 12)?,
        e: f64_at(buf, 20)?,
        incl_deg: f64_at(buf, 28)?,
        node_deg: f64_at(buf, 36)?,
        peri_deg: f64_at(buf, 44)?,
        ma_deg: f64_at(buf, 52)?,
        h: f32_at(buf, 60)?,
        g: f32_at(buf, 64)?,
        albedo: f32_at(buf, 68)?,
        rot_period_h: f32_at(buf, 72)?,
        radius_km: f32_at(buf, 76)?,
        gm_km3_s2: f32_at(buf, 80)?,
        sptype,
    })
}

pub const DB_RECORD_BYTES: usize = 835;

pub const COMET_RECORD_BYTES: usize = 976;

#[derive(Clone, Debug)]
pub struct CometRec {
    pub number: u32,
    pub nobs: u32,
    pub epoch_jd: f64,
    pub ma_deg: f64,
    pub w_deg: f64,
    pub om_deg: f64,
    pub in_deg: f64,
    pub ec: f64,
    pub a_au: f64,
    pub qr_au: f64,
    pub tp_jd: f64,
    pub h: f32,
    pub g: f32,
    pub m1: f32,
    pub rad_km: f32,
    pub albedo: f32,
    pub sbnam: [u8; 12],
    pub desig: [u8; 13],
    pub comnam: [u8; 29],
}

pub fn parse_comet_record(buf: &[u8]) -> Option<CometRec> {
    if buf.len() < COMET_RECORD_BYTES {
        return None;
    }
    let mut sbnam = [0u8; 12];
    sbnam.copy_from_slice(&buf[760..772]);
    let mut desig = [0u8; 13];
    desig.copy_from_slice(&buf[910..923]);
    let mut comnam = [0u8; 29];
    comnam.copy_from_slice(&buf[947..976]);
    Some(CometRec {
        number: i32::from_le_bytes(buf[0..4].try_into().ok()?) as u32,
        nobs: i32::from_le_bytes(buf[4..8].try_into().ok()?) as u32,
        epoch_jd: f64_at(buf, 16)?,
        ma_deg: f64_at(buf, 32)?,
        w_deg: f64_at(buf, 40)?,
        om_deg: f64_at(buf, 48)?,
        in_deg: f64_at(buf, 56)?,
        ec: f64_at(buf, 64)?,
        a_au: f64_at(buf, 72)?,
        qr_au: f64_at(buf, 80)?,
        tp_jd: f64_at(buf, 88)?,
        h: f32_at(buf, 578)?,
        g: f32_at(buf, 582)?,
        m1: f32_at(buf, 586)?,
        rad_km: f32_at(buf, 702)?,
        albedo: f32_at(buf, 722)?,
        sbnam,
        desig,
        comnam,
    })
}

pub fn comet_state_at(rec: &CometRec, t_jd: f64) -> Option<([f64; 3], [f64; 3])> {
    elements_to_icrs_state(
        rec.a_au,
        rec.ec,
        rec.in_deg,
        rec.om_deg,
        rec.w_deg,
        rec.ma_deg,
        rec.epoch_jd,
        t_jd,
    )
}

pub fn parse_db_record(buf: &[u8]) -> Option<AsteroidRec> {
    if buf.len() < DB_RECORD_BYTES {
        return None;
    }
    let mut sptype = [0u8; 5];
    sptype.copy_from_slice(&buf[646..651]);
    Some(AsteroidRec {
        number: i32::from_le_bytes(buf[0..4].try_into().ok()?) as u32,
        epoch_jd: f64_at(buf, 16)?,
        a_au: f64_at(buf, 72)?,
        e: f64_at(buf, 64)?,
        incl_deg: f64_at(buf, 56)?,
        node_deg: f64_at(buf, 48)?,
        peri_deg: f64_at(buf, 40)?,
        ma_deg: f64_at(buf, 32)?,
        h: f32_at(buf, 492)?,
        g: f32_at(buf, 496)?,
        albedo: f32_at(buf, 588)?,
        rot_period_h: f32_at(buf, 560)?,
        radius_km: f32_at(buf, 568)?,
        gm_km3_s2: f32_at(buf, 564)?,
        sptype,
    })
}

pub fn encode_record(rec: &AsteroidRec, out: &mut Vec<u8>) {
    out.extend_from_slice(&rec.number.to_le_bytes());
    out.extend_from_slice(&rec.epoch_jd.to_le_bytes());
    out.extend_from_slice(&rec.a_au.to_le_bytes());
    out.extend_from_slice(&rec.e.to_le_bytes());
    out.extend_from_slice(&rec.incl_deg.to_le_bytes());
    out.extend_from_slice(&rec.node_deg.to_le_bytes());
    out.extend_from_slice(&rec.peri_deg.to_le_bytes());
    out.extend_from_slice(&rec.ma_deg.to_le_bytes());
    out.extend_from_slice(&rec.h.to_le_bytes());
    out.extend_from_slice(&rec.g.to_le_bytes());
    out.extend_from_slice(&rec.albedo.to_le_bytes());
    out.extend_from_slice(&rec.rot_period_h.to_le_bytes());
    out.extend_from_slice(&rec.radius_km.to_le_bytes());
    out.extend_from_slice(&rec.gm_km3_s2.to_le_bytes());
    out.extend_from_slice(&rec.sptype);
    out.extend_from_slice(&[0u8; 3]);
}

pub fn state_at(rec: &AsteroidRec, t_jd: f64) -> Option<([f64; 3], [f64; 3])> {
    elements_to_icrs_state(
        rec.a_au,
        rec.e,
        rec.incl_deg,
        rec.node_deg,
        rec.peri_deg,
        rec.ma_deg,
        rec.epoch_jd,
        t_jd,
    )
}

pub fn speed_at_epoch(rec: &AsteroidRec) -> Option<f64> {
    let ecc = solve_kepler_ecc(rec.ma_deg.to_radians(), rec.e);
    let r_m = rec.a_au * AU_M * (1.0 - rec.e * ecc.cos());
    if r_m <= 0.0 {
        return None;
    }
    Some((GM_SUN_M3_S2 * (2.0 / r_m - 1.0 / (rec.a_au * AU_M))).sqrt())
}

pub fn accel_at_epoch(rec: &AsteroidRec) -> Option<f64> {
    let peri = rec.a_au * AU_M * (1.0 - rec.e);
    if peri <= 0.0 {
        return None;
    }
    Some(GM_SUN_M3_S2 / (peri * peri))
}

pub fn hill_radius_m(rec: &AsteroidRec) -> Option<f64> {
    let gm = rec.gm_km3_s2 as f64 * 1.0e9;
    if !(gm > 0.0) || !(rec.a_au > 0.0) {
        return None;
    }
    Some(rec.a_au * AU_M * (gm / (3.0 * GM_SUN_M3_S2)).cbrt())
}

#[cfg(test)]
mod tests {
    use super::{
        comet_state_at, encode_record, hill_radius_m, parse_comet_record, parse_record, state_at,
        AsteroidRec, CometRec, COMET_RECORD_BYTES,
    };

    fn halley() -> CometRec {
        CometRec {
            number: 900001,
            nobs: 4566,
            epoch_jd: 2461680.5,
            ma_deg: 92.13863,
            w_deg: 111.33249,
            om_deg: 58.42008,
            in_deg: 162.26269,
            ec: 0.96714291,
            a_au: 17.83414,
            qr_au: 0.58597811,
            tp_jd: 2446465.46,
            h: 4.0,
            g: 0.8,
            m1: 5.5,
            rad_km: 5.5,
            albedo: 0.04,
            sbnam: *b"1P/Halley   ",
            desig: *b"1P           ",
            comnam: *b"Halley                       ",
        }
    }

    fn comet_record_bytes(rec: &CometRec) -> Vec<u8> {
        let mut buf = vec![0u8; COMET_RECORD_BYTES];
        buf[0..4].copy_from_slice(&(rec.number as i32).to_le_bytes());
        buf[4..8].copy_from_slice(&(rec.nobs as i32).to_le_bytes());
        buf[16..24].copy_from_slice(&rec.epoch_jd.to_le_bytes());
        buf[32..40].copy_from_slice(&rec.ma_deg.to_le_bytes());
        buf[40..48].copy_from_slice(&rec.w_deg.to_le_bytes());
        buf[48..56].copy_from_slice(&rec.om_deg.to_le_bytes());
        buf[56..64].copy_from_slice(&rec.in_deg.to_le_bytes());
        buf[64..72].copy_from_slice(&rec.ec.to_le_bytes());
        buf[72..80].copy_from_slice(&rec.a_au.to_le_bytes());
        buf[80..88].copy_from_slice(&rec.qr_au.to_le_bytes());
        buf[88..96].copy_from_slice(&rec.tp_jd.to_le_bytes());
        buf[578..582].copy_from_slice(&rec.h.to_le_bytes());
        buf[582..586].copy_from_slice(&rec.g.to_le_bytes());
        buf[586..590].copy_from_slice(&rec.m1.to_le_bytes());
        buf[702..706].copy_from_slice(&rec.rad_km.to_le_bytes());
        buf[722..726].copy_from_slice(&rec.albedo.to_le_bytes());
        buf[760..772].copy_from_slice(&rec.sbnam);
        buf[910..923].copy_from_slice(&rec.desig);
        buf[947..976].copy_from_slice(&rec.comnam);
        buf
    }

    fn ceres() -> AsteroidRec {
        AsteroidRec {
            number: 1,
            epoch_jd: 2458849.5,
            a_au: 2.7692892921434837,
            e: 0.07687465013145245,
            incl_deg: 10.59127767086216,
            node_deg: 80.3011901917491,
            peri_deg: 73.80896808746482,
            ma_deg: 130.31596882009862,
            h: 3.34,
            g: 0.12,
            albedo: 0.09,
            rot_period_h: 9.07417,
            radius_km: 469.7,
            gm_km3_s2: 62.6284,
            sptype: [b'C', 0, 0, 0, 0],
        }
    }

    #[test]
    fn stride_roundtrip() {
        let mut buf = Vec::new();
        encode_record(&ceres(), &mut buf);
        assert_eq!(buf.len(), super::RECORD_STRIDE);
        let rec = parse_record(&buf).unwrap();
        assert_eq!(rec.number, 1);
        assert!((rec.a_au - 2.7692892921434837).abs() < 1e-15);
        assert!((rec.h - 3.34).abs() < 1e-6);
        assert_eq!(rec.sptype[0], b'C');
    }

    #[test]
    fn ceres_hill_radius_physics() {
        let r = hill_radius_m(&ceres()).unwrap();
        assert!(r > 1.0e8 && r < 4.0e8, "hill radius {} m", r);
    }

    #[test]
    fn ceres_state_at_epoch_matches_known_scale() {
        let (p, v) = state_at(&ceres(), 2458849.5).unwrap();
        let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        assert!((r - 4.362e11).abs() < 1.0e9, "r {} m", r);
        let speed = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        assert!(speed > 1.5e4 && speed < 2.5e4, "speed {} m/s", speed);
    }

    #[test]
    fn comet_record_roundtrip_halley_layout() {
        let rec = halley();
        let buf = comet_record_bytes(&rec);
        let parsed = parse_comet_record(&buf).unwrap();
        assert_eq!(parsed.number, 900001);
        assert!((parsed.epoch_jd - 2461680.5).abs() < 1e-9);
        assert!((parsed.ec - 0.96714291).abs() < 1e-9);
        assert!((parsed.qr_au - 0.58597811).abs() < 1e-9);
        assert!((parsed.h - 4.0).abs() < 1e-6);
        assert!((parsed.m1 - 5.5).abs() < 1e-6);
        assert_eq!(&parsed.sbnam[..9], b"1P/Halley");
        assert_eq!(&parsed.desig[..2], b"1P");
        assert_eq!(&parsed.comnam[..6], b"Halley");
    }

    #[test]
    fn comet_state_at_epoch_matches_two_body_radius() {
        let rec = halley();
        let (p, _) = comet_state_at(&rec, rec.epoch_jd).unwrap();
        let r_au = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt() / crate::kepler::AU_M;
        let ecc = crate::kepler::solve_kepler_ecc(rec.ma_deg.to_radians(), rec.ec);
        let expect = rec.a_au * (1.0 - rec.ec * ecc.cos());
        assert!(
            (r_au - expect).abs() < 1e-9,
            "r {} au, expect {} au",
            r_au,
            expect
        );
    }
}
