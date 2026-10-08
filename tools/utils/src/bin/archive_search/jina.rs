use crate::net::get;

pub fn reader_url(url: &str) -> String {
    format!("https://r.jina.ai/{}", url)
}

pub fn fetch(url: &str) -> Result<String, String> {
    let reader = reader_url(url);
    match get(&reader, &[], "60") {
        Some(f) if f.status == Some(200) => Ok(f.body),
        Some(f) => Err(format!("pending — jina HTTP {}", f.status_text())),
        None => Err("pending — no network".to_string()),
    }
}

pub fn jina_lines(url: &str) -> Vec<String> {
    match fetch(url) {
        Ok(body) => body.lines().map(str::to_string).collect(),
        Err(msg) => vec![msg],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_url_prefixes_the_target() {
        assert_eq!(
            reader_url("https://x.example/a"),
            "https://r.jina.ai/https://x.example/a"
        );
    }
}
