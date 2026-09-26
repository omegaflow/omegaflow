use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::geo::{COMP_SDARN_V, GeoRec, MAGIC_SDARN, parse_bin, write_bin};
use omegaflow::archivar::json::{JsonVal, jstr, parse_json};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::{Endian, Hdf5File};
use omegaflow::inflate::inflate;
use omegaflow::jwst::mjd_to_unix;
use omegaflow::lsk::parse as parse_lsk;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

const NETLOC: &str = "zenodo.org";

const FITACF_ZENODO: &str = "18525142";
const FITACF_DAY: &str = "20191113";
const GRID_ZENODO: &str = "8274510";
const GRID_FILE: &str = "20160711.sto.v3.0.grid.nc";
const RST_HDW_RAW: &str =
    "https://raw.githubusercontent.com/SuperDARN/rst/main/tables/superdarn/hdw/hdw.dat.";

const EARTH_RADIUS_KM: f64 = 6371.0;
const CHISHAM_A: [f64; 3] = [108.974, 384.416, 1098.28];
const CHISHAM_B: [f64; 3] = [0.0191271, -0.178640, -0.354557];
const CHISHAM_C: [f64; 3] = [6.68283e-5, 1.81405e-4, 9.39961e-5];

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
    let mut stdin = child.stdin.take()?;
    let mut stdout = child.stdout.take()?;
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&raw);
        let _ = stdin.flush();
    });
    let mut out = Vec::new();
    let copied = std::io::copy(&mut stdout, &mut out).is_ok();
    let status = child.wait().ok()?;
    let _ = writer.join();
    if status.success() && copied {
        Some(out)
    } else {
        None
    }
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

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y_adj = if m <= 2 { y - 1 } else { y };
    let era = y_adj.div_euclid(400);
    let yoe = y_adj - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

const DMAP_CODE: u32 = 0x0001_0001;
const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];
const BOUNCE_URL: &str = "https://superdarn.ca/db-fitacf-files-bounce";
const SDC_BASE: &str = "https://sdc-serv.usask.ca/data";

enum ScalarValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str,
}

impl ScalarValue {
    fn as_f64(&self) -> Option<f64> {
        match self {
            ScalarValue::I8(v) => Some(*v as f64),
            ScalarValue::I16(v) => Some(*v as f64),
            ScalarValue::I32(v) => Some(*v as f64),
            ScalarValue::I64(v) => Some(*v as f64),
            ScalarValue::F32(v) => Some(*v as f64),
            ScalarValue::F64(v) => Some(*v),
            ScalarValue::Str => None,
        }
    }
}

struct DmapScalar {
    name: String,
    value: ScalarValue,
}

struct DmapArray {
    name: String,
    nums: Vec<f64>,
}

struct DmapRecord {
    scalars: Vec<DmapScalar>,
    arrays: Vec<DmapArray>,
}

fn rec_scalar<'a>(rec: &'a DmapRecord, name: &str) -> Option<&'a ScalarValue> {
    rec.scalars
        .iter()
        .find(|s| s.name == name)
        .map(|s| &s.value)
}

fn rec_f64(rec: &DmapRecord, name: &str) -> Option<f64> {
    rec_scalar(rec, name).and_then(ScalarValue::as_f64)
}

fn rec_array<'a>(rec: &'a DmapRecord, name: &str) -> Option<&'a DmapArray> {
    rec.arrays.iter().find(|a| a.name == name)
}

fn le_u16_at(b: &[u8], p: usize) -> Option<u16> {
    let s = b.get(p..p + 2)?;
    let arr: [u8; 2] = s.try_into().ok()?;
    Some(u16::from_le_bytes(arr))
}

fn le_u32_at(b: &[u8], p: usize) -> Option<u32> {
    let s = b.get(p..p + 4)?;
    let arr: [u8; 4] = s.try_into().ok()?;
    Some(u32::from_le_bytes(arr))
}

