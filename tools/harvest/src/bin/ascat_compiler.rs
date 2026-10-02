use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{
    COMP_ASCAT_WDIR, COMP_ASCAT_WSPD, GeoRec, MAGIC_ASCAT, parse_bin, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{
    Endian, Hdf5Attribute, Hdf5Datatype, Hdf5File, Hdf5Object, decode_f32, decode_f64,
};
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const DEFAULT_NETLOC: &str = "manati.star.nesdis.noaa.gov-ascat";

const LAT_KEYS: [&str; 2] = ["lat", "latitude"];
const LON_KEYS: [&str; 2] = ["lon", "longitude"];
const TIME_KEYS: [&str; 3] = ["time", "timeM", "tdays"];
const SPEED_KEYS: [&str; 4] = ["wind_speed", "speed", "wspd", "L2B_speed"];
const DIR_KEYS: [&str; 5] = ["wind_dir", "wind_direction", "dir", "wdir", "L2B_dir"];

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("600")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "ascat: fetch {} -> {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn read_source(source: &str) -> Option<Vec<u8>> {
    if source.starts_with("http://") || source.starts_with("https://") {
        fetch(source)
    } else {
        match std::fs::read(source) {
            Ok(b) => Some(b),
            Err(e) => {
                eprintln!("ascat: read {source} -> {e}");
                None
            }
        }
    }
}

fn host_of(source: &str) -> Option<String> {
    let rest = source.split("//").nth(1)?;
    let host = rest.split('/').next()?;
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

fn attr_find<'a>(attrs: &'a [Hdf5Attribute], name: &str) -> Option<&'a Hdf5Attribute> {
    attrs.iter().find(|a| a.name == name)
}

fn decode_int_at(
    data: &[u8],
    off: usize,
    size: usize,
    endian: Endian,
    signed: bool,
) -> Option<i64> {
    let be = endian == Endian::Be;
    match size {
        1 => data
            .get(off)
            .map(|&b| if signed { b as i8 as i64 } else { b as i64 }),
        2 => {
            let b: [u8; 2] = data.get(off..off + 2)?.try_into().ok()?;
            let v = if be {
                u16::from_be_bytes(b)
            } else {
                u16::from_le_bytes(b)
            };
            Some(if signed { v as i16 as i64 } else { v as i64 })
        }
        4 => {
            let b: [u8; 4] = data.get(off..off + 4)?.try_into().ok()?;
            let v = if be {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            };
            Some(if signed { v as i32 as i64 } else { v as i64 })
        }
        8 => {
            let b: [u8; 8] = data.get(off..off + 8)?.try_into().ok()?;
            let v = if be {
                u64::from_be_bytes(b)
            } else {
                u64::from_le_bytes(b)
            };
            Some(v as i64)
        }
        _ => None,
    }
}

