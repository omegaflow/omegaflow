use omegaflow::cdn::upload_asset;
use omegaflow::json::{jnum, jpath_val, jstr, parse_json, JsonVal};
use std::collections::BTreeMap;
use std::process::Command;

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from(".")
}

fn mast_token() -> String {
    if let Ok(t) = std::env::var("MAST_TOKEN") {
        if !t.is_empty() {
            return t;
        }
    }
    std::fs::read_to_string(state_dir().join(".secrets.local"))
        .ok()
        .and_then(|body| {
            body.lines().find_map(|line| {
                let (k, v) = line.split_once('=')?;
                (k.trim() == "MAST_TOKEN" && !v.trim().is_empty()).then(|| v.trim().to_string())
            })
        })
        .unwrap_or_default()
}

fn curl_json(url: &str, data: &[(&str, String)], token: &str) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-L")
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg("300")
        .arg("-G");
    for (k, v) in data {
        cmd.arg("--data-urlencode").arg(format!("{}={}", k, v));
    }
    if !token.is_empty() {
        cmd.arg("-H")
            .arg(format!("Authorization: Bearer {}", token));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "curl {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

struct SpectrumRow {
    pl_name: String,
    host: String,
    bibcode: String,
    wl_min: Option<f64>,
    wl_max: Option<f64>,
}

fn tap_curated_spectrum_rows(token: &str) -> Vec<SpectrumRow> {
    let adql = "SELECT p.hostname, s.pl_name, s.bibcode, s.minwavelng, s.maxwavelng FROM spectra s, ps p WHERE s.pl_name = p.pl_name AND p.default_flag = 1 AND p.ra IS NOT NULL AND p.dec IS NOT NULL AND s.facility LIKE '%James Webb Space Telescope%' AND s.spec_type = 'Transmission' AND (s.instrument LIKE '%NIRSpec%' OR s.instrument LIKE '%NIRISS%' OR s.instrument LIKE '%MIRI%')";
    let body = match curl_json(
        "https://exoplanetarchive.ipac.caltech.edu/TAP/sync",
        &[
            ("REQUEST", "doQuery".to_string()),
            ("LANG", "ADQL".to_string()),
            ("FORMAT", "json".to_string()),
            ("QUERY", adql.to_string()),
        ],
        token,
    ) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let Some(root) = parse_json(&body) else {
        eprintln!("tap spectra: json absent");
        return Vec::new();
    };
    let rows: &[JsonVal] = match jpath_val(&root, "data") {
        Some(JsonVal::Arr(a)) => a,
        _ => match &root {
            JsonVal::Arr(a) => a,
            _ => {
                eprintln!("tap spectra: data array absent");
                return Vec::new();
            }
        },
    };
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let (Some(host), Some(pl_name)) = (jstr(row, "hostname"), jstr(row, "pl_name")) else {
            continue;
        };
        let bibcode = jstr(row, "bibcode").unwrap_or_default();
        let wl_min = jnum(row, "minwavelng").filter(|v| v.is_finite());
        let wl_max = jnum(row, "maxwavelng").filter(|v| v.is_finite());
        out.push(SpectrumRow {
            pl_name,
            host,
            bibcode,
            wl_min,
            wl_max,
        });
    }
    out
}

struct Detection {
    host: String,
    pl_name: String,
    species: String,
    bibcode: String,
    abundance: Option<f64>,
    snr: Option<f64>,
}

fn parse_seed(path: &str) -> Option<(Vec<Detection>, usize)> {
    let body = match std::fs::read_to_string(path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("seed {} absent — detections stay pending (0 honored)", path);
            return None;
        }
        Err(e) => {
            eprintln!("seed {} reads void — {}", path, e);
            return None;
        }
    };
    let Some(root) = parse_json(&body) else {
        eprintln!("seed {}: json absent — detections stay pending", path);
        return None;
    };
    let JsonVal::Arr(rows) = &root else {
        eprintln!(
            "seed {}: root is not an array — detections stay pending",
            path
        );
        return None;
    };
    let mut out = Vec::new();
    let mut skipped = 0usize;
    for row in rows {
        let (Some(host), Some(species)) = (jstr(row, "host"), jstr(row, "species")) else {
            skipped += 1;
            continue;
        };
        if host.is_empty() || species.is_empty() {
            skipped += 1;
            continue;
        }
        let bibcode = jstr(row, "bibcode").unwrap_or_default();
        let pl_name = jstr(row, "pl_name").unwrap_or_default();
        let abundance = jnum(row, "abundance").filter(|v| v.is_finite());
        let snr = jnum(row, "snr").filter(|v| v.is_finite());
        out.push(Detection {
            host,
            pl_name,
            species,
            bibcode,
            abundance,
            snr,
        });
    }
    Some((out, skipped))
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn spectrum_obj(r: &SpectrumRow) -> String {
    let mut fields = vec![format!("\"pl_name\":\"{}\"", json_escape(&r.pl_name))];
    if !r.bibcode.is_empty() {
        fields.push(format!("\"bibcode\":\"{}\"", json_escape(&r.bibcode)));
    }
    if let Some(a) = r.wl_min {
        fields.push(format!("\"wl_min\":{}", a));
    }
    if let Some(b) = r.wl_max {
        fields.push(format!("\"wl_max\":{}", b));
    }
    format!("{{{}}}", fields.join(","))
}

