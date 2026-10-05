use crate::archivar::pds3_table::unix_of_iso;

pub const MAGIC: [u8; 4] = *b"P4FW";
pub const COLUMN_NAME_BYTES: usize = 64;
pub const COLUMN_UNIT_BYTES: usize = 16;
pub const COLUMN_TYPE_BYTES: usize = 32;
pub const COLUMN_SP_NAME_BYTES: usize = 16;
pub const COLUMN_SP_UNIT_BYTES: usize = 16;
const UNIT_OFFSET: usize = COLUMN_NAME_BYTES;
const TYPE_OFFSET: usize = UNIT_OFFSET + COLUMN_UNIT_BYTES;
const SP_NAME_OFFSET: usize = TYPE_OFFSET + COLUMN_TYPE_BYTES;
const SP_UNIT_OFFSET: usize = SP_NAME_OFFSET + COLUMN_SP_NAME_BYTES;
const MISSING_OFFSET: usize = SP_UNIT_OFFSET + COLUMN_SP_UNIT_BYTES;
const SP_MIN_OFFSET: usize = MISSING_OFFSET + 8;
const SP_MAX_OFFSET: usize = SP_MIN_OFFSET + 8;
const START_OFFSET: usize = SP_MAX_OFFSET + 8;
const NBYTES_OFFSET: usize = START_OFFSET + 4;
const FLAGS_OFFSET: usize = NBYTES_OFFSET + 4;
pub const COLUMN_META_BYTES: usize = FLAGS_OFFSET + 4;
pub const ROW_VALUE_BYTES: usize = 8;
pub const ROW_PRESENCE_BYTES: usize = 8;

