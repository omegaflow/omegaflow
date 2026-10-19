#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sniffed {
    pub format: String,
    pub delimiter: Option<char>,
    pub header: Option<Vec<String>>,
    pub columns: Vec<String>,
    pub time_column: Option<usize>,
    pub unit: Option<String>,
    pub magic: Option<String>,
}

const MAGICS: &[(&[u8], &str)] = &[
    (b"\x89PNG\r\n\x1a\n", "png"),
    (b"\xff\xd8\xff", "jpeg"),
    (b"GIF87a", "gif"),
    (b"GIF89a", "gif"),
    (b"%PDF-", "pdf"),
    (b"PK\x03\x04", "zip"),
    (b"PK\x05\x06", "zip-empty"),
    (b"PK\x07\x08", "zip-spanned"),
    (b"\x1f\x8b", "gzip"),
    (b"BZh", "bzip2"),
    (b"\xfd7zXZ\x00", "xz"),
    (b"\x28\xb5\x2f\xfd", "zstd"),
    (b"7z\xbc\xaf\x27\x1c", "7z"),
    (b"Rar!\x1a\x07", "rar"),
    (b"SQLite format 3\x00", "sqlite"),
    (b"\x7fELF", "elf"),
    (b"\x00asm", "wasm"),
    (b"\x89HDF\r\n\x1a\n", "hdf5"),
    (b"\x0e\x03\x13\x01", "hdf4"),
    (b"CDF\x01", "netcdf"),
    (b"CDF\x02", "netcdf"),
    (b"SIMPLE  =", "fits"),
    (b"PAR1", "parquet"),
    (b"II*\x00", "tiff-le"),
    (b"MM\x00*", "tiff-be"),
    (b"RIFF", "riff"),
    (b"fLaC", "flac"),
    (b"OggS", "ogg"),
    (b"\x1aE\xdf\xa3", "matroska"),
    (b"GRIB", "grib"),
    (b"BUFR", "bufr"),
    (b"wOFF", "woff"),
    (b"wOF2", "woff2"),
    (b"\xca\xfe\xba\xbe", "macho-fat"),
    (b"\xcf\xfa\xed\xfe", "macho"),
    (b"\xce\xfa\xed\xfe", "macho"),
];

const NON_UNITS: &[&str] = &[
    "utc", "ut", "tai", "tt", "tdb", "tcb", "gps", "iso", "j2000", "jd", "mjd", "date", "time",
];

pub fn sniff(bytes: &[u8]) -> Sniffed {
    if let Some(name) = magic_of(bytes) {
        return Sniffed {
            format: "binary-magic".to_string(),
            delimiter: None,
            header: None,
            columns: Vec::new(),
            time_column: None,
            unit: None,
            magic: Some(name.to_string()),
        };
    }
    let sample = &bytes[..bytes.len().min(262_144)];
    if !text_like(sample) {
        return unknown();
    }
    let text = String::from_utf8_lossy(sample);
    let lines: Vec<&str> = text
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .take(256)
        .collect();
    if lines.is_empty() {
        return unknown();
    }

    let delimiter = detect_delimiter(&lines);
    let fixed = match delimiter {
        Some(_) => None,
        None => fixed_layout(&lines),
    };
    let format = match delimiter {
        Some('\t') => "tsv",
        Some(_) => "csv",
        None => match fixed {
            Some(_) => "fixed",
            None => "unknown",
        },
    };

    let first_fields = split_fields(lines[0], delimiter);
    let following: Vec<Vec<String>> = lines
        .iter()
        .skip(1)
        .take(64)
        .map(|l| split_fields(l, delimiter))
        .collect();
    let header = match looks_like_header(&first_fields, &following) {
        true => Some(first_fields.clone()),
        false => None,
    };
    let mut rows: Vec<Vec<String>> = Vec::new();
    if header.is_none() {
        rows.push(first_fields.clone());
    }
    rows.extend(following);

    let ncols = match &header {
        Some(h) => h.len(),
        None => match rows.first() {
            Some(r) => r.len(),
            None => first_fields.len(),
        },
    };
    let columns = match &header {
        Some(h) => h.clone(),
        None => (1..=ncols).map(|i| format!("col{i}")).collect(),
    };
    let time_column = match &header {
        Some(h) => time_in_header(h).or_else(|| time_in_values(&rows)),
        None => time_in_values(&rows),
    };
    let unit = match &header {
        Some(h) => unit_in_header(h),
        None => None,
    };

    Sniffed {
        format: format.to_string(),
        delimiter,
        header,
        columns,
        time_column,
        unit,
        magic: None,
    }
}

