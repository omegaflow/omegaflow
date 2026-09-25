use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_WOD_DOXY, COMP_WOD_PSAL, COMP_WOD_TEMP, GeoRec, MAGIC_WOD, parse_bin, write_bin,
};
use omegaflow::archivar::lsk::{self, days_from_civil};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::Hdf5File;
use omegaflow::netcdf::{nc4_group, nc4_ragged_f64};

const NETLOC: &str = "noaa-wod-pds.s3.amazonaws.com";

const WOD_INSTRUMENTS: &[&str] = &[
    "ctd", "drb", "gld", "mbt", "mrb", "osd", "pfl", "uor", "xbt",
];

const LEVEL_VARS: &[(&str, u32)] = &[
    ("Temperature", COMP_WOD_TEMP),
    ("Salinity", COMP_WOD_PSAL),
    ("Oxygen", COMP_WOD_DOXY),
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn attr_f64(file: &Hdf5File, ds: &str, an: &str) -> Option<f64> {
    let a = file.attribute(ds, an)?;
    let e = a.datatype.endian;
    match (a.datatype.class, a.datatype.size) {
        (1, 4) => omegaflow::hdf5::decode_f32(&a.data, 0, e).map(|x| x as f64),
        (1, 8) => omegaflow::hdf5::decode_f64(&a.data, 0, e),
        _ => None,
    }
}

fn attr_text(file: &Hdf5File, ds: &str, an: &str) -> Option<String> {
    let a = file.attribute(ds, an)?;
    if a.datatype.class != 3 {
        return None;
    }
    Some(
        String::from_utf8_lossy(&a.data)
            .trim_end_matches('\0')
            .to_string(),
    )
}

fn fill_of(file: &Hdf5File, ds: &str) -> Option<f64> {
    attr_f64(file, ds, "_FillValue").or_else(|| attr_f64(file, ds, "missing_value"))
}

fn is_fill(v: f64, fill: Option<f64>) -> bool {
    match fill {
        Some(f) => v == f,
        None => false,
    }
}

fn series(file: &Hdf5File, ds: &str) -> Option<Vec<f64>> {
    file.read_f64_dataset(ds).ok()
}

fn ragged(file: &Hdf5File, values: &str, row_size: &str) -> Option<Vec<Vec<f64>>> {
    nc4_ragged_f64(file, values, row_size).ok()
}

fn valid_level(v: f64, fill: Option<f64>) -> Option<f64> {
    if !v.is_finite() || is_fill(v, fill) {
        return None;
    }
    Some(v)
}

fn unix_of(time: f64) -> Option<f64> {
    if !time.is_finite() || time < 0.0 {
        return None;
    }
    let epoch = days_from_civil(1770, 1, 1)? as f64;
    Some((time + epoch) * 86400.0)
}

fn row_of(rows: &[Vec<f64>], i: usize) -> Option<&[f64]> {
    rows.get(i).map(|r| r.as_slice())
}

struct Counters {
    casts_emitted: usize,
    casts_skipped_time: usize,
    levels_skipped: usize,
}

fn compile_file(bytes: &[u8], lsk: &lsk::LeapSeconds) -> Result<Vec<GeoRec>, String> {
    let file = Hdf5File::parse(bytes).map_err(|n| format!("hdf5 parse: {n:?}"))?;
    let lat = series(&file, "lat").ok_or_else(|| "lat absent".to_string())?;
    let lon = series(&file, "lon").ok_or_else(|| "lon absent".to_string())?;
    let time = series(&file, "time").ok_or_else(|| "time absent".to_string())?;
    let z = ragged(&file, "z", "z_row_size").ok_or_else(|| "z/z_row_size absent".to_string())?;
    let ncast = lat.len().min(lon.len()).min(time.len());
    if ncast == 0 {
        return Err("no cast coordinate rows".to_string());
    }
    let mut out = Vec::new();
    let mut c = Counters {
        casts_emitted: 0,
        casts_skipped_time: 0,
        levels_skipped: 0,
    };
    for var in LEVEL_VARS {
        let (name, comp) = *var;
        let Some(values) = ragged(&file, name, &format!("{name}_row_size")) else {
            continue;
        };
        let flags = ragged(
            &file,
            &format!("{name}_WODflag"),
            &format!("{name}_row_size"),
        );
        let fill = fill_of(&file, name);
        for i in 0..ncast {
            let Some(latv) = lat.get(i).copied() else {
                continue;
            };
            let Some(lonv) = lon.get(i).copied() else {
                continue;
            };
            if !(-90.0..=90.0).contains(&latv) || !(-360.0..=360.0).contains(&lonv) {
                continue;
            }
            let Some(unix) = unix_of(time[i]) else {
                c.casts_skipped_time += 1;
                continue;
            };
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                c.casts_skipped_time += 1;
                continue;
            };
            let Some(vrow) = row_of(&values, i) else {
                continue;
            };
            let Some(zrow) = row_of(&z, i) else {
                continue;
            };
            let frow = flags.as_ref().and_then(|f| row_of(f, i));
            let mut emitted = 0usize;
            for (k, &raw) in vrow.iter().enumerate() {
                let Some(v) = valid_level(raw, fill) else {
                    c.levels_skipped += 1;
                    continue;
                };
                if let Some(f) = frow.and_then(|r| r.get(k).copied()) {
                    if f.is_finite() && f != 0.0 {
                        c.levels_skipped += 1;
                        continue;
                    }
                }
                let Some(&depth) = zrow.get(k) else {
                    c.levels_skipped += 1;
                    continue;
                };
                if !depth.is_finite() || depth < 0.0 {
                    c.levels_skipped += 1;
                    continue;
                }
                out.push(GeoRec {
                    t: tdb,
                    lat: latv,
                    lon: lonv,
                    alt: -depth,
                    freq: 0.0,
                    bin_width: 0.0,
                    val: v,
                    comp,
                    station: i as u32,
                });
                emitted += 1;
            }
            if emitted > 0 {
                c.casts_emitted += 1;
            }
        }
    }
    if out.is_empty() {
        return Err("no cast level rows — the asset stays unwritten (0 honored)".to_string());
    }
    eprintln!(
        "wod: {} level rows from {} casts, {} casts before 1972 (TDB void), {} levels skipped",
        out.len(),
        c.casts_emitted,
        c.casts_skipped_time,
        c.levels_skipped
    );
    Ok(out)
}

