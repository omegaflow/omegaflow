pub const EV_TO_HZ: f64 = 2.417989242e14;
pub const MEV_TO_HZ: f64 = EV_TO_HZ * 1.0e-3;

pub const SPIN_MAGIC: [u8; 2] = [0xCF, 0x86];
pub const SPIN_VERSION: u8 = 0x02;

#[derive(Clone, Debug)]
pub struct SpinSpectrum {
    pub eloss_ev: Vec<f64>,
    pub weight: Vec<f64>,
    pub err: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct SpinOscillator {
    pub freq_hz: f64,
    pub bin_width_hz: f64,
    pub val: f64,
    pub err: f64,
}

#[derive(Clone, Debug)]
pub struct SpinSpectrumBin {
    pub doping: u8,
    pub q_h: f64,
    pub q_l: f64,
    pub oscillators: Vec<SpinOscillator>,
}

#[derive(Clone, Debug)]
pub struct SpinBin {
    pub lab: Option<(f64, f64, f64)>,
    pub spectra: Vec<SpinSpectrumBin>,
}

pub fn parse_spin_bin(bytes: &[u8]) -> Option<SpinBin> {
    if bytes.len() < 13
        || bytes[0] != SPIN_MAGIC[0]
        || bytes[1] != SPIN_MAGIC[1]
        || bytes[2] != SPIN_VERSION
    {
        return None;
    }
    let n_spectra = u32::from_le_bytes(bytes[3..7].try_into().ok()?) as usize;
    let mut pos = 7usize;
    let lat = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
    pos += 8;
    let lon = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
    pos += 8;
    let alt = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
    pos += 8;
    let present = bytes[pos] != 0;
    pos += 1;
    let lab = if present && lat.is_finite() && lon.is_finite() && alt.is_finite() {
        Some((lat, lon, alt))
    } else {
        None
    };
    let mut spectra = Vec::with_capacity(n_spectra);
    for _ in 0..n_spectra {
        let q_h = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        pos += 8;
        let q_l = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        pos += 8;
        let doping = bytes[pos];
        pos += 1;
        let n_osc = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        let mut oscillators = Vec::with_capacity(n_osc);
        for _ in 0..n_osc {
            let freq_hz = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
            pos += 8;
            let bin_width_hz = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
            pos += 8;
            let val = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
            pos += 8;
            let err = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
            pos += 8;
            oscillators.push(SpinOscillator {
                freq_hz,
                bin_width_hz,
                val,
                err,
            });
        }
        spectra.push(SpinSpectrumBin {
            doping,
            q_h,
            q_l,
            oscillators,
        });
    }
    Some(SpinBin { lab, spectra })
}

pub fn encode_spin_bin(bin: &SpinBin) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&SPIN_MAGIC);
    out.push(SPIN_VERSION);
    out.extend_from_slice(&(bin.spectra.len() as u32).to_le_bytes());
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
    for s in &bin.spectra {
        out.extend_from_slice(&s.q_h.to_le_bytes());
        out.extend_from_slice(&s.q_l.to_le_bytes());
        out.push(s.doping);
        out.extend_from_slice(&(s.oscillators.len() as u32).to_le_bytes());
        for o in &s.oscillators {
            out.extend_from_slice(&o.freq_hz.to_le_bytes());
            out.extend_from_slice(&o.bin_width_hz.to_le_bytes());
            out.extend_from_slice(&o.val.to_le_bytes());
            out.extend_from_slice(&o.err.to_le_bytes());
        }
    }
    out
}

#[derive(Clone, Debug)]
pub struct ChargeSpectrum {
    pub momentum: f64,
    pub axis: u8,
    pub energy_mev: Vec<f64>,
    pub intensity: Vec<f64>,
}

