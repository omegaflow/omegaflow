use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::{embedded_lsk, quaoar_occlt};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "zenodo.org";
const ZIP_URL: &str = "https://zenodo.org/api/records/21185812/files/Quaoar_paper.zip/content";
const DEFAULT_OUT: &str = "data/zenodo.org/quaoar_occlt.bin";
const FETCH_TTL: u64 = 86400;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out-bin") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let lsk = match embedded_lsk() {
        Some(l) => l,
        None => {
            eprintln!("embedded naif0012 parses void — the epoch conversion stays unread");
            std::process::exit(1);
        }
    };
    let zip = match fetch_raw_bytes(ZIP_URL, FETCH_TTL) {
        Some(b) => b,
        None => {
            eprintln!("{ZIP_URL}: fetch void");
            std::process::exit(1);
        }
    };
    let entries = match quaoar_occlt::zip_entries(&zip) {
        Some(e) => e,
        None => {
            eprintln!("{ZIP_URL}: central directory void — the zip stays unread");
            std::process::exit(1);
        }
    };
    let mut records = Vec::new();
    let mut curves = 0usize;
    let mut extracted = 0usize;
    for e in &entries {
        let Some(("lc", date, site)) = quaoar_occlt::parse_filename(&e.name) else {
            continue;
        };
        curves += 1;
        let Some(midnight) = quaoar_occlt::date_midnight_unix(date) else {
            eprintln!("{}: date carries no calendar day", e.name);
            continue;
        };
        let Some(raw) = quaoar_occlt::zip_extract(&zip, e) else {
            eprintln!("{}: entry extract void", e.name);
            continue;
        };
        let text = String::from_utf8_lossy(&raw);
        let Some(samples) = quaoar_occlt::parse_light_curve(&text, midnight, &lsk) else {
            continue;
        };
        extracted += 1;
        records.extend(quaoar_occlt::to_recs(
            &samples,
            quaoar_occlt::site_code(site),
        ));
    }
    if records.is_empty() {
        eprintln!(
            "{ZIP_URL}: {} light-curve members carry no measured sample — the bin stays unwritten (0 honored)",
            curves
        );
        std::process::exit(1);
    }
    records.sort_by(|a, b| {
        a.tdb
            .total_cmp(&b.tdb)
            .then(a.site.cmp(&b.site))
            .then(a.comp.cmp(&b.comp))
    });
    let bin = match quaoar_occlt::write_bin(&records) {
        Some(b) => b,
        None => {
            eprintln!("{out}: record encode void — the bin stays unwritten");
            std::process::exit(1);
        }
    };
    if let Some(parent) = std::path::Path::new(&out).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).ok();
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    match quaoar_occlt::parse_bin(&bin) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} records ({} light-curve members of {} found, {} B, {ZIP_URL}), roundtrip parses",
                records.len(),
                extracted,
                curves,
                bin.len()
            );
        }
        _ => {
            eprintln!("{out}: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}
