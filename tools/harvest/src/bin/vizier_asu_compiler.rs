use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

const ASU_MIRRORS: [&str; 3] = [
    "https://vizier.cds.unistra.fr/viz-bin/asu-tsv",
    "https://vizier.cfa.harvard.edu/viz-bin/asu-tsv",
    "https://vizier.u-strasbg.fr/viz-bin/asu-tsv",
];
const NVSS_TABLE: &str = "VIII/65/nvss";
const SDSS_TABLE: &str = "V/154/sdss16";
const NVSS_SELECT: &str = "NVSS,S1.4";
const NVSS_ADD: &str = "_RAJ2000,_DEJ2000";
const SDSS_SELECT: &str = "RA_ICRS,DE_ICRS,zsp";
const ZSP_RANGE: &str = "-1..8";
const DEFAULT_RADIUS_ARCSEC: f64 = 10.0;
const DEFAULT_RA_BUFFER_DEG: f64 = 2.0;
const GRID_CELL_DEG: f64 = 0.05;
const ARCSEC_PER_RAD: f64 = 206_264.806_247_096_36;
const DEFAULT_LIMIT: usize = 5_000_000;

struct NvssRow {
    name: String,
    ra: f64,
    dec: f64,
    flux_raw: String,
}

struct SdssRow {
    ra: f64,
    dec: f64,
    z_raw: String,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn asu_timeout() -> u64 {
    std::env::var("OMEGAFLOW_ASU_TIMEOUT")
        .ok()
        .or_else(|| std::env::var("OMEGAFLOW_TAP_TIMEOUT").ok())
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(1800)
}

fn asu_fetch(url: &str, timeout: u64) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("--fail-with-body")
        .arg("-m")
        .arg(timeout.to_string())
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        let body = String::from_utf8_lossy(&out.stdout);
        eprintln!(
            "asu fetch http {}: {} body={}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim(),
            &body[..body.len().min(600)]
        );
        None
    }
}

fn asu_fetch_mirrors(query: &str, timeout: u64) -> Option<String> {
    for attempt in 1..=3 {
        for root in ASU_MIRRORS {
            if let Some(body) = asu_fetch(&format!("{root}?{query}"), timeout) {
                return Some(body);
            }
        }
        eprintln!("vizier_asu_compiler: ASU mirror round {attempt} returned void");
    }
    None
}

fn parse_asu_tsv(text: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let mut names: Vec<String> = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut dash: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim_end_matches('\r');
        if let Some(rest) = t.strip_prefix("#Column") {
            if let Some(name) = rest.trim_start().split_whitespace().next() {
                names.push(name.to_string());
            }
        } else if !t.trim().is_empty()
            && t.contains('-')
            && t.chars().all(|c| c == '-' || c == '\t' || c == ' ')
        {
            dash = Some(i);
            break;
        }
    }
    let mut rows = Vec::new();
    if let Some(i) = dash {
        for line in lines.iter().skip(i + 1) {
            let t = line.trim_end_matches('\r');
            if t.trim().is_empty() {
                continue;
            }
            let cells: Vec<String> = t.split('\t').map(|c| c.trim().to_string()).collect();
            rows.push(cells);
        }
    }
    (names, rows)
}

fn col_index(names: &[String], key: &str) -> Option<usize> {
    names.iter().position(|n| n == key)
}

fn nvss_rows(text: &str, window: Option<(f64, f64)>) -> Vec<NvssRow> {
    let (names, raw) = parse_asu_tsv(text);
    let (Some(i_name), Some(i_ra), Some(i_dec), Some(i_flux)) = (
        col_index(&names, "NVSS"),
        col_index(&names, "_RAJ2000"),
        col_index(&names, "_DEJ2000"),
        col_index(&names, "S1.4"),
    ) else {
        eprintln!("nvss asu: columns absent, available {names:?}");
        return Vec::new();
    };
    let mut out = Vec::new();
    for cells in raw {
        let (Some(ra_s), Some(dec_s)) = (cells.get(i_ra), cells.get(i_dec)) else {
            continue;
        };
        let (Ok(ra), Ok(dec)) = (ra_s.parse::<f64>(), dec_s.parse::<f64>()) else {
            continue;
        };
        if !ra.is_finite() || !dec.is_finite() {
            continue;
        }
        if let Some((lo, hi)) = window {
            if !(ra >= lo && ra < hi) {
                continue;
            }
        }
        let (Some(name), Some(flux_raw)) = (cells.get(i_name), cells.get(i_flux)) else {
            continue;
        };
        match flux_raw.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => {}
            _ => continue,
        }
        out.push(NvssRow {
            name: name.clone(),
            ra,
            dec,
            flux_raw: flux_raw.clone(),
        });
    }
    out
}

