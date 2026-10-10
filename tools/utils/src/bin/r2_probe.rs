use std::io::Write;
use std::process::{Command, ExitCode, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use omegaflow::archivar::range::{Sigv4Args, Sigv4PutArgs, sigv4_headers, sigv4_put_headers};
use omegaflow::archivar::{date_str, hour_str, load_env, sha256};

const R2_REGION: &str = "auto";
const BODY: &[u8] = b"omegaflow-r2-probe\n";
const OBJECT_KEY: &str = "ci-probe.txt";

fn append_ca(cmd: &mut Command) {
    let path = match std::env::var("OMEGAFLOW_CA_BUNDLE") {
        Ok(p) if !p.is_empty() && std::path::Path::new(&p).is_file() => p,
        _ => return,
    };
    cmd.arg("--cacert").arg(path);
}

fn endpoint_host(endpoint: &str) -> Option<String> {
    let rest = endpoint
        .strip_prefix("https://")
        .or_else(|| endpoint.strip_prefix("http://"))?;
    let host = rest.split('/').next().unwrap_or(rest);
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

fn send(
    method: &str,
    url: &str,
    headers: &[(String, String)],
    body: Option<&[u8]>,
) -> std::io::Result<std::process::Output> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS").arg("-f").arg("-X").arg(method).arg(url);
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    if body.is_some() {
        cmd.arg("--data-binary").arg("@-");
    }
    append_ca(&mut cmd);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn()?;
    if let Some(bytes) = body {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(bytes)?;
        }
    }
    child.wait_with_output()
}

fn named_reason(label: &str, output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!("r2_probe: {} failed — {}", label, stderr.trim())
}

fn run() -> Result<(), String> {
    let env = load_env();
    for key in [
        "R2_ACCESS_KEY_ID",
        "R2_SECRET_ACCESS_KEY",
        "R2_ENDPOINT",
        "R2_BUCKET",
    ] {
        if env.get(key).map(|v| v.is_empty()).unwrap_or(true) {
            return Err(format!("r2_probe: {} absent — pending", key));
        }
    }
    let access_key = env.get("R2_ACCESS_KEY_ID").unwrap();
    let secret_key = env.get("R2_SECRET_ACCESS_KEY").unwrap();
    let endpoint = env.get("R2_ENDPOINT").unwrap().trim_end_matches('/');
    let bucket = env.get("R2_BUCKET").unwrap();

    let host = endpoint_host(endpoint).ok_or_else(|| {
        format!("r2_probe: R2_ENDPOINT carries no host — {}", endpoint)
    })?;
    let canonical_uri = format!("/{}/{}", bucket, OBJECT_KEY);
    let url = format!("{}{}", endpoint, canonical_uri);
    let payload_sha256 = sha256::sha256_hex(BODY);

    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "r2_probe: system clock before UNIX_EPOCH — pending".to_string())?
        .as_secs();
    let date_stamp = date_str(unix).replace('-', "");
    let amz_date = hour_str(unix).replace(['-', ':'], "");

    let headers = sigv4_put_headers(&Sigv4PutArgs {
        method: "PUT",
        access_key,
        secret_key,
        region: R2_REGION,
        host: &host,
        canonical_uri: &canonical_uri,
        payload_sha256: &payload_sha256,
        content_length: Some(BODY.len() as u64),
        amz_date: &amz_date,
        date_stamp: &date_stamp,
    });

    let put = send("PUT", &url, &headers, Some(BODY))
        .map_err(|e| format!("r2_probe: put spawn failed — {}", e))?;
    if !put.status.success() {
        return Err(named_reason("put", &put));
    }

    let get_headers = sigv4_headers(&Sigv4Args {
        access_key,
        secret_key,
        region: R2_REGION,
        host: &host,
        canonical_uri: &canonical_uri,
        canonical_query: "",
        range: None,
        amz_date: &amz_date,
        date_stamp: &date_stamp,
        session_token: None,
    });
    let got = send("GET", &url, &get_headers, None)
        .map_err(|e| format!("r2_probe: get spawn failed — {}", e))?;
    if !got.status.success() {
        return Err(named_reason("get", &got));
    }
    if got.stdout != BODY {
        return Err(format!(
            "r2_probe: get body mismatch — {} bytes returned",
            got.stdout.len()
        ));
    }

    let empty_sha = sha256::sha256_hex(b"");
    let del_headers = sigv4_put_headers(&Sigv4PutArgs {
        method: "DELETE",
        access_key,
        secret_key,
        region: R2_REGION,
        host: &host,
        canonical_uri: &canonical_uri,
        payload_sha256: &empty_sha,
        content_length: None,
        amz_date: &amz_date,
        date_stamp: &date_stamp,
    });
    let del = send("DELETE", &url, &del_headers, None)
        .map_err(|e| format!("r2_probe: delete spawn failed — {}", e))?;
    if !del.status.success() {
        return Err(named_reason("delete", &del));
    }

    if std::env::var("OMEGAFLOW_R2_MIRROR").is_ok() {
        let tmp = std::env::temp_dir().join("r2-mirror-probe.txt");
        std::fs::write(&tmp, BODY)
            .map_err(|e| format!("r2_probe: mirror temp write void — {}", e))?;
        let mirrored =
            omegaflow::archivar::cdn::r2_mirror("ci-probe", &tmp.to_string_lossy());
        println!("r2_probe: cdn::r2_mirror -> {}", mirrored);
        let mkey = format!("/{}/ci-probe/r2-mirror-probe.txt", bucket);
        let muri = format!("{}{}", endpoint, mkey);
        let mheaders = sigv4_put_headers(&Sigv4PutArgs {
            method: "DELETE",
            access_key,
            secret_key,
            region: R2_REGION,
            host: &host,
            canonical_uri: &mkey,
            payload_sha256: &empty_sha,
            content_length: None,
            amz_date: &amz_date,
            date_stamp: &date_stamp,
        });
        let _ = send("DELETE", &muri, &mheaders, None);
    }

    println!(
        "r2_probe: put/get/delete ok — bucket={} key={} sha256={}",
        bucket, OBJECT_KEY, payload_sha256
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(reason) => {
            eprintln!("{}", reason);
            ExitCode::from(2)
        }
    }
}
