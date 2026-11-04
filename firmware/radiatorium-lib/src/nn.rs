pub const NN_MIN_MS: f64 = 250.0;
pub const NN_MAX_MS: f64 = 2000.0;

pub fn peaks_into(samples: &[u32], min_gap_samples: usize, out: &mut [usize]) -> usize {
    if samples.len() < 3 || out.is_empty() {
        return 0;
    }
    let (min, max) = range(samples);
    let threshold = min as f64 + 0.5 * (max - min) as f64;
    let mut count = 0;
    let mut last_peak: Option<usize> = None;
    for i in 1..samples.len() - 1 {
        let s = samples[i];
        if !(samples[i - 1] <= s && s >= samples[i + 1]) || (s as f64) <= threshold {
            continue;
        }
        if let Some(last) = last_peak {
            if i - last < min_gap_samples {
                if s > samples[last] {
                    out[count - 1] = i;
                    last_peak = Some(i);
                }
                continue;
            }
        }
        if count == out.len() {
            break;
        }
        out[count] = i;
        count += 1;
        last_peak = Some(i);
    }
    count
}

pub fn intervals_ms_into(peaks: &[usize], sample_rate_hz: f64, out: &mut [f64]) -> usize {
    if !sample_rate_hz.is_finite() || sample_rate_hz <= 0.0 || out.is_empty() {
        return 0;
    }
    let mut count = 0;
    for w in peaks.windows(2) {
        let dt_ms = (w[1] - w[0]) as f64 * 1000.0 / sample_rate_hz;
        if dt_ms >= NN_MIN_MS && dt_ms <= NN_MAX_MS {
            if count == out.len() {
                break;
            }
            out[count] = dt_ms;
            count += 1;
        }
    }
    count
}

fn range(samples: &[u32]) -> (u32, u32) {
    let mut min = samples[0];
    let mut max = samples[0];
    for &s in &samples[1..] {
        if s < min {
            min = s;
        }
        if s > max {
            max = s;
        }
    }
    (min, max)
}

#[cfg(test)]
pub fn peaks(samples: &[u32], min_gap_samples: usize) -> Vec<usize> {
    let mut buf = vec![0usize; samples.len()];
    let n = peaks_into(samples, min_gap_samples, &mut buf);
    buf.truncate(n);
    buf
}

#[cfg(test)]
pub fn intervals_ms(peaks: &[usize], sample_rate_hz: f64) -> Vec<f64> {
    let mut buf = vec![0.0f64; peaks.len().saturating_sub(1)];
    let n = intervals_ms_into(peaks, sample_rate_hz, &mut buf);
    buf.truncate(n);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    fn train(spikes: &[usize], amplitude: u32, len: usize) -> Vec<u32> {
        let mut s = vec![0u32; len];
        for &i in spikes {
            s[i] = amplitude;
        }
        s
    }

    #[test]
    fn steady_pulse_train_yields_600_ms_intervals() {
        let spikes = [60, 120, 180, 240, 300];
        let s = train(&spikes, 100, 361);
        let p = peaks(&s, 25);
        assert_eq!(p, vec![60, 120, 180, 240, 300]);
        let iv = intervals_ms(&p, 100.0);
        assert_eq!(iv.len(), 4);
        for v in iv {
            assert!((v - 600.0).abs() < 0.5, "interval {v}");
        }
    }

    #[test]
    fn varying_gaps_yield_500_and_700_ms() {
        let spikes = [50, 100, 170];
        let s = train(&spikes, 100, 231);
        let p = peaks(&s, 25);
        assert_eq!(p, vec![50, 100, 170]);
        let iv = intervals_ms(&p, 100.0);
        assert_eq!(iv, vec![500.0, 700.0]);
    }

    #[test]
    fn flat_signal_has_no_peaks() {
        let s = vec![100u32; 100];
        assert!(peaks(&s, 25).is_empty());
    }

    #[test]
    fn fewer_than_three_samples_is_empty() {
        assert!(peaks(&[100], 25).is_empty());
        assert!(peaks(&[100, 100], 25).is_empty());
        assert!(peaks(&[], 25).is_empty());
    }

    #[test]
    fn two_close_peaks_keep_the_larger_only() {
        let spikes = [10, 12];
        let mut s = train(&spikes, 100, 30);
        s[12] = 80;
        let p = peaks(&s, 25);
        assert_eq!(p, vec![10]);
        assert!(intervals_ms(&p, 100.0).is_empty());
    }

    #[test]
    fn interval_within_bounds_is_kept() {
        assert_eq!(intervals_ms(&[0, 100], 100.0), vec![1000.0]);
    }

    #[test]
    fn interval_above_max_is_dropped() {
        assert!(intervals_ms(&[0, 500], 100.0).is_empty());
    }

    #[test]
    fn below_min_interval_is_dropped() {
        assert!(intervals_ms(&[0, 20], 100.0).is_empty());
    }

    #[test]
    fn peaks_are_deterministic() {
        let spikes = [40, 100, 160, 220, 280];
        let s = train(&spikes, 100, 341);
        let a = peaks(&s, 25);
        let b = peaks(&s, 25);
        assert_eq!(a, b);
    }

    #[test]
    fn peaks_into_does_not_overflow_the_buffer() {
        let spikes = [40, 100, 160, 220, 280];
        let s = train(&spikes, 100, 341);
        let mut out = [0usize; 3];
        assert_eq!(peaks_into(&s, 25, &mut out), 3);
        assert_eq!(&out[..], &[40, 100, 160]);
    }

    #[test]
    fn intervals_ms_into_rejects_non_positive_rate() {
        let mut out = [0.0f64; 4];
        assert_eq!(intervals_ms_into(&[0, 60], 0.0, &mut out), 0);
        assert_eq!(intervals_ms_into(&[0, 60], f64::NAN, &mut out), 0);
    }
}
