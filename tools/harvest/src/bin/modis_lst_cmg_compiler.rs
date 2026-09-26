use omegaflow::archivar::json::{JsonVal, jpath_val, parse_json};
use omegaflow::archivar::range::fetch_bearer_range;
use omegaflow::archivar::{LeapSeconds, embedded_lsk, parse_iso_tdb};
use omegaflow::cdn::upload_release;
use omegaflow::hdf4::Hdf4;
use std::env;
use std::fs;
use std::process::Command;

const NETLOC: &str = "data.lpdaac.earthdatacloud.nasa.gov";
const CMR_UMM_GRANULES_URL: &str = "https://cmr.earthdata.nasa.gov/search/granules.umm_json";
const LP_PROD_PREFIX: &str = "https://data.lpdaac.earthdatacloud.nasa.gov/lp-prod-protected/";
const PROBE_WINDOW: u64 = 1 << 20;
const ESCALATION_WINDOW: u64 = 1 << 24;
const CMR_PAGE_SIZE: usize = 1 << 5;
const CMR_MAX_TIME_S: u64 = 1 << 7;
const CONNECT_BOUND_S: u64 = 1 << 5;
const MAGIC: [u8; 4] = *b"MCM1";
const REC_FIELDS: usize = 5;
const REC_BYTES: usize = REC_FIELDS * 8;
const COMP_DAY: f64 = 1.0;
const COMP_NIGHT: f64 = 2.0;
const DFTAG_NDG: u16 = 720;
const HDF4_MAGIC: [u8; 4] = [0x0e, 0x03, 0x13, 0x01];

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

fn edl_token() -> Option<String> {
    if let Ok(t) = env::var("EARTHDATA_EDL_TOKEN") {
        if !t.trim().is_empty() {
            return Some(t.trim().to_string());
        }
    }
    let text = fs::read_to_string(".secrets.local").ok()?;
    parse_secret(&text, "EARTHDATA_EDL_TOKEN")
}

