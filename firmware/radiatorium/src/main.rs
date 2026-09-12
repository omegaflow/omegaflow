#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::DriveMode;
use esp_hal::ledc::{
    channel::{self, ChannelIFace},
    timer::{self, TimerIFace},
    LSGlobalClkSource, Ledc, LowSpeed,
};
use esp_hal::main;
use esp_hal::time::Rate;
use esp_hal::usb::usb_serial_jtag::UsbSerialJtag;

use radiatorium_lib::frame::FrameParser;
use radiatorium_lib::pwm;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut usb = UsbSerialJtag::new(peripherals.USB_DEVICE);

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

    loop {
        let n = usb.drain_rx_fifo(&mut rx);
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
