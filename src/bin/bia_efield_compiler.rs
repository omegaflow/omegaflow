use omegaflow::cdf::{value_present, CdfFile};
use omegaflow::cdn::upload_asset;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use omegaflow::rpw::{parse_bin, write_bin, COMP_EY, COMP_EZ};
use std::collections::HashMap;
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const DB_URL: &str = "https://rpw-lira.obspm.fr/roc/data/pub/solo/rpw/data/L3/db.csv";
const BASE_URL: &str = "https://rpw-lira.obspm.fr/roc/data/pub/solo/rpw/data/L3/";
const PREFIX: &str = "solo_L3_rpw-bia-efield-10-seconds";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
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
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn date_of(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y}-{m:02}-{d:02}")
}

fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
}

fn median(vals: &mut Vec<f64>) -> f64 {
    vals.sort_by(|a, b| a.total_cmp(b));
    let n = vals.len();
    if n % 2 == 0 {
        (vals[n / 2 - 1] + vals[n / 2]) * 0.5
    } else {
        vals[n / 2]
    }
}

fn type_name(t: u32) -> &'static str {
    match t {
        1 => "CDF_INT1",
        2 => "CDF_INT2",
        4 => "CDF_INT4",
        8 => "CDF_INT8",
        11 => "CDF_UINT1",
        12 => "CDF_UINT2",
        14 => "CDF_UINT4",
        21 => "CDF_REAL4",
        22 => "CDF_REAL8",
        31 => "CDF_EPOCH",
        32 => "CDF_EPOCH16",
        33 => "CDF_TIME_TT2000",
        41 => "CDF_BYTE",
        44 => "CDF_FLOAT",
        45 => "CDF_DOUBLE",
        51 => "CDF_CHAR",
        52 => "CDF_UCHAR",
        _ => "unknown",
    }
}

fn ctype_name(t: Option<u32>) -> &'static str {
    match t {
        None => "none",
        Some(0) => "none",
        Some(1) => "rle",
        Some(2) => "huffman",
        Some(3) => "adaptive-huffman",
        Some(5) => "gzip",
        Some(_) => "unknown",
    }
}

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    match CdfFile::parse(&bytes) {
        Ok(file) => {
            eprintln!(
                "{}: CDF {}.{}.{} encoding {} majority {} {} zVariables {} attributes eof {}",
                path,
                file.version.0,
                file.version.1,
                file.version.2,
                file.encoding,
                file.majority,
                file.vars.len(),
                file.num_att,
                file.eof,
            );
            for var in &file.vars {
                eprintln!(
                    "  var {} {} {} x{} dims {:?} vary {:?} record_vary {} pad {} sparse {} compression {} max_rec {}",
                    var.var_num,
                    var.name,
                    type_name(var.data_type),
                    var.num_elements,
                    var.dim_sizes,
                    var.dim_vary,
                    var.record_vary,
                    var.pad,
                    var.sparse,
                    ctype_name(var.compression_type),
                    var.max_rec,
                );
                match file.blocks(&bytes, var) {
                    Ok(blocks) => {
                        let kinds: Vec<u32> = blocks.iter().map(|b| b.kind).collect();
                        let mut distinct = kinds.clone();
                        distinct.sort_unstable();
                        distinct.dedup();
                        let total = blocks.iter().map(|b| b.end - b.start + 1).sum::<u32>();
                        let first = blocks.first().map(|b| (b.start, b.end, b.offset));
                        let last = blocks.last().map(|b| (b.start, b.end, b.offset));
                        eprintln!(
                            "    {} blocks kinds {:?} {} records {:?}..{:?}",
                            blocks.len(),
                            distinct,
                            total,
                            first,
                            last
                        );
                    }
                    Err(note) => eprintln!("    blocks: {:?}", note),
                }
                if var.name == "EDC_SRF" || var.name == "Epoch" || var.name == "QUALITY_FLAG" {
                    match file.var_records(&bytes, var) {
                        Ok(records) => {
                            let first = records.first().map(|(r, v)| (r, v.clone()));
                            let last = records.last().map(|(r, v)| (r, v.clone()));
                            eprintln!("    {} records {:?} .. {:?}", records.len(), first, last);
                            if var.name == "EDC_SRF" {
                                let cols = records.first().map(|(_, v)| v.len()).unwrap_or(0);
                                for col in 0..cols {
                                    let mut real = 0usize;
                                    let mut mn = f64::INFINITY;
                                    let mut mx = f64::NEG_INFINITY;
                                    for (_, vals) in &records {
                                        let v = vals[col];
                                        if value_present(v) {
                                            real += 1;
                                            mn = mn.min(v);
                                            mx = mx.max(v);
                                        }
                                    }
                                    eprintln!(
                                        "      col {} {} real values [{}, {}]",
                                        col, real, mn, mx
                                    );
                                }
                            }
                        }
                        Err(note) => eprintln!("    records: {:?}", note),
                    }
                }
            }
        }
        Err(note) => eprintln!("{path}: {:?}", note),
    }
}

