use omegaflow::archivar::fits::{FitsColumn, FitsTable};
use omegaflow::archivar::inflate::zip_members;
use omegaflow::cdn::upload_release;
use omegaflow_harvest::table::{
    Table, arg_or, arg_value, emit, has_flag, load_source, parse_delimited_table, sanitize_name,
    stat,
};

const MAGIC: [u8; 4] = *b"ZIPT";
const NETLOC: &str = "zenodo.org";
const FORMAT: &str = "zip";
const COMPILER: &str = "tools/harvest/src/bin/zip_member_compiler.rs";
const DEFAULT_URL: &str = "https://zenodo.org/api/records/3886372/files/Table2.txt.zip/content";
const DEFAULT_OUT: &str = "zip_member.bin";
const HDU_BYTES: usize = 2880;

fn collect_members(bytes: &[u8]) -> Option<Vec<(String, Vec<u8>)>> {
    let mut out = Vec::new();
    let seen = zip_members(bytes, |name, data| {
        out.push((name.to_string(), data.to_vec()))
    })?;
    if seen == 0 || out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn is_text(data: &[u8]) -> bool {
    std::str::from_utf8(data).is_ok() && data.iter().any(u8::is_ascii_digit)
}

fn select_member<'a>(
    members: &'a [(String, Vec<u8>)],
    want: Option<&str>,
) -> Option<&'a (String, Vec<u8>)> {
    if let Some(w) = want {
        members
            .iter()
            .find(|(name, _)| name == w || name.contains(w))
    } else {
        members
            .iter()
            .find(|(_, data)| is_text(data))
            .or_else(|| members.first())
    }
}

fn fits_numeric(column: &FitsColumn) -> bool {
    matches!(column.code, 'D' | 'E' | 'J' | 'I' | 'K' | 'B')
}

fn fits_table_to_table(buf: &[u8], fits: &FitsTable) -> Option<Table> {
    if fits.n_rows == 0 {
        return None;
    }
    let names = fits
        .columns
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if c.name.is_empty() {
                format!("col{i}")
            } else {
                sanitize_name(&c.name, i)
            }
        })
        .collect::<Vec<_>>();
    let mut rows = Vec::with_capacity(fits.n_rows);
    for i in 0..fits.n_rows {
        let row = fits
            .columns
            .iter()
            .map(|c| {
                if fits_numeric(c) {
                    fits.cell_f64(buf, i, c).filter(|v| v.is_finite())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        rows.push(row);
    }
    Some(Table { names, rows })
}

fn table_from_fits(buf: &[u8]) -> Option<Table> {
    let mut off = 0usize;
    while off < buf.len() {
        if let Some((fits, _next)) = FitsTable::parse(buf, off) {
            return fits_table_to_table(buf, &fits);
        }
        off += HDU_BYTES;
    }
    None
}

fn table_from_member(data: &[u8]) -> Option<(Table, &'static str)> {
    if data.starts_with(b"SIMPLE  =") {
        return table_from_fits(data).map(|t| (t, "fits"));
    }
    if let Ok(text) = std::str::from_utf8(data) {
        if let Some(t) = parse_delimited_table(text) {
            return Some((t, "text"));
        }
    }
    None
}

fn selftest() {
    let fixture: &[u8] = b"a b\n1.0 2.0\n";
    if !is_text(fixture) {
        eprintln!("selftest: a text member reads as non-text");
        std::process::exit(1);
    }
    match table_from_member(fixture) {
        Some((t, "text")) if t.rows.len() == 1 => {}
        _ => {
            eprintln!("selftest: the text member does not parse");
            std::process::exit(1);
        }
    }
    eprintln!("zip_member_compiler: selftest passes (text member arm)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--selftest") {
        selftest();
        return;
    }
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Some(members) = collect_members(&bytes) else {
        eprintln!("{src}: no zip member read — absent (0 honored)");
        std::process::exit(1);
    };
    let want = arg_value(&args, "--member");
    let Some((name, data)) = select_member(&members, want.as_deref()) else {
        eprintln!(
            "{src}: no member selected from {} member(s) — absent (0 honored)",
            members.len()
        );
        std::process::exit(1);
    };
    let Some((table, kind)) = table_from_member(data) else {
        eprintln!("{src}:{name}: the member carries no numeric table — absent (0 honored)");
        std::process::exit(1);
    };
    eprintln!("{src}: member {name} read as {kind}");
    if has_flag(&args, "--stat") {
        stat(&src, &table);
        return;
    }
    let out = arg_or(&args, "--out", DEFAULT_OUT);
    let netloc = arg_or(&args, "--netloc", NETLOC);
    if !emit(&src, &out, &table, MAGIC, &netloc, FORMAT, COMPILER) {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") && !upload_release(&netloc, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_member_detection_needs_a_digit() {
        assert!(is_text(b"ra dec\n1.0 2.0\n"));
        assert!(!is_text(b"only words here"));
    }

    #[test]
    fn text_member_parses_to_a_table() {
        let (t, kind) = table_from_member(b"a,b\n1,2\n3,4\n").unwrap();
        assert_eq!(kind, "text");
        assert_eq!(t.names, vec!["a", "b"]);
        assert_eq!(t.rows.len(), 2);
    }
}
