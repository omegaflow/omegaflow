use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::inflate::inflate;
use std::collections::HashMap;

const NETLOC: &str = "opendata.dwd.de";
const BASE: &str =
    "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/recent";
const COMPILER: &str = "tools/harvest/src/bin/dwd_cdc_compiler.rs";

const MAGIC: [u8; 4] = *b"DWDC";
const REC_BYTES: usize = 26 * 8;
const NFIELDS: usize = 9;

const SECS_PER_DAY: f64 = 86400.0;
const CADENCE_S: f64 = 86400.0;
const TTL_S: f64 = 2592000.0;

const MISSING: f64 = -999.0;

const ZIP_LOCAL_SIGNATURE: u32 = 0x04034b50;
const ZIP_METHOD_STORED: u16 = 0;
const ZIP_METHOD_DEFLATE: u16 = 8;
const ZIP_FLAG_DATA_DESCRIPTOR: u16 = 0x0008;

const KERNEL_GAUSSIAN_INVERSE_SQUARE: u8 = 1;
const KERNEL_EXPONENTIAL_DECAY: u8 = 4;
const KERNEL_PATCH_LEVY: u8 = 5;

const FORCE_ACOUSTIC: u8 = 2;
const FORCE_THERMAL: u8 = 5;
const FORCE_DIFFUSION: u8 = 6;
const FORCE_ADVECTIVE: u8 = 7;

struct FieldCol {
    key: &'static str,
    col: usize,
    kernel: u8,
    kernel_token: &'static str,
    force: u8,
    force_token: &'static str,
    unit: &'static str,
    parse: fn(f64) -> Option<f64>,
}

const FIELDS: [FieldCol; NFIELDS] = [
    FieldCol {
        key: "dwd_air_temperature_mean",
        col: 13,
        kernel: KERNEL_EXPONENTIAL_DECAY,
        kernel_token: "exponential-decay",
        force: FORCE_THERMAL,
        force_token: "thermal",
        unit: "K",
        parse: celsius_to_kelvin,
    },
    FieldCol {
        key: "dwd_air_temperature_max",
        col: 15,
        kernel: KERNEL_EXPONENTIAL_DECAY,
        kernel_token: "exponential-decay",
        force: FORCE_THERMAL,
        force_token: "thermal",
        unit: "K",
        parse: celsius_to_kelvin,
    },
    FieldCol {
        key: "dwd_air_temperature_min",
        col: 16,
        kernel: KERNEL_EXPONENTIAL_DECAY,
        kernel_token: "exponential-decay",
        force: FORCE_THERMAL,
        force_token: "thermal",
        unit: "K",
        parse: celsius_to_kelvin,
    },
    FieldCol {
        key: "dwd_surface_pressure",
        col: 12,
        kernel: KERNEL_GAUSSIAN_INVERSE_SQUARE,
        kernel_token: "gaussian-inverse-square",
        force: FORCE_ACOUSTIC,
        force_token: "acoustic",
        unit: "Pa",
        parse: hpa_to_pascal,
    },
    FieldCol {
        key: "dwd_wind_speed_mean",
        col: 4,
        kernel: KERNEL_PATCH_LEVY,
        kernel_token: "patch-levy",
        force: FORCE_ADVECTIVE,
        force_token: "advective",
        unit: "m/s",
        parse: wind_speed_m_s,
    },
    FieldCol {
        key: "dwd_wind_gust_max",
        col: 3,
        kernel: KERNEL_PATCH_LEVY,
        kernel_token: "patch-levy",
        force: FORCE_ADVECTIVE,
        force_token: "advective",
        unit: "m/s",
        parse: wind_speed_m_s,
    },
    FieldCol {
        key: "dwd_precipitation_height",
        col: 6,
        kernel: KERNEL_GAUSSIAN_INVERSE_SQUARE,
        kernel_token: "gaussian-inverse-square",
        force: FORCE_DIFFUSION,
        force_token: "diffusion",
        unit: "m",
        parse: mm_to_metre,
    },
    FieldCol {
        key: "dwd_relative_humidity",
        col: 14,
        kernel: KERNEL_GAUSSIAN_INVERSE_SQUARE,
        kernel_token: "gaussian-inverse-square",
        force: FORCE_DIFFUSION,
        force_token: "diffusion",
        unit: "1",
        parse: percent_to_fraction,
    },
    FieldCol {
        key: "dwd_vapour_pressure",
        col: 11,
        kernel: KERNEL_GAUSSIAN_INVERSE_SQUARE,
        kernel_token: "gaussian-inverse-square",
        force: FORCE_DIFFUSION,
        force_token: "diffusion",
        unit: "Pa",
        parse: vapour_pa,
    },
];

