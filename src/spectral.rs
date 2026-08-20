// spectra.bin: header `0xCF 0x86 0x01 [epoch_tdb:f64] [count:u32]` LE (15 B),
// then count × [freq, bin_width, val] f64 LE (24 B per bin).
// freq = band center in Hz, bin_width = bandwidth in Hz from the native
// λ grid, val = spectral density in SI. Epoch = month middle of the measurement.
// The harvest step (NCEI-SSI netCDF-4/HDF5) is pending — the tabular
// form of the measurement (λ_nm, E_λ, flag) is the compiler's input; unreadable
// containers are named, never replaced (0 honored).

pub const SPECTRAL_MAGIC: [u8; 2] = [0xCF, 0x86];
pub const SPECTRAL_VERSION: u8 = 0x01;
pub const SPECTRAL_HEADER_BYTES: usize = 15;
pub const SPECTRAL_RECORD_BYTES: usize = 24;

pub const C_LIGHT: f64 = 299792458.0;

pub fn bins_from_lambda_rows(rows: &[(f64, f64, u8)]) -> Vec<(f64, f64, f64)> {
    let valid = |&(l, e, f): &(f64, f64, u8)| {
        f == 0 && l.is_finite() && e.is_finite() && l > 0.0 && e > 0.0
    };
    let mut out = Vec::new();
    for (i, &(lam_nm, e_lam, flag)) in rows.iter().enumerate() {
        if !valid(&(lam_nm, e_lam, flag)) {
            continue;
        }
        let lam_m = lam_nm * 1e-9;
        let freq = C_LIGHT / lam_m;
        let e_nu = e_lam * lam_m * lam_m / C_LIGHT;
        let prev_nu = rows[..i]
            .iter()
            .rev()
            .find(|r| valid(r))
            .map(|&(l, _, _)| C_LIGHT / (l * 1e-9));
        let next_nu = rows[i + 1..]
            .iter()
            .find(|r| valid(r))
            .map(|&(l, _, _)| C_LIGHT / (l * 1e-9));
        let bin_width = match (prev_nu, next_nu) {
            (Some(hi), Some(lo)) => (hi - lo) * 0.5,
            (Some(hi), None) => (hi - freq).abs(),
            (None, Some(lo)) => (freq - lo).abs(),
            (None, None) => 0.0,
        };
        out.push((freq, bin_width, e_nu));
    }
    out
}

