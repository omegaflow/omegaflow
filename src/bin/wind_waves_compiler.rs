use omegaflow::cdf::{value_present, CdfFile};
use omegaflow::cdn::upload_asset;
use omegaflow::lsk::parse as parse_lsk;
use omegaflow::wind::{
    parse_bin, receiver_name, write_bin, RECEIVER_RAD1, RECEIVER_RAD2, RECEIVER_TNR,
};
use std::process::Command;

const BASE_URL: &str = "https://spdf.gsfc.nasa.gov/pub/data/wind/waves/wav_h1/";

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
    rows: usize,
    bins: usize,
    emitted: usize,
    note: Option<String>,
}

fn harvest_day(
    year: i64,
    month: i64,
    day: i64,
    lsk: &omegaflow::lsk::LeapSeconds,
    records: &mut Vec<(f64, f64, f64, f64, u32)>,
) -> DayOutcome {
    let date = format!("{year:04}{month:02}{day:02}");
    let url = format!("{BASE_URL}{year}/wi_h1_wav_{date}_v01.cdf");
    let Some(bytes) = fetch(&url) else {
        return DayOutcome {
            date,
            rows: 0,
            bins: 0,
            emitted: 0,
            note: Some("fetch void".to_string()),
        };
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            return DayOutcome {
                date,
                rows: 0,
                bins: 0,
                emitted: 0,
                note: Some(format!("{note:?}")),
            };
        }
    };
    let Some(epoch) = file.var("Epoch") else {
        return DayOutcome {
            date,
            rows: 0,
            bins: 0,
            emitted: 0,
            note: Some("Epoch absent".to_string()),
        };
    };
    let epoch_records = match file.var_records(&bytes, epoch) {
        Ok(r) => r,
        Err(note) => {
            return DayOutcome {
                date,
                rows: 0,
                bins: 0,
                emitted: 0,
                note: Some(format!("Epoch {note:?}")),
            };
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
        return DayOutcome {
            date,
            rows: 0,
            bins: 0,
            emitted: 0,
            note: Some("TDB void".to_string()),
        };
    };
    let mut emitted = 0usize;
    let mut reported_bins = 0usize;
    for r in &RECEIVERS {
        let Some(volt) = file.var(r.volt_name) else {
            continue;
        };
        let Some(freq) = file.var(r.freq_name) else {
            continue;
        };
        let bins = file.num_values(volt);
        reported_bins = reported_bins.max(bins);
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
            records.push((t_tdb, freq_hz, binw, val, r.receiver));
            emitted += 1;
        }
    }
    DayOutcome {
        date,
        rows: epoch_records.len(),
        bins: reported_bins,
        emitted,
        note: None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = arg_value(&args, "--probe") {
        probe(&path);
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "wind_waves.bin".to_string());
    let year: i64 = match arg_value(&args, "--year").and_then(|v| v.parse().ok()) {
        Some(y) => y,
        None => {
            eprintln!("--year absent — the harvest window stays undeclared");
            std::process::exit(1);
        }
    };
    let month: i64 = match arg_value(&args, "--month").and_then(|v| v.parse().ok()) {
        Some(m) => m,
        None => {
            eprintln!("--month absent — the harvest window stays undeclared");
            std::process::exit(1);
        }
    };
    if !(1..=12).contains(&month) {
        eprintln!("--month {month} is not a calendar month");
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
    let days_in_month = match month {
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    let mut records: Vec<(f64, f64, f64, f64, u32)> = Vec::new();
    let mut outcomes: Vec<DayOutcome> = Vec::new();
    for day in 1..=days_in_month {
        let o = harvest_day(year, month, day, &lsk, &mut records);
        eprintln!(
            "{} rows {} bins {} emitted {} note {}",
            o.date,
            o.rows,
            o.bins,
            o.emitted,
            o.note.as_deref().unwrap_or("none")
        );
        outcomes.push(o);
    }
    let void = outcomes.iter().filter(|o| o.note.is_some()).count();
    let emitted_total: usize = outcomes.iter().map(|o| o.emitted).sum();
    eprintln!(
        "{year:04}-{month:02}: {} days, {} void, {} records emitted",
        days_in_month, void, emitted_total
    );
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
        eprintln!("{year:04}-{month:02}: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
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
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
