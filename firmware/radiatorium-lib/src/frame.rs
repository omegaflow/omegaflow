pub struct FrameParser {
    buf: [u8; 4],
    filled: usize,
}

impl FrameParser {
    pub const fn new() -> Self {
        Self {
            buf: [0; 4],
            filled: 0,
        }
    }

    pub fn push(&mut self, byte: u8) -> Option<f32> {
        self.buf[self.filled] = byte;
        self.filled += 1;
        if self.filled < 4 {
            return None;
        }
        self.filled = 0;
        let value = f32::from_bits(u32::from_le_bytes(self.buf));
        if value.is_finite() {
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembles_little_endian_frame() {
        let mut p = FrameParser::new();
        assert_eq!(p.push(0x00), None);
        assert_eq!(p.push(0x00), None);
        assert_eq!(p.push(0x80), None);
        assert_eq!(p.push(0x3F), Some(1.0));
    }

    #[test]
    fn splits_consecutive_frames() {
        let mut p = FrameParser::new();
        let bytes = [0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x40];
        let mut out = [None; 2];
        let mut i = 0;
        for b in bytes {
            if let Some(v) = p.push(b) {
                out[i] = Some(v);
                i += 1;
            }
        }
        assert_eq!(out, [Some(1.0), Some(2.0)]);
    }

    #[test]
    fn zero_is_a_value() {
        let mut p = FrameParser::new();
        p.push(0x00);
        p.push(0x00);
        p.push(0x00);
        assert_eq!(p.push(0x00), Some(0.0));
    }

    #[test]
    fn non_finite_is_absent() {
        let mut p = FrameParser::new();
        assert_eq!(p.push(0x00), None);
        assert_eq!(p.push(0x00), None);
        assert_eq!(p.push(0xC0), None);
        assert_eq!(p.push(0x7F), None);
    }
}
