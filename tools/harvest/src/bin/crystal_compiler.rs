use omegaflow::cdn::upload_asset;
use omegaflow::cif::{Crystal, parse_cif};
use omegaflow::matfile::{MatData, parse_mat};
use omegaflow::rixs::{
    MEV_TO_HZ, SpinBin, SpinOscillator, SpinSpectrumBin, charge_oscillators, encode_spin_bin,
    parse_rixs_mev, parse_sw_spin, spin_oscillators,
};
use std::process::Command;

const MAGIC: [u8; 2] = [0xCF, 0x86];
const VERSION: u8 = 0x01;
const ANGSTROM_M: f64 = 1.0e-10;

fn fetch(url: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("180")
        .arg(url)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "curl {} http {}: {}",
            url,
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn cod_url(id: &str) -> String {
    format!("https://www.crystallography.net/cod/{}.cif", id)
}

fn encode_crystal(crystal: &Crystal, lab: Option<(f64, f64, f64)>) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    for v in [
        crystal.a * ANGSTROM_M,
        crystal.b * ANGSTROM_M,
        crystal.c * ANGSTROM_M,
        crystal.alpha_deg,
        crystal.beta_deg,
        crystal.gamma_deg,
    ] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(&(crystal.space_group).to_le_bytes());
    out.extend_from_slice(&(crystal.atoms.len() as u32).to_le_bytes());
    push_lab(&mut out, lab);
    for atom in &crystal.atoms {
        out.extend_from_slice(&atom.species);
        out.push(atom.wyckoff);
        out.extend_from_slice(&[0u8; 3]);
        for f in atom.fract {
            out.extend_from_slice(&f.to_le_bytes());
        }
        out.extend_from_slice(&atom.occupancy.to_le_bytes());
    }
    out
}

fn push_lab(out: &mut Vec<u8>, lab: Option<(f64, f64, f64)>) {
    match lab {
        Some((lat, lon, alt)) => {
            out.extend_from_slice(&lat.to_le_bytes());
            out.extend_from_slice(&lon.to_le_bytes());
            out.extend_from_slice(&alt.to_le_bytes());
            out.push(1u8);
        }
        None => {
            out.extend_from_slice(&0.0f64.to_le_bytes());
            out.extend_from_slice(&0.0f64.to_le_bytes());
            out.extend_from_slice(&0.0f64.to_le_bytes());
            out.push(0u8);
        }
    }
}

fn parse_doping(prefix: &str) -> Option<u8> {
    match prefix {
        "UD" => Some(0),
        "OD1" => Some(1),
        "OD2" => Some(2),
        _ => None,
    }
}

