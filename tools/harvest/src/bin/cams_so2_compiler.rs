use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::hdf5::Hdf5File;
use omegaflow::archivar::netcdf::{Nc4Group, NetcdfFile, nc4_group};
use omegaflow::archivar::units::cf_time_unix_seconds;
use std::process::Command;

trait AbsentExt<T> {
    fn or_absent(self, marker: &str) -> String;
}

impl<T: ToString> AbsentExt<T> for Option<T> {
    fn or_absent(self, marker: &str) -> String {
        match self {
            Some(v) => v.to_string(),
            None => marker.to_string(),
        }
    }
}

pub const COMP_SO2: u32 = 1;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_SO2 => Some("cams_so2_emission_kg_m2_s"),
        _ => None,
    }
}

const DEFAULT_URL: &str = "https://github.com/omegaflow/sources/releases/download/ads.atmosphere.copernicus.eu/cams_so2_emission.nc";
const ORIGIN: &str = "https://ads.atmosphere.copernicus.eu/api/";

fn is_so2(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n == "so2" || n.contains("so2") || n.contains("sulphur_dioxide") || n.contains("sulfur_dioxide")
}

fn is_time(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n == "time" || n == "t" || n == "valid_time" || n == "forecast_time"
}

fn mean_finite(values: &[f64], fill: Option<f64>) -> Option<f64> {
    let mut sum = 0.0;
    let mut n = 0usize;
    for &v in values {
        if !v.is_finite() || fill.is_some_and(|f| v == f) {
            continue;
        }
        sum += v;
        n += 1;
    }
    (n > 0 && sum.is_finite()).then_some(sum / n as f64)
}

fn classic_attr_text(nc: &NetcdfFile, var: &str, key: &str) -> Option<String> {
    nc.var(var)?
        .attrs
        .iter()
        .find(|a| a.name == key)
        .and_then(|a| nc.attr_text(a))
}

fn classic_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let nc = NetcdfFile::parse(bytes).ok()?;
    let (var_name, shape) = nc
        .vars
        .iter()
        .find(|v| is_so2(&v.name))
        .and_then(|v| Some((v.name.clone(), nc.var_shape(v).ok()?)))?;
    if shape.is_empty() {
        return None;
    }
    let values = nc.values_numeric(bytes, &var_name)?;
    let fill = nc.fill_value(&var_name);
    let ntime = *shape.first()? as usize;
    if ntime == 0 || values.len() % ntime != 0 {
        return None;
    }
    let per_time = values.len() / ntime;

    let (times, scale, epoch) = nc
        .vars
        .iter()
        .find(|v| is_time(&v.name))
        .and_then(|v| {
            let units = classic_attr_text(&nc, &v.name, "units")?;
            let values = nc.values_numeric(bytes, &v.name)?;
            Some((values, units))
        })
        .and_then(|(values, units)| {
            let raw = values.first().copied()?;
            let t0 = cf_time_unix_seconds(&units, raw)?;
            let scale = cf_time_unix_seconds(&units, raw + 1.0)? - t0;
            Some((values, scale, t0 - raw * scale))
        })?;

    let mut out = Vec::with_capacity(ntime);
    for ti in 0..ntime {
        let chunk = values.get(ti * per_time..(ti + 1) * per_time)?;
        let value = mean_finite(chunk, fill)?;
        let raw = times.get(ti).copied().unwrap_or(ti as f64);
        let t = epoch + raw * scale;
        if t.is_finite() {
            out.push((t, value, COMP_SO2));
        }
    }
    (!out.is_empty()).then_some(out)
}

fn join(parent: &str, name: &str) -> String {
    if parent.is_empty() {
        name.to_string()
    } else {
        format!("{parent}/{name}")
    }
}

