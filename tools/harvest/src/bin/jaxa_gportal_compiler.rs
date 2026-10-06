use omegaflow::archivar::jaxa_gpm_ku::{
    FORCE_EM, KERNEL_INVERSE_SQUARE, TAU_S, TTL_S, parse_bin, write_bin,
};
use omegaflow::archivar::json::{JsonVal, jnum, jpath_val, json_num, jstr, parse_json, scalar_of};
use omegaflow::archivar::lsk::days_from_civil;
use omegaflow::archivar::{LeapSeconds, embedded_lsk};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::Hdf5File;
use std::collections::HashSet;
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
const CSW_URL: &str = "https://gportal.jaxa.jp/csw/csw";
const CHECK_DL_URL: &str = "https://gportal.jaxa.jp/gpr/search/service/check_dlconfig.json";
const ADD_DOWNLOAD_URL: &str = "https://gportal.jaxa.jp/gpr/search/service/add_download.json";
const SFTP_HOST: &str = "ftp.gportal.jaxa.jp";
const SFTP_PORT: u16 = 2051;
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0";
const SESSION_COOKIE: &str = "iPlanetDirectoryPro";
const CSRF_COOKIE: &str = "fuel_csrf_token";
const CONNECT_BOUND_S: u64 = 1 << 5;
const HTTP_BOUND_S: u64 = 1 << 6;
const CATALOG_BOUND_S: u64 = 1 << 7;

const GPM_KU_FORMAT: &str = "jaxa_gpm_ku";
const GPM_KU_MAGIC: [u8; 2] = [0xCF, 0x86];
const GPM_KU_FIELD_EM: u32 = 0;
const GPM_KU_CODE_MISSING: i64 = -30000;
const GPM_KU_ECHO_POWER: &str = "NS/Receiver/echoPower";
const GPM_KU_DEFAULT_NSCAN: u64 = 32;
const GPM_KU_ASSET: &str = "jaxa_gpm_ku.bin";
const GPM_KU_SC_LAT: &str = "NS/navigation/scLat";
const GPM_KU_SC_LON: &str = "NS/navigation/scLon";
const GPM_KU_SC_ALT: &str = "NS/navigation/scAlt";
const GPM_KU_RANGE_BIN_SIZE: &str = "NS/VertLocate/rangeBinSize";
const GPM_KU_YEAR: &str = "NS/ScanTime/Year";
const GPM_KU_MONTH: &str = "NS/ScanTime/Month";
const GPM_KU_DAY: &str = "NS/ScanTime/DayOfMonth";
const GPM_KU_SECOND_OF_DAY: &str = "NS/ScanTime/SecondOfDay";
const GPM_KU_HOUR: &str = "NS/ScanTime/Hour";
const GPM_KU_MINUTE: &str = "NS/ScanTime/Minute";
const GPM_KU_SECOND: &str = "NS/ScanTime/Second";
const GPM_KU_NRAY: u64 = 49;
const GPM_KU_NBIN: u64 = 260;

fn echo_power_si(raw: f64) -> Option<f64> {
    if raw as i64 == GPM_KU_CODE_MISSING {
        return None;
    }
    let watts = 10f64.powf(raw * 0.01 / 10.0) * 1.0e-3;
    if watts.is_finite() && watts > 0.0 {
        Some(watts)
    } else {
        None
    }
}

fn read_rows(file: &Hdf5File<'_>, dataset: &str, count: u64) -> Option<Vec<f64>> {
    file.read_range(dataset, 0, count).ok()
}

fn unit_vector(lat_deg: f64, lon_deg: f64) -> [f64; 3] {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    let (sl, cl) = lat.sin_cos();
    let (so, co) = lon.sin_cos();
    [cl * co, cl * so, sl]
}

fn scan_epoch_tdb(
    lsk: &LeapSeconds,
    year: f64,
    month: f64,
    day: f64,
    second_of_day: f64,
    hour: f64,
    minute: f64,
    second: f64,
) -> Option<f64> {
    let days = days_from_civil(year as i64, month as i64, day as i64)?;
    let sod = if (0.0..86400.0).contains(&second_of_day) {
        second_of_day
    } else if (0.0..24.0).contains(&hour)
        && (0.0..60.0).contains(&minute)
        && (0.0..61.0).contains(&second)
    {
        hour * 3600.0 + minute * 60.0 + second
    } else {
        return None;
    };
    let unix = days as f64 * 86400.0 + sod;
    lsk.unix_to_tdb(unix)
}

