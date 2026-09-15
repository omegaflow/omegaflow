use crate::lsk::LeapSeconds;

pub const MAGIC: [u8; 4] = *b"GSO1";
pub const GROUP_HEADER_BYTES: usize = 8;
pub const TRANSIT_STRIDE_BYTES: usize = 32;

pub const J2010_JD: f64 = 2455197.5;
pub const UNIX_JD_EPOCH: f64 = 2440587.5;

pub const MAS_PER_ARCSEC: f64 = 1000.0;

pub const GAIA_TAP_SYNC: &str = "https://gea.esac.esa.int/tap-server/tap/sync";
pub const GAIA_SSO_TABLE: &str = "gaiadr3.sso_observation";

pub const TNO_NAME: &[(&str, u32)] = &[
    ("pluto", 134340),
    ("eris", 136199),
    ("makemake", 136472),
    ("haumea", 136108),
    ("quaoar", 50000),
    ("orcus", 90482),
    ("varuna", 20000),
    ("ixion", 28978),
    ("salacia", 120347),
    ("varda", 174567),
    ("2003az84", 208996),
    ("2002aw197", 55565),
    ("2002tx300", 55636),
    ("2002ux25", 55637),
];

pub fn tno_name(number_mp: u32) -> Option<&'static str> {
    TNO_NAME
        .iter()
        .find(|(_, n)| *n == number_mp)
        .map(|(name, _)| *name)
}

pub fn tno_number(name: &str) -> Option<u32> {
    TNO_NAME
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, number)| *number)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GaiaTransit {
    pub tdb: f64,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub sigma_arcsec: f64,
}

pub struct GaiaBody {
    pub number_mp: u32,
    pub transits: Vec<GaiaTransit>,
}

pub fn write_bin(bodies: &[GaiaBody]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(GROUP_HEADER_BYTES + bodies.len() * TRANSIT_STRIDE_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(bodies.len() as u32).to_le_bytes());
    for b in bodies {
        out.extend_from_slice(&b.number_mp.to_le_bytes());
        out.extend_from_slice(&(b.transits.len() as u32).to_le_bytes());
        for t in &b.transits {
            if !t.tdb.is_finite() || !t.ra_deg.is_finite() || !t.dec_deg.is_finite() {
                return None;
            }
            if !t.sigma_arcsec.is_finite() || !(t.sigma_arcsec > 0.0) {
                return None;
            }
            out.extend_from_slice(&t.tdb.to_le_bytes());
            out.extend_from_slice(&t.ra_deg.to_le_bytes());
            out.extend_from_slice(&t.dec_deg.to_le_bytes());
            out.extend_from_slice(&t.sigma_arcsec.to_le_bytes());
        }
    }
    Some(out)
}

