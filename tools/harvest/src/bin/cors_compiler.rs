use omegaflow::archivar::cors::{
    CorsRecord, PRES_POSITION, pack_station, parse_bin, satellite_of, station_of, write_bin,
};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::rinex::{
    RinexFileType, SBF_SYNC, crx2rnx, ecef_to_geodetic, is_hatanaka, parse_rinex_header,
    parse_rinex_obs,
};
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip;

const NETLOC: &str = "noaa-cors-pds.s3.amazonaws.com";
const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];

struct Summary {
    station_name: String,
    position: Option<(f64, f64, f64)>,
    obs_type: String,
    sats: Vec<String>,
    skipped: usize,
    min_epoch: f64,
    max_epoch: f64,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn decode_bytes(bytes: &[u8]) -> Option<String> {
    let raw = if bytes.starts_with(&GZIP_MAGIC) {
        gunzip(bytes)?
    } else {
        bytes.to_vec()
    };
    Some(String::from_utf8_lossy(&raw).into_owned())
}

fn sat_bytes(sat: &str) -> [u8; 3] {
    let b = sat.as_bytes();
    let mut out = [b' '; 3];
    for (i, c) in b.iter().take(3).enumerate() {
        out[i] = *c;
    }
    out
}

fn obs_bytes(obs: &str) -> [u8; 2] {
    let b = obs.as_bytes();
    [*b.first().unwrap_or(&b' '), *b.get(1).unwrap_or(&b' ')]
}

fn plausible_value(obs: &[u8; 2], v: f64) -> bool {
    if !v.is_finite() {
        return false;
    }
    match obs[0] {
        b'C' | b'P' => v > 0.0,
        _ => true,
    }
}

fn collect(text: &str) -> Result<(Vec<CorsRecord>, Summary), String> {
    let decoded;
    let text = if is_hatanaka(text) {
        match crx2rnx(text) {
            Some(d) => {
                decoded = d;
                decoded.as_str()
            }
            None => {
                return Err(
                    "Hatanaka (CRINEX) decode returned void — the asset stays unwritten"
                        .to_string(),
                );
            }
        }
    } else {
        text
    };
    let header = parse_rinex_header(text)
        .ok_or_else(|| "RINEX header stays unread — the asset stays unwritten".to_string())?;
    if header.file_type != RinexFileType::Observation {
        return Err(format!(
            "{} is a navigation RINEX, not an observation file — the asset stays unwritten",
            header.marker_name
        ));
    }
    let position = header
        .approx_pos_xyz
        .and_then(|(x, y, z)| ecef_to_geodetic(x, y, z));
    let station = pack_station(&header.marker_name);
    let n_obs = header.obs_types.len();
    let Some(obs_type) = header.obs_types.first().cloned() else {
        return Err(
            "no observation type listed in the header — the asset stays unwritten (0 honored)"
                .to_string(),
        );
    };
    let obs = obs_bytes(&obs_type);

    let mut records: Vec<CorsRecord> = Vec::new();
    let mut sats: Vec<String> = Vec::new();
    let mut skipped = 0usize;
    let mut min_epoch = f64::INFINITY;
    let mut max_epoch = f64::NEG_INFINITY;

    for e in parse_rinex_obs(text, n_obs) {
        for sat in &e.sats {
            let Some(value) = sat.values.first().copied().flatten() else {
                skipped += 1;
                continue;
            };
            if !plausible_value(&obs, value) {
                skipped += 1;
                continue;
            }
            let sat_id = sat_bytes(&sat.sat);
            let sat_name = satellite_of(sat_id);
            if !sats.iter().any(|s| *s == sat_name) {
                sats.push(sat_name);
            }
            let present = if position.is_some() { PRES_POSITION } else { 0 };
            let (lat, lon, alt) = position.unwrap_or((0.0, 0.0, 0.0));
            records.push(CorsRecord {
                epoch: e.epoch_unix,
                lat,
                lon,
                alt,
                freq: 0.0,
                bin_width: 0.0,
                value,
                sat: sat_id,
                obs,
                station,
                present,
            });
            min_epoch = min_epoch.min(e.epoch_unix);
            max_epoch = max_epoch.max(e.epoch_unix);
        }
    }

    if records.is_empty() {
        return Err(
            "no measured observation — every value is absent or implausible (0 honored)"
                .to_string(),
        );
    }

    Ok((
        records,
        Summary {
            station_name: station_of(station),
            position,
            obs_type,
            sats,
            skipped,
            min_epoch,
            max_epoch,
        },
    ))
}

fn report(summary: &Summary, records: &[CorsRecord]) {
    match summary.position {
        Some((lat, lon, alt)) => eprintln!(
            "station {}: position lat {:.6} deg lon {:.6} deg alt {:.3} m",
            summary.station_name, lat, lon, alt
        ),
        None => eprintln!(
            "station {}: position absent (no APPROX POSITION XYZ in the header)",
            summary.station_name
        ),
    }
    eprintln!(
        "obs type {} across {} satellites: {}",
        summary.obs_type,
        summary.sats.len(),
        summary.sats.join(",")
    );
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    for r in records {
        min = min.min(r.value);
        max = max.max(r.value);
        sum += r.value;
    }
    let mean = sum / records.len() as f64;
    eprintln!(
        "{} records, epoch span {:.3} .. {:.3} s, {} absent/implausible values skipped, value min {:.3} max {:.3} mean {:.3}",
        records.len(),
        summary.min_epoch,
        summary.max_epoch,
        summary.skipped,
        min,
        max,
        mean
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: cors_compiler --input <rinex> | --url <rinex-url> --out <path> [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };

    let (bytes, name) = match arg_value(&args, "--input") {
        Some(path) => {
            let leaf = path.rsplit('/').next().unwrap_or(&path).to_string();
            match std::fs::read(&path) {
                Ok(b) => (b, leaf),
                Err(e) => {
                    eprintln!("{path}: read returned void ({e})");
                    std::process::exit(1);
                }
            }
        }
        None => match arg_value(&args, "--url") {
            Some(url) => {
                let leaf = url.rsplit('/').next().unwrap_or(&url).to_string();
                match fetch_raw_bytes(&url) {
                    Some(b) => (b, leaf),
                    None => {
                        eprintln!("{url}: fetch returned void");
                        std::process::exit(1);
                    }
                }
            }
            None => {
                eprintln!("{usage}");
                std::process::exit(1);
            }
        },
    };

    if bytes.len() >= 2 && bytes[0] == SBF_SYNC[0] && bytes[1] == SBF_SYNC[1] {
        eprintln!(
            "{name}: Septentrio SBF stream — the SBF→RINEX block decode is pending (block layout unverified against a real file); the bin stays unwritten"
        );
        std::process::exit(1);
    }

    let text = match decode_bytes(&bytes) {
        Some(t) => t,
        None => {
            eprintln!("{name}: gzip stays unread — the bin stays unwritten");
            std::process::exit(1);
        }
    };

    let (records, summary) = match collect(&text) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("cors_compiler: {e}");
            std::process::exit(1);
        }
    };
    report(&summary, &records);

    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if let Err(e) = std::fs::write(&out_path, &bytes) {
        eprintln!("cors_compiler: write {out_path} returned void: {e}");
        std::process::exit(1);
    }

