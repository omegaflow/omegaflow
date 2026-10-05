use omegaflow::archivar::cf_time_unix_seconds;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::opendap::{AsciiVar, parse_ascii};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "psl.noaa.gov";
const DEFAULT_YEAR: u32 = 2024;
const DEFAULT_OUT: &str = "data/psl.noaa.gov";
const DEFAULT_LEVEL: usize = 0;
const DEFAULT_LAT: usize = 225;
const DEFAULT_LON: usize = 180;
const TTL_SECONDS: u64 = 604_800;
const GODAS_TIME_UNITS: &str = "days since 1800-01-01 00:00:0.0";
const GODAS_FILL_MAGNITUDE: f64 = 1.0e30;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn var_leaf(name: &str) -> &str {
    match name.rfind('.') {
        Some(i) => &name[i + 1..],
        None => name,
    }
}

fn find_var<'a>(vars: &'a [AsciiVar], want: &str) -> Option<&'a AsciiVar> {
    vars.iter().find(|v| var_leaf(&v.name) == want)
}

fn base_endpoint(year: u32) -> String {
    format!("https://psl.noaa.gov/thredds/dodsC/Datasets/godas/pottmp.{year}.nc.ascii")
}

fn asset_for(year: u32) -> String {
    format!("godas_pottmp_{year}_cell.txt")
}

struct Cell {
    time: Vec<f64>,
    values: Vec<f64>,
    level: f64,
    lat: f64,
    lon: f64,
}

fn map_value(vars: &[AsciiVar], name: &str) -> Result<f64, String> {
    find_var(vars, name)
        .and_then(|v| v.values.first().copied())
        .ok_or_else(|| format!("no '{name}' map in the DAP2-ASCII body"))
}

fn cell_series(text: &str) -> Result<Cell, String> {
    let vars = parse_ascii(text).map_err(|e| format!("DAP2-ASCII parse {e:?}"))?;
    let pottmp = find_var(&vars, "pottmp")
        .ok_or_else(|| "no variable 'pottmp' in the DAP2-ASCII body".to_string())?;
    let shape = pottmp.shape.as_slice();
    if shape.len() != 4 {
        return Err(format!(
            "pottmp shape {shape:?} is not [time][level][lat][lon]"
        ));
    }
    let steps = shape[0];
    let stride = shape[1] * shape[2] * shape[3];
    if steps == 0 || stride == 0 {
        return Err(format!("pottmp shape {shape:?} carries no time step"));
    }
    let mut values = Vec::with_capacity(steps);
    for i in 0..steps {
        values.push(*pottmp.values.get(i * stride).ok_or_else(|| {
            format!(
                "pottmp carries {} values for {steps} time steps and stride {stride}",
                pottmp.values.len()
            )
        })?);
    }
    let time = find_var(&vars, "time")
        .ok_or_else(|| "no 'time' map in the DAP2-ASCII body".to_string())?;
    Ok(Cell {
        time: time.values.clone(),
        values,
        level: map_value(&vars, "level")?,
        lat: map_value(&vars, "lat")?,
        lon: map_value(&vars, "lon")?,
    })
}

fn series(cell: &Cell) -> Vec<(f64, f64)> {
    let steps = cell.time.len().min(cell.values.len());
    let mut out = Vec::new();
    for i in 0..steps {
        let value = cell.values[i];
        if !value.is_finite() || value.abs() >= GODAS_FILL_MAGNITUDE {
            continue;
        }
        let Some(unix) = cf_time_unix_seconds(GODAS_TIME_UNITS, cell.time[i]) else {
            continue;
        };
        out.push((unix, value));
    }
    out
}

fn time_steps(base: &str) -> Result<usize, String> {
    let url = format!("{base}?pottmp.time");
    let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let text = String::from_utf8_lossy(&bytes);
    let vars = parse_ascii(&text).map_err(|e| format!("{url}: DAP2-ASCII parse {e:?}"))?;
    let time = find_var(&vars, "time")
        .ok_or_else(|| format!("{url}: no 'time' map in the header response"))?;
    time.shape
        .first()
        .copied()
        .filter(|n| *n > 0)
        .ok_or_else(|| format!("{url}: the time axis carries no step"))
}

