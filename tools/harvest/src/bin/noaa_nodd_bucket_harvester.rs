use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_NRS_PSD, MAGIC_NRS};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File, Hdf5Object};
use omegaflow::lsk::parse as parse_lsk;
use std::env;
use std::fs;

const NETLOC: &str = "storage.googleapis.com";

const DEFAULT_BUCKET: &str = "noaa-passive-bioacoustic";
const DEFAULT_PREFIX: &str = "nrs/products/";
const STATIONS_TABLE: &str = "phi/nrs_stations.φ";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name).and_then(|v| v.parse::<usize>().ok())
}

fn curl(url: &str) -> Option<String> {
    let out = std::process::Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        None
    }
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = std::process::Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn gt_after(s: &str, from: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = from;
    let mut in_q = false;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => in_q = !in_q,
            b'>' if !in_q => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

fn tag_local(s: &str, from: usize) -> Option<(String, bool, usize)> {
    let b = s.as_bytes();
    if b.get(from) != Some(&b'<') {
        return None;
    }
    let mut i = from + 1;
    let mut closing = false;
    if b.get(i) == Some(&b'/') {
        closing = true;
        i += 1;
    }
    let start = i;
    while i < s.len() {
        match b[i] {
            b' ' | b'>' | b'/' | b'\t' | b'\r' | b'\n' => break,
            _ => i += 1,
        }
    }
    if i == start {
        return None;
    }
    let raw = &s[start..i];
    let local = raw.rsplit(':').next().unwrap_or("").to_string();
    let gt = gt_after(s, i)?;
    Some((local, closing, gt))
}

fn skip_to_lt(s: &str, from: usize) -> usize {
    match s[from..].find('<') {
        Some(p) => from + p,
        None => s.len(),
    }
}

fn child_value(block: &str, local: &str) -> String {
    let mut i = 0usize;
    while i < block.len() {
        i = skip_to_lt(block, i);
        if i >= block.len() {
            return String::new();
        }
        let Some((name, closing, gt)) = tag_local(block, i) else {
            return String::new();
        };
        if !closing && name == local {
            let start = gt + 1;
            let end = block[start..]
                .find('<')
                .map(|p| start + p)
                .unwrap_or(block.len());
            return block[start..end].trim().to_string();
        }
        i = gt + 1;
    }
    String::new()
}

fn extract_blocks<'a>(s: &'a str, local: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < s.len() {
        i = skip_to_lt(s, i);
        if i >= s.len() {
            break;
        }
        let Some((name, closing, gt)) = tag_local(s, i) else {
            i += 1;
            continue;
        };
        if !closing && name == local {
            let start = gt + 1;
            let mut j = start;
            let mut close_gt: Option<usize> = None;
            while j < s.len() {
                j = skip_to_lt(s, j);
                if j >= s.len() {
                    break;
                }
                if let Some((cname, cclosing, cgt)) = tag_local(s, j) {
                    if cclosing && cname == local {
                        close_gt = Some(cgt);
                        break;
                    }
                    j = cgt + 1;
                } else {
                    j += 1;
                }
            }
            if let Some(cgt) = close_gt {
                out.push(&s[start..cgt]);
                i = cgt + 1;
                continue;
            }
        }
        i = gt + 1;
    }
    out
}

fn root_value(s: &str, local: &str) -> String {
    child_value(s, local)
}

struct Object {
    key: String,
    size: Option<u64>,
    modified: String,
}

struct Page {
    objects: Vec<Object>,
    common: Vec<String>,
    truncated: bool,
    next_marker: String,
}

fn page(bucket: &str, prefix: &str, marker: &str) -> Option<Page> {
    let mut url = format!(
        "https://storage.googleapis.com/{}?prefix={}&delimiter=/&max-keys=1000",
        bucket, prefix
    );
    if !marker.is_empty() {
        url.push_str("&marker=");
        url.push_str(marker);
    }
    let body = curl(&url)?;
    let mut objects = Vec::new();
    for blk in extract_blocks(&body, "Contents") {
        objects.push(Object {
            key: child_value(blk, "Key"),
            size: child_value(blk, "Size").parse::<u64>().ok(),
            modified: child_value(blk, "LastModified"),
        });
    }
    let common: Vec<String> = extract_blocks(&body, "CommonPrefixes")
        .iter()
        .map(|b| child_value(b, "Prefix"))
        .filter(|p| !p.is_empty())
        .collect();
    let truncated = root_value(&body, "IsTruncated").eq_ignore_ascii_case("true");
    Some(Page {
        objects,
        common,
        truncated,
        next_marker: root_value(&body, "NextMarker"),
    })
}

struct Walk {
    objects: usize,
    dirs: usize,
}

fn walk(bucket: &str, prefix: &str, depth: usize, cap: usize, out: &mut String) -> Walk {
    let mut w = Walk {
        objects: 0,
        dirs: 0,
    };
    let mut marker = String::new();
    loop {
        let p = match page(bucket, prefix, &marker) {
            Some(p) => p,
            None => break,
        };
        if w.dirs == 0 {
            w.dirs = p.common.len();
        } else {
            w.dirs += p.common.len();
        }
        for o in p.objects {
            if w.objects >= cap {
                return w;
            }
            let size = match o.size {
                Some(s) => s.to_string(),
                None => String::new(),
            };
            out.push_str(&format!("{}|{}|{}\n", o.key, size, o.modified));
            w.objects += 1;
        }
        if depth > 0 {
            for dir in &p.common {
                let sub = walk(bucket, dir, depth - 1, cap - w.objects, out);
                w.objects += sub.objects;
                w.dirs += sub.dirs;
                if w.objects >= cap {
                    return w;
                }
            }
        }
        if !p.truncated {
            break;
        }
        if p.next_marker.is_empty() {
            break;
        }
        marker = p.next_marker;
    }
    w
}

