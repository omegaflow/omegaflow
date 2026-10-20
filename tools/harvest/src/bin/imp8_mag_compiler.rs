
use omegaflow::archivar::asset_path::compiled;
use omegaflow::lsk::days_from_civil;
use omegaflow_harvest::table::{Table, arg_value, emit, has_flag, load_source, parse_bin, stat};

const MAGIC: [u8; 4] = *b"IMP8";
const NETLOC: &str = "spdf.gsfc.nasa.gov";
const FORMAT: &str = "imp8_mag_hr";
const COMPILER: &str = "tools/harvest/src/bin/imp8_mag_compiler.rs";
const DEFAULT_URL: &str =
    "https://spdf.gsfc.nasa.gov/pub/data/imp/imp8/mag/1hr_ascii_sw/imp_mag_hr_sw1974.asc";
const DEFAULT_OUT: &str = "imp8_mag_hr.bin";

const WIDTHS: [usize; 23] = [
    4, 4, 3, 4, // year, doy, hour, npoints
    6, 6, 6, 6, 6, // X/Y/Z GSE, Y/Z GSM
    6, 6, 6, 6, // |B|, |avg B|, lat, lon
    6, 6, 6, 6, 6, // Bx/By/Bz GSE, By/Bz GSM
    6, 6, 6, 6, 6, // sigma |B|, sigma B, sigma Bx/By/Bz
];

const NAMES: [&str; 24] = [
    "mjd",
    "year",
    "doy",
    "hour",
    "npoints",
    "x_gse_re",
    "y_gse_re",
    "z_gse_re",
    "y_gsm_re",
    "z_gsm_re",
    "b",
    "b_vec",
    "b_lat_deg",
    "b_lon_deg",
    "bx",
    "by",
    "bz",
    "by_gsm",
    "bz_gsm",
    "sigma_b",
    "sigma_b_vec",
    "sigma_bx",
    "sigma_by",
    "sigma_bz",
];

fn cell<'a>(line: &'a str, at: usize, width: usize) -> Option<&'a str> {
    line.get(at..at + width)
}

fn number(raw: &str, max_abs: f64) -> Option<f64> {
    let value = raw.trim().parse::<f64>().ok()?;
    if !value.is_finite() || value.abs() >= max_abs {
        return None;
    }
    Some(value)
}

fn integer(raw: &str, max_abs: i64) -> Option<i64> {
    let value = raw.trim().parse::<i64>().ok()?;
    if value.abs() >= max_abs {
        return None;
    }
    Some(value)
}

fn doy_month_day(year: i64, doy: i64) -> Option<(i64, i64)> {
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    if doy < 1 || doy > if leap { 366 } else { 365 } {
        return None;
    }
    let mut left = doy;
    for month in 1..=12 {
        let days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if leap => 29,
            2 => 28,
            _ => return None,
        };
        if left <= days {
            return Some((month, left));
        }
        left -= days;
    }
    None
}

pub fn mjd_of(year: i64, doy: i64, hour: i64) -> Option<f64> {
    let (month, day) = doy_month_day(year, doy)?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 + 40587.0 + hour as f64 / 24.0)
}

pub fn parse_text(text: &str) -> Option<Table> {
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut at = 0usize;
        let mut raw = [""; 23];
        let mut complete = true;
        for (i, width) in WIDTHS.iter().enumerate() {
            match cell(line, at, *width) {
                Some(s) => raw[i] = s,
                None => {
                    complete = false;
                    break;
                }
            }
            at += *width;
        }
        if !complete {
            continue;
        }
        let (Some(year), Some(doy), Some(hour)) = (
            integer(raw[0], 10_000),
            integer(raw[1], 400),
            integer(raw[2], 100),
        ) else {
            continue;
        };
        if !(1973..=2010).contains(&year) || !(0..=23).contains(&hour) {
            continue;
        }
        let Some(mjd) = mjd_of(year, doy, hour) else {
            continue;
        };
        let mut row = vec![None; NAMES.len()];
        row[0] = Some(mjd);
        row[1] = Some(year as f64);
        row[2] = Some(doy as f64);
        row[3] = Some(hour as f64);
        row[4] = integer(raw[3], 900).map(|v| v as f64);
        let maxima = [
            100.0, 100.0, 100.0, 100.0, 100.0, // GSE/GSM positions
            900.0, 900.0, 900.0, 900.0, // |B|, |avg B|, lat, lon
            900.0, 900.0, 900.0, 900.0, 900.0, // B components
            900.0, 900.0, 900.0, 900.0, 900.0, // sigmas
        ];
        for (i, max) in maxima.iter().enumerate() {
            row[5 + i] = number(raw[4 + i], *max);
        }
        if row[5..].iter().any(Option::is_some) {
            rows.push(row);
        }
    }
    if rows.is_empty() {
        None
    } else {
        Some(Table {
            names: NAMES.iter().map(|s| s.to_string()).collect(),
            rows,
        })
    }
}

