use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    parse_bin, write_bin, GeoRec, COMP_ARGO_BBP700, COMP_ARGO_CHLA, COMP_ARGO_DOXY,
    COMP_ARGO_NITRATE, COMP_ARGO_PH_TOTAL, MAGIC_ARGO,
};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{decode_f32, decode_f64, Hdf5File};
use omegaflow::inflate::gunzip;
use omegaflow::lsk::parse as parse_lsk;
use omegaflow::netcdf::{NetcdfFile, NetcdfType};
use std::collections::HashSet;
use std::env;
use std::fs;

const NETLOC: &str = "data-argo.ifremer.fr";

const INDEX_URL: &str = "https://data-argo.ifremer.fr/argo_bio-profile_index.txt.gz";
const DAC_ROOT: &str = "https://data-argo.ifremer.fr/dac";
const COLUMN_HEADER: &str = "file,date,latitude,longitude,ocean,profiler_type,institution,parameters,parameter_data_mode,date_update";

const BGC_CODES: &[&str] = &[
    "DOXY",
    "DOXY2",
    "NITRATE",
    "NITRATE2",
    "NITRITE",
    "CHLA",
    "CDOM",
    "BBP700",
    "BBP532",
    "BBP470",
    "DOWNWELLING_PAR",
    "DOWNWELLING_IRRADIANCE",
    "UPWELLING_RADIANCE",
    "PH_IN_SITU_TOTAL",
    "PH_IN_SITU",
    "PH_IN_SITU_FREE",
];

const PROFILE_VARS: &[&str] = &[
    "PRES",
    "TEMP",
    "PSAL",
    "DOXY",
    "NITRATE",
    "CHLA",
    "BBP700",
    "CDOM",
    "PH_IN_SITU_TOTAL",
    "PH_IN_SITU",
    "PH_IN_SITU_FREE",
];

const SCALAR_VARS: &[&str] = &["LATITUDE", "LONGITUDE", "JULD"];

struct IndexRow {
    file: String,
    date: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
    ocean: String,
    profiler_type: String,
    institution: String,
    parameters: String,
    data_mode: String,
    date_update: String,
}

struct VarProbe {
    name: String,
    total: usize,
    valid: usize,
    min: Option<f64>,
    max: Option<f64>,
    units: Option<String>,
    note: Option<String>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name).and_then(|v| v.parse::<usize>().ok())
}

fn split_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    for c in line.chars() {
        if c == '"' {
            quoted = !quoted;
        } else if c == ',' && !quoted {
            out.push(field.trim().to_string());
            field.clear();
        } else {
            field.push(c);
        }
    }
    out.push(field.trim().to_string());
    out
}

fn token_set(s: &str) -> HashSet<String> {
    s.split_whitespace().map(|t| t.to_string()).collect()
}

fn is_bgc_profile(parameters: &str) -> bool {
    let toks = token_set(parameters);
    BGC_CODES.iter().any(|c| toks.contains(*c))
}

fn wmo_of(file: &str) -> Option<String> {
    let parts: Vec<&str> = file.split('/').collect();
    if parts.len() >= 3 {
        Some(parts[parts.len() - 3].to_string())
    } else {
        None
    }
}

fn parse_f64(cell: &str) -> Option<f64> {
    let v: f64 = cell.parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn parse_index(text: &str) -> (usize, usize, Vec<IndexRow>) {
    let mut comment_lines = 0usize;
    let mut header_seen = false;
    let mut parsed = 0usize;
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') {
            comment_lines += 1;
            continue;
        }
        if !header_seen {
            header_seen = true;
            if line == COLUMN_HEADER {
                continue;
            }
            if line.starts_with("file,date") {
                continue;
            }
        }
        let cols = split_fields(line);
        if cols.len() < 10 {
            continue;
        }
        parsed += 1;
        rows.push(IndexRow {
            file: cols[0].clone(),
            date: cols[1].clone(),
            latitude: parse_f64(&cols[2]),
            longitude: parse_f64(&cols[3]),
            ocean: cols[4].clone(),
            profiler_type: cols[5].clone(),
            institution: cols[6].clone(),
            parameters: cols[7].clone(),
            data_mode: cols[8].clone(),
            date_update: cols[9].clone(),
        });
    }
    (comment_lines, parsed, rows)
}

