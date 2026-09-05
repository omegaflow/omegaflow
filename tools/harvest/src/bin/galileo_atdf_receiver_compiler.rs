use std::collections::BTreeMap;

use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::atdf::{self, extract, tracking_record, Field, Tracking, LOGICAL_RECORD};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{days_from_civil, LeapSeconds};

const BASE: &str = "https://pds-ppi.igpp.ucla.edu/annex/";
const VOLUMES: &[&str] = &[
    "GO-J-RSS-1-TDF-V1.0",
    "GO-JS-RSS-1-TDF-V1.0",
    "GO-JG-RSS-1-TDF-V1.0",
    "GO-SS-RSS-1-TDF-V1.0",
    "GO-SUN-RSS-1-TDF-V1.0",
];

const RECEIVER_REF: Field = Field {
    item: 71,
    start: 1454,
    stop: 1457,
    signlength: 0,
    outlength: 8,
    name: "DOPPLER_RCVR_REF",
};
const RCVR_NUMBER: Field = Field {
    item: 92,
    start: 1596,
    stop: 1599,
    signlength: 0,
    outlength: 8,
    name: "RCVR_NUMBER",
};
const AMP_NUMBER: Field = Field {
    item: 94,
    start: 1601,
    stop: 1602,
    signlength: 0,
    outlength: 8,
    name: "AMP_NUMBER",
};
const AMP_TYPE: Field = Field {
    item: 95,
    start: 1603,
    stop: 1604,
    signlength: 0,
    outlength: 8,
    name: "AMP_TYPE",
};

fn tdb_of(tr: &Tracking, lsk: &LeapSeconds) -> Option<f64> {
    if tr.day <= 0 || tr.day > 366 {
        return None;
    }
    let year = atdf::full_year(tr.year);
    let days = days_from_civil(year, 1, 1)? + tr.day - 1;
    let unix = days as f64 * 86400.0
        + tr.hour as f64 * 3600.0
        + tr.minute as f64 * 60.0
        + tr.second as f64;
    lsk.unix_to_tdb(unix)
}

fn receiver_tuple(rec: &[u8]) -> [i64; 4] {
    [
        extract(rec, &RECEIVER_REF),
        extract(rec, &RCVR_NUMBER),
        extract(rec, &AMP_NUMBER),
        extract(rec, &AMP_TYPE),
    ]
}

fn reduce_receiver(name: &str, bytes: &[u8], lsk: &LeapSeconds) -> Option<Vec<[f64; 12]>> {
    let stripped = atdf::strip_markers(bytes)?;
    let nlog = stripped.len() / LOGICAL_RECORD;
    if nlog < 3 {
        eprintln!("{name}: {nlog} logical records — too short");
        return None;
    }
    let mut out: Vec<[f64; 12]> = Vec::new();
    let mut resid_min = f64::INFINITY;
    let mut resid_max = f64::NEG_INFINITY;
    let mut hist: BTreeMap<(i64, i64, i64, i64), usize> = BTreeMap::new();
    for i in 2..nlog {
        let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
        let tr = tracking_record(rec);
        if tr.day == 0 || !(tr.data_type == 1 || tr.data_type == 2) {
            continue;
        }
        let sampler = tr.sampler_time as f64 / 100.0;
        if sampler <= 0.0 {
            continue;
        }
        let Some(tdb) = tdb_of(&tr, lsk) else {
            continue;
        };
        let resid = tr.doppler_resid as f64 / 1000.0;
        if !resid.is_finite() {
            continue;
        }
        resid_min = resid_min.min(resid);
        resid_max = resid_max.max(resid);
        let rx = receiver_tuple(rec);
        *hist
            .entry((tr.station, tr.ground_mode, rx[0], rx[3]))
            .or_insert(0) += 1;
        out.push([
            tdb,
            resid,
            tr.station as f64,
            tr.ground_mode as f64,
            tr.data_type as f64,
            tr.doppler_ref as f64 / 10.0,
            sampler,
            tr.signal_strength as f64,
            rx[0] as f64,
            rx[1] as f64,
            rx[2] as f64,
            rx[3] as f64,
        ]);
    }
    if out.is_empty() {
        eprintln!("{name}: no doppler resid samples");
        return None;
    }
    eprintln!(
        "{name}: {} resid samples, resid {resid_min:.3}..{resid_max:.3} Hz",
        out.len()
    );
    for ((station, mode, rref, atype), count) in &hist {
        eprintln!(
            "  {name}: station {station} mode {mode}: receiver_ref {rref}, amplifier_type {atype} — {count}"
        );
    }
    Some(out)
}

fn jd_date(tdb_s: f64) -> String {
    let jd = 2451545.0 + tdb_s / 86400.0;
    let unix_day = (jd - 2440587.5).round() as i64;
    match omegaflow::spectral::civil_from_days(unix_day) {
        Some((y, m, d)) => format!("{y:04}-{m:02}-{d:02}"),
        None => format!("tdb {tdb_s:.0} s"),
    }
}

