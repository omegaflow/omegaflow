pub const MAGIC_BGR: [u8; 4] = *b"BGR1";
pub const MAGIC_NRS: [u8; 4] = *b"NRS1";
pub const MAGIC_SDARN: [u8; 4] = *b"SDN1";
pub const MAGIC_ARGO: [u8; 4] = *b"ARG1";
pub const MAGIC_FDSN: [u8; 4] = *b"FDS1";
pub const MAGIC_GIC: [u8; 4] = *b"GIC1";
pub const MAGIC_IGETS: [u8; 4] = *b"IGT1";
pub const MAGIC_GBCO: [u8; 4] = *b"GBCO";
pub const MAGIC_ISSLIS: [u8; 4] = *b"ISL1";

pub const REC_BYTES: usize = 60;
pub const GBCO_REC_BYTES: usize = 24;

pub const COMP_BGR_AZIM: u32 = 1;
pub const COMP_BGR_VAPP: u32 = 2;
pub const COMP_BGR_RMS: u32 = 3;
pub const COMP_BGR_FREQ: u32 = 4;
pub const COMP_BGR_MAX: u32 = 4;

pub const COMP_NRS_PSD: u32 = 1;

pub const COMP_SDARN_V: u32 = 1;

pub const COMP_FDSN_BHZ: u32 = 1;
pub const COMP_FDSN_MAX: u32 = 1;

pub const COMP_GIC_A: u32 = 1;
pub const COMP_GIC_MAX: u32 = 1;

pub const COMP_IGETS_G: u32 = 1;
pub const COMP_IGETS_MAX: u32 = 1;

pub const COMP_ISSLIS_FLASH_RAD: u32 = 1;
pub const COMP_ISSLIS_MAX: u32 = 1;

pub const COMP_ARGO_DOXY: u32 = 1;
pub const COMP_ARGO_NITRATE: u32 = 2;
pub const COMP_ARGO_CHLA: u32 = 3;
pub const COMP_ARGO_BBP700: u32 = 4;
pub const COMP_ARGO_PH_TOTAL: u32 = 5;
pub const COMP_ARGO_MAX: u32 = 5;

pub struct GeoRec {
    pub t: f64,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub val: f64,
    pub comp: u32,
}

pub struct GbcoRec {
    pub lat: f64,
    pub lon: f64,
    pub elev: f64,
}

pub fn magic_of(format: &str) -> Option<[u8; 4]> {
    match format {
        "bgr_infrasound" => Some(MAGIC_BGR),
        "noaa_nrs_psd" => Some(MAGIC_NRS),
        "superdarn_fitacf" => Some(MAGIC_SDARN),
        "argo_bgc" => Some(MAGIC_ARGO),
        "fdsn_waveform" => Some(MAGIC_FDSN),
        "fmi_gic" => Some(MAGIC_GIC),
        "igets" => Some(MAGIC_IGETS),
        "iss_lis" => Some(MAGIC_ISSLIS),
        _ => None,
    }
}

pub fn comp_max(format: &str) -> Option<u32> {
    match format {
        "bgr_infrasound" => Some(COMP_BGR_MAX),
        "noaa_nrs_psd" => Some(COMP_NRS_PSD),
        "superdarn_fitacf" => Some(COMP_SDARN_V),
        "argo_bgc" => Some(COMP_ARGO_MAX),
        "fdsn_waveform" => Some(COMP_FDSN_MAX),
        "fmi_gic" => Some(COMP_GIC_MAX),
        "igets" => Some(COMP_IGETS_MAX),
        "iss_lis" => Some(COMP_ISSLIS_MAX),
        _ => None,
    }
}

pub fn write_bin(magic: [u8; 4], records: &[GeoRec]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * REC_BYTES);
    buf.extend_from_slice(&magic);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.t.to_le_bytes());
        buf.extend_from_slice(&r.lat.to_le_bytes());
        buf.extend_from_slice(&r.lon.to_le_bytes());
        buf.extend_from_slice(&r.alt.to_le_bytes());
        buf.extend_from_slice(&r.freq.to_le_bytes());
        buf.extend_from_slice(&r.bin_width.to_le_bytes());
        buf.extend_from_slice(&r.val.to_le_bytes());
        buf.extend_from_slice(&r.comp.to_le_bytes());
    }
    buf
}

