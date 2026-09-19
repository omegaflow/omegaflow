use omegaflow::archivar::sha256::sha256_raw;
use omegaflow::brainvision::{BrainVisionEeg, extract, parse_vhdr};
use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, jpath_val, jstr, parse_json};
use omegaflow::openneuro_eeg::{Events, OpenNeuroEeg, Samples, parse_bin, write_bin};
use std::collections::HashMap;
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

fn all_files_from_json(body: &str) -> Option<HashMap<String, String>> {
    let root = parse_json(body)?;
    let files = jpath_val(&root, "data.dataset.latestSnapshot.files")?;
    let JsonVal::Arr(items) = files else {
        return Some(HashMap::new());
    };
    let mut out = HashMap::new();
    for it in items {
        let JsonVal::Obj(map) = it else {
            continue;
        };
        let filename = match map.get("filename") {
            Some(JsonVal::Str(s)) => s.clone(),
            _ => continue,
        };
        let url = match map.get("urls") {
            Some(JsonVal::Arr(u)) => u.iter().find_map(|v| match v {
                JsonVal::Str(s) => Some(s.clone()),
                _ => None,
            }),
            _ => None,
        };
        if let Some(url) = url {
            out.insert(filename, url);
        }
    }
    Some(out)
}

fn vhdr_rels(files: &HashMap<String, String>) -> Vec<String> {
    let mut v: Vec<String> = files
        .keys()
        .filter(|f| f.ends_with(".vhdr"))
        .cloned()
        .collect();
    v.sort();
    v
}

fn dir_of(rel: &str) -> &str {
    match rel.rfind('/') {
        Some(i) => &rel[..=i],
        None => "",
    }
}

fn latest_snapshot(dataset: &str) -> Option<(String, String)> {
    let body = graphql(&snapshot_query(dataset))?;
    snapshot_from_json(&body)
}

fn all_files(hexsha: &str, dataset: &str) -> Option<HashMap<String, String>> {
    let body = graphql(&files_query(hexsha, dataset))?;
    all_files_from_json(&body)
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

fn build_bin(
    ex: &BrainVisionEeg,
    vhdr: &[u8],
    vmrk: Option<&[u8]>,
    eeg: &[u8],
    snapshot_tag: &str,
    hexsha: &str,
    origin_url: &str,
) -> Option<Vec<u8>> {
    let mut source = Vec::with_capacity(vhdr.len() + vmrk.map_or(0, |v| v.len()) + eeg.len());
    source.extend_from_slice(vhdr);
    if let Some(v) = vmrk {
        source.extend_from_slice(v);
    }
    source.extend_from_slice(eeg);
    let rec = OpenNeuroEeg {
        nbchan: ex.nbchan,
        pnts: ex.pnts,
        trials: 1,
        srate: ex.srate,
        sha256: sha256_raw(&source),
        snapshot_tag: snapshot_tag.to_string(),
        hexsha: hexsha.to_string(),
        origin_url: origin_url.to_string(),
        labels: ex.labels.clone(),
        events: ex.events.clone(),
        samples: ex.samples.clone(),
    };
    Some(write_bin(&rec))
}

fn rate_label(srate: Option<f64>) -> String {
    match srate {
        Some(v) => format!("{v}"),
        None => "absent".to_string(),
    }
}

fn events_label(events: &Option<Events>) -> &'static str {
    match events {
        Some(Events::Verbatim(_)) => "verbatim",
        Some(Events::Normalized(_)) => "normalized",
        None => "absent",
    }
}

