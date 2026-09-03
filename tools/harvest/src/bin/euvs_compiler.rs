use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::euvs::{parse_bin, write_bin, COMP_LYA1216};
use omegaflow::cdn::upload_asset;
use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use std::process::Command;

const BASE: &str =
    "https://www.ncei.noaa.gov/data/goes-space-environment-monitor/access/science/euvs";
const SATS: [&str; 2] = ["goes14", "goes15"];
const EPOCH_UNIX: f64 = 946728000.0;
const FILL: f64 = -9999.0;
const VALID_MIN: f64 = 1e-9;
const INDEX_MARKER: &str = "sci_geuv-l2-avg1d_";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!(
            "fetch http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn index_files(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(pos) = rest.find(INDEX_MARKER) {
        let tail = &rest[pos..];
        let name: String = tail
            .chars()
            .take_while(|&c| c != '"' && c != '\'' && c != '<')
            .collect();
        if name.ends_with(".nc") && !out.contains(&name) {
            out.push(name);
        }
        rest = &tail[1..];
    }
    out
}

fn parse_avg1d(path: &str, records: &mut Vec<(f64, f64, u32)>) -> Option<(usize, usize)> {
    let bytes = std::fs::read(path).ok()?;
    let file = Hdf5File::parse(&bytes).ok()?;
    let time_raw = file.read_dataset("time").ok()?;
    let irr_1216 = file.read_dataset("irr_1216_1nm").ok()?;
    let flag_1216 = file.read_dataset("irr_1216_flag").ok()?;
    let au_factor = file.read_dataset("au_factor").ok()?;
    let n = time_raw.len() / 8;
    if irr_1216.len() != n * 4 || flag_1216.len() != n || au_factor.len() != n * 4 {
        eprintln!("{}: dataset shapes carry no common row count", path);
        return None;
    }
    let lsk = embedded_lsk()?;
    let mut kept = 0usize;
    for i in 0..n {
        let t = decode_f64(&time_raw, i * 8, Endian::Le)?;
        if t == FILL || !t.is_finite() {
            continue;
        }
        let t_unix = t + EPOCH_UNIX;
        let au = decode_f32(&au_factor, i * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
        if !au.is_finite() || au <= 0.0 {
            continue;
        }
        if flag_1216[i] != 0 {
            continue;
        }
        let v = decode_f32(&irr_1216, i * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
        if !v.is_finite() || v == FILL || v < VALID_MIN {
            continue;
        }
        let Some(tdb) = lsk.unix_to_tdb(t_unix) else {
            continue;
        };
        records.push((tdb, v * au, COMP_LYA1216));
        kept += 1;
    }
    Some((n, kept))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "goes_euvs.bin".to_string());
    let cache_dir = arg_value(&args, "--cache-dir").unwrap_or_else(|| {
        omegaflow::archivar::cache_root()
            .join("omegaflow_goes_euvs_cache")
            .to_string_lossy()
            .into_owned()
    });
    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("{}: cache dir stays uncreatable", cache_dir);
        std::process::exit(1);
    }
    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    if let Some(path) = arg_value(&args, "--file") {
        match parse_avg1d(&path, &mut records) {
            Some((n, k)) => eprintln!("{}: {} rows, lya1216 {} kept", path, n, k),
            None => {
                eprintln!("{}: parses void — no records", path);
                std::process::exit(1);
            }
        }
    } else {
        for sat in SATS {
            let index_url = format!("{BASE}/{sat}/geuv-l2-avg1d/");
            let Some(html) = fetch(&index_url) else {
                eprintln!("index {sat}: fetch void — the satellite stays unharvested");
                continue;
            };
            for name in index_files(&String::from_utf8_lossy(&html)) {
                let cache_path = format!("{}/{}_{}", cache_dir, sat, name);
                if std::fs::metadata(&cache_path).is_err() {
                    let url = format!("{BASE}/{sat}/geuv-l2-avg1d/{name}");
                    let Some(bytes) = fetch(&url) else {
                        eprintln!("file {name}: fetch void — the day stays unharvested");
                        continue;
                    };
                    if std::fs::write(&cache_path, &bytes).is_err() {
                        eprintln!("file {name}: cache write void");
                        continue;
                    }
                }
                match parse_avg1d(&cache_path, &mut records) {
                    Some((n, k)) => eprintln!("{}: {} rows, lya1216 {} kept", name, n, k),
                    None => eprintln!("{}: parses void — the day stays unharvested", name),
                }
            }
        }
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0));
    records.dedup_by(|a, b| a.0 == b.0 && a.2 == b.2);
    if records.is_empty() {
        eprintln!("{}: no records — the bin stays unwritten (0 honored)", out);
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            let (t0, _, _) = parsed[0];
            let (t1, _, _) = parsed[parsed.len() - 1];
            eprintln!(
                "{}: {} records ({} B), epoch_tdb {:.0}..{:.0}, roundtrip parses",
                out,
                parsed.len(),
                bytes.len(),
                t0,
                t1
            );
        }
        None => {
            eprintln!(
                "{}: roundtrip parse void — the series stays unverified",
                out
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
