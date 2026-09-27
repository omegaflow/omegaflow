use omegaflow::archivar::extract::{geo_series_component_name, geo_series_parse_bin};
use omegaflow::archivar::geo::magic_of;
use omegaflow::archivar::sha256::sha256_hex;
use std::process::Command;

const FORMAT: &str = "esacci_sst_l4_cdr3";
const DEFAULT_URL: &str = "https://github.com/omegaflow/sources/releases/download/data.ceda.ac.uk-eocis-sst/esacci_sst.bin";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("600")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!("asset {url}: curl exit {}", out.status);
        None
    }
}

fn finite_range(values: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut seen = false;
    for v in values {
        if v.is_finite() {
            seen = true;
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
    }
    if seen { Some((min, max)) } else { None }
}

fn print_range(label: &str, values: impl Iterator<Item = f64>) {
    match finite_range(values) {
        Some((min, max)) => println!("{label} {min} {max}"),
        None => println!("{label} absent"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let file = arg_value(&args, "--file");
    let url = match arg_value(&args, "--url") {
        Some(u) => u,
        None => DEFAULT_URL.to_string(),
    };

    let bytes = match &file {
        Some(path) => match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("asset {path}: {:?}", e.kind());
                std::process::exit(2);
            }
        },
        None => match fetch(&url) {
            Some(b) => b,
            None => std::process::exit(2),
        },
    };
    let source = match &file {
        Some(path) => path.clone(),
        None => url.clone(),
    };

    let magic = match magic_of(FORMAT) {
        Some(m) => m,
        None => std::process::exit(2),
    };
    if bytes.len() < 4 {
        eprintln!("asset {source}: {} bytes carry no magic", bytes.len());
        std::process::exit(2);
    }
    let actual: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    if actual != magic {
        eprintln!(
            "asset {source}: magic {:?} is not {FORMAT}",
            String::from_utf8_lossy(&actual)
        );
        std::process::exit(2);
    }

    let records = match geo_series_parse_bin(FORMAT, &bytes) {
        Some(r) => r,
        None => {
            eprintln!(
                "asset {source}: {} bytes do not parse as {FORMAT}",
                bytes.len()
            );
            std::process::exit(2);
        }
    };

    println!("bytes {}", bytes.len());
    println!("sha256 {}", sha256_hex(&bytes));
    println!("records {}", records.len());

    let mut comps: Vec<u32> = records.iter().map(|r| r.comp).collect();
    comps.sort_unstable();
    comps.dedup();
    for comp in comps {
        match geo_series_component_name(FORMAT, comp) {
            Some(name) => println!("component {name}"),
            None => println!("component comp {comp} unnamed"),
        }
    }

    print_range("t", records.iter().map(|r| r.t));
    print_range("lat", records.iter().map(|r| r.lat));
    print_range("lon", records.iter().map(|r| r.lon));

    let vals: Vec<f64> = records.iter().map(|r| r.val).collect();
    let finite: Vec<f64> = vals.iter().copied().filter(|v| v.is_finite()).collect();
    let nan = vals.iter().filter(|v| v.is_nan()).count();
    let infinite = vals.iter().filter(|v| v.is_infinite()).count();
    match finite_range(finite.iter().copied()) {
        Some((min, max)) => {
            let mean = finite.iter().sum::<f64>() / finite.len() as f64;
            println!(
                "val {min} {max} {mean} finite {}/{}",
                finite.len(),
                vals.len()
            );
        }
        None => println!("val absent finite 0/{}", vals.len()),
    }
    println!("nan {nan}");
    if infinite > 0 {
        println!("infinite {infinite}");
    }
}
