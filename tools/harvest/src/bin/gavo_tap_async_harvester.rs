use omegaflow::cdn::upload_release;
use std::io::Write;
use std::process::Command;

const DEFAULT_ROOT: &str = "https://dc.g-vo.org/tap/async";
const NETLOC: &str = "dc.g-vo.org";
const POLL_STEP_S: u64 = 10;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_status(prefix: &str, status: std::process::ExitStatus, stderr: &[u8]) {
    eprintln!(
        "{prefix} http {}: {}",
        status,
        String::from_utf8_lossy(stderr).trim()
    );
}

fn uws_create(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("120")
        .arg("-D")
        .arg("-")
        .arg("-o")
        .arg("/dev/null")
        .arg("-X")
        .arg("POST")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if !out.status.success() {
        curl_status("uws create", out.status, &out.stderr);
        return None;
    }
    let headers = String::from_utf8_lossy(&out.stdout);
    let location = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("location:"))
        .map(|l| l["location:".len()..].trim().to_string())?;
    if location.starts_with("http://") || location.starts_with("https://") {
        return Some(location);
    }
    let (scheme, after) = match root.split_once("://") {
        Some((s, a)) => (s, a),
        None => return Some(location),
    };
    let host = after.split('/').next().unwrap_or(after);
    if location.starts_with('/') {
        Some(format!("{scheme}://{host}{location}"))
    } else {
        let dir = after.rsplit_once('/').map(|(d, _)| d).unwrap_or(after);
        Some(format!("{scheme}://{host}{dir}/{location}"))
    }
}

fn uws_post_phase(job: &str, phase: &str) -> bool {
    match Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("120")
        .arg("-o")
        .arg("/dev/null")
        .arg("-X")
        .arg("POST")
        .arg("--data-urlencode")
        .arg(format!("PHASE={}", phase))
        .arg(format!("{job}/phase"))
        .output()
    {
        Ok(out) if out.status.success() => true,
        Ok(out) => {
            curl_status("uws phase post", out.status, &out.stderr);
            false
        }
        Err(_) => false,
    }
}

fn uws_phase(job: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("60")
        .arg(format!("{job}/phase"))
        .output()
        .ok()?;
    if !out.status.success() {
        curl_status("uws phase read", out.status, &out.stderr);
        return None;
    }
    String::from_utf8(out.stdout)
        .ok()
        .map(|s| s.trim().to_string())
}

fn uws_result(job: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("3600")
        .arg(format!("{job}/results/result"))
        .output()
        .ok()?;
    if !out.status.success() {
        curl_status("uws result", out.status, &out.stderr);
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

fn attr(seg: &str, key: &str) -> Option<String> {
    for part in seg.split_whitespace() {
        let Some(rest) = part.strip_prefix(key) else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let value = rest.trim_start().trim_matches('"').trim_matches('\'');
        return Some(value.to_string());
    }
    None
}

fn info_value(body: &str, want: &str) -> Option<String> {
    for seg in body.split("<INFO").skip(1) {
        let seg = match seg.split_once('>') {
            Some((s, _)) => s,
            None => seg,
        };
        let Some(name) = attr(seg, "name") else {
            continue;
        };
        if name == want {
            return attr(seg, "value");
        }
    }
    None
}

fn votable_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut fields: Vec<String> = Vec::new();
    for f in body.split("<FIELD").skip(1) {
        let seg = match f.split_once('>') {
            Some((s, _)) => s,
            None => continue,
        };
        let name = if let Some(n) = attr(seg, "name") {
            n
        } else {
            let inner = match f.split_once('>') {
                Some((_, rest)) => match rest.split_once('<') {
                    Some((i, _)) => i.trim().to_string(),
                    None => continue,
                },
                None => continue,
            };
            if inner.is_empty() {
                continue;
            }
            inner
        };
        fields.push(name);
    }
    let data = match body.split("<DATA>").nth(1) {
        Some(d) => d,
        None => {
            eprintln!(
                "votable: <DATA> absent, fields={} body_len={}",
                fields.len(),
                body.len()
            );
            return None;
        }
    };
    let mut rows = Vec::new();
    for tr in data.split("<TR>").skip(1) {
        let Some((end, _)) = tr.split_once("</TR>") else {
            continue;
        };
        let mut cells = Vec::new();
        for td in end.split("<TD>").skip(1) {
            let Some((raw, _)) = td.split_once("</TD>") else {
                continue;
            };
            let v = if let Some(c) = raw.trim().strip_prefix("<![CDATA[") {
                c.strip_suffix("]]>").unwrap_or(c).trim().to_string()
            } else {
                raw.trim().to_string()
            };
            cells.push(v);
        }
        rows.push(cells);
    }
    if fields.is_empty() || rows.is_empty() {
        eprintln!(
            "votable: fields={} rows={} body_len={}",
            fields.len(),
            rows.len(),
            body.len()
        );
        return None;
    }
    Some((fields, rows))
}

fn json_string(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            _ => o.push(c),
        }
    }
    o.push('"');
    o
}

