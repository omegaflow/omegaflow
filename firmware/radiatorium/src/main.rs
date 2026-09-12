#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{DriveMode, Pin};
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

const SERVO_TIMER_PERIOD_TICKS: u16 = 19_999;
const SERVO_TIMER_PRESCALER: u8 = 159;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let usb = UsbSerialJtag::new(peripherals.USB_DEVICE);
    let (mut usb_rx, _usb_tx) = usb.split();

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

    loop {
        let n = usb_rx.drain_rx_fifo(&mut rx);
        for &byte in &rx[..n] {
            let Some(intensity) = parser.push(byte) else {
                continue;
            };
            let Some(percent) = pwm::duty_percent(intensity) else {
                continue;
            };
            for ch in [
                &mut c0, &mut c1, &mut c2, &mut c3, &mut c4, &mut c5, &mut c6, &mut c7,
            ] {
                ch.set_duty(percent).unwrap();
            }
        }
    }
}
