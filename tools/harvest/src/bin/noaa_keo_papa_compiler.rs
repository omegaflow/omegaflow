use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    parse_bin, write_bin, GeoRec, COMP_KEO_PSAL, COMP_KEO_TEMP, COMP_KEO_UCUR, COMP_KEO_VCUR,
    MAGIC_KEO,
};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::Hdf5File;
use omegaflow::lsk::parse as parse_lsk;
use omegaflow::netcdf::{NetcdfFile, NetcdfType};
use std::env;
use std::fs;

const NETLOC: &str = "noaa-oar-keo-papa-pds.s3.amazonaws.com";
const BUCKET: &str = "https://noaa-oar-keo-papa-pds.s3.amazonaws.com";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn is_hdf5(b: &[u8]) -> bool {
    b.starts_with(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a])
}

fn is_classic(b: &[u8]) -> bool {
    b.starts_with(b"CDF")
}

fn probe(bytes: &[u8]) {
    if is_classic(bytes) {
        match NetcdfFile::parse(bytes) {
            Ok(nc) => {
                println!("classic dims {} vars {}", nc.dims.len(), nc.vars.len());
                for d in &nc.dims {
                    let label = if d.len == 0 {
                        "record".to_string()
                    } else {
                        d.len.to_string()
                    };
                    println!("  dim {} = {}", d.name, label);
                }
                for v in &nc.vars {
                    let units = v
                        .attrs
                        .iter()
                        .find(|a| a.name == "units")
                        .and_then(|a| nc.attr_text(a));
                    let dims: Vec<String> = v
                        .dim_ids
                        .iter()
                        .filter_map(|&id| nc.dims.get(id).map(|d| d.name.clone()))
                        .collect();
                    println!(
                        "  var {} {} [{}] units {}",
                        v.name,
                        v.nc_type.name(),
                        dims.join(","),
                        units.as_deref().unwrap_or("absent")
                    );
                }
            }
            Err(note) => println!("classic parse void: {note:?}"),
        }
    } else if is_hdf5(bytes) {
        match Hdf5File::parse(bytes) {
            Ok(file) => {
                let Ok(root) = file.root() else {
                    println!("hdf5 root void");
                    return;
                };
                let mut names: Vec<String> = root.links.iter().map(|l| l.name.clone()).collect();
                names.sort();
                for name in names {
                    match file.dataset(&name) {
                        Ok((obj, ds, dt)) => {
                            let units = obj.attrs.iter().find(|a| a.name == "units").map(|a| {
                                String::from_utf8_lossy(&a.data)
                                    .trim_end_matches('\0')
                                    .to_string()
                            });
                            println!(
                                "  ds {} class {} size {} dims {:?} units {}",
                                name,
                                dt.class,
                                dt.size,
                                ds.dims,
                                units.as_deref().unwrap_or("absent")
                            );
                        }
                        Err(_) => println!("  group {}", name),
                    }
                }
            }
            Err(note) => println!("hdf5 parse void: {note:?}"),
        }
    } else {
        println!("unknown byte stream");
    }
}

fn attr_num(nc: &NetcdfFile, name: &str, key: &str) -> Option<f64> {
    nc.var(name)?
        .attrs
        .iter()
        .find(|a| a.name == key)
        .and_then(|a| nc.attr_num(a))
}

fn attr_text(nc: &NetcdfFile, name: &str, key: &str) -> Option<String> {
    nc.var(name)?
        .attrs
        .iter()
        .find(|a| a.name == key)
        .and_then(|a| nc.attr_text(a))
}

fn values(nc: &NetcdfFile, bytes: &[u8], name: &str) -> Option<Vec<f64>> {
    let v = nc.var(name)?;
    Some(match v.nc_type {
        NetcdfType::Float => nc
            .values_f32(bytes, name)?
            .iter()
            .map(|x| *x as f64)
            .collect(),
        NetcdfType::Double => nc.values_f64(bytes, name)?,
        NetcdfType::Short => nc
            .values_i16(bytes, name)?
            .iter()
            .map(|x| *x as f64)
            .collect(),
        NetcdfType::Int => nc
            .values_i32(bytes, name)?
            .iter()
            .map(|x| *x as f64)
            .collect(),
        NetcdfType::Byte => nc
            .values_i8(bytes, name)?
            .iter()
            .map(|x| *x as f64)
            .collect(),
        NetcdfType::Char => return None,
    })
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
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
    let base = days_from_civil(y, m, d) as f64 * 86400.0;
    Some((factor, base))
}

fn comp_of(name: &str) -> Option<(u32, f64)> {
    match name {
        "TEMP" => Some((COMP_KEO_TEMP, 1.0)),
        "PSAL" => Some((COMP_KEO_PSAL, 1.0)),
        "UCUR" => Some((COMP_KEO_UCUR, 0.01)),
        "VCUR" => Some((COMP_KEO_VCUR, 0.01)),
        _ => None,
    }
}

