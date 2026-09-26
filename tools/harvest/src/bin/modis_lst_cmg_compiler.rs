use omegaflow::archivar::json::{JsonVal, jpath_val, parse_json};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{LeapSeconds, embedded_lsk, parse_iso_tdb};
use omegaflow::cdn::{CDN_REPO, upload_release};
use omegaflow::hdf4::{Hdf4, read_num, type_size};
use omegaflow::odf::PODF_SHARD_LIMIT;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::Command;

const NETLOC: &str = "data.lpdaac.earthdatacloud.nasa.gov";
const CMR_UMM_GRANULES_URL: &str = "https://cmr.earthdata.nasa.gov/search/granules.umm_json";
const LP_PROD_PREFIX: &str = "https://data.lpdaac.earthdatacloud.nasa.gov/lp-prod-protected/";
const CMR_PAGE_SIZE: usize = 1 << 5;
const CMR_MAX_TIME_S: u64 = 1 << 7;
const CONNECT_BOUND_S: u64 = 1 << 5;
const MAGIC: [u8; 4] = *b"MCM1";
const REC_FIELDS: usize = 5;
const REC_BYTES: usize = REC_FIELDS * 8;
const COMP_DAY: f64 = 1.0;
const COMP_NIGHT: f64 = 2.0;
const HDF4_MAGIC: [u8; 4] = [0x0e, 0x03, 0x13, 0x01];
const CMG_CELL: f64 = 0.05;

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

fn granule_day_midpoint(url: &str, lsk: &LeapSeconds) -> Option<f64> {
    let base = url.rsplit('/').next()?;
    let a = base.find(".A")?;
    let rest = base.get(a + 2..)?;
    let year = rest.get(0..4)?.parse::<i64>().ok()?;
    let doy = rest.get(4..7)?.parse::<i64>().ok()?;
    if !(year >= 1 && doy >= 1 && doy <= 366) {
        return None;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let mut day = doy;
    let mut month = 0i64;
    for (i, days) in [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ]
    .iter()
    .enumerate()
    {
        if day <= *days {
            month = i as i64 + 1;
            break;
        }
        day -= *days;
    }
    if month == 0 {
        return None;
    }
    let iso = format!("{year:04}-{month:02}-{day:02}T12:00:00.000Z");
    parse_iso_tdb(&iso, lsk)
}

fn bearer_probe(url: &str, token: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("600")
        .arg("-H")
        .arg(format!("Authorization: Bearer {token}"))
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        eprintln!("modis_lst_cmg: {} whole fetch returned {}", url, out.status);
        None
    }
}