fn day_index(db: &str) -> HashMap<i64, String> {
    let mut map: HashMap<i64, String> = HashMap::new();
    for line in db.lines().skip(1) {
        let Some(path) = line.split(',').next() else {
            continue;
        };
        if !path.contains(PREFIX) {
            continue;
        }
        let file = path.rsplit('/').next().unwrap_or("");
        let stem = file.strip_suffix(".cdf").unwrap_or(file);
        let mut parts = stem.rsplit('_');
        let ver = parts.next().unwrap_or("");
        let date = parts.next().unwrap_or("");
        let Some(n) = ver.strip_prefix('V').and_then(|v| v.parse::<u32>().ok()) else {
            continue;
        };
        let y = date.get(0..4).and_then(|s| s.parse::<i64>().ok());
        let m = date.get(4..6).and_then(|s| s.parse::<i64>().ok());
        let d = date.get(6..8).and_then(|s| s.parse::<i64>().ok());
        let Some(day) = y
            .zip(m)
            .zip(d)
            .and_then(|((y, m), d)| days_from_civil(y, m, d))
        else {
            continue;
        };
        map.entry(day)
            .and_modify(|existing: &mut String| {
                let cur = existing
                    .rsplit('_')
                    .next()
                    .and_then(|v| v.strip_prefix('V'))
                    .and_then(|v| v.parse::<u32>().ok())
                    .unwrap_or(0);
                if n > cur {
                    *existing = path.to_string();
                }
            })
            .or_insert_with(|| path.to_string());
    }
    map
}

struct DayOutcome {
    day: i64,
    rows: usize,
    fills: usize,
    ex_real: usize,
    missing_epochs: usize,
    note: Option<String>,
}

