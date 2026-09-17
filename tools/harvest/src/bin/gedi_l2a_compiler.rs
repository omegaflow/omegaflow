use omegaflow::archivar::range::{
    S3_ENDPOINT, S3_REGION, S3Credentials, Sigv4Args, edl_s3_credentials_for, fetch_s3_range,
    sigv4_headers,
};
use omegaflow::archivar::{LeapSeconds, embedded_lsk};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{
    Endian, Hdf5Datatype, Hdf5File, Hdf5Layout, Hdf5Object, decode_f32, decode_f64,
};
use omegaflow::lsk::days_from_civil;
use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "data.lpdaac.earthdatacloud.nasa.gov";
const BUCKET: &str = "lp-prod-protected";
const PRODUCT_ROOT: &str = "GEDI02_A.002/";
const DEFAULT_OUT: &str = "data/data.lpdaac.earthdatacloud.nasa.gov/gedi_l2a.bin";
const MAGIC: [u8; 4] = *b"GED1";
const REC_FIELDS: usize = 8;
const REC_BYTES: usize = REC_FIELDS * 8;
const META_WINDOW: u64 = 1 << 25;
const META_ESCALATION: u64 = 3 * (1 << 25);
const REC_CAP: usize = 1 << 13;
const LIST_MAX_KEYS: u32 = 20;
const LIST_MAX_T_S: u64 = 1 << 7;
const CONNECT_BOUND_S: u64 = 1 << 5;
const BEAMS: [&str; 8] = [
    "BEAM0000", "BEAM0001", "BEAM0010", "BEAM0011", "BEAM0100", "BEAM0101", "BEAM0110", "BEAM1011",
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name).and_then(|v| v.parse::<usize>().ok())
}

fn parse_secret(text: &str, key: &str) -> Option<String> {
    let mut found = None;
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                found = Some(v.trim().to_string());
            }
        }
    }
    found
}

