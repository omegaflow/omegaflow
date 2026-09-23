use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{COMP_SDARN_POWER, GeoRec, MAGIC_SDRAW, parse_bin, write_bin};
use omegaflow::archivar::units::ymd_to_days;
use omegaflow::cdn::upload_release;
use omegaflow::sha256::sha256_hex;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

const NETLOC: &str = "frdr-dfdr.ca";

const RST_HDW_RAW: &str =
    "https://raw.githubusercontent.com/SuperDARN/rst/main/tables/superdarn/hdw/hdw.dat.";

const EARTH_RADIUS_KM: f64 = 6371.0;

const DATACODE: i32 = 0x00010001;
const DATACHAR: u8 = 1;
const DATASHORT: u8 = 2;
const DATAINT: u8 = 3;
const DATAFLOAT: u8 = 4;
const DATADOUBLE: u8 = 8;
const DATASTRING: u8 = 9;
const DATALONG: u8 = 10;
const DATAUCHAR: u8 = 16;
const DATAUSHORT: u8 = 17;
const DATAUINT: u8 = 18;
const DATAULONG: u8 = 19;
const DATAMAP: u8 = 255;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn le_i16(b: &[u8], o: usize) -> Option<i16> {
    Some(i16::from_le_bytes(b.get(o..o + 2)?.try_into().ok()?))
}
fn le_u16(b: &[u8], o: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(o..o + 2)?.try_into().ok()?))
}
fn le_i32(b: &[u8], o: usize) -> Option<i32> {
    Some(i32::from_le_bytes(b.get(o..o + 4)?.try_into().ok()?))
}
fn le_u32(b: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(o..o + 4)?.try_into().ok()?))
}
fn le_i64(b: &[u8], o: usize) -> Option<i64> {
    Some(i64::from_le_bytes(b.get(o..o + 8)?.try_into().ok()?))
}
fn le_u64(b: &[u8], o: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(o..o + 8)?.try_into().ok()?))
}
fn le_f32(b: &[u8], o: usize) -> Option<f32> {
    Some(f32::from_le_bytes(b.get(o..o + 4)?.try_into().ok()?))
}
fn le_f64(b: &[u8], o: usize) -> Option<f64> {
    Some(f64::from_le_bytes(b.get(o..o + 8)?.try_into().ok()?))
}

fn read_cstr(bytes: &[u8], off: usize, end: usize) -> Option<(String, usize)> {
    let mut i = off;
    while i < end && bytes.get(i).copied() != Some(0) {
        i += 1;
    }
    if i >= end {
        return None;
    }
    let s = String::from_utf8_lossy(bytes.get(off..i)?).into_owned();
    Some((s, i + 1))
}

#[derive(Clone)]
struct DmapScalar {
    name: String,
    num: f64,
}

#[derive(Clone)]
struct DmapArray {
    name: String,
    floats: Vec<f64>,
}

struct DmapBlock {
    scalars: Vec<DmapScalar>,
    arrays: Vec<DmapArray>,
}

impl DmapBlock {
    fn scalar_num(&self, name: &str) -> Option<f64> {
        self.scalars
            .iter()
            .find(|s| s.name == name)
            .map(|s| s.num)
            .filter(|v| v.is_finite())
    }

    fn array(&self, name: &str) -> Option<&DmapArray> {
        self.arrays.iter().find(|a| a.name == name)
    }
}

fn scalar_value(bytes: &[u8], off: usize, kind: u8, end: usize) -> Option<(f64, usize)> {
    match kind {
        DATACHAR => Some((*bytes.get(off)? as i8 as f64, off + 1)),
        DATAUCHAR => Some((*bytes.get(off)? as f64, off + 1)),
        DATASHORT => Some((le_i16(bytes, off)? as f64, off + 2)),
        DATAUSHORT => Some((le_u16(bytes, off)? as f64, off + 2)),
        DATAINT => Some((le_i32(bytes, off)? as f64, off + 4)),
        DATAUINT => Some((le_u32(bytes, off)? as f64, off + 4)),
        DATALONG => Some((le_i64(bytes, off)? as f64, off + 8)),
        DATAULONG => Some((le_u64(bytes, off)? as f64, off + 8)),
        DATAFLOAT => Some((le_f32(bytes, off)? as f64, off + 4)),
        DATADOUBLE => Some((le_f64(bytes, off)?, off + 8)),
        DATASTRING => {
            let (_, next) = read_cstr(bytes, off, end)?;
            Some((0.0, next))
        }
        DATAMAP => {
            let tsze = le_i32(bytes, off)? as usize;
            let next = off.checked_add(4)?.checked_add(tsze)?;
            if next > end {
                return None;
            }
            Some((0.0, next))
        }
        _ => None,
    }
}

