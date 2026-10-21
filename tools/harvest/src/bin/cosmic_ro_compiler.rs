use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::geo::{
    COMP_COSMIC_PRES, COMP_COSMIC_REFRACT, COMP_COSMIC_TEMP, GeoRec, MAGIC_COSMIC, parse_bin,
    write_bin,
};
use omegaflow::hdf5::{Hdf5File, decode_f32, decode_f64};
use omegaflow::inflate::gunzip;
use omegaflow::lsk::LeapSeconds;
use omegaflow::netcdf::{NetcdfFile, NetcdfFormat, NetcdfType, directory_links, nc4_group};

const NETLOC: &str = "data.cosmic.ucar.edu";
const ROOT_COSMIC2: &str = "https://data.cosmic.ucar.edu/gnss-ro/cosmic2/nrt";
const ROOT_COSMIC1: &str = "https://data.cosmic.ucar.edu/gnss-ro/cosmic1/postProc";
const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];

const LEVEL2_PRODUCTS: [&str; 5] = ["atmPrf", "avnPrf", "bfrPrf", "echPrf", "wetPf2"];
const LEVEL3_PRODUCTS: [&str; 16] = [
    "alcDat", "alcDt2", "alcPl2", "alcPlt", "bubJL1", "bubJL102", "bubJU1", "bubJU102", "bubJsn",
    "bubJsn02", "bubML1", "bubML102", "bubMU1", "bubMU102", "bubMap", "bubMap02",
];

const COSMIC1_LEVEL1A_PRODUCTS: [&str; 2] = ["leoAtt", "podCrx"];
const COSMIC1_LEVEL1B_PRODUCTS: [&str; 7] = [
    "atmPhs", "gpsBit", "ionPhs", "leoClk", "leoOrb", "podTec", "scnLv1",
];
const COSMIC1_LEVEL2_PRODUCTS: [&str; 7] = [
    "atmPrf", "bfrPrf", "echPrf", "eraPrf", "gfsPrf", "ionPrf", "wetPrf",
];

const PROFILE_VARS: [&str; 5] = ["MSL_alt", "Ref", "Temp", "Dry_Temp", "Pres"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mission {
    Cosmic1,
    Cosmic2,
}

impl Mission {
    fn parse(text: &str) -> Option<Self> {
        match text {
            "cosmic1" | "cosmic-1" | "c1" => Some(Self::Cosmic1),
            "cosmic2" | "cosmic-2" | "c2" => Some(Self::Cosmic2),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Cosmic1 => "cosmic1",
            Self::Cosmic2 => "cosmic2",
        }
    }

    fn root(self) -> &'static str {
        match self {
            Self::Cosmic1 => ROOT_COSMIC1,
            Self::Cosmic2 => ROOT_COSMIC2,
        }
    }

    fn levels(self) -> &'static [&'static str] {
        match self {
            Self::Cosmic1 => &["level1a", "level1b", "level2"],
            Self::Cosmic2 => &["level2", "level3"],
        }
    }

    fn products(self, level: &str) -> &'static [&'static str] {
        match (self, level) {
            (Self::Cosmic1, "level1a") => &COSMIC1_LEVEL1A_PRODUCTS,
            (Self::Cosmic1, "level1b") => &COSMIC1_LEVEL1B_PRODUCTS,
            (Self::Cosmic1, "level2") => &COSMIC1_LEVEL2_PRODUCTS,
            (Self::Cosmic2, "level2") => &LEVEL2_PRODUCTS,
            (Self::Cosmic2, "level3") => &LEVEL3_PRODUCTS,
            _ => &[],
        }
    }

    fn default_product(self, level: &str) -> Option<&'static str> {
        match (self, level) {
            (Self::Cosmic1, "level2") => Some("wetPrf"),
            (Self::Cosmic1, "level1b") => Some("atmPhs"),
            (Self::Cosmic1, "level1a") => Some("podCrx"),
            (Self::Cosmic2, "level2") => Some("wetPf2"),
            (Self::Cosmic2, "level3") => Some("bubJL1"),
            _ => None,
        }
    }

    fn tarball_url(self, level: &str, product: &str, year: u32, doy: u32) -> Option<String> {
        if !self.levels().contains(&level) || !self.products(level).contains(&product) {
            return None;
        }
        let root = self.root();
        Some(match self {
            Self::Cosmic1 => {
                format!("{root}/{level}/{year}/{doy:03}/{product}_postProc_{year}_{doy:03}.tar.gz")
            }
            Self::Cosmic2 if level == "level3" => format!(
                "{root}/{level}/{year}/{doy:03}/{product}/{product}_nrt_{year}_{doy:03}.tar.gz"
            ),
            Self::Cosmic2 => {
                format!("{root}/{level}/{year}/{doy:03}/{product}_nrt_{year}_{doy:03}.tar.gz")
            }
        })
    }
}

