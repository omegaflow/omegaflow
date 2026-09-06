use crate::archivar::skydirection::{parse_bin, SkyDirection};

pub const S2_LMAX: u32 = 64;

pub const S2_OSC_CAP: u32 = 8192;

pub const S2_CPU_PROBE_CAP: usize = 256;

pub const S2_TAU_DEFAULT_S: f64 = 262144.0;

pub const S2_PERM_TAU_S: f64 = 32.0;

pub const S2_EPS: f64 = f64::EPSILON;

pub fn direction_tau_s(d: &SkyDirection, default_tau_s: f64) -> f64 {
    let mut gaps: Vec<f64> = Vec::new();
    for band in &d.bands {
        for w in band.samples.windows(2) {
            let dt = (w[1].tdb - w[0].tdb).abs();
            if dt.is_finite() && dt > 0.0 {
                gaps.push(dt);
            }
        }
    }
    if gaps.is_empty() {
        return default_tau_s;
    }
    gaps.sort_by(|a, b| a.total_cmp(b));
    gaps[gaps.len() / 2]
}

pub fn presence_weight(d: &SkyDirection, t: f64, default_tau_s: f64) -> f64 {
    let tau = direction_tau_s(d, default_tau_s);
    if !(tau.is_finite() && tau > 0.0) {
        return 0.0;
    }
    let mut w = 0.0f64;
    for band in &d.bands {
        for s in &band.samples {
            if !s.tdb.is_finite() {
                continue;
            }
            let dt = (t - s.tdb).abs();
            w += (-dt / tau).exp();
        }
    }
    w
}

pub struct S2Osc {
    pub p_hat: [f64; 3],
    pub sigma_rad: Option<f64>,
    pub weight: f64,
}

impl S2Osc {
    pub fn from_direction(d: &SkyDirection, t: f64, default_tau_s: f64) -> Self {
        S2Osc {
            p_hat: d.unit_direction(),
            sigma_rad: d.angular_uncertainty_rad(),
            weight: presence_weight(d, t, default_tau_s),
        }
    }
}

pub fn osc_window(directions: &[SkyDirection], t: f64, default_tau_s: f64) -> Vec<S2Osc> {
    directions
        .iter()
        .map(|d| S2Osc::from_direction(d, t, default_tau_s))
        .collect()
}

pub fn angular_kernel(cos_gamma: f64, sigma_rad: Option<f64>, lmax: u32) -> f64 {
    let x = cos_gamma.clamp(-1.0, 1.0);
    let sigma2: f64 = match sigma_rad {
        Some(s) if s.is_finite() && s > 0.0 => s * s,
        Some(_) => 0.0,
        None => 0.0,
    };
    let band_coef = |l: u32| -> f64 {
        if sigma2 <= 0.0 {
            return 1.0;
        }
        let lf = l as f64;
        (-0.5 * lf * (lf + 1.0) * sigma2).exp()
    };
    let mut acc = 1.0;
    if lmax >= 1 {
        acc += 3.0 * band_coef(1) * x;
    }
    let mut p0 = 1.0f64;
    let mut p1 = x;
    for l in 2..=lmax {
        let lf = l as f64;
        let p2 = ((2.0 * lf - 1.0) * x * p1 - (lf - 1.0) * p0) / lf;
        acc += (2.0 * lf + 1.0) * band_coef(l) * p2;
        p0 = p1;
        p1 = p2;
    }
    acc / (4.0 * std::f64::consts::PI)
}

pub fn field_at(q: [f64; 3], oscs: &[S2Osc], lmax: u32) -> f64 {
    let nq = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
    if !(nq.is_finite() && nq > 0.0) {
        return 0.0;
    }
    let uq = [q[0] / nq, q[1] / nq, q[2] / nq];
    let mut field = 0.0f64;
    for o in oscs {
        if !(o.weight.is_finite() && o.weight > 0.0) {
            continue;
        }
        let cosg = uq[0] * o.p_hat[0] + uq[1] * o.p_hat[1] + uq[2] * o.p_hat[2];
        field += o.weight * angular_kernel(cosg, o.sigma_rad, lmax);
    }
    field
}

pub fn self_field(o: &S2Osc, lmax: u32) -> f64 {
    if !(o.weight.is_finite() && o.weight > 0.0) {
        return 0.0;
    }
    o.weight * angular_kernel(1.0, o.sigma_rad, lmax)
}

pub fn shell_field(oscs: &[S2Osc], lmax: u32) -> Vec<f64> {
    oscs.iter().map(|o| field_at(o.p_hat, oscs, lmax)).collect()
}

pub struct S2Pack {
    pub osc: Vec<f32>,
    pub probes: Vec<f32>,
    pub count: u32,
    pub probe_count: u32,
}

