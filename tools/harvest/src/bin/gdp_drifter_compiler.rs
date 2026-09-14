use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::zarr::{Blosc, blosc_decompress};
use omegaflow::archivar::{JsonVal, jpath_val, json_num, jstr, parse_json};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;
use std::io::{BufWriter, Write};

const NETLOC: &str = "noaa-oar-hourly-gdp-pds.s3.amazonaws.com";
const BASE: &str = "https://noaa-oar-hourly-gdp-pds.s3.amazonaws.com/latest/gdp-v2.01.1.zarr";
const MAGIC: [u8; 4] = *b"GDPT";
const REC_BYTES: usize = 40;
const FETCH_TTL: u64 = 600;

struct DrifterRecord {
    id: u64,
    time: f64,
    lon: f64,
    lat: f64,
    sst: Option<f64>,
}

enum Select {
    Index(usize),
    Buoy(u64),
}

struct Zarray {
    chunks: Vec<usize>,
    dtype: String,
    compressed: bool,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn f64_le(b: &[u8]) -> f64 {
    f64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

fn f32_le(b: &[u8]) -> f32 {
    f32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn i64_le(b: &[u8]) -> i64 {
    i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

fn i32_le(b: &[u8]) -> i32 {
    i32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn decode_f64(raw: &[u8]) -> Option<Vec<f64>> {
    if raw.len() % 8 != 0 {
        return None;
    }
    Some(raw.chunks_exact(8).map(f64_le).collect())
}

fn decode_f32(raw: &[u8]) -> Option<Vec<f64>> {
    if raw.len() % 4 != 0 {
        return None;
    }
    Some(raw.chunks_exact(4).map(|c| f32_le(c) as f64).collect())
}

fn decode_i64(raw: &[u8]) -> Option<Vec<f64>> {
    if raw.len() % 8 != 0 {
        return None;
    }
    Some(raw.chunks_exact(8).map(|c| i64_le(c) as f64).collect())
}

fn decode_i32(raw: &[u8]) -> Option<Vec<f64>> {
    if raw.len() % 4 != 0 {
        return None;
    }
    Some(raw.chunks_exact(4).map(|c| i32_le(c) as f64).collect())
}

fn decode_i8(raw: &[u8]) -> Option<Vec<f64>> {
    Some(raw.iter().map(|&b| b as i8 as f64).collect())
}

fn decode(dtype: &str, raw: &[u8]) -> Option<Vec<f64>> {
    match dtype {
        "<f8" => decode_f64(raw),
        "<f4" => decode_f32(raw),
        "<i8" => decode_i64(raw),
        "<i4" => decode_i32(raw),
        "|i1" => decode_i8(raw),
        _ => None,
    }
}

fn metadata_object(json: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match jpath_val(json, "metadata")? {
        JsonVal::Obj(map) => Some(map),
        _ => None,
    }
}

fn usize_array(v: &JsonVal) -> Option<Vec<usize>> {
    let arr = match v {
        JsonVal::Arr(a) => a,
        _ => return None,
    };
    arr.iter()
        .map(json_num)
        .map(|n| {
            let x = n?;
            if x >= 0.0 && x.fract() == 0.0 {
                Some(x as usize)
            } else {
                None
            }
        })
        .collect()
}

fn zarray_of(meta: &HashMap<String, JsonVal>, name: &str) -> Option<Zarray> {
    let arr = meta.get(&format!("{name}/.zarray"))?;
    let chunks = usize_array(jpath_val(arr, "chunks")?)?;
    let dtype = jstr(arr, "dtype")?;
    let compressed = match jpath_val(arr, "compressor") {
        None | Some(JsonVal::Null) => false,
        Some(JsonVal::Obj(_)) => jstr(arr, "compressor.id").as_deref() == Some("blosc"),
        Some(_) => return None,
    };
    Some(Zarray {
        chunks,
        dtype,
        compressed,
    })
}

fn decompress(bytes: &[u8], url: &str) -> Option<Vec<u8>> {
    match blosc_decompress(bytes) {
        Some(Blosc::Decompressed(v)) => Some(v),
        Some(Blosc::Unhandled(codec)) => {
            eprintln!("{url}: blosc codec {codec} stays unhandled");
            None
        }
        None => {
            eprintln!("{url}: blosc stream stays undecoded");
            None
        }
    }
}

fn fetch_chunk(url: &str, compressed: bool) -> Option<Vec<u8>> {
    let bytes = fetch_raw_bytes(url, FETCH_TTL)?;
    if compressed {
        decompress(&bytes, url)
    } else {
        Some(bytes)
    }
}

fn read_scalar_array(base: &str, name: &str, za: &Zarray) -> Option<Vec<f64>> {
    let url = format!("{base}/{name}/0");
    let raw = fetch_chunk(&url, za.compressed)?;
    match decode(&za.dtype, &raw) {
        Some(v) => Some(v),
        None => {
            eprintln!("{name}/0: dtype {} stream stays undecoded", za.dtype);
            None
        }
    }
}

fn read_obs_slice(
    base: &str,
    name: &str,
    za: &Zarray,
    start: usize,
    count: usize,
) -> Option<Vec<f64>> {
    let chunk = *za.chunks.first()?;
    let mut out = Vec::with_capacity(count);
    let mut remaining = count;
    let mut cursor = start;
    while remaining > 0 {
        let ci = cursor / chunk;
        let local = cursor % chunk;
        let take = remaining.min(chunk - local);
        let url = format!("{base}/{name}/{ci}");
        let raw = fetch_chunk(&url, za.compressed)?;
        let vals = match decode(&za.dtype, &raw) {
            Some(v) => v,
            None => {
                eprintln!("{name}/{ci}: dtype {} stream stays undecoded", za.dtype);
                return None;
            }
        };
        for k in local..local + take {
            out.push(*vals.get(k)?);
        }
        cursor += take;
        remaining -= take;
    }
    Some(out)
}

fn records_for(
    id: u64,
    time: &[f64],
    lon: &[f64],
    lat: &[f64],
    sst: &[f64],
) -> (Vec<DrifterRecord>, usize) {
    let n = time.len().min(lon.len()).min(lat.len()).min(sst.len());
    let mut out = Vec::with_capacity(n);
    let mut skipped = 0usize;
    for i in 0..n {
        let t = time[i];
        let lo = lon[i];
        let la = lat[i];
        let s = sst[i];
        if !(t.is_finite() && t > 0.0)
            || !lo.is_finite()
            || !(la.is_finite() && (-90.0..=90.0).contains(&la))
        {
            skipped += 1;
            continue;
        }
        let sst = if s.is_finite() && s > 0.0 {
            Some(s)
        } else {
            None
        };
        out.push(DrifterRecord {
            id,
            time: t,
            lon: lo,
            lat: la,
            sst,
        });
    }
    (out, skipped)
}

struct Arrays {
    base: String,
    id_vals: Vec<f64>,
    rs_vals: Vec<f64>,
    time_arr: Zarray,
    lon_arr: Zarray,
    lat_arr: Zarray,
    sst_arr: Zarray,
}

struct Trajectory {
    id: u64,
    count: usize,
    records: Vec<DrifterRecord>,
    skipped: usize,
}

enum Outcome {
    Measured(Trajectory),
    Absent(String),
}

fn load_arrays(base: &str) -> Result<Arrays, String> {
    let zmeta_url = format!("{base}/.zmetadata");
    let zmeta_bytes = fetch_raw_bytes(&zmeta_url, FETCH_TTL)
        .ok_or_else(|| format!("{zmeta_url}: fetch returned void"))?;
    let zmeta_text = String::from_utf8_lossy(&zmeta_bytes).into_owned();
    let zjson =
        parse_json(&zmeta_text).ok_or_else(|| format!("{zmeta_url}: JSON stays unparsed"))?;
    let meta = metadata_object(&zjson)
        .ok_or_else(|| format!("{zmeta_url}: metadata object stays absent"))?;

    let id_arr = zarray_of(meta, "id").ok_or_else(|| "id/.zarray stays unread".to_string())?;
    let rs_arr =
        zarray_of(meta, "rowsize").ok_or_else(|| "rowsize/.zarray stays unread".to_string())?;
    let time_arr =
        zarray_of(meta, "time").ok_or_else(|| "time/.zarray stays unread".to_string())?;
    let lon_arr = zarray_of(meta, "lon").ok_or_else(|| "lon/.zarray stays unread".to_string())?;
    let lat_arr = zarray_of(meta, "lat").ok_or_else(|| "lat/.zarray stays unread".to_string())?;
    let sst_arr = zarray_of(meta, "sst").ok_or_else(|| "sst/.zarray stays unread".to_string())?;

    let id_vals = read_scalar_array(base, "id", &id_arr)
        .ok_or_else(|| "id: scalar array stays unread".to_string())?;
    let rs_vals = read_scalar_array(base, "rowsize", &rs_arr)
        .ok_or_else(|| "rowsize: scalar array stays unread".to_string())?;
    if id_vals.len() != rs_vals.len() {
        return Err(format!(
            "id ({}) and rowsize ({}) carry different trajectory counts",
            id_vals.len(),
            rs_vals.len()
        ));
    }
    Ok(Arrays {
        base: base.to_string(),
        id_vals,
        rs_vals,
        time_arr,
        lon_arr,
        lat_arr,
        sst_arr,
    })
}

fn trajectory_records(m: &Arrays, t: usize) -> Result<Outcome, String> {
    if t >= m.id_vals.len() {
        return Err(format!(
            "trajectory index {t}: beyond {} trajectories",
            m.id_vals.len()
        ));
    }
    let id_f = m.id_vals[t];
    if !(id_f > 0.0) {
        return Ok(Outcome::Absent("buoy id stays absent".to_string()));
    }
    let id = id_f as u64;
    let count = m.rs_vals[t] as usize;
    if count == 0 {
        return Ok(Outcome::Absent(
            "rowsize 0 — no observations (0 honored)".to_string(),
        ));
    }
    let start: usize = m.rs_vals[..t].iter().map(|&x| x as usize).sum();

    let time = read_obs_slice(&m.base, "time", &m.time_arr, start, count)
        .ok_or_else(|| format!("trajectory {t}: time slice stays unread"))?;
    let lon = read_obs_slice(&m.base, "lon", &m.lon_arr, start, count)
        .ok_or_else(|| format!("trajectory {t}: lon slice stays unread"))?;
    let lat = read_obs_slice(&m.base, "lat", &m.lat_arr, start, count)
        .ok_or_else(|| format!("trajectory {t}: lat slice stays unread"))?;
    let sst = read_obs_slice(&m.base, "sst", &m.sst_arr, start, count)
        .ok_or_else(|| format!("trajectory {t}: sst slice stays unread"))?;

    let (records, skipped) = records_for(id, &time, &lon, &lat, &sst);
    if records.is_empty() {
        return Ok(Outcome::Absent(
            "no valid record — every observation carries an absent position/time (0 honored)"
                .to_string(),
        ));
    }
    Ok(Outcome::Measured(Trajectory {
        id,
        count,
        records,
        skipped,
    }))
}

fn gather(base: &str, select: &Select) -> Result<Vec<DrifterRecord>, String> {
    let m = load_arrays(base)?;
    let t = match select {
        Select::Index(i) => *i,
        Select::Buoy(w) => m
            .id_vals
            .iter()
            .position(|&x| x as u64 == *w)
            .ok_or_else(|| format!("buoy {w}: no trajectory carries this id"))?,
    };
    match trajectory_records(&m, t)? {
        Outcome::Measured(traj) => {
            eprintln!(
                "gdp: trajectory {t} buoy {}, {} observations, {} records, {} observations skipped (absent/implausible position or clock)",
                traj.id,
                traj.count,
                traj.records.len(),
                traj.skipped
            );
            Ok(traj.records)
        }
        Outcome::Absent(reason) => Err(format!("trajectory {t}: {reason}")),
    }
}

fn gather_all(base: &str) -> Result<Vec<DrifterRecord>, String> {
    let m = load_arrays(base)?;
    let n_traj = m.id_vals.len();
    let mut out = Vec::new();
    let mut absent = 0usize;
    let mut unread = 0usize;
    for t in 0..n_traj {
        match trajectory_records(&m, t) {
            Ok(Outcome::Measured(traj)) => out.extend(traj.records),
            Ok(Outcome::Absent(_)) => absent += 1,
            Err(_) => unread += 1,
        }
    }
    if out.is_empty() {
        return Err(
            "no trajectory carries a valid record — the asset stays unwritten (0 honored)".into(),
        );
    }
    eprintln!(
        "gdp: {n_traj} trajectories, {} records manifested, {} trajectories absent, {} trajectories unread",
        out.len(),
        absent,
        unread
    );
    Ok(out)
}

fn encode_record(rec: &mut [u8; REC_BYTES], r: &DrifterRecord) {
    rec[0..8].copy_from_slice(&r.id.to_le_bytes());
    rec[8..16].copy_from_slice(&r.time.to_le_bytes());
    rec[16..24].copy_from_slice(&r.lon.to_le_bytes());
    rec[24..32].copy_from_slice(&r.lat.to_le_bytes());
    match r.sst {
        Some(v) => {
            rec[32..36].copy_from_slice(&(v as f32).to_le_bytes());
            rec[36] = 1;
        }
        None => {
            rec[32..36].copy_from_slice(&0.0f32.to_le_bytes());
            rec[36] = 0;
        }
    }
    rec[37..40].copy_from_slice(&[0, 0, 0]);
}

fn decode_record(rec: &[u8]) -> Option<DrifterRecord> {
    if rec.len() != REC_BYTES {
        return None;
    }
    let id = u64::from_le_bytes(rec[0..8].try_into().ok()?);
    let time = f64::from_le_bytes(rec[8..16].try_into().ok()?);
    let lon = f64::from_le_bytes(rec[16..24].try_into().ok()?);
    let lat = f64::from_le_bytes(rec[24..32].try_into().ok()?);
    let sst = f32::from_le_bytes(rec[32..36].try_into().ok()?);
    if !time.is_finite() || !lon.is_finite() || !lat.is_finite() {
        return None;
    }
    let sst = match rec[36] {
        1 if sst.is_finite() && sst > 0.0 => Some(sst as f64),
        _ => None,
    };
    Some(DrifterRecord {
        id,
        time,
        lon,
        lat,
        sst,
    })
}

fn write_asset(records: &[DrifterRecord], out_path: &str) -> Result<usize, String> {
    let file = std::fs::File::create(out_path).map_err(|e| format!("create {out_path}: {e}"))?;
    let mut out = BufWriter::new(file);
    out.write_all(&MAGIC)
        .map_err(|e| format!("write {out_path}: {e}"))?;
    out.write_all(&(records.len() as u32).to_le_bytes())
        .map_err(|e| format!("write {out_path}: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for r in records {
        encode_record(&mut rec, r);
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path}: {e}"))?;
    }
    out.flush().map_err(|e| format!("flush {out_path}: {e}"))?;
    let expect = 8 + records.len() * REC_BYTES;
    let actual = std::fs::metadata(out_path)
        .map_err(|e| format!("stat {out_path}: {e}"))?
        .len() as usize;
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }
    Ok(expect)
}

fn parse_asset(bytes: &[u8]) -> Option<Vec<DrifterRecord>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let rec = bytes.get(off..off + REC_BYTES)?;
        out.push(decode_record(rec)?);
        off += REC_BYTES;
    }
    Some(out)
}

fn sst_label(r: &DrifterRecord) -> String {
    match r.sst {
        Some(v) => format!("{v:.3} K"),
        None => "absent".to_string(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: gdp_drifter_compiler --out <path> [--index <n> | --id <buoy-id> | --all] [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };
    let all = args.iter().any(|a| a == "--all");
    let select: Option<Select> = if all {
        None
    } else if let Some(s) = arg_value(&args, "--id") {
        match s.parse::<u64>() {
            Ok(w) => Some(Select::Buoy(w)),
            Err(_) => {
                eprintln!("--id needs a numeric buoy id");
                std::process::exit(1);
            }
        }
    } else {
        Some(Select::Index(
            match arg_value(&args, "--index").and_then(|v| v.parse::<usize>().ok()) {
                Some(v) => v,
                None => 0,
            },
        ))
    };

    let records = match select {
        None => match gather_all(BASE) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("gdp_drifter_compiler: {e}");
                std::process::exit(1);
            }
        },
        Some(sel) => match gather(BASE, &sel) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("gdp_drifter_compiler: {e}");
                std::process::exit(1);
            }
        },
    };

