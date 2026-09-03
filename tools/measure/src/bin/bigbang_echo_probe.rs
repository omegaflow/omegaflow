use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::CDN_BASE;
use omegaflow::healpix::{ang2pix_nest, icrs_to_galactic};
use omegaflow::json::{JsonVal, jnum, parse_json};
use omegaflow::te::{phase_randomized_surrogate, transfer_entropy_lag};

const NSIDE_CELL: i64 = 8;
const NSIDE_CMB: i64 = 64;
const MIN_PER_CELL: usize = 30;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const LAG_MIN: usize = 1;
const LAG_MAX: usize = 3;
const SURROGATES: usize = 10;
const DEPTH_BINS: usize = 16;
const DEPTH_MAX_MPC: f64 = 800.0;

fn fetch_cached(name: &str, release: &str) -> Option<Vec<u8>> {
    let path = format!("data/{name}");
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all("data").ok();
        let url = format!("{CDN_BASE}/{release}/{name}");
        let bytes = fetch_raw_bytes(&url, 604800)?;
        if std::fs::write(&path, &bytes).is_err() {
            return None;
        }
    }
    std::fs::read(&path).ok()
}

fn as_arr(v: &JsonVal) -> Option<&Vec<JsonVal>> {
    match v {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

fn load_cmb() -> Option<Vec<f32>> {
    let bytes = fetch_cached("cmb_planck_smica_n64.json", "irsa.ipac.caltech.edu")?;
    let parsed = parse_json(std::str::from_utf8(&bytes).ok()?)?;
    let arr = as_arr(&parsed)?;
    let npix = (12 * NSIDE_CMB * NSIDE_CMB) as usize;
    if arr.len() != npix {
        eprintln!("cmb rows {} != 12*NSIDE^2 {npix}", arr.len());
        return None;
    }
    let mut t = Vec::with_capacity(npix);
    for row in arr {
        let v = jnum(row, "T")?;
        if !v.is_finite() {
            return None;
        }
        t.push(v as f32);
    }
    Some(t)
}

fn load_galaxies() -> Option<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    let bytes = fetch_cached("cosmicflows_cf4.json", "ssd.jpl.nasa.gov")?;
    let parsed = parse_json(std::str::from_utf8(&bytes).ok()?)?;
    let arr = as_arr(&parsed)?;
    let ncells = (12 * NSIDE_CELL * NSIDE_CELL) as usize;
    let mut count = vec![0u64; ncells];
    let mut depth_sum = vec![0.0f64; ncells];
    let mut ra_all = Vec::new();
    let mut dec_all = Vec::new();
    let mut dist_all = Vec::new();
    let mut vpec_all = Vec::new();
    for row in arr {
        let ra = jnum(row, "ra")?;
        let dec = jnum(row, "dec")?;
        let dist = jnum(row, "dist_mpc")?;
        let vpec = jnum(row, "vpec")?;
        if !ra.is_finite() || !dec.is_finite() || !dist.is_finite() || !vpec.is_finite() {
            continue;
        }
        let (theta, phi) = icrs_to_galactic(ra, dec);
        if let Some(cell) = ang2pix_nest(NSIDE_CELL, theta, phi) {
            count[cell as usize] += 1;
            depth_sum[cell as usize] += dist;
        }
        ra_all.push(ra);
        dec_all.push(dec);
        dist_all.push(dist);
        vpec_all.push(vpec);
    }
    let _ = (ra_all, dec_all);
    Some((
        count.iter().map(|&c| c as f64).collect(),
        depth_sum,
        dist_all,
        vpec_all,
    ))
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let var = v.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / n;
    (m, var.sqrt())
}

fn threshold_of(surrogates: &[f64]) -> f64 {
    let (m, sd) = mean_sd(surrogates);
    m + 2.0 * sd
}

fn surrogate_tes(target: &[f32], driver: &[f32], lag: usize, seed: u64) -> Vec<f64> {
    let mut rng = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut vals = Vec::new();
    for _ in 0..SURROGATES {
        let ys = phase_randomized_surrogate(driver, &mut rng);
        if let Some(te) = transfer_entropy_lag(target, &ys, lag) {
            vals.push(te);
        }
    }
    vals
}

fn pair_te(x: &[f32], y: &[f32], lag: usize) -> Option<(f64, f64, f64, f64)> {
    let te_fwd = transfer_entropy_lag(y, x, lag)?;
    let te_rev = transfer_entropy_lag(x, y, lag)?;
    let surr = surrogate_tes(y, x, lag, SEED);
    let thr = threshold_of(&surr);
    let s_max = surr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some((te_fwd, te_rev, thr, s_max))
}

fn main() {
    let Some(cmb) = load_cmb() else {
        eprintln!("cmb fetch/parse void");
        std::process::exit(1);
    };
    let Some((count, depth_sum, dist, vpec)) = load_galaxies() else {
        eprintln!("galaxy fetch/parse void");
        std::process::exit(1);
    };
    let ncells = (12 * NSIDE_CELL * NSIDE_CELL) as usize;
    let ratio = (NSIDE_CMB / NSIDE_CELL) as usize;
    let ppc = ratio * ratio;
    let mut x: Vec<f32> = Vec::new();
    let mut dens: Vec<f32> = Vec::new();
    let mut depth: Vec<f32> = Vec::new();
    for c in 0..ncells {
        if count[c] < MIN_PER_CELL as f64 {
            continue;
        }
        let mut csum = 0.0f64;
        for p in 0..ppc {
            csum += cmb[c * ppc + p] as f64;
        }
        x.push((csum / ppc as f64) as f32);
        dens.push(count[c] as f32);
        depth.push((depth_sum[c] / count[c]) as f32);
    }
    let n = x.len();
    eprintln!(
        "angular series: {n} cells (n>={MIN_PER_CELL}), density mean {:.1} gal/cell, depth mean {:.1} Mpc",
        mean_sd(&dens.iter().map(|&v| v as f64).collect::<Vec<_>>()).0,
        mean_sd(&depth.iter().map(|&v| v as f64).collect::<Vec<_>>()).0,
    );
    if n < 24 {
        eprintln!("series too short — no statement");
        std::process::exit(1);
    }

    println!(
        "=== Blatt XII — big-bang echo: TE(CMB δT → galaxy density) over the angular series (Nside {NSIDE_CELL}) ==="
    );
    println!("cells with n>={MIN_PER_CELL}: {n}");
    let mut fam = f64::NEG_INFINITY;
    for lag in LAG_MIN..=LAG_MAX {
        let Some((fwd, rev, thr, s)) = pair_te(&x, &dens, lag) else {
            println!("  lag {lag}: TE void");
            continue;
        };
        if s > fam {
            fam = s;
        }
        let word = if fwd > fam {
            "fam-carrying"
        } else if fwd > thr {
            "over own threshold"
        } else {
            "still"
        };
        println!(
            "  lag {lag}: TE(CMB→density) {fwd:.4e}  TE(density→CMB) {rev:.4e}  thr {thr:.4e}  asym {:+.4e}  | {word}",
            fwd - rev
        );
    }
    println!("fam (multiple comparison) = {fam:.4e}");

    println!("\n=== The z series — the depth is the time axis of creation ===");
    let bw = DEPTH_MAX_MPC / DEPTH_BINS as f64;
    let mut bin_n = vec![0u64; DEPTH_BINS];
    let mut bin_v = vec![0.0f64; DEPTH_BINS];
    for (&d, &v) in dist.iter().zip(&vpec) {
        if d <= 0.0 || d >= DEPTH_MAX_MPC {
            continue;
        }
        let b = ((d / bw) as usize).min(DEPTH_BINS - 1);
        bin_n[b] += 1;
        bin_v[b] += v;
    }
    for b in 0..DEPTH_BINS {
        let lo = b as f64 * bw;
        let hi = (b as f64 + 1.0) * bw;
        let mean_v = if bin_n[b] > 0 {
            bin_v[b] / bin_n[b] as f64
        } else {
            f64::NAN
        };
        let n_b = bin_n[b];
        println!(
            "  z series [{lo:5.0}..{hi:5.0}] Mpc: {n_b:6} galaxies, vpec mean {mean_v:8.1} km/s"
        );
    }
    println!(
        "  t = 0 refused: the deepest measurable surface is the CMB (z = 1100) — behind it no source carries samples (0 honored)."
    );

    println!(
        "\n=== Self-test — does the machine carry an artificial arrow (density[t] = 3·δT[t−1])? ==="
    );
    let mut y_synth = vec![0.0f32; n];
    for t in 1..n {
        y_synth[t] = 3.0 * x[t - 1];
    }
    let Some((fwd, _, thr, _)) = pair_te(&x, &y_synth, 1) else {
        eprintln!("selbsttest TE void");
        std::process::exit(1);
    };
    println!(
        "  TE(δT→3δT(t−1)) @ lag 1 = {fwd:.4e} (thr {thr:.4e}) | {}",
        if fwd > thr { "found" } else { "missed" }
    );

    println!(
        "\nNANOGrav residuals (second channel): route open — pending (nanograv.org, to verify)."
    );
}
