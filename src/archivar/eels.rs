use super::rixs::SpinOscillator;

pub const EELS_MAGIC: [u8; 4] = *b"EELS";
pub const EELS_VERSION: u8 = 1;

#[derive(Clone, Debug)]
pub struct EelsProfile {
    pub profile_index: u32,
    pub oscillators: Vec<SpinOscillator>,
}

#[derive(Clone, Debug)]
pub struct EelsBin {
    pub lab: Option<(f64, f64, f64)>,
    pub profiles: Vec<EelsProfile>,
}

pub fn parse_eels_bin(bytes: &[u8]) -> Option<EelsBin> {
    if bytes.len() < 9 || bytes[0..4] != EELS_MAGIC || bytes[4] != EELS_VERSION {
        return None;
    }
    let n_profiles = u32::from_le_bytes(bytes[5..9].try_into().ok()?) as usize;
    let mut pos = 9usize;
    let lat = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
    pos += 8;
    let lon = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
    pos += 8;
    let alt = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
    pos += 8;
    let present = *bytes.get(pos)? != 0;
    pos += 1;
    let lab = if present && lat.is_finite() && lon.is_finite() && alt.is_finite() {
        Some((lat, lon, alt))
    } else {
        None
    };
    let mut profiles = Vec::with_capacity(n_profiles);
    for _ in 0..n_profiles {
        let profile_index = u32::from_le_bytes(bytes.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;
        let n_osc = u32::from_le_bytes(bytes.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        let mut oscillators = Vec::with_capacity(n_osc);
        for _ in 0..n_osc {
            let freq_hz = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            let bin_width_hz = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            let val = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            let err = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
            oscillators.push(SpinOscillator {
                freq_hz,
                bin_width_hz,
                val,
                err,
            });
        }
        profiles.push(EelsProfile {
            profile_index,
            oscillators,
        });
    }
    Some(EelsBin { lab, profiles })
}

pub fn encode_eels_bin(bin: &EelsBin) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&EELS_MAGIC);
    out.push(EELS_VERSION);
    out.extend_from_slice(&(bin.profiles.len() as u32).to_le_bytes());
    match bin.lab {
        Some((lat, lon, alt)) => {
            out.extend_from_slice(&lat.to_le_bytes());
            out.extend_from_slice(&lon.to_le_bytes());
            out.extend_from_slice(&alt.to_le_bytes());
            out.push(1u8);
        }
        None => {
            out.extend_from_slice(&0.0f64.to_le_bytes());
            out.extend_from_slice(&0.0f64.to_le_bytes());
            out.extend_from_slice(&0.0f64.to_le_bytes());
            out.push(0u8);
        }
    }
    for p in &bin.profiles {
        out.extend_from_slice(&p.profile_index.to_le_bytes());
        out.extend_from_slice(&(p.oscillators.len() as u32).to_le_bytes());
        for o in &p.oscillators {
            out.extend_from_slice(&o.freq_hz.to_le_bytes());
            out.extend_from_slice(&o.bin_width_hz.to_le_bytes());
            out.extend_from_slice(&o.val.to_le_bytes());
            out.extend_from_slice(&o.err.to_le_bytes());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eels_bin_roundtrip() {
        let bin = EelsBin {
            lab: Some((45.206, 5.688, 200.0)),
            profiles: vec![
                EelsProfile {
                    profile_index: 0,
                    oscillators: vec![SpinOscillator {
                        freq_hz: 1.0e2,
                        bin_width_hz: 1.0,
                        val: 3.0,
                        err: 0.0,
                    }],
                },
                EelsProfile {
                    profile_index: 7,
                    oscillators: Vec::new(),
                },
            ],
        };
        let bytes = encode_eels_bin(&bin);
        assert_eq!(bytes[0..4], EELS_MAGIC);
        assert_eq!(bytes[4], EELS_VERSION);
        let back = parse_eels_bin(&bytes).unwrap();
        assert_eq!(back.lab, bin.lab);
        assert_eq!(back.profiles.len(), 2);
        assert_eq!(back.profiles[0].profile_index, 0);
        assert_eq!(back.profiles[0].oscillators.len(), 1);
        assert!((back.profiles[0].oscillators[0].val - 3.0).abs() < 1e-12);
        assert_eq!(back.profiles[1].profile_index, 7);
        assert!(back.profiles[1].oscillators.is_empty());
    }

    #[test]
    fn eels_bin_rejects_bad_magic() {
        assert!(parse_eels_bin(&[0, 0, 0, 0, EELS_VERSION, 0, 0, 0, 0]).is_none());
    }

    #[test]
    fn eels_bin_absent_anchor_is_none() {
        let back = parse_eels_bin(&encode_eels_bin(&EelsBin {
            lab: None,
            profiles: Vec::new(),
        }))
        .unwrap();
        assert_eq!(back.lab, None);
        assert!(back.profiles.is_empty());
    }
}
