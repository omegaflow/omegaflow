pub const MAGIC: [u8; 4] = *b"EVL1";

pub const COMP_94: u32 = 0;
pub const COMP_131: u32 = 1;
pub const COMP_171: u32 = 3;
pub const COMP_195: u32 = 6;
pub const COMP_211: u32 = 8;
pub const COMP_284: u32 = 10;
pub const COMP_304: u32 = 11;
pub const COMP_335: u32 = 12;
pub const COMP_584: u32 = 23;
pub const COMP_977: u32 = 36;
pub const COMP_1032: u32 = 38;
pub const COMP_DIODE: u32 = 100;

pub fn line_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_94 => Some("94"),
        COMP_131 => Some("131"),
        COMP_171 => Some("171"),
        COMP_195 => Some("195"),
        COMP_211 => Some("211"),
        COMP_284 => Some("284"),
        COMP_304 => Some("304"),
        COMP_335 => Some("335"),
        COMP_584 => Some("584"),
        COMP_977 => Some("977"),
        COMP_1032 => Some("1032"),
        _ => None,
    }
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
        if line_name(comp).is_none() && comp != COMP_DIODE {
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
        let records = vec![(10.0, 2.5e-9, COMP_94), (20.0, 3.0e-9, COMP_1032)];
        let bytes = bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"EVL1abc").is_none());
    }

    #[test]
    fn rejects_unknown_line() {
        let bytes = bin(&[(10.0, 1.0, 2)]);
        assert!(parse_bin(&bytes).is_none());
    }

    #[test]
    fn carries_the_diode_record_the_compiler_writes() {
        let records = vec![(10.0, 2.5e-9, COMP_94), (10.0, 1.0e-2, COMP_DIODE)];
        let bytes = bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed, records);
    }
}
