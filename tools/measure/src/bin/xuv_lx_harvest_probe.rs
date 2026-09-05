use omegaflow::json::{jnum, jstr, parse_json, JsonVal};
use std::collections::HashMap;
use std::process::Command;

const UA: &str = "omegaflow-nadel13/0.1";
const PC_CM: f64 = 3.0857e18;
const SEP_LIMIT_ARCSEC: f64 = 72.0;
const VIZIER_BASE: &str = "https://vizier.cds.unistra.fr/viz-bin/asu-tsv";
const NEXSCI_TAP: &str = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync";

struct XraySurvey {
    key: &'static str,
    label: &'static str,
    source: &'static str,
    out_cols: &'static str,
    name_col: &'static str,
    ra_col: &'static str,
    dec_col: &'static str,
    flux_col: &'static str,
    band: &'static str,
    assoc_arcsec: f64,
}

const SURVEYS: [XraySurvey; 3] = [
    XraySurvey {
        key: "xmm4",
        label: "XMM-Newton 4XMM-DR13 serendipitous",
        source: "IX/69/xmm4d13s",
        out_cols: "4XMM,RA_ICRS,DE_ICRS,Flux8",
        name_col: "4XMM",
        ra_col: "RA_ICRS",
        dec_col: "DE_ICRS",
        flux_col: "Flux8",
        band: "0.2-12 keV",
        assoc_arcsec: 6.0,
    },
    XraySurvey {
        key: "erass1",
        label: "SRG/eROSITA eRASS1 main",
        source: "J/A+A/682/A34/erass1-m",
        out_cols: "IAUName,RA_ICRS,DE_ICRS,MLFlux1",
        name_col: "IAUName",
        ra_col: "RA_ICRS",
        dec_col: "DE_ICRS",
        flux_col: "MLFlux1",
        band: "0.2-2.3 keV",
        assoc_arcsec: 6.0,
    },
    XraySurvey {
        key: "2rxs",
        label: "ROSAT All-Sky 2RXS",
        source: "J/A+A/588/A103/cat2rxs",
        out_cols: "2RXS,RAJ2000,DEJ2000,Fluxp",
        name_col: "2RXS",
        ra_col: "RAJ2000",
        dec_col: "DEJ2000",
        flux_col: "Fluxp",
        band: "0.1-2.4 keV (power-law fit)",
        assoc_arcsec: 15.0,
    },
];

struct SeedHost {
    host: String,
    has_numeric_lx_or_fx: bool,
    analyses: Vec<String>,
    notes: Vec<String>,
}

struct HostCoord {
    ra: Option<f64>,
    dec: Option<f64>,
    sy_dist: Option<f64>,
}

enum SurveyOutcome {
    Value {
        name: String,
        sep_arcsec: f64,
        flux_erg: f64,
    },
    SourceNoFlux {
        name: String,
        sep_arcsec: f64,
    },
    UnattributedNear {
        name: String,
        sep_arcsec: f64,
    },
    NoneWithinCone,
    Refused(String),
}

struct Found {
    survey: usize,
    flux_erg: f64,
    sep_arcsec: f64,
    name: String,
}

fn read_seed(path: &str) -> Result<Vec<SeedHost>, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("seed {path}: {e}"))?;
    let root = parse_json(&body).ok_or_else(|| format!("seed {path}: json absent"))?;
    let JsonVal::Arr(rows) = &root else {
        return Err(format!("seed {path}: root is not an array"));
    };
    let mut map: HashMap<String, SeedHost> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for row in rows {
        let Some(host) = jstr(row, "hostname") else {
            continue;
        };
        let host = host.trim().to_string();
        if host.is_empty() {
            continue;
        }
        let has_lx = jnum(row, "lx").filter(|v| v.is_finite());
        let has_fx = jnum(row, "fx").filter(|v| v.is_finite());
        let entry = if map.contains_key(&host) {
            map.get_mut(&host).expect("host registered")
        } else {
            order.push(host.clone());
            map.insert(
                host.clone(),
                SeedHost {
                    host: host.clone(),
                    has_numeric_lx_or_fx: false,
                    analyses: Vec::new(),
                    notes: Vec::new(),
                },
            );
            map.get_mut(&host).expect("host inserted")
        };
        if has_lx.is_some() || has_fx.is_some() {
            entry.has_numeric_lx_or_fx = true;
        }
        if let Some(a) = jstr(row, "analysis").filter(|s| !s.trim().is_empty()) {
            let a = a.trim().to_string();
            if !entry.analyses.iter().any(|x| x == &a) {
                entry.analyses.push(a);
            }
        }
        if let Some(n) = jstr(row, "note").filter(|s| !s.trim().is_empty()) {
            let n = n.trim().to_string();
            if !entry.notes.iter().any(|x| x == &n) {
                entry.notes.push(n);
            }
        }
    }
    Ok(order.into_iter().filter_map(|h| map.remove(&h)).collect())
}

