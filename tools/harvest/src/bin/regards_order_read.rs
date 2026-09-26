use std::fs;
use std::io::Read;
use std::process::Command;

use omegaflow::json::{JsonVal, jnum, jpath_val, jstr, parse_json};

const ORDER_API: &str = "https://regards.cnes.fr/api/v1/rs-order";
const AUTH: &str = "https://regards.cnes.fr/api/v1/rs-authentication/oauth/token";

fn first_str(v: &JsonVal, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| jstr(v, k))
}

fn first_int(v: &JsonVal, keys: &[&str]) -> Option<i64> {
    keys.iter().find_map(|k| jnum(v, k)).map(|n| n as i64)
}

fn first_bool(v: &JsonVal, keys: &[&str]) -> Option<bool> {
    keys.iter().find_map(|k| match jpath_val(v, k) {
        Some(JsonVal::Bool(b)) => Some(*b),
        _ => None,
    })
}

fn order_obj(root: &JsonVal) -> Option<&JsonVal> {
    match root {
        JsonVal::Obj(map) => {
            if let Some(JsonVal::Obj(c)) = map.get("content") {
                if c.contains_key("status") || c.contains_key("id") {
                    return map.get("content");
                }
            }
            Some(root)
        }
        _ => None,
    }
}

fn file_entries(root: &JsonVal) -> Option<Vec<&JsonVal>> {
    match root {
        JsonVal::Obj(map) => {
            if let Some(JsonVal::Arr(arr)) = map.get("content") {
                return Some(arr.iter().collect());
            }
            if let Some(JsonVal::Obj(c)) = map.get("content") {
                if let Some(JsonVal::Arr(arr)) = c.get("files") {
                    return Some(arr.iter().collect());
                }
            }
            if let Some(JsonVal::Arr(arr)) = map.get("files") {
                return Some(arr.iter().collect());
            }
            None
        }
        JsonVal::Arr(arr) => Some(arr.iter().collect()),
        _ => None,
    }
}

fn total_elements(root: &JsonVal) -> Option<i64> {
    first_int(
        root,
        &["metadata.totalElements", "totalElements", "metadata.total"],
    )
}

fn download_url(oid: &str, checksum: &str) -> String {
    format!("{ORDER_API}/user/orders/{oid}/files/{checksum}/download")
}

fn file_url(f: &JsonVal, oid: Option<&str>, checksum: &str) -> Option<String> {
    if let Some(u) = first_str(f, &["url", "self", "downloadUrl", "download", "href"]) {
        return Some(u);
    }
    oid.map(|o| download_url(o, checksum))
}

fn print_summary(oid: Option<&str>, order: &JsonVal) {
    println!(
        "order {}",
        match oid {
            Some(o) => o,
            None => "absent",
        }
    );
    let label = first_str(order, &["label"]);
    let status = first_str(order, &["status"]);
    let status_date = first_str(order, &["statusDate", "date"]);
    let in_error = first_int(order, &["filesInErrorCount", "filesInError"]);
    let available = first_int(order, &["availableFilesCount", "availableFiles"]);
    let total = first_int(order, &["filesCount", "totalFilesCount"]);

    match label {
        Some(l) => println!("  label            {l}"),
        None => println!("  label            absent"),
    }
    match status {
        Some(s) => println!("  status           {s}"),
        None => println!("  status           absent"),
    }
    match status_date {
        Some(d) => println!("  statusDate       {d}"),
        None => println!("  statusDate       absent"),
    }
    match total {
        Some(n) => println!("  filesCount       {n}"),
        None => println!("  filesCount       absent"),
    }
    match available {
        Some(n) => println!("  availableFiles   {n}"),
        None => println!("  availableFiles   absent"),
    }
    match in_error {
        Some(n) => println!("  filesInError     {n}"),
        None => println!("  filesInError     absent"),
    }
}

fn print_file_line(oid: Option<&str>, f: &JsonVal) -> Option<bool> {
    let checksum = first_str(f, &["checksum", "md5", "sum"])?;
    let filename = first_str(f, &["filename", "name", "title", "file"]);
    let size = first_int(f, &["filesize", "fileSize", "size", "length"]);
    let online = first_bool(f, &["online", "available"]);
    let url = file_url(f, oid, &checksum);

    let name = match filename {
        Some(n) => n,
        None => checksum.clone(),
    };
    let size_str = match size {
        Some(n) => n.to_string(),
        None => "-".to_string(),
    };
    let online_str = match online {
        Some(true) => "online",
        Some(false) => "offline",
        None => "unknown",
    };
    let url_present = url.is_some();
    let url_str = match url {
        Some(u) => u,
        None => "absent".to_string(),
    };
    println!("{checksum}\t{name}\t{size_str}\t{online_str}\t{url_str}");
    Some(url_present)
}

fn print_order(root: &JsonVal, order_id_flag: Option<&str>) -> i32 {
    let Some(order) = order_obj(root) else {
        eprintln!("regards_order_read: no order object in the JSON (0 honored)");
        return 1;
    };
    let oid = first_int(order, &["id"]).map(|n| n.to_string());
    let oid_ref: Option<&str> = oid.as_deref().or(order_id_flag);
    print_summary(oid_ref, order);

    let Some(entries) = file_entries(root) else {
        match oid_ref {
            Some(o) => {
                println!("  files            absent — the order carries no file list here");
                println!("files page  {ORDER_API}/user/orders/{o}/files");
                println!("order       {ORDER_API}/user/orders/{o}");
            }
            None => {
                println!("  files            absent — the order carries no file list here");
                println!("files page  {ORDER_API}/user/orders/<id>/files");
            }
        }
        return 0;
    };

    if let Some(tot) = total_elements(root) {
        println!(
            "  page             {} entries of {tot} total",
            entries.len()
        );
    } else {
        println!("  page             {} entries", entries.len());
    }

    let mut listed = 0usize;
    let mut no_checksum = 0usize;
    let mut online = 0usize;
    let mut offline = 0usize;
    let mut url_absent = 0usize;
    for f in &entries {
        match print_file_line(oid_ref, f) {
            Some(url_present) => {
                listed += 1;
                match first_bool(f, &["online", "available"]) {
                    Some(true) => online += 1,
                    _ => offline += 1,
                }
                if !url_present {
                    url_absent += 1;
                }
            }
            None => no_checksum += 1,
        }
    }
    println!(
        "listed {listed} files ({online} online, {offline} offline); {no_checksum} entries without a checksum; {url_absent} download URLs absent (no order id)"
    );
    0
}

