use omegaflow::cdn::upload_asset;
use omegaflow::json::{JsonVal, parse_json};
use std::io::Write;
use std::process::Command;

fn tap_query(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=json")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_query http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn tap_query_votable(root: &str, adql: &str, td: bool) -> Option<String> {
    let format = if td { "votable/td" } else { "votable" };
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg(format!("FORMAT={}", format))
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_query_votable http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn tap_query_csv(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("300")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_query_csv http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn csv_line(line: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_q {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_q = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_q = true;
        } else if c == ',' {
            out.push(std::mem::take(&mut cur));
        } else if c != '\r' {
            cur.push(c);
        }
    }
    out.push(cur);
    Some(out)
}

fn fetch_csv_rows(root: &str, adql: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let body = tap_query_csv(root, adql)?;
    let mut lines = body.split('\n');
    let fields = csv_line(lines.next()?)?;
    let mut rows = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(cells) = csv_line(line) {
            rows.push(cells);
        }
    }
    eprintln!("csv: {} fields, {} rows", fields.len(), rows.len());
    Some((fields, rows))
}

fn tap_query_text(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("300")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=text")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_query_text http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn fetch_text_rows(root: &str, adql: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let body = tap_query_text(root, adql)?;
    let mut lines = body.split('\n');
    let fields: Vec<String> = lines
        .next()?
        .split('|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let mut rows = Vec::new();
    for line in lines {
        let t = line.trim();
        if t.is_empty() || t.starts_with("Number of") {
            continue;
        }
        let cells: Vec<String> = t
            .split('|')
            .map(|s| {
                let c = s.trim().to_string();
                if c == "null" { String::new() } else { c }
            })
            .collect();
        rows.push(cells);
    }
    eprintln!("text: {} fields, {} rows", fields.len(), rows.len());
    Some((fields, rows))
}

fn tap_async(root: &str, adql: &str, poll_secs: u64) -> Option<String> {
    let base = root.replace("/tap/sync", "/tap/async");
    let out = Command::new("curl")
        .arg("-sS")
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
        .arg("FORMAT=votable")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(&base)
        .output()
        .ok()?;
    let headers = String::from_utf8_lossy(&out.stdout);
    let job = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("location:"))
        .map(|l| l["location:".len()..].trim().to_string())?;
    eprintln!("uws job: {}", job);
    let mut phase = String::new();
    for _ in 0..(poll_secs / 10 + 1) {
        phase = Command::new("curl")
            .arg("-sS")
            .arg(format!("{}/phase", job))
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())?
            .trim()
            .to_string();
        if phase == "COMPLETED" {
            break;
        }
        if phase == "ERROR" {
            eprintln!("uws job ERROR");
            return None;
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
    if phase != "COMPLETED" {
        eprintln!("uws job phase: {} after {} s", phase, poll_secs);
        return None;
    }
    Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("3600")
        .arg(format!("{}/results/result", job))
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
}

fn votable_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut fields: Vec<String> = Vec::new();
    for f in body.split("<FIELD").skip(1) {
        let seg = match f.split_once('>') {
            Some((s, _)) => s,
            None => continue,
        };
        let name = if let Some(attr) = seg.split("name=\"").nth(1) {
            attr.split_once('"')?.0.trim().to_string()
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
                "votable: <DATA> absent, felder={} len={}",
                fields.len(),
                body.len()
            );
            return None;
        }
    };
    let mut rows = Vec::new();
    for tr in data.split("<TR>").skip(1) {
        let end = match tr.split_once("</TR>") {
            Some((e, _)) => e,
            None => continue,
        };
        let mut cells = Vec::new();
        for td in end.split("<TD>").skip(1) {
            let raw = match td.split_once("</TD>") {
                Some((r, _)) => r.trim(),
                None => continue,
            };
            let v = if let Some(c) = raw.strip_prefix("<![CDATA[") {
                c.strip_suffix("]]>").unwrap_or(c).trim().to_string()
            } else {
                raw.to_string()
            };
            cells.push(v);
        }
        rows.push(cells);
    }
    eprintln!("votable: {} fields, {} rows", fields.len(), rows.len());
    Some((fields, rows))
}