fn object_url(bucket: &str, key: &str) -> String {
    format!("https://storage.googleapis.com/{}/{}", bucket, key)
}

fn elem_f64(raw: &[u8], idx: usize, class: u8, size: usize, endian: Endian) -> Option<f64> {
    let off = idx.checked_mul(size)?;
    let b = raw.get(off..off + size)?;
    let le = endian == Endian::Le;
    match (class, size) {
        (0, 1) => Some(b[0] as i8 as f64),
        (0, 2) => {
            let v = if le {
                i16::from_le_bytes([b[0], b[1]])
            } else {
                i16::from_be_bytes([b[0], b[1]])
            };
            Some(v as f64)
        }
        (0, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if le {
                i32::from_le_bytes(arr)
            } else {
                i32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (0, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if le {
                i64::from_le_bytes(arr)
            } else {
                i64::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if le {
                f32::from_le_bytes(arr)
            } else {
                f32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if le {
                f64::from_le_bytes(arr)
            } else {
                f64::from_be_bytes(arr)
            };
            Some(v)
        }
        _ => None,
    }
}

fn cell(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => {
            if x.fract() == 0.0 && x.abs() < 1.0e15 {
                format!("{}", x as i64)
            } else {
                format!("{}", x)
            }
        }
        _ => String::new(),
    }
}

fn attr_text(obj: &Hdf5Object, name: &str) -> Option<String> {
    let a = obj.attrs.iter().find(|a| a.name == name)?;
    let s = String::from_utf8_lossy(&a.data);
    let t = s.trim_end_matches('\0').trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

struct DsLoad {
    raw: Vec<u8>,
    class: u8,
    size: usize,
    endian: Endian,
    dims: Vec<u64>,
}

fn ds_load(file: &Hdf5File, name: &str) -> Option<DsLoad> {
    let (obj, ds, dt) = file.dataset(name).ok()?;
    if obj.is_group {
        return None;
    }
    let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset(name)))
        .ok()?
        .ok()?;
    Some(DsLoad {
        raw,
        class: dt.class,
        size: dt.size,
        endian: dt.endian,
        dims: ds.dims.clone(),
    })
}

fn raw_elems(raw_len: usize, size: usize) -> usize {
    if size == 0 {
        0
    } else {
        raw_len / size
    }
}

fn wkt_point(s: &str) -> Option<(f64, f64)> {
    let i = s.find('(')?;
    let j = s[i..].find(')').map(|k| i + k)?;
    let mut toks = s[i + 1..j].split_whitespace();
    let a = toks.next()?.parse::<f64>().ok()?;
    let b = toks.next()?.parse::<f64>().ok()?;
    Some((a, b))
}

fn shape_of(body: &str) -> Option<(f64, f64)> {
    let i = body.find("\"SHAPE\"")?;
    wkt_point(&body[i..])
}

fn nrs_site_key(site: &str) -> Option<String> {
    let digits: String = site.chars().filter(|c| c.is_ascii_digit()).collect();
    let n = digits.parse::<u32>().ok()?;
    if (1..=99).contains(&n) {
        Some(format!("{:02}", n))
    } else {
        None
    }
}

fn station_from_key(key: &str) -> Option<String> {
    for seg in key.split('/') {
        let lower = seg.to_ascii_lowercase();
        if let Some(i) = lower.find("nrs") {
            let after = &lower[i + 3..];
            let mut digits = String::new();
            for ch in after.chars() {
                if ch.is_ascii_digit() {
                    digits.push(ch);
                } else if !digits.is_empty() {
                    break;
                }
            }
            if !digits.is_empty() {
                return Some(digits);
            }
        }
    }
    None
}

fn parse_station_table(text: &str) -> Vec<(String, f64, f64)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(site), Some(lat), Some(lon)) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let (Ok(lat), Ok(lon)) = (lat.parse::<f64>(), lon.parse::<f64>()) else {
            continue;
        };
        if lat.is_finite() && lon.is_finite() {
            out.push((site.to_string(), lat, lon));
        }
    }
    out
}

fn station_position(
    body: &str,
    site: Option<&str>,
    rows: &[(String, f64, f64)],
) -> Option<(f64, f64)> {
    if let Some((lon, lat)) = shape_of(body) {
        return Some((lat, lon));
    }
    let site = site?;
    let key = nrs_site_key(site)?;
    rows.iter()
        .find(|(s, _, _)| nrs_site_key(s).as_deref() == Some(key.as_str()))
        .map(|(_, lat, lon)| (*lat, *lon))
}

fn parse_positive_lead(s: &str) -> Option<f64> {
    let t = s.trim_start();
    let bytes = t.as_bytes();
    let mut i = 0usize;
    let mut seen_dot = false;
    let mut seen_exp = false;
    if bytes.first() == Some(&b'+') || bytes.first() == Some(&b'-') {
        i = 1;
    }
    let mut got = false;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() {
            got = true;
            i += 1;
        } else if b == b'.' && !seen_dot && !seen_exp {
            seen_dot = true;
            i += 1;
        } else if (b == b'e' || b == b'E') && !seen_exp && got {
            seen_exp = true;
            i += 1;
            if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
                i += 1;
            }
        } else {
            break;
        }
    }
    if !got {
        return None;
    }
    let v = t[..i].parse::<f64>().ok()?;
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

