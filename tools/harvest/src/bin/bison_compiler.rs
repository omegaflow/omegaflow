use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::bison_velocity::{parse_bin, write_bin};
use omegaflow::cdn::upload_asset;
use omegaflow::fits::FitsHeader;
use omegaflow::inflate::gunzip;

const FILL_URL: &str =
    "https://bison.ph.bham.ac.uk/downloads/data/allsites-alldata-waverage-fill.fits.gz";
const JD_UNIX_OFFSET: f64 = 2440587.5;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "bison_pmode.bin".to_string());

    let bytes = match arg_value(&args, "--fill") {
        Some(p) => std::fs::read(&p).ok(),
        None => fetch_raw_bytes(FILL_URL, 3600),
    };
    let Some(bytes) = bytes else {
        eprintln!("BiSON-FITS void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(fits) = gunzip(&bytes) else {
        eprintln!("gunzip void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some((header, data_start)) = FitsHeader::parse(&fits, 0) else {
        eprintln!("FITS header void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let Some(naxis2) = header.int("NAXIS2") else {
        eprintln!("NAXIS2 absent — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let naxis2 = naxis2 as usize;
    let mut records: Vec<(f64, f64)> = Vec::with_capacity(naxis2 / 2);
    let mut gap = 0usize;
    for i in 0..naxis2 {
        let off = data_start + i * 16;
        let Some(tb) = fits.get(off..off + 8) else {
            break;
        };
        let Some(vb) = fits.get(off + 8..off + 16) else {
            break;
        };
        let jd = f64::from_be_bytes(tb.try_into().ok().unwrap());
        let v = f64::from_be_bytes(vb.try_into().ok().unwrap());
        if v == 0.0 {
            gap += 1;
            continue;
        }
        records.push(((jd - JD_UNIX_OFFSET) * 86400.0, v));
    }
    eprintln!(
        "BiSON p modes: {} records, {} gaps (0.0 fill) skipped",
        records.len(),
        gap
    );
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} records, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
