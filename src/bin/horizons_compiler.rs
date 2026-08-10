use std::process::Command;

const CHEBYSHEV_DEGREE: usize = 17;
const GRANULE_DAYS: f64 = 32.0;
const N_SAMPLES: usize = 25;
const MAGIC_HEADER: [u8; 4] = [0xCF, 0x86, 0x01, 0x00];

struct BodyProps {
    a0_deg: f64,
    da0_dt_deg_per_century: f64,
    d0_deg: f64,
    dd0_dt_deg_per_century: f64,
    w0_deg: f64,
    dw_dt_deg_per_day: f64,
    radius_m: f64,
    flattening: f64,
}

const BODIES_WITH_MEDIA: &[&str] = &[
    "sun",
    "mercury",
    "venus",
    "earth",
    "moon",
    "mars",
    "jupiter",
    "saturn",
    "uranus",
    "neptune",
    "pluto",
    "io",
    "europa",
    "ganymede",
    "callisto",
    "enceladus",
    "rhea",
    "dione",
    "tethys",
    "titan",
    "triton",
    "phobos",
    "deimos",
];

fn wgccre_for_body(body: &str) -> Option<BodyProps> {
    match body {
        "sun" => Some(BodyProps {
            a0_deg: 286.13,
            da0_dt_deg_per_century: 0.0,
            d0_deg: 63.87,
            dd0_dt_deg_per_century: 0.0,
            w0_deg: 84.176,
            dw_dt_deg_per_day: 14.1844,
            radius_m: 696000000.0,
            flattening: 0.0,
        }),
        "mercury" => Some(BodyProps {
            a0_deg: 281.01,
            da0_dt_deg_per_century: -0.033,
            d0_deg: 61.45,
            dd0_dt_deg_per_century: -0.005,
            w0_deg: 329.548,
            dw_dt_deg_per_day: 6.1385,
            radius_m: 2439700.0,
            flattening: 0.0,
        }),
        "venus" => Some(BodyProps {
            a0_deg: 272.76,
            da0_dt_deg_per_century: 0.0,
            d0_deg: 67.16,
            dd0_dt_deg_per_century: 0.0,
            w0_deg: 160.20,
            dw_dt_deg_per_day: -1.4814,
            radius_m: 6051800.0,
            flattening: 0.0,
        }),
        "earth" => Some(BodyProps {
            a0_deg: 0.0,
            da0_dt_deg_per_century: 0.0,
            d0_deg: 90.0,
            dd0_dt_deg_per_century: 0.0,
            w0_deg: 190.147,
            dw_dt_deg_per_day: 360.9856235,
            radius_m: 6378136.6,
            flattening: 0.0033527,
        }),
        "moon" => Some(BodyProps {
            a0_deg: 269.9949,
            da0_dt_deg_per_century: 0.0031,
            d0_deg: 66.5392,
            dd0_dt_deg_per_century: 0.013,
            w0_deg: 38.3213,
            dw_dt_deg_per_day: 13.17635815,
            radius_m: 1737400.0,
            flattening: 0.0,
        }),
        "mars" => Some(BodyProps {
            a0_deg: 317.68143,
            da0_dt_deg_per_century: -0.1061,
            d0_deg: 52.88650,
            dd0_dt_deg_per_century: -0.0609,
            w0_deg: 176.630,
            dw_dt_deg_per_day: 350.89198226,
            radius_m: 3396190.0,
            flattening: 0.00589,
        }),
        "jupiter" => Some(BodyProps {
            a0_deg: 268.056595,
            da0_dt_deg_per_century: -0.006499,
            d0_deg: 64.495303,
            dd0_dt_deg_per_century: 0.002413,
            w0_deg: 284.95,
            dw_dt_deg_per_day: 870.536,
            radius_m: 71492000.0,
            flattening: 0.06487,
        }),
        "saturn" => Some(BodyProps {
            a0_deg: 40.589,
            da0_dt_deg_per_century: -0.036,
            d0_deg: 83.537,
            dd0_dt_deg_per_century: -0.004,
            w0_deg: 38.90,
            dw_dt_deg_per_day: 810.7939024,
            radius_m: 60268000.0,
            flattening: 0.09796,
        }),
        "uranus" => Some(BodyProps {
            a0_deg: 257.311,
            da0_dt_deg_per_century: 0.0,
            d0_deg: -15.175,
            dd0_dt_deg_per_century: 0.0,
            w0_deg: 203.81,
            dw_dt_deg_per_day: -501.1600928,
            radius_m: 25559000.0,
            flattening: 0.02293,
        }),
        "neptune" => Some(BodyProps {
            a0_deg: 299.36,
            da0_dt_deg_per_century: 0.70,
            d0_deg: 43.46,
            dd0_dt_deg_per_century: -0.51,
            w0_deg: 253.18,
            dw_dt_deg_per_day: 536.3128492,
            radius_m: 24764000.0,
            flattening: 0.0171,
        }),
        "pluto" => Some(BodyProps {
            a0_deg: 132.993,
            da0_dt_deg_per_century: 0.0,
            d0_deg: -6.163,
            dd0_dt_deg_per_century: 0.0,
            w0_deg: 302.695,
            dw_dt_deg_per_day: 56.3625225,
            radius_m: 1188300.0,
            flattening: 0.0,
        }),
        "io" => Some(BodyProps {
            a0_deg: 268.05,
            da0_dt_deg_per_century: -0.009,
            d0_deg: 64.50,
            dd0_dt_deg_per_century: 0.003,
            w0_deg: 200.39,
            dw_dt_deg_per_day: 203.4889538,
            radius_m: 1821600.0,
            flattening: 0.0,
        }),
        "europa" => Some(BodyProps {
            a0_deg: 268.08,
            da0_dt_deg_per_century: -0.009,
            d0_deg: 64.51,
            dd0_dt_deg_per_century: 0.003,
            w0_deg: 35.98,
            dw_dt_deg_per_day: 101.3747235,
            radius_m: 1560800.0,
            flattening: 0.0,
        }),
        "ganymede" => Some(BodyProps {
            a0_deg: 268.20,
            da0_dt_deg_per_century: -0.009,
            d0_deg: 64.57,
            dd0_dt_deg_per_century: 0.003,
            w0_deg: 44.064,
            dw_dt_deg_per_day: 50.3176081,
            radius_m: 2631200.0,
            flattening: 0.0,
        }),
        "callisto" => Some(BodyProps {
            a0_deg: 268.72,
            da0_dt_deg_per_century: -0.009,
            d0_deg: 64.83,
            dd0_dt_deg_per_century: 0.003,
            w0_deg: 259.51,
            dw_dt_deg_per_day: 21.5710715,
            radius_m: 2410300.0,
            flattening: 0.0,
        }),
        "titan" => Some(BodyProps {
            a0_deg: 36.41,
            da0_dt_deg_per_century: -0.036,
            d0_deg: 83.94,
            dd0_dt_deg_per_century: -0.004,
            w0_deg: 189.64,
            dw_dt_deg_per_day: 22.5769768,
            radius_m: 2575500.0,
            flattening: 0.0,
        }),
        "triton" => Some(BodyProps {
            a0_deg: 299.36,
            da0_dt_deg_per_century: 0.70,
            d0_deg: 43.46,
            dd0_dt_deg_per_century: -0.51,
            w0_deg: 296.53,
            dw_dt_deg_per_day: -61.2572637,
            radius_m: 1353400.0,
            flattening: 0.0,
        }),
        "enceladus" => Some(BodyProps {
            a0_deg: 40.66,
            da0_dt_deg_per_century: -0.036,
            d0_deg: 83.52,
            dd0_dt_deg_per_century: -0.004,
            w0_deg: 36.41,
            dw_dt_deg_per_day: 262.7318996,
            radius_m: 252100.0,
            flattening: 0.0,
        }),
        "rhea" => Some(BodyProps {
            a0_deg: 40.38,
            da0_dt_deg_per_century: -0.036,
            d0_deg: 83.55,
            dd0_dt_deg_per_century: -0.004,
            w0_deg: 345.65,
            dw_dt_deg_per_day: 79.6900478,
            radius_m: 763800.0,
            flattening: 0.0,
        }),
        "dione" => Some(BodyProps {
            a0_deg: 40.66,
            da0_dt_deg_per_century: -0.036,
            d0_deg: 83.52,
            dd0_dt_deg_per_century: -0.004,
            w0_deg: 357.00,
            dw_dt_deg_per_day: 131.5349316,
            radius_m: 561400.0,
            flattening: 0.0,
        }),
        "tethys" => Some(BodyProps {
            a0_deg: 50.41,
            da0_dt_deg_per_century: -0.036,
            d0_deg: 83.55,
            dd0_dt_deg_per_century: -0.004,
            w0_deg: 299.11,
            dw_dt_deg_per_day: 190.6979086,
            radius_m: 531100.0,
            flattening: 0.0,
        }),
        "phobos" => Some(BodyProps {
            a0_deg: 317.68,
            da0_dt_deg_per_century: -0.108,
            d0_deg: 54.46,
            dd0_dt_deg_per_century: -0.061,
            w0_deg: 165.00,
            dw_dt_deg_per_day: 1128.844759,
            radius_m: 11260.0,
            flattening: 0.0,
        }),
        "deimos" => Some(BodyProps {
            a0_deg: 317.68,
            da0_dt_deg_per_century: -0.108,
            d0_deg: 54.46,
            dd0_dt_deg_per_century: -0.061,
            w0_deg: 240.00,
            dw_dt_deg_per_day: 285.161891,
            radius_m: 6230.0,
            flattening: 0.0,
        }),
        _ => None,
    }
}

