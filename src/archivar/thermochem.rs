pub const R_GAS: f64 = 8.314462618;
pub const P0_PA: f64 = 101325.0;

pub const SOLAR_H: f64 = 1.0;
pub const SOLAR_C: f64 = 3.63e-4;
pub const SOLAR_N: f64 = 1.12e-4;
pub const SOLAR_O: f64 = 8.51e-4;

pub const M_H_KG: f64 = 1.00794e-3;
pub const M_C_KG: f64 = 12.0107e-3;
pub const M_O_KG: f64 = 15.9994e-3;
pub const M_N_KG: f64 = 14.0067e-3;

pub fn solar() -> [f64; 4] {
    [SOLAR_H, SOLAR_C, SOLAR_O, SOLAR_N]
}

pub const NELEM: usize = 4;

pub struct NasaSpecies {
    pub name: &'static str,
    pub low: [f64; 7],
    pub high: [f64; 7],
    pub formula: [i32; NELEM],
}

const T_MID: f64 = 1000.0;

pub fn h_over_rt(a: &[f64; 7], t: f64) -> f64 {
    a[0] + a[1] * t / 2.0
        + a[2] * t * t / 3.0
        + a[3] * t * t * t / 4.0
        + a[4] * t * t * t * t / 5.0
        + a[5] / t
}

pub fn s_over_r(a: &[f64; 7], t: f64) -> f64 {
    a[0] * t.ln()
        + a[1] * t
        + a[2] * t * t / 2.0
        + a[3] * t * t * t / 3.0
        + a[4] * t * t * t * t / 4.0
        + a[6]
}

pub fn gibbs_over_rt(spec: &NasaSpecies, t: f64) -> f64 {
    let a = if t <= T_MID { &spec.low } else { &spec.high };
    h_over_rt(a, t) - s_over_r(a, t)
}

