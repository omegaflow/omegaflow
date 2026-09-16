use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::vtscat;
use omegaflow::cdn::upload_release;
use omegaflow::skymap::{
    HEADER_LEN, KIND_GAMMA, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header,
    write_header,
};
use std::io::{BufWriter, Write};

const TREE_URL: &str =
    "https://api.github.com/repos/VERITAS-Observatory/VERITAS-VTSCat/git/trees/main?recursive=1";
const RAW_BASE: &str = "https://raw.githubusercontent.com/VERITAS-Observatory/VERITAS-VTSCat/main/";
const CDN_TAG: &str = "github.com";
const DEFAULT_OUT: &str = "data/github.com/vtscat_flux.sky1";
const TTL: u64 = 604800;

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn is_sed_flux_map(path: &str) -> bool {
    let Some(file) = path.rsplit('/').next() else {
        return false;
    };
    let Some(stem) = file.strip_suffix(".ecsv") else {
        return false;
    };
    stem.contains("-sed")
}

fn sed_paths(tree_json: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut rest = tree_json;
    while let Some(pos) = rest.find("\"path\":") {
        let after = rest[pos + "\"path\":".len()..].trim_start();
        let Some(after) = after.strip_prefix('"') else {
            break;
        };
        let Some(end) = after.find('"') else {
            break;
        };
        let path = &after[..end];
        if is_sed_flux_map(path) {
            out.push(path.to_string());
        }
        rest = &after[end..];
    }
    out.sort();
    out.dedup();
    out
}

fn yaml_sidecar_path(ecsv_path: &str) -> Option<String> {
    let (dir, file) = ecsv_path.rsplit_once('/')?;
    let stem = file.strip_suffix(".ecsv")?;
    let yaml_stem = stem.replacen("-sed", "", 1);
    Some(format!("{dir}/{yaml_stem}.yaml"))
}

fn dnde_scale_to_si(unit: &str) -> Option<f64> {
    let toks: Vec<&str> = unit.split_whitespace().collect();
    let has_tev = toks
        .iter()
        .any(|t| t.starts_with("TeV") || t.starts_with("tev"));
    if !has_tev {
        return None;
    }
    let has_cm2 = toks.iter().any(|t| *t == "cm-2" || *t == "cm^-2");
    let has_m2 = toks.iter().any(|t| *t == "m-2" || *t == "m^-2");
    if has_cm2 {
        Some(1e4)
    } else if has_m2 {
        Some(1.0)
    } else {
        None
    }
}

fn flux_record(ecsv: &str, yaml: &str) -> Option<SkymapRecord> {
    let table = vtscat::parse_ecsv(ecsv)?;
    let e_idx = table.columns.iter().position(|c| c.name == "e_ref")?;
    let d_idx = table.columns.iter().position(|c| c.name == "dnde")?;
    let scale = dnde_scale_to_si(&table.columns[d_idx].unit)?;
    let mut best: Option<(f64, f64)> = None;
    for row in &table.rows {
        let e = *row.get(e_idx)?;
        let d = *row.get(d_idx)?;
        if !e.is_finite() || e <= 0.0 || !d.is_finite() || d <= 0.0 {
            continue;
        }
        let dist = (e - 1.0).abs();
        if best.map_or(true, |(bd, _)| dist < bd) {
            best = Some((dist, d));
        }
    }
    let (_, dnde) = best?;
    let value = dnde * scale;
    if !value.is_finite() || value <= 0.0 {
        return None;
    }
    let ra = vtscat::yaml_sexagesimal(yaml, "pos.ra")?;
    let dec = vtscat::yaml_sexagesimal(yaml, "pos.dec")?;
    if !ra.is_finite() || !(0.0..360.0).contains(&ra) {
        return None;
    }
    if !dec.is_finite() || !(-90.0..=90.0).contains(&dec) {
        return None;
    }
    let (order, ipix) = SkymapRecord::pixel_of(ra, dec)?;
    Some(SkymapRecord {
        order,
        kind: KIND_GAMMA,
        ipix,
        ra_deg: ra as f32,
        dec_deg: dec as f32,
        value: value as f32,
    })
}

