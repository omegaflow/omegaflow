use crate::archivar::types::C_LIGHT;

pub const ZTF_MAGIC: [u8; 4] = *b"ZTF1";
pub const ZTF_HEADER_BYTES: usize = 8;

pub const ZTF_G_LAMBDA_NM: f64 = 472.2;
pub const ZTF_R_LAMBDA_NM: f64 = 634.0;
pub const ZTF_I_LAMBDA_NM: f64 = 788.4;
pub const ZTF_G_BAND_NM: f64 = 150.0;
pub const ZTF_R_BAND_NM: f64 = 170.0;
pub const ZTF_I_BAND_NM: f64 = 210.0;

pub fn band_freq_width(filtercode: &str) -> Option<(f64, f64)> {
    let (lam_nm, band_nm) = match filtercode {
        "zg" => (ZTF_G_LAMBDA_NM, ZTF_G_BAND_NM),
        "zr" => (ZTF_R_LAMBDA_NM, ZTF_R_BAND_NM),
        "zi" => (ZTF_I_LAMBDA_NM, ZTF_I_BAND_NM),
        _ => return None,
    };
    let lam_m = lam_nm * 1e-9;
    let freq = C_LIGHT / lam_m;
    let bin_width = C_LIGHT * (band_nm * 1e-9) / (lam_m * lam_m);
    Some((freq, bin_width))
}

pub fn flux_from_mag(mag: f64) -> f64 {
    10f64.powf(-0.4 * mag)
}

pub fn hjd_to_unix(hjd: f64) -> f64 {
    (hjd - 2440587.5) * 86400.0
}

#[derive(Clone, Debug)]
pub struct ZtfCurve {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub plx_mas: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub samples: Vec<(f64, f32)>,
}

