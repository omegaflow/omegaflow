use omegaflow::archivar::llnl_g3d::{
    COORD_FILE, MAGIC, NLAYERS, NODES, ZIP_SHA256, ZIP_URL, compile_zip, parse_coordinates,
    parse_surface, read_bin, surface_index, write_bin, zip_entries, zip_entry_bytes,
};
use omegaflow::cdn::upload_release;
use omegaflow::zeuge::{FeldIdentitaet, ZeugeArt, magic_identity};
use std::process::Command;

const NETLOC: &str = "gs.llnl.gov";
const UA: &str = "omegaflow-llnl-g3d-jps-compiler/1.0";
const DEFAULT_ZIP: &str = "data/gs.llnl.gov/llnl_g3d_jps.interpolated.zip";
const DEFAULT_OUT: &str = "data/gs.llnl.gov/LLNL_G3D_JPS.interpolated.g3d1";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn witness_gestalt_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt)) => {
            eprintln!(
                "{} reads as a gestalt witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not gestalt — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn download(url: &str, path: &str) -> Result<(), String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("900")
        .arg("-A")
        .arg(UA)
        .arg("-o")
        .arg(path)
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: {} — the zip stays unfetched",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let sz = std::fs::metadata(path)
        .map_err(|e| format!("metadata {path} returned void: {e}"))?
        .len();
    if sz == 0 {
        return Err(format!("{url}: the zip carries no bytes"));
    }
    eprintln!("{url}: {sz} B -> {path}");
    Ok(())
}

fn diagnose_zip(bytes: &[u8]) {
    let entries = zip_entries(bytes);
    eprintln!(
        "llnl_g3d_jps_compiler: the zip carries {} central-directory record(s)",
        entries.len()
    );
    let mut by_index: Vec<Option<String>> = vec![None; NLAYERS + 1];
    for (name, _, _, _) in &entries {
        if let Some(idx) = surface_index(name) {
            if by_index[idx].is_none() {
                by_index[idx] = Some(name.clone());
            }
        }
    }
    match zip_entry_bytes(bytes, COORD_FILE) {
        None => eprintln!("llnl_g3d_jps_compiler: {COORD_FILE} is absent from the zip"),
        Some(raw) => match std::str::from_utf8(&raw) {
            Err(_) => eprintln!("llnl_g3d_jps_compiler: {COORD_FILE} is not strict UTF-8"),
            Ok(text) => match parse_coordinates(text) {
                Some(g) => eprintln!(
                    "llnl_g3d_jps_compiler: {COORD_FILE} parses as {} node record(s)",
                    g.len()
                ),
                None => {
                    eprintln!("llnl_g3d_jps_compiler: {COORD_FILE} refuses the grid parser")
                }
            },
        },
    }
    for idx in 1..=NLAYERS {
        match &by_index[idx] {
            None => eprintln!("llnl_g3d_jps_compiler: surface {idx} is absent from the zip"),
            Some(name) => {
                let raw = match zip_entry_bytes(bytes, name) {
                    Some(r) => r,
                    None => {
                        eprintln!("llnl_g3d_jps_compiler: surface {idx} ({name}) stays uninflated");
                        continue;
                    }
                };
                let text = match std::str::from_utf8(&raw) {
                    Ok(t) => t,
                    Err(_) => {
                        eprintln!(
                            "llnl_g3d_jps_compiler: surface {idx} ({name}) is not strict UTF-8"
                        );
                        continue;
                    }
                };
                match parse_surface(text) {
                    Some(rows) => eprintln!(
                        "llnl_g3d_jps_compiler: surface {idx} ({name}) parses as {} row record(s)",
                        rows.len()
                    ),
                    None => {
                        eprintln!(
                            "llnl_g3d_jps_compiler: surface {idx} ({name}) refuses the row parser"
                        )
                    }
                }
            }
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    witness_gestalt_identity(MAGIC)?;
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => DEFAULT_OUT.to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let zip_path = match arg_value(args, "--zip") {
        Some(p) => p,
        None => DEFAULT_ZIP.to_string(),
    };
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "--zip" => i += 2,
            "--ci-mode" => i += 1,
            other if other.starts_with("--") => {
                return Err(format!(
                    "unknown argument {other} — usage: llnl_g3d_jps_compiler [--zip <llnl_g3d_jps.interpolated.zip>] [--out <asset.g3d1>] [--ci-mode] — refused"
                ));
            }
            other => {
                return Err(format!(
                    "unexpected positional argument {other} — the model path is never silent — refused"
                ));
            }
        }
    }
    let bytes = match std::fs::read(&zip_path) {
        Ok(b) => b,
        Err(_) => {
            if let Some(parent) = std::path::Path::new(&zip_path).parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            download(ZIP_URL, &zip_path)?;
            std::fs::read(&zip_path).map_err(|e| format!("read {zip_path} returned void: {e}"))?
        }
    };
    let digest = omegaflow::sha256::sha256_hex(&bytes);
    if digest != ZIP_SHA256 {
        return Err(format!(
            "{zip_path}: sha256 {digest} — the measured zip is 3bb043779efd1c4e4cb7e99547ad7c2fa12f7477922bb3cb22dd9ea67450edc8 (gs.llnl.gov, measured 2026-09-26) — the compile stays closed"
        ));
    }
    let Some(model) = compile_zip(&bytes) else {
        diagnose_zip(&bytes);
        return Err(
            "the zip does not read as the LLNL-G3D-JPS interpolated model (59 surfaces of 65341 rows, regular 1-degree grid) — the asset stays unwritten"
                .into(),
        );
    };
    let asset = write_bin(&model);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out_path, &asset)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let back =
        std::fs::read(&out_path).map_err(|e| format!("read {out_path} returned void: {e}"))?;
    let Some(parsed) = read_bin(&back) else {
        return Err(format!(
            "{out_path}: the asset does not read back — it stays unverified"
        ));
    };
    if parsed.mean_depth != model.mean_depth
        || parsed.geometry != model.geometry
        || parsed.layers != model.layers
    {
        return Err(format!(
            "{out_path}: the roundtrip differs from the written records — the asset stays unverified"
        ));
    }
    let records = NLAYERS * NODES;
    eprintln!(
        "llnl_g3d_jps_compiler: {out_path}: {records} record(s) — {NLAYERS} surface(s) x {NODES} node(s), {} record byte(s) each, {} asset byte(s) — the roundtrip reads back",
        omegaflow::archivar::llnl_g3d::RECORD_BYTES,
        asset.len()
    );
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("llnl_g3d_jps_compiler: {msg}");
        std::process::exit(1);
    }
}
