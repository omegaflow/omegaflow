use crate::index::{Index, Kind, Query, Sort};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub const MODES: &[&str] = &[
    "local",
    "leads",
    "mft",
    "index",
    "git",
    "verdict",
    "arxiv",
    "ads",
    "ntrs",
    "wayback",
    "crossref",
    "wiki",
    "github",
    "crates",
    "librs",
    "openalex",
    "pubmed",
    "europepmc",
    "psychporta",
    "awmf",
    "cochrane",
    "core",
    "materialsproject",
    "semanticscholar",
    "clinicaltrials",
    "openfda",
    "pubchem",
    "uniprot",
    "pdb",
    "chembl",
    "ensembl",
    "entrez",
    "doaj",
    "go",
    "unpaywall",
    "reactome",
    "interpro",
    "alphafold",
];

pub struct AppState {
    pub index: RwLock<Index>,
    pub roots: Vec<PathBuf>,
    pub mft: Option<PathBuf>,
    pub repo: PathBuf,
    pub env: HashMap<String, String>,
}

pub fn run(state: Arc<AppState>, bind: &str) -> Result<(), String> {
    let listener = TcpListener::bind(bind).map_err(|e| format!("{bind} binds not: {e}"))?;
    eprintln!("archive_search --serve: http://{bind}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state = state.clone();
                std::thread::spawn(move || {
                    let _ = handle(stream, state);
                });
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

fn handle(mut stream: TcpStream, state: Arc<AppState>) -> Result<(), String> {
    let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|e| e.to_string())?;
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        if n == 0 || line == "\r\n" || line == "\n" {
            break;
        }
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    if method != "GET" {
        return respond(&mut stream, 405, "text/plain; charset=utf-8", "only GET");
    }
    let (path, query) = match target.split_once('?') {
        Some((p, q)) => (p, q),
        None => (target, ""),
    };
    let params = parse_params(query);
    match path {
        "/" => respond(
            &mut stream,
            200,
            "text/html; charset=utf-8",
            crate::web::PAGE,
        ),
        "/api/run" => {
            let mode = param(&params, "mode");
            let q = param(&params, "q");
            let body = api_run(&state, &mode, &q);
            respond(&mut stream, 200, "application/json; charset=utf-8", &body)
        }
        "/api/status" => {
            let body = api_status(&state);
            respond(&mut stream, 200, "application/json; charset=utf-8", &body)
        }
        "/api/modes" => {
            let body = api_modes();
            respond(&mut stream, 200, "application/json; charset=utf-8", &body)
        }
        _ => respond(&mut stream, 404, "text/plain; charset=utf-8", "not found"),
    }
}

fn respond(
    stream: &mut TcpStream,
    code: u16,
    content_type: &str,
    body: &str,
) -> Result<(), String> {
    let reason = match code {
        200 => "OK",
        405 => "Method Not Allowed",
        _ => "Not Found",
    };
    let head = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n",
        body.len()
    );
    stream
        .write_all(head.as_bytes())
        .map_err(|e| e.to_string())?;
    stream
        .write_all(body.as_bytes())
        .map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())
}

fn param(params: &[(String, String)], key: &str) -> String {
    match params.iter().find(|(k, _)| k == key) {
        Some((_, v)) => v.clone(),
        None => String::new(),
    }
}

fn run_mode(state: &AppState, mode: &str, q: &str) -> Vec<String> {
    match mode {
        "local" => {
            let keywords: Vec<String> = q.split_whitespace().map(|s| s.to_string()).collect();
            if keywords.is_empty() {
                return vec!["pending — the local search carries no query".to_string()];
            }
            let roots: Vec<String> = state
                .roots
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            crate::collect_plain(&roots, &keywords, 2, 40, 100, 0, false, false, None).lines
        }
        "leads" => {
            let keywords: Vec<String> = q.split_whitespace().map(|s| s.to_string()).collect();
            match crate::collect_leads(&state.repo, &[], &keywords, 2, 40, 100) {
                Ok((lines, _)) => lines,
                Err(msg) => vec![msg],
            }
        }
        "mft" => match &state.mft {
            Some(device) => {
                let keywords: Vec<String> = q.split_whitespace().map(|s| s.to_string()).collect();
                crate::ntfs::run_lines(device, &keywords, 40, 100, false)
            }
            None => vec!["pending — the server carries no --mft device".to_string()],
        },
        "index" => {
            let query = Query {
                text: q.to_string(),
                ci: true,
                path: false,
                kind: Kind::Any,
                sort: Sort::Name,
                limit: 100,
            };
            match state.index.read() {
                Ok(index) => index
                    .search(&query)
                    .iter()
                    .map(|h| {
                        let mtext = match h.mtime {
                            Some(m) => m.to_string(),
                            None => "absent".to_string(),
                        };
                        format!("{} ({} bytes, mtime {})", h.path, h.size, mtext)
                    })
                    .collect(),
                Err(_) => vec!["pending — the index lock carries no reading".to_string()],
            }
        }
        "git" => crate::git::run_lines(&state.repo, q),
        other => crate::net::run_lines(other, q, &state.env),
    }
}

