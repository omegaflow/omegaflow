use omegaflow::cdn::upload_asset;
use omegaflow::equilibrium::{AU_M, SUN_RADIUS_M, teq};
use omegaflow::json::{JsonVal, jnum, jpath_val, jstr, parse_json};
use omegaflow::jwst::{JwstSpectrum, parse_jwst_bin};
use omegaflow::jwst_equilibrium::{
    EQUILIBRIUM_NSPECIES, EquilibriumRecord, parse_equilibrium_bin, write_equilibrium_bin,
};
use omegaflow::thermochem::{P0_PA, equilibrium_concentrations, solar};
use std::collections::{HashMap, HashSet};
use std::process::Command;

fn state_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("OMEGAFLOW_STATE") {
        return std::path::PathBuf::from(dir);
    }
    std::path::PathBuf::from(".")
}

fn mast_token() -> String {
    if let Ok(t) = std::env::var("MAST_TOKEN") {
        if !t.is_empty() {
            return t;
        }
    }
    std::fs::read_to_string(state_dir().join(".secrets.local"))
        .ok()
        .and_then(|body| {
            body.lines().find_map(|line| {
                let (k, v) = line.split_once('=')?;
                (k.trim() == "MAST_TOKEN" && !v.trim().is_empty()).then(|| v.trim().to_string())
            })
        })
        .unwrap_or_default()
}

fn curl_json(url: &str, data: &[(&str, String)], token: &str) -> Option<String> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-L")
        .arg("--retry")
        .arg("3")
        .arg("--retry-delay")
        .arg("2")
        .arg("--retry-all-errors")
        .arg("--max-time")
        .arg("300")
        .arg("-G");
    for (k, v) in data {
        cmd.arg("--data-urlencode").arg(format!("{}={}", k, v));
    }
    if !token.is_empty() {
        cmd.arg("-H")
            .arg(format!("Authorization: Bearer {}", token));
    }
    cmd.arg(url);
    let out = cmd.output().ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "curl {}: {}",
            url,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn tap_single_planet_hosts(token: &str) -> (HashMap<String, (f64, f64, f64)>, HashSet<String>) {
    let adql = "SELECT pl_name,hostname,st_teff,st_rad,pl_orbsmax FROM pscomppars WHERE pl_tranmid IS NOT NULL AND st_teff IS NOT NULL AND st_rad IS NOT NULL AND pl_orbsmax IS NOT NULL";
    let body = match curl_json(
        "https://exoplanetarchive.ipac.caltech.edu/TAP/sync",
        &[
            ("REQUEST", "doQuery".to_string()),
            ("LANG", "ADQL".to_string()),
            ("FORMAT", "json".to_string()),
            ("QUERY", adql.to_string()),
        ],
        token,
    ) {
        Some(b) => b,
        None => return (HashMap::new(), HashSet::new()),
    };
    let Some(root) = parse_json(&body) else {
        eprintln!("tap: json absent");
        return (HashMap::new(), HashSet::new());
    };
    let rows: &[JsonVal] = match jpath_val(&root, "data") {
        Some(JsonVal::Arr(a)) => a,
        _ => match &root {
            JsonVal::Arr(a) => a,
            _ => {
                eprintln!("tap: data array absent");
                return (HashMap::new(), HashSet::new());
            }
        },
    };
    let mut host_planets: HashMap<String, HashSet<String>> = HashMap::new();
    let mut first_row: HashMap<(String, String), (f64, f64, f64)> = HashMap::new();
    for row in rows {
        let (Some(host), Some(pl)) = (jstr(row, "hostname"), jstr(row, "pl_name")) else {
            continue;
        };
        let (Some(teff), Some(rad), Some(orb)) = (
            jnum(row, "st_teff"),
            jnum(row, "st_rad"),
            jnum(row, "pl_orbsmax"),
        ) else {
            continue;
        };
        if !teff.is_finite()
            || teff <= 0.0
            || !rad.is_finite()
            || rad <= 0.0
            || !orb.is_finite()
            || orb <= 0.0
        {
            continue;
        }
        host_planets
            .entry(host.clone())
            .or_default()
            .insert(pl.clone());
        first_row.entry((host, pl)).or_insert((teff, rad, orb));
    }
    let mut single = HashMap::new();
    let mut multi = HashSet::new();
    for (host, planets) in host_planets {
        if planets.len() == 1 {
            let pl = planets.into_iter().next().unwrap();
            if let Some(params) = first_row.remove(&(host.clone(), pl)) {
                single.insert(host, params);
            }
        } else {
            multi.insert(host);
        }
    }
    (single, multi)
}

