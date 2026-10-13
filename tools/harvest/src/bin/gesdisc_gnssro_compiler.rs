use omegaflow::archivar::json::{JsonVal, jpath_val, parse_json};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{LeapSeconds, embedded_lsk, parse_iso_tdb};
use omegaflow::cdn::upload_release;
use omegaflow::geo::{
    COMP_COSMIC_PRES, COMP_COSMIC_REFRACT, COMP_COSMIC_TEMP, GeoRec, MAGIC_COSMIC, parse_bin,
    write_bin,
};
use omegaflow::hdf5::{Hdf5File, decode_f32, decode_f64};
use std::env;
use std::fs;
use std::process::Command;

const NETLOC: &str = "data.gesdisc.earthdata.nasa.gov";
const COLLECTION: &str = "C3628339510-GES_DISC";
const PRODUCT: &str = "gnssro_cosmic2_ucar_l2b";
const CMR_GRANULES_URL: &str = "https://cmr.earthdata.nasa.gov/search/granules.umm_json";
const GESDISC_PREFIX: &str = "https://data.gesdisc.earthdata.nasa.gov/data/GNSSRO/";
const HDF5_MAGIC: [u8; 8] = [0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a];
const PROFILE_ALT: &str = "MSL_alt";
const CMR_PAGE_SIZE: usize = 1 << 5;
const CMR_MAX_TIME_S: u64 = 1 << 7;
const CONNECT_BOUND_S: u64 = 1 << 5;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_usize(args: &[String], name: &str) -> Option<usize> {
    match arg_value(args, name) {
        Some(v) => v.parse::<usize>().ok(),
        None => None,
    }
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
    let out = Command::new("curl")
        .arg("-s")
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
        .arg(CONNECT_BOUND_S.to_string())
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        eprintln!(
            "gesdisc_gnssro: cmr {} returned ({}) — {}",
            url,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn granule_fetch(url: &str, token: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl")
        .arg("-sSLf")
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
        eprintln!(
            "gesdisc_gnssro: fetch {} returned ({}) — the granule stays pending",
            url, out.status
        );
        None
    }
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
            _ => match jpath_val(item, "umm.TemporalExtent.SingleDateTime") {
                Some(JsonVal::Str(s)) if !s.is_empty() => Some(s.clone()),
                _ => None,
            },
        };
        let urls = match jpath_val(item, "umm.RelatedUrls") {
            Some(JsonVal::Arr(a)) => a,
            _ => continue,
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
            if ty == "GET DATA" && url.starts_with(GESDISC_PREFIX) && url.ends_with(".nc4") {
                chosen = Some(url.to_string());
                break;
            }
        }
        let Some(url) = chosen else { continue };
        out.push(CmrGranule { ur, url, begin });
    }
    Some(out)
}

fn cmr_granules(temporal: Option<&str>, page_size: usize) -> Option<Vec<CmrGranule>> {
    let query = cmr_query(COLLECTION, temporal, page_size);
    let url = format!("{CMR_GRANULES_URL}?{query}");
    cmr_parse(&cmr_fetch(&url)?)
}

fn h5_attr_num(file: &Hdf5File, ds: &str, name: &str) -> Option<f64> {
    let a = file.attribute(ds, name)?;
    match (a.datatype.class, a.datatype.size) {
        (1, 4) => decode_f32(&a.data, 0, a.datatype.endian).map(|x| x as f64),
        (1, 8) => decode_f64(&a.data, 0, a.datatype.endian),
        _ => None,
    }
}

fn apply_fill(raw: Vec<f64>, fill: Option<f64>) -> Vec<f64> {
    raw.into_iter()
        .map(|x| {
            if !x.is_finite() {
                return f64::NAN;
            }
            match fill {
                Some(f) if x == f => f64::NAN,
                _ => x,
            }
        })
        .collect()
}

fn h5_var(file: &Hdf5File, name: &str) -> Option<Vec<f64>> {
    let raw = file.read_f64_dataset(name).ok()?;
    let fill = h5_attr_num(file, name, "_FillValue");
    Some(apply_fill(raw, fill))
}

fn h5_first(file: &Hdf5File, name: &str) -> Option<f64> {
    h5_var(file, name)?.into_iter().find(|x| x.is_finite())
}

