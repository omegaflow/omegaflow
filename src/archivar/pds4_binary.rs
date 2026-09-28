use crate::archivar::pds4::{XElem, parse_xml};

pub const MAGIC: [u8; 4] = *b"P4TB";
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
pub struct Pds4BinaryColumn {
    pub name: String,
    pub unit: Option<String>,
    pub data_type: Option<String>,
    pub missing_constant: Option<f64>,
    pub sampling_name: String,
    pub sampling_unit: String,
    pub sampling_min: Option<f64>,
    pub sampling_max: Option<f64>,
    pub start_byte: Option<usize>,
    pub bytes: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pds4BinaryRow {
    pub values: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pds4BinaryTable {
    pub columns: Vec<Pds4BinaryColumn>,
    pub rows: Vec<Pds4BinaryRow>,
}

#[derive(Clone, Debug)]
pub struct Pds4BinaryMeta {
    pub table_class: String,
    pub table_offset: Option<usize>,
    pub record_length: Option<usize>,
    pub fields: Option<usize>,
    pub groups: Option<usize>,
    pub file_records: Option<usize>,
    pub rows: Option<usize>,
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub start_time: Option<String>,
    pub stop_time: Option<String>,
    pub mission_name: Option<String>,
    pub instrument_name: Option<String>,
    pub host_name: Option<String>,
    pub target_name: Option<String>,
    pub title: Option<String>,
    pub logical_identifier: Option<String>,
    pub columns: Vec<Pds4BinaryColumn>,
}

type RowValues = Vec<Option<f64>>;
type DecodeOutput = (Vec<RowValues>, usize, usize);

fn local_name(name: &str) -> &str {
    name.rsplit_once(':').map_or(name, |(_, l)| l)
}

fn local_child<'a>(elem: &'a XElem, name: &str) -> Option<&'a XElem> {
    elem.children.iter().find(|c| local_name(&c.name) == name)
}

fn local_children<'a>(elem: &'a XElem, name: &str) -> Vec<&'a XElem> {
    elem.children
        .iter()
        .filter(|c| local_name(&c.name) == name)
        .collect()
}

fn local_text(elem: &XElem, name: &str) -> Option<String> {
    local_child(elem, name)
        .map(|c| c.text.trim().to_string())
        .filter(|t| !t.is_empty())
}

fn parse_number(v: &str) -> Option<f64> {
    v.trim().parse::<f64>().ok().filter(|x| x.is_finite())
}

