use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use omegaflow::archivar::json::{JsonVal, parse_json};

fn take_value(args: &[String], i: &mut usize) -> Result<String, String> {
    let flag = args[*i].clone();
    let value = args
        .get(*i + 1)
        .cloned()
        .ok_or_else(|| format!("{flag}: value missing"))?;
    *i += 2;
    Ok(value)
}

fn split_line(line: &str, delim: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for ch in line.chars() {
        match ch {
            '"' => quoted = !quoted,
            c if c == delim && !quoted => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            c => cur.push(c),
        }
    }
    out.push(cur.trim().to_string());
    out
}

fn norm(h: &str) -> String {
    let lower = h.to_lowercase();
    let base = match (lower.find('['), lower.find('(')) {
        (Some(i), Some(j)) => &lower[..i.min(j)],
        (Some(i), None) => &lower[..i],
        (None, Some(j)) => &lower[..j],
        (None, None) => &lower[..],
    };
    base.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

fn admitted(token: &str) -> bool {
    !token.is_empty()
        && !matches!(
            token,
            "no-license-stated"
                | "all-rights-reserved"
                | "auth-required"
                | "registration-required"
                | "no-license"
                | "no-redistribution"
                | "unclear"
                | "unknown"
        )
        && !token.ends_with("-permission-required")
}

fn parse_map(spec: &str, flag: &str) -> Result<BTreeMap<String, String>, String> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for pair in spec.split(',').filter(|p| !p.trim().is_empty()) {
        let Some((key, value)) = pair.split_once('=') else {
            return Err(format!("{flag}: '{pair}' is not file=value"));
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() {
            return Err(format!("{flag}: '{pair}' hat einen leeren Teil"));
        }
        map.insert(key.to_string(), value.to_string());
    }
    Ok(map)
}

struct Table {
    file: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    anchor: usize,
    terms: String,
    channel_ref: String,
}

fn read_table(
    path: &str,
    anchor: &str,
    terms: &BTreeMap<String, String>,
    channels: &BTreeMap<String, String>,
) -> Result<Table, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut lines = text
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'));
    let first = lines.next().ok_or_else(|| format!("{path}: leer"))?;
    let delim = if first.contains('\t') {
        '\t'
    } else if first.contains(';') {
        ';'
    } else {
        ','
    };
    let header = split_line(first, delim);
    let want = norm(anchor);
    let at = header.iter().position(|h| norm(h) == want).ok_or_else(|| {
        format!(
            "{path}: kein Anker '{anchor}' im Kopf ({})",
            header.join(",")
        )
    })?;
    let mut rows = Vec::new();
    for line in lines {
        let row = split_line(line, delim);
        if row.len() != header.len() {
            continue;
        }
        rows.push(row);
    }
    let term = match terms.get(path) {
        Some(t) => t.clone(),
        None => "unset".to_string(),
    };
    let channel_ref = match channels.get(path) {
        Some(c) => c.clone(),
        None => "absent".to_string(),
    };
    Ok(Table {
        file: path.to_string(),
        header,
        rows,
        anchor: at,
        terms: term,
        channel_ref,
    })
}

const FIND_COLUMNS: [&str; 8] = [
    "url",
    "title",
    "header_state",
    "value_state",
    "hex_state",
    "fits_state",
    "verdict",
    "terms",
];

fn json_scalar(value: &JsonVal) -> String {
    match value {
        JsonVal::Str(s) => s.clone(),
        JsonVal::Num(n) => format!("{n}"),
        JsonVal::Bool(b) => b.to_string(),
        JsonVal::Null => String::new(),
        _ => String::new(),
    }
}

fn read_find(
    path: &str,
    terms: &BTreeMap<String, String>,
    channels: &BTreeMap<String, String>,
) -> Result<Table, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut rows = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let Some(JsonVal::Obj(obj)) = parse_json(line) else {
            return Err(format!("{path}:{}: no JSON line", n + 1));
        };
        let row: Vec<String> = FIND_COLUMNS
            .iter()
            .map(|col| match obj.get(*col) {
                Some(v) => Ok(json_scalar(v)),
                None => Err(format!("{path}:{}: column '{col}' absent", n + 1)),
            })
            .collect::<Result<Vec<_>, String>>()?;
        if row[0].is_empty() {
            continue;
        }
        rows.push(row);
    }
    let term = match terms.get(path) {
        Some(t) => t.clone(),
        None => "unset".to_string(),
    };
    let channel_ref = match channels.get(path) {
        Some(c) => c.clone(),
        None => "absent".to_string(),
    };
    Ok(Table {
        file: path.to_string(),
        header: FIND_COLUMNS.iter().map(|c| c.to_string()).collect(),
        rows,
        anchor: 0,
        terms: term,
        channel_ref,
    })
}

