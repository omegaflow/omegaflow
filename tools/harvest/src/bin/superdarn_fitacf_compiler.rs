use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_SDARN_V, MAGIC_SDARN};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File};
use omegaflow::inflate::inflate;
use omegaflow::jwst::mjd_to_unix;
use omegaflow::lsk::parse as parse_lsk;
use std::env;
use std::fs;
use std::process::Command;

const NETLOC: &str = "zenodo.org";

const FITACF_ZENODO: &str = "18525142";
const FITACF_DAY: &str = "20191113";
const GRID_ZENODO: &str = "8274510";
const GRID_FILE: &str = "20160711.sto.v3.0.grid.nc";
const RST_HDW_RAW: &str =
    "https://raw.githubusercontent.com/SuperDARN/rst/main/tables/superdarn/hdw/hdw.dat.";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    arg_value(args, name).and_then(|v| v.parse::<usize>().ok())
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
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

fn curl_range(url: &str, from: u64, to: u64) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("180")
        .arg("-r")
        .arg(format!("{}-{}", from, to))
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn http_size(url: &str) -> Option<u64> {
    let out = Command::new("curl")
        .arg("-sSI")
        .arg("-L")
        .arg("-m")
        .arg("60")
        .arg(url)
        .output()
        .ok()?;
    let head = String::from_utf8_lossy(&out.stdout);
    for l in head.lines() {
        let l = l.to_ascii_lowercase();
        if l.starts_with("content-length:") {
            if let Some(v) = l
                .split(':')
                .nth(1)
                .and_then(|s| s.trim().parse::<u64>().ok())
            {
                return Some(v);
            }
        }
    }
    None
}

fn le16(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}
fn le32(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

#[derive(Clone)]
struct ZipEntry {
    name: String,
    comp: u64,
    method: u16,
    lho: u64,
}

fn zip_entries(url: &str) -> Option<Vec<ZipEntry>> {
    let size = http_size(url)?;
    let tail_n = 160000u64;
    let tail = curl_range(url, size - tail_n, size - 1)?;
    let mut eocd: Option<u64> = None;
    for i in (0..tail.len() - 4).rev() {
        if tail[i..i + 4] == [0x50, 0x4b, 0x05, 0x06] {
            eocd = Some((size - tail_n) + i as u64);
            break;
        }
    }
    let eo = (eocd? - (size - tail_n)) as usize;
    let cd_size = le32(&tail[eo + 12..]) as u64;
    let cd_off = le32(&tail[eo + 16..]) as u64;
    let cd = curl_range(url, cd_off, cd_off + cd_size.saturating_sub(1))?;
    let mut entries = Vec::new();
    let mut p = 0usize;
    while p + 46 <= cd.len() {
        if cd[p..p + 4] != [0x50, 0x4b, 0x01, 0x02] {
            p += 1;
            continue;
        }
        let method = le16(&cd[p + 10..]);
        let comp = le32(&cd[p + 20..]) as u64;
        let nlen = le16(&cd[p + 28..]) as usize;
        let xlen = le16(&cd[p + 30..]) as usize;
        let clen = le16(&cd[p + 32..]) as usize;
        let lho = le32(&cd[p + 42..]) as u64;
        let name = String::from_utf8_lossy(&cd[p + 46..p + 46 + nlen]).into_owned();
        entries.push(ZipEntry {
            name,
            comp,
            method,
            lho,
        });
        p += 46 + nlen + xlen + clen;
    }
    Some(entries)
}

fn fetch_entry_bytes(url: &str, e: &ZipEntry) -> Option<Vec<u8>> {
    let lh = curl_range(url, e.lho, e.lho + 30)?;
    let nlen = le16(&lh[26..]) as u64;
    let xlen = le16(&lh[28..]) as u64;
    let start = e.lho + 30 + nlen + xlen;
    let raw = curl_range(url, start, start + e.comp.saturating_sub(1))?;
    if e.method == 0 {
        Some(raw)
    } else {
        inflate(&raw)
    }
}

fn unix_to_iso(u: f64) -> String {
    let sec = u as i64;
    let days = sec.div_euclid(86400);
    let s = sec.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        s / 3600,
        (s % 3600) / 60,
        s % 60
    )
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn elem_f64(raw: &[u8], idx: usize, class: u8, size: usize, endian: Endian) -> Option<f64> {
    let off = idx.checked_mul(size)?;
    let b = raw.get(off..off + size)?;
    match (class, size) {
        (0, 1) => Some(b[0] as i8 as f64),
        (0, 2) => {
            let v = if endian == Endian::Le {
                i16::from_le_bytes([b[0], b[1]])
            } else {
                i16::from_be_bytes([b[0], b[1]])
            };
            Some(v as f64)
        }
        (0, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                i32::from_le_bytes(arr)
            } else {
                i32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (0, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                i64::from_le_bytes(arr)
            } else {
                i64::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 4) => {
            let arr: [u8; 4] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                f32::from_le_bytes(arr)
            } else {
                f32::from_be_bytes(arr)
            };
            Some(v as f64)
        }
        (1, 8) => {
            let arr: [u8; 8] = b.try_into().ok()?;
            let v = if endian == Endian::Le {
                f64::from_le_bytes(arr)
            } else {
                f64::from_be_bytes(arr)
            };
            Some(v)
        }
        _ => None,
    }
}

#[derive(Clone)]
struct Col {
    name: &'static str,
    kind: ColKind,
}

#[derive(Clone)]
enum ColKind {
    Mjd,
    F64,
}

struct Loaded {
    raw: Vec<u8>,
    class: u8,
    size: usize,
    endian: Endian,
    n: usize,
}

fn load(file: &Hdf5File, name: &str) -> Result<Loaded, String> {
    let (obj, ds, dt) = file
        .dataset(name)
        .map_err(|e| format!("{}: {:?}", name, e))?;
    if obj.is_group {
        return Err(format!("{}: group, not a dataset", name));
    }
    let n: usize = ds
        .dims
        .iter()
        .fold(1usize, |a, d| a.saturating_mul(*d as usize));
    let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| file.read_dataset(name)))
        .map_err(|_| format!("{}: layout stayed unreadable", name))?
        .map_err(|e| format!("{}: {:?}", name, e))?;
    Ok(Loaded {
        raw,
        class: dt.class,
        size: dt.size,
        endian: dt.endian,
        n,
    })
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

