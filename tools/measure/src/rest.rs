use std::f64::consts::PI;

pub const SIGMA_BRAIN: f64 = 0.33;
pub const SIGMA_SKULL: f64 = 0.0042;
pub const SIGMA_SCALP: f64 = 0.33;
pub const BRAIN_RADIUS_RATIO: f64 = 0.87;
pub const SKULL_RADIUS_RATIO: f64 = 0.93;
pub const SPHERE_ORDER: usize = 60;
pub const REST_REGULARIZATION_K: u32 = 32;
const LAMBDA_MAX_ITERATIONS: usize = 64;

pub struct Sphere {
    pub center: [f64; 3],
    pub radius: f64,
}

pub struct ThreeShell {
    pub brain_ratio: f64,
    pub skull_ratio: f64,
    pub sigma_brain: f64,
    pub sigma_skull: f64,
    pub sigma_scalp: f64,
    pub order: usize,
}

impl ThreeShell {
    pub fn standard() -> ThreeShell {
        ThreeShell {
            brain_ratio: BRAIN_RADIUS_RATIO,
            skull_ratio: SKULL_RADIUS_RATIO,
            sigma_brain: SIGMA_BRAIN,
            sigma_skull: SIGMA_SKULL,
            sigma_scalp: SIGMA_SCALP,
            order: SPHERE_ORDER,
        }
    }

    pub fn radial_forward(
        &self,
        sphere: &Sphere,
        electrode: &[f64; 3],
        source: &[f64; 3],
        moment: f64,
    ) -> Option<f64> {
        let e = [
            electrode[0] - sphere.center[0],
            electrode[1] - sphere.center[1],
            electrode[2] - sphere.center[2],
        ];
        let q = [
            source[0] - sphere.center[0],
            source[1] - sphere.center[1],
            source[2] - sphere.center[2],
        ];
        let re = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
        let rq = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
        if re <= 0.0 || rq <= 0.0 || !moment.is_finite() {
            return None;
        }
        let cos_gamma = (e[0] * q[0] + e[1] * q[1] + e[2] * q[2]) / (re * rq);
        let coeffs = self.radial_coefficients(sphere.radius, rq)?;
        let leg = legendre_cos(cos_gamma, self.order);
        let mut acc = 0.0;
        for n in 1..=self.order {
            acc += coeffs[n - 1] * leg[n];
        }
        Some(moment * acc / (4.0 * PI * self.sigma_brain))
    }

    pub fn lead_field_radial(
        &self,
        sphere: &Sphere,
        electrodes: &[[f64; 3]],
        sources: &[[f64; 3]],
    ) -> Option<Vec<Vec<f64>>> {
        let nch = electrodes.len();
        let nsrc = sources.len();
        if nch == 0 || nsrc == 0 {
            return None;
        }
        let mut out = vec![vec![0.0; nsrc]; nch];
        for (j, source) in sources.iter().enumerate() {
            for (i, electrode) in electrodes.iter().enumerate() {
                out[i][j] = self.radial_forward(sphere, electrode, source, 1.0)?;
            }
        }
        Some(out)
    }

