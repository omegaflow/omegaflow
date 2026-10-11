use crate::mathematikerin::tropo::Vmf3Coefficients;

use super::vmf3_sh_data::{ANM_BH, ANM_BW, ANM_CH, ANM_CW, BNM_BH, BNM_BW, BNM_CH, BNM_CW};

pub const VMF3_BC_CONVENTION: &str = "VMF3 b/c are the empirical degree-12 spherical-harmonic model (TU Wien codes/vmf3.f90, Landskron & Boehm 2018): the coefficient rows are [A0,A1,B1,A2,B2] per (n,m), n=0..12 m=0..n; the real spherical harmonics V,W are the un-normalised associated-Legendre recursion in vmf3.f90; the amplitude of each field is A0 + A1*cos(2*pi*doy/365.25) + B1*sin(...) + A2*cos(4*pi*doy/365.25) + B2*sin(...); doy is the day of year (with fraction), lat/lon in degrees; a non-finite or out-of-range input -> absent (None)";

const NMAX: usize = 12;

fn seasonal(row: &[f64; 5], pan: f64) -> f64 {
    row[0]
        + row[1] * pan.cos()
        + row[2] * pan.sin()
        + row[3] * (2.0 * pan).cos()
        + row[4] * (2.0 * pan).sin()
}

pub fn b_c(doy: f64, lat_deg: f64, lon_deg: f64) -> Option<Vmf3Coefficients> {
    if !doy.is_finite() || !lat_deg.is_finite() || !lon_deg.is_finite() {
        return None;
    }
    if !(-90.0..=90.0).contains(&lat_deg) {
        return None;
    }
    let pan = doy / 365.25 * std::f64::consts::TAU;
    let lon = lon_deg.to_radians();
    let lat = lat_deg.to_radians();
    let pol_dist = std::f64::consts::FRAC_PI_2 - lat;
    let x = pol_dist.sin() * lon.cos();
    let y = pol_dist.sin() * lon.sin();
    let z = pol_dist.cos();

    let mut v = [[0.0f64; NMAX + 1]; NMAX + 1];
    let mut w = [[0.0f64; NMAX + 1]; NMAX + 1];
    v[0][0] = 1.0;
    v[1][0] = z * v[0][0];
    for n in 2..=NMAX {
        let nf = n as f64;
        v[n][0] = ((2.0 * nf - 1.0) * z * v[n - 1][0] - (nf - 1.0) * v[n - 2][0]) / nf;
    }
    for m in 1..=NMAX {
        let mf = m as f64;
        v[m][m] = (2.0 * mf - 1.0) * (x * v[m - 1][m - 1] - y * w[m - 1][m - 1]);
        w[m][m] = (2.0 * mf - 1.0) * (x * w[m - 1][m - 1] + y * v[m - 1][m - 1]);
        if m < NMAX {
            v[m + 1][m] = (2.0 * mf + 1.0) * z * v[m][m];
            w[m + 1][m] = (2.0 * mf + 1.0) * z * w[m][m];
        }
        for n in (m + 2)..=NMAX {
            let nf = n as f64;
            v[n][m] =
                ((2.0 * nf - 1.0) * z * v[n - 1][m] - (nf + mf - 1.0) * v[n - 2][m]) / (nf - mf);
            w[n][m] =
                ((2.0 * nf - 1.0) * z * w[n - 1][m] - (nf + mf - 1.0) * w[n - 2][m]) / (nf - mf);
        }
    }

    let tables: [&[[f64; 5]; 91]; 8] = [
        &ANM_BH, &ANM_BW, &ANM_CH, &ANM_CW, &BNM_BH, &BNM_BW, &BNM_CH, &BNM_CW,
    ];
    let mut acc = [[0.0f64; 5]; 4];
    let mut i = 0usize;
    for n in 0..=NMAX {
        for m in 0..=n {
            let vv = v[n][m];
            let ww = w[n][m];
            for f in 0..4 {
                let anm = tables[f][i];
                let bnm = tables[4 + f][i];
                for c in 0..5 {
                    acc[f][c] += anm[c] * vv + bnm[c] * ww;
                }
            }
            i += 1;
        }
    }
    debug_assert_eq!(i, 91);

    let bh = seasonal(&acc[0], pan);
    let bw = seasonal(&acc[1], pan);
    let ch = seasonal(&acc[2], pan);
    let cw = seasonal(&acc[3], pan);
    if !(bh.is_finite() && bw.is_finite() && ch.is_finite() && cw.is_finite()) {
        return None;
    }
    Some(Vmf3Coefficients {
        a_h: None,
        b_h: Some(bh),
        c_h: Some(ch),
        a_w: None,
        b_w: Some(bw),
        c_w: Some(cw),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pole_bh(doy: f64) -> f64 {
        let pan = doy / 365.25 * std::f64::consts::TAU;
        let mut sum = 0.0;
        for n in 0..=NMAX {
            let idx = n * (n + 1) / 2;
            sum += seasonal(&ANM_BH[idx], pan);
        }
        sum
    }

    #[test]
    fn the_a0_constant_is_the_measured_model_constant() {
        assert_eq!(ANM_BH[0][0], 0.0027128586310995);
    }

    #[test]
    fn at_the_pole_the_w_terms_vanish_and_only_m0_survives() {
        let doy = 2.0;
        let c = b_c(doy, 90.0, 0.0).expect("the pole carries a coefficient set");
        let expected = pole_bh(doy);
        assert!(
            (c.b_h.expect("bh present") - expected).abs() < 1.0e-15,
            "pole bh {} vs m0 sum {expected}",
            c.b_h.expect("bh present")
        );
    }

    #[test]
    fn aggo_round_carries_finite_plausible_coefficients() {
        let c = b_c(2.0, -34.874, -58.140).expect("AGGO carries a coefficient set");
        let bh = c.b_h.expect("bh finite");
        let bw = c.b_w.expect("bw finite");
        let ch = c.c_h.expect("ch finite");
        let cw = c.c_w.expect("cw finite");
        assert!((0.001..0.005).contains(&bh), "bh {bh}");
        assert!((0.0002..0.005).contains(&bw), "bw {bw}");
        assert!((0.02..0.10).contains(&ch), "ch {ch}");
        assert!((0.02..0.08).contains(&cw), "cw {cw}");
    }

    #[test]
    fn non_finite_or_out_of_range_stays_absent() {
        assert!(b_c(f64::NAN, 0.0, 0.0).is_none());
        assert!(b_c(2.0, f64::INFINITY, 0.0).is_none());
        assert!(b_c(2.0, 91.0, 0.0).is_none());
    }
}
