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

fn element_body<'a>(s: &'a str, open_prefix: &str, close_tag: &str) -> Option<&'a str> {
    let open = s.find(open_prefix)?;
    let tag_end = s[open..].find('>')? + open;
    let rest = &s[tag_end + 1..];
    let close = rest.find(close_tag)?;
    Some(&rest[..close])
}

fn field_text<'a>(s: &'a str, open: &str, close: &str) -> Option<String> {
    extract_between(s, open, close)
        .map(|t| strip_tags(t).trim().to_string())
        .filter(|t| !t.is_empty())
}

struct Entry {
    id: String,
    title: String,
    authors: Option<String>,
    published: Option<String>,
    category: Option<String>,
    pdf: Option<String>,
    summary: Option<String>,
    journal_ref: Option<String>,
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
        let Some(title) = field_text(entry, "<title>", "</title>") else {
            rest = &rest[close..];
            continue;
        };
        let Some(id) = field_text(entry, "<id>", "</id>") else {
            rest = &rest[close..];
            continue;
        };
        let mut names = Vec::new();
        let mut r = entry;
        while let Some(n) = extract_between(r, "<name>", "</name>") {
            let name = strip_tags(n).trim().to_string();
            if !name.is_empty() {
                names.push(name);
            }
            let off = r.find("<name>").unwrap() + 6;
            r = &r[off..];
        }
        let authors = if names.is_empty() {
            None
        } else {
            Some(names.join("; "))
        };
        let summary = field_text(entry, "<summary>", "</summary>");
        let published = field_text(entry, "<published>", "</published>");
        let category = attr_after(entry, "primary_category", "term").filter(|t| !t.is_empty());
        let pdf = pdf_link(entry).filter(|t| !t.is_empty());
        let journal_ref = element_body(entry, "<arxiv:journal_ref", "</arxiv:journal_ref>")
            .map(|t| strip_tags(t).trim().to_string())
            .filter(|t| !t.is_empty());
        out.push(Entry {
            id,
            title,
            authors,
            published,
            category,
            pdf,
            summary,
            journal_ref,
        });
        rest = &rest[close..];
    }
    out
}

