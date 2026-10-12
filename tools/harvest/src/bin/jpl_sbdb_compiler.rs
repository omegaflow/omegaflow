use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, parse_json};
use omegaflow::mpcorb::{
    DESIG_BYTES, FLAG_G_PRESENT, FLAG_H_PRESENT, FLAG_NUMBER_PRESENT, MpcorbRec, RECORD_STRIDE,
    encode_record, parse_record,
};
use std::collections::HashMap;
use std::process::Command;

const BASE_URL: &str = "https://ssd-api.jpl.nasa.gov/sbdb_query.api?fields=full_name,a,e,i,om,w,ma,epoch,H,G&sb-kind=a&full-prec=true";
const CDN_TAG: &str = "ssd-api.jpl.nasa.gov";

fn curl(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("--max-time")
        .arg("120")
        .arg(url)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

fn field_index_map(fields: &[JsonVal]) -> HashMap<String, usize> {
    fields
        .iter()
        .enumerate()
        .filter_map(|(k, f)| match f {
            JsonVal::Str(s) => Some((s.clone(), k)),
            _ => None,
        })
        .collect()
}

fn cell_str(row: &[JsonVal], idx: usize) -> Option<&str> {
    match row.get(idx) {
        Some(JsonVal::Str(s)) => Some(s),
        _ => None,
    }
}

fn cell_f64(row: &[JsonVal], idx: usize) -> Option<f64> {
    cell_str(row, idx)?.trim().parse::<f64>().ok()
}

fn cell_f64_opt(row: &[JsonVal], idx: &HashMap<String, usize>, key: &str) -> Option<f64> {
    let i = *idx.get(key)?;
    cell_f64(row, i)
}

fn parse_sbdb_rows(body: &str) -> Option<(Vec<Vec<JsonVal>>, i64, HashMap<String, usize>)> {
    let root = parse_json(body)?;
    let (fields, data, count) = match &root {
        JsonVal::Obj(m) => (
            m.get("fields")?,
            m.get("data")?,
            m.get("count").and_then(|c| match c {
                JsonVal::Num(n) => Some(*n as i64),
                _ => None,
            })?,
        ),
        _ => return None,
    };
    let JsonVal::Arr(fields) = fields else {
        return None;
    };
    let idx = field_index_map(fields);
    let JsonVal::Arr(rows) = data else {
        return None;
    };
    let rows: Vec<Vec<JsonVal>> = rows
        .iter()
        .filter_map(|r| match r {
            JsonVal::Arr(cells) => Some(cells.clone()),
            _ => None,
        })
        .collect();
    Some((rows, count, idx))
}

fn paren_inner(t: &str) -> Option<&str> {
    let open = t.find('(')?;
    let close = t.find(')')?;
    if close > open + 1 {
        Some(&t[open + 1..close])
    } else {
        None
    }
}

fn leading_number(t: &str) -> (Option<u32>, &str) {
    let bytes = t.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 || (i < bytes.len() && bytes[i] != b' ') {
        return (None, t);
    }
    match t[..i].parse::<u32>() {
        Ok(n) => (Some(n), t[i..].trim_start()),
        Err(_) => (None, t),
    }
}

fn split_name(text: &str) -> Option<(Option<u32>, String)> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    let (number, rest) = leading_number(t);
    let desig = match paren_inner(t) {
        Some(inner) => inner.trim().to_string(),
        None => rest.trim().to_string(),
    };
    if desig.is_empty() {
        return None;
    }
    Some((number, desig))
}

