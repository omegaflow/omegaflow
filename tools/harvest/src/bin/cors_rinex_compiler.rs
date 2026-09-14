use omegaflow::archivar::cors::{
    CorsRecord, PRES_POSITION, pack_station, parse_bin, satellite_of, station_of, write_bin,
};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::rinex::{
    RinexFileType, ecef_to_geodetic, parse_rinex_header, parse_rinex_nav_gps, parse_rinex_nav_gps3,
    parse_rinex_obs,
};
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;
use omegaflow::lsk::parse as parse_lsk;
use std::collections::BTreeSet;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "noaa-cors-pds.s3.amazonaws.com";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn decode_body(bytes: Vec<u8>) -> Option<String> {
    let raw = if bytes.starts_with(&[0x1f, 0x8b]) {
        gunzip(&bytes)?
    } else {
        bytes
    };
    Some(String::from_utf8_lossy(&raw).into_owned())
}

fn hatanaka_marker(text: &str) -> bool {
    text.lines()
        .take(40)
        .any(|l| l.contains("CRINEX") || l.contains("COMPACT RINEX"))
}

fn read_rinex(url: Option<&str>, input: Option<&str>) -> Result<String, String> {
    if let Some(url) = url {
        let bytes =
            fetch_raw_bytes(url, 86400).ok_or_else(|| format!("{url}: fetch returned void"))?;
        decode_body(bytes).ok_or_else(|| format!("{url}: gunzip returned void"))
    } else if let Some(path) = input {
        let bytes = std::fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
        decode_body(bytes).ok_or_else(|| format!("{path}: gunzip returned void"))
    } else {
        Err("--url <rinex-url> or --input <rinex-file> required — refused".into())
    }
}

fn sat_bytes(sat: &str) -> [u8; 3] {
    let b = sat.as_bytes();
    let mut out = [b' '; 3];
    for (i, c) in b.iter().take(3).enumerate() {
        out[i] = *c;
    }
    out
}

fn gps_sat(prn: u32) -> [u8; 3] {
    let s = format!("G{prn:02}");
    sat_bytes(&s)
}

