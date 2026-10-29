use std::env;
use std::fs;

use omegaflow::archivar::mariner_occlt;
use omegaflow::archivar::mariner_rst;
use omegaflow::archivar::mariner4_odg;

struct Sample {
    time: f64,
    value: f64,
}

const GATED: [&str; 3] = ["mariner_occlt", "mariner4_odg", "mariner_rst"];

struct Classes {
    format: String,
    structure: bool,
    reader: bool,
    law: bool,
}

impl Classes {
    fn measured(&self) -> usize {
        [self.structure, self.reader, self.law]
            .iter()
            .filter(|m| **m)
            .count()
    }

    fn absent(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if !self.structure {
            out.push("struktur");
        }
        if !self.reader {
            out.push("leser");
        }
        if !self.law {
            out.push("gesetz");
        }
        out
    }
}

const DESCRIPTORS: &str = "tools/harvest/src/asset_descriptors";

fn descriptor_table() -> Option<std::collections::BTreeMap<String, String>> {
    let mut out = std::collections::BTreeMap::new();
    let mut files = 0usize;
    let entries = fs::read_dir("tools/harvest/src").ok()?;
    let mut paths: Vec<std::path::PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            name.starts_with("asset_descriptors") && name.ends_with(".tsv")
        })
        .collect();
    paths.sort();
    for path in paths {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        files += 1;
        for line in text.lines().skip(1) {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 2 {
                continue;
            }
            out.insert(
                cols[0].trim().to_string(),
                cols[cols.len() - 1].trim().to_string(),
            );
        }
    }
    if files == 0 {
        return None;
    }
    Some(out)
}

fn classes_table() -> Vec<Classes> {
    let text = include_str!("../asset_classes.tsv");
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 4 {
            continue;
        }
        out.push(Classes {
            format: cols[0].trim().to_string(),
            structure: cols[1].trim() == "gemessen",
            reader: cols[2].trim() == "gemessen",
            law: cols[3].trim() == "gemessen",
        });
    }
    out
}

fn series(path: &str) -> Result<Vec<Sample>, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    if bytes.len() < 8 {
        return Err(format!("{path} carries {} bytes", bytes.len()));
    }
    match &bytes[0..4] {
        b"MOCC" => {
            let rows = mariner_occlt::parse_mocc_bin(&bytes)
                .ok_or_else(|| format!("{path}: the MOCC rows do not parse"))?;
            Ok(rows
                .iter()
                .map(|r| Sample {
                    time: r[0],
                    value: r[9],
                })
                .collect())
        }
        b"M4OD" | b"MRST" => {
            let rows = if &bytes[0..4] == b"M4OD" {
                mariner4_odg::parse_bin(&bytes)
            } else {
                mariner_rst::parse_bin(&bytes)
            }
            .ok_or_else(|| format!("{path}: the rows do not parse"))?;
            Ok(rows
                .into_iter()
                .map(|(t, v, _)| Sample { time: t, value: v })
                .collect())
        }
        other => Err(format!(
            "{path}: signature {} is not in the gate table",
            String::from_utf8_lossy(other)
        )),
    }
}

fn audit(path: &str) -> Result<String, String> {
    let rows = series(path)?;
    if rows.is_empty() {
        return Err(format!("{path} carries no rows"));
    }
    let mut violations = 0;
    let mut absent = 0;
    let mut lowest = f64::INFINITY;
    let mut highest = f64::NEG_INFINITY;
    for (index, row) in rows.iter().enumerate() {
        if !row.time.is_finite() || !row.value.is_finite() {
            absent += 1;
            continue;
        }
        if index > 0 && row.time < rows[index - 1].time {
            violations += 1;
        }
        lowest = lowest.min(row.value);
        highest = highest.max(row.value);
    }
    let span = rows[rows.len() - 1].time - rows[0].time;
    let motion = highest - lowest;
    let first_is_absent = !rows[0].time.is_finite() || !rows[0].value.is_finite();
    let last_is_absent =
        !rows[rows.len() - 1].time.is_finite() || !rows[rows.len() - 1].value.is_finite();
    let verdict = if absent > 0 {
        "absent values"
    } else if first_is_absent || last_is_absent {
        "ends absent"
    } else if violations > 0 {
        "time not monotone"
    } else if !(span > 0.0) {
        "span carries no advance"
    } else if !(motion > 0.0) {
        "value carries no motion"
    } else {
        "passes"
    };
    Ok(format!(
        "{:<44} rows {:>7}  span {:>10.1} s  order {:>6}  absent {:>5}  motion {:>10.3}  {}",
        path,
        rows.len(),
        span,
        violations,
        absent,
        motion,
        verdict
    ))
}

