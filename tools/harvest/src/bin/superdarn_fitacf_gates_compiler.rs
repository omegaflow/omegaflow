use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::lsk::days_from_civil;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "sdc-serv.usask.ca";
const CALL: &str = "https://sdc-serv.usask.ca/ascii-call";
const FIELDS: &str = "[\"v\",\"p_l\",\"w_l\",\"gflg\",\"tfreq\"]";
const DEFAULT_BEAM: i64 = 0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_get(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "superdarn_fitacf_gates_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn curl_post(url: &str, body: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("300")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("--data")
        .arg(body)
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!(
            "superdarn_fitacf_gates_compiler: {url} returned ({}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn stamp(unix: i64) -> (String, String) {
    let (y, mo, d) = civil_from_days(unix.div_euclid(86400));
    let rem = unix.rem_euclid(86400);
    (
        format!("{y:04}-{mo:02}-{d:02}"),
        format!("{:02}:{:02}", rem / 3600, (rem % 3600) / 60),
    )
}

fn text<'a>(root: &'a JsonVal, key: &str) -> Option<&'a str> {
    match root {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Str(s)) => Some(s.as_str()),
            _ => None,
        },
        _ => None,
    }
}

fn number(root: &JsonVal, key: &str) -> Option<i64> {
    text(root, key)?.trim().parse().ok()
}

fn floats(s: &str) -> Option<Vec<f64>> {
    let inner = s.trim().strip_prefix('[')?.strip_suffix(']')?;
    let mut out = Vec::new();
    for token in inner.split_whitespace() {
        let v: f64 = token.parse().ok()?;
        if !v.is_finite() {
            return None;
        }
        out.push(v);
    }
    Some(out)
}

fn ints(s: &str) -> Option<Vec<i64>> {
    let inner = s.trim().strip_prefix('[')?.strip_suffix(']')?;
    let mut out = Vec::new();
    for token in inner.split_whitespace() {
        out.push(token.parse().ok()?);
    }
    Some(out)
}

struct Row {
    unix: i64,
    radar: String,
    beam: i64,
    gate: i64,
    velocity: f64,
    power: f64,
    width: f64,
    gflg: i64,
    tfreq: i64,
}

fn record_rows(record: &JsonVal, radar: &str) -> Option<Vec<Row>> {
    let year = number(record, "time.yr")?;
    let month = number(record, "time.mo")?;
    let day = number(record, "time.dy")?;
    let hour = number(record, "time.hr")?;
    let minute = number(record, "time.mt")?;
    let second = number(record, "time.sc")?;
    let days = days_from_civil(year, month, day)?;
    let unix = days * 86400 + hour * 3600 + minute * 60 + second;
    let beam = number(record, "bmnum")?;
    let tfreq = number(record, "tfreq")?;
    let gates = ints(text(record, "slist")?)?;
    let velocity = floats(text(record, "v")?)?;
    let power = floats(text(record, "p_l")?)?;
    let width = floats(text(record, "w_l")?)?;
    let gflg = ints(text(record, "gflg")?)?;
    let n = gates.len();
    if velocity.len() != n || power.len() != n || width.len() != n || gflg.len() != n {
        return None;
    }
    let mut rows = Vec::new();
    for i in 0..n {
        rows.push(Row {
            unix,
            radar: radar.to_string(),
            beam,
            gate: gates[i],
            velocity: velocity[i],
            power: power[i],
            width: width[i],
            gflg: gflg[i],
            tfreq,
        });
    }
    Some(rows)
}

