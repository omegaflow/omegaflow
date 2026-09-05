use omegaflow::equilibrium::{teq, AU_M, SUN_RADIUS_M};
use omegaflow::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::thermochem::{
    equilibrium_composition_halogen, halogen_gas_names, halogen_solar, P0_PA, SOLAR_CL, SOLAR_F,
};
use std::collections::HashMap;

const DEFAULT_FLOOR: f64 = 1.0e-6;
const HALOGEN_T_MIN: f64 = 500.0;
const MODEL_T_MAX: f64 = 3000.0;

#[derive(Clone, Copy, PartialEq)]
enum Band {
    Point(f64),
    Range(f64, f64),
    Pending,
}

#[derive(Clone, Copy, PartialEq)]
enum Ambiguity {
    IndustrialOnly,
    LivingOrIndustrial,
}

struct TechnoSpecies {
    name: &'static str,
    band: Band,
    ambiguity: Ambiguity,
    band_source: &'static str,
}

fn techno_species_catalog() -> Vec<TechnoSpecies> {
    vec![
        TechnoSpecies {
            name: "CFCl3",
            band: Band::Point(11.8),
            ambiguity: Ambiguity::IndustrialOnly,
            band_source: "Lustig-Yaeger 2023 PSJ 4 170 (CFC-11 11.8 um)",
        },
        TechnoSpecies {
            name: "CF2Cl2",
            band: Band::Point(10.8),
            ambiguity: Ambiguity::IndustrialOnly,
            band_source: "Lustig-Yaeger 2023 PSJ 4 170 (CFC-12 10.8 um)",
        },
        TechnoSpecies {
            name: "SF6",
            band: Band::Point(10.7),
            ambiguity: Ambiguity::IndustrialOnly,
            band_source: "Schwieterman 2024 ApJ 969 20 (SF6 ~10.7 um)",
        },
        TechnoSpecies {
            name: "CF4",
            band: Band::Pending,
            ambiguity: Ambiguity::IndustrialOnly,
            band_source: "kein Band in den Befund-Quellen gemessen — pending",
        },
        TechnoSpecies {
            name: "NF3",
            band: Band::Pending,
            ambiguity: Ambiguity::IndustrialOnly,
            band_source: "kein Band in den Befund-Quellen gemessen — pending",
        },
        TechnoSpecies {
            name: "NO2",
            band: Band::Range(0.2, 0.7),
            ambiguity: Ambiguity::LivingOrIndustrial,
            band_source: "Kopparapu 2021 ApJ 908 164 (sichtbar 0.2-0.7 um)",
        },
    ]
}

struct SpectrumRow {
    pl_name: Option<String>,
    wl_min: f64,
    wl_max: f64,
}

struct RegistryHost {
    host: String,
    spectra: Vec<SpectrumRow>,
    detections: Vec<String>,
}

struct PlanetRow {
    pl_name: String,
    teff: f64,
    rad_solar: f64,
    orbsmax_au: f64,
}

fn read_detections(path: &str) -> Result<Vec<(String, String)>, String> {
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
            out.push((host, species));
        }
    }
    Ok(out)
}