fn celsius_to_kelvin(raw: f64) -> Option<f64> {
    let k = raw + 273.15;
    k.is_finite()
        .then_some(k)
        .filter(|k| (150.0..=350.0).contains(k))
}

fn hpa_to_pascal(raw: f64) -> Option<f64> {
    let pa = raw * 100.0;
    pa.is_finite()
        .then_some(pa)
        .filter(|pa| (30000.0..=110000.0).contains(pa))
}

fn wind_speed_m_s(raw: f64) -> Option<f64> {
    raw.is_finite()
        .then_some(raw)
        .filter(|v| (0.0..=120.0).contains(v))
}

fn mm_to_metre(raw: f64) -> Option<f64> {
    let m = raw * 0.001;
    m.is_finite()
        .then_some(m)
        .filter(|m| (0.0..=5.0).contains(m))
}

fn percent_to_fraction(raw: f64) -> Option<f64> {
    let f = raw * 0.01;
    f.is_finite()
        .then_some(f)
        .filter(|f| (0.0..=1.0).contains(f))
}

fn vapour_pa(raw: f64) -> Option<f64> {
    let pa = raw * 100.0;
    pa.is_finite()
        .then_some(pa)
        .filter(|pa| (0.0..=10000.0).contains(pa))
}

fn parse_value(field: &FieldCol, token: &str) -> Option<f64> {
    let raw = token.trim().parse::<f64>().ok()?;
    if raw == MISSING {
        return None;
    }
    (field.parse)(raw)
}

struct Row {
    year: i64,
    month: i64,
    day: i64,
    values: [Option<f64>; NFIELDS],
}

struct Station {
    lat: f64,
    lon: f64,
    alt: f64,
}

fn usage() -> &'static str {
    "usage: dwd_cdc_compiler --body <name> --station <NNNNN> [--out <dir>] [--input <zip> | --url <url>] [--ephemeris <path|url>] [--emit-field-names] [--ci-mode]"
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
            "field {} {} {} {} {} {} 0.0 0.0",
            f.key, f.key, f.kernel_token, f.force_token, f.unit, CADENCE_S as u64
        );
    }
}

fn station_url(station: &str) -> Result<String, String> {
    if station.len() != 5 || !station.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!(
            "station '{station}' is no five-digit DWD station id — refused"
        ));
    }
    Ok(format!("{BASE}/tageswerte_KL_{station}_akt.zip"))
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

fn parse_date(token: &str) -> Option<(i64, i64, i64)> {
    let d: i64 = token.trim().parse().ok()?;
    if d < 10000101 {
        return None;
    }
    let year = d / 10000;
    let month = (d / 100) % 100;
    let day = d % 100;
    if day < 1 || day > days_in_month(year, month)? {
        return None;
    }
    Some((year, month, day))
}

fn parse_row(line: &str) -> Option<Row> {
    let t: Vec<&str> = line.split(';').collect();
    if t.len() < 18 {
        return None;
    }
    let (year, month, day) = parse_date(t[1])?;
    let mut values = [None; NFIELDS];
    for (i, f) in FIELDS.iter().enumerate() {
        values[i] = t.get(f.col).and_then(|s| parse_value(f, s));
    }
    if values.iter().all(Option::is_none) {
        return None;
    }
    Some(Row {
        year,
        month,
        day,
        values,
    })
}

