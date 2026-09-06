use std::collections::BTreeMap;
use std::fs;

use omegaflow::atdf::{extract, field_of, full_year, strip_markers, LOGICAL_RECORD, TKFORM};
fn civil_days(y: i64, m: i64, d: i64) -> i64 {
    let yy = if m <= 2 { y - 1 } else { y };
    let era = if yy >= 0 { yy } else { yy - 399 } / 400;
    let yoe = yy - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}


const CACHES: &[&str] = &[
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5327328A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5337339A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_5340341A.TDF",
    "data/pds-ppi.igpp.ucla.edu/galileo_tdf_cache_6177179A.TDF",
];

const IDENTITY_ITEMS: &[u32] = &[
    9, 11, 12, 13, 26, 28, 64, 69, 70, 71, 92, 94, 95, 96, 98, 111, 116,
];

fn rec_utc(rec: &[u8]) -> f64 {
    let year = full_year(extract(rec, field_of(TKFORM, 3).unwrap()));
    let day = extract(rec, field_of(TKFORM, 4).unwrap());
    let hour = extract(rec, field_of(TKFORM, 5).unwrap());
    let minute = extract(rec, field_of(TKFORM, 6).unwrap());
    let second = extract(rec, field_of(TKFORM, 7).unwrap());
    let base = civil_days(year, 1, 1) as f64 * 86400.0;
    base + (day - 1) as f64 * 86400.0 + hour as f64 * 3600.0 + minute as f64 * 60.0
        + second as f64
}

fn main() {
    let report_path = match std::env::args().skip(1).find(|a| !a.starts_with('-')) {
        Some(p) => p,
        None => "tmp/galileo_ded27_tdf_sender_report.txt".to_string(),
    };
    let mut out: Vec<String> = Vec::new();
    out.push("galileo TRK-2-25 raw TDF sender-field census over the four floor-era caches".to_string());
    out.push("record fields decoded with src/archivar/atdf.rs TKFORM (canonical map docs/reference/trk-2-25-atdf.txt)".to_string());
    out.push("item 10 = station number (the recording station); item 12 data type 6 = ramp record; item 13 ground mode 3/4 = three-way; item 64 = uplink frequency band / source id".to_string());
    out.push("question 1: does any field of a doppler tracking record carry a station-like value (11..99) besides item 10?".to_string());
    out.push("question 2: which ramp-record stations (the atdf2ascii Xmtr source) cover the three-way samples — same-band, other-station, active at the sample time?".to_string());
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
        let mut threeway_station: BTreeMap<i64, usize> = BTreeMap::new();
        let mut ramp_station: BTreeMap<i64, usize> = BTreeMap::new();
        let mut ramp_gm: BTreeMap<i64, usize> = BTreeMap::new();
        let mut ramp_band: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        let mut threeway_band: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        let mut threeway_xmtr_on: BTreeMap<(i64, i64), usize> = BTreeMap::new();
        let mut threeway_samples: Vec<(f64, i64, i64, i64)> = Vec::new();
        let mut ramp_first: BTreeMap<(i64, i64), f64> = BTreeMap::new();
        let mut threeway = 0usize;
        let mut ramp = 0usize;
        for i in 2..nlog {
            let rec = &stripped[i * LOGICAL_RECORD..(i + 1) * LOGICAL_RECORD];
            let day = extract(rec, field_of(TKFORM, 4).unwrap());
            if day == 0 {
                continue;
            }
            let data_type = extract(rec, field_of(TKFORM, 12).unwrap());
            let ground_mode = extract(rec, field_of(TKFORM, 13).unwrap());
            let station = extract(rec, field_of(TKFORM, 10).unwrap());
            let band = extract(rec, field_of(TKFORM, 64).unwrap());
            *dtype_hist.entry(data_type).or_insert(0) += 1;
            if data_type == 6 {
                ramp += 1;
                *ramp_station.entry(station).or_insert(0) += 1;
                *ramp_gm.entry(ground_mode).or_insert(0) += 1;
                *ramp_band.entry((station, band)).or_insert(0) += 1;
                let t = rec_utc(rec);
                ramp_first
                    .entry((station, band))
                    .and_modify(|v| *v = v.min(t))
                    .or_insert(t);
            }
            if (data_type == 1 || data_type == 2) && (ground_mode == 3 || ground_mode == 4) {
                threeway += 1;
                let xmtr_on = extract(rec, field_of(TKFORM, 26).unwrap());
                *threeway_station.entry(station).or_insert(0) += 1;
                *threeway_band.entry((station, band)).or_insert(0) += 1;
                *threeway_xmtr_on.entry((station, xmtr_on)).or_insert(0) += 1;
                threeway_samples.push((rec_utc(rec), station, band, xmtr_on));
                for &item in IDENTITY_ITEMS {
                    let v = extract(rec, field_of(TKFORM, item).unwrap());
                    if (11..=99).contains(&v) && v != station {
                        *station_like_hits.entry(item).or_insert(0) += 1;
                    }
                }
            }
        }
        out.push(format!(
            "{cache}: logical {nlog} | data_type census {dtype_hist:?} | three-way doppler {threeway} | ramp {ramp}"
        ));
        out.push(format!("  three-way receiving station census: {threeway_station:?}"));
        out.push(format!("  three-way (station, uplink band) census: {threeway_band:?}"));
        out.push(format!(
            "  three-way (station, item-26 transmitter on/off) census: {threeway_xmtr_on:?}"
        ));
        out.push(format!(
            "  ramp ground-mode census: {ramp_gm:?} | ramp (station, band) census: {ramp_band:?}"
        ));
        if station_like_hits.is_empty() {
            out.push("  station-like second value (11..99 != item-10 station) across identity items of three-way doppler records: none".to_string());
        } else {
            out.push(format!("  station-like second value items: {station_like_hits:?}"));
        }
        if threeway_samples.is_empty() {
            out.push("  no three-way doppler samples (0 honored)".to_string());
            continue;
        }
        let mut covered_other = 0usize;
        let mut covered_any = 0usize;
        let mut candidates_dist: BTreeMap<usize, usize> = BTreeMap::new();
        for (t, st, band, _on) in &threeway_samples {
            let mut n_other = 0usize;
            let mut any = false;
            for (&(rs, rb), first) in &ramp_first {
                if *first <= *t {
                    any = true;
                    if rb == *band && rs != *st {
                        n_other += 1;
                    }
                }
            }
            if any {
                covered_any += 1;
            }
            if n_other > 0 {
                covered_other += 1;
            }
            *candidates_dist.entry(n_other).or_insert(0) += 1;
        }
        out.push(format!(
            "  three-way samples with any same-file ramp record active at their time: {covered_any} / {}",
            threeway_samples.len()
        ));
        out.push(format!(
            "  three-way samples with >= 1 same-band other-station ramp active (atdf2ascii Xmtr join): {covered_other} / {}",
            threeway_samples.len()
        ));
        out.push(format!(
            "  count of candidate uplink stations (other-station same-band ramps) per three-way sample: {candidates_dist:?}"
        ));
    }
    let text = out.join("\n");
    println!("{text}");
    let _ = fs::write(&report_path, text);
}
