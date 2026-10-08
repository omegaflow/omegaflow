use crate::json::{self, Json};
use crate::secrets::Secret;
use std::process::Command;

const LASAIR_TOKEN_KEY: &str = "LASAIR_LSST_TOKEN";
const LASAIR_SOCKS: &str = "socks5h://127.0.0.1:25344";

pub fn object_url(object_id: &str) -> String {
    format!(
        "https://api.lasair.lsst.ac.uk/api/object/?objectId={}&format=json",
        crate::net::urlencode(object_id)
    )
}

struct Response {
    status: Option<i32>,
    body: String,
}

fn curl_get(url: &str, token: &str, proxy: Option<&str>) -> Option<Response> {
    let auth = format!("Authorization: Token {}", token);
    let mut cmd = Command::new("curl");
    cmd.args(["-sS", "-g", "--compressed", "--max-time", "40"]);
    cmd.arg("-H").arg(&auth);
    if let Some(p) = proxy {
        cmd.arg("--proxy").arg(p);
    }
    cmd.args(["-o", "-", "-w", "\n%{http_code}"]);
    cmd.arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some(Response {
        status: code.parse::<i32>().ok(),
        body: String::from_utf8_lossy(&stdout[..idx]).to_string(),
    })
}

fn fetch(url: &str, token: &str) -> Option<Response> {
    match curl_get(url, token, None) {
        Some(r) if r.status.is_some() && r.status != Some(0) => Some(r),
        _ => curl_get(url, token, Some(LASAIR_SOCKS)),
    }
}

fn sanitize_json(body: &str) -> String {
    let chars: Vec<char> = body.chars().collect();
    let mut out = String::with_capacity(body.len());
    let mut i = 0;
    let mut in_string = false;
    while i < chars.len() {
        let c = chars[i];
        if in_string {
            out.push(c);
            if c == '\\' {
                if i + 1 < chars.len() {
                    out.push(chars[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
                continue;
            }
            if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
            continue;
        }
        if chars[i..].starts_with(&['N', 'a', 'N']) {
            out.push_str("null");
            i += 3;
            continue;
        }
        if chars[i..].starts_with(&['-', 'I', 'n', 'f', 'i', 'n', 'i', 't', 'y']) {
            out.push_str("null");
            i += 9;
            continue;
        }
        if chars[i..].starts_with(&['I', 'n', 'f', 'i', 'n', 'i', 't', 'y']) {
            out.push_str("null");
            i += 8;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

fn field_scalar(v: &Json) -> Option<String> {
    match v {
        Json::Str(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Json::Num(n) if n.is_finite() => v.as_scalar_string(),
        Json::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

const PREFERRED: &[&str] = &[
    "diaObjectId",
    "objectId",
    "ra",
    "dec",
    "decl",
    "firstDiaSourceMjdTai",
    "lastDiaSourceMjdTai",
    "nDiaSources",
];

fn object_line(v: &Json) -> Option<String> {
    let root = match v {
        Json::Arr(items) => items.first()?,
        _ => v,
    };
    let Json::Obj(top) = root else {
        return None;
    };
    let source = match top.get("lasairData") {
        Some(Json::Obj(m)) => m,
        _ => match top.get("diaObject") {
            Some(Json::Obj(m)) => m,
            _ => top,
        },
    };
    let mut parts: Vec<String> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    if let Some(id) = top
        .get("diaObjectId")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        parts.push(format!("diaObjectId {}", id));
        seen.push("diaObjectId");
    }
    for key in PREFERRED {
        if seen.contains(key) {
            continue;
        }
        if let Some(field) = source.get(*key).and_then(field_scalar) {
            parts.push(format!("{} {}", key, field));
            seen.push(key);
        }
    }
    let mut rest: Vec<&String> = source
        .keys()
        .filter(|k| !seen.contains(&k.as_str()))
        .collect();
    rest.sort();
    for key in rest {
        if let Some(field) = source.get(key).and_then(field_scalar) {
            parts.push(format!("{} {}", key, field));
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}

pub fn lasair_lines(query: &str) -> Vec<String> {
    let object_id = query.trim();
    if object_id.is_empty() {
        return vec!["absent — lasair carries no objectId".to_string()];
    }
    let env = match crate::find_repo_root() {
        Some(repo) => crate::secrets::load_env(&repo),
        None => std::env::vars().collect(),
    };
    let token = match crate::secrets::resolve_key(
        env.get(LASAIR_TOKEN_KEY).map(String::as_str).unwrap_or(""),
        &env,
    ) {
        Secret::Value(t) => t,
        Secret::Absent(_) => {
            return vec![format!(
                "pending — {} absent from .secrets.local/.env",
                LASAIR_TOKEN_KEY
            )];
        }
    };
    let url = object_url(object_id);
    match fetch(&url, &token) {
        Some(r) if r.status == Some(200) => match json::parse(&sanitize_json(&r.body)) {
            Some(v) => match object_line(&v) {
                Some(l) => vec![l],
                None => vec![format!(
                    "absent — the Lasair register carries no object: {}",
                    object_id
                )],
            },
            None => vec!["pending — the Lasair response carries no JSON".to_string()],
        },
        Some(r) => {
            let status = match r.status {
                Some(s) => s.to_string(),
                None => "absent".to_string(),
            };
            vec![format!("pending — lasair HTTP {}", status)]
        }
        None => vec!["pending — no network".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_url_builds_the_object_endpoint() {
        assert_eq!(
            object_url("313998569858662581"),
            "https://api.lasair.lsst.ac.uk/api/object/?objectId=313998569858662581&format=json"
        );
    }

    #[test]
    fn object_url_encodes_the_object_id() {
        assert_eq!(
            object_url("a b"),
            "https://api.lasair.lsst.ac.uk/api/object/?objectId=a%20b&format=json"
        );
    }

    #[test]
    fn sanitize_replaces_bare_non_finite_tokens_outside_strings() {
        assert_eq!(
            sanitize_json(r#"{"a":NaN,"b":"NaN","c":-Infinity,"d":Infinity,"e":1}"#),
            r#"{"a":null,"b":"NaN","c":null,"d":null,"e":1}"#
        );
    }

    #[test]
    fn object_line_reads_the_dia_object_fields() {
        let body = r#"{"diaObject":{"diaObjectId":"313998569858662581","ra":148.87,"decl":2.52,"nDiaSources":4}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            object_line(&v),
            Some(
                "diaObjectId 313998569858662581 · ra 148.87 · decl 2.52 · nDiaSources 4"
                    .to_string()
            )
        );
    }

    #[test]
    fn object_line_reads_the_lasair_data_wrapper() {
        let body = r#"{"diaObjectId":"313998569858662581","lasairData":{"diaObjectId":313998569858662581,"ra":148.87,"dec":2.52}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(
            object_line(&v),
            Some("diaObjectId 313998569858662581 · ra 148.87 · dec 2.52".to_string())
        );
    }

    #[test]
    fn object_line_is_absent_without_scalar_fields() {
        let body = r#"{"diaObject":{"diaSourcesList":[]}}"#;
        let v = json::parse(body).unwrap();
        assert_eq!(object_line(&v), None);
    }
}
