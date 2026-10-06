use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{GeoRec, magic_of, parse_bin, write_bin};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5Attribute, Hdf5File, decode_f32, decode_f64};
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const DEFAULT_NETLOC: &str = "data.laadsdaac.earthdatacloud.nasa.gov";
const DEFAULT_STRIDE: usize = 8;
const FORMAT: &str = "black_marble_vnp46a3_nightlight";
const TEXT_FORMAT: &str = "vnp46a3_axis_value_text";
const COMP_NIGHTLIGHT: u32 = 1;
const MAX_WALK_DEPTH: usize = 48;
const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
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

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    parse_secret(&body, name)
}

fn fetch(url: &str, token: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSLf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("600")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "vnp46a3: fetch {url} returned ({}) — the granule stays pending",
            out.status
        );
        None
    }
}

fn read_source(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let Some(token) = secret("EARTHDATA_EDL_TOKEN") else {
            return Err(
                "EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token (0 honored)"
                    .to_string(),
            );
        };
        fetch(source, &token)
            .ok_or_else(|| format!("{source}: read void — the granule stays pending"))
    } else {
        std::fs::read(source).map_err(|e| format!("{source}: read void ({e})"))
    }
}

fn last_seg(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(s) => s,
        None => path,
    }
}

fn product(dims: &[u64]) -> Option<usize> {
    let mut n = 1usize;
    for d in dims {
        n = n.checked_mul(*d as usize)?;
    }
    Some(n)
}

fn decode_int_at(
    data: &[u8],
    off: usize,
    size: usize,
    endian: Endian,
    signed: bool,
) -> Option<i64> {
    let be = endian == Endian::Be;
    match size {
        1 => data
            .get(off)
            .map(|&b| if signed { b as i8 as i64 } else { b as i64 }),
        2 => {
            let b: [u8; 2] = data.get(off..off + 2)?.try_into().ok()?;
            let v = if be {
                u16::from_be_bytes(b)
            } else {
                u16::from_le_bytes(b)
            };
            Some(if signed { v as i16 as i64 } else { v as i64 })
        }
        4 => {
            let b: [u8; 4] = data.get(off..off + 4)?.try_into().ok()?;
            let v = if be {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            };
            Some(if signed { v as i32 as i64 } else { v as i64 })
        }
        8 => {
            let b: [u8; 8] = data.get(off..off + 8)?.try_into().ok()?;
            let v = if be {
                u64::from_be_bytes(b)
            } else {
                u64::from_le_bytes(b)
            };
            Some(v as i64)
        }
        _ => None,
    }
}

fn attr_number(a: &Hdf5Attribute) -> Option<f64> {
    match a.datatype.class {
        0 => decode_int_at(
            &a.data,
            0,
            a.datatype.size,
            a.datatype.endian,
            a.datatype.signed,
        )
        .map(|v| v as f64),
        1 => match a.datatype.size {
            4 => decode_f32(&a.data, 0, a.datatype.endian).map(|v| v as f64),
            8 => decode_f64(&a.data, 0, a.datatype.endian),
            _ => None,
        },
        _ => None,
    }
}

