use omegaflow::bl_narrowband::{parse_bin, write_bin, BlNarrowbandEvent};
use omegaflow::cdn::upload_asset;
use omegaflow::lsk::parse as parse_lsk;
use std::process::Command;

const MJD_UNIX_OFFSET: f64 = 40587.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_text(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--max-time")
        .arg("300")
        .arg("--retry")
        .arg("2")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        None
    }
}

fn sexagesimal_to_degrees(tok: &str, hours: bool) -> Option<f64> {
    let t = tok.trim();
    if t.is_empty() {
        return None;
    }
    let sign = if t.starts_with('-') { -1.0 } else { 1.0 };
    let t = t.trim_start_matches(['+', '-']);
    let lower = t.to_ascii_lowercase();
    if !lower.contains(['h', 'd', 'm', 's', ':', '\'', '"', ' ']) {
        let v = t.parse::<f64>().ok()?;
        if !v.is_finite() {
            return None;
        }
        return Some(if hours { v * 15.0 } else { v } * sign);
    }
    let spaced: String = lower
        .chars()
        .map(|c| {
            if matches!(c, 'h' | 'd' | 'm' | 's' | ':' | '\'' | '"' | ' ') {
                ' '
            } else {
                c
            }
        })
        .collect();
    let mut parts: Vec<f64> = Vec::new();
    for p in spaced.split_whitespace() {
        let v = p.parse::<f64>().ok()?;
        if !v.is_finite() {
            return None;
        }
        parts.push(v);
    }
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }
    let base = parts[0]
        + parts.get(1).map_or(0.0, |m| m / 60.0)
        + parts.get(2).map_or(0.0, |s| s / 3600.0);
    Some(if hours { base * 15.0 } else { base } * sign)
}

