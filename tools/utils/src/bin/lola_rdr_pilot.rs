use omegaflow::archivar::sha256::sha256_hex;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::Read;
use std::process::Command;

const DEFAULT_RECORDS: u64 = 100;
const MAX_RECORDS: u64 = 10_000;
const LABEL_CAP: u64 = 1_048_576;
const FRACTION_DENOMINATOR: f64 = 4_294_967_296.0;

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Csv,
    Jsonl,
}

#[derive(Clone)]
enum Missing {
    Int(i128),
    Real(f64),
    Text(String),
}

#[derive(Clone)]
struct Column {
    name: String,
    number: u64,
    data_type: String,
    start_byte: u64,
    bytes: u64,
    items: u64,
    item_bytes: u64,
    unit: Option<String>,
    missing: Option<Missing>,
}

struct Label {
    record_bytes: u64,
    file_records: u64,
    start_time: Option<String>,
    structure: Option<String>,
    columns: Vec<Column>,
}

struct Config {
    label_source: String,
    dat: Option<String>,
    records: u64,
    format: Format,
    out: Option<String>,
}

struct Part {
    text: String,
    json: String,
    number: bool,
}

struct Cell {
    parts: Vec<Part>,
}

struct ColumnDraft {
    name: Option<String>,
    number: Option<u64>,
    data_type: Option<String>,
    start_byte: Option<u64>,
    bytes: Option<u64>,
    items: Option<u64>,
    item_bytes: Option<u64>,
    unit: Option<String>,
    missing: Option<Missing>,
}

struct Parsed {
    record_bytes: Option<u64>,
    file_records: Option<u64>,
    start_time: Option<String>,
    structure: Option<String>,
    columns: Vec<Column>,
}

fn usage() -> String {
    [
        "usage: lola_rdr_pilot <lbl-url|file> [--dat <url|file>] [--records N] [--format csv|jsonl] [--out <file>]",
        "  --dat       data product; default derives .dat next to the label",
        "  --records   records to read, default 100, range 1..10000",
        "  --format    csv (default) or jsonl",
        "  --out       write output to file instead of stdout",
        "  output      time, record (1-based), columns, source, sha256 of the read prefix",
        "  columns     carry their UNIT as NAME[UNIT]; ITEMS values join with a space in CSV and form an array in JSONL",
        "  time        TRANSMIT_TIME as TDT seconds from J2000, else MET_SECONDS+SUBSECONDS; START_TIME stays a label anchor",
    ]
    .join("\n")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match execute(&args) {
        Ok((text, Some(path))) => {
            if let Err(e) = fs::write(&path, text) {
                eprintln!("lola_rdr_pilot: {path} writes not: {e}");
                std::process::exit(2);
            }
        }
        Ok((text, None)) => print!("{text}"),
        Err((0, text)) => {
            println!("{text}");
        }
        Err((code, text)) => {
            eprintln!("lola_rdr_pilot: {text}");
            std::process::exit(code);
        }
    }
}

fn execute(args: &[String]) -> Result<(String, Option<String>), (i32, String)> {
    let config = parse_args(args)?;
    let text = produce(&config)?;
    Ok((text, config.out.clone()))
}

fn parse_args(args: &[String]) -> Result<Config, (i32, String)> {
    let mut label: Option<String> = None;
    let mut dat: Option<String> = None;
    let mut records: Option<u64> = None;
    let mut format: Option<Format> = None;
    let mut out: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        match arg {
            "--help" | "-h" => return Err((0, usage())),
            "--dat" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| (2, "--dat needs a value".to_string()))?;
                if dat.replace(value.clone()).is_some() {
                    return Err((2, "--dat given twice".to_string()));
                }
            }
            "--records" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| (2, "--records needs a value".to_string()))?;
                if records.is_some() {
                    return Err((2, "--records given twice".to_string()));
                }
                if value.starts_with('-') {
                    return Err((2, format!("--records is not positive: {value}")));
                }
                let count: u64 = value
                    .parse()
                    .map_err(|_| (2, format!("--records is not an integer: {value}")))?;
                if count == 0 || count > MAX_RECORDS {
                    return Err((2, format!("--records outside 1..{MAX_RECORDS}: {value}")));
                }
                records = Some(count);
            }
            "--format" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| (2, "--format needs a value".to_string()))?;
                if format.is_some() {
                    return Err((2, "--format given twice".to_string()));
                }
                format = Some(match value.as_str() {
                    "csv" => Format::Csv,
                    "jsonl" => Format::Jsonl,
                    _ => {
                        return Err((2, format!("--format is not csv or jsonl: {value}")));
                    }
                });
            }
            "--out" => {
                i += 1;
                let value = args
                    .get(i)
                    .ok_or_else(|| (2, "--out needs a value".to_string()))?;
                if out.replace(value.clone()).is_some() {
                    return Err((2, "--out given twice".to_string()));
                }
            }
            _ if arg.starts_with('-') => {
                return Err((2, format!("unknown option: {arg}")));
            }
            _ => {
                if label.replace(arg.to_string()).is_some() {
                    return Err((2, format!("unexpected argument: {arg}")));
                }
            }
        }
        i += 1;
    }
    let label = label.ok_or_else(|| (2, usage()))?;
    Ok(Config {
        label_source: label,
        dat,
        records: records.unwrap_or(DEFAULT_RECORDS),
        format: format.unwrap_or(Format::Csv),
        out,
    })
}

