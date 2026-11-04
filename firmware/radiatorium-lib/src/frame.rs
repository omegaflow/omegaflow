pub const FRAME_TAG: u8 = 0x02;
pub const MASK_INTENSITY: u8 = 0x01;
pub const MASK_PAN: u8 = 0x02;
pub const MASK_TILT: u8 = 0x04;

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub intensity: Option<f32>,
    pub pan_ms: Option<f32>,
    pub tilt_ms: Option<f32>,
}

enum Phase {
    Tag,
    Mask,
    F32,
}

pub struct FrameParser {
    phase: Phase,
    mask: u8,
    slot: u8,
    buf: [u8; 4],
    filled: usize,
    intensity: Option<f32>,
    pan_ms: Option<f32>,
    tilt_ms: Option<f32>,
}

impl FrameParser {
    pub const fn new() -> Self {
        Self {
            phase: Phase::Tag,
            mask: 0,
            slot: 0,
            buf: [0; 4],
            filled: 0,
            intensity: None,
            pan_ms: None,
            tilt_ms: None,
        }
    }

    pub fn push(&mut self, byte: u8) -> Option<Frame> {
        match self.phase {
            Phase::Tag => {
                if byte == FRAME_TAG {
                    self.phase = Phase::Mask;
                }
                None
            }
            Phase::Mask => {
                if byte == 0 || byte & !(MASK_INTENSITY | MASK_PAN | MASK_TILT) != 0 {
                    self.phase = Phase::Tag;
                    return None;
                }
                self.mask = byte;
                self.intensity = None;
                self.pan_ms = None;
                self.tilt_ms = None;
                self.filled = 0;
                self.slot = first_set_bit(byte);
                self.phase = Phase::F32;
                None
            }
            Phase::F32 => {
                self.buf[self.filled] = byte;
                self.filled += 1;
                if self.filled < 4 {
                    return None;
                }
                self.filled = 0;
                let value = f32::from_bits(u32::from_le_bytes(self.buf));
                if !self.store_slot(value) {
                    self.resync();
                    return None;
                }
                let next = next_set_bit(self.mask, self.slot);
                match next {
                    Some(bit) => {
                        self.slot = bit;
                        None
                    }
                    None => {
                        self.phase = Phase::Tag;
                        Some(Frame {
                            intensity: self.intensity,
                            pan_ms: self.pan_ms,
                            tilt_ms: self.tilt_ms,
                        })
                    }
                }
            }
        }
    }

    fn store_slot(&mut self, value: f32) -> bool {
        match self.slot {
            0 => {
                if !value.is_finite() {
                    return false;
                }
                self.intensity = Some(value);
                true
            }
            1 => match crate::pwm::servo_ms(value) {
                Some(v) => {
                    self.pan_ms = Some(v);
                    true
                }
                None => false,
            },
            2 => match crate::pwm::servo_ms(value) {
                Some(v) => {
                    self.tilt_ms = Some(v);
                    true
                }
                None => false,
            },
            _ => false,
        }
    }

    fn resync(&mut self) {
        self.phase = Phase::Tag;
        self.mask = 0;
        self.slot = 0;
        self.filled = 0;
        self.intensity = None;
        self.pan_ms = None;
        self.tilt_ms = None;
    }
}

fn first_set_bit(mask: u8) -> u8 {
    let mut bit = 0u8;
    while bit < 3 && mask & (1 << bit) == 0 {
        bit += 1;
    }
    bit
}

