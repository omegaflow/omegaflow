use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::session::fetch_superdarn_ascii;
use omegaflow::archivar::units::ymd_to_days;
use omegaflow::cdn::upload_release;
use omegaflow::geo::{COMP_SDARN_V, GeoRec, MAGIC_SDARN, parse_bin, write_bin};
use omegaflow::json::{JsonVal, parse_json, scalar_of};
use std::process::Command;

const NETLOC: &str = "superdarn.ca";

const RST_HDW_RAW: &str =
    "https://raw.githubusercontent.com/SuperDARN/rst/main/tables/superdarn/hdw/hdw.dat.";

const EARTH_RADIUS_KM: f64 = 6371.0;

const FIELD_LIST: &str = "\"v\",\"bmazm\",\"frang\",\"rsep\",\"slist\",\"elv\"";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn build_body(
    radar: &str,
    start_date: &str,
    start_time: &str,
    end_date: &str,
    end_time: &str,
    beam: &str,
) -> String {
    format!(
        "{{\"radar\":\"{}\",\"startDate\":\"{}\",\"startTime\":\"{}\",\"endDate\":\"{}\",\"endTime\":\"{}\",\"beam\":\"{}\",\"ftype\":\"json\",\"fields\":[{}]}}",
        radar, start_date, start_time, end_date, end_time, beam, FIELD_LIST
    )
}

fn fetch_hdw(code: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("120")
        .arg(format!("{}{}", RST_HDW_RAW, code))
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

#[derive(Clone)]
struct RadarSite {
    lat_deg: f64,
    lon_deg: f64,
    boresight_deg: f64,
}

fn hdw_site(text: &str, start_yyyymmdd: &str) -> Option<RadarSite> {
    let mut best_date: Option<String> = None;
    let mut best_site: Option<RadarSite> = None;
    let mut last_site: Option<RadarSite> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 8 {
            continue;
        }
        let date = t[2];
        if date.len() != 8 || !date.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let (Some(lat), Some(lon), Some(boresight)) = (
            t[4].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[5].parse::<f64>().ok().filter(|v| v.is_finite()),
            t[7].parse::<f64>().ok().filter(|v| v.is_finite()),
        ) else {
            continue;
        };
        let site = RadarSite {
            lat_deg: lat,
            lon_deg: lon,
            boresight_deg: boresight,
        };
        last_site = Some(site.clone());
        if date <= start_yyyymmdd && best_date.as_ref().is_none_or(|b| date > b.as_str()) {
            best_date = Some(date.to_string());
            best_site = Some(site);
        }
    }
    best_site.or(last_site)
}

fn obj_num(obj: &JsonVal, key: &str) -> Option<f64> {
    match obj {
        JsonVal::Obj(map) => map.get(key).and_then(scalar_of),
        _ => None,
    }
}

