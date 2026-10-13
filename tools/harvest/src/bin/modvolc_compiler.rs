use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::hapi_csv::parse_iso_seconds;
use omegaflow::archivar::motion::body_fixed_to_icrs;
use omegaflow::archivar::parse_ephemeris_binary;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::body_url;
use omegaflow::cdn::upload_release;
use omegaflow::force::{force_id_of, kernel_id_for_force};
use std::collections::HashMap;

const NETLOC: &str = "nrt3.modaps.eosdis.nasa.gov";
const FORMAT: &str = "modvolc_thermal";
const COMPILER: &str = "tools/harvest/src/bin/modvolc_compiler.rs";
const FORCE_NAME: &str = "em";
const UNIT: &str = "w/m2/sr/um";

const FRAME_MAGIC: [u8; 3] = [0xCF, 0x86, 0x09];
const REC_BYTES: usize = 26 * 8;
const FRAME_HEADER: usize = 3 + 8 + 4 + 4;
const TTL_S: f64 = 2_592_000.0;
const TAU_S: f64 = 86_400.0;
const SURFACE_ALT_M: f64 = 0.0;

const OUT_DEFAULT: &str = "data/modvolc_thermal.bin";

fn usage() -> &'static str {
    "usage: modvolc_compiler <day-file.dat|url|directory> [--in <path|url>] [--out <file.bin>] \
     [--ephemeris <path|url>] [--limit N] [--ci-mode]"
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_limit(args: &[String], name: &str, default: usize) -> Result<usize, String> {
    match arg_value(args, name) {
        Some(v) => {
            let n = v
                .parse::<usize>()
                .map_err(|_| format!("{name} {v} carries no count"))?;
            if n == 0 {
                return Err(format!("{name} carries no positive count"));
            }
            Ok(n)
        }
        None => Ok(default),
    }
}

fn ids_of(force_name: &str) -> Result<(u8, u8), String> {
    let force =
        force_id_of(force_name).ok_or_else(|| format!("force '{force_name}' is not registered"))?;
    let kernel = kernel_id_for_force(force)
        .ok_or_else(|| format!("force '{force_name}' carries no kernel"))?;
    Ok((kernel, force))
}

fn field_line(force_name: &str) -> Result<String, String> {
    let kernel_name = omegaflow::force::default_kernel_for(force_name)
        .map(|(k, _)| k)
        .ok_or_else(|| format!("force '{force_name}' carries no default kernel"))?;
    Ok(format!(
        "field {} {} {} {} {} {} 0.0 0.0",
        FORMAT, FORMAT, kernel_name, force_name, UNIT, TAU_S as u64
    ))
}

fn positive(cell: &str) -> Option<f64> {
    let v: f64 = cell.parse().ok()?;
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Alert {
    lat: f64,
    lon: f64,
    radiance: f64,
}

struct Granule {
    epoch_unix: Option<f64>,
    alerts: Vec<Alert>,
}

fn parse_alert(line: &str) -> Option<Alert> {
    let f: Vec<&str> = line.split_whitespace().collect();
    if f.len() != 12 {
        return None;
    }
    let lat: f64 = f[5].parse().ok()?;
    let lon: f64 = f[6].parse().ok()?;
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return None;
    }
    if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
        return None;
    }
    let b21 = positive(f[0]);
    let b22 = positive(f[1]);
    let radiance = b21.or(b22)?;
    Some(Alert { lat, lon, radiance })
}

fn parse_granule(text: &str) -> Granule {
    let mut epoch_unix = None;
    let mut alerts = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t.split_whitespace().count() == 1 {
            if let Some(u) = parse_iso_seconds(t) {
                epoch_unix = Some(u);
                continue;
            }
        }
        if let Some(a) = parse_alert(t) {
            alerts.push(a);
        }
    }
    Granule { epoch_unix, alerts }
}

fn record(pos: [f64; 3], val: f64, tdb: f64, kernel: f64, force: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = tdb;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = 0.0;
    r[8] = kernel;
    r[9] = force;
    r[25] = 0.0;
    r
}

