use crate::archivar::fits::{FitsWcs, WcsProjection};
use crate::mathematikerin::healpix::ang2pix_nest;
use std::collections::HashMap;
use std::f64::consts::PI;

pub const SUBDIVISION: u32 = 16;

#[derive(Clone, Copy, Default, Debug)]
pub struct PixelAccum {
    pub count_sr: f64,
    pub area_sr: f64,
}

pub struct ZenithalRegrid {
    wcs: FitsWcs,
    projection: WcsProjection,
    nside: i64,
    tangent_pixel_sr: f64,
    hpixel_sr: f64,
    acc: HashMap<u32, PixelAccum>,
    unmapped: u64,
    pending_b: Option<usize>,
    pending_row: Vec<u32>,
}

impl ZenithalRegrid {
    pub fn new(wcs: FitsWcs, nside: i64) -> Option<Self> {
        let projection = wcs.projection();
        if projection != WcsProjection::Tan && projection != WcsProjection::Sin {
            return None;
        }
        if nside <= 0 || nside & (nside - 1) != 0 {
            return None;
        }
        if 12 * nside * nside > u32::MAX as i64 {
            return None;
        }
        let tangent_pixel_sr = wcs.tangent_pixel_sr()?;
        let hpixel_sr = 4.0 * PI / (12.0 * (nside * nside) as f64);
        Some(Self {
            wcs,
            projection,
            nside,
            tangent_pixel_sr,
            hpixel_sr,
            acc: HashMap::new(),
            unmapped: 0,
            pending_b: None,
            pending_row: Vec::new(),
        })
    }

    pub fn nside(&self) -> i64 {
        self.nside
    }

    pub fn hpixel_sr(&self) -> f64 {
        self.hpixel_sr
    }

    pub fn unmapped_pixels(&self) -> u64 {
        self.unmapped
    }

    fn hpx_world(&self, x: f64, y: f64) -> Option<u32> {
        let (ra, dec) = self.wcs.world(x, y)?;
        if !(ra.is_finite() && dec.is_finite() && (-90.0..=90.0).contains(&dec)) {
            return None;
        }
        let theta = (90.0 - dec).to_radians();
        let phi = ra.rem_euclid(360.0).to_radians();
        let p = ang2pix_nest(self.nside, theta, phi)?;
        if p < 0 || p >= 12 * self.nside * self.nside {
            return None;
        }
        Some(p as u32)
    }

    fn solid_at(&self, x: f64, y: f64, cell_area: f64) -> Option<f64> {
        let (xi_deg, eta_deg) = self.wcs.xi_eta_deg(x, y)?;
        let xi = xi_deg.to_radians();
        let eta = eta_deg.to_radians();
        let r2 = xi * xi + eta * eta;
        let projection = match self.projection {
            WcsProjection::Tan => (1.0 + r2).powf(-1.5),
            WcsProjection::Sin => {
                if r2 >= 1.0 {
                    return None;
                }
                (1.0 - r2).powf(-0.5)
            }
            WcsProjection::Linear => return None,
        };
        Some(self.tangent_pixel_sr * cell_area * projection)
    }

    fn lattice_row(&mut self, b: usize, width: usize) -> &[u32] {
        if self.pending_b == Some(b) && self.pending_row.len() == width + 1 {
            return &self.pending_row;
        }
        self.pending_row.clear();
        self.pending_row.reserve(width + 1);
        for a in 0..=width {
            let x = a as f64 + 0.5;
            let y = b as f64 + 0.5;
            let h = self.hpx_world(x, y).unwrap_or(u32::MAX);
            self.pending_row.push(h);
        }
        self.pending_b = Some(b);
        &self.pending_row
    }