pub fn write_ztf_bin(curves: &[ZtfCurve]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(ZTF_HEADER_BYTES + curves.len() * (8 * 5 + 4 + 64 * 12));
    out.extend_from_slice(&ZTF_MAGIC);
    out.extend_from_slice(&(curves.len() as u32).to_le_bytes());
    for c in curves {
        if !c.ra_deg.is_finite()
            || !c.dec_deg.is_finite()
            || !c.plx_mas.is_finite()
            || !c.freq.is_finite()
            || !c.bin_width.is_finite()
        {
            return None;
        }
        out.extend_from_slice(&c.ra_deg.to_le_bytes());
        out.extend_from_slice(&c.dec_deg.to_le_bytes());
        out.extend_from_slice(&c.plx_mas.to_le_bytes());
        out.extend_from_slice(&c.freq.to_le_bytes());
        out.extend_from_slice(&c.bin_width.to_le_bytes());
        out.extend_from_slice(&(c.samples.len() as u32).to_le_bytes());
        for (t, f) in &c.samples {
            if !t.is_finite() || !f.is_finite() {
                return None;
            }
            out.extend_from_slice(&t.to_le_bytes());
            out.extend_from_slice(&f.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_ztf_bin(bytes: &[u8]) -> Option<Vec<ZtfCurve>> {
    if bytes.len() < ZTF_HEADER_BYTES || bytes[0..4] != ZTF_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = ZTF_HEADER_BYTES;
    let mut curves = Vec::with_capacity(count);
    for _ in 0..count {
        let f64_at = |o: &mut usize| -> Option<f64> {
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            Some(v)
        };
        let ra_deg = f64_at(&mut off)?;
        let dec_deg = f64_at(&mut off)?;
        let plx_mas = f64_at(&mut off)?;
        let freq = f64_at(&mut off)?;
        let bin_width = f64_at(&mut off)?;
        let n_samples = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?) as usize;
        off += 4;
        if !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !plx_mas.is_finite()
            || !freq.is_finite()
            || !bin_width.is_finite()
        {
            return None;
        }
        let mut samples = Vec::with_capacity(n_samples);
        for _ in 0..n_samples {
            let t = f64_at(&mut off)?;
            let f = f32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            if !t.is_finite() || !f.is_finite() {
                return None;
            }
            samples.push((t, f));
        }
        curves.push(ZtfCurve {
            ra_deg,
            dec_deg,
            plx_mas,
            freq,
            bin_width,
            samples,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(curves)
}

pub struct ZtfRow {
    pub oid: String,
    pub hjd: f64,
    pub mag: f64,
    pub filtercode: String,
    pub ra: f64,
    pub dec: f64,
}

pub fn parse_csv(text: &str) -> Vec<ZtfRow> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let idx = |name: &str| cols.iter().position(|c| *c == name);
    let (Some(i_oid), Some(i_hjd), Some(i_mag), Some(i_fc), Some(i_ra), Some(i_dec)) = (
        idx("oid"),
        idx("hjd"),
        idx("mag"),
        idx("filtercode"),
        idx("ra"),
        idx("dec"),
    ) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in lines {
        let cells: Vec<&str> = line.split(',').collect();
        let get = |i: usize| cells.get(i).map(|s| s.trim());
        let Some(oid) = get(i_oid).filter(|s| !s.is_empty()) else {
            continue;
        };
        let Some(hjd) = get(i_hjd)
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
        else {
            continue;
        };
        let Some(mag) = get(i_mag)
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
        else {
            continue;
        };
        let Some(filtercode) = get(i_fc).filter(|s| !s.is_empty()) else {
            continue;
        };
        let Some(ra) = get(i_ra)
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
        else {
            continue;
        };
        let Some(dec) = get(i_dec)
            .and_then(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
        else {
            continue;
        };
        out.push(ZtfRow {
            oid: oid.to_string(),
            hjd,
            mag,
            filtercode: filtercode.to_string(),
            ra,
            dec,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_mapping_ztf_filters() {
        let (g_f, g_b) = band_freq_width("zg").unwrap();
        assert!((g_f - C_LIGHT / 472.2e-9).abs() / g_f < 1e-12);
        assert!((g_b - C_LIGHT * 150.0e-9 / (472.2e-9 * 472.2e-9)).abs() / g_b < 1e-12);
        let (r_f, _) = band_freq_width("zr").unwrap();
        assert!((r_f - C_LIGHT / 634.0e-9).abs() / r_f < 1e-12);
        let (i_f, _) = band_freq_width("zi").unwrap();
        assert!((i_f - C_LIGHT / 788.4e-9).abs() / i_f < 1e-12);
        assert!(band_freq_width("zg ").is_none());
        assert!(band_freq_width("g").is_none());
        assert!(band_freq_width("").is_none());
    }

    #[test]
    fn flux_from_mag_linear_scale() {
        assert_eq!(flux_from_mag(0.0), 1.0);
        assert!((flux_from_mag(5.0) - 0.01).abs() < 1e-12);
        assert!((flux_from_mag(-2.5) - 10.0).abs() < 1e-10);
    }

    #[test]
    fn hjd_to_unix_carries_j2000_offset() {
        assert_eq!(hjd_to_unix(2440587.5), 0.0);
        assert!((hjd_to_unix(2451545.0) - 946728000.0).abs() < 1e-6);
    }

    #[test]
    fn ztf_bin_roundtrip() {
        let curves = vec![
            ZtfCurve {
                ra_deg: 210.5,
                dec_deg: 30.25,
                plx_mas: 0.0,
                freq: 6.35e14,
                bin_width: 2.0e14,
                samples: vec![(8.0e8, 1.0e-6), (8.1e8, 1.1e-6)],
            },
            ZtfCurve {
                ra_deg: 211.0,
                dec_deg: 31.0,
                plx_mas: 0.0,
                freq: 4.73e14,
                bin_width: 1.3e14,
                samples: vec![(8.0e8, 2.0e-6)],
            },
        ];
        let bytes = write_ztf_bin(&curves).unwrap();
        let parsed = parse_ztf_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, 210.5);
        assert_eq!(parsed[0].dec_deg, 30.25);
        assert_eq!(parsed[0].freq, 6.35e14);
        assert_eq!(parsed[0].bin_width, 2.0e14);
        assert_eq!(parsed[0].samples, curves[0].samples);
        assert_eq!(parsed[1].samples, curves[1].samples);
    }

    #[test]
    fn ztf_bin_refuses_malformed() {
        assert!(parse_ztf_bin(b"ZTF1").is_none());
        assert!(parse_ztf_bin(b"ZTF2").is_none());
        let curves = vec![ZtfCurve {
            ra_deg: 210.5,
            dec_deg: 30.25,
            plx_mas: 0.0,
            freq: 6.35e14,
            bin_width: 2.0e14,
            samples: vec![(8.0e8, 1.0e-6)],
        }];
        let bytes = write_ztf_bin(&curves).unwrap();
        assert!(parse_ztf_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn parse_csv_columns_and_rows() {
        let text = "oid,expid,hjd,mjd,mag,magerr,catflags,filtercode,ra,dec,chi\n\
ZTF18aaaaaag,1,58000.5,58000.1,15.2,0.03,0,zg,210.5,30.25,1.0\n\
ZTF18aaaaaag,2,58001.5,58001.1,15.4,0.03,0,zr,210.5,30.25,1.0\n\
ZTF18bbbbbbb,1,58000.5,58000.1,,0.03,0,zi,211.0,31.0,1.0\n";
        let rows = parse_csv(text);
        assert_eq!(
            rows.len(),
            2,
            "a row without mag is skipped, never fabricated"
        );
        assert_eq!(rows[0].oid, "ZTF18aaaaaag");
        assert_eq!(rows[0].hjd, 58000.5);
        assert_eq!(rows[0].mag, 15.2);
        assert_eq!(rows[0].filtercode, "zg");
        assert_eq!(rows[0].ra, 210.5);
        assert_eq!(rows[1].filtercode, "zr");
    }

    #[test]
    fn parse_csv_absent_columns_returns_void() {
        assert!(parse_csv("oid,mag\nZTF18aaaaaag,15.2\n").is_empty());
        assert!(parse_csv("").is_empty());
    }
}
