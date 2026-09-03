use crate::hdf5::{Hdf5File, Hdf5Note};
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
        .map(|v| {
            let len = v.dims.first().copied().unwrap_or(0);
            (v.name, len)
        })
        .collect())
}

pub fn time_row_month(time_days: f64, epoch: (u32, u32, u32)) -> Option<(u32, u32)> {
    let base = days_from_civil(epoch.0 as i64, epoch.1 as i64, epoch.2 as i64)?;
    let (y, m, _) = civil_from_days(base + time_days.floor() as i64)?;
    Some((y, m))
}
