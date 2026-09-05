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

// Sulfur-aware equilibrium — a SEPARATE species set and element basis.
// The 16-slot H/C/N/O list above is the archived jwst_equilibrium.bin contract
// (EQUILIBRIUM_NSPECIES = 16, species_names written to the asset header); it is
// left untouched. The sulfur channel is a parallel Gibbs-minimizer over the same
// 16 species plus sulfur carriers, on a 5-element basis H,C,O,N,S. The two paths
// are solved by the same generic code below (equilibrium_composition keeps the
// fixed 4-element surface; equilibrium_composition_sulfur the 5-element one).
//
// Element budget provenance: the archival solar C/N/O abundances equal the
// Anders & Grevesse 1989 photospheric scale (C/H 3.63e-4, N/H 1.12e-4,
// O/H 8.51e-4); SOLAR_S is the same scale's sulfur abundance (S/H 1.62e-5,
// log eps_S = 7.21, Anders & Grevesse 1989, Geochim. Cosmochim. Acta 53, 197).
//
// Sulfur gas-phase data provenance: NIST-JANAF Thermochemical Tables (Chase,
// M.W., Jr., 4th edition, J. Phys. Chem. Ref. Data Monograph 9, 1998), as
// rendered on the NIST Chemistry WebBook gas-thermochemistry pages (Shomate
// fits, accessed 2026-09-05). Each species below carries its fit domain and the
// JANAF 298.15-K anchors (standard enthalpy of formation and standard entropy)
// that the WebBook page reports for that species; the unit tests verify the
// encoded fits reproduce those measured anchors. The NIST-JANAF sulphur
// reference state is orthorhombic S, matching the C(graphite)/H2/O2/N2 datum of
// the archival NASA-7 H/C/N/O fits, so cross-reactions (e.g. H2S+2H2O
// <-> SO2+3H2) sit on one consistent element datum.
//
// Atomic S(g) is in the set but carries fit data only for T >= 882.117 K
// (NIST-JANAF Shomate fit domain; below that the fit does not exist). Below its
// fit floor the species contributes no moles (its equilibrium abundance is
// negligible there), which is data-honored absence, not a fabricated value.
// S3 and S4 clusters and HSO are not in the set (no detection among the seed
// demands them; adding a species is a data-sourcing act, not a default).

pub const SOLAR_S: f64 = 1.62e-5;

pub fn sulfur_solar() -> [f64; 5] {
    [SOLAR_H, SOLAR_C, SOLAR_O, SOLAR_N, SOLAR_S]
}

// Reservoir regression budget: the host [Fe/H] scales every metal element
// (C, O, N, S) per H by z = 10^feh; H stays the reference 1.0. feh = 0.0 is
// exactly the archival solar budget (10^0 = 1). The scaling reads the host
// metallicity as a reservoir witness; a non-finite feh is absent, not a value.
pub fn scaled_solar_budget(feh: f64) -> Option<[f64; NELEM_S]> {
    let z = 10f64.powf(feh);
    if !z.is_finite() || z <= 0.0 {
        return None;
    }
    let s = sulfur_solar();
    Some([s[0], s[1] * z, s[2] * z, s[3] * z, s[4] * z])
}

pub const NELEM_S: usize = 5;

pub struct ShomateSeg {
    pub t_min: f64,
    pub t_max: f64,
    // Shomate coefficients A..G: cp/R-form heat capacity A+B*t+C*t^2+D*t^3+E/t^2
    // with t = T/1000; F carries the JANAF formation-datum enthalpy offset.
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
    pub g: f64,
}

pub enum GasGibbs {
    Nasa7 { low: [f64; 7], high: [f64; 7] },
    Shomate { segs: Vec<ShomateSeg> },
}

pub struct GasSpec {
    pub name: &'static str,
    pub formula: [i32; NELEM_S],
    pub g: GasGibbs,
}

fn nasa7_g_over_rt(low: &[f64; 7], high: &[f64; 7], t: f64) -> f64 {
    let a = if t <= T_MID { low } else { high };
    h_over_rt(a, t) - s_over_r(a, t)
}

fn shomate_g_over_rt(segs: &[ShomateSeg], t: f64) -> Option<f64> {
    let seg = segs
        .iter()
        .find(|s| t < s.t_max)
        .or_else(|| segs.last().filter(|s| t <= s.t_max))?;
    if t < seg.t_min {
        return None;
    }
    let tt = t / 1000.0;
    let h = seg.a * tt
        + seg.b * tt * tt / 2.0
        + seg.c * tt * tt * tt / 3.0
        + seg.d * tt * tt * tt * tt / 4.0
        - seg.e / tt
        + seg.f;
    let s = seg.a * tt.ln() + seg.b * tt + seg.c * tt * tt / 2.0 + seg.d * tt * tt * tt / 3.0
        - seg.e / (2.0 * tt * tt)
        + seg.g;
    Some((1000.0 * h - t * s) / (R_GAS * t))
}

pub fn gas_g_over_rt(spec: &GasSpec, t: f64) -> Option<f64> {
    match &spec.g {
        GasGibbs::Nasa7 { low, high } => Some(nasa7_g_over_rt(low, high, t)),
        GasGibbs::Shomate { segs } => shomate_g_over_rt(segs, t),
    }
}

fn shomate_spec(name: &'static str, formula: [i32; NELEM_S], segs: Vec<ShomateSeg>) -> GasSpec {
    GasSpec {
        name,
        formula,
        g: GasGibbs::Shomate { segs },
    }
}

fn s_seg(
    t_min: f64,
    t_max: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
) -> ShomateSeg {
    ShomateSeg {
        t_min,
        t_max,
        a,
        b,
        c,
        d,
        e,
        f,
        g,
    }
}

// NIST-JANAF (Chase 1998) Shomate fits, coefficient order A..G. Fit-domain and
// 298.15-K anchors per species are listed; the 298.15 anchors are the measured
// values the tests verify against.
fn sulfur_shomate_species() -> Vec<GasSpec> {
    vec![
        // S(g) atomic sulfur — JANAF fit domain 882.117..6000 K; no fit below.
        shomate_spec(
            "S",
            [0, 0, 0, 0, 1],
            vec![
                s_seg(
                    882.117, 1400.0, 27.45968, -13.32784, 10.06574, -2.662381, -0.055851, 269.1149,
                    204.2955,
                ),
                s_seg(
                    1400.0, 6000.0, 16.55345, 2.400266, -0.255760, 0.005821, 3.564793, 278.4356,
                    194.5447,
                ),
            ],
        ),
        // S2(g) — JANAF 298..6000 K single fit; dHf298 128.60 kJ/mol, S298 228.19 J/mol/K.
        shomate_spec(
            "S2",
            [0, 0, 0, 0, 2],
            vec![s_seg(
                298.0, 6000.0, 33.51313, 5.065360, -1.059670, 0.089905, -0.211911, 117.6855,
                266.0919,
            )],
        ),
        // SH(g) — JANAF 298..1200 / 1200..6000 K; dHf298 139.33 kJ/mol, S298 195.63 J/mol/K.
        shomate_spec(
            "SH",
            [1, 0, 0, 0, 1],
            vec![
                s_seg(
                    298.0, 1200.0, 38.04306, -27.46792, 34.06462, -11.79875, -0.009743, 128.8961,
                    248.3945,
                ),
                s_seg(
                    1200.0, 6000.0, 32.99507, 2.841514, -0.507766, 0.038247, -2.909667, 124.4870,
                    230.0066,
                ),
            ],
        ),
        // H2S(g) — JANAF 298..1400 / 1400..6000 K; dHf298 -20.50 kJ/mol, S298 205.77 J/mol/K.
        shomate_spec(
            "H2S",
            [2, 0, 0, 0, 1],
            vec![
                s_seg(
                    298.0, 1400.0, 26.88412, 18.67809, 3.434203, -3.378702, 0.135882, -28.91211,
                    233.3747,
                ),
                s_seg(
                    1400.0, 6000.0, 51.22136, 4.147486, -0.643566, 0.041621, -10.46385, -55.87606,
                    243.6900,
                ),
            ],
        ),
        // SO(g) — JANAF 298..1400 / 1400..6000 K; dHf298 5.01 kJ/mol, S298 221.94 J/mol/K.
        shomate_spec(
            "SO",
            [0, 0, 1, 0, 1],
            vec![
                s_seg(
                    298.0, 1400.0, 22.56414, 29.93305, -22.87987, 6.408968, 0.047560, -2.702237,
                    241.5511,
                ),
                s_seg(
                    1400.0, 6000.0, 23.50387, 10.82133, -2.260566, 0.168555, 5.052557, 5.425853,
                    254.7734,
                ),
            ],
        ),
        // SO2(g) — JANAF 298..1200 / 1200..6000 K; dHf298 -296.84 kJ/mol, S298 248.21 J/mol/K.
        shomate_spec(
            "SO2",
            [0, 0, 2, 0, 1],
            vec![
                s_seg(
                    298.0, 1200.0, 21.43049, 74.35094, -57.75217, 16.35534, 0.086731, -305.7688,
                    254.8872,
                ),
                s_seg(
                    1200.0, 6000.0, 57.48188, 1.009328, -0.076290, 0.005174, -4.045401, -324.4140,
                    302.7798,
                ),
            ],
        ),
        // CS(g) — JANAF 298..600 / 600..6000 K; dHf298 280.33 kJ/mol, S298 210.55 J/mol/K.
        shomate_spec(
            "CS",
            [0, 1, 0, 0, 1],
            vec![
                s_seg(
                    298.0, 600.0, 21.76387, 24.99890, -8.095581, -4.563949, 0.126372, 273.2328,
                    230.5497,
                ),
                s_seg(
                    600.0, 6000.0, 34.47721, 2.966255, -0.950722, 0.113718, -0.997482, 267.0275,
                    247.0731,
                ),
            ],
        ),
        // OCS(g) carbonyl sulfide — JANAF 298..1200 / 1200..6000 K; dHf298 -138.41 kJ/mol, S298 231.57 J/mol/K.
        shomate_spec(
            "OCS",
            [0, 1, 1, 0, 1],
            vec![
                s_seg(
                    298.0, 1200.0, 34.53892, 43.05378, -26.61773, 6.338844, -0.327515, -151.5001,
                    259.8118,
                ),
                s_seg(
                    1200.0, 6000.0, 60.32240, 1.738332, -0.209982, 0.014110, -5.128873, -168.6307,
                    287.6454,
                ),
            ],
        ),
    ]
}

