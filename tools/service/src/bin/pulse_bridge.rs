use std::io::Write;

use bluer::gatt::remote::Characteristic;
use bluer::{Address, Device, Session};
use futures::{StreamExt, pin_mut};
use uuid::Uuid;

const WATCH_MAC: &str = "F0:99:19:4E:0B:BF";
const PULSE_LINK: &str = "/tmp/omegaflow-pulse";
const HR_MEASUREMENT_UUID: u128 = 0x0000_2a37_0000_1000_8000_0080_5f9b_34fb;

struct Config {
    address: Address,
    link: String,
    hr_uuid: Uuid,
}

fn config_from_args() -> Result<Config, String> {
    let mut address = WATCH_MAC.to_string();
    let mut link = PULSE_LINK.to_string();
    let mut hr_uuid = Uuid::from_u128(HR_MEASUREMENT_UUID);

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--address" => {
                i += 1;
                address = args
                    .get(i)
                    .cloned()
                    .ok_or_else(|| "--address: value absent".to_string())?;
            }
            "--link" => {
                i += 1;
                link = args
                    .get(i)
                    .cloned()
                    .ok_or_else(|| "--link: value absent".to_string())?;
            }
            "--uuid" => {
                i += 1;
                let raw = args
                    .get(i)
                    .cloned()
                    .ok_or_else(|| "--uuid: value absent".to_string())?;
                hr_uuid = Uuid::try_parse(&raw).map_err(|e| format!("--uuid {raw}: {e}"))?;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        i += 1;
    }

    let address = address
        .parse::<Address>()
        .map_err(|e| format!("--address {address}: {e}"))?;

    Ok(Config {
        address,
        link,
        hr_uuid,
    })
}

fn open_pulse_pty(link: &str) -> Result<std::fs::File, String> {
    let pty = nix::pty::openpty(
        None::<&nix::pty::Winsize>,
        None::<&nix::sys::termios::Termios>,
    )
    .map_err(|e| format!("openpty: {e}"))?;

    let mut termios =
        nix::sys::termios::tcgetattr(&pty.master).map_err(|e| format!("tcgetattr: {e}"))?;
    nix::sys::termios::cfmakeraw(&mut termios);
    termios.control_chars[nix::sys::termios::SpecialCharacterIndices::VMIN as usize] = 1;
    termios.control_chars[nix::sys::termios::SpecialCharacterIndices::VTIME as usize] = 0;
    nix::sys::termios::tcsetattr(&pty.master, nix::sys::termios::SetArg::TCSANOW, &termios)
        .map_err(|e| format!("tcsetattr: {e}"))?;

    let slave_name = nix::unistd::ttyname(&pty.slave).map_err(|e| format!("ttyname: {e}"))?;

    let link_path = std::path::Path::new(link);
    match std::fs::symlink_metadata(link_path) {
        Ok(_) => {
            std::fs::remove_file(link_path).map_err(|e| format!("remove {link}: {e}"))?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("symlink_metadata {link}: {e}")),
    }
    std::os::unix::fs::symlink(&slave_name, link_path)
        .map_err(|e| format!("symlink {link} -> {}: {e}", slave_name.display()))?;

    Ok(std::fs::File::from(pty.master))
}

struct HrSample {
    hr_bpm: Option<u16>,
    rr_ms: Vec<f64>,
}

fn parse_hr_measurement(packet: &[u8]) -> HrSample {
    let mut sample = HrSample {
        hr_bpm: None,
        rr_ms: Vec::new(),
    };
    let Some(&flags) = packet.first() else {
        return sample;
    };
    let hr_is_u16 = flags & 0x01 != 0;
    let has_rr = flags & 0x10 != 0;

    if hr_is_u16 {
        if packet.len() >= 3 {
            sample.hr_bpm = Some(u16::from_le_bytes([packet[1], packet[2]]));
        }
    } else if packet.len() >= 2 {
        sample.hr_bpm = Some(u16::from(packet[1]));
    }

    if has_rr {
        let mut offset = if hr_is_u16 { 3 } else { 2 };
        while offset + 1 < packet.len() {
            let raw = u16::from_le_bytes([packet[offset], packet[offset + 1]]);
            let ms = f64::from(raw) * 1000.0 / 1024.0;
            if ms.is_finite() {
                sample.rr_ms.push(ms);
            }
            offset += 2;
        }
    }

    sample
}

