pub const MAGIC: [u8; 4] = *b"IR1X";

pub const IR_EXCESS_THRESHOLD_MAG: f64 = -0.5;

pub const RECORD_BYTES: usize = 48;

#[derive(Clone, Copy, Debug)]
pub struct IrSource {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub plx_mas: f64,
    pub w3mag: f64,
    pub w4mag: f64,
    pub excess: f64,
}

impl IrSource {
    pub fn is_excess(&self) -> bool {
        self.excess.is_finite() && self.excess < IR_EXCESS_THRESHOLD_MAG
    }
}

pub fn write_bin(sources: &[IrSource]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(8 + sources.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(sources.len() as u32).to_le_bytes());
    for s in sources {
        if !s.ra_deg.is_finite()
            || !s.dec_deg.is_finite()
            || !s.plx_mas.is_finite()
            || !s.w3mag.is_finite()
            || !s.w4mag.is_finite()
            || !s.excess.is_finite()
        {
            return None;
        }
        buf.extend_from_slice(&s.ra_deg.to_le_bytes());
        buf.extend_from_slice(&s.dec_deg.to_le_bytes());
        buf.extend_from_slice(&s.plx_mas.to_le_bytes());
        buf.extend_from_slice(&s.w3mag.to_le_bytes());
        buf.extend_from_slice(&s.w4mag.to_le_bytes());
        buf.extend_from_slice(&s.excess.to_le_bytes());
    }
    Some(buf)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<IrSource>> {
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
        let f64_at = |o: &mut usize| -> Option<f64> {
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            Some(v)
        };
        let ra_deg = f64_at(&mut off)?;
        let dec_deg = f64_at(&mut off)?;
        let plx_mas = f64_at(&mut off)?;
        let w3mag = f64_at(&mut off)?;
        let w4mag = f64_at(&mut off)?;
        let excess = f64_at(&mut off)?;
        if !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !plx_mas.is_finite()
            || !w3mag.is_finite()
            || !w4mag.is_finite()
            || !excess.is_finite()
        {
            return None;
        }
        out.push(IrSource {
            ra_deg,
            dec_deg,
            plx_mas,
            w3mag,
            w4mag,
            excess,
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
    fn ir_roundtrip() {
        let srcs = vec![
            IrSource {
                ra_deg: 45.04291666666666,
                dec_deg: 0.04002777777777777,
                plx_mas: 0.0,
                w3mag: 8.0,
                w4mag: 7.9,
                excess: 0.1,
            },
            IrSource {
                ra_deg: 314.95,
                dec_deg: -0.14,
                plx_mas: 0.0,
                w3mag: 7.0,
                w4mag: 8.0,
                excess: -1.0,
            },
        ];
        let bytes = write_bin(&srcs).expect("write");
        let parsed = parse_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, srcs[0].ra_deg);
        assert_eq!(parsed[0].excess, srcs[0].excess);
        assert!(!parsed[0].is_excess());
        assert!(parsed[1].is_excess());
    }

    #[test]
    fn ir_rejects_bad_magic() {
        assert!(parse_bin(b"XXXX00000000").is_none());
    }
}
