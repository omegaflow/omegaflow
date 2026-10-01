use crate::archivar::pds3_table::odl_kv;

pub const MAGIC: [u8; 4] = *b"P3BN";
pub const MAGIC_V2: [u8; 4] = *b"P3BV";
pub const FORMAT_VERSION: u32 = 2;
const HEADER_V1_BYTES: usize = 12;
const HEADER_V2_BYTES: usize = 16;
pub const COLUMN_NAME_BYTES: usize = 64;
pub const COLUMN_UNIT_BYTES: usize = 16;
pub const COLUMN_TYPE_BYTES: usize = 32;
const UNIT_OFFSET: usize = COLUMN_NAME_BYTES;
const TYPE_OFFSET: usize = UNIT_OFFSET + COLUMN_UNIT_BYTES;
const MISSING_OFFSET: usize = TYPE_OFFSET + COLUMN_TYPE_BYTES;
const START_OFFSET: usize = MISSING_OFFSET + 8;
const NBYTES_OFFSET: usize = START_OFFSET + 4;
const FLAGS_OFFSET: usize = NBYTES_OFFSET + 4;
pub const COLUMN_META_BYTES: usize = FLAGS_OFFSET + 4;
pub const ROW_VALUE_BYTES: usize = 8;
pub const ROW_PRESENCE_BYTES: usize = 8;

