use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use omegaflow::sha256::sha256_hex;

#[derive(Clone, Copy)]
enum Kind {
    JsonRecord,
    Pdf,
    Csv,
    Text,
    Html,
}

struct Target {
    label: &'static str,
    url: &'static str,
    kind: Kind,
    note: &'static str,
}

const TARGETS: &[Target] = &[
    Target {
        label: "qcrf-zenodo-record",
        url: "https://zenodo.org/api/records/10834955",
        kind: Kind::JsonRecord,
        note: "QCRF 2021, Krasna+2024; machine-readable metadata only",
    },
    Target {
        label: "qcrf-pdf",
        url: "https://zenodo.org/api/records/10834955/files/Krasna_etal_IVSGM2024_QCRF.pdf/content",
        kind: Kind::Pdf,
        note: "the only file in the QCRF record",
    },
    Target {
        label: "qcrf-2024-poster-record",
        url: "https://zenodo.org/api/records/13330088",
        kind: Kind::JsonRecord,
        note: "IAU GA 2024 status poster",
    },
    Target {
        label: "qcrf-2024-poster-pdf",
        url: "https://zenodo.org/api/records/13330088/files/Krasna_IAUGA2024_QCRF_poster.pdf/content",
        kind: Kind::Pdf,
        note: "second QCRF record, poster only",
    },
    Target {
        label: "kcrf-zenodo-record",
        url: "https://zenodo.org/api/records/10893171",
        kind: Kind::JsonRecord,
        note: "KCRF 2024, de Witt+2024",
    },
    Target {
        label: "kcrf-pdf",
        url: "https://zenodo.org/api/records/10893171/files/K-CRF_IVS-GM-2024_v3.pdf/content",
        kind: Kind::Pdf,
        note: "the only file in the KCRF 2024 record",
    },
    Target {
        label: "kcrf-2023-arxiv",
        url: "https://arxiv.org/abs/2306.09747v1",
        kind: Kind::Html,
        note: "K-band CRF paper, 20 years of sessions",
    },
    Target {
        label: "crf-k-2025-arxiv",
        url: "https://arxiv.org/abs/2605.27629v1",
        kind: Kind::Html,
        note: "CRF-K-2025, 1317 sources, paper only until a table appears",
    },
    Target {
        label: "vizier-lanyi-q",
        url: "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT%20*%20FROM%20%22J%2FAJ%2F139%2F1695%2Ftable3%22",
        kind: Kind::Csv,
        note: "Lanyi+2010 Q-band positions, 131 sources",
    },
    Target {
        label: "vizier-qcal1-flux",
        url: "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT%20*%20FROM%20%22J%2FAJ%2F144%2F150%2Ftable3%22",
        kind: Kind::Csv,
        note: "QCAL-1 flux densities, 637 rows",
    },
    Target {
        label: "vizier-qcal1-pos",
        url: "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT%20*%20FROM%20%22J%2FAJ%2F144%2F150%2Ftable4%22",
        kind: Kind::Csv,
        note: "QCAL-1 KVN positions, 33 rows",
    },
    Target {
        label: "lanyi-readme",
        url: "https://cdsarc.cds.unistra.fr/ftp/cats/J/AJ/139/1695/ReadMe",
        kind: Kind::Text,
        note: "declares table3.dat 131 rows at 43 GHz",
    },
    Target {
        label: "qcal1-readme",
        url: "https://cdsarc.cds.unistra.fr/ftp/cats/J/AJ/144/150/ReadMe",
        kind: Kind::Text,
        note: "declares table3 637 rows and table4 33 rows",
    },
    Target {
        label: "usno-crf-global-solutions",
        url: "https://crf.usno.navy.mil/global-solutions-crf",
        kind: Kind::Html,
        note: "USNO global CRF: S/X and K only, no Q solution",
    },
    Target {
        label: "usno-sx-source-positions",
        url: "https://crf.usno.navy.mil/data_products/RORFD/Quarterly/current/USNO_source_positions.iers",
        kind: Kind::Text,
        note: "USNO quarterly S/X positions, not a Q catalog",
    },
    Target {
        label: "bvid-database",
        url: "https://bvid.astrophy.u-bordeaux.fr/database.html",
        kind: Kind::Html,
        note: "BVID images, includes Q-band sessions RV146/RV148/RV150",
    },
    Target {
        label: "ivs-crf-products",
        url: "https://ivscc.gsfc.nasa.gov/products-data/products.html",
        kind: Kind::Html,
        note: "IVS CRF product description and data-center pointers",
    },
    Target {
        label: "ivs-bkg-crf-dir",
        url: "https://ivs.bkg.bund.de/data_dir/vlbi/ivsproducts/crf/",
        kind: Kind::Html,
        note: "IVS BKG CRF directory; connection refused on 2026-10-21",
    },
    Target {
        label: "cddis-crf-dir",
        url: "https://cddis.nasa.gov/archive/vlbi/ivsproducts/crf/",
        kind: Kind::Html,
        note: "CDDIS CRF directory behind Earthdata Login",
    },
    Target {
        label: "vievs-wiki",
        url: "https://vievswiki.geo.tuwien.ac.at/",
        kind: Kind::Html,
        note: "VieVS documentation entry point",
    },
    Target {
        label: "vievs-github",
        url: "https://github.com/TUW-VieVS/VLBI",
        kind: Kind::Html,
        note: "VieVS VLBI module source, GPL-3.0",
    },
    Target {
        label: "calc-solve-gsfc",
        url: "https://gemini.gsfc.nasa.gov/solve/",
        kind: Kind::Html,
        note: "Calc/Solve page; timed out on 2026-10-21",
    },
    Target {
        label: "aips",
        url: "https://www.aips.nrao.edu/",
        kind: Kind::Html,
        note: "AIPS, classic VLBI calibration and imaging package",
    },
    Target {
        label: "difmap-github",
        url: "https://github.com/sabourke/difmap",
        kind: Kind::Html,
        note: "Difmap source mirror; original Caltech page is 404",
    },
    Target {
        label: "casa",
        url: "https://casa.nrao.edu/",
        kind: Kind::Html,
        note: "CASA, open source, source on Bitbucket",
    },
    Target {
        label: "hops",
        url: "https://www.haystack.mit.edu/hops/",
        kind: Kind::Html,
        note: "HOPS post-processing, includes PIMA data import",
    },
];