async fn find_hr_characteristic(device: &Device, hr_uuid: Uuid) -> Result<Characteristic, String> {
    let services = device
        .services()
        .await
        .map_err(|_| "services returned void".to_string())?;
    for service in services {
        let chars = service
            .characteristics()
            .await
            .map_err(|_| "characteristics returned void".to_string())?;
        for c in chars {
            let uuid = c
                .uuid()
                .await
                .map_err(|_| "characteristic uuid returned void".to_string())?;
            if uuid == hr_uuid {
                let flags = c
                    .flags()
                    .await
                    .map_err(|_| "characteristic flags returned void".to_string())?;
                if flags.notify || flags.indicate {
                    return Ok(c);
                }
                return Err(format!(
                    "characteristic {hr_uuid} carries no notify or indicate"
                ));
            }
        }
    }
    Err(format!("characteristic {hr_uuid} not found"))
}

fn spawn_pulse_writer(mut master: std::fs::File) -> std::sync::mpsc::Sender<String> {
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    std::thread::spawn(move || {
        for line in rx {
            let _ = master.write_all(line.as_bytes());
        }
    });
    tx
}

async fn bridge_session(
    address: Address,
    hr_uuid: Uuid,
    tx: &std::sync::mpsc::Sender<String>,
) -> Result<(), String> {
    let session = Session::new()
        .await
        .map_err(|_| "bluez session returned void".to_string())?;
    let adapter = session
        .default_adapter()
        .await
        .map_err(|_| "adapter returned void".to_string())?;
    adapter
        .set_powered(true)
        .await
        .map_err(|_| "adapter power returned void".to_string())?;
    let device = adapter
        .device(address)
        .map_err(|_| format!("device {address} returned void"))?;

    if !device
        .is_connected()
        .await
        .map_err(|_| "connect state returned void".to_string())?
    {
        device
            .connect()
            .await
            .map_err(|_| format!("connect to {address} returned void"))?;
    }
    eprintln!("watch {address}: connected");

    let hr_char = find_hr_characteristic(&device, hr_uuid).await?;
    let notify = hr_char
        .notify()
        .await
        .map_err(|_| "notify returned void".to_string())?;
    pin_mut!(notify);

    let mut rr_absent_said = false;
    while let Some(packet) = notify.next().await {
        let sample = parse_hr_measurement(&packet);
        if let Some(bpm) = sample.hr_bpm {
            println!("pulse: {bpm} bpm");
        }
        if sample.rr_ms.is_empty() {
            if !rr_absent_said {
                eprintln!("R-R im Broadcast absent");
                rr_absent_said = true;
            }
        } else {
            rr_absent_said = false;
            for ms in &sample.rr_ms {
                let line = format!("nn={ms:.3}\n");
                let _ = tx.send(line);
            }
        }
    }

    Ok(())
}

async fn run_bridge(address: Address, hr_uuid: Uuid, master: std::fs::File) {
    let tx = spawn_pulse_writer(master);
    loop {
        match bridge_session(address, hr_uuid, &tx).await {
            Ok(()) => eprintln!("watch {address}: notification stream closed"),
            Err(reason) => eprintln!("watch {address}: {reason}"),
        }
        eprintln!("watch {address}: retry in 5 s");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let config = match config_from_args() {
        Ok(c) => c,
        Err(reason) => {
            eprintln!("{reason}");
            std::process::exit(2);
        }
    };

    let master = match open_pulse_pty(&config.link) {
        Ok(m) => m,
        Err(reason) => {
            eprintln!("{reason}");
            std::process::exit(2);
        }
    };

    println!("OMEGAFLOW_SERIAL_IN={} cargo run", config.link);
    println!("Uhr: Herzfrequenz senden anschalten");

    run_bridge(config.address, config.hr_uuid, master).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_hr_without_rr() {
        let sample = parse_hr_measurement(&[0x00, 72]);
        assert_eq!(sample.hr_bpm, Some(72));
        assert!(sample.rr_ms.is_empty());
    }

    #[test]
    fn u16_hr_without_rr() {
        let sample = parse_hr_measurement(&[0x01, 0x2C, 0x01]);
        assert_eq!(sample.hr_bpm, Some(300));
        assert!(sample.rr_ms.is_empty());
    }

    #[test]
    fn rr_present_two_values() {
        let sample = parse_hr_measurement(&[0x10, 60, 0x00, 0x04, 0x00, 0x08]);
        assert_eq!(sample.hr_bpm, Some(60));
        assert_eq!(sample.rr_ms.len(), 2);
        assert!((sample.rr_ms[0] - 1000.0).abs() < 1e-6);
        assert!((sample.rr_ms[1] - 2000.0).abs() < 1e-6);
    }

    #[test]
    fn truncated_packet_rr_absent() {
        let sample = parse_hr_measurement(&[0x11, 0x2C, 0x01, 0x00]);
        assert_eq!(sample.hr_bpm, Some(300));
        assert!(sample.rr_ms.is_empty());
    }
}
