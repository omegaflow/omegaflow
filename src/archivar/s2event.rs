use crate::mathematikerin::healpix::ang2pix_nest;

pub const MAGIC: [u8; 4] = *b"S2E1";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 52;
pub const NSIDE: i64 = 1024;

pub const PRES_SIGMA: u8 = 0x01;
pub const PRES_EPOCH: u8 = 0x02;
pub const PRES_ENERGY: u8 = 0x04;
pub const PRES_SIGNALNESS: u8 = 0x08;
pub const PRES_FAR: u8 = 0x10;

pub const ROOT_GENERIC: u8 = 0;
pub const ROOT_NEUTRINO: u8 = 1;
pub const ROOT_CR: u8 = 2;
pub const ROOT_GAMMA: u8 = 3;

#[derive(Clone, Copy)]
pub struct S2EventRecord {
    pub ra_deg: f32,
    pub dec_deg: f32,
    pub sigma_arcsec: Option<f64>,
    pub epoch_tdb: Option<f64>,
    pub energy: Option<f64>,
    pub signalness: Option<f64>,
    pub far: Option<f64>,
    pub particle_root: u8,
}

impl S2EventRecord {
    pub fn pixel_of(ra_deg: f64, dec_deg: f64) -> Option<(u8, u32)> {
        if !(ra_deg.is_finite() && dec_deg.is_finite() && (-90.0..=90.0).contains(&dec_deg)) {
            return None;
        }
        let theta = (90.0 - dec_deg).to_radians();
        let phi = ra_deg.rem_euclid(360.0).to_radians();
        let pix = ang2pix_nest(NSIDE, theta, phi)?;
        let npix = 12 * NSIDE * NSIDE;
        if pix < 0 || pix >= npix {
            return None;
        }
        Some((NSIDE.trailing_zeros() as u8, pix as u32))
    }

    pub fn unit_direction(&self) -> [f64; 3] {
        let ra = (self.ra_deg as f64).to_radians();
        let dec = (self.dec_deg as f64).to_radians();
        let (sa, ca) = ra.sin_cos();
        let (sd, cd) = dec.sin_cos();
        [cd * ca, cd * sa, sd]
    }

    pub fn angular_uncertainty_rad(&self) -> Option<f64> {
        match self.sigma_arcsec {
            Some(s) if s.is_finite() && s > 0.0 => Some(s.to_radians() / 3600.0),
            Some(_) => None,
            None => None,
        }
    }
}

fn present_byte(r: &S2EventRecord) -> u8 {
    let mut p = 0u8;
    if r.sigma_arcsec.is_some() {
        p |= PRES_SIGMA;
    }
    if r.epoch_tdb.is_some() {
        p |= PRES_EPOCH;
    }
    if r.energy.is_some() {
        p |= PRES_ENERGY;
    }
    if r.signalness.is_some() {
        p |= PRES_SIGNALNESS;
    }
    if r.far.is_some() {
        p |= PRES_FAR;
    }
    p
}

pub fn write_header(out: &mut Vec<u8>, n_rows: u64) {
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&n_rows.to_le_bytes());
}

