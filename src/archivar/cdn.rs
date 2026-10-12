use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};

use crate::archivar::range::{Sigv4PutArgs, sigv4_put_headers};
use crate::archivar::{date_str, hour_str, load_env, sha256};

const R2_REGION: &str = "auto";
const R2_MIRROR_MAX_BYTES: u64 = 16 * 1024 * 1024;

pub const GITHUB_RELEASE_ASSET_MAX_BYTES: u64 = 2 * 1024 * 1024 * 1024;

pub const CDN_PART_MAX_BYTES: u64 = GITHUB_RELEASE_ASSET_MAX_BYTES - 1024 * 1024;

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

pub struct PartsManifest {
    pub total_sha256: String,
    pub parts: Vec<(String, String, u64)>,
}

pub fn parts_manifest_name(asset: &str) -> String {
    format!("{asset}.parts")
}

pub fn part_name(asset: &str, idx: usize) -> String {
    format!("{asset}.p{idx:03}")
}

pub fn format_parts_manifest(total_sha256: &str, parts: &[(String, String, u64)]) -> String {
    let mut s = String::new();
    s.push_str(&format!("parts {}\n", parts.len()));
    s.push_str(&format!("sha256 {}\n", total_sha256));
    for (name, sha, len) in parts {
        s.push_str(&format!("{} {} {}\n", name, sha, len));
    }
    s
}

pub fn parse_parts_manifest(text: &str) -> Option<PartsManifest> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let n: usize = lines.next()?.strip_prefix("parts ")?.trim().parse().ok()?;
    let total = lines.next()?.strip_prefix("sha256 ")?.trim().to_string();
    if !is_sha256_hex(&total) {
        return None;
    }
    let mut parts = Vec::new();
    for line in lines {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() != 3 {
            return None;
        }
        let sha = f[1].to_string();
        if !is_sha256_hex(&sha) {
            return None;
        }
        let len: u64 = f[2].parse().ok()?;
        parts.push((f[0].to_string(), sha, len));
    }
    if parts.len() != n {
        return None;
    }
    Some(PartsManifest {
        total_sha256: total,
        parts,
    })
}

fn r2_mirror_cap() -> u64 {
    std::env::var("OMEGAFLOW_R2_MIRROR_MAX_BYTES")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(R2_MIRROR_MAX_BYTES)
}

fn r2_mirror_within_cap(size: u64) -> bool {
    size <= r2_mirror_cap()
}

pub const EPHEMERIS_TAG: &str = "ssd.jpl.nasa.gov-ephemeris";
pub const CAPPED_RELEASE: &str = "ssd.jpl.nasa.gov";
pub const CDN_REPO: &str = "omegaflow/sources";
pub const CDN_BASE: &str = "https://github.com/omegaflow/sources/releases/download";
pub const PS1_SLAB_BANDS: u32 = 80;
pub const MODIS_LST_CMG_FAMILY: &str = "data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg";

pub fn ps1_slab_tag(proj: u32) -> String {
    format!("ps1-dr2-{}", (proj / PS1_SLAB_BANDS) * PS1_SLAB_BANDS)
}

pub fn modis_lst_cmg_tag_of(name: &str) -> Option<String> {
    let rest = name.strip_prefix("modis_lst_cmg_")?;
    let (product, tail) = rest.split_once('_')?;
    if product.is_empty() {
        return None;
    }
    let year = modis_lst_cmg_year(tail)?;
    Some(format!("{MODIS_LST_CMG_FAMILY}-{product}-{year}"))
}

fn modis_lst_cmg_year(tail: &str) -> Option<&str> {
    if let Some(y) = tail.strip_suffix(".manifest") {
        return ascii_year(y);
    }
    let granule = tail.strip_suffix(".bin")?;
    let a = granule.find(".A")?;
    ascii_year(granule.get(a + 2..)?)
}

fn ascii_year(s: &str) -> Option<&str> {
    let y = s.get(0..4)?;
    if y.bytes().all(|b| b.is_ascii_digit()) {
        Some(y)
    } else {
        None
    }
}

pub fn cdn_base() -> String {
    if let Ok(base) = std::env::var("OMEGAFLOW_CDN_BASE") {
        return base;
    }
    CDN_BASE.to_string()
}

static VERIFIED_RELEASES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn verified_releases() -> &'static Mutex<HashSet<String>> {
    VERIFIED_RELEASES.get_or_init(|| Mutex::new(HashSet::new()))
}

fn release_verified(tag: &str) -> bool {
    verified_releases()
        .lock()
        .map(|set| set.contains(tag))
        .unwrap_or(false)
}

fn mark_release_verified(tag: &str) {
    if let Ok(mut set) = verified_releases().lock() {
        set.insert(tag.to_string());
    }
}

