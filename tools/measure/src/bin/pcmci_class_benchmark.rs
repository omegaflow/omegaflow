use omegaflow::te::{pcmci_links, TeNull};
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const SEED: u64 = 0x9E37_79B9_7F4A_7C15;
const MAX_LAG: usize = 2;
const BINS: usize = 4;
const BURN: usize = 200;

static NULL_LAG: AtomicUsize = AtomicUsize::new(12);
static N_SURR: AtomicUsize = AtomicUsize::new(100);
static NULL_MODEL: AtomicU8 = AtomicU8::new(1);
static BLOCK: AtomicUsize = AtomicUsize::new(0);

fn null_model() -> TeNull {
    match NULL_MODEL.load(Ordering::Relaxed) {
        0 => TeNull::Residual,
        2 => TeNull::Shift,
        _ => TeNull::Block,
    }
}

fn next_rng(rng: &mut u64) -> f64 {
    *rng = rng
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
}

fn uniform_signed(rng: &mut u64) -> f32 {
    (next_rng(rng) * 2.0 - 1.0) as f32
}

fn gauss(rng: &mut u64) -> f32 {
    loop {
        let u1 = uniform_signed(rng);
        let u2 = uniform_signed(rng);
        let s = u1 * u1 + u2 * u2;
        if s >= 1.0 || s <= 0.0 {
            continue;
        }
        let m = (-2.0 * (s as f64).ln() / (s as f64)).sqrt() as f32;
        return u1 * m;
    }
}

struct S60Link {
    driver: usize,
    target: usize,
    lag: usize,
    sign: f32,
    kind: usize,
}

fn s60_f(x: f32, kind: usize) -> f32 {
    let half_sq = 0.5 * x * x;
    match kind {
        0 => x,
        1 => (1.0 - 4.0 * (-half_sq).exp()) * x,
        _ => (1.0 - 4.0 * x * x * x * (-half_sq).exp()) * x,
    }
}

fn s60_topology(
    n_chan: usize,
    mixed: bool,
    a_set: &[f32],
    rng: &mut u64,
) -> (Vec<f32>, Vec<S60Link>) {
    let mut links: Vec<S60Link> = Vec::new();
    let l = if n_chan == 2 { 1 } else { n_chan };
    let mut used: Vec<(usize, usize)> = Vec::new();
    while links.len() < l {
        let driver = (next_rng(rng) * n_chan as f64) as usize;
        let target = (next_rng(rng) * n_chan as f64) as usize;
        if driver == target || used.contains(&(driver, target)) {
            continue;
        }
        used.push((driver, target));
        let lag = 1 + (next_rng(rng) * 2.0) as usize;
        let sign = if next_rng(rng) < 0.5 { -1.0 } else { 1.0 };
        let kind = if mixed {
            let u = next_rng(rng);
            if u < 0.5 {
                0
            } else if u < 0.75 {
                1
            } else {
                2
            }
        } else {
            0
        };
        links.push(S60Link {
            driver,
            target,
            lag,
            sign,
            kind,
        });
    }
    let a: Vec<f32> = (0..n_chan)
        .map(|_| a_set[(next_rng(rng) * a_set.len() as f64) as usize])
        .collect();
    (a, links)
}

fn s60_series(
    n_chan: usize,
    t: usize,
    c: f32,
    a: &[f32],
    links: &[S60Link],
    rng: &mut u64,
) -> Option<Vec<Vec<f32>>> {
    let mut x = vec![vec![0f32; BURN + t]; n_chan];
    for step in 1..BURN + t {
        for j in 0..n_chan {
            let mut v = a[j] * x[j][step - 1];
            for lk in links.iter().filter(|l| l.target == j) {
                if step >= lk.lag {
                    v += c * lk.sign * s60_f(x[lk.driver][step - lk.lag], lk.kind);
                }
            }
            v += gauss(rng);
            x[j][step] = v;
        }
    }
    let mut out = Vec::with_capacity(n_chan);
    for j in 0..n_chan {
        let col: Vec<f32> = x[j][BURN..].to_vec();
        for &v in &col {
            if !v.is_finite() || v.abs() > 100.0 {
                return None;
            }
        }
        out.push(col);
    }
    Some(out)
}

