pub const MAGIC: [u8; 4] = *b"TNS1";

pub const TNS_TTL_S: f64 = 604800.0;

pub const NAME_LEN: usize = 32;
pub const RECORD_BYTES: usize = NAME_LEN + 24;

#[derive(Clone, Debug)]
pub struct TnsObject {
    pub name: [u8; NAME_LEN],
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub z: f64,
}

pub fn write_bin(objs: &[TnsObject]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(8 + objs.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(objs.len() as u32).to_le_bytes());
    for o in objs {
        if !o.ra_deg.is_finite() || !o.dec_deg.is_finite() || !o.z.is_finite() {
            return None;
        }
        buf.extend_from_slice(&o.name);
        buf.extend_from_slice(&o.ra_deg.to_le_bytes());
        buf.extend_from_slice(&o.dec_deg.to_le_bytes());
        buf.extend_from_slice(&o.z.to_le_bytes());
    }
    Some(buf)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<TnsObject>> {
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
        let f64_at = |o: &mut usize| -> Option<f64> {
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            Some(v)
        };
        let ra_deg = f64_at(&mut off)?;
        let dec_deg = f64_at(&mut off)?;
        let z = f64_at(&mut off)?;
        if !ra_deg.is_finite() || !dec_deg.is_finite() || !z.is_finite() {
            return None;
        }
        out.push(TnsObject {
            name,
            ra_deg,
            dec_deg,
            z,
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
    fn tns_roundtrip() {
        let objs = vec![
            TnsObject {
                name: {
                    let mut n = [0u8; NAME_LEN];
                    n[..6].copy_from_slice(b"2021rf");
                    n
                },
                ra_deg: 144.353725635,
                dec_deg: 42.9745186268,
                z: 0.027172,
            },
            TnsObject {
                name: {
                    let mut n = [0u8; NAME_LEN];
                    n[..7].copy_from_slice(b"2020kme");
                    n
                },
                ra_deg: 144.3,
                dec_deg: 43.0,
                z: 0.1,
            },
        ];
        let bytes = write_bin(&objs).expect("write");
        let parsed = parse_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, objs[0].ra_deg);
        assert_eq!(parsed[0].z, objs[0].z);
        assert_eq!(&parsed[1].name[..7], b"2020kme");
    }

    #[test]
    fn tns_rejects_bad_magic() {
        assert!(parse_bin(b"XXXX00000000").is_none());
    }
}
