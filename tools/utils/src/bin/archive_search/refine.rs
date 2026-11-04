pub fn split_refine(query: &str, recognized: &[&str]) -> (String, Vec<(String, String)>) {
    let mut text: Vec<&str> = Vec::new();
    let mut opts: Vec<(String, String)> = Vec::new();
    for token in query.split_whitespace() {
        match token.split_once('=') {
            Some((key, value)) if recognized.contains(&key) => {
                opts.push((key.to_string(), value.to_string()));
            }
            _ => text.push(token),
        }
    }
    (text.join(" "), opts)
}

pub fn value_of<'a>(opts: &'a [(String, String)], key: &str) -> Option<&'a str> {
    opts.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifts_only_the_recognized_keys_out_of_the_query() {
        let (text, opts) = split_refine(
            "gravitational waves filter=type:article sort=cited_by_count",
            &["filter", "sort"],
        );
        assert_eq!(text, "gravitational waves");
        assert_eq!(
            opts,
            vec![
                ("filter".to_string(), "type:article".to_string()),
                ("sort".to_string(), "cited_by_count".to_string()),
            ]
        );
    }

    #[test]
    fn a_plain_query_carries_no_refinement() {
        let (text, opts) = split_refine("gravitational waves", &["filter"]);
        assert_eq!(text, "gravitational waves");
        assert!(opts.is_empty());
    }

    #[test]
    fn an_unrecognized_key_equals_stays_in_the_free_text() {
        let (text, opts) = split_refine("search=dark matter", &["filter"]);
        assert_eq!(text, "search=dark matter");
        assert!(opts.is_empty());
    }

    #[test]
    fn value_of_reads_the_first_match() {
        let opts = vec![
            ("sort".to_string(), "date".to_string()),
            ("sort".to_string(), "relevance".to_string()),
        ];
        assert_eq!(value_of(&opts, "sort"), Some("date"));
        assert_eq!(value_of(&opts, "fl"), None);
    }
}
