use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::{LeapSeconds, embedded_lsk, parse_iso_tdb};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Hdf5Attribute, Hdf5File, Hdf5Layout, decode_f32, decode_f64};
use omegaflow::lsk::days_from_civil;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

const NETLOC: &str = "product.gosat-gw.nies.go.jp";
const SEARCH_URL: &str = "https://product.gosat-gw.nies.go.jp/product_search/api/cui-search/";
const DOWNLOAD_URL: &str = "https://product.gosat-gw.nies.go.jp/product_search/api/cui-download/";
const MAGIC: [u8; 4] = *b"G3L1";
const REC_FIELDS: usize = 6;
const REC_BYTES: usize = REC_FIELDS * 8;
const SEARCH_BOUND_S: u64 = 1 << 7;
const CONNECT_BOUND_S: u64 = 1 << 5;
const DOWNLOAD_BOUND_S: u64 = 1 << 12;
const MAX_SEARCH_RESULTS: usize = 1 << 12;
const MAX_RECORDS: usize = 1 << 20;
const MAX_VAL_BYTES: u64 = 1 << 28;
const PROD_FOCUS: f64 = 1.0;
const PROD_WIDE: f64 = 2.0;
const BAND1: f64 = 1.0;
const BAND2: f64 = 2.0;
const BAND3: f64 = 3.0;
const RAD_MEDIAN_BOUND: f64 = 1.6384e4;

const BAND_GROUPS: [(&str, f64); 3] = [("Band1", BAND1), ("Band2", BAND2), ("Band3", BAND3)];
const COMMON_LAT: &str = "/Common/SoundingGeometry/latitude";
const COMMON_LON: &str = "/Common/SoundingGeometry/longitude";
const COMMON_OBS: &str = "/Common/FrameInfo/observationTime";

