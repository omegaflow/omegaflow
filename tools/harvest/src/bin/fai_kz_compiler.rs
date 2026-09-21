use omegaflow::archivar::fai_kz::{
    OBSCORE_TABLE, ObsRecord, SELECT_COLUMNS, TAP_SYNC, parse_bin, parse_obscore_csv, write_bin,
};
use omegaflow::cdn::upload_release;
use std::process::Command;

const NETLOC: &str = "dachs.fai.kz";
const DEFAULT_OUT: &str = "data/dachs.fai.kz/fai_kz_obscore.bin";
const DEFAULT_LIMIT: usize = 20_000;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn obscore_adql(limit: usize) -> String {
    format!("SELECT TOP {limit} {SELECT_COLUMNS} FROM {OBSCORE_TABLE}")
}

fn tap_csv(adql: &str, limit: usize) -> Option<(String, Vec<u8>)> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("180")
        .arg("-A")
        .arg("omegaflow-fai-kz-compiler/1.0")
        .arg("-G")
        .arg(TAP_SYNC)
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("MAXREC={limit}"))
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(p) => p,
        None => DEFAULT_OUT.to_string(),
    };
    let limit: usize = match arg_value(&args, "--limit") {
        Some(v) => match v.parse::<usize>() {
            Ok(n) if n >= 1 => n,
            _ => {
                eprintln!("--limit {v} carries no positive count");
                std::process::exit(1);
            }
        },
        None => DEFAULT_LIMIT,
    };
    let adql = obscore_adql(limit);
    let Some((code, body)) = tap_csv(&adql, limit) else {
        eprintln!(
            "{TAP_SYNC}: the TAP query did not answer (measured stall) — the harvest stays pending"
        );
        std::process::exit(1);
    };
    if code != "200" {
        eprintln!("{TAP_SYNC}: the TAP endpoint answered HTTP {code} — the harvest stays pending");
        std::process::exit(1);
    }
    let text = match std::str::from_utf8(&body) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("{TAP_SYNC}: the HTTP 200 body is not UTF-8 — the harvest stays pending");
            std::process::exit(1);
        }
    };
    let (records, counts) = match parse_obscore_csv(text) {
        Some(pair) => pair,
        None => {
            eprintln!(
                "{OBSCORE_TABLE}: the HTTP 200 CSV did not parse to measured observations (schema absent or rows void) — the harvest stays pending"
            );
            std::process::exit(1);
        }
    };
    let mut records: Vec<ObsRecord> = records;
    records.sort_by(|a, b| {
        a.t_min_mjd
            .total_cmp(&b.t_min_mjd)
            .then(a.s_ra_deg.total_cmp(&b.s_ra_deg))
            .then(a.s_dec_deg.total_cmp(&b.s_dec_deg))
    });
    let bin = match write_bin(&records) {
        Some(b) => b,
        None => {
            eprintln!("{OBSCORE_TABLE}: record encode void — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("write {out_path} void");
        std::process::exit(1);
    }
    match parse_bin(&bin) {
        Some(parsed) if parsed.len() == records.len() => {
            let with_em = records.iter().filter(|r| r.em_band_m.is_some()).count();
            let with_exp = records.iter().filter(|r| r.t_exptime_s.is_some()).count();
            eprintln!(
                "{out_path}: {} observations ({} B; rows {} | position_void {} | time_void {} | spectral_void {} | exposure_void {}; {} with spectral band, {} with exposure), roundtrip parses",
                records.len(),
                bin.len(),
                counts.rows,
                counts.position_void,
                counts.time_void,
                counts.spectral_void,
                counts.exposure_void,
                with_em,
                with_exp
            );
        }
        _ => {
            eprintln!("{out_path}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("{out_path}: did not reach the CDN");
        std::process::exit(1);
    }
}
