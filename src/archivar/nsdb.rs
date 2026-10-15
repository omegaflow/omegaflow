pub const MAGIC: [u8; 4] = *b"NSDB";
pub const HEADER_BYTES: usize = 8;
pub const RECORD_BYTES: usize = 40;
const FLAG_MAG: u32 = 1 << 8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obs {
    pub epoch_unix: f64,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub mag: Option<f64>,
    pub band: u8,
    pub obscode: u32,
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Obs>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * RECORD_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(
                bytes.get(base + o..base + o + 8)?.try_into().ok()?,
            ))
        };
        let u32_at = |o: usize| -> Option<u32> {
            Some(u32::from_le_bytes(
                bytes.get(base + o..base + o + 4)?.try_into().ok()?,
            ))
        };
        let epoch_unix = f64_at(0)?;
        let ra_deg = f64_at(8)?;
        let dec_deg = f64_at(16)?;
        let mag_raw = f64_at(24)?;
        let obscode = u32_at(32)?;
        let flags = u32_at(36)?;
        if !epoch_unix.is_finite() || !ra_deg.is_finite() || !dec_deg.is_finite() {
            return None;
        }
        if !(0.0..360.0).contains(&ra_deg) || !(-90.0..=90.0).contains(&dec_deg) {
            return None;
        }
        let mag = if flags & FLAG_MAG != 0 {
            mag_raw.is_finite().then_some(mag_raw)
        } else {
            None
        };
        out.push(Obs {
            epoch_unix,
            ra_deg,
            dec_deg,
            mag,
            band: (flags & 0xFF) as u8,
            obscode,
        });
    }
    Some(out)
}

pub fn direction_unit(ra_deg: f64, dec_deg: f64) -> Option<[f64; 3]> {
    if !ra_deg.is_finite() || !dec_deg.is_finite() {
        return None;
    }
    let ra = ra_deg.to_radians();
    let dec = dec_deg.to_radians();
    let cd = dec.cos();
    let d = [cd * ra.cos(), cd * ra.sin(), dec.sin()];
    let norm = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if !norm.is_finite() || norm <= 0.0 {
        return None;
    }
    Some([d[0] / norm, d[1] / norm, d[2] / norm])
}

pub fn directions(obs: &[Obs]) -> Vec<([f64; 3], f64)> {
    obs.iter()
        .filter_map(|o| direction_unit(o.ra_deg, o.dec_deg).map(|d| (d, o.epoch_unix)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enc(obs: &[Obs]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(obs.len() as u32).to_le_bytes());
        for o in obs {
            out.extend_from_slice(&o.epoch_unix.to_le_bytes());
            out.extend_from_slice(&o.ra_deg.to_le_bytes());
            out.extend_from_slice(&o.dec_deg.to_le_bytes());
            out.extend_from_slice(&o.mag.unwrap_or(0.0).to_le_bytes());
            out.extend_from_slice(&o.obscode.to_le_bytes());
            let flags = u32::from(o.band) | if o.mag.is_some() { FLAG_MAG } else { 0 };
            out.extend_from_slice(&flags.to_le_bytes());
        }
        out
    }

    #[test]
    fn bin_roundtrips_and_holds_absent_magnitude() {
        let obs = vec![
            Obs {
                epoch_unix: 1.02e9,
                ra_deg: 311.4545,
                dec_deg: -18.1667,
                mag: None,
                band: 0,
                obscode: 807,
            },
            Obs {
                epoch_unix: 1.03e9,
                ra_deg: 353.39,
                dec_deg: -4.32,
                mag: Some(26.5),
                band: b'r',
                obscode: 568,
            },
        ];
        let bytes = enc(&obs);
        assert_eq!(parse_bin(&bytes), Some(obs));
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn a_direction_is_a_unit_vector() {
        let d = direction_unit(0.0, 0.0).expect("the vernal equinox is a direction");
        assert!((d[0] - 1.0).abs() < 1e-12);
        assert!(d[1].abs() < 1e-12);
        assert!(d[2].abs() < 1e-12);
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((n - 1.0).abs() < 1e-12);
        let pole = direction_unit(0.0, 90.0).expect("the pole is a direction");
        assert!((pole[2] - 1.0).abs() < 1e-12);
        assert!(direction_unit(f64::NAN, 0.0).is_none());
    }

    #[test]
    fn directions_drop_only_non_finite_angles() {
        let obs = vec![
            Obs {
                epoch_unix: 1.0,
                ra_deg: 10.0,
                dec_deg: 20.0,
                mag: None,
                band: 0,
                obscode: 1,
            },
            Obs {
                epoch_unix: 2.0,
                ra_deg: f64::NAN,
                dec_deg: 0.0,
                mag: None,
                band: 0,
                obscode: 1,
            },
        ];
        assert_eq!(directions(&obs).len(), 1);
    }
}
