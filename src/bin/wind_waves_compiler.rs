use omegaflow::cdf::{value_present, CdfFile};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk};
use omegaflow::wind::{
    parse_bin, receiver_name, write_bin, RECEIVER_RAD1, RECEIVER_RAD2, RECEIVER_TNR,
};
use std::process::Command;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

const BASE_URL: &str = "https://spdf.gsfc.nasa.gov/pub/data/wind/waves/wav_h1/";
const CDN_RELEASE: &str = "spdf.gsfc.nasa.gov";

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

fn parse_days(s: &str) -> Option<i64> {
    let (y, rest) = s.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    days_from_civil(y.parse().ok()?, m.parse().ok()?, d.parse().ok()?)
}

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

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {:?}", note);
            std::process::exit(1);
        }
    };
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
            "  var {} {} {} x{} dims {:?} vary {:?} record_vary {} max_rec {}",
            var.var_num,
            var.name,
            type_name(var.data_type),
            var.num_elements,
            var.dim_sizes,
            var.dim_vary,
            var.record_vary,
            var.max_rec,
        );
    }
    if let Some(epoch) = file.var("Epoch") {
        match file.var_records(&bytes, epoch) {
            Ok(records) => {
                let first = records
                    .first()
                    .map(|(_, v)| v.first().copied().unwrap_or(f64::NAN));
                let last = records
                    .last()
                    .map(|(_, v)| v.first().copied().unwrap_or(f64::NAN));
                eprintln!(
                    "  Epoch {} records first {:?} last {:?} (unix seconds)",
                    records.len(),
                    first,
                    last
                );
            }
            Err(note) => eprintln!("  Epoch: {:?}", note),
        }
    }
    for freq_name in ["Frequency_RAD1", "Frequency_RAD2", "Frequency_TNR"] {
        let Some(var) = file.var(freq_name) else {
            continue;
        };
        match file.var_records(&bytes, var) {
            Ok(records) => {
                let Some((_, vals)) = records.first() else {
                    continue;
                };
                let lo = vals.iter().copied().fold(f64::INFINITY, f64::min);
                let hi = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let n = vals.len();
                eprintln!(
                    "  {freq_name} {} bins [{}, {}] first {:?} last {:?}",
                    n,
                    lo,
                    hi,
                    vals.first().copied(),
                    vals.last().copied()
                );
            }
            Err(note) => eprintln!("  {freq_name}: {:?}", note),
        }
    }
    for vname in ["E_VOLTAGE_RAD1", "E_VOLTAGE_RAD2", "E_VOLTAGE_TNR"] {
        let Some(var) = file.var(vname) else {
            continue;
        };
        match file.var_records(&bytes, var) {
            Ok(records) => {
                let bins = file.num_values(var);
                let mut real = 0usize;
                let mut beyond = 0usize;
                let mut zero = 0usize;
                let mut negative = 0usize;
                let mut mn = f64::INFINITY;
                let mut mx = f64::NEG_INFINITY;
                for (_, vals) in &records {
                    for v in vals {
                        if value_present(*v) {
                            real += 1;
                            mn = mn.min(*v);
                            mx = mx.max(*v);
                            if *v == 0.0 {
                                zero += 1;
                            }
                            if *v < 0.0 {
                                negative += 1;
                            }
                        } else {
                            beyond += 1;
                        }
                    }
                }
                eprintln!(
                    "  {vname} {} records x {} bins: {} real [{:e}, {:e}] {} beyond-fill {} zero {} negative",
                    records.len(),
                    bins,
                    real,
                    mn,
                    mx,
                    beyond,
                    zero,
                    negative
                );
                if let Some((_, first)) = records.first() {
                    eprintln!("    first record head: {:?}", &first[..first.len().min(8)]);
                }
            }
            Err(note) => eprintln!("  {vname}: {:?}", note),
        }
    }
}

struct Receiver {
    volt_name: &'static str,
    freq_name: &'static str,
    receiver: u32,
}

const RECEIVERS: [Receiver; 3] = [
    Receiver {
        volt_name: "E_VOLTAGE_RAD1",
        freq_name: "Frequency_RAD1",
        receiver: RECEIVER_RAD1,
    },
    Receiver {
        volt_name: "E_VOLTAGE_RAD2",
        freq_name: "Frequency_RAD2",
        receiver: RECEIVER_RAD2,
    },
    Receiver {
        volt_name: "E_VOLTAGE_TNR",
        freq_name: "Frequency_TNR",
        receiver: RECEIVER_TNR,
    },
];

struct DayOutcome {
    date: String,
    emitted: usize,
    note: Option<String>,
}

