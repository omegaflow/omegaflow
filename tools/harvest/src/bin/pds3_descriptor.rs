use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

use omegaflow::archivar::pds3_table::{odl_kv, parse_label};

#[derive(Clone, Debug, Default)]
pub struct Column {
    pub name: String,
    pub start_byte: usize,
    pub bytes: usize,
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub row_bytes: Option<usize>,
    pub columns: Vec<Column>,
}

fn parse_fmt(text: &str) -> Result<Layout, String> {
    let mut layout = Layout {
        row_bytes: None,
        columns: Vec::new(),
    };
    let mut current: Option<Column> = None;
    for (key, value) in odl_kv(text) {
        let value = value.trim().to_string();
        match key.as_str() {
            "ROW_BYTES" => {
                layout.row_bytes = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| format!("ROW_BYTES {value} carries no number"))?,
                );
            }
            "OBJECT" if value == "COLUMN" => current = Some(Column::default()),
            "END_OBJECT" => {
                if let Some(column) = current.take() {
                    layout.columns.push(column);
                }
            }
            "NAME" => {
                if let Some(column) = current.as_mut() {
                    column.name = value;
                }
            }
            "START_BYTE" => {
                if let Some(column) = current.as_mut() {
                    column.start_byte = value
                        .parse::<usize>()
                        .map_err(|_| format!("START_BYTE {value} carries no number"))?;
                }
            }
            "BYTES" => {
                if let Some(column) = current.as_mut() {
                    column.bytes = value
                        .parse::<usize>()
                        .map_err(|_| format!("BYTES {value} carries no number"))?;
                }
            }
            _ => {}
        }
    }
    Ok(layout)
}

