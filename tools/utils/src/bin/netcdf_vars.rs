use omegaflow::hdf5::{
    Endian, Hdf5Attribute, Hdf5Datatype, Hdf5File, Hdf5Note, Hdf5Object,
};
use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::Command;

const BASE_BYTES: u64 = 64 * 1024;
const FETCH_WINDOW: u64 = 32 * 1024;
const MAX_RANGE_BYTES: u64 = 8 * 1024 * 1024;
const HEADER: &str =
    "variable\tgroup\tdims\tdtype\tshape\tunits\tscale_factor\tadd_offset\tfill\tvalid_range";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Format {
    Tsv,
    Jsonl,
}

#[derive(Clone)]
enum Num {
    Int(i128),
    F32(f32),
    F64(f64),
}

impl Num {
    fn is_finite(&self) -> bool {
        match self {
            Num::Int(_) => true,
            Num::F32(v) => v.is_finite(),
            Num::F64(v) => v.is_finite(),
        }
    }

    fn text(&self) -> String {
        match self {
            Num::Int(v) => v.to_string(),
            Num::F32(v) => float_text_f32(*v),
            Num::F64(v) => float_text_f64(*v),
        }
    }

    fn json(&self) -> String {
        self.text()
    }
}

fn float_text_f32(v: f32) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    if v.fract() == 0.0 && v.abs() < 9.0e15 {
        return format!("{}", v as i64);
    }
    if v.abs() >= 1.0e16 || v.abs() < 1.0e-6 {
        return format!("{v:e}");
    }
    v.to_string()
}

fn float_text_f64(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    if v.fract() == 0.0 && v.abs() < 9.0e15 {
        return format!("{}", v as i64);
    }
    if v.abs() >= 1.0e16 || v.abs() < 1.0e-6 {
        return format!("{v:e}");
    }
    v.to_string()
}

#[derive(Clone)]
struct Row {
    path: String,
    variable: String,
    group: String,
    dims: String,
    dtype: String,
    shape: Vec<u64>,
    units: Option<String>,
    scale: Option<Num>,
    offset: Option<Num>,
    fill: Option<Num>,
    valid_range: Option<(Num, Num)>,
}

fn usage() -> String {
    "usage: netcdf_vars <url|file> [--var <name>] [--format tsv|jsonl] [--out <file>]".to_string()
}

fn skip_space(bytes: &[u8]) -> usize {
    let mut n = 0;
    while n < bytes.len() && (bytes[n] == 0 || bytes[n] == b' ') {
        n += 1;
    }
    n
}

fn attr_text(attr: &Hdf5Attribute) -> Option<String> {
    let text_like = attr.datatype.class == 3
        || (attr.datatype.class == 9 && attr.datatype.vlen_is_string);
    if !text_like || attr.data.is_empty() {
        return None;
    }
    let mut bytes = attr.data.clone();
    while bytes.last().is_some_and(|b| *b == 0 || *b == b' ') {
        bytes.pop();
    }
    let skip = skip_space(&bytes);
    let text = String::from_utf8(bytes[skip..].to_vec()).ok()?;
    let text = text.trim().to_string();
    if text.is_empty() { None } else { Some(text) }
}

fn decode_number(data: &[u8], off: usize, dt: &Hdf5Datatype, unsigned: bool) -> Option<Num> {
    let size = dt.size;
    let bytes = data.get(off..off + size)?;
    let be = dt.endian == Endian::Be;
    match dt.class {
        0 => {
            if dt.signed && !unsigned {
                match size {
                    1 => Some(Num::Int(bytes[0] as i8 as i128)),
                    2 => {
                        let a: [u8; 2] = bytes.try_into().ok()?;
                        Some(Num::Int(if be {
                            i16::from_be_bytes(a) as i128
                        } else {
                            i16::from_le_bytes(a) as i128
                        }))
                    }
                    4 => {
                        let a: [u8; 4] = bytes.try_into().ok()?;
                        Some(Num::Int(if be {
                            i32::from_be_bytes(a) as i128
                        } else {
                            i32::from_le_bytes(a) as i128
                        }))
                    }
                    8 => {
                        let a: [u8; 8] = bytes.try_into().ok()?;
                        Some(Num::Int(if be {
                            i64::from_be_bytes(a) as i128
                        } else {
                            i64::from_le_bytes(a) as i128
                        }))
                    }
                    _ => None,
                }
            } else {
                match size {
                    1 => Some(Num::Int(bytes[0] as i128)),
                    2 => {
                        let a: [u8; 2] = bytes.try_into().ok()?;
                        Some(Num::Int(if be {
                            u16::from_be_bytes(a) as i128
                        } else {
                            u16::from_le_bytes(a) as i128
                        }))
                    }
                    4 => {
                        let a: [u8; 4] = bytes.try_into().ok()?;
                        Some(Num::Int(if be {
                            u32::from_be_bytes(a) as i128
                        } else {
                            u32::from_le_bytes(a) as i128
                        }))
                    }
                    8 => {
                        let a: [u8; 8] = bytes.try_into().ok()?;
                        Some(Num::Int(if be {
                            u64::from_be_bytes(a) as i128
                        } else {
                            u64::from_le_bytes(a) as i128
                        }))
                    }
                    _ => None,
                }
            }
        }
        1 => match size {
            4 => {
                let a: [u8; 4] = bytes.try_into().ok()?;
                Some(Num::F32(if be {
                    f32::from_be_bytes(a)
                } else {
                    f32::from_le_bytes(a)
                }))
            }
            8 => {
                let a: [u8; 8] = bytes.try_into().ok()?;
                Some(Num::F64(if be {
                    f64::from_be_bytes(a)
                } else {
                    f64::from_le_bytes(a)
                }))
            }
            _ => None,
        },
        _ => None,
    }
}

