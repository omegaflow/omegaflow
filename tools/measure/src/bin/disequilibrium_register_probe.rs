use omegaflow::equilibrium::{teq, AU_M, SUN_RADIUS_M};
use omegaflow::json::{jnum, jstr, parse_json, JsonVal};
use omegaflow::thermochem::{
    elemental_budget_sulfur, equilibrium_composition_condensed,
    equilibrium_composition_condensed_budget, equilibrium_composition_condensed_scaled,
    equilibrium_composition_sulfur, equilibrium_composition_sulfur_budget,
    equilibrium_composition_sulfur_scaled, sulfur_gas_names, COOL_T_MIN, P0_PA, SOLAR_C, SOLAR_O,
    WATER_LIQ_T_MIN, WATER_P_TRIPLE_PA,
};
use std::collections::{HashMap, HashSet};

const RNG_MULT: u64 = 6364136223846793005;
const RNG_INC: u64 = 1442695040888963407;
const FULL_CIRCLE: f64 = (u32::MAX >> 1) as f64;
const RNG_SEED: u64 = 0x5EED_0D15_EA5E_2026;
const CORR_RNG_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_TRIALS: usize = 10_000;
const DEFAULT_FLOOR: f64 = 1.0e-6;
const SULFUR_T_MIN: f64 = 500.0;
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

#[derive(Clone)]
struct Detection {
    host: String,
    species: String,
    pl_name: Option<String>,
}

struct PlanetRow {
    pl_name: String,
    teff: f64,
    rad_solar: f64,
    orbsmax_au: f64,
    feh: Option<f64>,
    st_rotp_days: Option<f64>,
    st_vsin_kms: Option<f64>,
}

// One witness row as the register carries it: a (host, analysis) statement
// with any of the measured channels [C/H]/[O/H]/[N/H]/[Fe/H]/C-O (the C/O
// witness), log R'HK / L_X / F_X / L_X/L_bol / S-index / P_rot (the
// activity/XUV witness). A channel the row does not carry is None; a channel
// the row names as pending stays a token in `pending`, never a fabricated
// number. A row with a full [C/H]/[O/H]/[Fe/H] budget is what the C/O
// equilibrium regression can run; the activity channels feed the second
// cleaning step.
struct WitnessRow {
    analysis: String,
    delivery: Option<String>,
    ch: Option<f64>,
    oh: Option<f64>,
    nh: Option<f64>,
    feh: Option<f64>,
    co: Option<f64>,
    logrhk: Option<f64>,
    lx: Option<f64>,
    fx: Option<f64>,
    lxlbol: Option<f64>,
    s_index: Option<f64>,
    p_rot: Option<f64>,
    pending: Vec<String>,
    note: Option<String>,
}

impl WitnessRow {
    fn budget(&self) -> Option<(f64, f64, Option<f64>, f64)> {
        let (ch, oh, feh) = (self.ch?, self.oh?, self.feh?);
        if !ch.is_finite() || !oh.is_finite() || !feh.is_finite() {
            return None;
        }
        Some((ch, oh, self.nh.filter(|v| v.is_finite()), feh))
    }

    fn activity_number(&self) -> bool {
        [
            self.logrhk,
            self.lx,
            self.fx,
            self.lxlbol,
            self.s_index,
            self.p_rot,
        ]
        .iter()
        .any(|v| v.is_some())
    }

    fn pending_activity(&self) -> bool {
        self.pending.iter().any(|t| is_activity_token(t))
    }
}

fn read_witness(path: &str) -> Result<HashMap<String, Vec<WitnessRow>>, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("witness {path}: {e}"))?;
    let root = parse_json(&body).ok_or_else(|| format!("witness {path}: json absent"))?;
    let JsonVal::Arr(rows) = &root else {
        return Err(format!("witness {path}: root is not an array"));
    };
    let mut out: HashMap<String, Vec<WitnessRow>> = HashMap::new();
    for row in rows {
        let (Some(host), Some(analysis)) = (jstr(row, "hostname"), jstr(row, "analysis")) else {
            continue;
        };
        let nf = |k: &str| jnum(row, k).filter(|v| v.is_finite());
        let pending = jstr(row, "pending")
            .map(|s| {
                s.split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();
        let w = WitnessRow {
            analysis: analysis.to_string(),
            delivery: jstr(row, "delivery"),
            ch: nf("ch"),
            oh: nf("oh"),
            nh: nf("nh"),
            feh: nf("feh"),
            co: nf("co"),
            logrhk: nf("logrhk"),
            lx: nf("lx"),
            fx: nf("fx"),
            lxlbol: nf("lxlbol"),
            s_index: nf("s_index"),
            p_rot: nf("p_rot"),
            pending,
            note: jstr(row, "note"),
        };
        let empty = [
            w.ch, w.oh, w.nh, w.feh, w.co, w.logrhk, w.lx, w.fx, w.lxlbol, w.s_index, w.p_rot,
        ]
        .iter()
        .all(|v| v.is_none());
        if empty && w.note.is_none() && w.pending.is_empty() {
            continue;
        }
        out.entry(host.to_string()).or_default().push(w);
    }
    Ok(out)
}

struct CensusHost {
    hostname: String,
    class: String,
    note: Option<String>,
}

fn read_census(path: &str) -> Result<Vec<CensusHost>, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("census {path}: {e}"))?;
    let root = parse_json(&body).ok_or_else(|| format!("census {path}: json absent"))?;
    let JsonVal::Arr(rows) = &root else {
        return Err(format!("census {path}: root is not an array"));
    };
    let mut out = Vec::new();
    for row in rows {
        let (Some(hostname), Some(class)) = (jstr(row, "hostname"), jstr(row, "class")) else {
            continue;
        };
        if !hostname.is_empty() && !class.is_empty() {
            out.push(CensusHost {
                hostname,
                class,
                note: jstr(row, "note"),
            });
        }
    }
    Ok(out)
}

fn mean_v(v: &[f64]) -> f64 {
    let n = v.len() as f64;
    if n == 0.0 {
        f64::NAN
    } else {
        v.iter().sum::<f64>() / n
    }
}