fn report(ex: &BrainVisionEeg) {
    let n = match &ex.samples {
        Samples::Single(s) => s.len(),
        Samples::Double(d) => d.len(),
    };
    eprintln!(
        "nbchan {} pnts {} srate {} labels {} samples {} events {}",
        ex.nbchan,
        ex.pnts,
        rate_label(ex.srate),
        ex.labels.len(),
        n,
        events_label(&ex.events)
    );
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let dataset = match arg_value(args, "--dataset") {
        Some(d) => d,
        None => "ds007471".to_string(),
    };
    let out_root = match arg_value(args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/{dataset}"),
    };

    if let Some(local) = arg_value(args, "--local") {
        let vhdr_bytes =
            std::fs::read(&local).map_err(|e| format!("read {local} returned void: {e}"))?;
        let header = parse_vhdr(&vhdr_bytes)
            .ok_or_else(|| format!("{local}: the .vhdr carries no BrainVision contract"))?;
        let parent = std::path::Path::new(&local)
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| format!("{local}: the path carries no parent directory"))?;
        let data_file = match header.data_file.as_deref() {
            Some(f) => f,
            None => {
                return Err(format!(
                    "{local}: the .vhdr names no DataFile — the samples stay unread"
                ));
            }
        };
        let eeg_path = parent.join(data_file);
        let eeg_bytes = std::fs::read(&eeg_path)
            .map_err(|e| format!("read {} returned void: {e}", eeg_path.display()))?;
        let vmrk_bytes = header
            .marker_file
            .as_deref()
            .and_then(|f| std::fs::read(parent.join(f)).ok());
        let ex = extract(&vhdr_bytes, vmrk_bytes.as_deref(), &eeg_bytes)
            .ok_or_else(|| format!("{local}: the triple carries no BrainVision contract"))?;
        eprintln!("{local}:");
        report(&ex);
        return Ok(());
    }

    let subject_filter = arg_value(args, "--subject");
    let session_filter = arg_value(args, "--session");
    let task_filter = arg_value(args, "--task");

    let (snapshot_tag, hexsha) =
        latest_snapshot(&dataset).ok_or("the OpenNeuro snapshot carries no tag or hexsha")?;
    let files = all_files(&hexsha, &dataset).ok_or("the OpenNeuro file tree reads void")?;
    let vhdrs = vhdr_rels(&files);
    if vhdrs.is_empty() {
        return Err("the dataset carries no .vhdr file — nothing manifestiert (0 honored)".into());
    }
    let filtered: Vec<&String> = vhdrs
        .iter()
        .filter(|name| {
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
        "{dataset} snapshot {snapshot_tag} ({hexsha}): {} .vhdr file(s), out root {out_root}",
        filtered.len()
    );
    let mut staged: Vec<String> = Vec::new();
    let mut skipped = 0usize;
    for rel in filtered {
        let vhdr_url = &files[rel];
        let vhdr_path = format!("{out_root}/{rel}");
        if ci_mode || !std::path::Path::new(&vhdr_path).exists() {
            download(vhdr_url, &vhdr_path)?;
        }
        let vhdr_bytes = match std::fs::read(&vhdr_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {vhdr_path} returned void: {e}");
                skipped += 1;
                continue;
            }
        };
        let header = match parse_vhdr(&vhdr_bytes) {
            Some(h) => h,
            None => {
                eprintln!("{rel}: the .vhdr carries no BrainVision contract — skipped (0 honored)");
                skipped += 1;
                continue;
            }
        };
        let dir = dir_of(rel);
        let data_file = match header.data_file.as_deref() {
            Some(f) => f,
            None => {
                eprintln!("{rel}: the .vhdr names no DataFile — skipped (0 honored)");
                skipped += 1;
                continue;
            }
        };
        let eeg_rel = format!("{dir}{data_file}");
        let eeg_url = match files.get(&eeg_rel) {
            Some(u) => u,
            None => {
                eprintln!("{rel}: the .eeg stays unfetched — skipped (0 honored)");
                skipped += 1;
                continue;
            }
        };
        let eeg_path = format!("{out_root}/{eeg_rel}");
        if ci_mode || !std::path::Path::new(&eeg_path).exists() {
            download(eeg_url, &eeg_path)?;
        }
        let eeg_bytes = match std::fs::read(&eeg_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {eeg_path} returned void: {e}");
                skipped += 1;
                continue;
            }
        };
        let vmrk_bytes = match header.marker_file.as_deref() {
            Some(mf) => {
                let m_rel = format!("{dir}{mf}");
                match files.get(&m_rel) {
                    Some(url) => {
                        let m_path = format!("{out_root}/{m_rel}");
                        if ci_mode || !std::path::Path::new(&m_path).exists() {
                            download(url, &m_path)?;
                        }
                        match std::fs::read(&m_path) {
                            Ok(b) => Some(b),
                            Err(e) => {
                                eprintln!("read {m_path} returned void: {e}");
                                None
                            }
                        }
                    }
                    None => None,
                }
            }
            None => None,
        };
        let ex = match extract(&vhdr_bytes, vmrk_bytes.as_deref(), &eeg_bytes) {
            Some(x) => x,
            None => {
                eprintln!(
                    "{rel}: the triple carries no BrainVision contract — skipped (0 honored)"
                );
                skipped += 1;
                continue;
            }
        };
        eprintln!("{rel}:");
        report(&ex);
        let bin_rel = match rel.strip_suffix(".vhdr") {
            Some(s) => format!("{s}.bin"),
            None => format!("{rel}.bin"),
        };
        let bin_path = format!("{out_root}/{bin_rel}");
        let Some(bin) = build_bin(
            &ex,
            &vhdr_bytes,
            vmrk_bytes.as_deref(),
            &eeg_bytes,
            &snapshot_tag,
            &hexsha,
            vhdr_url,
        ) else {
            eprintln!("{rel}: the compact asset stays unwritten (0 honored)");
            skipped += 1;
            continue;
        };
        std::fs::write(&bin_path, &bin)
            .map_err(|e| format!("write {bin_path} returned void: {e}"))?;
        let parsed = parse_bin(&bin).ok_or_else(|| {
            format!("{bin_path}: roundtrip parse void — the asset stays unverified")
        })?;
        let n_events = match &parsed.events {
            Some(Events::Normalized(evs)) => evs.len(),
            Some(Events::Verbatim(_)) | None => 0,
        };
        eprintln!(
            "{bin_path}: {} bytes, roundtrip parses ({n_events} normalized events)",
            bin.len()
        );
        staged.push(bin_path);
    }
    if staged.is_empty() {
        return Err(
            "no .vhdr triple carries a BrainVision contract — the harvest stays void (0 honored)"
                .into(),
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
        eprintln!("brainvision_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_vhdr() -> Vec<u8> {
        "Brain Vision Data Exchange Header File Version 1.0\r\n\
         [Common Infos]\r\n\
         DataFile=sub-01_task-x_eeg.eeg\r\n\
         MarkerFile=sub-01_task-x_eeg.vmrk\r\n\
         DataFormat=BINARY\r\n\
         DataOrientation=MULTIPLEXED\r\n\
         NumberOfChannels=2\r\n\
         SamplingInterval=2000\r\n\
         [Binary Infos]\r\n\
         BinaryFormat=INT_16\r\n\
         [Channel Infos]\r\n\
         Ch1=Fp1,,0.1,µV\r\n\
         Ch2=Fp2,,0.1,µV\r\n"
            .as_bytes()
            .to_vec()
    }

    fn fixture_vmrk() -> Vec<u8> {
        "Brain Vision Data Exchange Marker File, Version 1.0\r\n\
         [Marker Infos]\r\n\
         Mk1=New Segment,,1,1,0,20200101000000000000\r\n\
         Mk2=Stimulus,S  1,101,1,0,20200101000100000000\r\n"
            .as_bytes()
            .to_vec()
    }

    fn fixture_eeg() -> Vec<u8> {
        let mut b = Vec::new();
        for v in [100i16, 400, 200, 500, 300, 600] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b
    }

    #[test]
    fn files_from_json_collects_every_file_with_a_url() {
        let body = r#"{"data":{"dataset":{"latestSnapshot":{"files":[
            {"filename":"sub-01/eeg/sub-01_task-rest_eeg.vhdr","size":5120,"directory":false,"urls":["https://s3.amazonaws.com/openneuro.org/ds007471/sub-01/eeg/sub-01_task-rest_eeg.vhdr?versionId=x"]},
            {"filename":"sub-01/eeg/sub-01_task-rest_eeg.vmrk","size":2304,"directory":false,"urls":["https://s3.amazonaws.com/openneuro.org/ds007471/sub-01/eeg/sub-01_task-rest_eeg.vmrk?versionId=y"]},
            {"filename":"sub-01/eeg/sub-01_task-rest_eeg.eeg","size":94761160,"directory":false,"urls":["https://s3.amazonaws.com/openneuro.org/ds007471/sub-01/eeg/sub-01_task-rest_eeg.eeg?versionId=z"]},
            {"filename":"sub-01","size":0,"directory":true,"urls":[]}
        ]}}}}"#;
        let files = all_files_from_json(body).expect("the file tree parses");
        assert_eq!(files.len(), 3);
        assert!(files.contains_key("sub-01/eeg/sub-01_task-rest_eeg.eeg"));
        assert_eq!(
            vhdr_rels(&files),
            vec!["sub-01/eeg/sub-01_task-rest_eeg.vhdr".to_string()]
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
    fn roundtrip_is_bit_identical() {
        let vhdr = fixture_vhdr();
        let vmrk = fixture_vmrk();
        let eeg = fixture_eeg();
        let ex = extract(&vhdr, Some(&vmrk), &eeg).expect("the fixture triple extracts");
        let bin = build_bin(
            &ex,
            &vhdr,
            Some(&vmrk),
            &eeg,
            "1.0.1",
            "470458bcff173ca37018a9cb7a55c3804ccc1759",
            "https://s3.amazonaws.com/openneuro.org/ds007471/sub-01/eeg/sub-01_task-x_eeg.vhdr",
        )
        .expect("the compact asset writes");
        let parsed = parse_bin(&bin).expect("the compact asset parses");

        assert_eq!(parsed.nbchan, ex.nbchan);
        assert_eq!(parsed.pnts, ex.pnts);
        assert_eq!(parsed.trials, 1);
        assert_eq!(parsed.srate, ex.srate);
        assert_eq!(parsed.labels, ex.labels);
        assert_eq!(parsed.snapshot_tag, "1.0.1");
        assert_eq!(parsed.hexsha, "470458bcff173ca37018a9cb7a55c3804ccc1759");
        assert_eq!(
            parsed.origin_url,
            "https://s3.amazonaws.com/openneuro.org/ds007471/sub-01/eeg/sub-01_task-x_eeg.vhdr"
        );
        let mut source = Vec::new();
        source.extend_from_slice(&vhdr);
        source.extend_from_slice(&vmrk);
        source.extend_from_slice(&eeg);
        assert_eq!(parsed.sha256, sha256_raw(&source));

        match (&ex.samples, &parsed.samples) {
            (Samples::Single(a), Samples::Single(b)) => {
                assert_eq!(a.len(), b.len());
                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x.to_bits(), y.to_bits());
                }
            }
            _ => panic!("sample precision mismatch"),
        }

        match (&ex.events, &parsed.events) {
            (Some(Events::Normalized(a)), Some(Events::Normalized(b))) => {
                assert_eq!(a.len(), b.len());
                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x.type_, y.type_);
                    assert_eq!(x.latency, y.latency);
                    assert_eq!(x.duration, y.duration);
                    assert_eq!(x.urevent, y.urevent);
                }
            }
            _ => panic!("event kind mismatch"),
        }
    }
}
