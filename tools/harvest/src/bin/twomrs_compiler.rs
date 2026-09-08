use omegaflow::archivar::membrane::MAX_SAMPLES;
use omegaflow::cdn::upload_release;
use omegaflow::json::{parse_json, JsonVal};
use omegaflow::twomrs::{read_bin, record, row_record, write_bin};
use std::process::Command;

const ROOT: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const NETLOC: &str = "tapvizier.cds.unistra.fr";
const ADQL: &str = r#"SELECT RAJ2000, DEJ2000, cz, e_cz FROM "J/ApJS/199/26/table3""#;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn tap_query(adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=json")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(ROOT)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap_query http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn as_arr(v: &JsonVal) -> Option<&Vec<JsonVal>> {
    match v {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn as_obj(v: &JsonVal) -> Option<&std::collections::HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(o) => Some(o),
        _ => None,
    }
}

fn cell_opt(c: &JsonVal) -> Option<Option<f64>> {
    match c {
        JsonVal::Null => Some(None),
        JsonVal::Num(v) => Some(Some(*v)),
        JsonVal::Str(s) => Some(s.parse::<f64>().ok()),
        _ => None,
    }
}

fn fetch_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<Option<f64>>>)> {
    let parsed = parse_json(body)?;
    let obj = as_obj(&parsed)?;
    let mut names = Vec::new();
    if let Some(meta) = obj.get("metadata").and_then(as_arr) {
        for md in meta {
            if let Some(o) = as_obj(md) {
                if let Some(nm) = o.get("name").and_then(|n| match n {
                    JsonVal::Str(s) => Some(s.clone()),
                    _ => None,
                }) {
                    names.push(nm);
                }
            }
        }
    }
    let data = obj.get("data").and_then(as_arr)?;
    let mut rows = Vec::new();
    for r in data {
        if let Some(cells) = as_arr(r) {
            let mut row = Vec::with_capacity(cells.len());
            for c in cells {
                match cell_opt(c) {
                    Some(v) => row.push(v),
                    None => return None,
                }
            }
            if row.len() == names.len() && !row.is_empty() {
                rows.push(row);
            }
        }
    }
    Some((names, rows))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(out) = arg_value(&args, "--out-bin") else {
        eprintln!("--out-bin absent — the asset path stays unnamed");
        std::process::exit(2);
    };
    let ci = has_flag(&args, "--ci-mode");

    let Some(body) = tap_query(ADQL) else {
        std::process::exit(1);
    };
    let Some((names, rows)) = fetch_rows(&body) else {
        eprintln!("TAP response shape unread");
        std::process::exit(1);
    };
    let idx = |n: &str| names.iter().position(|x| x == n);
    let (Some(i_ra), Some(i_dec), Some(i_cz), Some(i_ec)) =
        (idx("RAJ2000"), idx("DEJ2000"), idx("cz"), idx("e_cz"))
    else {
        eprintln!("columns {names:?} lack RAJ2000/DEJ2000/cz/e_cz");
        std::process::exit(1);
    };
    let mut out_rows: Vec<[f64; 4]> = Vec::new();
    let mut skipped = 0usize;
    for row in &rows {
        let Some(r) = record(row[i_ra], row[i_dec], row[i_cz], row[i_ec]) else {
            skipped += 1;
            continue;
        };
        out_rows.push(row_record(&r));
    }
    eprintln!(
        "{} rows read, {} records, {} skipped (cz absent or implausible)",
        rows.len(),
        out_rows.len(),
        skipped
    );
    if out_rows.is_empty() {
        eprintln!("no records — the catalog stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if out_rows.len() > MAX_SAMPLES {
        eprintln!(
            "selection carries {} sources over MAX_SAMPLES {MAX_SAMPLES} — the asset stays unwritten",
            out_rows.len()
        );
        std::process::exit(1);
    }
    let mut cz_min = f64::INFINITY;
    let mut cz_max = f64::NEG_INFINITY;
    for r in &out_rows {
        cz_min = cz_min.min(r[2]);
        cz_max = cz_max.max(r[2]);
    }
    let bin = write_bin(&out_rows);
    let Some(parsed) = read_bin(&bin) else {
        eprintln!("roundtrip parse void — the asset stays unwritten");
        std::process::exit(1);
    };
    if parsed.len() != out_rows.len() {
        eprintln!(
            "roundtrip count {} != {} — the asset stays unwritten",
            parsed.len(),
            out_rows.len()
        );
        std::process::exit(1);
    }
    if std::fs::write(&out, &bin).is_err() {
        eprintln!("write {out} void");
        std::process::exit(1);
    }
    eprintln!(
        "{out}: {} records, cz {cz_min:.0}..{cz_max:.0} km/s, {} B — roundtrip parses",
        parsed.len(),
        bin.len()
    );
    if ci && !upload_release(NETLOC, &out) {
        eprintln!("{out}: CDN upload returned void");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_rows_parses_json_cells() {
        let body = r#"{"metadata":[{"name":"RAJ2000"},{"name":"DEJ2000"},{"name":"cz"},{"name":"e_cz"}],"data":[[183.25,-2.5,9372,14],[46.28756,1.09348,6955,5],[null,null,null,null]]}"#;
        let (names, rows) = fetch_rows(body).unwrap();
        assert_eq!(names, vec!["RAJ2000", "DEJ2000", "cz", "e_cz"]);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0], Some(183.25));
        assert_eq!(rows[0][2], Some(9372.0));
        assert_eq!(rows[2][2], None);
    }

    #[test]
    fn adql_selects_the_twomrs_columns() {
        assert!(ADQL.contains("RAJ2000"));
        assert!(ADQL.contains("DEJ2000"));
        assert!(ADQL.contains("cz"));
        assert!(ADQL.contains("e_cz"));
        assert!(ADQL.contains("J/ApJS/199/26/table3"));
    }
}
