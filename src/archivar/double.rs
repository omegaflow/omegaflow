use std::collections::HashMap;

pub const MATCH_RADIUS_DEG: f64 = 0.01;
pub const IR_EXCESS_THRESHOLD_MAG: f64 = -0.5;

pub fn angular_sep(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    let (s1, c1) = ra1.to_radians().sin_cos();
    let (s2, c2) = ra2.to_radians().sin_cos();
    let (sd1, cd1) = dec1.to_radians().sin_cos();
    let (sd2, cd2) = dec2.to_radians().sin_cos();
    let dot = cd1 * cd2 * (c1 * c2 + s1 * s2) + sd1 * sd2;
    dot.clamp(-1.0, 1.0).acos().to_degrees()
}

pub const GRID_DEG: f64 = 0.5;

fn cell(ra: f64, dec: f64) -> (i64, i64) {
    (
        (ra / GRID_DEG).floor() as i64,
        ((dec + 90.0) / GRID_DEG).floor() as i64,
    )
}

pub struct ConeCatalog {
    ra: Vec<f64>,
    dec: Vec<f64>,
    val: Vec<f64>,
    name: Vec<[u8; 32]>,
    grid: HashMap<(i64, i64), Vec<u32>>,
}

impl ConeCatalog {
    pub fn with_values(ra: Vec<f64>, dec: Vec<f64>, val: Vec<f64>) -> ConeCatalog {
        let n = ra.len().min(dec.len()).min(val.len());
        Self::build(ra, dec, val, Vec::new(), n)
    }

    pub fn with_names(ra: Vec<f64>, dec: Vec<f64>, name: Vec<[u8; 32]>) -> ConeCatalog {
        let n = ra.len().min(dec.len()).min(name.len());
        Self::build(ra, dec, Vec::new(), name, n)
    }

    fn build(
        ra: Vec<f64>,
        dec: Vec<f64>,
        val: Vec<f64>,
        name: Vec<[u8; 32]>,
        n: usize,
    ) -> ConeCatalog {
        let mut grid: HashMap<(i64, i64), Vec<u32>> = HashMap::new();
        for k in 0..n {
            let c = cell(ra[k], dec[k]);
            grid.entry(c).or_default().push(k as u32);
        }
        let empty_val = val.is_empty();
        let empty_name = name.is_empty();
        ConeCatalog {
            ra: ra[..n].to_vec(),
            dec: dec[..n].to_vec(),
            val: if empty_val {
                Vec::new()
            } else {
                val[..n].to_vec()
            },
            name: if empty_name {
                Vec::new()
            } else {
                name[..n].to_vec()
            },
            grid,
        }
    }

    pub fn len(&self) -> usize {
        self.ra.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ra.is_empty()
    }

    pub fn nearest_val(&self, ra: f64, dec: f64, radius: f64) -> Option<f64> {
        if self.val.is_empty() {
            return None;
        }
        self.nearest(ra, dec, radius).map(|k| self.val[k])
    }

    pub fn nearest_name(&self, ra: f64, dec: f64, radius: f64) -> Option<[u8; 32]> {
        if self.name.is_empty() {
            return None;
        }
        self.nearest(ra, dec, radius).map(|k| self.name[k])
    }

    pub fn contains(&self, ra: f64, dec: f64, radius: f64) -> bool {
        self.nearest(ra, dec, radius).is_some()
    }

