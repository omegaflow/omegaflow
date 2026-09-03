pub const MAGIC: [u8; 4] = *b"EXCL";

pub const NAME_LEN: usize = 32;
pub const RECORD_BYTES: usize = NAME_LEN + 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExcludeKind {
    Variable = 0,
    Exoplanet = 1,
}

impl ExcludeKind {
    pub fn code(&self) -> u8 {
        *self as u8
    }
    pub fn from_code(c: u8) -> Option<ExcludeKind> {
        match c {
            0 => Some(ExcludeKind::Variable),
            1 => Some(ExcludeKind::Exoplanet),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExcludeRow {
    pub name: [u8; NAME_LEN],
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub kind: ExcludeKind,
}

pub fn write_bin(rows: &[ExcludeRow]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(8 + rows.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for r in rows {
        if !r.ra_deg.is_finite() || !r.dec_deg.is_finite() {
            return None;
        }
        buf.extend_from_slice(&r.name);
        buf.extend_from_slice(&r.ra_deg.to_le_bytes());
        buf.extend_from_slice(&r.dec_deg.to_le_bytes());
        buf.push(r.kind.code());
        buf.resize(buf.len() + 7, 0);
    }
    Some(buf)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<ExcludeRow>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let name: [u8; NAME_LEN] = bytes.get(off..off + NAME_LEN)?.try_into().ok()?;
        off += NAME_LEN;
        let ra = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let dec = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let kind = ExcludeKind::from_code(*bytes.get(off)?)?;
        off += RECORD_BYTES - (NAME_LEN + 16);
        if !ra.is_finite() || !dec.is_finite() {
            return None;
        }
        out.push(ExcludeRow {
            name,
            ra_deg: ra,
            dec_deg: dec,
            kind,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclude_roundtrip() {
        let rows = vec![
            ExcludeRow {
                name: {
                    let mut n = [0u8; NAME_LEN];
                    n[..8].copy_from_slice(b"ASASSN-V");
                    n
                },
                ra_deg: 313.258,
                dec_deg: 38.5889,
                kind: ExcludeKind::Variable,
            },
            ExcludeRow {
                name: {
                    let mut n = [0u8; NAME_LEN];
                    n[..4].copy_from_slice(b"Kepl");
                    n
                },
                ra_deg: 346.6263919,
                dec_deg: -5.0434618,
                kind: ExcludeKind::Exoplanet,
            },
        ];
        let bytes = write_bin(&rows).expect("write");
        let parsed = parse_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].kind, ExcludeKind::Variable);
        assert_eq!(parsed[1].kind, ExcludeKind::Exoplanet);
        assert_eq!(parsed[0].ra_deg, 313.258);
        assert_eq!(&parsed[1].name[..4], b"Kepl");
    }

    #[test]
    fn exclude_rejects_bad_magic() {
        assert!(parse_bin(b"XXXX00000000").is_none());
    }

    #[test]
    fn exclude_rejects_bad_kind() {
        let mut bytes = write_bin(&[ExcludeRow {
            name: [0u8; NAME_LEN],
            ra_deg: 1.0,
            dec_deg: 2.0,
            kind: ExcludeKind::Variable,
        }])
        .unwrap();
        let last = 8 + NAME_LEN + 16;
        bytes[last] = 9;
        assert!(parse_bin(&bytes).is_none());
    }
}
