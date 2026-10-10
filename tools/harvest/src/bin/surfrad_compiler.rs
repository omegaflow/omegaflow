use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use std::collections::HashMap;

const NETLOC: &str = "gml.noaa.gov";
const BASE: &str = "https://gml.noaa.gov/aftp/data/radiation/surfrad";
const COMPILER: &str = "tools/harvest/src/bin/surfrad_compiler.rs";

const MAGIC: [u8; 4] = *b"SRFR";
const REC_BYTES: usize = 26 * 8;
const NFIELDS: usize = 2;

const SECS_PER_DAY: f64 = 86400.0;
const SECS_PER_HOUR: f64 = 3600.0;
const TTL_S: f64 = 604800.0;
const TAU_S: f64 = 3600.0;
const KERNEL_INVERSE_SQUARE: f64 = 0.0;
const FORCE_EM: f64 = 0.0;

const MISSING: f64 = -9999.9;
const SOLAR_MAX_WM2: f64 = 1500.0;

struct FieldCol {
    key: &'static str,
    value_idx: usize,
    qc_idx: usize,
}

const FIELDS: [FieldCol; NFIELDS] = [
    FieldCol {
        key: "surfrad_shortwave_down",
        value_idx: 8,
        qc_idx: 9,
    },
    FieldCol {
        key: "surfrad_direct_normal",
        value_idx: 12,
        qc_idx: 13,
    },
];

struct Row {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    values: [Option<f64>; NFIELDS],
}

struct HourMean {
    epoch_unix: f64,
    values: [Option<f64>; NFIELDS],
}

fn usage() -> &'static str {
    "usage: surfrad_compiler --station <xxx> --year <YYYY> --doy <DDD> [--out <dir>] [--input <path> | --url <url>] [--name <slug>] [--emit-field-names] [--ci-mode]"
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn emit_field_names() {
    for f in &FIELDS {
        println!(
            "field {} {} inverse-square em W/m2 {} 0.0 0.0",
            f.key, f.key, TAU_S as u64
        );
    }
}

fn solar_gate(v: Option<f64>) -> Option<f64> {
    match v {
        Some(x) if x.is_finite() && (0.0..=SOLAR_MAX_WM2).contains(&x) => Some(x),
        _ => None,
    }
}

fn parse_solar(token: Option<&str>, qc: Option<&str>) -> Option<f64> {
    let raw = token?.trim().parse::<f64>().ok()?;
    if raw == MISSING {
        return None;
    }
    let flag: i32 = qc?.trim().parse().ok()?;
    if flag == 1 {
        return None;
    }
    solar_gate(Some(raw))
}

fn days_in_month(year: i64, month: i64) -> Option<i64> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            29
        } else {
            28
        }),
        _ => None,
    }
}

fn parse_header(line: &str) -> Option<(f64, f64, f64)> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 3 {
        return None;
    }
    let lat = t[0].parse::<f64>().ok()?;
    let lon = t[1].parse::<f64>().ok()?;
    let alt = t[2].parse::<f64>().ok()?;
    if !(lat.is_finite() && lon.is_finite() && alt.is_finite()) {
        return None;
    }
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    Some((lat, lon, alt))
}

fn parse_row(line: &str) -> Option<Row> {
    let t: Vec<&str> = line.split_whitespace().collect();
    if t.len() < 14 {
        return None;
    }
    let year = t[0].parse::<i64>().ok()?;
    let month = t[2].parse::<i64>().ok()?;
    let day = t[3].parse::<i64>().ok()?;
    let hour = t[4].parse::<i64>().ok()?;
    if !(0..=23).contains(&hour) {
        return None;
    }
    if day < 1 || day > days_in_month(year, month)? {
        return None;
    }
    let mut values = [None; NFIELDS];
    for (i, f) in FIELDS.iter().enumerate() {
        values[i] = parse_solar(t.get(f.value_idx).copied(), t.get(f.qc_idx).copied());
    }
    Some(Row {
        year,
        month,
        day,
        hour,
        values,
    })
}

fn station_url(station: &str, year: &str, doy: &str) -> Result<String, String> {
    if station.len() != 3 || !station.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!(
            "station '{station}' is no three-character SURFRAD code — refused"
        ));
    }
    if year.len() != 4 || !year.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("year '{year}' is no four-digit year — refused"));
    }
    let d: i64 = doy
        .parse()
        .map_err(|_| format!("doy '{doy}' is no day number — refused"))?;
    if !(1..=366).contains(&d) {
        return Err(format!("doy '{doy}' lies outside 1..=366 — refused"));
    }
    let yy = &year[2..];
    Ok(format!("{BASE}/{station}/{year}/{station}{yy}{d:03}.dat"))
}