fn chebyshev_nodes(n: usize) -> Vec<f64> {
    let mut nodes = Vec::with_capacity(n);
    for k in 0..n {
        nodes.push(((std::f64::consts::PI * (n as f64 - k as f64 - 0.5)) / n as f64).cos());
    }
    nodes
}

fn chebyshev_polys(n: usize, x: f64) -> Vec<f64> {
    let mut t = vec![1.0, x];
    for k in 2..n {
        let next = 2.0 * x * t[k - 1] - t[k - 2];
        t.push(next);
    }
    t.truncate(n);
    t
}

fn chebyshev_fit(
    samples: &[(f64, f64, f64)],
    degree: usize,
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let m = samples.len();
    if m < degree + 1 {
        return None;
    }
    let nodes = chebyshev_nodes(m);
    let mut a = vec![vec![0.0; degree + 1]; m];
    for i in 0..m {
        let polys = chebyshev_polys(degree + 1, nodes[i]);
        for j in 0..=degree {
            a[i][j] = polys[j];
        }
    }
    let mut ata = vec![vec![0.0; degree + 1]; degree + 1];
    for i in 0..m {
        for j in 0..=degree {
            for k in 0..=degree {
                ata[j][k] += a[i][j] * a[i][k];
            }
        }
    }
    let mut atx = vec![0.0; degree + 1];
    let mut aty = vec![0.0; degree + 1];
    let mut atz = vec![0.0; degree + 1];
    for i in 0..m {
        for j in 0..=degree {
            atx[j] += a[i][j] * samples[i].0;
            aty[j] += a[i][j] * samples[i].1;
            atz[j] += a[i][j] * samples[i].2;
        }
    }
    let (cx, cy, cz) = solve_normal_equations(&ata, &atx, &aty, &atz)?;
    Some((cx, cy, cz))
}

