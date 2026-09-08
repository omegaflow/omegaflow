pub const SPECTRAL_MAGIC: [u8; 2] = [0xCF, 0x86];
pub const SPECTRAL_VERSION: u8 = 0x01;
pub const SPECTRAL_HEADER_BYTES: usize = 15;
pub const SPECTRAL_RECORD_BYTES: usize = 24;

use crate::archivar::types::C_LIGHT;

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
        let e_nu = e_lam * 1e9 * lam_m * lam_m / C_LIGHT;
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

pub const XP_SPECTRAL_VERSION: u8 = 0x02;

pub struct XpStar {
    pub source_id: u64,
    pub ra: f64,
    pub dec: f64,
    pub plx_mas: f64,
    pub bins: Vec<(f64, f64, f64)>,
}

pub const XP_GRID_LAM_NM_FIRST: f64 = 400.0;
pub const XP_GRID_LAM_NM_STEP: f64 = 10.0;
pub const XP_GRID_SAMPLES: usize = 41;

pub fn xp_bins_from_flux_array(flux: &[f64]) -> Vec<(f64, f64, f64)> {
    if flux.len() != XP_GRID_SAMPLES {
        return Vec::new();
    }
    let mut rows = Vec::with_capacity(XP_GRID_SAMPLES);
    for (i, &e) in flux.iter().enumerate() {
        let lam_nm = XP_GRID_LAM_NM_FIRST + i as f64 * XP_GRID_LAM_NM_STEP;
        let flag = if e.is_finite() && e > 0.0 { 0 } else { 1 };
        rows.push((lam_nm, e, flag));
    }
    bins_from_lambda_rows(&rows)
}

pub fn write_xp_spectra_bin(epoch_tdb: f64, stars: &[XpStar]) -> Vec<u8> {
    let mut size = SPECTRAL_HEADER_BYTES;
    for s in stars {
        size += 36 + s.bins.len() * SPECTRAL_RECORD_BYTES;
    }
    let mut out = Vec::with_capacity(size);
    out.extend_from_slice(&SPECTRAL_MAGIC);
    out.push(XP_SPECTRAL_VERSION);
    out.extend_from_slice(&epoch_tdb.to_le_bytes());
    out.extend_from_slice(&(stars.len() as u32).to_le_bytes());
    for s in stars {
        out.extend_from_slice(&s.source_id.to_le_bytes());
        out.extend_from_slice(&s.ra.to_le_bytes());
        out.extend_from_slice(&s.dec.to_le_bytes());
        out.extend_from_slice(&s.plx_mas.to_le_bytes());
        out.extend_from_slice(&(s.bins.len() as u32).to_le_bytes());
        for &(freq, bin_width, val) in &s.bins {
            out.extend_from_slice(&freq.to_le_bytes());
            out.extend_from_slice(&bin_width.to_le_bytes());
            out.extend_from_slice(&val.to_le_bytes());
        }
    }
    out
}

