use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_ISSLIS_FLASH_RAD, MAGIC_ISSLIS};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk, LeapSeconds};
use std::process::Command;

const NETLOC: &str = "ghrc.nasa.gov";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSLf")
        .arg("--retry")
        .arg("3")
        .arg("--max-time")
        .arg("240");
    if let Some(token) = secret("EARTHDATA_EDL_TOKEN") {
        cmd.arg("-H").arg(format!("Authorization: Bearer {token}"));
    }
    let out = cmd.arg(url).output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn granule_bytes(path_or_url: &str) -> Option<Vec<u8>> {
    if let Ok(b) = std::fs::read(path_or_url) {
        return Some(b);
    }
    fetch(path_or_url)
}

struct VarLoad {
    raw: Vec<u8>,
    class: u8,
    size: usize,
    endian: Endian,
    n: usize,
}

fn dataset_load(file: &Hdf5File, name: &str) -> Option<VarLoad> {
    let (obj, ds, dt) = file.dataset(name).ok()?;
    if obj.is_group {
        return None;
    }
    let n: usize = ds
        .dims
        .iter()
        .fold(1usize, |a, d| a.saturating_mul(*d as usize));
    if n == 0 {
        return None;
    }
    let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset(name)))
        .ok()?
        .ok()?;
    Some(VarLoad {
        raw,
        class: dt.class,
        size: dt.size,
        endian: dt.endian,
        n,
    })
}

fn elem_f64(raw: &[u8], idx: usize, class: u8, size: usize, endian: Endian) -> Option<f64> {
    let off = idx.checked_mul(size)?;
    let b = raw.get(off..off + size)?;
    match (class, size) {
        (0, 1) => Some(b[0] as i8 as f64),
        (0, 2) => {
            let v = if endian == Endian::Le {
                i16::from_le_bytes([b[0], b[1]])
            } else {
                i16::from_be_bytes([b[0], b[1]])
            };
            Some(v as f64)
        }
        (0, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                i32::from_le_bytes(arr)
            } else {
                i32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (0, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                i64::from_le_bytes(arr)
            } else {
                i64::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                f32::from_le_bytes(arr)
            } else {
                f32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                f64::from_le_bytes(arr)
            } else {
                f64::from_be_bytes(arr)
            };
            Some(v)
        }
        _ => None,
    }
}

fn attr_f64(file: &Hdf5File, dataset: &str, attr: &str) -> Option<f64> {
    let a = file.attribute(dataset, attr)?;
    if a.dataspace.dims.is_empty() {
        return None;
    }
    let first: u64 = *a.dataspace.dims.first()?;
    if first == 0 {
        return None;
    }
    elem_f64(
        &a.data,
        0,
        a.datatype.class,
        a.datatype.size,
        a.datatype.endian,
    )
}

fn scaled_f64(v: Option<f64>, file: &Hdf5File, dataset: &str) -> Option<f64> {
    let mut val = v?;
    if let Some(scale) = attr_f64(file, dataset, "scale_factor") {
        val *= scale;
    }
    if let Some(off) = attr_f64(file, dataset, "add_offset") {
        val += off;
    }
    if val.is_finite() {
        Some(val)
    } else {
        None
    }
}

fn tai93_to_tdb(lsk: &LeapSeconds, tai93: f64) -> Option<f64> {
    if !tai93.is_finite() {
        return None;
    }
    let epoch_days = days_from_civil(1993, 1, 1)?;
    let c = tai93 + epoch_days as f64 * 86400.0;
    let mut u = c - 37.0;
    for _ in 0..3 {
        u = c - lsk.leap_at(u)?;
    }
    if u < 0.0 {
        return None;
    }
    lsk.unix_to_tdb(u)
}

