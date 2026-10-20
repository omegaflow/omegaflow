use omegaflow::hdf5::{Hdf5File, Hdf5Layout, Hdf5Note};
use std::process::Command;

fn note_text(note: &Hdf5Note) -> String {
    match note {
        Hdf5Note::Magic { bytes } => format!(
            "magic is {:02X} {:02X} {:02X} {:02X} — not HDF5",
            bytes[0], bytes[1], bytes[2], bytes[3]
        ),
        Hdf5Note::SuperblockVersion { v } => format!("superblock version {} unread", v),
        Hdf5Note::OffsetSize { n } => format!("offset size {} unread (4 or 8 only)", n),
        Hdf5Note::EndAtByte { off } => format!("file ends at byte {}", off),
        Hdf5Note::AbsentAtByte { off } => {
            format!("no bytes at address {} — the source does not carry it", off)
        }
        Hdf5Note::Signatur { off, found } => format!(
            "signature {:02X}{:02X}{:02X}{:02X} at byte {} — not the expected block",
            found[0], found[1], found[2], found[3], off
        ),
        Hdf5Note::Address { off } => format!("address at byte {} points into the void", off),
        Hdf5Note::ObjectHeaderVersion { v } => format!("object header version {} unread", v),
        Hdf5Note::Datatype { class, off } => {
            format!("datatype class {} at byte {} unread", class, off)
        }
        Hdf5Note::Dataspace { off } => format!("dataspace at byte {} unread", off),
        Hdf5Note::Layout { class, off } => format!("layout class {} at byte {} unread", class, off),
        Hdf5Note::Btree { typ, off } => format!("B-tree type {} at byte {} unread", typ, off),
        Hdf5Note::BtreeNode { found, off } => format!(
            "B-tree node {:02X}{:02X}{:02X}{:02X} at byte {} unread",
            found[0], found[1], found[2], found[3], off
        ),
        Hdf5Note::Heap { found, off } => format!(
            "heap {:02X}{:02X}{:02X}{:02X} at byte {} unread",
            found[0], found[1], found[2], found[3], off
        ),
        Hdf5Note::Filter { id, off } => format!("filter {} at byte {} unread", id, off),
        Hdf5Note::Checksum { off } => {
            format!("metadata checksum at byte {} carries a mismatch", off)
        }
        Hdf5Note::HugeHeapObject => "huge fractal-heap object unread".to_string(),
        Hdf5Note::SharedMessage => "shared object header message unread".to_string(),
        Hdf5Note::AbsentObject { name } => format!("object absent: {}", name),
        Hdf5Note::Chunk { off } => format!("chunk at byte {} lies outside the shape", off),
        Hdf5Note::VlenNotRead => "variable-length dataset data stays unread".to_string(),
        Hdf5Note::VirtualDataset => "virtual dataset unread".to_string(),
        Hdf5Note::ReadLength { off, len } => format!(
            "read of {} bytes at byte {} exceeds the per-read cap — the chunk stays unfetched",
            len, off
        ),
        Hdf5Note::FetchBudget { off, reads } => format!(
            "fetch budget exhausted after {} reads at byte {} — the read stops here",
            reads, off
        ),
        Hdf5Note::TraversalBudget { off, bytes } => format!(
            "traversal budget exhausted after {} bytes at byte {} — the walk stops here",
            bytes, off
        ),
    }
}

fn attr_value(a: &omegaflow::hdf5::Hdf5Attribute) -> String {
    use omegaflow::hdf5::Endian;
    let data = &a.data;
    let be = a.datatype.endian == Endian::Be;
    let f64n = |c: &[u8], be: bool| -> Option<String> {
        let arr: [u8; 8] = c.try_into().ok()?;
        let v = if be {
            f64::from_be_bytes(arr)
        } else {
            f64::from_le_bytes(arr)
        };
        Some(v.to_string())
    };
    let f32n = |c: &[u8], be: bool| -> Option<String> {
        let arr: [u8; 4] = c.try_into().ok()?;
        let v = if be {
            f32::from_be_bytes(arr)
        } else {
            f32::from_le_bytes(arr)
        };
        Some(v.to_string())
    };
    let i32n = |c: &[u8], be: bool| -> Option<String> {
        let arr: [u8; 4] = c.try_into().ok()?;
        let v = if be {
            i32::from_be_bytes(arr)
        } else {
            i32::from_le_bytes(arr)
        };
        Some(v.to_string())
    };
    let nums = |data: &[u8], f: &dyn Fn(&[u8], bool) -> Option<String>| {
        data.chunks(8)
            .filter_map(|c| f(c, be))
            .collect::<Vec<String>>()
            .join(", ")
    };
    match a.datatype.class {
        3 => String::from_utf8_lossy(data)
            .trim_matches('\0')
            .trim_end()
            .to_string(),
        1 if a.datatype.size == 8 => nums(data, &f64n),
        1 if a.datatype.size == 4 => nums(data, &f32n),
        0 if a.datatype.signed && a.datatype.size == 4 => nums(data, &i32n),
        _ => format!("class {} {} B", a.datatype.class, data.len()),
    }
}

