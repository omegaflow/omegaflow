use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::process::Command;

const MAGIC: &[u8; 4] = b"CPCP";
const FIELDS: usize = 3;
const NETLOC: &str = "zenodo.org";
const OUT_PATH: &str = "superdarn_cpcp.bin";
const DEFAULT_URL: &str =
    "https://zenodo.org/api/records/10374021/files/20160311.north.map2/content";
const DMAP_CODE: u32 = 0x0001_0001;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_bytes(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSL")
        .arg("-m")
        .arg("600")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
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

fn rec_f64_any(rec: &DmapRecord, names: &[&str]) -> Option<f64> {
    names.iter().find_map(|n| rec_f64(rec, n))
}

fn unix_seconds(
    year: f64,
    month: f64,
    day: f64,
    hour: f64,
    minute: f64,
    second: f64,
) -> Option<f64> {
    let y = year as i64;
    let mo = month as i64;
    let d = day as i64;
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) {
        return None;
    }
    let days = days_from_civil(y, mo, d)?;
    Some(days as f64 * 86400.0 + hour * 3600.0 + minute * 60.0 + second)
}

fn record_epoch(rec: &DmapRecord) -> Option<f64> {
    let year = rec_f64(rec, "start.year")?;
    let month = rec_f64(rec, "start.month")?;
    let day = rec_f64(rec, "start.day")?;
    let hour = rec_f64(rec, "start.hour")?;
    let minute = rec_f64(rec, "start.minute")?;
    let second = rec_f64(rec, "start.second")?;
    let epoch = unix_seconds(year, month, day, hour, minute, second)?;
    if epoch.is_finite() && epoch > 0.0 {
        Some(epoch)
    } else {
        None
    }
}