fn attr_number(attrs: &[Hdf5Attribute], name: &str) -> Option<f64> {
    let a = attr_find(attrs, name)?;
    match a.datatype.class {
        0 => decode_int_at(
            &a.data,
            0,
            a.datatype.size,
            a.datatype.endian,
            a.datatype.signed,
        )
        .map(|v| v as f64),
        1 => match a.datatype.size {
            4 => decode_f32(&a.data, 0, a.datatype.endian).map(|v| v as f64),
            8 => decode_f64(&a.data, 0, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
}

fn attr_text(attrs: &[Hdf5Attribute], name: &str) -> Option<String> {
    let a = attr_find(attrs, name)?;
    let end = a.data.iter().position(|&b| b == 0).unwrap_or(a.data.len());
    let text = std::str::from_utf8(a.data.get(..end)?).ok()?.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

struct Var {
    raw: Vec<u8>,
    dt: Hdf5Datatype,
    dims: Vec<u64>,
}

fn read_var(file: &Hdf5File, name: &str) -> Result<Var, String> {
    let (_, ds, dt) = file
        .dataset(name)
        .map_err(|e| format!("{name}: dataset reads void ({e:?})"))?;
    if dt.size == 0 {
        return Err(format!("{name}: datatype size reads void"));
    }
    let count = ds
        .dims
        .iter()
        .try_fold(1usize, |a, d| a.checked_mul(*d as usize))
        .ok_or_else(|| format!("{name}: dataspace overflows"))?;
    let raw = file
        .read_dataset(name)
        .map_err(|e| format!("{name}: read void ({e:?})"))?;
    if raw.len() != count * dt.size {
        return Err(format!(
            "{name}: {} B read against {count} cells x {} B",
            raw.len(),
            dt.size
        ));
    }
    Ok(Var {
        raw,
        dt: dt.clone(),
        dims: ds.dims.clone(),
    })
}

fn decode_num(raw: &[u8], off: usize, dt: &Hdf5Datatype) -> Option<f64> {
    match dt.class {
        1 => match dt.size {
            4 => decode_f32(raw, off, dt.endian).map(|v| v as f64),
            8 => decode_f64(raw, off, dt.endian),
            _ => None,
        },
        0 => decode_int_at(raw, off, dt.size, dt.endian, dt.signed).map(|v| v as f64),
        _ => None,
    }
}

struct Numbers {
    scale: f64,
    offset: f64,
    fill: Option<f64>,
}

fn numbers_of(obj: &Hdf5Object) -> Numbers {
    let scale = match attr_number(&obj.attrs, "scale_factor") {
        Some(v) => v,
        None => 1.0,
    };
    let offset = match attr_number(&obj.attrs, "add_offset") {
        Some(v) => v,
        None => 0.0,
    };
    Numbers {
        scale,
        offset,
        fill: attr_number(&obj.attrs, "_FillValue"),
    }
}

fn first_var<'a>(file: &'a Hdf5File, keys: &[&str]) -> Option<(String, Var)> {
    for key in keys {
        if let Ok(v) = read_var(file, key) {
            return Some(((*key).to_string(), v));
        }
    }
    None
}

fn time_unit(units: &str) -> Option<(f64, f64)> {
    let lower = units.to_lowercase();
    let (unit, since) = lower.split_once(" since ")?;
    let unit_seconds = match unit.trim() {
        "seconds" | "second" | "s" => 1.0,
        "minutes" | "minute" => 60.0,
        "hours" | "hour" | "h" => 3600.0,
        "days" | "day" | "d" => 86400.0,
        _ => return None,
    };
    let since_norm = since.replace('t', " ");
    let mut since_parts = since_norm.split_whitespace();
    let date_part = since_parts.next()?;
    let mut d = date_part.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let time_part = since_parts.next().unwrap_or("00:00:00");
    let mut t = time_part.split(':');
    let hh: f64 = match t.next().and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => 0.0,
    };
    let mm: f64 = match t.next().and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => 0.0,
    };
    let ss: f64 = match t.next().and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => 0.0,
    };
    Some((
        unit_seconds,
        days as f64 * 86400.0 + hh * 3600.0 + mm * 60.0 + ss,
    ))
}

fn inspect(file: &Hdf5File) {
    for link in file.links_of("") {
        if link.addr == u64::MAX {
            continue;
        }
        let Ok((obj, ds, dt)) = file.dataset(&link.name) else {
            continue;
        };
        eprintln!(
            "var {} dims {:?} class {} size {} signed {} endian {:?}",
            link.name, ds.dims, dt.class, dt.size, dt.signed, dt.endian
        );
        for a in &obj.attrs {
            match a.datatype.class {
                0 | 1 => {
                    let num = match a.datatype.class {
                        0 => decode_int_at(
                            &a.data,
                            0,
                            a.datatype.size,
                            a.datatype.endian,
                            a.datatype.signed,
                        )
                        .map(|v| v as f64),
                        _ => match a.datatype.size {
                            4 => decode_f32(&a.data, 0, a.datatype.endian).map(|v| v as f64),
                            8 => decode_f64(&a.data, 0, a.datatype.endian),
                            _ => None,
                        },
                    };
                    if let Some(v) = num {
                        eprintln!("    attr {} = {}", a.name, v);
                    }
                }
                _ => {
                    if let Some(t) = attr_text(&obj.attrs, &a.name) {
                        eprintln!("    attr {} = {:?}", a.name, t);
                    }
                }
            }
        }
    }
}

enum Grid {
    Swath { n: usize },
    Axes { n_lat: usize, n_lon: usize },
}

impl Grid {
    fn n_cells(&self) -> usize {
        match self {
            Grid::Swath { n } => *n,
            Grid::Axes { n_lat, n_lon } => n_lat * n_lon,
        }
    }

    fn lat_index(&self, j: usize) -> usize {
        match self {
            Grid::Swath { .. } => j,
            Grid::Axes { n_lon, .. } => j / n_lon,
        }
    }

    fn lon_index(&self, j: usize) -> usize {
        match self {
            Grid::Swath { .. } => j,
            Grid::Axes { n_lon, .. } => j % n_lon,
        }
    }
}