fn depth_value(body: &str, key: &str) -> Option<f64> {
    let mut from = 0usize;
    while let Some(p) = body[from..].find(key) {
        let at = from + p + key.len();
        let tail = &body[at..(at + 120).min(body.len())];
        let c = tail.chars().next()?;
        if !matches!(c, ':' | '>' | '=' | '"' | ' ' | '\t' | '\n' | '\r' | '[') {
            from = at + key.len();
            continue;
        }
        let v = tail.trim_start_matches([':', '>', '=', ' ', '"', '\t', '\r', '\n', '[', '{']);
        if v.starts_with('"') || v.starts_with('}') || v.starts_with(',') {
            from = at + key.len();
            continue;
        }
        let seek = match v.find(|ch: char| ch == '-' || ch.is_ascii_digit()) {
            Some(s) => s,
            None => {
                from = at + key.len();
                continue;
            }
        };
        if let Some(d) = parse_positive_lead(&v[seek..]) {
            return Some(d);
        }
        from = at + key.len();
    }
    None
}

fn depth_from_metadata_text(body: &str) -> Option<f64> {
    for key in ["DepthInstrument_m", "DEPLOY_INSTRUMENT_DEPTH"] {
        if let Some(v) = depth_value(body, key) {
            return Some(v);
        }
    }
    None
}

fn u16_le(b: &[u8], off: usize) -> u16 {
    b[off] as u16 | ((b[off + 1] as u16) << 8)
}

fn u32_le(b: &[u8], off: usize) -> u32 {
    b[off] as u32
        | ((b[off + 1] as u32) << 8)
        | ((b[off + 2] as u32) << 16)
        | ((b[off + 3] as u32) << 24)
}