pub fn parse_xp_spectra_bin(bytes: &[u8]) -> Option<(f64, Vec<XpStar>)> {
    if bytes.len() < SPECTRAL_HEADER_BYTES
        || bytes[0] != SPECTRAL_MAGIC[0]
        || bytes[1] != SPECTRAL_MAGIC[1]
        || bytes[2] != XP_SPECTRAL_VERSION
    {
        return None;
    }
    let epoch = f64::from_le_bytes(bytes[3..11].try_into().ok()?);
    let count = u32::from_le_bytes(bytes[11..15].try_into().ok()?) as usize;
    if !epoch.is_finite() {
        return None;
    }
    let mut stars = Vec::with_capacity(count);
    let mut off = SPECTRAL_HEADER_BYTES;
    for _ in 0..count {
        if off + 36 > bytes.len() {
            return None;
        }
        let source_id = u64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
        let ra = f64::from_le_bytes(bytes[off + 8..off + 16].try_into().ok()?);
        let dec = f64::from_le_bytes(bytes[off + 16..off + 24].try_into().ok()?);
        let plx_mas = f64::from_le_bytes(bytes[off + 24..off + 32].try_into().ok()?);
        let n_bins = u32::from_le_bytes(bytes[off + 32..off + 36].try_into().ok()?) as usize;
        if !ra.is_finite() || !dec.is_finite() || !plx_mas.is_finite() {
            return None;
        }
        off += 36;
        let mut bins = Vec::with_capacity(n_bins);
        for _ in 0..n_bins {
            if off + SPECTRAL_RECORD_BYTES > bytes.len() {
                return None;
            }
            let freq = f64::from_le_bytes(bytes[off..off + 8].try_into().ok()?);
            let bin_width = f64::from_le_bytes(bytes[off + 8..off + 16].try_into().ok()?);
            let val = f64::from_le_bytes(bytes[off + 16..off + 24].try_into().ok()?);
            if !freq.is_finite() || !bin_width.is_finite() || !val.is_finite() {
                return None;
            }
            bins.push((freq, bin_width, val));
            off += SPECTRAL_RECORD_BYTES;
        }
        stars.push(XpStar {
            source_id,
            ra,
            dec,
            plx_mas,
            bins,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some((epoch, stars))
}

fn is_leap(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: u32, month: u32) -> Option<u32> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap(year) { 29 } else { 28 }),
        _ => None,
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
        let d = match days_in_month(year, m) {
            Some(d) => d,
            None => return None,
        };
        days += d as u64;
    }
    let mid = match days_in_month(year, month) {
        Some(d) => d as f64,
        None => return None,
    };
    Some(days as f64 * 86400.0 + mid * 43200.0)
}

pub fn civil_from_days(days: i64) -> Option<(u32, u32, u32)> {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    Some((y as u32, m, d))
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

const PASSBAND_SENTINEL: f64 = 99.0;

fn passband_table() -> &'static Vec<(f64, f64, f64)> {
    static TABLE: std::sync::OnceLock<Vec<(f64, f64, f64)>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| parse_passbands(include_str!("kernels/gaia_edr3_passbands.dat")))
}

pub fn parse_passbands(raw: &str) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    for line in raw.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 7 {
            continue;
        }
        let (Ok(lambda), Ok(bp), Ok(rp)) = (
            cols[0].parse::<f64>(),
            cols[3].parse::<f64>(),
            cols[5].parse::<f64>(),
        ) else {
            continue;
        };
        if !lambda.is_finite() || !bp.is_finite() || !rp.is_finite() {
            continue;
        }
        let bp = if bp >= PASSBAND_SENTINEL { 0.0 } else { bp };
        let rp = if rp >= PASSBAND_SENTINEL { 0.0 } else { rp };
        out.push((lambda, bp, rp));
    }
    out
}

fn passband_at(table: &[(f64, f64, f64)], lam_nm: f64) -> (f64, f64) {
    let (first, last) = (table[0], table[table.len() - 1]);
    if lam_nm <= first.0 {
        return (first.1, first.2);
    }
    if lam_nm >= last.0 {
        return (last.1, last.2);
    }
    let idx = table.partition_point(|&(l, _, _)| l < lam_nm);
    let (l0, b0, r0) = table[idx - 1];
    let (l1, b1, r1) = table[idx];
    let t = (lam_nm - l0) / (l1 - l0);
    (b0 + t * (b1 - b0), r0 + t * (r1 - r0))
}

