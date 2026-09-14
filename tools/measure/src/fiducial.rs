use omegaflow::matfile::{MatArray, MatData, MatField, parse_mat};

#[derive(Clone, Copy, Debug)]
pub struct RigidTransform {
    pub rotation: [[f64; 3]; 3],
    pub translation: [f64; 3],
    pub scale: f64,
}

pub fn apply(transform: &RigidTransform, point: [f64; 3]) -> [f64; 3] {
    let mut out = [0.0; 3];
    for i in 0..3 {
        let mut acc = 0.0;
        for j in 0..3 {
            acc += transform.rotation[i][j] * point[j];
        }
        out[i] = transform.scale * acc + transform.translation[i];
    }
    out
}

fn centroids(from: &[[f64; 3]], to: &[[f64; 3]]) -> ([f64; 3], [f64; 3]) {
    let mut fc = [0.0; 3];
    let mut tc = [0.0; 3];
    for i in 0..from.len() {
        for j in 0..3 {
            fc[j] += from[i][j];
            tc[j] += to[i][j];
        }
    }
    let n = from.len() as f64;
    for j in 0..3 {
        fc[j] /= n;
        tc[j] /= n;
    }
    (fc, tc)
}

pub fn umeyama(from: &[[f64; 3]], to: &[[f64; 3]]) -> Option<RigidTransform> {
    let n = from.len();
    if n < 3 || n != to.len() {
        return None;
    }
    for i in 0..n {
        for j in 0..3 {
            if !from[i][j].is_finite() || !to[i][j].is_finite() {
                return None;
            }
        }
    }
    let (fc, tc) = centroids(from, to);
    let mut h = [[0.0; 3]; 3];
    let mut var_from = 0.0;
    for i in 0..n {
        let af = [from[i][0] - fc[0], from[i][1] - fc[1], from[i][2] - fc[2]];
        let at = [to[i][0] - tc[0], to[i][1] - tc[1], to[i][2] - tc[2]];
        var_from += af[0] * af[0] + af[1] * af[1] + af[2] * af[2];
        for r in 0..3 {
            for c in 0..3 {
                h[r][c] += af[r] * at[c];
            }
        }
    }
    if var_from <= 0.0 || !var_from.is_finite() {
        return None;
    }
    let (u, s, v) = svd3(h)?;
    if s[0] <= 0.0 || s[1] <= s[0] * 1e-12 {
        return None;
    }
    let d = det3_cols(u[0], u[1], u[2]) * det3_cols(v[0], v[1], v[2]);
    let dsign = [1.0, 1.0, d];
    let mut rotation = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut acc = 0.0;
            for k in 0..3 {
                acc += dsign[k] * v[k][i] * u[k][j];
            }
            rotation[i][j] = acc;
        }
    }
    let scale = (s[0] + s[1] + d * s[2]) / var_from;
    if !scale.is_finite() {
        return None;
    }
    let mut translation = [0.0; 3];
    for i in 0..3 {
        let mut rsum = 0.0;
        for j in 0..3 {
            rsum += rotation[i][j] * fc[j];
        }
        translation[i] = tc[i] - scale * rsum;
    }
    Some(RigidTransform {
        rotation,
        translation,
        scale,
    })
}

pub fn rigid_coregister(montage: &[[f64; 3]], mni: &[[f64; 3]]) -> Option<(RigidTransform, f64)> {
    let fitted = umeyama(montage, mni)?;
    let (fc, tc) = centroids(montage, mni);
    let mut translation = [0.0; 3];
    for i in 0..3 {
        let mut rsum = 0.0;
        for j in 0..3 {
            rsum += fitted.rotation[i][j] * fc[j];
        }
        translation[i] = tc[i] - rsum;
    }
    Some((
        RigidTransform {
            rotation: fitted.rotation,
            translation,
            scale: 1.0,
        },
        fitted.scale,
    ))
}

