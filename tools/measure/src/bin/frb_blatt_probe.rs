use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::CDN_BASE;
use omegaflow::json::{JsonVal, jnum, jstr, parse_json};
use omegaflow::te::{phase_randomized_surrogate, silverman, transfer_entropy_lag};

const CDN_RELEASE: &str = "ssd.jpl.nasa.gov";
const CDN_ASSET: &str = "frb_chime_cat1.json";
const LOCAL_HARVEST: &str = "phi/frb_harvest/frb_chime_cat1.json";
const MIN_N: usize = 30;
const N_SURR: usize = 10;
const DISTANCES: [usize; 3] = [1, 2, 3];
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn load_harvest(path: Option<String>) -> Option<Vec<JsonVal>> {
    let bytes = match path {
        Some(p) => std::fs::read(&p).ok(),
        None => {
            let local = std::fs::read(LOCAL_HARVEST).ok();
            match local {
                Some(b) => Some(b),
                None => {
                    let url = format!("{CDN_BASE}/{CDN_RELEASE}/{CDN_ASSET}");
                    fetch_raw_bytes(&url, 3600)
                }
            }
        }
    }?;
    let text = std::str::from_utf8(&bytes).ok()?;
    match parse_json(text) {
        Some(JsonVal::Arr(rows)) => Some(rows),
        _ => None,
    }
}

fn repeater_map(rows: &[JsonVal]) -> Vec<(String, Vec<f64>)> {
    let mut by_rep: Vec<(String, Vec<(f64, f64)>)> = Vec::new();
    for row in rows {
        let Some(rp) = jstr(row, "rpname") else {
            continue;
        };
        let Some(epoch) = jnum(row, "epoch_tdb") else {
            continue;
        };
        let span = match by_rep.iter_mut().find(|(n, _)| *n == rp) {
            Some((_, v)) => v,
            None => {
                by_rep.push((rp.clone(), Vec::new()));
                let (_, v) = by_rep.last_mut().unwrap();
                v
            }
        };
        span.push((epoch, 0.0));
    }
    by_rep
        .into_iter()
        .map(|(name, mut v)| {
            v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let mut dts: Vec<f64> = Vec::new();
            for w in v.windows(2) {
                let dt = w[1].0 - w[0].0;
                if dt.is_finite() && dt > 0.0 {
                    dts.push(dt);
                }
            }
            (name, dts)
        })
        .collect()
}

fn mean_plus_2sigma(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() {
        return None;
    }
    let m = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / vals.len() as f64;
    Some(m + 2.0 * var.sqrt())
}

fn surrogate_te_values(future: &[f32], past: &[f32], seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut vals = Vec::new();
    for _ in 0..N_SURR {
        let surr = phase_randomized_surrogate(past, &mut rng);
        if let Some(te) = transfer_entropy_lag(future, &surr, 0) {
            vals.push(te);
        }
    }
    vals
}

fn self_te(dts: &[f64], distance: usize) -> Option<f64> {
    if dts.len() <= distance {
        return None;
    }
    let future: Vec<f32> = dts[distance..].iter().map(|&d| d as f32).collect();
    let past: Vec<f32> = dts[..dts.len() - distance]
        .iter()
        .map(|&d| d as f32)
        .collect();
    transfer_entropy_lag(&future, &past, 0)
}

struct TrainVerdict {
    name: String,
    n: usize,
    best_distance: usize,
    te: f64,
    thr: f64,
    arrow: bool,
    family_bound: bool,
}

