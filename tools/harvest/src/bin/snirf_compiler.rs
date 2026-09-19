use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, jpath_val, jstr, parse_json};
use omegaflow::snirf::{
    Geometry, Samples, SnirfBin, SnirfExtract, TimeBase, parse_bin, parse_snirf, write_bin,
};
use std::process::Command;

const NETLOC: &str = "openneuro.org";
const GRAPHQL: &str = "https://openneuro.org/crn/graphql";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn graphql_body(query: &str) -> String {
    let mut body = String::from("{\"query\":\"");
    for c in query.chars() {
        match c {
            '"' => body.push_str("\\\""),
            '\\' => body.push_str("\\\\"),
            '\n' => body.push(' '),
            c => body.push(c),
        }
    }
    body.push_str("\"}");
    body
}

fn graphql(query: &str) -> Option<String> {
    let body = graphql_body(query);
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--max-time")
        .arg("120")
        .arg("-A")
        .arg("omegaflow-gate/1.0")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("--data-raw")
        .arg(&body)
        .arg(GRAPHQL)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        eprintln!("graphql: {}", String::from_utf8_lossy(&out.stderr).trim());
        None
    }
}

fn snapshot_query(dataset: &str) -> String {
    format!(r#"query {{ dataset(id: "{dataset}") {{ latestSnapshot {{ tag hexsha }} }} }}"#)
}

fn files_query(hexsha: &str, dataset: &str) -> String {
    format!(
        r#"query {{ dataset(id: "{dataset}") {{ latestSnapshot {{ files(tree: "{hexsha}", recursive: true) {{ filename size directory urls }} }} }} }}"#
    )
}

fn snapshot_from_json(body: &str) -> Option<(String, String)> {
    let root = parse_json(body)?;
    let tag = jstr(&root, "data.dataset.latestSnapshot.tag")?;
    let hexsha = jstr(&root, "data.dataset.latestSnapshot.hexsha")?;
    Some((tag, hexsha))
}

fn files_from_json(body: &str) -> Option<Vec<(String, String)>> {
    let root = parse_json(body)?;
    let files = jpath_val(&root, "data.dataset.latestSnapshot.files")?;
    let JsonVal::Arr(items) = files else {
        return Some(Vec::new());
    };
    let mut out = Vec::new();
    for it in items {
        let JsonVal::Obj(map) = it else {
            continue;
        };
        let filename = match map.get("filename") {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => continue,
        };
        if !filename.ends_with(".snirf") {
            continue;
        }
        let url = match map.get("urls") {
            Some(JsonVal::Arr(u)) => u.iter().find_map(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            }),
            _ => None,
        };
        if let Some(url) = url {
            out.push((filename, url));
        }
    }
    out.sort();
    Some(out)
}

fn latest_snapshot(dataset: &str) -> Option<(String, String)> {
    let body = graphql(&snapshot_query(dataset))?;
    snapshot_from_json(&body)
}

fn snirf_files(hexsha: &str, dataset: &str) -> Option<Vec<(String, String)>> {
    let body = graphql(&files_query(hexsha, dataset))?;
    files_from_json(&body)
}

fn download(url: &str, path: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
    }
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("1800")
        .arg("-o")
        .arg(path)
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: {} — the file stays unfetched",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let sz = std::fs::metadata(path)
        .map_err(|e| format!("metadata {path} returned void: {e}"))?
        .len();
    if sz == 0 {
        return Err(format!("{url}: the file carries no bytes"));
    }
    Ok(())
}

fn sha256_bytes(hex: &str) -> Option<[u8; 32]> {
    if hex.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

fn build_bin(
    extract: &SnirfExtract,
    snirf_bytes: &[u8],
    snapshot_tag: &str,
    hexsha: &str,
    origin_url: &str,
) -> Option<Vec<u8>> {
    let sha = sha256_bytes(&omegaflow::archivar::sha256::sha256_hex(snirf_bytes))?;
    let bin = SnirfBin {
        nchan: extract.nchan,
        pnts: extract.pnts,
        nwavelengths: extract.nwavelengths,
        nsources: extract.nsources,
        ndetectors: extract.ndetectors,
        sha256: sha,
        snapshot_tag: snapshot_tag.to_string(),
        hexsha: hexsha.to_string(),
        origin_url: origin_url.to_string(),
        labels: extract.labels.clone(),
        wavelengths: extract.wavelengths.clone(),
        source_geometry: extract.source_geometry.clone(),
        detector_geometry: extract.detector_geometry.clone(),
        meas_list: Some(extract.meas_list.clone()),
        time: extract.time.clone(),
        samples: extract.samples.clone(),
    };
    write_bin(&bin)
}

fn geom_label(g: &Option<Geometry>) -> &'static str {
    match g {
        None => "absent",
        Some(Geometry::Pos2(_)) => "2d",
        Some(Geometry::Pos3(_)) => "3d",
    }
}

fn time_label(t: &Option<TimeBase>) -> &'static str {
    match t {
        None => "absent",
        Some(TimeBase::Series(_)) => "series",
        Some(TimeBase::Spacing { .. }) => "spacing",
    }
}

