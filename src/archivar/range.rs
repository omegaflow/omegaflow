use super::*;

fn append_ca(cmd: &mut Command) {
    let path = match std::env::var("OMEGAFLOW_CA_BUNDLE") {
        Ok(p) if !p.is_empty() && std::path::Path::new(&p).is_file() => p,
        _ => return,
    };
    cmd.arg("--cacert").arg(path);
}

const RANGE_MAX_TIME_S: u64 = 1 << 7;
const RANGE_RETRY: u64 = 3;

pub const S3_REGION: &str = "us-west-2";
pub const S3_ENDPOINT: &str = "s3.us-west-2.amazonaws.com";
const S3_SERVICE: &str = "s3";
const NSIDC_S3_CREDENTIALS_URL: &str = "https://data.nsidc.earthdatacloud.nasa.gov/s3credentials";
const PODAAC_S3_CREDENTIALS_URL: &str = "https://archive.podaac.earthdata.nasa.gov/s3credentials";
const LPDAAC_S3_CREDENTIALS_URL: &str = "https://data.lpdaac.earthdatacloud.nasa.gov/s3credentials";
const AWS4_EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn offset_end(offset: u64, len: u64) -> Option<(u64, u64)> {
    if len == 0 {
        return None;
    }
    let end = offset.checked_add(len)?;
    Some((offset, end - 1))
}

pub fn fetch_range(
    url: &str,
    offset: u64,
    len: u64,
    headers: &[(String, String)],
) -> Option<Vec<u8>> {
    let (start, end) = offset_end(offset, len)?;
    let range = format!("{}-{}", start, end);
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg(RANGE_RETRY.to_string())
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(RANGE_MAX_TIME_S.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string())
        .arg("-r")
        .arg(range);
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    append_ca(&mut cmd);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "\r\x1b[Krange returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

fn hex_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut pad_key = [0u8; 64];
    if key.len() > 64 {
        pad_key[..32].copy_from_slice(&sha256::sha256_raw(key));
    } else {
        pad_key[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0u8; 64];
    let mut opad = [0u8; 64];
    for i in 0..64 {
        ipad[i] = pad_key[i] ^ 0x36;
        opad[i] = pad_key[i] ^ 0x5c;
    }
    let mut inner = Vec::with_capacity(64 + data.len());
    inner.extend_from_slice(&ipad);
    inner.extend_from_slice(data);
    let inner_hash = sha256::sha256_raw(&inner);
    let mut outer = Vec::with_capacity(96);
    outer.extend_from_slice(&opad);
    outer.extend_from_slice(&inner_hash);
    sha256::sha256_raw(&outer)
}

fn sigv4_signing_key(secret_key: &str, date_stamp: &str, region: &str) -> [u8; 32] {
    let k_date = hmac_sha256(
        format!("AWS4{}", secret_key).as_bytes(),
        date_stamp.as_bytes(),
    );
    let k_region = hmac_sha256(&k_date, region.as_bytes());
    let k_service = hmac_sha256(&k_region, S3_SERVICE.as_bytes());
    hmac_sha256(&k_service, b"aws4_request")
}

pub struct Sigv4Args<'a> {
    pub access_key: &'a str,
    pub secret_key: &'a str,
    pub region: &'a str,
    pub host: &'a str,
    pub canonical_uri: &'a str,
    pub canonical_query: &'a str,
    pub range: Option<&'a str>,
    pub amz_date: &'a str,
    pub date_stamp: &'a str,
    pub session_token: Option<&'a str>,
}

