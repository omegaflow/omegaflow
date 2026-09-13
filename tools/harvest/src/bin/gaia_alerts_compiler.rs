use omegaflow::archivar::{embedded_lsk, LeapSeconds};
use omegaflow::cdn::upload_release;
use omegaflow::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::lsk::days_from_civil;
use omegaflow::skydirection::{parse_bin, write_bin, SkyBandSeries, SkyDirection, SkySample};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

const NETLOC: &str = "gsaweb.ast.cam.ac.uk";
const BAND_G: &str = "G";
const ARRAY_MARKER: &str = "var alerts = [";

fn ra_dec_plausible(ra: f64, dec: f64) -> bool {
    ra.is_finite() && dec.is_finite() && (0.0..360.0).contains(&ra) && (-90.0..=90.0).contains(&dec)
}

fn utc_to_unix(s: &str) -> Option<f64> {
    let (date, time) = s.split_once(' ')?;
    let mut d = date.split('-');
    let year: i64 = d.next()?.parse().ok()?;
    let month: i64 = d.next()?.parse().ok()?;
    let day: i64 = d.next()?.parse().ok()?;
    let mut t = time.split(':');
    let hour: f64 = t.next()?.parse().ok()?;
    let minute: f64 = t.next()?.parse().ok()?;
    let second: f64 = t.next()?.parse().ok()?;
    let days = days_from_civil(year, month, day)? as f64;
    Some(days * 86400.0 + hour * 3600.0 + minute * 60.0 + second)
}

fn extract_alerts_array(html: &[u8]) -> Option<Vec<u8>> {
    let text = std::str::from_utf8(html).ok()?;
    let open = text.find(ARRAY_MARKER)? + ARRAY_MARKER.len() - 1;
    let rest = &text[open..];
    let close = rest.find("];")?;
    Some(rest[..close + 1].as_bytes().to_vec())
}

fn alert_direction(lsk: &LeapSeconds, row: &JsonVal) -> Option<(SkyDirection, bool)> {
    let name = jstr(row, "name")?;
    let ra = jnum(row, "ra")?;
    let dec = jnum(row, "dec")?;
    if !ra_dec_plausible(ra, dec) {
        return None;
    }
    let mut d = SkyDirection {
        name,
        ra_deg: ra,
        dec_deg: dec,
        sigma_arcsec: None,
        bands: Vec::new(),
        distance: None,
        redshift: None,
    };
    let (Some(mag), Some(obstime)) = (jnum(row, "alertMag"), jstr(row, "obstime")) else {
        return Some((d, false));
    };
    if !(mag.is_finite() && mag > 0.0) {
        return Some((d, false));
    }
    let Some(tdb) = utc_to_unix(&obstime).and_then(|u| lsk.unix_to_tdb(u)) else {
        return Some((d, false));
    };
    d.bands.push(SkyBandSeries {
        band: Some(BAND_G.to_string()),
        samples: vec![SkySample { tdb, mag }],
    });
    Some((d, true))
}

fn compile_into(input: &str, lsk: &LeapSeconds, out: &mut Vec<SkyDirection>) -> CompileCounts {
    let bytes = match std::fs::read(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("gaia_alerts: read {} returned void: {}", input, e);
            return CompileCounts::zero();
        }
    };
    let Some(array) = extract_alerts_array(&bytes) else {
        eprintln!(
            "gaia_alerts: {} carries no {} array — the parser stays pending",
            input, ARRAY_MARKER
        );
        return CompileCounts::zero();
    };
    let Ok(text) = std::str::from_utf8(&array) else {
        eprintln!("gaia_alerts: the alert array is not UTF-8 — the parser stays pending");
        return CompileCounts::zero();
    };
    let Some(JsonVal::Arr(rows)) = parse_json(text) else {
        eprintln!("gaia_alerts: the alert array does not parse — the parser stays pending");
        return CompileCounts::zero();
    };
    let mut counts = CompileCounts {
        rows: rows.len(),
        ..CompileCounts::zero()
    };
    let mut classes: HashSet<String> = HashSet::new();
    for row in &rows {
        if let Some(class) = jstr(row, "classification") {
            classes.insert(class);
        }
        match alert_direction(lsk, row) {
            Some((d, anchored)) => {
                if anchored {
                    counts.anchored += 1;
                } else {
                    counts.mag_void += 1;
                }
                out.push(d);
            }
            None => {
                counts.refused += 1;
            }
        }
    }
    counts.classes = classes.len();
    counts
}

struct CompileCounts {
    rows: usize,
    anchored: usize,
    mag_void: usize,
    refused: usize,
    classes: usize,
}

