use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::motion::{BodyEphemeris, body_fixed_to_icrs};
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::tiff::{GeoTransform, TiffImage, parse_tiff};
use omegaflow::cdn::{body_url, upload_release};
use omegaflow::force::{force_id_of, kernel_id_for_force};
use omegaflow::inflate::zip_members;
use std::collections::HashMap;

const NETLOC: &str = "geodata.ucdavis.edu";
const BASE: &str = "https://geodata.ucdavis.edu/climate/worldclim/2_1/base";
const COMPILER: &str = "tools/harvest/src/bin/worldclim_compiler.rs";
const FORMAT: &str = "worldclim";

const MAGIC: [u8; 4] = *b"WCLM";
const REC_BYTES: usize = 26 * 8;

const SECS_PER_DAY: f64 = 86400.0;
const SECS_PER_HOUR: f64 = 3600.0;
const SECS_PER_MONTH: f64 = 2592000.0;
const TTL_S: f64 = 315360000.0;
const TAU_S: f64 = SECS_PER_MONTH;

const CLIM_MID_YEAR: i64 = 1985;
const CLIM_MID_DAY: i64 = 15;
const CLIM_MID_HOUR: i64 = 12;

const KELVIN_OFFSET: f64 = 273.15;
const TEMP_MIN_C: f64 = -90.0;
const TEMP_MAX_C: f64 = 60.0;
const PREC_MIN_MM: f64 = 0.0;
const PREC_MAX_MM: f64 = 65535.0;
const NODATA_ABS: f64 = 3.0e38;
const REFERENCE_SURFACE_ALT: f64 = 0.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Variable {
    Tavg,
    Tmin,
    Tmax,
    Prec,
}

impl Variable {
    fn token(self) -> &'static str {
        match self {
            Variable::Tavg => "tavg",
            Variable::Tmin => "tmin",
            Variable::Tmax => "tmax",
            Variable::Prec => "prec",
        }
    }

    fn key(self) -> &'static str {
        match self {
            Variable::Tavg => "worldclim_tavg",
            Variable::Tmin => "worldclim_tmin",
            Variable::Tmax => "worldclim_tmax",
            Variable::Prec => "worldclim_prec",
        }
    }

    fn unit(self) -> &'static str {
        match self {
            Variable::Prec => "mm",
            _ => "K",
        }
    }

    fn force(self) -> &'static str {
        match self {
            Variable::Prec => "diffusion",
            _ => "thermal",
        }
    }

    fn kernel(self) -> &'static str {
        match self {
            Variable::Prec => "gaussian-inverse-square",
            _ => "exponential-decay",
        }
    }

    fn from_token(token: &str) -> Option<Variable> {
        match token {
            "tavg" => Some(Variable::Tavg),
            "tmin" => Some(Variable::Tmin),
            "tmax" => Some(Variable::Tmax),
            "prec" => Some(Variable::Prec),
            _ => None,
        }
    }
}

fn usage() -> &'static str {
    "usage: worldclim_compiler --body <name> --out <dir> (--input <zip|dir> | --url <zip-url>) \
     [--name <slug>] [--stride <n>] [--limit <n>] [--ephemeris <path|url>] \
     [--emit-field-names] [--ci-mode]"
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_step(args: &[String], name: &str, default: usize) -> Result<usize, String> {
    match arg_value(args, name) {
        Some(v) => {
            let n = v
                .parse::<usize>()
                .map_err(|_| format!("{name} {v} carries no step"))?;
            if n == 0 {
                return Err(format!("{name} carries no positive step"));
            }
            Ok(n)
        }
        None => Ok(default),
    }
}

fn field_line(v: Variable) -> String {
    format!(
        "field {} {} {} {} {} {} 0.0 0.0",
        v.key(),
        v.key(),
        v.kernel(),
        v.force(),
        v.unit(),
        SECS_PER_MONTH as u64
    )
}

fn emit_field_names() {
    for v in [
        Variable::Tavg,
        Variable::Tmin,
        Variable::Tmax,
        Variable::Prec,
    ] {
        println!("{}", field_line(v));
    }
}

