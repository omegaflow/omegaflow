use omegaflow::archivar::json::{JsonVal, jnum, jpath_val, json_num, jstr, parse_json, scalar_of};
use omegaflow::cdn::upload_release;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const NETLOC: &str = "gportal.jaxa.jp";
const LANDING_URL: &str = "https://gportal.jaxa.jp/gpr/";
const LOGIN_URL: &str = "https://gportal.jaxa.jp/gpr/auth/authenticate.json";
const QUOTA_URL: &str = "https://gportal.jaxa.jp/gpr/search/service/get_download_limit";
const CATALOG_URL: &str = "https://gportal.jaxa.jp/gpr/search/catalog_records.json";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0";
const SESSION_COOKIE: &str = "iPlanetDirectoryPro";
const CSRF_COOKIE: &str = "fuel_csrf_token";
const CONNECT_BOUND_S: u64 = 1 << 5;
const HTTP_BOUND_S: u64 = 1 << 6;
const CATALOG_BOUND_S: u64 = 1 << 7;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_secret(text: &str, key: &str) -> Option<String> {
    let mut found = None;
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key && !v.trim().is_empty() {
                found = Some(v.trim().to_string());
            }
        }
    }
    found
}

fn credential() -> Option<(String, String)> {
    let user = env::var("JAXA_GPORTAL_USER")
        .ok()
        .filter(|t| !t.trim().is_empty());
    let pass = env::var("JAXA_GPORTAL_PASS")
        .ok()
        .filter(|t| !t.trim().is_empty());
    if let (Some(u), Some(p)) = (user, pass) {
        return Some((u.trim().to_string(), p.trim().to_string()));
    }
    let text = fs::read_to_string(".secrets.local").ok()?;
    let u = parse_secret(&text, "JAXA_GPORTAL_USER")?;
    let p = parse_secret(&text, "JAXA_GPORTAL_PASS")?;
    Some((u, p))
}

fn proxy_env() -> Option<String> {
    env::var("ALL_PROXY").ok().filter(|p| !p.trim().is_empty())
}

fn uri_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn browser_headers() -> Vec<(&'static str, String)> {
    vec![
        ("User-Agent", USER_AGENT.to_string()),
        (
            "Accept",
            "application/json, text/javascript, */*; q=0.01".to_string(),
        ),
        ("Accept-Language", "en-US,en;q=0.9,ja;q=0.8".to_string()),
        ("X-Requested-With", "XMLHttpRequest".to_string()),
        ("Origin", "https://gportal.jaxa.jp".to_string()),
        ("Referer", LANDING_URL.to_string()),
        (
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8".to_string(),
        ),
    ]
}

fn landing_headers() -> Vec<(&'static str, String)> {
    vec![
        ("User-Agent", USER_AGENT.to_string()),
        (
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
        ),
        ("Accept-Language", "en-US,en;q=0.9,ja;q=0.8".to_string()),
        ("Upgrade-Insecure-Requests", "1".to_string()),
    ]
}

struct Jar {
    path: PathBuf,
    dir: PathBuf,
}

impl Jar {
    fn new() -> Option<Jar> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_nanos();
        let dir = env::temp_dir().join(format!("jaxa_gportal_{}_{}", std::process::id(), nanos));
        fs::create_dir_all(&dir).ok()?;
        let path = dir.join("cookies.txt");
        Some(Jar { path, dir })
    }

    fn cookie(&self, name: &str) -> Option<String> {
        let text = fs::read_to_string(&self.path).ok()?;
        let mut found = None;
        for line in text.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() >= 7 && fields[5] == name && !fields[6].is_empty() {
                found = Some(fields[6].to_string());
            }
        }
        found
    }

    fn response_path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }
}

