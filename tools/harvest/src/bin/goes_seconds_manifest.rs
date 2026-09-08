use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{decode_f64, Endian, Hdf5File};
use std::process::Command;

const CDN_TAG: &str = "ncei.noaa.gov";
const BASE: &str = "https://www.ncei.noaa.gov/data/goes-space-environment-monitor/access/science/xrs/goes15/gxrs-l2-irrad_science";
const NAME_PREFIX: &str = "sci_gxrs-l2-irrad_g15_d";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("120")
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

fn curl_text(url: &str) -> Option<String> {
    String::from_utf8(curl_bytes(url)?).ok()
}

fn parse_date(name: &str) -> Option<(u32, u32, u32)> {
    let stem = name.strip_suffix(".nc")?;
    let d = stem.strip_prefix(NAME_PREFIX)?;
    let digits: String = d.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.len() != 8 {
        return None;
    }
    Some((
        digits[0..4].parse().ok()?,
        digits[4..6].parse().ok()?,
        digits[6..8].parse().ok()?,
    ))
}

fn parse_version(name: &str) -> Option<(u32, u32, u32)> {
    let stem = name.strip_suffix(".nc")?;
    let v = stem.strip_prefix(NAME_PREFIX)?.split_once("_v")?.1;
    let mut p = v.split('-');
    Some((
        p.next()?.parse().ok()?,
        p.next()?.parse().ok()?,
        p.next()?.parse().ok()?,
    ))
}

fn index_names(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(pos) = rest.find(NAME_PREFIX) {
        let tail = &rest[pos..];
        let name: String = tail
            .chars()
            .take_while(|c| !matches!(c, '"' | '<' | '>'))
            .collect();
        if name.ends_with(".nc") && !out.contains(&name) {
            out.push(name);
        }
        rest = &rest[pos + NAME_PREFIX.len()..];
    }
    out
}

fn pick_latest(names: &[String]) -> Option<String> {
    names
        .iter()
        .filter(|n| parse_version(n).is_some())
        .max_by_key(|n| parse_version(*n))
        .cloned()
}

fn xr_name(date: (u32, u32, u32)) -> String {
    format!("xr_{:04}{:02}{:02}.nc", date.0, date.1, date.2)
}

fn read_series(path: &str, dataset: &str) -> Option<Vec<f64>> {
    let bytes = std::fs::read(path).ok()?;
    let file = Hdf5File::parse(&bytes).ok()?;
    let raw = file.read_dataset(dataset).ok()?;
    if raw.len() % 8 != 0 {
        return None;
    }
    let n = raw.len() / 8;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(decode_f64(&raw, i * 8, Endian::Le)?);
    }
    Some(out)
}

fn read_f32_series(path: &str, dataset: &str) -> Option<Vec<f32>> {
    let bytes = std::fs::read(path).ok()?;
    let file = Hdf5File::parse(&bytes).ok()?;
    let raw = file.read_dataset(dataset).ok()?;
    if raw.len() % 4 != 0 {
        return None;
    }
    let n = raw.len() / 4;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(f32::from_le_bytes(raw[i * 4..i * 4 + 4].try_into().ok()?));
    }
    Some(out)
}

fn read_flag_series(path: &str, dataset: &str) -> Option<Vec<u16>> {
    let bytes = std::fs::read(path).ok()?;
    let file = Hdf5File::parse(&bytes).ok()?;
    let raw = file.read_dataset(dataset).ok()?;
    if raw.len() % 2 != 0 {
        return None;
    }
    let n = raw.len() / 2;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(u16::from_le_bytes(raw[i * 2..i * 2 + 2].try_into().ok()?));
    }
    Some(out)
}

