pub const MAGIC: [u8; 4] = *b"RX1V";

pub const COMP_LUMINANCE: u32 = 1;
pub const COMP_MAX: u32 = 1;

pub const K_REFLECTED_CDSM2: f64 = 12.5;
pub const TAU_S: f64 = 60.0;

pub const FREQ_PHOTOPIC_HZ: f64 = 5.45e14;
pub const BIN_WIDTH_PHOTOPIC_HZ: f64 = 3.2e14;

pub const F_NUMBER_MIN: f64 = 1.7;
pub const F_NUMBER_MAX: f64 = 12.0;
pub const EXPOSURE_MAX_S: f64 = 30.0;
pub const ISO_MIN: f64 = 64.0;
pub const ISO_MAX: f64 = 25600.0;
pub const LUMINANCE_MIN_CDM2: f64 = 1e-4;
pub const LUMINANCE_MAX_CDM2: f64 = 1e5;

pub fn luminance(n: f64, t: f64, s: f64) -> Option<f64> {
    if !(n.is_finite() && (F_NUMBER_MIN..=F_NUMBER_MAX).contains(&n)) {
        return None;
    }
    if !(t.is_finite() && t > 0.0 && t <= EXPOSURE_MAX_S) {
        return None;
    }
    if !(s.is_finite() && (ISO_MIN..=ISO_MAX).contains(&s)) {
        return None;
    }
    let lv = K_REFLECTED_CDSM2 * n * n / (t * s);
    if !(lv.is_finite() && (LUMINANCE_MIN_CDM2..=LUMINANCE_MAX_CDM2).contains(&lv)) {
        return None;
    }
    Some(lv)
}

pub fn parse_f_number(raw: &str) -> Option<f64> {
    let s = raw.trim();
    let s = s
        .strip_prefix('F')
        .or_else(|| s.strip_prefix('f'))
        .unwrap_or(s);
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0)
}

pub fn parse_shutter(raw: &str) -> Option<f64> {
    let s = raw.trim();
    if let Some((num, den)) = s.split_once('/') {
        let n: f64 = num.trim().parse().ok()?;
        let d: f64 = den.trim().parse().ok()?;
        if !d.is_finite() || d == 0.0 || !n.is_finite() || n <= 0.0 {
            return None;
        }
        return Some(n / d);
    }
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0)
}

pub fn parse_iso(raw: &str) -> Option<f64> {
    let s = raw.trim();
    let s = s
        .strip_prefix("ISO")
        .or_else(|| s.strip_prefix("iso"))
        .unwrap_or(s);
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0)
}

const EXIF_TAG_EXPOSURE_TIME: u16 = 0x829A;
const EXIF_TAG_F_NUMBER: u16 = 0x829D;
const EXIF_TAG_ISO: u16 = 0x8827;
const EXIF_TAG_DATE_TIME_ORIGINAL: u16 = 0x9003;
const EXIF_TAG_EXIF_IFD: u16 = 0x8769;

const TIFF_TYPE_ASCII: u16 = 2;
const TIFF_TYPE_SHORT: u16 = 3;
const TIFF_TYPE_LONG: u16 = 4;
const TIFF_TYPE_RATIONAL: u16 = 5;

struct TiffEntry {
    tag: u16,
    field_type: u16,
    count: u32,
    value_pos: usize,
}

fn tiff_u16(data: &[u8], off: usize, little: bool) -> Option<u16> {
    let raw: [u8; 2] = data.get(off..off.checked_add(2)?)?.try_into().ok()?;
    Some(if little {
        u16::from_le_bytes(raw)
    } else {
        u16::from_be_bytes(raw)
    })
}

fn tiff_u32(data: &[u8], off: usize, little: bool) -> Option<u32> {
    let raw: [u8; 4] = data.get(off..off.checked_add(4)?)?.try_into().ok()?;
    Some(if little {
        u32::from_le_bytes(raw)
    } else {
        u32::from_be_bytes(raw)
    })
}

fn read_ifd(data: &[u8], off: usize, little: bool) -> Option<Vec<TiffEntry>> {
    let count = tiff_u16(data, off, little)? as usize;
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        let pos = off + 2 + i * 12;
        entries.push(TiffEntry {
            tag: tiff_u16(data, pos, little)?,
            field_type: tiff_u16(data, pos + 2, little)?,
            count: tiff_u32(data, pos + 4, little)?,
            value_pos: pos + 8,
        });
    }
    Some(entries)
}