fn run_trains(trains: &[(String, Vec<f64>)]) -> (f64, Vec<TrainVerdict>) {
    let mut fam = f64::NEG_INFINITY;
    let mut verdicts: Vec<TrainVerdict> = Vec::new();
    for (name, dts) in trains {
        if dts.len() < MIN_N {
            verdicts.push(TrainVerdict {
                name: name.clone(),
                n: dts.len(),
                best_distance: 0,
                te: f64::NAN,
                thr: f64::NAN,
                arrow: false,
                family_bound: false,
            });
            continue;
        }
        let mut best: Option<(usize, f64)> = None;
        let mut best_thr = f64::NAN;
        for &distance in DISTANCES.iter() {
            let seed = SEED ^ (distance as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
            let future: Vec<f32> = dts[distance..].iter().map(|&d| d as f32).collect();
            let past: Vec<f32> = dts[..dts.len() - distance]
                .iter()
                .map(|&d| d as f32)
                .collect();
            let surr = surrogate_te_values(&future, &past, seed);
            for &v in &surr {
                if v > fam {
                    fam = v;
                }
            }
            if let Some(te) = self_te(dts, distance) {
                if best.map_or(true, |(_, b)| te > b) {
                    best = Some((distance, te));
                    best_thr = mean_plus_2sigma(&surr).unwrap_or(f64::NAN);
                }
            }
        }
        let (best_distance, te) = match best {
            Some(b) => b,
            None => {
                verdicts.push(TrainVerdict {
                    name: name.clone(),
                    n: dts.len(),
                    best_distance: 0,
                    te: f64::NAN,
                    thr: f64::NAN,
                    arrow: false,
                    family_bound: false,
                });
                continue;
            }
        };
        let arrow = te > fam;
        let family_bound = !arrow && te > best_thr;
        verdicts.push(TrainVerdict {
            name: name.clone(),
            n: dts.len(),
            best_distance,
            te,
            thr: best_thr,
            arrow,
            family_bound,
        });
    }
    (fam, verdicts)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args
        .iter()
        .position(|a| a == "--harvest")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let rows = match load_harvest(path) {
        Some(r) => r,
        None => {
            eprintln!("frb_blatt_probe: no harvest — the Blatt stays empty (0 honored)");
            std::process::exit(1);
        }
    };

    let total = rows.len();
    let one_epoch = rows.iter().filter(|r| jstr(r, "rpname").is_none()).count();
    let trains = repeater_map(&rows);
    let with_series = trains.iter().filter(|(_, d)| d.len() >= MIN_N).count();

    println!("=== The Blatt of the FRB scatter-track — pulse against beam (Nadel IX) ===");
    println!(
        "Harvest: {} bursts total, {} one-epoch (no rpname), {} repeater trains ({} with n ≥ {}).",
        total,
        one_epoch,
        trains.len(),
        with_series,
        MIN_N
    );
    println!("Series axis: inter-burst interval series Δt (seconds, TDB-sorted) per repeater.");
    println!(
        "Measurement: self-memory TE(future ← past) at distances {:?} (the interval at position t carries information about the interval at t+distance+1 beyond t+distance), phase-randomized surrogates ({}, mean + 2σ), fam = strongest surrogate TE of the whole round. Arrow ⇔ TE > fam: the train carries memory — a modulated beam. Silence — the explosive-impulse form.",
        DISTANCES, N_SURR
    );

    let (fam, verdicts) = run_trains(&trains);

    println!("  fam = {:.4e}", fam);
    println!("  trains:");
    let mut arrows = 0;
    for v in &verdicts {
        let word = if v.arrow {
            "ARROW — the train carries memory: modulated beam"
        } else if v.family_bound {
            "family bound — no arrow, above its own surrogate threshold"
        } else if v.te.is_nan() {
            "no statement — fewer than MIN_N intervals"
        } else {
            "still — no memory: explosive impulse form"
        };
        if v.arrow {
            arrows += 1;
        }
        if v.te.is_nan() {
            println!("    {:16} | n = {:3} | no statement", v.name, v.n);
        } else {
            println!(
                "    {:16} | n = {:3} | dist {:2} | TE {:>10.4e} | thr {:>10.4e} | {}",
                v.name, v.n, v.best_distance, v.te, v.thr, word
            );
        }
    }

    let silverman_ok = trains
        .iter()
        .filter(|(_, d)| d.len() >= MIN_N)
        .filter(|(_, d)| {
            let xs: Vec<f32> = d.iter().map(|&x| x as f32).collect();
            silverman(&xs).is_some()
        })
        .count();
    println!(
        "  Silverman-positive series: {} (KDE bandwidth defined)",
        silverman_ok
    );

    let verdict = if trains.is_empty() {
        "no statement — the harvest carries no repeater trains"
    } else if with_series == 0 {
        "no statement — no repeater train reaches MIN_N = 30 intervals; the one-epoch bursts carry a single epoch (0 honored)"
    } else if arrows == 0 {
        "form: explosive impulse — no measured burst train carries self-memory beyond its surrogate family"
    } else {
        "form: modulated beam — at least one measured burst train carries self-memory (TE > fam)"
    };
    println!("  verdict: {}", verdict);
}