fn detection_obj(d: &Detection) -> String {
    let mut fields = vec![format!("\"species\":\"{}\"", json_escape(&d.species))];
    if !d.pl_name.is_empty() {
        fields.push(format!("\"pl_name\":\"{}\"", json_escape(&d.pl_name)));
    }
    if !d.bibcode.is_empty() {
        fields.push(format!("\"bibcode\":\"{}\"", json_escape(&d.bibcode)));
    }
    if let Some(a) = d.abundance {
        fields.push(format!("\"abundance\":{}", a));
    }
    if let Some(s) = d.snr {
        fields.push(format!("\"snr\":{}", s));
    }
    format!("{{{}}}", fields.join(","))
}

fn detection_key(d: &Detection) -> String {
    format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}",
        d.host,
        d.pl_name,
        d.species,
        d.bibcode,
        d.abundance.map(|v| v.to_string()).unwrap_or_default(),
        d.snr.map(|v| v.to_string()).unwrap_or_default()
    )
}

fn group_detections(dets: Vec<Detection>) -> BTreeMap<String, Vec<Detection>> {
    let mut seen = std::collections::HashSet::new();
    let mut by_host: BTreeMap<String, Vec<Detection>> = BTreeMap::new();
    for d in dets {
        if seen.insert(detection_key(&d)) {
            by_host.entry(d.host.clone()).or_default().push(d);
        }
    }
    for list in by_host.values_mut() {
        list.sort_by(|a, b| {
            a.species
                .cmp(&b.species)
                .then_with(|| a.bibcode.cmp(&b.bibcode))
        });
    }
    by_host
}

