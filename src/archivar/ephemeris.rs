use std::collections::HashMap;

use crate::bpc::BpcFile;
use crate::bsp_reader::spk::SpkFile;
use crate::fk::FkFile;
use crate::least_squares::solve_normal_equations;
use crate::mat::matmul;
use crate::pck::PckBody;

pub const CHEBYSHEV_DEGREE: usize = 17;
pub const NUT_DEGREE: usize = 11;
pub const GRANULE_DAYS: f64 = 32.0;
pub const ASTEROID_GRANULE_DAYS: f64 = 256.0;
pub const N_SAMPLES: usize = 25;
pub const J2000_EPOCH: f64 = 2451545.0;
pub const MAGIC_HEADER: [u8; 4] = [0xCF, 0x86, 0x02, 0x00];
const NAIF_ID_TABLE: &str = include_str!("kernels/naif_body_ids.tsv");

pub struct BodyId {
    pub name: String,
    pub parent: Option<i32>,
}

pub fn body_table() -> HashMap<i32, BodyId> {
    let mut table = HashMap::new();
    for line in NAIF_ID_TABLE.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let id: i32 = match parts.next().and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => continue,
        };
        let name = match parts.next() {
            Some(n) => n.to_string(),
            None => continue,
        };
        let parent = parts.next().and_then(|p| p.parse().ok());
        table.insert(id, BodyId { name, parent });
    }
    table
}

pub fn parent_of(body: i32) -> Option<i32> {
    body_table().get(&body).and_then(|b| b.parent)
}