pub fn parse_rixs_mev(text: &str) -> Option<ChargeSpectrum> {
    let mut momentum = 0.0f64;
    let mut axis = 0u8;
    let mut found = false;
    let mut energy = Vec::new();
    let mut intensity = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            let rest = &line[1..];
            if let Some(eq) = rest.find('=') {
                let key = rest[..eq].trim();
                let val_part = &rest[eq + 1..];
                let Some(val) = val_part.split_whitespace().next() else {
                    continue;
                };
                match key {
                    "H" => {
                        if let Ok(v) = val.parse::<f64>() {
                            if v.is_finite() {
                                momentum = v;
                                axis = 0;
                                found = true;
                            }
                        }
                    }
                    "L" => {
                        if let Ok(v) = val.parse::<f64>() {
                            if v.is_finite() {
                                momentum = v;
                                axis = 1;
                                found = true;
                            }
                        }
                    }
                    _ => {}
                }
            }
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 2 {
            continue;
        }
        let (Ok(e), Ok(i)) = (tokens[0].parse::<f64>(), tokens[1].parse::<f64>()) else {
            continue;
        };
        if !e.is_finite() || !i.is_finite() {
            continue;
        }
        energy.push(e);
        intensity.push(i);
    }
    if !found || energy.is_empty() {
        return None;
    }
    Some(ChargeSpectrum {
        momentum,
        axis,
        energy_mev: energy,
        intensity,
    })
}

pub fn charge_oscillators(spec: &ChargeSpectrum) -> Vec<SpinOscillator> {
    let n = spec.energy_mev.len();
    if n == 0 {
        return Vec::new();
    }
    let spacing = if n > 1 {
        let mut gaps: Vec<f64> = spec
            .energy_mev
            .windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .filter(|&g| g > 0.0 && g.is_finite())
            .collect();
        gaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mid = gaps.len() / 2;
        gaps.get(mid).copied().unwrap_or(0.0)
    } else {
        0.0
    };
    let bin_width = spacing * MEV_TO_HZ;
    spec.energy_mev
        .iter()
        .zip(spec.intensity.iter())
        .filter(|&(e, _)| *e > 0.0)
        .map(|(&e, &i)| SpinOscillator {
            freq_hz: e * MEV_TO_HZ,
            bin_width_hz: bin_width,
            val: i,
            err: 0.0,
        })
        .collect()
}

pub fn parse_sw_spin(text: &str) -> Option<SpinSpectrum> {
    let mut eloss = Vec::new();
    let mut weight = Vec::new();
    let mut err = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 3 {
            continue;
        }
        let (Ok(e), Ok(w), Ok(r)) = (
            tokens[0].parse::<f64>(),
            tokens[1].parse::<f64>(),
            tokens[2].parse::<f64>(),
        ) else {
            continue;
        };
        if !e.is_finite() || !w.is_finite() || !r.is_finite() {
            continue;
        }
        eloss.push(e);
        weight.push(w);
        err.push(r);
    }
    if eloss.is_empty() {
        return None;
    }
    Some(SpinSpectrum {
        eloss_ev: eloss,
        weight,
        err,
    })
}