fn fetch(text: &str) -> Result<Vec<u8>, String> {
    if text.starts_with("http://") || text.starts_with("https://") {
        let mut last = String::new();
        for attempt in 0..2 {
            let out = Command::new("curl")
                .args(["-sSfL", "--max-time", "120", text])
                .output()
                .map_err(|e| format!("curl for {text} returned void: {e}"))?;
            if out.status.success() {
                return Ok(out.stdout);
            }
            last = format!("curl reported {}", out.status);
            if attempt == 0 {
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
        Err(format!("{text}: {last}"))
    } else {
        fs::read(text).map_err(|e| format!("read {text} returned void: {e}"))
    }
}

fn fetch_range(
    text: &str,
    start: usize,
    len: usize,
    whole: Option<&[u8]>,
) -> Result<Vec<u8>, String> {
    if let Some(bytes) = whole {
        let end = (start + len).min(bytes.len());
        if start >= bytes.len() {
            return Err(format!("{start} lies past the {} bytes read", bytes.len()));
        }
        return Ok(bytes[start..end].to_vec());
    }
    let out = Command::new("curl")
        .args([
            "-sSfL",
            "--max-time",
            "120",
            "-r",
            &format!("{}-{}", start, start + len - 1),
            text,
        ])
        .output()
        .map_err(|e| format!("curl range for {text} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{text}: range {start}-{} returned {}",
            start + len - 1,
            out.status
        ));
    }
    Ok(out.stdout)
}

fn length(text: &str, whole: Option<&[u8]>) -> Result<usize, String> {
    if let Some(bytes) = whole {
        return Ok(bytes.len());
    }
    let out = Command::new("curl")
        .args(["-sSIL", "--max-time", "60", text])
        .output()
        .map_err(|e| format!("curl head for {text} returned void: {e}"))?;
    let head = String::from_utf8_lossy(&out.stdout).to_lowercase();
    if let Some(total) = head
        .lines()
        .filter_map(|line| line.strip_prefix("content-length:"))
        .filter_map(|value| value.trim().parse::<usize>().ok())
        .next_back()
    {
        return Ok(total);
    }
    let probe = Command::new("curl")
        .args(["-sSfL", "--max-time", "60", "-r", "0-0", "-D", "-", text])
        .output()
        .map_err(|e| format!("curl range probe for {text} returned void: {e}"))?;
    let headers = String::from_utf8_lossy(&probe.stdout).to_lowercase();
    headers
        .lines()
        .filter_map(|line| line.strip_prefix("content-range:"))
        .filter_map(|value| {
            value
                .rsplit_once('/')
                .map(|(_, total)| total.trim().to_string())
        })
        .filter_map(|total| total.parse::<usize>().ok())
        .next_back()
        .ok_or_else(|| format!("{text}: neither content-length nor content-range names a size"))
}

fn field(record: &[u8], column: &Column) -> Result<f64, String> {
    let start = column
        .start_byte
        .checked_sub(1)
        .ok_or_else(|| format!("column {} names start byte 0", column.name))?;
    let end = start + column.bytes;
    let slice = record.get(start..end).ok_or_else(|| {
        format!(
            "record carries {} bytes, the column wants {end}",
            record.len()
        )
    })?;
    let text = String::from_utf8_lossy(slice).trim().to_string();
    text.parse::<f64>()
        .map_err(|_| format!("column {} holds {:?}, not a number", column.name, text))
}

#[derive(Clone, Debug)]
pub struct Report {
    pub row_bytes: usize,
    pub rows: usize,
    pub key: String,
    pub key_start: usize,
    pub key_bytes: usize,
    pub first: f64,
    pub last: f64,
}

pub fn describe(
    label_text: &str,
    fmt_text: Option<&str>,
    first_record: &[u8],
    last_record: &[u8],
    total: usize,
) -> Result<Report, String> {
    let meta = parse_label(label_text).ok_or_else(|| "the label carries no TABLE".to_string())?;
    let layout = match fmt_text {
        Some(text) => parse_fmt(text)?,
        None => {
            let columns: Vec<Column> = meta
                .columns
                .iter()
                .map(|c| Column {
                    name: c.name.clone(),
                    start_byte: c.start_byte,
                    bytes: c.bytes,
                })
                .collect();
            if columns.is_empty() {
                return Err("neither the label nor a format file carries a column".to_string());
            }
            Layout {
                row_bytes: None,
                columns,
            }
        }
    };
    let kv = odl_kv(label_text);
    let get = |name: &str| -> Option<String> {
        kv.iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.trim().trim_matches('"').to_string())
    };
    let record_bytes = meta
        .record_bytes
        .or(layout.row_bytes)
        .ok_or_else(|| "neither label nor format file names a record length".to_string())?;
    if let Some(row_bytes) = layout.row_bytes
        && row_bytes != record_bytes
    {
        return Err(format!(
            "label says {record_bytes} bytes per record, the format file says {row_bytes}"
        ));
    }
    let rows = meta
        .rows
        .or(meta.file_records)
        .ok_or_else(|| "the label names no row count".to_string())?;
    let declared = record_bytes * rows;
    if declared != total {
        let delta = total as i64 - declared as i64;
        let extra = if delta % record_bytes as i64 == 0 {
            format!(", that is {} whole record(s)", delta / record_bytes as i64)
        } else {
            String::new()
        };
        return Err(format!(
            "{rows} rows of {record_bytes} bytes are {declared}, the file holds {total} ({delta:+} bytes{extra})"
        ));
    }
    let Some(key_name) = get("PRIMARY_KEY") else {
        return Ok(Report {
            row_bytes: record_bytes,
            rows,
            key: String::new(),
            key_start: 0,
            key_bytes: 0,
            first: f64::NAN,
            last: f64::NAN,
        });
    };
    let column = layout
        .columns
        .iter()
        .find(|c| c.name == key_name)
        .ok_or_else(|| format!("the format file carries no column {key_name}"))?;
    let first = field(first_record, column)?;
    let last = field(last_record, column)?;
    let start = get("START_PRIMARY_KEY")
        .and_then(|v| v.parse::<f64>().ok())
        .ok_or_else(|| "the label names no start of the primary key".to_string())?;
    let stop = get("STOP_PRIMARY_KEY")
        .and_then(|v| v.parse::<f64>().ok())
        .ok_or_else(|| "the label names no stop of the primary key".to_string())?;
    let tolerance = 1e-6 * stop.abs().max(start.abs()).max(1.0);
    if (first - start).abs() > tolerance {
        return Err(format!(
            "first key {first} is not the declared start {start}"
        ));
    }
    if (last - stop).abs() > tolerance {
        return Err(format!("last key {last} is not the declared stop {stop}"));
    }
    Ok(Report {
        row_bytes: record_bytes,
        rows,
        key: key_name,
        key_start: column.start_byte,
        key_bytes: column.bytes,
        first,
        last,
    })
}

#[derive(Clone, Debug)]
struct Entry {
    asset: String,
    origin: String,
}

