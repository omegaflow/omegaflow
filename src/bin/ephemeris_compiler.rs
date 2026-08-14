use omegaflow::bsp_reader::spk::SpkFile;
use omegaflow::pck::{neutral, PckBody};
use std::collections::HashMap;

const CHEBYSHEV_DEGREE: usize = 17;
const GRANULE_DAYS: f64 = 32.0;
const N_SAMPLES: usize = 25;
const J2000_EPOCH: f64 = 2451545.0;
const MAGIC_HEADER: [u8; 4] = [0xCF, 0x86, 0x01, 0x00];

fn parent_of(body: i32) -> Option<i32> {
    match body {
        301 => Some(399),
        501 | 502 | 503 | 504 => Some(5),
        602 | 603 | 604 | 605 | 606 => Some(6),
        801 => Some(8),
        401 | 402 => Some(4),
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
    let (cx, cy, cz) = match solve_normal_equations(&ata, &atx, &aty, &atz) {
        Some(c) => c,
        None => return None,
    };
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

fn state_ssb(spk: &SpkFile, target: i32, et: f64) -> Option<[f64; 6]> {
    if let Ok(s) = spk.state(target, 0, et) {
        return Some(s);
    }
    let parent = parent_of(target)?;
    let moon_state = spk.state(target, parent, et).ok()?;
    let planet_state = state_ssb(spk, parent, et)?;
    Some([
        moon_state[0] + planet_state[0],
        moon_state[1] + planet_state[1],
        moon_state[2] + planet_state[2],
        moon_state[3] + planet_state[3],
        moon_state[4] + planet_state[4],
        moon_state[5] + planet_state[5],
    ])
}

fn compute_rotation_matrix(wgccre: &PckBody, jd: f64) -> Option<[f64; 9]> {
    let tc = (jd - J2000_EPOCH) / 36525.0;
    let a = wgccre.pole_ra_at(tc)?.to_radians();
    let d = wgccre.pole_dec_at(tc)?.to_radians();
    let w = (wgccre.pm_at(jd - J2000_EPOCH)? - wgccre.pole_ra_at(tc)?).to_radians();
    let (sa, ca) = a.sin_cos();
    let (sd, cd) = d.sin_cos();
    let (sw, cw) = w.sin_cos();
    let xt_target = cw * ca - sw * sa * cd;
    let yt_target = cw * sa + sw * ca * cd;
    let zt_target = sw * sd;
    let xt_up = -sw * ca - cw * sa * cd;
    let yt_up = -sw * sa + cw * ca * cd;
    let zt_up = cw * sd;
    let xt_east = -sa * sd;
    let yt_east = ca * sd;
    let zt_east = -cd;
    Some([
        xt_target, yt_target, zt_target, xt_east, yt_east, zt_east, xt_up, yt_up, zt_up,
    ])
}

fn extract_granules(
    spk: &SpkFile,
    target: i32,
    wgccre: &PckBody,
) -> (
    Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)>,
    Vec<(f64, [f64; 9])>,
) {
    let mut granules = Vec::new();
    let mut rotations = Vec::new();
    let segments = spk.segments();
    let relevant: Vec<_> = segments
        .iter()
        .filter(|s| s.target == target && s.data_type == 2)
        .collect();
    if relevant.is_empty() {
        return (granules, rotations);
    }
    let mut min_et = f64::MAX;
    let mut max_et = f64::MIN;
    for seg in &relevant {
        if seg.start_et < min_et {
            min_et = seg.start_et;
        }
        if seg.end_et > max_et {
            max_et = seg.end_et;
        }
    }
    let granule_half_sec = GRANULE_DAYS * 86400.0 / 2.0;
    let n_granules = ((max_et - min_et) / (GRANULE_DAYS * 86400.0)).ceil() as usize;
    for i in 0..n_granules {
        let mid_et = min_et + (i as f64 + 0.5) * GRANULE_DAYS * 86400.0;
        let mid_jd = mid_et / 86400.0 + J2000_EPOCH;
        let half_jd = GRANULE_DAYS / 2.0;
        let cheb_nodes = chebyshev_nodes(N_SAMPLES);
        let mut samples_x = Vec::with_capacity(N_SAMPLES);
        let mut samples_y = Vec::with_capacity(N_SAMPLES);
        let mut samples_z = Vec::with_capacity(N_SAMPLES);
        let mut valid = true;
        for tau in &cheb_nodes {
            let et = mid_et + tau * granule_half_sec;
            match state_ssb(spk, target, et) {
                Some([x, y, z, _, _, _]) => {
                    samples_x.push(x * 1000.0);
                    samples_y.push(y * 1000.0);
                    samples_z.push(z * 1000.0);
                }
                None => {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            continue;
        }
        let combined: Vec<(f64, f64, f64)> = (0..N_SAMPLES)
            .map(|k| (samples_x[k], samples_y[k], samples_z[k]))
            .collect();
        if let Some((cx, cy, cz)) = chebyshev_fit(&combined, CHEBYSHEV_DEGREE) {
            granules.push((mid_jd, half_jd, cx, cy, cz));
        }
        if let Some(rot_m) = compute_rotation_matrix(wgccre, mid_jd) {
            rotations.push((mid_jd, rot_m));
        }
    }
    (granules, rotations)
}

fn write_binary(
    path: &str,
    body_name: &str,
    granules: &[(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    rotations: &[(f64, [f64; 9])],
    wgccre: &PckBody,
) {
    let mut n_sections: u32 = 2;
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
        buf.extend_from_slice(&12u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let params: [f64; 12] = [
            neutral(wgccre.pole_ra_deg),
            neutral(wgccre.pole_ra_rate_deg_per_century),
            neutral(wgccre.pole_dec_deg),
            neutral(wgccre.pole_dec_rate_deg_per_century),
            neutral(wgccre.pm_deg),
            neutral(wgccre.pm_rate_deg_per_day),
            neutral(wgccre.radii_m.map(|r| r[0])),
            neutral(wgccre.radii_m.map(|r| r[1])),
            neutral(wgccre.radii_m.map(|r| r[2])),
            neutral(wgccre.j2),
            neutral(wgccre.j4),
            neutral(wgccre.gm_m3_s2),
        ];
        for &p in &params {
            buf.extend_from_slice(&p.to_le_bytes());
        }
    }
    {
        let section_stype: u32 = 2;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&17u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let kernel_params: [f64; 5] = [0.0; 5];
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
            "  {}: {} granules, {} rotations, {} B",
            body_name,
            granules.len(),
            rotations.len(),
            buf.len()
        );
    }
}

fn body_id_to_name(id: i32) -> &'static str {
    match id {
        10 => "sun",
        1 => "mercury",
        2 => "venus",
        399 => "earth",
        301 => "moon",
        4 => "mars",
        5 => "jupiter",
        6 => "saturn",
        7 => "uranus",
        8 => "neptune",
        9 => "pluto",
        501 => "io",
        502 => "europa",
        503 => "ganymede",
        504 => "callisto",
        602 => "enceladus",
        603 => "rhea",
        604 => "dione",
        605 => "tethys",
        606 => "titan",
        801 => "triton",
        401 => "phobos",
        402 => "deimos",
        _ => "unknown",
    }
}

fn pck_id_of(target: i32) -> i32 {
    match target {
        1 => 199,
        2 => 299,
        4 => 499,
        5 => 599,
        6 => 699,
        7 => 799,
        8 => 899,
        9 => 999,
        other => other,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: ephemeris_compiler <kernel_path>... --gm <gm_de440.tpc> --pck <pck00010.tpc>"
        );
        eprintln!("output: ephemeris_<body>.bin in current directory");
        std::process::exit(1);
    }
    let mut kernel_paths: Vec<String> = Vec::new();
    let mut gm_path: Option<String> = None;
    let mut pck_paths: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--gm" => {
                gm_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--pck" => {
                if let Some(p) = args.get(i + 1) {
                    pck_paths.push(p.clone());
                }
                i += 1;
            }
            other => kernel_paths.push(other.to_string()),
        }
        i += 1;
    }
    if kernel_paths.is_empty() {
        eprintln!("no kernel paths given");
        std::process::exit(1);
    }
    let gm_text = gm_path.and_then(|p| std::fs::read_to_string(p).ok());
    let mut pck_merged = String::new();
    for p in &pck_paths {
        if let Ok(t) = std::fs::read_to_string(p) {
            pck_merged.push_str(&t);
            pck_merged.push('\n');
        }
    }
    let pck_text = if pck_merged.is_empty() {
        None
    } else {
        Some(pck_merged)
    };
    let pck_bodies: HashMap<i32, PckBody> =
        omegaflow::pck::parse(gm_text.as_deref(), pck_text.as_deref());
    let all_target_ids: [i32; 23] = [
        10, 1, 2, 399, 301, 4, 5, 6, 7, 8, 9, 501, 502, 503, 504, 602, 603, 604, 605, 606, 801,
        401, 402,
    ];
    let mut all_granules: Vec<Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)>> =
        vec![Vec::new(); all_target_ids.len()];
    let mut all_rotations: Vec<Vec<(f64, [f64; 9])>> = vec![Vec::new(); all_target_ids.len()];

    for kernel_path in &kernel_paths {
        let spk = match SpkFile::open(kernel_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("open {}: {}", kernel_path, e);
                std::process::exit(1);
            }
        };
        for (idx, &target_id) in all_target_ids.iter().enumerate() {
            let has_coverage = spk.segments().iter().any(|s| s.target == target_id);
            if !has_coverage {
                continue;
            }
            let wgccre = match pck_bodies.get(&pck_id_of(target_id)) {
                Some(w) => w,
                None => continue,
            };
            let (granules, rotations) = extract_granules(&spk, target_id, &wgccre);
            all_granules[idx].extend(granules);
            all_rotations[idx].extend(rotations);
        }
    }
    for (idx, &target_id) in all_target_ids.iter().enumerate() {
        let body_name = body_id_to_name(target_id);
        let mut granules = core::mem::take(&mut all_granules[idx]);
        let mut rotations = core::mem::take(&mut all_rotations[idx]);
        let wgccre = match pck_bodies.get(&pck_id_of(target_id)) {
            Some(w) => w,
            None => {
                eprintln!("  SKIP {}: no PCK params", body_name);
                continue;
            }
        };
        if granules.is_empty() && target_id != 10 {
            eprintln!("  SKIP {}: no granules in any kernel", body_name);
            continue;
        }
        granules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        rotations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let path = format!("ephemeris_{}.bin", body_name);
        write_binary(&path, body_name, &granules, &rotations, &wgccre);
    }
}