fn gpm_record(pos: [f64; 3], val: f64, epoch: f64, extent: f64, presence: f64) -> [f64; 26] {
    let mut r = [0.0f64; 26];
    r[0] = pos[0];
    r[1] = pos[1];
    r[2] = pos[2];
    r[3] = val;
    r[4] = epoch;
    r[5] = TTL_S;
    r[6] = TAU_S;
    r[7] = extent;
    r[8] = KERNEL_INVERSE_SQUARE;
    r[9] = FORCE_EM;
    r[25] = presence;
    r
}

fn compile_granule(path: &str, nscan: u64, lsk: &LeapSeconds) -> Result<Vec<[f64; 26]>, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {path}: {e}"))?;
    let file =
        Hdf5File::parse(&bytes).map_err(|_| format!("{path}: the granule stays unparsed"))?;
    let sc_lat = read_rows(&file, GPM_KU_SC_LAT, nscan)
        .ok_or_else(|| format!("{GPM_KU_SC_LAT} stays unread"))?;
    let sc_lon = read_rows(&file, GPM_KU_SC_LON, nscan)
        .ok_or_else(|| format!("{GPM_KU_SC_LON} stays unread"))?;
    let sc_alt = read_rows(&file, GPM_KU_SC_ALT, nscan)
        .ok_or_else(|| format!("{GPM_KU_SC_ALT} stays unread"))?;
    let n = sc_lat.len().min(sc_lon.len()).min(sc_alt.len());
    if n == 0 {
        return Err(format!("{path}: no scan carries a navigation vector"));
    }
    let row = GPM_KU_NRAY as usize * GPM_KU_NBIN as usize;
    let echo = read_rows(&file, GPM_KU_ECHO_POWER, n as u64)
        .ok_or_else(|| format!("{GPM_KU_ECHO_POWER} stays unread"))?;
    if echo.len() != n * row {
        return Err(format!(
            "{GPM_KU_ECHO_POWER} carries {} values, {} expected for {n} scan(s)",
            echo.len(),
            n * row
        ));
    }
    let extent = read_rows(&file, GPM_KU_RANGE_BIN_SIZE, n as u64)
        .ok_or_else(|| format!("{GPM_KU_RANGE_BIN_SIZE} stays unread"))?;
    let year = read_rows(&file, GPM_KU_YEAR, n as u64)
        .ok_or_else(|| format!("{GPM_KU_YEAR} stays unread"))?;
    let month = read_rows(&file, GPM_KU_MONTH, n as u64)
        .ok_or_else(|| format!("{GPM_KU_MONTH} stays unread"))?;
    let day = read_rows(&file, GPM_KU_DAY, n as u64)
        .ok_or_else(|| format!("{GPM_KU_DAY} stays unread"))?;
    let sod = read_rows(&file, GPM_KU_SECOND_OF_DAY, n as u64)
        .ok_or_else(|| format!("{GPM_KU_SECOND_OF_DAY} stays unread"))?;
    let hour = read_rows(&file, GPM_KU_HOUR, n as u64)
        .ok_or_else(|| format!("{GPM_KU_HOUR} stays unread"))?;
    let minute = read_rows(&file, GPM_KU_MINUTE, n as u64)
        .ok_or_else(|| format!("{GPM_KU_MINUTE} stays unread"))?;
    let second = read_rows(&file, GPM_KU_SECOND, n as u64)
        .ok_or_else(|| format!("{GPM_KU_SECOND} stays unread"))?;

    let mut records = Vec::with_capacity(n * GPM_KU_NBIN as usize);
    let (mut unplaced, mut unepoched, mut no_extent, mut absent) = (0usize, 0usize, 0usize, 0usize);
    for s in 0..n {
        let (lat, lon, alt) = (sc_lat[s], sc_lon[s], sc_alt[s]);
        if !(lat.is_finite() && lon.is_finite() && alt.is_finite())
            || !(-90.0..=90.0).contains(&lat)
            || !(-360.0..=360.0).contains(&lon)
        {
            unplaced += 1;
            continue;
        }
        let Some(ext) = extent.get(s).copied().filter(|e| e.is_finite() && *e > 0.0) else {
            no_extent += 1;
            continue;
        };
        let Some(epoch) = scan_epoch_tdb(
            lsk, year[s], month[s], day[s], sod[s], hour[s], minute[s], second[s],
        ) else {
            unepoched += 1;
            continue;
        };
        let pos = unit_vector(lat, lon);
        for b in 0..GPM_KU_NBIN as usize {
            let mut sum = 0.0f64;
            let mut count = 0usize;
            for r in 0..GPM_KU_NRAY as usize {
                let idx = s * row + r * GPM_KU_NBIN as usize + b;
                if let Some(w) = echo_power_si(echo[idx]) {
                    sum += w;
                    count += 1;
                }
            }
            if count > 0 {
                records.push(gpm_record(pos, sum / count as f64, epoch, ext, 1.0));
            } else {
                records.push(gpm_record(pos, 0.0, epoch, ext, 0.0));
                absent += 1;
            }
        }
    }
    if records.is_empty() {
        return Err(format!(
            "{path}: no record left the granule — {unplaced} unplaced, {unepoched} outside the leap table, {no_extent} without a bin step"
        ));
    }
    eprintln!(
        "jaxa_gpm_ku {path}: {n} scan(s) x {GPM_KU_NBIN} bin(s), {} record(s), {absent} absent, {unplaced} unplaced, {unepoched} outside the leap table, {no_extent} without a bin step; mean echoPower over {GPM_KU_NRAY} rays",
        records.len()
    );
    Ok(records)
}

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

