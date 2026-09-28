use crate::archivar::lsk::days_from_civil;

pub const MAGIC: [u8; 4] = *b"P3FW";
pub const COLUMN_NAME_BYTES: usize = 64;
pub const COLUMN_UNIT_BYTES: usize = 16;
pub const COLUMN_TYPE_BYTES: usize = 16;
pub const COLUMN_SP_NAME_BYTES: usize = 16;
pub const COLUMN_SP_UNIT_BYTES: usize = 16;
pub const COLUMN_META_BYTES: usize = COLUMN_NAME_BYTES
    + COLUMN_UNIT_BYTES
    + COLUMN_TYPE_BYTES
    + COLUMN_SP_NAME_BYTES
    + COLUMN_SP_UNIT_BYTES
    + 8
    + 8
    + 8
    + 4
    + 4
    + 4;
pub const ROW_VALUE_BYTES: usize = 8;
pub const ROW_PRESENCE_BYTES: usize = 8;

const FLAG_MISSING: u32 = 1;
const FLAG_SP_MIN: u32 = 2;
const FLAG_SP_MAX: u32 = 4;

#[derive(Clone, Debug, PartialEq)]
pub struct TableColumn {
    pub name: String,
    pub unit: String,
    pub data_type: String,
    pub missing_constant: Option<f64>,
    pub sampling_name: String,
    pub sampling_unit: String,
    pub sampling_min: Option<f64>,
    pub sampling_max: Option<f64>,
    pub start_byte: usize,
    pub bytes: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableRow {
    pub values: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pds3Table {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
}

#[derive(Clone, Debug)]
pub struct TableMeta {
    pub record_bytes: Option<usize>,
    pub file_records: Option<usize>,
    pub rows: Option<usize>,
    pub interchange: Option<String>,
    pub table_file: Option<String>,
    pub start_time: Option<String>,
    pub stop_time: Option<String>,
    pub spacecraft_name: Option<String>,
    pub target_name: Option<String>,
    pub instrument_name: Option<String>,
    pub product_id: Option<String>,
    pub columns: Vec<TableColumn>,
}

pub fn odl_kv(text: &str) -> Vec<(String, String)> {
    let mut cleaned = String::with_capacity(text.len());
    let mut rest = text;
    loop {
        match rest.find("/*") {
            Some(open) => {
                cleaned.push_str(&rest[..open]);
                match rest[open + 2..].find("*/") {
                    Some(close) => rest = &rest[open + 2 + close + 2..],
                    None => break,
                }
            }
            None => {
                cleaned.push_str(rest);
                break;
            }
        }
    }
    let mut out = Vec::new();
    for line in cleaned.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match line.split_once('=') {
            Some((key, value)) => {
                let value = value.trim().trim_matches('"').trim().to_string();
                if value.is_empty() {
                    continue;
                }
                out.push((key.trim().to_string(), value));
            }
            None => continue,
        }
    }
    out
}

fn parse_number(v: &str) -> Option<f64> {
    v.parse::<f64>().ok().filter(|x| x.is_finite())
}

fn sampling_value(v: &str) -> Option<f64> {
    let s = v.trim();
    let s = match s.rfind('<') {
        Some(open) if s.ends_with('>') => &s[..open],
        _ => s,
    };
    s.trim().parse::<f64>().ok().filter(|x| x.is_finite())
}

pub fn parse_label(text: &str) -> Option<TableMeta> {
    let mut meta = TableMeta {
        record_bytes: None,
        file_records: None,
        rows: None,
        interchange: None,
        table_file: None,
        start_time: None,
        stop_time: None,
        spacecraft_name: None,
        target_name: None,
        instrument_name: None,
        product_id: None,
        columns: Vec::new(),
    };
    let mut in_table = false;
    let mut column: Option<TableColumn> = None;
    for (key, value) in odl_kv(text) {
        match key.as_str() {
            "OBJECT" => {
                if value == "TABLE" && !in_table {
                    in_table = true;
                } else if value == "COLUMN" && in_table {
                    column = Some(TableColumn {
                        name: String::new(),
                        unit: String::new(),
                        data_type: String::new(),
                        missing_constant: None,
                        sampling_name: String::new(),
                        sampling_unit: String::new(),
                        sampling_min: None,
                        sampling_max: None,
                        start_byte: 0,
                        bytes: 0,
                    });
                }
            }
            "NAME" => {
                if let Some(c) = column.as_mut() {
                    c.name = value;
                }
            }
            "UNIT" => {
                if let Some(c) = column.as_mut() {
                    c.unit = value;
                }
            }
            "DATA_TYPE" => {
                if let Some(c) = column.as_mut() {
                    c.data_type = value;
                }
            }
            "START_BYTE" => {
                if let Some(c) = column.as_mut() {
                    if let Ok(v) = value.parse() {
                        c.start_byte = v;
                    }
                }
            }
            "BYTES" => {
                if let Some(c) = column.as_mut() {
                    if let Ok(v) = value.parse() {
                        c.bytes = v;
                    }
                }
            }
            "MISSING_CONSTANT" => {
                if let Some(c) = column.as_mut() {
                    c.missing_constant = parse_number(&value);
                }
            }
            "SAMPLING_PARAMETER_NAME" => {
                if let Some(c) = column.as_mut() {
                    c.sampling_name = value;
                }
            }
            "SAMPLING_PARAMETER_UNIT" | "SAMPLING_PARAMETR_UNIT" => {
                if let Some(c) = column.as_mut() {
                    c.sampling_unit = value;
                }
            }
            "MINIMUM_SAMPLING_PARAMETER" => {
                if let Some(c) = column.as_mut() {
                    c.sampling_min = sampling_value(&value);
                }
            }
            "MAXIMUM_SAMPLING_PARAMETER" => {
                if let Some(c) = column.as_mut() {
                    c.sampling_max = sampling_value(&value);
                }
            }
            "END_OBJECT" => {
                if value == "COLUMN" {
                    if let Some(c) = column.take() {
                        if c.start_byte > 0 && c.bytes > 0 {
                            meta.columns.push(c);
                        }
                    }
                } else if value == "TABLE" {
                    in_table = false;
                }
            }
            "RECORD_BYTES" => meta.record_bytes = value.parse().ok(),
            "FILE_RECORDS" => meta.file_records = value.parse().ok(),
            "ROWS" => meta.rows = value.parse().ok(),
            "INTERCHANGE_FORMAT" => meta.interchange = Some(value),
            "^TABLE" => meta.table_file = Some(value),
            "START_TIME" => meta.start_time = Some(value),
            "STOP_TIME" => meta.stop_time = Some(value),
            "SPACECRAFT_NAME" => meta.spacecraft_name = Some(value),
            "TARGET_NAME" => meta.target_name = Some(value),
            "INSTRUMENT_NAME" => meta.instrument_name = Some(value),
            "PRODUCT_ID" => meta.product_id = Some(value),
            _ => {}
        }
    }
    meta.columns.sort_by_key(|c| c.start_byte);
    Some(meta)
}

pub fn data_span(meta: &TableMeta) -> Option<usize> {
    meta.columns
        .iter()
        .map(|c| c.start_byte - 1 + c.bytes)
        .max()
}

pub fn record_stride(meta: &TableMeta, file_len: usize) -> Option<usize> {
    let span = data_span(meta)?;
    let mut candidates: Vec<usize> = Vec::new();
    match meta.record_bytes {
        Some(rb) if rb >= span => {
            candidates.push(rb);
            candidates.push(rb + 2);
            candidates.push(rb + 1);
        }
        _ => {
            candidates.push(span + 2);
            candidates.push(span + 1);
            candidates.push(span);
        }
    }
    let divisors: Vec<usize> = candidates
        .iter()
        .copied()
        .filter(|s| *s > 0 && file_len % *s == 0)
        .collect();
    if divisors.is_empty() {
        return None;
    }
    match meta.file_records {
        Some(fr) if fr > 0 => match divisors.iter().find(|s| file_len / **s == fr) {
            Some(s) => Some(*s),
            None => divisors.into_iter().min(),
        },
        _ => divisors.into_iter().min(),
    }
}

pub fn unix_of_iso(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 19 || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' {
        return None;
    }
    let year = s.get(0..4)?.parse::<i64>().ok()?;
    let month = s.get(5..7)?.parse::<i64>().ok()?;
    let day = s.get(8..10)?.parse::<i64>().ok()?;
    let hour = s.get(11..13)?.parse::<i64>().ok()?;
    let minute = s.get(14..16)?.parse::<i64>().ok()?;
    let second = s.get(17..19)?.parse::<i64>().ok()?;
    let days = days_from_civil(year, month, day)?;
    let frac = if b.get(19) == Some(&b'.') {
        let frac_str = s.get(20..)?.trim_end_matches('Z');
        if frac_str.is_empty() {
            0.0
        } else {
            let digits = frac_str.parse::<f64>().ok()?;
            digits / 10f64.powi(frac_str.len() as i32)
        }
    } else {
        0.0
    };
    Some(days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64 + frac)
}

