use omegaflow::te::{ols_residual, surrogate_stats_phase, transfer_entropy_lag};
use std::collections::BTreeMap;

const DEFAULT_SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DEFAULT_LAG_MAX: usize = 2;
const DEFAULT_N_SURR: usize = 10;
const MIN_SAMPLES: usize = 8;

struct Series {
    name: String,
    values: Vec<f32>,
}

fn read_series(name: &str, path: &str) -> Result<Series, String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("{name} {path}: {e}"))?;
    let mut values = Vec::new();
    for (i, line) in body.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let v: f64 = line
            .parse()
            .map_err(|_| format!("{name} {path}: line {} is not one f64 value", i + 1))?;
        if !v.is_finite() {
            return Err(format!("{name} {path}: line {} is not finite", i + 1));
        }
        values.push(v as f32);
    }
    if values.is_empty() {
        return Err(format!("{name} {path}: no values (0 honored)"));
    }
    Ok(Series {
        name: name.to_string(),
        values,
    })
}

fn rms(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    Some((v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt())
}

fn stddev(v: &[f32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let n = v.len() as f64;
    let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
    Some((v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Deploy {
    None,
    TechnoDip,
    TechnoNarrowband,
    BioDisequilibrium,
}

struct Manifest {
    candidate: &'static str,
    witnesses: &'static [&'static str],
    note: &'static str,
}

impl Deploy {
    fn parse(s: &str) -> Option<Deploy> {
        match s {
            "none" => Some(Deploy::None),
            "techno-dip" => Some(Deploy::TechnoDip),
            "techno-narrowband" => Some(Deploy::TechnoNarrowband),
            "bio-disequilibrium" => Some(Deploy::BioDisequilibrium),
            _ => None,
        }
    }
    fn manifest(self) -> Option<Manifest> {
        match self {
            Deploy::None => None,
            Deploy::TechnoDip => Some(Manifest {
                candidate: "techno_dip_series",
                witnesses: &[
                    "stellar_variability_vsx_gcvs",
                    "rotation_spots",
                    "dust_iras_akari",
                    "occultation_exoplanets",
                    "instrument_systematics",
                    "hephaistos_background",
                ],
                note: "Techno-Dip: Staub/Flecken/Variabilitaet/Bedeckung/Systematik als Boden-Zeugen (Befund 2026-09-05)",
            }),
            Deploy::TechnoNarrowband => Some(Manifest {
                candidate: "narrowband_channel_series",
                witnesses: &["rfi_off_source", "channel_baseline"],
                note: "Techno-Narrowband: die Kanal-Serie existiert nicht im Bestand; jeder Linien-Zeuge ist noetig (Befund 2026-09-05)",
            }),
            Deploy::BioDisequilibrium => Some(Manifest {
                candidate: "disequilibrium_series",
                witnesses: &["stellar_activity_xuv", "reservoir_feh"],
                note: "Bio-Disequilibrium: Primaer-Null = thermochemisches Gleichgewicht (disequilibrium_register_probe); Aktivitaet/XUV und [Fe/H] als Serie fehlen (Befund 2026-09-05)",
            }),
        }
    }
}

struct TestRow {
    witness: String,
    direction: String,
    lag: usize,
    te: Option<f64>,
    thr: Option<f64>,
}

struct Stage1Row {
    witness: String,
    r2: Option<f64>,
    rms_before: Option<f64>,
    rms_after: Option<f64>,
}

struct Run {
    candidate: Series,
    witnesses: Vec<Series>,
    lag_max: usize,
    seed: u64,
    deploy: Deploy,
    cadence_s: Option<f64>,
}

fn extraction(run: &Run) -> (Vec<f32>, Vec<Stage1Row>, Vec<String>) {
    let n = run.candidate.values.len();
    let mut resid = run.candidate.values.clone();
    let mut rows = Vec::new();
    let mut notes = Vec::new();
    for w in &run.witnesses {
        let before = rms(&resid);
        let Some(sdw) = stddev(&w.values) else {
            notes.push(format!("witness {}: zero length — not informative", w.name));
            rows.push(Stage1Row {
                witness: w.name.clone(),
                r2: None,
                rms_before: before,
                rms_after: before,
            });
            continue;
        };
        if sdw <= 1e-30 {
            notes.push(format!(
                "witness {}: zero variance — constant, nothing to regress, excluded (named)",
                w.name
            ));
            rows.push(Stage1Row {
                witness: w.name.clone(),
                r2: None,
                rms_before: before,
                rms_after: before,
            });
            continue;
        }
        match ols_residual(&resid, &w.values) {
            Some(r) => {
                let after = rms(&r);
                let m0 = resid.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
                let rss0 = resid.iter().map(|&x| (x as f64 - m0).powi(2)).sum::<f64>();
                let rss1 = r.iter().map(|&x| (x as f64).powi(2)).sum::<f64>();
                let r2 = if rss0 > 0.0 { 1.0 - rss1 / rss0 } else { 0.0 };
                rows.push(Stage1Row {
                    witness: w.name.clone(),
                    r2: Some(r2),
                    rms_before: before,
                    rms_after: after,
                });
                resid = r;
            }
            None => {
                notes.push(format!(
                    "witness {}: OLS on conditioning did not resolve — the linear fit is degenerate",
                    w.name
                ));
                rows.push(Stage1Row {
                    witness: w.name.clone(),
                    r2: None,
                    rms_before: before,
                    rms_after: before,
                });
            }
        }
    }
    debug_assert_eq!(resid.len(), n);
    (resid, rows, notes)
}

fn negative_test(run: &Run, resid: &[f32]) -> Vec<TestRow> {
    let n = run.candidate.values.len();
    let mut rows = Vec::new();
    let mut surr_seed = run.seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    for w in &run.witnesses {
        let sdw = stddev(&w.values);
        if sdw.map_or(true, |s| s <= 1e-30) {
            continue;
        }
        for lag in 0..=run.lag_max {
            if n < MIN_SAMPLES + lag {
                continue;
            }
            let te_w_res = transfer_entropy_lag(resid, &w.values, lag);
            let thr_w_res = surrogate_stats_phase(resid, &w.values, lag, surr_seed);
            surr_seed = surr_seed.wrapping_add(0x517C_C1B7_2722_0A95);
            rows.push(TestRow {
                witness: w.name.clone(),
                direction: "witness -> residue".to_string(),
                lag,
                te: te_w_res,
                thr: thr_w_res.map(|(_, _, t)| t),
            });
            let te_res_w = transfer_entropy_lag(&w.values, resid, lag);
            let thr_res_w = surrogate_stats_phase(&w.values, resid, lag, surr_seed);
            surr_seed = surr_seed.wrapping_add(0x517C_C1B7_2722_0A95);
            rows.push(TestRow {
                witness: w.name.clone(),
                direction: "residue -> witness".to_string(),
                lag,
                te: te_res_w,
                thr: thr_res_w.map(|(_, _, t)| t),
            });
        }
    }
    rows
}

fn fam_bound(rows: &[TestRow]) -> Option<f64> {
    let mut fam: Option<f64> = None;
    for r in rows {
        if let Some(t) = r.thr {
            fam = Some(match fam {
                Some(a) => a.max(t),
                None => t,
            });
        }
    }
    fam
}

fn fmt_te(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{:.4e}", x),
        None => "n/a (n-floor)".to_string(),
    }
}

fn deploy_name(d: Deploy) -> &'static str {
    match d {
        Deploy::None => "none",
        Deploy::TechnoDip => "techno-dip",
        Deploy::TechnoNarrowband => "techno-narrowband",
        Deploy::BioDisequilibrium => "bio-disequilibrium",
    }
}

fn run_probe(run: &Run, out_path: &str) -> Result<String, String> {
    let n = run.candidate.values.len();
    for w in &run.witnesses {
        if w.values.len() != n {
            return Err(format!(
                "witness {} length {} differs from candidate length {} — series must be co-sampled to equal length",
                w.name,
                w.values.len(),
                n
            ));
        }
    }
    if n < MIN_SAMPLES {
        return Err(format!(
            "candidate length {n} below the te.rs n-floor {MIN_SAMPLES} — the machine stays silent (0 honored)"
        ));
    }

    let mut out = String::new();
    out.push_str(
        "negativ_fuzzy_probe — the formal negative-fuzzy instrument (Bio/Techno exclusion)\n",
    );
    out.push_str(&format!(
        "candidate: {} ({n} samples, RMS {:.3e})\n",
        run.candidate.name,
        rms(&run.candidate.values).unwrap_or(f64::NAN)
    ));
    for w in &run.witnesses {
        out.push_str(&format!(
            "witness:   {} ({} samples, RMS {:.3e})\n",
            w.name,
            w.values.len(),
            rms(&w.values).unwrap_or(f64::NAN)
        ));
    }
    out.push_str(&format!(
        "machine: stage-1 extraction (OLS on conditioning, ols_residual of te.rs — the residual core shared with residual_surrogate_conditional/conditional_te_stats); stage-2 negative test (transfer_entropy_lag both directions over lags 0..={} vs surrogate_stats_phase mean+2sigma and the fam bound)\n",
        run.lag_max
    ));
    out.push_str(&format!(
        "seed {:016x} | phase surrogates {}\n",
        run.seed, DEFAULT_N_SURR
    ));

    if let Some(dt) = run.cadence_s {
        out.push_str(&format!(
            "cadence: {:.3e} s per sample step — tested physical lags 0 .. {:.3e} s\n",
            dt,
            dt * run.lag_max as f64
        ));
    }

    let (resid, stage1, notes) = extraction(run);

    out.push_str("stage-1 extraction (candidate regressed on each witness, sequential OLS):\n");
    out.push_str(&format!(
        "  candidate RMS {:.3e} -> residue RMS {:.3e}\n",
        rms(&run.candidate.values).unwrap_or(f64::NAN),
        rms(&resid).unwrap_or(f64::NAN)
    ));
    for r in &stage1 {
        match r.r2 {
            Some(r2) => out.push_str(&format!(
                "  {:<24} removed {:.4} of running residual variance (RMS {:.3e} -> {:.3e})\n",
                r.witness,
                r2,
                r.rms_before.unwrap_or(f64::NAN),
                r.rms_after.unwrap_or(f64::NAN)
            )),
            None => out.push_str(&format!(
                "  {:<24} no OLS dimension — not a regressor\n",
                r.witness
            )),
        }
    }
    for note in &notes {
        out.push_str(&format!("  note: {note}\n"));
    }
    let informative: Vec<&Series> = run
        .witnesses
        .iter()
        .filter(|w| stddev(&w.values).map_or(false, |s| s > 1e-30))
        .collect();
    if informative.is_empty() {
        out.push_str(
            "no informative witness provided — the negative test needs a witness; verdict pending (0 honored)\n",
        );
        std::fs::write(out_path, &out).map_err(|e| format!("{out_path}: {e}"))?;
        return Ok(out);
    }

    out.push_str(&format!(
        "n-floor limit: n={n} samples; the scan needs m = n - lag >= {MIN_SAMPLES} (te.rs canonical floor); lags that cannot reach it are refused, never zeroed\n"
    ));
    if run.cadence_s.is_none() {
        out.push_str(
            "wrong-lag limit: no physical cadence given — lags are sample units only; a coupling whose crossing time v*dt falls between samples or beyond the scan is not tested (the sharpest techno limit stays open)\n",
        );
    }

    let rows = negative_test(run, &resid);
    let fam = fam_bound(&rows);
    out.push_str(&format!(
        "stage-2 negative test on the residue (both directions, per lag; surrogate mean+2sigma per test, fam = strongest per-test null threshold over the whole run):\n"
    ));
    out.push_str(&format!(
        "  fam = {}\n",
        fam.map_or("n/a".to_string(), |f| format!("{:.4e}", f))
    ));
    let mut carried: Vec<String> = Vec::new();
    for r in &rows {
        let word = match (r.te, r.thr) {
            (Some(te), Some(thr)) => {
                if fam.map_or(false, |f| te > f.max(thr)) {
                    carried.push(format!("{} {}", r.witness, r.direction));
                    "carried".to_string()
                } else if te > thr {
                    "above own null, below fam".to_string()
                } else {
                    "still (TE ~ 0)".to_string()
                }
            }
            _ => "n/a".to_string(),
        };
        out.push_str(&format!(
            "  {:<14} {:<22} lag {:>2}  te {}  thr {}  | {}\n",
            r.witness,
            r.direction,
            r.lag,
            fmt_te(r.te),
            fmt_te(r.thr),
            word
        ));
    }

    out.push_str("\n");
    if carried.is_empty() {
        out.push_str("VERDICT: not carried\n");
        out.push_str(
            "  — the residue couples to no provided witness (TE near 0 in every direction over the scanned lags).\n",
        );
        out.push_str(
            "  — 'not carried' is the negative-fuzzy candidate designation. It is never read as 'independent': the three limits below bind it.\n",
        );
    } else {
        carried.sort_unstable();
        carried.dedup();
        out.push_str(&format!("VERDICT: carried by {}\n", carried.join("; ")));
        out.push_str(
            "  — the residue still speaks those Boden witnesses; the candidate's un-carried remainder is what the negative-fuzzy instrument would name next.\n",
        );
    }
    out.push_str(
        "missing-witness limit: TE near 0 against the provided witnesses never excludes an unknown natural class — the exclusion names only what was measured\n",
    );
    if let Some(m) = run.deploy.manifest() {
        out.push_str(&format!(
            "deploy: {} — {}\n",
            deploy_name(run.deploy),
            m.note
        ));
        let provided: Vec<&str> = run.witnesses.iter().map(|w| w.name.as_str()).collect();
        let missing: Vec<&str> = m
            .witnesses
            .iter()
            .copied()
            .filter(|s| !provided.contains(s))
            .collect();
        if !missing.is_empty() {
            out.push_str(&format!(
                "  deploy {}({}) witness slots not provided: {} — their natural classes stay open\n",
                deploy_name(run.deploy),
                m.candidate,
                missing.join(", ")
            ));
        }
    }

    std::fs::write(out_path, &out).map_err(|e| format!("{out_path}: {e}"))?;
    Ok(out)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut candidate_path: Option<String> = None;
    let mut witness_args: Vec<(String, String)> = Vec::new();
    let mut out = "/tmp/opencode/negativ_fuzzy_verdict.txt".to_string();
    let mut seed = DEFAULT_SEED;
    let mut lag_max = DEFAULT_LAG_MAX;
    let mut deploy = Deploy::None;
    let mut cadence_s: Option<f64> = None;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--candidate" => {
                i += 1;
                candidate_path = args.get(i).cloned();
            }
            "--witness" => {
                i += 1;
                if let Some(spec) = args.get(i) {
                    if let Some((name, path)) = spec.split_once('=') {
                        witness_args.push((name.to_string(), path.to_string()));
                    } else {
                        eprintln!("negativ_fuzzy_probe: --witness must be <name>=<path> — refused");
                        std::process::exit(1);
                    }
                }
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(out);
            }
            "--seed" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse::<u64>().ok()) {
                    seed = v;
                }
            }
            "--lag-max" => {
                i += 1;
                if let Some(v) = args.get(i).and_then(|s| s.parse::<usize>().ok()) {
                    lag_max = v;
                }
            }
            "--cadence-s" => {
                i += 1;
                cadence_s = args
                    .get(i)
                    .and_then(|s| s.parse::<f64>().ok())
                    .filter(|v| *v > 0.0);
            }
            "--deploy" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    match Deploy::parse(v) {
                        Some(d) => deploy = d,
                        None => {
                            eprintln!("negativ_fuzzy_probe: --deploy unknown value {v} — refused");
                            std::process::exit(1);
                        }
                    }
                }
            }
            other => {
                eprintln!("negativ_fuzzy_probe: unknown argument {other} — refused");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let run = match build_run(
        candidate_path.as_deref(),
        &witness_args,
        lag_max,
        seed,
        deploy,
        cadence_s,
    ) {
        Ok(r) => r,
        Err(msg) => {
            let _ = std::fs::write(&out, &msg);
            eprintln!("negativ_fuzzy_probe: {msg}");
            std::process::exit(1);
        }
    };

    let result = run_probe(&run, &out);
    match result {
        Ok(text) => {
            println!("{text}");
        }
        Err(msg) => {
            eprintln!("negativ_fuzzy_probe: {msg}");
            std::process::exit(1);
        }
    }
}