    fn radial_coefficients(&self, scalp_radius: f64, dipole_radius: f64) -> Option<Vec<f64>> {
        let r1 = scalp_radius * self.brain_ratio;
        let r2 = scalp_radius * self.skull_ratio;
        let r3 = scalp_radius;
        if r1 <= 0.0 || r2 <= r1 || r3 <= r2 {
            return None;
        }
        if dipole_radius <= 0.0 || dipole_radius >= r1 {
            return None;
        }
        let t = r2 / r1;
        let u = r3 / r2;
        let s1 = self.sigma_brain;
        let s2 = self.sigma_skull;
        let s3 = self.sigma_scalp;
        let mut out = Vec::with_capacity(self.order);
        for n in 1..=self.order {
            let nf = n as f64;
            let src = nf * (dipole_radius / r1).powi(n as i32 - 1) / (r1 * r1);
            let tn = t.powi(n as i32);
            let tn1 = t.powi(-(n as i32 + 1));
            let un = u.powi(-(n as i32));
            let un1 = u.powi(n as i32 + 1);
            let m = vec![
                vec![nf * (s1 - s2), nf * s1 + (nf + 1.0) * s2, 0.0],
                vec![tn, tn1, -(un + (nf / (nf + 1.0)) * un1)],
                vec![s2 * nf * tn, -s2 * (nf + 1.0) * tn1, -s3 * nf * (un - un1)],
            ];
            let rhs = vec![(2.0 * nf + 1.0) * s1 * src, 0.0, 0.0];
            let x = gauss_solve(m, rhs)?;
            let wd = x[2];
            out.push(wd * (2.0 * nf + 1.0) / (nf + 1.0));
        }
        Some(out)
    }
}

pub fn fit_sphere(points: &[[f64; 3]]) -> Option<Sphere> {
    if points.len() < 4 {
        return None;
    }
    let mut ata = vec![vec![0.0; 4]; 4];
    let mut atb = vec![0.0; 4];
    for p in points {
        let x = p[0];
        let y = p[1];
        let z = p[2];
        if !(x.is_finite() && y.is_finite() && z.is_finite()) {
            return None;
        }
        let rhs = x * x + y * y + z * z;
        let row = [2.0 * x, 2.0 * y, 2.0 * z, -1.0];
        for i in 0..4 {
            for j in 0..4 {
                ata[i][j] += row[i] * row[j];
            }
            atb[i] += row[i] * rhs;
        }
    }
    let sol = gauss_solve(ata, atb)?;
    let cx = sol[0];
    let cy = sol[1];
    let cz = sol[2];
    let d = sol[3];
    let r2 = cx * cx + cy * cy + cz * cz - d;
    if !r2.is_finite() || r2 <= 0.0 {
        return None;
    }
    let radius = r2.sqrt();
    if radius <= 0.0 || radius.is_nan() {
        return None;
    }
    Some(Sphere {
        center: [cx, cy, cz],
        radius,
    })
}

pub struct RestTransform {
    pub reference: String,
    pub regularization: f64,
    pub operator: Vec<Vec<f64>>,
}

fn gram_lambda_max(g: &[Vec<f64>]) -> Option<f64> {
    let n = g.len();
    if n == 0 || g.iter().any(|row| row.len() != n) {
        return None;
    }
    let mut v = vec![1.0 / (n as f64).sqrt(); n];
    let mut lambda = 0.0f64;
    for _ in 0..LAMBDA_MAX_ITERATIONS {
        let mut w = vec![0.0f64; n];
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += g[i][j] * v[j];
            }
            w[i] = s;
        }
        let norm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm <= 0.0 || !norm.is_finite() {
            return None;
        }
        for x in w.iter_mut() {
            *x /= norm;
        }
        let mut rayleigh = 0.0;
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += g[i][j] * w[j];
            }
            rayleigh += w[i] * s;
        }
        lambda = rayleigh;
        v = w;
    }
    if lambda.is_finite() && lambda > 0.0 {
        Some(lambda)
    } else {
        None
    }
}

