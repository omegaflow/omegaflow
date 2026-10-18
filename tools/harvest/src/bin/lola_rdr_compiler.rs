use omegaflow::archivar::pds3_table::{decode_rows, TableColumn, TableMeta};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;
use std::fmt::Write as _;

const TAG: &str = "pds-geosciences.wustl.edu";
const SPOTS: usize = 5;
const FLAG_MASK: u64 = 0xFCFF;
const GUARD_MM: f64 = 20_000_000.0;

fn empty_column() -> TableColumn {
    TableColumn {
        name: String::new(),
        unit: None,
        data_type: None,
        missing_constant: None,
        sampling_name: String::new(),
        sampling_unit: String::new(),
        sampling_min: None,
        sampling_max: None,
        start_byte: 0,
        bytes: 0,
    }
}

fn parse_fmt_columns(text: &str) -> Vec<TableColumn> {
    let mut columns = Vec::new();
    let mut current: Option<TableColumn> = None;
    let mut in_comment = false;
    for line in text.lines() {
        let t = line.trim();
        if in_comment {
            if t.contains("*/") {
                in_comment = false;
            }
            continue;
        }
        if t.starts_with("/*") {
            if !t.contains("*/") {
                in_comment = true;
            }
            continue;
        }
        if t.starts_with("OBJECT") && t.contains("COLUMN") {
            current = Some(empty_column());
            continue;
        }
        if t.starts_with("END_OBJECT") {
            if let Some(c) = current.take() {
                if !c.name.is_empty() {
                    columns.push(c);
                }
            }
            continue;
        }
        let Some(column) = current.as_mut() else {
            continue;
        };
        let Some((key, value)) = t.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "NAME" => column.name = value.to_string(),
            "DATA_TYPE" => column.data_type = Some(value.to_string()),
            "UNIT" => column.unit = Some(value.trim_matches('\'').to_string()),
            "START_BYTE" => {
                if let Ok(v) = value.parse() {
                    column.start_byte = v;
                }
            }
            "BYTES" => {
                if let Ok(v) = value.parse() {
                    column.bytes = v;
                }
            }
            "MISSING_CONSTANT" => column.missing_constant = value.parse().ok(),
            _ => {}
        }
    }
    columns
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn spot_is_valid(radius_mm: f64, selenoid_mm: f64, flag: u64) -> bool {
    radius_mm > 0.0
        && (flag & FLAG_MASK) == 0
        && (radius_mm - selenoid_mm).abs() <= GUARD_MM
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let fmt_path = match arg_value(&args, "--fmt") {
        Some(v) => v,
        None => {
            eprintln!("--fmt <lolardr.fmt> absent — refused");
            std::process::exit(1);
        }
    };
    let dat_path = match arg_value(&args, "--dat") {
        Some(v) => v,
        None => {
            eprintln!("--dat <lolardr_...dat> absent — refused");
            std::process::exit(1);
        }
    };
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("--out <csv> absent — refused");
            std::process::exit(1);
        }
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let fmt = match std::fs::read_to_string(&fmt_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("read {fmt_path} returned void: {e}");
            std::process::exit(1);
        }
    };
    let columns = parse_fmt_columns(&fmt);
    if columns.is_empty() {
        eprintln!("{fmt_path}: no COLUMN object parsed");
        std::process::exit(1);
    }
    let meta = TableMeta {
        record_bytes: Some(256),
        file_records: None,
        rows: None,
        interchange: Some("BINARY".to_string()),
        table_file: None,
        start_time: None,
        stop_time: None,
        spacecraft_name: None,
        target_name: None,
        instrument_name: None,
        product_id: None,
        columns,
    };
    let dat = match std::fs::read(&dat_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("read {dat_path} returned void: {e}");
            std::process::exit(1);
        }
    };
    let (rows, skipped, trailing) = match decode_rows(&dat, &meta) {
        Some(v) => v,
        None => {
            eprintln!("{dat_path}: table decodes void");
            std::process::exit(1);
        }
    };

    let idx: HashMap<&str, usize> = meta
        .columns
        .iter()
        .enumerate()
        .map(|(i, c)| (c.name.as_str(), i))
        .collect();
    let get = |row: &[Option<f64>], name: &str| -> Option<f64> {
        idx.get(name)
            .and_then(|i| row.get(*i).copied().flatten())
    };

    let mut out = String::from("t_tt_s,spot,lat_deg,lon_deg,radius_m,selenoid_m,elevation_m\n");
    let mut valid_spots = 0usize;
    for row in &rows {
        let (Some(raw_time), Some(selenoid)) = (
            get(row, "TRANSMIT_TIME"),
            get(row, "SELENOID_RADIUS"),
        ) else {
            continue;
        };
        let raw = raw_time as u64;
        let seconds = (raw & 0xFFFF_FFFF) as f64;
        let fraction = (raw >> 32) as f64 / 4_294_967_296.0;
        let t = seconds + fraction;
        for n in 1..=SPOTS {
            let (Some(radius), Some(flag), Some(lat), Some(lon)) = (
                get(row, &format!("RADIUS_{n}")),
                get(row, &format!("SHOT_FLAG_{n}")),
                get(row, &format!("LATITUDE_{n}")),
                get(row, &format!("LONGITUDE_{n}")),
            ) else {
                continue;
            };
            if !spot_is_valid(radius, selenoid, flag as u64) {
                continue;
            }
            let radius_m = radius / 1000.0;
            let selenoid_m = selenoid / 1000.0;
            let _ = writeln!(
                out,
                "{t:.6},{n},{:.7},{:.7},{radius_m:.3},{selenoid_m:.3},{:.3}",
                lat / 1e7,
                lon / 1e7,
                radius_m - selenoid_m
            );
            valid_spots += 1;
        }
    }
    if valid_spots == 0 {
        eprintln!("no valid spot — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if let Err(e) = std::fs::write(&out_path, out.as_bytes()) {
        eprintln!("write {out_path} returned void: {e}");
        std::process::exit(1);
    }
    eprintln!(
        "{dat_path}: {} rows ({skipped} skipped, {trailing} trailing bytes) -> {valid_spots} valid spots -> {out_path} ({} B)",
        rows.len(),
        out.len()
    );
    if ci_mode && !upload_release(TAG, &out_path) {
        eprintln!("{out_path}: CDN upload returned void");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn river_rule_keeps_r04_and_rmu_phases_only() {
        assert!(spot_is_valid(1_728_352_117.0, 1_737_479_142.0, 0x0200));
        assert!(spot_is_valid(1_737_000_000.0, 1_737_000_000.0, 0x0300));
        assert!(!spot_is_valid(1_823_130_852.0, 1_737_490_291.0, 0x2A41));
        assert!(!spot_is_valid(1_737_000_000.0, 1_737_000_000.0, 0x0001));
        assert!(!spot_is_valid(1_737_000_000.0, 1_737_000_000.0, 0x0400));
        assert!(!spot_is_valid(1_760_000_000.0, 1_737_000_000.0, 0x0200));
        assert!(!spot_is_valid(-1.0, 1_737_000_000.0, 0x0200));
    }
}
