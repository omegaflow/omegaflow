use std::io::{BufRead, BufReader, Read, Write};
use std::thread;
use std::time::Duration;

fn input_ports() -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/dev") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("ttyACM") || name.starts_with("ttyUSB") {
                out.push(format!("/dev/{name}"));
            }
        }
    }
    if let Ok(path) = std::env::var("OMEGAFLOW_SERIAL_IN") {
        if !path.is_empty() && !out.iter().any(|p| p == &path) {
            out.push(path);
        }
    }
    out
}

fn open(path: &str) -> Option<Box<dyn serialport::SerialPort>> {
    match serialport::new(path, 115_200)
        .timeout(Duration::from_millis(50))
        .open()
    {
        Ok(port) => Some(port),
        Err(err) => {
            eprintln!("serial_bridge: {path} unreachable: {err}");
            None
        }
    }
}

fn main() {
    let out_path = std::env::var("OMEGAFLOW_SERIAL_OUT")
        .ok()
        .filter(|path| !path.is_empty());
    if let Some(path) = out_path {
        thread::spawn(move || {
            let Some(mut port) = open(&path) else {
                return;
            };
            let stdin = std::io::stdin();
            let mut input = stdin.lock();
            let mut buf = [0u8; 256];
            loop {
                match input.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if port.write_all(&buf[..n]).is_err() {
                            break;
                        }
                        let _ = port.flush();
                    }
                    Err(_) => thread::sleep(Duration::from_secs(1)),
                }
            }
        });
    }

    let ports = input_ports();
    if ports.is_empty() {
        eprintln!("serial_bridge: no ttyACM/ttyUSB and OMEGAFLOW_SERIAL_IN unset — silence");
    }
    let mut handles = Vec::new();
    for path in ports {
        handles.push(thread::spawn(move || {
            let Some(port) = open(&path) else {
                return;
            };
            let mut reader = BufReader::new(port);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => thread::sleep(Duration::from_secs(1)),
                    Ok(_) => {
                        if line.trim().is_empty() {
                            continue;
                        }
                        let stdout = std::io::stdout();
                        let mut out = stdout.lock();
                        if writeln!(out, "{}", line.trim_end()).is_err() {
                            break;
                        }
                        let _ = out.flush();
                    }
                    Err(_) => thread::sleep(Duration::from_secs(1)),
                }
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