fn produce(config: &Config) -> Result<String, (i32, String)> {
    let label_text = read_all(&config.label_source)
        .map_err(|e| (3, format!("pending — label read void: {e}")))?;
    let mut label = parse_label(&label_text).map_err(|e| (3, format!("pending — {e}")))?;
    let mut structure_source: Option<String> = None;
    if label.columns.is_empty()
        && let Some(structure) = label.structure.clone()
    {
        let (columns, source) =
            load_structure(&config.label_source, &structure, label.record_bytes)
                .map_err(|e| (3, format!("pending — {e}")))?;
        label.columns = columns;
        structure_source = Some(source);
    }
    let count = config.records.min(label.file_records);
    if count == 0 {
        return Err((3, "pending — label carries no records".to_string()));
    }
    let prefix_bytes = count
        .checked_mul(label.record_bytes)
        .ok_or_else(|| (3, "pending — record prefix overflows".to_string()))?;
    let dat = match config.dat.clone() {
        Some(value) => value,
        None => derive_dat(&config.label_source).map_err(|e| (3, format!("pending — {e}")))?,
    };
    let prefix = read_prefix(&dat, prefix_bytes)
        .map_err(|e| (3, format!("pending — data read void: {e}")))?;
    let digest = sha256_hex(&prefix);
    let (text, time_source) =
        render(&label, &prefix, &dat, config.format).map_err(|e| (3, format!("pending — {e}")))?;
    let start_time = label.start_time.as_deref().unwrap_or("absent");
    let structure = structure_source.as_deref().unwrap_or("absent");
    eprintln!(
        "lola_rdr_pilot: records={} record_bytes={} file_records={} start_time={} time_source={} structure={} columns={} source={} sha256={}",
        count,
        label.record_bytes,
        label.file_records,
        start_time,
        time_source,
        structure,
        label.columns.len(),
        dat,
        digest
    );
    Ok(text)
}

fn read_all(source: &str) -> Result<String, String> {
    read_small(source, "label")
}

fn read_small(source: &str, kind: &str) -> Result<String, String> {
    if is_url(source) {
        let cap = LABEL_CAP.to_string();
        let range = format!("0-{}", LABEL_CAP - 1);
        let out = Command::new("curl")
            .args([
                "-sSf",
                "--max-time",
                "30",
                "--max-filesize",
                &cap,
                "-r",
                &range,
                source,
            ])
            .output()
            .map_err(|e| format!("curl does not run: {e}"))?;
        if !out.status.success() {
            return Err(format!("{source}: curl exits {}", out.status));
        }
        if out.stdout.is_empty() {
            return Err(format!("{source}: empty {kind} body"));
        }
        if out.stdout.len() as u64 >= LABEL_CAP {
            return Err(format!("{source}: {kind} reaches the {LABEL_CAP}-byte cap"));
        }
        return String::from_utf8(out.stdout)
            .map_err(|e| format!("{source}: {kind} is not UTF-8: {e}"));
    }
    fs::read_to_string(source).map_err(|e| format!("{source}: {e}"))
}

fn read_prefix(source: &str, length: u64) -> Result<Vec<u8>, String> {
    if is_url(source) {
        let cap = length.to_string();
        let range = format!("0-{}", length - 1);
        let out = Command::new("curl")
            .args([
                "-sSf",
                "--max-time",
                "60",
                "--max-filesize",
                &cap,
                "-r",
                &range,
                source,
            ])
            .output()
            .map_err(|e| format!("curl does not run: {e}"))?;
        if !out.status.success() {
            return Err(format!("range request exits {}", out.status));
        }
        if out.stdout.len() as u64 != length {
            return Err(format!(
                "range prefix carries {} of {} bytes",
                out.stdout.len(),
                length
            ));
        }
        return Ok(out.stdout);
    }
    let file = fs::File::open(source).map_err(|e| format!("{source}: {e}"))?;
    let mut prefix = Vec::new();
    file.take(length)
        .read_to_end(&mut prefix)
        .map_err(|e| format!("{source}: {e}"))?;
    if prefix.len() as u64 != length {
        return Err(format!(
            "file prefix carries {} of {} bytes",
            prefix.len(),
            length
        ));
    }
    Ok(prefix)
}

fn is_url(source: &str) -> bool {
    source.starts_with("http://") || source.starts_with("https://")
}

fn derive_dat(source: &str) -> Result<String, String> {
    let lower = source.to_ascii_lowercase();
    if lower.ends_with(".lbl") {
        return Ok(format!("{}.dat", &source[..source.len() - 4]));
    }
    Err("no .dat given and the label name does not end in .lbl".to_string())
}

fn structure_candidates(label_source: &str, structure: &str) -> Vec<String> {
    let dir = label_source
        .rsplit_once('/')
        .map(|(head, _)| head)
        .unwrap_or("");
    let mut bases: Vec<String> = Vec::new();
    if let Some((volume, _)) = dir.split_once("/data/") {
        bases.push(format!("{volume}/label"));
    }
    if dir.is_empty() {
        bases.push(String::new());
    } else {
        bases.push(format!("{dir}/label"));
        bases.push(dir.to_string());
        if let Some((parent, _)) = dir.rsplit_once('/')
            && !parent.is_empty()
            && !parent.ends_with(':')
        {
            bases.push(format!("{parent}/label"));
        }
    }
    let mut variants = vec![
        structure.to_ascii_lowercase(),
        structure.to_string(),
        structure.to_ascii_uppercase(),
    ];
    variants.dedup();
    let mut candidates = Vec::new();
    for base in bases {
        for variant in &variants {
            let candidate = if base.is_empty() {
                variant.clone()
            } else {
                format!("{base}/{variant}")
            };
            if !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
    }
    candidates
}

fn load_structure(
    label_source: &str,
    structure: &str,
    record_bytes: u64,
) -> Result<(Vec<Column>, String), String> {
    let mut seen = Vec::new();
    for candidate in structure_candidates(label_source, structure) {
        match read_small(&candidate, "structure") {
            Ok(text) => {
                seen.push(candidate.clone());
                let parsed = parse_objects(&text)
                    .map_err(|e| format!("{candidate}: structure parse void: {e}"))?;
                if parsed.columns.is_empty() {
                    return Err(format!("{candidate}: no OBJECT=COLUMN block"));
                }
                let mut columns = parsed.columns;
                validate_columns(&mut columns, record_bytes)?;
                return Ok((columns, candidate));
            }
            Err(_) => seen.push(candidate),
        }
    }
    Err(format!(
        "structure {structure} not found; tried {}",
        seen.join(", ")
    ))
}

fn strip_block_comments(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let mut in_comment = false;
    while i < bytes.len() {
        if !in_comment && i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            in_comment = true;
            i += 2;
            continue;
        }
        if in_comment && i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/' {
            in_comment = false;
            i += 2;
            continue;
        }
        if !in_comment {
            out.push(bytes[i] as char);
        } else if bytes[i] == b'\n' {
            out.push('\n');
        }
        i += 1;
    }
    out
}