pub fn sed_to_bp_rp(bins: &[(f64, f64, f64)]) -> Option<f64> {
    let table = passband_table();
    if table.len() < 2 {
        return None;
    }
    let mut num_bp = 0.0f64;
    let mut num_rp = 0.0f64;
    for &(freq, bin_width, val) in bins {
        if !freq.is_finite() || !bin_width.is_finite() || !val.is_finite() {
            continue;
        }
        if freq <= 0.0 || val <= 0.0 {
            continue;
        }
        let lam_nm = C_LIGHT / freq * 1e9;
        let (sbp, srp) = passband_at(table, lam_nm);
        num_bp += val * sbp * lam_nm * bin_width;
        num_rp += val * srp * lam_nm * bin_width;
    }
    if !num_bp.is_finite() || !num_rp.is_finite() || num_bp <= 0.0 || num_rp <= 0.0 {
        return None;
    }
    Some(-2.5 * (num_bp / num_rp).log10())
}

pub fn band_overlap(freq: f64, bin_width: f64, lo: f64, hi: f64) -> bool {
    if !freq.is_finite() || !bin_width.is_finite() || !lo.is_finite() || !hi.is_finite() {
        return false;
    }
    if freq <= 0.0 {
        return false;
    }
    let half = (bin_width * 0.5).abs();
    let band_lo = freq - half;
    let band_hi = freq + half;
    band_hi >= lo && band_lo <= hi
}

pub fn color_for_ci(ci: f64) -> [f32; 4] {
    if ci == 0.0 {
        return [1.0, 1.0, 1.0, 1.0];
    }
    let lut = color_lut_rgba();
    let lo = COLOR_LOCUS[0].0;
    let hi = COLOR_LOCUS[COLOR_LOCUS.len() - 1].0;
    if ci <= lo {
        return lut[0];
    }
    if ci >= hi {
        return lut[COLOR_LUT_LEN - 1];
    }
    let idx = ((ci - lo) / (hi - lo) * COLOR_LUT_LEN as f64) as usize;
    lut[idx.min(COLOR_LUT_LEN - 1)]
}

fn extinction_table() -> &'static Vec<(f64, f64)> {
    static TABLE: std::sync::OnceLock<Vec<(f64, f64)>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| parse_extinction(include_str!("kernels/ccm89_rv31.dat")))
}