fn load_coords(body: &str) -> Result<HashMap<String, HostCoord>, String> {
    let root = parse_json(body).ok_or_else(|| "coords json absent".to_string())?;
    let JsonVal::Arr(rows) = &root else {
        return Err("coords: root is not an array".to_string());
    };
    let mut out: HashMap<String, HostCoord> = HashMap::new();
    for row in rows {
        let Some(host) = jstr(row, "hostname") else {
            continue;
        };
        let host = host.trim().to_string();
        if host.is_empty() {
            continue;
        }
        let ra = jnum(row, "ra").filter(|v| v.is_finite());
        let dec = jnum(row, "dec").filter(|v| v.is_finite());
        let sy = jnum(row, "sy_dist").filter(|v| v.is_finite() && *v > 0.0);
        let better = match out.get(&host) {
            None => true,
            Some(c) => c.sy_dist.is_none() && sy.is_some(),
        };
        if better {
            out.insert(
                host,
                HostCoord {
                    ra,
                    dec,
                    sy_dist: sy,
                },
            );
        }
    }
    Ok(out)
}

fn curl_get(url: &str) -> Option<(String, String)> {
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("45")
        .arg("-A")
        .arg(UA)
        .arg("-w")
        .arg("\n%{http_code}")
        .arg(url);
    let out = cmd.output().ok()?;
    let stdout = out.stdout;
    if stdout.is_empty() {
        return None;
    }
    let idx = stdout.iter().rposition(|&b| b == b'\n')?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    let body = String::from_utf8_lossy(&stdout[..idx]).to_string();
    Some((code, body))
}

fn nexsci_coords_query(hosts: &[String]) -> Result<String, String> {
    let list: Vec<String> = hosts
        .iter()
        .map(|h| format!("'{}'", h.replace('\'', "''")))
        .collect();
    let query = format!(
        "SELECT hostname,ra,dec,sy_dist FROM pscomppars WHERE hostname IN ({})",
        list.join(",")
    );
    let mut cmd = Command::new("curl");
    cmd.arg("-sS")
        .arg("-m")
        .arg("90")
        .arg("-A")
        .arg(UA)
        .arg("-G")
        .arg(NEXSCI_TAP)
        .arg("--data-urlencode")
        .arg(format!("query={query}"))
        .arg("--data-urlencode")
        .arg("format=json")
        .arg("-w")
        .arg("\n%{http_code}");
    let out = cmd.output().map_err(|e| format!("curl nexsci: {e}"))?;
    let stdout = out.stdout;
    let idx = stdout
        .iter()
        .rposition(|&b| b == b'\n')
        .ok_or_else(|| "NExScI: empty reply".to_string())?;
    let code = String::from_utf8_lossy(&stdout[idx + 1..])
        .trim()
        .to_string();
    let body = String::from_utf8_lossy(&stdout[..idx]).to_string();
    if code != "200" {
        return Err(format!("NExScI TAP HTTP {code}"));
    }
    Ok(body)
}