impl Drop for Jar {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn curl_status(
    url: &str,
    body: Option<&str>,
    headers: &[(&str, String)],
    jar: &Jar,
    out_path: &Path,
    bound_s: u64,
) -> Option<u16> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-g")
        .arg("-L")
        .arg("-m")
        .arg(bound_s.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string())
        .arg("-b")
        .arg(&jar.path)
        .arg("-c")
        .arg(&jar.path)
        .arg("-o")
        .arg(out_path)
        .arg("-w")
        .arg("%{http_code}");
    if let Some(b) = body {
        cmd.arg("-X").arg("POST").arg("--data-raw").arg(b);
    }
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    if let Some(proxy) = proxy_env() {
        cmd.env("ALL_PROXY", proxy);
    }
    let output = cmd.arg(url).output().ok()?;
    if !output.status.success() {
        eprintln!(
            "jaxa curl returned ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }
    let code = String::from_utf8_lossy(&output.stdout);
    code.trim().parse::<u16>().ok()
}

fn body_text(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn refresh_token(jar: &Jar) -> Option<String> {
    let landing = jar.response_path("landing.html");
    let code = curl_status(
        LANDING_URL,
        None,
        &landing_headers(),
        jar,
        &landing,
        HTTP_BOUND_S,
    )?;
    if code != 200 {
        eprintln!("jaxa landing returned HTTP {code} — the CSRF token stays absent");
        return None;
    }
    jar.cookie(CSRF_COOKIE)
}

fn login(jar: &Jar) -> bool {
    let Some((user, pass)) = credential() else {
        eprintln!(
            "jaxa_gportal_compiler: JAXA_GPORTAL_USER/JAXA_GPORTAL_PASS absent in the environment and in .secrets.local"
        );
        return false;
    };
    let Some(token) = refresh_token(jar) else {
        eprintln!(
            "jaxa login: no {CSRF_COOKIE} cookie after the landing GET — the WAF gate stands"
        );
        return false;
    };
    let body = format!(
        "account={}&password={}&fuel_csrf_token={}",
        uri_encode(&user),
        uri_encode(&pass),
        uri_encode(&token)
    );
    let out = jar.response_path("login.json");
    let Some(code) = curl_status(
        LOGIN_URL,
        Some(&body),
        &browser_headers(),
        jar,
        &out,
        HTTP_BOUND_S,
    ) else {
        eprintln!("jaxa login: the POST returned void");
        return false;
    };
    if code != 200 {
        eprintln!("jaxa login returned HTTP {code}");
        return false;
    }
    let Some(text) = body_text(&out) else {
        eprintln!("jaxa login: the response body stays unread");
        return false;
    };
    let status = parse_json(&text).and_then(|j| match j {
        JsonVal::Obj(map) => map.get("status").and_then(scalar_of),
        _ => None,
    });
    if status != Some(1.0) {
        eprintln!("jaxa login: status != 1 — the credentials are not accepted");
        return false;
    }
    if jar.cookie(SESSION_COOKIE).is_none() {
        eprintln!("jaxa login: status 1 but no {SESSION_COOKIE} cookie — the session stays absent");
        return false;
    }
    true
}

fn service_post(
    jar: &Jar,
    url: &str,
    extra: &[(String, String)],
    out_name: &str,
    bound_s: u64,
) -> Option<String> {
    let Some(token) = refresh_token(jar) else {
        eprintln!("jaxa {url}: no fresh {CSRF_COOKIE} — the call stays absent");
        return None;
    };
    let mut body = format!("fuel_csrf_token={}", uri_encode(&token));
    for (k, v) in extra {
        body.push('&');
        body.push_str(&format!("{}={}", uri_encode(k), uri_encode(v)));
    }
    let out = jar.response_path(out_name);
    let Some(code) = curl_status(url, Some(&body), &browser_headers(), jar, &out, bound_s) else {
        eprintln!("jaxa {url}: the POST returned void");
        return None;
    };
    if code != 200 {
        eprintln!("jaxa {url} returned HTTP {code}");
        return None;
    }
    body_text(&out)
}

#[derive(Clone, Debug, PartialEq)]
struct Record {
    id: String,
    dataset_id: Option<String>,
    platform: Option<String>,
    instrument: Option<String>,
    begin: Option<String>,
    end: Option<String>,
    lon: f64,
    lat: f64,
    points: usize,
    bbox: [f64; 4],
    size: f64,
    version: Option<String>,
    url: Option<String>,
    ascending_node_lon: Option<f64>,
}

fn collect_pairs(v: &JsonVal, out: &mut Vec<(f64, f64)>) {
    match v {
        JsonVal::Arr(items) => {
            if items.len() == 2
                && let (Some(x), Some(y)) = (json_num(&items[0]), json_num(&items[1]))
            {
                out.push((x, y));
                return;
            }
            for it in items {
                collect_pairs(it, out);
            }
        }
        _ => {}
    }
}

fn footprint(v: &JsonVal) -> Option<(f64, f64, usize, [f64; 4])> {
    let coords = jpath_val(v, "geometry.coordinates")?;
    let mut pairs = Vec::new();
    collect_pairs(coords, &mut pairs);
    if pairs.is_empty() {
        return None;
    }
    let mut min_lon = f64::INFINITY;
    let mut min_lat = f64::INFINITY;
    let mut max_lon = f64::NEG_INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let (mut sum_lon, mut sum_lat) = (0.0f64, 0.0f64);
    let mut n = 0usize;
    for (lon, lat) in &pairs {
        if !(lon.is_finite() && lat.is_finite()) {
            continue;
        }
        if !(-180.0..=360.0).contains(lon) || !(-90.0..=90.0).contains(lat) {
            continue;
        }
        sum_lon += lon;
        sum_lat += lat;
        n += 1;
        if *lon < min_lon {
            min_lon = *lon;
        }
        if *lon > max_lon {
            max_lon = *lon;
        }
        if *lat < min_lat {
            min_lat = *lat;
        }
        if *lat > max_lat {
            max_lat = *lat;
        }
    }
    if n == 0 {
        return None;
    }
    let lon = sum_lon / n as f64;
    let lat = sum_lat / n as f64;
    if !(lon.is_finite() && lat.is_finite()) {
        return None;
    }
    Some((lon, lat, n, [min_lon, min_lat, max_lon, max_lat]))
}

fn one_record(feature: &JsonVal) -> Option<Record> {
    let props = jpath_val(feature, "properties")?;
    let id = jstr(props, "identifier")?;
    if id.is_empty() {
        return None;
    }
    let size_text = jstr(props, "product.size")?;
    let size: f64 = size_text.trim().parse().ok()?;
    if !(size.is_finite() && size > 0.0) {
        return None;
    }
    let (lon, lat, points, bbox) = footprint(feature)?;
    Some(Record {
        id,
        dataset_id: jstr(props, "gpp.datasetId"),
        platform: jstr(props, "platformShortName"),
        instrument: jstr(props, "instrumentShortName"),
        begin: jstr(props, "beginPosition"),
        end: jstr(props, "endPosition"),
        lon,
        lat,
        points,
        bbox,
        size,
        version: jstr(props, "product.version"),
        url: jstr(props, "product.fileName"),
        ascending_node_lon: jstr(props, "ascendingNodeLongitude")
            .and_then(|s| s.trim().parse::<f64>().ok())
            .filter(|v| v.is_finite()),
    })
}

fn parse_manifest(text: &str) -> Option<Vec<(String, f64, f64, f64)>> {
    let json = parse_json(text)?;
    let JsonVal::Arr(records) = jpath_val(&json, "records")? else {
        return None;
    };
    let mut out = Vec::with_capacity(records.len());
    for r in records {
        let id = jstr(r, "id")?;
        let lon = jnum(r, "lon")?;
        let lat = jnum(r, "lat")?;
        let size = jnum(r, "size")?;
        if !(lon.is_finite() && lat.is_finite() && size.is_finite() && size > 0.0) {
            return None;
        }
        out.push((id, lon, lat, size));
    }
    Some(out)
}

struct Catalog {
    matched: Option<u64>,
    returned: Option<u64>,
    records: Vec<Record>,
}

fn parse_catalog(text: &str) -> Option<Catalog> {
    let json = parse_json(text)?;
    let matched = jnum(&json, "properties.numberOfRecordsMatched")
        .filter(|v| v.is_finite() && *v >= 0.0)
        .map(|v| v as u64);
    let returned = jnum(&json, "properties.numberOfRecordsReturned")
        .filter(|v| v.is_finite() && *v >= 0.0)
        .map(|v| v as u64);
    let Some(JsonVal::Arr(features)) = jpath_val(&json, "features") else {
        return Some(Catalog {
            matched,
            returned,
            records: Vec::new(),
        });
    };
    let records = features.iter().filter_map(one_record).collect();
    Some(Catalog {
        matched,
        returned,
        records,
    })
}

fn record_json(r: &Record) -> String {
    let mut out = format!(
        "{{\"id\":{},\"lon\":{},\"lat\":{},\"points\":{},\"bbox\":[{},{},{},{}],\"size\":{}",
        json_str(&r.id),
        r.lon,
        r.lat,
        r.points,
        r.bbox[0],
        r.bbox[1],
        r.bbox[2],
        r.bbox[3],
        r.size
    );
    if let Some(v) = &r.dataset_id {
        out.push_str(&format!(",\"dataset_id\":{}", json_str(v)));
    }
    if let Some(v) = &r.platform {
        out.push_str(&format!(",\"platform\":{}", json_str(v)));
    }
    if let Some(v) = &r.instrument {
        out.push_str(&format!(",\"instrument\":{}", json_str(v)));
    }
    if let Some(v) = &r.begin {
        out.push_str(&format!(",\"begin\":{}", json_str(v)));
    }
    if let Some(v) = &r.end {
        out.push_str(&format!(",\"end\":{}", json_str(v)));
    }
    if let Some(v) = &r.version {
        out.push_str(&format!(",\"version\":{}", json_str(v)));
    }
    if let Some(v) = &r.url {
        out.push_str(&format!(",\"url\":{}", json_str(v)));
    }
    if let Some(v) = r.ascending_node_lon {
        out.push_str(&format!(",\"ascending_node_lon\":{v}"));
    }
    out.push('}');
    out
}

fn manifest_json(dataset: &str, from: &str, to: &str, catalog: &Catalog) -> String {
    let records: Vec<String> = catalog.records.iter().map(record_json).collect();
    format!(
        "{{\"dataset\":{},\"from\":{},\"to\":{},\"numberOfRecordsMatched\":{},\"numberOfRecordsReturned\":{},\"records\":[{}]}}\n",
        json_str(dataset),
        json_str(from),
        json_str(to),
        match catalog.matched {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        },
        match catalog.returned {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        },
        records.join(",")
    )
}

fn normalize_date(s: &str) -> String {
    let d = s.trim();
    if d.len() == 10 && d.as_bytes().get(4) == Some(&b'-') && d.as_bytes().get(7) == Some(&b'-') {
        let mut out = String::with_capacity(10);
        out.push_str(&d[0..4]);
        out.push('/');
        out.push_str(&d[5..7]);
        out.push('/');
        out.push_str(&d[8..10]);
        out
    } else {
        d.to_string()
    }
}

fn catalog_search(
    jar: &Jar,
    dataset: &str,
    from: &str,
    to: &str,
    count: u64,
    out_name: &str,
) -> Option<Catalog> {
    let extra = vec![
        ("dataset[0][id]".to_string(), dataset.to_string()),
        ("obsdate[0][from]".to_string(), normalize_date(from)),
        ("obsdate[0][to]".to_string(), normalize_date(to)),
        ("mapProjection".to_string(), "EQ".to_string()),
        ("count".to_string(), count.to_string()),
    ];
    let text = service_post(jar, CATALOG_URL, &extra, out_name, CATALOG_BOUND_S)?;
    parse_catalog(&text)
}

fn quota_json(text: &str) -> Option<String> {
    let json = parse_json(text)?;
    let mut out: Vec<String> = Vec::new();
    if let JsonVal::Obj(root) = &json
        && let Some(JsonVal::Obj(groups)) = root.get("result")
    {
        for (group, val) in groups {
            let JsonVal::Arr(items) = val else { continue };
            for item in items {
                let sat = jstr(item, "sat_name");
                let sensor = jstr(item, "sensor_name");
                let max_count = jnum(item, "download_max_count");
                let max_size = jnum(item, "download_max_size");
                let (Some(sat), Some(sensor)) = (sat, sensor) else {
                    continue;
                };
                if sat == "JAXA" {
                    continue;
                }
                let mut entry = format!(
                    "{{\"group\":{},\"sat\":{},\"sensor\":{}",
                    json_str(group),
                    json_str(&sat),
                    json_str(&sensor)
                );
                if let Some(v) = max_count {
                    entry.push_str(&format!(",\"max_count\":{v}"));
                }
                if let Some(v) = max_size {
                    entry.push_str(&format!(",\"max_size\":{v}"));
                }
                entry.push('}');
                out.push(entry);
            }
        }
    }
    if out.is_empty() {
        return None;
    }
    Some(format!("[{}]\n", out.join(",")))
}

fn selftest() {
    if uri_encode("a b&c=d") != "a%20b%26c%3Dd" {
        eprintln!("selftest: uri_encode drifted");
        std::process::exit(1);
    }
    if normalize_date("2015-01-02") != "2015/01/02" || normalize_date("2015/01/02") != "2015/01/02"
    {
        eprintln!("selftest: date normalization drifted");
        std::process::exit(1);
    }
    let dir = env::temp_dir().join(format!("jaxa_selftest_{}", std::process::id()));
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("cookies.txt");
    let _ = fs::write(
        &path,
        "# Netscape HTTP Cookie File\n\
.gportal.jaxa.jp\tTRUE\t/\tTRUE\t0\tfuel_csrf_token\tabc123\n\
.gportal.jaxa.jp\tTRUE\t/\tTRUE\t0\tiPlanetDirectoryPro\txyz\n",
    );
    let jar = Jar {
        path,
        dir: dir.clone(),
    };
    if jar.cookie("fuel_csrf_token").as_deref() != Some("abc123") || jar.cookie("absent").is_some()
    {
        eprintln!("selftest: cookie jar parse drifted");
        std::process::exit(1);
    }
    drop(jar);

    let fixture = r#"{"type":"FeatureCollection","features":[
      {"type":"Feature","geometry":{"type":"Polygon","coordinates":[[[10.0,20.0],[12.0,22.0],[10.0,24.0],[10.0,20.0]]]},
       "properties":{"identifier":"PROD_A","beginPosition":"2015-01-01T01:29:32.201Z","endPosition":"2015-01-01T03:02:04.422Z",
       "platformShortName":"GPM","instrumentShortName":"DPR","gpp":{"datasetId":"12001000"},
       "product":{"fileName":"https://gportal.jaxa.jp/download/A.h5","size":"100","version":"05A"}}},
      {"type":"Feature","geometry":null,
       "properties":{"identifier":"PROD_NOPOS","gpp":{"datasetId":"12001000"},
       "product":{"fileName":"https://gportal.jaxa.jp/download/B.h5","size":"200","version":"05A"}}},
      {"type":"Feature","geometry":{"type":"Polygon","coordinates":[[[1.0,2.0],[3.0,4.0]]]},
       "properties":{"identifier":"PROD_NOSIZE","gpp":{"datasetId":"12001000"},
       "product":{"fileName":"https://gportal.jaxa.jp/download/C.h5","size":"0","version":"05A"}}}
    ],"properties":{"numberOfRecordsMatched":3,"numberOfRecordsReturned":3}}"#;
    let catalog = parse_catalog(fixture).expect("the fixture parses");
    if catalog.matched != Some(3) {
        eprintln!("selftest: numberOfRecordsMatched drifted");
        std::process::exit(1);
    }
    if catalog.records.len() != 1 {
        eprintln!(
            "selftest: {} of 3 fixtures survived, 1 expected (missing position and zero size stay absent)",
            catalog.records.len()
        );
        std::process::exit(1);
    }
    let r = &catalog.records[0];
    if r.id != "PROD_A"
        || (r.lon - (10.0 + 12.0 + 10.0 + 10.0) / 4.0).abs() > 1e-9
        || r.size != 100.0
    {
        eprintln!("selftest: the surviving record drifted from the measured fields");
        std::process::exit(1);
    }
    if r.platform.as_deref() != Some("GPM") || r.version.as_deref() != Some("05A") {
        eprintln!("selftest: the surviving record lost its measured fields");
        std::process::exit(1);
    }
    if parse_catalog("{\"result\":\"error\"}")
        .and_then(|c| c.matched)
        .is_some()
    {
        eprintln!("selftest: the error shape carries a matched count");
        std::process::exit(1);
    }
    let manifest = manifest_json("12001000", "2015/01/01", "2015/01/01", &catalog);
    match parse_manifest(&manifest) {
        Some(rt) if rt.len() == 1 && rt[0].0 == "PROD_A" && rt[0].3 == 100.0 => {}
        _ => {
            eprintln!("selftest: the manifest roundtrip drifted");
            std::process::exit(1);
        }
    }
    eprintln!("jaxa_gportal_compiler: selftest passes (login seam + catalog plausibility gate)");
}

fn fetch_quota(jar: &Jar, out_dir: &str) -> bool {
    let Some(text) = service_post(jar, QUOTA_URL, &[], "quota.json", HTTP_BOUND_S) else {
        eprintln!("jaxa quota: the call returned void");
        return false;
    };
    let Some(summary) = quota_json(&text) else {
        eprintln!("jaxa quota: the response carries no sensor limit");
        return false;
    };
    let path = PathBuf::from(out_dir).join("jaxa_gportal_quota.json");
    if fs::write(&path, &summary).is_err() {
        eprintln!("jaxa quota: write {} returned void", path.display());
        return false;
    }
    eprintln!("jaxa quota: {} → {}", path.display(), summary.len());
    true
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let out_dir = match arg_value(&args, "--out") {
        Some(v) => v,
        None => omegaflow::archivar::cache_root()
            .to_string_lossy()
            .into_owned(),
    };
    let manifest = match arg_value(&args, "--manifest") {
        Some(v) => v,
        None => format!(
            "{}/jaxa_gportal_manifest.json",
            out_dir.trim_end_matches('/')
        ),
    };
    let Some(jar) = Jar::new() else {
        eprintln!("jaxa_gportal_compiler: the cookie jar returned void");
        std::process::exit(1);
    };
    if !login(&jar) {
        eprintln!("jaxa_gportal_compiler: the login stands — no manifest is written (0 honored)");
        std::process::exit(1);
    }
    eprintln!("jaxa_gportal_compiler: session open (login status 1)");

    if args.iter().any(|a| a == "--quota") {
        let _ = fetch_quota(&jar, &out_dir);
    }

    let Some(dataset) = arg_value(&args, "--dataset") else {
        eprintln!("jaxa_gportal_compiler: --dataset <id> absent — refused");
        std::process::exit(2);
    };
    let Some(from) = arg_value(&args, "--from") else {
        eprintln!("jaxa_gportal_compiler: --from <YYYY/MM/DD> absent — refused");
        std::process::exit(2);
    };
    let Some(to) = arg_value(&args, "--to") else {
        eprintln!("jaxa_gportal_compiler: --to <YYYY/MM/DD> absent — refused");
        std::process::exit(2);
    };
    let count: u64 = match arg_value(&args, "--count") {
        Some(v) => match v.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("jaxa_gportal_compiler: --count carries no number — refused");
                std::process::exit(2);
            }
        },
        None => 100,
    };

    let Some(catalog) = catalog_search(&jar, &dataset, &from, &to, count, "catalog.json") else {
        eprintln!(
            "jaxa_gportal_compiler: the catalog search returned void — no manifest (0 honored)"
        );
        std::process::exit(1);
    };
    eprintln!(
        "jaxa_gportal_compiler: dataset {dataset} {from}..{to} matched {} returned {} → {} record(s) with position+size",
        match catalog.matched {
            Some(v) => v.to_string(),
            None => "unknown".to_string(),
        },
        match catalog.returned {
            Some(v) => v.to_string(),
            None => "unknown".to_string(),
        },
        catalog.records.len()
    );

    let json = manifest_json(&dataset, &from, &to, &catalog);
    if let Some(parent) = Path::new(&manifest).parent()
        && !parent.as_os_str().is_empty()
        && fs::create_dir_all(parent).is_err()
    {
        eprintln!(
            "jaxa_gportal_compiler: create_dir_all for {} returned void",
            parent.display()
        );
        std::process::exit(1);
    }
    if fs::write(&manifest, &json).is_err() {
        eprintln!("jaxa_gportal_compiler: write {} returned void", manifest);
        std::process::exit(1);
    }
    match parse_manifest(&json) {
        Some(roundtrip)
            if roundtrip.len() == catalog.records.len()
                && roundtrip.first().map(|(id, _, _, _)| id.as_str())
                    == catalog.records.first().map(|r| r.id.as_str()) =>
        {
            eprintln!(
                "jaxa_gportal_compiler: {} {} record(s), {} B, roundtrip parses",
                manifest,
                roundtrip.len(),
                json.len()
            )
        }
        _ => {
            eprintln!(
                "jaxa_gportal_compiler: {} roundtrip parse void — the asset stays unverified",
                manifest
            );
            std::process::exit(1);
        }
    }
    if args.iter().any(|a| a == "--ci-mode") && !upload_release(NETLOC, &manifest) {
        eprintln!("jaxa_gportal_compiler: CDN upload returned void");
        std::process::exit(1);
    }
}
