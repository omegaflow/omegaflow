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

fn uws_create(root: &str, adql: &str, responseformat: &str) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
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
        .arg(format!("QUERY={}", adql));
    if !responseformat.is_empty() {
        cmd.arg("--data-urlencode")
            .arg(format!("RESPONSEFORMAT={}", responseformat));
    }
    let out = cmd.arg(root).output().ok()?;
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

struct FieldSpec {
    name: String,
    datatype: String,
    arraysize: Option<usize>,
}

fn field_specs(body: &str) -> Vec<FieldSpec> {
    let mut out = Vec::new();
    for f in body.split("<FIELD").skip(1) {
        let Some((head, _)) = f.split_once('>') else {
            continue;
        };
        let head = head.trim().trim_end_matches('/').trim_end();
        let name = match attr(head, "name") {
            Some(n) => n,
            None => {
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
            }
        };
        let Some(datatype) = attr(head, "datatype") else {
            continue;
        };
        let arraysize = match attr(head, "arraysize") {
            None => Some(1),
            Some(a) if a == "*" => None,
            Some(a) => a.parse().ok().filter(|n| *n > 0),
        };
        out.push(FieldSpec {
            name,
            datatype,
            arraysize,
        });
    }
    out
}

fn bin_width(datatype: &str) -> Option<usize> {
    match datatype.to_ascii_lowercase().as_str() {
        "boolean" | "unsignedbyte" | "char" => Some(1),
        "short" | "unicodechar" => Some(2),
        "int" | "float" => Some(4),
        "long" | "double" => Some(8),
        _ => None,
    }
}

