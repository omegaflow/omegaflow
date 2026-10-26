use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5Attribute, Hdf5Datatype, Hdf5Note, Hdf5Object, LazyHdf5};
use omegaflow::sha256::sha256_hex;
use std::io::Write;
use std::process::Command;

const TAG: &str = "meeo-s5p.s3.amazonaws.com";
const DEFAULT_VAR: &str = "ozone_tropospheric_vertical_column";
const DEFAULT_LIMIT: usize = 100_000;
const MAX_LIMIT: usize = 1_000_000;
const DEFAULT_MAX_MB: u64 = 16;
const CSV_HEADER: &str =
    "time_raw,time_units,lat,lon,ozone_tropospheric_vertical_column,units,source,sha256";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Format {
    Csv,
    Jsonl,
}

#[derive(Clone)]
enum Num {
    Int(i128),
    F32(f32),
    F64(f64),
}

impl Num {
    fn as_f64(&self) -> f64 {
        match self {
            Num::Int(v) => *v as f64,
            Num::F32(v) => *v as f64,
            Num::F64(v) => *v,
        }
    }

    fn text(&self) -> String {
        match self {
            Num::Int(v) => v.to_string(),
            Num::F32(v) => float_text_f32(*v),
            Num::F64(v) => float_text_f64(*v),
        }
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

struct VarMeta {
    dims: Vec<u64>,
    datatype: Hdf5Datatype,
    units: Option<String>,
    scale: Option<f64>,
    offset: Option<f64>,
    fill: Option<Num>,
}

fn usage() -> String {
    format!(
        "usage: meeo_o3_compiler <url|file> [--var <name>] [--limit N] [--format csv|jsonl] [--out <file>] [--max-mb N] [--ci-mode]\n  --var default {DEFAULT_VAR}; --limit 1..{MAX_LIMIT} default {DEFAULT_LIMIT}; --format default csv; --max-mb default {DEFAULT_MAX_MB}; --ci-mode uploads the CSV to the CDN"
    )
}

fn skip_space(bytes: &[u8]) -> usize {
    let mut n = 0;
    while n < bytes.len() && (bytes[n] == 0 || bytes[n] == b' ') {
        n += 1;
    }
    n
}

fn attr_text(attr: &Hdf5Attribute) -> Option<String> {
    let text_like =
        attr.datatype.class == 3 || (attr.datatype.class == 9 && attr.datatype.vlen_is_string);
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

fn attr_number(attr: &Hdf5Attribute, unsigned: bool) -> Option<Num> {
    let size = attr.datatype.size;
    if size == 0 || attr.data.is_empty() {
        return None;
    }
    decode_number(&attr.data, 0, &attr.datatype, unsigned)
}

fn find_attr<'a>(obj: &'a Hdf5Object, name: &str) -> Option<&'a Hdf5Attribute> {
    obj.attrs.iter().find(|a| a.name == name)
}

fn object_unsigned(obj: &Hdf5Object) -> bool {
    find_attr(obj, "_Unsigned")
        .and_then(attr_text)
        .is_some_and(|text| text.eq_ignore_ascii_case("true"))
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

fn probe_size(url: &str) -> Result<u64, String> {
    if let Ok(out) = Command::new("curl")
        .args(["-sSI", "--max-time", "30", url])
        .output()
    {
        if out.status.success() {
            let headers = String::from_utf8_lossy(&out.stdout);
            if let Some(size) =
                header_value(&headers, "content-length").and_then(|v| v.parse::<u64>().ok())
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

fn fetch_source(target: &str, max_bytes: u64) -> Result<(Vec<u8>, String, String), String> {
    if target.starts_with("http://") || target.starts_with("https://") {
        let size = probe_size(target)?;
        if size > max_bytes {
            return Err(format!(
                "source has {size} B, above --max-mb bound of {max_bytes} B"
            ));
        }
        let out = Command::new("curl")
            .args([
                "-sSf",
                "--max-time",
                "300",
                "--max-filesize",
                &max_bytes.to_string(),
                target,
            ])
            .output()
            .map_err(|err| format!("fetch failed: {err}"))?;
        if !out.status.success() {
            return Err(format!(
                "fetch failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        if out.stdout.len() as u64 != size {
            return Err(format!(
                "fetch returned {} B, source reports {size} B",
                out.stdout.len()
            ));
        }
        let digest = sha256_hex(&out.stdout);
        let diagnostic = format!("fetch: {size} B from {target}");
        return Ok((out.stdout, digest, diagnostic));
    }
    let size = std::fs::metadata(target)
        .map_err(|err| format!("file does not open: {target}: {err}"))?
        .len();
    if size == 0 {
        return Err(format!("file is empty: {target}"));
    }
    if size > max_bytes {
        return Err(format!(
            "file has {size} B, above --max-mb bound of {max_bytes} B"
        ));
    }
    let bytes =
        std::fs::read(target).map_err(|err| format!("file does not read: {target}: {err}"))?;
    let digest = sha256_hex(&bytes);
    let diagnostic = format!("file: {size} B from {target}");
    Ok((bytes, digest, diagnostic))
}

fn resolve_path<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    file: &mut LazyHdf5<'_, F>,
    name: &str,
) -> Option<String> {
    let candidates = if name.starts_with('/') {
        vec![name.to_string()]
    } else {
        vec![
            format!("/PRODUCT/{name}"),
            format!("/{name}"),
            name.to_string(),
        ]
    };
    for candidate in candidates {
        if file.dataset(&candidate).is_ok() {
            return Some(candidate);
        }
    }
    None
}

fn read_variable<F: FnMut(u64, u64) -> Option<Vec<u8>>>(
    file: &mut LazyHdf5<'_, F>,
    path: &str,
) -> Result<(VarMeta, Vec<Num>), String> {
    let (dims, datatype, unsigned, units, scale, offset, fill) = {
        let (obj, ds, dt) = file.dataset(path).map_err(|note| note_text(&note))?;
        let unsigned = object_unsigned(obj);
        (
            ds.dims.clone(),
            dt.clone(),
            unsigned,
            find_attr(obj, "units").and_then(attr_text),
            find_attr(obj, "scale_factor")
                .and_then(|a| attr_number(a, unsigned))
                .map(|n| n.as_f64()),
            find_attr(obj, "add_offset")
                .and_then(|a| attr_number(a, unsigned))
                .map(|n| n.as_f64()),
            find_attr(obj, "_FillValue").and_then(|a| attr_number(a, unsigned)),
        )
    };
    if datatype.class != 0 && datatype.class != 1 {
        return Err(format!("dtype class {} unread for {path}", datatype.class));
    }
    let raw = file.read_dataset(path).map_err(|note| note_text(&note))?;
    let count = dims
        .iter()
        .fold(1usize, |acc, dim| acc.saturating_mul(*dim as usize));
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        let value = decode_number(&raw, index * datatype.size, &datatype, unsigned)
            .ok_or_else(|| format!("value {index} unread in {path}"))?;
        values.push(value);
    }
    Ok((
        VarMeta {
            dims,
            datatype,
            units,
            scale,
            offset,
            fill,
        },
        values,
    ))
}

fn is_fill(raw: &Num, fill: &Num) -> bool {
    match (raw, fill) {
        (Num::Int(a), Num::Int(b)) => a == b,
        _ => {
            let a = raw.as_f64();
            let b = fill.as_f64();
            a == b || (a.is_nan() && b.is_nan())
        }
    }
}

fn raw_present<'a>(raw: &'a Num, meta: &VarMeta) -> Option<&'a Num> {
    if let Some(fill) = &meta.fill {
        if is_fill(raw, fill) {
            return None;
        }
    }
    if !raw.as_f64().is_finite() {
        return None;
    }
    Some(raw)
}

fn applied(raw: &Num, meta: &VarMeta) -> Option<f64> {
    let raw = raw_present(raw, meta)?;
    let mut value = raw.as_f64();
    if let Some(scale) = meta.scale {
        value *= scale;
    }
    if let Some(offset) = meta.offset {
        value += offset;
    }
    value.is_finite().then_some(value)
}

fn value_text(value: f64, datatype: &Hdf5Datatype) -> String {
    if datatype.class == 1 && datatype.size == 4 {
        float_text_f32(value as f32)
    } else {
        float_text_f64(value)
    }
}

struct Row {
    time_raw: Option<String>,
    time_units: Option<String>,
    lat: Option<String>,
    lon: Option<String>,
    value: Option<String>,
    units: Option<String>,
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

fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn render_csv(rows: &[Row], source: &str, digest: &str) -> String {
    let mut out = String::from(CSV_HEADER);
    out.push('\n');
    for row in rows {
        let fields = [
            csv_field(row.time_raw.as_deref().unwrap_or("")),
            csv_field(row.time_units.as_deref().unwrap_or("")),
            csv_field(row.lat.as_deref().unwrap_or("")),
            csv_field(row.lon.as_deref().unwrap_or("")),
            csv_field(row.value.as_deref().unwrap_or("")),
            csv_field(row.units.as_deref().unwrap_or("")),
            csv_field(source),
            csv_field(digest),
        ];
        out.push_str(&fields.join(","));
        out.push('\n');
    }
    out
}

fn json_number(value: &Option<String>) -> String {
    match value {
        Some(value) => value.clone(),
        None => "null".to_string(),
    }
}

fn json_text(value: &Option<String>) -> String {
    match value.as_deref() {
        Some(value) => json_str(value),
        None => "null".to_string(),
    }
}

fn render_jsonl(rows: &[Row], source: &str, digest: &str) -> String {
    let mut out = String::new();
    for row in rows {
        out.push_str(&format!(
            "{{\"time_raw\":{},\"time_units\":{},\"lat\":{},\"lon\":{},\"value\":{},\"units\":{},\"source\":{},\"sha256\":{}}}\n",
            json_number(&row.time_raw),
            json_text(&row.time_units),
            json_number(&row.lat),
            json_number(&row.lon),
            json_number(&row.value),
            json_text(&row.units),
            json_str(source),
            json_str(digest)
        ));
    }
    out
}

fn build_rows(
    field: &VarMeta,
    field_values: &[Num],
    lat: &VarMeta,
    lat_values: &[Num],
    lon: &VarMeta,
    lon_values: &[Num],
    time: &VarMeta,
    time_values: &[Num],
) -> Result<Vec<Row>, String> {
    if field.dims.len() != 3 {
        return Err(format!(
            "pending — field rank {} is not 1xlatxlon",
            field.dims.len()
        ));
    }
    if lat.dims.len() != 1 || lon.dims.len() != 1 || time.dims.len() != 1 {
        return Err("pending — latitude, longitude or time is not one-dimensional".to_string());
    }
    let steps = field.dims[0];
    let nlat = field.dims[1];
    let nlon = field.dims[2];
    if steps != 1 {
        return Err(format!(
            "pending — field carries {steps} time steps, the pilot reads one"
        ));
    }
    if lat.dims[0] != nlat || lon.dims[0] != nlon {
        return Err(format!(
            "pending — field {}x{} does not match latitude {}x longitude {}",
            nlat, nlon, lat.dims[0], lon.dims[0]
        ));
    }
    if time.dims[0] != steps || time_values.is_empty() {
        return Err(format!(
            "pending — time length {} does not match field steps {}",
            time.dims[0], steps
        ));
    }
    let time_raw = raw_present(&time_values[0], time).map(Num::text);
    let mut rows = Vec::with_capacity((nlat * nlon) as usize);
    for i in 0..nlat {
        for j in 0..nlon {
            let flat = (i * nlon + j) as usize;
            let value = field_values
                .get(flat)
                .and_then(|raw| applied(raw, field))
                .map(|v| value_text(v, &field.datatype));
            let lat_text = lat_values
                .get(i as usize)
                .and_then(|raw| applied(raw, lat))
                .map(|v| value_text(v, &lat.datatype));
            let lon_text = lon_values
                .get(j as usize)
                .and_then(|raw| applied(raw, lon))
                .map(|v| value_text(v, &lon.datatype));
            rows.push(Row {
                time_raw: time_raw.clone(),
                time_units: time.units.clone(),
                lat: lat_text,
                lon: lon_text,
                value,
                units: field.units.clone(),
            });
        }
    }
    Ok(rows)
}

fn parse_limit(value: &str) -> Result<usize, String> {
    let limit = value
        .parse::<usize>()
        .map_err(|_| format!("--limit is not a number: {value}"))?;
    if limit == 0 || limit > MAX_LIMIT {
        return Err(format!("--limit must be 1..{MAX_LIMIT}"));
    }
    Ok(limit)
}

fn parse_max_mb(value: &str) -> Result<u64, String> {
    let mb = value
        .parse::<u64>()
        .map_err(|_| format!("--max-mb is not a number: {value}"))?;
    if mb == 0 || mb > 1024 {
        return Err("--max-mb must be 1..1024".to_string());
    }
    Ok(mb)
}

fn execute(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    let mut target: Option<String> = None;
    let mut var = DEFAULT_VAR.to_string();
    let mut limit = DEFAULT_LIMIT;
    let mut format = Format::Csv;
    let mut out_path: Option<String> = None;
    let mut max_mb = DEFAULT_MAX_MB;
    let mut ci_mode = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--var" => {
                let Some(value) = args.get(i + 1) else {
                    let _ = writeln!(err, "--var needs a name\n{}", usage());
                    return 2;
                };
                var = value.clone();
                i += 2;
            }
            "--limit" => {
                let Some(value) = args.get(i + 1) else {
                    let _ = writeln!(err, "--limit needs a number\n{}", usage());
                    return 2;
                };
                limit = match parse_limit(value) {
                    Ok(limit) => limit,
                    Err(message) => {
                        let _ = writeln!(err, "{message}\n{}", usage());
                        return 2;
                    }
                };
                i += 2;
            }
            "--format" => {
                let Some(value) = args.get(i + 1) else {
                    let _ = writeln!(err, "--format needs csv or jsonl\n{}", usage());
                    return 2;
                };
                format = match value.as_str() {
                    "csv" => Format::Csv,
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
            "--max-mb" => {
                let Some(value) = args.get(i + 1) else {
                    let _ = writeln!(err, "--max-mb needs a number\n{}", usage());
                    return 2;
                };
                max_mb = match parse_max_mb(value) {
                    Ok(mb) => mb,
                    Err(message) => {
                        let _ = writeln!(err, "{message}\n{}", usage());
                        return 2;
                    }
                };
                i += 2;
            }
            "--ci-mode" => {
                ci_mode = true;
                i += 1;
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
    let max_bytes = max_mb.saturating_mul(1024 * 1024);
    let (bytes, digest, source_diagnostic) = match fetch_source(&target, max_bytes) {
        Ok(result) => result,
        Err(message) => {
            let _ = writeln!(err, "{message}");
            return 2;
        }
    };
    let mut file = match LazyHdf5::open(&bytes, |_off: u64, _len: u64| -> Option<Vec<u8>> { None })
    {
        Ok(file) => file,
        Err(note) => {
            let _ = writeln!(err, "{}", note_text(&note));
            return 2;
        }
    };
    let Some(field_path) = resolve_path(&mut file, &var) else {
        let _ = writeln!(err, "variable absent: {var}");
        return 3;
    };
    let Some(lat_path) = resolve_path(&mut file, "latitude_ccd") else {
        let _ = writeln!(err, "pending — no latitude_ccd");
        return 2;
    };
    let Some(lon_path) = resolve_path(&mut file, "longitude_ccd") else {
        let _ = writeln!(err, "pending — no longitude_ccd");
        return 2;
    };
    let Some(time_path) = resolve_path(&mut file, "time") else {
        let _ = writeln!(err, "pending — no time");
        return 2;
    };
    let (field_meta, field_values) = match read_variable(&mut file, &field_path) {
        Ok(result) => result,
        Err(message) => {
            let _ = writeln!(err, "pending — {message}");
            return 2;
        }
    };
    let (lat_meta, lat_values) = match read_variable(&mut file, &lat_path) {
        Ok(result) => result,
        Err(message) => {
            let _ = writeln!(err, "pending — {message}");
            return 2;
        }
    };
    let (lon_meta, lon_values) = match read_variable(&mut file, &lon_path) {
        Ok(result) => result,
        Err(message) => {
            let _ = writeln!(err, "pending — {message}");
            return 2;
        }
    };
    let (time_meta, time_values) = match read_variable(&mut file, &time_path) {
        Ok(result) => result,
        Err(message) => {
            let _ = writeln!(err, "pending — {message}");
            return 2;
        }
    };
    let rows = match build_rows(
        &field_meta,
        &field_values,
        &lat_meta,
        &lat_values,
        &lon_meta,
        &lon_values,
        &time_meta,
        &time_values,
    ) {
        Ok(rows) => rows,
        Err(message) => {
            let _ = writeln!(err, "{message}");
            return 2;
        }
    };
    if rows.is_empty() {
        let _ = writeln!(err, "pending — field carries no values");
        return 2;
    }
    let take = limit.min(rows.len());
    let rows = &rows[..take];
    let empty = rows.iter().filter(|row| row.value.is_none()).count();
    let real = rows.len() - empty;
    let rendered = match format {
        Format::Csv => render_csv(rows, &target, &digest),
        Format::Jsonl => render_jsonl(rows, &target, &digest),
    };
    let written_path = match &out_path {
        Some(path) => {
            if let Err(error) = std::fs::write(path, &rendered) {
                let _ = writeln!(err, "output file does not write: {path}: {error}");
                return 2;
            }
            Some(path.clone())
        }
        None => {
            if let Err(error) = out.write_all(rendered.as_bytes()) {
                let _ = writeln!(err, "output does not write: {error}");
                return 2;
            }
            None
        }
    };
    let csv_digest = sha256_hex(rendered.as_bytes());
    let _ = writeln!(
        err,
        "meeo_o3_compiler: field={field_path} shape={}x{}x{} values={} real={} empty={} {source_diagnostic} sha256={digest} csv_sha256={csv_digest} bytes={}",
        field_meta.dims[0],
        field_meta.dims[1],
        field_meta.dims[2],
        rows.len(),
        real,
        empty,
        rendered.len()
    );
    if ci_mode {
        let Some(path) = written_path else {
            let _ = writeln!(err, "--ci-mode needs --out <file>");
            return 2;
        };
        if !upload_release(TAG, &path) {
            return 2;
        }
    }
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

    fn datatype_i32() -> Vec<u8> {
        let d = vec![0x10, 0x08, 0, 0, 4, 0, 0, 0, 0, 0, 32, 0];
        message(0x03, &d)
    }

    fn attr_raw(dt: &[u8], ds: &[u8], name: &str, value: &[u8]) -> Vec<u8> {
        assert!(name.len() + 1 <= u16::MAX as usize);
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
        let mut dt = vec![0x13, 0, 0, 0];
        dt.extend_from_slice(&(value.len() as u32).to_le_bytes());
        attr_raw(&dt, &[2, 0, 0, 0], name, value.as_bytes())
    }

    fn attr_f32(name: &str, value: f32) -> Vec<u8> {
        let mut dt = vec![0x11, 0, 0, 0];
        dt.extend_from_slice(&4u32.to_le_bytes());
        dt.extend_from_slice(&[0, 0, 32, 0]);
        dt.extend_from_slice(&[0u8; 8]);
        attr_raw(&dt, &[2, 0, 0, 0], name, &value.to_le_bytes())
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

    struct Fixture {
        bytes: Vec<u8>,
        path: String,
    }

    fn build_fixture() -> Fixture {
        let field_attrs = vec![
            attr_string("units", "mol m-2"),
            attr_f32("scale_factor", 0.5),
            attr_f32("add_offset", 10.0),
            attr_f32("_FillValue", -9999.0),
        ];
        let lat_attrs = vec![attr_string("units", "degrees_north")];
        let lon_attrs = vec![attr_string("units", "degrees_east")];
        let time_attrs = vec![attr_string("units", "seconds")];
        let root_addr = 72u64;
        let root_proto = header(&[group_info(), link("PRODUCT", 0)]);
        let product_addr = root_addr + root_proto.len() as u64;
        let product_proto = header(&[
            group_info(),
            link("ozone_tropospheric_vertical_column", 0),
            link("latitude_ccd", 0),
            link("longitude_ccd", 0),
            link("time", 0),
            link("wrong_shape", 0),
        ]);
        let field_addr = product_addr + product_proto.len() as u64;
        let field_proto = dataset_header(&[1, 2, 3], &datatype_f32(), &field_attrs, 0, 24);
        let lat_addr = field_addr + field_proto.len() as u64;
        let lat_proto = dataset_header(&[2], &datatype_f32(), &lat_attrs, 0, 8);
        let lon_addr = lat_addr + lat_proto.len() as u64;
        let lon_proto = dataset_header(&[3], &datatype_f32(), &lon_attrs, 0, 12);
        let time_addr = lon_addr + lon_proto.len() as u64;
        let time_proto = dataset_header(&[1], &datatype_i32(), &time_attrs, 0, 4);
        let wrong_addr = time_addr + time_proto.len() as u64;
        let wrong_proto = dataset_header(&[2], &datatype_f32(), &[], 0, 8);
        let data_start = wrong_addr + wrong_proto.len() as u64;
        let field_data = data_start;
        let lat_data = field_data + 24;
        let lon_data = lat_data + 8;
        let time_data = lon_data + 12;
        let wrong_data = time_data + 4;

        let root = header(&[group_info(), link("PRODUCT", product_addr)]);
        let product = header(&[
            group_info(),
            link("ozone_tropospheric_vertical_column", field_addr),
            link("latitude_ccd", lat_addr),
            link("longitude_ccd", lon_addr),
            link("time", time_addr),
            link("wrong_shape", wrong_addr),
        ]);
        let field = dataset_header(&[1, 2, 3], &datatype_f32(), &field_attrs, field_data, 24);
        let lat = dataset_header(&[2], &datatype_f32(), &lat_attrs, lat_data, 8);
        let lon = dataset_header(&[3], &datatype_f32(), &lon_attrs, lon_data, 12);
        let time = dataset_header(&[1], &datatype_i32(), &time_attrs, time_data, 4);
        let wrong = dataset_header(&[2], &datatype_f32(), &[], wrong_data, 8);
        assert_eq!(root.len(), root_proto.len());
        assert_eq!(product.len(), product_proto.len());
        assert_eq!(field.len(), field_proto.len());
        assert_eq!(lat.len(), lat_proto.len());
        assert_eq!(lon.len(), lon_proto.len());
        assert_eq!(time.len(), time_proto.len());
        assert_eq!(wrong.len(), wrong_proto.len());

        let mut bytes = vec![0u8; 72];
        bytes[..8].copy_from_slice(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]);
        bytes[8] = 0;
        bytes[13] = 8;
        bytes[14] = 8;
        bytes[16..18].copy_from_slice(&4u16.to_le_bytes());
        bytes[18..20].copy_from_slice(&16u16.to_le_bytes());
        bytes[64..72].copy_from_slice(&root_addr.to_le_bytes());
        bytes.extend_from_slice(&root);
        bytes.extend_from_slice(&product);
        bytes.extend_from_slice(&field);
        bytes.extend_from_slice(&lat);
        bytes.extend_from_slice(&lon);
        bytes.extend_from_slice(&time);
        bytes.extend_from_slice(&wrong);
        for value in [1.5f32, -9999.0, f32::NAN, 2.0, -0.0, 4.0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        for value in [10.0f32, 20.0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        for value in [100.0f32, 200.0, 300.0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(&1234i32.to_le_bytes());
        bytes.extend_from_slice(&7.0f32.to_le_bytes());
        bytes.extend_from_slice(&8.0f32.to_le_bytes());
        let path = format!("/tmp/omegaflow_meeo_o3_{}.h5", std::process::id());
        std::fs::write(&path, &bytes).expect("fixture write");
        Fixture { bytes, path }
    }

    fn fixture() -> &'static Fixture {
        static FIXTURE: OnceLock<Fixture> = OnceLock::new();
        FIXTURE.get_or_init(build_fixture)
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

    fn csv_rows(output: &str) -> Vec<Vec<&str>> {
        let mut lines = output.lines();
        assert_eq!(lines.next(), Some(CSV_HEADER));
        lines.map(|line| line.split(',').collect()).collect()
    }

    fn is_str(value: &JsonVal, wanted: &str) -> bool {
        matches!(value, JsonVal::Str(text) if text == wanted)
    }

    fn is_num(value: &JsonVal, wanted: f64) -> bool {
        matches!(value, JsonVal::Num(number) if (*number - wanted).abs() < 1e-12)
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

    #[test]
    fn csv_carries_values_time_and_coordinates() {
        let f = fixture();
        let (code, out, err) = run(&[&f.path, "--format", "csv"]);
        assert_eq!(code, 0, "{err}");
        let rows = csv_rows(&out);
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0].len(), 8);
        assert_eq!(rows[0][0], "1234");
        assert_eq!(rows[0][1], "seconds");
        assert_eq!(rows[0][2], "10");
        assert_eq!(rows[0][3], "100");
        assert_eq!(rows[0][4], "10.75");
        assert_eq!(rows[0][5], "mol m-2");
        assert_eq!(rows[0][6], f.path);
        assert_eq!(rows[0][7], sha256_hex(&f.bytes));
        assert_eq!(rows[1][4], "");
        assert_eq!(rows[2][4], "");
        assert_eq!(rows[3][4], "11");
        assert_eq!(rows[4][2], "20");
        assert_eq!(rows[4][3], "200");
        assert_eq!(rows[4][4], "10");
        assert_eq!(rows[5][4], "12");
        assert!(err.contains("real=4"), "{err}");
        assert!(err.contains("empty=2"), "{err}");
    }

    #[test]
    fn jsonl_carries_values_and_nulls() {
        let f = fixture();
        let (code, out, err) = run(&[&f.path, "--format", "jsonl"]);
        assert_eq!(code, 0, "{err}");
        let objects: Vec<JsonVal> = out
            .lines()
            .map(|line| parse_json(line).expect("jsonl line"))
            .collect();
        assert_eq!(objects.len(), 6);
        assert!(is_num(object_field(&objects[0], "time_raw"), 1234.0));
        assert!(is_str(object_field(&objects[0], "time_units"), "seconds"));
        assert!(is_num(object_field(&objects[0], "lat"), 10.0));
        assert!(is_num(object_field(&objects[0], "lon"), 100.0));
        assert!(is_num(object_field(&objects[0], "value"), 10.75));
        assert!(is_str(object_field(&objects[0], "units"), "mol m-2"));
        assert!(is_str(object_field(&objects[0], "source"), &f.path));
        assert!(is_str(
            object_field(&objects[0], "sha256"),
            &sha256_hex(&f.bytes)
        ));
        assert!(matches!(object_field(&objects[1], "value"), JsonVal::Null));
        assert!(matches!(object_field(&objects[2], "value"), JsonVal::Null));
        assert!(is_num(object_field(&objects[4], "value"), 10.0));
    }

    #[test]
    fn var_filter_accepts_leaf_and_path() {
        let f = fixture();
        for name in [
            "ozone_tropospheric_vertical_column",
            "/PRODUCT/ozone_tropospheric_vertical_column",
        ] {
            let (code, out, err) = run(&[&f.path, "--var", name, "--limit", "1"]);
            assert_eq!(code, 0, "{name}: {err}");
            assert_eq!(csv_rows(&out).len(), 1);
        }
        let (code, out, err) = run(&[&f.path, "--var", "absent"]);
        assert_eq!(code, 3, "{err}");
        assert!(out.is_empty(), "{out}");
        assert!(err.contains("variable absent: absent"), "{err}");
    }

    #[test]
    fn non_field_variable_stays_pending() {
        let f = fixture();
        let (code, out, err) = run(&[&f.path, "--var", "latitude_ccd"]);
        assert_eq!(code, 2, "{err}");
        assert!(out.is_empty(), "{out}");
        assert!(err.contains("pending — field rank 1"), "{err}");
    }

    #[test]
    fn limit_caps_rows_and_rejects_out_of_range() {
        let f = fixture();
        for (limit, expected) in [("1", 1usize), ("2", 2), ("10", 6)] {
            let (code, out, err) = run(&[&f.path, "--limit", limit]);
            assert_eq!(code, 0, "{err}");
            assert_eq!(csv_rows(&out).len(), expected);
        }
        for bad in ["0", "1000001", "x"] {
            let (code, out, err) = run(&[&f.path, "--limit", bad]);
            assert_eq!(code, 2, "{bad}: {err}");
            assert!(out.is_empty(), "{bad}: {out}");
        }
    }

    #[test]
    fn out_writes_the_rows_and_keeps_stdout_clean() {
        let f = fixture();
        let path = format!("/tmp/omegaflow_meeo_o3_out_{}.csv", std::process::id());
        let (code, out, err) = run(&[&f.path, "--limit", "2", "--out", &path]);
        assert_eq!(code, 0, "{err}");
        assert!(out.is_empty(), "{out}");
        let written = std::fs::read_to_string(&path).expect("out file");
        assert_eq!(csv_rows(&written).len(), 2);
    }

    #[test]
    fn max_mb_bounds_a_large_source() {
        let f = fixture();
        let (code, out, err) = run(&[&f.path, "--max-mb", "1"]);
        assert_eq!(code, 0, "{err}");
        assert_eq!(csv_rows(&out).len(), 6);
        let big = format!("/tmp/omegaflow_meeo_o3_big_{}.bin", std::process::id());
        std::fs::write(&big, vec![0u8; 2 * 1024 * 1024]).expect("big write");
        let (code, out, err) = run(&[&big, "--max-mb", "1"]);
        assert_eq!(code, 2, "{err}");
        assert!(out.is_empty(), "{out}");
        assert!(err.contains("above --max-mb"), "{err}");
    }
}
