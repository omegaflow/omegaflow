#[derive(Clone, Debug)]
pub struct CrystalAtom {
    pub species: [u8; 4],
    pub fract: [f64; 3],
    pub occupancy: f64,
    pub wyckoff: u8,
}

#[derive(Clone, Debug)]
pub struct Crystal {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub alpha_deg: f64,
    pub beta_deg: f64,
    pub gamma_deg: f64,
    pub space_group: u32,
    pub atoms: Vec<CrystalAtom>,
}

impl Crystal {
    pub fn cell_volume(&self) -> f64 {
        let ca = self.alpha_deg.to_radians().cos();
        let cb = self.beta_deg.to_radians().cos();
        let cg = self.gamma_deg.to_radians().cos();
        self.a * self.b * self.c * (1.0 - ca * ca - cb * cb - cg * cg + 2.0 * ca * cb * cg).sqrt()
    }
}

fn esd_value(token: &str) -> Option<f64> {
    let core = token.split('(').next().unwrap_or(token).trim();
    if core.is_empty() {
        return None;
    }
    core.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn species_bytes(type_symbol: &str) -> [u8; 4] {
    let element: String = type_symbol
        .chars()
        .take_while(|c| c.is_alphabetic())
        .collect();
    let mut out = [b' '; 4];
    for (i, b) in element.bytes().take(4).enumerate() {
        out[i] = b;
    }
    out
}

pub fn parse_cif(text: &str) -> Option<Crystal> {
    let mut a = 0.0f64;
    let mut b = 0.0f64;
    let mut c = 0.0f64;
    let mut alpha = 0.0f64;
    let mut beta = 0.0f64;
    let mut gamma = 0.0f64;
    let mut space_group = 0u32;
    for line in text.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("_cell_length_a") {
            a = esd_value(v)?;
        } else if let Some(v) = line.strip_prefix("_cell_length_b") {
            b = esd_value(v)?;
        } else if let Some(v) = line.strip_prefix("_cell_length_c") {
            c = esd_value(v)?;
        } else if let Some(v) = line.strip_prefix("_cell_angle_alpha") {
            alpha = esd_value(v)?;
        } else if let Some(v) = line.strip_prefix("_cell_angle_beta") {
            beta = esd_value(v)?;
        } else if let Some(v) = line.strip_prefix("_cell_angle_gamma") {
            gamma = esd_value(v)?;
        } else if let Some(v) = line.strip_prefix("_space_group_IT_number") {
            space_group = esd_value(v)?.round() as u32;
        }
    }
    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        return None;
    }
    let atoms = parse_atom_sites(text)?;
    Some(Crystal {
        a,
        b,
        c,
        alpha_deg: alpha,
        beta_deg: beta,
        gamma_deg: gamma,
        space_group,
        atoms,
    })
}

fn parse_atom_sites(text: &str) -> Option<Vec<CrystalAtom>> {
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        if lines[i].trim() != "loop_" {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        let mut keys: Vec<&str> = Vec::new();
        while j < lines.len() && lines[j].trim_start().starts_with('_') {
            keys.push(lines[j].trim());
            j += 1;
        }
        let mut x_col: Option<usize> = None;
        let mut y_col: Option<usize> = None;
        let mut z_col: Option<usize> = None;
        let mut occ_col: Option<usize> = None;
        let mut species_col: Option<usize> = None;
        let mut wyckoff_col: Option<usize> = None;
        for (k, key) in keys.iter().enumerate() {
            if key.starts_with("_atom_site_fract_x") {
                x_col = Some(k);
            } else if key.starts_with("_atom_site_fract_y") {
                y_col = Some(k);
            } else if key.starts_with("_atom_site_fract_z") {
                z_col = Some(k);
            } else if key.starts_with("_atom_site_occupancy") {
                occ_col = Some(k);
            } else if key.starts_with("_atom_site_type_symbol") {
                species_col = Some(k);
            } else if key.starts_with("_atom_site_Wyckoff_symbol") {
                wyckoff_col = Some(k);
            }
        }
        let (x_col, y_col, z_col) = match (x_col, y_col, z_col) {
            (Some(x), Some(y), Some(z)) => (x, y, z),
            _ => {
                i = j;
                continue;
            }
        };
        let species_col = species_col?;
        let mut atoms: Vec<CrystalAtom> = Vec::new();
        let mut k = j;
        while k < lines.len() {
            let row = lines[k].trim();
            if row.is_empty() || row.starts_with('#') || row.starts_with('_') || row == "loop_" {
                break;
            }
            let tokens: Vec<&str> = row.split_whitespace().collect();
            if tokens.len() <= x_col.max(y_col).max(z_col).max(species_col) {
                break;
            }
            let fx = esd_value(tokens[x_col]);
            let fy = esd_value(tokens[y_col]);
            let fz = esd_value(tokens[z_col]);
            let occupancy = match occ_col {
                Some(o) => tokens.get(o).and_then(|t| esd_value(t)),
                None => Some(1.0),
            };
            let wyckoff = match wyckoff_col {
                Some(w) => tokens.get(w).and_then(|t| t.bytes().next()),
                None => None,
            }
            .unwrap_or(b' ');
            if let (Some(fx), Some(fy), Some(fz), Some(occ)) = (fx, fy, fz, occupancy) {
                if fx.is_finite()
                    && fy.is_finite()
                    && fz.is_finite()
                    && occ.is_finite()
                    && occ > 0.0
                {
                    atoms.push(CrystalAtom {
                        species: species_bytes(tokens[species_col]),
                        fract: [fx, fy, fz],
                        occupancy: occ,
                        wyckoff,
                    });
                }
            }
            k += 1;
        }
        if !atoms.is_empty() {
            return Some(atoms);
        }
        i = j;
    }
    None
}