fn zip_members(bytes: &[u8]) -> Option<Vec<(String, u16, usize, usize)>> {
    let n = bytes.len();
    if n < 22 {
        return None;
    }
    let mut eocd: Option<usize> = None;
    let mut i = n.saturating_sub(4);
    let lo = n.saturating_sub(65557);
    while i >= lo {
        if bytes.get(i..i + 4) == Some(&[0x50, 0x4b, 0x05, 0x06]) {
            eocd = Some(i);
            break;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    let e = eocd?;
    let cd_size = u32_le(bytes, e + 12) as usize;
    let cd_off = u32_le(bytes, e + 16) as usize;
    let cd = bytes.get(cd_off..cd_off + cd_size)?;
    let mut members = Vec::new();
    let mut p = 0usize;
    while p + 46 <= cd.len() {
        if cd.get(p..p + 4) == Some(&[0x50, 0x4b, 0x01, 0x02]) {
            let method = u16_le(cd, p + 10);
            let comp = u32_le(cd, p + 20) as usize;
            let nlen = u16_le(cd, p + 28) as usize;
            let xlen = u16_le(cd, p + 30) as usize;
            let clen = u16_le(cd, p + 32) as usize;
            let local = u32_le(cd, p + 42) as usize;
            let name = String::from_utf8_lossy(&cd[p + 46..p + 46 + nlen]).into_owned();
            members.push((name, method, comp, local));
            p += 46 + nlen + xlen + clen;
        } else {
            p += 1;
        }
    }
    if members.is_empty() {
        None
    } else {
        Some(members)
    }
}

fn xlsx_entry(
    bytes: &[u8],
    members: &[(String, u16, usize, usize)],
    name: &str,
) -> Option<Vec<u8>> {
    let (_, method, comp, local) = members.iter().find(|(m, ..)| m == name)?;
    if *local + 30 > bytes.len() {
        return None;
    }
    let nlen = u16_le(bytes, local + 26) as usize;
    let xlen = u16_le(bytes, local + 28) as usize;
    let data_start = local + 30 + nlen + xlen;
    let payload = bytes.get(data_start..data_start + comp)?;
    match method {
        0 => Some(payload.to_vec()),
        8 => omegaflow::inflate::inflate(payload),
        _ => None,
    }
}

fn shared_index(shared: &str, want: &str) -> Option<usize> {
    let mut idx = 0usize;
    let mut rest = shared;
    while let Some(p) = rest.find("<si>") {
        let body = &rest[p + 4..];
        let end = body.find("</si>")?;
        let cell = &body[..end];
        let t = if let Some(ts) = cell.find("<t") {
            let gt = cell[ts..].find('>').map(|g| ts + g + 1)?;
            let te = cell[gt..].find("</t>").map(|e| gt + e)?;
            cell[gt..te].trim().to_string()
        } else {
            String::new()
        };
        if t == want {
            return Some(idx);
        }
        idx += 1;
        rest = &body[end + 5..];
    }
    None
}

fn xml_v(s: &str) -> Option<String> {
    let p = s.find("<v>")?;
    let tail = &s[p + 3..];
    let e = tail.find("</v>")?;
    Some(tail[..e].trim().to_string())
}

fn col_of_ref(cr: &str) -> Option<&str> {
    let d = cr.find(|c: char| c.is_ascii_digit())?;
    if d == 0 {
        None
    } else {
        Some(&cr[..d])
    }
}

fn row_of_ref(cr: &str) -> Option<usize> {
    let d = cr.find(|c: char| c.is_ascii_digit())?;
    cr[d..].parse::<usize>().ok()
}

fn sheet_cells(sheet: &str) -> Vec<(String, Option<usize>, Option<f64>)> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while let Some(rel) = sheet[pos..].find("<c ") {
        let cs = pos + rel + 3;
        let Some(gt_rel) = sheet[cs..].find('>') else {
            break;
        };
        let gt = cs + gt_rel;
        let attrs = &sheet[cs..gt];
        let self_close = attrs.ends_with('/');
        let (inner, next) = if self_close {
            (String::new(), gt + 1)
        } else {
            let body_start = gt + 1;
            match sheet[body_start..].find("</c>") {
                Some(cc) => (
                    sheet[body_start..body_start + cc].to_string(),
                    body_start + cc + 4,
                ),
                None => (String::new(), sheet.len()),
            }
        };
        let mut cr = String::new();
        if let Some(rp) = attrs.find("r=\"") {
            let ra = &attrs[rp + 3..];
            if let Some(qe) = ra.find('"') {
                cr = ra[..qe].to_string();
            }
        }
        let shared = if attrs.contains("t=\"s\"") {
            xml_v(&inner).and_then(|v| v.parse::<usize>().ok())
        } else {
            None
        };
        let num = if attrs.contains("t=\"s\"") {
            None
        } else {
            xml_v(&inner).and_then(|v| parse_positive_lead(&v))
        };
        out.push((cr, shared, num));
        pos = next;
        if next >= sheet.len() {
            break;
        }
    }
    out
}

fn xlsx_depth_m(bytes: &[u8]) -> Option<f64> {
    let members = zip_members(bytes)?;
    let shared = xlsx_entry(bytes, &members, "xl/sharedStrings.xml")?;
    let shared = String::from_utf8_lossy(&shared);
    let depth_idx = shared_index(&shared, "Depth_m")?;
    let sheet = xlsx_entry(bytes, &members, "xl/worksheets/sheet1.xml")?;
    let sheet = String::from_utf8_lossy(&sheet);
    let cells = sheet_cells(&sheet);
    let header = cells
        .iter()
        .find(|(cr, shared, _num)| shared == &Some(depth_idx) && row_of_ref(cr) == Some(1))?;
    let col = col_of_ref(&header.0)?;
    let val = cells
        .iter()
        .find(|(cr, shared, _num)| {
            shared.is_none() && col_of_ref(cr) == Some(col) && row_of_ref(cr) == Some(2)
        })
        .and_then(|(_, _, num)| *num)?;
    Some(val)
}

fn nrs_id(s: &str) -> String {
    let b = s.as_bytes();
    let mut i = 0usize;
    while i + 3 <= b.len() {
        if &b[i..i + 3] == b"NRS" {
            let mut j = i + 3;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            if j > i + 3 {
                return s[i..j].to_string();
            }
        }
        i += 1;
    }
    String::new()
}

fn day_token(path: &str) -> String {
    let base = path.rsplit('/').next().unwrap_or("");
    for t in base.split('_') {
        if t.len() == 8 && t.is_ascii() {
            let year = &t[..4];
            if let Ok(y) = year.parse::<u32>() {
                if (1990..2100).contains(&y) {
                    return t.to_string();
                }
            }
        }
    }
    String::new()
}

fn collect_metadata_keys(bucket: &str, prefix: &str, depth: usize, out: &mut Vec<String>) -> usize {
    let mut found = 0usize;
    let mut marker = String::new();
    loop {
        let p = match page(bucket, prefix, &marker) {
            Some(p) => p,
            None => break,
        };
        for o in &p.objects {
            if o.key.ends_with("metadata.json") {
                out.push(o.key.clone());
                found += 1;
            }
        }
        if depth > 0 {
            for dir in &p.common {
                if *dir == prefix {
                    continue;
                }
                let leaf = dir.trim_end_matches('/').rsplit('/').next().unwrap_or("");
                if leaf == "data" || leaf == "docs" || leaf == "calibration" {
                    continue;
                }
                found += collect_metadata_keys(bucket, dir, depth - 1, out);
            }
        }
        if !p.truncated {
            break;
        }
        if p.next_marker.is_empty() {
            break;
        }
        marker = p.next_marker;
    }
    found
}

fn run_stations(bucket: &str, prefix: &str, out: Option<&str>) -> usize {
    let mut keys = Vec::new();
    let total = collect_metadata_keys(bucket, prefix, 10, &mut keys);
    let mut buf = String::from("#station|deployment|lat_deg|lon_deg|metadata_key\n");
    let mut shaped = 0usize;
    let mut unread = 0usize;
    for key in &keys {
        let body = match curl(&object_url(bucket, key)) {
            Some(b) => b,
            None => {
                unread += 1;
                continue;
            }
        };
        let Some((lon, lat)) = shape_of(&body) else {
            unread += 1;
            continue;
        };
        let rel = key.trim_start_matches(prefix);
        let mut segs = rel.split('/').filter(|s| !s.is_empty());
        let station = segs.next().unwrap_or("").to_string();
        let deployment = segs.next().unwrap_or("").to_string();
        buf.push_str(&format!(
            "{}|{}|{}|{}|{}\n",
            station,
            deployment,
            cell(Some(lat)),
            cell(Some(lon)),
            key
        ));
        shaped += 1;
    }
    if let Some(p) = out {
        if fs::write(p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    } else {
        print!("{}", buf);
    }
    eprintln!(
        "noaa-nodd: {} station positions from {} deployment metadata files under {}/{}",
        shaped, total, bucket, prefix
    );
    if unread > 0 {
        eprintln!(
            "noaa-nodd: {} deployment metadata files carried no SHAPE point — pending",
            unread
        );
    }
    shaped
}

fn nc_frequencies(path: &str) -> Vec<f64> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    let Some(file) = Hdf5File::parse(&bytes).ok() else {
        return Vec::new();
    };
    let Some(d) = ds_load(&file, "frequency") else {
        return Vec::new();
    };
    let Some(&n) = d.dims.first() else {
        return Vec::new();
    };
    let mut v = Vec::new();
    for i in 0..n as usize {
        if let Some(x) = elem_f64(&d.raw, i, d.class, d.size, d.endian) {
            v.push(x);
        }
    }
    v
}

fn run_nc_values(path: &str, args: &[String], out: Option<&str>) -> usize {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("noaa-nodd: {} does not open — pending", path);
            return 0;
        }
    };
    let Some(file) = Hdf5File::parse(&bytes).ok() else {
        eprintln!(
            "noaa-nodd: {} stays an unreadable HDF5 stream — pending",
            path
        );
        return 0;
    };
    let Some(psd) = ds_load(&file, "psd") else {
        eprintln!("noaa-nodd: {} carries no psd dataset — pending", path);
        return 0;
    };
    if psd.dims.len() != 2 {
        eprintln!("noaa-nodd: {} psd is not a time-by-frequency field", path);
        return 0;
    }
    let nt = psd.dims[0] as usize;
    let nf = psd.dims[1] as usize;
    let Some(time) = ds_load(&file, "time") else {
        eprintln!("noaa-nodd: {} carries no time axis — pending", path);
        return 0;
    };
    let Some(frequency) = ds_load(&file, "frequency") else {
        eprintln!("noaa-nodd: {} carries no frequency axis — pending", path);
        return 0;
    };
    let Some(quality) = ds_load(&file, "quality_flag") else {
        eprintln!(
            "noaa-nodd: {} carries no quality_flag field — pending",
            path
        );
        return 0;
    };
    let ax = match time.dims.first() {
        Some(&v) => v as usize,
        None => {
            eprintln!("noaa-nodd: {} time axis has no extent", path);
            return 0;
        }
    };
    let fx = match frequency.dims.first() {
        Some(&v) => v as usize,
        None => {
            eprintln!("noaa-nodd: {} frequency axis has no extent", path);
            return 0;
        }
    };
    if ax != nt
        || fx != nf
        || raw_elems(psd.raw.len(), psd.size) < nt * nf
        || raw_elems(quality.raw.len(), quality.size) < nt * nf
    {
        eprintln!(
            "noaa-nodd: {} axes disagree — psd {}x{} time {} frequency {} — pending",
            path, nt, nf, ax, fx
        );
        return 0;
    }
    let root = match file.root() {
        Ok(r) => r,
        Err(_) => {
            eprintln!("noaa-nodd: {} root stays unreadable — pending", path);
            return 0;
        }
    };
    let title = match attr_text(root, "title") {
        Some(t) => t,
        None => String::new(),
    };
    let bounds = match attr_text(root, "geospatial_bounds") {
        Some(b) => b,
        None => String::new(),
    };
    let station = match arg_value(args, "--station") {
        Some(s) => s,
        None => {
            let t = nrs_id(path);
            if t.is_empty() {
                nrs_id(&title)
            } else {
                t
            }
        }
    };
    let deployment = match arg_value(args, "--deployment") {
        Some(v) => v,
        None => String::new(),
    };
    let day = day_token(path);
    let limit = match arg_usize(args, "--limit") {
        Some(v) => v,
        None => 0,
    };
    let keep_unusable = args.iter().any(|a| a == "--keep-unusable");
    let psd_obj = file.resolve("psd").ok();
    let psd_unit = match psd_obj.as_ref().and_then(|o| attr_text(o, "units")) {
        Some(t) => t,
        None => String::new(),
    };
    let psd_measure = match psd_obj.as_ref().and_then(|o| attr_text(o, "long_name")) {
        Some(t) => t,
        None => String::new(),
    };

    let mut buf = String::new();
    buf.push_str(&format!("#file={}\n", path));
    if !station.is_empty() {
        buf.push_str(&format!("#station={}\n", station));
    }
    if !deployment.is_empty() {
        buf.push_str(&format!("#deployment={}\n", deployment));
    }
    if !day.is_empty() {
        buf.push_str(&format!("#day={}\n", day));
    }
    if !title.is_empty() {
        buf.push_str(&format!("#title={}\n", title));
    }
    if !bounds.is_empty() {
        buf.push_str(&format!("#position={} lat_deg_lon_deg\n", bounds));
    }
    if !psd_measure.is_empty() {
        buf.push_str(&format!("#psd_measure={}\n", psd_measure));
    }
    if !psd_unit.is_empty() {
        buf.push_str(&format!("#psd_unit={}\n", psd_unit));
    }
    buf.push_str("#time_unit=seconds since 1970-01-01T00:00:00Z\n");
    buf.push_str(
        "#quality_flag=1 Good|2 Not evaluated/Unknown|3 Compromised/Questionable|4 Unusable/Bad\n",
    );
    if !keep_unusable {
        buf.push_str("#quality_flag_4_exclusion=Unusable (4) samples are not emitted as values; 2/3 are a measured state and flow — pass --keep-unusable to carry 4 as well\n");
    }
    buf.push_str("#row=epoch_s|freq_hz|psd_db|quality_flag\n");

    let mut total = 0usize;
    let mut excluded_unusable = 0usize;
    let mut partial = false;
    for i in 0..nt * nf {
        let t = i / nf;
        let f = i % nf;
        let epoch = elem_f64(&time.raw, t, time.class, time.size, time.endian);
        let freq = elem_f64(
            &frequency.raw,
            f,
            frequency.class,
            frequency.size,
            frequency.endian,
        );
        let spl = elem_f64(&psd.raw, i, psd.class, psd.size, psd.endian);
        let q = elem_f64(&quality.raw, i, quality.class, quality.size, quality.endian);
        let (Some(epoch), Some(freq), Some(spl), Some(q)) = (epoch, freq, spl, q) else {
            continue;
        };
        if !keep_unusable && q == 4.0 {
            excluded_unusable += 1;
            continue;
        }
        if limit > 0 && total >= limit {
            partial = true;
            break;
        }
        buf.push_str(&format!(
            "{}|{}|{}|{}\n",
            cell(Some(epoch)),
            cell(Some(freq)),
            cell(Some(spl)),
            cell(Some(q))
        ));
        total += 1;
    }
    if let Some(p) = out {
        if fs::write(p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    } else {
        print!("{}", buf);
    }
    eprintln!("noaa-nodd: {} value lines from {}", total, path);
    if excluded_unusable > 0 {
        eprintln!(
            "noaa-nodd: {} quality_flag-4 (Unusable/Bad) samples excluded from the value field",
            excluded_unusable
        );
    }
    if partial {
        eprintln!(
            "noaa-nodd: limit {} reached — the extraction is partial",
            limit
        );
    }
    total
}

