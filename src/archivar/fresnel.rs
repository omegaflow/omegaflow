use crate::archivar::dastcom::{self, AsteroidRec};
use crate::archivar::spectral;

pub const MAGIC: [u8; 4] = *b"FRS1";

pub const RECORD_BYTES: usize = 60;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FresnelLine {
    pub number: u32,
    pub t_jd: f64,
    pub d_m: f64,
    pub a_m: f64,
    pub theta_rad: f64,
    pub lambda_m: f64,
    pub fresnel: f64,
    pub fringe_m: f64,
}

pub fn fresnel_line(
    rec: &AsteroidRec,
    bp_rp: f64,
    observer_icrs: [f64; 3],
    t_jd: f64,
) -> Option<FresnelLine> {
    let (pos, _vel) = dastcom::state_at(rec, t_jd)?;
    let dx = pos[0] - observer_icrs[0];
    let dy = pos[1] - observer_icrs[1];
    let dz = pos[2] - observer_icrs[2];
    let d = (dx * dx + dy * dy + dz * dz).sqrt();
    if !d.is_finite() || d <= 0.0 {
        return None;
    }
    let a = rec.radius_km as f64 * 1.0e3;
    if !a.is_finite() || a <= 0.0 {
        return None;
    }
    let lambda = spectral::bp_rp_to_lambda_nm(bp_rp)? * 1.0e-9;
    let theta = 2.0 * a / d;
    let fresnel = a * a / (lambda * d);
    let fringe = (lambda * d * 0.5).sqrt();
    if !theta.is_finite() || !fresnel.is_finite() || !fringe.is_finite() {
        return None;
    }
    Some(FresnelLine {
        number: rec.number,
        t_jd,
        d_m: d,
        a_m: a,
        theta_rad: theta,
        lambda_m: lambda,
        fresnel,
        fringe_m: fringe,
    })
}

pub fn write_bin(lines: &[FresnelLine]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(8 + lines.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(lines.len() as u32).to_le_bytes());
    for l in lines {
        if !l.t_jd.is_finite()
            || !l.d_m.is_finite()
            || !l.a_m.is_finite()
            || !l.theta_rad.is_finite()
            || !l.lambda_m.is_finite()
            || !l.fresnel.is_finite()
            || !l.fringe_m.is_finite()
        {
            return None;
        }
        out.extend_from_slice(&l.number.to_le_bytes());
        out.extend_from_slice(&l.t_jd.to_le_bytes());
        out.extend_from_slice(&l.d_m.to_le_bytes());
        out.extend_from_slice(&l.a_m.to_le_bytes());
        out.extend_from_slice(&l.theta_rad.to_le_bytes());
        out.extend_from_slice(&l.lambda_m.to_le_bytes());
        out.extend_from_slice(&l.fresnel.to_le_bytes());
        out.extend_from_slice(&l.fringe_m.to_le_bytes());
    }
    Some(out)
}