fn build_run(
    candidate_path: Option<&str>,
    witness_args: &[(String, String)],
    lag_max: usize,
    seed: u64,
    deploy: Deploy,
    cadence_s: Option<f64>,
) -> Result<Run, String> {
    if let Some(m) = deploy.manifest() {
        let provided: Vec<&str> = witness_args.iter().map(|(n, _)| n.as_str()).collect();
        let missing: Vec<&str> = m
            .witnesses
            .iter()
            .copied()
            .filter(|s| !provided.contains(s))
            .collect();
        let candidate_present = candidate_path.is_some();
        if candidate_path.is_none() || !missing.is_empty() {
            let mut buf = String::new();
            buf.push_str(&format!(
                "deploy {} requires series the archive does not carry — pending, not fabricated\n",
                deploy_name(deploy)
            ));
            if !candidate_present {
                buf.push_str(&format!("  candidate slot {}: absent\n", m.candidate));
            }
            for s in &missing {
                buf.push_str(&format!("  witness slot {s}: absent\n"));
            }
            return Err(buf);
        }
    }
    let Some(cand_path) = candidate_path else {
        return Err("--candidate <series file> is required".to_string());
    };
    if witness_args.is_empty() {
        return Err(
            "at least one --witness <name>=<path> is required — a negative test needs a witness"
                .to_string(),
        );
    }
    let candidate = read_series("candidate", cand_path)?;
    let mut witnesses = Vec::new();
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for (name, path) in witness_args {
        if seen.contains_key(name.as_str()) {
            return Err(format!(
                "witness name {name} given twice — names must be unique"
            ));
        }
        seen.insert(name, 1);
        witnesses.push(read_series(name, path)?);
    }
    Ok(Run {
        candidate,
        witnesses,
        lag_max,
        seed,
        deploy,
        cadence_s,
    })
}