fn harvest_day(
    day: i64,
    path: &str,
    decimate_s: f64,
    lsk: &omegaflow::lsk::LeapSeconds,
    records: &Mutex<Vec<(f64, f64, u32)>>,
    outcomes: &Mutex<Vec<DayOutcome>>,
) {
    let url = format!("{BASE_URL}{path}");
    let Some(bytes) = fetch(&url) else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                day,
                rows: 0,
                fills: 0,
                ex_real: 0,
                missing_epochs: 0,
                note: Some("fetch void".to_string()),
            });
        return;
    };
    let mut outcome = DayOutcome {
        day,
        rows: 0,
        fills: 0,
        ex_real: 0,
        missing_epochs: 0,
        note: None,
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            outcome.note = Some(format!("{:?}", note));
            outcomes
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(outcome);
            return;
        }
    };
    let Some(epoch_var) = file.var("Epoch") else {
        outcome.note = Some("Epoch absent".to_string());
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(outcome);
        return;
    };
    let Some(edc) = file.var("EDC_SRF") else {
        outcome.note = Some("EDC_SRF absent".to_string());
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(outcome);
        return;
    };
    let epoch_map = match file.epoch_map(&bytes, epoch_var) {
        Ok(m) => m,
        Err(note) => {
            outcome.note = Some(format!("Epoch {:?}", note));
            outcomes
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(outcome);
            return;
        }
    };
    let edc_records = match file.var_records(&bytes, edc) {
        Ok(r) => r,
        Err(note) => {
            outcome.note = Some(format!("EDC_SRF {:?}", note));
            outcomes
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(outcome);
            return;
        }
    };
    let mut buckets: HashMap<u64, (Vec<f64>, Vec<f64>)> = HashMap::new();
    for (rec_num, vals) in edc_records {
        let Some(t_unix) = epoch_map.get(&rec_num).copied() else {
            outcome.missing_epochs += 1;
            continue;
        };
        outcome.rows += 1;
        let bucket = (t_unix / decimate_s).floor() as u64;
        let entry = buckets.entry(bucket).or_default();
        let mut taken = 0usize;
        if let Some(ex) = vals.get(0) {
            if value_present(*ex) {
                outcome.ex_real += 1;
            }
        }
        if let Some(ey) = vals.get(1) {
            if value_present(*ey) {
                entry.0.push(*ey);
                taken += 1;
            }
        }
        if let Some(ez) = vals.get(2) {
            if value_present(*ez) {
                entry.1.push(*ez);
                taken += 1;
            }
        }
        if taken == 0 {
            outcome.fills += 1;
        }
    }
    let mut day_records: Vec<(f64, f64, u32)> = Vec::new();
    for (bucket, (ey, ez)) in &mut buckets {
        let t = *bucket as f64 * decimate_s + decimate_s * 0.5;
        if !ey.is_empty() {
            if let Some(tdb) = lsk.unix_to_tdb(t) {
                day_records.push((tdb, median(ey), COMP_EY));
            }
        }
        if !ez.is_empty() {
            if let Some(tdb) = lsk.unix_to_tdb(t) {
                day_records.push((tdb, median(ez), COMP_EZ));
            }
        }
    }
    day_records.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut guard = records.lock().unwrap_or_else(|e| e.into_inner());
    guard.extend(day_records);
    drop(guard);
    outcomes
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(outcome);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--probe") {
        probe(&path);
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "rpw_efield.bin".to_string());
    let decimate_min: f64 = arg_value(&args, "--decimate-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10.0);
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => match parse_days("2022-12-01") {
            Some(d) => d,
            None => {
                eprintln!("--window-start undeclared and the default start parses void");
                std::process::exit(1);
            }
        },
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => match parse_days("2025-12-31") {
            Some(d) => d,
            None => {
                eprintln!("--window-end undeclared and the default end parses void");
                std::process::exit(1);
            }
        },
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let db_text = match arg_value(&args, "--db")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(String::into_bytes)
        .or_else(|| fetch(DB_URL))
        .and_then(|b| String::from_utf8(b).ok())
    {
        Some(t) => t,
        None => {
            eprintln!(
                "db.csv: no local file and the fetch stays void — the day index stays unbuilt"
            );
            std::process::exit(1);
        }
    };
    let index = day_index(&db_text);
    eprintln!("db.csv: {} indexed days for {}", index.len(), PREFIX);
    let mut days: Vec<i64> = (start_day..=end_day)
        .filter(|d| index.contains_key(d))
        .collect();
    days.sort_unstable();
    let missing = (end_day - start_day + 1) as usize - days.len();
    eprintln!(
        "window {}..{}: {} files indexed, {} days absent from the tree",
        date_of(start_day),
        date_of(end_day),
        days.len(),
        missing
    );
    let records: Arc<Mutex<Vec<(f64, f64, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let outcomes: Arc<Mutex<Vec<DayOutcome>>> = Arc::new(Mutex::new(Vec::new()));
    let next = Arc::new(AtomicI64::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    let total = days.len();
    let lsk_ref = &lsk;
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let records = Arc::clone(&records);
            let outcomes = Arc::clone(&outcomes);
            let next = Arc::clone(&next);
            let done = Arc::clone(&done);
            let days = &days;
            let index = &index;
            s.spawn(move || loop {
                let i = next.fetch_add(1, Ordering::SeqCst) as usize;
                if i >= days.len() {
                    break;
                }
                let day = days[i];
                let path = index[&day].clone();
                harvest_day(
                    day,
                    &path,
                    decimate_min * 60.0,
                    lsk_ref,
                    &records,
                    &outcomes,
                );
                let n = done.fetch_add(1, Ordering::SeqCst) + 1;
                if n % 40 == 0 || n == total {
                    eprintln!("{n}/{total} days harvested");
                }
            });
        }
    });
    let outcomes_guard = Arc::try_unwrap(outcomes)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut void_days = 0usize;
    let mut ex_real_days = 0usize;
    let mut missing_epochs_total = 0usize;
    for o in &outcomes_guard {
        match &o.note {
            Some(note) => {
                void_days += 1;
                eprintln!("{}: {}", date_of(o.day), note);
            }
            None => {
                if o.ex_real > 0 {
                    ex_real_days += 1;
                }
                missing_epochs_total += o.missing_epochs;
            }
        }
    }
    eprintln!(
        "{} days void, {} days with real ex values, {} records without epoch",
        void_days, ex_real_days, missing_epochs_total
    );
    let records_guard = Arc::try_unwrap(records)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut raw = records_guard;
    raw.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut harvested = raw.len();
    let main_len = harvested;
    if args.iter().any(|a| a == "--merge") {
        match std::fs::read(&out) {
            Ok(existing) => match parse_bin(&existing) {
                Some(mut old) => {
                    let old_len = old.len();
                    raw.append(&mut old);
                    raw.sort_by(|a, b| a.0.total_cmp(&b.0));
                    raw.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
                    eprintln!(
                        "{}: {} harvested + {} existing = {} merged ({} dups dropped)",
                        out,
                        main_len,
                        old_len,
                        raw.len(),
                        main_len + old_len - raw.len()
                    );
                    harvested = raw.len();
                }
                None => {
                    eprintln!(
                        "{}: existing bin parses void — the bin stays unwritten",
                        out
                    );
                    std::process::exit(1);
                }
            },
            Err(_) => {
                eprintln!("{}: no existing bin — the harvest stands alone", out);
            }
        }
    }
    eprintln!(
        "{}: {} records, {} medians/min buckets",
        PREFIX, harvested, decimate_min
    );
    if raw.is_empty() {
        eprintln!(
            "{}: no records — the bin stays unwritten (0 honored)",
            PREFIX
        );
        std::process::exit(1);
    }
    let bytes = write_bin(&raw);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{}: {} records, roundtrip parses", out, parsed.len());
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