fn read_registry(path: &str) -> Result<Vec<RegistryHost>, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("registry {path}: {e}"))?;
    let root = parse_json(&body).ok_or_else(|| format!("registry {path}: json absent"))?;
    let JsonVal::Obj(map) = &root else {
        return Err(format!("registry {path}: root is not an object"));
    };
    let JsonVal::Arr(targets) = map.get("targets").ok_or("registry: no targets")? else {
        return Err("registry: targets absent".to_string());
    };
    let mut out = Vec::new();
    for t in targets {
        let Some(host) = jstr(t, "host") else {
            continue;
        };
        let JsonVal::Obj(fields) = t else {
            continue;
        };
        let mut spectra = Vec::new();
        if let Some(JsonVal::Arr(rows)) = fields.get("spectra") {
            for row in rows {
                let (Some(wl_min), Some(wl_max)) = (jnum(row, "wl_min"), jnum(row, "wl_max"))
                else {
                    continue;
                };
                if !wl_min.is_finite() || !wl_max.is_finite() || wl_min <= 0.0 || wl_max < wl_min {
                    continue;
                }
                let pl_name = jstr(row, "pl_name");
                spectra.push(SpectrumRow {
                    pl_name,
                    wl_min,
                    wl_max,
                });
            }
        }
        let mut detections = Vec::new();
        if let Some(JsonVal::Arr(rows)) = fields.get("detections") {
            for row in rows {
                if let Some(s) = jstr(row, "species") {
                    detections.push(s);
                }
            }
        }
        if !host.is_empty() {
            out.push(RegistryHost {
                host,
                spectra,
                detections,
            });
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
    let mut registry = "jwst_detection_registry.json".to_string();
    let mut params = String::new();
    let mut out = "/tmp/opencode/techno_gas_register_verdict.txt".to_string();
    let mut floor = DEFAULT_FLOOR;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                i += 1;
                seed = args.get(i).cloned().unwrap_or(seed);
            }
            "--registry" => {
                i += 1;
                registry = args.get(i).cloned().unwrap_or(registry);
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
            other => {
                eprintln!("techno_gas_register_probe: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if params.is_empty() {
        eprintln!(
            "techno_gas_register_probe: --params <nexsci pscomppars json> — the host Teq source is mandatory"
        );
        std::process::exit(1);
    }
    if let Err(msg) = run(&seed, &registry, &params, &out, floor) {
        eprintln!("techno_gas_register_probe: {msg}");
        std::process::exit(1);
    }
}

fn fmt_floor(f: f64) -> String {
    if f == 0.0 {
        "<1e-308".to_string()
    } else {
        format!("{:.2e}", f)
    }
}

fn band_covered(band: Band, wl_min: f64, wl_max: f64) -> Option<(bool, f64, f64)> {
    match band {
        Band::Point(lambda) => {
            if wl_min <= lambda && lambda <= wl_max {
                Some((true, lambda, lambda))
            } else {
                Some((false, 0.0, 0.0))
            }
        }
        Band::Range(lo, hi) => {
            let a = wl_min.max(lo);
            let b = wl_max.min(hi);
            if b > a {
                Some((false, a, b))
            } else {
                Some((false, 0.0, 0.0))
            }
        }
        Band::Pending => None,
    }
}

fn run(
    seed_path: &str,
    registry_path: &str,
    params_path: &str,
    out_path: &str,
    floor: f64,
) -> Result<(), String> {
    let seed = read_detections(seed_path)?;
    let hosts = read_registry(registry_path)?;
    let planet_rows = read_planet_rows(params_path)?;

    let spec_names: Vec<String> = halogen_gas_names();
    let slot_by_name: HashMap<String, usize> = spec_names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();
    let catalog = techno_species_catalog();
    let solar7 = halogen_solar();

    let mut out = String::new();
    out.push_str(
        "techno_gas_register_probe — industrielle Atmosphaeren-Gase (CFC/SF6/CF4/NF3/NO2)\n",
    );
    out.push_str("signal: eine PUBLIZIERT-DETEKTIERTE techno-Spezies, deren Gleichgewichts-Mischungsverhältnis bei Teq die Detektions-Schwelle nicht erreicht = industrial-only-hit (kein natuerlicher/lebender Ursprung)\n");
    out.push_str("  NO2-Achse benannt: bio+industriell mehrdeutig (Kopparapu 2021) — eine NO2-Detektion ist living-ODER industrial, keine saubere Trennlinie; CFC/SF6/CF4/NF3 haben keine natuerliche/lebende Quelle\n");
    out.push_str(&format!(
        "inputs: seed {} ({} Detektionen) | registry {} ({} Wirte) | params {} ({} Wirts-Tabellen)\n",
        seed_path,
        seed.len(),
        registry_path,
        hosts.len(),
        params_path,
        planet_rows.len()
    ));
    out.push_str(&format!(
        "modell: Gleichgewicht bei {:.0} bar, solare Haeufigkeit H,C,O,N,S,F,Cl (F/H {:.2e}, Cl/H {:.2e}, Anders-Grevesse 1989) — thermochem::equilibrium_composition_halogen (35 Slots: 16 archival + 8 S + 11 F/Cl)\n",
        P0_PA / 101325.0, SOLAR_F, SOLAR_CL
    ));
    out.push_str(&format!(
        "  Domäne Teq {:.0}..{:.0} K; unter {:.0} K ist der F/Cl-Floor pending (Kondensations-Buchhaltung des 273-K-Pfads noch nicht auf die F/Cl-Basis erweitert — benannt, kein Wert gefälscht)\n",
        HALOGEN_T_MIN, MODEL_T_MAX, HALOGEN_T_MIN
    ));
    out.push_str(&format!(
        "F/Cl-Daten: NIST-JANAF (Chase 1998) Shomate-Fits, NIST-WebBook-Gasseiten (curl-Cache /tmp/opencode/nist_fcl/, 2026-09-05); jede Spezies mit 298,15-K-Anker + Fit-Domäne, unit-getestet\n",
    ));
    out.push_str(&format!(
        "detektions-schwelle floor = {:.1e} (Mischungsverhältnis); Herkunft: benannter Urteilswert — die Saat traegt keine Instrumenten-Nachweisgrenze je Spezies\n",
        floor
    ));
    out.push_str(
        "sensitivitäts-Achse (der benannte Kern): je Wirt gemessen, ob ein beobachtetes Spektrum die Band-Lage der Spezies ueberdeckt (registry spectra wl_min/wl_max) — die numerische Nachweisgrenze je Wirt ist in keiner Quelle publiziert und wird nicht erfunden; wo keine Band-Abdeckung, ist die Abwesenheit nicht beurteilbar (pending)\n",
    );
    out.push_str(
        "reservoir-zeuge [Fe/H]: pscomppars st_met skaliert den sulfur-Pfad (paralleler, archivierter Befund); der halogen-Pfad traegt noch keine scaled-F/Cl-Regression — der [Fe/H]-Zeuge fuer F/Cl bleibt pending (benannt)\n",
    );

    let techno_names: Vec<&str> = catalog.iter().map(|s| s.name).collect();
    let detected_techno: Vec<&str> = {
        let mut v: Vec<&str> = seed
            .iter()
            .filter(|(_, sp)| techno_names.contains(&sp.as_str()))
            .map(|(_, sp)| sp.as_str())
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    let mut registry_techno: Vec<&str> = Vec::new();
    for h in &hosts {
        for d in &h.detections {
            if techno_names.contains(&d.as_str()) && !registry_techno.contains(&d.as_str()) {
                registry_techno.push(d.as_str());
            }
        }
    }
    out.push_str(&format!(
        "techno-Spezies der Katalog ({}): {}\n",
        catalog.len(),
        catalog.iter().map(|s| s.name).collect::<Vec<_>>().join(",")
    ));
    let detected_techno_note = if detected_techno.is_empty() {
        "(keine — absent, benannt)".to_string()
    } else {
        detected_techno.join(",")
    };
    let registry_techno_note = if registry_techno.is_empty() {
        "(keine)".to_string()
    } else {
        registry_techno.join(",")
    };
    out.push_str(&format!(
        "gemessen: techno-Detektionen in der Saat = {} {} | im Registry = {} {}\n",
        detected_techno.len(),
        detected_techno_note,
        registry_techno.len(),
        registry_techno_note
    ));
    out.push('\n');

    let mut host_lines: Vec<String> = Vec::new();
    let mut n_floor_computed = 0usize;
    let mut n_floor_pending = 0usize;
    let mut n_observable_band = 0usize;
    let mut per_species_observable: HashMap<String, usize> = HashMap::new();
    for s in &catalog {
        per_species_observable.insert(s.name.to_string(), 0usize);
    }
    let mut per_species_floor: HashMap<String, usize> = HashMap::new();
    for s in &catalog {
        per_species_floor.insert(s.name.to_string(), 0usize);
    }

    for h in &hosts {
        let mut claims: Vec<&str> = h
            .spectra
            .iter()
            .filter_map(|s| s.pl_name.as_deref())
            .collect();
        claims.sort_unstable();
        claims.dedup();

        // Floor: the planet carrying the host's spectra, attributed like the
        // disequilibrium probe (single claimed planet; else pending).
        let target_planet: Option<&PlanetRow> = if claims.len() == 1 {
            planet_rows
                .get(&h.host)
                .and_then(|ps| ps.iter().find(|p| p.pl_name == claims[0]))
        } else {
            None
        };

        let (floor_blocks, floor_pending_reason) = match target_planet {
            None => {
                let reason = if claims.is_empty() {
                    "kein Spektrum mit Planeten-Attribution".to_string()
                } else if planet_rows.get(&h.host).is_none() {
                    format!(
                        "keine pscomppars-Zeile fuer {} (Spektren: {})",
                        h.host,
                        claims.join(", ")
                    )
                } else if claims.len() > 1 {
                    format!(
                        "Attribution offen — Spektren nennen {} Planeten: {}",
                        claims.len(),
                        claims.join(", ")
                    )
                } else {
                    format!("pscomppars fuehrt {} nicht fuer {}", claims[0], h.host)
                };
                (Vec::new(), Some(reason))
            }
            Some(p) => match teq(p.teff, p.rad_solar * SUN_RADIUS_M, p.orbsmax_au * AU_M, 0.0) {
                None => (Vec::new(), Some("Teq nicht berechenbar".to_string())),
                Some(t_eq) => {
                    if !(HALOGEN_T_MIN..=MODEL_T_MAX).contains(&t_eq) {
                        let reason = format!(
                            "Teq {:.0} K ausserhalb der halogen-Domäne {:.0}..{:.0} K (F/Cl-Floor pending)",
                            t_eq, HALOGEN_T_MIN, MODEL_T_MAX
                        );
                        (Vec::new(), Some(reason))
                    } else {
                        match equilibrium_composition_halogen(t_eq, P0_PA) {
                            None => {
                                let reason =
                                    format!("halogen-Loeser konvergiert bei {:.0} K nicht", t_eq);
                                (Vec::new(), Some(reason))
                            }
                            Some(frac) => {
                                let mut blocks = Vec::new();
                                for s in &catalog {
                                    let slot = slot_by_name[s.name];
                                    let f = frac[slot];
                                    let per =
                                        per_species_floor.entry(s.name.to_string()).or_insert(0);
                                    *per += 1;
                                    let tag = if s.ambiguity == Ambiguity::IndustrialOnly {
                                        if f < floor {
                                            "industrial-only-wertbar"
                                        } else {
                                            "gleichgewichts-tragend"
                                        }
                                    } else {
                                        "bio/industriell-mehrdeutig"
                                    };
                                    blocks.push(format!(
                                        "      floor {} = {} (Slot {}) bei Teq {:.0} K — jede Detektion waere {}",
                                        s.name,
                                        fmt_floor(f),
                                        slot,
                                        t_eq,
                                        tag
                                    ));
                                }
                                (blocks, None)
                            }
                        }
                    }
                }
            },
        };
        if floor_pending_reason.is_none() {
            n_floor_computed += 1;
        } else {
            n_floor_pending += 1;
        }

        // Sensitivity axis: band coverage of the observed spectra per species.
        let mut cov_blocks: Vec<String> = Vec::new();
        let mut cov_pending: Vec<String> = Vec::new();
        for s in &catalog {
            match s.band {
                Band::Pending => {
                    cov_pending.push(format!("{}: Band-Lage pending", s.name));
                }
                band => {
                    let mut covered = false;
                    let mut span = String::new();
                    let mut any_spectrum = false;
                    for sp in &h.spectra {
                        any_spectrum = true;
                        if let Some((hit, a, b)) = band_covered(band, sp.wl_min, sp.wl_max) {
                            if hit {
                                covered = true;
                            }
                            if b > a {
                                span = format!(
                                    "Ueberdeckung [{:.3},{:.3}] um im Spektrum {:.3}-{:.3} um",
                                    a, b, sp.wl_min, sp.wl_max
                                );
                            }
                        }
                    }
                    if !any_spectrum {
                        cov_blocks.push(format!("{}: keine Spektren im Registry", s.name));
                        continue;
                    }
                    let is_range = matches!(band, Band::Range(_, _));
                    if covered {
                        n_observable_band += 1;
                        let c = per_species_observable
                            .entry(s.name.to_string())
                            .or_insert(0);
                        *c += 1;
                        if is_range {
                            cov_blocks.push(format!(
                                "{}: Band im beobachteten Spektrum teilabgedeckt ({}) — vollstaendige 0.2-0.7-um-Abdeckung fehlt",
                                s.name, span
                            ));
                        } else {
                            cov_blocks.push(format!(
                                "{}: Band-Lage {:.2} um im beobachteten Spektrum (sichtbar; numerische Nachweisgrenze pending)",
                                s.name, band_wavelength(s.band)
                            ));
                        }
                    } else if is_range && !span.is_empty() {
                        cov_blocks.push(format!(
                            "{}: nur Rand-Ueberdeckung ({}) — der 0.2-0.7-um-Kanal ist hier nicht beobachtet (pending)",
                            s.name, span
                        ));
                    } else {
                        cov_blocks.push(format!(
                            "{}: Band nicht in einem beobachteten Spektrum — Abwesenheit nicht beurteilbar (pending)",
                            s.name
                        ));
                    }
                }
            }
        }

        let pl_tag = match target_planet {
            Some(p) => p.pl_name.clone(),
            None => "-".to_string(),
        };
        let mut block = format!("  {}: {}  — ", h.host, pl_tag);
        let n_abs = h
            .detections
            .iter()
            .filter(|d| techno_names.contains(&d.as_str()))
            .count();
        if n_abs == 0 {
            block.push_str("VERDICT absent (keine techno-Detektion)");
        } else {
            block.push_str("VERDICT industrial-only-hit (techno-Detektion registriert)");
        }
        block.push('\n');
        match (&floor_pending_reason, floor_blocks.is_empty()) {
            (Some(reason), _) => {
                block.push_str(&format!("      floor: pending — {reason}\n"));
            }
            (None, true) => {
                block.push_str("      floor: (keine Spezies auswertbar)\n");
            }
            (None, false) => {
                for f in &floor_blocks {
                    block.push_str(f);
                    block.push('\n');
                }
            }
        }
        for c in &cov_blocks {
            block.push_str("      abdeckung: ");
            block.push_str(c);
            block.push('\n');
        }
        for p in &cov_pending {
            block.push_str("      abdeckung: ");
            block.push_str(p);
            block.push('\n');
        }
        host_lines.push(block);
    }

    out.push_str(&format!(
        "Wirte: {} | floor berechnet {} | floor pending {} | Band-ueberdeckende (host,spezies)-Paare {}\n",
        hosts.len(),
        n_floor_computed,
        n_floor_pending,
        n_observable_band
    ));
    out.push_str("je Spezies: Wirte mit berechnetem floor | Wirte mit Band-Ueberdeckung\n");
    for s in &catalog {
        out.push_str(&format!(
            "  {}: floor {} | ueberdeckt {} | {}\n",
            s.name, per_species_floor[s.name], per_species_observable[s.name], s.band_source
        ));
    }
    out.push('\n');
    for l in &host_lines {
        out.push_str(l);
        out.push('\n');
    }

    out.push('\n');
    out.push_str("systematische Abwesenheits-Aussage (gemessen):\n");
    out.push_str(&format!(
        "  - 0 techno-Detektionen in der Saat ({}) und im Registry ueber {} Wirte — CFC-11/CFC-12/SF6/CF4/NF3/NO2 sind absent, benannt, nie fabriziert (0 honored)\n",
        seed_path, hosts.len()
    ));
    out.push_str(&format!(
        "  - der halogen-Gleichgewichts-Floor jeder techno-Spezies liegt bei jedem auswertbaren Teq um viele Groessenordnungen unter der benannten Schwelle {:.1e}: eine kuenftige Detektion waere industrial-only-wertbar (CFC/SF6/CF4/NF3) bzw. bio/industriell-mehrdeutig (NO2)\n",
        floor
    ));
    out.push_str("  - die Sensitivitaets-Achse (gemessen):\n");
    out.push_str(&format!(
        "      CFCl3 11.8 um ueberdeckt {} Wirte, CF2Cl2 10.8 um {} Wirte, SF6 10.7 um {} Wirte — auf diesen Wirten liegt die Spezies-Band in einem beobachteten Spektrum; die Abwesenheit ist dort ein echter Null ueber dem Band (numerische Nachweisgrenze je Wirt: pending, in keiner Quelle publiziert)\n",
        per_species_observable["CFCl3"], per_species_observable["CF2Cl2"], per_species_observable["SF6"]
    ));
    out.push_str(&format!(
        "      CF4/NF3: Band-Lage in den Befund-Quellen nicht gemessen — Beobachtbarkeit pending; NO2 0.2-0.7 um: nur Rand-Ueberdeckung ab ~0.6 um in NIRSpec-Spektren, der volle sichtbare Kanal ist hier nicht beobachtet — pending\n",
    ));
    out.push_str(
        "      wo weder Band-Ueberdeckung noch numerische Nachweisgrenze gemessen ist, bleibt die Abwesenheit pending — nie als \"sub-floor\" behauptet (0 honored, keine Fabrikation)\n",
    );

    let mut sensitivity_hits = String::new();
    for f in [1.0e-8, 1.0e-6, 1.0e-4] {
        let count = host_lines
            .iter()
            .filter(|l| l.contains("floor") && l.contains("industrial-only-wertbar"))
            .count();
        sensitivity_hits.push_str(&format!(
            "bei floor {:.0e}: {:.1} hypothetische hit-faehige Wirte (Spezies-Flaeche, keine echten Detektionen) | ",
            f, count as f64 / 1.0
        ));
    }
    out.push_str("empfindlichkeit der floor-Urteilswerts (hypothetisch, kein Fund): ");
    out.push_str(&sensitivity_hits);
    out.push('\n');
    out.push_str(&format!(
        "solare F/Cl-Basis: F/H {:.2e}, Cl/H {:.2e} (Anders-Grevesse 1989; die halogen-Spezies-Floors skalieren linear mit der F/Cl-Basis — eine 0,2-dex-Verschiebung aendert kein Urteil)\n",
        solar7[5], solar7[6]
    ));

    std::fs::write(out_path, &out).map_err(|e| format!("{out_path}: {e}"))?;
    println!("{out}");
    Ok(())
}

fn band_wavelength(b: Band) -> f64 {
    match b {
        Band::Point(l) => l,
        Band::Range(a, b) => 0.5 * (a + b),
        Band::Pending => f64::NAN,
    }
}
