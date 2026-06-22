use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { eprintln!("compiler <input.is> <output_base>"); return; }
    let input = &args[1];
    let base = &args[2];
    let data = fs::read(input).unwrap_or_default();
    compile_raw(&data, base);
}

fn compile_raw(data: &[u8], base: &str) {
    let reader = BufReader::new(data);
    let mut idx = Vec::new();
    let mut dat = Vec::new();
    let mut entry_count: u32 = 0;

    let mut keys_order: Vec<String> = Vec::new();
    let mut record_fields: Vec<(String, u8)> = Vec::new();

    for line in reader.lines() {
        let line = match line { Ok(l) => l, Err(_) => continue };
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }

        let t: f64 = parts[0].parse().unwrap_or(0.0);
        let x: f64 = parts[1].parse().unwrap_or(0.0);
        let y: f64 = parts[2].parse().unwrap_or(0.0);
        let z: f64 = parts[3].parse().unwrap_or(0.0);
        let key = parts[4];
        let val: f64 = parts[5].parse().unwrap_or(0.0);

        if !keys_order.iter().any(|k| k == key) {
            keys_order.push(key.to_string());
            record_fields.push((key.to_string(), 0u8));
        }

        let offset = dat.len() as u64;
        dat.extend_from_slice(&t.to_le_bytes());
        dat.extend_from_slice(&x.to_le_bytes());
        dat.extend_from_slice(&y.to_le_bytes());
        dat.extend_from_slice(&z.to_le_bytes());
        for (_, _) in &record_fields {
            if record_fields.iter().any(|(k, _)| k == key) {
                dat.extend_from_slice(&val.to_le_bytes());
            } else {
                dat.extend_from_slice(&0.0f64.to_le_bytes());
            }
        }

        idx.extend_from_slice(&t.to_le_bytes());
        idx.extend_from_slice(&x.to_le_bytes());
        idx.extend_from_slice(&y.to_le_bytes());
        idx.extend_from_slice(&z.to_le_bytes());
        idx.extend_from_slice(&offset.to_le_bytes());
        entry_count += 1;
    }

    let mut idx_file = Vec::new();
    idx_file.extend_from_slice(&entry_count.to_le_bytes());
    idx_file.extend_from_slice(&(record_fields.len() as u32).to_le_bytes());
    for (name, _) in &record_fields {
        idx_file.push(name.len() as u8);
        idx_file.extend_from_slice(name.as_bytes());
    }
    let rec_size = (32 + record_fields.len() * 8) as u32;
    idx_file.extend_from_slice(&rec_size.to_le_bytes());
    idx_file.extend_from_slice(&idx);

    fs::write(format!("{}.idx", base), &idx_file).unwrap_or_default();
    fs::write(format!("{}.dat", base), &dat).unwrap_or_default();
    eprintln!("raw: {} entries, {} keys, idx={} bytes, dat={} bytes", entry_count, record_fields.len(), idx_file.len(), dat.len());
}