    match parse_bin(&bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{} records, {} B -> {out_path}, roundtrip parses",
                parsed.len(),
                bytes.len()
            );
        }
        _ => {
            eprintln!("{out_path}: roundtrip parse returned void — the bin stays unverified");
            std::process::exit(1);
        }
    }

    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("upload: {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs_file() -> String {
        let mut s = String::new();
        s.push_str(
            "     2.11           OBSERVATION DATA    G (GPS)             RINEX VERSION / TYPE\n",
        );
        s.push_str(&format!("{:60}{:>20}\n", "1LSU", "MARKER NAME"));
        s.push_str(
            "  4100516.2851  -455185.4244   4404346.7061                  APPROX POSITION XYZ\n",
        );
        s.push_str(
            "        0.0000        0.0000        0.0000                  ANTENNA: DELTA H/E/N\n",
        );
        s.push_str(
            "     2    C1    L1                                          # / TYPES OF OBSERV\n",
        );
        s.push_str("    30.0000                                                  INTERVAL\n");
        s.push_str(
            "                                                            END OF HEADER       \n",
        );
        let epoch = format!(
            "{:>3}{:>3}{:>3}{:>3}{:>3}{:>11.7}{:>3}{:>3}G01G02",
            24i64, 1i64, 4i64, 0i64, 0i64, 0.0, 0i64, 2i64
        );
        s.push_str(&epoch);
        s.push('\n');
        let cell = |v: Option<f64>| -> String {
            match v {
                Some(x) => format!("{x:>14.3}  "),
                None => "                ".to_string(),
            }
        };
        s.push_str(&format!(
            "{}{}\n",
            cell(Some(21345678.123)),
            cell(Some(-12345678.123))
        ));
        s.push_str(&format!("{}{}\n", cell(None), cell(Some(-12345679.123))));
        s
    }

    #[test]
    fn plausible_gates_by_observation_kind() {
        let c1 = *b"C1";
        assert!(plausible_value(&c1, 21345678.123));
        assert!(!plausible_value(&c1, 0.0));
        assert!(!plausible_value(&c1, -1.0));
        assert!(!plausible_value(&c1, f64::NAN));
        let l1 = *b"L1";
        assert!(plausible_value(&l1, -12345678.123));
    }

    #[test]
    fn collect_builds_records_from_rinex_211_obs() {
        let (records, summary) = collect(&obs_file()).unwrap();
        assert_eq!(summary.station_name, "1LSU");
        assert_eq!(summary.obs_type, "C1");
        assert_eq!(summary.sats, vec!["G01"]);
        assert_eq!(
            summary.skipped, 1,
            "G02 carries no C1 — its value stays absent"
        );
        assert_eq!(records.len(), 1);
        let r = &records[0];
        assert_eq!(r.sat, *b"G01");
        assert_eq!(r.obs, *b"C1");
        assert_eq!(station_of(r.station), "1LSU");
        assert!((r.value - 21345678.123).abs() < 1e-3);
        assert_eq!(r.present & PRES_POSITION, PRES_POSITION);
        assert!(r.freq == 0.0 && r.bin_width == 0.0);
        assert_eq!(r.present & 0b110, 0);

        let expected_epoch = 1704326400.0;
        assert!((r.epoch - expected_epoch).abs() < 1e-3);

        let bytes = write_bin(&records);
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].sat, *b"G01");
        assert!((parsed[0].value - 21345678.123).abs() < 1e-3);
    }

    #[test]
    fn absent_position_carries_no_flag() {
        let mut text = String::new();
        text.push_str(
            "     2.11           OBSERVATION DATA    G (GPS)             RINEX VERSION / TYPE\n",
        );
        text.push_str(&format!("{:60}{:>20}\n", "1LSU", "MARKER NAME"));
        text.push_str(
            "     2    C1    L1                                          # / TYPES OF OBSERV\n",
        );
        text.push_str(
            "                                                            END OF HEADER       \n",
        );
        let epoch = format!(
            "{:>3}{:>3}{:>3}{:>3}{:>3}{:>11.7}{:>3}{:>3}G01",
            24i64, 1i64, 4i64, 0i64, 0i64, 0.0, 0i64, 1i64
        );
        text.push_str(&epoch);
        text.push('\n');
        text.push_str(&format!(
            "{:>14.3}  {:>14.3}  \n",
            21345678.123, -12345678.123
        ));
        let (records, summary) = collect(&text).unwrap();
        assert!(summary.position.is_none());
        assert_eq!(records[0].present & PRES_POSITION, 0);
    }
}
