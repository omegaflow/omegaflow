use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsColumn, FitsHeader, FitsTable};
use omegaflow_harvest::table::{Table, arg_or, emit, has_flag, load_source, stat};

const MAGIC: [u8; 4] = *b"4LAC";
const NETLOC: &str = "fermi.gsfc.nasa.gov";
const FORMAT: &str = "fermi_4lac";
const COMPILER: &str = "tools/harvest/src/bin/fermi_4lac_compiler.rs";
const DEFAULT_URL: &str = "https://fermi.gsfc.nasa.gov/ssc/data/access/lat/4LACDR3/table_4LAC.fits";
const DEFAULT_OUT: &str = "fermi_4lac.bin";

const FIELDS: &[(&str, &[&str])] = &[
    ("ra", &["RAJ2000"]),
    ("dec", &["DEJ2000"]),
    ("glon", &["GLON"]),
    ("glat", &["GLAT"]),
    ("signif_avg", &["Signif_Avg"]),
    ("flux1000", &["Flux1000"]),
    ("unc_flux1000", &["Unc_Flux1000"]),
    ("energy_flux100", &["Energy_Flux100"]),
    ("unc_energy_flux100", &["Unc_Energy_Flux100"]),
    ("pl_index", &["PL_Index"]),
    ("unc_pl_index", &["Unc_PL_Index"]),
    ("pivot_energy", &["Pivot_Energy"]),
    ("lp_index", &["LP_Index"]),
    ("lp_beta", &["LP_beta"]),
    ("variability_index", &["Variability_Index"]),
    ("redshift", &["Redshift"]),
    ("nu_syn", &["nu_syn"]),
    ("nufnu_syn", &["nuFnu_syn"]),
    ("highest_energy", &["Highest_energy"]),
];

fn col<'a>(table: &'a FitsTable, names: &[&str]) -> Option<&'a FitsColumn> {
    names.iter().find_map(|n| table.column(n))
}

fn parse_fits(bytes: &[u8]) -> Option<FitsTable> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (table, _) = FitsTable::parse(bytes, off)?;
    Some(table)
}

fn schema(table: &FitsTable) {
    for c in &table.columns {
        eprintln!(
            "  {:<16} code {} repeat {} width {} tbcol {}",
            c.name, c.code, c.repeat, c.width, c.tbcol
        );
    }
}

fn gather(bytes: &[u8]) -> Option<Table> {
    let table = parse_fits(bytes)?;
    let resolved: Vec<(&str, &FitsColumn)> = FIELDS
        .iter()
        .filter_map(|(key, names)| col(&table, names).map(|c| (*key, c)))
        .collect();
    if resolved.is_empty() {
        eprintln!("fermi_4lac: no bound column resolved — absent (0 honored)");
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
    let Some((src, bytes)) = load_source(&args, DEFAULT_URL) else {
        std::process::exit(1);
    };
    let Some(table) = parse_fits(&bytes) else {
        eprintln!(
            "{src}: {} B carry no FITS BINTABLE — absent (0 honored)",
            bytes.len()
        );
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
    let Some(out_table) = gather(&bytes) else {
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

    fn fits_fixture(rows: &[(f64, f64, f64, f64)]) -> Vec<u8> {
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
        let nrows = rows.len().to_string();
        let mut ext: Vec<u8> = Vec::new();
        for (k, v) in [
            ("XTENSION", "'BINTABLE'"),
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "32"),
            ("NAXIS2", nrows.as_str()),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "4"),
            ("TTYPE1", "'RAJ2000'"),
            ("TFORM1", "D"),
            ("TTYPE2", "'DEJ2000'"),
            ("TFORM2", "D"),
            ("TTYPE3", "'Flux1000'"),
            ("TFORM3", "D"),
            ("TTYPE4", "'PL_Index'"),
            ("TFORM4", "D"),
            ("END", ""),
        ] {
            ext.extend_from_slice(&pad_card(k, v));
        }
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);
        for (ra, dec, flux, pl) in rows {
            buf.extend_from_slice(&ra.to_be_bytes());
            buf.extend_from_slice(&dec.to_be_bytes());
            buf.extend_from_slice(&flux.to_be_bytes());
            buf.extend_from_slice(&pl.to_be_bytes());
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn gather_binds_the_present_columns_and_keeps_absent_keys_out() {
        let buf = fits_fixture(&[(10.0, -5.0, 2.5e-8, 2.1), (20.0, 1.0, 3.5e-8, 1.9)]);
        let table = gather(&buf).expect("gather");
        assert_eq!(table.names, vec!["ra", "dec", "flux1000", "pl_index"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0][0], Some(10.0));
        assert_eq!(table.rows[0][2], Some(2.5e-8));
        assert_eq!(table.rows[1][1], Some(1.0));
    }

    #[test]
    fn a_non_finite_cell_stays_absent() {
        let buf = fits_fixture(&[(f64::NAN, -5.0, 2.5e-8, 2.1)]);
        let table = gather(&buf).expect("gather");
        assert_eq!(table.rows[0][0], None);
        assert_eq!(table.rows[0][1], Some(-5.0));
        assert_eq!(table.rows[0][2], Some(2.5e-8));
    }

    #[test]
    fn a_fits_without_any_bound_column_is_absent() {
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
            ("NAXIS1", "8"),
            ("NAXIS2", "1"),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "1"),
            ("TTYPE1", "'OTHER'"),
            ("TFORM1", "D"),
            ("END", ""),
        ] {
            ext.extend_from_slice(&pad_card(k, v));
        }
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);
        buf.extend_from_slice(&1.0f64.to_be_bytes());
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        assert!(gather(&buf).is_none());
    }

    #[test]
    fn a_non_fits_body_is_refused() {
        assert!(parse_fits(b"not a fits").is_none());
    }
}
