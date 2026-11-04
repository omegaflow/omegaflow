use std::env;
use std::fs;
use std::process::Command;

use omegaflow::archivar::pds4::parse_label;

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

#[derive(Clone, Debug)]
pub struct Report {
    pub record_length: usize,
    pub rows: usize,
    pub offset: Option<usize>,
    pub declared_size: Option<u64>,
}

pub fn describe(label_text: &str, total: usize) -> Result<Report, String> {
    let meta = parse_label(label_text).ok_or_else(|| "the label carries no table".to_string())?;
    let record_length = meta
        .record_length
        .ok_or_else(|| "the label names no record length".to_string())?;
    let rows = meta
        .rows
        .or(meta.file_records)
        .ok_or_else(|| "the label names no row count".to_string())?;
    let declared = record_length * rows;
    if let Some(size) = meta.file_size
        && size as usize != total
    {
        return Err(format!(
            "the label declares file_size {size}, the file holds {total}"
        ));
    }
    let expected = match meta.table_offset {
        Some(offset) => offset + declared,
        None => declared,
    };
    if expected != total {
        let delta = total as i64 - expected as i64;
        let extra = if record_length > 0 && delta % record_length as i64 == 0 {
            format!(", that is {} whole record(s)", delta / record_length as i64)
        } else {
            String::new()
        };
        let where_from = match meta.table_offset {
            Some(offset) => format!("from offset {offset}"),
            None => "with no offset declared".to_string(),
        };
        return Err(format!(
            "{rows} rows of {record_length} bytes {where_from} are {expected}, the file holds {total} ({delta:+} bytes{extra})"
        ));
    }
    Ok(Report {
        record_length,
        rows,
        offset: meta.table_offset,
        declared_size: meta.file_size,
    })
}

fn is_label(origin: &str) -> bool {
    let lower = origin.to_lowercase();
    lower.ends_with(".xml") || lower.ends_with(".lblx") || lower.ends_with(".lbl")
}

fn data_candidates(origin: &str) -> Vec<String> {
    if !is_label(origin) {
        return vec![origin.to_string()];
    }
    match origin.rfind('.') {
        Some(at) => {
            let stem = &origin[..at];
            vec![
                format!("{stem}.tab"),
                format!("{stem}.dat"),
                format!("{stem}.csv"),
            ]
        }
        None => vec![origin.to_string()],
    }
}

fn labels_of(origin: &str) -> Vec<String> {
    match origin.rfind('.') {
        Some(at) => {
            let stem = &origin[..at];
            vec![
                format!("{stem}.xml"),
                format!("{stem}.lblx"),
                format!("{stem}.lbl"),
            ]
        }
        None => vec![format!("{origin}.xml")],
    }
}