fn struktur(file: &Hdf5File) {
    let root = match file.root() {
        Ok(r) => r,
        Err(note) => {
            eprintln!("{}", note_text(&note));
            return;
        }
    };
    let walk = |path: String| {
        let obj = match file.resolve(&path) {
            Ok(o) => o,
            Err(note) => {
                eprintln!("{}: {}", path, note_text(&note));
                return;
            }
        };
        if obj.is_group {
            println!("group {}", path);
            for a in &obj.attrs {
                println!("  attr {} = {}", a.name, attr_value(a));
            }
            for l in &obj.links {
                if l.soft.is_some() {
                    println!("  {} -> {} (soft)", l.name, l.soft.as_ref().unwrap());
                } else {
                    println!("  {} @ {}", l.name, l.addr);
                }
            }
        } else if let (Some(ds), Some(dt)) = (&obj.dataspace, &obj.datatype) {
            let dims: Vec<String> = ds.dims.iter().map(|d| d.to_string()).collect();
            println!("dataset {} ({}) [{}]", path, dt.class, dims.join(","));
            match obj.layout {
                Some(Hdf5Layout::Contiguous { addr, size }) => {
                    println!("  layout contiguous at {} size {}", addr, size);
                }
                Some(Hdf5Layout::Chunked { .. }) => println!("  layout chunked"),
                Some(Hdf5Layout::Compact { .. }) => println!("  layout compact"),
                None => {}
            }
            for a in &obj.attrs {
                println!("  attr {} = {}", a.name, attr_value(a));
            }
        } else {
            println!("object {} @ {}", path, obj.addr);
        }
    };
    walk(String::new());
    let mut stack: Vec<String> = Vec::new();
    for l in &root.links {
        stack.push(l.name.clone());
    }
    let mut visited = std::collections::HashSet::new();
    while let Some(name) = stack.pop() {
        if !visited.insert(name.clone()) {
            continue;
        }
        walk(name.clone());
        if let Ok(obj) = file.resolve(&name) {
            for l in &obj.links {
                stack.push(format!("{}/{}", name, l.name));
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(url), Some(off), Some(len)) = (args.first(), args.get(1), args.get(2)) else {
        eprintln!("usage: hdf5_remote_reader <zip-url> <hdf5-offset> <hdf5-length> [--var <name>]");
        return;
    };
    let off: u64 = off.parse().expect("hdf5-offset");
    let len: u64 = len.parse().expect("hdf5-length");
    let base_len = 1_048_576u64.min(len);
    let base = match fetch(url, off, base_len) {
        Some(b) => b,
        None => {
            eprintln!("superblock fetch failed");
            return;
        }
    };
    let tmp = omegaflow::temp_path("hdf5_remote_chunk.bin");
    let mut fetches = 0usize;
    let mut fetched = 0u64;
    let file = {
        let mut fetch = |rel: u64, want: u64| -> Option<Vec<u8>> {
            let start = off + rel;
            let avail = len.saturating_sub(rel);
            let want = want.min(16 * 1024 * 1024);
            if want == 0 {
                return None;
            }
            let real = want.max(4 * 1024 * 1024).min(avail);
            let mut bytes = if real > 0 {
                fetch(url, start, real)?
            } else {
                Vec::new()
            };
            if bytes.len() < want as usize {
                bytes.resize(want as usize, 0);
            }
            fetches += 1;
            fetched += bytes.len() as u64;
            let _ = std::fs::write(tmp, &bytes);
            Some(bytes)
        };
        match Hdf5File::parse_fetch(&base, &mut fetch) {
            Ok(f) => f,
            Err(note) => {
                eprintln!("{}", note_text(&note));
                return;
            }
        }
    };
    eprintln!("fetches {} · bytes {}", fetches, fetched);
    if let Some(i) = args.iter().position(|a| a == "--var") {
        if let Some(var) = args.get(i + 1) {
            match file.dataset(var) {
                Ok((obj, ds, dt)) => {
                    println!(
                        "{}: rank {} dims {:?} class {} size {}",
                        var,
                        ds.dims.len(),
                        ds.dims,
                        dt.class,
                        dt.size
                    );
                    match obj.layout {
                        Some(Hdf5Layout::Contiguous { addr, size }) => {
                            println!("layout: contiguous at {} size {}", addr, size);
                        }
                        Some(Hdf5Layout::Chunked { .. }) => println!("layout: chunked"),
                        Some(Hdf5Layout::Compact { .. }) => println!("layout: compact"),
                        None => println!("layout: absent"),
                    }
                }
                Err(note) => eprintln!("{}", note_text(&note)),
            }
            return;
        }
    }
    struktur(&file);
}

fn fetch(url: &str, start: u64, len: u64) -> Option<Vec<u8>> {
    let end = start.checked_add(len)?.checked_sub(1)?;
    let out = omegaflow::temp_path(&format!("hdf5_range_{}_{}.bin", start, len));
    let status = Command::new("curl")
        .args([
            "-sS",
            "--retry",
            "3",
            "-r",
            &format!("{start}-{end}"),
            url,
            "-o",
            &out,
        ])
        .status()
        .ok()?;
    if !status.success() {
        return None;
    }
    let bytes = std::fs::read(&out).ok()?;
    let _ = std::fs::remove_file(&out);
    if bytes.len() as u64 != len {
        return None;
    }
    Some(bytes)
}
