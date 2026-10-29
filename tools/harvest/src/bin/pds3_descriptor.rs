use std::env;
use std::fs;
use std::process::Command;

use omegaflow::archivar::pds3_table::{odl_kv, parse_label};

#[derive(Clone, Debug, Default)]
pub struct Column {
    pub name: String,
    pub start_byte: usize,
    pub bytes: usize,
    pub format: String,
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
            "FORMAT" => {
                if let Some(column) = current.as_mut() {
                    column.format = value.trim_matches('"').to_string();
                }
            }
            _ => {}
        }
    }
    Ok(layout)
}

fn fetch(text: &str) -> Result<Vec<u8>, String> {
    if text.starts_with("http://") || text.starts_with("https://") {
        let out = Command::new("curl")
            .args(["-sSfL", "--max-time", "120", text])
            .output()
            .map_err(|e| format!("curl for {text} returned void: {e}"))?;
        if !out.status.success() {
            return Err(format!("{text}: curl reported {}", out.status));
        }
        Ok(out.stdout)
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
    head.lines()
        .filter_map(|line| line.strip_prefix("content-length:"))
        .filter_map(|value| value.trim().parse::<usize>().ok())
        .next_back()
        .ok_or_else(|| format!("{text}: the head carries no content-length"))
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
    fmt_text: &str,
    first_record: &[u8],
    last_record: &[u8],
    total: usize,
) -> Result<Report, String> {
    let meta = parse_label(label_text).ok_or_else(|| "the label carries no TABLE".to_string())?;
    let layout = parse_fmt(fmt_text)?;
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
        return Err(format!(
            "{rows} rows of {record_bytes} bytes are {declared}, the file holds {total}"
        ));
    }
    let key_name =
        get("PRIMARY_KEY").ok_or_else(|| "the label names no primary key".to_string())?;
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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
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

    match describe(&label_text, &fmt_text, &head, &tail, total) {
        Ok(report) => {
            println!(
                "{name}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.2}\tgated",
                report.row_bytes,
                report.rows,
                report.key,
                report.key_start,
                report.key_bytes,
                report.first,
                report.last
            );
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
            describe(LABEL, FMT, &bytes[0..20], &bytes[40..60], bytes.len()).expect("gated");
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
        let reason = describe(LABEL, &broken, &bytes[0..20], &bytes[40..60], bytes.len())
            .expect_err("must not pass");
        assert!(reason.contains("20") && reason.contains("24"), "{reason}");
    }

    #[test]
    fn a_length_that_contradicts_the_row_count_is_absent() {
        let bytes = data();
        let reason = describe(LABEL, FMT, &bytes[0..20], &bytes[40..60], bytes.len() - 1)
            .expect_err("must not pass");
        assert!(reason.contains("59"), "{reason}");
    }

    #[test]
    fn a_last_key_that_is_not_the_declared_stop_is_absent() {
        let bytes = data();
        let reason = describe(LABEL, FMT, &bytes[0..20], &bytes[20..40], bytes.len())
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
        let reason =
            describe(LABEL, FMT, &head, &bytes[40..60], bytes.len()).expect_err("must not pass");
        assert!(reason.contains("TIME"), "{reason}");
    }
}