fn pearson_r(x: &[f64], y: &[f64]) -> f64 {
    let mx = mean_v(x);
    let my = mean_v(y);
    let cov: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let vx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
    let vy: f64 = y.iter().map(|a| (a - my) * (a - my)).sum();
    if vx <= 0.0 || vy <= 0.0 {
        f64::NAN
    } else {
        cov / (vx * vy).sqrt()
    }
}

fn activity_label(t: &str) -> &str {
    match t {
        "logrhk" => "log R'HK",
        "lx" => "L_X",
        "fx" => "F_X",
        "lxlbol" => "L_X/L_bol",
        "s_index" => "S-index",
        "p_rot" => "P_rot",
        "xuv" => "XUV",
        _ => t,
    }
}

fn fmt_activity_witness(w: &WitnessRow) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(v) = w.logrhk {
        parts.push(format!("log R'HK {v:.2}"));
    }
    if let Some(v) = w.lx {
        parts.push(format!("L_X {v:.1e} erg/s"));
    }
    if let Some(v) = w.fx {
        parts.push(format!("F_X {v:.1e} erg/s/cm2"));
    }
    if let Some(v) = w.lxlbol {
        parts.push(format!("L_X/L_bol {v:.1e}"));
    }
    if let Some(v) = w.s_index {
        parts.push(format!("S-index {v:.3}"));
    }
    if let Some(v) = w.p_rot {
        parts.push(format!("P_rot {v:.1} d"));
    }
    let open: Vec<String> = w
        .pending
        .iter()
        .filter(|t| is_activity_token(t))
        .map(|t| activity_label(t).to_string())
        .collect();
    if parts.is_empty() && !open.is_empty() {
        parts.push(format!("{} pending", open.join("/")));
    } else if !open.is_empty() {
        parts.push(format!("offen: {}", open.join("/")));
    }
    let mut s = format!("{} [{}", parts.join(" | "), w.analysis);
    if let Some(d) = &w.delivery {
        s.push_str(&format!("; {d}"));
    }
    s.push(']');
    if let Some(n) = &w.note {
        s.push_str(&format!(" — {n}"));
    }
    s
}

