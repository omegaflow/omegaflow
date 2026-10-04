use crate::mathematikerin::healpix::ang2pix_nest;

pub const MAGIC: [u8; 4] = *b"ERBQ";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 32;
pub const NSIDE: i64 = 1024;

pub const J2000_JD: f64 = 2451545.0;
pub const J2000_UNIX_OFFSET: f64 = 946728000.0;
pub const SECONDS_PER_DAY: f64 = 86400.0;

pub const PRES_LAT: u8 = 0x01;
pub const PRES_LON: u8 = 0x02;
pub const PRES_MAG: u8 = 0x04;
pub const PRES_DEPTH: u8 = 0x08;
pub const PRES_TIME: u8 = 0x10;
pub const PRES_ALL: u8 = 0x1F;

pub const COMP_MAG: u32 = 1;
pub const COMP_DEPTH: u32 = 2;
pub const COMP_MAX: u32 = 2;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_MAG => Some("quake_ptevent_magnitude"),
        COMP_DEPTH => Some("quake_ptevent_depth_km"),
        _ => None,
    }
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<QuakeEvent>> {
    let n = parse_header(bytes)? as usize;
    let body = bytes.get(HEADER_LEN..)?;
    if body.len() < n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let chunk = body.get(i * REC_BYTES..(i + 1) * REC_BYTES)?;
        let fixed: [u8; REC_BYTES] = chunk.try_into().ok()?;
        out.push(decode_rec(&fixed)?);
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let events = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(events.len() * 2);
    for e in events {
        let unix = (e.jd_utc - J2000_JD) * SECONDS_PER_DAY + J2000_UNIX_OFFSET;
        if !unix.is_finite() {
            continue;
        }
        if e.present & PRES_MAG != 0 {
            out.push((unix, e.mag as f64, COMP_MAG));
        }
        if e.present & PRES_DEPTH != 0 {
            out.push((unix, e.depth_km as f64, COMP_DEPTH));
        }
    }
    Some(out)
}

#[derive(Clone, Copy)]
pub struct QuakeEvent {
    pub order: u8,
    pub present: u8,
    pub ipix: u32,
    pub lat_deg: f32,
    pub lon_deg: f32,
    pub mag: f32,
    pub depth_km: f32,
    pub jd_utc: f64,
}

impl QuakeEvent {
    pub fn pixel_of(lat_deg: f64, lon_deg: f64) -> Option<(u8, u32)> {
        if !(lat_deg.is_finite() && lon_deg.is_finite() && (-90.0..=90.0).contains(&lat_deg)) {
            return None;
        }
        let theta = (90.0 - lat_deg).to_radians();
        let phi = lon_deg.rem_euclid(360.0).to_radians();
        let pix = ang2pix_nest(NSIDE, theta, phi)?;
        let npix = 12 * NSIDE * NSIDE;
        if pix < 0 || pix >= npix {
            return None;
        }
        Some((NSIDE.trailing_zeros() as u8, pix as u32))
    }