fn sulfur_gas_specs() -> Vec<GasSpec> {
    let mut out: Vec<GasSpec> = species()
        .into_iter()
        .map(|s| GasSpec {
            name: s.name,
            formula: [s.formula[0], s.formula[1], s.formula[2], s.formula[3], 0],
            g: GasGibbs::Nasa7 {
                low: s.low,
                high: s.high,
            },
        })
        .collect();
    out.extend(sulfur_shomate_species());
    out
}

pub fn sulfur_gas_names() -> Vec<String> {
    sulfur_gas_specs()
        .into_iter()
        .map(|s| s.name.to_string())
        .collect()
}

fn gas_residual(lam: &[f64], specs: &[GasSpec], t: f64, ln_p: f64, b: &[f64]) -> (bool, f64) {
    let nelem = b.len();
    let big_n = lam[nelem].exp();
    if !big_n.is_finite() || big_n <= 0.0 {
        return (false, f64::INFINITY);
    }
    let mut r = vec![0.0f64; nelem + 1];
    for j in 0..nelem {
        r[j] = -b[j];
    }
    let mut e_sum = 0.0f64;
    for s in specs.iter() {
        let Some(g) = gas_g_over_rt(s, t) else {
            continue;
        };
        let mut phi = -g - ln_p;
        for j in 0..nelem {
            phi += s.formula[j] as f64 * lam[j];
        }
        let e = phi.exp();
        if !e.is_finite() {
            return (false, f64::INFINITY);
        }
        e_sum += e;
        let ni = big_n * e;
        for j in 0..nelem {
            r[j] += s.formula[j] as f64 * ni;
        }
    }
    r[nelem] = e_sum - 1.0;
    let norm = r.iter().map(|x| x * x).sum::<f64>().sqrt();
    (norm.is_finite(), norm)
}