pub fn write_spectral_bin(epoch_tdb: f64, bins: &[(f64, f64, f64)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(SPECTRAL_HEADER_BYTES + bins.len() * SPECTRAL_RECORD_BYTES);
    out.extend_from_slice(&SPECTRAL_MAGIC);
    out.push(SPECTRAL_VERSION);
    out.extend_from_slice(&epoch_tdb.to_le_bytes());
    out.extend_from_slice(&(bins.len() as u32).to_le_bytes());
    for &(freq, bin_width, val) in bins {
        out.extend_from_slice(&freq.to_le_bytes());
        out.extend_from_slice(&bin_width.to_le_bytes());
        out.extend_from_slice(&val.to_le_bytes());
    }
    out
}

pub fn parse_spectral_bin(bytes: &[u8]) -> Option<(f64, Vec<(f64, f64, f64)>)> {
    if bytes.len() < SPECTRAL_HEADER_BYTES
        || bytes[0] != SPECTRAL_MAGIC[0]
        || bytes[1] != SPECTRAL_MAGIC[1]
        || bytes[2] != SPECTRAL_VERSION
    {
        return None;
    }
    let epoch = f64::from_le_bytes(bytes[3..11].try_into().ok()?);
    let count = u32::from_le_bytes(bytes[11..15].try_into().ok()?) as usize;
    if !epoch.is_finite() || bytes.len() != SPECTRAL_HEADER_BYTES + count * SPECTRAL_RECORD_BYTES {
        return None;
    }
    let mut bins = Vec::with_capacity(count);
    for i in 0..count {
        let off = SPECTRAL_HEADER_BYTES + i * SPECTRAL_RECORD_BYTES;
        let freq = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        let bin_width = f64::from_le_bytes(bytes[off + 8..off + 16].try_into().ok()?);
        let val = f64::from_le_bytes(bytes[off + 16..off + 24].try_into().ok()?);
        if !freq.is_finite() || !bin_width.is_finite() || !val.is_finite() {
            return None;
        }
        bins.push((freq, bin_width, val));
    }
    Some((epoch, bins))
}

fn is_leap(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub fn month_middle_unix(year: u32, month: u32) -> Option<f64> {
    if year < 1970 || month < 1 || month > 12 {
        return None;
    }
    let mut days: u64 = 0;
    for y in 1970..year {
        days += if is_leap(y) { 366 } else { 365 };
    }
    for m in 1..month {
        days += days_in_month(year, m) as u64;
    }
    Some(days as f64 * 86400.0 + days_in_month(year, month) as f64 * 43200.0)
}

pub const COLOR_LUT_LEN: usize = 256;

const COLOR_LOCUS: [(f64, f64); 31] = [
    (-0.120, 10700.0),
    (-0.037, 9700.0),
    (0.005, 9300.0),
    (0.068, 8800.0),
    (0.110, 8600.0),
    (0.194, 8100.0),
    (0.320, 7590.0),
    (0.377, 7220.0),
    (0.490, 6820.0),
    (0.587, 6550.0),
    (0.694, 6180.0),
    (0.784, 5930.0),
    (0.823, 5770.0),
    (0.850, 5660.0),
    (0.900, 5480.0),
    (0.983, 5270.0),
    (1.100, 5100.0),
    (1.340, 4600.0),
    (1.530, 4300.0),
    (1.730, 3990.0),
    (1.840, 3850.0),
    (2.090, 3660.0),
    (2.230, 3560.0),
    (2.500, 3430.0),
    (2.940, 3210.0),
    (3.350, 3060.0),
    (3.710, 2930.0),
    (4.160, 2810.0),
    (4.650, 2680.0),
    (4.860, 2570.0),
    (5.100, 2420.0),
];

fn bp_rp_to_teff(ci: f64) -> f64 {
    if ci <= COLOR_LOCUS[0].0 {
        return COLOR_LOCUS[0].1;
    }
    for i in 1..COLOR_LOCUS.len() {
        let b = COLOR_LOCUS[i];
        if ci <= b.0 {
            let a = COLOR_LOCUS[i - 1];
            let t = (ci - a.0) / (b.0 - a.0).max(1e-6);
            return a.1 + t * (b.1 - a.1);
        }
    }
    COLOR_LOCUS[COLOR_LOCUS.len() - 1].1
}

fn teff_to_rgb(teff: f64) -> [f64; 3] {
    let t = teff.clamp(1000.0, 40000.0) / 100.0;
    let (r, g) = if t <= 66.0 {
        (255.0, 99.4708025861 * t.ln() - 161.1195681661)
    } else {
        (
            329.698727446 * (t - 60.0).powf(-0.1332047592),
            288.1221695283 * (t - 60.0).powf(-0.0755148492),
        )
    };
    let b = if t >= 66.0 {
        255.0
    } else if t <= 19.0 {
        0.0
    } else {
        138.5177312231 * (t - 10.0).ln() - 305.0447927307
    };
    [
        r.clamp(0.0, 255.0) / 255.0,
        g.clamp(0.0, 255.0) / 255.0,
        b.clamp(0.0, 255.0) / 255.0,
    ]
}

pub fn color_lut_rgba() -> [[f32; 4]; COLOR_LUT_LEN] {
    let lo = COLOR_LOCUS[0].0;
    let hi = COLOR_LOCUS[COLOR_LOCUS.len() - 1].0;
    let mut lut = [[0.0; 4]; COLOR_LUT_LEN];
    for (i, e) in lut.iter_mut().enumerate() {
        let ci = if i == 0 {
            lo
        } else if i == COLOR_LUT_LEN - 1 {
            hi
        } else {
            lo + (i as f64 + 0.5) * (hi - lo) / COLOR_LUT_LEN as f64
        };
        let rgb = teff_to_rgb(bp_rp_to_teff(ci));
        *e = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32, 1.0];
    }
    lut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bins_convert_lambda_to_frequency() {
        let rows = [(100.0, 1.0, 0), (200.0, 2.0, 0), (400.0, 3.0, 0)];
        let bins = bins_from_lambda_rows(&rows);
        assert_eq!(bins.len(), 3);
        let (f, w, v) = bins[1];
        let lam_m = 200e-9;
        assert!((f - C_LIGHT / lam_m).abs() / f < 1e-12);
        assert!((v - 2.0 * lam_m * lam_m / C_LIGHT).abs() < 1e-30);
        let nu_prev = C_LIGHT / 100e-9;
        let nu_next = C_LIGHT / 400e-9;
        let w_mid = (nu_prev - nu_next) * 0.5;
        assert!((w - w_mid).abs() / w_mid < 1e-12);
        let (_, w_first, _) = bins[0];
        let (_, w_last, _) = bins[2];
        let w_first_expected = C_LIGHT / 100e-9 - C_LIGHT / 200e-9;
        let w_last_expected = C_LIGHT / 200e-9 - C_LIGHT / 400e-9;
        assert!((w_first - w_first_expected).abs() / w_first_expected < 1e-12);
        assert!((w_last - w_last_expected).abs() / w_last_expected < 1e-12);
    }

    #[test]
    fn bins_drop_invalid_rows() {
        let rows = [
            (100.0, 1.0, 1),
            (200.0, -2.0, 0),
            (400.0, f64::NAN, 0),
            (800.0, f64::INFINITY, 0),
            (0.0, 1.0, 0),
            (300.0, 3.0, 0),
        ];
        let bins = bins_from_lambda_rows(&rows);
        assert_eq!(bins.len(), 1);
        assert!((bins[0].0 - C_LIGHT / 300e-9).abs() / bins[0].0 < 1e-12);
        assert_eq!(bins[0].1, 0.0);
    }

    #[test]
    fn bin_roundtrip() {
        let bins = vec![(5.0e14, 1.0e14, 2.5e-14), (5.5e14, 1.0e14, 2.0e-14)];
        let bytes = write_spectral_bin(1781488800.0, &bins);
        assert_eq!(bytes.len(), 15 + 48);
        let (epoch, parsed) = parse_spectral_bin(&bytes).unwrap();
        assert_eq!(epoch, 1781488800.0);
        assert_eq!(parsed, bins);
    }

    #[test]
    fn parse_refuses_malformed() {
        assert!(parse_spectral_bin(&[0xCF, 0x86, 0x01]).is_none());
        assert!(
            parse_spectral_bin(&[0xCF, 0x86, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).is_none()
        );
        let mut bytes = write_spectral_bin(0.0, &[(1.0, 2.0, 3.0)]);
        bytes[14] = 2;
        assert!(parse_spectral_bin(&bytes).is_none());
        let truncated = write_spectral_bin(0.0, &[(1.0, 2.0, 3.0)]);
        assert!(parse_spectral_bin(&truncated[..20]).is_none());
    }

    #[test]
    fn month_middle_known_value() {
        let unix = month_middle_unix(2026, 6).unwrap();
        assert_eq!(unix, 1781568000.0);
        assert!(month_middle_unix(1969, 12).is_none());
        assert!(month_middle_unix(2026, 13).is_none());
    }

    #[test]
    fn color_lut_entries_finite_in_unit_range() {
        let lut = color_lut_rgba();
        assert_eq!(lut.len(), COLOR_LUT_LEN);
        for e in lut {
            for v in e {
                assert!(v.is_finite(), "LUT entry non-finite: {e:?}");
            }
            assert!(e[3] == 1.0);
            for v in &e[..3] {
                assert!(*v >= 0.0 && *v <= 1.0, "RGB outside unit range: {e:?}");
            }
        }
    }

    #[test]
    fn color_lut_ends_match_locus_clamps() {
        let lut = color_lut_rgba();
        let first = teff_to_rgb(bp_rp_to_teff(COLOR_LOCUS[0].0));
        let last = teff_to_rgb(bp_rp_to_teff(COLOR_LOCUS[COLOR_LOCUS.len() - 1].0));
        for (i, v) in first.iter().enumerate() {
            assert!(
                ((lut[0][i] as f64) - v).abs() < 1e-6,
                "first entry mismatch"
            );
        }
        for (i, v) in last.iter().enumerate() {
            assert!(
                ((lut[255][i] as f64) - v).abs() < 1e-6,
                "last entry mismatch"
            );
        }
    }
}
