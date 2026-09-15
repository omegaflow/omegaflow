use crate::net::{get, urlencode};

const ISC_ENDPOINT: &str = "http://www.isc.ac.uk/fdsnws/event/1/query";

fn parameter(key: &str) -> Option<&'static str> {
    match key {
        "start" => Some("starttime"),
        "end" => Some("endtime"),
        "minmag" => Some("minmagnitude"),
        "minlat" => Some("minlatitude"),
        "maxlat" => Some("maxlatitude"),
        "minlon" => Some("minlongitude"),
        "maxlon" => Some("maxlongitude"),
        _ => None,
    }
}

fn query_url(query: &str) -> String {
    let mut url = format!("{}?format=text", ISC_ENDPOINT);
    for token in query.split_whitespace() {
        let Some((key, value)) = token.split_once('=') else {
            continue;
        };
        if let Some(name) = parameter(key) {
            url.push_str(&format!("&{}={}", name, urlencode(value)));
        }
    }
    url
}

pub fn isc_lines(query: &str, max: usize) -> Vec<String> {
    if !query.split_whitespace().any(|token| {
        token
            .split_once('=')
            .map_or(false, |(key, _)| parameter(key).is_some())
    }) {
        return vec![
            "usage — isc needs key=value: start/end/minmag/minlat/maxlat/minlon/maxlon".to_string(),
        ];
    }
    let url = query_url(query);
    match get(&url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            let mut out = parse_isc(&f.body);
            out.truncate(max);
            if out.is_empty() {
                vec![format!("absent — isc carries no entry: {}", query)]
            } else {
                out
            }
        }
        Some(f) => {
            let code = match f.status {
                Some(s) => s.to_string(),
                None => "absent".to_string(),
            };
            vec![format!("pending — isc HTTP {}", code)]
        }
        None => vec!["pending — no network".to_string()],
    }
}

fn parse_isc(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in body.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('|').collect();
        let (Some(id), Some(time), Some(lat), Some(lon), Some(depth), Some(mag)) = (
            cols.first(),
            cols.get(1),
            cols.get(2),
            cols.get(3),
            cols.get(4),
            cols.get(10),
        ) else {
            continue;
        };
        if id.is_empty() || lat.is_empty() || lon.is_empty() {
            continue;
        }
        out.push(format!(
            "event {}\ttime {}\tlat {}\tlon {}\tdepth {}\tmag {}",
            id, time, lat, lon, depth, mag
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_pipe_delimited_rows() {
        let body = "#EventID|Time|Latitude|Longitude|Depth/km|Author|Catalog|Contributor|ContributorID|MagType|Magnitude|MagAuthor|EventLocationName|EventType\n\
            617049575|2020-01-01T00:28:20.792|-5.3600|152.5302|45.3|ISC|ISC|ISC|616087894|MW|4.96|GCMT|New Britain region|earthquake\n\
            617050226|2020-01-01T03:53:27.504|52.5104|159.3492|52.6|ISC|ISC|ISC|616087915|MW|4.96|GCMT|Off east coast of Kamchatka Peninsula|earthquake\n\
            # Agencies whose data contributed towards the results of this search:\n";
        assert_eq!(
            parse_isc(body),
            vec![
                "event 617049575\ttime 2020-01-01T00:28:20.792\tlat -5.3600\tlon 152.5302\tdepth 45.3\tmag 4.96".to_string(),
                "event 617050226\ttime 2020-01-01T03:53:27.504\tlat 52.5104\tlon 159.3492\tdepth 52.6\tmag 4.96".to_string(),
            ]
        );
    }

    #[test]
    fn empty_response_carries_nothing() {
        assert!(parse_isc("").is_empty());
        assert!(parse_isc("#EventID|Time|Latitude\n").is_empty());
    }

    #[test]
    fn unknown_keys_are_ignored_and_values_encoded() {
        let url = query_url("start=2020-01-01 end=2020-01-02 minmag=5 bogus=1");
        assert!(url.contains("starttime=2020-01-01"));
        assert!(url.contains("endtime=2020-01-02"));
        assert!(url.contains("minmagnitude=5"));
        assert!(!url.contains("bogus"));
    }

    #[test]
    fn bare_query_is_usage_not_network() {
        let out = isc_lines("Pioneer 10", 10);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("usage — isc needs key=value"));
    }
}
