use omegaflow::archivar::J2000_EPOCH;
use omegaflow::archivar::motion::{BodyEphemeris, body_barycenter_position};
use omegaflow::cdf::{CdfFile, cdf_epoch_unix, tt2000_unix};
use omegaflow::ephemeris::{CHEBYSHEV_DEGREE, chebyshev_fit, write_binary};
use omegaflow::pck::PckBody;
use std::fs;

const R_EARTH_KM: f64 = 6371.0;
const GRANULE_DAYS: f64 = 32.0;
const DE2_NAIF: i32 = -17;
const ALTITUDE_TOLERANCE_KM: f64 = 50.0;
const UNIX_TO_JD: f64 = 2440587.5;

const CDF_MISSING_REAL4: f64 = -9.999999848243207e30;

fn is_missing(value: f64) -> bool {
    value == -1.0e31 || (value - CDF_MISSING_REAL4).abs() <= 1.0e20
}

fn jd_of_unix(unix_s: f64) -> f64 {
    unix_s / 86400.0 + UNIX_TO_JD
}

fn var_values(file: &CdfFile, bytes: &[u8], name: &str) -> Option<Vec<Vec<f64>>> {
    let var = match file.var(name) {
        Some(v) => v,
        None => {
            eprintln!("var {name}: absent");
            return None;
        }
    };
    let recs = match file.var_records(bytes, var) {
        Ok(r) => r,
        Err(note) => {
            eprintln!("var {name}: {note:?}");
            return None;
        }
    };
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

fn check(bin_path: &str, cdf_path: Option<&str>) {
    let bytes = match fs::read(bin_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{bin_path}: {e}");
            std::process::exit(2);
        }
    };
    let eph = match omegaflow::archivar::parse_ephemeris_binary(&bytes) {
        Some(e) => e,
        None => {
            eprintln!("{bin_path}: parse_ephemeris_binary liest die Datei nicht");
            std::process::exit(3);
        }
    };
    if eph.granules.is_empty() {
        eprintln!("{bin_path}: keine Granulen");
        std::process::exit(3);
    }
    if !eph.rotation_matrices.is_empty() {
        eprintln!(
            "{bin_path}: {} Rotationsmatrizen, erwartet 0",
            eph.rotation_matrices.len()
        );
        std::process::exit(3);
    }
    let mut bad = 0usize;
    for (i, g) in eph.granules.iter().enumerate() {
        let finite = g.t0_jd.is_finite()
            && g.dt_jd.is_finite()
            && g.dt_jd > 0.0
            && g.cx.iter().all(|c| c.is_finite())
            && g.cy.iter().all(|c| c.is_finite())
            && g.cz.iter().all(|c| c.is_finite());
        if !finite {
            eprintln!("granule {i}: nicht endlich oder dt <= 0");
            bad += 1;
        }
    }
    let lo = eph.granules[0].t0_jd;
    let hi = eph.granules[eph.granules.len() - 1].t0_jd + eph.granules[eph.granules.len() - 1].dt_jd;
    println!(
        "{bin_path}: {} Granulen, JD {lo:.3} bis {hi:.3}, Rotationen 0",
        eph.granules.len()
    );
    if let Some(cdf) = cdf_path {
        if let Ok(cbytes) = fs::read(cdf) {
            if let Ok(file) = CdfFile::parse(&cbytes) {
                if let Some(t) = epoch_seconds(&file, &cbytes, "Epoch") {
                    let first = jd_of_unix(t[0]);
                    let last = jd_of_unix(t[t.len() - 1]);
                    let covered = lo <= first + 1e-6 && hi + 1e-6 >= last;
                    println!(
                        "cdf: JD {first:.3} bis {last:.3} — {}",
                        if covered { "gedeckt" } else { "NICHT gedeckt" }
                    );
                    if !covered {
                        bad += 1;
                    }
                    if let (Some(px), Some(py), Some(pz)) = (
                        var_values(&file, &cbytes, "X_SC_Position_Vec"),
                        var_values(&file, &cbytes, "Y_SC_Position_Vec"),
                        var_values(&file, &cbytes, "Z_SC_Position_Vec"),
                    ) {
                        let mut map: std::collections::HashMap<String, BodyEphemeris> =
                            std::collections::HashMap::new();
                        map.insert("de2".to_string(), eph);
                        let mut worst = 0.0f64;
                        let mut compared = 0usize;
                        let step = if t.len() > 50 { t.len() / 50 } else { 1 };
                        let mut k = 0;
                        while k < t.len() {
                            let (Some(x), Some(y), Some(z)) = (
                                px[k].first().copied(),
                                py[k].first().copied(),
                                pz[k].first().copied(),
                            ) else {
                                k += step;
                                continue;
                            };
                            let tdb = (jd_of_unix(t[k]) - J2000_EPOCH) * 86400.0;
                            match body_barycenter_position("de2", tdb, &map) {
                                Some(p) => {
                                    let d = ((p[0] - x).powi(2)
                                        + (p[1] - y).powi(2)
                                        + (p[2] - z).powi(2))
                                    .sqrt();
                                    if d > worst {
                                        worst = d;
                                    }
                                    compared += 1;
                                }
                                None => {
                                    eprintln!("epoche {k}: keine Position aus dem Bin");
                                    bad += 1;
                                }
                            }
                            k += step;
                        }
                        println!(
                            "positionsvergleich: {compared} Epochen, max Abweichung {worst:.3} km"
                        );
                        if worst > 10.0 {
                            eprintln!("positionsvergleich: Abweichung zu gross");
                            bad += 1;
                        }
                    }
                }
            }
        } else {
            eprintln!("{cdf}: nicht lesbar");
            bad += 1;
        }
    }
    if bad > 0 {
        eprintln!("{bin_path}: {bad} Befund(e)");
        std::process::exit(3);
    }
    println!("{bin_path}: lesbar, Deckung stimmt");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "--check" {
        check(&args[2], args.get(3).map(String::as_str));
        return;
    }
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
            if is_missing(x) || is_missing(y) || is_missing(z) || is_missing(a) {
                continue;
            }
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
    let mut missing = 0usize;
    let mut skipped = 0usize;
    let mut start = 0usize;
    let jd_end = jd_of_unix(t[n - 1]);
    while start < n {
        let t0 = jd_of_unix(t[start]);
        let stop = (t0 + GRANULE_DAYS).min(jd_end);
        let mut samples: Vec<(f64, f64, f64)> = Vec::new();
        let mut zs: Vec<(f64, f64, f64)> = Vec::new();
        let mut i = start;
        while i < n && jd_of_unix(t[i]) <= stop {
            if let (Some(x), Some(y), Some(z)) = (
                px[i].first().copied(),
                py[i].first().copied(),
                pz[i].first().copied(),
            ) {
                if is_missing(x) || is_missing(y) || is_missing(z) {
                    missing += 1;
                } else if x.is_finite() && y.is_finite() && z.is_finite() {
                    let jd = jd_of_unix(t[i]);
                    samples.push((jd, x, y));
                    zs.push((jd, z, 0.0));
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
                skipped += 1;
                eprintln!("granule {start}: keine Messwerte, uebersprungen");
            }
        }
        start = i;
    }

    if granules.is_empty() {
        eprintln!(
            "{cdf_path}: kein Granule traegt eine Position — {missing} Werte stehen auf dem Fehlt-Marker"
        );
        std::process::exit(2);
    }
    println!("positionen: {missing} fehlend, {skipped} Granule ohne Messwerte uebersprungen");

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
        nut_dec: None,
        nut_pm: None,
    };

    println!(
        "{}: {n} Punkte, {} Granule, {:.1} Tage, NAIF {DE2_NAIF}",
        cdf_path,
        granules.len(),
        jd_end - jd_of_unix(t[0])
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
