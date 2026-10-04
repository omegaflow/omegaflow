use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use omegaflow::archivar::sha256::sha256_raw;
use omegaflow::openneuro_eeg::{OpenNeuroEeg, Samples, write_bin};

const BASE: &str = "https://www.ieeg.org/services";
const NETLOC: &str = "www.ieeg.org";
const GAP: i64 = i32::MIN as i64;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secrets_path() -> Option<PathBuf> {
    if let Ok(p) = env::var("OMEGAFLOW_SECRETS_FILE") {
        let p = PathBuf::from(p);
        if p.is_file() {
            return Some(p);
        }
    }
    let cwd = env::current_dir().ok()?;
    cwd.ancestors()
        .map(|d| d.join(".secrets.local"))
        .find(|f| f.is_file())
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = env::var(name) {
        let v = v.trim();
        if !v.is_empty() {
            return Some(v.to_string());
        }
    }
    let text = fs::read_to_string(secrets_path()?).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name {
                let v = v.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
    }
    None
}

fn md5_hex(input: &str) -> String {
    let mut data = input.as_bytes().to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_le_bytes());
    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let k: Vec<u32> = (1..=64)
        .map(|i| ((i as f64).sin().abs() * 4_294_967_296.0) as u32)
        .collect();
    let (mut a0, mut b0, mut c0, mut d0) = (
        0x6745_2301u32,
        0xefcd_ab89u32,
        0x98ba_dcfeu32,
        0x1032_5476u32,
    );
    for chunk in data.chunks(64) {
        let mut m = [0u32; 16];
        for (i, slot) in m.iter_mut().enumerate() {
            *slot = u32::from_le_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f2 = f.wrapping_add(a).wrapping_add(k[i]).wrapping_add(m[g]);
            let tmp = d;
            d = c;
            c = b;
            b = b.wrapping_add(f2.rotate_left(s[i]));
            a = tmp;
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut out = String::with_capacity(32);
    for v in [a0, b0, c0, d0] {
        out.push_str(&format!("{:08x}", v.swap_bytes()));
    }
    out
}

fn b64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            T[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            T[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn iso_utc_now() -> Option<String> {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    let secs = d.as_secs() as i64;
    let micros = d.subsec_micros();
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (y, m, dd) = civil_from_days(days);
    Some(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}+00:00",
        y,
        m,
        dd,
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60,
        micros
    ))
}

fn signature(
    user: &str,
    pass_md5: &str,
    method: &str,
    host: &str,
    path: &str,
    query: &str,
    ts: &str,
    body: &[u8],
) -> String {
    let payload_hash = b64_encode(&sha256_raw(body));
    let to_hash =
        format!("{user}\n{pass_md5}\n{method}\n{host}\n{path}\n{query}\n{ts}\n{payload_hash}");
    b64_encode(&sha256_raw(to_hash.as_bytes()))
}

struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

fn header<'a>(r: &'a Response, name: &str) -> Option<&'a str> {
    r.headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

