use omegaflow::archivar::cf_time_unix_seconds;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_HADISST, GeoRec, MAGIC_HADISST, parse_bin, write_bin};
use omegaflow::archivar::netcdf::{NetcdfFile, NetcdfType, NetcdfVar};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "metoffice.gov.uk";
const URL: &str = "https://www.metoffice.gov.uk/hadobs/hadisst/data/HadISST_sst.nc.gz";
const DEFAULT_OUT: &str = "data/metoffice.gov.uk";
const ASSET_NAME: &str = "hadisst_sst.bin";
const FORMAT: &str = "hadisst_sst";
const COMPILER: &str = "tools/harvest/src/bin/hadisst_compiler.rs";
const DEFAULT_STRIDE: usize = 32;
const TTL_SECONDS: u64 = 2_592_000;
const SST_MIN: f64 = -3.0;
const SST_MAX: f64 = 45.0;
const SECS_PER_DAY: f64 = 86400.0;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn attr_text(file: &NetcdfFile, var: &NetcdfVar, name: &str) -> Option<String> {
    var.attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| file.attr_text(a))
}

fn attr_num(file: &NetcdfFile, var: &NetcdfVar, name: &str) -> Option<f64> {
    var.attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| file.attr_num(a))
}

fn scaled_fill(file: &NetcdfFile, name: &str) -> Option<f64> {
    let raw = file.fill_value(name)?;
    let var = file.var(name)?;
    let scale = attr_num(file, var, "scale_factor");
    let offset = attr_num(file, var, "add_offset");
    Some(match (scale, offset) {
        (Some(scale), Some(offset)) => raw * scale + offset,
        (Some(scale), None) => raw * scale,
        _ => raw,
    })
}

fn is_fill(value: f64, fill: Option<f64>) -> bool {
    match fill {
        Some(fill) => (value - fill).abs() <= 1e-9 * fill.abs().max(1.0),
        None => false,
    }
}

fn accept_cell(value: f64, fill: Option<f64>) -> Option<f64> {
    if !value.is_finite() {
        return None;
    }
    if is_fill(value, fill) {
        return None;
    }
    if value > SST_MIN && value < SST_MAX {
        Some(value)
    } else {
        None
    }
}

fn dim_name<'a>(file: &'a NetcdfFile, var: &NetcdfVar, i: usize) -> Result<&'a str, String> {
    let id = *var
        .dim_ids
        .get(i)
        .ok_or_else(|| format!("dim slot {i} absent on '{}'", var.name))?;
    file.dims
        .get(id)
        .map(|d| d.name.as_str())
        .ok_or_else(|| format!("dim id {id} absent in the header"))
}

fn clock_seconds(token: &str) -> Option<f64> {
    let parts: Vec<&str> = token.trim_end_matches('Z').split(':').collect();
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }
    let mut seconds = 0.0;
    for (i, part) in parts.iter().enumerate() {
        let unit = match i {
            0 => 3600.0,
            1 => 60.0,
            _ => 1.0,
        };
        seconds += part.parse::<f64>().ok()? * unit;
    }
    Some(seconds)
}

fn months_since_unix(units: &str, value: f64) -> Option<f64> {
    let (unit, epoch) = units.split_once(" since ")?;
    if !unit.trim().eq_ignore_ascii_case("months") {
        return None;
    }
    if !value.is_finite() || value < 0.0 {
        return None;
    }
    let months = value.floor() as i64;
    if value != months as f64 {
        return None;
    }
    let mut fields = epoch.split_whitespace();
    let date = fields.next()?;
    let mut parts = date.split('-');
    let year = parts.next()?.parse::<i64>().ok()?;
    let month = parts.next()?.parse::<i64>().ok()?;
    let day = parts.next()?.parse::<i64>().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let clock = match fields.next() {
        Some(token) => clock_seconds(token)?,
        None => 0.0,
    };
    let total = year * 12 + (month - 1) + months;
    let new_year = total.div_euclid(12);
    let new_month = total.rem_euclid(12) + 1;
    let days = days_from_civil(new_year, new_month, day)?;
    Some(days as f64 * SECS_PER_DAY + clock)
}

