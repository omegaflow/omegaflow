use crate::json::{self, Json};
use crate::net::{get, json_escape, post, urlencode};

const CN_ENDPOINT: &str = "https://www.nssdc.ac.cn/nssdc/coreMetadata/coreMetadataList";
const NASA_QUERY: &str = "https://nssdc.gsfc.nasa.gov/nmc/dataset/query";
const NASA_DISPLAY: &str = "https://nssdc.gsfc.nasa.gov/nmc/dataset/display.action";

fn field(entry: &Json, key: &str) -> Option<String> {
    let value = entry.get(key)?.as_str()?;
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn cn_entry_line(entry: &Json) -> Option<String> {
    let name = field(entry, "dataNameEn").or_else(|| field(entry, "dataNameCh"))?;
    let mut parts = Vec::new();
    if let Some(id) = entry.get("id").and_then(|v| v.as_scalar_string()) {
        parts.push(format!("id {id}"));
    }
    if let Some(dataset) = field(entry, "datasetId") {
        parts.push(format!("dataset {dataset}"));
    }
    if let Some(doi) = field(entry, "doi") {
        parts.push(format!("doi {doi}"));
    }
    parts.push(format!("name {name}"));
    if let (Some(begin), Some(end)) = (field(entry, "timeSpanBegin"), field(entry, "timeSpanEnd")) {
        parts.push(format!("span {begin}..{end}"));
    }
    Some(parts.join("\t"))
}

pub fn nssdc_lines(query: &str, max: usize) -> Vec<String> {
    let keywords = query.trim();
    if keywords.is_empty() {
        return vec!["pending — nssdc: empty query".to_string()];
    }
    let body = format!(
        "{{\"releaseStatus\":5,\"releaseDateSort\":\"DESC\",\"datasetViewsSort\":{{\"sort\":null}},\"searchKeywords\":\"{}\",\"pageSize\":{},\"pageNum\":1,\"slidervalue\":20}}",
        json_escape(keywords),
        max
    );
    match post(CN_ENDPOINT, &body, &[], "40") {
        Some(f) if f.status == Some(200) => parse_cn(&f.body),
        Some(f) => vec![format!("pending — nssdc HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_cn(body: &str) -> Vec<String> {
    let Some(root) = json::parse(body) else {
        return vec!["pending — nssdc: the body carries no JSON".to_string()];
    };
    if let Some(code) = root.get("code").and_then(|v| v.as_scalar_string()) {
        if code != "0" {
            return vec![format!("pending — nssdc: code {code}")];
        }
    }
    let Some(entries) = root
        .get("data")
        .and_then(|d| d.get("coreMetadataList"))
        .and_then(|v| v.as_arr())
    else {
        return vec!["pending — nssdc: the body carries no coreMetadataList".to_string()];
    };
    let lines: Vec<String> = entries.iter().filter_map(cn_entry_line).collect();
    if lines.is_empty() {
        vec!["absent — nssdc: no dataset matches".to_string()]
    } else {
        lines
    }
}

fn looks_like_id(query: &str) -> bool {
    let Some((head, tail)) = query.split_once('-') else {
        return false;
    };
    head.len() == 4
        && head.chars().all(|c| c.is_ascii_uppercase())
        && !tail.is_empty()
        && tail.chars().all(|c| c.is_ascii_digit())
}

fn nasa_form_fields(query: &str) -> Vec<(String, String)> {
    if let Some((key, value)) = query.split_once('=') {
        let key = key.trim();
        let field = match key {
            "name" => Some("name"),
            "spacecraft" => Some("spacecraftName"),
            "experiment" => Some("experimentName"),
            "pi" => Some("piName"),
            _ => None,
        };
        if let Some(field) = field {
            return vec![(field.to_string(), value.trim().to_string())];
        }
    }
    vec![("name".to_string(), query.trim().to_string())]
}

pub fn nssdca_lines(query: &str, max: usize) -> Vec<String> {
    let query = query.trim();
    if query.is_empty() {
        return vec!["pending — nssdca: empty query".to_string()];
    }
    if query.split_whitespace().count() == 1 && looks_like_id(query) {
        return nasa_detail(query);
    }
    let mut extra: Vec<String> = vec!["-X".to_string(), "POST".to_string()];
    for (key, value) in nasa_form_fields(query) {
        extra.push("--data-urlencode".to_string());
        extra.push(format!("{key}={value}"));
    }
    let refs: Vec<&str> = extra.iter().map(String::as_str).collect();
    match get(NASA_QUERY, &refs, "40") {
        Some(f) if f.status == Some(200) => {
            let lines = nasa_query_lines(&f.body, max);
            if lines.is_empty() {
                vec![format!(
                    "absent — nssdca: no data collection matches {query}"
                )]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — nssdca HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn nasa_query_lines(body: &str, max: usize) -> Vec<String> {
    let marker = "/nmc/dataset/display.action?id=";
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(i) = rest.find(marker) {
        let after = &rest[i + marker.len()..];
        let Some(quote) = after.find('"') else {
            break;
        };
        let id = &after[..quote];
        let link = &after[quote..];
        let Some(fragment) = between(link, ">", "</a>") else {
            break;
        };
        out.push(format!("{}\t{}", id, text_of(fragment)));
        if out.len() >= max {
            break;
        }
        rest = &after[quote..];
    }
    out
}

fn nasa_detail(id: &str) -> Vec<String> {
    let url = format!("{NASA_DISPLAY}?id={}", urlencode(id));
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let lines = parse_nasa_detail(&f.body);
            if lines.is_empty() {
                vec![format!("absent — nssdca: no details for {id}")]
            } else {
                lines
            }
        }
        Some(f) => vec![format!("pending — nssdca HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_nasa_detail(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(title) = between(body, "<h1>", "</h1>") {
        out.push(format!("title\t{}", text_of(title)));
    }
    if let Some(id) = between(body, "NSSDCA ID:</strong>", "<") {
        out.push(format!("id\t{}", text_of(id)));
    }
    if let Some(availability) = between(body, "Availability:</strong>", "<") {
        out.push(format!("availability\t{}", text_of(availability)));
    }
    if let Some(old) = between(body, "Old NSSDCA ID", "<") {
        if let Some(value) = between(old, ">", "<") {
            if let Some(trimmed) = non_empty(text_of(value)) {
                out.push(format!("old_id\t{trimmed}"));
            }
        }
    }
    if let Some(spacecraft) = link_text(body, "/nmc/spacecraft/display.action?id=") {
        out.push(format!("spacecraft\t{spacecraft}"));
    }
    if let Some(experiment) = link_text(body, "/nmc/experiment/display.action?id=") {
        out.push(format!("experiment\t{experiment}"));
    }
    out
}

fn link_text(body: &str, marker: &str) -> Option<String> {
    let i = body.find(marker)?;
    let after = &body[i + marker.len()..];
    let quote = after.find('"')?;
    let link = &after[quote..];
    let name = between(link, ">", "</a>").map(text_of)?;
    non_empty(name)
}

fn non_empty(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

fn between<'a>(hay: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let i = hay.find(start)? + start.len();
    let rest = &hay[i..];
    let j = rest.find(end)?;
    Some(&rest[..j])
}

fn text_of(fragment: &str) -> String {
    decode_entities(&strip_tags(fragment))
}

fn strip_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut inside = false;
    for c in input.chars() {
        match c {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => out.push(c),
            _ => {}
        }
    }
    out
}

fn decode_entities(input: &str) -> String {
    let mut out = input.replace("&amp;", "&");
    for (entity, replacement) in [
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&apos;", "'"),
        ("&nbsp;", " "),
        ("&ndash;", "-"),
        ("&mdash;", "-"),
        ("&rsquo;", "'"),
    ] {
        out = out.replace(entity, replacement);
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_result_rows_carry_id_and_name() {
        let body = "<p>There were 2 data collections returned.</p><table><tbody>\
<tr><td><a href=\"/nmc/dataset/display.action?id=PSCM-00002\">SDDPT Mariner 9 &amp; CM</a></td><td>x</td></tr>\
<tr><td><a href=\"/nmc/dataset/display.action?id=PSCM-00009\">SDDPT Mariner 10 CM/RSS</a></td><td>y</td></tr>\
</tbody></table>";
        assert_eq!(
            nasa_query_lines(body, 10),
            vec![
                "PSCM-00002\tSDDPT Mariner 9 & CM".to_string(),
                "PSCM-00009\tSDDPT Mariner 10 CM/RSS".to_string(),
            ]
        );
    }

    #[test]
    fn empty_result_carries_no_row() {
        let body = "<p>There were 0 data collections returned.</p>";
        assert!(nasa_query_lines(body, 10).is_empty());
    }

    #[test]
    fn the_row_cap_stops_the_walk() {
        let body = "<a href=\"/nmc/dataset/display.action?id=A-1\">one</a><a href=\"/nmc/dataset/display.action?id=A-2\">two</a>";
        assert_eq!(nasa_query_lines(body, 1), vec!["A-1\tone".to_string()]);
    }

    #[test]
    fn detail_reads_the_named_fields() {
        let body = "<h1>Mariner 10 CM</h1><p><strong>NSSDCA ID:</strong> PSCM-00009</p>\
<p><strong>Availability:</strong> At NSSDC, Ready for Offline Distribution</p>\
<a href=\"/nmc/spacecraft/display.action?id=1973-085A\">Mariner 10</a>\
<a href=\"/nmc/experiment/display.action?id=1973-085A-02\">Celestial Mechanics and Radio Science</a>";
        assert_eq!(
            parse_nasa_detail(body),
            vec![
                "title\tMariner 10 CM".to_string(),
                "id\tPSCM-00009".to_string(),
                "availability\tAt NSSDC, Ready for Offline Distribution".to_string(),
                "spacecraft\tMariner 10".to_string(),
                "experiment\tCelestial Mechanics and Radio Science".to_string(),
            ]
        );
    }

    #[test]
    fn an_id_query_is_recognized() {
        assert!(looks_like_id("PSCM-00009"));
        assert!(looks_like_id("PSNO-00007"));
        assert!(!looks_like_id("Mariner 10"));
        assert!(!looks_like_id("pscm-00009"));
        assert!(!looks_like_id("PSCM-9A"));
    }

    #[test]
    fn named_keys_build_the_form_fields() {
        assert_eq!(
            nasa_form_fields("spacecraft=Mariner 10"),
            vec![("spacecraftName".to_string(), "Mariner 10".to_string())]
        );
        assert_eq!(
            nasa_form_fields("Mariner 10"),
            vec![("name".to_string(), "Mariner 10".to_string())]
        );
    }

    #[test]
    fn unknown_key_stays_a_name_query() {
        assert_eq!(
            nasa_form_fields("foo=bar"),
            vec![("name".to_string(), "foo=bar".to_string())]
        );
    }

    #[test]
    fn cn_entry_renders_the_measured_fields() {
        let entry = json::parse(
            r#"{"id":16227,"datasetId":"01.06.00148-V01","doi":"10.12176/01.06.00148-V01","dataNameEn":"CSES ULF background","dataNameCh":"磁场产品","timeSpanBegin":"2019-01-01 00:00:00","timeSpanEnd":"2019-12-31 00:00:00"}"#,
        )
        .unwrap();
        assert_eq!(
            cn_entry_line(&entry),
            Some("id 16227\tdataset 01.06.00148-V01\tdoi 10.12176/01.06.00148-V01\tname CSES ULF background\tspan 2019-01-01 00:00:00..2019-12-31 00:00:00".to_string())
        );
    }

    #[test]
    fn cn_parse_reads_the_list_and_names_absence() {
        let body = r#"{"msg":"ok","code":0,"data":{"coreMetadataList":[]}}"#;
        assert_eq!(
            parse_cn(body),
            vec!["absent — nssdc: no dataset matches".to_string()]
        );
        let pending = parse_cn(r#"{"code":500}"#);
        assert_eq!(pending, vec!["pending — nssdc: code 500".to_string()]);
    }

    #[test]
    fn entities_decode_to_text() {
        assert_eq!(decode_entities("A &amp; B &lt;x&gt;"), "A & B <x>");
        assert_eq!(strip_tags("<b>bold</b> plain"), "bold plain");
    }
}