fn probe(bytes: &[u8], name: &str) {
    let mut file = match Hdf5File::parse(bytes) {
        Ok(f) => f,
        Err(n) => {
            eprintln!("wod: {name} hdf5 parse: {n:?}");
            return;
        }
    };
    let group = match nc4_group(&mut file, "") {
        Ok(g) => g,
        Err(n) => {
            eprintln!("wod: {name} group enumeration: {n:?}");
            return;
        }
    };
    eprintln!(
        "wod: {name}: {} variables, {} groups, {} named types",
        group.variables.len(),
        group.groups.len(),
        group.named_types.len()
    );
    for v in &group.variables {
        let cls = match v.datatype_class {
            Some(0) => "int",
            Some(1) => "float",
            Some(3) => "string",
            Some(9) => "vlen",
            Some(_) => "class",
            None => "absent",
        };
        let size = v
            .datatype_size
            .map_or("absent".to_string(), |s| s.to_string());
        let dims: Vec<String> = v.dims.iter().map(|d| d.to_string()).collect();
        eprintln!(
            "  {:<28} {:<6} {:<2} dims [{}]",
            v.name,
            cls,
            size,
            dims.join(",")
        );
        if v.name == "z" || v.name == "Temperature" || v.name == "Salinity" {
            if let Some(fill) = fill_of(&file, &v.name) {
                eprintln!("      fill {fill}");
            }
            if let Some(u) = attr_text(&file, &v.name, "units") {
                eprintln!("      units {u}");
            }
        }
    }
    if let (Some(lat), Some(lon)) = (series(&file, "lat"), series(&file, "lon")) {
        eprintln!(
            "  lat[0]={} lon[0]={}",
            lat.first().copied().unwrap_or(f64::NAN),
            lon.first().copied().unwrap_or(f64::NAN)
        );
    }
    if let Some(d) = series(&file, "date") {
        let t = match series(&file, "time") {
            Some(v) => v,
            None => Vec::new(),
        };
        eprintln!(
            "  date[0]={} time[0]={} ncast={} time_units={:?} GMT_time[0]={}",
            d.first().copied().unwrap_or(f64::NAN),
            t.first().copied().unwrap_or(f64::NAN),
            d.len(),
            attr_text(&file, "time", "units"),
            series(&file, "GMT_time")
                .and_then(|g| g.first().copied())
                .unwrap_or(f64::NAN)
        );
    }
    if let Some(z) = ragged(&file, "z", "z_row_size") {
        if let Some(first) = z.first() {
            eprintln!("  first cast depth levels: {first:?}");
        }
    }
    if let Some(tmp) = ragged(&file, "Temperature", "Temperature_row_size") {
        if let Some(first) = tmp.first() {
            eprintln!("  first cast Temperature levels: {first:?}");
        }
    }
}