fn le_i32_at(b: &[u8], p: usize) -> Option<i32> {
    let s = b.get(p..p + 4)?;
    let arr: [u8; 4] = s.try_into().ok()?;
    Some(i32::from_le_bytes(arr))
}

fn le_i64_at(b: &[u8], p: usize) -> Option<i64> {
    let s = b.get(p..p + 8)?;
    let arr: [u8; 8] = s.try_into().ok()?;
    Some(i64::from_le_bytes(arr))
}

fn le_f32_at(b: &[u8], p: usize) -> Option<f32> {
    let s = b.get(p..p + 4)?;
    let arr: [u8; 4] = s.try_into().ok()?;
    Some(f32::from_le_bytes(arr))
}

fn le_f64_at(b: &[u8], p: usize) -> Option<f64> {
    let s = b.get(p..p + 8)?;
    let arr: [u8; 8] = s.try_into().ok()?;
    Some(f64::from_le_bytes(arr))
}

fn cstring_at(b: &[u8], p: usize) -> Option<String> {
    let mut end = p;
    while end < b.len() && b[end] != 0 {
        end += 1;
    }
    if end >= b.len() {
        return None;
    }
    Some(String::from_utf8_lossy(&b[p..end]).into_owned())
}

fn scalar_at(b: &[u8], p: usize, t: u8) -> Option<(ScalarValue, usize)> {
    match t {
        1 => Some((ScalarValue::I8(*b.get(p)? as i8), 1)),
        2 => Some((ScalarValue::I16(le_u16_at(b, p)? as i16), 2)),
        3 => Some((ScalarValue::I32(le_i32_at(b, p)?), 4)),
        4 => Some((ScalarValue::F32(le_f32_at(b, p)?), 4)),
        8 => Some((ScalarValue::F64(le_f64_at(b, p)?), 8)),
        9 => {
            let s = cstring_at(b, p)?;
            let adv = s.len() + 1;
            Some((ScalarValue::Str, adv))
        }
        10 => Some((ScalarValue::I64(le_i64_at(b, p)?), 8)),
        16 => Some((ScalarValue::I8(*b.get(p)? as i8), 1)),
        17 => Some((ScalarValue::I32(le_u16_at(b, p)? as i32), 2)),
        18 => Some((ScalarValue::I64(le_u32_at(b, p)? as i64), 4)),
        19 => {
            let s = b.get(p..p + 8)?;
            let arr: [u8; 8] = s.try_into().ok()?;
            let v = u64::from_le_bytes(arr);
            Some((ScalarValue::I64(i64::try_from(v).ok()?), 8))
        }
        255 => {
            let len = le_u32_at(b, p)? as usize;
            if p + 4 + len > b.len() {
                return None;
            }
            Some((ScalarValue::Str, 4 + len))
        }
        _ => None,
    }
}

fn array_at(b: &[u8], p: usize, t: u8, n: usize, name: String) -> Option<(DmapArray, usize)> {
    let mut nums = Vec::new();
    let mut q = p;
    match t {
        9 => {
            for _ in 0..n {
                let s = cstring_at(b, q)?;
                q += s.len() + 1;
            }
        }
        1 | 16 => {
            for _ in 0..n {
                let v = *b.get(q)?;
                q += 1;
                nums.push(v as f64);
            }
        }
        2 | 17 => {
            for _ in 0..n {
                let v = le_u16_at(b, q)?;
                q += 2;
                nums.push(v as f64);
            }
        }
        3 => {
            for _ in 0..n {
                let v = le_i32_at(b, q)?;
                q += 4;
                nums.push(v as f64);
            }
        }
        18 => {
            for _ in 0..n {
                let v = le_u32_at(b, q)?;
                q += 4;
                nums.push(v as f64);
            }
        }
        4 => {
            for _ in 0..n {
                let v = le_f32_at(b, q)?;
                q += 4;
                nums.push(v as f64);
            }
        }
        8 => {
            for _ in 0..n {
                let v = le_f64_at(b, q)?;
                q += 8;
                nums.push(v);
            }
        }
        10 => {
            for _ in 0..n {
                let v = le_i64_at(b, q)?;
                q += 8;
                nums.push(v as f64);
            }
        }
        19 => {
            for _ in 0..n {
                let s = b.get(q..q + 8)?;
                let arr: [u8; 8] = s.try_into().ok()?;
                q += 8;
                nums.push(u64::from_le_bytes(arr) as f64);
            }
        }
        _ => return None,
    }
    Some((DmapArray { name, nums }, q - p))
}