pub fn find_pattern(bytes: &[u8], needle: &[u8], max_hits: usize) -> Vec<usize> {
    let mut hits = Vec::new();
    if needle.is_empty() || max_hits == 0 || needle.len() > bytes.len() {
        return hits;
    }
    let last = bytes.len() - needle.len();
    let mut i = 0usize;
    while i <= last {
        if bytes[i] == needle[0] && &bytes[i..i + needle.len()] == needle {
            hits.push(i);
            if hits.len() >= max_hits {
                return hits;
            }
        }
        i += 1;
    }
    hits
}

fn unknown() -> Sniffed {
    Sniffed {
        format: "unknown".to_string(),
        delimiter: None,
        header: None,
        columns: Vec::new(),
        time_column: None,
        unit: None,
        magic: None,
    }
}

fn magic_of(bytes: &[u8]) -> Option<&'static str> {
    for (sig, name) in MAGICS {
        if bytes.starts_with(sig) {
            return Some(name);
        }
    }
    if bytes.len() > 262 && &bytes[257..262] == b"ustar" {
        return Some("tar");
    }
    None
}

fn text_like(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }
    let n = bytes.len();
    let mut nul = 0usize;
    let mut ctrl = 0usize;
    for &b in bytes {
        if b == 0 {
            nul += 1;
        } else if b < 0x20 && !matches!(b, b'\t' | b'\n' | b'\r' | 0x0c) {
            ctrl += 1;
        }
    }
    nul * 100 <= n && ctrl * 20 <= n
}

fn detect_delimiter(lines: &[&str]) -> Option<char> {
    let sample = &lines[..lines.len().min(64)];
    let mut best: Option<(char, usize, usize, usize)> = None;
    for d in [',', ';', '\t', '|'] {
        let counts: Vec<usize> = sample.iter().map(|l| count_delim(l, d)).collect();
        let mut mode_hits = 0usize;
        let mut mode_count = 0usize;
        for &c in &counts {
            if c == 0 {
                continue;
            }
            let hits = counts.iter().filter(|&&x| x == c).count();
            if hits > mode_hits || (hits == mode_hits && c > mode_count) {
                mode_hits = hits;
                mode_count = c;
            }
        }
        if mode_count == 0 {
            continue;
        }
        let ratio = mode_hits * 1000 / counts.len();
        if ratio < 750 {
            continue;
        }
        let headerish = match first_line_headerish(sample[0], d) {
            true => 1usize,
            false => 0usize,
        };
        let candidate = (d, ratio, headerish, mode_count);
        let better = match best {
            Some((_, br, bh, bm)) => {
                ratio > br
                    || (ratio == br && (headerish > bh || (headerish == bh && mode_count > bm)))
            }
            None => true,
        };
        if better {
            best = Some(candidate);
        }
    }
    match best {
        Some((d, _, _, _)) => Some(d),
        None => None,
    }
}

fn count_delim(line: &str, d: char) -> usize {
    let mut quoted = false;
    let mut n = 0usize;
    for c in line.chars() {
        if c == '"' {
            quoted = !quoted;
        } else if !quoted && c == d {
            n += 1;
        }
    }
    n
}

fn first_line_headerish(line: &str, d: char) -> bool {
    let fields = split_quoted(line, d);
    fields.len() >= 2
        && fields.iter().all(|f| !f.trim().is_empty())
        && fields.iter().any(|f| parse_number(f).is_none())
}

fn split_fields(line: &str, delimiter: Option<char>) -> Vec<String> {
    match delimiter {
        Some(d) => split_quoted(line, d),
        None => line.split_whitespace().map(str::to_string).collect(),
    }
}

fn split_quoted(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    quoted = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            quoted = true;
        } else if c == delimiter {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}

fn looks_like_header(first: &[String], rows: &[Vec<String>]) -> bool {
    if first.is_empty() {
        return false;
    }
    if first.iter().any(|f| f.trim().is_empty()) {
        return false;
    }
    let named = first.iter().any(|f| parse_number(f).is_none());
    if !named {
        return false;
    }
    if rows.is_empty() {
        return true;
    }
    let same_width = rows.iter().any(|r| r.len() == first.len());
    let numeric = rows
        .iter()
        .take(16)
        .any(|r| r.iter().any(|f| parse_number(f).is_some()));
    same_width && numeric
}

