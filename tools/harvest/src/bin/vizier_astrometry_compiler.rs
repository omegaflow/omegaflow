use omegaflow::archivar::LeapSeconds;
use omegaflow::archivar::astrometry_series::{
    AstroSample, AstroSeries, jd_utc_to_tdb, parse_bin, write_bin,
};
use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sexagesimal::{sexagesimal_dec_to_deg, sexagesimal_ra_to_deg};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::{CDN_BASE, upload_release};
use std::path::Path;

const NETLOC: &str = "vizier.cfa.harvard.edu";
const SOURCE_ROOT: &str = "https://vizier.cfa.harvard.edu/viz-bin/asu-tsv?-source=J/A+A/582/A8";
const TABLES: [&str; 6] = [
    "uranu_j", "ariel_j", "umbri_j", "titan_j", "obero_j", "miran_j",
];
const OUT_MAX: usize = 100_000;
const COMPILER: &str = "tools/harvest/src/bin/vizier_astrometry_compiler.rs";

struct RawObs {
    jd_utc: f64,
    ra_deg: f64,
    dec_deg: f64,
    e_ra_mas: f64,
    e_dec_mas: f64,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn table_args(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--table" {
            if let Some(t) = args.get(i + 1) {
                out.push(t.clone());
                i += 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn tsv_url(table: &str) -> String {
    format!("{SOURCE_ROOT}/{table}&-out.max={OUT_MAX}")
}

fn body_name(table: &str) -> String {
    match table.strip_suffix("_j") {
        Some(stem) if !stem.is_empty() => stem.to_string(),
        _ => table.to_string(),
    }
}

fn parse_asu_tsv(text: &str) -> Vec<RawObs> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with('-') {
            continue;
        }
        let fields: Vec<&str> = t.split('\t').map(|f| f.trim()).collect();
        if fields.len() < 5 {
            continue;
        }
        let Ok(jd_utc) = fields[0].parse::<f64>() else {
            continue;
        };
        if !jd_utc.is_finite() {
            continue;
        }
        let Some(ra_deg) = sexagesimal_ra_to_deg(fields[1]) else {
            continue;
        };
        let Some(dec_deg) = sexagesimal_dec_to_deg(fields[3]) else {
            continue;
        };
        let e_ra_mas = match fields[2].parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };
        let e_dec_mas = match fields[4].parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };
        out.push(RawObs {
            jd_utc,
            ra_deg,
            dec_deg,
            e_ra_mas,
            e_dec_mas,
        });
    }
    out
}

fn build_series(table: &str, raw: &[RawObs], lsk: &LeapSeconds) -> Option<AstroSeries> {
    let mut samples = Vec::with_capacity(raw.len());
    for r in raw {
        let Some(tdb) = jd_utc_to_tdb(lsk, r.jd_utc) else {
            continue;
        };
        samples.push(AstroSample {
            tdb,
            ra_deg: r.ra_deg,
            dec_deg: r.dec_deg,
            e_ra_mas: r.e_ra_mas,
            e_dec_mas: r.e_dec_mas,
        });
    }
    if samples.is_empty() {
        return None;
    }
    Some(AstroSeries {
        name: body_name(table),
        samples,
    })
}