const VARS: &[(&str, &str)] = &[
    ("TEMP", "DEPTEMP"),
    ("PSAL", "DEPPSAL"),
    ("UCUR", "DEPCUR"),
    ("VCUR", "DEPCUR"),
];

fn emit_classic(bytes: &[u8], lsk: &omegaflow::lsk::LeapSeconds) -> Vec<GeoRec> {
    let Ok(nc) = NetcdfFile::parse(bytes) else {
        return Vec::new();
    };
    let Some(lat) = values(&nc, bytes, "LATITUDE").and_then(|v| v.first().copied()) else {
        return Vec::new();
    };
    let Some(lon) = values(&nc, bytes, "LONGITUDE").and_then(|v| v.first().copied()) else {
        return Vec::new();
    };
    let Some(time) = values(&nc, bytes, "TIME") else {
        return Vec::new();
    };
    let Some(units) = attr_text(&nc, "TIME", "units") else {
        return Vec::new();
    };
    let Some((factor, base)) = epoch_seconds(&units) else {
        return Vec::new();
    };
    let nt = time.len();
    let mut out = Vec::new();
    for (name, depth_dim) in VARS {
        let Some((comp, scale)) = comp_of(name) else {
            continue;
        };
        let Some(depth) = values(&nc, bytes, depth_dim) else {
            continue;
        };
        let Some(vals) = values(&nc, bytes, name) else {
            continue;
        };
        let fill = attr_num(&nc, name, "_FillValue");
        let vscale = attr_num(&nc, name, "scale_factor").unwrap_or(1.0);
        let voffset = match attr_num(&nc, name, "add_offset") {
            Some(v) => v,
            None => 0.0,
        };
        let shape = match nc.var(name).and_then(|v| nc.var_shape(v).ok()) {
            Some(s) => s,
            None => continue,
        };
        let nd = depth.len();
        if nd == 0 || nt == 0 {
            continue;
        }
        let trailing: usize = shape
            .iter()
            .skip(2)
            .fold(1usize, |a, s| a.saturating_mul(*s as usize));
        for t in 0..nt {
            let Some(&tv) = time.get(t) else {
                continue;
            };
            let unix = base + tv * factor;
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                continue;
            };
            for d in 0..nd {
                let Some(&depth_m) = depth.get(d) else {
                    continue;
                };
                let idx = t * nd * trailing + d * trailing;
                let Some(&raw) = vals.get(idx) else {
                    continue;
                };
                if !raw.is_finite() {
                    continue;
                }
                if let Some(f) = fill {
                    if raw == f {
                        continue;
                    }
                }
                let val = (raw * vscale + voffset) * scale;
                if !val.is_finite() {
                    continue;
                }
                out.push(GeoRec {
                    t: tdb,
                    lat,
                    lon,
                    alt: -depth_m,
                    freq: 0.0,
                    bin_width: 0.0,
                    val,
                    comp,
                    station: 0,
                });
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
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

    let key = match arg_value(&args, "--key") {
        Some(k) => k,
        None => {
            eprintln!(
                "--key <bucket key> (e.g. KEO/OS_KEO_200406_M_TSVMBP_32N145E_dy.nc) required"
            );
            std::process::exit(1);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("--out <path> required");
            std::process::exit(1);
        }
    };
    let Some(lsk_text) = arg_value(&args, "--lsk").and_then(|p| fs::read_to_string(p).ok()) else {
        eprintln!("--lsk absent — the TDB conversion stays void");
        std::process::exit(1);
    };
    let Some(lsk) = parse_lsk(&lsk_text) else {
        eprintln!("--lsk parses void");
        std::process::exit(1);
    };
    let bytes = match arg_value(&args, "--in").and_then(|p| fs::read(p).ok()) {
        Some(b) => b,
        None => {
            let url = format!("{BUCKET}/{key}");
            match fetch_raw_bytes(&url, 3600) {
                Some(b) => b,
                None => {
                    eprintln!("{key}: fetch void");
                    std::process::exit(1);
                }
            }
        }
    };
    if is_hdf5(&bytes) {
        eprintln!(
            "{key}: netCDF-4/HDF5 container — the reader carries no classic record layout for this file (parser-gap)"
        );
        std::process::exit(1);
    }
    let mut records = emit_classic(&bytes, &lsk);
    if records.is_empty() {
        eprintln!("{key}: no measured records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.comp.cmp(&b.comp)));
    let bytes = write_bin(MAGIC_KEO, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(parent);
    }
    if fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_KEO, &bytes) {
        Some(parsed) => eprintln!(
            "{key}: {} geo records written, roundtrip parses",
            parsed.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if args.iter().any(|a| a == "--ci-mode") && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
