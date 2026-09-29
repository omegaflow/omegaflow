use omegaflow::archivar::geo::{COMP_ERSSTV5, GeoRec, MAGIC_ERSSTV5, parse_bin, write_bin};
use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{LeapSeconds, days_from_civil};

const NETLOC: &str = "coastwatch.pfeg.noaa.gov";
const BASE: &str = "https://coastwatch.pfeg.noaa.gov/erddap/griddap";
const DATASET: &str = "nceiErsstv5";
const DEFAULT_OUT: &str = "ersstv5_nino34.bin";
const J2000_UNIX_OFFSET: f64 = 946728000.0;
const SECS_PER_DAY: f64 = 86400.0;
const TIME_START: &str = "1854-01-01T00:00:00Z";
const TIME_STOP: &str = "2026-08-01T00:00:00Z";
const LAT_MIN: f64 = -5.0;
const LAT_MAX: f64 = 5.0;
const LON_MIN: f64 = 190.0;
const LON_MAX: f64 = 240.0;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_f64(args: &[String], name: &str, fallback: f64) -> Result<f64, String> {
    match arg_value(args, name) {
        Some(v) => v
            .parse::<f64>()
            .ok()
            .filter(|x| x.is_finite())
            .ok_or_else(|| format!("{name} {v} carries no finite degree value")),
        None => Ok(fallback),
    }
}

fn grid_url(
    time_start: &str,
    time_stop: &str,
    lat_min: f64,
    lat_max: f64,
    lon_min: f64,
    lon_max: f64,
) -> String {
    format!(
        "{BASE}/{DATASET}.csv?ssta[({time_start}):1:({time_stop})][(0.0):1:(0.0)][({lat_min}):1:({lat_max})][({lon_min}):1:({lon_max})]"
    )
}

fn iso_to_tdb(lsk: &LeapSeconds, iso: &str) -> Option<f64> {
    let date = iso.get(0..10)?;
    let (y, rest) = date.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    let year: i64 = y.parse().ok()?;
    let month: i64 = m.parse().ok()?;
    let day: i64 = d.parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    let unix = days as f64 * SECS_PER_DAY;
    let leap = lsk.leap_at(unix)?;
    Some(unix + lsk.delta_t_a + leap - J2000_UNIX_OFFSET)
}

fn parse_csv(text: &str, lsk: &LeapSeconds) -> Vec<GeoRec> {
    let mut records = Vec::new();
    for line in text.lines().skip(2) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5 {
            continue;
        }
        let Some(t) = iso_to_tdb(lsk, cols[0]) else {
            continue;
        };
        let Some(lat) = cols[2].parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        let Some(lon) = cols[3].parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        let Some(ssta) = cols[4].parse::<f64>().ok().filter(|v| v.is_finite()) else {
            continue;
        };
        records.push(GeoRec {
            t,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: ssta,
            comp: COMP_ERSSTV5,
            station: 0,
        });
    }
    records
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let time_start = match arg_value(args, "--time-start") {
        Some(v) => v,
        None => TIME_START.to_string(),
    };
    let time_stop = match arg_value(args, "--time-stop") {
        Some(v) => v,
        None => TIME_STOP.to_string(),
    };
    let lat_min = parse_f64(args, "--lat-min", LAT_MIN)?;
    let lat_max = parse_f64(args, "--lat-max", LAT_MAX)?;
    let lon_min = parse_f64(args, "--lon-min", LON_MIN)?;
    let lon_max = parse_f64(args, "--lon-max", LON_MAX)?;
    if !(lat_min < lat_max) || !(lon_min < lon_max) {
        return Err("the bounding box carries no positive extent".to_string());
    }

    let url = grid_url(&time_start, &time_stop, lat_min, lat_max, lon_min, lon_max);
    let bytes = fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let text = String::from_utf8_lossy(&bytes);
    let records = parse_csv(&text, &lsk);
    if records.is_empty() {
        return Err(format!(
            "{url}: no measured ssta cell left the harvest ({} B) — the bin stays unwritten (0 honored)",
            bytes.len()
        ));
    }

    let bin = write_bin(MAGIC_ERSSTV5, &records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(MAGIC_ERSSTV5, &bin) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} Niño3.4 ssta cells ({url}, box {lat_min}..{lat_max} lat / {lon_min}..{lon_max} lon), roundtrip parses",
                parsed.len()
            );
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ersstv5_compiler: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_url_names_the_measured_nino34_csv() {
        let url = grid_url(TIME_START, TIME_STOP, LAT_MIN, LAT_MAX, LON_MIN, LON_MAX);
        assert!(
            url.starts_with("https://coastwatch.pfeg.noaa.gov/erddap/griddap/nceiErsstv5.csv?ssta")
        );
        assert!(url.contains("[(1854-01-01T00:00:00Z):1:(2026-08-01T00:00:00Z)]"));
        assert!(url.contains("[(-5):1:(5)]"));
        assert!(url.contains("[(190):1:(240)]"));
    }

    #[test]
    fn iso_to_tdb_converts_civil_and_pre_1972() {
        let lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(0.0, -1e12)],
        };
        let days = days_from_civil(1854, 1, 1).unwrap() as f64;
        let expect = days * SECS_PER_DAY + 32.184 + 0.0 - J2000_UNIX_OFFSET;
        assert_eq!(iso_to_tdb(&lsk, "1854-01-01T00:00:00Z"), Some(expect));
        let lsk2 = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let days2 = days_from_civil(2020, 1, 1).unwrap() as f64;
        let expect2 = days2 * SECS_PER_DAY + 32.184 + 37.0 - J2000_UNIX_OFFSET;
        assert_eq!(iso_to_tdb(&lsk2, "2020-01-01T00:00:00Z"), Some(expect2));
    }

    #[test]
    fn bin_roundtrip_carries_the_ssta_record() {
        let records = vec![
            GeoRec {
                t: -4_607_323_167.816,
                lat: -4.0,
                lon: 190.0,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: -1.0162277,
                comp: COMP_ERSSTV5,
                station: 0,
            },
            GeoRec {
                t: 946_728_000.0,
                lat: 0.0,
                lon: 240.0,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: 2.5,
                comp: COMP_ERSSTV5,
                station: 0,
            },
        ];
        let bytes = write_bin(MAGIC_ERSSTV5, &records);
        let parsed = parse_bin(MAGIC_ERSSTV5, &bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].val, -1.0162277);
        assert_eq!(parsed[0].comp, COMP_ERSSTV5);
        assert_eq!(parsed[1].lat, 0.0);
    }
}