fn json_cell(raw: &str) -> String {
    if raw.is_empty() {
        return "null".to_string();
    }
    match raw.parse::<f64>() {
        Ok(v) if v.is_finite() => raw.to_string(),
        _ => json_string(raw),
    }
}

fn emit_json(fields: &[String], rows: &[Vec<String>]) -> String {
    let mut buf = String::from("[");
    for (r, cells) in rows.iter().enumerate() {
        if r > 0 {
            buf.push(',');
        }
        let mut obj = String::from("{");
        for (i, f) in fields.iter().enumerate() {
            if i > 0 {
                obj.push(',');
            }
            let raw = cells.get(i).map(|s| s.as_str()).unwrap_or("");
            obj.push_str(&format!("\"{}\":{}", f, json_cell(raw)));
        }
        obj.push('}');
        buf.push_str(&obj);
    }
    buf.push_str("]\n");
    buf
}

fn run(root: &str, adql: &str, poll_secs: u64, out: &str, ci_mode: bool) {
    let Some(job) = uws_create(root, adql) else {
        eprintln!("uws create void at {root} — the query stays unharvested");
        std::process::exit(1);
    };
    eprintln!("uws job: {job}");
    if !uws_post_phase(&job, "RUN") {
        eprintln!("uws run post void at {job}/phase — the job may stay PENDING");
    }
    let mut phase = uws_phase(&job);
    let mut steps = poll_secs / POLL_STEP_S + 1;
    while steps > 0 {
        match phase.as_deref() {
            Some("COMPLETED") => break,
            Some("ERROR") | Some("ABORTED") => {
                eprintln!(
                    "uws job phase {phase} — the query stays unharvested",
                    phase = phase.as_deref().unwrap_or("")
                );
                std::process::exit(1);
            }
            _ => {}
        }
        std::thread::sleep(std::time::Duration::from_secs(POLL_STEP_S));
        phase = uws_phase(&job);
        steps -= 1;
    }
    if phase.as_deref() != Some("COMPLETED") {
        eprintln!(
            "uws job phase {} after {} s — the query stays unharvested",
            phase.as_deref().unwrap_or("void"),
            poll_secs
        );
        std::process::exit(1);
    }
    let Some(body) = uws_result(&job) else {
        eprintln!("uws result void at {job} — the query stays unharvested");
        std::process::exit(1);
    };
    match info_value(&body, "QUERY_STATUS").as_deref() {
        Some("OK") => {}
        Some(status) => {
            eprintln!("votable QUERY_STATUS {status} — the result stays unwritten");
            std::process::exit(1);
        }
        None => {}
    }
    let Some((fields, rows)) = votable_rows(&body) else {
        eprintln!(
            "votable parse void ({body_len} B) — the result stays unwritten",
            body_len = body.len()
        );
        std::process::exit(1);
    };
    let json = emit_json(&fields, &rows);
    if let Some(parent) = std::path::Path::new(out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::File::create(out) {
        Ok(mut f) => {
            if let Err(err) = f.write_all(json.as_bytes()) {
                eprintln!("write {out}: {err}");
                std::process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("write {out} returned void");
            std::process::exit(1);
        }
    }
    eprintln!(
        "gavo tap async: {} fields, {} rows → {out} ({} B)",
        fields.len(),
        rows.len(),
        json.len()
    );
    if ci_mode && !upload_release(NETLOC, out) {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = match arg_value(&args, "--root") {
        Some(r) => r,
        None => DEFAULT_ROOT.to_string(),
    };
    let Some(adql) = arg_value(&args, "--query") else {
        eprintln!("--query <ADQL> absent");
        std::process::exit(1);
    };
    let Some(out) = arg_value(&args, "--out") else {
        eprintln!("--out <path> absent");
        std::process::exit(1);
    };
    let poll_secs = arg_value(&args, "--poll")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1800);
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    run(&root, &adql, poll_secs, &out, ci_mode);
}
