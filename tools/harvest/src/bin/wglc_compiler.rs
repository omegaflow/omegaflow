use omegaflow::archivar::hdf5::{Endian, Hdf5File};
use omegaflow::cdn::upload_release;
use std::io::Write;

const URL: &str =
    "https://zenodo.org/api/records/20277101/files/wglc_timeseries_30m_daily.nc/content";
const NETLOC: &str = "zenodo.org";
const FILL: f64 = -9999.0;
const R_EARTH_KM: f64 = 6371.0088;
const DAY: f64 = 86400.0;
const EPOCH_2010: f64 = 1262304000.0;
const TIME_UNITS: &str = "days since 2010-01-01 00:00:00";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn f32_at(bytes: &[u8], off: usize, endian: Endian) -> Option<f32> {
    let c = bytes.get(off..off + 4)?;
    let a: [u8; 4] = c.try_into().ok()?;
    Some(match endian {
        Endian::Le => f32::from_le_bytes(a),
        Endian::Be => f32::from_be_bytes(a),
    })
}

fn cell_areas_km2(lat: &[f64], dlon_deg: f64) -> Vec<f64> {
    let n = lat.len();
    let dlon_rad = dlon_deg.to_radians().abs();
    let mut areas = Vec::with_capacity(n);
    for i in 0..n {
        let lo = if i == 0 {
            lat[0] - (lat[1] - lat[0]) / 2.0
        } else {
            (lat[i - 1] + lat[i]) / 2.0
        };
        let hi = if i + 1 == n {
            lat[i] + (lat[i] - lat[i - 1]) / 2.0
        } else {
            (lat[i] + lat[i + 1]) / 2.0
        };
        let band = (hi.to_radians().sin() - lo.to_radians().sin()).abs();
        areas.push(R_EARTH_KM * R_EARTH_KM * dlon_rad * band);
    }
    areas
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "wglc_stroke_rate.csv".to_string(),
    };

    let bytes = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {} returned void: {}", path, e);
                std::process::exit(1);
            }
        },
        None => match omegaflow::archivar::fetch_raw_bytes(URL) {
            Some(b) => b,
            None => {
                eprintln!("fetch {} returned void", URL);
                std::process::exit(1);
            }
        },
    };

    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("hdf5 parse returned void: {:?}", e);
            std::process::exit(1);
        }
    };

    let units = match file.attribute("time", "units") {
        Some(a) => String::from_utf8_lossy(&a.data).trim().to_string(),
        None => {
            eprintln!(
                "time carries no units attribute — the epoch is unnamed, the asset stays unwritten"
            );
            std::process::exit(1);
        }
    };
    if units != TIME_UNITS {
        eprintln!(
            "time carries units \"{}\" — the compiler was measured against \"{}\", the asset stays unwritten",
            units, TIME_UNITS
        );
        std::process::exit(1);
    }

    let lon = match file.read_f64_dataset("lon") {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            eprintln!("lon carries no values — the asset stays unwritten (0 honored)");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("lon read returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    let lat = match file.read_f64_dataset("lat") {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            eprintln!("lat carries no values — the asset stays unwritten (0 honored)");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("lat read returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    let time = match file.read_f64_dataset("time") {
        Ok(v) if !v.is_empty() => v,
        Ok(_) => {
            eprintln!("time carries no values — the asset stays unwritten (0 honored)");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("time read returned void: {:?}", e);
            std::process::exit(1);
        }
    };

    let (_, ds, dt) = match file.dataset("density") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("density dataset returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    if dt.class != 1 || dt.size != 4 {
        eprintln!(
            "density carries class {} size {} — expected a 4-byte float",
            dt.class, dt.size
        );
        std::process::exit(1);
    }
    let dims = ds.dims.clone();
    if dims.len() != 3 {
        eprintln!(
            "density carries rank {} — expected [time, lat, lon]",
            dims.len()
        );
        std::process::exit(1);
    }
    let (nt, nlat, nlon) = (dims[0] as usize, dims[1] as usize, dims[2] as usize);
    if nlat != lat.len() || nlon != lon.len() || nt != time.len() {
        eprintln!(
            "density shape [{nt},{nlat},{nlon}] does not match lon {} lat {} time {}",
            lon.len(),
            lat.len(),
            time.len()
        );
        std::process::exit(1);
    }
    let dlon_deg = if nlon > 1 { lon[1] - lon[0] } else { 0.0 };
    if dlon_deg == 0.0 {
        eprintln!("lon carries a single column — the globe stays unintegrated");
        std::process::exit(1);
    }
    let areas = cell_areas_km2(&lat, dlon_deg);

    let raw = match file.read_dataset("density") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("density read returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    let expected = nt * nlat * nlon * 4;
    if raw.len() != expected {
        eprintln!("density carries {} bytes, expected {}", raw.len(), expected);
        std::process::exit(1);
    }

    let mut f = match std::fs::File::create(&out) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {} returned void: {}", out, e);
            std::process::exit(1);
        }
    };
    if f.write_all(b"day_since_2010_01_01,epoch_s,strokes_per_day\n")
        .is_err()
    {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }

    let mut records: u64 = 0;
    let mut positive: u64 = 0;
    for t in 0..nt {
        let mut sum = 0.0f64;
        for la in 0..nlat {
            let area = areas[la];
            let row = (t * nlat + la) * nlon;
            for lo in 0..nlon {
                let idx = row + lo;
                let Some(v) = f32_at(&raw, idx * 4, dt.endian) else {
                    eprintln!("density cell {} absent — the series stays unwritten", idx);
                    std::process::exit(1);
                };
                let v = v as f64;
                if !v.is_finite() || v == FILL {
                    continue;
                }
                sum += v * area;
            }
        }
        let epoch = EPOCH_2010 + time[t] * DAY;
        let line = format!("{},{},{}\n", time[t], epoch, sum);
        if f.write_all(line.as_bytes()).is_err() {
            eprintln!("write {} returned void", out);
            std::process::exit(1);
        }
        records += 1;
        if sum > 0.0 {
            positive += 1;
        }
    }
    if f.flush().is_err() {
        eprintln!("flush {} returned void", out);
        std::process::exit(1);
    }
    if records == 0 || positive == 0 {
        eprintln!("wglc: no positive global stroke day — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "wglc harvested {} daily global stroke-rate rows ({} positive) -> {}",
        records, positive, out
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("upload_release for {} returned void", out);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_regular_grid_band_area_matches_the_sphere() {
        let lat: Vec<f64> = (-179..180).map(|i| i as f64 * 0.5 + 0.25).collect();
        let areas = cell_areas_km2(&lat, 0.5);
        let total: f64 = areas.iter().sum();
        let sphere = 4.0 * std::f64::consts::PI * R_EARTH_KM * R_EARTH_KM;
        assert!((total - sphere).abs() / sphere < 1.0e-4);
    }

    #[test]
    fn a_descending_latitude_grid_keeps_positive_areas() {
        let lat: Vec<f64> = (0..360).rev().map(|i| i as f64 * 0.5 - 89.75).collect();
        let areas = cell_areas_km2(&lat, 0.5);
        assert!(areas.iter().all(|a| *a > 0.0));
    }
}
