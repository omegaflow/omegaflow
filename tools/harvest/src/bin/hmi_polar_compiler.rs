use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_asset;
use omegaflow::fits::{FitsCompressedImage, FitsHeader};
use omegaflow::hmi_polar::{parse_bin, write_bin};
use omegaflow::json::{jnum, parse_json, JsonVal};
use omegaflow::lsk::days_from_civil;

const JSOC_INFO: &str = "http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_info";
const JSOC_FETCH: &str = "http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_fetch";
const JSOC_DL: &str = "https://jsoc1.stanford.edu";
const SERIES: &str = "hmi.synoptic_mr_polfil_720s";
const REQUESTOR: &str = "code@omegaflow.space";
const CAP_SIN: f64 = 0.866_025_403_784_438_6;
const BATCH: usize = 32;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn keyword_strs(json: &JsonVal, name: &str) -> Option<Vec<Option<String>>> {
    let JsonVal::Obj(map) = json else { return None };
    let JsonVal::Arr(kws) = map.get("keywords")? else {
        return None;
    };
    for kw in kws {
        let JsonVal::Obj(km) = kw else { continue };
        if let Some(JsonVal::Str(n)) = km.get("name") {
            if n != name {
                continue;
            }
        } else {
            continue;
        }
        let JsonVal::Arr(vals) = km.get("values")? else {
            return None;
        };
        let mut out = Vec::with_capacity(vals.len());
        for v in vals {
            out.push(match v {
                JsonVal::Str(s) => Some(s.clone()),
                JsonVal::Num(x) => Some(format!("{}", x)),
                _ => None,
            });
        }
        return Some(out);
    }
    None
}