fn f64_le(bytes: &[u8], off: usize) -> Option<f64> {
    Some(f64::from_le_bytes(
        bytes.get(off..off + 8)?.try_into().ok()?,
    ))
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<GaiaBody>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = 8usize;
    let mut bodies = Vec::with_capacity(count);
    for _ in 0..count {
        let number_mp = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        let n = u32::from_le_bytes(bytes.get(off + 4..off + 8)?.try_into().ok()?) as usize;
        off += 8;
        let mut transits = Vec::with_capacity(n);
        for _ in 0..n {
            let tdb = f64_le(bytes, off)?;
            let ra_deg = f64_le(bytes, off + 8)?;
            let dec_deg = f64_le(bytes, off + 16)?;
            let sigma_arcsec = f64_le(bytes, off + 24)?;
            if !tdb.is_finite()
                || !ra_deg.is_finite()
                || !dec_deg.is_finite()
                || !sigma_arcsec.is_finite()
                || !(sigma_arcsec > 0.0)
            {
                return None;
            }
            off += TRANSIT_STRIDE_BYTES;
            transits.push(GaiaTransit {
                tdb,
                ra_deg,
                dec_deg,
                sigma_arcsec,
            });
        }
        bodies.push(GaiaBody {
            number_mp,
            transits,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(bodies)
}

pub fn epoch_utc_days_to_unix(days: f64) -> Option<f64> {
    if !days.is_finite() {
        return None;
    }
    Some((J2010_JD + days - UNIX_JD_EPOCH) * 86400.0)
}

pub fn epoch_utc_days_to_tdb(days: f64, lsk: &LeapSeconds) -> Option<f64> {
    let unix = epoch_utc_days_to_unix(days)?;
    lsk.unix_to_tdb(unix)
}

fn cell_f64(cells: &[&str], k: usize) -> Option<f64> {
    let cell = cells.get(k)?.trim();
    if cell.is_empty() {
        return None;
    }
    let v: f64 = cell.parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

pub struct GaiaCsvCounts {
    pub rows: usize,
    pub emitted: usize,
    pub epoch_void: usize,
    pub position_void: usize,
    pub sigma_void: usize,
}

pub fn combined_sigma_arcsec(
    ra_random_mas: f64,
    dec_random_mas: f64,
    ra_systematic_mas: f64,
    dec_systematic_mas: f64,
) -> Option<f64> {
    let values = [
        ra_random_mas,
        dec_random_mas,
        ra_systematic_mas,
        dec_systematic_mas,
    ];
    if values.iter().any(|v| !v.is_finite() || *v < 0.0) {
        return None;
    }
    let ra_arcsec = (ra_random_mas * ra_random_mas + ra_systematic_mas * ra_systematic_mas).sqrt()
        / MAS_PER_ARCSEC;
    let dec_arcsec = (dec_random_mas * dec_random_mas + dec_systematic_mas * dec_systematic_mas)
        .sqrt()
        / MAS_PER_ARCSEC;
    let radial = ((ra_arcsec * ra_arcsec + dec_arcsec * dec_arcsec) * 0.5).sqrt();
    if radial.is_finite() && radial > 0.0 {
        Some(radial)
    } else {
        None
    }
}

pub fn parse_observation_csv(
    body: &str,
    lsk: &LeapSeconds,
) -> Option<(Vec<GaiaBody>, GaiaCsvCounts)> {
    let mut lines = body.lines();
    let header = lines.next()?;
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let index_of = |name: &str| cols.iter().position(|c| *c == name);
    let (
        Some(i_mp),
        Some(i_epoch),
        Some(i_ra),
        Some(i_dec),
        Some(i_ra_r),
        Some(i_dec_r),
        Some(i_ra_s),
        Some(i_dec_s),
    ) = (
        index_of("number_mp"),
        index_of("epoch_utc"),
        index_of("ra"),
        index_of("dec"),
        index_of("ra_error_random"),
        index_of("dec_error_random"),
        index_of("ra_error_systematic"),
        index_of("dec_error_systematic"),
    )
    else {
        return None;
    };
    let mut rows = 0usize;
    let mut epoch_void = 0usize;
    let mut position_void = 0usize;
    let mut sigma_void = 0usize;
    let mut emitted = 0usize;
    let mut collected: Vec<GaiaBody> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split(',').collect();
        rows += 1;
        let (Some(number_mp), Some(epoch_days)) = (
            cells.get(i_mp).and_then(|c| c.trim().parse::<u32>().ok()),
            cell_f64(&cells, i_epoch),
        ) else {
            epoch_void += 1;
            continue;
        };
        let Some(tdb) = epoch_utc_days_to_tdb(epoch_days, lsk) else {
            epoch_void += 1;
            continue;
        };
        let (Some(ra_deg), Some(dec_deg)) = (cell_f64(&cells, i_ra), cell_f64(&cells, i_dec))
        else {
            position_void += 1;
            continue;
        };
        if !(ra_deg >= 0.0 && ra_deg < 360.0) || !(dec_deg >= -90.0 && dec_deg <= 90.0) {
            position_void += 1;
            continue;
        }
        let Some(ra_r) = cell_f64(&cells, i_ra_r) else {
            sigma_void += 1;
            continue;
        };
        let Some(dec_r) = cell_f64(&cells, i_dec_r) else {
            sigma_void += 1;
            continue;
        };
        let Some(ra_s) = cell_f64(&cells, i_ra_s) else {
            sigma_void += 1;
            continue;
        };
        let Some(dec_s) = cell_f64(&cells, i_dec_s) else {
            sigma_void += 1;
            continue;
        };
        let Some(sigma_arcsec) = combined_sigma_arcsec(ra_r, dec_r, ra_s, dec_s) else {
            sigma_void += 1;
            continue;
        };
        match collected.iter_mut().find(|b| b.number_mp == number_mp) {
            Some(b) => b.transits.push(GaiaTransit {
                tdb,
                ra_deg,
                dec_deg,
                sigma_arcsec,
            }),
            None => collected.push(GaiaBody {
                number_mp,
                transits: vec![GaiaTransit {
                    tdb,
                    ra_deg,
                    dec_deg,
                    sigma_arcsec,
                }],
            }),
        }
        emitted += 1;
    }
    if collected.is_empty() {
        return None;
    }
    for b in &mut collected {
        b.transits.sort_by(|a, c| a.tdb.total_cmp(&c.tdb));
    }
    collected.sort_by(|a, b| a.number_mp.cmp(&b.number_mp));
    Some((
        collected,
        GaiaCsvCounts {
            rows,
            emitted,
            epoch_void,
            position_void,
            sigma_void,
        },
    ))
}

pub fn direction_to_radec(u: &[f64; 3]) -> Option<(f64, f64)> {
    let norm = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt();
    if !norm.is_finite() || norm <= 0.0 {
        return None;
    }
    let x = u[0] / norm;
    let y = u[1] / norm;
    let z = u[2] / norm;
    let dec = z.asin();
    let ra = y.atan2(x);
    let mut ra_deg = ra.to_degrees();
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    }
    Some((ra_deg, dec.to_degrees()))
}

pub fn ang_sep_arcsec(ra1_deg: f64, dec1_deg: f64, ra2_deg: f64, dec2_deg: f64) -> f64 {
    let r1 = ra1_deg.to_radians();
    let d1 = dec1_deg.to_radians();
    let r2 = ra2_deg.to_radians();
    let d2 = dec2_deg.to_radians();
    let a = ((d2 - d1) * 0.5).sin().powi(2) + d1.cos() * d2.cos() * ((r2 - r1) * 0.5).sin().powi(2);
    2.0 * a.sqrt().asin().to_degrees() * 3600.0
}

pub fn predicted_radec(
    kepler_helio: [f64; 3],
    sun: [f64; 3],
    observer: [f64; 3],
) -> Option<(f64, f64)> {
    let dx = kepler_helio[0] + sun[0] - observer[0];
    let dy = kepler_helio[1] + sun[1] - observer[1];
    let dz = kepler_helio[2] + sun[2] - observer[2];
    direction_to_radec(&[dx, dy, dz])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lsk() -> LeapSeconds {
        crate::lsk::parse(
            "KPL/LSK\nDELTET/DELTA_T_A       =   32.184\nDELTET/DELTA_AT        = ( 36,   @2015-JAN-1,\n 37,   @2017-JAN-1 )\n",
        )
        .unwrap()
    }

    #[test]
    fn tno_table_maps_all_fourteen_bright_bodies() {
        assert_eq!(TNO_NAME.len(), 14);
        assert_eq!(tno_name(134340), Some("pluto"));
        assert_eq!(tno_name(50000), Some("quaoar"));
        assert_eq!(tno_name(136199), Some("eris"));
        assert_eq!(tno_number("2002ux25"), Some(55637));
        assert_eq!(tno_number("ceres"), None);
        assert_eq!(tno_name(1), None);
    }

    #[test]
    fn epoch_utc_days_carry_the_j2010_offset() {
        let days = 2664.842498487589;
        let unix = epoch_utc_days_to_unix(days).unwrap();
        let unix_expected = (J2010_JD + days - UNIX_JD_EPOCH) * 86400.0;
        assert!((unix - unix_expected).abs() < 1e-6);
        let tdb = epoch_utc_days_to_tdb(days, &lsk()).unwrap();
        assert!(tdb.is_finite());
        let jd = tdb / 86400.0 + crate::archivar::J2000_EPOCH;
        assert!(jd > 2457000.0 && jd < 2458000.0, "jd {jd}");
    }

    #[test]
    fn combined_sigma_is_the_circular_one_sigma_radius() {
        let s = combined_sigma_arcsec(1000.0, 0.0, 0.0, 0.0).unwrap();
        let one_axis = std::f64::consts::FRAC_1_SQRT_2;
        assert!((s - one_axis).abs() < 1e-12);
        let s = combined_sigma_arcsec(1000.0, 1000.0, 0.0, 0.0).unwrap();
        assert!((s - 1.0).abs() < 1e-12);
        assert!(combined_sigma_arcsec(f64::NAN, 0.0, 0.0, 0.0).is_none());
        assert!(combined_sigma_arcsec(-1.0, 0.0, 0.0, 0.0).is_none());
        assert!(combined_sigma_arcsec(0.0, 0.0, 0.0, 0.0).is_none());
    }

    #[test]
    fn bin_roundtrip_preserves_the_measured_transits() {
        let bodies = vec![GaiaBody {
            number_mp: 134340,
            transits: vec![
                GaiaTransit {
                    tdb: 5.4e8,
                    ra_deg: 290.6240245290491,
                    dec_deg: -21.197720404727228,
                    sigma_arcsec: 0.61,
                },
                GaiaTransit {
                    tdb: 5.4e8 + 86400.0,
                    ra_deg: 290.6,
                    dec_deg: -21.2,
                    sigma_arcsec: 0.62,
                },
            ],
        }];
        let bytes = write_bin(&bodies).unwrap();
        let parsed = parse_bin(&bytes).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].number_mp, 134340);
        assert_eq!(parsed[0].transits.len(), 2);
        assert_eq!(parsed[0].transits[0].ra_deg, 290.6240245290491);
        assert!((parsed[0].transits[0].sigma_arcsec - 0.61).abs() < 1e-12);
    }

    #[test]
    fn bin_refuses_non_finite_and_zero_sigma() {
        let bad = vec![GaiaBody {
            number_mp: 134340,
            transits: vec![GaiaTransit {
                tdb: 5.4e8,
                ra_deg: 290.6,
                dec_deg: -21.2,
                sigma_arcsec: 0.0,
            }],
        }];
        assert!(write_bin(&bad).is_none());
        let bad = vec![GaiaBody {
            number_mp: 134340,
            transits: vec![GaiaTransit {
                tdb: f64::NAN,
                ra_deg: 290.6,
                dec_deg: -21.2,
                sigma_arcsec: 0.5,
            }],
        }];
        assert!(write_bin(&bad).is_none());
    }

    #[test]
    fn parse_observation_csv_reads_a_measured_pluto_row() {
        let body = "number_mp,epoch_utc,ra,dec,ra_error_random,dec_error_random,ra_error_systematic,dec_error_systematic\n134340,2664.842498487589,290.6240245290491,-21.197720404727228,374.38806978595466,484.69346948402466,1.4067506408786947,1.8205034242007798\n134340,2664.843583656429,290.6240418931079,-21.197699491067045,374.38852517657955,484.69311190644294,1.406752366366093,1.8205020908702025\n";
        let (bodies, counts) = parse_observation_csv(body, &lsk()).unwrap();
        assert_eq!(bodies.len(), 1);
        assert_eq!(bodies[0].number_mp, 134340);
        assert_eq!(bodies[0].transits.len(), 2);
        assert!(bodies[0].transits[0].tdb > 5.0e8);
        assert!(
            (bodies[0].transits[0].sigma_arcsec - 0.433).abs() < 0.01,
            "sigma {}",
            bodies[0].transits[0].sigma_arcsec
        );
        assert_eq!(counts.rows, 2);
        assert_eq!(counts.emitted, 2);
    }

    #[test]
    fn parse_observation_csv_skips_rows_whose_epoch_is_void() {
        let body = "number_mp,epoch_utc,ra,dec,ra_error_random,dec_error_random,ra_error_systematic,dec_error_systematic\n134340,void,290.6,-21.2,374.0,484.0,1.4,1.8\n";
        let parsed = parse_observation_csv(body, &lsk());
        assert!(parsed.is_none());
    }

    #[test]
    fn direction_to_radec_places_poles_and_equator() {
        let (_, dec) = direction_to_radec(&[0.0, 0.0, 1.0]).unwrap();
        assert!(dec > 89.999999);
        let (ra, dec) = direction_to_radec(&[1.0, 0.0, 0.0]).unwrap();
        assert!((ra - 0.0).abs() < 1e-9);
        assert!((dec - 0.0).abs() < 1e-9);
        let (ra, dec) = direction_to_radec(&[0.0, -1.0, 0.0]).unwrap();
        assert!((ra - 270.0).abs() < 1e-9);
        assert!((dec - 0.0).abs() < 1e-9);
        assert!(direction_to_radec(&[0.0, 0.0, 0.0]).is_none());
    }

    #[test]
    fn ang_sep_measures_arcsec_and_zero_is_honored() {
        assert!(ang_sep_arcsec(10.0, 20.0, 10.0, 20.0).abs() < 1e-9);
        let s = ang_sep_arcsec(0.0, 0.0, 0.0, 1.0 / 3600.0);
        assert!((s - 1.0).abs() < 1e-6);
        let s = ang_sep_arcsec(0.0, 0.0, 1.0, 0.0);
        assert!((s - 3600.0).abs() < 1e-6);
    }

    #[test]
    fn predicted_radec_folds_heliocentric_to_the_observer() {
        let helio = [40.0 * crate::archivar::kepler::AU_M, 0.0, 0.0];
        let sun = [-crate::archivar::kepler::AU_M, 0.0, 0.0];
        let obs = [-crate::archivar::kepler::AU_M, 0.0, 0.0];
        let (ra, dec) = predicted_radec(helio, sun, obs).unwrap();
        assert!((ra - 0.0).abs() < 1e-9);
        assert!((dec - 0.0).abs() < 1e-9);
        let obs = [0.0, 0.0, crate::archivar::kepler::AU_M];
        let (ra, dec) = predicted_radec(helio, sun, obs).unwrap();
        assert!((ra - 0.0).abs() < 1e-9, "ra {ra}");
        assert!(dec < 0.0, "dec {dec}");
    }
}
