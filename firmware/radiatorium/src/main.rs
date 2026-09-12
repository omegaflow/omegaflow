#![no_std]
#![no_main]

use core::fmt::Write as _;

use esp_backtrace as _;
use esp_hal::gpio::{DriveMode, Pin};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::ledc::{
    channel::{self, ChannelIFace},
    timer::{self, TimerIFace},
    LSGlobalClkSource, Ledc, LowSpeed,
};
use esp_hal::main;
use esp_hal::mcpwm::{operator::PwmPinConfig, timer::PwmWorkingMode, McPwm, PeripheralClockConfig};
use esp_hal::time::Rate;
use esp_hal::usb::usb_serial_jtag::UsbSerialJtag;

use radiatorium_lib::frame::FrameParser;
use radiatorium_lib::pwm;
use radiatorium_lib::{max30102, mux, nn};

const SERVO_TIMER_PERIOD_TICKS: u16 = 19_999;
const SERVO_TIMER_PRESCALER: u8 = 159;
const SAMPLE_RATE_HZ: f64 = 100.0;
const WINDOW_LEN: usize = 256;
const FIFO_SAMPLES: usize = 32;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let usb = UsbSerialJtag::new(peripherals.USB_DEVICE);
    let (mut usb_rx, mut usb_tx) = usb.split();

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut timer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    timer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty13Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(24),
        })
        .unwrap();

    let mut c0 = ledc.channel::<LowSpeed>(channel::Number::Channel0, peripherals.GPIO48.degrade());
    let mut c1 = ledc.channel::<LowSpeed>(channel::Number::Channel1, peripherals.GPIO38.degrade());
    let mut c2 = ledc.channel::<LowSpeed>(channel::Number::Channel2, peripherals.GPIO39.degrade());
    let mut c3 = ledc.channel::<LowSpeed>(channel::Number::Channel3, peripherals.GPIO40.degrade());
    let mut c4 = ledc.channel::<LowSpeed>(channel::Number::Channel4, peripherals.GPIO41.degrade());
    let mut c5 = ledc.channel::<LowSpeed>(channel::Number::Channel5, peripherals.GPIO47.degrade());
    let mut c6 = ledc.channel::<LowSpeed>(channel::Number::Channel6, peripherals.GPIO42.degrade());
    let mut c7 = ledc.channel::<LowSpeed>(channel::Number::Channel7, peripherals.GPIO21.degrade());

    for ch in [
        &mut c0, &mut c1, &mut c2, &mut c3, &mut c4, &mut c5, &mut c6, &mut c7,
    ] {
        ch.configure(channel::config::Config {
            timer: &timer0,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .unwrap();
    }

    let mut parser = FrameParser::new();
    let mut rx = [0u8; 64];

    let mcpwm_clock = PeripheralClockConfig::with_prescaler(0);
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, mcpwm_clock);

    mcpwm.operator0.set_timer(&mcpwm.timer0);
    mcpwm.operator1.set_timer(&mcpwm.timer0);

    let mut pan = mcpwm
        .operator0
        .with_pin_a(peripherals.GPIO15, PwmPinConfig::UP_ACTIVE_HIGH);
    let mut tilt = mcpwm
        .operator1
        .with_pin_a(peripherals.GPIO16, PwmPinConfig::UP_ACTIVE_HIGH);

    let servo_timer = mcpwm_clock.timer_clock_with_prescaler(
        SERVO_TIMER_PERIOD_TICKS,
        PwmWorkingMode::Increase,
        SERVO_TIMER_PRESCALER,
    );
    mcpwm.timer0.start(servo_timer);

    let neutral = pwm::servo_ticks(
        SERVO_TIMER_PERIOD_TICKS,
        pwm::SERVO_NEUTRAL_MS,
        pwm::SERVO_PERIOD_MS,
    )
    .unwrap();
    pan.set_timestamp(neutral);
    tilt.set_timestamp(neutral);

    let mut i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO8)
    .with_scl(peripherals.GPIO9);

    let (mux_addr, control) = mux::select(0).expect("way 0");
    let _ = i2c.write(mux_addr, &[control]);

    let _ = i2c.write(
        max30102::I2C_ADDR,
        &[
            max30102::REG_SPO2_CONFIG,
            max30102::spo2_config_100hz_18bit_411us(),
        ],
    );
    let _ = i2c.write(
        max30102::I2C_ADDR,
        &[max30102::REG_LED1_PA, max30102::LED1_PA_DEFAULT],
    );
    let _ = i2c.write(
        max30102::I2C_ADDR,
        &[max30102::REG_LED2_PA, max30102::LED2_PA_DEFAULT],
    );
    let _ = i2c.write(
        max30102::I2C_ADDR,
        &[max30102::REG_FIFO_CONFIG, max30102::fifo_config_avg1()],
    );
    let _ = i2c.write(
        max30102::I2C_ADDR,
        &[max30102::REG_MODE_CONFIG, max30102::mode_config_spo2()],
    );

    let min_gap_samples = (nn::NN_MIN_MS / 1000.0 * SAMPLE_RATE_HZ) as usize;

    let mut window = [0u32; WINDOW_LEN];
    let mut window_len: usize = 0;
    let mut base_index: usize = 0;
    let mut last_emitted: usize = 0;

    let mut fifo_buf = [0u8; FIFO_SAMPLES * max30102::SAMPLE_BYTES];
    let mut peak_buf = [0usize; WINDOW_LEN];
    let mut interval_buf = [0.0f64; WINDOW_LEN];
    let mut line = [0u8; 16];

    loop {
        let n = usb_rx.drain_rx_fifo(&mut rx);
        for &byte in &rx[..n] {
            let Some(frame) = parser.push(byte) else {
                continue;
            };
            if let Some(intensity) = frame.intensity {
                if let Some(percent) = pwm::duty_percent(intensity) {
                    for ch in [
                        &mut c0, &mut c1, &mut c2, &mut c3, &mut c4, &mut c5, &mut c6, &mut c7,
                    ] {
                        ch.set_duty(percent).unwrap();
                    }
                }
            }
            if let Some(pan_ms) = frame.pan_ms {
                if let Some(ticks) =
                    pwm::servo_ticks(SERVO_TIMER_PERIOD_TICKS, pan_ms, pwm::SERVO_PERIOD_MS)
                {
                    pan.set_timestamp(ticks);
                }
            }
            if let Some(tilt_ms) = frame.tilt_ms {
                if let Some(ticks) =
                    pwm::servo_ticks(SERVO_TIMER_PERIOD_TICKS, tilt_ms, pwm::SERVO_PERIOD_MS)
                {
                    tilt.set_timestamp(ticks);
                }
            }
        }

        let mut rd_buf = [0u8; 1];
        let mut wr_buf = [0u8; 1];
        let pointers_ok = i2c
            .write_read(
                max30102::I2C_ADDR,
                &[max30102::REG_FIFO_RD_PTR],
                &mut rd_buf,
            )
            .is_ok()
            && i2c
                .write_read(
                    max30102::I2C_ADDR,
                    &[max30102::REG_FIFO_WR_PTR],
                    &mut wr_buf,
                )
                .is_ok();
        if pointers_ok {
            let rd = (rd_buf[0] & 0x1F) as usize;
            let wr = (wr_buf[0] & 0x1F) as usize;
            let avail = (wr.wrapping_sub(rd)) & 0x1F;
            if avail > 0 {
                let nbytes = avail * max30102::SAMPLE_BYTES;
                if i2c
                    .write_read(
                        max30102::I2C_ADDR,
                        &[max30102::REG_FIFO_DATA],
                        &mut fifo_buf[..nbytes],
                    )
                    .is_ok()
                {
                    for (_red, ir) in max30102::Fifo::new(&fifo_buf[..nbytes]) {
                        if window_len < WINDOW_LEN {
                            window[window_len] = ir;
                            window_len += 1;
                        } else {
                            window.copy_within(1.., 0);
                            window[WINDOW_LEN - 1] = ir;
                            base_index += 1;
                        }
                    }
                }
            }
        }

        let n_peaks = nn::peaks_into(&window[..window_len], min_gap_samples, &mut peak_buf);
        let n_int = nn::intervals_ms_into(&peak_buf[..n_peaks], SAMPLE_RATE_HZ, &mut interval_buf);

        let mut emitted = 0usize;
        for k in 1..n_peaks {
            let start = base_index + peak_buf[k - 1];
            let end = base_index + peak_buf[k];
            let dt_ms = (end - start) as f64 * 1000.0 / SAMPLE_RATE_HZ;
            if dt_ms < nn::NN_MIN_MS || dt_ms > nn::NN_MAX_MS {
                continue;
            }
            if emitted >= n_int {
                break;
            }
            let value = interval_buf[emitted];
            emitted += 1;
            if start < last_emitted {
                continue;
            }
            let len = write_nn_line(&mut line, value);
            let _ = usb_tx.write(&line[..len]);
            last_emitted = end;
        }
    }
}

struct LineWriter<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl core::fmt::Write for LineWriter<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let end = self.len + s.len();
        if end > self.buf.len() {
            return Err(core::fmt::Error);
        }
        self.buf[self.len..end].copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

fn write_nn_line(line: &mut [u8], ms: f64) -> usize {
    let mut w = LineWriter { buf: line, len: 0 };
    let tenths = (ms * 10.0 + 0.5) as i32;
    let whole = tenths / 10;
    let frac = tenths % 10;
    let _ = write!(w, "nn={}.{}\n", whole, frac);
    w.len
}