fn parse_station(text: &str) -> Option<Station> {
    let mut chosen: Option<Station> = None;
    for row in text.lines().skip(1) {
        let t: Vec<&str> = row.split(';').collect();
        if t.len() < 7 {
            continue;
        }
        let alt = match t[1].trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let lat = match t[2].trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let lon = match t[3].trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !(lat.is_finite() && lon.is_finite() && alt.is_finite()) {
            continue;
        }
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        let station = Station { lat, lon, alt };
        let current = t[5].trim().is_empty();
        chosen = Some(station);
        if current {
            break;
        }
    }
    chosen
}

fn zip_entry_bytes(data: &[u8], prefix: &str, suffix: &str) -> Result<(String, Vec<u8>), String> {
    let mut i = 0usize;
    while i + 30 <= data.len() {
        let sig = u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
        if sig != ZIP_LOCAL_SIGNATURE {
            break;
        }
        let flags = u16::from_le_bytes([data[i + 6], data[i + 7]]);
        let method = u16::from_le_bytes([data[i + 8], data[i + 9]]);
        let comp =
            u32::from_le_bytes([data[i + 18], data[i + 19], data[i + 20], data[i + 21]]) as usize;
        let nlen = u16::from_le_bytes([data[i + 26], data[i + 27]]) as usize;
        let elen = u16::from_le_bytes([data[i + 28], data[i + 29]]) as usize;
        let name_start = i + 30;
        let name_end = name_start
            .checked_add(nlen)
            .ok_or("the archive entry name overflows")?;
        let name = std::str::from_utf8(
            data.get(name_start..name_end)
                .ok_or("the archive entry name reads past the container")?,
        )
        .map_err(|_| "the archive entry name is not UTF-8".to_string())?;
        let data_start = name_end
            .checked_add(elen)
            .ok_or("the archive entry extra field overflows")?;
        let comp_end = data_start
            .checked_add(comp)
            .ok_or("the archive entry size overflows")?;
        if name.starts_with(prefix) && name.ends_with(suffix) {
            if flags & ZIP_FLAG_DATA_DESCRIPTOR != 0 {
                return Err(format!(
                    "archive entry '{name}' carries a data descriptor — the size stays unread"
                ));
            }
            let raw = data
                .get(data_start..comp_end)
                .ok_or("the archive entry reads past the container")?;
            let body = match method {
                ZIP_METHOD_STORED => raw.to_vec(),
                ZIP_METHOD_DEFLATE => inflate(raw)
                    .ok_or_else(|| format!("archive entry '{name}' deflate stream stays unread"))?,
                other => {
                    return Err(format!(
                        "archive entry '{name}' carries compression {other}, unread"
                    ));
                }
            };
            return Ok((name.to_string(), body));
        }
        i = comp_end;
    }
    Err(format!("no archive entry matches {prefix}*{suffix}"))
}