fn write_receiver_bin(records: &[[f64; 12]]) -> Vec<u8> {
    let mut bin = Vec::with_capacity(8 + records.len() * 96);
    bin.extend_from_slice(b"GARX");
    bin.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            bin.extend_from_slice(&v.to_le_bytes());
        }
    }
    bin
}

fn parse_receiver_bin(data: &[u8]) -> Option<Vec<[f64; 12]>> {
    if data.len() < 8 || &data[0..4] != b"GARX" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 96 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 96;
        let mut r = [0.0f64; 12];
        for k in 0..12 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

fn files_of(volume: &str) -> Vec<String> {
    let url = format!("{BASE}{volume}/TDF/");
    let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
        eprintln!("{volume}: dir listing fetch void ({url})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{volume}: dir listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = Vec::new();
    for token in text.split("href=\"") {
        let Some(end) = token.find('"') else {
            continue;
        };
        let name = &token[..end];
        if name.ends_with(".TDF") {
            out.push(format!("TDF/{name}"));
        }
    }
    out.sort();
    out.dedup();
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned()
    {
        Some(path) => path,
        None => "data/galileo_receiver.bin".to_string(),
    };
    let local: Option<&str> = args
        .iter()
        .position(|a| a == "--file")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());
    let limit: Option<usize> = args
        .iter()
        .position(|a| a == "--limit")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok());
    let report_only = local.is_some() && !args.iter().any(|a| a == "--out");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let mut merged: Vec<[f64; 12]> = Vec::new();
    if let Some(path) = local {
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("{path}: local read void");
            return;
        };
        if let Some(samples) = reduce_receiver(path, &bytes, &lsk) {
            merged.extend(samples);
        }
    } else {
        let mut fetched = 0usize;
        for volume in VOLUMES {
            let rels = files_of(volume);
            eprintln!("{volume}: {} TDF files", rels.len());
            for rel in rels {
                if let Some(l) = limit {
                    if fetched >= l {
                        break;
                    }
                }
                fetched += 1;
                let url = format!("{BASE}{volume}/{rel}");
                let Some(bytes) = fetch_raw_bytes(&url, 604800) else {
                    eprintln!("{rel}: fetch void ({url})");
                    continue;
                };
                if let Some(samples) = reduce_receiver(&rel, &bytes, &lsk) {
                    merged.extend(samples);
                }
            }
        }
    }
    if merged.is_empty() {
        eprintln!("no galileo receiver samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if report_only {
        let mut rows: Vec<(i64, i64, i64, i64, i64, i64, usize)> = Vec::new();
        for r in &merged {
            let key = (
                r[2] as i64,
                r[3] as i64,
                r[8] as i64,
                r[9] as i64,
                r[10] as i64,
                r[11] as i64,
            );
            let slot = rows.iter_mut().find(|row| {
                row.0 == key.0
                    && row.1 == key.1
                    && row.2 == key.2
                    && row.3 == key.3
                    && row.4 == key.4
                    && row.5 == key.5
            });
            match slot {
                Some(row) => row.6 += 1,
                None => rows.push((key.0, key.1, key.2, key.3, key.4, key.5, 1)),
            }
        }
        println!(
            "station mode receiver_ref receiver_number amplifier_number amplifier_type samples"
        );
        for (s, m, rr, rn, an, at, c) in &rows {
            println!("{s} {m} {rr} {rn} {an} {at} {c}");
        }
        return;
    }
    std::fs::create_dir_all("data").ok();
    let bin = write_receiver_bin(&merged);
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match parse_receiver_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let rmin = parsed.iter().map(|r| r[1]).fold(f64::INFINITY, f64::min);
            let rmax = parsed
                .iter()
                .map(|r| r[1])
                .fold(f64::NEG_INFINITY, f64::max);
            let mut stations: Vec<i64> = parsed.iter().map(|r| r[2] as i64).collect();
            stations.sort_unstable();
            stations.dedup();
            let mut modes: Vec<i64> = parsed.iter().map(|r| r[3] as i64).collect();
            modes.sort_unstable();
            modes.dedup();
            let mut refs: Vec<i64> = parsed.iter().map(|r| r[8] as i64).collect();
            refs.sort_unstable();
            refs.dedup();
            let mut types: Vec<i64> = parsed.iter().map(|r| r[11] as i64).collect();
            types.sort_unstable();
            types.dedup();
            eprintln!(
                "{out}: {} receiver samples ({}..{}), resid {rmin:.3}..{rmax:.3} Hz, stations {stations:?}, modes {modes:?}, receiver_ref {refs:?}, amplifier_type {types:?}, {} B — roundtrip parses",
                parsed.len(),
                jd_date(d0[0]),
                jd_date(d1[0]),
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
        }
    }
    if ci_mode && !upload_release("pds-ppi.igpp.ucla.edu", &out) {
        std::process::exit(1);
    }
}
