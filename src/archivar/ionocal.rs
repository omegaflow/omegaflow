
pub const SAMPLES_PER_SPAN: usize = 64;
pub const PACK_MAGIC: [u8; 4] = *b"GION";
pub const PACK_ENTRY_BYTES: usize = 96;
pub const COMP_DELAY: u32 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct IonocalRecord {
    pub coeffs: Vec<f64>,
    pub from_unix: f64,
    pub to_unix: f64,
    pub station: i64,
    pub scid: i64,
    pub chpart: bool,
    pub doprng: bool,
}

#[derive(Clone, Debug, Default)]
pub struct IonocalParse {
    pub records: Vec<IonocalRecord>,
    pub broken: usize,
}

#[derive(Clone, Debug)]
pub struct PackedIonocalFile {
    pub name: String,
    pub sha256: [u8; 32],
    pub record_count: u32,
    pub bytes: Vec<u8>,
}

pub fn full_year(two: i64) -> i64 {
    if two < 70 { 2000 + two } else { 1900 + two }
}

fn find_sub(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    hay.windows(needle.len()).position(|w| w == needle)
}

fn two_digits(s: &[u8], p: &mut usize) -> Option<i64> {
    if *p + 2 > s.len() {
        return None;
    }
    let d0 = s[*p].checked_sub(b'0').filter(|d| *d <= 9)?;
    let d1 = s[*p + 1].checked_sub(b'0').filter(|d| *d <= 9)?;
    *p += 2;
    Some(i64::from(d0) * 10 + i64::from(d1))
}

fn parse_float(tok: &[u8]) -> Option<f64> {
    let t = tok.iter().filter(|b| !b.is_ascii_whitespace()).copied();
    let mut s = String::with_capacity(tok.len() + 1);
    for b in t {
        s.push(b as char);
    }
    if s.starts_with('.') {
        s.insert(0, '0');
    } else if s.starts_with("+.") || s.starts_with("-.") {
        s.insert(1, '0');
    }
    s.parse().ok()
}

fn parse_utc(s: &[u8]) -> Option<f64> {
    let mut p = 0usize;
    let yy = two_digits(s, &mut p)?;
    if s.get(p) != Some(&b'/') {
        return None;
    }
    p += 1;
    let mm = two_digits(s, &mut p)?;
    if s.get(p) != Some(&b'/') {
        return None;
    }
    p += 1;
    let dd = two_digits(s, &mut p)?;
    if s.get(p) != Some(&b',') {
        return None;
    }
    p += 1;
    let hh = two_digits(s, &mut p)?;
    if s.get(p) != Some(&b':') {
        return None;
    }
    p += 1;
    let mi = two_digits(s, &mut p)?;
    let mut sec = 0.0f64;
    if s.get(p) == Some(&b':') {
        p += 1;
        sec = two_digits(s, &mut p)? as f64;
        if s.get(p) == Some(&b'.') {
            p += 1;
            let f0 = p;
            while s.get(p).is_some_and(|b| b.is_ascii_digit()) {
                p += 1;
            }
            let frac = parse_float(&s[f0..p])?;
            sec += frac / 10f64.powi((p - f0) as i32);
        }
    }
    let year = full_year(yy);
    let days = crate::lsk::days_from_civil(year, mm, dd)?;
    Some(days as f64 * 86400.0 + hh as f64 * 3600.0 + mi as f64 * 60.0 + sec)
}