fn attr_text(a: &Hdf5Attribute) -> Option<String> {
    if a.datatype.class != 3 {
        return None;
    }
    let end = match a.data.iter().position(|&b| b == 0) {
        Some(p) => p,
        None => a.data.len(),
    };
    let text = std::str::from_utf8(a.data.get(..end)?).ok()?.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn text_attr(file: &Hdf5File<'_>, path: &str, name: &str) -> Option<String> {
    file.attribute(path, name).and_then(attr_text)
}

fn walk(file: &Hdf5File<'_>, path: &str, out: &mut Vec<String>, depth: usize) {
    if depth > MAX_WALK_DEPTH {
        return;
    }
    for link in file.links_of(path) {
        if link.addr == u64::MAX {
            continue;
        }
        let child = if path.is_empty() {
            link.name.clone()
        } else {
            format!("{path}/{}", link.name)
        };
        let Ok(obj) = file.resolve(&child) else {
            continue;
        };
        if obj.dataspace.is_some() {
            out.push(child);
        } else if !obj.links.is_empty() {
            walk(file, &child, out, depth + 1);
        }
    }
}

fn all_datasets(file: &Hdf5File<'_>) -> Vec<String> {
    let mut paths = Vec::new();
    walk(file, "", &mut paths, 0);
    paths.sort();
    paths
}

fn inspect(file: &Hdf5File<'_>) {
    let paths = all_datasets(file);
    for p in &paths {
        match file.dataset(p) {
            Ok((obj, ds, dt)) => {
                eprintln!(
                    "dataset {p} dims {:?} class {} size {} signed {} endian {:?}",
                    ds.dims, dt.class, dt.size, dt.signed, dt.endian
                );
                for a in &obj.attrs {
                    match attr_text(a) {
                        Some(t) => eprintln!("    attr {} = {:?}", a.name, t),
                        None => {
                            if let Some(v) = attr_number(a) {
                                eprintln!("    attr {} = {}", a.name, v);
                            }
                        }
                    }
                }
            }
            Err(note) => eprintln!("dataset {p} reads void ({note:?})"),
        }
    }
    eprintln!("vnp46a3: {} datasets under the root", paths.len());
}

fn is_lat_name(seg: &str) -> bool {
    let s = seg.to_lowercase();
    s == "lat" || s == "latitude" || s.starts_with("lat_") || s.ends_with("_lat")
}

fn is_lon_name(seg: &str) -> bool {
    let s = seg.to_lowercase();
    s == "lon" || s == "longitude" || s.starts_with("lon_") || s.ends_with("_lon")
}

fn looks_nightlight(seg: &str) -> bool {
    let s = seg.to_lowercase();
    s.contains("composite") || s.contains("nightlight") || s.contains("ntl") || s.contains("dnb")
}

fn find_lat(paths: &[String]) -> Option<String> {
    paths.iter().find(|p| is_lat_name(last_seg(p))).cloned()
}

fn find_lon(paths: &[String]) -> Option<String> {
    paths.iter().find(|p| is_lon_name(last_seg(p))).cloned()
}

fn find_value(file: &Hdf5File<'_>, paths: &[String]) -> Option<String> {
    let mut fallback: Option<String> = None;
    for p in paths {
        let seg = last_seg(p);
        if is_lat_name(seg) || is_lon_name(seg) || seg.to_lowercase().contains("time") {
            continue;
        }
        let Some(dims) = file.dims(p) else {
            continue;
        };
        if dims.len() < 2 {
            continue;
        }
        if looks_nightlight(seg) {
            return Some(p.clone());
        }
        if fallback.is_none() {
            fallback = Some(p.clone());
        }
    }
    fallback
}

fn find_time_axis(file: &Hdf5File<'_>, paths: &[String]) -> Option<(String, Vec<f64>)> {
    for p in paths {
        if !last_seg(p).to_lowercase().contains("time") {
            continue;
        }
        let Some(units) = text_attr(file, p, "units") else {
            continue;
        };
        if !units.to_lowercase().contains("since") {
            continue;
        }
        let Some(dims) = file.dims(p) else {
            continue;
        };
        let Some(n) = product(&dims) else {
            continue;
        };
        if n <= 1 {
            continue;
        }
        let Ok(times) = file.read_f64_dataset(p) else {
            continue;
        };
        if times.len() != n {
            continue;
        }
        return Some((p.clone(), times));
    }
    None
}

fn find_series_value(
    file: &Hdf5File<'_>,
    paths: &[String],
    n: usize,
    skip: &[&str],
) -> Option<String> {
    for p in paths {
        if skip.iter().any(|s| s == p) {
            continue;
        }
        let seg = last_seg(p);
        if is_lat_name(seg) || is_lon_name(seg) {
            continue;
        }
        let Some(dims) = file.dims(p) else {
            continue;
        };
        if dims.len() != 1 || dims[0] as usize != n {
            continue;
        }
        let Ok(vals) = file.read_f64_dataset(p) else {
            continue;
        };
        if vals.len() == n {
            return Some(p.clone());
        }
    }
    None
}

fn fill_value_of(file: &Hdf5File<'_>, path: &str) -> Option<f64> {
    let raw = file.attr_f64(path, "_FillValue")?;
    let scale = match file.attr_f64(path, "scale_factor") {
        Some(s) => s,
        None => 1.0,
    };
    let offset = match file.attr_f64(path, "add_offset") {
        Some(o) => o,
        None => 0.0,
    };
    Some(raw * scale + offset)
}

fn keep_value(val: f64, fill: Option<f64>) -> bool {
    if !val.is_finite() {
        return false;
    }
    if let Some(f) = fill {
        if val == f {
            return false;
        }
    }
    val >= 0.0
}

fn time_unit(units: &str) -> Option<(f64, f64)> {
    let lower = units.to_lowercase();
    let (unit, since) = lower.split_once(" since ")?;
    let unit_seconds = match unit.trim() {
        "seconds" | "second" | "s" => 1.0,
        "minutes" | "minute" => 60.0,
        "hours" | "hour" | "h" => 3600.0,
        "days" | "day" | "d" => 86400.0,
        _ => return None,
    };
    let since_norm = since.replace('t', " ");
    let mut since_parts = since_norm.split_whitespace();
    let date_part = since_parts.next()?;
    let mut d = date_part.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let time_part = match since_parts.next() {
        Some(t) => t,
        None => "00:00:00",
    };
    let mut t = time_part.split(':');
    let hh: f64 = match t.next().and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => 0.0,
    };
    let mm: f64 = match t.next().and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => 0.0,
    };
    let ss: f64 = match t.next().and_then(|v| v.parse().ok()) {
        Some(v) => v,
        None => 0.0,
    };
    Some((
        unit_seconds,
        days as f64 * 86400.0 + hh * 3600.0 + mm * 60.0 + ss,
    ))
}

