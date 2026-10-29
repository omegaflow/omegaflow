use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mariner_rst;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::{gunzip, tar_members};
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "spdf.gsfc.nasa.gov";
const FORMAT: &str = "mariner_rst";
const URL: &str = "https://spdf.gsfc.nasa.gov/pub/data/mariner/mariner6/celestial_mechanics/2_way_doppler_radio_tracking/PSCM-00017_DD005408_05-MAR-69.tar";

const HEADER_RECORDS: usize = 3;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn words_of(payload: &[u8]) -> Vec<u64> {
    payload
        .chunks_exact(6)
        .map(|c| {
            let mut w = 0u64;
            for &b in c {
                w = (w << 6) | (b & 0x3F) as u64;
            }
            w
        })
        .collect()
}

const RST_POINT_WORDS: usize = 21;
const TWO_WAY_DOPPLER: u32 = 8;

fn single(w: u64) -> f64 {
    let sign = if (w >> 35) & 1 == 1 { -1.0 } else { 1.0 };
    let exp = ((w >> 27) & 0xFF) as i32;
    let frac = (w & 0x07FF_FFFF) as f64 / (2f64.powi(27));
    sign * frac * 2f64.powi(exp - 128)
}

fn rst_points(data: &[u8]) -> Vec<(f64, f64, u32)> {
    let mut out = Vec::new();
    let Some(epoch) = days_from_civil(1950, 1, 1).map(|d| d as f64 * 86400.0) else {
        return out;
    };
    let mut off = 0usize;
    while off + 2 <= data.len() {
        let len = u16::from_be_bytes([data[off], data[off + 1]]) as usize;
        let start = off + 2;
        let end = start + len;
        if len == 0 || end > data.len() {
            break;
        }
        let words = words_of(&data[start..end]);
        if words.len() == RST_POINT_WORDS {
            let t = epoch + single(words[2]);
            if (-252_460_800.0..=189_345_600.0).contains(&t) {
                out.push((t, single(words[19]), TWO_WAY_DOPPLER));
            }
        }
        off = end;
    }
    out
}

fn unix_from_words(w2: u64, w3: u64) -> Option<f64> {
    let v: u128 = ((w2 as u128) << 36) | (w3 as u128);
    let exp = ((v >> 63) & 0xFF) as i32;
    let frac = (v & ((1u128 << 63) - 1)) as f64;
    let sign = if (v >> 71) & 1 == 1 { -1.0 } else { 1.0 };
    let seconds = sign * (frac / (2f64.powi(63))) * 2f64.powi(exp - 128);
    if !seconds.is_finite() {
        return None;
    }
    let epoch1950 = (days_from_civil(1950, 1, 1)? as f64) * 86400.0;
    Some(epoch1950 + seconds)
}

fn code_bits(code: u64) -> Vec<u32> {
    (1..=12u32)
        .filter(|&k| (code >> (36 - k)) & 1 == 1)
        .collect()
}

const SP_ODG_MASTER_LABEL_WORDS: usize = 10;
const SP_ODG_STATION_SUMMARY_WORDS: usize = 201;
const SP_ODG_SUMMARY_CONTINUATION_WORDS: usize = 97;
const SP_ODG_DATA_PAYLOAD_WORDS: usize = 200;
const SP_ODG_DATA_RECORD_WORDS: usize = 201;
const SP_ODG_MIN_LOGICAL_WORDS: usize = 6;
const SP_ODG_MAX_LOGICAL_WORDS: usize = 36;

fn sp_odg_header_words() -> usize {
    SP_ODG_MASTER_LABEL_WORDS + SP_ODG_STATION_SUMMARY_WORDS + SP_ODG_SUMMARY_CONTINUATION_WORDS
}