fn write_asset(records: &[SkymapRecord], out_path: &str) -> Result<usize, String> {
    if let Some(parent) = std::path::Path::new(out_path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|e| format!("mkdir {out_path}: {e}"))?;
    }
    let file = std::fs::File::create(out_path).map_err(|e| format!("create {out_path}: {e}"))?;
    let mut out = BufWriter::new(file);
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, records.len() as u64);
    out.write_all(&hbuf)
        .map_err(|e| format!("write {out_path}: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for r in records {
        encode_rec(&mut rec, r);
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path}: {e}"))?;
    }
    out.flush().map_err(|e| format!("flush {out_path}: {e}"))?;
    let expect = HEADER_LEN + records.len() * REC_BYTES;
    let actual = std::fs::metadata(out_path)
        .map_err(|e| format!("stat {out_path}: {e}"))?
        .len() as usize;
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }
    Ok(expect)
}

fn verify_asset(out_path: &str, records: &[SkymapRecord]) -> Result<(), String> {
    let bytes = std::fs::read(out_path).map_err(|e| format!("read {out_path}: {e}"))?;
    let n = parse_header(&bytes).ok_or_else(|| format!("{out_path}: header stays unread"))?;
    if n != records.len() as u64 {
        return Err(format!("{out_path}: {n} rows, {} expected", records.len()));
    }
    if records.is_empty() {
        return Ok(());
    }
    let last_off = HEADER_LEN + (records.len() - 1) * REC_BYTES;
    let last = decode_rec(&bytes[last_off..last_off + REC_BYTES])
        .ok_or_else(|| format!("{out_path}: last record stays unread"))?;
    eprintln!(
        "last source: ra {:.4} dec {:.4} value {:.3e} kind {}",
        last.ra_deg, last.dec_deg, last.value, last.kind
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let tree_json = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--tree") {
                Some(u) => u,
                None => TREE_URL.to_string(),
            };
            match fetch_raw_bytes(&url, TTL) {
                Some(b) => match String::from_utf8(b) {
                    Ok(s) => s,
                    Err(_) => {
                        eprintln!("{url}: tree JSON not utf8");
                        std::process::exit(1);
                    }
                },
                None => {
                    eprintln!("{url}: tree fetch void");
                    std::process::exit(1);
                }
            }
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => DEFAULT_OUT.to_string(),
    };
    let paths = sed_paths(&tree_json);
    eprintln!("VTSCat: {} sed.ecsv flux maps", paths.len());
    let mut records: Vec<SkymapRecord> = Vec::new();
    let mut skipped = 0usize;
    for path in &paths {
        let Some(sidecar) = yaml_sidecar_path(path) else {
            skipped += 1;
            continue;
        };
        let ecsv_url = format!("{RAW_BASE}{path}");
        let yaml_url = format!("{RAW_BASE}{sidecar}");
        let Some(ecsv_bytes) = fetch_raw_bytes(&ecsv_url, TTL) else {
            eprintln!("{path}: ecsv fetch void ({ecsv_url})");
            skipped += 1;
            continue;
        };
        let Some(yaml_bytes) = fetch_raw_bytes(&yaml_url, TTL) else {
            eprintln!("{sidecar}: yaml fetch void ({yaml_url})");
            skipped += 1;
            continue;
        };
        let (Ok(ecsv), Ok(yaml)) = (
            std::str::from_utf8(&ecsv_bytes),
            std::str::from_utf8(&yaml_bytes),
        ) else {
            skipped += 1;
            continue;
        };
        match flux_record(ecsv, yaml) {
            Some(r) => records.push(r),
            None => {
                eprintln!("{path}: no measured dnde at 1 TeV — source skipped (0 honored)");
                skipped += 1;
            }
        }
    }
    if records.is_empty() {
        eprintln!(
            "no VTSCat source carries a measured dnde — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    if let Err(e) = write_asset(&records, &out) {
        eprintln!("vtscat_compiler: {e}");
        std::process::exit(1);
    }
    if let Err(e) = verify_asset(&out, &records) {
        eprintln!("vtscat_compiler: {e}");
        std::process::exit(1);
    }
    eprintln!(
        "vtscat: {} sources, {} skipped, {} B -> {}",
        records.len(),
        skipped,
        HEADER_LEN + records.len() * REC_BYTES,
        out
    );
    if ci_mode && !upload_release(CDN_TAG, &out) {
        eprintln!("upload: {} did not reach the CDN", out);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TREE: &str = r#"{
  "sha": "63dbf8242cd8fa6159b9ea33b9f8da39cdd56ac3",
  "tree": [
    {"path": ".github", "type": "tree"},
    {"path": "2008/2008ApJ...679..397A/VER-000058-lc.ecsv", "type": "blob"},
    {"path": "2008/2008ApJ...679..397A/VER-000058-sed.ecsv", "type": "blob"},
    {"path": "2008/2008ApJ...679..397A/VER-000058.yaml", "type": "blob"},
    {"path": "2009/2009ApJ...706L.275A/VER-000018-sed-1.ecsv", "type": "blob"},
    {"path": "2009/2009ApJ...706L.275A/VER-000018-1.yaml", "type": "blob"}
  ]
}"#;

    const SED: &str = r#"# %ECSV 0.9
# ---
# datatype:
# - {name: e_ref, unit: TeV, datatype: float32}
# - {name: dnde, unit: m-2 s-1 TeV-1, datatype: float32}
# - {name: dnde_err, unit: m-2 s-1 TeV-1, datatype: float32}
# - {name: significance, datatype: float32}
e_ref dnde dnde_err  significance
0.25    1.36e-7 0.60e-7 2.26
0.50    4.55e-8 1.15e-8 3.97
1.00    7.39e-9 2.65e-9 2.80
2.00    1.92e-9 0.68e-9 2.82
"#;

    const YAML: &str = r#"---
source_id: 58
reference_id: 2008ApJ...679..397A
telescope: veritas
pos:
  ra: {val: 12h30m46s, err: 0h0m4s, err_sys: 0h0m6s}
  dec: {val: 12d23m21s, err: 50s, err_sys: 0d1m30s}
spec:
  erange: {min: 0.2, max: 10., unit: TeV}
"#;

    #[test]
    fn sed_paths_picks_only_flux_maps() {
        let paths = sed_paths(TREE);
        assert_eq!(paths.len(), 2);
        assert!(paths[0].ends_with("VER-000058-sed.ecsv"));
        assert!(paths[1].ends_with("VER-000018-sed-1.ecsv"));
    }

    #[test]
    fn sidecar_maps_sed_to_yaml() {
        assert_eq!(
            yaml_sidecar_path("2008/2008ApJ...679..397A/VER-000058-sed.ecsv").as_deref(),
            Some("2008/2008ApJ...679..397A/VER-000058.yaml")
        );
        assert_eq!(
            yaml_sidecar_path("2009/2009ApJ...706L.275A/VER-000018-sed-1.ecsv").as_deref(),
            Some("2009/2009ApJ...706L.275A/VER-000018-1.yaml")
        );
        assert!(yaml_sidecar_path("nonsense").is_none());
    }

    #[test]
    fn unit_scale_normalizes_length_to_meters() {
        assert_eq!(dnde_scale_to_si("m-2 s-1 TeV-1"), Some(1.0));
        assert_eq!(dnde_scale_to_si("TeV-1 cm-2 s-1"), Some(1e4));
        assert_eq!(dnde_scale_to_si("cm-2 s-1 TeV-1"), Some(1e4));
        assert_eq!(dnde_scale_to_si("m-2 s-1 GeV-1"), None);
        assert_eq!(dnde_scale_to_si("erg"), None);
    }

    #[test]
    fn flux_record_reads_position_and_fiducial_dnde() {
        let r = flux_record(SED, YAML).unwrap();
        assert_eq!(r.kind, KIND_GAMMA);
        assert!((r.ra_deg as f64 - 187.6916666667).abs() < 1e-3);
        assert!((r.dec_deg as f64 - 12.3891666667).abs() < 1e-3);
        assert!((r.value as f64 - 7.39e-9).abs() < 1e-12);
    }

    #[test]
    fn flux_record_skips_absent_position() {
        assert!(flux_record(SED, "source_id: 58\n").is_none());
        assert!(flux_record("hello\nworld\n", YAML).is_none());
    }

    #[test]
    fn skymap_asset_roundtrips() {
        let records = vec![flux_record(SED, YAML).unwrap()];
        let path = std::env::temp_dir().join("vtscat_flux_test.sky1");
        let p = path.to_str().unwrap();
        write_asset(&records, p).unwrap();
        verify_asset(p, &records).unwrap();
        let expect = HEADER_LEN + records.len() * REC_BYTES;
        assert_eq!(std::fs::metadata(p).unwrap().len() as usize, expect);
        let _ = std::fs::remove_file(p);
        assert!(records.iter().all(|r| r.kind == KIND_GAMMA));
    }
}