pub fn parse_cell(field: &[u8], data_type: &str, missing: Option<f64>) -> Option<f64> {
    let text = match std::str::from_utf8(field) {
        Ok(t) => t.trim(),
        Err(_) => return None,
    };
    if text.is_empty() {
        return None;
    }
    let parsed = match data_type.to_ascii_uppercase().as_str() {
        "TIME" => unix_of_iso(text),
        "INTEGER" | "ASCII_INTEGER" | "MSB_INTEGER" | "LSB_INTEGER" | "PC_INTEGER" => {
            match text.parse::<i64>() {
                Ok(v) => Some(v as f64),
                Err(_) => None,
            }
        }
        _ => match text.parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => None,
        },
    };
    match parsed {
        Some(v) if !v.is_finite() => None,
        Some(v) => match missing {
            Some(m) if v == m => None,
            _ => Some(v),
        },
        None => None,
    }
}

pub fn decode_rows(
    bytes: &[u8],
    meta: &TableMeta,
) -> Option<(Vec<Vec<Option<f64>>>, usize, usize)> {
    let stride = record_stride(meta, bytes.len())?;
    let span = data_span(meta)?;
    let mut complete = bytes.len() / stride;
    let mut trailing = bytes.len() - complete * stride;
    if trailing >= span {
        complete += 1;
        trailing -= span;
    }
    if complete == 0 {
        return None;
    }
    let mut rows = Vec::with_capacity(complete);
    let mut skipped = 0usize;
    for i in 0..complete {
        let at = i * stride;
        let end = (at + stride).min(bytes.len());
        let rec = &bytes[at..end];
        let mut vals = Vec::with_capacity(meta.columns.len());
        for c in &meta.columns {
            let from = c.start_byte - 1;
            let v = match rec.get(from..from + c.bytes) {
                Some(field) => parse_cell(field, &c.data_type, c.missing_constant),
                None => None,
            };
            vals.push(v);
        }
        if vals.iter().all(|v| v.is_none()) {
            skipped += 1;
        }
        rows.push(vals);
    }
    Some((rows, skipped, trailing))
}