fn obj_str(obj: &JsonVal, key: &str) -> Option<String> {
    match obj {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Str(s)) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn parse_bracket_list(s: &str) -> Vec<f64> {
    let inner = s.trim().trim_start_matches('[').trim_end_matches(']');
    inner
        .split_whitespace()
        .filter_map(|w| w.parse::<f64>().ok())
        .filter(|v| v.is_finite())
        .collect()
}

fn list_of(obj: &JsonVal, key: &str) -> Option<Vec<f64>> {
    obj_str(obj, key).map(|s| parse_bracket_list(&s))
}

fn civil_unix(yr: i64, mo: u32, dy: u32, hr: u32, mt: u32, sc: u32, us: u32) -> Option<f64> {
    let days = ymd_to_days(yr, mo, dy)?;
    Some(
        days as f64 * 86400.0 + hr as f64 * 3600.0 + mt as f64 * 60.0 + sc as f64 + us as f64 / 1e6,
    )
}

fn time_unix(obj: &JsonVal) -> Option<f64> {
    let yr = obj_num(obj, "time.yr")?;
    let mo = obj_num(obj, "time.mo")?;
    let dy = obj_num(obj, "time.dy")?;
    let hr = obj_num(obj, "time.hr")?;
    let mt = obj_num(obj, "time.mt")?;
    let sc = obj_num(obj, "time.sc")?;
    let us = match obj_num(obj, "time.us") {
        Some(v) => v,
        None => 0.0,
    };
    civil_unix(
        yr as i64,
        mo as u32,
        dy as u32,
        hr as u32,
        mt as u32,
        sc as u32,
        us.max(0.0) as u32,
    )
}

fn slant_to_ground_km(slant_km: f64, elev_deg: f64) -> Option<f64> {
    if !slant_km.is_finite() || !elev_deg.is_finite() || slant_km <= 0.0 {
        return None;
    }
    let elv = elev_deg.to_radians();
    let x = slant_km * elv.cos();
    let h = slant_km * elv.sin();
    let theta = x.atan2(EARTH_RADIUS_KM + h);
    let rg = EARTH_RADIUS_KM * theta;
    if rg.is_finite() && rg >= 0.0 {
        Some(rg)
    } else {
        None
    }
}

fn destination(lat_deg: f64, lon_deg: f64, bearing_deg: f64, dist_km: f64) -> Option<(f64, f64)> {
    if !lat_deg.is_finite()
        || !lon_deg.is_finite()
        || !bearing_deg.is_finite()
        || !dist_km.is_finite()
        || dist_km < 0.0
    {
        return None;
    }
    let lat1 = lat_deg.to_radians();
    let lon1 = lon_deg.to_radians();
    let bearing = bearing_deg.to_radians();
    let delta = dist_km / EARTH_RADIUS_KM;
    let lat2 = (lat1.sin() * delta.cos() + lat1.cos() * delta.sin() * bearing.cos()).asin();
    let lon2 = lon1
        + (bearing.sin() * delta.sin() * lat1.cos()).atan2(delta.cos() - lat1.sin() * lat2.sin());
    Some((lat2.to_degrees(), lon2.to_degrees()))
}

fn gather(body: &str, site: &RadarSite, lsk: &LeapSeconds) -> Result<Vec<GeoRec>, String> {
    let json = fetch_superdarn_ascii(body, 300)
        .ok_or_else(|| "the ascii-call session stayed void (token or body absent)".to_string())?;
    let parsed = parse_json(&json).ok_or_else(|| "the ascii body parses void".to_string())?;
    let JsonVal::Arr(rows) = parsed else {
        return Err("the ascii body carries no record array".to_string());
    };
    let mut records = Vec::new();
    let mut skipped = 0usize;
    for row in rows {
        let (Some(gates), Some(vels), Some(elvs)) = (
            list_of(&row, "slist"),
            list_of(&row, "v"),
            list_of(&row, "elv"),
        ) else {
            skipped += 1;
            continue;
        };
        let (Some(bmazm), Some(frang), Some(rsep), Some(t_unix)) = (
            obj_num(&row, "bmazm"),
            obj_num(&row, "frang"),
            obj_num(&row, "rsep"),
            time_unix(&row),
        ) else {
            skipped += 1;
            continue;
        };
        if frang <= 0.0 || rsep <= 0.0 {
            skipped += 1;
            continue;
        }
        let bearing = site.boresight_deg + bmazm;
        for i in 0..gates.len() {
            let (Some(&gate), Some(&v), Some(&elv)) = (gates.get(i), vels.get(i), elvs.get(i))
            else {
                skipped += 1;
                continue;
            };
            if !v.is_finite() {
                skipped += 1;
                continue;
            }
            let slant_km = frang + gate * rsep;
            let Some(ground_km) = slant_to_ground_km(slant_km, elv) else {
                skipped += 1;
                continue;
            };
            let Some((lat, lon)) = destination(site.lat_deg, site.lon_deg, bearing, ground_km)
            else {
                skipped += 1;
                continue;
            };
            if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
                skipped += 1;
                continue;
            }
            let Some(tdb) = lsk.unix_to_tdb(t_unix) else {
                skipped += 1;
                continue;
            };
            records.push(GeoRec {
                t: tdb,
                lat,
                lon,
                alt: 0.0,
                freq: 0.0,
                bin_width: 0.0,
                val: v,
                comp: COMP_SDARN_V,
                station: 0,
            });
        }
    }
    if records.is_empty() {
        return Err(format!(
            "no cell rows ({} rows skipped) — the bin stays unwritten (0 honored)",
            skipped
        ));
    }
    eprintln!(
        "superdarn: {} cells, {} rows skipped",
        records.len(),
        skipped
    );
    Ok(records)
}

