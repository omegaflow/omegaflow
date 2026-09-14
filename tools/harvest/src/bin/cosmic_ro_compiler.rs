use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::geo::{
    parse_bin, write_bin, GeoRec, COMP_COSMIC_PRES, COMP_COSMIC_REFRACT, COMP_COSMIC_TEMP,
    MAGIC_COSMIC,
};
use omegaflow::hdf5::{decode_f32, decode_f64, Hdf5File};
use omegaflow::inflate::gunzip;
use omegaflow::lsk::LeapSeconds;
use omegaflow::netcdf::{directory_links, nc4_group, NetcdfFile, NetcdfFormat, NetcdfType};

const NETLOC: &str = "data.cosmic.ucar.edu";
const ROOT: &str = "https://data.cosmic.ucar.edu/gnss-ro/cosmic2/nrt";
const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];

const LEVEL2_PRODUCTS: [&str; 5] = ["atmPrf", "avnPrf", "bfrPrf", "echPrf", "wetPf2"];
const LEVEL3_PRODUCTS: [&str; 16] = [
    "alcDat", "alcDt2", "alcPl2", "alcPlt", "bubJL1", "bubJL102", "bubJU1", "bubJU102", "bubJsn",
    "bubJsn02", "bubML1", "bubML102", "bubMU1", "bubMU102", "bubMap", "bubMap02",
];

const PROFILE_VARS: [&str; 5] = ["MSL_alt", "Ref", "Temp", "Dry_Temp", "Pres"];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name).and_then(|v| v.parse::<usize>().ok())
}

fn fetch_text(url: &str) -> Option<String> {
    let bytes = fetch_raw_bytes(url, 60)?;
    String::from_utf8(bytes).ok()
}

fn level2_tarball_url(product: &str, year: u32, doy: u32) -> String {
    format!("{ROOT}/level2/{year}/{doy:03}/{product}_nrt_{year}_{doy:03}.tar.gz")
}

fn level3_tarball_url(product: &str, year: u32, doy: u32) -> String {
    format!("{ROOT}/level3/{year}/{doy:03}/{product}/{product}_nrt_{year}_{doy:03}.tar.gz")
}

fn naming_scheme() {
    eprintln!("COSMIC-2 NRT tree ({ROOT}):");
    eprintln!(
        "  level2/{{year}}/{{doy:03}}/ carries the radio-occultation profiles as daily tarballs ({{product}} in {:?}):",
        LEVEL2_PRODUCTS
    );
    eprintln!(
        "    {{product}}_nrt_{{year}}_{{doy:03}}.tar.gz  (example {})",
        level2_tarball_url("wetPf2", 2026, 251)
    );
    eprintln!(
        "  level3/{{year}}/{{doy:03}}/{{product}}/ carries ionosphere-scintillation JSON tarballs ({{product}} in {:?}):",
        LEVEL3_PRODUCTS
    );
    eprintln!(
        "    {{product}}_nrt_{{year}}_{{doy:03}}.tar.gz  (example {})",
        level3_tarball_url("bubJL1", 2026, 251)
    );
    eprintln!(
        "  granule: {{product}}_{{sat}}.{{year}}.{{doy}}.{{hh}}.{{mm}}.{{occ}}_{{year}}.{{secs}}_nc"
    );
}

fn list_links(body: &str) -> Vec<String> {
    directory_links(body)
        .into_iter()
        .filter(|n| n != "../" && !n.is_empty())
        .collect()
}

fn run_list(args: &[String]) -> Result<(), String> {
    naming_scheme();
    let year = arg_value(args, "--year").and_then(|v| v.parse::<u32>().ok());
    let doy = arg_value(args, "--doy").and_then(|v| v.parse::<u32>().ok());
    for level in ["level2", "level3"] {
        let index_url = match year {
            Some(y) => match doy {
                Some(d) => format!("{ROOT}/{level}/{y}/{d:03}/"),
                None => format!("{ROOT}/{level}/{y}/"),
            },
            None => format!("{ROOT}/{level}/"),
        };
        match fetch_text(&index_url) {
            Some(body) => {
                eprintln!("{index_url}:");
                for n in list_links(&body) {
                    eprintln!("  {n}");
                }
            }
            None => eprintln!("{index_url}: fetch returned void"),
        }
    }
    Ok(())
}

