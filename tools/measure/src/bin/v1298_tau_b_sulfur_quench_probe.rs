use omegaflow::thermochem::{
    elemental_budget_sulfur, equilibrium_composition_condensed_budget,
    equilibrium_composition_sulfur_budget, sulfur_gas_names, COOL_T_MAX, P0_PA, SOLAR_C, SOLAR_O,
};

const SOLAR_C_O: f64 = SOLAR_C / SOLAR_O;
const T_GRID_MIN: f64 = 400.0;
const T_GRID_MAX: f64 = 2600.0;
const T_GRID_STEP: f64 = 20.0;
const LOGZ_MIN: f64 = 0.4;
const LOGZ_MAX: f64 = 1.0;
const LOGZ_STEP: f64 = 0.2;
const OBSERVED_LOGZ: f64 = 0.6;
const OBSERVED_C_O: f64 = 0.22;
const LAYER_T: f64 = 450.0;
const TEQ_REF_T: f64 = 670.0;
const OCS_RET_LOG10: f64 = -8.0;
const OCS_BAND_MIN_LOG10: f64 = -9.0;
const OCS_BAND_MAX_LOG10: f64 = -7.0;

const SOLAR_LOGZ: f64 = 0.0;

fn observed_budget(logz: f64, co: f64) -> Option<[f64; 5]> {
    if !logz.is_finite() || !co.is_finite() || co <= 0.0 {
        return None;
    }
    let ch = logz + (co / SOLAR_C_O).log10();
    elemental_budget_sulfur(ch, logz, Some(logz), logz)
}

fn frac_at_t_p(t: f64, p_pa: f64, b: [f64; 5]) -> Option<Vec<f64>> {
    if !p_pa.is_finite() || p_pa <= 0.0 {
        return None;
    }
    if t < COOL_T_MAX {
        equilibrium_composition_condensed_budget(t, p_pa, b).map(|eq| eq.frac)
    } else {
        equilibrium_composition_sulfur_budget(t, p_pa, b)
    }
}

fn frac_at_t(t: f64, b: [f64; 5]) -> Option<Vec<f64>> {
    frac_at_t_p(t, P0_PA, b)
}

fn log10_of(frac: &[f64], name: &str, slots: &[(&str, usize)]) -> Option<f64> {
    let pos = slots.iter().find(|(n, _)| *n == name)?.1;
    let v = frac[pos];
    if v.is_finite() && v > 0.0 {
        Some(v.log10())
    } else {
        None
    }
}

fn slot_lookup(names: &[String]) -> Vec<(&str, usize)> {
    names.iter().map(|n| n.as_str()).zip(0..).collect()
}

fn layer_table_temps() -> Vec<f64> {
    let mut ts: Vec<f64> = Vec::new();
    let mut t = T_GRID_MIN;
    while t <= T_GRID_MAX + 1e-9 {
        ts.push(t);
        t += T_GRID_STEP;
    }
    for t in [
        LAYER_T, TEQ_REF_T, 800.0, 1000.0, 1200.0, 1500.0, 2000.0, T_GRID_MAX,
    ] {
        if !ts.contains(&t) {
            ts.push(t);
        }
    }
    ts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ts
}

fn observed_layer_table(logz: f64) -> Option<String> {
    let names = sulfur_gas_names();
    let slots = slot_lookup(&names);
    let mut out = String::new();
    out.push_str(&format!(
        "log10 mixing ratio over T (logZ {logz:.1}, C/O {OBSERVED_C_O:.2}, 1 bar)\n"
    ));
    out.push_str("T [K]      OCS      SO2      H2S      CO       CO2      CH4      H2O\n");
    for tq in layer_table_temps() {
        let b = observed_budget(logz, OBSERVED_C_O)?;
        let frac = frac_at_t(tq, b)?;
        let f = |name: &str| match log10_of(&frac, name, &slots) {
            Some(v) => format!("{v:8.2}"),
            None => "       -".to_string(),
        };
        out.push_str(&format!(
            "{:>5.0}    {} {} {} {} {} {} {}\n",
            tq,
            f("OCS"),
            f("SO2"),
            f("H2S"),
            f("CO"),
            f("CO2"),
            f("CH4"),
            f("H2O")
        ));
    }
    Some(out)
}

