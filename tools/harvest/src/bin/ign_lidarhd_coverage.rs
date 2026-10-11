use omegaflow::archivar::{RetryPolicy, TRANSFER_BOUND_S, fetch_raw_with};
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::env;
use std::fs;
use std::path::Path;

const MANIFEST_NAME: &str = "ign_lidarhd_coverage.manifest";
const COMPILER: &str = "tools/harvest/src/bin/ign_lidarhd_coverage.rs";
const NETLOC: &str = "data.geopf.fr";
const WFS: &str = "https://data.geopf.fr/wfs/ows";
const LAYER: &str = "IGNF_LIDAR-HD_METADONNEE:metadata";
const PAGE_SIZE: usize = 600;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn listing_fetch(url: &str) -> Option<String> {
    fetch_raw_with(url, None, &[], RetryPolicy::All, TRANSFER_BOUND_S)
}

fn wfs_url(start_index: usize, count: usize, bbox: Option<&str>) -> String {
    let mut url = format!(
        "{WFS}?SERVICE=WFS&VERSION=2.0.0&REQUEST=GetFeature&TYPENAMES={LAYER}&COUNT={count}&STARTINDEX={start_index}&OUTPUTFORMAT=application/json"
    );
    if let Some(b) = bbox {
        url.push_str("&BBOX=");
        url.push_str(b);
        url.push_str(",urn:ogc:def:crs:EPSG::4326");
    }
    url
}

fn json_string_at(body: &str, after_key: usize) -> Option<String> {
    let bytes = body.as_bytes();
    let mut i = after_key;
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\r' | b'\n') {
        i += 1;
    }
    if bytes.get(i) != Some(&b':') {
        return None;
    }
    i += 1;
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\r' | b'\n') {
        i += 1;
    }
    if bytes.get(i) != Some(&b'"') {
        return None;
    }
    let start = i + 1;
    let end = body[start..].find('"').map(|p| start + p)?;
    Some(body[start..end].to_string())
}

fn collect_json_strings(body: &str, key: &str) -> Vec<String> {
    let pat = format!("\"{key}\"");
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = body[from..].find(&pat) {
        let key_at = from + rel;
        if let Some(v) = json_string_at(body, key_at + pat.len()) {
            out.push(v);
        }
        from = key_at + pat.len();
    }
    out
}

fn json_number_after(body: &str, key: &str) -> Option<u64> {
    let pat = format!("\"{key}\"");
    let at = body.find(&pat)?;
    let bytes = body.as_bytes();
    let mut i = at + pat.len();
    while i < bytes.len() && bytes[i] != b':' {
        i += 1;
    }
    if bytes.get(i) != Some(&b':') {
        return None;
    }
    i += 1;
    while i < bytes.len() && !bytes[i].is_ascii_digit() {
        if matches!(bytes[i], b',' | b'}' | b']') {
            return None;
        }
        i += 1;
    }
    let start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    body[start..i].parse::<u64>().ok()
}

fn is_lidar_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.ends_with(".copc.laz") || lower.ends_with(".laz") || lower.ends_with(".las")
}

fn manifest_lines(urls: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for u in urls {
        if !is_lidar_url(u) {
            continue;
        }
        out.push(format!("shard {u}"));
    }
    out
}

fn bbox_value(raw: &str) -> Result<String, String> {
    let parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
    if parts.len() != 4 {
        return Err(format!(
            "--bbox {raw}: expected south,west,north,east — refused"
        ));
    }
    for p in &parts {
        if p.parse::<f64>().is_err() {
            return Err(format!("--bbox {raw}: '{p}' is not a number — refused"));
        }
    }
    Ok(parts.join(","))
}

fn enumerate(
    bbox: Option<&str>,
    max_pages: Option<usize>,
) -> Result<(Vec<String>, usize, Option<u64>), String> {
    let mut urls = Vec::new();
    let mut pages = 0usize;
    let mut start_index = 0usize;
    let mut matched: Option<u64> = None;
    loop {
        let url = wfs_url(start_index, PAGE_SIZE, bbox);
        let body = listing_fetch(&url).ok_or_else(|| {
            format!(
                "WFS page {} at STARTINDEX {start_index} returned void ({url}) — the walk stays incomplete",
                pages + 1
            )
        })?;
        let page_matched = json_number_after(&body, "numberMatched");
        let returned = match json_number_after(&body, "numberReturned") {
            Some(n) => n,
            None => collect_json_strings(&body, "url_npl").len() as u64,
        };
        if pages == 0 {
            matched = page_matched;
        }
        pages += 1;
        urls.extend(collect_json_strings(&body, "url_npl"));
        if returned == 0 {
            break;
        }
        start_index += returned as usize;
        if let Some(m) = matched {
            if start_index as u64 >= m {
                break;
            }
        }
        if let Some(cap) = max_pages {
            if pages >= cap {
                break;
            }
        }
    }
    Ok((urls, pages, matched))
}

