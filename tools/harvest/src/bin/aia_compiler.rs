use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_asset;
use omegaflow::fits::{FitsCompressedImage, FitsHeader};
use omegaflow::json::{JsonVal, jnum, parse_json};
use omegaflow::lsk::days_from_civil;

const JSOC_FETCH: &str = "http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_fetch";
const JSOC_INFO: &str = "http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_info";
const JSOC_DL: &str = "https://jsoc1.stanford.edu";
const MAGIC: [u8; 4] = *b"AIA1";
const BANDS: [(u32, &str); 7] = [
    (0, "94"),
    (1, "131"),
    (2, "171"),
    (3, "193"),
    (4, "211"),
    (5, "304"),
    (6, "335"),
];
const REQUESTOR: &str = "code@omegaflow.space";

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

fn parse_bands(args: &[String]) -> Vec<u32> {
    let Some(spec) = arg_value(args, "--bands") else {
        return BANDS.iter().map(|(i, _)| *i).collect();
    };
    let mut out = Vec::new();
    for part in spec.split(',') {
        if let Some((i, _)) = BANDS.iter().find(|(_, n)| *n == part.trim()) {
            out.push(*i);
        }
    }
    out
}

fn iso_unix(s: &str) -> Option<f64> {
    let t = s.trim();
    let (date, time) = t.split_once('T')?;
    let mut dparts = date.split('-');
    let year: i64 = dparts.next()?.parse().ok()?;
    let month: i64 = dparts.next()?.parse().ok()?;
    let day: i64 = dparts.next()?.parse().ok()?;
    let time = time.strip_suffix('Z').unwrap_or(time);
    let mut tparts = time.split(':');
    let hour: f64 = tparts.next()?.parse().ok()?;
    let minute: f64 = tparts.next()?.parse().ok()?;
    let sec: f64 = tparts.next().unwrap_or("0").parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + hour * 3600.0 + minute * 60.0 + sec)
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn json_keywords_values<'a>(json: &'a JsonVal, name: &str) -> Option<Vec<Option<f64>>> {
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
        let Some(JsonVal::Arr(vals)) = km.get("values") else {
            return None;
        };
        let mut out = Vec::with_capacity(vals.len());
        for v in vals {
            match v {
                JsonVal::Num(n) => out.push(Some(*n)),
                JsonVal::Str(s) => out.push(s.parse().ok()),
                _ => out.push(None),
            }
        }
        return Some(out);
    }
    None
}

fn json_keywords_strings(json: &JsonVal, name: &str) -> Option<Vec<Option<String>>> {
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
        let Some(JsonVal::Arr(vals)) = km.get("values") else {
            return None;
        };
        let mut out = Vec::with_capacity(vals.len());
        for v in vals {
            match v {
                JsonVal::Str(s) => out.push(Some(s.clone())),
                _ => out.push(None),
            }
        }
        return Some(out);
    }
    None
}

