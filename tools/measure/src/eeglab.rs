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
                    srate = value.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0);
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
            parse_set_header("datfile = a.fdt\nnbchan = 0\npnts = 3\ndatatype = float32\n").is_none()
        );
        assert!(
            parse_set_header("datfile = a.fdt\nnbchan = 2\npnts = 3\ndatatype = 'int16'\n").is_none()
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
        assert_eq!(channel_series(&samples, &set, 0), Some(vec![1.0, 2.0, 3.0, 4.0]));
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
}