fn time_index(grid: &Grid, time_total: usize, j: usize) -> Option<usize> {
    if time_total == 1 {
        return Some(0);
    }
    match grid {
        Grid::Swath { n } if time_total == *n => Some(j),
        Grid::Axes { n_lat, n_lon } if time_total == n_lat * n_lon => Some(j),
        Grid::Axes { n_lon, .. } if time_total == *n_lon => Some(j % n_lon),
        Grid::Axes { n_lat, n_lon } if time_total == *n_lat => Some(j / n_lon),
        _ => None,
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let inspect_mode = args.iter().any(|a| a == "--inspect");
    let source = positional_source(args)
        .ok_or_else(|| "no source — pass an ASCAT .nc URL or local path".to_string())?;
    let label = arg_value(args, "--label").filter(|l| !l.is_empty());
    let netloc = match arg_value(args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => match host_of(&source) {
            Some(h) if source.starts_with("http") => format!("{h}-ascat"),
            _ => DEFAULT_NETLOC.to_string(),
        },
    };
    let stride = match arg_value(args, "--stride") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--stride {v} carries no step"))?,
        None => 8,
    };
    if stride == 0 {
        return Err("--stride carries no positive sampling step".to_string());
    }

    let bytes = read_source(&source).ok_or_else(|| format!("{source}: read void"))?;
    let file = Hdf5File::parse(&bytes).map_err(|e| format!("{source}: hdf5 void ({e:?})"))?;

    if inspect_mode {
        inspect(&file);
        return Ok(());
    }
    let label =
        label.ok_or_else(|| "--label <product> is required — it names the lineage".to_string())?;

    let lsk = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no epoch".to_string())?;

    let (lat_name, lat) = first_var(&file, &LAT_KEYS)
        .ok_or_else(|| format!("{source}: no lat variable among {LAT_KEYS:?}"))?;
    let (lon_name, lon) = first_var(&file, &LON_KEYS)
        .ok_or_else(|| format!("{source}: no lon variable among {LON_KEYS:?}"))?;
    let (time_name, time) = first_var(&file, &TIME_KEYS)
        .ok_or_else(|| format!("{source}: no time variable among {TIME_KEYS:?}"))?;
    let (speed_name, speed) = first_var(&file, &SPEED_KEYS)
        .ok_or_else(|| format!("{source}: no wind speed variable among {SPEED_KEYS:?}"))?;

    let lat_total = lat.dims.iter().product::<u64>() as usize;
    let lon_total = lon.dims.iter().product::<u64>() as usize;
    let speed_total = speed.dims.iter().product::<u64>() as usize;
    let time_total = time.dims.iter().product::<u64>() as usize;
    let grid = if lat.dims.len() == 1
        && lon.dims.len() == 1
        && lat_total > 1
        && lon_total > 1
        && lat_total.checked_mul(lon_total) == Some(speed_total)
    {
        Grid::Axes {
            n_lat: lat_total,
            n_lon: lon_total,
        }
    } else if lat_total == speed_total && lon_total == speed_total && speed_total > 0 {
        Grid::Swath { n: speed_total }
    } else {
        return Err(format!(
            "{source}: lat {lat_name} {:?} / lon {lon_name} {:?} / speed {speed_name} {:?} carry no common grid — the bin stays unwritten",
            lat.dims, lon.dims, speed.dims
        ));
    };
    let n_cells = grid.n_cells();
    if time_index(&grid, time_total, 0).is_none() {
        return Err(format!(
            "{source}: time {time_name} {:?} does not map onto the {n_cells}-cell grid — the bin stays unwritten",
            time.dims
        ));
    }

    let lat_obj = file
        .dataset(&lat_name)
        .map(|(o, _, _)| o)
        .map_err(|e| format!("{lat_name}: {e:?}"))?;
    let lon_obj = file
        .dataset(&lon_name)
        .map(|(o, _, _)| o)
        .map_err(|e| format!("{lon_name}: {e:?}"))?;
    let time_obj = file
        .dataset(&time_name)
        .map(|(o, _, _)| o)
        .map_err(|e| format!("{time_name}: {e:?}"))?;
    let speed_obj = file
        .dataset(&speed_name)
        .map(|(o, _, _)| o)
        .map_err(|e| format!("{speed_name}: {e:?}"))?;

    let units = attr_text(&time_obj.attrs, "units")
        .ok_or_else(|| format!("{time_name}: units absent — the epoch stays unmeasured"))?;
    let (unit_seconds, epoch_unix) = time_unit(&units)
        .ok_or_else(|| format!("{time_name}: units {units:?} carries no 'since' epoch"))?;
    eprintln!("{source}: time units {:?}", units);

    let lat_n = numbers_of(lat_obj);
    let lon_n = numbers_of(lon_obj);
    let time_n = numbers_of(time_obj);
    let speed_n = numbers_of(speed_obj);
    let dir = match first_var(&file, &DIR_KEYS) {
        Some((name, var)) if var.dims.iter().product::<u64>() as usize == n_cells => {
            Some((name, var))
        }
        Some((name, var)) => {
            eprintln!(
                "{source}: direction {name} dims {:?} do not map onto the {n_cells}-cell grid — direction left absent",
                var.dims
            );
            None
        }
        None => None,
    };
    let dir_n = match &dir {
        Some((name, _)) => file.dataset(name).map(|(o, _, _)| numbers_of(o)).ok(),
        None => None,
    };

    let mut records: Vec<GeoRec> = Vec::new();
    let mut speed_kept = 0usize;
    let mut dir_kept = 0usize;
    let mut j = 0usize;
    while j < n_cells {
        let li = grid.lat_index(j);
        let oi = grid.lon_index(j);
        let ti = match time_index(&grid, time_total, j) {
            Some(t) => t,
            None => break,
        };
        let lat_v = match decode_num(&lat.raw, li * lat.dt.size, &lat.dt) {
            Some(v) if lat_n.fill != Some(v) => Some(v * lat_n.scale + lat_n.offset),
            _ => None,
        };
        let lon_v = match decode_num(&lon.raw, oi * lon.dt.size, &lon.dt) {
            Some(v) if lon_n.fill != Some(v) => Some(v * lon_n.scale + lon_n.offset),
            _ => None,
        };
        let time_v = match decode_num(&time.raw, ti * time.dt.size, &time.dt) {
            Some(v) if time_n.fill != Some(v) => Some(v * time_n.scale + time_n.offset),
            _ => None,
        };
        let speed_raw = decode_num(&speed.raw, j * speed.dt.size, &speed.dt);
        let speed_v = match speed_raw {
            Some(v) if speed_n.fill != Some(v) => Some(v * speed_n.scale + speed_n.offset),
            _ => None,
        };
        if let (Some(lat_v), Some(lon_v), Some(time_v), Some(speed_v)) =
            (lat_v, lon_v, time_v, speed_v)
            && lat_v.is_finite()
            && lon_v.is_finite()
            && time_v.is_finite()
            && speed_v.is_finite()
            && (-90.0..=90.0).contains(&lat_v)
            && (-360.0..=360.0).contains(&lon_v)
            && (0.0..=120.0).contains(&speed_v)
        {
            if let Some(epoch) = lsk.unix_to_tdb(epoch_unix + time_v * unit_seconds) {
                records.push(GeoRec {
                    t: epoch,
                    lat: lat_v,
                    lon: lon_v,
                    alt: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    val: speed_v,
                    comp: COMP_ASCAT_WSPD,
                    station: 0,
                });
                speed_kept += 1;
                if let (Some((_, dir_var)), Some(dir_n)) = (&dir, dir_n.as_ref()) {
                    let dir_raw = decode_num(&dir_var.raw, j * dir_var.dt.size, &dir_var.dt);
                    let dir_v = match dir_raw {
                        Some(v) if dir_n.fill != Some(v) => Some(v * dir_n.scale + dir_n.offset),
                        _ => None,
                    };
                    if let Some(dir_v) = dir_v
                        && dir_v.is_finite()
                        && (0.0..=360.0).contains(&dir_v)
                    {
                        records.push(GeoRec {
                            t: epoch,
                            lat: lat_v,
                            lon: lon_v,
                            alt: 0.0,
                            freq: 0.0,
                            bin_width: 0.0,
                            val: dir_v,
                            comp: COMP_ASCAT_WDIR,
                            station: 0,
                        });
                        dir_kept += 1;
                    }
                }
            }
        }
        j += stride;
    }

    if records.is_empty() {
        return Err(format!(
            "{source}: no measured ASCAT cell left the harvest — the bin stays unwritten (0 honored)"
        ));
    }
    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.comp.cmp(&b.comp))
            .then(a.lat.total_cmp(&b.lat))
            .then(a.lon.total_cmp(&b.lon))
    });

    let out = match arg_value(args, "--out") {
        Some(path) => path,
        None => format!("data/{netloc}/ascat_{label}.bin"),
    };
    if let Some(parent) = std::path::Path::new(&out).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create {} void: {e}", parent.display()))?;
    }
    let bytes = write_bin(MAGIC_ASCAT, &records);
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} void: {e}"))?;
    match parse_bin(MAGIC_ASCAT, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} records ({} speed, {} direction) from {n_cells} cells stride {stride}, {} B, roundtrip parses",
                parsed.len(),
                speed_kept,
                dir_kept,
                bytes.len()
            );
        }
        Some(parsed) => {
            return Err(format!(
                "{out}: {} parsed vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => {
            return Err(format!(
                "{out}: roundtrip parse void — the asset stays unverified"
            ));
        }
    }
    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {netloc} release"
        ));
    }
    Ok(())
}

