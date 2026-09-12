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

pub const SERVO_PERIOD_MS: f32 = 20.0;
pub const SERVO_NEUTRAL_MS: f32 = 1.5;

pub fn servo_ticks(period_ticks: u16, pulse_ms: f32, period_ms: f32) -> Option<u16> {
    if !pulse_ms.is_finite() || !period_ms.is_finite() || period_ms <= 0.0 {
        return None;
    }
    let duty = pulse_ms / period_ms;
    if !(duty >= 0.0 && duty <= 1.0) {
        return None;
    }
    Some((period_ticks as f32 * duty + 0.5) as u16)
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

    #[test]
    fn servo_ticks_map_one_ms_per_tick_at_twenty_ms_period() {
        let period_ticks = 19_999u16;
        assert_eq!(servo_ticks(period_ticks, 1.0, 20.0), Some(1000));
        assert_eq!(servo_ticks(period_ticks, 1.5, 20.0), Some(1500));
        assert_eq!(servo_ticks(period_ticks, 2.0, 20.0), Some(2000));
    }

    #[test]
    fn servo_ticks_clamps_to_the_period() {
        assert_eq!(servo_ticks(19_999, 0.0, 20.0), Some(0));
        assert_eq!(servo_ticks(19_999, 20.0, 20.0), Some(19_999));
    }

    #[test]
    fn servo_ticks_rejects_implausible_input() {
        assert_eq!(servo_ticks(19_999, f32::NAN, 20.0), None);
        assert_eq!(servo_ticks(19_999, 1.5, f32::NAN), None);
        assert_eq!(servo_ticks(19_999, 1.5, 0.0), None);
        assert_eq!(servo_ticks(19_999, -1.0, 20.0), None);
        assert_eq!(servo_ticks(19_999, 30.0, 20.0), None);
    }
}
