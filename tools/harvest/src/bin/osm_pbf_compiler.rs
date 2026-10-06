use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::geo::{COMP_OSM_PBF_NODE, GeoRec, MAGIC_OSM_PBF, parse_bin, write_bin};
use omegaflow::archivar::osm_pbf::{FORMAT, parse_nodes, parse_timestamp};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::witness::{FieldIdentity, magic_identity};

const NETLOC: &str = "download.geofabrik.de";
const URL: &str = "https://download.geofabrik.de/europe/monaco-latest.osm.pbf";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn positional_source(args: &[String]) -> Option<String> {
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if matches!(a.as_str(), "--out" | "--netloc") {
            i += 2;
            continue;
        }
        if a.starts_with("--") || a == "-h" {
            i += 1;
            continue;
        }
        return Some(a.clone());
    }
    None
}

fn netloc_of(args: &[String], source: &str) -> String {
    match arg_value(args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => match source.split("//").nth(1).and_then(|r| r.split('/').next()) {
            Some(h) if !h.is_empty() && source.starts_with("http") => h.to_string(),
            _ => NETLOC.to_string(),
        },
    }
}

fn out_path(args: &[String], netloc: &str) -> String {
    match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{netloc}/monaco_nodes.bin"),
    }
}

fn ensure_parent(out: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    Ok(())
}

fn harvest_epoch(bytes: &[u8]) -> Result<f64, String> {
    if let Some(t) = parse_timestamp(bytes) {
        return Ok(t);
    }
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .map_err(|_| "the system clock reads before 1970 — no harvest instant".to_string())
}

fn witness_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FieldIdentity::Oscillator) => Ok(()),
        Some(other) => Err(format!(
            "{} reads {:?}, not an oscillator — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let source = match positional_source(args) {
        Some(s) => s,
        None => URL.to_string(),
    };
    witness_identity(MAGIC_OSM_PBF)?;
    let bytes = fetch_raw_bytes(&source).ok_or_else(|| format!("{source}: fetch void"))?;
    let nodes = parse_nodes(&bytes).ok_or_else(|| {
        format!("{source}: the OSM PBF framing reads no node — the asset stays unwritten")
    })?;
    let t = harvest_epoch(&bytes)?;
    let mut records = Vec::with_capacity(nodes.len());
    for (lat, lon) in nodes {
        if !lat.is_finite() || !lon.is_finite() {
            continue;
        }
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        records.push(GeoRec {
            t,
            lat,
            lon,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 1.0,
            comp: COMP_OSM_PBF_NODE,
            station: 0,
        });
    }
    if records.is_empty() {
        return Err(format!(
            "{source}: no measured node coordinate left the harvest — the asset stays unwritten (0 honored)"
        ));
    }
    let netloc = netloc_of(args, &source);
    let out = out_path(args, &netloc);
    ensure_parent(&out)?;
    let data = write_bin(MAGIC_OSM_PBF, &records);
    std::fs::write(&out, &data).map_err(|e| format!("write {out} void: {e}"))?;
    match parse_bin(MAGIC_OSM_PBF, &data) {
        Some(parsed) if parsed.len() == records.len() => {
            println!(
                "url https://github.com/omegaflow/sources/releases/download/{netloc}/{}",
                last_seg(&out)
            );
            println!("origin {source}");
            println!("compiler tools/harvest/src/bin/osm_pbf_compiler.rs");
            println!("format {FORMAT}");
            println!("sha256 {}", sha256_hex(&data));
            eprintln!(
                "{out}: {} node coordinates from the OSM PBF, {} B, roundtrip parses",
                parsed.len(),
                data.len()
            );
        }
        Some(parsed) => {
            return Err(format!(
                "{out}: {} parsed vs {} written — the asset stays unverified",
                parsed.len(),
                records.len()
            ));
        }
        None => {
            return Err(format!(
                "{out}: roundtrip parse void — the asset stays unverified"
            ));
        }
    }
    if ci_mode && !upload_release(&netloc, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {netloc} release"
        ));
    }
    Ok(())
}

fn last_seg(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(s) => s,
        None => path,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: osm_pbf_compiler [<url|path>] [--out <path>] [--netloc <netloc>] [--ci-mode]"
        );
        eprintln!("  reads an OSM PBF (Protobuf) node stream, DenseNodes and Node");
        eprintln!("  unit: node coordinate (lat, lon) in degrees; val = 1 per node");
        eprintln!("  --ci-mode uploads the verified asset to the <netloc> CDN release");
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("osm_pbf_compiler: {msg}");
        std::process::exit(2);
    }
}
