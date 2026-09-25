use omegaflow::archivar::hdf5::{Endian, Hdf5File};
use omegaflow::cdn::upload_release;
use omegaflow::inflate::unzip;
use std::io::Write;

const URL: &str = "https://wwlln.net/climate/th_yr/data/WWLLN_th_2025.nc.zip";
const NETLOC: &str = "wwlln.net";
const NLON: usize = 7200;
const NLAT: usize = 3600;
const NMON: usize = 12;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "wwlln_th.csv".to_string(),
    };

    let zipped = match arg_value(&args, "--input") {
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
    let nc = if zipped.starts_with(b"PK\x03\x04") {
        match unzip(&zipped) {
            Some(b) => b,
            None => {
                eprintln!("zip carries no readable member — the asset stays unwritten (0 honored)");
                std::process::exit(1);
            }
        }
    } else {
        zipped
    };
    let file = match Hdf5File::parse(&nc) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("hdf5 parse returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    let lon = match file.read_f64_dataset("lon") {
        Ok(v) if v.len() == NLON => v,
        Ok(v) => {
            eprintln!("lon carries {} values, expected {NLON}", v.len());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("lon read returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    let lat = match file.read_f64_dataset("lat") {
        Ok(v) if v.len() == NLAT => v,
        Ok(v) => {
            eprintln!("lat carries {} values, expected {NLAT}", v.len());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("lat read returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    let (_, _, dt) = match file.dataset("thunder_hours") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("thunder_hours dataset returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    if dt.class != 0 || dt.size != 2 {
        eprintln!(
            "thunder_hours carries class {} size {} — expected a 2-byte integer",
            dt.class, dt.size
        );
        std::process::exit(1);
    }
    let raw = match file.read_dataset("thunder_hours") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("thunder_hours read returned void: {:?}", e);
            std::process::exit(1);
        }
    };
    if raw.len() != NLON * NLAT * NMON * 2 {
        eprintln!(
            "thunder_hours carries {} bytes, expected {}",
            raw.len(),
            NLON * NLAT * NMON * 2
        );
        std::process::exit(1);
    }
    let cell = |idx: usize| -> Option<i16> {
        let c = raw.get(idx * 2..idx * 2 + 2)?;
        Some(match dt.endian {
            Endian::Le => i16::from_le_bytes([c[0], c[1]]),
            Endian::Be => i16::from_be_bytes([c[0], c[1]]),
        })
    };

    let mut f = match std::fs::File::create(&out) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("create {} returned void: {}", out, e);
            std::process::exit(1);
        }
    };
    if f.write_all(b"lon,lat,thunder_hours\n").is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    let mut records: u64 = 0;
    let mut line = String::new();
    for li in 0..NLON {
        for la in 0..NLAT {
            let base = (li * NLAT + la) * NMON;
            let mut sum: i64 = 0;
            for m in 0..NMON {
                match cell(base + m) {
                    Some(v) if v > 0 => sum += v as i64,
                    Some(_) => {}
                    None => {
                        eprintln!(
                            "thunder_hours cell {} absent — the grid stays incomplete",
                            base + m
                        );
                        std::process::exit(1);
                    }
                }
            }
            if sum <= 0 {
                continue;
            }
            line.clear();
            line.push_str(&format!("{},{},{}\n", lon[li], lat[la], sum));
            if f.write_all(line.as_bytes()).is_err() {
                eprintln!("write {} returned void", out);
                std::process::exit(1);
            }
            records += 1;
        }
    }
    if f.flush().is_err() {
        eprintln!("flush {} returned void", out);
        std::process::exit(1);
    }
    if records == 0 {
        eprintln!("wwlln: no positive thunder-hour cell — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "wwlln harvested {} annual thunder-hour cells -> {}",
        records, out
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("upload_release for {} returned void", out);
        std::process::exit(1);
    }
}