fn parse_block(bytes: &[u8], start: usize) -> Option<(DmapBlock, usize)> {
    if le_i32(bytes, start)? != DATACODE {
        return None;
    }
    let sze = le_i32(bytes, start + 4)? as usize;
    let end = start.checked_add(sze)?;
    if sze == 0 || end > bytes.len() {
        return None;
    }
    let mut off = start + 8;
    let sn = le_i32(bytes, off)? as usize;
    off += 4;
    let an = le_i32(bytes, off)? as usize;
    off += 4;
    if sn > 4096 || an > 4096 {
        return None;
    }
    let mut scalars = Vec::with_capacity(sn);
    for _ in 0..sn {
        let (name, next) = read_cstr(bytes, off, end)?;
        off = next;
        let kind = *bytes.get(off)?;
        off += 1;
        let (num, next) = scalar_value(bytes, off, kind, end)?;
        off = next;
        scalars.push(DmapScalar { name, num });
    }
    let mut arrays = Vec::with_capacity(an);
    for _ in 0..an {
        let (name, next) = read_cstr(bytes, off, end)?;
        off = next;
        let kind = *bytes.get(off)?;
        off += 1;
        let dim = le_i32(bytes, off)? as usize;
        off += 4;
        if dim == 0 || dim > 999 {
            return None;
        }
        let mut dims = Vec::with_capacity(dim);
        for _ in 0..dim {
            let d = le_i32(bytes, off)? as usize;
            off += 4;
            dims.push(d);
        }
        let n: usize = dims.iter().product();
        if n > 10_000_000 {
            return None;
        }
        let mut floats = Vec::new();
        match kind {
            DATAFLOAT => {
                floats.reserve(n);
                for _ in 0..n {
                    floats.push(le_f32(bytes, off)? as f64);
                    off += 4;
                }
            }
            DATADOUBLE => {
                floats.reserve(n);
                for _ in 0..n {
                    floats.push(le_f64(bytes, off)?);
                    off += 8;
                }
            }
            DATASHORT => {
                for _ in 0..n {
                    le_i16(bytes, off)?;
                    off += 2;
                }
            }
            DATAINT => {
                for _ in 0..n {
                    le_i32(bytes, off)?;
                    off += 4;
                }
            }
            DATALONG => {
                for _ in 0..n {
                    le_i64(bytes, off)?;
                    off += 8;
                }
            }
            DATACHAR | DATAUCHAR => {
                for _ in 0..n {
                    bytes.get(off)?;
                    off += 1;
                }
            }
            DATAUSHORT => {
                for _ in 0..n {
                    le_u16(bytes, off)?;
                    off += 2;
                }
            }
            DATAUINT => {
                for _ in 0..n {
                    le_u32(bytes, off)?;
                    off += 4;
                }
            }
            DATAULONG => {
                for _ in 0..n {
                    le_u64(bytes, off)?;
                    off += 8;
                }
            }
            DATASTRING => {
                for _ in 0..n {
                    let (_, next) = read_cstr(bytes, off, end)?;
                    off = next;
                }
            }
            _ => return None,
        }
        arrays.push(DmapArray { name, floats });
    }
    Some((DmapBlock { scalars, arrays }, end))
}

fn dmap_blocks(bytes: &[u8]) -> Vec<DmapBlock> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 8 <= bytes.len() {
        match parse_block(bytes, off) {
            Some((block, next)) if next > off => {
                out.push(block);
                off = next;
            }
            _ => break,
        }
    }
    out
}

fn read_source(source: &str) -> Option<Vec<u8>> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let out = Command::new("curl")
            .arg("-sSL")
            .arg("-m")
            .arg("300")
            .arg(source)
            .output()
            .ok()?;
        if out.status.success() {
            Some(out.stdout)
        } else {
            None
        }
    } else {
        fs::read(source).ok()
    }
}

