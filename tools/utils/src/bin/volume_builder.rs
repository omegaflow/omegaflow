use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use omegaflow::archivar::channels::build_netcdf4_volume;
use omegaflow::archivar::types::{Extract, Frame, SourceConfig};
use omegaflow::hdf5::{Endian, Hdf5File, decode_f32, decode_f64};
use omegaflow::volume::{Axis, AxisKind, Volume};

fn axis_slot(name: &str) -> Option<usize> {
    let n = name.to_lowercase();
    if n.contains("dep") || n.contains("radius") || n == "z" {
        Some(0)
    } else if n.contains("lat") {
        Some(1)
    } else if n.contains("lon") || n.contains("long") {
        Some(2)
    } else {
        None
    }
}

fn compare_volumes(offline: &Volume, live: &Volume) -> Result<f32, String> {
    if offline.dims != live.dims {
        return Err(format!("dims {:?} vs {:?}", offline.dims, live.dims));
    }
    for a in 0..3 {
        let ax_off = &offline.axes[a];
        let ax_live = &live.axes[a];
        if ax_off.values.len() != ax_live.values.len() {
            return Err(format!(
                "axis {a} length {} vs {}",
                ax_off.values.len(),
                ax_live.values.len()
            ));
        }
        for (i, (vo, vl)) in ax_off.values.iter().zip(ax_live.values.iter()).enumerate() {
            if vo != vl {
                return Err(format!("axis {a} value {i}: {vo} vs {vl}"));
            }
        }
    }
    if offline.data.len() != live.data.len() {
        return Err(format!(
            "data length {} vs {}",
            offline.data.len(),
            live.data.len()
        ));
    }
    let mut max_abs = 0.0f32;
    for (i, (a, b)) in offline.data.iter().zip(live.data.iter()).enumerate() {
        let d = (a - b).abs();
        if d > max_abs {
            max_abs = d;
        }
        let tol = 1e-6f32 * a.abs().max(b.abs()).max(1.0);
        if d > tol {
            return Err(format!("data cell {i}: {a} vs {b} (Δ {d})"));
        }
    }
    Ok(max_abs)
}