fn scalar(raw: &str) -> String {
    let mut text = raw.trim();
    let mut quoted = false;
    let mut quote = '\0';
    let mut cut = text.len();
    for (i, c) in text.char_indices() {
        if quoted {
            if c == quote {
                quoted = false;
            }
        } else if c == '"' || c == '\'' {
            quoted = true;
            quote = c;
        } else if c == '/' && text[i..].starts_with("/*") {
            cut = i;
            break;
        }
    }
    text = text[..cut].trim();
    if text.len() >= 2 {
        let first = text.as_bytes()[0];
        let last = text.as_bytes()[text.len() - 1];
        if (first == b'"' || first == b'\'') && last == first {
            return text[1..text.len() - 1].trim().to_string();
        }
        if first == b'"' || first == b'\'' {
            let inner = &text[1..];
            if let Some(end) = inner.find(first as char) {
                return inner[..end].trim().to_string();
            }
            return inner.trim().to_string();
        }
    }
    text.to_string()
}

fn put<T>(slot: &mut Option<T>, value: T, key: &str) -> Result<(), String> {
    if slot.is_some() {
        return Err(format!("{key} appears twice"));
    }
    *slot = Some(value);
    Ok(())
}

fn parse_objects(text: &str) -> Result<Parsed, String> {
    let clean = strip_block_comments(text);
    let mut parsed = Parsed {
        record_bytes: None,
        file_records: None,
        start_time: None,
        structure: None,
        columns: Vec::new(),
    };
    let mut current: Option<ColumnDraft> = None;
    for raw in clean.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('/') {
            continue;
        }
        let Some(eq) = line.find('=') else {
            continue;
        };
        let key = line[..eq].trim().to_ascii_uppercase();
        let value = scalar(&line[eq + 1..]);
        match key.as_str() {
            "RECORD_BYTES" => {
                let number = value
                    .parse::<u64>()
                    .map_err(|_| format!("RECORD_BYTES is not an integer: {value}"))?;
                put(&mut parsed.record_bytes, number, "RECORD_BYTES")?;
            }
            "FILE_RECORDS" => {
                let number = value
                    .parse::<u64>()
                    .map_err(|_| format!("FILE_RECORDS is not an integer: {value}"))?;
                put(&mut parsed.file_records, number, "FILE_RECORDS")?;
            }
            "START_TIME" => {
                put(&mut parsed.start_time, value, "START_TIME")?;
            }
            "^STRUCTURE" => {
                put(&mut parsed.structure, value, "^STRUCTURE")?;
            }
            "OBJECT" => {
                if value.eq_ignore_ascii_case("COLUMN") {
                    if current.is_some() {
                        return Err("OBJECT=COLUMN nests inside a column".to_string());
                    }
                    current = Some(ColumnDraft {
                        name: None,
                        number: None,
                        data_type: None,
                        start_byte: None,
                        bytes: None,
                        items: None,
                        item_bytes: None,
                        unit: None,
                        missing: None,
                    });
                }
            }
            "END_OBJECT" => {
                if value.eq_ignore_ascii_case("COLUMN") {
                    let draft = current
                        .take()
                        .ok_or_else(|| "END_OBJECT=COLUMN without OBJECT=COLUMN".to_string())?;
                    parsed.columns.push(finish_column(draft)?);
                }
            }
            _ if current.is_some() => {
                let draft = current.as_mut().unwrap();
                match key.as_str() {
                    "NAME" => put(&mut draft.name, value, "NAME")?,
                    "COLUMN_NUMBER" => {
                        let number = value
                            .parse::<u64>()
                            .map_err(|_| format!("COLUMN_NUMBER is not an integer: {value}"))?;
                        put(&mut draft.number, number, "COLUMN_NUMBER")?;
                    }
                    "DATA_TYPE" => put(
                        &mut draft.data_type,
                        value.to_ascii_uppercase(),
                        "DATA_TYPE",
                    )?,
                    "START_BYTE" => {
                        let number = value
                            .parse::<u64>()
                            .map_err(|_| format!("START_BYTE is not an integer: {value}"))?;
                        put(&mut draft.start_byte, number, "START_BYTE")?;
                    }
                    "BYTES" => {
                        let number = value
                            .parse::<u64>()
                            .map_err(|_| format!("BYTES is not an integer: {value}"))?;
                        put(&mut draft.bytes, number, "BYTES")?;
                    }
                    "ITEMS" => {
                        let number = value
                            .parse::<u64>()
                            .map_err(|_| format!("ITEMS is not an integer: {value}"))?;
                        put(&mut draft.items, number, "ITEMS")?;
                    }
                    "ITEM_BYTES" => {
                        let number = value
                            .parse::<u64>()
                            .map_err(|_| format!("ITEM_BYTES is not an integer: {value}"))?;
                        put(&mut draft.item_bytes, number, "ITEM_BYTES")?;
                    }
                    "UNIT" if !value.is_empty() => put(&mut draft.unit, value, "UNIT")?,
                    "MISSING_CONSTANT" => put(
                        &mut draft.missing,
                        parse_missing(&value)?,
                        "MISSING_CONSTANT",
                    )?,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    if current.is_some() {
        return Err("OBJECT=COLUMN without END_OBJECT=COLUMN".to_string());
    }
    Ok(parsed)
}

fn parse_missing(value: &str) -> Result<Missing, String> {
    if let Ok(number) = value.parse::<i128>() {
        return Ok(Missing::Int(number));
    }
    if let Ok(number) = value.parse::<f64>() {
        return Ok(Missing::Real(number));
    }
    Ok(Missing::Text(value.to_string()))
}

fn parse_label(text: &str) -> Result<Label, String> {
    let parsed = parse_objects(text)?;
    let record_bytes = parsed
        .record_bytes
        .ok_or_else(|| "RECORD_BYTES absent".to_string())?;
    if record_bytes == 0 {
        return Err("RECORD_BYTES is zero".to_string());
    }
    let file_records = parsed
        .file_records
        .ok_or_else(|| "FILE_RECORDS absent".to_string())?;
    if file_records == 0 {
        return Err("FILE_RECORDS is zero".to_string());
    }
    let mut columns = parsed.columns;
    validate_columns(&mut columns, record_bytes)?;
    Ok(Label {
        record_bytes,
        file_records,
        start_time: parsed.start_time,
        structure: parsed.structure,
        columns,
    })
}

fn finish_column(draft: ColumnDraft) -> Result<Column, String> {
    let name = draft
        .name
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "column without NAME".to_string())?;
    let number = draft
        .number
        .ok_or_else(|| format!("column {name} without COLUMN_NUMBER"))?;
    if number == 0 {
        return Err(format!("column {name} has COLUMN_NUMBER zero"));
    }
    let data_type = draft
        .data_type
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("column {name} without DATA_TYPE"))?;
    let start_byte = draft
        .start_byte
        .ok_or_else(|| format!("column {name} without START_BYTE"))?;
    if start_byte == 0 {
        return Err(format!("column {name} has START_BYTE zero"));
    }
    let bytes = draft
        .bytes
        .ok_or_else(|| format!("column {name} without BYTES"))?;
    if bytes == 0 {
        return Err(format!("column {name} has BYTES zero"));
    }
    let items = draft.items.unwrap_or(1);
    if items == 0 {
        return Err(format!("column {name} has ITEMS zero"));
    }
    let item_bytes = match draft.item_bytes {
        Some(value) => value,
        None if items == 1 => bytes,
        None => {
            return Err(format!(
                "column {name} has ITEMS {items} without ITEM_BYTES"
            ));
        }
    };
    if item_bytes == 0 {
        return Err(format!("column {name} has ITEM_BYTES zero"));
    }
    match items.checked_mul(item_bytes) {
        Some(product) if product == bytes => {}
        _ => {
            return Err(format!(
                "column {name}: ITEMS {items} x ITEM_BYTES {item_bytes} != BYTES {bytes}"
            ));
        }
    }
    Ok(Column {
        name,
        number,
        data_type,
        start_byte,
        bytes,
        items,
        item_bytes,
        unit: draft.unit,
        missing: draft.missing,
    })
}

fn validate_columns(columns: &mut Vec<Column>, record_bytes: u64) -> Result<(), String> {
    columns.sort_by_key(|column| column.number);
    for pair in columns.windows(2) {
        if pair[0].number == pair[1].number {
            return Err(format!("COLUMN_NUMBER {} appears twice", pair[0].number));
        }
        if pair[0].name.eq_ignore_ascii_case(&pair[1].name) {
            return Err(format!("column NAME {} appears twice", pair[0].name));
        }
    }
    let mut previous_end = 0u64;
    for column in columns {
        match column.data_type.as_str() {
            "ASCII_INTEGER"
            | "ASCII_REAL"
            | "CHARACTER"
            | "IEEE_REAL"
            | "UNSIGNED_INTEGER"
            | "SIGNED_INTEGER"
            | "LSB_INTEGER"
            | "LSB_UNSIGNED_INTEGER" => {}
            other => {
                return Err(format!(
                    "column {} carries unsupported DATA_TYPE {other}",
                    column.name
                ));
            }
        }
        let end = column
            .start_byte
            .checked_add(column.bytes)
            .and_then(|value| value.checked_sub(1))
            .ok_or_else(|| format!("column {} bounds overflow", column.name))?;
        if end > record_bytes {
            return Err(format!(
                "column {} leaves record: START_BYTE {} BYTES {} RECORD_BYTES {}",
                column.name, column.start_byte, column.bytes, record_bytes
            ));
        }
        if column.start_byte <= previous_end {
            return Err(format!(
                "column {} starts at {} inside the previous column ending at {}",
                column.name, column.start_byte, previous_end
            ));
        }
        previous_end = end;
    }
    Ok(())
}

fn column_key(column: &Column) -> String {
    match column.unit.as_deref() {
        Some(unit) => format!("{}[{}]", column.name, unit),
        None => column.name.clone(),
    }
}

fn blank_part() -> Part {
    Part {
        text: String::new(),
        json: String::new(),
        number: false,
    }
}

fn integer_text(number: i128, missing: Option<&Missing>) -> Part {
    let absent = match missing {
        Some(Missing::Int(value)) => *value == number,
        Some(Missing::Real(value)) => *value == number as f64,
        _ => false,
    };
    if absent {
        return blank_part();
    }
    let text = number.to_string();
    Part {
        text: text.clone(),
        json: text,
        number: true,
    }
}

fn real_text(value: f64, missing: Option<&Missing>) -> Part {
    if !value.is_finite() {
        return blank_part();
    }
    let absent = match missing {
        Some(Missing::Real(constant)) => *constant == value,
        Some(Missing::Int(constant)) => *constant as f64 == value,
        _ => false,
    };
    if absent {
        return blank_part();
    }
    let text = if value == 0.0 {
        "0".to_string()
    } else {
        format!("{value}")
    };
    Part {
        text: text.clone(),
        json: text,
        number: true,
    }
}

fn unsigned_le(value: &[u8]) -> u64 {
    let mut number = 0u64;
    for (index, byte) in value.iter().enumerate() {
        number |= (*byte as u64) << (index * 8);
    }
    number
}

fn unsigned_be(value: &[u8]) -> u64 {
    let mut number = 0u64;
    for byte in value {
        number = (number << 8) | *byte as u64;
    }
    number
}

fn signed_le(value: &[u8]) -> i64 {
    let number = unsigned_le(value);
    if value.len() < 8 && value.last().is_some_and(|byte| byte & 0x80 != 0) {
        return (number | (!0u64 << (value.len() * 8))) as i64;
    }
    number as i64
}

fn signed_be(value: &[u8]) -> i64 {
    let number = unsigned_be(value);
    if value.len() < 8 && value.first().is_some_and(|byte| byte & 0x80 != 0) {
        return (number | (!0u64 << (value.len() * 8))) as i64;
    }
    number as i64
}

fn decode_item(value: &[u8], column: &Column) -> Result<Part, String> {
    match column.data_type.as_str() {
        "ASCII_INTEGER" | "ASCII_REAL" | "CHARACTER" => {
            let raw = String::from_utf8_lossy(value);
            let text = raw.trim_matches(|c: char| c == '\0' || c.is_whitespace());
            if text.is_empty()
                || text.eq_ignore_ascii_case("N/A")
                || text.eq_ignore_ascii_case("NULL")
                || text.eq_ignore_ascii_case("UNK")
                || text.eq_ignore_ascii_case("UNKNOWN")
                || text.eq_ignore_ascii_case("NAN")
            {
                return Ok(blank_part());
            }
            if let Some(Missing::Text(constant)) = &column.missing
                && text.eq_ignore_ascii_case(constant)
            {
                return Ok(blank_part());
            }
            let parsed = text.parse::<f64>().ok().filter(|value| value.is_finite());
            if let Some(value) = parsed {
                let absent = match &column.missing {
                    Some(Missing::Real(constant)) => *constant == value,
                    Some(Missing::Int(constant)) => *constant as f64 == value,
                    _ => false,
                };
                if absent {
                    return Ok(blank_part());
                }
            }
            let number = column.data_type != "CHARACTER" && parsed.is_some();
            let json = if number {
                match parsed {
                    Some(0.0) => "0".to_string(),
                    Some(value) => format!("{value}"),
                    None => String::new(),
                }
            } else {
                String::new()
            };
            Ok(Part {
                text: text.to_string(),
                json,
                number,
            })
        }
        "IEEE_REAL" => {
            let real = match value.len() {
                4 => {
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(value);
                    f32::from_le_bytes(bytes) as f64
                }
                8 => {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(value);
                    f64::from_le_bytes(bytes)
                }
                other => {
                    return Err(format!("IEEE_REAL carries {other} bytes, not 4 or 8"));
                }
            };
            Ok(real_text(real, column.missing.as_ref()))
        }
        "LSB_INTEGER" | "SIGNED_INTEGER" => {
            if value.is_empty() || value.len() > 8 {
                return Err(format!(
                    "{} carries {} bytes, not 1..8",
                    column.data_type,
                    value.len()
                ));
            }
            let number = if column.data_type == "LSB_INTEGER" {
                signed_le(value)
            } else {
                signed_be(value)
            };
            Ok(integer_text(number as i128, column.missing.as_ref()))
        }
        "LSB_UNSIGNED_INTEGER" | "UNSIGNED_INTEGER" => {
            if value.is_empty() || value.len() > 8 {
                return Err(format!(
                    "{} carries {} bytes, not 1..8",
                    column.data_type,
                    value.len()
                ));
            }
            let number = if column.data_type == "LSB_UNSIGNED_INTEGER" {
                unsigned_le(value)
            } else {
                unsigned_be(value)
            };
            Ok(integer_text(number as i128, column.missing.as_ref()))
        }
        other => Err(format!("unsupported DATA_TYPE {other}")),
    }
}

impl Cell {
    fn text(&self) -> String {
        if self.parts.len() == 1 {
            return self.parts[0].text.clone();
        }
        self.parts
            .iter()
            .map(|part| part.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn json(&self) -> String {
        if self.parts.len() == 1 {
            let part = &self.parts[0];
            if part.number {
                return part.json.clone();
            }
            return json_field(&part.text);
        }
        let body = self
            .parts
            .iter()
            .map(|part| {
                if part.number {
                    part.json.clone()
                } else {
                    json_field(&part.text)
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("[{body}]")
    }

    fn number_at(&self, index: usize) -> Option<f64> {
        let part = self.parts.get(index)?;
        if !part.number {
            return None;
        }
        part.text
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
    }
}

fn csv_field(text: &str) -> String {
    if text.contains([',', '"', '\n', '\r']) {
        return format!("\"{}\"", text.replace('"', "\"\""));
    }
    text.to_string()
}

fn json_field(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn column_index(columns: &[Column], name: &str) -> Option<usize> {
    columns
        .iter()
        .position(|column| column.name.eq_ignore_ascii_case(name))
}

fn epoch_text(value: f64) -> String {
    format!("{value:.6}")
}

fn record_time(cells: &[Cell], columns: &[Column]) -> (String, &'static str) {
    if let Some(index) = column_index(columns, "TRANSMIT_TIME")
        && let (Some(seconds), Some(fraction)) =
            (cells[index].number_at(0), cells[index].number_at(1))
    {
        return (
            epoch_text(seconds + fraction / FRACTION_DENOMINATOR),
            "TRANSMIT_TIME_TDT_J2000",
        );
    }
    if let Some(index) = column_index(columns, "MET_SECONDS")
        && let Some(seconds) = cells[index].number_at(0)
    {
        let (fraction, source) = match column_index(columns, "SUBSECONDS") {
            Some(sub) => match cells[sub].number_at(0) {
                Some(value) => (value / FRACTION_DENOMINATOR, "MET_SECONDS+SUBSECONDS"),
                None => (0.0, "MET_SECONDS"),
            },
            None => (0.0, "MET_SECONDS"),
        };
        return (epoch_text(seconds + fraction), source);
    }
    if let Some(index) = column_index(columns, "TIME") {
        return (cells[index].text(), "TIME");
    }
    (String::new(), "absent")
}

fn render(
    label: &Label,
    prefix: &[u8],
    source: &str,
    format: Format,
) -> Result<(String, String), String> {
    if !(prefix.len() as u64).is_multiple_of(label.record_bytes) {
        return Err(format!(
            "prefix {} bytes is not a multiple of RECORD_BYTES {}",
            prefix.len(),
            label.record_bytes
        ));
    }
    let records = prefix.len() as u64 / label.record_bytes;
    if records == 0 {
        return Err("prefix carries no record".to_string());
    }
    let time_index = column_index(&label.columns, "TIME");
    let digest = sha256_hex(prefix);
    let mut out = String::new();
    if format == Format::Csv {
        let mut header = vec!["time".to_string(), "record".to_string()];
        for (index, column) in label.columns.iter().enumerate() {
            if Some(index) != time_index {
                header.push(csv_field(&column_key(column)));
            }
        }
        header.push("source".to_string());
        header.push("sha256".to_string());
        out.push_str(&header.join(","));
        out.push('\n');
    }
    let mut time_source = "absent".to_string();
    for index in 0..records as usize {
        let start = index * label.record_bytes as usize;
        let end = start + label.record_bytes as usize;
        let record = &prefix[start..end];
        let mut cells = Vec::with_capacity(label.columns.len());
        for column in &label.columns {
            let base = (column.start_byte - 1) as usize;
            let mut parts = Vec::with_capacity(column.items as usize);
            for item in 0..column.items {
                let from = base + item as usize * column.item_bytes as usize;
                let to = from + column.item_bytes as usize;
                parts.push(decode_item(&record[from..to], column)?);
            }
            cells.push(Cell { parts });
        }
        let (time, source_name) = record_time(&cells, &label.columns);
        time_source = source_name.to_string();
        match format {
            Format::Csv => {
                let mut fields = vec![csv_field(&time), (index + 1).to_string()];
                for (position, cell) in cells.iter().enumerate() {
                    if Some(position) != time_index {
                        fields.push(csv_field(&cell.text()));
                    }
                }
                fields.push(csv_field(source));
                fields.push(digest.clone());
                out.push_str(&fields.join(","));
                out.push('\n');
            }
            Format::Jsonl => {
                let mut line = String::new();
                line.push('{');
                let _ = write!(
                    line,
                    "\"time\":{},\"record\":{}",
                    json_field(&time),
                    index + 1
                );
                for (position, column) in label.columns.iter().enumerate() {
                    if Some(position) == time_index {
                        continue;
                    }
                    let _ = write!(line, ",{}:", json_field(&column_key(column)));
                    line.push_str(&cells[position].json());
                }
                let _ = write!(
                    line,
                    ",{}:{},{}:{}}}",
                    json_field("source"),
                    json_field(source),
                    json_field("sha256"),
                    json_field(&digest)
                );
                out.push_str(&line);
                out.push('\n');
            }
        }
    }
    Ok((out, time_source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;
    use std::net::TcpListener;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};

    const LABEL: &str = "PDS_VERSION_ID = \"PDS3\"\n\
RECORD_BYTES = 256\n\
FILE_RECORDS = 2\n\
START_TIME = 2012-10-11T12:50:01.655\n\
OBJECT = COLUMN\n\
  NAME = \"TIME\"\n\
  COLUMN_NUMBER = 1\n\
  DATA_TYPE = ASCII_REAL\n\
  START_BYTE = 1\n\
  BYTES = 12\n\
END_OBJECT = COLUMN\n\
OBJECT = COLUMN\n\
  NAME = \"HEIGHT\"\n\
  COLUMN_NUMBER = 2\n\
  DATA_TYPE = IEEE_REAL\n\
  START_BYTE = 13\n\
  BYTES = 4\n\
  UNIT = \"m\"\n\
END_OBJECT = COLUMN\n\
OBJECT = COLUMN\n\
  NAME = \"FLAG\"\n\
  COLUMN_NUMBER = 3\n\
  DATA_TYPE = ASCII_INTEGER\n\
  START_BYTE = 17\n\
  BYTES = 4\n\
END_OBJECT = COLUMN\n\
END\n";

    const FMT_LABEL: &str = "PDS_VERSION_ID = \"PDS3\"\n\
RECORD_BYTES = 256\n\
FILE_RECORDS = 2\n\
START_TIME = 2012-10-11T12:50:01.655\n\
^STRUCTURE = \"TEST.FMT\"\n\
END\n";

    const FMT: &str = "OBJECT = COLUMN\n\
 COLUMN_NUMBER = 1\n\
 NAME = TRANSMIT_TIME\n\
 DATA_TYPE = LSB_UNSIGNED_INTEGER\n\
 START_BYTE = 1\n\
 BYTES = 8\n\
 ITEMS = 2\n\
 ITEM_BYTES = 4\n\
END_OBJECT = COLUMN\n\
OBJECT = COLUMN\n\
 COLUMN_NUMBER = 2\n\
 NAME = LASER_ENERGY\n\
 DATA_TYPE = LSB_INTEGER\n\
 START_BYTE = 9\n\
 BYTES = 4\n\
 UNIT = 'NANOJOULES'\n\
 MISSING_CONSTANT = -1\n\
END_OBJECT = COLUMN\n\
OBJECT = COLUMN\n\
 COLUMN_NUMBER = 3\n\
 NAME = FLAG\n\
 DATA_TYPE = CHARACTER\n\
 START_BYTE = 13\n\
 BYTES = 4\n\
END_OBJECT = COLUMN\n\
END\n";

    fn record(time: &str, height: Option<f32>, flag: &str) -> Vec<u8> {
        let mut bytes = vec![b' '; 256];
        let time_bytes = time.as_bytes();
        bytes[..time_bytes.len()].copy_from_slice(time_bytes);
        if let Some(height) = height {
            bytes[12..16].copy_from_slice(&height.to_le_bytes());
        }
        let flag_bytes = flag.as_bytes();
        bytes[16..16 + flag_bytes.len()].copy_from_slice(flag_bytes);
        bytes
    }

    fn fmt_record(seconds: u32, fraction: u32, energy: i32, flag: &str) -> Vec<u8> {
        let mut bytes = vec![b' '; 256];
        bytes[0..4].copy_from_slice(&seconds.to_le_bytes());
        bytes[4..8].copy_from_slice(&fraction.to_le_bytes());
        bytes[8..12].copy_from_slice(&energy.to_le_bytes());
        let flag_bytes = flag.as_bytes();
        bytes[12..12 + flag_bytes.len()].copy_from_slice(flag_bytes);
        bytes
    }

    fn temp_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("lola_rdr_pilot_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn fixture(tag: &str) -> (PathBuf, PathBuf, Vec<u8>) {
        let dir = temp_dir();
        let label = dir.join(format!("{tag}.lbl"));
        let dat = dir.join(format!("{tag}.dat"));
        let mut data = record("1000.5", Some(123.5), "7");
        data.extend(record("1001.25", Some(f32::NAN), ""));
        fs::write(&label, LABEL).unwrap();
        fs::write(&dat, &data).unwrap();
        (label, dat, data)
    }

    fn fmt_fixture(tag: &str) -> (PathBuf, PathBuf, Vec<u8>) {
        let dir = temp_dir();
        let label = dir.join(format!("{tag}.lbl"));
        let dat = dir.join(format!("{tag}.dat"));
        let fmt = dir.join("test.fmt");
        let mut data = fmt_record(1000, 2_147_483_648, 42, "AB");
        data.extend(fmt_record(1001, 0, -1, ""));
        fs::write(&label, FMT_LABEL).unwrap();
        fs::write(&fmt, FMT).unwrap();
        fs::write(&dat, &data).unwrap();
        (label, dat, data)
    }

    fn run_output(args: &[String]) -> Result<String, (i32, String)> {
        execute(args).map(|(text, _)| text)
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn csv_keeps_column_order_unit_and_empty_values() {
        let (label, dat, data) = fixture("csv");
        let out = run_output(&args(&[
            label.to_str().unwrap(),
            "--records",
            "2",
            "--format",
            "csv",
        ]))
        .unwrap();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "time,record,HEIGHT[m],FLAG,source,sha256");
        let first: Vec<&str> = lines[1].split(',').collect();
        assert_eq!(first[0], "1000.5");
        assert_eq!(first[1], "1");
        assert_eq!(first[2], "123.5");
        assert_eq!(first[3], "7");
        assert_eq!(first[4], dat.to_str().unwrap());
        assert_eq!(first[5], sha256_hex(&data));
        let second: Vec<&str> = lines[2].split(',').collect();
        assert_eq!(second[0], "1001.25");
        assert_eq!(second[1], "2");
        assert_eq!(second[2], "");
        assert_eq!(second[3], "");
    }

    #[test]
    fn jsonl_carries_numbers_and_empty_missing_values() {
        let (label, _, _) = fixture("jsonl");
        let out = run_output(&args(&[
            label.to_str().unwrap(),
            "--records",
            "2",
            "--format",
            "jsonl",
        ]))
        .unwrap();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with('{'));
        assert!(lines[0].ends_with('}'));
        assert!(lines[0].contains("\"time\":\"1000.5\""));
        assert!(lines[0].contains("\"record\":1"));
        assert!(lines[0].contains("\"HEIGHT[m]\":123.5"));
        assert!(lines[0].contains("\"FLAG\":7"));
        assert!(lines[1].contains("\"time\":\"1001.25\""));
        assert!(lines[1].contains("\"record\":2"));
        assert!(lines[1].contains("\"HEIGHT[m]\":\"\""));
        assert!(lines[1].contains("\"FLAG\":\"\""));
    }

    #[test]
    fn fmt_supplies_columns_items_unit_and_missing_constant() {
        let (label, dat, data) = fmt_fixture("fmt");
        let out = run_output(&args(&[
            label.to_str().unwrap(),
            "--records",
            "2",
            "--format",
            "csv",
        ]))
        .unwrap();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(
            lines[0],
            "time,record,TRANSMIT_TIME,LASER_ENERGY[NANOJOULES],FLAG,source,sha256"
        );
        let first: Vec<&str> = lines[1].split(',').collect();
        assert_eq!(first[0], "1000.500000");
        assert_eq!(first[1], "1");
        assert_eq!(first[2], "1000 2147483648");
        assert_eq!(first[3], "42");
        assert_eq!(first[4], "AB");
        assert_eq!(first[5], dat.to_str().unwrap());
        assert_eq!(first[6], sha256_hex(&data));
        let second: Vec<&str> = lines[2].split(',').collect();
        assert_eq!(second[0], "1001.000000");
        assert_eq!(second[2], "1001 0");
        assert_eq!(second[3], "");
        assert_eq!(second[4], "");
    }

    #[test]
    fn fmt_jsonl_keeps_items_as_array() {
        let (label, _, _) = fmt_fixture("fmt-jsonl");
        let out = run_output(&args(&[
            label.to_str().unwrap(),
            "--records",
            "2",
            "--format",
            "jsonl",
        ]))
        .unwrap();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("\"time\":\"1000.500000\""));
        assert!(lines[0].contains("\"TRANSMIT_TIME\":[1000,2147483648]"));
        assert!(lines[0].contains("\"LASER_ENERGY[NANOJOULES]\":42"));
        assert!(lines[0].contains("\"FLAG\":\"AB\""));
        assert!(lines[1].contains("\"LASER_ENERGY[NANOJOULES]\":\"\""));
        assert!(lines[1].contains("\"FLAG\":\"\""));
    }

    #[test]
    fn records_one_and_two_bound_the_prefix() {
        let (label, _, _) = fixture("records");
        let one = run_output(&args(&[
            label.to_str().unwrap(),
            "--records",
            "1",
            "--format",
            "csv",
        ]))
        .unwrap();
        assert_eq!(one.lines().count(), 2);
        let two = run_output(&args(&[
            label.to_str().unwrap(),
            "--records",
            "2",
            "--format",
            "csv",
        ]))
        .unwrap();
        assert_eq!(two.lines().count(), 3);
    }

    fn serve(
        label_body: String,
        fmt_body: String,
        dat: Vec<u8>,
        expected: usize,
    ) -> (String, Arc<Mutex<Vec<String>>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let record = seen.clone();
        let _handle = thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();
            let start = Instant::now();
            let mut served = 0;
            while served < expected && start.elapsed() < Duration::from_secs(10) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut request = Vec::new();
                        let mut chunk = [0u8; 1024];
                        let mut head_end = 0;
                        loop {
                            let read = stream.read(&mut chunk).unwrap();
                            if read == 0 {
                                break;
                            }
                            request.extend_from_slice(&chunk[..read]);
                            if let Some(at) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                                head_end = at + 4;
                                break;
                            }
                        }
                        let head = String::from_utf8_lossy(&request[..head_end]).to_string();
                        let path = head.split_whitespace().nth(1).unwrap_or("/").to_string();
                        let range = head.lines().find_map(|line| {
                            if line.to_ascii_lowercase().starts_with("range:") {
                                Some(line[6..].trim().to_string())
                            } else {
                                None
                            }
                        });
                        if path.ends_with(".lbl") {
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                                label_body.len()
                            );
                            stream.write_all(response.as_bytes()).unwrap();
                            stream.write_all(label_body.as_bytes()).unwrap();
                        } else if path.ends_with(".fmt") {
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                                fmt_body.len()
                            );
                            stream.write_all(response.as_bytes()).unwrap();
                            stream.write_all(fmt_body.as_bytes()).unwrap();
                        } else {
                            let range = range.unwrap_or_default();
                            record.lock().unwrap().push(range.clone());
                            let spec = range.trim_start_matches("bytes=");
                            let mut parts = spec.split('-');
                            let from: usize = parts.next().unwrap().parse().unwrap();
                            let to: usize = parts.next().unwrap().parse().unwrap();
                            let payload = &dat[from..=to];
                            let response = format!(
                                "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nContent-Range: bytes {}-{}/{}\r\nConnection: close\r\n\r\n",
                                payload.len(),
                                from,
                                to,
                                dat.len()
                            );
                            stream.write_all(response.as_bytes()).unwrap();
                            stream.write_all(payload).unwrap();
                        }
                        served += 1;
                    }
                    Err(_) => thread::sleep(Duration::from_millis(5)),
                }
            }
        });
        (format!("http://{addr}/test.lbl"), seen)
    }

    #[test]
    fn range_stub_fetches_fmt_and_only_the_requested_prefix() {
        let (_, _, data) = fmt_fixture("range");
        let (url, seen) = serve(FMT_LABEL.to_string(), FMT.to_string(), data.clone(), 3);
        let out = run_output(&args(&[&url, "--records", "1", "--format", "csv"])).unwrap();
        assert_eq!(seen.lock().unwrap().as_slice(), ["bytes=0-255"]);
        assert!(out.starts_with("time,record,TRANSMIT_TIME,LASER_ENERGY[NANOJOULES],FLAG"));
        assert!(out.contains(&sha256_hex(&data[..256])));
        assert!(out.contains("1000.500000"));
    }

    #[test]
    fn missing_structure_is_pending() {
        let dir = temp_dir();
        let label = dir.join("missing-fmt.lbl");
        fs::write(
            &label,
            "RECORD_BYTES = 256\nFILE_RECORDS = 2\n^STRUCTURE = \"ABSENT.FMT\"\n",
        )
        .unwrap();
        let error = run_output(&args(&[label.to_str().unwrap()])).unwrap_err();
        assert_eq!(error.0, 3);
        assert!(error.1.contains("pending"));
        assert!(error.1.contains("ABSENT.FMT"));
    }

    #[test]
    fn overlapping_structure_columns_are_pending() {
        let dir = temp_dir();
        let label = dir.join("overlap.lbl");
        let fmt = dir.join("overlap.fmt");
        fs::write(
            &label,
            "RECORD_BYTES = 256\nFILE_RECORDS = 2\n^STRUCTURE = \"OVERLAP.FMT\"\n",
        )
        .unwrap();
        fs::write(
            &fmt,
            "OBJECT = COLUMN\n COLUMN_NUMBER = 1\n NAME = A\n DATA_TYPE = LSB_INTEGER\n START_BYTE = 1\n BYTES = 4\nEND_OBJECT = COLUMN\n\
OBJECT = COLUMN\n COLUMN_NUMBER = 2\n NAME = B\n DATA_TYPE = LSB_INTEGER\n START_BYTE = 3\n BYTES = 4\nEND_OBJECT = COLUMN\n",
        )
        .unwrap();
        let error = run_output(&args(&[label.to_str().unwrap()])).unwrap_err();
        assert_eq!(error.0, 3);
        assert!(error.1.contains("pending"));
        assert!(error.1.contains("starts at 3"));
    }

    #[test]
    fn missing_record_bytes_is_pending() {
        let dir = temp_dir();
        let label = dir.join("missing.lbl");
        fs::write(&label, "FILE_RECORDS = 2\n").unwrap();
        let error = run_output(&args(&[label.to_str().unwrap()])).unwrap_err();
        assert_eq!(error.0, 3);
        assert!(error.1.contains("pending"));
    }

    #[test]
    fn zero_record_bytes_is_pending() {
        let dir = temp_dir();
        let label = dir.join("zero.lbl");
        fs::write(&label, "RECORD_BYTES = 0\nFILE_RECORDS = 2\n").unwrap();
        let error = run_output(&args(&[label.to_str().unwrap()])).unwrap_err();
        assert_eq!(error.0, 3);
        assert!(error.1.contains("RECORD_BYTES"));
    }

    #[test]
    fn record_bounds_are_usage_errors() {
        let (label, _, _) = fixture("bounds");
        let zero = run_output(&args(&[label.to_str().unwrap(), "--records", "0"])).unwrap_err();
        assert_eq!(zero.0, 2);
        let over = run_output(&args(&[label.to_str().unwrap(), "--records", "10001"])).unwrap_err();
        assert_eq!(over.0, 2);
        let negative =
            run_output(&args(&[label.to_str().unwrap(), "--records", "-1"])).unwrap_err();
        assert_eq!(negative.0, 2);
    }

    #[test]
    fn start_time_is_not_invented_per_record() {
        let dir = temp_dir();
        let label = dir.join("anchor.lbl");
        let dat = dir.join("anchor.dat");
        fs::write(
            &label,
            "RECORD_BYTES = 256\nFILE_RECORDS = 1\nSTART_TIME = 2012-10-11T12:50:01.655\n",
        )
        .unwrap();
        fs::write(&dat, record("", None, "")).unwrap();
        let out = run_output(&args(&[
            label.to_str().unwrap(),
            "--records",
            "1",
            "--format",
            "csv",
        ]))
        .unwrap();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines[0], "time,record,source,sha256");
        let fields: Vec<&str> = lines[1].split(',').collect();
        assert_eq!(fields[0], "");
        assert_eq!(fields[1], "1");
    }

    #[test]
    fn dat_override_wins_over_derived_path() {
        let (label, _, data) = fixture("override");
        let dir = temp_dir();
        let other = dir.join("other.dat");
        fs::write(&other, &data).unwrap();
        let out = run_output(&args(&[
            label.to_str().unwrap(),
            "--dat",
            other.to_str().unwrap(),
            "--records",
            "1",
            "--format",
            "csv",
        ]))
        .unwrap();
        assert!(out.contains(other.to_str().unwrap()));
    }
}