fn h5_position(file: &Hdf5File) -> (Option<f64>, Option<f64>) {
    let lat = h5_first(file, "lat")
        .or_else(|| h5_first(file, "Lat"))
        .or_else(|| h5_first(file, "latitude"))
        .or_else(|| h5_attr_num(file, "/", "occulting_lat"))
        .or_else(|| h5_attr_num(file, "/", "lat"));
    let lon = h5_first(file, "lon")
        .or_else(|| h5_first(file, "Lon"))
        .or_else(|| h5_first(file, "longitude"))
        .or_else(|| h5_attr_num(file, "/", "occulting_lon"))
        .or_else(|| h5_attr_num(file, "/", "lon"));
    (lat, lon)
}

fn comp_and_names(var: &str) -> Option<(u32, Vec<&'static str>)> {
    match var {
        "Ref" => Some((COMP_COSMIC_REFRACT, vec!["Ref", "Refractivity"])),
        "Temp" => Some((COMP_COSMIC_TEMP, vec!["Temp", "Dry_Temp"])),
        "Pres" => Some((COMP_COSMIC_PRES, vec!["Pres", "Pressure"])),
        _ => None,
    }
}

fn stats(vals: &[f64]) -> (usize, Option<f64>, Option<f64>) {
    let mut n = 0usize;
    let mut min: Option<f64> = None;
    let mut max: Option<f64> = None;
    for &x in vals {
        if !x.is_finite() {
            continue;
        }
        n += 1;
        min = Some(match min {
            Some(m) => m.min(x),
            None => x,
        });
        max = Some(match max {
            Some(m) => m.max(x),
            None => x,
        });
    }
    (n, min, max)
}

fn num_str(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => "absent".to_string(),
    }
}

fn harvest_granule(
    bytes: &[u8],
    begin: &str,
    comp: u32,
    var_names: &[&str],
    lsk: &LeapSeconds,
) -> Vec<GeoRec> {
    if !bytes.starts_with(&HDF5_MAGIC) {
        return Vec::new();
    }
    let Ok(file) = Hdf5File::parse(bytes) else {
        return Vec::new();
    };
    let Some(alt) = h5_var(&file, PROFILE_ALT) else {
        return Vec::new();
    };
    let Some(val) = var_names.iter().find_map(|n| h5_var(&file, n)) else {
        return Vec::new();
    };
    let (lat, lon) = h5_position(&file);
    let (Some(lat), Some(lon)) = (lat, lon) else {
        return Vec::new();
    };
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Vec::new();
    }
    let Some(t) = parse_iso_tdb(begin, lsk) else {
        return Vec::new();
    };
    if !t.is_finite() {
        return Vec::new();
    }
    let n = alt.len().min(val.len());
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let a = alt[k];
        let v = val[k];
        if !a.is_finite() || !v.is_finite() || a < 0.0 {
            continue;
        }
        out.push(GeoRec {
            t,
            lat,
            lon,
            alt: a,
            freq: 0.0,
            bin_width: 0.0,
            val: v,
            comp,
            station: 0,
        });
    }
    out
}

fn probe_granule(bytes: &[u8], name: &str) {
    if !bytes.starts_with(&HDF5_MAGIC) {
        eprintln!("{name}: unknown byte stream — not netCDF-4/HDF5");
        return;
    }
    let Ok(file) = Hdf5File::parse(bytes) else {
        eprintln!("{name}: netCDF-4 (HDF5) parse returned void");
        return;
    };
    let (lat, lon) = h5_position(&file);
    eprintln!(
        "{name}: netCDF-4 (HDF5); position lat {} lon {}",
        num_str(lat),
        num_str(lon)
    );
    match h5_var(&file, PROFILE_ALT) {
        Some(alt) => {
            let (n, min, max) = stats(&alt);
            eprintln!(
                "  {}  levels {n}  min {}  max {}",
                PROFILE_ALT,
                num_str(min),
                num_str(max)
            );
        }
        None => eprintln!("  {}  absent", PROFILE_ALT),
    }
    for var in ["Ref", "Temp", "Pres"] {
        match h5_var(&file, var) {
            Some(vals) => {
                let (n, min, max) = stats(&vals);
                eprintln!(
                    "  {var}  levels {n}  min {}  max {}",
                    num_str(min),
                    num_str(max)
                );
            }
            None => eprintln!("  {var}  absent"),
        }
    }
}