fn api_run(state: &AppState, mode: &str, q: &str) -> String {
    let lines = run_mode(state, mode, q);
    let mut out = String::from("[");
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        json_str(&mut out, line);
    }
    out.push(']');
    out
}

fn api_status(state: &AppState) -> String {
    let index = match state.index.read() {
        Ok(index) => index,
        Err(_) => return "{\"entries\":0,\"scanned_at\":0,\"labels\":[]}".to_string(),
    };
    let mut out = String::new();
    out.push('{');
    out.push_str(&format!(
        "\"entries\":{},\"scanned_at\":{},",
        index.entries.len(),
        index.scanned_at
    ));
    out.push_str("\"labels\":[");
    for (i, label) in index.labels.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        json_str(&mut out, label);
    }
    out.push_str("]}");
    out
}

fn api_modes() -> String {
    let mut out = String::from("[");
    for (i, mode) in MODES.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        json_str(&mut out, mode);
    }
    out.push(']');
    out
}

fn json_str(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn parse_params(query: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = match pair.split_once('=') {
            Some((k, v)) => (k, v),
            None => (pair, ""),
        };
        out.push((decode(key), decode(value)));
    }
    out
}

fn decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => match (hexval(bytes[i + 1]), hexval(bytes[i + 2])) {
                (Some(a), Some(b)) => {
                    out.push(a * 16 + b);
                    i += 3;
                }
                _ => {
                    out.push(bytes[i]);
                    i += 1;
                }
            },
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn hexval(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn test_state(roots: Vec<PathBuf>) -> AppState {
        AppState {
            index: RwLock::new(Index {
                entries: Vec::new(),
                labels: Vec::new(),
                scanned_at: 0,
            }),
            roots,
            mft: None,
            repo: std::env::temp_dir(),
            env: HashMap::new(),
        }
    }

    #[test]
    fn decode_percent_and_plus() {
        assert_eq!(decode("a%20b+c"), "a b c");
        assert_eq!(decode("%2Ftmp%2Fx"), "/tmp/x");
    }

    #[test]
    fn parse_params_reads_pairs() {
        let p = parse_params("q=foo&path=1&empty=");
        assert_eq!(p[0], ("q".to_string(), "foo".to_string()));
        assert_eq!(p[1], ("path".to_string(), "1".to_string()));
        assert_eq!(p[2], ("empty".to_string(), String::new()));
    }

    #[test]
    fn json_escapes_quotes_and_controls() {
        let mut out = String::new();
        json_str(&mut out, "a\"b\\c\nd");
        assert_eq!(out, "\"a\\\"b\\\\c\\nd\"");
    }

    #[test]
    fn local_mode_reads_a_temp_file() {
        let dir = std::env::temp_dir().join(format!("archive_serve_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("note.txt");
        std::fs::write(&path, "the needle lives here\n").unwrap();
        let state = test_state(vec![dir.clone()]);
        let lines = run_mode(&state, "local", "needle");
        assert!(lines.iter().any(|l| l.contains("note.txt")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn server_answers_modes_on_an_ephemeral_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let state = Arc::new(test_state(Vec::new()));
        let handle_state = state.clone();
        let worker = std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                let _ = handle(stream, handle_state);
            }
        });
        let mut client = TcpStream::connect(addr).unwrap();
        client
            .write_all(b"GET /api/modes HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();
        let mut body = String::new();
        client.read_to_string(&mut body).unwrap();
        assert!(body.contains("\"local\""));
        assert!(body.contains("\"verdict\""));
        worker.join().unwrap();
    }
}