fn dmap_record_at(bytes: &[u8], p: usize) -> Option<(DmapRecord, usize)> {
    let code = le_u32_at(bytes, p)?;
    let sze = le_u32_at(bytes, p + 4)? as usize;
    if code != DMAP_CODE || sze < 16 || p + sze > bytes.len() {
        return None;
    }
    let rec = &bytes[p..p + sze];
    let sn = le_u32_at(rec, 8)? as usize;
    let an = le_u32_at(rec, 12)? as usize;
    if sn > 4096 || an > 4096 {
        return None;
    }
    let mut q = 16usize;
    let mut scalars = Vec::new();
    for _ in 0..sn {
        let name = cstring_at(rec, q)?;
        q += name.len() + 1;
        let t = *rec.get(q)?;
        q += 1;
        let (value, adv) = scalar_at(rec, q, t)?;
        q += adv;
        scalars.push(DmapScalar { name, value });
    }
    let mut arrays = Vec::new();
    for _ in 0..an {
        let name = cstring_at(rec, q)?;
        q += name.len() + 1;
        let t = *rec.get(q)?;
        q += 1;
        let dim = le_u32_at(rec, q)?;
        q += 4;
        if dim < 1 || dim > 8 {
            return None;
        }
        let mut n: usize = 1;
        for _ in 0..dim as usize {
            let r = le_u32_at(rec, q)?;
            q += 4;
            n = match n.checked_mul(r as usize) {
                Some(v) => v,
                None => return None,
            };
        }
        if n > 200_000_000 {
            return None;
        }
        let (arr, adv) = array_at(rec, q, t, n, name)?;
        q += adv;
        arrays.push(arr);
    }
    if q != rec.len() {
        return None;
    }
    Some((DmapRecord { scalars, arrays }, p + sze))
}

fn dmap_parse(bytes: &[u8]) -> Option<Vec<DmapRecord>> {
    let mut p = 0usize;
    let mut recs = Vec::new();
    while p < bytes.len() {
        if p + 8 > bytes.len() {
            return None;
        }
        let (rec, next) = dmap_record_at(bytes, p)?;
        recs.push(rec);
        p = next;
    }
    if recs.is_empty() {
        return None;
    }
    Some(recs)
}

fn fitacf_mjd(rec: &DmapRecord) -> Option<f64> {
    let yr = rec_f64(rec, "time.yr")?;
    let mo = rec_f64(rec, "time.mo")?;
    let dy = rec_f64(rec, "time.dy")?;
    let hr = rec_f64(rec, "time.hr")?;
    let mt = rec_f64(rec, "time.mt")?;
    let sc = rec_f64(rec, "time.sc")?;
    if !(yr >= 1900.0
        && yr <= 2100.0
        && mo >= 1.0
        && mo <= 12.0
        && dy >= 1.0
        && dy <= 31.0
        && hr >= 0.0
        && hr <= 23.0
        && mt >= 0.0
        && mt <= 59.0
        && sc >= 0.0
        && sc <= 60.0)
    {
        return None;
    }
    let us = match rec_f64(rec, "time.us") {
        Some(v) if v >= 0.0 && v < 1_000_000.0 => v,
        Some(_) => return None,
        None => 0.0,
    };
    let days = days_from_civil(yr as i64, mo as i64, dy as i64) as f64;
    let frac = ((hr * 60.0 + mt) * 60.0 + sc + us * 1.0e-6) / 86400.0;
    Some(days + 40587.0 + frac)
}

