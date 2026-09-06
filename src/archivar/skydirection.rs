use crate::archivar::C_LIGHT;
use crate::archivar::HUBBLE_H0;

pub const MAGIC: [u8; 4] = *b"SKD1";
pub const HEADER_BYTES: usize = 8;

#[derive(Clone)]
pub struct SkySample {
    pub tdb: f64,
    pub mag: f64,
}

#[derive(Clone)]
pub struct SkyBandSeries {
    pub band: Option<String>,
    pub samples: Vec<SkySample>,
}

#[derive(Clone)]
pub struct SkyDirection {
    pub name: String,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub sigma_arcsec: Option<f64>,
    pub bands: Vec<SkyBandSeries>,
    pub distance: Option<f64>,
    pub redshift: Option<f64>,
}

impl SkyDirection {
    pub fn unit_direction(&self) -> [f64; 3] {
        let ra = self.ra_deg.to_radians();
        let dec = self.dec_deg.to_radians();
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

    pub fn distance_m(&self) -> Option<f64> {
        match (self.distance, self.redshift) {
            (Some(d), _) if d.is_finite() && d > 0.0 => Some(d),
            (None, Some(z)) if z.is_finite() && z > 0.0 => Some(z * C_LIGHT / HUBBLE_H0),
            _ => None,
        }
    }

    pub fn spatial_position(&self) -> Option<[f64; 3]> {
        let dist = self.distance_m()?;
        let p = self.unit_direction();
        Some([p[0] * dist, p[1] * dist, p[2] * dist])
    }
}

pub fn write_bin(directions: &[SkyDirection]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_BYTES + directions.len() * 96);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(directions.len() as u32).to_le_bytes());
    for d in directions {
        if !d.ra_deg.is_finite() || !d.dec_deg.is_finite() {
            return None;
        }
        let name_len = u32::try_from(d.name.len()).ok()?;
        out.extend_from_slice(&name_len.to_le_bytes());
        out.extend_from_slice(d.name.as_bytes());
        out.extend_from_slice(&d.ra_deg.to_le_bytes());
        out.extend_from_slice(&d.dec_deg.to_le_bytes());
        match d.sigma_arcsec {
            Some(s) if s.is_finite() && s > 0.0 => {
                out.push(1);
                out.extend_from_slice(&s.to_le_bytes());
            }
            Some(_) => return None,
            None => out.push(0),
        }
        match d.distance {
            Some(v) if v.is_finite() => {
                out.push(1);
                out.extend_from_slice(&v.to_le_bytes());
            }
            Some(_) => return None,
            None => out.push(0),
        }
        match d.redshift {
            Some(v) if v.is_finite() => {
                out.push(1);
                out.extend_from_slice(&v.to_le_bytes());
            }
            Some(_) => return None,
            None => out.push(0),
        }
        let bands_len = u32::try_from(d.bands.len()).ok()?;
        out.extend_from_slice(&bands_len.to_le_bytes());
        for b in &d.bands {
            match &b.band {
                Some(name) => {
                    let len = u32::try_from(name.len()).ok()?;
                    out.extend_from_slice(&len.to_le_bytes());
                    out.extend_from_slice(name.as_bytes());
                }
                None => out.extend_from_slice(&0u32.to_le_bytes()),
            }
            let samples_len = u32::try_from(b.samples.len()).ok()?;
            out.extend_from_slice(&samples_len.to_le_bytes());
            for s in &b.samples {
                if !s.tdb.is_finite() || !s.mag.is_finite() {
                    return None;
                }
                out.extend_from_slice(&s.tdb.to_le_bytes());
                out.extend_from_slice(&s.mag.to_le_bytes());
            }
        }
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<SkyDirection>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = HEADER_BYTES;
    let mut directions = Vec::with_capacity(count);
    for _ in 0..count {
        let name_len = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
        off += 4;
        let name_bytes = bytes.get(off..off + name_len)?;
        off += name_len;
        let name = std::str::from_utf8(name_bytes).ok()?.to_string();
        let f64_at = |o: &mut usize| -> Option<f64> {
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            Some(v)
        };
        let ra_deg = f64_at(&mut off)?;
        let dec_deg = f64_at(&mut off)?;
        if !ra_deg.is_finite() || !dec_deg.is_finite() {
            return None;
        }
        let opt_f64 = |o: &mut usize| -> Option<Option<f64>> {
            let flag = *bytes.get(*o)?;
            *o += 1;
            if flag == 0 {
                return Some(None);
            }
            if flag != 1 {
                return None;
            }
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            if !v.is_finite() {
                return None;
            }
            Some(Some(v))
        };
        let sigma_arcsec = opt_f64(&mut off)?;
        let distance = opt_f64(&mut off)?;
        let redshift = opt_f64(&mut off)?;
        let bands_len = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
        off += 4;
        let mut bands = Vec::with_capacity(bands_len);
        for _ in 0..bands_len {
            let band_name_len =
                u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
            off += 4;
            let band = if band_name_len == 0 {
                None
            } else {
                let b = bytes.get(off..off + band_name_len)?;
                off += band_name_len;
                Some(std::str::from_utf8(b).ok()?.to_string())
            };
            let samples_len =
                u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
            off += 4;
            let mut samples = Vec::with_capacity(samples_len);
            for _ in 0..samples_len {
                let tdb = f64_at(&mut off)?;
                let mag = f64_at(&mut off)?;
                if !tdb.is_finite() || !mag.is_finite() {
                    return None;
                }
                samples.push(SkySample { tdb, mag });
            }
            bands.push(SkyBandSeries { band, samples });
        }
        directions.push(SkyDirection {
            name,
            ra_deg,
            dec_deg,
            sigma_arcsec,
            bands,
            distance,
            redshift,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(directions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::PARSEC_M;

    fn sample_direction() -> SkyDirection {
        SkyDirection {
            name: "ZTF21abxxjrh".to_string(),
            ra_deg: 37.284397,
            dec_deg: 9.258595,
            sigma_arcsec: None,
            bands: vec![SkyBandSeries {
                band: None,
                samples: vec![SkySample {
                    tdb: 8.2e8,
                    mag: 19.45389747619629,
                }],
            }],
            distance: None,
            redshift: None,
        }
    }

    #[test]
    fn unit_direction_is_the_measured_icrs_unit_vector() {
        let d = SkyDirection {
            name: "north".to_string(),
            ra_deg: 0.0,
            dec_deg: 90.0,
            sigma_arcsec: None,
            bands: Vec::new(),
            distance: None,
            redshift: None,
        };
        let n = d.unit_direction();
        assert!((n[0]).abs() < 1e-15);
        assert!((n[1]).abs() < 1e-15);
        assert!((n[2] - 1.0).abs() < 1e-15);
        let d = SkyDirection {
            name: "equator".to_string(),
            ra_deg: 90.0,
            dec_deg: 0.0,
            sigma_arcsec: None,
            bands: Vec::new(),
            distance: None,
            redshift: None,
        };
        let e = d.unit_direction();
        assert!((e[0]).abs() < 1e-15);
        assert!((e[1] - 1.0).abs() < 1e-15);
        assert!(e[2].abs() < 1e-15);
        let d = SkyDirection {
            name: "spiral".to_string(),
            ra_deg: 30.0,
            dec_deg: 60.0,
            sigma_arcsec: None,
            bands: Vec::new(),
            distance: None,
            redshift: None,
        };
        let u = d.unit_direction();
        let norm = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt();
        assert!((norm - 1.0).abs() < 1e-12);
        let ra = 30f64.to_radians();
        let dec = 60f64.to_radians();
        assert!((u[0] - dec.cos() * ra.cos()).abs() < 1e-12);
        assert!((u[1] - dec.cos() * ra.sin()).abs() < 1e-12);
        assert!((u[2] - dec.sin()).abs() < 1e-12);
    }

    #[test]
    fn bin_roundtrip_preserves_absent_distance_as_none() {
        let directions = vec![sample_direction()];
        let bytes = write_bin(&directions).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "ZTF21abxxjrh");
        assert_eq!(parsed[0].ra_deg, 37.284397);
        assert_eq!(parsed[0].dec_deg, 9.258595);
        assert_eq!(parsed[0].bands.len(), 1);
        assert_eq!(parsed[0].bands[0].band, None);
        assert_eq!(parsed[0].bands[0].samples.len(), 1);
        assert_eq!(parsed[0].bands[0].samples[0].tdb, 8.2e8);
        assert_eq!(parsed[0].bands[0].samples[0].mag, 19.45389747619629);
        assert_eq!(parsed[0].distance, None);
        assert_eq!(parsed[0].redshift, None);
    }

    #[test]
    fn bin_roundtrip_preserves_a_delivered_distance() {
        let mut d = sample_direction();
        d.distance = Some(1.234e23);
        d.redshift = Some(0.05);
        d.bands[0].band = Some("g".to_string());
        let bytes = write_bin(&[d]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].distance, Some(1.234e23));
        assert_eq!(parsed[0].redshift, Some(0.05));
        assert_eq!(parsed[0].bands[0].band.as_deref(), Some("g"));
    }

    #[test]
    fn bin_roundtrip_holds_multiple_band_series() {
        let d = SkyDirection {
            name: "multiband".to_string(),
            ra_deg: 210.5,
            dec_deg: 30.25,
            sigma_arcsec: None,
            bands: vec![
                SkyBandSeries {
                    band: Some("g".to_string()),
                    samples: vec![
                        SkySample {
                            tdb: 8.1e8,
                            mag: 15.2,
                        },
                        SkySample {
                            tdb: 8.2e8,
                            mag: 15.4,
                        },
                    ],
                },
                SkyBandSeries {
                    band: Some("r".to_string()),
                    samples: vec![SkySample {
                        tdb: 8.1e8,
                        mag: 14.9,
                    }],
                },
            ],
            distance: None,
            redshift: None,
        };
        let bytes = write_bin(&[d]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].bands.len(), 2);
        assert_eq!(parsed[0].bands[0].samples.len(), 2);
        assert_eq!(parsed[0].bands[1].samples.len(), 1);
    }

    #[test]
    fn bin_refuses_malformed() {
        assert!(parse_bin(b"SKD1").is_none());
        assert!(parse_bin(b"SKD2").is_none());
        let bytes = write_bin(&[sample_direction()]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        assert!(parse_bin(b"").is_none());
    }

    #[test]
    fn bin_refuses_non_finite_magnitude() {
        let mut d = sample_direction();
        d.bands[0].samples[0].mag = f64::NAN;
        assert!(write_bin(&[d]).is_none());
    }

    #[test]
    fn empty_asset_is_a_valid_held_state() {
        let bytes = write_bin(&[]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert!(parsed.is_empty());
    }

    #[test]
    fn sigma_roundtrip_carries_the_measured_uncertainty_and_absent_stays_absent() {
        let mut d = sample_direction();
        d.sigma_arcsec = Some(0.512);
        let bytes = write_bin(&[d]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].sigma_arcsec, Some(0.512));
        let bytes = write_bin(&[sample_direction()]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed[0].sigma_arcsec, None);
    }

    #[test]
    fn sigma_rad_converts_arcsec_and_refuses_unplausible() {
        let mut d = sample_direction();
        d.sigma_arcsec = Some(1.0);
        let r = d.angular_uncertainty_rad().unwrap();
        assert!((r - std::f64::consts::PI / 648000.0).abs() < 1e-18);
        d.sigma_arcsec = Some(0.0);
        assert_eq!(d.angular_uncertainty_rad(), None);
        d.sigma_arcsec = None;
        assert_eq!(d.angular_uncertainty_rad(), None);
        assert!(write_bin(&[d]).is_some());
    }

    #[test]
    fn absent_distance_stays_absent_and_spatial_position_stays_none() {
        let d = sample_direction();
        assert_eq!(d.distance_m(), None);
        assert_eq!(d.spatial_position(), None);
    }

    #[test]
    fn delivered_distance_places_the_direction_in_the_block() {
        let mut d = SkyDirection {
            name: "gaia_placed".to_string(),
            ra_deg: 30.0,
            dec_deg: 60.0,
            sigma_arcsec: None,
            bands: Vec::new(),
            distance: Some(2.0 * PARSEC_M),
            redshift: None,
        };
        let pos = d.spatial_position().unwrap();
        let p = d.unit_direction();
        let expect = [
            p[0] * 2.0 * PARSEC_M,
            p[1] * 2.0 * PARSEC_M,
            p[2] * 2.0 * PARSEC_M,
        ];
        for k in 0..3 {
            assert!((pos[k] - expect[k]).abs() < 1.0);
        }
        d.distance = None;
        d.redshift = Some(0.05);
        let hd = d.distance_m().unwrap();
        let expect_hd = 0.05 * C_LIGHT / HUBBLE_H0;
        assert!((hd - expect_hd).abs() < expect_hd * 1e-12);
        let pos = d.spatial_position().unwrap();
        let p = d.unit_direction();
        for k in 0..3 {
            assert!((pos[k] - p[k] * expect_hd).abs() < expect_hd * 1e-9);
        }
    }

    #[test]
    fn bin_refuses_non_finite_sigma() {
        let mut d = sample_direction();
        d.sigma_arcsec = Some(f64::NAN);
        assert!(write_bin(&[d]).is_none());
        let mut d = sample_direction();
        d.sigma_arcsec = Some(-1.0);
        assert!(write_bin(&[d]).is_none());
    }
}