pub fn sigv4_headers(args: &Sigv4Args<'_>) -> Vec<(String, String)> {
    let Sigv4Args {
        access_key,
        secret_key,
        region,
        host,
        canonical_uri,
        canonical_query,
        range,
        amz_date,
        date_stamp,
        session_token,
    } = *args;
    let mut canonical_headers = String::new();
    canonical_headers.push_str(&format!("host:{}\n", host));
    let mut signed_headers = String::from("host");
    if let Some(range) = range {
        canonical_headers.push_str(&format!("range:{}\n", range));
        signed_headers.push_str(";range");
    }
    canonical_headers.push_str(&format!("x-amz-content-sha256:{}\n", AWS4_EMPTY_SHA256));
    canonical_headers.push_str(&format!("x-amz-date:{}\n", amz_date));
    signed_headers.push_str(";x-amz-content-sha256;x-amz-date");
    if let Some(token) = session_token {
        canonical_headers.push_str(&format!("x-amz-security-token:{}\n", token));
        signed_headers.push_str(";x-amz-security-token");
    }
    let canonical_request = format!(
        "GET\n{}\n{}\n{}\n{}\n{}",
        canonical_uri, canonical_query, canonical_headers, signed_headers, AWS4_EMPTY_SHA256
    );
    let scope = format!("{}/{}/{}/aws4_request", date_stamp, region, S3_SERVICE);
    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{}\n{}\n{}",
        amz_date,
        scope,
        sha256::sha256_hex(canonical_request.as_bytes())
    );
    let signing_key = sigv4_signing_key(secret_key, date_stamp, region);
    let signature = hex_bytes(&hmac_sha256(&signing_key, string_to_sign.as_bytes()));
    let authorization = format!(
        "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
        access_key, scope, signed_headers, signature
    );
    let mut out = Vec::new();
    out.push((
        "x-amz-content-sha256".to_string(),
        AWS4_EMPTY_SHA256.to_string(),
    ));
    out.push(("x-amz-date".to_string(), amz_date.to_string()));
    if let Some(token) = session_token {
        out.push(("x-amz-security-token".to_string(), token.to_string()));
    }
    out.push(("Authorization".to_string(), authorization));
    out
}

pub fn uri_encode_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for &b in path.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

pub fn s3_parts(url: &str) -> Option<(String, String)> {
    let rest = url.strip_prefix("s3://")?;
    let (bucket, key) = rest.split_once('/')?;
    if bucket.is_empty() || key.is_empty() {
        return None;
    }
    Some((bucket.to_string(), key.to_string()))
}

pub fn s3_https_url(url: &str) -> Option<String> {
    let (bucket, key) = s3_parts(url)?;
    Some(format!(
        "https://{}/{}/{}",
        S3_ENDPOINT,
        bucket,
        uri_encode_path(&key)
    ))
}