fn variable_from_name(name: &str) -> Option<Variable> {
    for part in name.split(['_', '.', '/']) {
        if let Some(v) = Variable::from_token(part) {
            return Some(v);
        }
    }
    None
}

fn month_from_name(name: &str) -> Option<u8> {
    let stem = name.rsplit('/').next()?;
    let base = stem.strip_suffix(".tif")?;
    let last = base.rsplit('_').next()?;
    match last.parse::<u8>() {
        Ok(m) if (1..=12).contains(&m) => Some(m),
        _ => None,
    }
}

fn resolution_from_name(name: &str) -> Option<String> {
    for part in name.split(['_', '.', '/']) {
        let mut chars = part.chars();
        if matches!(chars.next(), Some(c) if c.is_ascii_digit()) && part.ends_with(['m', 's']) {
            return Some(part.to_string());
        }
    }
    None
}

struct Raster {
    variable: Variable,
    month: u8,
    name: String,
    bytes: Vec<u8>,
}

fn members_from_zip(data: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut out = Vec::new();
    let seen = zip_members(data, |name, bytes| {
        out.push((name.to_string(), bytes.to_vec()));
    });
    match seen {
        Some(_) => Ok(out),
        None => Err(
            "the zip carries no readable member — the asset stays unwritten (0 honored)"
                .to_string(),
        ),
    }
}

fn read_local(path: &str) -> Result<Vec<(String, Vec<u8>)>, String> {
    let meta = std::fs::metadata(path).map_err(|e| format!("stat {path} returned void: {e}"))?;
    if meta.is_dir() {
        let mut out = Vec::new();
        let entries =
            std::fs::read_dir(path).map_err(|e| format!("read_dir {path} returned void: {e}"))?;
        for e in entries.flatten() {
            let p = e.path();
            let name = match p.file_name() {
                Some(n) => n.to_string_lossy().into_owned(),
                None => continue,
            };
            if !name.ends_with(".tif") {
                continue;
            }
            let bytes =
                std::fs::read(&p).map_err(|e| format!("read {path}/{name} returned void: {e}"))?;
            out.push((name, bytes));
        }
        return Ok(out);
    }
    let bytes = std::fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    if bytes.starts_with(b"PK\x03\x04") {
        return members_from_zip(&bytes);
    }
    let name = match path.rsplit('/').next() {
        Some(n) => n.to_string(),
        None => path.to_string(),
    };
    Ok(vec![(name, bytes)])
}

fn read_remote(url: &str) -> Result<Vec<(String, Vec<u8>)>, String> {
    let bytes = fetch_raw_bytes(url).ok_or_else(|| {
        format!("{url}: fetch returned void — the rasters stay unread (0 honored)")
    })?;
    members_from_zip(&bytes)
}

fn load_rasters(args: &[String]) -> Result<(Vec<Raster>, String), String> {
    let (members, origin) = if let Some(path) = arg_value(args, "--input") {
        (read_local(&path)?, path)
    } else if let Some(url) = arg_value(args, "--url") {
        (read_remote(&url)?, url)
    } else {
        return Err(usage().to_string());
    };
    if members.is_empty() {
        return Err(format!(
            "{origin}: carries no GeoTIFF member — the asset stays unwritten (0 honored)"
        ));
    }
    let mut rasters = Vec::new();
    for (name, bytes) in members {
        if !name.ends_with(".tif") {
            continue;
        }
        let variable = variable_from_name(&name).ok_or_else(|| {
            format!("{name}: carries no worldclim variable token (tavg/tmin/tmax/prec)")
        })?;
        let month =
            month_from_name(&name).ok_or_else(|| format!("{name}: carries no month 01..12"))?;
        rasters.push(Raster {
            variable,
            month,
            name,
            bytes,
        });
    }
    if rasters.is_empty() {
        return Err(format!(
            "{origin}: carries no worldclim monthly GeoTIFF — the asset stays unwritten"
        ));
    }
    Ok((rasters, origin))
}