fn harvest_day(
    day: i64,
    lsk: &omegaflow::lsk::LeapSeconds,
    records: &Mutex<Vec<(f64, f64, f64, f64, u32)>>,
    outcomes: &Mutex<Vec<DayOutcome>>,
) {
    let (year, month, mday) = civil_from_days(day);
    let date = format!("{year:04}{month:02}{mday:02}");
    let url = format!("{BASE_URL}{year}/wi_h1_wav_{date}_v01.cdf");
    let Some(bytes) = fetch(&url) else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date,
                emitted: 0,
                note: Some("fetch void".to_string()),
            });
        return;
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            outcomes
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(DayOutcome {
                    date,
                    emitted: 0,
                    note: Some(format!("{note:?}")),
                });
            return;
        }
    };
    let Some(epoch) = file.var("Epoch") else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date,
                emitted: 0,
                note: Some("Epoch absent".to_string()),
            });
        return;
    };
    let epoch_records = match file.var_records(&bytes, epoch) {
        Ok(r) => r,
        Err(note) => {
            outcomes
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(DayOutcome {
                    date,
                    emitted: 0,
                    note: Some(format!("Epoch {note:?}")),
                });
            return;
        }
    };
    let mut t_sum = 0.0f64;
    let mut t_n = 0usize;
    for (_, v) in &epoch_records {
        if let Some(t) = v.first().copied() {
            if t.is_finite() {
                t_sum += t;
                t_n += 1;
            }
        }
    }
    let t_noon = if t_n > 0 {
        t_sum / t_n as f64
    } else {
        f64::NAN
    };
    let Some(t_tdb) = lsk.unix_to_tdb(t_noon) else {
        outcomes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(DayOutcome {
                date,
                emitted: 0,
                note: Some("TDB void".to_string()),
            });
        return;
    };
    let mut day_records: Vec<(f64, f64, f64, f64, u32)> = Vec::with_capacity(608);
    let mut emitted = 0usize;
    for r in &RECEIVERS {
        let Some(volt) = file.var(r.volt_name) else {
            continue;
        };
        let Some(freq) = file.var(r.freq_name) else {
            continue;
        };
        let bins = file.num_values(volt);
        let Ok(freq_records) = file.var_records(&bytes, freq) else {
            continue;
        };
        let Some((_, freq_vals)) = freq_records.first() else {
            continue;
        };
        let Ok(volt_records) = file.var_records(&bytes, volt) else {
            continue;
        };
        let mut per_bin: Vec<Vec<f64>> = vec![Vec::new(); bins];
        for (_, vals) in &volt_records {
            for (i, v) in vals.iter().enumerate() {
                if i < bins && value_present(*v) {
                    per_bin[i].push(*v);
                }
            }
        }
        for i in 0..bins {
            if per_bin[i].is_empty() {
                continue;
            }
            let freq_hz = freq_vals.get(i).copied().unwrap_or(0.0) * 1000.0;
            let lo = if i > 0 {
                freq_vals.get(i - 1).copied().unwrap_or(0.0) * 1000.0
            } else {
                freq_hz - (freq_vals.get(1).copied().unwrap_or(0.0) * 1000.0 - freq_hz)
            };
            let hi = if i + 1 < bins {
                freq_vals.get(i + 1).copied().unwrap_or(0.0) * 1000.0
            } else {
                freq_hz + (freq_hz - freq_vals.get(bins - 2).copied().unwrap_or(0.0) * 1000.0)
            };
            let binw = ((hi - freq_hz) + (freq_hz - lo)) * 0.5;
            let val = median(&mut per_bin[i]);
            day_records.push((t_tdb, freq_hz, binw, val, r.receiver));
            emitted += 1;
        }
    }
    records
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .append(&mut day_records);
    outcomes
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(DayOutcome {
            date,
            emitted,
            note: None,
        });
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--probe") {
        probe(&path);
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "wind_waves.bin".to_string());
    let jobs: usize = arg_value(&args, "--jobs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(4);
    let start_day = match arg_value(&args, "--window-start")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => {
            eprintln!("--window-start absent — the harvest window stays undeclared");
            std::process::exit(1);
        }
    };
    let end_day = match arg_value(&args, "--window-end")
        .as_deref()
        .and_then(parse_days)
    {
        Some(d) => d,
        None => {
            eprintln!("--window-end absent — the harvest window stays undeclared");
            std::process::exit(1);
        }
    };
    if start_day > end_day {
        eprintln!("--window-start lies after --window-end — the window stays unharvested");
        std::process::exit(1);
    }
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
    let days: Vec<i64> = (start_day..=end_day).collect();
    let total = days.len();
    eprintln!("window: {} days", total);
    let records: Arc<Mutex<Vec<(f64, f64, f64, f64, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let outcomes: Arc<Mutex<Vec<DayOutcome>>> = Arc::new(Mutex::new(Vec::new()));
    let next = Arc::new(AtomicI64::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    let lsk_ref = &lsk;
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let records = Arc::clone(&records);
            let outcomes = Arc::clone(&outcomes);
            let next = Arc::clone(&next);
            let done = Arc::clone(&done);
            let days = &days;
            s.spawn(move || loop {
                let i = next.fetch_add(1, Ordering::SeqCst) as usize;
                if i >= days.len() {
                    break;
                }
                harvest_day(days[i], lsk_ref, &records, &outcomes);
                let n = done.fetch_add(1, Ordering::SeqCst) + 1;
                if n % 40 == 0 || n == total {
                    eprintln!("{n}/{total} days harvested");
                }
            });
        }
    });
    let outcomes = Arc::try_unwrap(outcomes)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let mut void = 0usize;
    let mut emitted_total = 0usize;
    for o in &outcomes {
        emitted_total += o.emitted;
        if let Some(note) = &o.note {
            void += 1;
            eprintln!("{} void: {}", o.date, note);
        }
    }
    eprintln!(
        "{} days, {} void, {} records emitted",
        total, void, emitted_total
    );
    let records = Arc::try_unwrap(records)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    let per_receiver: std::collections::HashMap<u32, usize> =
        records
            .iter()
            .fold(std::collections::HashMap::new(), |mut m, r| {
                *m.entry(r.4).or_default() += 1;
                m
            });
    for (r, n) in &per_receiver {
        eprintln!("  {}: {} records", receiver_name(*r), n);
    }
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let mut records = records;
    records.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then(a.1.total_cmp(&b.1))
            .then(a.4.cmp(&b.4))
    });
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!("{out}: {} records, roundtrip parses", parsed.len());
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_RELEASE, &out) {
        std::process::exit(1);
    }
}
