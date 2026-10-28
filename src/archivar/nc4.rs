use crate::hdf5::{Hdf5File, Hdf5Note, decode_numeric};
use crate::lsk::days_from_civil;
use crate::spectral::civil_from_days;

pub struct Nc4Variable {
    pub name: String,
    pub dims: Vec<u64>,
    pub is_dimension_scale: bool,
}

pub fn variables(file: &Hdf5File) -> Result<Vec<Nc4Variable>, Hdf5Note> {
    let root = file.root()?;
    let mut out = Vec::new();
    for link in &root.links {
        if link.addr == u64::MAX {
            continue;
        }
        let obj = file.resolve(&link.name)?;
        let Some(ds) = &obj.dataspace else {
            continue;
        };
        let is_scale = obj.attrs.iter().any(|a| {
            a.name == "CLASS" && a.datatype.class == 3 && a.data.starts_with(b"DIMENSION_SCALE")
        });
        out.push(Nc4Variable {
            name: link.name.clone(),
            dims: ds.dims.clone(),
            is_dimension_scale: is_scale,
        });
    }
    Ok(out)
}

pub fn dimensions(file: &Hdf5File) -> Result<Vec<(String, u64)>, Hdf5Note> {
    Ok(variables(file)?
        .into_iter()
        .filter(|v| v.is_dimension_scale)
        .filter_map(|v| v.dims.first().copied().map(|len| (v.name, len)))
        .collect())
}

pub fn packed(file: &Hdf5File, name: &str) -> bool {
    file.attr_f64(name, "scale_factor").is_some() || file.attr_f64(name, "add_offset").is_some()
}

pub fn values(file: &Hdf5File, name: &str) -> Result<Vec<f64>, Hdf5Note> {
    let (_, ds, dt) = file.dataset(name)?;
    let raw = file.read_dataset(name)?;
    let count: usize = ds.dims.iter().fold(1usize, |a, d| a * (*d as usize));
    let scale = file.attr_f64(name, "scale_factor");
    let offset = file.attr_f64(name, "add_offset");
    let fill = file
        .attr_f64(name, "_FillValue")
        .or_else(|| file.attr_f64(name, "missing_value"));
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let v = decode_numeric(&raw, i, dt)?;
        if let Some(f) = fill
            && v == f
        {
            out.push(f64::NAN);
            continue;
        }
        let packed = match (scale, offset) {
            (Some(s), Some(o)) => v * s + o,
            (Some(s), None) => v * s,
            (None, Some(o)) => v + o,
            (None, None) => v,
        };
        out.push(packed);
    }
    Ok(out)
}

pub fn time_row_month(time_days: f64, epoch: (u32, u32, u32)) -> Option<(u32, u32)> {
    let base = days_from_civil(epoch.0 as i64, epoch.1 as i64, epoch.2 as i64)?;
    let (y, m, _) = civil_from_days(base + time_days.floor() as i64)?;
    Some((y, m))
}


#[cfg(test)]
mod tests {
    use super::{packed, values};
    use crate::hdf5::Hdf5File;

    #[test]
    fn a_packed_netcdf4_file_unpacks_to_the_classic_values() {
        let bytes = include_bytes!("kernels/cf_air_packed4.nc");
        let file = Hdf5File::parse(bytes).expect("netcdf-4 file");
        assert!(packed(&file, "air"), "scale_factor and add_offset are read");
        let air = values(&file, "air").expect("air values");
        assert_eq!(air.len(), 126_144);
        assert!((air[0] + 28.13465).abs() < 5e-3, "first value {}", air[0]);
        let lo = air.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = air.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert!((lo + 62.64235).abs() < 5e-3, "minimum {lo}");
        assert!((hi - 39.73564).abs() < 5e-3, "maximum {hi}");
    }
}