pub fn species() -> Vec<NasaSpecies> {
    vec![
        NasaSpecies {
            name: "H2",
            formula: [2, 0, 0, 0],
            low: [
                2.34433112,
                7.98052075e-03,
                -1.9478151e-05,
                2.01572094e-08,
                -7.37611761e-12,
                -917.935173,
                0.683010238,
            ],
            high: [
                2.93286579,
                8.26607967e-04,
                -1.46402335e-07,
                1.54100359e-11,
                -6.88804432e-16,
                -813.065597,
                -1.02432887,
            ],
        },
        NasaSpecies {
            name: "H",
            formula: [1, 0, 0, 0],
            low: [2.5, 0.0, 0.0, 0.0, 0.0, 2.54736599e+04, -0.446682853],
            high: [
                2.50000286,
                -5.65334214e-09,
                3.63251723e-12,
                -9.1994972e-16,
                7.95260746e-20,
                2.54736589e+04,
                -0.446698494,
            ],
        },
        NasaSpecies {
            name: "O2",
            formula: [0, 0, 2, 0],
            low: [
                3.78245636,
                -2.99673415e-03,
                9.847302e-06,
                -9.68129508e-09,
                3.24372836e-12,
                -1063.94356,
                3.65767573,
            ],
            high: [
                3.66096083,
                6.56365523e-04,
                -1.41149485e-07,
                2.05797658e-11,
                -1.29913248e-15,
                -1215.97725,
                3.41536184,
            ],
        },
        NasaSpecies {
            name: "O",
            formula: [0, 0, 1, 0],
            low: [
                3.1682671,
                -3.27931884e-03,
                6.64306396e-06,
                -6.12806624e-09,
                2.11265971e-12,
                2.91222592e+04,
                2.05193346,
            ],
            high: [
                2.54363697,
                -2.73162486e-05,
                -4.1902952e-09,
                4.95481845e-12,
                -4.79553694e-16,
                2.9226012e+04,
                4.92229457,
            ],
        },
        NasaSpecies {
            name: "N2",
            formula: [0, 0, 0, 2],
            low: [
                3.53100528,
                -1.23660987e-04,
                -5.02999437e-07,
                2.43530612e-09,
                -1.40881235e-12,
                -1046.97628,
                2.96747468,
            ],
            high: [
                2.95257626,
                1.39690057e-03,
                -4.92631691e-07,
                7.86010367e-11,
                -4.60755321e-15,
                -923.948645,
                5.87189252,
            ],
        },
        NasaSpecies {
            name: "N",
            formula: [0, 0, 0, 1],
            low: [2.5, 0.0, 0.0, 0.0, 0.0, 5.61046378e+04, 4.19390932],
            high: [
                2.41594293,
                1.748906e-04,
                -1.19023667e-07,
                3.02262387e-11,
                -2.0360979e-15,
                5.61337748e+04,
                4.64960986,
            ],
        },
        NasaSpecies {
            name: "C",
            formula: [0, 1, 0, 0],
            low: [
                2.55423955,
                -3.21537724e-04,
                7.33792245e-07,
                -7.32234889e-10,
                2.66521446e-13,
                8.54438832e+04,
                4.53130848,
            ],
            high: [
                2.60558298,
                -1.95934335e-04,
                1.06737219e-07,
                -1.6423939e-11,
                8.18705752e-16,
                8.54129443e+04,
                4.19238681,
            ],
        },
        NasaSpecies {
            name: "H2O",
            formula: [2, 0, 1, 0],
            low: [
                4.19864056,
                -2.0364341e-03,
                6.52040211e-06,
                -5.48797062e-09,
                1.77197817e-12,
                -3.02937267e+04,
                -0.849032208,
            ],
            high: [
                2.67703787,
                2.97318329e-03,
                -7.7376969e-07,
                9.44336689e-11,
                -4.26900959e-15,
                -2.98858938e+04,
                6.88255571,
            ],
        },
        NasaSpecies {
            name: "OH",
            formula: [1, 0, 1, 0],
            low: [
                3.99201543,
                -2.40131752e-03,
                4.61793841e-06,
                -3.88113333e-09,
                1.3641147e-12,
                3615.08056,
                -0.103925458,
            ],
            high: [
                2.83864607,
                1.10725586e-03,
                -2.93914978e-07,
                4.20524247e-11,
                -2.42169092e-15,
                3943.95852,
                5.84452662,
            ],
        },
        NasaSpecies {
            name: "CO",
            formula: [0, 1, 1, 0],
            low: [
                3.57953347,
                -6.1035368e-04,
                1.01681433e-06,
                9.07005884e-10,
                -9.04424499e-13,
                -1.4344086e+04,
                3.50840928,
            ],
            high: [
                3.04848583,
                1.35172818e-03,
                -4.85794075e-07,
                7.88536486e-11,
                -4.69807489e-15,
                -1.42661171e+04,
                6.0170979,
            ],
        },
        NasaSpecies {
            name: "CO2",
            formula: [0, 1, 2, 0],
            low: [
                2.35677352,
                8.98459677e-03,
                -7.12356269e-06,
                2.45919022e-09,
                -1.43699548e-13,
                -4.83719697e+04,
                9.90105222,
            ],
            high: [
                4.63659493,
                2.74131991e-03,
                -9.95828531e-07,
                1.60373011e-10,
                -9.16103468e-15,
                -4.90249341e+04,
                -1.93534855,
            ],
        },
        NasaSpecies {
            name: "CH4",
            formula: [4, 1, 0, 0],
            low: [
                5.14987613,
                -0.0136709788,
                4.91800599e-05,
                -4.84743026e-08,
                1.66693956e-11,
                -1.02466476e+04,
                -4.64130376,
            ],
            high: [
                1.63552643,
                0.0100842795,
                -3.36916254e-06,
                5.34958667e-10,
                -3.15518833e-14,
                -1.00056455e+04,
                9.99313326,
            ],
        },
        NasaSpecies {
            name: "NH3",
            formula: [3, 0, 0, 1],
            low: [
                4.30177808,
                -4.7712733e-03,
                2.19341619e-05,
                -2.29856489e-08,
                8.28992268e-12,
                -6748.06394,
                -0.690644393,
            ],
            high: [
                2.71709692,
                5.56856338e-03,
                -1.76886396e-06,
                2.6741726e-10,
                -1.52731419e-14,
                -6584.51989,
                6.09289837,
            ],
        },
        NasaSpecies {
            name: "HCN",
            formula: [1, 1, 0, 1],
            low: [
                2.25901123,
                0.0100510591,
                -1.33514911e-05,
                1.00920882e-08,
                -3.00882048e-12,
                1.52158495e+04,
                8.9163459,
            ],
            high: [
                3.80231733,
                3.14630009e-03,
                -1.06315698e-06,
                1.66185395e-10,
                -9.79891789e-15,
                1.49104829e+04,
                1.57503584,
            ],
        },
        NasaSpecies {
            name: "NO",
            formula: [0, 0, 1, 1],
            low: [
                4.21859896,
                -4.63988124e-03,
                1.10443049e-05,
                -9.34055507e-09,
                2.80554874e-12,
                9845.09964,
                2.28061001,
            ],
            high: [
                3.26071234,
                1.19101135e-03,
                -4.29122646e-07,
                6.94481463e-11,
                -4.03295681e-15,
                9921.43132,
                6.36900518,
            ],
        },
        NasaSpecies {
            name: "NO2",
            formula: [0, 0, 2, 1],
            low: [
                3.94403907,
                -1.58547444e-03,
                1.66578984e-05,
                -2.04754478e-08,
                7.83503265e-12,
                2896.59865,
                6.31196225,
            ],
            high: [
                4.88474429,
                2.17241639e-03,
                -8.2807902e-07,
                1.57477293e-10,
                -1.05110549e-14,
                2316.48462,
                -0.117357075,
            ],
        },
    ]
}