pub fn rest_transform(
    lead_field: &[Vec<f64>],
    reference: String,
    reference_row: usize,
) -> Option<RestTransform> {
    let nch = lead_field.len();
    if nch == 0 || reference_row >= nch {
        return None;
    }
    let nsrc = lead_field[0].len();
    if nsrc == 0 {
        return None;
    }
    let lref = lead_field[reference_row].clone();
    let mut lr = lead_field.to_vec();
    for row in lr.iter_mut() {
        for (j, v) in row.iter_mut().enumerate() {
            *v -= lref[j];
        }
    }
    let mut m = vec![vec![0.0; nsrc]; nsrc];
    for a in 0..nsrc {
        for b in 0..nsrc {
            let mut s = 0.0;
            for ch in 0..nch {
                s += lr[ch][a] * lr[ch][b];
            }
            m[a][b] = s;
        }
    }
    let regularization = gram_lambda_max(&m)? * (0.5f64).powi(REST_REGULARIZATION_K as i32);
    for a in 0..nsrc {
        m[a][a] += regularization;
    }
    let l = cholesky(m)?;
    let mut g = vec![vec![0.0; nch]; nsrc];
    for ch in 0..nch {
        let b: Vec<f64> = (0..nsrc).map(|a| lr[ch][a]).collect();
        let x = cholesky_solve(&l, &b);
        for a in 0..nsrc {
            g[a][ch] = x[a];
        }
    }
    let mut operator = vec![vec![0.0; nch]; nch];
    for i in 0..nch {
        for ch in 0..nch {
            let mut s = 0.0;
            for a in 0..nsrc {
                s += lead_field[i][a] * g[a][ch];
            }
            operator[i][ch] = s;
        }
    }
    Some(RestTransform {
        reference,
        regularization,
        operator,
    })
}

pub fn apply_reference(operator: &[Vec<f64>], channel_major: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let nch = operator.len();
    if nch == 0 || channel_major.len() != nch {
        return None;
    }
    let nt = channel_major[0].len();
    let mut out = vec![vec![0.0; nt]; nch];
    for t in 0..nt {
        for i in 0..nch {
            let mut s = 0.0;
            for ch in 0..nch {
                s += operator[i][ch] * channel_major[ch][t];
            }
            out[i][t] = s;
        }
    }
    Some(out)
}

pub fn reference_row(labels: &[String], reference: &str) -> Option<usize> {
    labels.iter().position(|l| l == reference)
}

fn legendre_cos(cos_gamma: f64, order: usize) -> Vec<f64> {
    let mut p = Vec::with_capacity(order + 1);
    p.push(1.0);
    if order >= 1 {
        p.push(cos_gamma);
    }
    for n in 2..=order {
        let nf = n as f64;
        let pn = ((2.0 * nf - 1.0) * cos_gamma * p[n - 1] - (nf - 1.0) * p[n - 2]) / nf;
        p.push(pn);
    }
    p
}

fn gauss_solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    if a.len() != n || n == 0 {
        return None;
    }
    for i in 0..n {
        let mut pivot = i;
        for j in i + 1..n {
            if a[j][i].abs() > a[pivot][i].abs() {
                pivot = j;
            }
        }
        if a[pivot][i].abs() < 1e-15 {
            return None;
        }
        a.swap(i, pivot);
        b.swap(i, pivot);
        let inv = a[i][i];
        for j in i + 1..n {
            let factor = a[j][i] / inv;
            for k in i..n {
                a[j][k] -= factor * a[i][k];
            }
            b[j] -= factor * b[i];
        }
    }
    let mut x = b;
    for i in (0..n).rev() {
        for j in i + 1..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    Some(x)
}

fn cholesky(a: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    if n == 0 || a.iter().any(|row| row.len() != n) {
        return None;
    }
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut sum = a[i][j];
            for k in 0..j {
                sum -= l[i][k] * l[j][k];
            }
            if i == j {
                if sum <= 0.0 {
                    return None;
                }
                l[i][j] = sum.sqrt();
            } else {
                l[i][j] = sum / l[j][j];
            }
        }
    }
    Some(l)
}

