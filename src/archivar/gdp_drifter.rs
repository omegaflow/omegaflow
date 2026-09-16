pub const MAGIC: [u8; 4] = *b"GDPT";
pub const HEADER_BYTES: usize = 8;
pub const REC_BYTES: usize = 40;

pub const COMP_SST: u32 = 1;

#[derive(Clone, Debug)]
pub struct DrifterRecord {
    pub id: u64,
    pub time: f64,
    pub lon: f64,
    pub lat: f64,
    pub sst: Option<f64>,
}

pub fn write_bin(records: &[DrifterRecord]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        if !r.time.is_finite()
            || !r.lon.is_finite()
            || !r.lat.is_finite()
            || !(-90.0..=90.0).contains(&r.lat)
            || !(-180.0..=180.0).contains(&r.lon)
        {
            return None;
        }
        if let Some(sst) = r.sst
            && (!sst.is_finite() || sst <= 0.0)
        {
            return None;
        }
        let mut rec = [0u8; REC_BYTES];
        rec[0..8].copy_from_slice(&r.id.to_le_bytes());
        rec[8..16].copy_from_slice(&r.time.to_le_bytes());
        rec[16..24].copy_from_slice(&r.lon.to_le_bytes());
        rec[24..32].copy_from_slice(&r.lat.to_le_bytes());
        match r.sst {
            Some(v) => {
                rec[32..36].copy_from_slice(&(v as f32).to_le_bytes());
                rec[36] = 1;
            }
            None => {
                rec[32..36].copy_from_slice(&0.0f32.to_le_bytes());
                rec[36] = 0;
            }
        }
        out.extend_from_slice(&rec);
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<DrifterRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = HEADER_BYTES;
    for _ in 0..n {
        let rec = bytes.get(off..off + REC_BYTES)?;
        off += REC_BYTES;
        let time = f64::from_le_bytes(rec[8..16].try_into().ok()?);
        let lon = f64::from_le_bytes(rec[16..24].try_into().ok()?);
        let lat = f64::from_le_bytes(rec[24..32].try_into().ok()?);
        if !time.is_finite()
            || !lon.is_finite()
            || !lat.is_finite()
            || !(-90.0..=90.0).contains(&lat)
            || !(-180.0..=180.0).contains(&lon)
        {
            return None;
        }
        let sst = f32::from_le_bytes(rec[32..36].try_into().ok()?);
        let sst = match rec[36] {
            1 if sst.is_finite() && sst > 0.0 => Some(sst as f64),
            _ => None,
        };
        out.push(DrifterRecord {
            id: u64::from_le_bytes(rec[0..8].try_into().ok()?),
            time,
            lon,
            lat,
            sst,
        });
    }
    Some(out)
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_SST => Some("gdp_drifter_sst_k"),
        _ => None,
    }
}

pub fn to_geo(records: &[DrifterRecord]) -> Vec<crate::geo::GeoRec> {
    records
        .iter()
        .filter_map(|r| {
            let sst = r.sst?;
            Some(crate::geo::GeoRec {
                t: r.time,
                lat: r.lat,
                lon: r.lon,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: sst,
                comp: COMP_SST,
                station: 0,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record() -> DrifterRecord {
        DrifterRecord {
            id: 12345,
            time: 729_777_632.227_242_1,
            lon: -45.5,
            lat: 30.25,
            sst: Some(300.15),
        }
    }

    #[test]
    fn roundtrip_preserves_sst_absence() {
        let present = record();
        let absent = DrifterRecord {
            sst: None,
            ..record()
        };
        let bytes = write_bin(&[present.clone(), absent.clone()]).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].id, present.id);
        assert_eq!(parsed[0].lon, present.lon);
        assert_eq!(parsed[0].lat, present.lat);
        assert_eq!(parsed[0].sst, Some(300.15f32 as f64));
        assert!(parsed[1].sst.is_none());
    }

    #[test]
    fn refuses_foreign_magic_and_truncated() {
        assert!(parse_bin(b"GDPT").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let bytes = write_bin(&[record()]).unwrap();
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn refuses_out_of_range_position() {
        let bad = DrifterRecord {
            lat: 91.0,
            ..record()
        };
        assert!(write_bin(&[bad]).is_none());
    }

    #[test]
    fn to_geo_skips_absent_sst_and_carries_position() {
        let geo = to_geo(&[
            record(),
            DrifterRecord {
                sst: None,
                ..record()
            },
        ]);
        assert_eq!(geo.len(), 1);
        assert_eq!(geo[0].lat, 30.25);
        assert_eq!(geo[0].lon, -45.5);
        assert!((geo[0].val - 300.15).abs() < 1e-9);
        assert_eq!(geo[0].comp, COMP_SST);
    }

    #[test]
    fn component_name_maps_sst() {
        assert_eq!(component_name(COMP_SST), Some("gdp_drifter_sst_k"));
        assert_eq!(component_name(0), None);
    }
}