fn hdf5_find(
    file: &mut Hdf5File,
    group: &str,
    pred: &dyn Fn(&str) -> bool,
) -> Option<(String, Vec<u64>)> {
    let g: Nc4Group = nc4_group(file, group).ok()?;
    for v in &g.variables {
        if pred(&v.name) {
            return Some((join(group, &v.name), v.dims.clone()));
        }
    }
    for child in &g.groups {
        if let Some(found) = hdf5_find(file, &join(group, child), pred) {
            return Some(found);
        }
    }
    None
}

fn hdf5_attr_text(file: &Hdf5File, path: &str, key: &str) -> Option<String> {
    let a = file.attribute(path, key)?;
    Some(
        String::from_utf8_lossy(&a.data)
            .trim_end_matches('\0')
            .trim()
            .to_string(),
    )
}

fn hdf5_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    let mut file = Hdf5File::parse(bytes).ok()?;
    let (var_path, shape) = hdf5_find(&mut file, "", &is_so2)?;
    let (time_path, _) = hdf5_find(&mut file, "", &is_time)?;
    let units = hdf5_attr_text(&file, &time_path, "units")?;
    let times = file.read_f64_dataset(&time_path).ok()?;
    let values = file.read_f64_dataset(&var_path).ok()?;
    let fill = file
        .attr_f64(&var_path, "_FillValue")
        .or_else(|| file.attr_f64(&var_path, "missing_value"));
    let ntime = *shape.first()? as usize;
    if ntime == 0 || values.len() % ntime != 0 {
        return None;
    }
    let per_time = values.len() / ntime;
    let raw0 = times.first().copied()?;
    let t0 = cf_time_unix_seconds(&units, raw0)?;
    let scale = cf_time_unix_seconds(&units, raw0 + 1.0)? - t0;
    let epoch = t0 - raw0 * scale;

    let mut out = Vec::with_capacity(ntime);
    for ti in 0..ntime {
        let chunk = values.get(ti * per_time..(ti + 1) * per_time)?;
        let value = mean_finite(chunk, fill)?;
        let t = epoch + times.get(ti).copied().unwrap_or(ti as f64) * scale;
        if t.is_finite() {
            out.push((t, value, COMP_SO2));
        }
    }
    (!out.is_empty()).then_some(out)
}

pub fn parse_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.starts_with(b"CDF") {
        classic_series(bytes)
    } else if bytes.starts_with(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]) {
        hdf5_series(bytes)
    } else {
        None
    }
}

fn u32b(x: u32) -> Vec<u8> {
    x.to_be_bytes().to_vec()
}

fn cdf_name(s: &str) -> Vec<u8> {
    let mut b = u32b(s.len() as u32);
    b.extend_from_slice(s.as_bytes());
    while !b.len().is_multiple_of(4) {
        b.push(0);
    }
    b
}

fn cdf_absent(b: &mut Vec<u8>) {
    b.extend(u32b(0));
    b.extend(u32b(0));
}

fn cdf_attr_text(k: &str, value: &str) -> Vec<u8> {
    let mut b = cdf_name(k);
    b.extend(u32b(2));
    b.extend(u32b(value.len() as u32));
    b.extend_from_slice(value.as_bytes());
    while !b.len().is_multiple_of(4) {
        b.push(0);
    }
    b
}

