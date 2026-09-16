#[derive(Clone, Debug, PartialEq)]
pub struct EcsvColumn {
    pub name: String,
    pub unit: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EcsvTable {
    pub columns: Vec<EcsvColumn>,
    pub rows: Vec<Vec<f64>>,
}

fn split_header(text: &str) -> Option<(&str, &str)> {
    let mut offset = 0usize;
    for line in text.split_inclusive('\n') {
        let t = line.trim_start();
        if !t.is_empty() && !t.starts_with('#') {
            return Some((&text[..offset], &text[offset..]));
        }
        offset += line.len();
    }
    None
}

fn parse_datatypes(head: &str) -> Vec<EcsvColumn> {
    let mut cols = Vec::new();
    let mut in_datatype = false;
    for line in head.lines() {
        let t = line.trim().trim_start_matches('#').trim();
        if t == "datatype:" {
            in_datatype = true;
            continue;
        }
        if !in_datatype {
            continue;
        }
        if !t.starts_with("- {") {
            continue;
        }
        let Some(name) = t
            .split("name:")
            .nth(1)
            .and_then(|s| s.split(',').next())
            .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
        else {
            continue;
        };
        let unit = match t.split("unit:").nth(1).and_then(|s| s.split(',').next()) {
            Some(s) => s.trim().trim_matches('"').trim_matches('\'').to_string(),
            None => String::new(),
        };
        if !name.is_empty() {
            cols.push(EcsvColumn { name, unit });
        }
    }
    cols
}

pub fn parse_ecsv(text: &str) -> Option<EcsvTable> {
    let (head, body) = split_header(text)?;
    let mut columns = parse_datatypes(head);
    let mut lines = body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    let header_line = lines.next()?;
    if columns.is_empty() {
        columns = header_line
            .split_whitespace()
            .map(|n| EcsvColumn {
                name: n.to_string(),
                unit: String::new(),
            })
            .collect();
    }
    if columns.is_empty() {
        return None;
    }
    let mut rows = Vec::new();
    for line in lines {
        let toks: Vec<&str> = line.split_whitespace().collect();
        let mut row = Vec::with_capacity(columns.len());
        for tok in toks.iter().take(columns.len()) {
            let Ok(v) = tok.trim_matches('"').parse::<f64>() else {
                break;
            };
            if !v.is_finite() {
                break;
            }
            row.push(v);
        }
        if row.len() == columns.len() {
            rows.push(row);
        }
    }
    if rows.is_empty() {
        return None;
    }
    Some(EcsvTable { columns, rows })
}

pub fn parse_sexagesimal(s: &str) -> Option<f64> {
    let t = s.trim();
    let (neg, t) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t),
    };
    let (sign, t) = if neg {
        (-1.0, t)
    } else {
        (1.0, t.strip_prefix('+').unwrap_or(t))
    };
    let first_unit = t.find(['h', 'd'])?;
    let deg_part: f64 = t[..first_unit].trim().parse().ok()?;
    let unit = t.as_bytes()[first_unit];
    let deg = if unit == b'h' {
        deg_part * 15.0
    } else {
        deg_part
    };
    let rest = &t[first_unit + 1..];
    let mut parts = rest.split('m');
    let min_field = parts.next().unwrap_or("").trim();
    let min = if min_field.is_empty() {
        0.0
    } else {
        min_field.parse::<f64>().ok()?
    };
    let sec_field = parts
        .next()
        .map(|s| s.trim_end_matches('s').trim())
        .unwrap_or("");
    let sec = if sec_field.is_empty() {
        0.0
    } else {
        sec_field.parse::<f64>().ok()?
    };
    let total = deg + min / 60.0 + sec / 3600.0;
    let v = sign * total;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

pub fn yaml_val(text: &str, path: &str) -> Option<String> {
    let keys: Vec<&str> = path.split('.').collect();
    let mut cur_indent = 0usize;
    for (ki, key) in keys.iter().enumerate() {
        let last = ki == keys.len() - 1;
        let target = format!("{key}:");
        let mut found: Option<(usize, String)> = None;
        for line in text.lines() {
            let indent = line.len() - line.trim_start().len();
            if indent != cur_indent || !line.trim_start().starts_with(&target) {
                continue;
            }
            let rest = line.trim_start()[target.len()..].trim();
            if last {
                return Some(rest.to_string());
            }
            found = Some((indent + 2, rest.to_string()));
            break;
        }
        match found {
            Some((next_indent, rest)) => {
                if rest == "{}" || rest.is_empty() {
                    if last {
                        return Some(rest.to_string());
                    }
                    return None;
                }
                cur_indent = next_indent;
            }
            None => return None,
        }
    }
    None
}

pub fn yaml_sexagesimal(text: &str, path: &str) -> Option<f64> {
    let raw = yaml_val(text, path)?;
    let val = if raw.contains("val:") {
        raw.split("val:")
            .nth(1)
            .and_then(|s| s.split([',', '}']).next())
            .map(str::trim)
            .unwrap_or("")
    } else {
        &raw
    };
    parse_sexagesimal(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SED: &str = r#"# %ECSV 0.9
# ---
# datatype:
# - {name: e_ref, unit: TeV, datatype: float32}
# - {name: dnde, unit: m-2 s-1 TeV-1, datatype: float32}
# - {name: dnde_err, unit: m-2 s-1 TeV-1, datatype: float32}
# - {name: significance, datatype: float32}
# meta: !!omap
# - data_type: sed
# - source_id: 58
# - reference_id: 2008ApJ...679..397A
# - telescope: veritas
# - comments: |
#     Table 1
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
    fn ecsv_parses_datatypes_and_rows() {
        let table = parse_ecsv(SED).unwrap();
        assert_eq!(table.columns.len(), 4);
        assert_eq!(table.columns[0].name, "e_ref");
        assert_eq!(table.columns[0].unit, "TeV");
        assert_eq!(table.columns[1].name, "dnde");
        assert_eq!(table.columns[2].name, "dnde_err");
        assert_eq!(table.columns[3].name, "significance");
        assert_eq!(table.rows.len(), 4);
        assert!((table.rows[0][0] - 0.25).abs() < 1e-9);
        assert!((table.rows[0][1] - 1.36e-7).abs() < 1e-15);
    }

    #[test]
    fn ecsv_rejects_non_ecsv_text() {
        assert!(parse_ecsv("hello\nworld\n").is_none());
        assert!(parse_ecsv("").is_none());
    }

    #[test]
    fn sexagesimal_decodes_ra_and_dec() {
        let ra = parse_sexagesimal("12h30m46s").unwrap();
        assert!((ra - 187.6916666667).abs() < 1e-6, "ra {ra}");
        let dec = parse_sexagesimal("12d23m21s").unwrap();
        assert!((dec - 12.3891666667).abs() < 1e-6, "dec {dec}");
        let neg = parse_sexagesimal("-12d23m21s").unwrap();
        assert!((neg + 12.3891666667).abs() < 1e-6);
        assert!(parse_sexagesimal("nonsense").is_none());
    }

    #[test]
    fn yaml_val_walks_dotted_paths() {
        let ra = yaml_sexagesimal(YAML, "pos.ra").unwrap();
        assert!((ra - 187.6916666667).abs() < 1e-6);
        let dec = yaml_sexagesimal(YAML, "pos.dec").unwrap();
        assert!((dec - 12.3891666667).abs() < 1e-6);
        assert_eq!(
            yaml_val(YAML, "source_id").as_deref(),
            Some("58")
        );
    }
}
