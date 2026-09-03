use omegaflow::doppler::parse_bin;

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() {
        return f64::NAN;
    }
    v[v.len() / 2]
}

fn main() {
    let path = "data/pioneer10_doppler.bin";
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("pioneer10: doppler bin void ({path})");
        return;
    };
    let Some(records) = parse_bin(&bytes) else {
        eprintln!("pioneer10: doppler bin parse void");
        return;
    };
    let n = records.len();
    let mut obs = Vec::with_capacity(n);
    let mut freq = Vec::with_capacity(n);
    let mut dtype = Vec::with_capacity(n);
    for r in &records {
        obs.push(r[1]);
        freq.push(r[2]);
        dtype.push(r[4] as i64);
    }
    let mut os = obs.clone();
    let mut fs = freq.clone();
    eprintln!(
        "pioneer10: {} records — OBSVBL [{:.3e}, {:.3e}] Hz (Median {:.3e}); FREQCY [{:.3e}, {:.3e}] Hz (Median {:.3e})",
        n,
        obs.iter().cloned().fold(f64::INFINITY, f64::min),
        obs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        median(&mut os),
        freq.iter().cloned().fold(f64::INFINITY, f64::min),
        freq.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        median(&mut fs),
    );
    let mut ds: Vec<i64> = dtype.clone();
    ds.sort_unstable();
    ds.dedup();
    eprintln!("DTYPE: {ds:?}");

    let mut jumps: Vec<f64> = Vec::with_capacity(n - 1);
    let mut jump_idx: Vec<usize> = Vec::with_capacity(n - 1);
    for i in 0..n - 1 {
        let dt = records[i + 1][0] - records[i][0];
        if dt > 0.0 && dt < 300.0 {
            jumps.push(obs[i + 1] - obs[i]);
            jump_idx.push(i);
        }
    }
    let mut j = jumps.clone();
    let jmed = median(&mut j);
    let jmin = jumps.iter().cloned().fold(f64::INFINITY, f64::min);
    let jmax = jumps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut bins = [0usize; 6];
    for &x in &jumps {
        let a = x.abs();
        let b = if a < 5e3 {
            0
        } else if a < 20e3 {
            1
        } else if a < 100e3 {
            2
        } else if a < 500e3 {
            3
        } else if a < 2e6 {
            4
        } else {
            5
        };
        bins[b] += 1;
    }
    eprintln!(
        "OBSVBL jumps (n={}): median {jmed:.3e} Hz, min {jmin:.3e}, max {jmax:.3e} — |jump| <5k:{}, 5–20k:{}, 20–100k:{}, 100–500k:{}, 0.5–2M:{}, >2M:{}",
        jumps.len(),
        bins[0],
        bins[1],
        bins[2],
        bins[3],
        bins[4],
        bins[5]
    );

    let mut idx: Vec<usize> = (0..jumps.len()).collect();
    idx.sort_by(|&a, &b| jumps[b].abs().total_cmp(&jumps[a].abs()));
    for &k in idx.iter().take(8) {
        let i = jump_idx[k];
        eprintln!(
            "  largest jump @ record {i}: {:.3e} Hz (OBSVBL {:.3e} → {:.3e})",
            jumps[k],
            obs[i],
            obs[i + 1]
        );
    }

    let mut fj: Vec<f64> = Vec::with_capacity(n - 1);
    for i in 0..n - 1 {
        let dt = records[i + 1][0] - records[i][0];
        if dt > 0.0 && dt < 300.0 {
            fj.push(freq[i + 1] - freq[i]);
        }
    }
    let mut fjs = fj.clone();
    let fjmed = median(&mut fjs);
    let fjmin = fj.iter().cloned().fold(f64::INFINITY, f64::min);
    let fjmax = fj.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "FREQCY jumps (n={}): median {fjmed:.3e} Hz, min {fjmin:.3e}, max {fjmax:.3e}",
        fj.len()
    );
}
