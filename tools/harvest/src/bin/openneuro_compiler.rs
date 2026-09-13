use omegaflow::cdn::upload_release;
use omegaflow::json::{jpath_val, jstr, parse_json, JsonVal};
use omegaflow::matfile::{parse_mat, MatArray, MatData, MatField};
use omegaflow::openneuro_eeg::{parse_bin, write_bin, EegEvent, Events, OpenNeuroEeg, Samples};
use std::process::Command;

const DATASET: &str = "ds005034";
const NETLOC: &str = "openneuro.org";
const GRAPHQL: &str = "https://openneuro.org/crn/graphql";

struct EegExtract {
    nbchan: u32,
    pnts: u64,
    trials: u32,
    srate: Option<f64>,
    labels: Vec<String>,
    samples: Samples,
    events: Option<Events>,
}

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

fn snapshot_query() -> String {
    format!(r#"query {{ dataset(id: "{DATASET}") {{ latestSnapshot {{ tag hexsha }} }} }}"#)
}

fn files_query(hexsha: &str) -> String {
    format!(
        r#"query {{ dataset(id: "{DATASET}") {{ latestSnapshot {{ files(tree: "{hexsha}", recursive: true) {{ filename size directory urls }} }} }} }}"#
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
        if !filename.ends_with(".set") {
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

fn latest_snapshot() -> Option<(String, String)> {
    let body = graphql(&snapshot_query())?;
    snapshot_from_json(&body)
}

fn set_files(hexsha: &str) -> Option<Vec<(String, String)>> {
    let body = graphql(&files_query(hexsha))?;
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

enum EegSource<'a> {
    Struct(&'a [MatField]),
    Flat(&'a [MatArray]),
}

impl<'a> EegSource<'a> {
    fn array(&self, name: &str) -> Option<&'a MatArray> {
        match self {
            EegSource::Struct(fields) => fields
                .iter()
                .find(|f| f.name == name)
                .and_then(|f| f.values.first()),
            EegSource::Flat(arrays) => arrays.iter().find(|a| a.name == name),
        }
    }
}

fn field_double(source: &EegSource, name: &str) -> Option<f64> {
    match &source.array(name)?.data {
        MatData::Double(d) => d.first().copied(),
        MatData::Int32(v) => v.first().map(|x| *x as f64),
        _ => None,
    }
}

fn field_usize(source: &EegSource, name: &str) -> Option<usize> {
    let v = field_double(source, name)?;
    if !v.is_finite() || v < 0.0 || v.fract() != 0.0 {
        return None;
    }
    Some(v as usize)
}

fn chanlocs_labels(source: &EegSource) -> Option<Vec<String>> {
    let chanlocs = source.array("chanlocs")?;
    let MatData::Struct(label_fields) = &chanlocs.data else {
        return None;
    };
    let labels = label_fields.iter().find(|f| f.name == "labels")?;
    labels
        .values
        .iter()
        .map(|v| match &v.data {
            MatData::Char(c) => {
                let s = String::from_utf8_lossy(c)
                    .trim_end_matches('\0')
                    .trim()
                    .to_string();
                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            }
            _ => None,
        })
        .collect()
}

fn event_string(v: &MatArray) -> Option<Option<String>> {
    match &v.data {
        MatData::Char(c) => {
            let s = String::from_utf8_lossy(c)
                .trim_end_matches('\0')
                .trim()
                .to_string();
            if s.is_empty() {
                Some(None)
            } else {
                Some(Some(s))
            }
        }
        MatData::Empty => Some(None),
        _ => None,
    }
}

fn event_scalar(v: &MatArray) -> Option<Option<f64>> {
    match &v.data {
        MatData::Double(d) => match d.as_slice() {
            [x] => Some(Some(*x)),
            [] => Some(None),
            _ => None,
        },
        MatData::Int32(x) => match x.as_slice() {
            [x] => Some(Some(*x as f64)),
            [] => Some(None),
            _ => None,
        },
        MatData::Empty => Some(None),
        _ => None,
    }
}

fn normalize_events(fields: &[MatField]) -> Option<Vec<EegEvent>> {
    let n = fields.first()?.values.len();
    let mut type_field: Option<&MatField> = None;
    let mut latency_field: Option<&MatField> = None;
    let mut duration_field: Option<&MatField> = None;
    let mut urevent_field: Option<&MatField> = None;
    for field in fields {
        if field.values.len() != n {
            return None;
        }
        match field.name.as_str() {
            "type" => type_field = Some(field),
            "latency" => latency_field = Some(field),
            "duration" => duration_field = Some(field),
            "urevent" => urevent_field = Some(field),
            _ => return None,
        }
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(EegEvent {
            type_: match type_field {
                Some(f) => event_string(f.values.get(i)?)?,
                None => None,
            },
            latency: match latency_field {
                Some(f) => event_scalar(f.values.get(i)?)?,
                None => None,
            },
            duration: match duration_field {
                Some(f) => event_scalar(f.values.get(i)?)?,
                None => None,
            },
            urevent: match urevent_field {
                Some(f) => event_scalar(f.values.get(i)?)?,
                None => None,
            },
        });
    }
    Some(out)
}

fn extract_events(source: &EegSource, bytes: &[u8]) -> Option<Events> {
    let event = source.array("event")?;
    if let Some((start, end)) = event.span {
        if start < end && end <= bytes.len() {
            return Some(Events::Verbatim(bytes[start..end].to_vec()));
        }
    }
    let MatData::Struct(fields) = &event.data else {
        return None;
    };
    Some(Events::Normalized(normalize_events(fields)?))
}

fn extract_eeg(bytes: &[u8]) -> Option<EegExtract> {
    let arrays = parse_mat(bytes)?;
    let source = match arrays.iter().find(|a| a.name == "EEG") {
        Some(eeg) => match &eeg.data {
            MatData::Struct(fields) => EegSource::Struct(fields),
            _ => return None,
        },
        None => EegSource::Flat(&arrays),
    };
    let nbchan = u32::try_from(field_usize(&source, "nbchan")?).ok()?;
    let pnts = field_usize(&source, "pnts")? as u64;
    let trials = u32::try_from(field_usize(&source, "trials").unwrap_or(1)).ok()?;
    let srate = field_double(&source, "srate").filter(|v| v.is_finite() && *v > 0.0);
    let labels = chanlocs_labels(&source)?;
    if labels.iter().any(|l| l.len() > 0xFFFF) {
        return None;
    }
    let samples = match &source.array("data")?.data {
        MatData::Single(s) => Samples::Single(s.clone()),
        MatData::Double(d) => Samples::Double(d.clone()),
        _ => return None,
    };
    let total = (nbchan as u64)
        .checked_mul(pnts)?
        .checked_mul(trials as u64)?;
    let finite_and_count = match &samples {
        Samples::Single(s) => s.iter().all(|x| x.is_finite()) && s.len() as u64 == total,
        Samples::Double(d) => d.iter().all(|x| x.is_finite()) && d.len() as u64 == total,
    };
    if !finite_and_count {
        return None;
    }
    let events = extract_events(&source, bytes);
    Some(EegExtract {
        nbchan,
        pnts,
        trials,
        srate,
        labels,
        samples,
        events,
    })
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
    extract: &EegExtract,
    set_bytes: &[u8],
    snapshot_tag: &str,
    hexsha: &str,
    origin_url: &str,
) -> Option<Vec<u8>> {
    let sha = sha256_bytes(&omegaflow::archivar::sha256::sha256_hex(set_bytes))?;
    let eeg = OpenNeuroEeg {
        nbchan: extract.nbchan,
        pnts: extract.pnts,
        trials: extract.trials,
        srate: extract.srate,
        sha256: sha,
        snapshot_tag: snapshot_tag.to_string(),
        hexsha: hexsha.to_string(),
        origin_url: origin_url.to_string(),
        labels: extract.labels.clone(),
        events: extract.events.clone(),
        samples: extract.samples.clone(),
    };
    Some(write_bin(&eeg))
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

fn report(extract: &EegExtract) {
    let n = match &extract.samples {
        Samples::Single(s) => s.len(),
        Samples::Double(d) => d.len(),
    };
    eprintln!(
        "nbchan {} pnts {} trials {} srate {} labels {} samples {} events {}",
        extract.nbchan,
        extract.pnts,
        extract.trials,
        rate_label(extract.srate),
        extract.labels.len(),
        n,
        events_label(&extract.events)
    );
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_root = match arg_value(args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/{DATASET}"),
    };

    if let Some(local) = arg_value(args, "--local") {
        let bytes =
            std::fs::read(&local).map_err(|e| format!("read {local} returned void: {e}"))?;
        let extract = extract_eeg(&bytes)
            .ok_or_else(|| format!("{local}: the .set carries no EEG contract"))?;
        eprintln!("{local}:");
        report(&extract);
        return Ok(());
    }

    let subject_filter = arg_value(args, "--subject");
    let session_filter = arg_value(args, "--session");
    let task_filter = arg_value(args, "--task");

    let (snapshot_tag, hexsha) =
        latest_snapshot().ok_or("the OpenNeuro snapshot carries no tag or hexsha")?;
    let files = set_files(&hexsha).ok_or("the OpenNeuro file tree reads void")?;
    if files.is_empty() {
        return Err("the dataset carries no .set file — nothing manifestiert (0 honored)".into());
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
        "{DATASET} snapshot {snapshot_tag} ({hexsha}): {} .set file(s), out root {out_root}",
        filtered.len()
    );
    let mut staged: Vec<String> = Vec::new();
    let mut skipped = 0usize;
    for (rel, url) in &filtered {
        let set_path = format!("{out_root}/{rel}");
        if ci_mode || !std::path::Path::new(&set_path).exists() {
            download(url, &set_path)?;
        }
        let bytes = match std::fs::read(&set_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {set_path} returned void: {e}");
                skipped += 1;
                continue;
            }
        };
        let extract = match extract_eeg(&bytes) {
            Some(ex) => ex,
            None => {
                eprintln!("{rel}: the .set carries no EEG contract — skipped (0 honored)");
                skipped += 1;
                continue;
            }
        };
        eprintln!("{rel}:");
        report(&extract);
        let bin_rel = match rel.strip_suffix(".set") {
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
            "no .set file carries an EEG contract — the harvest stays void (0 honored)".into(),
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
        eprintln!("openneuro_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn align8(p: usize) -> usize {
        (p + 7) & !7
    }

    fn mi_matrix(body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&14u32.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(body);
        out
    }

    fn name_tag(name: &str) -> Vec<u8> {
        let mut b = Vec::new();
        let len = align8(name.len());
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&(len as u32).to_le_bytes());
        b.extend_from_slice(name.as_bytes());
        for _ in name.len()..len {
            b.push(0);
        }
        b
    }

    fn flags_dims_double(name: &str, dims: &[i32], values: &[f64]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        body.extend_from_slice(&name_tag(name));
        body.extend_from_slice(&9u32.to_le_bytes());
        body.extend_from_slice(&((values.len() * 8) as u32).to_le_bytes());
        for &v in values {
            body.extend_from_slice(&v.to_le_bytes());
        }
        mi_matrix(&body)
    }

    fn single_matrix(name: &str, dims: &[i32], values: &[f32]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&7u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        body.extend_from_slice(&name_tag(name));
        body.extend_from_slice(&7u32.to_le_bytes());
        body.extend_from_slice(&((values.len() * 4) as u32).to_le_bytes());
        for &v in values {
            body.extend_from_slice(&v.to_le_bytes());
        }
        pad_body(&mut body);
        mi_matrix(&body)
    }

    fn char_matrix(name: &str, text: &[u8]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&4u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&1i32.to_le_bytes());
        body.extend_from_slice(&(text.len() as i32).to_le_bytes());
        body.extend_from_slice(&name_tag(name));
        body.extend_from_slice(&1u32.to_le_bytes());
        body.extend_from_slice(&(text.len() as u32).to_le_bytes());
        body.extend_from_slice(text);
        pad_body(&mut body);
        mi_matrix(&body)
    }

    fn pad_body(body: &mut Vec<u8>) {
        while body.len() % 8 != 0 {
            body.push(0);
        }
    }

    fn struct_matrix(name: &str, dims: &[i32], fields: &[(&str, Vec<Vec<u8>>)]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&2u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        body.extend_from_slice(&name_tag(name));
        let mut table = Vec::new();
        for (f, _) in fields {
            table.extend_from_slice(f.as_bytes());
            table.push(0);
        }
        let field_len = table.len();
        body.extend_from_slice(&5u16.to_le_bytes());
        body.extend_from_slice(&4u16.to_le_bytes());
        body.extend_from_slice(&(field_len as u32).to_le_bytes());
        let table_len = align8(field_len);
        body.extend_from_slice(&1u32.to_le_bytes());
        body.extend_from_slice(&(table_len as u32).to_le_bytes());
        body.extend_from_slice(&table);
        for _ in field_len..table_len {
            body.push(0);
        }
        let n_elements = dims.iter().product::<i32>() as usize;
        for i in 0..n_elements {
            for (_, values) in fields {
                body.extend_from_slice(&values[i]);
            }
        }
        mi_matrix(&body)
    }

    fn header() -> Vec<u8> {
        let mut bytes = vec![0u8; 128];
        let text = b"MATLAB 5.0 MAT-file";
        bytes[..text.len()].copy_from_slice(text);
        bytes[124] = 0x00;
        bytes[125] = 0x01;
        bytes[126] = b'I';
        bytes[127] = b'M';
        bytes
    }

    fn event_struct_matrix() -> Vec<u8> {
        struct_matrix(
            "event",
            &[1, 2],
            &[
                (
                    "type",
                    vec![
                        char_matrix("type", b"boundary"),
                        char_matrix("type", b"rest"),
                    ],
                ),
                (
                    "latency",
                    vec![
                        flags_dims_double("latency", &[1, 1], &[1.5]),
                        flags_dims_double("latency", &[1, 1], &[2.5]),
                    ],
                ),
                (
                    "duration",
                    vec![
                        flags_dims_double("duration", &[1, 1], &[0.0]),
                        flags_dims_double("duration", &[1, 1], &[0.25]),
                    ],
                ),
            ],
        )
    }

    fn eeg_with_events_fixture() -> Vec<u8> {
        let chanlocs = struct_matrix(
            "chanlocs",
            &[1, 2],
            &[(
                "labels",
                vec![char_matrix("labels", b"Fp1"), char_matrix("labels", b"Fp2")],
            )],
        );
        let eeg = struct_matrix(
            "EEG",
            &[1, 1],
            &[
                ("nbchan", vec![flags_dims_double("nbchan", &[1, 1], &[2.0])]),
                ("pnts", vec![flags_dims_double("pnts", &[1, 1], &[3.0])]),
                ("srate", vec![flags_dims_double("srate", &[1, 1], &[100.0])]),
                (
                    "data",
                    vec![single_matrix(
                        "data",
                        &[2, 3],
                        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                    )],
                ),
                ("chanlocs", vec![chanlocs]),
                ("event", vec![event_struct_matrix()]),
            ],
        );
        let mut bytes = header();
        bytes.extend_from_slice(&eeg);
        bytes
    }

    fn flattened_eeg_fixture() -> Vec<u8> {
        let chanlocs = struct_matrix(
            "chanlocs",
            &[1, 2],
            &[(
                "labels",
                vec![char_matrix("labels", b"E1"), char_matrix("labels", b"E2")],
            )],
        );
        let mut bytes = header();
        bytes.extend_from_slice(&flags_dims_double("nbchan", &[1, 1], &[2.0]));
        bytes.extend_from_slice(&flags_dims_double("pnts", &[1, 1], &[3.0]));
        bytes.extend_from_slice(&flags_dims_double("srate", &[1, 1], &[100.0]));
        bytes.extend_from_slice(&single_matrix(
            "data",
            &[2, 3],
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        ));
        bytes.extend_from_slice(&chanlocs);
        bytes
    }

    #[test]
    fn a_flattened_mat_v5_eeg_extracts() {
        let bytes = flattened_eeg_fixture();
        let extract = extract_eeg(&bytes).expect("the flattened EEG extracts");
        assert_eq!(extract.nbchan, 2);
        assert_eq!(extract.pnts, 3);
        assert_eq!(extract.trials, 1);
        assert_eq!(extract.srate, Some(100.0));
        assert_eq!(extract.labels.len(), 2);
        assert!(extract.events.is_none());
        match extract.samples {
            Samples::Single(s) => assert_eq!(s.len(), 6),
            _ => panic!("not single"),
        }
    }

    #[test]
    fn absent_or_junk_bytes_extract_absent() {
        assert!(extract_eeg(&[]).is_none());
        assert!(extract_eeg(&[0u8; 64]).is_none());
        assert!(extract_eeg(b"not a mat file").is_none());
    }

    #[test]
    fn files_from_json_collects_set_files_and_their_urls() {
        let body = r#"{"data":{"dataset":{"latestSnapshot":{"files":[
            {"filename":"sub-02/ses-verum/eeg/sub-02_ses-verum_task-rest_eeg.set","size":94761160,"directory":false,"urls":["https://s3.amazonaws.com/openneuro.org/ds005034/sub-02/ses-verum/eeg/sub-02_ses-verum_task-rest_eeg.set?versionId=x"]},
            {"filename":"README","size":503,"directory":false,"urls":["https://s3.amazonaws.com/openneuro.org/ds005034/README?versionId=y"]},
            {"filename":"sub-02","size":0,"directory":true,"urls":[]}
        ]}}}}"#;
        let files = files_from_json(body).expect("the file tree parses");
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].0,
            "sub-02/ses-verum/eeg/sub-02_ses-verum_task-rest_eeg.set"
        );
        assert!(files[0]
            .1
            .starts_with("https://s3.amazonaws.com/openneuro.org/"));
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
        let body = graphql_body(r#"query { dataset(id: "ds005034") { id } }"#);
        assert_eq!(
            body,
            "{\"query\":\"query { dataset(id: \\\"ds005034\\\") { id } }\"}"
        );
    }

    fn char_array(text: &str) -> MatArray {
        MatArray {
            name: String::new(),
            dims: vec![1, text.len()],
            data: MatData::Char(text.as_bytes().to_vec()),
            span: None,
        }
    }

    fn double_array(v: f64) -> MatArray {
        MatArray {
            name: String::new(),
            dims: vec![1, 1],
            data: MatData::Double(vec![v]),
            span: None,
        }
    }

    #[test]
    fn normalize_events_carries_standard_fields() {
        let fields = vec![
            MatField {
                name: "type".to_string(),
                values: vec![char_array("boundary"), char_array("rest")],
            },
            MatField {
                name: "latency".to_string(),
                values: vec![double_array(1.5), double_array(2.5)],
            },
            MatField {
                name: "duration".to_string(),
                values: vec![double_array(0.0), double_array(0.25)],
            },
        ];
        let events = normalize_events(&fields).expect("the standard fields normalize");
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].type_.as_deref(), Some("boundary"));
        assert_eq!(events[0].latency, Some(1.5));
        assert_eq!(events[0].duration, Some(0.0));
        assert_eq!(events[0].urevent, None);
        assert_eq!(events[1].type_.as_deref(), Some("rest"));
        assert_eq!(events[1].latency, Some(2.5));
    }

    #[test]
    fn normalize_events_refuses_a_nonstandard_field() {
        let fields = vec![
            MatField {
                name: "type".to_string(),
                values: vec![char_array("boundary")],
            },
            MatField {
                name: "latency".to_string(),
                values: vec![double_array(1.5)],
            },
            MatField {
                name: "reaction_time".to_string(),
                values: vec![double_array(0.3)],
            },
        ];
        assert!(normalize_events(&fields).is_none());
    }

    #[test]
    fn normalize_events_refuses_a_numeric_type() {
        let fields = vec![MatField {
            name: "type".to_string(),
            values: vec![double_array(1.0)],
        }];
        assert!(normalize_events(&fields).is_none());
    }

    #[test]
    fn roundtrip_is_bit_identical() {
        let set = eeg_with_events_fixture();
        let extract = extract_eeg(&set).expect("the fixture extracts");
        let bin = build_bin(
            &extract,
            &set,
            "1.0.1",
            "470458bcff173ca37018a9cb7a55c3804ccc1759",
            "https://s3.amazonaws.com/openneuro.org/ds005034/x.set",
        )
        .expect("the compact asset writes");
        let parsed = parse_bin(&bin).expect("the compact asset parses");

        assert_eq!(parsed.nbchan, extract.nbchan);
        assert_eq!(parsed.pnts, extract.pnts);
        assert_eq!(parsed.trials, extract.trials);
        assert_eq!(parsed.srate, extract.srate);
        assert_eq!(parsed.labels, extract.labels);
        assert_eq!(parsed.snapshot_tag, "1.0.1");
        assert_eq!(parsed.hexsha, "470458bcff173ca37018a9cb7a55c3804ccc1759");
        assert_eq!(
            parsed.origin_url,
            "https://s3.amazonaws.com/openneuro.org/ds005034/x.set"
        );
        assert_eq!(
            parsed.sha256,
            sha256_bytes(&omegaflow::archivar::sha256::sha256_hex(&set)).unwrap()
        );

        match (&extract.samples, &parsed.samples) {
            (Samples::Single(a), Samples::Single(b)) => {
                assert_eq!(a.len(), b.len());
                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x.to_bits(), y.to_bits());
                }
            }
            _ => panic!("sample precision mismatch"),
        }

        match (&extract.events, &parsed.events) {
            (Some(Events::Verbatim(a)), Some(Events::Verbatim(b))) => {
                assert_eq!(a, b);
            }
            _ => panic!("event kind mismatch"),
        }
    }
}
