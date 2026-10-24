use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::inflate::zip_members;

trait AbsentExt<T> {
    fn or_absent(self, marker: &str) -> String;
}

impl<T: ToString> AbsentExt<T> for Option<T> {
    fn or_absent(self, marker: &str) -> String {
        match self {
            Some(v) => v.to_string(),
            None => marker.to_string(),
        }
    }
}

pub const COMP_CDOD: u32 = 1;
pub const COMP_SIZE: u32 = 2;
pub const COMP_CENTROID_LATITUDE: u32 = 3;
pub const COMP_CENTROID_LONGITUDE: u32 = 4;
pub const COMP_MARS_LS: u32 = 5;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_CDOD => Some("emm_exi_cdod"),
        COMP_SIZE => Some("emm_exi_size"),
        COMP_CENTROID_LATITUDE => Some("emm_exi_centroid_latitude"),
        COMP_CENTROID_LONGITUDE => Some("emm_exi_centroid_longitude"),
        COMP_MARS_LS => Some("emm_exi_mars_ls"),
        _ => None,
    }
}

pub const DEFAULT_URL: &str = "https://github.com/omegaflow/sources/releases/download/zenodo.org/DustStormDatabaseEXI_MY36_V1.xlsx";
pub const ORIGIN: &str =
    "https://zenodo.org/api/records/10275974/files/DustStormDatabaseEXI_MY36_V1.xlsx/content";

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub names: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
}

fn xml_text(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

fn between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let end = s[start..].find(close)? + start;
    Some(&s[start..end])
}

fn shared_strings(xml: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(si) = between(rest, "<si>", "</si>") {
        let mut text = String::new();
        let mut inner = si;
        while let Some(t) = between(inner, "<t", "</t>") {
            let body = t.find('>').map(|i| &t[i + 1..]).unwrap_or(t);
            text.push_str(&xml_text(body));
            inner = &t[t.len()..];
        }
        out.push(text);
        let consumed = si.as_ptr() as usize - rest.as_ptr() as usize + si.len() + 5;
        rest = &rest[consumed.min(rest.len())..];
    }
    out
}

fn col_index(reference: &str) -> Option<usize> {
    let mut n = 0usize;
    let mut any = false;
    for b in reference.bytes() {
        if b.is_ascii_alphabetic() {
            n = n * 26 + (b.to_ascii_uppercase() - b'A' + 1) as usize;
            any = true;
        } else {
            break;
        }
    }
    any.then_some(n.saturating_sub(1))
}

fn cell_value(attrs: &str, body: &str, shared: &[String]) -> Option<String> {
    let kind = attrs
        .split_whitespace()
        .find_map(|a| a.strip_prefix("t=\""))
        .and_then(|a| a.strip_suffix('"'))
        .unwrap_or("");
    if kind == "inlineStr" {
        if let Some(t) = between(body, "<t", "</t>") {
            let text = t.find('>').map(|i| &t[i + 1..]).unwrap_or(t);
            return Some(xml_text(text));
        }
    }
    let raw = between(body, "<v>", "</v>")?;
    if kind == "s" {
        let idx = raw.parse::<usize>().ok()?;
        shared.get(idx).cloned()
    } else {
        Some(xml_text(raw))
    }
}

fn sheet_rows(xml: &str, shared: &[String]) -> Vec<Vec<Option<String>>> {
    let mut rows = Vec::new();
    let mut rest = xml;
    while let Some(row_body) = between(rest, "<row", "</row>") {
        let body = row_body.find('>').map(|i| &row_body[i + 1..]).unwrap_or("");
        let mut cells: Vec<Option<String>> = Vec::new();
        let mut cell_rest = body;
        while let Some(start) = cell_rest.find("<c") {
            let after = &cell_rest[start..];
            let Some(tag_end) = after.find('>') else {
                break;
            };
            let tag = &after[..tag_end];
            let attrs = tag.trim_start_matches("<c");
            let self_closing = tag.ends_with('/');
            let (inner, consumed) = if self_closing {
                ("", tag_end + 1)
            } else {
                let Some(close) = after.find("</c>") else {
                    break;
                };
                (&after[tag_end + 1..close], close + 4)
            };
            let reference = attrs
                .split_whitespace()
                .find_map(|a| a.strip_prefix("r=\""))
                .and_then(|a| a.strip_suffix('"'))
                .unwrap_or("");
            if let Some(idx) = col_index(reference) {
                if cells.len() <= idx {
                    cells.resize(idx + 1, None);
                }
                cells[idx] = cell_value(attrs, inner, shared);
            }
            cell_rest = &after[consumed..];
        }
        if !cells.is_empty() {
            rows.push(cells);
        }
        let consumed = row_body.as_ptr() as usize - rest.as_ptr() as usize + row_body.len() + 6;
        rest = &rest[consumed.min(rest.len())..];
    }
    rows
}