fn tar_octal(field: &[u8]) -> Option<usize> {
    let mut v = 0usize;
    let mut any = false;
    for &b in field {
        if b == 0 || b == b' ' {
            break;
        }
        if !(b'0'..=b'7').contains(&b) {
            return None;
        }
        v = v * 8 + (b - b'0') as usize;
        any = true;
    }
    if any {
        Some(v)
    } else {
        None
    }
}

fn tar_name(header: &[u8]) -> String {
    let end = header[..100].iter().position(|&b| b == 0).unwrap_or(100);
    String::from_utf8_lossy(&header[..end]).to_string()
}

struct TarMember {
    name: String,
    start: usize,
    end: usize,
}

fn tar_members(tar: &[u8]) -> Vec<TarMember> {
    let mut out = Vec::new();
    let mut off = 0usize;
    let mut long_name: Option<String> = None;
    while off + 512 <= tar.len() {
        let header = &tar[off..off + 512];
        if header.iter().all(|&b| b == 0) {
            break;
        }
        let name = match &long_name {
            Some(n) => n.clone(),
            None => tar_name(header),
        };
        let Some(size) = tar_octal(&header[124..136]) else {
            break;
        };
        let typeflag = header[156];
        let data_off = off + 512;
        let data_end = data_off.saturating_add(size);
        if typeflag == b'L' {
            long_name = tar.get(data_off..data_end).map(|d| {
                let e = d.iter().position(|&b| b == 0).unwrap_or(d.len());
                String::from_utf8_lossy(&d[..e]).to_string()
            });
        } else {
            long_name = None;
            if typeflag == b'0' || typeflag == 0 {
                out.push(TarMember {
                    name,
                    start: data_off,
                    end: data_end,
                });
            }
        }
        off = data_end + ((512 - size % 512) % 512);
    }
    out
}

fn is_ro_granule(name: &str) -> bool {
    name.ends_with("_nc")
}

fn load_tar(arg: &str) -> Result<Vec<u8>, String> {
    let gz = if arg.starts_with("http://") || arg.starts_with("https://") {
        fetch_raw_bytes(arg, 3600).ok_or_else(|| format!("{arg}: fetch returned void"))?
    } else {
        std::fs::read(arg).map_err(|e| format!("read {arg} returned void: {e}"))?
    };
    gunzip(&gz).ok_or_else(|| format!("{arg}: gzip stream stays unreadable"))
}

fn apply_scale(
    raw: Vec<f64>,
    scale: Option<f64>,
    offset: Option<f64>,
    fill: Option<f64>,
) -> Vec<f64> {
    raw.into_iter()
        .map(|x| {
            if !x.is_finite() {
                return f64::NAN;
            }
            if let Some(f) = fill {
                if x == f {
                    return f64::NAN;
                }
            }
            let s = match scale {
                Some(k) => x * k,
                None => x,
            };
            let y = match offset {
                Some(b) => s + b,
                None => s,
            };
            if y.is_finite() {
                y
            } else {
                f64::NAN
            }
        })
        .collect()
}

fn nc3_series(nc: &NetcdfFile, bytes: &[u8], name: &str) -> Option<Vec<f64>> {
    let v = nc.var(name)?;
    match v.nc_type {
        NetcdfType::Float => Some(
            nc.values_f32(bytes, name)?
                .iter()
                .map(|x| *x as f64)
                .collect(),
        ),
        NetcdfType::Double => nc.values_f64(bytes, name),
        NetcdfType::Short => Some(
            nc.values_i16(bytes, name)?
                .iter()
                .map(|x| *x as f64)
                .collect(),
        ),
        NetcdfType::Int => Some(
            nc.values_i32(bytes, name)?
                .iter()
                .map(|x| *x as f64)
                .collect(),
        ),
        _ => None,
    }
}