fn netloc_of(url: &str) -> Option<&str> {
    let rest = url.split_once("://")?.1;
    rest.split(['/', '?']).next().filter(|s| !s.is_empty())
}

fn normalize_tag(tag: &str) -> &str {
    tag.strip_prefix("www.").unwrap_or(tag)
}

fn cdn_tag_of(url: &str) -> Option<&str> {
    let rest = url.strip_prefix(CDN_BASE)?.strip_prefix('/')?;
    rest.split('/').next().filter(|s| !s.is_empty())
}

fn registered_netlocs() -> &'static HashSet<String> {
    static SET: OnceLock<HashSet<String>> = OnceLock::new();
    SET.get_or_init(|| {
        let mut set = HashSet::new();
        if let Ok(text) = std::fs::read_to_string("phi/sources.φ") {
            for line in text.lines() {
                if line.starts_with("decline ") {
                    continue;
                }
                if let Some(rest) = line.strip_prefix("url ") {
                    let key = cdn_tag_of(rest).or_else(|| netloc_of(rest));
                    if let Some(k) = key {
                        set.insert(normalize_tag(k).to_string());
                    }
                } else if let Some(rest) = line.strip_prefix("origin ")
                    && let Some(nl) = netloc_of(rest)
                {
                    set.insert(normalize_tag(nl).to_string());
                }
            }
        }
        set
    })
}

pub fn upload_release(tag: &str, path: &str) -> bool {
    if tag == CAPPED_RELEASE {
        eprintln!(
            "upload {}: release {} is capped (1000 assets) — upload to the family tag \"<host>-<family>\" instead",
            path, tag
        );
        return false;
    }
    if std::env::var("GH_TOKEN").is_err() {
        eprintln!("upload {}: GH_TOKEN absent", path);
        return false;
    }
    if let Ok(meta) = std::fs::metadata(path)
        && meta.len() > GITHUB_RELEASE_ASSET_MAX_BYTES
    {
        eprintln!(
            "upload {}: {} B over the GitHub release 2 GiB asset limit — the asset needs the split/R2 route, not a release upload",
            path,
            meta.len()
        );
        return false;
    }
    if !registered_netlocs().contains(normalize_tag(tag)) {
        eprintln!(
            "upload {}: tag {} is not a live source in phi/sources.φ — a CDN release is only created for a registered source",
            path, tag
        );
        return false;
    }
    if !ensure_release(tag) {
        eprintln!("upload {}: release {} not created", path, tag);
        return false;
    }
    let out = Command::new("gh")
        .arg("release")
        .arg("upload")
        .arg(tag)
        .arg(path)
        .arg("--clobber")
        .arg("--repo")
        .arg(CDN_REPO)
        .output();
    match out {
        Ok(o) if o.status.success() => {
            r2_mirror(tag, path);
            true
        }
        Ok(o) => {
            eprintln!(
                "upload {}: gh returned void: {}",
                path,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("upload {}: gh absent: {}", path, e);
            false
        }
    }
}

pub fn r2_mirror(tag: &str, path: &str) -> bool {
    if std::env::var("OMEGAFLOW_R2_MIRROR").is_err() {
        return false;
    }
    let size = match std::fs::metadata(path) {
        Ok(m) => m.len(),
        Err(e) => {
            eprintln!("r2_mirror {}: metadata absent: {}", path, e);
            return false;
        }
    };
    if !r2_mirror_within_cap(size) {
        eprintln!(
            "r2_mirror {}: {} B over the {}-B cap — the release keeps it, R2 stays the manifest/idx tier",
            path,
            size,
            r2_mirror_cap()
        );
        return false;
    }
    if let Ok(worker) = std::env::var("OMEGAFLOW_R2_WORKER")
        && !worker.is_empty()
        && std::env::var("ACTIONS_ID_TOKEN_REQUEST_URL").is_ok()
    {
        return r2_mirror_oidc(&worker, tag, path);
    }
    r2_mirror_s3(tag, path)
}

fn oidc_token() -> Option<String> {
    let url = std::env::var("ACTIONS_ID_TOKEN_REQUEST_URL").ok()?;
    let token = std::env::var("ACTIONS_ID_TOKEN_REQUEST_TOKEN").ok()?;
    let sep = if url.contains('?') { '&' } else { '?' };
    let request = format!("{}{}audience=omegaflow-r2-verifier", url, sep);
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-f")
        .arg("-H")
        .arg(format!("Authorization: bearer {}", token))
        .arg(&request)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let body = String::from_utf8_lossy(&out.stdout);
    let start = body.find("\"value\":\"")? + "\"value\":\"".len();
    let rest = &body[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn r2_mirror_oidc(worker: &str, tag: &str, path: &str) -> bool {
    let Some(jwt) = oidc_token() else {
        eprintln!("r2_mirror {}: OIDC token absent — pending", path);
        return false;
    };
    let body = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("r2_mirror {}: read void: {}", path, e);
            return false;
        }
    };
    let name = std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path);
    let url = format!("{}/{}/{}", worker.trim_end_matches('/'), tag, name);
    let payload_sha256 = sha256::sha256_hex(&body);
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-f")
        .arg("-X")
        .arg("PUT")
        .arg(&url)
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", jwt))
        .arg("-H")
        .arg(format!("x-amz-content-sha256: {}", payload_sha256))
        .arg("--data-binary")
        .arg("@-");
    if let Ok(ca) = std::env::var("OMEGAFLOW_CA_BUNDLE")
        && !ca.is_empty()
        && std::path::Path::new(&ca).is_file()
    {
        cmd.arg("--cacert").arg(ca);
    }
    let mut child = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("r2_mirror {}: curl absent: {}", path, e);
            return false;
        }
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&body);
    }
    match child.wait_with_output() {
        Ok(o) if o.status.success() => {
            eprintln!("r2_mirror: oidc {} ok", url);
            true
        }
        Ok(o) => {
            eprintln!(
                "r2_mirror {}: worker returned void: {}",
                path,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("r2_mirror {}: wait void: {}", path, e);
            false
        }
    }
}