struct Fetch {
    label: &'static str,
    url: &'static str,
    kind: Kind,
    note: &'static str,
    path: PathBuf,
    status: Option<u16>,
    bytes: usize,
    sha256: String,
    content_type: Option<String>,
    last_modified: Option<String>,
    origin: &'static str,
    body: Option<Vec<u8>>,
}

fn header_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name.trim().eq_ignore_ascii_case(key) {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

fn curl(url: &str, body_path: &Path, hdr_path: &Path, timeout_sec: u64) -> Option<u16> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-m")
        .arg(timeout_sec.to_string())
        .arg("-A")
        .arg("omegaflow-qcrf-twin-probe/1.0")
        .arg("-D")
        .arg(hdr_path)
        .arg("-o")
        .arg(body_path)
        .arg("-w")
        .arg("%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!("curl: exit {:?} for {}", out.status.code(), url);
        return None;
    }
    String::from_utf8_lossy(&out.stdout).trim().parse().ok()
}

fn fetch(target: &Target, cache_root: &Path, offline: bool, timeout_sec: u64) -> Fetch {
    let safe = target.label.replace('/', "_");
    let path = cache_root.join(safe);
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            eprintln!("cache: cannot create {}", parent.display());
        }
    }
    let part = path.with_extension("part");
    let hdr_path = path.with_extension("hdr");
    let mut status = None;
    let mut origin = "cache";
    if !offline {
        origin = "http";
        match curl(target.url, &part, &hdr_path, timeout_sec) {
            Some(code) => {
                status = Some(code);
                if fs::rename(&part, &path).is_err() {
                    eprintln!("cache: cannot move {}", part.display());
                }
            }
            None => {
                let _ = fs::remove_file(&part);
                eprintln!("fetch: no HTTP answer for {}", target.url);
                origin = "cache";
            }
        }
    }
    let body = fs::read(&path).ok();
    let hdr = match fs::read_to_string(&hdr_path) {
        Ok(text) => text,
        Err(_) => String::new(),
    };
    let bytes = body.as_ref().map_or(0, |b| b.len());
    let sha256 = match body.as_ref().map(|b| sha256_hex(b)) {
        Some(value) => value,
        None => "absent".to_string(),
    };
    Fetch {
        label: target.label,
        url: target.url,
        kind: target.kind,
        note: target.note,
        path,
        status,
        bytes,
        sha256,
        content_type: header_value(&hdr, "content-type"),
        last_modified: header_value(&hdr, "last-modified"),
        origin,
        body,
    }
}

fn sniff(body: &[u8]) -> &'static str {
    if body.starts_with(b"%PDF-") {
        return "pdf";
    }
    let head = String::from_utf8_lossy(&body[..body.len().min(512)]);
    let trimmed = head.trim_start();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        return "json";
    }
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("<!doctype") || lower.starts_with("<html") {
        return "html";
    }
    if trimmed.starts_with("table_name,") {
        return "csv";
    }
    if body.iter().any(|b| *b == 0) {
        return "binary";
    }
    "text"
}