pub fn pck_id_of(target: i32) -> i32 {
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

pub fn chebyshev_nodes(n: usize) -> Vec<f64> {
    let mut nodes = Vec::with_capacity(n);
    for k in 0..n {
        nodes.push(((std::f64::consts::PI * (n as f64 - k as f64 - 0.5)) / n as f64).cos());
    }
    nodes
}

pub fn chebyshev_polys(n: usize, x: f64) -> Vec<f64> {
    let mut t = vec![1.0, x];
    for k in 2..n {
        let next = 2.0 * x * t[k - 1] - t[k - 2];
        t.push(next);
    }
    t.truncate(n);
    t
}

pub fn chebyshev_fit(
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

pub fn state_ssb_multi(kernels: &[SpkFile], target: i32, et: f64) -> Option<[f64; 6]> {
    for spk in kernels {
        if let Ok(s) = spk.state(target, 0, et) {
            return Some(s);
        }
    }
    let parent = parent_of(target)?;
    let mut moon_state = None;
    for spk in kernels {
        if let Ok(s) = spk.state(target, parent, et) {
            moon_state = Some(s);
            break;
        }
    }
    let moon_state = moon_state?;
    let planet_state = state_ssb_multi(kernels, parent, et)?;
    Some([
        moon_state[0] + planet_state[0],
        moon_state[1] + planet_state[1],
        moon_state[2] + planet_state[2],
        moon_state[3] + planet_state[3],
        moon_state[4] + planet_state[4],
        moon_state[5] + planet_state[5],
    ])
}

pub fn rotation_matrix_from_angles(ra_deg: f64, dec_deg: f64, pm_deg: f64) -> [f64; 9] {
    let a = ra_deg.to_radians();
    let d = dec_deg.to_radians();
    let w = (pm_deg - ra_deg).to_radians();
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
    [
        xt_target, yt_target, zt_target, xt_east, yt_east, zt_east, xt_up, yt_up, zt_up,
    ]
}

pub fn libration_matrix(phi_deg: f64, theta_rad: f64, psi_rad: f64) -> [f64; 9] {
    let (sp, cp) = phi_deg.to_radians().sin_cos();
    let (st, ct) = theta_rad.sin_cos();
    let (ss, cs) = psi_rad.sin_cos();
    let r3p = [cp, sp, 0.0, -sp, cp, 0.0, 0.0, 0.0, 1.0];
    let r1t = [1.0, 0.0, 0.0, 0.0, ct, st, 0.0, -st, ct];
    let r3s = [cs, ss, 0.0, -ss, cs, 0.0, 0.0, 0.0, 1.0];
    matmul(&r3s, &matmul(&r1t, &r3p))
}

pub fn iau_angles_from_matrix(m: [f64; 9]) -> (f64, f64, f64) {
    let p = [m[6], m[7], m[8]];
    let ra = p[1].atan2(p[0]).to_degrees();
    let dec = p[2].asin().to_degrees();
    let n_norm = (p[0] * p[0] + p[1] * p[1]).sqrt();
    let n = [-p[1] / n_norm, p[0] / n_norm, 0.0];
    let t = [m[0], m[1], m[2]];
    let t_dot_p = t[0] * p[0] + t[1] * p[1] + t[2] * p[2];
    let tp = [
        t[0] - t_dot_p * p[0],
        t[1] - t_dot_p * p[1],
        t[2] - t_dot_p * p[2],
    ];
    let n_cross_tp = [
        n[1] * tp[2] - n[2] * tp[1],
        n[2] * tp[0] - n[0] * tp[2],
        n[0] * tp[1] - n[1] * tp[0],
    ];
    let w_num = n_cross_tp[0] * p[0] + n_cross_tp[1] * p[1] + n_cross_tp[2] * p[2];
    let w_den = n[0] * tp[0] + n[1] * tp[1] + n[2] * tp[2];
    (ra, dec, w_num.atan2(w_den).to_degrees())
}

pub fn full_orientation(
    wgccre: &PckBody,
    bpc_files: &[BpcFile],
    fk: &FkFile,
    body_id: i32,
    jd: f64,
) -> Option<(f64, f64, f64)> {
    let et = (jd - J2000_EPOCH) * 86400.0;
    for pa_frame in fk
        .frames
        .iter()
        .filter(|f| f.class == Some(2) && f.center == Some(body_id))
    {
        for bpc in bpc_files {
            let Some((phi, theta, psi)) = bpc.orient(pa_frame.id, 1, et) else {
                continue;
            };
            let m_pa = libration_matrix(phi, theta, psi);
            let m_me = match fk.tkframe_child_of(&pa_frame.name) {
                Some(child) => match fk.tkframe_rotation(child.id) {
                    Some((rot, _)) => matmul(&rot, &m_pa),
                    None => m_pa,
                },
                None => m_pa,
            };
            let (mut ra2, mut dec2, mut w2) = iau_angles_from_matrix(m_me);
            if let Some((_, lin_dec, _)) = linear_orientation(wgccre, jd) {
                if (dec2 - lin_dec).abs() > 90.0 {
                    ra2 += 180.0;
                    dec2 = -dec2;
                    w2 += 180.0;
                }
            }
            return Some((ra2, dec2, w2));
        }
    }
    let tc = (jd - J2000_EPOCH) / 36525.0;
    Some((
        wgccre.pole_ra_at(tc)?,
        wgccre.pole_dec_at(tc)?,
        wgccre.pm_at(jd - J2000_EPOCH)?,
    ))
}

pub fn linear_orientation(wgccre: &PckBody, jd: f64) -> Option<(f64, f64, f64)> {
    let tc = (jd - J2000_EPOCH) / 36525.0;
    Some((
        wgccre.pole_ra_deg? + wgccre.pole_ra_rate_deg_per_century? * tc,
        wgccre.pole_dec_deg? + wgccre.pole_dec_rate_deg_per_century? * tc,
        wgccre.pm_deg? + wgccre.pm_rate_deg_per_day? * (jd - J2000_EPOCH),
    ))
}

pub fn nutation_delta_fit(
    wgccre: &PckBody,
    bpc_files: &[BpcFile],
    fk: &FkFile,
    body_id: i32,
    mid_jd: f64,
    half_jd: f64,
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let nodes = chebyshev_nodes(N_SAMPLES);
    let mut samples: Vec<(f64, f64, f64)> = Vec::with_capacity(N_SAMPLES);
    for tau in &nodes {
        let jd = mid_jd + tau * half_jd;
        let full = full_orientation(wgccre, bpc_files, fk, body_id, jd)?;
        let lin = linear_orientation(wgccre, jd)?;
        samples.push((full.0 - lin.0, full.1 - lin.1, full.2 - lin.2));
    }
    let (ra_d, dec_d, pm_d) = (samples[0].0.abs(), samples[0].1.abs(), samples[0].2.abs());
    if ra_d < 1e-9 && dec_d < 1e-9 && pm_d < 1e-9 {
        return None;
    }
    chebyshev_fit(&samples, NUT_DEGREE)
}

pub fn extract_granules(
    spk: &SpkFile,
    all_kernels: &[SpkFile],
    target: i32,
    wgccre: &PckBody,
    bpc_files: &[BpcFile],
    fk: &FkFile,
    granule_days: f64,
) -> (
    Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)>,
    Vec<(f64, [f64; 9])>,
    Vec<(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)>,
) {
    let mut granules = Vec::new();
    let mut rotations = Vec::new();
    let mut nutation = Vec::new();
    let segments = spk.segments();
    let relevant: Vec<_> = segments
        .iter()
        .filter(|s| s.target == target && s.data_type == 2)
        .collect();
    if relevant.is_empty() {
        return (granules, rotations, nutation);
    }
    let pck_id = pck_id_of(target);
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
    let granule_half_sec = granule_days * 86400.0 / 2.0;
    let n_granules = ((max_et - min_et) / (granule_days * 86400.0)).ceil() as usize;
    for i in 0..n_granules {
        let mid_et = min_et + (i as f64 + 0.5) * granule_days * 86400.0;
        let mid_jd = mid_et / 86400.0 + J2000_EPOCH;
        let half_jd = granule_days / 2.0;
        let cheb_nodes = chebyshev_nodes(N_SAMPLES);
        let mut samples_x = Vec::with_capacity(N_SAMPLES);
        let mut samples_y = Vec::with_capacity(N_SAMPLES);
        let mut samples_z = Vec::with_capacity(N_SAMPLES);
        let mut valid = true;
        for tau in &cheb_nodes {
            let et = mid_et + tau * granule_half_sec;
            match state_ssb_multi(all_kernels, target, et) {
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
        if let Some((ra, dec, pm)) = full_orientation(wgccre, bpc_files, fk, pck_id, mid_jd) {
            rotations.push((mid_jd, rotation_matrix_from_angles(ra, dec, pm)));
        }
        if let Some((nra, ndec, npm)) =
            nutation_delta_fit(wgccre, bpc_files, fk, pck_id, mid_jd, half_jd)
        {
            nutation.push((mid_jd, half_jd, nra, ndec, npm));
        }
    }
    (granules, rotations, nutation)
}

pub fn write_binary(
    path: &str,
    body_name: &str,
    granules: &[(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    rotations: &[(f64, [f64; 9])],
    nutation: &[(f64, f64, Vec<f64>, Vec<f64>, Vec<f64>)],
    wgccre: &PckBody,
    omega_g: Option<(f64, f64)>,
) -> bool {
    let mut n_sections: u32 = 3;
    if !rotations.is_empty() {
        n_sections += 1;
    }
    if !nutation.is_empty() {
        n_sections += 1;
    }
    if omega_g.is_some() {
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
        let slots: [Option<f64>; 12] = [
            wgccre.pole_ra_deg,
            wgccre.pole_ra_rate_deg_per_century,
            wgccre.pole_dec_deg,
            wgccre.pole_dec_rate_deg_per_century,
            wgccre.pm_deg,
            wgccre.pm_rate_deg_per_day,
            wgccre.radii_m.map(|r| r[0]),
            wgccre.radii_m.map(|r| r[1]),
            wgccre.radii_m.map(|r| r[2]),
            wgccre.j2,
            wgccre.j4,
            wgccre.gm_m3_s2,
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
            crate::media::medium_params_of(body_name).map_or([0.0; 5], |m| m.wire());
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
    if !nutation.is_empty() {
        let section_stype: u32 = 4;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&(nutation.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(NUT_DEGREE as u32).to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        for (t0, dt, cx, cy, cz) in nutation {
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
    }
    if let Some((omega_g_hz, sigma_hz)) = omega_g {
        let section_stype: u32 = 7;
        buf.extend_from_slice(&section_stype.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&omega_g_hz.to_le_bytes());
        buf.extend_from_slice(&sigma_hz.to_le_bytes());
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
        buf.extend_from_slice(&0.0_f64.to_le_bytes());
    }
    match std::fs::write(path, &buf) {
        Ok(()) => {
            eprintln!(
                "  {}: {} granules, {} rotations, {} nutation, {} B, omega_g {:?}",
                body_name,
                granules.len(),
                rotations.len(),
                nutation.len(),
                buf.len(),
                omega_g.map(|(v, _)| v)
            );
            true
        }
        Err(e) => {
            eprintln!("write {}: {}", path, e);
            false
        }
    }
}
