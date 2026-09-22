use super::*;

pub const MAGIC: [u8; 4] = *b"QAO1";

pub const SAMPLE_BYTES: usize = 24;

pub const COMP_FLUX: u32 = 1;
pub const COMP_MODEL: u32 = 2;
pub const COMP_GHOST: u32 = 3;

const EOCD_SIG: [u8; 4] = *b"PK\x05\x06";
const CD_SIG: [u8; 4] = *b"PK\x01\x02";
const LOCAL_SIG: [u8; 4] = *b"PK\x03\x04";

#[derive(Clone, Debug)]
pub struct ZipEntry {
    pub name: String,
    pub method: u16,
    pub comp_size: u64,
    pub local_offset: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuaoarSample {
    pub tdb: f64,
    pub flux: f64,
    pub model: Option<f64>,
    pub ghost: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuaoarRec {
    pub tdb: f64,
    pub val: f64,
    pub comp: u32,
    pub site: u32,
}

fn le16(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn le32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn find_eocd(data: &[u8]) -> Option<usize> {
    let min = data.len().saturating_sub(65_557);
    let mut i = data.len();
    while i >= min + 4 {
        if data[i - 4..i] == EOCD_SIG && data.len() - (i - 4) >= 22 {
            return Some(i - 4);
        }
        i -= 1;
    }
    None
}

pub fn zip_entries(data: &[u8]) -> Option<Vec<ZipEntry>> {
    let eocd = find_eocd(data)?;
    let count = le16(data, eocd + 10)? as usize;
    if count == 0xffff {
        return None;
    }
    let cd_size = le32(data, eocd + 12)? as usize;
    let cd_offset = le32(data, eocd + 16)? as usize;
    let cd_end = cd_offset.checked_add(cd_size)?;
    if cd_end > data.len() {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    let mut off = cd_offset;
    for _ in 0..count {
        if off + 46 > cd_end || data.get(off..off + 4)? != CD_SIG {
            return None;
        }
        let method = le16(data, off + 10)?;
        let comp_size = le32(data, off + 20)? as u64;
        let name_len = le16(data, off + 28)? as usize;
        let extra_len = le16(data, off + 30)? as usize;
        let comment_len = le16(data, off + 32)? as usize;
        let local_offset = le32(data, off + 42)? as u64;
        let name_start = off + 46;
        let name_end = name_start.checked_add(name_len)?;
        if name_end > cd_end {
            return None;
        }
        let name = String::from_utf8_lossy(data.get(name_start..name_end)?).into_owned();
        out.push(ZipEntry {
            name,
            method,
            comp_size,
            local_offset,
        });
        off = name_end.checked_add(extra_len)?.checked_add(comment_len)?;
    }
    Some(out)
}

pub fn zip_extract(data: &[u8], e: &ZipEntry) -> Option<Vec<u8>> {
    let off = e.local_offset as usize;
    if off + 30 > data.len() || data.get(off..off + 4)? != LOCAL_SIG {
        return None;
    }
    let name_len = le16(data, off + 26)? as usize;
    let extra_len = le16(data, off + 28)? as usize;
    let start = off
        .checked_add(30)?
        .checked_add(name_len)?
        .checked_add(extra_len)?;
    let end = start.checked_add(e.comp_size as usize)?;
    if end > data.len() {
        return None;
    }
    let raw = data.get(start..end)?;
    match e.method {
        0 => Some(raw.to_vec()),
        8 => crate::inflate::inflate(raw),
        _ => None,
    }
}

fn finite(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("nan") || t.eq_ignore_ascii_case("inf") {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn date_midnight_unix(yyyymmdd: &str) -> Option<f64> {
    let b = yyyymmdd.as_bytes();
    if b.len() != 8 || !b.iter().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let year = std::str::from_utf8(&b[0..4]).ok()?.parse::<i64>().ok()?;
    let month = std::str::from_utf8(&b[4..6]).ok()?.parse::<u32>().ok()?;
    let day = std::str::from_utf8(&b[6..8]).ok()?.parse::<u32>().ok()?;
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let dim = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        _ => return None,
    };
    if day == 0 || day > dim {
        return None;
    }
    let days = ymd_to_days(year, month, day)?;
    Some(days as f64 * 86400.0)
}

pub fn parse_filename(name: &str) -> Option<(&'static str, &str, &str)> {
    let stem = name.strip_prefix("data/")?.strip_suffix(".txt")?;
    let (date, rest) = stem.split_at_checked(8)?;
    if !date.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if let Some(site) = rest.strip_prefix("_models_") {
        Some(("models", date, site))
    } else if let Some(site) = rest.strip_prefix("_chi2_") {
        Some(("chi2", date, site))
    } else {
        rest.strip_prefix('_').map(|site| ("lc", date, site))
    }
}

pub fn site_code(site: &str) -> u32 {
    let mut code = 0u32;
    for (i, &c) in site.as_bytes().iter().take(4).enumerate() {
        code |= u32::from(c) << (8 * i);
    }
    code
}

pub fn site_name(code: u32) -> Option<String> {
    let cs = [
        (code & 0xff) as u8,
        ((code >> 8) & 0xff) as u8,
        ((code >> 16) & 0xff) as u8,
        ((code >> 24) & 0xff) as u8,
    ];
    let end = cs.iter().position(|&b| b == 0).unwrap_or(cs.len());
    if end == 0 || !cs[..end].iter().all(|b| b.is_ascii_graphic()) {
        return None;
    }
    String::from_utf8(cs[..end].to_vec()).ok()
}

pub fn parse_light_curve(
    text: &str,
    midnight_unix: f64,
    lsk: &LeapSeconds,
) -> Option<Vec<QuaoarSample>> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 3 {
            continue;
        }
        if cols[0].parse::<i64>().is_err() {
            continue;
        }
        let Some(t_sec) = finite(cols[1]) else {
            continue;
        };
        let Some(flux) = finite(cols[2]) else {
            continue;
        };
        let model = cols.get(3).and_then(|c| finite(c));
        let ghost = cols.get(4).and_then(|c| finite(c));
        let Some(tdb) = lsk.unix_to_tdb(midnight_unix + t_sec) else {
            continue;
        };
        out.push(QuaoarSample {
            tdb,
            flux,
            model,
            ghost,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn to_recs(samples: &[QuaoarSample], site: u32) -> Vec<QuaoarRec> {
    let mut out = Vec::with_capacity(samples.len() * 3);
    for s in samples {
        out.push(QuaoarRec {
            tdb: s.tdb,
            val: s.flux,
            comp: COMP_FLUX,
            site,
        });
        if let Some(m) = s.model {
            out.push(QuaoarRec {
                tdb: s.tdb,
                val: m,
                comp: COMP_MODEL,
                site,
            });
        }
        if let Some(g) = s.ghost {
            out.push(QuaoarRec {
                tdb: s.tdb,
                val: g,
                comp: COMP_GHOST,
                site,
            });
        }
    }
    out
}

pub fn write_bin(records: &[QuaoarRec]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(8 + records.len() * SAMPLE_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        if !r.tdb.is_finite() || !r.val.is_finite() || r.comp > COMP_GHOST {
            return None;
        }
        buf.extend_from_slice(&r.tdb.to_le_bytes());
        buf.extend_from_slice(&r.val.to_le_bytes());
        buf.extend_from_slice(&r.comp.to_le_bytes());
        buf.extend_from_slice(&r.site.to_le_bytes());
    }
    Some(buf)
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<QuaoarRec>> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + n * SAMPLE_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let f64_at = |o: usize| {
            data.get(o..o + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
        };
        let tdb = f64_at(off)?;
        let val = f64_at(off + 8)?;
        let comp = u32::from_le_bytes(data.get(off + 16..off + 20)?.try_into().ok()?);
        let site = u32::from_le_bytes(data.get(off + 20..off + 24)?.try_into().ok()?);
        if !tdb.is_finite() || !val.is_finite() || comp > COMP_GHOST {
            return None;
        }
        out.push(QuaoarRec {
            tdb,
            val,
            comp,
            site,
        });
        off += SAMPLE_BYTES;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lsk() -> LeapSeconds {
        crate::lsk::parse(
            "KPL/LSK\nDELTET/DELTA_T_A       =   32.184\nDELTET/DELTA_AT        = ( 10,   @1972-JAN-1,\n 37,   @2017-JAN-1 )\n",
        )
        .unwrap()
    }

    fn tiny_zip(name: &str, content: &[u8]) -> Vec<u8> {
        let mut buf = Vec::new();
        let name_b = name.as_bytes();
        buf.extend_from_slice(&LOCAL_SIG);
        buf.extend_from_slice(&20u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&(content.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(content.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(name_b.len() as u16).to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(name_b);
        buf.extend_from_slice(content);
        let cd_start = buf.len();
        buf.extend_from_slice(&CD_SIG);
        buf.extend_from_slice(&0x031eu16.to_le_bytes());
        buf.extend_from_slice(&20u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&(content.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(content.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(name_b.len() as u16).to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(name_b);
        let cd_size = (buf.len() - cd_start) as u32;
        buf.extend_from_slice(&EOCD_SIG);
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&1u16.to_le_bytes());
        buf.extend_from_slice(&1u16.to_le_bytes());
        buf.extend_from_slice(&cd_size.to_le_bytes());
        buf.extend_from_slice(&(cd_start as u32).to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf
    }

    #[test]
    fn zip_entries_and_extract_read_a_stored_member() {
        let zip = tiny_zip("data/20110211_Scaggsville.txt", b"0 100.0 1.02 0.98 0.01\n");
        let entries = zip_entries(&zip).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "data/20110211_Scaggsville.txt");
        assert_eq!(entries[0].method, 0);
        let raw = zip_extract(&zip, &entries[0]).unwrap();
        assert_eq!(raw, b"0 100.0 1.02 0.98 0.01\n");
    }

    #[test]
    fn zip_entries_refuse_truncated_and_foreign() {
        assert!(zip_entries(b"not a zip").is_none());
        let zip = tiny_zip("data/20110211_Scaggsville.txt", b"x");
        assert!(zip_entries(&zip[..zip.len() - 1]).is_none());
    }

    #[test]
    fn parse_filename_names_the_three_member_kinds() {
        assert_eq!(
            parse_filename("data/20230715_SLN.txt"),
            Some(("lc", "20230715", "SLN"))
        );
        assert_eq!(
            parse_filename("data/20180726_models_Gault.txt"),
            Some(("models", "20180726", "Gault"))
        );
        assert_eq!(
            parse_filename("data/20220809_chi2_TOHUKU.txt"),
            Some(("chi2", "20220809", "TOHUKU"))
        );
        assert_eq!(parse_filename("README.md"), None);
        assert_eq!(parse_filename("data/curve_SAfrica.dat"), None);
    }

    #[test]
    fn date_midnight_unix_reads_the_calendar_date() {
        let unix = date_midnight_unix("20110211").unwrap();
        assert!((unix / 86400.0 - 15016.0).abs() < 1e-9);
        assert!(date_midnight_unix("2011").is_none());
        assert!(date_midnight_unix("20111301").is_none());
    }

    #[test]
    fn parse_light_curve_reads_measured_columns_and_drops_absent() {
        let body = "0 100.0 1.02 0.98 0.01\n1 110.0 0.95 nan 0.02\n2 120.0 0.88 0.90\n";
        let unix = date_midnight_unix("20230715").unwrap();
        let rows = parse_light_curve(body, unix, &lsk()).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].flux, 1.02);
        assert_eq!(rows[0].model, Some(0.98));
        assert_eq!(rows[0].ghost, Some(0.01));
        assert_eq!(rows[1].model, None);
        assert_eq!(rows[1].ghost, Some(0.02));
        assert_eq!(rows[2].ghost, None);
        let expect = lsk().unix_to_tdb(unix + 100.0).unwrap();
        assert!((rows[0].tdb - expect).abs() < 1e-9);
    }

    #[test]
    fn parse_light_curve_rejects_header_and_void() {
        let unix = date_midnight_unix("20230715").unwrap();
        assert!(parse_light_curve("frame time flux\n", unix, &lsk()).is_none());
        assert!(parse_light_curve("", unix, &lsk()).is_none());
    }

    #[test]
    fn site_code_roundtrips_four_ascii_chars() {
        assert_eq!(site_name(site_code("SOAR")).as_deref(), Some("SOAR"));
        assert_eq!(site_name(site_code("Speculoos")).as_deref(), Some("Spec"));
        assert_eq!(site_name(0), None);
        assert_eq!(site_name(site_code("Gault")).as_deref(), Some("Gaul"));
    }

    #[test]
    fn to_recs_emits_one_record_per_present_component() {
        let samples = vec![QuaoarSample {
            tdb: 8.0e8,
            flux: 1.02,
            model: Some(0.98),
            ghost: Some(0.01),
        }];
        let recs = to_recs(&samples, site_code("SOAR"));
        assert_eq!(recs.len(), 3);
        assert_eq!(recs[0].comp, COMP_FLUX);
        assert_eq!(recs[1].comp, COMP_MODEL);
        assert_eq!(recs[2].comp, COMP_GHOST);
        assert_eq!(recs[1].val, 0.98);
        assert_eq!(recs[0].site, site_code("SOAR"));
    }

    #[test]
    fn bin_roundtrip_and_rejections() {
        let recs = vec![
            QuaoarRec {
                tdb: 8.0e8,
                val: 1.02,
                comp: COMP_FLUX,
                site: site_code("SOAR"),
            },
            QuaoarRec {
                tdb: 8.0e8 + 10.0,
                val: 0.98,
                comp: COMP_MODEL,
                site: site_code("SOAR"),
            },
        ];
        let bytes = write_bin(&recs).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[1].val, 0.98);
        assert_eq!(parsed[0].comp, COMP_FLUX);
        assert!(parse_bin(b"X").is_none());
        assert!(parse_bin(b"QAO1abc").is_none());
        let bad = vec![QuaoarRec {
            tdb: f64::NAN,
            val: 1.0,
            comp: COMP_FLUX,
            site: 0,
        }];
        assert!(write_bin(&bad).is_none());
    }
}