fn load_lsk(path: Option<&str>) -> Option<LeapSeconds> {
    match path {
        Some(p) => {
            let text = std::fs::read_to_string(p).ok()?;
            omegaflow::lsk::parse(&text)
        }
        None => embedded_lsk(),
    }
}

fn mission_of(args: &[String]) -> Result<Mission, String> {
    match arg_value(args, "--mission") {
        Some(v) => {
            Mission::parse(&v).ok_or_else(|| format!("--mission {v}: unknown (cosmic1|cosmic2)"))
        }
        None => Ok(Mission::Cosmic2),
    }
}

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
    let bytes = fetch_raw_bytes(url)?;
    String::from_utf8(bytes).ok()
}

fn naming_scheme(mission: Mission) {
    eprintln!("{} tree ({}):", mission.name(), mission.root());
    for level in mission.levels() {
        eprintln!(
            "  {level}/{{year}}/{{doy:03}}/ carries daily tarballs (products {:?}):",
            mission.products(level)
        );
        let example = mission
            .default_product(level)
            .and_then(|p| mission.tarball_url(level, p, 2026, 251));
        if let Some(url) = example {
            eprintln!("    {url}");
        }
    }
    match mission {
        Mission::Cosmic1 => {
            eprintln!(
                "  granule: {{product}}_{{sat}}.{{year}}.{{doy}}.{{hh}}.{{mm}}.{{occ}}_{{year}}.{{secs}}_nc"
            );
            eprintln!(
                "  level1b netCDF: atmPhs, ionPhs, podTec, scnLv1; level1b SP3/CLK/text: leoOrb, leoClk, gpsBit; level1a: leoAtt (text), podCrx (raw .crx.gz, not netCDF)"
            );
        }
        Mission::Cosmic2 => {
            eprintln!(
                "  granule: {{product}}_{{sat}}.{{year}}.{{doy}}.{{hh}}.{{mm}}.{{occ}}_{{year}}.{{secs}}_nc"
            );
        }
    }
}

fn list_links(body: &str) -> Vec<String> {
    directory_links(body)
        .into_iter()
        .filter(|n| n != "../" && !n.is_empty())
        .collect()
}

