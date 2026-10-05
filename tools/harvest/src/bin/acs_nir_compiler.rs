use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const DEFAULT_NETLOC: &str = "archives.esac.esa.int";
const DEFAULT_TAB: &str = "https://archives.esac.esa.int/psa/ftp/ExoMars2016/em16_tgo_acs/data_raw/Science_Phase/Orbit_Range_4100_4199/Orbit_4140/acs_raw_sc_nir_20181027T221352-20181027T223353-4140-1-1-EC__4_0.tab";
const MEASURED_SHA256: &str = "465f3c07c4a04053895046cb8853ce96f1c38fe7038d23d7cd05d91ce87419c1";
const MEASURED_BYTES: usize = 543_800;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let netloc = match arg_value(&args, "--netloc") {
        Some(n) if !n.is_empty() => n,
        _ => DEFAULT_NETLOC.to_string(),
    };
    let tab_spec = match arg_value(&args, "--tab") {
        Some(t) => t,
        None => DEFAULT_TAB.to_string(),
    };
    let Some(bytes) = fetch_raw_bytes(&tab_spec) else {
        eprintln!("{tab_spec}: fetch void");
        std::process::exit(1);
    };
    let sha = sha256_hex(&bytes);
    if sha != MEASURED_SHA256 || bytes.len() != MEASURED_BYTES {
        eprintln!(
            "{tab_spec}: sha256 {sha} / {} byte(s) — the asset differs from the measured product, nothing written (0 honored)",
            bytes.len()
        );
        std::process::exit(1);
    }
    let asset = tab_spec
        .rsplit('/')
        .next()
        .unwrap_or("acs_ec.tab")
        .to_ascii_lowercase();
    let out_path = format!("data/{netloc}/pds4_acs_nir/{asset}");
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bytes).is_err() {
        eprintln!("write {out_path} returned void");
        std::process::exit(1);
    }
    eprintln!(
        "{out_path}: {} byte(s), sha256 {sha} — the raw EC asset stands",
        bytes.len()
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{netloc}/{asset}");
    println!("format pds4_acs_nir");
    println!("ttl 604800");
    if ci_mode && !upload_release(&netloc, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        std::process::exit(1);
    }
}