fn measure(
    series: &[Vec<f32>],
    true_links: &[(usize, usize, usize)],
    max_lag: usize,
    bins: usize,
    seed: u64,
) -> Option<(Vec<bool>, usize, usize)> {
    let refs: Vec<&[f32]> = series.iter().map(|s| s.as_slice()).collect();
    let links = pcmci_links(
        &refs,
        max_lag,
        NULL_LAG.load(Ordering::Relaxed),
        bins,
        seed,
        N_SURR.load(Ordering::Relaxed),
        null_model(),
        BLOCK.load(Ordering::Relaxed),
    )?;
    let n_chan = series.len();
    let found: Vec<bool> = true_links
        .iter()
        .map(|&(d, t, l)| {
            links
                .iter()
                .any(|k| k.driver == d && k.target == t && k.lag == l && k.te > k.threshold)
        })
        .collect();
    let mut fp = 0usize;
    let mut neg = 0usize;
    for d in 0..n_chan {
        for t in 0..n_chan {
            if d == t {
                continue;
            }
            for lag in 1..=max_lag {
                if true_links.contains(&(d, t, lag)) {
                    continue;
                }
                neg += 1;
                if links
                    .iter()
                    .any(|k| k.driver == d && k.target == t && k.lag == lag && k.te > k.threshold)
                {
                    fp += 1;
                }
            }
        }
    }
    Some((found, fp, neg))
}

fn print_point(
    label: &str,
    hits: &[(usize, usize)],
    fp: usize,
    neg: usize,
    excluded: usize,
    void: usize,
    top_redraws: usize,
    top_excl: usize,
) {
    let realized: Vec<(usize, usize)> = hits.iter().copied().filter(|&(_, s)| s > 0).collect();
    let unrealized = hits.len() - realized.len();
    if realized.is_empty() {
        println!(
            "    {label}: links unrealized={unrealized} excluded={excluded} void={void} top_redraws={top_redraws} top_excl={top_excl}"
        );
        return;
    }
    let mut fracs: Vec<f64> = realized.iter().map(|&(h, s)| h as f64 / s as f64).collect();
    fracs.sort_by(|a, b| a.total_cmp(b));
    let n = fracs.len();
    let min = fracs[0];
    let med = fracs[n / 2];
    let max = fracs[n - 1];
    let above = fracs.iter().filter(|&&f| f > 0.7).count();
    let fpr = if neg > 0 {
        100.0 * fp as f64 / neg as f64
    } else {
        0.0
    };
    println!(
        "    {label}: links={n} unrealized={unrealized} power min/med/max={min:.2}/{med:.2}/{max:.2} links>70%={above}/{n} FPR={fpr:.2}% (neg={neg}) excluded={excluded} void={void} top_redraws={top_redraws} top_excl={top_excl}"
    );
}

fn s60_point(
    n_chan: usize,
    t: usize,
    c: f32,
    mixed: bool,
    a_set: &[f32],
    label: &str,
    r: usize,
    s: usize,
    max_lag: usize,
    bins: usize,
    seed: u64,
) {
    let mut hit_pairs: Vec<(usize, usize)> = Vec::new();
    let mut fp = 0usize;
    let mut neg = 0usize;
    let mut excluded = 0usize;
    let mut void = 0usize;
    let mut top_redraws = 0usize;
    let mut top_excl = 0usize;
    for ri in 0..r {
        let mut trng = seed.wrapping_add((ri as u64).wrapping_mul(0x9E37_79B9));
        let mut accepted: Option<(Vec<f32>, Vec<S60Link>)> = None;
        for _attempt in 0..50 {
            let (a, links) = s60_topology(n_chan, mixed, a_set, &mut trng);
            let mut grng = seed
                .wrapping_add((ri as u64).wrapping_mul(0x85EB_CA6B))
                .wrapping_add((top_redraws as u64).wrapping_mul(0x3C1D_9E4F));
            if s60_series(n_chan, t, c, &a, &links, &mut grng).is_some() {
                accepted = Some((a, links));
                break;
            }
            top_redraws += 1;
        }
        let Some((a, links)) = accepted else {
            top_excl += 1;
            continue;
        };
        let true_links: Vec<(usize, usize, usize)> =
            links.iter().map(|l| (l.driver, l.target, l.lag)).collect();
        let mut hits = vec![0usize; links.len()];
        let mut top_excluded = 0usize;
        for si in 0..s {
            let mut grng = seed
                .wrapping_add((ri as u64).wrapping_mul(0x85EB_CA6B))
                .wrapping_add((si as u64).wrapping_mul(0xC2B2_AE3D));
            let mut series = None;
            for _attempt in 0..20 {
                series = s60_series(n_chan, t, c, &a, &links, &mut grng);
                if series.is_some() {
                    break;
                }
            }
            let Some(ser) = series else {
                excluded += 1;
                top_excluded += 1;
                continue;
            };
            match measure(
                &ser,
                &true_links,
                max_lag,
                bins,
                SEED.wrapping_add(ri as u64),
            ) {
                Some((found, f, n)) => {
                    for (k, &fnd) in found.iter().enumerate() {
                        if fnd {
                            hits[k] += 1;
                        }
                    }
                    fp += f;
                    neg += n;
                }
                None => void += 1,
            }
        }
        for &h in &hits {
            hit_pairs.push((h, s - top_excluded));
        }
    }
    let full = format!("{label} R={r} S={s}");
    print_point(
        &full,
        &hit_pairs,
        fp,
        neg,
        excluded,
        void,
        top_redraws,
        top_excl,
    );
}

