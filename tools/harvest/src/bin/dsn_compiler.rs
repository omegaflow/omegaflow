use omegaflow::archivar::dsn;
use omegaflow::archivar::{embedded_lsk, fetch_raw};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::parse as parse_lsk;

const NETLOC: &str = "eyes.nasa.gov";
const URL: &str = "https://eyes.nasa.gov/dsn/data/dsn.xml";
const DEFAULT_OUT: &str = "data/eyes.nasa.gov/dsn_snapshot.bin";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], key: &str) -> bool {
    args.iter().any(|a| a == key)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = has_flag(args, "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(path) => path,
        None => DEFAULT_OUT.to_string(),
    };
    let lsk = match arg_value(args, "--lsk") {
        Some(path) => {
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("--lsk {path} reads void: {e}"))?;
            parse_lsk(&text).ok_or_else(|| format!("--lsk {path} carries no leap-second table"))?
        }
        None => embedded_lsk().ok_or_else(|| {
            "the embedded naif0012.tls stays unread — date→TDB unavailable".to_string()
        })?,
    };

    let body = match arg_value(args, "--xml") {
        Some(path) => {
            std::fs::read_to_string(&path).map_err(|e| format!("--xml {path} reads void: {e}"))?
        }
        None => fetch_raw(URL, None, &[]).ok_or_else(|| format!("{URL}: fetch void"))?,
    };

    let (epoch_unix, signals) = dsn::parse_xml(&body)
        .ok_or_else(|| format!("{URL}: no dish/signal left the parse — the bin stays unwritten"))?;
    let epoch_tdb = lsk
        .unix_to_tdb(epoch_unix)
        .ok_or_else(|| format!("{epoch_unix}: unix→TDB reads void"))?;
    let active = signals.iter().filter(|s| s.active).count();
    let records = dsn::records_at(epoch_tdb, signals);

    let bytes = dsn::write_bin(epoch_unix, &records);
    let parsed = dsn::parse_bin(&bytes)
        .ok_or_else(|| format!("{out}: roundtrip parse void — the asset stays unverified"))?;
    if parsed.records.len() != records.len() {
        return Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.records.len(),
            records.len()
        ));
    }
    let (rows, _) =
        dsn::series_parse(&bytes).ok_or_else(|| format!("{out}: series projection reads void"))?;

    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    eprintln!(
        "{out}: {} signals ({} active), {} series rows, {} B at epoch {epoch_unix} — roundtrip parses",
        parsed.records.len(),
        active,
        rows.len(),
        bytes.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("dsn_compiler: {msg}");
        std::process::exit(2);
    }
}
