use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::fetch_raw_bytes_headers;
use omegaflow::archivar::jstr;
use omegaflow::archivar::load_env;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::parse_json;
use omegaflow::archivar::render_headers;
use omegaflow::archivar::secret_resolves_void;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::netcdf::NetcdfFile;
use std::collections::HashMap;

const NETLOC: &str = "api.dataplatform.knmi.nl";
const BASE: &str = "https://api.dataplatform.knmi.nl/open-data/v1";
const COMPILER: &str = "tools/harvest/src/bin/knmi_compiler.rs";
const DEFAULT_DATASET: &str = "10-minute-in-situ-meteorological-observations";
const DEFAULT_VERSION: &str = "1.0";

const MAGIC: [u8; 4] = *b"KNMI";
const REC_BYTES: usize = 26 * 8;
const NFIELDS: usize = 4;

const SECS_PER_DAY: f64 = 86400.0;
const CADENCE_S: f64 = 600.0;
const TTL_S: f64 = 86400.0;

const KERNEL_GAUSSIAN_INVERSE_SQUARE: u8 = 1;
const KERNEL_EXPONENTIAL_DECAY: u8 = 4;
const KERNEL_PATCH_LEVY: u8 = 5;

const FORCE_THERMAL: u8 = 5;
const FORCE_DIFFUSION: u8 = 6;
const FORCE_ADVECTIVE: u8 = 7;

const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];

struct FieldSpec {
    key: &'static str,
    var: &'static str,
    kernel: u8,
    kernel_token: &'static str,
    force: u8,
    force_token: &'static str,
    unit: &'static str,
    parse: fn(f64) -> Option<f64>,
}

const FIELDS: [FieldSpec; NFIELDS] = [
    FieldSpec {
        key: "knmi_air_temperature",
        var: "ta",
        kernel: KERNEL_EXPONENTIAL_DECAY,
        kernel_token: "exponential-decay",
        force: FORCE_THERMAL,
        force_token: "thermal",
        unit: "K",
        parse: celsius_to_kelvin,
    },
    FieldSpec {
        key: "knmi_wind_speed",
        var: "ff",
        kernel: KERNEL_PATCH_LEVY,
        kernel_token: "patch-levy",
        force: FORCE_ADVECTIVE,
        force_token: "advective",
        unit: "m/s",
        parse: wind_speed_m_s,
    },
    FieldSpec {
        key: "knmi_wind_gust",
        var: "fx",
        kernel: KERNEL_PATCH_LEVY,
        kernel_token: "patch-levy",
        force: FORCE_ADVECTIVE,
        force_token: "advective",
        unit: "m/s",
        parse: wind_speed_m_s,
    },
    FieldSpec {
        key: "knmi_precipitation_intensity",
        var: "rg",
        kernel: KERNEL_GAUSSIAN_INVERSE_SQUARE,
        kernel_token: "gaussian-inverse-square",
        force: FORCE_DIFFUSION,
        force_token: "diffusion",
        unit: "kg/(m^2 s)",
        parse: mm_per_hour_to_si,
    },
];

fn celsius_to_kelvin(raw: f64) -> Option<f64> {
    let k = raw + 273.15;
    k.is_finite()
        .then_some(k)
        .filter(|k| (150.0..=350.0).contains(k))
}

fn wind_speed_m_s(raw: f64) -> Option<f64> {
    raw.is_finite()
        .then_some(raw)
        .filter(|v| (0.0..=120.0).contains(v))
}

fn mm_per_hour_to_si(raw: f64) -> Option<f64> {
    let flux = raw / 3600.0;
    flux.is_finite()
        .then_some(flux)
        .filter(|v| (0.0..=0.1).contains(v))
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_or(args: &[String], name: &str, fallback: &str) -> String {
    match arg_value(args, name) {
        Some(v) => v,
        None => fallback.to_string(),
    }
}

fn usage() -> &'static str {
    "usage: knmi_compiler --body <name> --out <dir> [--dataset <name>] [--version <ver>] [--input <path>] [--name <slug>] [--ephemeris <path|url>] [--emit-field-names] [--ci-mode]"
}

