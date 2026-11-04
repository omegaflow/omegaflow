use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use omegaflow::inflate::gunzip_stream;

const MAS_PER_RAD: f64 = 206_264_806.247_096_36;

#[derive(Clone, Debug)]
struct Source {
    name: String,
    iers: String,
    defining: bool,
    ra_deg: f64,
    dec_deg: f64,
    e_ra_s: f64,
    e_dec_arcsec: f64,
    corr: f64,
}

#[derive(Clone, Copy, Debug)]
struct Sep {
    dra: f64,
    ddec: f64,
    cxx: f64,
    cyy: f64,
    cxy: f64,
}

impl Sep {
    fn sigma(self) -> f64 {
        let det = self.cxx * self.cyy - self.cxy * self.cxy;
        if !(det > 0.0) {
            return f64::NAN;
        }
        let chi2 = (self.cyy * self.dra * self.dra - 2.0 * self.cxy * self.dra * self.ddec
            + self.cxx * self.ddec * self.ddec)
            / det;
        chi2.max(0.0).sqrt()
    }

    fn sep_mas(self) -> f64 {
        self.dra.hypot(self.ddec) * MAS_PER_RAD
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Rot {
    omega_mas: [f64; 3],
    sigma_mas: [f64; 3],
    chi2_dof: f64,
    n: usize,
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn declared_count(text: &str) -> Option<usize> {
    let first = text.lines().next()?;
    let token = first
        .split_whitespace()
        .find(|t| t.chars().all(|c| c.is_ascii_digit()))?;
    token.parse().ok()
}

fn parse_icrf(text: &str) -> Vec<Source> {
    let mut out = Vec::new();
    for line in text.lines() {
        if !line.starts_with("ICRF J") {
            continue;
        }
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() < 15 {
            continue;
        }
        let mut i = 3;
        let defining = t.get(i) == Some(&"D");
        if defining {
            i += 1;
        }
        let Ok(h) = t[i].parse::<f64>() else {
            continue;
        };
        let Ok(m) = t[i + 1].parse::<f64>() else {
            continue;
        };
        let Ok(s) = t[i + 2].parse::<f64>() else {
            continue;
        };
        i += 3;
        let dec_deg_token = t[i];
        let dec_sign = if dec_deg_token.starts_with('-') {
            -1.0
        } else {
            1.0
        };
        let Ok(dd) = dec_deg_token.parse::<f64>() else {
            continue;
        };
        let Ok(dm) = t[i + 1].parse::<f64>() else {
            continue;
        };
        let Ok(ds) = t[i + 2].parse::<f64>() else {
            continue;
        };
        i += 3;
        let Ok(e_ra_s) = t[i].parse::<f64>() else {
            continue;
        };
        let Ok(e_dec_arcsec) = t[i + 1].parse::<f64>() else {
            continue;
        };
        let Some(corr) = t.get(i + 2).and_then(|v| v.parse::<f64>().ok()) else {
            continue;
        };
        if !(h.is_finite()
            && m.is_finite()
            && s.is_finite()
            && dd.is_finite()
            && e_ra_s.is_finite()
            && e_dec_arcsec.is_finite())
        {
            continue;
        }
        out.push(Source {
            name: t[1].to_string(),
            iers: t[2].to_string(),
            defining,
            ra_deg: (h + m / 60.0 + s / 3600.0) * 15.0,
            dec_deg: (dd.abs() + dm / 60.0 + ds / 3600.0) * dec_sign,
            e_ra_s,
            e_dec_arcsec,
            corr,
        });
    }
    out
}

fn by_name(v: Vec<Source>) -> BTreeMap<String, Source> {
    v.into_iter().map(|s| (s.name.clone(), s)).collect()
}

fn sep(a: &Source, b: &Source) -> Sep {
    let dec_mid = ((a.dec_deg + b.dec_deg) * 0.5).to_radians();
    let dra = ((a.ra_deg - b.ra_deg).to_radians()) * dec_mid.cos();
    let ddec = (a.dec_deg - b.dec_deg).to_radians();
    let ax = a.e_ra_s * 15.0 * dec_mid.cos() / 3600.0 * (std::f64::consts::PI / 180.0);
    let ay = a.e_dec_arcsec / 3600.0 * (std::f64::consts::PI / 180.0);
    let bx = b.e_ra_s * 15.0 * dec_mid.cos() / 3600.0 * (std::f64::consts::PI / 180.0);
    let by = b.e_dec_arcsec / 3600.0 * (std::f64::consts::PI / 180.0);
    Sep {
        dra,
        ddec,
        cxx: ax * ax + bx * bx,
        cyy: ay * ay + by * by,
        cxy: a.corr * ax * ay + b.corr * bx * by,
    }
}

fn pct(part: usize, n: usize) -> f64 {
    if n == 0 {
        f64::NAN
    } else {
        100.0 * part as f64 / n as f64
    }
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    let n = values.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        values[n / 2]
    } else {
        0.5 * (values[n / 2 - 1] + values[n / 2])
    }
}

fn cross(u: [f64; 3], v: [f64; 3]) -> [f64; 3] {
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

fn unit(ra_rad: f64, dec_rad: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let (sa, ca) = ra_rad.sin_cos();
    let (sd, cd) = dec_rad.sin_cos();
    let r = [cd * ca, cd * sa, sd];
    let e = [-sa, ca, 0.0];
    let n = [-sd * ca, -sd * sa, cd];
    (r, e, n)
}

fn fit_rotation(
    names: &[String],
    a: &BTreeMap<String, Source>,
    b: &BTreeMap<String, Source>,
) -> Rot {
    let mut rows: Vec<([f64; 3], f64, f64)> = Vec::with_capacity(names.len() * 2);
    for name in names {
        let (Some(sa), Some(sb)) = (a.get(name), b.get(name)) else {
            continue;
        };
        let s = sep(sa, sb);
        if !(s.cxx > 0.0 && s.cyy > 0.0) {
            continue;
        }
        let ra = ((sa.ra_deg + sb.ra_deg) * 0.5).to_radians();
        let dec = ((sa.dec_deg + sb.dec_deg) * 0.5).to_radians();
        let (r, e, n) = unit(ra, dec);
        rows.push((cross(r, e), s.dra, 1.0 / s.cxx));
        rows.push((cross(r, n), s.ddec, 1.0 / s.cyy));
    }
    if rows.len() < 6 {
        return Rot::default();
    }

    let mut m = [[0.0f64; 3]; 3];
    let mut v = [0.0f64; 3];
    for (row, y, w) in &rows {
        for i in 0..3 {
            v[i] += w * row[i] * y;
            for j in 0..3 {
                m[i][j] += w * row[i] * row[j];
            }
        }
    }

    let mut aug = [[0.0f64; 6]; 3];
    for i in 0..3 {
        for j in 0..3 {
            aug[i][j] = m[i][j];
        }
        aug[i][3 + i] = 1.0;
    }
    for i in 0..3 {
        let mut p = i;
        for r in i + 1..3 {
            if aug[r][i].abs() > aug[p][i].abs() {
                p = r;
            }
        }
        aug.swap(i, p);
        let d = aug[i][i];
        if d.abs() < 1e-30 {
            return Rot::default();
        }
        for j in 0..6 {
            aug[i][j] /= d;
        }
        for r in 0..3 {
            if r != i {
                let f = aug[r][i];
                for j in 0..6 {
                    aug[r][j] -= f * aug[i][j];
                }
            }
        }
    }
    let mut inv = [[0.0f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            inv[i][j] = aug[i][3 + j];
        }
    }
    let mut omega = [0.0f64; 3];
    for i in 0..3 {
        omega[i] = (0..3).map(|j| inv[i][j] * v[j]).sum();
    }

    let mut chi2 = 0.0;
    for (row, y, w) in &rows {
        let pred = omega[0] * row[0] + omega[1] * row[1] + omega[2] * row[2];
        chi2 += w * (y - pred) * (y - pred);
    }
    let dof = rows.len() as f64 - 3.0;
    let scale = if dof > 0.0 {
        (chi2 / dof).max(1.0)
    } else {
        1.0
    };

    Rot {
        omega_mas: [
            omega[0] * MAS_PER_RAD,
            omega[1] * MAS_PER_RAD,
            omega[2] * MAS_PER_RAD,
        ],
        sigma_mas: [
            (inv[0][0] * scale).sqrt() * MAS_PER_RAD,
            (inv[1][1] * scale).sqrt() * MAS_PER_RAD,
            (inv[2][2] * scale).sqrt() * MAS_PER_RAD,
        ],
        chi2_dof: chi2 / dof,
        n: names.len(),
    }
}

struct PairStats {
    n: usize,
    median: f64,
    above3: usize,
    above5: usize,
    max: f64,
}

fn pair_stats(
    names: &[String],
    a: &BTreeMap<String, Source>,
    b: &BTreeMap<String, Source>,
) -> PairStats {
    let mut sigmas: Vec<f64> = Vec::with_capacity(names.len());
    let mut max = 0.0f64;
    for name in names {
        if let (Some(sa), Some(sb)) = (a.get(name), b.get(name)) {
            let s = sep(sa, sb).sigma();
            if s.is_finite() {
                sigmas.push(s);
                max = max.max(s);
            }
        }
    }
    let above3 = sigmas.iter().filter(|s| **s > 3.0).count();
    let above5 = sigmas.iter().filter(|s| **s > 5.0).count();
    let median = median(&mut sigmas);
    PairStats {
        n: sigmas.len(),
        median,
        above3,
        above5,
        max,
    }
}

fn verdikt(r: &Rot) -> &'static str {
    let worst = (0..3)
        .map(|i| {
            if r.sigma_mas[i] > 0.0 {
                (r.omega_mas[i] / r.sigma_mas[i]).abs()
            } else {
                0.0
            }
        })
        .fold(0.0f64, f64::max);
    if worst > 5.0 {
        "Rift (>5 sigma, frame level only)"
    } else if worst > 3.0 {
        "Watch (3-5 sigma)"
    } else {
        "Placed (<=3 sigma)"
    }
}

struct Gaia {
    rows: usize,
    all_three_flags: usize,
    positions: BTreeMap<String, Source>,
    ids: BTreeMap<String, String>,
}

fn read_gaia_gz(path: &Path) -> Option<Gaia> {
    let file = File::open(path).ok()?;
    let mut gaia = Gaia {
        rows: 0,
        all_three_flags: 0,
        positions: BTreeMap::new(),
        ids: BTreeMap::new(),
    };
    let mut pending: Vec<u8> = Vec::new();
    let mut start = 0usize;
    let process = |line: &[u8], gaia: &mut Gaia| {
        if line.len() < 67 {
            return;
        }
        gaia.rows += 1;
        let Ok(icrf_raw) = std::str::from_utf8(&line[46..67]) else {
            return;
        };
        let icrf_name = icrf_raw.trim();
        if let Some(name) = icrf_name.strip_prefix("ICRF ") {
            let name = name.to_string();
            let Ok(source_raw) = std::str::from_utf8(&line[20..39]) else {
                return;
            };
            let source_id = source_raw.trim().to_string();
            let flags = [line[40], line[42], line[44]];
            gaia.ids.insert(name, source_id);
            if flags.iter().all(|f| *f == b'1') {
                gaia.all_three_flags += 1;
            }
        }
    };
    let res = gunzip_stream(file, |chunk| {
        pending.extend_from_slice(chunk);
        while let Some(rel) = pending[start..].iter().position(|b| *b == b'\n') {
            let end = start + rel;
            let line = &pending[start..end];
            process(line, &mut gaia);
            start = end + 1;
        }
        if start > 1 << 20 {
            pending.drain(..start);
            start = 0;
        }
    });
    if res.is_err() {
        return None;
    }
    if start < pending.len() {
        process(&pending[start..], &mut gaia);
    }
    Some(gaia)
}

fn read_gaia_positions(path: &Path, gaia: &mut Gaia) {
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 5 {
            continue;
        }
        let (Ok(ra), Ok(dec), Ok(e_ra_mas), Ok(e_dec_mas)) = (
            f[1].parse::<f64>(),
            f[2].parse::<f64>(),
            f[3].parse::<f64>(),
            f[4].parse::<f64>(),
        ) else {
            continue;
        };
        let Some(name) = gaia
            .ids
            .iter()
            .find_map(|(name, id)| (id == f[0]).then_some(name.clone()))
        else {
            continue;
        };
        let defining = false;
        gaia.positions.insert(
            name,
            Source {
                name: String::new(),
                iers: String::new(),
                defining,
                ra_deg: ra,
                dec_deg: dec,
                e_ra_s: e_ra_mas / 1000.0 / 15.0,
                e_dec_arcsec: e_dec_mas / 1000.0,
                corr: 0.0,
            },
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = match arg_value(&args, "--sources") {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("sources"),
    };
    let no_gaia = args.iter().any(|a| a == "--no-gaia");

    let sx_path = root.join("hpiers.obspm.fr/icrf3sx.txt");
    let k_path = root.join("hpiers.obspm.fr/icrf3k.txt");
    let xka_path = root.join("hpiers.obspm.fr/icrf3xka.txt");

    println!("== Channels");
    let mut loaded = Vec::new();
    for (label, path, named_count) in [
        ("S/X", &sx_path, 4536usize),
        ("K", &k_path, 824),
        ("X/Ka", &xka_path, 678),
    ] {
        let Ok(text) = std::fs::read_to_string(path) else {
            println!("{label}: absent ({})", path.display());
            return;
        };
        let declared = declared_count(&text);
        let parsed = parse_icrf(&text).len();
        let bytes = text.len();
        println!(
            "{label}: path {} | bytes {bytes} | header {} | parsed {parsed} | check {}",
            path.display(),
            match declared {
                Some(n) => n.to_string(),
                None => "?".into(),
            },
            if declared == Some(parsed) && parsed == named_count {
                "ok"
            } else {
                "MISMATCH"
            }
        );
        loaded.push(parse_icrf(&text));
    }
    let sx = by_name(loaded.remove(0));
    let k = by_name(loaded.remove(0));
    let xka = by_name(loaded.remove(0));

    let d_sx = sx.values().filter(|s| s.defining).count();
    let d_k = k.values().filter(|s| s.defining).count();
    let d_xka = xka.values().filter(|s| s.defining).count();
    println!("\n== Defining (D)");
    println!("S/X {d_sx} | K {d_k} | X/Ka {d_xka}");

    let triple: Vec<String> = sx
        .keys()
        .filter(|n| k.contains_key(*n) && xka.contains_key(*n))
        .cloned()
        .collect();
    let d3: Vec<String> = triple
        .iter()
        .filter(|n| {
            sx.get(*n).map(|s| s.defining).unwrap_or(false)
                && k.get(*n).map(|s| s.defining).unwrap_or(false)
                && xka.get(*n).map(|s| s.defining).unwrap_or(false)
        })
        .cloned()
        .collect();
    println!("\n== Triple intersection");
    println!(
        "triple n = {} | D in all three = {}",
        triple.len(),
        d3.len()
    );

    println!("\n== Sigma (2D, Mahalanobis, correlations)");
    for (label, a, b) in [
        ("S/X - K", &sx, &k),
        ("K - X/Ka", &k, &xka),
        ("S/X - X/Ka", &sx, &xka),
    ] {
        let st = pair_stats(&triple, a, b);
        println!(
            "{label}: n {} | median {:.3} | >3 {:.1}% ({}) | >5 {} | max {:.2}",
            st.n,
            st.median,
            pct(st.above3, st.n),
            st.above3,
            st.above5,
            st.max
        );
    }

    let target = "J074554.0-004417";
    if let (Some(a), Some(b), Some(c)) = (sx.get(target), k.get(target), xka.get(target)) {
        println!(
            "\n== 0743-006 ({target}, IERS {}) | D {} {} {}",
            a.iers, a.defining, b.defining, c.defining
        );
        println!(
            "S/X-K {:.2} | K-X/Ka {:.2} | S/X-X/Ka {:.2} | separation S/X-X/Ka {:.3} mas",
            sep(a, b).sigma(),
            sep(b, c).sigma(),
            sep(a, c).sigma(),
            sep(a, c).sep_mas()
        );
    }

    let mut gaia_names: Vec<String> = Vec::new();
    if !no_gaia {
        let gz = root.join("cdsarc.cds.unistra.fr/gcrf3xm.dat.gz");
        println!("\n== Gaia-CRF3");
        if let Some(mut gaia) = read_gaia_gz(&gz) {
            let gaia_rows = gaia.rows;
            let pos_path = root.join("cdsarc.cds.unistra.fr/gaia_crf3_icrf3_positions.csv");
            read_gaia_positions(&pos_path, &mut gaia);
            for (name, src) in gaia.positions.iter_mut() {
                if let Some(s) = sx.get(name) {
                    src.name = name.clone();
                    src.iers = s.iers.clone();
                    src.defining = s.defining
                        && k.get(name).map(|x| x.defining).unwrap_or(false)
                        && xka.get(name).map(|x| x.defining).unwrap_or(false);
                }
                gaia_names.push(name.clone());
            }
            let in_triple = triple.iter().filter(|n| gaia.ids.contains_key(*n)).count();
            println!(
                "gcrf3xm rows {gaia_rows} | in triple intersection {in_triple}/{} | all three flags {}",
                triple.len(),
                gaia.all_three_flags
            );
            println!(
                "Gaia positions loaded {} | triple-intersection coverage {:.1}%",
                gaia.positions.len(),
                100.0 * gaia.positions.len() as f64 / triple.len() as f64
            );
            if !gaia.positions.is_empty() {
                let st = pair_stats(&gaia_names, &gaia.positions, &sx);
                println!(
                    "Gaia - S/X: n {} | median {:.3} | >3 {:.1}% ({}) | >5 {} | max {:.2}",
                    st.n,
                    st.median,
                    pct(st.above3, st.n),
                    st.above3,
                    st.above5,
                    st.max
                );
                let gd3: Vec<String> = gaia_names
                    .iter()
                    .filter(|n| gaia.positions.get(*n).map(|s| s.defining).unwrap_or(false))
                    .cloned()
                    .collect();
                let r = fit_rotation(&gd3, &gaia.positions, &sx);
                println!(
                    "Gaia-S/X Rotation (D3 n {}): omega [{:+.3} {:+.3} {:+.3}] mas, sigma [{:.3} {:.3} {:.3}], chi2/dof {:.1} -> {}",
                    r.n,
                    r.omega_mas[0],
                    r.omega_mas[1],
                    r.omega_mas[2],
                    r.sigma_mas[0],
                    r.sigma_mas[1],
                    r.sigma_mas[2],
                    r.chi2_dof,
                    verdikt(&r)
                );
            }
            if let Some(path) = arg_value(&args, "--missing-out") {
                let missing: Vec<&String> = triple
                    .iter()
                    .filter(|n| !gaia.positions.contains_key(*n))
                    .collect();
                let mut out = String::from("iers,ra_deg,dec_deg\n");
                for name in &missing {
                    if let Some(src) = sx.get(*name) {
                        out.push_str(&format!(
                            "{},{:.9},{:.9}\n",
                            src.iers, src.ra_deg, src.dec_deg
                        ));
                    }
                }
                match std::fs::write(&path, out) {
                    Ok(()) => {
                        println!("missing Gaia anchors: {} -> {}", missing.len(), path)
                    }
                    Err(e) => println!("missing-out {path}: {e}"),
                }
            }
            if let Some(path) = arg_value(&args, "--gaia-map-out") {
                let mut out = String::from("iers,gaia_source_id,ra_deg,dec_deg\n");
                for name in &triple {
                    let (Some(id), Some(src)) = (gaia.ids.get(name), sx.get(name)) else {
                        continue;
                    };
                    out.push_str(&format!(
                        "{},{},{:.9},{:.9}\n",
                        src.iers, id, src.ra_deg, src.dec_deg
                    ));
                }
                match std::fs::write(&path, out) {
                    Ok(()) => println!("Gaia anchor map -> {path}"),
                    Err(e) => println!("gaia-map-out {path}: {e}"),
                }
            }
        } else {
            println!("gcrf3xm absent or unreadable: {}", gz.display());
        }
    }

    println!("\n== Structure candidates (individual sources, max pair sigma)");
    let mut structure: Vec<(String, f64, f64, f64, bool, bool, bool, String)> = Vec::new();
    for name in &triple {
        let (a, b, c) = (
            sx.get(name).unwrap(),
            k.get(name).unwrap(),
            xka.get(name).unwrap(),
        );
        let s1 = sep(a, b).sigma();
        let s2 = sep(b, c).sigma();
        let s3 = sep(a, c).sigma();
        let m = s1.max(s2).max(s3);
        if m > 5.0 {
            structure.push((
                name.clone(),
                s1,
                s2,
                s3,
                a.defining,
                b.defining,
                c.defining,
                a.iers.clone(),
            ));
        }
    }
    structure.sort_by(|x, y| {
        y.1.max(y.2)
            .max(y.3)
            .partial_cmp(&x.1.max(x.2).max(x.3))
            .unwrap()
    });
    println!("n(>5) = {}", structure.len());
    for (name, s1, s2, s3, da, db, dc, iers) in structure.iter().take(15) {
        println!(
            "{name} {iers} D[{}{}{}] | SX-K {:.2} | K-XKa {:.2} | SX-XKa {:.2}",
            if *da { "D" } else { "-" },
            if *db { "D" } else { "-" },
            if *dc { "D" } else { "-" },
            s1,
            s2,
            s3
        );
    }

    println!("\n== Frame suspicion (ensemble of D3 sources, rotation a-b)");
    for (label, a, b) in [
        ("S/X - K", &sx, &k),
        ("K - X/Ka", &k, &xka),
        ("S/X - X/Ka", &sx, &xka),
    ] {
        let r = fit_rotation(&d3, a, b);
        let worst = (0..3)
            .map(|i| {
                if r.sigma_mas[i] > 0.0 {
                    (r.omega_mas[i] / r.sigma_mas[i]).abs()
                } else {
                    0.0
                }
            })
            .fold(0.0f64, f64::max);
        println!(
            "{label}: n {} | omega [{:+.3} {:+.3} {:+.3}] mas | sigma [{:.3} {:.3} {:.3}] | max {:.2} sigma | chi2/dof {:.1} -> {}",
            r.n,
            r.omega_mas[0],
            r.omega_mas[1],
            r.omega_mas[2],
            r.sigma_mas[0],
            r.sigma_mas[1],
            r.sigma_mas[2],
            worst,
            r.chi2_dof,
            verdikt(&r)
        );
    }

    println!("\n== Frame suspicion (ensemble of D in S/X, 303)");
    let d_sx: Vec<String> = sx
        .iter()
        .filter(|(_, s)| s.defining)
        .map(|(n, _)| n.clone())
        .collect();
    println!("D in S/X: {}", d_sx.len());
    for (label, b) in [("S/X - K", &k), ("S/X - X/Ka", &xka)] {
        let names: Vec<String> = d_sx
            .iter()
            .filter(|n| b.contains_key(*n))
            .cloned()
            .collect();
        let r = fit_rotation(&names, &sx, b);
        let worst = (0..3)
            .map(|i| {
                if r.sigma_mas[i] > 0.0 {
                    (r.omega_mas[i] / r.sigma_mas[i]).abs()
                } else {
                    0.0
                }
            })
            .fold(0.0f64, f64::max);
        println!(
            "{label}: n {} | omega [{:+.3} {:+.3} {:+.3}] mas | sigma [{:.3} {:.3} {:.3}] | max {:.2} sigma | chi2/dof {:.1} -> {}",
            r.n,
            r.omega_mas[0],
            r.omega_mas[1],
            r.omega_mas[2],
            r.sigma_mas[0],
            r.sigma_mas[1],
            r.sigma_mas[2],
            worst,
            r.chi2_dof,
            verdikt(&r)
        );
    }

    println!("\n== Limits");
    println!(
        "S/X and K/X/Ka share telescopes and source structure: the triple intersection is a consistency test,"
    );
    println!(
        "not a proof of independence. Gaia-CRF3 is the optical counter-line, but only for the"
    );
    println!("intersected subset and with its own epoch and systematics. Error bars are formal;");
    println!(
        "chi2/dof > 1 indicates underestimated formal errors or real core shift, not a frame rift."
    );
}
