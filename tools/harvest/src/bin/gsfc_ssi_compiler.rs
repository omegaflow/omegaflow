use omegaflow::archivar::embedded_lsk;
use omegaflow::hdf5::{Hdf5File, decode_f32};
use omegaflow::lsk::days_from_civil;
use std::env;
use std::fs;

const MAGIC: [u8; 4] = *b"GSI1";

const N07_WAVELENGTHS: [f64; 3] = [205.5, 280.5, 380.5];
const UV_WAVELENGTHS: [f64; 3] = [121.5, 205.5, 280.5];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Product {
    N07,
    Susim,
    SorceSols,
}

impl Product {
    fn of_name(name: &str) -> Option<Product> {
        let lower = name.to_ascii_lowercase();
        if lower.contains("n07") {
            Some(Product::N07)
        } else if lower.contains("susim") {
            Some(Product::Susim)
        } else if lower.contains("sorce") && lower.contains("sol") {
            Some(Product::SorceSols)
        } else {
            None
        }
    }

    fn wavelengths(self) -> [f64; 3] {
        match self {
            Product::N07 => N07_WAVELENGTHS,
            Product::Susim | Product::SorceSols => UV_WAVELENGTHS,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Product::N07 => "n07",
            Product::Susim => "susim",
            Product::SorceSols => "sorcesols",
        }
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn positional_source(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(a.as_str(), "--source" | "--product" | "--out") {
            i += 2;
            continue;
        }
        if a.starts_with('-') {
            i += 1;
            continue;
        }
        return Some(a.clone());
    }
    None
}

fn read_source(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let out = std::process::Command::new("curl")
            .args(["-sSfL", "--retry", "3", "--max-time", "600", source])
            .output()
            .map_err(|e| format!("{source}: curl void ({e})"))?;
        if !out.status.success() {
            return Err(format!(
                "{source}: fetch void ({})",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        Ok(out.stdout)
    } else {
        fs::read(source).map_err(|e| format!("{source}: read void ({e})"))
    }
}

fn floats(file: &Hdf5File, name: &str) -> Result<Vec<f64>, String> {
    let (_, ds, dt) = file.dataset(name).map_err(|e| format!("{name}: {e:?}"))?;
    if dt.class != 1 || dt.size != 4 {
        return Err(format!(
            "{name}: class {} size {} is not float32",
            dt.class, dt.size
        ));
    }
    let raw = file
        .read_dataset(name)
        .map_err(|e| format!("{name}: {e:?}"))?;
    let total: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let mut out = Vec::with_capacity(total);
    for i in 0..total {
        let v = decode_f32(&raw, i * 4, dt.endian)
            .ok_or_else(|| format!("{name}: float read void at {i}"))?;
        out.push(v as f64);
    }
    Ok(out)
}

fn nearest(waves: &[f64], target: f64) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for (i, w) in waves.iter().enumerate() {
        let d = (w - target).abs();
        if d <= 0.51 && best.map(|(_, bd)| d < bd).unwrap_or(true) {
            best = Some((i, d));
        }
    }
    best.map(|(i, _)| i)
}

fn day_unix(year: f64, doy: f64) -> Option<f64> {
    if !year.is_finite() || !doy.is_finite() {
        return None;
    }
    let year = year as i64;
    let doy = doy as i64;
    if !(1900..=2100).contains(&year) || !(1..=367).contains(&doy) {
        return None;
    }
    let days = days_from_civil(year, 1, 1)? + doy - 1;
    Some(days as f64 * 86400.0 + 43_200.0)
}

fn extract(
    product: Product,
    waves: &[f64],
    irradiance: &[f64],
    fill: &[f64],
    year: &[f64],
    doy: &[f64],
    lsk: &omegaflow::archivar::LeapSeconds,
) -> Result<Vec<(f64, f64, u32)>, String> {
    let nday = year.len();
    if doy.len() != nday {
        return Err(format!("year {} / doy {} lengths disagree", nday, doy.len()));
    }
    if waves.is_empty() || nday == 0 {
        return Err("the grid stays empty".to_string());
    }
    let expect = waves.len() * nday;
    if irradiance.len() != expect {
        return Err(format!(
            "irradiance {} values do not fill {} x {}",
            irradiance.len(),
            waves.len(),
            nday
        ));
    }
    if fill.len() != expect {
        return Err(format!(
            "fill {} values do not fill {} x {}",
            fill.len(),
            waves.len(),
            nday
        ));
    }
    let targets = product.wavelengths();
    let mut columns = [None; 3];
    for (k, target) in targets.iter().enumerate() {
        columns[k] = Some(
            nearest(waves, *target)
                .ok_or_else(|| format!("no 1 nm bin within 0.51 of {target} nm"))?,
        );
    }
    let mut records = Vec::new();
    for d in 0..nday {
        let unix = match day_unix(year[d], doy[d]) {
            Some(t) => t,
            None => continue,
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        for (idx, col) in columns.iter().enumerate() {
            let Some(col) = *col else { continue };
            let at = col * nday + d;
            let v = irradiance[at] / 1000.0;
            if fill[at] != 0.0 || !v.is_finite() || v <= 0.0 {
                continue;
            }
            records.push((tdb, v, idx as u32));
        }
    }
    Ok(records)
}

fn run(args: &[String]) -> Result<(), String> {
    let source = arg_value(args, "--source")
        .or_else(|| positional_source(args))
        .ok_or_else(|| "no source — pass an n07/susim/sorcesols .h5 path or URL".to_string())?;
    let product = match arg_value(args, "--product") {
        Some(v) => Product::of_name(&v)
            .ok_or_else(|| format!("--product {v} is not n07, susim or sorcesols"))?,
        None => Product::of_name(&source)
            .ok_or_else(|| format!("{source}: the product does not follow from the name — pass --product"))?,
    };
    let out = arg_value(args, "--out").ok_or_else(|| "--out <bin> is absent".to_string())?;

    let bytes = read_source(&source)?;
    let file = Hdf5File::parse(&bytes).map_err(|e| format!("{source}: hdf5 void ({e:?})"))?;

    let waves = floats(&file, "WavelengthScale")?;
    let irradiance = floats(&file, "Irradiance")?;
    let fill = floats(&file, "FillFlag")?;
    let year = floats(&file, "Year")?;
    let doy = floats(&file, "DayofYear")?;

    if args.iter().any(|a| a == "--inspect") {
        println!("{source}: product {}", product.label());
        println!("waves {}: {:?} .. {:?}", waves.len(), waves.first(), waves.last());
        let first_day = year.first().map(|v| *v as i64);
        let last_day = year.last().map(|v| *v as i64);
        println!("days {}: {:?} .. {:?}", year.len(), first_day, last_day);
        println!("targets {:?}", product.wavelengths());
        return Ok(());
    }

    let lsk = embedded_lsk().ok_or_else(|| "the embedded leap seconds stay unread".to_string())?;
    let mut records = extract(product, &waves, &irradiance, &fill, &year, &doy, &lsk)?;
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        return Err(format!("{source}: no records — the bin stays unwritten"));
    }

    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, idx) in &records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&idx.to_le_bytes());
    }
    fs::write(&out, &buf).map_err(|e| format!("{out}: write void ({e})"))?;
    let (t0, _, _) = records[0];
    let (t1, _, _) = records[records.len() - 1];
    eprintln!(
        "{out}: {} records ({} B), product {}, epoch_tdb {:.0}..{:.0}",
        records.len(),
        buf.len(),
        product.label(),
        t0,
        t1
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("usage: gsfc_ssi_compiler <n07|susim|sorcesols>.h5 --out <bin> [--source <path|url>] [--product n07|susim|sorcesols] [--inspect]");
        return;
    }
    if let Err(e) = run(&args) {
        eprintln!("gsfc_ssi: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::archivar::embedded_lsk;

    fn lsk() -> omegaflow::archivar::LeapSeconds {
        embedded_lsk().expect("leap seconds")
    }

    #[test]
    fn extracts_the_declared_columns_and_drops_fill() {
        let waves = [120.5, 121.5, 122.5, 204.5, 205.5, 206.5, 279.5, 280.5, 281.5];
        let nday = 2;
        let mut irr = vec![0.0; waves.len() * nday];
        let mut fill = vec![0.0; waves.len() * nday];
        irr[1 * nday + 0] = 7419.3;
        irr[4 * nday + 0] = 11146.0;
        irr[7 * nday + 0] = 1200.0;
        irr[1 * nday + 1] = 7400.0;
        irr[4 * nday + 1] = 9999.0;
        irr[7 * nday + 1] = 1210.0;
        fill[4 * nday + 1] = 1.0;
        let year = [2003.0, 2003.0];
        let doy = [134.0, 135.0];
        let recs = extract(
            Product::SorceSols,
            &waves,
            &irr,
            &fill,
            &year,
            &doy,
            &lsk(),
        )
        .expect("extract");
        assert_eq!(recs.len(), 5);
        assert_eq!(recs[0].2, 0);
        assert!((recs[0].1 - 7.4193).abs() < 1e-9);
        assert!((recs[1].1 - 11.146).abs() < 1e-9);
        assert!((recs[2].1 - 1.2).abs() < 1e-9);
        assert!(recs[3..].iter().all(|r| r.2 != 1));
    }

    #[test]
    fn n07_field_order_is_205_280_380() {
        assert_eq!(Product::N07.wavelengths(), [205.5, 280.5, 380.5]);
        assert_eq!(Product::Susim.wavelengths(), [121.5, 205.5, 280.5]);
        assert_eq!(
            Product::of_name("sorcesols_v16_1nm.h5"),
            Some(Product::SorceSols)
        );
    }

    #[test]
    fn a_missing_column_is_not_silently_defaulted() {
        let waves = [200.5, 210.5];
        let irr = vec![1.0, 1.0];
        let fill = vec![0.0, 0.0];
        let year = [1999.0];
        let doy = [1.0];
        let err = extract(
            Product::Susim,
            &waves,
            &irr,
            &fill,
            &year,
            &doy,
            &lsk(),
        )
        .expect_err("no 205.5 bin");
        assert!(err.contains("no 1 nm bin"));
    }
}