fn granule_anchor(source: &str, lsk: &LeapSeconds) -> Option<f64> {
    let base = source.rsplit('/').next()?;
    let at = base.find(".A")?;
    let rest = base.get(at + 2..)?;
    let year: i64 = rest.get(0..4)?.parse().ok()?;
    let doy: i64 = rest.get(4..7)?.parse().ok()?;
    if year < 1 || !(1..=366).contains(&doy) {
        return None;
    }
    let first_day = days_from_civil(year, 1, 1)?;
    let unix = (first_day + (doy - 1)) as f64 * 86400.0 + 43200.0;
    lsk.unix_to_tdb(unix)
}

fn sanitize(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("vnp46a3");
    }
    out
}

fn host_netloc(source: &str) -> Option<String> {
    let rest = source.split("//").nth(1)?;
    let host = rest.split('/').next()?;
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

fn netloc_of(args: &[String], source: &str) -> String {
    match arg_value(args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => match host_netloc(source) {
            Some(h) if source.starts_with("http") => h,
            _ => DEFAULT_NETLOC.to_string(),
        },
    }
}

fn out_path(args: &[String], netloc: &str, label: &str, ext: &str) -> String {
    match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{netloc}/vnp46a3_{label}.{ext}"),
    }
}

fn ensure_parent(out: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    Ok(())
}