pub fn pack_oscs(oscs: &[S2Osc], cap: u32) -> S2Pack {
    let n = oscs.len().min(cap as usize);
    let mut osc = vec![0.0f32; (cap as usize) * 8];
    let mut probes = vec![0.0f32; (cap as usize) * 4];
    for (i, o) in oscs.iter().take(n).enumerate() {
        let f = i * 8;
        osc[f] = o.p_hat[0] as f32;
        osc[f + 1] = o.p_hat[1] as f32;
        osc[f + 2] = o.p_hat[2] as f32;
        osc[f + 3] = o.weight as f32;
        let sigma = match o.sigma_rad {
            Some(s) if s.is_finite() && s > 0.0 => s as f32,
            Some(_) => 0.0,
            None => 0.0,
        };
        osc[f + 4] = sigma;
        let p = i * 4;
        probes[p] = o.p_hat[0] as f32;
        probes[p + 1] = o.p_hat[1] as f32;
        probes[p + 2] = o.p_hat[2] as f32;
    }
    S2Pack {
        osc,
        probes,
        count: n as u32,
        probe_count: n as u32,
    }
}

pub fn breath_target(shell: f64, shell_prev: f64, level: f64) -> f64 {
    let v_c = (shell - shell_prev).abs();
    let g = level.abs();
    (v_c / (g + S2_EPS)).tanh()
}

pub struct SkyPoint {
    pub p: [f32; 3],
    pub sigma_rad: f32,
    pub activity: f32,
    pub field: f32,
}

pub struct SkyReport {
    pub osc_count: usize,
    pub live_count: usize,
    pub shell: f64,
    pub shell_prev: f64,
    pub forward_field: f32,
    pub permeability: f32,
    pub tau_s: f64,
}

pub struct SkyState {
    pub directions: Vec<SkyDirection>,
    pub oscs: Vec<S2Osc>,
    pub points: Vec<SkyPoint>,
    pub shell: f64,
    pub shell_prev: f64,
    pub forward_field: f32,
    pub permeability: f32,
    pub loaded: bool,
}

impl SkyState {
    pub fn new() -> Self {
        SkyState {
            directions: Vec::new(),
            oscs: Vec::new(),
            points: Vec::new(),
            shell: 0.0,
            shell_prev: 0.0,
            forward_field: 0.0,
            permeability: 0.0,
            loaded: false,
        }
    }

    pub fn report(&self) -> SkyReport {
        let live_count = self
            .oscs
            .iter()
            .filter(|o| o.weight.is_finite() && o.weight > 0.0)
            .count();
        SkyReport {
            osc_count: self.oscs.len(),
            live_count,
            shell: self.shell,
            shell_prev: self.shell_prev,
            forward_field: self.forward_field,
            permeability: self.permeability,
            tau_s: S2_TAU_DEFAULT_S,
        }
    }
}

pub fn load_asset(path: &std::path::Path) -> Option<Vec<SkyDirection>> {
    let bytes = std::fs::read(path).ok()?;
    parse_bin(&bytes)
}