fn sbdb_rec(row: &[JsonVal], idx: &HashMap<String, usize>) -> Option<MpcorbRec> {
    let name = cell_str(row, *idx.get("full_name")?)?;
    let (number, desig) = split_name(name)?;
    let bytes = desig.as_bytes();
    if bytes.is_empty() || bytes.len() > DESIG_BYTES {
        return None;
    }
    let epoch_jd = cell_f64(row, *idx.get("epoch")?)?;
    let a_au = cell_f64(row, *idx.get("a")?)?;
    let e = cell_f64(row, *idx.get("e")?)?;
    let incl_deg = cell_f64(row, *idx.get("i")?)?;
    let node_deg = cell_f64(row, *idx.get("om")?)?;
    let peri_deg = cell_f64(row, *idx.get("w")?)?;
    let ma_deg = cell_f64(row, *idx.get("ma")?)?;
    if !epoch_jd.is_finite() || !a_au.is_finite() || a_au <= 0.0 {
        return None;
    }
    if !e.is_finite() || !(0.0..1.0).contains(&e) {
        return None;
    }
    if !incl_deg.is_finite()
        || !node_deg.is_finite()
        || !peri_deg.is_finite()
        || !ma_deg.is_finite()
    {
        return None;
    }
    let mut desig_bytes = [0u8; DESIG_BYTES];
    desig_bytes[..bytes.len()].copy_from_slice(bytes);
    let mut flags = 0u8;
    let mut number_u32 = 0u32;
    if let Some(n) = number {
        number_u32 = n;
        flags |= FLAG_NUMBER_PRESENT;
    }
    let mut h_mag = 0.0f32;
    let mut g_mag = 0.0f32;
    if let Some(h) = cell_f64_opt(row, idx, "H")
        && h.is_finite()
    {
        h_mag = h as f32;
        flags |= FLAG_H_PRESENT;
    }
    if let Some(g) = cell_f64_opt(row, idx, "G")
        && g.is_finite()
    {
        g_mag = g as f32;
        flags |= FLAG_G_PRESENT;
    }
    Some(MpcorbRec {
        number: number_u32,
        desig: desig_bytes,
        epoch_jd,
        a_au,
        e,
        incl_deg,
        node_deg,
        peri_deg,
        ma_deg,
        h_mag,
        g_mag,
        flags,
    })
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let usage =
        "usage: jpl_sbdb_compiler --out <jpl_sbdb.bin> [--limit <n>] [--page <n>] [--ci-mode]";
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    };
    let limit: usize = match arg_value(&args, "--limit") {
        Some(v) => match v.parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("--limit {v}: {e}");
                std::process::exit(2);
            }
        },
        None => 0,
    };
    let page: usize = match arg_value(&args, "--page") {
        Some(v) => match v.parse() {
            Ok(n) if n > 0 => n,
            Ok(n) => {
                eprintln!("--page {n} carries no positive page bound");
                std::process::exit(2);
            }
            Err(e) => {
                eprintln!("--page {v}: {e}");
                std::process::exit(2);
            }
        },
        None => 1000,
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let mut buf: Vec<u8> = Vec::new();
    let mut records = 0usize;
    let mut rows_seen = 0usize;
    let mut skipped = 0usize;
    let mut pages = 0usize;
    let mut total_count: i64 = -1;
    let mut from = 0usize;
    loop {
        let url = format!("{BASE_URL}&limit={page}&limit-from={from}");
        let body = match curl(&url) {
            Some(b) => b,
            None => {
                eprintln!("sbdb page {from}: fetch void");
                break;
            }
        };
        let (rows, count, idx) = match parse_sbdb_rows(&body) {
            Some(v) => v,
            None => {
                eprintln!("sbdb page {from}: json shape void");
                break;
            }
        };
        if total_count < 0 {
            total_count = count;
        }
        if rows.is_empty() {
            break;
        }
        pages += 1;
        for row in &rows {
            rows_seen += 1;
            match sbdb_rec(row, &idx) {
                Some(rec) => {
                    encode_record(&rec, &mut buf);
                    records += 1;
                }
                None => skipped += 1,
            }
            if limit > 0 && rows_seen >= limit {
                break;
            }
        }
        if limit > 0 && rows_seen >= limit {
            break;
        }
        if rows.len() < page {
            break;
        }
        from += page;
    }

    eprintln!(
        "jpl_sbdb: pages {pages}, rows {rows_seen}, emitted {records}, skipped {skipped}, api count {total_count}"
    );
    if records == 0 {
        eprintln!("jpl_sbdb: no elliptical orbit records — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match std::fs::write(&out_path, &buf) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("write {out_path}: {e}");
            std::process::exit(1);
        }
    }
    let (chunks, _) = buf.as_chunks::<{ RECORD_STRIDE }>();
    let parsed = chunks.iter().filter(|c| parse_record(*c).is_some()).count();
    eprintln!(
        "jpl_sbdb: {parsed} of {records} records parse back, {} B -> {out_path}",
        buf.len()
    );
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("jpl_sbdb: upload {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_name_reads_numbered_and_unnumbered() {
        let (n, d) = split_name("     1 Ceres (A801 AA)").unwrap();
        assert_eq!(n, Some(1));
        assert_eq!(d, "A801 AA");
        let (n, d) = split_name(" 15874 (1996 TL66)").unwrap();
        assert_eq!(n, Some(15874));
        assert_eq!(d, "1996 TL66");
        let (n, d) = split_name("       (2025 OD728)").unwrap();
        assert_eq!(n, None);
        assert_eq!(d, "2025 OD728");
    }

    #[test]
    fn sbdb_row_maps_fields_to_elements() {
        let fields = [
            "full_name",
            "a",
            "e",
            "i",
            "om",
            "w",
            "ma",
            "epoch",
            "H",
            "G",
        ];
        let idx: HashMap<String, usize> = fields
            .iter()
            .enumerate()
            .map(|(k, s)| (s.to_string(), k))
            .collect();
        let row: Vec<JsonVal> = [
            "     1 Ceres (A801 AA)",
            "2.765552595034094",
            ".07969229514816586",
            "10.58802780183462",
            "80.24862682043221",
            "73.29421453021587",
            "274.4193463761342",
            "2461200.5",
            "3.34",
            "0.12",
        ]
        .iter()
        .map(|s| JsonVal::Str(s.to_string()))
        .collect();
        let rec = sbdb_rec(&row, &idx).unwrap();
        assert_eq!(rec.number, 1);
        assert!((rec.a_au - 2.765552595034094).abs() < 1e-12);
        assert!((rec.h_mag - 3.34).abs() < 1e-6);
        assert_eq!(
            rec.flags,
            FLAG_H_PRESENT | FLAG_G_PRESENT | FLAG_NUMBER_PRESENT
        );
    }

    #[test]
    fn hyperbolic_orbit_is_absent() {
        let fields = [
            "full_name",
            "a",
            "e",
            "i",
            "om",
            "w",
            "ma",
            "epoch",
            "H",
            "G",
        ];
        let idx: HashMap<String, usize> = fields
            .iter()
            .enumerate()
            .map(|(k, s)| (s.to_string(), k))
            .collect();
        let row: Vec<JsonVal> = [
            "       (2019 Q4)",
            "-1.2",
            "3.35",
            "44.0",
            "1.0",
            "1.0",
            "1.0",
            "2461200.5",
            "12.0",
            "0.15",
        ]
        .iter()
        .map(|s| JsonVal::Str(s.to_string()))
        .collect();
        assert!(sbdb_rec(&row, &idx).is_none());
    }
}
