use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::stac::{StacAsset, StacItem, parse_items};
use omegaflow::archivar::{LeapSeconds, embedded_lsk};
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::Hdf5File;

const STAC_ROOT: &str = "https://data.inpe.br/bdc/stac/v1";
const COLLECTION_ID: &str = "samet_daily-1";
const COLLECTION_LICENSE: &str = "Creative Commons Attribution 4.0 International";
const DEFAULT_BAND: &str = "tmax";
const NETLOC: &str = "data.inpe.br";
const FORMAT: &str = "inpe_stac_samet_daily";

const MAGIC: [u8; 4] = *b"INPE";
const HEADER_BYTES: usize = 8;
const RECORD_BYTES: usize = 20;

#[derive(Clone, Debug, PartialEq)]
struct InpeRecord {
    t: f64,
    value: f64,
    comp: u32,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_step(args: &[String], name: &str, default: usize) -> Result<usize, String> {
    match arg_value(args, name) {
        Some(v) => {
            let n = v
                .parse::<usize>()
                .map_err(|_| format!("{name} {v} carries no step"))?;
            if n == 0 {
                return Err(format!("{name} carries no positive step"));
            }
            Ok(n)
        }
        None => Ok(default),
    }
}

fn band_code(band: &str) -> Option<u32> {
    match band {
        "tmax" => Some(1),
        "tmin" => Some(2),
        "tmean" => Some(3),
        _ => None,
    }
}

fn json_string_field(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let start = body.find(&needle)? + needle.len();
    let after = &body[start..];
    let colon = after.find(':')?;
    let value = &after[colon + 1..];
    let open = value.find('"')?;
    let rest = &value[open + 1..];
    let close = rest.find('"')?;
    Some(rest[..close].to_string())
}

fn license_is_open(license: &str) -> bool {
    license.contains("Attribution 4.0")
        || license == "CC-BY-4.0"
        || license.contains("Creative Commons Zero")
        || license.contains("Public Domain")
}

fn find_asset<'a>(item: &'a StacItem, band: &str) -> Option<&'a StacAsset> {
    item.assets.iter().find(|a| a.key == band)
}

fn is_fill(v: f64, marker: Option<f64>) -> bool {
    match marker {
        Some(m) => m.is_finite() && (v - m).abs() <= m.abs() * 1e-6 + 1e-9,
        None => false,
    }
}

fn build_records(
    values: &[f64],
    t: f64,
    comp: u32,
    fill: Option<f64>,
    missing: Option<f64>,
    stride: usize,
    limit: usize,
) -> Vec<InpeRecord> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < values.len() {
        let v = values[i];
        if v.is_finite() && !is_fill(v, fill) && !is_fill(v, missing) {
            out.push(InpeRecord { t, value: v, comp });
            if out.len() >= limit {
                break;
            }
        }
        i += stride;
    }
    out
}

fn write_bin(records: &[InpeRecord]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        if !r.t.is_finite() || !r.value.is_finite() {
            return None;
        }
        out.extend_from_slice(&r.t.to_le_bytes());
        out.extend_from_slice(&r.value.to_le_bytes());
        out.extend_from_slice(&r.comp.to_le_bytes());
    }
    Some(out)
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<InpeRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(RECORD_BYTES)? {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let value = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let comp = u32::from_le_bytes(bytes.get(off + 16..off + 20)?.try_into().ok()?);
        if !t.is_finite() || !value.is_finite() {
            return None;
        }
        out.push(InpeRecord { t, value, comp });
        off += RECORD_BYTES;
    }
    Some(out)
}