fn attr_numbers(attr: &Hdf5Attribute, unsigned: bool) -> Vec<Num> {
    let size = attr.datatype.size;
    if size == 0 || attr.data.is_empty() {
        return Vec::new();
    }
    let count = attr
        .dataspace
        .dims
        .iter()
        .fold(1usize, |acc, dim| acc.saturating_mul(*dim as usize));
    let available = attr.data.len() / size;
    let n = count.min(available);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(value) = decode_number(&attr.data, i * size, &attr.datatype, unsigned) {
            if value.is_finite() {
                out.push(value);
            }
        }
    }
    out
}

fn attr_number(attr: &Hdf5Attribute, unsigned: bool) -> Option<Num> {
    attr_numbers(attr, unsigned).into_iter().next()
}

fn find_attr<'a>(obj: &'a Hdf5Object, name: &str) -> Option<&'a Hdf5Attribute> {
    obj.attrs.iter().find(|a| a.name == name)
}

fn object_unsigned(obj: &Hdf5Object) -> bool {
    find_attr(obj, "_Unsigned")
        .and_then(attr_text)
        .is_some_and(|text| text.eq_ignore_ascii_case("true"))
}

fn dtype_text(dt: &Hdf5Datatype) -> String {
    match dt.class {
        0 => format!(
            "{}{}",
            if dt.signed { "int" } else { "uint" },
            dt.size * 8
        ),
        1 => format!("float{}", dt.size * 8),
        3 => "string".to_string(),
        4 => format!("bitfield{}", dt.size * 8),
        6 => format!("compound{}", dt.size * 8),
        7 => "reference".to_string(),
        8 => "enum".to_string(),
        9 if dt.vlen_is_string => "string".to_string(),
        9 => "vlen".to_string(),
        10 => "array".to_string(),
        class => format!("class{}/{}", class, dt.size * 8),
    }
}

fn shape_text(shape: &[u64]) -> String {
    if shape.is_empty() {
        return "scalar".to_string();
    }
    shape
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join("x")
}

fn dimension_scale_name(obj: &Hdf5Object) -> Option<String> {
    let class = find_attr(obj, "CLASS").and_then(attr_text)?;
    if class != "DIMENSION_SCALE" {
        return None;
    }
    find_attr(obj, "NAME").and_then(attr_text)
}

fn row_of(path: &str, obj: &Hdf5Object) -> Option<Row> {
    let ds = obj.dataspace.as_ref()?;
    let dt = obj.datatype.as_ref()?;
    let (group, variable) = match path.rfind('/') {
        Some(0) => ("/".to_string(), path[1..].to_string()),
        Some(i) => (path[..i].to_string(), path[i + 1..].to_string()),
        None => ("/".to_string(), path.to_string()),
    };
    let unsigned = object_unsigned(obj);
    let units = find_attr(obj, "units").and_then(attr_text);
    let scale = find_attr(obj, "scale_factor").and_then(|a| attr_number(a, unsigned));
    let offset = find_attr(obj, "add_offset").and_then(|a| attr_number(a, unsigned));
    let fill = find_attr(obj, "_FillValue").and_then(|a| attr_number(a, unsigned));
    let valid_range = find_attr(obj, "valid_range").and_then(|a| {
        let values = attr_numbers(a, unsigned);
        match values.as_slice() {
            [lo, hi] => Some((lo.clone(), hi.clone())),
            _ => None,
        }
    });
    Some(Row {
        path: path.to_string(),
        variable,
        group,
        dims: match dimension_scale_name(obj) {
            Some(value) => value,
            None => String::new(),
        },
        dtype: dtype_text(dt),
        shape: ds.dims.clone(),
        units,
        scale,
        offset,
        fill,
        valid_range,
    })
}

fn collect_rows(file: &Hdf5File) -> Result<Vec<Row>, Hdf5Note> {
    let root = file.root()?;
    let mut seen = HashSet::new();
    seen.insert(root.addr);
    let mut stack = Vec::new();
    for link in &root.links {
        if link.addr == u64::MAX || link.soft.is_some() {
            continue;
        }
        stack.push(format!("/{}", link.name));
    }
    let mut rows = Vec::new();
    while let Some(path) = stack.pop() {
        let obj = match file.resolve(&path) {
            Ok(obj) => obj,
            Err(_) => continue,
        };
        if !seen.insert(obj.addr) {
            continue;
        }
        if let Some(row) = row_of(&path, obj) {
            rows.push(row);
        }
        for link in &obj.links {
            if link.addr == u64::MAX || link.soft.is_some() {
                continue;
            }
            stack.push(format!("{}/{}", path, link.name));
        }
    }
    rows.sort_by(|a, b| (a.group.as_str(), a.variable.as_str()).cmp(&(b.group.as_str(), b.variable.as_str())));
    Ok(rows)
}

fn tsv_clean(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            '\t' | '\n' | '\r' => ' ',
            other => other,
        })
        .collect()
}

fn opt_text(value: &Option<String>) -> String {
    match value {
        Some(value) => value.clone(),
        None => String::new(),
    }
}