fn time_unix(units: &str, value: f64) -> Option<f64> {
    if !value.is_finite() {
        return None;
    }
    if let Some(unix) = cf_time_unix_seconds(units, value) {
        return Some(unix);
    }
    months_since_unix(units, value)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_dir = match arg_value(args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let url = match arg_value(args, "--url") {
        Some(v) => v,
        None => URL.to_string(),
    };
    let stride = match arg_value(args, "--stride") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--stride '{v}' is not a step"))?,
        None => DEFAULT_STRIDE,
    };
    if stride == 0 {
        return Err("--stride carries no positive sampling step".to_string());
    }

    let gz = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let bytes = gunzip(&gz).ok_or_else(|| format!("{url}: gzip stream stays unreadable"))?;
    let file = NetcdfFile::parse(&bytes).map_err(|e| format!("{url}: netCDF parse {e:?}"))?;

    let sst = file
        .var("sst")
        .ok_or_else(|| format!("{url}: no variable 'sst' in the header"))?;
    if sst.nc_type != NetcdfType::Float {
        return Err(format!(
            "{url}: sst carries type {} not float — the values stay unmeasured",
            sst.nc_type.name()
        ));
    }
    let shape: Vec<usize> = file
        .var_shape(sst)
        .map_err(|e| format!("{url}: sst shape {e:?}"))?
        .into_iter()
        .map(|n| n as usize)
        .collect();
    let mut dim_names = Vec::with_capacity(sst.dim_ids.len());
    for i in 0..sst.dim_ids.len() {
        dim_names.push(dim_name(&file, sst, i)?.to_string());
    }
    let expected = ["time", "lat|latitude", "lon|longitude"];
    let matched = dim_names.len() == 3
        && dim_names[0] == "time"
        && matches!(dim_names[1].as_str(), "lat" | "latitude")
        && matches!(dim_names[2].as_str(), "lon" | "longitude");
    if !matched {
        return Err(format!(
            "{url}: sst dims {dim_names:?} are not {expected:?}"
        ));
    }
    let (nt, nlat, nlon) = (shape[0], shape[1], shape[2]);

    let time_var = file
        .var("time")
        .ok_or_else(|| format!("{url}: no time coordinate in the header"))?;
    let time_units = attr_text(&file, time_var, "units")
        .ok_or_else(|| format!("{url}: 'time' carries no units attribute"))?;
    let time_values = file
        .values_numeric(&bytes, "time")
        .ok_or_else(|| format!("{url}: time carries no numeric values"))?;
    let lat_values = file
        .values_numeric(&bytes, "lat")
        .ok_or_else(|| format!("{url}: lat carries no numeric values"))?;
    let lon_values = file
        .values_numeric(&bytes, "lon")
        .ok_or_else(|| format!("{url}: lon carries no numeric values"))?;
    if time_values.len() != nt || lat_values.len() != nlat || lon_values.len() != nlon {
        return Err(format!(
            "{url}: a coordinate carries {} / {} / {} values against the shape {nt}x{nlat}x{nlon}",
            time_values.len(),
            lat_values.len(),
            lon_values.len()
        ));
    }

    let scale = attr_num(&file, sst, "scale_factor");
    let offset = attr_num(&file, sst, "add_offset");
    let fill = scaled_fill(&file, "sst");

    let mut records = Vec::new();
    let mut sampled = 0usize;
    for ti in 0..nt {
        let Some(unix) = time_unix(&time_units, time_values[ti]) else {
            continue;
        };
        let raw = file
            .read_var(&bytes, "sst", &[ti, 0, 0], &[1, nlat, nlon])
            .map_err(|e| format!("{url}: sst slab {ti} {e:?}"))?;
        if raw.len() != nlat * nlon * 4 {
            return Err(format!(
                "{url}: sst slab {ti} carries {} B against {nlat}x{nlon} float cells",
                raw.len()
            ));
        }
        for li in (0..nlat).step_by(stride) {
            let Some(&lat) = lat_values.get(li) else {
                continue;
            };
            if !lat.is_finite() {
                continue;
            }
            for lj in (0..nlon).step_by(stride) {
                let Some(&lon) = lon_values.get(lj) else {
                    continue;
                };
                if !lon.is_finite() {
                    continue;
                }
                let off = (li * nlon + lj) * 4;
                let Some(bits) = raw
                    .get(off..off + 4)
                    .and_then(|b| <[u8; 4]>::try_from(b).ok())
                    .map(u32::from_be_bytes)
                else {
                    continue;
                };
                sampled += 1;
                let mut v = f32::from_bits(bits) as f64;
                if let Some(scale) = scale {
                    v *= scale;
                }
                if let Some(offset) = offset {
                    v += offset;
                }
                if let Some(val) = accept_cell(v, fill) {
                    records.push(GeoRec {
                        t: unix,
                        lat,
                        lon,
                        alt: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        val,
                        comp: COMP_HADISST,
                        station: 0,
                    });
                }
            }
        }
    }

    if records.is_empty() {
        return Err(format!(
            "{url}: no measured SST cell left the harvest ({sampled} sampled) — the bin stays unwritten (0 honored)"
        ));
    }

    let bin = write_bin(MAGIC_HADISST, &records);
    std::fs::create_dir_all(&out_dir).map_err(|e| format!("create {out_dir} returned {e}"))?;
    let path = format!("{out_dir}/{ASSET_NAME}");
    std::fs::write(&path, &bin).map_err(|e| format!("write {path} returned {e}"))?;

    match parse_bin(MAGIC_HADISST, &bin) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{path}: {} SST cells ({} sampled, {} time steps, {}x{} grid stride {stride}), roundtrip parses",
                parsed.len(),
                sampled,
                nt,
                nlat,
                nlon
            );
        }
        Some(parsed) => {
            return Err(format!(
                "{path}: {} parsed vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => {
            return Err(format!(
                "{path}: roundtrip parse void — the asset stays unverified"
            ));
        }
    }

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{ASSET_NAME}");
    println!("format {FORMAT}");
    println!("origin {url}");
    println!("compiler {COMPILER}");
    println!("on earth 0 0 0");
    println!("ttl {TTL_SECONDS}");
    println!("field {FORMAT} {FORMAT} exponential-decay thermal C {TTL_SECONDS} 0.0 0.0");
    println!("sha256 {}", sha256_hex(&bin));

    if ci_mode && !upload_release(NETLOC, &path) {
        return Err(format!("{path}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("hadisst_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_and_range_gate_rejects_absent_and_fill() {
        assert_eq!(accept_cell(20.0, Some(-1e30)), Some(20.0));
        assert_eq!(accept_cell(0.0, Some(-1e30)), Some(0.0));
        assert_eq!(accept_cell(0.0, Some(0.0)), None);
        assert_eq!(accept_cell(-1e30, Some(-1e30)), None);
        assert_eq!(accept_cell(f64::NAN, Some(-1e30)), None);
        assert_eq!(accept_cell(-3.0, Some(-1e30)), None);
        assert_eq!(accept_cell(45.0, Some(-1e30)), None);
        assert_eq!(accept_cell(-1.8, Some(-1e30)), Some(-1.8));
        assert_eq!(accept_cell(f64::INFINITY, None), None);
    }

    #[test]
    fn months_since_epoch_resolves_the_hadisst_clock() {
        let base = days_from_civil(1870, 1, 1).unwrap() as f64 * SECS_PER_DAY;
        assert_eq!(
            time_unix("months since 1870-01-01 00:00:00", 0.0),
            Some(base)
        );
        let feb = days_from_civil(1870, 2, 1).unwrap() as f64 * SECS_PER_DAY;
        assert_eq!(
            time_unix("months since 1870-01-01 00:00:00", 1.0),
            Some(feb)
        );
        let year_one = days_from_civil(1871, 1, 1).unwrap() as f64 * SECS_PER_DAY;
        assert_eq!(
            time_unix("months since 1870-01-01 00:00:00", 12.0),
            Some(year_one)
        );
        assert_eq!(time_unix("furlongs since 1870-01-01", 1.0), None);
        assert_eq!(time_unix("months since 1870-01-01", -1.0), None);
    }

    #[test]
    fn hadisst_bin_roundtrip_carries_the_sst_record() {
        let records = vec![
            GeoRec {
                t: -3_155_750_400.0,
                lat: -67.5,
                lon: 62.5,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: -1.8,
                comp: COMP_HADISST,
                station: 0,
            },
            GeoRec {
                t: 1_700_000_000.0,
                lat: 0.0,
                lon: 180.0,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: 28.4,
                comp: COMP_HADISST,
                station: 0,
            },
        ];
        let bytes = write_bin(MAGIC_HADISST, &records);
        let parsed = parse_bin(MAGIC_HADISST, &bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), records.len());
        assert_eq!(parsed[0].val, -1.8);
        assert_eq!(parsed[1].val, 28.4);
        assert_eq!(parsed[0].comp, COMP_HADISST);
    }
}