fn is_gzip(b: &[u8]) -> bool {
    b.starts_with(&[0x1f, 0x8b])
}

fn load_index_text(args: &[String]) -> Option<String> {
    let path = arg_value(args, "--index");
    let bytes = match path {
        Some(p) => fs::read(&p).ok(),
        None => fetch_raw_bytes(INDEX_URL, 60),
    }?;
    let raw = if is_gzip(&bytes) {
        gunzip(&bytes)?
    } else {
        bytes
    };
    String::from_utf8(raw).ok()
}

fn num_str(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => "absent".to_string(),
    }
}

fn units_str(u: Option<&String>) -> String {
    match u {
        Some(s) => format!("[{}]", s),
        None => String::new(),
    }
}

fn note_str(n: Option<&String>) -> String {
    match n {
        Some(s) => format!(" note={}", s),
        None => String::new(),
    }
}

fn var_attr_num(nc: &NetcdfFile, v: &omegaflow::netcdf::NetcdfVar, name: &str) -> Option<f64> {
    v.attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| nc.attr_num(a))
}

fn var_attr_text(nc: &NetcdfFile, v: &omegaflow::netcdf::NetcdfVar, name: &str) -> Option<String> {
    v.attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| nc.attr_text(a))
}

fn stats_from(
    raw: Vec<f64>,
    fill: Option<f64>,
    scale: Option<f64>,
    offset: Option<f64>,
) -> (usize, Option<f64>, Option<f64>) {
    let mut valid = 0usize;
    let mut min: Option<f64> = None;
    let mut max: Option<f64> = None;
    for x in raw {
        if !x.is_finite() {
            continue;
        }
        if let Some(f) = fill {
            if x == f {
                continue;
            }
        }
        let s = match scale {
            Some(k) => x * k,
            None => x,
        };
        let y = match offset {
            Some(b) => s + b,
            None => s,
        };
        if !y.is_finite() {
            continue;
        }
        valid += 1;
        min = Some(match min {
            Some(m) => m.min(y),
            None => y,
        });
        max = Some(match max {
            Some(m) => m.max(y),
            None => y,
        });
    }
    (valid, min, max)
}

fn probe_nc(bytes: &[u8], name: &str) -> Option<VarProbe> {
    let nc = NetcdfFile::parse(bytes).ok()?;
    let v = nc.var(name)?;
    let scale = var_attr_num(&nc, v, "scale_factor");
    let offset = var_attr_num(&nc, v, "add_offset");
    let fill = var_attr_num(&nc, v, "_FillValue");
    let units = var_attr_text(&nc, v, "units");
    let (raw, note): (Vec<f64>, Option<String>) = match v.nc_type {
        NetcdfType::Float => match nc.values_f32(bytes, name) {
            Some(xs) => (xs.iter().map(|x| *x as f64).collect(), None),
            None => (Vec::new(), Some("float slab void".to_string())),
        },
        NetcdfType::Double => match nc.values_f64(bytes, name) {
            Some(xs) => (xs, None),
            None => (Vec::new(), Some("double slab void".to_string())),
        },
        NetcdfType::Short => match nc.values_i16(bytes, name) {
            Some(xs) => (xs.iter().map(|x| *x as f64).collect(), None),
            None => (Vec::new(), Some("short slab void".to_string())),
        },
        NetcdfType::Int => match nc.values_i32(bytes, name) {
            Some(xs) => (xs.iter().map(|x| *x as f64).collect(), None),
            None => (Vec::new(), Some("int slab void".to_string())),
        },
        NetcdfType::Byte => match nc.values_i8(bytes, name) {
            Some(xs) => (xs.iter().map(|x| *x as f64).collect(), None),
            None => (Vec::new(), Some("byte slab void".to_string())),
        },
        NetcdfType::Char => (Vec::new(), Some("text variable".to_string())),
    };
    let total = raw.len();
    let (valid, min, max) = stats_from(raw, fill, scale, offset);
    Some(VarProbe {
        name: name.to_string(),
        total,
        valid,
        min,
        max,
        units,
        note,
    })
}

