use omegaflow::equilibrium::{teq, AU_M, SUN_RADIUS_M};
use omegaflow::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::thermochem::{equilibrium_composition, solar, species, P0_PA};
use std::collections::HashMap;

const RNG_MULT: u64 = 6364136223846793005;
const RNG_INC: u64 = 1442695040888963407;
const FULL_CIRCLE: f64 = (u32::MAX >> 1) as f64;
const RNG_SEED: u64 = 0x5EED_0D15_EA5E_2026;
const DEFAULT_TRIALS: usize = 10_000;
const DEFAULT_FLOOR: f64 = 1.0e-6;
const MODEL_T_MIN: f64 = 500.0;
const MODEL_T_MAX: f64 = 3000.0;

fn rng_unit(rng: &mut u64) -> f64 {
    *rng = rng.wrapping_mul(RNG_MULT).wrapping_add(RNG_INC);
    ((*rng >> 33) as f64) / FULL_CIRCLE
}

fn shuffle<T>(v: &mut [T], rng: &mut u64) {
    for i in (1..v.len()).rev() {
        let j = ((rng_unit(rng) * (i + 1) as f64).floor() as usize).min(i);
        v.swap(i, j);
    }
}

struct Detection {
    host: String,
    species: String,
}

struct PlanetRow {
    pl_name: String,
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
        if !host.is_empty() && !species.is_empty() {
            out.push(Detection { host, species });
        }
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
        let (Some(teff), Some(rad_solar), Some(orbsmax_au)) = (
            jnum(row, "st_teff"),
            jnum(row, "st_rad"),
            jnum(row, "pl_orbsmax"),
        ) else {
            continue;
        };
        if !teff.is_finite()
            || teff <= 0.0
            || !rad_solar.is_finite()
            || rad_solar <= 0.0
            || !orbsmax_au.is_finite()
            || orbsmax_au <= 0.0
        {
            continue;
        }
        let planets = hosts.entry(host).or_default();
        if !planets.iter().any(|p| p.pl_name == pl_name) {
            planets.push(PlanetRow {
                pl_name,
                teff,
                rad_solar,
                orbsmax_au,
            });
        }
    }
    Ok(hosts)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut seed = "docs/reference/jwst_detection_seed.json".to_string();
    let mut params = String::new();
    let mut out = "/tmp/opencode/disequilibrium_register_verdict.txt".to_string();
    let mut floor = DEFAULT_FLOOR;
    let mut trials = DEFAULT_TRIALS;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                i += 1;
                seed = args.get(i).cloned().unwrap_or(seed);
            }
            "--params" => {
                i += 1;
                params = args.get(i).cloned().unwrap_or(params);
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(out);
            }
            "--floor" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse::<f64>().ok()) {
                    if v.is_finite() && v > 0.0 {
                        floor = v;
                    }
                }
            }
            "--trials" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse::<usize>().ok()) {
                    if v > 0 {
                        trials = v;
                    }
                }
            }
            other => {
                eprintln!("disequilibrium_register_probe: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if params.is_empty() {
        eprintln!(
            "disequilibrium_register_probe: --params <nexsci pscomppars json> — the host Teq source is mandatory"
        );
        std::process::exit(1);
    }
    if let Err(msg) = run(&seed, &params, &out, floor, trials) {
        eprintln!("disequilibrium_register_probe: {msg}");
        std::process::exit(1);
    }
}

#[derive(Clone)]
struct HostPlan {
    pl_name: String,
    teq_k: f64,
    frac: Vec<f64>,
}

