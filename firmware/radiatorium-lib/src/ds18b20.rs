pub const CUTOFF_C: f32 = 80.0;

pub const RELEASE_C: f32 = 70.0;

pub const CMD_CONVERT_T: u8 = 0x44;
pub const CMD_READ_SCRATCHPAD: u8 = 0xBE;
pub const CMD_SKIP_ROM: u8 = 0xCC;

pub const SCRATCHPAD_LEN: usize = 9;

pub fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x01 != 0 {
                crc = (crc >> 1) ^ 0x8C;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

pub fn temperature_from_scratchpad(scratchpad: &[u8]) -> Option<f32> {
    if scratchpad.len() < SCRATCHPAD_LEN {
        return None;
    }
    if crc8(&scratchpad[..SCRATCHPAD_LEN]) != 0 {
        return None;
    }
    let raw = i16::from_le_bytes([scratchpad[0], scratchpad[1]]);
    Some(raw as f32 / 16.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Run,
    Trip,
    Hold,
    SensorAbsent,
}

pub struct Cutoff {
    tripped: bool,
}

impl Cutoff {
    pub const fn new() -> Self {
        Self { tripped: true }
    }

    pub fn is_tripped(&self) -> bool {
        self.tripped
    }

    pub fn evaluate(&mut self, temp_c: Option<f32>) -> Verdict {
        let Some(t) = temp_c else {
            self.tripped = true;
            return Verdict::SensorAbsent;
        };
        if !t.is_finite() {
            self.tripped = true;
            return Verdict::SensorAbsent;
        }
        if t >= CUTOFF_C {
            self.tripped = true;
            Verdict::Trip
        } else if t <= RELEASE_C {
            self.tripped = false;
            Verdict::Run
        } else if self.tripped {
            Verdict::Hold
        } else {
            Verdict::Run
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratchpad(temp_raw: i16, crc: u8) -> [u8; SCRATCHPAD_LEN] {
        let [lsb, msb] = temp_raw.to_le_bytes();
        let mut s = [0u8; SCRATCHPAD_LEN];
        s[0] = lsb;
        s[1] = msb;
        s[2] = 0x4B;
        s[3] = 0x46;
        s[4] = 0x7F;
        s[5] = 0xFF;
        s[6] = 0x0C;
        s[7] = 0x10;
        s[8] = crc;
        s
    }

    #[test]
    fn crc8_matches_the_maxim_check_value() {
        assert_eq!(crc8(b"123456789"), 0xA1);
    }

    #[test]
    fn crc8_empty_and_zero_data_is_zero() {
        assert_eq!(crc8(&[]), 0x00);
        assert_eq!(crc8(&[0x00]), 0x00);
    }

    #[test]
    fn crc8_is_zero_over_a_message_with_its_appended_crc() {
        let msg = [0x50, 0x05, 0x4B, 0x46, 0x7F, 0xFF, 0x0C, 0x10];
        let crc = crc8(&msg);
        let mut with_crc = [0u8; 9];
        with_crc[..8].copy_from_slice(&msg);
        with_crc[8] = crc;
        assert_eq!(crc8(&with_crc), 0x00);
    }

    #[test]
    fn positive_temperature_decodes() {
        let crc = crc8(&[0x50, 0x05, 0x4B, 0x46, 0x7F, 0xFF, 0x0C, 0x10]);
        assert_eq!(
            temperature_from_scratchpad(&scratchpad(0x0550, crc)),
            Some(85.0)
        );
    }

    #[test]
    fn negative_temperature_decodes() {
        let crc = crc8(&[0xF8, 0xFF, 0x4B, 0x46, 0x7F, 0xFF, 0x0C, 0x10]);
        assert_eq!(temperature_from_scratchpad(&scratchpad(-8, crc)), Some(-0.5));
    }

    #[test]
    fn corrupt_crc_is_absent() {
        assert_eq!(temperature_from_scratchpad(&scratchpad(0x0550, 0x00)), None);
    }

    #[test]
    fn short_scratchpad_is_absent() {
        assert_eq!(temperature_from_scratchpad(&[0x50, 0x05]), None);
        assert_eq!(temperature_from_scratchpad(&[]), None);
    }

    #[test]
    fn cutoff_trips_at_the_threshold() {
        let mut c = Cutoff::new();
        assert!(c.is_tripped());
        assert_eq!(c.evaluate(Some(25.0)), Verdict::Run);
        assert!(!c.is_tripped());
        assert_eq!(c.evaluate(Some(80.0)), Verdict::Trip);
        assert!(c.is_tripped());
    }

    #[test]
    fn cutoff_holds_hysteresis_between_seventy_and_eighty() {
        let mut c = Cutoff::new();
        c.evaluate(Some(25.0));
        assert_eq!(c.evaluate(Some(75.0)), Verdict::Run);
        assert!(!c.is_tripped());
        c.evaluate(Some(80.0));
        assert_eq!(c.evaluate(Some(75.0)), Verdict::Hold);
        assert!(c.is_tripped());
        assert_eq!(c.evaluate(Some(69.9)), Verdict::Run);
        assert!(!c.is_tripped());
    }

    #[test]
    fn release_threshold_rearms_exactly_at_seventy() {
        let mut c = Cutoff::new();
        c.evaluate(Some(25.0));
        c.evaluate(Some(80.0));
        assert_eq!(c.evaluate(Some(70.0)), Verdict::Run);
        assert!(!c.is_tripped());
    }

    #[test]
    fn sensor_absent_fails_safe() {
        let mut c = Cutoff::new();
        assert!(c.is_tripped());
        assert_eq!(c.evaluate(None), Verdict::SensorAbsent);
        assert!(c.is_tripped());
        c.evaluate(Some(25.0));
        assert!(!c.is_tripped());
        assert_eq!(c.evaluate(None), Verdict::SensorAbsent);
        assert!(c.is_tripped());
    }

    #[test]
    fn non_finite_temperature_is_absent() {
        let mut c = Cutoff::new();
        assert_eq!(c.evaluate(Some(f32::NAN)), Verdict::SensorAbsent);
        assert_eq!(c.evaluate(Some(f32::INFINITY)), Verdict::SensorAbsent);
        assert_eq!(c.evaluate(Some(f32::NEG_INFINITY)), Verdict::SensorAbsent);
        assert!(c.is_tripped());
    }
}