fn jacobi_symmetric(a0: [[f64; 3]; 3]) -> Option<(Vec<f64>, Vec<[f64; 3]>)> {
    let mut a = a0;
    let mut v = [[1.0f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    for _ in 0..40 {
        let mut p = 0usize;
        let mut q = 1usize;
        let mut max = a[0][1].abs();
        if a[0][2].abs() > max {
            max = a[0][2].abs();
            p = 0;
            q = 2;
        }
        if a[1][2].abs() > max {
            max = a[1][2].abs();
            p = 1;
            q = 2;
        }
        if max < 1e-14 {
            break;
        }
        let app = a[p][p];
        let aqq = a[q][q];
        let apq = a[p][q];
        let (c, s) = if (app - aqq).abs() < 1e-14 {
            (
                std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
            )
        } else {
            let tau = (aqq - app) / (2.0 * apq);
            let t = if tau < 0.0 {
                -(tau.abs() + (1.0 + tau * tau).sqrt()).recip()
            } else {
                (tau.abs() + (1.0 + tau * tau).sqrt()).recip()
            };
            let c = 1.0 / (1.0 + t * t).sqrt();
            (c, c * t)
        };
        for k in 0..3 {
            if k != p && k != q {
                let akp = a[k][p];
                let akq = a[k][q];
                a[k][p] = c * akp - s * akq;
                a[k][q] = s * akp + c * akq;
                a[p][k] = a[k][p];
                a[q][k] = a[k][q];
            }
        }
        a[p][p] = c * c * app - 2.0 * s * c * apq + s * s * aqq;
        a[q][q] = s * s * app + 2.0 * s * c * apq + c * c * aqq;
        a[p][q] = 0.0;
        a[q][p] = 0.0;
        for k in 0..3 {
            let vkp = v[k][p];
            let vkq = v[k][q];
            v[k][p] = c * vkp - s * vkq;
            v[k][q] = s * vkp + c * vkq;
        }
    }
    let eig: Vec<f64> = (0..3).map(|i| a[i][i]).collect();
    if eig.iter().any(|e| !e.is_finite()) {
        return None;
    }
    let mut order = [0usize, 1, 2];
    order.sort_by(|&x, &y| {
        eig[y]
            .partial_cmp(&eig[x])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let eigs = vec![eig[order[0]], eig[order[1]], eig[order[2]]];
    let cols = vec![
        [v[0][order[0]], v[1][order[0]], v[2][order[0]]],
        [v[0][order[1]], v[1][order[1]], v[2][order[1]]],
        [v[0][order[2]], v[1][order[2]], v[2][order[2]]],
    ];
    Some((eigs, cols))
}

fn svd3(h: [[f64; 3]; 3]) -> Option<(Vec<[f64; 3]>, Vec<f64>, Vec<[f64; 3]>)> {
    let mut ata = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s = 0.0;
            for k in 0..3 {
                s += h[k][i] * h[k][j];
            }
            ata[i][j] = s;
        }
    }
    let (ev, vcols) = jacobi_symmetric(ata)?;
    let mut s = [0.0f64; 3];
    for k in 0..3 {
        s[k] = if ev[k] > 0.0 { ev[k].sqrt() } else { 0.0 };
    }
    let eps = if s[0] > 0.0 { s[0] * 1e-12 } else { 0.0 };
    let mut ucols: Vec<[f64; 3]> = vec![[0.0; 3]; 3];
    let mut rank = 0usize;
    for k in 0..3 {
        if s[k] > eps {
            let vk = vcols[k];
            let mut hv = [0.0; 3];
            for i in 0..3 {
                let mut acc = 0.0;
                for j in 0..3 {
                    acc += h[i][j] * vk[j];
                }
                hv[i] = acc;
            }
            for i in 0..3 {
                ucols[k][i] = hv[i] / s[k];
            }
            rank += 1;
        } else {
            ucols[k] = [0.0; 3];
        }
    }
    if rank < 3 {
        let mut basis: Vec<[f64; 3]> = ucols[..rank].to_vec();
        for cand in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] {
            if basis.len() >= 3 {
                break;
            }
            let mut v = cand;
            for b in &basis {
                let dot = v[0] * b[0] + v[1] * b[1] + v[2] * b[2];
                for i in 0..3 {
                    v[i] -= dot * b[i];
                }
            }
            let norm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if norm > 1e-9 {
                for i in 0..3 {
                    v[i] /= norm;
                }
                basis.push(v);
            }
        }
        for k in rank..3 {
            ucols[k] = basis[k];
        }
    }
    Some((ucols, s.to_vec(), vcols))
}

fn det3_cols(c0: [f64; 3], c1: [f64; 3], c2: [f64; 3]) -> f64 {
    c0[0] * (c1[1] * c2[2] - c1[2] * c2[1]) - c0[1] * (c1[0] * c2[2] - c1[2] * c2[0])
        + c0[2] * (c1[0] * c2[1] - c1[1] * c2[0])
}

#[derive(Clone, Debug)]
pub struct BemShell {
    pub vertices: Vec<[f64; 3]>,
    pub faces: Vec<[usize; 3]>,
}

#[derive(Clone, Debug)]
pub struct BemHeadModel {
    pub shells: Vec<BemShell>,
    pub unit: Option<String>,
}

fn field<'a>(fields: &'a [MatField], name: &str) -> Option<&'a MatField> {
    fields.iter().find(|f| f.name == name)
}

