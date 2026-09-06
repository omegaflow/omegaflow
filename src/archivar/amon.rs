use crate::mathematikerin::healpix::ang2pix_nest;

pub const MAGIC: [u8; 4] = *b"AMN1";
pub const VERSION: u8 = 2;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 56;
pub const NSIDE: i64 = 1024;

pub const PRES_ERR90: u8 = 0x01;
pub const PRES_ERR50: u8 = 0x02;
pub const PRES_ENERGY: u8 = 0x04;
pub const PRES_SIGNALNESS: u8 = 0x08;
pub const PRES_FAR: u8 = 0x10;
pub const PRES_REVISION: u8 = 0x20;
pub const PRES_TJD: u8 = 0x40;
pub const PRES_SOD: u8 = 0x80;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoticeClass {
    Gold = 1,
    Bronze = 2,
}

#[derive(Clone, Copy)]
pub struct AmonRecord {
    pub order: u8,
    pub class: NoticeClass,
    pub ipix: u32,
    pub present: u8,
    pub run: u32,
    pub event: u32,
    pub ra_deg: f32,
    pub dec_deg: f32,
    pub err90_arcmin: f32,
    pub err50_arcmin: f32,
    pub energy_tev: f32,
    pub signalness: f32,
    pub far_per_yr: f32,
    pub revision: u32,
    pub tjd: u32,
    pub sod_s: f32,
}

impl AmonRecord {
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

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &AmonRecord) {
    out.fill(0);
    out[0] = r.order;
    out[1] = r.class as u8;
    out[2] = r.present;
    out[4..8].copy_from_slice(&r.ipix.to_le_bytes());
    out[8..12].copy_from_slice(&r.run.to_le_bytes());
    out[12..16].copy_from_slice(&r.event.to_le_bytes());
    out[16..20].copy_from_slice(&r.ra_deg.to_le_bytes());
    out[20..24].copy_from_slice(&r.dec_deg.to_le_bytes());
    out[24..28].copy_from_slice(&r.err90_arcmin.to_le_bytes());
    out[28..32].copy_from_slice(&r.err50_arcmin.to_le_bytes());
    out[32..36].copy_from_slice(&r.energy_tev.to_le_bytes());
    out[36..40].copy_from_slice(&r.signalness.to_le_bytes());
    out[40..44].copy_from_slice(&r.far_per_yr.to_le_bytes());
    out[44..48].copy_from_slice(&r.revision.to_le_bytes());
    out[48..52].copy_from_slice(&r.tjd.to_le_bytes());
    out[52..56].copy_from_slice(&r.sod_s.to_le_bytes());
}

