use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{
    COMP_GOEV_DURATION_H, COMP_GOEV_MAX_CUSTOMERS, GeoRec, MAGIC_GOEV, parse_bin, write_bin,
};
use omegaflow::archivar::grid_outage::{COMP_CAUSE_CLASS, cause_code, component_name};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::zip_members;
use omegaflow::lsk::days_from_civil;
use std::collections::HashMap;
use std::path::Path;

const NETLOC: &str = "data.openei.org";
const DEFAULT_URL: &str = "https://data.openei.org/files/6458/Outage_Dataset_R1.zip";
const GAZETTEER_URL: &str = "https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2023_Gazetteer/2023_Gaz_counties_national.zip";
const COMPILER: &str = "tools/harvest/src/bin/grid_outage_event_compiler.rs";
const FORMAT: &str = "grid_outage_event";
const OUT_PATH: &str = "grid_outage_events.bin";
const TTL: f64 = 604800.0;
const SAMPLE_TAU_S: f64 = 3600.0;

const COL_EVENT_TYPE: &str = "Event Type";
const COL_FIPS: &str = "fips";
const COL_START: &str = "start_time";
const COL_DURATION: &str = "duration";
const COL_MAX_CUSTOMERS: &str = "max_customers";

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
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if in_quotes => {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            }
            '"' => in_quotes = true,
            ',' if !in_quotes => out.push(std::mem::take(&mut cur)),
            _ => cur.push(ch),
        }
    }
    out.push(cur);
    out
}

