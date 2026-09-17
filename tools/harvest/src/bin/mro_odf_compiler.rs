use omegaflow::archivar::{LeapSeconds, embedded_lsk, fetch_raw_bytes, http_code};
use omegaflow::cdn::upload_release;
use omegaflow::odf;

const BASE: &str = "https://pds-geosciences.wustl.edu/mro/mro-m-rss-1-magr-v1/mrors_0xxx/odf/";
const UNIX_1950_OFFSET: f64 = 631152000.0;
const REQUEST_TTL_S: u64 = 1 << 9;
const WORKERS: usize = 1 << 3;
const PREFIX: &str = "mro_odf";

fn fetch_listing(dir: &str) -> Option<Vec<u8>> {
    match http_code(dir, &[]) {
        Some(code) if (200..300).contains(&code) => fetch_raw_bytes(dir, REQUEST_TTL_S),
        Some(code) => {
            eprintln!("{dir}: http {code} — no listing");
            None
        }
        None => {
            eprintln!("{dir}: unreachable — no listing");
            None
        }
    }
}

fn files_of() -> Vec<String> {
    let Some(bytes) = fetch_listing(BASE) else {
        eprintln!("odf dir listing fetch void ({BASE})");
        return Vec::new();
    };
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("odf dir listing not utf8");
        return Vec::new();
    };
    let low = text.to_ascii_lowercase();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(p) = low[from..].find("href=\"") {
        let start = from + p + 6;
        let Some(end) = low[start..].find('"') else {
            break;
        };
        let name = &text[start..start + end];
        if name.to_ascii_lowercase().ends_with(".odf") {
            out.push(name.rsplit('/').next().unwrap_or("").to_string());
        }
        from = start + end;
    }
    out.sort();
    out.dedup();
    out
}

fn harvest(url: &str, lsk: &LeapSeconds) -> Vec<[f64; 9]> {
    let Some(bytes) = fetch_raw_bytes(url, REQUEST_TTL_S) else {
        eprintln!("{url}: fetch void");
        return Vec::new();
    };
    let Some(recs) = odf::parse_odf(&bytes) else {
        eprintln!("{url}: parse void — {} B", bytes.len());
        return Vec::new();
    };
    let mut rows: Vec<[f64; 9]> = Vec::new();
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
        rows.push([
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
    }
    eprintln!(
        "{url}: {} orbit records, {} kept ({skipped} discarded)",
        recs.len(),
        rows.len()
    );
    rows
}

fn harvest_stream<F: FnMut(Vec<[f64; 9]>)>(urls: &[String], lsk: &LeapSeconds, mut on_rows: F) {
    for chunk in urls.chunks(WORKERS) {
        let mut chunk_rows: Vec<Vec<[f64; 9]>> = std::thread::scope(|s| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|url| s.spawn(move || harvest(url, lsk)))
                .collect();
            handles
                .into_iter()
                .map(|h| match h.join() {
                    Ok(rows) => rows,
                    Err(_) => {
                        eprintln!("a harvest worker stayed unjoined");
                        Vec::new()
                    }
                })
                .collect()
        });
        for mut rows in chunk_rows.drain(..) {
            rows.sort_by(|a, b| a[0].total_cmp(&b[0]));
            on_rows(rows);
        }
    }
}

fn flush_shard(buffer: &mut Vec<[f64; 9]>, names: &mut Vec<String>, paths: &mut Vec<String>) {
    let t_lo = buffer[0][0];
    let t_hi = buffer[buffer.len() - 1][0];
    let mut name = odf::podf_shard_name(PREFIX, t_lo, t_hi);
    if names.contains(&name) {
        name = odf::podf_shard_name_ord(PREFIX, t_lo, t_hi, names.len());
    }
    let bin = odf::write_podf_bin(buffer);
    assert!(bin.len() <= odf::PODF_SHARD_LIMIT);
    let parsed = match odf::parse_podf_bin(&bin) {
        Some(p) => p,
        None => {
            eprintln!("{name}: roundtrip parse void — the series stays unverified");
            std::process::exit(1);
        }
    };
    let d0 = parsed[0];
    let d1 = parsed[parsed.len() - 1];
    eprintln!(
        "{name}: {} orbit samples (tdb {}..{}), {} B — roundtrip parses",
        parsed.len(),
        d0[0],
        d1[0],
        bin.len()
    );
    let path = format!("data/pds-geosciences.wustl.edu/{name}");
    if std::fs::write(&path, &bin).is_err() {
        eprintln!("write {path} void");
        std::process::exit(1);
    }
    names.push(name);
    paths.push(path);
    buffer.clear();
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(lsk) = embedded_lsk() else {
        eprintln!("naif0012 table void — the series stays unwritten (0 honored)");
        return;
    };
    let rels = files_of();
    eprintln!("mro-m-rss-1-magr-v1/mrors_0xxx/odf: {} files", rels.len());
    let urls: Vec<String> = rels.iter().map(|rel| format!("{BASE}{rel}")).collect();
    std::fs::create_dir_all("data/pds-geosciences.wustl.edu").ok();

    let shard_records = (odf::PODF_SHARD_BUDGET - 8) / 72;
    let mut buffer: Vec<[f64; 9]> = Vec::with_capacity(shard_records);
    let mut names: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();
    let mut total = 0usize;
    harvest_stream(&urls, &lsk, |rows| {
        total += rows.len();
        for r in rows {
            buffer.push(r);
            if buffer.len() >= shard_records {
                flush_shard(&mut buffer, &mut names, &mut paths);
            }
        }
    });
    if total == 0 {
        eprintln!("no MRO ODF orbit samples — the series stays unwritten (0 honored)");
        return;
    }
    if !buffer.is_empty() {
        flush_shard(&mut buffer, &mut names, &mut paths);
    }
    if names.len() == 1 {
        let single = format!("data/pds-geosciences.wustl.edu/{PREFIX}.bin");
        if std::fs::rename(&paths[0], &single).is_err() {
            eprintln!("rename {} -> {single} void", paths[0]);
            std::process::exit(1);
        }
        paths[0] = single;
        if ci_mode && !upload_release("pds-geosciences.wustl.edu", &paths[0]) {
            std::process::exit(1);
        }
        return;
    }
    for name in &names {
        println!(
            "url https://github.com/omegaflow/sources/releases/download/pds-geosciences.wustl.edu/{name}"
        );
        println!("format {PREFIX}");
        println!("at earth");
        println!("ttl 604800");
        println!("field observable_hz {PREFIX}_observable_hz inverse-square em Hz 604800 0.0 0.0");
        println!();
    }
    if ci_mode {
        for path in &paths {
            if !upload_release("pds-geosciences.wustl.edu", path) {
                std::process::exit(1);
            }
        }
    }
}
