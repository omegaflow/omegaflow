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

fn load_velocities() -> Option<(Vec<f64>, Vec<u64>)> {
    let bytes = fetch_cached("cosmicflows_cf4.json", "ssd.jpl.nasa.gov")?;
    let parsed = parse_json(std::str::from_utf8(&bytes).ok()?)?;
    let arr = as_arr(&parsed)?;
    let ncells = (12 * NSIDE_CELL * NSIDE_CELL) as usize;
    let mut sum = vec![0.0f64; ncells];
    let mut n = vec![0u64; ncells];
    let mut total = 0u64;
    for row in arr {
        let ra = jnum(row, "ra")?;
        let dec = jnum(row, "dec")?;
        let vpec = jnum(row, "vpec")?;
        if !ra.is_finite() || !dec.is_finite() || !vpec.is_finite() {
            continue;
        }
        let (theta, phi) = icrs_to_galactic(ra, dec);
        let Some(cell) = ang2pix_nest(NSIDE_CELL, theta, phi) else {
            continue;
        };
        sum[cell as usize] += vpec;
        n[cell as usize] += 1;
        total += 1;
    }
    eprintln!("velocities: {total} assigned into {ncells} cells");
    Some((sum, n))
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

fn pair_te(x: &[f32], y: &[f32], lag: usize) -> Option<(f64, f64, f64, f64, f64)> {
    let te_fwd = transfer_entropy_lag(y, x, lag)?;
    let te_rev = transfer_entropy_lag(x, y, lag)?;
    let surr = surrogate_tes(y, x, lag, SEED);
    let thr = threshold_of(&surr);
    let fam_surr = surr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some((te_fwd, te_rev, thr, fam_surr, te_fwd - te_rev))
}

fn main() {
    let Some(cmb) = load_cmb() else {
        eprintln!("cmb fetch/parse void");
        std::process::exit(1);
    };
    let Some((vsum, vn)) = load_velocities() else {
        eprintln!("velocity fetch/parse void");
        std::process::exit(1);
    };
    let ncells = (12 * NSIDE_CELL * NSIDE_CELL) as usize;
    let ratio = (NSIDE_CMB / NSIDE_CELL) as usize;
    let ppc = ratio * ratio;
    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    for c in 0..ncells {
        if vn[c] < MIN_PER_CELL as u64 {
            continue;
        }
        let mut csum = 0.0f64;
        for p in 0..ppc {
            csum += cmb[c * ppc + p] as f64;
        }
        x.push((csum / ppc as f64) as f32);
        y.push((vsum[c] / vn[c] as f64) as f32);
    }
    eprintln!(
        "angular series: {} cells (n>={MIN_PER_CELL}), CMB mean {:.3e} K, vpec mean {:.1} km/s",
        x.len(),
        mean_sd(&x.iter().map(|&v| v as f64).collect::<Vec<_>>()).0,
        mean_sd(&y.iter().map(|&v| v as f64).collect::<Vec<_>>()).0,
    );
    if x.len() < 24 {
        eprintln!("series too short — no statement");
        std::process::exit(1);
    }

    println!(
        "=== Blatt VIII — dark flow: TE(CMB δT → vpec) over the angular series (Nside {NSIDE_CELL}) ==="
    );
    println!("cells with n>={MIN_PER_CELL}: {}", x.len());
    let mut fam = f64::NEG_INFINITY;
    for lag in LAG_MIN..=LAG_MAX {
        let Some((fwd, rev, thr, s, asym)) = pair_te(&x, &y, lag) else {
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
            "  lag {lag}: TE(CMB→v) {fwd:.4e}  TE(v→CMB) {rev:.4e}  thr {thr:.4e}  asym {asym:+.4e}  | {word}"
        );
    }
    println!("fam (multiple comparison) = {fam:.4e}");

    println!(
        "\n=== Self-test — does the machine carry an artificial arrow (v[t] = 3·δT[t−1])? ==="
    );
    let n = x.len();
    let mut y_synth = vec![0.0f32; n];
    for t in 1..n {
        y_synth[t] = 3.0 * x[t - 1];
    }
    let Some((fwd, _, thr, _, _)) = pair_te(&x, &y_synth, 1) else {
        eprintln!("selbsttest TE void");
        std::process::exit(1);
    };
    println!(
        "  TE(δT→3δT(t−1)) @ lag 1 = {fwd:.4e} (thr {thr:.4e}) | {}",
        if fwd > thr { "found" } else { "missed" }
    );
}