fn build_registry(
    rows: &[SpectrumRow],
    detections: &BTreeMap<String, Vec<Detection>>,
    seed_state: &str,
    generated_unix: Option<u64>,
) -> String {
    let mut by_host: BTreeMap<String, Vec<&SpectrumRow>> = BTreeMap::new();
    for r in rows {
        by_host.entry(r.host.clone()).or_default().push(r);
    }
    let mut targets = String::new();
    let mut first_target = true;
    for (host, host_rows) in &by_host {
        if !first_target {
            targets.push(',');
        }
        first_target = false;
        let mut sorted = host_rows.clone();
        sorted.sort_by(|a, b| {
            a.pl_name
                .cmp(&b.pl_name)
                .then_with(|| a.bibcode.cmp(&b.bibcode))
        });
        targets.push_str("\n    {\n      \"host\": \"");
        targets.push_str(&json_escape(host));
        targets.push_str("\",\n      \"spectra\": [");
        for (i, r) in sorted.iter().enumerate() {
            if i > 0 {
                targets.push(',');
            }
            targets.push_str("\n        ");
            targets.push_str(&spectrum_obj(r));
        }
        targets.push_str("\n      ],\n      \"detections\": [");
        let dets = detections.get(host);
        match dets {
            Some(list) => {
                for (i, d) in list.iter().enumerate() {
                    if i > 0 {
                        targets.push(',');
                    }
                    targets.push_str("\n        ");
                    targets.push_str(&detection_obj(d));
                }
            }
            None => {}
        }
        targets.push_str("\n      ]\n    }");
    }
    let mut out = String::new();
    out.push_str("{\n  \"asset\": \"jwst_detection_registry\",");
    out.push_str(&format!("\n  \"seed_state\": \"{}\",", seed_state));
    match generated_unix {
        Some(t) => out.push_str(&format!("\n  \"generated_unix\": {},", t)),
        None => out.push_str("\n  \"generated_unix\": null,"),
    }
    out.push_str("\n  \"targets\": [");
    out.push_str(&targets);
    out.push_str("\n  ]\n}\n");
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out = "jwst_detection_registry.json".to_string();
    let mut seed_path: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out = args.get(i + 1).cloned().unwrap_or(out);
                i += 1;
            }
            "--seed" => {
                seed_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {
                eprintln!(
                    "usage: jwst_detection_registry_compiler [--out <jwst_detection_registry.json>] [--seed <seed.json>] [--ci-mode]"
                );
                return;
            }
        }
        i += 1;
    }
    let token = mast_token();
    let rows = tap_curated_spectrum_rows(&token);
    eprintln!("curated jwst transmission spectrum rows: {}", rows.len());
    let mut hosts: Vec<String> = rows.iter().map(|r| r.host.clone()).collect();
    hosts.sort();
    hosts.dedup();
    eprintln!("distinct curated hosts: {}", hosts.len());
    let (detections, seed_state) = match &seed_path {
        Some(path) => match parse_seed(path) {
            Some((dets, skipped)) => {
                eprintln!(
                    "seed {}: {} detections read, {} rows skipped (host/species absent)",
                    path,
                    dets.len(),
                    skipped
                );
                let by_host = group_detections(dets);
                let seed_count: usize = by_host.values().map(|v| v.len()).sum();
                eprintln!("seed dedup leaves {} detections", seed_count);
                let curated: std::collections::HashSet<String> = hosts.iter().cloned().collect();
                let outside: Vec<&String> =
                    by_host.keys().filter(|h| !curated.contains(*h)).collect();
                if !outside.is_empty() {
                    eprintln!(
                        "seed hosts without a curated JWST spectrum: {} ({})",
                        outside.len(),
                        outside
                            .iter()
                            .map(|h| h.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                (by_host, "loaded")
            }
            None => (BTreeMap::new(), "absent"),
        },
        None => {
            eprintln!("--seed absent — detections stay pending (0 honored)");
            (BTreeMap::new(), "absent")
        }
    };
    let generated_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs());
    let text = build_registry(&rows, &detections, seed_state, generated_unix);
    let tmp = format!("{}.tmp", out);
    if std::fs::write(&tmp, text.as_bytes()).is_err() {
        eprintln!("write {}: void — the registry stays unwritten", out);
        return;
    }
    if std::fs::rename(&tmp, &out).is_err() {
        eprintln!("rename to {}: void — the registry stays unwritten", out);
        let _ = std::fs::remove_file(&tmp);
        return;
    }
    let Some(root) = parse_json(&text) else {
        eprintln!(
            "{}: roundtrip parse void — the registry stays unverified",
            out
        );
        return;
    };
    match jpath_val(&root, "targets") {
        Some(JsonVal::Arr(arr)) => {
            eprintln!(
                "{}: {} targets, {} B — roundtrip parses",
                out,
                arr.len(),
                text.len()
            );
        }
        _ => {
            eprintln!(
                "{}: roundtrip parse void — the registry stays unverified",
                out
            );
            return;
        }
    }
    if ci_mode && !upload_asset(&out) {
        eprintln!("upload: {} did not reach the CDN", out);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_rows() -> Vec<SpectrumRow> {
        vec![
            SpectrumRow {
                pl_name: "WASP-39 b".to_string(),
                host: "WASP-39".to_string(),
                bibcode: "2018Natur.555..227W".to_string(),
                wl_min: Some(0.6),
                wl_max: Some(2.8),
            },
            SpectrumRow {
                pl_name: "WASP-39 b".to_string(),
                host: "WASP-39".to_string(),
                bibcode: "".to_string(),
                wl_min: Some(5.0),
                wl_max: Some(12.0),
            },
            SpectrumRow {
                pl_name: "HAT-P-12 b".to_string(),
                host: "HAT-P-12".to_string(),
                bibcode: "2025A&A...703A.264C".to_string(),
                wl_min: Some(2.85),
                wl_max: Some(5.17),
            },
        ]
    }

    fn detections_flat() -> Vec<Detection> {
        vec![
            Detection {
                host: "WASP-39".to_string(),
                species: "H2O".to_string(),
                bibcode: "2023Natur.614..649F".to_string(),
                abundance: Some(3.2e-5),
                snr: Some(8.4),
            },
            Detection {
                host: "WASP-39".to_string(),
                species: "H2O".to_string(),
                bibcode: "2023Natur.614..649F".to_string(),
                abundance: Some(3.2e-5),
                snr: Some(8.4),
            },
            Detection {
                host: "WASP-39".to_string(),
                species: "CO2".to_string(),
                bibcode: "2023Natur.614..649F".to_string(),
                abundance: None,
                snr: None,
            },
            Detection {
                host: "WASP-96".to_string(),
                species: "H2O".to_string(),
                bibcode: "2022Natur..946A".to_string(),
                abundance: None,
                snr: Some(11.0),
            },
        ]
    }

    #[test]
    fn registry_emits_targets_spectra_and_only_matching_detections() {
        let by_host = group_detections(detections_flat());
        assert_eq!(by_host.get("WASP-39").unwrap().len(), 2);
        assert!(by_host.contains_key("WASP-96"));
        let text = build_registry(&synthetic_rows(), &by_host, "loaded", Some(1750000000));
        let root = parse_json(&text).unwrap();
        let JsonVal::Arr(targets) = jpath_val(&root, "targets").unwrap() else {
            panic!("targets absent");
        };
        assert_eq!(targets.len(), 2);
        let JsonVal::Obj(t0) = &targets[0] else {
            panic!("target object absent");
        };
        let JsonVal::Str(host0) = t0.get("host").unwrap() else {
            panic!("host absent");
        };
        assert_eq!(host0, "HAT-P-12");
        let JsonVal::Arr(spectra) = t0.get("spectra").unwrap() else {
            panic!("spectra absent");
        };
        assert_eq!(spectra.len(), 1);
        assert_eq!(jnum(&targets[0], "spectra.0.wl_min"), Some(2.85));
        let JsonVal::Arr(det) = t0.get("detections").unwrap() else {
            panic!("detections absent");
        };
        assert!(det.is_empty());
        let JsonVal::Obj(t1) = &targets[1] else {
            panic!("target object absent");
        };
        let JsonVal::Arr(spectra1) = t1.get("spectra").unwrap() else {
            panic!("spectra absent");
        };
        assert_eq!(spectra1.len(), 2);
        assert!(jpath_val(&spectra1[0], "bibcode").is_none());
        assert_eq!(jstr(&spectra1[0], "pl_name").unwrap(), "WASP-39 b");
        assert_eq!(
            jstr(&spectra1[1], "bibcode").unwrap(),
            "2018Natur.555..227W"
        );
        let JsonVal::Arr(det1) = t1.get("detections").unwrap() else {
            panic!("detections absent");
        };
        assert_eq!(det1.len(), 2);
        assert_eq!(jstr(&det1[0], "species").unwrap(), "CO2");
        assert!(jpath_val(&det1[0], "snr").is_none());
        assert_eq!(jstr(&det1[1], "species").unwrap(), "H2O");
        let text_has_no_wasp96_target = targets
            .iter()
            .all(|t| jstr(t, "host").as_deref() != Some("WASP-96"));
        assert!(text_has_no_wasp96_target);
    }

    #[test]
    fn registry_without_seed_carries_no_detections() {
        let empty = BTreeMap::new();
        let text = build_registry(&synthetic_rows(), &empty, "absent", None);
        let root = parse_json(&text).unwrap();
        assert_eq!(jstr(&root, "seed_state").unwrap(), "absent");
        let JsonVal::Arr(targets) = jpath_val(&root, "targets").unwrap() else {
            panic!("targets absent");
        };
        for t in targets {
            let JsonVal::Arr(det) = jpath_val(t, "detections").unwrap() else {
                panic!("detections absent");
            };
            assert!(det.is_empty());
        }
        assert!(matches!(
            jpath_val(&root, "generated_unix"),
            Some(JsonVal::Null)
        ));
    }

    #[test]
    fn parse_seed_reads_curation_and_marks_malformed() {
        let dir = std::env::temp_dir();
        let good = dir.join("jwst_detection_seed_good.json");
        let bad = dir.join("jwst_detection_seed_bad.json");
        std::fs::write(
            &good,
            r#"[{"host":"WASP-39","species":"H2O","abundance":3.2e-5,"snr":8.4,"bibcode":"2023Natur.614..649F"},{"host":"WASP-39","species":"CO2"},{"host":"","species":"H2O"}]"#,
        )
        .unwrap();
        std::fs::write(&bad, r#"not json"#).unwrap();
        let (dets, skipped) = parse_seed(good.to_str().unwrap()).unwrap();
        assert_eq!(dets.len(), 2);
        assert_eq!(skipped, 1);
        assert!(dets[1].abundance.is_none());
        assert!(parse_seed(bad.to_str().unwrap()).is_none());
        let _ = std::fs::remove_file(&good);
        let _ = std::fs::remove_file(&bad);
    }
}
