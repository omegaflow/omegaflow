use omegaflow::cdn::upload_release;
use omegaflow::demeter::{self, parse_bin};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const CDN_TAG: &str = "regards.cnes.fr";

fn unix_to_ym(u: i64) -> (i32, u32) {
    let days = u.div_euclid(86400);
    let era = days.div_euclid(146097);
    let doe = days.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32)
}

fn diagnose(path: &str) -> Option<(PathBuf, Vec<demeter::DemeterBlock>)> {
    let (blocks, bin) = demeter::compile_file(path).ok()?;
    let file_stem = Path::new(path).file_stem()?.to_str()?;
    let out = Path::new("data").join(format!("{file_stem}.bin"));
    fs::write(&out, &bin).ok()?;
    let recs = parse_bin(&bin);
    if recs.is_empty() {
        return None;
    }
    let ne_min = blocks
        .iter()
        .map(|b| b.ne as f64)
        .fold(f64::INFINITY, f64::min);
    let ne_max = blocks
        .iter()
        .map(|b| b.ne as f64)
        .fold(f64::NEG_INFINITY, f64::max);
    let te_mean = blocks.iter().map(|b| b.te as f64).sum::<f64>() / blocks.len() as f64;
    let first_sec = blocks.first().map(|b| b.unix_seconds()).unwrap_or(0);
    let last_sec = blocks.last().map(|b| b.unix_seconds()).unwrap_or(0);
    let orbit = blocks.first().map(|b| b.orbit as u64).unwrap_or(0);
    let roundtrip = recs.len() == blocks.len();
    let fdate = |s: i64| {
        let secs = s.rem_euclid(86400);
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let sec = secs % 60;
        format!("{h:02}:{m:02}:{sec:02}")
    };
    println!(
        "{}: {} records, {}..{} ({:?}), orbit {}, ne {:.0}..{:.0} cm-3, te {:.0} K, roundtrip {}",
        out.display(),
        recs.len(),
        fdate(first_sec),
        fdate(last_sec),
        Path::new(path)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default(),
        orbit,
        ne_min,
        ne_max,
        te_mean,
        if roundtrip { "ok" } else { "FAIL" },
    );
    Some((out, blocks))
}

fn aggregate(dir: &str) -> BTreeMap<(i32, u32), Vec<u8>> {
    let mut by_month: BTreeMap<(i32, u32), Vec<u8>> = BTreeMap::new();
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map(|x| x == "DAT").unwrap_or(false))
        .collect();
    entries.sort();
    let mut n = 0usize;
    for p in &entries {
        let Ok((blocks, bin)) = demeter::compile_file(&p.to_string_lossy()) else {
            continue;
        };
        if blocks.is_empty() {
            continue;
        }
        let recs = parse_bin(&bin);
        for rec in recs {
            let unix = rec[1] as i64;
            let ym = unix_to_ym(unix);
            let bucket = by_month.entry(ym).or_default();
            for v in rec {
                bucket.extend_from_slice(&v.to_le_bytes());
            }
        }
        n += 1;
    }
    println!("aggregate: {n} files → {} monthly assets", by_month.len());
    by_month
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("demeter_compiler: no input — the series stays unwritten (0 honored)");
        eprintln!("  demeter_compiler <file.DAT>...     → per-file data/<stem>.bin");
        eprintln!("  demeter_compiler --aggregate <dir> [--ci-mode]  → demeter_isl_YYYYMM.bin");
        return;
    }
    if args.iter().any(|a| a == "--aggregate") {
        let dir = args
            .iter()
            .position(|a| a == "--aggregate")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.as_str())
            .unwrap_or(".");
        let ci_mode = args.iter().any(|a| a == "--ci-mode");
        fs::create_dir_all("data").ok();
        let by_month = aggregate(dir);
        for ((y, m), bytes) in by_month {
            let path = format!("data/demeter_isl_{y:04}{m:02}.bin");
            fs::write(&path, &bytes).expect("aggregate bin write");
            println!(
                "  {path}: {} bytes ({} records)",
                bytes.len(),
                bytes.len() / 64
            );
            if ci_mode {
                if upload_release(CDN_TAG, &path) {
                    println!("  uploaded {path} → {CDN_TAG}");
                } else {
                    eprintln!("  upload {path} void — the asset stays local (0 honored)");
                }
            }
        }
        return;
    }
    fs::create_dir_all("data").ok();
    for a in &args {
        diagnose(a);
    }
}