fn nc3_var(nc: &NetcdfFile, bytes: &[u8], name: &str) -> Option<Vec<f64>> {
    let v = nc.var(name)?;
    let scale = v
        .attrs
        .iter()
        .find(|a| a.name == "scale_factor")
        .and_then(|a| nc.attr_num(a));
    let offset = v
        .attrs
        .iter()
        .find(|a| a.name == "add_offset")
        .and_then(|a| nc.attr_num(a));
    let fill = v
        .attrs
        .iter()
        .find(|a| a.name == "_FillValue")
        .and_then(|a| nc.attr_num(a));
    nc3_series(nc, bytes, name).map(|raw| apply_scale(raw, scale, offset, fill))
}

fn nc3_first(nc: &NetcdfFile, bytes: &[u8], name: &str) -> Option<f64> {
    nc3_var(nc, bytes, name)?
        .into_iter()
        .find(|x| x.is_finite())
}

fn nc3_position(nc: &NetcdfFile, bytes: &[u8]) -> (Option<f64>, Option<f64>) {
    let lat = nc3_first(nc, bytes, "lat")
        .or_else(|| nc3_first(nc, bytes, "latitude"))
        .or_else(|| nc.gattr("occulting_lat").and_then(|a| nc.attr_num(a)))
        .or_else(|| nc.gattr("lat").and_then(|a| nc.attr_num(a)));
    let lon = nc3_first(nc, bytes, "lon")
        .or_else(|| nc3_first(nc, bytes, "longitude"))
        .or_else(|| nc.gattr("occulting_lon").and_then(|a| nc.attr_num(a)))
        .or_else(|| nc.gattr("lon").and_then(|a| nc.attr_num(a)));
    (lat, lon)
}

fn h5_attr_num(file: &Hdf5File, ds: &str, name: &str) -> Option<f64> {
    let a = file.attribute(ds, name)?;
    match (a.datatype.class, a.datatype.size) {
        (1, 4) => decode_f32(&a.data, 0, a.datatype.endian).map(|x| x as f64),
        (1, 8) => decode_f64(&a.data, 0, a.datatype.endian),
        _ => None,
    }
}

fn h5_var(file: &Hdf5File, name: &str) -> Option<Vec<f64>> {
    let raw = file.read_f64_dataset(name).ok()?;
    let fill = h5_attr_num(file, name, "_FillValue");
    Some(apply_scale(raw, None, None, fill))
}

fn h5_first(file: &Hdf5File, name: &str) -> Option<f64> {
    h5_var(file, name)?.into_iter().find(|x| x.is_finite())
}

fn h5_position(file: &Hdf5File) -> (Option<f64>, Option<f64>) {
    let lat = h5_first(file, "lat")
        .or_else(|| h5_first(file, "latitude"))
        .or_else(|| h5_attr_num(file, "/", "occulting_lat"))
        .or_else(|| h5_attr_num(file, "/", "lat"));
    let lon = h5_first(file, "lon")
        .or_else(|| h5_first(file, "longitude"))
        .or_else(|| h5_attr_num(file, "/", "occulting_lon"))
        .or_else(|| h5_attr_num(file, "/", "lon"));
    (lat, lon)
}

fn granule_time(name: &str) -> Option<f64> {
    let stem = name.strip_suffix("_nc")?;
    let mut parts = stem.split('.');
    let _product_sat = parts.next()?;
    let year: i64 = parts.next()?.parse().ok()?;
    let doy: i64 = parts.next()?.parse().ok()?;
    let _hh = parts.next()?;
    let _mm = parts.next()?;
    let _occ_year = parts.next()?;
    let secs: f64 = parts.next()?.parse().ok()?;
    if !(1..=366).contains(&doy) || !(0.0..=86400.0).contains(&secs) {
        return None;
    }
    let days = omegaflow::lsk::days_from_civil(year, 1, 1)? as f64;
    Some(days * 86400.0 + (doy - 1) as f64 * 86400.0 + secs)
}

