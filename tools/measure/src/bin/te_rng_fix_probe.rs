use omegaflow::te::{embed_series, find_mi_lag, transfer_entropy_embedded};

const N_TRIALS: usize = 30;
const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const DIM: usize = 3;

fn next_rng_fixed(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn ar1_fixed(n: usize, phi: f64, rng: &mut u64) -> Vec<f32> {
    let mut v = Vec::with_capacity(n);
    let mut x = 0.0f64;
    for _ in 0..n {
        x = phi * x + next_rng_fixed(rng) * 2.0 - 1.0;
        v.push(x as f32);
    }
    v
}

fn fft(re: &mut [f64], im: &mut [f64], inverse: bool) {
    let n = re.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2usize;
    while len <= n {
        let ang = if inverse {
            2.0 * std::f64::consts::PI / len as f64
        } else {
            -2.0 * std::f64::consts::PI / len as f64
        };
        let (s, c) = ang.sin_cos();
        let mut i = 0usize;
        while i < n {
            let mut w_re = 1.0f64;
            let mut w_im = 0.0f64;
            for k in 0..len / 2 {
                let u_re = re[i + k];
                let u_im = im[i + k];
                let v_re = re[i + k + len / 2] * w_re - im[i + k + len / 2] * w_im;
                let v_im = re[i + k + len / 2] * w_im + im[i + k + len / 2] * w_re;
                re[i + k] = u_re + v_re;
                im[i + k] = u_im + v_im;
                re[i + k + len / 2] = u_re - v_re;
                im[i + k + len / 2] = u_im - v_im;
                let w2_re = w_re * c - w_im * s;
                let w2_im = w_re * s + w_im * c;
                w_re = w2_re;
                w_im = w2_im;
            }
            i += len;
        }
        len <<= 1;
    }
    if inverse {
        let scale = 1.0 / n as f64;
        for (a, b) in re.iter_mut().zip(im.iter_mut()) {
            *a *= scale;
            *b *= scale;
        }
    }
}

fn surrogate_fixed(v: &[f32], rng: &mut u64) -> Vec<f32> {
    let n = v.len();
    if n < 2 {
        return v.to_vec();
    }
    let m = n.next_power_of_two();
    let mut re: Vec<f64> = vec![0.0; m];
    let mut im: Vec<f64> = vec![0.0; m];
    for (i, &x) in v.iter().enumerate() {
        re[i] = x as f64;
    }
    fft(&mut re, &mut im, false);
    for k in 1..m / 2 {
        let phi = next_rng_fixed(rng) * 2.0 * std::f64::consts::PI;
        let (s, c) = phi.sin_cos();
        let (ar, ai) = (re[k], im[k]);
        re[k] = ar * c - ai * s;
        im[k] = ar * s + ai * c;
        let j = m - k;
        re[j] = re[k];
        im[j] = -im[k];
    }
    fft(&mut re, &mut im, true);
    v.iter().enumerate().map(|(i, _)| re[i] as f32).collect()
}

fn embedded_silverman(emb: &[Vec<f64>]) -> Option<f64> {
    let n = emb.len() as f64;
    let dim = emb.first()?.len();
    if n < 2.0 || dim == 0 {
        return None;
    }
    let mut mean = vec![0.0; dim];
    for state in emb.iter() {
        for (k, &v) in state.iter().enumerate() {
            mean[k] += v;
        }
    }
    for k in 0..dim {
        mean[k] /= n;
    }
    let mut var = 0.0;
    for state in emb.iter() {
        let mut d2 = 0.0;
        for (k, &v) in state.iter().enumerate() {
            let d = v - mean[k];
            d2 += d * d;
        }
        var += d2;
    }
    var /= n;
    if var <= 0.0 {
        return None;
    }
    Some(1.06 * var.sqrt() * n.powf(-0.2))
}

fn main() {
    let mut rng = SEED;
    let mut fp = 0usize;
    let mut meas = 0usize;
    let mut te_sum = 0.0;
    for t in 0..N_TRIALS {
        let seed = SEED ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let a = ar1_fixed(300, 0.7, &mut rng);
        let b = ar1_fixed(300, 0.7, &mut rng);
        let xf: Vec<f64> = a.iter().map(|&v| v as f64).collect();
        let yf: Vec<f64> = b.iter().map(|&v| v as f64).collect();
        let (Some(tau_x), Some(tau_y)) = (find_mi_lag(&xf), find_mi_lag(&yf)) else {
            continue;
        };
        let emb_x = embed_series(&xf, tau_x, DIM);
        let emb_y = embed_series(&yf, tau_y, DIM);
        if emb_x.is_empty() || emb_y.is_empty() {
            continue;
        }
        let Some(te) = transfer_entropy_embedded(&xf, &emb_x, &emb_y, tau_x, tau_y) else {
            continue;
        };
        te_sum += te;
        let mut vals: Vec<f64> = Vec::with_capacity(10);
        let mut srng = seed.wrapping_add(0x9e3779b97f4a7c15);
        for _ in 0..10 {
            let ys = surrogate_fixed(&b, &mut srng);
            let ysf: Vec<f64> = ys.iter().map(|&v| v as f64).collect();
            let emb_s = embed_series(&ysf, tau_y, DIM);
            if emb_s.is_empty() {
                continue;
            }
            let h_check = embedded_silverman(&emb_s);
            if h_check.is_none() {
                continue;
            }
            if let Some(te_s) = transfer_entropy_embedded(&xf, &emb_x, &emb_s, tau_x, tau_y) {
                vals.push(te_s);
            }
        }
        if vals.len() >= 2 {
            let mean = vals.iter().sum::<f64>() / vals.len() as f64;
            let var =
                vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / vals.len() as f64;
            meas += 1;
            if te > mean + 2.0 * var.sqrt() {
                fp += 1;
            }
        }
    }
    println!(
        "FP with CORRECT RNG (full circle, phi ∈ [0, 2π)): {}/{} = {:.1} %, te mean {:.4e}",
        fp,
        meas,
        fp as f64 * 100.0 / meas.max(1) as f64,
        if meas > 0 {
            te_sum / meas as f64
        } else {
            f64::NAN
        }
    );
    println!(
        "Counter-value (canonical, broken half circle): 100 % — the row above reads whether the RNG is the root."
    );
}