fn opt_num(value: &Option<Num>) -> String {
    match value {
        Some(value) => value.text(),
        None => String::new(),
    }
}

fn opt_range(value: &Option<(Num, Num)>) -> String {
    match value {
        Some((lo, hi)) => format!("{},{}", lo.text(), hi.text()),
        None => String::new(),
    }
}

fn render_tsv(rows: &[Row]) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');
    for row in rows {
        let fields = [
            tsv_clean(&row.variable),
            tsv_clean(&row.group),
            tsv_clean(&row.dims),
            tsv_clean(&row.dtype),
            tsv_clean(&shape_text(&row.shape)),
            tsv_clean(&opt_text(&row.units)),
            tsv_clean(&opt_num(&row.scale)),
            tsv_clean(&opt_num(&row.offset)),
            tsv_clean(&opt_num(&row.fill)),
            tsv_clean(&opt_range(&row.valid_range)),
        ];
        out.push_str(&fields.join("\t"));
        out.push('\n');
    }
    out
}

fn json_str(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_opt_str(value: &Option<String>) -> String {
    match value {
        Some(text) => json_str(text),
        None => "null".to_string(),
    }
}

fn json_opt_num(value: &Option<Num>) -> String {
    match value {
        Some(number) => number.json(),
        None => "null".to_string(),
    }
}

fn json_range(value: &Option<(Num, Num)>) -> String {
    match value {
        Some((lo, hi)) => format!("[{},{}]", lo.json(), hi.json()),
        None => "null".to_string(),
    }
}

fn render_jsonl(rows: &[Row]) -> String {
    let mut out = String::new();
    for row in rows {
        let shape = row
            .shape
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!(
            "{{\"variable\":{},\"group\":{},\"dims\":{},\"dtype\":{},\"shape\":[{}],\"units\":{},\"scale_factor\":{},\"add_offset\":{},\"fill\":{},\"valid_range\":{}}}\n",
            json_str(&row.variable),
            json_str(&row.group),
            json_str(&row.dims),
            json_str(&row.dtype),
            shape,
            json_opt_str(&row.units),
            json_opt_num(&row.scale),
            json_opt_num(&row.offset),
            json_opt_num(&row.fill),
            json_range(&row.valid_range)
        ));
    }
    out
}

fn note_text(note: &Hdf5Note) -> String {
    match note {
        Hdf5Note::Magic { bytes } => format!(
            "magic is {:02X} {:02X} {:02X} {:02X} — not HDF5",
            bytes[0], bytes[1], bytes[2], bytes[3]
        ),
        Hdf5Note::SuperblockVersion { v } => format!("superblock version {} unread", v),
        Hdf5Note::OffsetSize { n } => format!("offset size {} unread (4 or 8 only)", n),
        Hdf5Note::EndAtByte { off } => format!("file ends at byte {}", off),
        Hdf5Note::AbsentAtByte { off } => {
            format!("no bytes at address {} — the source does not carry it", off)
        }
        Hdf5Note::Signatur { off, found } => format!(
            "signature {:02X}{:02X}{:02X}{:02X} at byte {} — not the expected block",
            found[0], found[1], found[2], found[3], off
        ),
        Hdf5Note::Address { off } => format!("address at byte {} points into the void", off),
        Hdf5Note::ObjectHeaderVersion { v } => format!("object header version {} unread", v),
        Hdf5Note::Datatype { class, off } => {
            format!("datatype class {} at byte {} unread", class, off)
        }
        Hdf5Note::Dataspace { off } => format!("dataspace at byte {} unread", off),
        Hdf5Note::Layout { class, off } => format!("layout class {} at byte {} unread", class, off),
        Hdf5Note::Btree { typ, off } => format!("B-tree type {} at byte {} unread", typ, off),
        Hdf5Note::BtreeNode { found, off } => format!(
            "B-tree node {:02X}{:02X}{:02X}{:02X} at byte {} unread",
            found[0], found[1], found[2], found[3], off
        ),
        Hdf5Note::Heap { found, off } => format!(
            "heap {:02X}{:02X}{:02X}{:02X} at byte {} unread",
            found[0], found[1], found[2], found[3], off
        ),
        Hdf5Note::Filter { id, off } => format!("filter {} at byte {} unread", id, off),
        Hdf5Note::Checksum { off } => {
            format!("metadata checksum at byte {} carries a mismatch", off)
        }
        Hdf5Note::HugeHeapObject => "huge fractal-heap object unread".to_string(),
        Hdf5Note::SharedMessage => "shared object header message unread".to_string(),
        Hdf5Note::AbsentObject { name } => format!("object absent: {}", name),
        Hdf5Note::Chunk { off } => format!("chunk at byte {} lies outside the shape", off),
        Hdf5Note::VlenNotRead => "variable-length dataset data stays unread".to_string(),
        Hdf5Note::VirtualDataset => "virtual dataset unread".to_string(),
        Hdf5Note::ReadLength { off, len } => format!(
            "read of {} bytes at byte {} exceeds the per-read cap — the chunk stays unfetched",
            len, off
        ),
        Hdf5Note::FetchBudget { off, reads } => format!(
            "fetch budget exhausted after {} reads at byte {} — the read stops here",
            reads, off
        ),
        Hdf5Note::TraversalBudget { off, bytes } => format!(
            "traversal budget exhausted after {} bytes at byte {} — the walk stops here",
            bytes, off
        ),
    }
}