fn chaos_maps(n: usize, sigma: f32, rng: &mut u64) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut z = 0.3f32;
    let mut x = 0.5f32;
    let mut y = 0.7f32;
    let mut zs = vec![0f32; n];
    let mut xs = vec![0f32; n];
    let mut ys = vec![0f32; n];
    for step in 0..BURN + n {
        let zz = (z * (4.0 - 4.0 * z + sigma * uniform_signed(rng))).rem_euclid(1.0);
        let xx = (x * (4.0 - 4.0 * x - z + sigma * uniform_signed(rng))).rem_euclid(1.0);
        let yy = (y * (4.0 - 4.0 * y - z + sigma * uniform_signed(rng))).rem_euclid(1.0);
        z = zz;
        x = xx;
        y = yy;
        if step >= BURN {
            zs[step - BURN] = z;
            xs[step - BURN] = x;
            ys[step - BURN] = y;
        }
    }
    (zs, xs, ys)
}

fn common_driver(
    n: usize,
    a: f32,
    c: f32,
    d_z: usize,
    b: f32,
    sigma_z: f32,
    rng: &mut u64,
) -> Vec<Vec<f32>> {
    let n_chan = 2 + d_z;
    let mut x = vec![vec![0f32; BURN + n]; n_chan];
    for step in 1..BURN + n {
        x[0][step] = a * x[0][step - 1] + gauss(rng);
        let mut yv = a * x[1][step - 1] + c * x[0][step - 1] + gauss(rng);
        for d in 0..d_z {
            x[2 + d][step] = a * x[2 + d][step - 1] + sigma_z * gauss(rng);
            yv += b * x[2 + d][step - 1];
        }
        x[1][step] = yv;
    }
    let mut out = Vec::with_capacity(n_chan);
    for j in 0..n_chan {
        out.push(x[j][BURN..].to_vec());
    }
    out
}

fn mute_network(n: usize, rng: &mut u64) -> Vec<Vec<f32>> {
    let sq2 = 2.0f32.sqrt();
    let mut x = vec![vec![0f32; n]; 5];
    for j in 0..5 {
        for t in 0..3 {
            x[j][t] = gauss(rng);
        }
    }
    for t in 3..n {
        let x0_1 = x[0][t - 1];
        let x0_2 = x[0][t - 2];
        let x0_3 = x[0][t - 3];
        x[0][t] = 0.95 * sq2 * x0_1 - 0.9025 * x0_2 + gauss(rng);
        x[1][t] = 0.5 * x0_2 * x0_2 + gauss(rng);
        x[2][t] = -0.4 * x0_3 + gauss(rng);
        x[3][t] =
            -0.5 * x0_2 * x0_2 + 0.25 * sq2 * x[3][t - 1] + 0.25 * sq2 * x[4][t - 1] + gauss(rng);
        x[4][t] = -0.25 * sq2 * x[3][t - 1] + 0.25 * sq2 * x[4][t - 1] + gauss(rng);
    }
    x
}