fn cpcp_record(rec: &DmapRecord) -> Option<[f64; FIELDS]> {
    let epoch = record_epoch(rec)?;
    let cpcp = rec_f64(rec, "pot.drop")?;
    if !(cpcp.is_finite() && cpcp > 0.0) {
        return None;
    }
    let err = rec_f64_any(rec, &["pot.drop.err", "pot.drop.error"])?;
    if !err.is_finite() {
        return None;
    }
    Some([epoch, cpcp, err])
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
            let arr: [u8; 8] = b.get(p..p + 8)?.try_into().ok()?;
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
                let arr: [u8; 8] = b.get(q..q + 8)?.try_into().ok()?;
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
            n = n.checked_mul(r as usize)?;
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

fn dmap_parse(bytes: &[u8]) -> Vec<DmapRecord> {
    let mut p = 0usize;
    let mut recs = Vec::new();
    while p + 8 <= bytes.len() {
        match dmap_record_at(bytes, p) {
            Some((rec, next)) => {
                recs.push(rec);
                p = next;
            }
            None => break,
        }
    }
    recs
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    if bytes.len() != 8 + n * FIELDS * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut r = [0.0f64; FIELDS];
        for (j, slot) in r.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn emit(bytes: &[u8], out: &str, source: &str) {
    let recs = dmap_parse(bytes);
    let mut records = Vec::new();
    let mut void_records = 0usize;
    for rec in &recs {
        match cpcp_record(rec) {
            Some(r) => records.push(r),
            None => void_records += 1,
        }
    }
    if records.is_empty() {
        eprintln!(
            "{source}: {} dmap records, no CPCP record carries a finite positive pot.drop — the bin stays unwritten (0 honored)",
            recs.len()
        );
        std::process::exit(1);
    }
    let bin = write_bin(&records);
    if let Some(parsed) = parse_bin(&bin) {
        eprintln!(
            "{source}: {} dmap records, {} cpcp records, {} without a finite positive pot.drop, {} B, roundtrip parses",
            recs.len(),
            records.len(),
            void_records,
            bin.len()
        );
        if parsed.len() == records.len() {
            if let Err(e) = std::fs::write(out, &bin) {
                eprintln!("{source}: the target {out} carries not the bin: {e}");
                std::process::exit(1);
            }
            return;
        }
    }
    eprintln!("{source}: the written bin does not parse back — nothing written");
    std::process::exit(1);
}

fn probe(bytes: &[u8]) {
    let recs = dmap_parse(bytes);
    eprintln!("{} dmap records", recs.len());
    for (i, rec) in recs.iter().take(3).enumerate() {
        eprintln!(
            "record {i}: {} scalars, {} arrays",
            rec.scalars.len(),
            rec.arrays.len()
        );
        for s in &rec.scalars {
            eprintln!("  scalar {}", s.name);
        }
        for a in &rec.arrays {
            eprintln!("  array {} ({} values)", a.name, a.nums.len());
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out-bin").or_else(|| arg_value(&args, "--out")) {
        Some(o) => o,
        None => OUT_PATH.to_string(),
    };

    if let Some(path) = arg_value(&args, "--probe") {
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("{path}: the file stays unread");
            std::process::exit(1);
        };
        probe(&bytes);
        return;
    }

    let url = match arg_value(&args, "--url") {
        Some(u) => u,
        None => DEFAULT_URL.to_string(),
    };
    let bytes =
        if let Some(path) = arg_value(&args, "--file").or_else(|| arg_value(&args, "--input")) {
            match std::fs::read(&path) {
                Ok(b) => b,
                Err(_) => {
                    eprintln!("{path}: the file stays unread");
                    std::process::exit(1);
                }
            }
        } else {
            match curl_bytes(&url) {
                Some(b) => b,
                None => {
                    eprintln!("{url}: fetch void — the bin stays unwritten (0 honored)");
                    std::process::exit(1);
                }
            }
        };
    emit(&bytes, &out, &url);

    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_map_record() -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        let mut scalar = |name: &str, t: u8, value: &[u8]| {
            body.extend_from_slice(name.as_bytes());
            body.push(0);
            body.push(t);
            body.extend_from_slice(value);
        };
        scalar("start.year", 2, &2016i16.to_le_bytes());
        scalar("start.month", 2, &3i16.to_le_bytes());
        scalar("start.day", 2, &11i16.to_le_bytes());
        scalar("start.hour", 2, &5i16.to_le_bytes());
        scalar("start.minute", 2, &0i16.to_le_bytes());
        scalar("start.second", 2, &0i16.to_le_bytes());
        scalar("pot.drop", 8, &42.5f64.to_le_bytes());
        scalar("pot.drop.err", 8, &0.5f64.to_le_bytes());
        let size = 8 + body.len();
        let mut rec = Vec::with_capacity(size);
        rec.extend_from_slice(&DMAP_CODE.to_le_bytes());
        rec.extend_from_slice(&(size as u32).to_le_bytes());
        rec.extend_from_slice(&body);
        rec
    }

    fn cpcp_of(cpcp: f64) -> DmapRecord {
        DmapRecord {
            scalars: vec![
                DmapScalar {
                    name: "start.year".to_string(),
                    value: ScalarValue::I16(2016),
                },
                DmapScalar {
                    name: "start.month".to_string(),
                    value: ScalarValue::I16(3),
                },
                DmapScalar {
                    name: "start.day".to_string(),
                    value: ScalarValue::I16(11),
                },
                DmapScalar {
                    name: "start.hour".to_string(),
                    value: ScalarValue::I16(5),
                },
                DmapScalar {
                    name: "start.minute".to_string(),
                    value: ScalarValue::I16(0),
                },
                DmapScalar {
                    name: "start.second".to_string(),
                    value: ScalarValue::I16(0),
                },
                DmapScalar {
                    name: "pot.drop".to_string(),
                    value: ScalarValue::F64(cpcp),
                },
                DmapScalar {
                    name: "pot.drop.err".to_string(),
                    value: ScalarValue::F64(0.5),
                },
            ],
            arrays: Vec::new(),
        }
    }

    #[test]
    fn dmap_record_reads_cpcp_from_a_tiny_sample() {
        let rec = sample_map_record();
        let (parsed, next) = dmap_record_at(&rec, 0).expect("record parses");
        assert_eq!(next, rec.len());
        assert_eq!(cpcp_record(&parsed), Some([1_457_672_400.0, 42.5, 0.5]));
    }

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![[1_457_672_400.0, 42.5, 1.25], [1_457_675_800.0, 38.0, 2.0]];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn absent_or_nonpositive_cpcp_is_not_a_record() {
        assert_eq!(
            cpcp_record(&cpcp_of(42.5)),
            Some([1_457_672_400.0, 42.5, 0.5])
        );
        assert!(cpcp_record(&cpcp_of(f64::NAN)).is_none());
        assert!(cpcp_record(&cpcp_of(-1.0)).is_none());
        assert!(cpcp_record(&cpcp_of(0.0)).is_none());
        assert!(parse_bin(b"CPCP").is_none());
        assert!(parse_bin(b"XXXX").is_none());
    }
}