fn load_bytes(args: &[String]) -> Option<(Vec<u8>, String)> {
    if let Some(path) = arg_value(args, "--probe") {
        return std::fs::read(&path).ok().map(|b| (b, path));
    }
    let path = arg_value(args, "--input")?;
    let bytes = if path.starts_with("http://") || path.starts_with("https://") {
        fetch_raw_bytes(&path)
    } else {
        std::fs::read(&path).ok()
    };
    bytes.map(|b| (b, path))
}

fn current_year() -> Option<i64> {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs() as i64;
    let days = secs.div_euclid(86400);
    let mut year = 1970 + days / 365;
    loop {
        let start = days_from_civil(year, 1, 1)?;
        if start > days {
            year -= 1;
            continue;
        }
        let next = days_from_civil(year + 1, 1, 1)?;
        if days < next {
            return Some(year);
        }
        year += 1;
    }
}

fn run_loop(args: &[String]) -> i32 {
    let Some(lsk_text) = arg_value(args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok())
    else {
        eprintln!("wod: --lsk <naif0012.tls> stays unread — the TDB clock stays unread");
        return 1;
    };
    let Some(lsk) = lsk::parse(&lsk_text) else {
        eprintln!("wod: --lsk parses void");
        return 1;
    };
    let start_year: i64 = arg_value(args, "--start-year")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000);
    let end_year: i64 = arg_value(args, "--end-year")
        .and_then(|v| v.parse().ok())
        .or_else(current_year)
        .unwrap_or(2026);
    let instruments: Vec<String> = match arg_value(args, "--instruments") {
        Some(v) => v
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        None => WOD_INSTRUMENTS.iter().map(|s| s.to_string()).collect(),
    };
    let out_dir = match arg_value(args, "--out-dir") {
        Some(v) => v,
        None => ".".to_string(),
    };
    let ci_mode = flag(args, "--ci-mode");
    let _ = std::fs::create_dir_all(&out_dir);
    let mut compiled = 0usize;
    let mut absent = 0usize;
    let mut void = 0usize;
    for year in start_year..=end_year {
        for instrument in &instruments {
            let url =
                format!("https://noaa-wod-pds.s3.amazonaws.com/{year}/wod_{instrument}_{year}.nc");
            let Some(bytes) = fetch_raw_bytes(&url) else {
                absent += 1;
                continue;
            };
            let records = match compile_file(&bytes, &lsk) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("wod: {instrument} {year}: {e}");
                    void += 1;
                    continue;
                }
            };
            let out_path = format!("{out_dir}/noaa_wod_{year}_{instrument}.bin");
            let bytes_out = write_bin(MAGIC_WOD, &records);
            if std::fs::write(&out_path, &bytes_out).is_err() {
                eprintln!("wod: write {out_path} returned void");
                void += 1;
                continue;
            }
            if ci_mode && !upload_release(NETLOC, &out_path) {
                void += 1;
                continue;
            }
            compiled += 1;
        }
    }
    eprintln!(
        "wod loop {start_year}..={end_year} x {} instruments: {compiled} assets compiled, {absent} granules absent, {void} granules void",
        instruments.len()
    );
    if compiled == 0 { 1 } else { 0 }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if flag(&args, "--probe") {
        if let Some((bytes, name)) = load_bytes(&args) {
            probe(&bytes, &name);
            return;
        }
        eprintln!("wod_compiler: --probe needs a readable <file>");
        std::process::exit(1);
    }
    if flag(&args, "--loop") {
        std::process::exit(run_loop(&args));
    }
    let Some((bytes, name)) = load_bytes(&args) else {
        eprintln!(
            "usage: wod_compiler --probe <file>  |  --input <file|url> --out <asset> --lsk <naif0012.tls> [--ci-mode]  |  --loop --lsk <naif0012.tls> [--start-year YYYY] [--end-year YYYY] [--instruments ctd,osd,...] [--out-dir <dir>] [--ci-mode]"
        );
        std::process::exit(1);
    };
    let Some(out_path) = arg_value(&args, "--out") else {
        eprintln!("wod_compiler: --out absent");
        std::process::exit(1);
    };
    let Some(lsk_text) = arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok())
    else {
        eprintln!("wod: --lsk <naif0012.tls> stays unread — the TDB clock stays unread");
        std::process::exit(1);
    };
    let Some(lsk) = lsk::parse(&lsk_text) else {
        eprintln!("wod: --lsk parses void");
        std::process::exit(1);
    };
    let records = match compile_file(&bytes, &lsk) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("wod_compiler: {name}: {e}");
            std::process::exit(1);
        }
    };
    let bytes_out = write_bin(MAGIC_WOD, &records);
    if std::fs::write(&out_path, &bytes_out).is_err() {
        eprintln!("wod: write {out_path} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_WOD, &bytes_out) {
        Some(parsed) => eprintln!(
            "wod: {name} -> {} records, {} B, roundtrip parses",
            parsed.len(),
            bytes_out.len()
        ),
        None => {
            eprintln!("wod: {out_path} roundtrip parse void");
            std::process::exit(1);
        }
    }
    if flag(&args, "--ci-mode") && !upload_release(NETLOC, &out_path) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_of_maps_days_since_1770() {
        let epoch = days_from_civil(1770, 1, 1).unwrap() as f64;
        assert_eq!(unix_of(0.0), Some(epoch * 86400.0));
        assert_eq!(unix_of(1.0), Some((epoch + 1.0) * 86400.0));
        let d1903 = days_from_civil(1903, 5, 6).unwrap() as f64;
        assert_eq!(unix_of(48701.0), Some(d1903 * 86400.0));
        assert!(unix_of(f64::NAN).is_none());
        assert!(unix_of(-1.0).is_none());
    }

    #[test]
    fn valid_level_refuses_fill_and_non_finite() {
        assert_eq!(valid_level(12.5, Some(-99.9)), Some(12.5));
        assert_eq!(valid_level(-99.9, Some(-99.9)), None);
        assert_eq!(valid_level(f64::NAN, None), None);
        assert_eq!(valid_level(f64::INFINITY, None), None);
        assert_eq!(valid_level(0.0, None), Some(0.0));
    }

    #[test]
    fn current_year_reads_the_system_clock() {
        let y = current_year().expect("system clock reads");
        assert!((2000..=2100).contains(&y), "clock year {y}");
    }
}
