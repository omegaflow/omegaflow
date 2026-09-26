pub const STA_WINDOW_S: f64 = 1.0;
pub const LTA_WINDOW_S: f64 = 30.0;
pub const STA_LTA_RATIO: f64 = 4.0;

const PI: f64 = std::f64::consts::PI;

pub fn median_abs(xs: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = xs.iter().map(|v| v.abs()).collect();
    sorted.sort_by(|a, b| a.total_cmp(b));
    sorted[sorted.len() / 2]
}

pub fn bandpass(samples: &[(f64, f64)], rate: f64) -> Vec<f64> {
    let Some(&(_, first)) = samples.first() else {
        return Vec::new();
    };
    let dt = 1.0 / rate;
    let rc_hp = 1.0 / (2.0 * PI * 0.5);
    let a_hp = rc_hp / (rc_hp + dt);
    let rc_lp = 1.0 / (2.0 * PI * 2.0);
    let a_lp = dt / (rc_lp + dt);
    let mut hp = 0.0;
    let mut lp = 0.0;
    let mut prev_x = first;
    let mut out = Vec::with_capacity(samples.len());
    for &(_, x) in samples {
        hp = a_hp * (hp + x - prev_x);
        prev_x = x;
        lp += a_lp * (hp - lp);
        out.push(lp);
    }
    out
}

pub fn sta_lta_arrival(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    let n_sta = (STA_WINDOW_S * rate).round() as usize;
    let n_lta = (LTA_WINDOW_S * rate).round() as usize;
    if n_sta == 0 || n_lta == 0 || samples.len() < n_lta + 1 {
        return None;
    }
    let n = samples.len();
    let mut prefix = Vec::with_capacity(n + 1);
    let mut acc = 0.0;
    prefix.push(0.0);
    for (_, v) in samples.iter() {
        acc += v.abs();
        prefix.push(acc);
    }
    for i in (n_lta - 1)..n {
        let sta = (prefix[i + 1] - prefix[i + 1 - n_sta]) / n_sta as f64;
        let lta = (prefix[i + 1] - prefix[i + 1 - n_lta]) / n_lta as f64;
        if lta > 1e-12 && sta / lta >= STA_LTA_RATIO {
            return Some(samples[i].0);
        }
    }
    None
}

pub fn first_break_arrival(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    let vals = bandpass(samples, rate);
    let noise_len = ((20.0 * rate).round() as usize).min(vals.len() / 2);
    if noise_len == 0 {
        return None;
    }
    let floor = median_abs(&vals[..noise_len]);
    if floor <= 1e-12 {
        return None;
    }
    let threshold = 5.0 * floor;
    let sustain = (1.0 * rate).round() as usize;
    if sustain == 0 {
        return None;
    }
    let mut count = 0usize;
    for i in noise_len..vals.len() {
        if vals[i].abs() > threshold {
            count += 1;
            if count >= sustain {
                return Some(samples[i - sustain + 1].0);
            }
        } else {
            count = 0;
        }
    }
    None
}

pub fn p_onset(samples: &[(f64, f64)], rate: f64) -> Option<f64> {
    first_break_arrival(samples, rate).or_else(|| sta_lta_arrival(samples, rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step_trace(rate: f64, dur_s: f64, onset: f64) -> Vec<(f64, f64)> {
        let n = (dur_s * rate) as usize;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let t = 1000.0 + i as f64 / rate;
            let v = if t >= onset { 10.0 } else { 0.01 };
            out.push((t, v));
        }
        out
    }

    #[test]
    fn a_step_onset_is_picked_at_its_time() {
        let rate = 40.0;
        let samples = step_trace(rate, 60.0, 1030.0);
        let pick = sta_lta_arrival(&samples, rate).unwrap();
        assert!(
            (pick - 1030.0).abs() < 1.0,
            "picked {pick} but onset at 1030.0"
        );
    }

    #[test]
    fn a_quiet_trace_carries_no_pick() {
        let rate = 40.0;
        let n = (60.0 * rate) as usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = 1000.0 + i as f64 / rate;
            samples.push((t, 0.01));
        }
        assert!(sta_lta_arrival(&samples, rate).is_none());
        assert!(p_onset(&samples, rate).is_none());
    }

    #[test]
    fn an_emergent_onset_is_picked_at_its_first_break() {
        let rate = 40.0;
        let n = (120.0 * rate) as usize;
        let onset = 1060.0;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = 1000.0 + i as f64 / rate;
            let v = if t < onset {
                0.005 * (2.0 * PI * 0.2 * t).sin()
            } else {
                let ramp = ((t - onset) / 3.0).min(1.0);
                ramp * 2.0 * (2.0 * PI * 2.0 * t).sin()
            };
            samples.push((t, v));
        }
        let pick = first_break_arrival(&samples, rate).unwrap();
        assert!(
            (pick - onset).abs() < 1.0,
            "first break {pick} should sit near the {onset} onset"
        );
    }

    #[test]
    fn bandpass_removes_a_slow_drift() {
        let rate = 40.0;
        let n = 4000usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f64 / rate;
            samples.push((t, 50.0 * t));
        }
        let vals = bandpass(&samples, rate);
        let tail = &vals[n / 2..];
        let min = tail.iter().copied().fold(f64::INFINITY, f64::min);
        let max = tail.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let spread = max - min;
        assert!(
            spread < 2.0,
            "a 50 t/s ramp must flatten after the high-pass, spread {spread}"
        );
    }

    #[test]
    fn a_pick_time_is_a_finite_option_never_a_zero_pad() {
        let rate = 40.0;
        let samples = step_trace(rate, 60.0, 1030.0);
        match p_onset(&samples, rate) {
            Some(t) => {
                assert!(t.is_finite(), "the pick time {t} is finite");
                assert!(
                    t > 0.0,
                    "the pick time {t} is a real epoch, never a zero pad"
                );
            }
            None => panic!("an onset carries a pick, never an absent zero"),
        }
    }

    #[test]
    fn a_degenerate_rate_carries_no_pick_not_a_panic() {
        let samples = step_trace(40.0, 60.0, 1030.0);
        assert!(
            p_onset(&samples, 0.0).is_none(),
            "a degenerate rate carries no pick and never divides by zero"
        );
    }
}