    if let Err(e) = write_asset(&records, &out_path) {
        eprintln!("gdp_drifter_compiler: {e}");
        std::process::exit(1);
    }
    let written = match std::fs::read(&out_path) {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "gdp_drifter_compiler: {out_path} read returned void — the roundtrip stays unverified"
            );
            std::process::exit(1);
        }
    };
    match parse_asset(&written) {
        Some(parsed) if parsed.len() == records.len() => {
            let last = parsed.last().unwrap();
            eprintln!(
                "gdp: {} records, {} B -> {out_path}, roundtrip parses; last buoy {} time {:.3} lon {:.4} lat {:.4} sst {}",
                parsed.len(),
                written.len(),
                last.id,
                last.time,
                last.lon,
                last.lat,
                sst_label(last)
            );
        }
        _ => {
            eprintln!("{out_path}: roundtrip parse returned void — the bin stays unverified");
            std::process::exit(1);
        }
    }

    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("upload: {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_le_widths() {
        assert_eq!(decode_f64(&1.5f64.to_le_bytes()).unwrap(), vec![1.5]);
        assert_eq!(decode_f32(&2.5f32.to_le_bytes()).unwrap(), vec![2.5]);
        assert_eq!(decode_i64(&(-7i64).to_le_bytes()).unwrap(), vec![-7.0]);
        assert_eq!(decode_i32(&(-3i32).to_le_bytes()).unwrap(), vec![-3.0]);
        assert_eq!(decode_i8(&[0xff, 0x01]).unwrap(), vec![-1.0, 1.0]);
        assert!(decode_f64(&[0u8; 7]).is_none());
        assert!(decode("<f9", &[0u8; 8]).is_none());
    }

    #[test]
    fn record_roundtrip_preserves_sst_absence() {
        let present = DrifterRecord {
            id: 7_123_456,
            time: 1_704_067_204.5,
            lon: -150.25,
            lat: 12.5,
            sst: Some(300.15),
        };
        let absent = DrifterRecord {
            id: 7_123_456,
            time: 1_704_067_204.5,
            lon: -150.25,
            lat: 12.5,
            sst: None,
        };
        let mut rec = [0u8; REC_BYTES];
        encode_record(&mut rec, &present);
        let back = decode_record(&rec).unwrap();
        assert_eq!(back.id, present.id);
        assert_eq!(back.time, present.time);
        assert_eq!(back.lon, present.lon);
        assert_eq!(back.lat, present.lat);
        assert!(back.sst.is_some());
        encode_record(&mut rec, &absent);
        let back = decode_record(&rec).unwrap();
        assert!(back.sst.is_none());
        assert!(decode_record(&rec[..REC_BYTES - 1]).is_none());
    }

    #[test]
    fn records_for_skips_absent_position_and_flags_sst() {
        let id = 7_123_456u64;
        let time = vec![1.0, f64::NAN, 3.0, 4.0];
        let lon = vec![10.0, 20.0, f64::NAN, 40.0];
        let lat = vec![11.0, 22.0, 33.0, 444.0];
        let sst = vec![300.0, f64::NAN, 302.0, 301.0];
        let (records, skipped) = records_for(id, &time, &lon, &lat, &sst);
        assert_eq!(skipped, 3);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].sst, Some(300.0));
        assert_eq!(records[0].lon, 10.0);
        assert_eq!(records[0].lat, 11.0);
    }

    #[test]
    fn zarray_of_reads_chunks_dtype_compressor() {
        let j = parse_json(
            r#"{"metadata":{"time/.zarray":{"shape":[197214787],"chunks":[385186],"dtype":"<f8","compressor":{"id":"blosc","cname":"lz4","clevel":5,"shuffle":1},"fill_value":null,"order":"C"}}}"#,
        )
        .unwrap();
        let meta = metadata_object(&j).unwrap();
        let za = zarray_of(meta, "time").unwrap();
        assert_eq!(za.chunks, vec![385186]);
        assert_eq!(za.dtype, "<f8");
        assert!(za.compressed);

        let j2 = parse_json(
            r#"{"metadata":{"raw/.zarray":{"shape":[10],"chunks":[10],"dtype":"<i4","compressor":null}}}"#,
        )
        .unwrap();
        let meta2 = metadata_object(&j2).unwrap();
        let raw = zarray_of(meta2, "raw").unwrap();
        assert!(!raw.compressed);
        assert_eq!(raw.dtype, "<i4");
    }

    #[test]
    fn asset_roundtrip_and_rejections() {
        let records = vec![
            DrifterRecord {
                id: 7_123_456,
                time: 1_704_067_204.5,
                lon: -150.25,
                lat: 12.5,
                sst: Some(300.5),
            },
            DrifterRecord {
                id: 7_123_456,
                time: 1_704_067_204.5,
                lon: -150.75,
                lat: 12.6,
                sst: None,
            },
        ];
        let out = "/tmp/opencode/gdp_drifter_test_asset.bin";
        let bytes = write_asset(&records, out).unwrap();
        assert_eq!(bytes, 8 + records.len() * REC_BYTES);
        let read = std::fs::read(out).unwrap();
        let parsed = parse_asset(&read).unwrap();
        assert_eq!(parsed.len(), records.len());
        assert_eq!(parsed[0].sst, Some(300.5));
        assert_eq!(parsed[1].sst, None);

        assert!(parse_asset(b"X").is_none());
        assert!(parse_asset(b"GDPTabc").is_none());
        assert!(parse_asset(&read[..read.len() - 1]).is_none());
    }
}