pub fn spin_oscillators(spec: &SpinSpectrum) -> Vec<SpinOscillator> {
    let n = spec.eloss_ev.len();
    if n == 0 {
        return Vec::new();
    }
    let spacing = if n > 1 {
        let mut gaps: Vec<f64> = spec
            .eloss_ev
            .windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .filter(|&g| g > 0.0 && g.is_finite())
            .collect();
        gaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mid = gaps.len() / 2;
        gaps.get(mid).copied().unwrap_or(0.0)
    } else {
        0.0
    };
    let bin_width = spacing * EV_TO_HZ;
    let mut out = Vec::new();
    for i in 0..n {
        let e = spec.eloss_ev[i];
        if e > 0.0 {
            out.push(SpinOscillator {
                freq_hz: e * EV_TO_HZ,
                bin_width_hz: bin_width,
                val: spec.weight[i],
                err: spec.err[i],
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SW_SPIN: &str = "\
 Eloss   weight      err
-0.8951 19.6285 1.5233
-0.2432 93.5101 3.0285
-0.0040 40.1364 4.5892
0.0121 33.6050 4.8767
0.1903 3.8543 0.6532
0.2066 3.6474 0.6251
0.2228 3.9192 0.6828
";

    #[test]
    fn parse_sw_spin_columns() {
        let spec = parse_sw_spin(SW_SPIN).unwrap();
        assert_eq!(spec.eloss_ev.len(), 7);
        assert!((spec.eloss_ev[0] + 0.8951).abs() < 1e-9);
        assert!((spec.weight[3] - 33.6050).abs() < 1e-9);
        assert!((spec.err[3] - 4.8767).abs() < 1e-9);
    }

    #[test]
    fn oscillators_take_stokes_side_only() {
        let spec = parse_sw_spin(SW_SPIN).unwrap();
        let osc = spin_oscillators(&spec);
        assert_eq!(osc.len(), 4);
        assert!((osc[0].freq_hz - 0.0121 * EV_TO_HZ).abs() < 1e-6);
        assert!((osc[0].val - 33.6050).abs() < 1e-9);
        assert!(osc[0].bin_width_hz > 0.0);
    }

    #[test]
    fn empty_or_junk_is_none() {
        assert!(parse_sw_spin("").is_none());
        assert!(parse_sw_spin("Eloss weight err\n").is_none());
        assert!(parse_sw_spin("not a number here\n").is_none());
    }

    #[test]
    fn parse_rixs_mev_momentum_and_columns() {
        let text = "\
# H = -0.17 (theta scan)
# Energy (meV) Intensity (arb. units)
2.886587e+03 6.629702e+01
2.884613e+03 7.312404e+01
2.882639e+03 6.397913e+01
";
        let spec = parse_rixs_mev(text).unwrap();
        assert!((spec.momentum + 0.17).abs() < 1e-9);
        assert_eq!(spec.axis, 0);
        assert_eq!(spec.energy_mev.len(), 3);
        assert!((spec.intensity[1] - 73.12404).abs() < 1e-9);
        let osc = charge_oscillators(&spec);
        assert_eq!(osc.len(), 3);
        assert!((osc[0].freq_hz - 2.886587e3 * MEV_TO_HZ).abs() < 1e-3);
        assert!(osc[0].bin_width_hz > 0.0);
        assert!((osc[0].val - 66.29702).abs() < 1e-9);
    }

    #[test]
    fn parse_rixs_mev_l_axis() {
        let text = "# L = -2.5 (L scan)\n# Energy (meV) Intensity (arb. units)\n2.9e3 1.0\n";
        let spec = parse_rixs_mev(text).unwrap();
        assert_eq!(spec.axis, 1);
        assert!((spec.momentum + 2.5).abs() < 1e-9);
    }

    #[test]
    fn charge_oscillators_take_loss_side_only() {
        let text = "\
# H = -0.1 (H scan)
# Energy (meV) Intensity (arb. units)
-3.2e2 0.8
0.0e0 87.1
1.0e2 3.0
2.0e2 5.5
";
        let spec = parse_rixs_mev(text).unwrap();
        let osc = charge_oscillators(&spec);
        assert_eq!(osc.len(), 2, "gain and elastic rows carry no oscillator");
        assert!((osc[0].freq_hz - 1.0e2 * MEV_TO_HZ).abs() < 1e-3);
        assert!((osc[1].freq_hz - 2.0e2 * MEV_TO_HZ).abs() < 1e-3);
        assert!(osc.iter().all(|o| o.freq_hz > 0.0));
    }

    #[test]
    fn spin_bin_roundtrip() {
        let bin = SpinBin {
            lab: Some((45.206, 5.688, 200.0)),
            spectra: vec![
                SpinSpectrumBin {
                    doping: 1,
                    q_h: 0.23,
                    q_l: 0.23,
                    oscillators: vec![
                        SpinOscillator {
                            freq_hz: 0.0121 * EV_TO_HZ,
                            bin_width_hz: 0.0162 * EV_TO_HZ,
                            val: 33.6050,
                            err: 4.8767,
                        },
                        SpinOscillator {
                            freq_hz: 0.19 * EV_TO_HZ,
                            bin_width_hz: 0.0162 * EV_TO_HZ,
                            val: 3.8543,
                            err: 0.6532,
                        },
                    ],
                },
                SpinSpectrumBin {
                    doping: 0,
                    q_h: 0.18,
                    q_l: 0.0,
                    oscillators: vec![SpinOscillator {
                        freq_hz: 0.1 * EV_TO_HZ,
                        bin_width_hz: 0.0162 * EV_TO_HZ,
                        val: 10.0,
                        err: 1.0,
                    }],
                },
            ],
        };
        let bytes = encode_spin_bin(&bin);
        let back = parse_spin_bin(&bytes).unwrap();
        assert_eq!(back.lab, bin.lab);
        assert_eq!(back.spectra.len(), 2);
        assert_eq!(back.spectra[0].doping, 1);
        assert!((back.spectra[0].q_h - 0.23).abs() < 1e-12);
        assert_eq!(back.spectra[0].oscillators.len(), 2);
        assert!((back.spectra[0].oscillators[0].val - 33.6050).abs() < 1e-12);
        assert_eq!(back.spectra[1].doping, 0);
    }
}
