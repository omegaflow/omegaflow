use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut links = false;
    let mut title_only = false;
    let mut url: Option<String> = None;
    for a in args.iter().skip(1) {
        match a.as_str() {
            "--links" => links = true,
            "--title" => title_only = true,
            _ if url.is_none() => url = Some(a.clone()),
            _ => {}
        }
    }
    let Some(url) = url else {
        eprintln!("usage: sfetch [--links] [--title] <url>");
        std::process::exit(2);
    };
    let raw = match Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("60")
        .arg(&url)
        .output()
    {
        Ok(o) if o.status.success() => o.stdout,
        _ => {
            eprintln!("sfetch: fetch void");
            std::process::exit(1);
        }
    };
    let html = String::from_utf8_lossy(&raw);
    if title_only {
        if let Some(t) = extract_title(&html) {
            println!("{}", t);
        }
        return;
    }
    if links {
        for l in extract_links(&html) {
            println!("{}", l);
        }
        return;
    }
    print!("{}", strip_tags(&html));
}

fn extract_title(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    let start = lower.find("<title")?;
    let end = lower[start..].find('>')? + start;
    let close = html[end..].find("</title>")? + end;
    Some(strip_tags(&html[end + 1..close]).trim().to_string())
}

fn extract_links(html: &str) -> Vec<String> {
    let mut links = Vec::new();
    let lower = html.to_lowercase();
    let mut pos = 0;
    while let Some(rel) = lower[pos..].find("href") {
        let idx = pos + rel;
        let rest = &html[idx + 4..];
        let mut q = '\0';
        let mut start = 0;
        let mut end = 0;
        for (i, c) in rest.char_indices() {
            if c == '"' || c == '\'' {
                if q == '\0' {
                    q = c;
                    start = i + c.len_utf8();
                } else if q == c {
                    end = i;
                    break;
                }
            }
        }
        if start > 0 && end > start {
            let link = rest[start..end].to_string();
            if !link.starts_with("javascript:") && !link.starts_with('#') {
                links.push(link);
            }
        }
        pos = idx + 4;
    }
    links
}

fn strip_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
            }
            continue;
        }
        if c == '<' {
            in_tag = true;
            continue;
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_tags() {
        let html = "<html><head><title>t</title></head><body><p>hi there</p></body></html>";
        let text = strip_tags(html);
        assert!(text.contains("hi there"));
        assert!(!text.contains('<'));
    }

    #[test]
    fn title_extraction() {
        let html = "<html><head><title>Der Gegenhang</title></head></html>";
        assert_eq!(extract_title(html).as_deref(), Some("Der Gegenhang"));
    }

    #[test]
    fn link_extraction() {
        let html = "<a href=\"https://omegaflow.space\">x</a><a href='#top'>y</a>";
        let links = extract_links(html);
        assert!(links.contains(&"https://omegaflow.space".to_string()));
        assert!(!links.contains(&"#top".to_string()));
    }
}