fn run_csv_values(path: &str, args: &[String], out: Option<&str>) -> usize {
    let body = match fs::read_to_string(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("noaa-nodd: {} does not open — pending", path);
            return 0;
        }
    };
    let station = match arg_value(args, "--station") {
        Some(s) => s,
        None => nrs_id(path),
    };
    let deployment = match arg_value(args, "--deployment") {
        Some(v) => v,
        None => String::new(),
    };
    let day = day_token(path);
    let limit = match arg_usize(args, "--limit") {
        Some(v) => v,
        None => 0,
    };
    let nc_path = format!("{}.nc", &path[..path.len() - 4]);
    let freq_axis = nc_frequencies(&nc_path);

    let mut buf = String::new();
    buf.push_str(&format!("#file={}\n", path));
    if !station.is_empty() {
        buf.push_str(&format!("#station={}\n", station));
    }
    if !deployment.is_empty() {
        buf.push_str(&format!("#deployment={}\n", deployment));
    }
    if !day.is_empty() {
        buf.push_str(&format!("#day={}\n", day));
    }
    if freq_axis.is_empty() {
        buf.push_str("#psd_measure absent in the CSV — the sibling netCDF product carries the measured unit and the band centers\n");
    } else {
        buf.push_str("#psd_measure=single-sided mean-square sound pressure spectral density\n");
        buf.push_str("#psd_unit=dB re 1 micropascal^2/Hz\n");
    }
    buf.push_str("#row=datetime|effort_s|bin|freq_hz|spl_db\n");

    let mut total = 0usize;
    let mut partial = false;
    for (ri, line) in body.lines().enumerate() {
        if ri == 0 {
            continue;
        }
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 3 {
            continue;
        }
        let datetime = fields[0];
        let effort = fields[1].parse::<f64>().ok();
        for (ci, raw) in fields.iter().enumerate().skip(2) {
            let spl = raw.trim().parse::<f64>().ok();
            if spl.is_none() {
                continue;
            }
            if limit > 0 && total >= limit {
                partial = true;
                break;
            }
            let bin = ci - 2;
            let freq = freq_axis.get(bin).copied();
            buf.push_str(&format!(
                "{}|{}|{}|{}|{}\n",
                datetime,
                cell(effort),
                bin,
                cell(freq),
                cell(spl)
            ));
            total += 1;
        }
        if partial {
            break;
        }
    }
    if let Some(p) = out {
        if fs::write(p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    } else {
        print!("{}", buf);
    }
    eprintln!("noaa-nodd: {} value lines from {}", total, path);
    if partial {
        eprintln!(
            "noaa-nodd: limit {} reached — the extraction is partial",
            limit
        );
    }
    total
}