fn entries(register: &str, format: &str) -> Result<Vec<Entry>, String> {
    let text =
        fs::read_to_string(register).map_err(|e| format!("read {register} returned void: {e}"))?;
    let mut blocks: Vec<BTreeMap<String, String>> = Vec::new();
    let mut current: BTreeMap<String, String> = BTreeMap::new();
    let flush = |current: &mut BTreeMap<String, String>,
                 blocks: &mut Vec<BTreeMap<String, String>>| {
        if !current.is_empty() {
            blocks.push(std::mem::take(current));
        }
    };
    for line in text.lines() {
        if line.trim().is_empty() {
            flush(&mut current, &mut blocks);
            continue;
        }
        let mut parts = line.splitn(2, char::is_whitespace);
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").trim();
        if !key.is_empty() {
            current.insert(key.to_string(), value.to_string());
        }
    }
    flush(&mut current, &mut blocks);
    let mut found: Vec<Entry> = Vec::new();
    for block in blocks {
        if block.get("format").map(|f| f == format).unwrap_or(false)
            && let (Some(asset), Some(origin)) = (block.get("url"), block.get("origin"))
        {
            found.push(Entry {
                asset: asset.clone(),
                origin: origin.clone(),
            });
        }
    }
    if found.is_empty() {
        return Err(format!(
            "{register} carries no {format} entry with an origin"
        ));
    }
    Ok(found)
}

fn label_of(origin: &str) -> String {
    match origin.rfind('.') {
        Some(at) => format!("{}.lbl", &origin[..at]),
        None => format!("{origin}.lbl"),
    }
}

fn resolve_fmt(
    label_url: &str,
    structure: &str,
    cache: &mut BTreeMap<String, String>,
) -> Option<String> {
    if structure.starts_with("http") {
        return Some(structure.to_string());
    }
    let base = match label_url.rfind('/') {
        Some(at) => &label_url[..at],
        None => return None,
    };
    if let Some(hit) = cache.get(base) {
        return Some(format!("{hit}/{structure}"));
    }
    let mut dir = base.to_string();
    for _depth in 0..5 {
        for sub in ["", "/label", "/catalog", "/document"] {
            let url = format!("{dir}{sub}/{structure}");
            if fetch(&url).is_ok() {
                cache.insert(base.to_string(), format!("{dir}{sub}"));
                return Some(url);
            }
        }
        match dir.rsplit_once('/') {
            Some((parent, _)) => dir = parent.to_string(),
            None => break,
        }
    }
    None
}