fn r2_mirror_s3(tag: &str, path: &str) -> bool {
    let env = load_env();
    let (Some(access_key), Some(secret_key), Some(endpoint), Some(bucket)) = (
        env.get("R2_ACCESS_KEY_ID"),
        env.get("R2_SECRET_ACCESS_KEY"),
        env.get("R2_ENDPOINT"),
        env.get("R2_BUCKET"),
    ) else {
        eprintln!("r2_mirror {}: R2_* absent — pending", path);
        return false;
    };
    let endpoint = endpoint.trim_end_matches('/');
    let Some(rest) = endpoint
        .strip_prefix("https://")
        .or_else(|| endpoint.strip_prefix("http://"))
    else {
        eprintln!("r2_mirror {}: R2_ENDPOINT malformed", path);
        return false;
    };
    let host = rest.split('/').next().unwrap_or(rest);
    let body = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("r2_mirror {}: read void: {}", path, e);
            return false;
        }
    };
    let name = std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path);
    let canonical_uri = format!("/{}/{}/{}", bucket, tag, name);
    let url = format!("{}{}", endpoint, canonical_uri);
    let payload_sha256 = sha256::sha256_hex(&body);
    let unix = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => {
            eprintln!("r2_mirror {}: system clock before UNIX_EPOCH", path);
            return false;
        }
    };
    let date_stamp = date_str(unix).replace('-', "");
    let amz_date = hour_str(unix).replace(['-', ':'], "");
    let headers = sigv4_put_headers(&Sigv4PutArgs {
        method: "PUT",
        access_key,
        secret_key,
        region: R2_REGION,
        host,
        canonical_uri: &canonical_uri,
        payload_sha256: &payload_sha256,
        content_length: Some(body.len() as u64),
        amz_date: &amz_date,
        date_stamp: &date_stamp,
    });
    let mut cmd = Command::new("curl");
    cmd.arg("-sS").arg("-f").arg("-X").arg("PUT").arg(&url);
    for (k, v) in &headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    cmd.arg("--data-binary").arg("@-");
    if let Ok(ca) = std::env::var("OMEGAFLOW_CA_BUNDLE")
        && !ca.is_empty()
        && std::path::Path::new(&ca).is_file()
    {
        cmd.arg("--cacert").arg(ca);
    }
    let mut child = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("r2_mirror {}: curl absent: {}", path, e);
            return false;
        }
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&body);
    }
    match child.wait_with_output() {
        Ok(o) if o.status.success() => {
            eprintln!("r2_mirror: s3 {} ok", url);
            true
        }
        Ok(o) => {
            eprintln!(
                "r2_mirror {}: curl returned void: {}",
                path,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("r2_mirror {}: wait void: {}", path, e);
            false
        }
    }
}

pub fn body_url(name: &str) -> String {
    format!("{}/{}/ephemeris_{}.bin", CDN_BASE, EPHEMERIS_TAG, name)
}

