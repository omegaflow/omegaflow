use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_NRS_PSD, MAGIC_NRS};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File, Hdf5Object};
use omegaflow::lsk::parse as parse_lsk;
use std::env;
use std::fs;

const NETLOC: &str = "storage.googleapis.com";

const DEFAULT_BUCKET: &str = "noaa-passive-bioacoustic";
const DEFAULT_PREFIX: &str = "nrs/products/";

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
    buf.push_str("#row=epoch_s|freq_hz|psd_db|quality_flag\n");

    let mut total = 0usize;
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

fn nc_rows(bytes: &[u8], lat: f64, lon: f64, lsk: &omegaflow::lsk::LeapSeconds) -> Vec<GeoRec> {
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
    if ax != nt
        || fx != nf
        || raw_elems(psd.raw.len(), psd.size) < nt * nf
        || raw_elems(time.raw.len(), time.size) < nt
        || raw_elems(frequency.raw.len(), frequency.size) < nf
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
        let Some(tdb) = lsk.unix_to_tdb(epoch) else {
            continue;
        };
        out.push(GeoRec {
            t: tdb,
            lat,
            lon,
            alt: 0.0,
            freq,
            bin_width: 0.0,
            val: spl,
            comp: COMP_NRS_PSD,
        });
    }
    out
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
    let station_filter = match arg_value(args, "--station") {
        Some(v) => v,
        None => String::new(),
    };
    let mut meta = Vec::new();
    collect_metadata_keys(bucket, prefix, 10, &mut meta);
    let mut recs: Vec<GeoRec> = Vec::new();
    let mut n_deploy = 0usize;
    for mk in &meta {
        if !station_filter.is_empty() && !mk.contains(&format!("/{}/", station_filter)) {
            continue;
        }
        let Some(body) = curl(&object_url(bucket, mk)) else {
            eprintln!("noaa-nodd: {} stayed unreadable — pending", mk);
            continue;
        };
        let Some((lon, lat)) = shape_of(&body) else {
            eprintln!("noaa-nodd: {} carries no SHAPE point — pending", mk);
            continue;
        };
        let Some(dep) = depdir_of_metadata_key(mk) else {
            continue;
        };
        let mut ncs = Vec::new();
        collect_nc_keys(bucket, &format!("{}data/", dep), &mut ncs, days);
        n_deploy += 1;
        for key in ncs {
            let Some(bytes) = curl_bytes(&object_url(bucket, &key)) else {
                eprintln!("noaa-nodd: {} stayed unreadable — pending", key);
                continue;
            };
            let rows = nc_rows(&bytes, lat, lon, &lsk);
            eprintln!("noaa-nodd: {} → {} psd rows", key, rows.len());
            recs.extend(rows);
        }
    }
    eprintln!(
        "noaa-nodd: {} deployments under {}/{} carry psd rows",
        n_deploy, bucket, prefix
    );
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