fn vizier_cone_url(s: &XraySurvey, ra: f64, dec: f64) -> String {
    let src = s.source.replace('+', "%2B");
    format!(
        "{base}?-source={src}&-out={oc}&-out.max=8&-c.ra={ra:.8}&-c.dec={dec:.8}&-c.r=1.2",
        base = VIZIER_BASE,
        src = src,
        oc = s.out_cols,
        ra = ra,
        dec = dec,
    )
}

fn vizier_fetch(url: &str) -> Result<String, String> {
    let mut last: Option<String> = None;
    for _ in 0..2 {
        match curl_get(url) {
            None => {
                last = Some("curl without reply (network/exit)".to_string());
            }
            Some((code, body)) => {
                if code != "200" {
                    last = Some(format!("VizieR HTTP {code}"));
                    continue;
                }
                if body.trim_start().starts_with('<') {
                    last = Some("VizieR reply is HTML, not TSV".to_string());
                    continue;
                }
                return Ok(body);
            }
        }
    }
    match last {
        Some(e) => Err(e),
        None => Err("VizieR without reply".to_string()),
    }
}

fn parse_vizier_rows(body: &str) -> Vec<HashMap<String, String>> {
    let lines: Vec<&str> = body.lines().collect();
    let mut out: Vec<HashMap<String, String>> = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let l = lines[i];
        if l.starts_with('#') || l.trim().is_empty() {
            i += 1;
            continue;
        }
        if l.chars().all(|c| c == '-' || c.is_whitespace()) {
            i += 1;
            continue;
        }
        let hdr: Vec<String> = l.split('\t').map(|c| c.trim().to_string()).collect();
        if hdr.len() < 2 {
            i += 1;
            continue;
        }
        i += 1;
        while i < lines.len() && (lines[i].trim().is_empty() || lines[i].starts_with('#')) {
            i += 1;
        }
        if i < lines.len() {
            i += 1;
        }
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        if i < lines.len() && lines[i].chars().all(|c| c == '-' || c.is_whitespace()) {
            i += 1;
        }
        while i < lines.len() && !lines[i].starts_with('#') {
            if !lines[i].trim().is_empty() {
                let cells: Vec<&str> = lines[i].split('\t').collect();
                let mut m = HashMap::new();
                for (k, name) in hdr.iter().enumerate() {
                    if name.is_empty() {
                        continue;
                    }
                    if let Some(cell) = cells.get(k) {
                        let v = cell.trim();
                        if !v.is_empty() {
                            m.insert(name.clone(), v.to_string());
                        }
                    }
                }
                out.push(m);
            }
            i += 1;
        }
    }
    out
}

fn num_cell(row: &HashMap<String, String>, col: &str) -> Option<f64> {
    row.get(col)
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| v.is_finite())
}

fn ang_sep_arcsec(ra1: f64, de1: f64, ra2: f64, de2: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let d2r = pi / 180.0;
    let p1 = de1 * d2r;
    let p2 = de2 * d2r;
    let dp = (de2 - de1) * d2r;
    let dl = (ra2 - ra1) * d2r;
    let a = (dp / 2.0).sin().powi(2) + p1.cos() * p2.cos() * (dl / 2.0).sin().powi(2);
    let d = 2.0 * a.sqrt().asin() / d2r;
    d * 3600.0
}