fn entry_data<'a>(data: &'a [u8], e: &TiffEntry, little: bool, size: usize) -> Option<&'a [u8]> {
    let len = (e.count as usize).checked_mul(size)?;
    let base = if len <= 4 {
        e.value_pos
    } else {
        tiff_u32(data, e.value_pos, little)? as usize
    };
    data.get(base..base.checked_add(len)?)
}

fn entry_rational(data: &[u8], e: &TiffEntry, little: bool) -> Option<f64> {
    if e.field_type != TIFF_TYPE_RATIONAL || e.count != 1 {
        return None;
    }
    let raw = entry_data(data, e, little, 8)?;
    let num = tiff_u32(raw, 0, little)? as f64;
    let den = tiff_u32(raw, 4, little)? as f64;
    if den <= 0.0 {
        return None;
    }
    let v = num / den;
    if !(v.is_finite() && v > 0.0) {
        return None;
    }
    Some(v)
}

fn entry_short(data: &[u8], e: &TiffEntry, little: bool) -> Option<f64> {
    if e.field_type != TIFF_TYPE_SHORT || e.count == 0 {
        return None;
    }
    let raw = entry_data(data, e, little, 2)?;
    let v = tiff_u16(raw, 0, little)? as f64;
    if v <= 0.0 {
        return None;
    }
    Some(v)
}

fn entry_ascii<'a>(data: &'a [u8], e: &TiffEntry, little: bool) -> Option<&'a [u8]> {
    if e.field_type != TIFF_TYPE_ASCII {
        return None;
    }
    let raw = entry_data(data, e, little, 1)?;
    let end = match raw.iter().position(|&b| b == 0) {
        Some(p) => p,
        None => raw.len(),
    };
    Some(&raw[..end])
}

fn be_u16(data: &[u8], off: usize) -> Option<u16> {
    let raw: [u8; 2] = data.get(off..off + 2)?.try_into().ok()?;
    Some(u16::from_be_bytes(raw))
}

fn exif_tiff(jpeg: &[u8]) -> Option<&[u8]> {
    if jpeg.len() < 4 || jpeg[0] != 0xFF || jpeg[1] != 0xD8 {
        return None;
    }
    let mut pos = 2usize;
    while jpeg.get(pos) == Some(&0xFF) {
        let marker = *jpeg.get(pos + 1)?;
        pos += 2;
        match marker {
            0xD8 | 0xD9 => return None,
            0x01 | 0xD0..=0xD7 => {}
            _ => {
                let len = be_u16(jpeg, pos)? as usize;
                if len < 2 {
                    return None;
                }
                pos += 2;
                let seg_end = pos + len - 2;
                let seg = jpeg.get(pos..seg_end)?;
                if marker == 0xE1 && seg.starts_with(b"Exif\0\0") {
                    return seg.get(6..);
                }
                pos = seg_end;
            }
        }
    }
    None
}

fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month <= 2 { month + 9 } else { month - 3 };
    let doy = (153 * mp as i64 + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_to_unix(
    year: i64,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Option<f64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let secs = days_from_civil(year, month, day) * 86_400
        + hour as i64 * 3600
        + minute as i64 * 60
        + second as i64;
    Some(secs as f64)
}

fn parse_civil_utc(s: &[u8]) -> Option<f64> {
    if s.len() != 19 {
        return None;
    }
    if s.get(4) != Some(&b':')
        || s.get(7) != Some(&b':')
        || s.get(10) != Some(&b' ')
        || s.get(13) != Some(&b':')
        || s.get(16) != Some(&b':')
    {
        return None;
    }
    let d2 = |i: usize| -> Option<u32> {
        let hi = *s.get(i)?;
        let lo = *s.get(i + 1)?;
        if !(hi.is_ascii_digit() && lo.is_ascii_digit()) {
            return None;
        }
        Some((hi - b'0') as u32 * 10 + (lo - b'0') as u32)
    };
    let year = (d2(0)? * 100 + d2(2)?) as i64;
    let month = d2(5)?;
    let day = d2(8)?;
    let hour = d2(11)?;
    let minute = d2(14)?;
    let second = d2(17)?;
    civil_to_unix(year, month, day, hour, minute, second)
}

pub fn exif_exposure(bytes: &[u8]) -> Option<(f64, f64, f64, f64)> {
    let tiff = exif_tiff(bytes)?;
    let little = match tiff.get(0..2)? {
        b"II" => true,
        b"MM" => false,
        _ => return None,
    };
    if tiff_u16(tiff, 2, little)? != 42 {
        return None;
    }
    let ifd0 = read_ifd(tiff, tiff_u32(tiff, 4, little)? as usize, little)?;
    let exif_ifd: Option<Vec<TiffEntry>> = ifd0
        .iter()
        .find(|e| e.tag == EXIF_TAG_EXIF_IFD && e.field_type == TIFF_TYPE_LONG)
        .and_then(|e| tiff_u32(tiff, e.value_pos, little))
        .and_then(|off| read_ifd(tiff, off as usize, little));
    let find = |tag: u16| {
        ifd0.iter()
            .chain(exif_ifd.iter().flatten())
            .find(|e| e.tag == tag)
    };
    let t = entry_rational(tiff, find(EXIF_TAG_EXPOSURE_TIME)?, little)?;
    let n = entry_rational(tiff, find(EXIF_TAG_F_NUMBER)?, little)?;
    let s = entry_short(tiff, find(EXIF_TAG_ISO)?, little)?;
    let epoch =
        entry_ascii(tiff, find(EXIF_TAG_DATE_TIME_ORIGINAL)?, little).and_then(parse_civil_utc)?;
    Some((n, t, s, epoch))
}

pub fn write_bin(records: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, val, comp) in records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&val.to_le_bytes());
        buf.extend_from_slice(&comp.to_le_bytes());
    }
    buf
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if n > (bytes.len() - 8) / 20 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let val = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        off += 8;
        let comp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        if !(COMP_LUMINANCE..=COMP_MAX).contains(&comp) {
            return None;
        }
        out.push((t, val, comp));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminance_holds_the_exposure_equation() {
        let lv = luminance(2.8, 1.0 / 60.0, 100.0).unwrap();
        let expected = 12.5 * 2.8 * 2.8 / ((1.0 / 60.0) * 100.0);
        assert!((lv - expected).abs() < 1e-9, "{lv} vs {expected}");
    }

    #[test]
    fn luminance_skips_absent_readbacks() {
        assert!(luminance(0.0, 1.0 / 60.0, 100.0).is_none());
        assert!(luminance(f64::NAN, 1.0 / 60.0, 100.0).is_none());
        assert!(luminance(2.8, 0.0, 100.0).is_none());
        assert!(luminance(2.8, 31.0, 100.0).is_none());
        assert!(luminance(2.8, 1.0 / 60.0, 32.0).is_none());
        assert!(luminance(1.0, 1.0 / 60.0, 100.0).is_none());
    }

    #[test]
    fn luminance_skips_saturated_scenes() {
        assert!(luminance(12.0, 1.0 / 32000.0, 64.0).is_none());
        assert!(luminance(1.7, 30.0, 25600.0).is_none());
    }

    #[test]
    fn f_number_strips_the_sony_prefix() {
        assert_eq!(parse_f_number("F2.8"), Some(2.8));
        assert_eq!(parse_f_number("f4.0"), Some(4.0));
        assert_eq!(parse_f_number("5.6"), Some(5.6));
        assert!(parse_f_number("").is_none());
        assert!(parse_f_number("F0").is_none());
    }

    #[test]
    fn shutter_parses_fractions_and_decimals() {
        let s = parse_shutter("1/60").unwrap();
        assert!((s - 1.0 / 60.0).abs() < 1e-12);
        assert_eq!(parse_shutter("0.5"), Some(0.5));
        assert_eq!(parse_shutter("30"), Some(30.0));
        assert!(parse_shutter("1/0").is_none());
        assert!(parse_shutter("").is_none());
    }

    #[test]
    fn iso_strips_the_sony_prefix() {
        assert_eq!(parse_iso("ISO100"), Some(100.0));
        assert_eq!(parse_iso("3200"), Some(3200.0));
        assert!(parse_iso("").is_none());
    }

    #[test]
    fn reciprocity_holds_across_exposure_pairs_of_one_scene() {
        let lv_a = luminance(2.0, 1.0 / 100.0, 100.0).unwrap();
        let lv_b = luminance(4.0, 1.0 / 25.0, 100.0).unwrap();
        assert!((lv_a - lv_b).abs() < 1e-9, "{lv_a} vs {lv_b}");
        let lv_c = luminance(5.6, 1.0 / 8.0, 200.0).unwrap();
        let lv_d = luminance(2.8, 1.0 / 32.0, 200.0).unwrap();
        assert!((lv_c - lv_d).abs() < 1e-9, "{lv_c} vs {lv_d}");
    }

    #[test]
    fn bin_roundtrip() {
        let records = vec![(-220_000_000.0, 58.8, COMP_LUMINANCE)];
        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert!((parsed[0].1 - 58.8).abs() < 1e-9);
        assert_eq!(parsed[0].2, COMP_LUMINANCE);
    }

    #[test]
    fn bin_rejects_foreign_bytes() {
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"RX1Vabc").is_none());
        let bad = write_bin(&[(1.0, 1.0, 4)]);
        assert!(parse_bin(&bad).is_none());
    }

    fn wrap_exif(tiff: &[u8]) -> Vec<u8> {
        let mut jpeg = Vec::with_capacity(4 + 2 + 2 + 6 + tiff.len() + 2);
        jpeg.extend_from_slice(&[0xFF, 0xD8]);
        jpeg.extend_from_slice(&[0xFF, 0xE1]);
        jpeg.extend_from_slice(&((tiff.len() + 8) as u16).to_be_bytes());
        jpeg.extend_from_slice(b"Exif\0\0");
        jpeg.extend_from_slice(tiff);
        jpeg.extend_from_slice(&[0xFF, 0xD9]);
        jpeg
    }

    fn exif_jpeg(
        little: bool,
        exposure_num: u32,
        exposure_den: u32,
        f_num: u32,
        f_den: u32,
        iso: u16,
        date_time: &str,
    ) -> Vec<u8> {
        let dt = date_time.as_bytes();
        let payload = (8 + 2 + 4 * 12 + 4) as u32;
        let iso_value = if little {
            iso as u32
        } else {
            (iso as u32) << 16
        };
        let u16b = |v: u16| {
            if little {
                v.to_le_bytes()
            } else {
                v.to_be_bytes()
            }
        };
        let u32b = |v: u32| {
            if little {
                v.to_le_bytes()
            } else {
                v.to_be_bytes()
            }
        };
        let mut tiff = Vec::new();
        tiff.extend_from_slice(if little { b"II" } else { b"MM" });
        tiff.extend_from_slice(&u16b(42));
        tiff.extend_from_slice(&u32b(8));
        tiff.extend_from_slice(&u16b(4));
        let entry = |out: &mut Vec<u8>, tag: u16, field_type: u16, count: u32, value: u32| {
            out.extend_from_slice(&u16b(tag));
            out.extend_from_slice(&u16b(field_type));
            out.extend_from_slice(&u32b(count));
            out.extend_from_slice(&u32b(value));
        };
        entry(&mut tiff, 0x829A, 5, 1, payload);
        entry(&mut tiff, 0x829D, 5, 1, payload + 8);
        entry(&mut tiff, 0x8827, 3, 1, iso_value);
        entry(&mut tiff, 0x9003, 2, dt.len() as u32 + 1, payload + 16);
        tiff.extend_from_slice(&u32b(0));
        tiff.extend_from_slice(&u32b(exposure_num));
        tiff.extend_from_slice(&u32b(exposure_den));
        tiff.extend_from_slice(&u32b(f_num));
        tiff.extend_from_slice(&u32b(f_den));
        tiff.extend_from_slice(dt);
        tiff.push(0);
        wrap_exif(&tiff)
    }

    #[test]
    fn exif_exposure_reads_the_exposure_triplet_and_capture_time() {
        let jpeg = exif_jpeg(true, 1, 60, 28, 10, 100, "2026:09:27 10:30:15");
        let (n, t, s, epoch) = exif_exposure(&jpeg).unwrap();
        assert!((n - 2.8).abs() < 1e-12);
        assert!((t - 1.0 / 60.0).abs() < 1e-12);
        assert_eq!(s, 100.0);
        assert_eq!(epoch, 1_790_505_015.0);
    }

    #[test]
    fn exif_exposure_reads_big_endian_tiff() {
        let jpeg = exif_jpeg(false, 1, 60, 28, 10, 100, "2026:09:27 10:30:15");
        let (n, t, s, epoch) = exif_exposure(&jpeg).unwrap();
        assert!((n - 2.8).abs() < 1e-12);
        assert!((t - 1.0 / 60.0).abs() < 1e-12);
        assert_eq!(s, 100.0);
        assert_eq!(epoch, 1_790_505_015.0);
    }

    #[test]
    fn exif_exposure_reads_tags_from_the_exif_ifd() {
        let dt = b"2026:09:27 10:30:15";
        let payload = (8 + (2 + 12 + 4) + (2 + 4 * 12 + 4)) as u32;
        let mut tiff = Vec::new();
        tiff.extend_from_slice(b"II");
        tiff.extend_from_slice(&42u16.to_le_bytes());
        tiff.extend_from_slice(&8u32.to_le_bytes());
        tiff.extend_from_slice(&1u16.to_le_bytes());
        let entry = |out: &mut Vec<u8>, tag: u16, field_type: u16, count: u32, value: u32| {
            out.extend_from_slice(&tag.to_le_bytes());
            out.extend_from_slice(&field_type.to_le_bytes());
            out.extend_from_slice(&count.to_le_bytes());
            out.extend_from_slice(&value.to_le_bytes());
        };
        entry(&mut tiff, 0x8769, 4, 1, 8);
        tiff.extend_from_slice(&0u32.to_le_bytes());
        tiff.extend_from_slice(&4u16.to_le_bytes());
        entry(&mut tiff, 0x829A, 5, 1, payload);
        entry(&mut tiff, 0x829D, 5, 1, payload + 8);
        entry(&mut tiff, 0x8827, 3, 1, 125);
        entry(&mut tiff, 0x9003, 2, dt.len() as u32 + 1, payload + 16);
        tiff.extend_from_slice(&0u32.to_le_bytes());
        tiff.extend_from_slice(&1u32.to_le_bytes());
        tiff.extend_from_slice(&125u32.to_le_bytes());
        tiff.extend_from_slice(&28u32.to_le_bytes());
        tiff.extend_from_slice(&10u32.to_le_bytes());
        tiff.extend_from_slice(dt);
        tiff.push(0);
        let (n, t, s, epoch) = exif_exposure(&wrap_exif(&tiff)).unwrap();
        assert!((n - 2.8).abs() < 1e-12);
        assert!((t - 1.0 / 125.0).abs() < 1e-12);
        assert_eq!(s, 125.0);
        assert_eq!(epoch, 1_790_505_015.0);
    }

    #[test]
    fn exif_exposure_is_absent_without_a_capture() {
        assert!(exif_exposure(b"").is_none());
        assert!(exif_exposure(b"\xFF\xD8\xFF\xD9").is_none());
        assert!(exif_exposure(b"PK\x03\x04").is_none());
    }

    #[test]
    fn exif_exposure_is_absent_when_a_tag_is_missing() {
        let dt = b"2026:09:27 10:30:15";
        let payload = (8 + 2 + 3 * 12 + 4) as u32;
        let mut tiff = Vec::new();
        tiff.extend_from_slice(b"II");
        tiff.extend_from_slice(&42u16.to_le_bytes());
        tiff.extend_from_slice(&8u32.to_le_bytes());
        tiff.extend_from_slice(&3u16.to_le_bytes());
        for (tag, field_type, count, value) in [
            (0x829Au16, 5u16, 1u32, payload),
            (0x829D, 5, 1, payload + 8),
            (0x9003, 2, dt.len() as u32 + 1, payload + 16),
        ] {
            tiff.extend_from_slice(&tag.to_le_bytes());
            tiff.extend_from_slice(&field_type.to_le_bytes());
            tiff.extend_from_slice(&count.to_le_bytes());
            tiff.extend_from_slice(&value.to_le_bytes());
        }
        tiff.extend_from_slice(&0u32.to_le_bytes());
        tiff.extend_from_slice(&1u32.to_le_bytes());
        tiff.extend_from_slice(&60u32.to_le_bytes());
        tiff.extend_from_slice(&28u32.to_le_bytes());
        tiff.extend_from_slice(&10u32.to_le_bytes());
        tiff.extend_from_slice(dt);
        tiff.push(0);
        assert!(exif_exposure(&wrap_exif(&tiff)).is_none());
    }

    #[test]
    fn exif_exposure_rejects_a_zero_exposure_denominator() {
        let jpeg = exif_jpeg(true, 1, 0, 28, 10, 100, "2026:09:27 10:30:15");
        assert!(exif_exposure(&jpeg).is_none());
    }

    #[test]
    fn exif_exposure_rejects_a_malformed_capture_time() {
        let jpeg = exif_jpeg(true, 1, 60, 28, 10, 100, "2026:13:40 99:99:99");
        assert!(exif_exposure(&jpeg).is_none());
    }
}
