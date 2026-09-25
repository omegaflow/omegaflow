use crate::net;

const OAI_BASE: &str = "https://export.arxiv.org/oai2";

struct OaiRecord {
    identifier: String,
    title: String,
}

struct Page {
    records: Vec<OaiRecord>,
    skipped: usize,
    resumption: Option<String>,
    error: Option<String>,
}

pub fn arxiv_oai_lines(set: Option<&str>, pages: Option<usize>) -> Vec<String> {
    let headers = [
        "-H",
        "User-Agent: omegaflow-archive-search (https://github.com/omegaflow/omegaflow)",
    ];
    let mut url = initial_url(set);
    let mut out = Vec::new();
    let mut fetched = 0usize;
    let mut skipped = 0usize;
    loop {
        if let Some(cap) = pages {
            if fetched >= cap {
                break;
            }
        }
        let fetch = match net::get(&url, &headers, "60") {
            Some(f) => f,
            None => {
                if out.is_empty() {
                    return vec!["pending — no network".to_string()];
                }
                eprintln!(
                    "arxiv-oai: network ended after {} pages ({} records kept)",
                    fetched,
                    out.len()
                );
                break;
            }
        };
        if fetch.status != Some(200) {
            if out.is_empty() {
                return vec![format!("pending — arxiv OAI HTTP {}", fetch.status_text())];
            }
            eprintln!(
                "arxiv-oai: HTTP {} at page {} ({} records kept)",
                fetch.status_text(),
                fetched + 1,
                out.len()
            );
            break;
        }
        let page = parse_page(&fetch.body);
        if let Some(err) = page.error {
            if out.is_empty() {
                return vec![format!("pending — arxiv OAI: {err}")];
            }
            eprintln!(
                "arxiv-oai: {err} at page {} ({} records kept)",
                fetched + 1,
                out.len()
            );
            break;
        }
        fetched += 1;
        skipped += page.skipped;
        for r in &page.records {
            out.push(format!("{} | {}", r.identifier, r.title));
        }
        match page.resumption {
            Some(token) => url = next_url(&token),
            None => break,
        }
    }
    eprintln!(
        "arxiv-oai: {} records ({} skipped), {} pages",
        out.len(),
        skipped,
        fetched
    );
    out
}

fn initial_url(set: Option<&str>) -> String {
    match set {
        Some(s) if !s.trim().is_empty() => format!(
            "{OAI_BASE}?verb=ListRecords&metadataPrefix=arXiv&set={}",
            net::urlencode(s)
        ),
        _ => format!("{OAI_BASE}?verb=ListRecords&metadataPrefix=arXiv"),
    }
}

fn next_url(token: &str) -> String {
    format!("{OAI_BASE}?verb=ListRecords&resumptionToken={token}")
}

fn parse_page(body: &str) -> Page {
    let mut records = Vec::new();
    let mut skipped = 0usize;
    let mut rest = body;
    while let Some(chunk) = open_tag_text(rest, "record") {
        match parse_record(chunk) {
            Some(r) => records.push(r),
            None => skipped += 1,
        }
        let Some(close) = rest.find("</record>") else {
            break;
        };
        rest = &rest[close + "</record>".len()..];
    }
    Page {
        records,
        skipped,
        resumption: open_tag_text(body, "resumptionToken").map(|t| t.trim().to_string()),
        error: tag_line(body, "error"),
    }
}

fn parse_record(chunk: &str) -> Option<OaiRecord> {
    let header = open_tag_text(chunk, "header")?;
    let identifier = tag_line(header, "identifier")?;
    let title = match open_tag_text(chunk, "arXiv") {
        Some(meta) => tag_line(meta, "title")?,
        None => return None,
    };
    Some(OaiRecord { identifier, title })
}

fn tag_line(s: &str, tag: &str) -> Option<String> {
    open_tag_text(s, tag)
        .map(|t| flatten(&decode_entities(&strip_tags(t))))
        .filter(|t| !t.is_empty())
}

fn open_tag_text<'a>(s: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}");
    let mut from = 0usize;
    while let Some(pos) = s[from..].find(&open) {
        let after_tag = from + pos + open.len();
        let boundary = s[after_tag..].chars().next()?;
        if boundary == '>' || boundary.is_whitespace() {
            let after = &s[after_tag..];
            let gt = after.find('>')?;
            let inner = &after[gt + 1..];
            let close = inner.find(&format!("</{tag}>"))?;
            return Some(&inner[..close]);
        }
        from = after_tag;
    }
    None
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
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

fn flatten(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(pos) = rest.find('&') {
        out.push_str(&rest[..pos]);
        rest = &rest[pos..];
        let Some(semi) = rest.find(';') else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let entity = &rest[..=semi];
        match entity {
            "&lt;" => out.push('<'),
            "&gt;" => out.push('>'),
            "&amp;" => out.push('&'),
            "&quot;" => out.push('"'),
            "&apos;" => out.push('\''),
            _ => match decode_char_ref(&rest[1..semi]) {
                Some(c) => out.push(c),
                None => out.push_str(entity),
            },
        }
        rest = &rest[semi + 1..];
    }
    out.push_str(rest);
    out
}

