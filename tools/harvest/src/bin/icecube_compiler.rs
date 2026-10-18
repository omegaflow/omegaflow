use omegaflow::cdn::upload_release;
use omegaflow::skydirection::{SkyDirection, write_bin};
use std::process::Command;

const URL: &str = "https://dataverse.harvard.edu/api/access/datafile/13597936";
const TAG: &str = "dataverse.harvard.edu";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let text = match arg_value(&args, "--in") {
        Some(p) => match std::fs::read_to_string(&p) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{} reads void: {e}", p);
                std::process::exit(1);
            }
        },
        None => match fetch(URL) {
            Some(t) => t,
            None => {
                eprintln!("{URL} carries void");
                std::process::exit(1);
            }
        },
    };
    let mut dirs: Vec<SkyDirection> = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let t = t.trim_matches('"');
        let f: Vec<&str> = t.split_whitespace().collect();
        if f.len() < 10 {
            continue;
        }
        let (Some(run), Some(event)) = (f[0].parse::<u64>().ok(), f[1].parse::<u64>().ok()) else {
            continue;
        };
        let (Some(ang), Some(ra), Some(dec)) = (
            f[5].parse::<f64>().ok(),
            f[6].parse::<f64>().ok(),
            f[7].parse::<f64>().ok(),
        ) else {
            continue;
        };
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) || !(ang > 0.0) {
            continue;
        }
        dirs.push(SkyDirection {
            name: format!("IC{run}_{event}"),
            ra_deg: ra,
            dec_deg: dec,
            sigma_arcsec: Some(ang * 3600.0),
            bands: Vec::new(),
            flux_bands: Vec::new(),
            distance: None,
            redshift: None,
        });
    }
    if dirs.is_empty() {
        eprintln!("icecube_compiler: no events — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let Some(bytes) = write_bin(&dirs) else {
        eprintln!("icecube_compiler: write_bin returned void");
        std::process::exit(1);
    };
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("icecube_compiler: write {out} returned void");
        std::process::exit(1);
    }
    eprintln!("icecube_compiler: wrote {out} ({} events)", dirs.len());
    if ci_mode && !upload_release(TAG, &out) {
        eprintln!("icecube_compiler: {out} did not reach the CDN");
        std::process::exit(1);
    }
}
