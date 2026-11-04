#[derive(Clone, Debug)]
pub struct Astrometry {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub pm_ra_mas_yr: f64,
    pub pm_dec_mas_yr: f64,
    pub ra_error_mas: f64,
    pub dec_error_mas: f64,
    pub pm_ra_error_mas_yr: f64,
    pub pm_dec_error_mas_yr: f64,
}

const ARCSEC_PER_DEG: f64 = 3600.0;

pub fn propagate(a: &Astrometry, dt_yr: f64) -> Option<(f64, f64)> {
    if !a.ra_deg.is_finite()
        || !a.dec_deg.is_finite()
        || !a.pm_ra_mas_yr.is_finite()
        || !a.pm_dec_mas_yr.is_finite()
        || !dt_yr.is_finite()
        || a.dec_deg.abs() >= 90.0
        || dt_yr.abs() > 1000.0
    {
        return None;
    }
    let cosd = a.dec_deg.to_radians().cos();
    if cosd.abs() < 1e-12 {
        return None;
    }
    let dra_deg = a.pm_ra_mas_yr * dt_yr / (ARCSEC_PER_DEG * 1000.0 * cosd);
    let ddec_deg = a.pm_dec_mas_yr * dt_yr / (ARCSEC_PER_DEG * 1000.0);
    let dec = a.dec_deg + ddec_deg;
    if dec.abs() > 90.0 {
        return None;
    }
    Some(((a.ra_deg + dra_deg).rem_euclid(360.0), dec))
}

pub fn propagated_sigma_mas(a: &Astrometry, dt_yr: f64) -> Option<(f64, f64)> {
    let ra = a.ra_error_mas.hypot(a.pm_ra_error_mas_yr * dt_yr);
    let dec = a.dec_error_mas.hypot(a.pm_dec_error_mas_yr * dt_yr);
    if !ra.is_finite() || !dec.is_finite() || ra <= 0.0 || dec <= 0.0 {
        return None;
    }
    Some((ra, dec))
}

pub fn normalized_pm_residual(a: &Astrometry, b: &Astrometry) -> Option<(f64, f64)> {
    let s_ra = a.pm_ra_error_mas_yr.hypot(b.pm_ra_error_mas_yr);
    let s_dec = a.pm_dec_error_mas_yr.hypot(b.pm_dec_error_mas_yr);
    if s_ra <= 0.0 || s_dec <= 0.0 {
        return None;
    }
    Some((
        (a.pm_ra_mas_yr - b.pm_ra_mas_yr).abs() / s_ra,
        (a.pm_dec_mas_yr - b.pm_dec_mas_yr).abs() / s_dec,
    ))
}

pub fn normalized_parallax_residual(
    a_parallax: f64,
    a_error: f64,
    b_parallax: f64,
    b_error: f64,
) -> Option<f64> {
    let s = a_error.hypot(b_error);
    if s <= 0.0 || !a_parallax.is_finite() || !b_parallax.is_finite() {
        return None;
    }
    Some((a_parallax - b_parallax).abs() / s)
}

pub fn normalized_position_residual(
    from: &Astrometry,
    dt_yr: f64,
    to: &Astrometry,
) -> Option<(f64, f64)> {
    let (ra, dec) = propagate(from, dt_yr)?;
    let (s_ra_from, s_dec_from) = propagated_sigma_mas(from, dt_yr)?;
    let s_ra = s_ra_from.hypot(to.ra_error_mas);
    let s_dec = s_dec_from.hypot(to.dec_error_mas);
    if s_ra <= 0.0 || s_dec <= 0.0 || !to.dec_deg.is_finite() {
        return None;
    }
    let cosd = to.dec_deg.to_radians().cos();
    let mut d_ra = (to.ra_deg - ra).rem_euclid(360.0);
    if d_ra > 180.0 {
        d_ra -= 360.0;
    }
    let sep_ra = d_ra * cosd * ARCSEC_PER_DEG * 1000.0;
    let sep_dec = (to.dec_deg - dec) * ARCSEC_PER_DEG * 1000.0;
    Some((sep_ra.abs() / s_ra, sep_dec.abs() / s_dec))
}

pub fn median_and_p90(values: &[f64]) -> Option<(f64, f64)> {
    let mut v: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    let index = |p: f64| ((v.len() - 1) as f64 * p).round() as usize;
    Some((v[index(0.5)], v[index(0.9)]))
}

