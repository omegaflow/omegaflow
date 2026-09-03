use omegaflow::cdn::upload_asset;
use omegaflow::json::{JsonVal, parse_json};
use std::process::Command;

const ROOT: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
const H0_CF4: f64 = 75.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn tap_query(root: &str, adql: &str) -> Option<String> {
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
        .arg(root)
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

fn dist_mpc(dm: f64) -> f64 {
    10f64.powf(dm / 5.0 + 1.0) / 1e6
}

fn vpec(vcmb: f64, dm: f64) -> f64 {
    vcmb - H0_CF4 * dist_mpc(dm)
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

fn cell_num(c: &JsonVal) -> Option<f64> {
    match c {
        JsonVal::Num(v) => Some(*v),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn fetch_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<f64>>)> {
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
                match c {
                    JsonVal::Null => row.push(f64::NAN),
                    other => row.push(cell_num(other)?),
                }
            }
            if row.len() == names.len() && !row.is_empty() {
                rows.push(row);
            }
        }
    }
    Some((names, rows))
}

fn write_json(rows: &[(f64, f64, f64, f64)], path: &str) -> bool {
    let mut out = String::with_capacity(rows.len() * 64 + 2);
    out.push('[');
    for (i, (ra, dec, dist, vp)) in rows.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"ra\":{},\"dec\":{},\"dist_mpc\":{},\"vpec\":{}}}",
            ra, dec, dist, vp
        ));
    }
    out.push(']');
    std::fs::write(path, out.as_bytes()).is_ok()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = arg_value(&args, "--out").unwrap_or_else(|| "cosmicflows_cf4.json".to_string());
    let ci_mode = has_flag(&args, "--ci-mode");

    let adql = "SELECT PGC, RAJ2000, DEJ2000, DM, Vcmb \
                FROM \"J/ApJ/944/94/table2\" \
                WHERE DM IS NOT NULL AND Vcmb IS NOT NULL";
    let Some(body) = tap_query(ROOT, adql) else {
        std::process::exit(1);
    };
    let Some((names, rows)) = fetch_rows(&body) else {
        eprintln!("TAP response shape unread");
        std::process::exit(1);
    };
    let idx = |n: &str| names.iter().position(|x| x == n);
    let (Some(i_ra), Some(i_dec), Some(i_dm), Some(i_vcmb)) =
        (idx("RAJ2000"), idx("DEJ2000"), idx("DM"), idx("Vcmb"))
    else {
        eprintln!("columns {names:?} lack RAJ2000/DEJ2000/DM/Vcmb");
        std::process::exit(1);
    };
    let mut out_rows: Vec<(f64, f64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    for row in &rows {
        let ra = row[i_ra];
        let dec = row[i_dec];
        let dm = row[i_dm];
        let vcmb = row[i_vcmb];
        if !ra.is_finite() || !dec.is_finite() || !dm.is_finite() || !vcmb.is_finite() {
            skipped += 1;
            continue;
        }
        let d = dist_mpc(dm);
        let v = vpec(vcmb, dm);
        if !d.is_finite() || d <= 0.0 || !v.is_finite() {
            skipped += 1;
            continue;
        }
        out_rows.push((ra, dec, d, v));
    }
    eprintln!(
        "{} rows, {} skipped, dist {:.3}..{:.1} Mpc, vpec {:.0}..{:.0} km/s",
        out_rows.len(),
        skipped,
        out_rows.iter().map(|r| r.2).fold(f64::INFINITY, f64::min),
        out_rows.iter().map(|r| r.2).fold(0.0, f64::max),
        out_rows.iter().map(|r| r.3).fold(f64::INFINITY, f64::min),
        out_rows
            .iter()
            .map(|r| r.3)
            .fold(f64::NEG_INFINITY, f64::max),
    );
    if out_rows.is_empty() {
        eprintln!("no rows — the catalog stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if !write_json(&out_rows, &out) {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match std::fs::read_to_string(&out)
        .ok()
        .and_then(|s| parse_json(&s))
    {
        Some(JsonVal::Arr(arr)) => eprintln!("{out}: {} rows roundtrip-parse", arr.len()),
        _ => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode {
        let _ = upload_asset(&out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_modulus_to_mpc() {
        assert!((dist_mpc(35.0) - 100.0).abs() < 1e-9);
        assert!((dist_mpc(30.0) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn peculiar_velocity_zero_point() {
        assert!(vpec(7500.0, 35.0).abs() < 1e-9);
        assert!((vpec(7500.0 + 300.0, 35.0) - 300.0).abs() < 1e-9);
    }

    #[test]
    fn fetch_rows_json() {
        let body = r#"{"metadata":[{"name":"PGC"},{"name":"RAJ2000"},{"name":"DEJ2000"},{"name":"DM"},{"name":"Vcmb"}],"data":[[1186356,44.9736,1.16,35.0,7500.0],[11465,45.5982,0.9793,null,8312.0]]}"#;
        let (names, rows) = fetch_rows(body).unwrap();
        assert_eq!(names, vec!["PGC", "RAJ2000", "DEJ2000", "DM", "Vcmb"]);
        assert_eq!(rows.len(), 2);
        assert!(rows[1][3].is_nan());
    }
}