fn fetch_text(url: &str) -> Result<String, String> {
    match fetch_raw_bytes(url) {
        Some(b) => Ok(String::from_utf8_lossy(&b).into_owned()),
        None => Err(format!(
            "{url}: fetch returned void — the day stays unread (0 honored)"
        )),
    }
}

fn aggregate(rows: &[Row]) -> Vec<HourMean> {
    let mut map: HashMap<(i64, i64, i64, i64), (f64, [f64; NFIELDS], [u64; NFIELDS])> =
        HashMap::new();
    for r in rows {
        let Some(days) = days_from_civil(r.year, r.month, r.day) else {
            continue;
        };
        let epoch = days as f64 * SECS_PER_DAY + r.hour as f64 * SECS_PER_HOUR;
        let key = (r.year, r.month, r.day, r.hour);
        let entry = map
            .entry(key)
            .or_insert((epoch, [0.0; NFIELDS], [0; NFIELDS]));
        for i in 0..NFIELDS {
            if let Some(v) = r.values[i] {
                entry.1[i] += v;
                entry.2[i] += 1;
            }
        }
    }
    let mut keys: Vec<(i64, i64, i64, i64)> = map.keys().copied().collect();
    keys.sort();
    let mut out = Vec::with_capacity(keys.len());
    for k in keys {
        let (epoch, sums, counts) = map[&k];
        let mut values = [None; NFIELDS];
        for i in 0..NFIELDS {
            if counts[i] > 0 {
                let mean = sums[i] / counts[i] as f64;
                if mean.is_finite() {
                    values[i] = Some(mean);
                }
            }
        }
        out.push(HourMean {
            epoch_unix: epoch,
            values,
        });
    }
    out
}

fn record(pos: [f64; 3], val: f64, tdb: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = 0.0;
    r[8] = KERNEL_INVERSE_SQUARE;
    r[9] = FORCE_EM;
    r[25] = 1.0;
    r
}

