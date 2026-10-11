use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::oe417::{
    COMP_CUSTOMERS_AFFECTED, COMP_DEMAND_LOSS_MW, FRAME_HEADER, MAGIC, SLOTS, parse_series,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{PRESENCE_FLAG_QUANTITY, embedded_lsk};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::days_from_civil;
use std::path::Path;

const URL: &str = "https://openenergyhub.ornl.gov/api/records/1.0/download/?dataset=oe-417-annual-summaries&format=csv";
const NETLOC: &str = "openenergyhub.ornl.gov";
const OUT_PATH: &str = "oe417.bin";
const COMPILER: &str = "tools/harvest/src/bin/oe417_compiler.rs";
const FORMAT: &str = "oe417";
const TTL: f64 = 604800.0;
const SAMPLE_TAU_S: f64 = 86400.0;
const REC_BYTES: usize = 26 * 8;

const SLOT_VAL: usize = 3;
const SLOT_EPOCH: usize = 4;
const SLOT_TTL: usize = 5;
const SLOT_CHANNEL: usize = 9;
const SLOT_PRESENCE: usize = 25;

const COL_DATE_BEGAN: usize = 1;
const COL_DEMAND_LOSS: usize = 9;
const COL_CUSTOMERS: usize = 10;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn split_row(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in line.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ';' if !in_quotes => out.push(std::mem::take(&mut cur)),
            _ => cur.push(ch),
        }
    }
    out.push(cur);
    out
}

fn parse_date(cell: &str) -> Option<f64> {
    let mut d = cell.trim().split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    if d.next().is_some() {
        return None;
    }
    Some((days_from_civil(y, mo, da)? * 86400) as f64)
}

fn num(cell: &str) -> Option<f64> {
    let t = cell.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

struct EventRow {
    epoch_unix: f64,
    demand_loss_mw: Option<f64>,
    customers_affected: Option<f64>,
}

fn parse_events(text: &str) -> Vec<EventRow> {
    let mut out = Vec::new();
    for line in text.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let cells = split_row(line);
        if cells.len() <= COL_CUSTOMERS {
            continue;
        }
        let Some(epoch_unix) = parse_date(&cells[COL_DATE_BEGAN]) else {
            continue;
        };
        let demand_loss_mw = num(&cells[COL_DEMAND_LOSS]);
        let customers_affected = num(&cells[COL_CUSTOMERS]);
        if demand_loss_mw.is_none() && customers_affected.is_none() {
            continue;
        }
        out.push(EventRow {
            epoch_unix,
            demand_loss_mw,
            customers_affected,
        });
    }
    out
}

fn record(epoch_tdb: f64, channel: f64, val: f64) -> [f64; SLOTS] {
    let mut r = [0.0f64; SLOTS];
    r[SLOT_VAL] = val;
    r[SLOT_EPOCH] = epoch_tdb;
    r[SLOT_TTL] = TTL;
    r[SLOT_CHANNEL] = channel;
    r[SLOT_PRESENCE] = PRESENCE_FLAG_QUANTITY;
    r
}

