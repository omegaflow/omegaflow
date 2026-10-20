
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::FitsImage;
use omegaflow::kpvt_polar::{parse_bin, write_bin};

const INDEX: &str = "https://nispdata.nso.edu/ftp/kpvt/synoptic/mag/";
const BASE: &str = "https://nispdata.nso.edu/ftp/kpvt/synoptic/mag/";
const CAP_SIN: f64 = 0.866_025_403_784_438_6;
const GAUSS: f64 = 1e-4;
const CR_ANCHOR: i64 = 1908;
const CR_ANCHOR_DAY: i64 = 9607;
const CARRINGTON_DAYS: f64 = 27.27638;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn carrington_day(cr: i64) -> i64 {
    CR_ANCHOR_DAY + (((cr - CR_ANCHOR) as f64) * CARRINGTON_DAYS).round() as i64
}

fn parse_index(html: &str) -> Vec<i64> {
    let mut out: Vec<i64> = Vec::new();
    let mut rest = html;
    while let Some(i) = rest.find("m") {
        rest = &rest[i + 1..];
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            continue;
        }
        let after = &rest[digits.len()..];
        if !after.starts_with("f.fits") {
            continue;
        }
        let Ok(cr) = digits.parse::<i64>() else {
            continue;
        };
        if !(1600..=2200).contains(&cr) {
            continue;
        }
        if !out.contains(&cr) {
            out.push(cr);
        }
    }
    out.sort_unstable();
    out
}

