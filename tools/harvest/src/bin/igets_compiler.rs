use omegaflow::archivar::geo::{parse_bin, write_bin, GeoRec, COMP_IGETS_G, MAGIC_IGETS};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{days_from_civil, parse as parse_lsk, LeapSeconds};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const NETLOC: &str = "igetsftp.gfz.de";
const SFTP_HOST: &str = "sftp://igetsftp.gfz.de";
const FILL: f64 = 999999.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = std::fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn login_name() -> Option<String> {
    let user = secret("IGETS_USER")?;
    Some(user.replace('@', "_at_"))
}

struct Sftp {
    login: String,
    password: String,
}

impl Sftp {
    fn list(&self, rel: &str) -> Option<Vec<(String, bool, Option<u64>)>> {
        let url = format!("{SFTP_HOST}/{rel}/");
        let out = Command::new("curl")
            .arg("-sS")
            .arg("--insecure")
            .arg("--max-time")
            .arg("120")
            .arg("-u")
            .arg(format!("{}:{}", self.login, self.password))
            .arg(&url)
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let text = String::from_utf8_lossy(&out.stdout);
        let mut entries = Vec::new();
        for line in text.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() < 9 {
                continue;
            }
            let kind = fields[0].as_bytes().first().copied()?;
            let name = fields[8..].join(" ");
            if name == "." || name == ".." {
                continue;
            }
            let size: Option<u64> = fields[4].parse().ok();
            if kind == b'd' {
                entries.push((name, true, size));
            } else if kind == b'-' {
                entries.push((name, false, size));
            }
        }
        Some(entries)
    }

    fn fetch(&self, rel: &str) -> Option<Vec<u8>> {
        let url = format!("{SFTP_HOST}/{rel}");
        for attempt in 0..3 {
            if attempt > 0 {
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
            let out = Command::new("curl")
                .arg("-sS")
                .arg("--insecure")
                .arg("--max-time")
                .arg("300")
                .arg("-u")
                .arg(format!("{}:{}", self.login, self.password))
                .arg(&url)
                .output()
                .ok();
            if let Some(out) = out {
                if out.status.success() {
                    return Some(out.stdout);
                }
                eprintln!(
                    "fetch {rel}: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                );
            }
        }
        None
    }
}

struct Ggp {
    lat: f64,
    lon: f64,
    alt: f64,
    scale: f64,
    samples: Vec<(f64, f64)>,
}

fn first_num(v: &str) -> Option<f64> {
    v.split_whitespace().next()?.parse().ok()
}

fn parse_ggp(text: &str) -> Option<Ggp> {
    let mut lat = None;
    let mut lon = None;
    let mut alt = None;
    let mut gcal = None;
    let mut grav_col: Option<(usize, bool)> = None;
    let mut samples = Vec::new();
    for line in text.lines() {
        if grav_col.is_none() {
            if let Some((k, v)) = line.split_once(':') {
                match k.trim() {
                    "N Latitude (deg)" => lat = first_num(v),
                    "E Longitude (deg)" => lon = first_num(v),
                    "Height (m)" | "Elevation MSL (m)" | "Geoid Height (m)" => alt = first_num(v),
                    "Gravity Cal (nm.s-2/V)" => gcal = first_num(v),
                    _ => {}
                }
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.first() == Some(&"yyyymmdd") {
                for (i, p) in parts.iter().enumerate() {
                    if *p == "g_fil" {
                        grav_col = Some((i, false));
                    } else if *p == "gravity(V)" {
                        grav_col = Some((i, true));
                    } else if p.starts_with("gravity(") {
                        grav_col = Some((i, false));
                    }
                }
            }
            continue;
        }
        let col = grav_col.unwrap().0;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() <= col
            || parts[0].len() != 8
            || !parts[0].bytes().all(|b| b.is_ascii_digit())
        {
            continue;
        }
        let y: i64 = match parts[0][0..4].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mo: i64 = match parts[0][4..6].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let d: i64 = match parts[0][6..8].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let clock: i64 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let hh = clock / 10000;
        let mm = (clock / 100) % 100;
        let ss = clock % 100;
        if hh > 23 || mm > 59 || ss > 60 {
            continue;
        }
        let raw: f64 = match parts[col].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !raw.is_finite() || raw.abs() >= FILL {
            continue;
        }
        let Some(days) = days_from_civil(y, mo, d) else {
            continue;
        };
        let unix = days as f64 * 86400.0 + hh as f64 * 3600.0 + mm as f64 * 60.0 + ss as f64;
        samples.push((unix, raw));
    }
    let (lat, lon, alt) = (lat?, lon?, alt?);
    if !(lat.is_finite() && lon.is_finite() && alt.is_finite()) {
        return None;
    }
    if samples.is_empty() {
        return None;
    }
    let (_, is_volts) = grav_col?;
    let scale = if is_volts {
        let g = gcal?;
        if !g.is_finite() {
            return None;
        }
        g
    } else {
        1.0
    };
    Some(Ggp {
        lat,
        lon,
        alt,
        scale,
        samples,
    })
}

fn median_interval(samples: &[(f64, f64)]) -> f64 {
    let mut ts: Vec<f64> = samples.iter().map(|s| s.0).collect();
    ts.sort_by(f64::total_cmp);
    let mut diffs: Vec<f64> = ts
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|d| *d > 0.0)
        .collect();
    if diffs.is_empty() {
        return 0.0;
    }
    diffs.sort_by(f64::total_cmp);
    diffs[diffs.len() / 2]
}

fn compile_file(text: &str, lsk: &LeapSeconds, bucket_s: f64, records: &mut Vec<GeoRec>) -> usize {
    let Some(ggp) = parse_ggp(text) else {
        return 0;
    };
    let raw_width = median_interval(&ggp.samples);
    let mut series: Vec<(f64, f64)> = Vec::new();
    if bucket_s > 0.0 {
        let mut sums: HashMap<i64, (f64, usize)> = HashMap::new();
        for (unix, raw) in &ggp.samples {
            let key = (*unix / bucket_s).floor() as i64;
            let slot = sums.entry(key).or_insert((0.0, 0));
            slot.0 += raw;
            slot.1 += 1;
        }
        let mut keys: Vec<i64> = sums.keys().copied().collect();
        keys.sort_unstable();
        for key in keys {
            let (sum, n) = sums[&key];
            series.push((key as f64 * bucket_s, sum / n as f64));
        }
    } else {
        series = ggp.samples.clone();
    }
    let width = if bucket_s > 0.0 { bucket_s } else { raw_width };
    let mut n = 0usize;
    for (unix, raw) in &series {
        let Some(t) = lsk.unix_to_tdb(*unix) else {
            continue;
        };
        records.push(GeoRec {
            t,
            lat: ggp.lat,
            lon: ggp.lon,
            alt: ggp.alt,
            freq: 0.0,
            bin_width: width,
            val: raw * ggp.scale,
            comp: COMP_IGETS_G,
            station: 0,
        });
        n += 1;
    }
    n
}

fn walk_local(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_local(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("ggp") {
            out.push(path);
        }
    }
}

fn path_allowed(rel: &str, station: &Option<String>, level: &Option<String>) -> bool {
    let comps: Vec<&str> = rel.split('/').collect();
    if let Some(st) = station {
        if comps.first() != Some(&st.as_str()) {
            return false;
        }
    }
    if let Some(lv) = level {
        let want = format!("Level{lv}");
        let has_level = comps.iter().any(|c| c.starts_with("Level"));
        if has_level && !comps.iter().any(|c| c.eq_ignore_ascii_case(&want)) {
            return false;
        }
    }
    true
}

fn walk_sftp(
    sftp: &Sftp,
    rel: &str,
    station: &Option<String>,
    level: &Option<String>,
    remote: &mut Vec<(String, Option<u64>)>,
    limit: Option<usize>,
) {
    if let Some(cap) = limit {
        if remote.len() >= cap {
            return;
        }
    }
    let Some(entries) = sftp.list(rel) else {
        eprintln!("list {rel}: void — the branch stays unharvested");
        return;
    };
    for (name, is_dir, size) in entries {
        let child = if rel.is_empty() {
            name.clone()
        } else {
            format!("{rel}/{name}")
        };
        if is_dir {
            if path_allowed(&child, station, level) {
                walk_sftp(sftp, &child, station, level, remote, limit);
            }
        } else if name.ends_with(".ggp") && path_allowed(&child, station, level) {
            remote.push((child, size));
            if let Some(cap) = limit {
                if remote.len() >= cap {
                    return;
                }
            }
        }
    }
}

fn download(sftp: &Sftp, remote: &[(String, Option<u64>)], cache: &Path, files: &mut Vec<PathBuf>) {
    for (rel, _) in remote {
        let local = cache.join(rel);
        if !local.exists() {
            let Some(bytes) = sftp.fetch(rel) else {
                continue;
            };
            if let Some(parent) = local.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::write(&local, &bytes).is_err() {
                continue;
            }
        }
        files.push(local);
    }
}

enum Source {
    Local(PathBuf),
    Remote(String),
}

impl Source {
    fn label(&self) -> String {
        match self {
            Source::Local(p) => p.display().to_string(),
            Source::Remote(r) => r.clone(),
        }
    }
}

fn source_text(src: &Source, sftp: &Option<Sftp>) -> Option<String> {
    match src {
        Source::Local(p) => std::fs::read_to_string(p).ok(),
        Source::Remote(r) => {
            let bytes = sftp.as_ref()?.fetch(r)?;
            String::from_utf8(bytes).ok()
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let probe = args.iter().any(|a| a == "--probe");
    let list_only = args.iter().any(|a| a == "--list");
    let station = arg_value(&args, "--station");
    let level = arg_value(&args, "--level");
    let limit: Option<usize> = arg_value(&args, "--limit-files").and_then(|v| v.parse().ok());

    let stream = args.iter().any(|a| a == "--stream");
    let sources: Vec<Source>;
    let mut sftp_holder: Option<Sftp> = None;
    if let Some(dir) = arg_value(&args, "--in") {
        let mut files: Vec<PathBuf> = Vec::new();
        walk_local(Path::new(&dir), &mut files);
        files.sort();
        sources = files.into_iter().map(Source::Local).collect();
    } else {
        let Some(login) = login_name() else {
            eprintln!("IGETS_USER absent — no login name (SFTP stays unread)");
            std::process::exit(1);
        };
        let Some(password) = secret("IGETS_PASS") else {
            eprintln!("IGETS_PASS absent — no credential (SFTP stays unread)");
            std::process::exit(1);
        };
        let cache = match arg_value(&args, "--cache") {
            Some(v) => v,
            None => "data/igetsftp.gfz.de".to_string(),
        };
        let sftp = Sftp { login, password };
        let mut remote = Vec::new();
        walk_sftp(&sftp, "", &station, &level, &mut remote, limit);
        if list_only {
            let total: u64 = remote.iter().filter_map(|(_, s)| *s).sum();
            for (rel, size) in &remote {
                match size {
                    Some(s) => println!("{s}\t{rel}"),
                    None => println!("?\t{rel}"),
                }
            }
            eprintln!("{} .ggp files, {} B on the SFTP tree", remote.len(), total);
            return;
        }
        if stream {
            sources = remote.into_iter().map(|(r, _)| Source::Remote(r)).collect();
        } else {
            let mut files: Vec<PathBuf> = Vec::new();
            download(&sftp, &remote, Path::new(&cache), &mut files);
            files.sort();
            sources = files.into_iter().map(Source::Local).collect();
        }
        sftp_holder = Some(sftp);
    }

    if sources.is_empty() {
        eprintln!("igets: no .ggp files flow — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }

    if probe {
        for src in &sources {
            let Some(text) = source_text(src, &sftp_holder) else {
                continue;
            };
            println!("=== {} ===", src.label());
            for l in text.lines().take(10) {
                println!("{l}");
            }
        }
        eprintln!("{} .ggp files flow from the tree", sources.len());
        return;
    }

    let out_bin = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => {
            eprintln!("--out-bin <path> required");
            std::process::exit(1);
        }
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let bucket_min: f64 = arg_value(&args, "--bucket-min")
        .and_then(|v| v.parse().ok())
        .unwrap_or(60.0);
    if !bucket_min.is_finite() || bucket_min < 0.0 {
        eprintln!("--bucket-min {bucket_min} carries no measured bucket width");
        std::process::exit(1);
    }
    let bucket_s = bucket_min * 60.0;

    let mut records: Vec<GeoRec> = Vec::new();
    let mut parsed = 0usize;
    for src in &sources {
        let Some(text) = source_text(src, &sftp_holder) else {
            eprintln!("{}: unreadable — file skipped", src.label());
            continue;
        };
        let n = compile_file(&text, &lsk, bucket_s, &mut records);
        if n == 0 {
            eprintln!(
                "{}: carries no measured gravity samples — file skipped",
                src.label()
            );
            continue;
        }
        parsed += 1;
    }

    if records.is_empty() {
        eprintln!("igets: no measured gravity samples — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a.t.total_cmp(&b.t));

    let bytes = write_bin(MAGIC_IGETS, &records);
    if std::fs::write(&out_bin, &bytes).is_err() {
        eprintln!("write {out_bin} returned void");
        std::process::exit(1);
    }
    match parse_bin(MAGIC_IGETS, &bytes) {
        Some(parsed_bin) => eprintln!(
            "{}: {} geo records ({parsed} files, {}-min mean buckets, {} B), roundtrip parses",
            out_bin,
            parsed_bin.len(),
            bucket_min,
            bytes.len()
        ),
        None => {
            eprintln!("{out_bin}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_bin) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_level1_volts_with_calibration() {
        let text = "N Latitude (deg)        :    49.1458\n\
                    E Longitude (deg)       :    12.8794\n\
                    Height (m)              :   612.0000\n\
                    Gravity Cal (nm.s-2/V)  :  -981.0000\n\
                    yyyymmdd hhmmss gravity(V) pressure(V)\n\
                    19960728   3100  -3.20846  -1.23986\n";
        let ggp = parse_ggp(text).expect("level1 parses");
        assert_eq!(ggp.samples.len(), 1);
        assert!((ggp.scale - -981.0).abs() < 1e-9);
        let val = ggp.samples[0].1 * ggp.scale;
        assert!((val - 3147.49926).abs() < 1e-3);
    }

    #[test]
    fn parses_level2_g_fil_without_calibration() {
        let text = "N Latitude (deg)     :  50.6093      0.0005 measured\n\
                    E Longitude (deg)    :   6.0066      0.0005 measured\n\
                    Geoid Height (m)     : 250.0000      0.1000 measured\n\
                    Calibration          :   784.220 &    60.000 from 20110616 to 20210331\n\
                    yyyymmdd hhmmss     g_fil     p_fil   g_nofil   p_nofil\n\
                    20210201      0 -1788.305   983.872 99999.999 99999.999\n";
        let ggp = parse_ggp(text).expect("level2 parses");
        assert_eq!(ggp.scale, 1.0);
        assert_eq!(ggp.samples.len(), 1);
        assert!((ggp.samples[0].1 - -1788.305).abs() < 1e-9);
    }

    #[test]
    fn parses_gravity_nm_s2_column() {
        let text = "N Latitude (deg)     :  50.6093    0.0005 measured\n\
                    E Longitude (deg)    :   6.0066    0.0005 measured\n\
                    Geoid Height (m)     : 250.0       0.1    measured\n\
                    Gravity Cal (uGal/V) : -784.2200   0.7    measured\n\
                    yyyymmdd hhmmss gravity(nm/s**2) pressure(hpa)\n\
                    20110101 000000 -5121.711  1008.248\n";
        let ggp = parse_ggp(text).expect("nm/s2 column parses");
        assert_eq!(ggp.scale, 1.0);
        assert!((ggp.samples[0].1 - -5121.711).abs() < 1e-9);
    }

    #[test]
    fn rejects_uncalibrated_volts() {
        let text = "N Latitude (deg)     :  50.608520    0.0005 measured\n\
                    E Longitude (deg)    :   6.009539    0.0005 measured\n\
                    Elevation MSL (m)    : 250.0         0.1    measured\n\
                    yyyymmdd hhmmss gravity(V) pressure(V)\n\
                    20210101 000000 -6.335938  2.930177\n";
        assert!(parse_ggp(text).is_none());
    }
}