    pub fn push_row(&mut self, y: usize, values: &[f64], measured: &[bool]) {
        let width = values.len().min(measured.len());
        let bottom = self.lattice_row(y, width).to_vec();
        let top = self.lattice_row(y + 1, width).to_vec();
        let sub = SUBDIVISION as f64;
        let subcell = 1.0 / (sub * sub);
        for x in 0..width {
            if measured.get(x) != Some(&true) {
                continue;
            }
            let lb = bottom[x];
            let rb = bottom[x + 1];
            let lt = top[x];
            let rt = top[x + 1];
            if lb != u32::MAX && lb == rb && rb == lt && lt == rt {
                if let Some(solid) = self.solid_at(x as f64 + 1.0, y as f64 + 1.0, 1.0) {
                    let e = self.acc.entry(lb).or_default();
                    e.count_sr += values[x] * solid;
                    e.area_sr += solid;
                }
                continue;
            }
            let mut placed = false;
            for j in 0..SUBDIVISION {
                for i in 0..SUBDIVISION {
                    let xc = x as f64 + (i as f64 + 0.5) / sub + 0.5;
                    let yc = y as f64 + (j as f64 + 0.5) / sub + 0.5;
                    let h = match self.hpx_world(xc, yc) {
                        Some(h) => h,
                        None => {
                            self.unmapped += 1;
                            continue;
                        }
                    };
                    let solid = match self.solid_at(xc, yc, subcell) {
                        Some(s) => s,
                        None => {
                            self.unmapped += 1;
                            continue;
                        }
                    };
                    let e = self.acc.entry(h).or_default();
                    e.count_sr += values[x] * solid;
                    e.area_sr += solid;
                    placed = true;
                }
            }
            if !placed {
                self.unmapped += 1;
            }
        }
    }

