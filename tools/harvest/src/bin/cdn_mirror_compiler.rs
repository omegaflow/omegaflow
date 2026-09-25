use omegaflow::archivar::fetch::{fetch_raw_bytes, live_markers};
use omegaflow::archivar::naming::{cdn_manifest_map, extract_netloc, source_name_from_url};
use omegaflow::cdn::upload_release;
use omegaflow::json::parse_json;

fn mirror_name(url: &str) -> String {
    match cdn_manifest_map().get(url) {
        Some(name) => name.clone(),
        None => source_name_from_url(url),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let mut json_urls: Vec<String> = Vec::new();
    let mut xml_urls: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--url" => {
                if let Some(u) = args.get(i + 1) {
                    json_urls.push(u.clone());
                }
                i += 1;
            }
            "--xml-url" => {
                if let Some(u) = args.get(i + 1) {
                    xml_urls.push(u.clone());
                }
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    if json_urls.is_empty() && xml_urls.is_empty() {
        eprintln!("cdn_mirror: --url/--xml-url absent");
        std::process::exit(1);
    }
    let markers = live_markers();
    let mut voids = 0usize;
    let mut mirrored = 0usize;
    for (url, xml) in json_urls
        .iter()
        .map(|u| (u, false))
        .chain(xml_urls.iter().map(|u| (u, true)))
    {
        let mut resolved = url.clone();
        for (k, v) in &markers {
            resolved = resolved.replace(k, v);
        }
        let Some(netloc) = extract_netloc(&resolved) else {
            eprintln!("cdn_mirror {url}: netloc void — skipped");
            voids += 1;
            continue;
        };
        let name = mirror_name(&resolved);
        if name.is_empty() {
            eprintln!("cdn_mirror {url}: name void — skipped");
            voids += 1;
            continue;
        }
        let Some(body) = fetch_raw_bytes(&resolved) else {
            eprintln!("cdn_mirror {netloc}/{name}: live body void — asset untouched");
            voids += 1;
            continue;
        };
        let text = String::from_utf8_lossy(&body);
        let plausible = if xml {
            !body.is_empty() && text.contains("quakeml")
        } else {
            !body.is_empty() && parse_json(&text).is_some()
        };
        if !plausible {
            eprintln!(
                "cdn_mirror {netloc}/{name}: live body does not carry the registered format — asset untouched"
            );
            voids += 1;
            continue;
        }
        let path = format!("{name}.json");
        if std::fs::write(&path, &body).is_err() {
            eprintln!("cdn_mirror {netloc}/{name}: local write void");
            voids += 1;
            continue;
        }
        if ci_mode && !upload_release(netloc, &path) {
            eprintln!("cdn_mirror {netloc}/{name}: release upload void");
            voids += 1;
            continue;
        }
        eprintln!("cdn_mirror {netloc}/{name}: mirrored {} bytes", body.len());
        mirrored += 1;
    }
    eprintln!("cdn_mirror: mirrored {mirrored}, voids {voids}");
    if voids > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::mirror_name;

    #[test]
    fn mirror_name_reads_the_runtime_derivation() {
        assert_eq!(
            mirror_name("https://api.vedur.is/quakes/events"),
            "quakes-events"
        );
        assert_eq!(
            mirror_name("https://api.p2pquake.net/v2/jma/quake?limit=100&order=-1"),
            "v2-jma-quake-limit-100-order--1"
        );
        assert_eq!(
            mirror_name("https://api.wolfx.jp/jma_eew.json"),
            "jma_eew.json"
        );
        assert_eq!(
            mirror_name("https://services.swpc.noaa.gov/products/noaa-planetary-k-index.json"),
            "products-noaa-planetary-k-index.json"
        );
    }
}
