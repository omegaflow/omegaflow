use omegaflow::fit::solve_normal_equations;
use std::process::Command;

const CHEBYSHEV_DEGREE: usize = 17;
const GRANULE_DAYS: f64 = 32.0;
const N_SAMPLES: usize = 25;
const MAGIC_HEADER: [u8; 4] = [0xCF, 0x86, 0x02, 0x00];

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

fn write_binary(
    path: &str,
    body_name: &str,
    granules: &[(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    rotations: &[(f64, [f64; 9])],
    gm_m3_s2: Option<f64>,
) {
    let has_props = gm_m3_s2.is_some();
    let mut n_sections: u32 = 2;
    if has_props {
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
    if has_props {
        let section_stype: u32 = 1;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let slots: [Option<f64>; 12] = [
            None, None, None, None, None, None, None, None, None, None, None, gm_m3_s2,
        ];
        let mut mask: u16 = 0;
        for (i, v) in slots.iter().enumerate() {
            match v {
                Some(x) => {
                    buf.extend_from_slice(&x.to_le_bytes());
                    mask |= 1 << i;
                }
                None => buf.extend_from_slice(&0.0_f64.to_le_bytes()),
            }
        }
        buf.extend_from_slice(&mask.to_le_bytes());
        buf.extend_from_slice(&[0u8; 6]);
    }
    {
        let section_stype: u32 = 2;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let kernel_params =
            omegaflow::media::medium_params_of(body_name).map_or([0.0; 5], |m| m.wire());
        for &p in &kernel_params {
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
    let cmd_safe = command.replace(';', "%3B");
    let url = format!(
        "https://ssd.jpl.nasa.gov/api/horizons.api?format=text\
         &COMMAND='{cmd}'&CENTER='500@0'&MAKE_EPHEM='YES'&EPHEM_TYPE='VECTORS'\
         &START_TIME='JD+{t_start:.2}'&STOP_TIME='JD+{t_stop:.2}'&STEP_SIZE='1+d'\
         &QUANTITIES='1,2,4'",
        cmd = cmd_safe,
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

fn extract_vectors(text: &str) -> (Vec<(f64, f64, f64, f64)>, Option<f64>) {
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
                    let val = if *p == "X" || *p == "Y" || *p == "Z" {
                        if i + 1 < parts.len() && parts[i + 1].starts_with('=') {
                            let v = &parts[i + 1][1..];
                            if v.is_empty() && i + 2 < parts.len() {
                                parts[i + 2].parse().ok()
                            } else {
                                v.parse().ok()
                            }
                        } else if i + 2 < parts.len() && parts[i + 1] == "=" {
                            parts[i + 2].parse().ok()
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    match *p {
                        "X" => x = val,
                        "Y" => y = val,
                        "Z" => z = val,
                        _ => {}
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
    (vectors, extract_gm_km3_s2(text).map(|g| g * 1.0e9))
}

fn extract_gm_km3_s2(text: &str) -> Option<f64> {
    for line in text.lines() {
        let line = line.trim();
        if let Some(pos) = line.find("GM=") {
            let rest = &line[pos + 3..];
            let first = rest.split_whitespace().next()?;
            return first.parse().ok();
        }
    }
    None
}

fn current_jd() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    const UNIX_AT_J2000: f64 = 946728000.0;
    const J2000_JD: f64 = 2451545.0;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    J2000_JD + (now - UNIX_AT_J2000) / 86400.0
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
) -> (Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)>, Option<f64>) {
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
            return (Vec::new(), None);
        }
    };
    let (vectors, gm_m3_s2) = extract_vectors(&text);
    if vectors.len() < 10 {
        eprintln!(
            "  SKIP {}: {} vectors (min 10), lead: {:?}",
            body_name,
            vectors.len(),
            text.get(..200)
        );
        return (Vec::new(), gm_m3_s2);
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
    (granules, gm_m3_s2)
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
        ("506", "himalia"),
        ("610", "janus"),
        ("611", "epimetheus"),
        ("615", "atlas"),
        ("616", "prometheus"),
        ("617", "pandora"),
    ];
    let bodies_dynamic: &[(&str, &str, f64)] = &[
        ("-125544", "iss", 0.9),
        ("-31", "voyager1", 1.0),
        ("-32", "voyager2", 1.0),
        ("-98", "new_horizons", 1.0),
        ("-96", "parker_solar_probe", 1.0),
        ("-144", "solar_orbiter", 1.0),
        ("-170", "jwst", 1.0),
        ("-61", "juno", 1.0),
        ("2020-047A", "atlas_3i", 1.0),
    ];
    let bodies_retry: &[(&str, &str)] = &[];
    let ci_mode = std::env::args().any(|a| a == "--ci-mode");
    let run_body = |command: &str, body_name: &str, months: f64, lookback: f64| {
        let (granules, gm_m3_s2) = generate_from_horizons(command, body_name, months, lookback);
        if granules.is_empty() {
            return;
        }
        if gm_m3_s2.is_none() {
            eprintln!("  {}: granules only, no Horizons GM", body_name);
        }
        let path = format!("ephemeris_{}.bin", body_name);
        write_binary(&path, body_name, &granules, &[], gm_m3_s2);
        if ci_mode && !omegaflow::cdn::upload_asset(&path) {
            eprintln!("upload: {} did not reach the CDN", path);
            std::process::exit(1);
        }
    };
    for (cmd, name) in bodies_stable {
        eprintln!("  {} (Horizons stable)", name);
        run_body(cmd, name, 12.0, 30.0);
    }
    for (cmd, name, months) in bodies_dynamic {
        eprintln!("  {} (Horizons dynamic, {:.1}mo)", name, months);
        run_body(cmd, name, *months, 5.0);
    }
    for (cmd, name) in bodies_retry {
        eprintln!("  {} (Horizons retry)", name);
        run_body(cmd, name, 1.0, 1.0);
    }
}