fn edl_token() -> Option<String> {
    if let Ok(t) = env::var("EARTHDATA_EDL_TOKEN") {
        if !t.trim().is_empty() {
            return Some(t.trim().to_string());
        }
    }
    let text = fs::read_to_string(".secrets.local").ok()?;
    parse_secret(&text, "EARTHDATA_EDL_TOKEN")
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn amz_now() -> Option<(String, String)> {
    let unix = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let days = (unix / 86_400) as i64;
    let secs = unix % 86_400;
    let (y, m, d) = civil_from_days(days);
    let date_stamp = format!("{y:04}{m:02}{d:02}");
    let h = secs / 3600;
    let mi = (secs % 3600) / 60;
    let s = secs % 60;
    Some((
        date_stamp.clone(),
        format!("{date_stamp}T{h:02}{mi:02}{s:02}Z"),
    ))
}

fn gedi_granule_start_unix(key: &str) -> Option<f64> {
    let file = key.rsplit('/').next()?;
    let digits = file.strip_prefix("GEDI02_A_")?.get(0..13)?;
    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year: i64 = digits.get(0..4)?.parse().ok()?;
    let doy: i64 = digits.get(4..7)?.parse().ok()?;
    let hh: i64 = digits.get(7..9)?.parse().ok()?;
    let mm: i64 = digits.get(9..11)?.parse().ok()?;
    let ss: i64 = digits.get(11..13)?.parse().ok()?;
    if !(1..=366).contains(&doy) || !(0..=23).contains(&hh) || !(0..=59).contains(&mm) {
        return None;
    }
    if !(0..=60).contains(&ss) {
        return None;
    }
    let day0 = days_from_civil(year, 1, 1)?;
    Some((day0 + doy - 1) as f64 * 86400.0 + (hh * 3600 + mm * 60 + ss) as f64)
}

fn granule_anchor_tdb(key: &str, lsk: &LeapSeconds) -> Option<f64> {
    lsk.unix_to_tdb(gedi_granule_start_unix(key)?)
}

fn gedi_day_prefix(day: &str) -> Option<String> {
    let year: i64 = day.get(0..4)?.parse().ok()?;
    let month: i64 = day.get(5..7)?.parse().ok()?;
    let dom: i64 = day.get(8..10)?.parse().ok()?;
    let doy = days_from_civil(year, month, dom)? - days_from_civil(year, 1, 1)? + 1;
    if !(1..=366).contains(&doy) {
        return None;
    }
    Some(format!("GEDI02_A_{year:04}{doy:03}"))
}

fn uri_encode_query(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

struct Obj {
    key: String,
    size: u64,
    modified: String,
}

fn xml_text(doc: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let start = doc.find(&open)? + open.len();
    let close = format!("</{tag}>");
    let end = doc[start..].find(&close)? + start;
    Some(doc[start..end].to_string())
}

fn parse_page(body: &str) -> Option<(Vec<Obj>, bool, Vec<String>)> {
    let mut objects = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("<Contents>") {
        let after_open = &rest[start + "<Contents>".len()..];
        let end = after_open.find("</Contents>")?;
        let block = &after_open[..end];
        let key = xml_text(block, "Key")?;
        let size = xml_text(block, "Size")?.parse::<u64>().ok()?;
        let modified = xml_text(block, "LastModified")?;
        objects.push(Obj {
            key,
            size,
            modified,
        });
        rest = &after_open[end + "</Contents>".len()..];
    }
    let mut dirs = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("<CommonPrefixes>") {
        let after_open = &rest[start + "<CommonPrefixes>".len()..];
        let end = after_open.find("</CommonPrefixes>")?;
        let block = &after_open[..end];
        if let Some(p) = xml_text(block, "Prefix") {
            dirs.push(p);
        }
        rest = &after_open[end + "</CommonPrefixes>".len()..];
    }
    let truncated = xml_text(body, "IsTruncated").as_deref() == Some("true");
    Some((objects, truncated, dirs))
}

fn list_page(
    bucket: &str,
    prefix: &str,
    dirs: bool,
    max_keys: u32,
    creds: &S3Credentials,
) -> Option<(Vec<Obj>, bool, Vec<String>)> {
    let mut params: Vec<(String, String)> = vec![
        ("list-type".to_string(), "2".to_string()),
        ("max-keys".to_string(), max_keys.to_string()),
    ];
    if !prefix.is_empty() {
        params.push(("prefix".to_string(), prefix.to_string()));
    }
    if dirs {
        params.push(("delimiter".to_string(), "/".to_string()));
    }
    params.sort_by(|a, b| a.0.cmp(&b.0));
    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", uri_encode_query(k), uri_encode_query(v)))
        .collect::<Vec<_>>()
        .join("&");
    let canonical_uri = format!("/{bucket}");
    let (date_stamp, amz_date) = amz_now()?;
    let headers = sigv4_headers(&Sigv4Args {
        access_key: &creds.access_key,
        secret_key: &creds.secret_key,
        region: S3_REGION,
        host: S3_ENDPOINT,
        canonical_uri: &canonical_uri,
        canonical_query: &query,
        range: None,
        amz_date: &amz_date,
        date_stamp: &date_stamp,
        session_token: Some(&creds.session_token),
    });
    let url = format!("https://{S3_ENDPOINT}{canonical_uri}?{query}");
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(LIST_MAX_T_S.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string());
    for (k, v) in &headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    cmd.arg(&url);
    let out = cmd.output().ok()?;
    if !out.status.success() {
        eprintln!(
            "list returned ({}): {} {}",
            out.status,
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    parse_page(&String::from_utf8_lossy(&out.stdout))
}

fn attr_number(obj: &Hdf5Object, name: &str) -> Option<f64> {
    let a = obj.attrs.iter().find(|a| a.name == name)?;
    match a.datatype.class {
        1 => match a.datatype.size {
            4 => decode_f32(&a.data, 0, a.datatype.endian).map(|v| v as f64),
            8 => decode_f64(&a.data, 0, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
}

fn apply_scale_offset(obj: &Hdf5Object, values: &mut [f64]) {
    if let Some(scale) = attr_number(obj, "scale_factor") {
        let offset = attr_number(obj, "add_offset");
        for v in values.iter_mut() {
            *v = match offset {
                Some(o) => *v * scale + o,
                None => *v * scale,
            };
        }
    }
}

fn decode_value(raw: &[u8], i: usize, dt: &Hdf5Datatype) -> Option<f64> {
    let sz = dt.size;
    if sz == 0 {
        return None;
    }
    let off = i.checked_mul(sz)?;
    let b = raw.get(off..off + sz)?;
    let le = dt.endian == Endian::Le;
    match dt.class {
        0 => {
            if dt.signed {
                match sz {
                    1 => Some(b[0] as i8 as f64),
                    2 => {
                        let a: [u8; 2] = b.try_into().ok()?;
                        let v = if le {
                            i16::from_le_bytes(a)
                        } else {
                            i16::from_be_bytes(a)
                        };
                        Some(v as f64)
                    }
                    4 => {
                        let a: [u8; 4] = b.try_into().ok()?;
                        let v = if le {
                            i32::from_le_bytes(a)
                        } else {
                            i32::from_be_bytes(a)
                        };
                        Some(v as f64)
                    }
                    8 => {
                        let a: [u8; 8] = b.try_into().ok()?;
                        let v = if le {
                            i64::from_le_bytes(a)
                        } else {
                            i64::from_be_bytes(a)
                        };
                        Some(v as f64)
                    }
                    _ => None,
                }
            } else {
                match sz {
                    1 => Some(b[0] as f64),
                    2 => {
                        let a: [u8; 2] = b.try_into().ok()?;
                        let v = if le {
                            u16::from_le_bytes(a)
                        } else {
                            u16::from_be_bytes(a)
                        };
                        Some(v as f64)
                    }
                    4 => {
                        let a: [u8; 4] = b.try_into().ok()?;
                        let v = if le {
                            u32::from_le_bytes(a)
                        } else {
                            u32::from_be_bytes(a)
                        };
                        Some(v as f64)
                    }
                    8 => {
                        let a: [u8; 8] = b.try_into().ok()?;
                        let v = if le {
                            u64::from_le_bytes(a)
                        } else {
                            u64::from_be_bytes(a)
                        };
                        Some(v as f64)
                    }
                    _ => None,
                }
            }
        }
        1 => match sz {
            4 => {
                let a: [u8; 4] = b.try_into().ok()?;
                let v = if le {
                    f32::from_le_bytes(a)
                } else {
                    f32::from_be_bytes(a)
                };
                Some(v as f64)
            }
            8 => {
                let a: [u8; 8] = b.try_into().ok()?;
                Some(if le {
                    f64::from_le_bytes(a)
                } else {
                    f64::from_be_bytes(a)
                })
            }
            _ => None,
        },
        _ => None,
    }
}

fn first_values(
    file: &Hdf5File,
    s3_url: &str,
    creds: &S3Credentials,
    path: &str,
) -> Option<Vec<f64>> {
    let (obj, ds, dt) = file.dataset(path).ok()?;
    if dt.class == 9 || dt.class == 3 {
        return None;
    }
    let elem = dt.size;
    if elem == 0 {
        return None;
    }
    let total: u64 = ds.dims.iter().fold(1u64, |a, d| a.saturating_mul(*d));
    let rank = ds.dims.len();
    if total == 0 || rank == 0 {
        return None;
    }
    let want = total.min(REC_CAP as u64);
    let mut values = Vec::with_capacity(want as usize);
    match obj.layout.as_ref() {
        Some(Hdf5Layout::Chunked { .. }) => {
            let coords = vec![0u64; rank];
            let chunk = file.read_chunk(path, &coords, |off, len| {
                fetch_s3_range(s3_url, off, len, Some(creds))
            })?;
            values.extend(chunk.into_iter().take(want as usize));
        }
        Some(Hdf5Layout::Contiguous { addr, size }) => {
            let bytes_want = (want * elem as u64).min(*size);
            if bytes_want == 0 {
                return None;
            }
            let raw = fetch_s3_range(s3_url, *addr, bytes_want, Some(creds))?;
            for i in 0..(raw.len() / elem) {
                values.push(decode_value(&raw, i, dt)?);
            }
            apply_scale_offset(obj, &mut values);
        }
        Some(Hdf5Layout::Compact { data }) => {
            for i in 0..(data.len() / elem).min(want as usize) {
                values.push(decode_value(data, i, dt)?);
            }
            apply_scale_offset(obj, &mut values);
        }
        None => return None,
    }
    Some(values)
}

fn rh98_column(
    file: &Hdf5File,
    s3_url: &str,
    creds: &S3Credentials,
    path: &str,
) -> Option<Vec<f64>> {
    let (obj, ds, dt) = file.dataset(path).ok()?;
    if dt.class == 9 {
        return None;
    }
    let rank = ds.dims.len();
    if rank != 2 {
        return None;
    }
    let width = ds.dims[1];
    if width != 101 {
        return None;
    }
    let rows = ds.dims[0].min(REC_CAP as u64) as usize;
    let elem = dt.size;
    if elem == 0 {
        return None;
    }
    match obj.layout.as_ref() {
        Some(Hdf5Layout::Chunked { .. }) => {
            let chunk = file.read_chunk(path, &[0, 0], |off, len| {
                fetch_s3_range(s3_url, off, len, Some(creds))
            })?;
            let avail = chunk.len() / 101;
            let mut out = Vec::with_capacity(avail);
            for r in 0..avail {
                out.push(chunk[r * 101 + 98]);
            }
            Some(out)
        }
        Some(Hdf5Layout::Contiguous { addr, size }) => {
            let bytes_want = ((rows * 101) as u64 * elem as u64).min(*size);
            if bytes_want == 0 {
                return None;
            }
            let raw = fetch_s3_range(s3_url, *addr, bytes_want, Some(creds))?;
            let mut out = Vec::with_capacity(raw.len() / (elem * 101));
            for r in 0..(raw.len() / (elem * 101)) {
                out.push(decode_value(&raw, r * 101 + 98, dt)?);
            }
            apply_scale_offset(obj, &mut out);
            Some(out)
        }
        Some(Hdf5Layout::Compact { data }) => {
            let mut out = Vec::with_capacity((data.len() / (elem * 101)).min(rows));
            for r in 0..(data.len() / (elem * 101)).min(rows) {
                out.push(decode_value(data, r * 101 + 98, dt)?);
            }
            apply_scale_offset(obj, &mut out);
            Some(out)
        }
        None => None,
    }
}

fn extract_from(
    file: &Hdf5File,
    s3_url: &str,
    creds: &S3Credentials,
    beams: usize,
    anchor_tdb: f64,
) -> Vec<[f64; REC_FIELDS]> {
    let mut out = Vec::new();
    for (bi, beam) in BEAMS.iter().enumerate().take(beams) {
        let room = REC_CAP.saturating_sub(out.len());
        if room == 0 {
            break;
        }
        let Some(dts) = first_values(file, s3_url, creds, &format!("{beam}/delta_time")) else {
            continue;
        };
        let Some(lats) = first_values(file, s3_url, creds, &format!("{beam}/lat_lowestmode"))
        else {
            continue;
        };
        let Some(lons) = first_values(file, s3_url, creds, &format!("{beam}/lon_lowestmode"))
        else {
            continue;
        };
        let Some(elevs) = first_values(file, s3_url, creds, &format!("{beam}/elev_lowestmode"))
        else {
            continue;
        };
        let Some(quals) = first_values(file, s3_url, creds, &format!("{beam}/quality_flag")) else {
            continue;
        };
        let Some(rh98) = rh98_column(file, s3_url, creds, &format!("{beam}/rh")) else {
            continue;
        };
        let Some(degrades) = first_values(file, s3_url, creds, &format!("{beam}/degrade_flag"))
        else {
            continue;
        };
        let n = [
            dts.len(),
            lats.len(),
            lons.len(),
            elevs.len(),
            quals.len(),
            rh98.len(),
            degrades.len(),
        ]
        .into_iter()
        .fold(usize::MAX, usize::min)
        .min(room);
        for i in 0..n {
            let dt = dts[i];
            let lat = lats[i];
            let lon = lons[i];
            let elev = elevs[i];
            let rh = rh98[i];
            let q = quals[i];
            let deg = degrades[i];
            if !(dt.is_finite() && dt > 0.0 && dt < 1.0e12)
                || !(lat.is_finite() && (-90.0..=90.0).contains(&lat))
                || !(lon.is_finite() && (-180.0..=180.0).contains(&lon))
                || !(elev.is_finite() && (-500.0..=9000.0).contains(&elev))
                || !(rh.is_finite() && (0.0..=150.0).contains(&rh))
                || q != 1.0
                || deg != 0.0
            {
                continue;
            }
            out.push([dt, lat, lon, elev, rh, bi as f64, q, anchor_tdb]);
        }
    }
    out
}

fn harvest_granule(
    s3_url: &str,
    creds: &S3Credentials,
    beams: usize,
    anchor_tdb: f64,
) -> Vec<[f64; REC_FIELDS]> {
    let Some(w1) = fetch_s3_range(s3_url, 0, META_WINDOW, Some(creds)) else {
        eprintln!(
            "{}: range read returned void — granule stays pending",
            s3_url
        );
        return Vec::new();
    };
    match Hdf5File::parse(&w1) {
        Ok(file) => extract_from(&file, s3_url, creds, beams, anchor_tdb),
        Err(note) => {
            eprintln!(
                "{}: header window of {} B stayed unread ({:?}) — escalating once",
                s3_url, META_WINDOW, note
            );
            let Some(w2) = fetch_s3_range(s3_url, 0, META_ESCALATION, Some(creds)) else {
                eprintln!(
                    "{}: escalated range read returned void — granule stays pending",
                    s3_url
                );
                return Vec::new();
            };
            match Hdf5File::parse(&w2) {
                Ok(file) => extract_from(&file, s3_url, creds, beams, anchor_tdb),
                Err(n2) => {
                    eprintln!(
                        "{}: metadata beyond {} B ({:?}) — granule stays pending",
                        s3_url, META_ESCALATION, n2
                    );
                    Vec::new()
                }
            }
        }
    }
}

fn pack(recs: &[[f64; REC_FIELDS]]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + recs.len() * REC_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(recs.len() as u32).to_le_bytes());
    for r in recs {
        for v in r {
            buf.extend_from_slice(&v.to_le_bytes());
        }
    }
    buf
}

fn unpack(bytes: &[u8]) -> Option<Vec<[f64; REC_FIELDS]>> {
    if bytes.len() < 8 || bytes[..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let mut r = [0f64; REC_FIELDS];
        for slot in r.iter_mut() {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        out.push(r);
    }
    Some(out)
}

fn run_list(args: &[String]) {
    let prefix = arg_value(args, "--prefix").unwrap_or(PRODUCT_ROOT.to_string());
    let dirs = args.iter().any(|a| a == "--dirs");
    let max_keys = arg_usize(args, "--max-keys").unwrap_or(LIST_MAX_KEYS as usize) as u32;
    let Some(token) = edl_token() else {
        eprintln!("EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token");
        std::process::exit(2);
    };
    let Some(creds) = edl_s3_credentials_for(BUCKET, &token) else {
        eprintln!("{} returned void for s3://{}/", NETLOC, BUCKET);
        std::process::exit(2);
    };
    let Some((objects, truncated, common)) = list_page(BUCKET, &prefix, dirs, max_keys, &creds)
    else {
        eprintln!("the listing of s3://{BUCKET}/{prefix} returned void");
        std::process::exit(1);
    };
    if dirs {
        for d in &common {
            println!("{d}");
        }
        eprintln!(
            "gedi-l2a: {} common prefixes under s3://{}/{}",
            common.len(),
            BUCKET,
            prefix
        );
        if truncated {
            eprintln!("gedi-l2a: the listing is truncated — the page is partial");
        }
        if common.is_empty() {
            std::process::exit(1);
        }
        return;
    }
    println!("#key|size_bytes|last_modified");
    for o in &objects {
        println!("{}|{}|{}", o.key, o.size, o.modified);
    }
    eprintln!(
        "gedi-l2a: {} keys under s3://{}/{}",
        objects.len(),
        BUCKET,
        prefix
    );
    if truncated {
        eprintln!("gedi-l2a: the listing is truncated — the page is partial");
    }
    if objects.is_empty() {
        std::process::exit(1);
    }
}

fn run_harvest(args: &[String]) {
    let out_path = arg_value(args, "--out").unwrap_or(DEFAULT_OUT.to_string());
    let prefix = match arg_value(args, "--day") {
        Some(d) => match gedi_day_prefix(&d) {
            Some(p) => format!("{PRODUCT_ROOT}{p}"),
            None => {
                eprintln!("gedi-l2a: --day {d} carries no civil date (YYYY.MM.DD) — refused");
                std::process::exit(2);
            }
        },
        None => match arg_value(args, "--prefix") {
            Some(p) => p,
            None => {
                eprintln!(
                    "usage: gedi_l2a_compiler --day <YYYY.MM.DD> [--limit N] [--beams N] [--out <path>] [--ci-mode] | --list [--prefix <p>] [--dirs] [--max-keys N] — refused"
                );
                std::process::exit(2);
            }
        },
    };
    let limit = arg_usize(args, "--limit").unwrap_or(2).clamp(1, 8);
    let beams = arg_usize(args, "--beams")
        .unwrap_or(BEAMS.len())
        .clamp(1, BEAMS.len());
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(token) = edl_token() else {
        eprintln!("EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token");
        std::process::exit(2);
    };
    let Some(creds) = edl_s3_credentials_for(BUCKET, &token) else {
        eprintln!("{} returned void for s3://{}/", NETLOC, BUCKET);
        std::process::exit(2);
    };
    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "gedi-l2a: the embedded naif0012.tls leap table is absent — no granule anchor folds to the TDB clock; the harvest stays unwritten (0 honored, pending)"
        );
        std::process::exit(1);
    };
    let list_keys = (limit as u32).saturating_add(8).min(LIST_MAX_KEYS);
    let Some((objects, truncated, _)) = list_page(BUCKET, &prefix, false, list_keys, &creds) else {
        eprintln!("the listing of s3://{BUCKET}/{prefix} returned void");
        std::process::exit(1);
    };
    if truncated {
        eprintln!(
            "gedi-l2a: the listing at s3://{BUCKET}/{prefix} is truncated — the harvest is partial"
        );
    }
    if objects.is_empty() {
        eprintln!("gedi-l2a: no granules at s3://{BUCKET}/{prefix} — nothing fabricated");
        std::process::exit(1);
    }
    let mut chosen = objects;
    chosen.sort_by(|a, b| a.key.cmp(&b.key));
    chosen.truncate(limit);
    let mut recs: Vec<[f64; REC_FIELDS]> = Vec::new();
    for obj in &chosen {
        let Some(anchor_tdb) = granule_anchor_tdb(&obj.key, &lsk) else {
            eprintln!(
                "gedi-l2a: {} carries no absolute granule anchor — the granule stays pending",
                obj.key
            );
            continue;
        };
        let s3_url = format!("s3://{BUCKET}/{}", obj.key);
        let before = recs.len();
        let granule = harvest_granule(&s3_url, &creds, beams, anchor_tdb);
        recs.extend(granule);
        eprintln!(
            "gedi-l2a: {} → {} records ({} B granule)",
            obj.key,
            recs.len() - before,
            obj.size
        );
    }
    recs.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if recs.is_empty() {
        eprintln!("gedi-l2a: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = pack(&recs);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() && fs::create_dir_all(parent).is_err() {
            eprintln!("create_dir_all for {} returned void", parent.display());
            std::process::exit(1);
        }
    }
    if fs::write(&out_path, &bytes).is_err() {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    match unpack(&bytes) {
        Some(parsed) => eprintln!(
            "{}: {} records, {} B, roundtrip parses",
            out_path,
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{}: roundtrip parse void", out_path);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--list") {
        run_list(&args);
    } else {
        run_harvest(&args);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_unpack_roundtrip() {
        let recs = vec![
            [1.0, -12.5, 33.25, 210.5, 27.3, 0.0, 1.0, 800_000_000.0],
            [2.0, 45.0, -110.0, 1200.0, 12.8, 3.0, 1.0, 800_000_000.0],
        ];
        let bytes = pack(&recs);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        let parsed = unpack(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], recs[0]);
        assert_eq!(parsed[1], recs[1]);
    }

    #[test]
    fn unpack_rejects_foreign_bytes() {
        assert!(unpack(b"X").is_none());
        assert!(unpack(b"GED1abc").is_none());
        assert!(unpack(b"NRS1").is_none());
        let mut short = pack(&[[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]]);
        short.truncate(short.len() - 1);
        assert!(unpack(&short).is_none());
    }

    #[test]
    fn gedi_filename_folds_to_tdb_anchor() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let key = "GEDI02_A.002/2020.01.01/GEDI02_A_2020001000000_O00001_T00001_02_001_01_V002.h5";
        let anchor = granule_anchor_tdb(key, &lsk).expect("granule start folds to tdb");
        let unix = days_from_civil(2020, 1, 1).unwrap() as f64 * 86400.0;
        assert_eq!(anchor, lsk.unix_to_tdb(unix).expect("tdb"));
    }

    #[test]
    fn gedi_anchor_absent_on_nonmatching_key() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        assert!(granule_anchor_tdb("not-a-gedi-key", &lsk).is_none());
        assert!(granule_anchor_tdb("GEDI02_A_20X0001000000", &lsk).is_none());
    }

    #[test]
    fn gedi_day_prefix_folds_to_granule_name_prefix() {
        assert_eq!(
            gedi_day_prefix("2025.07.09").as_deref(),
            Some("GEDI02_A_2025190")
        );
        assert_eq!(
            gedi_day_prefix("2020.01.01").as_deref(),
            Some("GEDI02_A_2020001")
        );
        assert!(gedi_day_prefix("not-a-day").is_none());
    }

    #[test]
    fn query_encoding_sorts_and_encodes() {
        let mut params: Vec<(String, String)> = vec![
            ("list-type".to_string(), "2".to_string()),
            ("prefix".to_string(), "GEDI02_A.002/2026.01.01/".to_string()),
        ];
        params.sort_by(|a, b| a.0.cmp(&b.0));
        let query = params
            .iter()
            .map(|(k, v)| format!("{}={}", uri_encode_query(k), uri_encode_query(v)))
            .collect::<Vec<_>>()
            .join("&");
        assert_eq!(query, "list-type=2&prefix=GEDI02_A.002%2F2026.01.01%2F");
    }

    #[test]
    fn parse_page_decodes_objects_dirs_and_truncation() {
        let doc = "<ListBucketResult><IsTruncated>true</IsTruncated>\
<Contents><Key>GEDI02_A.002/2026.01.01/a.h5</Key><LastModified>2026-01-01T00:00:00.000Z</LastModified><Size>1048576</Size></Contents>\
<CommonPrefixes><Prefix>GEDI02_A.002/2026.01.02/</Prefix></CommonPrefixes>\
</ListBucketResult>";
        let (objects, truncated, dirs) = parse_page(doc).unwrap();
        assert!(truncated);
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].key, "GEDI02_A.002/2026.01.01/a.h5");
        assert_eq!(objects[0].size, 1048576);
        assert_eq!(dirs, vec!["GEDI02_A.002/2026.01.02/"]);
    }

    #[test]
    fn civil_days_roundtrip_epoch_and_2026() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(20_000), (2024, 10, 3));
        assert_eq!(civil_from_days(20_600), (2026, 5, 26));
    }

    #[test]
    fn amz_now_builds_the_scope_shape() {
        let (date_stamp, amz) = amz_now().unwrap();
        assert_eq!(date_stamp.len(), 8);
        assert!(amz.starts_with(&date_stamp));
        assert!(amz.ends_with('Z'));
        assert_eq!(amz.len(), 16);
    }

    #[test]
    fn parse_secret_takes_the_last_duplicate_key() {
        let text = "EARTHDATA_EDL_TOKEN=first\nEARTHDATA_EDL_TOKEN=last\n";
        assert_eq!(
            parse_secret(text, "EARTHDATA_EDL_TOKEN").as_deref(),
            Some("last")
        );
    }

    #[test]
    fn parse_secret_rejects_commented_and_suffixed_keys() {
        let text = "#EARTHDATA_EDL_TOKEN=commented\nEARTHDATA_EDL_TOKEN_OLD=stale\n";
        assert_eq!(parse_secret(text, "EARTHDATA_EDL_TOKEN"), None);
    }
}
