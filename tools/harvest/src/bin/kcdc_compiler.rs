use omegaflow::archivar::kcdc::{
    MAGIC, Table, array_comp, calorimeter_comp, general_comp, grande_comp, is_log10, lopes_comp,
    mapped_times, own_gt_times, parse_bin, parse_row_mapping, plausible, read_table, write_bin,
};
use omegaflow::cdn::upload_release;
use omegaflow::lsk::{LeapSeconds, parse as parse_lsk};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const BASE: &str = "https://kcdc.iap.kit.edu";
const LOGIN_PATH: &str = "/accounts/login/";
const NETLOC: &str = "kcdc.iap.kit.edu";
const DEFAULT_JAR: &str = "tmp/kcdc_cookies.txt";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn secret(name: &str) -> Option<String> {
    if let Ok(v) = std::env::var(name) {
        if !v.is_empty() {
            return Some(v);
        }
    }
    let body = fs::read_to_string(".secrets.local").ok()?;
    for line in body.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == name && !v.trim().is_empty() {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn form_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn job_body(out_format: &str, data_json: &str) -> String {
    format!(
        "out_format={}&data={}",
        form_encode(out_format),
        form_encode(data_json)
    )
}

fn jar_value(jar: &Path, name: &str) -> Option<String> {
    let text = fs::read_to_string(jar).ok()?;
    for line in text.lines() {
        let line = line.strip_prefix("#HttpOnly_").unwrap_or(line);
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 7 {
            continue;
        }
        if parts[5] == name && !parts[6].is_empty() {
            return Some(parts[6].to_string());
        }
    }
    None
}

fn curl_get(jar: &Path, url: &str, extra: &[&str]) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-m")
        .arg("60")
        .arg("-c")
        .arg(jar)
        .arg("-b")
        .arg(jar);
    for e in extra {
        cmd.arg(e);
    }
    let out = cmd.arg(url).output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "kcdc: GET {} returned ({}): {}",
            url,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn login(jar: &Path, user: &str, pass: &str) -> Option<String> {
    let login_url = format!("{BASE}{LOGIN_PATH}");
    curl_get(jar, &login_url, &[])?;
    let csrf = jar_value(jar, "csrftoken")?;
    let body = format!(
        "username={}&password={}&csrfmiddlewaretoken={}",
        form_encode(user),
        form_encode(pass),
        form_encode(&csrf)
    );
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-m")
        .arg("60")
        .arg("-c")
        .arg(jar)
        .arg("-b")
        .arg(jar)
        .arg("-H")
        .arg(format!("Referer: {BASE}{LOGIN_PATH}"))
        .arg("-d")
        .arg(body)
        .arg(&login_url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!(
            "kcdc: login POST returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    if jar_value(jar, "sessionid").is_none() {
        return None;
    }
    jar_value(jar, "csrftoken")
}

fn quants(jar: &Path, det_name: &str) -> Option<String> {
    curl_get(
        jar,
        &format!("{BASE}/datashop/quants/?det_prefix=&det_name={det_name}"),
        &["-H", "X-Requested-With: XMLHttpRequest"],
    )
}

fn descr(jar: &Path, det_name: &str, quant_name: Option<&str>) -> Option<String> {
    let mut url = format!("{BASE}/datashop/descr/?det_prefix=&det_name={det_name}");
    if let Some(q) = quant_name {
        url.push_str("&quant_name=");
        url.push_str(q);
    }
    curl_get(jar, &url, &["-H", "X-Requested-With: XMLHttpRequest"])
}

fn post_command(jar: &Path, csrf: &str, prefix: &str, body: &str) -> String {
    let url = format!("{BASE}/datashop/{prefix}");
    format!(
        "curl -s -S -f -L -m 300 -c {jar} -b {jar} \
         -H \"X-CSRFToken: {csrf}\" -H \"Referer: {BASE}/datashop/\" -d '{body}' {url}",
        jar = jar.display()
    )
}

fn submit_request(jar: &Path, csrf: &str, prefix: &str, body: &str) -> Option<String> {
    println!(
        "kcdc: the POST reaches the KCDC DataShop and creates a request there ({url})",
        url = format!("{BASE}/datashop/{prefix}")
    );
    let out = Command::new("curl")
        .arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-m")
        .arg("300")
        .arg("-c")
        .arg(jar)
        .arg("-b")
        .arg(jar)
        .arg("-H")
        .arg(format!("X-CSRFToken: {csrf}"))
        .arg("-H")
        .arg(format!("Referer: {BASE}/datashop/"))
        .arg("-d")
        .arg(body)
        .arg(format!("{BASE}/datashop/{prefix}"))
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "kcdc: submit returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn le16(d: &[u8], off: usize) -> usize {
    d[off] as usize | (d[off + 1] as usize) << 8
}

fn le32(d: &[u8], off: usize) -> usize {
    d[off] as usize
        | (d[off + 1] as usize) << 8
        | (d[off + 2] as usize) << 16
        | (d[off + 3] as usize) << 24
}

fn zip_entries(data: &[u8]) -> Vec<(String, usize, usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 46 <= data.len() {
        if &data[i..i + 4] == b"PK\x01\x02" {
            let method = le16(data, i + 10);
            let comp_size = le32(data, i + 20);
            let name_len = le16(data, i + 28);
            let extra_len = le16(data, i + 30);
            let comment_len = le16(data, i + 32);
            let local_off = le32(data, i + 42);
            if i + 46 + name_len <= data.len() {
                let name = String::from_utf8_lossy(&data[i + 46..i + 46 + name_len]).into_owned();
                out.push((name, method, comp_size, local_off));
            }
            i += 46 + name_len + extra_len + comment_len;
            continue;
        }
        i += 1;
    }
    out
}

fn zip_entry_bytes(data: &[u8], want: &str) -> Option<Vec<u8>> {
    for (name, method, comp_size, local_off) in zip_entries(data) {
        if name.rsplit('/').next() != Some(want) {
            continue;
        }
        if local_off + 30 > data.len() {
            continue;
        }
        let name_len = le16(data, local_off + 26);
        let extra_len = le16(data, local_off + 28);
        let start = local_off + 30 + name_len + extra_len;
        if start + comp_size > data.len() {
            continue;
        }
        let body = &data[start..start + comp_size];
        return match method {
            0 => Some(body.to_vec()),
            8 => omegaflow::inflate::inflate(body),
            _ => None,
        };
    }
    None
}

fn table_text(src: &Path, name: &str, zip: Option<&[u8]>) -> Option<String> {
    match zip {
        Some(data) => zip_entry_bytes(data, name).map(|b| String::from_utf8_lossy(&b).into_owned()),
        None => {
            let path = src.join(name);
            fs::read_to_string(&path).ok().map(|s| {
                eprintln!("kcdc: {} read ({} B)", name, s.len());
                s
            })
        }
    }
}

fn compile_table(
    table: &Table,
    comp_of: fn(&str) -> Option<u32>,
    times: &[Option<f64>],
    lsk: &LeapSeconds,
    records: &mut Vec<(f64, f64, u32)>,
) -> (usize, usize, usize) {
    let mut emitted = 0usize;
    let mut absent = 0usize;
    let mut timed_out = 0usize;
    for (r, row) in table.rows.iter().enumerate() {
        let Some(t) = times.get(r).copied().flatten() else {
            timed_out += 1;
            continue;
        };
        let Some(tdb) = lsk.unix_to_tdb(t) else {
            timed_out += 1;
            continue;
        };
        for (i, token) in table.columns.iter().enumerate() {
            let Some(comp) = comp_of(token) else {
                continue;
            };
            let Some(cell) = row.get(i).copied().flatten() else {
                absent += 1;
                continue;
            };
            let v = if is_log10(comp) {
                10.0f64.powf(cell)
            } else {
                cell
            };
            if !plausible(comp, v) {
                absent += 1;
                continue;
            }
            records.push((tdb, v, comp));
            emitted += 1;
        }
    }
    (emitted, absent, timed_out)
}

fn compile(src: &Path, out_path: &str, lsk: &LeapSeconds, ci: bool) -> Result<(), String> {
    let data = if src.is_file() {
        fs::read(src).map_err(|e| format!("{}: {e}", src.display()))?
    } else {
        Vec::new()
    };
    let zip: Option<&[u8]> = if data.len() >= 4 && &data[0..4] == b"PK\x03\x04" {
        Some(&data)
    } else {
        None
    };

    let mapping = table_text(src, "row_mapping.txt", zip).and_then(|t| parse_row_mapping(&t));
    let general = table_text(src, "general.txt", zip).and_then(|t| read_table(&t));
    let general_times = match &general {
        Some(t) => own_gt_times(t),
        None => Vec::new(),
    };
    let mapping_label = match &mapping {
        Some(m) => m.tables.join("|"),
        None => String::from("absent"),
    };
    match &general {
        Some(t) => eprintln!(
            "kcdc: general {} rows, {} timed; row_mapping {}",
            t.rows.len(),
            general_times.iter().filter(|t| t.is_some()).count(),
            mapping_label,
        ),
        None => eprintln!("kcdc: general absent; row_mapping {mapping_label}"),
    }

    let mut records: Vec<(f64, f64, u32)> = Vec::new();
    let component_maps: [(&str, fn(&str) -> Option<u32>); 5] = [
        ("array", array_comp),
        ("grande", grande_comp),
        ("calorimeter", calorimeter_comp),
        ("general", general_comp),
        ("lopes", lopes_comp),
    ];
    for (stem, comp_of) in component_maps {
        let file = format!("{stem}.txt");
        let Some(text) = table_text(src, &file, zip) else {
            eprintln!("kcdc: {file} absent — nothing to compile from it");
            continue;
        };
        let Some(table) = read_table(&text) else {
            eprintln!("kcdc: {file} carries no header line — the layout stays unread (0 honored)");
            continue;
        };
        let times = match &mapping {
            Some(m) if stem != "general" => mapped_times(m, &general_times, stem),
            _ => own_gt_times(&table),
        };
        let (emitted, absent, timed_out) =
            compile_table(&table, comp_of, &times, lsk, &mut records);
        eprintln!(
            "kcdc: {file}: {} rows, {} records, {} cells absent/implausible, {} rows without time",
            table.rows.len(),
            emitted,
            absent,
            timed_out
        );
    }
    if records.is_empty() {
        return Err("no air-shower records harvested — the bin stays unwritten (0 honored)".into());
    }
    records.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)));
    let bytes = write_bin(&records);
    fs::write(out_path, &bytes).map_err(|e| format!("{out_path}: {e}"))?;
    match parse_bin(&bytes) {
        Some(parsed) => eprintln!(
            "kcdc: {}: {} records, {} B, roundtrip parses (magic {})",
            out_path,
            parsed.len(),
            bytes.len(),
            String::from_utf8_lossy(&MAGIC)
        ),
        None => {
            return Err(format!(
                "{out_path}: roundtrip parse void — the bin stays unverified"
            ));
        }
    }
    if ci && !upload_release(NETLOC, out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn usage() {
    eprintln!(
        "kcdc_compiler — KCDC / KASCADE-Grande DataShop consumer (session+CSRF, no API key)
  --login                      session login (credentials: --user/--pass, env KCDC_USER/KCDC_PASS or .secrets.local)
  --quants <det_name>          login, then GET /datashop/quants/ (JSON to stdout)
  --descr <det_name> [--quant <name>]  login, then GET /datashop/descr/
  --job <format> --data <file> login, build the measured job POST body (out_format + data JSON), print the exact command (no execution)
  --request <body>             login, then print the exact POST /datashop/ command (no execution)
  --submit <body>              login, then execute the POST (creates a request at KCDC — the operator's word)
  --prefix <p>                 datashop prefix for the POST (default \"\")
  --compile <request.zip|dir>  compile the ASCII tables into the series bin
  --out <file.bin>             output path for --compile
  --lsk <naif0012.tls>         leap-second table for the unix→TDB clock
  --jar <path>                 cookie jar (default tmp/kcdc_cookies.txt)
  --ci-mode                    upload the bin to the CDN release kcdc.iap.kit.edu"
    );
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let jar_path = match arg_value(&args, "--jar") {
        Some(p) => p,
        None => DEFAULT_JAR.to_string(),
    };
    let jar = PathBuf::from(&jar_path);
    let user = arg_value(&args, "--user")
        .or_else(|| secret("KCDC_USER"))
        .ok_or("KCDC_USER absent — env, .secrets.local marker or --user".to_string())?;
    let pass = arg_value(&args, "--pass")
        .or_else(|| secret("KCDC_PASS"))
        .ok_or("KCDC_PASS absent — env, .secrets.local marker or --pass".to_string())?;

    if args.iter().any(|a| a == "--login") {
        let csrf = login(&jar, &user, &pass)
            .ok_or("login void — the session stays unopened (0 honored)")?;
        println!(
            "kcdc: session in {jar_path} (csrftoken {len} B)",
            len = csrf.len()
        );
        return Ok(());
    }
    if let Some(det) = arg_value(&args, "--quants") {
        let csrf =
            login(&jar, &user, &pass).ok_or("login void — quants stays unread (0 honored)")?;
        println!(
            "kcdc: session in {jar_path} (csrftoken {len} B)",
            len = csrf.len()
        );
        let body =
            quants(&jar, &det).ok_or("quants void — the endpoint returned nothing parseable")?;
        print!("{body}");
        return Ok(());
    }
    if let Some(det) = arg_value(&args, "--descr") {
        login(&jar, &user, &pass).ok_or("login void — descr stays unread (0 honored)")?;
        let quant = arg_value(&args, "--quant");
        let body = descr(&jar, &det, quant.as_deref())
            .ok_or("descr void — the endpoint returned nothing parseable")?;
        print!("{body}");
        return Ok(());
    }
    if let Some(format) = arg_value(&args, "--job") {
        let csrf =
            login(&jar, &user, &pass).ok_or("login void — the job stays unbuilt (0 honored)")?;
        let prefix = match arg_value(&args, "--prefix") {
            Some(p) => p,
            None => String::new(),
        };
        let data_file = arg_value(&args, "--data")
            .ok_or("--data <file> absent — the collected_data JSON stays unread".to_string())?;
        let data_json = fs::read_to_string(&data_file)
            .map_err(|e| format!("{data_file}: {e} — the collected_data JSON stays unread"))?;
        let body = job_body(&format, data_json.trim());
        println!("{}", post_command(&jar, &csrf, &prefix, &body));
        return Ok(());
    }
    if let Some(body) = arg_value(&args, "--request") {
        let csrf = login(&jar, &user, &pass)
            .ok_or("login void — the request stays unbuilt (0 honored)")?;
        let prefix = match arg_value(&args, "--prefix") {
            Some(p) => p,
            None => String::new(),
        };
        println!("{}", post_command(&jar, &csrf, &prefix, &body));
        return Ok(());
    }
    if let Some(body) = arg_value(&args, "--submit") {
        let csrf = login(&jar, &user, &pass)
            .ok_or("login void — the submit stays unexecuted (0 honored)")?;
        let prefix = match arg_value(&args, "--prefix") {
            Some(p) => p,
            None => String::new(),
        };
        match submit_request(&jar, &csrf, &prefix, &body) {
            Some(resp) => {
                println!("{resp}");
                return Ok(());
            }
            None => return Err("submit void — the request stayed uncreated".into()),
        }
    }
    if let Some(src) = arg_value(&args, "--compile") {
        let out = arg_value(&args, "--out")
            .ok_or("--out <file.bin> absent — the output path is never silent")?;
        let lsk_path = arg_value(&args, "--lsk")
            .ok_or("--lsk <naif0012.tls> absent — the TDB clock stays unread")?;
        let lsk_text = fs::read_to_string(&lsk_path)
            .map_err(|e| format!("{lsk_path}: {e} — the TDB clock stays unread"))?;
        let lsk = parse_lsk(&lsk_text).ok_or(format!("{lsk_path}: leap table parses void"))?;
        let ci = args.iter().any(|a| a == "--ci-mode");
        return compile(Path::new(&src), &out, &lsk, ci);
    }
    usage();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("kcdc_compiler: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_encode_keeps_unreserved_and_encodes_the_rest() {
        assert_eq!(form_encode("omegaflow"), "omegaflow");
        assert_eq!(form_encode("a b"), "a%20b");
        assert_eq!(form_encode("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn job_body_matches_the_measured_datashop_post() {
        assert_eq!(
            job_body("ascii", "{\"array\":{}}"),
            "out_format=ascii&data=%7B%22array%22%3A%7B%7D%7D"
        );
        assert_eq!(job_body("root", "{}"), "out_format=root&data=%7B%7D");
    }

    #[test]
    fn compile_joins_component_rows_through_the_registry_stems() {
        let dir = std::env::temp_dir().join(format!("kcdc_compile_test_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("row_mapping.txt"),
            "calorimeter\tgrande\tgeneral\tarray\tlopes\n-1\t-1\t0\t0\t-1\n",
        )
        .unwrap();
        fs::write(
            dir.join("general.txt"),
            "R Ev Gt T P\n877 1001 894645350 12.5 998.0\n",
        )
        .unwrap();
        fs::write(
            dir.join("array.txt"),
            "R Ev E Xc Ze\n877 1001 14.5 -12.5 33.1\n",
        )
        .unwrap();
        let out = dir.join("series.bin");
        let lsk = LeapSeconds {
            delta_t_a: 0.0,
            deltas: vec![(0.0, 0.0)],
        };
        let result = compile(&dir, out.to_str().unwrap(), &lsk, false);
        assert!(result.is_ok(), "compile returned {result:?}");
        let bytes = fs::read(&out).unwrap();
        let parsed = parse_bin(&bytes).expect("the bin roundtrips");
        assert!(
            parsed.len() >= 5,
            "general T+P and array E+Xc+Ze = 5 records, parsed {}",
            parsed.len()
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn jar_value_reads_plain_and_httponly_lines() {
        let dir = std::env::temp_dir().join(format!("kcdc_jar_test_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let jar = dir.join("cookies.txt");
        fs::write(
            &jar,
            "#HttpOnly_.kcdc.iap.kit.edu\tTRUE\t/\tFALSE\t0\tsessionid\tabc123\n\
             .kcdc.iap.kit.edu\tTRUE\t/\tFALSE\t0\tcsrftoken\txyz789\n",
        )
        .unwrap();
        assert_eq!(jar_value(&jar, "sessionid").as_deref(), Some("abc123"));
        assert_eq!(jar_value(&jar, "csrftoken").as_deref(), Some("xyz789"));
        assert_eq!(jar_value(&jar, "absent"), None);
        let _ = fs::remove_dir_all(&dir);
    }
}