fn export_one(ds: &str, lsk: &LeapSeconds, records: &mut Vec<(f64, f64, u32)>) -> Option<usize> {
    let url = format!(
        "{}?ds={}&op=exp_request&protocol=fits&method=url_quick&compress=rice&requestor={}&notify={}",
        JSOC_FETCH,
        percent_encode(ds),
        REQUESTOR,
        REQUESTOR
    );
    let body = fetch_raw_bytes(&url, 300)?;
    let text = String::from_utf8_lossy(&body).to_string();
    let json = parse_json(&text)?;
    let requestid = match &json {
        JsonVal::Obj(m) => match m.get("requestid") {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => return None,
        },
        _ => return None,
    };
    let mut final_json: Option<JsonVal> = None;
    for _ in 0..60 {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let status_url = format!(
            "{}?op=exp_status&requestid={}&protocol=fits",
            JSOC_FETCH,
            percent_encode(&requestid)
        );
        let body = fetch_raw_bytes(&status_url, 300)?;
        let text = String::from_utf8_lossy(&body).to_string();
        let json = parse_json(&text)?;
        let status = jnum(&json, "status").unwrap_or(-1.0);
        if status == 0.0 {
            final_json = Some(json);
            break;
        }
        if status > 2.0 {
            eprintln!("{}: export status {}", ds, status);
            return None;
        }
    }
    let json = final_json?;
    let dir = match &json {
        JsonVal::Obj(m) => match m.get("dir") {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => return None,
        },
        _ => return None,
    };
    let JsonVal::Obj(m) = &json else { return None };
    let Some(JsonVal::Arr(data)) = m.get("data") else {
        eprintln!("{}: the data array is absent", ds);
        return None;
    };
    let mut kept = 0usize;
    let mut skipped_dl = 0usize;
    let mut skipped_sum = 0usize;
    let mut skipped_meta = 0usize;
    for entry in data {
        let JsonVal::Obj(em) = entry else { continue };
        let Some(JsonVal::Str(fn_)) = em.get("filename") else {
            skipped_meta += 1;
            continue;
        };
        if !fn_.contains("image_lev1") {
            continue;
        }
        let Some(JsonVal::Str(rec)) = em.get("record") else {
            skipped_meta += 1;
            continue;
        };
        let Some(band) = band_of_record(rec) else {
            skipped_meta += 1;
            continue;
        };
        let Some(tobs) = record_time(rec) else {
            skipped_meta += 1;
            continue;
        };
        let Some(unix) = iso_unix(&tobs) else {
            skipped_meta += 1;
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            skipped_meta += 1;
            continue;
        };
        let dl_url = format!("{}{}/{}", JSOC_DL, dir, fn_);
        let Some(bytes) = fetch_raw_bytes(&dl_url, 1800) else {
            skipped_dl += 1;
            continue;
        };
        let Some((sum, datamean, totvals)) = full_disk_verify(&bytes) else {
            skipped_sum += 1;
            continue;
        };
        let frame_mean = sum / totvals.max(1) as f64;
        eprintln!(
            "{} band {}: disk sum {:.6e}, DATAMEAN {:.3}, ratio {:.4}",
            fn_,
            band,
            sum,
            datamean,
            frame_mean / datamean
        );
        records.push((tdb, sum, band));
        kept += 1;
    }
    eprintln!(
        "{}: kept {} (download void {}, sum void {}, meta void {})",
        ds, kept, skipped_dl, skipped_sum, skipped_meta
    );
    Some(kept)
}

fn band_of_record(rec: &str) -> Option<u32> {
    let wl = rec.rsplit("][").next()?.trim_end_matches(']');
    BANDS.iter().find(|(_, n)| *n == wl).map(|(i, _)| *i)
}

fn record_time(rec: &str) -> Option<String> {
    let first = rec.rsplit("][").nth(1)?;
    Some(first.split('[').next_back()?.to_string())
}

fn full_disk_verify(bytes: &[u8]) -> Option<(f64, f64, usize)> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (img, _) = FitsCompressedImage::parse(bytes, off)?;
    if img.dims[0] == 0 || img.dims[1] == 0 {
        return None;
    }
    let r2 = img.r_sun * img.r_sun;
    if !r2.is_finite() || r2 <= 0.0 {
        return None;
    }
    let mut sum = 0.0;
    let mut n = 0usize;
    for y in 0..img.dims[1] {
        let row = img.tile_pixels(bytes, [0, y, 0])?;
        let dy = y as f64 + 1.0 - img.crpix2;
        for x in 0..img.dims[0] {
            let dx = x as f64 + 1.0 - img.crpix1;
            if dx * dx + dy * dy > r2 {
                continue;
            }
            if let Some(v) = img.pixel_value(row[x]) {
                if v.is_finite() {
                    sum += v;
                    n += 1;
                }
            }
        }
    }
    if n == 0 {
        return None;
    }
    Some((sum, img.datamean, img.totvals))
}

fn verify_mode(args: &[String], lsk: &LeapSeconds) {
    let bands = parse_bands(args);
    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    if let Some(path) = arg_value(args, "--file") {
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("{} reads void", path);
            return;
        };
        let Some((sum, datamean, totvals)) = full_disk_verify(&bytes) else {
            eprintln!("{}: the disk sum stays void", path);
            return;
        };
        let frame_mean = sum / totvals.max(1) as f64;
        println!(
            "{}: disk sum {:.6e} DN, DATAMEAN {:.6} DN, TOTVALS {}, sum/TOTVALS {:.6} DN, ratio {:.4}",
            path,
            sum,
            datamean,
            totvals,
            frame_mean,
            frame_mean / datamean
        );
        return;
    }
    let Some(spec) = arg_value(args, "--verify") else {
        return;
    };
    for band in bands {
        let name = BANDS.iter().find(|(i, _)| *i == band).unwrap().1;
        let ds = format!("aia.lev1_euv_12s[{}][{}]", spec, name);
        eprintln!("export {}", ds);
        match export_one(&ds, lsk, &mut records) {
            Some(k) => eprintln!("{}: {} records kept", ds, k),
            None => eprintln!("{}: export void", ds),
        }
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    for (t, v, b) in &records {
        let name = BANDS.iter().find(|(i, _)| i == b).unwrap().1;
        eprintln!("band {} tdb {:.0} disk_sum {:.6e} DN", name, t, v);
    }
}