fn harvest_granule(url: &str, t: Option<f64>, token: &str) -> Vec<[f64; REC_FIELDS]> {
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
    let Some(hdf) = Hdf4::parse(&bytes) else {
        eprintln!(
            "modis_lst_cmg: {} the DD-chain walk returned void within {} B — the container does not parse",
            url,
            bytes.len()
        );
        return Vec::new();
    };
    let Some(t) = t else {
        eprintln!(
            "modis_lst_cmg: {} carries no temporal anchor — granule stays pending",
            url
        );
        return Vec::new();
    };
    let mut recs: Vec<[f64; REC_FIELDS]> = Vec::new();
    for sds in hdf.sds() {
        let Some(name) = sds.name.as_deref() else {
            continue;
        };
        let comp = match name {
            "LST_Day_CMG" => COMP_DAY,
            "LST_Night_CMG" => COMP_NIGHT,
            _ => continue,
        };
        if sds.dims.len() != 2 {
            eprintln!(
                "modis_lst_cmg: {} {name} carries {} dims — the CMG grid expects 2",
                url,
                sds.dims.len()
            );
            continue;
        }
        let rows = sds.dims[0] as usize;
        let cols = sds.dims[1] as usize;
        let Some(esize) = type_size(sds.typ) else {
            eprintln!(
                "modis_lst_cmg: {} {name} type {} carries no byte size",
                url, sds.typ
            );
            continue;
        };
        let Some(expected) = rows.checked_mul(cols).and_then(|n| n.checked_mul(esize)) else {
            eprintln!("modis_lst_cmg: {} {name} dims overflow the data size", url);
            continue;
        };
        if sds.data.len() != expected {
            eprintln!(
                "modis_lst_cmg: {} {name} data {} B vs dims {}x{}x{} = {} B — records stay absent",
                url,
                sds.data.len(),
                rows,
                cols,
                esize,
                expected
            );
            continue;
        }
        let before = recs.len();
        for r in 0..rows {
            let lat = 90.0 - CMG_CELL * (r as f64 + 0.5);
            let row_off = r * cols * esize;
            for c in 0..cols {
                let idx = row_off + c * esize;
                let Some(raw) = read_num(&sds.data, idx, sds.typ) else {
                    continue;
                };
                if let Some(f) = sds.fill {
                    if raw == f {
                        continue;
                    }
                }
                if let Some((lo, hi)) = sds.range {
                    if raw < lo || raw > hi {
                        continue;
                    }
                }
                let val = match (sds.scale, sds.offset) {
                    (Some(s), Some(o)) => raw * s + o,
                    (Some(s), None) => raw * s,
                    (None, Some(o)) => raw + o,
                    (None, None) => raw,
                };
                if !val.is_finite() || val <= 0.0 {
                    continue;
                }
                let lon = -180.0 + CMG_CELL * (c as f64 + 0.5);
                recs.push([t, lat, lon, val, comp]);
            }
        }
        eprintln!(
            "modis_lst_cmg: {} {name} type {} dims {}x{} scale {:?} offset {:?} fill {:?} range {:?} → {} records",
            url,
            sds.typ,
            rows,
            cols,
            sds.scale,
            sds.offset,
            sds.fill,
            sds.range,
            recs.len() - before
        );
    }
    recs
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

fn shard_stem(url: &str) -> Option<String> {
    let base = url.rsplit('/').next()?;
    let stem = base.strip_suffix(".hdf").unwrap_or(base);
    if stem.is_empty() {
        return None;
    }
    Some(stem.to_string())
}

fn cdn_digests(tag: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let out = match Command::new("gh")
        .arg("release")
        .arg("view")
        .arg(tag)
        .arg("--repo")
        .arg(CDN_REPO)
        .arg("--json")
        .arg("assets")
        .arg("--jq")
        .arg(".assets[] | \"\\(.name) \\(.digest)\"")
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => {
            eprintln!(
                "modis_lst_cmg: gh release view {tag} returned void — the resume digest map stays empty"
            );
            return map;
        }
    };
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let Some((name, digest)) = line.split_once(' ') else {
            continue;
        };
        let Some(hex) = digest.strip_prefix("sha256:") else {
            continue;
        };
        if hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            map.insert(name.to_string(), hex.to_string());
        }
    }
    map
}

fn verify_bin_file(path: &str) -> Option<usize> {
    let mut file = File::open(path).ok()?;
    let len = file.metadata().ok()?.len();
    if len < 8 {
        return None;
    }
    let mut head = [0u8; 8];
    file.read_exact(&mut head).ok()?;
    if head[..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(head[4..8].try_into().ok()?) as usize;
    if len != (8 + n * REC_BYTES) as u64 {
        return None;
    }
    let mut buf = vec![0u8; REC_BYTES * 8192];
    let mut remaining = n * REC_BYTES;
    while remaining > 0 {
        let want = remaining.min(buf.len());
        file.read_exact(&mut buf[..want]).ok()?;
        for chunk in buf[..want].chunks_exact(REC_BYTES) {
            let comp = f64::from_le_bytes(chunk[32..40].try_into().ok()?);
            if !(comp == COMP_DAY || comp == COMP_NIGHT) {
                return None;
            }
        }
        remaining -= want;
    }
    Some(n)
}

struct Granule {
    url: String,
    anchor: Option<f64>,
}

fn boot() -> (String, LeapSeconds) {
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
    (token, lsk)
}

fn collect_granules(args: &[String], lsk: &LeapSeconds) -> Vec<Granule> {
    if args.iter().any(|a| a == "--cmr") {
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
                    (Some(b), Some(e)) => granule_anchor(b, e, lsk),
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
                "usage: modis_lst_cmg_compiler (--cmr --concept-id <id> [--temporal <start>,<end>] [--limit N] | --granule <url|path> [...]) (--out <file.bin> | --shard-dir <dir> --manifest <file>) [--series <name>] [--ci-mode] — refused"
            );
            std::process::exit(2);
        }
        direct
            .into_iter()
            .map(|url| Granule { url, anchor: None })
            .collect()
    }
}

