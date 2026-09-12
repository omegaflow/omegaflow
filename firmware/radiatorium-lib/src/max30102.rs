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
}