fn double_vectors(m: &MatArray) -> Option<Vec<[f64; 3]>> {
    let MatData::Double(d) = &m.data else {
        return None;
    };
    if m.dims.len() != 2 || m.dims[1] != 3 {
        return None;
    }
    let n = m.dims[0];
    if d.len() != n * 3 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let v = [d[i], d[n + i], d[2 * n + i]];
        if v.iter().any(|x| !x.is_finite()) {
            return None;
        }
        out.push(v);
    }
    Some(out)
}

fn face_indices(m: &MatArray) -> Option<Vec<[usize; 3]>> {
    let MatData::Double(d) = &m.data else {
        return None;
    };
    if m.dims.len() != 2 || m.dims[1] != 3 {
        return None;
    }
    let n = m.dims[0];
    if d.len() != n * 3 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let v = [d[i], d[n + i], d[2 * n + i]];
        if v.iter().any(|x| *x < 1.0 || x.fract() != 0.0) {
            return None;
        }
        out.push([
            (v[0] as usize) - 1,
            (v[1] as usize) - 1,
            (v[2] as usize) - 1,
        ]);
    }
    Some(out)
}

fn shell_from_struct(fields: &[MatField]) -> Option<BemShell> {
    let vertices = double_vectors(field(fields, "pos")?.values.first()?)?;
    let faces = face_indices(field(fields, "tri")?.values.first()?)?;
    if faces.iter().any(|f| f.iter().any(|&i| i >= vertices.len())) {
        return None;
    }
    Some(BemShell { vertices, faces })
}

fn extract_shells(bnd: &MatData) -> Option<Vec<BemShell>> {
    match bnd {
        MatData::Struct(fields) => {
            let pos = field(fields, "pos")?;
            let tri = field(fields, "tri")?;
            if pos.values.is_empty() || pos.values.len() != tri.values.len() {
                return None;
            }
            pos.values
                .iter()
                .zip(tri.values.iter())
                .map(|(p, t)| {
                    let vertices = double_vectors(p)?;
                    let faces = face_indices(t)?;
                    if faces.iter().any(|f| f.iter().any(|&i| i >= vertices.len())) {
                        return None;
                    }
                    Some(BemShell { vertices, faces })
                })
                .collect()
        }
        MatData::Cell(cells) => cells
            .iter()
            .map(|cell| match &cell.data {
                MatData::Struct(fields) => shell_from_struct(fields),
                _ => None,
            })
            .collect(),
        _ => None,
    }
}

fn char_text(m: &MatArray) -> Option<String> {
    match &m.data {
        MatData::Char(c) => {
            let s = String::from_utf8_lossy(c)
                .trim_end_matches('\0')
                .trim()
                .to_string();
            if s.is_empty() { None } else { Some(s) }
        }
        _ => None,
    }
}

pub fn read_bem_mat(bytes: &[u8]) -> Option<BemHeadModel> {
    let arrays = parse_mat(bytes)?;
    let vol = arrays.iter().find(|a| a.name == "vol").or(arrays.first())?;
    let MatData::Struct(fields) = &vol.data else {
        return None;
    };
    let bnd = field(fields, "bnd")?;
    let shells = extract_shells(&bnd.values.first()?.data)?;
    if shells.is_empty() {
        return None;
    }
    let unit = field(fields, "unit").and_then(|f| char_text(f.values.first()?));
    Some(BemHeadModel { shells, unit })
}