pub fn cartesian_from_fractal(fract: [f64; 3], crystal: &Crystal) -> [f64; 3] {
    let ar = crystal.alpha_deg.to_radians();
    let br = crystal.beta_deg.to_radians();
    let gr = crystal.gamma_deg.to_radians();
    let ca = ar.cos();
    let cb = br.cos();
    let (sg, cg) = gr.sin_cos();
    let cx = (cb - ca * cg) / sg;
    let cy = (1.0 - ca * ca - cb * cb - cg * cg + 2.0 * ca * cb * cg).sqrt() / sg;
    let ax = crystal.a;
    let bx = crystal.b * cg;
    let by = crystal.b * sg;
    let cxv = crystal.c * cx;
    let cyv = crystal.c * cy;
    let czv = crystal.c * (1.0 - cx * cx - cy * cy).sqrt();
    [
        ax * fract[0] + bx * fract[1] + cxv * fract[2],
        by * fract[1] + cyv * fract[2],
        czv * fract[2],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "\
data_1001452
_cell_length_a                3.8128(1)
_cell_length_b                3.8806(2)
_cell_length_c                11.6303(5)
_cell_angle_alpha             90
_cell_angle_beta              90
_cell_angle_gamma             90
_space_group_IT_number        47
loop_
_atom_site_label
_atom_site_type_symbol
_atom_site_symmetry_multiplicity
_atom_site_Wyckoff_symbol
_atom_site_fract_x
_atom_site_fract_y
_atom_site_fract_z
_atom_site_occupancy
_atom_site_attached_hydrogens
_atom_site_calc_flag
Ba1 Ba2+ 2 t 0.5 0.5 0.1826(5) 1. 0 d
Y1 Y3+ 1 h 0.5 0.5 0.5 1. 0 d
Cu1 Cu2+ 1 a 0. 0. 0. 1. 0 d
Cu2 Cu2+ 2 q 0. 0. 0.3542(3) 1. 0 d
O1 O2- 2 q 0. 0. 0.1595(4) 1. 0 d
O2 O2- 2 s 0.5 0. 0.3773(4) 1. 0 d
O3 O2- 2 r 0. 0.5 0.3769(5) 1. 0 d
O4 O2- 1 e 0. 0.5 0. 1. 0 d
";

    #[test]
    fn parse_lattice_constants_and_space_group() {
        let crystal = parse_cif(FIXTURE).unwrap();
        assert!((crystal.a - 3.8128).abs() < 1e-9);
        assert!((crystal.b - 3.8806).abs() < 1e-9);
        assert!((crystal.c - 11.6303).abs() < 1e-9);
        assert_eq!(crystal.alpha_deg, 90.0);
        assert_eq!(crystal.space_group, 47);
    }

    #[test]
    fn parse_atom_sites_species_and_esd() {
        let crystal = parse_cif(FIXTURE).unwrap();
        assert_eq!(crystal.atoms.len(), 8);
        let ba = &crystal.atoms[0];
        assert_eq!(&ba.species, b"Ba  ");
        assert!((ba.fract[2] - 0.1826).abs() < 1e-9);
        assert_eq!(ba.wyckoff, b't');
        assert_eq!(crystal.atoms[2].species, *b"Cu  ");
    }

    #[test]
    fn orthorhombic_cartesian_maps_lattice_axes() {
        let crystal = parse_cif(FIXTURE).unwrap();
        let origin = cartesian_from_fractal([0.0, 0.0, 0.0], &crystal);
        assert_eq!(origin, [0.0, 0.0, 0.0]);
        let a_axis = cartesian_from_fractal([1.0, 0.0, 0.0], &crystal);
        assert!((a_axis[0] - 3.8128).abs() < 1e-9);
        assert!(a_axis[1].abs() < 1e-9 && a_axis[2].abs() < 1e-9);
    }

    #[test]
    fn missing_atom_sites_is_none() {
        let bare = "data_x\n_cell_length_a 1.0\n_cell_length_b 1.0\n_cell_length_c 1.0\n";
        assert!(parse_cif(bare).is_none());
    }
}
