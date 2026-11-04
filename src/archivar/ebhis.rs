pub const MAGIC: [u8; 4] = *b"EBH1";
pub const N_CHANNELS: usize = 945;
pub const HEADER_BYTES: usize = 8;
pub const MASK_BYTES: usize = N_CHANNELS.div_ceil(8);
pub const SERIES_FIXED_BYTES: usize = 24 + MASK_BYTES;

#[derive(Clone, Debug, PartialEq)]
pub struct EbhisSeries {
    pub hpx_index: u64,
    pub glon: f64,
    pub glat: f64,
    pub channels: Vec<Option<f64>>,
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<EbhisSeries>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let hpx_index = u64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let glon = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let glat = f64::from_le_bytes(bytes.get(off + 16..off + 24)?.try_into().ok()?);
        off += 24;
        let mask = bytes.get(off..off + MASK_BYTES)?;
        off += MASK_BYTES;
        if !glon.is_finite() || !glat.is_finite() {
            return None;
        }
        let mut channels = Vec::with_capacity(N_CHANNELS);
        for c in 0..N_CHANNELS {
            if (mask[c / 8] >> (c % 8)) & 1 == 1 {
                let v = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
                if !v.is_finite() {
                    return None;
                }
                off += 8;
                channels.push(Some(v));
            } else {
                channels.push(None);
            }
        }
        out.push(EbhisSeries {
            hpx_index,
            glon,
            glat,
            channels,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(out)
}

pub fn component_name(comp: u32) -> String {
    format!("ebhis_ch_{comp:03}")
}

pub fn component_value(rec: &EbhisSeries, comp: u32) -> Option<f64> {
    rec.channels.get(comp as usize).copied().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_bin(series: &[EbhisSeries]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&(series.len() as u32).to_le_bytes());
        for s in series {
            out.extend_from_slice(&s.hpx_index.to_le_bytes());
            out.extend_from_slice(&s.glon.to_le_bytes());
            out.extend_from_slice(&s.glat.to_le_bytes());
            let mut mask = [0u8; MASK_BYTES];
            for (c, ch) in s.channels.iter().enumerate() {
                if ch.is_some() {
                    mask[c / 8] |= 1 << (c % 8);
                }
            }
            out.extend_from_slice(&mask);
            for v in s.channels.iter().flatten() {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out
    }

    fn sample_channels() -> Vec<Option<f64>> {
        (0..N_CHANNELS)
            .map(|c| match c {
                0 => Some(-12.5),
                7 => Some(0.0),
                16 => Some(41.25),
                944 => Some(3.5),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn roundtrip_reads_the_mask_and_channel_values() {
        let series = vec![
            EbhisSeries {
                hpx_index: 1_000_003,
                glon: 122.25,
                glat: 15.5,
                channels: sample_channels(),
            },
            EbhisSeries {
                hpx_index: 1_000_004,
                glon: 122.75,
                glat: 15.0,
                channels: (0..N_CHANNELS).map(|_| None).collect(),
            },
        ];
        let bin = write_bin(&series);
        assert_eq!(parse_bin(&bin), Some(series));
    }

    #[test]
    fn variable_channel_count_survives_the_roundtrip() {
        let mut channels = vec![None; N_CHANNELS];
        channels[0] = Some(1.0);
        channels[500] = Some(2.0);
        let series = vec![EbhisSeries {
            hpx_index: 7,
            glon: 0.0,
            glat: 0.0,
            channels,
        }];
        let bin = write_bin(&series);
        assert_eq!(bin.len(), HEADER_BYTES + SERIES_FIXED_BYTES + 2 * 8);
        assert_eq!(parse_bin(&bin), Some(series));
    }

    #[test]
    fn an_empty_bin_roundtrips_to_no_records() {
        assert_eq!(parse_bin(&write_bin(&[])), Some(Vec::new()));
    }

    #[test]
    fn a_truncated_bin_is_refused() {
        let series = vec![EbhisSeries {
            hpx_index: 3,
            glon: 1.0,
            glat: 2.0,
            channels: sample_channels(),
        }];
        let bin = write_bin(&series);
        assert!(parse_bin(&bin[..bin.len() - 1]).is_none());
    }

    #[test]
    fn a_foreign_magic_is_refused() {
        let series = vec![EbhisSeries {
            hpx_index: 3,
            glon: 1.0,
            glat: 2.0,
            channels: sample_channels(),
        }];
        let mut bin = write_bin(&series);
        bin[0] = b'X';
        assert!(parse_bin(&bin).is_none());
    }

    #[test]
    fn component_names_and_values_name_the_channel() {
        assert_eq!(component_name(0), "ebhis_ch_000");
        assert_eq!(component_name(7), "ebhis_ch_007");
        assert_eq!(component_name(944), "ebhis_ch_944");
        let series = EbhisSeries {
            hpx_index: 1,
            glon: 0.0,
            glat: 0.0,
            channels: sample_channels(),
        };
        assert_eq!(component_value(&series, 7), Some(0.0));
        assert_eq!(component_value(&series, 1), None);
        assert_eq!(component_value(&series, N_CHANNELS as u32), None);
    }
}