fn positional_source(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(a.as_str(), "--label" | "--netloc" | "--stride" | "--out") {
            i += 2;
            continue;
        }
        if a.starts_with("--") || a == "-h" {
            i += 1;
            continue;
        }
        return Some(a.clone());
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: ascat_compiler <url|file.nc> --label <product> [--netloc <netloc>] [--stride N] [--out <path>] [--ci-mode] [--inspect]"
        );
        eprintln!("  reads an ASCAT L2 wind granule (netCDF-4/HDF5) through the archivar HDF5 arm");
        eprintln!("  emits data/<netloc>/ascat_<label>.bin (wind speed m/s, wind direction deg)");
        eprintln!("  --inspect prints the granule's variables and attributes and stops");
        eprintln!("  --ci-mode uploads the verified asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("ascat_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_unit_reads_the_cf_epoch() {
        assert_eq!(
            time_unit("seconds since 1970-01-01 00:00:00"),
            Some((1.0, 0.0))
        );
        assert_eq!(
            time_unit("days since 1981-01-01 00:00:00"),
            Some((86400.0, 347155200.0))
        );
        assert_eq!(
            time_unit("seconds since 2000-01-01T00:00:00"),
            Some((1.0, 946684800.0))
        );
        assert_eq!(time_unit("fortnights since 2000-01-01"), None);
        assert_eq!(time_unit("seconds"), None);
    }

    #[test]
    fn host_of_names_the_url_host() {
        assert_eq!(
            host_of("https://manati.star.nesdis.noaa.gov/UHR_ASCAT/x.nc"),
            Some("manati.star.nesdis.noaa.gov".to_string())
        );
        assert_eq!(host_of("/tmp/opencode/x.nc"), None);
    }

    #[test]
    fn ascat_format_resolves_through_the_archivar_read_path() {
        let rec = GeoRec {
            t: 0.0,
            lat: 1.0,
            lon: 2.0,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 7.5,
            comp: COMP_ASCAT_WSPD,
            station: 0,
        };
        let bytes = write_bin(MAGIC_ASCAT, &[rec]);
        let parsed = omegaflow::archivar::extract::geo_series_parse_bin("ascat_wind", &bytes)
            .expect("the ascat_wind format resolves the ASC1 magic");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].val, 7.5);
        assert_eq!(
            omegaflow::archivar::extract::geo_series_component_name("ascat_wind", COMP_ASCAT_WSPD),
            Some("ascat_wind_speed_m_s")
        );
        assert_eq!(
            omegaflow::archivar::extract::geo_series_component_name("ascat_wind", COMP_ASCAT_WDIR),
            Some("ascat_wind_direction_deg")
        );
    }

    #[test]
    fn grid_axes_maps_rows_and_columns() {
        let g = Grid::Axes { n_lat: 3, n_lon: 4 };
        assert_eq!(g.n_cells(), 12);
        assert_eq!(g.lat_index(5), 1);
        assert_eq!(g.lon_index(5), 1);
        assert_eq!(time_index(&g, 1, 5), Some(0));
        assert_eq!(time_index(&g, 12, 5), Some(5));
        assert_eq!(time_index(&g, 4, 5), Some(1));
        assert_eq!(time_index(&g, 3, 5), Some(1));
        assert_eq!(time_index(&g, 7, 5), None);
    }

    #[test]
    fn grid_swath_maps_cells_one_to_one() {
        let g = Grid::Swath { n: 12 };
        assert_eq!(g.n_cells(), 12);
        assert_eq!(g.lat_index(5), 5);
        assert_eq!(g.lon_index(5), 5);
        assert_eq!(time_index(&g, 1, 5), Some(0));
        assert_eq!(time_index(&g, 12, 5), Some(5));
        assert_eq!(time_index(&g, 4, 5), None);
    }

    #[test]
    fn positional_source_skips_flag_values() {
        let args = vec![
            "--label".to_string(),
            "uhr".to_string(),
            "--stride".to_string(),
            "4".to_string(),
            "--ci-mode".to_string(),
            "https://example.org/a.nc".to_string(),
        ];
        assert_eq!(
            positional_source(&args),
            Some("https://example.org/a.nc".to_string())
        );
    }
}
