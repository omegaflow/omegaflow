use omegaflow::cdn::upload_release;
use omegaflow_harvest::table::{arg_or, emit, has_flag, load_source, parse_delimited_table, stat};

const MAGIC: [u8; 4] = *b"ASCT";
const NETLOC: &str = "zenodo.org";
const FORMAT: &str = "ascii";
const COMPILER: &str = "tools/harvest/src/bin/ascii_table_compiler.rs";
const DEFAULT_URL: &str =
    "https://zenodo.org/api/records/16365279/files/Pantheon_SH0ES.dat/content";
const DEFAULT_OUT: &str = "ascii_table.bin";

fn selftest() {
    let text = "# fixture\nname z mu\nSN1 0.01 34.5\nSN2 0.02 nan\n";
    let Some(t) = parse_delimited_table(text) else {
        eprintln!("selftest: the fixture parses void");
        std::process::exit(1);
    };
    if t.names != vec!["name", "z", "mu"] {
        eprintln!("selftest: the header row does not name the columns");
        std::process::exit(1);
    }
    if t.rows.len() != 2 || t.rows[0][2] != Some(34.5) || t.rows[1][2].is_some() {
        eprintln!("selftest: a row does not carry the measured value (nan stays absent)");
        std::process::exit(1);
    }
    eprintln!("ascii_table_compiler: selftest passes (header + plausibility gate)");
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
    let Ok(text) = std::str::from_utf8(&bytes) else {
        eprintln!("{src}: the body is not utf8 — absent (0 honored)");
        std::process::exit(1);
    };
    let Some(table) = parse_delimited_table(text) else {
        eprintln!("{src}: no table rows survived — absent (0 honored)");
        std::process::exit(1);
    };
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
    fn whitespace_table_reads_header_and_absence() {
        let t = parse_delimited_table("ra dec\n1.0 2.0\n3.0 \n").unwrap();
        assert_eq!(t.names, vec!["ra", "dec"]);
        assert_eq!(t.rows.len(), 2);
        assert_eq!(t.rows[1][1], None);
    }
}