fn flash_records(file: &Hdf5File, lsk: &LeapSeconds, src: &str) -> Vec<GeoRec> {
    let lat_v = match dataset_load(file, "lightning_flash_lat") {
        Some(v) => v,
        None => {
            eprintln!("iss_lis: {} carries no lightning_flash_lat", src);
            return Vec::new();
        }
    };
    let lon_v = match dataset_load(file, "lightning_flash_lon") {
        Some(v) => v,
        None => {
            eprintln!("iss_lis: {} carries no lightning_flash_lon", src);
            return Vec::new();
        }
    };
    let rad_v = match dataset_load(file, "lightning_flash_radiance") {
        Some(v) => v,
        None => {
            eprintln!("iss_lis: {} carries no lightning_flash_radiance", src);
            return Vec::new();
        }
    };
    let time_v = match dataset_load(file, "lightning_flash_TAI93_time") {
        Some(v) => v,
        None => {
            eprintln!("iss_lis: {} carries no lightning_flash_TAI93_time", src);
            return Vec::new();
        }
    };
    let n = lat_v.n.min(lon_v.n).min(rad_v.n).min(time_v.n);
    let mut out = Vec::new();
    for j in 0..n {
        let lat = scaled_f64(
            elem_f64(&lat_v.raw, j, lat_v.class, lat_v.size, lat_v.endian),
            file,
            "lightning_flash_lat",
        );
        let lon = scaled_f64(
            elem_f64(&lon_v.raw, j, lon_v.class, lon_v.size, lon_v.endian),
            file,
            "lightning_flash_lon",
        );
        let rad = scaled_f64(
            elem_f64(&rad_v.raw, j, rad_v.class, rad_v.size, rad_v.endian),
            file,
            "lightning_flash_radiance",
        );
        let tai = elem_f64(&time_v.raw, j, time_v.class, time_v.size, time_v.endian);
        let (Some(lat), Some(lon), Some(rad), Some(tai)) = (lat, lon, rad, tai) else {
            continue;
        };
        if !(rad > 0.0) || !(lat.abs() <= 90.0) || !(lon.abs() <= 180.0) {
            continue;
        }
        let Some(tdb) = tai93_to_tdb(lsk, tai) else {
            continue;
        };
        out.push(GeoRec {
            t: tdb,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: rad,
            comp: COMP_ISSLIS_FLASH_RAD,
        });
    }
    out
}

fn run(out_path: &str, granules: &[String], lsk: &LeapSeconds, ci: bool) -> Result<(), String> {
    let mut records: Vec<GeoRec> = Vec::new();
    for src in granules {
        let bytes = granule_bytes(src).ok_or(format!("{src} stayed unreadable"))?;
        let file = Hdf5File::parse(&bytes).map_err(|e| format!("{src} parses void ({e:?})"))?;
        let recs = flash_records(&file, lsk, src);
        eprintln!("iss_lis: {} → {} flashes", src, recs.len());
        records.extend(recs);
    }
    if records.is_empty() {
        return Err("no flashes harvested — the bin stays unwritten (0 honored)".into());
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bytes = write_bin(MAGIC_ISSLIS, &records);
    std::fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    match parse_bin(MAGIC_ISSLIS, &bytes) {
        Some(parsed) => eprintln!(
            "{}: {} flashes, {} B, roundtrip parses",
            out_path,
            parsed.len(),
            bytes.len()
        ),
        None => {
            return Err(format!(
                "{out_path}: roundtrip parse void — the bin stays unverified"
            ))
        }
    }
    if ci && !upload_release(NETLOC, out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!(
                "iss_lis_compiler: --out <file.bin> absent — the output path is never silent"
            );
            std::process::exit(1);
        }
    };
    let ci = args.iter().any(|a| a == "--ci-mode");
    let granules: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == "--granule")
        .filter_map(|(i, _)| args.get(i + 1))
        .cloned()
        .collect();
    if granules.is_empty() {
        eprintln!("iss_lis_compiler: --granule <url|path> absent — refused");
        std::process::exit(1);
    }
    let lsk_path = match arg_value(&args, "--lsk") {
        Some(p) => p,
        None => {
            eprintln!("iss_lis_compiler: --lsk <naif0012.tls> absent — the TDB clock stays unread");
            std::process::exit(1);
        }
    };
    let lsk_text = match std::fs::read_to_string(&lsk_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "iss_lis_compiler: {} unreadable: {e} — the TDB clock stays unread",
                lsk_path
            );
            std::process::exit(1);
        }
    };
    let Some(lsk) = parse_lsk(&lsk_text) else {
        eprintln!(
            "iss_lis_compiler: {} parses void — the leap table stays unread",
            lsk_path
        );
        std::process::exit(1);
    };
    if let Err(msg) = run(&out, &granules, &lsk, ci) {
        eprintln!("iss_lis_compiler: {msg}");
        std::process::exit(1);
    }
}