fn sdss_rows(text: &str) -> Vec<SdssRow> {
    let (names, raw) = parse_asu_tsv(text);
    let (Some(i_ra), Some(i_dec), Some(i_z)) = (
        col_index(&names, "RA_ICRS"),
        col_index(&names, "DE_ICRS"),
        col_index(&names, "zsp"),
    ) else {
        eprintln!("sdss asu: columns absent, available {names:?}");
        return Vec::new();
    };
    let mut out = Vec::new();
    for cells in raw {
        let (Some(ra_s), Some(dec_s)) = (cells.get(i_ra), cells.get(i_dec)) else {
            continue;
        };
        let (Ok(ra), Ok(dec)) = (ra_s.parse::<f64>(), dec_s.parse::<f64>()) else {
            continue;
        };
        let Some(z_raw) = cells.get(i_z) else {
            continue;
        };
        let Ok(z) = z_raw.parse::<f64>() else {
            continue;
        };
        if !ra.is_finite() || !dec.is_finite() || !z.is_finite() {
            continue;
        }
        out.push(SdssRow {
            ra,
            dec,
            z_raw: z_raw.clone(),
        });
    }
    out
}

fn angular_arcsec(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let d1 = dec1.to_radians();
    let d2 = dec2.to_radians();
    let dlat = (dec2 - dec1).to_radians();
    let dlon = (ra2 - ra1).to_radians();
    let h = (dlat / 2.0).sin().powi(2) + d1.cos() * d2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * h.sqrt().clamp(0.0, 1.0).asin();
    c * ARCSEC_PER_RAD
}

fn cell_key(ra: f64, dec: f64) -> (i64, i64) {
    (
        (ra / GRID_CELL_DEG).floor() as i64,
        ((dec + 90.0) / GRID_CELL_DEG).floor() as i64,
    )
}

fn json_string(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            _ => o.push(c),
        }
    }
    o.push('"');
    o
}

fn crossmatch(nvss: &[NvssRow], sdss: &[SdssRow], radius_arcsec: f64) -> Vec<String> {
    let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (i, s) in sdss.iter().enumerate() {
        grid.entry(cell_key(s.ra, s.dec)).or_default().push(i);
    }
    let mut out = Vec::new();
    for n in nvss {
        let (cx, cy) = cell_key(n.ra, n.dec);
        let mut best: Option<(f64, &SdssRow)> = None;
        for dx in -1..=1 {
            for dy in -1..=1 {
                let Some(ids) = grid.get(&(cx + dx, cy + dy)) else {
                    continue;
                };
                for &i in ids {
                    let s = &sdss[i];
                    let d = angular_arcsec(n.ra, n.dec, s.ra, s.dec);
                    if !d.is_finite() || d > radius_arcsec {
                        continue;
                    }
                    if best.map(|(bd, _)| d < bd).unwrap_or(true) {
                        best = Some((d, s));
                    }
                }
            }
        }
        if let Some((_, s)) = best {
            out.push(format!(
                "{{\"name\":{},\"ra\":{},\"dec\":{},\"flux\":{},\"z\":{}}}",
                json_string(&n.name),
                n.ra,
                n.dec,
                n.flux_raw,
                s.z_raw
            ));
        }
    }
    out
}

fn ra_windows(lo: f64, hi: f64, buffer: f64) -> Vec<(f64, f64)> {
    let low = (lo - buffer).max(0.0);
    let high = hi + buffer;
    if high <= 360.0 {
        vec![(low, high)]
    } else {
        vec![(low, 360.0), (0.0, (high - 360.0).min(360.0))]
    }
}