fn cell_num(c: &JsonVal) -> Option<f64> {
    match c {
        JsonVal::Num(v) => Some(*v),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn fetch_json_rows(root: &str, adql: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let body = tap_query(root, adql)?;
    let parsed = parse_json(&body)?;
    let (meta_opt, data) = match &parsed {
        JsonVal::Obj(m) => (
            m.get("metadata")
                .or_else(|| m.get("columns"))
                .and_then(as_arr),
            m.get("data").and_then(as_arr),
        ),
        JsonVal::Arr(a) => (None, Some(a)),
        _ => return None,
    };
    let mut col_names = Vec::new();
    if let Some(meta) = meta_opt {
        for md in meta {
            if let Some(o) = as_obj(md) {
                if let Some(nm) = get_str(o, "name") {
                    col_names.push(nm);
                }
            }
        }
    } else if let Some(JsonVal::Obj(o)) = data.as_ref().and_then(|d| d.first()) {
        for k in o.keys() {
            col_names.push(k.clone());
        }
    }
    let data = data?;
    let mut rows = Vec::new();
    for row in data {
        let mut cells = Vec::new();
        if let Some(r) = as_arr(row) {
            for c in r {
                cells.push(match c {
                    JsonVal::Str(s) => s.clone(),
                    JsonVal::Num(v) => format!("{}", v),
                    _ => String::new(),
                });
            }
        } else if let Some(o) = as_obj(row) {
            for nm in &col_names {
                cells.push(match o.get(nm) {
                    Some(JsonVal::Str(s)) => s.clone(),
                    Some(JsonVal::Num(v)) => format!("{}", v),
                    _ => String::new(),
                });
            }
        } else {
            continue;
        }
        rows.push(cells);
    }
    Some((col_names, rows))
}

const STAR_BIN_STRIDE: usize = 44;

fn star_record_bytes(cells: &[String], col_idx: &[(String, usize)]) -> Option<Vec<u8>> {
    let get = |k: &str| -> Option<f64> {
        col_idx
            .iter()
            .find(|(key, _)| key == k)
            .and_then(|(_, i)| cells.get(*i))
            .and_then(|s| s.parse::<f64>().ok())
    };
    let ra = get("ra")?;
    let dec = get("dec")?;
    let dist_pc = get("dist_pc")?;
    let mag = get("mag")?;
    let Some(pmra) = get("pmra") else {
        return None;
    };
    let Some(pmdec) = get("pmdec") else {
        return None;
    };
    if !dist_pc.is_finite()
        || !(dist_pc > 0.0)
        || !ra.is_finite()
        || !dec.is_finite()
        || !mag.is_finite()
    {
        return None;
    }
    let ci = match (get("bpmag"), get("rpmag")) {
        (Some(b), Some(r)) if b.is_finite() && r.is_finite() => b - r,
        _ => 0.0,
    };
    let plx_mas = 1000.0 / dist_pc;
    let Some(rv_km_s) = get("rv") else {
        return None;
    };
    let rv = rv_km_s * 1000.0;
    let flux = 10f64.powf(-0.4 * mag) as f32;
    let mut out = Vec::with_capacity(STAR_BIN_STRIDE);
    out.extend_from_slice(&ra.to_le_bytes());
    out.extend_from_slice(&dec.to_le_bytes());
    out.extend_from_slice(&(pmra as f32).to_le_bytes());
    out.extend_from_slice(&(pmdec as f32).to_le_bytes());
    out.extend_from_slice(&(plx_mas as f32).to_le_bytes());
    out.extend_from_slice(&(mag as f32).to_le_bytes());
    out.extend_from_slice(&flux.to_le_bytes());
    out.extend_from_slice(&(ci as f32).to_le_bytes());
    out.extend_from_slice(&(rv as f32).to_le_bytes());
    Some(out)
}

fn lattice_rank(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64) * 0.5
}

fn emit_rows(
    col_idx: &[(String, usize)],
    epoch_prop: Option<f64>,
    cells_rows: &[Vec<String>],
    skip_nulls: &[String],
    lattice: Option<i64>,
    rng: &mut u64,
) -> Vec<String> {
    let i_ra = col_idx.iter().find(|(k, _)| k == "ra").map(|(_, i)| *i);
    let i_dec = col_idx.iter().find(|(k, _)| k == "dec").map(|(_, i)| *i);
    let i_pmra = col_idx.iter().find(|(k, _)| k == "pmra").map(|(_, i)| *i);
    let i_pmdec = col_idx.iter().find(|(k, _)| k == "pmdec").map(|(_, i)| *i);
    let skip_is: Vec<usize> = skip_nulls
        .iter()
        .filter_map(|k| col_idx.iter().find(|(kk, _)| kk == k).map(|(_, i)| *i))
        .collect();
    let mut rows_out = Vec::new();
    for cells in cells_rows {
        let mut ra_v = i_ra
            .and_then(|i| cells.get(i))
            .and_then(|s| s.parse::<f64>().ok());
        let mut dec_v = i_dec
            .and_then(|i| cells.get(i))
            .and_then(|s| s.parse::<f64>().ok());
        if ra_v.is_none() || dec_v.is_none() {
            continue;
        }
        if skip_is
            .iter()
            .any(|i| cells.get(*i).and_then(|s| s.parse::<f64>().ok()).is_none())
        {
            continue;
        }
        if let (Some(epoch), Some(ra), Some(dec), Some(i_pmra), Some(i_pmdec)) =
            (epoch_prop, ra_v, dec_v, i_pmra, i_pmdec)
        {
            if let (Some(pmra), Some(pmdec)) = (
                cells.get(i_pmra).and_then(|s| s.parse::<f64>().ok()),
                cells.get(i_pmdec).and_then(|s| s.parse::<f64>().ok()),
            ) {
                ra_v =
                    Some(ra + pmra / (3.6e6 * dec.to_radians().cos().max(1e-6)) * (2000.0 - epoch));
                dec_v = Some(dec + pmdec / 3.6e6 * (2000.0 - epoch));
            }
        }
        let mut obj = String::from("{");
        let mut pos = 0;
        for (k, i) in col_idx {
            if pos > 0 {
                obj.push(',');
            }
            pos += 1;
            let out = if k == "ra" {
                ra_v.map(|v| format!("{}", v))
                    .unwrap_or_else(|| "null".to_string())
            } else if k == "dec" {
                dec_v
                    .map(|v| format!("{}", v))
                    .unwrap_or_else(|| "null".to_string())
            } else {
                let raw = cells.get(*i).map(|s| s.as_str()).unwrap_or("");
                if raw.is_empty() {
                    "null".to_string()
                } else if raw.parse::<f64>().is_ok() {
                    raw.to_string()
                } else {
                    json_string(raw)
                }
            };
            obj.push_str(&format!("\"{}\":{}", k, out));
        }
        if let (Some(n), Some(ra), Some(dec)) = (lattice, ra_v, dec_v) {
            let dec_idx = (((dec + 90.0) / 180.0 * n as f64).floor() as i64).clamp(0, n - 1);
            let ra_idx = ((ra / 360.0 * n as f64).floor() as i64).clamp(0, n - 1);
            obj.push_str(&format!(
                ",\"cell\":{},\"rank\":{}",
                dec_idx * n + ra_idx,
                lattice_rank(rng)
            ));
        }
        obj.push('}');
        rows_out.push(obj);
    }
    rows_out
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

fn get_str(o: &std::collections::HashMap<String, JsonVal>, k: &str) -> Option<String> {
    match o.get(k) {
        Some(JsonVal::Str(s)) => Some(s.clone()),
        Some(JsonVal::Num(v)) => Some(format!("{}", v)),
        _ => None,
    }
}

fn as_arr(j: &JsonVal) -> Option<&Vec<JsonVal>> {
    match j {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn as_obj(j: &JsonVal) -> Option<&std::collections::HashMap<String, JsonVal>> {
    match j {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

fn dedup_crossmatch(
    cells_rows: Vec<Vec<String>>,
    ra_i: usize,
    dec_i: usize,
    dist_i: Option<usize>,
) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut seen: HashMap<(String, String), usize> = HashMap::new();
    let mut keep: Vec<Vec<String>> = Vec::with_capacity(cells_rows.len());
    for row in cells_rows.into_iter() {
        let key = (
            row.get(ra_i).cloned().unwrap_or_default(),
            row.get(dec_i).cloned().unwrap_or_default(),
        );
        let has_dist = dist_i
            .map(|i2| row.get(i2).map(|s| !s.is_empty()).unwrap_or(false))
            .unwrap_or(false);
        match seen.get(&key) {
            Some(&idx) => {
                let stored_has_dist = dist_i
                    .map(|i2| keep[idx].get(i2).map(|s| !s.is_empty()).unwrap_or(false))
                    .unwrap_or(false);
                if has_dist && !stored_has_dist {
                    keep[idx] = row;
                }
            }
            None => {
                seen.insert(key, keep.len());
                keep.push(row);
            }
        }
    }
    keep
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut root: Option<String> = None;
    let mut out: Option<String> = None;
    let mut table: Option<String> = None;
    let mut columns: Option<String> = None;
    let mut ci_mode = false;
    let mut probe: Option<String> = None;
    let mut limit: usize = 50_000;
    let mut index_mode = false;
    let mut style = String::from("uws");
    let mut async_mode: Option<u64> = None;
    let mut epoch_prop: Option<f64> = None;
    let mut votable_flag = false;
    let mut order_by: Option<String> = None;
    let mut where_clause: Option<String> = None;
    let mut join_spec: Option<(String, String)> = None;
    let mut crossmatch_spec: Option<String> = None;
    let mut crossmatch_z_spec: Option<String> = None;
    let mut crossmatch_pm: Option<String> = None;

    let mut xmatch_radius: f64 = 1.5;
    let mut mag_bands: Option<(f64, f64, f64)> = None;
    let mut band_spec: Option<(String, f64, f64, f64)> = None;
    let mut star_bin = false;
    let mut union_bright: Option<String> = None;
    let mut skip_nulls: Vec<String> = Vec::new();
    let mut cols_unquoted = false;
    let mut votable_td = false;
    let mut csv_flag = false;
    let mut text_flag = false;
    let mut lattice: Option<i64> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                root = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--table" => {
                table = args.get(i + 1).cloned();
                i += 1;
            }
            "--columns" => {
                columns = args.get(i + 1).cloned();
                i += 1;
            }
            "--limit" => {
                limit = args
                    .get(i + 1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(50_000);
                i += 1;
            }
            "--async" => {
                async_mode = args.get(i + 1).and_then(|s| s.parse().ok()).or(Some(600));
                i += 1;
            }
            "--epoch" => {
                epoch_prop = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--votable" => votable_flag = true,
            "--join" => {
                join_spec = Some((
                    args.get(i + 1).cloned().unwrap_or_default(),
                    args.get(i + 2).cloned().unwrap_or_default(),
                ));
                i += 2;
            }
            "--crossmatch" => {
                crossmatch_spec = args.get(i + 1).cloned();
                i += 1;
            }
            "--crossmatch-z" => {
                crossmatch_z_spec = args.get(i + 1).cloned();
                i += 1;
            }
            "--crossmatch-pm" => {
                crossmatch_pm = args.get(i + 1).cloned();
                i += 1;
            }
            "--xmatch-radius" => {
                xmatch_radius = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1.5);
                i += 1;
            }
            "--star-bin" => star_bin = true,
            "--union-bright" => {
                union_bright = args.get(i + 1).cloned();
                i += 1;
            }
            "--mag-bands" => {
                mag_bands = Some((
                    args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    args.get(i + 2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    args.get(i + 3).and_then(|s| s.parse().ok()).unwrap_or(0.1),
                ));
                i += 3;
            }
            "--band" => {
                band_spec = Some((
                    args.get(i + 1).cloned().unwrap_or_default(),
                    args.get(i + 2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    args.get(i + 3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    args.get(i + 4).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                ));
                i += 4;
            }
            "--order" => {
                order_by = args.get(i + 1).cloned();
                i += 1;
            }
            "--where" => {
                where_clause = args.get(i + 1).cloned();
                i += 1;
            }
            "--skip-null" => {
                skip_nulls.push(args.get(i + 1).cloned().unwrap_or_default());
                i += 1;
            }
            "--cols-unquoted" => cols_unquoted = true,
            "--votable-td" => votable_td = true,
            "--csv" => csv_flag = true,
            "--text" => text_flag = true,
            "--lattice" => {
                lattice = args.get(i + 1).and_then(|s| s.parse().ok());
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--probe" => {
                probe = args.get(i + 1).cloned();
                i += 1;
            }
            "--index" => index_mode = true,
            "--style" => {
                style = args
                    .get(i + 1)
                    .cloned()
                    .unwrap_or_else(|| "uws".to_string());
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    let root = match root {
        Some(r) => r,
        None => {
            eprintln!("--root absent");
            std::process::exit(1);
        }
    };
    if lattice.is_some() && (mag_bands.is_some() || band_spec.is_some()) {
        eprintln!("--lattice and band split are separate harvest shapes — refused together");
        std::process::exit(1);
    }
    let mut lattice_rng: u64 = 0x9E37_79B9_7F4A_7C15;
    if index_mode {
        let adql = "SELECT table_name, table_type, schema_name, description FROM tap_schema.tables";
        let mast = style == "mast";
        let fetch = |format: &str| -> Option<String> {
            let mut cmd = Command::new("curl");
            cmd.arg("-sSL").arg("-m").arg("120");
            if mast {
                cmd.arg("-X")
                    .arg("POST")
                    .arg("--data-urlencode")
                    .arg(format!("query={}", adql))
                    .arg("--data-urlencode")
                    .arg(format!("format={}", format));
            } else {
                cmd.arg("-G")
                    .arg("--data-urlencode")
                    .arg("REQUEST=doQuery")
                    .arg("--data-urlencode")
                    .arg("LANG=ADQL")
                    .arg("--data-urlencode")
                    .arg(format!("FORMAT={}", format))
                    .arg("--data-urlencode")
                    .arg(format!("QUERY={}", adql));
            }
            let out = cmd.arg(&root).output().ok()?;
            if !out.status.success() {
                eprintln!(
                    "index http {}: {}",
                    out.status,
                    String::from_utf8_lossy(&out.stderr).trim()
                );
                return None;
            }
            String::from_utf8(out.stdout).ok()
        };
        let preview = |label: &str, body: &str| {
            let flat: String = body
                .chars()
                .filter(|c| !c.is_whitespace())
                .take(240)
                .collect();
            eprintln!("{} body[240]: {}", label, flat);
        };
        let mut triples: Vec<(String, String, String)> = Vec::new();
        if votable_flag {
            let Some(body) = fetch("votable") else {
                eprintln!("index query returned void");
                std::process::exit(1);
            };
            let Some((fields, rows)) = votable_rows(&body) else {
                preview("votable", &body);
                eprintln!("index votable returned void");
                std::process::exit(1);
            };
            let pos = |name: &str| fields.iter().position(|f| f.eq_ignore_ascii_case(name));
            let (Some(tn), Some(tt), Some(sn)) =
                (pos("table_name"), pos("table_type"), pos("schema_name"))
            else {
                eprintln!("index votable columns absent: {:?}", fields);
                std::process::exit(1);
            };
            for r in &rows {
                let name = r.get(tn).cloned().unwrap_or_default();
                if name.is_empty() {
                    continue;
                }
                let schema = r.get(sn).cloned().unwrap_or_default();
                let typ = r.get(tt).cloned().unwrap_or_default();
                triples.push((name, schema, typ));
            }
        } else {
            let Some(body) = fetch("json") else {
                eprintln!("index query returned void");
                std::process::exit(1);
            };
            let parsed = match parse_json(&body) {
                Some(p) => p,
                None => {
                    preview("json", &body);
                    let dump = "/tmp/opencode/tap_index_body.txt";
                    if let Ok(mut f) = std::fs::File::create(dump) {
                        let _ = f.write_all(body.as_bytes());
                    }
                    eprintln!(
                        "index json returned void, body → {} ({} B)",
                        dump,
                        body.len()
                    );
                    std::process::exit(1);
                }
            };
            let cols = match &parsed {
                JsonVal::Obj(m) => m.get("data").and_then(as_arr),
                JsonVal::Arr(a) => Some(a),
                _ => None,
            };
            let Some(cols) = cols else {
                preview("json-data", &body);
                eprintln!("index data absent");
                std::process::exit(1);
            };
            for row in cols {
                let (name, schema, typ) = if let Some(r) = as_arr(row) {
                    if r.len() < 4 {
                        continue;
                    }
                    (row_str(&r[0]), row_str(&r[2]), row_str(&r[1]))
                } else if let Some(o) = as_obj(row) {
                    (
                        o.get("table_name").map(row_str).unwrap_or_default(),
                        o.get("schema_name").map(row_str).unwrap_or_default(),
                        o.get("table_type").map(row_str).unwrap_or_default(),
                    )
                } else {
                    continue;
                };
                triples.push((name, schema, typ));
            }
        }
        let out_path = out.unwrap_or_else(|| "tap_index.φ".to_string());
        let mut buf = String::new();
        buf.push_str(&format!("# tap-inventar {}\n", root));
        for (name, schema, typ) in &triples {
            buf.push_str(&format!("catalog {} {} {}\n", name, schema, typ));
        }
        match std::fs::File::create(&out_path) {
            Ok(mut f) => {
                if let Err(err) = f.write_all(buf.as_bytes()) {
                    eprintln!("write {}: {}", out_path, err);
                    std::process::exit(1);
                }
            }
            Err(_) => {
                eprintln!("write {} returned void", out_path);
                std::process::exit(1);
            }
        }
        eprintln!("tap index: {} tables → {}", triples.len(), out_path);
        return;
    }
    let table = match table {
        Some(t) => t,
        None => {
            eprintln!("--table absent");
            std::process::exit(1);
        }
    };
    let columns = match columns {
        Some(c) => c,
        None => {
            eprintln!("--columns absent (mapping: name:ColA;ra:ColB;...)");
            std::process::exit(1);
        }
    };
    let mut mapping: Vec<(String, String)> = columns
        .split(';')
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let mut it = s.splitn(2, ':');
            Some((it.next()?.trim().to_string(), it.next()?.trim().to_string()))
        })
        .collect();
    if mapping.is_empty() {
        eprintln!("--columns empty");
        std::process::exit(1);
    }
    let mut col_names: Vec<String> = Vec::new();
    let col_idx: Vec<(String, usize)>;
    let any_positional = mapping.iter().any(|(_, c)| c.starts_with('@'));
    let table_ref = if table.contains('/') {
        format!("\"{}\"", table)
    } else {
        table.clone()
    };
    let xq = |c: &str| -> String { format!("\"{}\"", c) };
    let (cols_sel, from_clause) = match (&crossmatch_spec, &crossmatch_z_spec) {
        (Some(spec), _) => {
            let parts: Vec<&str> = spec.split(':').collect();
            if parts.len() != 4 {
                eprintln!("--crossmatch format: table:ra_col:dec_col:dist_col");
                std::process::exit(1);
            }
            let (xtable, xra, xdec, xdist) = (parts[0], parts[1], parts[2], parts[3]);
            let cat_ra = mapping
                .iter()
                .find(|(k, _)| k == "ra")
                .map(|(_, c)| c.clone());
            let cat_dec = mapping
                .iter()
                .find(|(k, _)| k == "dec")
                .map(|(_, c)| c.clone());
            let (Some(cra), Some(cdec)) = (cat_ra, cat_dec) else {
                eprintln!("--crossmatch needs ra + dec in --columns");
                std::process::exit(1);
            };
            let radius_deg = xmatch_radius / 3600.0;
            let fc = format!(
                "{} AS t LEFT JOIN \"{}\" AS j ON 1=CONTAINS(POINT('ICRS', t.{}, t.{}), CIRCLE('ICRS', j.{}, j.{}, {}))",
                table_ref,
                xtable,
                xq(&cra),
                xq(&cdec),
                xq(xra),
                xq(xdec),
                radius_deg
            );
            let mut cs = if any_positional {
                eprintln!("--crossmatch needs named columns, not @positional");
                std::process::exit(1);
            } else {
                let mut s = mapping
                    .iter()
                    .map(|(_, c)| format!("t.{}", xq(c)))
                    .collect::<Vec<_>>()
                    .join(",");
                s.push_str(&format!(", j.{} AS \"dist_pc\"", xq(xdist)));
                s
            };
            if let Some(pm) = &crossmatch_pm {
                let parts: Vec<&str> = pm.split(':').collect();
                for (alias, idx) in [
                    ("pmra", 0usize),
                    ("pmdec", 1),
                    ("plx", 2),
                    ("rv", 3),
                    ("gaia_teff", 4),
                    ("bpmag", 5),
                    ("rpmag", 6),
                    ("gmag", 7),
                ] {
                    if let Some(pc) = parts.get(idx).filter(|c| !c.is_empty()) {
                        cs.push_str(&format!(", j.{} AS \"{}\"", xq(pc), alias));
                        mapping.push((alias.to_string(), alias.to_string()));
                    }
                }
            }
            mapping.push(("dist_pc".to_string(), "dist_pc".to_string()));
            (cs, fc)
        }
        (None, Some(spec)) => {
            let parts: Vec<&str> = spec.split(':').collect();
            if parts.len() != 4 {
                eprintln!("--crossmatch-z format: table:ra_col:dec_col:z_col");
                std::process::exit(1);
            }
            let (xtable, xra, xdec, xz) = (parts[0], parts[1], parts[2], parts[3]);
            let cat_ra = mapping
                .iter()
                .find(|(k, _)| k == "ra")
                .map(|(_, c)| c.clone());
            let cat_dec = mapping
                .iter()
                .find(|(k, _)| k == "dec")
                .map(|(_, c)| c.clone());
            let (Some(cra), Some(cdec)) = (cat_ra, cat_dec) else {
                eprintln!("--crossmatch-z needs ra + dec in --columns");
                std::process::exit(1);
            };
            let radius_deg = xmatch_radius / 3600.0;
            let fc = format!(
                "{} AS t LEFT JOIN \"{}\" AS j ON 1=CONTAINS(POINT('ICRS', t.{}, t.{}), CIRCLE('ICRS', j.{}, j.{}, {}))",
                table_ref,
                xtable,
                xq(&cra),
                xq(&cdec),
                xq(xra),
                xq(xdec),
                radius_deg
            );
            let cs = if any_positional {
                eprintln!("--crossmatch-z needs named columns, not @positional");
                std::process::exit(1);
            } else {
                let mut s = mapping
                    .iter()
                    .map(|(_, c)| format!("t.{}", xq(c)))
                    .collect::<Vec<_>>()
                    .join(",");
                s.push_str(&format!(", j.{} AS \"z\"", xq(xz)));
                s
            };
            mapping.push(("z".to_string(), "z".to_string()));
            (cs, fc)
        }
        (None, None) => {
            let cs = if any_positional {
                "*".to_string()
            } else if cols_unquoted {
                mapping
                    .iter()
                    .map(|(_, c)| c.clone())
                    .collect::<Vec<_>>()
                    .join(",")
            } else {
                mapping
                    .iter()
                    .map(|(_, c)| xq(c))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let fc = match &join_spec {
                Some((jt, oc)) => {
                    format!("{} AS t JOIN {} AS j ON t.{} = j.{}", table_ref, jt, oc, oc)
                }
                None => table_ref.clone(),
            };
            (cs, fc)
        }
    };
    let mut cells_rows: Vec<Vec<String>>;
    if async_mode.is_some() {
        let mut adql = format!("SELECT TOP {} {} FROM {}", limit, cols_sel, from_clause);
        if let Some(w) = &where_clause {
            adql.push_str(&format!(" WHERE {}", w));
        }
        let Some(poll) = async_mode else {
            unreachable!()
        };
        let Some(body) = tap_async(&root, &adql, poll) else {
            eprintln!("async returned void");
            std::process::exit(1);
        };
        let Some((fields, rows)) = votable_rows(&body) else {
            eprintln!("votable returned void");
            std::process::exit(1);
        };
        col_names = fields;
        cells_rows = rows;
    } else if votable_flag {
        let mut q = format!("SELECT TOP {} {} FROM {}", limit, cols_sel, from_clause);
        if let Some(w) = &where_clause {
            q.push_str(&format!(" WHERE {}", w));
        }
        if let Some(o) = &order_by {
            q.push_str(&format!(" ORDER BY \"{}\"", o));
        }
        let Some(body) = tap_query_votable(&root, &q, votable_td) else {
            eprintln!("query returned void");
            std::process::exit(1);
        };
        match votable_rows(&body) {
            Some((fields, rows)) => {
                col_names = fields;
                cells_rows = rows;
            }
            None => {
                eprintln!(
                    "votable returned void, body[:160] = {}",
                    &body[..body.len().min(160)]
                );
                std::process::exit(1);
            }
        }
    } else if mag_bands.is_none() && band_spec.is_none() {
        let mut q = format!("SELECT TOP {} {} FROM {}", limit, cols_sel, from_clause);
        if let Some(w) = &where_clause {
            q.push_str(&format!(" WHERE {}", w));
        }
        if let Some(o) = &order_by {
            q.push_str(&format!(" ORDER BY \"{}\"", o));
        }
        let fetched = if csv_flag {
            fetch_csv_rows(&root, &q)
        } else if text_flag {
            fetch_text_rows(&root, &q)
        } else {
            fetch_json_rows(&root, &q)
        };
        match fetched {
            Some((names, rows)) => {
                col_names = names;
                cells_rows = rows;
            }
            None => {
                eprintln!("query returned void");
                std::process::exit(1);
            }
        }
    } else {
        let band_col = band_spec
            .as_ref()
            .map(|(c, _, _, _)| c.clone())
            .or_else(|| {
                mapping
                    .iter()
                    .find(|(k, _)| k == "mag")
                    .map(|(_, c)| c.clone())
            });
        let band_bounds = band_spec
            .as_ref()
            .map(|(_, lo, hi, step)| (*lo, *hi, *step))
            .or(mag_bands);
        let is_xm = crossmatch_spec.is_some() || crossmatch_z_spec.is_some();
        let band_qual = |c: &str| -> String { if is_xm { format!("t.{}", xq(c)) } else { xq(c) } };
        let left_from = if is_xm {
            format!("{} AS t", table_ref)
        } else {
            table_ref.clone()
        };
        let mut ranges: Vec<(f64, f64)> = match (&band_spec, band_bounds) {
            (Some(_), Some((lo, hi, step))) => {
                let mut out = Vec::new();
                let mut a = lo;
                while a < hi {
                    let b = (a + step).min(hi);
                    out.push((a, b));
                    a = b;
                }
                out
            }
            (None, Some((lo, hi, step))) => {
                let mut queue = vec![(lo, hi)];
                let mut out = Vec::new();
                while let Some((a, b)) = queue.pop() {
                    if b - a <= step {
                        out.push((a, b));
                        continue;
                    }
                    let w = band_col
                        .as_ref()
                        .map(|c| {
                            format!(
                                " WHERE {} >= {} AND {} < {}",
                                band_qual(c),
                                a,
                                band_qual(c),
                                b
                            )
                        })
                        .unwrap_or_default();
                    let count_adql = format!("SELECT COUNT(*) FROM {} {}", left_from, w);
                    let n = tap_query(&root, &count_adql)
                        .and_then(|body| parse_json(&body))
                        .and_then(|j| {
                            let JsonVal::Obj(m) = j else { return None };
                            let data = m.get("data")?;
                            let JsonVal::Arr(rows) = data else {
                                return None;
                            };
                            match rows.first()? {
                                JsonVal::Arr(a) => a.first().and_then(cell_num),
                                JsonVal::Obj(o) => o
                                    .get("COUNT_ALL")
                                    .or_else(|| o.get("COUNT"))
                                    .and_then(cell_num),
                                _ => None,
                            }
                        });
                    match n {
                        Some(v) if v.is_finite() && (v as usize) <= limit => out.push((a, b)),
                        _ => {
                            let mid = (a + b) / 2.0;
                            queue.push((mid, b));
                            queue.push((a, mid));
                        }
                    }
                }
                out
            }
            (_, None) => vec![(f64::NEG_INFINITY, f64::INFINITY)],
        };
        ranges.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut n_bands = 0usize;
        let mut resumed = 0usize;
        let mut void_bands = 0usize;
        let mut total = 0usize;
        let mut col_idx_band: Option<Vec<(String, usize)>> = None;
        let out_path_band = out.clone().unwrap_or_default();
        let mut out_file: Option<std::fs::File> = None;
        let mut first_row = true;
        for &(a, b) in &ranges {
            let part_path = format!("{}.part{:.4}", out_path_band, a);
            if star_bin && !out_path_band.is_empty() {
                let part = std::fs::read(&part_path).unwrap_or_default();
                if !part.is_empty() && part.len() % STAR_BIN_STRIDE == 0 {
                    if out_file.is_none() {
                        match std::fs::File::create(&out_path_band) {
                            Ok(f) => out_file = Some(f),
                            Err(_) => {
                                eprintln!("write {} returned void", out_path_band);
                                std::process::exit(1);
                            }
                        }
                    }
                    if let Some(f) = out_file.as_mut() {
                        if let Err(err) = f.write_all(&part) {
                            eprintln!("write {}: {}", out_path_band, err);
                            std::process::exit(1);
                        }
                        total += part.len() / STAR_BIN_STRIDE;
                        resumed += 1;
                        eprintln!(
                            "band [{:.4}, {:.4}) resumed: +{} records (total {})",
                            a,
                            b,
                            part.len() / STAR_BIN_STRIDE,
                            total
                        );
                        continue;
                    }
                }
            }
            let w = band_col
                .as_ref()
                .filter(|_| b.is_finite())
                .map(|c| {
                    format!(
                        " WHERE {} >= {} AND {} < {}",
                        band_qual(c),
                        a,
                        band_qual(c),
                        b
                    )
                })
                .unwrap_or_default();
            let mut q = format!("SELECT TOP {} {} FROM {}", limit, cols_sel, from_clause);
            q.push_str(&w);
            if let Some(o) = &order_by {
                q.push_str(&format!(" ORDER BY \"{}\"", o));
            }
            let mut fetched: Option<(Vec<String>, Vec<Vec<String>>)> = None;
            for attempt in 0..3 {
                match fetch_json_rows(&root, &q) {
                    Some(pair) => {
                        fetched = Some(pair);
                        break;
                    }
                    None => {
                        eprintln!(
                            "band [{:.4}, {:.4}) attempt {} returned void",
                            a,
                            b,
                            attempt + 1
                        );
                        if attempt < 2 {
                            std::thread::sleep(std::time::Duration::from_secs(60));
                        }
                    }
                }
            }
            match fetched {
                Some((names, rows)) => {
                    if col_names.is_empty() {
                        col_names = names;
                        let idx_of = |col: &str| -> Option<usize> {
                            if let Some(n) = col.strip_prefix('@') {
                                n.parse::<usize>().ok()
                            } else {
                                let base = col.rsplit('.').next().unwrap_or(col);
                                let sanitized = col.replace('.', "_");
                                col_names
                                    .iter()
                                    .position(|c| c == col || c == base || c == sanitized.as_str())
                            }
                        };
                        col_idx_band = Some(
                            mapping
                                .iter()
                                .filter_map(|(k, c)| idx_of(c).map(|i| (k.clone(), i)))
                                .collect(),
                        );
                        if col_idx_band.as_ref().map(|v| v.len()).unwrap_or(0) != mapping.len() {
                            for (k, c) in &mapping {
                                if idx_of(c).is_none() {
                                    eprintln!(
                                        "column absent: {} ({}), available: {:?}",
                                        c, k, col_names
                                    );
                                }
                            }
                            std::process::exit(1);
                        }
                    }
                    let Some(ci) = col_idx_band.as_ref() else {
                        continue;
                    };
                    if out_file.is_none() {
                        if out_path_band.is_empty() {
                            eprintln!("--out absent");
                            std::process::exit(1);
                        }
                        match std::fs::File::create(&out_path_band) {
                            Ok(f) => {
                                out_file = Some(f);
                            }
                            Err(_) => {
                                eprintln!("write {} returned void", out_path_band);
                                std::process::exit(1);
                            }
                        }
                    }
                    let Some(f) = out_file.as_mut() else {
                        continue;
                    };
                    if star_bin {
                        let mut part_bytes: Vec<u8> = Vec::new();
                        for cells in &rows {
                            if let Some(rec) = star_record_bytes(cells, ci) {
                                part_bytes.extend_from_slice(&rec);
                            }
                        }
                        if let Err(err) = f.write_all(&part_bytes) {
                            eprintln!("write {}: {}", out_path_band, err);
                            std::process::exit(1);
                        }
                        total += part_bytes.len() / STAR_BIN_STRIDE;
                        n_bands += 1;
                        eprintln!(
                            "band [{:.4}, {:.4}): +{} records (total {})",
                            a,
                            b,
                            part_bytes.len() / STAR_BIN_STRIDE,
                            total
                        );
                        if !out_path_band.is_empty() {
                            let tmp = format!("{}.tmp", part_path);
                            match std::fs::write(&tmp, &part_bytes) {
                                Ok(_) => {
                                    if let Err(err) = std::fs::rename(&tmp, &part_path) {
                                        eprintln!("write {}: {}", part_path, err);
                                    }
                                }
                                Err(err) => eprintln!("write {}: {}", tmp, err),
                            }
                        }
                        continue;
                    }
                    if first_row {
                        if let Err(err) = f.write_all(b"[") {
                            eprintln!("write {}: {}", out_path_band, err);
                            std::process::exit(1);
                        }
                    }
                    let emitted = emit_rows(
                        ci,
                        epoch_prop,
                        &rows,
                        &skip_nulls,
                        lattice,
                        &mut lattice_rng,
                    );
                    for r in &emitted {
                        if !first_row {
                            if let Err(err) = f.write_all(b",") {
                                eprintln!("write {}: {}", out_path_band, err);
                                std::process::exit(1);
                            }
                        }
                        if let Err(err) = f.write_all(r.as_bytes()) {
                            eprintln!("write {}: {}", out_path_band, err);
                            std::process::exit(1);
                        }
                        first_row = false;
                    }
                    total += emitted.len();
                    n_bands += 1;
                    eprintln!(
                        "band [{:.2}, {:.2}): +{} (total {})",
                        a,
                        b,
                        emitted.len(),
                        total
                    );
                }
                None => {
                    void_bands += 1;
                    eprintln!(
                        "band [{:.4}, {:.4}) returned void after 3 attempts — parts kept, the field stays incomplete",
                        a, b
                    );
                }
            }
        }
        if let Some(mut f) = out_file.take() {
            if !star_bin {
                if let Err(err) = f.write_all(b"]\n") {
                    eprintln!("write {}: {}", out_path_band, err);
                    std::process::exit(1);
                }
            }
            if star_bin {
                if let Some(up) = &union_bright {
                    if let Ok(text) = std::fs::read_to_string(up) {
                        if let Some(JsonVal::Arr(rows)) = parse_json(&text) {
                            let mut added = 0usize;
                            for r in &rows {
                                if let Some(o) = as_obj(r) {
                                    let get = |k: &str| -> Option<f64> {
                                        match o.get(k) {
                                            Some(JsonVal::Num(v)) => Some(*v),
                                            Some(JsonVal::Str(s)) => s.parse().ok(),
                                            _ => None,
                                        }
                                    };
                                    if let (
                                        Some(ra),
                                        Some(dec),
                                        Some(mag),
                                        Some(dist_pc),
                                        Some(rv),
                                        Some(pmra),
                                        Some(pmdec),
                                    ) = (
                                        get("ra"),
                                        get("dec"),
                                        get("mag"),
                                        get("dist_pc"),
                                        get("rv"),
                                        get("pmra"),
                                        get("pmdec"),
                                    ) {
                                        if dist_pc > 0.0 && ra.is_finite() && dec.is_finite() {
                                            let plx_mas = 1000.0 / dist_pc;
                                            let flux = 10f64.powf(-0.4 * mag) as f32;
                                            let ci = get("bp_rp").unwrap_or(0.0);
                                            let mut rec = Vec::with_capacity(STAR_BIN_STRIDE);
                                            rec.extend_from_slice(&ra.to_le_bytes());
                                            rec.extend_from_slice(&dec.to_le_bytes());
                                            rec.extend_from_slice(&(pmra as f32).to_le_bytes());
                                            rec.extend_from_slice(&(pmdec as f32).to_le_bytes());
                                            rec.extend_from_slice(&(plx_mas as f32).to_le_bytes());
                                            rec.extend_from_slice(&(mag as f32).to_le_bytes());
                                            rec.extend_from_slice(&flux.to_le_bytes());
                                            rec.extend_from_slice(&(ci as f32).to_le_bytes());
                                            rec.extend_from_slice(&(rv as f32).to_le_bytes());
                                            if let Err(err) = f.write_all(&rec) {
                                                eprintln!("write {}: {}", out_path_band, err);
                                                std::process::exit(1);
                                            }
                                            added += 1;
                                        }
                                    }
                                }
                            }
                            total += added;
                            eprintln!("union-bright: +{} records (total {})", added, total);
                        }
                    }
                }
            }
        }
        eprintln!(
            "bands: {} ({} resumed, {} void), rows: {} → {}",
            n_bands, resumed, void_bands, total, out_path_band
        );
        if void_bands > 0 {
            eprintln!(
                "{} bands returned void — upload refused, parts kept for resume",
                void_bands
            );
            std::process::exit(1);
        }
        for &(a, _) in &ranges {
            let part_path = format!("{}.part{:.4}", out_path_band, a);
            let _ = std::fs::remove_file(&part_path);
        }
        if ci_mode && !out_path_band.is_empty() && !upload_asset(&out_path_band) {
            eprintln!("upload: {} did not reach the CDN", out_path_band);
            std::process::exit(1);
        }
        return;
    }
    let idx_of = |col: &str| -> Option<usize> {
        if let Some(n) = col.strip_prefix('@') {
            n.parse::<usize>().ok()
        } else {
            let base = col.rsplit('.').next().unwrap_or(col);
            let sanitized = col.replace('.', "_");
            col_names
                .iter()
                .position(|c| c == col || c == base || c == sanitized.as_str())
        }
    };
    col_idx = mapping
        .iter()
        .filter_map(|(k, c)| idx_of(c).map(|i| (k.clone(), i)))
        .collect();
    if col_idx.len() != mapping.len() {
        for (k, c) in &mapping {
            if idx_of(c).is_none() {
                eprintln!("column absent: {} ({}), available: {:?}", c, k, col_names);
            }
        }
        std::process::exit(1);
    }
    let xmatch_col = if crossmatch_spec.is_some() {
        Some("dist_pc")
    } else if crossmatch_z_spec.is_some() {
        Some("z")
    } else {
        None
    };
    if let Some(xc) = xmatch_col {
        let ra_i = col_idx.iter().find(|(k, _)| k == "ra").map(|(_, i)| *i);
        let dec_i = col_idx.iter().find(|(k, _)| k == "dec").map(|(_, i)| *i);
        let dist_i = col_idx.iter().find(|(k, _)| k == xc).map(|(_, i)| *i);
        if let (Some(ri), Some(di)) = (ra_i, dec_i) {
            cells_rows = dedup_crossmatch(cells_rows, ri, di, dist_i);
        }
    }
    let rows_out = emit_rows(
        &col_idx,
        epoch_prop,
        &cells_rows,
        &skip_nulls,
        lattice,
        &mut lattice_rng,
    );
    eprintln!(
        "tap fetch: {} columns, {} rows",
        col_names.len(),
        rows_out.len()
    );
    if let Some(name) = &probe {
        for r in &rows_out {
            if r.contains(&format!("\"name\":\"{}", name)) || r.contains(name) {
                eprintln!("probe: {}", r);
                return;
            }
        }
        eprintln!("probe: {} not present", name);
        return;
    }
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    let mut buf = String::from("[");
    for (k, r) in rows_out.iter().enumerate() {
        if k > 0 {
            buf.push(',');
        }
        buf.push_str(r);
    }
    buf.push_str("]\n");
    match std::fs::File::create(&out_path) {
        Ok(mut f) => {
            if let Err(err) = f.write_all(buf.as_bytes()) {
                eprintln!("write {}: {}", out_path, err);
                std::process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("write {} returned void", out_path);
            std::process::exit(1);
        }
    }
    eprintln!(
        "tap: {} rows → {} ({} B)",
        rows_out.len(),
        out_path,
        buf.len()
    );
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

fn row_str(j: &JsonVal) -> String {
    match j {
        JsonVal::Str(s) => s.trim_matches('"').to_string(),
        JsonVal::Num(v) => format!("{}", v),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::dedup_crossmatch;

    fn row(ra: &str, dec: &str, dist: &str) -> Vec<String> {
        vec![ra.to_string(), dec.to_string(), dist.to_string()]
    }

    #[test]
    fn dedup_keeps_first_row_with_dist() {
        let rows = vec![
            row("10.0", "20.0", ""),
            row("10.0", "20.0", "1238"),
            row("10.0", "20.0", "900"),
        ];
        let out = dedup_crossmatch(rows, 0, 1, Some(2));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0][2], "1238");
    }

    #[test]
    fn dedup_keeps_dist_when_first_row_lacks_it() {
        let rows = vec![
            row("10.0", "20.0", ""),
            row("11.0", "21.0", ""),
            row("11.0", "21.0", "500"),
        ];
        let out = dedup_crossmatch(rows, 0, 1, Some(2));
        assert_eq!(out.len(), 2);
        assert_eq!(out[0][2], "");
        assert_eq!(out[1][2], "500");
    }

    #[test]
    fn dedup_preserves_distinct_positions() {
        let rows = vec![
            row("10.0", "20.0", "100"),
            row("11.0", "21.0", "200"),
            row("12.0", "22.0", "300"),
        ];
        let out = dedup_crossmatch(rows, 0, 1, Some(2));
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn dedup_without_dist_column_keeps_first_only() {
        let rows = vec![
            row("10.0", "20.0", "x"),
            row("10.0", "20.0", "y"),
            row("11.0", "21.0", "z"),
        ];
        let out = dedup_crossmatch(rows, 0, 1, None);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0][2], "x");
    }

    #[test]
    fn dedup_empty_rows_is_empty() {
        let out = dedup_crossmatch(Vec::new(), 0, 1, Some(2));
        assert!(out.is_empty());
    }
}