pub fn pack(table: &Pds3Table) -> Vec<u8> {
    let cols = table.columns.len();
    let words = (cols + 63) / 64;
    let mut bin = vec![
        0u8;
        12 + cols * COLUMN_META_BYTES
            + table.rows.len()
                * (cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES)
    ];
    bin[0..4].copy_from_slice(&MAGIC);
    bin[4..8].copy_from_slice(&(cols as u32).to_le_bytes());
    bin[8..12].copy_from_slice(&(table.rows.len() as u32).to_le_bytes());
    for (i, c) in table.columns.iter().enumerate() {
        let base = 12 + i * COLUMN_META_BYTES;
        let name = c.name.as_bytes();
        let n = name.len().min(COLUMN_NAME_BYTES);
        bin[base..base + n].copy_from_slice(&name[..n]);
        let unit = c.unit.as_bytes();
        let u = unit.len().min(COLUMN_UNIT_BYTES);
        bin[base + 64..base + 64 + u].copy_from_slice(&unit[..u]);
        let dt = c.data_type.as_bytes();
        let d = dt.len().min(COLUMN_TYPE_BYTES);
        bin[base + 80..base + 80 + d].copy_from_slice(&dt[..d]);
        let sn = c.sampling_name.as_bytes();
        let snl = sn.len().min(COLUMN_SP_NAME_BYTES);
        bin[base + 96..base + 96 + snl].copy_from_slice(&sn[..snl]);
        let su = c.sampling_unit.as_bytes();
        let sul = su.len().min(COLUMN_SP_UNIT_BYTES);
        bin[base + 112..base + 112 + sul].copy_from_slice(&su[..sul]);
        let missing = match c.missing_constant {
            Some(v) => v,
            None => 0.0,
        };
        bin[base + 128..base + 136].copy_from_slice(&missing.to_le_bytes());
        let sp_min = match c.sampling_min {
            Some(v) => v,
            None => 0.0,
        };
        bin[base + 136..base + 144].copy_from_slice(&sp_min.to_le_bytes());
        let sp_max = match c.sampling_max {
            Some(v) => v,
            None => 0.0,
        };
        bin[base + 144..base + 152].copy_from_slice(&sp_max.to_le_bytes());
        bin[base + 152..base + 156].copy_from_slice(&(c.start_byte as u32).to_le_bytes());
        bin[base + 156..base + 160].copy_from_slice(&(c.bytes as u32).to_le_bytes());
        let mut flags = 0u32;
        if c.missing_constant.is_some() {
            flags |= FLAG_MISSING;
        }
        if c.sampling_min.is_some() {
            flags |= FLAG_SP_MIN;
        }
        if c.sampling_max.is_some() {
            flags |= FLAG_SP_MAX;
        }
        bin[base + 160..base + 164].copy_from_slice(&flags.to_le_bytes());
    }
    let data_base = 12 + cols * COLUMN_META_BYTES;
    for (r, row) in table.rows.iter().enumerate() {
        let at = data_base + r * (cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES);
        let mut pw = vec![0u64; words];
        for (j, v) in row.values.iter().enumerate() {
            match v {
                Some(x) => {
                    bin[at + j * 8..at + j * 8 + 8].copy_from_slice(&x.to_le_bytes());
                    pw[j / 64] |= 1u64 << (j % 64);
                }
                None => {}
            }
        }
        for w in 0..words {
            let wb = at + cols * ROW_VALUE_BYTES + w * 8;
            bin[wb..wb + 8].copy_from_slice(&pw[w].to_le_bytes());
        }
    }
    bin
}