const FLAG_MISSING: u32 = 1;
const FLAG_SP_MIN: u32 = 2;
const FLAG_SP_MAX: u32 = 4;
const TFLAG_DELIMITED: u32 = 1;
const TFLAG_DELIMITER_SHIFT: u32 = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct Pds4Column {
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
pub struct Pds4Row {
    pub values: Vec<Option<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pds4Table {
    pub columns: Vec<Pds4Column>,
    pub rows: Vec<Pds4Row>,
    pub delimited: bool,
    pub delimiter: Option<u8>,
}

type RowValues = Vec<Option<f64>>;
type DecodeOutput = (Vec<RowValues>, usize, usize);

#[derive(Clone, Debug)]
pub struct Pds4Meta {
    pub table_class: String,
    pub record_delimiter: Option<String>,
    pub field_delimiter: Option<String>,
    pub record_length: Option<usize>,
    pub fields: Option<usize>,
    pub groups: Option<usize>,
    pub file_records: Option<usize>,
    pub rows: Option<usize>,
    pub table_offset: Option<usize>,
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
    pub columns: Vec<Pds4Column>,
    pub delimited: bool,
    pub delimiter: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct XElem {
    pub name: String,
    pub text: String,
    pub children: Vec<XElem>,
}

impl XElem {
    fn child(&self, name: &str) -> Option<&XElem> {
        self.children.iter().find(|c| c.name == name)
    }

    fn children_named(&self, name: &str) -> Vec<&XElem> {
        self.children.iter().filter(|c| c.name == name).collect()
    }

    fn text_of(&self, name: &str) -> Option<String> {
        self.child(name)
            .map(|c| c.text.trim().to_string())
            .filter(|t| !t.is_empty())
    }
}

fn xml_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

fn find_sub(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn skip_ws(bytes: &[u8], pos: &mut usize) {
    while let Some(b) = bytes.get(*pos) {
        match b {
            b' ' | b'\t' | b'\r' | b'\n' => *pos += 1,
            _ => break,
        }
    }
}

fn skip_prolog(bytes: &[u8], pos: &mut usize) {
    loop {
        match bytes.get(*pos) {
            Some(b'<') => match bytes.get(*pos + 1) {
                Some(b'?') => match find_sub(&bytes[*pos..], b"?>") {
                    Some(rel) => *pos += rel + 2,
                    None => *pos += 1,
                },
                Some(b'!') if bytes.get(*pos + 2) == Some(&b'-') => {
                    match find_sub(&bytes[*pos..], b"-->") {
                        Some(rel) => *pos += rel + 3,
                        None => *pos += 1,
                    }
                }
                _ => return,
            },
            Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') => *pos += 1,
            _ => return,
        }
    }
}

fn read_name(bytes: &[u8], pos: &mut usize) -> Option<String> {
    let start = *pos;
    while let Some(b) = bytes.get(*pos) {
        match b {
            b'>' | b'/' | b' ' | b'\t' | b'\r' | b'\n' => break,
            _ => *pos += 1,
        }
    }
    if *pos == start {
        return None;
    }
    Some(String::from_utf8_lossy(&bytes[start..*pos]).to_string())
}

fn read_attrs(bytes: &[u8], pos: &mut usize) {
    loop {
        skip_ws(bytes, pos);
        match bytes.get(*pos) {
            Some(b'>') | Some(b'/') | None => return,
            _ => {}
        }
        let key_start = *pos;
        while let Some(b) = bytes.get(*pos) {
            match b {
                b'=' | b' ' | b'\t' | b'\r' | b'\n' | b'>' | b'/' => break,
                _ => *pos += 1,
            }
        }
        if *pos == key_start {
            *pos += 1;
            continue;
        }
        skip_ws(bytes, pos);
        if bytes.get(*pos) != Some(&b'=') {
            continue;
        }
        *pos += 1;
        skip_ws(bytes, pos);
        let q = match bytes.get(*pos) {
            Some(&c @ (b'"' | b'\'')) => c,
            _ => continue,
        };
        *pos += 1;
        while let Some(b) = bytes.get(*pos) {
            if *b == q {
                break;
            }
            *pos += 1;
        }
        if bytes.get(*pos) == Some(&q) {
            *pos += 1;
        }
    }
}

fn parse_element(bytes: &[u8], pos: &mut usize, depth: usize) -> Option<XElem> {
    if depth > 64 || bytes.get(*pos) != Some(&b'<') {
        return None;
    }
    *pos += 1;
    let name = read_name(bytes, pos)?;
    read_attrs(bytes, pos);
    match bytes.get(*pos) {
        Some(b'>') => *pos += 1,
        Some(b'/') if bytes.get(*pos + 1) == Some(&b'>') => {
            *pos += 2;
            return Some(XElem {
                name,
                text: String::new(),
                children: Vec::new(),
            });
        }
        _ => return None,
    }
    let close = format!("</{}>", name);
    let mut elem = XElem {
        name,
        text: String::new(),
        children: Vec::new(),
    };
    loop {
        let rest = &bytes[*pos..];
        let lt = find_sub(rest, b"<")?;
        let lt = *pos + lt;
        if lt > *pos {
            elem.text
                .push_str(&String::from_utf8_lossy(&bytes[*pos..lt]));
        }
        *pos = lt;
        if bytes[*pos..].starts_with(close.as_bytes()) {
            *pos += close.len();
            break;
        }
        match bytes.get(*pos + 1) {
            Some(b'/') => return None,
            Some(b'?') => {
                let rel = find_sub(&bytes[*pos..], b"?>")?;
                *pos += rel + 2;
            }
            Some(b'!') => {
                if bytes.get(*pos + 2) == Some(&b'-') {
                    let rel = find_sub(&bytes[*pos..], b"-->")?;
                    *pos += rel + 3;
                } else if bytes[*pos..].starts_with(b"<![CDATA[") {
                    let rel = find_sub(&bytes[*pos..], b"]]>")?;
                    elem.text
                        .push_str(&String::from_utf8_lossy(&bytes[*pos + 9..*pos + rel]));
                    *pos += rel + 3;
                } else {
                    let rel = find_sub(&bytes[*pos..], b">")?;
                    *pos += rel + 1;
                }
            }
            _ => {
                let child = parse_element(bytes, pos, depth + 1)?;
                elem.children.push(child);
            }
        }
    }
    elem.text = xml_unescape(elem.text.trim());
    Some(elem)
}

pub fn parse_xml(doc: &str) -> Option<XElem> {
    let bytes = doc.as_bytes();
    let mut pos = 0usize;
    skip_prolog(bytes, &mut pos);
    if bytes.get(pos) == Some(&b'<') {
        return parse_element(bytes, &mut pos, 0);
    }
    None
}

fn parse_number(v: &str) -> Option<f64> {
    v.trim().parse::<f64>().ok().filter(|x| x.is_finite())
}

fn field_delimiter_byte(name: &str) -> Option<u8> {
    match name {
        "Comma" => Some(b','),
        "Semicolon" => Some(b';'),
        "Tab" | "Horizontal Tab" => Some(b'\t'),
        "Vertical-Tab" => Some(b'\x0B'),
        "Pipe" => Some(b'|'),
        _ => None,
    }
}

fn record_delimiter_bytes(name: Option<&str>) -> Option<&'static [u8]> {
    match name {
        Some("Carriage-Return Line-Feed") => Some(b"\r\n"),
        Some("Line-Feed") => Some(b"\n"),
        Some("Carriage-Return") => Some(b"\r"),
        _ => None,
    }
}

fn group_column(field: &XElem, repetition_base: usize) -> Option<Pds4Column> {
    let name = field.text_of("name")?;
    let location = field
        .text_of("field_location")
        .and_then(|s| s.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)?;
    let bytes = field
        .text_of("field_length")
        .and_then(|s| s.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)?;
    let mut missing_constant = None;
    if let Some(sc) = field.child("Special_Constants") {
        missing_constant = sc
            .text_of("missing_constant")
            .and_then(|s| parse_number(&s));
    }
    Some(Pds4Column {
        name,
        unit: field.text_of("unit"),
        data_type: field.text_of("data_type"),
        missing_constant,
        sampling_name: String::new(),
        sampling_unit: String::new(),
        sampling_min: None,
        sampling_max: None,
        start_byte: Some(repetition_base + location),
        bytes: Some(bytes),
    })
}

fn expand_group(group: &XElem, parent_base: usize, out: &mut Vec<Pds4Column>) {
    let Some(repetitions) = group
        .text_of("repetitions")
        .and_then(|s| s.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)
    else {
        return;
    };
    let Some(group_location) = group
        .text_of("group_location")
        .and_then(|s| s.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)
    else {
        return;
    };
    let stride = match group
        .text_of("group_length")
        .and_then(|s| s.trim().parse::<usize>().ok())
    {
        Some(len) if len.is_multiple_of(repetitions) => len / repetitions,
        _ => return,
    };
    if stride == 0 {
        return;
    }
    let base = parent_base + group_location - 1;
    let fields = group.children_named("Field_Character");
    let subgroups = group.children_named("Group_Field_Character");
    for i in 0..repetitions {
        let repetition_base = base + i * stride;
        for field in &fields {
            if let Some(mut column) = group_column(field, repetition_base) {
                if repetitions > 1 {
                    column.name = format!("{}_{i:04}", column.name);
                }
                out.push(column);
            }
        }
        for sub in &subgroups {
            expand_group(sub, repetition_base, out);
        }
    }
}

pub fn parse_label(text: &str) -> Option<Pds4Meta> {
    parse_label_for_file(text, None)
}

pub fn parse_label_for_file(text: &str, file_name: Option<&str>) -> Option<Pds4Meta> {
    let doc = parse_xml(text)?;
    let mut meta = Pds4Meta {
        table_class: String::new(),
        record_delimiter: None,
        field_delimiter: None,
        record_length: None,
        fields: None,
        groups: None,
        file_records: None,
        rows: None,
        table_offset: None,
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
        delimited: false,
        delimiter: None,
    };
    if let Some(ident) = doc.child("Identification_Area") {
        meta.logical_identifier = ident.text_of("logical_identifier");
        meta.title = ident.text_of("title");
    }
    if let Some(obs) = doc.child("Observation_Area") {
        if let Some(tc) = obs.child("Time_Coordinates") {
            meta.start_time = tc.text_of("start_date_time");
            meta.stop_time = tc.text_of("stop_date_time");
        }
        if let Some(inv) = obs.child("Investigation_Area") {
            meta.mission_name = inv.text_of("name");
        }
        if let Some(sys) = obs.child("Observing_System") {
            for comp in sys.children_named("Observing_System_Component") {
                let ty = comp.text_of("type");
                let name = comp.text_of("name");
                match ty.as_deref() {
                    Some("Instrument") => meta.instrument_name = name,
                    Some("Host") => meta.host_name = name,
                    _ => {}
                }
            }
        }
        if let Some(tgt) = obs.child("Target_Identification") {
            meta.target_name = tgt.text_of("name");
        }
    }
    let fao = match file_name {
        Some(want) => doc
            .children_named("File_Area_Observational")
            .into_iter()
            .find(|fao| {
                fao.child("File")
                    .and_then(|f| f.text_of("file_name"))
                    .as_deref()
                    == Some(want)
            })?,
        None => doc.child("File_Area_Observational")?,
    };
    if let Some(file) = fao.child("File") {
        meta.file_name = file.text_of("file_name");
        meta.file_size = file
            .text_of("file_size")
            .and_then(|s| s.trim().parse::<u64>().ok());
        meta.file_records = file
            .text_of("records")
            .and_then(|s| s.trim().parse::<usize>().ok());
    }
    let (table, class, delimited) = match fao.child("Table_Character") {
        Some(t) => (t, "Table_Character", false),
        None => {
            let t = fao.child("Table_Delimited")?;
            (t, "Table_Delimited", true)
        }
    };
    meta.table_class = class.to_string();
    meta.delimited = delimited;
    meta.table_offset = table
        .text_of("offset")
        .and_then(|s| s.trim().parse::<usize>().ok());
    meta.rows = table
        .text_of("records")
        .and_then(|s| s.trim().parse::<usize>().ok());
    meta.record_delimiter = table.text_of("record_delimiter");
    meta.field_delimiter = table.text_of("field_delimiter");
    meta.delimiter = meta
        .field_delimiter
        .as_deref()
        .and_then(field_delimiter_byte);
    let record = if delimited {
        table.child("Record_Delimited")?
    } else {
        table.child("Record_Character")?
    };
    meta.fields = record
        .text_of("fields")
        .and_then(|s| s.trim().parse::<usize>().ok());
    meta.groups = record
        .text_of("groups")
        .and_then(|s| s.trim().parse::<usize>().ok());
    if !delimited {
        meta.record_length = record
            .text_of("record_length")
            .and_then(|s| s.trim().parse::<usize>().ok());
    }
    let field_tag = if delimited {
        "Field_Delimited"
    } else {
        "Field_Character"
    };
    for f in record.children_named(field_tag) {
        let Some(name) = f.text_of("name") else {
            continue;
        };
        let data_type = f.text_of("data_type");
        let unit = f.text_of("unit");
        let start_byte = if delimited {
            f.text_of("field_number")
                .and_then(|s| s.trim().parse::<usize>().ok())
                .filter(|v| *v > 0)
        } else {
            f.text_of("field_location")
                .and_then(|s| s.trim().parse::<usize>().ok())
                .filter(|v| *v > 0)
        };
        let bytes = if delimited {
            f.text_of("maximum_field_length")
                .and_then(|s| s.trim().parse::<usize>().ok())
                .filter(|v| *v > 0)
        } else {
            f.text_of("field_length")
                .and_then(|s| s.trim().parse::<usize>().ok())
                .filter(|v| *v > 0)
        };
        let mut missing_constant = None;
        if let Some(sc) = f.child("Special_Constants") {
            missing_constant = sc
                .text_of("missing_constant")
                .and_then(|s| parse_number(&s));
        }
        if !delimited && (start_byte.is_none() || bytes.is_none()) {
            continue;
        }
        meta.columns.push(Pds4Column {
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
    if !delimited {
        for group in record.children_named("Group_Field_Character") {
            expand_group(group, 0, &mut meta.columns);
        }
    }
    meta.columns.sort_by_key(|c| c.start_byte);
    Some(meta)
}

pub fn data_span(meta: &Pds4Meta) -> Option<usize> {
    meta.columns
        .iter()
        .filter_map(|c| {
            let start = c.start_byte?.checked_sub(1)?;
            Some(start + c.bytes?)
        })
        .max()
}

pub fn record_stride(meta: &Pds4Meta, file_len: usize) -> Option<usize> {
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
        .filter(|s| *s > 0 && file_len.is_multiple_of(*s))
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

pub fn parse_cell(field: &[u8], data_type: &str, missing: Option<f64>) -> Option<f64> {
    let text = match std::str::from_utf8(field) {
        Ok(t) => t.trim(),
        Err(_) => return None,
    };
    if text.is_empty() {
        return None;
    }
    let up = data_type.to_ascii_uppercase();
    let parsed = if up.starts_with("ASCII_DATE_TIME") {
        unix_of_iso(text)
    } else {
        match up.as_str() {
            "ASCII_INTEGER" | "ASCII_NONNEGATIVE_INTEGER" => match text.parse::<i64>() {
                Ok(v) => Some(v as f64),
                Err(_) => None,
            },
            "ASCII_REAL" => text.parse::<f64>().ok(),
            "ASCII_NUMERIC_BASE16" => u64::from_str_radix(text, 16).ok().map(|v| v as f64),
            _ => None,
        }
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

pub fn is_numeric_type(data_type: &str) -> bool {
    matches!(
        data_type.to_ascii_uppercase().as_str(),
        "ASCII_INTEGER" | "ASCII_NONNEGATIVE_INTEGER" | "ASCII_REAL" | "ASCII_NUMERIC_BASE16"
    )
}

pub enum Axis {
    IsoTime(usize),
    Offset(usize, f64, f64),
}

fn axis_column(meta: &Pds4Meta, idx: usize) -> Pds4Column {
    let c = &meta.columns[idx];
    Pds4Column {
        name: c.name.clone(),
        unit: Some("SECOND".to_string()),
        data_type: Some("TIME".to_string()),
        missing_constant: c.missing_constant,
        sampling_name: c.sampling_name.clone(),
        sampling_unit: c.sampling_unit.clone(),
        sampling_min: c.sampling_min,
        sampling_max: c.sampling_max,
        start_byte: c.start_byte,
        bytes: c.bytes,
    }
}

pub fn axis_of(meta: &Pds4Meta) -> Option<(Axis, String)> {
    for (i, c) in meta.columns.iter().enumerate() {
        if let Some(dt) = &c.data_type
            && dt.to_ascii_uppercase().starts_with("ASCII_DATE_TIME")
        {
            return Some((Axis::IsoTime(i), format!("{} ({dt} date-time)", c.name)));
        }
    }
    for (i, c) in meta.columns.iter().enumerate() {
        if c.name.eq_ignore_ascii_case("MET") {
            let base = unix_of_iso(meta.start_time.as_deref()?)?;
            return Some((
                Axis::Offset(i, base, 1.0 / 32.0),
                format!("{} (MET s ticks + START_TIME {base:.3})", c.name),
            ));
        }
    }
    None
}

pub fn assemble(
    meta: &Pds4Meta,
    raw_rows: Vec<Vec<Option<f64>>>,
    axis: Option<(Axis, String)>,
) -> Option<(Pds4Table, usize, String)> {
    let mut columns: Vec<Pds4Column> = Vec::new();
    let mut kept: Vec<usize> = Vec::new();
    let axis_note = match &axis {
        Some((_, note)) => note.clone(),
        None => String::from("none"),
    };
    match &axis {
        Some((Axis::IsoTime(i), _)) | Some((Axis::Offset(i, _, _), _)) => {
            columns.push(axis_column(meta, *i));
            for (j, c) in meta.columns.iter().enumerate() {
                if j != *i && c.data_type.as_deref().is_some_and(is_numeric_type) {
                    columns.push(c.clone());
                    kept.push(j);
                }
            }
        }
        None => {
            for (j, c) in meta.columns.iter().enumerate() {
                if c.data_type.as_deref().is_some_and(is_numeric_type) {
                    columns.push(c.clone());
                    kept.push(j);
                }
            }
        }
    }
    let mut rows: Vec<Pds4Row> = Vec::new();
    let mut skipped = 0usize;
    for raw in raw_rows {
        let mut values: Vec<Option<f64>> = Vec::new();
        match &axis {
            Some((Axis::IsoTime(i), _)) => match raw.get(*i) {
                Some(Some(v)) => values.push(Some(*v)),
                _ => {
                    skipped += 1;
                    continue;
                }
            },
            Some((Axis::Offset(i, base, scale), _)) => match raw.get(*i) {
                Some(Some(v)) => values.push(Some(v * scale + base)),
                _ => {
                    skipped += 1;
                    continue;
                }
            },
            None => {}
        }
        for j in &kept {
            values.push(match raw.get(*j) {
                Some(Some(v)) => Some(*v),
                _ => None,
            });
        }
        if axis.is_none() && values.iter().all(|v| v.is_none()) {
            skipped += 1;
            continue;
        }
        rows.push(Pds4Row { values });
    }
    if rows.is_empty() {
        return None;
    }
    Some((
        Pds4Table {
            columns,
            rows,
            delimited: false,
            delimiter: None,
        },
        skipped,
        axis_note,
    ))
}

fn decode_fixed(bytes: &[u8], meta: &Pds4Meta) -> Option<DecodeOutput> {
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
            let from = c.start_byte? - 1;
            let nbytes = c.bytes?;
            let v = match (rec.get(from..from + nbytes), c.data_type.as_deref()) {
                (Some(field), Some(data_type)) => parse_cell(field, data_type, c.missing_constant),
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

fn strip_quotes(s: &str) -> &str {
    let t = s.trim();
    if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') {
        &t[1..t.len() - 1]
    } else {
        t
    }
}

fn decode_delimited(bytes: &[u8], meta: &Pds4Meta) -> Option<DecodeOutput> {
    let rec_sep = record_delimiter_bytes(meta.record_delimiter.as_deref())?;
    let field_sep = meta.delimiter?;
    let mut rows: Vec<RowValues> = Vec::new();
    let mut skipped = 0usize;
    let mut trailing = 0usize;
    let mut rest: &[u8] = bytes;
    while !rest.is_empty() {
        let line = match find_sub(rest, rec_sep) {
            Some(rel) => {
                let line = &rest[..rel];
                rest = &rest[rel + rec_sep.len()..];
                line
            }
            None => {
                let line = rest;
                rest = &[];
                if line.is_empty() {
                    break;
                }
                trailing = line.len();
                line
            }
        };
        if line.is_empty() {
            continue;
        }
        let mut vals = Vec::with_capacity(meta.columns.len());
        let mut cells = line.split(|b| *b == field_sep);
        for c in &meta.columns {
            let cell = cells.next().unwrap_or(b"");
            let text = match std::str::from_utf8(cell) {
                Ok(t) => strip_quotes(t),
                Err(_) => "",
            };
            vals.push(match c.data_type.as_deref() {
                Some(data_type) => parse_cell(text.as_bytes(), data_type, c.missing_constant),
                None => None,
            });
        }
        if vals.iter().all(|v| v.is_none()) {
            skipped += 1;
        }
        rows.push(vals);
    }
    if rows.is_empty() {
        return None;
    }
    Some((rows, skipped, trailing))
}

pub fn decode_rows(bytes: &[u8], meta: &Pds4Meta) -> Option<DecodeOutput> {
    if meta.delimited {
        decode_delimited(bytes, meta)
    } else {
        decode_fixed(bytes, meta)
    }
}

pub fn pack(table: &Pds4Table) -> Vec<u8> {
    let cols = table.columns.len();
    let words = cols.div_ceil(64);
    let mut bin = vec![
        0u8;
        16 + cols * COLUMN_META_BYTES
            + table.rows.len()
                * (cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES)
    ];
    bin[0..4].copy_from_slice(&MAGIC);
    bin[4..8].copy_from_slice(&(cols as u32).to_le_bytes());
    bin[8..12].copy_from_slice(&(table.rows.len() as u32).to_le_bytes());
    let mut tflags = 0u32;
    if table.delimited {
        tflags |= TFLAG_DELIMITED;
        if let Some(d) = table.delimiter {
            tflags |= (d as u32) << TFLAG_DELIMITER_SHIFT;
        }
    }
    bin[12..16].copy_from_slice(&tflags.to_le_bytes());
    for (i, c) in table.columns.iter().enumerate() {
        let base = 16 + i * COLUMN_META_BYTES;
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
        let sn = c.sampling_name.as_bytes();
        let snl = sn.len().min(COLUMN_SP_NAME_BYTES);
        bin[base + SP_NAME_OFFSET..base + SP_NAME_OFFSET + snl].copy_from_slice(&sn[..snl]);
        let su = c.sampling_unit.as_bytes();
        let sul = su.len().min(COLUMN_SP_UNIT_BYTES);
        bin[base + SP_UNIT_OFFSET..base + SP_UNIT_OFFSET + sul].copy_from_slice(&su[..sul]);
        if let Some(v) = c.missing_constant {
            bin[base + MISSING_OFFSET..base + MISSING_OFFSET + 8].copy_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = c.sampling_min {
            bin[base + SP_MIN_OFFSET..base + SP_MIN_OFFSET + 8].copy_from_slice(&v.to_le_bytes());
        }
        if let Some(v) = c.sampling_max {
            bin[base + SP_MAX_OFFSET..base + SP_MAX_OFFSET + 8].copy_from_slice(&v.to_le_bytes());
        }
        let start_byte = match c.start_byte {
            Some(v) => v as u32,
            None => 0,
        };
        bin[base + START_OFFSET..base + START_OFFSET + 4]
            .copy_from_slice(&start_byte.to_le_bytes());
        let nbytes = match c.bytes {
            Some(v) => v as u32,
            None => 0,
        };
        bin[base + NBYTES_OFFSET..base + NBYTES_OFFSET + 4].copy_from_slice(&nbytes.to_le_bytes());
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
        bin[base + FLAGS_OFFSET..base + FLAGS_OFFSET + 4].copy_from_slice(&flags.to_le_bytes());
    }
    let data_base = 16 + cols * COLUMN_META_BYTES;
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
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    String::from_utf8(field[..end].to_vec()).ok()
}

pub fn parse_table(bytes: &[u8]) -> Option<Pds4Table> {
    if bytes.len() < 16 || bytes[0..4] != MAGIC {
        return None;
    }
    let cols = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let row_count = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let tflags = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let delimited = tflags & TFLAG_DELIMITED != 0;
    let delimiter = if delimited {
        match (tflags >> TFLAG_DELIMITER_SHIFT) & 0xFF {
            0 => None,
            d => Some(d as u8),
        }
    } else {
        None
    };
    if cols == 0 {
        return None;
    }
    let words = cols.div_ceil(64);
    let expected = 16
        + cols * COLUMN_META_BYTES
        + row_count * (cols * ROW_VALUE_BYTES + words * ROW_PRESENCE_BYTES);
    if bytes.len() != expected {
        return None;
    }
    let mut columns = Vec::with_capacity(cols);
    for i in 0..cols {
        let base = 16 + i * COLUMN_META_BYTES;
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
        let sampling_name = str_field(
            bytes,
            base + SP_NAME_OFFSET,
            base + SP_NAME_OFFSET + COLUMN_SP_NAME_BYTES,
        )?;
        let sampling_unit = str_field(
            bytes,
            base + SP_UNIT_OFFSET,
            base + SP_UNIT_OFFSET + COLUMN_SP_UNIT_BYTES,
        )?;
        let missing = f64::from_le_bytes(
            bytes[base + MISSING_OFFSET..base + MISSING_OFFSET + 8]
                .try_into()
                .ok()?,
        );
        let sp_min = f64::from_le_bytes(
            bytes[base + SP_MIN_OFFSET..base + SP_MIN_OFFSET + 8]
                .try_into()
                .ok()?,
        );
        let sp_max = f64::from_le_bytes(
            bytes[base + SP_MAX_OFFSET..base + SP_MAX_OFFSET + 8]
                .try_into()
                .ok()?,
        );
        let start_byte = match u32::from_le_bytes(
            bytes[base + START_OFFSET..base + START_OFFSET + 4]
                .try_into()
                .ok()?,
        ) {
            0 => None,
            v => Some(v as usize),
        };
        let nbytes = match u32::from_le_bytes(
            bytes[base + NBYTES_OFFSET..base + NBYTES_OFFSET + 4]
                .try_into()
                .ok()?,
        ) {
            0 => None,
            v => Some(v as usize),
        };
        let flags = u32::from_le_bytes(
            bytes[base + FLAGS_OFFSET..base + FLAGS_OFFSET + 4]
                .try_into()
                .ok()?,
        );
        columns.push(Pds4Column {
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
    let data_base = 16 + cols * COLUMN_META_BYTES;
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
        rows.push(Pds4Row { values });
    }
    Some(Pds4Table {
        columns,
        rows,
        delimited,
        delimiter,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CDR_LABEL: &str = include_str!("pds4_fixtures/cdr_f_20051029_20051118.xml");
    const CDR_FIRST2: &[u8] = include_bytes!("pds4_fixtures/cdr_f_20051029_20051118_first2.tab");

    #[test]
    fn parse_label_reads_the_real_cdr_label() {
        let meta = parse_label(CDR_LABEL).expect("label parses");
        assert_eq!(meta.table_class, "Table_Character");
        assert!(!meta.delimited);
        assert_eq!(
            meta.record_delimiter.as_deref(),
            Some("Carriage-Return Line-Feed")
        );
        assert_eq!(meta.record_length, Some(291));
        assert_eq!(meta.fields, Some(31));
        assert_eq!(meta.groups, Some(0));
        assert_eq!(meta.file_records, Some(154708));
        assert_eq!(meta.rows, Some(154708));
        assert_eq!(meta.file_size, Some(45020028));
        assert_eq!(
            meta.file_name.as_deref(),
            Some("cdr_f_20051029_20051118.tab")
        );
        assert_eq!(meta.mission_name.as_deref(), Some("HAYABUSA"));
        assert_eq!(
            meta.instrument_name.as_deref(),
            Some("LIGHT DETECTION AND RANGING INSTRUMENT")
        );
        assert_eq!(meta.host_name.as_deref(), Some("HAYABUSA"));
        assert_eq!(meta.target_name.as_deref(), Some("(25143) Itokawa"));
        assert_eq!(meta.start_time.as_deref(), Some("2005-10-29T07:54:00.961Z"));
        assert_eq!(meta.stop_time.as_deref(), Some("2005-11-18T21:56:22.216Z"));
        assert_eq!(meta.columns.len(), 31);
        assert_eq!(meta.columns[0].name, "MET");
        assert_eq!(meta.columns[0].data_type.as_deref(), Some("ASCII_Integer"));
        assert_eq!(meta.columns[0].start_byte, Some(1));
        assert_eq!(meta.columns[0].bytes, Some(10));
        assert_eq!(meta.columns[1].name, "UTC");
        assert_eq!(
            meta.columns[1].data_type.as_deref(),
            Some("ASCII_Date_Time_YMD")
        );
        assert_eq!(meta.columns[1].start_byte, Some(12));
        assert_eq!(meta.columns[1].bytes, Some(23));
        assert_eq!(meta.columns[2].name, "RANGE");
        assert_eq!(meta.columns[2].data_type.as_deref(), Some("ASCII_Real"));
        assert_eq!(meta.columns[2].start_byte, Some(36));
        assert_eq!(meta.columns[2].bytes, Some(8));
        assert_eq!(meta.columns[2].unit.as_deref(), Some("Kilometer"));
        assert_eq!(meta.columns[30].name, "N");
        assert_eq!(meta.columns[30].data_type.as_deref(), Some("ASCII_Integer"));
        assert_eq!(meta.columns[30].start_byte, Some(289));
        assert_eq!(meta.columns[30].bytes, Some(1));
        assert!(meta.columns.iter().all(|c| c.missing_constant.is_none()));
    }

    #[test]
    fn decode_reads_the_real_cdr_rows() {
        let meta = parse_label(CDR_LABEL).expect("label parses");
        let (rows, skipped, trailing) = decode_rows(CDR_FIRST2, &meta).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0].len(), 31);
        assert_eq!(rows[0][0], Some(2500433178.0));
        assert_eq!(rows[1][0], Some(2500433210.0));
        assert_eq!(rows[0][1], Some(1130572440.961));
        assert_eq!(rows[1][1], Some(1130572441.961));
        assert_eq!(rows[0][2], Some(5.20588));
        assert_eq!(rows[1][2], Some(5.19688));
        assert_eq!(rows[0][3], Some(3.05147));
        assert_eq!(rows[0][17], Some(-145.265));
        assert_eq!(rows[0][18], Some(39.624));
        assert_eq!(rows[0][30], Some(9.0));
    }

    #[test]
    fn record_stride_follows_the_label_counts() {
        let meta = parse_label(CDR_LABEL).expect("label parses");
        assert_eq!(record_stride(&meta, 2 * 291), Some(291));
        assert_eq!(record_stride(&meta, 154708 * 291), Some(291));
        assert_eq!(record_stride(&meta, 154709 * 291), Some(291));
    }

    #[test]
    fn pack_and_roundtrip_hold_with_the_real_rows() {
        let meta = parse_label(CDR_LABEL).expect("label parses");
        let (rows, _, _) = decode_rows(CDR_FIRST2, &meta).expect("rows");
        let columns = meta.columns.clone();
        let table = Pds4Table {
            columns,
            rows: rows.into_iter().map(|values| Pds4Row { values }).collect(),
            delimited: false,
            delimiter: None,
        };
        let bin = pack(&table);
        assert_eq!(&bin[0..4], b"P4FW");
        let parsed = parse_table(&bin).expect("packed table parses");
        assert_eq!(parsed, table);
    }

    #[test]
    fn parse_table_rejects_foreign_bytes() {
        assert!(parse_table(b"").is_none());
        assert!(parse_table(b"XXXX").is_none());
        assert!(parse_table(b"P3FW000000000000").is_none());
    }

    #[test]
    fn delimited_label_extraction_decode_and_roundtrip() {
        let label = r#"<?xml version="1.0" encoding="UTF-8"?>
<Product_Observational xmlns="http://pds.nasa.gov/pds4/pds/v1">
  <Observation_Area>
    <Time_Coordinates>
      <start_date_time>2020-01-01T00:00:00Z</start_date_time>
    </Time_Coordinates>
  </Observation_Area>
  <File_Area_Observational>
    <File>
      <file_name>sample.csv</file_name>
      <records>2</records>
    </File>
    <Table_Delimited>
      <offset unit="byte">0</offset>
      <records>2</records>
      <record_delimiter>Carriage-Return Line-Feed</record_delimiter>
      <field_delimiter>Comma</field_delimiter>
      <Record_Delimited>
        <fields>3</fields>
        <groups>0</groups>
        <Field_Delimited>
          <name>TIME</name>
          <field_number>1</field_number>
          <data_type>ASCII_Date_Time_YMD</data_type>
        </Field_Delimited>
        <Field_Delimited>
          <name>RANGE</name>
          <field_number>2</field_number>
          <data_type>ASCII_Real</data_type>
          <unit>Kilometer</unit>
          <Special_Constants>
            <missing_constant>-999.0</missing_constant>
          </Special_Constants>
        </Field_Delimited>
        <Field_Delimited>
          <name>QUALITY</name>
          <field_number>3</field_number>
          <data_type>ASCII_Integer</data_type>
          <maximum_field_length unit="byte">2</maximum_field_length>
        </Field_Delimited>
      </Record_Delimited>
    </Table_Delimited>
  </File_Area_Observational>
</Product_Observational>
"#;
        let meta = parse_label(label).expect("label parses");
        assert_eq!(meta.table_class, "Table_Delimited");
        assert!(meta.delimited);
        assert_eq!(meta.delimiter, Some(b','));
        assert_eq!(meta.columns.len(), 3);
        assert_eq!(meta.columns[0].name, "TIME");
        assert_eq!(meta.columns[0].start_byte, Some(1));
        assert_eq!(meta.columns[0].bytes, None);
        assert_eq!(meta.columns[1].start_byte, Some(2));
        assert_eq!(meta.columns[1].missing_constant, Some(-999.0));
        assert_eq!(meta.columns[1].bytes, None);
        assert_eq!(meta.columns[2].start_byte, Some(3));
        assert_eq!(meta.columns[2].bytes, Some(2));
        let data = b"2020-01-01T00:00:00,4.25,2\r\n2020-01-01T00:00:01,-999.0,3\r\n";
        let (rows, skipped, trailing) = decode_rows(data, &meta).expect("rows");
        assert_eq!(rows.len(), 2);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0][0], unix_of_iso("2020-01-01T00:00:00"));
        assert_eq!(rows[0][1], Some(4.25));
        assert_eq!(rows[0][2], Some(2.0));
        assert_eq!(rows[1][0], unix_of_iso("2020-01-01T00:00:01"));
        assert_eq!(rows[1][1], None);
        assert_eq!(rows[1][2], Some(3.0));
        let columns = meta.columns.clone();
        let table = Pds4Table {
            columns,
            rows: rows.into_iter().map(|values| Pds4Row { values }).collect(),
            delimited: true,
            delimiter: Some(b','),
        };
        let bin = pack(&table);
        let parsed = parse_table(&bin).expect("packed table parses");
        assert_eq!(parsed, table);
    }

    #[test]
    fn parse_cell_gates_nan_inf_blank_and_string() {
        assert_eq!(
            parse_cell(b"2005-10-29T07:54:00.961", "ASCII_Date_Time_YMD", None),
            Some(1130572440.961)
        );
        assert_eq!(parse_cell(b"  42  ", "ASCII_Integer", None), Some(42.0));
        assert_eq!(parse_cell(b"-999.0", "ASCII_Real", Some(-999.0)), None);
        assert_eq!(parse_cell(b"", "ASCII_Real", None), None);
        assert_eq!(parse_cell(b"   ", "ASCII_Real", None), None);
        assert_eq!(parse_cell(b"nan", "ASCII_Real", None), None);
        assert_eq!(parse_cell(b"inf", "ASCII_Real", None), None);
        assert_eq!(parse_cell(b"1e308", "ASCII_Real", None), Some(1e308));
        assert_eq!(parse_cell(b"2500433178", "ASCII_String", None), None);
        assert_eq!(
            parse_cell(b"2005-10-29T07:54:00.961Z", "ASCII_Date_Time_UTC", None),
            Some(1130572440.961)
        );
        assert_eq!(parse_cell(b"FF", "ASCII_Numeric_Base16", None), Some(255.0));
        assert_eq!(parse_cell(b"ff", "ASCII_Numeric_Base16", None), Some(255.0));
        assert_eq!(parse_cell(b"10", "ASCII_Numeric_Base16", None), Some(16.0));
        assert_eq!(parse_cell(b"", "ASCII_Numeric_Base16", None), None);
        assert_eq!(parse_cell(b"zz", "ASCII_Numeric_Base16", None), None);
        assert!(is_numeric_type("ASCII_Numeric_Base16"));
        assert!(is_numeric_type("ascii_numeric_base16"));
    }

    const EXOMARS_ACS_LABEL: &str = include_str!("pds4_fixtures/exomars_acs_nir_ec.xml");
    const EXOMARS_ACS_FIRST1: &[u8] = include_bytes!("pds4_fixtures/exomars_acs_nir_ec_first1.tab");

    const BASE16_GROUP_LABEL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Product_Observational xmlns="http://pds.nasa.gov/pds4/pds/v1">
  <File_Area_Observational>
    <File>
      <file_name>s.tab</file_name>
      <records>1</records>
    </File>
    <Table_Character>
      <offset unit="byte">0</offset>
      <records>1</records>
      <record_delimiter>Carriage-Return Line-Feed</record_delimiter>
      <Record_Character>
        <fields>2</fields>
        <groups>1</groups>
        <record_length unit="byte">10</record_length>
        <Field_Character>
          <name>FRAME</name>
          <field_number>1</field_number>
          <field_location unit="byte">1</field_location>
          <data_type>ASCII_NonNegative_Integer</data_type>
          <field_length unit="byte">2</field_length>
        </Field_Character>
        <Group_Field_Character>
          <group_number>1</group_number>
          <repetitions>2</repetitions>
          <fields>2</fields>
          <groups>0</groups>
          <group_location unit="byte">3</group_location>
          <group_length unit="byte">8</group_length>
          <Field_Character>
            <name>SPEC</name>
            <field_location unit="byte">1</field_location>
            <data_type>ASCII_Numeric_Base16</data_type>
            <field_length unit="byte">2</field_length>
          </Field_Character>
          <Field_Character>
            <name>COUNT</name>
            <field_location unit="byte">3</field_location>
            <data_type>ASCII_Numeric_Base16</data_type>
            <field_length unit="byte">1</field_length>
          </Field_Character>
        </Group_Field_Character>
      </Record_Character>
    </Table_Character>
  </File_Area_Observational>
</Product_Observational>
"#;

    #[test]
    fn base16_group_splits_subfields_by_offset_and_length() {
        let meta = parse_label(BASE16_GROUP_LABEL).expect("label parses");
        assert_eq!(meta.table_class, "Table_Character");
        assert_eq!(meta.record_length, Some(10));
        assert_eq!(meta.groups, Some(1));
        assert_eq!(meta.columns.len(), 5);
        assert_eq!(meta.columns[0].name, "FRAME");
        assert_eq!(meta.columns[0].start_byte, Some(1));
        assert_eq!(meta.columns[0].bytes, Some(2));
        assert_eq!(meta.columns[1].name, "SPEC_0000");
        assert_eq!(meta.columns[1].start_byte, Some(3));
        assert_eq!(meta.columns[1].bytes, Some(2));
        assert_eq!(meta.columns[2].name, "COUNT_0000");
        assert_eq!(meta.columns[2].start_byte, Some(5));
        assert_eq!(meta.columns[2].bytes, Some(1));
        assert_eq!(meta.columns[3].name, "SPEC_0001");
        assert_eq!(meta.columns[3].start_byte, Some(7));
        assert_eq!(meta.columns[4].name, "COUNT_0001");
        assert_eq!(meta.columns[4].start_byte, Some(9));
        let (rows, skipped, trailing) = decode_rows(b"12A100B210", &meta).expect("rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(
            rows[0],
            vec![Some(12.0), Some(161.0), Some(0.0), Some(178.0), Some(1.0)]
        );
    }

    #[test]
    fn exomars_acs_base16_group_decodes_the_real_first_record() {
        let dat_file = "acs_raw_sc_nir_20181027T221352-20181027T223353-4140-1-1-EC__4_0.tab";
        let meta = parse_label_for_file(EXOMARS_ACS_LABEL, Some(dat_file)).expect("label parses");
        assert_eq!(meta.table_class, "Table_Character");
        assert_eq!(meta.record_length, Some(2719));
        assert_eq!(meta.rows, Some(200));
        assert_eq!(meta.file_records, Some(200));
        assert_eq!(meta.file_size, Some(543800));
        assert_eq!(meta.columns.len(), 1295);
        assert_eq!(meta.columns[0].name, "INSTRUMENT");
        assert_eq!(meta.columns[12].name, "SIZE");
        assert_eq!(meta.columns[13].name, "ROW_DATA_0000");
        assert_eq!(
            meta.columns[13].data_type.as_deref(),
            Some("ASCII_Numeric_Base16")
        );
        assert_eq!(meta.columns[13].start_byte, Some(136));
        assert_eq!(meta.columns[13].bytes, Some(2));
        assert_eq!(meta.columns[13].unit, None);
        assert_eq!(meta.columns[1292].name, "ROW_DATA_1279");
        assert_eq!(meta.columns[1292].start_byte, Some(2694));
        assert_eq!(meta.columns[1293].name, "STATUS");
        assert_eq!(meta.columns[1294].name, "CRC");
        assert_eq!(record_stride(&meta, 543800), Some(2719));
        let (rows, skipped, trailing) = decode_rows(EXOMARS_ACS_FIRST1, &meta).expect("rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(skipped, 0);
        assert_eq!(trailing, 0);
        assert_eq!(rows[0].len(), 1295);
        assert_eq!(rows[0][0], Some(0.0));
        assert_eq!(rows[0][1], Some(12582354.0));
        assert_eq!(rows[0][11], Some(164.0));
        assert_eq!(rows[0][12], Some(1280.0));
        assert_eq!(rows[0][13], Some(215.0));
        assert_eq!(rows[0][14], Some(45.0));
        assert_eq!(rows[0][15], Some(216.0));
    }
}
