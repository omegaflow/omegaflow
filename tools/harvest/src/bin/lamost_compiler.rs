use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::process::Command;

const MAGIC: &[u8; 4] = b"LMST";
const VERSION: u32 = 1;
const NETLOC: &str = "www.lamost.org";
const FORMAT: &str = "lamost_dr10";
const COMPILER: &str = "tools/harvest/src/bin/lamost_compiler.rs";
const TAP_ROOT: &str = "https://www.lamost.org/dr10/v2.0/voservice/tap/sync";
const TABLE: &str = "xmatch_obs.sdssdr14";
const TTL_S: u64 = 31_536_000;
const DEFAULT_LIMIT: usize = 100_000;
const DEFAULT_OUT: &str = "lamost_dr10.bin";
const ABSENT_SENTINEL: f64 = -999.0;

const SOURCE_COLUMNS: [&str; 6] = [
    "lamost_ra",
    "lamost_dec",
    "cz",
    "fibermag_u",
    "fibermag_g",
    "fibermag_r",
];
const OUTPUT_COLUMNS: [&str; 6] = ["ra", "dec", "z", "fibermag_u", "fibermag_g", "fibermag_r"];
const MANDATORY_COLUMNS: [&str; 2] = ["ra", "dec"];
const FIELD_SPECS: [(&str, &str); 3] = [
    ("fibermag_u", "lamost_u_fiber_mag"),
    ("fibermag_g", "lamost_g_fiber_mag"),
    ("fibermag_r", "lamost_r_fiber_mag"),
];

