use crate::net::{get, urlencode};

pub const LPF_SELECTOR: &str = "https://heasarc.gsfc.nasa.gov/lpf/cgi/selector";
pub const LPF_DEFAULT_HDU: &str = "SCI_SCIENCE_1Hz";

pub struct LpfArgs {
    pub start: String,
    pub end: String,
    pub hdu: String,
}

pub fn parse_args(query: &str) -> Option<LpfArgs> {
    let mut tokens = query.split_whitespace();
    let start = tokens.next()?.to_string();
    let end = tokens.next()?.to_string();
    let hdu = match tokens.next() {
        Some(hdu) => hdu.to_string(),
        None => LPF_DEFAULT_HDU.to_string(),
    };
    Some(LpfArgs { start, end, hdu })
}

pub fn selector_url(args: &LpfArgs) -> String {
    format!(
        "{}?start={}&end={}&hdu={}",
        LPF_SELECTOR,
        urlencode(&args.start),
        urlencode(&args.end),
        urlencode(&args.hdu)
    )
}

fn header_value(headers: &str, name: &str) -> Option<String> {
    headers.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        if key.trim().eq_ignore_ascii_case(name) {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

fn response_summary(raw: &[u8]) -> (String, usize) {
    match raw.windows(4).position(|w| w == b"\r\n\r\n") {
        Some(pos) => {
            let headers = String::from_utf8_lossy(&raw[..pos]);
            let content_type = match header_value(&headers, "content-type") {
                Some(v) => v,
                None => "absent".to_string(),
            };
            let body = &raw[pos + 4..];
            let bytes = match header_value(&headers, "content-length")
                .and_then(|v| v.parse::<usize>().ok())
            {
                Some(n) => n,
                None => body.len(),
            };
            (content_type, bytes)
        }
        None => ("absent".to_string(), raw.len()),
    }
}

pub fn lpf_lines(query: &str) -> Vec<String> {
    let args = match parse_args(query) {
        Some(args) => args,
        None => return vec!["pending — --lpf needs <start> <end> [hdu]".to_string()],
    };
    let url = selector_url(&args);
    match get(&url, &["-D", "-"], "40") {
        Some(f) if f.status == Some(200) => {
            let (content_type, bytes) = response_summary(&f.raw);
            vec![format!(
                "lpf {} {}..{} · HTTP 200 · {} · {} bytes",
                args.hdu, args.start, args.end, content_type, bytes
            )]
        }
        Some(f) => vec![format!("pending — lpf HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_reads_tokens_and_defaults_the_hdu() {
        let a = parse_args("57450.0 57450.05").unwrap();
        assert_eq!(a.start, "57450.0");
        assert_eq!(a.end, "57450.05");
        assert_eq!(a.hdu, LPF_DEFAULT_HDU);
        assert_eq!(
            selector_url(&a),
            "https://heasarc.gsfc.nasa.gov/lpf/cgi/selector?start=57450.0&end=57450.05&hdu=SCI_SCIENCE_1Hz"
        );

        let b = parse_args("2020-01-01T00:00:00 2020-01-02T00:00:00 HOUSEKEEPING").unwrap();
        assert_eq!(b.hdu, "HOUSEKEEPING");
        assert_eq!(
            selector_url(&b),
            "https://heasarc.gsfc.nasa.gov/lpf/cgi/selector?start=2020-01-01T00%3A00%3A00&end=2020-01-02T00%3A00%3A00&hdu=HOUSEKEEPING"
        );
    }

    #[test]
    fn fewer_than_two_tokens_carries_no_args() {
        assert!(parse_args("57450.0").is_none());
        assert!(parse_args("").is_none());
    }

    #[test]
    fn response_summary_reads_type_and_length() {
        let raw =
            b"HTTP/1.1 200 OK\r\nContent-Type: application/fits\r\nContent-Length: 5\r\n\r\nSIMPLE";
        assert_eq!(response_summary(raw), ("application/fits".to_string(), 5));
    }
}