fn depdir_of_metadata_key(key: &str) -> Option<String> {
    let (dir, _) = key.rsplit_once("metadata/")?;
    let dir = dir.trim_end_matches('/');
    if dir.is_empty() {
        None
    } else {
        Some(format!("{dir}/"))
    }
}

fn collect_nc_keys(bucket: &str, depdir: &str, out: &mut Vec<String>, days: usize) {
    let mut marker = String::new();
    loop {
        let p = match page(bucket, depdir, &marker) {
            Some(p) => p,
            None => return,
        };
        for o in p.objects {
            if o.key.ends_with("_DAILY_MILLIDEC_MinRes_v3.nc") {
                out.push(o.key.clone());
                if out.len() >= days {
                    return;
                }
            }
        }
        if !p.truncated {
            return;
        }
        if p.next_marker.is_empty() {
            return;
        }
        marker = p.next_marker;
    }
}

fn nc_rows(
    bytes: &[u8],
    lat: f64,
    lon: f64,
    alt: f64,
    lsk: &omegaflow::lsk::LeapSeconds,
    keep_unusable: bool,
) -> Vec<GeoRec> {
    let Ok(file) = Hdf5File::parse(bytes) else {
        return Vec::new();
    };
    let Some(psd) = ds_load(&file, "psd") else {
        return Vec::new();
    };
    let Some(time) = ds_load(&file, "time") else {
        return Vec::new();
    };
    let Some(frequency) = ds_load(&file, "frequency") else {
        return Vec::new();
    };
    let quality = ds_load(&file, "quality_flag");
    if psd.dims.len() != 2 {
        return Vec::new();
    }
    let nt = psd.dims[0] as usize;
    let nf = psd.dims[1] as usize;
    let ax = match time.dims.first() {
        Some(&v) => v as usize,
        None => return Vec::new(),
    };
    let fx = match frequency.dims.first() {
        Some(&v) => v as usize,
        None => return Vec::new(),
    };
    let quality_ok = match &quality {
        None => true,
        Some(q) => raw_elems(q.raw.len(), q.size) >= nt * nf,
    };
    if ax != nt
        || fx != nf
        || raw_elems(psd.raw.len(), psd.size) < nt * nf
        || raw_elems(time.raw.len(), time.size) < nt
        || raw_elems(frequency.raw.len(), frequency.size) < nf
        || !quality_ok
    {
        return Vec::new();
    }
    let mut out = Vec::new();
    for i in 0..nt * nf {
        let t = i / nf;
        let f = i % nf;
        let (Some(epoch), Some(freq), Some(spl)) = (
            elem_f64(&time.raw, t, time.class, time.size, time.endian),
            elem_f64(
                &frequency.raw,
                f,
                frequency.class,
                frequency.size,
                frequency.endian,
            ),
            elem_f64(&psd.raw, i, psd.class, psd.size, psd.endian),
        ) else {
            continue;
        };
        if !(spl.is_finite() && freq.is_finite() && freq > 0.0 && epoch.is_finite()) {
            continue;
        }
        if let Some(q) = &quality {
            if let Some(flag) = elem_f64(&q.raw, i, q.class, q.size, q.endian) {
                if !keep_unusable && flag == 4.0 {
                    continue;
                }
            }
        }
        let Some(tdb) = lsk.unix_to_tdb(epoch) else {
            continue;
        };
        out.push(GeoRec {
            t: tdb,
            lat,
            lon,
            alt,
            freq,
            bin_width: 0.0,
            val: spl,
            comp: COMP_NRS_PSD,
        });
    }
    out
}

