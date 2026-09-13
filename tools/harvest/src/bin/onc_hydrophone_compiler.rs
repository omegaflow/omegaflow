use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_ONC_PSD, MAGIC_ONC};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;
use omegaflow::matfile::{parse_mat, MatArray, MatData, MatField};
use std::env;
use std::fs;

const NETLOC: &str = "data.oceannetworks.ca";

const MATLAB_1970_DAYS: f64 = 719529.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_f64(args: &[String], name: &str) -> Option<f64> {
    arg_value(args, name)
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| v.is_finite())
}

fn struct_fields(arrays: &[MatArray]) -> Option<&[MatField]> {
    for a in arrays {
        if let MatData::Struct(fields) = &a.data {
            return Some(fields);
        }
    }
    None
}

fn field<'a>(fields: &'a [MatField], name: &str) -> Option<&'a MatArray> {
    fields
        .iter()
        .find(|f| f.name == name)
        .and_then(|f| f.values.first())
}

fn numeric(m: &MatArray) -> Option<Vec<f64>> {
    match &m.data {
        MatData::Double(v) => Some(v.clone()),
        MatData::Single(v) => Some(v.iter().map(|&x| x as f64).collect()),
        MatData::Int32(v) => Some(v.iter().map(|&x| x as f64).collect()),
        _ => None,
    }
}

fn datenum_to_unix(dn: f64) -> f64 {
    (dn - MATLAB_1970_DAYS) * 86400.0
}