fn cholesky_solve(l: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut y = b.to_vec();
    for i in 0..n {
        let mut sum = y[i];
        for k in 0..i {
            sum -= l[i][k] * y[k];
        }
        y[i] = sum / l[i][i];
    }
    let mut x = y;
    for i in (0..n).rev() {
        let mut sum = x[i];
        for k in i + 1..n {
            sum -= l[k][i] * x[k];
        }
        x[i] = sum / l[i][i];
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use omegaflow::te::{
        TeNull, TeStatsParams, conditional_te_stats_lagged_n, transfer_entropy_binned,
    };

    fn next_rng(rng: &mut u64) -> f64 {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)
    }

    fn white(n: usize, rng: &mut u64) -> Vec<f64> {
        (0..n).map(|_| next_rng(rng) * 2.0 - 1.0).collect()
    }

    fn fibonacci_sphere(n: usize) -> Vec<[f64; 3]> {
        let golden = PI * (3.0 - 5.0f64.sqrt());
        (0..n)
            .map(|i| {
                let y = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
                let radius = (1.0 - y * y).sqrt();
                let theta = golden * i as f64;
                [radius * theta.cos(), y, radius * theta.sin()]
            })
            .collect()
    }

    fn montage(center: [f64; 3], radius: f64, n: usize) -> Vec<[f64; 3]> {
        fibonacci_sphere(n)
            .into_iter()
            .map(|p| {
                [
                    center[0] + radius * p[0],
                    center[1] + radius * p[1],
                    center[2] + radius * p[2],
                ]
            })
            .collect()
    }

    fn angular(a: [f64; 3], b: [f64; 3]) -> f64 {
        let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let na = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
        let nb = (b[0] * b[0] + b[1] * b[1] + b[2] * b[2]).sqrt();
        (dot / (na * nb)).clamp(-1.0, 1.0).acos()
    }

    fn min_norm_estimate(lead_field: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
        let nch = lead_field.len();
        let nsrc = lead_field[0].len();
        let mut a = vec![vec![0.0; nch]; nch];
        for i in 0..nch {
            for j in 0..nch {
                let mut s = 0.0;
                for k in 0..nsrc {
                    s += lead_field[i][k] * lead_field[j][k];
                }
                a[i][j] = s;
            }
        }
        let lambda = gram_lambda_max(&a).map(|lm| lm * (0.5f64).powi(REST_REGULARIZATION_K as i32));
        if let Some(lambda) = lambda {
            for i in 0..nch {
                a[i][i] += lambda;
            }
        }
        let l = cholesky(a).expect("the gram matrix is SPD");
        let x = cholesky_solve(&l, y);
        (0..nsrc)
            .map(|k| {
                lead_field
                    .iter()
                    .enumerate()
                    .map(|(i, row)| row[k] * x[i])
                    .sum()
            })
            .collect()
    }

    #[test]
    fn gram_lambda_max_recovers_the_known_largest_eigenvalue() {
        let g = vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 5.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let lm = gram_lambda_max(&g).expect("the diagonal gram carries a largest eigenvalue");
        assert!((lm - 5.0).abs() < 1e-9);
        assert!(gram_lambda_max(&vec![vec![0.0; 3]; 3]).is_none());
    }

    #[test]
    fn sphere_fit_recovers_the_center_and_radius() {
        let center = [0.01, -0.02, 0.03];
        let radius = 0.085;
        let points = montage(center, radius, 64);
        let sphere = fit_sphere(&points).expect("the sphere fits");
        assert!((sphere.center[0] - center[0]).abs() < 1e-6);
        assert!((sphere.center[1] - center[1]).abs() < 1e-6);
        assert!((sphere.center[2] - center[2]).abs() < 1e-6);
        assert!((sphere.radius - radius).abs() < 1e-6);
    }

    #[test]
    fn three_shell_forward_matches_the_single_sphere_in_the_homogeneous_limit() {
        let sigma = 0.33;
        let shell = ThreeShell {
            brain_ratio: 0.87,
            skull_ratio: 0.93,
            sigma_brain: sigma,
            sigma_skull: sigma,
            sigma_scalp: sigma,
            order: SPHERE_ORDER,
        };
        let sphere = Sphere {
            center: [0.0, 0.0, 0.0],
            radius: 0.085,
        };
        let electrode = [0.085, 0.0, 0.0];
        let source = [0.03, 0.02, 0.01];
        let moment = 1.0;
        let got = shell
            .radial_forward(&sphere, &electrode, &source, moment)
            .expect("the forward evaluates");
        let e = electrode;
        let q = source;
        let re = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
        let rq = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
        let cos_gamma = (e[0] * q[0] + e[1] * q[1] + e[2] * q[2]) / (re * rq);
        let leg = legendre_cos(cos_gamma, SPHERE_ORDER);
        let mut want = 0.0;
        for n in 1..=SPHERE_ORDER {
            let nf = n as f64;
            want += (2.0 * nf + 1.0) * (rq / sphere.radius).powi(n as i32 - 1) * leg[n];
        }
        want *= moment / (4.0 * PI * sigma * sphere.radius * sphere.radius);
        assert!(
            (got - want).abs() / want.abs() < 1e-8,
            "three-shell in the homogeneous limit deviates: {got} vs {want}"
        );
    }

    #[test]
    fn a_known_dipole_projects_back_to_its_location() {
        let center = [0.0, 0.0, 0.0];
        let scalp_radius = 0.085;
        let electrodes = montage(center, scalp_radius, 128);
        let sphere = fit_sphere(&electrodes).expect("the montage fits a sphere");
        let shell = ThreeShell::standard();
        let source_radius = 0.75 * scalp_radius;
        let grid = montage(center, source_radius, 256);
        let lead_field = shell
            .lead_field_radial(&sphere, &electrodes, &grid)
            .expect("the lead field builds");
        let true_index = 100;
        let y: Vec<f64> = (0..lead_field.len())
            .map(|i| lead_field[i][true_index])
            .collect();
        let estimate = min_norm_estimate(&lead_field, &y);
        let best = estimate
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).expect("finite"))
            .map(|(i, _)| i)
            .expect("a source carries the peak");
        let sep = angular(grid[best], grid[true_index]);
        assert!(
            sep.to_degrees() < 15.0,
            "the dipole projects to {:.1} degrees from its true location",
            sep.to_degrees()
        );
    }

    #[test]
    fn rest_transform_names_the_reference_and_reconstructs_channel_differences() {
        let center = [0.0, 0.0, 0.0];
        let scalp_radius = 0.085;
        let electrodes = montage(center, scalp_radius, 64);
        let sphere = fit_sphere(&electrodes).expect("the montage fits a sphere");
        let shell = ThreeShell::standard();
        let grid = montage(center, 0.75 * scalp_radius, 64);
        let lead_field = shell
            .lead_field_radial(&sphere, &electrodes, &grid)
            .expect("the lead field builds");
        let reference_row = 0usize;
        let reference = "Cz".to_string();
        let transform = rest_transform(&lead_field, reference.clone(), reference_row)
            .expect("the REST transform builds");
        assert_eq!(transform.reference, reference);
        let lref = lead_field[reference_row].clone();
        let mut lr = lead_field.clone();
        for row in lr.iter_mut() {
            for (j, v) in row.iter_mut().enumerate() {
                *v -= lref[j];
            }
        }
        let nsrc = lead_field[0].len();
        let nch = lead_field.len();
        let mut gram = vec![vec![0.0; nsrc]; nsrc];
        for a in 0..nsrc {
            for b in 0..nsrc {
                let mut s = 0.0;
                for ch in 0..nch {
                    s += lr[ch][a] * lr[ch][b];
                }
                gram[a][b] = s;
            }
        }
        let expected_reg = gram_lambda_max(&gram).expect("the gram carries a largest eigenvalue")
            * (0.5f64).powi(REST_REGULARIZATION_K as i32);
        assert!(
            (transform.regularization - expected_reg).abs() < 1e-12,
            "the REST regularization is not the derived lambda_max * 2^-k: {} vs {}",
            transform.regularization,
            expected_reg
        );
        let nsrc = lead_field[0].len();
        let mut rng = 0x7C15_9E37_79B9_9E37u64;
        let s: Vec<f64> = (0..nsrc).map(|_| next_rng(&mut rng) * 2.0 - 1.0).collect();
        let v: Vec<f64> = (0..lead_field.len())
            .map(|i| lead_field[i].iter().zip(&s).map(|(l, s)| l * s).sum())
            .collect();
        let v_r: Vec<f64> = v.iter().map(|x| x - v[reference_row]).collect();
        let v_r_col: Vec<Vec<f64>> = v_r.iter().map(|x| vec![*x]).collect();
        let v_inf = apply_reference(&transform.operator, &v_r_col).expect("the operator applies");
        let max_diff = (0..v.len())
            .map(|i| {
                (0..v.len())
                    .map(|j| ((v_inf[i][0] - v_inf[j][0]) - (v[i] - v[j])).abs())
                    .fold(0.0f64, f64::max)
            })
            .fold(0.0f64, f64::max);
        let max_span = (0..v.len())
            .map(|i| {
                (0..v.len())
                    .map(|j| (v[i] - v[j]).abs())
                    .fold(0.0f64, f64::max)
            })
            .fold(0.0f64, f64::max);
        assert!(
            max_diff / max_span < 1e-4,
            "the REST operator distorts channel differences: relative {:.3e} (absolute {max_diff}) — the measured ridge distortion at k = {REST_REGULARIZATION_K} (solver-stable); no k holds the 1e-6 gate, the measured value names the tolerance",
            max_diff / max_span
        );
    }

    #[test]
    fn source_space_te_gets_a_fresh_phase_null() {
        let center = [0.0, 0.0, 0.0];
        let scalp_radius = 0.085;
        let electrodes = montage(center, scalp_radius, 64);
        let sphere = fit_sphere(&electrodes).expect("the montage fits a sphere");
        let shell = ThreeShell::standard();
        let sources = [
            [0.5 * scalp_radius, 0.0, 0.0],
            [-0.5 * scalp_radius, 0.0, 0.0],
        ];
        let lead_field = shell
            .lead_field_radial(&sphere, &electrodes, &sources)
            .expect("the lead field builds");
        let n = 512usize;
        let delay = 4usize;
        let seed = 0x9E37_79B9_7F4A_7C15u64;
        let mut rng = seed;
        let s1 = white(n, &mut rng);
        let mut s2 = vec![0.0f64; n];
        for t in 0..n {
            s2[t] = if t >= delay {
                0.95 * s1[t - delay] + 0.05 * next_rng(&mut rng)
            } else {
                0.1 * next_rng(&mut rng)
            };
        }
        let nch = lead_field.len();
        let mut sensor = vec![vec![0.0; n]; nch];
        for t in 0..n {
            for i in 0..nch {
                sensor[i][t] = lead_field[i][0] * s1[t] + lead_field[i][1] * s2[t];
            }
        }
        let mut reconstructed = vec![vec![0.0; n]; 2];
        for t in 0..n {
            let yt: Vec<f64> = (0..nch).map(|i| sensor[i][t]).collect();
            let est = min_norm_estimate(&lead_field, &yt);
            reconstructed[0][t] = est[0];
            reconstructed[1][t] = est[1];
        }
        let a: Vec<f32> = reconstructed[0].iter().map(|v| *v as f32).collect();
        let b: Vec<f32> = reconstructed[1].iter().map(|v| *v as f32).collect();
        let te = transfer_entropy_binned(&b, &a, delay, 4).expect("the source TE is measurable");
        let (_, _, fam) = conditional_te_stats_lagged_n(
            &b,
            &a,
            &[],
            TeStatsParams {
                lag: delay,
                max_lag: delay,
                bins: 4,
                seed,
                n_surr: 50,
                null: TeNull::Phase,
            },
        )
        .expect("the fresh source null is measurable");
        assert!(
            te > fam,
            "the driven source breaks its own fam-Schwelle: TE {te} vs fam {fam}"
        );
    }
}