fn comp_and_names(var: &str) -> Option<(u32, Vec<&'static str>)> {
    match var {
        "Ref" => Some((COMP_COSMIC_REFRACT, vec!["Ref"])),
        "Temp" => Some((COMP_COSMIC_TEMP, vec!["Temp", "Dry_Temp"])),
        "Pres" => Some((COMP_COSMIC_PRES, vec!["Pres"])),
        _ => None,
    }
}

fn stats(vals: &[f64]) -> (usize, Option<f64>, Option<f64>) {
    let mut n = 0usize;
    let mut min: Option<f64> = None;
    let mut max: Option<f64> = None;
    for &x in vals {
        if !x.is_finite() {
            continue;
        }
        n += 1;
        min = Some(match min {
            Some(m) => m.min(x),
            None => x,
        });
        max = Some(match max {
            Some(m) => m.max(x),
            None => x,
        });
    }
    (n, min, max)
}

fn num_str(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => "absent".to_string(),
    }
}

fn report_profile_nc3(nc: &NetcdfFile, bytes: &[u8]) {
    for name in PROFILE_VARS {
        match nc3_var(nc, bytes, name) {
            Some(vals) => {
                let (n, min, max) = stats(&vals);
                let units = nc
                    .var(name)
                    .and_then(|v| v.attrs.iter().find(|a| a.name == "units"))
                    .and_then(|a| nc.attr_text(a));
                let unit = units.as_deref().unwrap_or("");
                eprintln!(
                    "  {name}  levels {n}  min {}  max {} [{}]",
                    num_str(min),
                    num_str(max),
                    unit
                );
            }
            None => eprintln!("  {name}  absent"),
        }
    }
}

fn report_profile_h5(file: &Hdf5File) {
    for name in PROFILE_VARS {
        match h5_var(file, name) {
            Some(vals) => {
                let (n, min, max) = stats(&vals);
                let units = file.attribute(name, "units").map(|a| {
                    String::from_utf8_lossy(&a.data)
                        .trim_end_matches('\0')
                        .to_string()
                });
                let unit = units.as_deref().unwrap_or("");
                eprintln!(
                    "  {name}  levels {n}  min {}  max {} [{}]",
                    num_str(min),
                    num_str(max),
                    unit
                );
            }
            None => eprintln!("  {name}  absent"),
        }
    }
}

fn probe_granule(bytes: &[u8], name: &str) {
    if bytes.starts_with(b"CDF") {
        let Ok(nc) = NetcdfFile::parse(bytes) else {
            eprintln!("{name}: netCDF-3 parse returned void");
            return;
        };
        let fmt = match nc.format {
            NetcdfFormat::Cdf1 => "CDF-1",
            NetcdfFormat::Cdf2 => "CDF-2",
        };
        eprintln!(
            "{name}: netCDF-3 {fmt}, {} dims, {} vars",
            nc.dims.len(),
            nc.vars.len()
        );
        for d in &nc.dims {
            if d.len == 0 {
                eprintln!("  dim {} = record", d.name);
            } else {
                eprintln!("  dim {} = {}", d.name, d.len);
            }
        }
        for v in &nc.vars {
            let shape: Vec<String> = v
                .dim_ids
                .iter()
                .map(|&id| match nc.dims.get(id).map(|d| d.name.clone()) {
                    Some(v) => v,
                    None => id.to_string(),
                })
                .collect();
            eprintln!(
                "  var {} {} ({})",
                v.name,
                v.nc_type.name(),
                shape.join(",")
            );
        }
        let (lat, lon) = nc3_position(&nc, bytes);
        eprintln!("  position lat {} lon {}", num_str(lat), num_str(lon));
        report_profile_nc3(&nc, bytes);
        return;
    }
    if bytes.starts_with(&HDF5_MAGIC) {
        let Ok(file) = Hdf5File::parse(bytes) else {
            eprintln!("{name}: netCDF-4 parse returned void");
            return;
        };
        let group = match nc4_group(&file, "") {
            Ok(g) => g,
            Err(_) => {
                eprintln!("{name}: netCDF-4 group read returned void");
                return;
            }
        };
        eprintln!(
            "{name}: netCDF-4 (HDF5), {} variables",
            group.variables.len()
        );
        for v in &group.variables {
            eprintln!(
                "  var {} dims {:?} type-class {:?} type-size {:?}",
                v.name, v.dims, v.datatype_class, v.datatype_size
            );
        }
        let (lat, lon) = h5_position(&file);
        eprintln!("  position lat {} lon {}", num_str(lat), num_str(lon));
        report_profile_h5(&file);
        return;
    }
    eprintln!("{name}: unknown byte stream — neither CDF nor HDF5");
}