const FLAG_MISSING: u32 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct BinColumn {
    pub name: String,
    pub unit: Option<String>,
    pub data_type: Option<String>,
    pub missing_constant: Option<f64>,
    pub start_byte: usize,
    pub bytes: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinRow {
    pub values: Vec<Option<f64>>,
    pub arrays: Vec<Vec<u8>>,
}

fn numeric_type(up: &str) -> bool {
    matches!(
        up,
        "MSB_INTEGER"
            | "SUN_INTEGER"
            | "SIGNED_INTEGER"
            | "LSB_INTEGER"
            | "INTEL_INTEGER"
            | "MSB_UNSIGNED_INTEGER"
            | "SUN_UNSIGNED_INTEGER"
            | "LSB_UNSIGNED_INTEGER"
            | "INTEL_UNSIGNED_INTEGER"
            | "IEEE_REAL"
            | "MSB_REAL"
            | "LSB_REAL"
            | "PC_REAL"
    )
}

pub fn is_array_column(c: &BinColumn) -> bool {
    let Some(data_type) = c.data_type.as_deref() else {
        return false;
    };
    numeric_type(&data_type.to_ascii_uppercase()) && !matches!(c.bytes, 1 | 2 | 4 | 8)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pds3BinaryTable {
    pub columns: Vec<BinColumn>,
    pub rows: Vec<BinRow>,
}

#[derive(Clone, Debug)]
pub struct BinMeta {
    pub record_type: Option<String>,
    pub record_bytes: Option<usize>,
    pub file_records: Option<usize>,
    pub data_file: Option<String>,
    pub start_time: Option<String>,
    pub stop_time: Option<String>,
    pub spacecraft_name: Option<String>,
    pub target_name: Option<String>,
    pub instrument_name: Option<String>,
    pub product_id: Option<String>,
    pub columns: Vec<BinColumn>,
}

type RowValues = Vec<Option<f64>>;
type DecodeOutput = (Vec<RowValues>, usize, usize);

pub fn parse_label(text: &str) -> Option<BinMeta> {
    let mut meta = BinMeta {
        record_type: None,
        record_bytes: None,
        file_records: None,
        data_file: None,
        start_time: None,
        stop_time: None,
        spacecraft_name: None,
        target_name: None,
        instrument_name: None,
        product_id: None,
        columns: Vec::new(),
    };
    let mut column: Option<BinColumn> = None;
    for (key, value) in odl_kv(text) {
        match key.as_str() {
            "OBJECT" => {
                if value == "COLUMN" {
                    column = Some(BinColumn {
                        name: String::new(),
                        unit: None,
                        data_type: None,
                        missing_constant: None,
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
                    c.unit = Some(value);
                }
            }
            "DATA_TYPE" => {
                if let Some(c) = column.as_mut() {
                    c.data_type = Some(value);
                }
            }
            "START_BYTE" => {
                if let Some(c) = column.as_mut()
                    && let Ok(v) = value.parse()
                {
                    c.start_byte = v;
                }
            }
            "BYTES" => {
                if let Some(c) = column.as_mut()
                    && let Ok(v) = value.parse()
                {
                    c.bytes = v;
                }
            }
            "MISSING_CONSTANT" => {
                if let Some(c) = column.as_mut() {
                    c.missing_constant = value.parse::<f64>().ok().filter(|x| x.is_finite());
                }
            }
            "END_OBJECT" => {
                if value == "COLUMN"
                    && let Some(c) = column.take()
                    && c.start_byte > 0
                    && c.bytes > 0
                {
                    meta.columns.push(c);
                }
            }
            "RECORD_TYPE" => meta.record_type = Some(value),
            "RECORD_BYTES" => meta.record_bytes = value.parse().ok(),
            "FILE_RECORDS" => meta.file_records = value.parse().ok(),
            "^TABLE" | "^DATA_SET" => meta.data_file = Some(value),
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

fn int_of(raw: &[u8], big_endian: bool, signed: bool) -> Option<f64> {
    let be = |b: [u8; 2]| {
        if big_endian {
            i16::from_be_bytes(b)
        } else {
            i16::from_le_bytes(b)
        }
    };
    let be4 = |b: [u8; 4]| {
        if big_endian {
            i32::from_be_bytes(b)
        } else {
            i32::from_le_bytes(b)
        }
    };
    let be8 = |b: [u8; 8]| {
        if big_endian {
            i64::from_be_bytes(b)
        } else {
            i64::from_le_bytes(b)
        }
    };
    let ue = |b: [u8; 2]| {
        if big_endian {
            u16::from_be_bytes(b)
        } else {
            u16::from_le_bytes(b)
        }
    };
    let ue4 = |b: [u8; 4]| {
        if big_endian {
            u32::from_be_bytes(b)
        } else {
            u32::from_le_bytes(b)
        }
    };
    let ue8 = |b: [u8; 8]| {
        if big_endian {
            u64::from_be_bytes(b)
        } else {
            u64::from_le_bytes(b)
        }
    };
    let v = match raw.len() {
        1 => {
            if signed {
                raw[0] as i8 as f64
            } else {
                raw[0] as f64
            }
        }
        2 => {
            let b = [raw[0], raw[1]];
            if signed { be(b) as f64 } else { ue(b) as f64 }
        }
        4 => {
            let b = [raw[0], raw[1], raw[2], raw[3]];
            if signed { be4(b) as f64 } else { ue4(b) as f64 }
        }
        8 => {
            let b = [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ];
            if signed { be8(b) as f64 } else { ue8(b) as f64 }
        }
        _ => return None,
    };
    Some(v)
}

pub fn real_of(raw: &[u8], big_endian: bool) -> Option<f64> {
    let v = match raw.len() {
        4 => {
            let b = [raw[0], raw[1], raw[2], raw[3]];
            if big_endian {
                f32::from_be_bytes(b) as f64
            } else {
                f32::from_le_bytes(b) as f64
            }
        }
        8 => {
            let b = [
                raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
            ];
            if big_endian {
                f64::from_be_bytes(b)
            } else {
                f64::from_le_bytes(b)
            }
        }
        _ => return None,
    };
    if v.is_finite() { Some(v) } else { None }
}

fn decode_binary_cell(field: &[u8], data_type: &str, missing: Option<f64>) -> Option<f64> {
    let up = data_type.to_ascii_uppercase();
    if up == "TIME" || up.starts_with("ASCII") {
        return crate::archivar::pds3_table::parse_cell(field, &up, missing);
    }
    let raw = match up.as_str() {
        "MSB_INTEGER" | "SUN_INTEGER" | "SIGNED_INTEGER" => int_of(field, true, true),
        "LSB_INTEGER" | "INTEL_INTEGER" => int_of(field, false, true),
        "MSB_UNSIGNED_INTEGER" | "SUN_UNSIGNED_INTEGER" => int_of(field, true, false),
        "LSB_UNSIGNED_INTEGER" | "INTEL_UNSIGNED_INTEGER" => int_of(field, false, false),
        "IEEE_REAL" | "MSB_REAL" => real_of(field, true),
        "LSB_REAL" | "PC_REAL" => real_of(field, false),
        "INTEGER"
        | "UNSIGNED_INTEGER"
        | "REAL"
        | "FLOAT"
        | "PC_INTEGER"
        | "PC_UNSIGNED_INTEGER"
        | "VAX_INTEGER"
        | "VAX_REAL" => return None,
        _ => return None,
    };
    match raw {
        Some(v) if !v.is_finite() => None,
        Some(v) => match missing {
            Some(m) if v == m => None,
            _ => Some(v),
        },
        None => None,
    }
}

pub fn decode_rows(bytes: &[u8], meta: &BinMeta) -> Option<DecodeOutput> {
    if meta
        .record_type
        .as_deref()
        .is_some_and(|r| !r.eq_ignore_ascii_case("FIXED_LENGTH"))
    {
        return None;
    }
    let stride = meta.record_bytes?;
    if stride == 0 {
        return None;
    }
    let complete = bytes.len() / stride;
    let trailing = bytes.len() - complete * stride;
    if complete == 0 {
        return None;
    }
    let mut rows = Vec::with_capacity(complete);
    let mut skipped = 0usize;
    for i in 0..complete {
        let at = i * stride;
        let rec = bytes.get(at..at + stride)?;
        let mut vals = Vec::with_capacity(meta.columns.len());
        for c in &meta.columns {
            let from = c.start_byte.checked_sub(1)?;
            let v = match (rec.get(from..from + c.bytes), c.data_type.as_deref()) {
                (Some(field), Some(data_type)) => {
                    decode_binary_cell(field, data_type, c.missing_constant)
                }
                _ => None,
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

pub fn pack(table: &Pds3BinaryTable) -> Vec<u8> {
    let cols = table.columns.len();
    let words = cols.div_ceil(64);
    let scalar_row = cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES;
    let array_bytes: usize = table
        .columns
        .iter()
        .filter(|c| is_array_column(c))
        .map(|c| c.bytes)
        .sum();
    let row_stride = scalar_row + array_bytes;
    let mut bin =
        vec![0u8; HEADER_V2_BYTES + cols * COLUMN_META_BYTES + table.rows.len() * row_stride];
    bin[0..4].copy_from_slice(&MAGIC_V2);
    bin[4..8].copy_from_slice(&FORMAT_VERSION.to_le_bytes());
    bin[8..12].copy_from_slice(&(cols as u32).to_le_bytes());
    bin[12..16].copy_from_slice(&(table.rows.len() as u32).to_le_bytes());
    for (i, c) in table.columns.iter().enumerate() {
        let base = HEADER_V2_BYTES + i * COLUMN_META_BYTES;
        let name = c.name.as_bytes();
        let n = name.len().min(COLUMN_NAME_BYTES);
        bin[base..base + n].copy_from_slice(&name[..n]);
        if let Some(unit) = &c.unit {
            let unit = unit.as_bytes();
            let u = unit.len().min(COLUMN_UNIT_BYTES);
            bin[base + UNIT_OFFSET..base + UNIT_OFFSET + u].copy_from_slice(&unit[..u]);
        }
        if let Some(data_type) = &c.data_type {
            let dt = data_type.as_bytes();
            let d = dt.len().min(COLUMN_TYPE_BYTES);
            bin[base + TYPE_OFFSET..base + TYPE_OFFSET + d].copy_from_slice(&dt[..d]);
        }
        let mut flags = 0u32;
        if let Some(v) = c.missing_constant {
            bin[base + MISSING_OFFSET..base + MISSING_OFFSET + 8].copy_from_slice(&v.to_le_bytes());
            flags |= FLAG_MISSING;
        }
        bin[base + START_OFFSET..base + START_OFFSET + 4]
            .copy_from_slice(&(c.start_byte as u32).to_le_bytes());
        bin[base + NBYTES_OFFSET..base + NBYTES_OFFSET + 4]
            .copy_from_slice(&(c.bytes as u32).to_le_bytes());
        bin[base + FLAGS_OFFSET..base + FLAGS_OFFSET + 4].copy_from_slice(&flags.to_le_bytes());
    }
    let data_base = HEADER_V2_BYTES + cols * COLUMN_META_BYTES;
    for (r, row) in table.rows.iter().enumerate() {
        let at = data_base + r * row_stride;
        let mut pw = vec![0u64; words];
        for (j, v) in row.values.iter().enumerate() {
            if let Some(x) = v {
                bin[at + j * 8..at + j * 8 + 8].copy_from_slice(&x.to_le_bytes());
                pw[j / 64] |= 1u64 << (j % 64);
            }
        }
        for (w, word) in pw.iter().enumerate() {
            let wb = at + cols * ROW_VALUE_BYTES + w * 8;
            bin[wb..wb + 8].copy_from_slice(&word.to_le_bytes());
        }
        let mut off = at + scalar_row;
        for (j, c) in table.columns.iter().enumerate() {
            if !is_array_column(c) {
                continue;
            }
            if let Some(raw) = row.arrays.get(j) {
                let n = raw.len().min(c.bytes);
                bin[off..off + n].copy_from_slice(&raw[..n]);
            }
            off += c.bytes;
        }
    }
    bin
}

fn str_field(bytes: &[u8], from: usize, to: usize) -> Option<String> {
    let field = bytes.get(from..to)?;
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    String::from_utf8(field[..end].to_vec()).ok()
}

pub fn parse_table(bytes: &[u8]) -> Option<Pds3BinaryTable> {
    if bytes.len() < 4 {
        return None;
    }
    let (header_bytes, versioned) = if bytes[0..4] == MAGIC_V2 {
        if bytes.len() < HEADER_V2_BYTES {
            return None;
        }
        let version = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
        if version != FORMAT_VERSION {
            return None;
        }
        (HEADER_V2_BYTES, true)
    } else if bytes[0..4] == MAGIC {
        if bytes.len() < HEADER_V1_BYTES {
            return None;
        }
        (HEADER_V1_BYTES, false)
    } else {
        return None;
    };
    let cols =
        u32::from_le_bytes(bytes[header_bytes - 8..header_bytes - 4].try_into().ok()?) as usize;
    let row_count =
        u32::from_le_bytes(bytes[header_bytes - 4..header_bytes].try_into().ok()?) as usize;
    if cols == 0 {
        return None;
    }
    let words = cols.div_ceil(64);
    let scalar_row = cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES;
    let mut columns = Vec::with_capacity(cols);
    for i in 0..cols {
        let base = header_bytes + i * COLUMN_META_BYTES;
        let name = str_field(bytes, base, base + COLUMN_NAME_BYTES)?;
        let unit = str_field(
            bytes,
            base + UNIT_OFFSET,
            base + UNIT_OFFSET + COLUMN_UNIT_BYTES,
        )
        .filter(|s| !s.is_empty());
        let data_type = str_field(
            bytes,
            base + TYPE_OFFSET,
            base + TYPE_OFFSET + COLUMN_TYPE_BYTES,
        )
        .filter(|s| !s.is_empty());
        let missing = f64::from_le_bytes(
            bytes[base + MISSING_OFFSET..base + MISSING_OFFSET + 8]
                .try_into()
                .ok()?,
        );
        let start_byte = u32::from_le_bytes(
            bytes[base + START_OFFSET..base + START_OFFSET + 4]
                .try_into()
                .ok()?,
        ) as usize;
        let nbytes = u32::from_le_bytes(
            bytes[base + NBYTES_OFFSET..base + NBYTES_OFFSET + 4]
                .try_into()
                .ok()?,
        ) as usize;
        let flags = u32::from_le_bytes(
            bytes[base + FLAGS_OFFSET..base + FLAGS_OFFSET + 4]
                .try_into()
                .ok()?,
        );
        columns.push(BinColumn {
            name,
            unit,
            data_type,
            missing_constant: if flags & FLAG_MISSING != 0 {
                Some(missing)
            } else {
                None
            },
            start_byte,
            bytes: nbytes,
        });
    }
    let array_bytes: usize = columns
        .iter()
        .filter(|c| is_array_column(c))
        .map(|c| c.bytes)
        .sum();
    let row_stride = if versioned {
        scalar_row + array_bytes
    } else {
        scalar_row
    };
    let expected = header_bytes + cols * COLUMN_META_BYTES + row_count * row_stride;
    if bytes.len() != expected {
        return None;
    }
    let data_base = header_bytes + cols * COLUMN_META_BYTES;
    let mut rows = Vec::with_capacity(row_count);
    for r in 0..row_count {
        let at = data_base + r * row_stride;
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
        let mut arrays = Vec::with_capacity(cols);
        if versioned {
            let mut off = at + scalar_row;
            for c in &columns {
                if is_array_column(c) {
                    arrays.push(bytes.get(off..off + c.bytes)?.to_vec());
                    off += c.bytes;
                } else {
                    arrays.push(Vec::new());
                }
            }
        } else {
            arrays.resize(cols, Vec::new());
        }
        rows.push(BinRow { values, arrays });
    }
    Some(Pds3BinaryTable { columns, rows })
}

#[cfg(test)]
mod tests {
    use super::*;

    const LRS_LABEL: &str = "PDS_VERSION_ID = PDS3
RECORD_TYPE = FIXED_LENGTH
RECORD_BYTES = 12
FILE_RECORDS = 2
^DATA_SET = \"LRS_RAW.DAT\"
SPACECRAFT_NAME = KAGUYA
TARGET_NAME = MOON
INSTRUMENT_NAME = \"LUNAR RADAR SOUNDER\"
START_TIME = 2007-09-14T01:31:01.000Z

OBJECT = TABLE
  COLUMNS = 3
  OBJECT = COLUMN
    NAME = PACKET_TIME
    DATA_TYPE = MSB_INTEGER
    START_BYTE = 1
    BYTES = 4
  END_OBJECT = COLUMN
  OBJECT = COLUMN
    NAME = ECHO_POWER
    DATA_TYPE = MSB_UNSIGNED_INTEGER
    START_BYTE = 5
    BYTES = 4
  END_OBJECT = COLUMN
  OBJECT = COLUMN
    NAME = STATUS
    DATA_TYPE = LSB_INTEGER
    MISSING_CONSTANT = -1
    START_BYTE = 9
    BYTES = 4
  END_OBJECT = COLUMN
END_OBJECT = TABLE
END
";

    fn lrs_records() -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&1000i32.to_be_bytes());
        out.extend_from_slice(&42u32.to_be_bytes());
        out.extend_from_slice(&7i32.to_le_bytes());
        out.extend_from_slice(&2000i32.to_be_bytes());
        out.extend_from_slice(&0u32.to_be_bytes());
        out.extend_from_slice(&(-1i32).to_le_bytes());
        out
    }

    #[test]
    fn parse_label_reads_the_binary_label() {
        let meta = parse_label(LRS_LABEL).expect("label parses");
        assert_eq!(meta.record_type.as_deref(), Some("FIXED_LENGTH"));
        assert_eq!(meta.record_bytes, Some(12));
        assert_eq!(meta.file_records, Some(2));
        assert_eq!(meta.data_file.as_deref(), Some("LRS_RAW.DAT"));
        assert_eq!(meta.spacecraft_name.as_deref(), Some("KAGUYA"));
        assert_eq!(meta.target_name.as_deref(), Some("MOON"));
        assert_eq!(meta.columns.len(), 3);
        assert_eq!(meta.columns[0].name, "PACKET_TIME");
        assert_eq!(meta.columns[0].data_type.as_deref(), Some("MSB_INTEGER"));
        assert_eq!(meta.columns[0].start_byte, 1);
        assert_eq!(meta.columns[0].bytes, 4);
        assert_eq!(
            meta.columns[1].data_type.as_deref(),
            Some("MSB_UNSIGNED_INTEGER")
        );
        assert_eq!(meta.columns[2].missing_constant, Some(-1.0));
    }

    #[test]
    fn decode_reads_binary_records_and_skips_the_missing_constant() {
        let meta = parse_label(LRS_LABEL).expect("label parses");
        let bytes = lrs_records();
        let (rows, skipped, trailing) = decode_rows(&bytes, &meta).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0][0], Some(1000.0));
        assert_eq!(rows[0][1], Some(42.0));
        assert_eq!(rows[0][2], Some(7.0));
        assert_eq!(rows[1][0], Some(2000.0));
        assert_eq!(rows[1][1], Some(0.0));
        assert_eq!(rows[1][2], None);
    }

    #[test]
    fn decode_rejects_non_fixed_length_and_missing_record_bytes() {
        let mut meta = parse_label(LRS_LABEL).expect("label parses");
        meta.record_type = Some("STREAM".to_string());
        assert!(decode_rows(&lrs_records(), &meta).is_none());
        let mut meta = parse_label(LRS_LABEL).expect("label parses");
        meta.record_bytes = None;
        assert!(decode_rows(&lrs_records(), &meta).is_none());
    }

    #[test]
    fn decode_binary_cell_never_emits_a_fabricated_zero() {
        assert_eq!(
            decode_binary_cell(&[0xFF, 0xFE], "MSB_INTEGER", None),
            Some(-2.0)
        );
        assert_eq!(
            decode_binary_cell(&[0xFF, 0xFE], "LSB_INTEGER", None),
            Some(-257.0)
        );
        assert_eq!(
            decode_binary_cell(&[0xFF, 0xFE], "MSB_UNSIGNED_INTEGER", None),
            Some(65534.0)
        );
        assert_eq!(
            decode_binary_cell(&[0xFF, 0xFE], "LSB_UNSIGNED_INTEGER", None),
            Some(65279.0)
        );
        assert_eq!(decode_binary_cell(&[0x00, 0x00], "IEEE_REAL", None), None);
        assert_eq!(decode_binary_cell(&[0x00, 0x00], "VAX_REAL", None), None);
        assert_eq!(
            decode_binary_cell(&[0x3F, 0x80, 0x00, 0x00], "IEEE_REAL", None),
            Some(1.0)
        );
        assert_eq!(
            decode_binary_cell(&[0x00, 0x00, 0x80, 0x3F], "PC_REAL", None),
            Some(1.0)
        );
        assert_eq!(
            decode_binary_cell(&[0x7F, 0x80, 0x00, 0x00], "IEEE_REAL", None),
            None
        );
        assert_eq!(
            decode_binary_cell(&[0x00, 0x00], "UNKNOWN_TYPE", None),
            None
        );
        assert_eq!(
            decode_binary_cell(&[0xFF, 0xFF], "MSB_INTEGER", Some(-1.0)),
            None
        );
        assert_eq!(decode_binary_cell(b"", "MSB_INTEGER", None), None);
    }

    #[test]
    fn pack_and_roundtrip_hold_with_missing_cells() {
        let meta = parse_label(LRS_LABEL).expect("label parses");
        let (rows, _, _) = decode_rows(&lrs_records(), &meta).expect("rows");
        let columns = meta.columns.clone();
        let n = columns.len();
        let table = Pds3BinaryTable {
            columns,
            rows: rows
                .into_iter()
                .map(|values| BinRow {
                    values,
                    arrays: vec![Vec::new(); n],
                })
                .collect(),
        };
        let bin = pack(&table);
        let parsed = parse_table(&bin).expect("packed table parses");
        assert_eq!(parsed, table);
    }

    fn v1_fixture() -> Vec<u8> {
        let cols = 1usize;
        let rows = 1usize;
        let mut b = vec![0u8; HEADER_V1_BYTES + cols * COLUMN_META_BYTES + rows * (cols * 8 + 8)];
        b[0..4].copy_from_slice(&MAGIC);
        b[4..8].copy_from_slice(&(cols as u32).to_le_bytes());
        b[8..12].copy_from_slice(&(rows as u32).to_le_bytes());
        let base = HEADER_V1_BYTES;
        b[base] = b'X';
        let dt = b"MSB_INTEGER";
        b[base + TYPE_OFFSET..base + TYPE_OFFSET + dt.len()].copy_from_slice(dt);
        b[base + START_OFFSET..base + START_OFFSET + 4].copy_from_slice(&1u32.to_le_bytes());
        b[base + NBYTES_OFFSET..base + NBYTES_OFFSET + 4].copy_from_slice(&4u32.to_le_bytes());
        let at = HEADER_V1_BYTES + COLUMN_META_BYTES;
        b[at..at + 8].copy_from_slice(&5.0f64.to_le_bytes());
        b[at + 8..at + 16].copy_from_slice(&1u64.to_le_bytes());
        b
    }

    #[test]
    fn parse_table_reads_a_version_1_asset_unchanged() {
        let bin = v1_fixture();
        let table = parse_table(&bin).expect("v1 parses");
        assert_eq!(table.columns.len(), 1);
        assert_eq!(table.columns[0].name, "X");
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0].values, vec![Some(5.0)]);
        assert_eq!(table.rows[0].arrays, vec![Vec::<u8>::new()]);
    }

    #[test]
    fn parse_table_rejects_an_unseen_version_word() {
        let mut bin = v1_fixture();
        bin[0..4].copy_from_slice(&MAGIC_V2);
        bin[4..8].copy_from_slice(&3u32.to_le_bytes());
        assert!(parse_table(&bin).is_none());
    }

    #[test]
    fn pack_and_parse_roundtrip_a_version_2_array_column_byte_true() {
        let columns = vec![
            BinColumn {
                name: "S".to_string(),
                unit: None,
                data_type: Some("MSB_INTEGER".to_string()),
                missing_constant: None,
                start_byte: 1,
                bytes: 4,
            },
            BinColumn {
                name: "WAVEFORM".to_string(),
                unit: Some("digit".to_string()),
                data_type: Some("MSB_UNSIGNED_INTEGER".to_string()),
                missing_constant: None,
                start_byte: 5,
                bytes: 4096,
            },
        ];
        let raw: Vec<u8> = (0..4096).map(|i| (i % 251) as u8).collect();
        let table = Pds3BinaryTable {
            columns,
            rows: vec![BinRow {
                values: vec![Some(7.0), None],
                arrays: vec![Vec::new(), raw.clone()],
            }],
        };
        let bin = pack(&table);
        assert_eq!(&bin[0..4], &MAGIC_V2);
        assert_eq!(u32::from_le_bytes(bin[4..8].try_into().expect("word")), 2);
        let parsed = parse_table(&bin).expect("v2 parses");
        assert_eq!(parsed, table);
        assert_eq!(parsed.rows[0].arrays[1], raw);
    }

    #[test]
    fn parse_table_rejects_foreign_bytes() {
        assert!(parse_table(b"").is_none());
        assert!(parse_table(b"XXXX").is_none());
        assert!(parse_table(b"P3FW000000000000").is_none());
    }
}