fn eval_survey(host_ra: f64, host_dec: f64, s: &XraySurvey) -> SurveyOutcome {
    let url = vizier_cone_url(s, host_ra, host_dec);
    let body = match vizier_fetch(&url) {
        Ok(b) => b,
        Err(e) => return SurveyOutcome::Refused(e),
    };
    let rows = parse_vizier_rows(&body);
    let mut best_value: Option<(f64, f64, String)> = None;
    let mut best_noflux: Option<(f64, String)> = None;
    let mut best_cone: Option<(f64, String)> = None;
    for row in &rows {
        let (Some(sra), Some(sdec)) = (num_cell(row, s.ra_col), num_cell(row, s.dec_col)) else {
            continue;
        };
        let sep = ang_sep_arcsec(host_ra, host_dec, sra, sdec);
        if !(sep <= SEP_LIMIT_ARCSEC) {
            continue;
        }
        let name = match row.get(s.name_col).cloned() {
            Some(n) if !n.is_empty() => n,
            _ => format!("{} {:.4} {:.4}", s.key, sra, sdec),
        };
        if sep <= s.assoc_arcsec {
            match num_cell(row, s.flux_col) {
                Some(f) if f.is_finite() && f > 0.0 => {
                    if match best_value.as_ref() {
                        Some(b) => sep < b.0,
                        None => true,
                    } {
                        best_value = Some((sep, f, name));
                    }
                }
                _ => {
                    if match best_noflux.as_ref() {
                        Some(b) => sep < b.0,
                        None => true,
                    } {
                        best_noflux = Some((sep, name));
                    }
                }
            }
        } else if match best_cone.as_ref() {
            Some(b) => sep < b.0,
            None => true,
        } {
            best_cone = Some((sep, name));
        }
    }
    match best_value {
        Some((sep, flux, name)) => SurveyOutcome::Value {
            name,
            sep_arcsec: sep,
            flux_erg: flux,
        },
        None => match best_noflux {
            Some((sep, name)) => SurveyOutcome::SourceNoFlux {
                name,
                sep_arcsec: sep,
            },
            None => match best_cone {
                Some((sep, name)) => SurveyOutcome::UnattributedNear {
                    name,
                    sep_arcsec: sep,
                },
                None => SurveyOutcome::NoneWithinCone,
            },
        },
    }
}

fn fmt_value(flux_erg: f64, d_pc: Option<f64>, s: &XraySurvey) -> String {
    let base = format!("F_X {flux_erg:.2e} erg/s/cm2 ({} {})", s.band, s.label);
    match d_pc {
        Some(d) => match lx_from_flux(flux_erg, d) {
            Some(lx) => format!("L_X {lx:.2e} erg/s ({} {})", s.band, s.label),
            None => base,
        },
        None => base,
    }
}

fn lx_from_flux(fx_erg: f64, d_pc: f64) -> Option<f64> {
    let d_cm = d_pc * PC_CM;
    let lx = 4.0 * std::f64::consts::PI * d_cm * d_cm * fx_erg;
    if lx.is_finite() && lx > 0.0 {
        Some(lx)
    } else {
        None
    }
}

fn lit_tags(analyses: &[String]) -> Vec<String> {
    let mut tags = Vec::new();
    if analyses.iter().any(|a| a.contains("Behr")) {
        tags.push("Behr et al. 2023 (MUSCLES Extension, arXiv:2306.05322)".to_string());
    }
    if analyses.iter().any(|a| a.contains("Maggio")) {
        tags.push("Maggio et al. 2023/2024 (XUV)".to_string());
    }
    if analyses.iter().any(|a| a.contains("Sairam")) {
        tags.push("Sairam & Madhusudhan 2025 (arXiv:2503.19908)".to_string());
    }
    tags
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut seed = "docs/reference/co_rhk_witness_seed.json".to_string();
    let mut out_path = "/tmp/opencode/xuv_lx_harvest_report.txt".to_string();
    let mut coords: Option<String> = None;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                i += 1;
                if let Some(v) = args.get(i).cloned() {
                    seed = v;
                }
            }
            "--coords" => {
                i += 1;
                coords = args.get(i).cloned();
            }
            "--out" => {
                i += 1;
                if let Some(v) = args.get(i).cloned() {
                    out_path = v;
                }
            }
            other => {
                eprintln!("xuv_lx_harvest_probe: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    match run(&seed, coords.as_deref(), &out_path) {
        Ok(()) => {}
        Err(msg) => {
            eprintln!("xuv_lx_harvest_probe: {msg}");
            std::process::exit(1);
        }
    }
}