fn member(bytes: &[u8], wanted: &str) -> Option<Vec<u8>> {
    let mut found = None;
    zip_members(bytes, |name, data| {
        if name == wanted {
            found = Some(data.to_vec());
        }
    })?;
    found
}

pub fn parse_xlsx(bytes: &[u8]) -> Option<Table> {
    let shared = match member(bytes, "xl/sharedStrings.xml") {
        Some(b) => shared_strings(&String::from_utf8_lossy(&b)),
        None => Vec::new(),
    };
    let sheet = member(bytes, "xl/worksheets/sheet1.xml")?;
    let sheet = String::from_utf8_lossy(&sheet);
    let mut rows = sheet_rows(&sheet, &shared);
    if rows.len() < 2 {
        return None;
    }
    let names = rows.remove(0);
    let names = names
        .into_iter()
        .enumerate()
        .map(|(i, n)| n.or_absent(&format!("col{i}")))
        .collect::<Vec<_>>();
    let width = names.len();
    for row in rows.iter_mut() {
        row.resize(width, None);
    }
    Some(Table { names, rows })
}

fn column(table: &Table, needles: &[&str]) -> Option<usize> {
    table.names.iter().position(|name| {
        let n = name.to_ascii_lowercase();
        needles.iter().any(|needle| n.contains(needle))
    })
}

pub fn cdod_column(table: &Table) -> Option<usize> {
    column(
        table,
        &["cdod", "column dust optical", "dust optical depth"],
    )
}

fn value_f64(table: &Table, row: usize, col: usize) -> Option<f64> {
    table
        .rows
        .get(row)?
        .get(col)?
        .as_deref()?
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}