fn run_register(args: &[String]) -> Result<(), String> {
    let usage = "usage: pds3_descriptor --register <phi/sources.φ> --format <name> [--limit <n>] [--out <tsv>]";
    let mut register = None;
    let mut format = None;
    let mut limit = usize::MAX;
    let mut out: Option<String> = None;
    let mut index = 0;
    while index < args.len() {
        let value = || {
            args.get(index + 1)
                .cloned()
                .ok_or_else(|| usage.to_string())
        };
        match args[index].as_str() {
            "--register" => register = Some(value()?),
            "--format" => format = Some(value()?),
            "--limit" => {
                limit = value()?
                    .parse::<usize>()
                    .map_err(|_| "limit carries no number".to_string())?
            }
            "--out" => out = Some(value()?),
            other => return Err(format!("{other}: not a flag of this mode")),
        }
        index += 2;
    }
    let register = register.ok_or_else(|| usage.to_string())?;
    let format = format.ok_or_else(|| usage.to_string())?;
    let list = entries(&register, &format)?;
    let mut cache: BTreeMap<String, String> = BTreeMap::new();
    let mut sink: Option<std::io::BufWriter<fs::File>> = match &out {
        None => None,
        Some(path) => {
            let file =
                fs::File::create(path).map_err(|e| format!("create {path} returned void: {e}"))?;
            let mut writer = std::io::BufWriter::new(file);
            writeln!(
                writer,
                "asset\trecord_bytes\trows\tkey\tkey_start\tkey_bytes\tfirst\tlast\tverdict"
            )
            .map_err(|e| format!("write {path} returned void: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("flush {path} returned void: {e}"))?;
            Some(writer)
        }
    };
    let mut gated = 0usize;
    let mut absent = 0usize;
    for entry in list.iter().take(limit) {
        let name = entry
            .asset
            .rsplit('/')
            .next()
            .unwrap_or("unnamed")
            .to_string();
        let label_url = label_of(&entry.origin);
        let verdict = match fetch(&label_url) {
            Err(reason) => {
                format!("absent\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}")
            }
            Ok(bytes) => {
                let label_text = String::from_utf8_lossy(&bytes).to_string();
                let structure = odl_kv(&label_text)
                    .iter()
                    .find(|(k, _)| k == "^STRUCTURE")
                    .map(|(_, v)| v.trim().trim_matches('"').to_string());
                let resolved = structure.and_then(|s| resolve_fmt(&label_url, &s, &mut cache));
                let loaded = match &resolved {
                    None => Ok(None),
                    Some(url) => {
                        fetch(url).map(|bytes| Some(String::from_utf8_lossy(&bytes).to_string()))
                    }
                };
                match loaded {
                    Err(reason) => {
                        format!("absent\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}")
                    }
                    Ok(fmt_text) => {
                        let rb = parse_label(&label_text).and_then(|m| m.record_bytes);
                        match rb {
                            None => format!(
                                "absent\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\tthe label names no record length"
                            ),
                            Some(rb) => match length(&entry.origin, None) {
                                Err(reason) => format!(
                                    "absent\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}"
                                ),
                                Ok(total) => {
                                    let head = fetch_range(&entry.origin, 0, rb, None);
                                    let tail = fetch_range(
                                        &entry.origin,
                                        total.saturating_sub(rb),
                                        rb,
                                        None,
                                    );
                                    match (head, tail) {
                                        (Ok(head), Ok(tail)) => match describe(
                                            &label_text,
                                            fmt_text.as_deref(),
                                            &head,
                                            &tail,
                                            total,
                                        ) {
                                            Ok(report) => render(&report),
                                            Err(reason) => {
                                                format!(
                                                    "absent\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}"
                                                )
                                            }
                                        },
                                        (Err(reason), _) | (_, Err(reason)) => {
                                            format!(
                                                "absent\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}"
                                            )
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        };
        if verdict.contains("gated") {
            gated += 1;
        } else {
            absent += 1;
        }
        let line = format!("{name}\t{verdict}");
        eprintln!("{line}");
        if let Some(writer) = sink.as_mut() {
            writeln!(writer, "{line}").map_err(|e| format!("write returned void: {e}"))?;
            writer
                .flush()
                .map_err(|e| format!("flush returned void: {e}"))?;
        }
    }
    println!(
        "gated {gated}  absent {absent}  of {} entries",
        list.len().min(limit)
    );
    Ok(())
}

fn render(report: &Report) -> String {
    if report.key.is_empty() {
        format!(
            "{}\t{}\tabsent\tabsent\tabsent\tabsent\tabsent\tgated-structure",
            report.row_bytes, report.rows
        )
    } else {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.2}\tgated",
            report.row_bytes,
            report.rows,
            report.key,
            report.key_start,
            report.key_bytes,
            report.first,
            report.last
        )
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--register") {
        return run_register(&args);
    }
    let usage = "usage: pds3_descriptor --label <path|url> --fmt <path|url> --data <path|url> [--name <asset>]";
    let mut label: Option<String> = None;
    let mut fmt_source: Option<String> = None;
    let mut data: Option<String> = None;
    let mut name = String::from("unnamed");
    let mut index = 0;
    while index < args.len() {
        let value = || {
            args.get(index + 1)
                .cloned()
                .ok_or_else(|| usage.to_string())
        };
        match args[index].as_str() {
            "--label" => label = Some(value()?),
            "--fmt" => fmt_source = Some(value()?),
            "--data" => data = Some(value()?),
            "--name" => name = value()?,
            other => return Err(format!("{other}: not a flag of this tool")),
        }
        index += 2;
    }
    let label = label.ok_or_else(|| usage.to_string())?;
    let fmt_source = fmt_source.ok_or_else(|| usage.to_string())?;
    let data = data.ok_or_else(|| usage.to_string())?;

    let label_text = String::from_utf8_lossy(&fetch(&label)?).to_string();
    let fmt_text = String::from_utf8_lossy(&fetch(&fmt_source)?).to_string();
    let meta = parse_label(&label_text).ok_or_else(|| "the label carries no TABLE".to_string())?;
    let record_bytes = meta
        .record_bytes
        .ok_or_else(|| "the label names no record length".to_string())?;

    let whole = if data.starts_with("http") {
        None
    } else {
        Some(fs::read(&data).map_err(|e| format!("read {data} returned void: {e}"))?)
    };
    let total = length(&data, whole.as_deref())?;
    let head = fetch_range(&data, 0, record_bytes, whole.as_deref())?;
    let tail = fetch_range(
        &data,
        total.saturating_sub(record_bytes),
        record_bytes,
        whole.as_deref(),
    )?;

    match describe(&label_text, Some(fmt_text.as_str()), &head, &tail, total) {
        Ok(report) => {
            println!("{name}\t{}", render(&report));
            Ok(())
        }
        Err(reason) => {
            println!("{name}\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\tabsent\t{reason}");
            Err(reason)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LABEL: &str = "PDS_VERSION_ID = PDS3\n\
RECORD_TYPE = FIXED_LENGTH\n\
FILE_RECORDS = 3\n\
RECORD_BYTES = 20\n\
^TABLE = (\"T.TAB\",1)\n\
OBJECT = TABLE\n\
  NAME = T\n\
  INTERCHANGE_FORMAT = ASCII\n\
  PRIMARY_KEY = \"TIME\"\n\
  START_PRIMARY_KEY = 10.5\n\
  STOP_PRIMARY_KEY = 12.5\n\
  ROWS = 3\n\
END_OBJECT = TABLE\nEND\n";

    const FMT: &str = "ROW_BYTES = 20\n\
COLUMNS = 2\n\
OBJECT = COLUMN\n  NAME = TIME\n  START_BYTE = 1\n  BYTES = 10\n  FORMAT = \"F10.1\"\nEND_OBJECT = COLUMN\n\
OBJECT = COLUMN\n  NAME = VALUE\n  START_BYTE = 11\n  BYTES = 10\n  FORMAT = \"F10.1\"\nEND_OBJECT = COLUMN\n";

    fn data() -> Vec<u8> {
        let mut text = String::new();
        for (time, value) in [(10.5, 1.0), (11.5, 2.0), (12.5, 3.0)] {
            text.push_str(&format!("{time:>10.1}{value:>10.1}"));
        }
        text.into_bytes()
    }

    #[test]
    fn label_and_format_file_agree_on_a_gated_table() {
        let bytes = data();
        let report =
            describe(LABEL, Some(FMT), &bytes[0..20], &bytes[40..60], bytes.len()).expect("gated");
        assert_eq!(report.row_bytes, 20);
        assert_eq!(report.rows, 3);
        assert_eq!(report.key, "TIME");
        assert_eq!(report.key_start, 1);
        assert_eq!(report.key_bytes, 10);
        assert!((report.first - 10.5).abs() < 1e-9);
        assert!((report.last - 12.5).abs() < 1e-9);
    }

    #[test]
    fn two_documents_that_disagree_on_the_record_length_are_absent() {
        let broken = FMT.replace("ROW_BYTES = 20", "ROW_BYTES = 24");
        let bytes = data();
        let reason = describe(
            LABEL,
            Some(&broken),
            &bytes[0..20],
            &bytes[40..60],
            bytes.len(),
        )
        .expect_err("must not pass");
        assert!(reason.contains("20") && reason.contains("24"), "{reason}");
    }

    #[test]
    fn a_length_that_contradicts_the_row_count_is_absent() {
        let bytes = data();
        let reason = describe(
            LABEL,
            Some(FMT),
            &bytes[0..20],
            &bytes[40..60],
            bytes.len() - 1,
        )
        .expect_err("must not pass");
        assert!(reason.contains("59"), "{reason}");
    }

    #[test]
    fn a_last_key_that_is_not_the_declared_stop_is_absent() {
        let bytes = data();
        let reason = describe(LABEL, Some(FMT), &bytes[0..20], &bytes[20..40], bytes.len())
            .expect_err("must not pass");
        assert!(
            reason.contains("11.5") && reason.contains("12.5"),
            "{reason}"
        );
    }

    #[test]
    fn a_column_that_does_not_parse_as_a_number_is_absent() {
        let bytes = data();
        let mut head = bytes[0..20].to_vec();
        head[0] = b'?';
        let reason = describe(LABEL, Some(FMT), &head, &bytes[40..60], bytes.len())
            .expect_err("must not pass");
        assert!(reason.contains("TIME"), "{reason}");
    }
}