fn tigramite_overview(n: usize, rng: &mut u64) -> Vec<Vec<f32>> {
    let mut x = vec![vec![0f32; n]; 4];
    for t in 0..n {
        let x0 = if t == 0 { 0.0 } else { x[0][t - 1] };
        let x1 = if t == 0 { 0.0 } else { x[1][t - 1] };
        let x2 = if t == 0 { 0.0 } else { x[2][t - 1] };
        let x3 = if t == 0 { 0.0 } else { x[3][t - 1] };
        let x1_2 = if t >= 2 { x[1][t - 2] } else { 0.0 };
        let x3_3 = if t >= 3 { x[3][t - 3] } else { 0.0 };
        x[0][t] = 0.7 * x0 - 0.8 * x1 + gauss(rng);
        x[1][t] = 0.8 * x1 + 0.8 * x3 + gauss(rng);
        x[2][t] = 0.5 * x2 + 0.5 * x1_2 + 0.6 * x3_3 + gauss(rng);
        x[3][t] = 0.4 * x3 + gauss(rng);
    }
    x
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let quick = args.iter().any(|a| a == "--quick");
    let null_arg = args
        .iter()
        .position(|a| a == "--null")
        .and_then(|p| args.get(p + 1))
        .cloned();
    match null_arg.as_deref() {
        None => {}
        Some("residual") => NULL_MODEL.store(0, Ordering::Relaxed),
        Some("block") => NULL_MODEL.store(1, Ordering::Relaxed),
        Some("shift") => NULL_MODEL.store(2, Ordering::Relaxed),
        Some(other) => {
            eprintln!("--null carries {other} — the probe builds residual, block, shift");
            std::process::exit(1);
        }
    }
    let n_surr_arg = args
        .iter()
        .position(|a| a == "--n-surr")
        .and_then(|p| args.get(p + 1))
        .cloned();
    if let Some(v) = n_surr_arg {
        match v.parse::<usize>() {
            Ok(n) if n >= 2 => N_SURR.store(n, Ordering::Relaxed),
            Ok(n) => {
                eprintln!("--n-surr carries {n} — a null needs at least 2 surrogates");
                std::process::exit(1);
            }
            Err(_) => {
                eprintln!("--n-surr carries {v} — not a surrogate count");
                std::process::exit(1);
            }
        }
    }
    let block_arg = args
        .iter()
        .position(|a| a == "--block")
        .and_then(|p| args.get(p + 1))
        .cloned();
    if let Some(v) = block_arg {
        match v.parse::<usize>() {
            Ok(n) => BLOCK.store(n, Ordering::Relaxed),
            Err(_) => {
                eprintln!("--block carries {v} — not a block length");
                std::process::exit(1);
            }
        }
    }
    let div = |s: usize| if quick { (s / 5).max(2) } else { s };
    let top = |r: usize| if quick { 1 } else { r };
    println!("=== PCMCI class benchmark — pcmci_links against the published suite ===");
    println!(
        "machine operating point: max_lag {MAX_LAG} null_lag {} bins {BINS} n_surr {} null {} block {} seed {SEED:#X} quick={quick}",
        NULL_LAG.load(Ordering::Relaxed),
        N_SURR.load(Ordering::Relaxed),
        match null_model() {
            TeNull::Residual => "residual",
            TeNull::Block => "block",
            TeNull::Shift => "shift",
        },
        BLOCK.load(Ordering::Relaxed)
    );

    println!();
    println!("[1] Sci. Adv. 5, eaau4996 (arXiv:1702.07007v2), SM Eq. (S60) — random lagged VAR, T=150, tau in {{1,2}}:");
    println!("    published text anchors: FP around/below 5% (Fig. 4C, all N); PCMCI: 99% of links with power > 70% at N=10; FullCI power 80% (N=5) -> 40% (N=20)");
    let set1 = [0.0f32, 0.2, 0.4, 0.6, 0.8, 0.9];
    for n_chan in [2usize, 5, 10] {
        s60_point(
            n_chan,
            150,
            0.287,
            false,
            &set1,
            &format!("N={n_chan} c=0.287 a-set1"),
            top(3),
            div(20),
            2,
            BINS,
            SEED,
        );
    }
    let set2 = [0.6f32, 0.8, 0.9, 0.95];
    s60_point(
        10,
        150,
        0.287,
        false,
        &set2,
        "N=10 c=0.287 a-set2 (strong autocorr)",
        top(2),
        div(20),
        2,
        BINS,
        SEED,
    );
    s60_point(
        10,
        150,
        0.287,
        false,
        &set1,
        "N=10 c=0.287 a-set1 max_lag=5 (published lag budget)",
        top(2),
        div(10),
        5,
        BINS,
        SEED,
    );
    println!("    c-scaling (Fig. 6, qualitative):");
    for c in [0.2f32, 0.247, 0.324, 0.414] {
        s60_point(
            10,
            150,
            c,
            false,
            &set1,
            &format!("N=10 c={c}"),
            top(2),
            div(10),
            2,
            BINS,
            SEED,
        );
    }
    println!("    T-scaling (Fig. S8, qualitative):");
    for t in [150usize, 300, 600] {
        s60_point(
            10,
            t,
            0.2,
            false,
            &set1,
            &format!("N=10 T={t} c=0.2"),
            top(2),
            div(10),
            2,
            BINS,
            SEED,
        );
    }
    println!("    bins sweep at the N=10 anchor (machine config surface):");
    for bins in [3usize, 8] {
        s60_point(
            10,
            150,
            0.287,
            false,
            &set1,
            &format!("N=10 c=0.287 a-set1 bins={bins}"),
            top(2),
            div(10),
            2,
            bins,
            SEED,
        );
    }

    println!();
    println!("[2] Sci. Adv. same model, nonlinear mix 50% f1 / 25% f2 / 25% f3:");
    println!("    published text anchors: PCMCI highest power; slight FP inflation at large N (Fig. 5A/B)");
    for n_chan in [5usize, 10] {
        s60_point(
            n_chan,
            150,
            0.287,
            true,
            &set1,
            &format!("N={n_chan} c=0.287 mixed"),
            top(2),
            div(10),
            2,
            BINS,
            SEED,
        );
    }

    println!();
    println!(
        "[3] Chaos 28, 075310 (2018) §VII.A, Eqs. (36)/(37) — coupled logistic maps, r=4, n=150:"
    );
    println!("    published text anchors: plain PCMCI almost no power at sigma=0; PCMCI0 rate 0.8 at sigma=0; power peak at sigma=0.2; PCMCI FP ~0.05");
    for sigma in [0.0f32, 0.2, 0.4] {
        let s = div(50);
        let mut zx = 0usize;
        let mut zy = 0usize;
        let mut fp = 0usize;
        let mut neg = 0usize;
        let mut void = 0usize;
        for si in 0..s {
            let mut rng = SEED
                .wrapping_add((si as u64).wrapping_mul(0x9E37_79B9))
                .wrapping_add((sigma.to_bits() as u64).wrapping_mul(0x85EB_CA6B));
            let (z, x, y) = chaos_maps(150, sigma, &mut rng);
            let series = [z, x, y];
            let true_links = [(0usize, 1usize, 1usize), (0, 2, 1)];
            match measure(&series, &true_links, 2, BINS, SEED) {
                Some((found, f, n)) => {
                    if found[0] {
                        zx += 1;
                    }
                    if found[1] {
                        zy += 1;
                    }
                    fp += f;
                    neg += n;
                }
                None => void += 1,
            }
        }
        let fpr = if neg > 0 {
            100.0 * fp as f64 / neg as f64
        } else {
            0.0
        };
        println!("    sigma={sigma}: Z->X {zx}/{s} Z->Y {zy}/{s} FP={fp} FPR={fpr:.2}% (neg={neg}) void={void}");
    }

    println!();
    println!("[4] Chaos 28, 075310 (2018) §VII.B — linear autocorrelation + common drivers, n=150, b=0.5, sigma_z=0.25:");
    println!("    published text anchors: PCMCI FP well-controlled (~0.05) with TP levels constant across autocorrelation a; the published b(D_Z,a) calibration is figure-only, the probe sets b/sigma_z explicitly");
    for d_z in [0usize, 4] {
        for a in [0.0f32, 0.5, 0.9] {
            let s = div(20);
            let mut fp = 0usize;
            let mut neg = 0usize;
            let mut void = 0usize;
            for si in 0..s {
                let mut rng = SEED
                    .wrapping_add((si as u64).wrapping_mul(0x9E37_79B9))
                    .wrapping_add((d_z as u64).wrapping_mul(0x85EB_CA6B))
                    .wrapping_add((a.to_bits() as u64).wrapping_mul(0xC2B2_AE3D));
                let series = common_driver(150, a, 0.0, d_z, 0.5, 0.25, &mut rng);
                match measure(&series, &[], 2, BINS, SEED) {
                    Some((_, f, n)) => {
                        fp += f;
                        neg += n;
                    }
                    None => void += 1,
                }
            }
            let mut hit = 0usize;
            let mut realized = 0usize;
            for si in 0..s {
                let mut rng = SEED
                    .wrapping_add((si as u64).wrapping_mul(0x3C1D_9E4F))
                    .wrapping_add((d_z as u64).wrapping_mul(0x85EB_CA6B))
                    .wrapping_add((a.to_bits() as u64).wrapping_mul(0xC2B2_AE3D));
                let series = common_driver(150, a, 0.3, d_z, 0.5, 0.25, &mut rng);
                match measure(&series, &[(0usize, 1usize, 1usize)], 2, BINS, SEED) {
                    Some((found, _, _)) => {
                        realized += 1;
                        if found[0] {
                            hit += 1;
                        }
                    }
                    None => void += 1,
                }
            }
            let fpr = if neg > 0 {
                100.0 * fp as f64 / neg as f64
            } else {
                0.0
            };
            println!(
                "    D_Z={d_z} a={a}: c=0 FPR={fpr:.2}% (neg={neg}) c=0.3 TPR={hit}/{realized} void={void}"
            );
        }
    }

    println!();
    println!("[5] IDTxl MuTE network (Wollstadt et al. 2019, JOSS 10.21105/joss.01081; idtxl/data.py:849) — n=1000, max_lag 3:");
    println!(
        "    published recovery numbers: none (generator only) — the machine's sheet stands alone"
    );
    {
        let s = div(5);
        let true_links = [
            (0usize, 1usize, 2usize),
            (0, 2, 3),
            (0, 3, 2),
            (3, 4, 1),
            (4, 3, 1),
        ];
        let mut hits = [0usize; 5];
        let mut fp = 0usize;
        let mut neg = 0usize;
        let mut void = 0usize;
        for si in 0..s {
            let mut rng = SEED.wrapping_add((si as u64).wrapping_mul(0x85EB_CA6B));
            let series = mute_network(1000, &mut rng);
            match measure(&series, &true_links, 3, BINS, SEED) {
                Some((found, f, n)) => {
                    for (k, &fnd) in found.iter().enumerate() {
                        if fnd {
                            hits[k] += 1;
                        }
                    }
                    fp += f;
                    neg += n;
                }
                None => void += 1,
            }
        }
        let fpr = if neg > 0 {
            100.0 * fp as f64 / neg as f64
        } else {
            0.0
        };
        println!(
            "    x0->x1 lag2 {}/{}  x0->x2 lag3 {}/{}  x0->x3 lag2 {}/{}  x3->x4 lag1 {}/{}  x4->x3 lag1 {}/{}  FPR={fpr:.2}% (neg={neg}) void={void}",
            hits[0], s, hits[1], s, hits[2], s, hits[3], s, hits[4], s
        );
    }

    println!();
    println!("[6] Tigramite overview linear model (tutorials/causal_discovery/tigramite_tutorial_causal_discovery_overview.ipynb) — n=1000, max_lag 3:");
    println!("    published recovery numbers: none (single-run tutorial) — the machine's sheet stands alone");
    {
        let s = div(5);
        let true_links = [(1usize, 0usize, 1usize), (3, 1, 1), (1, 2, 2), (3, 2, 3)];
        let mut hits = [0usize; 4];
        let mut fp = 0usize;
        let mut neg = 0usize;
        let mut void = 0usize;
        for si in 0..s {
            let mut rng = SEED.wrapping_add((si as u64).wrapping_mul(0xC2B2_AE3D));
            let series = tigramite_overview(1000, &mut rng);
            match measure(&series, &true_links, 3, BINS, SEED) {
                Some((found, f, n)) => {
                    for (k, &fnd) in found.iter().enumerate() {
                        if fnd {
                            hits[k] += 1;
                        }
                    }
                    fp += f;
                    neg += n;
                }
                None => void += 1,
            }
        }
        let fpr = if neg > 0 {
            100.0 * fp as f64 / neg as f64
        } else {
            0.0
        };
        println!(
            "    x1->x0 lag1 {}/{}  x3->x1 lag1 {}/{}  x1->x2 lag2 {}/{}  x3->x2 lag3 {}/{}  FPR={fpr:.2}% (neg={neg}) void={void}",
            hits[0], s, hits[1], s, hits[2], s, hits[3], s
        );
    }
}
