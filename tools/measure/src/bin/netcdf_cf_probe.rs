use omegaflow::archivar::netcdf::{NetcdfFile, NetcdfType};

fn attr_num(file: &NetcdfFile, name: &str, key: &str) -> Option<f64> {
    let var = file.var(name)?;
    let a = var.attrs.iter().find(|a| a.name == key)?;
    match a.nc_type {
        NetcdfType::Float if a.raw.len() >= 4 => {
            Some(
                f32::from_bits(u32::from_be_bytes([a.raw[0], a.raw[1], a.raw[2], a.raw[3]])) as f64,
            )
        }
        NetcdfType::Double if a.raw.len() >= 8 => Some(f64::from_bits(u64::from_be_bytes([
            a.raw[0], a.raw[1], a.raw[2], a.raw[3], a.raw[4], a.raw[5], a.raw[6], a.raw[7],
        ]))),
        NetcdfType::Short if a.raw.len() >= 2 => {
            Some(i16::from_be_bytes([a.raw[0], a.raw[1]]) as f64)
        }
        NetcdfType::Int if a.raw.len() >= 4 => {
            Some(i32::from_be_bytes([a.raw[0], a.raw[1], a.raw[2], a.raw[3]]) as f64)
        }
        _ => None,
    }
}

fn raw_values(file: &NetcdfFile, buf: &[u8], name: &str, ty: NetcdfType) -> Option<Vec<f64>> {
    Some(match ty {
        NetcdfType::Short => file
            .values_i16(buf, name)?
            .iter()
            .map(|v| *v as f64)
            .collect(),
        NetcdfType::Int => file
            .values_i32(buf, name)?
            .iter()
            .map(|v| *v as f64)
            .collect(),
        NetcdfType::Float => file
            .values_f32(buf, name)?
            .iter()
            .map(|v| *v as f64)
            .collect(),
        NetcdfType::Double => file.values_f64(buf, name)?,
        _ => return None,
    })
}

fn run(path: &str) -> bool {
    let Some(buf) = std::fs::read(path).ok() else {
        println!("{path}: unreadable");
        return false;
    };
    match omegaflow::archivar::netcdf::NetcdfFile::parse(&buf) {
        Ok(file) => run_classic(path, &buf, &file),
        Err(_) => run_netcdf4(path, &buf),
    }
}

fn run_netcdf4(path: &str, buf: &[u8]) -> bool {
    use omegaflow::archivar::hdf5::Hdf5File;
    use omegaflow::archivar::nc4;
    let Ok(file) = Hdf5File::parse(buf) else {
        println!("{path}: neither classic NetCDF nor HDF5");
        return false;
    };
    let Ok(vars) = nc4::variables(&file) else {
        println!("{path}: HDF5 without a variable table");
        return false;
    };
    let mut packed = 0usize;
    let mut listed = 0usize;
    for v in &vars {
        if v.is_dimension_scale {
            continue;
        }
        listed += 1;
        if !nc4::packed(&file, &v.name) {
            continue;
        }
        packed += 1;
        let scale = file.attr_f64(&v.name, "scale_factor");
        let offset = file.attr_f64(&v.name, "add_offset");
        match nc4::values(&file, &v.name) {
            Ok(values) => {
                let lo = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let hi = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                println!(
                    "{path}: {} scale {scale:?} offset {offset:?} n {} range [{lo:.6e}, {hi:.6e}]",
                    v.name,
                    values.len()
                );
            }
            Err(e) => {
                println!("{path}: {} values unread {e:?}", v.name);
                return false;
            }
        }
    }
    println!("{path}: netcdf4, {listed} variables, {packed} packed");
    true
}

fn run_classic(path: &str, buf: &[u8], file: &omegaflow::archivar::netcdf::NetcdfFile) -> bool {
    let mut checked = 0usize;
    let mut worst = 0.0f64;
    for var in &file.vars {
        let scale = attr_num(&file, &var.name, "scale_factor");
        let offset = attr_num(&file, &var.name, "add_offset");
        if scale.is_none() && offset.is_none() {
            continue;
        }
        let Some(packed) = file.values_numeric(&buf, &var.name) else {
            println!(
                "{path}: {} has the attributes, the values are absent",
                var.name
            );
            return false;
        };
        let Some(raw) = raw_values(&file, &buf, &var.name, var.nc_type) else {
            continue;
        };
        if raw.len() != packed.len() {
            println!(
                "{path}: {} count mismatch raw {} packed {}",
                var.name,
                raw.len(),
                packed.len()
            );
            return false;
        }
        let mut local = 0.0f64;
        for (i, r) in raw.iter().enumerate() {
            let want = match (scale, offset) {
                (Some(s), Some(o)) => r * s + o,
                (Some(s), None) => r * s,
                (None, Some(o)) => r + o,
                (None, None) => continue,
            };
            let d = (want - packed[i]).abs();
            if d > local {
                local = d;
            }
        }
        let lo = packed.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = packed.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        println!(
            "{path}: {} type {:?} scale {scale:?} offset {offset:?} n {} range [{lo:.6e}, {hi:.6e}] raw-vs-unpacked {local:.3e}",
            var.name,
            var.nc_type,
            packed.len()
        );
        if local > worst {
            worst = local;
        }
        checked += 1;
    }
    println!("{path}: {checked} packed variables, worst raw-vs-unpacked {worst:.3e}");
    checked > 0 && worst < 1e-9
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: netcdf_cf_probe <file.nc> [...]");
        std::process::exit(2);
    }
    let mut ok = true;
    for a in &args {
        ok &= run(a);
    }
    std::process::exit(if ok { 0 } else { 1 });
}