fn record(pos: [f64; 3], val: f64, tdb: f64, kernel: u8, force: u8) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = CADENCE_S;
    r[7] = 0.0;
    r[8] = kernel as f64;
    r[9] = force as f64;
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
    station: &Station,
    rows: &[Row],
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
    body: &str,
) -> (Vec<Vec<[f64; 26]>>, usize, usize) {
    let mut per_field: Vec<Vec<[f64; 26]>> = (0..NFIELDS).map(|_| Vec::new()).collect();
    let mut clock_void = 0usize;
    let mut frame_void = 0usize;
    for r in rows {
        let Some(days) = days_from_civil(r.year, r.month, r.day) else {
            clock_void += 1;
            continue;
        };
        let epoch = days as f64 * SECS_PER_DAY;
        let Some(tdb) = lsk.unix_to_tdb(epoch) else {
            clock_void += 1;
            continue;
        };
        let Some(pos) = body_fixed_to_icrs(body, station.lat, station.lon, station.alt, tdb, eph)
        else {
            frame_void += 1;
            continue;
        };
        for (i, f) in FIELDS.iter().enumerate() {
            if let Some(v) = r.values[i] {
                per_field[i].push(record(pos, v, tdb, f.kernel, f.force));
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
    let station_arg = arg_value(args, "--station");
    let out_dir = arg_value(args, "--out").ok_or_else(|| {
        "--out <dir> is required — the asset is never written to a guessed path".to_string()
    })?;

    let (zip_bytes, origin, station_id) = if let Some(path) = arg_value(args, "--input") {
        let bytes = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        let id = station_arg.ok_or_else(|| {
            "--station <NNNNN> is required with --input — the receiver identity is never guessed"
                .to_string()
        })?;
        if id.len() != 5 || !id.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!(
                "station '{id}' is no five-digit DWD station id — refused"
            ));
        }
        (bytes, None, id)
    } else if let Some(url) = arg_value(args, "--url") {
        let bytes = fetch_raw_bytes(&url).ok_or_else(|| {
            format!("{url}: fetch returned void — the archive stays unread (0 honored)")
        })?;
        let id = station_arg.ok_or_else(|| {
            "--station <NNNNN> is required with --url — the receiver identity is never guessed"
                .to_string()
        })?;
        if id.len() != 5 || !id.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!(
                "station '{id}' is no five-digit DWD station id — refused"
            ));
        }
        (bytes, Some(url), id)
    } else {
        let id = station_arg.ok_or_else(|| usage().to_string())?;
        let url = station_url(&id)?;
        let bytes = fetch_raw_bytes(&url).ok_or_else(|| {
            format!("{url}: fetch returned void — the archive stays unread (0 honored)")
        })?;
        (bytes, Some(url), id)
    };

    let (_, product_body) = zip_entry_bytes(&zip_bytes, "produkt_klima_tag_", ".txt")?;
    let (_, geo_body) = zip_entry_bytes(&zip_bytes, "Metadaten_Geographie_", ".txt")?;
    let product = String::from_utf8_lossy(&product_body);
    let geography = String::from_utf8_lossy(&geo_body);

    let Some(station) = parse_station(&geography) else {
        return Err(
            "the geography record carries no finite geodetic point — the asset stays unwritten"
                .into(),
        );
    };

    let mut rows: Vec<Row> = Vec::new();
    let mut malformed = 0usize;
    for line in product.lines().skip(1) {
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
            "the product carries no parseable daily row — {malformed} malformed — the asset stays unwritten (0 honored)"
        ));
    }
    let first = match rows.first() {
        Some(r) => format!("{}-{:02}-{:02}", r.year, r.month, r.day),
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

    let (per_field, clock_void, frame_void) = compile(&station, &rows, &lsk, &eph, &body);
    let total: usize = per_field.iter().map(Vec::len).sum();
    if total == 0 {
        return Err(format!(
            "no record left the harvest — {} rows read; clock_void {}, frame_void {}",
            rows.len(),
            clock_void,
            frame_void
        ));
    }
    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("create {out_dir} returned void: {e}"))?;
    eprintln!(
        "dwd_cdc: station {station_id} ({}, {}, {} m), {} rows, first {first}, {malformed} malformed",
        station.lat,
        station.lon,
        station.alt,
        rows.len(),
    );

    for (i, recs) in per_field.iter().enumerate() {
        if recs.is_empty() {
            eprintln!(
                "dwd_cdc: {} carries no measured day — skipped",
                FIELDS[i].key
            );
            continue;
        }
        let name = format!("dwd_cdc_{station_id}_{}.bin", FIELDS[i].key);
        let path = format!("{out_dir}/{name}");
        let bin = write_bin(recs);
        std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned void: {e}"))?;
        let roundtrip = read_bin(&bin)
            .ok_or_else(|| format!("{path}: roundtrip parse void — the asset stays unverified"))?;
        eprintln!(
            "dwd_cdc: {} records, {} B -> {path} (roundtrip {roundtrip}); clock_void {}, frame_void {}",
            recs.len(),
            bin.len(),
            clock_void,
            frame_void
        );
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format dwd_cdc");
        if let Some(src) = &origin {
            println!("origin {src}");
        }
        println!("compiler {COMPILER}");
        println!("at {body}");
        println!("ttl {}", TTL_S as u64);
        println!(
            "field {} {} {} {} {} {} 0.0 0.0",
            FIELDS[i].key,
            FIELDS[i].key,
            FIELDS[i].kernel_token,
            FIELDS[i].force_token,
            FIELDS[i].unit,
            CADENCE_S as u64
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
        eprintln!("dwd_cdc_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCT: &str = "STATIONS_ID;MESS_DATUM;QN_3;  FX;  FM;QN_4; RSK;RSKF; SDK;SHK_TAG;  NM; VPM;  PM; TMK; UPM; TXK; TNK; TGK;eor\n\
          44;20260824;-999;-999;-999;    1;   0.0;   0;-999;-999;  -999;  12.4;    -999;   15.8;   71.33;   21.8;   11.4;    8.1;eor\n\
          44;20260825;-999;-999;-999;    1;   3.5;   4;-999;-999;  -999;  13.0;    -999;   16.2;   88.13;   24.0;   13.1;   10.6;eor\n";

    const GEOGRAPHY: &str = "Stations_id;Stationshoehe;Geogr.Breite;Geogr.Laenge;von_datum;bis_datum;Stationsname\n\
     44;   45.00; 52.8900;  8.2300;19450801;19481231;Grossenkneten\n\
     44;   44.00; 52.9336;  8.2370;20070401;        ;Grossenkneten\n";

    fn product_rows(text: &str) -> Vec<Row> {
        text.lines().skip(1).filter_map(parse_row).collect()
    }

    fn close(a: Option<f64>, b: f64) -> bool {
        a.map(|x| (x - b).abs() < 1e-9).unwrap_or(false)
    }

    #[test]
    fn row_reads_the_measured_columns() {
        let rows = product_rows(PRODUCT);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].year, 2026);
        assert_eq!(rows[0].month, 8);
        assert_eq!(rows[0].day, 24);
        assert!(close(rows[0].values[0], 288.95));
        assert!(close(rows[0].values[1], 294.95));
        assert!(close(rows[0].values[2], 284.55));
        assert!(close(rows[0].values[7], 0.7133));
        assert_eq!(rows[0].values[6], Some(0.0));
        assert_eq!(rows[0].values[3], None);
    }

    #[test]
    fn missing_sentinel_is_absent_not_zero() {
        assert_eq!(parse_value(&FIELDS[0], "  -999"), None);
        assert_eq!(parse_value(&FIELDS[4], "-999"), None);
        assert_eq!(parse_value(&FIELDS[4], "nonsense"), None);
    }

    #[test]
    fn physical_gates_hold_the_ranges() {
        assert!(close(celsius_to_kelvin(15.8), 288.95));
        assert_eq!(celsius_to_kelvin(2000.0), None);
        assert!(close(hpa_to_pascal(1013.25), 101325.0));
        assert_eq!(hpa_to_pascal(0.0), None);
        assert_eq!(wind_speed_m_s(0.0), Some(0.0));
        assert_eq!(wind_speed_m_s(200.0), None);
        assert_eq!(mm_to_metre(0.0), Some(0.0));
        assert!(close(percent_to_fraction(88.13), 0.8813));
        assert_eq!(percent_to_fraction(150.0), None);
        assert!(close(vapour_pa(12.4), 1240.0));
    }

    #[test]
    fn geography_selects_the_current_coordinate() {
        let station = parse_station(GEOGRAPHY).unwrap();
        assert_eq!(station.lat, 52.9336);
        assert_eq!(station.lon, 8.2370);
        assert_eq!(station.alt, 44.0);
        assert_eq!(parse_station("Stations_id;X\n nope"), None);
    }

    #[test]
    fn station_url_reads_the_measured_layout() {
        assert_eq!(
            station_url("00044").unwrap(),
            "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/recent/tageswerte_KL_00044_akt.zip"
        );
        assert!(station_url("44").is_err());
        assert!(station_url("0004X").is_err());
    }

    #[test]
    fn stored_zip_entry_is_read_by_prefix_and_suffix() {
        fn stored_entry(name: &str, body: &[u8]) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(&ZIP_LOCAL_SIGNATURE.to_le_bytes());
            out.extend_from_slice(&20u16.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&ZIP_METHOD_STORED.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&(body.len() as u32).to_le_bytes());
            out.extend_from_slice(&(body.len() as u32).to_le_bytes());
            out.extend_from_slice(&(name.len() as u16).to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(name.as_bytes());
            out.extend_from_slice(body);
            out
        }
        let mut zip = stored_entry("Metadaten_Geographie_00044.txt", GEOGRAPHY.as_bytes());
        zip.extend_from_slice(&stored_entry(
            "produkt_klima_tag_20250408_20261009_00044.txt",
            PRODUCT.as_bytes(),
        ));
        let (name, body) = zip_entry_bytes(&zip, "produkt_klima_tag_", ".txt").unwrap();
        assert!(name.starts_with("produkt_klima_tag_"));
        assert_eq!(String::from_utf8(body).unwrap(), PRODUCT);
        let (gname, _) = zip_entry_bytes(&zip, "Metadaten_Geographie_", ".txt").unwrap();
        assert_eq!(gname, "Metadaten_Geographie_00044.txt");
        assert!(zip_entry_bytes(&zip, "produkt_klima_tag_", ".csv").is_err());
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record(
            [1.0, 2.0, 3.0],
            0.5,
            8.0e8,
            KERNEL_PATCH_LEVY,
            FORCE_ADVECTIVE,
        );
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 2.0);
        assert_eq!(r[2], 3.0);
        assert_eq!(r[3], 0.5);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], CADENCE_S);
        assert_eq!(r[8], KERNEL_PATCH_LEVY as f64);
        assert_eq!(r[9], FORCE_ADVECTIVE as f64);
        assert_eq!(r[25], 1.0);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
    }

    #[test]
    fn bin_roundtrip_counts_records() {
        let records = vec![
            record(
                [1.0, 2.0, 3.0],
                0.4,
                8.0e8,
                KERNEL_GAUSSIAN_INVERSE_SQUARE,
                FORCE_THERMAL,
            ),
            record(
                [4.0, 5.0, 6.0],
                0.7,
                8.0e8 + 1.0,
                KERNEL_GAUSSIAN_INVERSE_SQUARE,
                FORCE_THERMAL,
            ),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(read_bin(&bytes), Some(2));
        assert!(read_bin(b"X").is_none());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn field_names_emit_the_si_contract() {
        assert_eq!(FIELDS.len(), NFIELDS);
        assert_eq!(
            format!(
                "field {} {} {} {} {} {} 0.0 0.0",
                FIELDS[0].key,
                FIELDS[0].key,
                FIELDS[0].kernel_token,
                FIELDS[0].force_token,
                FIELDS[0].unit,
                CADENCE_S as u64
            ),
            "field dwd_air_temperature_mean dwd_air_temperature_mean exponential-decay thermal K 86400 0.0 0.0"
        );
        assert_eq!(FIELDS[3].unit, "Pa");
        assert_eq!(FIELDS[3].force_token, "acoustic");
        assert_eq!(FIELDS[6].unit, "m");
    }
}