fn cmr_fetch(url: &str) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-S")
        .arg("-f")
        .arg("-L")
        .arg("-g")
        .arg("--retry")
        .arg("3")
        .arg("--retry-all-errors")
        .arg("--retry-delay")
        .arg("2")
        .arg("-m")
        .arg(CMR_MAX_TIME_S.to_string())
        .arg("--connect-timeout")
        .arg(CONNECT_BOUND_S.to_string());
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if !out.status.success() {
        eprintln!(
            "cmr returned ({}): {} {}",
            out.status,
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
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

fn cmr_query(concept_id: &str, temporal: Option<&str>, page_size: usize) -> String {
    let mut params: Vec<(String, String)> = vec![
        ("concept_id".to_string(), concept_id.to_string()),
        ("page_size".to_string(), page_size.to_string()),
    ];
    if let Some(t) = temporal {
        params.push(("temporal".to_string(), t.to_string()));
    }
    params.sort_by(|a, b| a.0.cmp(&b.0));
    params
        .iter()
        .map(|(k, v)| format!("{}={}", uri_encode_query(k), uri_encode_query(v)))
        .collect::<Vec<_>>()
        .join("&")
}

struct CmrGranule {
    ur: String,
    url: String,
    begin: Option<String>,
    end: Option<String>,
}

fn cmr_parse(body: &str) -> Option<Vec<CmrGranule>> {
    let json = parse_json(body)?;
    let JsonVal::Arr(items) = jpath_val(&json, "items")? else {
        return None;
    };
    let mut out = Vec::new();
    for item in items {
        let ur = match jpath_val(item, "umm.GranuleUR") {
            Some(JsonVal::Str(s)) if !s.is_empty() => s.clone(),
            _ => continue,
        };
        let begin = match jpath_val(item, "umm.TemporalExtent.RangeDateTime.BeginningDateTime") {
            Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
            _ => None,
        };
        let end = match jpath_val(item, "umm.TemporalExtent.RangeDateTime.EndingDateTime") {
            Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
            _ => None,
        };
        let JsonVal::Arr(urls) = jpath_val(item, "umm.RelatedUrls")? else {
            continue;
        };
        let mut chosen = None;
        for u in urls {
            let JsonVal::Obj(m) = u else { continue };
            let ty = match m.get("Type") {
                Some(JsonVal::Str(s)) => s.as_str(),
                _ => continue,
            };
            let url = match m.get("URL") {
                Some(JsonVal::Str(s)) => s.as_str(),
                _ => continue,
            };
            if ty == "GET DATA" && url.starts_with(LP_PROD_PREFIX) && url.ends_with(".hdf") {
                chosen = Some(url.to_string());
                break;
            }
        }
        let Some(url) = chosen else { continue };
        out.push(CmrGranule {
            ur,
            url,
            begin,
            end,
        });
    }
    Some(out)
}

fn cmr_granules(
    concept_id: &str,
    temporal: Option<&str>,
    page_size: usize,
) -> Option<Vec<CmrGranule>> {
    let query = cmr_query(concept_id, temporal, page_size);
    let url = format!("{CMR_UMM_GRANULES_URL}?{query}");
    cmr_parse(&cmr_fetch(&url)?)
}

fn granule_anchor(begin: &str, end: &str, lsk: &LeapSeconds) -> Option<f64> {
    let b = parse_iso_tdb(begin, lsk)?;
    let e = parse_iso_tdb(end, lsk)?;
    if !(b.is_finite() && e.is_finite() && e > b) {
        return None;
    }
    Some((b + e) / 2.0)
}

fn hdf4_magic(bytes: &[u8]) -> bool {
    bytes.get(0..4) == Some(&HDF4_MAGIC[..])
}

fn ndg_count(bytes: &[u8]) -> Option<usize> {
    let hdf = Hdf4::parse(bytes)?;
    Some(hdf.dds().iter().filter(|dd| dd.tag == DFTAG_NDG).count())
}

fn bearer_probe(url: &str, token: &str) -> Option<Vec<u8>> {
    match fetch_bearer_range(url, 0, PROBE_WINDOW, token) {
        Some(b) if hdf4_magic(&b) => Some(b),
        Some(_) => {
            eprintln!(
                "modis_lst_cmg: {} the {} B probe carries no HDF4 magic — escalating once",
                url, PROBE_WINDOW
            );
            fetch_bearer_range(url, 0, ESCALATION_WINDOW, token)
        }
        None => None,
    }
}

fn harvest_granule(url: &str, token: &str) -> Vec<[f64; REC_FIELDS]> {
    let bytes = match fs::read(url) {
        Ok(b) => b,
        Err(_) => match bearer_probe(url, token) {
            Some(b) => b,
            None => {
                eprintln!(
                    "modis_lst_cmg: {} the bearer probe returned void — granule stays pending",
                    url
                );
                return Vec::new();
            }
        },
    };
    if !hdf4_magic(&bytes) {
        eprintln!(
            "modis_lst_cmg: {} carries no HDF4 magic — granule stays pending",
            url
        );
        return Vec::new();
    }
    match ndg_count(&bytes) {
        Some(ndg) => {
            eprintln!(
                "modis_lst_cmg: {} HDF4 container parses with {} NDG data descriptors — the HDF4 SD-API reader (SDstart/SDselect/SDread over DFTAG_NDG 720) is unbuilt in src/archivar/hdf4.rs; LST_Day_CMG/LST_Night_CMG stay pending — no LST values synthesized (0 honored)",
                url, ndg
            );
        }
        None => {
            eprintln!(
                "modis_lst_cmg: {} HDF4 magic present, the DD-chain walk returned void within {} B — the container layer (src/archivar/hdf4.rs) does not cover this EOS DD-list continuation; the HDF4 SD-API reader (SDstart/SDselect/SDread over DFTAG_NDG 720) is the missing block — no LST values synthesized (0 honored)",
                url,
                bytes.len()
            );
        }
    }
    Vec::new()
}

fn pack(recs: &[[f64; REC_FIELDS]]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8 + recs.len() * REC_BYTES);
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&(recs.len() as u32).to_le_bytes());
    for r in recs {
        for v in r {
            buf.extend_from_slice(&v.to_le_bytes());
        }
    }
    buf
}