fn classic_fixture() -> Vec<u8> {
    let mut b = Vec::new();
    b.extend([0x43, 0x44, 0x46, 0x01]);
    b.extend(u32b(0));
    b.extend(u32b(0x0A));
    b.extend(u32b(3));
    b.extend(cdf_name("time"));
    b.extend(u32b(2));
    b.extend(cdf_name("lat"));
    b.extend(u32b(2));
    b.extend(cdf_name("lon"));
    b.extend(u32b(2));
    cdf_absent(&mut b);
    b.extend(u32b(0x0B));
    b.extend(u32b(2));
    b.extend(cdf_name("time"));
    b.extend(u32b(1));
    b.extend(u32b(0));
    b.extend(u32b(0x0C));
    b.extend(u32b(1));
    b.extend(cdf_attr_text("units", "hours since 2020-01-01 00:00:00"));
    b.extend(u32b(6));
    b.extend(u32b(16));
    let time_begin = b.len();
    b.extend(u32b(0));

    b.extend(cdf_name("so2_emission"));
    b.extend(u32b(3));
    b.extend(u32b(0));
    b.extend(u32b(1));
    b.extend(u32b(2));
    b.extend(u32b(0x0C));
    b.extend(u32b(1));
    b.extend(cdf_attr_text("units", "kg m-2 s-1"));
    b.extend(u32b(5));
    b.extend(u32b(32));
    let so2_begin = b.len();
    b.extend(u32b(0));

    let time_off = b.len() as u32;
    b[time_begin..time_begin + 4].copy_from_slice(&time_off.to_be_bytes());
    for t in [0.0f64, 6.0] {
        b.extend(t.to_bits().to_be_bytes());
    }
    let so2_off = b.len() as u32;
    b[so2_begin..so2_begin + 4].copy_from_slice(&so2_off.to_be_bytes());
    for v in [1.0f32, 2.0, 3.0, 4.0, 10.0, 20.0, 30.0, 40.0] {
        b.extend(v.to_bits().to_be_bytes());
    }
    b
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load(args: &[String]) -> Option<(String, Vec<u8>)> {
    if let Some(path) = arg_value(args, "--file") {
        std::fs::read(&path).ok().map(|b| (path, b))
    } else {
        let url = arg_value(args, "--url").or_absent(DEFAULT_URL);
        fetch_raw_bytes(&url).map(|b| (url, b))
    }
}

fn fetch_head(url: &str, bytes: usize) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("60")
        .arg("-r")
        .arg(format!("0-{}", bytes.saturating_sub(1)))
        .arg(url)
        .output()
        .ok()?;
    out.status.success().then_some(out.stdout)
}

fn print_rows(src: &str, rows: &[(f64, f64, u32)]) {
    eprintln!("{src}: {} time step(s)", rows.len());
    for (t, v, comp) in rows.iter().take(8) {
        eprintln!(
            "  {} t={t:.0} value={v:e} {}",
            component_name(*comp).unwrap_or("unknown"),
            "kg/m2/s"
        );
    }
}

fn selftest() {
    let fixture = classic_fixture();
    match parse_series(&fixture) {
        Some(rows) if rows.len() == 2 => {
            eprintln!(
                "cams_so2_compiler: selftest reads t={:.0} so2={:.3} kg/m2/s",
                rows[0].0, rows[0].1
            );
        }
        _ => {
            eprintln!("cams_so2_compiler: selftest does not read the CDF-1 fixture");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let head = arg_value(&args, "--head").and_then(|n| n.parse::<usize>().ok());
    let loaded = match head {
        Some(n) => {
            let url = match arg_value(&args, "--url") {
                Some(v) => v,
                None => DEFAULT_URL.to_string(),
            };
            fetch_head(&url, n).map(|b| (url, b))
        }
        None => load(&args),
    };
    let Some((src, bytes)) = loaded else {
        eprintln!("cams_so2: {ORIGIN} needs CDS_API_KEY; no NetCDF head is readable here");
        std::process::exit(1);
    };
    match parse_series(&bytes) {
        Some(rows) => print_rows(&src, &rows),
        None => {
            eprintln!(
                "{src}: {} B carry no readable so2 variable (CDF/HDF5 or keyed ADS response absent)",
                bytes.len()
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_cdf1_so2_mean_per_time_step() {
        let rows = parse_series(&classic_fixture()).expect("CDF-1 so2 reads");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].2, COMP_SO2);
        assert!((rows[0].1 - 2.5).abs() < 1e-6);
        assert!((rows[1].1 - 25.0).abs() < 1e-6);
        assert!((rows[1].0 - rows[0].0 - 6.0 * 3600.0).abs() < 1e-6);
    }

    #[test]
    fn absent_for_non_netcdf_bytes() {
        assert!(parse_series(b"not a netcdf file").is_none());
    }
}
