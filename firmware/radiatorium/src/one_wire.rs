use esp_hal::delay::Delay;
use esp_hal::gpio::{DriveMode, Flex, InputConfig, OutputConfig, Pull};

use radiatorium_lib::ds18b20;

const SLOT_LOW_US: u32 = 2;
const READ_SAMPLE_US: u32 = 10;
const SLOT_WIDTH_US: u32 = 60;
const RECOVER_US: u32 = 1;
const RESET_LOW_US: u32 = 480;
const PRESENCE_SAMPLE_US: u32 = 70;
const RESET_TAIL_US: u32 = 410;
const CONVERT_MS: u32 = 750;

pub struct OneWire<'d> {
    pin: Flex<'d>,
    delay: Delay,
}

impl<'d> OneWire<'d> {
    pub fn new(pin: Flex<'d>) -> Self {
        let mut pin = pin;
        pin.apply_output_config(&OutputConfig::default().with_drive_mode(DriveMode::OpenDrain));
        pin.apply_input_config(&InputConfig::default().with_pull(Pull::Up));
        pin.set_input_enable(true);
        pin.set_output_enable(true);
        pin.set_high();
        Self {
            pin,
            delay: Delay::new(),
        }
    }

    pub fn reset(&mut self) -> bool {
        self.pin.set_low();
        self.delay.delay_micros(RESET_LOW_US);
        self.pin.set_high();
        self.delay.delay_micros(PRESENCE_SAMPLE_US);
        let present = self.pin.is_low();
        self.delay.delay_micros(RESET_TAIL_US);
        present
    }

    fn write_bit(&mut self, bit: bool) {
        self.pin.set_low();
        if bit {
            self.delay.delay_micros(SLOT_LOW_US);
            self.pin.set_high();
            self.delay.delay_micros(SLOT_WIDTH_US);
        } else {
            self.delay.delay_micros(SLOT_WIDTH_US);
            self.pin.set_high();
        }
        self.delay.delay_micros(RECOVER_US);
    }

    fn read_bit(&mut self) -> bool {
        self.pin.set_low();
        self.delay.delay_micros(SLOT_LOW_US);
        self.pin.set_high();
        self.delay.delay_micros(READ_SAMPLE_US);
        let bit = self.pin.is_high();
        self.delay.delay_micros(SLOT_WIDTH_US);
        bit
    }

    pub fn write_byte(&mut self, byte: u8) {
        for i in 0..8 {
            self.write_bit(byte & (1 << i) != 0);
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let mut byte = 0u8;
        for i in 0..8 {
            if self.read_bit() {
                byte |= 1 << i;
            }
        }
        byte
    }

    pub fn read_temperature(&mut self) -> Option<f32> {
        if !self.reset() {
            return None;
        }
        self.write_byte(ds18b20::CMD_SKIP_ROM);
        self.write_byte(ds18b20::CMD_CONVERT_T);
        self.delay.delay_millis(CONVERT_MS);

        if !self.reset() {
            return None;
        }
        self.write_byte(ds18b20::CMD_SKIP_ROM);
        self.write_byte(ds18b20::CMD_READ_SCRATCHPAD);
        let mut scratchpad = [0u8; ds18b20::SCRATCHPAD_LEN];
        for byte in &mut scratchpad {
            *byte = self.read_byte();
        }
        ds18b20::temperature_from_scratchpad(&scratchpad)
    }
}