fn emit_axis_value(
    file: &Hdf5File<'_>,
    time_path: &str,
    times: &[f64],
    val_path: &str,
    source: &str,
    netloc: &str,
    out: &str,
    ci_mode: bool,
) -> Result<(), String> {
    let units = text_attr(file, time_path, "units")
        .ok_or_else(|| format!("{time_path}: units absent — the epoch stays unmeasured"))?;
    let (unit_seconds, since_unix) = time_unit(&units)
        .ok_or_else(|| format!("{time_path}: units {units:?} carries no 'since' epoch"))?;
    let vals = file
        .read_f64_dataset(val_path)
        .map_err(|e| format!("{val_path}: read void ({e:?})"))?;
    if vals.len() != times.len() {
        return Err(format!(
            "{val_path}: {} values against {} time steps — the series stays unwritten",
            vals.len(),
            times.len()
        ));
    }
    let fill = fill_value_of(file, val_path);
    let mut text =
        format!("# vnp46a3 axis-value | origin {source} | time {time_path} value {val_path}\n");
    let mut kept = 0usize;
    for (t, v) in times.iter().zip(vals.iter()) {
        let unix = since_unix + t * unit_seconds;
        if !unix.is_finite() || !keep_value(*v, fill) {
            continue;
        }
        text.push_str(&format!("{unix:.3} {v}\n"));
        kept += 1;
    }
    if kept == 0 {
        return Err(format!(
            "{source}: no measured axis-value pair — the series stays unwritten (0 honored)"
        ));
    }
    ensure_parent(out)?;
    std::fs::write(out, text.as_bytes()).map_err(|e| format!("write {out} void: {e}"))?;
    println!(
        "url https://github.com/omegaflow/sources/releases/download/{netloc}/{}",
        last_seg(out)
    );
    println!("origin {source}");
    println!("compiler tools/harvest/src/bin/vnp46a3_compiler.rs");
    println!("format {TEXT_FORMAT}");
    println!("sha256 {}", sha256_hex(text.as_bytes()));
    eprintln!("{out}: {kept} axis-value pairs written");
    if ci_mode && !upload_release(netloc, out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {netloc} release"
        ));
    }
    Ok(())
}