#[derive(Default)]
struct SkipCounts {
    multi_planet: usize,
    no_pscomppars: usize,
    out_of_domain: usize,
}

fn compile_records(
    spectra: &[JwstSpectrum],
    single: &HashMap<String, (f64, f64, f64)>,
    multi: &HashSet<String>,
) -> (Vec<EquilibriumRecord>, SkipCounts) {
    let mut records = Vec::new();
    let mut skips = SkipCounts::default();
    for spec in spectra {
        let Some(&(teff, rad, orb)) = single.get(&spec.host) else {
            if multi.contains(&spec.host) {
                skips.multi_planet += 1;
            } else {
                skips.no_pscomppars += 1;
            }
            continue;
        };
        let Some(t_eq) = teq(teff, rad * SUN_RADIUS_M, orb * AU_M, 0.0) else {
            skips.out_of_domain += 1;
            continue;
        };
        let Some(rho) = equilibrium_concentrations(t_eq, P0_PA, solar()) else {
            skips.out_of_domain += 1;
            continue;
        };
        let mut x = [0.0f64; EQUILIBRIUM_NSPECIES];
        for (slot, v) in x.iter_mut().zip(rho.iter()) {
            *slot = *v;
        }
        records.push(EquilibriumRecord {
            host: spec.host.clone(),
            obs_id: spec.obs_id.clone(),
            teq: t_eq,
            x,
        });
    }
    (records, skips)
}

