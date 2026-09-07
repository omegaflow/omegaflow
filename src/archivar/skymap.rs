use crate::mathematikerin::healpix::ang2pix_nest;

pub const MAGIC: [u8; 4] = *b"SKY1";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 20;
pub const NSIDE: i64 = 1024;

pub const KIND_GENERIC: u8 = 0;
pub const KIND_NEUTRINO: u8 = 1;
pub const KIND_CR: u8 = 2;
pub const KIND_GAMMA: u8 = 3;
pub const KIND_GRAVITY: u8 = 4;

#[derive(Clone, Copy)]
pub struct SkymapRecord {
    pub order: u8,
    pub kind: u8,
    pub ipix: u32,
    pub ra_deg: f32,
    pub dec_deg: f32,
    pub value: f32,
}

impl SkymapRecord {
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

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &SkymapRecord) {
    out.fill(0);
    out[0] = r.order;
    out[1] = r.kind;
    out[4..8].copy_from_slice(&r.ipix.to_le_bytes());
    out[8..12].copy_from_slice(&r.ra_deg.to_le_bytes());
    out[12..16].copy_from_slice(&r.dec_deg.to_le_bytes());
    out[16..20].copy_from_slice(&r.value.to_le_bytes());
}

pub fn decode_rec(b: &[u8]) -> Option<SkymapRecord> {
    if b.len() != REC_BYTES {
        return None;
    }
    let order = b[0];
    if order > 29 {
        return None;
    }
    let kind = b[1];
    if kind > KIND_GRAVITY {
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
    let value = f32::from_le_bytes(b[16..20].try_into().ok()?);
    if !value.is_finite() {
        return None;
    }
    Some(SkymapRecord {
        order,
        kind,
        ipix,
        ra_deg,
        dec_deg,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SkymapRecord {
        let (order, ipix) = SkymapRecord::pixel_of(148.8746, 2.5208).unwrap();
        SkymapRecord {
            order,
            kind: KIND_NEUTRINO,
            ipix,
            ra_deg: 148.8746,
            dec_deg: 2.5208,
            value: 191.91,
        }
    }

    #[test]
    fn pixel_of_accepts_valid_directions() {
        assert!(SkymapRecord::pixel_of(0.0, 90.0).is_some());
        assert!(SkymapRecord::pixel_of(148.8746, 2.5208).is_some());
        assert!(SkymapRecord::pixel_of(10.0, -91.0).is_none());
        assert!(SkymapRecord::pixel_of(f64::NAN, 0.0).is_none());
    }

    #[test]
    fn header_roundtrip() {
        let mut b = Vec::new();
        write_header(&mut b, 7);
        assert_eq!(b.len(), HEADER_LEN);
        assert_eq!(parse_header(&b), Some(7));
        let mut bad = b.clone();
        bad[0] = b'X';
        assert_eq!(parse_header(&bad), None);
    }

    #[test]
    fn record_roundtrip() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        let back = decode_rec(&b).unwrap();
        assert_eq!(back.order, r.order);
        assert_eq!(back.kind, KIND_NEUTRINO);
        assert_eq!(back.ipix, r.ipix);
        assert_eq!(back.ra_deg, r.ra_deg);
        assert_eq!(back.dec_deg, r.dec_deg);
        assert_eq!(back.value, r.value);
    }

    #[test]
    fn record_refuses_corruption() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert!(decode_rec(&b[..REC_BYTES - 1]).is_none());
        b[1] = 9;
        assert!(decode_rec(&b).is_none());
    }
}
