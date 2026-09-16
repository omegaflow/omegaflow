use crate::mathematikerin::healpix::pix2ang_nest;

pub const MAGIC: [u8; 4] = *b"VLDE";
pub const VERSION: u8 = 1;
pub const HEADER_BYTES: usize = 9;

pub struct DensityField {
    pub nside: i64,
    pub counts: Vec<u64>,
}

fn npix_of(nside: i64) -> Option<usize> {
    if nside <= 0 || nside & (nside - 1) != 0 {
        return None;
    }
    let n = nside as u64;
    let npix = 12u64.checked_mul(n)?.checked_mul(n)?;
    usize::try_from(npix).ok()
}

pub fn parse_asset(b: &[u8]) -> Option<DensityField> {
    if b.len() < HEADER_BYTES || &b[0..4] != MAGIC || b[4] != VERSION {
        return None;
    }
    let nside = u32::from_le_bytes(b[5..9].try_into().ok()?) as i64;
    let npix = npix_of(nside)?;
    if b.len() != HEADER_BYTES + npix * 8 {
        return None;
    }
    let mut counts = Vec::with_capacity(npix);
    for off in (HEADER_BYTES..b.len()).step_by(8) {
        let raw: [u8; 8] = b[off..off + 8].try_into().ok()?;
        counts.push(u64::from_le_bytes(raw));
    }
    Some(DensityField { nside, counts })
}

pub fn pixel_direction(nside: i64, pix: i64) -> Option<[f64; 3]> {
    let (theta, phi) = pix2ang_nest(nside, pix)?;
    let (st, ct) = theta.sin_cos();
    let (sp, cp) = phi.sin_cos();
    Some([st * cp, st * sp, ct])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset_bytes(nside: i64, counts: &[u64]) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + counts.len() * 8);
        out.extend_from_slice(&MAGIC);
        out.push(VERSION);
        out.extend_from_slice(&(nside as u32).to_le_bytes());
        for c in counts {
            out.extend_from_slice(&c.to_le_bytes());
        }
        out
    }

    #[test]
    fn roundtrip_reads_back_counts() {
        let nside = 4i64;
        let npix = (12 * nside * nside) as usize;
        let counts: Vec<u64> = (0..npix as u64).map(|i| i * 3).collect();
        let field = parse_asset(&asset_bytes(nside, &counts)).unwrap();
        assert_eq!(field.nside, nside);
        assert_eq!(field.counts, counts);
    }

    #[test]
    fn parse_refuses_bad_magic_version_and_length() {
        let nside = 2i64;
        let npix = (12 * nside * nside) as usize;
        let counts = vec![0u64; npix];
        assert!(parse_asset(&[]).is_none());
        assert!(parse_asset(b"VLDE\x01").is_none());

        let mut bad_magic = asset_bytes(nside, &counts);
        bad_magic[0] = b'X';
        assert!(parse_asset(&bad_magic).is_none());

        let mut bad_version = asset_bytes(nside, &counts);
        bad_version[4] = 2;
        assert!(parse_asset(&bad_version).is_none());

        let asset = asset_bytes(nside, &counts);
        assert!(parse_asset(&asset[..asset.len() - 8]).is_none());
    }

    #[test]
    fn parse_refuses_non_power_of_two_nside() {
        let mut bad = asset_bytes(2, &vec![0u64; 48]);
        bad[5..9].copy_from_slice(&3u32.to_le_bytes());
        assert!(parse_asset(&bad).is_none());
    }

    #[test]
    fn pixel_direction_is_unit_length() {
        let nside = 8i64;
        let npix = 12 * nside * nside;
        for pix in [0i64, 1, 17, 100, npix - 1] {
            let d = pixel_direction(nside, pix).unwrap();
            let r = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            assert!((r - 1.0).abs() < 1e-9, "pixel {pix} radius {r}");
        }
        assert!(pixel_direction(nside, npix).is_none());
        assert!(pixel_direction(nside, -1).is_none());
    }
}
