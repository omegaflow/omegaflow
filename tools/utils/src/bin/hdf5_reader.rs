use omegaflow::hdf5::{Hdf5File, Hdf5Layout, Hdf5Note};

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = args.first() else {
        eprintln!(
            "usage: hdf5_reader <file.h5> [--var <name>] [--range <start[,count]>] [--chunk <c0[,c1,...]>]"
        );
        return;
    };
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("file does not open: {}", path);
            return;
        }
    };
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{}", note_text(&note));
            return;
        }
    };
    match args
        .iter()
        .position(|a| a == "--var")
        .and_then(|i| args.get(i + 1))
    {
        Some(var) => match file.dataset(var) {
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
                    Some(Hdf5Layout::Chunked {
                        btree,
                        ref chunk_dims,
                        elem_size,
                    }) => {
                        println!(
                            "layout: chunked btree {} chunks {:?} elem {}",
                            btree, chunk_dims, elem_size
                        );
                    }
                    Some(Hdf5Layout::Compact { ref data }) => {
                        println!("layout: compact {} B", data.len());
                    }
                    None => println!("layout: absent"),
                }
                if !obj.filters.is_empty() {
                    println!(
                        "filters: {}",
                        obj.filters
                            .iter()
                            .map(|f| f.id.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    );
                }
                let chunk_flag = args
                    .iter()
                    .position(|a| a == "--chunk")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| {
                        s.split(',')
                            .filter_map(|t| t.trim().parse::<u64>().ok())
                            .collect::<Vec<u64>>()
                    });
                let range_flag = args
                    .iter()
                    .position(|a| a == "--range")
                    .and_then(|i| args.get(i + 1))
                    .and_then(|s| {
                        let mut it = s.split(',').filter_map(|t| t.trim().parse::<u64>().ok());
                        let start = it.next()?;
                        let count = it.next().unwrap_or(1);
                        Some((start, count))
                    });
                let chunked = matches!(obj.layout, Some(Hdf5Layout::Chunked { .. }));
                if let (Some(c), true) = (chunk_flag, chunked) {
                    match file.read_chunk(var, &c, |_o: u64, _l: u64| -> Option<Vec<u8>> { None }) {
                        Some(vals) => println!("data: {} values (chunk {:?})", vals.len(), c),
                        None => eprintln!("data: chunk {:?} stays absent", c),
                    }
                } else if let Some((start, count)) = range_flag {
                    match file.read_range(var, start, count) {
                        Ok(vals) => println!(
                            "data: {} values (range {}..{})",
                            vals.len(),
                            start,
                            start + count
                        ),
                        Err(note) => eprintln!("data: {}", note_text(&note)),
                    }
                } else if chunked {
                    let coords = vec![0u64; ds.dims.len()];
                    match file
                        .read_chunk(var, &coords, |_o: u64, _l: u64| -> Option<Vec<u8>> { None })
                    {
                        Some(vals) => println!(
                            "data: {} values (bounded first chunk {:?}; --chunk/--range for a subset)",
                            vals.len(),
                            coords
                        ),
                        None => eprintln!("data: the first chunk stays absent"),
                    }
                } else {
                    match file.read_dataset(var) {
                        Ok(data) => println!("data: {} B read", data.len()),
                        Err(note) => eprintln!("data: {}", note_text(&note)),
                    }
                }
            }
            Err(note) => eprintln!("{}", note_text(&note)),
        },
        None => struktur(&file),
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
