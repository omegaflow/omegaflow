use omegaflow::equilibrium::{teq, AU_M, SUN_RADIUS_M};
use omegaflow::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::thermochem::{
    equilibrium_composition_condensed, equilibrium_composition_sulfur, sulfur_gas_names, COOL_T_MIN,
    P0_PA,
};
use std::collections::HashMap;

const SULFUR_T_MIN: f64 = 500.0;
const MODEL_T_MAX: f64 = 3000.0;
const DEFAULT_FLOOR: f64 = 1.0e-6;

struct Detection {
    host: String,
    pl_name: Option<String>,
    species: String,
}

struct PlanetRow {
    pl_name: String,
    pl_eqt: Option<f64>,
    teff: f64,
    rad_solar: f64,
    orbsmax_au: f64,
}

fn read_detections(path: &str) -> Result<Vec<Detection>, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("seed {path}: {e}"))?;
    let root = parse_json(&body).ok_or_else(|| format!("seed {path}: json absent"))?;
    let JsonVal::Arr(rows) = &root else {
        return Err(format!("seed {path}: root is not an array"));
    };
    let mut out = Vec::new();
    for row in rows {
        let (Some(host), Some(species)) = (jstr(row, "host"), jstr(row, "species")) else {
            continue;
        };
        out.push(Detection {
            host: host.to_string(),
            pl_name: jstr(row, "pl_name").map(|s| s.to_string()),
            species: species.to_string(),
        });
    }
    Ok(out)
}

fn read_planet_rows(path: &str) -> Result<HashMap<String, Vec<PlanetRow>>, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("params {path}: {e}"))?;
    let root = parse_json(&body).ok_or_else(|| format!("params {path}: json absent"))?;
    let JsonVal::Arr(rows) = &root else {
        return Err(format!("params {path}: root is not an array"));
    };
    let mut hosts: HashMap<String, Vec<PlanetRow>> = HashMap::new();
    for row in rows {
        let (Some(host), Some(pl_name)) = (jstr(row, "hostname"), jstr(row, "pl_name")) else {
            continue;
        };
        let pl_eqt = jnum(row, "pl_eqt").filter(|v| v.is_finite() && *v > 0.0);
        let (Some(teff), Some(rad_solar), Some(orbsmax_au)) = (
            jnum(row, "st_teff"),
            jnum(row, "st_rad"),
            jnum(row, "pl_orbsmax"),
        ) else {
            continue;
        };
        if !teff.is_finite() || !rad_solar.is_finite() || !orbsmax_au.is_finite() {
            continue;
        }
        let planets = hosts.entry(host.to_string()).or_default();
        if !planets.iter().any(|p| p.pl_name == pl_name) {
            planets.push(PlanetRow {
                pl_name: pl_name.to_string(),
                pl_eqt,
                teff,
                rad_solar,
                orbsmax_au,
            });
        }
    }
    Ok(hosts)
}

