use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_HFR_U, GeoRec, MAGIC_HFR, parse_bin, write_bin};
use omegaflow::archivar::{embedded_lsk, hfrnet_rtv};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "coastwatch.pfeg.noaa.gov";
const BASE: &str = "https://coastwatch.pfeg.noaa.gov/erddap/griddap";
const DEFAULT_DATASET: &str = "ucsdHfrW1_Lon0360";
const DEFAULT_OUT: &str = "data/coastwatch.pfeg.noaa.gov/hfrnet_rtv.bin";
const LAT_MIN: f64 = 30.25;
const LAT_MAX: f64 = 49.99204;
const LON_MIN: f64 = 229.64;
const LON_MAX: f64 = 244.19443;

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
        "[(last)]".to_string()
    } else {
        format!("[({time})]")
    };
    let lat = format!("[({lat_min}):{stride}:({lat_max})]");
    let lon = format!("[({lon_min}):{stride}:({lon_max})]");
    format!("{BASE}/{dataset}.csv?water_u{t}{lat}{lon},water_v{t}{lat}{lon}")
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
    let bytes = match fetch_raw_bytes(&url) {
        Some(b) => b,
        None => {
            eprintln!("{url}: fetch void");
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&bytes);
    let samples = match hfrnet_rtv::parse_csv(&text, &lsk) {
        Some(s) => s,
        None => {
            eprintln!(
                "{dataset} {time}: {} B carry no measured water_u/water_v cell — the bin stays unwritten (0 honored)",
                bytes.len()
            );
            std::process::exit(1);
        }
    };
    let mut records: Vec<GeoRec> = hfrnet_rtv::to_geo_rows(&samples);
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
