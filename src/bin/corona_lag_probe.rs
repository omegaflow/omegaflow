// Der sub-minütige Korona-Lag-Probe — die Messung, die das Rätsel angreift.
// Liest zwei NCEI-Dateien eines Flare-Tages: gxrs-l2-irrad (2-s-Röntgen,
// a_flux = kurzes Band 0,5–4 Å, Unix-Epoche) und geuv-l2-ir10s (10-s-EUV,
// irr_1216_1nm = Lyman-α, Epoche seit 2000-01-01T12:00 UTC → +946728000).
// Beide auf ein 10-s-Gitter (Median), dann TE in beide Richtungen über einen
// Lag-Sweep. Alfvén-Vorhersage: die Chromosphäre (Lyman-α) führt die Korona
// (Röntgen) um die Laufzeit (~100 s ≈ 10 Zellen); Nanoflares tragen keinen
// konsistenten Lag. src/te.rs bleibt unberührt (öffentliche API).

use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
use omegaflow::te::transfer_entropy_lag;

const EPOCH_2000: f64 = 946728000.0;
const DT: f64 = 10.0;
const LAG_MAX: usize = 120;
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

fn pair(a: &[Option<f32>], b: &[Option<f32>]) -> (Vec<f32>, Vec<f32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (ca, cb) in a.iter().zip(b.iter()) {
        if let (Some(x), Some(y)) = (ca, cb) {
            xs.push(*x);
            ys.push(*y);
        }
    }
    (xs, ys)
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

    let xr = series(&xr_path, "time", "a_flux", "a_flags", 2, None, 0.0);
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
        "X-ray 2s: {} gültige Samples | EUV 10s: {} gültige Samples",
        xr.len(),
        euv.len()
    );

    let t0 = xr
        .first()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .max(euv.first().map(|&(t, _)| t).unwrap_or(0.0));
    let t1 = xr
        .last()
        .map(|&(t, _)| t)
        .unwrap_or(0.0)
        .min(euv.last().map(|&(t, _)| t).unwrap_or(0.0));
    let bins = ((t1 - t0) / DT).floor() as usize;
    let xb = bin_median(&xr, t0, bins);
    let eb = bin_median(&euv, t0, bins);
    let (x, e) = pair(&xb, &eb);
    println!("gemeinsame 10-s-Zellen: {}", x.len());

    println!();
    println!(
        "TE-Lag-Sweep (Lyman-α ↔ Röntgen, 10-s-Zellen, lag 0..{} = 0..{} min):",
        LAG_MAX,
        LAG_MAX * 10 / 60
    );
    println!("{:>4} | {:>12} | {:>12}", "lag", "TE(Lya→X)", "TE(X→Lya)");
    let mut best: Option<(usize, f64)> = None;
    for lag in 0..=LAG_MAX {
        let te_l2x = transfer_entropy_lag(&x, &e, lag).unwrap_or(f64::NAN);
        let te_x2l = transfer_entropy_lag(&e, &x, lag).unwrap_or(f64::NAN);
        if te_l2x > best.map_or(f64::NEG_INFINITY, |(_, b)| b) {
            best = Some((lag, te_l2x));
        }
        println!(
            "{:>4} | {:>12.4e} | {:>12.4e}{}",
            lag,
            te_l2x,
            te_x2l,
            if lag == best.map(|(l, _)| l).unwrap_or(usize::MAX) {
                "   <--"
            } else {
                ""
            }
        );
    }
    println!();
    println!(
        "Spitze TE(Lya→X) bei lag {} Zellen = {} s{} — {}",
        best.map(|(l, _)| l).unwrap_or(0),
        best.map(|(l, _)| l * 10).unwrap_or(0),
        best.map(|(l, _)| l)
            .map(|l| if l == 0 { "" } else { "" })
            .unwrap_or(""),
        if let Some((_, v)) = best {
            if v.is_finite() {
                format!("TE {:.4e}", v)
            } else {
                "kein TE".into()
            }
        } else {
            "kein TE".into()
        }
    );
    println!("Alfvén-Erwartung: Spitze bei ~100 s ≈ lag 10, TE(Lya→X) > TE(X→Lya) an der Spitze.");
}