    pub fn jd_from_unix(unix_seconds: f64) -> Option<f64> {
        if !unix_seconds.is_finite() {
            return None;
        }
        let jd = J2000_JD + (unix_seconds - J2000_UNIX_OFFSET) / SECONDS_PER_DAY;
        if !jd.is_finite() {
            return None;
        }
        Some(jd)
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

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &QuakeEvent) {
    out.fill(0);
    out[0] = r.order;
    out[1] = r.present;
    out[4..8].copy_from_slice(&r.ipix.to_le_bytes());
    out[8..12].copy_from_slice(&r.lat_deg.to_le_bytes());
    out[12..16].copy_from_slice(&r.lon_deg.to_le_bytes());
    out[16..20].copy_from_slice(&r.mag.to_le_bytes());
    out[20..24].copy_from_slice(&r.depth_km.to_le_bytes());
    out[24..32].copy_from_slice(&r.jd_utc.to_le_bytes());
}

pub fn decode_rec(b: &[u8]) -> Option<QuakeEvent> {
    if b.len() != REC_BYTES {
        return None;
    }
    let order = b[0];
    if order > 29 {
        return None;
    }
    let present = b[1];
    if present == 0 || present & !PRES_ALL != 0 {
        return None;
    }
    let ipix = u32::from_le_bytes(b[4..8].try_into().ok()?);
    let nside = 1i64 << order;
    if (ipix as i64) >= 12 * nside * nside {
        return None;
    }
    let lat_deg = f32::from_le_bytes(b[8..12].try_into().ok()?);
    let lon_deg = f32::from_le_bytes(b[12..16].try_into().ok()?);
    let mag = f32::from_le_bytes(b[16..20].try_into().ok()?);
    let depth_km = f32::from_le_bytes(b[20..24].try_into().ok()?);
    let jd_utc = f64::from_le_bytes(b[24..32].try_into().ok()?);
    if !(lat_deg.is_finite()
        && lon_deg.is_finite()
        && mag.is_finite()
        && depth_km.is_finite()
        && jd_utc.is_finite())
    {
        return None;
    }
    if present & PRES_LAT != 0 && !(-90.0..=90.0).contains(&lat_deg) {
        return None;
    }
    if present & PRES_LON != 0 && !(-180.0..=180.0).contains(&lon_deg) {
        return None;
    }
    if present & PRES_DEPTH != 0 && depth_km < 0.0 {
        return None;
    }
    if present & PRES_TIME != 0 && jd_utc <= 0.0 {
        return None;
    }
    Some(QuakeEvent {
        order,
        present,
        ipix,
        lat_deg,
        lon_deg,
        mag,
        depth_km,
        jd_utc,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> QuakeEvent {
        let (order, ipix) = QuakeEvent::pixel_of(-31.5695, -71.6543).unwrap();
        QuakeEvent {
            order,
            present: PRES_ALL,
            ipix,
            lat_deg: -31.5695,
            lon_deg: -71.6543,
            mag: 8.3,
            depth_km: 25.0,
            jd_utc: 2457279.0,
        }
    }

    #[test]
    fn pixel_of_accepts_plausible_coordinates() {
        assert!(QuakeEvent::pixel_of(0.0, 0.0).is_some());
        assert!(QuakeEvent::pixel_of(90.0, 0.0).is_some());
        assert!(QuakeEvent::pixel_of(-90.0, 0.0).is_some());
        assert!(QuakeEvent::pixel_of(35.8, 140.7).is_some());
        assert!(QuakeEvent::pixel_of(-31.5695, -71.6543).is_some());
        assert!(QuakeEvent::pixel_of(37.7453, 141.7494).is_some());
    }

    #[test]
    fn pixel_of_refuses_unplausible_coordinates() {
        assert!(QuakeEvent::pixel_of(91.0, 0.0).is_none());
        assert!(QuakeEvent::pixel_of(-91.0, 0.0).is_none());
        assert!(QuakeEvent::pixel_of(f64::NAN, 0.0).is_none());
        assert!(QuakeEvent::pixel_of(0.0, f64::INFINITY).is_none());
    }

    #[test]
    fn pixel_of_is_stable_for_a_nearby_direction() {
        let (o1, p1) = QuakeEvent::pixel_of(35.8, 140.7).unwrap();
        let (o2, p2) = QuakeEvent::pixel_of(35.81, 140.7).unwrap();
        assert_eq!(o1, o2);
        assert_eq!(p1, p2);
    }

    #[test]
    fn jd_from_unix_carries_the_j2000_convention() {
        let jd = QuakeEvent::jd_from_unix(J2000_UNIX_OFFSET).unwrap();
        assert!((jd - J2000_JD).abs() < 1e-9);
    }

    #[test]
    fn header_roundtrip() {
        let mut b = Vec::new();
        write_header(&mut b, 11);
        assert_eq!(b.len(), HEADER_LEN);
        assert_eq!(parse_header(&b), Some(11));
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
        assert_eq!(back.present, r.present);
        assert_eq!(back.ipix, r.ipix);
        assert_eq!(back.lat_deg, r.lat_deg);
        assert_eq!(back.lon_deg, r.lon_deg);
        assert_eq!(back.mag, r.mag);
        assert_eq!(back.depth_km, r.depth_km);
        assert_eq!(back.jd_utc, r.jd_utc);
    }

    #[test]
    fn absent_fields_keep_the_present_bit_off() {
        let mut r = sample();
        r.present = PRES_MAG | PRES_TIME;
        r.lat_deg = 0.0;
        r.lon_deg = 0.0;
        r.depth_km = 0.0;
        r.order = 0;
        r.ipix = 0;
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        let back = decode_rec(&b).unwrap();
        assert_eq!(back.present, PRES_MAG | PRES_TIME);
        assert_eq!(back.lat_deg, 0.0);
    }

    #[test]
    fn record_refuses_corruption() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert!(decode_rec(&b[..REC_BYTES - 1]).is_none());
        let zero = [0u8; REC_BYTES];
        assert!(decode_rec(&zero).is_none());
        b[1] = 0x80;
        assert!(decode_rec(&b).is_none());
        b[1] = r.present;
        b[0] = 30;
        assert!(decode_rec(&b).is_none());
    }

    #[test]
    fn record_refuses_an_out_of_range_latitude() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        let bad = 91.0f32;
        b[8..12].copy_from_slice(&bad.to_le_bytes());
        assert!(decode_rec(&b).is_none());
    }

    #[test]
    fn record_refuses_a_pixel_beyond_the_hierarchy() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        let nside = 1u32 << b[0];
        let bad = (12 * nside * nside).to_le_bytes();
        b[4..8].copy_from_slice(&bad);
        assert!(decode_rec(&b).is_none());
    }
}