struct Granule {
    filename: String,
    start: Option<String>,
    end: Option<String>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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

fn gosat_cred() -> Option<(String, String)> {
    let mail = env::var("GOSAT_GW_MAIL")
        .ok()
        .filter(|t| !t.trim().is_empty());
    let pass = env::var("GOSAT_GW_PASS")
        .ok()
        .filter(|t| !t.trim().is_empty());
    if let (Some(m), Some(p)) = (mail, pass) {
        return Some((m.trim().to_string(), p.trim().to_string()));
    }
    let text = fs::read_to_string(".secrets.local").ok()?;
    let m = parse_secret(&text, "GOSAT_GW_MAIL")?;
    let p = parse_secret(&text, "GOSAT_GW_PASS")?;
    Some((m, p))
}

fn cookie_flag(mail: &str, pass: &str) -> String {
    format!("mail={mail}; password={pass}")
}

fn uri_encode_query(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn search_url(product: &str, start: &str, end: &str) -> String {
    let params: Vec<(String, String)> = vec![
        ("level".to_string(), "2".to_string()),
        ("mode".to_string(), "2".to_string()),
        ("product".to_string(), product.to_string()),
        ("format".to_string(), "2".to_string()),
        ("version".to_string(), "1".to_string()),
        ("area".to_string(), "1".to_string()),
        ("start".to_string(), start.to_string()),
        ("end".to_string(), end.to_string()),
    ];
    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", uri_encode_query(k), uri_encode_query(v)))
        .collect::<Vec<_>>()
        .join("&");
    format!("{SEARCH_URL}?{query}")
}

fn search_fetch(url: &str, cookie: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-m")
        .arg(SEARCH_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string())
        .arg("-b")
        .arg(cookie)
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!(
            "gosat search returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn search_parse(body: &str) -> Option<Vec<Granule>> {
    let json = parse_json(body)?;
    let JsonVal::Obj(root) = &json else {
        return None;
    };
    let JsonVal::Arr(items) = root.get("result")? else {
        let summary = match root.get("messages") {
            Some(JsonVal::Obj(m)) => match m.get("summary") {
                Some(JsonVal::Str(s)) => s.clone(),
                _ => String::new(),
            },
            _ => String::new(),
        };
        eprintln!("gosat search: no result array — server: {summary}");
        return None;
    };
    let mut out = Vec::new();
    for item in items {
        let JsonVal::Obj(o) = item else { continue };
        let filename = match o.get("filename") {
            Some(JsonVal::Str(s)) if !s.is_empty() => s.clone(),
            _ => continue,
        };
        let start = match o.get("obs_start_time") {
            Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
            _ => continue,
        };
        let end = match o.get("obs_end_time") {
            Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
            _ => continue,
        };
        out.push(Granule {
            filename,
            start,
            end,
        });
    }
    Some(out)
}

fn search_granules(product: &str, start: &str, end: &str, cookie: &str) -> Option<Vec<Granule>> {
    let url = search_url(product, start, end);
    search_parse(&search_fetch(&url, cookie)?)
}

fn download_fetch(filename: &str, cookie: &str, out_path: &str) -> bool {
    let url = format!("{DOWNLOAD_URL}?filename={}", uri_encode_query(filename));
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-m")
        .arg(DOWNLOAD_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string())
        .arg("-b")
        .arg(cookie)
        .arg("-o")
        .arg(out_path)
        .arg(url)
        .output();
    match out {
        Ok(o) if o.status.success() => match fs::metadata(out_path) {
            Ok(m) if m.len() > 0 => true,
            Ok(_) => {
                eprintln!("gosat download {filename}: zero bytes on disk");
                false
            }
            Err(_) => {
                eprintln!("gosat download {filename}: file absent after fetch");
                false
            }
        },
        Ok(o) => {
            eprintln!(
                "gosat download {filename} returned ({}): {}",
                o.status,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(_) => {
            eprintln!("gosat download {filename}: curl spawn returned void");
            false
        }
    }
}

fn anchor_tdb(start: &str, end: &str, lsk: &LeapSeconds) -> Option<f64> {
    let s = parse_iso_tdb(start, lsk)?;
    let e = parse_iso_tdb(end, lsk)?;
    if !(s.is_finite() && e.is_finite() && e > s) {
        return None;
    }
    Some((s + e) / 2.0)
}

fn product_code(filename: &str) -> Option<f64> {
    if filename.contains("O1F") {
        Some(PROD_FOCUS)
    } else if filename.contains("O1W") {
        Some(PROD_WIDE)
    } else {
        None
    }
}

fn class_name(class: u8) -> &'static str {
    match class {
        0 => "fixed",
        1 => "float",
        3 => "string",
        6 => "compound",
        7 => "array",
        8 => "enum",
        9 => "vlen",
        10 => "reference",
        _ => "other",
    }
}

fn layout_name(layout: &Option<Hdf5Layout>) -> String {
    match layout {
        Some(Hdf5Layout::Compact { .. }) => "compact".to_string(),
        Some(Hdf5Layout::Contiguous { .. }) => "contiguous".to_string(),
        Some(Hdf5Layout::Chunked { chunk_dims, .. }) => {
            format!("chunked({chunk_dims:?})")
        }
        None => "absent".to_string(),
    }
}

fn attr_text(attr: &Hdf5Attribute) -> Option<String> {
    match attr.datatype.class {
        3 => {
            let mut end = attr.data.len();
            while end > 0 && (attr.data[end - 1] == 0 || attr.data[end - 1] == b' ') {
                end -= 1;
            }
            let text = String::from_utf8_lossy(&attr.data[..end]).into_owned();
            if text.is_empty() { None } else { Some(text) }
        }
        _ => None,
    }
}

fn probe_hdf5(path: &str) {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("probe {path}: read returned void");
            return;
        }
    };
    if bytes.len() > MAX_VAL_BYTES as usize {
        eprintln!(
            "probe {path}: {} B over the {} B read bound",
            bytes.len(),
            MAX_VAL_BYTES
        );
        return;
    }
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(n) => {
            eprintln!("probe {path}: hdf5 parse void ({n:?})");
            return;
        }
    };
    let mut queue: Vec<(String, u64)> = vec![(
        "/".to_string(),
        match file.root() {
            Ok(o) => o.addr,
            Err(_) => u64::MAX,
        },
    )];
    let mut seen: HashMap<u64, bool> = HashMap::new();
    while let Some((path, addr)) = queue.pop() {
        if addr == u64::MAX || seen.insert(addr, true).is_some() {
            continue;
        }
        let obj = match file.resolve(&path) {
            Ok(o) => o,
            Err(n) => {
                eprintln!("probe {path}: resolve void ({n:?})");
                continue;
            }
        };
        for link in &obj.links {
            let child = if path == "/" {
                format!("/{}", link.name)
            } else {
                format!("{path}/{}", link.name)
            };
            queue.push((child.clone(), link.addr));
        }
        if obj.is_group {
            continue;
        }
        let dims = match &obj.dataspace {
            Some(ds) => format!("{:?}", ds.dims),
            None => "scalar".to_string(),
        };
        let dt = match &obj.datatype {
            Some(dt) => format!(
                "class={}({}) size={} endian={:?} signed={} precision={}",
                class_name(dt.class),
                dt.class,
                dt.size,
                dt.endian,
                dt.signed,
                dt.precision
            ),
            None => "dtype absent".to_string(),
        };
        let filters: Vec<String> = obj.filters.iter().map(|f| f.id.to_string()).collect();
        let attrs: Vec<String> = obj.attrs.iter().map(|a| a.name.clone()).collect();
        let units = obj
            .attrs
            .iter()
            .find(|a| a.name == "units")
            .and_then(attr_text);
        let fill = obj
            .attrs
            .iter()
            .find(|a| a.name == "_FillValue")
            .and_then(attr_num_text);
        let long = obj
            .attrs
            .iter()
            .find(|a| a.name == "long_name")
            .and_then(attr_text);
        eprintln!(
            "ds {path} dims={dims} {dt} layout={} filters=[{}] attrs=[{}] units={} fill={} long={}",
            layout_name(&obj.layout),
            filters.join(","),
            attrs.join(","),
            match units {
                Some(u) => u,
                None => "absent".to_string(),
            },
            match fill {
                Some(f) => f,
                None => "absent".to_string(),
            },
            match long {
                Some(l) => l,
                None => "absent".to_string(),
            }
        );
    }
}

fn attr_num_text(attr: &Hdf5Attribute) -> Option<String> {
    match attr.datatype.class {
        1 => match attr.datatype.size {
            4 => match decode_f32(&attr.data, 0, attr.datatype.endian) {
                Some(v) => Some(format!("{v}")),
                None => None,
            },
            8 => match decode_f64(&attr.data, 0, attr.datatype.endian) {
                Some(v) => Some(format!("{v}")),
                None => None,
            },
            _ => None,
        },
        0 => match attr.datatype.size {
            1 => attr.data.first().map(|b| format!("{b}")),
            2 => {
                let b = attr.data.get(0..2)?;
                Some(format!("{}", u16::from_le_bytes(b.try_into().ok()?)))
            }
            4 => {
                let b = attr.data.get(0..4)?;
                Some(format!("{}", u32::from_le_bytes(b.try_into().ok()?)))
            }
            _ => None,
        },
        _ => None,
    }
}

fn read_f64_flat(file: &Hdf5File, name: &str) -> Option<Vec<f64>> {
    let (obj, ds, dt) = file.dataset(name).ok()?;
    if obj.layout.is_none() {
        return None;
    }
    let raw = file.read_dataset(name).ok()?;
    let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    if count > MAX_RECORDS {
        eprintln!("dataset {name}: {count} elements over the record bound");
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let v = match dt.class {
            0 => decode_f64(&raw, i * dt.size, dt.endian)?,
            1 => match dt.size {
                8 => decode_f64(&raw, i * 8, dt.endian)?,
                4 => decode_f32(&raw, i * 4, dt.endian)? as f64,
                _ => return None,
            },
            _ => return None,
        };
        out.push(v);
    }
    Some(out)
}

fn first_of(file: &Hdf5File, names: &[String]) -> Option<Vec<f64>> {
    for name in names {
        if let Some(v) = read_f64_flat(file, name) {
            return Some(v);
        }
    }
    None
}

fn radiance_raw(file: &Hdf5File, group: &str) -> Option<(Vec<u8>, Vec<u64>, u8)> {
    let name = format!("/{group}/SoundingData/radiance");
    let (_obj, ds, dt) = file.dataset(&name).ok()?;
    if dt.class != 1 || dt.size != 4 {
        eprintln!(
            "harvest {name}: float32 expected, class {} size {}",
            dt.class, dt.size
        );
        return None;
    }
    let raw = file.read_dataset(&name).ok()?;
    Some((raw, ds.dims.clone(), dt.endian as u8))
}

fn epoch_unix_from_units(units: &str) -> Option<f64> {
    let since = units.find("since")?;
    let rest = units[since + "since".len()..].trim();
    let (date, time) = match rest.split_once('T') {
        Some((d, t)) => (d, Some(t)),
        None => match rest.split_once(' ') {
            Some((d, t)) => (d, Some(t)),
            None => (rest, None),
        },
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let m: i64 = dp.next()?.parse().ok()?;
    let d: i64 = dp.next()?.parse().ok()?;
    let days = days_from_civil(y, m, d)?;
    let secs: i64 = match time {
        Some(t) => {
            let t = t.trim();
            let mut tp = t.split([':', 'Z', 'z']);
            let hh: i64 = tp.next()?.parse().ok()?;
            let mm: i64 = match tp.next() {
                Some(v) => v.parse().ok()?,
                None => 0,
            };
            let ss: i64 = match tp.next() {
                Some(v) => v.parse().ok()?,
                None => 0,
            };
            hh * 3600 + mm * 60 + ss
        }
        None => 0,
    };
    Some(days as f64 * 86400.0 + secs as f64)
}

fn decode_rad_f32(raw: &[u8], off: usize, endian: u8) -> Option<f64> {
    let b = raw.get(off..off + 4)?;
    let bits = if endian == 0 {
        u32::from_le_bytes(b.try_into().ok()?)
    } else {
        u32::from_be_bytes(b.try_into().ok()?)
    };
    Some(f32::from_bits(bits) as f64)
}

fn frame_tdb(
    file: &Hdf5File,
    group: &str,
    nf: usize,
    anchor: Option<f64>,
    lsk: &LeapSeconds,
) -> Option<Vec<Option<f64>>> {
    let band_obs = format!("/{group}/FrameInfo/observationTime");
    let obs_name = match read_f64_flat(file, &band_obs) {
        Some(_) => band_obs,
        None => match read_f64_flat(file, COMMON_OBS) {
            Some(_) => COMMON_OBS.to_string(),
            None => {
                eprintln!(
                    "harvest {group}: no observationTime dataset under the measured names — {}",
                    match anchor {
                        Some(_) => "the window midpoint anchor carries the rows",
                        None => "no anchor exists, the band stays pending",
                    }
                );
                return anchor.map(|a| vec![Some(a); nf]);
            }
        },
    };
    let values = read_f64_flat(file, &obs_name)?;
    let units = file.attribute(&obs_name, "units").and_then(attr_text);
    let Some(units) = units else {
        eprintln!(
            "harvest {group}: {obs_name} carries no units attribute — {}",
            match anchor {
                Some(_) => "the window midpoint anchor carries the rows",
                None => "no anchor exists, the band stays pending",
            }
        );
        return anchor.map(|a| vec![Some(a); nf]);
    };
    let Some(epoch) = epoch_unix_from_units(&units) else {
        eprintln!(
            "harvest {group}: the units epoch {units} stays unparsed — {}",
            match anchor {
                Some(_) => "the window midpoint anchor carries the rows",
                None => "no anchor exists, the band stays pending",
            }
        );
        return anchor.map(|a| vec![Some(a); nf]);
    };
    let mut out = Vec::with_capacity(nf);
    for i in 0..nf {
        let t = match values.get(i).copied() {
            Some(t) if t.is_finite() => t,
            _ => {
                out.push(anchor);
                continue;
            }
        };
        out.push(lsk.unix_to_tdb(epoch + t));
    }
    Some(out)
}

fn harvest_band(
    file: &Hdf5File,
    group: &str,
    band: f64,
    product: f64,
    anchor: Option<f64>,
    lsk: &LeapSeconds,
) -> Vec<[f64; REC_FIELDS]> {
    let lat_names = vec![
        format!("/{group}/SoundingGeometry/latitude"),
        COMMON_LAT.to_string(),
    ];
    let lon_names = vec![
        format!("/{group}/SoundingGeometry/longitude"),
        COMMON_LON.to_string(),
    ];
    let Some(lat) = first_of(file, &lat_names) else {
        eprintln!("harvest {group}: no latitude dataset under the measured names");
        return Vec::new();
    };
    let Some(lon) = first_of(file, &lon_names) else {
        eprintln!("harvest {group}: no longitude dataset under the measured names");
        return Vec::new();
    };
    if lat.len() != lon.len() {
        eprintln!(
            "harvest {group}: latitude {} vs longitude {} — shapes refuse a common row",
            lat.len(),
            lon.len()
        );
        return Vec::new();
    }
    let Some((raw, dims, endian)) = radiance_raw(file, group) else {
        eprintln!("harvest {group}: the radiance dataset stays unread");
        return Vec::new();
    };
    if dims.len() != 3 {
        eprintln!(
            "harvest {group}: radiance rank {} — the [frame, spatial, spectral] shape is absent",
            dims.len()
        );
        return Vec::new();
    }
    let (nf, ns, nw) = (dims[0] as usize, dims[1] as usize, dims[2] as usize);
    if nf * ns != lat.len() {
        eprintln!(
            "harvest {group}: radiance frames×spatial {nf}x{ns} vs latitude {} — the axes refuse a pairing",
            lat.len()
        );
        return Vec::new();
    }
    let Some(tdb) = frame_tdb(file, group, nf, anchor, lsk) else {
        eprintln!("harvest {group}: the frame time fold returned void");
        return Vec::new();
    };
    let mut recs = Vec::new();
    for i in 0..nf {
        let Some(t) = tdb[i] else { continue };
        for j in 0..ns {
            let idx = i * ns + j;
            let la = lat[idx];
            let lo = lon[idx];
            if !(la.is_finite() && lo.is_finite()) {
                continue;
            }
            if !(-90.0..=90.0).contains(&la) || !(-180.0..=180.0).contains(&lo) {
                continue;
            }
            let mut channels = Vec::new();
            for w in 0..nw {
                let Some(v) = decode_rad_f32(&raw, (idx * nw + w) * 4, endian) else {
                    continue;
                };
                if v.is_finite() && v > 0.0 {
                    channels.push(v);
                }
            }
            if channels.is_empty() {
                continue;
            }
            channels.sort_by(|a, b| a.total_cmp(b));
            let mid = channels.len() / 2;
            let value = if channels.len().is_multiple_of(2) {
                (channels[mid - 1] + channels[mid]) / 2.0
            } else {
                channels[mid]
            };
            if !(value.is_finite() && value > 0.0 && value <= RAD_MEDIAN_BOUND) {
                continue;
            }
            recs.push([t, la, lo, value, band, product]);
        }
    }
    eprintln!(
        "harvest {group}: {nf} frames × {ns} spatial × {nw} spectral → {} rows",
        recs.len()
    );
    recs
}

fn harvest_hdf5(
    path: &str,
    product: f64,
    anchor: Option<f64>,
    lsk: &LeapSeconds,
) -> Vec<[f64; REC_FIELDS]> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("harvest {path}: read returned void");
            return Vec::new();
        }
    };
    if bytes.len() > MAX_VAL_BYTES as usize {
        eprintln!(
            "harvest {path}: {} B over the {} B read bound",
            bytes.len(),
            MAX_VAL_BYTES
        );
        return Vec::new();
    }
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(n) => {
            eprintln!("harvest {path}: hdf5 parse void ({n:?})");
            return Vec::new();
        }
    };
    let mut recs = Vec::new();
    for (group, band) in BAND_GROUPS {
        recs.extend(harvest_band(&file, group, band, product, anchor, lsk));
    }
    recs
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
        if !(r[4] == BAND1 || r[4] == BAND2 || r[4] == BAND3) {
            return None;
        }
        if !(r[5] == PROD_FOCUS || r[5] == PROD_WIDE) {
            return None;
        }
        out.push(r);
    }
    Some(out)
}