fn polar_means(bytes: &[u8]) -> Option<(f64, f64)> {
    let (img, _) = FitsImage::parse(bytes, 0)?;
    let (w, h) = (img.dims[0], img.dims[1]);
    if w == 0 || h < 2 {
        return None;
    }
    let mut north_sum = 0.0f64;
    let mut north_n = 0usize;
    let mut south_sum = 0.0f64;
    let mut south_n = 0usize;
    for y in 0..h {
        let sin_lat = -1.0 + 2.0 * y as f64 / (h - 1) as f64;
        let north = sin_lat >= CAP_SIN;
        let south = sin_lat <= -CAP_SIN;
        if !north && !south {
            continue;
        }
        for x in 0..w {
            let Some(gauss) = img.value_f64(bytes, [x, y, 0]) else {
                continue;
            };
            if !gauss.is_finite() {
                continue;
            }
            if north {
                north_sum += gauss;
                north_n += 1;
            } else {
                south_sum += gauss;
                south_n += 1;
            }
        }
    }
    if north_n == 0 || south_n == 0 {
        return None;
    }
    Some((
        north_sum / north_n as f64 * GAUSS,
        south_sum / south_n as f64 * GAUSS,
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "kpvt_polar.bin".to_string(),
    };
    let cache_dir = match arg_value(&args, "--cache-dir") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .join("omegaflow_kpvt_cache")
            .to_string_lossy()
            .into_owned(),
    };
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{cache_dir}: cache dir stays uncreatable");
        std::process::exit(1);
    }

    let Some(index) = fetch_raw_bytes(INDEX) else {
        eprintln!("KPVT index void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let html = String::from_utf8_lossy(&index);
    let mut crs = parse_index(&html);
    if crs.is_empty() {
        eprintln!("KPVT index carries no m<CR>f.fits — unwritten");
        std::process::exit(1);
    }
    if let Some(limit) = arg_value(&args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        crs.truncate(limit);
    }
    eprintln!("KPVT synoptic: {} Karten", crs.len());

    let mut records: Vec<(i64, f64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    for cr in &crs {
        let path = format!("{cache_dir}/m{cr}f.fits");
        let bytes = match std::fs::read(&path) {
            Ok(b) => Some(b),
            Err(_) => {
                let got = fetch_raw_bytes(&format!("{BASE}m{cr}f.fits"));
                if let Some(b) = &got {
                    let _ = std::fs::write(&path, b);
                }
                got
            }
        };
        let Some(bytes) = bytes else {
            eprintln!("CR{cr}: fetch void");
            skipped += 1;
            continue;
        };
        let Some((north, south)) = polar_means(&bytes) else {
            eprintln!("CR{cr}: no polar rows");
            skipped += 1;
            continue;
        };
        let avg = (north - south) / 2.0;
        records.push((carrington_day(*cr), north, south, avg));
    }
    records.sort_by_key(|(d, _, _, _)| *d);
    records.dedup_by_key(|(d, _, _, _)| *d);
    eprintln!(
        "KPVT polar field: {} records, {} skipped (absent)",
        records.len(),
        skipped
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "{out}: {} records, roundtrip parses ({} B)",
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("nispdata.nso.edu", &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::lsk::days_from_civil;

    fn card(key: &str, value: &str) -> String {
        let mut s = format!("{key:<8}= {value:>20}");
        while s.len() < 80 {
            s.push(' ');
        }
        s
    }

    fn fits_fixture(values: &[i16], w: usize, h: usize) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&card("SIMPLE", "T"));
        out.push_str(&card("BITPIX", "16"));
        out.push_str(&card("NAXIS", "2"));
        out.push_str(&card("NAXIS1", &w.to_string()));
        out.push_str(&card("NAXIS2", &h.to_string()));
        out.push_str(&card("BSCALE", "0.1"));
        out.push_str(&card("BZERO", "0.0"));
        out.push_str(&card("END", ""));
        while out.len() % 2880 != 0 {
            out.push(' ');
        }
        let mut bytes = out.into_bytes();
        for v in values {
            bytes.extend_from_slice(&v.to_be_bytes());
        }
        bytes
    }

    #[test]
    fn index_reads_only_the_corrected_maps() {
        let html = r#"
            <a href="m1625f.fits">m1625f.fits</a>
            <a href="m1626f.fits">m1626f.fits</a>
            <a href="m1800g.fits">m1800g.fits</a>
            <a href="m1900f.fits">m1900f.fits</a>
            <a href="../">Parent Directory</a>
        "#;
        assert_eq!(parse_index(html), vec![1625, 1626, 1900]);
    }

    #[test]
    fn carrington_day_stays_on_the_mdi_stamps() {
        assert_eq!(carrington_day(1908), days_from_civil(1996, 4, 21).unwrap());
        assert_eq!(carrington_day(2107), days_from_civil(2011, 3, 2).unwrap());
        assert_eq!(carrington_day(1625), days_from_civil(1975, 3, 4).unwrap());
    }

    #[test]
    fn polar_means_uses_only_the_cap_rows() {
        let values = [
            100i16, 100, 100, 100, // south cap
            999, 999, 999, 999, // equator
            999, 999, 999, 999, // equator
            200, 200, 200, 200, // north cap
        ];
        let bytes = fits_fixture(&values, 4, 4);
        let (north, south) = polar_means(&bytes).unwrap();
        assert!((north - 20.0e-4).abs() < 1e-12, "north {north}");
        assert!((south - 10.0e-4).abs() < 1e-12, "south {south}");
    }

    #[test]
    fn polar_means_reads_the_float32_generation() {
        let mut out = String::new();
        out.push_str(&card("SIMPLE", "T"));
        out.push_str(&card("BITPIX", "-32"));
        out.push_str(&card("NAXIS", "2"));
        out.push_str(&card("NAXIS1", "2"));
        out.push_str(&card("NAXIS2", "2"));
        out.push_str(&card("END", ""));
        while out.len() % 2880 != 0 {
            out.push(' ');
        }
        let mut bytes = out.into_bytes();
        for v in [-30.0f32, -30.0, 40.0, 40.0] {
            bytes.extend_from_slice(&v.to_be_bytes());
        }
        let (north, south) = polar_means(&bytes).unwrap();
        assert!((north - 40.0e-4).abs() < 1e-12, "north {north}");
        assert!((south + 30.0e-4).abs() < 1e-12, "south {south}");
    }
}