fn csv_field(v: &str) -> String {
    if v.contains(',') || v.contains('"') || v.contains('\n') {
        format!("\"{}\"", v.replace('"', "\"\""))
    } else {
        v.to_string()
    }
}

enum Source {
    Csv(String),
    Find(String),
}

fn run(args: &[String]) -> Result<(), String> {
    let mut anchor_arg: Option<String> = None;
    let mut out: Option<String> = None;
    let mut terms_spec: Option<String> = None;
    let mut channel_spec: Option<String> = None;
    let mut no_terms = false;
    let mut require_channel_ref = false;
    let mut sources: Vec<Source> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--on" => anchor_arg = Some(take_value(args, &mut i)?),
            "--out" => out = Some(take_value(args, &mut i)?),
            "--terms" => terms_spec = Some(take_value(args, &mut i)?),
            "--channel-ref" => channel_spec = Some(take_value(args, &mut i)?),
            "--from-find" => sources.push(Source::Find(take_value(args, &mut i)?)),
            "--no-terms" => {
                no_terms = true;
                i += 1;
            }
            "--require-channel-ref" => {
                require_channel_ref = true;
                i += 1;
            }
            a if a.starts_with("--") => return Err(format!("unknown option '{a}'")),
            a => {
                sources.push(Source::Csv(a.to_string()));
                i += 1;
            }
        }
    }

    let out = out.ok_or("--out <file> missing")?;
    if terms_spec.is_some() && no_terms {
        return Err("--terms and --no-terms are mutually exclusive".to_string());
    }
    if terms_spec.is_none() && !no_terms {
        return Err(
            "--terms <file=token,...> missing; without a licence pass --no-terms explicitly (writes unset)"
                .to_string(),
        );
    }
    if sources.is_empty() {
        return Err("weave needs at least one source".to_string());
    }
    let has_find = sources.iter().any(|s| matches!(s, Source::Find(_)));
    if !has_find && sources.len() < 2 {
        return Err("weave braucht mindestens zwei Tabellen".to_string());
    }
    let anchor = match &anchor_arg {
        Some(a) => a.clone(),
        None if has_find => "url".to_string(),
        None => return Err("--on <column> missing".to_string()),
    };
    if has_find && norm(&anchor) != "url" {
        return Err(format!(
            "--from-find uses the anchor 'url'; --on '{anchor}' is not compatible"
        ));
    }

    let terms = match &terms_spec {
        Some(spec) => parse_map(spec, "--terms")?,
        None => BTreeMap::new(),
    };
    let channels = match &channel_spec {
        Some(spec) => parse_map(spec, "--channel-ref")?,
        None => BTreeMap::new(),
    };

    let mut tables: Vec<Table> = Vec::new();
    let mut excluded: Vec<(String, String)> = Vec::new();
    for source in &sources {
        let t = match source {
            Source::Csv(path) => read_table(path, &anchor, &terms, &channels)?,
            Source::Find(path) => read_find(path, &terms, &channels)?,
        };
        if terms_spec.is_some() && !admitted(&t.terms) {
            excluded.push((t.file.clone(), t.terms.clone()));
        }
        tables.push(t);
    }
    if !excluded.is_empty() {
        for (f, t) in &excluded {
            eprintln!(
                "weave: WARNUNG {f} — terms '{t}' ist nicht admitiert (gemergt, aber markiert)"
            );
        }
    }
    if require_channel_ref {
        let absent: Vec<&str> = tables
            .iter()
            .filter(|t| t.channel_ref == "absent")
            .map(|t| t.file.as_str())
            .collect();
        if !absent.is_empty() {
            return Err(format!(
                "--require-channel-ref: channel_ref missing (absent) for {}",
                absent.join(", ")
            ));
        }
    }

    let mut cols: Vec<(String, String, String, String)> = Vec::new();
    for t in &tables {
        let stem = match Path::new(&t.file).file_stem() {
            Some(s) => s.to_string_lossy().to_string(),
            None => t.file.clone(),
        };
        for (i, h) in t.header.iter().enumerate() {
            if i == t.anchor {
                continue;
            }
            let mut name = h.clone();
            if cols.iter().any(|(n, _, _, _)| n == &name) {
                name = format!("{h}@{stem}");
            }
            let mut k = 1;
            while cols.iter().any(|(n, _, _, _)| n == &name) {
                name = format!("{h}@{stem}#{k}");
                k += 1;
            }
            cols.push((name, t.file.clone(), h.clone(), norm(h)));
        }
    }

    let mut rows: BTreeMap<String, Vec<Option<String>>> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    for t in &tables {
        for r in &t.rows {
            let key = r[t.anchor].clone();
            if !rows.contains_key(&key) {
                rows.insert(key.clone(), vec![None; cols.len()]);
                order.push(key.clone());
            }
            let slot = rows.get_mut(&key).expect("key present");
            let mut ci = 0;
            for (i, h) in t.header.iter().enumerate() {
                if i == t.anchor {
                    continue;
                }
                while ci < cols.len() && !(cols[ci].1 == t.file && cols[ci].2 == *h) {
                    ci += 1;
                }
                if ci >= cols.len() {
                    break;
                }
                slot[ci] = Some(r[i].clone());
                ci += 1;
            }
        }
    }

    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, c) in cols.iter().enumerate() {
        groups.entry(c.3.clone()).or_default().push(i);
    }

    let mut text = String::new();
    let head: Vec<String> = std::iter::once(anchor.clone())
        .chain(cols.iter().map(|(n, _, _, _)| n.clone()))
        .chain(std::iter::once("__conflict".to_string()))
        .collect();
    text.push_str(
        &head
            .iter()
            .map(|h| csv_field(h))
            .collect::<Vec<_>>()
            .join(","),
    );
    text.push('\n');
    let mut conflict_rows = 0usize;
    for key in &order {
        let values = rows.get(key).expect("row present");
        let mut notes: Vec<String> = Vec::new();
        for (norm_name, idxs) in &groups {
            if idxs.len() < 2 {
                continue;
            }
            let distinct: BTreeSet<&str> = idxs
                .iter()
                .filter_map(|i| values[*i].as_deref())
                .filter(|v| !v.is_empty())
                .collect();
            if distinct.len() > 1 {
                let names: Vec<&str> = idxs.iter().map(|i| cols[*i].0.as_str()).collect();
                notes.push(format!("{}:[{}]", norm_name, names.join("|")));
            }
        }
        if !notes.is_empty() {
            conflict_rows += 1;
        }
        let mut fields = vec![csv_field(key)];
        for v in values {
            fields.push(csv_field(v.as_deref().unwrap_or("")));
        }
        fields.push(csv_field(&notes.join(";")));
        text.push_str(&fields.join(","));
        text.push('\n');
    }
    fs::write(&out, text.as_bytes()).map_err(|e| format!("{out}: {e}"))?;

    let mut map = String::from("out_column\tsource_file\tsource_column\tterms\tchannel_ref\n");
    for (n, f, c, _) in &cols {
        let t = tables.iter().find(|t| &t.file == f);
        let terms = t.map(|t| t.terms.as_str()).unwrap_or("unset");
        let channel_ref = t.map(|t| t.channel_ref.as_str()).unwrap_or("absent");
        map.push_str(&format!("{n}\t{f}\t{c}\t{terms}\t{channel_ref}\n"));
    }
    fs::write(format!("{out}.map"), map.as_bytes()).map_err(|e| format!("{out}.map: {e}"))?;

    let mut edges = String::from("from\tto\trelation\n");
    for i in 0..cols.len() {
        for j in (i + 1)..cols.len() {
            if cols[i].3 != cols[j].3 || cols[i].1 == cols[j].1 {
                continue;
            }
            let mut seen = 0usize;
            let mut same = true;
            for values in rows.values() {
                match (values[i].as_deref(), values[j].as_deref()) {
                    (Some(a), Some(b)) if !a.is_empty() && !b.is_empty() => {
                        seen += 1;
                        if a != b {
                            same = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if same && seen > 0 {
                edges.push_str(&format!(
                    "{}#{}\tmirror-of\t{}#{}\n",
                    cols[i].1, cols[i].2, cols[j].1, cols[j].2
                ));
            }
        }
    }
    for (n, f, c, _) in &cols {
        edges.push_str(&format!("{f}#{c}\tmerge:{out}\tcontributes\n"));
        edges.push_str(&format!("merge:{out}\t{n}\tmaterializes\n"));
    }
    fs::write(format!("{out}.edges"), edges.as_bytes()).map_err(|e| format!("{out}.edges: {e}"))?;

    println!(
        "weave: {} Quellen, {} Zeilen, {} Spalten + Anker -> {out} ({} Konfliktzeilen; Karte {out}.map, Kanten {out}.edges)",
        tables.len(),
        order.len(),
        cols.len(),
        conflict_rows
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("weave: {msg}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let base = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let dir = std::path::PathBuf::from(base)
            .join("scratch")
            .join(format!("weave-{name}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    macro_rules! args {
        ($($item:expr),* $(,)?) => {
            vec![$($item.to_string()),*]
        };
    }

    #[test]
    fn norm_strips_units_and_case() {
        assert_eq!(norm("Discharge [m3/s]"), "discharge");
        assert_eq!(norm("Discharge (m3/s)"), "discharge");
        assert_eq!(norm(" station_id "), "station_id");
    }

    #[test]
    fn admitted_refuses_no_redistribution() {
        assert!(admitted("CC0-1.0"));
        assert!(!admitted("no-redistribution"));
        assert!(!admitted("auth-required"));
    }

    #[test]
    fn weave_marks_conflicts_instead_of_overwriting() {
        let dir = scratch("c");
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,flow\n1,5\n2,6\n").unwrap();
        fs::write(&b, "time,flow\n1,7\n3,8\n").unwrap();
        let out = dir.join("out.csv");
        let args = args![
            "--on",
            "time",
            "--out",
            out.to_string_lossy(),
            "--no-terms",
            a.to_string_lossy(),
            b.to_string_lossy(),
        ];
        run(&args).unwrap();
        let text = fs::read_to_string(&out).unwrap();
        let mut lines = text.lines();
        assert!(lines.next().unwrap().contains("__conflict"));
        let row1 = lines.next().unwrap();
        assert!(row1.contains("1"), "{row1}");
        assert!(row1.contains("flow:"), "{row1}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_marks_non_admitted_terms_but_merges() {
        let dir = scratch("t");
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,v\n1,5\n").unwrap();
        fs::write(&b, "time,v\n1,6\n").unwrap();
        let out = dir.join("out.csv");
        let spec = format!("{}=CC0-1.0,{}=no-redistribution", a.display(), b.display());
        let args = args![
            "--on",
            "time",
            "--out",
            out.to_string_lossy(),
            "--terms",
            spec.as_str(),
            a.to_string_lossy(),
            b.to_string_lossy(),
        ];
        run(&args).unwrap();
        let map = fs::read_to_string(format!("{}.map", out.display())).unwrap();
        assert!(map.contains("no-redistribution"), "{map}");
        assert!(map.contains("CC0-1.0"), "{map}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_marks_identical_columns_as_mirror() {
        let dir = scratch("m");
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,flow\n1,5\n2,6\n").unwrap();
        fs::write(&b, "time,flow\n1,5\n2,6\n").unwrap();
        let out = dir.join("out.csv");
        let args = args![
            "--on",
            "time",
            "--out",
            out.to_string_lossy(),
            "--no-terms",
            a.to_string_lossy(),
            b.to_string_lossy(),
        ];
        run(&args).unwrap();
        let edges = fs::read_to_string(format!("{}.edges", out.display())).unwrap();
        assert!(edges.contains("mirror-of"), "{edges}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_reads_find_jsonl_as_table() {
        let dir = scratch("find");
        let jsonl = dir.join("find.jsonl");
        fs::write(
            &jsonl,
            concat!(
                r#"{"url":"https://example.org/1","title":"One","header_state":"erfüllt","value_state":"erfüllt","hex_state":"","fits_state":"","verdict":"MATCH","terms":"CC0-1.0"}"#,
                "\n",
                r#"{"url":"https://example.org/2","title":"Two","header_state":"fehlt","value_state":"","hex_state":"","fits_state":"","verdict":"kein-match","terms":"unset"}"#,
                "\n"
            ),
        )
        .unwrap();
        let out = dir.join("out.csv");
        let args = args![
            "--from-find",
            jsonl.to_string_lossy(),
            "--out",
            out.to_string_lossy(),
            "--no-terms",
        ];
        run(&args).unwrap();
        let text = fs::read_to_string(&out).unwrap();
        let mut lines = text.lines();
        let head = lines.next().unwrap();
        assert!(head.starts_with("url,title,header_state"), "{head}");
        assert!(head.ends_with("__conflict"), "{head}");
        assert_eq!(lines.count(), 2);
        let map = fs::read_to_string(format!("{}.map", out.display())).unwrap();
        assert!(map.starts_with("out_column\tsource_file\tsource_column\tterms\tchannel_ref"));
        assert!(map.contains("\tabsent"), "{map}");
        let edges = fs::read_to_string(format!("{}.edges", out.display())).unwrap();
        assert!(edges.contains("contributes"), "{edges}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_from_find_marks_conflicts() {
        let dir = scratch("find-conflict");
        let a = dir.join("a.jsonl");
        let b = dir.join("b.jsonl");
        fs::write(
            &a,
            r#"{"url":"https://example.org/1","title":"One","header_state":"erfüllt","value_state":"","hex_state":"","fits_state":"","verdict":"MATCH","terms":"unset"}"#,
        )
        .unwrap();
        fs::write(
            &b,
            r#"{"url":"https://example.org/1","title":"Other","header_state":"fehlt","value_state":"","hex_state":"","fits_state":"","verdict":"MATCH","terms":"unset"}"#,
        )
        .unwrap();
        let out = dir.join("out.csv");
        let args = args![
            "--from-find",
            a.to_string_lossy(),
            "--from-find",
            b.to_string_lossy(),
            "--out",
            out.to_string_lossy(),
            "--no-terms",
        ];
        run(&args).unwrap();
        let text = fs::read_to_string(&out).unwrap();
        let row = text.lines().nth(1).unwrap();
        assert!(row.contains("title:"), "{row}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_requires_terms_unless_no_terms() {
        let dir = scratch("terms-required");
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,v\n1,5\n").unwrap();
        fs::write(&b, "time,v\n1,6\n").unwrap();
        let out = dir.join("out.csv");
        let args = args![
            "--on",
            "time",
            "--out",
            out.to_string_lossy(),
            a.to_string_lossy(),
            b.to_string_lossy(),
        ];
        let err = run(&args).unwrap_err();
        assert!(err.contains("--terms"), "{err}");
        assert!(!out.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn weave_map_carries_channel_ref_and_can_require_it() {
        let dir = scratch("channel");
        let a = dir.join("a.csv");
        let b = dir.join("b.csv");
        fs::write(&a, "time,v\n1,5\n").unwrap();
        fs::write(&b, "time,v\n1,6\n").unwrap();
        let out = dir.join("out.csv");
        let spec = format!("{}=antenna-1,{}=gain-2", a.display(), b.display());
        let args = args![
            "--on",
            "time",
            "--out",
            out.to_string_lossy(),
            "--no-terms",
            "--channel-ref",
            spec.as_str(),
            a.to_string_lossy(),
            b.to_string_lossy(),
        ];
        run(&args).unwrap();
        let map = fs::read_to_string(format!("{}.map", out.display())).unwrap();
        assert!(map.contains("antenna-1"), "{map}");
        assert!(map.contains("gain-2"), "{map}");

        let out2 = dir.join("out2.csv");
        let args = args![
            "--on",
            "time",
            "--out",
            out2.to_string_lossy(),
            "--no-terms",
            "--require-channel-ref",
            a.to_string_lossy(),
            b.to_string_lossy(),
        ];
        let err = run(&args).unwrap_err();
        assert!(err.contains("channel_ref"), "{err}");
        assert!(!out2.exists());
        let _ = fs::remove_dir_all(&dir);
    }
}
