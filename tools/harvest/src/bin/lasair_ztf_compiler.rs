use omegaflow::archivar::{LeapSeconds, embedded_lsk, load_env};
use omegaflow::cdn::upload_release;
use omegaflow::json::{JsonVal, jnum, jstr, parse_json};
use omegaflow::skydirection::{SkyBandSeries, SkyDirection, SkySample, parse_bin, write_bin};
use std::collections::HashMap;
use std::process::Command;

const UA: &str = "omegaflow-lasair-ztf-compiler/1.0";
const NETLOC: &str = "api.lasair.lsst.ac.uk";
const LAS_QUERY: &str = "https://api.lasair.lsst.ac.uk/api/query/";
const LAS_LIMIT: usize = 1000;
const DEFAULT_SELECTED: &str = "objectId,ramean,decmean,gmag,jdmin,jdmax";
const DEFAULT_TABLES: &str = "objects";
const DEFAULT_CONDITIONS: &str = "jdmax>0";

fn curl_post_json(url: &str, token: &str, body: &str) -> Option<(String, Vec<u8>)> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("120")
        .arg("-A")
        .arg(UA)
        .arg("-H")
        .arg(format!("Authorization: Token {token}"))
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(body)
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url)
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn lasair_token(env: &HashMap<String, String>) -> Option<String> {
    for key in ["LASAIR_LSST_TOKEN", "LASAIR_TOKEN"] {
        if let Some(t) = env.get(key) {
            if !t.is_empty() {
                return Some(t.clone());
            }
        }
    }
    None
}

fn ra_dec_plausible(ra: f64, dec: f64) -> bool {
    ra.is_finite() && dec.is_finite() && (0.0..360.0).contains(&ra) && (-90.0..=90.0).contains(&dec)
}

fn jd_utc_to_tdb(lsk: &LeapSeconds, jd: f64) -> Option<f64> {
    lsk.unix_to_tdb((jd - 2440587.5) * 86400.0)
}

fn pick_str(row: &JsonVal, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| jstr(row, k))
}

fn pick_num(row: &JsonVal, keys: &[&str]) -> Option<f64> {
    keys.iter().find_map(|k| jnum(row, k))
}

fn response_rows(root: &JsonVal) -> Option<&Vec<JsonVal>> {
    match root {
        JsonVal::Arr(rows) => Some(rows),
        JsonVal::Obj(map) => match map.get("objects").or_else(|| map.get("data")) {
            Some(JsonVal::Arr(rows)) => Some(rows),
            _ => None,
        },
        _ => None,
    }
}