impl CompileCounts {
    fn zero() -> CompileCounts {
        CompileCounts {
            rows: 0,
            anchored: 0,
            mag_void: 0,
            refused: 0,
            classes: 0,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    let Some(input) = input else {
        eprintln!("gaia_alerts: --input absent — the index path is never silent");
        std::process::exit(1);
    };
    let Some(out_path) = out_path else {
        eprintln!("gaia_alerts: --out absent — the asset path is never silent");
        std::process::exit(1);
    };
    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "gaia_alerts: the embedded naif0012.tls leap table is absent — no sample epoch folds to the TDB clock; the harvest stays unrun (0 honored, pending)"
        );
        return;
    };
    let mut directions: Vec<SkyDirection> = Vec::new();
    let counts = compile_into(&input, &lsk, &mut directions);
    if directions.is_empty() {
        eprintln!(
            "gaia_alerts: {} row(s) measured, none held — the asset stays unwritten (0 honored)",
            counts.rows
        );
        return;
    }
    let Some(bytes) = write_bin(&directions) else {
        eprintln!("gaia_alerts: a held direction is not finite or not serializable — the asset stays unwritten (0 honored)");
        return;
    };
    match parse_bin(&bytes) {
        Some(parsed) if parsed.len() == directions.len() => {
            let band_series: usize = parsed.iter().map(|d| d.bands.len()).sum();
            eprintln!(
                "gaia_alerts: {} alert row(s) measured, {} direction(s) held, {} band series, {} magnitude(s) anchored, {} magnitude(s) void, {} row(s) refused (position absent or out of the ICRS gate), {} distinct classification label(s) measured — {out_path} holds {} bytes, the roundtrip reads back",
                counts.rows,
                directions.len(),
                band_series,
                counts.anchored,
                counts.mag_void,
                counts.refused,
                counts.classes,
                bytes.len()
            );
        }
        _ => {
            eprintln!("gaia_alerts: the roundtrip does not read back — the asset stays unwritten");
            return;
        }
    }
    let file = match File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("gaia_alerts: write {} returned void: {}", out_path, e);
            return;
        }
    };
    let mut w = BufWriter::new(file);
    if w.write_all(&bytes).is_err() {
        eprintln!("gaia_alerts: write {} returned void", out_path);
        return;
    }
    if w.flush().is_err() {
        eprintln!("gaia_alerts: flush {} returned void", out_path);
        return;
    }
    if ci_mode && !upload_release(NETLOC, &out_path) {
        eprintln!("gaia_alerts: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_folds_to_unix_days_epoch() {
        let u = utc_to_unix("1970-01-01 00:00:00").unwrap();
        assert_eq!(u, 0.0);
        let u = utc_to_unix("2025-01-15 00:59:00").unwrap();
        assert!(
            u > 1.7e9 && u < 1.8e9,
            "a 2025 epoch lives near the measured range: {u}"
        );
        assert!(utc_to_unix("not a date").is_none());
        assert!(utc_to_unix("2025-13-40 00:00:00").is_none());
    }

    #[test]
    fn array_extraction_bounds_the_single_json_array() {
        let html = b"<html><script>var url = window.location;var alerts = [{\"name\":\"Gaia25aeh\"},{\"name\":\"Gaia25aeg\"}];var index_table = 1;</script></html>";
        let array = extract_alerts_array(html).unwrap();
        assert_eq!(
            array,
            b"[{\"name\":\"Gaia25aeh\"},{\"name\":\"Gaia25aeg\"}]".as_slice()
        );
        assert!(extract_alerts_array(b"<html>no array</html>").is_none());
    }

    #[test]
    fn a_measured_alert_row_becomes_a_g_band_direction() {
        let lsk = embedded_lsk().unwrap();
        let json = r#"{"name":"Gaia25aeh","tnsid":null,"obstime":"2025-01-15 00:59:00","ra":"312.55124","dec":"28.89383","alertMag":"15.86","historicMag":"18.31","historicStdDev":"0.26","classification":"unknown","published":"2025-01-16 17:56:09","comment":"","per_alert":{"link":"/alerts/alert/Gaia25aeh/","name":"Gaia25aeh"},"rvs":false}"#;
        let parsed = parse_json(json).unwrap();
        let (d, anchored) = alert_direction(&lsk, &parsed).unwrap();
        assert!(anchored);
        assert_eq!(d.name, "Gaia25aeh");
        assert!((d.ra_deg - 312.55124).abs() < 1e-9);
        assert!((d.dec_deg - 28.89383).abs() < 1e-9);
        assert_eq!(d.bands.len(), 1);
        assert_eq!(d.bands[0].band.as_deref(), Some("G"));
        assert_eq!(d.bands[0].samples.len(), 1);
        assert!((d.bands[0].samples[0].mag - 15.86).abs() < 1e-9);
        assert!(d.bands[0].samples[0].tdb.is_finite());
    }

    #[test]
    fn an_out_of_icrs_row_is_refused() {
        let lsk = embedded_lsk().unwrap();
        let json = r#"{"name":"bad","ra":"400.0","dec":"28.0","alertMag":"15.0","obstime":"2025-01-15 00:59:00"}"#;
        let parsed = parse_json(json).unwrap();
        assert!(alert_direction(&lsk, &parsed).is_none());
    }
}