fn harvest_mode(args: &[String], lsk: &LeapSeconds) {
    let bands = parse_bands(args);
    let out = arg_value(args, "--out").unwrap_or_else(|| "aia_lines.bin".to_string());
    let cache_dir =
        arg_value(args, "--cache-dir").unwrap_or_else(|| "/tmp/omegaflow_aia_cache".to_string());
    let start = arg_value(args, "--start").unwrap_or_else(|| "2014.03.01".to_string());
    let end = arg_value(args, "--end").unwrap_or_else(|| "2014.05.30".to_string());
    let (sy, sm, sd) = parse_civil_date(&start);
    let (ey, em, ed) = parse_civil_date(&end);
    let start_days = days_from_civil(sy, sm, sd).unwrap();
    let end_days = days_from_civil(ey, em, ed).unwrap();
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{}: cache dir stays uncreatable", cache_dir);
        return;
    }
    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    let mut void_chunks = 0usize;
    let chunk_days: i64 = arg_value(args, "--chunk-days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(30);
    let mut chunk_start = start_days;
    while chunk_start <= end_days {
        let chunk_end = (chunk_start + chunk_days - 1).min(end_days);
        let (y, m, d) = civil_from_days(chunk_start);
        let span = chunk_end - chunk_start + 1;
        for band in &bands {
            let name = BANDS.iter().find(|(i, _)| i == band).unwrap().1;
            let ds = format!(
                "aia.lev1_euv_12s[{:04}.{:02}.{:02}_00:00:00_TAI/{}d@12s][{}]",
                y, m, d, span, name
            );
            let cache_path = format!(
                "{}/aia_{}_{:04}{:02}{:02}_{}d.json",
                cache_dir, name, y, m, d, span
            );
            let text = if let Ok(bytes) = std::fs::read(&cache_path) {
                String::from_utf8_lossy(&bytes).to_string()
            } else {
                let url = format!(
                    "{}?ds={}&op=rs_list&key=T_OBS,DATAMEAN,EXPTIME",
                    JSOC_INFO,
                    percent_encode(&ds)
                );
                let Some(bytes) = fetch_raw_bytes(&url, 600) else {
                    eprintln!("{}: fetch void", ds);
                    void_chunks += 1;
                    continue;
                };
                String::from_utf8_lossy(&bytes).to_string()
            };
            let Some(json) = parse_json(&text) else {
                eprintln!("{}: json void", ds);
                void_chunks += 1;
                continue;
            };
            let status = jnum(&json, "status").unwrap_or(-1.0);
            if status != 0.0 {
                eprintln!("{}: status {}", ds, status);
                void_chunks += 1;
                continue;
            }
            if std::fs::metadata(&cache_path).is_err() {
                let _ = std::fs::write(&cache_path, text.as_bytes());
            }
            let Some(times) = json_keywords_strings(&json, "T_OBS") else {
                eprintln!("{}: T_OBS absent", ds);
                void_chunks += 1;
                continue;
            };
            let Some(vals) = json_keywords_values(&json, "DATAMEAN") else {
                eprintln!("{}: DATAMEAN absent", ds);
                void_chunks += 1;
                continue;
            };
            let Some(exps) = json_keywords_values(&json, "EXPTIME") else {
                eprintln!("{}: EXPTIME absent", ds);
                void_chunks += 1;
                continue;
            };
            let mut kept = 0usize;
            for ((t_opt, v_opt), e_opt) in times.iter().zip(vals.iter()).zip(exps.iter()) {
                let (Some(t), Some(v), Some(e)) = (t_opt, v_opt, e_opt) else {
                    continue;
                };
                if !v.is_finite() || *v <= 0.0 || !e.is_finite() || *e <= 0.0 {
                    continue;
                }
                let Some(unix) = iso_unix(t) else {
                    continue;
                };
                let Some(tdb) = lsk.unix_to_tdb(unix) else {
                    continue;
                };
                records.push((tdb, v / e, *band));
                kept += 1;
            }
            eprintln!("{} band {}: {} records", ds, name, kept);
        }
        chunk_start = chunk_end + 1;
    }
    eprintln!(
        "harvest: {} void chunks, {} records",
        void_chunks,
        records.len()
    );
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        eprintln!("{}: no records — the bin stays unwritten (0 honored)", out);
        return;
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, idx) in &records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&idx.to_le_bytes());
    }
    if std::fs::write(&out, &buf).is_err() {
        eprintln!("write {} returned void", out);
        return;
    }
    let (t0, _, _) = records[0];
    let (t1, _, _) = records[records.len() - 1];
    eprintln!(
        "{}: {} records ({} B), epoch_tdb {:.0}..{:.0}",
        out,
        records.len(),
        buf.len(),
        t0,
        t1
    );
    if has_flag(args, "--ci-mode") && !upload_asset(&out) {
        std::process::exit(1);
    }
}