pub fn decode_rec(b: &[u8]) -> Option<AmonRecord> {
    if b.len() != REC_BYTES {
        return None;
    }
    let order = b[0];
    if order > 29 {
        return None;
    }
    let class = match b[1] {
        1 => NoticeClass::Gold,
        2 => NoticeClass::Bronze,
        _ => return None,
    };
    let ipix = u32::from_le_bytes(b[4..8].try_into().ok()?);
    let nside = 1i64 << order;
    if (ipix as i64) >= 12 * nside * nside {
        return None;
    }
    let ra_deg = f32::from_le_bytes(b[16..20].try_into().ok()?);
    let dec_deg = f32::from_le_bytes(b[20..24].try_into().ok()?);
    if !(ra_deg.is_finite() && dec_deg.is_finite()) {
        return None;
    }
    Some(AmonRecord {
        order,
        class,
        ipix,
        present: b[2],
        run: u32::from_le_bytes(b[8..12].try_into().ok()?),
        event: u32::from_le_bytes(b[12..16].try_into().ok()?),
        ra_deg,
        dec_deg,
        err90_arcmin: f32::from_le_bytes(b[24..28].try_into().ok()?),
        err50_arcmin: f32::from_le_bytes(b[28..32].try_into().ok()?),
        energy_tev: f32::from_le_bytes(b[32..36].try_into().ok()?),
        signalness: f32::from_le_bytes(b[36..40].try_into().ok()?),
        far_per_yr: f32::from_le_bytes(b[40..44].try_into().ok()?),
        revision: u32::from_le_bytes(b[44..48].try_into().ok()?),
        tjd: u32::from_le_bytes(b[48..52].try_into().ok()?),
        sod_s: f32::from_le_bytes(b[52..56].try_into().ok()?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> AmonRecord {
        let (order, ipix) = AmonRecord::pixel_of(86.31, 8.1199).unwrap();
        AmonRecord {
            order,
            class: NoticeClass::Gold,
            ipix,
            present: 0xFF,
            run: 143048,
            event: 12474624,
            ra_deg: 86.31,
            dec_deg: 8.1199,
            err90_arcmin: 28.73,
            err50_arcmin: 14.81,
            energy_tev: 191.91,
            signalness: 0.51535,
            far_per_yr: 1.134,
            revision: 0,
            tjd: 21279,
            sod_s: 56136.0,
        }
    }

    #[test]
    fn pixel_of_north_pole_and_equator_are_valid() {
        assert!(AmonRecord::pixel_of(0.0, 90.0).is_some());
        assert!(AmonRecord::pixel_of(0.0, -90.0).is_some());
        assert!(AmonRecord::pixel_of(0.0, 0.0).is_some());
        assert!(AmonRecord::pixel_of(360.0, 0.0).is_some());
        assert!(AmonRecord::pixel_of(86.31, 8.1199).is_some());
    }

    #[test]
    fn pixel_of_refuses_unplausible_dec() {
        assert!(AmonRecord::pixel_of(10.0, 91.0).is_none());
        assert!(AmonRecord::pixel_of(10.0, -91.0).is_none());
        assert!(AmonRecord::pixel_of(f64::NAN, 0.0).is_none());
        assert!(AmonRecord::pixel_of(10.0, f64::INFINITY).is_none());
    }

    #[test]
    fn header_roundtrip() {
        let mut b = Vec::new();
        write_header(&mut b, 7);
        assert_eq!(b.len(), HEADER_LEN);
        assert_eq!(parse_header(&b), Some(7));
        assert_eq!(parse_header(&b[..b.len() - 1]), None);
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
        assert_eq!(back.ipix, r.ipix);
        assert_eq!(back.present, r.present);
        assert_eq!(back.class, NoticeClass::Gold);
        assert_eq!(back.run, 143048);
        assert_eq!(back.event, 12474624);
        assert_eq!(back.ra_deg, 86.31);
        assert_eq!(back.dec_deg, 8.1199);
        assert_eq!(back.err90_arcmin, 28.73);
        assert_eq!(back.err50_arcmin, 14.81);
        assert_eq!(back.energy_tev, 191.91);
        assert_eq!(back.signalness, 0.51535);
        assert_eq!(back.far_per_yr, 1.134);
        assert_eq!(back.revision, 0);
        assert_eq!(back.tjd, 21279);
        assert_eq!(back.sod_s, 56136.0);
    }

    #[test]
    fn bronze_class_roundtrips_and_gold_is_not_zero() {
        let mut r = sample();
        r.class = NoticeClass::Bronze;
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert_eq!(b[1], 2);
        assert_eq!(decode_rec(&b).unwrap().class, NoticeClass::Bronze);
        let zero = [0u8; REC_BYTES];
        assert!(decode_rec(&zero).is_none());
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

    #[test]
    fn pixel_of_is_the_same_pixel_for_a_nearby_direction() {
        let (o1, p1) = AmonRecord::pixel_of(86.31, 8.1199).unwrap();
        let (o2, p2) = AmonRecord::pixel_of(86.32, 8.1199).unwrap();
        assert_eq!(o1, o2);
        assert_eq!(p1, p2);
    }
}