fn is_atomic_ionic(species: &str) -> bool {
    matches!(
        species,
        "Na I"
            | "Na"
            | "K I"
            | "K"
            | "H I"
            | "H I Ly-a"
            | "H-"
            | "He I"
            | "He II"
            | "He"
            | "Fe I"
            | "Fe II"
            | "Fe"
            | "Mg I"
            | "Mg II"
            | "Ca I"
            | "Ca II"
            | "C I"
            | "C II"
            | "O I"
            | "Si I"
            | "Si II"
            | "Ti I"
            | "Ti II"
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut seed = "/tmp/opencode/hst_detection_seed.json".to_string();
    let mut params = "/tmp/opencode/pscomppars_hst.json".to_string();
    let mut out = "/tmp/opencode/hst_disequilibrium_verdict.txt".to_string();
    let mut floor = DEFAULT_FLOOR;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    seed = v.clone();
                }
            }
            "--params" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    params = v.clone();
                }
            }
            "--out" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    out = v.clone();
                }
            }
            "--floor" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse::<f64>().ok()) {
                    if v.is_finite() && v > 0.0 {
                        floor = v;
                    }
                }
            }
            other => {
                eprintln!("hst_disequilibrium_survey_probe: unknown argument {other}");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let detections = match read_detections(&seed) {
        Ok(d) => d,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };
    let planet_rows = match read_planet_rows(&params) {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };

    let spec_names: Vec<String> = sulfur_gas_names();
    let slot_by_name: HashMap<String, usize> = spec_names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();

    let mut host_order: Vec<String> = Vec::new();
    for d in &detections {
        if !host_order.iter().any(|h| h == &d.host) {
            host_order.push(d.host.clone());
        }
    }

    let mut lines: Vec<String> = Vec::new();
    let mut teq_sources: Vec<String> = Vec::new();
    let mut n_hit_hosts = 0usize;

    for host in &host_order {
        let dets: Vec<&Detection> = detections.iter().filter(|d| &d.host == host).collect();
        let planets = match planet_rows.get(host) {
            Some(p) => p,
            None => {
                lines.push(format!(
                    "{host} | -- | pending -- no pscomppars line (hostname not in the archive)"
                ));
                continue;
            }
        };
        let mut named: Vec<&str> = dets
            .iter()
            .filter_map(|d| d.pl_name.as_deref())
            .collect();
        named.sort();
        named.dedup();
        let target: Option<&PlanetRow> = if named.len() == 1 {
            planets.iter().find(|p| p.pl_name == named[0])
        } else if named.is_empty() && planets.len() == 1 {
            Some(&planets[0])
        } else {
            None
        };
        let Some(p) = target else {
            let names: Vec<&str> = planets.iter().map(|p| p.pl_name.as_str()).collect();
            lines.push(format!(
                "{host} | -- | pending -- attribution unresolved: pscomppars lists {}",
                names.join(", ")
            ));
            continue;
        };
        let t_eq = match p.pl_eqt {
            Some(e) => {
                teq_sources.push(format!("{host} {}: pl_eqt {e:.1} K (pscomppars)", p.pl_name));
                e
            }
            None => {
                let Some(t) = teq(p.teff, p.rad_solar * SUN_RADIUS_M, p.orbsmax_au * AU_M, 0.0)
                else {
                    lines.push(format!(
                        "{host} | {} | pending -- Teq not computable",
                        p.pl_name
                    ));
                    continue;
                };
                teq_sources.push(format!(
                    "{host} {}: Teq from st_teff/st_rad/pl_orbsmax {t:.1} K (no pl_eqt)",
                    p.pl_name
                ));
                t
            }
        };
        let frac = if t_eq >= SULFUR_T_MIN {
            if t_eq > MODEL_T_MAX {
                lines.push(format!(
                    "{host} | {} | {t_eq:.0} K | pending -- Teq outside the model domain (>{MODEL_T_MAX:.0} K)",
                    p.pl_name
                ));
                continue;
            }
            match equilibrium_composition_sulfur(t_eq, P0_PA) {
                Some(f) => f,
                None => {
                    lines.push(format!(
                        "{host} | {} | {t_eq:.0} K | pending -- equilibrium solver does not converge",
                        p.pl_name
                    ));
                    continue;
                }
            }
        } else if t_eq >= COOL_T_MIN {
            match equilibrium_composition_condensed(t_eq, P0_PA) {
                Some(eq) => eq.frac,
                None => {
                    lines.push(format!(
                        "{host} | {} | {t_eq:.0} K | pending -- condensation-aware solver does not converge",
                        p.pl_name
                    ));
                    continue;
                }
            }
        } else {
            lines.push(format!(
                "{host} | {} | {t_eq:.0} K | pending -- Teq below the model domain ({COOL_T_MIN:.0} K)",
                p.pl_name
            ));
            continue;
        };

        let mut host_has_hit = false;
        for d in &dets {
            let name = d.species.as_str();
            let verdict_cell: String;
            let ratio_cell: String;
            match slot_by_name.get(name) {
                Some(slot) => {
                    let ratio = frac[*slot];
                    ratio_cell = format!("{ratio:.3e} (log10 {:.2})", ratio.log10());
                    if ratio < floor {
                        verdict_cell = "disequilibrium-HIT".to_string();
                        host_has_hit = true;
                    } else {
                        verdict_cell = "equilibrium-present".to_string();
                    }
                }
                None => {
                    if is_atomic_ionic(name) {
                        ratio_cell = "-".to_string();
                        verdict_cell =
                            "atomic/ionic line - not a disequilibrium-model carrier".to_string();
                    } else {
                        ratio_cell = "-".to_string();
                        verdict_cell = "model-absent-pending".to_string();
                    }
                }
            }
            lines.push(format!(
                "{host} | {name} | {:.0} K | {} | {}",
                t_eq, ratio_cell, verdict_cell
            ));
        }
        if host_has_hit {
            n_hit_hosts += 1;
        }
    }

    let mut text = String::new();
    text.push_str("HST Disequilibrium Survey (Nadel XIII, HST species hosts)\n");
    text.push_str(&format!(
        "model: thermochem::equilibrium_composition_sulfur, Teq at the planet, 1 bar, solar H,C,O,N,S (Floor 1e-6)\n"
    ));
    text.push_str(&format!(
        "seed {} ({} detection lines) | params pscomppars ({} hosts)\n",
        seed,
        detections.len(),
        planet_rows.len()
    ));
    text.push_str(&format!(
        "model species slots ({}): {}\n",
        spec_names.len(),
        spec_names.join(",")
    ));
    text.push_str("Teq source: pscomppars pl_eqt (archive value); without pl_eqt the st_teff/st_rad/pl_orbsmax calculation\n");
    text.push_str(&format!("floor = {:.1e}\n", floor));
    text.push_str("Attribution (literature, measured against SIMBAD-Ref/Abstract):\n");
    text.push_str(
        "  HD 106315 -> c: 2021AJ....161...19G ARES IV (Guilluy 2021) 'two warm small planets HD 106315c and HD 3167c'; H2O 5.68 sigma\n",
    );
    text.push_str(
        "  HD 3167 -> c: 2021AJ....161...18M (Mikal-Evans 2021) HD 3167c H2O/CO2/HCN/CH4 2.5 sigma blend; ARES IV lists H2O 3.17 sigma, CO2 3.28 sigma (possibly systematics)\n",
    );
    text.push('\n');
    text.push_str("hostname | species | planet-Teq K | equilibrium mixing ratio | verdict\n");
    for l in &lines {
        text.push_str(l);
        text.push('\n');
    }
    text.push('\n');
    text.push_str(&format!(
        "Number of the 19 HST species hosts with a disequilibrium-HIT: {}\n",
        n_hit_hosts
    ));
    for s in &teq_sources {
        text.push_str(&format!("Teq source: {s}\n"));
    }

    if let Err(e) = std::fs::write(&out, &text) {
        eprintln!("{out}: {e}");
        std::process::exit(1);
    }
    println!("{text}");
}
