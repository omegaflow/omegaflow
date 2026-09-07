use omegaflow::cdn::upload_release;
use omegaflow::json::{jpath, parse_json};
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::Command;

const NETLOC: &str = "opentopodata.org";
const ENDPOINT: &str = "https://api.opentopodata.org/v1/gebco2020?locations=";
const MAGIC: [u8; 4] = *b"GBCO";
const REC_BYTES: usize = 24;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_depth(lat: f64, lon: f64) -> Option<f64> {
    let url = format!("{ENDPOINT}{lat},{lon}");
    let out = Command::new("curl")
        .arg("-s")
        .arg("--max-time")
        .arg("20")
        .arg(&url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let body = String::from_utf8_lossy(&out.stdout);
    let json = parse_json(&body)?;
    let v = jpath(&json, "results.0.elevation")?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn parse_locations(spec: &str) -> Vec<(f64, f64)> {
    let mut pts = Vec::new();
    for part in spec.split(';') {
        let mut it = part.split(',');
        let lat = it.next().and_then(|t| t.trim().parse::<f64>().ok());
        let lon = it.next().and_then(|t| t.trim().parse::<f64>().ok());
        if let (Some(a), Some(b)) = (lat, lon) {
            if a.is_finite() && b.is_finite() {
                pts.push((a, b));
            }
        }
    }
    pts
}

fn run(args: &[String]) -> Result<(), String> {
    let Some(loc) = arg_value(args, "--locations") else {
        return Err(
            "usage: gebco_bathymetry_compiler --locations <lat,lon[;lat,lon...]> --out <map.gbco> [--ci-mode] — refused"
                .into(),
        );
    };
    let Some(out_path) = arg_value(args, "--out") else {
        return Err("--out <map.gbco>: the asset path is never silent — refused".into());
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let pts = parse_locations(&loc);

    let mut records: Vec<(f64, f64, f64)> = Vec::new();
    for (lat, lon) in &pts {
        match fetch_depth(*lat, *lon) {
            Some(d) => records.push((*lat, *lon, d)),
            None => eprintln!("gebco: {lat},{lon} returned no depth — skipped (0 honored)"),
        }
    }
    if records.is_empty() {
        return Err("no depth measured — the asset stays unwritten (0 honored)".into());
    }

    let mut f = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    f.write_all(&MAGIC)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    f.write_all(&(records.len() as u32).to_le_bytes())
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    for (lat, lon, d) in &records {
        f.write_all(&lat.to_le_bytes())
            .and_then(|_| f.write_all(&lon.to_le_bytes()))
            .and_then(|_| f.write_all(&d.to_le_bytes()))
            .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    }
    let _ = f.flush();

    let mut vf = std::fs::File::open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; 8];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    if head[0..4] != MAGIC {
        return Err(format!("{out_path}: the magic stays unread"));
    }
    let n = u32::from_le_bytes(head[4..8].try_into().map_err(|_| "count unread")?) as usize;
    if n != records.len() {
        return Err(format!(
            "{out_path}: {n} records read, {} written — the asset stays unwritten",
            records.len()
        ));
    }
    vf.seek(SeekFrom::Start(8 + ((n - 1) * REC_BYTES) as u64))
        .map_err(|e| format!("seek {out_path} returned void: {e}"))?;
    let mut tail = [0u8; REC_BYTES];
    vf.read_exact(&mut tail)
        .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
    let d = f64::from_le_bytes(tail[16..24].try_into().map_err(|_| "depth unread")?);
    if !d.is_finite() {
        return Err(format!("{out_path}: the last depth stays unread"));
    }
    eprintln!("{out_path}: {} depth records, roundtrip verified", n);

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("gebco_bathymetry_compiler: {msg}");
        std::process::exit(1);
    }
}
