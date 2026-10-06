use std::env;
use std::fs;

const DEFAULT_REGISTER: &str = "phi/sources.φ";
const HAPI_URL_PREFIX: &str = "url https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = String::from(DEFAULT_REGISTER);
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with('-') {
            eprintln!("hapi_station_tag: unknown flag '{}'", arg);
            usage();
            std::process::exit(2);
        }
        if path == DEFAULT_REGISTER {
            path = arg.clone();
        } else {
            eprintln!("hapi_station_tag: multiple paths '{}' '{}'", path, arg);
            usage();
            std::process::exit(2);
        }
        i += 1;
    }

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("hapi_station_tag: read {}: {}", path, e);
            std::process::exit(2);
        }
    };

    let tag = tag_blocks(&content);
    if tag.inserted > 0 {
        if let Err(e) = fs::write(&path, &tag.rendered) {
            eprintln!("hapi_station_tag: write {}: {}", path, e);
            std::process::exit(2);
        }
    }
    println!(
        "tagged {} blocks ({} already tagged)",
        tag.inserted, tag.already
    );
}

fn usage() {
    eprintln!("usage: hapi_station_tag [path]");
    eprintln!(
        "  sources register (default {}): for every block opening with the",
        DEFAULT_REGISTER
    );
    eprintln!("  BGS GIN HAPI url, writes 'station <CODE>' (CODE uppercased) directly");
    eprintln!("  after the url line, unless the block already carries a station line.");
    eprintln!("  idempotent; other bytes preserved. prints the tagged/already counts.");
}

struct Tag {
    rendered: String,
    inserted: usize,
    already: usize,
}

fn hapi_code(line: &str) -> Option<String> {
    let rest = line.strip_prefix(HAPI_URL_PREFIX)?;
    let end = rest.find('/')?;
    let code = &rest[..end];
    if code.is_empty() {
        return None;
    }
    Some(code.to_uppercase())
}

fn tag_blocks(content: &str) -> Tag {
    let pieces: Vec<&str> = content.split_inclusive('\n').collect();
    let mut insertion: Vec<Option<String>> = vec![None; pieces.len()];
    let mut inserted = 0usize;
    let mut already = 0usize;

    for (i, piece) in pieces.iter().enumerate() {
        let line = piece.strip_suffix('\n').unwrap_or(piece);
        let code = match hapi_code(line) {
            Some(c) => c,
            None => continue,
        };
        let mut has_station = false;
        for follower in pieces.iter().skip(i + 1) {
            let follower_line = follower.strip_suffix('\n').unwrap_or(follower);
            if follower_line.trim().is_empty() {
                break;
            }
            if follower_line.trim_start().starts_with("station ") {
                has_station = true;
                break;
            }
        }
        if has_station {
            already += 1;
        } else {
            insertion[i] = Some(code);
            inserted += 1;
        }
    }

    let mut rendered = String::with_capacity(content.len() + inserted * 16);
    for (i, piece) in pieces.iter().enumerate() {
        rendered.push_str(piece);
        if let Some(code) = &insertion[i] {
            rendered.push_str("station ");
            rendered.push_str(code);
            rendered.push('\n');
        }
    }

    Tag {
        rendered,
        inserted,
        already,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_extracted_and_uppercased() {
        let line =
            "url https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=aae/best-avail/PT1M/xyzf&start=x";
        assert_eq!(hapi_code(line), Some("AAE".to_string()));
    }

    #[test]
    fn non_hapi_line_is_ignored() {
        assert_eq!(hapi_code("url https://example.com/data"), None);
        assert_eq!(
            hapi_code("url https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=/x"),
            None
        );
    }

    #[test]
    fn inserts_after_url_line_only() {
        let text = [
            "url https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=abg/best-avail",
            "ttl 86400",
            "on earth 18.638 72.872 7",
            "",
            "url https://example.com/x",
            "ttl 5",
        ]
        .join("\n")
            + "\n";
        let tag = tag_blocks(&text);
        assert_eq!(tag.inserted, 1);
        assert_eq!(tag.already, 0);
        assert_eq!(
            tag.rendered,
            "url https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=abg/best-avail\nstation ABG\nttl 86400\non earth 18.638 72.872 7\n\nurl https://example.com/x\nttl 5\n"
        );
    }

    #[test]
    fn already_tagged_block_is_skipped_and_idempotent() {
        let text = [
            "url https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=aae/best-avail",
            "station AAE",
            "ttl 86400",
            "",
        ]
        .join("\n")
            + "\n";
        let tag = tag_blocks(&text);
        assert_eq!(tag.inserted, 0);
        assert_eq!(tag.already, 1);
        assert_eq!(tag.rendered, text);
    }
}