fn probe_values(path: &str, dataset: &str) {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("probe-values {path}: read returned void");
            return;
        }
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(n) => {
            eprintln!("probe-values {path}: hdf5 parse void ({n:?})");
            return;
        }
    };
    let (_obj, ds, dt) = match file.dataset(dataset) {
        Ok(v) => v,
        Err(n) => {
            eprintln!("probe-values {dataset}: dataset void ({n:?})");
            return;
        }
    };
    let raw = match file.read_dataset(dataset) {
        Ok(r) => r,
        Err(n) => {
            eprintln!("probe-values {dataset}: read void ({n:?})");
            return;
        }
    };
    let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let (mut n, mut min, mut max) = (0usize, f64::INFINITY, f64::NEG_INFINITY);
    let (mut n_pos, mut n_small, mut n_mid, mut n_big) = (0usize, 0usize, 0usize, 0usize);
    let mut first: Vec<f64> = Vec::new();
    for i in 0..count {
        let v = match dt.class {
            1 => match dt.size {
                4 => decode_f32(&raw, i * 4, dt.endian).map(|v| v as f64),
                8 => decode_f64(&raw, i * 8, dt.endian),
                _ => None,
            },
            0 => decode_f64(&raw, i * dt.size, dt.endian),
            _ => None,
        };
        let Some(v) = v else { continue };
        if !v.is_finite() {
            continue;
        }
        n += 1;
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
        if v > 0.0 {
            n_pos += 1;
            if v <= 1e4 {
                n_small += 1;
            } else if v <= 1e8 {
                n_mid += 1;
            } else {
                n_big += 1;
            }
        }
        if first.len() < 8 {
            first.push(v);
        }
    }
    eprintln!(
        "probe-values {dataset}: dims={:?} class={} size={} → {n} finite of {count}, min={min}, max={max}, pos={n_pos} (0..1e4: {n_small}, 1e4..1e8: {n_mid}, >1e8: {n_big}), first={first:?}",
        ds.dims, dt.class, dt.size
    );
}

