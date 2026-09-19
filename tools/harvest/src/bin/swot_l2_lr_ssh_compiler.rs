use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::range::{
    S3_ENDPOINT, S3_REGION, S3Credentials, Sigv4Args, edl_s3_credentials_for, fetch_bearer_range,
    fetch_s3_range, s3_parts, sigv4_headers,
};
use omegaflow::archivar::{LeapSeconds, embedded_lsk};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{
    Endian, Hdf5Datatype, Hdf5File, Hdf5Layout, Hdf5Object, decode_f32, decode_f64,
};
use omegaflow::lsk::days_from_civil;
use omegaflow::netcdf::{NetcdfFile, NetcdfType, NetcdfVar, nc4_group};
use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "archive.podaac.earthdata.nasa.gov";
const BUCKET_PUBLIC: &str = "podaac-swot-ops-cumulus-public";
const BUCKET_PROTECTED: &str = "podaac-swot-ops-cumulus-protected";
const PRODUCT_ROOT: &str = "SWOT_L2_LR_SSH_D/";
const DEFAULT_OUT: &str = "data/archive.podaac.earthdata.nasa.gov/swot_l2_lr_ssh.bin";
const MAGIC: [u8; 4] = *b"SWS1";
const REC_FIELDS: usize = 5;
const REC_BYTES: usize = REC_FIELDS * 8;
const META_WINDOW: u64 = 1 << 25;
const META_ESCALATION: u64 = 3 * (1 << 25);
const REC_CAP: usize = 1 << 13;
const LIST_MAX_KEYS: u32 = 20;
const LIST_MAX_T_S: u64 = 1 << 7;
const CONNECT_BOUND_S: u64 = 1 << 5;
const CDF_MAGIC: [u8; 3] = *b"CDF";
const HDF_MAGIC: [u8; 4] = [0x89, b'H', b'D', b'F'];
const NC4_GROUPS: [&str; 3] = ["left", "right", ""];
const SSH_NAMES: [&str; 2] = ["ssh_karin_2", "ssha"];
const CMR_GRANULES_URL: &str = "https://cmr.earthdata.nasa.gov/search/granules.json";
const SWOT_SHORT_NAME: &str = "SWOT_L2_LR_SSH_D";
const SWOT_HTTPS_HOST: &str = "archive.swot.podaac.earthdata.nasa.gov";

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

fn day_temporal(day: &str) -> Option<(String, String)> {
    let year: i64 = day.get(0..4)?.parse().ok()?;
    let month: i64 = day.get(5..7)?.parse().ok()?;
    let dom: i64 = day.get(8..10)?.parse().ok()?;
    days_from_civil(year, month, dom)?;
    Some((
        format!("{year:04}-{month:02}-{dom:02}T00:00:00Z"),
        format!("{year:04}-{month:02}-{dom:02}T23:59:59Z"),
    ))
}

fn epoch_from_units(units: &str, lsk: &LeapSeconds) -> Option<f64> {
    let since = units.find("since")?;
    let rest = units[since + "since".len()..].trim();
    let mut tokens = rest.split_whitespace();
    let date = tokens.next()?;
    let (ymd, time) = match date.split_once('T') {
        Some((d, t)) => (d, Some(t.to_string())),
        None => (date, tokens.next().map(|t| t.to_string())),
    };
    let year: i64 = ymd.get(0..4)?.parse().ok()?;
    let month: i64 = ymd.get(5..7)?.parse().ok()?;
    let day: i64 = ymd.get(8..10)?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let (hh, mm, ss) = match time.as_deref() {
        Some(t) if t.len() >= 8 => {
            let (hh, rest) = t.split_once(':')?;
            let (mm, ss) = rest.split_once(':')?;
            let ss = ss.split('.').next()?;
            (
                hh.parse::<i64>().ok()?,
                mm.parse::<i64>().ok()?,
                ss.parse::<i64>().ok()?,
            )
        }
        _ => (0, 0, 0),
    };
    let unix = days as f64 * 86400.0 + (hh * 3600 + mm * 60 + ss) as f64;
    lsk.unix_to_tdb(unix)
}