fn iso_cell(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() && x > 0.0 => unix_to_iso(x),
        _ => String::new(),
    }
}

struct RowSet {
    header: Vec<Col>,
    data: Vec<Vec<String>>,
}

fn read_rows(file: &Hdf5File, cols: &[Col]) -> Result<RowSet, String> {
    let mut loads: Vec<Option<Loaded>> = Vec::new();
    for c in cols {
        match load(file, c.name) {
            Ok(l) => loads.push(Some(l)),
            Err(e) => {
                eprintln!("superdarn: {} (column stays empty)", e);
                loads.push(None);
            }
        }
    }
    let n = loads
        .iter()
        .filter_map(|l| l.as_ref())
        .map(|l| l.n)
        .next()
        .ok_or_else(|| "the file carries no dataset".to_string())?;
    let mut data = Vec::new();
    for j in 0..n {
        let mut row = Vec::new();
        for (ci, c) in cols.iter().enumerate() {
            let v = match &loads[ci] {
                Some(l) if j < l.n => elem_f64(&l.raw, j, l.class, l.size, l.endian),
                _ => None,
            };
            row.push(match c.kind {
                ColKind::Mjd => iso_cell(v.map(mjd_to_unix)),
                ColKind::F64 => cell(v),
            });
        }
        data.push(row);
    }
    Ok(RowSet {
        header: cols.to_vec(),
        data,
    })
}

fn attr_text(data: &[u8]) -> String {
    String::from_utf8_lossy(data)
        .trim_matches('\0')
        .trim()
        .to_string()
}

fn fitacf_bin_records(file: &Hdf5File, lsk: &omegaflow::lsk::LeapSeconds) -> Vec<GeoRec> {
    let cols = fitacf_cols();
    let pos = |name: &str| cols.iter().position(|c| c.name == name);
    let (Some(im), Some(ila), Some(ilo), Some(iv)) = (pos("mjd"), pos("lat"), pos("lon"), pos("v"))
    else {
        return Vec::new();
    };
    let loads: Vec<Option<Loaded>> = cols.iter().map(|c| load(file, c.name).ok()).collect();
    let n = match loads.iter().filter_map(|l| l.as_ref()).next() {
        Some(l) => l.n,
        None => return Vec::new(),
    };
    let cell = |ci: usize, j: usize| -> Option<f64> {
        match &loads[ci] {
            Some(l) if j < l.n => elem_f64(&l.raw, j, l.class, l.size, l.endian),
            _ => None,
        }
    };
    let mut out = Vec::new();
    for j in 0..n {
        let (Some(mjd), Some(lat), Some(lon), Some(v)) =
            (cell(im, j), cell(ila, j), cell(ilo, j), cell(iv, j))
        else {
            continue;
        };
        if !(mjd.is_finite()
            && lat.is_finite()
            && lon.is_finite()
            && v.is_finite()
            && (-90.0..=90.0).contains(&lat)
            && (-360.0..=360.0).contains(&lon))
        {
            continue;
        }
        let unix = mjd_to_unix(mjd);
        if !unix.is_finite() {
            continue;
        }
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        out.push(GeoRec {
            t: tdb,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: v,
            comp: COMP_SDARN_V,
            station: 0,
        });
    }
    out
}

