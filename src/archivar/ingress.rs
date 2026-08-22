use super::*;

pub fn serial_ports() -> Vec<String> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir("/dev") else {
        return out;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("ttyACM") || name.starts_with("ttyUSB") {
            out.push(format!("/dev/{}", name));
        }
    }
    out
}

pub fn serial_ingress(tx: mpsc::Sender<Vec<(String, f64, f64)>>) {
    loop {
        for name in serial_ports() {
            let mut port = match serialport::new(&name, 115_200)
                .timeout(std::time::Duration::from_millis(50))
                .open()
            {
                Ok(p) => p,
                Err(_) => continue,
            };
            let mut line = String::new();
            let mut buf = [0u8; 256];
            let mut batch: Vec<(String, f64, f64)> = Vec::new();
            loop {
                let n = match port.read(&mut buf) {
                    Ok(n) if n > 0 => n,
                    _ => break,
                };
                for b in &buf[..n] {
                    if *b == b'\n' {
                        if let Some((k, v)) = line.split_once('=') {
                            if let Ok(val) = v.trim().parse::<f64>() {
                                batch.push((k.trim().to_string(), val, 0.0));
                            }
                        }
                        line.clear();
                        if batch.len() >= 64 {
                            let _ = tx.send(batch);
                            batch = Vec::new();
                        }
                    } else {
                        line.push(*b as char);
                        line.clear();
                    }
                }
            }
            if !batch.is_empty() {
                let _ = tx.send(batch);
            }
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

pub fn battery_ingress(tx: mpsc::Sender<Vec<(String, f64, f64)>>) {
    loop {
        let mut batch: Vec<(String, f64, f64)> = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let path = entry.path();
                let read_num = |name: &str| -> Option<f64> {
                    std::fs::read_to_string(path.join(name))
                        .ok()
                        .and_then(|s| s.trim().parse::<f64>().ok())
                };
                let capacity = read_num("capacity");
                let voltage = read_num("voltage_now").map(|v| v / 1e6);
                let current = read_num("current_now").map(|a| a / 1e6);
                let status = std::fs::read_to_string(path.join("status")).unwrap_or_default();
                if let Some(c) = capacity {
                    batch.push(("battery.level".to_string(), c, 60.0));
                }
                if let Some(v) = voltage {
                    batch.push(("battery.voltage".to_string(), v, 60.0));
                }
                if let Some(a) = current {
                    batch.push(("battery.current".to_string(), a, 10.0));
                }
                if status.trim() == "Charging" {
                    batch.push(("battery.charging".to_string(), 1.0, 60.0));
                }
            }
        }
        if !batch.is_empty() {
            let _ = tx.send(batch);
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}