fn str_field(bytes: &[u8], from: usize, to: usize) -> Option<String> {
    let field = bytes.get(from..to)?;
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    String::from_utf8(field[..end].to_vec()).ok()
}

pub fn parse_table(bytes: &[u8]) -> Option<Pds3Table> {
    if bytes.len() < 12 || bytes[0..4] != MAGIC {
        return None;
    }
    let cols = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let row_count = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if cols == 0 {
        return None;
    }
    let words = (cols + 63) / 64;
    let expected = 12
        + cols * COLUMN_META_BYTES
        + row_count * (cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES);
    if bytes.len() != expected {
        return None;
    }
    let mut columns = Vec::with_capacity(cols);
    for i in 0..cols {
        let base = 12 + i * COLUMN_META_BYTES;
        let name = str_field(bytes, base, base + COLUMN_NAME_BYTES)?;
        let unit = str_field(bytes, base + 64, base + 64 + COLUMN_UNIT_BYTES)?;
        let data_type = str_field(bytes, base + 80, base + 80 + COLUMN_TYPE_BYTES)?;
        let sampling_name = str_field(bytes, base + 96, base + 96 + COLUMN_SP_NAME_BYTES)?;
        let sampling_unit = str_field(bytes, base + 112, base + 112 + COLUMN_SP_UNIT_BYTES)?;
        let missing = f64::from_le_bytes(bytes[base + 128..base + 136].try_into().ok()?);
        let sp_min = f64::from_le_bytes(bytes[base + 136..base + 144].try_into().ok()?);
        let sp_max = f64::from_le_bytes(bytes[base + 144..base + 152].try_into().ok()?);
        let start_byte =
            u32::from_le_bytes(bytes[base + 152..base + 156].try_into().ok()?) as usize;
        let nbytes = u32::from_le_bytes(bytes[base + 156..base + 160].try_into().ok()?) as usize;
        let flags = u32::from_le_bytes(bytes[base + 160..base + 164].try_into().ok()?);
        columns.push(TableColumn {
            name,
            unit,
            data_type,
            missing_constant: if flags & FLAG_MISSING != 0 {
                Some(missing)
            } else {
                None
            },
            sampling_name,
            sampling_unit,
            sampling_min: if flags & FLAG_SP_MIN != 0 {
                Some(sp_min)
            } else {
                None
            },
            sampling_max: if flags & FLAG_SP_MAX != 0 {
                Some(sp_max)
            } else {
                None
            },
            start_byte,
            bytes: nbytes,
        });
    }
    let data_base = 12 + cols * COLUMN_META_BYTES;
    let mut rows = Vec::with_capacity(row_count);
    for r in 0..row_count {
        let at = data_base + r * (cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES);
        let mut values = Vec::with_capacity(cols);
        for j in 0..cols {
            let wb = at + cols * ROW_VALUE_BYTES + (j / 64) * 8;
            let present = u64::from_le_bytes(bytes[wb..wb + 8].try_into().ok()?);
            let v = f64::from_le_bytes(bytes[at + j * 8..at + j * 8 + 8].try_into().ok()?);
            let value = if present & (1u64 << (j % 64)) != 0 && v.is_finite() {
                Some(v)
            } else {
                None
            };
            values.push(value);
        }
        rows.push(TableRow { values });
    }
    Some(Pds3Table { columns, rows })
}

