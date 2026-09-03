use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::mitdb::{decimate, decode_212, envelope, parse_hea};
use omegaflow::te::{phase_randomized_surrogate, topological_te_phase, transfer_entropy_lag};

const BASE: &str = "https://physionet.org/files/mitdb/1.0.0/";
const BUCKET_S: f64 = 1.0;
const LAG_MAX: usize = 60;
const N_SURR: usize = 10;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let record = arg_value(&args, "--record").unwrap_or_else(|| "100".to_string());
    let hea_url = format!("{}{}.hea", BASE, record);
    let dat_url = format!("{}{}.dat", BASE, record);
    let Some(hea_bytes) = fetch_raw_bytes(&hea_url, 3600) else {
        eprintln!("{}.hea absent — the probe stays still (0 honored)", record);
        return;
    };
    let Some(hea) = parse_hea(&String::from_utf8_lossy(&hea_bytes)) else {
        eprintln!("{}.hea: WFDB-contract void", record);
        return;
    };
    if hea.nchan != 2 {
        eprintln!(
            "{}: {} channels — the probe pairs exactly two leads",
            record, hea.nchan
        );
        return;
    }
    if hea.leads.iter().any(|l| l.format != 212) {
        eprintln!("{}: a lead carries a non-212 format", record);
        return;
    }
    let Some(dat_bytes) = fetch_raw_bytes(&dat_url, 3600) else {
        eprintln!("{}.dat absent — the probe stays still (0 honored)", record);
        return;
    };
    let expected = match hea.nsamp.checked_mul(3) {
        Some(v) => v,
        None => {
            eprintln!("{}: nsamp overflows", record);
            return;
        }
    };
    if dat_bytes.len() != expected {
        eprintln!(
            "{}: sample drift — hea {} samples, dat {} bytes",
            record,
            hea.nsamp,
            dat_bytes.len()
        );
        return;
    }
    let (ch0, ch1) = match decode_212(&dat_bytes, hea.nsamp) {
        Some(v) => v,
        None => {
            eprintln!("{}: 212 decoding void", record);
            return;
        }
    };
    let a = envelope(
        &ch0,
        hea.leads[0].gain,
        hea.leads[0].adc_zero,
        hea.sample_rate,
        BUCKET_S,
    );
    let b = envelope(
        &ch1,
        hea.leads[1].gain,
        hea.leads[1].adc_zero,
        hea.sample_rate,
        BUCKET_S,
    );
    let n = a.len().min(b.len());
    let a = decimate(&a[..n], 300);
    let b = decimate(&b[..n], 300);
    let n = a.len().min(b.len());
    let a: Vec<f32> = a[..n].to_vec();
    let b: Vec<f32> = b[..n].to_vec();
    if n < 32 {
        eprintln!(
            "{}: {} buckets — too few for a lag sweep (0 honored)",
            record, n
        );
        return;
    }

    let na = &hea.leads[0].name;
    let nb = &hea.leads[1].name;
    println!(
        "{}: {} ↔ {} ({} Hz, {}-s envelope buckets, {} buckets, ~{} s, epoch relative)",
        record,
        na,
        nb,
        hea.sample_rate,
        BUCKET_S,
        n,
        n as f64 * BUCKET_S
    );
    if let Some(c) = &hea.comment {
        println!("patient line: {}", c);
    }
    println!();
    println!(
        "TE lag sweep ({} ↔ {}, lag 0..{} buckets):",
        na, nb, LAG_MAX
    );
    println!(
        "{:>4} | {:>11} | {:>11} | {:>11}",
        "lag",
        format!("TE({na}→{nb})"),
        format!("TE({nb}→{na})"),
        "fam"
    );
    let mut fam = f64::NEG_INFINITY;
    let mut best: Option<(usize, f64)> = None;
    for lag in 0..=LAG_MAX {
        let seed = SEED ^ (lag as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        let ab = transfer_entropy_lag(&a, &b, lag).unwrap_or(f64::NAN);
        let ba = transfer_entropy_lag(&b, &a, lag).unwrap_or(f64::NAN);
        let mut rng = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        for _ in 0..N_SURR {
            let sa = phase_randomized_surrogate(&a, &mut rng);
            if let Some(v) = transfer_entropy_lag(&sa, &b, lag) {
                if v > fam {
                    fam = v;
                }
            }
        }
        if ab > best.map_or(f64::NEG_INFINITY, |(_, v)| v) {
            best = Some((lag, ab));
        }
        println!(
            "{:>4} | {:>11.4e} | {:>11.4e} | {:>11.4e}",
            lag, ab, ba, fam
        );
    }
    println!();
    if let Some((l, v)) = best {
        let word = if v > fam { "ARROW (over fam)" } else { "still" };
        println!(
            "Peak TE({na}→{nb}) at lag {} buckets ({} s), TE {:.4e} vs fam {:.4e} — {}",
            l,
            l as f64 * BUCKET_S,
            v,
            fam,
            word
        );
    }
    println!(
        "fam = strongest surrogate TE of the round (multiple-comparison correction). Two leads of the same heart couple symmetrically at lag 0 — an edge arrow at high lag is the KDE-sweep artifact, no finding."
    );
    println!();
    println!("Counter-probe — Takens-embedded TE (phase space, dim 3, order 3, auto-MI-τ):");
    let topo_ab = topological_te_phase(&a, &b, 3, 3, SEED);
    let topo_ba = topological_te_phase(&b, &a, 3, 3, SEED);
    for (name, v) in [
        (format!("TE({na}→{nb})"), &topo_ab),
        (format!("TE({nb}→{na})"), &topo_ba),
    ] {
        match v {
            Some(t) => {
                let word = if t.te > t.threshold {
                    "ARROW (over thr)"
                } else {
                    "still"
                };
                println!(
                    "  {}: te {:.4e} vs thr {:.4e} ({} Surrogate, τ_x {} τ_y {}) — {}",
                    name, t.te, t.threshold, t.surrogates_used, t.tau_x, t.tau_y, word
                );
            }
            None => println!(
                "  {name}: no MI-τ — the phase space carries no coupling (still, 0 honored)"
            ),
        }
    }
}
