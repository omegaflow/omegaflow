pub const MAGIC: [u8; 4] = *b"EUV1";

pub const COMP_171: u32 = 0;
pub const COMP_195: u32 = 1;
pub const COMP_284: u32 = 2;
pub const COMP_304: u32 = 3;

pub fn wavelength(comp: u32) -> Option<u32> {
    match comp {
        COMP_171 => Some(171),
        COMP_195 => Some(195),
        COMP_284 => Some(284),
        COMP_304 => Some(304),
        _ => None,
    }
}

pub fn comp_of_wavelength(wavelnth: u32) -> Option<u32> {
    match wavelnth {
        171 => Some(COMP_171),
        195 => Some(COMP_195),
        284 => Some(COMP_284),
        304 => Some(COMP_304),
        _ => None,
    }
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_171 => Some("stereo_euvi_171_mean"),
        COMP_195 => Some("stereo_euvi_195_mean"),
        COMP_284 => Some("stereo_euvi_284_mean"),
        COMP_304 => Some("stereo_euvi_304_mean"),
        _ => None,
    }
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        wavelength(comp)?;
        if !t.is_finite() || !val.is_finite() {
            return None;
        }
        out.push((t, val, comp));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + records.len() * 20);
        buf.extend_from_slice(&MAGIC);
        buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
        for (t, val, comp) in records {
            buf.extend_from_slice(&t.to_le_bytes());
            buf.extend_from_slice(&val.to_le_bytes());
            buf.extend_from_slice(&comp.to_le_bytes());
        }
        buf
    }

    #[test]
    fn roundtrip() {
        let records = vec![(10.0, 1200.5, COMP_195), (20.0, 900.0, COMP_171)];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"EUV1\x02\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"EIT1abcdefghijklmnop").is_none());
    }

    #[test]
    fn rejects_unknown_band() {
        let bytes = bin(&[(10.0, 1.0, 4)]);
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn wavelength_and_component_names_are_bijective() {
        for comp in [COMP_171, COMP_195, COMP_284, COMP_304] {
            let w = wavelength(comp).unwrap();
            assert_eq!(comp_of_wavelength(w), Some(comp));
            assert!(component_name(comp).unwrap().starts_with("stereo_euvi_"));
        }
        assert_eq!(comp_of_wavelength(193), None);
        assert_eq!(component_name(7), None);
    }
}