fn deployment_depth(bucket: &str, dep: &str, meta_body: &str) -> Option<f64> {
    if let Some(d) = depth_from_metadata_text(meta_body) {
        return Some(d);
    }
    if let Some(p) = page(bucket, &format!("{}metadata/", dep), "") {
        for o in &p.objects {
            if !(o.key.ends_with(".json") || o.key.ends_with(".xml")) {
                continue;
            }
            let Some(b) = curl(&object_url(bucket, &o.key)) else {
                continue;
            };
            if let Some(d) = depth_from_metadata_text(&b) {
                return Some(d);
            }
        }
    }
    if let Some(p) = page(bucket, &format!("{}calibration/", dep), "") {
        for o in &p.objects {
            if !o.key.ends_with(".xlsx") {
                continue;
            }
            let Some(bytes) = curl_bytes(&object_url(bucket, &o.key)) else {
                continue;
            };
            if let Some(d) = xlsx_depth_m(&bytes) {
                return Some(d);
            }
        }
    }
    None
}

fn run_emit_bin(args: &[String], bucket: &str, prefix: &str, out_path: &str, ci: bool) -> usize {
    let Some(lsk_text) = arg_value(args, "--lsk").and_then(|p| fs::read_to_string(p).ok()) else {
        eprintln!("noaa-nodd: --emit-bin needs --lsk <naif0012.tls> — the TDB clock stays unread");
        std::process::exit(1);
    };
    let Some(lsk) = parse_lsk(&lsk_text) else {
        eprintln!("noaa-nodd: --lsk parses void");
        std::process::exit(1);
    };
    let days = arg_usize(args, "--days").unwrap_or(1);
    let keep_unusable = args.iter().any(|a| a == "--keep-unusable");
    let station_filter = match arg_value(args, "--station") {
        Some(v) => v,
        None => String::new(),
    };
    let mut meta = Vec::new();
    collect_metadata_keys(bucket, prefix, 10, &mut meta);
    let mut recs: Vec<GeoRec> = Vec::new();
    let mut n_deploy = 0usize;
    let mut n_depth = 0usize;
    let stations_path = match arg_value(args, "--stations-table") {
        Some(path) => path,
        None => STATIONS_TABLE.to_string(),
    };
    let stations = match fs::read_to_string(&stations_path) {
        Ok(text) => parse_station_table(&text),
        Err(_) => {
            eprintln!(
                "noaa: station table {} unreadable — no table fallback for positions (0 honored)",
                stations_path
            );
            Vec::new()
        }
    };
    for mk in &meta {
        if !station_filter.is_empty() && !mk.contains(&format!("/{}/", station_filter)) {
            continue;
        }
        let Some(body) = curl(&object_url(bucket, mk)) else {
            eprintln!("noaa-nodd: {} stayed unreadable — pending", mk);
            continue;
        };
        let site = station_from_key(mk);
        let site_word = match site.as_deref() {
            Some(s) => format!("station {s}"),
            None => "no site key".to_string(),
        };
        let Some((lat, lon)) = station_position(&body, site.as_deref(), &stations) else {
            eprintln!(
                "noaa-nodd: {} carries no SHAPE point and {} has no table position — deployment stays position-less (0 honored)",
                mk, site_word
            );
            continue;
        };
        let Some(dep) = depdir_of_metadata_key(mk) else {
            continue;
        };
        let alt = match deployment_depth(bucket, &dep, &body) {
            Some(d) => {
                n_depth += 1;
                eprintln!(
                    "noaa-nodd: deployment {} sits {} m below the surface — alt {}",
                    dep, d, -d
                );
                -d
            }
            None => {
                eprintln!(
                    "noaa-nodd: deployment {} carries no measured depth — records stay at the surface (0 honored)",
                    dep
                );
                0.0
            }
        };
        let mut ncs = Vec::new();
        collect_nc_keys(bucket, &format!("{}data/", dep), &mut ncs, days);
        n_deploy += 1;
        for key in ncs {
            let Some(bytes) = curl_bytes(&object_url(bucket, &key)) else {
                eprintln!("noaa-nodd: {} stayed unreadable — pending", key);
                continue;
            };
            let rows = nc_rows(&bytes, lat, lon, alt, &lsk, keep_unusable);
            eprintln!("noaa-nodd: {} → {} psd rows", key, rows.len());
            recs.extend(rows);
        }
    }
    eprintln!(
        "noaa-nodd: {} deployments under {}/{} carry psd rows, {} with a measured depth",
        n_deploy, bucket, prefix, n_depth
    );
    if !keep_unusable {
        eprintln!(
            "noaa-nodd: quality_flag 4 (Unusable/Bad) samples are not carried as values (0 honored); --keep-unusable carries them"
        );
    }
    if recs.is_empty() {
        eprintln!("noaa-nodd: no psd rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    recs.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.freq.total_cmp(&b.freq)));
    let bytes = write_bin(MAGIC_NRS, &recs);
    if fs::write(out_path, &bytes).is_err() {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_NRS, &bytes) {
        Some(parsed) => eprintln!(
            "{}: {} geo records, {} B, roundtrip parses",
            out_path,
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{}: roundtrip parse void", out_path);
            std::process::exit(1);
        }
    }
    if ci && !upload_release(NETLOC, out_path) {
        std::process::exit(1);
    }
    recs.len()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let bucket = match arg_value(&args, "--bucket") {
        Some(v) => v,
        None => DEFAULT_BUCKET.to_string(),
    };
    let prefix = match arg_value(&args, "--prefix") {
        Some(v) => v,
        None => DEFAULT_PREFIX.to_string(),
    };
    let cap = arg_usize(&args, "--cap").unwrap_or(3000);
    let out = arg_value(&args, "--out");
    let depth = arg_usize(&args, "--depth").unwrap_or(6);

    if args.iter().any(|a| a == "--stations") {
        let n = run_stations(&bucket, &prefix, out.as_deref());
        if n == 0 {
            eprintln!(
                "noaa-nodd: no station position measured under gs://{}/{} — nothing fabricated",
                bucket, prefix
            );
            std::process::exit(1);
        }
        return;
    }
    if let Some(path) = arg_value(&args, "--emit-bin") {
        let ci = args.iter().any(|a| a == "--ci-mode");
        let n = run_emit_bin(&args, &bucket, &prefix, &path, ci);
        if n == 0 {
            std::process::exit(1);
        }
        return;
    }
    if let Some(path) = arg_value(&args, "--values") {
        let n = if path.ends_with(".nc") {
            run_nc_values(&path, &args, out.as_deref())
        } else if path.ends_with(".csv") {
            run_csv_values(&path, &args, out.as_deref())
        } else {
            eprintln!("noaa-nodd: {} carries neither .nc nor .csv — pending", path);
            0
        };
        if n == 0 {
            eprintln!(
                "noaa-nodd: {} carried no numeric value — nothing fabricated",
                path
            );
            std::process::exit(1);
        }
        return;
    }

    let mut buf = String::new();
    buf.push_str("#key|size_bytes|last_modified\n");
    let w = walk(&bucket, &prefix, depth, cap, &mut buf);
    if let Some(p) = out.as_deref() {
        if fs::write(p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    }
    eprintln!(
        "noaa-nodd: {} objects, {} common prefixes under gs://{}/{}",
        w.objects, w.dirs, bucket, prefix
    );
    if w.objects == 0 {
        eprintln!(
            "noaa-nodd: the bucket listing carried no objects at prefix {} — nothing fabricated",
            prefix
        );
        std::process::exit(1);
    }
    if out.is_none() {
        print!("{}", buf);
    } else {
        for line in buf.lines().skip(1).take(10) {
            println!("{}", line);
        }
    }
    if w.objects >= cap {
        eprintln!("noaa-nodd: cap {} reached — the walk is partial", cap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATION_TABLE: &str = "\
# NRS-Stationstabelle (SITE_NAME lat lon) — gemessen 2026-09-07
# Quelle: PMEL Ocean Noise Reference Station Network
NRS01 72.44 -156.55
NRS02 50.25 -145.13
NRS11 37.88 -123.44
";

    fn table() -> Vec<(String, f64, f64)> {
        parse_station_table(STATION_TABLE)
    }

    #[test]
    fn station_table_parses_measured_rows() {
        let rows = table();
        assert_eq!(rows.len(), 3);
        let nrs01 = rows.iter().find(|(site, _, _)| site == "NRS01").unwrap();
        assert_eq!(nrs01.1, 72.44);
        assert_eq!(nrs01.2, -156.55);
    }

    #[test]
    fn station_from_metadata_key_yields_the_site_digits() {
        let key = "nrs/products/sound_level_metrics/11/nrs_11_20191023-20211004_hmd_v3/metadata/NRS_11_20191023-20211004_HMD_v3-metadata.json";
        assert_eq!(station_from_key(key).as_deref(), Some("11"));
    }

    #[test]
    fn deployment_shape_is_kept_when_present() {
        let body =
            r#"{"PACKAGE": "NRS_11_20191023-20211004_HMD", "SHAPE": "POINT (-123.45 37.88)"}"#;
        let pos = station_position(body, Some("11"), &table());
        assert_eq!(pos, Some((37.88, -123.45)));
    }

    #[test]
    fn station_without_shape_takes_the_harvested_table_position() {
        let body = r#"{"PACKAGE": "NRS_11_20191023-20211004_HMD"}"#;
        let pos = station_position(body, Some("11"), &table());
        assert_eq!(pos, Some((37.88, -123.44)));
    }

    #[test]
    fn station_without_any_measured_position_stays_positionless() {
        let body = r#"{"PACKAGE": "NRS_13_2020-2022"}"#;
        let pos = station_position(body, Some("13"), &table());
        assert_eq!(pos, None);
    }

    #[test]
    fn resolved_position_flows_into_georec_rows() {
        let body = r#"{"PACKAGE": "NRS_11_20191023-20211004_HMD"}"#;
        let (lat, lon) = station_position(body, Some("11"), &table()).unwrap();
        let rec = GeoRec {
            t: 0.0,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 0.0,
            comp: COMP_NRS_PSD,
        };
        let bytes = write_bin(MAGIC_NRS, &[rec]);
        let parsed = parse_bin(MAGIC_NRS, &bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].lat, 37.88);
        assert_eq!(parsed[0].lon, -123.44);
    }
}
