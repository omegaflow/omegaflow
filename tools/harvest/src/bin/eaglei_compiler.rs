use omegaflow::archivar::geo::{
    COMP_EAGLEI_CUSTOMERS_OUT, GeoRec, MAGIC_EAGLEI, parse_bin, write_bin,
};
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::inflate::zip_members;
use omegaflow::lsk::{LeapSeconds, days_from_civil};
use std::collections::HashMap;
use std::path::Path;

const NETLOC: &str = "figshare.com";
const DEFAULT_YEAR: &str = "2014";
const DEFAULT_URL: &str = "https://ndownloader.figshare.com/files/42547717";
const GAZETTEER_URL: &str = "https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2023_Gazetteer/2023_Gaz_counties_national.zip";
const COMPILER: &str = "tools/harvest/src/bin/eaglei_compiler.rs";
const FORMAT: &str = "eaglei_outage";
const TTL: f64 = 604800.0;
const SAMPLE_TAU_S: f64 = 900.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
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
    let mut t = time.split(':');
    let hh: i64 = t.next()?.parse().ok()?;
    let mm: i64 = t.next()?.parse().ok()?;
    let ss: i64 = t.next()?.parse().ok()?;
    Some((days_from_civil(y, mo, da)? * 86400 + hh * 3600 + mm * 60 + ss) as f64)
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

fn parse_eaglei(
    text: &str,
    gazetteer: &HashMap<String, (f64, f64)>,
    lsk: &LeapSeconds,
) -> (Vec<GeoRec>, usize, usize) {
    let mut records = Vec::new();
    let mut missing_centroid = 0usize;
    let mut absent_value = 0usize;
    for line in text.lines() {
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() < 5 || cells[0] == "fips_code" {
            continue;
        }
        let Some(&(lat, lon)) = gazetteer.get(cells[0]) else {
            missing_centroid += 1;
            continue;
        };
        let customers = match num(cells[cells.len() - 2]) {
            Some(v) => v,
            None => {
                absent_value += 1;
                continue;
            }
        };
        let Some(unix) = parse_stamp(cells[cells.len() - 1]) else {
            continue;
        };
        let Some(t) = lsk.unix_to_tdb(unix) else {
            continue;
        };
        records.push(GeoRec {
            t,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: customers,
            comp: COMP_EAGLEI_CUSTOMERS_OUT,
            station: 0,
        });
    }
    (records, missing_centroid, absent_value)
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

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or("the embedded naif0012 table stays unread")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let year = match arg_value(args, "--year") {
        Some(v) => v,
        None => DEFAULT_YEAR.to_string(),
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("eaglei_{year}.bin"),
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

    let (csv_text, origin) = match arg_value(args, "--in").or_else(|| arg_value(args, "--input")) {
        Some(path) => {
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("read {path} returned void: {e}"))?;
            (text, path)
        }
        None => {
            let url = match arg_value(args, "--url") {
                Some(v) => v,
                None => DEFAULT_URL.to_string(),
            };
            let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
            (String::from_utf8_lossy(&bytes).into_owned(), url)
        }
    };

    let (mut records, missing_centroid, absent_value) = parse_eaglei(&csv_text, &gazetteer, &lsk);
    if records.is_empty() {
        return Err(format!(
            "{origin}: no county outage row left the join ({missing_centroid} without a centroid, {absent_value} without a value) — the bin stays unwritten (0 honored)"
        ));
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));

    let bin = write_bin(MAGIC_EAGLEI, &records);
    if let Some(parent) = Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|e| format!("create parent void: {e}"))?;
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;

    match parse_bin(MAGIC_EAGLEI, &bin) {
        Some(parsed) if parsed.len() == records.len() => {}
        Some(parsed) => {
            return Err(format!(
                "{out}: {} rows read back vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => return Err(format!("{out}: roundtrip parse returned void")),
    }

    let sha = omegaflow::archivar::sha256::sha256_hex(&bin);
    let v_min = records.iter().map(|r| r.val).fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r.val)
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} county outage records ({}, {} counties in the join, {} rows without a centroid, {} absent values), customers_out [{}, {}], {} bytes, sha256 {}",
        records.len(),
        year,
        gazetteer.len(),
        missing_centroid,
        absent_value,
        v_min,
        v_max,
        bin.len(),
        sha
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
        "quantity eaglei_customers_out eaglei_customers_out inverse-square relative count {} 0.0 0.0",
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
        eprintln!("eaglei_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAZ: &str = "USPS\tGEOID\tANSICODE\tNAME\tALAND\tAWATER\tALAND_SQMI\tAWATER_SQMI\tINTPTLAT\tINTPTLONG\n\
AL\t01001\t00161526\tAutauga County\t1539631461\t25677536\t594.455\t9.914\t32.532237\t-86.64644\n\
CO\t08037\t00161582\tEagle County\t1000000\t5000\t100.0\t5.0\t39.6277\t-106.695\n";

    const CSV: &str = "fips_code,county,state,customers_out,run_start_time\n\
01001,Autauga,Alabama,12,2014-11-01 04:00:00\n\
08037,Eagle,Colorado,0,2014-11-01 04:15:00\n\
99000,Foreign,Nowhere,5,2014-11-01 04:00:00\n\
01001,Autauga,Alabama,,2014-11-01 04:30:00\n";

    #[test]
    fn gazetteer_reads_geoid_and_centroid() {
        let g = parse_gazetteer(GAZ);
        assert_eq!(g.len(), 2);
        assert_eq!(g.get("01001"), Some(&(32.532237, -86.64644)));
    }

    #[test]
    fn the_join_supplies_the_coordinate_and_a_zero_is_kept() {
        let g = parse_gazetteer(GAZ);
        let lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(0.0, -1e12)],
        };
        let (records, missing, absent) = parse_eaglei(CSV, &g, &lsk);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].lat, 32.532237);
        assert_eq!(records[0].lon, -86.64644);
        assert_eq!(records[0].val, 12.0);
        assert_eq!(records[1].val, 0.0);
        assert_eq!(records[1].lat, 39.6277);
        assert_eq!(missing, 1);
        assert_eq!(absent, 1);
    }

    #[test]
    fn the_stamp_reads_civil_utc() {
        let days = days_from_civil(2014, 11, 1).unwrap();
        assert_eq!(
            parse_stamp("2014-11-01 04:15:00"),
            Some((days * 86400 + 4 * 3600 + 15 * 60) as f64)
        );
        assert_eq!(parse_stamp("not-a-stamp"), None);
    }

    #[test]
    fn the_frame_carries_the_geo_magic_and_roundtrips() {
        let records = vec![GeoRec {
            t: 1.0,
            lat: 32.5,
            lon: -86.6,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 12.0,
            comp: COMP_EAGLEI_CUSTOMERS_OUT,
            station: 0,
        }];
        let bytes = write_bin(MAGIC_EAGLEI, &records);
        assert_eq!(&bytes[0..4], MAGIC_EAGLEI.as_slice());
        let parsed = parse_bin(MAGIC_EAGLEI, &bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].val, 12.0);
        assert_eq!(parsed[0].comp, COMP_EAGLEI_CUSTOMERS_OUT);
    }
}