fn num(cell: &str) -> Option<f64> {
    let t = cell.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_stamp(cell: &str) -> Option<f64> {
    let (date, time) = cell.trim().split_once(' ')?;
    let mut d = date.split('-');
    let y: i64 = d.next()?.parse().ok()?;
    let mo: i64 = d.next()?.parse().ok()?;
    let da: i64 = d.next()?.parse().ok()?;
    if d.next().is_some() {
        return None;
    }
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: i64 = t.next().unwrap_or("0").parse().ok()?;
    if t.next().is_some() {
        return None;
    }
    Some((days_from_civil(y, mo, da)? * 86400 + hh * 3600 + mm * 60 + ss) as f64)
}

fn fips_key(cell: &str) -> Option<String> {
    let t = cell.trim();
    if t.is_empty() {
        return None;
    }
    let digits = t.split('.').next()?;
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(format!("{:0>5}", digits))
}

fn parse_gazetteer(text: &str) -> HashMap<String, (f64, f64)> {
    let mut out = HashMap::new();
    for line in text.lines().skip(1) {
        let cells: Vec<&str> = line.split('\t').map(str::trim).collect();
        if cells.len() < 10 {
            continue;
        }
        let geoid = cells[1];
        if geoid.is_empty() {
            continue;
        }
        let Some(lat) = num(cells[8]) else { continue };
        let Some(lon) = num(cells[9]) else { continue };
        out.insert(geoid.to_string(), (lat, lon));
    }
    out
}

fn gazetteer_text(url: &str) -> Result<String, String> {
    let bytes = fetch_raw_bytes(url).ok_or_else(|| format!("{url}: fetch void"))?;
    let mut text: Option<String> = None;
    zip_members(&bytes, |name, data| {
        if text.is_none() && name.ends_with(".txt") {
            text = Some(String::from_utf8_lossy(data).into_owned());
        }
    });
    text.ok_or_else(|| format!("{url}: zip carries no .txt gazetteer member"))
}

struct Stats {
    rows: usize,
    missing_centroid: usize,
    absent_start: usize,
    absent_duration: usize,
    absent_max_customers: usize,
    unclassified_cause: usize,
}

impl Stats {
    fn new() -> Self {
        Stats {
            rows: 0,
            missing_centroid: 0,
            absent_start: 0,
            absent_duration: 0,
            absent_max_customers: 0,
            unclassified_cause: 0,
        }
    }
}

struct EventRow {
    epoch_unix: f64,
    lat: f64,
    lon: f64,
    cause: u32,
    duration_h: Option<f64>,
    max_customers: Option<f64>,
}

fn parse_member(
    text: &str,
    gazetteer: &HashMap<String, (f64, f64)>,
    stats: &mut Stats,
) -> Vec<EventRow> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let cols = split_row(header);
    let col = |name: &str| cols.iter().position(|c| c.trim() == name);
    let (Some(c_event), Some(c_fips), Some(c_start), Some(c_duration), Some(c_max)) = (
        col(COL_EVENT_TYPE),
        col(COL_FIPS),
        col(COL_START),
        col(COL_DURATION),
        col(COL_MAX_CUSTOMERS),
    ) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells = split_row(line);
        let Some(&(lat, lon)) = fips_key(cells.get(c_fips).map(String::as_str).unwrap_or(""))
            .as_ref()
            .and_then(|f| gazetteer.get(f))
        else {
            stats.missing_centroid += 1;
            continue;
        };
        let Some(epoch_unix) = cells.get(c_start).and_then(|c| parse_stamp(c)) else {
            stats.absent_start += 1;
            continue;
        };
        let cause = cause_code(cells.get(c_event).map(String::as_str).unwrap_or(""));
        if cause == omegaflow::archivar::grid_outage::CAUSE_UNCLASSIFIED {
            stats.unclassified_cause += 1;
        }
        let duration_h = cells.get(c_duration).and_then(|c| num(c));
        let max_customers = cells.get(c_max).and_then(|c| num(c));
        if duration_h.is_none() {
            stats.absent_duration += 1;
        }
        if max_customers.is_none() {
            stats.absent_max_customers += 1;
        }
        if duration_h.is_none() && max_customers.is_none() {
            continue;
        }
        stats.rows += 1;
        out.push(EventRow {
            epoch_unix,
            lat,
            lon,
            cause,
            duration_h,
            max_customers,
        });
    }
    out
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or("the embedded naif0012 table stays unread")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => OUT_PATH.to_string(),
    };

    let gaz_url = match arg_value(args, "--gazetteer") {
        Some(v) => v,
        None => GAZETTEER_URL.to_string(),
    };
    let gaz_text = match arg_value(args, "--gazetteer-in") {
        Some(path) => {
            std::fs::read_to_string(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => gazetteer_text(&gaz_url)?,
    };
    let gazetteer = parse_gazetteer(&gaz_text);
    if gazetteer.is_empty() {
        return Err(format!(
            "{gaz_url}: no county centroid parses — the join table stays empty"
        ));
    }

    let (zip_bytes, origin) = match arg_value(args, "--in").or_else(|| arg_value(args, "--input")) {
        Some(path) => {
            let bytes =
                std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?;
            (bytes, path)
        }
        None => {
            let url = match arg_value(args, "--url") {
                Some(v) => v,
                None => DEFAULT_URL.to_string(),
            };
            let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
            (bytes, url)
        }
    };

    let mut stats = Stats::new();
    let mut events: Vec<EventRow> = Vec::new();
    let mut members = 0usize;
    let ok = zip_members(&zip_bytes, |name, data| {
        let file = name.rsplit('/').next().unwrap_or(name);
        if !file.starts_with("eaglei_outages_with_events_")
            || !file.ends_with(".csv")
            || file.contains("_lag")
        {
            return;
        }
        members += 1;
        let text = String::from_utf8_lossy(data);
        events.extend(parse_member(&text, &gazetteer, &mut stats));
    });
    if ok.is_none() {
        return Err(format!("{origin}: zip members stay unread"));
    }
    if members == 0 {
        return Err(format!(
            "{origin}: no eaglei_outages_with_events_YYYY.csv member — the bin stays unwritten"
        ));
    }
    if events.is_empty() {
        return Err(format!(
            "{origin}: no county outage event leaves the join ({members} members) — the bin stays unwritten (0 honored)"
        ));
    }

    let mut records: Vec<GeoRec> = Vec::new();
    for e in &events {
        let Some(epoch_tdb) = lsk.unix_to_tdb(e.epoch_unix) else {
            continue;
        };
        for (comp, value) in [
            (COMP_GOEV_DURATION_H, e.duration_h),
            (COMP_GOEV_MAX_CUSTOMERS, e.max_customers),
            (COMP_CAUSE_CLASS, Some(e.cause as f64)),
        ] {
            let Some(val) = value else { continue };
            records.push(GeoRec {
                t: epoch_tdb,
                lat: e.lat,
                lon: e.lon,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val,
                comp,
                station: 0,
            });
        }
    }
    if records.is_empty() {
        return Err(format!(
            "{origin}: {} events carry no measured component — the bin stays unwritten",
            events.len()
        ));
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));

    let bin = write_bin(MAGIC_GOEV, &records);
    if let Some(parent) = Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|e| format!("create parent void: {e}"))?;
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;

    match parse_bin(MAGIC_GOEV, &bin) {
        Some(parsed) if parsed.len() == records.len() => {}
        Some(parsed) => {
            return Err(format!(
                "{out}: {} records read back vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => return Err(format!("{out}: roundtrip parse returned void")),
    }

    let sha = sha256_hex(&bin);
    let t_min = records.iter().map(|r| r.t).fold(f64::INFINITY, f64::min);
    let t_max = records
        .iter()
        .map(|r| r.t)
        .fold(f64::NEG_INFINITY, f64::max);
    let duration = records
        .iter()
        .filter(|r| r.comp == COMP_GOEV_DURATION_H)
        .count();
    let customers = records
        .iter()
        .filter(|r| r.comp == COMP_GOEV_MAX_CUSTOMERS)
        .count();
    let causes = records
        .iter()
        .filter(|r| r.comp == COMP_CAUSE_CLASS)
        .count();
    eprintln!(
        "{out}: {} county events from {} CSV members → {} records ({} duration_h, {} max_customers, {} cause_class), {} bytes, sha256 {}",
        events.len(),
        members,
        records.len(),
        duration,
        customers,
        causes,
        bin.len(),
        sha
    );
    eprintln!(
        "  join: {} rows without a county centroid, {} without a start stamp, {} without duration, {} without max customers, {} with an unclassified Event Type",
        stats.missing_centroid,
        stats.absent_start,
        stats.absent_duration,
        stats.absent_max_customers,
        stats.unclassified_cause
    );
    eprintln!(
        "  window: TDB {:.0} … {:.0} ({} rows read)",
        t_min, t_max, stats.rows
    );
    eprintln!(
        "  cause: categorical codebook in src/archivar/grid_outage.rs; the asset carries it as \
         the grid_outage_cause_class component. The cause is a label, not a measured unit, so it \
         is not declared as a field/quantity"
    );
    eprintln!(
        "  position: county centroid from the 2023 Census Gazetteer (FIPS join); the source carries \
         the county, not a point — the centroid is a representative region anchor, never a measured coordinate"
    );
    for comp in [
        COMP_GOEV_DURATION_H,
        COMP_GOEV_MAX_CUSTOMERS,
        COMP_CAUSE_CLASS,
    ] {
        if component_name(comp).is_none() {
            return Err(format!("component {comp} carries no register key"));
        }
    }

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
        "quantity grid_outage_duration_h grid_outage_duration_h inverse-square source-parameter h {} 0.0 0.0",
        SAMPLE_TAU_S as u64
    );
    println!(
        "quantity grid_outage_max_customers grid_outage_max_customers inverse-square relative count {} 0.0 0.0",
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
        eprintln!("grid_outage_event_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAZ: &str = "USPS\tGEOID\tANSICODE\tNAME\tALAND\tAWATER\tALAND_SQMI\tAWATER_SQMI\tINTPTLAT\tINTPTLONG\n\
AL\t01001\t00161526\tAutauga County\t1539631461\t25677536\t594.455\t9.914\t32.532237\t-86.64644\n\
CO\t08037\t00161582\tEagle County\t1000000\t5000\t100.0\t5.0\t39.6277\t-106.695\n";

    const CSV: &str = "event_id,state_event,Datetime Event Began,Datetime Restoration,Event Type,fips,state,county,start_time,duration,end_time,min_customers,max_customers,mean_customers\n\
Alabama-0,Alabama,2023-01-12 14:00:00,2023-01-13 03:00:00,Severe Weather,1001,Alabama,Autauga,2023-01-12 19:00:00,52.0,2023-01-14 23:00:00,216,6873,1259.7\n\
Colorado-0,Colorado,2023-01-12 14:00:00,2023-01-13 03:00:00,Vandalism,08037,Colorado,Eagle,2023-01-12 20:00:00,,2023-01-13 00:00:00,0,0,0\n\
Nowhere-0,Nowhere,2023-01-12 14:00:00,2023-01-13 03:00:00,Other,99000,Nowhere,Nowhere,2023-01-12 21:00:00,1.0,2023-01-12 22:00:00,1,1,1\n";

    #[test]
    fn the_join_supplies_the_county_centroid_and_keeps_a_measured_zero() {
        let gaz = parse_gazetteer(GAZ);
        let mut stats = Stats::new();
        let events = parse_member(CSV, &gaz, &mut stats);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].lat, 32.532237);
        assert_eq!(events[0].lon, -86.64644);
        assert_eq!(events[0].duration_h, Some(52.0));
        assert_eq!(events[0].max_customers, Some(6873.0));
        assert_eq!(events[1].max_customers, Some(0.0));
        assert_eq!(events[1].duration_h, None);
        assert_eq!(stats.missing_centroid, 1);
        assert_eq!(stats.absent_duration, 1);
    }

    #[test]
    fn the_fips_key_pads_and_rejects_a_foreign_key() {
        assert_eq!(fips_key("1001").as_deref(), Some("01001"));
        assert_eq!(fips_key("01001").as_deref(), Some("01001"));
        assert_eq!(fips_key("99000"), Some("99000".to_string()));
        assert_eq!(fips_key(""), None);
        assert_eq!(fips_key("nowhere"), None);
    }

    #[test]
    fn the_stamp_reads_civil_utc() {
        let days = days_from_civil(2023, 1, 12).unwrap();
        assert_eq!(
            parse_stamp("2023-01-12 19:00:00"),
            Some((days * 86400 + 19 * 3600) as f64)
        );
        assert_eq!(parse_stamp("not-a-stamp"), None);
    }

    #[test]
    fn the_frame_carries_the_event_magic_and_roundtrips_with_cause() {
        let mut r = GeoRec {
            t: 1.0,
            lat: 32.5,
            lon: -86.6,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 52.0,
            comp: COMP_GOEV_DURATION_H,
            station: 0,
        };
        omegaflow::archivar::grid_outage::set_cause(&mut r, cause_code("Severe Weather"));
        let bytes = write_bin(MAGIC_GOEV, &[r]);
        assert_eq!(&bytes[0..4], MAGIC_GOEV.as_slice());
        let parsed = parse_bin(MAGIC_GOEV, &bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].comp, COMP_GOEV_DURATION_H);
        assert_eq!(
            omegaflow::archivar::grid_outage::cause_of(&parsed[0]),
            omegaflow::archivar::grid_outage::CAUSE_SEVERE_WEATHER
        );
    }
}