fn is_url(target: &str) -> bool {
    target.starts_with("http://") || target.starts_with("https://")
}

fn header_value(headers: &str, key: &str) -> Option<String> {
    headers.lines().rev().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name.trim().eq_ignore_ascii_case(key) {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

fn percent(bytes: u64, total: u64) -> String {
    if total == 0 {
        return "0.0%".to_string();
    }
    format!("{:.1}%", bytes as f64 * 100.0 / total as f64)
}

struct RemoteReader {
    url: String,
    size: u64,
    requests: u64,
    bytes: u64,
    last_error: Option<String>,
}

impl RemoteReader {
    fn open(url: &str) -> Result<RemoteReader, String> {
        let size = probe(url)?;
        Ok(RemoteReader {
            url: url.to_string(),
            size,
            requests: 0,
            bytes: 0,
            last_error: None,
        })
    }

    fn fetch(&mut self, off: u64, len: u64) -> Option<Vec<u8>> {
        if off >= self.size || len == 0 {
            return None;
        }
        let want = len.max(FETCH_WINDOW).min(self.size - off).min(MAX_RANGE_BYTES);
        let end = off + want - 1;
        let out = Command::new("curl")
            .args([
                "-sSf",
                "--max-time",
                "120",
                "--max-filesize",
                &want.to_string(),
                "-r",
                &format!("{off}-{end}"),
                &self.url,
            ])
            .output();
        let out = match out {
            Ok(out) => out,
            Err(err) => {
                self.last_error = Some(err.to_string());
                return None;
            }
        };
        if !out.status.success() {
            let text = String::from_utf8_lossy(&out.stderr).trim().to_string();
            self.last_error = Some(if text.is_empty() {
                format!("curl exit {}", out.status)
            } else {
                text
            });
            return None;
        }
        if out.stdout.len() as u64 != want {
            self.last_error = Some(format!(
                "range request returned {} bytes, asked for {}",
                out.stdout.len(),
                want
            ));
            return None;
        }
        self.requests += 1;
        self.bytes += want;
        Some(out.stdout)
    }

    fn diagnostic(&self) -> String {
        format!(
            "range: {} requests · {} B of {} B ({})",
            self.requests,
            self.bytes,
            self.size,
            percent(self.bytes, self.size)
        )
    }
}

fn probe(url: &str) -> Result<u64, String> {
    if let Ok(out) = Command::new("curl")
        .args(["-sSI", "--max-time", "30", url])
        .output()
    {
        if out.status.success() {
            let headers = String::from_utf8_lossy(&out.stdout);
            if let Some(size) = header_value(&headers, "content-length")
                .and_then(|v| v.parse::<u64>().ok())
            {
                if size > 0 {
                    return Ok(size);
                }
            }
        }
    }
    let out = Command::new("curl")
        .args([
            "-sS",
            "--max-time",
            "30",
            "-D",
            "-",
            "-o",
            "/dev/null",
            "-r",
            "0-0",
            url,
        ])
        .output()
        .map_err(|err| format!("probe failed: {err}"))?;
    if !out.status.success() {
        return Err(format!(
            "probe failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let headers = String::from_utf8_lossy(&out.stdout);
    let range = header_value(&headers, "content-range")
        .ok_or_else(|| "probe carries no content-range".to_string())?;
    let total = range
        .rsplit_once('/')
        .and_then(|(_, total)| total.trim().parse::<u64>().ok())
        .ok_or_else(|| format!("content-range unread: {range}"))?;
    if total == 0 {
        return Err("source reports zero bytes".to_string());
    }
    Ok(total)
}

struct LocalReader {
    file: File,
    size: u64,
    requests: u64,
    bytes: u64,
}

impl LocalReader {
    fn open(path: &str) -> Result<LocalReader, String> {
        let file = File::open(path).map_err(|err| format!("file does not open: {path}: {err}"))?;
        let size = file
            .metadata()
            .map_err(|err| format!("file metadata unread: {path}: {err}"))?
            .len();
        if size == 0 {
            return Err(format!("file is empty: {path}"));
        }
        Ok(LocalReader {
            file,
            size,
            requests: 0,
            bytes: 0,
        })
    }

    fn fetch(&mut self, off: u64, len: u64) -> Option<Vec<u8>> {
        if off >= self.size || len == 0 {
            return None;
        }
        let want = len.max(FETCH_WINDOW).min(self.size - off).min(MAX_RANGE_BYTES);
        let mut buf = vec![0u8; want as usize];
        self.file.seek(SeekFrom::Start(off)).ok()?;
        self.file.read_exact(&mut buf).ok()?;
        self.requests += 1;
        self.bytes += want;
        Some(buf)
    }

    fn diagnostic(&self) -> String {
        format!(
            "file: {} reads · {} B of {} B ({})",
            self.requests,
            self.bytes,
            self.size,
            percent(self.bytes, self.size)
        )
    }
}

fn read_rows(
    target: &str,
) -> Result<(Vec<Row>, String), String> {
    if is_url(target) {
        let mut reader = RemoteReader::open(target)?;
        let base_len = BASE_BYTES.min(reader.size);
        let base = reader.fetch(0, base_len).ok_or_else(|| match &reader.last_error {
            Some(error) => error.clone(),
            None => "first range request failed".to_string(),
        })?;
        let rows = {
            let file = Hdf5File::parse_fetch(&base, |off, len| reader.fetch(off, len))
                .map_err(|note| range_failure(&reader, &note))?;
            collect_rows(&file).map_err(|note| note_text(&note))?
        };
        let diagnostic = reader.diagnostic();
        Ok((rows, diagnostic))
    } else {
        let mut reader = LocalReader::open(target)?;
        let base_len = BASE_BYTES.min(reader.size);
        let base = reader
            .fetch(0, base_len)
            .ok_or_else(|| format!("file does not read: {target}"))?;
        let rows = {
            let file = Hdf5File::parse_fetch(&base, |off, len| reader.fetch(off, len))
                .map_err(|note| note_text(&note))?;
            collect_rows(&file).map_err(|note| note_text(&note))?
        };
        let diagnostic = reader.diagnostic();
        Ok((rows, diagnostic))
    }
}

fn range_failure(reader: &RemoteReader, note: &Hdf5Note) -> String {
    match &reader.last_error {
        Some(err) => format!("{} — range request failed: {}", note_text(note), err),
        None => note_text(note),
    }
}

fn select<'a>(rows: &'a [Row], var: &str) -> Vec<&'a Row> {
    let wanted = if var.starts_with('/') {
        var.to_string()
    } else {
        format!("/{var}")
    };
    let by_path: Vec<&Row> = rows.iter().filter(|row| row.path == wanted).collect();
    if !by_path.is_empty() {
        return by_path;
    }
    rows.iter().filter(|row| row.variable == var).collect()
}

fn execute(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    let mut target: Option<String> = None;
    let mut var: Option<String> = None;
    let mut format = Format::Tsv;
    let mut out_path: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--var" => {
                let Some(value) = args.get(i + 1) else {
                    let _ = writeln!(err, "--var needs a name\n{}", usage());
                    return 2;
                };
                var = Some(value.clone());
                i += 2;
            }
            "--format" => {
                let Some(value) = args.get(i + 1) else {
                    let _ = writeln!(err, "--format needs tsv or jsonl\n{}", usage());
                    return 2;
                };
                format = match value.as_str() {
                    "tsv" => Format::Tsv,
                    "jsonl" => Format::Jsonl,
                    other => {
                        let _ = writeln!(err, "format unread: {other}\n{}", usage());
                        return 2;
                    }
                };
                i += 2;
            }
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    let _ = writeln!(err, "--out needs a file\n{}", usage());
                    return 2;
                };
                out_path = Some(value.clone());
                i += 2;
            }
            "--help" | "-h" => {
                let _ = writeln!(out, "{}", usage());
                return 0;
            }
            flag if flag.starts_with('-') => {
                let _ = writeln!(err, "flag unread: {flag}\n{}", usage());
                return 2;
            }
            value => {
                if target.is_some() {
                    let _ = writeln!(err, "one source only\n{}", usage());
                    return 2;
                }
                target = Some(value.to_string());
                i += 1;
            }
        }
    }
    let Some(target) = target else {
        let _ = writeln!(err, "{}", usage());
        return 2;
    };
    let (rows, diagnostic) = match read_rows(&target) {
        Ok(result) => result,
        Err(message) => {
            let _ = writeln!(err, "{message}");
            return 2;
        }
    };
    let selected: Vec<&Row> = match &var {
        Some(name) => {
            let hits = select(&rows, name);
            if hits.is_empty() {
                let _ = writeln!(err, "variable absent: {name}");
                return 3;
            }
            hits
        }
        None => {
            if rows.is_empty() {
                let _ = writeln!(err, "pending — no variable metadata");
                return 2;
            }
            rows.iter().collect()
        }
    };
    let owned: Vec<Row> = selected.iter().map(|row| (*row).clone()).collect();
    let rendered = match format {
        Format::Tsv => render_tsv(&owned),
        Format::Jsonl => render_jsonl(&owned),
    };
    if let Some(path) = out_path {
        if let Err(error) = std::fs::write(&path, rendered) {
            let _ = writeln!(err, "output file does not write: {path}: {error}");
            return 2;
        }
    } else if let Err(error) = out.write_all(rendered.as_bytes()) {
        let _ = writeln!(err, "output does not write: {error}");
        return 2;
    }
    let _ = writeln!(err, "{diagnostic}");
    0
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out = std::io::stdout();
    let mut err = std::io::stderr();
    let code = execute(&args, &mut out, &mut err);
    let _ = out.flush();
    let _ = err.flush();
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::json::{JsonVal, parse_json};
    use std::sync::OnceLock;

    fn message(typ: u8, data: &[u8]) -> Vec<u8> {
        assert!(data.len() <= u16::MAX as usize);
        let mut m = vec![typ, 0];
        m.extend_from_slice(&(data.len() as u16).to_le_bytes());
        m.push(0);
        m.extend_from_slice(&[0, 0, 0]);
        m.extend_from_slice(data);
        m
    }

    fn header(messages: &[Vec<u8>]) -> Vec<u8> {
        let body: usize = messages.iter().map(Vec::len).sum();
        assert!(body <= u16::MAX as usize);
        let mut h = vec![1, 0];
        h.extend_from_slice(&(messages.len() as u16).to_le_bytes());
        h.extend_from_slice(&1u32.to_le_bytes());
        h.extend_from_slice(&(body as u32).to_le_bytes());
        h.extend_from_slice(&0u32.to_le_bytes());
        for m in messages {
            h.extend_from_slice(m);
        }
        h
    }

    fn group_info() -> Vec<u8> {
        message(0x0a, &[0, 0])
    }

    fn link(name: &str, addr: u64) -> Vec<u8> {
        assert!(name.len() < 256);
        let mut data = vec![0, 0, name.len() as u8];
        data.extend_from_slice(name.as_bytes());
        data.extend_from_slice(&addr.to_le_bytes());
        message(0x06, &data)
    }

    fn dataspace(dims: &[u64]) -> Vec<u8> {
        let mut data = vec![2, dims.len() as u8, 0, 0];
        for dim in dims {
            data.extend_from_slice(&dim.to_le_bytes());
        }
        message(0x01, &data)
    }

    fn datatype_f32() -> Vec<u8> {
        let mut d = vec![0x11, 0, 0, 0, 4, 0, 0, 0, 0, 0, 32, 0];
        d.extend_from_slice(&[0u8; 8]);
        message(0x03, &d)
    }

    fn datatype_i16() -> Vec<u8> {
        let d = vec![0x10, 0x08, 0, 0, 2, 0, 0, 0, 0, 0, 16, 0];
        message(0x03, &d)
    }

    fn datatype_string(len: u32) -> Vec<u8> {
        let mut d = vec![0x13, 0, 0, 0];
        d.extend_from_slice(&len.to_le_bytes());
        d
    }

    fn attr_raw(dt: &[u8], ds: &[u8], name: &str, value: &[u8]) -> Vec<u8> {
        assert!(name.len() + 1 <= u16::MAX as usize);
        assert!(dt.len() <= u16::MAX as usize);
        assert!(ds.len() <= u16::MAX as usize);
        let mut raw = vec![3, 0];
        raw.extend_from_slice(&((name.len() + 1) as u16).to_le_bytes());
        raw.extend_from_slice(&(dt.len() as u16).to_le_bytes());
        raw.extend_from_slice(&(ds.len() as u16).to_le_bytes());
        raw.push(0);
        raw.extend_from_slice(name.as_bytes());
        raw.push(0);
        raw.extend_from_slice(dt);
        raw.extend_from_slice(ds);
        raw.extend_from_slice(value);
        message(0x0c, &raw)
    }

    fn attr_string(name: &str, value: &str) -> Vec<u8> {
        let dt = datatype_string(value.len() as u32);
        let ds = vec![2, 0, 0, 0];
        attr_raw(&dt, &ds, name, value.as_bytes())
    }

    fn scalar_dataspace() -> Vec<u8> {
        vec![2, 0, 0, 0]
    }

    fn attr_f64(name: &str, value: f64) -> Vec<u8> {
        let mut dt = vec![0x11, 0, 0, 0];
        dt.extend_from_slice(&8u32.to_le_bytes());
        dt.extend_from_slice(&[0, 0, 64, 0]);
        dt.extend_from_slice(&[0u8; 8]);
        attr_raw(&dt, &scalar_dataspace(), name, &value.to_le_bytes())
    }

    fn attr_f32(name: &str, value: f32) -> Vec<u8> {
        let mut dt = vec![0x11, 0, 0, 0];
        dt.extend_from_slice(&4u32.to_le_bytes());
        dt.extend_from_slice(&[0, 0, 32, 0]);
        dt.extend_from_slice(&[0u8; 8]);
        attr_raw(&dt, &scalar_dataspace(), name, &value.to_le_bytes())
    }

    fn attr_f32_pair(name: &str, values: [f32; 2]) -> Vec<u8> {
        let mut dt = vec![0x11, 0, 0, 0];
        dt.extend_from_slice(&4u32.to_le_bytes());
        dt.extend_from_slice(&[0, 0, 32, 0]);
        dt.extend_from_slice(&[0u8; 8]);
        let ds = vec![2, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];
        let mut value = Vec::new();
        for v in values {
            value.extend_from_slice(&v.to_le_bytes());
        }
        attr_raw(&dt, &ds, name, &value)
    }

    fn layout(addr: u64, size: u64) -> Vec<u8> {
        let mut data = vec![3, 1];
        data.extend_from_slice(&addr.to_le_bytes());
        data.extend_from_slice(&size.to_le_bytes());
        message(0x08, &data)
    }

    fn dataset_header(
        shape: &[u64],
        dt: &[u8],
        attrs: &[Vec<u8>],
        data_addr: u64,
        data_len: u64,
    ) -> Vec<u8> {
        let mut messages = vec![dataspace(shape), dt.to_vec(), layout(data_addr, data_len)];
        messages.extend(attrs.iter().cloned());
        header(&messages)
    }

    fn build_fixture() -> (Vec<u8>, u64, u64) {
        let sst_attrs = vec![
            attr_string("units", "kelvin"),
            attr_f64("scale_factor", 0.01),
            attr_f64("add_offset", 273.15),
            attr_f32("_FillValue", -32767.0),
            attr_f32_pair("valid_range", [-100.0, 100.0]),
        ];
        let ozone_attrs = vec![attr_string("units", "mol m-2")];
        let root_addr = 72u64;
        let root_proto = header(&[
            group_info(),
            link("sst", 0),
            link("no_attrs", 0),
            link("big", 0),
            link("PRODUCT", 0),
        ]);
        let sst_proto = dataset_header(&[2, 3], &datatype_f32(), &sst_attrs, 0, 24);
        let no_attrs_proto = dataset_header(&[4], &datatype_i16(), &[], 0, 8);
        let big_proto = dataset_header(&[65536], &datatype_f32(), &[], 0, 262144);
        let product_addr = root_addr
            + root_proto.len() as u64
            + sst_proto.len() as u64
            + no_attrs_proto.len() as u64
            + big_proto.len() as u64;
        let product_proto = header(&[group_info(), link("ozone", 0)]);
        let ozone_addr = product_addr + product_proto.len() as u64;
        let ozone_proto = dataset_header(&[5], &datatype_f32(), &ozone_attrs, 0, 20);
        let sst_addr = root_addr + root_proto.len() as u64;
        let no_attrs_addr = sst_addr + sst_proto.len() as u64;
        let big_addr = no_attrs_addr + no_attrs_proto.len() as u64;
        let data_start = ozone_addr + ozone_proto.len() as u64;
        let sst_data = data_start;
        let no_attrs_data = sst_data + 24;
        let ozone_data = no_attrs_data + 8;
        let big_data = 131072u64;
        assert!(big_data > ozone_data + 20);

        let root = header(&[
            group_info(),
            link("sst", sst_addr),
            link("no_attrs", no_attrs_addr),
            link("big", big_addr),
            link("PRODUCT", product_addr),
        ]);
        let sst = dataset_header(&[2, 3], &datatype_f32(), &sst_attrs, sst_data, 24);
        let no_attrs = dataset_header(&[4], &datatype_i16(), &[], no_attrs_data, 8);
        let big = dataset_header(&[65536], &datatype_f32(), &[], big_data, 262144);
        let product = header(&[group_info(), link("ozone", ozone_addr)]);
        let ozone = dataset_header(&[5], &datatype_f32(), &ozone_attrs, ozone_data, 20);
        assert_eq!(root.len(), root_proto.len());
        assert_eq!(sst.len(), sst_proto.len());
        assert_eq!(no_attrs.len(), no_attrs_proto.len());
        assert_eq!(big.len(), big_proto.len());
        assert_eq!(product.len(), product_proto.len());
        assert_eq!(ozone.len(), ozone_proto.len());

        let mut out = vec![0u8; 72];
        out[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        out[8] = 0;
        out[13] = 8;
        out[14] = 8;
        out[16..18].copy_from_slice(&4u16.to_le_bytes());
        out[18..20].copy_from_slice(&16u16.to_le_bytes());
        out[64..72].copy_from_slice(&root_addr.to_le_bytes());
        out.extend_from_slice(&root);
        out.extend_from_slice(&sst);
        out.extend_from_slice(&no_attrs);
        out.extend_from_slice(&big);
        out.extend_from_slice(&product);
        out.extend_from_slice(&ozone);
        out.resize(big_data as usize, 0);
        out.resize((big_data + 262144) as usize, 0x5a);
        (out, big_data, 262144)
    }

    fn fixture() -> &'static (Vec<u8>, u64, u64) {
        static FIXTURE: OnceLock<(Vec<u8>, u64, u64)> = OnceLock::new();
        FIXTURE.get_or_init(build_fixture)
    }

    fn fixture_path() -> &'static str {
        static PATH: OnceLock<String> = OnceLock::new();
        PATH.get_or_init(|| {
            let path = format!("/tmp/omegaflow_netcdf_vars_{}.h5", std::process::id());
            std::fs::write(&path, &fixture().0).expect("fixture write");
            path
        })
        .as_str()
    }

    fn run(args: &[&str]) -> (i32, String, String) {
        let owned: Vec<String> = args.iter().map(|s| (*s).to_string()).collect();
        let mut out = Vec::new();
        let mut err = Vec::new();
        let code = execute(&owned, &mut out, &mut err);
        (
            code,
            String::from_utf8(out).expect("stdout utf8"),
            String::from_utf8(err).expect("stderr utf8"),
        )
    }

    fn tsv_rows(output: &str) -> Vec<Vec<&str>> {
        let mut lines = output.lines();
        assert_eq!(lines.next(), Some(HEADER));
        lines.map(|line| line.split('\t').collect()).collect()
    }

    fn object_field<'a>(obj: &'a JsonVal, key: &str) -> &'a JsonVal {
        match obj {
            JsonVal::Obj(map) => match map.get(key) {
                Some(value) => value,
                None => panic!("field absent: {key}"),
            },
            other => panic!("not an object: {other:?}"),
        }
    }

    fn is_str(value: &JsonVal, wanted: &str) -> bool {
        matches!(value, JsonVal::Str(text) if text == wanted)
    }

    fn is_null(value: &JsonVal) -> bool {
        matches!(value, JsonVal::Null)
    }

    #[test]
    fn tsv_is_parseable_and_carries_metadata() {
        let (code, out, err) = run(&[fixture_path(), "--format", "tsv"]);
        assert_eq!(code, 0, "{err}");
        assert!(err.contains("file:"), "{err}");
        let rows = tsv_rows(&out);
        assert_eq!(rows.len(), 4);
        let sst = rows
            .iter()
            .find(|row| row[0] == "sst")
            .expect("sst row");
        assert_eq!(sst.len(), 10);
        assert_eq!(sst[1], "/");
        assert_eq!(sst[3], "float32");
        assert_eq!(sst[4], "2x3");
        assert_eq!(sst[5], "kelvin");
        assert_eq!(sst[6], "0.01");
        assert_eq!(sst[7], "273.15");
        assert_eq!(sst[8], "-32767");
        assert_eq!(sst[9], "-100,100");
        let ozone = rows
            .iter()
            .find(|row| row[0] == "ozone")
            .expect("ozone row");
        assert_eq!(ozone[1], "/PRODUCT");
        assert_eq!(ozone[4], "5");
        assert_eq!(ozone[5], "mol m-2");
    }

    #[test]
    fn jsonl_is_parseable_and_carries_metadata() {
        let (code, out, err) = run(&[fixture_path(), "--format", "jsonl"]);
        assert_eq!(code, 0, "{err}");
        let objects: Vec<JsonVal> = out
            .lines()
            .map(|line| parse_json(line).expect("jsonl line"))
            .collect();
        assert_eq!(objects.len(), 4);
        let sst = objects
            .iter()
            .find(|obj| is_str(object_field(obj, "variable"), "sst"))
            .expect("sst object");
        assert!(is_str(object_field(sst, "group"), "/"));
        assert!(is_str(object_field(sst, "dtype"), "float32"));
        assert!(is_str(object_field(sst, "units"), "kelvin"));
        match object_field(sst, "shape") {
            JsonVal::Arr(values) => {
                let dims: Vec<f64> = values
                    .iter()
                    .filter_map(|v| match v {
                        JsonVal::Num(n) => Some(*n),
                        _ => None,
                    })
                    .collect();
                assert_eq!(dims, vec![2.0, 3.0]);
            }
            other => panic!("shape is not an array: {other:?}"),
        }
        match object_field(sst, "scale_factor") {
            JsonVal::Num(n) => assert!((n - 0.01).abs() < 1e-12),
            other => panic!("scale_factor is not numeric: {other:?}"),
        }
        match object_field(sst, "valid_range") {
            JsonVal::Arr(values) => assert_eq!(values.len(), 2),
            other => panic!("valid_range is not an array: {other:?}"),
        }
    }

    #[test]
    fn missing_attributes_stay_empty() {
        let (code, out, err) = run(&[fixture_path(), "--format", "tsv"]);
        assert_eq!(code, 0, "{err}");
        let rows = tsv_rows(&out);
        let row = rows
            .iter()
            .find(|row| row[0] == "no_attrs")
            .expect("no_attrs row");
        for field in [5usize, 6, 7, 8, 9] {
            assert_eq!(row[field], "", "field {field} is not empty");
        }
        let (code, out, err) = run(&[fixture_path(), "--format", "jsonl"]);
        assert_eq!(code, 0, "{err}");
        let objects: Vec<JsonVal> = out
            .lines()
            .map(|line| parse_json(line).expect("jsonl line"))
            .collect();
        let row = objects
            .iter()
            .find(|obj| is_str(object_field(obj, "variable"), "no_attrs"))
            .expect("no_attrs object");
        for key in ["units", "scale_factor", "add_offset", "fill", "valid_range"] {
            assert!(is_null(object_field(row, key)), "{key} is not null");
        }
    }

    #[test]
    fn var_filter_selects_one_variable() {
        for name in ["sst", "/sst", "ozone", "/PRODUCT/ozone", "PRODUCT/ozone"] {
            let (code, out, err) = run(&[fixture_path(), "--var", name]);
            assert_eq!(code, 0, "{name}: {err}");
            let rows = tsv_rows(&out);
            assert_eq!(rows.len(), 1, "{name}");
            let leaf = name.rsplit('/').next().expect("leaf");
            assert_eq!(rows[0][0], leaf, "{name}");
        }
    }

    #[test]
    fn unknown_variable_exits_three_without_a_success_line() {
        let (code, out, err) = run(&[fixture_path(), "--var", "absent"]);
        assert_eq!(code, 3);
        assert!(out.is_empty(), "{out}");
        assert!(err.contains("variable absent: absent"), "{err}");
    }

    #[test]
    fn range_stub_reads_headers_only() {
        let (buf, big_start, big_len) = fixture();
        let base_len = (BASE_BYTES.min(buf.len() as u64)) as usize;
        let base = buf[..base_len].to_vec();
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        let mut bytes = 0u64;
        let rows = {
            let mut fetch = |off: u64, len: u64| -> Option<Vec<u8>> {
                if off >= buf.len() as u64 {
                    return None;
                }
                let want = len.max(FETCH_WINDOW).min(buf.len() as u64 - off);
                ranges.push((off, want));
                bytes += want;
                Some(buf[off as usize..(off + want) as usize].to_vec())
            };
            let file = Hdf5File::parse_fetch(&base, &mut fetch).expect("fixture parse");
            collect_rows(&file).expect("rows")
        };
        assert_eq!(rows.len(), 4);
        assert!(bytes < buf.len() as u64 / 2, "{bytes} of {}", buf.len());
        for (off, len) in &ranges {
            assert!(
                off + len <= *big_start,
                "range {off}+{len} reaches the {big_len} B payload at {big_start}"
            );
        }
        let big = rows.iter().find(|row| row.variable == "big").expect("big");
        assert_eq!(big.shape, vec![65536]);
        assert_eq!(big.dtype, "float32");
    }

    #[test]
    fn out_writes_the_same_rows_and_keeps_stdout_clean() {
        let path = format!("/tmp/omegaflow_netcdf_vars_out_{}.tsv", std::process::id());
        let (code, out, err) = run(&[fixture_path(), "--var", "sst", "--out", &path]);
        assert_eq!(code, 0, "{err}");
        assert!(out.is_empty(), "{out}");
        let written = std::fs::read_to_string(&path).expect("out file");
        let rows = tsv_rows(&written);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "sst");
    }
}