fn entries(register: &str, format: &str) -> Result<Vec<(String, String)>, String> {
    let text =
        fs::read_to_string(register).map_err(|e| format!("read {register} returned void: {e}"))?;
    let mut blocks: Vec<std::collections::BTreeMap<String, String>> = Vec::new();
    let mut current: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let flush = |current: &mut std::collections::BTreeMap<String, String>,
                 blocks: &mut Vec<std::collections::BTreeMap<String, String>>| {
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
    let mut found = Vec::new();
    for block in blocks {
        if block.get("format").map(|f| f == format).unwrap_or(false)
            && let (Some(asset), Some(origin)) = (block.get("url"), block.get("origin"))
        {
            found.push((asset.clone(), origin.clone()));
        }
    }
    if found.is_empty() {
        return Err(format!(
            "{register} carries no {format} entry with an origin"
        ));
    }
    Ok(found)
}

fn run_register(args: &[String]) -> Result<(), String> {
    let usage = "usage: pds4_descriptor --register <phi/sources.φ> --format <name> [--limit <n>] [--out <tsv>]";
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
    let mut sink: Option<std::io::BufWriter<fs::File>> = match &out {
        None => None,
        Some(path) => {
            let file =
                fs::File::create(path).map_err(|e| format!("create {path} returned void: {e}"))?;
            let mut writer = std::io::BufWriter::new(file);
            use std::io::Write;
            writeln!(
                writer,
                "asset\trecord_bytes\trows\toffset\tdeclared_size\tverdict"
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
    for (asset, origin) in list.iter().take(limit) {
        let name = asset.rsplit('/').next().unwrap_or("unnamed").to_string();
        let mut label: Option<String> = None;
        let mut tried: Vec<String> = Vec::new();
        for candidate in labels_of(origin) {
            match fetch(&candidate) {
                Ok(bytes) => {
                    label = Some(String::from_utf8_lossy(&bytes).to_string());
                    break;
                }
                Err(reason) => {
                    let name = candidate.rsplit('/').next().unwrap_or("");
                    let why = reason
                        .strip_prefix(&format!("{candidate}: "))
                        .unwrap_or(&reason)
                        .to_string();
                    tried.push(format!("{name} → {why}"));
                }
            }
        }
        let last = format!("no label reachable, tried {}", tried.join(", "));
        let Some(label) = label else {
            absent += 1;
            let line = format!("{name}\tabsent\tabsent\tabsent\tabsent\t{last}");
            eprintln!("{line}");
            if let Some(writer) = sink.as_mut() {
                use std::io::Write;
                writeln!(writer, "{line}").map_err(|e| format!("write returned void: {e}"))?;
                writer
                    .flush()
                    .map_err(|e| format!("flush returned void: {e}"))?;
            }
            continue;
        };
        let mut measured: Option<(String, usize)> = None;
        for candidate in data_candidates(origin) {
            if let Ok(len) = length(&candidate, None) {
                measured = Some((candidate, len));
                break;
            }
        }
        let line = match measured {
            None => {
                absent += 1;
                format!(
                    "{name}\tabsent\tabsent\tabsent\tabsent\tno data file reachable beside {}",
                    origin.rsplit('/').next().unwrap_or("the label")
                )
            }
            Some((_, total)) => match describe(&label, total) {
                Ok(report) => {
                    gated += 1;
                    format!(
                        "{name}\t{}\t{}\t{}\t{}\tgated-structure",
                        report.record_length,
                        report.rows,
                        match report.offset {
                            Some(offset) => offset.to_string(),
                            None => "absent".to_string(),
                        },
                        match report.declared_size {
                            Some(size) => size.to_string(),
                            None => "absent".to_string(),
                        }
                    )
                }
                Err(reason) => {
                    absent += 1;
                    format!("{name}\tabsent\tabsent\tabsent\tabsent\t{reason}")
                }
            },
        };
        eprintln!("{line}");
        if let Some(writer) = sink.as_mut() {
            use std::io::Write;
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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--register") {
        return run_register(&args);
    }
    let usage = "usage: pds4_descriptor --label <path|url> --data <path|url> [--name <asset>]";
    let mut label = None;
    let mut data = None;
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
            "--data" => data = Some(value()?),
            "--name" => name = value()?,
            other => return Err(format!("{other}: not a flag of this tool")),
        }
        index += 2;
    }
    let label = label.ok_or_else(|| usage.to_string())?;
    let data = data.ok_or_else(|| usage.to_string())?;
    let label_text = String::from_utf8_lossy(&fetch(&label)?).to_string();
    let whole = if data.starts_with("http") {
        None
    } else {
        Some(fs::read(&data).map_err(|e| format!("read {data} returned void: {e}"))?)
    };
    let total = length(&data, whole.as_deref())?;
    match describe(&label_text, total) {
        Ok(report) => {
            println!(
                "{name}\t{}\t{}\t{}\t{}\tgated-structure",
                report.record_length,
                report.rows,
                match report.offset {
                    Some(offset) => offset.to_string(),
                    None => "absent".to_string(),
                },
                match report.declared_size {
                    Some(size) => size.to_string(),
                    None => "absent".to_string(),
                }
            );
            Ok(())
        }
        Err(reason) => {
            println!("{name}\tabsent\tabsent\tabsent\tabsent\t{reason}");
            Err(reason)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LABEL: &str = "<Product_Observational>\
<File><file_name>T.DAT</file_name><file_size unit=\"byte\">60</file_size></File>\
<Table_Character><records>3</records><record_length unit=\"byte\">20</record_length>\
<Record_Character><fields>2</fields></Record_Character></Table_Character></Product_Observational>";

    #[test]
    fn a_consistent_label_passes_with_its_declared_size() {
        let report = describe(LABEL, 60).expect("gated");
        assert_eq!(report.record_length, 20);
        assert_eq!(report.rows, 3);
        assert_eq!(report.declared_size, Some(60));
    }

    #[test]
    fn a_declared_size_that_contradicts_the_wire_is_absent() {
        let reason = describe(LABEL, 80).expect_err("must not pass");
        assert!(reason.contains("60") && reason.contains("80"), "{reason}");
    }

    #[test]
    fn a_label_without_file_size_checks_the_record_against_the_wire() {
        let label = LABEL.replace("<file_size unit=\"byte\">60</file_size>", "");
        let report = describe(&label, 60).expect("gated");
        assert_eq!(report.declared_size, None);
        let reason = describe(&label, 61).expect_err("must not pass");
        assert!(reason.contains("+1"), "{reason}");
    }
}
