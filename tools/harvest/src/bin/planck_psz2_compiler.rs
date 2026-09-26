use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use std::collections::HashMap;
use std::process::Command;

const NETLOC: &str = "irsa.ipac.caltech.edu";
const TAP_SYNC: &str = "https://irsa.ipac.caltech.edu/TAP/sync";
const QUERY: &str = "SELECT TOP 5000 name,ra,dec,snr FROM planck.com_pccs2_sz_mmf3";
const UNION_QUERY: &str = "SELECT TOP 5000 name,indx,redshift,msz FROM planck.com_pccs2_sz_union";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn csv_line(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            out.push(std::mem::take(&mut cur));
        } else if c != '\r' {
            cur.push(c);
        }
    }
    out.push(cur);
    out
}

fn fetch_csv(adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(TAP_SYNC)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "planck_psz2_compiler: tap http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn col_index(headers: &[String], name: &str) -> Option<usize> {
    headers.iter().position(|h| h == name)
}

fn union_z_map(body: &str) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    let mut lines = body.lines().filter(|l| !l.trim().is_empty());
    let Some(header_line) = lines.next() else {
        return map;
    };
    let headers = csv_line(header_line);
    let (Some(i_name), Some(i_redshift)) =
        (col_index(&headers, "name"), col_index(&headers, "redshift"))
    else {
        return map;
    };
    for line in lines {
        let cells = csv_line(line);
        if cells.len() != headers.len() {
            continue;
        }
        let name = match cells.get(i_name) {
            Some(n) => n.trim().to_string(),
            None => continue,
        };
        let z = match cells
            .get(i_redshift)
            .and_then(|s| s.trim().parse::<f64>().ok())
            .and_then(positive)
        {
            Some(z) => z,
            None => continue,
        };
        map.insert(name, z);
    }
    map
}

