use omegaflow::archivar::embedded_lsk;
use omegaflow::cdn::upload_release;
use omegaflow::gaia_sso::{
    parse_bin, parse_observation_csv, write_bin, GaiaBody, GAIA_SSO_TABLE, GAIA_TAP_SYNC, TNO_NAME,
};
use std::process::Command;
const CDN_RELEASE: &str = "gea.esac.esa.int";
const DEFAULT_OUT: &str = "data/gea.esac.esa.int/gaia_sso_tno.bin";

const SELECT_COLUMNS: &str = "number_mp, epoch_utc, ra, dec, ra_error_random, dec_error_random, ra_error_systematic, dec_error_systematic";

pub fn observation_adql(number_mp: u32) -> String {
    format!("SELECT {SELECT_COLUMNS} FROM {GAIA_SSO_TABLE} WHERE number_mp = {number_mp}")
}

fn tap_csv(adql: &str) -> Option<(String, Vec<u8>)> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("-m")
        .arg("180")
        .arg("-A")
        .arg("omegaflow-gaia-sso-compiler/1.0")
        .arg("-G")
        .arg(GAIA_TAP_SYNC)
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=csv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg("-o")
        .arg("-")
        .arg("-w")
        .arg("\n%{http_code}")
        .output()
        .ok()?;
    let stdout = out.stdout;
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    Some((code, stdout[..idx].to_vec()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path = DEFAULT_OUT.to_string();
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = match args.get(i + 1) {
                    Some(p) => p.clone(),
                    None => {
                        eprintln!("--out needs a path");
                        std::process::exit(1);
                    }
                };
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            "--help" | "-h" => {
                println!("usage: gaia_sso_compiler [--out <gaia_sso_tno.bin>] [--ci-mode]");
                return;
            }
            _ => {}
        }
        i += 1;
    }
    let Some(lsk) = embedded_lsk() else {
        eprintln!("gaia_sso: the embedded leap-second table reads void — the transit epochs stay unconverted");
        std::process::exit(1);
    };
    let mut all: Vec<GaiaBody> = Vec::new();
    let mut total_rows = 0usize;
    let mut total_emitted = 0usize;
    for (name, number_mp) in TNO_NAME {
        let adql = observation_adql(*number_mp);
        let Some((code, body)) = tap_csv(&adql) else {
            println!("gaia_sso {name} ({number_mp}): the TAP query did not answer (measured stall) — the harvest stays pending");
            std::process::exit(1);
        };
        if code != "200" {
            println!("gaia_sso {name} ({number_mp}): the TAP endpoint answered HTTP {code} — the harvest stays pending");
            std::process::exit(1);
        }
        let text = match std::str::from_utf8(&body) {
            Ok(t) => t,
            Err(_) => {
                println!("gaia_sso {name} ({number_mp}): the HTTP 200 body is not UTF-8 — the harvest stays pending");
                std::process::exit(1);
            }
        };
        match parse_observation_csv(text, &lsk) {
            Some((bodies, counts)) => {
                println!(
                    "gaia_sso {name}: rows {} emitted {} (epoch_void {} position_void {} sigma_void {})",
                    counts.rows,
                    counts.emitted,
                    counts.epoch_void,
                    counts.position_void,
                    counts.sigma_void
                );
                total_rows += counts.rows;
                total_emitted += counts.emitted;
                for b in bodies {
                    match all.iter_mut().find(|g| g.number_mp == b.number_mp) {
                        Some(g) => g.transits.extend(b.transits),
                        None => all.push(b),
                    }
                }
            }
            None => {
                println!(
                    "gaia_sso {name} ({number_mp}): the HTTP 200 CSV did not parse to measured transits (schema absent or rows void) — the harvest stays pending"
                );
                std::process::exit(1);
            }
        }
    }
    println!(
        "gaia_sso: bodies {} | rows {total_rows} | emitted {total_emitted}",
        all.len()
    );
    let bytes = match write_bin(&all) {
        Some(b) => b,
        None => {
            eprintln!("gaia_sso: the measured transits do not encode to the fixed-stride asset");
            std::process::exit(1);
        }
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bytes).is_err() {
        eprintln!("gaia_sso: write {} returned void", out_path);
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            let transits: usize = parsed.iter().map(|b| b.transits.len()).sum();
            println!(
                "gaia_sso: {} B -> {} ({} bodies, {transits} transits, roundtrip verified)",
                bytes.len(),
                out_path,
                parsed.len()
            );
        }
        None => {
            eprintln!("gaia_sso: roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_RELEASE, &out_path) {
        eprintln!("gaia_sso: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observation_adql_selects_the_measured_columns_for_one_number() {
        let q = observation_adql(50000);
        assert!(q.starts_with(&format!(
            "SELECT {SELECT_COLUMNS} FROM {GAIA_SSO_TABLE} WHERE number_mp = 50000"
        )));
        assert!(q.contains("ra_error_random"));
        assert!(q.contains("dec_error_systematic"));
        assert!(!q.contains("g_mag"));
    }

    #[test]
    fn table_covers_fourteen_bright_bodies() {
        assert_eq!(TNO_NAME.len(), 14);
    }
}
