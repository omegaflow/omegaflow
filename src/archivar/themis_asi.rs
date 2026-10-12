use crate::archivar::FieldConfig;

pub const MAGIC: [u8; 4] = *b"TASI";
pub const HEADER_BYTES: usize = 12;
pub const PIXEL_BYTES: usize = 2;

pub const UNIT: &str = "relative";

pub struct AsiStation {
    pub code: &'static str,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_m: Option<f64>,
}

pub const STATIONS: [AsiStation; 24] = [
    AsiStation {
        code: "ATHA",
        lat_deg: 54.714,
        lon_deg: -113.314,
        alt_m: Some(549.82),
    },
    AsiStation {
        code: "CHBG",
        lat_deg: 49.814,
        lon_deg: -74.419,
        alt_m: Some(363.92),
    },
    AsiStation {
        code: "EKAT",
        lat_deg: 64.717,
        lon_deg: -110.6703,
        alt_m: Some(430.77),
    },
    AsiStation {
        code: "FSIM",
        lat_deg: 61.762,
        lon_deg: -121.221,
        alt_m: Some(154.61),
    },
    AsiStation {
        code: "FSMI",
        lat_deg: 59.984,
        lon_deg: -111.842,
        alt_m: Some(175.62),
    },
    AsiStation {
        code: "FYKN",
        lat_deg: 66.560,
        lon_deg: -145.214,
        alt_m: Some(137.15),
    },
    AsiStation {
        code: "GAKO",
        lat_deg: 62.407,
        lon_deg: -145.158,
        alt_m: Some(589.72),
    },
    AsiStation {
        code: "GBAY",
        lat_deg: 53.316,
        lon_deg: -60.460,
        alt_m: None,
    },
    AsiStation {
        code: "GILL",
        lat_deg: 56.354,
        lon_deg: -94.656,
        alt_m: Some(103.12),
    },
    AsiStation {
        code: "INUV",
        lat_deg: 68.413,
        lon_deg: -133.770,
        alt_m: Some(11.47),
    },
    AsiStation {
        code: "KAPU",
        lat_deg: 49.392,
        lon_deg: -82.320,
        alt_m: Some(179.94),
    },
    AsiStation {
        code: "KIAN",
        lat_deg: 66.971,
        lon_deg: -160.438,
        alt_m: Some(33.15),
    },
    AsiStation {
        code: "KUUJ",
        lat_deg: 58.155,
        lon_deg: -68.532,
        alt_m: Some(85.39),
    },
    AsiStation {
        code: "MCGR",
        lat_deg: 62.953,
        lon_deg: -155.596,
        alt_m: Some(102.57),
    },
    AsiStation {
        code: "NRSQ",
        lat_deg: 61.162,
        lon_deg: -45.442,
        alt_m: Some(40.53),
    },
    AsiStation {
        code: "PGEO",
        lat_deg: 53.815,
        lon_deg: -122.828,
        alt_m: Some(756.26),
    },
    AsiStation {
        code: "PINA",
        lat_deg: 50.163,
        lon_deg: -96.066,
        alt_m: Some(247.63),
    },
    AsiStation {
        code: "RANK",
        lat_deg: 62.828,
        lon_deg: -92.113,
        alt_m: Some(-17.15),
    },
    AsiStation {
        code: "SNAP",
        lat_deg: 63.580,
        lon_deg: -110.870,
        alt_m: Some(406.20),
    },
    AsiStation {
        code: "SNKQ",
        lat_deg: 56.536,
        lon_deg: -79.231,
        alt_m: Some(-11.22),
    },
    AsiStation {
        code: "TALO",
        lat_deg: 69.541,
        lon_deg: -93.554,
        alt_m: Some(0.19),
    },
    AsiStation {
        code: "TPAS",
        lat_deg: 53.994,
        lon_deg: -100.941,
        alt_m: Some(240.59),
    },
    AsiStation {
        code: "WHIT",
        lat_deg: 61.010,
        lon_deg: -135.223,
        alt_m: None,
    },
    AsiStation {
        code: "YKNF",
        lat_deg: 62.520,
        lon_deg: -114.313,
        alt_m: Some(179.14),
    },
];

pub fn station(code: &str) -> Option<&'static AsiStation> {
    STATIONS.iter().find(|s| s.code.eq_ignore_ascii_case(code))
}

pub fn component_name(comp: u32) -> String {
    format!("themis_asi_px_{comp:04}")
}