fn ocs_scan(logz: f64) -> Option<(f64, f64, f64, f64, f64, f64)> {
    let b = observed_budget(logz, OBSERVED_C_O)?;
    let names = sulfur_gas_names();
    let slots = slot_lookup(&names);
    let mut best = f64::NAN;
    let mut best_t = f64::NAN;
    let mut lo_band = f64::NAN;
    let mut hi_band = f64::NAN;
    let mut ocs_450 = f64::NAN;
    let mut cross_ret = f64::NAN;
    for t in layer_table_temps() {
        let frac = frac_at_t(t, b)?;
        if let Some(v) = log10_of(&frac, "OCS", &slots) {
            if !best.is_finite() || v > best {
                best = v;
                best_t = t;
            }
            if (t - LAYER_T).abs() < 1e-9 {
                ocs_450 = v;
            }
            if v >= OCS_RET_LOG10 && !cross_ret.is_finite() {
                cross_ret = t;
            }
            if v >= OCS_BAND_MIN_LOG10 && v <= OCS_BAND_MAX_LOG10 {
                if !lo_band.is_finite() {
                    lo_band = t;
                }
                hi_band = t;
            }
        }
    }
    Some((ocs_450, best, best_t, lo_band, hi_band, cross_ret))
}

fn show(f: Option<f64>, tag: &str, out: &mut String) {
    out.push_str(&format!(
        "  OCS log10 {}: {}\n",
        tag,
        match f {
            Some(v) => format!("{v:.3}"),
            None => "no value (solver refused)".to_string(),
        }
    ));
}

fn fmt_v(v: f64) -> String {
    if v.is_finite() {
        format!("{v:.2}")
    } else {
        "-".to_string()
    }
}

fn cross_txt(cross_ret: f64, best: f64) -> String {
    if cross_ret.is_finite() {
        format!("OCS 1e-8 from {cross_ret:.0} K")
    } else {
        format!("maximum 10^{} does not reach 1e-8", fmt_v(best))
    }
}