pub fn parse_header(b: &[u8]) -> Option<u64> {
    if b.len() < HEADER_LEN || b[0..4] != MAGIC || b[4] != VERSION {
        return None;
    }
    Some(u64::from_le_bytes(b[5..13].try_into().ok()?))
}

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &S2EventRecord) {
    out.fill(0);
    out[0] = r.particle_root;
    out[1] = present_byte(r);
    out[4..8].copy_from_slice(&r.ra_deg.to_le_bytes());
    out[8..12].copy_from_slice(&r.dec_deg.to_le_bytes());
    if let Some(v) = r.sigma_arcsec {
        out[12..20].copy_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = r.epoch_tdb {
        out[20..28].copy_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = r.energy {
        out[28..36].copy_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = r.signalness {
        out[36..44].copy_from_slice(&v.to_le_bytes());
    }
    if let Some(v) = r.far {
        out[44..52].copy_from_slice(&v.to_le_bytes());
    }
}

pub fn decode_rec(b: &[u8]) -> Option<S2EventRecord> {
    if b.len() != REC_BYTES {
        return None;
    }
    let root = b[0];
    if root > ROOT_GAMMA {
        return None;
    }
    let present = b[1];
    if present & 0xE0 != 0 {
        return None;
    }
    let ra_deg = f32::from_le_bytes(b[4..8].try_into().ok()?);
    let dec_deg = f32::from_le_bytes(b[8..12].try_into().ok()?);
    if !(ra_deg.is_finite() && dec_deg.is_finite()) {
        return None;
    }
    if !(0.0..360.0).contains(&(ra_deg as f64)) || !(-90.0..=90.0).contains(&(dec_deg as f64)) {
        return None;
    }
    let optional = |off: usize, flag: u8, positive: bool| -> Option<Option<f64>> {
        if present & flag == 0 {
            return Some(None);
        }
        let v = f64::from_le_bytes(b[off..off + 8].try_into().ok()?);
        let ok = if positive {
            v.is_finite() && v > 0.0
        } else {
            v.is_finite()
        };
        if !ok {
            return None;
        }
        Some(Some(v))
    };
    Some(S2EventRecord {
        ra_deg,
        dec_deg,
        sigma_arcsec: optional(12, PRES_SIGMA, true)?,
        epoch_tdb: optional(20, PRES_EPOCH, false)?,
        energy: optional(28, PRES_ENERGY, true)?,
        signalness: optional(36, PRES_SIGNALNESS, false)?,
        far: optional(44, PRES_FAR, true)?,
        particle_root: root,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> S2EventRecord {
        S2EventRecord {
            ra_deg: 148.8746,
            dec_deg: 2.5208,
            sigma_arcsec: Some(13574.56),
            epoch_tdb: Some(358608784.84),
            energy: Some(187.0),
            signalness: Some(0.508),
            far: Some(1.3),
            particle_root: ROOT_NEUTRINO,
        }
    }

    fn bare() -> S2EventRecord {
        S2EventRecord {
            ra_deg: 86.31,
            dec_deg: 8.1199,
            sigma_arcsec: None,
            epoch_tdb: None,
            energy: None,
            signalness: None,
            far: None,
            particle_root: ROOT_CR,
        }
    }

    #[test]
    fn pixel_of_covers_poles_and_refuses_unplausible() {
        assert!(S2EventRecord::pixel_of(0.0, 90.0).is_some());
        assert!(S2EventRecord::pixel_of(0.0, -90.0).is_some());
        assert!(S2EventRecord::pixel_of(86.31, 8.1199).is_some());
        assert!(S2EventRecord::pixel_of(10.0, 91.0).is_none());
        assert!(S2EventRecord::pixel_of(f64::NAN, 0.0).is_none());
    }

    #[test]
    fn header_roundtrip() {
        let mut b = Vec::new();
        write_header(&mut b, 348);
        assert_eq!(b.len(), HEADER_LEN);
        assert_eq!(parse_header(&b), Some(348));
        assert_eq!(parse_header(&b[..b.len() - 1]), None);
        let mut bad = b.clone();
        bad[0] = b'X';
        assert_eq!(parse_header(&bad), None);
    }

    #[test]
    fn record_roundtrip_carries_all_present_fields() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert_eq!(b[0], ROOT_NEUTRINO);
        assert_eq!(b[1], 0x1F);
        let back = decode_rec(&b).unwrap();
        assert_eq!(back.ra_deg, r.ra_deg);
        assert_eq!(back.dec_deg, r.dec_deg);
        assert_eq!(back.sigma_arcsec, r.sigma_arcsec);
        assert_eq!(back.epoch_tdb, r.epoch_tdb);
        assert_eq!(back.energy, r.energy);
        assert_eq!(back.signalness, r.signalness);
        assert_eq!(back.far, r.far);
        assert_eq!(back.particle_root, ROOT_NEUTRINO);
    }

    #[test]
    fn absent_fields_stay_absent_and_their_slots_stay_zero_pad() {
        let r = bare();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert_eq!(b[0], ROOT_CR);
        assert_eq!(b[1], 0);
        assert_eq!(&b[12..REC_BYTES], &[0u8; REC_BYTES - 12]);
        let back = decode_rec(&b).unwrap();
        assert_eq!(back.sigma_arcsec, None);
        assert_eq!(back.epoch_tdb, None);
        assert_eq!(back.energy, None);
        assert_eq!(back.signalness, None);
        assert_eq!(back.far, None);
        assert_eq!(back.ra_deg, 86.31);
        assert_eq!(back.dec_deg, 8.1199);
    }

    #[test]
    fn record_refuses_corruption() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert!(decode_rec(&b[..REC_BYTES - 1]).is_none());
        let mut bad_root = b;
        bad_root[0] = 9;
        assert!(decode_rec(&bad_root).is_none());

        let mut b2 = [0u8; REC_BYTES];
        encode_rec(&mut b2, &bare());
        let mut reserved = b2;
        reserved[1] = 0x80;
        assert!(decode_rec(&reserved).is_none());

        let mut b3 = [0u8; REC_BYTES];
        encode_rec(&mut b3, &bare());
        b3[1] = PRES_ENERGY;
        assert!(decode_rec(&b3).is_none());

        let mut b4 = [0u8; REC_BYTES];
        encode_rec(&mut b4, &bare());
        b4[8..12].copy_from_slice(&95.0f32.to_le_bytes());
        assert!(decode_rec(&b4).is_none());

        let mut nan_sigma = bare();
        nan_sigma.sigma_arcsec = Some(f64::NAN);
        let mut b5 = [0u8; REC_BYTES];
        encode_rec(&mut b5, &nan_sigma);
        assert!(decode_rec(&b5).is_none());
    }

    #[test]
    fn unit_direction_is_the_measured_icrs_unit_vector() {
        let n = S2EventRecord {
            ra_deg: 0.0,
            dec_deg: 90.0,
            ..bare()
        };
        let u = n.unit_direction();
        assert!(u[2] > 0.999999);
        let e = S2EventRecord {
            ra_deg: 90.0,
            dec_deg: 0.0,
            ..bare()
        };
        let v = e.unit_direction();
        assert!(v[1] > 0.999999);
        let spiral = S2EventRecord {
            ra_deg: 30.0,
            dec_deg: 60.0,
            ..bare()
        };
        let w = spiral.unit_direction();
        let norm = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
        assert!((norm - 1.0).abs() < 1e-12);
    }

    #[test]
    fn angular_uncertainty_converts_arcsec_to_rad_and_refuses_unplausible() {
        let mut r = bare();
        r.sigma_arcsec = Some(1.0);
        let rad = r.angular_uncertainty_rad().unwrap();
        assert!((rad - std::f64::consts::PI / 648000.0).abs() < 1e-18);
        r.sigma_arcsec = Some(0.0);
        assert_eq!(r.angular_uncertainty_rad(), None);
        r.sigma_arcsec = None;
        assert_eq!(r.angular_uncertainty_rad(), None);
        r.sigma_arcsec = Some(f64::NAN);
        assert_eq!(r.angular_uncertainty_rad(), None);
    }
}
