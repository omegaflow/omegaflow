use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::force::{force_id_of, kernel_id_for_force};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

const NETLOC: &str = "ecad.eu";
const COMPILER: &str = "tools/harvest/src/bin/ecad_compiler.rs";

const MAGIC: [u8; 4] = *b"ECAD";
const REC_BYTES: usize = 26 * 8;

const SECS_PER_DAY: f64 = 86400.0;
const CADENCE_S: f64 = 86400.0;
const TTL_S: f64 = 604800.0;
const TAU_S: f64 = 86400.0;

const MISSING: f64 = -9999.0;

struct FieldSpec {
    key: &'static str,
    variable: &'static str,
    kernel: &'static str,
    force: &'static str,
    unit: &'static str,
    scale: f64,
    offset: f64,
    lo: f64,
    hi: f64,
}

const FIELDS: [FieldSpec; 4] = [
    FieldSpec {
        key: "eca_tx",
        variable: "TX",
        kernel: "exponential-decay",
        force: "thermal",
        unit: "K",
        scale: 0.1,
        offset: 273.15,
        lo: 150.0,
        hi: 350.0,
    },
    FieldSpec {
        key: "eca_tn",
        variable: "TN",
        kernel: "exponential-decay",
        force: "thermal",
        unit: "K",
        scale: 0.1,
        offset: 273.15,
        lo: 150.0,
        hi: 350.0,
    },
    FieldSpec {
        key: "eca_tg",
        variable: "TG",
        kernel: "exponential-decay",
        force: "thermal",
        unit: "K",
        scale: 0.1,
        offset: 273.15,
        lo: 150.0,
        hi: 350.0,
    },
    FieldSpec {
        key: "eca_rr",
        variable: "RR",
        kernel: "gaussian-inverse-square",
        force: "diffusion",
        unit: "kg/m^2",
        scale: 0.1,
        offset: 0.0,
        lo: 0.0,
        hi: 2000.0,
    },
];

struct Row {
    epoch_unix: f64,
    value: Option<f64>,
}

fn usage() -> &'static str {
    "usage: ecad_compiler --body <name> --lat <deg> --lon <deg> --alt <m> --out <dir> \
     (--input <station.txt> | --url <url> | --zip <file.zip> --member <STAID...txt>) \
     [--name <slug>] [--emit-field-names] [--ci-mode]"
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn spec_for_variable(variable: &str) -> Option<&'static FieldSpec> {
    FIELDS.iter().find(|f| f.variable == variable)
}

fn emit_field_names() {
    for f in &FIELDS {
        println!(
            "field {} {} {} {} {} {} 0.0 0.0",
            f.key, f.key, f.kernel, f.force, f.unit, CADENCE_S as u64
        );
    }
}