fn parity_mode(args: &[String]) {
    let Some(a) = arg_value(args, "--parity") else {
        eprintln!("usage: --parity <local.nc> --origin <origin.nc>");
        std::process::exit(2);
    };
    let Some(b) = arg_value(args, "--origin") else {
        eprintln!("usage: --parity <local.nc> --origin <origin.nc>");
        std::process::exit(2);
    };
    let datasets = ["time", "a_flux", "b_flux", "a_flags", "b_flags"];
    for ds in datasets {
        let av = read_series(&a, ds);
        let bv = read_series(&b, ds);
        match (&av, &bv) {
            (Some(x), Some(y)) => {
                let mut max_abs = 0.0f64;
                let mut n_diff = 0usize;
                let n = x.len().min(y.len());
                for i in 0..n {
                    let d = (x[i] - y[i]).abs();
                    if d > max_abs {
                        max_abs = d;
                    }
                    if x[i] != y[i] {
                        n_diff += 1;
                    }
                }
                println!(
                    "PARITY {:<8} records {}/{} max_abs_diff {:.6e} differing {}",
                    ds,
                    x.len(),
                    y.len(),
                    max_abs,
                    n_diff
                );
                if ds == "time" && n > 0 {
                    println!(
                        "PARITY {:<8} first {:.3} / {:.3} last {:.3} / {:.3}",
                        ds,
                        x[0],
                        y[0],
                        x[n - 1],
                        y[n - 1]
                    );
                }
            }
            _ => {}
        }
        if ds != "time" {
            let af32 = read_f32_series(&a, ds);
            let bf32 = read_f32_series(&b, ds);
            match (&af32, &bf32) {
                (Some(x), Some(y)) => {
                    let mut max_abs = 0.0f32;
                    let mut n_diff = 0usize;
                    let n = x.len().min(y.len());
                    for i in 0..n {
                        let d = (x[i] - y[i]).abs();
                        if d > max_abs {
                            max_abs = d;
                        }
                        if x[i] != y[i] {
                            n_diff += 1;
                        }
                    }
                    println!(
                        "PARITY {:<8} f32 records {}/{} max_abs_diff {:.6e} differing {}",
                        ds,
                        x.len(),
                        y.len(),
                        max_abs,
                        n_diff
                    );
                }
                _ => {
                    let af16 = read_flag_series(&a, ds);
                    let bf16 = read_flag_series(&b, ds);
                    match (&af16, &bf16) {
                        (Some(x), Some(y)) => {
                            let mut n_diff = 0usize;
                            let n = x.len().min(y.len());
                            for i in 0..n {
                                if x[i] != y[i] {
                                    n_diff += 1;
                                }
                            }
                            println!(
                                "PARITY {:<8} flags records {}/{} differing {}",
                                ds,
                                x.len(),
                                y.len(),
                                n_diff
                            );
                        }
                        _ => {
                            println!(
                                "PARITY {:<8} series absent — local series {} origin series {}",
                                ds,
                                af32.is_some() || af16.is_some(),
                                bf32.is_some() || bf16.is_some()
                            );
                        }
                    }
                }
            }
        }
    }
}