fn solve_normal_equations(
    ata: &[Vec<f64>],
    atx: &[f64],
    aty: &[f64],
    atz: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let n = ata.len();
    let mut a = ata.to_vec();
    for i in 0..n {
        let mut pivot = i;
        for j in i + 1..n {
            if a[j][i].abs() > a[pivot][i].abs() {
                pivot = j;
            }
        }
        if a[pivot][i].abs() < 1e-15 {
            return None;
        }
        a.swap(i, pivot);
        for j in i + 1..n {
            let factor = a[j][i] / a[i][i];
            for k in i..n {
                a[j][k] -= factor * a[i][k];
            }
        }
    }
    let x = back_substitute(&a, atx);
    let y = back_substitute(&a, aty);
    let z = back_substitute(&a, atz);
    Some((x, y, z))
}

fn back_substitute(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = a.len();
    let mut x = b.to_vec();
    for i in (0..n).rev() {
        for j in i + 1..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    x
}

fn write_binary(
    path: &str,
    body_name: &str,
    granules: &[(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    rotations: &[(f64, [f64; 9])],
    wgccre: &BodyProps,
    has_media: bool,
) {
    let mut n_sections: u32 = 1;
    n_sections += 1;
    if has_media {
        n_sections += 1;
    }
    if !rotations.is_empty() {
        n_sections += 1;
    }
    let mut buf = Vec::new();
    buf.extend_from_slice(&MAGIC_HEADER);
    buf.extend_from_slice(&n_sections.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&(granules.len() as u32).to_le_bytes());
    buf.extend_from_slice(&(CHEBYSHEV_DEGREE as u32).to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    for (t0, dt, cx, cy, cz) in granules {
        buf.extend_from_slice(&t0.to_le_bytes());
        buf.extend_from_slice(&dt.to_le_bytes());
        for &c in cx {
            buf.extend_from_slice(&c.to_le_bytes());
        }
        for &c in cy {
            buf.extend_from_slice(&c.to_le_bytes());
        }
        for &c in cz {
            buf.extend_from_slice(&c.to_le_bytes());
        }
    }
    {
        let section_stype: u32 = 1;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let params: [f64; 8] = [
            wgccre.a0_deg,
            wgccre.da0_dt_deg_per_century,
            wgccre.d0_deg,
            wgccre.dd0_dt_deg_per_century,
            wgccre.w0_deg,
            wgccre.dw_dt_deg_per_day,
            wgccre.radius_m,
            wgccre.flattening,
        ];
        for &p in &params {
            buf.extend_from_slice(&p.to_le_bytes());
        }
    }
    if has_media {
        let section_stype: u32 = 2;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for &p in &[0.0_f64; 5] {
            buf.extend_from_slice(&p.to_le_bytes());
        }
    }
    if !rotations.is_empty() {
        let section_stype: u32 = 3;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&(rotations.len() as u32).to_le_bytes());
        buf.extend_from_slice(&8u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for (t0, mat) in rotations {
            buf.extend_from_slice(&t0.to_le_bytes());
            for &v in mat {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    if let Err(e) = std::fs::write(path, &buf) {
        eprintln!("write {}: {}", path, e);
    } else {
        eprintln!(
            "  {}: {} granules, {} B",
            body_name,
            granules.len(),
            buf.len()
        );
    }
}

fn horizons_request(command: &str, t_start_jd: f64, t_stop_jd: f64) -> Option<String> {
    let url = format!(
        "https://ssd.jpl.nasa.gov/api/horizons.api?format=text\
         &COMMAND='{cmd}'&CENTER='500@0'&MAKE_EPHEM='YES'&EPHEM_TYPE='VECTORS'\
         &START_TIME='JD+{t_start:.2}'&STOP_TIME='JD+{t_stop:.2}'&STEP_SIZE='1+d'",
        cmd = command,
        t_start = t_start_jd,
        t_stop = t_stop_jd,
    );
    let output = Command::new("curl")
        .args([
            "-sS",
            "-L",
            "-k",
            "--max-time",
            "180",
            "--retry",
            "2",
            "-H",
            "User-Agent: omegaflow-ci/1.0",
        ])
        .arg(&url)
        .output()
        .ok()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("  curl exit {}: {}", output.status, stderr.trim());
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

fn extract_vectors(text: &str) -> Vec<(f64, f64, f64, f64)> {
    let mut vectors = Vec::new();
    let mut in_block = false;
    let mut current_jd: Option<f64> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("$$SOE") {
            in_block = true;
            continue;
        }
        if line.starts_with("$$EOE") {
            break;
        }
        if !in_block || line.is_empty() {
            continue;
        }
        if line.starts_with("VX=") || line.starts_with("LT=") {
            continue;
        }
        if let Some(eq_pos) = line.find('=') {
            let before_eq = line[..eq_pos].trim();
            if let Ok(jd) = before_eq.parse::<f64>() {
                current_jd = Some(jd);
            }
        }
        if let Some(jd) = current_jd {
            if line.contains("X =") && line.contains("Y =") && line.contains("Z =") {
                let mut x: Option<f64> = None;
                let mut y: Option<f64> = None;
                let mut z: Option<f64> = None;
                let parts: Vec<&str> = line.split_whitespace().collect();
                for (i, p) in parts.iter().enumerate() {
                    if *p == "X" && i + 2 < parts.len() {
                        x = parts[i + 2].parse().ok();
                    }
                    if *p == "Y" && i + 2 < parts.len() {
                        y = parts[i + 2].parse().ok();
                    }
                    if *p == "Z" && i + 2 < parts.len() {
                        z = parts[i + 2].parse().ok();
                    }
                }
                if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                    let x_m = x * 1000.0;
                    let ecliptic_c = 0.409092804_f64.cos();
                    let ecliptic_s = 0.409092804_f64.sin();
                    let y_icrs = y * 1000.0 * ecliptic_c - z * 1000.0 * ecliptic_s;
                    let z_icrs = y * 1000.0 * ecliptic_s + z * 1000.0 * ecliptic_c;
                    vectors.push((jd, x_m, y_icrs, z_icrs));
                }
            }
        }
    }
    vectors
}

fn current_jd() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    2451545.0 + now / 86400.0
}

fn fit_granule_from_samples(
    samples: &[(f64, f64, f64, f64)],
    t0_jd: f64,
    half_jd: f64,
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if samples.len() < 4 {
        return None;
    }
    let sample_times: Vec<f64> = samples.iter().map(|s| s.0).collect();
    let xs: Vec<f64> = samples.iter().map(|s| s.1).collect();
    let ys: Vec<f64> = samples.iter().map(|s| s.2).collect();
    let zs: Vec<f64> = samples.iter().map(|s| s.3).collect();
    let cheb_nodes = chebyshev_nodes(N_SAMPLES);
    let mut interp_x = vec![0.0; N_SAMPLES];
    let mut interp_y = vec![0.0; N_SAMPLES];
    let mut interp_z = vec![0.0; N_SAMPLES];
    for k in 0..N_SAMPLES {
        let t = t0_jd + cheb_nodes[k] * half_jd;
        let idx = sample_times
            .binary_search_by(|v| v.partial_cmp(&t).unwrap_or(std::cmp::Ordering::Less));
        match idx {
            Ok(i) => {
                interp_x[k] = xs[i];
                interp_y[k] = ys[i];
                interp_z[k] = zs[i];
            }
            Err(i) => {
                if i == 0 {
                    interp_x[k] = xs[0];
                    interp_y[k] = ys[0];
                    interp_z[k] = zs[0];
                } else if i >= sample_times.len() {
                    let last = sample_times.len() - 1;
                    interp_x[k] = xs[last];
                    interp_y[k] = ys[last];
                    interp_z[k] = zs[last];
                } else {
                    let frac = (t - sample_times[i - 1]) / (sample_times[i] - sample_times[i - 1]);
                    interp_x[k] = xs[i - 1] + frac * (xs[i] - xs[i - 1]);
                    interp_y[k] = ys[i - 1] + frac * (ys[i] - ys[i - 1]);
                    interp_z[k] = zs[i - 1] + frac * (zs[i] - zs[i - 1]);
                }
            }
        }
    }
    let combined: Vec<(f64, f64, f64)> = (0..N_SAMPLES)
        .map(|k| (interp_x[k], interp_y[k], interp_z[k]))
        .collect();
    chebyshev_fit(&combined, CHEBYSHEV_DEGREE)
}

fn generate_from_horizons(
    command: &str,
    body_name: &str,
    months: f64,
    lookback: f64,
) -> Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)> {
    let jd_now = current_jd();
    let t_start = jd_now - lookback;
    let t_stop = jd_now + months * 30.44;
    let text = match horizons_request(command, t_start, t_stop) {
        Some(t) => t,
        None => {
            eprintln!(
                "  {}: Horizons request returned void (curl or API issue)",
                body_name
            );
            return Vec::new();
        }
    };
    let vectors = extract_vectors(&text);
    if vectors.len() < 10 {
        eprintln!(
            "  SKIP {}: {} vectors (min 10), lead: {:?}",
            body_name,
            vectors.len(),
            text.get(..200)
        );
        return Vec::new();
    }
    let mut granules = Vec::new();
    let n = ((t_stop - t_start) / GRANULE_DAYS).ceil() as usize;
    for i in 0..n {
        let mid_jd = t_start + (i as f64 + 0.5) * GRANULE_DAYS;
        let half_jd = GRANULE_DAYS / 2.0;
        if let Some((cx, cy, cz)) = fit_granule_from_samples(&vectors, mid_jd, half_jd) {
            granules.push((mid_jd, half_jd, cx, cy, cz));
        }
    }
    granules
}

fn main() {
    let bodies_stable: &[(&str, &str)] = &[
        ("Ceres;", "ceres"),
        ("Vesta", "vesta"),
        ("Eris;", "eris"),
        ("Haumea;", "haumea"),
        ("Makemake;", "makemake"),
        ("Apophis", "apophis"),
        ("Bennu", "bennu"),
        ("90000031", "encke"),
    ];
    let bodies_dynamic: &[(&str, &str)] = &[
        ("-125544", "iss"),
        ("-31", "voyager1"),
        ("-32", "voyager2"),
        ("-98", "new_horizons"),
        ("-96", "parker_solar_probe"),
        ("-144", "solar_orbiter"),
        ("-170", "jwst"),
        ("-61", "juno"),
        ("2020-047A", "atlas_3i"),
    ];
    let bodies_retry: &[(&str, &str)] = &[];
    let run_body = |command: &str, body_name: &str, months: f64, lookback: f64| {
        let granules = generate_from_horizons(command, body_name, months, lookback);
        if granules.is_empty() {
            return;
        }
        let wgccre = match wgccre_for_body(body_name) {
            Some(w) => w,
            None => BodyProps {
                a0_deg: 0.0,
                da0_dt_deg_per_century: 0.0,
                d0_deg: 0.0,
                dd0_dt_deg_per_century: 0.0,
                w0_deg: 0.0,
                dw_dt_deg_per_day: 0.0,
                radius_m: 0.0,
                flattening: 0.0,
            },
        };
        let has_media = BODIES_WITH_MEDIA.contains(&body_name);
        let path = format!("ephemeris_{}.bin", body_name);
        write_binary(&path, body_name, &granules, &[], &wgccre, has_media);
    };
    for (cmd, name) in bodies_stable {
        eprintln!("  {} (Horizons stable)", name);
        run_body(cmd, name, 12.0, 30.0);
    }
    for (cmd, name) in bodies_dynamic {
        let months = if *name == "iss" { 0.9 } else { 1.0 };
        eprintln!("  {} (Horizons dynamic, {:.1}mo)", name, months);
        run_body(cmd, name, months, 5.0);
    }
    for (cmd, name) in bodies_retry {
        eprintln!("  {} (Horizons retry)", name);
        run_body(cmd, name, 1.0, 1.0);
    }
}