    fn nearest(&self, ra: f64, dec: f64, radius: f64) -> Option<usize> {
        let rad_cells = (radius / GRID_DEG).ceil() as i64 + 1;
        let (cx, cy) = cell(ra, dec);
        let mut best: Option<(f64, usize)> = None;
        for dx in -rad_cells..=rad_cells {
            for dy in -rad_cells..=rad_cells {
                let key = (cx + dx, cy + dy);
                let Some(bucket) = self.grid.get(&key) else {
                    continue;
                };
                for &b in bucket {
                    let k = b as usize;
                    let s = angular_sep(ra, dec, self.ra[k], self.dec[k]);
                    if s <= radius && best.map_or(true, |(bs, _)| s < bs) {
                        best = Some((s, k));
                    }
                }
            }
        }
        best.map(|(_, k)| k)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnomalyRow {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub excess_mag: f64,
    pub gaia_color: f64,
    pub radio_flux: f64,
    pub tns_z: f64,
    pub excluded: bool,
    pub ex_name: [u8; 32],
}

pub fn crossmatch(
    ir_ra: &[f64],
    ir_dec: &[f64],
    ir_excess: &[f64],
    gaia: &ConeCatalog,
    radio: &ConeCatalog,
    tns: &ConeCatalog,
    excl: &ConeCatalog,
    radius: f64,
    only_excess: bool,
) -> Vec<AnomalyRow> {
    let mut rows = Vec::new();
    for i in 0..ir_ra.len() {
        let is_excess = ir_excess[i].is_finite() && ir_excess[i] < IR_EXCESS_THRESHOLD_MAG;
        if only_excess && !is_excess {
            continue;
        }
        let gaia_color = gaia
            .nearest_val(ir_ra[i], ir_dec[i], radius)
            .unwrap_or(f64::NAN);
        let radio_flux = radio
            .nearest_val(ir_ra[i], ir_dec[i], radius)
            .unwrap_or(f64::NAN);
        let tns_z = tns
            .nearest_val(ir_ra[i], ir_dec[i], radius)
            .unwrap_or(f64::NAN);
        let ex_name = excl.nearest_name(ir_ra[i], ir_dec[i], radius);
        let excluded = ex_name.is_some();
        rows.push(AnomalyRow {
            ra_deg: ir_ra[i],
            dec_deg: ir_dec[i],
            excess_mag: ir_excess[i],
            gaia_color,
            radio_flux,
            tns_z,
            excluded,
            ex_name: ex_name.unwrap_or([0u8; 32]),
        });
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angular_sep_zero_and_known() {
        assert!(angular_sep(10.0, 20.0, 10.0, 20.0) < 1e-5);
        assert!((angular_sep(0.0, 0.0, 0.0, 1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cone_catalog_nearest_within_radius() {
        let cat = ConeCatalog::with_values(
            vec![313.261, 313.2605 + 5.0],
            vec![38.5995, 38.5991],
            vec![2.135, 0.5],
        );
        assert_eq!(cat.nearest_val(313.2605, 38.5991, 0.01), Some(2.135));
    }

    #[test]
    fn cone_catalog_returns_none_beyond_radius() {
        let cat = ConeCatalog::with_values(vec![313.2605 + 0.05], vec![38.5991], vec![1.0]);
        assert!(cat.nearest_val(313.2605, 38.5991, 0.01).is_none());
    }

    #[test]
    fn crossmatch_honors_threshold_and_radius() {
        let gaia = ConeCatalog::with_values(vec![313.2605], vec![38.5991], vec![2.135]);
        let radio = ConeCatalog::with_values(vec![313.2605 + 0.05], vec![38.5991], vec![1.0e-26]);
        let tns = ConeCatalog::with_values(vec![313.2605], vec![38.5991], vec![0.027172]);
        let excl = ConeCatalog::with_names(vec![], vec![], vec![]);
        let rows = crossmatch(
            &[313.2605],
            &[38.5991],
            &[-0.624],
            &gaia,
            &radio,
            &tns,
            &excl,
            0.01,
            true,
        );
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].gaia_color, 2.135);
        assert_eq!(rows[0].tns_z, 0.027172);
        assert!(rows[0].radio_flux.is_nan());
        assert!(!rows[0].excluded);
    }

    #[test]
    fn crossmatch_applies_exclusion_filter() {
        let mut exname = [0u8; 32];
        exname[..8].copy_from_slice(b"ASASSN-V");
        let gaia = ConeCatalog::with_values(vec![313.2605], vec![38.5991], vec![2.135]);
        let radio = ConeCatalog::with_values(vec![], vec![], vec![]);
        let tns = ConeCatalog::with_values(vec![], vec![], vec![]);
        let excl = ConeCatalog::with_names(vec![313.261], vec![38.5995], vec![exname]);
        let rows = crossmatch(
            &[313.2605],
            &[38.5991],
            &[-0.624],
            &gaia,
            &radio,
            &tns,
            &excl,
            0.01,
            true,
        );
        assert_eq!(rows.len(), 1);
        assert!(rows[0].excluded);
        assert_eq!(&rows[0].ex_name[..8], b"ASASSN-V");
    }

    #[test]
    fn sky_sweep_includes_non_excess_when_flagged() {
        let gaia =
            ConeCatalog::with_values(vec![313.2605, 10.0], vec![38.5991, 20.0], vec![2.135, 0.5]);
        let radio = ConeCatalog::with_values(vec![], vec![], vec![]);
        let tns = ConeCatalog::with_values(vec![], vec![], vec![]);
        let excl = ConeCatalog::with_names(vec![], vec![], vec![]);
        let rows = crossmatch(
            &[313.2605, 10.0],
            &[38.5991, 20.0],
            &[-0.624, 0.2],
            &gaia,
            &radio,
            &tns,
            &excl,
            0.01,
            false,
        );
        assert_eq!(rows.len(), 2, "full sky-sweep keeps all positions");
    }
}