fn column_index(header: &[String], want: &str) -> Option<usize> {
    header.iter().position(|h| h.trim() == want)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out").unwrap_or_else(|| "/tmp/opencode/bl_narrowband.bin".into());
    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
    let limit: Option<usize> = arg_value(&args, "--limit").and_then(|v| v.parse().ok());
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
    let text = if let Some(p) = &input {
        match std::fs::read_to_string(p) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("read {} returned void", p);
                std::process::exit(1);
            }
        }
    } else if let Some(u) = &url {
        match fetch_text(u) {
            Some(t) => t,
            None => {
                eprintln!("curl {} returned void", u);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("--input or --url absent — the measurement table is undeclared");
        std::process::exit(1);
    };

    let mut lines = text.lines();
    let header_line = match lines.next() {
        Some(h) => h,
        None => {
            eprintln!("the CSV carries no header row — nothing compiled");
            std::process::exit(1);
        }
    };
    let header: Vec<String> = header_line
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let idx = |name: &str| -> usize {
        column_index(&header, name).unwrap_or_else(|| {
            eprintln!(
                "column {} absent from the header — the row stays uncompiled",
                name
            );
            std::process::exit(1);
        })
    };
    let ra_i = idx("RA");
    let dec_i = idx("DEC");
    let freq_i = idx("Freq");
    let snr_i = idx("SNR");
    let mjd_i = idx("MJD");
    let fs_i = column_index(&header, "FreqStart");
    let fe_i = column_index(&header, "FreqEnd");
    let bw_i = column_index(&header, "bin_width").or_else(|| column_index(&header, "BinWidth"));
    if (fs_i.is_some()) != (fe_i.is_some()) {
        eprintln!("FreqStart and FreqEnd must both be present — the band width stays unread");
        std::process::exit(1);
    }
    if fs_i.is_none() && bw_i.is_none() {
        eprintln!(
            "no band width column (FreqStart/FreqEnd or bin_width) — bin_width is mandatory for a line, 0 is not its truth"
        );
        std::process::exit(1);
    }

    let mut events: Vec<BlNarrowbandEvent> = Vec::new();
    let mut malformed = 0usize;
    let mut pending_width = 0usize;
    for line in lines {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        let at = |i: usize| -> Option<&str> { cols.get(i).copied().filter(|s| !s.is_empty()) };
        let (Some(ra_tok), Some(dec_tok)) = (at(ra_i), at(dec_i)) else {
            malformed += 1;
            continue;
        };
        let (Some(freq_mhz), Some(snr), Some(mjd)) = (at(freq_i), at(snr_i), at(mjd_i)) else {
            malformed += 1;
            continue;
        };
        let (Some(ra_deg), Some(dec_deg)) = (
            sexagesimal_to_degrees(ra_tok, ra_tok.to_ascii_lowercase().contains('h')),
            sexagesimal_to_degrees(dec_tok, false),
        ) else {
            malformed += 1;
            continue;
        };
        let (freq_hz, snr_v, mjd_v) = match (
            freq_mhz.parse::<f64>(),
            snr.parse::<f64>(),
            mjd.parse::<f64>(),
        ) {
            (Ok(f), Ok(s), Ok(m)) if f.is_finite() && s.is_finite() && m.is_finite() => {
                (f * 1.0e6, s, m)
            }
            _ => {
                malformed += 1;
                continue;
            }
        };
        let bin_width_hz = if let (Some(fs), Some(fe)) = (fs_i, fe_i) {
            match (
                at(fs).and_then(|v| v.parse::<f64>().ok()),
                at(fe).and_then(|v| v.parse::<f64>().ok()),
            ) {
                (Some(a), Some(b)) if a.is_finite() && b.is_finite() && b > a => (b - a) * 1.0e6,
                _ => {
                    pending_width += 1;
                    continue;
                }
            }
        } else if let Some(bw) = bw_i {
            match at(bw).and_then(|v| v.parse::<f64>().ok()) {
                Some(w) if w.is_finite() && w > 0.0 => w * 1.0e6,
                _ => {
                    pending_width += 1;
                    continue;
                }
            }
        } else {
            unreachable!("width column presence checked above");
        };
        let unix = (mjd_v - MJD_UNIX_OFFSET) * 86400.0;
        let epoch_tdb = match lsk.unix_to_tdb(unix) {
            Some(t) => t,
            None => {
                malformed += 1;
                continue;
            }
        };
        events.push(BlNarrowbandEvent {
            ra_deg,
            dec_deg,
            epoch_tdb,
            freq_hz,
            bin_width_hz,
            val: snr_v,
        });
        if let Some(l) = limit {
            if events.len() >= l {
                break;
            }
        }
    }
    eprintln!(
        "{}: {} hits compiled, {} rows malformed, {} rows band-width pending",
        input.as_deref().or(url.as_deref()).unwrap_or("CSV"),
        events.len(),
        malformed,
        pending_width
    );
    if events.is_empty() {
        eprintln!("no line hits compiled — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = match write_bin(&events) {
        Some(b) => b,
        None => {
            eprintln!("write_bin: non-finite value refused");
            std::process::exit(1);
        }
    };
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {} returned void", out);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{}: {} hits, roundtrip parses ({} B)",
                out,
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{}: roundtrip parse void — the bin stays unverified", out);
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_asset(&out) {
        eprintln!("{}: CDN upload returned void", out);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sexagesimal_ra_hours_to_degrees() {
        let deg = sexagesimal_to_degrees("22h57m28.2s", true).expect("parse");
        let expect = (22.0 + 57.0 / 60.0 + 28.2 / 3600.0) * 15.0;
        assert!((deg - expect).abs() < 1e-9);
        let dec = sexagesimal_to_degrees("20d46m08.04s", false).expect("parse");
        let expect_dec = 20.0 + 46.0 / 60.0 + 8.04 / 3600.0;
        assert!((dec - expect_dec).abs() < 1e-9);
    }

    #[test]
    fn sexagesimal_decimal_and_south() {
        assert_eq!(
            sexagesimal_to_degrees("344.3675", false).expect("parse"),
            344.3675
        );
        let south = sexagesimal_to_degrees("-05d30m00s", false).expect("parse");
        assert!((south - (-5.5)).abs() < 1e-12);
        assert_eq!(sexagesimal_to_degrees("", false), None);
    }
}