pub struct S3Credentials {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S3CredentialRoute {
    Bearer(&'static str),
    OAuth,
}

pub fn s3_credential_route(bucket: &str) -> Option<S3CredentialRoute> {
    let bucket = bucket.strip_prefix("s3://").unwrap_or(bucket);
    let bucket = bucket.split('/').next().unwrap_or(bucket);
    if bucket.starts_with("podaac-") {
        Some(S3CredentialRoute::Bearer(PODAAC_S3_CREDENTIALS_URL))
    } else if bucket.starts_with("nsidc-") {
        Some(S3CredentialRoute::Bearer(NSIDC_S3_CREDENTIALS_URL))
    } else if bucket.starts_with("lp-prod-") {
        Some(S3CredentialRoute::Bearer(LPDAAC_S3_CREDENTIALS_URL))
    } else if bucket.starts_with("gesdisc")
        || bucket.starts_with("goldsmr5")
        || bucket.starts_with("goldsmr2")
    {
        Some(S3CredentialRoute::OAuth)
    } else {
        None
    }
}

fn edl_bearer_credentials(url: &str, edl_token: &str) -> Option<S3Credentials> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", edl_token))
        .arg("-m")
        .arg(RANGE_MAX_TIME_S.to_string());
    append_ca(&mut cmd);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let body = String::from_utf8_lossy(&output.stdout);
    let json = parse_json(&body)?;
    let access_key = jstr(&json, "accessKeyId")?;
    let secret_key = jstr(&json, "secretAccessKey")?;
    let session_token = jstr(&json, "sessionToken")?;
    if access_key.is_empty() || secret_key.is_empty() || session_token.is_empty() {
        return None;
    }
    Some(S3Credentials {
        access_key,
        secret_key,
        session_token,
    })
}

pub fn edl_s3_credentials_for(bucket: &str, edl_token: &str) -> Option<S3Credentials> {
    let route = s3_credential_route(bucket)?;
    match route {
        S3CredentialRoute::Bearer(url) => edl_bearer_credentials(url, edl_token),
        S3CredentialRoute::OAuth => None,
    }
}

pub fn fetch_s3_range(
    s3_url: &str,
    offset: u64,
    len: u64,
    creds: Option<&S3Credentials>,
) -> Option<Vec<u8>> {
    let (bucket, key) = s3_parts(s3_url)?;
    let (start, end) = offset_end(offset, len)?;
    let canonical_uri = format!("/{}/{}", bucket, uri_encode_path(&key));
    let https_url = format!("https://{}{}", S3_ENDPOINT, canonical_uri);
    let range = format!("bytes={}-{}", start, end);
    let headers = match creds {
        Some(c) => {
            let unix = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
            let date_stamp = date_str(unix).replace('-', "");
            let amz_date = hour_str(unix).replace(['-', ':'], "");
            sigv4_headers(&Sigv4Args {
                access_key: &c.access_key,
                secret_key: &c.secret_key,
                region: S3_REGION,
                host: S3_ENDPOINT,
                canonical_uri: &canonical_uri,
                canonical_query: "",
                range: Some(&range),
                amz_date: &amz_date,
                date_stamp: &date_stamp,
                session_token: Some(&c.session_token),
            })
        }
        None => Vec::new(),
    };
    fetch_range(&https_url, offset, len, &headers)
}

fn fetch_whole(url: &str, ttl: u64, headers: &[(String, String)]) -> Option<Vec<u8>> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg(RANGE_RETRY.to_string())
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(ttl_transfer_bound(ttl).to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string());
    for (k, v) in headers {
        cmd.arg("-H").arg(format!("{}: {}", k, v));
    }
    append_ca(&mut cmd);
    cmd.arg(url);
    let output = cmd.output().ok()?;
    if output.status.success() {
        Some(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "\r\x1b[Ks3 returned ({}): {} {}",
            output.status,
            url,
            stderr.trim()
        );
        None
    }
}

fn public_s3_https_url(bucket: &str, key: &str) -> String {
    format!(
        "https://{}.s3.amazonaws.com/{}",
        bucket,
        uri_encode_path(key)
    )
}

pub fn fetch_s3_whole(s3_url: &str, ttl: u64) -> Option<Vec<u8>> {
    let (bucket, key) = s3_parts(s3_url)?;
    match s3_credential_route(&bucket) {
        Some(S3CredentialRoute::Bearer(url)) => {
            let token = std::env::var("EARTHDATA_EDL_TOKEN")
                .ok()
                .filter(|t| !t.is_empty())?;
            let creds = edl_bearer_credentials(url, &token)?;
            let canonical_uri = format!("/{}/{}", bucket, uri_encode_path(&key));
            let https_url = format!("https://{}{}", S3_ENDPOINT, canonical_uri);
            let unix = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
            let date_stamp = date_str(unix).replace('-', "");
            let amz_date = hour_str(unix).replace(['-', ':'], "");
            let headers = sigv4_headers(&Sigv4Args {
                access_key: &creds.access_key,
                secret_key: &creds.secret_key,
                region: S3_REGION,
                host: S3_ENDPOINT,
                canonical_uri: &canonical_uri,
                canonical_query: "",
                range: None,
                amz_date: &amz_date,
                date_stamp: &date_stamp,
                session_token: Some(&creds.session_token),
            });
            fetch_whole(&https_url, ttl, &headers)
        }
        Some(S3CredentialRoute::OAuth) => None,
        None => fetch_whole(&public_s3_https_url(&bucket, &key), ttl, &[]),
    }
}

pub struct S3Object {
    pub key: String,
    pub size: u64,
    pub last_modified: String,
}

struct ListPage {
    objects: Vec<S3Object>,
    next_token: Option<String>,
    truncated: bool,
}