fn series_match(a: &[AstroSeries], b: &[AstroSeries]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|(x, y)| {
        x.name == y.name
            && x.samples.len() == y.samples.len()
            && x.samples.iter().zip(&y.samples).all(|(s, t)| {
                s.tdb.to_bits() == t.tdb.to_bits()
                    && s.ra_deg.to_bits() == t.ra_deg.to_bits()
                    && s.dec_deg.to_bits() == t.dec_deg.to_bits()
                    && s.e_ra_mas.to_bits() == t.e_ra_mas.to_bits()
                    && s.e_dec_mas.to_bits() == t.e_dec_mas.to_bits()
            })
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!(
                "usage: vizier_astrometry_compiler --out <path> [--table <t>]... [--ci-mode]"
            );
            std::process::exit(2);
        }
    };
    let ci = args.iter().any(|a| a == "--ci-mode");
    let requested = table_args(&args);
    let tables: Vec<String> = if requested.is_empty() {
        TABLES.iter().map(|t| t.to_string()).collect()
    } else {
        requested
    };
    for t in &tables {
        if !TABLES.contains(&t.as_str()) {
            eprintln!(
                "vizier_astrometry_compiler: unknown table {t} — the six measured asu-tsv tables are {TABLES:?}"
            );
            std::process::exit(2);
        }
    }

    let Some(lsk) = embedded_lsk() else {
        eprintln!(
            "vizier_astrometry_compiler: the embedded naif0012 table is absent — the JD/UTC rows stay on the UTC axis"
        );
        std::process::exit(1);
    };

    let mut series: Vec<AstroSeries> = Vec::with_capacity(tables.len());
    for table in &tables {
        let url = tsv_url(table);
        let Some(bytes) = fetch_raw_bytes(&url) else {
            eprintln!(
                "vizier_astrometry_compiler: {url} did not answer 200 — {table} stays pending"
            );
            std::process::exit(1);
        };
        let text = String::from_utf8_lossy(&bytes);
        let raw = parse_asu_tsv(&text);
        if raw.is_empty() {
            eprintln!(
                "vizier_astrometry_compiler: {table} carried no parsable astrometry row — the column block changed or the reply is not asu-tsv"
            );
            std::process::exit(1);
        }
        let Some(s) = build_series(table, &raw, &lsk) else {
            eprintln!(
                "vizier_astrometry_compiler: {table} rows do not fold onto the TDB axis — the series stays unwritten"
            );
            std::process::exit(1);
        };
        eprintln!(
            "vizier_astrometry_compiler: {} -> {} samples ({} JD/UTC rows)",
            table,
            s.samples.len(),
            raw.len()
        );
        series.push(s);
    }

    let Some(bin) = write_bin(&series) else {
        eprintln!(
            "vizier_astrometry_compiler: write_bin refused a non-finite or errorless sample — the asset stays unwritten"
        );
        std::process::exit(1);
    };
    let Some(parsed) = parse_bin(&bin) else {
        eprintln!(
            "vizier_astrometry_compiler: the emitted AST1 bytes do not parse — the asset stays unwritten"
        );
        std::process::exit(1);
    };
    if !series_match(&series, &parsed) {
        eprintln!("vizier_astrometry_compiler: the AST1 roundtrip differs from the emitted set");
        std::process::exit(1);
    }

    if let Some(parent) = Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&out, &bin) {
        eprintln!("vizier_astrometry_compiler: write {out}: {e}");
        std::process::exit(1);
    }
    let sha = sha256_hex(&bin);
    let asset = match Path::new(&out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => {
            eprintln!("vizier_astrometry_compiler: {out} carries no file name — refused");
            std::process::exit(1);
        }
    };

    println!("url {CDN_BASE}/{NETLOC}/{asset}");
    println!("format astrometry_series");
    println!("origin {SOURCE_ROOT}");
    println!("compiler {COMPILER}");
    println!("sha256 {sha}");
    eprintln!(
        "vizier_astrometry_compiler: {out}: {} bytes, {} series, {asset}",
        bin.len(),
        series.len()
    );

    if ci && !upload_release(NETLOC, &out) {
        eprintln!("vizier_astrometry_compiler: CDN upload returned void for {out}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "\
#   VizieR Astronomical Server vizier.cfa.harvard.edu\n\
#Table\tJ_A_A_582_A8_uranu_j:\n\
JD\tRAJ2000\te_RAJ2000\tDEJ2000\te_DEJ2000\tSat\n\
d\t\"h:m:s\"\tmas\t\"d:m:s\"\tmas\t\n\
----------------\t------------\t---\t------------\t---\t----\n\
2448782.68756100\t19 14 54.553\t 57\t-22 45 10.28\t 49\to\n\
2448782.70138981\t19 14 54.431\t 47\t-22 45 10.51\t 42\ttauo\n\
2451545.00000000\t25 00 00.000\t 50\t-22 45 10.28\t 49\to\n\
2451545.00000000\t19 14 54.553\t  0\t-22 45 10.28\t 49\to\n";

    #[test]
    fn asu_tsv_rows_become_a_roundtripping_series() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let raw = parse_asu_tsv(FIXTURE);
        assert_eq!(
            raw.len(),
            2,
            "the out-of-range RA and the zero error are refused, never defaulted"
        );
        let series = build_series("uranu_j", &raw, &lsk).expect("two measured rows build a series");
        assert_eq!(series.name, "uranu");
        assert_eq!(series.samples.len(), 2);
        assert!(series.samples[0].tdb.is_finite() && series.samples[0].tdb > 0.0);
        assert_eq!(series.samples[0].e_ra_mas, 57.0);
        assert_eq!(series.samples[0].e_dec_mas, 49.0);

        let bin = write_bin(std::slice::from_ref(&series)).expect("finite measured samples write");
        let parsed = parse_bin(&bin).expect("the AST1 bytes parse");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "uranu");
        assert!(series_match(std::slice::from_ref(&series), &parsed));
        assert_ne!(
            parsed[0].samples[0].e_ra_mas,
            parsed[0].samples[0].e_dec_mas
        );
    }

    #[test]
    fn write_bin_refuses_an_implausible_row() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let raw = parse_asu_tsv(FIXTURE);
        let series = build_series("uranu_j", &raw, &lsk).expect("two measured rows build a series");
        let mut bad = series.clone();
        bad.samples[0].e_ra_mas = 0.0;
        assert!(write_bin(std::slice::from_ref(&bad)).is_none());
    }

    #[test]
    fn body_name_strips_the_table_suffix() {
        assert_eq!(body_name("uranu_j"), "uranu");
        assert_eq!(body_name("miran_j"), "miran");
        assert_eq!(body_name("titan"), "titan");
    }

    #[test]
    fn jd_utc_folds_onto_the_tdb_axis() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let tdb = jd_utc_to_tdb(&lsk, 2448782.687561).expect("the measured JD folds onto TDB");
        assert!(tdb.is_finite() && tdb > 0.0);
    }
}