fn nc4_time_units(file: &Hdf5File, time_path: &str) -> Option<String> {
    let a = file.attribute(time_path, "units")?;
    if a.datatype.class != 3 {
        return None;
    }
    Some(
        String::from_utf8_lossy(&a.data)
            .trim_end_matches('\0')
            .to_string(),
    )
}

fn classic_time_units(ncf: &NetcdfFile, time_name: &str) -> Option<String> {
    let v = ncf.var(time_name)?;
    let a = v.attrs.iter().find(|a| a.name == "units")?;
    ncf.attr_text(a)
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

struct CmrGranule {
    bucket: String,
    key: String,
    data_url: Option<String>,
}

fn cmr_fetch(url: &str) -> Option<String> {
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
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if !out.status.success() {
        eprintln!(
            "cmr returned ({}): {} {}",
            out.status,
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn cmr_parse(body: &str) -> Option<Vec<CmrGranule>> {
    let json = parse_json(body)?;
    let JsonVal::Obj(mut map) = json else {
        return None;
    };
    let feed = map.remove("feed")?;
    let JsonVal::Obj(mut feed) = feed else {
        return None;
    };
    let entry = feed.remove("entry")?;
    let JsonVal::Arr(entries) = entry else {
        return None;
    };
    let mut out = Vec::new();
    for e in entries {
        let JsonVal::Obj(mut e) = e else { continue };
        let links = e.remove("links")?;
        let JsonVal::Arr(links) = links else { continue };
        let mut s3_href = None;
        let mut data_href = None;
        for l in links {
            let JsonVal::Obj(mut l) = l else { continue };
            let rel = match l.remove("rel") {
                Some(JsonVal::Str(s)) => s,
                _ => continue,
            };
            let h = match l.remove("href") {
                Some(JsonVal::Str(s)) => s,
                _ => continue,
            };
            if rel.ends_with("/s3#") && h.ends_with(".nc") {
                s3_href = Some(h);
            } else if rel.ends_with("/data#") && h.ends_with(".nc") && h.starts_with("https://") {
                data_href = Some(h);
            }
        }
        let Some(s3_href) = s3_href else { continue };
        let Some((bucket, key)) = s3_parts(&s3_href) else {
            continue;
        };
        out.push(CmrGranule {
            bucket,
            key,
            data_url: data_href,
        });
    }
    Some(out)
}

fn cmr_granules(short_name: &str, temporal: &str, page_size: usize) -> Option<Vec<CmrGranule>> {
    let query = format!(
        "short_name={}&temporal={}&page_size={}",
        uri_encode_query(short_name),
        uri_encode_query(temporal),
        page_size
    );
    let url = format!("{CMR_GRANULES_URL}?{query}");
    cmr_parse(&cmr_fetch(&url)?)
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

fn nc4_first_values(file: &Hdf5File, g: &Granule, path: &str) -> Option<Vec<f64>> {
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
            let chunk = file.read_chunk(path, &coords, |off, len| g.read_range(off, len))?;
            values.extend(chunk.into_iter().take(want as usize));
        }
        Some(Hdf5Layout::Contiguous { addr, size }) => {
            let bytes_want = (want * elem as u64).min(*size);
            if bytes_want == 0 {
                return None;
            }
            let raw = g.read_range(*addr, bytes_want)?;
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

fn nc4_fill(file: &Hdf5File, path: &str) -> Option<f64> {
    let obj = file.resolve(path).ok()?;
    attr_number(obj, "_FillValue").or_else(|| attr_number(obj, "missing_value"))
}

fn choose_paths_nc4(file: &Hdf5File) -> Option<(String, String, String, String)> {
    for g in NC4_GROUPS {
        let Ok(grp) = nc4_group(file, g) else {
            continue;
        };
        let has = |n: &str| grp.variables.iter().any(|v| v.name == n);
        if !has("latitude") || !has("longitude") || !has("time") {
            continue;
        }
        let Some(ssha) = grp
            .variables
            .iter()
            .map(|v| v.name.as_str())
            .find(|n| n.contains("ssh_karin") || SSH_NAMES.contains(n))
        else {
            continue;
        };
        let join = |n: &str| {
            if g.is_empty() {
                n.to_string()
            } else {
                format!("{g}/{n}")
            }
        };
        return Some((
            join(ssha),
            join("latitude"),
            join("longitude"),
            join("time"),
        ));
    }
    None
}

fn classic_set(ncf: &NetcdfFile) -> Option<(String, String, String, String)> {
    let ssha = SSH_NAMES
        .iter()
        .find_map(|n| ncf.var(n).map(|_| n.to_string()))?;
    ncf.var("latitude")?;
    ncf.var("longitude")?;
    ncf.var("time")?;
    Some((
        ssha,
        "latitude".to_string(),
        "longitude".to_string(),
        "time".to_string(),
    ))
}

fn decode_classic(raw: &[u8], i: usize, t: NetcdfType) -> Option<f64> {
    let sz = t.size();
    if sz == 0 {
        return None;
    }
    let off = i.checked_mul(sz)?;
    let b = raw.get(off..off + sz)?;
    match t {
        NetcdfType::Byte => Some(b[0] as i8 as f64),
        NetcdfType::Short => Some(i16::from_be_bytes(b.try_into().ok()?) as f64),
        NetcdfType::Int => Some(i32::from_be_bytes(b.try_into().ok()?) as f64),
        NetcdfType::Float => Some(f32::from_bits(u32::from_be_bytes(b.try_into().ok()?)) as f64),
        NetcdfType::Double => Some(f64::from_bits(u64::from_be_bytes(b.try_into().ok()?))),
        NetcdfType::Char => None,
    }
}

fn classic_attr_num(ncf: &NetcdfFile, v: &NetcdfVar, name: &str) -> Option<f64> {
    let a = v.attrs.iter().find(|a| a.name == name)?;
    ncf.attr_num(a)
}

fn classic_first_values(ncf: &NetcdfFile, g: &Granule, name: &str) -> Option<Vec<f64>> {
    let v = ncf.var(name)?;
    let shape = ncf.var_shape(v).ok()?;
    let total: u64 = shape.iter().fold(1u64, |a, d| a.saturating_mul(*d));
    if total == 0 {
        return None;
    }
    let want = total.min(REC_CAP as u64);
    let tsize = v.nc_type.size() as u64;
    if tsize == 0 {
        return None;
    }
    let bytes_want = want.saturating_mul(tsize).min(v.vsize as u64);
    let raw = g.read_range(v.begin, bytes_want)?;
    let mut out = Vec::with_capacity((raw.len() / tsize as usize).min(want as usize));
    for i in 0..(raw.len() / tsize as usize) {
        out.push(decode_classic(&raw, i, v.nc_type)?);
    }
    if let Some(scale) = classic_attr_num(ncf, v, "scale_factor") {
        let offset = classic_attr_num(ncf, v, "add_offset");
        for x in out.iter_mut() {
            *x = match offset {
                Some(o) => *x * scale + o,
                None => *x * scale,
            };
        }
    }
    Some(out)
}

fn classic_fill(ncf: &NetcdfFile, name: &str) -> Option<f64> {
    let v = ncf.var(name)?;
    v.attrs
        .iter()
        .find(|a| a.name == "_FillValue" || a.name == "missing_value")
        .and_then(|a| ncf.attr_num(a))
}

fn assemble(
    times: &[f64],
    lats: &[f64],
    lons: &[f64],
    sshas: &[f64],
    ssha_fill: Option<f64>,
    anchor_tdb: f64,
) -> Vec<[f64; REC_FIELDS]> {
    let n = [times.len(), lats.len(), lons.len(), sshas.len()]
        .into_iter()
        .fold(usize::MAX, usize::min)
        .min(REC_CAP);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = times[i];
        let lat = lats[i];
        let lon = lons[i];
        let ssha = sshas[i];
        if !(t.is_finite() && t > 0.0 && t < 2.0e9)
            || !(lat.is_finite() && (-90.0..=90.0).contains(&lat))
            || !(lon.is_finite() && (-180.0..=180.0).contains(&lon))
            || !(ssha.is_finite() && (-20.0..=20.0).contains(&ssha))
            || ssha_fill.is_some_and(|f| ssha == f)
        {
            continue;
        }
        out.push([t, lat, lon, ssha, anchor_tdb]);
    }
    out
}

fn extract_nc4(file: &Hdf5File, g: &Granule, lsk: &LeapSeconds) -> Vec<[f64; REC_FIELDS]> {
    let Some((ssha_path, lat_path, lon_path, time_path)) = choose_paths_nc4(file) else {
        return Vec::new();
    };
    let Some(units) = nc4_time_units(file, &time_path) else {
        return Vec::new();
    };
    let Some(anchor_tdb) = epoch_from_units(&units, lsk) else {
        return Vec::new();
    };
    let Some(times) = nc4_first_values(file, g, &time_path) else {
        return Vec::new();
    };
    let Some(lats) = nc4_first_values(file, g, &lat_path) else {
        return Vec::new();
    };
    let Some(lons) = nc4_first_values(file, g, &lon_path) else {
        return Vec::new();
    };
    let Some(sshas) = nc4_first_values(file, g, &ssha_path) else {
        return Vec::new();
    };
    let fill = nc4_fill(file, &ssha_path);
    assemble(&times, &lats, &lons, &sshas, fill, anchor_tdb)
}

fn extract_classic(ncf: &NetcdfFile, g: &Granule, lsk: &LeapSeconds) -> Vec<[f64; REC_FIELDS]> {
    let Some((ssha_name, lat_name, lon_name, time_name)) = classic_set(ncf) else {
        return Vec::new();
    };
    let Some(units) = classic_time_units(ncf, &time_name) else {
        return Vec::new();
    };
    let Some(anchor_tdb) = epoch_from_units(&units, lsk) else {
        return Vec::new();
    };
    let Some(times) = classic_first_values(ncf, g, &time_name) else {
        return Vec::new();
    };
    let Some(lats) = classic_first_values(ncf, g, &lat_name) else {
        return Vec::new();
    };
    let Some(lons) = classic_first_values(ncf, g, &lon_name) else {
        return Vec::new();
    };
    let Some(sshas) = classic_first_values(ncf, g, &ssha_name) else {
        return Vec::new();
    };
    let fill = classic_fill(ncf, &ssha_name);
    assemble(&times, &lats, &lons, &sshas, fill, anchor_tdb)
}

struct Granule {
    url: String,
    bearer: Option<String>,
    creds: Option<S3Credentials>,
}

impl Granule {
    fn read_range(&self, offset: u64, len: u64) -> Option<Vec<u8>> {
        match &self.bearer {
            Some(tok) => fetch_bearer_range(&self.url, offset, len, tok),
            None => fetch_s3_range(&self.url, offset, len, self.creds.as_ref()),
        }
    }
}

fn harvest_granule(g: &Granule, lsk: &LeapSeconds) -> Vec<[f64; REC_FIELDS]> {
    let Some(w1) = g.read_range(0, META_WINDOW) else {
        eprintln!(
            "{}: range read returned void — granule stays pending",
            g.url
        );
        return Vec::new();
    };
    if w1.len() >= 3 && w1[..3] == CDF_MAGIC {
        match NetcdfFile::parse(&w1) {
            Ok(ncf) => return extract_classic(&ncf, g, lsk),
            Err(note) => {
                eprintln!(
                    "{}: classic header in {} B stayed unread ({:?}) — escalating once",
                    g.url, META_WINDOW, note
                );
            }
        }
    } else if w1.len() >= 4 && w1[..4] == HDF_MAGIC {
        match Hdf5File::parse_fetch(&w1, |off, len| g.read_range(off, len)) {
            Ok(file) => return extract_nc4(&file, g, lsk),
            Err(note) => {
                eprintln!(
                    "{}: nc4 header window of {} B stayed unread ({:?}) — escalating once",
                    g.url, META_WINDOW, note
                );
            }
        }
    } else {
        eprintln!(
            "{}: magic is {:X?} — neither CDF nor HDF5 — granule stays pending",
            g.url, w1
        );
        return Vec::new();
    }
    let Some(w2) = g.read_range(0, META_ESCALATION) else {
        eprintln!(
            "{}: escalated range read returned void — granule stays pending",
            g.url
        );
        return Vec::new();
    };
    if w2.len() >= 3 && w2[..3] == CDF_MAGIC {
        match NetcdfFile::parse(&w2) {
            Ok(ncf) => extract_classic(&ncf, g, lsk),
            Err(n2) => {
                eprintln!(
                    "{}: metadata beyond {} B ({:?}) — granule stays pending",
                    g.url, META_ESCALATION, n2
                );
                Vec::new()
            }
        }
    } else if w2.len() >= 4 && w2[..4] == HDF_MAGIC {
        match Hdf5File::parse_fetch(&w2, |off, len| g.read_range(off, len)) {
            Ok(file) => extract_nc4(&file, g, lsk),
            Err(n2) => {
                eprintln!(
                    "{}: metadata beyond {} B ({:?}) — granule stays pending",
                    g.url, META_ESCALATION, n2
                );
                Vec::new()
            }
        }
    } else {
        eprintln!(
            "{}: magic changed across windows — granule stays pending",
            g.url
        );
        Vec::new()
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

fn creds_for(token: &str, bucket: &str) -> Option<S3Credentials> {
    let c = edl_s3_credentials_for(bucket, token)?;
    Some(c)
}

fn run_list(args: &[String]) {
    let prefix = arg_value(args, "--prefix").unwrap_or(PRODUCT_ROOT.to_string());
    let dirs = args.iter().any(|a| a == "--dirs");
    let max_keys = arg_usize(args, "--max-keys").unwrap_or(LIST_MAX_KEYS as usize) as u32;
    let protected = args.iter().any(|a| a == "--protected");
    let bucket = if protected {
        BUCKET_PROTECTED
    } else {
        BUCKET_PUBLIC
    };
    let Some(token) = edl_token() else {
        eprintln!("EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token");
        std::process::exit(2);
    };
    let Some(creds) = creds_for(&token, bucket) else {
        eprintln!("{} returned void for s3://{}/", NETLOC, bucket);
        std::process::exit(2);
    };
    let Some((objects, truncated, common)) = list_page(bucket, &prefix, dirs, max_keys, &creds)
    else {
        eprintln!("the listing of s3://{bucket}/{prefix} returned void");
        std::process::exit(1);
    };
    if dirs {
        for d in &common {
            println!("{d}");
        }
        eprintln!(
            "swot-l2-lr-ssh: {} common prefixes under s3://{}/{}",
            common.len(),
            bucket,
            prefix
        );
        if truncated {
            eprintln!("swot-l2-lr-ssh: the listing is truncated — the page is partial");
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
        "swot-l2-lr-ssh: {} keys under s3://{}/{}",
        objects.len(),
        bucket,
        prefix
    );
    if truncated {
        eprintln!("swot-l2-lr-ssh: the listing is truncated — the page is partial");
    }
    if objects.is_empty() {
        std::process::exit(1);
    }
}

fn run_harvest(args: &[String]) {
    let out_path = arg_value(args, "--out").unwrap_or(DEFAULT_OUT.to_string());
    let limit = arg_usize(args, "--limit").unwrap_or(2).clamp(1, 8);
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let protected = args.iter().any(|a| a == "--protected");
    let Some(token) = edl_token() else {
        eprintln!("EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token");
        std::process::exit(2);
    };
    let direct_granule = arg_value(args, "--granule");
    let cmr = args.iter().any(|a| a == "--cmr");
    let mut chosen: Vec<(Granule, String, u64)> = Vec::new();
    if cmr {
        let short_name = arg_value(args, "--short-name").unwrap_or(SWOT_SHORT_NAME.to_string());
        let temporal = match arg_value(args, "--temporal") {
            Some(t) => t,
            None => match arg_value(args, "--day") {
                Some(d) => match day_temporal(&d) {
                    Some((s, e)) => format!("{s},{e}"),
                    None => {
                        eprintln!(
                            "swot-l2-lr-ssh: --day {d} carries no civil date (YYYY.MM.DD) — refused"
                        );
                        std::process::exit(2);
                    }
                },
                None => {
                    eprintln!(
                        "usage: swot_l2_lr_ssh_compiler --cmr (--temporal <start>,<end> | --day <YYYY.MM.DD>) [--short-name <n>] [--limit N] [--out <path>] [--ci-mode] — refused"
                    );
                    std::process::exit(2);
                }
            },
        };
        let page_size = limit.saturating_add(8).min(64);
        let Some(granules) = cmr_granules(&short_name, &temporal, page_size) else {
            eprintln!(
                "swot-l2-lr-ssh: the CMR granule search returned void for {short_name} on {temporal}"
            );
            std::process::exit(1);
        };
        if granules.is_empty() {
            eprintln!(
                "swot-l2-lr-ssh: no CMR granules for {short_name} on {temporal} — nothing fabricated"
            );
            std::process::exit(1);
        }
        let mut granules = granules;
        granules.sort_by(|a, b| a.key.cmp(&b.key));
        granules.truncate(limit);
        for g in granules {
            let reader = match g.data_url {
                Some(url) => Granule {
                    url,
                    bearer: Some(token.clone()),
                    creds: None,
                },
                None => {
                    let Some(creds) = creds_for(&token, &g.bucket) else {
                        eprintln!("{} returned void for s3://{}/", NETLOC, g.bucket);
                        std::process::exit(2);
                    };
                    Granule {
                        url: format!("s3://{}/{}", g.bucket, g.key),
                        bearer: None,
                        creds: Some(creds),
                    }
                }
            };
            chosen.push((reader, g.key, 0));
        }
    } else if let Some(key) = direct_granule {
        let bucket = if protected {
            BUCKET_PROTECTED
        } else {
            BUCKET_PUBLIC
        };
        let reader = if protected {
            Granule {
                url: format!("https://{SWOT_HTTPS_HOST}/{bucket}/{key}"),
                bearer: Some(token.clone()),
                creds: None,
            }
        } else {
            let Some(creds) = creds_for(&token, bucket) else {
                eprintln!("{} returned void for s3://{}/", NETLOC, bucket);
                std::process::exit(2);
            };
            Granule {
                url: format!("s3://{bucket}/{key}"),
                bearer: None,
                creds: Some(creds),
            }
        };
        chosen.push((reader, key, 0));
    } else {
        let prefix = match arg_value(args, "--prefix") {
            Some(p) => p,
            None => {
                eprintln!(
                    "usage: swot_l2_lr_ssh_compiler --prefix <p> | --granule <key> [--protected] [--limit N] [--out <path>] [--ci-mode] | --list [--prefix <p>] [--dirs] [--max-keys N] [--protected] — refused"
                );
                std::process::exit(2);
            }
        };
        let list_keys = (limit as u32).saturating_add(8).min(LIST_MAX_KEYS);
        let mut found: Option<(Vec<Obj>, bool)> = None;
        let mut found_bucket = String::new();
        let order: [&str; 2] = [BUCKET_PROTECTED, BUCKET_PUBLIC];
        for bucket in order {
            let Some(creds) = creds_for(&token, bucket) else {
                eprintln!("{} returned void for s3://{}/", NETLOC, bucket);
                std::process::exit(2);
            };
            match list_page(bucket, &prefix, false, list_keys, &creds) {
                Some((objects, truncated, _)) if !objects.is_empty() => {
                    found = Some((objects, truncated));
                    found_bucket = bucket.to_string();
                    break;
                }
                Some((_, truncated, _)) => {
                    eprintln!(
                        "swot-l2-lr-ssh: s3://{bucket}/{prefix} carries no keys{} — trying the sibling bucket",
                        if truncated { " (page truncated)" } else { "" }
                    );
                }
                None => {
                    eprintln!(
                        "swot-l2-lr-ssh: the listing of s3://{bucket}/{prefix} returned void"
                    );
                }
            }
        }
        let Some((objects, truncated)) = found else {
            eprintln!(
                "swot-l2-lr-ssh: no granules at either SWOT bucket under {prefix} — nothing fabricated"
            );
            std::process::exit(1);
        };
        if truncated {
            eprintln!(
                "swot-l2-lr-ssh: the listing at s3://{found_bucket}/{prefix} is truncated — the harvest is partial"
            );
        }
        let mut objects = objects;
        objects.sort_by(|a, b| a.key.cmp(&b.key));
        objects.truncate(limit);
        let Some(creds) = creds_for(&token, &found_bucket) else {
            eprintln!("{} returned void for s3://{}/", NETLOC, found_bucket);
            std::process::exit(2);
        };
        for o in objects {
            let reader = Granule {
                url: format!("s3://{found_bucket}/{}", o.key),
                bearer: None,
                creds: Some(creds.clone()),
            };
            chosen.push((reader, o.key, o.size));
        }
    }
    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "swot-l2-lr-ssh: the embedded naif0012.tls leap table is absent — no time anchor folds to the TDB clock; the harvest stays unwritten (0 honored, pending)"
        );
        std::process::exit(1);
    };
    let mut recs: Vec<[f64; REC_FIELDS]> = Vec::new();
    for (g, key, size) in &chosen {
        let before = recs.len();
        let granule = harvest_granule(g, &lsk);
        recs.extend(granule);
        eprintln!(
            "swot-l2-lr-ssh: {} → {} records ({} B granule)",
            key,
            recs.len() - before,
            size
        );
    }
    recs.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if recs.is_empty() {
        eprintln!("swot-l2-lr-ssh: no records — the bin stays unwritten (0 honored)");
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
            [750_000_000.0, -30.5, 10.25, 0.42, -43_071.816],
            [750_000_001.0, -30.51, 10.26, -0.15, -43_071.816],
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
        assert!(unpack(b"SWS1abc").is_none());
        assert!(unpack(b"GED1").is_none());
        let mut short = pack(&[[1.0, 2.0, 3.0, 4.0, 5.0]]);
        short.truncate(short.len() - 1);
        assert!(unpack(&short).is_none());
    }

    #[test]
    fn assemble_masks_fill_and_ranges() {
        let times = [750_000_000.0, 750_000_001.0, 750_000_002.0];
        let lats = [-30.5, -91.0, -30.7];
        let lons = [10.25, 10.25, 190.0];
        let sshas = [0.42, -21_474_836_47.0, 0.1];
        let recs = assemble(
            &times,
            &lats,
            &lons,
            &sshas,
            Some(-21_474_836_47.0),
            -43_071.816,
        );
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0], [750_000_000.0, -30.5, 10.25, 0.42, -43_071.816]);
    }

    #[test]
    fn time_units_fold_to_tdb_anchor() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let anchor =
            epoch_from_units("seconds since 2000-01-01 00:00:00 UTC", &lsk).expect("epoch folds");
        let unix = days_from_civil(2000, 1, 1).unwrap() as f64 * 86400.0;
        assert_eq!(anchor, lsk.unix_to_tdb(unix).expect("tdb"));
        assert!(epoch_from_units("no since token here", &lsk).is_none());
    }

    #[test]
    fn classic_big_endian_decode() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&12.5f64.to_be_bytes());
        raw.extend_from_slice(&(-3.25f32).to_be_bytes());
        raw.extend_from_slice(&42i16.to_be_bytes());
        assert_eq!(decode_classic(&raw, 0, NetcdfType::Double), Some(12.5));
        assert_eq!(decode_classic(&raw, 1, NetcdfType::Float), Some(-3.25));
        assert_eq!(decode_classic(&raw, 2, NetcdfType::Short), Some(42.0));
        assert_eq!(decode_classic(&raw, 1, NetcdfType::Char), None);
    }

    #[test]
    fn query_encoding_sorts_and_encodes() {
        let mut params: Vec<(String, String)> = vec![
            ("list-type".to_string(), "2".to_string()),
            (
                "prefix".to_string(),
                "SWOT_L2_LR_SSH_D/cycle_500/".to_string(),
            ),
        ];
        params.sort_by(|a, b| a.0.cmp(&b.0));
        let query = params
            .iter()
            .map(|(k, v)| format!("{}={}", uri_encode_query(k), uri_encode_query(v)))
            .collect::<Vec<_>>()
            .join("&");
        assert_eq!(query, "list-type=2&prefix=SWOT_L2_LR_SSH_D%2Fcycle_500%2F");
    }

    #[test]
    fn parse_page_decodes_objects_dirs_and_truncation() {
        let doc = "<ListBucketResult><IsTruncated>false</IsTruncated>\
<Contents><Key>SWOT_L2_LR_SSH_D/cycle_500/a.nc</Key><LastModified>2026-01-01T00:00:00.000Z</LastModified><Size>125829120</Size></Contents>\
<CommonPrefixes><Prefix>SWOT_L2_LR_SSH_D/cycle_501/</Prefix></CommonPrefixes>\
</ListBucketResult>";
        let (objects, truncated, dirs) = parse_page(doc).unwrap();
        assert!(!truncated);
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].key, "SWOT_L2_LR_SSH_D/cycle_500/a.nc");
        assert_eq!(objects[0].size, 125_829_120);
        assert_eq!(dirs, vec!["SWOT_L2_LR_SSH_D/cycle_501/"]);
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
    fn day_temporal_spans_the_civil_day() {
        let (start, end) = day_temporal("2026.06.30").expect("civil day");
        assert_eq!(start, "2026-06-30T00:00:00Z");
        assert_eq!(end, "2026-06-30T23:59:59Z");
        assert!(day_temporal("not-a-day").is_none());
    }

    #[test]
    fn cmr_parse_takes_the_nc_s3_key_with_bucket() {
        let body = r#"{"feed":{"entry":[
          {"links":[
            {"rel":"http://esipfed.org/ns/fedsearch/1.1/s3#","href":"s3://podaac-swot-ops-cumulus-protected/SWOT_L2_LR_SSH_D/SWOT_L2_LR_SSH_Basic_001_001_20200101T000000_20200101T000100_PIC0_01.nc"},
            {"rel":"http://esipfed.org/ns/fedsearch/1.1/s3#","href":"s3://podaac-swot-ops-cumulus-protected/SWOT_L2_LR_SSH_D/x.log"},
            {"rel":"http://esipfed.org/ns/fedsearch/1.1/data#","href":"https://archive.swot.podaac.earthdata.nasa.gov/podaac-swot-ops-cumulus-protected/SWOT_L2_LR_SSH_D/x.nc"}
          ]}
        ]}}"#;
        let granules = cmr_parse(body).expect("feed parses");
        assert_eq!(granules.len(), 1);
        assert_eq!(granules[0].bucket, "podaac-swot-ops-cumulus-protected");
        assert_eq!(
            granules[0].key,
            "SWOT_L2_LR_SSH_D/SWOT_L2_LR_SSH_Basic_001_001_20200101T000000_20200101T000100_PIC0_01.nc"
        );
        assert_eq!(
            granules[0].data_url.as_deref(),
            Some(
                "https://archive.swot.podaac.earthdata.nasa.gov/podaac-swot-ops-cumulus-protected/SWOT_L2_LR_SSH_D/x.nc"
            )
        );
    }
}