pub fn declared_fields(names: &[String], tau: f64) -> Vec<FieldConfig> {
    let Some(channel_ref) = crate::force::force_id_of("em") else {
        return Vec::new();
    };
    let Some(kernel) = crate::force::kernel_id_for_force(channel_ref) else {
        return Vec::new();
    };
    names
        .iter()
        .enumerate()
        .map(|(comp, _)| {
            let name = component_name(comp as u32);
            FieldConfig {
                key: name.clone(),
                name,
                band_id: None,
                kernel,
                channel_ref: Some(channel_ref),
                tau,
                absorption: 0.0,
                advection: 0.0,
                unit: UNIT.to_string(),
                freq: crate::archivar::spectral::SPECTRAL_NO_BAND,
                bin_width: crate::archivar::spectral::SPECTRAL_NO_BAND,
                fold: None,
                aperture: crate::archivar::Aperture::None,
            }
        })
        .collect()
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let pixels = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let per = 8 + pixels.checked_mul(PIXEL_BYTES)?;
    if bytes.len() != HEADER_BYTES + count.checked_mul(per)? {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count.saturating_mul(pixels));
    for _ in 0..count {
        let epoch = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        if !epoch.is_finite() {
            return None;
        }
        for j in 0..pixels {
            let at = off + 8 + j * PIXEL_BYTES;
            let value = u16::from_le_bytes(bytes.get(at..at + PIXEL_BYTES)?.try_into().ok()?);
            out.push((epoch, value as f64, j as u32));
        }
        off += per;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_bin(frames: &[(f64, Vec<u16>)], pixels: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + frames.len() * (8 + pixels * PIXEL_BYTES));
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(frames.len() as u32).to_le_bytes());
        out.extend_from_slice(&(pixels as u32).to_le_bytes());
        for (epoch, values) in frames {
            out.extend_from_slice(&epoch.to_le_bytes());
            for v in values {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn bin_roundtrip_flattens_each_frame_to_pixel_rows() {
        let frames = vec![
            (1.5e9, vec![10u16, 20, 30]),
            (1.5e9 + 1.0, vec![40u16, 50, 60]),
        ];
        let bytes = write_bin(&frames, 3);
        let series = parse_bin(&bytes).expect("series parses");
        assert_eq!(
            series,
            vec![
                (1.5e9, 10.0, 0),
                (1.5e9, 20.0, 1),
                (1.5e9, 30.0, 2),
                (1.5e9 + 1.0, 40.0, 0),
                (1.5e9 + 1.0, 50.0, 1),
                (1.5e9 + 1.0, 60.0, 2),
            ]
        );
    }

    #[test]
    fn parse_rejects_foreign_truncated_and_non_finite() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(&MAGIC).is_none());
        let bytes = write_bin(&[(1.0, vec![1u16, 2])], 2);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let nan = write_bin(&[(f64::NAN, vec![1u16, 2])], 2);
        assert!(parse_bin(&nan).is_none());
    }

    #[test]
    fn declared_fields_carry_pixel_names_em_relative_and_no_band() {
        let names: Vec<String> = (0..2).map(component_name).collect();
        let fields = declared_fields(&names, 60.0);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "themis_asi_px_0000");
        assert_eq!(fields[1].name, "themis_asi_px_0001");
        assert_eq!(fields[0].unit, UNIT);
        assert_eq!(fields[0].tau, 60.0);
        assert_eq!(fields[0].freq, crate::archivar::spectral::SPECTRAL_NO_BAND);
        assert_eq!(
            fields[0].bin_width,
            crate::archivar::spectral::SPECTRAL_NO_BAND
        );
        assert_eq!(fields[0].aperture, crate::archivar::Aperture::None);
    }

    #[test]
    fn station_table_carries_measured_ellipsoidal_height_and_refuses_pending() {
        let gill = station("gill").expect("gill is measured");
        assert_eq!(gill.code, "GILL");
        assert_eq!(gill.alt_m, Some(103.12));
        let rank = station("RANK").expect("rank is measured");
        assert_eq!(rank.alt_m, Some(-17.15));
        let ekat = station("EKAT").expect("ekat is measured");
        assert_eq!(ekat.lon_deg, -110.6703);
        assert_eq!(ekat.alt_m, Some(430.77));
        assert_eq!(station("ATHA").and_then(|s| s.alt_m), Some(549.82));
        assert_eq!(station("PGEO").and_then(|s| s.alt_m), Some(756.26));
        assert_eq!(station("MCGR").and_then(|s| s.alt_m), Some(102.57));
        assert_eq!(station("GAKO").and_then(|s| s.alt_m), Some(589.72));
        assert_eq!(station("whit").and_then(|s| s.alt_m), None);
        assert_eq!(station("gbay").and_then(|s| s.alt_m), None);
        assert!(station("zzzz").is_none());
    }
}
