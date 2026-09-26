use omegaflow::archivar::charm2;
use omegaflow::cdn::upload_release;
use std::process::Command;

const NETLOC: &str = "tapvizier.cds.unistra.fr";
const TAP: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const JOIN_QUERY: &str = r#"SELECT c.recno AS crecno, c.RAJ2000 AS cra, c.DEJ2000 AS cdec, c.UD AS cud, c.LD AS cld, g.RAJ2000 AS gra, g.DEJ2000 AS gdec, g.Plx AS gplx, g.Gmag AS gmag FROM "J/A+A/431/773/charm2" AS c JOIN "I/355/gaiadr3" AS g ON DISTANCE(POINT('ICRS', c.RAJ2000, c.DEJ2000), POINT('ICRS', g.RAJ2000, g.DEJ2000)) < 2.0/3600.0 WHERE c.Method='LO' AND (c.UD > 0 OR c.LD > 0)"#;
const DEFAULT_OUT: &str = "data/tapvizier.cds.unistra.fr/charm2_gaia.bin";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn tap_tsv(query: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=tsv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={query}"))
        .arg(TAP)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "charm2_compiler http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent()
        && !parent.as_os_str().is_empty()
        && std::fs::create_dir_all(parent).is_err()
    {
        eprintln!("charm2_compiler: create parent dir of {out_path} void");
        std::process::exit(1);
    }
    let body = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("charm2_compiler: read {path}: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let query = match arg_value(&args, "--query") {
                Some(v) => v,
                None => JOIN_QUERY.to_string(),
            };
            match tap_tsv(&query) {
                Some(b) => b,
                None => std::process::exit(1),
            }
        }
    };
    if body.is_empty() {
        eprintln!("charm2_compiler: the result body is void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let stars = match charm2::parse_crossmatch_tsv(&body) {
        Some(s) => s,
        None => {
            eprintln!(
                "charm2 cross-match: {} B carry no measured row — the bin stays unwritten (0 honored)",
                body.len()
            );
            std::process::exit(1);
        }
    };
    let bin = charm2::write_bin(&stars);
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("charm2_compiler: write {out_path} void");
        std::process::exit(1);
    }
    match charm2::parse_bin(&bin) {
        Some(parsed) if parsed.len() == stars.len() => {
            eprintln!(
                "charm2 gaia: {} rows, {} B -> {out_path} (roundtrip parses)",
                parsed.len(),
                bin.len()
            );
        }
        _ => {
            eprintln!(
                "charm2_compiler: {out_path}: roundtrip parse void — the asset stays unverified"
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("charm2_compiler: upload {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}