fn fetch(url: &str) -> Result<Vec<u8>, String> {
    fetch_raw_bytes(url).ok_or_else(|| format!("{url}: fetch void — the harvest stays unwritten"))
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let band = match arg_value(args, "--band") {
        Some(v) => v,
        None => DEFAULT_BAND.to_string(),
    };
    let collection = match arg_value(args, "--collection") {
        Some(v) => v,
        None => COLLECTION_ID.to_string(),
    };
    let stride = parse_step(args, "--stride", 1)?;
    let limit = parse_step(args, "--limit", usize::MAX)?;
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };
    let Some(comp) = band_code(&band) else {
        return Err(format!("band {band} carries no companion code — refused"));
    };

    let coll_url = format!("{STAC_ROOT}/collections/{collection}");
    let coll_bytes = fetch(&coll_url)?;
    let coll_text = std::str::from_utf8(&coll_bytes)
        .map_err(|_| format!("{coll_url}: the collection body is not UTF-8"))?;
    let license = json_string_field(coll_text, "license")
        .ok_or_else(|| format!("{coll_url}: carries no license token — refused"))?;
    if !license_is_open(&license) {
        return Err(format!(
            "{collection}: license '{license}' is not an open license — refused (the register names {COLLECTION_LICENSE})"
        ));
    }

    let items_url = format!("{STAC_ROOT}/collections/{collection}/items?limit=1");
    let items_bytes = fetch(&items_url)?;
    let items = parse_items(&items_bytes).ok_or_else(|| {
        format!("{items_url}: carries no STAC item — the harvest stays unwritten")
    })?;
    let item = items.into_iter().next().ok_or_else(|| {
        format!("{items_url}: the item list is empty — the harvest stays unwritten")
    })?;
    let unix = item.datetime_epoch.ok_or_else(|| {
        format!(
            "{}: carries no datetime — the time coordinate stays unnamed, refused",
            item.id
        )
    })?;
    let lsk: LeapSeconds = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no TDB clock".to_string())?;
    let t = lsk
        .unix_to_tdb(unix)
        .ok_or_else(|| format!("{}: the datetime stays untranslated to TDB", item.id))?;
    let asset = find_asset(&item, &band).ok_or_else(|| {
        format!(
            "{}: carries no '{band}' asset — the harvest stays unwritten",
            item.id
        )
    })?;

    let bytes = fetch(&asset.href)?;
    let file = Hdf5File::parse(&bytes)
        .map_err(|e| format!("{}: the HDF5 arm reads no raster ({e:?})", asset.href))?;
    let values = file
        .read_f64_dataset(&band)
        .map_err(|e| format!("{}: the '{band}' variable stays unread ({e:?})", asset.href))?;
    let fill = file.attr_f64(&band, "_FillValue");
    let missing = file.attr_f64(&band, "missing_value");
    let records = build_records(&values, t, comp, fill, missing, stride, limit);
    if records.is_empty() {
        return Err(format!(
            "{}: no measured {band} sample left the harvest — the bin stays unwritten (0 honored; fill {fill:?}, missing {missing:?})",
            asset.href
        ));
    }

    let bin = write_bin(&records)
        .ok_or_else(|| "a held sample is not finite — the bin stays unwritten".to_string())?;
    match parse_bin(&bin) {
        Some(parsed) if parsed == records => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {}", asset.href);
    println!("compiler tools/harvest/src/bin/inpe_stac_compiler.rs");
    println!("format {FORMAT}");
    println!("collection {collection}");
    println!("license {license}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} {band} cells, {} B, roundtrip parses",
        records.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let records = vec![
        InpeRecord {
            t: 100.0,
            value: 25.5,
            comp: 1,
        },
        InpeRecord {
            t: 100.0,
            value: 9.75,
            comp: 1,
        },
    ];
    let Some(bin) = write_bin(&records) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != HEADER_BYTES + 2 * RECORD_BYTES {
        eprintln!("selftest: the bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if parse_bin(&bin) != Some(records.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    let cells = [11.0, 9.96921e36, 12.0, f64::NAN, 13.0];
    let held = build_records(&cells, 100.0, 1, Some(9.96921e36), None, 1, usize::MAX);
    if held.len() != 3 {
        eprintln!("selftest: the absent samples are not skipped");
        std::process::exit(1);
    }
    if band_code("tmax") != Some(1) || band_code("bogus").is_some() {
        eprintln!("selftest: the band roster is not the measured one");
        std::process::exit(1);
    }
    if !license_is_open(COLLECTION_LICENSE) || license_is_open("proprietary") {
        eprintln!("selftest: the license gate is not the measured one");
        std::process::exit(1);
    }
    eprintln!("inpe_stac_compiler: selftest passes (t/value/comp flat bin, absent skipped)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: inpe_stac_compiler [--collection {COLLECTION_ID}] [--band {DEFAULT_BAND}] [--stride N] [--limit N] [--out <file.bin>] [--ci-mode] | --selftest"
        );
        eprintln!("  reads one openly-licensed INPE BIG STAC collection (CC-BY-4.0 / CC0 / PD);");
        eprintln!("  default {COLLECTION_ID} = {COLLECTION_LICENSE}");
        eprintln!(
            "  the default band tmax is the 2 m maximum temperature grid (SAMeT daily, CPTEC/INPE)"
        );
        eprintln!("  --ci-mode uploads the verified flat bin to the {NETLOC} CDN release");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("inpe_stac_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_bin_roundtrips_the_measured_stride() {
        let records = vec![
            InpeRecord {
                t: 100.0,
                value: 25.5,
                comp: 1,
            },
            InpeRecord {
                t: 100.0,
                value: 9.75,
                comp: 1,
            },
        ];
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(bin.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(records));
    }

    #[test]
    fn parse_bin_refuses_a_truncated_asset() {
        let records = vec![InpeRecord {
            t: 1.0,
            value: 2.0,
            comp: 3,
        }];
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(parse_bin(&bin[..bin.len() - 1]), None);
    }

    #[test]
    fn absent_cells_are_skipped_never_zero() {
        let cells = [11.0, 9.96921e36, 12.0, f64::NAN, 13.0];
        let held = build_records(&cells, 5.0, 1, Some(9.96921e36), None, 1, usize::MAX);
        assert_eq!(held.len(), 3);
        assert_eq!(held[0].value, 11.0);
        assert_eq!(held[2].value, 13.0);
    }

    #[test]
    fn stride_and_limit_bound_the_grid() {
        let cells = [1.0, 2.0, 3.0, 4.0, 5.0];
        let held = build_records(&cells, 0.0, 1, None, None, 2, usize::MAX);
        assert_eq!(held.len(), 3);
        assert_eq!(held[1].value, 3.0);
        let held = build_records(&cells, 0.0, 1, None, None, 1, 2);
        assert_eq!(held.len(), 2);
    }

    #[test]
    fn license_gate_holds_the_measured_tokens() {
        assert!(license_is_open(COLLECTION_LICENSE));
        assert!(license_is_open("CC-BY-4.0"));
        assert!(license_is_open("Creative Commons Zero (CC0)"));
        assert!(license_is_open("Public Domain"));
        assert!(!license_is_open("proprietary"));
    }
}
