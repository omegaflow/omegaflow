use crate::lsk::LeapSeconds;

pub const MAGIC: [u8; 4] = *b"MAX1";
pub const HEADER_BYTES: usize = 8;

pub const BAND_2_20: u32 = 0;
pub const BAND_2_4: u32 = 1;
pub const BAND_4_10: u32 = 2;
pub const BAND_10_20: u32 = 3;

const H_PLANCK: f64 = 6.626_070_15e-34;
const J_PER_KEV: f64 = 1.602_176_634e-16;
const MJD_UNIX_EPOCH: f64 = 40587.0;
const SECS_PER_DAY: f64 = 86400.0;

pub fn band_kev_range(band: u32) -> Option<(f64, f64)> {
    match band {
        BAND_2_20 => Some((2.0, 20.0)),
        BAND_2_4 => Some((2.0, 4.0)),
        BAND_4_10 => Some((4.0, 10.0)),
        BAND_10_20 => Some((10.0, 20.0)),
        _ => None,
    }
}

pub fn band_freq_width(band: u32) -> Option<(f64, f64)> {
    let (lo, hi) = band_kev_range(band)?;
    let f_lo = lo * J_PER_KEV / H_PLANCK;
    let f_hi = hi * J_PER_KEV / H_PLANCK;
    Some((0.5 * (f_lo + f_hi), f_hi - f_lo))
}

pub fn mjd_to_tdb(mjd: f64, lsk: &LeapSeconds) -> Option<f64> {
    lsk.unix_to_tdb((mjd - MJD_UNIX_EPOCH) * SECS_PER_DAY)
}

#[derive(Clone, Debug)]
pub struct MaxiSample {
    pub t_tdb: f64,
    pub flux: f32,
    pub err: f32,
}

#[derive(Clone, Debug)]
pub struct MaxiCurve {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub band: u32,
    pub freq_hz: f64,
    pub bin_width_hz: f64,
    pub samples: Vec<MaxiSample>,
}