fn decompress_if_needed(raw: Vec<u8>) -> Option<Vec<u8>> {
    if !raw.starts_with(b"BZh") {
        return Some(raw);
    }
    let mut child = Command::new("bzip2")
        .arg("-dc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    {
        let mut stdin = child.stdin.take()?;
        stdin.write_all(&raw).ok()?;
    }
    let out = child.wait_with_output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn code_from_path(path: &str) -> Option<String> {
    let base = path.rsplit('/').next().unwrap_or(path);
    for part in base.split('.') {
        if part.len() == 3 && part.bytes().all(|b| b.is_ascii_alphabetic()) {
            return Some(part.to_ascii_lowercase());
        }
    }
    None
}

fn date_from_path(path: &str) -> Option<String> {
    let base = path.rsplit('/').next().unwrap_or(path);
    let first = base.split('.').next()?;
    if first.len() == 8 && first.bytes().all(|b| b.is_ascii_digit()) {
        Some(first.to_string())
    } else {
        None
    }
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

#[derive(Clone)]
struct RadarSite {
    lat_deg: f64,
    lon_deg: f64,
    boresight_deg: f64,
}

fn hdw_site(text: &str, start_yyyymmdd: Option<&str>) -> Option<RadarSite> {
    let mut best_date: Option<String> = None;
    let mut best_site: Option<RadarSite> = None;
    let mut last_site: Option<RadarSite> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 8 {
            continue;
        }
        let date = t[2];
        if date.len() != 8 || !date.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let (Some(lat), Some(lon), Some(boresight)) = (
            t[4].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[5].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[7].parse::<f64>().ok().filter(|v| v.is_finite()),
        ) else {
            continue;
        };
        let site = RadarSite {
            lat_deg: lat,
            lon_deg: lon,
            boresight_deg: boresight,
        };
        last_site = Some(site.clone());
        if let Some(start) = start_yyyymmdd {
            if date <= start && best_date.as_ref().is_none_or(|b| date > b.as_str()) {
                best_date = Some(date.to_string());
                best_site = Some(site);
            }
        }
    }
    best_site.or(last_site)
}

const CHISHAM_A: [f64; 3] = [108.974, 384.416, 1098.28];
const CHISHAM_B: [f64; 3] = [0.0191271, -0.178640, -0.354557];
const CHISHAM_C: [f64; 3] = [6.68283e-5, 1.81405e-4, 9.39961e-5];

fn chisham_virtual_height_km(slant_km: f64) -> Option<f64> {
    if !slant_km.is_finite() || slant_km <= 0.0 {
        return None;
    }
    let h = if slant_km < 115.0 {
        (slant_km / 115.0) * 112.0
    } else if slant_km < 787.5 {
        CHISHAM_A[0] + CHISHAM_B[0] * slant_km + CHISHAM_C[0] * slant_km * slant_km
    } else if slant_km <= 2137.5 {
        CHISHAM_A[1] + CHISHAM_B[1] * slant_km + CHISHAM_C[1] * slant_km * slant_km
    } else {
        CHISHAM_A[2] + CHISHAM_B[2] * slant_km + CHISHAM_C[2] * slant_km * slant_km
    };
    if h.is_finite() && h > 0.0 {
        Some(h)
    } else {
        None
    }
}

fn chisham_ground_km(slant_km: f64) -> Option<f64> {
    let h = chisham_virtual_height_km(slant_km)?;
    let re = EARTH_RADIUS_KM;
    let rp = re + h;
    let cos_theta = (re * re + rp * rp - slant_km * slant_km) / (2.0 * re * rp);
    if !cos_theta.is_finite() || !(-1.0..=1.0).contains(&cos_theta) {
        return None;
    }
    let ground = re * cos_theta.acos();
    if ground.is_finite() && ground >= 0.0 {
        Some(ground)
    } else {
        None
    }
}

fn destination(lat_deg: f64, lon_deg: f64, bearing_deg: f64, dist_km: f64) -> Option<(f64, f64)> {
    if !lat_deg.is_finite()
        || !lon_deg.is_finite()
        || !bearing_deg.is_finite()
        || !dist_km.is_finite()
        || dist_km < 0.0
    {
        return None;
    }
    let lat1 = lat_deg.to_radians();
    let lon1 = lon_deg.to_radians();
    let bearing = bearing_deg.to_radians();
    let delta = dist_km / EARTH_RADIUS_KM;
    let lat2 = (lat1.sin() * delta.cos() + lat1.cos() * delta.sin() * bearing.cos()).asin();
    let lon2 = lon1
        + (bearing.sin() * delta.sin() * lat1.cos()).atan2(delta.cos() - lat1.sin() * lat2.sin());
    Some((lat2.to_degrees(), lon2.to_degrees()))
}

fn civil_unix(yr: i64, mo: u32, dy: u32, hr: u32, mt: u32, sc: u32, us: u32) -> Option<f64> {
    let days = ymd_to_days(yr, mo, dy)?;
    Some(
        days as f64 * 86400.0 + hr as f64 * 3600.0 + mt as f64 * 60.0 + sc as f64 + us as f64 / 1e6,
    )
}

fn gather(blocks: &[DmapBlock], site: &RadarSite, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let mut out = Vec::new();
    let mut skipped = 0usize;
    for b in blocks {
        let (Some(yr), Some(mo), Some(dy), Some(hr), Some(mt), Some(sc), Some(us)) = (
            b.scalar_num("time.yr"),
            b.scalar_num("time.mo"),
            b.scalar_num("time.dy"),
            b.scalar_num("time.hr"),
            b.scalar_num("time.mt"),
            b.scalar_num("time.sc"),
            b.scalar_num("time.us"),
        ) else {
            skipped += 1;
            continue;
        };
        if !(0.0..1_000_000.0).contains(&us) {
            skipped += 1;
            continue;
        }
        let Some(unix) = civil_unix(
            yr as i64, mo as u32, dy as u32, hr as u32, mt as u32, sc as u32, us as u32,
        ) else {
            skipped += 1;
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            skipped += 1;
            continue;
        };
        let (Some(frang), Some(rsep), Some(nrang)) = (
            b.scalar_num("frang"),
            b.scalar_num("rsep"),
            b.scalar_num("nrang"),
        ) else {
            skipped += 1;
            continue;
        };
        if frang < 0.0 || rsep <= 0.0 || nrang < 1.0 {
            skipped += 1;
            continue;
        }
        let nrang = nrang as usize;
        let Some(pwr0) = b.array("pwr0") else {
            skipped += 1;
            continue;
        };
        if pwr0.floats.len() < nrang {
            skipped += 1;
            continue;
        }
        let bmazm = match b.scalar_num("bmazm") {
            Some(v) => v,
            None => {
                skipped += 1;
                continue;
            }
        };
        let bearing = site.boresight_deg + bmazm;
        for gate in 0..nrang {
            let val = pwr0.floats[gate];
            if !val.is_finite() {
                skipped += 1;
                continue;
            }
            let slant_km = frang + gate as f64 * rsep;
            let Some(ground_km) = chisham_ground_km(slant_km) else {
                skipped += 1;
                continue;
            };
            let Some((lat, lon)) = destination(site.lat_deg, site.lon_deg, bearing, ground_km)
            else {
                skipped += 1;
                continue;
            };
            if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
                skipped += 1;
                continue;
            }
            out.push(GeoRec {
                t: tdb,
                lat,
                lon,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val,
                comp: COMP_SDARN_POWER,
                station: 0,
            });
        }
    }
    eprintln!(
        "superdarn rawacf: {} cells, {} records/gates skipped",
        out.len(),
        skipped
    );
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let usage = "usage: superdarn_rawacf_compiler --input <file.rawacf[.bz2]> | --url <url> --out-bin <superdarn_rawacf.bin> [--radar <code>] [--date <YYYYMMDD>] [--ci-mode]";
    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    };
    let source = match arg_value(&args, "--input").or_else(|| arg_value(&args, "--url")) {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    };
    let radar = match arg_value(&args, "--radar").or_else(|| code_from_path(&source)) {
        Some(v) => v,
        None => {
            eprintln!(
                "superdarn rawacf: {} carries no radar code — --radar <code> stays required",
                source
            );
            std::process::exit(1);
        }
    };
    let date = arg_value(&args, "--date").or_else(|| date_from_path(&source));

    let raw = match read_source(&source) {
        Some(b) => b,
        None => {
            eprintln!("superdarn rawacf: {} stayed unreadable — pending", source);
            std::process::exit(1);
        }
    };
    let bytes = match decompress_if_needed(raw) {
        Some(b) => b,
        None => {
            eprintln!(
                "superdarn rawacf: {} carries no DMap body — pending",
                source
            );
            std::process::exit(1);
        }
    };
    let blocks = dmap_blocks(&bytes);
    if blocks.is_empty() {
        eprintln!(
            "superdarn rawacf: {} carries no DMap record — pending",
            source
        );
        std::process::exit(1);
    }
    eprintln!(
        "superdarn rawacf: {} DMap records, {} B from {}",
        blocks.len(),
        bytes.len(),
        source
    );

    let hdw = match fetch_hdw(&radar) {
        Some(b) => b,
        None => {
            eprintln!(
                "superdarn rawacf: hdw.dat.{} stayed unreadable — position stays absent",
                radar
            );
            std::process::exit(1);
        }
    };
    let hdw_text = String::from_utf8_lossy(&hdw).into_owned();
    let site = match hdw_site(&hdw_text, date.as_deref()) {
        Some(s) => s,
        None => {
            eprintln!(
                "superdarn rawacf: hdw.dat.{} carries no site geometry — position stays absent",
                radar
            );
            std::process::exit(1);
        }
    };
    eprintln!(
        "superdarn rawacf: {} site lat {:.4} lon {:.4} boresight {:.2}",
        radar, site.lat_deg, site.lon_deg, site.boresight_deg
    );

    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!(
                "superdarn rawacf: the embedded naif0012 table parses void — TDB stays unread"
            );
            std::process::exit(1);
        }
    };

    let mut records = gather(&blocks, &site, &lsk);
    if records.is_empty() {
        eprintln!("superdarn rawacf: no cell rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.lat.total_cmp(&b.lat)));
    let out = write_bin(MAGIC_SDRAW, &records);
    if fs::write(&out_bin, &out).is_err() {
        eprintln!("write {} returned void", out_bin);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_SDRAW, &out) {
        Some(parsed) => eprintln!(
            "{}: {} geo records, {} B, roundtrip parses",
            out_bin,
            parsed.len(),
            out.len()
        ),
        None => {
            eprintln!("{}: roundtrip parse void", out_bin);
            std::process::exit(1);
        }
    }
    println!("{} sha256 {}", out_bin, sha256_hex(&out));

    if args.iter().any(|a| a == "--ci-mode") && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enc_cstr(out: &mut Vec<u8>, s: &str) {
        out.extend_from_slice(s.as_bytes());
        out.push(0);
    }

    fn enc_block(scalars: &[(&str, i32)], array_name: &str, data: &[f32]) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&(scalars.len() as i32).to_le_bytes());
        payload.extend_from_slice(&1i32.to_le_bytes());
        for (name, v) in scalars {
            enc_cstr(&mut payload, name);
            payload.push(DATAINT);
            payload.extend_from_slice(&v.to_le_bytes());
        }
        enc_cstr(&mut payload, array_name);
        payload.push(DATAFLOAT);
        payload.extend_from_slice(&1i32.to_le_bytes());
        payload.extend_from_slice(&(data.len() as i32).to_le_bytes());
        for v in data {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        let sze = (8 + payload.len()) as i32;
        let mut out = Vec::new();
        out.extend_from_slice(&DATACODE.to_le_bytes());
        out.extend_from_slice(&sze.to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    fn sample_block() -> Vec<u8> {
        enc_block(
            &[
                ("time.yr", 2023),
                ("time.mo", 1),
                ("time.dy", 1),
                ("time.hr", 0),
                ("time.mt", 0),
                ("time.sc", 0),
                ("time.us", 0),
                ("frang", 180),
                ("rsep", 45),
                ("nrang", 3),
                ("bmazm", 0),
            ],
            "pwr0",
            &[10.0, 20.0, 30.0],
        )
    }

    #[test]
    fn dmap_block_reads_scalars_and_arrays() {
        let bytes = sample_block();
        let blocks = dmap_blocks(&bytes);
        assert_eq!(blocks.len(), 1);
        let b = &blocks[0];
        assert_eq!(b.scalar_num("nrang"), Some(3.0));
        assert_eq!(b.scalar_num("frang"), Some(180.0));
        let pwr0 = b.array("pwr0").expect("pwr0");
        assert_eq!(pwr0.floats, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn dmap_rejects_a_foreign_code() {
        let mut bytes = sample_block();
        bytes[0] = 0x02;
        assert!(dmap_blocks(&bytes).is_empty());
    }

    fn enc_mixed_block() -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&3i32.to_le_bytes());
        payload.extend_from_slice(&1i32.to_le_bytes());
        enc_cstr(&mut payload, "radar.revision.major");
        payload.push(DATACHAR);
        payload.push(5u8);
        enc_cstr(&mut payload, "origin.time");
        payload.push(DATASTRING);
        enc_cstr(&mut payload, "Sat Dec 31 23:59:59 2022");
        enc_cstr(&mut payload, "time.yr");
        payload.push(DATASHORT);
        payload.extend_from_slice(&(2022i16).to_le_bytes());
        enc_cstr(&mut payload, "pwr0");
        payload.push(DATAFLOAT);
        payload.extend_from_slice(&1i32.to_le_bytes());
        payload.extend_from_slice(&3i32.to_le_bytes());
        for v in [10.0f32, 20.0f32, 30.0f32] {
            payload.extend_from_slice(&v.to_le_bytes());
        }
        let sze = (8 + payload.len()) as i32;
        let mut out = Vec::new();
        out.extend_from_slice(&DATACODE.to_le_bytes());
        out.extend_from_slice(&sze.to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    #[test]
    fn datacode_encodes_little_endian() {
        assert_eq!(DATACODE.to_le_bytes(), [0x01, 0x00, 0x01, 0x00]);
    }

    #[test]
    fn dmap_block_reads_little_endian_scalar_types() {
        let blocks = dmap_blocks(&enc_mixed_block());
        assert_eq!(blocks.len(), 1);
        let b = &blocks[0];
        assert_eq!(b.scalar_num("radar.revision.major"), Some(5.0));
        assert_eq!(b.scalar_num("time.yr"), Some(2022.0));
        let pwr0 = b.array("pwr0").expect("pwr0");
        assert_eq!(pwr0.floats, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn path_carries_radar_and_date() {
        let path = "/x/20230101.0000.00.fir.a.rawacf.bz2";
        assert_eq!(code_from_path(path).as_deref(), Some("fir"));
        assert_eq!(date_from_path(path).as_deref(), Some("20230101"));
        assert_eq!(
            code_from_path("/x/20230101.0000.03.gbr.rawacf.bz2").as_deref(),
            Some("gbr")
        );
        assert_eq!(code_from_path("/x/plain").as_deref(), None);
    }

    #[test]
    fn chisham_ground_range_is_shorter_than_slant_range() {
        let ground = chisham_ground_km(180.0).expect("finite");
        assert!(ground > 0.0 && ground < 180.0, "ground {ground}");
        let far = chisham_ground_km(1500.0).expect("finite");
        assert!(far > 0.0 && far < 1500.0, "far ground {far}");
        assert!(chisham_ground_km(-1.0).is_none());
        assert!(chisham_ground_km(f64::NAN).is_none());
    }

    #[test]
    fn gather_compiles_gate_power_to_georecords() {
        let blocks = dmap_blocks(&sample_block());
        let site = RadarSite {
            lat_deg: 52.16,
            lon_deg: -106.53,
            boresight_deg: 23.1,
        };
        let Some(lsk) = embedded_lsk() else {
            return;
        };
        let records = gather(&blocks, &site, &lsk);
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].val, 10.0);
        assert_eq!(records[0].comp, COMP_SDARN_POWER);
        assert!(records[0].t.is_finite() && records[0].lat.is_finite());
    }

    fn zero_frang_block() -> Vec<u8> {
        enc_block(
            &[
                ("time.yr", 2023),
                ("time.mo", 1),
                ("time.dy", 1),
                ("time.hr", 0),
                ("time.mt", 0),
                ("time.sc", 0),
                ("time.us", 0),
                ("frang", 0),
                ("rsep", 45),
                ("nrang", 2),
                ("bmazm", 0),
            ],
            "pwr0",
            &[10.0, 20.0],
        )
    }

    #[test]
    fn gather_keeps_a_zero_frang_record() {
        let blocks = dmap_blocks(&zero_frang_block());
        let site = RadarSite {
            lat_deg: 52.16,
            lon_deg: -106.53,
            boresight_deg: 23.1,
        };
        let Some(lsk) = embedded_lsk() else {
            return;
        };
        let records = gather(&blocks, &site, &lsk);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].val, 20.0);
    }
}
