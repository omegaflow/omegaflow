
use omegaflow::archivar::asset_path::compiled;
use omegaflow::lsk::days_from_civil;
use omegaflow_harvest::table::{Table, arg_value, emit, has_flag, load_source, parse_bin, stat};

const MAGIC: [u8; 4] = *b"HELI";
const NETLOC: &str = "spdf.gsfc.nasa.gov";
const FORMAT: &str = "helios_merged";
const COMPILER: &str = "tools/harvest/src/bin/helios_merged_compiler.rs";
const DEFAULT_URL: &str =
    "https://spdf.gsfc.nasa.gov/pub/data/helios/helios1/merged/helios1_daily.asc";
const DEFAULT_OUT: &str = "helios_merged.bin";

const WIDTHS: [usize; 31] = [
    4, 4, 3, 5, // year, doy, hour, Carrington rotation
    7, 7, 7, 7, // r, HGI lat, HGI lon, Earth-Sun-SC angle
    9, 9, 9, 9, 9, 9, 9, // BX/BY/BZ SSE, BR/BT/BN RTN, |B|
    7, 7, 7, // speed, elevation, azimuth
    6, 9, // density, temperature
    11, 11, 11, 11, // E6 4-13, 13-27, 27-37, 37-51 MeV
    10, 10, 10, 10, 10, 10, 10, // E7 3.4-6.05 ... 135.2-206.5 MeV
];

const NAMES: [&str; 32] = [
    "mjd",
    "year",
    "doy",
    "hour",
    "cr",
    "r",
    "lat",
    "lon",
    "ess",
    "bx",
    "by",
    "bz",
    "br",
    "bt",
    "bn",
    "bmag",
    "speed",
    "elev",
    "azim",
    "density",
    "temp",
    "e6_4_13",
    "e6_13_27",
    "e6_27_37",
    "e6_37_51",
    "e7_3_4_6",
    "e7_6_11",
    "e7_11_21",
    "e7_24_28",
    "e7_32_46",
    "e7_46_57",
    "e7_135_206",
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
        let mut raw = [""; 31];
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
        if !(1974..=1981).contains(&year) || !(0..=23).contains(&hour) {
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
        row[4] = integer(raw[3], 3000).map(|v| v as f64);
        let maxima = [
            99.0,
            900.0,
            900.0,
            900.0, // r, HGI lat/lon, ESS angle
            900.0,
            900.0,
            900.0,
            900.0,
            900.0,
            900.0,
            900.0, // B components
            3000.0,
            900.0,
            900.0, // speed, elevation, azimuth
            900.0,
            2_000_000.0, // density, temperature
            9000.0,
            9000.0,
            9000.0,
            9000.0, // E6
            9000.0,
            9000.0,
            9000.0,
            9000.0,
            9000.0,
            9000.0,
            9000.0, // E7
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
    let line = "1974 348  0 1622   0.98   -0.7    5.6  358.7    -3.31     1.51    -0.60     3.21    -1.23    -0.17     4.55  574.9    0.8   -0.3   2.5  110352. 1.3517E-04 3.9797E-05 7.7144E-05 3.8841E-05 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03";
    let Some(table) = parse_text(line) else {
        eprintln!("selftest: the fixed-width fixture parses void");
        std::process::exit(1);
    };
    let row = &table.rows[0];
    if row[1] != Some(1974.0) || row[15] != Some(4.55) || row[16] != Some(574.9) {
        eprintln!("selftest: the fixture does not carry year / |B| / speed");
        std::process::exit(1);
    }
    if row[0].is_none() || row[19] != Some(2.5) {
        eprintln!("selftest: the MJD anchor or density stays absent");
        std::process::exit(1);
    }
    let fill = "1974 348  0 9999 999.99 9999.9 9999.9 9999.9 99999.99 99999.99 99999.99 99999.99 99999.99 99999.99 99999.99 9999.9 9999.9 9999.9 999.9 9999999. 9.9999E+04 9.9999E+04 9.9999E+04 9.9999E+04 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03";
    if parse_text(fill).is_some() {
        eprintln!("selftest: an all-fill row is not omitted");
        std::process::exit(1);
    }
    eprintln!("helios_merged_compiler: selftest passes (fixed width + fill absence)");
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
        eprintln!("{src}: no measured row survived — absent (0 honored)");
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
    if !emit(&src, &out, &table, MAGIC, NETLOC, FORMAT, COMPILER) {
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

    const LINE: &str = "1974 348  0 1622   0.98   -0.7    5.6  358.7    -3.31     1.51    -0.60     3.21    -1.23    -0.17     4.55  574.9    0.8   -0.3   2.5  110352. 1.3517E-04 3.9797E-05 7.7144E-05 3.8841E-05 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03";

    #[test]
    fn fixed_width_row_maps_b_and_plasma() {
        let table = parse_text(LINE).unwrap();
        assert_eq!(table.names.len(), 32);
        assert_eq!(table.rows.len(), 1);
        let row = &table.rows[0];
        assert_eq!(row[1], Some(1974.0));
        assert_eq!(row[2], Some(348.0));
        assert_eq!(row[4], Some(1622.0));
        assert_eq!(row[13], Some(-1.23));
        assert_eq!(row[15], Some(4.55));
        assert_eq!(row[16], Some(574.9));
        assert_eq!(row[19], Some(2.5));
        assert_eq!(row[20], Some(110352.0));
        assert_eq!(row[21], Some(1.3517e-4));
        assert_eq!(row[25], None);
        assert!(row[0].unwrap() > 42000.0);
    }

    #[test]
    fn helios2_fixture_shares_the_layout() {
        let line = "1976  17  0 1637   0.98   -4.7   40.3  359.3     3.79    -2.95    -0.46    -3.79     2.89    -0.74     6.45  425.4   -0.1    2.4   6.6   91933. 1.1888E-04 3.4654E-05 1.2699E-04 2.9333E-05 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03";
        let table = parse_text(line).unwrap();
        assert_eq!(table.rows[0][1], Some(1976.0));
        assert_eq!(table.rows[0][16], Some(425.4));
    }

    #[test]
    fn fill_rows_and_short_lines_stay_absent() {
        let fill = "1974 348  0 9999 999.99 9999.9 9999.9 9999.9 99999.99 99999.99 99999.99 99999.99 99999.99 99999.99 99999.99 9999.9 9999.9 9999.9 999.9 9999999. 9.9999E+04 9.9999E+04 9.9999E+04 9.9999E+04 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03 9.999E+03";
        assert!(parse_text(fill).is_none());
        assert!(parse_text("1974 348 0 1622 0.98").is_none());
    }
}
