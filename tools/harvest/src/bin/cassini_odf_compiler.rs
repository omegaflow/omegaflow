use omegaflow::archivar::{embedded_lsk, fetch_raw_bytes};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const DATA: &str = "https://atmos.nmsu.edu/pdsd/archive/data/";
const NETLOC: &str = "atmos.nmsu.edu";
const UNIX_1950_OFFSET: f64 = 631152000.0;
const REQUEST_TTL_S: u64 = 604800;

fn hrefs(text: &str) -> Vec<String> {
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        let Some(end) = low[start..].find('"') else {
            break;
        };
        out.push(text[start..start + end].to_string());
        from = start + end;
    }
    out
}

fn volumes() -> Vec<String> {
    let Some(bytes) = fetch_raw_bytes(DATA, REQUEST_TTL_S) else {
        eprintln!("rss volume listing fetch void ({DATA})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("rss volume listing not utf8");
        return Vec::new();
    };
    let mut out: Vec<String> = hrefs(text)
        .into_iter()
        .filter(|h| h.ends_with('/'))
        .map(|h| {
            h.trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .filter(|n| n.starts_with("co-ss-rss-1-"))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn crawl(url: &str, depth: u32, files: &mut Vec<String>) {
    if depth > 6 {
        return;
    }
    let Some(bytes) = fetch_raw_bytes(url, REQUEST_TTL_S) else {
        eprintln!("{url}: listing fetch void");
        return;
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{url}: listing not utf8");
        return;
    };
    for name in hrefs(text) {
        if name.starts_with('?') || name.starts_with('/') || name == "../" {
            continue;
        }
        if name.to_ascii_lowercase().ends_with(".odf") {
            files.push(format!("{url}{name}"));
            continue;
        }
        if name.ends_with('/') {
            let dir = name.trim_end_matches('/').to_ascii_lowercase();
            if matches!(
                dir.as_str(),
                "index" | "calib" | "catalog" | "document" | "errata"
            ) {
                continue;
            }
            crawl(&format!("{url}{name}"), depth + 1, files);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let vols = volumes();
    eprintln!("cassini rss odf volumes: {}", vols.len());
    let mut files: Vec<String> = Vec::new();
    for vol in &vols {
        crawl(&format!("{DATA}{vol}/"), 0, &mut files);
    }
    files.sort();
    files.dedup();
    eprintln!("cassini rss odf files: {}", files.len());
    let mut merged: Vec<[f64; 9]> = Vec::new();
    for url in &files {
        let Some(bytes) = fetch_raw_bytes(url, REQUEST_TTL_S) else {
            eprintln!("{url}: fetch void");
            continue;
        };
        let Some(recs) = odf::parse_odf(&bytes) else {
            eprintln!("{url}: parse void — {} B", bytes.len());
            continue;
        };
        let mut kept = 0usize;
        let mut skipped = 0usize;
        for r in &recs {
            let doppler = (11..=14).contains(&r.data_type);
            if !r.valid || !doppler {
                skipped += 1;
                continue;
            }
            let unix = r.t_since_1950 - UNIX_1950_OFFSET;
            let Some(tdb) = lsk.unix_to_tdb(unix) else {
                skipped += 1;
                continue;
            };
            merged.push([
                tdb,
                r.observable_hz,
                r.ref_hz,
                r.dss_rx as f64,
                r.dss_tx as f64,
                r.data_type as f64,
                r.downlink_band as f64,
                r.scid as f64,
                r.compression_s,
            ]);
            kept += 1;
        }
        eprintln!(
            "{url}: {} orbit records, {kept} kept ({skipped} discarded)",
            recs.len()
        );
    }
    if merged.is_empty() {
        eprintln!("no Cassini ODF orbit samples — the series stays unwritten (0 honored)");
        return;
    }
    merged.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let out = "data/atmos.nmsu.edu/cassini_odf.bin";
    std::fs::create_dir_all("data/atmos.nmsu.edu").ok();
    let bin = odf::write_podf_bin(&merged);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match odf::parse_podf_bin(&bin) {
        Some(parsed) => {
            let d0 = parsed[0];
            let d1 = parsed[parsed.len() - 1];
            let mut stations: Vec<i64> = parsed.iter().map(|r| r[3] as i64).collect();
            stations.sort_unstable();
            stations.dedup();
            let mut dts: Vec<i64> = parsed.iter().map(|r| r[5] as i64).collect();
            dts.sort_unstable();
            dts.dedup();
            eprintln!(
                "{out}: {} orbit samples (tdb {}..{}), stations {stations:?}, data_type {dts:?}, {} B — roundtrip parses",
                parsed.len(),
                d0[0],
                d1[0],
                bin.len()
            );
        }
        None => eprintln!("{out}: roundtrip parse void — the series stays unverified"),
    }
    if ci_mode && !upload_release(NETLOC, out) {
        std::process::exit(1);
    }
}
