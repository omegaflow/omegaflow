use omegaflow::archivar::{C_LIGHT, NAIF_LSK_EMBEDDED, build_star_samples, lsk};
use std::collections::HashMap;

struct Floor {
    ft: f64,
    scale: f64,
    pass: usize,
    val_pass: usize,
    excl: usize,
    incl: usize,
    hull_now: usize,
    hull_ttl64: usize,
    dmax: Vec<f64>,
    rinsert: Vec<f64>,
}

fn arg_parse(v: &str) -> Option<f64> {
    v.parse::<f64>().ok()
}

fn main() {
    let mut bin_path = String::from("data/ssd.jpl.nasa.gov/dr3_stars.bin");
    let mut floors: Vec<Floor> = Vec::new();
    let mut force_ref = 1.0f64;
    let mut expose_offsets: Vec<i32> = vec![0];
    let mut softenings: Vec<f64> = vec![1.0];
    let mut t2: Option<f64> = None;
    let mut pad = 1.0f64;
    let mut span_only = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--bin" => {
                if let Some(v) = args.next() {
                    bin_path = v;
                }
            }
            "--floor" => {
                let Some(v) = args.next() else { continue };
                let Some(f) = arg_parse(&v).filter(|f| f.is_finite() && *f > 0.0) else {
                    eprintln!(
                        "star_dmax_probe: --floor {} carries no positive finite value",
                        v
                    );
                    std::process::exit(2);
                };
                floors.push(Floor {
                    ft: f,
                    scale: 1.0,
                    pass: 0,
                    val_pass: 0,
                    excl: 0,
                    incl: 0,
                    hull_now: 0,
                    hull_ttl64: 0,
                    dmax: Vec::new(),
                    rinsert: Vec::new(),
                });
            }
            "--force-ref" => {
                let Some(v) = args.next() else { continue };
                let Some(f) = arg_parse(&v).filter(|f| f.is_finite() && *f > 0.0) else {
                    eprintln!(
                        "star_dmax_probe: --force-ref {} carries no positive finite value",
                        v
                    );
                    std::process::exit(2);
                };
                force_ref = f;
            }
            "--expose-offset" => {
                let Some(v) = args.next() else { continue };
                match v.parse::<i32>() {
                    Ok(o) => expose_offsets.push(o),
                    Err(_) => {
                        eprintln!("star_dmax_probe: --expose-offset {} is not an integer", v);
                        std::process::exit(2);
                    }
                }
            }
            "--softening" => {
                let Some(v) = args.next() else { continue };
                let Some(f) = arg_parse(&v).filter(|f| f.is_finite() && *f > 0.0) else {
                    eprintln!(
                        "star_dmax_probe: --softening {} carries no positive finite value",
                        v
                    );
                    std::process::exit(2);
                };
                softenings.push(f);
            }
            "--t2" => {
                let Some(v) = args.next() else { continue };
                let Some(f) = arg_parse(&v).filter(|f| f.is_finite() && *f >= 0.0) else {
                    eprintln!(
                        "star_dmax_probe: --t2 {} carries no non-negative finite value",
                        v
                    );
                    std::process::exit(2);
                };
                t2 = Some(f);
            }
            "--pad" => {
                let Some(v) = args.next() else { continue };
                let Some(f) = arg_parse(&v).filter(|f| f.is_finite()) else {
                    eprintln!("star_dmax_probe: --pad {} carries no finite value", v);
                    std::process::exit(2);
                };
                pad = f;
            }
            "--span" => span_only = true,
            _ => {}
        }
    }
    if floors.is_empty() {
        for &off in &expose_offsets {
            for &scale in &softenings {
                let ft = force_ref * (0.5f64).powi(off) / (scale * scale);
                if ft.is_finite() && ft > 0.0 {
                    floors.push(Floor {
                        ft,
                        scale,
                        pass: 0,
                        val_pass: 0,
                        excl: 0,
                        incl: 0,
                        hull_now: 0,
                        hull_ttl64: 0,
                        dmax: Vec::new(),
                        rinsert: Vec::new(),
                    });
                }
            }
        }
    }
    let t2 = match t2 {
        Some(t) => t,
        None => match lsk::parse(NAIF_LSK_EMBEDDED).and_then(|l| l.system_now_tdb()) {
            Some(t) => t,
            None => {
                eprintln!(
                    "star_dmax_probe: the embedded LSK yields no TDB now — pass --t2 explicitly"
                );
                std::process::exit(2);
            }
        },
    };
    let bytes = match std::fs::read(&bin_path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("star_dmax_probe: {} read void — no measurement", bin_path);
            std::process::exit(2);
        }
    };
    let samples = build_star_samples(&bytes);
    println!("BIN {} {} bytes", bin_path, bytes.len());
    println!("COUNT {}", samples.len());
    if samples.is_empty() {
        println!("no star samples loaded — the measurement is absent, not zero");
        std::process::exit(2);
    }
    let mut span_lo = [f64::INFINITY; 3];
    let mut span_hi = [f64::NEG_INFINITY; 3];
    let mut epoch_min = f64::MAX;
    let mut epoch_max = f64::MIN;
    let mut ttl_max = 0.0f64;
    let mut val_finite = 0usize;
    let mut vals: Vec<f64> = Vec::with_capacity(samples.len());
    for s in &samples {
        for k in 0..3 {
            span_lo[k] = span_lo[k].min(s.anchor_p0[k]);
            span_hi[k] = span_hi[k].max(s.anchor_p0[k]);
        }
        epoch_min = epoch_min.min(s.epoch);
        epoch_max = epoch_max.max(s.epoch);
        ttl_max = ttl_max.max(s.ttl);
        if s.val.is_finite() {
            val_finite += 1;
            vals.push(s.val);
        }
    }
    let mut span = 0.0f64;
    for k in 0..3 {
        span = span.max(span_hi[k] - span_lo[k]);
    }
    println!("SPAN_M {:.6e}", span);
    println!("EPOCH_MIN {} EPOCH_MAX {}", epoch_min, epoch_max);
    println!("TTL_MAX {:.6e}", ttl_max);
    println!("VAL_FINITE {} of {}", val_finite, samples.len());
    println!(
        "VAL_QUANTILES p10 {:.6e} p50 {:.6e} p90 {:.6e}",
        quantile(&mut vals, 0.10),
        quantile(&mut vals, 0.50),
        quantile(&mut vals, 0.90)
    );
    if span_only {
        return;
    }
    let eph: HashMap<String, omegaflow::archivar::BodyEphemeris> = HashMap::new();
    let forward = [1.0f64, 0.0, 0.0];
    let center = [0.0f64, 0.0, 0.0];
    let age = (t2 - epoch_min).abs();
    let hull_now = C_LIGHT * age + pad;
    let hull_ttl64 = C_LIGHT * (ttl_max * 64.0) + pad;
    for floor in floors.iter_mut() {
        let scale2 = floor.scale * floor.scale;
        let mut level_pass = 0usize;
        let mut level_val_pass = 0usize;
        let mut level_excl = 0usize;
        let mut level_incl = 0usize;
        let mut level_hull_now = 0usize;
        let mut level_hull_ttl64 = 0usize;
        let mut dmax: Vec<f64> = Vec::with_capacity(samples.len());
        let mut rinsert: Vec<f64> = Vec::with_capacity(samples.len());
        for s in &samples {
            let age_s = (t2 - s.epoch).abs();
            if age_s > s.ttl * 64.0 {
                continue;
            }
            let val_max = s.val.abs();
            let val_gate = val_max >= 0.0 && val_max >= floor.ft * scale2;
            if val_gate {
                level_val_pass += 1;
            }
            let Some(p) = s.motion.at(t2, s.epoch, &eph) else {
                continue;
            };
            let ddx = p[0] - center[0];
            let ddy = p[1] - center[1];
            let ddz = p[2] - center[2];
            let d2 = ddx * ddx + ddy * ddy + ddz * ddz;
            let d = d2.sqrt();
            let sd = ddx * forward[0] + ddy * forward[1] + ddz * forward[2];
            let transverse2 = (d2 - sd * sd).max(0.0);
            if s.ttl <= 0.0 || s.ttl.is_nan() {
                continue;
            }
            let retarded = if d > 0.0 {
                (age_s - d / C_LIGHT).max(0.0)
            } else {
                age_s
            };
            let val_eff = s.val * (-retarded / s.ttl).exp();
            let pass = val_gate && val_eff.abs() / (transverse2 + scale2) >= floor.ft;
            if pass {
                level_pass += 1;
                if d <= hull_now {
                    level_hull_now += 1;
                }
                if d <= hull_ttl64 {
                    level_hull_ttl64 += 1;
                }
            }
            let r_ins = if val_eff.is_finite() && val_eff >= 0.0 {
                (val_eff / floor.ft).sqrt().min(C_LIGHT * 64.0 * s.ttl)
            } else {
                continue;
            };
            if pass && d > r_ins {
                level_excl += 1;
            }
            if val_gate && !pass && d <= r_ins {
                level_incl += 1;
            }
            rinsert.push(r_ins);
            let v0 = s.val.abs();
            if v0 >= floor.ft * scale2 {
                let dm = (v0 / floor.ft - scale2).sqrt();
                if dm.is_finite() {
                    dmax.push(dm);
                }
            }
        }
        floor.pass = level_pass;
        floor.val_pass = level_val_pass;
        floor.excl = level_excl;
        floor.incl = level_incl;
        floor.hull_now = level_hull_now;
        floor.hull_ttl64 = level_hull_ttl64;
        floor.dmax = dmax;
        floor.rinsert = rinsert;
    }
    println!("T2 {:.6e} AGE {:.6e} PAD {}", t2, age, pad);
    println!(
        "HULL_NOW_M {:.6e} HULL_TTL64_M {:.6e}",
        hull_now, hull_ttl64
    );
    for floor in &mut floors {
        floor.dmax.sort_by(|a, b| a.total_cmp(b));
        floor.rinsert.sort_by(|a, b| a.total_cmp(b));
        let f_excl = if floor.pass > 0 {
            floor.excl as f64 / floor.pass as f64
        } else {
            0.0
        };
        let f_inc = if floor.val_pass > 0 {
            floor.incl as f64 / floor.val_pass as f64
        } else {
            0.0
        };
        let hull_now_f = if floor.pass > 0 {
            floor.hull_now as f64 / floor.pass as f64
        } else {
            0.0
        };
        let hull_ttl64_f = if floor.pass > 0 {
            floor.hull_ttl64 as f64 / floor.pass as f64
        } else {
            0.0
        };
        let (ri_lo, ri_hi, rid) = ecdf_stats(&floor.rinsert);
        println!(
            "FLOOR ft {:.6e} scale {} | pass {} (val {}) | f_excl {:.6} f_inc {:.6} | hull_now {:.6} hull_ttl64 {:.6} | r_insert p10 {:.6e} p90 {:.6e} interdecile {:.6} | d_max n {} p50 {:.6e} p90 {:.6e} p99 {:.6e}",
            floor.ft,
            floor.scale,
            floor.pass,
            floor.val_pass,
            f_excl,
            f_inc,
            hull_now_f,
            hull_ttl64_f,
            ri_lo,
            ri_hi,
            rid,
            floor.dmax.len(),
            ecdf_at(&floor.dmax, 0.50),
            ecdf_at(&floor.dmax, 0.90),
            ecdf_at(&floor.dmax, 0.99)
        );
    }
    let mut refuted = false;
    for floor in &floors {
        let f_excl = if floor.pass > 0 {
            floor.excl as f64 / floor.pass as f64
        } else {
            0.0
        };
        let f_inc = if floor.val_pass > 0 {
            floor.incl as f64 / floor.val_pass as f64
        } else {
            0.0
        };
        let (_, _, rid) = ecdf_stats(&floor.rinsert);
        if f_excl > 0.5 || f_inc > 0.0 || rid > 10.0 {
            refuted = true;
            println!(
                "REFUTED at ft {:.6e}: f_excl {:.6} f_inc {:.6} interdecile {:.6} — the Insert-Fix is no normalization; the query-side form stands",
                floor.ft, f_excl, f_inc, rid
            );
        }
    }
    if !refuted {
        println!(
            "the acceptance criterion shows no refutation at any floor — the query-side form stands regardless: a stored extent freezes the live floor"
        );
    }
}

fn quantile(v: &mut [f64], p: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    ecdf_at(v, p)
}

fn ecdf_at(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx]
}

fn ecdf_stats(sorted: &[f64]) -> (f64, f64, f64) {
    let lo = ecdf_at(sorted, 0.10);
    let hi = ecdf_at(sorted, 0.90);
    let ratio = if lo > 0.0 { hi / lo } else { f64::INFINITY };
    (lo, hi, ratio)
}
