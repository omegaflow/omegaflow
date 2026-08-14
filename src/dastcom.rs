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
    use super::{encode_record, hill_radius_m, parse_record, state_at, AsteroidRec};

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
}
