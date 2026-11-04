use omegaflow::archivar::hapi_csv::{HapiParam, write_bin};
use omegaflow::archivar::json::{JsonVal, jpath_val, parse_json, scalar_of};
use omegaflow::cdn::upload_release;

fn fetch_text(url: &str) -> Option<String> {
    let out = std::process::Command::new("curl")
        .arg("-sSL")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_all(args: &[String], name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == name {
            if let Some(v) = args.get(i + 1) {
                out.push(v.clone());
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

fn resolve(url: &str, now: f64) -> String {
    let day = 86_400.0;
    let iso = |t: f64| {
        let secs = t.floor() as i64;
        let days = secs.div_euclid(86_400);
        let rem = secs.rem_euclid(86_400);
        let (y, m, d) = civil_from_days(days);
        format!(
            "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}",
            rem / 3600,
            (rem % 3600) / 60,
            rem % 60
        )
    };
    url.replace("{now}", &iso(now))
        .replace(
            "{hour_ago}",
            &iso(now - 3600.0).split('T').next().unwrap_or(""),
        )
        .replace(
            "{week_ago}",
            &iso(now - 7.0 * day).split('T').next().unwrap_or(""),
        )
        .replace("{today}", &iso(now).split('T').next().unwrap_or(""))
        .replace(
            "{tomorrow}",
            &iso(now + day).split('T').next().unwrap_or(""),
        )
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

struct Column {
    index: usize,
    sub: Option<usize>,
    fill: Option<f64>,
}

fn columns(mapping: &[String], params: &[HapiParam]) -> Vec<Column> {
    let mut out = Vec::new();
    for m in mapping {
        let Some((left, _key)) = m.split_once('=') else {
            continue;
        };
        let (name, sub) = match left.split_once('.') {
            Some((n, i)) => (n, i.parse::<usize>().ok()),
            None => (left, None),
        };
        let Some(index) = params.iter().position(|p| p.name == name) else {
            continue;
        };
        let fill = params.get(index).and_then(|p| p.fill);
        out.push(Column { index, sub, fill });
    }
    out
}

fn params_of(json: &JsonVal) -> Vec<HapiParam> {
    let Some(JsonVal::Arr(list)) = jpath_val(json, "parameters") else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|p| {
            let name = match jpath_val(p, "name") {
                Some(JsonVal::Str(s)) => s.clone(),
                _ => return None,
            };
            let units = match jpath_val(p, "units") {
                Some(JsonVal::Str(s)) => s.clone(),
                _ => String::new(),
            };
            let fill = jpath_val(p, "fill").and_then(scalar_of);
            Some(HapiParam {
                name,
                units,
                fill,
                positive: false,
            })
        })
        .collect()
}

fn rows(json: &JsonVal, cols: &[Column]) -> Vec<(f64, f64, u32)> {
    let Some(JsonVal::Arr(data)) = jpath_val(json, "data") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for row in data {
        let JsonVal::Arr(cells) = row else { continue };
        let Some(t) = cells.first().and_then(|c| match c {
            JsonVal::Str(s) => parse_time(s),
            other => scalar_of(other),
        }) else {
            continue;
        };
        for (comp, col) in cols.iter().enumerate() {
            let cell = cells.get(col.index);
            let v = match (cell, col.sub) {
                (Some(JsonVal::Arr(a)), Some(i)) => a.get(i).and_then(scalar_of),
                (Some(JsonVal::Arr(a)), None) => a.first().and_then(scalar_of),
                (Some(other), _) => scalar_of(other),
                _ => None,
            };
            let Some(v) = v else {
                continue;
            };
            if !v.is_finite() {
                continue;
            }
            if col.fill.is_some_and(|f| (f - v).abs() < 1e-9) {
                continue;
            }
            out.push((t, v, comp as u32));
        }
    }
    out
}

fn parse_time(s: &str) -> Option<f64> {
    let (d, rest) = s.split_once('T')?;
    let mut it = d.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let mo: i64 = it.next()?.parse().ok()?;
    let da: i64 = it.next()?.parse().ok()?;
    let rest = rest.trim_end_matches('Z');
    let mut p = rest.split(':');
    let h: i64 = p.next()?.parse().ok()?;
    let mi: i64 = p.next()?.parse().ok()?;
    let se: i64 = p.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let days = omegaflow::lsk::days_from_civil(y, mo, da)?;
    Some(days as f64 * 86400.0 + (h * 3600 + mi * 60 + se) as f64)
}

fn selftest() {
    let body = r#"{"Hapi":"3.0","status":{"code":200},"parameters":[{"name":"Time","units":"UTC"},{"name":"Bx","units":"nT","fill":"99999.0"},{"name":"By","units":"nT"}],"data":[["2024-01-01T00:00:00Z",1.5,99999.0],["2024-01-01T00:00:01Z",2.5,3.5],["2024-01-01T00:00:02Z",99999.0,4.5]]}"#;
    let json = parse_json(body).expect("hapi json");
    let params = params_of(&json);
    assert_eq!(params.len(), 3);
    assert_eq!(params[1].fill, Some(99999.0));
    let cols = columns(&["Bx=bx_nt".to_string(), "By=by_nt".to_string()], &params);
    assert_eq!(cols.len(), 2);
    let recs = rows(&json, &cols);
    assert_eq!(
        recs.len(),
        5,
        "the Bx fill value is skipped, By has no fill"
    );
    assert_eq!(recs[0], (1.7040672e9, 1.5, 0u32));
    let bin = write_bin(2, &recs);
    assert_eq!(bin.len(), 12 + recs.len() * 20);
    println!("hapi_emit selftest: fill skipped, {} records", recs.len());
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    omegaflow::archivar::cdn::require_ci();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let url = match arg_value(&args, "--url") {
        Some(u) => u,
        None => {
            eprintln!("hapi_emit: --url absent");
            std::process::exit(2);
        }
    };
    let out = arg_value(&args, "--out").unwrap_or_else(|| "hapi.bin".to_string());
    let netloc = arg_value(&args, "--netloc").unwrap_or_default();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);
    let route = resolve(&url, now);
    let Some(body) = fetch_text(&route) else {
        eprintln!("hapi_emit: fetch void ({route})");
        std::process::exit(1);
    };
    let text = body.as_str();
    if text.contains("1201") && text.contains("no data for time range") {
        eprintln!(
            "hapi_emit: the answer names no data for this range — the bin stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    let Some(json) = parse_json(text) else {
        eprintln!("hapi_emit: body is not JSON");
        std::process::exit(1);
    };
    let mut params = params_of(&json);
    if params.is_empty() {
        let id = url
            .split("id=")
            .nth(1)
            .and_then(|s| s.split('&').next())
            .unwrap_or("");
        let base = url.split("/hapi/").next().unwrap_or("");
        let info = format!("{base}/hapi/info?id={id}");
        if let Some(body) = fetch_text(&info) {
            if let Some(j) = parse_json(&body) {
                params = params_of(&j);
            }
        }
    }
    if params.is_empty() {
        eprintln!("hapi_emit: no parameters in the answer or the info record");
        std::process::exit(1);
    }
    let cols = columns(&arg_all(&args, "--map"), &params);
    if cols.is_empty() {
        eprintln!("hapi_emit: --map empty or names not in the answer");
        std::process::exit(1);
    }
    let recs = rows(&json, &cols);
    if recs.is_empty() {
        eprintln!("hapi_emit: the answer carries no sample — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(cols.len() as u32, &recs);
    std::fs::write(&out, &bytes).unwrap_or_else(|e| {
        eprintln!("hapi_emit: write {out}: {e}");
        std::process::exit(1);
    });
    eprintln!(
        "hapi_emit: {} values, {} channels, {} bytes",
        recs.len(),
        cols.len(),
        bytes.len()
    );
    if ci_mode && !netloc.is_empty() {
        upload_release(&netloc, &out);
    }
}
