use std::collections::HashMap;

use omegaflow::archivar::bsp_reader::spk::SpkFile;
use omegaflow::archivar::{body_barycenter_position, parse_ephemeris_binary, BodyEphemeris};

const JD_J2000: f64 = 2451545.0;

fn load(path: &str, name: &str) -> Option<HashMap<String, BodyEphemeris>> {
    let bytes = std::fs::read(path).ok()?;
    let eph = parse_ephemeris_binary(&bytes)?;
    let mut map = HashMap::new();
    map.insert(name.to_string(), eph);
    Some(map)
}

fn arg_token(args: &[String], key: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == key)?;
    args.get(pos + 1).cloned()
}

fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn vec_len(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn decade(jd: f64) -> i64 {
    (1900.0 + (jd - 2415020.0) / 365.25).floor() as i64 / 10 * 10
}

struct Inject {
    d: [f64; 3],
    jd0: f64,
    jd1: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Neptune center rift — the per-point version difference between the three ephemeris houses at the planet center.");

    let eph_dir = arg_token(&args, "--eph-dir").unwrap_or("data".to_string());
    let spk_path =
        arg_token(&args, "--spk").unwrap_or("data/naif.jpl.nasa.gov/nep097xl-899.bsp".to_string());

    let inject: Option<Inject> = {
        let pos = args.iter().position(|a| a == "--calibrate-inject");
        let mut d = [0.0; 3];
        let mut jd0 = 0.0;
        let mut jd1 = 0.0;
        match pos {
            Some(p) => {
                let mut ok = true;
                for i in 0..3 {
                    match args.get(p + 1 + i).and_then(|s| s.parse::<f64>().ok()) {
                        Some(v) if v.is_finite() => d[i] = v,
                        _ => ok = false,
                    }
                }
                match (
                    args.get(p + 4).and_then(|s| s.parse::<f64>().ok()),
                    args.get(p + 5).and_then(|s| s.parse::<f64>().ok()),
                ) {
                    (Some(a), Some(b)) if a.is_finite() && b.is_finite() => {
                        jd0 = a;
                        jd1 = b;
                    }
                    _ => ok = false,
                }
                if ok {
                    Some(Inject { d, jd0, jd1 })
                } else {
                    println!("neptune-center-rift: --calibrate-inject wants dx_km dy_km dz_km jd0 jd1 (finite) — injection stays off");
                    None
                }
            }
            None => None,
        }
    };

    let cm = load(
        &format!("{eph_dir}/ssd.jpl.nasa.gov/ephemeris_neptune_c.bin"),
        "neptune_c",
    );
    let im = load(
        &format!("{eph_dir}/ftp.imcce.fr/ephemeris_inpop_neptune.bin"),
        "neptune",
    );
    let em = load(
        &format!("{eph_dir}/ftp.iaaras.ru/ephemeris_epm_neptune.bin"),
        "neptune",
    );
    let (Some(cm), Some(im), Some(em)) = (cm, im, em) else {
        eprintln!("neptune-center-rift: one of ephemeris_neptune_c.bin / ephemeris_inpop_neptune.bin / ephemeris_epm_neptune.bin reads void");
        return;
    };
    let spk = match SpkFile::open(&spk_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("neptune-center-rift: {spk_path} opens void — {e:?}");
            return;
        }
    };

    let start_jd = 2440587.5;
    let stop_jd = 2462502.5;
    let step = 30.0;

    let names = ["de441", "inpop19a", "epm2021"];
    let mut pair_vec: [[Vec<[f64; 3]>; 3]; 3] =
        std::array::from_fn(|_| [Vec::new(), Vec::new(), Vec::new()]);
    let mut jds: Vec<f64> = Vec::new();
    let mut injections: Vec<([f64; 3], f64)> = Vec::new();

    let mut jd = start_jd;
    while jd <= stop_jd {
        let tdb = (jd - JD_J2000) * 86400.0;
        let mut de = match body_barycenter_position("neptune_c", tdb, &cm) {
            Some(c) => c,
            None => {
                jd += step;
                continue;
            }
        };
        if let Some(ij) = &inject {
            if jd >= ij.jd0 && jd <= ij.jd1 {
                let plain = de;
                de = vec_add(de, [ij.d[0] * 1000.0, ij.d[1] * 1000.0, ij.d[2] * 1000.0]);
                injections.push((vec_sub(de, plain), jd));
            }
        }
        let (Some(in_b), Some(ep_b)) = (
            body_barycenter_position("neptune", tdb, &im),
            body_barycenter_position("neptune", tdb, &em),
        ) else {
            jd += step;
            continue;
        };
        let Ok(rel) = spk.state(899, 8, tdb) else {
            jd += step;
            continue;
        };
        let off = [rel[0] * 1000.0, rel[1] * 1000.0, rel[2] * 1000.0];
        let in_c = vec_add(in_b, off);
        let ep_c = vec_add(ep_b, off);
        let centers = [de, in_c, ep_c];
        for a in 0..3 {
            for b in (a + 1)..3 {
                pair_vec[a][b].push(vec_sub(centers[a], centers[b]));
            }
        }
        jds.push(jd);
        jd += step;
    }

    let n = jds.len();
    println!(
        "neptune-center-rift: {} epochs over JD {:.2}..{:.2} (step {step} d)",
        n, start_jd, stop_jd
    );

    let mut report = String::new();
    report.push_str(
        "Neptune center rift — per-point version difference (line a − line b, ICRS, km)\n\n",
    );

    for a in 0..3 {
        for b in (a + 1)..3 {
            let v = &pair_vec[a][b];
            if v.is_empty() {
                report.push_str(&format!("  {} − {} : absent\n", names[a], names[b]));
                continue;
            }
            let mut sx = 0.0;
            let mut sy = 0.0;
            let mut sz = 0.0;
            let mut sd = 0.0;
            let mut maxd = 0.0;
            for d in v {
                sx += d[0];
                sy += d[1];
                sz += d[2];
                let m = vec_len(*d);
                sd += m;
                if m > maxd {
                    maxd = m;
                }
            }
            let k = v.len() as f64;
            report.push_str(&format!(
                "  {} − {} : Δx {:+.0} km, Δy {:+.0} km, Δz {:+.0} km, |Δ| mean {:.0} km, max {:.0} km ({} epochs)\n",
                names[a], names[b],
                sx / k / 1000.0,
                sy / k / 1000.0,
                sz / k / 1000.0,
                sd / k / 1000.0,
                maxd / 1000.0,
                v.len()
            ));
        }
    }

    report.push_str("\nDecade structure (per-bin mean |Δ|, km):\n");
    let mut decs: Vec<i64> = Vec::new();
    let mut dec_idx: HashMap<i64, Vec<usize>> = HashMap::new();
    for (i, &jdv) in jds.iter().enumerate() {
        let d = decade(jdv);
        dec_idx.entry(d).or_default().push(i);
        if !decs.contains(&d) {
            decs.push(d);
        }
    }
    decs.sort();
    for d in decs {
        let idxs = &dec_idx[&d];
        let mut line = format!("  {}s: {} epochs |", d, idxs.len());
        for a in 0..3 {
            for b in (a + 1)..3 {
                let mut s = 0.0;
                for &i in idxs {
                    s += vec_len(pair_vec[a][b][i]);
                }
                line.push_str(&format!(
                    " {}−{} {:6.0} |",
                    names[a],
                    names[b],
                    s / idxs.len() as f64 / 1000.0
                ));
            }
        }
        report.push_str(&format!("{}\n", line));
    }

    if let Some(ij) = &inject {
        report.push_str(&format!(
            "\nCalibration injection: {:.0}/{:.0}/{:.0} km into de441 over JD {:.2}..{:.2}:\n",
            ij.d[0], ij.d[1], ij.d[2], ij.jd0, ij.jd1
        ));
        if injections.is_empty() {
            report.push_str("  recovered nothing — the window carried no epochs\n");
        } else {
            let mut rec = [0.0; 3];
            for (r, _) in &injections {
                rec[0] += r[0];
                rec[1] += r[1];
                rec[2] += r[2];
            }
            let k = injections.len() as f64;
            let mag = (ij.d[0] * ij.d[0] + ij.d[1] * ij.d[1] + ij.d[2] * ij.d[2]).sqrt();
            report.push_str(&format!(
                "  recovered injection vector Δ {:+.3}/{:+.3}/{:+.3} km — injected {:+.0}/{:+.0}/{:+.0} km (magnitude {:.0} km, {} epochs)\n",
                rec[0] / k / 1000.0,
                rec[1] / k / 1000.0,
                rec[2] / k / 1000.0,
                ij.d[0], ij.d[1], ij.d[2], mag, injections.len()
            ));
        }
    }

    print!("{report}");
    if let Err(e) = std::fs::create_dir_all("state/reports") {
        eprintln!("neptune-center-rift: report dir stays unbuilt — {e}");
    }
    let report_path = "state/reports/neptune_center_rift.txt";
    if let Err(e) = std::fs::write(report_path, &report) {
        eprintln!("neptune-center-rift: report write void — {e}");
    }
    let series_path = "state/reports/neptune_center_rift_series.tsv";
    let mut series = String::new();
    series.push_str("jd\tdecade\tdx_de441_inpop_km\tdy_de441_inpop_km\tdz_de441_inpop_km\tdx_de441_epm_km\tdy_de441_epm_km\tdz_de441_epm_km\tdx_inpop_epm_km\tdy_inpop_epm_km\tdz_inpop_epm_km\n");
    for i in 0..n {
        series.push_str(&format!(
            "{:.8}\t{}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\n",
            jds[i],
            decade(jds[i]),
            pair_vec[0][1][i][0] / 1000.0,
            pair_vec[0][1][i][1] / 1000.0,
            pair_vec[0][1][i][2] / 1000.0,
            pair_vec[0][2][i][0] / 1000.0,
            pair_vec[0][2][i][1] / 1000.0,
            pair_vec[0][2][i][2] / 1000.0,
            pair_vec[1][2][i][0] / 1000.0,
            pair_vec[1][2][i][1] / 1000.0,
            pair_vec[1][2][i][2] / 1000.0,
        ));
    }
    if let Err(e) = std::fs::write(series_path, &series) {
        eprintln!("neptune-center-rift: series write void — {e}");
    }
    println!("series: {series_path}");
    println!("report: {report_path}");
}
