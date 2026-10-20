use omegaflow::archivar::hmi::MAGIC;
use omegaflow::archivar::membrane::embedded_lsk;
use omegaflow::archivar::{JsonVal, parse_json};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{LeapSeconds, days_from_civil};
use std::process::Command;

const TAG: &str = "jsoc.stanford.edu";
const BASE: &str = "https://jsoc1.stanford.edu/data/hmi/fits";
const HEADER_BYTES: usize = 20000;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str, range: Option<&str>) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sSfL").arg("-m").arg("180");
    if let Some(r) = range {
        cmd.arg("-r").arg(r);
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn fits_value(text: &str, key: &str) -> Option<String> {
    let i = text.find(key)?;
    let after = &text[i + key.len()..];
    let eq = after.find('=')?;
    if eq > 14 {
        return None;
    }
    let value = after[eq + 1..].trim_start();
    let token = value.split_whitespace().next()?;
    Some(token.trim_matches('\'').to_string())
}

fn parse_day(s: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<&str> = if s.contains('-') {
        s.split('-').collect()
    } else if s.len() == 8 {
        vec![&s[0..4], &s[4..6], &s[6..8]]
    } else {
        return None;
    };
    if parts.len() != 3 {
        return None;
    }
    Some((
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
    ))
}

fn next_day(y: i64, m: i64, d: i64) -> (i64, i64, i64) {
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let dim = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 31,
    };
    if d < dim {
        (y, m, d + 1)
    } else if m < 12 {
        (y, m + 1, 1)
    } else {
        (y + 1, 1, 1)
    }
}

fn gauss_to_nt(gauss: f64) -> f64 {
    gauss * 100_000.0
}

fn tai_to_tdb(tai: f64, lsk: &LeapSeconds) -> Option<f64> {
    let mut utc = tai - 37.0;
    for _ in 0..3 {
        utc = tai - lsk.leap_at(utc)?;
    }
    lsk.unix_to_tdb(utc)
}

fn parse_t_rec(s: &str, lsk: &LeapSeconds) -> Option<f64> {
    let s = s.trim_end_matches("_TAI");
    let (d, t) = s.split_once('_')?;
    let dp: Vec<&str> = d.split('.').collect();
    let tp: Vec<&str> = t.split(':').collect();
    if dp.len() != 3 || tp.len() != 3 {
        return None;
    }
    let y: i64 = dp[0].parse().ok()?;
    let mo: i64 = dp[1].parse().ok()?;
    let da: i64 = dp[2].parse().ok()?;
    let hh: f64 = tp[0].parse().ok()?;
    let mm: f64 = tp[1].parse().ok()?;
    let ss: f64 = tp[2].parse().ok()?;
    let days = days_from_civil(y, mo, da)?;
    let tai = days as f64 * 86400.0 + hh * 3600.0 + mm * 60.0 + ss;
    tai_to_tdb(tai, lsk)
}

fn fits_names(html: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut rest = html;
    while let Some(i) = rest.find("hmi.M_720s.") {
        let tail = &rest[i..];
        let Some(j) = tail.find(".fits") else {
            break;
        };
        let name = &tail[..j + 5];
        if !names.iter().any(|n| n == name) {
            names.push(name.to_string());
        }
        rest = &tail[j + 5..];
    }
    names
}

fn rs_list_year(start: &str) -> Option<JsonVal> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("300")
        .arg("-G")
        .arg("--data-urlencode")
        .arg(format!("ds=hmi.M_720s[{start}/367d@720s]"))
        .arg("--data-urlencode")
        .arg("op=rs_list")
        .arg("--data-urlencode")
        .arg("key=DATAMEAN,T_REC,CAMERA")
        .arg("http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_info")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    parse_json(&String::from_utf8_lossy(&out.stdout))
}

fn keyword_values(root: &JsonVal, name: &str) -> Option<Vec<String>> {
    let JsonVal::Obj(obj) = root else {
        return None;
    };
    let JsonVal::Arr(keywords) = obj.get("keywords")? else {
        return None;
    };
    for kw in keywords {
        let JsonVal::Obj(entry) = kw else {
            continue;
        };
        let Some(JsonVal::Str(found)) = entry.get("name") else {
            continue;
        };
        if found != name {
            continue;
        }
        let JsonVal::Arr(values) = entry.get("values")? else {
            return None;
        };
        return Some(
            values
                .iter()
                .filter_map(|v| match v {
                    JsonVal::Str(s) => Some(s.clone()),
                    JsonVal::Num(n) => Some(n.to_string()),
                    _ => None,
                })
                .collect(),
        );
    }
    None
}