fn sp_odg_logical_records(words: &[u64]) -> Vec<&[u64]> {
    let mut out = Vec::new();
    let mut rec = sp_odg_header_words();
    while rec + SP_ODG_DATA_RECORD_WORDS <= words.len() {
        let Some(payload) = words.get(rec..rec + SP_ODG_DATA_PAYLOAD_WORDS) else {
            break;
        };
        let mut p = 0usize;
        while p < payload.len() {
            let d = payload[p] as usize;
            if d < SP_ODG_MIN_LOGICAL_WORDS || d > SP_ODG_MAX_LOGICAL_WORDS || d % 2 != 0 {
                break;
            }
            let Some(record) = payload.get(p..p + d) else {
                break;
            };
            out.push(record);
            p += d;
        }
        rec += SP_ODG_DATA_RECORD_WORDS;
    }
    out
}

fn logical_code_bits(code: u64) -> Vec<u32> {
    (1..=15u32)
        .filter(|&k| (code >> (36 - k)) & 1 == 1)
        .collect()
}

fn sp_odg_record_observables(record: &[u64]) -> Vec<(f64, f64, u32)> {
    let Some(&dw) = record.first() else {
        return Vec::new();
    };
    let d = dw as usize;
    if d < SP_ODG_MIN_LOGICAL_WORDS
        || d > SP_ODG_MAX_LOGICAL_WORDS
        || d % 2 != 0
        || d != record.len()
    {
        return Vec::new();
    }
    let (Some(&w1), Some(&w2), Some(&code)) = (record.get(1), record.get(2), record.get(5)) else {
        return Vec::new();
    };
    let bits = logical_code_bits(code);
    if 6 + 2 * bits.len() != d {
        return Vec::new();
    }
    let Some(t) = unix_from_words(w1, w2) else {
        return Vec::new();
    };
    if !(-252_460_800.0..=189_345_600.0).contains(&t) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (pair, &dtype) in bits.iter().enumerate() {
        let Some(&obs) = record.get(6 + 2 * pair) else {
            break;
        };
        let v = single(obs);
        if v.is_finite() {
            out.push((t, v, dtype));
        }
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let sp = args.iter().any(|a| a == "--sp");
    let out = match arg_value(args, "--out-bin") {
        Some(v) => v,
        None => "mariner_rst.bin".to_string(),
    };

    let raw = match arg_value(args, "--in") {
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => fetch_raw_bytes(URL).ok_or_else(|| format!("{URL}: fetch void"))?,
    };
    let mut rows: Vec<(f64, f64, u32)> = Vec::new();

    let gunzipped;
    let dats: Vec<&[u8]> = if let Some(members) = tar_members(&raw) {
        members
            .iter()
            .filter(|m| m.name.ends_with("_F1.DAT"))
            .filter_map(|m| raw.get(m.start..m.end))
            .collect()
    } else if let Some(t) = gunzip(&raw) {
        gunzipped = t;
        let members = tar_members(&gunzipped)
            .ok_or_else(|| format!("{URL}: tar stream does not enumerate"))?;
        members
            .iter()
            .filter(|m| m.name.ends_with("_F1.DAT"))
            .filter_map(|m| gunzipped.get(m.start..m.end))
            .collect()
    } else {
        vec![&raw[..]]
    };

    for data in dats {
        if !sp {
            rows.extend(rst_points(data));
            continue;
        }
        if sp {
            let mut words = Vec::new();
            let mut off = 0usize;
            while off + 2 <= data.len() {
                let len = u16::from_be_bytes([data[off], data[off + 1]]) as usize;
                let payload = off + 2;
                let end = payload + len;
                if len == 0 || end > data.len() {
                    break;
                }
                words.extend(words_of(&data[payload..end]));
                off = end;
            }
            for record in sp_odg_logical_records(&words) {
                rows.extend(sp_odg_record_observables(record));
            }
            continue;
        }
        let mut off = 0usize;
        let mut index = 0usize;
        while off + 2 <= data.len() {
            let len = u16::from_be_bytes([data[off], data[off + 1]]) as usize;
            let payload = off + 2;
            let end = payload + len;
            if len == 0 || end > data.len() {
                break;
            }
            let words = words_of(&data[payload..end]);
            if index >= HEADER_RECORDS && words.len() >= 8 {
                if let Some(t) = unix_from_words(words[2], words[3]) {
                    if !(-252_460_800.0..=189_345_600.0).contains(&t) {
                        off = end;
                        index += 1;
                        continue;
                    }
                    let bits = code_bits(words[7]);
                    for (j, &dtype) in bits.iter().enumerate() {
                        if let Some(&w) = words.get(8 + j) {
                            rows.push((t, single(w), dtype));
                        }
                    }
                }
            }
            off = end;
            index += 1;
        }
    }
    if rows.is_empty() {
        return Err("no observables parsed — the bin stays unwritten (0 honored)".to_string());
    }
    let bin = mariner_rst::write_bin(&rows);
    if mariner_rst::parse_bin(&bin).is_none() {
        return Err(format!("{out}: roundtrip parse void"));
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    let t_min = rows.iter().map(|r| r.0).fold(f64::INFINITY, f64::min);
    let t_max = rows.iter().map(|r| r.0).fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} observables, t [{t_min:.0}..{t_max:.0}] unix, format {FORMAT}, sha256 {}",
        rows.len(),
        sha256_hex(&bin)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    omegaflow::archivar::cdn::require_ci();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: mariner_rst_compiler [--in <F1.DAT|tar>] [--out-bin <name>] [--sp] [--ci-mode]"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("mariner_rst_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack_words(words: &[u64]) -> Vec<u8> {
        let mut out = Vec::with_capacity(words.len() * 6);
        for &w in words {
            for shift in (0..36).step_by(6).rev() {
                out.push(((w >> shift) & 0x3F) as u8);
            }
        }
        out
    }

    fn dp_time_words(seconds: u64) -> (u64, u64) {
        let frac: u128 = (seconds as u128) * 2;
        let exp_field: u128 = 128 + 62;
        let v: u128 = (exp_field << 63) | (frac & ((1u128 << 63) - 1));
        ((v >> 36) as u64, (v & ((1u128 << 36) - 1)) as u64)
    }

    fn logical_record(code: u64, pairs: &[u64], t1: u64, t2: u64) -> Vec<u64> {
        let mut record = vec![0u64; 6 + 2 * pairs.len()];
        record[0] = record.len() as u64;
        record[1] = t1;
        record[2] = t2;
        record[5] = code;
        for (k, &obs) in pairs.iter().enumerate() {
            record[6 + 2 * k] = obs;
        }
        record
    }

    const DATA_PAYLOAD_WORDS: usize = 200;

    fn data_record(payload: &[u64]) -> Vec<u64> {
        assert!(payload.len() <= DATA_PAYLOAD_WORDS);
        let mut record = payload.to_vec();
        record.resize(DATA_PAYLOAD_WORDS, 0);
        record.push(0);
        record
    }

    fn stream_payload(payload: &[u64]) -> Vec<u64> {
        let mut words = vec![0u64; sp_odg_header_words()];
        words.extend_from_slice(&data_record(payload));
        words_of(&pack_words(&words))
    }

    fn stream(records: &[Vec<u64>]) -> Vec<u64> {
        let mut payload = Vec::new();
        for record in records {
            payload.extend_from_slice(record);
        }
        stream_payload(&payload)
    }

    #[test]
    fn sp_odg_two_datatypes_read_variable_d() {
        let (t1, t2) = dp_time_words(605_059_200);
        let code = (1u64 << 35) | (1u64 << 27);
        let record = logical_record(
            code,
            &[(64u64 << 28) | (1u64 << 27), (64u64 << 28) | (1u64 << 26)],
            t1,
            t2,
        );
        assert_eq!(record[0], 10);
        let decoded = stream(&[record]);
        let records = sp_odg_logical_records(&decoded);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0][0], 10);
        let obs = sp_odg_record_observables(records[0]);
        assert_eq!(obs.len(), 2);
        assert_eq!(obs[0].2, 1);
        assert_eq!(obs[1].2, 9);
        assert!((obs[0].1 - 0.5).abs() < 1e-12);
        assert!((obs[1].1 - 0.25).abs() < 1e-12);
    }

    #[test]
    fn sp_odg_single_datatype_read_variable_d() {
        let (t1, t2) = dp_time_words(605_059_200);
        let code = 1u64 << 35;
        let record = logical_record(code, &[(64u64 << 28) | (1u64 << 27)], t1, t2);
        assert_eq!(record[0], 8);
        let decoded = stream(&[record]);
        assert_eq!(
            decoded.len(),
            sp_odg_header_words() + SP_ODG_DATA_RECORD_WORDS,
            "308-word header + one 201-word data record"
        );
        let records = sp_odg_logical_records(&decoded);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0][0], 8);
        let obs = sp_odg_record_observables(records[0]);
        assert_eq!(obs.len(), 1);
        assert_eq!(obs[0].2, 1);
        assert!((obs[0].1 - 0.5).abs() < 1e-12);
    }

    #[test]
    fn sp_odg_two_logical_records_in_one_payload() {
        let (t1, t2) = dp_time_words(605_059_200);
        let r8 = logical_record(1u64 << 35, &[(64u64 << 28) | (1u64 << 27)], t1, t2);
        let code = (1u64 << 35) | (1u64 << 27);
        let r10 = logical_record(
            code,
            &[(64u64 << 28) | (1u64 << 27), (64u64 << 28) | (1u64 << 26)],
            t1,
            t2,
        );
        assert_eq!(r8[0], 8);
        assert_eq!(r10[0], 10);
        let decoded = stream(&[r8, r10]);
        let records = sp_odg_logical_records(&decoded);
        assert_eq!(
            records.len(),
            2,
            "two logical records inside one data record"
        );
        let total: usize = records
            .iter()
            .map(|r| sp_odg_record_observables(r).len())
            .sum();
        assert_eq!(
            total, 3,
            "one observable (bit 1) + two observables (bits 1,9)"
        );
    }

    #[test]
    fn sp_odg_partial_payload_tail_dropped() {
        let (t1, t2) = dp_time_words(605_059_200);
        let full = logical_record(1u64 << 35, &[(64u64 << 28) | (1u64 << 27)], t1, t2);
        assert_eq!(full.len(), 8);
        let mut payload = Vec::new();
        while payload.len() + full.len() <= DATA_PAYLOAD_WORDS - 8 {
            payload.extend_from_slice(&full);
        }
        let complete = payload.len() / full.len();
        payload.push(10);
        let decoded = stream_payload(&payload);
        let records = sp_odg_logical_records(&decoded);
        assert_eq!(
            records.len(),
            complete,
            "the partial tail record is dropped"
        );
        assert!(
            records
                .iter()
                .all(|r| sp_odg_record_observables(r).len() == 1)
        );
    }

    #[test]
    fn sp_odg_code_length_mismatch_stays_empty() {
        let (t1, t2) = dp_time_words(605_059_200);
        let record = logical_record(1u64 << 35, &[64u64 << 27, 1u64 << 27], t1, t2);
        assert_eq!(record[0], 10);
        let decoded = stream(&[record]);
        let records = sp_odg_logical_records(&decoded);
        assert_eq!(records.len(), 1);
        assert!(sp_odg_record_observables(records[0]).is_empty());
    }

    #[test]
    fn the_first_mariner6_point_lands_on_its_label() {
        let hex = "000014000001000000000013133420162f19101e00000000122f300128000f100000000011142c0000000d380000000013163100000500050300003c03000501070301000101070003000501070300000000000000000000000000000000000000000000000000000000000000080319231b1206113122340e2e062d2920";
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex"))
            .collect();
        let mut framed = Vec::new();
        framed.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
        framed.extend_from_slice(&bytes);
        let rows = rst_points(&framed);
        assert_eq!(rows.len(), 1, "{rows:?}");
        assert!(
            (rows[0].0 - (-26_427_064.0)).abs() < 1.0,
            "t = {}",
            rows[0].0
        );
        assert!(
            (rows[0].1 - 51_427.087_891).abs() < 1e-3,
            "obs = {}",
            rows[0].1
        );
        assert_eq!(rows[0].2, TWO_WAY_DOPPLER);
    }
}