#[cfg(test)]
mod tests {
    use super::*;

    const KRFM_LABEL: &str = "CCSD3ZF0000100000001NJPL3ZF0PDS200000001 = SFDU_LABEL
RECORD_TYPE             = FIXED_LENGTH
RECORD_BYTES            = 78
FILE_RECORDS            = 3338
^TABLE                  = \"KRFM.DAT\"
SPACECRAFT_NAME         = PHOBOS_2
TARGET_NAME             = MARS
INSTRUMENT_NAME         = \"KRFM-COMBINED RADIOMETER SPECTROPHOTOMETER
    FOR MARS\"
INSTRUMENT_ID           = KRFM
PRODUCT_ID              = P1203
START_TIME              = 1989-03-12T21:36:02.497Z

OBJECT                    = TABLE
  INTERCHANGE_FORMAT      = ASCII
  ROWS                    = 3338
  ROW_BYTES               = 78
  COLUMNS                 = 15

  OBJECT                    = COLUMN
    NAME                    = TIME_OFFSET
    DATA_TYPE               = INTEGER
    SAMPLING_PARAMETER_UNIT = SECOND
    START_BYTE              = 1
    BYTES                   = 6
  END_OBJECT                = COLUMN

  OBJECT                    = COLUMN
    NAME                    = RADIOMETER1
    DATA_TYPE               = INTEGER
    SAMPLING_PARAMETER_UNIT = MICROMETER
    MINIMUM_SAMPLING_PARAMETER = 5.89
    MAXIMUM_SAMPLING_PARAMETER = 8.24
    START_BYTE              = 8
    BYTES                   = 4
  END_OBJECT                = COLUMN

