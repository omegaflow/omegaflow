pub const NN_MIN_MS: f64 = 250.0;
pub const NN_MAX_MS: f64 = 2000.0;
pub const RMSSD_MIN_DIFFS: usize = 2;
pub const TONE_FLOOR: usize = 10;
pub const TONE_ABSENT: u8 = 0;
pub const TONE_CALM: u8 = 1;
pub const TONE_STRESSED: u8 = 2;
pub const NN_WINDOW: usize = 30;

pub fn tone_code(tone: Option<Tone>) -> u8 {
    match tone {
        None => TONE_ABSENT,
        Some(Tone::Calm) => TONE_CALM,
        Some(Tone::Stressed) => TONE_STRESSED,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tone {
    Calm,
    Stressed,
}

pub fn rmssd(nn_ms: &[f64]) -> Option<f64> {
    let mut prev: Option<f64> = None;
    let mut sq_sum = 0.0;
    let mut n = 0usize;
    for &v in nn_ms {
        if !(v.is_finite() && (NN_MIN_MS..=NN_MAX_MS).contains(&v)) {
            prev = None;
            continue;
        }
        if let Some(p) = prev {
            let d = v - p;
            sq_sum += d * d;
            n += 1;
        }
        prev = Some(v);
    }
    if n < RMSSD_MIN_DIFFS {
        return None;
    }
    Some((sq_sum / n as f64).sqrt())
}

pub struct VagusTone {
    baseline: Vec<f64>,
}

impl Default for VagusTone {
    fn default() -> Self {
        Self::new()
    }
}

impl VagusTone {
    pub fn new() -> Self {
        Self {
            baseline: Vec::new(),
        }
    }

    pub fn feed(&mut self, rmssd_ms: f64) -> Option<Tone> {
        if !rmssd_ms.is_finite() || rmssd_ms <= 0.0 {
            return None;
        }
        self.baseline.push(rmssd_ms);
        if self.baseline.len() < TONE_FLOOR {
            return None;
        }
        let n = self.baseline.len() as f64;
        let mean = self.baseline.iter().sum::<f64>() / n;
        let var = self
            .baseline
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / n;
        let sd = var.sqrt();
        let latest = self.baseline[self.baseline.len() - 1];
        if latest < mean - 2.0 * sd {
            Some(Tone::Stressed)
        } else {
            Some(Tone::Calm)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rmssd_calm_series_is_high() {
        let nn: Vec<f64> = (0..60).map(|i| 900.0 + 15.0 * (i % 5) as f64).collect();
        let r = rmssd(&nn).unwrap();
        assert!(r > 0.0 && r < 30.0, "calm RMSSD {r}");
    }

    #[test]
    fn rmssd_stress_series_is_low() {
        let mut nn = Vec::new();
        for i in 0..60 {
            nn.push(if i % 2 == 0 { 700.0 } else { 1100.0 });
        }
        let r = rmssd(&nn).unwrap();
        assert!(r > 300.0, "stressed RMSSD {r}");
    }

    #[test]
    fn rmssd_skips_implausible_and_breaks_the_chain() {
        let nn = [900.0, 910.0, f64::NAN, 920.0, 930.0];
        let r = rmssd(&nn).unwrap();
        assert_eq!(r, 10.0, "only the first valid pair carries a difference");
    }

    #[test]
    fn rmssd_too_few_diffs_is_absent() {
        assert_eq!(rmssd(&[]), None);
        assert_eq!(rmssd(&[900.0]), None);
        assert_eq!(rmssd(&[900.0, 910.0]), None);
        assert!(rmssd(&[900.0, 910.0, 920.0]).is_some());
    }

    #[test]
    fn gate_stays_pending_below_the_floor() {
        let mut g = VagusTone::new();
        for _ in 0..9 {
            assert_eq!(g.feed(50.0), None);
        }
    }

    #[test]
    fn gate_speaks_calm_on_a_stable_baseline() {
        let mut g = VagusTone::new();
        let mut last = None;
        for _ in 0..12 {
            last = g.feed(50.0);
        }
        assert_eq!(last, Some(Tone::Calm));
    }

    #[test]
    fn gate_blocks_on_a_dip_below_two_sd() {
        let mut g = VagusTone::new();
        for _ in 0..10 {
            g.feed(50.0);
        }
        assert_eq!(g.feed(20.0), Some(Tone::Stressed));
    }

    #[test]
    fn gate_ignores_an_implausible_feed() {
        let mut g = VagusTone::new();
        for _ in 0..10 {
            g.feed(50.0);
        }
        assert_eq!(g.feed(f64::NAN), None);
        assert_eq!(g.feed(0.0), None);
        assert_eq!(g.feed(-1.0), None);
    }

    #[test]
    fn tone_code_maps_absent_calm_stressed() {
        assert_eq!(tone_code(None), TONE_ABSENT);
        assert_eq!(tone_code(Some(Tone::Calm)), TONE_CALM);
        assert_eq!(tone_code(Some(Tone::Stressed)), TONE_STRESSED);
    }
}