fn hdf5_attr_f64(file: &Hdf5File, ds: &str, an: &str) -> Option<f64> {
    let a = file.attribute(ds, an)?;
    let e = a.datatype.endian;
    match (a.datatype.class, a.datatype.size) {
        (1, 4) => decode_f32(&a.data, 0, e).map(|x| x as f64),
        (1, 8) => decode_f64(&a.data, 0, e),
        _ => None,
    }
}

fn hdf5_attr_text(file: &Hdf5File, ds: &str, an: &str) -> Option<String> {
    let a = file.attribute(ds, an)?;
    if a.datatype.class != 3 {
        return None;
    }
    Some(
        String::from_utf8_lossy(&a.data)
            .trim_end_matches('\0')
            .to_string(),
    )
}

fn probe_hdf5(bytes: &[u8], name: &str) -> Option<VarProbe> {
    let file = Hdf5File::parse(bytes).ok()?;
    let (_, _, dt) = file.dataset(name).ok()?;
    let raw = file.read_dataset(name).ok()?;
    let units = hdf5_attr_text(&file, name, "units");
    let fill = hdf5_attr_f64(&file, name, "_FillValue");
    let scale = hdf5_attr_f64(&file, name, "scale_factor");
    let offset = hdf5_attr_f64(&file, name, "add_offset");
    let endian = dt.endian;
    let (elems, note): (Vec<f64>, Option<String>) = match (dt.class, dt.size) {
        (1, 4) => (
            raw.chunks_exact(4)
                .filter_map(|c| decode_f32(c, 0, endian))
                .map(|x| x as f64)
                .collect(),
            None,
        ),
        (1, 8) => (
            raw.chunks_exact(8)
                .filter_map(|c| decode_f64(c, 0, endian))
                .collect(),
            None,
        ),
        _ => (Vec::new(), Some("non-float variable".to_string())),
    };
    let total = elems.len();
    let (valid, min, max) = stats_from(elems, fill, scale, offset);
    Some(VarProbe {
        name: name.to_string(),
        total,
        valid,
        min,
        max,
        units,
        note,
    })
}

fn probe_var(bytes: &[u8], name: &str) -> Option<VarProbe> {
    if bytes.starts_with(b"CDF") {
        probe_nc(bytes, name)
    } else if bytes.starts_with(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]) {
        probe_hdf5(bytes, name)
    } else {
        None
    }
}

fn format_label(bytes: &[u8]) -> String {
    if bytes.starts_with(b"CDF") {
        if bytes.get(3) == Some(&1) {
            "netCDF classic CDF-1".to_string()
        } else if bytes.get(3) == Some(&2) {
            "netCDF classic CDF-2".to_string()
        } else if bytes.get(3) == Some(&5) {
            "netCDF CDF-5".to_string()
        } else {
            "CDF magic, unknown version".to_string()
        }
    } else if bytes.starts_with(&[0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a]) {
        "netCDF-4 (HDF5 container)".to_string()
    } else {
        "unknown byte stream".to_string()
    }
}

fn probe_profile(bytes: &[u8], row: &IndexRow) {
    println!("profile {}", row.file);
    println!(
        "  format {}   measured {}  lat {}  lon {}  ocean {}  profiler {}  institution {}",
        format_label(bytes),
        row.date,
        num_str(row.latitude),
        num_str(row.longitude),
        row.ocean,
        row.profiler_type,
        row.institution
    );
    if bytes.starts_with(b"CDF") {
        match NetcdfFile::parse(bytes) {
            Ok(nc) => {
                println!(
                    "  netcdf {}  dims {}  vars {}",
                    format_label(bytes),
                    nc.dims.len(),
                    nc.vars.len()
                );
                for d in &nc.dims {
                    if d.len == 0 {
                        println!("    dim {} = record", d.name);
                    } else {
                        println!("    dim {} = {}", d.name, d.len);
                    }
                }
            }
            Err(note) => {
                println!("  netcdf parse void: {:?}", note);
                return;
            }
        }
    }
    let mut names: Vec<&str> = Vec::new();
    for n in SCALAR_VARS {
        names.push(n);
    }
    for n in PROFILE_VARS {
        names.push(n);
    }
    for n in names {
        let p = probe_var(bytes, n);
        match p {
            Some(p) => println!(
                "  {}  total {}  valid {}  min {}  max {}{}{}",
                p.name,
                p.total,
                p.valid,
                num_str(p.min),
                num_str(p.max),
                units_str(p.units.as_ref()),
                note_str(p.note.as_ref())
            ),
            None => println!("  {}  absent", n),
        }
    }
}

