use omegaflow::archivar::asset_path::{compiled, raw};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::inflate::gunzip_tar_members;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage};
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::lsk::days_from_civil;
use omegaflow::mwo_magnetogram::{Record, parse_bin, write_bin};
use std::process::Command;

const NETLOC: &str = "dataverse.harvard.edu";
const DEFAULT_OUT: &str = "mwo_magnetogram.bin";
const FITS_BZERO_ABSENT: f64 = 0.0;
const DATASET: &str =
    "https://dataverse.harvard.edu/api/datasets/:persistentId/?persistentId=doi:10.7910/DVN/VQK56Z";
const FILE_BASE: &str = "https://dataverse.harvard.edu/api/access/datafile/";
const GAUSS: f64 = 1e-4;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn obj<'a>(json: &'a JsonVal, key: &str) -> Option<&'a JsonVal> {
    let JsonVal::Obj(map) = json else { return None };
    map.get(key)
}

fn arr<'a>(json: &'a JsonVal, key: &str) -> Option<&'a Vec<JsonVal>> {
    match obj(json, key)? {
        JsonVal::Arr(v) => Some(v),
        _ => None,
    }
}

fn str_of(json: &JsonVal, key: &str) -> Option<String> {
    match obj(json, key)? {
        JsonVal::Str(s) => Some(s.clone()),
        _ => None,
    }
}

fn num_of(json: &JsonVal, key: &str) -> Option<f64> {
    match obj(json, key)? {
        JsonVal::Num(n) => Some(*n),
        _ => None,
    }
}

fn list_years(json: &JsonVal) -> Vec<(i64, i64)> {
    let mut out = Vec::new();
    let Some(files) = obj(json, "data")
        .and_then(|d| obj(d, "latestVersion"))
        .and_then(|v| arr(v, "files"))
    else {
        return out;
    };
    for f in files {
        let Some(df) = obj(f, "dataFile") else {
            continue;
        };
        let Some(name) = str_of(df, "filename") else {
            continue;
        };
        let Some(stem) = name
            .strip_prefix("MWO_")
            .and_then(|s| s.strip_suffix(".tar.gz"))
        else {
            continue;
        };
        let Ok(year) = stem.parse::<i64>() else {
            continue;
        };
        let Some(id) = num_of(df, "id") else {
            continue;
        };
        out.push((year, id as i64));
    }
    out.sort_by_key(|(y, _)| *y);
    out
}

fn download(url: &str, path: &str) -> bool {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--retry-delay")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg("1800")
        .arg("-o")
        .arg(path)
        .arg(url)
        .output();
    matches!(out, Ok(o) if o.status.success())
}

fn parse_date_obs(s: &str, era_year: i64) -> Option<i64> {
    let s = s.split(',').next()?.trim();
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let (year, month, day) = if parts[0].len() == 4 {
        (
            parts[0].parse::<i64>().ok()?,
            parts[1].parse::<i64>().ok()?,
            parts[2].parse::<i64>().ok()?,
        )
    } else {
        let y2 = parts[2].parse::<i64>().ok()?;
        let base = era_year - era_year.rem_euclid(100);
        let mut year = base + y2;
        if (year - era_year).abs() > 50 {
            year += if year < era_year { 100 } else { -100 };
        }
        if (year - era_year).abs() > 1 {
            return None;
        }
        (
            year,
            parts[0].parse::<i64>().ok()?,
            parts[1].parse::<i64>().ok()?,
        )
    };
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    days_from_civil(year, month, day)
}

