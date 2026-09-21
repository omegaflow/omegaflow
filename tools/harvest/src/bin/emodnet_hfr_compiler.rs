use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_HFR_U, GeoRec, MAGIC_HFR, parse_bin, write_bin};
use omegaflow::archivar::{embedded_lsk, emodnet_hfr};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "erddap.emodnet-physics.eu";
const BASE: &str = "https://erddap.emodnet-physics.eu/erddap/griddap";
const DEFAULT_DATASET: &str = "EUHFR_NRTcurrent_HFR-NAdr-Total";
const DEFAULT_OUT: &str = "data/erddap.emodnet-physics.eu/emodnet_hfr_nadr.bin";
const LAT_MIN: f64 = 45.52685;
const LAT_MAX: f64 = 45.78333;
const LON_MIN: f64 = 13.375;
const LON_MAX: f64 = 13.78056;
const DEPTH: f64 = 0.0;
const FETCH_TTL: u64 = 86400;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn axis(args: &[String], name: &str, fallback: f64) -> f64 {
    match arg_value(args, name) {
        Some(v) => match v.parse::<f64>() {
            Ok(x) if x.is_finite() => x,
            _ => {
                eprintln!("{name} {v} carries no finite degree value");
                std::process::exit(1);
            }
        },
        None => fallback,
    }
}

fn enc(s: &str) -> String {
    s.replace('[', "%5B").replace(']', "%5D")
}

fn grid_url(
    dataset: &str,
    time: &str,
    stride: u64,
    lat_min: f64,
    lat_max: f64,
    lon_min: f64,
    lon_max: f64,
) -> String {
    let t = if time == "last" {
        enc("[(last)]")
    } else {
        enc(&format!("[({time})]"))
    };
    let depth = enc(&format!("[({DEPTH})]"));
    let lat = enc(&format!("[({lat_min}):{stride}:({lat_max})]"));
    let lon = enc(&format!("[({lon_min}):{stride}:({lon_max})]"));
    format!("{BASE}/{dataset}.csv?EWCT{t}{depth}{lat}{lon},NSCT{t}{depth}{lat}{lon}")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let dataset = match arg_value(&args, "--dataset") {
        Some(v) => v,
        None => DEFAULT_DATASET.to_string(),
    };
    let time = match arg_value(&args, "--time") {
        Some(v) => v,
        None => "last".to_string(),
    };
    let stride: u64 = match arg_value(&args, "--stride") {
        Some(v) => match v.parse::<u64>() {
            Ok(n) if n >= 1 => n,
            _ => {
                eprintln!("--stride {v} carries no positive step");
                std::process::exit(1);
            }
        },
        None => 1,
    };
    let lat_min = axis(&args, "--lat-min", LAT_MIN);
    let lat_max = axis(&args, "--lat-max", LAT_MAX);
    let lon_min = axis(&args, "--lon-min", LON_MIN);
    let lon_max = axis(&args, "--lon-max", LON_MAX);
    if !(lat_min < lat_max) || !(lon_min < lon_max) {
        eprintln!("the bounding box carries no positive extent");
        std::process::exit(1);
    }
    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("embedded naif0012 parses void — the epoch conversion stays unread");
            std::process::exit(1);
        }
    };
    let url = grid_url(&dataset, &time, stride, lat_min, lat_max, lon_min, lon_max);
    let bytes = match fetch_raw_bytes(&url, FETCH_TTL) {
        Some(b) => b,
        None => {
            eprintln!("{url}: fetch void");
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&bytes);
    let samples = match emodnet_hfr::parse_csv(&text, &lsk) {
        Some(s) => s,
        None => {
            eprintln!(
                "{dataset} {time}: {} B carry no measured EWCT/NSCT cell — the bin stays unwritten (0 honored)",
                bytes.len()
            );
            std::process::exit(1);
        }
    };
    let mut records: Vec<GeoRec> = emodnet_hfr::to_geo_rows(&samples);
    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.comp.cmp(&b.comp))
            .then(a.lat.total_cmp(&b.lat))
            .then(a.lon.total_cmp(&b.lon))
    });
    let u_count = records.iter().filter(|r| r.comp == COMP_HFR_U).count();
    let v_count = records.len() - u_count;
    if records.is_empty() {
        eprintln!("{dataset} {time}: no measured component — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bin = write_bin(MAGIC_HFR, &records);
    if let Some(parent) = std::path::Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_HFR, &bin) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} cells, {u_count} eastward + {v_count} northward records ({} B, dataset {dataset}, time {time}, stride {stride}, box {lat_min}..{lat_max} lat / {lon_min}..{lon_max} lon), roundtrip parses",
                samples.len(),
                bin.len()
            );
        }
        _ => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
