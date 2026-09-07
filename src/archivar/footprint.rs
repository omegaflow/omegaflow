use crate::mathematikerin::healpix::ang2pix_nest;

pub const MAGIC: [u8; 4] = *b"FP01";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 13;
pub const REC_BYTES: usize = 12;
pub const NSIDE: i64 = 4096;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FootprintBand {
    U = 0,
    G = 1,
    R = 2,
    I = 3,
    Z = 4,
    Y = 5,
    J = 6,
    H = 7,
    Ks = 8,
    W1 = 9,
    W2 = 10,
}

pub fn band_code(band: FootprintBand) -> u8 {
    band as u8
}

pub fn band_from_code(code: u8) -> Option<FootprintBand> {
    match code {
        0 => Some(FootprintBand::U),
        1 => Some(FootprintBand::G),
        2 => Some(FootprintBand::R),
        3 => Some(FootprintBand::I),
        4 => Some(FootprintBand::Z),
        5 => Some(FootprintBand::Y),
        6 => Some(FootprintBand::J),
        7 => Some(FootprintBand::H),
        8 => Some(FootprintBand::Ks),
        9 => Some(FootprintBand::W1),
        10 => Some(FootprintBand::W2),
        _ => None,
    }
}

#[derive(Clone, Copy)]
pub struct FootprintRecord {
    pub order: u8,
    pub band: FootprintBand,
    pub ipix: u32,
    pub frac: f32,
}

impl FootprintRecord {
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

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &FootprintRecord) {
    out.fill(0);
    out[0] = r.order;
    out[1] = band_code(r.band);
    out[4..8].copy_from_slice(&r.ipix.to_le_bytes());
    out[8..12].copy_from_slice(&r.frac.to_le_bytes());
}