fn naming_scheme() {
    eprintln!("GES DISC GNSS-RO ({NETLOC}):");
    eprintln!(
        "  collection {COLLECTION} = {PRODUCT}.2.0 (COSMIC-2 L2B temperature, pressure, water vapor, contributed by UCAR)"
    );
    eprintln!("  granule discovery: {CMR_GRANULES_URL}?concept_id={COLLECTION}&page_size=N");
    eprintln!(
        "  granule access: https://data.gesdisc.earthdata.nasa.gov/data/GNSSRO/{PRODUCT}.2.0/{{YYYY}}/{{MM}}/{{DD}}/{PRODUCT}_{{...}}_{{YYYYMMDDHHMM}}.nc4"
    );
    eprintln!(
        "  auth: EARTHDATA_EDL_TOKEN from the environment or .secrets.local (header Authorization)"
    );
    eprintln!("  variables: {PROFILE_ALT}, Ref (n-units), Temp (K), Pres (hPa), lat/lon");
}

fn run_probe(args: &[String]) -> Result<(), String> {
    let granule = arg_value(args, "--granule")
        .ok_or_else(|| "gesdisc_gnssro_compiler: --probe needs --granule <file|url>".to_string())?;
    let bytes = if granule.starts_with("http://") || granule.starts_with("https://") {
        let Some(token) = edl_token() else {
            return Err(
                "EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token (0 honored)"
                    .to_string(),
            );
        };
        granule_fetch(&granule, &token)
            .ok_or_else(|| format!("{granule}: read void — the granule stays pending"))?
    } else {
        fs::read(&granule).map_err(|e| format!("read {granule} returned void: {e}"))?
    };
    probe_granule(&bytes, &granule);
    Ok(())
}

