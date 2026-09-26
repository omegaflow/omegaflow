use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{COMP_NOHR_SNOWFALL, GeoRec, MAGIC_NOHR, parse_bin, write_bin};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File, decode_f32, decode_f64};
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const NETLOC: &str = "nohrsc.noaa.gov";
const BASE: &str = "https://www.nohrsc.noaa.gov/snowfall/data";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn decode_series(bytes: &[u8], elem: usize, off: usize) -> Option<f64> {
    match elem {
        4 => decode_f32(bytes, off * 4, Endian::Le).map(|v| v as f64),
        8 => decode_f64(bytes, off * 8, Endian::Le),
        _ => None,
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let year = arg_value(args, "--year")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| "--year (e.g. 2026) required".to_string())?;
    let month = arg_value(args, "--month")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| "--month (1-12) required".to_string())?;
    let day = arg_value(args, "--day")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| "--day (1-31) required".to_string())?;
    let stride = arg_value(args, "--stride")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(32);
    if stride == 0 {
        return Err("--stride carries no positive sampling step".to_string());
    }
    let out = arg_value(args, "--out").ok_or_else(|| "--out (path) required".to_string())?;

    let url = format!("{BASE}/{year}{month:02}/sfav2_CONUS_24h_{year}{month:02}{day:02}12.nc");
    let bytes = fetch(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let file = Hdf5File::parse(&bytes).map_err(|e| format!("{url}: hdf5 parses void ({e:?})"))?;
    let (lat_raw, lat_elem, n_lat) = read_axis(&file, "lat")?;
    let (lon_raw, lon_elem, n_lon) = read_axis(&file, "lon")?;
    let (data_raw, data_elem, n_data) = read_axis(&file, "Data")?;
    let n_grid = n_lat
        .checked_mul(n_lon)
        .ok_or_else(|| "the grid shape overflows — the bin stays unwritten".to_string())?;
    if n_data != n_grid {
        return Err(format!(
            "the grid shape reads void — {n_lat} lat, {n_lon} lon, {n_data} Data cells; the bin stays unwritten"
        ));
    }

    let tdb = days_from_civil(year, month, day)
        .and_then(|d| lsk.unix_to_tdb(d as f64 * 86400.0 + 12.0 * 3600.0))
        .ok_or_else(|| "the accumulation-end date reads void".to_string())?;

    let mut records = Vec::new();
    let mut sampled = 0usize;
    let mut kept = 0usize;
    let mut i = 0usize;
    while i < n_lat {
        let Some(lat) = decode_series(&lat_raw, lat_elem, i) else {
            break;
        };
        let mut j = 0usize;
        while j < n_lon {
            let Some(lon) = decode_series(&lon_raw, lon_elem, j) else {
                break;
            };
            let Some(val) = decode_series(&data_raw, data_elem, i * n_lon + j) else {
                break;
            };
            sampled += 1;
            if val.is_finite() && val >= 0.0 && val <= 10000.0 {
                records.push(GeoRec {
                    t: tdb,
                    lat,
                    lon,
                    alt: 0.0,
                    freq: 0.0,
                    bin_width: 0.0,
                    val,
                    comp: COMP_NOHR_SNOWFALL,
                    station: 0,
                });
                kept += 1;
            }
            j += stride;
        }
        i += stride;
    }

    if records.is_empty() {
        return Err(format!(
            "{url}: no measured cell left the harvest ({sampled} sampled, {kept} kept) — the bin stays unwritten (0 honored)"
        ));
    }
    let bytes = write_bin(MAGIC_NOHR, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(MAGIC_NOHR, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} snowfall cells ({} sampled, {} kept, grid {n_lat}x{n_lon}), roundtrip parses",
                parsed.len(),
                sampled,
                kept
            );
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn read_axis(file: &Hdf5File, name: &str) -> Result<(Vec<u8>, usize, usize), String> {
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
    Ok((raw, dt.size, count))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("nohrsc_snowfall_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn series_decode_follows_the_element_size() {
        let f32_le = 2.0f32.to_le_bytes();
        let f64_le = 2.0f64.to_le_bytes();
        assert_eq!(decode_series(&f32_le, 4, 0), Some(2.0));
        assert_eq!(decode_series(&f64_le, 8, 0), Some(2.0));
        assert_eq!(decode_series(&f32_le, 8, 0), None);
    }
}