pub fn ensure_release(tag: &str) -> bool {
    if std::env::var("GH_TOKEN").is_err() {
        return false;
    }
    if release_verified(tag) {
        return true;
    }
    let view = Command::new("gh")
        .arg("release")
        .arg("view")
        .arg(tag)
        .arg("--repo")
        .arg(CDN_REPO)
        .output();
    if view.map(|o| o.status.success()).unwrap_or(false) {
        mark_release_verified(tag);
        return true;
    }
    let out = Command::new("gh")
        .arg("release")
        .arg("create")
        .arg(tag)
        .arg("--repo")
        .arg(CDN_REPO)
        .arg("--title")
        .arg(tag)
        .arg("--notes")
        .arg("reference dataset mirror")
        .output();
    match out {
        Ok(o) if o.status.success() => {
            mark_release_verified(tag);
            true
        }
        Ok(o) => {
            let re = Command::new("gh")
                .arg("release")
                .arg("view")
                .arg(tag)
                .arg("--repo")
                .arg(CDN_REPO)
                .output();
            if re.map(|r| r.status.success()).unwrap_or(false) {
                mark_release_verified(tag);
                return true;
            }
            eprintln!(
                "ensure release {}: gh returned void: {}",
                tag,
                String::from_utf8_lossy(&o.stderr).trim()
            );
            false
        }
        Err(e) => {
            eprintln!("ensure release {}: gh absent: {}", tag, e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r2_mirror_is_off_unless_the_flag_is_set() {
        if std::env::var("OMEGAFLOW_R2_MIRROR").is_ok() {
            return;
        }
        assert!(!r2_mirror("ci-probe", "Cargo.toml"));
    }

    #[test]
    fn r2_mirror_cap_excludes_dataset_scale_assets() {
        if std::env::var("OMEGAFLOW_R2_MIRROR_MAX_BYTES").is_ok() {
            return;
        }
        assert!(r2_mirror_within_cap(1024));
        assert!(r2_mirror_within_cap(R2_MIRROR_MAX_BYTES));
        assert!(!r2_mirror_within_cap(R2_MIRROR_MAX_BYTES + 1));
        assert!(!r2_mirror_within_cap(100 * 1024 * 1024));
    }

    #[test]
    fn netloc_matches_the_release_tag_after_www_strip() {
        assert_eq!(
            netloc_of("https://www.geoazur.fr/astrogeo/x"),
            Some("www.geoazur.fr")
        );
        assert_eq!(
            netloc_of("http://jvo.nao.ac.jp/skynode/do/x"),
            Some("jvo.nao.ac.jp")
        );
        assert_eq!(normalize_tag("www.geoazur.fr"), "geoazur.fr");
        assert_eq!(normalize_tag("jvo.nao.ac.jp"), "jvo.nao.ac.jp");
        assert_eq!(
            cdn_tag_of(
                "https://github.com/omegaflow/sources/releases/download/ser.im-ldi.com/x.bin"
            ),
            Some("ser.im-ldi.com")
        );
        assert_eq!(cdn_tag_of("https://api.open-meteo.com/v1/forecast"), None);
        assert_eq!(netloc_of("procedure: SPK/DAF harvest"), None);
    }

    #[test]
    fn parts_manifest_round_trips() {
        let parts = vec![
            (part_name("big.tif", 0), "a".repeat(64), CDN_PART_MAX_BYTES),
            (part_name("big.tif", 1), "b".repeat(64), 1234),
        ];
        let text = format_parts_manifest(&"c".repeat(64), &parts);
        let parsed = parse_parts_manifest(&text).expect("round trip");
        assert_eq!(parsed.total_sha256, "c".repeat(64));
        assert_eq!(parsed.parts, parts);
        assert_eq!(parts_manifest_name("big.tif"), "big.tif.parts");
        assert_eq!(part_name("big.tif", 0), "big.tif.p000");
        assert_eq!(part_name("big.tif", 11), "big.tif.p011");
    }

    #[test]
    fn parts_manifest_rejects_a_count_mismatch() {
        let text = "parts 2\nsha256 ".to_string()
            + &"c".repeat(64)
            + "\nbig.tif.p000 "
            + &"a".repeat(64)
            + " 10\n";
        assert!(parse_parts_manifest(&text).is_none());
    }

    #[test]
    fn modis_lst_cmg_tag_parses_the_granule_shard() {
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_8day_MOD11C2.A2000049.061.2020330085614.bin")
                .as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-8day-2000")
        );
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_monthly_MOD11C3.A2026348.061.2026182060603.bin")
                .as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-monthly-2026")
        );
    }

    #[test]
    fn modis_lst_cmg_tag_parses_the_year_manifest() {
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_8day_2000.manifest").as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-8day-2000")
        );
        assert_eq!(
            modis_lst_cmg_tag_of("modis_lst_cmg_monthly_2026.manifest").as_deref(),
            Some("data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-monthly-2026")
        );
    }

    #[test]
    fn modis_lst_cmg_tag_returns_void_for_series_manifests_and_foreign_names() {
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg_8day.manifest"), None);
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg_monthly.manifest"), None);
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg_8day_garbage.bin"), None);
        assert_eq!(modis_lst_cmg_tag_of("modis_lst_cmg__2000.manifest"), None);
        assert_eq!(modis_lst_cmg_tag_of("ps1-dr2-0"), None);
        assert_eq!(modis_lst_cmg_tag_of(""), None);
    }
}