fn run(args: &[String]) -> Result<(), String> {
    let url = arg_value(args, "--url");
    let input = arg_value(args, "--input");
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("--out <path>: the asset path is never silent — refused".into());
    };
    let Some(lsk_path) = arg_value(args, "--lsk") else {
        return Err("--lsk <naif0012.tls>: the TDB conversion stays void — refused".into());
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let lsk = std::fs::read_to_string(&lsk_path)
        .map_err(|e| format!("read {lsk_path} returned void: {e}"))
        .and_then(|t| {
            parse_lsk(&t).ok_or_else(|| format!("{lsk_path}: leap seconds stay unread"))
        })?;

    let text = read_rinex(url.as_deref(), input.as_deref())?;
    let header = parse_rinex_header(&text)
        .ok_or_else(|| "RINEX header stays unread — the asset stays unwritten".to_string())?;
    let position = header
        .approx_pos_xyz
        .and_then(|(x, y, z)| ecef_to_geodetic(x, y, z));
    let station = pack_station(&header.marker_name);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(f64::NAN);

    let mut records: Vec<CorsRecord> = Vec::new();
    let mut skipped_future = 0u64;
    let mut skipped_lsk = 0u64;
    let mut sats = BTreeSet::new();
    let mut obs_types = BTreeSet::new();
    let mut min_epoch = f64::INFINITY;
    let mut max_epoch = f64::NEG_INFINITY;

    match header.file_type {
        RinexFileType::Observation => {
            let n_obs = header.obs_types.len();
            for e in parse_rinex_obs(&text, n_obs) {
                if e.epoch_unix > now {
                    skipped_future += 1;
                    continue;
                }
                let Some(epoch) = lsk.unix_to_tdb(e.epoch_unix) else {
                    skipped_lsk += 1;
                    continue;
                };
                for sat in &e.sats {
                    let sat_id = sat_bytes(&sat.sat);
                    for (i, v) in sat.values.iter().enumerate() {
                        let Some(value) = v else { continue };
                        let Some(obs) = header.obs_types.get(i) else {
                            continue;
                        };
                        let obs_b = obs.as_bytes();
                        let obs = [
                            *obs_b.first().unwrap_or(&b' '),
                            *obs_b.get(1).unwrap_or(&b' '),
                        ];
                        let present = if position.is_some() { PRES_POSITION } else { 0 };
                        let (lat, lon, alt) = position.unwrap_or((0.0, 0.0, 0.0));
                        records.push(CorsRecord {
                            epoch,
                            lat,
                            lon,
                            alt,
                            freq: 0.0,
                            bin_width: 0.0,
                            value: *value,
                            sat: sat_id,
                            obs,
                            station,
                            present,
                        });
                        sats.insert(sat_id);
                        obs_types.insert(obs);
                        min_epoch = min_epoch.min(epoch);
                        max_epoch = max_epoch.max(epoch);
                    }
                }
            }
        }
        RinexFileType::Navigation => {
            let navs = if header.version >= 3.0 {
                parse_rinex_nav_gps3(&text)
            } else {
                parse_rinex_nav_gps(&text)
            };
            for n in navs {
                if n.epoch_unix > now {
                    skipped_future += 1;
                    continue;
                }
                let Some(epoch) = lsk.unix_to_tdb(n.epoch_unix) else {
                    skipped_lsk += 1;
                    continue;
                };
                let sat_id = gps_sat(n.prn);
                let present = if position.is_some() { PRES_POSITION } else { 0 };
                let (lat, lon, alt) = position.unwrap_or((0.0, 0.0, 0.0));
                records.push(CorsRecord {
                    epoch,
                    lat,
                    lon,
                    alt,
                    freq: 0.0,
                    bin_width: 0.0,
                    value: n.a0,
                    sat: sat_id,
                    obs: *b"a0",
                    station,
                    present,
                });
                sats.insert(sat_id);
                obs_types.insert(*b"a0");
                min_epoch = min_epoch.min(epoch);
                max_epoch = max_epoch.max(epoch);
            }
        }
        RinexFileType::Unknown => {
            if hatanaka_marker(&text) {
                return Err("Hatanaka-compressed RINEX (.d) — decompression stays unread (named parser gap), the asset stays unwritten".into());
            }
            return Err("RINEX file type reads unknown — the asset stays unwritten".into());
        }
    }

    if records.is_empty() {
        return Err("no measured observation — the asset stays unwritten (0 honored)".into());
    }

    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out_path, &bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;

    let parsed =
        parse_bin(&bytes).ok_or_else(|| format!("{out_path}: roundtrip parse returned void"))?;
    if parsed.len() != records.len() {
        return Err(format!(
            "{out_path}: {} rows read back, {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        ));
    }

    let station_name = station_of(station);
    let obs_list: Vec<String> = obs_types
        .iter()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .collect();
    let sat_list: Vec<String> = sats.iter().map(|s| satellite_of(*s)).collect();
    eprintln!(
        "{out_path}: {} {} records, station {} ({}), {} satellites, obs {}",
        records.len(),
        match header.file_type {
            RinexFileType::Observation => "observation",
            RinexFileType::Navigation => "navigation",
            RinexFileType::Unknown => "unknown",
        },
        station_name,
        if position.is_some() {
            "position measured"
        } else {
            "position absent"
        },
        sat_list.len(),
        obs_list.join("/")
    );
    eprintln!(
        "epoch span TDB {:.3} .. {:.3} s, satellites {}, skipped {} future, {} without leap-second",
        min_epoch,
        max_epoch,
        sat_list.join(","),
        skipped_future,
        skipped_lsk
    );

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("cors_rinex_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hatanaka_marker_names_the_compact_gap() {
        let plain =
            "     2.11           OBSERVATION DATA    G (GPS)             RINEX VERSION / TYPE\n";
        assert!(!hatanaka_marker(plain));
        let compact = "     2.11           COMPACT RINEX FORMAT                    RINEX VERSION / TYPE\n\
CRINEX VERS   3.02  COMPACT RINEX FORMAT                    CRINEX VERS   / TYPE\n";
        assert!(hatanaka_marker(compact));
    }
}