fn decode_char_ref(body: &str) -> Option<char> {
    if let Some(digits) = body.strip_prefix("#x").or_else(|| body.strip_prefix("#X")) {
        return u32::from_str_radix(digits, 16)
            .ok()
            .and_then(char::from_u32);
    }
    if let Some(digits) = body.strip_prefix('#') {
        return digits.parse::<u32>().ok().and_then(char::from_u32);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<OAI-PMH xmlns=\"http://www.openarchives.org/OAI/2.0/\">
<responseDate>2026-09-25T19:50:29Z</responseDate>
<request verb=\"ListRecords\" metadataPrefix=\"arXiv\">http://oaipmh.arxiv.org/oai</request>
<ListRecords>
<record>
<header>
<identifier>oai:arXiv.org:adap-org/9710003</identifier>
<datestamp>2005-09-17</datestamp>
<setSpec>physics:nlin:AO</setSpec>
</header>
<metadata>
<arXiv xmlns=\"http://arxiv.org/OAI/arXiv/\">
<id>adap-org/9710003</id>
<created>1997-10-21</created>
<updated>2009-11-30</updated>
<authors>
<author>
<keyname>Nagel</keyname>
<forenames>Kai</forenames>
</author>
<author>
<keyname>Stretz</keyname>
<forenames>Paula</forenames>
</author>
</authors>
<title>The &#34;traffic&#34; &amp; its &#39;flow&#39; &lt;characteristics&gt;</title>
<categories>adap-org nlin.AO</categories>
<comments>Paper has 23 pages</comments>
<report-no>adap-org/9710003</report-no>
<abstract>  Knowledge of fundamental traffic flow characteristics.</abstract>
</arXiv>
</metadata>
</record>
<record>
<header status=\"deleted\">
<identifier>oai:arXiv.org:adap-org/9712004</identifier>
<datestamp>2026-09-20</datestamp>
</header>
</record>
<resumptionToken expirationDate='2026-09-26T00:00:00Z'>verb%3DListRecords%26metadataPrefix%3DarXiv%26skip%3D1300</resumptionToken>
</ListRecords>
</OAI-PMH>
";

    #[test]
    fn parse_page_takes_identifier_title_and_resumption_and_skips_titleless() {
        let page = parse_page(FIXTURE);
        assert_eq!(page.records.len(), 1);
        assert_eq!(page.records[0].identifier, "oai:arXiv.org:adap-org/9710003");
        assert_eq!(
            page.records[0].title,
            "The \"traffic\" & its 'flow' <characteristics>"
        );
        assert_eq!(page.skipped, 1);
        assert_eq!(
            page.resumption.as_deref(),
            Some("verb%3DListRecords%26metadataPrefix%3DarXiv%26skip%3D1300")
        );
        assert_eq!(page.error, None);
    }

    #[test]
    fn record_field_extractors_read_the_measured_arxiv_shape() {
        let header = open_tag_text(FIXTURE, "header").unwrap();
        assert_eq!(tag_line(header, "datestamp").as_deref(), Some("2005-09-17"));
        let meta = open_tag_text(FIXTURE, "arXiv").unwrap();
        assert_eq!(
            tag_line(meta, "categories").as_deref(),
            Some("adap-org nlin.AO")
        );
        assert_eq!(
            tag_line(meta, "abstract").as_deref(),
            Some("Knowledge of fundamental traffic flow characteristics.")
        );
    }

    #[test]
    fn resumption_token_builds_the_next_list_records_url() {
        assert_eq!(
            next_url("verb%3DListRecords%26metadataPrefix%3DarXiv%26skip%3D1300"),
            "https://export.arxiv.org/oai2?verb=ListRecords&resumptionToken=verb%3DListRecords%26metadataPrefix%3DarXiv%26skip%3D1300"
        );
    }

    #[test]
    fn set_filter_lands_in_the_initial_url() {
        assert_eq!(
            initial_url(Some("physics:astro-ph")),
            "https://export.arxiv.org/oai2?verb=ListRecords&metadataPrefix=arXiv&set=physics%3Aastro-ph"
        );
        assert_eq!(
            initial_url(None),
            "https://export.arxiv.org/oai2?verb=ListRecords&metadataPrefix=arXiv"
        );
    }

    #[test]
    fn error_element_surfaces_the_server_message() {
        let body = "<OAI-PMH><error code=\"badArgument\">Illegal set spec</error></OAI-PMH>";
        let page = parse_page(body);
        assert_eq!(page.error.as_deref(), Some("Illegal set spec"));
        assert!(page.records.is_empty());
    }

    #[test]
    fn decode_entities_handles_named_decimal_and_hex_forms() {
        assert_eq!(
            decode_entities("&#34;x&#34; &#x27;y&#x27; &lt;z&gt; &amp; &apos;"),
            "\"x\" 'y' <z> & '"
        );
        assert_eq!(decode_entities("&nbsp; stays"), "&nbsp; stays");
    }
}