#[derive(Clone, Debug)]
pub struct CatalogStar {
    pub hip: u32,
    pub gaia: Astrometry,
    pub gaia_parallax_mas: f64,
    pub gaia_parallax_error_mas: f64,
    pub hipparcos: Astrometry,
    pub hipparcos_parallax_mas: f64,
    pub hipparcos_parallax_error_mas: f64,
    pub tycho: Option<Astrometry>,
}

const CATALOG: &str = include_str!("kernels/star_catalog.tsv");

fn cell(cols: &[&str], index: usize) -> Option<f64> {
    let v: f64 = cols.get(index)?.trim().parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

pub fn parse(text: &str) -> Option<Vec<CatalogStar>> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 30 {
            return None;
        }
        let hip: u32 = cols[0].trim().parse().ok()?;
        let gaia = Astrometry {
            ra_deg: cell(&cols, 2)?,
            dec_deg: cell(&cols, 3)?,
            pm_ra_mas_yr: cell(&cols, 5)?,
            pm_dec_mas_yr: cell(&cols, 6)?,
            ra_error_mas: cell(&cols, 7)?,
            dec_error_mas: cell(&cols, 8)?,
            pm_ra_error_mas_yr: cell(&cols, 10)?,
            pm_dec_error_mas_yr: cell(&cols, 11)?,
        };
        let hipparcos = Astrometry {
            ra_deg: cell(&cols, 12)?,
            dec_deg: cell(&cols, 13)?,
            pm_ra_mas_yr: cell(&cols, 15)?,
            pm_dec_mas_yr: cell(&cols, 16)?,
            ra_error_mas: cell(&cols, 17)?,
            dec_error_mas: cell(&cols, 18)?,
            pm_ra_error_mas_yr: cell(&cols, 20)?,
            pm_dec_error_mas_yr: cell(&cols, 21)?,
        };
        let tycho = match (cell(&cols, 22), cell(&cols, 26)) {
            (Some(ra_deg), Some(pm_ra_mas_yr)) => Some(Astrometry {
                ra_deg,
                dec_deg: cell(&cols, 23)?,
                pm_ra_mas_yr,
                pm_dec_mas_yr: cell(&cols, 27)?,
                ra_error_mas: cell(&cols, 24)?,
                dec_error_mas: cell(&cols, 25)?,
                pm_ra_error_mas_yr: cell(&cols, 28)?,
                pm_dec_error_mas_yr: cell(&cols, 29)?,
            }),
            _ => None,
        };
        out.push(CatalogStar {
            hip,
            gaia,
            gaia_parallax_mas: cell(&cols, 4)?,
            gaia_parallax_error_mas: cell(&cols, 9)?,
            hipparcos,
            hipparcos_parallax_mas: cell(&cols, 14)?,
            hipparcos_parallax_error_mas: cell(&cols, 19)?,
            tycho,
        });
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn fixture() -> Option<Vec<CatalogStar>> {
    parse(CATALOG)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAIA_EPOCH: f64 = 2016.0;
    const HIPPARCOS_EPOCH: f64 = 1991.25;
    const TYCHO_EPOCH: f64 = 2000.0;

    fn pair_max(a: (f64, f64)) -> f64 {
        a.0.max(a.1)
    }

    #[test]
    fn the_fixture_carries_three_catalogues_for_eighty_stars() {
        let stars = fixture().expect("catalogue fixture");
        assert_eq!(stars.len(), 80, "bright-star sample, G 8 to 10.5");
        let with_tycho = stars.iter().filter(|s| s.tycho.is_some()).count();
        assert_eq!(with_tycho, 80, "every star has a Tycho-2 entry");
        for s in &stars {
            assert!(s.hip > 0);
            assert!(s.gaia_parallax_mas.is_finite());
            assert!(s.hipparcos_parallax_mas.is_finite());
        }
    }

    #[test]
    fn proper_motions_agree_with_three_catalogues() {
        let stars = fixture().expect("catalogue fixture");
        let mut hip = Vec::new();
        let mut tyc = Vec::new();
        for s in &stars {
            hip.push(pair_max(
                normalized_pm_residual(&s.gaia, &s.hipparcos).expect("two proper motions"),
            ));
            let t = s.tycho.as_ref().expect("tycho entry");
            tyc.push(pair_max(
                normalized_pm_residual(&s.gaia, t).expect("two proper motions"),
            ));
        }
        let (h_med, h_p90) = median_and_p90(&hip).expect("a distribution");
        let (t_med, t_p90) = median_and_p90(&tyc).expect("a distribution");
        assert!(h_med <= 1.75, "Gaia vs Hipparcos median {h_med}");
        assert!(h_p90 <= 3.0, "Gaia vs Hipparcos p90 {h_p90}");
        assert!(t_med <= 1.75, "Gaia vs Tycho-2 median {t_med}");
        assert!(t_p90 <= 3.0, "Gaia vs Tycho-2 p90 {t_p90}");
        for (i, z) in tyc.iter().enumerate() {
            assert!(
                *z <= 3.0,
                "Tycho-2 proper motion {z} for star {}",
                stars[i].hip
            );
        }
    }

    #[test]
    fn parallax_matches_hipparcos_within_its_own_sigma() {
        let stars = fixture().expect("catalogue fixture");
        let mut values = Vec::new();
        for s in &stars {
            let z = normalized_parallax_residual(
                s.gaia_parallax_mas,
                s.gaia_parallax_error_mas,
                s.hipparcos_parallax_mas,
                s.hipparcos_parallax_error_mas,
            )
            .expect("two parallaxes");
            assert!(z <= 3.0, "parallax residual {z} for star {}", s.hip);
            values.push(z);
        }
        let (median, p90) = median_and_p90(&values).expect("a distribution");
        assert!(median <= 1.5, "median {median}");
        assert!(p90 <= 3.0, "p90 {p90}");
    }

    #[test]
    fn propagation_reaches_the_other_catalogue_epoch() {
        let stars = fixture().expect("catalogue fixture");
        let mut hip = Vec::new();
        let mut tyc = Vec::new();
        for s in &stars {
            let dt = GAIA_EPOCH - HIPPARCOS_EPOCH;
            hip.push(pair_max(
                normalized_position_residual(&s.hipparcos, dt, &s.gaia)
                    .expect("a propagated position"),
            ));
            let t = s.tycho.as_ref().expect("tycho entry");
            let dtt = GAIA_EPOCH - TYCHO_EPOCH;
            tyc.push(pair_max(
                normalized_position_residual(t, dtt, &s.gaia).expect("a propagated position"),
            ));
        }
        let (h_med, h_p90) = median_and_p90(&hip).expect("a distribution");
        let (t_med, t_p90) = median_and_p90(&tyc).expect("a distribution");
        assert!(h_med <= 1.75, "Hipparcos propagation median {h_med}");
        assert!(h_p90 <= 3.0, "Hipparcos propagation p90 {h_p90}");
        assert!(t_med <= 1.75, "Tycho-2 propagation median {t_med}");
        assert!(t_p90 <= 3.0, "Tycho-2 propagation p90 {t_p90}");
        for (i, z) in hip.iter().chain(tyc.iter()).enumerate() {
            assert!(*z <= 6.0, "gross position residual {z} at index {i}");
        }
    }

    #[test]
    fn propagation_is_reversible() {
        let a = Astrometry {
            ra_deg: 123.456,
            dec_deg: -45.5,
            pm_ra_mas_yr: -87.3,
            pm_dec_mas_yr: 42.1,
            ra_error_mas: 1.0,
            dec_error_mas: 1.0,
            pm_ra_error_mas_yr: 0.5,
            pm_dec_error_mas_yr: 0.5,
        };
        let (ra, dec) = propagate(&a, 24.75).expect("forward");
        let back = Astrometry {
            ra_deg: ra,
            dec_deg: dec,
            ..a.clone()
        };
        let (ra2, dec2) = propagate(&back, -24.75).expect("back");
        assert!((ra2 - a.ra_deg).abs() < 1e-8, "ra round trip {ra2}");
        assert!((dec2 - a.dec_deg).abs() < 1e-8, "dec round trip {dec2}");
    }

    #[test]
    fn a_pole_or_a_non_finite_input_is_absent() {
        let base = Astrometry {
            ra_deg: 10.0,
            dec_deg: 90.0,
            pm_ra_mas_yr: 1.0,
            pm_dec_mas_yr: 1.0,
            ra_error_mas: 1.0,
            dec_error_mas: 1.0,
            pm_ra_error_mas_yr: 1.0,
            pm_dec_error_mas_yr: 1.0,
        };
        assert!(propagate(&base, 10.0).is_none());
        let nan = Astrometry {
            pm_ra_mas_yr: f64::NAN,
            ..base
        };
        assert!(propagate(&nan, 10.0).is_none());
    }
}