fn write_bin(records: &[[f64; 26]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn read_bin(data: &[u8]) -> Option<usize> {
    if data.len() < 8 || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * REC_BYTES {
        return None;
    }
    Some(count)
}

fn compile(
    lat: f64,
    lon: f64,
    alt: f64,
    rows: &[Row],
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
    body: &str,
) -> (Vec<Vec<[f64; 26]>>, usize, usize) {
    let hours = aggregate(rows);
    let mut per_field: Vec<Vec<[f64; 26]>> = (0..NFIELDS).map(|_| Vec::new()).collect();
    let mut clock_void = 0usize;
    let mut frame_void = 0usize;
    for h in &hours {
        let Some(tdb) = lsk.unix_to_tdb(h.epoch_unix) else {
            clock_void += 1;
            continue;
        };
        let Some(pos) = body_fixed_to_icrs(body, lat, lon, alt, tdb, eph) else {
            frame_void += 1;
            continue;
        };
        for i in 0..NFIELDS {
            if let Some(v) = h.values[i] {
                per_field[i].push(record(pos, v, tdb));
            }
        }
    }
    (per_field, clock_void, frame_void)
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--emit-field-names") {
        emit_field_names();
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let station = arg_value(args, "--station");
    let year = arg_value(args, "--year");
    let doy = arg_value(args, "--doy");

    let (body, origin) = if let Some(path) = arg_value(args, "--input") {
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("read {path} returned void: {e}"))?;
        (text, None)
    } else if let Some(url) = arg_value(args, "--url") {
        (fetch_text(&url)?, Some(url))
    } else {
        match (&station, &year, &doy) {
            (Some(s), Some(y), Some(d)) => {
                let url = station_url(s, y, d)?;
                (fetch_text(&url)?, Some(url))
            }
            _ => return Err(usage().to_string()),
        }
    };

    let slug = match (&station, &year, &doy) {
        (Some(s), Some(y), Some(d)) => {
            let dd: u32 = d
                .parse()
                .map_err(|_| format!("--doy '{d}' is no day number"))?;
            if y.len() != 4 {
                return Err(format!("--year '{y}' is no four-digit year"));
            }
            format!("{s}_{y}{dd:03}")
        }
        _ => arg_value(args, "--name").ok_or_else(|| {
            "--name <slug> is required with --url/--input and no --station/--year/--doy".to_string()
        })?,
    };
    let out_dir = arg_value(args, "--out")
        .ok_or_else(|| "--out <dir> is required — the asset is never written to a guessed path".to_string())?;

    let mut lines = body.lines();
    let Some(name_line) = lines.next() else {
        return Err("the day carries no station header — the asset stays unwritten".into());
    };
    let station_name = name_line.trim();
    let header_line = lines.next().ok_or_else(|| {
        "the day carries no coordinate header — the asset stays unwritten".to_string()
    })?;
    let Some((lat, lon, alt)) = parse_header(header_line) else {
        return Err(format!(
            "the coordinate header '{header_line}' carries no finite geodetic point — the asset stays unwritten"
        ));
    };

    let mut rows: Vec<Row> = Vec::new();
    let mut malformed = 0usize;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        match parse_row(line) {
            Some(r) => rows.push(r),
            None => malformed += 1,
        }
    }
    if rows.is_empty() {
        return Err(format!(
            "the day carries no parseable data row — {malformed} malformed — the asset stays unwritten (0 honored)"
        ));
    }
    let first = match rows.first() {
        Some(r) => format!("{}-{:02}-{:02} {:02}h", r.year, r.month, r.day, r.hour),
        None => String::new(),
    };

    let lsk = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no date→TDB step".to_string())?;
    let body = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;
    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(src) if src.starts_with("http") => {
            fetch_raw_bytes(&src).ok_or_else(|| format!("body ephemeris fetch void ({src})"))?
        }
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("body ephemeris read {path}: {e}"))?
        }
        None => fetch_raw_bytes(&body_url(&body))
            .ok_or_else(|| "body ephemeris fetch void (CDN) — no ICRS frame".to_string())?,
    };
    let body_eph = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the body ephemeris binary stays unread — no ICRS frame".to_string())?;
    let eph = HashMap::from([(body.clone(), body_eph)]);

    let (per_field, clock_void, frame_void) = compile(lat, lon, alt, &rows, &lsk, &eph, &body);
    let total: usize = per_field.iter().map(Vec::len).sum();
    if total == 0 {
        return Err(format!(
            "no record left the harvest — {} rows read, {} hours aggregated; clock_void {}, frame_void {}",
            rows.len(),
            aggregate(&rows).len(),
            clock_void,
            frame_void
        ));
    }
    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("create {out_dir} returned void: {e}"))?;
    eprintln!(
        "surfrad: {station_name} ({lat}, {lon}, {alt} m), {} rows, first {first}, {} hours, {malformed} malformed",
        rows.len(),
        aggregate(&rows).len(),
    );

    for (i, recs) in per_field.iter().enumerate() {
        if recs.is_empty() {
            eprintln!(
                "surfrad: {} carries no measured hour — skipped",
                FIELDS[i].key
            );
            continue;
        }
        let name = format!("surfrad_{slug}_{}.bin", FIELDS[i].key);
        let path = format!("{out_dir}/{name}");
        let bin = write_bin(recs);
        std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned void: {e}"))?;
        let roundtrip = read_bin(&bin)
            .ok_or_else(|| format!("{path}: roundtrip parse void — the asset stays unverified"))?;
        eprintln!(
            "surfrad: {} records, {} B -> {path} (roundtrip {roundtrip}); clock_void {}, frame_void {}",
            recs.len(),
            bin.len(),
            clock_void,
            frame_void
        );
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format surfrad");
        if let Some(src) = &origin {
            println!("origin {src}");
        }
        println!("compiler {COMPILER}");
        println!("at {body}");
        println!("ttl {}", TTL_S as u64);
        println!(
            "field {} {} inverse-square em W/m2 {} 0.0 0.0",
            FIELDS[i].key, FIELDS[i].key, TAU_S as u64
        );
        println!("sha256 {}", sha256_hex(&bin));
        if ci_mode && !upload_release(NETLOC, &path) {
            return Err(format!("{path}: CDN upload returned void"));
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("surfrad_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = " Table Mountain\n\
   40.125 -105.237 1689 m version 1\n\
 2024   1  1  1  0  0  0.000  93.00   100.0 0     0.0 0    90.0 0     0.7 0   261.3 0   275.0 0   275.0 0   303.8 0   274.6 0   274.6 0     0.0 0     0.3 0     0.7 0   -42.5 0   -41.8 0     0.7 0    59.6 0     1.6 0    48.1 0   832.4 0\n\
 2024   1  1  1  0  1  0.017  93.10   200.0 0     0.0 0   110.0 0     0.7 0   260.0 0   275.0 0   275.0 0   303.5 0   274.6 0   274.6 0     0.0 0     0.2 0     0.7 0   -42.9 0   -42.2 0     0.7 0    59.8 0     1.6 0    48.3 0   832.5 0\n\
 2024   1  1  1  1  0  0.033  93.20    -0.4 0     0.0 0     0.0 0     0.7 0   259.9 0   275.0 0   275.0 0   303.1 0   274.6 0   274.6 0     0.0 0     0.2 0     0.7 0   -43.1 0   -42.4 0     0.6 0    59.9 0     1.5 0    48.3 0   832.5 0\n";

    fn data_rows(text: &str) -> Vec<Row> {
        let mut lines = text.lines();
        lines.next();
        lines.next();
        lines.filter_map(parse_row).collect()
    }

    #[test]
    fn header_reads_the_geodetic_point() {
        assert_eq!(
            parse_header("   40.125 -105.237 1689 m version 1"),
            Some((40.125, -105.237, 1689.0))
        );
        assert_eq!(parse_header("nonsense"), None);
        assert_eq!(parse_header("999.0 0.0 0 m"), None);
    }

    #[test]
    fn row_reads_the_measured_column_indices() {
        let rows = data_rows(FIXTURE);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].year, 2024);
        assert_eq!(rows[0].month, 1);
        assert_eq!(rows[0].day, 1);
        assert_eq!(rows[0].hour, 0);
        assert_eq!(rows[0].values[0], Some(100.0));
        assert_eq!(rows[0].values[1], Some(90.0));
        assert_eq!(rows[2].values[0], None);
        assert_eq!(rows[2].values[1], Some(0.0));
    }

    #[test]
    fn solar_gate_honors_zero_and_drops_absent_negative_and_absurd() {
        assert_eq!(parse_solar(Some("0.0"), Some("0")), Some(0.0));
        assert_eq!(parse_solar(Some("250.4"), Some("0")), Some(250.4));
        assert_eq!(parse_solar(Some("-0.4"), Some("0")), None);
        assert_eq!(parse_solar(Some("-9999.9"), Some("1")), None);
        assert_eq!(parse_solar(Some("100.0"), Some("1")), None);
        assert_eq!(parse_solar(Some("2000.0"), Some("0")), None);
        assert_eq!(parse_solar(Some("nan"), Some("0")), None);
        assert_eq!(parse_solar(None, Some("0")), None);
    }

    #[test]
    fn aggregate_averages_the_hour_and_skips_absent_fields() {
        let rows = data_rows(FIXTURE);
        let hours = aggregate(&rows);
        assert_eq!(hours.len(), 2);
        assert_eq!(hours[0].values[0], Some(150.0));
        assert_eq!(hours[0].values[1], Some(100.0));
        assert_eq!(hours[1].values[0], None);
        assert_eq!(hours[1].values[1], Some(0.0));
        assert_eq!(hours[1].epoch_unix - hours[0].epoch_unix, SECS_PER_HOUR);
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record([1.0, 2.0, 3.0], 0.5, 8.0e8);
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 2.0);
        assert_eq!(r[2], 3.0);
        assert_eq!(r[3], 0.5);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], TAU_S);
        assert_eq!(r[8], KERNEL_INVERSE_SQUARE);
        assert_eq!(r[9], FORCE_EM);
        assert_eq!(r[25], 1.0);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
    }

    #[test]
    fn bin_roundtrip_counts_records() {
        let records = vec![
            record([1.0, 2.0, 3.0], 0.4, 8.0e8),
            record([4.0, 5.0, 6.0], 0.7, 8.0e8 + 1.0),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(read_bin(&bytes), Some(2));
        assert!(read_bin(b"X").is_none());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn station_url_reads_the_measured_layout() {
        assert_eq!(
            station_url("tbl", "2024", "1").unwrap(),
            "https://gml.noaa.gov/aftp/data/radiation/surfrad/tbl/2024/tbl24001.dat"
        );
        assert!(station_url("toolong", "2024", "1").is_err());
        assert!(station_url("tb", "2024", "1").is_err());
        assert!(station_url("tbl", "24", "1").is_err());
        assert!(station_url("tbl", "2024", "0").is_err());
    }
}