fn download_url(r: &Record) -> Option<&str> {
    r.app_file_path.as_deref().or(r.url.as_deref())
}

fn fetch_file(jar: &Jar, url: &str, out_path: &Path, max_bytes: Option<u64>) -> Option<(u16, u64)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-g")
        .arg("-L")
        .arg("-m")
        .arg(CATALOG_BOUND_S.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string())
        .arg("-b")
        .arg(&jar.path)
        .arg("-c")
        .arg(&jar.path)
        .arg("-o")
        .arg(out_path)
        .arg("-w")
        .arg("%{http_code} %{size_download}");
    if let Some(n) = max_bytes {
        cmd.arg("-r").arg(format!("0-{}", n.saturating_sub(1)));
    }
    for (k, v) in landing_headers() {
        cmd.arg("-H").arg(format!("{k}: {v}"));
    }
    if let Some(proxy) = proxy_env() {
        cmd.env("ALL_PROXY", proxy);
    }
    let output = cmd.arg(url).output().ok()?;
    if !output.status.success() {
        eprintln!(
            "jaxa fetch: curl returned ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut fields = text.split_whitespace();
    let code = fields.next()?.parse::<u16>().ok()?;
    let size = fields.next()?.parse::<u64>().ok()?;
    Some((code, size))
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
        let detail = body_text(&out)
            .map(|t| t.trim().chars().take(400).collect::<String>())
            .filter(|t| !t.is_empty());
        match detail {
            Some(t) => eprintln!("jaxa {url} returned HTTP {code}: {t}"),
            None => eprintln!("jaxa {url} returned HTTP {code}"),
        }
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
    app_file_path: Option<String>,
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
        app_file_path: jstr(props, "product.appFilePath"),
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
    if let Some(v) = &r.app_file_path {
        out.push_str(&format!(",\"app_url\":{}", json_str(v)));
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
    out_dir: &str,
    dump: bool,
) -> Option<Catalog> {
    let extra = vec![
        ("dataset[0][id]".to_string(), dataset.to_string()),
        ("obsdate[0][from]".to_string(), normalize_date(from)),
        ("obsdate[0][to]".to_string(), normalize_date(to)),
        ("mapProjection".to_string(), "EQ".to_string()),
        ("count".to_string(), count.to_string()),
    ];
    let text = service_post(jar, CATALOG_URL, &extra, out_name, CATALOG_BOUND_S)?;
    if dump {
        let path = PathBuf::from(out_dir).join("jaxa_gportal_catalog_raw.json");
        if fs::write(&path, &text).is_err() {
            eprintln!("jaxa catalog: write {} returned void", path.display());
        }
    }
    parse_catalog(&text)
}

fn iso_day(day: &str, end_of_day: bool) -> String {
    let dashed = normalize_date(day).replace('/', "-");
    if end_of_day {
        format!("{dashed}T23:59:59")
    } else {
        format!("{dashed}T00:00:00")
    }
}

fn csw_search(
    jar: &Jar,
    dataset: &str,
    from: &str,
    to: &str,
    count: u64,
    start_index: u64,
    out_name: &str,
    out_dir: &str,
    dump: bool,
) -> Option<Catalog> {
    let url = format!(
        "{CSW_URL}?service=CSW&version=3.0.0&request=GetRecords&outputFormat=application/json&datasetId={}&startTime={}&endTime={}&count={count}&startIndex={start_index}",
        uri_encode(dataset),
        uri_encode(&iso_day(from, false)),
        uri_encode(&iso_day(to, true)),
    );
    let out = jar.response_path(out_name);
    let code = curl_status(&url, None, &landing_headers(), jar, &out, CATALOG_BOUND_S)?;
    if code != 200 {
        eprintln!("jaxa csw {url} returned HTTP {code}");
        return None;
    }
    let text = body_text(&out)?;
    if dump {
        let path = PathBuf::from(out_dir).join(format!("jaxa_gportal_csw_raw_{start_index}.json"));
        if fs::write(&path, &text).is_err() {
            eprintln!("jaxa csw: write {} returned void", path.display());
        }
    }
    parse_catalog(&text)
}

fn merge_page(merged: &mut Catalog, seen: &mut HashSet<String>, cat: Catalog, page: u64) -> usize {
    if page == 0 {
        merged.matched = cat.matched;
    }
    let mut added = 0usize;
    for record in cat.records {
        if seen.insert(record.id.clone()) {
            merged.records.push(record);
            added += 1;
        }
    }
    merged.returned = Some(merged.records.len() as u64);
    added
}

fn catalog_pages(
    jar: &Jar,
    dataset: &str,
    from: &str,
    to: &str,
    count: u64,
    pages: u64,
    out_dir: &str,
    dump: bool,
    use_csw: bool,
) -> Option<Catalog> {
    let mut merged = Catalog {
        matched: None,
        returned: None,
        records: Vec::new(),
    };
    let mut seen: HashSet<String> = HashSet::new();
    if !use_csw {
        let cat = catalog_search(
            jar,
            dataset,
            from,
            to,
            count,
            "catalog_0.json",
            out_dir,
            dump,
        )?;
        merge_page(&mut merged, &mut seen, cat, 0);
        return Some(merged);
    }
    let mut start_index: u64 = 1;
    for page in 0..pages {
        let out_name = format!("csw_{page}.json");
        let cat = csw_search(
            jar,
            dataset,
            from,
            to,
            count,
            start_index,
            &out_name,
            out_dir,
            dump,
        )?;
        let returned = cat.returned;
        let added = merge_page(&mut merged, &mut seen, cat, page);
        if added == 0 {
            eprintln!(
                "jaxa_gportal_compiler: csw page {page} (startIndex {start_index}) carried no new record — paging stops"
            );
            break;
        }
        match returned {
            Some(0) => break,
            Some(n) => start_index += n,
            None => {
                eprintln!(
                    "jaxa_gportal_compiler: csw page {page} carries no numberOfRecordsReturned — paging stops at startIndex {start_index}"
                );
                break;
            }
        }
        if let Some(matched) = merged.matched
            && merged.records.len() as u64 >= matched
        {
            break;
        }
    }
    Some(merged)
}

fn check_download_config(jar: &Jar, records: &[&Record], out_dir: &str) -> Option<bool> {
    let mut extra: Vec<(String, String)> = Vec::new();
    for (i, r) in records.iter().enumerate() {
        let file = r.app_file_path.as_deref().or(r.url.as_deref())?;
        let dataset = r.dataset_id.as_deref()?;
        extra.push((format!("checkDlList[{i}][granuleId]"), r.id.clone()));
        extra.push((format!("checkDlList[{i}][size]"), format!("{}", r.size)));
        extra.push((format!("checkDlList[{i}][fileName]"), file.to_string()));
        extra.push((format!("checkDlList[{i}][datasetId]"), dataset.to_string()));
    }
    let text = service_post(
        jar,
        CHECK_DL_URL,
        &extra,
        "check_dlconfig.json",
        HTTP_BOUND_S,
    )?;
    let path = PathBuf::from(out_dir).join("jaxa_gportal_check_dlconfig.json");
    if fs::write(&path, &text).is_err() {
        eprintln!(
            "jaxa check_dlconfig: write {} returned void",
            path.display()
        );
        return None;
    }
    let json = parse_json(&text)?;
    let status = jstr(&json, "status")?;
    if status != "SUCCESS" {
        eprintln!(
            "jaxa check_dlconfig: status {status} — the quota gate refuses (response at {})",
            path.display()
        );
        return Some(false);
    }
    Some(true)
}

fn add_download(jar: &Jar, r: &Record, idx: usize, out_dir: &str) -> Option<bool> {
    let file = r.app_file_path.as_deref().or(r.url.as_deref())?;
    let dataset = r.dataset_id.as_deref()?;
    let extra = vec![
        ("datasetId[]".to_string(), dataset.to_string()),
        ("size".to_string(), format!("{}", r.size)),
        ("granuleId".to_string(), r.id.clone()),
    ];
    let out_name = format!("add_download_{idx}.json");
    let text = service_post(jar, ADD_DOWNLOAD_URL, &extra, &out_name, HTTP_BOUND_S)?;
    let path = PathBuf::from(out_dir).join(format!("jaxa_gportal_{out_name}"));
    if fs::write(&path, &text).is_err() {
        eprintln!("jaxa add_download: write {} returned void", path.display());
        return None;
    }
    eprintln!(
        "jaxa add_download: {} | granule {} size {} file_url {}",
        path.display(),
        r.id,
        r.size,
        file
    );
    Some(true)
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
    if let Some(path) = arg_value(&args, "--granule") {
        let nscan = match arg_value(&args, "--nscan") {
            Some(v) => match v.parse::<u64>() {
                Ok(n) if n >= 1 => n,
                _ => {
                    eprintln!("jaxa_gpm_ku: --nscan carries no count >= 1 — refused");
                    std::process::exit(2);
                }
            },
            None => GPM_KU_DEFAULT_NSCAN,
        };
        let Some(lsk) = embedded_lsk() else {
            eprintln!(
                "jaxa_gpm_ku: the embedded naif0012.tls stays unread — the UTC→TDB step is absent"
            );
            std::process::exit(1);
        };
        let records = match compile_granule(&path, nscan, &lsk) {
            Ok(r) => r,
            Err(msg) => {
                eprintln!("jaxa_gpm_ku: {msg}");
                std::process::exit(1);
            }
        };
        let bin = write_bin(&records);
        let out = match arg_value(&args, "--bin") {
            Some(v) => v,
            None => format!("data/{NETLOC}/{GPM_KU_ASSET}"),
        };
        if let Some(parent) = Path::new(&out).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if fs::write(&out, &bin).is_err() {
            eprintln!("jaxa_gpm_ku: write {out} returned void");
            std::process::exit(1);
        }
        let read = match fs::read(&out) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("jaxa_gpm_ku: {out} stays unread — the asset stays unverified");
                std::process::exit(1);
            }
        };
        if read != bin {
            eprintln!("jaxa_gpm_ku: {out}: read-back differs — the asset stays unverified");
            std::process::exit(1);
        }
        let present = match parse_bin(&read) {
            Some(rs) => rs.iter().filter(|r| r[25] == 1.0).count(),
            None => {
                eprintln!(
                    "jaxa_gpm_ku: {out}: roundtrip parse void — the asset stays unverified"
                );
                std::process::exit(1);
            }
        };
        eprintln!(
            "jaxa_gpm_ku: {out}: {} record(s), {present} present, {} B, format {GPM_KU_FORMAT} magic {:02X}{:02X} field em({GPM_KU_FIELD_EM})",
            records.len(),
            bin.len(),
            GPM_KU_MAGIC[0],
            GPM_KU_MAGIC[1]
        );
        println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{GPM_KU_ASSET}");
        println!("format {GPM_KU_FORMAT}");
        println!("origin {CATALOG_URL}");
        println!("ttl {}", TTL_S as u64);
        println!(
            "field jaxa_gpm_ku_echo_power_w jaxa_gpm_ku_echo_power_w inverse-square em W {} 0.0 0.0",
            TTL_S as u64
        );
        if args.iter().any(|a| a == "--ci-mode") && !upload_release(NETLOC, &out) {
            eprintln!("jaxa_gpm_ku: CDN upload returned void");
            std::process::exit(1);
        }
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
    let pages: u64 = match arg_value(&args, "--pages") {
        Some(v) => match v.parse() {
            Ok(n) if n >= 1 => n,
            _ => {
                eprintln!("jaxa_gportal_compiler: --pages carries no page count >= 1 — refused");
                std::process::exit(2);
            }
        },
        None => 1,
    };

    let dump_raw = args.iter().any(|a| a == "--catalog-raw");
    let use_csw = pages > 1 || args.iter().any(|a| a == "--csw");
    let Some(catalog) = catalog_pages(
        &jar, &dataset, &from, &to, count, pages, &out_dir, dump_raw, use_csw,
    ) else {
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
    if args.iter().any(|a| a == "--download") {
        let limit: u64 = match arg_value(&args, "--download-limit") {
            Some(v) => match v.parse() {
                Ok(n) if n >= 1 => n,
                _ => {
                    eprintln!(
                        "jaxa_gportal_compiler: --download-limit carries no count >= 1 — refused"
                    );
                    std::process::exit(2);
                }
            },
            None => 1,
        };
        let mut orderable: Vec<&Record> = Vec::new();
        let mut skipped = 0usize;
        for r in catalog.records.iter().take(limit as usize) {
            if r.dataset_id.is_some() && (r.app_file_path.is_some() || r.url.is_some()) {
                orderable.push(r);
            } else {
                skipped += 1;
            }
        }
        if orderable.is_empty() {
            eprintln!(
                "jaxa_gportal_compiler: no record of the first {limit} carries datasetId+file url — no order ({} skipped, 0 honored)",
                skipped
            );
            std::process::exit(1);
        }
        match check_download_config(&jar, &orderable, &out_dir) {
            Some(true) => eprintln!(
                "jaxa check_dlconfig: SUCCESS ({} item(s), {skipped} skipped)",
                orderable.len()
            ),
            Some(false) => {
                eprintln!("jaxa_gportal_compiler: the download config gate refused");
                std::process::exit(1);
            }
            None => {
                eprintln!("jaxa_gportal_compiler: the download config response returned void");
                std::process::exit(1);
            }
        }
        for (i, r) in orderable.iter().enumerate() {
            match add_download(&jar, r, i, &out_dir) {
                Some(true) => {}
                Some(false) | None => {
                    eprintln!(
                        "jaxa_gportal_compiler: add_download for {} returned void",
                        r.id
                    );
                    std::process::exit(1);
                }
            }
        }
        if args.iter().any(|a| a == "--fetch-file") {
            let max_bytes = match arg_value(&args, "--probe-bytes") {
                Some(v) => match v.parse::<u64>() {
                    Ok(n) if n >= 1 => Some(n),
                    _ => {
                        eprintln!(
                            "jaxa_gportal_compiler: --probe-bytes carries no byte count >= 1 — refused"
                        );
                        std::process::exit(2);
                    }
                },
                None => None,
            };
            for (i, r) in orderable.iter().enumerate() {
                let Some(url) = download_url(r) else {
                    eprintln!("jaxa fetch: {} carries no file url", r.id);
                    std::process::exit(1);
                };
                let path = PathBuf::from(&out_dir).join(format!("jaxa_gportal_{i}_{}.bin", r.id));
                match fetch_file(&jar, url, &path, max_bytes) {
                    Some((code, size)) if (200..300).contains(&code) => eprintln!(
                        "jaxa fetch: {} → HTTP {code}, {size} B, {}",
                        r.id,
                        path.display()
                    ),
                    Some((code, _)) => {
                        eprintln!("jaxa fetch: {} returned HTTP {code}", r.id);
                        std::process::exit(1);
                    }
                    None => {
                        eprintln!("jaxa fetch: {} returned void", r.id);
                        std::process::exit(1);
                    }
                }
            }
        } else {
            eprintln!(
                "jaxa_gportal_compiler: {} product(s) registered; payload transfer via HTTP GET file_url with the session cookie, or SFTP {SFTP_HOST}:{SFTP_PORT}",
                orderable.len()
            );
        }
    }

    if args.iter().any(|a| a == "--ci-mode") && !upload_release(NETLOC, &manifest) {
        eprintln!("jaxa_gportal_compiler: CDN upload returned void");
        std::process::exit(1);
    }
}
