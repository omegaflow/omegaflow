use omegaflow::archivar::membrane::NAIF_LSK_EMBEDDED;
use omegaflow::archivar::tle::{self, RADIUS_EARTH_KM};
use omegaflow::lsk::parse as parse_lsk;
use omegaflow::wind_orbit::{self, write_bin};

const DEFAULT_L1: &str = "1 26998U 01055B   26287.22414614  .00001152  00000+0  10753-3 0  9991";
const DEFAULT_L2: &str = "2 26998  74.0694 355.2967 0000920 228.8676 131.2429 14.95511591348618";
const POD_S: f64 = 86400.0;
const DEFAULT_CADENCE_S: f64 = 60.0;
const DEFAULT_OUT: &str = "data/timed_orbit.bin";
const UNIX_JD: f64 = 2440587.5;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn tle_lines(path: &str) -> Option<(String, String)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut lines = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("#"));
    let l1 = lines.next()?.to_string();
    let l2 = lines.next()?.to_string();
    Some((l1, l2))
}

fn moment(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (l1, l2) = match arg_value(&args, "--tle-file") {
        Some(path) => match tle_lines(&path) {
            Some(pair) => pair,
            None => {
                eprintln!("--tle-file {path} carries no element set, unwritten");
                std::process::exit(1);
            }
        },
        None => (DEFAULT_L1.to_string(), DEFAULT_L2.to_string()),
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let cadence = match arg_value(&args, "--cadence") {
        Some(v) => match v.parse::<f64>() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("--cadence {v} carries no interval, the worldline stays unwritten");
                std::process::exit(1);
            }
        },
        None => DEFAULT_CADENCE_S,
    };
    if !(cadence.is_finite() && cadence > 0.0) {
        eprintln!("--cadence {cadence} is no positive interval, the worldline stays unwritten");
        std::process::exit(1);
    }

    let tle = match tle::parse(&l1, &l2) {
        Some(t) => t,
        None => {
            eprintln!("the element set does not parse as a TLE, the worldline stays unwritten");
            std::process::exit(1);
        }
    };
    let sat = match tle::sgp4_init(&tle) {
        Some(s) => s,
        None => {
            eprintln!("outside the near-earth SGP4 branch, unwritten");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(NAIF_LSK_EMBEDDED) {
        Some(l) => l,
        None => {
            eprintln!("the embedded leap-second table parses void, the TDB axis stays unread");
            std::process::exit(1);
        }
    };

    let epoch_unix = (tle.epoch_jd - UNIX_JD) * 86400.0;
    let (r0, v0) = match tle::sgp4(&sat, 0.0) {
        Some(state) => state,
        None => {
            eprintln!("the epoch state stays off the propagator, the worldline stays unwritten");
            std::process::exit(1);
        }
    };
    let alt0 = moment(r0) - RADIUS_EARTH_KM;
    let speed0 = moment(v0);
    eprintln!(
        "TIMED {} self-check: epoch altitude {alt0:.1} km, speed {speed0:.3} km/s",
        tle.satnum
    );
    if !(400.0..=900.0).contains(&alt0) || !(7.0..=8.0).contains(&speed0) {
        eprintln!("outside 400-900 km and 7.0-8.0 km/s, unwritten");
        std::process::exit(1);
    }

    let count = (POD_S / cadence).floor() as usize + 1;
    let mut records: Vec<(f64, [f64; 3], [f64; 3])> = Vec::with_capacity(count);
    for i in 0..count {
        let tsince_min = i as f64 * cadence / 60.0;
        let (r, v) = match tle::sgp4(&sat, tsince_min) {
            Some(state) => state,
            None => {
                eprintln!("sample {i} carries no state, the worldline stays unwritten");
                std::process::exit(1);
            }
        };
        let unix = epoch_unix + i as f64 * cadence;
        let tdb = match lsk.unix_to_tdb(unix) {
            Some(t) => t,
            None => {
                eprintln!("sample {i} carries no TDB epoch, the worldline stays unwritten");
                std::process::exit(1);
            }
        };
        let jd = unix / 86400.0 + UNIX_JD;
        let (ri, vi) = match tle::teme_to_icrs(r, v, jd) {
            Some(pair) => pair,
            None => {
                eprintln!("sample {i} carries no ICRS state, the worldline stays unwritten");
                std::process::exit(1);
            }
        };
        records.push((
            tdb,
            [ri[0] * 1000.0, ri[1] * 1000.0, ri[2] * 1000.0],
            [vi[0] * 1000.0, vi[1] * 1000.0, vi[2] * 1000.0],
        ));
    }

    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void, the worldline stays unwritten");
        std::process::exit(1);
    }
    match wind_orbit::parse_bin(&bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!("{out}: {} records, roundtrip parses", parsed.len());
        }
        _ => {
            eprintln!("{out}: roundtrip parse carries a drift, the bin stays unverified");
            std::process::exit(1);
        }
    }
    eprintln!(
        "window {:.1} h at {:.0} s cadence, {} records, ICRS m and m/s",
        POD_S / 3600.0,
        cadence,
        records.len()
    );
}
