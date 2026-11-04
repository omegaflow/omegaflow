pub const I2C_ADDR: u8 = 0x57;

pub const REG_INTR_STATUS_1: u8 = 0x00;
pub const REG_INTR_STATUS_2: u8 = 0x01;
pub const REG_FIFO_WR_PTR: u8 = 0x04;
pub const REG_OVF_COUNTER: u8 = 0x05;
pub const REG_FIFO_RD_PTR: u8 = 0x06;
pub const REG_FIFO_DATA: u8 = 0x07;
pub const REG_FIFO_CONFIG: u8 = 0x08;
pub const REG_MODE_CONFIG: u8 = 0x09;
pub const REG_SPO2_CONFIG: u8 = 0x0A;
pub const REG_LED1_PA: u8 = 0x0C;
pub const REG_LED2_PA: u8 = 0x0D;

pub const SAMPLE_BYTES: usize = 6;
pub const SAMPLE_MASK: u32 = 0x3FFFF;

pub const LED1_PA_DEFAULT: u8 = 0x24;
pub const LED2_PA_DEFAULT: u8 = 0x24;

pub fn mode_config_spo2() -> u8 {
    0x03
}

pub fn spo2_config_100hz_18bit_411us() -> u8 {
    0x07
}

pub fn fifo_config_avg1() -> u8 {
    0x00
}

pub fn parse_sample(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < SAMPLE_BYTES {
        return None;
    }
    let red = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    let ir = ((bytes[3] as u32) << 16) | ((bytes[4] as u32) << 8) | (bytes[5] as u32);
    Some((red & SAMPLE_MASK, ir & SAMPLE_MASK))
}

pub struct Fifo<'a> {
    rest: &'a [u8],
}

impl<'a> Fifo<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { rest: bytes }
    }
}

impl Iterator for Fifo<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.len() < SAMPLE_BYTES {
            return None;
        }
        let sample = parse_sample(&self.rest[..SAMPLE_BYTES]);
        self.rest = &self.rest[SAMPLE_BYTES..];
        sample
    }
}

pub fn spo2_from_samples(red: &[u32], ir: &[u32]) -> Option<f64> {
    if red.is_empty() || red.len() != ir.len() {
        return None;
    }
    let n = red.len() as f64;
    let mean_red = red.iter().map(|&v| v as f64).sum::<f64>() / n;
    let mean_ir = ir.iter().map(|&v| v as f64).sum::<f64>() / n;
    if mean_red <= 0.0 || mean_ir <= 0.0 {
        return None;
    }
    let ac_red = red
        .iter()
        .map(|&v| {
            let d = v as f64 - mean_red;
            d * d
        })
        .sum::<f64>();
    let ac_ir = ir
        .iter()
        .map(|&v| {
            let d = v as f64 - mean_ir;
            d * d
        })
        .sum::<f64>();
    let r2 = (ac_red / (mean_red * mean_red)) / (ac_ir / (mean_ir * mean_ir));
    if !r2.is_finite() || r2 <= 0.0 {
        return None;
    }
    let r = sqrt(r2);
    let spo2 = -45.060 * r2 + 30.354 * r + 94.845;
    if spo2.is_finite() && spo2 > 0.0 && spo2 <= 100.0 {
        Some(spo2)
    } else {
        None
    }
}

fn sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return x;
    }
    let mut guess = x;
    for _ in 0..64 {
        guess = 0.5 * (guess + x / guess);
    }
    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_one_sample() {
        let bytes = [0x00, 0x00, 0x01, 0x00, 0x00, 0x02];
        assert_eq!(parse_sample(&bytes), Some((1, 2)));
    }

    #[test]
    fn masks_channel_flags_to_18_bits() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(parse_sample(&bytes), Some((SAMPLE_MASK, SAMPLE_MASK)));
    }

    #[test]
    fn short_buffer_is_absent() {
        assert_eq!(parse_sample(&[0x00; 5]), None);
    }

    #[test]
    fn fifo_iterates_samples_and_drops_partial_tail() {
        let bytes = [
            0x00, 0x00, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x03, 0x00, 0x00, 0x04, 0x00, 0x00,
        ];
        let samples: Vec<(u32, u32)> = Fifo::new(&bytes).collect();
        assert_eq!(samples, vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn mode_config_byte_is_spo2() {
        assert_eq!(mode_config_spo2(), 0x03);
    }

    #[test]
    fn spo2_config_byte_is_100hz_18bit_411us() {
        assert_eq!(spo2_config_100hz_18bit_411us(), 0x07);
    }

    #[test]
    fn led_pa_bytes_are_the_named_constants() {
        assert_eq!(LED1_PA_DEFAULT, 0x24);
        assert_eq!(LED2_PA_DEFAULT, 0x24);
    }

    #[test]
    fn fifo_config_byte_is_avg1_no_rollover() {
        assert_eq!(fifo_config_avg1(), 0x00);
    }

    #[test]
    fn the_pulsatile_ratio_maps_to_a_plausible_saturation() {
        let red = [990u32, 1010, 990, 1010];
        let ir = [980u32, 1020, 980, 1020];
        let spo2 = spo2_from_samples(&red, &ir).expect("saturation");
        assert!(spo2 > 0.0 && spo2 <= 100.0);
        assert!((spo2 - 98.757).abs() < 0.01, "got {spo2}");
    }

    #[test]
    fn the_flat_signal_has_no_pulsatile_ratio() {
        let red = [1000u32; 4];
        let ir = [1000u32; 4];
        assert_eq!(spo2_from_samples(&red, &ir), None);
    }

    #[test]
    fn the_mismatched_windows_are_absent() {
        assert_eq!(spo2_from_samples(&[1u32, 2], &[1u32]), None);
        assert_eq!(spo2_from_samples(&[], &[]), None);
    }

    #[test]
    fn the_implausible_ratio_is_absent() {
        let red = [0u32, 10000, 0, 10000];
        let ir = [1000u32, 1001, 1000, 1001];
        assert_eq!(spo2_from_samples(&red, &ir), None);
    }
}
