use omegaflow::archivar::fits::FitsHeader;
use omegaflow::archivar::inflate::{gunzip, tar_members, zip_members};
use omegaflow::archivar::json::{JsonVal, parse_json};
use omegaflow::archivar::load_env;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::collections::{BTreeMap, HashMap};
use std::process::Command;

const API_ROOT: &str = "https://api.emiratedatacenter.com";
const METADATA_PATH: &str = "/v1/science/files";
const DOWNLOAD_PATH: &str = "/v1/science/files/download";
const NETLOC: &str = "sdc.emiratesmarsmission.ae";
const TOKEN_KEY: &str = "EMM_COGNITO_TOKEN";
const REFRESH_KEY: &str = "EMM_COGNITO_REFRESH";
const REFRESH_KEY_ALT: &str = "EMM_COGNITO_REFRESH_TOKEN";
const CLIENT_ID_KEY: &str = "EMM_COGNITO_CLIENT_ID";
const TOKEN_URL: &str = "https://auth.emiratesmarsmission.ae/oauth2/token";
const DEFAULT_CLIENT_ID: &str = "n5e6d97bl4ba76rrdtm0qaq6n";
const DEFAULT_INSTRUMENT: &str = "exi";
const DEFAULT_LEVEL: &str = "l2a";
const DEFAULT_TIMEOUT_S: &str = "3600";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn cred_opt(env: &HashMap<String, String>, key: &str) -> Option<String> {
    env.get(key).filter(|v| !v.is_empty()).cloned()
}

