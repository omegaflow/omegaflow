use crate::json::{self, Json};
use crate::net::{get, urlencode};

const SUPERMAG_SERVICES: &str = "https://supermag.jhuapl.edu/services";

fn parameter(query: &str, key: &str) -> Option<String> {
    query.split_whitespace().find_map(|token| {
        let (name, value) = token.split_once('=')?;
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn data_url(logon: &str, station: &str, start: &str, end: &str) -> String {
    format!(
        "{}/data-api.php?logon={}&start={}&end={}&station={}",
        SUPERMAG_SERVICES,
        urlencode(logon),
        urlencode(start),
        urlencode(end),
        urlencode(station)
    )
}

fn inventory_url(logon: &str, start: &str, extent: &str) -> String {
    format!(
        "{}/inventory.php?logon={}&start={}&extent={}",
        SUPERMAG_SERVICES,
        urlencode(logon),
        urlencode(start),
        urlencode(extent)
    )
}

fn number(value: Option<&Json>) -> Option<String> {
    match value {
        Some(Json::Num(n)) if n.is_finite() => Some(format!("{}", n)),
        _ => None,
    }
}

fn entry_line(entry: &Json) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(iaga) = entry.get("iaga").and_then(|v| v.as_str()) {
        if !iaga.is_empty() {
            parts.push(format!("station {}", iaga));
        }
    }
    if let Some(tval) = number(entry.get("tval")) {
        parts.push(format!("t {}", tval));
    }
    if let Some(north) = entry.get("N") {
        if let Some(nez) = number(north.get("nez")) {
            parts.push(format!("N_nez {}", nez));
        }
        if let Some(geo) = number(north.get("geo")) {
            parts.push(format!("N_geo {}", geo));
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\t"))
    }
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").trim().to_string()
}

fn parse_supermag(body: &str) -> Vec<String> {
    let text = body.trim_start();
    let Some(rest) = text.strip_prefix("OK") else {
        return vec![format!(
            "pending — supermag: non-OK body ({})",
            first_line(text)
        )];
    };
    let rest = rest.trim_start();
    if rest.starts_with("ERROR") {
        return vec![format!("pending — supermag: {}", first_line(rest))];
    }
    if rest.is_empty() {
        return vec!["absent — supermag: the OK body carries no rows".to_string()];
    }
    match json::parse(rest).and_then(|v| v.as_arr().map(<[Json]>::to_vec)) {
        Some(rows) => rows.iter().filter_map(entry_line).collect(),
        None => vec![format!(
            "pending — supermag: the OK body carries no JSON array ({})",
            first_line(rest)
        )],
    }
}

fn parse_inventory(body: &str) -> Vec<String> {
    let text = body.trim_start();
    let Some(rest) = text.strip_prefix("OK") else {
        return vec![format!(
            "pending — supermag: non-OK body ({})",
            first_line(text)
        )];
    };
    if rest.trim_start().starts_with("ERROR") {
        return vec![format!("pending — supermag: {}", first_line(rest))];
    }
    rest.trim_start()
        .lines()
        .skip(1)
        .map(str::trim)
        .filter(|station| !station.is_empty())
        .map(|station| format!("station {}", station))
        .collect()
}

fn fetch_supermag(url: &str, inventory: bool) -> Vec<String> {
    match get(url, &[], "40") {
        Some(f) if f.status == Some(200) => {
            if inventory {
                parse_inventory(&f.body)
            } else {
                parse_supermag(&f.body)
            }
        }
        Some(f) => vec![format!("pending — supermag HTTP {}", f.status_text())],
        None => vec!["pending — no network".to_string()],
    }
}

pub fn supermag_lines(query: &str, max: usize, user: Option<&str>) -> Vec<String> {
    let station = parameter(query, "station");
    let start = parameter(query, "start");
    let end = parameter(query, "end");
    let extent = parameter(query, "extent");
    let logon = user.unwrap_or("omegaflow");

    let (url, inventory) = match (station.as_deref(), start.as_deref(), end.as_deref()) {
        (Some(station), Some(start), Some(end)) => (data_url(logon, station, start, end), false),
        (_, Some(start), None) => match extent.as_deref() {
            Some(extent) => (inventory_url(logon, start, extent), true),
            None => {
                return vec![format!(
                    "pending — supermag: query names no extent for the inventory: {}",
                    query
                )];
            }
        },
        _ => {
            return vec![format!(
                "pending — supermag: query names no station/start/end: {}",
                query
            )];
        }
    };

    let mut out = fetch_supermag(&url, inventory);
    if out.iter().any(|line| line.contains("non-OK body")) {
        out = fetch_supermag(&url, inventory);
    }
    out.truncate(max);
    if out.is_empty() {
        vec![format!("absent — supermag carries no entry: {}", query)]
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_ok_array_into_station_lines() {
        let body = "OK\n[{\"tval\":1426550400.0,\"ext\":60.0,\"iaga\":\"ABK\",\"N\":{\"nez\":123.4,\"geo\":65.0}}]";
        assert_eq!(
            parse_supermag(body),
            vec!["station ABK\tt 1426550400\tN_nez 123.4\tN_geo 65".to_string()]
        );
    }

    #[test]
    fn missing_fields_are_omitted_not_fabricated() {
        let body = "OK\n[{\"tval\":1426550400.0,\"iaga\":\"ABK\",\"N\":{\"nez\":60.0}}]";
        assert_eq!(
            parse_supermag(body),
            vec!["station ABK\tt 1426550400\tN_nez 60".to_string()]
        );
    }

    #[test]
    fn error_line_flows_as_pending() {
        assert_eq!(
            parse_supermag("OK\nERROR: No username"),
            vec!["pending — supermag: ERROR: No username".to_string()]
        );
    }

    #[test]
    fn non_ok_body_flows_as_pending_not_absent() {
        let body = "<br />\n<b>Warning</b>:  shell_exec(): Unable to execute";
        let out = parse_supermag(body);
        assert_eq!(out.len(), 1);
        assert!(out[0].starts_with("pending — supermag: non-OK body"));
    }

    #[test]
    fn empty_array_carries_nothing() {
        assert!(parse_supermag("OK\n[]").is_empty());
    }

    #[test]
    fn a_bare_ok_names_the_absence() {
        assert_eq!(
            parse_supermag("OK\n"),
            vec!["absent — supermag: the OK body carries no rows".to_string()]
        );
    }

    #[test]
    fn inventory_reads_the_station_register() {
        assert_eq!(
            parse_inventory("OK\n2\nABK\nBJN\n"),
            vec!["station ABK".to_string(), "station BJN".to_string()]
        );
    }

    #[test]
    fn query_tokens_build_the_data_url() {
        let url = data_url(
            "omegaflow",
            "abk",
            "2020-01-01T00:00:00",
            "2020-01-01T01:00:00",
        );
        assert!(url.contains("logon=omegaflow"));
        assert!(url.contains("station=abk"));
        assert!(url.contains("start=2020-01-01T00%3A00%3A00"));
    }

    #[test]
    fn query_tokens_build_the_inventory_url() {
        let url = inventory_url("omegaflow", "2020-01-01T00:00:00", "3600");
        assert!(url.contains("logon=omegaflow"));
        assert!(url.contains("extent=3600"));
        assert!(!url.contains("station="));
    }

    #[test]
    fn a_registered_user_replaces_the_default_logon() {
        let url = data_url("j.t", "abk", "2020-01-01T00:00:00", "2020-01-01T01:00:00");
        assert!(url.contains("logon=j.t"));
        assert!(!url.contains("logon=omegaflow"));
    }

    #[test]
    fn parameter_reads_the_named_token() {
        assert_eq!(
            parameter("station=abk start=2020-01-01", "station").as_deref(),
            Some("abk")
        );
        assert!(parameter("station=abk", "end").is_none());
    }
}