fn selftest() {
    let line = "1974   1  1  41  19.4   1.9  16.0  -2.1  16.0   4.4   2.9  47.2 289.8   0.7  -1.9   2.1  -2.3   1.6   0.6   3.0   2.0   1.2   1.8";
    let Some(table) = parse_text(line) else {
        eprintln!("selftest: the fixed-width fixture parses void");
        std::process::exit(1);
    };
    let row = &table.rows[0];
    if row[1] != Some(1974.0) || row[10] != Some(4.4) || row[13] != Some(16.0) {
        eprintln!("selftest: the fixture does not carry year / |B| / Bz");
        std::process::exit(1);
    }
    if row[0].is_none() {
        eprintln!("selftest: the MJD anchor stays absent");
        std::process::exit(1);
    }
    let fill = "1974   1  0 999 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9";
    if parse_text(fill).is_some() {
        eprintln!("selftest: an all-fill row is not omitted");
        std::process::exit(1);
    }
    eprintln!("imp8_mag_compiler: selftest passes (fixed width + fill absence)");
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
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{src}: the body is not utf8 — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(table) = parse_text(text) else {
        eprintln!("{src}: no measured hour survived — absent (0 honored)");
        std::process::exit(1);
    };
    if has_flag(&args, "--stat") {
        stat(&src, &table);
        return;
    }
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            let path = compiled(NETLOC, DEFAULT_OUT);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let result = emit(&src, &out, &table, MAGIC, NETLOC, FORMAT, COMPILER);
    if !result {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") {
        let parsed = std::fs::read(&out)
            .ok()
            .and_then(|bytes| parse_bin(MAGIC, &bytes));
        let Some(parsed) = parsed else {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        };
        if parsed.rows.len() != table.rows.len() {
            eprintln!("{out}: roundtrip row count differs — the bin stays unverified");
            std::process::exit(1);
        }
        if !omegaflow::cdn::upload_release(NETLOC, &out) {
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE: &str = "1974   1  1  41  19.4   1.9  16.0  -2.1  16.0   4.4   2.9  47.2 289.8   0.7  -1.9   2.1  -2.3   1.6   0.6   3.0   2.0   1.2   1.8";

    #[test]
    fn fixed_width_row_maps_the_measured_columns() {
        let table = parse_text(LINE).unwrap();
        assert_eq!(table.names.len(), 24);
        assert_eq!(table.rows.len(), 1);
        let row = &table.rows[0];
        assert_eq!(row[1], Some(1974.0));
        assert_eq!(row[2], Some(1.0));
        assert_eq!(row[3], Some(1.0));
        assert_eq!(row[4], Some(41.0));
        assert!((row[0].unwrap() - 42048.041666666664).abs() < 1e-6);
        assert_eq!(row[10], Some(4.4));
        assert_eq!(row[14], Some(0.7));
        assert_eq!(row[16], Some(2.1));
        assert_eq!(row[19], Some(0.6));
        assert_eq!(row[20], Some(3.0));
    }

    #[test]
    fn fill_words_stay_absent_and_all_fill_rows_drop() {
        let fill = "1974   1  0 999 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9";
        assert!(parse_text(fill).is_none());
        let mixed = "1974   1  1  41 999.9 999.9 999.9 999.9 999.9   4.4 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9 999.9";
        let table = parse_text(mixed).unwrap();
        assert_eq!(table.rows[0][10], Some(4.4));
        assert_eq!(table.rows[0][5], None);
    }

    #[test]
    fn truncated_or_foreign_lines_are_not_placeable() {
        assert!(parse_text("1974 1 1 41 19.4").is_none());
        assert!(parse_text("not an IMP-8 record").is_none());
    }
}