fn chaninfo_array(arrays: &[MatArray]) -> Option<&MatArray> {
    if let Some(a) = arrays.iter().find(|a| a.name == "chaninfo") {
        return Some(a);
    }
    for a in arrays {
        if a.name == "EEG" {
            if let MatData::Struct(fields) = &a.data {
                if let Some(ci) = fields.iter().find(|f| f.name == "chaninfo") {
                    if let Some(first) = ci.values.first() {
                        return Some(first);
                    }
                }
            }
        }
    }
    None
}

pub fn montage_fiducials(bytes: &[u8]) -> Option<Vec<(Option<String>, [f64; 3])>> {
    let arrays = parse_mat(bytes)?;
    let chaninfo = chaninfo_array(&arrays)?;
    let MatData::Struct(fields) = &chaninfo.data else {
        return None;
    };
    let nodatchans = field(fields, "nodatchans")?.values.first()?;
    let MatData::Struct(nf) = &nodatchans.data else {
        return None;
    };
    let labels = field(nf, "labels")?;
    let types = field(nf, "type");
    let desc = field(nf, "description");
    let xs = field(nf, "X")?;
    let ys = field(nf, "Y")?;
    let zs = field(nf, "Z")?;
    let n = labels.values.len();
    if xs.values.len() != n || ys.values.len() != n || zs.values.len() != n {
        return None;
    }
    let scalar = |m: &MatArray| -> Option<f64> {
        match &m.data {
            MatData::Double(d) => d.first().copied().filter(|x| x.is_finite()),
            _ => None,
        }
    };
    let mut out = Vec::new();
    for i in 0..n {
        if types.and_then(|t| char_text(t.values.get(i)?)).as_deref() != Some("FID") {
            continue;
        }
        let (Some(x), Some(y), Some(z)) = (
            scalar(&xs.values[i]),
            scalar(&ys.values[i]),
            scalar(&zs.values[i]),
        ) else {
            continue;
        };
        let label = desc
            .and_then(|d| char_text(d.values.get(i)?))
            .or_else(|| char_text(labels.values.get(i)?));
        out.push((label, [x, y, z]));
    }
    if out.is_empty() { None } else { Some(out) }
}

#[derive(Clone, Copy, Debug)]
pub struct MniFiducials {
    pub lpa: Option<[f64; 3]>,
    pub rpa: Option<[f64; 3]>,
    pub nz: Option<[f64; 3]>,
    pub iz: Option<[f64; 3]>,
}

fn parse_xyz(line: &str) -> Option<[f64; 3]> {
    let mut it = line.split_whitespace();
    let x = it.next()?.parse::<f64>().ok()?;
    let y = it.next()?.parse::<f64>().ok()?;
    let z = it.next()?.parse::<f64>().ok()?;
    let v = [x, y, z];
    if v.iter().any(|c| !c.is_finite()) {
        return None;
    }
    Some(v)
}