fn write_manifest(rows: &[IndexRow], out: &Option<String>) {
    let mut buf = String::new();
    for r in rows {
        buf.push_str(&format!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {}\n",
            r.file,
            r.date,
            num_str(r.latitude),
            num_str(r.longitude),
            r.ocean,
            r.profiler_type,
            r.institution,
            r.parameters,
            r.data_mode,
            r.date_update
        ));
    }
    match out {
        Some(path) => match fs::write(path, &buf) {
            Ok(()) => eprintln!("argo_bgc: {} profiles → {}", rows.len(), path),
            Err(e) => eprintln!("argo_bgc: manifest write void: {} {}", path, e),
        },
        None => eprint!("{}", buf),
    }
}

fn nc_series(bytes: &[u8], nc: &NetcdfFile, name: &str) -> Option<Vec<f64>> {
    let v = nc.var(name)?;
    let scale = var_attr_num(nc, v, "scale_factor");
    let offset = var_attr_num(nc, v, "add_offset");
    let fill = var_attr_num(nc, v, "_FillValue");
    let raw: Vec<f64> = match v.nc_type {
        NetcdfType::Float => nc
            .values_f32(bytes, name)?
            .iter()
            .map(|x| *x as f64)
            .collect(),
        NetcdfType::Double => nc.values_f64(bytes, name)?,
        NetcdfType::Short => nc
            .values_i16(bytes, name)?
            .iter()
            .map(|x| *x as f64)
            .collect(),
        NetcdfType::Int => nc
            .values_i32(bytes, name)?
            .iter()
            .map(|x| *x as f64)
            .collect(),
        _ => return None,
    };
    let mut out = Vec::with_capacity(raw.len());
    for x in raw {
        if !x.is_finite() {
            continue;
        }
        if let Some(f) = fill {
            if x == f {
                continue;
            }
        }
        let s = match scale {
            Some(k) => x * k,
            None => x,
        };
        let y = match offset {
            Some(b) => s + b,
            None => s,
        };
        if y.is_finite() {
            out.push(y);
        }
    }
    Some(out)
}

fn nc_comp_of(name: &str) -> Option<u32> {
    match name {
        "DOXY" => Some(COMP_ARGO_DOXY),
        "NITRATE" => Some(COMP_ARGO_NITRATE),
        "CHLA" => Some(COMP_ARGO_CHLA),
        "BBP700" => Some(COMP_ARGO_BBP700),
        "PH_IN_SITU_TOTAL" => Some(COMP_ARGO_PH_TOTAL),
        _ => None,
    }
}

const BGC_LEVEL_VARS: &[&str] = &["DOXY", "NITRATE", "CHLA", "BBP700", "PH_IN_SITU_TOTAL"];

fn bgc_rows(bytes: &[u8], lsk: &omegaflow::lsk::LeapSeconds) -> Vec<GeoRec> {
    let Ok(nc) = NetcdfFile::parse(bytes) else {
        return Vec::new();
    };
    let Some(lat) = nc_series(bytes, &nc, "LATITUDE").and_then(|v| v.first().copied()) else {
        return Vec::new();
    };
    let Some(lon) = nc_series(bytes, &nc, "LONGITUDE").and_then(|v| v.first().copied()) else {
        return Vec::new();
    };
    let Some(juld) = nc_series(bytes, &nc, "JULD").and_then(|v| v.first().copied()) else {
        return Vec::new();
    };
    if !(-90.0..=90.0).contains(&lat) || !(-360.0..=360.0).contains(&lon) || !juld.is_finite() {
        return Vec::new();
    }
    let unix = (juld - 7305.0) * 86400.0;
    let Some(tdb) = lsk.unix_to_tdb(unix) else {
        return Vec::new();
    };
    let Some(pres) = nc_series(bytes, &nc, "PRES") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for var in BGC_LEVEL_VARS {
        let Some(comp) = nc_comp_of(var) else {
            continue;
        };
        let Some(vals) = nc_series(bytes, &nc, var) else {
            continue;
        };
        for (k, v) in vals.iter().enumerate() {
            let Some(p) = pres.get(k).copied() else {
                continue;
            };
            if !(p.is_finite() && p > 0.0 && v.is_finite()) {
                continue;
            }
            out.push(GeoRec {
                t: tdb,
                lat,
                lon,
                alt: -p,
                freq: 0.0,
                bin_width: 0.0,
                val: *v,
                comp,
            });
        }
    }
    out
}