fn write_atomic(path: &str, bytes: &[u8]) -> bool {
    let tmp = format!("{}.tmp", path);
    if std::fs::write(&tmp, bytes).is_err() {
        return false;
    }
    if let Ok(f) = std::fs::File::open(&tmp) {
        let _ = f.sync_all();
    }
    if std::fs::rename(&tmp, path).is_err() {
        let _ = std::fs::remove_file(&tmp);
        return false;
    }
    true
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = "jwst_spectra.bin".to_string();
    let mut output = "jwst_equilibrium.bin".to_string();
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--in" => {
                input = args.get(i + 1).cloned().unwrap_or(input);
                i += 1;
            }
            "--out" => {
                output = args.get(i + 1).cloned().unwrap_or(output);
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {
                eprintln!(
                    "usage: jwst_equilibrium_compiler [--in <jwst_spectra.bin>] [--out <jwst_equilibrium.bin>] [--ci-mode]"
                );
                return;
            }
        }
        i += 1;
    }
    let bytes = match std::fs::read(&input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {}: {} — the input asset stays unread", input, e);
            return;
        }
    };
    let Some(spectra) = parse_jwst_bin(&bytes) else {
        eprintln!(
            "{}: bin reads void — {} B carry no JWS1 contract",
            input,
            bytes.len()
        );
        return;
    };
    eprintln!("{}: {} spectra", input, spectra.len());
    let (single, multi) = tap_single_planet_hosts(&mast_token());
    eprintln!(
        "pscomppars: {} single-transit-planet hosts, {} multi-planet hosts",
        single.len(),
        multi.len()
    );
    let (records, skips) = compile_records(&spectra, &single, &multi);
    eprintln!(
        "compiled {} records; skipped {} multi-planet, {} without pscomppars row, {} out of domain",
        records.len(),
        skips.multi_planet,
        skips.no_pscomppars,
        skips.out_of_domain
    );
    let Some(out_bytes) = write_equilibrium_bin(&records) else {
        eprintln!("write: the bin stays unwritten (0 honored)");
        return;
    };
    if !write_atomic(&output, &out_bytes) {
        eprintln!("write {}: void — the bin stays unwritten", output);
        return;
    }
    match parse_equilibrium_bin(&out_bytes) {
        Some(parsed) => {
            eprintln!(
                "{}: {} records, {} B — roundtrip parses",
                output,
                parsed.len(),
                out_bytes.len()
            );
        }
        None => {
            eprintln!(
                "{}: roundtrip parse void — the bin stays unverified",
                output
            );
            return;
        }
    }
    if ci_mode && !upload_asset(&output) {
        eprintln!("upload: {} did not reach the CDN", output);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::jwst::write_jwst_bin;

    fn synthetic_spectra() -> Vec<JwstSpectrum> {
        vec![
            JwstSpectrum {
                ra_deg: 217.32673,
                dec_deg: -3.4445,
                plx_mas: 4.6,
                epoch_tdb: 8.4e8,
                host: "WASP-39".to_string(),
                obs_id: "jw01366-o001_t001_niriss".to_string(),
                bins: vec![(2.5e14, 1.0e13, 4.2e-24)],
            },
            JwstSpectrum {
                ra_deg: 343.0,
                dec_deg: -22.0,
                plx_mas: 3.9,
                epoch_tdb: 8.4e8,
                host: "TRAPPIST-1".to_string(),
                obs_id: "jw00001".to_string(),
                bins: vec![(2.5e14, 1.0e13, 4.2e-24)],
            },
            JwstSpectrum {
                ra_deg: 0.0,
                dec_deg: 0.0,
                plx_mas: 1.0,
                epoch_tdb: 8.4e8,
                host: "UNKNOWN-HOST".to_string(),
                obs_id: "jw00002".to_string(),
                bins: vec![(2.5e14, 1.0e13, 4.2e-24)],
            },
        ]
    }

    #[test]
    fn compiler_feeds_a_synthetic_jws1_without_network() {
        let bytes = write_jwst_bin(&synthetic_spectra()).unwrap();
        let spectra = parse_jwst_bin(&bytes).unwrap();
        let mut single = HashMap::new();
        single.insert("WASP-39".to_string(), (5485.0, 0.895, 0.0486));
        let mut multi = HashSet::new();
        multi.insert("TRAPPIST-1".to_string());
        let (records, skips) = compile_records(&spectra, &single, &multi);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].host, "WASP-39");
        assert_eq!(
            records[0].teq,
            teq(5485.0, 0.895 * SUN_RADIUS_M, 0.0486 * AU_M, 0.0).unwrap()
        );
        assert!(records[0].x.iter().all(|v| v.is_finite() && *v > 0.0));
        assert_eq!(skips.multi_planet, 1);
        assert_eq!(skips.no_pscomppars, 1);
        assert_eq!(skips.out_of_domain, 0);
        let out = write_equilibrium_bin(&records).unwrap();
        assert_eq!(parse_equilibrium_bin(&out).unwrap().len(), 1);
    }

    #[test]
    fn cold_host_leaves_the_model_domain() {
        let spectra = vec![JwstSpectrum {
            ra_deg: 0.0,
            dec_deg: 0.0,
            plx_mas: 1.0,
            epoch_tdb: 8.4e8,
            host: "COLD".to_string(),
            obs_id: "jw00003".to_string(),
            bins: vec![(2.5e14, 1.0e13, 4.2e-24)],
        }];
        let mut single = HashMap::new();
        single.insert("COLD".to_string(), (3000.0, 0.2, 0.5));
        let multi = HashSet::new();
        let (records, skips) = compile_records(&spectra, &single, &multi);
        assert!(records.is_empty());
        assert_eq!(skips.out_of_domain, 1);
    }
}