fn b64_decode(s: &str) -> Option<Vec<u8>> {
    let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out: Vec<u8> = Vec::new();
    let mut buf: u32 = 0;
    let mut bits = 0u32;
    for c in s.bytes() {
        if c == b'=' || c.is_ascii_whitespace() {
            continue;
        }
        let v = table.iter().position(|&t| t == c)?;
        buf = (buf << 6) | v as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    Some(out)
}

fn binary_cell(datatype: &str, bytes: &[u8]) -> String {
    match datatype.to_ascii_lowercase().as_str() {
        "double" => {
            let Ok(a) = <[u8; 8]>::try_from(bytes) else {
                return String::new();
            };
            let v = f64::from_be_bytes(a);
            if v.is_finite() {
                format!("{}", v)
            } else {
                String::new()
            }
        }
        "float" => {
            let Ok(a) = <[u8; 4]>::try_from(bytes) else {
                return String::new();
            };
            let v = f32::from_be_bytes(a) as f64;
            if v.is_finite() {
                format!("{}", v)
            } else {
                String::new()
            }
        }
        "long" => {
            let Ok(a) = <[u8; 8]>::try_from(bytes) else {
                return String::new();
            };
            format!("{}", i64::from_be_bytes(a))
        }
        "int" => {
            let Ok(a) = <[u8; 4]>::try_from(bytes) else {
                return String::new();
            };
            format!("{}", i32::from_be_bytes(a))
        }
        "short" => {
            let Ok(a) = <[u8; 2]>::try_from(bytes) else {
                return String::new();
            };
            format!("{}", i16::from_be_bytes(a))
        }
        "unsignedbyte" | "boolean" => match bytes.first() {
            Some(b) => format!("{}", b),
            None => String::new(),
        },
        "char" => String::from_utf8_lossy(bytes)
            .trim_end_matches('\0')
            .trim()
            .to_string(),
        "unicodechar" => {
            let units: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16_lossy(&units)
                .trim_end_matches('\0')
                .trim()
                .to_string()
        }
        _ => String::new(),
    }
}

fn decode_binary_stream(bytes: &[u8], specs: &[FieldSpec]) -> Option<Vec<Vec<String>>> {
    let mut rows = Vec::new();
    let mut pos = 0usize;
    while pos < bytes.len() {
        let mut cells = Vec::with_capacity(specs.len());
        let mut complete = true;
        for s in specs {
            let Some(w) = bin_width(&s.datatype) else {
                eprintln!(
                    "votable BINARY: field {} datatype {} unsupported — the result stays unwritten",
                    s.name, s.datatype
                );
                return None;
            };
            let n = match s.arraysize {
                Some(n) => n,
                None => {
                    let Some(len) = bytes.get(pos..pos + 4) else {
                        complete = false;
                        break;
                    };
                    pos += 4;
                    u32::from_be_bytes([len[0], len[1], len[2], len[3]]) as usize
                }
            };
            let size = w.checked_mul(n)?;
            let Some(cell) = bytes.get(pos..pos + size) else {
                complete = false;
                break;
            };
            pos += size;
            cells.push(binary_cell(&s.datatype, cell));
        }
        if !complete {
            eprintln!("votable BINARY: row truncated at byte {pos} — the result stays unwritten");
            return None;
        }
        rows.push(cells);
    }
    if rows.is_empty() {
        return None;
    }
    Some(rows)
}

fn binary_rows(
    tail: &str,
    specs: &[FieldSpec],
    fields: &[String],
) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let tail = tail.trim_start();
    if tail.starts_with('2') {
        eprintln!("votable BINARY2 stream — unsupported, the result stays unwritten");
        return None;
    }
    let Some((_, after)) = tail.split_once('>') else {
        eprintln!("votable BINARY tag unterminated — the result stays unwritten");
        return None;
    };
    let Some(st) = after.split("<STREAM").nth(1) else {
        eprintln!("votable BINARY without STREAM — the result stays unwritten");
        return None;
    };
    let Some((sthead, sttail)) = st.split_once('>') else {
        eprintln!("votable STREAM tag unterminated — the result stays unwritten");
        return None;
    };
    let Some(encoding) = attr(sthead, "encoding") else {
        eprintln!("votable BINARY STREAM encoding absent — the result stays unwritten");
        return None;
    };
    if !encoding.eq_ignore_ascii_case("base64") {
        eprintln!(
            "votable BINARY STREAM encoding {encoding} — unsupported, the result stays unwritten"
        );
        return None;
    }
    let content = match sttail.split_once("</STREAM>") {
        Some((c, _)) => c,
        None => sttail,
    };
    let Some(bytes) = b64_decode(content) else {
        eprintln!("votable BINARY STREAM base64 void — the result stays unwritten");
        return None;
    };
    let Some(rows) = decode_binary_stream(&bytes, specs) else {
        return None;
    };
    if fields.is_empty() || rows.is_empty() {
        eprintln!(
            "votable BINARY: fields={} rows={}",
            fields.len(),
            rows.len()
        );
        return None;
    }
    Some((fields.to_vec(), rows))
}

fn votable_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let specs = field_specs(body);
    let fields: Vec<String> = specs.iter().map(|s| s.name.clone()).collect();
    let data = match body.split("<DATA").nth(1) {
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
    let inner = match data.split_once('>') {
        Some((_, rest)) => rest,
        None => {
            eprintln!(
                "votable: <DATA> tag unterminated, fields={} body_len={}",
                fields.len(),
                body.len()
            );
            return None;
        }
    };
    if let Some(tail) = inner.split("<BINARY").nth(1) {
        return binary_rows(tail, &specs, &fields);
    }
    let mut rows = Vec::new();
    for tr in inner.split("<TR>").skip(1) {
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

fn run(root: &str, adql: &str, poll_secs: u64, out: &str, ci_mode: bool, responseformat: &str) {
    let Some(job) = uws_create(root, adql, responseformat) else {
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
    let responseformat = match arg_value(&args, "--responseformat") {
        Some(v) => v,
        None => "votable/td".to_string(),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    run(&root, &adql, poll_secs, &out, ci_mode, &responseformat);
}