fn harvest_granule(
    bytes: &[u8],
    name: &str,
    comp: u32,
    var_names: &[&str],
    lsk: &LeapSeconds,
) -> Vec<GeoRec> {
    let (lat, lon, alt, val) = if bytes.starts_with(b"CDF") {
        let Ok(nc) = NetcdfFile::parse(bytes) else {
            return Vec::new();
        };
        let (lat, lon) = nc3_position(&nc, bytes);
        let Some(alt) = nc3_var(&nc, bytes, "MSL_alt") else {
            return Vec::new();
        };
        let Some(val) = var_names.iter().find_map(|n| nc3_var(&nc, bytes, n)) else {
            return Vec::new();
        };
        (lat, lon, alt, val)
    } else if bytes.starts_with(&HDF5_MAGIC) {
        let Ok(file) = Hdf5File::parse(bytes) else {
            return Vec::new();
        };
        let (lat, lon) = h5_position(&file);
        let Some(alt) = h5_var(&file, "MSL_alt") else {
            return Vec::new();
        };
        let Some(val) = var_names.iter().find_map(|n| h5_var(&file, n)) else {
            return Vec::new();
        };
        (lat, lon, alt, val)
    } else {
        return Vec::new();
    };
    let (Some(lat), Some(lon), Some(t_unix)) = (lat, lon, granule_time(name)) else {
        return Vec::new();
    };
    let Some(t) = lsk.unix_to_tdb(t_unix) else {
        return Vec::new();
    };
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) || !t.is_finite() {
        return Vec::new();
    }
    let n = alt.len().min(val.len());
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let a = alt[k];
        let v = val[k];
        if !a.is_finite() || !v.is_finite() || a < 0.0 {
            continue;
        }
        out.push(GeoRec {
            t,
            lat,
            lon,
            alt: a,
            freq: 0.0,
            bin_width: 0.0,
            val: v,
            comp,
            station: 0,
        });
    }
    out
}

fn run_probe(args: &[String]) -> Result<(), String> {
    if let Some(path) = arg_value(args, "--granule") {
        let bytes = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        probe_granule(&bytes, &path);
        return Ok(());
    }
    let tarball = arg_value(args, "--tarball")
        .ok_or_else(|| "cosmic_ro_compiler: --probe needs --granule or --tarball".to_string())?;
    let max = arg_usize(args, "--max-granules").unwrap_or(1);
    let tar = load_tar(&tarball)?;
    let members = tar_members(&tar);
    let granules: Vec<&TarMember> = members.iter().filter(|m| is_ro_granule(&m.name)).collect();
    eprintln!(
        "{tarball}: {} tar members, {} netCDF granules",
        members.len(),
        granules.len()
    );
    for m in granules.iter().take(max) {
        probe_granule(&tar[m.start..m.end], &m.name);
    }
    Ok(())
}