fn write_bin(records: &[[f64; 26]], response_epoch: f64, id: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(FRAME_HEADER + records.len() * REC_BYTES);
    out.extend_from_slice(&FRAME_MAGIC);
    out.extend_from_slice(&response_epoch.to_le_bytes());
    out.extend_from_slice(&id.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn verify_bin(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < FRAME_HEADER || bytes[0..3] != FRAME_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[15..19].try_into().ok()?) as usize;
    if bytes.len() != FRAME_HEADER + count * REC_BYTES {
        return None;
    }
    Some(count)
}

fn source_bytes(src: &str) -> Result<Vec<u8>, String> {
    if src.starts_with("http://") || src.starts_with("https://") {
        fetch_raw_bytes(src).ok_or_else(|| format!("{src}: fetch returned void"))
    } else {
        std::fs::read(src).map_err(|e| format!("read {src} returned void: {e}"))
    }
}

fn gather(src: &str) -> Result<Vec<String>, String> {
    let path = std::path::Path::new(src);
    if !path.is_dir() {
        return Ok(vec![src.to_string()]);
    }
    let mut files: Vec<String> = std::fs::read_dir(path)
        .map_err(|e| format!("read_dir {src} returned void: {e}"))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("dat"))
        .filter(|p| {
            p.file_name()
                .and_then(|x| x.to_str())
                .map(|n| n.starts_with("MODVOLC"))
                .unwrap_or(false)
        })
        .filter_map(|p| p.to_str().map(str::to_string))
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!("{src}: carries no MODVOLC .dat granule"));
    }
    Ok(files)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_flag = arg_value(args, "--out");
    let src = match arg_value(args, "--in").or_else(|| {
        args.iter()
            .find(|a| !a.starts_with("--") && out_flag.as_deref() != Some(a.as_str()))
            .cloned()
    }) {
        Some(v) => v,
        None => return Err(usage().to_string()),
    };
    let out = match out_flag.filter(|p| !p.is_empty()) {
        Some(v) => v,
        None => OUT_DEFAULT.to_string(),
    };
    let limit = parse_limit(args, "--limit", usize::MAX)?;
    let receiver = arg_value(args, "--body").ok_or_else(|| {
        "--body <name> is required — the receiver body is declared, never defaulted".to_string()
    })?;

    let lsk: LeapSeconds =
        embedded_lsk().ok_or_else(|| "the embedded naif0012.tls stays unread".to_string())?;
    let eph_bytes = match arg_value(args, "--ephemeris") {
        Some(s) if s.starts_with("http") => {
            fetch_raw_bytes(&s).ok_or_else(|| format!("earth ephemeris fetch void ({s})"))?
        }
        Some(p) => std::fs::read(&p).map_err(|e| format!("earth ephemeris read {p}: {e}"))?,
        None => fetch_raw_bytes(&body_url(&receiver))
            .ok_or_else(|| "receiver ephemeris fetch void (CDN)".to_string())?,
    };
    let earth = parse_ephemeris_binary(&eph_bytes)
        .ok_or_else(|| "the receiver ephemeris binary stays unread — no ICRS frame".to_string())?;
    let eph = HashMap::from([(receiver.clone(), earth)]);

    let (kernel, force) = ids_of(FORCE_NAME)?;
    let files = gather(&src)?;

    let mut records: Vec<[f64; 26]> = Vec::new();
    let mut response_epoch = 0.0f64;
    let mut skipped = 0usize;
    for file in &files {
        let bytes = source_bytes(file)?;
        let text = String::from_utf8_lossy(&bytes);
        let g = parse_granule(&text);
        let Some(unix) = g.epoch_unix else {
            eprintln!("modvolc: {file}: carries no granule timestamp — skipped");
            skipped += 1;
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(unix) else {
            eprintln!("modvolc: {file}: the granule clock reads void — skipped");
            skipped += 1;
            continue;
        };
        for a in &g.alerts {
            let Some(pos) = body_fixed_to_icrs(&receiver, a.lat, a.lon, SURFACE_ALT_M, tdb, &eph)
            else {
                skipped += 1;
                continue;
            };
            records.push(record(
                pos,
                a.radiance,
                tdb,
                f64::from(kernel),
                f64::from(force),
            ));
            if records.len() >= limit {
                break;
            }
        }
        if response_epoch == 0.0 && !records.is_empty() {
            response_epoch = tdb;
        }
        if records.len() >= limit {
            break;
        }
    }

    if records.is_empty() {
        return Err(format!(
            "{src}: no measured thermal alert left the harvest — the bin stays unwritten \
             (0 honored; {skipped} alerts skipped)"
        ));
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    let bin = write_bin(&records, response_epoch, 0);
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} returned void: {e}"))?;
    let count = verify_bin(&bin).ok_or_else(|| {
        format!("{out}: roundtrip parse returned void — the bin stays unverified")
    })?;
    if count != records.len() {
        return Err(format!(
            "{out}: {count} parsed against {} written — the bin stays unverified",
            records.len()
        ));
    }

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {src}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("at {receiver}");
    println!("ttl {}", TTL_S as u64);
    println!("{}", field_line(FORCE_NAME)?);
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} thermal alerts, {} B, roundtrip parses",
        records.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("{}", usage());
        eprintln!("  reads a MODVOLC daily granule (.dat) or a directory of granules");
        eprintln!("  the first line of a granule is its observation timestamp (UTC)");
        eprintln!("  writes the 0xCF 0x86 0x09 framed 26 x f64 wire records");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("modvolc_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STAMP: &str = "2019-12-31T00:15:00";
    const LINE: &str = " 5.207 -10.000 20.679  8.694  8.055  -41.586140  147.941391  0.529  0.204 -1.413   191   806";

    #[test]
    fn granule_reads_the_timestamp_and_the_alert() {
        let text = format!("{STAMP}\n{LINE}\n");
        let g = parse_granule(&text);
        assert_eq!(g.epoch_unix, parse_iso_seconds(STAMP));
        assert_eq!(g.alerts.len(), 1);
        let a = g.alerts[0];
        assert!((a.lat + 41.586140).abs() < 1e-9, "lat {}", a.lat);
        assert!((a.lon - 147.941391).abs() < 1e-9, "lon {}", a.lon);
        assert!((a.radiance - 5.207).abs() < 1e-12, "rad {}", a.radiance);
    }

    #[test]
    fn saturated_band21_falls_back_to_band22_and_double_fill_is_absent() {
        let b21_saturated = " -10.000 1.181 179.926  8.437  7.889  -28.017923  -48.902634  2.221  0.691 -1.771  1804   246";
        let a = parse_alert(b21_saturated).expect("band22 carries the 3.959 um radiance");
        assert!((a.radiance - 1.181).abs() < 1e-12, "rad {}", a.radiance);

        let both_saturated = " -10.000 -10.000 179.926  8.437  7.889  -28.017923  -48.902634  2.221  0.691 -1.771  1804   246";
        assert!(parse_alert(both_saturated).is_none());
    }

    #[test]
    fn a_foreign_line_is_refused() {
        assert!(parse_alert("not a granule").is_none());
        assert!(parse_alert("1 2 3").is_none());
        let out_of_range = " 5.0 -10.0 20.0 8.0 8.0  200.0  147.9  0.5  0.2 -1.4  191  806";
        assert!(parse_alert(out_of_range).is_none());
    }

    #[test]
    fn record_carries_the_wire_slots() {
        let (kernel, force) = ids_of(FORCE_NAME).expect("thermal ids");
        let r = record(
            [1.0, 2.0, 3.0],
            5.207,
            8.0e8,
            f64::from(kernel),
            f64::from(force),
        );
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 2.0);
        assert_eq!(r[2], 3.0);
        assert_eq!(r[3], 5.207);
        assert_eq!(r[4], 8.0e8);
        assert_eq!(r[5], TTL_S);
        assert_eq!(r[6], TAU_S);
        assert_eq!(r[8], f64::from(kernel));
        assert_eq!(r[9], f64::from(force));
        assert_eq!(r[25], 0.0);
        for slot in 10..25 {
            assert_eq!(r[slot], 0.0);
        }
    }

    #[test]
    fn the_bin_is_framed_and_roundtrips() {
        let (kernel, force) = ids_of(FORCE_NAME).expect("thermal ids");
        let rec = record(
            [1.0, 2.0, 3.0],
            5.207,
            8.0e8,
            f64::from(kernel),
            f64::from(force),
        );
        let bin = write_bin(&[rec], 8.0e8, 0);
        assert_eq!(&bin[0..3], &FRAME_MAGIC);
        assert_eq!(verify_bin(&bin), Some(1));
        assert_eq!(bin.len(), FRAME_HEADER + REC_BYTES);
    }
}
