pub const MAGIC: [u8; 4] = *b"SEM1";

pub const COMP_FLUX_26_34: u32 = 0;
pub const COMP_FLUX_0_1_50: u32 = 1;
pub const COMP_CH1: u32 = 2;
pub const COMP_CH2: u32 = 3;
pub const COMP_CH3: u32 = 4;

pub const SECONDS_PER_DAY: f64 = 86400.0;

pub type Record = (i64, f64, f64, f64, f64, f64);

pub const RECORD_BYTES: usize = 8 + 5 * 8;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_FLUX_26_34 => Some("soho_sem_flux_26_34_nm"),
        COMP_FLUX_0_1_50 => Some("soho_sem_flux_0_1_50_nm"),
        COMP_CH1 => Some("soho_sem_ch1_count_rate"),
        COMP_CH2 => Some("soho_sem_ch2_count_rate"),
        COMP_CH3 => Some("soho_sem_ch3_count_rate"),
        _ => None,
    }
}

pub fn write_bin(records: &[Record]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (d, ch1, ch2, ch3, f2634, f0150) in records {
        buf.extend_from_slice(&d.to_le_bytes());
        buf.extend_from_slice(&ch1.to_le_bytes());
        buf.extend_from_slice(&ch2.to_le_bytes());
        buf.extend_from_slice(&ch3.to_le_bytes());
        buf.extend_from_slice(&f2634.to_le_bytes());
        buf.extend_from_slice(&f0150.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<Record>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let d = i64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let mut v = [0.0f64; 5];
        for slot in &mut v {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        out.push((d, v[0], v[1], v[2], v[3], v[4]));
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let lsk = crate::archivar::embedded_lsk()?;
    let records = parse_bin(bytes)?;
    let mut out = Vec::with_capacity(records.len() * 5);
    for (day, ch1, ch2, ch3, f2634, f0150) in records {
        let Some(t) = lsk.unix_to_tdb(day as f64 * SECONDS_PER_DAY) else {
            continue;
        };
        out.push((t, f2634, COMP_FLUX_26_34));
        out.push((t, f0150, COMP_FLUX_0_1_50));
        out.push((t, ch1, COMP_CH1));
        out.push((t, ch2, COMP_CH2));
        out.push((t, ch3, COMP_CH3));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Record> {
        vec![
            (9496, 162.07, 11309.39, 130.91, 1.43652e10, 2.92961e10),
            (9497, 160.91, 10584.87, 130.25, 1.42298e10, 2.88664e10),
        ]
    }

    #[test]
    fn roundtrip() {
        let records = sample();
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"SEM1\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"EIT1abcdefghijklmnop").is_none());
    }

    #[test]
    fn component_names_match_the_register_fields() {
        assert_eq!(
            component_name(COMP_FLUX_26_34),
            Some("soho_sem_flux_26_34_nm")
        );
        assert_eq!(
            component_name(COMP_FLUX_0_1_50),
            Some("soho_sem_flux_0_1_50_nm")
        );
        assert_eq!(component_name(COMP_CH1), Some("soho_sem_ch1_count_rate"));
        assert_eq!(component_name(COMP_CH2), Some("soho_sem_ch2_count_rate"));
        assert_eq!(component_name(COMP_CH3), Some("soho_sem_ch3_count_rate"));
        assert_eq!(component_name(5), None);
    }
}