fn auth_header(token: &str) -> Vec<(String, String)> {
    vec![("Authorization".into(), format!("Bearer {token}"))]
}

fn login(user: &str, pass: &str) -> Option<String> {
    let url = format!("{AUTH}?grant_type=password&scope=cdpp&username={user}&password={pass}");
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-X")
        .arg("POST")
        .arg("-m")
        .arg("45")
        .arg("-H")
        .arg("Authorization: Basic Y2xpZW50OnNlY3JldA==")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!(
            "login returned {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    let body = String::from_utf8_lossy(&out.stdout).into_owned();
    let v = parse_json(&body)?;
    jstr(&v, "access_token")
}

struct HttpReply {
    status: u16,
    body: String,
}

fn fetch_http(url: &str, method: &str, headers: &[(String, String)]) -> Option<HttpReply> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-g")
        .arg("-m")
        .arg("45")
        .arg("--connect-timeout")
        .arg("20")
        .arg("-w")
        .arg("\n%{http_code}");
    if method != "GET" {
        cmd.arg("-X").arg(method);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        eprintln!(
            "http {method} transport void ({}): {} {}",
            output.status,
            url,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let (body, code) = stdout.rsplit_once('\n')?;
    let status: u16 = code.trim().parse().ok()?;
    Some(HttpReply {
        status,
        body: body.to_string(),
    })
}

fn live(order_id: &str) -> i32 {
    let user = match std::env::var("CDPP_USER") {
        Ok(u) => u,
        Err(_) => {
            eprintln!(
                "regards_order_read --live: CDPP_USER absent — the browser-exported JSON path is the standing route (0 honored)"
            );
            return 2;
        }
    };
    let pass = match std::env::var("CDPP_PASS") {
        Ok(p) => p,
        Err(_) => {
            eprintln!(
                "regards_order_read --live: CDPP_PASS absent — the browser-exported JSON path is the standing route (0 honored)"
            );
            return 2;
        }
    };
    let Some(token) = login(&user, &pass) else {
        eprintln!("regards_order_read --live: login void — the order stays unread (0 honored)");
        return 1;
    };
    let url = format!("{ORDER_API}/user/orders/{order_id}");
    match fetch_http(&url, "GET", &auth_header(&token)) {
        Some(r) if (200..300).contains(&r.status) => match parse_json(&r.body) {
            Some(v) => print_order(&v, None),
            None => {
                eprintln!(
                    "regards_order_read --live: order {order_id} 200 but unparsable JSON — {} bytes",
                    r.body.len()
                );
                1
            }
        },
        Some(r) if r.status == 403 => {
            eprintln!(
                "regards_order_read --live: order {order_id} measured 403 F5-ASM-WAF (direct) — the API is WAF-blocked; the reader runs on the browser-exported JSON"
            );
            2
        }
        Some(r) => {
            eprintln!(
                "regards_order_read --live: order {order_id} HTTP {}",
                r.status
            );
            1
        }
        None => {
            eprintln!("regards_order_read --live: order {order_id} transport void");
            1
        }
    }
}

fn print_urls(order_id: &str) {
    println!("order       {ORDER_API}/user/orders/{order_id}");
    println!("files page  {ORDER_API}/user/orders/{order_id}/files");
    println!("download    {ORDER_API}/user/orders/{order_id}/files/<checksum>/download");
}

fn read_input(path: Option<&str>) -> String {
    match path {
        Some(p) => match fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("regards_order_read: {p} unreadable: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let mut buf = String::new();
            match std::io::stdin().read_to_string(&mut buf) {
                Ok(_) => buf,
                Err(e) => {
                    eprintln!("regards_order_read: stdin unreadable: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut order_id_flag: Option<String> = None;
    let mut rest: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--order-id" => {
                if i + 1 < args.len() {
                    i += 1;
                    order_id_flag = Some(args[i].clone());
                }
            }
            other => rest.push(other),
        }
        i += 1;
    }
    match rest.first().copied() {
        Some("--live") => match rest.get(1) {
            Some(id) => live(id),
            None => {
                eprintln!("regards_order_read --live: an order id is the argument");
                2
            }
        },
        Some("--urls") => match rest.get(1) {
            Some(id) => {
                print_urls(id);
                0
            }
            None => {
                eprintln!("regards_order_read --urls: an order id is the argument");
                2
            }
        },
        Some("--help") => {
            println!(
                "regards_order_read [--order-id <id>] <order.json> | (stdin) | --urls <id> | --live <id>"
            );
            0
        }
        Some(p) if p.starts_with('-') => {
            eprintln!("regards_order_read: unknown flag {p}");
            2
        }
        path => {
            let body = read_input(path);
            let Some(root) = parse_json(&body) else {
                eprintln!(
                    "regards_order_read: input unparsable — {} bytes, no JSON object (0 honored)",
                    body.len()
                );
                return 1;
            };
            print_order(&root, order_id_flag.as_deref())
        }
    }
}

fn main() {
    std::process::exit(run());
}