fn sample_f32(pixels: &[u8], off: usize, little: bool) -> Option<f64> {
    let s = pixels.get(off..off + 4)?;
    let v = if little {
        f32::from_le_bytes([s[0], s[1], s[2], s[3]])
    } else {
        f32::from_be_bytes([s[0], s[1], s[2], s[3]])
    };
    Some(f64::from(v))
}

fn convert(v: Variable, raw: f64) -> Option<f64> {
    if !raw.is_finite() || raw.abs() >= NODATA_ABS {
        return None;
    }
    match v {
        Variable::Prec => {
            if (PREC_MIN_MM..=PREC_MAX_MM).contains(&raw) {
                Some(raw)
            } else {
                None
            }
        }
        _ => {
            if !(TEMP_MIN_C..=TEMP_MAX_C).contains(&raw) {
                return None;
            }
            let k = raw + KELVIN_OFFSET;
            if k.is_finite() { Some(k) } else { None }
        }
    }
}

fn month_epoch(month: u8) -> Result<f64, String> {
    let days = days_from_civil(CLIM_MID_YEAR, i64::from(month), CLIM_MID_DAY)
        .ok_or_else(|| format!("month {month:02} carries no civil date"))?;
    Ok(days as f64 * SECS_PER_DAY + CLIM_MID_HOUR as f64 * SECS_PER_HOUR)
}

struct RasterGrid {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    little: bool,
    geo: GeoTransform,
}

fn grid_of(raster: &Raster) -> Result<RasterGrid, String> {
    let little = match raster.bytes.get(0..2) {
        Some(b"II") => true,
        Some(b"MM") => false,
        _ => {
            return Err(format!(
                "{}: carries no TIFF byte-order mark — the raster stays unread",
                raster.name
            ));
        }
    };
    let img: TiffImage = parse_tiff(&raster.bytes).ok_or_else(|| {
        format!(
            "{}: the TIFF arm reads no raster (compression or tiling outside its domain) — the grid stays unwritten",
            raster.name
        )
    })?;
    if img.samples_per_pixel != 1 {
        return Err(format!(
            "{}: {} bands per pixel — a single-band raster is expected",
            raster.name, img.samples_per_pixel
        ));
    }
    if img.bits_per_sample != vec![32] {
        return Err(format!(
            "{}: {:?} bits per sample — a float32 raster is expected",
            raster.name, img.bits_per_sample
        ));
    }
    let geo = img
        .geo
        .clone()
        .ok_or_else(|| format!("{}: carries no geotransform — no cell centre", raster.name))?;
    let width = img.width as usize;
    let height = img.height as usize;
    let expected = width
        .checked_mul(height)
        .and_then(|n| n.checked_mul(4))
        .ok_or_else(|| format!("{}: the {width}x{height} grid overflows", raster.name))?;
    if img.pixels.len() != expected {
        return Err(format!(
            "{}: {} raster bytes against the {width}x{height}x4 float32 grid — the arm reads no common grid",
            raster.name,
            img.pixels.len()
        ));
    }
    Ok(RasterGrid {
        pixels: img.pixels,
        width,
        height,
        little,
        geo,
    })
}

fn record(pos: [f64; 3], val: f64, tdb: f64, kernel: u8, force: u8) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = 0.0;
    r[8] = f64::from(kernel);
    r[9] = f64::from(force);
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

fn ids_of(v: Variable) -> Result<(u8, u8), String> {
    let force = force_id_of(v.force())
        .ok_or_else(|| format!("{}: force '{}' is not registered", v.key(), v.force()))?;
    let kernel = kernel_id_for_force(force)
        .ok_or_else(|| format!("{}: no kernel for force '{}'", v.key(), v.force()))?;
    Ok((kernel, force))
}

struct Compiled {
    records: Vec<[f64; 26]>,
    months: usize,
    clock_void: usize,
    frame_void: usize,
    cells: usize,
}