pub fn parse_elc(bytes: &[u8]) -> Option<Vec<(Option<String>, [f64; 3])>> {
    let text = std::str::from_utf8(bytes).ok()?;
    let lines: Vec<&str> = text.lines().collect();
    let n = lines
        .iter()
        .find(|l| l.starts_with("NumberPositions"))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|s| s.trim().parse::<usize>().ok())?;
    if n == 0 {
        return None;
    }
    let pos_start = lines.iter().position(|l| l.trim() == "Positions")? + 1;
    let label_start = lines.iter().position(|l| l.trim() == "Labels")? + 1;
    if pos_start + n > lines.len() || label_start + n > lines.len() {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let xyz = parse_xyz(lines[pos_start + i])?;
        let raw = lines[label_start + i].trim();
        let label = if raw.is_empty() {
            None
        } else {
            Some(raw.to_string())
        };
        out.push((label, xyz));
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn elc_mni_fiducials(bytes: &[u8]) -> Option<MniFiducials> {
    let pairs = parse_elc(bytes)?;
    let find = |name: &str| {
        pairs
            .iter()
            .find(|(l, _)| l.as_deref() == Some(name))
            .map(|(_, p)| *p)
    };
    Some(MniFiducials {
        lpa: find("LPA"),
        rpa: find("RPA"),
        nz: find("Nz"),
        iz: find("Iz"),
    })
}

fn montage_fiducial_point(
    montage: &[(Option<String>, [f64; 3])],
    needle: &str,
) -> Option<[f64; 3]> {
    let needle = needle.to_lowercase();
    montage
        .iter()
        .find(|(l, _)| {
            l.as_deref()
                .map_or(false, |s| s.to_lowercase().contains(&needle))
        })
        .map(|(_, p)| *p)
}

pub fn montage_to_mni(
    montage: &[(Option<String>, [f64; 3])],
    mni: &MniFiducials,
) -> Option<(RigidTransform, f64)> {
    let from = [
        montage_fiducial_point(montage, "nasion")?,
        montage_fiducial_point(montage, "left")?,
        montage_fiducial_point(montage, "right")?,
    ];
    let to = [mni.nz?, mni.lpa?, mni.rpa?];
    rigid_coregister(&from, &to)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn align8(p: usize) -> usize {
        (p + 7) & !7
    }

    fn mi_matrix(body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&14u32.to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(body);
        out
    }

    fn name_tag(name: &str) -> Vec<u8> {
        let mut b = Vec::new();
        let len = align8(name.len());
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&(len as u32).to_le_bytes());
        b.extend_from_slice(name.as_bytes());
        for _ in name.len()..len {
            b.push(0);
        }
        b
    }

    fn flags_dims_double(class: u32, name: &str, dims: &[i32], values: &[f64]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&class.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        body.extend_from_slice(&name_tag(name));
        body.extend_from_slice(&9u32.to_le_bytes());
        body.extend_from_slice(&((values.len() * 8) as u32).to_le_bytes());
        for &v in values {
            body.extend_from_slice(&v.to_le_bytes());
        }
        mi_matrix(&body)
    }

    fn char_matrix(name: &str, text: &[u8]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&4u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&1i32.to_le_bytes());
        body.extend_from_slice(&(text.len() as i32).to_le_bytes());
        body.extend_from_slice(&name_tag(name));
        body.extend_from_slice(&1u32.to_le_bytes());
        body.extend_from_slice(&(text.len() as u32).to_le_bytes());
        body.extend_from_slice(text);
        pad_body(&mut body);
        mi_matrix(&body)
    }

    fn pad_body(body: &mut Vec<u8>) {
        while body.len() % 8 != 0 {
            body.push(0);
        }
    }

    fn struct_matrix(name: &str, dims: &[i32], fields: &[(&str, Vec<Vec<u8>>)]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&2u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        body.extend_from_slice(&name_tag(name));
        let mut table = Vec::new();
        for (f, _) in fields {
            table.extend_from_slice(f.as_bytes());
            table.push(0);
        }
        let field_len = table.len();
        body.extend_from_slice(&5u16.to_le_bytes());
        body.extend_from_slice(&4u16.to_le_bytes());
        body.extend_from_slice(&(field_len as u32).to_le_bytes());
        let table_len = align8(field_len);
        body.extend_from_slice(&1u32.to_le_bytes());
        body.extend_from_slice(&(table_len as u32).to_le_bytes());
        body.extend_from_slice(&table);
        for _ in field_len..table_len {
            body.push(0);
        }
        let n_elements = dims.iter().product::<i32>() as usize;
        for i in 0..n_elements {
            for (_, values) in fields {
                body.extend_from_slice(&values[i]);
            }
        }
        mi_matrix(&body)
    }

    fn cell_matrix(name: &str, dims: &[i32], elements: &[Vec<u8>]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&6u32.to_le_bytes());
        body.extend_from_slice(&8u32.to_le_bytes());
        body.extend_from_slice(&1u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&5u32.to_le_bytes());
        body.extend_from_slice(&((dims.len() * 4) as u32).to_le_bytes());
        for &d in dims {
            body.extend_from_slice(&d.to_le_bytes());
        }
        body.extend_from_slice(&name_tag(name));
        for e in elements {
            body.extend_from_slice(e);
        }
        mi_matrix(&body)
    }

    fn header() -> Vec<u8> {
        let mut h = vec![0u8; 128];
        let text = b"MATLAB 5.0 MAT-file";
        h[..text.len()].copy_from_slice(text);
        h[124] = 0x00;
        h[125] = 0x01;
        h[126] = b'I';
        h[127] = b'M';
        h
    }

    fn rotation_matrix(axis: [f64; 3], angle: f64) -> [[f64; 3]; 3] {
        let (x, y, z) = (axis[0], axis[1], axis[2]);
        let norm = (x * x + y * y + z * z).sqrt();
        let (x, y, z) = (x / norm, y / norm, z / norm);
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        [
            [t * x * x + c, t * x * y - s * z, t * x * z + s * y],
            [t * x * y + s * z, t * y * y + c, t * y * z - s * x],
            [t * x * z - s * y, t * y * z + s * x, t * z * z + c],
        ]
    }

    #[test]
    fn umeyama_recovers_a_known_rotation_scale_and_translation() {
        let from = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let r = rotation_matrix([1.0, 0.5, 0.25], 0.7);
        let scale = 2.5;
        let translation = [1.0, -2.0, 0.5];
        let mut to = [[0.0; 3]; 4];
        for i in 0..4 {
            for a in 0..3 {
                let mut acc = 0.0;
                for b in 0..3 {
                    acc += r[a][b] * from[i][b];
                }
                to[i][a] = scale * acc + translation[a];
            }
        }
        let t = umeyama(&from, &to).expect("the transform recovers");
        assert!((t.scale - scale).abs() < 1e-9);
        for a in 0..3 {
            assert!((t.translation[a] - translation[a]).abs() < 1e-9);
            for b in 0..3 {
                assert!(
                    (t.rotation[a][b] - r[a][b]).abs() < 1e-9,
                    "rotation entry [{a}][{b}] drifts"
                );
            }
        }
    }

    #[test]
    fn umeyama_reads_absent_for_degenerate_input() {
        let from = [[0.0; 3]; 3];
        let to = [[1.0; 3]; 3];
        assert!(umeyama(&from[..2], &to[..2]).is_none());
        assert!(umeyama(&from, &to[..2]).is_none());
        let collinear = [
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
            [3.0, 3.0, 3.0],
        ];
        let target = [
            [1.0, 0.0, 0.0],
            [2.0, 1.0, 0.0],
            [3.0, 2.0, 0.0],
            [4.0, 3.0, 0.0],
        ];
        assert!(umeyama(&collinear, &target).is_none());
        let nan = [
            [0.0, 0.0, f64::NAN],
            [1.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        assert!(umeyama(&nan, &to).is_none());
    }

    #[test]
    fn rigid_coregister_locks_scale_and_reports_the_fitted_scale() {
        let montage = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let r = rotation_matrix([0.0, 0.0, 1.0], 0.5);
        let true_scale = 1.5;
        let translation = [0.3, -0.4, 0.2];
        let mut mni = [[0.0; 3]; 4];
        for i in 0..4 {
            for a in 0..3 {
                let mut acc = 0.0;
                for b in 0..3 {
                    acc += r[a][b] * montage[i][b];
                }
                mni[i][a] = true_scale * acc + translation[a];
            }
        }
        let (rigid, fitted) = rigid_coregister(&montage, &mni).expect("the rigid transform fits");
        assert_eq!(rigid.scale, 1.0);
        assert!((fitted - true_scale).abs() < 1e-9);
        for a in 0..3 {
            for b in 0..3 {
                assert!((rigid.rotation[a][b] - r[a][b]).abs() < 1e-9);
            }
        }
    }

    fn shell_fixture(offset: f64) -> Vec<u8> {
        struct_matrix(
            "shell",
            &[1, 1],
            &[
                (
                    "pos",
                    vec![flags_dims_double(
                        6,
                        "pos",
                        &[3, 3],
                        &[
                            offset + 0.0,
                            offset + 1.0,
                            offset + 0.0,
                            offset + 0.0,
                            offset + 0.0,
                            offset + 1.0,
                            offset + 0.0,
                            offset + 0.0,
                            offset + 0.0,
                        ],
                    )],
                ),
                (
                    "tri",
                    vec![flags_dims_double(6, "tri", &[1, 3], &[1.0, 2.0, 3.0])],
                ),
            ],
        )
    }

    #[test]
    fn read_bem_mat_parses_a_cell_of_three_shells_with_unit() {
        let shells = vec![shell_fixture(0.0), shell_fixture(10.0), shell_fixture(20.0)];
        let bnd = cell_matrix("bnd", &[1, 3], &shells);
        let vol = struct_matrix(
            "vol",
            &[1, 1],
            &[
                ("bnd", vec![bnd]),
                ("unit", vec![char_matrix("unit", b"mm")]),
            ],
        );
        let mut bytes = header();
        bytes.extend_from_slice(&vol);
        let bem = read_bem_mat(&bytes).expect("the head model parses");
        assert_eq!(bem.shells.len(), 3);
        assert_eq!(bem.unit.as_deref(), Some("mm"));
        assert_eq!(bem.shells[0].vertices.len(), 3);
        assert_eq!(bem.shells[0].faces.len(), 1);
        assert_eq!(bem.shells[0].faces[0], [0, 1, 2]);
    }

    #[test]
    fn read_bem_mat_parses_a_struct_array_of_shells() {
        let bnd = struct_matrix(
            "bnd",
            &[1, 3],
            &[
                (
                    "pos",
                    vec![
                        flags_dims_double(
                            6,
                            "pos",
                            &[3, 3],
                            &[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                        ),
                        flags_dims_double(
                            6,
                            "pos",
                            &[3, 3],
                            &[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                        ),
                        flags_dims_double(
                            6,
                            "pos",
                            &[3, 3],
                            &[0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                        ),
                    ],
                ),
                (
                    "tri",
                    vec![
                        flags_dims_double(6, "tri", &[1, 3], &[1.0, 2.0, 3.0]),
                        flags_dims_double(6, "tri", &[1, 3], &[1.0, 2.0, 3.0]),
                        flags_dims_double(6, "tri", &[1, 3], &[1.0, 2.0, 3.0]),
                    ],
                ),
            ],
        );
        let vol = struct_matrix("vol", &[1, 1], &[("bnd", vec![bnd])]);
        let mut bytes = header();
        bytes.extend_from_slice(&vol);
        let bem = read_bem_mat(&bytes).expect("the struct-array head model parses");
        assert_eq!(bem.shells.len(), 3);
        assert_eq!(bem.unit, None);
    }

    #[test]
    fn read_bem_mat_reads_absent_for_a_missing_bnd() {
        let vol = struct_matrix(
            "vol",
            &[1, 1],
            &[("unit", vec![char_matrix("unit", b"mm")])],
        );
        let mut bytes = header();
        bytes.extend_from_slice(&vol);
        assert!(read_bem_mat(&bytes).is_none());
    }

    fn chaninfo_fixture() -> Vec<u8> {
        let nodatchans = struct_matrix(
            "nodatchans",
            &[1, 3],
            &[
                (
                    "labels",
                    vec![
                        char_matrix("labels", b"E130"),
                        char_matrix("labels", b"E131"),
                        char_matrix("labels", b"E132"),
                    ],
                ),
                (
                    "type",
                    vec![
                        char_matrix("type", b"FID"),
                        char_matrix("type", b"FID"),
                        char_matrix("type", b"FID"),
                    ],
                ),
                (
                    "description",
                    vec![
                        char_matrix("description", b"Nasion"),
                        char_matrix("description", b"Left periauricular point"),
                        char_matrix("description", b"Right periauricular point"),
                    ],
                ),
                (
                    "X",
                    vec![
                        flags_dims_double(6, "X", &[1, 1], &[10.35]),
                        flags_dims_double(6, "X", &[1, 1], &[0.046]),
                        flags_dims_double(6, "X", &[1, 1], &[0.046]),
                    ],
                ),
                (
                    "Y",
                    vec![
                        flags_dims_double(6, "Y", &[1, 1], &[0.0]),
                        flags_dims_double(6, "Y", &[1, 1], &[6.71]),
                        flags_dims_double(6, "Y", &[1, 1], &[-6.71]),
                    ],
                ),
                (
                    "Z",
                    vec![
                        flags_dims_double(6, "Z", &[1, 1], &[-2.69]),
                        flags_dims_double(6, "Z", &[1, 1], &[-3.71]),
                        flags_dims_double(6, "Z", &[1, 1], &[-3.71]),
                    ],
                ),
            ],
        );
        let chaninfo = struct_matrix("chaninfo", &[1, 1], &[("nodatchans", vec![nodatchans])]);
        let mut bytes = header();
        bytes.extend_from_slice(&chaninfo);
        bytes
    }

    #[test]
    fn montage_fiducials_reads_the_three_fid_landmarks() {
        let bytes = chaninfo_fixture();
        let fids = montage_fiducials(&bytes).expect("the fiducials read");
        assert_eq!(fids.len(), 3);
        assert_eq!(fids[0].0.as_deref(), Some("Nasion"));
        assert_eq!(fids[1].0.as_deref(), Some("Left periauricular point"));
        assert_eq!(fids[2].0.as_deref(), Some("Right periauricular point"));
        assert!((fids[0].1[0] - 10.35).abs() < 1e-9);
        assert!((fids[1].1[1] - 6.71).abs() < 1e-9);
        assert!((fids[2].1[1] + 6.71).abs() < 1e-9);
    }

    #[test]
    fn montage_fiducials_reads_absent_without_a_chaninfo() {
        let bytes = header();
        assert!(montage_fiducials(&bytes).is_none());
    }

    fn elc_fixture() -> Vec<u8> {
        let text = concat!(
            "# ASA electrode file\n",
            "ReferenceLabel\tavg\n",
            "UnitPosition\tmm\n",
            "NumberPositions=\t4\n",
            "Positions\n",
            "-86.0761 -19.9897 -47.9860\n",
            "85.7939 -20.0093 -48.0310\n",
            "0.0083 86.8110 -39.9830\n",
            "0.0045 -118.5650 -23.0780\n",
            "Labels\n",
            "LPA\n",
            "RPA\n",
            "Nz\n",
            "Iz\n",
        );
        text.as_bytes().to_vec()
    }

    #[test]
    fn parse_elc_reads_positions_zipped_to_labels() {
        let pairs = parse_elc(&elc_fixture()).expect("the elc parses");
        assert_eq!(pairs.len(), 4);
        assert_eq!(pairs[0].0.as_deref(), Some("LPA"));
        assert!((pairs[0].1[0] + 86.0761).abs() < 1e-4);
        assert_eq!(pairs[2].0.as_deref(), Some("Nz"));
        assert!((pairs[2].1[1] - 86.8110).abs() < 1e-4);
        assert_eq!(pairs[3].0.as_deref(), Some("Iz"));
    }

    #[test]
    fn elc_mni_fiducials_reads_the_four_landmarks_in_mm() {
        let mni = elc_mni_fiducials(&elc_fixture()).expect("the fiducials read");
        let lpa = mni.lpa.expect("LPA present");
        assert!((lpa[0] + 86.0761).abs() < 1e-4);
        assert!((lpa[1] + 19.9897).abs() < 1e-4);
        assert!((lpa[2] + 47.9860).abs() < 1e-4);
        let nz = mni.nz.expect("Nz present");
        assert!((nz[1] - 86.8110).abs() < 1e-4);
        let iz = mni.iz.expect("Iz present");
        assert!((iz[1] + 118.5650).abs() < 1e-4);
    }

    #[test]
    fn elc_mni_fiducials_reads_absent_on_foreign_bytes() {
        assert!(parse_elc(b"").is_none());
        assert!(parse_elc(b"not an elc").is_none());
        assert!(elc_mni_fiducials(b"Labels\nLPA\nRPA\nNz\nIz\n").is_none());
    }

    #[test]
    fn montage_to_mni_recovers_a_known_rigid_transform() {
        let r = rotation_matrix([0.3, -0.2, 0.7], 0.4);
        let translation = [5.0, -3.0, 2.0];
        let from = [
            [10.35, 0.0, -2.69],
            [0.046, 6.71, -3.71],
            [0.046, -6.71, -3.71],
        ];
        let mut to = [[0.0; 3]; 3];
        for i in 0..3 {
            for a in 0..3 {
                let mut acc = 0.0;
                for b in 0..3 {
                    acc += r[a][b] * from[i][b];
                }
                to[i][a] = acc + translation[a];
            }
        }
        let montage = vec![
            (Some("Nasion".to_string()), from[0]),
            (Some("Left periauricular point".to_string()), from[1]),
            (Some("Right periauricular point".to_string()), from[2]),
        ];
        let mni = MniFiducials {
            nz: Some(to[0]),
            lpa: Some(to[1]),
            rpa: Some(to[2]),
            iz: None,
        };
        let (fitted, scale) = montage_to_mni(&montage, &mni).expect("the transform fits");
        assert!((scale - 1.0).abs() < 1e-9);
        for a in 0..3 {
            assert!((fitted.translation[a] - translation[a]).abs() < 1e-9);
            for b in 0..3 {
                assert!((fitted.rotation[a][b] - r[a][b]).abs() < 1e-9);
            }
        }
    }
}
