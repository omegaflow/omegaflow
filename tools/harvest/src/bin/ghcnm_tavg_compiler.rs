use omegaflow::archivar::inflate::gunzip_tar_members;
use omegaflow::cdn::upload_release;
use omegaflow_harvest::table::{Table, arg_or, emit, has_flag, load_source, stat};

const MAGIC: [u8; 4] = *b"GHCN";
const NETLOC: &str = "ncei.noaa.gov";
const FORMAT: &str = "ghcnm_tavg";
const COMPILER: &str = "tools/harvest/src/bin/ghcnm_tavg_compiler.rs";
const DEFAULT_URL: &str = "https://www.ncei.noaa.gov/pub/data/ghcn/v4/ghcnm.tavg.latest.qcf.tar.gz";
const DEFAULT_OUT: &str = "ghcnm_tavg.bin";

const YEAR_AT: usize = 11;
const ELEMENT_AT: usize = 15;
const VALUE_AT: usize = 19;
const VALUE_STRIDE: usize = 8;
const VALUE_WIDTH: usize = 5;
const VALUE_COLS: usize = 12;
const LINE_BYTES: usize = VALUE_AT + VALUE_COLS * VALUE_STRIDE; // 115
const MISSING: i64 = -9999;

fn month_names() -> Vec<String> {
    let mut names = vec!["year".to_string()];
    for m in 1..=VALUE_COLS {
        names.push(format!("m{m:02}"));
    }
    names
}

fn cell_i64(bytes: &[u8], at: usize, width: usize) -> Option<i64> {
    let raw = std::str::from_utf8(bytes.get(at..at + width)?).ok()?;
    raw.trim().parse().ok()
}

fn dat_to_table(dat: &[u8]) -> Option<Table> {
    let names = month_names();
    let mut rows = Vec::new();
    for line in dat.split(|&b| b == b'\n') {
        if line.len() < LINE_BYTES {
            continue;
        }
        if &line[ELEMENT_AT..ELEMENT_AT + 4] != b"TAVG".as_slice() {
            continue;
        }
        let Some(year) = cell_i64(line, YEAR_AT, 4) else {
            continue;
        };
        let mut row = vec![None; names.len()];
        row[0] = Some(year as f64);
        let mut present = false;
        for (i, slot) in row.iter_mut().enumerate().skip(1) {
            let Some(value) = cell_i64(line, VALUE_AT + (i - 1) * VALUE_STRIDE, VALUE_WIDTH) else {
                continue;
            };
            if value == MISSING {
                continue;
            }
            *slot = Some(value as f64 / 100.0);
            present = true;
        }
        if present {
            rows.push(row);
        }
    }
    if rows.is_empty() {
        None
    } else {
        Some(Table { names, rows })
    }
}

fn collect_dat(bytes: &[u8]) -> Option<Vec<u8>> {
    let mut dat: Option<Vec<u8>> = None;
    let seen = gunzip_tar_members(
        std::io::Cursor::new(bytes),
        |name| name.ends_with(".dat"),
        |_name, data| {
            if dat.is_none() {
                dat = Some(data.to_vec());
            }
        },
    )
    .ok()?;
    if seen == 0 {
        return None;
    }
    dat
}

fn fixture_line(values: [i64; VALUE_COLS]) -> String {
    let mut line = String::from("ACW000116041961TAVG");
    for value in values {
        line.push_str(&format!("{value:>5}   "));
    }
    line
}

fn selftest() {
    let line = fixture_line([-71, 254, 0, -9999, 100, 200, 300, 400, 500, 600, 700, -9999]);
    let Some(table) = dat_to_table(line.as_bytes()) else {
        eprintln!("selftest: the fixed-width fixture parses void");
        std::process::exit(1);
    };
    if table.names[0] != "year" || table.names.len() != VALUE_COLS + 1 {
        eprintln!("selftest: the header does not name year + 12 months");
        std::process::exit(1);
    }
    let row = &table.rows[0];
    if row[0] != Some(1961.0) || row[1] != Some(-0.71) || row[3] != Some(0.0) {
        eprintln!("selftest: a month does not carry the measured value");
        std::process::exit(1);
    }
    if row[4].is_some() || row[12].is_some() {
        eprintln!("selftest: -9999 does not stay absent (0 honored)");
        std::process::exit(1);
    }
    let foreign = "ACW000116041961TAVG  not fixed width";
    if dat_to_table(foreign.as_bytes()).is_some() {
        eprintln!("selftest: a short foreign line is not refused");
        std::process::exit(1);
    }
    eprintln!("ghcnm_tavg_compiler: selftest passes (fixed width + -9999 absence)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--selftest") {
        selftest();
        return;
    }
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Some(dat) = collect_dat(&bytes) else {
        eprintln!("{src}: no .dat member read — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(table) = dat_to_table(&dat) else {
        eprintln!("{src}: no TAVG rows survived — absent (0 honored)");
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
    fn fixed_width_reads_values_and_keeps_missing_absent() {
        let line = fixture_line([-71, 254, 0, -9999, 100, 200, 300, 400, 500, 600, 700, -9999]);
        let t = dat_to_table(line.as_bytes()).unwrap();
        assert_eq!(t.rows.len(), 1);
        assert_eq!(t.rows[0][0], Some(1961.0));
        assert_eq!(t.rows[0][1], Some(-0.71));
        assert_eq!(t.rows[0][3], Some(0.0));
        assert_eq!(t.rows[0][4], None);
        assert_eq!(t.rows[0][5], Some(1.0));
        assert_eq!(t.rows[0][12], None);
    }

    #[test]
    fn non_tavg_and_short_lines_are_refused() {
        let mut other = fixture_line([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        other.replace_range(ELEMENT_AT..ELEMENT_AT + 4, "PRCP");
        assert!(dat_to_table(other.as_bytes()).is_none());
        assert!(dat_to_table(b"ACW000116041961TAVG  short").is_none());
    }

    #[test]
    fn an_all_missing_year_stays_unrecorded() {
        let line = fixture_line([MISSING; VALUE_COLS]);
        assert!(dat_to_table(line.as_bytes()).is_none());
    }
}
