use super::*;

pub fn derive_frame(parsed: &JsonVal, coords: &str) -> (String, String) {
    if coords.contains("lat ") || coords.contains("lon ") {
        (
            "on earth 0 0\n".to_string(),
            "geographic coords".to_string(),
        )
    } else if coords.contains("ra ") || coords.contains("dec ") {
        ("at sun\n".to_string(), "celestial coords".to_string())
    } else if json_has_key_ci(parsed, "ra") && json_has_key_ci(parsed, "dec") {
        ("at sun\n".to_string(), "celestial ra/dec".to_string())
    } else {
        ("".to_string(), "frame pending".to_string())
    }
}




pub const CELESTIAL_NETLOCS: &[&str] = &[
    "tapvizier.cds.unistra.fr",
    "vizier.cds.unistra.fr",
    "cds.unistra.fr",
    "irsa.ipac.caltech.edu",
    "dc.g-vo.org",
    "gaia.ari.uni-heidelberg.de",
    "exoplanetarchive.ipac.caltech.edu",
    "heasarc.gsfc.nasa.gov",
    "simbad.u-strasbg.fr",
    "gea.esac.esa.int",
    "wis-tns.org",
    "ssd.jpl.nasa.gov",
    "ssd-api.jpl.nasa.gov",
    "naif.jpl.nasa.gov",
    "archive.stsci.edu",
    "mast.stsci.edu",
    "archive.gemini.edu",
    "archive.nrao.edu",
    "skyserver.sdss.org",
    "atnf.csiro.au",
    "noirlab.edu",
    "eso.org",
    "astrocats.space",
];




pub fn draft_frame_guess(
    url: &str,
    context: &str,
    registry: &HashMap<String, String>,
) -> (String, String) {
    let netloc = extract_netloc(url).unwrap_or_default();
    for key in route_prefix_keys(url) {
        if let Some(f) = registry.get(&key) {
            return (format!("{}\n", f), format!("route-registry: {}", f));
        }
    }
    for n in CELESTIAL_NETLOCS {
        if netloc == *n || netloc.ends_with(n) {
            return ("at sun\n".to_string(), "celestial netloc".to_string());
        }
    }
    let lower = context.to_lowercase();
    for w in [
        "station",
        "buoy",
        "quake",
        "earthquake",
        "weather",
        "wind",
        "temperature",
        "water",
        "tide",
        "sea ",
        "ocean",
        "snow",
        "rain",
        "seismic",
        "metar",
        "airport",
        "pegel",
        "air quality",
        "hurricane",
    ] {
        if lower.contains(w) {
            return (
                "on earth 0 0\n".to_string(),
                format!("terrestrial vocab: {}", w.trim()),
            );
        }
    }
    ("".to_string(), "frame pending".to_string())
}




pub fn build_frame_registry() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for path in [
        "phi/sources.φ",
        "phi/dead_sources.φ",
        "phi/blocked_sources.φ",
    ] {
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        let mut cur_url: Option<String> = None;
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url ") {
                cur_url = Some(rest.trim().to_string());
            } else if let Some(url) = &cur_url {
                if t.starts_with("on ") {
                    if let Some(rk) = route_key(url) {
                        map.entry(rk).or_insert_with(|| "on earth".to_string());
                    }
                } else if let Some(rest) = t.strip_prefix("at ") {
                    let body = rest.split_whitespace().next().unwrap_or("sun");
                    if let Some(rk) = route_key(url) {
                        map.entry(rk).or_insert_with(|| format!("at {}", body));
                    }
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string("phi/pipeline/frame_learned.φ") {
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            if let Some((nl, frame)) = t.split_once('|') {
                let nl = nl.trim();
                let frame = frame.trim();
                if !nl.is_empty() && !frame.is_empty() {
                    map.entry(nl.to_string())
                        .or_insert_with(|| frame.to_string());
                }
            }
        }
    }
    map
}




pub fn learn_frames(new: &HashMap<String, String>) {
    let mut map: HashMap<String, String> = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("phi/pipeline/frame_learned.φ") {
        for line in content.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            if let Some((nl, frame)) = t.split_once('|') {
                map.insert(nl.trim().to_string(), frame.trim().to_string());
            }
        }
    }
    for (nl, frame) in new {
        map.entry(nl.to_string())
            .or_insert_with(|| frame.to_string());
    }
    let mut out = String::from(
            "# frame-learned — route (host/path, query stripped) → frame, self-learning from probe responses (--draft)\n",
        );
    let mut keys: Vec<(&String, &String)> = map.iter().collect();
    keys.sort();
    for (nl, frame) in keys {
        out.push_str(&format!("{} | {}\n", nl, frame));
    }
    std::fs::create_dir_all("phi/pipeline").ok();
    if std::fs::write("phi/pipeline/frame_learned.φ", out).is_err() {
        eprintln!("write phi/pipeline/frame_learned.φ: the register does not remember");
    }
}
