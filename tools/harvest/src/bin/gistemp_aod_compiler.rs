use omegaflow::archivar::cf_time_unix_seconds;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::netcdf::{NetcdfFile, NetcdfVar};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "data.giss.nasa.gov";
const URL: &str = "https://data.giss.nasa.gov/modelforce/strataer/data/tau_reff_Sato-Lacis.nc";
const DEFAULT_OUT: &str = "data/data.giss.nasa.gov";
const ASSET_NAME: &str = "gistemp_aod550_global_mean.txt";
const DEFAULT_LEVEL: usize = 0;
const TTL_SECONDS: u64 = 31_536_000;

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

fn find_coord<'a>(file: &'a NetcdfFile, dim: &str, len: usize) -> Option<&'a NetcdfVar> {
    for var in &file.vars {
        if var.dim_ids.len() != 1 {
            continue;
        }
        let Some(d) = file.dims.get(var.dim_ids[0]) else {
            continue;
        };
        if d.name == dim && d.len as usize == len {
            return Some(var);
        }
    }
    None
}

struct SeriesPoint {
    t: f64,
    v: f64,
}

fn reduce_series(
    shape: &[usize],
    tau: &[f64],
    time_axis: usize,
    lat_axis: usize,
    level_axis: Option<usize>,
    level_index: usize,
    lat: &[f64],
    time: &[f64],
    time_units: &str,
    fill: Option<f64>,
) -> Result<Vec<SeriesPoint>, String> {
    let rank = shape.len();
    let cells: usize = shape.iter().product();
    if tau.len() != cells {
        return Err(format!(
            "tau carries {} values for shape {shape:?} ({cells} cells)",
            tau.len()
        ));
    }
    if time.len() != shape[time_axis] {
        return Err(format!(
            "time axis carries {} values for length {}",
            time.len(),
            shape[time_axis]
        ));
    }
    if lat.len() != shape[lat_axis] {
        return Err(format!(
            "lat axis carries {} values for length {}",
            lat.len(),
            shape[lat_axis]
        ));
    }
    if let Some(axis) = level_axis {
        if level_index >= shape[axis] {
            return Err(format!(
                "level index {level_index} outside the level axis length {}",
                shape[axis]
            ));
        }
    }
    let mut strides = vec![1usize; rank];
    for i in (0..rank.saturating_sub(1)).rev() {
        strides[i] = strides[i + 1] * shape[i + 1];
    }
    let mut out = Vec::new();
    for (ti, &raw_t) in time.iter().enumerate() {
        let Some(unix) = cf_time_unix_seconds(time_units, raw_t) else {
            continue;
        };
        let mut sum = 0.0;
        let mut weight_sum = 0.0;
        for (li, &raw_lat) in lat.iter().enumerate() {
            if !raw_lat.is_finite() {
                continue;
            }
            let mut index = vec![0usize; rank];
            index[time_axis] = ti;
            index[lat_axis] = li;
            if let Some(axis) = level_axis {
                index[axis] = level_index;
            }
            let mut flat = 0usize;
            for (k, &d) in index.iter().enumerate() {
                flat += d * strides[k];
            }
            let Some(&value) = tau.get(flat) else {
                continue;
            };
            if !value.is_finite() || is_fill(value, fill) {
                continue;
            }
            let weight = raw_lat.to_radians().cos().abs();
            sum += value * weight;
            weight_sum += weight;
        }
        if weight_sum > 0.0 {
            out.push(SeriesPoint {
                t: unix,
                v: sum / weight_sum,
            });
        }
    }
    Ok(out)
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
    let level_index = match arg_value(args, "--level") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--level '{v}' is not an index"))?,
        None => DEFAULT_LEVEL,
    };
    let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let file = NetcdfFile::parse(&bytes).map_err(|e| format!("{url}: netCDF parse {e:?}"))?;
    let tau = file
        .var("tau")
        .ok_or_else(|| format!("{url}: no variable 'tau' in the header"))?;
    let shape: Vec<usize> = file
        .var_shape(tau)
        .map_err(|e| format!("{url}: tau shape {e:?}"))?
        .into_iter()
        .map(|n| n as usize)
        .collect();
    let mut dim_names = Vec::with_capacity(tau.dim_ids.len());
    for i in 0..tau.dim_ids.len() {
        dim_names.push(dim_name(&file, tau, i)?);
    }
    let axis_of = |want: &[&str]| dim_names.iter().position(|n| want.contains(n));
    let time_axis = axis_of(&["time", "month", "t", "date"])
        .ok_or_else(|| format!("{url}: no time dimension in {dim_names:?}"))?;
    let lat_axis = axis_of(&["lat", "latitude", "y"])
        .ok_or_else(|| format!("{url}: no latitude dimension in {dim_names:?}"))?;
    let level_axis = axis_of(&["level", "lev", "depth", "height"]);
    let time_len = shape[time_axis];
    let lat_len = shape[lat_axis];

    let time_var = find_coord(&file, dim_names[time_axis], time_len).ok_or_else(|| {
        format!(
            "{url}: no coordinate variable for '{}'",
            dim_names[time_axis]
        )
    })?;
    let time_values = file
        .values_numeric(&bytes, &time_var.name)
        .ok_or_else(|| format!("{url}: '{}' carries no numeric values", time_var.name))?;
    let time_units = attr_text(&file, time_var, "units")
        .ok_or_else(|| format!("{url}: '{}' carries no units attribute", time_var.name))?;

    let lat_var = find_coord(&file, dim_names[lat_axis], lat_len).ok_or_else(|| {
        format!(
            "{url}: no coordinate variable for '{}'",
            dim_names[lat_axis]
        )
    })?;
    let lat_values = file
        .values_numeric(&bytes, &lat_var.name)
        .ok_or_else(|| format!("{url}: '{}' carries no numeric values", lat_var.name))?;

    let tau_values = file
        .values_numeric(&bytes, "tau")
        .ok_or_else(|| format!("{url}: tau carries no numeric values"))?;
    let fill = scaled_fill(&file, "tau");
    let series = reduce_series(
        &shape,
        &tau_values,
        time_axis,
        lat_axis,
        level_axis,
        level_index,
        &lat_values,
        &time_values,
        &time_units,
        fill,
    )?;
    if series.is_empty() {
        return Err(format!(
            "{url}: no measured AOD month left the reduction ({} B) — the asset stays unwritten (0 honored)",
            bytes.len()
        ));
    }

    let shape_text: Vec<String> = shape.iter().map(|n| n.to_string()).collect();
    let level_text = match level_axis {
        Some(axis) => format!("{level_index} of {}", shape[axis]),
        None => format!("{level_index} (tau carries no level axis)"),
    };
    let mut text = format!(
        "# gistemp volcanic stratospheric AOD 550nm series | var tau | shape {} | global mean = cos(lat)-weighted over the full latitude domain | level {level_text} | time {time_units} | value AOD550 dimensionless | origin {url}\n",
        shape_text.join("x")
    );
    for point in &series {
        text.push_str(&format!("{} {}\n", point.t, point.v));
    }

    std::fs::create_dir_all(&out_dir).map_err(|e| format!("create {out_dir} returned {e}"))?;
    let path = format!("{out_dir}/{ASSET_NAME}");
    std::fs::write(&path, text.as_bytes()).map_err(|e| format!("write {path} returned {e}"))?;

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{ASSET_NAME}");
    println!("format gistemp_aod550_axis_value_text");
    println!("origin {url}");
    println!("compiler tools/harvest/src/bin/gistemp_aod_compiler.rs");
    println!("on earth 0 0 0");
    println!("ttl {TTL_SECONDS}");
    println!(
        "field gistemp_aod550 gistemp_aod550_aod550 exponential-decay em 1 {TTL_SECONDS} 0.0 0.0"
    );
    println!("sha256 {}", sha256_hex(text.as_bytes()));

    eprintln!(
        "gistemp_aod_compiler: {} AOD months written to {path} ({url})",
        series.len()
    );
    if ci_mode && !upload_release(NETLOC, &path) {
        return Err(format!("{path}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn selftest() {
    let shape = [3usize, 2];
    let tau = [0.1, 0.2, 0.9999, 0.2, 0.3, 0.3];
    let lat = [-45.0, 45.0];
    let time = [0.0, 1.0, 2.0];
    let series = match reduce_series(
        &shape,
        &tau,
        0,
        1,
        None,
        0,
        &lat,
        &time,
        "days since 1850-01-01 00:00:00",
        Some(0.9999),
    ) {
        Ok(series) => series,
        Err(e) => {
            eprintln!("selftest: reduce_series void: {e}");
            std::process::exit(1);
        }
    };
    if series.len() != 3 {
        eprintln!(
            "selftest: {} months left the reduction, not the measured 3",
            series.len()
        );
        std::process::exit(1);
    }
    if series.iter().any(|p| !p.v.is_finite() || !p.t.is_finite()) {
        eprintln!("selftest: a series point is not finite");
        std::process::exit(1);
    }
    if (series[1].v - 0.2).abs() > 1e-12 {
        eprintln!("selftest: the fill cell entered the mean ({})", series[1].v);
        std::process::exit(1);
    }
    let bad = reduce_series(
        &shape,
        &tau[0..5],
        0,
        1,
        None,
        0,
        &lat,
        &time,
        "days since 1850-01-01 00:00:00",
        None,
    );
    if bad.is_ok() {
        eprintln!("selftest: a short tau slab read as a full grid");
        std::process::exit(1);
    }
    eprintln!("gistemp_aod_compiler: selftest passes (netCDF tau grid → cos-lat global mean)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if let Err(msg) = run(&args) {
        eprintln!("gistemp_aod_compiler: {msg}");
        std::process::exit(2);
    }
}
