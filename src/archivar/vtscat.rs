#[derive(Clone, Debug, PartialEq)]
pub struct EcsvColumn {
    pub name: String,
    pub unit: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EcsvTable {
    pub columns: Vec<EcsvColumn>,
    pub rows: Vec<Vec<Option<f64>>>,
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
            row.push(if v.is_finite() { Some(v) } else { None });
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
    let total = deg_part + min / 60.0 + sec / 3600.0;
    let total = if unit == b'h' { total * 15.0 } else { total };
    let v = sign * total;
    if v.is_finite() { Some(v) } else { None }
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
            Some((next_indent, _)) => {
                cur_indent = next_indent;
            }
            None => return None,
        }
    }
    None
}

fn yaml_scalar(text: &str, path: &str) -> Option<String> {
    let raw = yaml_val(text, path)?;
    if raw.contains("val:") {
        raw.split("val:")
            .nth(1)
            .and_then(|s| s.split([',', '}']).next())
            .map(|s| s.trim().to_string())
    } else {
        Some(raw.trim().to_string())
    }
}

pub fn yaml_sexagesimal(text: &str, path: &str) -> Option<f64> {
    parse_sexagesimal(&yaml_scalar(text, path)?)
}

pub fn yaml_degrees(text: &str, path: &str) -> Option<f64> {
    let v: f64 = yaml_scalar(text, path)?.parse().ok()?;
    v.is_finite().then_some(v)
}

pub fn meta_source_id(text: &str) -> Option<u32> {
    let rest = text.split("source_id").nth(1)?;
    let rest = rest.trim_start().strip_prefix(':')?.trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
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
        assert!((table.rows[0][0].unwrap() - 0.25).abs() < 1e-9);
        assert!((table.rows[0][1].unwrap() - 1.36e-7).abs() < 1e-15);
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
        assert_eq!(yaml_val(YAML, "source_id").as_deref(), Some("58"));
    }

    const SED_UL: &str = r#"# %ECSV 0.9
# ---
# datatype:
# - {name: e_ref, datatype: float32, unit: TeV}
# - {name: dnde, datatype: float32, unit: TeV-1 cm-2 s-1}
# - {name: dnde_errn, datatype: float32, unit: TeV-1 cm-2 s-1}
# - {name: dnde_errp, datatype: float32, unit: TeV-1 cm-2 s-1}
# - {name: dnde_ul, datatype: float32, unit: cm-2 s-1 TeV-1}
# meta: !!omap
# - {data_type: sed}
# - {source_id: 14}
# - {reference_id: 2009ApJ...700.1034A}
# - {telescope: veritas}
# - UL_CONF: 0.95
e_ref dnde dnde_errn dnde_errp dnde_ul
0.779   4.858e-12  2.256e-12  2.193e-12  nan
1.232   1.346e-12  6.654e-13  7.1e-13  nan
1.957   3.873e-13  2.178e-13  2.218e-13  nan
3.095   1.364e-13  8.505e-14  8.7e-14 nan
4.977   nan  nan  nan 1.094e-13
7.895   nan  nan  nan 3.248e-14
"#;

    const REGISTRY_014: &str = r#"---
source_id: 14
common_name: LS I +61 303
where: gal
pos:
  simbad_id: LS I +61 303
  ra: 40.131938163
  dec: 61.229336515
reference_id:
  - 2009ApJ...700.1034A
"#;

    #[test]
    fn ecsv_keeps_rows_with_absent_cells() {
        let table = parse_ecsv(SED_UL).unwrap();
        assert_eq!(table.columns.len(), 5);
        assert_eq!(table.rows.len(), 6);
        assert_eq!(table.rows[0][1].unwrap(), 4.858e-12);
        assert_eq!(table.rows[0][4], None);
        assert_eq!(table.rows[4][1], None);
        assert_eq!(table.rows[4][4].unwrap(), 1.094e-13);
    }

    #[test]
    fn meta_source_id_reads_inline_and_block_forms() {
        assert_eq!(meta_source_id(SED_UL), Some(14));
        assert_eq!(meta_source_id(SED), Some(58));
        assert_eq!(meta_source_id("e_ref dnde\n1.0 2.0\n"), None);
    }

    #[test]
    fn yaml_degrees_reads_decimal_pos() {
        let ra = yaml_degrees(REGISTRY_014, "pos.ra").unwrap();
        assert!((ra - 40.131938163).abs() < 1e-9, "ra {ra}");
        let dec = yaml_degrees(REGISTRY_014, "pos.dec").unwrap();
        assert!((dec - 61.229336515).abs() < 1e-9, "dec {dec}");
    }
}