fn harvest_mode(args: &[String]) {
    let out_dir = match arg_value(args, "--out") {
        Some(v) => v,
        None => "goes-seconds".to_string(),
    };
    let years: Vec<u32> = match arg_value(args, "--years") {
        Some(v) => v,
        None => "2013,2014,2015".to_string(),
    }
    .split(',')
    .filter_map(|y| y.trim().parse().ok())
    .collect();
    if years.is_empty() {
        eprintln!("--years carries no year");
        std::process::exit(2);
    }
    if std::fs::create_dir_all(&out_dir).is_err() {
        eprintln!("{} stays unwritable", out_dir);
        std::process::exit(1);
    }
    let mut downloaded = 0usize;
    let mut skipped_days: Vec<(u32, u32, u32)> = Vec::new();
    for &year in &years {
        for month in 1..=12u32 {
            let idx_url = format!("{BASE}/{year}/{month:02}/");
            let Some(html) = curl_text(&idx_url) else {
                eprintln!(
                    "index {} void — month {:02} stays unharvested",
                    idx_url, month
                );
                continue;
            };
            let mut by_date: Vec<((u32, u32, u32), Vec<String>)> = Vec::new();
            for name in index_names(&html) {
                let Some(date) = parse_date(&name) else {
                    continue;
                };
                if date.0 == year && date.1 == month {
                    match by_date.iter_mut().find(|(d, _)| *d == date) {
                        Some((_, v)) => v.push(name),
                        None => by_date.push((date, vec![name])),
                    }
                }
            }
            for (date, candidates) in by_date {
                let Some(chosen) = pick_latest(&candidates) else {
                    eprintln!(
                        "origin day {:04}{:02}{:02}: no versioned file — skipped",
                        date.0, date.1, date.2
                    );
                    skipped_days.push(date);
                    continue;
                };
                let url = format!("{BASE}/{year}/{month:02}/{chosen}");
                let Some(bytes) = curl_bytes(&url) else {
                    eprintln!("{} void — day {:02} skipped", url, date.2);
                    skipped_days.push(date);
                    continue;
                };
                let out_path = format!("{}/{}", out_dir, xr_name(date));
                if std::fs::write(&out_path, &bytes).is_err() {
                    eprintln!("{} write void", out_path);
                    std::process::exit(1);
                }
                eprintln!(
                    "origin {} -> asset {} (origin verbatim, {} bytes)",
                    chosen,
                    out_path,
                    bytes.len()
                );
                downloaded += 1;
            }
        }
    }
    if downloaded == 0 {
        eprintln!("the corpus stays empty — nothing manifestiert (0 honored)");
        std::process::exit(1);
    }
    if !skipped_days.is_empty() {
        eprintln!(
            "{} origin days carry no reachable file — the corpus keeps its gaps (0 honored)",
            skipped_days.len()
        );
    }
    if has_flag(args, "--ci-mode") {
        for entry in std::fs::read_dir(&out_dir).into_iter().flatten().flatten() {
            let path = entry.path().to_string_lossy().to_string();
            if !path.ends_with(".nc") {
                continue;
            }
            eprintln!("upload {} -> cdn tag {}", path, CDN_TAG);
            if !upload_release(CDN_TAG, &path) {
                eprintln!("upload {} void", path);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--parity") {
        parity_mode(&args);
        return;
    }
    harvest_mode(&args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_date_from_origin_name() {
        assert_eq!(
            parse_date("sci_gxrs-l2-irrad_g15_d20140101_v0-1-0.nc"),
            Some((2014, 1, 1))
        );
        assert_eq!(parse_date("xr_20140101.nc"), None);
        assert_eq!(
            parse_date("sci_gxrs-l2-irrad_g15_d2014x101_v0-1-0.nc"),
            None
        );
    }

    #[test]
    fn parses_version_and_picks_latest() {
        assert_eq!(
            parse_version("sci_gxrs-l2-irrad_g15_d20140101_v0-1-0.nc"),
            Some((0, 1, 0))
        );
        let names = vec![
            "sci_gxrs-l2-irrad_g15_d20140101_v0-1-0.nc".to_string(),
            "sci_gxrs-l2-irrad_g15_d20140101_v2-2-1.nc".to_string(),
        ];
        assert_eq!(
            pick_latest(&names).as_deref(),
            Some("sci_gxrs-l2-irrad_g15_d20140101_v2-2-1.nc")
        );
    }

    #[test]
    fn xr_name_maps_the_day() {
        assert_eq!(xr_name((2014, 1, 1)), "xr_20140101.nc");
    }

    #[test]
    fn index_names_collects_only_the_corpus() {
        let html = "<a href=\"sci_gxrs-l2-irrad_g15_d20140101_v0-1-0.nc\">a</a>
<a href=\"sci_gxrs-l2-irrad_g15_d20140102_v0-1-0.nc\">b</a>
<a href=\"sci_xrsf-l2-avg1m_g15_d20140101_v2-2-1.nc\">c</a>";
        let names = index_names(html);
        assert_eq!(names.len(), 2);
        assert!(names.iter().all(|n| n.starts_with(NAME_PREFIX)));
    }
}