fn next_set_bit(mask: u8, current: u8) -> Option<u8> {
    let mut bit = current + 1;
    while bit < 3 {
        if mask & (1 << bit) != 0 {
            return Some(bit);
        }
        bit += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(p: &mut FrameParser, bytes: &[u8]) -> Option<Frame> {
        let mut out = None;
        for &b in bytes {
            if let Some(f) = p.push(b) {
                out = Some(f);
            }
        }
        out
    }

    fn frame_bytes(mask: u8, values: &[f32]) -> Vec<u8> {
        let mut out = vec![FRAME_TAG, mask];
        for v in values {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    #[test]
    fn intensity_only_frame_decodes() {
        let mut p = FrameParser::new();
        let f = decode(&mut p, &frame_bytes(MASK_INTENSITY, &[1.5])).expect("frame");
        assert_eq!(f.intensity, Some(1.5));
        assert_eq!(f.pan_ms, None);
        assert_eq!(f.tilt_ms, None);
    }

    #[test]
    fn full_frame_decodes_all_three_slots() {
        let mut p = FrameParser::new();
        let f = decode(
            &mut p,
            &frame_bytes(MASK_INTENSITY | MASK_PAN | MASK_TILT, &[1.5, 1.0, 2.0]),
        )
        .expect("frame");
        assert_eq!(f.intensity, Some(1.5));
        assert_eq!(f.pan_ms, Some(1.0));
        assert_eq!(f.tilt_ms, Some(2.0));
    }

    #[test]
    fn pan_tilt_frame_skips_intensity() {
        let mut p = FrameParser::new();
        let f = decode(&mut p, &frame_bytes(MASK_PAN | MASK_TILT, &[1.0, 2.0])).expect("frame");
        assert_eq!(f.intensity, None);
        assert_eq!(f.pan_ms, Some(1.0));
        assert_eq!(f.tilt_ms, Some(2.0));
    }

    #[test]
    fn pan_only_frame_leaves_intensity_and_tilt_absent() {
        let mut p = FrameParser::new();
        let f = decode(&mut p, &frame_bytes(MASK_PAN, &[1.25])).expect("frame");
        assert_eq!(f.intensity, None);
        assert_eq!(f.pan_ms, Some(1.25));
        assert_eq!(f.tilt_ms, None);
    }

    #[test]
    fn absent_bit_is_none() {
        let mut p = FrameParser::new();
        let f = decode(&mut p, &frame_bytes(MASK_TILT, &[2.0])).expect("frame");
        assert_eq!(f.intensity, None);
        assert_eq!(f.pan_ms, None);
        assert_eq!(f.tilt_ms, Some(2.0));
    }

    #[test]
    fn zero_intensity_is_a_value() {
        let mut p = FrameParser::new();
        let f = decode(&mut p, &frame_bytes(MASK_INTENSITY, &[0.0])).expect("frame");
        assert_eq!(f.intensity, Some(0.0));
    }

    #[test]
    fn zero_pan_ms_is_a_real_direction() {
        let mut p = FrameParser::new();
        let f = decode(&mut p, &frame_bytes(MASK_PAN, &[0.0])).expect("frame");
        assert_eq!(f.pan_ms, Some(0.0));
    }

    #[test]
    fn non_finite_intensity_refuses_the_frame() {
        for v in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut p = FrameParser::new();
            assert_eq!(decode(&mut p, &frame_bytes(MASK_INTENSITY, &[v])), None);
        }
    }

    #[test]
    fn non_finite_pan_refuses_the_frame() {
        let mut p = FrameParser::new();
        assert_eq!(decode(&mut p, &frame_bytes(MASK_PAN, &[f32::NAN])), None);
    }

    #[test]
    fn out_of_range_pan_refuses_the_frame() {
        let mut p = FrameParser::new();
        assert_eq!(decode(&mut p, &frame_bytes(MASK_PAN, &[25.0])), None);
    }

    #[test]
    fn empty_mask_resyncs() {
        let mut p = FrameParser::new();
        assert_eq!(decode(&mut p, &[FRAME_TAG, 0x00]), None);
    }

    #[test]
    fn mask_with_bit_outside_range_resyncs() {
        let mut p = FrameParser::new();
        assert_eq!(decode(&mut p, &[FRAME_TAG, 0x08]), None);
    }

    #[test]
    fn stray_leading_bytes_are_dropped() {
        let mut p = FrameParser::new();
        let mut bytes = vec![0x00, 0xFF, 0x13];
        bytes.extend_from_slice(&frame_bytes(MASK_INTENSITY, &[1.5]));
        let f = decode(&mut p, &bytes).expect("frame after stray bytes");
        assert_eq!(f.intensity, Some(1.5));
    }

    #[test]
    fn consecutive_frames_decode_independently() {
        let mut p = FrameParser::new();
        let mut bytes = frame_bytes(MASK_INTENSITY, &[1.5]);
        bytes.extend_from_slice(&frame_bytes(MASK_INTENSITY, &[2.5]));
        let mut frames = Vec::new();
        for b in bytes {
            if let Some(f) = p.push(b) {
                frames.push(f);
            }
        }
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].intensity, Some(1.5));
        assert_eq!(frames[1].intensity, Some(2.5));
    }
}