struct FitGateRow {
    mjd: Option<f64>,
    lat: Option<f64>,
    lon: Option<f64>,
    v: Option<f64>,
    v_e: Option<f64>,
    w_l: Option<f64>,
    w_l_e: Option<f64>,
    p_l: Option<f64>,
    beam: Option<f64>,
    range: Option<f64>,
    tfreq: Option<f64>,
    cp: Option<f64>,
    gflg: Option<f64>,
    elv: Option<f64>,
    noise_sky: Option<f64>,
}

impl FitGateRow {
    fn cell(&self, name: &str) -> Option<f64> {
        match name {
            "mjd" => self.mjd,
            "lat" => self.lat,
            "lon" => self.lon,
            "v" => self.v,
            "v_e" => self.v_e,
            "w_l" => self.w_l,
            "w_l_e" => self.w_l_e,
            "p_l" => self.p_l,
            "beam" => self.beam,
            "range" => self.range,
            "tfreq" => self.tfreq,
            "cp" => self.cp,
            "gflg" => self.gflg,
            "elv" => self.elv,
            "noise.sky" => self.noise_sky,
            _ => None,
        }
    }

    fn to_cells(&self, cols: &[Col]) -> Vec<String> {
        cols.iter()
            .map(|c| match c.kind {
                ColKind::Mjd => iso_cell(self.cell(c.name).map(mjd_to_unix)),
                ColKind::F64 => cell(self.cell(c.name)),
            })
            .collect()
    }
}

fn fitacf_record_rows(rec: &DmapRecord, site: Option<&HdwSite>) -> Option<Vec<FitGateRow>> {
    let mjd = fitacf_mjd(rec)?;
    let Some(slist) = rec_array(rec, "slist") else {
        return Some(Vec::new());
    };
    let nrang = rec_f64(rec, "nrang")?;
    let qflg = rec_array(rec, "qflg");
    let elv = rec_array(rec, "elv");
    let beam = rec_f64(rec, "bmnum");
    let frang = rec_f64(rec, "frang");
    let rsep = rec_f64(rec, "rsep");
    let tfreq = rec_f64(rec, "tfreq");
    let cp = rec_f64(rec, "cp");
    let noise_sky = rec_f64(rec, "noise.sky");
    let at = |name: &str, i: usize| -> Option<f64> {
        rec_array(rec, name)
            .and_then(|a| a.nums.get(i).copied())
            .and_then(|v| if v.is_finite() { Some(v) } else { None })
    };
    let mut rows = Vec::new();
    for i in 0..slist.nums.len() {
        let gate = slist.nums.get(i).copied()?;
        if gate < 0.0 || gate >= nrang || gate.fract() != 0.0 {
            continue;
        }
        let q_ok = match qflg {
            Some(a) => match a.nums.get(i) {
                Some(v) => *v == 1.0,
                None => continue,
            },
            None => true,
        };
        if !q_ok {
            continue;
        }
        let elv_i = match elv {
            Some(a) => a
                .nums
                .get(i)
                .copied()
                .and_then(|v| if v.is_finite() { Some(v) } else { None }),
            None => None,
        };
        let (lat, lon) = match (site, frang, rsep, beam) {
            (Some(s), Some(fr), Some(rs), Some(bm)) => match gate_position(s, bm, fr, rs, gate) {
                Some((la, lo)) => (Some(la), Some(lo)),
                None => (None, None),
            },
            _ => (None, None),
        };
        rows.push(FitGateRow {
            mjd: Some(mjd),
            lat,
            lon,
            v: at("v", i),
            v_e: at("v_e", i),
            w_l: at("w_l", i),
            w_l_e: at("w_l_e", i),
            p_l: at("p_l", i),
            beam,
            range: Some(gate),
            tfreq,
            cp,
            gflg: at("gflg", i),
            elv: elv_i,
            noise_sky,
        });
    }
    Some(rows)
}