fn constrained(base: &str, steps: usize, level: usize, lat: usize, lon: usize) -> String {
    let last = steps - 1;
    format!("{base}?pottmp[0:1:{last}][{level}:1:{level}][{lat}:1:{lat}][{lon}:1:{lon}]")
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_dir = match arg_value(args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let year = match arg_value(args, "--year") {
        Some(v) => v
            .parse::<u32>()
            .map_err(|_| format!("--year '{v}' is not a year"))?,
        None => DEFAULT_YEAR,
    };
    let level = match arg_value(args, "--level") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--level '{v}' is not an index"))?,
        None => DEFAULT_LEVEL,
    };
    let lat = match arg_value(args, "--lat") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--lat '{v}' is not an index"))?,
        None => DEFAULT_LAT,
    };
    let lon = match arg_value(args, "--lon") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--lon '{v}' is not an index"))?,
        None => DEFAULT_LON,
    };
    let given = arg_value(args, "--url");
    let (url, bytes) = match &given {
        Some(u) if u.contains('?') => {
            let bytes = fetch_raw_bytes(u).ok_or_else(|| format!("{u}: fetch void"))?;
            (u.clone(), bytes)
        }
        Some(base) => {
            let steps = time_steps(base)?;
            let url = constrained(base, steps, level, lat, lon);
            let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
            (url, bytes)
        }
        None => {
            let base = base_endpoint(year);
            let steps = time_steps(&base)?;
            let url = constrained(&base, steps, level, lat, lon);
            let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
            (url, bytes)
        }
    };
    let text = String::from_utf8_lossy(&bytes);
    let cell = cell_series(&text)?;
    let pairs = series(&cell);
    if pairs.is_empty() {
        return Err(format!(
            "{url}: no measured potential-temperature step left the harvest ({} B) — the asset stays unwritten (0 honored)",
            bytes.len()
        ));
    }

    let mut out_text = format!(
        "# godas pottmp cell series | year {year} | level {level} ({}) | lat {lat} ({}) | lon {lon} ({}) | time {GODAS_TIME_UNITS} | value potential temperature K | origin {url}\n",
        cell.level, cell.lat, cell.lon
    );
    for (t, v) in &pairs {
        out_text.push_str(&format!("{t} {v}\n"));
    }

    std::fs::create_dir_all(&out_dir).map_err(|e| format!("create {out_dir} returned {e}"))?;
    let asset = asset_for(year);
    let path = format!("{out_dir}/{asset}");
    std::fs::write(&path, out_text.as_bytes()).map_err(|e| format!("write {path} returned {e}"))?;

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("format godas_pottmp_axis_value_text");
    println!("origin {}", base_endpoint(year));
    println!("compiler tools/harvest/src/bin/godas_pottmp_compiler.rs");
    println!("on earth {} {} 0", cell.lat, cell.lon);
    println!("ttl {TTL_SECONDS}");
    println!("field godas_pottmp godas_pottmp_k exponential-decay thermal K {TTL_SECONDS} 0.0 0.0");
    println!("sha256 {}", sha256_hex(out_text.as_bytes()));

    eprintln!(
        "godas_pottmp_compiler: {} potential-temperature steps written to {path} ({url})",
        pairs.len()
    );
    if ci_mode && !upload_release(NETLOC, &path) {
        return Err(format!("{path}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn selftest() {
    let ascii = "Dataset {\n    Grid {\n     ARRAY:\n        Float32 pottmp[time = 3][level = 1][lat = 1][lon = 1];\n     MAPS:\n        Float64 time[time = 3];\n        Float32 level[level = 1];\n        Float32 lat[lat = 1];\n        Float32 lon[lon = 1];\n    } pottmp;\n} Datasets/godas/pottmp.2024.nc;\n---------------------------------------------\npottmp.pottmp[3][1][1][1]\n[0][0][0], 5.0\n[1][0][0], -9.96921E36\n[2][0][0], 6.0\n\npottmp.time[3]\n81814.0, 81815.0, 81816.0\n\npottmp.level[1]\n5.0\n\npottmp.lat[1]\n-74.5\n\npottmp.lon[1]\n0.5\n\n";
    let cell = match cell_series(ascii) {
        Ok(cell) => cell,
        Err(e) => {
            eprintln!("selftest: cell_series void: {e}");
            std::process::exit(1);
        }
    };
    if cell.values != vec![5.0, -9.96921e36, 6.0] {
        eprintln!("selftest: the parsed cell is not the measured grid");
        std::process::exit(1);
    }
    let pairs = series(&cell);
    if pairs.len() != 2 {
        eprintln!(
            "selftest: {} steps left the harvest, not the measured 2 finite steps",
            pairs.len()
        );
        std::process::exit(1);
    }
    if pairs.iter().any(|(t, v)| !t.is_finite() || !v.is_finite()) {
        eprintln!("selftest: a series point is not finite");
        std::process::exit(1);
    }
    if (pairs[1].1 - 6.0).abs() > 1e-12 {
        eprintln!("selftest: the fill step is not the one removed");
        std::process::exit(1);
    }
    if !base_endpoint(2024).contains("pottmp.2024.nc.ascii") {
        eprintln!("selftest: the year does not reach the endpoint");
        std::process::exit(1);
    }
    eprintln!("godas_pottmp_compiler: selftest passes (DAP2-ASCII cell → time/value series)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if let Err(msg) = run(&args) {
        eprintln!("godas_pottmp_compiler: {msg}");
        std::process::exit(2);
    }
}
