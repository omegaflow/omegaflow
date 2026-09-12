pub fn duty_percent(intensity: f32) -> Option<u8> {
    if !intensity.is_finite() {
        return None;
    }
    Some((intensity.clamp(0.0, 1.0) * 100.0) as u8)
}

pub fn servo_pulse_ms(intensity: f32) -> Option<f32> {
    if !intensity.is_finite() {
        return None;
    }
    Some(1.0 + intensity.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_intensity_is_full_duty() {
        assert_eq!(duty_percent(1.0), Some(100));
    }

    #[test]
    fn zero_intensity_is_zero_duty() {
        assert_eq!(duty_percent(0.0), Some(0));
    }

    #[test]
    fn half_intensity_is_half_duty() {
        assert_eq!(duty_percent(0.5), Some(50));
    }

    #[test]
    fn negative_clamps_to_zero() {
        assert_eq!(duty_percent(-0.5), Some(0));
    }

    #[test]
    fn above_one_clamps_to_full() {
        assert_eq!(duty_percent(1.5), Some(100));
    }

    #[test]
    fn non_finite_is_absent() {
        assert_eq!(duty_percent(f32::NAN), None);
        assert_eq!(duty_percent(f32::INFINITY), None);
    }

    #[test]
    fn servo_spans_one_to_two_ms() {
        assert_eq!(servo_pulse_ms(0.0), Some(1.0));
        assert_eq!(servo_pulse_ms(1.0), Some(2.0));
        assert_eq!(servo_pulse_ms(0.5), Some(1.5));
        assert_eq!(servo_pulse_ms(-1.0), Some(1.0));
        assert_eq!(servo_pulse_ms(2.0), Some(2.0));
    }
}