fn write_asset(records: &[GeoRec], out_path: &str) -> Result<usize, String> {
    let bytes = write_bin(MAGIC_SDARN, records);
    std::fs::write(out_path, &bytes).map_err(|e| format!("write {out_path}: {e}"))?;
    match parse_bin(MAGIC_SDARN, &bytes) {
        Some(parsed) => {
            eprintln!(
                "{}: {} geo records, {} B, roundtrip parses",
                out_path,
                parsed.len(),
                bytes.len()
            );
            Ok(bytes.len())
        }
        None => Err(format!("{out_path}: roundtrip parse void")),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let radar = match arg_value(&args, "--radar") {
        Some(v) => v,
        None => "sas".to_string(),
    };
    let start_date = match arg_value(&args, "--start") {
        Some(v) => v,
        None => "2026-08-20".to_string(),
    };
    let start_time = match arg_value(&args, "--start-time") {
        Some(v) => v,
        None => "18:00".to_string(),
    };
    let end_date = match arg_value(&args, "--end") {
        Some(v) => v,
        None => "2026-08-20".to_string(),
    };
    let end_time = match arg_value(&args, "--end-time") {
        Some(v) => v,
        None => "20:00".to_string(),
    };
    let beam = match arg_value(&args, "--beam") {
        Some(v) => v,
        None => "all".to_string(),
    };
    let out_path = arg_value(&args, "--out");
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let usage = "usage: superdarn_compiler --radar <code> --start <YYYY-MM-DD> [--start-time HH:MM] --end <YYYY-MM-DD> [--end-time HH:MM] [--beam all|0..15] --out <superdarn_fitacf.bin> [--ci-mode]";
    let out_path = match out_path {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };

    let start_compact: String = start_date.chars().filter(|c| c.is_ascii_digit()).collect();

    let hdw = match fetch_hdw(&radar) {
        Some(b) => b,
        None => {
            eprintln!(
                "superdarn: hdw.dat.{} stayed unreadable — position stays absent",
                radar
            );
            std::process::exit(1);
        }
    };
    let hdw_text = String::from_utf8_lossy(&hdw).into_owned();
    let site = match hdw_site(&hdw_text, &start_compact) {
        Some(s) => s,
        None => {
            eprintln!(
                "superdarn: hdw.dat.{} carries no site geometry — position stays absent",
                radar
            );
            std::process::exit(1);
        }
    };
    eprintln!(
        "superdarn: {} site lat {:.4} lon {:.4} boresight {:.2}",
        radar, site.lat_deg, site.lon_deg, site.boresight_deg
    );

    let lsk = match omegaflow::archivar::embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("superdarn: the embedded naif0012 table parses void — TDB stays unread");
            std::process::exit(1);
        }
    };

    let body = build_body(
        &radar,
        &start_date,
        &start_time,
        &end_date,
        &end_time,
        &beam,
    );
    let records = match gather(&body, &site, &lsk) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("superdarn_compiler: {e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = write_asset(&records, &out_path) {
        eprintln!("superdarn_compiler: {e}");
        std::process::exit(1);
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bracket_list_reads_gate_and_velocity_vectors() {
        assert_eq!(
            parse_bracket_list("[ 0  1 16 25 26 61]"),
            vec![0.0, 1.0, 16.0, 25.0, 26.0, 61.0]
        );
        assert_eq!(
            parse_bracket_list("[  13.868572   12.583638 -412.59323  2062.8972  ]"),
            vec![13.868572, 12.583638, -412.59323, 2062.8972]
        );
        assert!(parse_bracket_list("[]").is_empty());
        assert!(parse_bracket_list("[nan inf]").is_empty());
    }

    #[test]
    fn slant_to_ground_maps_a_high_elevation_echo_to_a_short_arc() {
        let ground = slant_to_ground_km(180.0, 34.82).expect("finite echo");
        assert!((ground - 147.0).abs() < 2.0, "got {ground}");
        let ground_horizon = slant_to_ground_km(180.0, 0.0).expect("finite echo");
        assert!((ground_horizon - 180.0).abs() < 2.0);
        assert!(slant_to_ground_km(-1.0, 20.0).is_none());
        assert!(slant_to_ground_km(180.0, f64::NAN).is_none());
    }

    #[test]
    fn destination_walks_north_and_east_on_the_sphere() {
        let (lat, lon) = destination(0.0, 0.0, 0.0, 111.195).expect("finite");
        assert!((lat - 1.0).abs() < 0.01, "north arc lat {lat}");
        assert!(lon.abs() < 0.01);
        let (lat, lon) = destination(0.0, 0.0, 90.0, 111.195).expect("finite");
        assert!(lat.abs() < 0.01);
        assert!((lon - 1.0).abs() < 0.01, "east arc lon {lon}");
        assert!(destination(f64::NAN, 0.0, 0.0, 10.0).is_none());
    }

    #[test]
    fn civil_unix_carries_epoch_and_microseconds() {
        assert_eq!(civil_unix(1970, 1, 1, 0, 0, 0, 0), Some(0.0));
        assert_eq!(civil_unix(1970, 1, 1, 0, 0, 1, 0), Some(1.0));
        assert_eq!(civil_unix(1970, 1, 1, 0, 0, 0, 500000), Some(0.5));
    }

    #[test]
    fn build_body_carries_the_measured_ascii_contract() {
        let body = build_body("sas", "2026-08-20", "18:00", "2026-08-20", "20:00", "all");
        assert!(body.contains("\"radar\":\"sas\""));
        assert!(body.contains("\"startDate\":\"2026-08-20\""));
        assert!(body.contains("\"beam\":\"all\""));
        assert!(body.contains("\"ftype\":\"json\""));
        assert!(body.contains("\"fields\":[\"v\",\"bmazm\",\"frang\",\"rsep\",\"slist\",\"elv\"]"));
    }

    #[test]
    fn hdw_site_reads_the_saskatoon_geometry() {
        let text = "# header\n   5  1 19930929 00:00:00  52.16    -106.53     494.0   23.1  0.00  3.24\n# EOF\n";
        let site = hdw_site(text, "20260820").expect("site");
        assert!((site.lat_deg - 52.16).abs() < 1e-6);
        assert!((site.lon_deg + 106.53).abs() < 1e-6);
        assert!((site.boresight_deg - 23.1).abs() < 1e-6);
    }

    #[test]
    fn hdw_site_picks_the_config_valid_at_the_start_date() {
        let text = "# a\n   5  1 19930929 00:00:00  52.16 -106.53 494.0 23.1 0.00 3.24\n   5  1 20220201 18:00:00  52.16 -106.53 494.0 23.1 0.00 3.24\n# EOF\n";
        let site = hdw_site(text, "20210101").expect("site");
        assert!((site.boresight_deg - 23.1).abs() < 1e-6);
    }
}