fn parse_rlu(token: &str) -> Option<f64> {
    let fixed = token.replace('p', ".");
    fixed.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_folder(name: &str) -> Option<(u8, f64, f64)> {
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() != 3 {
        return None;
    }
    let doping = parse_doping(parts[0])?;
    let q_h = parse_rlu(parts[1])?;
    let q_l = parse_rlu(parts[2])?;
    Some((doping, q_h, q_l))
}

fn harvest_rixs(dir: &str) -> Vec<SpinSpectrumBin> {
    let mut out = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(dir)];
    while let Some(path) = stack.pop() {
        if path.file_name().map_or(false, |n| {
            let n = n.to_string_lossy();
            n.starts_with("__MACOSX") || n == "Theory_figures" || n == "codes"
        }) {
            continue;
        }
        if path.is_dir() {
            if let Ok(rd) = std::fs::read_dir(&path) {
                for e in rd.flatten() {
                    stack.push(e.path());
                }
            }
            continue;
        }
        if path.file_name().map_or(false, |n| n == "sw_spin.txt") {
            let folder = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string());
            if let Some((doping, q_h, q_l)) = folder.as_deref().and_then(parse_folder) {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    if let Some(spec) = parse_sw_spin(&text) {
                        out.push(SpinSpectrumBin {
                            doping,
                            q_h,
                            q_l,
                            oscillators: spin_oscillators(&spec),
                        });
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| {
        (a.doping, a.q_h.to_bits(), a.q_l.to_bits()).cmp(&(
            b.doping,
            b.q_h.to_bits(),
            b.q_l.to_bits(),
        ))
    });
    out
}

const CHARGE_VERSION: u8 = 0x03;

fn harvest_plasmon(dir: &str) -> Vec<(f64, u8, Vec<omegaflow::rixs::SpinOscillator>)> {
    let mut out = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(dir)];
    while let Some(path) = stack.pop() {
        if path.file_name().map_or(false, |n| {
            let n = n.to_string_lossy();
            n.starts_with("__MACOSX") || n == ".DS_Store"
        }) {
            continue;
        }
        if path.is_dir() {
            if let Ok(rd) = std::fs::read_dir(&path) {
                for e in rd.flatten() {
                    stack.push(e.path());
                }
            }
            continue;
        }
        let is_txt = path.extension().map_or(false, |e| e == "txt");
        if !is_txt {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Some(spec) = parse_rixs_mev(&text) {
                out.push((spec.momentum, spec.axis, charge_oscillators(&spec)));
            }
        }
    }
    out.sort_by(|a, b| (a.1, a.0.to_bits()).cmp(&(b.1, b.0.to_bits())));
    out
}

fn encode_charge_bin(
    spectra: &[(f64, u8, Vec<omegaflow::rixs::SpinOscillator>)],
    lab: Option<(f64, f64, f64)>,
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.push(CHARGE_VERSION);
    out.extend_from_slice(&(spectra.len() as u32).to_le_bytes());
    push_lab(&mut out, lab);
    for (momentum, axis, osc) in spectra {
        out.extend_from_slice(&momentum.to_le_bytes());
        out.push(*axis);
        out.extend_from_slice(&(osc.len() as u32).to_le_bytes());
        for o in osc {
            out.extend_from_slice(&o.freq_hz.to_le_bytes());
            out.extend_from_slice(&o.bin_width_hz.to_le_bytes());
            out.extend_from_slice(&o.val.to_le_bytes());
            out.extend_from_slice(&o.err.to_le_bytes());
        }
    }
    out
}

const EELS_VERSION: u8 = 0x04;

fn median_gap(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return 0.0;
    }
    let mut gaps: Vec<f64> = v
        .windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .filter(|&g| g > 0.0 && g.is_finite())
        .collect();
    gaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    gaps.get(gaps.len() / 2).copied().unwrap_or(0.0)
}

fn harvest_eels(path: &str) -> Vec<(u32, Vec<SpinOscillator>)> {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("crystal_compiler: --eels {} reads void", path);
        return Vec::new();
    };
    let Some(arrays) = parse_mat(&bytes) else {
        eprintln!("crystal_compiler: --eels {} parses void", path);
        return Vec::new();
    };
    let ene = arrays
        .iter()
        .find(|a| a.name == "ene")
        .and_then(|a| match &a.data {
            MatData::Double(v) => Some(v),
            _ => None,
        });
    let profile = arrays
        .iter()
        .find(|a| a.name == "eels_lineprofile")
        .and_then(|a| match &a.data {
            MatData::Double(v) => Some((a.dims.clone(), v)),
            _ => None,
        });
    let (Some(ene), Some((dims, profile))) = (ene, profile) else {
        eprintln!(
            "crystal_compiler: --eels {} carries no ene/eels_lineprofile",
            path
        );
        return Vec::new();
    };
    if dims.len() != 2 || ene.len() != dims[1] || profile.len() != dims[0] * dims[1] {
        eprintln!("crystal_compiler: --eels {} axis shape void", path);
        return Vec::new();
    }
    let n_rows = dims[0];
    let n_cols = dims[1];
    let bin_width = median_gap(ene) * MEV_TO_HZ;
    let mut out = Vec::new();
    for i in 0..n_rows {
        let mut osc = Vec::new();
        for j in 0..n_cols {
            let e = ene[j];
            if e > 0.0 {
                osc.push(SpinOscillator {
                    freq_hz: e * MEV_TO_HZ,
                    bin_width_hz: bin_width,
                    val: profile[i + j * n_rows],
                    err: 0.0,
                });
            }
        }
        if !osc.is_empty() {
            out.push((i as u32, osc));
        }
    }
    out
}