fn emit_field_names() {
    for f in &FIELDS {
        println!(
            "field {} {} {} {} {} {} 0.0 0.0",
            f.key, f.key, f.kernel_token, f.force_token, f.unit, CADENCE_S as u64
        );
    }
}

fn is_hdf5(bytes: &[u8]) -> bool {
    bytes.starts_with(&HDF5_MAGIC)
}

fn safe_token(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

fn slug_of(dataset: &str, version: &str) -> String {
    format!("{dataset}_{version}")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn latest_filename(text: &str) -> Option<String> {
    let json = parse_json(text)?;
    jstr(&json, "files.0.filename")
}

fn download_url(text: &str) -> Option<String> {
    let json = parse_json(text)?;
    jstr(&json, "temporaryDownloadUrl")
}

fn epoch_seconds(units: &str) -> Option<(f64, f64)> {
    let since = units.find("since")?;
    let factor = if units.starts_with("days") {
        86400.0
    } else if units.starts_with("hours") {
        3600.0
    } else if units.starts_with("minutes") {
        60.0
    } else if units.starts_with("seconds") {
        1.0
    } else {
        return None;
    };
    let rest = units[since + 5..].trim();
    let date = rest.split(|c: char| c == 'T' || c == ' ').next()?;
    let mut it = date.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    let base = days_from_civil(y, m, d)? as f64 * SECS_PER_DAY;
    Some((factor, base))
}

fn attr_text(nc: &NetcdfFile, name: &str, key: &str) -> Option<String> {
    nc.var(name)?
        .attrs
        .iter()
        .find(|a| a.name == key)
        .and_then(|a| nc.attr_text(a))
}

fn var_fill(nc: &NetcdfFile, name: &str) -> Option<f64> {
    nc.var(name)?
        .attrs
        .iter()
        .find(|a| a.name == "_FillValue")
        .and_then(|a| nc.attr_num(a))
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
    lat: &[f64],
    lon: &[f64],
    height: Option<&[f64]>,
    values: &[Option<Vec<f64>>],
    fills: &[Option<f64>],
    tdb: f64,
    eph: &HashMap<String, BodyEphemeris>,
    body: &str,
) -> (Vec<Vec<[f64; 26]>>, usize, usize) {
    let mut per_field: Vec<Vec<[f64; 26]>> = (0..NFIELDS).map(|_| Vec::new()).collect();
    let mut frame_void = 0usize;
    let mut fill_void = 0usize;
    let stations = lat.len().min(lon.len());
    for j in 0..stations {
        let Some(&latj) = lat.get(j) else {
            continue;
        };
        let Some(&lonj) = lon.get(j) else {
            continue;
        };
        let alt = match height.and_then(|h| h.get(j).copied()) {
            Some(a) => a,
            None => {
                frame_void += 1;
                continue;
            }
        };
        let Some(pos) = body_fixed_to_icrs(body, latj, lonj, alt, tdb, eph) else {
            frame_void += 1;
            continue;
        };
        for i in 0..NFIELDS {
            let Some(vals) = values.get(i).and_then(Option::as_ref) else {
                continue;
            };
            let Some(&raw) = vals.get(j) else {
                continue;
            };
            if let Some(fill) = fills.get(i).and_then(|f| *f) {
                if raw == fill {
                    fill_void += 1;
                    continue;
                }
            }
            if let Some(v) = (FIELDS[i].parse)(raw) {
                per_field[i].push(record(pos, v, tdb, FIELDS[i].kernel, FIELDS[i].force));
            }
        }
    }
    (per_field, frame_void, fill_void)
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--emit-field-names") {
        emit_field_names();
        return Ok(());
    }
    if args.is_empty() || args.iter().any(|a| a == "--help") {
        println!("{}", usage());
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let dataset = arg_or(args, "--dataset", DEFAULT_DATASET);
    let version = arg_or(args, "--version", DEFAULT_VERSION);
    let input = arg_value(args, "--input");

    let env = load_env();
    if input.is_none() && secret_resolves_void("{KNMI_API_KEY}", &env) {
        println!("pending — KNMI_API_KEY absent");
        return Ok(());
    }

    let body = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;
    let out_dir = arg_value(args, "--out").ok_or_else(|| {
        "--out <dir> is required — the asset is never written to a guessed path".to_string()
    })?;

    let (bytes, origin) = if let Some(path) = input {
        let raw = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        (raw, None)
    } else {
        if !safe_token(&dataset) || !safe_token(&version) {
            return Err(format!(
                "dataset '{dataset}' or version '{version}' carries a token outside the API path alphabet — refused"
            ));
        }
        let headers = render_headers(
            &[("Authorization".to_string(), "{KNMI_API_KEY}".to_string())],
            &env,
        );
        let list_url = format!(
            "{BASE}/datasets/{dataset}/versions/{version}/files?maxKeys=1&orderBy=created&sorting=desc"
        );
        let list_body = fetch_raw_bytes_headers(&list_url, &headers).ok_or_else(|| {
            format!("{list_url}: fetch returned void — the file list stays unread")
        })?;
        let filename = latest_filename(&String::from_utf8_lossy(&list_body)).ok_or_else(|| {
            format!(
                "{list_url}: the response carries no files[0].filename — the dataset stays unread"
            )
        })?;
        let url_url = format!("{BASE}/datasets/{dataset}/versions/{version}/files/{filename}/url");
        let url_body = fetch_raw_bytes_headers(&url_url, &headers).ok_or_else(|| {
            format!("{url_url}: fetch returned void — the download URL stays unread")
        })?;
        let download = download_url(&String::from_utf8_lossy(&url_body)).ok_or_else(|| {
            format!(
                "{url_url}: the response carries no temporaryDownloadUrl — the file stays unread"
            )
        })?;
        let raw = fetch_raw_bytes(&download).ok_or_else(|| {
            format!("{filename}: fetch returned void — the observation stays unread (0 honored)")
        })?;
        (
            raw,
            Some(format!(
                "{BASE}/datasets/{dataset}/versions/{version}/files"
            )),
        )
    };

    if is_hdf5(&bytes) {
        return Err(
            "the file is a NetCDF-4/HDF5 container — the classic reader carries no record layout for it (parser-gap)".to_string(),
        );
    }
    let nc = NetcdfFile::parse(&bytes)
        .map_err(|note| format!("the NetCDF container stays unread: {note:?}"))?;

    let time = nc
        .values_numeric(&bytes, "time")
        .ok_or_else(|| "the time variable stays unread — no observation epoch".to_string())?;
    let t_raw = *time
        .first()
        .ok_or_else(|| "the time variable carries no value — no observation epoch".to_string())?;
    let units = attr_text(&nc, "time", "units")
        .ok_or_else(|| "the time variable carries no units content".to_string())?;
    let (factor, base) =
        epoch_seconds(&units).ok_or_else(|| format!("the time units '{units}' carry no epoch"))?;
    let unix = base + t_raw * factor;
    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no date→TDB step".to_string())?;
    let tdb = lsk
        .unix_to_tdb(unix)
        .ok_or_else(|| format!("unix {unix} lies outside the leap table — no TDB step"))?;

    let lat = nc
        .values_numeric(&bytes, "lat")
        .ok_or_else(|| "the lat variable stays unread — no station position".to_string())?;
    let lon = nc
        .values_numeric(&bytes, "lon")
        .ok_or_else(|| "the lon variable stays unread — no station position".to_string())?;
    let height = nc.values_numeric(&bytes, "height");

    let values: Vec<Option<Vec<f64>>> = FIELDS
        .iter()
        .map(|f| nc.values_numeric(&bytes, f.var))
        .collect();
    let fills: Vec<Option<f64>> = FIELDS.iter().map(|f| var_fill(&nc, f.var)).collect();

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

    let (per_field, frame_void, fill_void) = compile(
        &lat,
        &lon,
        height.as_deref(),
        &values,
        &fills,
        tdb,
        &eph,
        &body,
    );
    let total: usize = per_field.iter().map(Vec::len).sum();
    if total == 0 {
        return Err(format!(
            "no record left the harvest — {} stations read; frame_void {}, fill_void {}",
            lat.len(),
            frame_void,
            fill_void
        ));
    }

    let slug = arg_or(args, "--name", &slug_of(&dataset, &version));
    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("create {out_dir} returned void: {e}"))?;
    eprintln!(
        "knmi: {slug} ({} stations, tdb {tdb}); frame_void {}, fill_void {}",
        lat.len(),
        frame_void,
        fill_void
    );

    for (i, recs) in per_field.iter().enumerate() {
        if recs.is_empty() {
            eprintln!(
                "knmi: {} carries no measured station — skipped",
                FIELDS[i].key
            );
            continue;
        }
        let name = format!("knmi_{slug}_{}.bin", FIELDS[i].key);
        let path = format!("{out_dir}/{name}");
        let bin = write_bin(recs);
        std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned void: {e}"))?;
        let roundtrip = read_bin(&bin)
            .ok_or_else(|| format!("{path}: roundtrip parse void — the asset stays unverified"))?;
        eprintln!(
            "knmi: {} records, {} B -> {path} (roundtrip {roundtrip})",
            recs.len(),
            bin.len()
        );
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format knmi");
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
        eprintln!("knmi_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_filename_reads_the_first_file() {
        let text =
            r#"{"files":[{"filename":"KMDS__OPER_P___10M_OBS_L2_202401010000.nc","size":1}]}"#;
        assert_eq!(
            latest_filename(text).as_deref(),
            Some("KMDS__OPER_P___10M_OBS_L2_202401010000.nc")
        );
        assert_eq!(latest_filename("{}"), None);
    }

    #[test]
    fn download_url_reads_the_temporary_link() {
        let text = r#"{"temporaryDownloadUrl":"https://example.invalid/x.nc","filename":"x.nc"}"#;
        assert_eq!(
            download_url(text).as_deref(),
            Some("https://example.invalid/x.nc")
        );
        assert_eq!(download_url("{}"), None);
    }

    #[test]
    fn epoch_seconds_reads_the_1950_base() {
        let (factor, base) = epoch_seconds("seconds since 1950-01-01 00:00:00").expect("units");
        assert_eq!(factor, 1.0);
        assert_eq!(
            base,
            days_from_civil(1950, 1, 1).expect("1950") as f64 * SECS_PER_DAY
        );
    }

    #[test]
    fn conversions_honor_zero_and_drop_absurd() {
        assert_eq!(celsius_to_kelvin(0.0), Some(273.15));
        assert_eq!(celsius_to_kelvin(-9999.0), None);
        assert_eq!(wind_speed_m_s(0.0), Some(0.0));
        assert_eq!(wind_speed_m_s(500.0), None);
        assert_eq!(mm_per_hour_to_si(0.0), Some(0.0));
        assert_eq!(mm_per_hour_to_si(3600.0), Some(1.0));
        assert_eq!(mm_per_hour_to_si(-1.0), None);
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record(
            [1.0, 2.0, 3.0],
            0.5,
            8.0e8,
            KERNEL_EXPONENTIAL_DECAY,
            FORCE_THERMAL,
        );
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 2.0);
        assert_eq!(r[2], 3.0);
        assert_eq!(r[3], 0.5);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], CADENCE_S);
        assert_eq!(r[8], KERNEL_EXPONENTIAL_DECAY as f64);
        assert_eq!(r[9], FORCE_THERMAL as f64);
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
                KERNEL_PATCH_LEVY,
                FORCE_ADVECTIVE,
            ),
            record(
                [4.0, 5.0, 6.0],
                0.7,
                8.0e8 + 1.0,
                KERNEL_PATCH_LEVY,
                FORCE_ADVECTIVE,
            ),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(read_bin(&bytes), Some(2));
        assert!(read_bin(b"X").is_none());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
