use omegaflow::jwst::parse_jwst_bin;
use omegaflow::jwst_equilibrium::parse_equilibrium_bin;
use omegaflow::te::{surrogate_stats_phase, transfer_entropy_lag};
use std::collections::HashMap;

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAG_MAX: usize = 2;
const MIN_N: usize = 8;

fn verdict_label(forward: f64, thr: f64, fam: f64) -> &'static str {
    if forward > fam {
        "fam-tragend"
    } else if forward > thr {
        "ueber eigener Schwelle, unter Familien-Schwelle"
    } else {
        "still"
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut spectra = "/tmp/opencode/jwst_spectra.bin".to_string();
    let mut equilibrium = "/tmp/opencode/jwst_equilibrium.bin".to_string();
    let mut out = "/tmp/opencode/jwst_biosignature_verdict.txt".to_string();
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spectra" => {
                i += 1;
                spectra = args.get(i).cloned().unwrap_or(spectra);
            }
            "--equilibrium" => {
                i += 1;
                equilibrium = args.get(i).cloned().unwrap_or(equilibrium);
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(out);
            }
            other => {
                eprintln!("jwst_biosignature_scanner: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if let Err(msg) = run(&spectra, &equilibrium, &out) {
        eprintln!("jwst_biosignature_scanner: {msg}");
        std::process::exit(1);
    }
}

fn run(spectra_path: &str, equilibrium_path: &str, out_path: &str) -> Result<(), String> {
    let sb = std::fs::read(spectra_path).map_err(|e| format!("spectra {spectra_path}: {e}"))?;
    let spectra = parse_jwst_bin(&sb).ok_or("jwst_spectra.bin: no JWS1 contract")?;
    let eb = std::fs::read(equilibrium_path)
        .map_err(|e| format!("equilibrium {equilibrium_path}: {e}"))?;
    let equilibrium = parse_equilibrium_bin(&eb).ok_or("jwst_equilibrium.bin: no JWE1 contract")?;

    let mut eq_by_obs: HashMap<String, &omegaflow::jwst_equilibrium::EquilibriumRecord> =
        HashMap::new();
    for r in &equilibrium {
        eq_by_obs.insert(r.obs_id.clone(), r);
    }

    let mut lines: Vec<String> = Vec::new();
    let mut n_te = 0usize;
    let mut n_fam = 0usize;
    let mut sums_te = 0.0f64;
    let mut results: Vec<(String, f64, f64, f64, String)> = Vec::new();

    for spec in &spectra {
        let Some(eq) = eq_by_obs.get(&spec.obs_id) else {
            continue;
        };
        let mut x: Vec<f32> = spec
            .bins
            .iter()
            .filter(|b| b.2.is_finite() && b.2 > 0.0)
            .map(|b| b.2 as f32)
            .collect();
        let mut y: Vec<f32> = eq.x.iter().map(|&v| v as f32).collect();
        if x.len() < MIN_N || y.len() < MIN_N {
            continue;
        }
        x.truncate(y.len().min(x.len()));
        y.truncate(x.len());
        let mut best_te = 0.0f64;
        let mut best_lag = 0usize;
        for lag in 0..=LAG_MAX {
            if let Some(te) = transfer_entropy_lag(&x, &y, lag) {
                if te > best_te {
                    best_te = te;
                    best_lag = lag;
                }
            }
        }
        let mut rev_te = 0.0f64;
        for lag in 0..=LAG_MAX {
            if let Some(te) = transfer_entropy_lag(&y, &x, lag) {
                if te > rev_te {
                    rev_te = te;
                }
            }
        }
        let (forward, dir) = if best_te >= rev_te {
            (best_te, "stellar->chem")
        } else {
            (rev_te, "chem->stellar")
        };
        let lag = best_lag;
        let thr = surrogate_stats_phase(&x, &y, lag, SEED)
            .map(|(_, _, t)| t)
            .unwrap_or(0.0);
        let fam = best_te.max(rev_te);
        let word = verdict_label(forward, thr, fam);
        n_te += 1;
        sums_te += forward;
        if forward > fam {
            n_fam += 1;
        }
        results.push((spec.obs_id.clone(), forward, thr, fam, word.to_string()));
        lines.push(format!(
            "  {}  te {:.4e} ({dir}, lag {}) thr {:.4e} fam {:.4e} | {}",
            spec.obs_id, forward, lag, thr, fam, word
        ));
    }

    let fam = if n_te > 0 {
        n_fam as f64 / n_te as f64
    } else {
        0.0
    };
    let mean_te = if n_te > 0 { sums_te / n_te as f64 } else { 0.0 };

    let mut out = String::new();
    out.push_str("JWST biosignature scanner — Auftrag 4 verdict\n");
    out.push_str("TE(stellar input -> atmosphere chemistry) per atmosphere\n");
    out.push_str(&format!(
        "inputs: jwst_spectra.bin {} | jwst_equilibrium.bin {}\n",
        spectra.len(),
        equilibrium.len()
    ));
    out.push_str(&format!(
        "atmospheres evaluated: {} | mean TE {:.4e} | fam {:.4}\n",
        n_te, mean_te, fam
    ));
    if n_te == 0 {
        out.push_str(
            "  (no atmosphere with both assets — the biosignature stays absent, 0 honored)\n",
        );
    }
    for l in &lines {
        out.push_str(l);
        out.push('\n');
    }
    out.push_str(&format!(
        "fam over the catalog: {} ({:.2}% of atmospheres above the family threshold) — {}\n",
        n_fam,
        fam * 100.0,
        if n_te == 0 {
            "absent"
        } else if fam == 0.0 {
            "no causal filter breaks the field — silence"
        } else {
            "life = causal filter that breaks the field"
        }
    ));
    std::fs::write(out_path, &out).map_err(|e| format!("{out_path}: {e}"))?;
    println!("{out}");
    Ok(())
}
