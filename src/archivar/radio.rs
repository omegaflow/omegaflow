pub const MAGIC: [u8; 4] = *b"RAD1";

pub const RADIO_FREQ_HZ: f64 = 1.4e9;
pub const RADIO_BIN_WIDTH_HZ: f64 = 42.0e6;

pub const RECORD_BYTES: usize = 48;

#[derive(Clone, Copy, Debug)]
pub struct RadioSource {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub plx_mas: f64,
    pub freq: f64,
    pub bin_width: f64,
    pub flux: f64,
}

pub fn write_bin(sources: &[RadioSource]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(8 + sources.len() * RECORD_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(sources.len() as u32).to_le_bytes());
    for s in sources {
        if !s.ra_deg.is_finite()
            || !s.dec_deg.is_finite()
            || !s.plx_mas.is_finite()
            || !s.freq.is_finite()
            || !s.bin_width.is_finite()
            || !s.flux.is_finite()
        {
            return None;
        }
        buf.extend_from_slice(&s.ra_deg.to_le_bytes());
        buf.extend_from_slice(&s.dec_deg.to_le_bytes());
        buf.extend_from_slice(&s.plx_mas.to_le_bytes());
        buf.extend_from_slice(&s.freq.to_le_bytes());
        buf.extend_from_slice(&s.bin_width.to_le_bytes());
        buf.extend_from_slice(&s.flux.to_le_bytes());
    }
    Some(buf)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<RadioSource>> {
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
        let freq = f64_at(&mut off)?;
        let bin_width = f64_at(&mut off)?;
        let flux = f64_at(&mut off)?;
        if !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !plx_mas.is_finite()
            || !freq.is_finite()
            || !bin_width.is_finite()
            || !flux.is_finite()
        {
            return None;
        }
        out.push(RadioSource {
            ra_deg,
            dec_deg,
            plx_mas,
            freq,
            bin_width,
            flux,
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
    fn radio_roundtrip() {
        let srcs = vec![
            RadioSource {
                ra_deg: 45.04291666666666,
                dec_deg: 0.04002777777777777,
                plx_mas: 0.0,
                freq: RADIO_FREQ_HZ,
                bin_width: RADIO_BIN_WIDTH_HZ,
                flux: 2.7,
            },
            RadioSource {
                ra_deg: 314.95191666666665,
                dec_deg: -0.1424722222222222,
                plx_mas: 0.0,
                freq: RADIO_FREQ_HZ,
                bin_width: RADIO_BIN_WIDTH_HZ,
                flux: 399.2,
            },
        ];
        let bytes = write_bin(&srcs).expect("write");
        let parsed = parse_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, srcs[0].ra_deg);
        assert_eq!(parsed[0].flux, srcs[0].flux);
        assert_eq!(parsed[1].freq, srcs[1].freq);
        assert_eq!(parsed[1].bin_width, srcs[1].bin_width);
    }

    #[test]
    fn radio_rejects_bad_magic() {
        assert!(parse_bin(b"XXXX00000000").is_none());
    }
}