fn fmt_time_step(dn: &[f64]) -> String {
    if dn.len() < 2 {
        return "one sample, no spacing".to_string();
    }
    let step = (dn[1] - dn[0]) * 86400.0;
    format!("{:.1} s", step)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let input = match arg_value(&args, "--input") {
        Some(v) => v,
        None => {
            eprintln!("onc hydrophone: --input <mat file> is required");
            std::process::exit(1);
        }
    };
    let out = arg_value(&args, "--out");
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let bytes = match fs::read(&input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {input}: {e}");
            std::process::exit(1);
        }
    };
    let Some(arrays) = parse_mat(&bytes) else {
        eprintln!("{input}: MATLAB v5 parse void — the file stays unread");
        std::process::exit(1);
    };
    let Some(fields) = struct_fields(&arrays) else {
        eprintln!("{input}: no MATLAB struct at the top level — the spectrum stays unread");
        std::process::exit(1);
    };

    let names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
    eprintln!(
        "onc hydrophone: {input} carries a struct with fields {}",
        names.join(", ")
    );

    let Some(freq_m) = field(fields, "frequency") else {
        eprintln!("onc hydrophone: {input} carries no `frequency` field — pending");
        std::process::exit(1);
    };
    let Some(psd_m) = field(fields, "PSD") else {
        eprintln!("onc hydrophone: {input} carries no `PSD` field — pending");
        std::process::exit(1);
    };
    let Some(time_m) = field(fields, "time") else {
        eprintln!("onc hydrophone: {input} carries no `time` field — pending");
        std::process::exit(1);
    };

    let Some(freq) = numeric(freq_m) else {
        eprintln!("onc hydrophone: `frequency` carries no numeric axis — pending");
        std::process::exit(1);
    };
    let Some(psd) = numeric(psd_m) else {
        eprintln!("onc hydrophone: `PSD` carries no numeric values — pending");
        std::process::exit(1);
    };
    let Some(time) = numeric(time_m) else {
        eprintln!("onc hydrophone: `time` carries no numeric values — pending");
        std::process::exit(1);
    };

    let nfreq = freq.len();
    let ntime = time.len();
    if nfreq == 0 || ntime == 0 {
        eprintln!("onc hydrophone: {input} carries an empty frequency or time axis — pending");
        std::process::exit(1);
    }
    if psd.len() != nfreq * ntime {
        eprintln!(
            "onc hydrophone: {input} PSD carries {} values, frequency {} x time {} = {} — the grid stays unmapped",
            psd.len(),
            nfreq,
            ntime,
            nfreq * ntime
        );
        std::process::exit(1);
    }

    let bin_width = if nfreq >= 2 { freq[1] - freq[0] } else { 0.0 };
    let psd_min = psd
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .fold(f64::INFINITY, f64::min);
    let psd_max = psd
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .fold(f64::NEG_INFINITY, f64::max);
    let psd_nonfinite = psd.iter().filter(|v| !v.is_finite()).count();
    let count_psd = field(fields, "countPSD").and_then(numeric);
    let calibrated = field(fields, "isCalibrated")
        .and_then(numeric)
        .and_then(|v| v.first().copied());

    eprintln!(
        "onc hydrophone: frequency {} bins, {} Hz .. {} Hz, step {} Hz",
        nfreq,
        freq.first()
            .map_or("absent".to_string(), |v| format!("{v}")),
        freq.last().map_or("absent".to_string(), |v| format!("{v}")),
        bin_width
    );
    eprintln!(
        "onc hydrophone: time {} samples, datenum {} .. {}, spacing {}",
        ntime,
        time.first()
            .map_or("absent".to_string(), |v| format!("{v:.6}")),
        time.last()
            .map_or("absent".to_string(), |v| format!("{v:.6}")),
        fmt_time_step(&time)
    );
    eprintln!(
        "onc hydrophone: PSD {} values, range {} .. {}, {} non-finite",
        psd.len(),
        if psd_min.is_finite() {
            format!("{psd_min}")
        } else {
            "absent".to_string()
        },
        if psd_max.is_finite() {
            format!("{psd_max}")
        } else {
            "absent".to_string()
        },
        psd_nonfinite
    );
    if let Some(c) = &count_psd {
        eprintln!(
            "onc hydrophone: countPSD {}",
            c.iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    } else {
        eprintln!("onc hydrophone: countPSD absent");
    }
    match calibrated {
        Some(1.0) => eprintln!("onc hydrophone: isCalibrated 1 (PSD in dB re 1 uPa^2/Hz)"),
        Some(0.0) => {
            eprintln!("onc hydrophone: isCalibrated 0 — the PSD carries uncalibrated counts")
        }
        Some(v) => eprintln!("onc hydrophone: isCalibrated {v}"),
        None => eprintln!("onc hydrophone: isCalibrated absent"),
    }

    let Some(out) = out else {
        return;
    };

    let lat = match arg_f64(&args, "--lat") {
        Some(v) => v,
        None => {
            eprintln!(
                "onc hydrophone: --lat absent — the station position stays unmeasured (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let lon = match arg_f64(&args, "--lon") {
        Some(v) => v,
        None => {
            eprintln!(
                "onc hydrophone: --lon absent — the station position stays unmeasured (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let alt = match arg_f64(&args, "--alt") {
        Some(v) => v,
        None => {
            eprintln!(
                "onc hydrophone: --alt absent — the station depth stays unmeasured (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("onc hydrophone: --lsk <naif0012.tls> absent — the TDB clock stays unread");
            std::process::exit(1);
        }
    };
    let Some(lsk) = parse_lsk(&lsk_text) else {
        eprintln!("onc hydrophone: the leap-second kernel parses void — pending");
        std::process::exit(1);
    };

    let mut recs: Vec<GeoRec> = Vec::new();
    for t in 0..ntime {
        let Some(tdb) = lsk.unix_to_tdb(datenum_to_unix(time[t])) else {
            eprintln!("onc hydrophone: time sample {t} maps to no TDB second — skipped");
            continue;
        };
        for f in 0..nfreq {
            let val = psd[f + nfreq * t];
            if !val.is_finite() {
                continue;
            }
            recs.push(GeoRec {
                t: tdb,
                lat,
                lon,
                alt,
                freq: freq[f],
                bin_width,
                val,
                comp: COMP_ONC_PSD,
                station: 0,
            });
        }
    }
    if recs.is_empty() {
        eprintln!("onc hydrophone: no spectral records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    recs.sort_by(|a, b| a.t.total_cmp(&b.t).then(a.freq.total_cmp(&b.freq)));
    let bytes = write_bin(MAGIC_ONC, &recs);
    if fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_ONC, &bytes) {
        Some(parsed) => eprintln!(
            "{out}: {} geo records, {} B, roundtrip parses",
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
