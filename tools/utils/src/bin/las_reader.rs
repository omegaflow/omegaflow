use omegaflow::las::{
    LasHeader, LasNote, LazDecoder, copc_hierarchy, copc_info, ept_json, has_laszip_vlr,
    point_format_name,
};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: las_reader <file.las|laz|copc.laz> [--dump N]");
        eprintln!("       las_reader --url <url> [--dump N]");
        eprintln!("       las_reader --ept <ept.json>");
        return;
    }

    if args[0] == "--ept" {
        match args.get(1).map(|s| s.as_str()) {
            Some(path) => read_ept(path),
            None => {
                println!("--ept needs a path to ept.json");
                return;
            }
        }
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
        match std::fs::read(&args[0]) {
            Ok(b) => b,
            Err(_) => {
                println!("file does not open: {}", args[0]);
                return;
            }
        }
    };

    let header = match LasHeader::parse(&bytes) {
        Ok(h) => h,
        Err(note) => {
            println!("{}", note_text(&note));
            return;
        }
    };

    struktur(&header);

    let vlrs = match header.vlrs(&bytes) {
        Ok(v) => v,
        Err(note) => {
            println!("{}", note_text(&note));
            return;
        }
    };
    for v in &vlrs {
        println!(
            "VLR user_id={} record_id={} {} B",
            v.user_id,
            v.record_id,
            v.payload.len()
        );
    }

    if let Some(info) = copc_info(&vlrs) {
        print_copc(&bytes, &info);
    }

    match arg_value(&args, "--dump").and_then(|s| s.parse::<u64>().ok()) {
        Some(n) => dump_points(&header, &bytes, n),
        None => {}
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

fn struktur(h: &LasHeader) {
    println!(
        "LAS {}.{} point format {} ({})",
        h.version_major,
        h.version_minor,
        h.point_format,
        point_format_name(h.point_format)
    );
    println!(
        "points {} record length {} B",
        h.point_count, h.point_length
    );
    println!(
        "system \"{}\" by \"{}\"",
        h.system_identifier, h.generating_software
    );
    println!(
        "scale xyz {:.6} {:.6} {:.6}",
        h.scale[0], h.scale[1], h.scale[2]
    );
    println!(
        "offset xyz {:.3} {:.3} {:.3}",
        h.offset[0], h.offset[1], h.offset[2]
    );
    println!(
        "bounds x {:.3}..{:.3} y {:.3}..{:.3} z {:.3}..{:.3}",
        h.min[0], h.max[0], h.min[1], h.max[1], h.min[2], h.max[2]
    );
    println!("vlrs {}", h.num_vlrs);
}

fn print_copc(bytes: &[u8], info: &omegaflow::las::CopcInfo) {
    println!(
        "COPC center {:.3} {:.3} {:.3} halfsize {:.3} spacing {:.3}",
        info.center[0], info.center[1], info.center[2], info.halfsize, info.spacing
    );
    println!(
        "COPC gpstime {:.6}..{:.6}",
        info.gpstime_min, info.gpstime_max
    );
    match copc_hierarchy(bytes, info) {
        Some(entries) => {
            let mut points = 0u64;
            for e in &entries {
                if e.point_count > 0 {
                    points += e.point_count as u64;
                }
            }
            println!("COPC hierarchy {} nodes {} points", entries.len(), points);
            for e in entries.iter().take(16) {
                println!(
                    "  {}-{}-{}-{} off {} size {} points {}",
                    e.level, e.x, e.y, e.z, e.offset, e.byte_size, e.point_count
                );
            }
            if entries.len() > 16 {
                println!("  ... {} more nodes", entries.len() - 16);
            }
        }
        None => println!("COPC hierarchy stays unreadable"),
    }
}

fn dump_points(h: &LasHeader, bytes: &[u8], n: u64) {
    let count = h.point_count.min(n);
    let laz = h.vlrs(bytes).map(|v| has_laszip_vlr(&v)).unwrap_or(false);
    if laz {
        let mut dec = match LazDecoder::new(h, bytes) {
            Ok(d) => d,
            Err(note) => {
                println!("{}", note_text(&note));
                return;
            }
        };
        for i in 0..count {
            match dec.point_at(i) {
                Ok(p) => println!(
                    "{} {:.3} {:.3} {:.3} i={} cls={} rn={}/{}",
                    i,
                    p.x,
                    p.y,
                    p.z,
                    p.intensity,
                    p.classification,
                    p.return_number,
                    p.number_of_returns
                ),
                Err(note) => {
                    println!("{}", note_text(&note));
                    return;
                }
            }
        }
        return;
    }
    for i in 0..count {
        match h.point_at(bytes, i) {
            Ok(p) => println!(
                "{} {:.3} {:.3} {:.3} i={} cls={} rn={}/{}",
                i,
                p.x,
                p.y,
                p.z,
                p.intensity,
                p.classification,
                p.return_number,
                p.number_of_returns
            ),
            Err(note) => {
                println!("{}", note_text(&note));
                return;
            }
        }
    }
}

fn read_ept(path: &str) {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => {
            println!("file does not open: {}", path);
            return;
        }
    };
    match ept_json(&text) {
        Some(layout) => {
            println!(
                "EPT span {} points {} dataType {} hierarchyType {}",
                layout.span, layout.points, layout.data_type, layout.hierarchy_type
            );
            println!(
                "bounds x {:.3}..{:.3} y {:.3}..{:.3} z {:.3}..{:.3}",
                layout.bounds[0],
                layout.bounds[3],
                layout.bounds[1],
                layout.bounds[4],
                layout.bounds[2],
                layout.bounds[5]
            );
            for f in &layout.schema {
                println!(
                    "  {} {} size {} scale {} offset {}",
                    f.name, f.field_type, f.size, f.scale, f.offset
                );
            }
        }
        None => println!("ept.json stays unreadable"),
    }
}

fn note_text(note: &LasNote) -> String {
    match note {
        LasNote::Magic { bytes } => format!(
            "magic is {:02X} {:02X} {:02X} {:02X} — not LASF",
            bytes[0], bytes[1], bytes[2], bytes[3]
        ),
        LasNote::EndAtByte { off } => format!("file ends at byte {}", off),
        LasNote::HeaderSize { size } => format!("header size {} overruns the bytes", size),
        LasNote::PointFormat { format } => format!("point format {} — outside 0..=10", format),
        LasNote::PointLength { format, length } => format!(
            "point format {} record length {} B — shorter than the format needs",
            format, length
        ),
        LasNote::PointDataAt { off } => format!("point data overruns the bytes at {}", off),
        LasNote::LazAbsent => "the file carries no laszip encoded VLR".to_string(),
        LasNote::LazItem { item } => format!("laszip item type {item} stays undecoded"),
        LasNote::LazChunkTable { off } => format!("chunk table absent at byte {off}"),
        LasNote::LazChunkOverrun { off } => format!("chunk overruns at byte {off}"),
        LasNote::LazCoderStall { off } => format!("coder stalls at byte {off}"),
    }
}