fn run_out_bin(args: &[String]) -> Result<(), String> {
    let var = match arg_value(args, "--var") {
        Some(v) => v,
        None => "Ref".to_string(),
    };
    let (comp, var_names) = comp_and_names(&var)
        .ok_or_else(|| format!("--var {var}: unknown profile variable (Ref|Temp|Pres)"))?;
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/cosmic_ro_{}.bin", var.to_lowercase()),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let max = arg_usize(args, "--max-granules");
    let Some(lsk) = embedded_lsk() else {
        return Err("naif0012 table void — the TDB epoch stays void (no fabricated epoch)".into());
    };

    let mut records: Vec<GeoRec> = Vec::new();
    if let Some(path) = arg_value(args, "--granule") {
        let bytes = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        records = harvest_granule(&bytes, &path, comp, &var_names, &lsk);
    } else if let Some(tarball) = arg_value(args, "--tarball") {
        let tar = load_tar(&tarball)?;
        let members = tar_members(&tar);
        let mut read = 0usize;
        for m in members {
            if !is_ro_granule(&m.name) {
                continue;
            }
            if let Some(cap) = max {
                if read >= cap {
                    break;
                }
            }
            read += 1;
            let recs = harvest_granule(&tar[m.start..m.end], &m.name, comp, &var_names, &lsk);
            if recs.is_empty() {
                eprintln!("{tarball}: {} carries no {var} level rows", m.name);
            } else {
                records.extend(recs);
            }
        }
    } else {
        return Err("cosmic_ro_compiler: --out-bin needs --tarball or --granule".to_string());
    }

    if records.is_empty() {
        return Err(
            "no profile level measured — the asset stays unwritten (0 honored)".to_string(),
        );
    }
    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.lat.total_cmp(&b.lat))
            .then(a.comp.cmp(&b.comp))
            .then(a.alt.total_cmp(&b.alt))
    });
    let bytes = write_bin(MAGIC_COSMIC, &records);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out_path, &bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    match parse_bin(MAGIC_COSMIC, &bytes) {
        Some(parsed) => eprintln!(
            "{out_path}: {} level records, {} B, roundtrip parses ({var})",
            parsed.len(),
            bytes.len()
        ),
        None => return Err(format!("{out_path}: roundtrip parse returned void")),
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: cosmic_ro_compiler <mode>");
        eprintln!("  --list [--year YYYY] [--doy DDD]");
        eprintln!("  --probe --granule <file.nc> | --tarball <url|file.tar.gz> [--max-granules N]");
        eprintln!(
            "  --out-bin --tarball <url|file.tar.gz> [--granule <file.nc>] [--var Ref|Temp|Pres] [--out <path>] [--max-granules N] [--ci-mode]"
        );
        std::process::exit(1);
    }
    let result = if args.iter().any(|a| a == "--out-bin") {
        run_out_bin(&args)
    } else if args.iter().any(|a| a == "--probe") {
        run_probe(&args)
    } else {
        run_list(&args)
    };
    if let Err(msg) = result {
        eprintln!("cosmic_ro_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tar_block(data: &[u8]) -> [u8; 512] {
        let mut b = [0u8; 512];
        let n = data.len().min(512);
        b[..n].copy_from_slice(&data[..n]);
        b
    }

    fn tar_header(name: &str, size: usize, typeflag: u8) -> [u8; 512] {
        let mut h = [0u8; 512];
        h[..name.len().min(100)].copy_from_slice(&name.as_bytes()[..name.len().min(100)]);
        let sz = format!("{size:o}\0");
        h[124..124 + sz.len()].copy_from_slice(sz.as_bytes());
        h[156] = typeflag;
        h[257..263].copy_from_slice(b"ustar\0");
        h
    }

    fn build_tar(members: &[(&str, &[u8])]) -> Vec<u8> {
        let mut out = Vec::new();
        for (name, content) in members {
            out.extend_from_slice(&tar_header(name, content.len(), b'0'));
            out.extend_from_slice(&tar_block(content));
        }
        out.extend_from_slice(&[0u8; 1024]);
        out
    }

    #[test]
    fn tarball_urls_carry_year_and_padded_doy() {
        assert_eq!(
            level2_tarball_url("wetPf2", 2026, 251),
            "https://data.cosmic.ucar.edu/gnss-ro/cosmic2/nrt/level2/2026/251/wetPf2_nrt_2026_251.tar.gz"
        );
        assert_eq!(
            level3_tarball_url("bubJL1", 2026, 251),
            "https://data.cosmic.ucar.edu/gnss-ro/cosmic2/nrt/level3/2026/251/bubJL1/bubJL1_nrt_2026_251.tar.gz"
        );
    }

    #[test]
    fn tar_octal_parses_size_and_name_reads_until_nul() {
        let mut f = [b' '; 12];
        f[..3].copy_from_slice(b"700");
        assert_eq!(tar_octal(&f), Some(448));
        let mut h = [0u8; 512];
        h[..4].copy_from_slice(b"abcd");
        assert_eq!(tar_name(&h), "abcd");
    }

    #[test]
    fn tar_members_enumerates_regular_files_only() {
        let tar = build_tar(&[
            ("atmPrf_C2E1.2026.251.00.02.G06_2026.3210_nc", b"CDF\x01"),
            ("ignored.json", b"{}"),
        ]);
        let members = tar_members(&tar);
        assert_eq!(members.len(), 2);
        assert!(is_ro_granule(&members[0].name));
        assert!(!is_ro_granule(&members[1].name));
        assert_eq!(&tar[members[0].start..members[0].end], b"CDF\x01");
    }

    #[test]
    fn granule_time_reads_seconds_of_day() {
        let t = granule_time("atmPrf_C2E1.2026.251.00.02.G06_2026.3210_nc");
        assert!(t.is_some());
        let t = t.unwrap();
        let day0 = omegaflow::lsk::days_from_civil(2026, 1, 1).unwrap() as f64 * 86400.0;
        assert!((t - (day0 + 250.0 * 86400.0 + 3210.0)).abs() < 1e-6);
        assert_eq!(granule_time("not_a_granule.txt"), None);
    }

    #[test]
    fn granule_unix_binds_through_the_leap_second_table() {
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        let t_unix = granule_time("atmPrf_C2E1.2026.251.00.02.G06_2026.3210_nc").unwrap();
        let tdb = lsk
            .unix_to_tdb(t_unix)
            .expect("a 2026 occultation carries a TDB epoch");
        let j2000 = 946728000.0f64;
        assert!(tdb.is_finite());
        assert!(
            tdb > t_unix - j2000 - 1.0 && tdb < t_unix - j2000 + 100.0,
            "the occultation TDB lands in the J2000 domain, was {tdb}"
        );
        assert!(
            lsk.unix_to_tdb(-40_000_000.0).is_none(),
            "pre-1972 the leap table reads void — no fabricated epoch"
        );
    }

    #[test]
    fn comp_mapping_names_profile_variables() {
        assert_eq!(
            comp_and_names("Ref").map(|(c, _)| c),
            Some(COMP_COSMIC_REFRACT)
        );
        assert_eq!(
            comp_and_names("Temp").map(|(_, v)| v),
            Some(vec!["Temp", "Dry_Temp"])
        );
        assert_eq!(
            comp_and_names("Pres").map(|(c, _)| c),
            Some(COMP_COSMIC_PRES)
        );
        assert!(comp_and_names("Shum").is_none());
    }

    #[test]
    fn cosmic1_wetprf_fixture_profile_reads() {
        const FIXTURE: &str =
            "phi/pipeline/catalog/cosmic_wetprf/wetPrf_C001.2014.121.00.02.G27_2014.2860_nc";
        if !std::path::Path::new(FIXTURE).exists() {
            eprintln!(
                "skipped (fixture absent): cosmic wetPrf — fetch from data.cosmic.ucar.edu/gnss-ro/cosmic1/postProc/level2/2014/121/wetPrf_postProc_2014_121.tar.gz"
            );
            return;
        }
        let bytes = std::fs::read(FIXTURE).expect("fixture read");
        let nc = NetcdfFile::parse(&bytes).unwrap();
        let alt = nc3_var(&nc, &bytes, "MSL_alt").expect("MSL_alt");
        let temp = nc3_var(&nc, &bytes, "Temp").expect("Temp");
        assert_eq!(alt.len(), temp.len());
        let (n, _, _) = stats(&temp);
        assert!(n > 0, "temperature levels stay unread");
    }
}