  OBJECT                    = COLUMN
    NAME                    = RADIOMETER4
    DATA_TYPE               = INTEGER
    SAMPLING_PARAMETR_UNIT  = MICROMETER
    MINIMUM_SAMPLING_PARAMETER = 13.70
    MAXIMUM_SAMPLING_PARAMETER = 16.72
    START_BYTE              = 23
    BYTES                   = 4
  END_OBJECT                = COLUMN

  OBJECT                    = COLUMN
    NAME                    = PHOTOMETER1
    DATA_TYPE               = INTEGER
    SAMPLING_PARAMETER_NAME = WAVELENGTH
    SAMPLING_PARAMETER_UNIT = MICROMETER
    MINIMUM_SAMPLING_PARAMETER = 320<NM>
    MAXIMUM_SAMPLING_PARAMETER = \"N/A\"
    START_BYTE              = 33
    BYTES                   = 4
  END_OBJECT                = COLUMN

END_OBJECT                = TABLE

END
";

    const KRFM_ROWS: &str = "     0,1865,2654,2947,3168,3762,  42, 232, 284, 293, 291, 285, 244, 234, 264\r\n\
     1,1841,2597,2919,3129,3762,  38, 257, 309, 319, 332, 330, 291, 281, 317\r\n";

    const VEGA_LABEL: &str = "PDS_VERSION_ID                = PDS3
RECORD_TYPE                   = FIXED_LENGTH
RECORD_BYTES                  = 89
FILE_RECORDS                  = 260
^TABLE                        = \"0221S.TAB\"
SPACECRAFT_NAME               = \"VEGA-2\"
INSTRUMENT_NAME               = \"FLUXGATE MAGNETOMETER\"
TARGET_NAME                   = \"HALLEY\"
START_TIME                    = 1986-02-21T02:51:52.000Z
OBJECT                        = TABLE
  INTERCHANGE_FORMAT          = ASCII
  ROWS                        = 260
  COLUMNS                     = 8
  ROW_BYTES                   = 89

  OBJECT                      = COLUMN
    NAME                      = \"UT\"
    DATA_TYPE                 = TIME
    START_BYTE                = 1
    BYTES                     = 24
    FORMAT                    = \"A24\"
  END_OBJECT                  = COLUMN

  OBJECT                      = COLUMN
    NAME                      = \"BX PSSO\"
    UNIT                      = \"NANOTESLA\"
    DATA_TYPE                 = ASCII_REAL
    START_BYTE                = 26
    BYTES                     = 8
    MISSING_CONSTANT          = -999.999
    FORMAT                    = \"F8.3\"
  END_OBJECT                  = COLUMN

  OBJECT                      = COLUMN
    NAME                      = \"BY PSSO\"
    UNIT                      = \"NANOTESLA\"
    DATA_TYPE                 = ASCII_REAL
    START_BYTE                = 35
    BYTES                     = 8
    MISSING_CONSTANT          = -999.999
    FORMAT                    = \"F8.3\"
  END_OBJECT                  = COLUMN

  OBJECT                      = COLUMN
    NAME                      = \"BZ PSSO\"
    UNIT                      = \"NANOTESLA\"
    DATA_TYPE                 = ASCII_REAL
    START_BYTE                = 44
    BYTES                     = 8
    MISSING_CONSTANT          = -999.999
    FORMAT                    = \"F8.3\"
  END_OBJECT                  = COLUMN

  OBJECT                      = COLUMN
    NAME                      = \"BT PSSO\"
    UNIT                      = \"NANOTESLA\"
    DATA_TYPE                 = ASCII_REAL
    START_BYTE                = 53
    BYTES                     = 8
    MISSING_CONSTANT          = -999.999
    FORMAT                    = \"F8.3\"
  END_OBJECT                  = COLUMN