fn dmap_fitacf_rows(
    recs: &[DmapRecord],
    cols: &[Col],
    source: &str,
    site: Option<&HdwSite>,
) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut void_records = 0usize;
    for rec in recs {
        match fitacf_record_rows(rec, site) {
            Some(rs) => rows.extend(rs),
            None => void_records += 1,
        }
    }
    eprintln!(
        "{}: {} dmap records, {} fitted range rows, {} records without a valid time/slist frame",
        source,
        recs.len(),
        rows.len(),
        void_records
    );
    rows.iter().map(|r| r.to_cells(cols)).collect()
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
    if bytes.len() >= 8 && bytes[..8] == HDF5_MAGIC {
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
        return;
    }
    if le_u32_at(&bytes, 0) == Some(DMAP_CODE) {
        let recs = match dmap_parse(&bytes) {
            Some(r) => r,
            None => {
                eprintln!("{}: the dmap fitacf stream parses void", source);
                std::process::exit(1);
            }
        };
        let site = site_for(source, None);
        let rows = dmap_fitacf_rows(&recs, &cols, source, site.as_ref());
        let rs = RowSet {
            header: cols.clone(),
            data: rows,
        };
        if rs.data.is_empty() {
            eprintln!(
                "{}: the dmap stream carried no fitted range rows — nothing fabricated",
                source
            );
            std::process::exit(1);
        }
        emit(&rs, &[], out, limit, source);
        return;
    }
    eprintln!(
        "{}: carries neither the HDF5/netCDF magic nor the dmap code — the text arm stays pending (no verified text sample)",
        source
    );
    std::process::exit(1);
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

#[derive(Clone)]
struct HdwSite {
    stid: f64,
    lat_deg: f64,
    lon_deg: f64,
    alt_m: f64,
    boresight_deg: f64,
    bmoff_deg: f64,
    bmsep_deg: f64,
    maxbeam: f64,
}

