fn strip_value(raw: &str) -> String {
    raw.trim()
        .trim_end_matches(';')
        .trim()
        .trim_matches(|c| c == '\'' || c == '"')
        .trim()
        .to_string()
}

fn split_kv(line: &str) -> Option<(&str, String)> {
    let eq = line.find('=');
    let colon = line.find(':');
    let at = match (eq, colon) {
        (Some(e), Some(c)) => e.min(c),
        (Some(e), None) => e,
        (None, Some(c)) => c,
        (None, None) => return None,
    };
    let key = line[..at].trim();
    if key.is_empty() || key.contains(char::is_whitespace) {
        return None;
    }
    Some((key, strip_value(&line[at + 1..])))
}

fn channel_row(line: &str) -> Option<(usize, String)> {
    let mut it = line.split_whitespace();
    let idx = it.next()?.parse::<usize>().ok()?;
    if idx == 0 || idx > 4096 {
        return None;
    }
    let label = it.next()?;
    if label.parse::<f64>().is_ok() {
        return None;
    }
    Some((idx, label.to_string()))
}

pub struct EeglabSet {
    pub datfile: String,
    pub nbchan: usize,
    pub pnts: usize,
    pub trials: usize,
    pub srate: Option<f64>,
    pub labels: Vec<String>,
}

pub fn parse_set_header(text: &str) -> Option<EeglabSet> {
    let mut datfile: Option<String> = None;
    let mut nbchan: Option<usize> = None;
    let mut pnts: Option<usize> = None;
    let mut trials: Option<usize> = None;
    let mut srate: Option<f64> = None;
    let mut datatype: Option<String> = None;
    let mut labels: Vec<String> = Vec::new();

    for raw in text.lines() {
        let line = raw.trim().trim_start_matches('\u{feff}');
        if line.is_empty()
            || line.starts_with('#')
            || line.starts_with('%')
            || line.starts_with("//")
        {
            continue;
        }
        if let Some((key, value)) = split_kv(line) {
            match key.to_ascii_lowercase().as_str() {
                "datfile" => {
                    if !value.is_empty() {
                        datfile = Some(value);
                    }
                }
                "nbchan" => nbchan = Some(value.parse::<usize>().ok().filter(|n| *n > 0)?),
                "pnts" => pnts = Some(value.parse::<usize>().ok().filter(|n| *n > 0)?),
                "trials" => trials = Some(value.parse::<usize>().ok().filter(|n| *n > 0)?),
                "srate" => {
                    srate = value
                        .parse::<f64>()
                        .ok()
                        .filter(|v| v.is_finite() && *v > 0.0);
                }
                "datatype" => datatype = Some(value.to_ascii_lowercase()),
                _ => {}
            }
        } else if let Some((idx, label)) = channel_row(line) {
            if labels.len() < idx {
                labels.resize(idx, String::new());
            }
            labels[idx - 1] = label;
        }
    }

    let datfile = datfile?;
    let nbchan = nbchan?;
    let pnts = pnts?;
    let datatype = datatype?;
    if !matches!(datatype.as_str(), "float32" | "single" | "float") {
        return None;
    }
    Some(EeglabSet {
        datfile,
        nbchan,
        pnts,
        trials: trials.unwrap_or(1),
        srate,
        labels,
    })
}

pub fn read_fdt(bytes: &[u8], set: &EeglabSet) -> Option<Vec<f32>> {
    let total = set.nbchan.checked_mul(set.pnts)?.checked_mul(set.trials)?;
    if bytes.len() < total.checked_mul(4)? {
        return None;
    }
    let mut out = Vec::with_capacity(total);
    for i in 0..total {
        let at = i * 4;
        let v = f32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
        if !v.is_finite() {
            return None;
        }
        out.push(v);
    }
    Some(out)
}

pub fn channel_series(samples: &[f32], set: &EeglabSet, channel: usize) -> Option<Vec<f32>> {
    let total = set.nbchan.checked_mul(set.pnts)?.checked_mul(set.trials)?;
    if channel >= set.nbchan || samples.len() < total {
        return None;
    }
    let mut out = Vec::with_capacity(set.pnts * set.trials);
    for trial in 0..set.trials {
        for point in 0..set.pnts {
            out.push(samples[(trial * set.pnts + point) * set.nbchan + channel]);
        }
    }
    Some(out)
}

pub fn common_average_series(samples: &[f32], set: &EeglabSet) -> Option<Vec<f32>> {
    let total = set.nbchan.checked_mul(set.pnts)?.checked_mul(set.trials)?;
    if samples.len() < total {
        return None;
    }
    let points = set.pnts.checked_mul(set.trials)?;
    let mut out = Vec::with_capacity(points);
    for point in 0..points {
        let base = point * set.nbchan;
        let mut sum = 0.0f64;
        for channel in 0..set.nbchan {
            sum += samples[base + channel] as f64;
        }
        out.push((sum / set.nbchan as f64) as f32);
    }
    Some(out)
}