fn collect_paths(file: &Hdf5File) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    if let Ok(root) = file.root() {
        for l in &root.links {
            stack.push(l.name.clone());
        }
    }
    let mut visited = HashSet::new();
    while let Some(name) = stack.pop() {
        if !visited.insert(name.clone()) {
            continue;
        }
        if let Ok(obj) = file.resolve(&name) {
            if obj.is_group {
                for l in &obj.links {
                    stack.push(format!("{}/{}", name, l.name));
                }
            } else if obj.dataspace.is_some() && obj.datatype.is_some() {
                out.push(name);
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut path: Option<String> = None;
    let mut out_override: Option<String> = None;
    let mut ci_mode = false;
    let mut tag: Option<String> = None;
    let mut verify_live: Option<[String; 4]> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                match args.get(i) {
                    Some(v) => out_override = Some(v.clone()),
                    None => {
                        eprintln!("volume_builder: --out needs a path");
                        return;
                    }
                }
            }
            "--ci-mode" => ci_mode = true,
            "--tag" => {
                i += 1;
                match args.get(i) {
                    Some(v) => tag = Some(v.clone()),
                    None => {
                        eprintln!("volume_builder: --tag needs a netloc");
                        return;
                    }
                }
            }
            "--verify-live" => {
                let mut keys = [String::new(), String::new(), String::new(), String::new()];
                for slot in keys.iter_mut() {
                    i += 1;
                    match args.get(i) {
                        Some(v) => *slot = v.clone(),
                        None => {
                            eprintln!(
                                "volume_builder: --verify-live needs <value_key> <lat_key> <lon_key> <depth_key>"
                            );
                            return;
                        }
                    }
                }
                verify_live = Some(keys);
            }
            other => {
                if path.is_none() {
                    path = Some(other.to_string());
                } else {
                    eprintln!("volume_builder: unexpected argument {other}");
                    return;
                }
            }
        }
        i += 1;
    }
    let Some(path) = path else {
        eprintln!("usage: volume_builder <file.nc> [--out <path>] [--ci-mode --tag <netloc>]");
        return;
    };
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("volume_builder: file does not open: {path}: {e}");
            return;
        }
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("volume_builder: {path} parses not: {note:?}");
            return;
        }
    };

    let mut mask: Option<(String, [u64; 3])> = None;
    let mut rank1: Vec<(String, usize, u64, Endian, usize)> = Vec::new();
    for name in collect_paths(&file) {
        let Ok((_, ds, dt)) = file.dataset(&name) else {
            continue;
        };
        if dt.class != 1 {
            continue;
        }
        if ds.dims.len() == 3 {
            if mask.is_none() {
                mask = Some((name, [ds.dims[0], ds.dims[1], ds.dims[2]]));
            }
        } else if ds.dims.len() == 1 && (dt.size == 4 || dt.size == 8) {
            let slot = axis_slot(&name).unwrap_or(usize::MAX);
            rank1.push((name, slot, ds.dims[0], dt.endian, dt.size));
        }
    }
    let Some((mask_name, mask_dims)) = mask else {
        eprintln!("volume_builder: no rank-3 float dataset in {path}");
        return;
    };

    let mut axis_var: [Option<(&str, u64, Endian, usize)>; 3] = [None, None, None];
    for (name, slot, len, endian, size) in &rank1 {
        if *slot >= 3 {
            continue;
        }
        match axis_var[*slot] {
            Some((prior, _, _, _)) => {
                eprintln!(
                    "volume_builder: axis slot {slot} carries {prior} and {name} — two coordinate variables name the axis; refused"
                );
                return;
            }
            None => axis_var[*slot] = Some((name, *len, *endian, *size)),
        }
    }
    for d in 0..3 {
        let Some((name, len, _, _)) = axis_var[d] else {
            eprintln!("volume_builder: axis slot {d} carries no coordinate variable");
            return;
        };
        if len != mask_dims[d] {
            eprintln!(
                "volume_builder: axis order is not depth,lat,lon — {name} length {len} vs mask dim {d} length {}; refused",
                mask_dims[d]
            );
            return;
        }
    }

    let mut axes: [Option<Axis>; 3] = [None, None, None];
    for (name, slot, len, endian, size) in &rank1 {
        if *slot >= 3 {
            continue;
        }
        let data = match file.read_dataset(name) {
            Ok(d) => d,
            Err(note) => {
                eprintln!("volume_builder: {name} reads not: {note:?}");
                return;
            }
        };
        let mut values = Vec::with_capacity(*len as usize);
        for i in 0..*len as usize {
            let v = match *size {
                8 => match decode_f64(&data, i * 8, *endian) {
                    Some(v) if v.is_finite() => v,
                    _ => {
                        eprintln!("volume_builder: {name} carries a non-finite value at {i}");
                        return;
                    }
                },
                _ => match decode_f32(&data, i * 4, *endian) {
                    Some(v) if v.is_finite() => v as f64,
                    _ => {
                        eprintln!("volume_builder: {name} carries a non-finite value at {i}");
                        return;
                    }
                },
            };
            values.push(v);
        }
        axes[*slot] = Some(Axis {
            kind: AxisKind::Explicit,
            values,
        });
    }
    let axes = match (axes[0].take(), axes[1].take(), axes[2].take()) {
        (Some(d), Some(la), Some(lo)) => [d, la, lo],
        _ => {
            eprintln!("volume_builder: a coordinate axis is absent");
            return;
        }
    };

    let data = match file.read_dataset(&mask_name) {
        Ok(d) => d,
        Err(note) => {
            eprintln!("volume_builder: {mask_name} reads not: {note:?}");
            return;
        }
    };
    let (_, _, dt) = match file.dataset(&mask_name) {
        Ok(t) => t,
        Err(note) => {
            eprintln!("volume_builder: {mask_name} resolves not: {note:?}");
            return;
        }
    };
    let cells = (mask_dims[0] * mask_dims[1] * mask_dims[2]) as usize;
    let mut out_data = Vec::with_capacity(cells);
    if dt.size == 4 {
        for i in 0..cells {
            let v = match decode_f32(&data, i * 4, dt.endian) {
                Some(v) if v.is_finite() => v,
                _ => {
                    eprintln!("volume_builder: {mask_name} carries a non-finite cell at {i}");
                    return;
                }
            };
            out_data.push(v);
        }
    } else if dt.size == 8 {
        for i in 0..cells {
            let v = match decode_f64(&data, i * 8, dt.endian) {
                Some(v) if v.is_finite() => v as f32,
                _ => {
                    eprintln!("volume_builder: {mask_name} carries a non-finite cell at {i}");
                    return;
                }
            };
            out_data.push(v);
        }
    } else {
        eprintln!(
            "volume_builder: {mask_name} element size {} unread (f32/f64 only)",
            dt.size
        );
        return;
    }

    let volume = Volume {
        dims: [
            mask_dims[0] as u32,
            mask_dims[1] as u32,
            mask_dims[2] as u32,
        ],
        axes,
        data: out_data,
        mask: vec![0u8; cells.div_ceil(8)],
        frame_body: None,
    };
    let bin = volume.write_bin();

    let stem = path
        .rsplit('/')
        .next()
        .unwrap_or(path.as_str())
        .trim_end_matches(".nc");
    let out_path = match &out_override {
        Some(p) => p.clone(),
        None => format!("data/{stem}.volume.bin"),
    };
    if let Err(e) = fs::write(&out_path, &bin) {
        eprintln!("volume_builder: {out_path} writes not: {e}");
        return;
    }
    let digest = omegaflow::sha256::sha256_hex(&bin);
    println!(
        "{out_path} {} {}x{}x{} {}",
        digest,
        mask_dims[0],
        mask_dims[1],
        mask_dims[2],
        bin.len()
    );
    if let Some([value_key, lat_key, lon_key, depth_key]) = verify_live {
        let src = SourceConfig {
            ttl: 0,
            url: path.clone(),
            origin: None,
            terms: None,
            rights_identifier: None,
            rights_scheme: None,
            rights_uri: None,
            frame: Frame::Manifest,
            fanout_center: None,
            format: "volume_netcdf".to_string(),
            extracts: vec![Extract::Volume {
                value_key,
                lat_key,
                lon_key,
                depth_key,
                depth_scale: 1.0,
                name: stem.to_string(),
                frame_body: None,
            }],
            headers: Vec::new(),
            channels: Vec::new(),
            post_body: None,
            target: None,
            catalog: None,
            range: None,
            max_freq: None,
            min_freq: None,
            body: None,
            stations_url: None,
            stations_path: String::new(),
            stations_lat: String::new(),
            stations_lon: String::new(),
            stations_id: String::new(),
            hapi_fill: HashMap::new(),
            flux_from_mag: None,
            abs_mag_from: None,
            catalog_epoch: None,
            repeat_ra_bins: 0,
            fanout_cap: 0,
            stations_flatten: String::new(),
            stations_filter: None,
            fanout_delay: 0,
            sha256: None,
            window: None,
            live_only: false,
            station_code: None,
            cgm_lat: None,
            cgm_source: None,
            geomag_lat: None,
            weberin_role: None,
        };
        match build_netcdf4_volume(&src, &bytes) {
            Some((live_name, live)) => match compare_volumes(&volume, &live) {
                Ok(max_d) => {
                    println!(
                        "volume_builder: live parity holds — {live_name} {}x{}x{} max|Δ|={max_d:e}",
                        live.dims[0], live.dims[1], live.dims[2]
                    );
                }
                Err(field) => {
                    eprintln!("volume_builder: live parity holds not — {field}");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!(
                    "volume_builder: live arm reads void — {} B carry no grid contract for the live parser",
                    bytes.len()
                );
                std::process::exit(1);
            }
        }
    }
    if ci_mode {
        let Some(tag) = tag else {
            eprintln!("volume_builder: --ci-mode needs --tag <netloc>");
            std::process::exit(2);
        };
        if !omegaflow::cdn::upload_release(&tag, &out_path) {
            eprintln!("volume_builder: {out_path} to {tag} manifests not");
            std::process::exit(1);
        }
    }
}
