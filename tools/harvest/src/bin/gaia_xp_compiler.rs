use omegaflow::cdn::upload_asset;
use omegaflow::spectral::{
    parse_xp_spectra_bin, write_xp_spectra_bin, xp_bins_from_flux_array, XpStar, XP_GRID_SAMPLES,
};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}

fn parse_flux_array(cell: &str) -> Option<Vec<f64>> {
    let inner = cell.trim().trim_start_matches('[').trim_end_matches(']');
    let mut out = Vec::new();
    for part in inner.split(',') {
        let v: f64 = part.trim().parse().ok()?;
        out.push(v);
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!("--out absent — the catalog path is undeclared");
            std::process::exit(1);
        }
    };
    let input = match arg_value(&args, "--input") {
        Some(p) => p,
        None => {
            eprintln!("--input absent — the TAP export is undeclared");
            std::process::exit(1);
        }
    };
    let epoch_tdb: f64 = match arg_value(&args, "--epoch-tdb").and_then(|v| v.parse::<f64>().ok()) {
        Some(e) if e.is_finite() => e,
        _ => {
            eprintln!(
                "--epoch-tdb absent — the catalog epoch is undeclared (TDB seconds since J2000)"
            );
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
    let lines: Vec<&str> = text.lines().collect();
    let header_idx = match lines.iter().position(|l| l.contains(',')) {
        Some(i) => i,
        None => {
            eprintln!("{}: header absent — the columns stay unnamed", input);
            std::process::exit(1);
        }
    };
    let header = split_csv_line(lines[header_idx]);
    let col = |name: &str| header.iter().position(|c| c.trim() == name);
    let (Some(ci_source), Some(ci_ra), Some(ci_dec), Some(ci_plx), Some(ci_flux)) = (
        col("source_id"),
        col("ra"),
        col("dec"),
        col("parallax"),
        col("flux"),
    ) else {
        eprintln!(
            "{}: header lacks a required column (source_id/ra/dec/parallax/flux) — the export is not the withpos view",
            input
        );
        std::process::exit(1);
    };
    let mut stars: Vec<XpStar> = Vec::new();
    let mut malformed = 0usize;
    let mut skipped_bins = 0usize;
    let mut skipped_plx = 0usize;
    for line in &lines[header_idx + 1..] {
        let parts = split_csv_line(line);
        let max = ci_source.max(ci_ra).max(ci_dec).max(ci_plx).max(ci_flux);
        if parts.len() <= max {
            malformed += 1;
            continue;
        }
        let source_id: u64 = match parts[ci_source].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                malformed += 1;
                continue;
            }
        };
        let ra: f64 = match parts[ci_ra].trim().parse::<f64>() {
            Ok(v) if v.is_finite() => v,
            _ => {
                malformed += 1;
                continue;
            }
        };
        let dec: f64 = match parts[ci_dec].trim().parse::<f64>() {
            Ok(v) if v.is_finite() => v,
            _ => {
                malformed += 1;
                continue;
            }
        };
        let plx_mas: f64 = match parts[ci_plx].trim().parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            Ok(_) => {
                skipped_plx += 1;
                continue;
            }
            _ => {
                malformed += 1;
                continue;
            }
        };
        let flux = match parse_flux_array(&parts[ci_flux]) {
            Some(f) if f.len() == XP_GRID_SAMPLES => f,
            _ => {
                malformed += 1;
                continue;
            }
        };
        let bins = xp_bins_from_flux_array(&flux);
        if bins.is_empty() {
            skipped_bins += 1;
            continue;
        }
        stars.push(XpStar {
            source_id,
            ra,
            dec,
            plx_mas,
            bins,
        });
    }
    eprintln!(
        "{}: {} stars, {} malformed rows, {} without parallax, {} without a valid bin",
        input,
        stars.len(),
        malformed,
        skipped_plx,
        skipped_bins
    );
    if stars.is_empty() {
        eprintln!("no valid stars — the catalog stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_xp_spectra_bin(epoch_tdb, &stars);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_xp_spectra_bin(&bytes) {
        Some((epoch, parsed)) => {
            eprintln!(
                "{}: {} stars, epoch_tdb {} — roundtrip parses ({} B)",
                out,
                parsed.len(),
                epoch,
                bytes.len()
            );
        }
        None => {
            eprintln!(
                "{}: roundtrip parse void — the catalog stays unverified",
                out
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        std::process::exit(1);
    }
}