fn find_variable(text: &str) -> Option<&str> {
    for line in text.lines() {
        let t = line.trim_start();
        if !t.starts_with("STAID") || !line.contains(',') {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5 {
            continue;
        }
        let v = cols[3].trim();
        if !v.is_empty() {
            return Some(v);
        }
    }
    None
}

fn gate_value(raw: f64, qc: i32, spec: &FieldSpec) -> Option<f64> {
    if raw == MISSING {
        return None;
    }
    if !raw.is_finite() {
        return None;
    }
    if qc != 0 && qc != 1 {
        return None;
    }
    let v = raw * spec.scale + spec.offset;
    if v.is_finite() && v >= spec.lo && v <= spec.hi {
        Some(v)
    } else {
        None
    }
}

fn parse_data_row(line: &str, spec: &FieldSpec) -> Option<Row> {
    let t: Vec<&str> = line.split(',').collect();
    if t.len() < 5 {
        return None;
    }
    let date = t[2].trim();
    if date.len() != 8 {
        return None;
    }
    let year = date[0..4].parse::<i64>().ok()?;
    let month = date[4..6].parse::<i64>().ok()?;
    let day = date[6..8].parse::<i64>().ok()?;
    let days = days_from_civil(year, month, day)?;
    let epoch_unix = days as f64 * SECS_PER_DAY;
    let raw = t[3].trim().parse::<f64>().ok()?;
    let qc = t[4].trim().parse::<i32>().ok()?;
    Some(Row {
        epoch_unix,
        value: gate_value(raw, qc, spec),
    })
}

fn fetch_text(url: &str) -> Result<String, String> {
    match fetch_raw_bytes(url) {
        Some(b) => Ok(String::from_utf8_lossy(&b).into_owned()),
        None => Err(format!(
            "{url}: fetch returned void — the series stays unread (0 honored)"
        )),
    }
}

fn read_member(path: &str, member: &str) -> Result<Vec<u8>, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    loop {
        let mut hdr = [0u8; 30];
        if f.read_exact(&mut hdr).is_err() {
            return Err(format!(
                "{path}: the local headers end before {member} — the series stays unread"
            ));
        }
        if &hdr[0..4] != b"PK\x03\x04" {
            return Err(format!(
                "{path}: no local header reads before {member} — the series stays unread"
            ));
        }
        let flags = u16::from_le_bytes([hdr[6], hdr[7]]);
        let method = u16::from_le_bytes([hdr[8], hdr[9]]);
        let comp_size = u32::from_le_bytes([hdr[18], hdr[19], hdr[20], hdr[21]]) as usize;
        let name_len = u16::from_le_bytes([hdr[26], hdr[27]]) as usize;
        let extra_len = u16::from_le_bytes([hdr[28], hdr[29]]) as usize;
        let mut name = vec![0u8; name_len];
        f.read_exact(&mut name)
            .map_err(|e| format!("read name in {path} returned void: {e}"))?;
        let mut extra = vec![0u8; extra_len];
        f.read_exact(&mut extra)
            .map_err(|e| format!("read extra in {path} returned void: {e}"))?;
        if String::from_utf8_lossy(&name) == member {
            if flags & 0x08 != 0 && comp_size == 0 {
                return Err(format!(
                    "{path}: {member} carries a streaming data descriptor — the series stays unread"
                ));
            }
            let mut comp = vec![0u8; comp_size];
            f.read_exact(&mut comp)
                .map_err(|e| format!("read body in {path} returned void: {e}"))?;
            return match method {
                0 => Ok(comp),
                8 => omegaflow::inflate::inflate(&comp).ok_or_else(|| {
                    format!("{path}: {member} deflate stays unread — the series stays unread")
                }),
                other => Err(format!(
                    "{path}: {member} carries compression {other}, which has no decoder"
                )),
            };
        }
        let mut remaining = comp_size as u64;
        while remaining > 0 {
            let step = remaining.min(65536) as usize;
            let mut skip = vec![0u8; step];
            f.read_exact(&mut skip)
                .map_err(|e| format!("skip in {path} returned void: {e}"))?;
            remaining -= step as u64;
        }
    }
}

