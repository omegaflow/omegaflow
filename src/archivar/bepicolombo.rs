pub const MAGIC: &[u8; 4] = b"BCPL";

pub fn component_name(comp: u32) -> String {
    format!("bepicolombo_d{}_{:03}", comp >> 8, comp & 0xff)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let mut pos = 4usize;
    let n = u32::from_le_bytes(bytes.get(pos..pos + 4)?.try_into().ok()?) as usize;
    pos += 4;
    let mut out: Vec<(f64, f64, u32)> = Vec::new();
    for _ in 0..n {
        let dtype = *bytes.get(pos)?;
        let channel = *bytes.get(pos + 1)?;
        pos += 2;
        bytes.get(pos..pos + 2)?;
        pos += 2;
        let count = u32::from_le_bytes(bytes.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let comp = (dtype as u32) << 8 | channel as u32;
        for _ in 0..count {
            let t = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            let v = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            if !t.is_finite() || !v.is_finite() {
                return None;
            }
            out.push((t, v, comp));
        }
    }
    if pos != bytes.len() {
        return None;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    type BinSeries = (u8, u8, Vec<(f64, f64)>);

    fn write_bin(series: &[BinSeries]) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&(series.len() as u32).to_le_bytes());
        for (dtype, channel, pairs) in series {
            out.push(*dtype);
            out.push(*channel);
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&(pairs.len() as u32).to_le_bytes());
            for (t, v) in pairs {
                out.extend_from_slice(&t.to_le_bytes());
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    #[test]
    fn bin_roundtrip_flattens_each_series_with_composite_comp() {
        let series = vec![
            (2u8, 0u8, vec![(1.0, 2.0), (3.0, 4.0)]),
            (40u8, 2u8, vec![(5.0, 6.0)]),
        ];
        let bytes = write_bin(&series);
        let rows = parse_bin(&bytes).expect("series parses");
        assert_eq!(
            rows,
            vec![
                (1.0, 2.0, 2u32 << 8),
                (3.0, 4.0, 2u32 << 8),
                (5.0, 6.0, (40u32 << 8) | 2),
            ]
        );
    }

    #[test]
    fn component_name_decodes_dtype_and_channel() {
        assert_eq!(component_name(2u32 << 8), "bepicolombo_d2_000");
        assert_eq!(component_name((40u32 << 8) | 2), "bepicolombo_d40_002");
    }

    #[test]
    fn parse_rejects_foreign_truncated_and_non_finite() {
        assert!(parse_bin(b"").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        assert!(parse_bin(MAGIC).is_none());
        let bytes = write_bin(&[(2u8, 0u8, vec![(1.0, 2.0)])]);
        assert!(parse_bin(&bytes[..bytes.len() - 1]).is_none());
        let nan = write_bin(&[(2u8, 0u8, vec![(f64::NAN, 2.0)])]);
        assert!(parse_bin(&nan).is_none());
    }

    #[test]
    fn parse_rejects_leftover_bytes() {
        let mut bytes = write_bin(&[(2u8, 0u8, vec![(1.0, 2.0)])]);
        bytes.push(0);
        assert!(parse_bin(&bytes).is_none());
    }
}