fn json_str_field(body: &str, field: &str) -> Option<String> {
    match parse_json(body) {
        Some(JsonVal::Obj(map)) => match map.get(field) {
            Some(JsonVal::Str(s)) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn fetch_access_token(refresh_token: &str, client_id: &str) -> Option<String> {
    let work = std::env::temp_dir();
    let rt_tmp = work.join(format!("emm_cognito_rt_{}.tmp", std::process::id()));
    if let Err(e) = std::fs::write(&rt_tmp, refresh_token) {
        eprintln!("emm_sdc_compiler: refresh-token scratch did not write: {e}");
        return None;
    }
    let out_tmp = work.join(format!("emm_cognito_token_{}.json", std::process::id()));
    let rt_spec = format!("refresh_token@{}", rt_tmp.to_string_lossy());
    let out_s = out_tmp.to_string_lossy().to_string();
    let output = Command::new("curl")
        .arg("-sS")
        .arg("--connect-timeout")
        .arg("30")
        .arg("--max-time")
        .arg("60")
        .arg("-X")
        .arg("POST")
        .arg(TOKEN_URL)
        .arg("-H")
        .arg("Content-Type: application/x-www-form-urlencoded")
        .arg("--data-urlencode")
        .arg("grant_type=refresh_token")
        .arg("--data-urlencode")
        .arg(format!("client_id={client_id}"))
        .arg("--data-urlencode")
        .arg(&rt_spec)
        .arg("-o")
        .arg(&out_s)
        .arg("-w")
        .arg("%{http_code}")
        .output();
    let _ = std::fs::remove_file(&rt_tmp);
    let output = match output {
        Ok(o) => o,
        Err(e) => {
            eprintln!("emm_sdc_compiler: Cognito token exchange did not run: {e}");
            let _ = std::fs::remove_file(&out_tmp);
            return None;
        }
    };
    let code: Option<u16> = String::from_utf8_lossy(&output.stdout).trim().parse().ok();
    let body = std::fs::read_to_string(&out_tmp).ok();
    let _ = std::fs::remove_file(&out_tmp);
    let body = match body {
        Some(b) => b,
        None => {
            eprintln!("emm_sdc_compiler: Cognito token exchange carried no response body");
            return None;
        }
    };
    if code != Some(200) {
        let reason =
            match json_str_field(&body, "error").or_else(|| json_str_field(&body, "message")) {
                Some(v) => v,
                None => format!(
                    "unrecognized body: {}",
                    body.chars().take(200).collect::<String>()
                ),
            };
        let code_s = match code.map(|c| c.to_string()) {
            Some(v) => v,
            None => "void".to_string(),
        };
        eprintln!(
            "emm_sdc_compiler: Cognito token exchange HTTP {code_s} — {reason}; the access token is not renewed"
        );
        return None;
    }
    match json_str_field(&body, "access_token").filter(|t| !t.is_empty()) {
        Some(t) => Some(t),
        None => {
            eprintln!(
                "emm_sdc_compiler: Cognito token exchange answered 200 without an access_token field"
            );
            None
        }
    }
}

fn bearer_fetch(
    token: &mut String,
    refresh: &Option<(String, String)>,
    url: &str,
    out: &str,
    follow: bool,
) -> Option<(u16, String, String)> {
    let first = curl_fetch(url, Some(token), out, follow);
    if let Some((code, _, _)) = &first {
        if *code == 401 || *code == 403 {
            if let Some((rt, client_id)) = refresh {
                eprintln!(
                    "emm_sdc_compiler: HTTP {code} — renewing the Cognito access token once and repeating the request"
                );
                if let Some(fresh) = fetch_access_token(rt, client_id) {
                    *token = fresh;
                    return curl_fetch(url, Some(token), out, follow);
                }
            }
        }
    }
    first
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

fn collect_http_urls(v: &JsonVal, out: &mut Vec<String>) {
    match v {
        JsonVal::Str(s) => {
            let normalized = s.replace("\\/", "/");
            let start = normalized
                .find("https://")
                .or_else(|| normalized.find("http://"));
            if let Some(start) = start {
                let rest = &normalized[start..];
                let end = rest
                    .find(|c: char| {
                        c == '"' || c == '\'' || c == '<' || c == '>' || c.is_whitespace()
                    })
                    .unwrap_or(rest.len());
                let url = rest[..end].to_string();
                if !out.contains(&url) {
                    out.push(url);
                }
            }
        }
        JsonVal::Arr(items) => {
            for item in items {
                collect_http_urls(item, out);
            }
        }
        JsonVal::Obj(map) => {
            for item in map.values() {
                collect_http_urls(item, out);
            }
        }
        _ => {}
    }
}

fn collect_file_ids(v: &JsonVal, out: &mut Vec<String>) {
    match v {
        JsonVal::Obj(map) => {
            let id = match map.get("id") {
                Some(JsonVal::Str(id)) => Some(id.clone()),
                Some(JsonVal::Num(n)) if n.fract() == 0.0 => Some(format!("{}", *n as i64)),
                _ => None,
            };
            if let Some(id) = id {
                if !out.contains(&id) {
                    out.push(id);
                }
            }
            for item in map.values() {
                collect_file_ids(item, out);
            }
        }
        JsonVal::Arr(items) => {
            for item in items {
                collect_file_ids(item, out);
            }
        }
        _ => {}
    }
}

fn extension_of(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((_, ext)) if !ext.is_empty() && ext.len() <= 8 => ext.to_ascii_lowercase(),
        _ => "(none)".to_string(),
    }
}

fn report_tar(tar: &[u8], label: &str) {
    let members = match tar_members(tar) {
        Some(m) => m,
        None => {
            eprintln!("emm_sdc_compiler: {label}: tar_members did not read a tar container");
            return;
        }
    };
    println!("emm_sdc_compiler: {label}: {} tar members", members.len());
    let mut hist: BTreeMap<String, usize> = BTreeMap::new();
    for m in &members {
        *hist.entry(extension_of(&m.name)).or_insert(0) += 1;
    }
    let mut ranked: Vec<(String, usize)> = hist.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let line: Vec<String> = ranked.iter().map(|(e, n)| format!("{e}={n}")).collect();
    println!("emm_sdc_compiler: extensions: {}", line.join(" "));
    for m in members.iter().take(60) {
        println!("emm_sdc_compiler: member {} {}", m.end - m.start, m.name);
    }
    if members.len() > 60 {
        println!(
            "emm_sdc_compiler: ... {} further members",
            members.len() - 60
        );
    }
    for m in &members {
        let lower = m.name.to_ascii_lowercase();
        if lower.ends_with(".xml")
            || lower.ends_with(".lbl")
            || lower.ends_with(".tab")
            || lower.ends_with(".csv")
            || lower.ends_with(".txt")
            || lower.ends_with(".label")
        {
            if let Some(slice) = tar.get(m.start..m.end) {
                if let Ok(text) = std::str::from_utf8(slice) {
                    let head: String = text.chars().take(2400).collect();
                    println!(
                        "emm_sdc_compiler: label {} ({} bytes):\n{head}",
                        m.name,
                        m.end - m.start
                    );
                }
            }
            break;
        }
    }
}

fn report_fits(label: &str, gz: &[u8]) {
    let fits = match gunzip(gz) {
        Some(v) => v,
        None => {
            eprintln!("emm_sdc_compiler: gzip member {label} did not decode");
            return;
        }
    };
    println!(
        "emm_sdc_compiler: FITS {label} ({} bytes gunzipped)",
        fits.len()
    );
    let keys = [
        "XTENSION", "SIMPLE", "BITPIX", "NAXIS", "NAXIS1", "NAXIS2", "NAXIS3", "BUNIT", "CTYPE1",
        "CTYPE2", "CTYPE3", "CUNIT1", "CUNIT2", "CUNIT3", "CRVAL1", "CRVAL2", "CRVAL3", "CDELT1",
        "CDELT2", "CDELT3", "BSCALE", "BZERO", "OBJECT", "INSTRUME", "TELESCOP", "DATAMIN",
        "DATAMAX", "EXPTIME", "TIMESYS", "EXTNAME",
    ];
    let mut start = 0usize;
    'hdu: for hdu in 0..4 {
        let (header, aligned) = match FitsHeader::parse(&fits, start) {
            Some(v) => v,
            None => break,
        };
        println!("emm_sdc_compiler: HDU {hdu}:");
        for key in keys {
            if let Some(v) = header.value(key) {
                println!("emm_sdc_compiler:   {key} = {v}");
            }
        }
        let Some(bitpix) = header.int("BITPIX") else {
            break;
        };
        let bitpix = bitpix.unsigned_abs();
        let Some(naxis) = header.int("NAXIS") else {
            break;
        };
        let mut elems: u64 = if naxis > 0 { 1 } else { 0 };
        for axis in 1..=naxis {
            let key = format!("NAXIS{axis}");
            let Some(dim) = header.int(&key) else {
                break 'hdu;
            };
            elems *= dim.unsigned_abs();
        }
        let data_bytes = elems * bitpix / 8;
        let padded = data_bytes.div_ceil(2880) * 2880;
        start = aligned + padded as usize;
        if start >= fits.len() {
            break;
        }
    }
}

fn report_archive(bytes: &[u8], kind: &str) {
    match kind {
        "tar" => report_tar(bytes, "raw tar"),
        "gzip" => match gunzip(bytes) {
            Some(tar) => report_tar(&tar, "gunzipped tar"),
            None => eprintln!("emm_sdc_compiler: gzip decode did not complete"),
        },
        "zip" => {
            let mut names: Vec<(String, usize)> = Vec::new();
            let mut hist: BTreeMap<String, usize> = BTreeMap::new();
            let mut fits_reported = false;
            let count = zip_members(bytes, |name, data| {
                *hist.entry(extension_of(name)).or_insert(0) += 1;
                if names.len() < 60 {
                    names.push((name.to_string(), data.len()));
                }
                if !fits_reported && name.to_ascii_lowercase().ends_with(".fits.gz") {
                    report_fits(name, data);
                    fits_reported = true;
                }
            });
            match count {
                Some(n) => println!("emm_sdc_compiler: zip: {n} members"),
                None => eprintln!("emm_sdc_compiler: zip_members did not read a zip container"),
            }
            for (name, size) in &names {
                println!("emm_sdc_compiler: member {size} {name}");
            }
        }
        _ => {}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let env = load_env();

    let refresh = cred_opt(&env, REFRESH_KEY).or_else(|| cred_opt(&env, REFRESH_KEY_ALT));
    let client_id = match cred_opt(&env, CLIENT_ID_KEY) {
        Some(v) => v,
        None => DEFAULT_CLIENT_ID.to_string(),
    };
    let refresh_spec: Option<(String, String)> =
        refresh.as_ref().map(|rt| (rt.clone(), client_id.clone()));
    let mut token = String::new();
    if let Some(rt) = &refresh {
        match fetch_access_token(rt, &client_id) {
            Some(t) => {
                println!("emm_sdc_compiler: access token renewed from {REFRESH_KEY}");
                token = t;
            }
            None => eprintln!(
                "emm_sdc_compiler: {REFRESH_KEY} did not yield an access token — trying {TOKEN_KEY}"
            ),
        }
    }
    if token.is_empty() {
        match cred_opt(&env, TOKEN_KEY) {
            Some(t) => {
                token = t;
            }
            None => {
                eprintln!(
                    "emm_sdc_compiler: neither {REFRESH_KEY} nor {TOKEN_KEY} stands in the environment and .secrets.local — the Cognito hand is the operator's (Future-Queue); the query stays void"
                );
                std::process::exit(2);
            }
        }
    }

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
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/emm_{instrument}_{level}.tar"),
    };
    let ci = args.iter().any(|a| a == "--ci-mode");
    let metadata_only = args.iter().any(|a| a == "--metadata-only");
    let mut explicit_ids: Vec<String> = Vec::new();
    let mut ai = 0;
    while ai < args.len() {
        if args[ai] == "--id" {
            if let Some(id) = args.get(ai + 1) {
                explicit_ids.push(id.clone());
            }
        }
        ai += 1;
    }
    let limit: usize = arg_value(&args, "--limit")
        .and_then(|v| v.parse().ok())
        .filter(|n| *n > 0)
        .unwrap_or(2);

    let work = std::env::temp_dir();
    let meta_tmp = work.join(format!("emm_sdc_meta_{}.json", std::process::id()));
    let meta_s = meta_tmp.to_string_lossy().to_string();

    let date_ranges: Vec<(String, String)> = match (
        arg_value(&args, "--start-date"),
        arg_value(&args, "--end-date"),
    ) {
        (Some(s), Some(e)) => vec![(s, e)],
        _ => [
            ("2023-01-01", "2023-04-01"),
            ("2022-01-01", "2022-04-01"),
            ("2024-01-01", "2024-04-01"),
            ("2021-01-01", "2021-04-01"),
            ("2025-01-01", "2025-04-01"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect(),
    };

    let mut query = String::new();
    let mut meta_body = String::new();
    let mut parsed: Option<JsonVal> = None;
    let mut records = 0usize;
    for (start, end) in &date_ranges {
        let mut pairs = query_pairs(&args, &instrument, &level);
        pairs.push(("start_date".to_string(), start.clone()));
        pairs.push(("end_date".to_string(), end.clone()));
        let candidate = build_query(&pairs);
        let meta_url = format!("{api_root}{METADATA_PATH}?{candidate}");
        let (code, content_type, _) = match bearer_fetch(
            &mut token,
            &refresh_spec,
            &meta_url,
            &meta_s,
            false,
        ) {
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
            eprintln!(
                "emm_sdc_compiler: metadata HTTP {code} ({content_type}) for {candidate} — the query stays void"
            );
            let _ = std::fs::remove_file(&meta_tmp);
            std::process::exit(2);
        }
        let body = match std::fs::read_to_string(&meta_tmp) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("emm_sdc_compiler: read metadata: {e}");
                let _ = std::fs::remove_file(&meta_tmp);
                std::process::exit(1);
            }
        };
        let candidate_records = match parse_json(&body) {
            Some(JsonVal::Arr(arr)) => Some(arr.len()),
            Some(JsonVal::Obj(map)) => Some(match map.get("files") {
                Some(JsonVal::Arr(arr)) => arr.len(),
                _ => 1,
            }),
            _ => None,
        };
        let Some(candidate_records) = candidate_records else {
            continue;
        };
        if candidate_records == 0 {
            println!("emm_sdc_compiler: 0 files for {candidate} (0 honored)");
            continue;
        }
        println!("emm_sdc_compiler: metadata {candidate}");
        query = candidate;
        meta_body = body;
        records = candidate_records;
        parsed = parse_json(&meta_body);
        break;
    }
    let _ = std::fs::remove_file(&meta_tmp);

    let parsed = match parsed {
        Some(v) => v,
        None => {
            println!(
                "emm_sdc_compiler: no candidate date range carried files — pass --start-date/--end-date; the asset stays pending"
            );
            return;
        }
    };

    let meta_head: String = meta_body.chars().take(2000).collect();
    println!("emm_sdc_compiler: metadata head:\n{meta_head}");

    let mut asset_urls: Vec<String> = Vec::new();
    let mut file_ids: Vec<String> = Vec::new();
    collect_http_urls(&parsed, &mut asset_urls);
    collect_file_ids(&parsed, &mut file_ids);
    asset_urls.sort();
    file_ids.sort();
    println!(
        "emm_sdc_compiler: {records} files, {} ids, {} asset url(s)",
        file_ids.len(),
        asset_urls.len()
    );
    if let Some(first) = asset_urls.first() {
        println!("emm_sdc_compiler: first asset url {first}");
    }

    if metadata_only {
        println!("emm_sdc_compiler: metadata-only — the download is not attempted");
        return;
    }
    if records == 0 {
        println!("emm_sdc_compiler: 0 files for {query} (0 honored)");
        return;
    }

    let dl_tmp = work.join(format!("emm_sdc_dl_{}.tar", std::process::id()));
    let dl_s = dl_tmp.to_string_lossy().to_string();
    let (final_type, final_effective, mut bytes);
    if !explicit_ids.is_empty() {
        let dl_query = explicit_ids
            .iter()
            .map(|id| format!("id={id}"))
            .collect::<Vec<_>>()
            .join("&");
        println!(
            "emm_sdc_compiler: downloading the {} explicitly named id(s)",
            explicit_ids.len()
        );
        let dl_url = format!("{api_root}{DOWNLOAD_PATH}?{dl_query}");
        let (code, content_type, effective) =
            match bearer_fetch(&mut token, &refresh_spec, &dl_url, &dl_s, true) {
                Some(v) => v,
                None => {
                    eprintln!(
                        "emm_sdc_compiler: the explicit download did not complete — the asset stays unwritten"
                    );
                    std::process::exit(1);
                }
            };
        if code != 200 {
            eprintln!(
                "emm_sdc_compiler: the explicit download returned HTTP {code} ({content_type}) for {dl_query} — the asset stays unwritten"
            );
            let _ = std::fs::remove_file(&dl_tmp);
            std::process::exit(1);
        }
        final_type = content_type;
        final_effective = effective;
        bytes = match std::fs::read(&dl_tmp) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("emm_sdc_compiler: read the explicit download: {e}");
                let _ = std::fs::remove_file(&dl_tmp);
                std::process::exit(1);
            }
        };
    } else if let Some(asset_url) = asset_urls.first() {
        println!("emm_sdc_compiler: fetching the deterministic first asset url");
        let (acode, atype, aeffective) =
            match bearer_fetch(&mut token, &refresh_spec, asset_url, &dl_s, true) {
                Some(v) => v,
                None => {
                    eprintln!(
                        "emm_sdc_compiler: the asset url did not complete — the asset stays unwritten"
                    );
                    std::process::exit(1);
                }
            };
        if acode != 200 {
            eprintln!(
                "emm_sdc_compiler: the asset url returned HTTP {acode} ({atype}) — the asset stays unwritten"
            );
            let _ = std::fs::remove_file(&dl_tmp);
            std::process::exit(1);
        }
        final_type = atype;
        final_effective = aeffective;
        bytes = match std::fs::read(&dl_tmp) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("emm_sdc_compiler: read the asset url: {e}");
                let _ = std::fs::remove_file(&dl_tmp);
                std::process::exit(1);
            }
        };
    } else if !file_ids.is_empty() {
        let selected: Vec<&String> = file_ids.iter().take(limit).collect();
        let dl_query = selected
            .iter()
            .map(|id| format!("id={id}"))
            .collect::<Vec<_>>()
            .join("&");
        println!(
            "emm_sdc_compiler: {records} files — downloading the first {} of {} sorted id(s) (--limit {limit})",
            selected.len(),
            file_ids.len()
        );
        let dl_url = format!("{api_root}{DOWNLOAD_PATH}?{dl_query}");
        let (code, content_type, effective) =
            match bearer_fetch(&mut token, &refresh_spec, &dl_url, &dl_s, true) {
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
                "emm_sdc_compiler: download HTTP {code} ({content_type}) for {dl_query} — the asset stays unwritten"
            );
            let _ = std::fs::remove_file(&dl_tmp);
            std::process::exit(1);
        }
        final_type = content_type;
        final_effective = effective;
        bytes = match std::fs::read(&dl_tmp) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("emm_sdc_compiler: read download: {e}");
                let _ = std::fs::remove_file(&dl_tmp);
                std::process::exit(1);
            }
        };
    } else {
        println!(
            "emm_sdc_compiler: {records} files and no per-file id or url in the metadata — pass --id or narrow the query; the aggregate download is refused (the asset stays pending)"
        );
        return;
    }
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
                "emm_sdc_compiler: download carried no archive and no link ({final_type}, {} bytes, final {final_effective}) — the asset stays unwritten",
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
    report_archive(&bytes, kind);

    if ci && !upload_release(NETLOC, &out) {
        eprintln!("emm_sdc_compiler: CDN upload returned void for {out}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_pairs_defaults_to_exi_l2a() {
        let args: Vec<String> = Vec::new();
        let pairs = query_pairs(&args, DEFAULT_INSTRUMENT, DEFAULT_LEVEL);
        assert_eq!(pairs[0], ("instrument_id".to_string(), "exi".to_string()));
        assert_eq!(pairs[1], ("data_level".to_string(), "l2a".to_string()));
        assert_eq!(build_query(&pairs), "instrument_id=exi&data_level=l2a");
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
    fn json_str_field_reads_token_and_error_fields() {
        let body = r#"{"access_token":"x","error":"invalid_grant"}"#;
        assert_eq!(json_str_field(body, "access_token"), Some("x".to_string()));
        assert_eq!(
            json_str_field(body, "error"),
            Some("invalid_grant".to_string())
        );
        assert_eq!(json_str_field(body, "absent"), None);
        assert_eq!(json_str_field("not json", "access_token"), None);
    }

    #[test]
    fn cred_opt_skips_empty_values() {
        let mut env = HashMap::new();
        env.insert("A".to_string(), "v".to_string());
        env.insert("B".to_string(), String::new());
        assert_eq!(cred_opt(&env, "A"), Some("v".to_string()));
        assert_eq!(cred_opt(&env, "B"), None);
        assert_eq!(cred_opt(&env, "C"), None);
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

    #[test]
    fn collect_http_urls_reads_every_string_field() {
        let body =
            r#"{"files":[{"url":"https:\/\/a.example\/1.tar"},{"url":"https://b.example/2.tar"}]}"#;
        let parsed = parse_json(body).expect("the fixture is JSON");
        let mut urls = Vec::new();
        collect_http_urls(&parsed, &mut urls);
        assert_eq!(
            urls,
            vec![
                "https://a.example/1.tar".to_string(),
                "https://b.example/2.tar".to_string()
            ]
        );
    }

    #[test]
    fn extension_of_reads_the_suffix() {
        assert_eq!(extension_of("a/b/c.XML"), "xml");
        assert_eq!(extension_of("granule_1.tar.gz"), "gz");
        assert_eq!(extension_of("no_extension"), "(none)");
    }
}