fn parse_civil_date(s: &str) -> (i64, i64, i64) {
    let mut p = s.split('.');
    (
        p.next().and_then(|v| v.parse().ok()).unwrap_or(2014),
        p.next().and_then(|v| v.parse().ok()).unwrap_or(3),
        p.next().and_then(|v| v.parse().ok()).unwrap_or(1),
    )
}

fn merge_mode(args: &[String]) {
    let out = arg_value(args, "--out").unwrap_or_else(|| "aia_lines.bin".to_string());
    let Some(spec) = arg_value(args, "--merge") else {
        return;
    };
    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    for part in spec.split(',') {
        let path = part.trim();
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("{} reads void", path);
            continue;
        };
        if bytes.len() < 8 || bytes[0..4] != MAGIC {
            eprintln!("{} carries no AIA1 contract", path);
            continue;
        }
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        for i in 0..n {
            let o = 8 + i * 20;
            let Some(t) = bytes
                .get(o..o + 8)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
            else {
                continue;
            };
            let Some(v) = bytes
                .get(o + 8..o + 16)
                .and_then(|b| b.try_into().ok())
                .map(f64::from_le_bytes)
            else {
                continue;
            };
            let Some(idx) = bytes
                .get(o + 16..o + 20)
                .and_then(|b| b.try_into().ok())
                .map(u32::from_le_bytes)
            else {
                continue;
            };
            records.push((t, v, idx));
        }
        eprintln!("{}: {} records", path, n);
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        eprintln!("{}: no records — the bin stays unwritten (0 honored)", out);
        return;
    }
    let mut buf = Vec::with_capacity(8 + records.len() * 20);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for (t, v, idx) in &records {
        buf.extend_from_slice(&t.to_le_bytes());
        buf.extend_from_slice(&v.to_le_bytes());
        buf.extend_from_slice(&idx.to_le_bytes());
    }
    if std::fs::write(&out, &buf).is_err() {
        eprintln!("write {} returned void", out);
        return;
    }
    let (t0, _, _) = records[0];
    let (t1, _, _) = records[records.len() - 1];
    eprintln!(
        "{}: {} records ({} B), epoch_tdb {:.0}..{:.0}",
        out,
        records.len(),
        buf.len(),
        t0,
        t1
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(lsk) = omegaflow::archivar::embedded_lsk() else {
        eprintln!("the time base is absent");
        std::process::exit(1);
    };
    if has_flag(&args, "--merge") {
        merge_mode(&args);
        return;
    }
    if has_flag(&args, "--harvest") {
        harvest_mode(&args, &lsk);
        return;
    }
    if has_flag(&args, "--verify") || has_flag(&args, "--file") {
        verify_mode(&args, &lsk);
        return;
    }
    eprintln!(
        "usage: --harvest --start 2014.03.01 --end 2014.05.30 [--bands 94,131,171,193,211,304,335] [--out aia_lines.bin] | --verify <startTAI/dur> | --file <image_lev1.fits>"
    );
}