fn run(
    seed_path: &str,
    params_path: &str,
    out_path: &str,
    floor: f64,
    trials: usize,
) -> Result<(), String> {
    let detections = read_detections(seed_path)?;
    let planet_rows = read_planet_rows(params_path)?;

    let spec_names: Vec<String> = species().into_iter().map(|s| s.name.to_string()).collect();
    let slot_by_name: HashMap<String, usize> = spec_names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();

    let mut out = String::new();
    out.push_str("disequilibrium_register_probe — ko-indizierter Disequilibrium-Befund\n");
    out.push_str("signal: eine PUBLIZIERT-DETEKTIERTE Spezies, deren Gleichgewichts-Mischungsverhältnis bei Teq die Detektions-Schwelle nicht erreicht = disequilibrium-hit\n");
    out.push_str(&format!(
        "inputs: seed {} ({} Detektionen) | params {} ({} Wirte)\n",
        seed_path,
        detections.len(),
        params_path,
        planet_rows.len()
    ));
    out.push_str(&format!(
        "modell: thermochem::equilibrium_composition bei {:.0} bar, solare Haeufigkeit (H,C,O,N) | Domäne {}..{} K\n",
        P0_PA / 101325.0, MODEL_T_MIN, MODEL_T_MAX
    ));
    out.push_str(&format!(
        "detektions-schwelle floor = {:.1e} (Mischungsverhältnis); Herkunft: Urteilswert, benannt —\n",
        floor
    ));
    out.push_str(
        "  die Saat traegt keine Instrumenten-Nachweisgrenze je Spezies; die Schwelle steht als --floor und ihre Empfindlichkeit ist unten gemessen\n",
    );

    out.push_str(&format!(
        "gleichgewichts-Spezies des Modells ({} Slots): ",
        spec_names.len()
    ));
    for (i, n) in spec_names.iter().enumerate() {
        out.push_str(&format!("{}={} ", i, n));
    }
    out.push('\n');

    let mut detected_species: Vec<&str> = detections.iter().map(|d| d.species.as_str()).collect();
    detected_species.sort_unstable();
    detected_species.dedup();
    let in_model: Vec<&str> = detected_species
        .iter()
        .copied()
        .filter(|s| slot_by_name.contains_key(*s))
        .collect();
    let out_model: Vec<&str> = detected_species
        .iter()
        .copied()
        .filter(|s| !slot_by_name.contains_key(*s))
        .collect();
    out.push_str(&format!(
        "erkannte Spezies ({}): mit Slot {}; out-of-model {}\n",
        detected_species.len(),
        in_model.join(","),
        out_model.join(",")
    ));

    let mut host_ids: Vec<String> = Vec::new();
    let mut host_det: Vec<Vec<String>> = Vec::new();
    let mut rows: Vec<(usize, String)> = Vec::new();
    for d in &detections {
        let id = match host_ids.iter().position(|h| h == &d.host) {
            Some(id) => id,
            None => {
                host_ids.push(d.host.clone());
                host_det.push(Vec::new());
                host_ids.len() - 1
            }
        };
        host_det[id].push(d.species.clone());
        rows.push((id, d.species.clone()));
    }

    let mut plan: Vec<Option<HostPlan>> = vec![None; host_ids.len()];
    let mut pending_reason: Vec<Option<String>> = vec![None; host_ids.len()];
    let mut n_pending_params = 0usize;
    let mut n_pending_multi = 0usize;
    let mut n_pending_domain = 0usize;
    let mut n_pending_solver = 0usize;

    for id in 0..host_ids.len() {
        let host = &host_ids[id];
        let Some(planets) = planet_rows.get(host) else {
            pending_reason[id] = Some("keine pscomppars-Zeile".to_string());
            n_pending_params += 1;
            continue;
        };
        if planets.len() > 1 {
            let names: Vec<&str> = planets.iter().map(|p| p.pl_name.as_str()).collect();
            pending_reason[id] = Some(format!(
                "Attribution offen — {} transiting-Planeten: {}",
                planets.len(),
                names.join(", ")
            ));
            n_pending_multi += 1;
            continue;
        }
        let p = &planets[0];
        let Some(t_eq) = teq(p.teff, p.rad_solar * SUN_RADIUS_M, p.orbsmax_au * AU_M, 0.0) else {
            pending_reason[id] = Some("Teq nicht berechenbar".to_string());
            n_pending_domain += 1;
            continue;
        };
        if !(MODEL_T_MIN..=MODEL_T_MAX).contains(&t_eq) {
            pending_reason[id] = Some(format!(
                "Teq {:.0} K ausserhalb der Modell-Domäne {}..{} K",
                t_eq, MODEL_T_MIN, MODEL_T_MAX
            ));
            n_pending_domain += 1;
            continue;
        }
        let Some(frac) = equilibrium_composition(t_eq, P0_PA, solar()) else {
            pending_reason[id] = Some(format!(
                "Gleichgewichts-Loeser konvergiert bei {:.0} K nicht",
                t_eq
            ));
            n_pending_solver += 1;
            continue;
        };
        plan[id] = Some(HostPlan {
            pl_name: p.pl_name.clone(),
            teq_k: t_eq,
            frac,
        });
    }

    let mut n_hit_hosts = 0usize;
    let mut n_eq_hosts = 0usize;
    let mut n_oom_only_hosts = 0usize;
    let mut lines: Vec<String> = Vec::new();
    for id in 0..host_ids.len() {
        let Some(hp) = &plan[id] else {
            continue;
        };
        let host = &host_ids[id];
        let mut hit_species: Vec<String> = Vec::new();
        let mut present_species: Vec<String> = Vec::new();
        let mut oom: Vec<String> = Vec::new();
        for name in &host_det[id] {
            match slot_by_name.get(name) {
                Some(slot) => {
                    if hp.frac[*slot] >= floor {
                        present_species.push(format!(
                            "{} (Slot {}; gleichgewichts-Anteil {:.3e})",
                            name, slot, hp.frac[*slot]
                        ));
                    } else {
                        hit_species.push(format!(
                            "{} (Slot {}; gleichgewichts-Anteil {:.3e})",
                            name, slot, hp.frac[*slot]
                        ));
                    }
                }
                None => oom.push(name.clone()),
            }
        }
        oom.sort();
        oom.dedup();
        if hit_species.is_empty() && present_species.is_empty() {
            n_oom_only_hosts += 1;
            lines.push(format!(
                "  {host}: {}  Teq {:.0} K  — nur out-of-model-Detektionen ({}): Gleichgewicht nicht prüfbar",
                hp.pl_name, hp.teq_k, oom.join(",")
            ));
            continue;
        }
        let word = if !hit_species.is_empty() {
            n_hit_hosts += 1;
            "disequilibrium-hit"
        } else {
            n_eq_hosts += 1;
            "equilibrium-present"
        };
        let mut block = format!(
            "  {host}: {}  Teq {:.0} K  VERDICT {word}",
            hp.pl_name, hp.teq_k
        );
        if !oom.is_empty() {
            block.push_str(&format!("  (out-of-model benannt: {})", oom.join(",")));
        }
        block.push('\n');
        for s in &hit_species {
            block.push_str(&format!("      disequilibrium  {s}\n"));
        }
        for s in &present_species {
            block.push_str(&format!("      equilibrium    {s}\n"));
        }
        lines.push(block);
    }

    let host_count = host_ids.len();
    out.push_str(&format!(
        "Wirte: {} gesamt | disequilibrium-hit {} | equilibrium-present {} | out-of-model-only {} | pending {} (params-fehlend {}, multi-planet {}, Domäne {}, Loeser {})\n",
        host_count, n_hit_hosts, n_eq_hosts, n_oom_only_hosts,
        n_pending_params + n_pending_multi + n_pending_domain + n_pending_solver,
        n_pending_params, n_pending_multi, n_pending_domain, n_pending_solver
    ));
    for id in 0..host_ids.len() {
        if let Some(reason) = &pending_reason[id] {
            out.push_str(&format!(
                "  {host}: pending — {reason} (detektiert: {det})\n",
                host = host_ids[id],
                det = host_det[id].join(",")
            ));
        }
    }
    out.push('\n');
    for l in &lines {
        out.push_str(l);
        out.push('\n');
    }

    let possible: Vec<bool> = plan
        .iter()
        .enumerate()
        .map(|(id, hp)| hp.is_some() && host_det[id].iter().any(|s| slot_by_name.contains_key(s)))
        .collect();
    let possible_count = possible.iter().filter(|b| **b).count();
    let observed = possible
        .iter()
        .enumerate()
        .filter(|(id, p)| {
            **p && host_det[*id].iter().any(|name| {
                slot_by_name
                    .get(name)
                    .map(|slot| plan[*id].as_ref().unwrap().frac[*slot] < floor)
                    .unwrap_or(false)
            })
        })
        .count();

    let mut rng = RNG_SEED;
    let mut species_perm: Vec<String> = rows.iter().map(|(_, s)| s.clone()).collect();
    let mut hit_sum = 0.0f64;
    let mut hit_sum_sq = 0.0f64;
    let mut over_obs = 0usize;
    for _ in 0..trials {
        shuffle(&mut species_perm, &mut rng);
        let mut host_hit = vec![false; host_ids.len()];
        for (i, (hid, _)) in rows.iter().enumerate() {
            if !possible[*hid] || host_hit[*hid] {
                continue;
            }
            if let Some(slot) = slot_by_name.get(&species_perm[i]) {
                if plan[*hid].as_ref().unwrap().frac[*slot] < floor {
                    host_hit[*hid] = true;
                }
            }
        }
        let h = host_hit.iter().filter(|b| **b).count() as f64;
        hit_sum += h;
        hit_sum_sq += h * h;
        if h >= observed as f64 {
            over_obs += 1;
        }
    }
    let mean = hit_sum / trials as f64;
    let var = (hit_sum_sq / trials as f64 - mean * mean).max(0.0);
    let sd = var.sqrt();
    let threshold = mean + 2.0 * sd;
    let tail = over_obs as f64 / trials as f64;

    out.push_str(&format!(
        "Katalog-Null: Permutation der Spezies-Zuweisung ueber {} Detektions-Zeilen, {} Ziehungen, fester Samen {:016x}\n",
        rows.len(), trials, RNG_SEED
    ));
    out.push_str(&format!(
        "  Hosts, in denen eine Detektion als disequilibrium wertbar ist (möglich): {}\n",
        possible_count
    ));
    out.push_str(&format!(
        "  beobachtete Treffer-Wirte {} | Null mean {:.2} | sigma {:.2} | Schwelle mean+2sigma {:.2} | tail P(T >= {}) = {:.4}\n",
        observed, mean, sd, threshold, observed, tail
    ));
    if observed as f64 >= threshold {
        out.push_str(
            "  Befund: die beobachtete Treffer-Zahl liegt auf/ueber der mean+2sigma-Schwelle — die Koinzidenz ist aussergewoehnlich\n",
        );
    } else {
        out.push_str(
            "  Befund: die beobachtete Treffer-Zahl liegt unter der mean+2sigma-Schwelle — mit dem Zufall vertraeglich\n",
        );
    }

    let sens: Vec<String> = [1.0e-7, 1.0e-6, 1.0e-5, 1.0e-4]
        .iter()
        .map(|f| {
            let h = (0..host_ids.len())
                .filter(|id| {
                    plan[*id].is_some()
                        && host_det[*id].iter().any(|name| {
                            slot_by_name
                                .get(name)
                                .map(|slot| plan[*id].as_ref().unwrap().frac[*slot] < *f)
                                .unwrap_or(false)
                        })
                })
                .count();
            format!("floor {:.0e} -> {} Treffer-Wirte", f, h)
        })
        .collect();
    out.push_str("Empfindlichkeit der floor-Urteilswerts (beobachtet): ");
    out.push_str(&sens.join(" | "));
    out.push('\n');

    std::fs::write(out_path, &out).map_err(|e| format!("{out_path}: {e}"))?;
    println!("{out}");
    Ok(())
}