pub fn molar_mass_kg(spec: &NasaSpecies) -> f64 {
    spec.formula[0] as f64 * M_H_KG
        + spec.formula[1] as f64 * M_C_KG
        + spec.formula[2] as f64 * M_O_KG
        + spec.formula[3] as f64 * M_N_KG
}

fn gauss_solve(a: &mut [Vec<f64>], b: &mut [f64]) {
    let n = b.len();
    for col in 0..n {
        let mut piv = col;
        for r in col + 1..n {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        a.swap(col, piv);
        b.swap(col, piv);
        let d = a[col][col];
        if d == 0.0 {
            continue;
        }
        for r in 0..n {
            if r == col {
                continue;
            }
            let f = a[r][col] / d;
            for c in col..n {
                a[r][c] -= f * a[col][c];
            }
            b[r] -= f * b[col];
        }
    }
    for r in 0..n {
        if a[r][r] != 0.0 {
            b[r] /= a[r][r];
        }
    }
}

fn residual5(lam: &[f64; 5], specs: &[NasaSpecies], t: f64, ln_p: f64, b: [f64; 4]) -> (bool, f64) {
    let big_n = lam[4].exp();
    if !big_n.is_finite() || big_n <= 0.0 {
        return (false, f64::INFINITY);
    }
    let mut r = [0.0f64; 5];
    for j in 0..4 {
        r[j] = -b[j];
    }
    let mut e_sum = 0.0f64;
    for s in specs.iter() {
        let mut phi = -gibbs_over_rt(s, t) - ln_p;
        for j in 0..4 {
            phi += s.formula[j] as f64 * lam[j];
        }
        let e = phi.exp();
        if !e.is_finite() {
            return (false, f64::INFINITY);
        }
        e_sum += e;
        let ni = big_n * e;
        for j in 0..4 {
            r[j] += s.formula[j] as f64 * ni;
        }
    }
    r[4] = e_sum - 1.0;
    let norm = r.iter().map(|x| x * x).sum::<f64>().sqrt();
    (norm.is_finite(), norm)
}

fn newton_step(specs: &[NasaSpecies], t: f64, ln_p: f64, b: [f64; 4], lam: &mut [f64; 5]) -> bool {
    for _ in 0..80 {
        let big_n = lam[4].exp();
        let mut e = [0.0f64; 16];
        let mut n = [0.0f64; 16];
        for (i, s) in specs.iter().enumerate() {
            let mut phi = -gibbs_over_rt(s, t) - ln_p;
            for j in 0..4 {
                phi += s.formula[j] as f64 * lam[j];
            }
            e[i] = phi.exp();
            n[i] = big_n * e[i];
        }
        let mut r = [0.0f64; 5];
        for j in 0..4 {
            r[j] = -b[j];
            for (i, s) in specs.iter().enumerate() {
                r[j] += s.formula[j] as f64 * n[i];
            }
        }
        r[4] = e.iter().sum::<f64>() - 1.0;
        let rel = (0..4)
            .map(|j| (r[j] / b[j].max(1e-30)).abs())
            .fold(r[4].abs(), f64::max);
        if rel < 1e-10 {
            return true;
        }
        let mut jac = vec![vec![0.0f64; 5]; 5];
        for j in 0..4 {
            for k in 0..4 {
                for (i, s) in specs.iter().enumerate() {
                    jac[j][k] += s.formula[j] as f64 * s.formula[k] as f64 * n[i];
                }
            }
        }
        for j in 0..4 {
            for (i, s) in specs.iter().enumerate() {
                jac[j][4] += s.formula[j] as f64 * n[i];
            }
        }
        for k in 0..4 {
            for (i, s) in specs.iter().enumerate() {
                jac[4][k] += s.formula[k] as f64 * e[i];
            }
        }
        let mut dl = [-r[0], -r[1], -r[2], -r[3], -r[4]];
        gauss_solve(&mut jac, &mut dl);
        let norm0 = r.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mut alpha = 1.0f64;
        let mut accepted = false;
        while alpha > 1e-14 {
            let mut trial = [0.0f64; 5];
            for k in 0..5 {
                trial[k] = lam[k] + alpha * dl[k];
            }
            let (ok, norm) = residual5(&trial, specs, t, ln_p, b);
            if ok && norm < norm0 {
                *lam = trial;
                accepted = true;
                break;
            }
            alpha *= 0.5;
        }
        if !accepted {
            return false;
        }
    }
    false
}

pub fn equilibrium_composition(t_k: f64, p_pa: f64, b: [f64; 4]) -> Option<Vec<f64>> {
    if !t_k.is_finite() || t_k < 500.0 || t_k > 3000.0 || !p_pa.is_finite() || p_pa <= 0.0 {
        return None;
    }
    if b.iter().any(|x| !x.is_finite() || *x <= 0.0) {
        return None;
    }
    let specs = species();
    let ln_p = (p_pa / P0_PA).ln();

    let atomic_idx = [1usize, 6, 3, 5];
    let mut lam = [0.0f64; 5];
    for (j, &ai) in atomic_idx.iter().enumerate() {
        lam[j] = gibbs_over_rt(&specs[ai], 6000.0) + ln_p + b[j].ln();
    }
    lam[4] = 0.0;

    let mut tcur = 6000.0f64;
    let mut converged = true;
    while tcur > t_k {
        let next = (tcur * 0.96).max(t_k);
        converged = newton_step(&specs, next, ln_p, b, &mut lam);
        if !converged {
            break;
        }
        tcur = next;
    }
    if !converged {
        return None;
    }
    let big_n = lam[4].exp();
    let mut n = vec![0.0f64; specs.len()];
    for (i, s) in specs.iter().enumerate() {
        let mut phi = -gibbs_over_rt(s, t_k) - ln_p;
        for j in 0..4 {
            phi += s.formula[j] as f64 * lam[j];
        }
        n[i] = big_n * phi.exp();
    }
    let total: f64 = n.iter().sum();
    if !total.is_finite() || total <= 0.0 {
        return None;
    }
    for x in &mut n {
        *x /= total;
    }
    Some(n)
}

pub fn equilibrium_concentrations(t_k: f64, p_pa: f64, b: [f64; 4]) -> Option<Vec<f64>> {
    let x = equilibrium_composition(t_k, p_pa, b)?;
    let specs = species();
    let mut rho = Vec::with_capacity(x.len());
    for (i, xi) in x.iter().enumerate() {
        rho.push(xi * molar_mass_kg(&specs[i]) * p_pa / (R_GAS * t_k));
    }
    Some(rho)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idx(name: &str) -> usize {
        species()
            .iter()
            .position(|s| s.name == name)
            .unwrap_or_else(|| panic!("species {} absent", name))
    }

    fn g(s: &NasaSpecies, t: f64) -> f64 {
        gibbs_over_rt(s, t)
    }

    #[test]
    fn conservation_is_exact() {
        let x = equilibrium_composition(1200.0, P0_PA, solar()).unwrap();
        let specs = species();
        let mut a = [0.0f64; 4];
        for (i, s) in specs.iter().enumerate() {
            for j in 0..4 {
                a[j] += s.formula[j] as f64 * x[i];
            }
        }
        let r = |v: f64| v / a[0];
        assert!(
            (r(a[1]) / SOLAR_C - 1.0).abs() < 1e-6,
            "C balance, got {}",
            r(a[1])
        );
        assert!(
            (r(a[2]) / SOLAR_O - 1.0).abs() < 1e-6,
            "O balance, got {}",
            r(a[2])
        );
        assert!(
            (r(a[3]) / SOLAR_N - 1.0).abs() < 1e-6,
            "N balance, got {}",
            r(a[3])
        );
    }

    #[test]
    fn equilibrium_matches_direct_kp() {
        let t = 1200.0f64;
        let x = equilibrium_composition(t, P0_PA, solar()).unwrap();
        let specs = species();
        let (ic, ih2, ih2o, ich4) = (idx("CO"), idx("H2"), idx("H2O"), idx("CH4"));
        let dg =
            g(&specs[ich4], t) + g(&specs[ih2o], t) - g(&specs[ic], t) - 3.0 * g(&specs[ih2], t);
        let kp = (-dg).exp();
        let from_solver = (x[ich4] * x[ih2o]) / (x[ic] * x[ih2].powi(3));
        assert!(
            (from_solver / kp - 1.0).abs() < 1e-6,
            "solver Kp {} vs direct {}",
            from_solver,
            kp
        );
    }

    #[test]
    fn co_methane_crossover_matches_the_paper() {
        let x_cold = equilibrium_composition(700.0, P0_PA, solar()).unwrap();
        let x_hot = equilibrium_composition(1600.0, P0_PA, solar()).unwrap();
        let (ic, ich4) = (idx("CO"), idx("CH4"));
        assert!(x_cold[ich4] > x_cold[ic], "CH4 should dominate at 700 K");
        assert!(x_hot[ic] > x_hot[ich4], "CO should dominate at 1600 K");
    }

    #[test]
    fn water_is_the_oxygen_reservoir_at_low_t() {
        let x = equilibrium_composition(600.0, P0_PA, solar()).unwrap();
        let ih2o = idx("H2O");
        assert!(
            (x[ih2o] / (2.0 * SOLAR_O) - 1.0).abs() < 0.05,
            "H2O ≈ 2·O/H at 600 K, got {:.3e}",
            x[ih2o]
        );
    }

    #[test]
    fn nitrogen_n2_above_ammonia_below() {
        let x_cold = equilibrium_composition(600.0, P0_PA, solar()).unwrap();
        let x_hot = equilibrium_composition(1300.0, P0_PA, solar()).unwrap();
        let (in2, inh3) = (idx("N2"), idx("NH3"));
        assert!(x_cold[inh3] > 0.1 * x_cold[in2], "NH3 forms below ~800 K");
        assert!(x_hot[inh3] < x_hot[in2], "N2 dominates above ~800 K");
    }

    #[test]
    fn h2_dominates_the_gas() {
        let x = equilibrium_composition(1200.0, P0_PA, solar()).unwrap();
        let ih2 = idx("H2");
        assert!(x[ih2] > 0.99, "H2 should dominate, got {}", x[ih2]);
    }

    #[test]
    fn refuses_out_of_domain() {
        assert!(equilibrium_composition(300.0, P0_PA, solar()).is_none());
        assert!(equilibrium_composition(4000.0, P0_PA, solar()).is_none());
        assert!(equilibrium_composition(1200.0, 0.0, solar()).is_none());
        assert!(equilibrium_composition(1200.0, P0_PA, [0.0, SOLAR_C, SOLAR_O, SOLAR_N]).is_none());
        assert!(equilibrium_composition(f64::NAN, P0_PA, solar()).is_none());
    }

    #[test]
    fn molar_mass_is_the_formula_weight() {
        let specs = species();
        let m = |name: &str| molar_mass_kg(&specs[idx(name)]);
        assert!((m("H2") - 2.0 * M_H_KG).abs() < 1e-12);
        assert!((m("H") - M_H_KG).abs() < 1e-12);
        assert!((m("CO2") - (M_C_KG + 2.0 * M_O_KG)).abs() < 1e-12);
        assert!((m("NH3") - (3.0 * M_H_KG + M_N_KG)).abs() < 1e-12);
    }

    #[test]
    fn concentrations_are_the_ideal_gas_mass_density() {
        let t = 1200.0;
        let rho = equilibrium_concentrations(t, P0_PA, solar()).unwrap();
        let specs = species();
        let x = equilibrium_composition(t, P0_PA, solar()).unwrap();
        let mut mean_m = 0.0;
        for (i, xi) in x.iter().enumerate() {
            let expected = xi * molar_mass_kg(&specs[i]) * P0_PA / (R_GAS * t);
            assert!(
                (rho[i] - expected).abs() / expected < 1e-12,
                "concentration {} disagrees with ideal gas",
                specs[i].name
            );
            mean_m += xi * molar_mass_kg(&specs[i]);
        }
        let total: f64 = rho.iter().sum();
        let density = mean_m * P0_PA / (R_GAS * t);
        assert!(
            (total - density).abs() / density < 1e-12,
            "total density {} vs ideal gas {}",
            total,
            density
        );
    }

    #[test]
    fn concentrations_refuse_out_of_domain() {
        assert!(equilibrium_concentrations(300.0, P0_PA, solar()).is_none());
        assert!(equilibrium_concentrations(1200.0, 0.0, solar()).is_none());
    }
}