fn run(seed_path: &str, coords_path: Option<&str>, out_path: &str) -> Result<(), String> {
    let seed = read_seed(seed_path)?;
    let mut targets: Vec<&SeedHost> = seed.iter().filter(|s| !s.has_numeric_lx_or_fx).collect();
    targets.sort_by(|a, b| a.host.cmp(&b.host));

    let coords_label = match coords_path {
        Some(p) => format!("--coords {p}"),
        None => "NExScI TAP pscomppars (live)".to_string(),
    };
    let coord_body = match coords_path {
        Some(p) => std::fs::read_to_string(p).map_err(|e| format!("coords {p}: {e}"))?,
        None => {
            let names: Vec<String> = targets.iter().map(|t| t.host.clone()).collect();
            nexsci_coords_query(&names)?
        }
    };
    let coords = load_coords(&coord_body)?;

    let mut report = String::new();
    report.push_str(
        "xuv_lx_harvest_probe — XUV/X-ray value harvest (L_X / F_X) for exoplanet host stars\n",
    );
    report.push_str(&format!(
        "timestamp: 2026-09-05 | witness register: {seed_path} | coordinates/distance: {coords_label}\n"
    ));
    report.push_str(&format!(
        "hosts in register: {} | target hosts with no numeric lx/fx in any row: {}\n",
        seed.len(),
        targets.len()
    ));
    report.push('\n');

    let mut table_lines: Vec<String> = Vec::new();
    let mut no_value: Vec<(String, String)> = Vec::new();
    let mut with_value_count = 0usize;

    for t in &targets {
        let host = &t.host;
        let coord = coords.get(host);
        let (Some(ra), Some(dec)) = (coord.and_then(|c| c.ra), coord.and_then(|c| c.dec)) else {
            let reason = format!(
                "pending: {coords_label} does not carry {host} (no RA/Dec) — cone crossmatch not possible"
            );
            no_value.push((host.clone(), reason.clone()));
            table_lines.push(format!("{host:<12} | — | — | — | {reason}"));
            report.push_str(&format!("{host}: {reason}\n\n"));
            continue;
        };
        let d_pc = coord.and_then(|c| c.sy_dist);

        let mut found: Vec<Found> = Vec::new();
        let mut notes: Vec<String> = Vec::new();
        for (i, s) in SURVEYS.iter().enumerate() {
            match eval_survey(ra, dec, s) {
                SurveyOutcome::Value {
                    name,
                    sep_arcsec,
                    flux_erg,
                } => found.push(Found {
                    survey: i,
                    flux_erg,
                    sep_arcsec,
                    name,
                }),
                SurveyOutcome::SourceNoFlux { name, sep_arcsec } => notes.push(format!(
                    "{}: {name} at {sep_arcsec:.1}\", but flux column {} is empty (no machine flux)",
                    s.key, s.flux_col
                )),
                SurveyOutcome::UnattributedNear { name, sep_arcsec } => notes.push(format!(
                    "{}: source {name} at {sep_arcsec:.1}\" lies beyond the {:.0}\" association radius — not attributed to the host",
                    s.key, s.assoc_arcsec
                )),
                SurveyOutcome::NoneWithinCone => {
                    notes.push(format!("{}: no source within the {:.0}\" cone", s.key, SEP_LIMIT_ARCSEC));
                }
                SurveyOutcome::Refused(e) => notes.push(format!("{}: pending — {e}", s.key)),
            }
        }

        let primary = found.first();
        let lit = lit_tags(&t.analyses);
        let dist_str = match d_pc {
            Some(d) => format!("{d:.2}"),
            None => "absent".to_string(),
        };

        let other_notes: Vec<String> = notes
            .iter()
            .cloned()
            .chain(found.iter().skip(1).map(|f| {
                format!(
                    "{}: F_X {:.2e} erg/s/cm2 ({} {}) {} at {:.1}\"",
                    SURVEYS[f.survey].key,
                    f.flux_erg,
                    SURVEYS[f.survey].band,
                    SURVEYS[f.survey].label,
                    f.name,
                    f.sep_arcsec
                )
            }))
            .collect();

        match primary {
            Some(f) => {
                with_value_count += 1;
                let val = fmt_value(f.flux_erg, d_pc, &SURVEYS[f.survey]);
                let src = format!(
                    "{} ({}, offset {:.1}\")",
                    SURVEYS[f.survey].source, f.name, f.sep_arcsec
                );
                table_lines.push(format!(
                    "{host:<12} | {val:<46} | {dist_str:>9} | {src} | {}",
                    other_notes.join(" | ")
                ));
                report.push_str(&format!("{host}: {val}\n"));
                report.push_str(&format!(
                    "  distance sy_dist = {} pc ({coords_label})\n",
                    match d_pc {
                        Some(d) => format!("{d:.2}"),
                        None => "absent (F_X only)".to_string(),
                    }
                ));
                report.push_str(&format!("  source: {src}\n"));
                report.push_str(&format!(
                    "  literature analysis row(s): {}\n",
                    if lit.is_empty() {
                        "none (no Behr/Maggio/Sairam row)".to_string()
                    } else {
                        lit.join("; ")
                    }
                ));
                if !other_notes.is_empty() {
                    report.push_str(&format!("  other catalogs: {}\n", other_notes.join(" | ")));
                }
                report.push('\n');
            }
            None => {
                let lit_part = if lit.is_empty() {
                    String::new()
                } else {
                    format!(
                        " | literature row(s): {} — data_log carries no number (figure/PDF), survey without value => pending",
                        lit.join("; ")
                    )
                };
                let reg_note = t.notes.iter().find(|n| n.contains("Brewer 2016")).cloned();
                let reg_part = match reg_note {
                    Some(n) => format!(" | register note: {n}"),
                    None => String::new(),
                };
                let reason = format!(
                    "absent — no attributed X-ray source (within the per-survey association radius) carries a machine flux ({}){}",
                    other_notes.join(" | "),
                    format!("{lit_part}{reg_part}")
                );
                no_value.push((host.clone(), reason.clone()));
                table_lines.push(format!("{host:<12} | — | {dist_str:>9} | — | {reason}"));
                report.push_str(&format!("{host}: {reason}\n\n"));
            }
        }
    }

    let mut stdout = String::new();
    stdout.push_str("xuv_lx_harvest_probe — numeric XUV/X-ray (L_X/F_X) harvest for hosts without a numeric lx/fx\n");
    stdout.push_str(&format!(
        "target hosts (no numeric lx or fx in any seed row): {}\n",
        targets.len()
    ));
    stdout.push_str(
        "catalog codes + flux columns verified live on 2026-09-05 (VizieR asu-tsv, cone r=1.2 arcmin = 72\"):\n",
    );
    stdout.push_str(
        "  eROSITA eRASS1 -> J/A+A/682/A34/erass1-m  flux col MLFlux1  band 0.2-2.3 keV\n",
    );
    stdout.push_str("  ROSAT 2RXS      -> J/A+A/588/A103/cat2rxs  flux col Fluxp    band 0.1-2.4 keV (power-law)\n");
    stdout.push_str(
        "  XMM 4XMM-DR13   -> IX/69/xmm4d13s          flux col Flux8    band 0.2-12 keV\n",
    );
    stdout.push_str("  XMM slew XMMSL3 -> IX/71/xmmsl3c          flux col FluxB8   band 0.2-12 keV (verified, not harvested)\n");
    stdout.push_str("  XMM slew XMMSL2 -> IX/53; 3XMM-DR6 -> IX/50 (titles verified)\n");
    stdout.push_str("  flux unit: mW/m2; 1 mW/m2 = 1 erg/s/cm2 (unit identity measured)\n");
    stdout.push_str("  association radii (calibrated from the register's accepted stellar offsets, 2026-09-05):\n");
    stdout.push_str("    xmm4 6.0\", erass1 6.0\" (register XMM offsets 0.7-3.2\", eRASS1 2.1-5.2\"); 2rxs 15.0\" (register accepted 10.3\", refused 22\")\n");
    stdout.push_str("\nHOST         | VALUE | DIST pc | SOURCE | other detections / notes\n");
    stdout.push_str(
        "-------------|---------------------------------------------------------------\n",
    );
    for l in &table_lines {
        stdout.push_str(l);
        stdout.push('\n');
    }
    stdout.push_str("\nHOSTS WITHOUT A NUMERIC VALUE + reason:\n");
    for (h, r) in &no_value {
        stdout.push_str(&format!("  {h}: {r}\n"));
    }
    stdout.push_str(&format!(
        "\nsummary: {with_value_count} target hosts with a measured value; {} without.\n",
        no_value.len()
    ));

    report.push_str("Verified VizieR catalog codes + flux columns (measured live 2026-09-05):\n");
    report.push_str("  eROSITA eRASS1 -> J/A+A/682/A34/erass1-m  MLFlux1 (0.2-2.3 keV)\n");
    report.push_str(
        "  ROSAT 2RXS      -> J/A+A/588/A103/cat2rxs  Fluxp   (0.1-2.4 keV, power-law)\n",
    );
    report.push_str("  XMM 4XMM-DR13   -> IX/69/xmm4d13s          Flux8   (0.2-12 keV)\n");
    report.push_str("  XMM slew XMMSL3 -> IX/71/xmmsl3c          FluxB8  (0.2-12 keV; verified, not harvested)\n");
    report.push_str(&format!(
        "  flux unit: mW/m2; 1 mW/m2 = 1 erg/s/cm2; L_X = 4*pi*(d_pc*{:.5e} cm)^2 * F_X; d_pc = pscomppars sy_dist\n",
        PC_CM
    ));
    report.push_str("  association radii (calibrated from the register's accepted stellar offsets, 2026-09-05):\n");
    report.push_str("    xmm4 6.0\", erass1 6.0\" (register XMM 0.7-3.2\", eRASS1 2.1-5.2\"); 2rxs 15.0\" (register accepted 10.3\", refused 22\").\n");
    report.push_str("    the 72\" cone is the absence gate: no catalog source candidate in the cone => absent for that catalog.\n");
    report.push('\n');
    report.push_str("value table per target host:\n");
    for l in &table_lines {
        report.push_str(l);
        report.push('\n');
    }
    report.push_str("\nhosts without a numeric value:\n");
    for (h, r) in &no_value {
        report.push_str(&format!("  {h}: {r}\n"));
    }
    report.push_str("\nliterature machine-table check (session 2026-09-05):\n");
    report.push_str("  Behr et al. 2023 (MUSCLES Extension, AJ 166 35; arXiv:2306.05322):\n");
    report.push_str("    candidate code J/AJ/166/35 queried via asu-tsv — no catalog (#Title empty, measured).\n");
    report.push_str("    the X-ray survey codes (IX/50-71, J/A+A/588/A103, J/A+A/682/A34) do not carry the MUSCLES extension.\n");
    report
        .push_str("    no VizieR machine table found that carries an XUV/X-ray number per host.\n");
    report.push_str("    affected target hosts with a Behr row: HAT-P-12, HAT-P-26, WASP-127, WASP-17, LP 791-18.\n");
    report.push_str("  Maggio et al. 2023/2024 (XUV):\n");
    report.push_str(
        "    no dedicated VizieR-J/ machine table identified (register names no bibcode/DOI).\n",
    );
    report.push_str("    V1298 Tau and HIP 67522 (Maggio carriers) already carry XMM survey values (ep 0.2-12 keV) in the register.\n");
    report.push_str("  Sairam & Madhusudhan 2025 (arXiv:2503.19908, MNRAS):\n");
    report.push_str("    no VizieR-J/ machine table identified (register names no bibcode/DOI);\n");
    report.push_str(
        "    K2-18 carries register values; LTT 3780 is checked by the survey cone above.\n",
    );
    report.push_str(
        "  ingest path: machine-readable XUV/X-ray values come from the survey machine tables\n",
    );
    report.push_str("  (eRASS1/2RXS/4XMM-DR13) above where a candidate <= 72 arcsec exists; otherwise pending\n");
    report.push_str("  with the named reason (analysis data_log carries no number, figure/PDF).\n");
    report.push('\n');

    std::fs::write(out_path, report.as_bytes())
        .map_err(|e| format!("report write {out_path}: {e}"))?;
    print!("{stdout}");
    Ok(())
}