fn disc_means(bytes: &[u8]) -> Option<(f64, f64)> {
    let (header, _) = FitsHeader::parse(bytes, 0)?;
    let (img, _) = FitsImage::parse(bytes, 0)?;
    let (w, h) = (img.dims[0], img.dims[1]);
    if w == 0 || h == 0 {
        return None;
    }
    let x0 = header.f64("X0").unwrap_or((w as f64 + 1.0) / 2.0);
    let y0 = header.f64("Y0").unwrap_or((h as f64 + 1.0) / 2.0);
    let r0 = header.f64("R0").unwrap_or(w.min(h) as f64 / 2.0).max(1.0);
    let bscale = header.f64("BSCALE").unwrap_or(1.0);
    let bzero = header.f64("BZERO").unwrap_or(FITS_BZERO_ABSENT);
    let blank = header.int("BLANK").map(|b| b as f64 * bscale + bzero);
    let r2 = r0 * r0;
    let mut sum = 0.0f64;
    let mut abs_sum = 0.0f64;
    let mut n = 0usize;
    for y in 0..h {
        let dy = y as f64 + 1.0 - y0;
        for x in 0..w {
            let dx = x as f64 + 1.0 - x0;
            if dx * dx + dy * dy > r2 {
                continue;
            }
            let Some(v) = img.value_f64(bytes, [x, y, 0]) else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            if let Some(b) = blank {
                if v == b {
                    continue;
                }
            }
            sum += v;
            abs_sum += v.abs();
            n += 1;
        }
    }
    if n < 4 {
        return None;
    }
    Some((sum / n as f64 * GAUSS, abs_sum / n as f64 * GAUSS))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            let path = compiled(NETLOC, DEFAULT_OUT);
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            path.to_string_lossy().into_owned()
        }
    };
    let cache_dir = match arg_value(&args, "--cache-dir") {
        Some(v) => v,
        None => raw(NETLOC, "cache/mwo").to_string_lossy().into_owned(),
    };
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{cache_dir}: cache dir stays uncreatable");
        std::process::exit(1);
    }

    let Some(body) = fetch_raw_bytes(DATASET) else {
        eprintln!("dataverse index void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let text = String::from_utf8_lossy(&body);
    let Some(json) = parse_json(&text) else {
        eprintln!("dataverse index is not json — the series stays unwritten");
        std::process::exit(1);
    };
    let mut years = list_years(&json);
    if years.is_empty() {
        eprintln!("dataverse index carries no MWO_<year>.tar.gz — unwritten");
        std::process::exit(1);
    }
    if let Some(year) = arg_value(&args, "--year").and_then(|v| v.parse::<i64>().ok()) {
        years.retain(|(y, _)| *y == year);
    }
    if let Some(limit) = arg_value(&args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        years.truncate(limit);
    }
    eprintln!("MWO mirror: {} Jahres-Archive", years.len());

    let mut by_day: std::collections::BTreeMap<i64, (f64, f64, usize)> =
        std::collections::BTreeMap::new();
    let mut scans = 0usize;
    let mut failed_years = 0usize;
    for (year, id) in &years {
        let path = format!("{cache_dir}/mwo_{year}.tar.gz");
        if !std::path::Path::new(&path).exists() && !download(&format!("{FILE_BASE}{id}"), &path) {
            eprintln!("{year}: download void — year skipped");
            failed_years += 1;
            continue;
        }
        let Ok(file) = std::fs::File::open(&path) else {
            eprintln!("{year}: cache unreadable — year skipped");
            failed_years += 1;
            continue;
        };
        let mut year_scans = 0usize;
        let mut year_bad = 0usize;
        let res = gunzip_tar_members(
            file,
            |name: &str| name.ends_with(".fits"),
            |_name: &str, bytes: &[u8]| {
                let Some((mean, mean_abs)) = disc_means(bytes) else {
                    year_bad += 1;
                    return;
                };
                let Some(day) = FitsHeader::parse(bytes, 0)
                    .and_then(|(h, _)| h.str_unescaped("DATE-OBS"))
                    .and_then(|s| parse_date_obs(&s, *year))
                else {
                    year_bad += 1;
                    return;
                };
                let slot = by_day.entry(day).or_insert((0.0, 0.0, 0));
                slot.0 += mean;
                slot.1 += mean_abs;
                slot.2 += 1;
                year_scans += 1;
            },
        );
        match res {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{year}: {e}");
                failed_years += 1;
                continue;
            }
        }
        scans += year_scans;
        eprintln!("{year}: {year_scans} scans, {year_bad} skipped (no date/disc)");
    }
    if scans == 0 {
        eprintln!("no scans — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }

    let records: Vec<Record> = by_day
        .into_iter()
        .map(|(day, (sum, abs_sum, n))| (day, sum / n as f64, abs_sum / n as f64))
        .collect();
    eprintln!(
        "MWO magnetogram: {} days from {} scans ({} years failed)",
        records.len(),
        scans,
        failed_years
    );
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
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        out.push_str(&card("BLANK", "-32768"));
        out.push_str(&card("X0", "2.0"));
        out.push_str(&card("Y0", "2.0"));
        out.push_str(&card("R0", "1.2"));
        out.push_str(&card("DATE-OBS", "'01-14-13'"));
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
    fn date_obs_covers_all_three_eras() {
        assert_eq!(
            parse_date_obs("03-20-67,   2", 1967),
            days_from_civil(1967, 3, 20)
        );
        assert_eq!(
            parse_date_obs("11-14-95", 1995),
            days_from_civil(1995, 11, 14)
        );
        assert_eq!(
            parse_date_obs("01-14-13", 2013),
            days_from_civil(2013, 1, 14)
        );
        assert_eq!(
            parse_date_obs("2013-01-14", 2013),
            days_from_civil(2013, 1, 14)
        );
        assert_eq!(parse_date_obs("06-01-64", 1967), None);
        assert_eq!(parse_date_obs("nonsense", 2000), None);
    }

    #[test]
    fn disc_means_skips_blank_and_outside_pixels() {
        let values = [
            -32768i16, 100, -32768, -32768, 200, 100, -32768, 100, -32768,
        ];
        let bytes = fits_fixture(&values, 3, 3);
        let (mean, mean_abs) = disc_means(&bytes).unwrap();
        assert!((mean - 12.5e-4).abs() < 1e-12, "mean {mean}");
        assert!((mean_abs - 12.5e-4).abs() < 1e-12, "abs {mean_abs}");
    }

    #[test]
    fn disc_means_skips_an_all_blank_frame() {
        let values = [-32768i16; 9];
        let bytes = fits_fixture(&values, 3, 3);
        assert!(disc_means(&bytes).is_none());
    }

    #[test]
    fn bin_roundtrip_keeps_the_daily_pair() {
        let records = vec![(-1013, -2.1e-5, 1.18e-3), (-1044, 3.4e-5, 1.21e-3)];
        let bytes = write_bin(&records);
        assert_eq!(parse_bin(&bytes).unwrap(), records);
    }
}
