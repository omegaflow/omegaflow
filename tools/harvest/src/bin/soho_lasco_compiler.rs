use omegaflow::cdn::upload_release;
use omegaflow::soho_lasco::{parse_bin, parse_text, write_bin};

const BASE: &str = "https://cdaw.gsfc.nasa.gov/CME_list/UNIVERSAL_ver2/text_ver";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn url_for_month(month: &str) -> Option<String> {
    let (y, m) = month.split_once('_')?;
    if y.len() != 4 || m.len() != 2 || !y.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if !m.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some(format!("{BASE}/univ{}_{}.txt", y, m))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let month = arg_value(&args, "--month");
    let url = match arg_value(&args, "--url") {
        Some(u) => u,
        None => match month.as_deref() {
            Some(m) => {
                let Some(u) = url_for_month(m) else {
                    eprintln!("--month expects YYYY_MM");
                    std::process::exit(2);
                };
                u
            }
            None => {
                eprintln!("soho_lasco_compiler needs --url <url> or --month <YYYY_MM>");
                std::process::exit(2);
            }
        },
    };
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => match month.as_deref() {
            Some(m) => format!("soho_lasco_cme_{m}.bin"),
            None => "soho_lasco_cme.bin".to_string(),
        },
    };

    let Some(body) = omegaflow::archivar::fetch_raw(&url, None, &[]) else {
        eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let mut records = parse_text(&body);
    records.sort_by(|a, b| {
        a.t_unix
            .partial_cmp(&b.t_unix)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    eprintln!("SOHO/LASCO CME {url}: {} records", records.len());
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release("cdaw.gsfc.nasa.gov", &out) {
        std::process::exit(1);
    }
}