fn unpack(bytes: &[u8]) -> Option<Vec<[f64; REC_FIELDS]>> {
    if bytes.len() < 8 || bytes[..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    for _ in 0..n {
        let mut r = [0f64; REC_FIELDS];
        for slot in r.iter_mut() {
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
            off += 8;
        }
        if !(r[4] == COMP_DAY || r[4] == COMP_NIGHT) {
            return None;
        }
        out.push(r);
    }
    Some(out)
}

struct Granule {
    url: String,
    anchor: Option<f64>,
}

fn run(args: &[String]) {
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => {
            eprintln!(
                "modis_lst_cmg_compiler: --out <file.bin> absent — the output path is never silent"
            );
            std::process::exit(2);
        }
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(token) = edl_token() else {
        eprintln!(
            "modis_lst_cmg_compiler: EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token"
        );
        std::process::exit(2);
    };
    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "modis_lst_cmg_compiler: the embedded naif0012.tls leap table is absent — no time anchor folds to the TDB clock"
        );
        std::process::exit(1);
    };
    let granules: Vec<Granule> = if args.iter().any(|a| a == "--cmr") {
        let Some(concept_id) = arg_value(args, "--concept-id") else {
            eprintln!("modis_lst_cmg_compiler: --cmr carries no --concept-id — refused");
            std::process::exit(2);
        };
        let temporal = arg_value(args, "--temporal");
        let limit = match arg_value(args, "--limit") {
            Some(v) => match v.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => {
                    eprintln!("modis_lst_cmg_compiler: --limit {v} is no count — refused");
                    std::process::exit(2);
                }
            },
            None => None,
        };
        let Some(found) = cmr_granules(&concept_id, temporal.as_deref(), CMR_PAGE_SIZE) else {
            eprintln!(
                "modis_lst_cmg_compiler: the CMR granule search returned void for {concept_id}"
            );
            std::process::exit(1);
        };
        if found.is_empty() {
            eprintln!(
                "modis_lst_cmg_compiler: no CMR granules for {concept_id} — nothing fabricated"
            );
            std::process::exit(1);
        }
        let mut found = found;
        found.sort_by(|a, b| a.ur.cmp(&b.ur));
        if let Some(n) = limit {
            found.truncate(n);
        }
        found
            .into_iter()
            .map(|g| {
                let anchor = match (&g.begin, &g.end) {
                    (Some(b), Some(e)) => granule_anchor(b, e, &lsk),
                    _ => None,
                };
                Granule { url: g.url, anchor }
            })
            .collect()
    } else {
        let direct: Vec<String> = args
            .iter()
            .enumerate()
            .filter(|(_, a)| a.as_str() == "--granule")
            .filter_map(|(i, _)| args.get(i + 1))
            .cloned()
            .collect();
        if direct.is_empty() {
            eprintln!(
                "usage: modis_lst_cmg_compiler (--cmr --concept-id <id> [--temporal <start>,<end>] [--limit N] | --granule <url|path> [...]) --out <file.bin> [--ci-mode] — refused"
            );
            std::process::exit(2);
        }
        direct
            .into_iter()
            .map(|url| Granule { url, anchor: None })
            .collect()
    };
    let mut recs: Vec<[f64; REC_FIELDS]> = Vec::new();
    for g in &granules {
        match g.anchor {
            Some(a) => eprintln!("modis_lst_cmg: {} temporal midpoint TDB {a:.6}", g.url),
            None => eprintln!(
                "modis_lst_cmg: {} carries no temporal extent — anchor pending",
                g.url
            ),
        }
        let before = recs.len();
        recs.extend(harvest_granule(&g.url, &token));
        eprintln!("modis_lst_cmg: {} → {} records", g.url, recs.len() - before);
    }
    recs.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if recs.is_empty() {
        eprintln!("modis_lst_cmg_compiler: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = pack(&recs);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() && fs::create_dir_all(parent).is_err() {
            eprintln!(
                "modis_lst_cmg_compiler: create_dir_all for {} returned void",
                parent.display()
            );
            std::process::exit(1);
        }
    }
    if fs::write(&out_path, &bytes).is_err() {
        eprintln!("modis_lst_cmg_compiler: write {} returned void", out_path);
        std::process::exit(1);
    }
    match unpack(&bytes) {
        Some(parsed) => eprintln!(
            "modis_lst_cmg_compiler: {} {} records, {} B, roundtrip parses",
            out_path,
            parsed.len(),
            bytes.len()
        ),
        None => {
            eprintln!("modis_lst_cmg_compiler: {} roundtrip parse void", out_path);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    run(&args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_unpack_roundtrip() {
        let recs = vec![
            [750_000_000.0, -30.5, 10.25, 288.4, COMP_DAY],
            [750_000_001.0, -30.5, 10.25, 280.1, COMP_NIGHT],
        ];
        let bytes = pack(&recs);
        assert_eq!(bytes.len(), 8 + 2 * REC_BYTES);
        let parsed = unpack(&bytes).expect("the packed bin parses");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], recs[0]);
        assert_eq!(parsed[1], recs[1]);
    }

    #[test]
    fn unpack_rejects_foreign_bytes() {
        assert!(unpack(b"X").is_none());
        assert!(unpack(b"MCM2abcd").is_none());
        let mut short = pack(&[[1.0, 2.0, 3.0, 4.0, COMP_DAY]]);
        short.truncate(short.len() - 1);
        assert!(unpack(&short).is_none());
        let foreign_comp = pack(&[[1.0, 2.0, 3.0, 4.0, 3.0]]);
        assert!(unpack(&foreign_comp).is_none());
    }

    #[test]
    fn cmr_parse_takes_the_lp_prod_get_data_url() {
        let body = r#"{"hits":2,"items":[{"umm":{"GranuleUR":"MOD11C1.A2000058.061.2020058191556",
            "TemporalExtent":{"RangeDateTime":{"BeginningDateTime":"2000-02-27T00:00:00.000Z","EndingDateTime":"2000-02-27T23:59:59.000Z"}},
            "RelatedUrls":[
                {"URL":"s3://lp-prod-protected/MOD11C1.061/x.hdf","Type":"GET DATA VIA DIRECT ACCESS"},
                {"URL":"https://data.lpdaac.earthdatacloud.nasa.gov/lp-prod-protected/MOD11C1.061/MOD11C1.A2000058.061.2020058191556/MOD11C1.A2000058.061.2020058191556.hdf","Type":"GET DATA"},
                {"URL":"https://example.com/other.hdf","Type":"GET DATA"}
            ]}}]}"#;
        let granules = cmr_parse(body).expect("the umm feed parses");
        assert_eq!(granules.len(), 1);
        assert_eq!(granules[0].ur, "MOD11C1.A2000058.061.2020058191556");
        assert_eq!(
            granules[0].url,
            "https://data.lpdaac.earthdatacloud.nasa.gov/lp-prod-protected/MOD11C1.061/MOD11C1.A2000058.061.2020058191556/MOD11C1.A2000058.061.2020058191556.hdf"
        );
        assert_eq!(
            granules[0].begin.as_deref(),
            Some("2000-02-27T00:00:00.000Z")
        );
        assert_eq!(granules[0].end.as_deref(), Some("2000-02-27T23:59:59.000Z"));
    }

    #[test]
    fn cmr_query_sorts_and_encodes() {
        assert_eq!(
            cmr_query("C2565788888-LPCLOUD", None, 1 << 5),
            "concept_id=C2565788888-LPCLOUD&page_size=32"
        );
        assert_eq!(
            cmr_query(
                "C2565788888-LPCLOUD",
                Some("2024-01-01T00:00:00Z,2024-01-02T00:00:00Z"),
                10
            ),
            "concept_id=C2565788888-LPCLOUD&page_size=10&temporal=2024-01-01T00%3A00%3A00Z%2C2024-01-02T00%3A00%3A00Z"
        );
    }

    #[test]
    fn temporal_midpoint_folds_to_tdb() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let b = parse_iso_tdb("2000-02-27T00:00:00.000Z", &lsk).expect("begin folds");
        let e = parse_iso_tdb("2000-02-27T23:59:59.000Z", &lsk).expect("end folds");
        let mid = granule_anchor("2000-02-27T00:00:00.000Z", "2000-02-27T23:59:59.000Z", &lsk)
            .expect("the midpoint folds");
        assert!(b < mid && mid < e);
        assert!((mid - (b + e) / 2.0).abs() < 1e-9);
        assert!(
            granule_anchor("2000-02-27T23:59:59.000Z", "2000-02-27T00:00:00.000Z", &lsk).is_none()
        );
    }

    #[test]
    fn ndg_descriptors_are_counted() {
        let mut file = Vec::new();
        file.extend_from_slice(&HDF4_MAGIC);
        file.extend_from_slice(&2u16.to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        let off1 = 4 + 6 + 24;
        let off2 = off1 + 4;
        file.extend_from_slice(&720u16.to_be_bytes());
        file.extend_from_slice(&1u16.to_be_bytes());
        file.extend_from_slice(&(off1 as i32).to_be_bytes());
        file.extend_from_slice(&4i32.to_be_bytes());
        file.extend_from_slice(&720u16.to_be_bytes());
        file.extend_from_slice(&2u16.to_be_bytes());
        file.extend_from_slice(&(off2 as i32).to_be_bytes());
        file.extend_from_slice(&4i32.to_be_bytes());
        file.extend_from_slice(&[1, 2, 3, 4]);
        file.extend_from_slice(&[5, 6, 7, 8]);
        assert!(hdf4_magic(&file));
        assert_eq!(ndg_count(&file), Some(2));
        assert!(ndg_count(b"not hdf4").is_none());
    }

    #[test]
    fn magic_present_with_a_broken_dd_chain_is_distinguished() {
        let mut file = Vec::new();
        file.extend_from_slice(&HDF4_MAGIC);
        file.extend_from_slice(&200u16.to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        file.extend_from_slice(&[0u8; 32]);
        assert!(hdf4_magic(&file));
        assert!(ndg_count(&file).is_none());
    }
}
