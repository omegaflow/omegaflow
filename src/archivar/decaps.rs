pub const MAGIC: [u8; 4] = [0xCF, 0x86, 0x05, 0x00];
pub const HEADER_BYTES: usize = 16;
pub const RECORD_BYTES: usize = 56;
pub const F32_FIELDS: usize = 10;

pub const COMP_PARALLAX: u32 = 1;
pub const COMP_MAG_R: u32 = 2;
pub const COMP_MAG_I: u32 = 3;
pub const COMP_MAG_Z: u32 = 4;
pub const COMP_DIST: u32 = 5;
pub const COMP_EXTINCTION: u32 = 6;
pub const COMP_RV: u32 = 7;
pub const COMP_LOGT: u32 = 8;
pub const COMP_MINI: u32 = 9;
pub const COMP_MAG_G: u32 = 10;
pub const COMP_MAX: u32 = 10;

#[derive(Clone, Debug, PartialEq)]
pub struct DecapsStar {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub parallax_arcsec: Option<f64>,
    pub mag_r: Option<f64>,
    pub mag_i: Option<f64>,
    pub mag_z: Option<f64>,
    pub dist_pc: Option<f64>,
    pub extinction_mag: Option<f64>,
    pub rv_km_s: Option<f64>,
    pub logt_yr: Option<f64>,
    pub mini: Option<f64>,
    pub mag_g: f64,
}

fn present_f32(v: f32) -> Option<f64> {
    if v.is_finite() && v > 0.0 {
        Some(v as f64)
    } else {
        None
    }
}

pub fn write_bin(stars: &[DecapsStar]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_BYTES + stars.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(stars.len() as u32).to_le_bytes());
    out.extend_from_slice(&(RECORD_BYTES as u32).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    for s in stars {
        if !s.ra_deg.is_finite() || !(0.0..360.0).contains(&s.ra_deg) {
            return None;
        }
        if !s.dec_deg.is_finite() || !(-90.0..=90.0).contains(&s.dec_deg) {
            return None;
        }
        if !s.mag_g.is_finite() || s.mag_g <= 0.0 {
            return None;
        }
        out.extend_from_slice(&s.ra_deg.to_le_bytes());
        out.extend_from_slice(&s.dec_deg.to_le_bytes());
        for v in [
            s.parallax_arcsec,
            s.mag_r,
            s.mag_i,
            s.mag_z,
            s.dist_pc,
            s.extinction_mag,
            s.rv_km_s,
            s.logt_yr,
            s.mini,
        ] {
            match v {
                Some(x) if x.is_finite() && x > 0.0 => {
                    out.extend_from_slice(&(x as f32).to_le_bytes())
                }
                _ => out.extend_from_slice(&0.0f32.to_le_bytes()),
            }
        }
        out.extend_from_slice(&(s.mag_g as f32).to_le_bytes());
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<DecapsStar>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let rec_bytes = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if rec_bytes != RECORD_BYTES {
        return None;
    }
    if bytes.len() != HEADER_BYTES + count * RECORD_BYTES {
        return None;
    }
    let mut stars = Vec::with_capacity(count);
    for i in 0..count {
        let start = HEADER_BYTES + i * RECORD_BYTES;
        let rec = bytes.get(start..start + RECORD_BYTES)?;
        let ra_deg = f64::from_le_bytes(rec[0..8].try_into().ok()?);
        let dec_deg = f64::from_le_bytes(rec[8..16].try_into().ok()?);
        if !ra_deg.is_finite() || !(0.0..360.0).contains(&ra_deg) {
            return None;
        }
        if !dec_deg.is_finite() || !(-90.0..=90.0).contains(&dec_deg) {
            return None;
        }
        let f32_at = |k: usize| -> Option<f32> {
            Some(f32::from_le_bytes(
                rec.get(16 + k * 4..20 + k * 4)?.try_into().ok()?,
            ))
        };
        let mag_g = f32_at(9)?;
        if !mag_g.is_finite() || mag_g <= 0.0 {
            return None;
        }
        stars.push(DecapsStar {
            ra_deg,
            dec_deg,
            parallax_arcsec: present_f32(f32_at(0)?),
            mag_r: present_f32(f32_at(1)?),
            mag_i: present_f32(f32_at(2)?),
            mag_z: present_f32(f32_at(3)?),
            dist_pc: present_f32(f32_at(4)?),
            extinction_mag: present_f32(f32_at(5)?),
            rv_km_s: present_f32(f32_at(6)?),
            logt_yr: present_f32(f32_at(7)?),
            mini: present_f32(f32_at(8)?),
            mag_g: mag_g as f64,
        });
    }
    Some(stars)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_PARALLAX => Some("decaps_dr2_parallax_arcsec"),
        COMP_MAG_R => Some("decaps_dr2_r_mag"),
        COMP_MAG_I => Some("decaps_dr2_i_mag"),
        COMP_MAG_Z => Some("decaps_dr2_z_mag"),
        COMP_DIST => Some("decaps_dr2_dist_pc"),
        COMP_EXTINCTION => Some("decaps_dr2_extinction_mag"),
        COMP_RV => Some("decaps_dr2_rv_km_s"),
        COMP_LOGT => Some("decaps_dr2_logt_yr"),
        COMP_MINI => Some("decaps_dr2_mini"),
        COMP_MAG_G => Some("decaps_dr2_g_mag"),
        _ => None,
    }
}

pub fn component_value(star: &DecapsStar, comp: u32) -> Option<f64> {
    match comp {
        COMP_PARALLAX => star.parallax_arcsec,
        COMP_MAG_R => star.mag_r,
        COMP_MAG_I => star.mag_i,
        COMP_MAG_Z => star.mag_z,
        COMP_DIST => star.dist_pc,
        COMP_EXTINCTION => star.extinction_mag,
        COMP_RV => star.rv_km_s,
        COMP_LOGT => star.logt_yr,
        COMP_MINI => star.mini,
        COMP_MAG_G => Some(star.mag_g),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn star() -> DecapsStar {
        DecapsStar {
            ra_deg: 123.456,
            dec_deg: -12.5,
            parallax_arcsec: Some(0.25),
            mag_r: Some(18.4),
            mag_i: None,
            mag_z: None,
            dist_pc: Some(4000.0),
            extinction_mag: None,
            rv_km_s: None,
            logt_yr: None,
            mini: None,
            mag_g: 19.1,
        }
    }

    #[test]
    fn roundtrip_preserves_absence() {
        let bytes = write_bin(&[star()]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].ra_deg, 123.456);
        assert_eq!(parsed[0].dec_deg, -12.5);
        assert_eq!(parsed[0].parallax_arcsec, Some(0.25f32 as f64));
        assert_eq!(parsed[0].mag_i, None);
        assert_eq!(parsed[0].mag_g, 19.1f32 as f64);
    }

    #[test]
    fn refuses_foreign_magic_and_stride() {
        assert!(parse_bin(&[]).is_none());
        assert!(parse_bin(b"NOPE").is_none());
        let bytes = write_bin(&[star()]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut shifted = bytes.clone();
        shifted[8..12].copy_from_slice(&60u32.to_le_bytes());
        assert!(parse_bin(&shifted).is_none());
    }
}
