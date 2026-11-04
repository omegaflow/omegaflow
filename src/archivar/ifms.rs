use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct IfmsBody {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfmsFile {
    pub header: BTreeMap<String, String>,
    pub config: Vec<(String, String)>,
    pub bodies: Vec<IfmsBody>,
}

fn header_tag(line: &str) -> Option<(String, String)> {
    let start = line.find('<')?;
    let end = line.find('>')?;
    if end <= start {
        return None;
    }
    let tag = &line[start + 1..end];
    if tag.starts_with('/') || tag.starts_with('?') || tag.starts_with('!') {
        return None;
    }
    let rest = &line[end + 1..];
    let close = rest.find("</")?;
    let value = rest[..close].trim().to_string();
    Some((tag.to_string(), value))
}

pub fn parse_ifms(text: &str) -> Option<IfmsFile> {
    let mut header = BTreeMap::new();
    let mut config = Vec::new();
    let mut bodies: Vec<IfmsBody> = Vec::new();
    let mut current: Option<IfmsBody> = None;
    let mut in_header = false;
    let mut in_config = false;
    for raw in text.lines() {
        let line = raw.trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "<header>" {
            in_header = true;
            continue;
        }
        if trimmed == "</header>" {
            in_header = false;
            continue;
        }
        if trimmed == "<active_table>" {
            in_config = true;
            continue;
        }
        if trimmed == "</active_table>" {
            in_config = false;
            continue;
        }
        if in_config {
            if let Some((key, rest)) = trimmed.split_once('=')
                && !key.trim().is_empty()
            {
                let value = match rest.split(';').next() {
                    Some(value) => value.trim().trim_matches('"').to_string(),
                    None => continue,
                };
                config.push((key.trim().to_string(), value));
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("<body_")
            && let Some(name) = rest.strip_suffix('>')
        {
            current = Some(IfmsBody {
                name: name.to_string(),
                columns: Vec::new(),
                rows: Vec::new(),
            });
            continue;
        }
        if trimmed.starts_with("</body_") {
            if let Some(done) = current.take()
                && !done.rows.is_empty()
            {
                bodies.push(done);
            }
            continue;
        }
        if in_header {
            if let Some((tag, value)) = header_tag(trimmed) {
                header.insert(tag, value);
            }
            continue;
        }
        if let Some(body) = current.as_mut() {
            if let Some(cols) = trimmed.strip_prefix("//") {
                if body.columns.is_empty() {
                    body.columns = cols.split_whitespace().map(str::to_string).collect();
                }
                continue;
            }
            let tokens: Vec<String> = trimmed.split_whitespace().map(str::to_string).collect();
            if !tokens.is_empty() {
                body.rows.push(tokens);
            }
        }
    }
    if let Some(done) = current.take()
        && !done.rows.is_empty()
    {
        bodies.push(done);
    }
    if header.is_empty() || bodies.is_empty() {
        return None;
    }
    Some(IfmsFile {
        header,
        config,
        bodies,
    })
}

pub fn rows(file: &IfmsFile) -> usize {
    file.bodies.iter().map(|b| b.rows.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "<header>
<station_id>             TEST01                                </station_id>
<spacecraft_id>          DEMO                                  </spacecraft_id>
<dset_kind>              OP                                    </dset_kind>
<dap_type>               D1                                    </dap_type>
<sample_period>          1.                                    </sample_period>
<total_samples>          2                                     </total_samples>
</header>
<body_Doppler>
// Number  SampleTime                  CarrierPhase
        1  20000101.000000.000           1000.0
        2  20000101.000001.000           1001.5
</body_Doppler>
";

    #[test]
    fn an_ifms_file_carries_header_and_body() {
        let file = parse_ifms(SAMPLE).expect("an ifms file");
        assert_eq!(
            file.header.get("station_id").map(String::as_str),
            Some("NN11")
        );
        assert_eq!(file.header.get("dap_type").map(String::as_str), Some("D1"));
        assert_eq!(file.bodies.len(), 1);
        let body = &file.bodies[0];
        assert_eq!(body.name, "Doppler");
        assert_eq!(body.columns, vec!["Number", "SampleTime", "CarrierPhase"]);
        assert_eq!(body.rows.len(), 2);
        assert_eq!(body.rows[0][1], "[redacted]");
        assert_eq!(rows(&file), 2);
    }

    #[test]
    fn a_file_without_body_is_absent() {
        let text = SAMPLE
            .replace("<body_Doppler>", "")
            .replace("</body_Doppler>", "");
        assert!(parse_ifms(&text).is_none());
    }

    #[test]
    fn a_file_without_header_is_absent() {
        let text = SAMPLE.replace("<header>", "").replace("</header>", "");
        assert!(parse_ifms(&text).is_none());
    }

    #[test]
    fn comment_lines_inside_the_body_do_not_become_rows() {
        let file =
            parse_ifms(&SAMPLE.replace("</body_Doppler>", "// another note\n</body_Doppler>"))
                .expect("an ifms file");
        assert_eq!(file.bodies[0].rows.len(), 2);
    }

    #[test]
    fn the_active_table_travels_as_config() {
        let text = SAMPLE.replace(
            "</header>",
            "</header>\n<active_table>\n   RcdSpecInv = No ; //\n   FreqUplkConv = 7000000000 ; // Hz\n</active_table>",
        );
        let file = parse_ifms(&text).expect("an ifms file");
        assert!(
            file.config
                .iter()
                .any(|(key, value)| key == "FreqUplkConv" && value == "7000000000")
        );
        assert!(file.config.iter().any(|(key, _)| key == "RcdSpecInv"));
        assert_eq!(file.bodies.len(), 1);
    }
}