fn request(
    method: &str,
    url: &str,
    body: &[u8],
    accept: &str,
    content_type: Option<&str>,
) -> Option<Response> {
    let user = secret("IEEG_USER")?;
    let pass = secret("IEEG_PASS")?;
    let pass_md5 = md5_hex(&pass);
    let rest = url.strip_prefix("https://")?;
    let (host, pathq) = rest.split_once('/')?;
    let (path, query) = match pathq.split_once('?') {
        Some((p, q)) => (format!("/{p}"), q.to_string()),
        None => (format!("/{pathq}"), String::new()),
    };
    let ts = iso_utc_now()?;
    let sig = signature(&user, &pass_md5, method, host, &path, &query, &ts, body);
    let hdr_path = "/tmp/opencode/ieeg_hdr.txt";
    let body_path = "/tmp/opencode/ieeg_body.bin";
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("--max-time")
        .arg("600")
        .arg("-D")
        .arg(hdr_path)
        .arg("-o")
        .arg(body_path)
        .arg("-H")
        .arg(format!("username: {user}"))
        .arg("-H")
        .arg(format!("timestamp: {ts}"))
        .arg("-H")
        .arg(format!("signature: {sig}"))
        .arg("-H")
        .arg(format!("Accept: {accept}"));
    if method == "POST" {
        cmd.arg("-X").arg("POST");
        if let Some(ct) = content_type {
            cmd.arg("-H").arg(format!("Content-Type: {ct}"));
        }
        cmd.arg("--data-binary").arg("@-");
        cmd.stdin(Stdio::piped());
    }
    cmd.arg(url);
    let mut child = cmd.spawn().ok()?;
    if method == "POST" {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(body);
        }
    }
    let out = child.wait_with_output().ok()?;
    if !out.status.success() {
        eprintln!(
            "ieeg request {method} {path}: curl void — {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    let hdr_text = fs::read_to_string(hdr_path).ok()?;
    let body_bytes = fs::read(body_path).ok()?;
    let mut status = 0u16;
    let mut headers = Vec::new();
    for (i, line) in hdr_text.lines().enumerate() {
        if i == 0 {
            let Some(code) = line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.parse::<u16>().ok())
            else {
                return None;
            };
            status = code;
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    Some(Response {
        status,
        headers,
        body: body_bytes,
    })
}

fn tag_values(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(s) = rest.find(&open) {
        let after = &rest[s + open.len()..];
        let Some(e) = after.find(&close) else {
            break;
        };
        out.push(after[..e].trim().to_string());
        rest = &after[e + close.len()..];
    }
    out
}

struct Detail {
    label: String,
    revision_id: String,
    data_check: String,
}

fn detail_blocks(xml: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(s) = rest.find("<detail>") {
        let after = &rest[s + "<detail>".len()..];
        let Some(e) = after.find("</detail>") else {
            break;
        };
        out.push(after[..e].to_string());
        rest = &after[e + "</detail>".len()..];
    }
    out
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let dataset = arg_value(&args, "--dataset")
        .ok_or("ieeg_compiler --dataset <name> [--out <path>] [--ci-mode]")?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/{dataset}.bin"),
    };

    let id_url = format!("{BASE}/timeseries/getIdByDataSnapshotName/{dataset}");
    let id_resp = request("GET", &id_url, &[], "application/json", None)
        .ok_or("ieeg: getIdByDataSnapshotName returned void")?;
    if id_resp.status != 200 {
        return Err(format!("ieeg: getId http {}", id_resp.status));
    }
    let snapshot_id = String::from_utf8_lossy(&id_resp.body).trim().to_string();
    if snapshot_id.is_empty() {
        return Err("ieeg: empty snapshot id".into());
    }
    eprintln!("ieeg {dataset}: snapshot {snapshot_id}");

    let det_url = format!("{BASE}/timeseries/getDataSnapshotTimeSeriesDetails/{snapshot_id}");
    let det_resp = request("GET", &det_url, &[], "application/xml", None)
        .ok_or("ieeg: getDataSnapshotTimeSeriesDetails returned void")?;
    let det_xml = String::from_utf8_lossy(&det_resp.body).to_string();
    let mut details = Vec::new();
    for block in detail_blocks(&det_xml) {
        let label = tag_values(&block, "channelLabel").into_iter().next();
        let revision_id = tag_values(&block, "revisionId").into_iter().next();
        let data_check = tag_values(&block, "dataCheck").into_iter().next();
        if let (Some(l), Some(r), Some(dc)) = (label, revision_id, data_check) {
            details.push(Detail {
                label: l,
                revision_id: r,
                data_check: dc,
            });
        }
    }
    if details.is_empty() {
        return Err("ieeg: the details XML carries no channel detail".into());
    }
    eprintln!("ieeg {dataset}: {} channel(s)", details.len());

    let duration = match arg_value(&args, "--duration") {
        Some(d) => d,
        None => "10000000".to_string(),
    };
    let start = match arg_value(&args, "--start") {
        Some(s) => s,
        None => "0".to_string(),
    };
    let mut data_req = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><timeSeriesIdAndDChecks><timeSeriesIdAndDChecks>",
    );
    for d in &details {
        data_req.push_str(&format!(
            "<timeSeriesIdAndCheck><dataCheck>{}</dataCheck><id>{}</id></timeSeriesIdAndCheck>",
            d.data_check, d.revision_id
        ));
    }
    data_req.push_str("</timeSeriesIdAndDChecks></timeSeriesIdAndDChecks>");

    let data_url = format!(
        "{BASE}/timeseries/getUnscaledTimeSeriesSetBinaryRaw/{snapshot_id}?start={start}&duration={duration}"
    );
    let data_resp = request(
        "POST",
        &data_url,
        data_req.as_bytes(),
        "application/xml",
        Some("application/xml"),
    )
    .ok_or("ieeg: getUnscaledTimeSeriesSetBinaryRaw returned void")?;
    if data_resp.status != 200 {
        return Err(format!("ieeg: getData http {}", data_resp.status));
    }
    let samples_per_row: u64 = header(&data_resp, "samples-per-row")
        .and_then(|v| v.split(',').next())
        .and_then(|s| s.trim().parse().ok())
        .ok_or("ieeg: response carries no samples-per-row")?;
    let factors: Vec<f64> = header(&data_resp, "voltage-conversion-factors-mv")
        .ok_or("ieeg: response carries no voltage-conversion-factors-mv")?
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();
    if factors.len() != details.len() {
        return Err("ieeg: conversion factors do not match the channel count".into());
    }
    let expect = (samples_per_row as usize)
        .checked_mul(details.len())
        .ok_or("ieeg: sample count overflow")?;
    if data_resp.body.len() != expect * 4 {
        return Err(format!(
            "ieeg: binary carries {} bytes, expected {}",
            data_resp.body.len(),
            expect * 4
        ));
    }
    let mut samples: Vec<f64> = Vec::with_capacity(expect);
    for (c, factor) in factors.iter().enumerate() {
        for s in 0..samples_per_row as usize {
            let off = (c * samples_per_row as usize + s) * 4;
            let raw = i32::from_be_bytes([
                data_resp.body[off],
                data_resp.body[off + 1],
                data_resp.body[off + 2],
                data_resp.body[off + 3],
            ]) as i64;
            if raw == GAP {
                return Err("ieeg: the response carries the server gap value".into());
            }
            let value = raw as f64 * factor;
            if !value.is_finite() {
                return Err("ieeg: a converted sample reads non-finite".into());
            }
            samples.push(value);
        }
    }
    let labels: Vec<String> = details.iter().map(|d| d.label.clone()).collect();
    let eeg = OpenNeuroEeg {
        nbchan: details.len() as u32,
        pnts: samples_per_row,
        trials: 1,
        srate: None,
        sha256: sha256_raw(&data_resp.body),
        snapshot_tag: snapshot_id.clone(),
        hexsha: snapshot_id.clone(),
        origin_url: format!("https://www.ieeg.org/"),
        labels,
        events: None,
        samples: Samples::Double(samples),
    };
    let bin = write_bin(&eeg);
    if let Some(parent) = Path::new(&out).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {} void: {e}", parent.display()))?;
    }
    fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;
    eprintln!(
        "ieeg {dataset}: {} channel(s) x {} samples -> {out} ({} bytes){}",
        details.len(),
        samples_per_row,
        bin.len(),
        if ci_mode {
            ", manifest via workflow"
        } else {
            ""
        }
    );
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ieeg_compiler: {e}");
        std::process::exit(1);
    }
}
