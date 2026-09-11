use omegaflow::cdn::{upload_asset, upload_release};
use omegaflow::spectral::{
    parse_xp_spectra_bin, write_xp_spectra_bin, xp_bins_from_flux_array, XpStar, XP_GRID_SAMPLES,
};

const SYNC_CAP_GUARD: usize = 20_000;
const TAP_SYNC: &str = "https://dc.g-vo.org/tap/sync";

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

fn fetch_source_range(lo: u64, hi: u64) -> Option<String> {
    let adql = format!(
        "SELECT s.source_id, s.ra, s.dec, w.parallax, s.flux, s.phot_bp_mean_mag, s.phot_rp_mean_mag FROM gdr3spec.withpos AS s JOIN gaia.dr3lite AS w ON s.source_id = w.source_id WHERE s.source_id >= {} AND s.source_id < {}",
        lo, hi
    );
    let out = std::process::Command::new("curl")
        .arg("-sSfL")
        .arg("-m")
        .arg("3600")
        .arg("--retry")
        .arg("3")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={}", adql))
        .arg(TAP_SYNC)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "source-range {}-{} http {}: {}",
            lo,
            hi,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn archive_verdict(stars: &[XpStar], colors: &[Option<f64>]) {
    if stars.is_empty() {
        return;
    }
    let med_of = |v: &mut Vec<f64>| -> f64 {
        if v.is_empty() {
            0.0
        } else {
            v.sort_by(|a, b| a.total_cmp(b));
            v[v.len() / 2]
        }
    };
    let mut plx: Vec<f64> = stars.iter().map(|s| s.plx_mas).collect();
    plx.sort_by(|a, b| a.total_cmp(b));
    let n_bins = stars.iter().map(|s| s.bins.len()).max().unwrap();
    let mut shapes: Vec<Vec<f64>> = Vec::with_capacity(stars.len());
    for s in stars {
        let mut vals: Vec<f64> = s
            .bins
            .iter()
            .map(|&(_, _, v)| v)
            .filter(|v| v.is_finite() && *v > 0.0)
            .collect();
        let scale = med_of(&mut vals);
        if scale <= 0.0 {
            shapes.push(Vec::new());
            continue;
        }
        shapes.push(
            s.bins
                .iter()
                .map(|&(_, _, v)| {
                    if v.is_finite() && v > 0.0 {
                        v / scale
                    } else {
                        0.0
                    }
                })
                .collect(),
        );
    }
    let mut chunk_shape = vec![0.0f64; n_bins];
    for i in 0..n_bins {
        let mut col: Vec<f64> = shapes
            .iter()
            .filter(|sh| sh.len() > i && sh[i] > 0.0)
            .map(|sh| sh[i])
            .collect();
        chunk_shape[i] = med_of(&mut col);
    }
    let mut residuals: Vec<f64> = Vec::new();
    for sh in &shapes {
        if sh.len() != n_bins {
            continue;
        }
        let mut acc = 0.0;
        let mut m = 0usize;
        for (i, &v) in sh.iter().enumerate() {
            let cs = chunk_shape[i];
            if cs > 0.0 && v > 0.0 {
                acc += ((v - cs) / cs).powi(2);
                m += 1;
            }
        }
        if m > 0 {
            residuals.push((acc / m as f64).sqrt());
        }
    }
    residuals.sort_by(|a, b| b.total_cmp(a));
    let n_outliers = residuals.iter().filter(|r| **r > 0.5).count();
    let mut ci: Vec<f64> = colors
        .iter()
        .filter_map(|c| *c)
        .filter(|c| c.is_finite())
        .collect();
    ci.sort_by(|a, b| a.total_cmp(b));
    let (c_med, n_blue, n_red) = if ci.is_empty() {
        (0.0, 0, 0)
    } else {
        let cm = ci[ci.len() / 2];
        let lo = ci.iter().filter(|c| **c < cm - 1.0).count();
        let hi = ci.iter().filter(|c| **c > cm + 1.0).count();
        (cm, lo, hi)
    };
    let top_residual = residuals.first();
    eprintln!(
        "archive verdict: {} spectra; parallax [{:.3}, {:.3}] mas median {:.3}; color: {} measured, median BP-RP {:.2}, {} blue-side, {} red-side (>|median|+1 mag); spectral-form: {} shape outliers (rms > 0.5 vs chunk median shape){}",
        stars.len(),
        plx[0],
        plx[plx.len() - 1],
        plx[plx.len() / 2],
        ci.len(),
        c_med,
        n_blue,
        n_red,
        n_outliers,
        match top_residual {
            Some(r) => format!(", top residual {:.3}", r),
            None => String::new(),
        }
    );
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
    let source_range: Option<(u64, u64)> = {
        let idx = args.iter().position(|a| a == "--source-range");
        idx.and_then(|i| {
            let lo = args.get(i + 1)?.parse::<u64>().ok()?;
            let hi = args.get(i + 2)?.parse::<u64>().ok()?;
            Some((lo, hi))
        })
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
    let release_tag = arg_value(&args, "--release-tag");
    let (label, text) = match source_range {
        Some((lo, hi)) => match fetch_source_range(lo, hi) {
            Some(t) => (format!("source_id[{}, {})", lo, hi), t),
            None => {
                eprintln!("source-range {}-{} returned void", lo, hi);
                std::process::exit(1);
            }
        },
        None => {
            let p = match arg_value(&args, "--input") {
                Some(p) => p,
                None => {
                    eprintln!(
                        "--input absent — the TAP export is undeclared (or pass --source-range <lo> <hi>)"
                    );
                    std::process::exit(1);
                }
            };
            match std::fs::read_to_string(&p) {
                Ok(t) => (p, t),
                Err(_) => {
                    eprintln!("read {} returned void", p);
                    std::process::exit(1);
                }
            }
        }
    };
    let lines: Vec<&str> = text.lines().collect();
    let header_idx = match lines.iter().position(|l| l.contains(',')) {
        Some(i) => i,
        None => {
            eprintln!("{}: header absent — the columns stay unnamed", label);
            std::process::exit(1);
        }
    };
    if source_range.is_some() && lines.len().saturating_sub(header_idx + 1) >= SYNC_CAP_GUARD {
        eprintln!(
            "{}: {} rows at the ~{} sync cap — the range is truncated, narrow --source-range",
            label,
            lines.len().saturating_sub(header_idx + 1),
            SYNC_CAP_GUARD
        );
        std::process::exit(1);
    }
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
            label
        );
        std::process::exit(1);
    };
    let ci_bp = col("phot_bp_mean_mag");
    let ci_rp = col("phot_rp_mean_mag");
    let mut stars: Vec<XpStar> = Vec::new();
    let mut colors: Vec<Option<f64>> = Vec::new();
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
        let color = match (ci_bp, ci_rp) {
            (Some(b), Some(r)) => {
                let bp = parts
                    .get(b)
                    .and_then(|s| s.trim().parse::<f64>().ok())
                    .filter(|v| v.is_finite());
                let rp = parts
                    .get(r)
                    .and_then(|s| s.trim().parse::<f64>().ok())
                    .filter(|v| v.is_finite());
                match (bp, rp) {
                    (Some(bp), Some(rp)) => Some(bp - rp),
                    _ => None,
                }
            }
            _ => None,
        };
        colors.push(color);
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
        label,
        stars.len(),
        malformed,
        skipped_plx,
        skipped_bins
    );
    if stars.is_empty() {
        eprintln!("no valid stars — the catalog stays unwritten (0 honored)");
        std::process::exit(1);
    }
    archive_verdict(&stars, &colors);
    let bytes = write_xp_spectra_bin(epoch_tdb, &stars);
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_xp_spectra_bin(&bytes) {
        Some((epoch, parsed)) => {
            let lossless = write_xp_spectra_bin(epoch, &parsed) == bytes;
            eprintln!(
                "{}: {} stars, epoch_tdb {} — the archive parses; re-serialization {} ({} B)",
                out,
                parsed.len(),
                epoch,
                if lossless {
                    "byte-identical"
                } else {
                    "drifted"
                },
                bytes.len()
            );
            if parsed.len() != stars.len() || epoch != epoch_tdb || !lossless {
                eprintln!(
                    "{}: content verdict void ({} of {} stars, epoch {}, re-serialization {}) — the catalog stays unverified",
                    out,
                    parsed.len(),
                    stars.len(),
                    epoch,
                    if lossless { "byte-identical" } else { "drifted" }
                );
                std::process::exit(1);
            }
        }
        None => {
            eprintln!(
                "{}: roundtrip parse void — the catalog stays unverified",
                out
            );
            std::process::exit(1);
        }
    }
    if ci_mode {
        let reached = match release_tag.as_deref() {
            Some(tag) => upload_release(tag, &out),
            None => upload_asset(&out),
        };
        if !reached {
            std::process::exit(1);
        }
    }
}
