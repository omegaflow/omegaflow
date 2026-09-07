use std::collections::BTreeMap;
use std::fs;

use omegaflow::atdf::{extract, field_of, strip_markers, LOGICAL_RECORD, TKFORM};

const CACHES: &[&str] = &[
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5327328A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5337339A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5340341A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_6177179A.TDF",
];

const IDENTITY_ITEMS: &[u32] = &[
    9, 11, 26, 28, 64, 69, 70, 71, 92, 94, 95, 96, 98, 116,
];

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_trk225_sender_census_report.txt".to_string(),
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo TRK-2-25 sender field census over the raw cached TDFs".to_string());
    out.push("binding: record fields decoded with src/archivar/atdf.rs TKFORM bit map".to_string());
    out.push("question: does any raw tracking record carry a station-like second value (11..99) beside the item-10 station word, and which data types are present".to_string());
    for cache in CACHES {
        let Ok(bytes) = fs::read(cache) else {
            out.push(format!("{cache}: read void"));
            continue;
        };
        let Some(stripped) = strip_markers(&bytes) else {
            out.push(format!("{cache}: marker strip void (len {})", bytes.len()));
            continue;
        };
        let nlog = stripped.len() / LOGICAL_RECORD;
        if nlog < 3 {
            out.push(format!("{cache}: {nlog} logical records — too short"));
            continue;
        }
        let mut dtype_hist: BTreeMap<i64, usize> = BTreeMap::new();
        let mut station_like_hits: BTreeMap<u32, usize> = BTreeMap::new();
        let mut threeway: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        let mut ramp: BTreeMap<i64, usize> = BTreeMap::new();
        let mut station_word: BTreeMap<i64, usize> = BTreeMap::new();
        let mut doppler_records = 0usize;
        let mut threeway_records = 0usize;
        for i in 2..nlog {
            let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
            let day = extract(rec, field_of(TKFORM, 4).unwrap());
            if day == 0 {
                continue;
            }
            let data_type = extract(rec, field_of(TKFORM, 12).unwrap());
            let ground_mode = extract(rec, field_of(TKFORM, 13).unwrap());
            let station = extract(rec, field_of(TKFORM, 10).unwrap());
            *dtype_hist.entry(data_type).or_insert(0) += 1;
            *station_word.entry(station).or_insert(0) += 1;
            if data_type == 1 || data_type == 2 {
                doppler_records += 1;
            }
            if (ground_mode == 3 || ground_mode == 4) && (data_type == 1 || data_type == 2) {
                threeway_records += 1;
                *threeway.entry((station, ground_mode)).or_insert(0) += 1;
                for &item in IDENTITY_ITEMS {
                    let v = extract(rec, field_of(TKFORM, item).unwrap());
                    if (11..=99).contains(&v) && v != station {
                        *station_like_hits.entry(item).or_insert(0) += 1;
                    }
                }
            }
            if data_type == 6 {
                *ramp.entry(station).or_insert(0) += 1;
            }
        }
        out.push(format!(
            "{cache}: logical {nlog} | data_type histogram {dtype_hist:?} | doppler {doppler_records} | three-way {threeway_records}"
        ));
        out.push(format!("  station word census: {station_word:?}"));
        if !threeway.is_empty() {
            out.push(format!("  three-way (station, ground_mode) records: {threeway:?}"));
        }
        if station_like_hits.is_empty() {
            out.push("  station-like second value (11..99 != item-10 station) across identity items: none".to_string());
        } else {
            out.push(format!(
                "  station-like second value items: {station_like_hits:?}"
            ));
        }
        if ramp.is_empty() {
            out.push("  ramp records (data_type 6): none".to_string());
        } else {
            out.push(format!("  ramp records (data_type 6) by station: {ramp:?}"));
        }
    }
    let text = out.join("\n");
    println!("{text}");
    let _ = fs::write(&report_path, text);
}