fn run_emit_bin(args: &[String], bgc: &[IndexRow], out_path: &str, ci: bool) {
    let Some(lsk_text) = arg_value(args, "--lsk").and_then(|p| fs::read_to_string(p).ok()) else {
        eprintln!("argo_bgc: --out-bin needs --lsk <naif0012.tls> — the TDB clock stays unread");
        std::process::exit(1);
    };
    let Some(lsk) = parse_lsk(&lsk_text) else {
        eprintln!("argo_bgc: --lsk parses void");
        std::process::exit(1);
    };
    let wmo = arg_value(args, "--wmo");
    let max = arg_usize(args, "--max-profiles").unwrap_or(300);
    let mut records: Vec<GeoRec> = Vec::new();
    let mut n_profiles = 0usize;
    for row in bgc {
        if n_profiles >= max {
            break;
        }
        if let Some(w) = &wmo {
            if wmo_of(&row.file).as_deref() != Some(w.as_str()) {
                continue;
            }
        }
        let url = format!("{}/{}", DAC_ROOT, row.file);
        let Some(bytes) = fetch_raw_bytes(&url, 60) else {
            eprintln!("argo_bgc: {} fetch void — pending", url);
            continue;
        };
        if !bytes.starts_with(b"CDF") {
            continue;
        }
        let recs = bgc_rows(&bytes, &lsk);
        n_profiles += 1;
        eprintln!("argo_bgc: {} → {} level rows", row.file, recs.len());
        records.extend(recs);
    }
    eprintln!("argo_bgc: {} profiles read", n_profiles);
    if records.is_empty() {
        eprintln!("argo_bgc: no level rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.lat.total_cmp(&b.lat))
            .then(a.comp.cmp(&b.comp))
    });
    let bytes = write_bin(MAGIC_ARGO, &records);
    if fs::write(out_path, &bytes).is_err() {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_ARGO, &bytes) {
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let out = arg_value(&args, "--out");
    let probe = arg_value(&args, "--probe");
    let Some(text) = load_index_text(&args) else {
        eprintln!("argo_bgc: index stays unreadable");
        std::process::exit(1);
    };
    let (comment_lines, parsed, all_rows) = parse_index(&text);
    let mut bgc: Vec<IndexRow> = all_rows
        .into_iter()
        .filter(|r| is_bgc_profile(&r.parameters))
        .collect();
    bgc.sort_by(|a, b| a.file.cmp(&b.file));
    bgc.dedup_by(|a, b| a.file == b.file);
    let mut floats: HashSet<String> = HashSet::new();
    for r in &bgc {
        if let Some(w) = wmo_of(&r.file) {
            floats.insert(w);
        }
    }
    eprintln!(
        "argo_bgc: comment lines {}  rows parsed {}  BGC profiles {}  distinct BGC floats {}",
        comment_lines,
        parsed,
        bgc.len(),
        floats.len()
    );
    if let Some(bin_path) = arg_value(&args, "--out-bin") {
        let ci = args.iter().any(|a| a == "--ci-mode");
        run_emit_bin(&args, &bgc, &bin_path, ci);
        return;
    }
    if let Some(sel) = probe {
        let idx: usize = match sel.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("argo_bgc: --probe needs a row index");
                std::process::exit(1);
            }
        };
        let Some(row) = bgc.get(idx) else {
            eprintln!(
                "argo_bgc: probe row {} beyond {} BGC profiles",
                idx,
                bgc.len()
            );
            std::process::exit(1);
        };
        let url = format!("{}/{}", DAC_ROOT, row.file);
        let Some(bytes) = fetch_raw_bytes(&url, 60) else {
            eprintln!("argo_bgc: profile fetch void: {}", url);
            std::process::exit(1);
        };
        probe_profile(&bytes, row);
    }
    write_manifest(&bgc, &out);
}
