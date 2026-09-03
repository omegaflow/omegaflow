use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sL")
        .arg("--max-time")
        .arg("40")
        .arg(&url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

fn extract_between<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = s.find(open)? + open.len();
    let rest = &s[start..];
    let end = rest.find(close)?;
    Some(&rest[..end])
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

fn attr_after(s: &str, needle: &str, attr: &str) -> Option<String> {
    let pos = s.find(needle)?;
    let rest = &s[pos..];
    let key = format!("{}=\"", attr);
    let a = rest.find(&key)? + key.len();
    let after = &rest[a..];
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

fn pdf_link(entry: &str) -> Option<String> {
    let pos = entry.find("title=\"pdf\"")?;
    let before = &entry[..pos];
    let h = before.rfind("href=\"")? + 6;
    let after = &before[h..];
    let end = after.find('"')?;
    Some(after[..end].to_string())
}

struct Entry {
    id: String,
    title: String,
    authors: String,
    published: String,
    category: String,
    pdf: String,
    summary: String,
}

fn parse_entries(xml: &str) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(open) = rest.find("<entry") {
        let close = match rest[open..].find("</entry>") {
            Some(c) => open + c,
            None => break,
        };
        let entry = &rest[open..close];
        let title = extract_between(entry, "<title>", "</title>")
            .map(|t| strip_tags(t).trim().to_string())
            .unwrap_or_default();
        let summary = extract_between(entry, "<summary>", "</summary>")
            .map(|t| strip_tags(t).trim().to_string())
            .unwrap_or_default();
        let mut names = Vec::new();
        let mut r = entry;
        while let Some(n) = extract_between(r, "<name>", "</name>") {
            names.push(strip_tags(n).trim().to_string());
            let off = r.find("<name>").unwrap() + 6;
            r = &r[off..];
        }
        let id = extract_between(entry, "<id>", "</id>")
            .map(|t| strip_tags(t).trim().to_string())
            .unwrap_or_default();
        let published = extract_between(entry, "<published>", "</published>")
            .map(|t| strip_tags(t).trim().to_string())
            .unwrap_or_default();
        let category = attr_after(entry, "primary_category", "term").unwrap_or_default();
        let pdf = pdf_link(entry).unwrap_or_default();
        out.push(Entry {
            id,
            title,
            authors: names.join("; "),
            published,
            category,
            pdf,
            summary,
        });
        rest = &rest[close..];
    }
    out
}

fn print_entry(e: &Entry) {
    println!("== {} ==", e.id);
    println!("\ttitle:     {}", e.title);
    println!("\tauthors:   {}", e.authors);
    println!("\tpublished: {}", e.published);
    println!("\tcategory:  {}", e.category);
    if !e.pdf.is_empty() {
        println!("\tpdf:       {}", e.pdf);
    }
    if !e.summary.is_empty() {
        println!("\tabstract:  {}", e.summary);
    }
}

fn cmd_id(ids: &[String]) -> i32 {
    let joined = ids.join(",");
    let url = format!("http://export.arxiv.org/api/query?id_list={}", joined);
    let mut code = 0;
    match fetch(&url) {
        None => {
            eprintln!("pending — no network / timeout");
            code = 1;
        }
        Some(xml) => {
            let entries = parse_entries(&xml);
            if entries.is_empty() {
                eprintln!("absent — the register carries no entry for: {}", joined);
                code = 1;
            }
            for e in &entries {
                print_entry(e);
            }
        }
    }
    code
}

fn cmd_search(query: &str, max: usize) -> i32 {
    let q = query.replace(' ', "+");
    let url = format!(
        "http://export.arxiv.org/api/query?search_query={}&max_results={}",
        q, max
    );
    let mut code = 0;
    match fetch(&url) {
        None => {
            eprintln!("pending — no network / timeout");
            code = 1;
        }
        Some(xml) => {
            let entries = parse_entries(&xml);
            if entries.is_empty() {
                eprintln!("absent — the query carries no entry: {}", query);
                code = 1;
            }
            for e in &entries {
                print_entry(e);
            }
        }
    }
    code
}

fn cmd_oai(set: &str, from: Option<&str>) -> i32 {
    let mut url = format!(
        "http://export.arxiv.org/oai2?verb=ListRecords&metadataPrefix=oai_dc&set={}",
        set
    );
    if let Some(f) = from {
        url.push_str(&format!("&from={}", f));
    }
    let mut code = 0;
    match fetch(&url) {
        None => {
            eprintln!("pending — no network / timeout");
            code = 1;
        }
        Some(xml) => {
            let mut count = 0usize;
            let mut rest = xml.as_str();
            while let Some(open) = rest.find("<record>") {
                let close = match rest[open..].find("</record>") {
                    Some(c) => open + c,
                    None => break,
                };
                let rec = &rest[open..close];
                let ident = extract_between(rec, "<identifier>", "</identifier>")
                    .map(|t| strip_tags(t).trim().to_string())
                    .unwrap_or_default();
                let title = extract_between(rec, "<dc:title>", "</dc:title>")
                    .or_else(|| extract_between(rec, "<title>", "</title>"))
                    .map(|t| strip_tags(t).trim().to_string())
                    .unwrap_or_default();
                println!("{}\t{}", ident, title);
                count += 1;
                rest = &rest[close..];
            }
            println!("records: {}", count);
            if xml.contains("<resumptionToken") {
                let token = extract_between(&xml, "<resumptionToken", "</resumptionToken>")
                    .map(|t| strip_tags(t).trim().to_string())
                    .unwrap_or_default();
                eprintln!(
                    "more records pending — resumptionToken present (not followed): {}",
                    token
                );
                code = 1;
            }
            if count == 0 {
                eprintln!("absent — the set carries no record: {}", set);
                code = 1;
            }
        }
    }
    code
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!(
            "usage: arxiv id <id>... | arxiv search <query> [--max N] | arxiv oai <set> [--from YYYY-MM-DD]"
        );
        std::process::exit(2);
    }
    let code = match args[0].as_str() {
        "id" => {
            let ids: Vec<String> = args[1..]
                .iter()
                .filter(|a| !a.starts_with("--"))
                .cloned()
                .collect();
            if ids.is_empty() {
                eprintln!("usage: arxiv id <id> [<id> ...]");
                std::process::exit(2);
            }
            let mut c = 0;
            for (i, id) in ids.iter().enumerate() {
                if i > 0 {
                    thread::sleep(Duration::from_secs(3));
                }
                c |= cmd_id(&[id.clone()]);
            }
            c
        }
        "search" => {
            let mut max = 10usize;
            let mut terms: Vec<String> = Vec::new();
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--max" {
                    max = args.get(i + 1).and_then(|m| m.parse().ok()).unwrap_or(10);
                    i += 2;
                } else {
                    terms.push(args[i].clone());
                    i += 1;
                }
            }
            if terms.is_empty() {
                eprintln!("usage: arxiv search <query> [--max N]");
                std::process::exit(2);
            }
            cmd_search(&terms.join(" "), max)
        }
        "oai" => {
            let set = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if set.is_empty() {
                eprintln!("usage: arxiv oai <set> [--from YYYY-MM-DD]");
                std::process::exit(2);
            }
            let mut from = None;
            if args.get(2).map(|s| s.as_str()) == Some("--from") {
                from = args.get(3).map(|s| s.as_str());
            }
            cmd_oai(set, from)
        }
        other => {
            eprintln!("unknown subcommand: {}", other);
            std::process::exit(2);
        }
    };
    if code != 0 {
        std::process::exit(1);
    }
}
