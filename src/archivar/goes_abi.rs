pub const MAGIC: [u8; 4] = *b"GAB1";
pub const VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 12;
pub const REC_BYTES: usize = 56;

pub const CALIB_L1B: u8 = 0;
pub const CALIB_GSICS_PENDING: u8 = 1;
pub const CALIB_GSICS: u8 = 2;

pub const COMP_RADIANCE: u32 = 0;

#[derive(Clone, Debug)]
pub struct AbiGranule {
    pub t: f64,
    pub band_id: u8,
    pub calib: u8,
    pub band_wavelength: f32,
    pub esun: f32,
    pub kappa0: f32,
    pub sub_lon: f32,
    pub persp_h: f32,
    pub rad_mean: f32,
    pub rad_std: f32,
    pub rad_min: f32,
    pub rad_max: f32,
    pub valid: u32,
    pub total: u32,
}

fn calib_known(calib: u8) -> bool {
    matches!(calib, CALIB_L1B | CALIB_GSICS_PENDING | CALIB_GSICS)
}

pub fn write_bin(records: &[AbiGranule]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&[0u8; 3]);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for g in records {
        if !g.t.is_finite()
            || !g.band_wavelength.is_finite()
            || !g.esun.is_finite()
            || !g.kappa0.is_finite()
            || !g.sub_lon.is_finite()
            || !g.persp_h.is_finite()
            || !g.rad_mean.is_finite()
            || g.rad_mean <= 0.0
            || !g.rad_std.is_finite()
            || !g.rad_min.is_finite()
            || !g.rad_max.is_finite()
            || !calib_known(g.calib)
        {
            return None;
        }
        let mut rec = [0u8; REC_BYTES];
        rec[0..8].copy_from_slice(&g.t.to_le_bytes());
        rec[8] = g.band_id;
        rec[9] = g.calib;
        rec[12..16].copy_from_slice(&g.band_wavelength.to_le_bytes());
        rec[16..20].copy_from_slice(&g.esun.to_le_bytes());
        rec[20..24].copy_from_slice(&g.kappa0.to_le_bytes());
        rec[24..28].copy_from_slice(&g.sub_lon.to_le_bytes());
        rec[28..32].copy_from_slice(&g.persp_h.to_le_bytes());
        rec[32..36].copy_from_slice(&g.rad_mean.to_le_bytes());
        rec[36..40].copy_from_slice(&g.rad_std.to_le_bytes());
        rec[40..44].copy_from_slice(&g.rad_min.to_le_bytes());
        rec[44..48].copy_from_slice(&g.rad_max.to_le_bytes());
        rec[48..52].copy_from_slice(&g.valid.to_le_bytes());
        rec[52..56].copy_from_slice(&g.total.to_le_bytes());
        out.extend_from_slice(&rec);
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<AbiGranule>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC || bytes[4] != VERSION {
        return None;
    }
    let n = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let rec = bytes.get(off..off + REC_BYTES)?;
        off += REC_BYTES;
        let f32_at = |r: std::ops::Range<usize>| {
            rec.get(r)
                .and_then(|x| x.try_into().ok())
                .map(f32::from_le_bytes)
        };
        let g = AbiGranule {
            t: f64::from_le_bytes(rec[0..8].try_into().ok()?),
            band_id: rec[8],
            calib: rec[9],
            band_wavelength: f32_at(12..16)?,
            esun: f32_at(16..20)?,
            kappa0: f32_at(20..24)?,
            sub_lon: f32_at(24..28)?,
            persp_h: f32_at(28..32)?,
            rad_mean: f32_at(32..36)?,
            rad_std: f32_at(36..40)?,
            rad_min: f32_at(40..44)?,
            rad_max: f32_at(44..48)?,
            valid: u32::from_le_bytes(rec[48..52].try_into().ok()?),
            total: u32::from_le_bytes(rec[52..56].try_into().ok()?),
        };
        if !calib_known(g.calib)
            || !g.t.is_finite()
            || !g.band_wavelength.is_finite()
            || !g.rad_mean.is_finite()
            || g.rad_mean <= 0.0
        {
            return None;
        }
        out.push(g);
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_RADIANCE => Some("goes_abi_radiance"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn granule() -> AbiGranule {
        AbiGranule {
            t: 729_777_632.227_242_1,
            band_id: 1,
            calib: CALIB_GSICS,
            band_wavelength: 0.47,
            esun: 0.0,
            kappa0: 0.0,
            sub_lon: -75.0,
            persp_h: 35_786_000.0,
            rad_mean: 5.56,
            rad_std: 0.5,
            rad_min: 0.1,
            rad_max: 12.0,
            valid: 30_000_000,
            total: 30_250_000,
        }
    }

    #[test]
    fn roundtrip() {
        let g = granule();
        let bytes = write_bin(std::slice::from_ref(&g)).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        let b = &parsed[0];
        assert_eq!(b.t, g.t);
        assert_eq!(b.band_id, g.band_id);
        assert_eq!(b.calib, g.calib);
        assert_eq!(b.band_wavelength, g.band_wavelength);
        assert_eq!(b.sub_lon, g.sub_lon);
        assert_eq!(b.persp_h, g.persp_h);
        assert_eq!(b.rad_mean, g.rad_mean);
        assert_eq!(b.valid, g.valid);
        assert_eq!(b.total, g.total);
    }

    #[test]
    fn refuses_foreign_magic_and_truncated() {
        assert!(parse_bin(b"GKA1").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&[granule()]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn refuses_unknown_calibration_and_nonpositive_radiance() {
        let mut g = granule();
        g.calib = 9;
        assert!(write_bin(&[g]).is_none());
        let mut g = granule();
        g.rad_mean = 0.0;
        assert!(write_bin(&[g]).is_none());
        let mut g = granule();
        g.rad_mean = f32::NAN;
        assert!(write_bin(&[g]).is_none());
    }

    #[test]
    fn component_name_maps_radiance() {
        assert_eq!(component_name(COMP_RADIANCE), Some("goes_abi_radiance"));
        assert_eq!(component_name(7), None);
    }
}
