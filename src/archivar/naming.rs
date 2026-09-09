use super::*;

pub fn extract_netloc(url: &str) -> Option<&str> {
    let after = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let netloc = after.split('/').next()?;
    Some(if let Some(s) = netloc.strip_prefix("www.") {
        s
    } else {
        netloc
    })
}

pub fn route_segments(url: &str) -> Option<(String, Vec<String>)> {
    let after = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    let (netloc, rest) = match after.split_once('/') {
        Some((n, r)) => (n, r),
        None => (after, ""),
    };
    let host = netloc.strip_prefix("www.").unwrap_or(netloc);
    let path = rest.split(|c| c == '?' || c == '#').next().unwrap_or("");
    let mut segs: Vec<String> = Vec::new();
    for s in path.split('/') {
        if s.is_empty() {
            continue;
        }
        let seg = if s.starts_with('{') && s.ends_with('}') {
            "*".to_string()
        } else {
            s.to_string()
        };
        segs.push(seg);
    }
    Some((host.to_string(), segs))
}

pub fn route_key(url: &str) -> Option<String> {
    let (host, segs) = route_segments(url)?;
    if segs.is_empty() {
        Some(host)
    } else {
        Some(format!("{}/{}", host, segs.join("/")))
    }
}

pub fn route_prefix_keys(url: &str) -> Vec<String> {
    let Some((host, segs)) = route_segments(url) else {
        return Vec::new();
    };
    let mut keys = vec![host.clone()];
    let mut acc = host;
    for s in segs {
        acc.push('/');
        acc.push_str(&s);
        keys.push(acc.clone());
    }
    keys.reverse();
    keys
}

pub fn source_name_from_url(url: &str) -> String {
    let s1 = match url.strip_prefix("https://") {
        Some(s) => s,
        None => url,
    };
    let s2 = match s1.strip_prefix("http://") {
        Some(s) => s,
        None => s1,
    };
    let without_scheme = match s2.strip_prefix("www.") {
        Some(s) => s,
        None => s2,
    };
    let after_domain: Vec<&str> = without_scheme.splitn(2, '/').collect();
    if after_domain.len() < 2 {
        return "index.json".to_string();
    }
    let path_and_query = after_domain[1];
    let cleaned = path_and_query
        .chars()
        .map(|c| match c {
            c if c.is_ascii_alphanumeric()
                || c == '-'
                || c == '.'
                || c == '_'
                || c == '{'
                || c == '}' =>
            {
                c
            }
            '/' | '?' | '&' | '=' => '-',
            _ => '_',
        })
        .collect::<String>();
    if cleaned.is_empty() {
        "index".to_string()
    } else {
        cleaned.trim_matches('-').to_string()
    }
}

pub fn reference_name_from_url(url: &str) -> String {
    let without_query = url.split(['?', '#']).next().unwrap_or(url);
    let after_scheme = match without_query.split_once("://") {
        Some((_, rest)) => rest,
        None => without_query,
    };
    match after_scheme.split_once('/') {
        Some((_, path)) => match path.split('/').filter(|s| !s.is_empty()).last() {
            Some(seg) => seg.to_string(),
            None => "reference".to_string(),
        },
        None => "reference".to_string(),
    }
}

pub fn cdn_manifest_for(urls: impl Iterator<Item = String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut seen: HashMap<String, u32> = HashMap::new();
    for url in urls {
        if url.starts_with("https://github.com/omegaflow/sources") {
            continue;
        }
        let name = source_name_from_url(&url);
        let k = match seen.get_mut(&name) {
            Some(n) => {
                *n += 1;
                *n
            }
            None => {
                seen.insert(name, 1);
                continue;
            }
        };
        map.insert(url, format!("{}-{}", name, k));
    }
    map
}

pub fn cdn_manifest_map() -> &'static HashMap<String, String> {
    static MANIFEST: OnceLock<HashMap<String, String>> = OnceLock::new();
    MANIFEST.get_or_init(|| {
        if let Ok(content) = std::fs::read_to_string("phi/sources.φ") {
            let sources = load_sources_from(&content);
            cdn_manifest_for(sources.iter().map(|s| s.url.clone()))
        } else {
            HashMap::new()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_name_is_the_origin_filename() {
        assert_eq!(
            reference_name_from_url("https://arxiv.org/e-print/2012.08534"),
            "2012.08534"
        );
        assert_eq!(
            reference_name_from_url(
                "https://raw.githubusercontent.com/PantheonPlusSH0ES/DataRelease/main/Pantheon+_Data/4_DISTANCES_AND_COVAR/Pantheon+SH0ES.dat"
            ),
            "Pantheon+SH0ES.dat"
        );
    }

    #[test]
    fn reference_name_strips_query_and_fragment() {
        assert_eq!(
            reference_name_from_url("https://example.org/data.csv?download=1#top"),
            "data.csv"
        );
    }

    #[test]
    fn reference_name_falls_back_on_an_empty_path() {
        assert_eq!(reference_name_from_url("https://example.org"), "reference");
    }
}