fn frame(records: &[[f64; SLOTS]]) -> Option<Vec<u8>> {
    if records.is_empty() {
        return None;
    }
    let response_epoch = records
        .iter()
        .map(|r| r[SLOT_EPOCH])
        .fold(f64::NEG_INFINITY, f64::max);
    let mut out = Vec::with_capacity(FRAME_HEADER + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&response_epoch.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or("the embedded naif0012 table stays unread")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => OUT_PATH.to_string(),
    };
    let input = arg_value(args, "--in").or_else(|| arg_value(args, "--input"));
    let (text, origin) = match input {
        Some(path) => {
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("read {path} returned void: {e}"))?;
            (text, path)
        }
        None => {
            let url = match arg_value(args, "--url") {
                Some(v) => v,
                None => URL.to_string(),
            };
            let text = fetch_raw(&url, None, &[]).ok_or_else(|| format!("{url}: fetch void"))?;
            (text, url)
        }
    };

    let events = parse_events(&text);
    if events.is_empty() {
        return Err(format!(
            "{origin}: no OE-417 event row parses — the bin stays unwritten (0 honored)"
        ));
    }

    let mut records: Vec<[f64; SLOTS]> = Vec::new();
    let mut absent = 0usize;
    for e in &events {
        let Some(epoch_tdb) = lsk.unix_to_tdb(e.epoch_unix) else {
            continue;
        };
        for (channel, value) in [
            (COMP_DEMAND_LOSS_MW as f64, e.demand_loss_mw),
            (COMP_CUSTOMERS_AFFECTED as f64, e.customers_affected),
        ] {
            match value {
                Some(v) => records.push(record(epoch_tdb, channel, v)),
                None => absent += 1,
            }
        }
    }
    records.sort_by(|a, b| a[SLOT_EPOCH].total_cmp(&b[SLOT_EPOCH]));

    let bytes = frame(&records).ok_or("framing returned void")?;
    if let Some(parent) = Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|e| format!("create parent void: {e}"))?;
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;

    let parsed = parse_series(&bytes).ok_or("roundtrip parse returned void")?;
    if parsed.len() != records.len() {
        return Err(format!(
            "{out}: {} records read back vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        ));
    }

    let sha = sha256_hex(&bytes);
    let demand = records
        .iter()
        .filter(|r| r[SLOT_CHANNEL] == COMP_DEMAND_LOSS_MW as f64)
        .count();
    let customers = records
        .iter()
        .filter(|r| r[SLOT_CHANNEL] == COMP_CUSTOMERS_AFFECTED as f64)
        .count();
    eprintln!(
        "{out}: {} events → {} records ({} demand_loss_mw, {} customers_affected, {} absent cells skipped), {} bytes, sha256 {}",
        events.len(),
        records.len(),
        demand,
        customers,
        absent,
        bytes.len(),
        sha
    );
    eprintln!(
        "  position: pending — OE-417 carries only free-text `area_affected` (no coordinate); \
         records are positionless event quantities at `earth`, never a fabricated 0.0 lat/lon"
    );
    eprintln!(
        "  oe417_demand_loss_mw: no power QuantityKind exists in the register — declared as \
         source-parameter MW (the unit gate is skipped for source-parameter); a true power/demand \
         quantity kind is a named pending architecture block"
    );

    let out_name = match Path::new(&out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.clone(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("sha256 {sha}");
    println!("format {FORMAT}");
    println!("ttl {}", TTL as u64);
    println!("at earth");
    println!(
        "quantity oe417_demand_loss_mw oe417_demand_loss_mw inverse-square source-parameter MW {} 0.0 0.0",
        SAMPLE_TAU_S as u64
    );
    println!(
        "quantity oe417_customers_affected oe417_customers_affected inverse-square relative count {} 0.0 0.0",
        SAMPLE_TAU_S as u64
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("oe417_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBE: &str = "event_month;date_event_began;time_event_began;date_of_restoration;time_of_restoration;area_affected;nerc_region;alert_criteria;event_type;demand_loss_mw;number_of_customers_affected\n\
January;2023-01-23;;2023-01-24;;Connecticut: Rhode Island:;NPCC;Loss of electric service;Severe Weather;;41000\n\
January;2023-01-25;;2023-01-26;;\"Washington: Kitsap County;\";WECC;\"Physical attack; or vandalism\";Vandalism;0.0;0\n";

    #[test]
    fn quotes_hide_semicolons_in_the_alert_cell() {
        let cells = split_row(
            "January;2023-01-25;;\"Washington: Kitsap County;\";WECC;\"a; b\";Vandalism;0.0;0",
        );
        assert_eq!(cells.len(), 9);
        assert_eq!(cells[3], "Washington: Kitsap County;");
        assert_eq!(cells[5], "a; b");
    }

    #[test]
    fn parse_events_reads_both_numeric_fields_and_skips_absent() {
        let events = parse_events(PROBE);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].demand_loss_mw, None);
        assert_eq!(events[0].customers_affected, Some(41000.0));
        assert_eq!(events[1].demand_loss_mw, Some(0.0));
        assert_eq!(events[1].customers_affected, Some(0.0));
        let days = days_from_civil(2023, 1, 23).unwrap();
        assert_eq!(events[0].epoch_unix, (days * 86400) as f64);
    }

    #[test]
    fn a_measured_zero_is_kept_never_dropped() {
        let events = parse_events(PROBE);
        assert!(events[1].demand_loss_mw.is_some());
        assert_eq!(events[1].demand_loss_mw, Some(0.0));
    }

    #[test]
    fn the_frame_carries_the_wire_magic_and_roundtrips() {
        let records = vec![
            record(1.0, COMP_DEMAND_LOSS_MW as f64, 50.0),
            record(1.0, COMP_CUSTOMERS_AFFECTED as f64, 41000.0),
        ];
        let bytes = frame(&records).expect("non-empty frames");
        assert_eq!(&bytes[0..3], MAGIC.as_slice());
        assert_eq!(bytes.len(), FRAME_HEADER + 2 * REC_BYTES);
        assert!(parse_series(&bytes).is_some());
    }

    #[test]
    fn a_foreign_or_truncated_frame_is_void() {
        assert!(parse_series(b"XXXX").is_none());
        let good = frame(&[record(1.0, 1.0, 1.0)]).expect("frames");
        assert!(parse_series(&good[..good.len() - 1]).is_none());
    }
}
