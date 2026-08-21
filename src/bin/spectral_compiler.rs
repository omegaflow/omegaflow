// CSV contract (queue draft master.φ:31611): rows
// `wavelength_nm,irradiance_W_m2_nm,uncertainty_W_m2_nm,quality_flag`;
// flag != 0, non-finite and non-positive values drop (0 honored).
// Conversion: ν = c/λ, E_ν = E_λ·λ²/c, bin_width from the native λ grid.
// Epoch = month middle of the measurement (not fetch time), TDB via LSK.
//
// Since the NCEI-SSI harvest: `--input-nc <file.nc>` reads the netCDF-4
// (= HDF5) monthly file via src/hdf5.rs — wavelength (nm), SSI (W/m²/nm),
// time (days since 1610-01-01). `--month YYYY-MM` selects the row whose
// month the time axis carries (named refusal when the file has no such
// month). The measurement carries no quality_flag variable: flag 0, the
// -99 missing_value rows drop through the positivity gate (0 honored).

use omegaflow::cdn::upload_asset;
use omegaflow::hdf5::{decode_f32, Endian, Hdf5File};
use omegaflow::lsk::parse as parse_lsk;
use omegaflow::nc4::time_row_month;
use omegaflow::spectral::{
    bins_from_lambda_rows, month_middle_unix, parse_spectral_bin, write_spectral_bin,
};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "spectra.bin".to_string());
    let (year, month) = match arg_value(&args, "--month").as_deref().and_then(|m| {
        let (y, mo) = m.split_once('-')?;
        Some((y.parse::<u32>().ok()?, mo.parse::<u32>().ok()?))
    }) {
        Some(v) => v,
        None => {
            eprintln!("--month absent — the measurement month is undeclared (YYYY-MM)");
            std::process::exit(1);
        }
    };
    let mut rows: Vec<(f64, f64, u8)> = Vec::new();
    if let Some(nc_path) = arg_value(&args, "--input-nc") {
        let bytes = match std::fs::read(&nc_path) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("read {} returned void", nc_path);
                std::process::exit(1);
            }
        };
        let file = match Hdf5File::parse(&bytes) {
            Ok(f) => f,
            Err(note) => {
                eprintln!("{} parses void: {:?}", nc_path, note);
                std::process::exit(1);
            }
        };
        let wl_raw = match file.read_dataset("wavelength") {
            Ok(d) => d,
            Err(note) => {
                eprintln!("wavelength stays unread: {:?}", note);
                std::process::exit(1);
            }
        };
        let ssi_raw = match file.read_dataset("SSI") {
            Ok(d) => d,
            Err(note) => {
                eprintln!("SSI stays unread: {:?}", note);
                std::process::exit(1);
            }
        };
        let time_raw = match file.read_dataset("time") {
            Ok(d) => d,
            Err(note) => {
                eprintln!("time stays unread: {:?}", note);
                std::process::exit(1);
            }
        };
        let n_wl = wl_raw.len() / 4;
        let n_months = time_raw.len() / 4;
        let mut month_row: Option<usize> = None;
        for i in 0..n_months {
            let days = decode_f32(&time_raw, i * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
            if time_row_month(days, (1610, 1, 1)) == Some((year, month)) {
                month_row = Some(i);
                break;
            }
        }
        let row = match month_row {
            Some(r) => r,
            None => {
                eprintln!(
                    "{} carries no {}-{:02} row on its time axis — the month stays unread",
                    nc_path, year, month
                );
                std::process::exit(1);
            }
        };
        for w in 0..n_wl {
            let lam = decode_f32(&wl_raw, w * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
            let e =
                decode_f32(&ssi_raw, (row * n_wl + w) * 4, Endian::Le).unwrap_or(f32::NAN) as f64;
            rows.push((lam, e, 0));
        }
        eprintln!(
            "{}: {} rows read, month row {} of {}",
            nc_path,
            rows.len(),
            row,
            n_months
        );
    } else {
        let input = match arg_value(&args, "--input") {
            Some(p) => p,
            None => {
                eprintln!("--input absent — the measurement table is undeclared");
                std::process::exit(1);
            }
        };
        let text = match std::fs::read_to_string(&input) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("read {} returned void", input);
                std::process::exit(1);
            }
        };
        let mut malformed = 0usize;
        for line in text.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 4 {
                malformed += 1;
                continue;
            }
            let lam: f64 = match parts[0].trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    malformed += 1;
                    continue;
                }
            };
            let e_lam: f64 = match parts[1].trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    malformed += 1;
                    continue;
                }
            };
            let flag: u8 = match parts[3].trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    malformed += 1;
                    continue;
                }
            };
            rows.push((lam, e_lam, flag));
        }
        if malformed > 0 {
            eprintln!("{}: {} malformed rows", input, malformed);
        }
    }
    let bins = bins_from_lambda_rows(&rows);
    eprintln!("{} rows, {} valid bins", rows.len(), bins.len());
    if bins.is_empty() {
        eprintln!("no valid bins — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let unix = match month_middle_unix(year, month) {
        Some(u) => u,
        None => {
            eprintln!(
                "--month {}-{:02} carries no epoch — the month middle stays void",
                year, month
            );
            std::process::exit(1);
        }
    };
    let lsk_text = match arg_value(&args, "--lsk").and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => t,
        None => {
            eprintln!("--lsk absent — the TDB conversion stays void (no fabricated epoch)");
            std::process::exit(1);
        }
    };
    let lsk = match parse_lsk(&lsk_text) {
        Some(l) => l,
        None => {
            eprintln!("--lsk parses void — the leap-second table stays unread");
            std::process::exit(1);
        }
    };
    let epoch_tdb = match lsk.unix_to_tdb(unix) {
        Some(t) => t,
        None => {
            eprintln!("unix_to_tdb returned void for {}-{:02}", year, month);
            std::process::exit(1);
        }
    };
    let bytes = write_spectral_bin(epoch_tdb, &bins);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_spectral_bin(&bytes) {
        Some((epoch, parsed)) => {
            eprintln!(
                "{}: {} Bins, epoch_tdb {} — roundtrip parses ({} B)",
                out,
                parsed.len(),
                epoch,
                bytes.len()
            );
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