    pub fn into_accum(self) -> HashMap<u32, PixelAccum> {
        self.acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::fits::{FitsWcs, WcsProjection};

    fn solid_projection(wcs: &FitsWcs, xp: f64, yp: f64) -> Option<f64> {
        let (xi_deg, eta_deg) = wcs.xi_eta_deg(xp, yp)?;
        let xi = xi_deg.to_radians();
        let eta = eta_deg.to_radians();
        let r2 = xi * xi + eta * eta;
        match wcs.projection() {
            WcsProjection::Tan => Some((1.0 + r2).powf(-1.5)),
            WcsProjection::Sin => {
                if r2 >= 1.0 {
                    None
                } else {
                    Some((1.0 - r2).powf(-0.5))
                }
            }
            WcsProjection::Linear => None,
        }
    }

    fn expected_integral(
        wcs: &FitsWcs,
        width: usize,
        height: usize,
        measured: &[bool],
        values: &[f64],
    ) -> f64 {
        let sr_per_pixel = wcs.tangent_pixel_sr().unwrap();
        let mut total = 0.0;
        for y in 0..height {
            for x in 0..width {
                if measured[y * width + x] {
                    let xp = x as f64 + 1.0;
                    let yp = y as f64 + 1.0;
                    let projection = solid_projection(wcs, xp, yp).unwrap();
                    total += values[y * width + x] * sr_per_pixel * projection;
                }
            }
        }
        total
    }

    fn run_plane(
        wcs: &FitsWcs,
        nside: i64,
        width: usize,
        height: usize,
        measured: &[bool],
        values: &[f64],
    ) -> (HashMap<u32, PixelAccum>, f64) {
        let expected = expected_integral(wcs, width, height, measured, values);
        let mut g = ZenithalRegrid::new(wcs.clone(), nside).unwrap();
        for y in 0..height {
            let mut row_v = Vec::with_capacity(width);
            let mut row_m = Vec::with_capacity(width);
            for x in 0..width {
                row_v.push(values[y * width + x]);
                row_m.push(measured[y * width + x]);
            }
            g.push_row(y, &row_v, &row_m);
        }
        (g.into_accum(), expected)
    }

    #[test]
    fn refuses_linear_and_degenerate_accepts_tan_and_sin() {
        let tan = FitsWcs::tan(0.0, 0.0, 1.0, 1.0, [[0.01, 0.0], [0.0, 0.01]]);
        let sin = FitsWcs::sin(0.0, 0.0, 1.0, 1.0, [[0.01, 0.0], [0.0, 0.01]]);
        assert!(ZenithalRegrid::new(tan.clone(), 4096).is_some());
        assert!(ZenithalRegrid::new(sin.clone(), 4096).is_some());
        assert!(ZenithalRegrid::new(
            FitsWcs::tan(0.0, 0.0, 1.0, 1.0, [[0.0, 0.0], [0.0, 0.0]]),
            4096
        )
        .is_none());
        assert!(ZenithalRegrid::new(tan.clone(), 0).is_none());
        assert!(ZenithalRegrid::new(tan.clone(), 3).is_none());
    }

    #[test]
    fn sky_integral_is_preserved_across_hpixel_boundaries() {
        let wcs = FitsWcs::tan(20.0, 30.0, 201.0, 201.0, [[0.005, 0.0], [0.0, 0.005]]);
        let width = 400;
        let height = 400;
        let values = vec![5.0; width * height];
        let measured = vec![true; width * height];

        let (acc, expected) = run_plane(&wcs, 512, width, height, &measured, &values);
        let got: f64 = acc.values().map(|a| a.count_sr).sum();
        assert!(
            (got - expected).abs() / expected < 1e-6,
            "count-sky integral {got} vs expected {expected}"
        );
        assert!(!acc.is_empty());
    }

    #[test]
    fn sin_sky_integral_is_preserved_across_hpixel_boundaries() {
        let wcs = FitsWcs::sin(20.0, 30.0, 201.0, 201.0, [[0.005, 0.0], [0.0, 0.005]]);
        let width = 400;
        let height = 400;
        let values = vec![3.0; width * height];
        let measured = vec![true; width * height];

        let (acc, expected) = run_plane(&wcs, 512, width, height, &measured, &values);
        let got: f64 = acc.values().map(|a| a.count_sr).sum();
        assert!(
            (got - expected).abs() / expected < 1e-6,
            "sin count-sky integral {got} vs expected {expected}"
        );
        assert!(!acc.is_empty());
    }

    #[test]
    fn absent_never_contributes_measured_zero_presence_differs() {
        let wcs = FitsWcs::tan(0.0, 0.0, 151.0, 151.0, [[0.02, 0.0], [0.0, 0.02]]);
        let width = 300;
        let height = 300;
        let lo = 40usize;
        let hi = 260usize;
        let value = 7.0;
        let hp_sr = ZenithalRegrid::new(wcs.clone(), 32).unwrap().hpixel_sr();

        let mut measured = vec![false; width * height];
        let mut values = vec![0.0; width * height];
        for y in lo..hi {
            for x in lo..hi {
                measured[y * width + x] = true;
                values[y * width + x] = value;
            }
        }
        let (acc, expected) = run_plane(&wcs, 32, width, height, &measured, &values);
        let got: f64 = acc.values().map(|a| a.count_sr).sum();
        assert!(
            (got - expected).abs() / expected < 1e-6,
            "central measured box, absent ring excluded: {got} vs {expected}"
        );
        let max_area = acc
            .values()
            .map(|a| a.area_sr)
            .fold(f64::NEG_INFINITY, f64::max);
        assert!(
            max_area > 0.9 * hp_sr,
            "a near-fully covered hpixel is present (the quadrature deficit scales as pixel/(subdivision x hpixel)); max_area {max_area} hpixel {hp_sr}"
        );
        assert!(
            acc.values()
                .all(|a| (a.count_sr - value * a.area_sr).abs() <= 1e-9 * value * a.area_sr),
            "every present hpixel carries exactly the uniform plane value, absent areas contribute nothing"
        );

        let mut zero_values = vec![0.0; width * height];
        for y in lo..hi {
            for x in lo..hi {
                zero_values[y * width + x] = 0.0;
            }
        }
        let (zero_acc, _) = run_plane(&wcs, 32, width, height, &measured, &zero_values);
        assert!(
            zero_acc
                .values()
                .any(|a| a.area_sr > 0.9 * hp_sr && a.count_sr == 0.0),
            "a measured-real-zero box leaves a present pixel with zero count"
        );

        let all_absent = vec![false; width * height];
        let (empty_acc, _) = run_plane(&wcs, 32, width, height, &all_absent, &values);
        assert!(
            empty_acc.is_empty(),
            "a fully absent plane leaves no record behind"
        );
    }

    #[test]
    fn output_is_deterministic_and_row_order_independent() {
        let wcs = FitsWcs::tan(10.0, -15.0, 101.0, 101.0, [[0.005, 0.0], [0.0, 0.005]]);
        let width = 200;
        let height = 200;
        let mut values = Vec::new();
        let mut measured = Vec::new();
        for y in 0..height {
            for x in 0..width {
                values.push(((x * 7 + y) % 11) as f64);
                measured.push((x + y) % 3 != 0);
            }
        }
        let run = || {
            let (acc, _) = run_plane(&wcs, 64, width, height, &measured, &values);
            let mut acc_keys: Vec<(u32, u64, u64)> = acc
                .into_iter()
                .map(|(k, a)| (k, a.count_sr.to_bits(), a.area_sr.to_bits()))
                .collect();
            acc_keys.sort();
            acc_keys
        };
        assert_eq!(run(), run());
    }
}
