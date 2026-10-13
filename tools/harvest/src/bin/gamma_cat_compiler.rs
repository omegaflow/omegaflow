use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsColumn, FitsHeader, FitsTable};
use omegaflow::inflate::gunzip;
use omegaflow_harvest::table::{Table, arg_or, emit, has_flag, load_source, stat};

const MAGIC: [u8; 4] = *b"GCAT";
const NETLOC: &str = "github.com";
const FORMAT: &str = "gamma_cat";
const COMPILER: &str = "tools/harvest/src/bin/gamma_cat_compiler.rs";
const DEFAULT_URL: &str = "https://github.com/gammapy/gamma-cat/raw/master/output/gammacat.fits.gz";
const DEFAULT_OUT: &str = "gamma_cat.bin";

const FIELDS: &[(&str, &[&str])] = &[
    ("ra", &["ra"]),
    ("dec", &["dec"]),
    ("glon", &["glon"]),
    ("glat", &["glat"]),
    ("significance", &["significance"]),
    ("livetime", &["livetime"]),
    ("spec_pl_norm", &["spec_pl_norm"]),
    ("spec_pl_index", &["spec_pl_index"]),
    ("spec_pl_e_ref", &["spec_pl_e_ref"]),
    ("spec_dnde_1tev", &["spec_dnde_1TeV"]),
    ("spec_flux_1tev", &["spec_flux_1TeV"]),
    ("spec_eflux_1tev_10tev", &["spec_eflux_1TeV_10TeV"]),
    ("morph_sigma", &["morph_sigma"]),
];

fn col<'a>(table: &'a FitsTable, names: &[&str]) -> Option<&'a FitsColumn> {
    names.iter().find_map(|n| table.column(n))
}

fn parse_fits(mut bytes: Vec<u8>) -> Option<(Vec<u8>, FitsTable)> {
    if bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b {
        bytes = gunzip(&bytes)?;
    }
    let (_, off) = FitsHeader::parse(&bytes, 0)?;
    let (table, _) = FitsTable::parse(&bytes, off)?;
    Some((bytes, table))
}

fn schema(table: &FitsTable) {
    for c in &table.columns {
        eprintln!(
            "  {:<18} code {} repeat {} width {}",
            c.name, c.code, c.repeat, c.width
        );
    }
}

fn gather(bytes: &[u8], table: &FitsTable) -> Option<Table> {
    let resolved: Vec<(&str, &FitsColumn)> = FIELDS
        .iter()
        .filter_map(|(key, names)| col(table, names).map(|c| (*key, c)))
        .collect();
    if resolved.is_empty() {
        eprintln!("gamma_cat: no bound column resolved — absent (0 honored)");
        return None;
    }
    let names: Vec<String> = resolved.iter().map(|(k, _)| (*k).to_string()).collect();
    let mut rows = Vec::with_capacity(table.n_rows);
    for row in 0..table.n_rows {
        let mut cells = Vec::with_capacity(resolved.len());
        for (_, c) in &resolved {
            cells.push(table.cell_f64(bytes, row, c).filter(|v| v.is_finite()));
        }
        rows.push(cells);
    }
    Some(Table { names, rows })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((src, raw)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Some((bytes, table)) = parse_fits(raw) else {
        eprintln!("{src}: the body carries no FITS BINTABLE — absent (0 honored)");
        std::process::exit(1);
    };
    if has_flag(&args, "--schema") {
        eprintln!(
            "{src}: {} rows, {} columns",
            table.n_rows,
            table.columns.len()
        );
        schema(&table);
        return;
    }
    let Some(out_table) = gather(&bytes, &table) else {
        std::process::exit(1);
    };
    if has_flag(&args, "--stat") {
        stat(&src, &out_table);
        return;
    }
    let out = arg_or(&args, "--out", DEFAULT_OUT);
    let netloc = arg_or(&args, "--netloc", NETLOC);
    if !emit(&src, &out, &out_table, MAGIC, &netloc, FORMAT, COMPILER) {
        std::process::exit(1);
    }
    if has_flag(&args, "--ci-mode") && !upload_release(&netloc, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(18)].copy_from_slice(&v[..v.len().min(18)]);
        card
    }

    fn fits_fixture() -> Vec<u8> {
        let mut buf = Vec::new();
        let mut hdr: Vec<u8> = Vec::new();
        for (k, v) in [
            ("SIMPLE", "T"),
            ("BITPIX", "8"),
            ("NAXIS", "0"),
            ("END", ""),
        ] {
            hdr.extend_from_slice(&pad_card(k, v));
        }
        while hdr.len() % 2880 != 0 {
            hdr.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&hdr);
        let mut ext: Vec<u8> = Vec::new();
        for (k, v) in [
            ("XTENSION", "'BINTABLE'"),
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "24"),
            ("NAXIS2", "1"),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "3"),
            ("TTYPE1", "'ra'"),
            ("TFORM1", "D"),
            ("TTYPE2", "'dec'"),
            ("TFORM2", "D"),
            ("TTYPE3", "'significance'"),
            ("TFORM3", "D"),
            ("END", ""),
        ] {
            ext.extend_from_slice(&pad_card(k, v));
        }
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);
        buf.extend_from_slice(&83.6f64.to_be_bytes());
        buf.extend_from_slice(&22.0f64.to_be_bytes());
        buf.extend_from_slice(&12.5f64.to_be_bytes());
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn gather_binds_the_present_columns_and_keeps_absent_keys_out() {
        let buf = fits_fixture();
        let (bytes, table) = parse_fits(buf).expect("parse");
        let t = gather(&bytes, &table).expect("gather");
        assert_eq!(t.names, vec!["ra", "dec", "significance"]);
        assert_eq!(t.rows[0][0], Some(83.6));
        assert_eq!(t.rows[0][2], Some(12.5));
    }

    #[test]
    fn a_non_finite_cell_stays_absent() {
        let mut buf = fits_fixture();
        let at = 2 * 2880 + 16;
        buf[at..at + 8].copy_from_slice(&f64::NAN.to_be_bytes());
        let (bytes, table) = parse_fits(buf).expect("parse");
        let t = gather(&bytes, &table).expect("gather");
        assert_eq!(t.rows[0][2], None);
        assert_eq!(t.rows[0][0], Some(83.6));
    }

    #[test]
    fn gzip_magic_is_decompressed_before_the_fits_parse() {
        assert!(parse_fits(vec![0x1f, 0x8b, 0x08, 0x00]).is_none());
    }

    #[test]
    fn a_non_fits_body_is_refused() {
        assert!(parse_fits(b"not a fits".to_vec()).is_none());
    }
}