pub fn parse_label(text: &str) -> Option<Pds4BinaryMeta> {
    let doc = parse_xml(text)?;
    let mut meta = Pds4BinaryMeta {
        table_class: String::new(),
        table_offset: None,
        record_length: None,
        fields: None,
        groups: None,
        file_records: None,
        rows: None,
        file_name: None,
        file_size: None,
        start_time: None,
        stop_time: None,
        mission_name: None,
        instrument_name: None,
        host_name: None,
        target_name: None,
        title: None,
        logical_identifier: None,
        columns: Vec::new(),
    };
    if let Some(ident) = local_child(&doc, "Identification_Area") {
        meta.logical_identifier = local_text(ident, "logical_identifier");
        meta.title = local_text(ident, "title");
    }
    if let Some(obs) = local_child(&doc, "Observation_Area") {
        if let Some(tc) = local_child(obs, "Time_Coordinates") {
            meta.start_time = local_text(tc, "start_date_time");
            meta.stop_time = local_text(tc, "stop_date_time");
        }
        if let Some(inv) = local_child(obs, "Investigation_Area") {
            meta.mission_name = local_text(inv, "name");
        }
        if let Some(sys) = local_child(obs, "Observing_System") {
            for comp in local_children(sys, "Observing_System_Component") {
                let ty = local_text(comp, "type");
                let name = local_text(comp, "name");
                match ty.as_deref() {
                    Some("Instrument") => meta.instrument_name = name,
                    Some("Host") => meta.host_name = name,
                    _ => {}
                }
            }
        }
        if let Some(tgt) = local_child(obs, "Target_Identification") {
            meta.target_name = local_text(tgt, "name");
        }
    }
    let fao = local_child(&doc, "File_Area_Observational")?;
    if let Some(file) = local_child(fao, "File") {
        meta.file_name = local_text(file, "file_name");
        meta.file_size = local_text(file, "file_size").and_then(|s| s.trim().parse::<u64>().ok());
        meta.file_records =
            local_text(file, "records").and_then(|s| s.trim().parse::<usize>().ok());
    }
    let table = local_child(fao, "Table_Binary")?;
    meta.table_class = "Table_Binary".to_string();
    meta.table_offset = local_text(table, "offset").and_then(|s| s.trim().parse::<usize>().ok());
    meta.rows = local_text(table, "records").and_then(|s| s.trim().parse::<usize>().ok());
    let record = local_child(table, "Record_Binary")?;
    meta.fields = local_text(record, "fields").and_then(|s| s.trim().parse::<usize>().ok());
    meta.groups = local_text(record, "groups").and_then(|s| s.trim().parse::<usize>().ok());
    meta.record_length =
        local_text(record, "record_length").and_then(|s| s.trim().parse::<usize>().ok());
    for f in local_children(record, "Field_Binary") {
        let Some(name) = local_text(f, "name") else {
            continue;
        };
        let data_type = local_text(f, "data_type");
        let unit = local_text(f, "unit");
        let start_byte = local_text(f, "field_location")
            .and_then(|s| s.trim().parse::<usize>().ok())
            .filter(|v| *v > 0);
        let bytes = local_text(f, "field_length")
            .and_then(|s| s.trim().parse::<usize>().ok())
            .filter(|v| *v > 0);
        let mut missing_constant = None;
        if let Some(sc) = local_child(f, "Special_Constants") {
            missing_constant = local_text(sc, "missing_constant").and_then(|s| parse_number(&s));
        }
        if start_byte.is_none() || bytes.is_none() {
            continue;
        }
        meta.columns.push(Pds4BinaryColumn {
            name,
            unit,
            data_type,
            missing_constant,
            sampling_name: String::new(),
            sampling_unit: String::new(),
            sampling_min: None,
            sampling_max: None,
            start_byte,
            bytes,
        });
    }
    meta.columns.sort_by_key(|c| c.start_byte);
    Some(meta)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryCellReject {
    Short,
    NotFinite,
    MissingConstant,
    UnsupportedType(String),
}

fn take_fixed<const N: usize>(field: &[u8]) -> Result<[u8; N], BinaryCellReject> {
    let arr: &[u8; N] = field
        .get(..N)
        .and_then(|s| <&[u8; N]>::try_from(s).ok())
        .ok_or(BinaryCellReject::Short)?;
    Ok(*arr)
}

pub fn parse_binary_cell(
    field: &[u8],
    data_type: &str,
    missing: Option<f64>,
) -> Result<f64, BinaryCellReject> {
    let norm = data_type.to_ascii_uppercase().replace('_', "");
    let value = match norm.as_str() {
        "IEEE754MSBSINGLE" => {
            let v = f32::from_be_bytes(take_fixed(field)?);
            if v.is_finite() {
                v as f64
            } else {
                return Err(BinaryCellReject::NotFinite);
            }
        }
        "IEEE754LSBSINGLE" => {
            let v = f32::from_le_bytes(take_fixed(field)?);
            if v.is_finite() {
                v as f64
            } else {
                return Err(BinaryCellReject::NotFinite);
            }
        }
        "IEEE754MSBDOUBLE" => {
            let v = f64::from_be_bytes(take_fixed(field)?);
            if v.is_finite() {
                v
            } else {
                return Err(BinaryCellReject::NotFinite);
            }
        }
        "IEEE754LSBDOUBLE" => {
            let v = f64::from_le_bytes(take_fixed(field)?);
            if v.is_finite() {
                v
            } else {
                return Err(BinaryCellReject::NotFinite);
            }
        }
        "SIGNEDBYTE" => i8::from_be_bytes(take_fixed(field)?) as f64,
        "UNSIGNEDBYTE" => take_fixed::<1>(field)?[0] as f64,
        "SIGNEDMSB2" => i16::from_be_bytes(take_fixed(field)?) as f64,
        "SIGNEDLSB2" => i16::from_le_bytes(take_fixed(field)?) as f64,
        "UNSIGNEDMSB2" => u16::from_be_bytes(take_fixed(field)?) as f64,
        "UNSIGNEDLSB2" => u16::from_le_bytes(take_fixed(field)?) as f64,
        "SIGNEDMSB4" => i32::from_be_bytes(take_fixed(field)?) as f64,
        "SIGNEDLSB4" => i32::from_le_bytes(take_fixed(field)?) as f64,
        "UNSIGNEDMSB4" => u32::from_be_bytes(take_fixed(field)?) as f64,
        "UNSIGNEDLSB4" => u32::from_le_bytes(take_fixed(field)?) as f64,
        "SIGNEDMSB8" => i64::from_be_bytes(take_fixed(field)?) as f64,
        "SIGNEDLSB8" => i64::from_le_bytes(take_fixed(field)?) as f64,
        "UNSIGNEDMSB8" => u64::from_be_bytes(take_fixed(field)?) as f64,
        "UNSIGNEDLSB8" => u64::from_le_bytes(take_fixed(field)?) as f64,
        other => return Err(BinaryCellReject::UnsupportedType(other.to_string())),
    };
    match missing {
        Some(m) if value == m => Err(BinaryCellReject::MissingConstant),
        _ => Ok(value),
    }
}

pub fn data_span(meta: &Pds4BinaryMeta) -> Option<usize> {
    meta.columns
        .iter()
        .filter_map(|c| {
            let start = c.start_byte?.checked_sub(1)?;
            Some(start + c.bytes?)
        })
        .max()
}

pub fn record_stride(meta: &Pds4BinaryMeta, data_len: usize) -> Option<usize> {
    let span = data_span(meta)?;
    let mut candidates: Vec<usize> = Vec::new();
    match meta.record_length {
        Some(rl) if rl >= span => {
            candidates.push(rl);
            candidates.push(rl + 2);
            candidates.push(rl + 1);
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
        .filter(|s| *s > 0 && data_len.is_multiple_of(*s))
        .collect();
    if divisors.is_empty() {
        return None;
    }
    let declared = meta.rows.or(meta.file_records);
    match declared {
        Some(n) if n > 0 => match divisors.iter().find(|s| data_len / **s == n) {
            Some(s) => Some(*s),
            None => divisors.into_iter().min(),
        },
        _ => divisors.into_iter().min(),
    }
}

pub fn decode_rows(bytes: &[u8], meta: &Pds4BinaryMeta) -> Option<DecodeOutput> {
    if meta.groups != Some(0) {
        return None;
    }
    let offset = match meta.table_offset {
        Some(o) => o,
        None => 0,
    };
    let data = bytes.get(offset..)?;
    let stride = record_stride(meta, data.len())?;
    let span = data_span(meta)?;
    let mut complete = data.len() / stride;
    let mut trailing = data.len() - complete * stride;
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
        let end = (at + stride).min(data.len());
        let rec = &data[at..end];
        let mut vals = Vec::with_capacity(meta.columns.len());
        for c in &meta.columns {
            let from = c.start_byte? - 1;
            let nbytes = c.bytes?;
            let v = match (rec.get(from..from + nbytes), c.data_type.as_deref()) {
                (Some(field), Some(data_type)) => {
                    parse_binary_cell(field, data_type, c.missing_constant).ok()
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

pub fn pack(table: &Pds4BinaryTable) -> Vec<u8> {
    let cols = table.columns.len();
    let words = cols.div_ceil(64);
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
        if let Some(unit) = &c.unit {
            let unit = unit.as_bytes();
            let u = unit.len().min(COLUMN_UNIT_BYTES);
            bin[base + 64..base + 64 + u].copy_from_slice(&unit[..u]);
        }
        if let Some(data_type) = &c.data_type {
            let dt = data_type.as_bytes();
            let d = dt.len().min(COLUMN_TYPE_BYTES);
            bin[base + 80..base + 80 + d].copy_from_slice(&dt[..d]);
        }
        let sn = c.sampling_name.as_bytes();
        let snl = sn.len().min(COLUMN_SP_NAME_BYTES);
        bin[base + 96..base + 96 + snl].copy_from_slice(&sn[..snl]);
        let su = c.sampling_unit.as_bytes();
        let sul = su.len().min(COLUMN_SP_UNIT_BYTES);
        bin[base + 112..base + 112 + sul].copy_from_slice(&su[..sul]);
        if let Some(v) = c.missing_constant {
            bin[base + 128..base + 136].copy_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = c.sampling_min {
            bin[base + 136..base + 144].copy_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = c.sampling_max {
            bin[base + 144..base + 152].copy_from_slice(&v.to_le_bytes());
        }
        let start_byte = match c.start_byte {
            Some(v) => v as u32,
            None => 0,
        };
        bin[base + 152..base + 156].copy_from_slice(&start_byte.to_le_bytes());
        let nbytes = match c.bytes {
            Some(v) => v as u32,
            None => 0,
        };
        bin[base + 156..base + 160].copy_from_slice(&nbytes.to_le_bytes());
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
            if let Some(x) = v {
                bin[at + j * 8..at + j * 8 + 8].copy_from_slice(&x.to_le_bytes());
                pw[j / 64] |= 1u64 << (j % 64);
            }
        }
        for (w, word) in pw.iter().enumerate() {
            let wb = at + cols * ROW_VALUE_BYTES + w * 8;
            bin[wb..wb + 8].copy_from_slice(&word.to_le_bytes());
        }
    }
    bin
}

fn str_field(bytes: &[u8], from: usize, to: usize) -> Option<String> {
    let field = bytes.get(from..to)?;
    let end = match field.iter().position(|b| *b == 0) {
        Some(p) => p,
        None => field.len(),
    };
    String::from_utf8(field[..end].to_vec()).ok()
}

pub fn parse_table(bytes: &[u8]) -> Option<Pds4BinaryTable> {
    if bytes.len() < 12 || bytes[0..4] != MAGIC {
        return None;
    }
    let cols = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let row_count = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    if cols == 0 {
        return None;
    }
    let words = cols.div_ceil(64);
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
        let unit =
            str_field(bytes, base + 64, base + 64 + COLUMN_UNIT_BYTES).filter(|s| !s.is_empty());
        let data_type =
            str_field(bytes, base + 80, base + 80 + COLUMN_TYPE_BYTES).filter(|s| !s.is_empty());
        let sampling_name = str_field(bytes, base + 96, base + 96 + COLUMN_SP_NAME_BYTES)?;
        let sampling_unit = str_field(bytes, base + 112, base + 112 + COLUMN_SP_UNIT_BYTES)?;
        let missing = f64::from_le_bytes(bytes[base + 128..base + 136].try_into().ok()?);
        let sp_min = f64::from_le_bytes(bytes[base + 136..base + 144].try_into().ok()?);
        let sp_max = f64::from_le_bytes(bytes[base + 144..base + 152].try_into().ok()?);
        let start_byte = match u32::from_le_bytes(bytes[base + 152..base + 156].try_into().ok()?) {
            0 => None,
            v => Some(v as usize),
        };
        let nbytes = match u32::from_le_bytes(bytes[base + 156..base + 160].try_into().ok()?) {
            0 => None,
            v => Some(v as usize),
        };
        let flags = u32::from_le_bytes(bytes[base + 160..base + 164].try_into().ok()?);
        columns.push(Pds4BinaryColumn {
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
        rows.push(Pds4BinaryRow { values });
    }
    Some(Pds4BinaryTable { columns, rows })
}

#[cfg(test)]
mod tests {
    use super::*;

    const BINARY_LABEL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Product_Observational xmlns="http://pds.nasa.gov/pds4/pds/v1">
  <Identification_Area>
    <logical_identifier>urn:test:binary</logical_identifier>
    <title>synthetic table binary</title>
  </Identification_Area>
  <Observation_Area>
    <Time_Coordinates>
      <start_date_time>2018-01-01T00:00:00Z</start_date_time>
      <stop_date_time>2018-01-01T01:00:00Z</stop_date_time>
    </Time_Coordinates>
    <Investigation_Area>
      <name>TEST</name>
    </Investigation_Area>
    <Observing_System>
      <Observing_System_Component>
        <name>TEST-HOST</name>
        <type>Host</type>
      </Observing_System_Component>
      <Observing_System_Component>
        <name>TEST-INST</name>
        <type>Instrument</type>
      </Observing_System_Component>
    </Observing_System>
    <Target_Identification>
      <name>MARS</name>
    </Target_Identification>
  </Observation_Area>
  <File_Area_Observational>
    <File>
      <file_name>sample.dat</file_name>
      <file_size unit="byte">26</file_size>
      <records>2</records>
    </File>
    <Table_Binary>
      <offset unit="byte">0</offset>
      <records>2</records>
      <Record_Binary>
        <fields>3</fields>
        <groups>0</groups>
        <record_length unit="byte">13</record_length>
        <Field_Binary>
          <name>TIME</name>
          <field_location unit="byte">1</field_location>
          <data_type>IEEE754LSBDouble</data_type>
          <field_length unit="byte">8</field_length>
          <unit>s</unit>
        </Field_Binary>
        <Field_Binary>
          <name>DOPPLER</name>
          <field_location unit="byte">9</field_location>
          <data_type>SignedMSB4</data_type>
          <field_length unit="byte">4</field_length>
          <unit>Hz</unit>
          <Special_Constants>
            <missing_constant>-2147483648</missing_constant>
          </Special_Constants>
        </Field_Binary>
        <Field_Binary>
          <name>QUALITY</name>
          <field_location unit="byte">13</field_location>
          <data_type>UnsignedByte</data_type>
          <field_length unit="byte">1</field_length>
        </Field_Binary>
      </Record_Binary>
    </Table_Binary>
  </File_Area_Observational>
</Product_Observational>
"#;

    fn record_bytes(t: f64, doppler: i32, quality: u8) -> Vec<u8> {
        let mut b = Vec::with_capacity(13);
        b.extend_from_slice(&t.to_le_bytes());
        b.extend_from_slice(&doppler.to_be_bytes());
        b.push(quality);
        b
    }

    fn two_records() -> Vec<u8> {
        let mut data = record_bytes(1.5, 123, 7);
        data.extend_from_slice(&record_bytes(2.5, -2147483648, 255));
        data
    }

    #[test]
    fn parse_label_reads_the_table_binary_label() {
        let meta = parse_label(BINARY_LABEL).expect("label parses");
        assert_eq!(meta.table_class, "Table_Binary");
        assert_eq!(meta.table_offset, Some(0));
        assert_eq!(meta.rows, Some(2));
        assert_eq!(meta.file_records, Some(2));
        assert_eq!(meta.record_length, Some(13));
        assert_eq!(meta.fields, Some(3));
        assert_eq!(meta.groups, Some(0));
        assert_eq!(meta.file_name.as_deref(), Some("sample.dat"));
        assert_eq!(meta.file_size, Some(26));
        assert_eq!(meta.mission_name.as_deref(), Some("TEST"));
        assert_eq!(meta.host_name.as_deref(), Some("TEST-HOST"));
        assert_eq!(meta.instrument_name.as_deref(), Some("TEST-INST"));
        assert_eq!(meta.target_name.as_deref(), Some("MARS"));
        assert_eq!(meta.logical_identifier.as_deref(), Some("urn:test:binary"));
        assert_eq!(meta.columns.len(), 3);
        assert_eq!(meta.columns[0].name, "TIME");
        assert_eq!(
            meta.columns[0].data_type.as_deref(),
            Some("IEEE754LSBDouble")
        );
        assert_eq!(meta.columns[0].start_byte, Some(1));
        assert_eq!(meta.columns[0].bytes, Some(8));
        assert_eq!(meta.columns[0].unit.as_deref(), Some("s"));
        assert_eq!(meta.columns[1].name, "DOPPLER");
        assert_eq!(meta.columns[1].start_byte, Some(9));
        assert_eq!(meta.columns[1].bytes, Some(4));
        assert_eq!(meta.columns[1].missing_constant, Some(-2147483648.0));
        assert_eq!(meta.columns[2].name, "QUALITY");
        assert_eq!(meta.columns[2].start_byte, Some(13));
        assert_eq!(meta.columns[2].bytes, Some(1));
    }

    #[test]
    fn decode_reads_binary_records_column_wise() {
        let meta = parse_label(BINARY_LABEL).expect("label parses");
        let (rows, skipped, trailing) = decode_rows(&two_records(), &meta).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0].len(), 3);
        assert_eq!(rows[0][0], Some(1.5));
        assert_eq!(rows[0][1], Some(123.0));
        assert_eq!(rows[0][2], Some(7.0));
        assert_eq!(rows[1][0], Some(2.5));
        assert_eq!(rows[1][1], None);
        assert_eq!(rows[1][2], Some(255.0));
    }

    #[test]
    fn nan_and_inf_cells_are_none_never_zero() {
        let meta = parse_label(BINARY_LABEL).expect("label parses");
        let mut data = record_bytes(f64::NAN, 1, 2);
        data.extend_from_slice(&record_bytes(f64::INFINITY, 3, 4));
        let (rows, skipped, _) = decode_rows(&data, &meta).expect("rows");
        assert_eq!(skipped, 2);
        assert_eq!(rows[0][0], None);
        assert_eq!(rows[1][0], None);
    }

    #[test]
    fn parse_binary_cell_decodes_every_scalar_type() {
        assert_eq!(
            parse_binary_cell(&1.5f64.to_le_bytes(), "IEEE754LSBDouble", None),
            Ok(1.5)
        );
        assert_eq!(
            parse_binary_cell(&2.5f64.to_be_bytes(), "IEEE754MSBDouble", None),
            Ok(2.5)
        );
        assert_eq!(
            parse_binary_cell(&1.25f32.to_le_bytes(), "IEEE754LSBSingle", None),
            Ok(1.25)
        );
        assert_eq!(
            parse_binary_cell(&(-2.5f32).to_be_bytes(), "IEEE754MSBSingle", None),
            Ok(-2.5)
        );
        assert_eq!(parse_binary_cell(&[1], "SignedByte", None), Ok(1.0));
        assert_eq!(parse_binary_cell(&[250], "SignedByte", None), Ok(-6.0));
        assert_eq!(parse_binary_cell(&[255], "UnsignedByte", None), Ok(255.0));
        assert_eq!(
            parse_binary_cell(&(-2i16).to_be_bytes(), "SignedMSB2", None),
            Ok(-2.0)
        );
        assert_eq!(
            parse_binary_cell(&3i16.to_le_bytes(), "SignedLSB2", None),
            Ok(3.0)
        );
        assert_eq!(
            parse_binary_cell(&65530u16.to_be_bytes(), "UnsignedMSB2", None),
            Ok(65530.0)
        );
        assert_eq!(
            parse_binary_cell(&65531u16.to_le_bytes(), "UnsignedLSB2", None),
            Ok(65531.0)
        );
        assert_eq!(
            parse_binary_cell(&(-40000i32).to_be_bytes(), "SignedMSB4", None),
            Ok(-40000.0)
        );
        assert_eq!(
            parse_binary_cell(&123456i32.to_le_bytes(), "SignedLSB4", None),
            Ok(123456.0)
        );
        assert_eq!(
            parse_binary_cell(&4000000000u32.to_be_bytes(), "UnsignedMSB4", None),
            Ok(4000000000.0)
        );
        assert_eq!(
            parse_binary_cell(&4000000001u32.to_le_bytes(), "UnsignedLSB4", None),
            Ok(4000000001.0)
        );
        assert_eq!(
            parse_binary_cell(&(-6i64).to_be_bytes(), "SignedMSB8", None),
            Ok(-6.0)
        );
        assert_eq!(
            parse_binary_cell(&7i64.to_le_bytes(), "SignedLSB8", None),
            Ok(7.0)
        );
        assert_eq!(
            parse_binary_cell(&u64::MAX.to_be_bytes(), "UnsignedMSB8", None),
            Ok(u64::MAX as f64)
        );
        assert_eq!(
            parse_binary_cell(&u64::MAX.to_le_bytes(), "UnsignedLSB8", None),
            Ok(u64::MAX as f64)
        );
    }

    #[test]
    fn parse_binary_cell_names_rejections() {
        assert_eq!(
            parse_binary_cell(&123i32.to_be_bytes(), "SignedMSB4", Some(123.0)),
            Err(BinaryCellReject::MissingConstant)
        );
        assert_eq!(
            parse_binary_cell(&f64::NAN.to_le_bytes(), "IEEE754LSBDouble", None),
            Err(BinaryCellReject::NotFinite)
        );
        assert_eq!(
            parse_binary_cell(&f64::INFINITY.to_le_bytes(), "IEEE754LSBDouble", None),
            Err(BinaryCellReject::NotFinite)
        );
        assert_eq!(
            parse_binary_cell(&[0, 0, 0], "IEEE754LSBDouble", None),
            Err(BinaryCellReject::Short)
        );
        assert_eq!(
            parse_binary_cell(&[0u8; 8], "UnsignedBitString", None),
            Err(BinaryCellReject::UnsupportedType(
                "UNSIGNEDBITSTRING".to_string()
            ))
        );
        assert_eq!(
            parse_binary_cell(&[0u8; 16], "ComplexMSB8", None),
            Err(BinaryCellReject::UnsupportedType("COMPLEXMSB8".to_string()))
        );
        assert_eq!(
            parse_binary_cell(&[0u8; 4], "ASCII_REAL", None),
            Err(BinaryCellReject::UnsupportedType("ASCIIREAL".to_string()))
        );
    }

    #[test]
    fn group_records_and_absent_groups_are_named_absence() {
        let data = two_records();
        let mut groups_one = parse_label(BINARY_LABEL).expect("label parses");
        groups_one.groups = Some(1);
        assert!(decode_rows(&data, &groups_one).is_none());
        let mut groups_absent = parse_label(BINARY_LABEL).expect("label parses");
        groups_absent.groups = None;
        assert!(decode_rows(&data, &groups_absent).is_none());
    }

    #[test]
    fn table_offset_moves_the_record_start() {
        let mut meta = parse_label(BINARY_LABEL).expect("label parses");
        meta.table_offset = Some(2);
        let mut data = vec![0xDE, 0xAD];
        data.extend_from_slice(&record_bytes(1.5, 123, 7));
        data.extend_from_slice(&record_bytes(2.5, 5, 6));
        let (rows, skipped, trailing) = decode_rows(&data, &meta).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0][0], Some(1.5));
        assert_eq!(rows[0][1], Some(123.0));
        assert_eq!(rows[0][2], Some(7.0));
        assert_eq!(rows[1][0], Some(2.5));
    }

    #[test]
    fn prefixed_namespace_labels_parse() {
        let label = r#"<pds:Product_Observational xmlns:pds="http://pds.nasa.gov/pds4/pds/v1">
  <pds:File_Area_Observational>
    <pds:File>
      <pds:file_name>sample.dat</pds:file_name>
      <pds:records>1</pds:records>
    </pds:File>
    <pds:Table_Binary>
      <pds:offset unit="byte">0</pds:offset>
      <pds:records>1</pds:records>
      <pds:Record_Binary>
        <pds:fields>1</pds:fields>
        <pds:groups>0</pds:groups>
        <pds:record_length unit="byte">8</pds:record_length>
        <pds:Field_Binary>
          <pds:name>TIME</pds:name>
          <pds:field_location unit="byte">1</pds:field_location>
          <pds:data_type>IEEE754LSBDouble</pds:data_type>
          <pds:field_length unit="byte">8</pds:field_length>
        </pds:Field_Binary>
      </pds:Record_Binary>
    </pds:Table_Binary>
  </pds:File_Area_Observational>
</pds:Product_Observational>
"#;
        let meta = parse_label(label).expect("prefixed label parses");
        assert_eq!(meta.table_class, "Table_Binary");
        assert_eq!(meta.file_name.as_deref(), Some("sample.dat"));
        assert_eq!(meta.columns.len(), 1);
        assert_eq!(meta.columns[0].name, "TIME");
        assert_eq!(
            meta.columns[0].data_type.as_deref(),
            Some("IEEE754LSBDouble")
        );
        let (rows, _, _) = decode_rows(&3.5f64.to_le_bytes(), &meta).expect("rows");
        assert_eq!(rows[0][0], Some(3.5));
    }

    #[test]
    fn pack_and_roundtrip_hold_with_missing_cells() {
        let meta = parse_label(BINARY_LABEL).expect("label parses");
        let (rows, _, _) = decode_rows(&two_records(), &meta).expect("rows");
        let columns = meta.columns.clone();
        let table = Pds4BinaryTable {
            columns,
            rows: rows
                .into_iter()
                .map(|values| Pds4BinaryRow { values })
                .collect(),
        };
        let bin = pack(&table);
        assert_eq!(&bin[0..4], b"P4TB");
        let parsed = parse_table(&bin).expect("packed table parses");
        assert_eq!(parsed, table);
    }

    #[test]
    fn parse_table_rejects_foreign_bytes() {
        assert!(parse_table(b"").is_none());
        assert!(parse_table(b"XXXX").is_none());
        assert!(parse_table(b"P4FW000000000000").is_none());
        let meta = parse_label(BINARY_LABEL).expect("label parses");
        let (rows, _, _) = decode_rows(&two_records(), &meta).expect("rows");
        let columns = meta.columns.clone();
        let table = Pds4BinaryTable {
            columns,
            rows: rows
                .into_iter()
                .map(|values| Pds4BinaryRow { values })
                .collect(),
        };
        let mut bin = pack(&table);
        bin.truncate(bin.len() - 1);
        assert!(parse_table(&bin).is_none());
    }
}