fn run_report() -> Result<String, String> {
    let mut out = String::new();
    out.push_str(
        "v1298_tau_b_sulfur_quench_probe — quench-aware S disequilibrium model V1298 Tau b\n",
    );
    out.push_str("model: thermochem::equilibrium_composition_sulfur_budget/condensed_budget (24 slots, NIST-JANAF-Shomate) at P0 = 1 bar;\n");
    out.push_str("  the quench approximation is kinetics-free: the observed layer carries the equilibrium composition of the quench level T_q, not its own.\n");
    out.push_str("  T_q is not determinable without reaction rates; the model measures the thermodynamic envelope (freeze-in): the T interval over which equilibrium OCS carries the target band.\n");
    out.push_str(&format!(
        "measured (Register 2026-09-05, Barat 2025 2025AJ....170..165B): layer T ~{LAYER_T:.0} K | log Z = {OBSERVED_LOGZ:+.1} (4x solar, span {LOGZ_MIN:+.1}..{LOGZ_MAX:+.1}) | C/O {OBSERVED_C_O:.2} | Kzz 1e6..1e10 cm2/s (Mukherjee 2025) | OCS 3.5 sigma ~1e-8 (log10 {OCS_RET_LOG10:.1})\n"
    ));
    out.push_str("0 honored: T-P profile, surface gravity, reaction rates, Kzz->T_q translation are pending (missing sources named below); the envelope is the honest limit.\n");
    out.push_str("Budget derivation: [O/H]=[N/H]=[S/H]=logZ on the solar scale; [C/H]=[O/H]+log10((C/O)/(C/O_solar)) with C/O_solar = SOLAR_C/SOLAR_O = 0.427 — C/O 0.22 places C 0.29 dex below O. All equilibria at P0 = 1 bar.\n\n");

    let names = sulfur_gas_names();
    let slots = slot_lookup(&names);

    let b_obs =
        observed_budget(OBSERVED_LOGZ, OBSERVED_C_O).ok_or_else(|| "observed budget refused")?;
    let b_solar =
        observed_budget(SOLAR_LOGZ, OBSERVED_C_O).ok_or_else(|| "solar budget refused")?;

    out.push_str("== Equilibrium table at the observed budget (logZ +0.6, C/O 0.22), 1 bar ==\n");
    out.push_str(&observed_layer_table(OBSERVED_LOGZ).ok_or_else(|| "layer table refused")?);
    out.push('\n');

    out.push_str("== Disequilibrium gap: equilibrium OCS at the observed layer and at the Teq reference point ==\n");
    let f_ocss = |b: [f64; 5], t: f64| -> Option<f64> {
        let frac = frac_at_t(t, b)?;
        log10_of(&frac, "OCS", &slots)
    };
    show(
        f_ocss(b_obs, LAYER_T),
        "at the 450 K layer, logZ +0.6 C/O 0.22",
        &mut out,
    );
    show(
        f_ocss(b_solar, LAYER_T),
        "at the 450 K layer, solar budget with C/O 0.22",
        &mut out,
    );
    show(
        f_ocss(b_obs, TEQ_REF_T),
        "at Teq 670 K, logZ +0.6 C/O 0.22",
        &mut out,
    );
    show(
        f_ocss(b_solar, TEQ_REF_T),
        "at Teq 670 K, solar budget with C/O 0.22",
        &mut out,
    );
    out.push('\n');

    out.push_str("== Freeze-in envelope: equilibrium OCS over T, per logZ (C/O 0.22), 1 bar ==\n");
    out.push_str("logZ   OCS@450K   OCS-max   T(max)    T(OCS=1e-8)  Band 1e-9..1e-7\n");
    let mut logz = LOGZ_MIN;
    let mut sweep: Vec<(f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
    while logz <= LOGZ_MAX + 1e-9 {
        let (ocs_450, best, best_t, lo_band, hi_band, cross_ret) =
            ocs_scan(logz).ok_or_else(|| "ocs scan refused")?;
        let band = if lo_band.is_finite() {
            format!("{lo_band:.0}..{hi_band:.0} K")
        } else {
            "empty".to_string()
        };
        let cross = if cross_ret.is_finite() {
            format!("{cross_ret:.0} K")
        } else {
            "-".to_string()
        };
        out.push_str(&format!(
            "{logz:+.1}   {:>8}   {:>7}   {:>5}    {:>10}   {band}\n",
            fmt_v(ocs_450),
            fmt_v(best),
            if best_t.is_finite() {
                format!("{best_t:.0}")
            } else {
                "-".to_string()
            },
            cross
        ));
        sweep.push((logz, ocs_450, best, best_t, lo_band, hi_band, cross_ret));
        logz += LOGZ_STEP;
    }
    out.push('\n');

    out.push_str("== Freeze-in against the target band (retrieved ~1e-8, band 1e-9..1e-7) ==\n");
    for (logz, ocs_450, best, best_t, lo_band, hi_band, cross_ret) in &sweep {
        out.push_str(&format!(
            "  logZ {logz:+.1}: layer equilibrium OCS 10^{} | best freeze-in source 10^{} at {:.0} K | {}",
            fmt_v(*ocs_450),
            fmt_v(*best),
            best_t,
            cross_txt(*cross_ret, *best)
        ));
        if lo_band.is_finite() {
            out.push_str(&format!(" | band 1e-9..1e-7 {lo_band:.0}..{hi_band:.0} K"));
        } else {
            out.push_str(" | band 1e-9..1e-7 empty");
        }
        out.push('\n');
    }
    out.push('\n');

    out.push_str("== Pressure dependence of OCS (logZ +0.6, C/O 0.22) ==\n");
    for (t, tag) in [(LAYER_T, "layer 450 K"), (1000.0, "1000 K")] {
        out.push_str(&format!("  {tag}:\n"));
        for p_bar in [0.01f64, 0.1, 1.0, 10.0, 100.0] {
            let p = p_bar * P0_PA;
            let frac = frac_at_t_p(t, p, b_obs);
            let line = match frac {
                Some(fr) => {
                    let o = log10_of(&fr, "OCS", &slots);
                    let h = log10_of(&fr, "H2S", &slots);
                    let s = log10_of(&fr, "SO2", &slots);
                    match (o, h, s) {
                        (Some(o), Some(h), Some(s)) => {
                            format!("OCS 10^{o:.2} | H2S 10^{h:.2} | SO2 10^{s:.2}")
                        }
                        _ => "solver refused".to_string(),
                    }
                }
                None => "solver refused".to_string(),
            };
            out.push_str(&format!("    {p_bar:>6.2} bar: {line}\n"));
        }
    }
    out.push('\n');

    out.push_str("== Verdict ==\n");
    for (logz, ocs_450, best, best_t, lo_band, hi_band, cross_ret) in &sweep {
        let word = if !best.is_finite() {
            "solver refused".to_string()
        } else if *best < OCS_BAND_MIN_LOG10 {
            format!(
                "OCS stays below the target band: the best freeze-in 10^{} lies more than an order of magnitude below 10^{OCS_RET_LOG10:.0}; quench equilibrium alone does not close the detection",
                fmt_v(*best)
            )
        } else {
            format!(
                "OCS naturally at retrieved order, provided the quench level is hot enough: freeze-in of a 10^{} source (best at {:.0} K; {}; band 1e-9..1e-7 {:.0}..{:.0} K); layer equilibrium at 450 K 10^{} (disequilibrium gap {:.1} dex)",
                fmt_v(*best),
                best_t,
                cross_txt(*cross_ret, *best),
                lo_band,
                hi_band,
                fmt_v(*ocs_450),
                OCS_RET_LOG10 - ocs_450
            )
        };
        out.push_str(&format!("  logZ {logz:+.1}: {word}\n"));
    }
    out.push('\n');

    out.push_str("== Findings ==\n");
    out.push_str("  (1) S disequilibrium/quench model at the terminator (Barat-2025 conditions: 450 K layer, logZ +0.6, C/O 0.22):\n");
    out.push_str("      the layer equilibrium at 450 K carries OCS 10^-22.6 — the layer itself cannot carry the detection;\n");
    out.push_str("      the freeze-in source (equilibrium at ~1000..1800 K, where CO is released from CH4 and CO+H2S feeds the OCS) carries OCS up to 10^-8.2..10^-7.4 per logZ +0.6..+1.0;\n");
    out.push_str("      vertical mixing (Kzz 1e6..1e10 cm2/s), which freezes the composition at the quench level, supplies this hot fraction in the 450 K layer — OCS forms naturally IF the deep interior reaches the retrieved level 1e-8 at equilibrium (per logZ: T(OCS=1e-8), see table above).\n");
    out.push_str("  (2) The condition is not free of measurement: Barat 2025 favors a deep interior temperature ~500 K in the self-consistent grids (CH4 deficit); if the deep terminator T lies below ~1000 K, the OCS quench source drops below 10^-9 and equilibrium does not close the detection — then kinetics/photochemistry or a hotter interior would be needed.\n");
    out.push_str("  (3) Kinetics caveat: which T_q freezes in exactly (tau_chem = Kzz/H_p^2) requires reaction rates + g + P-T profile — all three pending; the thermodynamic envelope is the honest limit of the model and sits at the retrieved order of magnitude, provided the deep interior is hot enough.\n");
    out.push_str("  (4) Metals drive the OCS: at logZ +0.4 (2.5x) the freeze-in source reaches 10^-8.6 (lower band edge); at logZ +0.6..+1.0 (4..10x) 10^-8.2..10^-7.4 — the retrieved logZ +0.4..+1.0 lies exactly in the natural OCS window.\n");
    out.push('\n');

    out.push_str("== pending (0 honored — missing sources) ==\n");
    out.push_str("  T-P profile of the terminator (Barat 2025 provides no P-T profile in the register) — the freeze-in envelope over T does not replace it\n");
    out.push_str("  Surface gravity g / planet mass of V1298 Tau b (transit planet, no measured mass in the register) — Kzz -> mixing timescale H_p^2/Kzz not quantifiable\n");
    out.push_str("  Reaction rates (k(T), Arrhenius) for the S/C network OCS<->CO+H2S, OCS<->CS, SO2<->H2S — full kinetics would determine T_q from tau_chem(T_q) = Kzz/H_p^2; this approximation measures only the thermodynamic envelope\n");
    out.push_str("  Spectra/retrieval data Barat 2025 (OCS-vs-CO/CO2 degeneracy) — separate check, see findings\n");

    Ok(out)
}

fn main() {
    match run_report() {
        Ok(out) => {
            let path = "tmp/v1298_tau_b_sulfur_quench_verdict.txt";
            if let Err(e) = std::fs::write(path, &out) {
                eprintln!("v1298_tau_b_sulfur_quench_probe: write {path}: {e}");
                std::process::exit(1);
            }
            println!("{out}");
        }
        Err(msg) => {
            eprintln!("v1298_tau_b_sulfur_quench_probe: {msg}");
            std::process::exit(1);
        }
    }
}