fn parse_hdw(text: &str, start_yyyymmdd: Option<&str>) -> Option<HdwSite> {
    let mut best_date: Option<String> = None;
    let mut best_site: Option<HdwSite> = None;
    let mut last_site: Option<HdwSite> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 22 {
            continue;
        }
        let date = t[2];
        if date.len() != 8 || !date.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let (
            Some(stid),
            Some(lat),
            Some(lon),
            Some(alt),
            Some(boresight),
            Some(bmoff),
            Some(bmsep),
            Some(maxbeam),
        ) = (
            t[0].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[4].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[5].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[6].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[7].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[8].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[9].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[21].parse::<f64>().ok().filter(|v| v.is_finite()),
        )
        else {
            continue;
        };
        let site = HdwSite {
            stid,
            lat_deg: lat,
            lon_deg: lon,
            alt_m: alt,
            boresight_deg: boresight,
            bmoff_deg: bmoff,
            bmsep_deg: bmsep,
            maxbeam,
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

fn beam_azimuth(site: &HdwSite, bmnum: f64) -> Option<f64> {
    if !bmnum.is_finite() || site.maxbeam < 1.0 {
        return None;
    }
    let offset = site.maxbeam / 2.0 - 0.5;
    let psi = site.bmsep_deg * (bmnum - offset) + site.bmoff_deg;
    let azi = site.boresight_deg + psi;
    if azi.is_finite() { Some(azi) } else { None }
}

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

fn gate_position(
    site: &HdwSite,
    bmnum: f64,
    frang: f64,
    rsep: f64,
    gate: f64,
) -> Option<(f64, f64)> {
    let bearing = beam_azimuth(site, bmnum)?;
    let slant_km = frang + gate * rsep;
    let ground_km = chisham_ground_km(slant_km)?;
    destination(site.lat_deg, site.lon_deg, bearing, ground_km)
}

fn code_from_path(path: &str) -> Option<String> {
    let base = match path.rsplit('/').next() {
        Some(b) => b,
        None => return None,
    };
    for part in base.split('.') {
        if part.len() == 3 && part.bytes().all(|b| b.is_ascii_alphabetic()) {
            return Some(part.to_ascii_lowercase());
        }
    }
    None
}

fn date_from_path(path: &str) -> Option<String> {
    let base = match path.rsplit('/').next() {
        Some(b) => b,
        None => return None,
    };
    let first = base.split('.').next()?;
    if first.len() == 8 && first.bytes().all(|b| b.is_ascii_digit()) {
        Some(first.to_string())
    } else {
        None
    }
}

fn site_for(source: &str, radar: Option<&str>) -> Option<HdwSite> {
    let code = match radar {
        Some(c) => Some(c.to_string()),
        None => code_from_path(source),
    }?;
    let bytes = fetch_hdw(&code)?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    parse_hdw(&text, date_from_path(source).as_deref())
}

fn dmap_fitacf_georecords(
    recs: &[DmapRecord],
    site: &HdwSite,
    lsk: &omegaflow::lsk::LeapSeconds,
) -> Vec<GeoRec> {
    let mut out = Vec::new();
    for rec in recs {
        let Some(mjd) = fitacf_mjd(rec) else {
            continue;
        };
        let unix = mjd_to_unix(mjd);
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        let (Some(frang), Some(rsep), Some(bmnum)) = (
            rec_f64(rec, "frang"),
            rec_f64(rec, "rsep"),
            rec_f64(rec, "bmnum"),
        ) else {
            continue;
        };
        if frang < 0.0 || rsep <= 0.0 {
            continue;
        }
        let Some(slist) = rec_array(rec, "slist") else {
            continue;
        };
        let nrang = rec_f64(rec, "nrang");
        let qflg = rec_array(rec, "qflg");
        let v_arr = rec_array(rec, "v");
        for i in 0..slist.nums.len() {
            let Some(gate) = slist.nums.get(i).copied() else {
                continue;
            };
            if nrang.is_some_and(|n| gate < 0.0 || gate >= n || gate.fract() != 0.0) {
                continue;
            }
            let q_ok = match qflg {
                Some(a) => match a.nums.get(i) {
                    Some(v) => *v == 1.0,
                    None => continue,
                },
                None => true,
            };
            if !q_ok {
                continue;
            }
            let Some(val) = v_arr
                .and_then(|a| a.nums.get(i).copied())
                .filter(|x| x.is_finite())
            else {
                continue;
            };
            let Some((lat, lon)) = gate_position(site, bmnum, frang, rsep, gate) else {
                continue;
            };
            if !(-90.0..=90.0).contains(&lat) || !(-360.0..=360.0).contains(&lon) {
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
                comp: COMP_SDARN_V,
                station: 0,
            });
        }
    }
    out
}

fn emit_dmap_fitacf_bin(bytes: &[u8], source: &str, radar: Option<&str>, bin_path: &str, ci: bool) {
    if le_u32_at(bytes, 0) != Some(DMAP_CODE) {
        eprintln!(
            "superdarn: {} carries no dmap fitacf body — the bin stays unwritten",
            source
        );
        std::process::exit(1);
    }
    let recs = match dmap_parse(bytes) {
        Some(r) => r,
        None => {
            eprintln!("superdarn: {} parses void", source);
            std::process::exit(1);
        }
    };
    let code = match radar
        .map(|s| s.to_string())
        .or_else(|| code_from_path(source))
    {
        Some(c) => c,
        None => {
            eprintln!(
                "superdarn: {} carries no radar code — the position stays absent",
                source
            );
            std::process::exit(1);
        }
    };
    let site = match site_for(source, Some(&code)) {
        Some(s) => s,
        None => {
            eprintln!(
                "superdarn: hdw.dat.{} carries no site geometry — the position stays absent",
                code
            );
            std::process::exit(1);
        }
    };
    eprintln!(
        "superdarn: {} site stid {} lat {:.4} lon {:.4} alt {:.1} boresight {:.2}",
        code, site.stid, site.lat_deg, site.lon_deg, site.alt_m, site.boresight_deg
    );
    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("superdarn: the embedded naif0012 table parses void — TDB stays unread");
            std::process::exit(1);
        }
    };
    let mut records = dmap_fitacf_georecords(&recs, &site, &lsk);
    if records.is_empty() {
        eprintln!("superdarn: no cell rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.lat.total_cmp(&b.lat)));
    let bytes_out = write_bin(MAGIC_SDARN, &records);
    if fs::write(bin_path, &bytes_out).is_err() {
        eprintln!("write {} returned void", bin_path);
        std::process::exit(1);
    }
    match parse_bin(MAGIC_SDARN, &bytes_out) {
        Some(parsed) => eprintln!(
            "{}: {} geo records, {} B, roundtrip parses",
            bin_path,
            parsed.len(),
            bytes_out.len()
        ),
        None => {
            eprintln!("{}: roundtrip parse void", bin_path);
            std::process::exit(1);
        }
    }
    if ci && !upload_release(NETLOC, bin_path) {
        std::process::exit(1);
    }
}

fn bounce_date(raw: &str) -> String {
    let b = raw.as_bytes();
    if b.len() == 10 && b.get(4) == Some(&b'-') && b.get(7) == Some(&b'-') {
        let mut d = String::with_capacity(8);
        for (i, c) in b.iter().enumerate() {
            if i != 4 && i != 7 {
                d.push(*c as char);
            }
        }
        return format!("{} 00:00:00", d);
    }
    raw.to_string()
}

fn bounce_files(start: &str, end: &str, radar: &str) -> Option<Vec<String>> {
    let body = format!(
        r#"{{"date_start":"{}","date_end":"{}","radars":"{}"}}"#,
        start, end, radar
    );
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("180")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(&body)
        .arg(BOUNCE_URL)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let root = parse_json(&text)?;
    match &root {
        JsonVal::Obj(map) => {
            if !matches!(map.get("db_error"), Some(JsonVal::Null) | None) {
                return None;
            }
        }
        _ => return None,
    }
    let resp = jstr(&root, "db_response")?;
    let cleaned = resp.replace('\'', "\"");
    let list = parse_json(&cleaned)?;
    let JsonVal::Arr(items) = list else {
        return None;
    };
    let mut names: Vec<String> = items
        .iter()
        .filter_map(|i| jstr(i, "filename"))
        .filter(|n| !n.is_empty())
        .collect();
    names.sort();
    names.dedup();
    Some(names)
}

fn run_bounce(start: &str, end: &str, radar: &str, out: Option<&str>, limit: usize) {
    let start_norm = bounce_date(start);
    let end_norm = bounce_date(end);
    let files = match bounce_files(&start_norm, &end_norm, radar) {
        Some(f) => f,
        None => {
            eprintln!(
                "superdarn fitacf: {} {} {} carried no file list — pending",
                start, end, radar
            );
            std::process::exit(1);
        }
    };
    if files.is_empty() {
        eprintln!(
            "superdarn fitacf: {} {} {} names no files in the db — pending",
            start, end, radar
        );
        std::process::exit(1);
    }
    let cols = fitacf_cols();
    let site = site_for(radar, Some(radar));
    let mut all_rows = Vec::new();
    for name in &files {
        let (Some(y), Some(m)) = (name.get(0..4), name.get(4..6)) else {
            eprintln!(
                "superdarn fitacf: {} carries no YYYYMM prefix — pending",
                name
            );
            continue;
        };
        let url = format!("{}/{}/{}/{}", SDC_BASE, y, m, name);
        let raw = match curl_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("superdarn fitacf: {} stayed unreadable — pending", url);
                continue;
            }
        };
        let bytes = match decompress_if_needed(raw) {
            Some(b) => b,
            None => {
                eprintln!("superdarn fitacf: {} stays bz2-void — pending", url);
                continue;
            }
        };
        let recs = match dmap_parse(&bytes) {
            Some(r) => r,
            None => {
                eprintln!("superdarn fitacf: {} parses void", url);
                continue;
            }
        };
        let rows = dmap_fitacf_rows(&recs, &cols, &url, site.as_ref());
        eprintln!("superdarn fitacf: {} → {} range rows", name, rows.len());
        all_rows.extend(rows);
    }
    if all_rows.is_empty() {
        eprintln!("superdarn fitacf: no cell rows — nothing fabricated (0 honored)");
        std::process::exit(1);
    }
    emit_rows(
        cols.iter().map(|c| c.name.to_string()).collect(),
        &all_rows,
        &[],
        out,
        limit,
        &format!("db-fitacf {} {} {}", start, end, radar),
    );
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
        let raw = match fs::read(&input) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("read {} returned void", input);
                std::process::exit(1);
            }
        };
        let bytes = match decompress_if_needed(raw) {
            Some(b) => b,
            None => {
                eprintln!("{}: the bz2 stream stays unreadable — pending", input);
                std::process::exit(1);
            }
        };
        if let Some(bin_path) = arg_value(&args, "--out-bin") {
            if mode != "fitacf" {
                eprintln!(
                    "superdarn: --out-bin runs in fitacf mode, the mode is {}",
                    mode
                );
                std::process::exit(1);
            }
            let ci = args.iter().any(|a| a == "--ci-mode");
            emit_dmap_fitacf_bin(
                &bytes,
                &input,
                arg_value(&args, "--radar").as_deref(),
                &bin_path,
                ci,
            );
            return;
        }
        let cols = if mode == "grid" {
            grid_cols()
        } else {
            fitacf_cols()
        };
        process_bytes(bytes, cols, &input, out.as_deref(), limit);
        return;
    }

    if let Some(url) = arg_value(&args, "--url") {
        let raw = match curl_bytes(&url) {
            Some(b) => b,
            None => {
                eprintln!("superdarn fitacf: {} carried no body — pending", url);
                std::process::exit(1);
            }
        };
        let bytes = match decompress_if_needed(raw) {
            Some(b) => b,
            None => {
                eprintln!("{}: the bz2 stream stays unreadable — pending", url);
                std::process::exit(1);
            }
        };
        if let Some(bin_path) = arg_value(&args, "--out-bin") {
            if mode != "fitacf" {
                eprintln!(
                    "superdarn: --out-bin runs in fitacf mode, the mode is {}",
                    mode
                );
                std::process::exit(1);
            }
            let ci = args.iter().any(|a| a == "--ci-mode");
            emit_dmap_fitacf_bin(
                &bytes,
                &url,
                arg_value(&args, "--radar").as_deref(),
                &bin_path,
                ci,
            );
            return;
        }
        let cols = if mode == "grid" {
            grid_cols()
        } else {
            fitacf_cols()
        };
        process_bytes(bytes, cols, &url, out.as_deref(), limit);
        return;
    }

    if let Some(start) = arg_value(&args, "--date-start") {
        if mode != "fitacf" {
            eprintln!(
                "superdarn: --date-start/--date-end run in fitacf mode, the mode is {}",
                mode
            );
            std::process::exit(1);
        }
        let end = match arg_value(&args, "--date-end") {
            Some(v) => v,
            None => {
                eprintln!("superdarn: --date-start carries --date-end <YYYYMMDD HH:MM:SS>");
                std::process::exit(1);
            }
        };
        let radar = match arg_value(&args, "--radar") {
            Some(v) => v,
            None => {
                eprintln!("superdarn: --date-start carries --radar <code>");
                std::process::exit(1);
            }
        };
        run_bounce(&start, &end, &radar, out.as_deref(), limit);
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