fn write_hmi(
    records: &mut Vec<(f64, f64, u32)>,
    out_path: &str,
    ci_mode: bool,
) -> Result<(), String> {
    records.sort_by(|a, b| a.0.total_cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    if records.is_empty() {
        return Err("no DATAMEAN record — the bin stays unwritten (0 honored)".into());
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, c) in records.iter() {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
    }
    std::fs::write(out_path, &buf).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    eprintln!(
        "hmi_compiler: {} records ({} B) -> {out_path}",
        records.len(),
        buf.len()
    );
    if ci_mode && !upload_release(TAG, out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn run_rs_list(args: &[String]) -> Result<(), String> {
    let from = arg_value(args, "--from").ok_or("--from YYYY-MM-DD absent — refused")?;
    let to = arg_value(args, "--to").ok_or("--to YYYY-MM-DD absent — refused")?;
    let out_path = arg_value(args, "--out").ok_or("--out <hmi.bin> absent — refused")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lsk = embedded_lsk()
        .ok_or("the embedded leap-second table stays unread — T_REC stays unassigned")?;
    let (fy, fm, fd) = parse_day(&from).ok_or(format!("{from}: not a day — refused"))?;
    let (ty, tm, td) = parse_day(&to).ok_or(format!("{to}: not a day — refused"))?;
    let t_lo = parse_t_rec(&format!("{fy:04}.{fm:02}.{fd:02}_00:00:00_TAI"), &lsk)
        .ok_or("--from stays unassigned")?;
    let t_hi = parse_t_rec(&format!("{ty:04}.{tm:02}.{td:02}_23:59:59_TAI"), &lsk)
        .ok_or("--to stays unassigned")?;

    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    for year in fy..=ty {
        let start = if year == fy {
            format!("{fy:04}.{fm:02}.{fd:02}")
        } else {
            format!("{year}.01.01")
        };
        let Some(root) = rs_list_year(&start) else {
            eprintln!("{year}: rs_list carries void");
            continue;
        };
        let (Some(values), Some(t_recs), Some(cameras)) = (
            keyword_values(&root, "DATAMEAN"),
            keyword_values(&root, "T_REC"),
            keyword_values(&root, "CAMERA"),
        ) else {
            eprintln!("{year}: DATAMEAN/T_REC/CAMERA absent");
            continue;
        };
        let rows = values.len().min(t_recs.len()).min(cameras.len());
        let mut kept = 0usize;
        for i in 0..rows {
            let Ok(value) = values[i].parse::<f64>() else {
                continue;
            };
            if !value.is_finite() {
                continue;
            }
            let Some(t) = parse_t_rec(&t_recs[i], &lsk) else {
                continue;
            };
            let Ok(camera) = cameras[i].parse::<u32>() else {
                continue;
            };
            if !(1..=3).contains(&camera) || t < t_lo || t > t_hi {
                continue;
            }
            records.push((t, gauss_to_nt(value), camera));
            kept += 1;
        }
        eprintln!("{year}: {rows} rows, {kept} kept");
    }
    write_hmi(&mut records, &out_path, ci_mode)
}

fn run_fits(args: &[String]) -> Result<(), String> {
    let day = arg_value(args, "--day").ok_or(
        "usage: hmi_compiler --day YYYY-MM-DD [--days N] --out <hmi.bin> [--ci-mode] — refused",
    )?;
    let out_path = arg_value(args, "--out").ok_or("--out <hmi.bin> absent — refused")?;
    let span: usize = arg_value(args, "--days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lsk = embedded_lsk()
        .ok_or("the embedded leap-second table stays unread — T_REC stays unassigned")?;
    let (mut y, mut m, mut d) =
        parse_day(&day).ok_or(format!("{day}: not a day (YYYY-MM-DD) — refused"))?;
    if span == 0 {
        return Err("--days 0 carries no window — refused".into());
    }

    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    for _ in 0..span {
        let dir = format!("{BASE}/{y:04}/{m:02}/{d:02}/");
        let html = fetch(&dir, None).ok_or(format!("{dir} carries void"))?;
        let text = String::from_utf8_lossy(&html);
        let names = fits_names(&text);
        let mut kept = 0usize;
        for name in &names {
            let url = format!("{dir}{name}");
            let Some(buf) = fetch(&url, Some(&format!("0-{HEADER_BYTES}"))) else {
                continue;
            };
            let head = String::from_utf8_lossy(&buf);
            let Some(raw) = fits_value(&head, "DATAMEAN") else {
                continue;
            };
            let Ok(value) = raw.parse::<f64>() else {
                continue;
            };
            if !value.is_finite() {
                continue;
            }
            let Some(t_rec) = fits_value(&head, "T_REC") else {
                continue;
            };
            let Some(t) = parse_t_rec(&t_rec, &lsk) else {
                continue;
            };
            let Some(camera) = fits_value(&head, "CAMERA").and_then(|c| c.parse::<u32>().ok())
            else {
                continue;
            };
            if !(1..=3).contains(&camera) {
                continue;
            }
            records.push((t, gauss_to_nt(value), camera));
            kept += 1;
        }
        eprintln!(
            "{y:04}-{m:02}-{d:02}: {} files, {kept} records",
            names.len()
        );
        (y, m, d) = next_day(y, m, d);
    }
    write_hmi(&mut records, &out_path, ci_mode)
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--from") {
        run_rs_list(args)
    } else {
        run_fits(args)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("hmi_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_value_reads_padded_and_quoted_cards() {
        let head = "CAMERA  =                    3\nDATAMEAN= -0.108827\nT_REC   = '2020.01.01_00:00:00_TAI'";
        assert_eq!(fits_value(head, "CAMERA").unwrap(), "3");
        assert_eq!(fits_value(head, "DATAMEAN").unwrap(), "-0.108827");
        assert_eq!(
            fits_value(head, "T_REC").unwrap(),
            "2020.01.01_00:00:00_TAI"
        );
    }

    #[test]
    fn missing_datamean_stays_unparsed() {
        assert!(
            fits_value("DATAMEAN= MISSING", "DATAMEAN")
                .unwrap()
                .parse::<f64>()
                .is_err()
        );
    }

    #[test]
    fn t_rec_is_tai_and_lands_near_the_2020_epoch() {
        let lsk = embedded_lsk().unwrap();
        let tdb = parse_t_rec("2020.01.01_00:00:00_TAI", &lsk).unwrap();
        assert!((tdb - 631108832.184).abs() < 0.01, "tdb={tdb}");
    }

    #[test]
    fn datamean_gauss_lands_in_nanotesla() {
        assert!((gauss_to_nt(-0.108827) + 10882.7).abs() < 1e-9);
        assert!((gauss_to_nt(0.21) - 21000.0).abs() < 1e-9);
    }

    #[test]
    fn keyword_values_reads_the_three_arrays() {
        let s = r#"{"keywords":[{"name":"DATAMEAN","values":["-0.1","MISSING"]},{"name":"T_REC","values":["2020.01.01_00:00:00_TAI","2020.01.01_00:12:00_TAI"]},{"name":"CAMERA","values":["3","3"]}]}"#;
        let root = parse_json(s).unwrap();
        assert_eq!(keyword_values(&root, "DATAMEAN").unwrap().len(), 2);
        assert_eq!(keyword_values(&root, "CAMERA").unwrap()[0], "3");
        assert!(keyword_values(&root, "NOPE").is_none());
    }

    #[test]
    fn day_rollover_handles_leap_and_year_end() {
        assert_eq!(next_day(2020, 2, 28), (2020, 2, 29));
        assert_eq!(next_day(2021, 2, 28), (2021, 3, 1));
        assert_eq!(next_day(2020, 12, 31), (2021, 1, 1));
    }

    #[test]
    fn names_are_deduped_from_the_directory_html() {
        let html = "<a href=\"hmi.M_720s.20200101_000000_TAI.fits\">x</a>\
                    <a href=\"hmi.M_720s.20200101_010000_TAI.fits\">y</a>\
                    <a href=\"hmi.M_720s.20200101_000000_TAI.fits\">x</a>";
        let n = fits_names(html);
        assert_eq!(n.len(), 2);
        assert_eq!(n[0], "hmi.M_720s.20200101_000000_TAI.fits");
    }
}
