use crate::mathematikerin::healpix::ang2pix_nest;

pub const MAGIC: [u8; 4] = *b"PAO1";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 28;
pub const NSIDE: i64 = 1024;

#[derive(Clone, Copy)]
pub struct AugerRecord {
    pub order: u8,
    pub ipix: u32,
    pub ra_deg: f32,
    pub dec_deg: f32,
    pub energy_eef: f32,
    pub gpstime: i64,
}

impl AugerRecord {
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

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &AugerRecord) {
    out.fill(0);
    out[0] = r.order;
    out[4..8].copy_from_slice(&r.ipix.to_le_bytes());
    out[8..12].copy_from_slice(&r.ra_deg.to_le_bytes());
    out[12..16].copy_from_slice(&r.dec_deg.to_le_bytes());
    out[16..20].copy_from_slice(&r.energy_eef.to_le_bytes());
    out[20..28].copy_from_slice(&r.gpstime.to_le_bytes());
}

pub fn decode_rec(b: &[u8]) -> Option<AugerRecord> {
    if b.len() != REC_BYTES {
        return None;
    }
    let order = b[0];
    if order > 29 {
        return None;
    }
    let ipix = u32::from_le_bytes(b[4..8].try_into().ok()?);
    let nside = 1i64 << order;
    if (ipix as i64) >= 12 * nside * nside {
        return None;
    }
    let ra_deg = f32::from_le_bytes(b[8..12].try_into().ok()?);
    let dec_deg = f32::from_le_bytes(b[12..16].try_into().ok()?);
    if !(ra_deg.is_finite() && dec_deg.is_finite()) {
        return None;
    }
    Some(AugerRecord {
        order,
        ipix,
        ra_deg,
        dec_deg,
        energy_eef: f32::from_le_bytes(b[16..20].try_into().ok()?),
        gpstime: i64::from_le_bytes(b[20..28].try_into().ok()?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> AugerRecord {
        let (order, ipix) = AugerRecord::pixel_of(163.7, -74.1).unwrap();
        AugerRecord {
            order,
            ipix,
            ra_deg: 163.7,
            dec_deg: -74.1,
            energy_eef: 85.3,
            gpstime: 1066656800,
        }
    }

    #[test]
    fn pixel_of_covers_both_poles_and_refuses_unplausible() {
        assert!(AugerRecord::pixel_of(0.0, 90.0).is_some());
        assert!(AugerRecord::pixel_of(0.0, -90.0).is_some());
        assert!(AugerRecord::pixel_of(163.7, -74.1).is_some());
        assert!(AugerRecord::pixel_of(10.0, 90.5).is_none());
        assert!(AugerRecord::pixel_of(f64::NAN, 0.0).is_none());
    }

    #[test]
    fn header_roundtrip() {
        let mut b = Vec::new();
        write_header(&mut b, 109);
        assert_eq!(b.len(), HEADER_LEN);
        assert_eq!(parse_header(&b), Some(109));
        let mut bad = b.clone();
        bad[4] = 2;
        assert_eq!(parse_header(&bad), None);
    }

    #[test]
    fn record_roundtrip() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        let back = decode_rec(&b).unwrap();
        assert_eq!(back.order, r.order);
        assert_eq!(back.ipix, r.ipix);
        assert_eq!(back.ra_deg, 163.7);
        assert_eq!(back.dec_deg, -74.1);
        assert_eq!(back.energy_eef, 85.3);
        assert_eq!(back.gpstime, 1066656800);
    }

    #[test]
    fn record_refuses_corruption() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert!(decode_rec(&b[..REC_BYTES - 1]).is_none());
        b[0] = 40;
        assert!(decode_rec(&b).is_none());
    }
}
