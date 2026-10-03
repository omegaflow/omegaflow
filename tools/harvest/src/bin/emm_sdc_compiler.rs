use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::load_env;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::collections::HashMap;
use std::process::Command;

const API_ROOT: &str = "https://mdhkq4bfae.execute-api.eu-west-1.amazonaws.com/prod";
const METADATA_PATH: &str = "/science-files-metadata";
const DOWNLOAD_PATH: &str = "/science-files-download";
const NETLOC: &str = "sdc.emiratesmarsmission.ae";
const TOKEN_KEY: &str = "EMM_COGNITO_TOKEN";
const DEFAULT_INSTRUMENT: &str = "exi";
const DEFAULT_LEVEL: &str = "l2";
const DEFAULT_TIMEOUT_S: &str = "3600";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn credential(env: &HashMap<String, String>, key: &str) -> String {
    match env.get(key).filter(|v| !v.is_empty()) {
        Some(v) => v.clone(),
        None => {
            eprintln!(
                "emm_sdc_compiler: {key} absent in the environment and .secrets.local — the Cognito access token is the operator's hand (Future-Queue); the query stays void"
            );
            std::process::exit(2);
        }
    }
}

fn query_pairs(args: &[String], instrument: &str, level: &str) -> Vec<(String, String)> {
    let mut pairs = vec![
        ("instrument_id".to_string(), instrument.to_string()),
        ("data_level".to_string(), level.to_string()),
    ];
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--query" {
            if let Some(kv) = args.get(i + 1) {
                if let Some((k, v)) = kv.split_once('=') {
                    pairs.push((k.to_string(), v.to_string()));
                }
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    pairs
}

fn build_query(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&")
}

fn curl_fetch(
    url: &str,
    token: Option<&str>,
    out: &str,
    follow: bool,
) -> Option<(u16, String, String)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("--connect-timeout")
        .arg("30")
        .arg("--max-time")
        .arg(DEFAULT_TIMEOUT_S)
        .arg("-H")
        .arg("Accept: application/json, text/plain, */*")
        .arg("-o")
        .arg(out)
        .arg("-w")
        .arg("%{http_code} %{content_type} %{url_effective}");
    if let Some(t) = token {
        cmd.arg("-H").arg(format!("Authorization: {t}"));
    }
    if follow {
        cmd.arg("-L");
    }
    cmd.arg(url);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("emm_sdc_compiler: curl did not run: {e}");
            return None;
        }
    };
    let info = String::from_utf8_lossy(&output.stdout);
    let mut parts = info.split_whitespace();
    let code: u16 = parts.next()?.parse().ok()?;
    let content_type = parts.next().unwrap_or("").to_string();
    let effective = parts.next().unwrap_or("").to_string();
    Some((code, content_type, effective))
}

fn archive_kind(bytes: &[u8]) -> Option<&'static str> {
    if bytes.len() > 262 && &bytes[257..262] == b"ustar" {
        return Some("tar");
    }
    if bytes.starts_with(&[0x1f, 0x8b]) {
        return Some("gzip");
    }
    if bytes.starts_with(b"PK\x03\x04") {
        return Some("zip");
    }
    None
}