fn per_cell_records(
    file: &Hdf5File<'_>,
    paths: &[String],
    stride: usize,
    t: f64,
) -> Result<Vec<GeoRec>, String> {
    let val_path = find_value(file, paths).ok_or_else(|| {
        "no 2-D SDS among the datasets — the per-cell grid stays unwritten".to_string()
    })?;
    let lat_path = find_lat(paths)
        .ok_or_else(|| "no lat dataset — the per-cell grid stays unwritten".to_string())?;
    let lon_path = find_lon(paths)
        .ok_or_else(|| "no lon dataset — the per-cell grid stays unwritten".to_string())?;
    let vd = file
        .dims(&val_path)
        .ok_or_else(|| format!("{val_path}: dims absent"))?;
    let ld = file
        .dims(&lat_path)
        .ok_or_else(|| format!("{lat_path}: dims absent"))?;
    let od = file
        .dims(&lon_path)
        .ok_or_else(|| format!("{lon_path}: dims absent"))?;
    let vals = file
        .read_f64_dataset(&val_path)
        .map_err(|e| format!("{val_path}: read void ({e:?})"))?;
    let lats = file
        .read_f64_dataset(&lat_path)
        .map_err(|e| format!("{lat_path}: read void ({e:?})"))?;
    let lons = file
        .read_f64_dataset(&lon_path)
        .map_err(|e| format!("{lon_path}: read void ({e:?})"))?;
    let fill = fill_value_of(file, &val_path);

    let mut records: Vec<GeoRec> = Vec::new();
    if ld.len() == 1 && od.len() == 1 && vd.len() == 2 && vd[0] == ld[0] && vd[1] == od[0] {
        let n_lat = vd[0] as usize;
        let n_lon = vd[1] as usize;
        let n = n_lat * n_lon;
        if vals.len() != n || lats.len() != n_lat || lons.len() != n_lon {
            return Err(format!(
                "{val_path} {:?} / {lat_path} {:?} / {lon_path} {:?} carry no common axes grid",
                vd, ld, od
            ));
        }
        let mut k = 0usize;
        while k < n {
            let i = k / n_lon;
            let j = k % n_lon;
            let lat = lats.get(i).copied();
            let lon = lons.get(j).copied();
            let val = vals.get(k).copied();
            if let (Some(lat), Some(lon), Some(val)) = (lat, lon, val) {
                if lat.is_finite()
                    && lon.is_finite()
                    && (-90.0..=90.0).contains(&lat)
                    && (-360.0..=360.0).contains(&lon)
                    && keep_value(val, fill)
                {
                    records.push(GeoRec {
                        t,
                        lat,
                        lon,
                        alt: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        val,
                        comp: COMP_NIGHTLIGHT,
                        station: 0,
                    });
                }
            }
            k += stride;
        }
    } else if vd.len() == 2
        && ld == vd
        && od == vd
        && lats.len() == vals.len()
        && lons.len() == vals.len()
    {
        let n = vals.len();
        let mut k = 0usize;
        while k < n {
            let lat = lats.get(k).copied();
            let lon = lons.get(k).copied();
            let val = vals.get(k).copied();
            if let (Some(lat), Some(lon), Some(val)) = (lat, lon, val) {
                if lat.is_finite()
                    && lon.is_finite()
                    && (-90.0..=90.0).contains(&lat)
                    && (-360.0..=360.0).contains(&lon)
                    && keep_value(val, fill)
                {
                    records.push(GeoRec {
                        t,
                        lat,
                        lon,
                        alt: 0.0,
                        freq: 0.0,
                        bin_width: 0.0,
                        val,
                        comp: COMP_NIGHTLIGHT,
                        station: 0,
                    });
                }
            }
            k += stride;
        }
    } else {
        return Err(format!(
            "{val_path} {:?} / {lat_path} {:?} / {lon_path} {:?} carry no common per-cell grid — the raster stays unwritten",
            vd, ld, od
        ));
    }
    Ok(records)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let inspect_mode = args.iter().any(|a| a == "--inspect");
    let source = positional_source(args)
        .ok_or_else(|| "no source — pass a VNP46A3 .h5 URL or local path".to_string())?;
    let label = arg_value(args, "--label").filter(|l| !l.is_empty());
    let stride = match arg_value(args, "--stride") {
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| format!("--stride {v} carries no step"))?,
        None => DEFAULT_STRIDE,
    };
    if stride == 0 {
        return Err("--stride carries no positive sampling step".to_string());
    }

    let bytes = read_source(&source)?;
    if bytes.len() < 8 || bytes[..8] != HDF5_MAGIC {
        return Err(format!(
            "{source}: carries no HDF5 magic ({} B) — the token may point at the login page, the granule stays pending",
            bytes.len()
        ));
    }
    let file = Hdf5File::parse(&bytes).map_err(|e| format!("{source}: hdf5 void ({e:?})"))?;

    if inspect_mode {
        inspect(&file);
        return Ok(());
    }

    let label =
        label.ok_or_else(|| "--label <product> is required — it names the lineage".to_string())?;
    let netloc = netloc_of(args, &source);
    let lsk = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no epoch".to_string())?;
    let t = granule_anchor(&source, &lsk).ok_or_else(|| {
        format!("{source}: carries no .AYYYYDDD. day — the epoch stays unmeasured")
    })?;

    let paths = all_datasets(&file);

    if let Some((time_path, times)) = find_time_axis(&file, &paths) {
        let n = times.len();
        if let Some(val_path) =
            find_series_value(&file, &paths, n, &[time_path.as_str(), "lat", "lon"])
        {
            let out = out_path(args, &netloc, &sanitize(&label), "txt");
            return emit_axis_value(
                &file, &time_path, &times, &val_path, &source, &netloc, &out, ci_mode,
            );
        }
        return Err(format!(
            "{source}: time axis {time_path} ({n} steps) carries no 1-D value series — no per-epoch axis-value pair, the granule stays pending"
        ));
    }

    let mut records = per_cell_records(&file, &paths, stride, t)?;
    let val_path = find_value(&file, &paths).ok_or_else(|| "value SDS vanished".to_string())?;
    let val_dims = match file.dims(&val_path) {
        Some(d) => d,
        None => return Err(format!("{val_path}: dims absent")),
    };
    if records.is_empty() {
        return Err(format!(
            "{source}: no measured VNP46A3 cell left the harvest — the bin stays unwritten (0 honored)"
        ));
    }

    let Some(magic) = magic_of(FORMAT) else {
        return Err(format!(
            "{source}: per-cell {val_dims:?} raster measured ({} records at stride {stride}) — no covering arm: `format {FORMAT}` carries no `geo::magic_of`/`comp_max`, no `extract.rs` reader and no `main_flow` token; the per-cell arm stays unwritten (0 honored)",
            records.len()
        ));
    };

    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.lat.total_cmp(&b.lat))
            .then(a.lon.total_cmp(&b.lon))
    });
    let out = out_path(args, &netloc, &sanitize(&label), "bin");
    ensure_parent(&out)?;
    let bytes = write_bin(magic, &records);
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} void: {e}"))?;
    match parse_bin(magic, &bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            println!(
                "url https://github.com/omegaflow/sources/releases/download/{netloc}/{}",
                last_seg(&out)
            );
            println!("origin {source}");
            println!("compiler tools/harvest/src/bin/vnp46a3_compiler.rs");
            println!("format {FORMAT}");
            println!("sha256 {}", sha256_hex(&bytes));
            eprintln!(
                "{out}: {} records from the VNP46A3 raster, {} B, roundtrip parses",
                parsed.len(),
                bytes.len()
            );
        }
        Some(parsed) => {
            return Err(format!(
                "{out}: {} parsed vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => {
            return Err(format!(
                "{out}: roundtrip parse void — the asset stays unverified"
            ));
        }
    }
    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {netloc} release"
        ));
    }
    Ok(())
}