pub fn parse_bin(bytes: &[u8]) -> Option<Vec<FresnelLine>> {
    if bytes.len() < 8 || bytes[0..4] != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != 8 + n * RECORD_BYTES {
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
        let t_jd = f64_at(&mut off)?;
        let d_m = f64_at(&mut off)?;
        let a_m = f64_at(&mut off)?;
        let theta_rad = f64_at(&mut off)?;
        let lambda_m = f64_at(&mut off)?;
        let fresnel = f64_at(&mut off)?;
        let fringe_m = f64_at(&mut off)?;
        if !t_jd.is_finite()
            || !d_m.is_finite()
            || !a_m.is_finite()
            || !theta_rad.is_finite()
            || !lambda_m.is_finite()
            || !fresnel.is_finite()
            || !fringe_m.is_finite()
        {
            return None;
        }
        out.push(FresnelLine {
            number,
            t_jd,
            d_m,
            a_m,
            theta_rad,
            lambda_m,
            fresnel,
            fringe_m,
        });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::dastcom::AsteroidRec;

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
    fn fresnel_line_computes_the_geometric_triad() {
        let rec = ceres();
        let (pos, _) = dastcom::state_at(&rec, rec.epoch_jd).unwrap();
        let line = fresnel_line(&rec, 0.82, [0.0; 3], rec.epoch_jd).unwrap();
        let d = (pos[0] * pos[0] + pos[1] * pos[1] + pos[2] * pos[2]).sqrt();
        assert!(
            (line.d_m - d).abs() < 1e-9,
            "distance {} vs {}",
            line.d_m,
            d
        );
        let a = rec.radius_km as f64 * 1.0e3;
        assert_eq!(line.a_m, a);
        assert!(
            (line.theta_rad - 2.0 * a / d).abs() < 1e-24,
            "theta {} vs {}",
            line.theta_rad,
            2.0 * a / d
        );
        assert!(
            (line.fresnel - a * a / (line.lambda_m * d)).abs() < 1e-12,
            "fresnel {}",
            line.fresnel
        );
        assert!(
            (line.fringe_m - (line.lambda_m * d * 0.5).sqrt()).abs() < 1e-6,
            "fringe {}",
            line.fringe_m
        );
        assert!(line.lambda_m > 300.0e-9 && line.lambda_m < 1100.0e-9);
    }

    #[test]
    fn fresnel_line_refuses_absent_radius() {
        let mut rec = ceres();
        rec.radius_km = 0.0;
        assert!(fresnel_line(&rec, 0.82, [0.0; 3], rec.epoch_jd).is_none());
        rec.radius_km = f32::NAN;
        assert!(fresnel_line(&rec, 0.82, [0.0; 3], rec.epoch_jd).is_none());
    }

    #[test]
    fn fresnel_line_refuses_nonfinite_color() {
        assert!(fresnel_line(&ceres(), f64::NAN, [0.0; 3], ceres().epoch_jd).is_none());
    }

    #[test]
    fn fresnel_line_refuses_the_observer_inside_the_asteroid() {
        let rec = ceres();
        let (pos, _) = dastcom::state_at(&rec, rec.epoch_jd).unwrap();
        assert!(fresnel_line(&rec, 0.82, pos, rec.epoch_jd).is_none());
    }

    #[test]
    fn fresnel_line_angular_diameter_scales_with_distance() {
        let rec = ceres();
        let near = fresnel_line(&rec, 0.82, [0.0; 3], rec.epoch_jd).unwrap();
        let (pos, _) = dastcom::state_at(&rec, rec.epoch_jd).unwrap();
        let antipode = [-pos[0], -pos[1], -pos[2]];
        let far = fresnel_line(&rec, 0.82, antipode, rec.epoch_jd).unwrap();
        assert!(
            far.d_m > near.d_m && far.theta_rad < near.theta_rad,
            "a farther observer sees a smaller disk"
        );
        assert!(far.fresnel < near.fresnel, "F falls with distance");
    }

    #[test]
    fn fresnel_bin_roundtrip() {
        let rec = ceres();
        let lines = vec![
            fresnel_line(&rec, 0.82, [0.0; 3], rec.epoch_jd).unwrap(),
            fresnel_line(&rec, -0.1, [0.0; 3], rec.epoch_jd + 100.0).unwrap(),
        ];
        let bin = write_bin(&lines).unwrap();
        assert_eq!(&bin[0..4], &MAGIC);
        let parsed = parse_bin(&bin).unwrap();
        assert_eq!(parsed.len(), lines.len());
        for (a, b) in parsed.iter().zip(lines.iter()) {
            assert_eq!(a.number, b.number);
            assert_eq!(a.t_jd, b.t_jd);
            assert_eq!(a.d_m, b.d_m);
            assert_eq!(a.a_m, b.a_m);
            assert_eq!(a.theta_rad, b.theta_rad);
            assert_eq!(a.lambda_m, b.lambda_m);
            assert_eq!(a.fresnel, b.fresnel);
            assert_eq!(a.fringe_m, b.fringe_m);
        }
    }

    #[test]
    fn fresnel_bin_refuses_foreign_magic_and_truncation() {
        let rec = ceres();
        let line = fresnel_line(&rec, 0.82, [0.0; 3], rec.epoch_jd).unwrap();
        let bin = write_bin(&[line]).unwrap();
        assert!(parse_bin(&bin[..8]).is_none());
        let mut wrong = bin.clone();
        wrong[0] = b'X';
        assert!(parse_bin(&wrong).is_none());
        assert!(parse_bin(&bin[..bin.len() - 1]).is_none());
    }
}