fn compile_variable(
    variable: Variable,
    rasters: &[&Raster],
    lsk: &LeapSeconds,
    eph: &HashMap<String, BodyEphemeris>,
    body: &str,
    stride: usize,
    limit: usize,
) -> Result<Compiled, String> {
    let (kernel, force) = ids_of(variable)?;
    let mut records = Vec::new();
    let mut clock_void = 0usize;
    let mut frame_void = 0usize;
    let mut cells = 0usize;
    for raster in rasters {
        let tdb = match lsk.unix_to_tdb(month_epoch(raster.month)?) {
            Some(t) => t,
            None => {
                clock_void += 1;
                continue;
            }
        };
        let grid = grid_of(raster)?;
        let mut row = 0usize;
        while row < grid.height {
            let mut col = 0usize;
            while col < grid.width {
                let off = (row * grid.width + col) * 4;
                if let Some(raw) = sample_f32(&grid.pixels, off, grid.little) {
                    if let Some(val) = convert(variable, raw) {
                        let lat = grid.geo.y0 + (row as f64 + 0.5) * grid.geo.dy;
                        let lon = grid.geo.x0 + (col as f64 + 0.5) * grid.geo.dx;
                        if lat.is_finite()
                            && lon.is_finite()
                            && (-90.0..=90.0).contains(&lat)
                            && (-360.0..=360.0).contains(&lon)
                        {
                            match body_fixed_to_icrs(
                                body,
                                lat,
                                lon,
                                REFERENCE_SURFACE_ALT,
                                tdb,
                                eph,
                            ) {
                                Some(pos) => {
                                    records.push(record(pos, val, tdb, kernel, force));
                                    cells += 1;
                                    if records.len() >= limit {
                                        return Ok(Compiled {
                                            records,
                                            months: 1,
                                            clock_void,
                                            frame_void,
                                            cells,
                                        });
                                    }
                                }
                                None => frame_void += 1,
                            }
                        }
                    }
                }
                col += stride;
            }
            row += stride;
        }
    }
    Ok(Compiled {
        records,
        months: rasters.len(),
        clock_void,
        frame_void,
        cells,
    })
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--emit-field-names") {
        emit_field_names();
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let stride = parse_step(args, "--stride", 1)?;
    let limit = parse_step(args, "--limit", usize::MAX)?;
    let body = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;
    let out_dir = arg_value(args, "--out").ok_or_else(|| {
        "--out <dir> is required — the asset is never written to a guessed path".to_string()
    })?;

    let (rasters, origin) = load_rasters(args)?;
    let slug = match arg_value(args, "--name") {
        Some(n) => n,
        None => match resolution_from_name(&rasters[0].name) {
            Some(r) => r,
            None => "global".to_string(),
        },
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

    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("create {out_dir} returned void: {e}"))?;

    for variable in [
        Variable::Tavg,
        Variable::Tmin,
        Variable::Tmax,
        Variable::Prec,
    ] {
        let group: Vec<&Raster> = rasters.iter().filter(|r| r.variable == variable).collect();
        if group.is_empty() {
            continue;
        }
        let compiled = compile_variable(variable, &group, &lsk, &eph, &body, stride, limit)?;
        if compiled.records.is_empty() {
            eprintln!(
                "worldclim: {} leaves no measured cell — skipped (0 honored; clock_void {}, frame_void {})",
                variable.key(),
                compiled.clock_void,
                compiled.frame_void
            );
            continue;
        }
        let name = format!("worldclim_{}_{}.bin", variable.token(), slug);
        let path = format!("{out_dir}/{name}");
        let bin = write_bin(&compiled.records);
        std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned void: {e}"))?;
        let roundtrip = read_bin(&bin)
            .ok_or_else(|| format!("{path}: roundtrip parse void — the asset stays unverified"))?;
        eprintln!(
            "worldclim: {} records over {} months ({} cells), {} B -> {path} (roundtrip {}); clock_void {}, frame_void {}",
            variable.key(),
            compiled.months,
            compiled.cells,
            bin.len(),
            roundtrip,
            compiled.clock_void,
            compiled.frame_void
        );
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
        println!("format {FORMAT}");
        println!("origin {origin}");
        println!("compiler {COMPILER}");
        println!("at {body}");
        println!("ttl {}", TTL_S as u64);
        println!("{}", field_line(variable));
        println!("sha256 {}", sha256_hex(&bin));
        if ci_mode && !upload_release(NETLOC, &path) {
            return Err(format!("{path}: CDN upload returned void"));
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("{}", usage());
        eprintln!("base {BASE}");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("worldclim_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_reads_the_member_name() {
        assert_eq!(
            variable_from_name("wc2.1_10m_tavg_01.tif"),
            Some(Variable::Tavg)
        );
        assert_eq!(
            variable_from_name("wc2.1_30s_prec_12.tif"),
            Some(Variable::Prec)
        );
        assert_eq!(variable_from_name("archive.zip"), None);
    }

    #[test]
    fn month_reads_the_trailing_two_digits() {
        assert_eq!(month_from_name("wc2.1_10m_tavg_01.tif"), Some(1));
        assert_eq!(month_from_name("wc2.1_10m_tavg_12.tif"), Some(12));
        assert_eq!(month_from_name("wc2.1_10m_tavg_13.tif"), None);
        assert_eq!(month_from_name("wc2.1_10m_tavg.tif"), None);
    }

    #[test]
    fn resolution_is_measured_from_the_name() {
        assert_eq!(
            resolution_from_name("wc2.1_10m_tavg_01.tif"),
            Some("10m".to_string())
        );
        assert_eq!(
            resolution_from_name("wc2.1_30s_prec_01.tif"),
            Some("30s".to_string())
        );
    }

    #[test]
    fn convert_gates_the_physical_range_and_scales_to_si() {
        assert_eq!(convert(Variable::Tavg, 20.0), Some(293.15));
        assert_eq!(convert(Variable::Tavg, 0.0), Some(273.15));
        assert_eq!(convert(Variable::Tavg, -100.0), None);
        assert_eq!(convert(Variable::Tavg, 100.0), None);
        assert_eq!(convert(Variable::Tavg, -3.4e38), None);
        assert_eq!(convert(Variable::Tavg, f64::NAN), None);
        assert_eq!(convert(Variable::Prec, 0.0), Some(0.0));
        assert_eq!(convert(Variable::Prec, 245.0), Some(245.0));
        assert_eq!(convert(Variable::Prec, -1.0), None);
    }

    #[test]
    fn slot_values_follow_the_variable() {
        match convert(Variable::Tavg, 15.0) {
            Some(v) => assert!((v - 288.15).abs() < 1e-9),
            None => panic!("15 degC is a measured value"),
        }
    }

    #[test]
    fn ids_map_to_registered_kernel_and_force() {
        assert_eq!(ids_of(Variable::Tavg), Ok((4, 6)));
        assert_eq!(ids_of(Variable::Prec), Ok((1, 7)));
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let r = record([1.0, 2.0, 3.0], 293.15, 8.0e8, 4, 6);
        assert_eq!(r[0], 1.0);
        assert_eq!(r[3], 293.15);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], TAU_S);
        assert_eq!(r[8], 4.0);
        assert_eq!(r[9], 6.0);
        assert_eq!(r[25], 1.0);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
    }

    #[test]
    fn bin_roundtrip_counts_records() {
        let records = vec![
            record([1.0, 2.0, 3.0], 293.15, 8.0e8, 4, 6),
            record([4.0, 5.0, 6.0], 0.0, 8.0e8 + 1.0, 1, 7),
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        assert_eq!(read_bin(&bytes), Some(2));
        assert!(read_bin(b"X").is_none());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn field_lines_emit_the_si_contract() {
        assert_eq!(
            field_line(Variable::Tavg),
            "field worldclim_tavg worldclim_tavg exponential-decay thermal K 2592000 0.0 0.0"
        );
        assert_eq!(
            field_line(Variable::Prec),
            "field worldclim_prec worldclim_prec gaussian-inverse-square diffusion mm 2592000 0.0 0.0"
        );
    }

    #[test]
    fn month_epoch_is_the_mid_month_of_the_climatology() {
        let jan = month_epoch(1).expect("january epoch");
        let feb = month_epoch(2).expect("february epoch");
        assert_eq!(feb - jan, 31.0 * SECS_PER_DAY);
        assert_eq!(jan, 474638400.0);
    }
}