fn probe_chunks(path: &str, dataset: &str) {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("probe-chunks {path}: read returned void");
            return;
        }
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(n) => {
            eprintln!("probe-chunks {path}: hdf5 parse void ({n:?})");
            return;
        }
    };
    let Some(index) = file.chunk_index(dataset) else {
        eprintln!("probe-chunks {dataset}: chunk index void");
        return;
    };
    for (coords, addr) in index {
        let Some(head) = bytes
            .get(addr as usize..(addr as usize + 8))
            .map(|s| s.to_vec())
        else {
            eprintln!("probe-chunks {dataset}: chunk {coords:?} at {addr} head absent (out of bounds)");
            continue;
        };
        eprintln!("probe-chunks {dataset}: chunk {coords:?} at {addr} head={head:02x?}");
    }
}

fn probe_inflate(path: &str, dataset: &str) {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("probe-inflate {path}: read returned void");
            return;
        }
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(n) => {
            eprintln!("probe-inflate {path}: hdf5 parse void ({n:?})");
            return;
        }
    };
    let (_obj, _ds, dt) = match file.dataset(dataset) {
        Ok(v) => v,
        Err(n) => {
            eprintln!("probe-inflate {dataset}: dataset void ({n:?})");
            return;
        }
    };
    if dt.class != 1 || dt.size != 4 {
        eprintln!("probe-inflate {dataset}: float32 expected");
        return;
    }
    let Some(index) = file.chunk_index(dataset) else {
        eprintln!("probe-inflate {dataset}: chunk index void");
        return;
    };
    for (coords, addr) in index {
        let end = match (addr as usize).checked_add(1 << 22) {
            Some(e) if e <= bytes.len() => e,
            _ => bytes.len(),
        };
        let window = &bytes[addr as usize..end];
        let body = if window.len() >= 2
            && window[0] & 0x0f == 8
            && ((window[0] as u16) << 8 | window[1] as u16).is_multiple_of(31)
        {
            let skip = if window[1] & 0x20 != 0 { 6 } else { 2 };
            &window[skip..]
        } else {
            window
        };
        let out = omegaflow::archivar::inflate::inflate(body);
        let (out_len, first, head, unshuffled): (usize, Vec<f64>, Vec<u8>, Vec<f64>) = match &out {
            Some(o) => {
                let mut v = Vec::new();
                for i in 0..8 {
                    match decode_f32(o, i * 4, dt.endian) {
                        Some(x) => v.push(x as f64),
                        None => break,
                    }
                }
                let n = o.len() / 4;
                let mut un = Vec::new();
                for i in 0..8 {
                    let mut b = [0u8; 4];
                    let mut ok = true;
                    for (slot, k) in b.iter_mut().zip([i, i + n, i + 2 * n, i + 3 * n]) {
                        match o.get(k) {
                            Some(&x) => *slot = x,
                            None => ok = false,
                        }
                    }
                    if !ok {
                        break;
                    }
                    un.push(f32::from_le_bytes(b) as f64);
                }
                (o.len(), v, o.iter().take(16).copied().collect(), un)
            }
            None => (0, Vec::new(), Vec::new(), Vec::new()),
        };
        eprintln!(
            "probe-inflate {dataset}: chunk {coords:?} at {addr} window {} B → {out_len} B, head={head:02x?}, first={first:?}, unshuffled_first={unshuffled:?}",
            window.len()
        );
    }
}