pub fn parse_bin(magic: [u8; 4], bytes: &[u8]) -> Option<Vec<GeoRec>> {
    if bytes.len() < 8 || bytes[0..4] != magic {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let t = f64_of(off)?;
        off += 8;
        let lat = f64_of(off)?;
        off += 8;
        let lon = f64_of(off)?;
        off += 8;
        let alt = f64_of(off)?;
        off += 8;
        let freq = f64_of(off)?;
        off += 8;
        let bin_width = f64_of(off)?;
        off += 8;
        let val = f64_of(off)?;
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        out.push(GeoRec {
            t,
            lat,
            lon,
            alt,
            freq,
            bin_width,
            val,
            comp,
        });
    }
    Some(out)
}

pub fn write_gbco(records: &[GbcoRec]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * GBCO_REC_BYTES);
    buf.extend_from_slice(&MAGIC_GBCO);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        buf.extend_from_slice(&r.lat.to_le_bytes());
        buf.extend_from_slice(&r.lon.to_le_bytes());
        buf.extend_from_slice(&r.elev.to_le_bytes());
    }
    buf
}

pub fn parse_gbco(bytes: &[u8]) -> Option<Vec<GbcoRec>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC_GBCO {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * GBCO_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_of = |off: usize| {
            bytes
                .get(off..off + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let lat = f64_of(off)?;
        off += 8;
        let lon = f64_of(off)?;
        off += 8;
        let elev = f64_of(off)?;
        off += 8;
        if !lat.is_finite() || !lon.is_finite() || !elev.is_finite() {
            return None;
        }
        out.push(GbcoRec { lat, lon, elev });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = vec![
            GeoRec {
                t: 123456789.0,
                lat: -67.6,
                lon: 62.87,
                alt: 60.0,
                freq: 0.0,
                bin_width: 0.0,
                val: 30.45786,
                comp: COMP_BGR_AZIM,
            },
            GeoRec {
                t: 123456849.0,
                lat: -67.6,
                lon: 62.87,
                alt: 60.0,
                freq: 0.0,
                bin_width: 0.0,
                val: 355.75248,
                comp: COMP_BGR_VAPP,
            },
        ];
        let bytes = write_bin(MAGIC_BGR, &records);
        let parsed = parse_bin(MAGIC_BGR, &bytes).unwrap();
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            assert_eq!(a.t, b.t);
            assert_eq!(a.lat, b.lat);
            assert_eq!(a.lon, b.lon);
            assert_eq!(a.alt, b.alt);
            assert_eq!(a.freq, b.freq);
            assert_eq!(a.val, b.val);
            assert_eq!(a.comp, b.comp);
        }
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(MAGIC_BGR, b"X").is_none());
        assert!(parse_bin(MAGIC_BGR, b"BGR1abc").is_none());
        assert!(parse_bin(MAGIC_BGR, b"NRS1").is_none());
    }

    #[test]
    fn gbco_roundtrip() {
        let records = vec![
            GbcoRec {
                lat: -67.6,
                lon: 62.87,
                elev: 30.0,
            },
            GbcoRec {
                lat: 72.49,
                lon: -156.6,
                elev: -833.0,
            },
        ];
        let bytes = write_gbco(&records);
        let parsed = parse_gbco(&bytes).unwrap();
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            assert_eq!(a.lat, b.lat);
            assert_eq!(a.lon, b.lon);
            assert_eq!(a.elev, b.elev);
        }
    }

    #[test]
    fn gbco_rejects_foreign_bytes() {
        assert!(parse_gbco(b"X").is_none());
        assert!(parse_gbco(b"GBCOabc").is_none());
        assert!(parse_gbco(b"BGR1").is_none());
    }

    #[test]
    fn rejects_oversized_declared_count_and_non_finite_elevation() {
        let records = vec![GbcoRec {
            lat: 72.49,
            lon: -156.6,
            elev: -833.0,
        }];
        let bytes = write_gbco(&records);
        assert!(parse_gbco(&bytes).is_some());
        let mut short = bytes[..bytes.len() - 1].to_vec();
        let n = u32::from_le_bytes(short[4..8].try_into().unwrap());
        short[4..8].copy_from_slice(&(n + 1).to_le_bytes());
        assert!(parse_gbco(&short).is_none());
        let nan = write_gbco(&[GbcoRec {
            lat: 72.49,
            lon: -156.6,
            elev: f64::NAN,
        }]);
        assert!(parse_gbco(&nan).is_none());
    }
}