fn obj_str<'a>(json: &'a JsonVal, name: &str) -> Option<&'a str> {
    let JsonVal::Obj(map) = json else { return None };
    match map.get(name) {
        Some(JsonVal::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}

fn parse_tobs(s: &str) -> Option<i64> {
    let date = s.split('_').next()?;
    let mut parts = date.split('.');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    days_from_civil(year, month, day)
}

fn fetch_json(url: &str) -> Option<JsonVal> {
    let bytes = fetch_raw_bytes(url, 600)?;
    let text = String::from_utf8_lossy(&bytes).to_string();
    parse_json(&text)
}

fn list_rotations() -> Option<Vec<(i64, String)>> {
    let url = format!(
        "{}?ds={}&op=rs_list&key=CAR_ROT,T_OBS",
        JSOC_INFO,
        percent_encode(SERIES)
    );
    let json = fetch_json(&url)?;
    let rots = keyword_strs(&json, "CAR_ROT")?;
    let tobs = keyword_strs(&json, "T_OBS")?;
    let mut out = Vec::new();
    for (r, t) in rots.iter().zip(tobs.iter()) {
        let (Some(rs), Some(ts)) = (r, t) else {
            continue;
        };
        let Some(car_rot) = rs.parse::<i64>().ok() else {
            continue;
        };
        out.push((car_rot, ts.clone()));
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn record_car_rot(rec: &str) -> Option<i64> {
    let inner = rec.rsplit('[').next()?.trim_end_matches(']');
    inner.parse().ok()
}

fn export_chunk(
    first: i64,
    last: i64,
    cache_dir: &str,
    jobs: usize,
) -> Vec<(i64, Option<Vec<u8>>)> {
    let ds = format!("{}[{first}-{last}]", SERIES);
    let req_url = format!(
        "{}?ds={}&op=exp_request&protocol=fits&method=url_quick&compress=rice&requestor={}&notify={}",
        JSOC_FETCH,
        percent_encode(&ds),
        REQUESTOR,
        REQUESTOR
    );
    let Some(json) = fetch_json(&req_url) else {
        eprintln!("{ds}: exp_request void");
        return (first..=last).map(|r| (r, None)).collect();
    };
    let Some(requestid) = obj_str(&json, "requestid").map(|s| s.to_string()) else {
        eprintln!("{ds}: requestid absent");
        return (first..=last).map(|r| (r, None)).collect();
    };
    let mut final_json: Option<JsonVal> = None;
    for _ in 0..120 {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let status_url = format!(
            "{}?op=exp_status&requestid={}&protocol=fits",
            JSOC_FETCH,
            percent_encode(&requestid)
        );
        let Some(sjson) = fetch_json(&status_url) else {
            continue;
        };
        let status = jnum(&sjson, "status").unwrap_or(-1.0);
        if status == 0.0 {
            final_json = Some(sjson);
            break;
        }
        if status > 2.0 {
            eprintln!("{ds}: export status {}", status);
            return (first..=last).map(|r| (r, None)).collect();
        }
    }
    let Some(json) = final_json else {
        eprintln!("{ds}: export timed out");
        return (first..=last).map(|r| (r, None)).collect();
    };
    let Some(dir) = obj_str(&json, "dir").map(|s| s.to_string()) else {
        eprintln!("{ds}: dir absent");
        return (first..=last).map(|r| (r, None)).collect();
    };
    let JsonVal::Obj(map) = &json else {
        return (first..=last).map(|r| (r, None)).collect();
    };
    let Some(JsonVal::Arr(data)) = map.get("data") else {
        eprintln!("{ds}: data array absent");
        return (first..=last).map(|r| (r, None)).collect();
    };
    let mut tasks: Vec<(i64, String)> = Vec::new();
    for entry in data {
        let JsonVal::Obj(_) = entry else { continue };
        let Some(rec) = obj_str(entry, "record") else {
            continue;
        };
        let Some(car_rot) = record_car_rot(rec) else {
            continue;
        };
        let Some(fname) = obj_str(entry, "filename").map(|s| s.to_string()) else {
            continue;
        };
        tasks.push((car_rot, fname));
    }
    let dl_base = format!("{}{}/", JSOC_DL, dir);
    let jobs = jobs.max(1);
    let per = tasks.len().div_ceil(jobs).max(1);
    let cache_owned = cache_dir.to_string();
    let mut out: Vec<(i64, Option<Vec<u8>>)> = Vec::with_capacity(tasks.len());
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in tasks.chunks(per) {
            let dl_base = &dl_base;
            let cache_owned = &cache_owned;
            handles.push(scope.spawn(move || {
                let mut local = Vec::with_capacity(chunk.len());
                for (car_rot, fname) in chunk {
                    let cache_path = format!("{}/{}.fits", cache_owned, car_rot);
                    let bytes = if let Ok(b) = std::fs::read(&cache_path) {
                        Some(b)
                    } else {
                        let dl_url = format!("{}{}", dl_base, fname);
                        let got = fetch_raw_bytes(&dl_url, 1800);
                        if let Some(b) = &got {
                            let _ = std::fs::write(&cache_path, b);
                        }
                        got
                    };
                    local.push((*car_rot, bytes));
                }
                local
            }));
        }
        for h in handles {
            out.extend(h.join().unwrap_or_default());
        }
    });
    out
}

fn polar_means(bytes: &[u8]) -> Option<(f64, f64)> {
    let (_, hdu2) = FitsHeader::parse(bytes, 0)?;
    let (header, _) = FitsHeader::parse(bytes, hdu2)?;
    let bscale = header.f64("BSCALE").unwrap_or(1.0);
    let bzero = header.f64("BZERO").unwrap_or(0.0);
    let cdelt2 = header.f64("CDELT2")?;
    let crval2 = header.f64("CRVAL2").unwrap_or(0.0);
    let crpix2 = header.f64("CRPIX2")?;
    let (img, _) = FitsCompressedImage::parse(bytes, hdu2)?;
    let blank = img.blank;
    let mut north_sum = 0.0f64;
    let mut north_n = 0usize;
    let mut south_sum = 0.0f64;
    let mut south_n = 0usize;
    for y in 0..img.dims[1] {
        let sin_lat = crval2 + (y as f64 + 1.0 - crpix2) * cdelt2;
        let north = sin_lat >= CAP_SIN;
        let south = sin_lat <= -CAP_SIN;
        if !north && !south {
            continue;
        }
        let Some(row) = img.tile_pixels(bytes, [0, y, 0]) else {
            continue;
        };
        for raw in &row {
            if let Some(b) = blank {
                if *raw == b {
                    continue;
                }
            }
            let gauss = *raw as f64 * bscale + bzero;
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
    let tesla = 1e-4;
    Some((
        north_sum / north_n as f64 * tesla,
        south_sum / south_n as f64 * tesla,
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "hmi_polar.bin".to_string());
    let cache_dir = arg_value(&args, "--cache-dir").unwrap_or_else(|| {
        omegaflow::archivar::cache_root()
            .join("omegaflow_hmi_cache")
            .to_string_lossy()
            .into_owned()
    });
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{}: cache dir stays uncreatable", cache_dir);
        std::process::exit(1);
    }

    let Some(mut rots) = list_rotations() else {
        eprintln!("rs_list void — the series stays unwritten (0 honored)");
        std::process::exit(1);
    };
    if let Some(limit) = arg_value(&args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        rots.truncate(limit);
    }
    eprintln!("{}: {} Carrington-Rotationen", SERIES, rots.len());

    let mut records: Vec<(i64, f64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    let mut by_rot: std::collections::HashMap<i64, &str> = std::collections::HashMap::new();
    for (r, t) in &rots {
        by_rot.insert(*r, t.as_str());
    }
    let mut chunk_start = 0usize;
    while chunk_start < rots.len() {
        let chunk: Vec<(i64, String)> = rots[chunk_start..(chunk_start + BATCH).min(rots.len())]
            .iter()
            .cloned()
            .collect();
        let first = chunk.first().unwrap().0;
        let last = chunk.last().unwrap().0;
        eprintln!("export {first}..{last}");
        for (car_rot, bytes) in export_chunk(first, last, &cache_dir, jobs) {
            let Some(tobs) = by_rot.get(&car_rot).cloned() else {
                skipped += 1;
                continue;
            };
            let Some(bytes) = bytes else {
                skipped += 1;
                continue;
            };
            let Some(days) = parse_tobs(tobs) else {
                skipped += 1;
                continue;
            };
            let Some((north, south)) = polar_means(&bytes) else {
                skipped += 1;
                continue;
            };
            let avg = (north - south) / 2.0;
            records.push((days, north, south, avg));
        }
        chunk_start += BATCH;
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0);
    eprintln!(
        "HMI polar field: {} records, {} skipped (absent)",
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
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