pub fn sky_asset_path() -> std::path::PathBuf {
    if let Ok(p) = std::env::var("OMEGAFLOW_SKY_ASSET") {
        return std::path::PathBuf::from(p);
    }
    let state = crate::archivar::state_dir().join("skydirections.bin");
    if state.exists() {
        return state;
    }
    std::path::PathBuf::from("data/skydirections.bin")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archivar::skydirection::{SkyBandSeries, SkySample};
    use crate::archivar::{C_LIGHT, HUBBLE_H0};

    fn direction(name: &str, ra: f64, dec: f64, samples: Vec<(f64, f64)>) -> SkyDirection {
        SkyDirection {
            name: name.to_string(),
            ra_deg: ra,
            dec_deg: dec,
            sigma_arcsec: None,
            bands: vec![SkyBandSeries {
                band: Some("g".to_string()),
                samples: samples
                    .into_iter()
                    .map(|(tdb, mag)| SkySample { tdb, mag })
                    .collect(),
            }],
            distance: None,
            redshift: None,
        }
    }

    #[test]
    fn a_single_direction_oscillator_creates_angular_field_mass_at_its_direction() {
        let d = direction("flare", 30.0, 60.0, vec![(8.4e8, 18.5)]);
        let t = 8.4e8;
        let oscs = osc_window(&[d], t, S2_TAU_DEFAULT_S);
        assert_eq!(oscs.len(), 1);
        let field = field_at(oscs[0].p_hat, &oscs, S2_LMAX);
        assert!(field.is_finite());
        assert!(field > 0.0);
        let peak = oscs[0].weight * ((S2_LMAX + 1) as f64).powi(2) / (4.0 * std::f64::consts::PI);
        let rel = ((field - peak) / peak).abs();
        assert!(
            rel < 1e-9,
            "self-field carries the full band-limited peak: field {field} peak {peak} rel {rel}"
        );
    }

    #[test]
    fn the_presence_weight_relaxes_exponentially_from_the_directions_own_samples() {
        let d = direction("fading", 10.0, 20.0, vec![(8.4e8, 19.0)]);
        let w_now = presence_weight(&d, 8.4e8, S2_TAU_DEFAULT_S);
        assert!((w_now - 1.0).abs() < 1e-9);
        let w_later = presence_weight(&d, 8.4e8 + S2_TAU_DEFAULT_S, S2_TAU_DEFAULT_S);
        assert!((w_later - (-1.0f64).exp()).abs() < 1e-9);
        let w_early = presence_weight(&d, 8.4e8 - S2_TAU_DEFAULT_S, S2_TAU_DEFAULT_S);
        assert!((w_early - (-1.0f64).exp()).abs() < 1e-9);
    }

    #[test]
    fn a_direction_without_a_band_series_stays_silent() {
        let d = SkyDirection {
            name: "fink".to_string(),
            ra_deg: 150.0,
            dec_deg: -10.0,
            sigma_arcsec: None,
            bands: Vec::new(),
            distance: None,
            redshift: None,
        };
        let oscs = osc_window(&[d], 8.4e8, S2_TAU_DEFAULT_S);
        assert_eq!(oscs[0].weight, 0.0);
        let field = field_at(oscs[0].p_hat, &oscs, S2_LMAX);
        assert_eq!(field, 0.0);
    }

    #[test]
    fn an_empty_sky_is_an_empty_field() {
        let oscs = osc_window(&[], 8.4e8, S2_TAU_DEFAULT_S);
        assert!(oscs.is_empty());
        assert_eq!(field_at([1.0, 0.0, 0.0], &oscs, S2_LMAX), 0.0);
        let shell = shell_field(&oscs, S2_LMAX);
        assert!(shell.is_empty());
    }

    #[test]
    fn a_measured_sigma_broadens_the_kernel_below_the_band_peak() {
        let d = direction("sharp", 45.0, 45.0, vec![(8.4e8, 17.0)]);
        let mut wide = direction("wide", 45.0, 45.0, vec![(8.4e8, 17.0)]);
        wide.sigma_arcsec = Some(0.0);
        let sharp = osc_window(&[d], 8.4e8, S2_TAU_DEFAULT_S);
        let wide_oscs = osc_window(&[wide], 8.4e8, S2_TAU_DEFAULT_S);
        let field_sharp = field_at(sharp[0].p_hat, &sharp, S2_LMAX);
        let field_wide = field_at(wide_oscs[0].p_hat, &wide_oscs, S2_LMAX);
        assert_eq!(field_sharp, field_wide);
        let mut broad = direction("broad", 45.0, 45.0, vec![(8.4e8, 17.0)]);
        broad.sigma_arcsec = Some(20626.5);
        let osc2 = osc_window(&[broad], 8.4e8, S2_TAU_DEFAULT_S);
        let field_broad = field_at(osc2[0].p_hat, &osc2, S2_LMAX);
        assert!(field_broad < field_sharp);
        assert!(field_broad > 0.0);
    }

    #[test]
    fn absent_distance_stays_on_the_sphere_and_delivered_distance_enters_the_block() {
        let absent = direction("no_dist", 30.0, 60.0, vec![(8.4e8, 18.0)]);
        assert_eq!(absent.distance_m(), None);
        assert_eq!(absent.spatial_position(), None);
        let oscs = osc_window(&[absent], 8.4e8, S2_TAU_DEFAULT_S);
        assert!((oscs[0].weight - 1.0).abs() < 1e-9);
        let placed_deg = 0.05;
        let dist = placed_deg * C_LIGHT / HUBBLE_H0;
        let mut placed = direction("redshifted", 30.0, 60.0, vec![(8.4e8, 18.0)]);
        placed.redshift = Some(placed_deg);
        let pos = placed.spatial_position().unwrap();
        let p = placed.unit_direction();
        for k in 0..3 {
            assert!((pos[k] - p[k] * dist).abs() < dist * 1e-9);
        }
    }

    #[test]
    fn pack_roundtrip_holds_oscillators_and_probes() {
        let dirs = vec![
            direction("a", 0.0, 0.0, vec![(8.4e8, 18.0)]),
            direction("b", 90.0, 0.0, vec![(8.4e8, 19.0)]),
        ];
        let oscs = osc_window(&dirs, 8.4e8, S2_TAU_DEFAULT_S);
        let pack = pack_oscs(&oscs, S2_OSC_CAP);
        assert_eq!(pack.count, 2);
        assert_eq!(pack.probe_count, 2);
        assert_eq!(pack.osc.len(), S2_OSC_CAP as usize * 8);
        assert_eq!(pack.probes.len(), S2_OSC_CAP as usize * 4);
        let n0 =
            (pack.osc[0] * pack.osc[0] + pack.osc[1] * pack.osc[1] + pack.osc[2] * pack.osc[2])
                .sqrt();
        assert!((n0 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn the_breath_target_is_the_tanh_of_the_shell_derivative() {
        let t = breath_target(2.0, 1.0, 1.5);
        assert!((t - (1.0f64 / (1.5 + S2_EPS)).tanh()).abs() < 1e-12);
        let quiet = breath_target(1.0, 1.0, 1.0);
        assert!(quiet.abs() < 1e-12);
    }
}