fn print_entry(e: &Entry) {
    println!("== {} ==", e.id);
    println!("\ttitle:     {}", e.title);
    if let Some(a) = &e.authors {
        println!("\tauthors:   {}", a);
    }
    if let Some(p) = &e.published {
        println!("\tpublished: {}", p);
    }
    if let Some(c) = &e.category {
        println!("\tcategory:  {}", c);
    }
    if let Some(p) = &e.pdf {
        println!("\tpdf:       {}", p);
    }
    if let Some(s) = &e.summary {
        println!("\tabstract:  {}", s);
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
                let Some(ident) = extract_between(rec, "<identifier>", "</identifier>")
                    .map(|t| strip_tags(t).trim().to_string())
                    .filter(|t| !t.is_empty())
                else {
                    rest = &rest[close..];
                    continue;
                };
                let title = extract_between(rec, "<dc:title>", "</dc:title>")
                    .or_else(|| extract_between(rec, "<title>", "</title>"))
                    .map(|t| strip_tags(t).trim().to_string())
                    .filter(|t| !t.is_empty());
                match title {
                    Some(t) => println!("{}\t{}", ident, t),
                    None => println!("{}", ident),
                }
                count += 1;
                rest = &rest[close..];
            }
            println!("records: {}", count);
            if xml.contains("<resumptionToken") {
                let token = extract_between(&xml, "<resumptionToken", "</resumptionToken>")
                    .map(|t| strip_tags(t).trim().to_string())
                    .filter(|t| !t.is_empty());
                match token {
                    Some(t) => eprintln!(
                        "more records pending — resumptionToken present (not followed): {}",
                        t
                    ),
                    None => {
                        eprintln!("more records pending — resumptionToken present (not followed)")
                    }
                }
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

const JOURNAL_REF_YEAR_WEIGHT: i32 = 3;
const YEAR_PROXIMITY_WEIGHT: i32 = 2;
const KEYWORD_HIT_WEIGHT: i32 = 1;
const RELEVANCE_KEYWORDS: [&str; 6] = [
    "transit",
    "spectr",
    "atmosphere",
    "water",
    "haze",
    "featureless",
];
const RANK_CUTOFF: usize = 6;
const SEARCH_MAX: usize = 60;
const CENSUS_PATH: &str = "docs/surveys/survey-transmission-host-census.json";

fn bibcode_year(bibcode: &str) -> Option<i32> {
    let head = bibcode.get(..4)?;
    head.parse::<i32>().ok()
}

fn published_year(published: &str) -> Option<i32> {
    let head = published.get(..4)?;
    head.parse::<i32>().ok()
}

fn query_host(host: &str) -> String {
    let t = host.trim_end();
    match t.strip_suffix(" A") {
        Some(rest) => rest.trim_end().to_string(),
        None => t.to_string(),
    }
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn contains_whole_year(text: &str, year: i32) -> bool {
    let needle = year.to_string();
    let bytes = text.as_bytes();
    let n = needle.len();
    if n == 0 || bytes.len() < n {
        return false;
    }
    bytes.windows(n).enumerate().any(|(i, w)| {
        w == needle.as_bytes()
            && (i == 0 || !is_word_byte(bytes[i - 1]))
            && (i + n == bytes.len() || !is_word_byte(bytes[i + n]))
    })
}

fn hit_score(
    title: &str,
    abs_text: Option<&str>,
    journal_ref: Option<&str>,
    entry_year: Option<i32>,
    bibcode_year: i32,
) -> i32 {
    let mut score = 0;
    if let Some(jr) = journal_ref {
        if contains_whole_year(jr, bibcode_year) {
            score += JOURNAL_REF_YEAR_WEIGHT;
        }
    }
    if let Some(y) = entry_year {
        if y == bibcode_year || y == bibcode_year - 1 {
            score += YEAR_PROXIMITY_WEIGHT;
        }
    }
    let title_lower = title.to_ascii_lowercase();
    let abs_lower = abs_text.map(|a| a.to_ascii_lowercase());
    for kw in RELEVANCE_KEYWORDS {
        let abs_hit = match &abs_lower {
            Some(a) => a.contains(kw),
            None => false,
        };
        if title_lower.contains(kw) || abs_hit {
            score += KEYWORD_HIT_WEIGHT;
        }
    }
    score
}

fn score_entry(e: &Entry, bibcode_year: i32) -> i32 {
    let y = e.published.as_deref().and_then(published_year);
    hit_score(
        &e.title,
        e.summary.as_deref(),
        e.journal_ref.as_deref(),
        y,
        bibcode_year,
    )
}

fn rank_entries(entries: &[Entry], bibcode_year: i32) -> Vec<(usize, i32)> {
    let mut scored: Vec<(usize, i32)> = (0..entries.len())
        .map(|i| (i, score_entry(&entries[i], bibcode_year)))
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    scored
}

fn flatten_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn clip(s: &str, max: usize) -> &str {
    match s.char_indices().nth(max) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

fn print_scored_hit(e: &Entry, score: i32, rank: usize) {
    let y = match e.published.as_deref().and_then(published_year) {
        Some(v) => v.to_string(),
        None => "????".to_string(),
    };
    println!("rank={} score={} year={} | {}", rank, score, y, e.title);
    match &e.journal_ref {
        Some(jr) => println!("id={} | jr={}", e.id, clip(&flatten_ws(jr), 110)),
        None => println!("id={} | jr=(absent)", e.id),
    }
    match &e.summary {
        Some(s) => println!("ABS: {}", clip(&flatten_ws(s), 1400)),
        None => println!("ABS: (absent)"),
    }
}

fn print_rank_report(entries: &[Entry], bibcode_year: i32, top: usize) {
    let ranked = rank_entries(entries, bibcode_year);
    let shown = ranked.len().min(top);
    for (pos, (ei, s)) in ranked.iter().take(shown).enumerate() {
        print_scored_hit(&entries[*ei], *s, pos + 1);
    }
}

fn search_entries(query: &str, max: usize) -> Vec<Entry> {
    let q = query.replace(' ', "+");
    let url = format!(
        "http://export.arxiv.org/api/query?search_query={}&max_results={}",
        q, max
    );
    match fetch(&url) {
        None => Vec::new(),
        Some(xml) => parse_entries(&xml),
    }
}

fn cmd_rank(host: &str, bibcode: &str, max: usize, top: usize) -> i32 {
    let Some(yr) = bibcode_year(bibcode) else {
        eprintln!("absent — the bibcode carries no leading year: {}", bibcode);
        return 1;
    };
    let hq = query_host(host);
    let mut entries = search_entries(&format!("all:\"{}\" AND abs:\"transmission\"", hq), max);
    if entries.is_empty() {
        entries = search_entries(&format!("all:\"{}\"", hq), max);
    }
    println!("{}", "=".repeat(80));
    println!("HOST={} | BIB={} ({})", host, bibcode, yr);
    println!("{}", "-".repeat(70));
    if entries.is_empty() {
        eprintln!("absent — the host query carries no entry: {}", host);
        return 1;
    }
    print_rank_report(&entries, yr, top);
    0
}

use omegaflow::json::{jstr, parse_json, JsonVal};

struct CensusHost {
    hostname: String,
    facility: String,
    instruments: Option<String>,
    bibcode: String,
}

fn read_census_hosts(path: &str) -> Result<Vec<CensusHost>, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("census {}: {}", path, e))?;
    let root = parse_json(&body).ok_or_else(|| format!("census {}: json absent", path))?;
    let JsonVal::Arr(rows) = &root else {
        return Err(format!("census {}: root is not an array", path));
    };
    let mut out = Vec::new();
    for row in rows {
        let JsonVal::Obj(map) = row else {
            continue;
        };
        let instruments = match map.get("instrument") {
            Some(JsonVal::Arr(items)) => {
                let joined = items
                    .iter()
                    .filter_map(|it| match it {
                        JsonVal::Str(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                if joined.is_empty() {
                    None
                } else {
                    Some(joined)
                }
            }
            _ => None,
        };
        let (Some(hostname), Some(facility)) = (jstr(row, "hostname"), jstr(row, "facility"))
        else {
            continue;
        };
        let Some(bibcode) = jstr(row, "primary_bibcode") else {
            continue;
        };
        out.push(CensusHost {
            hostname,
            facility,
            instruments,
            bibcode,
        });
    }
    Ok(out)
}

fn cmd_censusrank(path: &str, max: usize, top: usize) -> i32 {
    let hosts = match read_census_hosts(path) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("{}", msg);
            return 1;
        }
    };
    let mut seen: Vec<(String, Option<String>, String)> = Vec::new();
    let mut processed = 0usize;
    for h in hosts {
        if h.facility != "HST" {
            continue;
        }
        let key = (h.hostname.clone(), h.instruments.clone(), h.bibcode.clone());
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);
        let Some(yr) = bibcode_year(&h.bibcode) else {
            eprintln!(
                "absent — the host carries no primary bibcode year: {}",
                h.hostname
            );
            continue;
        };
        let hq = query_host(&h.hostname);
        let mut entries = search_entries(&format!("all:\"{}\" AND abs:\"transmission\"", hq), max);
        if entries.is_empty() {
            entries = search_entries(&format!("all:\"{}\"", hq), max);
        }
        println!("{}", "=".repeat(80));
        println!("HOST={} | BIB={} ({})", h.hostname, h.bibcode, yr);
        println!("{}", "-".repeat(70));
        if !entries.is_empty() {
            print_rank_report(&entries, yr, top);
        }
        processed += 1;
        thread::sleep(Duration::from_secs(3));
    }
    println!("hosts processed: {}", processed);
    if processed == 0 {
        eprintln!("absent — the census carries no HST host: {}", path);
        return 1;
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!(
            "usage: arxiv id <id>... | arxiv search <query> [--max N] | arxiv oai <set> [--from YYYY-MM-DD] | arxiv rank --host <host> --bibcode <bibcode> [--max N] [--top N] | arxiv censusrank [<census.json>] [--max N] [--top N]"
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
        "rank" => {
            let mut host = None;
            let mut bibcode = None;
            let mut max = SEARCH_MAX;
            let mut top = RANK_CUTOFF;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--host" => {
                        host = args.get(i + 1).cloned();
                        i += 2;
                    }
                    "--bibcode" => {
                        bibcode = args.get(i + 1).cloned();
                        i += 2;
                    }
                    "--max" => {
                        max = args
                            .get(i + 1)
                            .and_then(|m| m.parse().ok())
                            .unwrap_or(SEARCH_MAX);
                        i += 2;
                    }
                    "--top" => {
                        top = args
                            .get(i + 1)
                            .and_then(|m| m.parse().ok())
                            .unwrap_or(RANK_CUTOFF);
                        i += 2;
                    }
                    other => {
                        eprintln!("unknown flag: {}", other);
                        std::process::exit(2);
                    }
                }
            }
            let (Some(host), Some(bibcode)) = (host, bibcode) else {
                eprintln!(
                    "usage: arxiv rank --host <host> --bibcode <primary bibcode> [--max N] [--top N]"
                );
                std::process::exit(2);
            };
            cmd_rank(&host, &bibcode, max, top)
        }
        "censusrank" => {
            let mut path = CENSUS_PATH.to_string();
            let mut max = SEARCH_MAX;
            let mut top = RANK_CUTOFF;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--max" => {
                        max = args
                            .get(i + 1)
                            .and_then(|m| m.parse().ok())
                            .unwrap_or(SEARCH_MAX);
                        i += 2;
                    }
                    "--top" => {
                        top = args
                            .get(i + 1)
                            .and_then(|m| m.parse().ok())
                            .unwrap_or(RANK_CUTOFF);
                        i += 2;
                    }
                    a if a.starts_with("--") => {
                        eprintln!("unknown flag: {}", a);
                        std::process::exit(2);
                    }
                    _ => {
                        path = args[i].clone();
                        i += 1;
                    }
                }
            }
            cmd_censusrank(&path, max, top)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(title: &str, abs: &str, published: &str, journal_ref: &str) -> Entry {
        let summary = if abs.is_empty() {
            None
        } else {
            Some(abs.to_string())
        };
        let published = if published.is_empty() {
            None
        } else {
            Some(published.to_string())
        };
        Entry {
            id: "http://arxiv.org/abs/fixture".to_string(),
            title: title.to_string(),
            authors: None,
            published,
            category: None,
            pdf: None,
            summary,
            journal_ref: if journal_ref.is_empty() {
                None
            } else {
                Some(journal_ref.to_string())
            },
        }
    }

    #[test]
    fn keyword_hits_are_substrings_case_insensitive() {
        let e = hit(
            "Transiting planet",
            "the atmosphere transmission spectrum shows a water haze",
            "2020-03-01",
            "",
        );
        assert_eq!(score_entry(&e, 1999), 5);
        let plain = hit("the rock", "nothing planetary here", "2020-03-01", "");
        assert_eq!(score_entry(&plain, 1999), 0);
    }

    #[test]
    fn journal_ref_year_weights_only_as_whole_word() {
        assert!(contains_whole_year(
            "Astrophysical Journal Letters 885, L8 (2016)",
            2016
        ));
        assert!(!contains_whole_year("2016ApJ...820...99T", 2016));
        assert!(!contains_whole_year("published in 12016", 2016));
        assert!(!contains_whole_year("c2016", 2016));
        assert!(!contains_whole_year("2016a", 2016));
        assert!(!contains_whole_year("no year here", 2016));
    }

    #[test]
    fn year_proximity_is_bibcode_year_or_year_before() {
        let e = hit("", "", "2014-05-20", "");
        assert_eq!(score_entry(&e, 2014), 2);
        let before = hit("", "", "2013-01-01", "");
        assert_eq!(score_entry(&before, 2014), 2);
        let far = hit("", "", "2012-01-01", "");
        assert_eq!(score_entry(&far, 2014), 0);
        let unknown = hit("", "", "", "");
        assert_eq!(score_entry(&unknown, 2014), 0);
    }

    #[test]
    fn score_weights_add_across_the_three_sources() {
        let e = hit(
            "transmission spectrum",
            "an atmosphere with water haze",
            "2016-03-01",
            "The Astrophysical Journal, 820, 99 (2016)",
        );
        assert_eq!(score_entry(&e, 2016), 3 + 2 + 4);
    }

    #[test]
    fn rank_orders_descending_and_stays_stable_on_ties() {
        let a = hit("", "", "1990-01-01", "J. 1990");
        let b = hit("transit", "", "1991-01-01", "");
        let c = hit("other", "", "1990-01-01", "J. 1990");
        let entries = vec![a, b, c];
        let ranked = rank_entries(&entries, 1990);
        assert_eq!(ranked, vec![(0, 5), (2, 5), (1, 1)]);
    }

    #[test]
    fn query_host_strips_a_trailing_component_letter() {
        assert_eq!(query_host("GJ 1214"), "GJ 1214");
        assert_eq!(query_host("55 Cnc A"), "55 Cnc");
        assert_eq!(query_host("HAT-P-1 A "), "HAT-P-1");
        assert_eq!(query_host("L 168-9"), "L 168-9");
    }

    #[test]
    fn year_parsers_read_the_leading_four_characters() {
        assert_eq!(bibcode_year("2016ApJ...820...99T"), Some(2016));
        assert_eq!(bibcode_year("2026arXiv260631281W"), Some(2026));
        assert_eq!(bibcode_year("nonsense"), None);
        assert_eq!(bibcode_year(""), None);
        assert_eq!(published_year("2015-04-03T12:00:00Z"), Some(2015));
        assert_eq!(published_year(""), None);
        assert_eq!(published_year("nope"), None);
    }

    #[test]
    fn census_reference_drives_the_score() {
        let census = format!(
            "{}/../../docs/surveys/survey-transmission-host-census.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let hosts = read_census_hosts(&census).unwrap();
        assert!(!hosts.is_empty());
        let hst: Vec<&CensusHost> = hosts.iter().filter(|h| h.facility == "HST").collect();
        assert!(!hst.is_empty());
        for h in &hst {
            assert!(!h.hostname.is_empty());
            assert!(bibcode_year(&h.bibcode).is_some());
        }
        let h = hst[0];
        let yr = bibcode_year(&h.bibcode).unwrap();
        let primary_like = hit(
            "transmission spectroscopy",
            "water in the atmosphere",
            &yr.to_string(),
            &format!("Journal, 820 ({}), 99", yr),
        );
        let off_target = hit("the field of view", "", &(yr + 3).to_string(), "");
        let ranked = rank_entries(&[off_target, primary_like], yr);
        assert_eq!(ranked[0].0, 1);
    }

    fn fixture_entry(body: &str) -> String {
        format!("<feed><entry>{}</entry></feed>", body)
    }

    #[test]
    fn parse_carries_the_full_entry_fields() {
        let xml = fixture_entry(
            "<id>http://arxiv.org/abs/2401.01234v1</id>\
             <title>Transmission spectrum of a warm sub-Neptune</title>\
             <summary>A measured atmosphere with haze.</summary>\
             <author><name>Ada Miller</name></author>\
             <author><name>Bo Chen</name></author>\
             <published>2024-01-03T00:00:00Z</published>\
             <arxiv:primary_category term=\"astro-ph.EP\"/>\
             <link href=\"http://arxiv.org/pdf/2401.01234v1\" rel=\"related\" title=\"pdf\"/>\
             <arxiv:journal_ref>Astrophysical Journal, 820, 99 (2016)</arxiv:journal_ref>",
        );
        let parsed = parse_entries(&xml);
        assert_eq!(parsed.len(), 1);
        let e = &parsed[0];
        assert_eq!(e.id, "http://arxiv.org/abs/2401.01234v1");
        assert_eq!(e.title, "Transmission spectrum of a warm sub-Neptune");
        assert_eq!(e.authors.as_deref(), Some("Ada Miller; Bo Chen"));
        assert_eq!(e.published.as_deref(), Some("2024-01-03T00:00:00Z"));
        assert_eq!(e.category.as_deref(), Some("astro-ph.EP"));
        assert_eq!(e.pdf.as_deref(), Some("http://arxiv.org/pdf/2401.01234v1"));
        assert_eq!(
            e.summary.as_deref(),
            Some("A measured atmosphere with haze.")
        );
        assert_eq!(
            e.journal_ref.as_deref(),
            Some("Astrophysical Journal, 820, 99 (2016)")
        );
    }

    #[test]
    fn parse_skips_entries_without_id_or_title() {
        let xml = fixture_entry(
            "<title>No identifier here</title><author><name>Ada Miller</name></author>",
        ) + &fixture_entry(
            "<id>http://arxiv.org/abs/1</id><author><name>Ada Miller</name></author>",
        ) + &fixture_entry("<id>http://arxiv.org/abs/2</id><title></title>")
            + &fixture_entry("<id>http://arxiv.org/abs/3</id><title>Kept entry</title>");
        let parsed = parse_entries(&xml);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].id, "http://arxiv.org/abs/3");
        assert_eq!(parsed[0].title, "Kept entry");
    }

    #[test]
    fn parse_optional_fields_absent_stay_none() {
        let xml = fixture_entry(
            "<id>http://arxiv.org/abs/2401.05678v1</id>\
             <title>Only the identifier and the title</title>\
             <author><name>Ada Miller</name></author>",
        );
        let parsed = parse_entries(&xml);
        assert_eq!(parsed.len(), 1);
        let e = &parsed[0];
        assert_eq!(e.published, None);
        assert_eq!(e.category, None);
        assert_eq!(e.pdf, None);
        assert_eq!(e.summary, None);
        assert_eq!(e.journal_ref, None);
        assert_eq!(e.authors.as_deref(), Some("Ada Miller"));
    }

    #[test]
    fn parse_absent_author_names_stay_none() {
        let xml = fixture_entry(
            "<id>http://arxiv.org/abs/2401.09999v1</id>\
             <title>An entry that names no author</title>",
        );
        let parsed = parse_entries(&xml);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].authors, None);
    }

    #[test]
    fn scoring_keeps_an_absent_abstract_neutral() {
        let e = hit("Transiting planet", "", "2016-03-01", "");
        assert_eq!(score_entry(&e, 2016), 3);
    }
}