fn probe_rows(path: &str, dataset: &str) {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("probe-rows {path}: read returned void");
            return;
        }
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(n) => {
            eprintln!("probe-rows {path}: hdf5 parse void ({n:?})");
            return;
        }
    };
    let (_obj, ds, dt) = match file.dataset(dataset) {
        Ok(v) => v,
        Err(n) => {
            eprintln!("probe-rows {dataset}: dataset void ({n:?})");
            return;
        }
    };
    if ds.dims.len() != 3 || dt.class != 1 || dt.size != 4 {
        eprintln!("probe-rows {dataset}: [nf, ns, nw] float32 expected");
        return;
    }
    let raw = match file.read_dataset(dataset) {
        Ok(r) => r,
        Err(n) => {
            eprintln!("probe-rows {dataset}: read void ({n:?})");
            return;
        }
    };
    let (nf, ns, nw) = (
        ds.dims[0] as usize,
        ds.dims[1] as usize,
        ds.dims[2] as usize,
    );
    let endian = dt.endian as u8;
    let mut shown = 0usize;
    for i in 0..nf {
        for j in 0..ns {
            if shown >= 24 {
                return;
            }
            let idx = i * ns + j;
            let mut small = 0usize;
            let mut big = 0usize;
            let mut channels = Vec::new();
            for w in 0..nw {
                let Some(v) = decode_rad_f32(&raw, (idx * nw + w) * 4, endian) else {
                    continue;
                };
                if v.is_finite() && v > 0.0 {
                    channels.push(v);
                    if v <= 1e4 {
                        small += 1;
                    } else {
                        big += 1;
                    }
                }
            }
            if channels.is_empty() {
                eprintln!("probe-rows {dataset}: row ({i},{j}) void");
                shown += 1;
                continue;
            }
            channels.sort_by(|a, b| a.total_cmp(b));
            let mid = channels.len() / 2;
            let median = if channels.len().is_multiple_of(2) {
                (channels[mid - 1] + channels[mid]) / 2.0
            } else {
                channels[mid]
            };
            eprintln!(
                "probe-rows {dataset}: row ({i},{j}) {small} small {big} big median {median}",
            );
            shown += 1;
        }
    }
}

