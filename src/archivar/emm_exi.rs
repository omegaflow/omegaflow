use crate::archivar::fits::{FitsHeader, FitsImage, FitsTable};
use crate::archivar::inflate;

pub const COMP_COUNT: u32 = 0;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_COUNT => Some("emm_exi_count"),
        _ => None,
    }
}

fn bunit_declares_count(bunit: Option<&str>) -> bool {
    match bunit {
        None => true,
        Some(s) => {
            let t = s.trim().to_ascii_lowercase();
            t.contains("dn") || t.contains("count")
        }
    }
}

fn iso8601_unix(s: &str) -> Option<f64> {
    let s = s.trim();
    let (date, time) = s.split_once('T').or_else(|| s.split_once(' '))?;
    let mut dp = date.split('-');
    let year: i64 = dp.next()?.parse().ok()?;
    let month: u32 = dp.next()?.parse().ok()?;
    let day: u32 = dp.next()?.parse().ok()?;
    let time = time.trim_end_matches(['Z', 'z']);
    let mut tp = time.split(':');
    let hour: f64 = tp.next()?.parse().ok()?;
    let minute: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let second: f64 = tp.next().unwrap_or("0").parse().ok()?;
    let days = crate::archivar::ymd_to_days(year, month, day)? as f64;
    let unix = days * 86400.0 + hour * 3600.0 + minute * 60.0 + second;
    if unix.is_finite() { Some(unix) } else { None }
}

fn fits_member(data: &[u8]) -> Option<Vec<u8>> {
    if data.starts_with(&[0x1f, 0x8b]) {
        return inflate::gunzip(data);
    }
    if data.starts_with(b"SIMPLE") {
        return Some(data.to_vec());
    }
    None
}

fn fits_members(bytes: &[u8]) -> Option<Vec<Vec<u8>>> {
    let mut docs: Vec<Vec<u8>> = Vec::new();
    if bytes.starts_with(b"PK\x03\x04") {
        inflate::zip_members(bytes, |_name, data| {
            if let Some(doc) = fits_member(data) {
                docs.push(doc);
            }
        })?;
    } else {
        let tar_bytes = if bytes.starts_with(&[0x1f, 0x8b]) {
            inflate::gunzip(bytes)?
        } else {
            bytes.to_vec()
        };
        for member in inflate::tar_members(&tar_bytes)? {
            let data = tar_bytes.get(member.start..member.end)?;
            if let Some(doc) = fits_member(data) {
                docs.push(doc);
            }
        }
    }
    if docs.is_empty() { None } else { Some(docs) }
}

fn sum_finite(buf: &[u8], img: &FitsImage) -> (f64, usize) {
    let mut sum = 0.0f64;
    let mut count = 0usize;
    for b in 0..img.dims[2] {
        for y in 0..img.dims[1] {
            for x in 0..img.dims[0] {
                if let Some(v) = img.value_f64(buf, [x, y, b]).filter(|v| v.is_finite()) {
                    sum += v;
                    count += 1;
                }
            }
        }
    }
    (sum, count)
}

fn sci_mean(doc: &[u8]) -> Option<(f64, f64)> {
    let (primary, _) = FitsHeader::parse(doc, 0)?;
    let t = primary.str_unescaped("DATE-OBS")?;
    let t = iso8601_unix(t.trim())?;
    if !t.is_finite() {
        return None;
    }
    let mut off = 0usize;
    for _ in 0..32 {
        let (header, _) = FitsHeader::parse(doc, off)?;
        if header.value("XTENSION") == Some("'BINTABLE'") {
            let (_, next) = FitsTable::parse(doc, off)?;
            if next <= off || next >= doc.len() {
                return None;
            }
            off = next;
            continue;
        }
        let (img, next) = FitsImage::parse(doc, off)?;
        let is_sci = header
            .str_unescaped("EXTNAME")
            .map(|s| s.trim().eq_ignore_ascii_case("SCI"))
            .unwrap_or(false);
        if is_sci {
            if img.dims[0] == 0 || img.dims[1] == 0 {
                return None;
            }
            if !bunit_declares_count(header.str_unescaped("BUNIT").as_deref()) {
                return None;
            }
            let (sum, count) = sum_finite(doc, &img);
            if count == 0 {
                return None;
            }
            return Some((t, sum / count as f64));
        }
        if next <= off || next >= doc.len() {
            return None;
        }
        off = next;
    }
    None
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let mut out = Vec::new();
    for doc in fits_members(bytes)? {
        if let Some((t, mean)) = sci_mean(&doc) {
            out.push((t, mean, COMP_COUNT));
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        let n = v.len().min(68);
        card[10..10 + n].copy_from_slice(&v[..n]);
        card
    }

    fn synth_with_bunit(bunit: Option<&str>) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let mut primary: Vec<u8> = Vec::new();
        primary.extend_from_slice(&card("SIMPLE", "T"));
        primary.extend_from_slice(&card("BITPIX", "8"));
        primary.extend_from_slice(&card("NAXIS", "0"));
        primary.extend_from_slice(&card("DATE-OBS", "'2023-01-05T12:21:15'"));
        primary.extend_from_slice(&card("END", ""));
        while !primary.len().is_multiple_of(2880) {
            primary.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&primary);

        let mut sci: Vec<u8> = Vec::new();
        sci.extend_from_slice(&card("XTENSION", "'IMAGE'"));
        sci.extend_from_slice(&card("BITPIX", "16"));
        sci.extend_from_slice(&card("NAXIS", "2"));
        sci.extend_from_slice(&card("NAXIS1", "2"));
        sci.extend_from_slice(&card("NAXIS2", "2"));
        sci.extend_from_slice(&card("EXTNAME", "'SCI'"));
        if let Some(b) = bunit {
            sci.extend_from_slice(&card("BUNIT", &format!("'{b}'")));
        }
        sci.extend_from_slice(&card("END", ""));
        while !sci.len().is_multiple_of(2880) {
            sci.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&sci);
        for v in [1i16, 2, 3, 4] {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn sci_mean_reads_date_obs_and_pixel_mean() {
        let doc = synth_with_bunit(Some("Calibrated DN"));
        let (t, mean) = sci_mean(&doc).unwrap();
        assert_eq!(t, iso8601_unix("2023-01-05T12:21:15").unwrap());
        assert_eq!(mean, 2.5);
    }

    #[test]
    fn sci_mean_refuses_a_non_count_bunit() {
        let doc = synth_with_bunit(Some("W/m2/sr/um"));
        assert!(sci_mean(&doc).is_none());
    }

    #[test]
    fn parse_series_refuses_bytes_without_a_fits_member() {
        assert!(parse_series(b"not an archive").is_none());
        assert!(parse_series(b"").is_none());
    }
}