fn row_time(table: &Table, row: usize) -> f64 {
    if let Some(col) = column(table, &["date/time", "date", "time", "marsut"]) {
        if let Some(v) = value_f64(table, row, col) {
            return v;
        }
    }
    row as f64
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let table = parse_xlsx(bytes)?;
    let col = cdod_column(&table)?;
    let mut out = Vec::new();
    for r in 0..table.rows.len() {
        if let Some(v) = value_f64(&table, r, col) {
            out.push((row_time(&table, r), v, COMP_CDOD));
        }
    }
    (!out.is_empty()).then_some(out)
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load(args: &[String]) -> Option<(String, Vec<u8>)> {
    if let Some(path) = arg_value(args, "--file") {
        std::fs::read(&path).ok().map(|b| (path, b))
    } else {
        let url = arg_value(&args, "--url").or_absent(DEFAULT_URL);
        fetch_raw_bytes(&url).map(|b| (url, b))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((src, bytes)) = load(&args) else {
        eprintln!("emm_exi_dust: {ORIGIN} stays unfetched");
        std::process::exit(1);
    };
    let Some(table) = parse_xlsx(&bytes) else {
        eprintln!("{src}: {} B carry no xlsx sheet", bytes.len());
        std::process::exit(1);
    };
    eprintln!("{src}: {} data row(s)", table.rows.len());
    eprintln!("  columns: {}", table.names.join(" | "));
    if let Some(first) = table.rows.first() {
        let cells = first
            .iter()
            .enumerate()
            .map(|(i, v)| format!("{}={}", table.names[i], v.as_deref().unwrap_or("absent")))
            .collect::<Vec<_>>()
            .join("  ");
        eprintln!("  first row: {cells}");
    }
    match cdod_column(&table) {
        Some(i) => eprintln!(
            "  {}: column {}",
            component_name(COMP_CDOD).unwrap(),
            table.names[i]
        ),
        None => eprintln!(
            "  {}: absent (workbook has no CDOD/optical-depth column)",
            component_name(COMP_CDOD).unwrap()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crc32(data: &[u8]) -> u32 {
        let mut crc = 0xffff_ffffu32;
        for &b in data {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xedb8_8320
                } else {
                    crc >> 1
                };
            }
        }
        !crc
    }

    fn stored_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut central = Vec::new();
        for (name, data) in entries {
            let off = out.len() as u32;
            let crc = crc32(data);
            out.extend(0x0403_4b50u32.to_le_bytes());
            out.extend(20u16.to_le_bytes());
            out.extend(0u16.to_le_bytes());
            out.extend(0u16.to_le_bytes());
            out.extend(0u16.to_le_bytes());
            out.extend(0u16.to_le_bytes());
            out.extend(crc.to_le_bytes());
            out.extend((data.len() as u32).to_le_bytes());
            out.extend((data.len() as u32).to_le_bytes());
            out.extend((name.len() as u16).to_le_bytes());
            out.extend(0u16.to_le_bytes());
            out.extend(name.as_bytes());
            out.extend_from_slice(data);

            central.extend(0x0201_4b50u32.to_le_bytes());
            central.extend(20u16.to_le_bytes());
            central.extend(20u16.to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(crc.to_le_bytes());
            central.extend((data.len() as u32).to_le_bytes());
            central.extend((data.len() as u32).to_le_bytes());
            central.extend((name.len() as u16).to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(0u16.to_le_bytes());
            central.extend(0u32.to_le_bytes());
            central.extend(off.to_le_bytes());
            central.extend(name.as_bytes());
        }
        let cd_off = out.len() as u32;
        out.extend_from_slice(&central);
        out.extend(0x0605_4b50u32.to_le_bytes());
        out.extend(0u16.to_le_bytes());
        out.extend(0u16.to_le_bytes());
        out.extend((entries.len() as u16).to_le_bytes());
        out.extend((entries.len() as u16).to_le_bytes());
        out.extend((central.len() as u32).to_le_bytes());
        out.extend(cd_off.to_le_bytes());
        out.extend(0u16.to_le_bytes());
        out
    }

    fn fixture() -> Vec<u8> {
        let shared = br#"<?xml version="1.0"?><sst><si><t>Image </t></si><si><t>Storm no.</t></si><si><t>Centroid Latitude</t></si><si><t>Centroid Longitude</t></si><si><t>MarsLs</t></si><si><t>Size</t></si><si><t>emm_exi_l2a_20211129T223059_0140_xos1_f635_p_v03-01</t></si></sst>"#;
        let sheet = br#"<?xml version="1.0"?><worksheet><sheetData><row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c><c r="C1" t="s"><v>2</v></c><c r="D1" t="s"><v>3</v></c><c r="E1" t="s"><v>4</v></c><c r="F1" t="s"><v>5</v></c></row><row r="2"><c r="A2" t="s"><v>6</v></c><c r="B2"><v>1</v></c><c r="C2"><v>-10</v></c><c r="D2"><v>-100</v></c><c r="E2"><v>134.62</v></c><c r="F2"><v>33781.14</v></c></row></sheetData></worksheet>"#;
        stored_zip(&[
            ("xl/sharedStrings.xml", shared),
            ("xl/worksheets/sheet1.xml", sheet),
        ])
    }

    #[test]
    fn reads_workbook_header_and_first_row() {
        let table = parse_xlsx(&fixture()).expect("xlsx fixture parses");
        assert_eq!(table.names[0], "Image ");
        assert_eq!(table.names[2], "Centroid Latitude");
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0][1].as_deref(), Some("1"));
        assert!((value_f64(&table, 0, 2).unwrap() - -10.0).abs() < 1e-9);
        assert!((value_f64(&table, 0, 4).unwrap() - 134.62).abs() < 1e-9);
    }

    #[test]
    fn cdod_is_absent_in_this_workbook() {
        let table = parse_xlsx(&fixture()).unwrap();
        assert!(cdod_column(&table).is_none());
        assert!(parse_series(&fixture()).is_none());
    }

    #[test]
    fn reads_cdod_when_the_column_exists() {
        let shared = br#"<?xml version="1.0"?><sst><si><t>Image </t></si><si><t>CDOD</t></si><si><t>MarsUT</t></si><si><t>img_a</t></si></sst>"#;
        let sheet = br#"<?xml version="1.0"?><worksheet><sheetData><row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c><c r="C1" t="s"><v>2</v></c></row><row r="2"><c r="A2" t="s"><v>3</v></c><c r="B2"><v>0.42</v></c><c r="C2"><v>12.5</v></c></row></sheetData></worksheet>"#;
        let zip = stored_zip(&[
            ("xl/sharedStrings.xml", shared),
            ("xl/worksheets/sheet1.xml", sheet),
        ]);
        let rows = parse_series(&zip).expect("CDOD reads");
        assert_eq!(rows.len(), 1);
        assert!((rows[0].0 - 12.5).abs() < 1e-9);
        assert!((rows[0].1 - 0.42).abs() < 1e-9);
        assert_eq!(rows[0].2, COMP_CDOD);
    }

    #[test]
    fn absent_for_non_zip_bytes() {
        assert!(parse_xlsx(b"not an xlsx").is_none());
    }
}
