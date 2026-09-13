use crate::net::{get, urlencode};
use omegaflow::fits::{FitsHeader, FitsTable, FitsValue};

fn parameter(query: &str, key: &str) -> Option<String> {
    query.split_whitespace().find_map(|token| {
        let (name, value) = token.split_once('=')?;
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn xtension(header: &FitsHeader) -> Option<String> {
    header
        .value("XTENSION")
        .map(|v| v.trim_matches('\'').trim().to_string())
}

fn table_header(header: &FitsHeader, rows: usize, cols: usize) -> String {
    match header.str_unescaped("EXTNAME") {
        Some(name) => format!("table {}\trows {}\tcols {}", name.trim(), rows, cols),
        None => format!("table\trows {}\tcols {}", rows, cols),
    }
}

fn numeric_text(value: &FitsValue) -> Option<String> {
    match value {
        FitsValue::Int(v) => Some(v.to_string()),
        FitsValue::Float(v) if v.is_finite() => Some(format!("{}", v)),
        FitsValue::Ints(vs) if !vs.is_empty() => Some(
            vs.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ),
        FitsValue::Floats(vs) if !vs.is_empty() && vs.iter().all(|v| v.is_finite()) => Some(
            vs.iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join(","),
        ),
        _ => None,
    }
}

fn bintable_lines(bytes: &[u8], table: &FitsTable, header: &FitsHeader, max: usize) -> Vec<String> {
    let mut lines = vec![table_header(header, table.n_rows, table.columns.len())];
    for row in 0..table.n_rows.min(max) {
        let mut parts = Vec::new();
        for col in &table.columns {
            if let Some(value) = table.cell_value(bytes, row, col) {
                if let Some(text) = numeric_text(&value) {
                    parts.push(format!("{}={}", col.name, text));
                }
            }
        }
        lines.push(parts.join("\t"));
    }
    lines
}

struct AsciiColumn {
    name: String,
    code: char,
    tbcol: usize,
    width: usize,
}

fn tform_parts(tform: &str) -> Option<(char, usize)> {
    let mut chars = tform.chars();
    let code = chars.next()?;
    let rest: String = chars.collect();
    let width = rest.split('.').next()?.trim().parse().ok()?;
    Some((code, width))
}

fn ascii_columns(header: &FitsHeader) -> Vec<AsciiColumn> {
    let mut cols = Vec::new();
    let tfields = match header.int("TFIELDS") {
        Some(v) => v as usize,
        None => 0,
    };
    for i in 1..=tfields {
        let name = match header.str_unescaped(&format!("TTYPE{}", i)) {
            Some(n) => n.trim().to_string(),
            None => String::new(),
        };
        let tform = header
            .value(&format!("TFORM{}", i))
            .unwrap_or("")
            .trim_matches('\'')
            .trim()
            .to_string();
        let (code, width) = match tform_parts(&tform) {
            Some(v) => v,
            None => continue,
        };
        let tbcol = match header.int(&format!("TBCOL{}", i)) {
            Some(v) if v > 0 => v as usize,
            _ => continue,
        };
        cols.push(AsciiColumn {
            name,
            code,
            tbcol,
            width,
        });
    }
    cols
}

fn ascii_cell(bytes: &[u8], row_start: usize, col: &AsciiColumn) -> Option<String> {
    let start = row_start + col.tbcol - 1;
    let raw = bytes.get(start..start + col.width)?;
    let text = std::str::from_utf8(raw).ok()?.trim();
    if text.is_empty() {
        return None;
    }
    match col.code {
        'I' => text.parse::<i64>().ok().map(|v| v.to_string()),
        'F' | 'E' | 'D' => match text.parse::<f64>() {
            Ok(v) if v.is_finite() => Some(format!("{}", v)),
            _ => None,
        },
        _ => None,
    }
}

fn ascii_table_lines(
    bytes: &[u8],
    header: &FitsHeader,
    data_start: usize,
    max: usize,
) -> Vec<String> {
    let row_bytes = match header.int("NAXIS1") {
        Some(v) if v > 0 => v as usize,
        _ => return Vec::new(),
    };
    let n_rows = match header.int("NAXIS2") {
        Some(v) if v >= 0 => v as usize,
        _ => return Vec::new(),
    };
    let cols = ascii_columns(header);
    if cols.is_empty() {
        return Vec::new();
    }
    let table_bytes = match row_bytes.checked_mul(n_rows) {
        Some(v) => v,
        None => return Vec::new(),
    };
    if data_start
        .checked_add(table_bytes)
        .is_none_or(|end| end > bytes.len())
    {
        return Vec::new();
    }
    let mut lines = vec![table_header(header, n_rows, cols.len())];
    for row in 0..n_rows.min(max) {
        let row_start = data_start + row * row_bytes;
        let mut parts = Vec::new();
        for col in &cols {
            if let Some(text) = ascii_cell(bytes, row_start, col) {
                parts.push(format!("{}={}", col.name, text));
            }
        }
        lines.push(parts.join("\t"));
    }
    lines
}

fn parse_heasarc_fits(bytes: &[u8], max: usize) -> Vec<String> {
    let mut off = 0usize;
    for _ in 0..16 {
        let (header, next) = match FitsHeader::parse(bytes, off) {
            Some(v) => v,
            None => return Vec::new(),
        };
        match xtension(&header).as_deref() {
            Some("BINTABLE") => {
                if let Some((table, _)) = FitsTable::parse(bytes, off) {
                    return bintable_lines(bytes, &table, &header, max);
                }
            }
            Some("TABLE") => return ascii_table_lines(bytes, &header, next, max),
            _ => {}
        }
        if next <= off {
            return Vec::new();
        }
        off = next;
    }
    Vec::new()
}

pub fn heasarc_lines(query: &str, max: usize) -> Vec<String> {
    let table = match parameter(query, "table") {
        Some(name) => name,
        None => {
            return vec![format!(
                "pending — heasarc: query names no table: {}",
                query
            )];
        }
    };
    let rows = parameter(query, "rows")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(max);
    let url = format!(
        "https://heasarc.gsfc.nasa.gov/cgi-bin/W3Browse/w3query.pl?tablehead={}&ResultMax={}&displaymode=FitsDisplay&Fields=All&Action=Start",
        urlencode(&format!("name={}", table)),
        rows
    );
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_heasarc_fits(f.body.as_bytes(), max);
            if lines.is_empty() {
                vec!["pending — heasarc: the body carries no FITS table".to_string()]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — heasarc HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn pad_hdu(mut hdu: Vec<u8>) -> Vec<u8> {
        while hdu.len() % 2880 != 0 {
            hdu.extend_from_slice(&[b' '; 80]);
        }
        hdu
    }

    fn synth_bintable() -> Vec<u8> {
        let mut primary = Vec::new();
        primary.extend_from_slice(&pad_card("SIMPLE", "T"));
        primary.extend_from_slice(&pad_card("BITPIX", "8"));
        primary.extend_from_slice(&pad_card("NAXIS", "0"));
        primary.extend_from_slice(&pad_card("END", ""));

        let mut ext = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "8"));
        ext.extend_from_slice(&pad_card("NAXIS2", "1"));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "2"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'FLUX'"));
        ext.extend_from_slice(&pad_card("TFORM1", "E"));
        ext.extend_from_slice(&pad_card("TBCOL1", "1"));
        ext.extend_from_slice(&pad_card("TTYPE2", "'COUNT'"));
        ext.extend_from_slice(&pad_card("TFORM2", "J"));
        ext.extend_from_slice(&pad_card("TBCOL2", "5"));
        ext.extend_from_slice(&pad_card("EXTNAME", "'SYNTH'"));
        ext.extend_from_slice(&pad_card("END", ""));

        let mut buf = pad_hdu(primary);
        buf.extend_from_slice(&pad_hdu(ext));
        buf.extend_from_slice(&2.5f32.to_be_bytes());
        buf.extend_from_slice(&7i32.to_be_bytes());
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn bintable_reads_the_named_columns() {
        let buf = synth_bintable();
        let lines = parse_heasarc_fits(&buf, 10);
        assert_eq!(lines[0], "table SYNTH\trows 1\tcols 2");
        assert_eq!(lines[1], "FLUX=2.5\tCOUNT=7");
    }

    #[test]
    fn bintable_honors_the_row_cap() {
        let buf = synth_bintable();
        assert_eq!(parse_heasarc_fits(&buf, 0).len(), 1);
    }

    fn synth_ascii_table() -> Vec<u8> {
        let mut primary = Vec::new();
        primary.extend_from_slice(&pad_card("SIMPLE", "T"));
        primary.extend_from_slice(&pad_card("BITPIX", "8"));
        primary.extend_from_slice(&pad_card("NAXIS", "0"));
        primary.extend_from_slice(&pad_card("EXTEND", "T"));
        primary.extend_from_slice(&pad_card("END", ""));

        let mut ext = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'TABLE   '"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "20"));
        ext.extend_from_slice(&pad_card("NAXIS2", "1"));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "2"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'NAME'"));
        ext.extend_from_slice(&pad_card("TFORM1", "'A5'"));
        ext.extend_from_slice(&pad_card("TBCOL1", "1"));
        ext.extend_from_slice(&pad_card("TTYPE2", "'FLUX'"));
        ext.extend_from_slice(&pad_card("TFORM2", "'F8.3'"));
        ext.extend_from_slice(&pad_card("TBCOL2", "7"));
        ext.extend_from_slice(&pad_card("EXTNAME", "'ASCII'"));
        ext.extend_from_slice(&pad_card("END", ""));

        let mut buf = pad_hdu(primary);
        buf.extend_from_slice(&pad_hdu(ext));
        buf.extend_from_slice(b"AB     12.500      ");
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn ascii_table_reads_the_numeric_cell_and_omits_the_string() {
        let buf = synth_ascii_table();
        let lines = parse_heasarc_fits(&buf, 10);
        assert_eq!(lines[0], "table ASCII\trows 1\tcols 2");
        assert_eq!(lines[1], "FLUX=12.5");
    }

    #[test]
    fn corrupt_and_empty_buffers_carry_no_table() {
        assert!(parse_heasarc_fits(&[], 10).is_empty());
        assert!(parse_heasarc_fits(b"not a fits buffer", 10).is_empty());
    }

    #[test]
    fn parameter_reads_the_named_token() {
        assert_eq!(
            parameter("table=sao rows=3", "table").as_deref(),
            Some("sao")
        );
        assert_eq!(parameter("table=sao rows=3", "rows").as_deref(), Some("3"));
        assert!(parameter("table=sao", "rows").is_none());
    }
}