fn run(args: &[String]) {
    if let Some(path) = arg_value(args, "--probe") {
        probe_hdf5(&path);
        return;
    }
    if let Some(path) = arg_value(args, "--probe-rows") {
        let Some(dataset) = arg_value(args, "--dataset") else {
            eprintln!("gosat_tanso3_compiler: --probe-rows carries no --dataset — refused");
            std::process::exit(2);
        };
        probe_rows(&path, &dataset);
        return;
    }
    if let Some(path) = arg_value(args, "--probe-inflate") {
        let Some(dataset) = arg_value(args, "--dataset") else {
            eprintln!("gosat_tanso3_compiler: --probe-inflate carries no --dataset — refused");
            std::process::exit(2);
        };
        probe_inflate(&path, &dataset);
        return;
    }
    if let Some(path) = arg_value(args, "--probe-chunks") {
        let Some(dataset) = arg_value(args, "--dataset") else {
            eprintln!("gosat_tanso3_compiler: --probe-chunks carries no --dataset — refused");
            std::process::exit(2);
        };
        probe_chunks(&path, &dataset);
        return;
    }
    if let Some(path) = arg_value(args, "--probe-values") {
        let Some(dataset) = arg_value(args, "--dataset") else {
            eprintln!("gosat_tanso3_compiler: --probe-values carries no --dataset — refused");
            std::process::exit(2);
        };
        probe_values(&path, &dataset);
        return;
    }
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => {
            eprintln!(
                "gosat_tanso3_compiler: --out <file.bin> absent — the output path is never silent"
            );
            std::process::exit(2);
        }
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some((mail, pass)) = gosat_cred() else {
        eprintln!(
            "gosat_tanso3_compiler: GOSAT_GW_MAIL/GOSAT_GW_PASS absent — the environment and .secrets.local carry no credential"
        );
        std::process::exit(2);
    };
    let cookie = cookie_flag(&mail, &pass);
    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "gosat_tanso3_compiler: the embedded naif0012.tls leap table is absent — no time anchor folds to the TDB clock"
        );
        std::process::exit(1);
    };
    let granules: Vec<Granule> = if args.iter().any(|a| a == "--search") {
        let Some(product) = arg_value(args, "--product") else {
            eprintln!("gosat_tanso3_compiler: --search carries no --product — refused");
            std::process::exit(2);
        };
        let Some(start) = arg_value(args, "--start") else {
            eprintln!("gosat_tanso3_compiler: --search carries no --start — refused");
            std::process::exit(2);
        };
        let Some(end) = arg_value(args, "--end") else {
            eprintln!("gosat_tanso3_compiler: --search carries no --end — refused");
            std::process::exit(2);
        };
        let Some(found) = search_granules(&product, &start, &end, &cookie) else {
            eprintln!(
                "gosat_tanso3_compiler: the search for {product} {start}..{end} returned void"
            );
            std::process::exit(1);
        };
        if found.is_empty() {
            eprintln!(
                "gosat_tanso3_compiler: no granules for {product} {start}..{end} — nothing fabricated"
            );
            std::process::exit(1);
        }
        if found.len() > MAX_SEARCH_RESULTS {
            eprintln!(
                "gosat_tanso3_compiler: {product} {start}..{end} carries {} granules — over the {} granule bound; narrow the window",
                found.len(),
                MAX_SEARCH_RESULTS
            );
            std::process::exit(1);
        }
        let mut found = found;
        found.sort_by(|a, b| a.filename.cmp(&b.filename));
        if let Some(limit) = args.iter().position(|a| a == "--limit") {
            let n = match args.get(limit + 1).and_then(|v| v.parse::<usize>().ok()) {
                Some(n) => n,
                None => {
                    eprintln!("gosat_tanso3_compiler: --limit carries no count — refused");
                    std::process::exit(2);
                }
            };
            found.truncate(n);
        }
        found
    } else {
        let direct: Vec<String> = args
            .iter()
            .enumerate()
            .filter(|(_, a)| a.as_str() == "--filename")
            .filter_map(|(i, _)| args.get(i + 1))
            .cloned()
            .collect();
        if direct.is_empty() {
            eprintln!(
                "usage: gosat_tanso3_compiler (--search --product <P> --start <YYYY-MM-DD> --end <YYYY-MM-DD> [--limit N] | --filename <name|path> [...]) --out <file.bin> [--ci-mode] — refused"
            );
            std::process::exit(2);
        }
        direct
            .into_iter()
            .map(|f| Granule {
                filename: f.clone(),
                start: None,
                end: None,
            })
            .collect()
    };
    let mut recs: Vec<[f64; REC_FIELDS]> = Vec::new();
    for g in &granules {
        let Some(product) = product_code(&g.filename) else {
            eprintln!(
                "gosat_tanso3: {} carries no JO1F/JO1W mode token",
                g.filename
            );
            continue;
        };
        let anchor: Option<f64> = match (&g.start, &g.end) {
            (Some(s), Some(e)) => anchor_tdb(s, e, &lsk),
            _ => None,
        };
        let tmp = format!(
            "{}/gosat_tanso3_{}",
            std::env::temp_dir().display(),
            g.filename
        );
        let local = if std::path::Path::new(&g.filename).is_file() {
            g.filename.clone()
        } else if std::path::Path::new(&tmp).is_file() {
            tmp.clone()
        } else {
            if !download_fetch(&g.filename, &cookie, &tmp) {
                eprintln!("gosat_tanso3: {} download void — stays pending", g.filename);
                continue;
            }
            tmp.clone()
        };
        let before = recs.len();
        recs.extend(harvest_hdf5(&local, product, anchor, &lsk));
        let _ = fs::remove_file(&tmp);
        eprintln!(
            "gosat_tanso3: {} → {} records (anchor {})",
            g.filename,
            recs.len() - before,
            match anchor {
                Some(a) => format!("{a:.3}"),
                None => "absent".to_string(),
            }
        );
    }
    recs.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if recs.is_empty() {
        eprintln!("gosat_tanso3_compiler: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = pack(&recs);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() && fs::create_dir_all(parent).is_err() {
            eprintln!(
                "gosat_tanso3_compiler: create_dir_all for {} returned void",
                parent.display()
            );
            std::process::exit(1);
        }
    }
    if fs::write(&out_path, &bytes).is_err() {
        eprintln!("gosat_tanso3_compiler: write {} returned void", out_path);
        std::process::exit(1);
    }
    match unpack(&bytes) {
        Some(parsed) => eprintln!(
            "gosat_tanso3_compiler: {} {} records, {} B, roundtrip parses",
            out_path,
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("gosat_tanso3_compiler: {} roundtrip parse void", out_path);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    run(&args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_parse_reads_the_measured_result_shape() {
        let body = r#"{"result": [
            {"filename": "TANSO3_202508151758008JO1F35004100_1BO00_101101.h5",
             "obs_start_time": "2025-08-15 17:58:07",
             "obs_end_time": "2025-08-15 17:58:21",
             "version": "101101", "path_no": "", "filesize": "17440691",
             "product_quality": "Good"},
            {"filename": "TANSO3_202512301358001NO1W35001400_1BO00_101101.h5",
             "obs_start_time": "2025-12-30 13:58:27",
             "obs_end_time": "2025-12-30 13:58:41",
             "version": "101101", "path_no": "", "filesize": "17276783",
             "product_quality": "Good"}
        ]}"#;
        let granules = search_parse(body).expect("the result feed parses");
        assert_eq!(granules.len(), 2);
        assert_eq!(
            granules[0].filename,
            "TANSO3_202508151758008JO1F35004100_1BO00_101101.h5"
        );
        assert_eq!(granules[0].start.as_deref(), Some("2025-08-15 17:58:07"));
        assert_eq!(granules[0].end.as_deref(), Some("2025-08-15 17:58:21"));
        assert_eq!(
            granules[1].filename,
            "TANSO3_202512301358001NO1W35001400_1BO00_101101.h5"
        );
    }

    #[test]
    fn search_parse_names_the_server_overflow_message() {
        let body = r#"{"status": "success", "results": {}, "messages": {"level": "info",
            "summary": "The number of search results exceeds the maximum number (3000). Please change your search conditions and try again.", "details": ""}}"#;
        assert!(search_parse(body).is_none());
    }

    #[test]
    fn search_url_carries_the_measured_query() {
        assert_eq!(
            search_url("GWT3F_L1B", "2025-08-01", "2025-08-31"),
            "https://product.gosat-gw.nies.go.jp/product_search/api/cui-search/?level=2&mode=2&product=GWT3F_L1B&format=2&version=1&area=1&start=2025-08-01&end=2025-08-31"
        );
    }

    #[test]
    fn product_code_reads_the_mode_token() {
        assert_eq!(
            product_code("TANSO3_202508151758008JO1F35004100_1BO00_101101.h5"),
            Some(PROD_FOCUS)
        );
        assert_eq!(
            product_code("TANSO3_202512300223021NO1F35000100_1BO00_101101.h5"),
            Some(PROD_FOCUS)
        );
        assert_eq!(
            product_code("TANSO3_202508112210016IO1WD5001400_1BO00_101101.h5"),
            Some(PROD_WIDE)
        );
        assert!(product_code("TANSO3_X").is_none());
    }

    #[test]
    fn anchor_folds_the_window_midpoint_to_tdb() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let a = anchor_tdb("2025-08-15 17:58:07", "2025-08-15 17:58:21", &lsk)
            .expect("the midpoint folds");
        let s = parse_iso_tdb("2025-08-15 17:58:07", &lsk).expect("start folds");
        let e = parse_iso_tdb("2025-08-15 17:58:21", &lsk).expect("end folds");
        assert!(s < a && a < e);
        assert!((a - (s + e) / 2.0).abs() < 1e-9);
        assert!(anchor_tdb("2025-08-15 17:58:21", "2025-08-15 17:58:07", &lsk).is_none());
    }

    #[test]
    fn pack_unpack_roundtrip() {
        let recs = vec![
            [750_000_000.0, 35.41, 139.41, 1.5, BAND1, PROD_FOCUS],
            [750_000_001.0, -20.25, 122.55, 2.5, BAND3, PROD_WIDE],
        ];
        let bytes = pack(&recs);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        let parsed = unpack(&bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], recs[0]);
        assert_eq!(parsed[1], recs[1]);
    }

    #[test]
    fn unpack_rejects_foreign_bytes() {
        assert!(unpack(b"X").is_none());
        assert!(unpack(b"G3L2abcd").is_none());
        let mut short = pack(&[[1.0, 2.0, 3.0, 4.0, BAND1, PROD_FOCUS]]);
        short.truncate(short.len() - 1);
        assert!(unpack(&short).is_none());
        let foreign_band = pack(&[[1.0, 2.0, 3.0, 4.0, 7.0, PROD_FOCUS]]);
        assert!(unpack(&foreign_band).is_none());
        let foreign_prod = pack(&[[1.0, 2.0, 3.0, 4.0, BAND1, 7.0]]);
        assert!(unpack(&foreign_prod).is_none());
    }

    #[test]
    fn epoch_units_fold_to_unix() {
        let epoch = epoch_unix_from_units("seconds since 2012-12-31T23:59:59Z")
            .expect("the measured units string folds");
        assert_eq!(epoch, 1_356_998_399.0);
        let epoch2 = epoch_unix_from_units("seconds since 2013-01-01 00:00:00")
            .expect("the spaced units string folds");
        assert_eq!(epoch2, 1_356_998_400.0);
        assert!(epoch_unix_from_units("no since here").is_none());
    }
}