pub fn write_bin(curves: &[MaxiCurve]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(curves.len() as u32).to_le_bytes());
    for c in curves {
        if !c.ra_deg.is_finite()
            || !c.dec_deg.is_finite()
            || !c.freq_hz.is_finite()
            || !c.bin_width_hz.is_finite()
            || band_kev_range(c.band).is_none()
        {
            return None;
        }
        out.extend_from_slice(&c.ra_deg.to_le_bytes());
        out.extend_from_slice(&c.dec_deg.to_le_bytes());
        out.extend_from_slice(&c.band.to_le_bytes());
        out.extend_from_slice(&c.freq_hz.to_le_bytes());
        out.extend_from_slice(&c.bin_width_hz.to_le_bytes());
        out.extend_from_slice(&(c.samples.len() as u32).to_le_bytes());
        for s in &c.samples {
            if !s.t_tdb.is_finite() || !s.flux.is_finite() || !s.err.is_finite() {
                return None;
            }
            out.extend_from_slice(&s.t_tdb.to_le_bytes());
            out.extend_from_slice(&s.flux.to_le_bytes());
            out.extend_from_slice(&s.err.to_le_bytes());
        }
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<MaxiCurve>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = HEADER_BYTES;
    let mut curves = Vec::with_capacity(count);
    for _ in 0..count {
        let f64_at = |o: &mut usize| -> Option<f64> {
            let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
            *o += 8;
            Some(v)
        };
        let ra_deg = f64_at(&mut off)?;
        let dec_deg = f64_at(&mut off)?;
        let band = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let freq_hz = f64_at(&mut off)?;
        let bin_width_hz = f64_at(&mut off)?;
        let n = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if band_kev_range(band).is_none()
            || !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !freq_hz.is_finite()
            || !bin_width_hz.is_finite()
        {
            return None;
        }
        let mut samples = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let t_tdb = f64_at(&mut off)?;
            let flux = f32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            let err = f32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
            off += 4;
            if !t_tdb.is_finite() || !flux.is_finite() || !err.is_finite() {
                return None;
            }
            samples.push(MaxiSample { t_tdb, flux, err });
        }
        curves.push(MaxiCurve {
            ra_deg,
            dec_deg,
            band,
            freq_hz,
            bin_width_hz,
            samples,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(curves)
}

pub fn parse_dat(text: &str) -> Vec<(f64, [f64; 4], [f64; 4])> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split_whitespace().collect();
        if cols.len() != 9 {
            continue;
        }
        let f = |i: usize| {
            cols.get(i)
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite())
        };
        let (
            Some(mjd),
            Some(f20),
            Some(e20),
            Some(f24),
            Some(e24),
            Some(f410),
            Some(e410),
            Some(f1020),
            Some(e1020),
        ) = (f(0), f(1), f(2), f(3), f(4), f(5), f(6), f(7), f(8))
        else {
            continue;
        };
        rows.push((mjd, [f20, f24, f410, f1020], [e20, e24, e410, e1020]));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_ranges_map_to_frequency() {
        assert_eq!(band_kev_range(BAND_2_20), Some((2.0, 20.0)));
        assert_eq!(band_kev_range(BAND_10_20), Some((10.0, 20.0)));
        assert_eq!(band_kev_range(9), None);
        let (freq, width) = band_freq_width(BAND_2_4).unwrap();
        assert!(freq > 0.0 && width > 0.0 && width < freq);
    }

    #[test]
    fn mjd_zero_is_unix_epoch_minus_mjd_offset() {
        let lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(0.0, -1e12)],
        };
        let tdb = mjd_to_tdb(MJD_UNIX_EPOCH, &lsk).unwrap();
        assert_eq!(tdb, 32.184 + 0.0 - 946728000.0);
    }

    #[test]
    fn parse_dat_reads_headerless_columns_and_skips_malformed() {
        let text = "55056.500000 0.040581 0.021283 0.005341 0.009955 0.010642 0.009569 0.018873 0.013515\n\
55057.500000 -0.028320 0.017064 0.013319 0.006230 -0.007779 0.006996 -0.025332 0.011980\n\
short line\n\
55058.500000 nan 0.016655 -0.000538 0.005616 0.001295 0.006654 0.008026 0.012016\n";
        let rows = parse_dat(text);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, 55056.5);
        assert_eq!(rows[0].1[0], 0.040581);
        assert_eq!(rows[0].2[3], 0.013515);
        assert_eq!(
            rows[1].1[0], -0.028320,
            "background-subtracted flux stays negative"
        );
    }

    #[test]
    fn bin_roundtrip() {
        let curves = vec![MaxiCurve {
            ra_deg: 1.5814,
            dec_deg: 20.2029,
            band: BAND_2_20,
            freq_hz: 2.6e18,
            bin_width_hz: 4.3e18,
            samples: vec![
                MaxiSample {
                    t_tdb: 8.0e8,
                    flux: 0.04,
                    err: 0.02,
                },
                MaxiSample {
                    t_tdb: 8.1e8,
                    flux: -0.03,
                    err: 0.01,
                },
            ],
        }];
        let bytes = write_bin(&curves).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].ra_deg, 1.5814);
        assert_eq!(parsed[0].band, BAND_2_20);
        assert_eq!(parsed[0].samples.len(), 2);
        assert_eq!(parsed[0].samples[1].flux, -0.03);
    }

    #[test]
    fn register_block_parses_and_extracts_measured_columns() {
        let block = "url https://maxi.riken.jp/star_data/J0006+202/J0006+202_g_lc_1day_all.dat\n\
ttl 86400\n\
format text\n\
at earth\n\
rows\n\
field 1 maxi_J0006+202_2_20kev_flux inverse-square em ph/s/cm2 86400 0.0 0.0\n\
field 3 maxi_J0006+202_2_4kev_flux inverse-square em ph/s/cm2 86400 0.0 0.0\n\
field 5 maxi_J0006+202_4_10kev_flux inverse-square em ph/s/cm2 86400 0.0 0.0\n\
field 7 maxi_J0006+202_10_20kev_flux inverse-square em ph/s/cm2 86400 0.0 0.0\n";
        let srcs = crate::archivar::parse_sources(block);
        assert_eq!(srcs.len(), 1);
        let src = &srcs[0];
        let crate::archivar::Extract::Rows { fields, .. } = &src.extracts[0] else {
            panic!("Rows extract absent");
        };
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[0].key, "1");
        assert_eq!(fields[0].name, "maxi_J0006+202_2_20kev_flux");
        assert_eq!(fields[0].unit, "ph/s/cm2");
        assert_eq!(fields[3].key, "7");

        let body = "55056.500000 0.040581 0.021283 0.005341 0.009955 0.010642 0.009569 0.018873 0.013515\n";
        let lsk = crate::archivar::embedded_lsk().expect("embedded naif0012 parses");
        let crate::archivar::ExtractResult::Measurements(channels) =
            crate::archivar::extract(src, body, 8.0e8, &lsk)
        else {
            panic!("extract is not Measurements");
        };
        assert_eq!(channels.len(), 4);
        assert!(
            (channels[0].0.value - 0.040581).abs() < 1e-9,
            "2-20 keV flux from column 1"
        );
        assert!(
            (channels[1].0.value - 0.005341).abs() < 1e-9,
            "2-4 keV flux from column 3"
        );
        assert!(
            (channels[2].0.value - 0.010642).abs() < 1e-9,
            "4-10 keV flux from column 5"
        );
        assert!(
            (channels[3].0.value - 0.018873).abs() < 1e-9,
            "10-20 keV flux from column 7"
        );
    }

    #[test]
    fn bin_refuses_foreign_or_nonfinite() {
        assert!(parse_bin(b"MAX1").is_none());
        assert!(parse_bin(b"XXXXabc").is_none());
        let bad = vec![MaxiCurve {
            ra_deg: f64::NAN,
            dec_deg: 0.0,
            band: BAND_2_20,
            freq_hz: 1.0,
            bin_width_hz: 1.0,
            samples: vec![],
        }];
        assert!(write_bin(&bad).is_none());
    }
}