pub fn resolve_channel(set: &EeglabSet, selector: &str) -> Option<usize> {
    if let Ok(n) = selector.parse::<usize>() {
        if n >= 1 && n <= set.nbchan {
            return Some(n - 1);
        }
        return None;
    }
    set.labels.iter().position(|label| label == selector)
}

pub fn open_set(path: &str) -> Option<(EeglabSet, Vec<f32>)> {
    let text = std::fs::read_to_string(path).ok()?;
    let set = parse_set_header(&text)?;
    let dir = std::path::Path::new(path).parent()?;
    let bytes = std::fs::read(dir.join(&set.datfile)).ok()?;
    let samples = read_fdt(&bytes, &set)?;
    Some((set, samples))
}

enum EegSource<'a> {
    Struct(&'a [omegaflow::matfile::MatField]),
    Flat(&'a [omegaflow::matfile::MatArray]),
}

impl<'a> EegSource<'a> {
    fn array(&self, name: &str) -> Option<&'a omegaflow::matfile::MatArray> {
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
        omegaflow::matfile::MatData::Double(d) => d.first().copied(),
        omegaflow::matfile::MatData::Int32(v) => v.first().map(|x| *x as f64),
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
    let omegaflow::matfile::MatData::Struct(label_fields) = &chanlocs.data else {
        return None;
    };
    let labels = label_fields.iter().find(|f| f.name == "labels")?;
    labels
        .values
        .iter()
        .map(|v| match &v.data {
            omegaflow::matfile::MatData::Char(c) => {
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

fn eeg_from_source(source: &EegSource) -> Option<(EeglabSet, Vec<f32>)> {
    let nbchan = field_usize(source, "nbchan")?;
    let pnts = field_usize(source, "pnts")?;
    let trials = field_usize(source, "trials").unwrap_or(1);
    let srate = field_double(source, "srate").filter(|v| v.is_finite() && *v > 0.0);
    let labels = chanlocs_labels(source)?;
    let samples = match &source.array("data")?.data {
        omegaflow::matfile::MatData::Single(s) => s.clone(),
        _ => return None,
    };
    let total = nbchan.checked_mul(pnts)?.checked_mul(trials)?;
    if samples.len() != total || samples.iter().any(|s| !s.is_finite()) {
        return None;
    }
    let set = EeglabSet {
        datfile: String::new(),
        nbchan,
        pnts,
        trials,
        srate,
        labels,
    };
    Some((set, samples))
}

pub fn eeg_from_mat(bytes: &[u8]) -> Option<(EeglabSet, Vec<f32>)> {
    let arrays = omegaflow::matfile::parse_mat(bytes)?;
    let source = match arrays.iter().find(|a| a.name == "EEG") {
        Some(eeg) => match &eeg.data {
            omegaflow::matfile::MatData::Struct(fields) => EegSource::Struct(fields),
            _ => return None,
        },
        None => EegSource::Flat(&arrays),
    };
    eeg_from_source(&source)
}

pub fn open_set_mat(path: &str) -> Option<(EeglabSet, Vec<f32>)> {
    let bytes = std::fs::read(path).ok()?;
    eeg_from_mat(&bytes)
}

pub fn eeg_from_bin(bytes: &[u8]) -> Option<(EeglabSet, Vec<f32>)> {
    let omegaflow::openneuro_eeg::OpenNeuroEeg {
        nbchan,
        pnts,
        trials,
        srate,
        labels,
        samples,
        ..
    } = omegaflow::openneuro_eeg::parse_bin(bytes)?;
    let samples = match samples {
        omegaflow::openneuro_eeg::Samples::Single(s) => s,
        omegaflow::openneuro_eeg::Samples::Double(_) => return None,
    };
    let set = EeglabSet {
        datfile: String::new(),
        nbchan: usize::try_from(nbchan).ok()?,
        pnts: usize::try_from(pnts).ok()?,
        trials: usize::try_from(trials).ok()?,
        srate,
        labels,
    };
    Some((set, samples))
}

pub fn open_set_bin(path: &str) -> Option<(EeglabSet, Vec<f32>)> {
    let bytes = std::fs::read(path).ok()?;
    eeg_from_bin(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_reads_keys_and_the_channel_block() {
        let text = "\u{feff}% EEGLAB dataset\n\
                    datfile = 'sub-02_ses-verum_task-rest_eeg.fdt';\n\
                    nbchan = 2;\n\
                    pnts = 3;\n\
                    trials = 2;\n\
                    srate = 100;\n\
                    datatype = 'float32';\n\
                    channels:\n\
                    1 Fp1\n\
                    2 Fp2\n";
        let set = parse_set_header(text).expect("the text header parses");
        assert_eq!(set.datfile, "sub-02_ses-verum_task-rest_eeg.fdt");
        assert_eq!(set.nbchan, 2);
        assert_eq!(set.pnts, 3);
        assert_eq!(set.trials, 2);
        assert_eq!(set.srate, Some(100.0));
        assert_eq!(set.labels, vec!["Fp1".to_string(), "Fp2".to_string()]);
    }

    #[test]
    fn a_minimal_header_defaults_trials_and_leaves_srate_absent() {
        let text = "datfile = a.fdt\nnbchan = 2\npnts = 3\ndatatype = float32\n";
        let set = parse_set_header(text).expect("the minimal header parses");
        assert_eq!(set.trials, 1);
        assert_eq!(set.srate, None);
        assert!(set.labels.is_empty());
    }

    #[test]
    fn absent_or_unsupported_keys_read_absent() {
        assert!(parse_set_header("").is_none());
        assert!(parse_set_header("nbchan = 2\npnts = 3\ndatatype = float32\n").is_none());
        assert!(
            parse_set_header("datfile = a.fdt\nnbchan = 0\npnts = 3\ndatatype = float32\n")
                .is_none()
        );
        assert!(
            parse_set_header("datfile = a.fdt\nnbchan = 2\npnts = 3\ndatatype = 'int16'\n")
                .is_none()
        );
    }

    #[test]
    fn fdt_unpacks_channel_fastest_little_endian_f32() {
        let text = "datfile = a.fdt\nnbchan = 2\npnts = 3\ndatatype = float32\n";
        let set = parse_set_header(text).expect("the header parses");
        let mut bytes = Vec::new();
        for v in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        let samples = read_fdt(&bytes, &set).expect("the fdt unpacks");
        assert_eq!(channel_series(&samples, &set, 0), Some(vec![1.0, 3.0, 5.0]));
        assert_eq!(channel_series(&samples, &set, 1), Some(vec![2.0, 4.0, 6.0]));
    }

    #[test]
    fn trials_concatenate_and_bad_bytes_read_absent() {
        let text = "datfile = a.fdt\nnbchan = 1\npnts = 2\ntrials = 2\ndatatype = float32\n";
        let set = parse_set_header(text).expect("the header parses");
        let mut bytes = Vec::new();
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        let samples = read_fdt(&bytes, &set).expect("the fdt unpacks");
        assert_eq!(
            channel_series(&samples, &set, 0),
            Some(vec![1.0, 2.0, 3.0, 4.0])
        );
        assert!(read_fdt(&bytes[..8], &set).is_none());
        let mut bad = bytes.clone();
        bad[0..4].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(read_fdt(&bad, &set).is_none());
    }

    #[test]
    fn a_channel_selector_takes_an_index_or_a_label() {
        let text = "datfile = a.fdt\nnbchan = 2\npnts = 3\ndatatype = float32\n1 Fp1\n2 Fp2\n";
        let set = parse_set_header(text).expect("the header parses");
        assert_eq!(resolve_channel(&set, "1"), Some(0));
        assert_eq!(resolve_channel(&set, "Fp2"), Some(1));
        assert_eq!(resolve_channel(&set, "3"), None);
        assert_eq!(resolve_channel(&set, "Cz"), None);
    }

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

    fn flags_dims_double(class: u32, name: &str, dims: &[i32], values: &[f64]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&class.to_le_bytes());
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

    fn eeg_fixture() -> Vec<u8> {
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
                (
                    "nbchan",
                    vec![flags_dims_double(6, "nbchan", &[1, 1], &[2.0])],
                ),
                ("pnts", vec![flags_dims_double(6, "pnts", &[1, 1], &[3.0])]),
                (
                    "srate",
                    vec![flags_dims_double(6, "srate", &[1, 1], &[100.0])],
                ),
                (
                    "data",
                    vec![single_matrix(
                        "data",
                        &[2, 3],
                        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                    )],
                ),
                ("chanlocs", vec![chanlocs]),
            ],
        );
        let mut bytes = vec![0u8; 128];
        let text = b"MATLAB 5.0 MAT-file";
        bytes[..text.len()].copy_from_slice(text);
        bytes[124] = 0x00;
        bytes[125] = 0x01;
        bytes[126] = b'I';
        bytes[127] = b'M';
        bytes.extend_from_slice(&eeg);
        bytes
    }

    #[test]
    fn a_mat_v5_eeg_struct_maps_onto_the_set_and_series() {
        let bytes = eeg_fixture();
        let (set, samples) = eeg_from_mat(&bytes).expect("the EEG struct maps");
        assert_eq!(set.nbchan, 2);
        assert_eq!(set.pnts, 3);
        assert_eq!(set.trials, 1);
        assert_eq!(set.srate, Some(100.0));
        assert_eq!(set.labels, vec!["Fp1".to_string(), "Fp2".to_string()]);
        assert_eq!(channel_series(&samples, &set, 0), Some(vec![1.0, 3.0, 5.0]));
        assert_eq!(channel_series(&samples, &set, 1), Some(vec![2.0, 4.0, 6.0]));
        assert_eq!(resolve_channel(&set, "Fp2"), Some(1));
    }

    #[test]
    fn an_absent_mat_eeg_struct_reads_absent() {
        assert!(eeg_from_mat(&[]).is_none());
        assert!(eeg_from_mat(&[0u8; 64]).is_none());
        let mut bytes = eeg_fixture();
        bytes[125] = 0x02;
        assert!(eeg_from_mat(&bytes).is_none());
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
        let mut bytes = vec![0u8; 128];
        let text = b"MATLAB 5.0 MAT-file";
        bytes[..text.len()].copy_from_slice(text);
        bytes[124] = 0x00;
        bytes[125] = 0x01;
        bytes[126] = b'I';
        bytes[127] = b'M';
        bytes.extend_from_slice(&flags_dims_double(6, "nbchan", &[1, 1], &[2.0]));
        bytes.extend_from_slice(&flags_dims_double(6, "pnts", &[1, 1], &[3.0]));
        bytes.extend_from_slice(&flags_dims_double(6, "srate", &[1, 1], &[100.0]));
        bytes.extend_from_slice(&single_matrix(
            "data",
            &[2, 3],
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        ));
        bytes.extend_from_slice(&chanlocs);
        bytes
    }

    #[test]
    fn a_flattened_mat_v5_eeg_maps_onto_the_set_and_series() {
        let bytes = flattened_eeg_fixture();
        let (set, samples) = eeg_from_mat(&bytes).expect("the flattened EEG maps");
        assert_eq!(set.nbchan, 2);
        assert_eq!(set.pnts, 3);
        assert_eq!(set.trials, 1);
        assert_eq!(set.srate, Some(100.0));
        assert_eq!(set.labels, vec!["E1".to_string(), "E2".to_string()]);
        assert_eq!(channel_series(&samples, &set, 0), Some(vec![1.0, 3.0, 5.0]));
        assert_eq!(channel_series(&samples, &set, 1), Some(vec![2.0, 4.0, 6.0]));
        assert_eq!(resolve_channel(&set, "E2"), Some(1));
    }

    #[test]
    fn a_compact_bin_maps_onto_the_set_and_series() {
        let eeg = omegaflow::openneuro_eeg::OpenNeuroEeg {
            nbchan: 2,
            pnts: 3,
            trials: 1,
            srate: Some(100.0),
            sha256: [0u8; 32],
            snapshot_tag: String::new(),
            hexsha: String::new(),
            origin_url: String::new(),
            labels: vec!["Fp1".to_string(), "Fp2".to_string()],
            events: None,
            samples: omegaflow::openneuro_eeg::Samples::Single(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
        };
        let bytes = omegaflow::openneuro_eeg::write_bin(&eeg);
        let (set, samples) = eeg_from_bin(&bytes).expect("the compact bin maps");
        assert_eq!(set.nbchan, 2);
        assert_eq!(set.pnts, 3);
        assert_eq!(set.trials, 1);
        assert_eq!(set.srate, Some(100.0));
        assert_eq!(set.labels, vec!["Fp1".to_string(), "Fp2".to_string()]);
        assert_eq!(channel_series(&samples, &set, 0), Some(vec![1.0, 3.0, 5.0]));
        assert_eq!(channel_series(&samples, &set, 1), Some(vec![2.0, 4.0, 6.0]));
    }

    #[test]
    fn a_double_precision_bin_reads_absent_in_the_f32_reader() {
        let eeg = omegaflow::openneuro_eeg::OpenNeuroEeg {
            nbchan: 1,
            pnts: 1,
            trials: 1,
            srate: None,
            sha256: [0u8; 32],
            snapshot_tag: String::new(),
            hexsha: String::new(),
            origin_url: String::new(),
            labels: vec!["Cz".to_string()],
            events: None,
            samples: omegaflow::openneuro_eeg::Samples::Double(vec![1.0]),
        };
        let bytes = omegaflow::openneuro_eeg::write_bin(&eeg);
        assert!(eeg_from_bin(&bytes).is_none());
    }
}
