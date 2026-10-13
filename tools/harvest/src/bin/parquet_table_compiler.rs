use omegaflow::archivar::parquet::{ParquetValue, parse_parquet};
use omegaflow::cdn::upload_release;
use omegaflow_harvest::table::{Table, arg_or, cell_num, emit, has_flag, load_source, stat};

const MAGIC: [u8; 4] = *b"PRQT";
const NETLOC: &str = "zenodo.org";
const FORMAT: &str = "parquet";
const COMPILER: &str = "tools/harvest/src/bin/parquet_table_compiler.rs";
const DEFAULT_URL: &str = "https://zenodo.org/api/records/11123894/files/features.parquet/content";
const DEFAULT_OUT: &str = "parquet_table.bin";

fn value_num(value: &ParquetValue) -> Option<f64> {
    match value {
        ParquetValue::I32(x) => Some(*x as f64),
        ParquetValue::I64(x) => Some(*x as f64),
        ParquetValue::Float(x) => {
            let y = *x as f64;
            y.is_finite().then_some(y)
        }
        ParquetValue::Double(x) => x.is_finite().then_some(*x),
        ParquetValue::Bytes(b) => std::str::from_utf8(b).ok().and_then(cell_num),
        ParquetValue::Bool(_) | ParquetValue::Int96(_) | ParquetValue::Unhandled(_) => None,
    }
}

fn parse_table(bytes: &[u8]) -> Option<Table> {
    let columns = parse_parquet(bytes)?;
    if columns.is_empty() {
        return None;
    }
    let n_rows = columns.iter().map(|c| c.values.len()).max()?;
    if n_rows == 0 {
        return None;
    }
    let names = columns.iter().map(|c| c.name.clone()).collect::<Vec<_>>();
    let mut rows = vec![vec![None; columns.len()]; n_rows];
    for (j, column) in columns.iter().enumerate() {
        for (i, value) in column.values.iter().enumerate() {
            if let Some(slot) = rows.get_mut(i).and_then(|r| r.get_mut(j)) {
                *slot = value_num(value);
            }
        }
    }
    Some(Table { names, rows })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--selftest") {
        eprintln!(
            "parquet_table_compiler: no fixture — the reader is measured against a live file"
        );
        return;
    }
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Some(table) = parse_table(&bytes) else {
        eprintln!(
            "{src}: {} B carry no parquet contract — absent (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    };
    if has_flag(&args, "--stat") {
        stat(&src, &table);
        return;
    }
    let out = arg_or(&args, "--out", DEFAULT_OUT);
    let netloc = arg_or(&args, "--netloc", NETLOC);
    if !emit(&src, &out, &table, MAGIC, &netloc, FORMAT, COMPILER) {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") && !upload_release(&netloc, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_num_maps_numeric_types_and_refuses_bool() {
        assert_eq!(value_num(&ParquetValue::I32(-3)), Some(-3.0));
        assert_eq!(value_num(&ParquetValue::Double(2.5)), Some(2.5));
        assert_eq!(value_num(&ParquetValue::Float(f32::NAN)), None);
        assert_eq!(value_num(&ParquetValue::Double(f64::INFINITY)), None);
        assert_eq!(value_num(&ParquetValue::Bool(true)), None);
    }

    #[test]
    fn value_num_parses_numeric_byte_columns() {
        assert_eq!(value_num(&ParquetValue::Bytes(b"3.5".to_vec())), Some(3.5));
        assert_eq!(value_num(&ParquetValue::Bytes(b"x".to_vec())), None);
    }
}