fn strip_comments(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            out.push(b'\n');
            i += usize::from(i < bytes.len());
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

fn parse_command(cmd: &[u8]) -> Option<IonocalRecord> {
    let dtype_end = find_sub(&cmd[7..], b")")? + 7;
    let doprng = &cmd[7..dtype_end] == b"DOPRNG";

    let by_rel = find_sub(&cmd[dtype_end..], b"BY")? + dtype_end;
    let mut s = by_rel + 2;
    while s < cmd.len() && cmd[s].is_ascii_whitespace() {
        s += 1;
    }
    let spec_start = s;
    while s < cmd.len() && cmd[s].is_ascii_alphabetic() {
        s += 1;
    }
    let spec = &cmd[spec_start..s];
    if spec != b"NRMPOW" && spec != b"DNRMPOW" {
        return None;
    }
    while s < cmd.len() && cmd[s].is_ascii_whitespace() {
        s += 1;
    }
    if cmd.get(s) != Some(&b'(') {
        return None;
    }
    s += 1;
    let rp = find_sub(&cmd[s..], b")")? + s;
    let mut coeffs = Vec::new();
    for tok in cmd[s..rp].split(|b| *b == b',') {
        coeffs.push(parse_float(tok)?);
    }
    if coeffs.is_empty() {
        return None;
    }

    let m_rel = find_sub(&cmd[rp..], b"MODEL(")? + rp + 6;
    let m_end = find_sub(&cmd[m_rel..], b")")? + m_rel;
    let chpart = &cmd[m_rel..m_end] == b"CHPART";

    let f_rel = find_sub(&cmd[m_end..], b"FROM(")? + m_end + 5;
    let f_end = find_sub(&cmd[f_rel..], b")")? + f_rel;
    let from_unix = parse_utc(&cmd[f_rel..f_end])?;

    let t_rel = find_sub(&cmd[f_end..], b"TO(")? + f_end + 3;
    let t_end = find_sub(&cmd[t_rel..], b")")? + t_rel;
    let to_unix = parse_utc(&cmd[t_rel..t_end])?;

    let d_rel = find_sub(&cmd[t_end..], b"DSN(")? + t_end + 4;
    let d_end = find_sub(&cmd[d_rel..], b")")? + d_rel;
    let station = parse_station(&cmd[d_rel..d_end])?;

    let sc_rel = find_sub(&cmd[d_end..], b"SCID(")? + d_end + 5;
    let sc_end = find_sub(&cmd[sc_rel..], b")")? + sc_rel;
    let scid = parse_uint(&cmd[sc_rel..sc_end])?;

    Some(IonocalRecord {
        coeffs,
        from_unix,
        to_unix,
        station,
        scid,
        chpart,
        doprng,
    })
}

fn parse_station(s: &[u8]) -> Option<i64> {
    let mut t = s;
    if t.first() == Some(&b'C') {
        t = &t[1..];
    }
    parse_uint(t)
}

fn parse_uint(s: &[u8]) -> Option<i64> {
    if s.is_empty() || s.iter().any(|b| !b.is_ascii_digit()) {
        return None;
    }
    std::str::from_utf8(s).ok()?.parse().ok()
}

pub fn parse_records(text: &[u8]) -> IonocalParse {
    let clean = strip_comments(text);
    let mut out = IonocalParse::default();
    let mut pos = 0usize;
    while pos < clean.len() {
        let Some(start) = find_sub(&clean[pos..], b"ADJUST(") else {
            break;
        };
        let start = pos + start;
        let Some(sc_rel) = find_sub(&clean[start..], b"SCID(") else {
            out.broken += 1;
            pos = start + 6;
            continue;
        };
        let scid = start + sc_rel;
        let Some(rp_rel) = find_sub(&clean[scid..], b")") else {
            out.broken += 1;
            pos = start + 6;
            continue;
        };
        let rp = scid + rp_rel;
        if rp + 1 >= clean.len() || clean[rp + 1] != b'.' {
            out.broken += 1;
            pos = start + 6;
            continue;
        }
        match parse_command(&clean[start..=rp + 1]) {
            Some(r) => out.records.push(r),
            None => out.broken += 1,
        }
        pos = rp + 2;
    }
    out
}

fn horner(c: &[f64], x: f64) -> f64 {
    let mut v = 0.0;
    for &k in c.iter().rev() {
        v = v * x + k;
    }
    v
}

pub fn series(records: &[IonocalRecord], lsk: &crate::lsk::LeapSeconds) -> Vec<(f64, f64, u32)> {
    let mut out = Vec::new();
    for rec in records {
        if !rec.doprng || !rec.chpart || rec.coeffs.is_empty() {
            continue;
        }
        if !rec.from_unix.is_finite() || !rec.to_unix.is_finite() || rec.to_unix <= rec.from_unix {
            continue;
        }
        let Some(t0) = lsk.unix_to_tdb(rec.from_unix) else {
            continue;
        };
        let Some(t1) = lsk.unix_to_tdb(rec.to_unix) else {
            continue;
        };
        if t1 <= t0 {
            continue;
        }
        for k in 0..SAMPLES_PER_SPAN {
            let x = 2.0 * (k as f64) / ((SAMPLES_PER_SPAN - 1) as f64) - 1.0;
            let v = horner(&rec.coeffs, x);
            if !v.is_finite() {
                continue;
            }
            out.push((t0 + (x + 1.0) * 0.5 * (t1 - t0), v, COMP_DELAY));
        }
    }
    out
}

pub fn write_bin(files: &[PackedIonocalFile]) -> Vec<u8> {
    let count = files.len();
    let data_start = 8 + count * PACK_ENTRY_BYTES;
    let mut bin = vec![0u8; data_start];
    bin[0..4].copy_from_slice(&PACK_MAGIC);
    bin[4..8].copy_from_slice(&(count as u32).to_le_bytes());
    let mut offset = data_start as u64;
    for (i, f) in files.iter().enumerate() {
        let base = 8 + i * PACK_ENTRY_BYTES;
        let nameb = f.name.as_bytes();
        let n = nameb.len().min(32);
        bin[base..base + n].copy_from_slice(&nameb[..n]);
        bin[base + 32..base + 64].copy_from_slice(&f.sha256);
        bin[base + 64..base + 68].copy_from_slice(&f.record_count.to_le_bytes());
        bin[base + 72..base + 80].copy_from_slice(&offset.to_le_bytes());
        bin[base + 80..base + 88].copy_from_slice(&(f.bytes.len() as u64).to_le_bytes());
        offset += f.bytes.len() as u64;
    }
    for f in files {
        bin.extend_from_slice(&f.bytes);
    }
    bin
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<PackedIonocalFile>> {
    if data.len() < 8 || &data[0..4] != PACK_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() < 8 + count * PACK_ENTRY_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * PACK_ENTRY_BYTES;
        let name_end = data[base..base + 32]
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(32);
        let name = String::from_utf8(data[base..base + name_end].to_vec()).ok()?;
        let mut sha256 = [0u8; 32];
        sha256.copy_from_slice(&data[base + 32..base + 64]);
        let record_count = u32::from_le_bytes(data[base + 64..base + 68].try_into().ok()?);
        let data_offset = u64::from_le_bytes(data[base + 72..base + 80].try_into().ok()?) as usize;
        let data_length = u64::from_le_bytes(data[base + 80..base + 88].try_into().ok()?) as usize;
        if data_offset + data_length > data.len() {
            return None;
        }
        out.push(PackedIonocalFile {
            name,
            sha256,
            record_count,
            bytes: data[data_offset..data_offset + data_length].to_vec(),
        });
    }
    Some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let files = parse_bin(bytes)?;
    let lsk = crate::archivar::membrane::embedded_lsk()?;
    let mut out = Vec::new();
    for f in &files {
        let parsed = parse_records(&f.bytes);
        out.extend(series(&parsed.records, &lsk));
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_DELAY => Some("galileo_ionocal_delay_m"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEASURED_2003032: &[u8] = b"ADJUST(DOPRNG)BY NRMPOW(   0.7048,   0.0938,   0.8502,  -0.2993,   1.0217,      \n   0.0260)                                                         MODEL(CHPART)\nFROM(03/02/01,01:21)TO(03/02/01,15:01)DSN(C10)SCID(77).    #S20 ADJ 030203 11:19\n";

    const MEASURED_SINGLE_LINE: &[u8] = b"ADJUST(DOPRNG)BY NRMPOW(1.0,-3.0773)MODEL(CHPART)FROM(03/09/12,12:18:00)TO(03/09/13,01:19)DSN(C10)SCID(77). #S01\n";

    const MEASURED_TEN_COEFFS: &[u8] = b"ADJUST(DOPRNG)BY NRMPOW(2.2125, .5151, 1.4436, 1.3388, 1.5204, 4.7811,-1.3726,\n-5.9940,-.1180, 1.4368)MODEL(CHPART)\nFROM(03/09/30,18:36:00)TO(03/10/01,05:38)DSN(C40)SCID(77). #S03\n";

    #[test]
    fn parses_measured_record_with_wrapped_coefficients() {
        let p = parse_records(MEASURED_2003032);
        assert_eq!(p.broken, 0);
        assert_eq!(p.records.len(), 1);
        let r = &p.records[0];
        assert_eq!(
            r.coeffs,
            vec![0.7048, 0.0938, 0.8502, -0.2993, 1.0217, 0.0260]
        );
        assert!(r.chpart && r.doprng);
        assert_eq!(r.station, 10);
        assert_eq!(r.scid, 77);
        let days = crate::lsk::days_from_civil(2003, 2, 1).unwrap();
        assert_eq!(r.from_unix, days as f64 * 86400.0 + 4860.0);
        assert_eq!(r.to_unix, r.from_unix + 49200.0);
    }

    #[test]
    fn parses_single_line_command_with_seconds() {
        let p = parse_records(MEASURED_SINGLE_LINE);
        assert_eq!(p.broken, 0);
        assert_eq!(p.records.len(), 1);
        let r = &p.records[0];
        assert_eq!(r.coeffs, vec![1.0, -3.0773]);
        assert_eq!(r.station, 10);
        let days = crate::lsk::days_from_civil(2003, 9, 12).unwrap();
        assert_eq!(
            r.from_unix,
            days as f64 * 86400.0 + 12.0 * 3600.0 + 18.0 * 60.0
        );
    }

    #[test]
    fn parses_ten_coefficients_and_bare_fractions() {
        let p = parse_records(MEASURED_TEN_COEFFS);
        assert_eq!(p.broken, 0);
        let r = &p.records[0];
        assert_eq!(r.coeffs.len(), 10);
        assert_eq!(r.coeffs[1], 0.5151);
        assert_eq!(r.coeffs[8], -0.118);
        assert_eq!(r.coeffs[3], 1.3388);
    }

    #[test]
    fn full_year_maps_two_digit_years() {
        assert_eq!(full_year(89), 1989);
        assert_eq!(full_year(03), 2003);
        assert_eq!(full_year(68), 2068);
        assert_eq!(full_year(69), 1969);
    }

    #[test]
    fn broken_command_is_counted_not_fabricated() {
        let p = parse_records(b"ADJUST(DOPRNG)BY NRMPOW(1.0)MODEL(CHPART)\n");
        assert_eq!(p.broken, 1);
        assert!(p.records.is_empty());
    }

    #[test]
    fn non_chpart_model_stays_out_of_series() {
        let p = parse_records(
            b"ADJUST(ALL)BY NRMPOW(1.0, 2.0)MODEL(WET NUPART)FROM(06/05/01,03:00)TO(06/05/01,09:00)DSN(C10)SCID(77).\n",
        );
        assert_eq!(p.broken, 0);
        assert_eq!(p.records.len(), 1);
        assert!(!p.records[0].chpart);
        let lsk = crate::archivar::membrane::embedded_lsk().unwrap();
        assert!(series(&p.records, &lsk).is_empty());
    }

    #[test]
    fn series_grid_maps_span_endpoints_and_evaluates_polynomial() {
        let p = parse_records(MEASURED_2003032);
        let lsk = crate::archivar::membrane::embedded_lsk().unwrap();
        let rows = series(&p.records, &lsk);
        assert_eq!(rows.len(), SAMPLES_PER_SPAN);
        let t0 = lsk.unix_to_tdb(p.records[0].from_unix).unwrap();
        let t1 = lsk.unix_to_tdb(p.records[0].to_unix).unwrap();
        assert!((rows[0].0 - t0).abs() < 1e-6);
        assert!((rows[SAMPLES_PER_SPAN - 1].0 - t1).abs() < 1e-6);
        let c = &p.records[0].coeffs;
        let v_at_minus_one: f64 = c
            .iter()
            .enumerate()
            .map(|(i, k)| if i % 2 == 0 { *k } else { -*k })
            .sum();
        let v_at_plus_one: f64 = c.iter().sum();
        assert!((rows[0].1 - v_at_minus_one).abs() < 1e-9);
        assert!((rows[SAMPLES_PER_SPAN - 1].1 - v_at_plus_one).abs() < 1e-9);
        assert_eq!(rows[0].2, COMP_DELAY);
    }

    #[test]
    fn gion_bin_roundtrips() {
        let file = PackedIonocalFile {
            name: "gll_rss_2003032t0121_dssmm_ion.txt".to_string(),
            sha256: [7u8; 32],
            record_count: 85,
            bytes: MEASURED_2003032.to_vec(),
        };
        let bin = write_bin(&[file]);
        let parsed = parse_bin(&bin).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "gll_rss_2003032t0121_dssmm_ion.txt");
        assert_eq!(parsed[0].record_count, 85);
        assert_eq!(parsed[0].sha256, [7u8; 32]);
        assert_eq!(parsed[0].bytes, MEASURED_2003032);
        assert!(parse_bin(&bin[..bin.len() - 1]).is_none());
        assert!(parse_bin(b"GION").is_none());
        assert!(parse_bin(b"XXXX").is_none());
    }

    #[test]
    fn parse_series_decodes_the_asset() {
        let file = PackedIonocalFile {
            name: "gll_rss_2003032t0121_dssmm_ion.txt".to_string(),
            sha256: [7u8; 32],
            record_count: 1,
            bytes: MEASURED_2003032.to_vec(),
        };
        let bin = write_bin(&[file]);
        let rows = parse_series(&bin).unwrap();
        assert_eq!(rows.len(), SAMPLES_PER_SPAN);
        assert_eq!(rows[0].2, COMP_DELAY);
        assert!(rows[0].1 > 0.0);
    }

    #[test]
    fn parse_series_none_when_no_carried_delay() {
        let file = PackedIonocalFile {
            name: "empty.txt".to_string(),
            sha256: [7u8; 32],
            record_count: 0,
            bytes: b"# comment only\n".to_vec(),
        };
        let bin = write_bin(&[file]);
        assert!(parse_series(&bin).is_none());
    }

    #[test]
    fn component_name_names_the_delay() {
        assert_eq!(component_name(COMP_DELAY), Some("galileo_ionocal_delay_m"));
        assert_eq!(component_name(99), None);
    }

    #[test]
    fn comments_and_blank_lines_do_not_break_records() {
        let mut text = MEASURED_2003032.to_vec();
        text.extend_from_slice(b"# FITSIG=  0.0122  FLG=01\n\n");
        text.extend_from_slice(MEASURED_SINGLE_LINE);
        let p = parse_records(&text);
        assert_eq!(p.broken, 0);
        assert_eq!(p.records.len(), 2);
    }
}