fn positive(v: f64) -> Option<f64> {
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

fn ra_deg(v: f64) -> Option<f64> {
    if v.is_finite() && (0.0..360.0).contains(&v) {
        Some(v)
    } else {
        None
    }
}

fn dec_deg(v: f64) -> Option<f64> {
    if v.is_finite() && (-90.0..=90.0).contains(&v) {
        Some(v)
    } else {
        None
    }
}

struct Row {
    ra: f64,
    dec: f64,
    snr: f64,
    z: Option<f64>,
}

fn write_json(rows: &[Row], path: &str) -> bool {
    let mut out = String::with_capacity(rows.len() * 40 + 2);
    out.push('[');
    for (i, row) in rows.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        match row.z {
            Some(z) => out.push_str(&format!(
                "{{\"ra\":{},\"dec\":{},\"snr\":{},\"z\":{}}}",
                row.ra, row.dec, row.snr, z
            )),
            None => out.push_str(&format!(
                "{{\"ra\":{},\"dec\":{},\"snr\":{}}}",
                row.ra, row.dec, row.snr
            )),
        }
    }
    out.push(']');
    if std::fs::write(path, out.as_bytes()).is_err() {
        eprintln!("planck_psz2_compiler: write {path} returned void");
        return false;
    }
    true
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match arg_value(&args, "--out") {
        Some(path) => path,
        None => {
            eprintln!("planck_psz2_compiler: --out (path) required");
            std::process::exit(2);
        }
    };
    let ci_mode = has_flag(&args, "--ci-mode");

    let body = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("planck_psz2_compiler: read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let adql = match arg_value(&args, "--query") {
                Some(q) => q,
                None => QUERY.to_string(),
            };
            match fetch_csv(&adql) {
                Some(b) => b,
                None => {
                    eprintln!(
                        "planck_psz2_compiler: tap csv returned void — the bin stays unwritten"
                    );
                    std::process::exit(1);
                }
            }
        }
    };

    let union_body = match arg_value(&args, "--union") {
        Some(path) => match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("planck_psz2_compiler: read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let adql = match arg_value(&args, "--union-query") {
                Some(q) => q,
                None => UNION_QUERY.to_string(),
            };
            match fetch_csv(&adql) {
                Some(b) => b,
                None => {
                    eprintln!(
                        "planck_psz2_compiler: union tap csv returned void — the bin stays unwritten"
                    );
                    std::process::exit(1);
                }
            }
        }
    };
    let zmap = union_z_map(&union_body);
    eprintln!(
        "planck_psz2_compiler: union crossmatch map holds {} positive redshifts",
        zmap.len()
    );

    let mut lines = body.lines().filter(|l| !l.trim().is_empty());
    let header_line = match lines.next() {
        Some(h) => h,
        None => {
            eprintln!(
                "planck_psz2_compiler: csv header void — the bin stays unwritten (0 honored)"
            );
            std::process::exit(1);
        }
    };
    let headers = csv_line(header_line);
    let (Some(i_name), Some(i_ra), Some(i_dec), Some(i_snr)) = (
        col_index(&headers, "name"),
        col_index(&headers, "ra"),
        col_index(&headers, "dec"),
        col_index(&headers, "snr"),
    ) else {
        eprintln!(
            "planck_psz2_compiler: header lacks name/ra/dec/snr ({} columns: {}) — the bin stays unwritten",
            headers.len(),
            headers.join(",")
        );
        std::process::exit(1);
    };

    let mut rows: Vec<Row> = Vec::new();
    let mut skipped = 0usize;
    let mut malformed = 0usize;
    let mut z_matched = 0usize;
    for line in lines {
        let cells = csv_line(line);
        if cells.len() != headers.len() {
            malformed += 1;
            continue;
        }
        let parse =
            |i: usize| -> Option<f64> { cells.get(i).and_then(|s| s.trim().parse::<f64>().ok()) };
        let (Some(ra), Some(dec), Some(snr)) = (
            parse(i_ra).and_then(ra_deg),
            parse(i_dec).and_then(dec_deg),
            parse(i_snr).and_then(positive),
        ) else {
            skipped += 1;
            continue;
        };
        let z = cells
            .get(i_name)
            .map(|s| s.trim())
            .and_then(|n| zmap.get(n))
            .copied();
        if z.is_some() {
            z_matched += 1;
        }
        rows.push(Row { ra, dec, snr, z });
    }

    eprintln!(
        "planck_psz2_mmf3: {} rows, {} rows z-crossmatched, {} rows skipped (invalid/absent ra/dec/snr), {} malformed lines",
        rows.len(),
        z_matched,
        skipped,
        malformed
    );
    if rows.is_empty() {
        eprintln!("planck_psz2_compiler: no rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if !write_json(&rows, &out) {
        std::process::exit(1);
    }
    match std::fs::read_to_string(&out)
        .ok()
        .and_then(|s| parse_json(&s))
    {
        Some(JsonVal::Arr(arr)) if arr.len() == rows.len() => {
            eprintln!(
                "planck_psz2_compiler: {out}: {} rows roundtrip-parse",
                arr.len()
            );
        }
        _ => {
            eprintln!(
                "planck_psz2_compiler: {out}: roundtrip parse void — the asset stays unverified"
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("planck_psz2_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_line_splits_18_columns_and_keeps_the_space_in_name() {
        let line = "901,PSZ2 G198.73+13.34,198.7326,13.3435,108.7047,18.5210,5.3798,6.0292,0.1,33.981,0,0.015342,-0.3040818886283351,0.8981264986224453,0.3176523216984437,222301100,16154502722473,694";
        let cells = csv_line(line);
        assert_eq!(cells.len(), 18);
        assert_eq!(cells[1], "PSZ2 G198.73+13.34");
        assert_eq!(cells[4], "108.7047");
        assert_eq!(cells[7], "6.0292");
    }

    #[test]
    fn csv_line_reads_quoted_cell_with_comma() {
        let cells = csv_line("1,\"a,b\",3");
        assert_eq!(cells.len(), 3);
        assert_eq!(cells[1], "a,b");
    }

    #[test]
    fn plausibility_gates_reject_nonpositive_and_nonfinite() {
        assert_eq!(positive(6.0292), Some(6.0292));
        assert_eq!(positive(0.0), None);
        assert_eq!(positive(-1.0), None);
        assert_eq!(positive(f64::NAN), None);
        assert_eq!(positive(f64::INFINITY), None);
        assert_eq!(ra_deg(108.7047), Some(108.7047));
        assert_eq!(ra_deg(360.0), None);
        assert_eq!(ra_deg(-0.5), None);
        assert_eq!(dec_deg(-15.5702), Some(-15.5702));
        assert_eq!(dec_deg(90.1), None);
    }

    #[test]
    fn union_z_map_keeps_positive_redshift_and_drops_sentinel() {
        let body = "name,indx,redshift,msz\nPSZ2 G198.73+13.34,901,-1.000,0\nPSZ2 G134.52+31.98,656,0.129,2.4713\nPSZ2 G134.59+53.38,657,0.345,4.4549\n";
        let map = union_z_map(body);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("PSZ2 G134.52+31.98"), Some(&0.129));
        assert_eq!(map.get("PSZ2 G134.59+53.38"), Some(&0.345));
        assert!(!map.contains_key("PSZ2 G198.73+13.34"));
    }
}
