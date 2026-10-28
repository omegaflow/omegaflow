use omegaflow::cdf::{CdfFile, cdf_epoch_unix, tt2000_unix};
use omegaflow::ephemeris::{CHEBYSHEV_DEGREE, chebyshev_fit, write_binary};
use omegaflow::pck::PckBody;
use std::fs;

const R_EARTH_KM: f64 = 6371.0;
const GRANULE_SECONDS: f64 = 32.0 * 86400.0;
const DE2_NAIF: i32 = -17;
const ALTITUDE_TOLERANCE_KM: f64 = 50.0;

fn var_values(file: &CdfFile, bytes: &[u8], name: &str) -> Option<Vec<Vec<f64>>> {
    let var = file.var(name)?;
    let recs = file.var_records(bytes, var).ok()?;
    Some(recs.into_iter().map(|(_, v)| v).collect())
}

fn epoch_seconds(file: &CdfFile, bytes: &[u8], name: &str) -> Option<Vec<f64>> {
    let var = file.var(name)?;
    let recs = file.var_records(bytes, var).ok()?;
    let mut out = Vec::with_capacity(recs.len());
    for (_, v) in recs {
        let first = *v.first()?;
        let t = match var.data_type {
            31 => cdf_epoch_unix(first),
            32 => {
                let ps = *v.get(1).unwrap_or(&0.0);
                omegaflow::cdf::cdf_epoch16_unix(first, ps)
            }
            33 => tt2000_unix(first as i64),
            other => {
                eprintln!("Epoch-Typ {other} nicht unterstuetzt");
                return None;
            }
        };
        out.push(t);
    }
    Some(out)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: de2_orbit_compiler <cdf> <out.bin> [--dry]");
        std::process::exit(2);
    }
    let cdf_path = &args[1];
    let out_path = &args[2];
    let dry = args.iter().any(|a| a == "--dry");

    let bytes = match fs::read(cdf_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{cdf_path}: {e}");
            std::process::exit(2);
        }
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{cdf_path}: CDF nicht lesbar: {note:?}");
            std::process::exit(2);
        }
    };

    let t = match epoch_seconds(&file, &bytes, "Epoch") {
        Some(v) if !v.is_empty() => v,
        _ => {
            eprintln!("{cdf_path}: keine Epoch-Variable");
            std::process::exit(2);
        }
    };
    let (px, py, pz) = match (
        var_values(&file, &bytes, "X_SC_Position_Vec"),
        var_values(&file, &bytes, "Y_SC_Position_Vec"),
        var_values(&file, &bytes, "Z_SC_Position_Vec"),
    ) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => {
            eprintln!("{cdf_path}: Positionsvektoren fehlen");
            std::process::exit(2);
        }
    };

    let n = t.len().min(px.len()).min(py.len()).min(pz.len());
    if n < CHEBYSHEV_DEGREE + 1 {
        eprintln!("{cdf_path}: nur {n} Punkte, zu wenig fuer Grad {CHEBYSHEV_DEGREE}");
        std::process::exit(2);
    }

    if let Some(alt) = var_values(&file, &bytes, "Altitude") {
        let mut worst = 0.0f64;
        let mut checked = 0usize;
        for i in 0..n.min(alt.len()) {
            let (Some(x), Some(y), Some(z), Some(a)) = (
                px[i].first().copied(),
                py[i].first().copied(),
                pz[i].first().copied(),
                alt[i].first().copied(),
            ) else {
                continue;
            };
            let r = (x * x + y * y + z * z).sqrt();
            let d = (r - R_EARTH_KM - a).abs();
            if d > worst {
                worst = d;
            }
            checked += 1;
        }
        println!("hoehenprobe: {checked} Punkte, max Abweichung {worst:.3} km");
        if worst > ALTITUDE_TOLERANCE_KM {
            eprintln!("ABBRUCH: |r| - Re trifft Altitude nicht (max {worst:.3} km)");
            std::process::exit(3);
        }
    } else {
        println!("hoehenprobe: keine Altitude-Variable, nicht geprueft");
    }

    let mut granules = Vec::new();
    let mut start = 0usize;
    let t_end = t[n - 1];
    while start < n {
        let t0 = t[start];
        let stop = (t0 + GRANULE_SECONDS).min(t_end);
        let mut samples: Vec<(f64, f64, f64)> = Vec::new();
        let mut zs: Vec<(f64, f64, f64)> = Vec::new();
        let mut i = start;
        while i < n && t[i] <= stop {
            if let (Some(x), Some(y), Some(z)) = (
                px[i].first().copied(),
                py[i].first().copied(),
                pz[i].first().copied(),
            ) {
                if x.is_finite() && y.is_finite() && z.is_finite() {
                    samples.push((t[i], x, y));
                    zs.push((t[i], z, 0.0));
                }
            }
            i += 1;
        }
        let fit = chebyshev_fit(&samples, CHEBYSHEV_DEGREE);
        let zfit = chebyshev_fit(&zs, CHEBYSHEV_DEGREE);
        match (fit, zfit) {
            (Some((cx, cy, _)), Some((cz, _, _))) => {
                granules.push((t0, stop - t0, cx, cy, cz));
            }
            _ => {
                eprintln!("granule {start}: Fit fehlgeschlagen");
                std::process::exit(2);
            }
        }
        start = i;
    }

    let pck = PckBody {
        naif_id: DE2_NAIF,
        gm_m3_s2: None,
        pole_ra_deg: None,
        pole_ra_rate_deg_per_century: None,
        pole_dec_deg: None,
        pole_dec_rate_deg_per_century: None,
        pm_deg: None,
        pm_rate_deg_per_day: None,
        radii_m: None,
        j2: None,
        j4: None,
        nut_ra: None,
    };

    println!(
        "{}: {n} Punkte, {} Granule, {:.1} Tage, NAIF {DE2_NAIF}",
        cdf_path,
        granules.len(),
        (t_end - t[0]) / 86400.0
    );
    if dry {
        println!("dry: nichts geschrieben");
        return;
    }
    if !write_binary(out_path, "de2", &granules, &[], &[], &pck, None) {
        eprintln!("{out_path}: write_binary meldet Fehler");
        std::process::exit(2);
    }
    match fs::metadata(out_path) {
        Ok(m) => println!("{out_path}: {} B geschrieben", m.len()),
        Err(e) => eprintln!("{out_path}: {e}"),
    }
}