pub fn decode_rec(b: &[u8]) -> Option<FootprintRecord> {
    if b.len() != REC_BYTES {
        return None;
    }
    let order = b[0];
    if order > 29 {
        return None;
    }
    let band = band_from_code(b[1])?;
    let ipix = u32::from_le_bytes(b[4..8].try_into().ok()?);
    let nside = 1i64 << order;
    if (ipix as i64) >= 12 * nside * nside {
        return None;
    }
    let frac = f32::from_le_bytes(b[8..12].try_into().ok()?);
    if !(frac.is_finite() && (0.0..=1.0).contains(&frac)) {
        return None;
    }
    Some(FootprintRecord {
        order,
        band,
        ipix,
        frac,
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FootprintVerdict {
    Observed,
    NeverObserved,
    BandUncovered,
    Pending,
}

pub fn find_pixel_records(records: &[FootprintRecord], ipix: u32) -> &[FootprintRecord] {
    let start = records.partition_point(|r| r.ipix < ipix);
    let end = records.partition_point(|r| r.ipix <= ipix);
    &records[start..end]
}

pub fn direction_gate(
    records: &[FootprintRecord],
    ra_deg: f64,
    dec_deg: f64,
    band: FootprintBand,
) -> FootprintVerdict {
    match FootprintRecord::pixel_of(ra_deg, dec_deg) {
        Some((_, ipix)) => footprint_gate(Some(find_pixel_records(records, ipix)), band),
        None => FootprintVerdict::Pending,
    }
}

pub fn footprint_gate(pixel: Option<&[FootprintRecord]>, band: FootprintBand) -> FootprintVerdict {
    let Some(records) = pixel else {
        return FootprintVerdict::Pending;
    };
    if records.is_empty() {
        return FootprintVerdict::NeverObserved;
    }
    match records.iter().find(|r| r.band == band) {
        Some(r) if r.frac.is_finite() && r.frac > 0.0 => FootprintVerdict::Observed,
        _ => FootprintVerdict::BandUncovered,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> FootprintRecord {
        let (order, ipix) = FootprintRecord::pixel_of(40.0, -30.0).unwrap();
        FootprintRecord {
            order,
            band: FootprintBand::G,
            ipix,
            frac: 0.75,
        }
    }

    #[test]
    fn pixel_of_covers_both_poles_and_refuses_unplausible() {
        assert!(FootprintRecord::pixel_of(0.0, 90.0).is_some());
        assert!(FootprintRecord::pixel_of(0.0, -90.0).is_some());
        assert!(FootprintRecord::pixel_of(40.0, -30.0).is_some());
        assert!(FootprintRecord::pixel_of(10.0, 90.5).is_none());
        assert!(FootprintRecord::pixel_of(f64::NAN, 0.0).is_none());
    }

    #[test]
    fn pixel_order_is_nside4096() {
        let (order, _) = FootprintRecord::pixel_of(40.0, -30.0).unwrap();
        assert_eq!(order, 12);
    }

    #[test]
    fn band_code_roundtrip() {
        for code in 0..=10u8 {
            assert_eq!(band_code(band_from_code(code).unwrap()), code);
        }
        assert_eq!(band_from_code(11), None);
        assert_eq!(band_from_code(255), None);
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
        assert_eq!(back.band, FootprintBand::G);
        assert_eq!(back.ipix, r.ipix);
        assert_eq!(back.frac, 0.75);
    }

    #[test]
    fn record_holds_a_real_zero_frac() {
        let mut r = sample();
        r.frac = 0.0;
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        let back = decode_rec(&b).unwrap();
        assert_eq!(back.frac, 0.0);
    }

    #[test]
    fn record_refuses_corruption() {
        let r = sample();
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        assert!(decode_rec(&b[..REC_BYTES - 1]).is_none());
        b[0] = 40;
        assert!(decode_rec(&b).is_none());
        b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        b[1] = 11;
        assert!(decode_rec(&b).is_none());
        b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        b[8..12].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(decode_rec(&b).is_none());
        b = [0u8; REC_BYTES];
        encode_rec(&mut b, &r);
        b[8..12].copy_from_slice(&1.5f32.to_le_bytes());
        assert!(decode_rec(&b).is_none());
    }

    #[test]
    fn gate_distinguishes_the_three_states() {
        let g = FootprintRecord {
            order: 12,
            band: FootprintBand::G,
            ipix: 0,
            frac: 0.9,
        };
        let z_zero = FootprintRecord {
            order: 12,
            band: FootprintBand::Z,
            ipix: 0,
            frac: 0.0,
        };
        assert_eq!(
            footprint_gate(None, FootprintBand::G),
            FootprintVerdict::Pending
        );
        assert_eq!(
            footprint_gate(Some(&[]), FootprintBand::G),
            FootprintVerdict::NeverObserved
        );
        assert_eq!(
            footprint_gate(Some(&[g]), FootprintBand::G),
            FootprintVerdict::Observed
        );
        assert_eq!(
            footprint_gate(Some(&[g]), FootprintBand::Z),
            FootprintVerdict::BandUncovered
        );
        assert_eq!(
            footprint_gate(Some(&[g, z_zero]), FootprintBand::Z),
            FootprintVerdict::BandUncovered
        );
    }

    #[test]
    fn find_pixel_records_returns_the_pixel_band_range() {
        let mut recs = Vec::new();
        for (ipix, band, frac) in [
            (5u32, FootprintBand::G, 0.9f32),
            (5, FootprintBand::R, 0.8),
            (5, FootprintBand::Z, 0.0),
            (9, FootprintBand::G, 0.5),
        ] {
            recs.push(FootprintRecord {
                order: 12,
                band,
                ipix,
                frac,
            });
        }
        recs.sort_by(|a, b| {
            a.ipix
                .cmp(&b.ipix)
                .then(band_code(a.band).cmp(&band_code(b.band)))
        });
        let five = find_pixel_records(&recs, 5);
        assert_eq!(five.len(), 3);
        assert!(five.iter().all(|r| r.ipix == 5));
        assert_eq!(find_pixel_records(&recs, 7).len(), 0);
        assert_eq!(find_pixel_records(&recs, 9).len(), 1);
    }

    #[test]
    fn direction_gate_places_observed_never_observed_and_uncovered() {
        let (_, ipix) = FootprintRecord::pixel_of(40.0, -30.0).unwrap();
        let recs = [FootprintRecord {
            order: 12,
            band: FootprintBand::G,
            ipix,
            frac: 0.9,
        }];
        assert_eq!(
            direction_gate(&recs, 40.0, -30.0, FootprintBand::G),
            FootprintVerdict::Observed
        );
        assert_eq!(
            direction_gate(&recs, 40.0, -30.0, FootprintBand::Z),
            FootprintVerdict::BandUncovered
        );
        assert_eq!(
            direction_gate(&recs, 200.0, 85.0, FootprintBand::G),
            FootprintVerdict::NeverObserved
        );
        assert_eq!(
            direction_gate(&recs, f64::NAN, 0.0, FootprintBand::G),
            FootprintVerdict::Pending
        );
    }
}