fn run_list(args: &[String]) -> Result<(), String> {
    let mission = mission_of(args)?;
    naming_scheme(mission);
    let year = arg_value(args, "--year").and_then(|v| v.parse::<u32>().ok());
    let doy = arg_value(args, "--doy").and_then(|v| v.parse::<u32>().ok());
    let only_level = arg_value(args, "--level");
    for level in mission.levels() {
        if let Some(want) = &only_level {
            if want != level {
                continue;
            }
        }
        let index_url = match year {
            Some(y) => match doy {
                Some(d) => format!("{}/{level}/{y}/{d:03}/", mission.root()),
                None => format!("{}/{level}/{y}/", mission.root()),
            },
            None => format!("{}/{level}/", mission.root()),
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

fn tarball_arg(args: &[String], mission: Mission) -> Result<Option<String>, String> {
    if let Some(path) = arg_value(args, "--tarball") {
        return Ok(Some(path));
    }
    let year = arg_value(args, "--year").and_then(|v| v.parse::<u32>().ok());
    let doy = arg_value(args, "--doy").and_then(|v| v.parse::<u32>().ok());
    let (Some(year), Some(doy)) = (year, doy) else {
        return Ok(None);
    };
    if !(1..=366).contains(&doy) {
        return Err(format!("--doy {doy}: outside 1..=366"));
    }
    let level = match arg_value(args, "--level") {
        Some(v) => v,
        None => "level2".to_string(),
    };
    let product = match arg_value(args, "--product") {
        Some(p) => p,
        None => mission
            .default_product(&level)
            .ok_or_else(|| format!("{level}: no default product for {}", mission.name()))?
            .to_string(),
    };
    mission
        .tarball_url(&level, &product, year, doy)
        .map(Some)
        .ok_or_else(|| {
            format!(
                "{} {level} {product}: not in the measured tree (levels {:?}, products {:?})",
                mission.name(),
                mission.levels(),
                mission.products(&level)
            )
        })
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
    if any { Some(v) } else { None }
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
        fetch_raw_bytes(arg).ok_or_else(|| format!("{arg}: fetch returned void"))?
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
            if y.is_finite() { y } else { f64::NAN }
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
        let Ok(mut file) = Hdf5File::parse(bytes) else {
            eprintln!("{name}: netCDF-4 parse returned void");
            return;
        };
        let group = match nc4_group(&mut file, "") {
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

fn nc3_units(nc: &NetcdfFile, name: &str) -> Option<String> {
    nc.var(name)
        .and_then(|v| v.attrs.iter().find(|a| a.name == "units"))
        .and_then(|a| nc.attr_text(a))
}

fn h5_units(file: &Hdf5File, name: &str) -> Option<String> {
    file.attribute(name, "units").map(|a| {
        String::from_utf8_lossy(&a.data)
            .trim_end_matches('\0')
            .to_string()
    })
}

fn canonical_units(comp: u32, unit: Option<&str>, vals: Vec<f64>) -> Vec<f64> {
    if comp != COMP_COSMIC_TEMP {
        return vals;
    }
    let Some(unit) = unit else {
        return vals;
    };
    let u = unit.trim().to_ascii_lowercase();
    if u == "c" || u.starts_with("deg c") || u.starts_with("degree") || u.contains("celsius") {
        return vals
            .into_iter()
            .map(|v| if v.is_finite() { v + 273.15 } else { v })
            .collect();
    }
    vals
}

fn harvest_granule(
    bytes: &[u8],
    name: &str,
    comp: u32,
    var_names: &[&str],
    lsk: &LeapSeconds,
) -> Vec<GeoRec> {
    let (lat, lon, alt, val, unit) = if bytes.starts_with(b"CDF") {
        let Ok(nc) = NetcdfFile::parse(bytes) else {
            return Vec::new();
        };
        let (lat, lon) = nc3_position(&nc, bytes);
        let Some(alt) = nc3_var(&nc, bytes, "MSL_alt") else {
            return Vec::new();
        };
        let Some((val, unit)) = var_names
            .iter()
            .find_map(|n| nc3_var(&nc, bytes, n).map(|v| (v, nc3_units(&nc, n))))
        else {
            return Vec::new();
        };
        (lat, lon, alt, val, unit)
    } else if bytes.starts_with(&HDF5_MAGIC) {
        let Ok(file) = Hdf5File::parse(bytes) else {
            return Vec::new();
        };
        let (lat, lon) = h5_position(&file);
        let Some(alt) = h5_var(&file, "MSL_alt") else {
            return Vec::new();
        };
        let Some((val, unit)) = var_names
            .iter()
            .find_map(|n| h5_var(&file, n).map(|v| (v, h5_units(&file, n))))
        else {
            return Vec::new();
        };
        (lat, lon, alt, val, unit)
    } else {
        return Vec::new();
    };
    let val = canonical_units(comp, unit.as_deref(), val);
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
    let mission = mission_of(args)?;
    if let Some(path) = arg_value(args, "--granule") {
        let bytes = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        probe_granule(&bytes, &path);
        return Ok(());
    }
    let tarball = tarball_arg(args, mission)?.ok_or_else(|| {
        "cosmic_ro_compiler: --probe needs --granule, --tarball or --year/--doy".to_string()
    })?;
    eprintln!("{}: {tarball}", mission.name());
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
    let mission = mission_of(args)?;
    let var = match arg_value(args, "--var") {
        Some(v) => v,
        None => "Ref".to_string(),
    };
    let (comp, var_names) = comp_and_names(&var)
        .ok_or_else(|| format!("--var {var}: unknown profile variable (Ref|Temp|Pres)"))?;
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!(
            "data/{NETLOC}/{}_ro_{}.bin",
            mission.name(),
            var.to_lowercase()
        ),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let max = arg_usize(args, "--max-granules");
    let Some(lsk) = load_lsk(arg_value(args, "--lsk").as_deref()) else {
        return Err("naif0012 table void — the TDB epoch stays void (no fabricated epoch)".into());
    };

    let mut records: Vec<GeoRec> = Vec::new();
    if let Some(path) = arg_value(args, "--granule") {
        let bytes = std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
        records = harvest_granule(&bytes, &path, comp, &var_names, &lsk);
    } else if let Some(tarball) = tarball_arg(args, mission)? {
        eprintln!("{}: {tarball}", mission.name());
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
        eprintln!("usage: cosmic_ro_compiler <mode> [--mission cosmic1|cosmic2]");
        eprintln!("  --list [--level <level>] [--year YYYY] [--doy DDD]");
        eprintln!(
            "  --probe --granule <file.nc> | (--tarball <url|file.tar.gz> | --year YYYY --doy DDD [--level <level> --product <name>]) [--max-granules N]"
        );
        eprintln!(
            "  --out-bin (--tarball <url|file.tar.gz> | --granule <file.nc> | --year YYYY --doy DDD [--level <level> --product <name>]) [--var Ref|Temp|Pres] [--out <path>] [--lsk <naif0012.tls>] [--max-granules N] [--ci-mode]"
        );
        eprintln!(
            "  cosmic1: level1a (leoAtt,podCrx), level1b (atmPhs,gpsBit,ionPhs,leoClk,leoOrb,podTec,scnLv1), level2 (atmPrf,bfrPrf,echPrf,eraPrf,gfsPrf,ionPrf,wetPrf)"
        );
        eprintln!(
            "  cosmic2: level2 (atmPrf,avnPrf,bfrPrf,echPrf,wetPf2), level3 (ionosphere scintillation)"
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
        let m = Mission::Cosmic2;
        assert_eq!(
            m.tarball_url("level2", "wetPf2", 2026, 251).unwrap(),
            "https://data.cosmic.ucar.edu/gnss-ro/cosmic2/nrt/level2/2026/251/wetPf2_nrt_2026_251.tar.gz"
        );
        assert_eq!(
            m.tarball_url("level3", "bubJL1", 2026, 251).unwrap(),
            "https://data.cosmic.ucar.edu/gnss-ro/cosmic2/nrt/level3/2026/251/bubJL1/bubJL1_nrt_2026_251.tar.gz"
        );
    }

    #[test]
    fn cosmic1_url_template_covers_all_three_levels() {
        let m = Mission::Cosmic1;
        assert_eq!(
            m.tarball_url("level2", "wetPrf", 2014, 121).unwrap(),
            "https://data.cosmic.ucar.edu/gnss-ro/cosmic1/postProc/level2/2014/121/wetPrf_postProc_2014_121.tar.gz"
        );
        assert_eq!(
            m.tarball_url("level1b", "atmPhs", 2014, 121).unwrap(),
            "https://data.cosmic.ucar.edu/gnss-ro/cosmic1/postProc/level1b/2014/121/atmPhs_postProc_2014_121.tar.gz"
        );
        assert_eq!(
            m.tarball_url("level1a", "podCrx", 2014, 121).unwrap(),
            "https://data.cosmic.ucar.edu/gnss-ro/cosmic1/postProc/level1a/2014/121/podCrx_postProc_2014_121.tar.gz"
        );
        assert_eq!(m.tarball_url("level3", "bubJL1", 2014, 121), None);
    }

    #[test]
    fn mission_aliases_select_the_measured_tree() {
        assert_eq!(Mission::parse("cosmic1"), Some(Mission::Cosmic1));
        assert_eq!(Mission::parse("cosmic-1"), Some(Mission::Cosmic1));
        assert_eq!(Mission::parse("cosmic2"), Some(Mission::Cosmic2));
        assert_eq!(Mission::parse("cosmic9"), None);
        assert_eq!(
            Mission::Cosmic1.products("level2"),
            &COSMIC1_LEVEL2_PRODUCTS
        );
    }

    #[test]
    fn canonical_units_pair_cosmic1_celsius_with_the_kelvin_field() {
        let c = canonical_units(COMP_COSMIC_TEMP, Some("C"), vec![0.0, -48.5]);
        assert!((c[0] - 273.15).abs() < 1e-12);
        assert!((c[1] - 224.65).abs() < 1e-12);
        let k = canonical_units(COMP_COSMIC_TEMP, Some("K"), vec![273.15]);
        assert_eq!(k, vec![273.15]);
        let absent = canonical_units(COMP_COSMIC_TEMP, None, vec![12.0]);
        assert_eq!(absent, vec![12.0]);
        let refr = canonical_units(COMP_COSMIC_REFRACT, Some("N"), vec![300.0]);
        assert_eq!(refr, vec![300.0]);
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
        let pre_1972 = lsk
            .unix_to_tdb(-40_000_000.0)
            .expect("the Delta T polynomial anchors pre-1972 epochs");
        assert!(pre_1972.is_finite(), "pre-1972 stays a measured epoch");
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
            "src/archivar/testdata/cosmic_wetprf/wetPrf_C001.2014.121.00.02.G27_2014.2860_nc";
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
        let lsk = embedded_lsk().expect("embedded naif0012 parses");
        let recs = harvest_granule(
            &bytes,
            "wetPrf_C001.2014.121.00.02.G27_2014.2860_nc",
            COMP_COSMIC_TEMP,
            &["Temp"],
            &lsk,
        );
        assert!(!recs.is_empty(), "the profile harvest stays empty");
        assert!(
            recs.iter().all(|r| (200.0..=320.0).contains(&r.val)),
            "Temp header unit C must reach the K field as kelvin"
        );
    }
}
