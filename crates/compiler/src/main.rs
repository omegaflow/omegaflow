use std::env;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn fetch(url: &str) -> Option<String> {
    let o = Command::new("curl").arg("-s").arg("-m").arg("60").arg("--connect-timeout").arg("10").arg(url).output().ok()?;
    if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).to_string()) } else { None }
}

fn text_vector(text: &str) -> Option<(f64,f64,f64)> {
    let unescaped = text.replace("\\n", "\n");
    let mut last = None;
    for line in unescaped.lines() {
        if let (Some(xp), Some(yp), Some(zp)) = (line.find("X ="), line.find("Y ="), line.find("Z =")) {
            if xp < yp && yp < zp {
                let xs = line[xp+3..yp].trim();
                let ys = line[yp+3..zp].trim();
                let zs = line[zp+3..].trim();
                if let (Ok(xv), Ok(yv), Ok(zv)) = (xs.parse::<f64>(), ys.parse::<f64>(), zs.parse::<f64>()) {
                    last = Some((xv, yv, zv));
                }
            }
        }
    }
    last
}

fn fetch_horizons(cmd: &str) -> Option<(f64,f64,f64)> {
    let url = format!("https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27{}%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%272026-06-22%27&STOP_TIME=%272026-06-23%27&STEP_SIZE=%271%20d%27", cmd);
    let body = fetch(&url)?;
    let p = "\"result\":\"";
    let pos = body.find(p)?;
    let rest = &body[pos + p.len()..];
    text_vector(rest)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "asteroids" {
        let bodies = [
            ("Ceres", "2000001"), ("Vesta", "2000004"), ("Pallas", "2000002"),
            ("Hygiea", "2000010"), ("Eunomia", "2000015"), ("Psyche", "2000016"),
            ("Juno", "2000003"), ("Iris", "2000007"), ("Hebe", "2000006"),
            ("Eros", "2000433"), ("Halley", "1P"), ("Encke", "2P"),
        ];
        let mut out = String::new();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs_f64();
        for (name, cmd) in &bodies {
            std::thread::sleep(std::time::Duration::from_secs(1));
            eprint!("{}... ", name);
            if let Some((x,y,z)) = fetch_horizons(cmd) {
                out.push_str(&format!("{} {} {} {} {} {}\n", now, x*1000.0, y*1000.0, z*1000.0, name, 0.0));
                eprintln!("OK");
            } else { eprintln!("FAIL"); }
        }
        fs::write("is/asteroids.is", &out).unwrap_or_default();
        eprintln!("wrote is/asteroids.is");
        compile_raw(out.as_bytes(), "is/asteroids");
        return;
    }

    if args.len() < 3 { eprintln!("compiler <input.is> <output_base>  |  compiler asteroids"); return; }
    let input = &args[1]; let base = &args[2];
    let data = fs::read(input).unwrap_or_default();
    compile_raw(&data, base);
}

fn compile_raw(data: &[u8], base: &str) {
    let text = String::from_utf8_lossy(data);
    let mut idx = Vec::new(); let mut dat = Vec::new(); let mut entry_count: u32 = 0;
    let mut keys_order: Vec<String> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }
        let t: f64 = parts[0].parse().unwrap_or(0.0);
        let x: f64 = parts[1].parse().unwrap_or(0.0);
        let y: f64 = parts[2].parse().unwrap_or(0.0);
        let z: f64 = parts[3].parse().unwrap_or(0.0);
        let key = parts[4]; let val: f64 = parts[5].parse().unwrap_or(0.0);
        if !keys_order.iter().any(|k| k == key) { keys_order.push(key.to_string()); }
        let offset = dat.len() as u64;
        dat.extend_from_slice(&t.to_le_bytes()); dat.extend_from_slice(&x.to_le_bytes()); dat.extend_from_slice(&y.to_le_bytes()); dat.extend_from_slice(&z.to_le_bytes());
        for _ in &keys_order { dat.extend_from_slice(&val.to_le_bytes()); }
        idx.extend_from_slice(&t.to_le_bytes()); idx.extend_from_slice(&x.to_le_bytes()); idx.extend_from_slice(&y.to_le_bytes()); idx.extend_from_slice(&z.to_le_bytes()); idx.extend_from_slice(&offset.to_le_bytes());
        entry_count += 1;
    }

    let mut idx_file = Vec::new();
    idx_file.extend_from_slice(&entry_count.to_le_bytes());
    idx_file.extend_from_slice(&(keys_order.len() as u32).to_le_bytes());
    for name in &keys_order { idx_file.push(name.len() as u8); idx_file.extend_from_slice(name.as_bytes()); }
    let rec_size = (32 + keys_order.len()*8) as u32;
    idx_file.extend_from_slice(&rec_size.to_le_bytes());
    idx_file.extend_from_slice(&idx);
    fs::write(format!("{}.idx", base), &idx_file).unwrap_or_default();
    fs::write(format!("{}.dat", base), &dat).unwrap_or_default();
    eprintln!("raw: {} entries, {} keys", entry_count, keys_order.len());
}