fn report(extract: &SnirfExtract) {
    let n = match &extract.samples {
        Samples::Single(s) => s.len(),
        Samples::Double(d) => d.len(),
    };
    eprintln!(
        "nchan {} pnts {} wavelengths {} sources {} detectors {} src_geom {} det_geom {} meas {} time {} samples {}",
        extract.nchan,
        extract.pnts,
        extract.nwavelengths,
        extract.nsources,
        extract.ndetectors,
        geom_label(&extract.source_geometry),
        geom_label(&extract.detector_geometry),
        extract.meas_list.len(),
        time_label(&extract.time),
        n,
    );
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let dataset = match arg_value(args, "--dataset") {
        Some(d) => d,
        None => "ds008192".to_string(),
    };
    let out_root = match arg_value(args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/{dataset}"),
    };

    if let Some(probe) = arg_value(args, "--probe") {
        let bytes =
            std::fs::read(&probe).map_err(|e| format!("read {probe} returned void: {e}"))?;
        let extract = parse_snirf(&bytes).map_err(|m| format!("{probe}: {m}"))?;
        eprintln!("{probe}:");
        report(&extract);
        return Ok(());
    }

    let subject_filter = arg_value(args, "--subject");
    let session_filter = arg_value(args, "--session");
    let task_filter = arg_value(args, "--task");

    let (snapshot_tag, hexsha) =
        latest_snapshot(&dataset).ok_or("the OpenNeuro snapshot carries no tag or hexsha")?;
    let files = snirf_files(&hexsha, &dataset).ok_or("the OpenNeuro file tree reads void")?;
    if files.is_empty() {
        return Err("the dataset carries no .snirf file — nothing manifestiert (0 honored)".into());
    }
    let filtered: Vec<&(String, String)> = files
        .iter()
        .filter(|(name, _)| {
            let subject_ok = match &subject_filter {
                Some(s) => name.contains(s.as_str()),
                None => true,
            };
            let session_ok = match &session_filter {
                Some(s) => name.contains(&format!("/ses-{s}/")),
                None => true,
            };
            let task_ok = match &task_filter {
                Some(t) => name.contains(&format!("_task-{t}_")),
                None => true,
            };
            subject_ok && session_ok && task_ok
        })
        .collect();

    eprintln!(
        "{dataset} snapshot {snapshot_tag} ({hexsha}): {} .snirf file(s), out root {out_root}",
        filtered.len()
    );
    let mut staged: Vec<String> = Vec::new();
    let mut skipped = 0usize;
    for (rel, url) in &filtered {
        let snirf_path = format!("{out_root}/{rel}");
        if ci_mode || !std::path::Path::new(&snirf_path).exists() {
            download(url, &snirf_path)?;
        }
        let bytes = match std::fs::read(&snirf_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {snirf_path} returned void: {e}");
                skipped += 1;
                continue;
            }
        };
        let extract = match parse_snirf(&bytes) {
            Ok(ex) => ex,
            Err(msg) => {
                eprintln!("{rel}: {msg} — skipped (0 honored)");
                skipped += 1;
                continue;
            }
        };
        eprintln!("{rel}:");
        report(&extract);
        let bin_rel = match rel.strip_suffix(".snirf") {
            Some(s) => format!("{s}.bin"),
            None => format!("{rel}.bin"),
        };
        let bin_path = format!("{out_root}/{bin_rel}");
        let Some(bin) = build_bin(&extract, &bytes, &snapshot_tag, &hexsha, url) else {
            eprintln!("{rel}: the compact asset stays unwritten (0 honored)");
            skipped += 1;
            continue;
        };
        std::fs::write(&bin_path, &bin)
            .map_err(|e| format!("write {bin_path} returned void: {e}"))?;
        let parsed = parse_bin(&bin).ok_or_else(|| {
            format!("{bin_path}: roundtrip parse void — the asset stays unverified")
        })?;
        let n_meas = parsed.meas_list.map_or(0, |m| m.len());
        eprintln!(
            "{bin_path}: {} bytes, roundtrip parses ({n_meas} measurement entries)",
            bin.len()
        );
        staged.push(bin_path);
    }
    if staged.is_empty() {
        return Err(
            "no .snirf file carries a SNIRF contract — the harvest stays void (0 honored)".into(),
        );
    }
    if !ci_mode {
        eprintln!(
            "{} asset(s) staged, {} skipped, no upload without --ci-mode",
            staged.len(),
            skipped
        );
        return Ok(());
    }
    for path in &staged {
        eprintln!("upload {path} -> cdn tag {NETLOC}");
        if !upload_release(NETLOC, path) {
            return Err(format!("upload {path} returned void"));
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("snirf_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::snirf::{Geometry, SnirfChannel};

    fn extract() -> SnirfExtract {
        SnirfExtract {
            nchan: 2,
            pnts: 3,
            nwavelengths: 2,
            nsources: 2,
            ndetectors: 2,
            labels: vec!["S1-D1@1".to_string(), "S2-D2@1".to_string()],
            wavelengths: vec![760.0, 850.0],
            source_geometry: Some(Geometry::Pos2(vec![(0.0, 0.0), (3.0, 0.0)])),
            detector_geometry: Some(Geometry::Pos2(vec![(1.0, 0.0), (4.0, 0.0)])),
            meas_list: vec![
                SnirfChannel {
                    source_index: 1,
                    detector_index: 1,
                    wavelength_index: 1,
                    data_type_index: Some(1),
                    data_type_label: Some("HbO".to_string()),
                },
                SnirfChannel {
                    source_index: 2,
                    detector_index: 2,
                    wavelength_index: 1,
                    data_type_index: Some(2),
                    data_type_label: Some("HbR".to_string()),
                },
            ],
            time: Some(TimeBase::Series(vec![0.0, 0.1, 0.2])),
            samples: Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
        }
    }

    #[test]
    fn files_from_json_collects_snirf_files_and_their_urls() {
        let body = r#"{"data":{"dataset":{"latestSnapshot":{"files":[
            {"filename":"sub-01/fnirs/sub-01_task-social_acq-hyperscanning_run-01_fnirs.snirf","size":53420184,"directory":false,"urls":["https://s3.amazonaws.com/openneuro.org/ds008192/sub-01/fnirs/sub-01_task-social_acq-hyperscanning_run-01_fnirs.snirf?versionId=x"]},
            {"filename":"README","size":503,"directory":false,"urls":["https://s3.amazonaws.com/openneuro.org/ds008192/README?versionId=y"]},
            {"filename":"sub-01","size":0,"directory":true,"urls":[]}
        ]}}}}"#;
        let files = files_from_json(body).expect("the file tree parses");
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].0,
            "sub-01/fnirs/sub-01_task-social_acq-hyperscanning_run-01_fnirs.snirf"
        );
        assert!(
            files[0]
                .1
                .starts_with("https://s3.amazonaws.com/openneuro.org/")
        );
    }

    #[test]
    fn snapshot_from_json_reads_the_latest_snapshot() {
        let body = r#"{"data":{"dataset":{"latestSnapshot":{"tag":"1.0.1","hexsha":"470458bcff173ca37018a9cb7a55c3804ccc1759"}}}}"#;
        assert_eq!(
            snapshot_from_json(body),
            Some((
                "1.0.1".to_string(),
                "470458bcff173ca37018a9cb7a55c3804ccc1759".to_string()
            ))
        );
    }

    #[test]
    fn graphql_body_escapes_quotes_into_a_json_payload() {
        let body = graphql_body(r#"query { dataset(id: "ds008192") { id } }"#);
        assert_eq!(
            body,
            "{\"query\":\"query { dataset(id: \\\"ds008192\\\") { id } }\"}"
        );
    }

    #[test]
    fn roundtrip_is_bit_identical() {
        let source: Vec<u8> = vec![0x89, b'H', b'D', b'F', 0x0d, 0x0a, 0x1a, 0x0a, 0, 0, 0];
        let ex = extract();
        let bin = build_bin(
            &ex,
            &source,
            "1.0.1",
            "470458bcff173ca37018a9cb7a55c3804ccc1759",
            "https://s3.amazonaws.com/openneuro.org/ds008192/x.snirf",
        )
        .expect("the compact asset writes");
        let parsed = parse_bin(&bin).expect("the compact asset parses");
        assert_eq!(parsed.nchan, ex.nchan);
        assert_eq!(parsed.pnts, ex.pnts);
        assert_eq!(parsed.nwavelengths, ex.nwavelengths);
        assert_eq!(parsed.nsources, ex.nsources);
        assert_eq!(parsed.ndetectors, ex.ndetectors);
        assert_eq!(parsed.labels, ex.labels);
        assert_eq!(parsed.wavelengths, ex.wavelengths);
        assert_eq!(parsed.source_geometry, ex.source_geometry);
        assert_eq!(parsed.detector_geometry, ex.detector_geometry);
        assert_eq!(parsed.meas_list, Some(ex.meas_list.clone()));
        assert_eq!(parsed.time, ex.time);
        assert_eq!(parsed.snapshot_tag, "1.0.1");
        assert_eq!(parsed.hexsha, "470458bcff173ca37018a9cb7a55c3804ccc1759");
        assert_eq!(
            parsed.origin_url,
            "https://s3.amazonaws.com/openneuro.org/ds008192/x.snirf"
        );
        assert_eq!(
            parsed.sha256,
            sha256_bytes(&omegaflow::archivar::sha256::sha256_hex(&source)).unwrap()
        );
        match (&ex.samples, &parsed.samples) {
            (Samples::Single(a), Samples::Single(b)) => {
                assert_eq!(a.len(), b.len());
                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x.to_bits(), y.to_bits());
                }
            }
            _ => panic!("sample precision mismatch"),
        }
    }

    #[test]
    fn junk_bytes_extract_absent() {
        assert!(parse_snirf(&[]).is_err());
        assert!(parse_snirf(&[0u8; 64]).is_err());
        assert!(parse_snirf(b"not an hdf5 file").is_err());
    }
}
