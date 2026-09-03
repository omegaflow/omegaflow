use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsColumn, FitsHeader, FitsTable};

const URL: &str = "https://fermi.gsfc.nasa.gov/ssc/data/access/lat/14yr_catalog/gll_psc_v32.fit";

fn write_bin(records: &[[f64; 4]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 32);
    out.extend_from_slice(b"F4GL");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn read_bin(data: &[u8]) -> Option<Vec<[f64; 4]>> {
    if data.len() < 8 || &data[0..4] != b"F4GL" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 32 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 32;
        let mut r = [0.0f64; 4];
        for k in 0..4 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let Some(bytes) = fetch_raw_bytes(URL, 604800) else {
        eprintln!("gll_psc_v32.fit: fetch void ({URL})");
        return;
    };
    let (primary, data_start) = match FitsHeader::parse(&bytes, 0) {
        Some(h) => h,
        None => {
            eprintln!("gll_psc_v32.fit: primary header parse void");
            return;
        }
    };
    let _ = primary;
    let (table, _next) = match FitsTable::parse(&bytes, data_start) {
        Some(t) => t,
        None => {
            eprintln!("gll_psc_v32.fit: BINTABLE parse void");
            return;
        }
    };
    let cols: Vec<(&str, &FitsColumn)> = ["GLON", "GLAT", "Flux1000", "Energy_Flux100"]
        .iter()
        .filter_map(|n| table.column(n).map(|c| (*n, c)))
        .collect();
    if cols.len() != 4 {
        let have: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();
        eprintln!(
            "gll_psc_v32.fit: {} of 4 columns found — columns carry {:?}",
            cols.len(),
            have
        );
        return;
    }
    let mut records: Vec<[f64; 4]> = Vec::with_capacity(table.n_rows);
    for row in 0..table.n_rows {
        let mut r = [0.0f64; 4];
        let mut ok = true;
        for (k, (_, c)) in cols.iter().enumerate() {
            match table.cell_f64(&bytes, row, c) {
                Some(v) => r[k] = v,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && r[2] > 0.0 && r[3] > 0.0 {
            records.push(r);
        }
    }
    if records.is_empty() {
        eprintln!("gll_psc_v32.fit: no valid sources — the series stays unwritten (0 honored)");
        return;
    }
    let out = "data/fermi_4fgl_dr4.bin";
    std::fs::create_dir_all("data").ok();
    let bin = write_bin(&records);
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} void");
        return;
    }
    match read_bin(&bin) {
        Some(parsed) => {
            let mut fmax = 0.0f64;
            for r in &parsed {
                if r[2] > fmax {
                    fmax = r[2];
                }
            }
            eprintln!(
                "{out}: {} sources ({} rows im Katalog), flux1000 bis {fmax:.3e} ph/cm²/s, {} B — roundtrip parses",
                parsed.len(),
                table.n_rows,
                bin.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void — the series stays unverified");
            return;
        }
    }
    if ci_mode && !upload_release("fermi.gsfc.nasa.gov", &out) {
        std::process::exit(1);
    }
}
