use std::collections::{HashMap, HashSet};

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
const NAIF_SPACECRAFT_ID_TABLE: &str = include_str!("kernels/naif_spacecraft_ids.tsv");

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

pub fn spacecraft_table() -> HashMap<i32, BodyId> {
    let mut table = HashMap::new();
    for line in NAIF_SPACECRAFT_ID_TABLE.lines() {
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
    let mut visited = HashSet::new();
    visited.insert(target);
    resolve_to_ssb(kernels, target, et, &visited, 0)
}

fn resolve_to_ssb(
    kernels: &[SpkFile],
    cur: i32,
    et: f64,
    visited: &HashSet<i32>,
    depth: usize,
) -> Option<[f64; 6]> {
    if cur == 0 {
        return Some([0.0; 6]);
    }
    if depth >= 32 {
        return None;
    }
    if let Some(s) = state_wrt(kernels, cur, 0, et) {
        return Some(s);
    }
    let mut candidates: Vec<i32> = Vec::new();
    for spk in kernels {
        for seg in spk.segments() {
            if seg.target == cur
                && matches!(seg.data_type, 2 | 3 | 9 | 13 | 20)
                && et >= seg.start_et
                && et <= seg.end_et
            {
                if !candidates.contains(&seg.center) {
                    candidates.push(seg.center);
                }
            }
        }
    }
    for c in candidates {
        if visited.contains(&c) {
            continue;
        }
        let delta = match state_wrt(kernels, cur, c, et) {
            Some(d) => d,
            None => continue,
        };
        let mut next_visited = visited.clone();
        next_visited.insert(c);
        if let Some(rest) = resolve_to_ssb(kernels, c, et, &next_visited, depth + 1) {
            return Some(add_state(delta, rest));
        }
    }
    None
}

fn state_wrt(kernels: &[SpkFile], target: i32, center: i32, et: f64) -> Option<[f64; 6]> {
    for spk in kernels {
        if let Ok(s) = spk.state(target, center, et) {
            return Some(s);
        }
    }
    None
}

fn add_state(a: [f64; 6], b: [f64; 6]) -> [f64; 6] {
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
        a[3] + b[3],
        a[4] + b[4],
        a[5] + b[5],
    ]
}