fn run(args: &[String]) -> Result<(), String> {
    let out = match arg_value(args, "--out").filter(|o| !o.is_empty()) {
        Some(o) => o,
        None => format!("data/{NETLOC}/{MANIFEST_NAME}"),
    };
    let out_name = match Path::new(&out).file_name().and_then(|n| n.to_str()) {
        Some(n) => n.to_string(),
        None => MANIFEST_NAME.to_string(),
    };
    let bbox = match arg_value(args, "--bbox").filter(|b| !b.is_empty()) {
        Some(raw) => Some(bbox_value(&raw)?),
        None => None,
    };
    let max_pages = arg_value(args, "--max-pages").and_then(|v| v.parse::<usize>().ok());
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let (urls, pages, matched) = enumerate(bbox.as_deref(), max_pages)?;
    if pages == 0 || urls.is_empty() {
        return Err(format!(
            "LiDAR HD over {pages} page(s): the WFS carries no url_npl — nothing manifestiert (0 honored)"
        ));
    }
    let lines = manifest_lines(&urls);
    if lines.is_empty() {
        return Err(format!(
            "LiDAR HD: {} url_npl over {pages} page(s), none a lidar key — nothing manifestiert (0 honored)",
            urls.len()
        ));
    }

    let mut text = String::with_capacity(lines.len() * 128);
    for line in &lines {
        text.push_str(line);
        text.push('\n');
    }
    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    fs::write(&out, &text).map_err(|e| format!("write {out} returned void: {e}"))?;
    let back = fs::read_to_string(&out).map_err(|_| format!("{out} re-read returned void"))?;
    if back.lines().count() != lines.len() {
        return Err(format!(
            "{out} roundtrip lists {} of {} lines — the manifest stays unverified",
            back.lines().count(),
            lines.len()
        ));
    }
    let matched_note = match matched {
        Some(m) => m.to_string(),
        None => "unmeasured".to_string(),
    };
    eprintln!(
        "ign_lidarhd_coverage: {pages} page(s), {} url_npl, {} lidar tiles of {matched_note} matched -> {out}",
        urls.len(),
        lines.len()
    );

    println!("url {CDN_BASE}/{NETLOC}/{out_name}");
    println!("origin {WFS}?SERVICE=WFS&VERSION=2.0.0&REQUEST=GetFeature&TYPENAMES={LAYER}");
    println!("compiler {COMPILER}");
    println!("format ign_lidarhd_coverage");

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out} did not reach the CDN"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ign_lidarhd_coverage: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"{"type":"FeatureCollection","numberMatched":524687,"numberReturned":600,"features":[{"type":"Feature","id":"metadata.10616","properties":{"url_npl":"https://data.geopf.fr/telechargement/download/LiDARHD-NUALID/NUALHD_1-0__LAZ_LAMB93_KE_2025-06-06/LHD_FXX_0651_6863_PTS_LAMB93_IGN69.copc.laz","nombre_points":14553289,"date_debut_acquisition":"2023-03-03Z","date_fin_acquisition":"2023-03-03Z"}},{"type":"Feature","id":"metadata.11036","properties":{"url_npl":"https://data.geopf.fr/telechargement/download/LiDARHD-NUALID/NUALHD_1-0__LAZ_LAMB93_KE_2025-06-06/LHD_FXX_0650_6861_PTS_LAMB93_IGN69.copc.laz","nombre_points":18161309,"date_debut_acquisition":"2023-03-03Z","date_fin_acquisition":"2023-03-03Z"}}]}"#;

    #[test]
    fn url_npl_values_become_shard_lines_on_an_absolute_url() {
        let urls = collect_json_strings(PAGE, "url_npl");
        assert_eq!(urls.len(), 2);
        let lines = manifest_lines(&urls);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "shard https://data.geopf.fr/telechargement/download/LiDARHD-NUALID/NUALHD_1-0__LAZ_LAMB93_KE_2025-06-06/LHD_FXX_0651_6863_PTS_LAMB93_IGN69.copc.laz"
        );
        assert_eq!(
            lines[1],
            "shard https://data.geopf.fr/telechargement/download/LiDARHD-NUALID/NUALHD_1-0__LAZ_LAMB93_KE_2025-06-06/LHD_FXX_0650_6861_PTS_LAMB93_IGN69.copc.laz"
        );
    }

    #[test]
    fn page_envelope_is_read() {
        assert_eq!(json_number_after(PAGE, "numberMatched"), Some(524687));
        assert_eq!(json_number_after(PAGE, "numberReturned"), Some(600));
    }

    #[test]
    fn non_lidar_urls_are_left_out() {
        let body = r#"{"features":[{"properties":{"url_npl":"https://data.geopf.fr/wms-r?LAYERS=IGNF_LIDAR-HD_MNT_ELEVATION&FORMAT=image/geotiff"}},{"properties":{"url_npl":"https://data.geopf.fr/telechargement/download/x/LHD_FXX_0651_6863_MNT_O_0M50.tif"}}]}"#;
        let urls = collect_json_strings(body, "url_npl");
        assert_eq!(urls.len(), 2);
        assert!(manifest_lines(&urls).is_empty());
    }

    #[test]
    fn wfs_url_carries_pagination_and_bbox() {
        let url = wfs_url(600, 600, Some("48.84,2.33,48.86,2.37"));
        assert!(url.contains("STARTINDEX=600"));
        assert!(url.contains("COUNT=600"));
        assert!(url.contains("TYPENAMES=IGNF_LIDAR-HD_METADONNEE:metadata"));
        assert!(url.contains("BBOX=48.84,2.33,48.86,2.37,urn:ogc:def:crs:EPSG::4326"));
    }
}
