pub const MAGIC: [u8; 4] = *b"2MRS";
pub const RECORD_BYTES: usize = 32;
pub const FIELD_COUNT: usize = 4;

pub struct TwomrsRow {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub cz_km_s: f64,
    pub e_cz_km_s: f64,
}

pub fn record(
    ra: Option<f64>,
    dec: Option<f64>,
    cz: Option<f64>,
    e_cz: Option<f64>,
) -> Option<TwomrsRow> {
    let ra = ra?;
    let dec = dec?;
    let cz = cz?;
    let e_cz = e_cz?;
    if !ra.is_finite() || !dec.is_finite() || !cz.is_finite() || !e_cz.is_finite() {
        return None;
    }
    if !(0.0..360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
        return None;
    }
    if e_cz < 0.0 {
        return None;
    }
    Some(TwomrsRow {
        ra_deg: ra,
        dec_deg: dec,
        cz_km_s: cz,
        e_cz_km_s: e_cz,
    })
}

pub fn row_record(row: &TwomrsRow) -> [f64; 4] {
    [row.ra_deg, row.dec_deg, row.cz_km_s, row.e_cz_km_s]
}

pub fn write_bin(records: &[[f64; 4]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn read_bin(data: &[u8]) -> Option<Vec<[f64; 4]>> {
    if data.len() < 8 || &data[0..4] != &MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * RECORD_BYTES;
        let mut r = [0.0f64; 4];
        for k in 0..FIELD_COUNT {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_gates() {
        let ok = record(Some(183.25), Some(-2.5), Some(9372.0), Some(14.0))
            .expect("a plausible galaxy row builds");
        assert_eq!(ok.ra_deg, 183.25);
        assert_eq!(ok.cz_km_s, 9372.0);
        assert!(record(None, Some(-2.5), Some(9372.0), Some(14.0)).is_none());
        assert!(record(Some(183.25), Some(-2.5), None, Some(14.0)).is_none());
        assert!(record(Some(183.25), Some(-2.5), Some(9372.0), None).is_none());
        assert!(record(Some(183.25), Some(-2.5), Some(f64::NAN), Some(14.0)).is_none());
        assert!(record(Some(183.25), Some(-95.0), Some(9372.0), Some(14.0)).is_none());
        assert!(record(Some(360.5), Some(-2.5), Some(9372.0), Some(14.0)).is_none());
        assert!(record(Some(183.25), Some(-2.5), Some(9372.0), Some(-1.0)).is_none());
        let zero_cz = record(Some(183.25), Some(-2.5), Some(0.0), Some(14.0))
            .expect("cz 0 is a real value, never a sentinel");
        assert_eq!(zero_cz.cz_km_s, 0.0);
    }

    #[test]
    fn test_bin_roundtrip() {
        let records = vec![
            [183.25, -2.5, 9372.0, 14.0],
            [359.995, 89.14652, -300.0, 12.0],
            [0.00354, -89.33452, 51864.0, 999.0],
        ];
        let bin = write_bin(&records);
        assert_eq!(bin.len(), 8 + records.len() * RECORD_BYTES);
        assert_eq!(&bin[0..4], b"2MRS");
        let parsed = read_bin(&bin).expect("roundtrip parses");
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            for k in 0..FIELD_COUNT {
                assert_eq!(a[k], b[k]);
            }
        }
        assert!(read_bin(&bin[..7]).is_none());
        assert!(read_bin(&bin[..9]).is_none());
        let mut wrong = bin.clone();
        wrong[0] = b'X';
        assert!(read_bin(&wrong).is_none());
        let mut short = bin.clone();
        let n = u32::from_le_bytes(short[4..8].try_into().unwrap());
        short[4..8].copy_from_slice(&(n + 1).to_le_bytes());
        assert!(read_bin(&short).is_none());
    }
}