fn parse_rows(lsk: &LeapSeconds, rows: &[JsonVal]) -> Vec<SkyDirection> {
    let mut out: Vec<SkyDirection> = Vec::new();
    let mut refused = 0usize;
    let mut anchored_g = 0usize;
    let mut multi_detection_g = 0usize;
    let mut epoch_void_g = 0usize;
    for r in rows {
        let (Some(name), Some(ra), Some(dec)) = (
            pick_str(r, &["objectId", "diaObjectId"]),
            pick_num(r, &["ramean", "ra"]),
            pick_num(r, &["decmean", "decl", "dec"]),
        ) else {
            refused += 1;
            continue;
        };
        if !ra_dec_plausible(ra, dec) {
            refused += 1;
            continue;
        }
        let mut d = SkyDirection {
            name,
            ra_deg: ra,
            dec_deg: dec,
            sigma_arcsec: None,
            bands: Vec::new(),
            flux_bands: Vec::new(),
            distance: None,
            redshift: None,
        };
        match (
            pick_num(r, &["gmag"]),
            pick_num(r, &["jdmin"]),
            pick_num(r, &["jdmax"]),
        ) {
            (Some(mag), Some(jd_min), Some(jd_max))
                if mag.is_finite() && jd_min.is_finite() && jd_max.is_finite() =>
            {
                if jd_min == jd_max {
                    match jd_utc_to_tdb(lsk, jd_max) {
                        Some(tdb) => {
                            d.bands.push(SkyBandSeries {
                                band: Some("g".to_string()),
                                samples: vec![SkySample { tdb, mag }],
                            });
                            anchored_g += 1;
                        }
                        None => epoch_void_g += 1,
                    }
                } else {
                    multi_detection_g += 1;
                }
            }
            _ => {}
        }
        out.push(d);
    }
    let n = out.len();
    println!(
        "lasair_ztf: {n} object row(s) held as directions; {anchored_g} carry the delivered gmag anchored to a single-detection epoch (jdmin==jdmax); {multi_detection_g} carry a g magnitude without an anchorable epoch (multi-detection object, the g-band epoch is not delivered); {epoch_void_g} carry a g magnitude whose JD does not fold onto the TDB clock (outside the leap table) — those magnitudes stay unheld (0 honored); {refused} row(s) refused (no id/position or out of the ICRS gate)"
    );
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out_path: Option<String> = None;
    let mut ci = false;
    let mut selected = DEFAULT_SELECTED.to_string();
    let mut tables = DEFAULT_TABLES.to_string();
    let mut conditions = DEFAULT_CONDITIONS.to_string();
    let mut limit = LAS_LIMIT;
    let mut offset = 0usize;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci = true,
            "--selected" => {
                if let Some(v) = args.get(i + 1) {
                    selected = v.clone();
                }
                i += 1;
            }
            "--tables" => {
                if let Some(v) = args.get(i + 1) {
                    tables = v.clone();
                }
                i += 1;
            }
            "--conditions" => {
                if let Some(v) = args.get(i + 1) {
                    conditions = v.clone();
                }
                i += 1;
            }
            "--limit" => {
                let v = args.get(i + 1).and_then(|s| s.parse::<usize>().ok());
                match v {
                    Some(n) if n > 0 => limit = n,
                    _ => println!(
                        "lasair_ztf: --limit carries no positive count — the default {LAS_LIMIT} stands"
                    ),
                }
                i += 1;
            }
            "--offset" => {
                let v = args.get(i + 1).and_then(|s| s.parse::<usize>().ok());
                match v {
                    Some(n) => offset = n,
                    None => {
                        println!("lasair_ztf: --offset carries no count — the window starts at 0")
                    }
                }
                i += 1;
            }
            other => {
                println!(
                    "lasair_ztf_compiler: unknown argument {other} — refused. usage:\n  \
                     --out <lasair_ztf.bin> [--selected <cols>] [--tables <table>] [--conditions <expr>] [--limit <N>] [--offset <N>] [--ci-mode]"
                );
                return;
            }
        }
        i += 1;
    }
    let Some(out) = out_path else {
        println!("lasair_ztf_compiler: --out absent — the asset path is never silent");
        return;
    };
    if tables.is_empty() {
        println!(
            "lasair_ztf_compiler: --tables is empty — the query is refused (the API requires a non-empty table)"
        );
        return;
    }
    let env = load_env();
    let Some(token) = lasair_token(&env) else {
        println!(
            "lasair_ztf_compiler: LASAIR_LSST_TOKEN / LASAIR_TOKEN absent (env or .secrets.local) — the query stays unrun"
        );
        return;
    };
    let payload = format!(
        "{{\"selected\":\"{selected}\",\"tables\":\"{tables}\",\"conditions\":\"{conditions}\",\"limit\":{limit},\"offset\":{offset}}}"
    );
    let Some((code, body)) = curl_post_json(LAS_QUERY, &token, &payload) else {
        println!(
            "lasair_ztf_compiler: {LAS_QUERY} did not answer (measured stall) — the query stays pending"
        );
        return;
    };
    if code != "200" {
        println!(
            "lasair_ztf_compiler: {LAS_QUERY} answered HTTP {code} — the query stays pending (an anonymous read answers 401)"
        );
        return;
    }
    let Ok(text) = std::str::from_utf8(&body) else {
        println!("lasair_ztf_compiler: the query body is not UTF-8 — the parser stays pending");
        return;
    };
    let Some(root) = parse_json(text) else {
        println!("lasair_ztf_compiler: the query body is not JSON — the parser stays pending");
        return;
    };
    let Some(rows) = response_rows(&root) else {
        println!(
            "lasair_ztf_compiler: the query body carries no row array — the parser stays pending"
        );
        return;
    };
    let Some(lsk) = embedded_lsk() else {
        println!(
            "lasair_ztf_compiler: the embedded naif0012.tls leap table is absent — no gmag epoch folds to the TDB clock; the harvest stays unrun (0 honored, pending)"
        );
        return;
    };
    let directions = parse_rows(&lsk, rows);
    if directions.is_empty() {
        println!(
            "lasair_ztf_compiler: no object row was held — the asset stays unwritten (0 honored)"
        );
        return;
    }
    let Some(bytes) = write_bin(&directions) else {
        println!(
            "lasair_ztf_compiler: a held direction is not finite or not serializable — the asset stays unwritten (0 honored)"
        );
        return;
    };
    match parse_bin(&bytes) {
        Some(parsed) if parsed.len() == directions.len() => {
            println!(
                "lasair_ztf_compiler: {out} holds {} direction record(s), {} bytes — the roundtrip reads back",
                parsed.len(),
                bytes.len()
            );
        }
        _ => {
            println!(
                "lasair_ztf_compiler: the roundtrip does not read back — the asset stays unwritten"
            );
            return;
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        println!("lasair_ztf_compiler: write {out} returned void — the asset stays unwritten");
        return;
    }
    if ci && !upload_release(NETLOC, &out) {
        println!(
            "lasair_ztf_compiler: {out} did not reach the CDN — the local asset stands, the manifest is pending"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_icrs_gate_holds_measured_directions_and_refuses_the_rest() {
        assert!(ra_dec_plausible(10.0, -47.0));
        assert!(!ra_dec_plausible(0.0, 91.0));
        assert!(!ra_dec_plausible(f64::INFINITY, 0.0));
    }

    #[test]
    fn a_row_array_reads_back_through_the_commit_writer() {
        let lsk = embedded_lsk().expect("the embedded naif0012 table is program identity");
        let root = parse_json(
            r#"[{"objectId":"ZTF21abxxjrh","ramean":37.5,"decmean":9.25,"gmag":19.4,"jdmin":2459461.4,"jdmax":2459461.4}]"#,
        )
        .expect("the fixture parses");
        let rows = response_rows(&root).expect("the fixture carries a row array");
        let out = parse_rows(&lsk, rows);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "ZTF21abxxjrh");
        assert_eq!(out[0].ra_deg, 37.5);
        assert_eq!(out[0].dec_deg, 9.25);
        assert_eq!(out[0].bands.len(), 1);
        assert_eq!(out[0].bands[0].samples.len(), 1);
    }
}