fn parse_number(field: &str) -> Option<f64> {
    let t = field.trim().trim_matches('"').trim();
    if t.is_empty() {
        return None;
    }
    match t.parse::<f64>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn fixed_layout(lines: &[&str]) -> Option<usize> {
    let rows = &lines[..lines.len().min(16)];
    if rows.len() >= 2 {
        for start in 0..rows.len().min(2) {
            let template = spans(rows[start]);
            if template.len() < 2 {
                continue;
            }
            let rest = rows.len() - start - 1;
            let agree = rows
                .iter()
                .skip(start + 1)
                .filter(|l| aligned(l, &template))
                .count();
            if rest > 0 && agree > 0 && agree * 2 >= rest {
                return Some(template.len());
            }
        }
    }
    if rows.iter().all(|l| l.split_whitespace().count() == 1) {
        return Some(1);
    }
    None
}

fn spans(line: &str) -> Vec<usize> {
    let mut starts = Vec::new();
    let mut in_token = false;
    for (i, c) in line.char_indices() {
        if c.is_whitespace() {
            in_token = false;
        } else if !in_token {
            starts.push(i);
            in_token = true;
        }
    }
    starts
}

fn aligned(line: &str, template: &[usize]) -> bool {
    let starts = spans(line);
    if starts.len() != template.len() {
        return false;
    }
    starts
        .iter()
        .zip(template)
        .all(|(&a, &b)| a.abs_diff(b) <= 1)
}

fn time_in_header(header: &[String]) -> Option<usize> {
    let mut best: Option<(u8, usize)> = None;
    for (i, name) in header.iter().enumerate() {
        let rank = match time_name_rank(name) {
            Some(r) => r,
            None => continue,
        };
        best = match best {
            Some((br, _)) if br <= rank => best,
            _ => Some((rank, i)),
        };
    }
    match best {
        Some((_, i)) => Some(i),
        None => None,
    }
}

fn time_name_rank(name: &str) -> Option<u8> {
    let lower = name.trim().to_ascii_lowercase();
    let tokens: Vec<&str> = lower
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();
    if tokens.iter().any(|t| *t == "ut1") {
        return None;
    }
    if tokens
        .iter()
        .any(|t| matches!(*t, "date" | "datetime" | "timestamp" | "datum" | "day"))
    {
        return Some(0);
    }
    if tokens.iter().any(|t| *t == "epoch") {
        return Some(1);
    }
    if tokens.iter().any(|t| *t == "mjd") {
        return Some(2);
    }
    if tokens.iter().any(|t| *t == "jd" || *t == "julian") {
        return Some(3);
    }
    if tokens.iter().any(|t| *t == "time") {
        return Some(4);
    }
    if tokens
        .iter()
        .any(|t| matches!(*t, "utc" | "gps" | "tai" | "tt"))
    {
        return Some(5);
    }
    None
}

fn time_in_values(rows: &[Vec<String>]) -> Option<usize> {
    let sample: Vec<&Vec<String>> = rows.iter().take(64).collect();
    if sample.is_empty() {
        return None;
    }
    let ncols = sample.iter().map(|r| r.len()).max()?;
    for col in 0..ncols {
        let values: Vec<&str> = sample
            .iter()
            .filter_map(|r| r.get(col))
            .map(|s| s.trim().trim_matches('"'))
            .filter(|s| !s.is_empty())
            .collect();
        if values.is_empty() {
            continue;
        }
        if values.iter().all(|v| is_iso_date(v)) {
            return Some(col);
        }
    }
    if let Some(col) = numeric_time_pass(&sample, ncols, 10_000.0, 100_000.0) {
        return Some(col);
    }
    if let Some(col) = numeric_time_pass(&sample, ncols, 2_400_000.0, 2_600_000.0) {
        return Some(col);
    }
    numeric_time_pass(&sample, ncols, 1.0e8, 4.0e9)
}

fn numeric_time_pass(sample: &[&Vec<String>], ncols: usize, lo: f64, hi: f64) -> Option<usize> {
    for col in 0..ncols {
        let values: Vec<f64> = sample
            .iter()
            .filter_map(|r| r.get(col))
            .filter_map(|s| parse_number(s))
            .collect();
        if values.len() < sample.len() {
            continue;
        }
        if values.iter().any(|v| !v.is_finite() || *v < lo || *v > hi) {
            continue;
        }
        if values.windows(2).any(|w| w[1] < w[0]) {
            continue;
        }
        return Some(col);
    }
    None
}

fn is_iso_date(value: &str) -> bool {
    let b = value.as_bytes();
    if b.len() < 10 {
        return false;
    }
    if b[4] != b'-' || b[7] != b'-' {
        return false;
    }
    if !b[0..4].iter().all(u8::is_ascii_digit)
        || !b[5..7].iter().all(u8::is_ascii_digit)
        || !b[8..10].iter().all(u8::is_ascii_digit)
    {
        return false;
    }
    if b.len() > 10 && !matches!(b[10], b'T' | b't' | b' ') {
        return false;
    }
    let year = (b[0] - b'0') as u32 * 1000
        + (b[1] - b'0') as u32 * 100
        + (b[2] - b'0') as u32 * 10
        + (b[3] - b'0') as u32;
    let month = (b[5] - b'0') as u32 * 10 + (b[6] - b'0') as u32;
    let day = (b[8] - b'0') as u32 * 10 + (b[9] - b'0') as u32;
    (1500..=3000).contains(&year) && (1..=12).contains(&month) && (1..=31).contains(&day)
}

fn unit_in_header(header: &[String]) -> Option<String> {
    for name in header {
        for (open, close) in [('[', ']'), ('(', ')')] {
            let start = match name.find(open) {
                Some(p) => p,
                None => continue,
            };
            let rest = &name[start + open.len_utf8()..];
            let end = match rest.find(close) {
                Some(p) => p,
                None => continue,
            };
            let unit = rest[..end].trim();
            if unit_ok(unit) {
                return Some(unit.to_string());
            }
        }
    }
    None
}

fn unit_ok(unit: &str) -> bool {
    if unit.is_empty() || unit.len() > 24 {
        return false;
    }
    if NON_UNITS.iter().any(|u| unit.eq_ignore_ascii_case(u)) {
        return false;
    }
    unit.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || matches!(
                c,
                '°' | '%' | 'µ' | 'Ω' | '^' | '*' | '/' | '·' | '.' | '-' | ' ' | '\'' | '"'
            )
    }) && unit
        .chars()
        .any(|c| c.is_ascii_alphabetic() || matches!(c, '°' | '%' | 'µ' | 'Ω'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semicolon_csv_with_time_header() {
        let s = sniff(b"time;value\n2020-01-01T00:00:00;1.5\n2020-01-02T00:00:00;2.5\n");
        assert_eq!(s.format, "csv");
        assert_eq!(s.delimiter, Some(';'));
        assert_eq!(
            s.header,
            Some(vec!["time".to_string(), "value".to_string()])
        );
        assert_eq!(s.columns, vec!["time".to_string(), "value".to_string()]);
        assert_eq!(s.time_column, Some(0));
        assert_eq!(s.unit, None);
    }

    #[test]
    fn tsv_with_bracket_unit() {
        let s = sniff(b"time\tvalue [s]\n0\t1\n1\t2\n2\t3\n");
        assert_eq!(s.format, "tsv");
        assert_eq!(s.delimiter, Some('\t'));
        assert_eq!(s.unit, Some("s".to_string()));
        assert_eq!(s.time_column, Some(0));
    }

    #[test]
    fn binary_with_magic() {
        let s = sniff(b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR");
        assert_eq!(s.format, "binary-magic");
        assert_eq!(s.magic, Some("png".to_string()));
        assert_eq!(s.delimiter, None);
    }

    #[test]
    fn find_pattern_two_hits() {
        let bytes = b"time,value\ntime,value\n";
        assert_eq!(find_pattern(bytes, b"time", 16), vec![0, 11]);
        assert_eq!(find_pattern(bytes, b"time", 1), vec![0]);
        assert_eq!(find_pattern(bytes, b"", 16), Vec::<usize>::new());
        assert_eq!(find_pattern(bytes, b"absent", 16), Vec::<usize>::new());
    }

    #[test]
    fn comma_csv_headerless_detects_epoch_column() {
        let s = sniff(b"1609459200,1.5\n1609545600,2.5\n1609632000,3.5\n");
        assert_eq!(s.format, "csv");
        assert_eq!(s.delimiter, Some(','));
        assert_eq!(s.header, None);
        assert_eq!(s.columns.len(), 2);
        assert_eq!(s.time_column, Some(0));
    }

    #[test]
    fn nul_heavy_bytes_are_unknown() {
        let s = sniff(&[0u8, 1, 2, 3, 0, 0, 4, 5, 0, 0, 0, 6]);
        assert_eq!(s.format, "unknown");
        assert_eq!(s.magic, None);
    }

    #[test]
    fn parenthesized_unit_in_header() {
        let s = sniff(b"time;speed (m/s)\n0;1.0\n1;2.0\n");
        assert_eq!(s.unit, Some("m/s".to_string()));
    }

    #[test]
    fn find_pattern_is_overlapping() {
        assert_eq!(find_pattern(b"aaaa", b"aa", 8), vec![0, 1, 2]);
    }
}