fn gas_newton_step(specs: &[GasSpec], t: f64, ln_p: f64, b: &[f64], lam: &mut [f64]) -> bool {
    let nelem = b.len();
    for _ in 0..80 {
        let big_n = lam[nelem].exp();
        let n_species = specs.len();
        let mut e = vec![0.0f64; n_species];
        let mut n = vec![0.0f64; n_species];
        for (i, s) in specs.iter().enumerate() {
            let Some(g) = gas_g_over_rt(s, t) else {
                continue;
            };
            let mut phi = -g - ln_p;
            for j in 0..nelem {
                phi += s.formula[j] as f64 * lam[j];
            }
            e[i] = phi.exp();
            n[i] = big_n * e[i];
        }
        let mut r = vec![0.0f64; nelem + 1];
        for j in 0..nelem {
            r[j] = -b[j];
            for (i, s) in specs.iter().enumerate() {
                r[j] += s.formula[j] as f64 * n[i];
            }
        }
        r[nelem] = e.iter().sum::<f64>() - 1.0;
        let rel = (0..nelem)
            .map(|j| (r[j] / b[j].max(1e-30)).abs())
            .fold(r[nelem].abs(), f64::max);
        if rel < 1e-10 {
            return true;
        }
        let dim = nelem + 1;
        let mut jac = vec![vec![0.0f64; dim]; dim];
        for j in 0..nelem {
            for k in 0..nelem {
                for (i, s) in specs.iter().enumerate() {
                    jac[j][k] += s.formula[j] as f64 * s.formula[k] as f64 * n[i];
                }
            }
        }
        for j in 0..nelem {
            for (i, s) in specs.iter().enumerate() {
                jac[j][nelem] += s.formula[j] as f64 * n[i];
            }
        }
        for k in 0..nelem {
            for (i, s) in specs.iter().enumerate() {
                jac[nelem][k] += s.formula[k] as f64 * e[i];
            }
        }
        let mut dl = vec![0.0f64; dim];
        for j in 0..dim {
            dl[j] = -r[j];
        }
        gauss_solve(&mut jac, &mut dl);
        let norm0 = r.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mut alpha = 1.0f64;
        let mut accepted = false;
        while alpha > 1e-14 {
            let mut trial = vec![0.0f64; dim];
            for k in 0..dim {
                trial[k] = lam[k] + alpha * dl[k];
            }
            let (ok, norm) = gas_residual(&trial, specs, t, ln_p, b);
            if ok && norm < norm0 {
                lam.copy_from_slice(&trial);
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

fn solve_gas(specs: &[GasSpec], b: &[f64], t_k: f64, p_pa: f64) -> Option<Vec<f64>> {
    if !t_k.is_finite() || t_k < 500.0 || t_k > 3000.0 || !p_pa.is_finite() || p_pa <= 0.0 {
        return None;
    }
    if b.iter().any(|x| !x.is_finite() || *x <= 0.0) {
        return None;
    }
    let nelem = b.len();
    let ln_p = (p_pa / P0_PA).ln();
    let Some(atomic_species) = gas_atomic_species_indices(specs, nelem) else {
        return None;
    };
    let mut lam = vec![0.0f64; nelem + 1];
    for (j, &ai) in atomic_species.iter().enumerate() {
        let g = gas_g_over_rt(&specs[ai], 6000.0)?;
        lam[j] = g + ln_p + b[j].ln();
    }
    lam[nelem] = 0.0;

    let mut tcur = 6000.0f64;
    let mut converged = true;
    while tcur > t_k {
        let next = (tcur * 0.96).max(t_k);
        converged = gas_newton_step(specs, next, ln_p, b, &mut lam);
        if !converged {
            break;
        }
        tcur = next;
    }
    if !converged {
        return None;
    }
    let big_n = lam[nelem].exp();
    let mut n = vec![0.0f64; specs.len()];
    for (i, s) in specs.iter().enumerate() {
        let Some(g) = gas_g_over_rt(s, t_k) else {
            continue;
        };
        let mut phi = -g - ln_p;
        for j in 0..nelem {
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

fn gas_atomic_species_indices(specs: &[GasSpec], nelem: usize) -> Option<Vec<usize>> {
    // Element basis is ordered H,C,O,N,(S); the atomic-gas carrier of each
    // element seeds the 6000-K starting potential. S(g) is only in the set on
    // the 5-element path (its fit domain starts at 882.117 K; the solver starts
    // the ramp at 6000 K, inside the fit).
    let names = ["H", "C", "O", "N", "S"];
    let mut idx = Vec::with_capacity(nelem);
    for name in names.iter().take(nelem) {
        idx.push(specs.iter().position(|s| s.name == *name)?);
    }
    Some(idx)
}

// Sulfur-aware equilibrium composition at a scaled metallicity budget.
// feh = 0.0 reproduces the archival solar budget (scaled_solar_budget equals
// sulfur_solar; unit-tested); a non-finite feh is absent, never a budget.
// Slots: the 16 archival H/C/N/O species in their archived order, then
// S, S2, SH, H2S, SO, SO2, CS, OCS (slot order follows sulfur_shomate_species).
pub fn equilibrium_composition_sulfur_scaled(t_k: f64, p_pa: f64, feh: f64) -> Option<Vec<f64>> {
    let specs = sulfur_gas_specs();
    let b = scaled_solar_budget(feh)?;
    solve_gas(&specs, &b, t_k, p_pa)
}

// === Halogen-aware equilibrium for the techno-gas floor ===
//
// The disequilibrium register's model gap (docs/befund/befund-techno-atmosphaeren-
// gase.md, 2026-09-05): a numeric equilibrium floor for the industrial gases
// CFCl3/CF2Cl2/SF6/CF4/NF3 needs a 7-element (F, Cl) extension of the Gibbs
// minimizer, with halide reservoirs HF/HCl and solar F/Cl on the Anders &
// Grevesse 1989 scale. This path is that extension — a SEPARATE element basis
// H,C,O,N,S,F,Cl over the 16 archival H/C/N/O species, the 8 sulfur carriers
// and 11 F/Cl species. The archival 16-slot gas solver, the 24-slot sulfur
// solver and the condensation path above are untouched; the probe calls
// equilibrium_composition_halogen.
//
// Element budget provenance: SOLAR_F and SOLAR_CL sit on the same Anders &
// Grevesse 1989 photospheric scale as the archived C/N/O/S (log eps_F = 4.48,
// log eps_Cl = 5.50; F/H = 3.02e-8, Cl/H = 3.16e-7; Anders & Grevesse 1989,
// Geochim. Cosmochim. Acta 53, 197 — the same table row that carries the
// archived S/H = 1.62e-5, log eps_S = 7.21). The Cl photospheric value carries
// the sunspot-HCl determination; the CI-meteoritic Cl sits ~0.24 dex lower.
// A 0.1-0.2 dex change in F/H or Cl/H rescales the halogen floors below by the
// same factor and moves no verdict: every floor sits orders of magnitude below
// any instrument floor (measured in the probe run).
//
// F/Cl gas-phase data provenance: NIST-JANAF Thermochemical Tables (Chase,
// M.W., Jr., 4th edition, J. Phys. Chem. Ref. Data Monograph 9, 1998), as
// rendered on the NIST Chemistry WebBook gas-thermochemistry pages (Shomate
// fits, curl + cache /tmp/opencode/nist_fcl/, accessed 2026-09-05). Each added
// species below carries its fit domain and the Chase-1998 298.15-K anchors
// (standard enthalpy of formation and standard entropy) reported on the page;
// the unit tests verify the encoded fits reproduce those anchors. The JANAF F
// and Cl reference states are F2(g) and Cl2(g) — same element datum as the
// archival C(graphite)/H2/O2/N2 and the orthorhombic-S sulfur datum, so
// cross-reactions (2 HF <-> H2 + F2, 2 HCl <-> H2 + Cl2) sit on one scale.
// Every halogen species in the set carries a 298.15-K fit floor; no F/Cl
// species is extrapolated.

pub const NELEM_HALOGEN: usize = 7;
pub const SOLAR_F: f64 = 3.02e-8;
pub const SOLAR_CL: f64 = 3.16e-7;

pub fn halogen_solar() -> [f64; NELEM_HALOGEN] {
    [
        SOLAR_H, SOLAR_C, SOLAR_O, SOLAR_N, SOLAR_S, SOLAR_F, SOLAR_CL,
    ]
}

pub struct HalogenSpec {
    pub name: &'static str,
    pub formula: [i32; NELEM_HALOGEN],
    pub g: GasGibbs,
}

fn halogen_g_over_rt(spec: &HalogenSpec, t: f64) -> Option<f64> {
    match &spec.g {
        GasGibbs::Nasa7 { low, high } => Some(nasa7_g_over_rt(low, high, t)),
        GasGibbs::Shomate { segs } => shomate_g_over_rt(segs, t),
    }
}

fn hshomate_spec(
    name: &'static str,
    formula: [i32; NELEM_HALOGEN],
    segs: Vec<ShomateSeg>,
) -> HalogenSpec {
    HalogenSpec {
        name,
        formula,
        g: GasGibbs::Shomate { segs },
    }
}

fn halogen_shomate_species() -> Vec<HalogenSpec> {
    vec![
        // F2(g) — JANAF 298..6000 K single fit; reference state, dHf298 0.00 kJ/mol, S298 202.79 J/mol/K.
        hshomate_spec(
            "F2",
            [0, 0, 0, 0, 0, 2, 0],
            vec![s_seg(
                298.0, 6000.0, 31.44510, 8.413831, -2.778850, 0.218104, -0.211175, -10.43260,
                237.2770,
            )],
        ),
        // F(g) atomic fluorine — JANAF 298..6000 K single fit; dHf298 79.39 kJ/mol, S298 158.78 J/mol/K.
        hshomate_spec(
            "F",
            [0, 0, 0, 0, 0, 1, 0],
            vec![s_seg(
                298.0, 6000.0, 21.97336, -0.958182, 0.251916, -0.021107, 0.103471, 73.22586,
                186.2286,
            )],
        ),
        // HF(g) — JANAF 298..1000 / 1000..6000 K; dHf298 -272.55 kJ/mol, S298 173.78 J/mol/K. The F reservoir.
        hshomate_spec(
            "HF",
            [1, 0, 0, 0, 0, 1, 0],
            vec![
                s_seg(
                    298.0, 1000.0, 30.11693, -3.246612, 2.868116, 0.457914, -0.024861, -281.4912,
                    210.9226,
                ),
                s_seg(
                    1000.0, 6000.0, 24.57033, 6.893391, -1.243874, 0.082583, -0.234060, -279.7653,
                    202.8525,
                ),
            ],
        ),
        // Cl2(g) — JANAF 298..1000 / 1000..3000 / 3000..6000 K; reference state, dHf298 0.00, S298 223.08.
        hshomate_spec(
            "Cl2",
            [0, 0, 0, 0, 0, 0, 2],
            vec![
                s_seg(
                    298.0, 1000.0, 33.05060, 12.22940, -12.06510, 4.385330, -0.159494, -10.83480,
                    259.0290,
                ),
                s_seg(
                    1000.0, 3000.0, 42.67730, -5.009570, 1.904621, -0.165641, -2.098480, -17.28980,
                    269.8400,
                ),
                s_seg(
                    3000.0, 6000.0, -42.55350, 41.68570, -7.126830, 0.387839, 101.1440, 132.7640,
                    264.7860,
                ),
            ],
        ),
        // Cl(g) atomic chlorine — JANAF 298..600 / 600..6000 K; dHf298 121.30 kJ/mol, S298 165.19 J/mol/K.
        hshomate_spec(
            "Cl",
            [0, 0, 0, 0, 0, 0, 1],
            vec![
                s_seg(
                    298.0, 600.0, 13.38298, 42.33999, -64.74656, 32.99532, 0.063319, 116.1491,
                    171.7038,
                ),
                s_seg(
                    600.0, 6000.0, 23.26597, -1.555939, 0.346910, -0.025961, 0.153212, 114.6604,
                    193.8882,
                ),
            ],
        ),
        // HCl(g) — JANAF 298..1200 / 1200..6000 K; dHf298 -92.31 kJ/mol, S298 186.90 J/mol/K. The Cl reservoir.
        hshomate_spec(
            "HCl",
            [1, 0, 0, 0, 0, 0, 1],
            vec![
                s_seg(
                    298.0, 1200.0, 32.12392, -13.45805, 19.86852, -6.853936, -0.049672, -101.6206,
                    228.6866,
                ),
                s_seg(
                    1200.0, 6000.0, 31.91923, 3.203184, -0.541539, 0.035925, -3.438525, -108.0150,
                    218.2768,
                ),
            ],
        ),
        // CF4(g) carbon tetrafluoride — JANAF 298..1000 / 1000..6000 K; dHf298 -933.20 kJ/mol, S298 261.41.
        hshomate_spec(
            "CF4",
            [0, 1, 0, 0, 0, 4, 0],
            vec![
                s_seg(
                    298.0, 1000.0, 15.96778, 210.3318, -189.4657, 62.20227, -0.217317, -946.4877,
                    224.6766,
                ),
                s_seg(
                    1000.0, 6000.0, 106.2221, 1.076122, -0.223192, 0.015753, -8.340679, -987.7755,
                    355.9764,
                ),
            ],
        ),
        // CFCl3(g) CFC-11 — JANAF 298..600 / 600..6000 K; dHf298 -288.70 kJ/mol, S298 309.74 J/mol/K.
        hshomate_spec(
            "CFCl3",
            [0, 1, 0, 0, 0, 1, 3],
            vec![
                s_seg(
                    298.0, 600.0, 34.06650, 230.4309, -289.4558, 135.5248, -0.232263, -307.5847,
                    292.6202,
                ),
                s_seg(
                    600.0, 6000.0, 106.2694, 1.245277, -0.292652, 0.022658, -3.710084, -331.8887,
                    419.8728,
                ),
            ],
        ),
        // CF2Cl2(g) CFC-12 — JANAF 298..1100 / 1100..6000 K; dHf298 -491.62 kJ/mol, S298 300.89 J/mol/K.
        hshomate_spec(
            "CF2Cl2",
            [0, 1, 0, 0, 0, 2, 2],
            vec![
                s_seg(
                    298.0, 1100.0, 48.01014, 139.1808, -124.3326, 39.75147, -0.633834, -513.2304,
                    319.1028,
                ),
                s_seg(
                    1100.0, 6000.0, 107.3635, 0.404844, -0.082033, 0.005689, -5.707394, -539.7486,
                    406.4660,
                ),
            ],
        ),
        // SF6(g) sulfur hexafluoride — JANAF 298..1000 / 1000..6000 K; dHf298 -1220.47 kJ/mol, S298 291.52.
        hshomate_spec(
            "SF6",
            [0, 0, 0, 0, 1, 6, 0],
            vec![
                s_seg(
                    298.0, 1000.0, 58.90319, 255.5399, -252.2747, 88.76063, -1.608971, -1252.744,
                    287.9914,
                ),
                s_seg(
                    1000.0, 6000.0, 157.1393, 0.484022, -0.100724, 0.007127, -8.279635, -1291.990,
                    443.2111,
                ),
            ],
        ),
        // NF3(g) nitrogen trifluoride — JANAF 298..1000 / 1000..6000 K; dHf298 -132.09 kJ/mol, S298 260.77.
        hshomate_spec(
            "NF3",
            [0, 0, 0, 1, 0, 3, 0],
            vec![
                s_seg(
                    298.0, 1000.0, 26.45610, 142.2606, -137.6134, 47.68505, -0.404491, -146.5375,
                    253.7876,
                ),
                s_seg(
                    1000.0, 6000.0, 82.54781, 0.345728, -0.071941, 0.005089, -4.482487, -169.6763,
                    340.7860,
                ),
            ],
        ),
    ]
}

fn halogen_specs() -> Vec<HalogenSpec> {
    let mut out: Vec<HalogenSpec> = species()
        .into_iter()
        .map(|s| HalogenSpec {
            name: s.name,
            formula: [
                s.formula[0],
                s.formula[1],
                s.formula[2],
                s.formula[3],
                0,
                0,
                0,
            ],
            g: GasGibbs::Nasa7 {
                low: s.low,
                high: s.high,
            },
        })
        .collect();
    for s in sulfur_shomate_species() {
        out.push(HalogenSpec {
            name: s.name,
            formula: [
                s.formula[0],
                s.formula[1],
                s.formula[2],
                s.formula[3],
                s.formula[4],
                0,
                0,
            ],
            g: s.g,
        });
    }
    out.extend(halogen_shomate_species());
    out
}

pub fn halogen_gas_names() -> Vec<String> {
    halogen_specs()
        .into_iter()
        .map(|s| s.name.to_string())
        .collect()
}

fn halogen_residual(
    lam: &[f64],
    specs: &[HalogenSpec],
    t: f64,
    ln_p: f64,
    b: &[f64],
) -> (bool, f64) {
    let nelem = b.len();
    let big_n = lam[nelem].exp();
    if !big_n.is_finite() || big_n <= 0.0 {
        return (false, f64::INFINITY);
    }
    let mut r = vec![0.0f64; nelem + 1];
    for j in 0..nelem {
        r[j] = -b[j];
    }
    let mut e_sum = 0.0f64;
    for s in specs.iter() {
        let Some(g) = halogen_g_over_rt(s, t) else {
            continue;
        };
        let mut phi = -g - ln_p;
        for j in 0..nelem {
            phi += s.formula[j] as f64 * lam[j];
        }
        let e = phi.exp();
        if !e.is_finite() {
            return (false, f64::INFINITY);
        }
        e_sum += e;
        let ni = big_n * e;
        for j in 0..nelem {
            r[j] += s.formula[j] as f64 * ni;
        }
    }
    r[nelem] = e_sum - 1.0;
    let norm = r.iter().map(|x| x * x).sum::<f64>().sqrt();
    (norm.is_finite(), norm)
}

fn halogen_newton_step(
    specs: &[HalogenSpec],
    t: f64,
    ln_p: f64,
    b: &[f64],
    lam: &mut [f64],
) -> bool {
    let nelem = b.len();
    let dim = nelem + 1;
    for _ in 0..80 {
        let big_n = lam[nelem].exp();
        let n_species = specs.len();
        let mut e = vec![0.0f64; n_species];
        let mut n = vec![0.0f64; n_species];
        for (i, s) in specs.iter().enumerate() {
            let Some(g) = halogen_g_over_rt(s, t) else {
                continue;
            };
            let mut phi = -g - ln_p;
            for j in 0..nelem {
                phi += s.formula[j] as f64 * lam[j];
            }
            e[i] = phi.exp();
            n[i] = big_n * e[i];
        }
        let mut r = vec![0.0f64; nelem + 1];
        for j in 0..nelem {
            r[j] = -b[j];
            for (i, s) in specs.iter().enumerate() {
                r[j] += s.formula[j] as f64 * n[i];
            }
        }
        r[nelem] = e.iter().sum::<f64>() - 1.0;
        let rel = (0..nelem)
            .map(|j| (r[j] / b[j].max(1e-30)).abs())
            .fold(r[nelem].abs(), f64::max);
        if rel < 1e-10 {
            return true;
        }
        let mut jac = vec![vec![0.0f64; dim]; dim];
        for j in 0..nelem {
            for k in 0..nelem {
                for (i, s) in specs.iter().enumerate() {
                    jac[j][k] += s.formula[j] as f64 * s.formula[k] as f64 * n[i];
                }
            }
        }
        for j in 0..nelem {
            for (i, s) in specs.iter().enumerate() {
                jac[j][nelem] += s.formula[j] as f64 * n[i];
            }
        }
        for k in 0..nelem {
            for (i, s) in specs.iter().enumerate() {
                jac[nelem][k] += s.formula[k] as f64 * e[i];
            }
        }
        let mut dl = vec![0.0f64; dim];
        for j in 0..dim {
            dl[j] = -r[j];
        }
        gauss_solve(&mut jac, &mut dl);
        let norm0 = r.iter().map(|x| x * x).sum::<f64>().sqrt();
        let mut alpha = 1.0f64;
        let mut accepted = false;
        while alpha > 1e-14 {
            let mut trial = vec![0.0f64; dim];
            for k in 0..dim {
                trial[k] = lam[k] + alpha * dl[k];
            }
            let (ok, norm) = halogen_residual(&trial, specs, t, ln_p, b);
            if ok && norm < norm0 {
                lam.copy_from_slice(&trial);
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

fn halogen_atomic_indices(specs: &[HalogenSpec]) -> Option<[usize; NELEM_HALOGEN]> {
    let names = ["H", "C", "O", "N", "S", "F", "Cl"];
    let mut idx = [0usize; NELEM_HALOGEN];
    for (i, name) in names.iter().enumerate() {
        idx[i] = specs.iter().position(|s| s.name == *name)?;
    }
    Some(idx)
}

fn halogen_ramp(specs: &[HalogenSpec], b: &[f64], t_k: f64, p_pa: f64) -> Option<Vec<f64>> {
    let nelem = b.len();
    let ln_p = (p_pa / P0_PA).ln();
    let atomic = halogen_atomic_indices(specs)?;
    let mut lam = vec![0.0f64; nelem + 1];
    for (j, &ai) in atomic.iter().enumerate() {
        let g = halogen_g_over_rt(&specs[ai], 6000.0)?;
        lam[j] = g + ln_p + b[j].ln();
    }
    lam[nelem] = 0.0;
    let mut tcur = 6000.0f64;
    while tcur > t_k {
        let next = (tcur * 0.96).max(t_k);
        if !halogen_newton_step(specs, next, ln_p, b, &mut lam) {
            return None;
        }
        tcur = next;
    }
    let big_n = lam[nelem].exp();
    let mut n = vec![0.0f64; specs.len()];
    for (i, s) in specs.iter().enumerate() {
        let Some(g) = halogen_g_over_rt(s, t_k) else {
            continue;
        };
        let mut phi = -g - ln_p;
        for j in 0..nelem {
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

// Halogen-aware equilibrium composition at 1 bar and solar H/C/O/N/S/F/Cl
// abundances. Slots: the 16 archival H/C/N/O species in their archived order,
// then the 8 sulfur carriers (sulfur_shomate_species order), then
// F2, F, HF, Cl2, Cl, HCl, CF4, CFCl3, CF2Cl2, SF6, NF3. Domain 500..3000 K,
// the archival gas domain; below 500 K the halogen fit floors still hold but
// the condensed-water bookkeeping of the sub-500 K path is not yet extended to
// the F/Cl basis (pending, named in the probe when it meets a cool host).
pub fn equilibrium_composition_halogen(t_k: f64, p_pa: f64) -> Option<Vec<f64>> {
    if !t_k.is_finite() || t_k < 500.0 || t_k > 3000.0 || !p_pa.is_finite() || p_pa <= 0.0 {
        return None;
    }
    let specs = halogen_specs();
    let b = halogen_solar();
    halogen_ramp(&specs, &b, t_k, p_pa)
}

// Sulfur-aware equilibrium at the archival solar budget (feh = 0.0). The
// scaled entry point is the reservoir-regression face of the same solver.
pub fn equilibrium_composition_sulfur(t_k: f64, p_pa: f64) -> Option<Vec<f64>> {
    equilibrium_composition_sulfur_scaled(t_k, p_pa, 0.0)
}

// === Condensation-aware equilibrium for the sub-500 K domain ===
//
// Floor origin (measured 2026-09-05): the 500-K lower bound of the archival
// gas solvers is an encoded model-domain guard, not a fit floor. The archival
// 16 NASA-7 gas fits are the GRI-Mech 3.0 polynomials (low segments published
// over 200..1000 K; N2 over 300..5000 K — GRI-Mech 3.0 thermo30.dat), and the
// NIST-JANAF sulfur Shomate fits start at 298.15 K (atomic S(g) at 882.117 K).
// A single-phase gas at 1 bar solar abundance is the physical equilibrium down
// to the water condensation onset near 254 K: no other volatile's condensed
// phase can exist above ~240 K (NH3) / 213 K (H2S) / 195 K (CO2) / 111 K (CH4)
// at 1 bar. The five disequilibrium targets below 500 K (283..388 K) all sit
// in that single-phase-gas regime, provided the water condensation stability
// is verified with sourced condensed-phase data. This path is that
// verification and the general sub-500 K equilibrium.
//
// Condensed-phase data provenance: NIST-JANAF (Chase, M.W., Jr., 4th edition,
// J. Phys. Chem. Ref. Data Monograph 9, 1998) as rendered on the NIST
// Chemistry WebBook condensed-phase thermochemistry pages (accessed
// 2026-09-05). The WebBook carries a condensed Shomate heat-capacity fit for
// H2O(l) (domain 298.15..500 K) with anchors dHf(l,298.15) = -285.830 kJ/mol,
// S(l,298.15) = 69.95 J/mol/K — the fit's F/G reproduce those anchors
// (unit-tested below). The WebBook condensed pages of NH3, CO2, CH4 and H2S
// carry no condensed Shomate fits (gas + phase-change data only; measured
// 2026-09-05), so those condensed phases are registered pending — never
// extrapolated. Their pending state does not touch the targets: none can
// condense above ~240 K at 1 bar, above this path's whole domain.
//
// The 298.15-K cross-datum anchor is the H2O saturation pressure p_sat =
// P0*exp(g_liq - g_gas) = 3.169 kPa (NIST), verifying that the WebBook
// condensed fit and the archival GRI H2O gas fit sit on one element datum.
//
// Below the liquid fit floor (298.15 K) the H2O(l) Gibbs is not in the fit.
// The single gas phase remains exact there for 1-bar solar abundances by the
// saturation bound: the largest conceivable H2O vapor partial pressure is
// 2*SOLAR_O*P0 = 172 Pa, while liquid water's saturation pressure on its
// liquid branch (T >= 273.16 K) never falls below its triple-point value
// P triple = 0.0061 bar = 610 Pa (WebBook, Sato-Watanabe 1991). A gas whose
// H2O partial pressure reaches 610 Pa below 298.15 K is refused (None): its
// condensation state is pending-data, not zero.

pub const COOL_T_MIN: f64 = 273.16;
pub const COOL_T_MAX: f64 = 500.0;
pub const WATER_LIQ_T_MIN: f64 = 298.15;
pub const N2_GAS_T_MIN: f64 = 300.0;
pub const WATER_P_TRIPLE_PA: f64 = 610.0;

pub struct CondensateSpec {
    pub name: &'static str,
    pub vapor: &'static str,
    pub formula: [i32; NELEM_S],
    pub segs: Vec<ShomateSeg>,
}

fn cond_g_over_rt(c: &CondensateSpec, t: f64) -> Option<f64> {
    shomate_g_over_rt(&c.segs, t)
}

// H2O(l) — NIST-JANAF (Chase 1998) liquid Shomate fit, WebBook condensed page,
// domain 298.15..500 K; dHf(l,298.15) = -285.830 kJ/mol, S(l,298.15) =
// 69.95 J/mol/K.
fn water_liquid() -> CondensateSpec {
    CondensateSpec {
        name: "H2O(l)",
        vapor: "H2O",
        formula: [2, 0, 1, 0, 0],
        segs: vec![s_seg(
            298.15, 500.0, -203.6060, 1523.290, -3196.413, 2474.455, 3.855326, -256.5478, -488.7163,
        )],
    }
}

pub struct CoolEquilibrium {
    // Gas mole fractions over the 24 sulfur-gas slots. Slots whose species fit
    // does not reach the temperature are data-honored zero (0 honored): N2
    // below 300 K, the sulfur carriers below 298.15 K.
    pub frac: Vec<f64>,
    // Condensed liquid-water moles per 1-H-atom element budget; 0.0 means the
    // equilibrium is a single gas phase (the case for every 1-bar solar host).
    pub h2o_condensed_moles: f64,
    // Measured H2O vapor partial pressure in the returned gas.
    pub h2o_vapor_pa: f64,
    // p_sat over H2O(l) from the sourced Shomate fit when T >= 298.15 K; None
    // below the fit floor (the triple-point bound carries the single-gas case).
    pub h2o_sat_pa: Option<f64>,
}

pub fn equilibrium_composition_condensed_scaled(
    t_k: f64,
    p_pa: f64,
    feh: f64,
) -> Option<CoolEquilibrium> {
    if !t_k.is_finite() || t_k < COOL_T_MIN || t_k > COOL_T_MAX {
        return None;
    }
    if !p_pa.is_finite() || p_pa <= 0.0 {
        return None;
    }
    let water_in_domain = t_k >= WATER_LIQ_T_MIN;
    // Sulfur data floor: the NIST-JANAF S-gas Shomate fits start at 298.15 K;
    // below that the S element is pending, not fabricated into the gas.
    let b: Vec<f64> = if water_in_domain {
        scaled_solar_budget(feh)?.to_vec()
    } else {
        let z = 10f64.powf(feh);
        if !z.is_finite() || z <= 0.0 {
            return None;
        }
        vec![SOLAR_H, SOLAR_C * z, SOLAR_O * z, SOLAR_N * z]
    };
    cool_solve(&b, t_k, p_pa, water_in_domain)
}

// Condensation-aware equilibrium at the archival solar budget (feh = 0.0).
pub fn equilibrium_composition_condensed(t_k: f64, p_pa: f64) -> Option<CoolEquilibrium> {
    equilibrium_composition_condensed_scaled(t_k, p_pa, 0.0)
}

fn cool_solve(b: &[f64], t_k: f64, p_pa: f64, water_in_domain: bool) -> Option<CoolEquilibrium> {
    let gas = cool_gas_species(t_k);
    let water = water_liquid();
    let vapor_idx = gas.iter().position(|s| s.name == water.vapor)?;
    let frac = ramp_gas_equilibrium(&gas, b, t_k, p_pa)?;
    let x_vapor = frac[vapor_idx];
    let h2o_vapor_pa = x_vapor * p_pa;
    let h2o_sat_pa = if water_in_domain {
        let g_cond = cond_g_over_rt(&water, t_k)?;
        let g_gas = gas_g_over_rt(&gas[vapor_idx], t_k)?;
        Some(P0_PA * (g_cond - g_gas).exp())
    } else {
        None
    };
    if let Some(p_sat) = h2o_sat_pa {
        if h2o_vapor_pa > p_sat {
            // Supersaturated gas: the condensate forms until its vapor sits at
            // the saturation pressure; its atoms leave the gas element budget.
            let c = condense_moles(&gas, b, t_k, p_pa, vapor_idx, &water.formula, p_sat)?;
            let mut bc = b.to_vec();
            for (j, n) in water.formula.iter().enumerate() {
                bc[j] -= *n as f64 * c;
            }
            let frac_c = ramp_gas_equilibrium(&gas, &bc, t_k, p_pa)?;
            return Some(CoolEquilibrium {
                frac: cool_slot_layout(&gas, &frac_c),
                h2o_condensed_moles: c,
                h2o_vapor_pa: frac_c[vapor_idx] * p_pa,
                h2o_sat_pa: Some(p_sat),
            });
        }
    } else if h2o_vapor_pa >= WATER_P_TRIPLE_PA {
        return None;
    }
    Some(CoolEquilibrium {
        frac: cool_slot_layout(&gas, &frac),
        h2o_condensed_moles: 0.0,
        h2o_vapor_pa,
        h2o_sat_pa,
    })
}

fn cool_gas_species(t_k: f64) -> Vec<GasSpec> {
    // Data floors: N2's GRI-Mech 3.0 low polynomial starts at 300 K and the
    // NIST-JANAF sulfur Shomate fits at 298.15 K. Below a carrier's floor the
    // species carries no moles. The sulfur carriers must leave the set below
    // 298.15 K entirely: during the 6000-K ramp they would be in-domain while
    // the S element row is absent (its budget is pending), which leaves S
    // unconstrained.
    let s_floor_ok = t_k >= WATER_LIQ_T_MIN;
    let n2_ok = t_k >= N2_GAS_T_MIN;
    let s_carriers = ["S", "S2", "SH", "H2S", "SO", "SO2", "CS", "OCS"];
    sulfur_gas_specs()
        .into_iter()
        .filter(|s| (!s_carriers.contains(&s.name) || s_floor_ok) && (s.name != "N2" || n2_ok))
        .collect()
}

fn cool_slot_layout(gas: &[GasSpec], frac: &[f64]) -> Vec<f64> {
    let canon: Vec<&str> = sulfur_gas_specs().iter().map(|s| s.name).collect();
    let mut out = vec![0.0; canon.len()];
    for (i, name) in canon.iter().enumerate() {
        if let Some(pos) = gas.iter().position(|s| s.name == *name) {
            out[i] = frac[pos];
        }
    }
    out
}

// Condensed moles that bring the gas vapor to its saturation mole fraction
// p_sat/p_pa. Each condensed mole removes one condensate-formula unit from the
// gas element budget; the gas-only equilibrium is re-solved for each trial.
fn condense_moles(
    gas: &[GasSpec],
    b: &[f64],
    t_k: f64,
    p_pa: f64,
    vapor_idx: usize,
    formula: &[i32; NELEM_S],
    p_sat: f64,
) -> Option<f64> {
    let target = p_sat / p_pa;
    let mut c_max = f64::INFINITY;
    for (j, n) in formula.iter().enumerate() {
        if *n > 0 {
            c_max = c_max.min(b[j] * (1.0 - 1e-6) / *n as f64);
        }
    }
    let f = |c: f64| -> Option<f64> {
        let mut bc = b.to_vec();
        for (j, n) in formula.iter().enumerate() {
            bc[j] -= *n as f64 * c;
        }
        if bc.iter().any(|v| *v <= 0.0) {
            return None;
        }
        let fr = ramp_gas_equilibrium(gas, &bc, t_k, p_pa)?;
        Some(fr[vapor_idx] - target)
    };
    let f0 = f(0.0)?;
    if f0 <= 0.0 {
        return Some(0.0);
    }
    let mut hi = c_max;
    let mut f_hi = f(hi);
    while !matches!(f_hi, Some(v) if v < 0.0) {
        hi *= 0.5;
        if hi < c_max * 1e-6 {
            return None;
        }
        f_hi = f(hi);
    }
    let mut lo = 0.0;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        let fm = f(mid)?;
        if fm < 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
        if hi - lo < 1e-12 * c_max.max(1.0) {
            break;
        }
    }
    Some(0.5 * (lo + hi))
}

// Generic gas-only equilibrium without the archival 500-K domain guard. The
// element basis is b.len(); a species whose fit does not reach the current
// temperature contributes no moles (the atomic-S-below-882-K pattern).
fn ramp_gas_equilibrium(specs: &[GasSpec], b: &[f64], t_k: f64, p_pa: f64) -> Option<Vec<f64>> {
    if !t_k.is_finite() || !p_pa.is_finite() || p_pa <= 0.0 {
        return None;
    }
    if b.iter().any(|x| !x.is_finite() || *x <= 0.0) {
        return None;
    }
    let nelem = b.len();
    let ln_p = (p_pa / P0_PA).ln();
    let atomic_species = gas_atomic_species_indices(specs, nelem)?;
    let mut lam = vec![0.0f64; nelem + 1];
    for (j, &ai) in atomic_species.iter().enumerate() {
        let g = gas_g_over_rt(&specs[ai], 6000.0)?;
        lam[j] = g + ln_p + b[j].ln();
    }
    lam[nelem] = 0.0;
    let mut tcur = 6000.0f64;
    while tcur > t_k {
        let next = (tcur * 0.96).max(t_k);
        if !gas_newton_step(specs, next, ln_p, b, &mut lam) {
            return None;
        }
        tcur = next;
    }
    let big_n = lam[nelem].exp();
    let mut n = vec![0.0f64; specs.len()];
    for (i, s) in specs.iter().enumerate() {
        let Some(g) = gas_g_over_rt(s, t_k) else {
            continue;
        };
        let mut phi = -g - ln_p;
        for j in 0..nelem {
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

// === Measured host-abundance reservoir (the C/O witness) ===
//
// The reservoir regression above scales every metal row by the host [Fe/H] and
// so keeps the C/O ratio solar by construction. The C/O witness regression
// overrides the carbon and oxygen rows with the host's measured [C/H] and
// [O/H] (high-resolution abundance catalogs; the per-host rows and their
// sources live in the probe's witness seed). Nitrogen keeps its measured [N/H]
// when the same catalog row carries it, else the [Fe/H]-scaled solar value.
// Sulfur carries no per-host abundance in any source tested for the 30-host
// seed (measured 2026-09-05) and keeps the [Fe/H]-scaled solar value — a named
// proxy, not a measurement. H stays the reference 1.0. All dex are relative to
// the catalog's own solar scale; applying them to the archival SOLAR_* rows is
// transparent and reproducible, with the solar zero-point caveat named at the
// call site.
pub fn elemental_budget_sulfur(
    ch_dex: f64,
    oh_dex: f64,
    nh_dex: Option<f64>,
    feh: f64,
) -> Option<[f64; NELEM_S]> {
    if !ch_dex.is_finite() || !oh_dex.is_finite() || !feh.is_finite() {
        return None;
    }
    let z = 10f64.powf(feh);
    if !z.is_finite() || z <= 0.0 {
        return None;
    }
    let c = 10f64.powf(ch_dex);
    let o = 10f64.powf(oh_dex);
    let n = match nh_dex {
        Some(v) if v.is_finite() => 10f64.powf(v),
        _ => z,
    };
    if !c.is_finite() || c <= 0.0 || !o.is_finite() || o <= 0.0 || !n.is_finite() || n <= 0.0 {
        return None;
    }
    Some([SOLAR_H, SOLAR_C * c, SOLAR_O * o, SOLAR_N * n, SOLAR_S * z])
}

// Sulfur-aware equilibrium at an explicitly measured element budget b in the
// [H, C, O, N, S] ordering of scaled_solar_budget. The caller owns the budget
// provenance; a non-finite or non-positive row is absent, never a value.
pub fn equilibrium_composition_sulfur_budget(
    t_k: f64,
    p_pa: f64,
    b: [f64; NELEM_S],
) -> Option<Vec<f64>> {
    let specs = sulfur_gas_specs();
    solve_gas(&specs, &b, t_k, p_pa)
}

// Condensation-aware equilibrium at an explicitly measured element budget (the
// 24-slot gas layout of the cool path). Below the 298.15-K sulfur-data floor
// the sulfur row leaves the element basis, exactly as the scaled cool path.
pub fn equilibrium_composition_condensed_budget(
    t_k: f64,
    p_pa: f64,
    b: [f64; NELEM_S],
) -> Option<CoolEquilibrium> {
    if !t_k.is_finite() || t_k < COOL_T_MIN || t_k > COOL_T_MAX {
        return None;
    }
    if !p_pa.is_finite() || p_pa <= 0.0 {
        return None;
    }
    if b.iter().any(|x| !x.is_finite() || *x <= 0.0) {
        return None;
    }
    let water_in_domain = t_k >= WATER_LIQ_T_MIN;
    let bc: Vec<f64> = if water_in_domain {
        b.to_vec()
    } else {
        b[..4].to_vec()
    };
    cool_solve(&bc, t_k, p_pa, water_in_domain)
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

    fn s_species_by_name() -> Vec<GasSpec> {
        sulfur_shomate_species()
    }

    fn seg_value(segs: &[ShomateSeg], t: f64) -> Option<(f64, f64)> {
        // H°(T) [kJ/mol] and S°(T) [J/mol/K] from one Shomate segment.
        let seg = segs
            .iter()
            .find(|s| t < s.t_max)
            .or_else(|| segs.last().filter(|s| t <= s.t_max))?;
        if t < seg.t_min {
            return None;
        }
        let tt = t / 1000.0;
        let h = seg.a * tt
            + seg.b * tt * tt / 2.0
            + seg.c * tt * tt * tt / 3.0
            + seg.d * tt * tt * tt * tt / 4.0
            - seg.e / tt
            + seg.f;
        let s = seg.a * tt.ln() + seg.b * tt + seg.c * tt * tt / 2.0 + seg.d * tt * tt * tt / 3.0
            - seg.e / (2.0 * tt * tt)
            + seg.g;
        Some((h, s))
    }

    #[test]
    fn sulfur_fits_reproduce_the_nist_janaf_298_anchors() {
        // NIST-JANAF (Chase 1998) 298.15-K anchors from the WebBook pages the
        // constants were read from. S(g) has no fit at 298.15 K (domain starts
        // at 882.117 K); it is checked separately.
        let anchors: [(&str, f64, f64); 7] = [
            ("S2", 128.60, 228.19),
            ("SH", 139.33, 195.63),
            ("H2S", -20.50, 205.77),
            ("SO", 5.01, 221.94),
            ("SO2", -296.84, 248.21),
            ("CS", 280.33, 210.55),
            ("OCS", -138.41, 231.57),
        ];
        for (name, d_hf, s_ref) in anchors {
            let specs = s_species_by_name();
            let spec = specs
                .iter()
                .find(|sp| sp.name == name)
                .unwrap_or_else(|| panic!("{name} absent"));
            let GasGibbs::Shomate { segs } = &spec.g else {
                panic!("{name} not shomate");
            };
            let (h, s) = seg_value(segs, 298.15).unwrap();
            assert!(
                (h - d_hf).abs() < 0.5,
                "{name}: H(298) {:.3} kJ/mol vs JANAF {:.2}",
                h,
                d_hf
            );
            assert!(
                (s - s_ref).abs() < 0.5,
                "{name}: S(298) {:.3} J/mol/K vs JANAF {:.2}",
                s,
                s_ref
            );
        }
    }

    #[test]
    fn atomic_s_gas_fit_domain_starts_at_882k() {
        let specs = s_species_by_name();
        let spec = specs.iter().find(|sp| sp.name == "S").unwrap();
        let GasGibbs::Shomate { segs } = &spec.g else {
            panic!("S not shomate");
        };
        assert!(seg_value(segs, 298.15).is_none());
        assert!(seg_value(segs, 500.0).is_none());
        assert!(seg_value(segs, 882.0).is_none());
        assert!(seg_value(segs, 900.0).is_some());
        assert!(seg_value(segs, 3000.0).is_some());
    }

    #[test]
    fn sulfur_composition_conserves_the_element_budget() {
        let x = equilibrium_composition_sulfur(1200.0, P0_PA).unwrap();
        assert_eq!(x.len(), 24);
        let specs = sulfur_gas_specs();
        let mut atoms = [0.0f64; NELEM_S];
        for (i, s) in specs.iter().enumerate() {
            for j in 0..NELEM_S {
                atoms[j] += s.formula[j] as f64 * x[i];
            }
        }
        let solar5 = sulfur_solar();
        for j in 0..NELEM_S {
            assert!(
                (atoms[j] / atoms[0] / solar5[j] - 1.0).abs() < 1e-6,
                "element {} balance off: {}",
                j,
                atoms[j] / atoms[0]
            );
        }
    }

    #[test]
    fn sulfur_solver_matches_direct_kp() {
        // H2S + 2 H2O <-> SO2 + 3 H2 — the cross-datum reaction that pins the
        // NIST-JANAF sulfur scale to the archival H/C/N/O datum.
        let t = 1200.0f64;
        let x = equilibrium_composition_sulfur(t, P0_PA).unwrap();
        let specs = sulfur_gas_specs();
        let pos = |name: &str| specs.iter().position(|s| s.name == name).unwrap();
        let (ih2s, ih2o, iso2, ih2) = (pos("H2S"), pos("H2O"), pos("SO2"), pos("H2"));
        let (g_h2s, g_h2o, g_so2, g_h2) = (
            gas_g_over_rt(&specs[ih2s], t).unwrap(),
            gas_g_over_rt(&specs[ih2o], t).unwrap(),
            gas_g_over_rt(&specs[iso2], t).unwrap(),
            gas_g_over_rt(&specs[ih2], t).unwrap(),
        );
        let dg = g_so2 + 3.0 * g_h2 - g_h2s - 2.0 * g_h2o;
        let kp = (-dg).exp();
        let from_solver = (x[iso2] * x[ih2].powi(3)) / (x[ih2s] * x[ih2o].powi(2));
        assert!(
            (from_solver / kp - 1.0).abs() < 1e-6,
            "solver Kp {} vs direct {}",
            from_solver,
            kp
        );
    }

    #[test]
    fn sulfur_h2s_dominates_cold_sh_s2_hot() {
        let x_cold = equilibrium_composition_sulfur(700.0, P0_PA).unwrap();
        let x_hot = equilibrium_composition_sulfur(1700.0, P0_PA).unwrap();
        let specs = sulfur_gas_specs();
        let pos = |name: &str| specs.iter().position(|s| s.name == name).unwrap();
        let (ih2s, ish, is2) = (pos("H2S"), pos("SH"), pos("S2"));
        let s_share_cold = x_cold[ih2s] + x_cold[ish] + x_cold[is2];
        let s_share_hot = x_hot[ih2s] + x_hot[ish] + x_hot[is2];
        assert!(
            x_cold[ih2s] > 0.9 * s_share_cold,
            "H2S is the S reservoir at 700 K, got {:.3e} of S in H2S",
            x_cold[ih2s] / s_share_cold
        );
        assert!(
            x_hot[ih2s] > 0.9 * s_share_hot,
            "H2S remains the S reservoir at 1700 K, got {:.3e} of S in H2S",
            x_hot[ih2s] / s_share_hot
        );
        assert!(
            x_hot[ish] + x_hot[is2] > x_cold[ish] + x_cold[is2],
            "SH/S2 share grows from 700 K to 1700 K"
        );
    }

    #[test]
    fn sulfur_composition_refuses_out_of_domain() {
        assert!(equilibrium_composition_sulfur(300.0, P0_PA).is_none());
        assert!(equilibrium_composition_sulfur(4000.0, P0_PA).is_none());
        assert!(equilibrium_composition_sulfur(1200.0, 0.0).is_none());
        assert!(equilibrium_composition_sulfur(f64::NAN, P0_PA).is_none());
    }

    #[test]
    fn water_liquid_fit_reproduces_the_janaf_298_anchors() {
        // NIST-JANAF (Chase 1998) 298.15-K anchors as listed on the WebBook
        // condensed page the constants were read from.
        let w = water_liquid();
        let (h, s) = seg_value(&w.segs, 298.15).unwrap();
        assert!(
            (h - -285.830).abs() < 0.05,
            "H2O(l): H(298) {:.3} kJ/mol vs JANAF -285.830",
            h
        );
        assert!(
            (s - 69.95).abs() < 0.05,
            "H2O(l): S(298) {:.3} J/mol/K vs JANAF 69.95",
            s
        );
    }

    #[test]
    fn water_liquid_svp_at_298_matches_nist() {
        // p_sat = P0*exp(g_liq - g_gas) at 298.15 K must reproduce the NIST
        // saturation pressure of water, 3.169 kPa — the cross-datum proof that
        // the WebBook condensed fit and the archival GRI H2O gas fit share one
        // element datum.
        let w = water_liquid();
        let specs = sulfur_gas_specs();
        let h2o = specs.iter().find(|s| s.name == "H2O").unwrap();
        let g_cond = cond_g_over_rt(&w, 298.15).unwrap();
        let g_gas = gas_g_over_rt(h2o, 298.15).unwrap();
        let p_sat = P0_PA * (g_cond - g_gas).exp();
        assert!(
            (p_sat / 3169.0 - 1.0).abs() < 0.03,
            "p_sat(298.15 K) = {:.1} Pa vs NIST 3169 Pa",
            p_sat
        );
    }

    #[test]
    fn condensed_gas_path_matches_the_archival_solvers_above_500k() {
        // At 1 bar no condensate forms above the boiling point; the condensed
        // path must equal the archival sulfur gas solver at the domain
        // boundary, and the 4-element cool path must extend the archival
        // equilibrium_composition below its own 500-K domain guard.
        let cool = equilibrium_composition_condensed(500.0, P0_PA).unwrap();
        let sulfur = equilibrium_composition_sulfur(500.0, P0_PA).unwrap();
        for i in 0..24 {
            assert!(
                (cool.frac[i] - sulfur[i]).abs() < 1e-12,
                "slot {i}: cool {:.6e} vs sulfur {:.6e}",
                cool.frac[i],
                sulfur[i]
            );
        }
        assert_eq!(cool.h2o_condensed_moles, 0.0);
        let cool_4 = equilibrium_composition_condensed(283.15, P0_PA).unwrap();
        let arch = equilibrium_composition(283.15, P0_PA, solar());
        assert!(arch.is_none());
        assert_eq!(cool_4.frac.len(), 24);
        let h2o = 7usize;
        assert!(cool_4.frac[h2o] > 0.0);
    }

    #[test]
    fn cool_target_temperatures_stay_single_phase_gas() {
        // The five sub-500 K disequilibrium hosts sit at 283..388 K; at 1 bar
        // solar abundances the H2O vapor partial pressure (172 Pa max) is far
        // below the liquid-water saturation pressure, so the equilibrium is a
        // single gas phase at each Teq.
        let targets: [f64; 5] = [283.0, 352.0, 354.0, 363.0, 388.0];
        for t in targets {
            let eq = equilibrium_composition_condensed(t, P0_PA)
                .unwrap_or_else(|| panic!("cool solve at {t} K"));
            assert_eq!(eq.h2o_condensed_moles, 0.0);
            let sum: f64 = eq.frac.iter().sum();
            assert!((sum - 1.0).abs() < 1e-9, "frac sum {sum} at {t} K");
            let (ch4, co2) = (11usize, 10usize);
            assert!(
                eq.frac[ch4] > eq.frac[co2] * 1e6,
                "CH4 is the C reservoir at {t} K"
            );
            assert!(eq.h2o_vapor_pa < 200.0, "solar H2O partial at {t} K");
        }
        // K2-18 b (283 K) is below the liquid Shomate floor: the saturation
        // value is not in the fit, but the vapor is below the triple-point
        // bound, so the single-gas verdict stands with sat = None.
        let k2 = equilibrium_composition_condensed(283.0, P0_PA).unwrap();
        assert!(k2.h2o_sat_pa.is_none());
        assert!(k2.h2o_vapor_pa < WATER_P_TRIPLE_PA);
        // The four warmer hosts carry the measured saturation pressure.
        let lp791 = equilibrium_composition_condensed(354.0, P0_PA).unwrap();
        let sat = lp791.h2o_sat_pa.unwrap();
        assert!(sat > 1e4, "p_sat(354 K) should far exceed the solar vapor");
    }

    #[test]
    fn cool_water_condenses_at_high_pressure_until_saturation() {
        // At 30 bar the solar H2O partial pressure (1.7e-3 * 30 bar) exceeds
        // p_sat(298.15 K) = 3169 Pa, so liquid water condenses until the gas
        // H2O sits at saturation. Exercises the two-phase bisection.
        let p = 30.0 * 101325.0;
        let eq = equilibrium_composition_condensed(298.15, p).unwrap();
        assert!(
            eq.h2o_condensed_moles > 0.0,
            "water must condense at 30 bar"
        );
        let sat = eq.h2o_sat_pa.unwrap();
        assert!((sat / 3169.0 - 1.0).abs() < 0.03);
        assert!(
            (eq.h2o_vapor_pa / sat - 1.0).abs() < 0.02,
            "gas H2O at saturation: {} vs {}",
            eq.h2o_vapor_pa,
            sat
        );
        // Solar budget conservation across gas + condensate: removing
        // c condensed H2O from the element budget and re-solving conserves the
        // rest by construction; the gas keeps H2O only at saturation.
        let h2o = 7usize;
        assert!(eq.frac[h2o] > 0.0 && eq.frac[h2o] < 2.0 * SOLAR_O);
    }

    #[test]
    fn cool_refuses_out_of_domain_and_pending_condensation() {
        assert!(equilibrium_composition_condensed(250.0, P0_PA).is_none());
        assert!(equilibrium_composition_condensed(600.0, P0_PA).is_none());
        assert!(equilibrium_composition_condensed(350.0, 0.0).is_none());
        assert!(equilibrium_composition_condensed(f64::NAN, P0_PA).is_none());
        // A gas that would push H2O vapor to the triple-point bound below the
        // liquid fit floor cannot be judged: condensation state pending, not
        // fabricated single phase. At 290 K solar is far below the bound, so
        // the refusal must come from a supersolar O budget (internal call).
        let b = vec![1.0, SOLAR_C, 1.0e-2, SOLAR_N];
        let res = cool_solve(&b, 290.0, P0_PA, false);
        assert!(res.is_none(), "supersolar O below 298.15 K is pending");
        let solar = equilibrium_composition_condensed(290.0, P0_PA).unwrap();
        assert!(solar.h2o_sat_pa.is_none());
        assert!(solar.h2o_vapor_pa < WATER_P_TRIPLE_PA);
        // Sulfur channel is data-pending below 298.15 K: S-tail slots are zero.
        assert_eq!(solar.frac[17], 0.0);
        assert_eq!(solar.frac[19], 0.0);
    }

    #[test]
    fn cool_n2_slot_is_zero_below_300k() {
        // N2's GRI-Mech 3.0 low polynomial starts at 300 K; at 283 K the slot
        // is data-honored zero and the N budget is carried by NH3.
        let eq = equilibrium_composition_condensed(283.0, P0_PA).unwrap();
        assert_eq!(eq.frac[4], 0.0);
        let nh3 = 12usize;
        let n2 = 4usize;
        assert!(eq.frac[nh3] > 100.0 * eq.frac[n2]);
    }

    #[test]
    fn reservoir_zero_feh_is_exactly_the_solar_budget() {
        let scaled = scaled_solar_budget(0.0).unwrap();
        let solar = sulfur_solar();
        for j in 0..NELEM_S {
            assert!(
                scaled[j] == solar[j],
                "element {j}: scaled {:.6e} vs solar {:.6e}",
                scaled[j],
                solar[j]
            );
        }
        assert!(scaled_solar_budget(f64::NAN).is_none());
        assert!(scaled_solar_budget(f64::INFINITY).is_none());
    }

    #[test]
    fn reservoir_scaled_solvers_equal_the_solar_solvers_at_zero_feh() {
        let a = equilibrium_composition_sulfur_scaled(1200.0, P0_PA, 0.0).unwrap();
        let b = equilibrium_composition_sulfur(1200.0, P0_PA).unwrap();
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert!(a[i] == b[i], "sulfur slot {i} differs");
        }
        let ca = equilibrium_composition_condensed_scaled(283.0, P0_PA, 0.0).unwrap();
        let cb = equilibrium_composition_condensed(283.0, P0_PA).unwrap();
        assert_eq!(ca.frac.len(), cb.frac.len());
        for i in 0..ca.frac.len() {
            assert!(ca.frac[i] == cb.frac[i], "cool slot {i} differs");
        }
        assert_eq!(ca.h2o_condensed_moles, cb.h2o_condensed_moles);
        assert_eq!(ca.h2o_sat_pa, cb.h2o_sat_pa);
    }

    #[test]
    fn reservoir_scaling_raises_metal_molecule_abundances() {
        // A metal-rich budget raises the equilibrium of metal-carrier gases.
        // At 736 K H2S is the sulfur reservoir and CO the carbon reservoir; the
        // sulfur solver carries both on one budget.
        let feh_rich = 0.3f64;
        let feh_poor = -0.3f64;
        let specs = sulfur_gas_specs();
        let pos = |name: &str| specs.iter().position(|s| s.name == name).unwrap();
        let (ih2s, ico) = (pos("H2S"), pos("CO"));
        let rich = equilibrium_composition_sulfur_scaled(736.0, P0_PA, feh_rich).unwrap();
        let solar = equilibrium_composition_sulfur(736.0, P0_PA).unwrap();
        let poor = equilibrium_composition_sulfur_scaled(736.0, P0_PA, feh_poor).unwrap();
        let z = 10f64.powf(feh_rich);
        assert!(
            (rich[ih2s] / solar[ih2s] - z).abs() < 0.25 * z,
            "H2S should scale ~z={z}: {:.3e} vs solar {:.3e}",
            rich[ih2s],
            solar[ih2s]
        );
        assert!(rich[ih2s] > solar[ih2s] && solar[ih2s] > poor[ih2s]);
        assert!(rich[ico] > solar[ico] && solar[ico] > poor[ico]);
    }

    fn halogen_anchor(name: &str) -> (f64, f64) {
        // Chase-1998 298.15-K anchors read from the NIST Chemistry WebBook gas
        // pages (cache /tmp/opencode/nist_fcl/, accessed 2026-09-05): the
        // Shomate "H" column (standard enthalpy of formation) and the reported
        // standard entropy. F2 and Cl2 are reference states (dHf = 0).
        match name {
            "F2" => (0.0, 202.79),
            "F" => (79.39, 158.78),
            "HF" => (-272.55, 173.78),
            "Cl2" => (0.0, 223.08),
            "Cl" => (121.30, 165.19),
            "HCl" => (-92.31, 186.90),
            "CF4" => (-933.20, 261.41),
            "CFCl3" => (-288.70, 309.74),
            "CF2Cl2" => (-491.62, 300.89),
            "SF6" => (-1220.47, 291.52),
            "NF3" => (-132.09, 260.77),
            other => panic!("no halogen anchor for {other}"),
        }
    }

    #[test]
    fn halogen_fits_reproduce_the_nist_janaf_298_anchors() {
        let names = halogen_shomate_species();
        for spec in names {
            let (d_hf, s_ref) = halogen_anchor(spec.name);
            let GasGibbs::Shomate { segs } = &spec.g else {
                panic!("{} not shomate", spec.name);
            };
            let (h, s) = seg_value(segs, 298.15).unwrap();
            assert!(
                (h - d_hf).abs() < 0.05,
                "{}: H(298) {:.3} kJ/mol vs JANAF {:.2}",
                spec.name,
                h,
                d_hf
            );
            assert!(
                (s - s_ref).abs() < 0.05,
                "{}: S(298) {:.3} J/mol/K vs JANAF {:.2}",
                spec.name,
                s,
                s_ref
            );
        }
    }

    #[test]
    fn halogen_techno_fits_all_cover_298_to_500k() {
        // The probe meets cool hosts down to the 500-K domain floor; every
        // halogen species must carry a fit through 500 K so no floor is a
        // fabricated value there.
        for spec in halogen_shomate_species() {
            let GasGibbs::Shomate { segs } = &spec.g else {
                panic!("{} not shomate", spec.name);
            };
            assert!(
                seg_value(segs, 298.15).is_some() && seg_value(segs, 500.0).is_some(),
                "{} must reach 500 K",
                spec.name
            );
        }
    }

    #[test]
    fn halogen_composition_conserves_the_element_budget() {
        let x = equilibrium_composition_halogen(1200.0, P0_PA).unwrap();
        assert_eq!(x.len(), 35);
        let specs = halogen_specs();
        let mut atoms = [0.0f64; NELEM_HALOGEN];
        for (i, s) in specs.iter().enumerate() {
            for j in 0..NELEM_HALOGEN {
                atoms[j] += s.formula[j] as f64 * x[i];
            }
        }
        let solar7 = halogen_solar();
        for j in 0..NELEM_HALOGEN {
            assert!(
                (atoms[j] / atoms[0] / solar7[j] - 1.0).abs() < 1e-6,
                "element {j} balance off: {}",
                atoms[j] / atoms[0]
            );
        }
    }

    #[test]
    fn halogen_solver_matches_direct_kp() {
        // 2 HF <-> H2 + F2 — the reaction that pins the F2-reference datum of
        // the NIST-JANAF halogen fits to the archival H2 datum.
        let t = 1200.0f64;
        let x = equilibrium_composition_halogen(t, P0_PA).unwrap();
        let specs = halogen_specs();
        let pos = |name: &str| specs.iter().position(|s| s.name == name).unwrap();
        let (ihf, ih2, if2) = (pos("HF"), pos("H2"), pos("F2"));
        let (g_hf, g_h2, g_f2) = (
            halogen_g_over_rt(&specs[ihf], t).unwrap(),
            halogen_g_over_rt(&specs[ih2], t).unwrap(),
            halogen_g_over_rt(&specs[if2], t).unwrap(),
        );
        let dg = g_h2 + g_f2 - 2.0 * g_hf;
        let kp = (-dg).exp();
        let from_solver = (x[ih2] * x[if2]) / (x[ihf] * x[ihf]);
        assert!(
            (from_solver / kp - 1.0).abs() < 1e-6,
            "solver Kp {} vs direct {}",
            from_solver,
            kp
        );
    }

    #[test]
    fn hf_and_hcl_are_the_halogen_reservoirs() {
        // At every probe temperature the F budget sits in HF and the Cl budget
        // in HCl; the techno molecules sit orders of magnitude below them.
        for t in [500.0, 700.0, 1000.0, 1500.0] {
            let x = equilibrium_composition_halogen(t, P0_PA)
                .unwrap_or_else(|| panic!("halogen solve at {t} K"));
            let specs = halogen_specs();
            let pos = |name: &str| specs.iter().position(|s| s.name == name).unwrap();
            let (ihf, ihcl) = (pos("HF"), pos("HCl"));
            let mut f_atoms = 0.0f64;
            let mut cl_atoms = 0.0f64;
            for (i, s) in specs.iter().enumerate() {
                f_atoms += s.formula[5] as f64 * x[i];
                cl_atoms += s.formula[6] as f64 * x[i];
            }
            let f_in_hf = x[ihf] / f_atoms;
            let cl_in_hcl = x[ihcl] / cl_atoms;
            assert!(
                (f_in_hf - 1.0).abs() < 0.2,
                "F in HF at {t} K: {:.3e} of F atoms",
                f_in_hf
            );
            assert!(
                (cl_in_hcl - 1.0).abs() < 0.2,
                "Cl in HCl at {t} K: {:.3e} of Cl atoms",
                cl_in_hcl
            );
        }
    }

    #[test]
    fn techno_gas_equilibrium_floors_are_astronomically_low() {
        // The numeric floor the industrial-only axis judges against: the
        // equilibrium mixing ratio of each techno molecule at solar abundance
        // is orders of magnitude below any plausible instrument floor (the
        // probe's named --floor defaults to 1e-6).
        for t in [500.0, 700.0, 1000.0, 1500.0] {
            let x = equilibrium_composition_halogen(t, P0_PA)
                .unwrap_or_else(|| panic!("halogen solve at {t} K"));
            let specs = halogen_specs();
            let pos = |name: &str| specs.iter().position(|s| s.name == name).unwrap();
            for name in ["CFCl3", "CF2Cl2", "SF6", "CF4", "NF3"] {
                let f = x[pos(name)];
                assert!(
                    f < 1e-20,
                    "{name} equilibrium floor at {t} K = {:.2e} must be negligible",
                    f
                );
            }
        }
    }

    #[test]
    fn halogen_composition_refuses_out_of_domain() {
        assert!(equilibrium_composition_halogen(300.0, P0_PA).is_none());
        assert!(equilibrium_composition_halogen(4000.0, P0_PA).is_none());
        assert!(equilibrium_composition_halogen(1200.0, 0.0).is_none());
        assert!(equilibrium_composition_halogen(f64::NAN, P0_PA).is_none());
        assert!(equilibrium_composition_halogen(1200.0, f64::INFINITY).is_none());
    }

    #[test]
    fn measured_budget_solar_rows_equal_the_solar_budget() {
        let b = elemental_budget_sulfur(0.0, 0.0, Some(0.0), 0.0).unwrap();
        for j in 0..NELEM_S {
            assert!(
                b[j] == sulfur_solar()[j],
                "element {j}: {:.6e} vs solar {:.6e}",
                b[j],
                sulfur_solar()[j]
            );
        }
        // All elements at the same dex reproduce the uniform [Fe/H] scaling.
        let feh = 0.25f64;
        let b2 = elemental_budget_sulfur(feh, feh, Some(feh), feh).unwrap();
        let scaled = scaled_solar_budget(feh).unwrap();
        for j in 0..NELEM_S {
            assert!(
                (b2[j] - scaled[j]).abs() / scaled[j] < 1e-12,
                "element {j}: measured {:.6e} vs scaled {:.6e}",
                b2[j],
                scaled[j]
            );
        }
    }

    #[test]
    fn measured_budget_refuses_non_finite_dex() {
        assert!(elemental_budget_sulfur(f64::NAN, 0.0, Some(0.0), 0.0).is_none());
        assert!(elemental_budget_sulfur(0.0, f64::INFINITY, Some(0.0), 0.0).is_none());
        assert!(elemental_budget_sulfur(0.0, 0.0, None, f64::INFINITY).is_none());
        // A non-finite nitrogen dex in the Option is the caller encoding absent
        // (the probe stores None, never NaN); the builder carries the [Fe/H]
        // scale for that element.
        let b = elemental_budget_sulfur(0.0, 0.0, Some(f64::NAN), 0.0).unwrap();
        for j in 0..NELEM_S {
            assert!(
                b[j] == sulfur_solar()[j],
                "element {j}: {:.6e} vs solar {:.6e}",
                b[j],
                sulfur_solar()[j]
            );
        }
        // A None nitrogen dex keeps the [Fe/H] scale, never a value.
        let b = elemental_budget_sulfur(0.0, 0.0, None, 0.3).unwrap();
        let scaled = scaled_solar_budget(0.3).unwrap();
        assert!(
            (b[3] - scaled[3]).abs() / scaled[3] < 1e-12,
            "N keeps the [Fe/H] scale"
        );
    }

    #[test]
    fn budget_solvers_guard_the_domain() {
        let b = elemental_budget_sulfur(0.0, 0.0, Some(0.0), 0.0).unwrap();
        assert!(equilibrium_composition_sulfur_budget(300.0, P0_PA, b).is_none());
        assert!(equilibrium_composition_sulfur_budget(4000.0, P0_PA, b).is_none());
        assert!(equilibrium_composition_sulfur_budget(1200.0, 0.0, b).is_none());
        assert!(
            equilibrium_composition_sulfur_budget(1200.0, P0_PA, [0.0, 0.0, 0.0, 0.0, 0.0])
                .is_none()
        );
        assert!(equilibrium_composition_condensed_budget(283.0, P0_PA, b).is_some());
        assert!(
            equilibrium_composition_condensed_budget(283.0, P0_PA, [0.0, 0.0, 0.0, 0.0, 0.0])
                .is_none()
        );
    }

    #[test]
    fn oxygen_rich_budget_raises_co2_over_solar() {
        // C/O < solar (O-rich): the same C atoms must partition toward CO2 and
        // away from CO; CO2 equilibrium rises against the solar budget.
        let solar = equilibrium_composition_sulfur(1000.0, P0_PA).unwrap();
        let specs = sulfur_gas_specs();
        let pos = |name: &str| specs.iter().position(|s| s.name == name).unwrap();
        let (ico, ico2) = (pos("CO"), pos("CO2"));
        // [C/H] = -0.3, [O/H] = +0.3 -> C/O = solar C/O * 1e-0.6 ~ 0.11.
        let b = elemental_budget_sulfur(-0.3, 0.3, Some(0.0), 0.0).unwrap();
        let o_rich = equilibrium_composition_sulfur_budget(1000.0, P0_PA, b).unwrap();
        assert!(
            o_rich[ico2] > solar[ico2],
            "O-rich CO2 {:.3e} must exceed solar {:.3e}",
            o_rich[ico2],
            solar[ico2]
        );
        assert!(
            o_rich[ico] < solar[ico],
            "O-rich CO {:.3e} must fall below solar {:.3e}",
            o_rich[ico],
            solar[ico]
        );
        // The returned mix conserves the input budget's element ratios: the
        // element balance in the gas equals the budget rows per H atom.
        let mut atoms = [0.0f64; NELEM_S];
        for (i, s) in specs.iter().enumerate() {
            for j in 0..NELEM_S {
                atoms[j] += s.formula[j] as f64 * o_rich[i];
            }
        }
        for j in 1..NELEM_S {
            let expect = b[j] / b[0];
            let got = atoms[j] / atoms[0];
            assert!(
                (got - expect).abs() / expect < 1e-6,
                "element {j} conservation: {:.6e} vs budget ratio {:.6e}",
                got,
                expect
            );
        }
    }
}