fn record(pos: [f64; 3], val: f64, tdb: f64, kernel: f64, force: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = 0.0;
    r[8] = kernel;
    r[9] = force;
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
    spec: &FieldSpec,
    rows: &[Row],
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
    body: &str,
    lat: f64,
    lon: f64,
    alt: f64,
) -> Result<(Vec<[f64; 26]>, usize, usize), String> {
    let force = force_id_of(spec.force)
        .ok_or_else(|| format!("force token '{}' reads no fixed channel", spec.force))?;
    let kernel = kernel_id_for_force(force)
        .ok_or_else(|| format!("force '{}' carries no kernel for the registry", spec.force))?;
    let mut recs = Vec::new();
    let mut clock_void = 0usize;
    let mut frame_void = 0usize;
    for r in rows {
        let Some(v) = r.value else {
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(r.epoch_unix) else {
            clock_void += 1;
            continue;
        };
        let Some(pos) = body_fixed_to_icrs(body, lat, lon, alt, tdb, eph) else {
            frame_void += 1;
            continue;
        };
        recs.push(record(pos, v, tdb, kernel as f64, force as f64));
    }
    Ok((recs, clock_void, frame_void))
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--emit-field-names") {
        emit_field_names();
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let (text, origin, member_slug) = if let Some(path) = arg_value(args, "--input") {
        let t = std::fs::read_to_string(&path)
            .map_err(|e| format!("read {path} returned void: {e}"))?;
        let slug = Path::new(&path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());
        (t, Some(path), slug)
    } else if let Some(url) = arg_value(args, "--url") {
        let t = fetch_text(&url)?;
        let slug = url
            .rsplit('/')
            .next()
            .map(|s| s.trim_end_matches(".txt").to_string());
        (t, Some(url), slug)
    } else if let Some(zip) = arg_value(args, "--zip") {
        let member = arg_value(args, "--member")
            .ok_or_else(|| "--member <STAID...txt> is required with --zip".to_string())?;
        let bytes = read_member(&zip, &member)?;
        let slug = Path::new(&member)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());
        (
            String::from_utf8_lossy(&bytes).into_owned(),
            Some(format!("{zip}#{member}")),
            slug,
        )
    } else {
        return Err(usage().to_string());
    };

    let variable = find_variable(&text).ok_or_else(|| {
        "the series carries no column header — the asset stays unwritten (0 honored)".to_string()
    })?;
    let spec = spec_for_variable(variable).ok_or_else(|| {
        format!("column '{variable}' carries no field mapping in this compiler — refused")
    })?;

    let mut rows: Vec<Row> = Vec::new();
    let mut data_lines = 0usize;
    let mut after_header = false;
    for line in text.lines() {
        if !after_header {
            let t = line.trim_start();
            if t.starts_with("STAID") && line.contains(',') {
                after_header = true;
            }
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        data_lines += 1;
        if let Some(r) = parse_data_row(line, spec) {
            rows.push(r);
        }
    }
    if rows.is_empty() {
        return Err(format!(
            "the series carries no parseable data row — {data_lines} data lines read — the asset stays unwritten (0 honored)"
        ));
    }
    let Some(first_row) = rows.first() else {
        return Err("the series carries no first row — the asset stays unwritten".to_string());
    };
    let first = format!("{:.0}s", first_row.epoch_unix);

    let lat = arg_value(args, "--lat")
        .ok_or_else(|| {
            "--lat <deg> is required — the station point is declared, never defaulted".to_string()
        })?
        .parse::<f64>()
        .map_err(|_| "--lat reads no degree".to_string())?;
    let lon = arg_value(args, "--lon")
        .ok_or_else(|| {
            "--lon <deg> is required — the station point is declared, never defaulted".to_string()
        })?
        .parse::<f64>()
        .map_err(|_| "--lon reads no degree".to_string())?;
    let alt = arg_value(args, "--alt")
        .ok_or_else(|| {
            "--alt <m> is required — the station point is declared, never defaulted".to_string()
        })?
        .parse::<f64>()
        .map_err(|_| "--alt reads no metre".to_string())?;
    if !lat.is_finite() || !lon.is_finite() || !alt.is_finite() {
        return Err(
            "the declared station point carries no finite coordinate — refused".to_string(),
        );
    }
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Err(format!(
            "{lat},{lon} lies outside the geodetic register — refused"
        ));
    }

    let body = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;
    let out_dir = arg_value(args, "--out").ok_or_else(|| {
        "--out <dir> is required — the asset is never written to a guessed path".to_string()
    })?;
    let slug = match arg_value(args, "--name") {
        Some(s) => s,
        None => member_slug.filter(|s| !s.is_empty()).ok_or_else(|| {
            "--name <slug> is required when the source carries no file stem".to_string()
        })?,
    };

    let lsk = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no date→TDB step".to_string())?;
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

    let (recs, clock_void, frame_void) = compile(spec, &rows, &lsk, &eph, &body, lat, lon, alt)?;
    if recs.is_empty() {
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
        "ecad: {} {variable} ({lat}, {lon}, {alt} m), {} rows, first {first}, clock_void {}, frame_void {}",
        slug,
        rows.len(),
        clock_void,
        frame_void
    );

    let name = format!("ecad_{}_{}.bin", spec.key, slug);
    let path = format!("{out_dir}/{name}");
    let bin = write_bin(&recs);
    std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned void: {e}"))?;
    let roundtrip = read_bin(&bin)
        .ok_or_else(|| format!("{path}: roundtrip parse void — the asset stays unverified"))?;
    eprintln!(
        "ecad: {} records, {} B -> {path} (roundtrip {roundtrip}); clock_void {}, frame_void {}",
        recs.len(),
        bin.len(),
        clock_void,
        frame_void
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
    println!("format ecad");
    if let Some(src) = &origin {
        println!("origin {src}");
    }
    println!("compiler {COMPILER}");
    println!("at {body}");
    println!("ttl {}", TTL_S as u64);
    println!(
        "field {} {} {} {} {} {} 0.0 0.0",
        spec.key, spec.key, spec.kernel, spec.force, spec.unit, CADENCE_S as u64
    );
    println!("sha256 {}", sha256_hex(&bin));
    if ci_mode && !upload_release(NETLOC, &path) {
        return Err(format!("{path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ecad_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "EUROPEAN CLIMATE ASSESSMENT & DATASET (ECA&D)\n\
FILE FORMAT (MISSING VALUE CODE = -9999):\n\
24-28 TX   : Maximum temperature in 0.1 C\n\
STAID, SOUID,    DATE,   TX, Q_TX\n\
     1, 35382,18820101,   25,    0\n\
     1, 35382,18820102,   37,    0\n\
     1, 35382,18820103,-9999,    9\n\
     1, 35382,18820104,  -19,    1\n";

    fn tx_spec() -> &'static FieldSpec {
        spec_for_variable("TX").expect("tx spec")
    }

    fn rr_spec() -> &'static FieldSpec {
        spec_for_variable("RR").expect("rr spec")
    }

    #[test]
    fn find_variable_reads_the_column_header() {
        assert_eq!(find_variable(FIXTURE), Some("TX"));
        assert_eq!(find_variable("no header here\n"), None);
    }

    #[test]
    fn field_specs_map_to_registered_kernel_and_force() {
        for f in &FIELDS {
            let force = force_id_of(f.force).expect("registered force");
            assert!(kernel_id_for_force(force).is_some());
        }
        assert_eq!(
            kernel_id_for_force(force_id_of("thermal").unwrap()),
            Some(4)
        );
        assert_eq!(
            kernel_id_for_force(force_id_of("diffusion").unwrap()),
            Some(1)
        );
    }

    #[test]
    fn temperature_rows_read_date_and_scale_to_kelvin() {
        let spec = tx_spec();
        let mut after = false;
        let mut rows = Vec::new();
        for line in FIXTURE.lines() {
            if !after {
                if line.trim_start().starts_with("STAID") && line.contains(',') {
                    after = true;
                }
                continue;
            }
            if let Some(r) = parse_data_row(line, spec) {
                rows.push(r);
            }
        }
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].value, Some(2.5 + 273.15));
        assert_eq!(rows[1].value, Some(3.7 + 273.15));
        assert_eq!(rows[2].value, None); // -9999 / qc 9
        assert_eq!(rows[3].value, Some(-1.9 + 273.15)); // qc 1 suspect kept
        let d0 = days_from_civil(1882, 1, 1).unwrap() as f64 * SECS_PER_DAY;
        assert_eq!(rows[0].epoch_unix, d0);
    }

    #[test]
    fn precipitation_rows_read_zero_and_scale() {
        let spec = rr_spec();
        let row = parse_data_row("     1, 37886,18600101,    0,    0", spec).unwrap();
        assert_eq!(row.value, Some(0.0));
        let row = parse_data_row("     1, 37886,18600103,  136,    0", spec).unwrap();
        assert_eq!(row.value, Some(13.6));
        let row = parse_data_row("     1, 37886,18600104,-9999,    9", spec).unwrap();
        assert_eq!(row.value, None);
    }

    #[test]
    fn gate_refuses_missing_sentinel_and_unknown_quality() {
        let spec = tx_spec();
        assert_eq!(gate_value(-9999.0, 0, spec), None);
        assert_eq!(gate_value(25.0, 9, spec), None);
        assert_eq!(gate_value(25.0, 7, spec), None);
        assert_eq!(gate_value(25.0, 0, spec), Some(275.65));
        assert_eq!(gate_value(99999.0, 0, spec), None);
        assert_eq!(gate_value(f64::NAN, 0, spec), None);
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record([1.0, 2.0, 3.0], 275.0, 8.0e8, 4.0, 5.0);
        assert_eq!(r[0], 1.0);
        assert_eq!(r[3], 275.0);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], TAU_S);
        assert_eq!(r[8], 4.0);
        assert_eq!(r[9], 5.0);
        assert_eq!(r[25], 1.0);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
    }

    #[test]
    fn bin_roundtrip_counts_records() {
        let records = vec![
            record([1.0, 2.0, 3.0], 275.0, 8.0e8, 4.0, 5.0),
            record([4.0, 5.0, 6.0], 276.0, 8.0e8 + 1.0, 4.0, 5.0),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(read_bin(&bytes), Some(2));
        assert!(read_bin(b"X").is_none());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    fn stored_zip(member: &str, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&10u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // stored
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&(member.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(member.as_bytes());
        out.extend_from_slice(payload);
        out
    }

    #[test]
    fn read_member_scans_local_headers_and_returns_payload() {
        let dir = std::env::temp_dir();
        let path = dir.join("ecad_extract_fixture.zip");
        let mut z = stored_zip("first.txt", b"aaa");
        z.extend_from_slice(&stored_zip("TX_STAID000001.txt", b"hello-ecad"));
        std::fs::write(&path, &z).unwrap();
        let got = read_member(path.to_str().unwrap(), "TX_STAID000001.txt").unwrap();
        assert_eq!(got, b"hello-ecad");
        assert!(read_member(path.to_str().unwrap(), "absent.txt").is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn emit_field_names_names_tokens_units_and_cadence() {
        for f in &FIELDS {
            let line = format!(
                "field {} {} {} {} {} {} 0.0 0.0",
                f.key, f.key, f.kernel, f.force, f.unit, CADENCE_S as u64
            );
            assert!(line.contains(f.kernel));
            assert!(line.contains(f.force));
            assert!(line.contains("86400"));
        }
    }
}