pub fn rotation_matrix_from_angles(ra_deg: f64, dec_deg: f64, pm_deg: f64) -> [f64; 9] {
    let a = (90.0 + ra_deg).to_radians();
    let d = (90.0 - dec_deg).to_radians();
    let w = pm_deg.to_radians();
    let (sa, ca) = a.sin_cos();
    let (sd, cd) = d.sin_cos();
    let (sw, cw) = w.sin_cos();
    let rz_a = [ca, -sa, 0.0, sa, ca, 0.0, 0.0, 0.0, 1.0];
    let rx_d = [1.0, 0.0, 0.0, 0.0, cd, -sd, 0.0, sd, cd];
    let rz_w = [cw, -sw, 0.0, sw, cw, 0.0, 0.0, 0.0, 1.0];
    matmul(&rz_a, &matmul(&rx_d, &rz_w))
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
        .filter(|s| s.target == target && matches!(s.data_type, 2 | 3 | 9 | 13 | 20))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::bsp_reader::daf::{DOUBLE_BYTES, DafFile, RECORD_BYTES};
    use crate::archivar::bsp_reader::spk::SpkFile;

    const DATA_START_ADDR: u32 = 3 * (RECORD_BYTES as u32) / (DOUBLE_BYTES as u32) + 1;
    const SUMMARY_SIZE: usize = 2 * DOUBLE_BYTES + 6 * 4;

    struct SegSpec {
        target: i32,
        center: i32,
        state: [f64; 6],
    }

    fn synthetic_spk(segs: &[SegSpec]) -> SpkFile {
        let mut addrs = Vec::with_capacity(segs.len());
        let mut cursor = DATA_START_ADDR;
        for _ in segs {
            let start = cursor;
            let end = start + 8;
            addrs.push((start, end));
            cursor = end + 1;
        }
        let end_addr = cursor - 1;
        let mut buf = vec![0u8; end_addr as usize * DOUBLE_BYTES];

        buf[0..8].copy_from_slice(b"DAF/SPK ");
        let nd: u32 = 2;
        let ni: u32 = 6;
        buf[8..12].copy_from_slice(&nd.to_le_bytes());
        buf[12..16].copy_from_slice(&ni.to_le_bytes());
        let fward: u32 = 2;
        buf[76..80].copy_from_slice(&fward.to_le_bytes());
        buf[88..96].copy_from_slice(b"LTL-IEEE");

        let sum_rec = RECORD_BYTES;
        let nsum: f64 = segs.len() as f64;
        buf[sum_rec + 16..sum_rec + 24].copy_from_slice(&nsum.to_le_bytes());
        for (i, seg) in segs.iter().enumerate() {
            let soff = sum_rec + 24 + i * SUMMARY_SIZE;
            buf[soff..soff + 8].copy_from_slice(&0.0_f64.to_le_bytes());
            buf[soff + 8..soff + 16].copy_from_slice(&1.0e12_f64.to_le_bytes());
            let (sa, ea) = addrs[i];
            let ints: [i32; 6] = [seg.target, seg.center, 1, 9, sa as i32, ea as i32];
            for (k, v) in ints.iter().enumerate() {
                let off = soff + 16 + k * 4;
                buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
            }
        }

        let name_rec = 2 * RECORD_BYTES;
        for (i, _) in segs.iter().enumerate() {
            let noff = name_rec + i * SUMMARY_SIZE;
            let name = format!("S{i}");
            buf[noff..noff + name.len()].copy_from_slice(name.as_bytes());
        }

        for (i, seg) in segs.iter().enumerate() {
            let (sa, _) = addrs[i];
            let base = (sa as usize - 1) * DOUBLE_BYTES;
            for k in 0..6 {
                let off = base + k * DOUBLE_BYTES;
                buf[off..off + DOUBLE_BYTES].copy_from_slice(&seg.state[k].to_le_bytes());
            }
            let eoff = base + 6 * DOUBLE_BYTES;
            buf[eoff..eoff + DOUBLE_BYTES].copy_from_slice(&0.0_f64.to_le_bytes());
            let toff = base + 7 * DOUBLE_BYTES;
            buf[toff..toff + DOUBLE_BYTES].copy_from_slice(&0.0_f64.to_le_bytes());
            buf[toff + DOUBLE_BYTES..toff + 2 * DOUBLE_BYTES]
                .copy_from_slice(&1.0_f64.to_le_bytes());
        }

        let daf = DafFile::from_data(buf).expect("synthetic DAF parses");
        SpkFile::from_daf(daf).expect("synthetic SPK parses")
    }

    #[test]
    fn chain_moon_planet_ssb() {
        let spk = synthetic_spk(&[
            SegSpec {
                target: 301,
                center: 399,
                state: [1.0, 2.0, 3.0, 0.0, 0.0, 0.0],
            },
            SegSpec {
                target: 399,
                center: 0,
                state: [10.0, 20.0, 30.0, 0.0, 0.0, 0.0],
            },
        ]);
        let kernels = [spk];
        let s = state_ssb_multi(&kernels, 301, 100.0).expect("resolves");
        assert_eq!(s, [11.0, 22.0, 33.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn juice_cross_kernel_chain() {
        let a = synthetic_spk(&[
            SegSpec {
                target: -28,
                center: 599,
                state: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            SegSpec {
                target: -28,
                center: 10,
                state: [7.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
        ]);
        let b = synthetic_spk(&[
            SegSpec {
                target: 599,
                center: 0,
                state: [10.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            SegSpec {
                target: 10,
                center: 0,
                state: [70.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
        ]);
        let kernels = [a, b];
        let s = state_ssb_multi(&kernels, -28, 100.0).expect("resolves");
        assert_eq!(s, [11.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn flyby_overlap_backtracking() {
        let a = synthetic_spk(&[
            SegSpec {
                target: -28,
                center: 7,
                state: [9.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            SegSpec {
                target: -28,
                center: 599,
                state: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
        ]);
        let b = synthetic_spk(&[SegSpec {
            target: 599,
            center: 0,
            state: [10.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }]);
        let kernels = [a, b];
        let s = state_ssb_multi(&kernels, -28, 100.0).expect("resolves");
        assert_eq!(s, [11.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
}
