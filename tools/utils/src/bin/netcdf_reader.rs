use omegaflow::inflate::gunzip;
use omegaflow::netcdf::{NetcdfFile, NetcdfNote, NetcdfType};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!(
            "usage: netcdf_reader <file.nc> [--var <name> [--start a,b,c] [--count x,y,z] [--out <path>]]"
        );
        eprintln!("       netcdf_reader --url <url> [--out <path>]");
        return;
    }

    let bytes: Vec<u8> = if args[0] == "--url" {
        let url = args.get(1).map(|s| s.as_str()).unwrap_or("");
        match fetch(url) {
            Some(b) => b,
            None => {
                println!("curl fetch without answer: {}", url);
                return;
            }
        }
    } else {
        let path = &args[0];
        match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => {
                println!("file does not open: {}", path);
                return;
            }
        }
    };

    let bytes = if bytes.starts_with(&[0x1f, 0x8b]) {
        match gunzip(&bytes) {
            Some(b) => b,
            None => {
                println!("gzip stream stays unreadable");
                return;
            }
        }
    } else {
        bytes
    };

    let out_path = arg_value(&args, "--out");
    if let Some(p) = &out_path {
        if let Err(_) = std::fs::write(p, &bytes) {
            println!("output file does not write: {}", p);
            return;
        }
    }

    let nc = match NetcdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            println!("{}", note_text(&note));
            return;
        }
    };

    match arg_value(&args, "--var") {
        Some(var) => {
            let start = list_arg(&args, "--start");
            let count = list_arg(&args, "--count");
            extract_var(&nc, &bytes, &var, &start, &count);
        }
        None => {
            if args[0] != "--url" {
                struktur(&nc);
            } else if arg_value(&args, "--out").is_none() {
                struktur(&nc);
            }
        }
    }
}

fn fetch(url: &str) -> Option<Vec<u8>> {
    let out = Command::new("curl").arg("-sSf").arg(url).output().ok()?;
    if out.status.success() {
        Some(out.stdout)
    } else {
        None
    }
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn list_arg(args: &[String], flag: &str) -> Vec<usize> {
    arg_value(args, flag)
        .map(|s| {
            s.split(',')
                .filter_map(|t| t.trim().parse::<usize>().ok())
                .collect()
        })
        .unwrap_or_default()
}

fn format_name(f: &NetcdfFile) -> &'static str {
    match f.format {
        omegaflow::netcdf::NetcdfFormat::Cdf1 => "CDF-1",
        omegaflow::netcdf::NetcdfFormat::Cdf2 => "CDF-2",
    }
}

fn struktur(f: &NetcdfFile) {
    println!("Format: {}", format_name(f));
    match f.numrecs {
        Some(n) => println!("numrecs: {}", n),
        None => println!("numrecs: open (STREAMING)"),
    }
    println!("Dimensionen:");
    for (i, d) in f.dims.iter().enumerate() {
        if d.len == 0 {
            println!("  [{}] {} = record", i, d.name);
        } else {
            println!("  [{}] {} = {}", i, d.name, d.len);
        }
    }
    println!("Globale Attribute: {}", f.gattrs.len());
    for a in &f.gattrs {
        println!(
            "  {}: {} [{}]",
            a.name,
            a.nc_type.name(),
            a.raw.len() / a.nc_type.size().max(1)
        );
    }
    println!("Variablen:");
    for v in &f.vars {
        let shape: Vec<String> = v
            .dim_ids
            .iter()
            .map(|&id| {
                f.dims
                    .get(id)
                    .map(|d| d.name.clone())
                    .unwrap_or_else(|| id.to_string())
            })
            .collect();
        let record = if f.record_var(v) { " (record)" } else { "" };
        println!(
            "  {}: {} ({}) begin={} vsize={}{}",
            v.name,
            v.nc_type.name(),
            shape.join(","),
            v.begin,
            v.vsize,
            record
        );
    }
}

fn extract_var(f: &NetcdfFile, file: &[u8], name: &str, start: &[usize], count: &[usize]) {
    let idx = match f.var_index(name) {
        Some(i) => i,
        None => {
            println!("variable absent: {}", name);
            return;
        }
    };
    let v = &f.vars[idx];
    let rank = v.dim_ids.len();
    let start = if start.is_empty() {
        vec![0usize; rank]
    } else if start.len() == rank {
        start.to_vec()
    } else {
        println!("--start needs {} entries, has {}", rank, start.len());
        return;
    };
    let count = if count.is_empty() {
        match f.var_shape(v) {
            Ok(shape) => shape.into_iter().map(|s| s as usize).collect(),
            Err(note) => {
                println!("{}", note_text(&note));
                return;
            }
        }
    } else if count.len() == rank {
        count.to_vec()
    } else {
        println!("--count needs {} entries, has {}", rank, count.len());
        return;
    };

    let raw = match f.read_var_index(file, idx, &start, &count) {
        Ok(b) => b,
        Err(note) => {
            println!("{}", note_text(&note));
            return;
        }
    };

    match v.nc_type {
        NetcdfType::Char => {
            let s = String::from_utf8_lossy(&raw).into_owned();
            println!("{}", s.trim_end_matches('\0'));
        }
        NetcdfType::Byte => {
            for b in raw {
                println!("{}", b as i8);
            }
        }
        NetcdfType::Short => {
            for c in raw.chunks_exact(2) {
                println!("{}", i16::from_be_bytes([c[0], c[1]]));
            }
        }
        NetcdfType::Int => {
            for c in raw.chunks_exact(4) {
                println!("{}", i32::from_be_bytes([c[0], c[1], c[2], c[3]]));
            }
        }
        NetcdfType::Float => {
            for c in raw.chunks_exact(4) {
                println!(
                    "{}",
                    f32::from_bits(u32::from_be_bytes([c[0], c[1], c[2], c[3]]))
                );
            }
        }
        NetcdfType::Double => {
            for c in raw.chunks_exact(8) {
                println!(
                    "{}",
                    f64::from_bits(u64::from_be_bytes([
                        c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]
                    ]))
                );
            }
        }
    }
}

fn note_text(note: &NetcdfNote) -> String {
    match note {
        NetcdfNote::Magic { bytes } => format!(
            "magic is {:02X} {:02X} {:02X} {:02X} — not CDF",
            bytes[0], bytes[1], bytes[2], bytes[3]
        ),
        NetcdfNote::Cdf5 => "format is CDF-5 — pending, its own atom".to_string(),
        NetcdfNote::EndAtByte { off } => format!("file ends at byte {}", off),
        NetcdfNote::Type { tag, off } => format!("nc_type {} at byte {}", tag, off),
        NetcdfNote::Tag { tag, off } => format!("list tag 0x{:02X} at byte {}", tag, off),
        NetcdfNote::DimId { id } => format!("dimension id {} points into the void", id),
        NetcdfNote::CountOpen => "record count open (STREAMING)".to_string(),
        NetcdfNote::AbsentVariable { name } => format!("variable absent: {}", name),
        NetcdfNote::Slab { var } => format!("slab lies outside the shape of {}", var),
    }
}
