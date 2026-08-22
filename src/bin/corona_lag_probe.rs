// Der sub-minütige Korona-Lag-Probe — die Messung, die das Rätsel angreift.
// Liest zwei NCEI-Dateien eines Flare-Tages: gxrs-l2-irrad (2-s-Röntgen,
// a_flux = kurzes Band 0,5–4 Å, Unix-Epoche) und geuv-l2-ir10s (10-s-EUV,
// irr_1216_1nm = Lyman-α, Epoche seit 2000-01-01T12:00 UTC → +946728000).
// Beide auf ein 10-s-Gitter (Median), dann TE in beide Richtungen über einen
// Lag-Sweep. Alfvén-Vorhersage: die Chromosphäre (Lyman-α) führt die Korona
// (Röntgen) um die Laufzeit (~100 s ≈ 10 Zellen); Nanoflares tragen keinen
// konsistenten Lag. src/te.rs bleibt unberührt (öffentliche API).

use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::te::{gaussian, silverman, transfer_entropy_lag};

const EPOCH_2000: f64 = 946728000.0;
const DT: f64 = 10.0;
const FILL: f64 = -9999.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn series(
    path: &str,
    tname: &str,
    vname: &str,
    fname: &str,
    flag_bytes: usize,
    gc_name: Option<&str>,
    t_offset: f64,
) -> Vec<(f64, f64)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{} reads void", path);
        return Vec::new();
    };
    let Ok(file) = Hdf5File::parse(&bytes) else {
        eprintln!("{} parses void", path);
        return Vec::new();
    };
    let Ok(t_raw) = file.read_dataset(tname) else {
        eprintln!("{}: dataset {} absent", path, tname);
        return Vec::new();
    };
    let Ok(v_raw) = file.read_dataset(vname) else {
        eprintln!("{}: dataset {} absent", path, vname);
        return Vec::new();
    };
    let Ok(f_raw) = file.read_dataset(fname) else {
        eprintln!("{}: dataset {} absent", path, fname);
        return Vec::new();
    };
    let gc_raw = match gc_name {
        Some(g) => file.read_dataset(g).ok(),
        None => None,
    };
    let n = t_raw.len() / 8;
    if v_raw.len() != n * 4 || f_raw.len() != n * flag_bytes {
        eprintln!("{}: shapes carry no common row count", path);
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let Some(t) = decode_f64(&t_raw, i * 8, Endian::Le) else {
            continue;
        };
        let flag: u16 = if flag_bytes == 2 {
            u16::from_le_bytes([f_raw[i * 2], f_raw[i * 2 + 1]])
        } else {
            f_raw[i] as u16
        };
        if flag != 0 {
            continue;
        }
        if let Some(g) = &gc_raw {
            if g[i] != 0 {
                continue;
            }
        }
        let Some(v) = decode_f32(&v_raw, i * 4, Endian::Le) else {
            continue;
        };
        let v = v as f64;
        if !v.is_finite() || v == FILL || v <= 0.0 {
            continue;
        }
        out.push((t + t_offset, v));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn bin_median(series: &[(f64, f64)], t0: f64, bins: usize) -> Vec<Option<f32>> {
    let mut sums = vec![0.0f64; bins];
    let mut cnt = vec![0u32; bins];
    let mut vals: Vec<Vec<f64>> = vec![Vec::new(); bins];
    for &(t, v) in series {
        let idx = ((t - t0) / DT).floor();
        if idx < 0.0 || idx >= bins as f64 {
            continue;
        }
        vals[idx as usize].push(v);
        sums[idx as usize] += v;
        cnt[idx as usize] += 1;
    }
    (0..bins)
        .map(|i| {
            if cnt[i] > 0 {
                let mut v = std::mem::take(&mut vals[i]);
                v.sort_by(|a, b| a.total_cmp(b));
                let m = if v.len() % 2 == 0 {
                    (v[v.len() / 2 - 1] + v[v.len() / 2]) * 0.5
                } else {
                    v[v.len() / 2]
                };
                Some(m as f32)
            } else {
                None
            }
        })
        .collect()
}

// Bedingte TE: TE(y→x | z) — dieselbe KDE-Ratio wie der skalare Pfad, aber
// jede Dichte trägt zusätzlich den Zustand z_t der Konditions-Kraft. Lag = 0
// heißt wie beim skalaren Pfad ein Zeitschritt (10 s), lag > 0 heißt
// τ Zeitschritte. Ein gemeinsamer Treiber, der y und x zusammen treibt,
// verschwindet, sobald z seinen Anteil erklärt — das ist der Kern der
// Multi-Force-TE (nobel_probe_corona v2), hier als lokale Variante.
fn te_conditional(x: &[f32], y: &[f32], z: &[f32], lag: usize) -> Option<f64> {
    let n = x.len();
    if n < 8 {
        return None;
    }
    let hx = silverman(x)?;
    let hy = silverman(y)?;
    let hz = silverman(z)?;
    let m = if lag == 0 { n - 1 } else { n - lag };
    if m < 8 {
        return None;
    }
    let mut te = 0.0;
    for t in 0..m {
        let xt = x[t] as f64;
        let xt1 = x[if lag == 0 { t + 1 } else { t + lag }] as f64;
        let yt = y[t] as f64;
        let zt = z[t] as f64;
        let mut k3 = 0.0;
        for s in 0..m {
            let xf = x[if lag == 0 { s + 1 } else { s + lag }] as f64;
            k3 += gaussian(xt1 - xf, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - z[s] as f64, hz);
        }
        let p3 = k3 / m as f64;
        let mut k1 = 0.0;
        for s in 0..n {
            k1 += gaussian(xt - x[s] as f64, hx) * gaussian(zt - z[s] as f64, hz);
        }
        let p1 = k1 / n as f64;
        let mut k2xy = 0.0;
        for s in 0..n {
            k2xy += gaussian(xt - x[s] as f64, hx)
                * gaussian(yt - y[s] as f64, hy)
                * gaussian(zt - z[s] as f64, hz);
        }
        let p2xy = k2xy / n as f64;
        let mut k2x = 0.0;
        for s in 0..m {
            let xf = x[if lag == 0 { s + 1 } else { s + lag }] as f64;
            k2x += gaussian(xt1 - xf, hx)
                * gaussian(xt - x[s] as f64, hx)
                * gaussian(zt - z[s] as f64, hz);
        }
        let p2x = k2x / m as f64;
        te += ((p3 * p1) / (p2xy * p2x).max(1e-300)).ln();
    }
    Some(te / m as f64)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(xr_path) = arg_value(&args, "--xr") else {
        eprintln!("--xr <gxrs-l2-irrad.nc> fehlt");
        return;
    };
    let Some(euv_path) = arg_value(&args, "--euv") else {
        eprintln!("--euv <geuv-l2-ir10s.nc> fehlt");
        return;
    };

    let xra = series(&xr_path, "time", "a_flux", "a_flags", 2, None, 0.0);
    let xrb = series(&xr_path, "time", "b_flux", "b_flags", 2, None, 0.0);
    let euv = series(
        &euv_path,
        "time_chanE",
        "irr_1216_1nm",
        "irr_1216_flag",
        1,
        Some("geocorona_flag"),
        EPOCH_2000,
    );
    println!(
        "XRSA: {} | XRSB: {} | Lyman-α: {} gültige Samples",
        xra.len(),
        xrb.len(),
        euv.len()
    );

    let t0 = xra
        .first()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .max(xrb.first().map(|&(t, _)| t).unwrap_or(0.0))
        .max(euv.first().map(|&(t, _)| t).unwrap_or(0.0));
    let t1 = xra
        .last()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .min(xrb.last().map(|&(t, _)| t).unwrap_or(0.0))
        .min(euv.last().map(|&(t, _)| t).unwrap_or(0.0));
    let bins = ((t1 - t0) / DT).floor() as usize;
    let ab = bin_median(&xra, t0, bins);
    let bb = bin_median(&xrb, t0, bins);
    let eb = bin_median(&euv, t0, bins);
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut e = Vec::new();
    for i in 0..bins {
        if let (Some(av), Some(bv), Some(ev)) = (ab[i], bb[i], eb[i]) {
            a.push(av);
            b.push(bv);
            e.push(ev);
        }
    }
    println!("gemeinsame 10-s-Zellen (drei Kräfte): {}", a.len());

    let lags = [0usize, 5, 10, 15, 20, 25, 30];
    println!();
    println!("Paarweise vs. bedingte TE — ist der Lyman-α→Röntgen-Vorlauf echt oder der gemeinsame Flare-Treiber?");
    println!(
        "{:>4} | {:>10} | {:>10} | {:>10} | {:>10}",
        "lag", "Lya→A", "Lya→A|B", "A→Lya", "A→Lya|B"
    );
    for &lag in &lags {
        let pf = transfer_entropy_lag(&a, &e, lag).unwrap_or(f64::NAN);
        let cf = te_conditional(&a, &e, &b, lag).unwrap_or(f64::NAN);
        let pr = transfer_entropy_lag(&e, &a, lag).unwrap_or(f64::NAN);
        let cr = te_conditional(&e, &a, &b, lag).unwrap_or(f64::NAN);
        println!(
            "{:>4} | {:>10.4e} | {:>10.4e} | {:>10.4e} | {:>10.4e}",
            lag, pf, cf, pr, cr
        );
    }
    println!();
    println!("Dasselbe mit B (kühles Röntgen) als Ziel, konditioniert auf A:");
    println!(
        "{:>4} | {:>10} | {:>10} | {:>10} | {:>10}",
        "lag", "Lya→B", "Lya→B|A", "B→Lya", "B→Lya|A"
    );
    for &lag in &lags {
        let pf = transfer_entropy_lag(&b, &e, lag).unwrap_or(f64::NAN);
        let cf = te_conditional(&b, &e, &a, lag).unwrap_or(f64::NAN);
        let pr = transfer_entropy_lag(&e, &b, lag).unwrap_or(f64::NAN);
        let cr = te_conditional(&e, &b, &a, lag).unwrap_or(f64::NAN);
        println!(
            "{:>4} | {:>10.4e} | {:>10.4e} | {:>10.4e} | {:>10.4e}",
            lag, pf, cf, pr, cr
        );
    }
    println!();
    println!("Lesart: verschwindet TE(Lya→X), wenn auf das andere Röntgen-Band konditioniert wird, war der Vorlauf der gemeinsame Treiber (Nanoflare-Kopplung). Bleibt er, trägt die Chromosphäre eigene Information über die Korona-Zukunft (Alfvén-vereinbar).");
}