fn csv_fields(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for c in line.chars() {
        match c {
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

fn json_string_field(text: &str, field: &str) -> Vec<String> {
    let mut out = Vec::new();
    let needle = format!("\"{field}\"");
    for part in text.split(&needle).skip(1) {
        let Some(colon) = part.find(':') else {
            continue;
        };
        let value = part[colon + 1..].trim_start();
        let Some(quoted) = value.strip_prefix('"') else {
            continue;
        };
        if let Some(end) = quoted.find('"') {
            out.push(quoted[..end].to_string());
        }
    }
    out
}

fn json_keys(text: &str) -> Vec<String> {
    json_string_field(text, "key")
}

fn json_license(text: &str) -> Option<String> {
    let at = text.find("\"license\"")?;
    let tail = &text[at..text.len().min(at + 400)];
    let id_at = tail.find("\"id\"")?;
    let rest = &tail[id_at + 4..];
    let colon = rest.find(':')?;
    let quoted = rest[colon + 1..].trim_start();
    let quoted = quoted.strip_prefix('"')?;
    Some(quoted.split('"').next()?.to_string())
}

fn pdf_pages(path: &Path) -> Option<usize> {
    let out = Command::new("pdfinfo").arg(path).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    text.lines().find_map(|line| {
        let value = line.strip_prefix("Pages:")?;
        value.trim().parse().ok()
    })
}

fn pdf_text_lines(path: &Path) -> Option<(usize, usize)> {
    let out = Command::new("pdftotext").arg(path).arg("-").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let lines = text.lines().count();
    let source_rows = text
        .lines()
        .filter(|line| {
            let t = line.trim_start();
            let b = t.as_bytes();
            b.len() > 6
                && b[0] == b'J'
                && b[1..5].iter().all(|c| c.is_ascii_digit())
                && (b[5] == b'+' || b[5] == b'-')
        })
        .count();
    Some((lines, source_rows))
}

fn marker_count(text: &str, needle: &str) -> usize {
    text.to_ascii_lowercase().matches(needle).count()
}

fn report(f: &Fetch) {
    println!("[{}] {}", f.label, f.url);
    println!("  note: {}", f.note);
    let body = match &f.body {
        Some(body) => body,
        None => {
            println!(
                "  absent | status {} | origin {} | no body and no cache",
                match f.status {
                    Some(code) => code.to_string(),
                    None => "none".to_string(),
                },
                f.origin
            );
            return;
        }
    };
    println!(
        "  origin {} | status {} | bytes {} | sha256 {} | content-type {} | last-modified {}",
        f.origin,
        match f.status {
            Some(code) => code.to_string(),
            None => "cache-only".to_string(),
        },
        f.bytes,
        f.sha256,
        f.content_type.as_deref().unwrap_or("absent"),
        f.last_modified.as_deref().unwrap_or("absent")
    );
    let mut format = sniff(body);
    if matches!(f.kind, Kind::Csv) && format == "text" {
        format = "csv";
    }
    if matches!(f.kind, Kind::Html) && format == "text" {
        format = "html";
    }
    print!("  format {format}");
    match (f.kind, format) {
        (Kind::JsonRecord, _) => {
            let text = String::from_utf8_lossy(body);
            let keys = json_keys(&text);
            println!(
                " | files {} | license {}",
                keys.len(),
                json_license(&text).as_deref().unwrap_or("absent")
            );
            for key in &keys {
                println!("    file {key}");
            }
        }
        (Kind::Csv, _) => {
            let text = String::from_utf8_lossy(body);
            let mut lines = text.lines().filter(|l| !l.trim().is_empty());
            let header = match lines.next().map(csv_fields) {
                Some(value) => value,
                None => Vec::new(),
            };
            let rows = lines.count();
            println!(
                " | rows {rows} | columns {} | header {}",
                header.len(),
                header.join(",")
            );
        }
        (Kind::Pdf, _) => {
            let pages = pdf_pages(&f.path);
            let text = pdf_text_lines(&f.path);
            println!(
                " | pages {} | text-lines {} | source-rows {}",
                match pages {
                    Some(n) => n.to_string(),
                    None => "unmeasured".to_string(),
                },
                match text {
                    Some((n, _)) => n.to_string(),
                    None => "unmeasured".to_string(),
                },
                match text {
                    Some((_, n)) => n.to_string(),
                    None => "unmeasured".to_string(),
                }
            );
        }
        (Kind::Html, _) | (Kind::Text, _) => {
            let text = String::from_utf8_lossy(body);
            println!(
                " | lines {} | q-band {} | 43ghz {} | celestial {}",
                text.lines().count(),
                marker_count(&text, "q-band") + marker_count(&text, "q band"),
                marker_count(&text, "43 ghz"),
                marker_count(&text, "celestial reference frame")
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cache_root = match args.iter().position(|a| a == "--cache") {
        Some(i) => match args.get(i + 1) {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("/tmp/opencode/qcrf-twin-probe"),
        },
        None => PathBuf::from("/tmp/opencode/qcrf-twin-probe"),
    };
    let offline = args.iter().any(|a| a == "--offline");
    let timeout_sec = match args.iter().position(|a| a == "--timeout") {
        Some(i) => args
            .get(i + 1)
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(40),
        None => 40,
    };
    let only = args.iter().position(|a| a == "--only").and_then(|i| args.get(i + 1));
    let mut answered = 0usize;
    let mut absent = 0usize;
    for target in TARGETS {
        if let Some(filter) = only {
            if !target.label.contains(filter.as_str()) {
                continue;
            }
        }
        let f = fetch(target, &cache_root, offline, timeout_sec);
        if f.body.is_some() {
            answered += 1;
        } else {
            absent += 1;
        }
        report(&f);
    }
    println!(
        "summary | targets {} | answered {answered} | absent {absent} | cache {} | offline {offline}",
        answered + absent,
        cache_root.display()
    );
}