fn positional_source(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(a.as_str(), "--label" | "--netloc" | "--stride" | "--out") {
            i += 2;
            continue;
        }
        if a.starts_with("--") || a == "-h" {
            i += 1;
            continue;
        }
        return Some(a.clone());
    }
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: vnp46a3_compiler <url|file.h5> --label <product> [--netloc <netloc>] [--stride N] [--out <path>] [--ci-mode] [--inspect]"
        );
        eprintln!(
            "  reads a NASA Black Marble VNP46A3 granule (HDF-EOS5/HDF5) through the archivar HDF5 arm"
        );
        eprintln!(
            "  auth: EARTHDATA_EDL_TOKEN from the environment or .secrets.local (header Authorization)"
        );
        eprintln!("  --inspect prints the granule's datasets and attributes and stops");
        eprintln!("  --ci-mode uploads the verified asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("vnp46a3_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_seg_names_the_leaf() {
        assert_eq!(last_seg("a/b/c"), "c");
        assert_eq!(last_seg("c"), "c");
    }

    #[test]
    fn product_folds_dims() {
        assert_eq!(product(&[2, 3]), Some(6));
        assert_eq!(product(&[]), Some(1));
    }

    #[test]
    fn name_rules_hold() {
        assert!(is_lat_name("lat"));
        assert!(is_lat_name("latitude"));
        assert!(!is_lat_name("lon"));
        assert!(is_lon_name("longitude"));
        assert!(looks_nightlight("AllAngle_Composite_Snow_Free"));
        assert!(looks_nightlight("DNB_BRDF-Corrected_NTL"));
        assert!(!looks_nightlight("Mandatory_Quality_Flag"));
    }

    #[test]
    fn time_unit_reads_the_cf_epoch() {
        assert_eq!(
            time_unit("seconds since 1970-01-01 00:00:00"),
            Some((1.0, 0.0))
        );
        assert_eq!(
            time_unit("days since 1981-01-01 00:00:00"),
            Some((86400.0, 347155200.0))
        );
        assert_eq!(time_unit("fortnights since 2000-01-01"), None);
        assert_eq!(time_unit("seconds"), None);
    }

    #[test]
    fn keep_value_keeps_zero_and_drops_fill() {
        assert!(keep_value(0.0, None));
        assert!(keep_value(1.5, Some(-999.9)));
        assert!(!keep_value(-999.9, Some(-999.9)));
        assert!(!keep_value(-1.0, None));
        assert!(!keep_value(f64::NAN, None));
    }
}
