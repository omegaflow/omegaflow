use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{LeapSeconds, embedded_lsk};
use std::collections::BTreeSet;

const BASE: &str = "https://ssd-api.jpl.nasa.gov/sb_radar.api";
const NETLOC: &str = "ssd-api.jpl.nasa.gov";
const ASSET: &str = "sb_radar.bin";
const FORMAT: &str = "sb_radar";
const COMPILER: &str = "tools/harvest/src/bin/sb_radar_compiler.rs";

const MAGIC: [u8; 4] = *b"SBRD";
const REC_BYTES: usize = 26 * 8;
const MICROSECONDS: f64 = 1.0e-6;
const TTL_S: f64 = 86400.0;
const TAU_S: f64 = 86400.0;
const KERNEL_INVERSE_SQUARE: f64 = 0.0;
const FORCE_EM: f64 = 0.0;

const SLOT_VAL: usize = 3;
const SLOT_EPOCH: usize = 4;
const SLOT_TTL: usize = 5;
const SLOT_TAU: usize = 6;
const SLOT_KERNEL: usize = 8;
const SLOT_FORCE: usize = 9;
const SLOT_FREQ: usize = 22;
const SLOT_BIN_WIDTH: usize = 23;
const SLOT_PHASE: usize = 24;
const SLOT_PRESENCE: usize = 25;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn positional(args: &[String]) -> Option<String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--params" || args[i] == "--emit" {
            i += 2;
            continue;
        }
        if !args[i].starts_with('-') {
            return Some(args[i].clone());
        }
        i += 1;
    }
    None
}

fn scalar_text(v: &JsonVal) -> Option<String> {
    match v {
        JsonVal::Str(s) => Some(s.clone()),
        JsonVal::Num(n) => Some(format!("{n}")),
        JsonVal::Bool(b) => Some(b.to_string()),
        JsonVal::Null => None,
        JsonVal::Arr(_) | JsonVal::Obj(_) => None,
    }
}

fn field_index(fields: &[String], name: &str) -> Option<usize> {
    fields.iter().position(|f| f == name)
}

fn cell_at<'a>(cells: &'a [JsonVal], fields: &[String], name: &str) -> Option<&'a JsonVal> {
    cells.get(field_index(fields, name)?)
}

fn parse_epoch_utc(stamp: &str) -> Option<f64> {
    let (date, time) = stamp.split_once(' ')?;
    let mut d = date.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hour: i64 = t.next()?.parse().ok()?;
    let minute: i64 = t.next()?.parse().ok()?;
    let second: f64 = t.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0 + second)
}

fn record_bytes(cells: &[JsonVal], fields: &[String], lsk: &LeapSeconds) -> Option<Vec<u8>> {
    let units = cell_at(cells, fields, "units").and_then(scalar_text)?;
    if units != "us" {
        return None;
    }
    let value_us: f64 = cell_at(cells, fields, "value")
        .and_then(scalar_text)?
        .parse()
        .ok()?;
    if !value_us.is_finite() || value_us <= 0.0 {
        return None;
    }
    let epoch_utc = cell_at(cells, fields, "epoch").and_then(scalar_text)?;
    let epoch_tdb = lsk.unix_to_tdb(parse_epoch_utc(&epoch_utc)?)?;
    let mut r = [0.0f64; 26];
    r[SLOT_VAL] = value_us * MICROSECONDS;
    r[SLOT_EPOCH] = epoch_tdb;
    r[SLOT_TTL] = TTL_S;
    r[SLOT_TAU] = TAU_S;
    r[SLOT_KERNEL] = KERNEL_INVERSE_SQUARE;
    r[SLOT_FORCE] = FORCE_EM;
    r[SLOT_FREQ] = 0.0;
    r[SLOT_BIN_WIDTH] = 0.0;
    r[SLOT_PHASE] = 0.0;
    r[SLOT_PRESENCE] = 1.0;
    let mut out = Vec::with_capacity(REC_BYTES);
    for v in r {
        out.extend_from_slice(&v.to_le_bytes());
    }
    Some(out)
}

fn write_bin(records: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        out.extend_from_slice(r);
    }
    out
}