fn run_out_bin(args: &[String]) -> Result<(), String> {
    let var = match arg_value(args, "--var") {
        Some(v) => v,
        None => "Ref".to_string(),
    };
    let (comp, var_names) = comp_and_names(&var)
        .ok_or_else(|| format!("--var {var}: unknown profile variable (Ref|Temp|Pres)"))?;
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/{PRODUCT}_{}.bin", var.to_lowercase()),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let temporal = arg_value(args, "--temporal");
    let max = arg_usize(args, "--max-granules");
    let Some(lsk) = embedded_lsk() else {
        return Err("naif0012 table void — the TDB epoch stays void (no fabricated epoch)".into());
    };
    let Some(token) = edl_token() else {
        return Err(
            "EARTHDATA_EDL_TOKEN absent — the environment and .secrets.local carry no token (0 honored)"
                .into(),
        );
    };
    let Some(found) = cmr_granules(temporal.as_deref(), CMR_PAGE_SIZE) else {
        return Err("the CMR granule search returned void".into());
    };
    if found.is_empty() {
        return Err("no CMR granule for the collection — nothing fabricated".into());
    }
    let mut records: Vec<GeoRec> = Vec::new();
    let mut read = 0usize;
    for g in &found {
        if let Some(cap) = max {
            if read >= cap {
                break;
            }
        }
        read += 1;
        let Some(begin) = g.begin.as_deref() else {
            eprintln!("{}: carries no temporal extent — skipped", g.ur);
            continue;
        };
        let Some(bytes) = granule_fetch(&g.url, &token) else {
            continue;
        };
        let recs = harvest_granule(&bytes, begin, comp, &var_names, &lsk);
        if recs.is_empty() {
            eprintln!("{}: carries no {var} level rows", g.ur);
        } else {
            records.extend(recs);
        }
    }
    if records.is_empty() {
        return Err("no profile level measured — the asset stays unwritten (0 honored)".into());
    }
    records.sort_by(|a, b| {
        a.t.total_cmp(&b.t)
            .then(a.lat.total_cmp(&b.lat))
            .then(a.comp.cmp(&b.comp))
            .then(a.alt.total_cmp(&b.alt))
    });
    let bytes = write_bin(MAGIC_COSMIC, &records);
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    fs::write(&out_path, &bytes).map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let digest = sha256_hex(&bytes);
    match parse_bin(MAGIC_COSMIC, &bytes) {
        Some(parsed) => eprintln!(
            "{out_path}: {} level records, {} B, sha256 {digest}, roundtrip parses ({var})",
            parsed.len(),
            bytes.len()
        ),
        None => return Err(format!("{out_path}: roundtrip parse returned void")),
    }
    let name = match std::path::Path::new(&out_path)
        .file_name()
        .and_then(|n| n.to_str())
    {
        Some(n) => n,
        None => return Err(format!("{out_path}: carries no file name")),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
    println!("format cosmic_ro");
    println!(
        "origin https://data.gesdisc.earthdata.nasa.gov/data/GNSSRO/{PRODUCT}.2.0/<YYYY>/<MM>/<DD>/<granule>.nc4"
    );
    println!("compiler tools/harvest/src/bin/gesdisc_gnssro_compiler.rs");
    println!("sha256 {digest}");
    println!("at earth");
    println!("ttl 604800");
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: gesdisc_gnssro_compiler <mode>");
        eprintln!("  --list");
        eprintln!("  --probe --granule <file|url>");
        eprintln!(
            "  --out-bin [--var Ref|Temp|Pres] [--temporal <start>,<end>] [--out <path>] [--max-granules N] [--ci-mode]"
        );
        std::process::exit(1);
    }
    let result = if args.iter().any(|a| a == "--out-bin") {
        run_out_bin(&args)
    } else if args.iter().any(|a| a == "--probe") {
        run_probe(&args)
    } else {
        naming_scheme();
        Ok(())
    };
    if let Err(msg) = result {
        eprintln!("gesdisc_gnssro_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmr_query_carries_concept_and_sorted_params() {
        let q = cmr_query(COLLECTION, None, 8);
        assert_eq!(q, "concept_id=C3628339510-GES_DISC&page_size=8");
        let q = cmr_query(
            COLLECTION,
            Some("2020-01-01T00:00:00Z,2020-01-02T00:00:00Z"),
            8,
        );
        assert!(q.contains("temporal=2020-01-01T00%3A00%3A00Z%2C2020-01-02T00%3A00%3A00Z"));
    }

    #[test]
    fn cmr_parse_reads_the_gesdisc_nc4_granule() {
        let body = r#"{"items":[{"umm":{"GranuleUR":"gnssro_cosmic2_ucar_l2b_0001","TemporalExtent":{"RangeDateTime":{"BeginningDateTime":"2019-10-01T00:00:53.000Z","EndingDateTime":"2019-10-01T00:01:00.000Z"}},"RelatedUrls":[{"Type":"GET DATA","URL":"https://data.gesdisc.earthdata.nasa.gov/data/GNSSRO/gnssro_cosmic2_ucar_l2b.2.0/2019/10/01/x.nc4"},{"Type":"METADATA","URL":"https://disc.gsfc.nasa.gov/x.html"}]}}]}"#;
        let found = cmr_parse(body).unwrap();
        assert_eq!(found.len(), 1);
        assert!(found[0].url.ends_with("x.nc4"));
        assert_eq!(found[0].begin.as_deref(), Some("2019-10-01T00:00:53.000Z"));
    }

    #[test]
    fn cmr_parse_reads_a_single_datetime_and_skips_foreign_hosts() {
        let body = r#"{"items":[{"umm":{"GranuleUR":"g1","TemporalExtent":{"SingleDateTime":"2019-10-01T00:00:53.000Z"},"RelatedUrls":[{"Type":"GET DATA","URL":"https://example.org/x.nc4"}]}}]}"#;
        let found = cmr_parse(body).unwrap();
        assert!(found.is_empty());
    }

    #[test]
    fn comp_mapping_names_the_profile_variables() {
        assert_eq!(
            comp_and_names("Ref").map(|(c, _)| c),
            Some(COMP_COSMIC_REFRACT)
        );
        assert_eq!(
            comp_and_names("Temp").map(|(_, v)| v),
            Some(vec!["Temp", "Dry_Temp"])
        );
        assert_eq!(
            comp_and_names("Pres").map(|(c, _)| c),
            Some(COMP_COSMIC_PRES)
        );
        assert!(comp_and_names("Shum").is_none());
    }

    #[test]
    fn apply_fill_masks_fill_and_non_finite() {
        let vals = apply_fill(vec![1.0, -999.0, f64::NAN], Some(-999.0));
        assert!(vals[0].is_finite());
        assert!(vals[1].is_nan());
        assert!(vals[2].is_nan());
    }
}