#[derive(Clone, PartialEq, Debug)]
struct Table {
    names: Vec<String>,
    rows: Vec<Vec<Option<f64>>>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn build_adql(limit: usize) -> String {
    format!(
        "SELECT TOP {} {} FROM {} WHERE fibermag_g IS NOT NULL",
        limit,
        SOURCE_COLUMNS.join(", "),
        TABLE
    )
}

fn tap_query(root: &str, adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("--fail-with-body")
        .arg("-m")
        .arg("300")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=votable")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg(root)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "tap query http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn votable_rows(body: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    let mut fields: Vec<String> = Vec::new();
    for f in body.split("<FIELD").skip(1) {
        let seg = match f.split_once('>') {
            Some((s, _)) => s,
            None => continue,
        };
        match seg.split("name=\"").nth(1) {
            Some(attr) => match attr.split_once('"') {
                Some((name, _)) => fields.push(name.trim().to_string()),
                None => continue,
            },
            None => continue,
        }
    }
    let data = match body.split("<DATA>").nth(1) {
        Some(d) => d,
        None => {
            eprintln!("votable: <DATA> absent — {} field(s) named", fields.len());
            return None;
        }
    };
    let mut rows = Vec::new();
    for tr in data.split("<TR>").skip(1) {
        let end = match tr.split_once("</TR>") {
            Some((e, _)) => e,
            None => continue,
        };
        let mut cells = Vec::new();
        for td in end.split("<TD>").skip(1) {
            let raw = match td.split_once("</TD>") {
                Some((r, _)) => r.trim(),
                None => continue,
            };
            let v = if let Some(c) = raw.strip_prefix("<![CDATA[") {
                match c.strip_suffix("]]>") {
                    Some(inner) => inner.trim().to_string(),
                    None => c.trim().to_string(),
                }
            } else {
                raw.to_string()
            };
            cells.push(v);
        }
        rows.push(cells);
    }
    Some((fields, rows))
}

fn cell_opt(raw: &str) -> Option<f64> {
    let t = raw.trim();
    if t.is_empty() {
        return None;
    }
    match t.parse::<f64>() {
        Ok(v) if v.is_finite() && v > ABSENT_SENTINEL => Some(v),
        _ => None,
    }
}

fn cell_at(row: &[String], i: usize) -> &str {
    match row.get(i) {
        Some(s) => s.as_str(),
        None => "",
    }
}

fn build_table(fields: &[String], rows: &[Vec<String>]) -> Option<Table> {
    let mut idx: Vec<(String, usize)> = Vec::new();
    for (src, out) in SOURCE_COLUMNS.iter().zip(OUTPUT_COLUMNS.iter()) {
        match fields.iter().position(|f| f == src) {
            Some(i) => idx.push(((*out).to_string(), i)),
            None => {
                eprintln!("votable lacks column {src}");
                return None;
            }
        }
    }
    let mandatory: Vec<usize> = MANDATORY_COLUMNS
        .iter()
        .filter_map(|m| OUTPUT_COLUMNS.iter().position(|o| o == m))
        .collect();
    let names: Vec<String> = OUTPUT_COLUMNS.iter().map(|s| (*s).to_string()).collect();
    let mut table = Table {
        names,
        rows: Vec::new(),
    };
    for row in rows {
        let mut cells: Vec<Option<f64>> = Vec::with_capacity(idx.len());
        for (_, i) in &idx {
            cells.push(cell_opt(cell_at(row, *i)));
        }
        let mut skip = false;
        for j in &mandatory {
            match cells.get(*j) {
                Some(Some(_)) => {}
                _ => skip = true,
            }
        }
        if skip {
            continue;
        }
        table.rows.push(cells);
    }
    Some(table)
}

fn write_bin(table: &Table) -> Vec<u8> {
    let n_cols = table.names.len();
    let words = (n_cols + 63) / 64;
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(n_cols as u32).to_le_bytes());
    out.extend_from_slice(&(table.rows.len() as u32).to_le_bytes());
    for name in &table.names {
        let b = name.as_bytes();
        out.extend_from_slice(&(b.len() as u16).to_le_bytes());
        out.extend_from_slice(b);
    }
    for row in &table.rows {
        let mut mask = vec![0u64; words];
        for (j, cell) in row.iter().enumerate() {
            if cell.is_some() {
                mask[j / 64] |= 1u64 << (j % 64);
            }
        }
        for w in &mask {
            out.extend_from_slice(&w.to_le_bytes());
        }
        for cell in row {
            if let Some(v) = cell {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Table> {
    if bytes.len() < 16 || &bytes[0..4] != MAGIC {
        return None;
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    if version != VERSION {
        return None;
    }
    let n_cols = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let n_rows = u32::from_le_bytes(bytes[12..16].try_into().ok()?) as usize;
    if n_cols == 0 || n_cols > bytes.len() / 2 {
        return None;
    }
    let mut pos = 16usize;
    let mut names = Vec::with_capacity(n_cols);
    for _ in 0..n_cols {
        let len = u16::from_le_bytes(bytes.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let raw = bytes.get(pos..pos + len)?;
        names.push(String::from_utf8(raw.to_vec()).ok()?);
        pos += len;
    }
    let words = (n_cols + 63) / 64;
    let min_row = words.checked_mul(8)?;
    if bytes.len().checked_sub(pos)? < n_rows.checked_mul(min_row)? {
        return None;
    }
    let mut rows = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
        let mut mask = vec![0u64; words];
        for w in mask.iter_mut() {
            *w = u64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
        }
        let mut row = vec![None; n_cols];
        for (j, slot) in row.iter_mut().enumerate() {
            if mask[j / 64] & (1u64 << (j % 64)) != 0 {
                let v = f64::from_le_bytes(bytes.get(pos..pos + 8)?.try_into().ok()?);
                pos += 8;
                *slot = Some(v);
            }
        }
        rows.push(row);
    }
    if pos != bytes.len() {
        return None;
    }
    Some(Table { names, rows })
}

fn emit(src: &str, out: &str, table: &Table) {
    if table.rows.is_empty() {
        eprintln!("{out}: 0 rows — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes_out = write_bin(table);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == *table => {
            eprintln!(
                "{out}: {} row(s), {} column(s), {} B — roundtrip parses",
                table.rows.len(),
                table.names.len(),
                bytes_out.len()
            );
        }
        Some(_) => {
            eprintln!("{out}: roundtrip parses a different set — the bin stays unwritten");
            std::process::exit(1);
        }
        None => {
            eprintln!("{out}: roundtrip parse returned void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {src}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("at sun");
    println!("ttl {TTL_S}");
    for (key, name) in FIELD_SPECS {
        println!("field {key} {name} inverse-square em mag {TTL_S} 0.0 0.0");
    }
    println!("sha256 {}", sha256_hex(&bytes_out));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "lamost_compiler --out <file.bin> [--limit N] [--url <endpoint>] [--stat] [--ci-mode]"
        );
        eprintln!("  reads the LAMOST DR10 SDSS-DR14 crossmatch catalogue over TAP");
        eprintln!(
            "  writes the self-describing LMST bin (absent cell = mask bit clear, never 0.0)"
        );
        std::process::exit(2);
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let stat_mode = args.iter().any(|a| a == "--stat");
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => DEFAULT_OUT.to_string(),
    };
    let limit = match arg_value(&args, "--limit") {
        Some(v) => match v.parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => {
                eprintln!("--limit {v} carries no positive count");
                std::process::exit(2);
            }
        },
        None => DEFAULT_LIMIT,
    };
    let root = match arg_value(&args, "--url") {
        Some(u) => u,
        None => TAP_ROOT.to_string(),
    };

    let adql = build_adql(limit);
    let Some(body) = tap_query(&root, &adql) else {
        eprintln!("{root}: the TAP reply stays unfetched");
        std::process::exit(1);
    };
    let Some((fields, rows)) = votable_rows(&body) else {
        eprintln!("{root}: the VOTABLE stays unread");
        std::process::exit(1);
    };
    if stat_mode {
        eprintln!("{root}: {} field(s), {} row(s)", fields.len(), rows.len());
        for f in &fields {
            eprintln!("  field {f}");
        }
        return;
    }
    let Some(table) = build_table(&fields, &rows) else {
        eprintln!("{root}: the catalogue columns stay unresolved");
        std::process::exit(1);
    };
    if rows.len() >= limit {
        eprintln!(
            "{root}: the reply carries {limit} rows — the selection is truncated, the bin stays unwritten"
        );
        std::process::exit(1);
    }
    eprintln!(
        "{root}: {} row(s) read, {} kept",
        rows.len(),
        table.rows.len()
    );
    emit(&root, &out, &table);
    if ci_mode && !upload_release(NETLOC, &out) {
        eprintln!("{out}: CDN upload returned void");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "\
<?xml version=\"1.0\"?>\n\
<VOTABLE><RESOURCE type=\"results\"><TABLE>\n\
<FIELD name=\"lamost_ra\" datatype=\"double\" />\n\
<FIELD name=\"lamost_dec\" datatype=\"double\" />\n\
<FIELD name=\"cz\" datatype=\"double\" />\n\
<FIELD name=\"fibermag_u\" datatype=\"double\" />\n\
<FIELD name=\"fibermag_g\" datatype=\"double\" />\n\
<FIELD name=\"fibermag_r\" datatype=\"double\" />\n\
<DATA><TABLEDATA>\n\
<TR><TD>38.268433</TD><TD>5.004872</TD><TD>-0.0017</TD><TD>19.216715</TD><TD>16.694424</TD><TD>15.539936</TD></TR>\n\
<TR><TD></TD><TD></TD><TD></TD><TD></TD><TD></TD><TD></TD></TR>\n\
<TR><TD>10.0</TD><TD>-2.0</TD><TD></TD><TD>12.0</TD><TD></TD><TD>11.0</TD></TR>\n\
</TABLEDATA></DATA></TABLE></RESOURCE></VOTABLE>\n";

    #[test]
    fn votable_names_and_cells_are_read() {
        let (fields, rows) = votable_rows(BODY).expect("votable");
        assert_eq!(fields.len(), 6);
        assert_eq!(fields[0], "lamost_ra");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0], "38.268433");
    }

    #[test]
    fn absent_cells_stay_none_never_zero() {
        let (fields, rows) = votable_rows(BODY).expect("votable");
        let table = build_table(&fields, &rows).expect("columns");
        assert_eq!(table.rows.len(), 2, "the all-empty row is refused");
        assert_eq!(table.rows[0][3], Some(19.216715));
        assert_eq!(table.rows[1][2], None);
        assert_eq!(table.rows[1][4], None);
    }

    #[test]
    fn sentinel_and_blank_resolve_to_none() {
        assert_eq!(cell_opt(""), None);
        assert_eq!(cell_opt("-999.0"), None);
        assert_eq!(cell_opt("15.5"), Some(15.5));
        assert_eq!(cell_opt("not-a-number"), None);
    }

    #[test]
    fn bin_roundtrip_preserves_absence() {
        let (fields, rows) = votable_rows(BODY).expect("votable");
        let table = build_table(&fields, &rows).expect("columns");
        let bytes = write_bin(&table);
        assert_eq!(&bytes[0..4], MAGIC);
        assert_eq!(parse_bin(&bytes), Some(table));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_truncation() {
        assert!(parse_bin(b"XXXX").is_none());
        let (fields, rows) = votable_rows(BODY).expect("votable");
        let table = build_table(&fields, &rows).expect("columns");
        let good = write_bin(&table);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }

    #[test]
    fn adql_names_the_crossmatch_table() {
        let q = build_adql(5);
        assert!(q.contains("xmatch_obs.sdssdr14"));
        assert!(q.contains("fibermag_g IS NOT NULL"));
    }
}
