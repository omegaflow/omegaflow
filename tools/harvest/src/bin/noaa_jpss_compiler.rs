use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File, Hdf5Object};
use omegaflow::lsk::{days_from_civil, parse as parse_lsk, LeapSeconds};
use std::env;
use std::fs;

const NETLOC: &str = "noaa-jpss.s3.amazonaws.com";
const MAGIC_OMPS: [u8; 4] = *b"OMP1";
const COMP_OMPS_RADIANCE: u32 = 1;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn attr_text(obj: &Hdf5Object, name: &str) -> Option<String> {
    let a = obj.attrs.iter().find(|a| a.name == name)?;
    let s = String::from_utf8_lossy(&a.data);
    let t = s.trim_end_matches('\0').trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn walk_datasets(file: &Hdf5File, path: &str, out: &mut Vec<String>) {
    if file.dataset(path).is_ok() {
        out.push(path.to_string());
        return;
    }
    for l in file.links_of(path) {
        let child = if path.is_empty() {
            l.name.clone()
        } else {
            format!("{}/{}", path, l.name)
        };
        walk_datasets(file, &child, out);
    }
}

fn leaf(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn find_by_leaf(paths: &[String], want: &str) -> Option<String> {
    paths
        .iter()
        .find(|p| leaf(p).eq_ignore_ascii_case(want))
        .cloned()
}

fn probe(bytes: &[u8]) {
    let Ok(file) = Hdf5File::parse(bytes) else {
        println!("hdf5 parse void");
        return;
    };
    let mut paths = Vec::new();
    walk_datasets(&file, "", &mut paths);
    paths.sort();
    for p in paths {
        let Ok((obj, ds, dt)) = file.dataset(&p) else {
            continue;
        };
        let units = attr_text(obj, "units");
        let long = attr_text(obj, "long_name");
        println!(
            "  ds {} class {} size {} dims {:?} units {} long {}",
            p,
            dt.class,
            dt.size,
            ds.dims,
            units.as_deref().unwrap_or("absent"),
            long.as_deref().unwrap_or("absent")
        );
    }
}

fn elem_f64(raw: &[u8], idx: usize, class: u8, size: usize, endian: Endian) -> Option<f64> {
    let off = idx.checked_mul(size)?;
    let b = raw.get(off..off + size)?;
    match (class, size) {
        (0, 1) => Some(b[0] as i8 as f64),
        (0, 2) => {
            let arr: [u8; 2] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                i16::from_le_bytes(arr)
            } else {
                i16::from_be_bytes(arr)
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
        (1, 4) => decode_f32(b, 0, endian).map(|v| v as f64),
        (1, 8) => decode_f64(b, 0, endian),
        _ => None,
    }
}

fn granule_start_unix(name: &str) -> Option<f64> {
    let d = name.split('_').find(|s| {
        s.len() == 9 && s.starts_with('d') && s[1..].bytes().all(|b| b.is_ascii_digit())
    })?;
    let y: i64 = d[1..5].parse().ok()?;
    let m: i64 = d[5..7].parse().ok()?;
    let dd: i64 = d[7..9].parse().ok()?;
    let t = name.split('_').find(|s| {
        s.len() == 8 && s.starts_with('t') && s[1..].bytes().all(|b| b.is_ascii_digit())
    })?;
    let h: i64 = t[1..3].parse().ok()?;
    let mi: i64 = t[3..5].parse().ok()?;
    let ss: f64 = t[5..7].parse().ok()?;
    let tenths: f64 = t[7..8].parse().ok()?;
    let days = days_from_civil(y, m, dd)?;
    Some(days as f64 * 86400.0 + h as f64 * 3600.0 + mi as f64 * 60.0 + ss + tenths * 0.1)
}

fn emit(bytes: &[u8], granule_name: &str, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let Ok(file) = Hdf5File::parse(bytes) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    walk_datasets(&file, "", &mut paths);
    let Some(rad_path) = find_by_leaf(&paths, "Radiance") else {
        eprintln!("{granule_name}: no Radiance dataset — the bin stays unwritten (0 honored)");
        return Vec::new();
    };
    let Some(lat_path) = find_by_leaf(&paths, "Latitude") else {
        eprintln!("{granule_name}: no Latitude dataset — the bin stays unwritten (0 honored)");
        return Vec::new();
    };
    let Some(lon_path) = find_by_leaf(&paths, "Longitude") else {
        eprintln!("{granule_name}: no Longitude dataset — the bin stays unwritten (0 honored)");
        return Vec::new();
    };
    let (rad_obj, rad_ds, rad_dt) = match file.dataset(&rad_path) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let rad_raw = match file.read_dataset(&rad_path) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let lat_raw = match file.read_dataset(&lat_path) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let lon_raw = match file.read_dataset(&lon_path) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let lat_dt = match file.dataset(&lat_path) {
        Ok((_, _, dt)) => dt.clone(),
        Err(_) => return Vec::new(),
    };
    let lon_dt = match file.dataset(&lon_path) {
        Ok((_, _, dt)) => dt.clone(),
        Err(_) => return Vec::new(),
    };
    let rad_fill = attr_text(rad_obj, "_FillValue").and_then(|s| s.trim().parse::<f64>().ok());
    let rad_scale: Option<f64> =
        attr_text(rad_obj, "ScaleFactor").and_then(|s| s.trim().parse::<f64>().ok());
    let rad_offset: Option<f64> =
        attr_text(rad_obj, "Offset").and_then(|s| s.trim().parse::<f64>().ok());
    if lat_dt.size == 0 || lon_dt.size == 0 {
        eprintln!(
            "{granule_name}: a lat/lon datatype carries size 0 — the axes stay uncounted, the bin stays unwritten"
        );
        return Vec::new();
    }
    let n = rad_ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let n_lat = lat_raw.len() / lat_dt.size;
    let n_lon = lon_raw.len() / lon_dt.size;
    if n == 0 || n_lat != n || n_lon != n {
        eprintln!(
            "{granule_name}: radiance {} elements, latitude {} / longitude {} — axes disagree, the bin stays unwritten",
            n, n_lat, n_lon
        );
        return Vec::new();
    }
    let Some(unix) = granule_start_unix(granule_name) else {
        eprintln!(
            "{granule_name}: the granule name carries no d/t epoch token — the bin stays unwritten"
        );
        return Vec::new();
    };
    let Some(tdb) = lsk.unix_to_tdb(unix) else {
        eprintln!("{granule_name}: the granule epoch stays outside the leap-second table — the bin stays unwritten");
        return Vec::new();
    };
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let Some(rad) = elem_f64(&rad_raw, i, rad_dt.class, rad_dt.size, rad_dt.endian) else {
            continue;
        };
        let Some(lat) = elem_f64(&lat_raw, i, lat_dt.class, lat_dt.size, lat_dt.endian) else {
            continue;
        };
        let Some(lon) = elem_f64(&lon_raw, i, lon_dt.class, lon_dt.size, lon_dt.endian) else {
            continue;
        };
        if !lat.is_finite() || !lon.is_finite() || !rad.is_finite() {
            continue;
        }
        if lat < -90.0 || lat > 90.0 || lon < -180.0 || lon > 180.0 {
            continue;
        }
        if let Some(f) = rad_fill {
            if rad == f {
                continue;
            }
        }
        let scaled = match rad_scale {
            Some(s) => rad * s,
            None => rad,
        };
        let val = match rad_offset {
            Some(o) => scaled + o,
            None => scaled,
        };
        if !val.is_finite() {
            continue;
        }
        out.push(GeoRec {
            t: tdb,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val,
            comp: COMP_OMPS_RADIANCE,
            station: 0,
        });
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    if let Some(path) = arg_value(&args, "--probe") {
        match fs::read(&path) {
            Ok(bytes) => {
                println!("{path}: {} B", bytes.len());
                probe(&bytes);
            }
            Err(e) => {
                eprintln!("{path}: read void {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let lsk = match arg_value(&args, "--lsk")
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|t| parse_lsk(&t))
    {
        Some(l) => l,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void");
            std::process::exit(1);
        }
    };
    let in_arg = arg_value(&args, "--in");
    let (bytes, granule_name) = match in_arg.as_deref().and_then(|p| fs::read(p).ok()) {
        Some(b) => {
            let name = match in_arg.as_deref() {
                Some(p) => p.rsplit('/').next().unwrap_or(p).to_string(),
                None => "granule".to_string(),
            };
            (b, name)
        }
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None => {
                    eprintln!("--url (OMPS-SDR .h5 granule) or --in <file> required");
                    std::process::exit(1);
                }
            };
            let name = url.rsplit('/').next().unwrap_or("granule").to_string();
            match fetch_raw_bytes(&url, 3600) {
                Some(b) => (b, name),
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            }
        }
    };
    let mut records = emit(&bytes, &granule_name, &lsk);
    if records.is_empty() {
        eprintln!(
            "{granule_name}: no measured radiance records — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.lat.total_cmp(&b.lat)));
    let bin = write_bin(MAGIC_OMPS, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(parent);
    }
    if fs::write(&out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_OMPS, &bin) {
        Some(parsed) => eprintln!(
            "{granule_name}: {} radiance records written, roundtrip parses",
            parsed.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
