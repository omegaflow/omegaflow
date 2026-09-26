use crate::archivar::dastcom::{self, AsteroidRec};
use crate::archivar::types::C_LIGHT;

pub fn write_bin(records: &[[f64; 6]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 48);
    out.extend_from_slice(b"PDPL");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_bin(data: &[u8]) -> Option<Vec<[f64; 6]>> {
    if data.len() < 8 || &data[0..4] != b"PDPL" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 48 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 48;
        let mut r = [0.0f64; 6];
        for k in 0..6 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub fn write_pnav_bin(records: &[[f64; 9]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * 72);
    out.extend_from_slice(b"PNAV");
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

pub fn parse_pnav_bin(data: &[u8]) -> Option<Vec<[f64; 9]>> {
    if data.len() < 8 || &data[0..4] != b"PNAV" {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != 8 + count * 72 {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = 8 + i * 72;
        let mut r = [0.0f64; 9];
        for k in 0..9 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[base + k * 8..base + k * 8 + 8]);
            r[k] = f64::from_le_bytes(buf);
        }
        out.push(r);
    }
    Some(out)
}

pub const DOPPLER_GEIST_MAGIC: [u8; 4] = *b"DGZ1";

pub const DOPPLER_GEIST_RECORD_BYTES: usize = 48;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DopplerResidual {
    pub number: u32,
    pub t_jd: f64,
    pub b_m: f64,
    pub gm_m3_s2: f64,
    pub dz_pred: f64,
    pub dz_meas: Option<f64>,
}

pub fn doppler_dz(gm_m3_s2: f64, b_m: f64) -> Option<f64> {
    if !gm_m3_s2.is_finite() || gm_m3_s2 <= 0.0 || !b_m.is_finite() || b_m <= 0.0 {
        return None;
    }
    let dz = gm_m3_s2 / (C_LIGHT * C_LIGHT * b_m);
    if dz.is_finite() { Some(dz) } else { None }
}

pub fn impact_parameter(
    asteroid_icrs: [f64; 3],
    observer_icrs: [f64; 3],
    star_unit: [f64; 3],
) -> Option<f64> {
    let d = [
        asteroid_icrs[0] - observer_icrs[0],
        asteroid_icrs[1] - observer_icrs[1],
        asteroid_icrs[2] - observer_icrs[2],
    ];
    let n =
        (star_unit[0] * star_unit[0] + star_unit[1] * star_unit[1] + star_unit[2] * star_unit[2])
            .sqrt();
    if !n.is_finite() || n <= 0.0 {
        return None;
    }
    let u = [star_unit[0] / n, star_unit[1] / n, star_unit[2] / n];
    let cx = d[1] * u[2] - d[2] * u[1];
    let cy = d[2] * u[0] - d[0] * u[2];
    let cz = d[0] * u[1] - d[1] * u[0];
    let b2 = cx * cx + cy * cy + cz * cz;
    if !b2.is_finite() {
        return None;
    }
    Some(b2.sqrt())
}

pub fn doppler_residual(
    rec: &AsteroidRec,
    observer_icrs: [f64; 3],
    star_unit: [f64; 3],
    t_jd: f64,
) -> Option<DopplerResidual> {
    let (pos, _vel) = dastcom::state_at(rec, t_jd)?;
    let b = impact_parameter(pos, observer_icrs, star_unit)?;
    let gm = rec.gm_km3_s2 as f64 * 1.0e9;
    let dz = doppler_dz(gm, b)?;
    Some(DopplerResidual {
        number: rec.number,
        t_jd,
        b_m: b,
        gm_m3_s2: gm,
        dz_pred: dz,
        dz_meas: None,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DopplerResidualStat {
    pub n: usize,
    pub mean: f64,
    pub sd: Option<f64>,
}

fn mean_sd(xs: &[f64]) -> Option<DopplerResidualStat> {
    if xs.is_empty() {
        return None;
    }
    let mean = xs.iter().sum::<f64>() / xs.len() as f64;
    if !mean.is_finite() {
        return None;
    }
    let sd = if xs.len() >= 2 {
        let var = xs.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (xs.len() - 1) as f64;
        if var.is_finite() && var >= 0.0 {
            Some(var.sqrt())
        } else {
            None
        }
    } else {
        None
    };
    Some(DopplerResidualStat {
        n: xs.len(),
        mean,
        sd,
    })
}

pub fn prediction_stat(lines: &[DopplerResidual]) -> Option<DopplerResidualStat> {
    let pred: Vec<f64> = lines
        .iter()
        .map(|l| l.dz_pred)
        .filter(|d| d.is_finite())
        .collect();
    mean_sd(&pred)
}

pub fn residual_stat(lines: &[DopplerResidual]) -> Option<DopplerResidualStat> {
    let res: Vec<f64> = lines
        .iter()
        .filter_map(|l| l.dz_meas.map(|m| m - l.dz_pred))
        .filter(|r| r.is_finite())
        .collect();
    mean_sd(&res)
}

pub fn write_doppler_bin(lines: &[DopplerResidual]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(8 + lines.len() * DOPPLER_GEIST_RECORD_BYTES);
    out.extend_from_slice(&DOPPLER_GEIST_MAGIC);
    out.extend_from_slice(&(lines.len() as u32).to_le_bytes());
    for l in lines {
        if !l.t_jd.is_finite()
            || !l.b_m.is_finite()
            || !l.gm_m3_s2.is_finite()
            || !l.dz_pred.is_finite()
        {
            return None;
        }
        let (flag, meas) = match l.dz_meas {
            Some(m) if m.is_finite() => (1u32, m),
            Some(_) => return None,
            None => (0u32, 0.0),
        };
        out.extend_from_slice(&l.number.to_le_bytes());
        out.extend_from_slice(&flag.to_le_bytes());
        out.extend_from_slice(&l.t_jd.to_le_bytes());
        out.extend_from_slice(&l.b_m.to_le_bytes());
        out.extend_from_slice(&l.gm_m3_s2.to_le_bytes());
        out.extend_from_slice(&l.dz_pred.to_le_bytes());
        out.extend_from_slice(&meas.to_le_bytes());
    }
    Some(out)
}

pub fn parse_doppler_bin(bytes: &[u8]) -> Option<Vec<DopplerResidual>> {
    if bytes.len() < 8 || bytes[0..4] != DOPPLER_GEIST_MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * DOPPLER_GEIST_RECORD_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut off = 8usize;
    let f64_at = |o: &mut usize| -> Option<f64> {
        let v = f64::from_le_bytes(bytes.get(*o..*o + 8)?.try_into().ok()?);
        *o += 8;
        Some(v)
    };
    for _ in 0..n {
        let number = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let flag = u32::from_le_bytes(bytes.get(off..off + 4)?.try_into().ok()?);
        off += 4;
        let t_jd = f64_at(&mut off)?;
        let b_m = f64_at(&mut off)?;
        let gm = f64_at(&mut off)?;
        let dz_pred = f64_at(&mut off)?;
        let meas = f64_at(&mut off)?;
        if !t_jd.is_finite()
            || !b_m.is_finite()
            || !gm.is_finite()
            || !dz_pred.is_finite()
            || !meas.is_finite()
        {
            return None;
        }
        let dz_meas = match flag {
            0 => None,
            1 => Some(meas),
            _ => return None,
        };
        out.push(DopplerResidual {
            number,
            t_jd,
            b_m,
            gm_m3_s2: gm,
            dz_pred,
            dz_meas,
        });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnav_roundtrip() {
        let records: Vec<[f64; 9]> = vec![
            [
                -8.28095e8,
                5.012066e5,
                2.1108144e9,
                1.98e3,
                12.0,
                0.0,
                12.0,
                12.0,
                12.0,
            ],
            [0.0, 0.0, 2.1e9, 1.0e3, 13.0, 0.0, 11.0, 43.0, 13.0],
        ];
        let bin = write_pnav_bin(&records);
        assert_eq!(&bin[0..4], b"PNAV");
        let parsed = parse_pnav_bin(&bin).expect("parse void");
        assert_eq!(parsed.len(), records.len());
        for (a, b) in parsed.iter().zip(records.iter()) {
            for k in 0..9 {
                assert_eq!(a[k], b[k]);
            }
        }
        assert!(parse_pnav_bin(&bin[..8]).is_none());
        assert!(parse_pnav_bin(b"XXXX12345678").is_none());
    }

    fn ceres() -> AsteroidRec {
        AsteroidRec {
            number: 1,
            epoch_jd: 2458849.5,
            a_au: 2.7692892921434837,
            e: 0.07687465013145245,
            incl_deg: 10.59127767086216,
            node_deg: 80.3011901917491,
            peri_deg: 73.80896808746482,
            ma_deg: 130.31596882009862,
            h: 3.34,
            g: 0.12,
            albedo: 0.09,
            rot_period_h: 9.07417,
            radius_km: 469.7,
            gm_km3_s2: 62.6284,
            sptype: [b'C', 0, 0, 0, 0],
        }
    }

    #[test]
    fn doppler_dz_reads_the_potential_depth() {
        let gm = 62.6284e9;
        let b = 469.7e3;
        let dz = doppler_dz(gm, b).unwrap();
        assert_eq!(dz, gm / (C_LIGHT * C_LIGHT * b));
        assert!(dz > 0.0);
        let small = doppler_dz(5.0e5, 1.0e4).unwrap();
        assert_eq!(small, 5.0e5 / (C_LIGHT * C_LIGHT * 1.0e4));
    }

    #[test]
    fn doppler_dz_refuses_absent_inputs() {
        assert!(doppler_dz(0.0, 1.0).is_none());
        assert!(doppler_dz(1.0, 0.0).is_none());
        assert!(doppler_dz(f64::NAN, 1.0).is_none());
        assert!(doppler_dz(1.0, f64::NAN).is_none());
        assert!(doppler_dz(-1.0, 1.0).is_none());
    }

    #[test]
    fn impact_parameter_reads_the_perpendicular_offset() {
        let b = impact_parameter([1.5e11, 0.0, 0.0], [0.0; 3], [1.0, 0.0, 0.0]).unwrap();
        assert_eq!(b, 0.0, "the central ray carries b = 0");
        let b = impact_parameter([1.5e11, 1.0e3, 0.0], [0.0; 3], [1.0, 0.0, 0.0]).unwrap();
        assert!((b - 1.0e3).abs() < 1e-6, "b {b}");
        let b = impact_parameter([1.5e11, 1.0e3, 0.0], [0.0; 3], [2.0, 0.0, 0.0]).unwrap();
        assert!(
            (b - 1.0e3).abs() < 1e-6,
            "the unit vector normalizes, b {b}"
        );
        assert!(impact_parameter([1.5e11, 0.0, 0.0], [0.0; 3], [0.0; 3]).is_none());
        assert!(impact_parameter([1.5e11, 0.0, 0.0], [0.0; 3], [f64::NAN, 0.0, 0.0]).is_none());
    }

    #[test]
    fn doppler_residual_reads_gm_and_geometry() {
        let rec = ceres();
        let (pos, _) = dastcom::state_at(&rec, rec.epoch_jd).unwrap();
        let line = doppler_residual(&rec, [0.0; 3], [1.0, 0.0, 0.0], rec.epoch_jd).unwrap();
        let b_expect = (pos[1] * pos[1] + pos[2] * pos[2]).sqrt();
        assert!(
            (line.b_m - b_expect).abs() < 1e-6,
            "b {} vs {}",
            line.b_m,
            b_expect
        );
        let gm = rec.gm_km3_s2 as f64 * 1.0e9;
        assert_eq!(line.dz_pred, gm / (C_LIGHT * C_LIGHT * b_expect));
        assert!(line.dz_meas.is_none(), "no measurement is harvested yet");
    }

    #[test]
    fn residual_stat_returns_none_without_measurements() {
        let line = DopplerResidual {
            number: 1,
            t_jd: 2458849.5,
            b_m: 1.0,
            gm_m3_s2: 1.0,
            dz_pred: 1.0e-16,
            dz_meas: None,
        };
        assert!(residual_stat(&[line]).is_none());
        assert!(residual_stat(&[]).is_none());
    }

    #[test]
    fn residual_stat_reads_the_null_distribution() {
        let mk = |meas: f64| DopplerResidual {
            number: 1,
            t_jd: 2458849.5,
            b_m: 1.0,
            gm_m3_s2: 1.0,
            dz_pred: 1.0e-16,
            dz_meas: Some(meas),
        };
        let lines = [mk(1.1e-16), mk(1.3e-16)];
        let r1 = 1.1e-16f64 - 1.0e-16;
        let r2 = 1.3e-16f64 - 1.0e-16;
        let stat = residual_stat(&lines).unwrap();
        assert_eq!(stat.n, 2);
        assert!(
            (stat.mean - (r1 + r2) / 2.0).abs() < 1e-32,
            "mean {}",
            stat.mean
        );
        let expect_sd = ((r1 - (r1 + r2) / 2.0).powi(2) + (r2 - (r1 + r2) / 2.0).powi(2)).sqrt();
        assert!((stat.sd.unwrap() - expect_sd).abs() < 1e-32, "sd");
    }

    #[test]
    fn residual_stat_skips_absent_and_nonfinite_lines() {
        let measured = DopplerResidual {
            number: 1,
            t_jd: 2458849.5,
            b_m: 1.0,
            gm_m3_s2: 1.0,
            dz_pred: 2.0e-16,
            dz_meas: Some(2.1e-16),
        };
        let absent = DopplerResidual {
            dz_meas: None,
            ..measured
        };
        let bad = DopplerResidual {
            dz_meas: Some(f64::NAN),
            ..measured
        };
        let stat = residual_stat(&[absent, measured, bad]).unwrap();
        assert_eq!(stat.n, 1, "one measured line remains");
        assert!((stat.mean - (2.1e-16 - 2.0e-16)).abs() < 1e-32);
        assert!(stat.sd.is_none(), "one line carries a mean and no σ");
    }

    #[test]
    fn prediction_stat_reads_the_expectation() {
        let mk = |pred: f64| DopplerResidual {
            number: 1,
            t_jd: 2458849.5,
            b_m: 1.0,
            gm_m3_s2: 1.0,
            dz_pred: pred,
            dz_meas: None,
        };
        let stat = prediction_stat(&[mk(1.0e-16), mk(2.0e-16)]).unwrap();
        assert_eq!(stat.n, 2);
        let m = (1.0e-16f64 + 2.0e-16) / 2.0;
        assert!((stat.mean - m).abs() < 1e-32);
        let expect_sd = ((1.0e-16 - m).powi(2) + (2.0e-16 - m).powi(2)).sqrt();
        assert!((stat.sd.unwrap() - expect_sd).abs() < 1e-32);
        assert!(prediction_stat(&[]).is_none());
    }

    #[test]
    fn doppler_bin_roundtrip() {
        let rec = ceres();
        let mut measured = doppler_residual(&rec, [0.0; 3], [1.0, 0.0, 0.0], rec.epoch_jd).unwrap();
        let plain = doppler_residual(&rec, [0.0; 3], [0.0, 1.0, 0.0], rec.epoch_jd + 50.0).unwrap();
        measured.dz_meas = Some(measured.dz_pred);
        let lines = [measured, plain];
        let bin = write_doppler_bin(&lines).unwrap();
        assert_eq!(&bin[0..4], &DOPPLER_GEIST_MAGIC);
        let parsed = parse_doppler_bin(&bin).unwrap();
        assert_eq!(parsed.len(), lines.len());
        assert_eq!(parsed[0].dz_meas, Some(lines[0].dz_pred));
        assert_eq!(parsed[1].dz_meas, None);
        assert!(parse_doppler_bin(&bin[..8]).is_none());
        let mut wrong = bin.clone();
        wrong[0] = b'X';
        assert!(parse_doppler_bin(&wrong).is_none());
    }
}