fn emit_fitacf_bin(
    zip_url: &str,
    cand: &[ZipEntry],
    lsk: &omegaflow::lsk::LeapSeconds,
    out_path: &str,
    ci: bool,
) {
    let mut records: Vec<GeoRec> = Vec::new();
    for entry in cand {
        let bytes = match fetch_entry_bytes(zip_url, entry) {
            Some(b) => b,
            None => {
                eprintln!(
                    "superdarn: entry {} stayed unreadable — pending",
                    entry.name
                );
                continue;
            }
        };
        let file = match Hdf5File::parse(&bytes) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("superdarn: {} parses void", entry.name);
                continue;
            }
        };
        let recs = fitacf_bin_records(&file, lsk);
        eprintln!("superdarn: {} → {} cells", entry.name, recs.len());
        records.extend(recs);
    }
    if records.is_empty() {
        eprintln!("superdarn: no cell rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.lat.total_cmp(&b.lat)));
    let bytes = write_bin(MAGIC_SDARN, &records);
    if fs::write(out_path, &bytes).is_err() {
        eprintln!("write {} returned void", out_path);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_SDARN, &bytes) {
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

fn coordinate_notes(file: &Hdf5File, cols: &[Col]) -> Vec<String> {
    let mut notes = Vec::new();
    for c in cols {
        if !(c.name.ends_with("lat") || c.name.ends_with("lon")) {
            continue;
        }
        let Some(a) = file.attribute(c.name, "long_name") else {
            continue;
        };
        if a.datatype.class != 3 {
            continue;
        }
        let name = attr_text(&a.data);
        if name.is_empty() {
            continue;
        }
        let mut note = format!("# {} frame: {}", c.name, name);
        if let Some(u) = file.attribute(c.name, "units") {
            if u.datatype.class == 3 {
                let units = attr_text(&u.data);
                if !units.is_empty() {
                    note.push_str(&format!(" ({})", units));
                }
            }
        }
        notes.push(note);
    }
    notes
}

fn emit_rows(
    header: Vec<String>,
    data: &[Vec<String>],
    notes: &[String],
    out: Option<&str>,
    limit: usize,
    source: &str,
) {
    let mut buf = String::new();
    for n in notes {
        buf.push_str(n);
        buf.push('\n');
    }
    buf.push_str(&format!("#{}\n", header.join("|")));
    for row in data {
        buf.push_str(&format!("{}\n", row.join("|")));
    }
    if let Some(p) = out {
        if fs::write(p, &buf).is_err() {
            eprintln!("write {} returned void", p);
            std::process::exit(1);
        }
    }
    eprintln!("superdarn: {} rows read from {}", data.len(), source);
    if data.is_empty() {
        eprintln!("superdarn: the file carried no rows — nothing fabricated");
        std::process::exit(1);
    }
    for n in notes {
        println!("{}", n);
    }
    println!("#{}", header.join("|"));
    for row in data.iter().take(limit) {
        println!("{}", row.join("|"));
    }
}

fn emit(rs: &RowSet, notes: &[String], out: Option<&str>, limit: usize, source: &str) {
    let header: Vec<String> = rs.header.iter().map(|c| c.name.to_string()).collect();
    emit_rows(header, &rs.data, notes, out, limit, source);
}

fn fitacf_cols() -> Vec<Col> {
    vec![
        Col {
            name: "mjd",
            kind: ColKind::Mjd,
        },
        Col {
            name: "lat",
            kind: ColKind::F64,
        },
        Col {
            name: "lon",
            kind: ColKind::F64,
        },
        Col {
            name: "v",
            kind: ColKind::F64,
        },
        Col {
            name: "v_e",
            kind: ColKind::F64,
        },
        Col {
            name: "w_l",
            kind: ColKind::F64,
        },
        Col {
            name: "w_l_e",
            kind: ColKind::F64,
        },
        Col {
            name: "p_l",
            kind: ColKind::F64,
        },
        Col {
            name: "beam",
            kind: ColKind::F64,
        },
        Col {
            name: "range",
            kind: ColKind::F64,
        },
        Col {
            name: "tfreq",
            kind: ColKind::F64,
        },
        Col {
            name: "cp",
            kind: ColKind::F64,
        },
        Col {
            name: "gflg",
            kind: ColKind::F64,
        },
        Col {
            name: "elv",
            kind: ColKind::F64,
        },
        Col {
            name: "noise.sky",
            kind: ColKind::F64,
        },
    ]
}

fn grid_cols() -> Vec<Col> {
    vec![
        Col {
            name: "mjd_start",
            kind: ColKind::Mjd,
        },
        Col {
            name: "mjd_end",
            kind: ColKind::Mjd,
        },
        Col {
            name: "vector.glat",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.glon",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.mlat",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.mlon",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.vel.median",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.vel.sd",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.vel.dirn",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.wdt.median",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.pwr.median",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.g_kvect",
            kind: ColKind::F64,
        },
        Col {
            name: "vector.kvect",
            kind: ColKind::F64,
        },
    ]
}

fn process_bytes(bytes: Vec<u8>, cols: Vec<Col>, source: &str, out: Option<&str>, limit: usize) {
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{}: the container parses void ({:?})", source, note);
            std::process::exit(1);
        }
    };
    let notes = coordinate_notes(&file, &cols);
    let rs = match read_rows(&file, &cols) {
        Ok(rs) => rs,
        Err(e) => {
            eprintln!("{}: {}", source, e);
            std::process::exit(1);
        }
    };
    emit(&rs, &notes, out, limit, source);
}

fn fetch_hdw(code: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("120")
        .arg(format!("{}{}", RST_HDW_RAW, code))
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn code_of_source(path: &str) -> String {
    let base = path.rsplit('/').next().unwrap_or(path);
    if let Some(rest) = base.strip_prefix("hdw.dat.") {
        return rest.to_string();
    }
    if let Some(rest) = base.strip_prefix("hdw.") {
        return rest.to_string();
    }
    base.to_string()
}

fn hdw_iso(date: &str, time: &str) -> Option<String> {
    let time_ok = time.len() == 8
        && time
            .bytes()
            .enumerate()
            .all(|(i, b)| (b == b':' && (i == 2 || i == 5)) || b.is_ascii_digit());
    if date.len() == 8 && date.bytes().all(|b| b.is_ascii_digit()) && time_ok {
        Some(format!(
            "{}-{}-{}T{}Z",
            &date[0..4],
            &date[4..6],
            &date[6..8],
            time
        ))
    } else {
        None
    }
}

fn hdw_rows(text: &str, code: &str) -> Vec<Vec<String>> {
    let mut rows: Vec<(String, Vec<String>)> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 10 {
            continue;
        }
        let Some(from) = hdw_iso(t[2], t[3]) else {
            continue;
        };
        let mut cells = vec![
            code.to_string(),
            t[0].to_string(),
            t[1].to_string(),
            from.clone(),
            String::new(),
        ];
        cells.push(t[4].to_string());
        cells.push(t[5].to_string());
        cells.push(t[6].to_string());
        cells.push(t[7].to_string());
        cells.push(t[9].to_string());
        rows.push((from, cells));
    }
    for i in 0..rows.len() {
        let next_from = rows.get(i + 1).map(|r| r.0.clone());
        if let Some(nf) = next_from {
            rows[i].1[4] = nf;
        }
    }
    rows.into_iter().map(|(_, c)| c).collect()
}

fn run_stations(args: &[String], out: Option<&str>, limit: usize) {
    let header: Vec<String> = [
        "code",
        "stid",
        "status",
        "valid_from",
        "valid_to",
        "geolat",
        "geolon",
        "alt_m",
        "boresight_deg",
        "beam_sep_deg",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    if let Some(input) = arg_value(args, "--input") {
        let bytes = match fs::read(&input) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("read {} returned void", input);
                std::process::exit(1);
            }
        };
        let code = code_of_source(&input);
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let rows = hdw_rows(&text, &code);
        emit_rows(header, &rows, &[], out, limit, &input);
        return;
    }
    let code = match arg_value(args, "--radar") {
        Some(c) => c,
        None => {
            eprintln!(
                "--mode stations carries --radar <code> (hdw.dat table from SuperDARN/rst) or --input <hdw.dat file>"
            );
            std::process::exit(1);
        }
    };
    let url = format!("{}{}", RST_HDW_RAW, code);
    let bytes = match fetch_hdw(&code) {
        Some(b) => b,
        None => {
            eprintln!("superdarn stations: {} stayed unreadable — pending", url);
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let rows = hdw_rows(&text, &code);
    emit_rows(header, &rows, &[], out, limit, &url);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out = arg_value(&args, "--out");
    let limit = arg_usize(&args, "--limit").unwrap_or(8);
    let mode = match arg_value(&args, "--mode") {
        Some(m) => m,
        None => "fitacf".to_string(),
    };

    if mode == "stations" {
        run_stations(&args, out.as_deref(), limit);
        return;
    }

    if let Some(input) = arg_value(&args, "--input") {
        let bytes = match fs::read(&input) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("read {} returned void", input);
                std::process::exit(1);
            }
        };
        let cols = if mode == "grid" {
            grid_cols()
        } else {
            fitacf_cols()
        };
        process_bytes(bytes, cols, &input, out.as_deref(), limit);
        return;
    }

    if mode == "grid" {
        let record = match arg_value(&args, "--record") {
            Some(v) => v,
            None => GRID_ZENODO.to_string(),
        };
        let file = match arg_value(&args, "--file") {
            Some(v) => v,
            None => GRID_FILE.to_string(),
        };
        let url = format!(
            "https://zenodo.org/api/records/{}/files/{}/content",
            record, file
        );
        let bytes = match curl_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("superdarn grid: {} carried no body — pending", url);
                std::process::exit(1);
            }
        };
        process_bytes(bytes, grid_cols(), &url, out.as_deref(), limit);
        return;
    }

    let record = match arg_value(&args, "--record") {
        Some(v) => v,
        None => FITACF_ZENODO.to_string(),
    };
    let day = match arg_value(&args, "--day") {
        Some(v) => v,
        None => FITACF_DAY.to_string(),
    };
    let radar = arg_value(&args, "--radar");
    let url = format!(
        "https://zenodo.org/api/records/{}/files/{}.nc.zip/content",
        record, day
    );
    let entries = match zip_entries(&url) {
        Some(e) => e,
        None => {
            eprintln!("superdarn fitacf: {} stayed unreadable — pending", url);
            std::process::exit(1);
        }
    };
    let mut cand: Vec<ZipEntry> = entries
        .iter()
        .filter(|e| {
            e.name.ends_with(".nc")
                && (e.name.starts_with(&format!("{}.", day))
                    || e.name.contains(&format!("/{}.", day)))
        })
        .cloned()
        .collect();
    if let Some(r) = &radar {
        cand.retain(|e| e.name.contains(&format!(".{}.nc", r)) || e.name.contains(r));
    }
    cand.sort_by_key(|e| e.comp);
    if cand.is_empty() {
        let radar_name = match radar {
            Some(r) => r.to_string(),
            None => String::new(),
        };
        eprintln!(
            "superdarn fitacf: {} {} carried no {} netCDF entry — pending",
            day, radar_name, url
        );
        std::process::exit(1);
    }
    if let Some(bin_path) = arg_value(&args, "--out-bin") {
        let ci = args.iter().any(|a| a == "--ci-mode");
        let Some(lsk_text) = arg_value(&args, "--lsk").and_then(|p| fs::read_to_string(p).ok())
        else {
            eprintln!(
                "superdarn: --out-bin needs --lsk <naif0012.tls> — the TDB clock stays unread"
            );
            std::process::exit(1);
        };
        let Some(lsk) = parse_lsk(&lsk_text) else {
            eprintln!("superdarn: --lsk parses void");
            std::process::exit(1);
        };
        emit_fitacf_bin(&url, &cand, &lsk, &bin_path, ci);
        return;
    }
    let chosen = match cand.first() {
        Some(c) => c,
        None => {
            let radar_name = match radar {
                Some(r) => r.to_string(),
                None => String::new(),
            };
            eprintln!(
                "superdarn fitacf: {} {} carried no {} netCDF entry — pending",
                day, radar_name, url
            );
            std::process::exit(1);
        }
    };
    let bytes = match fetch_entry_bytes(&url, chosen) {
        Some(b) => b,
        None => {
            eprintln!(
                "superdarn fitacf: entry {} stayed unreadable — pending",
                chosen.name
            );
            std::process::exit(1);
        }
    };
    let source = format!("{}/{}", url, chosen.name);
    process_bytes(bytes, fitacf_cols(), &source, out.as_deref(), limit);
}