fn uri_encode_query(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn list_query(prefix: &str, token: Option<&str>) -> String {
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(t) = token {
        params.push(("continuation-token", t.to_string()));
    }
    params.push(("list-type", "2".to_string()));
    if !prefix.is_empty() {
        params.push(("prefix", prefix.to_string()));
    }
    params.sort_by(|a, b| a.0.cmp(b.0));
    let mut q = String::new();
    for (i, (k, v)) in params.iter().enumerate() {
        if i > 0 {
            q.push('&');
        }
        q.push_str(&uri_encode_query(k));
        q.push('=');
        q.push_str(&uri_encode_query(v));
    }
    q
}

fn xml_elem_text(doc: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let start = doc.find(&open)? + open.len();
    let close = format!("</{}>", tag);
    let end = doc[start..].find(&close)? + start;
    Some(doc[start..end].to_string())
}

fn parse_list_objects(bytes: &[u8]) -> Option<ListPage> {
    let doc = std::str::from_utf8(bytes).ok()?;
    let mut objects = Vec::new();
    let mut rest = doc;
    while let Some(start) = rest.find("<Contents>") {
        let after_open = &rest[start + "<Contents>".len()..];
        let Some(end) = after_open.find("</Contents>") else {
            break;
        };
        let block = &after_open[..end];
        let key = xml_elem_text(block, "Key")?;
        let size = xml_elem_text(block, "Size")?.parse::<u64>().ok()?;
        let last_modified = xml_elem_text(block, "LastModified")?;
        objects.push(S3Object {
            key,
            size,
            last_modified,
        });
        rest = &after_open[end + "</Contents>".len()..];
    }
    let truncated = xml_elem_text(doc, "IsTruncated").as_deref() == Some("true");
    let next_token = xml_elem_text(doc, "NextContinuationToken");
    Some(ListPage {
        objects,
        next_token,
        truncated,
    })
}

fn s3_list_request(bucket: &str, canonical_query: &str) -> Option<(String, Vec<(String, String)>)> {
    let canonical_uri = format!("/{}", bucket);
    match s3_credential_route(bucket) {
        Some(S3CredentialRoute::Bearer(url)) => {
            let token = std::env::var("EARTHDATA_EDL_TOKEN")
                .ok()
                .filter(|t| !t.is_empty())?;
            let creds = edl_bearer_credentials(url, &token)?;
            let https_url = format!(
                "https://{}{}?{}",
                S3_ENDPOINT, canonical_uri, canonical_query
            );
            let unix = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
            let date_stamp = date_str(unix).replace('-', "");
            let amz_date = hour_str(unix).replace(['-', ':'], "");
            let headers = sigv4_headers(&Sigv4Args {
                access_key: &creds.access_key,
                secret_key: &creds.secret_key,
                region: S3_REGION,
                host: S3_ENDPOINT,
                canonical_uri: &canonical_uri,
                canonical_query,
                range: None,
                amz_date: &amz_date,
                date_stamp: &date_stamp,
                session_token: Some(&creds.session_token),
            });
            Some((https_url, headers))
        }
        Some(S3CredentialRoute::OAuth) => None,
        None => {
            let https_url = format!("https://{}.s3.amazonaws.com/?{}", bucket, canonical_query);
            Some((https_url, Vec::new()))
        }
    }
}

pub fn s3_list(bucket: &str, prefix: &str, ttl: u64) -> Option<Vec<S3Object>> {
    let mut out: Vec<S3Object> = Vec::new();
    let mut token: Option<String> = None;
    loop {
        let query = list_query(prefix, token.as_deref());
        let (url, headers) = s3_list_request(bucket, &query)?;
        let bytes = fetch_whole(&url, ttl, &headers)?;
        let page = parse_list_objects(&bytes)?;
        out.extend(page.objects);
        if !page.truncated {
            break;
        }
        token = page.next_token;
        if token.is_none() {
            break;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_arithmetic_is_inclusive_end() {
        assert_eq!(offset_end(512, 16), Some((512, 527)));
        assert_eq!(offset_end(0, 1), Some((0, 0)));
        assert_eq!(offset_end(u64::MAX, 2), None);
        assert_eq!(offset_end(10, 0), None);
    }

    #[test]
    fn empty_range_is_absent() {
        assert!(fetch_range("https://example.com/absent", 0, 0, &[]).is_none());
    }

    #[test]
    fn hmac_sha256_matches_rfc4231() {
        let key = [0x0bu8; 20];
        assert_eq!(
            hex_bytes(&hmac_sha256(&key, b"Hi There")),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
        assert_eq!(
            hex_bytes(&hmac_sha256(b"Jefe", b"what do ya want for nothing?")),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    #[test]
    fn empty_payload_hash_is_the_empty_stream_digest() {
        assert_eq!(AWS4_EMPTY_SHA256, sha256::sha256_hex(b""));
    }

    #[test]
    fn sigv4_reproduces_the_aws_get_object_example() {
        let headers = sigv4_headers(&Sigv4Args {
            access_key: "AKIAIOSFODNN7EXAMPLE",
            secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            region: "us-east-1",
            host: "examplebucket.s3.amazonaws.com",
            canonical_uri: "/test.txt",
            canonical_query: "",
            range: Some("bytes=0-9"),
            amz_date: "20130524T000000Z",
            date_stamp: "20130524",
            session_token: None,
        });
        let auth = headers
            .iter()
            .find(|(k, _)| k == "Authorization")
            .map(|(_, v)| v.as_str())
            .unwrap();
        assert_eq!(
            auth,
            "AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, SignedHeaders=host;range;x-amz-content-sha256;x-amz-date, Signature=f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41"
        );
    }

    #[test]
    fn sigv4_whole_headers_omit_the_range_header() {
        let headers = sigv4_headers(&Sigv4Args {
            access_key: "AKIAIOSFODNN7EXAMPLE",
            secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            region: "us-east-1",
            host: "examplebucket.s3.amazonaws.com",
            canonical_uri: "/test.txt",
            canonical_query: "",
            range: None,
            amz_date: "20130524T000000Z",
            date_stamp: "20130524",
            session_token: None,
        });
        assert!(headers.iter().all(|(k, _)| k != "range"));
        let auth = headers
            .iter()
            .find(|(k, _)| k == "Authorization")
            .map(|(_, v)| v.as_str())
            .unwrap();
        assert!(
            auth.contains("SignedHeaders=host;x-amz-content-sha256;x-amz-date"),
            "the whole-object signature carries no range header"
        );
    }

    #[test]
    fn public_s3_url_uses_virtual_host_style() {
        assert_eq!(
            public_s3_https_url("noaa-goes16", "ABI-L1b-RadF/2020/001/00/foo.nc"),
            "https://noaa-goes16.s3.amazonaws.com/ABI-L1b-RadF/2020/001/00/foo.nc"
        );
        assert_eq!(
            public_s3_https_url("bucket", "a b.nc"),
            "https://bucket.s3.amazonaws.com/a%20b.nc"
        );
    }

    #[test]
    fn s3_credential_route_maps_bucket_to_daac_endpoint() {
        assert_eq!(
            s3_credential_route("podaac-swot-ops-cumulus-public"),
            Some(S3CredentialRoute::Bearer(PODAAC_S3_CREDENTIALS_URL))
        );
        assert_eq!(
            s3_credential_route("nsidc-cumulus-prod-public"),
            Some(S3CredentialRoute::Bearer(NSIDC_S3_CREDENTIALS_URL))
        );
        assert_eq!(s3_credential_route("noaa-goes16"), None);
        assert_eq!(
            s3_credential_route("goldsmr5"),
            Some(S3CredentialRoute::OAuth)
        );
        assert_eq!(
            s3_credential_route("s3://lp-prod-protected/key"),
            Some(S3CredentialRoute::Bearer(LPDAAC_S3_CREDENTIALS_URL))
        );
    }

    #[test]
    fn s3_resolver_maps_bucket_key_to_https() {
        assert_eq!(
            s3_https_url("s3://podaac-ops-cumulus-public/GRACE-FO_L2/foo.nc").as_deref(),
            Some("https://s3.us-west-2.amazonaws.com/podaac-ops-cumulus-public/GRACE-FO_L2/foo.nc")
        );
        assert_eq!(
            s3_https_url("s3://bucket/a b.nc").as_deref(),
            Some("https://s3.us-west-2.amazonaws.com/bucket/a%20b.nc")
        );
        assert!(s3_https_url("https://example.com/x").is_none());
        assert!(s3_https_url("s3://bucket").is_none());
    }

    #[test]
    fn list_query_sorts_and_encodes_parameters() {
        assert_eq!(list_query("", None), "list-type=2");
        assert_eq!(
            list_query("GRACE-FO_L2/", None),
            "list-type=2&prefix=GRACE-FO_L2%2F"
        );
        assert_eq!(
            list_query("a b", Some("tok/1")),
            "continuation-token=tok%2F1&list-type=2&prefix=a%20b"
        );
    }

    #[test]
    fn uri_encode_query_encodes_the_reserved_set() {
        assert_eq!(uri_encode_query("a b/c"), "a%20b%2Fc");
        assert_eq!(uri_encode_query("x=y"), "x%3Dy");
        assert_eq!(uri_encode_query("abc-._~"), "abc-._~");
    }

    #[test]
    fn parse_list_objects_decodes_a_page() {
        let doc = "<ListBucketResult>\
<IsTruncated>true</IsTruncated>\
<NextContinuationToken>tok2</NextContinuationToken>\
<Contents><Key>GRACE-FO_L2/a.nc</Key><LastModified>2026-08-14T00:00:00.000Z</LastModified><Size>1048576</Size></Contents>\
<Contents><Key>GRACE-FO_L2/b.nc</Key><LastModified>2026-08-14T01:00:00.000Z</LastModified><Size>2048</Size></Contents>\
</ListBucketResult>";
        let page = parse_list_objects(doc.as_bytes()).unwrap();
        assert!(page.truncated);
        assert_eq!(page.next_token.as_deref(), Some("tok2"));
        assert_eq!(page.objects.len(), 2);
        assert_eq!(page.objects[0].key, "GRACE-FO_L2/a.nc");
        assert_eq!(page.objects[0].size, 1048576);
        assert_eq!(page.objects[0].last_modified, "2026-08-14T00:00:00.000Z");
        assert_eq!(page.objects[1].key, "GRACE-FO_L2/b.nc");
        assert_eq!(page.objects[1].size, 2048);
    }

    #[test]
    fn sigv4_list_headers_sign_the_query_string() {
        let query = "list-type=2&prefix=GRACE-FO_L2%2F";
        let no_query = sigv4_headers(&Sigv4Args {
            access_key: "AKIAIOSFODNN7EXAMPLE",
            secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            region: "us-east-1",
            host: "examplebucket.s3.amazonaws.com",
            canonical_uri: "/examplebucket",
            canonical_query: "",
            range: None,
            amz_date: "20130524T000000Z",
            date_stamp: "20130524",
            session_token: None,
        });
        let with_query = sigv4_headers(&Sigv4Args {
            access_key: "AKIAIOSFODNN7EXAMPLE",
            secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            region: "us-east-1",
            host: "examplebucket.s3.amazonaws.com",
            canonical_uri: "/examplebucket",
            canonical_query: query,
            range: None,
            amz_date: "20130524T000000Z",
            date_stamp: "20130524",
            session_token: None,
        });
        let auth_no_query = no_query
            .iter()
            .find(|(k, _)| k == "Authorization")
            .map(|(_, v)| v.as_str())
            .unwrap();
        let auth_with_query = with_query
            .iter()
            .find(|(k, _)| k == "Authorization")
            .map(|(_, v)| v.as_str())
            .unwrap();
        assert_ne!(auth_no_query, auth_with_query);
        assert!(auth_with_query.contains("SignedHeaders=host;x-amz-content-sha256;x-amz-date"));
    }
}