fn gate_rows(root: &JsonVal, radar: &str) -> Option<Vec<Row>> {
    let JsonVal::Arr(records) = root else {
        return None;
    };
    let mut out = Vec::new();
    for record in records {
        if let Some(rows) = record_rows(record, radar) {
            out.extend(rows);
        }
    }
    out.sort_by(|a, b| {
        (a.unix, a.radar.as_str(), a.beam, a.gate).cmp(&(b.unix, b.radar.as_str(), b.beam, b.gate))
    });
    Some(out)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let radar = match arg_value(args, "--radar") {
        Some(v) => v,
        None => "sas".to_string(),
    };
    let beam = match arg_value(args, "--beam") {
        Some(v) => v
            .parse::<i64>()
            .map_err(|_| format!("--beam {v} is not an integer"))?,
        None => DEFAULT_BEAM,
    };
    if !radar
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        || radar.is_empty()
    {
        return Err(format!("--radar {radar} is not a plain radar code"));
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system clock returned {e}"))?
        .as_secs() as i64;
    let default_from = now - 5400;
    let (start_date, start_time) = match arg_value(args, "--start") {
        Some(v) => parse_start(&v)?,
        None => stamp(default_from),
    };
    let (end_date, end_time) = match arg_value(args, "--end") {
        Some(v) => parse_start(&v)?,
        None => stamp(now),
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/superdarn_{radar}.txt"),
    };

    let request = format!(
        "{{\"radar\":\"{radar}\",\"startDate\":\"{start_date}\",\"startTime\":\"{start_time}\",\"endDate\":\"{end_date}\",\"endTime\":\"{end_time}\",\"beam\":\"{beam}\",\"ftype\":\"json\",\"fields\":{FIELDS}}}"
    );
    let reply = curl_post(CALL, &request).ok_or_else(|| format!("{CALL}: fetch void"))?;
    let path = reply.trim();
    if path.is_empty() {
        return Err(format!(
            "{CALL}: empty body for {radar} {start_date} {start_time} - {end_date} {end_time}"
        ));
    }
    if path.starts_with('<') {
        return Err(format!("{CALL}: body is HTML, not a conversion path"));
    }
    if !path.starts_with('/') || !path.contains("/ascii_files/") || !path.ends_with(".json") {
        return Err(format!("{CALL}: unexpected conversion reply {path}"));
    }
    let url = format!("https://{NETLOC}{path}");

    let body = curl_get(&url).ok_or_else(|| format!("{url}: fetch void"))?;
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Err(format!("{url}: empty body"));
    }
    if trimmed.starts_with('<') {
        return Err(format!("{url}: body is HTML, not JSON"));
    }
    let root = parse_json(&body).ok_or_else(|| format!("{url}: response parse void"))?;
    let Some(rows) = gate_rows(&root, &radar) else {
        return Err(format!("{url}: response carries no fitacf record array"));
    };
    if rows.is_empty() {
        return Err(format!(
            "{url}: no gate carries a finite velocity, power and width — 0 honored"
        ));
    }

    let mut text = format!(
        "# superdarn fitacf | unix radar beam gate velocity_m_s power_db width_m_s gflg tfreq_khz | v velocity from ACF fit, p_l SNR from lambda fit, w_l spectral width from lambda fit, gflg 1 ground scatter 0 other | source {CALL} radar {radar} beam {beam} {start_date}T{start_time}Z {end_date}T{end_time}Z | file {url}\n"
    );
    for r in &rows {
        text.push_str(&format!(
            "{} {} {} {} {} {} {} {} {}\n",
            r.unix, r.radar, r.beam, r.gate, r.velocity, r.power, r.width, r.gflg, r.tfreq
        ));
    }

    if let Some(parent) = Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, text.as_bytes()).map_err(|e| format!("write {out} returned {e}"))?;

    eprintln!(
        "superdarn_fitacf_gates_compiler: {} gate rows written to {out} ({} B)",
        rows.len(),
        text.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn parse_start(v: &str) -> Result<(String, String), String> {
    let text = v.trim();
    let (date, clock) = text
        .split_once('T')
        .ok_or_else(|| format!("{v} is not YYYY-MM-DDTHH:MM"))?;
    let mut d = date.split('-');
    let year = d.next().ok_or_else(|| format!("{v}: missing year"))?;
    let month = d.next().ok_or_else(|| format!("{v}: missing month"))?;
    let day = d.next().ok_or_else(|| format!("{v}: missing day"))?;
    let mut t = clock.split(':');
    let hour = t.next().ok_or_else(|| format!("{v}: missing hour"))?;
    let minute = t.next().ok_or_else(|| format!("{v}: missing minute"))?;
    let y: i64 = year.parse().map_err(|_| format!("{v}: bad year"))?;
    let mo: i64 = month.parse().map_err(|_| format!("{v}: bad month"))?;
    let dd: i64 = day.parse().map_err(|_| format!("{v}: bad day"))?;
    if days_from_civil(y, mo, dd).is_none() {
        return Err(format!("{v}: not a civil date"));
    }
    let h: i64 = hour.parse().map_err(|_| format!("{v}: bad hour"))?;
    let mi: i64 = minute.parse().map_err(|_| format!("{v}: bad minute"))?;
    if !(0..24).contains(&h) || !(0..60).contains(&mi) {
        return Err(format!("{v}: clock outside 00:00-23:59"));
    }
    Ok((date.to_string(), format!("{h:02}:{mi:02}")))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("superdarn_fitacf_gates_compiler: {msg}");
        std::process::exit(2);
    }
}