fn first_http_url(text: &str) -> Option<String> {
    let normalized = text.replace("\\/", "/");
    let start = normalized
        .find("https://")
        .or_else(|| normalized.find("http://"))?;
    let rest = &normalized[start..];
    let end = rest
        .find(|c: char| c == '"' || c == '\'' || c == '<' || c == '>' || c.is_whitespace())
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let env = load_env();
    let token = credential(&env, TOKEN_KEY);

    let api_root = match arg_value(&args, "--api-root") {
        Some(v) => v,
        None => API_ROOT.to_string(),
    };
    let instrument = match arg_value(&args, "--instrument") {
        Some(v) => v,
        None => DEFAULT_INSTRUMENT.to_string(),
    };
    let level = match arg_value(&args, "--data-level") {
        Some(v) => v,
        None => DEFAULT_LEVEL.to_string(),
    };
    let pairs = query_pairs(&args, &instrument, &level);
    let query = build_query(&pairs);
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/emm_{instrument}_{level}.tar"),
    };
    let ci = args.iter().any(|a| a == "--ci-mode");

    let work = std::env::temp_dir();
    let meta_tmp = work.join(format!("emm_sdc_meta_{}.json", std::process::id()));
    let meta_s = meta_tmp.to_string_lossy().to_string();
    let meta_url = format!("{api_root}{METADATA_PATH}?{query}");

    let (code, content_type, _) = match curl_fetch(&meta_url, Some(&token), &meta_s, false) {
        Some(v) => v,
        None => {
            eprintln!(
                "emm_sdc_compiler: metadata query did not complete — the asset stays unwritten"
            );
            std::process::exit(1);
        }
    };
    if code == 401 || code == 403 {
        eprintln!(
            "emm_sdc_compiler: metadata HTTP {code} — the Cognito access token did not open the query"
        );
        let _ = std::fs::remove_file(&meta_tmp);
        std::process::exit(2);
    }
    if code != 200 {
        eprintln!("emm_sdc_compiler: metadata HTTP {code} ({content_type}) — the query stays void");
        let _ = std::fs::remove_file(&meta_tmp);
        std::process::exit(2);
    }
    let meta_body = match std::fs::read_to_string(&meta_tmp) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("emm_sdc_compiler: read metadata: {e}");
            let _ = std::fs::remove_file(&meta_tmp);
            std::process::exit(1);
        }
    };
    let _ = std::fs::remove_file(&meta_tmp);

    let records = match parse_json(&meta_body) {
        Some(JsonVal::Arr(arr)) => arr.len(),
        Some(JsonVal::Obj(map)) => match map.get("files") {
            Some(JsonVal::Arr(arr)) => arr.len(),
            _ => 1,
        },
        Some(_) => 0,
        None => {
            eprintln!(
                "emm_sdc_compiler: metadata carries no JSON — the contract changed; the asset stays unwritten"
            );
            std::process::exit(1);
        }
    };
    if records == 0 {
        println!("emm_sdc_compiler: 0 files for {query} (0 honored)");
        return;
    }
    println!("emm_sdc_compiler: {records} files for {query} — downloading");

    let dl_tmp = work.join(format!("emm_sdc_dl_{}.tar", std::process::id()));
    let dl_s = dl_tmp.to_string_lossy().to_string();
    let dl_url = format!("{api_root}{DOWNLOAD_PATH}?{query}");
    let (code, content_type, effective) = match curl_fetch(&dl_url, Some(&token), &dl_s, true) {
        Some(v) => v,
        None => {
            eprintln!(
                "emm_sdc_compiler: download query did not complete — the asset stays unwritten"
            );
            std::process::exit(1);
        }
    };
    if code != 200 {
        eprintln!(
            "emm_sdc_compiler: download HTTP {code} ({content_type}) for {query} — the asset stays unwritten"
        );
        let _ = std::fs::remove_file(&dl_tmp);
        std::process::exit(1);
    }
    let mut bytes = match std::fs::read(&dl_tmp) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("emm_sdc_compiler: read download: {e}");
            let _ = std::fs::remove_file(&dl_tmp);
            std::process::exit(1);
        }
    };
    let _ = std::fs::remove_file(&dl_tmp);

    if archive_kind(&bytes).is_none() {
        let text = String::from_utf8_lossy(&bytes).to_string();
        if let Some(link) = first_http_url(&text) {
            println!("emm_sdc_compiler: download returned a link — fetching the archive");
            let link_tmp = work.join(format!("emm_sdc_link_{}.tar", std::process::id()));
            let link_s = link_tmp.to_string_lossy().to_string();
            let (lcode, ltype, _) = match curl_fetch(&link, None, &link_s, true) {
                Some(v) => v,
                None => {
                    eprintln!(
                        "emm_sdc_compiler: the archive link did not complete — the asset stays unwritten"
                    );
                    std::process::exit(1);
                }
            };
            if lcode != 200 {
                eprintln!(
                    "emm_sdc_compiler: the archive link returned HTTP {lcode} ({ltype}) — the asset stays unwritten"
                );
                let _ = std::fs::remove_file(&link_tmp);
                std::process::exit(1);
            }
            bytes = match std::fs::read(&link_tmp) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("emm_sdc_compiler: read the archive link: {e}");
                    let _ = std::fs::remove_file(&link_tmp);
                    std::process::exit(1);
                }
            };
            let _ = std::fs::remove_file(&link_tmp);
        } else if text.to_ascii_lowercase().contains("email") {
            eprintln!(
                "emm_sdc_compiler: the SDC queued the archive for email — the link arrives by mail; the asset stays pending"
            );
            std::process::exit(1);
        } else {
            eprintln!(
                "emm_sdc_compiler: download carried no archive and no link ({content_type}, {} bytes, final {effective}) — the asset stays unwritten",
                bytes.len()
            );
            std::process::exit(1);
        }
    }

    let kind = match archive_kind(&bytes) {
        Some(k) => k,
        None => {
            eprintln!(
                "emm_sdc_compiler: the fetched bytes carry no archive magic — the asset stays unwritten"
            );
            std::process::exit(1);
        }
    };
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&out, &bytes) {
        eprintln!("emm_sdc_compiler: write {out}: {e}");
        std::process::exit(1);
    }
    let sha = sha256_hex(&bytes);
    println!(
        "emm_sdc_compiler: {out}: {} bytes ({kind}), sha256 {sha}",
        bytes.len()
    );

    if ci && !upload_release(NETLOC, &out) {
        eprintln!("emm_sdc_compiler: CDN upload returned void for {out}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_pairs_defaults_to_exi_l2() {
        let args: Vec<String> = Vec::new();
        let pairs = query_pairs(&args, DEFAULT_INSTRUMENT, DEFAULT_LEVEL);
        assert_eq!(pairs[0], ("instrument_id".to_string(), "exi".to_string()));
        assert_eq!(pairs[1], ("data_level".to_string(), "l2".to_string()));
        assert_eq!(build_query(&pairs), "instrument_id=exi&data_level=l2");
    }

    #[test]
    fn query_pairs_appends_extra_pairs() {
        let args: Vec<String> = vec!["--query".to_string(), "start_sc_lst=00:00:00".to_string()];
        let pairs = query_pairs(&args, "exi", "l2");
        assert_eq!(pairs.len(), 3);
        assert_eq!(
            pairs[2],
            ("start_sc_lst".to_string(), "00:00:00".to_string())
        );
    }

    #[test]
    fn archive_kind_reads_the_magic() {
        let mut tar = vec![0u8; 512];
        tar[257..262].copy_from_slice(b"ustar");
        assert_eq!(archive_kind(&tar), Some("tar"));
        assert_eq!(archive_kind(&[0x1f, 0x8b, 0x08]), Some("gzip"));
        assert_eq!(archive_kind(b"PK\x03\x04rest"), Some("zip"));
        assert_eq!(archive_kind(b"not an archive"), None);
        assert_eq!(archive_kind(&[]), None);
    }

    #[test]
    fn first_http_url_reads_json_and_plain_text() {
        assert_eq!(
            first_http_url(r#"{"url":"https://example.org/a.tar"}"#),
            Some("https://example.org/a.tar".to_string())
        );
        assert_eq!(
            first_http_url("https://example.org/b.tar\n"),
            Some("https://example.org/b.tar".to_string())
        );
        assert_eq!(
            first_http_url(r#"{"url":"https:\/\/example.org\/c.tar"}"#),
            Some("https://example.org/c.tar".to_string())
        );
        assert_eq!(first_http_url("no link here"), None);
    }
}
