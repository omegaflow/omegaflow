use omegaflow::archivar::quaoar_occlt::{zip_entries, zip_extract};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::zcta::{self, FORMAT, NETLOC};
use omegaflow::cdn::upload_release;
use std::process::Command;

const ZIP_URL: &str = "https://www2.census.gov/geo/docs/maps-data/data/gazetteer/2020_Gazetteer/2020_Gaz_zcta_national.zip";
const ASSET: &str = "zcta_gazetteer.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("300")
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: reads no bytes — {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    if out.stdout.is_empty() {
        return Err(format!("{url}: carries no bytes"));
    }
    Ok(out.stdout)
}

fn source_bytes(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        fetch(source)
    } else {
        std::fs::read(source).map_err(|e| format!("read {source} returned void: {e}"))
    }
}

fn selftest() {
    let rows = zcta::parse_txt(
        "GEOID\tALAND\tAWATER\tALAND_SQMI\tAWATER_SQMI\tINTPTLAT\tINTPTLONG\n\
00601\t1\t1\t1\t1\t18.180555\t-66.749961\n\
21093\t1\t1\t1\t1\t39.4\t-76.7\n",
    );
    assert_eq!(rows.len(), 2);
    assert_eq!(zcta::lookup(&rows, "00601"), Some((18.180555, -66.749961)));
    let bytes = zcta::write_bin(&rows);
    assert_eq!(zcta::parse_bin(&bytes).as_deref(), Some(rows.as_slice()));
    eprintln!("zcta_gazetteer_compiler: selftest passes (parse + binary roundtrip + lookup)");
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let source = match arg_value(args, "--input") {
        Some(p) => p,
        None => match arg_value(args, "--url") {
            Some(u) => u,
            None => ZIP_URL.to_string(),
        },
    };
    let bytes = source_bytes(&source)?;
    let entries = zip_entries(&bytes)
        .ok_or_else(|| format!("{source}: central directory void — the zip stays unread"))?;
    let member = entries
        .iter()
        .find(|e| e.name.ends_with(".txt"))
        .ok_or_else(|| format!("{source}: no .txt member — the gazetteer stays unread"))?;
    let raw = zip_extract(&bytes, member)
        .ok_or_else(|| format!("{}: entry extract void", member.name))?;
    let text = String::from_utf8_lossy(&raw);
    let rows = zcta::parse_txt(&text);
    if rows.is_empty() {
        return Err(format!(
            "{source}: no ZCTA row carries a measured GEOID and centroid — the asset stays unwritten (0 honored)"
        ));
    }
    let bin = zcta::write_bin(&rows);
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{ASSET}"),
    };
    if let Some(parent) = std::path::Path::new(&out).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create {} void: {e}", parent.display()))?;
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;
    let read = std::fs::read(&out).map_err(|e| format!("{out} stays unread: {e}"))?;
    if read != bin {
        return Err(format!(
            "{out}: read-back differs — the asset stays unverified"
        ));
    }
    let present = zcta::parse_bin(&read)
        .ok_or_else(|| format!("{out}: roundtrip parse void — the asset stays unverified"))?;
    eprintln!(
        "{out}: {} ZCTA centroid(s), {} B from {source}",
        present.len(),
        bin.len()
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{ASSET}");
    println!("origin {source}");
    println!("compiler tools/harvest/src/bin/zcta_gazetteer_compiler.rs");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!(
            "{out}: CDN upload did not reach the {NETLOC} release"
        ));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: zcta_gazetteer_compiler [--input <zip-path>] [--url <zip-url>] [--out <bin>] [--ci-mode] [--selftest]"
        );
        eprintln!(
            "  reads the US Census 2020 ZCTA gazetteer zip; writes a GEOID->INTPTLAT/INTPTLONG lookup bin"
        );
        std::process::exit(1);
    }
    if let Err(msg) = run(&args) {
        eprintln!("zcta_gazetteer_compiler: {msg}");
        std::process::exit(1);
    }
}