fn coverage(register: &str) -> Result<(), String> {
    let text =
        fs::read_to_string(register).map_err(|e| format!("read {register} returned void: {e}"))?;
    let gated = GATED;
    let mut format: Option<String> = None;
    let mut total = 0usize;
    let mut covered = 0usize;
    let mut with_key = 0usize;
    let mut structure_only = 0usize;
    let descriptors = descriptor_table();
    let classes = classes_table();
    let mut full = 0usize;
    let mut gated_formats = 0usize;
    let mut by_format: std::collections::BTreeMap<String, (usize, usize)> =
        std::collections::BTreeMap::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("url ") {
            let is_bin = rest.trim_end().ends_with(".bin");
            let name = match &format {
                Some(name) => name.clone(),
                None => "entry without a format line".to_string(),
            };
            let entry = by_format.entry(name).or_insert((0, 0));
            if is_bin {
                entry.0 += 1;
                total += 1;
                let asset = rest.trim().rsplit('/').next().unwrap_or("");
                let verdict = descriptors.as_ref().and_then(|d| d.get(asset));
                match verdict.map(|v| v.as_str()) {
                    Some("gated") => {
                        with_key += 1;
                        covered += 1;
                        entry.1 += 1;
                    }
                    Some("gated-structure") => {
                        structure_only += 1;
                        covered += 1;
                        entry.1 += 1;
                    }
                    _ => {
                        let is_gated = gated.iter().any(|g| format.as_deref() == Some(g));
                        if is_gated {
                            covered += 1;
                            entry.1 += 1;
                        }
                    }
                }
            }
        } else if let Some(rest) = line.strip_prefix("format ") {
            format = Some(rest.trim().to_string());
        }
    }
    println!("register {register}: {total} bin assets, {covered} carried by a gate descriptor");
    match &descriptors {
        Some(map) => println!(
            "  descriptors {DESCRIPTORS}: {} rows — {with_key} with key, {structure_only} structure only",
            map.len()
        ),
        None => println!("  descriptors absent: {DESCRIPTORS}"),
    }
    let mut open = 0usize;
    for (name, (count, gated_count)) in &by_format {
        if *count == 0 {
            continue;
        }
        let missing = count - gated_count;
        open += missing;
        if *gated_count == 0 {
            println!("  {:<28} {:>4}  no descriptor", name, count);
            continue;
        }
        let state = if missing == 0 {
            gated_formats += 1;
            "gated".to_string()
        } else {
            format!("partial {gated_count}/{count}")
        };
        match classes.iter().find(|c| &c.format == name) {
            Some(c) if c.measured() == 3 => {
                full += 1;
                println!("  {:<28} {:>4}  {state}, classes 3/3", name, count);
            }
            Some(c) => {
                println!(
                    "  {:<28} {:>4}  {state}, classes {}/3, absent: {}",
                    name,
                    count,
                    c.measured(),
                    c.absent().join(", ")
                );
            }
            None => {
                println!(
                    "  {:<28} {:>4}  {state}, classes 0/3, no class row",
                    name, count
                );
            }
        }
    }
    println!("open without descriptor: {open}");
    println!("gated formats: {gated_formats}, with three classes: {full}");
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(
            "usage: asset_gate <bin> [<bin> ...]  |  asset_gate --register [<phi/sources.φ>]"
                .to_string(),
        );
    }
    if args[0] == "--register" {
        let register = args
            .get(1)
            .ok_or_else(|| "usage: asset_gate --register <phi/sources.φ>".to_string())?;
        return coverage(register);
    }
    let mut failed = 0;
    for path in &args {
        match audit(path) {
            Ok(line) => {
                if !line.ends_with("passes") {
                    failed += 1;
                }
                println!("{line}");
            }
            Err(reason) => {
                failed += 1;
                println!("{reason}");
            }
        }
    }
    if failed > 0 {
        return Err(format!(
            "{failed} of {} assets carry no gate pass",
            args.len()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mocc_bin(rows: &[(f64, f64)]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"MOCC");
        out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
        for (t, v) in rows {
            let mut row = [0.0f64; mariner_occlt::MOCC_STRIDE];
            row[0] = *t;
            row[9] = *v;
            for cell in row {
                out.extend_from_slice(&cell.to_le_bytes());
            }
        }
        out
    }

    fn write(path: &str, bytes: &[u8]) {
        fs::write(path, bytes).expect("write");
    }

    fn scratch(name: &str) -> String {
        let base = env::temp_dir();
        base.join(format!("asset_gate_{name}.bin"))
            .to_string_lossy()
            .to_string()
    }

    #[test]
    fn a_growing_series_passes() {
        let path = scratch("growing");
        write(&path, &mocc_bin(&[(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)]));
        let line = audit(&path).expect("audit");
        assert!(line.ends_with("passes"), "{line}");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_series_that_steps_back_in_time_earns_a_verdict() {
        let path = scratch("backwards");
        write(&path, &mocc_bin(&[(10.0, 1.0), (20.0, 2.0), (5.0, 1.5)]));
        let line = audit(&path).expect("audit");
        assert!(line.ends_with("time not monotone"), "{line}");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_series_without_motion_earns_a_verdict() {
        let path = scratch("flat");
        write(&path, &mocc_bin(&[(0.0, 3.0), (1.0, 3.0), (2.0, 3.0)]));
        let line = audit(&path).expect("audit");
        assert!(line.ends_with("value carries no motion"), "{line}");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn an_unknown_signature_is_named() {
        let path = scratch("unknown");
        write(&path, b"ZZZZ0000");
        let reason = audit(&path).expect_err("audit");
        assert!(reason.contains("is not in the gate table"), "{reason}");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn every_gated_format_has_a_row_in_the_class_table() {
        let table = classes_table();
        for name in GATED {
            assert!(
                table.iter().any(|c| c.format == name),
                "{name} parses but carries no class row"
            );
        }
    }

    #[test]
    fn a_row_with_fewer_than_three_classes_names_the_absent_ones() {
        for row in classes_table() {
            assert_eq!(
                row.measured() + row.absent().len(),
                3,
                "{}: a class is neither measured nor named absent",
                row.format
            );
        }
    }

    #[test]
    fn a_format_with_three_measured_classes_is_counted_as_full() {
        let row = Classes {
            format: "probe".to_string(),
            structure: true,
            reader: true,
            law: true,
        };
        assert_eq!(row.measured(), 3);
        assert!(row.absent().is_empty());
    }
}
