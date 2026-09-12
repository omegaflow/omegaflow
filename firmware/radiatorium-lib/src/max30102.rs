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
}