fn emit_delay(data: &[JsonVal], fields: &[String], out_path: &str) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the epoch stays absent".to_string()
    })?;
    let mut records: Vec<Vec<u8>> = Vec::new();
    let mut held = 0usize;
    let mut receiver: BTreeSet<String> = BTreeSet::new();
    let mut transmitter: BTreeSet<String> = BTreeSet::new();
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        if let Some(v) = cell_at(cells, fields, "rcvr").and_then(scalar_text) {
            receiver.insert(v);
        }
        if let Some(v) = cell_at(cells, fields, "xmit").and_then(scalar_text) {
            transmitter.insert(v);
        }
        match record_bytes(cells, fields, &lsk) {
            Some(rec) => records.push(rec),
            None => held += 1,
        }
    }
    if records.is_empty() {
        return Err(
            "no delay row carries a positive value — the asset stays unwritten (0 honored)"
                .to_string(),
        );
    }
    let bin = write_bin(&records);
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| format!("create {out_path} void: {e}"))?;
        }
    }
    std::fs::write(out_path, &bin).map_err(|e| format!("write {out_path} void: {e}"))?;
    eprintln!(
        "{out_path}: {} delay records, {held} held, receiver {receiver:?}, transmitter {transmitter:?}, {} B, stride {REC_BYTES}",
        records.len(),
        bin.len()
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: sb_radar_compiler <query> | --params <k=v&k=v> [--inspect] [--emit <path>|delay]"
        );
        eprintln!("  --params <k=v&k=v> the URL query string appended to {BASE}");
        eprintln!("  --inspect          print the response field names and stop");
        eprintln!(
            "  --emit <path>      write one 208-byte wire record per radar delay row (units us)"
        );
        eprintln!("  --emit delay       the same, to data/{NETLOC}/{ASSET}");
        eprintln!(
            "  without --emit the compiler prints field=value rows only and writes no wire record"
        );
        std::process::exit(2);
    }
    let inspect = args.iter().any(|a| a == "--inspect");
    let emit = args.iter().any(|a| a == "--emit");
    let params = match arg_value(&args, "--params") {
        Some(p) if !p.is_empty() => p,
        _ => match positional(&args) {
            Some(p) => p,
            None => {
                eprintln!("sb_radar_compiler needs a positional query or --params <k=v&k=v>");
                std::process::exit(2);
            }
        },
    };
    let url = format!("{BASE}?{params}");
    let Some(body) = omegaflow::archivar::fetch_raw(&url, None, &[]) else {
        eprintln!("{url} fetch void — no row is printed (0 honored)");
        std::process::exit(1);
    };
    let Some(JsonVal::Obj(root)) = parse_json(&body) else {
        eprintln!("{url} carries no JSON object — no row is printed");
        std::process::exit(1);
    };
    let fields: Vec<String> = match root.get("fields") {
        Some(JsonVal::Arr(arr)) => arr
            .iter()
            .filter_map(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => {
            eprintln!("{url} carries no fields array — no row is printed");
            std::process::exit(1);
        }
    };
    if inspect {
        for field in &fields {
            println!("{field}");
        }
        std::process::exit(0);
    }
    let data = match root.get("data") {
        Some(JsonVal::Arr(arr)) => arr,
        _ => {
            eprintln!("{url} carries no data array — no row is printed");
            std::process::exit(1);
        }
    };
    if emit {
        let out = match arg_value(&args, "--emit").as_deref() {
            None | Some("delay") => format!("data/{NETLOC}/{ASSET}"),
            Some(path) => path.to_string(),
        };
        if let Err(msg) = emit_delay(data, &fields, &out) {
            eprintln!("sb_radar_compiler: {msg}");
            std::process::exit(1);
        }
        let digest = match std::fs::read(&out) {
            Ok(bytes) => sha256_hex(&bytes),
            Err(e) => {
                eprintln!(
                    "sb_radar_compiler: {out} unread after emit ({e}) — no sha256, no register line"
                );
                std::process::exit(1);
            }
        };
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{ASSET}");
        println!("origin {url}");
        println!("compiler {COMPILER}");
        println!("format {FORMAT}");
        println!("sha256 {digest}");
        std::process::exit(0);
    }
    let mut rows = 0usize;
    for row in data {
        let JsonVal::Arr(cells) = row else {
            continue;
        };
        let mut parts: Vec<String> = Vec::new();
        for (i, field) in fields.iter().enumerate() {
            if let Some(cell) = cells.get(i).and_then(scalar_text) {
                parts.push(format!("{field}={cell}"));
            }
        }
        println!("{}", parts.join(" "));
        rows += 1;
    }
    println!("sb_radar_compiler: {rows} rows, {} fields", fields.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_row(body: &str) -> (Vec<String>, Vec<JsonVal>) {
        let JsonVal::Obj(root) = parse_json(body).expect("the fixture parses") else {
            panic!("the fixture carries a JSON object");
        };
        let fields = match root.get("fields") {
            Some(JsonVal::Arr(arr)) => arr
                .iter()
                .filter_map(|v| match v {
                    JsonVal::Str(s) => Some(s.clone()),
                    _ => None,
                })
                .collect(),
            _ => panic!("the fixture carries the fields array"),
        };
        let data = match root.get("data") {
            Some(JsonVal::Arr(arr)) => arr,
            _ => panic!("the fixture carries the data array"),
        };
        let JsonVal::Arr(cells) = &data[0] else {
            panic!("the fixture row is an array");
        };
        (fields, cells.clone())
    }

    #[test]
    fn a_delay_row_emits_one_wire_record_or_none() {
        let lsk = embedded_lsk().expect("the embedded lsk reads");
        let delay = r#"{"fields":["des","epoch","value","sigma","units","freq","rcvr","xmit","bp"],"data":[["6489","2003-05-27 09:18:00","103183685.","0.300","us","2380","-1","-1","C"]]}"#;
        let (fields, cells) = first_row(delay);
        let rec = record_bytes(&cells, &fields, &lsk).expect("the delay row emits");
        assert_eq!(rec.len(), REC_BYTES);
        assert_eq!(REC_BYTES, 208);
        let val = f64::from_le_bytes(rec[SLOT_VAL * 8..SLOT_VAL * 8 + 8].try_into().unwrap());
        assert!((val - 103.183685).abs() < 1e-9);
        let presence = f64::from_le_bytes(
            rec[SLOT_PRESENCE * 8..SLOT_PRESENCE * 8 + 8]
                .try_into()
                .unwrap(),
        );
        assert_eq!(presence, 1.0);

        let absent =
            r#"{"fields":["epoch","value","units"],"data":[["2003-05-27 09:18:00","","us"]]}"#;
        let (fields, cells) = first_row(absent);
        assert!(record_bytes(&cells, &fields, &lsk).is_none());

        let nonpositive =
            r#"{"fields":["epoch","value","units"],"data":[["2003-05-27 09:18:00","-3.0","us"]]}"#;
        let (fields, cells) = first_row(nonpositive);
        assert!(record_bytes(&cells, &fields, &lsk).is_none());
    }
}