  OBJECT                      = COLUMN
    NAME                      = \"BU PSSO\"
    UNIT                      = \"NANOTESLA\"
    DATA_TYPE                 = ASCII_REAL
    START_BYTE                = 62
    BYTES                     = 8
    MISSING_CONSTANT          = -999.999
    FORMAT                    = \"F8.3\"
  END_OBJECT                  = COLUMN

  OBJECT                      = COLUMN
    NAME                      = \"BUXPSSO\"
    UNIT                      = \"NANOTESLA\"
    DATA_TYPE                 = ASCII_REAL
    START_BYTE                = 71
    BYTES                     = 8
    MISSING_CONSTANT          = -999.999
    FORMAT                    = \"F8.3\"
  END_OBJECT                  = COLUMN

  OBJECT                      = COLUMN
    NAME                      = \"FLAG\"
    DATA_TYPE                 = ASCII_REAL
    START_BYTE                = 80
    BYTES                     = 8
    MISSING_CONSTANT          = -999.999
    FORMAT                    = \"F8.3\"
  END_OBJECT                  = COLUMN

END_OBJECT                    = TABLE
END
";

    const VEGA_ROWS: &str = "1986-02-21T02:51:52.000Z    9.486    9.902   -0.375   13.718  -22.803  -21.289    0.000\r\n\
1986-02-21T02:53:05.000Z    9.145 -999.999  -12.143 -999.999  -22.510  -20.654    0.000\r\n";

    #[test]
    fn parse_label_reads_the_krfm_label() {
        let meta = parse_label(KRFM_LABEL).expect("label parses");
        assert_eq!(meta.record_bytes, Some(78));
        assert_eq!(meta.file_records, Some(3338));
        assert_eq!(meta.interchange.as_deref(), Some("ASCII"));
        assert_eq!(meta.table_file.as_deref(), Some("KRFM.DAT"));
        assert_eq!(meta.start_time.as_deref(), Some("1989-03-12T21:36:02.497Z"));
        assert_eq!(meta.columns.len(), 4);
        assert_eq!(meta.columns[0].name, "TIME_OFFSET");
        assert_eq!(meta.columns[0].start_byte, 1);
        assert_eq!(meta.columns[0].bytes, 6);
        assert_eq!(meta.columns[1].name, "RADIOMETER1");
        assert_eq!(meta.columns[1].sampling_min, Some(5.89));
        assert_eq!(meta.columns[1].sampling_max, Some(8.24));
        assert_eq!(meta.columns[2].name, "RADIOMETER4");
        assert_eq!(meta.columns[2].sampling_unit, "MICROMETER");
        assert_eq!(meta.columns[3].name, "PHOTOMETER1");
        assert_eq!(meta.columns[3].sampling_name, "WAVELENGTH");
        assert_eq!(meta.columns[3].sampling_min, Some(320.0));
        assert_eq!(meta.columns[3].sampling_max, None);
    }

