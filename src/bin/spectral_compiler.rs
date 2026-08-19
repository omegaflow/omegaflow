// spectral_compiler — Spektral-Atom B: tabellarische Messung → spectra.bin.
// usage:
//   spectral_compiler --input <csv> --month YYYY-MM --lsk <naif0012.tls> [--out spectra.bin] [--ci-mode]
// CSV-Kontrakt (Queue-Draft master.φ:31611): Zeilen
// `wavelength_nm,irradiance_W_m2_nm,uncertainty_W_m2_nm,quality_flag`;
// flag != 0, nicht-endliche und nicht-positive Werte fallen (0 honored).
// Konversion: ν = c/λ, E_ν = E_λ·λ²/c, bin_width aus dem nativen λ-Gitter.
// Epoch = Monatsmitte der Messung (nicht Fetchzeit), TDB via LSK.
// Die netCDF-4/HDF5-Ernte bleibt pending — HDF5 wird benannt, nicht ersetzt.

use omegaflow::cdn::upload_asset;
use omegaflow::lsk::parse as parse_lsk;
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
    let mut rows: Vec<(f64, f64, u8)> = Vec::new();
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
    let bins = bins_from_lambda_rows(&rows);
    eprintln!(
        "{}: {} Zeilen, {} gültige Bins, {} malformed",
        input,
        rows.len(),
        bins.len(),
        malformed
    );
    if bins.is_empty() {
        eprintln!(
            "{}: no valid bins — the bin stays unwritten (0 honored)",
            input
        );
        std::process::exit(1);
    }
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
