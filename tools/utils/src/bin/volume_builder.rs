use std::collections::HashSet;
use std::env;
use std::fs;

use omegaflow::hdf5::{decode_f32, decode_f64, Endian, Hdf5File};
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
    let Some(path) = args.first() else {
        eprintln!("usage: volume_builder <file.n4c.nc>");
        return;
    };
    let bytes = match fs::read(path) {
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
    let mut rank1: Vec<(String, usize, u64, Endian)> = Vec::new();
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
        } else if ds.dims.len() == 1 && dt.size == 8 {
            let slot = axis_slot(&name).unwrap_or(usize::MAX);
            rank1.push((name, slot, ds.dims[0], dt.endian));
        }
    }
    let Some((mask_name, mask_dims)) = mask else {
        eprintln!("volume_builder: no rank-3 float dataset in {path}");
        return;
    };

    let mut slot_of_dim: [Option<usize>; 3] = [None, None, None];
    for d in 0..3 {
        for (_, slot, len, _) in &rank1 {
            if *slot < 3 && *len == mask_dims[d] {
                if slot_of_dim[d].is_some() {
                    eprintln!("volume_builder: axis {d} matches more than one coordinate variable");
                    return;
                }
                slot_of_dim[d] = Some(*slot);
            }
        }
    }
    let ordered = slot_of_dim == [Some(0), Some(1), Some(2)];
    if !ordered {
        eprintln!(
            "volume_builder: axis order is not depth,lat,lon — measured slots {:?}; refused",
            slot_of_dim
        );
        return;
    }

    let mut axes: [Option<Axis>; 3] = [None, None, None];
    for (name, slot, len, endian) in &rank1 {
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
            let v = match decode_f64(&data, i * 8, *endian) {
                Some(v) if v.is_finite() => v,
                _ => {
                    eprintln!("volume_builder: {name} carries a non-finite value at {i}");
                    return;
                }
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
    };
    let bin = volume.write_bin();

    let stem = path
        .rsplit('/')
        .next()
        .unwrap_or(path.as_str())
        .trim_end_matches(".nc");
    let out_path = format!("data/{stem}.volume.bin");
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
}