fn parse_ra_range(spec: &str) -> Option<(f64, f64)> {
    let (a, b) = spec.split_once(':')?;
    let lo = a.trim().parse::<f64>().ok()?;
    let hi = b.trim().parse::<f64>().ok()?;
    if lo.is_finite() && hi.is_finite() && lo >= 0.0 && hi <= 360.0 && lo < hi {
        Some((lo, hi))
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = match arg_value(&args, "--out") {
        Some(p) => p,
        None => {
            eprintln!(
                "vizier_asu_compiler --out <path> [--ra-range <lo>:<hi>] [--xmatch-radius <arcsec>]"
            );
            std::process::exit(2);
        }
    };
    let radius = arg_value(&args, "--xmatch-radius")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite() && *v > 0.0)
        .unwrap_or(DEFAULT_RADIUS_ARCSEC);
    let limit = arg_value(&args, "--limit")
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_LIMIT);
    let window = match arg_value(&args, "--ra-range") {
        Some(spec) => match parse_ra_range(&spec) {
            Some(w) => Some(w),
            None => {
                eprintln!("vizier_asu_compiler: --ra-range {spec} is not <lo>:<hi> in [0,360)");
                std::process::exit(2);
            }
        },
        None => None,
    };
    let timeout = asu_timeout();

    let nvss_query = match window {
        Some((lo, hi)) => format!(
            "-source={NVSS_TABLE}&-out={NVSS_SELECT}&-out.add={NVSS_ADD}&-out.max={limit}&_RAJ2000={lo}..{hi}"
        ),
        None => {
            format!("-source={NVSS_TABLE}&-out={NVSS_SELECT}&-out.add={NVSS_ADD}&-out.max={limit}")
        }
    };
    let Some(body) = asu_fetch_mirrors(&nvss_query, timeout) else {
        eprintln!("vizier_asu_compiler: NVSS ASU returned void");
        std::process::exit(1);
    };
    let nvss = nvss_rows(&body, window);
    if nvss.is_empty() {
        eprintln!("vizier_asu_compiler: NVSS ASU carried no parsable row");
        std::process::exit(1);
    }

    let windows = match window {
        Some((lo, hi)) => ra_windows(lo, hi, DEFAULT_RA_BUFFER_DEG),
        None => vec![(0.0, 360.0)],
    };
    let mut sdss: Vec<SdssRow> = Vec::new();
    for (a, b) in windows {
        let query = format!(
            "-source={SDSS_TABLE}&-out={SDSS_SELECT}&-out.max={limit}&RA_ICRS={a}..{b}&zsp={ZSP_RANGE}"
        );
        let Some(body) = asu_fetch_mirrors(&query, timeout) else {
            eprintln!("vizier_asu_compiler: SDSS ASU {a}..{b} returned void");
            std::process::exit(1);
        };
        let mut part = sdss_rows(&body);
        if part.len() >= limit {
            eprintln!(
                "vizier_asu_compiler: SDSS ASU {a}..{b} returned {limit} rows — the reply is truncated, the crossmatch stays unwritten"
            );
            std::process::exit(1);
        }
        eprintln!(
            "vizier_asu_compiler: SDSS spec-z {a}..{b}: {} rows",
            part.len()
        );
        sdss.append(&mut part);
    }
    if sdss.is_empty() {
        eprintln!("vizier_asu_compiler: SDSS ASU carried no parsable spec-z row");
        std::process::exit(1);
    }

    let rows_out = crossmatch(&nvss, &sdss, radius);
    let mut buf = String::from("[");
    for (k, r) in rows_out.iter().enumerate() {
        if k > 0 {
            buf.push(',');
        }
        buf.push_str(r);
    }
    buf.push_str("]\n");
    match std::fs::File::create(&out) {
        Ok(mut f) => {
            if let Err(err) = f.write_all(buf.as_bytes()) {
                eprintln!("vizier_asu_compiler: write {out}: {err}");
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("vizier_asu_compiler: write {out} returned void: {err}");
            std::process::exit(1);
        }
    }
    eprintln!(
        "vizier_asu_compiler: {} NVSS rows, {} SDSS spec-z rows, {} matched → {out}",
        nvss.len(),
        sdss.len(),
        rows_out.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const NVSS_FIXTURE: &str = "\
#\tVizieR Astronomical Server\n\
#Column\t_RAJ2000\t(F11.7)\tRight ascension J2000\n\
#Column\t_DEJ2000\t(F11.7)\tDeclination J2000\n\
#Column\tNVSS\t(a14)\tSource name\n\
#Column\tS1.4\t(F8.1)\tIntegrated 1.4GHz flux density\n\
_RAJ2000\t_DEJ2000\tNVSS\tS1.4\n\
deg\tdeg\t\tmJy\n\
-----------\t-----------\t--------------\t--------\n\
000.0003750\t-34.1193056\t000000-340709 \t     2.7\n\
010.0000000\t+00.0000000\t010000+000000 \t    50.0\n\
010.0055556\t+00.0000000\t010000+000000B\t    30.0\n\
020.0000000\t+00.0000000\t020000+000000 \t         \n";

    const SDSS_FIXTURE: &str = "\
#\tVizieR Astronomical Server\n\
#Column\tRA_ICRS\t(F10.6)\tRight Ascension of the object\n\
#Column\tDE_ICRS\t(F10.6)\tDeclination of the object\n\
#Column\tzsp\t(F8.5)\tSpectroscopic final redshift\n\
RA_ICRS\tDE_ICRS\tzsp\n\
deg\tdeg\t\n\
----------\t----------\t--------\n\
010.0000000\t+00.0000000\t 0.10000\n\
010.0010000\t+00.0000000\t 0.20000\n";

    #[test]
    fn asu_tsv_names_are_read_in_output_order() {
        let (names, rows) = parse_asu_tsv(NVSS_FIXTURE);
        assert_eq!(names, vec!["_RAJ2000", "_DEJ2000", "NVSS", "S1.4"]);
        assert_eq!(rows.len(), 4);
    }

    #[test]
    fn nvss_rows_hold_only_a_positive_flux() {
        let rows = nvss_rows(NVSS_FIXTURE, None);
        assert_eq!(rows.len(), 3, "the flux-less fourth row is refused");
        assert_eq!(rows[0].name, "000000-340709");
        assert!((rows[0].ra - 0.000375).abs() < 1e-9);
        assert!((rows[0].dec + 34.1193056).abs() < 1e-7);
    }

    #[test]
    fn nvss_window_filter_holds_the_half_open_interval() {
        let rows = nvss_rows(NVSS_FIXTURE, Some((10.0, 11.0)));
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|r| r.ra >= 10.0 && r.ra < 11.0));
    }

    #[test]
    fn crossmatch_takes_the_nearest_row_within_radius() {
        let nvss = nvss_rows(NVSS_FIXTURE, None);
        let sdss = sdss_rows(SDSS_FIXTURE);
        let out = crossmatch(&nvss, &sdss, 10.0);
        assert_eq!(
            out.len(),
            1,
            "only 010000 sits within ten arcsec; 000000 and 010005 stay unmatched"
        );
        assert!(out[0].contains("\"name\":\"010000+000000\""));
        assert!(
            out[0].contains("\"z\":0.10000"),
            "nearest row wins: {}",
            out[0]
        );
    }

    #[test]
    fn out_of_radius_position_stays_unmatched() {
        let nvss = vec![NvssRow {
            name: "far".to_string(),
            ra: 10.02,
            dec: 0.0,
            flux_raw: "1.0".to_string(),
        }];
        let sdss = sdss_rows(SDSS_FIXTURE);
        assert!(crossmatch(&nvss, &sdss, 10.0).is_empty());
    }

    #[test]
    fn angular_arcsec_names_a_known_separation() {
        let d = angular_arcsec(10.0, 0.0, 10.001, 0.0);
        assert!((d - 3.6).abs() < 0.01, "0.001 deg at dec 0 is 3.6 arcsec");
    }

    #[test]
    fn ra_windows_wrap_the_last_slice() {
        assert_eq!(ra_windows(0.0, 45.0, 2.0), vec![(0.0, 47.0)]);
        assert_eq!(
            ra_windows(315.0, 360.0, 2.0),
            vec![(313.0, 360.0), (0.0, 2.0)]
        );
    }

    #[test]
    fn ra_range_rejects_a_bad_spec() {
        assert_eq!(parse_ra_range("0:45"), Some((0.0, 45.0)));
        assert_eq!(parse_ra_range("45:0"), None);
        assert_eq!(parse_ra_range("x:45"), None);
        assert_eq!(parse_ra_range("0;45"), None);
    }
}