fn run_series(
    granules: &[Granule],
    shard_dir: &str,
    manifest: &str,
    series: &str,
    ci_mode: bool,
    token: &str,
    lsk: &LeapSeconds,
) {
    if fs::create_dir_all(shard_dir).is_err() {
        eprintln!("modis_lst_cmg_compiler: create_dir_all for {shard_dir} returned void");
        std::process::exit(1);
    }
    let digests = if ci_mode {
        cdn_digests(NETLOC)
    } else {
        HashMap::new()
    };
    let mut lines: Vec<(String, String)> = Vec::new();
    let mut pending = 0usize;
    for g in granules {
        let Some(stem) = shard_stem(&g.url) else {
            eprintln!(
                "modis_lst_cmg: {} carries no shard stem — granule stays pending",
                g.url
            );
            pending += 1;
            continue;
        };
        let name = format!("{series}_{stem}.bin");
        let path = format!("{shard_dir}/{name}");
        if Path::new(&path).exists() {
            match fs::read(&path) {
                Ok(bytes) if !bytes.is_empty() => {
                    lines.push((name.clone(), sha256_hex(&bytes)));
                    eprintln!("modis_lst_cmg: {name} present on disk — resumed");
                    continue;
                }
                _ => {}
            }
        }
        if ci_mode {
            if let Some(hex) = digests.get(&name) {
                lines.push((name.clone(), hex.clone()));
                eprintln!("modis_lst_cmg: {name} present on the CDN — resumed");
                continue;
            }
        }
        match g.anchor {
            Some(a) => eprintln!("modis_lst_cmg: {} temporal midpoint TDB {a:.6}", g.url),
            None => eprintln!(
                "modis_lst_cmg: {} carries no temporal extent — anchor pending",
                g.url
            ),
        }
        let t = g.anchor.or_else(|| granule_day_midpoint(&g.url, lsk));
        let recs = harvest_granule(&g.url, t, token);
        if recs.is_empty() {
            eprintln!(
                "modis_lst_cmg: {} → no records — shard stays unwritten (0 honored)",
                g.url
            );
            pending += 1;
            continue;
        }
        let bytes = pack(&recs);
        drop(recs);
        if bytes.len() > PODF_SHARD_LIMIT {
            eprintln!(
                "modis_lst_cmg: {name} {} B exceed the {PODF_SHARD_LIMIT}-byte CDN asset limit — shard stays unwritten (0 honored)",
                bytes.len()
            );
            pending += 1;
            continue;
        }
        match unpack(&bytes) {
            Some(parsed) => eprintln!(
                "modis_lst_cmg: {name} {} records, {} B, roundtrip parses",
                parsed.len(),
                bytes.len()
            ),
            None => {
                eprintln!("modis_lst_cmg: {name} roundtrip parse void — shard stays pending");
                pending += 1;
                continue;
            }
        }
        let sha = sha256_hex(&bytes);
        if fs::write(&path, &bytes).is_err() {
            eprintln!("modis_lst_cmg: write {path} returned void");
            pending += 1;
            continue;
        }
        lines.push((name.clone(), sha));
        if ci_mode {
            if !upload_release(NETLOC, &path) {
                pending += 1;
                continue;
            }
            if fs::remove_file(&path).is_err() {
                eprintln!("modis_lst_cmg: {path} local shard stays (remove void)");
            }
        }
    }
    if lines.is_empty() {
        eprintln!("modis_lst_cmg_compiler: no shards — the series stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let manifest_path = format!("{shard_dir}/{manifest}");
    let mut text = String::new();
    for (name, sha) in &lines {
        text.push_str(&format!("{name} {sha}\n"));
    }
    if fs::write(&manifest_path, &text).is_err() {
        eprintln!("modis_lst_cmg: write {manifest_path} returned void");
        std::process::exit(1);
    }
    eprintln!("modis_lst_cmg: {manifest} lists {} shards", lines.len());
    if ci_mode {
        if pending > 0 {
            eprintln!(
                "modis_lst_cmg: {pending} granules pending — {manifest} stays unuploaded, the resume keeps the series honest"
            );
            std::process::exit(1);
        }
        if !upload_release(NETLOC, &manifest_path) {
            std::process::exit(1);
        }
        if fs::remove_file(&manifest_path).is_err() {
            eprintln!("modis_lst_cmg: {manifest_path} local manifest stays (remove void)");
        }
    }
}

fn run_out(granules: &[Granule], out_path: &str, ci_mode: bool, token: &str, lsk: &LeapSeconds) {
    if let Some(parent) = Path::new(out_path).parent() {
        if !parent.as_os_str().is_empty() && fs::create_dir_all(parent).is_err() {
            eprintln!(
                "modis_lst_cmg_compiler: create_dir_all for {} returned void",
                parent.display()
            );
            std::process::exit(1);
        }
    }
    let mut file = match File::create(out_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("modis_lst_cmg_compiler: create {out_path} returned void");
            std::process::exit(1);
        }
    };
    if file.write_all(&MAGIC).is_err() || file.write_all(&0u32.to_le_bytes()).is_err() {
        eprintln!("modis_lst_cmg_compiler: header write {out_path} returned void");
        std::process::exit(1);
    }
    let mut total = 0usize;
    for g in granules {
        match g.anchor {
            Some(a) => eprintln!("modis_lst_cmg: {} temporal midpoint TDB {a:.6}", g.url),
            None => eprintln!(
                "modis_lst_cmg: {} carries no temporal extent — anchor pending",
                g.url
            ),
        }
        let t = g.anchor.or_else(|| granule_day_midpoint(&g.url, lsk));
        let recs = harvest_granule(&g.url, t, token);
        let before = total;
        if !recs.is_empty() {
            let bytes = pack(&recs);
            if file.write_all(&bytes[8..]).is_err() {
                eprintln!("modis_lst_cmg_compiler: append {out_path} returned void");
                std::process::exit(1);
            }
        }
        total += recs.len();
        eprintln!("modis_lst_cmg: {} → {} records", g.url, total - before);
    }
    if total == 0 {
        drop(file);
        if fs::remove_file(out_path).is_err() {
            eprintln!("modis_lst_cmg_compiler: {out_path} removal void");
        }
        eprintln!("modis_lst_cmg_compiler: no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let Ok(count) = u32::try_from(total) else {
        eprintln!(
            "modis_lst_cmg_compiler: {total} records exceed the u32 count slot — {out_path} stays unfinished"
        );
        std::process::exit(1);
    };
    if file.seek(SeekFrom::Start(4)).is_err() || file.write_all(&count.to_le_bytes()).is_err() {
        eprintln!("modis_lst_cmg_compiler: count patch {out_path} returned void");
        std::process::exit(1);
    }
    drop(file);
    let total_bytes = 8 + total * REC_BYTES;
    match verify_bin_file(out_path) {
        Some(n) if n == total => eprintln!(
            "modis_lst_cmg_compiler: {out_path} {total} records, {total_bytes} B, roundtrip parses"
        ),
        _ => {
            eprintln!("modis_lst_cmg_compiler: {out_path} roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode {
        if (total_bytes as u64) > (PODF_SHARD_LIMIT as u64) {
            eprintln!(
                "modis_lst_cmg_compiler: {out_path} {total_bytes} B exceed the {PODF_SHARD_LIMIT}-byte CDN asset limit — the asset stays unuploaded"
            );
            std::process::exit(1);
        }
        if !upload_release(NETLOC, out_path) {
            std::process::exit(1);
        }
    }
}

fn run(args: &[String]) {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let shard_dir = arg_value(args, "--shard-dir");
    let manifest = arg_value(args, "--manifest");
    let out_path = arg_value(args, "--out");
    if shard_dir.is_some() || manifest.is_some() {
        let (Some(dir), Some(man)) = (shard_dir.as_deref(), manifest.as_deref()) else {
            eprintln!(
                "usage: modis_lst_cmg_compiler (--cmr --concept-id <id> [--temporal <start>,<end>] [--limit N] | --granule <url|path> [...]) --shard-dir <dir> --manifest <file> [--series <name>] [--ci-mode] — the shard dir and the manifest are never split"
            );
            std::process::exit(2);
        };
        if out_path.is_some() {
            eprintln!("modis_lst_cmg_compiler: --out and --shard-dir exclude each other — refused");
            std::process::exit(2);
        }
        let Some(series) = arg_value(args, "--series") else {
            eprintln!(
                "modis_lst_cmg_compiler: --series <name> absent — the shard stem carries no series name"
            );
            std::process::exit(2);
        };
        let (token, lsk) = boot();
        let granules = collect_granules(args, &lsk);
        run_series(&granules, dir, man, &series, ci_mode, &token, &lsk);
        return;
    }
    let Some(out) = out_path.as_deref() else {
        eprintln!(
            "modis_lst_cmg_compiler: --out <file.bin> absent — the output path is never silent"
        );
        std::process::exit(2);
    };
    let (token, lsk) = boot();
    let granules = collect_granules(args, &lsk);
    run_out(&granules, out, ci_mode, &token, &lsk);
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
        let hdf = Hdf4::parse(&file).expect("the synthetic chain parses");
        assert_eq!(hdf.dds().len(), 2);
        assert_eq!(hdf.dds().iter().filter(|dd| dd.tag == 720).count(), 2);
        assert!(Hdf4::parse(b"not hdf4").is_none());
    }

    #[test]
    fn magic_present_with_a_broken_dd_chain_is_distinguished() {
        let mut file = Vec::new();
        file.extend_from_slice(&HDF4_MAGIC);
        file.extend_from_slice(&200u16.to_be_bytes());
        file.extend_from_slice(&0i32.to_be_bytes());
        file.extend_from_slice(&[0u8; 32]);
        assert!(hdf4_magic(&file));
        assert!(Hdf4::parse(&file).is_none());
    }

    #[test]
    fn granule_name_midday_folds() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let t = granule_day_midpoint(
            "https://data.lpdaac.earthdatacloud.nasa.gov/lp-prod-protected/MOD11C1.061/MOD11C1.A2000058.061.2020058191556/MOD11C1.A2000058.061.2020058191556.hdf",
            &lsk,
        )
        .expect("the granule name folds");
        let day_start = parse_iso_tdb("2000-02-27T00:00:00.000Z", &lsk).expect("start folds");
        let day_end = parse_iso_tdb("2000-02-27T23:59:59.000Z", &lsk).expect("end folds");
        assert!(t > day_start && t < day_end);
        assert!(granule_day_midpoint("https://example.com/no-name.hdf", &lsk).is_none());
        assert!(
            granule_day_midpoint("https://example.com/MOD11C1.A2000001.061.hdf", &lsk).is_none()
        );
    }

    #[test]
    fn shard_stem_names_the_granule_file() {
        assert_eq!(
            shard_stem("https://data.lpdaac.earthdatacloud.nasa.gov/lp-prod-protected/MOD11C2.061/MOD11C2.A2000049.061.2020330085614/MOD11C2.A2000049.061.2020330085614.hdf").as_deref(),
            Some("MOD11C2.A2000049.061.2020330085614")
        );
        assert_eq!(
            shard_stem("data/MOD11C3.A2000061.061.2020182060603.hdf").as_deref(),
            Some("MOD11C3.A2000061.061.2020182060603")
        );
        assert_eq!(
            shard_stem("plain.granule").as_deref(),
            Some("plain.granule")
        );
        assert!(shard_stem("").is_none());
    }

    #[test]
    fn verify_bin_file_accepts_pack_and_names_corruption() {
        let recs = vec![
            [750_000_000.0, -30.5, 10.25, 288.4, COMP_DAY],
            [750_000_001.0, -30.5, 10.25, 280.1, COMP_NIGHT],
        ];
        let bytes = pack(&recs);
        let path =
            std::env::temp_dir().join(format!("modis_lst_cmg_verify_{}.bin", std::process::id()));
        let path_s = path.to_str().expect("the temp path is utf8");
        fs::write(&path, &bytes).expect("the temp bin writes");
        assert_eq!(verify_bin_file(path_s), Some(2));
        let mut foreign = bytes.clone();
        foreign[8 + 32] ^= 0xFF;
        fs::write(&path, &foreign).expect("the temp bin rewrites");
        assert!(verify_bin_file(path_s).is_none());
        let mut short = bytes.clone();
        short.truncate(8 + REC_BYTES - 1);
        fs::write(&path, &short).expect("the temp bin rewrites");
        assert!(verify_bin_file(path_s).is_none());
        fs::remove_file(&path).ok();
    }
}