pub fn parse_extinction(raw: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for line in raw.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = t.split_whitespace().collect();
        if cols.len() < 2 {
            continue;
        }
        let (Ok(lam), Ok(av)) = (cols[0].parse::<f64>(), cols[1].parse::<f64>()) else {
            continue;
        };
        if !lam.is_finite() || !av.is_finite() || lam <= 0.0 || av <= 0.0 {
            continue;
        }
        out.push((lam, av));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

pub fn extinction_at(lam_nm: f64) -> Option<f64> {
    let table = extinction_table();
    if table.len() < 2 || !lam_nm.is_finite() || lam_nm <= 0.0 {
        return None;
    }
    let (first, last) = (table[0], table[table.len() - 1]);
    if lam_nm <= first.0 {
        return Some(first.1);
    }
    if lam_nm >= last.0 {
        return Some(last.1);
    }
    let idx = table.partition_point(|&(l, _)| l < lam_nm);
    let (l0, a0) = table[idx - 1];
    let (l1, a1) = table[idx];
    let t = (lam_nm - l0) / (l1 - l0);
    Some(a0 + t * (a1 - a0))
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
        assert!((v - 2.0 * 1e9 * lam_m * lam_m / C_LIGHT).abs() / v < 1e-12);
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

    #[test]
    fn parse_passbands_reads_the_embedded_table() {
        let table = parse_passbands(include_str!("kernels/gaia_edr3_passbands.dat"));
        assert_eq!(table.len(), 781);
        assert_eq!(table[0].0, 320.0);
        assert_eq!(table[table.len() - 1].0, 1100.0);
        assert!(table
            .iter()
            .all(|&(l, b, r)| l.is_finite() && b.is_finite() && r.is_finite()));
        assert!(table
            .iter()
            .all(|&(_, b, r)| b >= 0.0 && b < 99.0 && r >= 0.0 && r < 99.0));
    }

    #[test]
    fn sed_to_bp_rp_returns_none_for_an_empty_spectrum() {
        assert!(sed_to_bp_rp(&[]).is_none());
    }

    #[test]
    fn sed_to_bp_rp_refuses_a_bin_without_rp_response() {
        let lam_m = 400.0e-9;
        let freq = C_LIGHT / lam_m;
        assert!(sed_to_bp_rp(&[(freq, 1.0e13, 1.0)]).is_none());
    }

    #[test]
    fn sed_to_bp_rp_rp_dominant_bin_is_positive() {
        let lam_m = 660.0e-9;
        let freq = C_LIGHT / lam_m;
        let ci = sed_to_bp_rp(&[(freq, 1.0e13, 1.0)]).unwrap();
        assert!(ci.is_finite());
        assert!(ci > 0.0, "red band color {ci}");
    }

    #[test]
    fn sed_to_bp_rp_hot_blackbody_is_bluer_than_cool() {
        let hot = sed_to_bp_rp(&blackbody_bins(10_000.0)).unwrap();
        let cool = sed_to_bp_rp(&blackbody_bins(3_000.0)).unwrap();
        assert!(hot < cool, "hot blackbody {hot} vs cool blackbody {cool}");
    }

    #[test]
    fn band_overlap_matches_the_window() {
        assert!(band_overlap(500.0, 20.0, 490.0, 510.0));
        assert!(band_overlap(500.0, 20.0, 505.0, 520.0));
        assert!(!band_overlap(500.0, 20.0, 600.0, 700.0));
        assert!(
            !band_overlap(0.0, 0.0, 0.0, 1.0e15),
            "a point source carries no band"
        );
        assert!(!band_overlap(f64::NAN, 20.0, 0.0, 1.0));
    }

    #[test]
    fn color_for_ci_zero_is_white() {
        assert_eq!(color_for_ci(0.0), [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn color_for_ci_clamps_to_the_lut_ends() {
        let lo = COLOR_LOCUS[0].0;
        let hi = COLOR_LOCUS[COLOR_LOCUS.len() - 1].0;
        assert_eq!(color_for_ci(lo - 1.0), color_lut_rgba()[0]);
        assert_eq!(color_for_ci(hi + 1.0), color_lut_rgba()[COLOR_LUT_LEN - 1]);
    }

    #[test]
    fn xp_bins_map_the_fixed_grid() {
        let mut flux = [0.0f64; XP_GRID_SAMPLES];
        flux[0] = 1.0;
        flux[XP_GRID_SAMPLES - 1] = 2.0;
        let bins = xp_bins_from_flux_array(&flux);
        assert_eq!(bins.len(), 2);
        let lam0 = 400e-9;
        assert!((bins[0].0 - C_LIGHT / lam0).abs() / bins[0].0 < 1e-12);
        assert!((bins[0].2 - 1.0 * 1e9 * lam0 * lam0 / C_LIGHT).abs() / bins[0].2 < 1e-12);
        let lam1 = 800e-9;
        assert!((bins[1].0 - C_LIGHT / lam1).abs() / bins[1].0 < 1e-12);
    }

    #[test]
    fn xp_bins_drop_nonpositive_and_reject_wrong_arity() {
        let mut flux = [1.0f64; XP_GRID_SAMPLES];
        flux[10] = -0.5;
        flux[20] = f64::NAN;
        let bins = xp_bins_from_flux_array(&flux);
        assert_eq!(bins.len(), XP_GRID_SAMPLES - 2);
        assert!(xp_bins_from_flux_array(&[1.0; 5]).is_empty());
    }

    #[test]
    fn xp_spectra_roundtrip() {
        let stars = vec![
            XpStar {
                source_id: 424140906078848,
                ra: 45.0,
                dec: 30.0,
                plx_mas: 1.0,
                bins: vec![(5.0e14, 1.0e14, 2.5e-14)],
            },
            XpStar {
                source_id: 1,
                ra: 180.0,
                dec: -60.0,
                plx_mas: 0.5,
                bins: Vec::new(),
            },
        ];
        let bytes = write_xp_spectra_bin(1781488800.0, &stars);
        let (epoch, parsed) = parse_xp_spectra_bin(&bytes).unwrap();
        assert_eq!(epoch, 1781488800.0);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].source_id, 424140906078848);
        assert_eq!(parsed[0].ra, 45.0);
        assert_eq!(parsed[0].plx_mas, 1.0);
        assert_eq!(parsed[0].bins, stars[0].bins);
        assert!(parsed[1].bins.is_empty());
    }

    #[test]
    fn xp_spectra_refuses_malformed() {
        assert!(parse_xp_spectra_bin(&[0xCF, 0x86, 0x01]).is_none());
        assert!(parse_xp_spectra_bin(&[0xCF, 0x86, 0x02]).is_none());
        let bytes = write_xp_spectra_bin(
            0.0,
            &[XpStar {
                source_id: 1,
                ra: 0.0,
                dec: 0.0,
                plx_mas: 0.0,
                bins: vec![(1.0, 2.0, 3.0)],
            }],
        );
        assert!(parse_xp_spectra_bin(&bytes[..bytes.len() - 1]).is_none());
        let mut wrong_nbins = bytes.clone();
        wrong_nbins[47] = 2;
        assert!(parse_xp_spectra_bin(&wrong_nbins).is_none());
    }

    fn planck_nu(nu: f64, t: f64) -> f64 {
        const H: f64 = 6.626_070_15e-34;
        const K: f64 = 1.380_649e-23;
        let x = H * nu / (K * t);
        if x > 700.0 {
            return 0.0;
        }
        let b = 2.0 * H * nu.powi(3) / (C_LIGHT * C_LIGHT) * 1.0 / (x.exp() - 1.0);
        if b.is_finite() {
            b
        } else {
            0.0
        }
    }

    fn blackbody_bins(t: f64) -> Vec<(f64, f64, f64)> {
        let mut bins = Vec::new();
        let mut lam_nm = 320.0f64;
        while lam_nm <= 1100.0 {
            let lam_m = lam_nm * 1e-9;
            let nu = C_LIGHT / lam_m;
            let dnu = C_LIGHT / (lam_m * lam_m) * 20.0e-9;
            bins.push((nu, dnu, planck_nu(nu, t)));
            lam_nm += 20.0;
        }
        bins
    }

    #[test]
    fn parse_extinction_reads_the_embedded_table() {
        let table = parse_extinction(include_str!("kernels/ccm89_rv31.dat"));
        assert!(table.len() >= 30);
        assert!(
            table.windows(2).all(|w| w[0].0 < w[1].0),
            "must be sorted by lambda"
        );
    }

    #[test]
    fn extinction_is_normalized_at_v() {
        let a = extinction_at(549.45).unwrap();
        assert!(
            (a - 1.0).abs() < 1e-3,
            "A/A_V must be ~1 at the V band, got {}",
            a
        );
    }

    #[test]
    fn extinction_carries_the_2175_bump_and_a_monotone_ir() {
        let bump = extinction_at(217.5).unwrap();
        let optical = extinction_at(550.0).unwrap();
        assert!(
            bump > optical,
            "the 2175 A bump must exceed the optical value"
        );
        let a1000 = extinction_at(1000.0).unwrap();
        let a2000 = extinction_at(2000.0).unwrap();
        let a3000 = extinction_at(3000.0).unwrap();
        assert!(
            a1000 > a2000 && a2000 > a3000,
            "the IR must fall monotonically"
        );
    }

    #[test]
    fn extinction_clamps_to_the_table_ends() {
        let lo = extinction_at(50.0).unwrap();
        let hi = extinction_at(5000.0).unwrap();
        assert!(lo.is_finite() && hi.is_finite() && hi > 0.0);
    }
}