fn encode_eels_bin(
    spectra: &[(u32, Vec<SpinOscillator>)],
    lab: Option<(f64, f64, f64)>,
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.push(EELS_VERSION);
    out.extend_from_slice(&(spectra.len() as u32).to_le_bytes());
    push_lab(&mut out, lab);
    for (idx, osc) in spectra {
        out.extend_from_slice(&idx.to_le_bytes());
        out.extend_from_slice(&(osc.len() as u32).to_le_bytes());
        for o in osc {
            out.extend_from_slice(&o.freq_hz.to_le_bytes());
            out.extend_from_slice(&o.bin_width_hz.to_le_bytes());
            out.extend_from_slice(&o.val.to_le_bytes());
            out.extend_from_slice(&o.err.to_le_bytes());
        }
    }
    out
}

fn mat_dump(path: &str) {
    let Some(bytes) = std::fs::read(path).ok() else {
        eprintln!("crystal_compiler: {} reads void", path);
        return;
    };
    let Some(arrays) = parse_mat(&bytes) else {
        eprintln!("crystal_compiler: {} parses void", path);
        return;
    };
    for a in &arrays {
        let (kind, len) = match &a.data {
            MatData::Double(v) => ("double", v.len()),
            MatData::Single(v) => ("single", v.len()),
            MatData::Int32(v) => ("int32", v.len()),
            MatData::Char(v) => ("char", v.len()),
            MatData::Empty => ("empty", 0),
        };
        eprintln!("mat: {} dims {:?} {} len {}", a.name, a.dims, kind, len);
        if let MatData::Double(v) = &a.data {
            if !v.is_empty() {
                eprintln!("  first: {:?}", &v[..v.len().min(6)]);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cods: Vec<String> = Vec::new();
    let mut rixs_dirs: Vec<String> = Vec::new();
    let mut plasmon_dirs: Vec<String> = Vec::new();
    let mut mat_files: Vec<String> = Vec::new();
    let mut eels_files: Vec<String> = Vec::new();
    let mut lab: Option<(f64, f64, f64)> = None;
    let mut ci_mode = false;
    let mut out_dir = String::from(".");
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--cod" => {
                i += 1;
                if let Some(id) = args.get(i) {
                    cods.push(id.clone());
                }
            }
            "--rixs" => {
                i += 1;
                if let Some(d) = args.get(i) {
                    rixs_dirs.push(d.clone());
                }
            }
            "--plasmon" => {
                i += 1;
                if let Some(d) = args.get(i) {
                    plasmon_dirs.push(d.clone());
                }
            }
            "--mat" => {
                i += 1;
                if let Some(f) = args.get(i) {
                    mat_files.push(f.clone());
                }
            }
            "--eels" => {
                i += 1;
                if let Some(f) = args.get(i) {
                    eels_files.push(f.clone());
                }
            }
            "--lab" => {
                if args.len() >= i + 4 {
                    let lat = args[i + 1].parse::<f64>().ok();
                    let lon = args[i + 2].parse::<f64>().ok();
                    let alt = args[i + 3].parse::<f64>().ok();
                    if let (Some(lat), Some(lon), Some(alt)) = (lat, lon, alt) {
                        if lat.is_finite() && lon.is_finite() && alt.is_finite() {
                            lab = Some((lat, lon, alt));
                        }
                    }
                    i += 3;
                }
            }
            "--ci-mode" => ci_mode = true,
            "--out" => {
                i += 1;
                if let Some(d) = args.get(i) {
                    out_dir = d.clone();
                }
            }
            _ => {}
        }
        i += 1;
    }
    if cods.is_empty()
        && rixs_dirs.is_empty()
        && plasmon_dirs.is_empty()
        && mat_files.is_empty()
        && eels_files.is_empty()
    {
        eprintln!(
            "crystal_compiler: no --cod <id>, --rixs <dir>, --plasmon <dir>, --mat <file> or --eels <file> given"
        );
        std::process::exit(1);
    }
    let mut compiled = 0usize;
    for id in &cods {
        let url = cod_url(id);
        let Some(text) = fetch(&url) else {
            eprintln!("crystal_compiler: {} carries no CIF", id);
            continue;
        };
        let Some(crystal) = parse_cif(&text) else {
            eprintln!("crystal_compiler: {} parses void", id);
            continue;
        };
        let atom_count = crystal.atoms.len();
        let volume = crystal.cell_volume() * ANGSTROM_M.powi(3);
        let path = format!("{}/crystal_{}.bin", out_dir, id);
        let bytes = encode_crystal(&crystal, lab);
        if let Err(e) = std::fs::write(&path, &bytes) {
            eprintln!("crystal_compiler: write {} returned {}", path, e);
            continue;
        }
        eprintln!(
            "crystal_compiler: {} {} atoms, volume {:.3e} m^3, anchor {} -> {}",
            id,
            atom_count,
            volume,
            match lab {
                Some(_) => "lab",
                None => "pending",
            },
            path,
        );
        if ci_mode {
            let _ = upload_asset(&path);
        }
        compiled += 1;
    }
    for dir in &rixs_dirs {
        let spectra = harvest_rixs(dir);
        if spectra.is_empty() {
            eprintln!("crystal_compiler: --rixs {} carries no sw_spin.txt", dir);
            continue;
        }
        let osc_total: usize = spectra.iter().map(|s| s.oscillators.len()).sum();
        let n_spectra = spectra.len();
        let path = format!("{}/rixs_spin.bin", out_dir);
        let bytes = encode_spin_bin(&SpinBin { lab, spectra });
        if let Err(e) = std::fs::write(&path, &bytes) {
            eprintln!("crystal_compiler: write {} returned {}", path, e);
            continue;
        }
        eprintln!(
            "crystal_compiler: rixs {} spectra, {} oscillators, anchor {} -> {}",
            n_spectra,
            osc_total,
            match lab {
                Some(_) => "lab",
                None => "pending",
            },
            path,
        );
        if ci_mode {
            let _ = upload_asset(&path);
        }
        compiled += 1;
    }
    for dir in &plasmon_dirs {
        let spectra = harvest_plasmon(dir);
        if spectra.is_empty() {
            eprintln!(
                "crystal_compiler: --plasmon {} carries no RIXS meV spectra",
                dir
            );
            continue;
        }
        let osc_total: usize = spectra.iter().map(|(_, _, o)| o.len()).sum();
        let n_spectra = spectra.len();
        let path = format!("{}/rixs_charge.bin", out_dir);
        let bytes = encode_charge_bin(&spectra, lab);
        if let Err(e) = std::fs::write(&path, &bytes) {
            eprintln!("crystal_compiler: write {} returned {}", path, e);
            continue;
        }
        eprintln!(
            "crystal_compiler: plasmon {} spectra, {} oscillators, anchor {} -> {}",
            n_spectra,
            osc_total,
            match lab {
                Some(_) => "lab",
                None => "pending",
            },
            path,
        );
        if ci_mode {
            let _ = upload_asset(&path);
        }
        compiled += 1;
    }
    for f in &mat_files {
        mat_dump(f);
    }
    for f in &eels_files {
        let spectra = harvest_eels(f);
        if spectra.is_empty() {
            continue;
        }
        let osc_total: usize = spectra.iter().map(|(_, o)| o.len()).sum();
        let path = format!("{}/eels_acoustic.bin", out_dir);
        let bytes = encode_eels_bin(&spectra, lab);
        if let Err(e) = std::fs::write(&path, &bytes) {
            eprintln!("crystal_compiler: write {} returned {}", path, e);
            continue;
        }
        eprintln!(
            "crystal_compiler: eels {} profiles, {} oscillators, anchor {} -> {}",
            spectra.len(),
            osc_total,
            match lab {
                Some(_) => "lab",
                None => "pending",
            },
            path,
        );
        if ci_mode {
            let _ = upload_asset(&path);
        }
        compiled += 1;
    }
    eprintln!("crystal_compiler: {} asset compiled", compiled);
}