fn is_activity_token(t: &str) -> bool {
    matches!(
        t,
        "logrhk" | "lx" | "fx" | "lxlbol" | "s_index" | "p_rot" | "xuv"
    )
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
            let pl_name = jstr(row, "pl_name").map(|s| s.to_string());
            out.push(Detection {
                host,
                species,
                pl_name,
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
        let feh = jnum(row, "st_met").filter(|v| v.is_finite());
        let st_rotp_days = jnum(row, "st_rotp").filter(|v| v.is_finite() && *v > 0.0);
        let st_vsin_kms = jnum(row, "st_vsin").filter(|v| v.is_finite());
        let planets = hosts.entry(host).or_default();
        if !planets.iter().any(|p| p.pl_name == pl_name) {
            planets.push(PlanetRow {
                pl_name,
                teff,
                rad_solar,
                orbsmax_au,
                feh,
                st_rotp_days,
                st_vsin_kms,
            });
        }
    }
    Ok(hosts)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut seed = "docs/reference/jwst_detection_seed.json".to_string();
    let mut params = String::new();
    let mut witness = "docs/reference/co_rhk_witness_seed.json".to_string();
    let mut census = "docs/reference/jwst_host_census.json".to_string();
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
            "--witness" => {
                i += 1;
                witness = args.get(i).cloned().unwrap_or(witness);
            }
            "--census" => {
                i += 1;
                census = args.get(i).cloned().unwrap_or(census);
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
    if let Err(msg) = run(&seed, &params, &witness, &census, &out, floor, trials) {
        eprintln!("disequilibrium_register_probe: {msg}");
        std::process::exit(1);
    }
}

#[derive(Clone)]
enum HostModel {
    Sulfur,
    Cool {
        condensed_h2o_moles: f64,
        vapor_pa: f64,
        sat_pa: Option<f64>,
    },
}

#[derive(Clone)]
struct HostPlan {
    pl_name: String,
    teq_k: f64,
    frac: Vec<f64>,
    frac_res: Option<Vec<f64>>,
    feh: Option<f64>,
    st_rotp_days: Option<f64>,
    st_vsin_kms: Option<f64>,
    model: HostModel,
}

// The measured host-abundance regression: equilibrium at the host's own
// [C/H]/[O/H]/[N/H] budget (S on the row's [Fe/H]) instead of solar C/O.
#[derive(Clone)]
struct MeasuredRun {
    analysis: String,
    ch: f64,
    oh: f64,
    feh: f64,
    c_over_o: f64,
    frac: Vec<f64>,
}

fn measured_budget_frac(
    t_eq: f64,
    ch: f64,
    oh: f64,
    nh: Option<f64>,
    feh: f64,
) -> Option<Vec<f64>> {
    let b = elemental_budget_sulfur(ch, oh, nh, feh)?;
    if t_eq >= SULFUR_T_MIN {
        equilibrium_composition_sulfur_budget(t_eq, P0_PA, b)
    } else {
        equilibrium_composition_condensed_budget(t_eq, P0_PA, b).map(|eq| eq.frac)
    }
}

fn c_over_o_solar(ch: f64, oh: f64) -> f64 {
    (SOLAR_C / SOLAR_O) * 10f64.powf(ch - oh)
}

fn run(
    seed_path: &str,
    params_path: &str,
    witness_path: &str,
    census_path: &str,
    out_path: &str,
    floor: f64,
    trials: usize,
) -> Result<(), String> {
    let detections = read_detections(seed_path)?;
    let planet_rows = read_planet_rows(params_path)?;
    let witness = read_witness(witness_path)?;
    let census = read_census(census_path)?;

    let n_witness_rows: usize = witness.values().map(|v| v.len()).sum();
    let n_budget_rows: usize = witness
        .values()
        .flatten()
        .filter(|w| w.budget().is_some())
        .count();
    let n_act_hosts: usize = witness
        .iter()
        .filter(|(_, rows)| {
            rows.iter()
                .any(|w| w.activity_number() || w.pending_activity())
        })
        .count();
    let n_xuv_hosts: usize = witness
        .iter()
        .filter(|(_, rows)| rows.iter().any(|w| w.lx.is_some() || w.fx.is_some()))
        .count();

    let spec_names: Vec<String> = sulfur_gas_names();
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
        "modell: Gleichgewicht bei {:.0} bar, solare Haeufigkeit H,C,O,N,S (S/H 1.62e-5, Anders-Grevesse 1989)\n",
        P0_PA / 101325.0
    ));
    out.push_str(&format!(
        "  Teq {:.0}..{:.0} K: thermochem::equilibrium_composition_sulfur (einphasiges Gas, 24 Slots)\n",
        SULFUR_T_MIN, MODEL_T_MAX
    ));
    out.push_str(&format!(
        "  Teq {:.0}..{:.0} K: thermochem::equilibrium_composition_condensed (kondensations-bewusst; dieselben 24 Gas-Slots)\n",
        COOL_T_MIN, SULFUR_T_MIN
    ));
    out.push_str(
        "sulfur-Daten: NIST-JANAF (Chase 1998) Shomate-Fits, atomares S(g) nur ab 882 K Fit-Domäne\n",
    );
    out.push_str(
        "kondensat-Daten: H2O(l) NIST-JANAF-Shomate 298.15..500 K (dHf298 -285.830 kJ/mol, S298 69.95 J/mol/K, WebBook 2026-09-05); NH3/CO2/CH4/H2S-kondensiert pending (WebBook traegt keine kondensierten Shomate-Fits; 0 honored); unter 298.15 K traegt die Dreipunkt-Sättigung 610 Pa den Einphasen-Befund bei 1-bar-solarer Häufigkeit\n",
    );
    out.push_str(&format!(
        "detektions-schwelle floor = {:.1e} (Mischungsverhältnis); Herkunft: Urteilswert, benannt —\n",
        floor
    ));
    out.push_str(
        "  die Saat traegt keine Instrumenten-Nachweisgrenze je Spezies; die Schwelle steht als --floor und ihre Empfindlichkeit ist unten gemessen\n",
    );
    out.push_str(
        "reservoir-zeuge: pscomppars st_met ([Fe/H]) gelesen — je Wirt skaliert die Regression die Metall-Elemente (C,O,N,S je H) um z = 10^[Fe/H]; H bleibt die Referenz\n",
    );
    out.push_str(
        "  C/O traegt pscomppars nicht (gemessen am NExScI-TAP-Schema 2026-09-05: keine C/O-, keine log R'HK-, keine XUV-Spalte in irgendeiner Tabelle) — C/O wird aus dem C/O-Zeugen gelesen\n",
    );
    out.push_str(&format!(
        "  Zeugen-Register {}: {} Zeilen ({} davon Budget-Analysen mit [C/H]/[O/H]/[Fe/H]; {} Wirte tragen einen Aktivitaets-/XUV-Zeugen, {} einen XUV-Fluss L_X/F_X)\n",
        witness_path, n_witness_rows, n_budget_rows, n_act_hosts, n_xuv_hosts
    ));
    out.push_str(
        "  C/O-Zeuge: hochaufgeloeste Wirts-Abundanzen [C/H]/[O/H]/[N/H] — Quellen: VizieR-Kataloge J/ApJS/225/32 (2016ApJS..225...32B) + J/ApJS/237/38 (2018ApJS..237...38B; abgerufen 2026-09-05, Wirt per RA/Dec kreuzidentifiziert, <~5\";) und die externe Literatur-Rueckmeldung (Brewer & Fischer 2018, Brewer et al. 2016, Mesa et al. 2019, Polanski et al. 2022)\n",
    );
    out.push_str(
        "  der C/O-Zeuge ersetzt C und O durch die gemessenen [C/H]/[O/H]; N traegt sein gemessenes [N/H] (sonst [Fe/H]); S hat in keiner geprueften Quelle eine Wirts-Abundanz und bleibt auf [Fe/H] — benannter Proxy. Die dex stehen auf der Katalog-eigenen solaren Skala; die Anwendung auf die Archiv-SOLAR_-Werte ist transparent\n",
    );
    out.push_str(
        "  Aktivitaets-Zeuge: log R'HK, S-index, L_X/F_X/L_X-L_bol und P_rot aus dem Zeugen-Register (externe Rueckmeldung + Brewer-Kataloge) je Wirt; Rotation st_rotp/v sin i zusaetzlich aus pscomppars. Ein Wert, den das Register nicht traegt, bleibt pending — nie geschaetzt\n",
    );
    out.push_str(
        "  der zweite Reinigungsschritt regressiert die Hit-Indikator-Spalte gegen das publizierte log R'HK der Wirte (XUV/Aktivitaets-Zeuge); die Photochemie-Re-Erklaerung (XUV-Fluss treibt SO2/CO2) ist ein nicht-Gleichgewichts-Modell und bleibt als solche pending\n",
    );
    out.push_str(
        "  das Gleichgewichts-Urteil (hit/praesent) bleibt das solare; die Reservoir-Regression und die C/O-Zeugen-Regression melden Bewegung, wenn eine Spezies die floor-Klassifikation wechselt\n",
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
        "erkannte Spezies ({}): mit Slot {}; ohne Modell-Daten {}\n",
        detected_species.len(),
        in_model.join(","),
        out_model.join(",")
    ));

    let mut host_ids: Vec<String> = Vec::new();
    let mut host_det: Vec<Vec<Detection>> = Vec::new();
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
        host_det[id].push(d.clone());
        rows.push((id, d.species.clone()));
    }

    let mut plan: Vec<Option<HostPlan>> = vec![None; host_ids.len()];
    let mut pending_reason: Vec<Option<String>> = vec![None; host_ids.len()];
    let mut reservoir_pending_hosts: Vec<String> = Vec::new();
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
        let mut claims: Vec<&String> = host_det[id]
            .iter()
            .filter_map(|d| d.pl_name.as_ref())
            .collect();
        claims.sort();
        claims.dedup();
        if claims.len() > 1 {
            let named: Vec<&str> = claims.iter().map(|s| s.as_str()).collect();
            pending_reason[id] = Some(format!(
                "Saat-Zeilen nennen mehrere Planeten: {}",
                named.join(", ")
            ));
            n_pending_multi += 1;
            continue;
        }
        let target: Option<&PlanetRow> = if let Some(pl) = claims.first() {
            planets.iter().find(|p| p.pl_name == **pl)
        } else if planets.len() == 1 {
            Some(&planets[0])
        } else {
            None
        };
        let Some(p) = target else {
            if let Some(pl) = claims.first() {
                let names: Vec<&str> = planets.iter().map(|p| p.pl_name.as_str()).collect();
                pending_reason[id] = Some(format!(
                    "Saat nennt {pl}, pscomppars fuehrt nur {}",
                    names.join(", ")
                ));
                n_pending_params += 1;
            } else {
                let names: Vec<&str> = planets.iter().map(|p| p.pl_name.as_str()).collect();
                pending_reason[id] = Some(format!(
                    "Attribution offen — {} transiting-Planeten: {}",
                    planets.len(),
                    names.join(", ")
                ));
                n_pending_multi += 1;
            }
            continue;
        };
        let Some(t_eq) = teq(p.teff, p.rad_solar * SUN_RADIUS_M, p.orbsmax_au * AU_M, 0.0) else {
            pending_reason[id] = Some("Teq nicht berechenbar".to_string());
            n_pending_domain += 1;
            continue;
        };
        let (frac, model) = if t_eq >= SULFUR_T_MIN {
            if !(SULFUR_T_MIN..=MODEL_T_MAX).contains(&t_eq) {
                pending_reason[id] = Some(format!(
                    "Teq {:.0} K ausserhalb der Modell-Domäne {}..{} K",
                    t_eq, SULFUR_T_MIN, MODEL_T_MAX
                ));
                n_pending_domain += 1;
                continue;
            }
            let Some(frac) = equilibrium_composition_sulfur(t_eq, P0_PA) else {
                pending_reason[id] = Some(format!(
                    "Gleichgewichts-Loeser konvergiert bei {:.0} K nicht",
                    t_eq
                ));
                n_pending_solver += 1;
                continue;
            };
            (frac, HostModel::Sulfur)
        } else {
            if t_eq < COOL_T_MIN {
                pending_reason[id] = Some(format!(
                    "Teq {:.0} K unter der Daten-Domäne {:.0}..{:.0} K (kondensiertes NH3/CO2/CH4/H2S pending)",
                    t_eq, COOL_T_MIN, SULFUR_T_MIN
                ));
                n_pending_domain += 1;
                continue;
            }
            let Some(eq) = equilibrium_composition_condensed(t_eq, P0_PA) else {
                pending_reason[id] = Some(format!(
                    "kondensations-bewusster Loeser konvergiert bei {:.0} K nicht (oder Kondensation unter 298.15 K nicht beurteilbar)",
                    t_eq
                ));
                n_pending_solver += 1;
                continue;
            };
            let model = HostModel::Cool {
                condensed_h2o_moles: eq.h2o_condensed_moles,
                vapor_pa: eq.h2o_vapor_pa,
                sat_pa: eq.h2o_sat_pa,
            };
            (eq.frac, model)
        };
        let frac_res = match p.feh {
            Some(feh) => {
                let r = if t_eq >= SULFUR_T_MIN {
                    equilibrium_composition_sulfur_scaled(t_eq, P0_PA, feh)
                } else {
                    equilibrium_composition_condensed_scaled(t_eq, P0_PA, feh).map(|eq| eq.frac)
                };
                if r.is_none() {
                    reservoir_pending_hosts.push(host.clone());
                }
                r
            }
            None => None,
        };
        plan[id] = Some(HostPlan {
            pl_name: p.pl_name.clone(),
            teq_k: t_eq,
            frac,
            frac_res,
            feh: p.feh,
            st_rotp_days: p.st_rotp_days,
            st_vsin_kms: p.st_vsin_kms,
            model,
        });
    }

    // Measured host-abundance regression (the C/O witness): every sourced
    // analysis row is its own equilibrium run against the host's measured
    // [C/H]/[O/H]/[N/H] budget. S carries no host measurement and rides the
    // row's own [Fe/H] — a named proxy. A row whose solver refuses stays
    // pending (counted), never a fabricated budget.
    let mut witness_runs: Vec<Vec<MeasuredRun>> = vec![Vec::new(); host_ids.len()];
    let mut n_co_hosts = 0usize;
    let mut n_co_rows = 0usize;
    let mut n_co_refused = 0usize;
    let mut co_refused_hosts: Vec<String> = Vec::new();
    let mut co_no_budget_hosts: Vec<String> = Vec::new();
    for id in 0..host_ids.len() {
        let host = &host_ids[id];
        let Some(hp) = &plan[id] else {
            continue;
        };
        let Some(rows) = witness.get(host) else {
            continue;
        };
        let mut any_run = false;
        for w in rows {
            let Some((ch, oh, nh, feh)) = w.budget() else {
                continue;
            };
            match measured_budget_frac(hp.teq_k, ch, oh, nh, feh) {
                Some(frac) => {
                    witness_runs[id].push(MeasuredRun {
                        analysis: w.analysis.clone(),
                        ch,
                        oh,
                        feh,
                        c_over_o: c_over_o_solar(ch, oh),
                        frac,
                    });
                    n_co_rows += 1;
                    any_run = true;
                }
                None => n_co_refused += 1,
            }
        }
        if any_run {
            n_co_hosts += 1;
        } else if !rows.is_empty() {
            if rows.iter().any(|w| w.budget().is_some()) {
                co_refused_hosts.push(host.clone());
            } else {
                co_no_budget_hosts.push(host.clone());
            }
        }
    }

    let mut n_hit_hosts = 0usize;
    let mut n_eq_hosts = 0usize;
    let mut n_oom_only_hosts = 0usize;
    let mut n_reservoir_pending = 0usize;
    let mut n_moved_hosts = 0usize;
    let mut n_moved_species = 0usize;
    let mut n_co_moved_hosts = 0usize;
    let mut n_co_moved_species = 0usize;
    let mut lines: Vec<String> = Vec::new();
    for id in 0..host_ids.len() {
        let Some(hp) = &plan[id] else {
            continue;
        };
        let host = &host_ids[id];
        let mut hit_species: Vec<String> = Vec::new();
        let mut present_species: Vec<String> = Vec::new();
        let mut oom: Vec<String> = Vec::new();
        for d in &host_det[id] {
            let name = d.species.as_str();
            match slot_by_name.get(name) {
                Some(slot) => {
                    let entry = format!(
                        "{} (Slot {}; gleichgewichts-Anteil {:.3e})",
                        name, slot, hp.frac[*slot]
                    );
                    if hp.frac[*slot] >= floor {
                        present_species.push(entry);
                    } else {
                        hit_species.push(entry);
                    }
                }
                None => oom.push(name.to_string()),
            }
        }
        oom.sort();
        oom.dedup();
        if hit_species.is_empty() && present_species.is_empty() {
            n_oom_only_hosts += 1;
            lines.push(format!(
                "  {host}: {}  Teq {:.0} K  — nur ohne-Modell-Daten-Detektionen ({}): Gleichgewicht nicht prüfbar (pending)",
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
            "  {host}: {}  Teq {:.0} K  [Fe/H] {}  VERDICT {word}",
            hp.pl_name,
            hp.teq_k,
            match hp.feh {
                Some(f) => format!("{f:+.2}"),
                None => "pending (st_met absent)".to_string(),
            }
        );
        if !oom.is_empty() {
            block.push_str(&format!(
                "  (ohne Modell-Daten benannt: {} — pending)",
                oom.join(",")
            ));
        }
        block.push('\n');
        match &hp.frac_res {
            Some(frac_res) => {
                let feh = hp.feh.unwrap();
                let z = 10f64.powf(feh);
                let mut moved: Vec<String> = Vec::new();
                for d in &host_det[id] {
                    let Some(slot) = slot_by_name.get(&d.species) else {
                        continue;
                    };
                    let solar_word = if hp.frac[*slot] >= floor {
                        "equilibrium-present"
                    } else {
                        "disequilibrium"
                    };
                    let res_word = if frac_res[*slot] >= floor {
                        "equilibrium-present"
                    } else {
                        "disequilibrium"
                    };
                    if solar_word != res_word {
                        moved.push(format!(
                            "{}: {} -> {} (reservoir-Anteil {:.3e})",
                            d.species, solar_word, res_word, frac_res[*slot]
                        ));
                    }
                }
                moved.sort();
                moved.dedup();
                if moved.is_empty() {
                    block.push_str(&format!(
                        "      reservoir: [Fe/H] {feh:+.2} (z {z:.3}) — keine Klassifikations-Aenderung\n"
                    ));
                } else {
                    n_moved_hosts += 1;
                    n_moved_species += moved.len();
                    block.push_str(&format!(
                        "      reservoir: [Fe/H] {feh:+.2} (z {z:.3}) — Spezies-Bewegung: {}\n",
                        moved.join("; ")
                    ));
                }
            }
            None => {
                if hp.feh.is_some() {
                    n_reservoir_pending += 1;
                    block.push_str(&format!(
                        "      reservoir: [Fe/H] {:+.2} — Reservoir-Loeser verweigert (pending)\n",
                        hp.feh.unwrap()
                    ));
                } else {
                    block.push_str(
                        "      reservoir: pscomppars traegt kein st_met — Reservoir pending\n",
                    );
                }
            }
        }
        for mr in &witness_runs[id] {
            let mut moved: Vec<String> = Vec::new();
            for d in &host_det[id] {
                let Some(slot) = slot_by_name.get(&d.species) else {
                    continue;
                };
                let solar_word = if hp.frac[*slot] >= floor {
                    "equilibrium-present"
                } else {
                    "disequilibrium"
                };
                let meas_word = if mr.frac[*slot] >= floor {
                    "equilibrium-present"
                } else {
                    "disequilibrium"
                };
                if solar_word != meas_word {
                    moved.push(format!(
                        "{}: {} -> {} (C/O-Zeuge-Anteil {:.3e})",
                        d.species, solar_word, meas_word, mr.frac[*slot]
                    ));
                }
            }
            moved.sort();
            moved.dedup();
            if moved.is_empty() {
                block.push_str(&format!(
                    "      C/O-Zeuge {}: [C/H] {:+.2} [O/H] {:+.2} (S@[Fe/H] {:+.2}) C/O {:.3} — keine Klassifikations-Aenderung\n",
                    mr.analysis, mr.ch, mr.oh, mr.feh, mr.c_over_o
                ));
            } else {
                n_co_moved_hosts += 1;
                n_co_moved_species += moved.len();
                block.push_str(&format!(
                    "      C/O-Zeuge {}: [C/H] {:+.2} [O/H] {:+.2} (S@[Fe/H] {:+.2}) C/O {:.3} — Spezies-Bewegung: {}\n",
                    mr.analysis, mr.ch, mr.oh, mr.feh, mr.c_over_o, moved.join("; ")
                ));
            }
        }
        let mut act_lines: Vec<String> = Vec::new();
        if let Some(rows) = witness.get(host) {
            for w in rows {
                if w.activity_number() || w.pending_activity() {
                    act_lines.push(fmt_activity_witness(w));
                }
            }
        }
        let mut rot_part = match hp.st_rotp_days {
            Some(p) => format!("st_rotp {p:.1} d (pscomppars)"),
            None => "st_rotp pending (pscomppars leer)".to_string(),
        };
        if let Some(v) = hp.st_vsin_kms {
            rot_part.push_str(&format!(" | v sin i {v:.1} km/s (pscomppars)"));
        }
        if act_lines.is_empty() {
            block.push_str(&format!("      Aktivitaets-Zeuge: {rot_part}\n"));
        } else {
            for l in &act_lines {
                block.push_str(&format!("      Aktivitaets-Zeuge: {l}\n"));
            }
            block.push_str(&format!("      Rotation-Zeuge: {rot_part}\n"));
        }
        if let Some(rows) = witness.get(host) {
            for w in rows {
                if w.budget().is_some() {
                    continue;
                }
                let mut vparts: Vec<String> = Vec::new();
                if let Some(v) = w.co {
                    vparts.push(format!("C/O {v:.2}"));
                }
                if let (Some(ch), Some(oh)) = (w.ch, w.oh) {
                    vparts.push(format!("[C/H] {ch:+.2} [O/H] {oh:+.2}"));
                }
                if let Some(feh) = w.feh {
                    vparts.push(format!("[Fe/H] {feh:+.2}"));
                }
                let abundance_pending: Vec<&str> = w
                    .pending
                    .iter()
                    .map(|t| t.as_str())
                    .filter(|t| matches!(*t, "ch" | "oh" | "nh" | "co" | "feh"))
                    .collect();
                if vparts.is_empty() && abundance_pending.is_empty() {
                    continue;
                }
                let mut s = format!("      C/O-Zeuge {}: {}", w.analysis, vparts.join(" | "));
                if !abundance_pending.is_empty() {
                    if w.activity_number() {
                        s.push_str(&format!(" — nur pending: {}", abundance_pending.join(",")));
                    } else {
                        s.push_str(&format!(" — pending: {}", abundance_pending.join(",")));
                        if let Some(n) = &w.note {
                            s.push_str(&format!(" — {n}"));
                        }
                    }
                } else if let Some(n) = &w.note {
                    s.push_str(&format!(" — {n}"));
                }
                block.push_str(&s);
                block.push('\n');
            }
        }
        for s in &hit_species {
            block.push_str(&format!("      disequilibrium  {s}\n"));
        }
        for s in &present_species {
            block.push_str(&format!("      equilibrium    {s}\n"));
        }
        if let HostModel::Cool {
            condensed_h2o_moles,
            vapor_pa,
            sat_pa,
        } = &hp.model
        {
            if *condensed_h2o_moles > 0.0 {
                block.push_str(&format!(
                    "      kondensat: H2O(l) praesent ({:.3e} mol je H-Atom); Gas-H2O bei Sättigung {:.3e} Pa\n",
                    condensed_h2o_moles, vapor_pa
                ));
            } else if let Some(p_sat) = sat_pa {
                block.push_str(&format!(
                    "      kondensat: einphasiges Gas — H2O(l) untersättigt (p_H2O {:.3e} Pa < p_sat {:.3e} Pa)\n",
                    vapor_pa, p_sat
                ));
            } else {
                block.push_str(&format!(
                    "      kondensat: einphasiges Gas — H2O(l)-Sättigung unter {:.0} K nicht im NIST-Fit; p_H2O {:.3e} Pa < Dreipunkt-Bindung {:.0} Pa\n",
                    WATER_LIQ_T_MIN, vapor_pa, WATER_P_TRIPLE_PA
                ));
            }
        }
        lines.push(block);
    }

    let host_count = host_ids.len();
    out.push_str(&format!(
        "Wirte: {} gesamt | disequilibrium-hit {} | equilibrium-present {} | ohne-Modell-Daten-only {} | pending {} (params-fehlend {}, attribution-offen {}, Domäne {}, Loeser {})\n",
        host_count, n_hit_hosts, n_eq_hosts, n_oom_only_hosts,
        n_pending_params + n_pending_multi + n_pending_domain + n_pending_solver,
        n_pending_params, n_pending_multi, n_pending_domain, n_pending_solver
    ));
    out.push_str(&format!(
        "Reservoir [Fe/H]-Regression: {} Wirte gelesen | {} ohne st_met (pending) | {} Reservoir-Loeser verweigert (pending) | Klassifikations-Bewegung: {} Wirte, {} Spezies\n",
        host_count,
        plan.iter()
            .filter(|p| p.as_ref().map(|hp| hp.feh.is_none()).unwrap_or(false))
            .count(),
        n_reservoir_pending,
        n_moved_hosts,
        n_moved_species
    ));
    out.push_str(&format!(
        "C/O-Zeugen-Regression (gemessene [C/H]/[O/H]/[N/H]-Haushalte): {} Wirte gelesen ({} Analyse-Zeilen) | {} Zeilen Loeser verweigert (pending) | Klassifikations-Bewegung: {} Wirte, {} Spezies\n",
        n_co_hosts, n_co_rows, n_co_refused, n_co_moved_hosts, n_co_moved_species
    ));
    if !co_refused_hosts.is_empty() {
        out.push_str(&format!(
            "  C/O-Loeser verweigert (pending): {}\n",
            co_refused_hosts.join(", ")
        ));
    }
    if !co_no_budget_hosts.is_empty() {
        out.push_str(&format!(
            "  C/O-Regression nicht anwendbar (Zeuge vorhanden, aber keine [C/H]+[O/H]+[Fe/H]-Budget-Zeile im Register): {}\n",
            co_no_budget_hosts.join(", ")
        ));
    }
    for id in 0..host_ids.len() {
        if let Some(reason) = &pending_reason[id] {
            out.push_str(&format!(
                "  {host}: pending — {reason} (detektiert: {det})\n",
                host = host_ids[id],
                det = host_det[id]
                    .iter()
                    .map(|d| d.species.clone())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
    }
    if !reservoir_pending_hosts.is_empty() {
        out.push_str(&format!(
            "  Reservoir-pending (Loeser verweigert): {}\n",
            reservoir_pending_hosts.join(", ")
        ));
    }
    out.push('\n');
    for l in &lines {
        out.push_str(l);
        out.push('\n');
    }

    let possible: Vec<bool> = plan
        .iter()
        .enumerate()
        .map(|(id, hp)| {
            hp.is_some()
                && host_det[id]
                    .iter()
                    .any(|d| slot_by_name.contains_key(&d.species))
        })
        .collect();
    let possible_count = possible.iter().filter(|b| **b).count();
    let observed = possible
        .iter()
        .enumerate()
        .filter(|(id, p)| {
            **p && host_det[*id].iter().any(|d| {
                slot_by_name
                    .get(&d.species)
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
                        && host_det[*id].iter().any(|d| {
                            slot_by_name
                                .get(&d.species)
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

    // Hit-vs-reservoir correlation: host [Fe/H] against the solar hit
    // indicator (>=1 in-model species below the floor). Hosts without an
    // in-model species or without st_met carry no pair.
    let mut feh_list: Vec<f64> = Vec::new();
    let mut hit_list: Vec<f64> = Vec::new();
    for id in 0..host_ids.len() {
        let Some(hp) = &plan[id] else {
            continue;
        };
        if !possible[id] {
            continue;
        }
        let Some(feh) = hp.feh else {
            continue;
        };
        let hit = host_det[id].iter().any(|d| {
            slot_by_name
                .get(&d.species)
                .map(|slot| hp.frac[*slot] < floor)
                .unwrap_or(false)
        });
        feh_list.push(feh);
        hit_list.push(if hit { 1.0 } else { 0.0 });
    }
    let n_pairs = feh_list.len();
    let pearson = |x: &[f64], y: &[f64]| -> f64 {
        let n = x.len() as f64;
        let mx = x.iter().sum::<f64>() / n;
        let my = y.iter().sum::<f64>() / n;
        let cov: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
        let vx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
        let vy: f64 = y.iter().map(|a| (a - my) * (a - my)).sum();
        if vx <= 0.0 || vy <= 0.0 {
            return f64::NAN;
        }
        cov / (vx * vy).sqrt()
    };
    if n_pairs < 3 {
        out.push_str(&format!(
            "Reservoir-Korrelation: {} Wirte mit [Fe/H]-Zeuge — zu wenige Paare (pending)\n",
            n_pairs
        ));
    } else {
        let r_obs = pearson(&feh_list, &hit_list);
        if !r_obs.is_finite() {
            out.push_str(
                "Reservoir-Korrelation: [Fe/H]-Spalte konstant — Pearson nicht definiert (pending)\n",
            );
        } else {
            let mut rng_corr = CORR_RNG_SEED;
            let mut hit_perm = hit_list.clone();
            let mut over_abs = 0usize;
            for _ in 0..trials {
                shuffle(&mut hit_perm, &mut rng_corr);
                let r = pearson(&feh_list, &hit_perm);
                if r.is_finite() && r.abs() >= r_obs.abs() {
                    over_abs += 1;
                }
            }
            let p_corr = over_abs as f64 / trials as f64;
            let mean = |v: &[f64]| -> f64 {
                let n = v.len() as f64;
                if n == 0.0 {
                    f64::NAN
                } else {
                    v.iter().sum::<f64>() / n
                }
            };
            let hit_feh: Vec<f64> = feh_list
                .iter()
                .zip(&hit_list)
                .filter(|(_, h)| **h == 1.0)
                .map(|(f, _)| *f)
                .collect();
            let pres_feh: Vec<f64> = feh_list
                .iter()
                .zip(&hit_list)
                .filter(|(_, h)| **h == 0.0)
                .map(|(f, _)| *f)
                .collect();
            out.push_str(&format!(
                "Reservoir-Korrelation: host [Fe/H] gegen Hit-Indikator, n = {} Wirte (solar-Klassifikation)\n",
                n_pairs
            ));
            out.push_str(&format!(
                "  Pearson r = {r_obs:+.3} | Permutations-Null ({} Ziehungen, |r_perm| >= |r|): P = {p_corr:.4}\n",
                trials
            ));
            out.push_str(&format!(
                "  Hit-Wirte ({}): mean [Fe/H] {:+.3} | equilibrium-present ({}): mean [Fe/H] {:+.3}\n",
                hit_feh.len(),
                mean(&hit_feh),
                pres_feh.len(),
                mean(&pres_feh)
            ));
        }
    }

    // Second cleaning step — the XUV/activity regression. Host coordinate: the
    // mean of the published numeric log R'HK values the register carries for
    // the host (each value printed above at its read site). Only hosts that are
    // evaluable (in-model species) AND carry a numeric log R'HK witness enter;
    // hosts without a measured value stay pending, never estimated.
    let mut act_host: Vec<String> = Vec::new();
    let mut act_x: Vec<f64> = Vec::new();
    let mut act_n: Vec<usize> = Vec::new();
    let mut act_hit: Vec<f64> = Vec::new();
    let mut xuv_hosts: Vec<(String, f64)> = Vec::new();
    let mut act_pending_hosts: Vec<String> = Vec::new();
    for id in 0..host_ids.len() {
        let host = &host_ids[id];
        if !possible[id] {
            continue;
        }
        let Some(rows) = witness.get(host) else {
            act_pending_hosts.push(host.clone());
            continue;
        };
        let logrhk: Vec<f64> = rows.iter().filter_map(|w| w.logrhk).collect();
        if logrhk.is_empty() {
            act_pending_hosts.push(host.clone());
        } else {
            let m = logrhk.iter().sum::<f64>() / logrhk.len() as f64;
            let hit = host_det[id].iter().any(|d| {
                slot_by_name
                    .get(&d.species)
                    .map(|slot| plan[id].as_ref().unwrap().frac[*slot] < floor)
                    .unwrap_or(false)
            });
            act_host.push(host.clone());
            act_x.push(m);
            act_n.push(logrhk.len());
            act_hit.push(if hit { 1.0 } else { 0.0 });
        }
        for w in rows {
            let xv = [w.lx, w.fx, w.lxlbol].iter().find_map(|o| *o);
            if let Some(v) = xv {
                xuv_hosts.push((host.clone(), v));
            }
        }
    }
    let n_act = act_x.len();
    out.push_str(&format!(
        "Aktivitaets-Regression (zweiter Reinigungsschritt): {} Wirte mit numerischem log R'HK-Zeuge + wertbarer Detektion\n",
        n_act
    ));
    if n_act < 3 {
        out.push_str(&format!("  {} Wirte — zu wenige Paare (pending)\n", n_act));
    } else {
        let r_obs = pearson_r(&act_x, &act_hit);
        if !r_obs.is_finite() {
            out.push_str("  log R'HK-Spalte konstant — Pearson nicht definiert (pending)\n");
        } else {
            let mut rng_act = CORR_RNG_SEED.wrapping_add(0xACE7);
            let mut hit_perm = act_hit.clone();
            let mut over_abs = 0usize;
            for _ in 0..trials {
                shuffle(&mut hit_perm, &mut rng_act);
                let r = pearson_r(&act_x, &hit_perm);
                if r.is_finite() && r.abs() >= r_obs.abs() {
                    over_abs += 1;
                }
            }
            let p_act = over_abs as f64 / trials as f64;
            let hit_x: Vec<f64> = act_x
                .iter()
                .zip(&act_hit)
                .filter(|(_, h)| **h == 1.0)
                .map(|(x, _)| *x)
                .collect();
            let pres_x: Vec<f64> = act_x
                .iter()
                .zip(&act_hit)
                .filter(|(_, h)| **h == 0.0)
                .map(|(x, _)| *x)
                .collect();
            out.push_str(&format!(
                "  Pearson r = {r_obs:+.3} | Permutations-Null ({} Ziehungen, |r_perm| >= |r|): P = {p_act:.4}\n",
                trials
            ));
            out.push_str(&format!(
                "  Hit-Wirte ({}): mean log R'HK {:+.2} | equilibrium-present ({}): mean log R'HK {:+.2}  (log R'HK aktiver = weniger negativ)\n",
                hit_x.len(),
                mean_v(&hit_x),
                pres_x.len(),
                mean_v(&pres_x)
            ));
            let mut rows_txt: Vec<String> = Vec::new();
            for (h, (x, n)) in act_host.iter().zip(act_x.iter().zip(&act_n)) {
                rows_txt.push(format!("{h} {x:.2} ({n} Werte)"));
            }
            out.push_str(&format!("  Wirte: {}\n", rows_txt.join(" | ")));
        }
    }
    if !act_pending_hosts.is_empty() {
        out.push_str(&format!(
            "  Aktivitaets-pending (wertbar, kein numerischer log R'HK-Zeuge): {}\n",
            act_pending_hosts.join(", ")
        ));
    }
    let n_xuv_distinct = xuv_hosts
        .iter()
        .map(|(h, _)| h.as_str())
        .collect::<HashSet<_>>()
        .len();
    out.push_str(&format!(
        "XUV-Fluss-Zeugen (L_X/F_X/L_X-L_bol numerisch im Register): {} Wirte ({} Messwerte) — {}\n",
        n_xuv_distinct,
        xuv_hosts.len(),
        if xuv_hosts.is_empty() {
            "keine".to_string()
        } else {
            xuv_hosts
                .iter()
                .map(|(h, v)| format!("{h} {v:.1e}"))
                .collect::<Vec<_>>()
                .join(" | ")
        }
    ));
    if n_xuv_distinct < 4 {
        out.push_str(
            "  XUV-vs-Hit-Regression: zu wenige XUV-Wirte (pending; die Photochemie-Re-Erklaerung braucht einen XUV-Fluss je Wirt)\n",
        );
    }
    out.push_str(
        "  Gleichgewichts-Urteile bewegen sich unter den Aktivitaets-Zeugen nicht: thermochemisches Gleichgewicht hat keinen Aktivitaets-Kanal — die gemessene Regression ist die statistische Fassung; die Photochemie-Re-Erklaerung (XUV treibt SO2/CO2) bleibt ein nicht-Gleichgewichts-Modell (pending)\n",
    );

    // Full 48-host census: every JWST-transmission host of the survey gets a
    // row. The 30 with a published detection carry the probe verdict; the 18
    // without a published detection are explicit non-detection rows (0 honored
    // — the observed absence of a published species detection is the measured
    // fact, never a fabricated species).
    let census_detected: Vec<&CensusHost> =
        census.iter().filter(|c| c.class == "detection").collect();
    let census_nd: Vec<&CensusHost> = census
        .iter()
        .filter(|c| c.class == "non_detection")
        .collect();
    let mut census_word: Vec<String> = Vec::new();
    let mut census_missing: Vec<String> = Vec::new();
    for c in &census_detected {
        let Some(id) = host_ids.iter().position(|h| h == &c.hostname) else {
            census_missing.push(c.hostname.clone());
            census_word.push(format!(
                "{}: nicht in der Detektions-Saat (Register-Konflikt)",
                c.hostname
            ));
            continue;
        };
        let word = if let Some(hp) = &plan[id] {
            let has_in_model = host_det[id]
                .iter()
                .any(|d| slot_by_name.contains_key(&d.species));
            if !has_in_model {
                "ohne-Modell-Daten-only (Gleichgewicht nicht prüfbar — pending)".to_string()
            } else if host_det[id].iter().any(|d| {
                slot_by_name
                    .get(&d.species)
                    .map(|slot| hp.frac[*slot] < floor)
                    .unwrap_or(false)
            }) {
                "disequilibrium-hit".to_string()
            } else {
                "equilibrium-present".to_string()
            }
        } else if let Some(r) = &pending_reason[id] {
            format!("pending — {r}")
        } else {
            "pending".to_string()
        };
        census_word.push(format!("{}: {}", c.hostname, word));
    }
    out.push_str(&format!(
        "\nZensus ({} Wirte): {} mit publizierter Detektion, {} ohne publizierte Detektion (non-detection, 0 honored)\n",
        census.len(),
        census_detected.len(),
        census_nd.len()
    ));
    for w in &census_word {
        out.push_str(&format!("  {w}\n"));
    }
    for c in &census_nd {
        let mut s = format!(
            "  {}: non-detection (0 honored) — keine publizierte Spezies-Detektion",
            c.hostname
        );
        if let Some(note) = &c.note {
            s.push_str(&format!(" ({note})"));
        }
        out.push_str(&s);
        out.push('\n');
    }
    if !census_missing.is_empty() {
        out.push_str(&format!(
            "  Zensus-Register-Konflikt (detection-Klasse ohne Saat-Eintrag): {}\n",
            census_missing.join(", ")
        ));
    }

    std::fs::write(out_path, &out).map_err(|e| format!("{out_path}: {e}"))?;
    println!("{out}");
    Ok(())
}
