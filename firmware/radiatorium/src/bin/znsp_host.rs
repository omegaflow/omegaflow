#![no_std]
#![no_main]

use core::fmt::Write as _;

use esp_backtrace as _;
use esp_hal::main;
use esp_hal::uart::{Config as UartConfig, Uart};
use esp_hal::usb::usb_serial_jtag::UsbSerialJtag;

use radiatorium_lib::znsp::{Frame, FrameType, NetworkMachine, Slip, SlipDecoder, cmd};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut usb = UsbSerialJtag::new(peripherals.USB_DEVICE);

    let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
        .expect("uart1 115200 8n1")
        .with_rx(peripherals.GPIO5)
        .with_tx(peripherals.GPIO4);

    let mut decoder = SlipDecoder::new();
    let mut host = NetworkMachine::new();
    let mut encoded = [0u8; 64];
    let mut rx = [0u8; 128];

    let _ = writeln!(usb, "znsp host ready");

    if let Some(payload) = host.init_request() {
        if let Some(n) = Frame::encode(
            FrameType::Request,
            cmd::NETWORK_INIT,
            0,
            payload,
            &mut encoded,
        ) {
            let _ = uart.write(&encoded[..n]);
        }
    }

    loop {
        let n = match uart.read(&mut rx) {
            Ok(n) => n,
            Err(_) => continue,
        };
        for &byte in &rx[..n] {
            match decoder.push(byte) {
                Slip::Packet => {
                    if let Some(frame) = Frame::parse(decoder.packet()) {
                        let _ = print_frame(&mut usb, &frame);
                        let _ = host.on_response(&frame);
                        let _ = host.on_notify(&frame);
                    }
                }
                Slip::Pending => {}
                Slip::Resync => {}
            }
        }
    }
}

fn print_frame<W: core::fmt::Write>(tx: &mut W, frame: &Frame) -> core::fmt::Result {
    let kind = match frame.kind {
        FrameType::Request => "req",
        FrameType::Response => "rsp",
        FrameType::Notify => "ntf",
    };
    writeln!(
        tx,
        "{} v{} id=0x{:04x} sn={} len={}",
        kind,
        frame.version,
        frame.id,
        frame.sn,
        frame.payload.len()
    )
}