    #[test]
    fn decode_reads_the_krfm_rows() {
        let meta = parse_label(KRFM_LABEL).expect("label parses");
        let (rows, skipped, trailing) = decode_rows(KRFM_ROWS.as_bytes(), &meta).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0][0], Some(0.0));
        assert_eq!(rows[1][0], Some(1.0));
        assert_eq!(rows[0][1], Some(1865.0));
        assert_eq!(rows[1][2], Some(3129.0));
        assert_eq!(rows[0][3], Some(42.0));
    }

    #[test]
    fn decode_reads_the_vega_rows_and_missing_cells() {
        let meta = parse_label(VEGA_LABEL).expect("label parses");
        let (rows, skipped, trailing) = decode_rows(VEGA_ROWS.as_bytes(), &meta).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0][0], Some(509338312.0));
        assert_eq!(rows[1][0], Some(509338385.0));
        assert_eq!(rows[0][1], Some(9.486));
        assert_eq!(rows[1][1], Some(9.145));
        assert_eq!(rows[1][2], None);
        assert_eq!(rows[1][3], Some(-12.143));
        assert_eq!(rows[1][4], None);
        assert_eq!(rows[1][7], Some(0.0));
    }

    #[test]
    fn record_stride_follows_the_label_counts() {
        let krfm = parse_label(KRFM_LABEL).expect("label parses");
        assert_eq!(record_stride(&krfm, 2 * 78), Some(78));
        assert_eq!(record_stride(&krfm, 3339 * 78), Some(78));
        let vega = parse_label(VEGA_LABEL).expect("label parses");
        assert_eq!(record_stride(&vega, 2 * 89), Some(89));
        assert_eq!(record_stride(&vega, 260 * 89), Some(89));
    }

    #[test]
    fn record_stride_detects_a_crlf_appended_stride() {
        let meta = parse_label(VEGA_LABEL).expect("label parses");
        let mut meta_wo_terminator = meta.clone();
        meta_wo_terminator.record_bytes = Some(87);
        meta_wo_terminator.file_records = Some(260);
        assert_eq!(record_stride(&meta_wo_terminator, 260 * 89), Some(89));
    }

    #[test]
    fn parse_cell_gates_nan_inf_and_blank() {
        assert_eq!(
            parse_cell(b"1986-02-21T02:51:52.000Z", "TIME", None),
            Some(509338312.0)
        );
        assert_eq!(parse_cell(b"  42  ", "INTEGER", None), Some(42.0));
        assert_eq!(parse_cell(b"-999.999", "ASCII_REAL", Some(-999.999)), None);
        assert_eq!(
            parse_cell(b"9.486", "ASCII_REAL", Some(-999.999)),
            Some(9.486)
        );
        assert_eq!(parse_cell(b"", "ASCII_REAL", None), None);
        assert_eq!(parse_cell(b"   ", "ASCII_REAL", None), None);
        assert_eq!(parse_cell(b"nan", "ASCII_REAL", None), None);
        assert_eq!(parse_cell(b"inf", "ASCII_REAL", None), None);
        assert_eq!(parse_cell(b"1e308", "ASCII_REAL", None), Some(1e308));
        assert_eq!(parse_cell(b"N/A", "ASCII_REAL", None), None);
    }

    #[test]
    fn odl_kv_strips_comments_and_quotes() {
        let text = "/* head */\nKEY = \"value\" /* tail */\nEMPTY = \"\"\nNO_EQUALS\n";
        let kv = odl_kv(text);
        assert_eq!(kv, vec![("KEY".to_string(), "value".to_string())]);
    }

    #[test]
    fn pack_and_roundtrip_hold_with_missing_cells() {
        let meta = parse_label(VEGA_LABEL).expect("label parses");
        let (rows, _, _) = decode_rows(VEGA_ROWS.as_bytes(), &meta).expect("rows");
        let columns = meta.columns.clone();
        let table = Pds3Table {
            columns,
            rows: rows.into_iter().map(|values| TableRow { values }).collect(),
        };
        let bin = pack(&table);
        let parsed = parse_table(&bin).expect("packed table parses");
        assert_eq!(parsed, table);
    }

    #[test]
    fn parse_table_rejects_foreign_bytes() {
        assert!(parse_table(b"").is_none());
        assert!(parse_table(b"XXXX").is_none());
        let meta = parse_label(VEGA_LABEL).expect("label parses");
        let (rows, _, _) = decode_rows(VEGA_ROWS.as_bytes(), &meta).expect("rows");
        let columns = meta.columns.clone();
        let table = Pds3Table {
            columns,
            rows: rows.into_iter().map(|values| TableRow { values }).collect(),
        };
        let mut bin = pack(&table);
        bin.truncate(bin.len() - 1);
        assert!(parse_table(&bin).is_none());
    }
}
