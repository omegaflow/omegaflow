use omegaflow::grib2::{Grib2Message, Grib2Note};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: grib2_reader <file.grib2>");
        eprintln!("       grib2_reader --url <url>");
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

    let messages = match Grib2Message::parse_all(&bytes) {
        Ok(m) => m,
        Err(note) => {
            println!("{}", note_text(&note));
            return;
        }
    };

    for (i, m) in messages.iter().enumerate() {
        if messages.len() > 1 {
            println!("message {}", i);
        }
        struktur(m);
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

fn struktur(m: &Grib2Message) {
    println!(
        "GRIB-2 edition {}, {} B, {} sections",
        m.indicator.edition,
        m.indicator.total_length,
        m.sections.len()
    );
    for s in &m.sections {
        if s.number == 8 {
            println!("  [8] terminator at {}", s.offset);
        } else {
            println!("  [{}] at {} len {}", s.number, s.offset, s.length);
        }
    }
    match &m.identification {
        Some(id) => {
            println!("Identification:");
            println!("  centre {} subcentre {}", id.centre, id.subcentre);
            println!(
                "  tables {} local tables {}",
                id.tables_version, id.local_tables_version
            );
            println!("  reference time significance {}", id.ref_time_significance);
            println!(
                "  reference time {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                id.year, id.month, id.day, id.hour, id.minute, id.second
            );
            println!(
                "  production status {} processed data {}",
                id.production_status, id.processed_data_type
            );
        }
        None => println!("Identification: absent"),
    }
    match m.local_use_length {
        Some(len) => println!("Local use: {} B", len),
        None => println!("Local use: absent"),
    }
    match &m.grid {
        Some(g) => {
            println!("Grid definition:");
            println!("  source {}", g.source);
            println!("  data points {}", g.num_points);
            println!(
                "  list octets {} interpretation {} list {} B",
                g.list_octets,
                g.list_interpretation,
                g.list.len()
            );
            println!(
                "  template {} template bytes {}",
                g.template_number,
                g.template.len()
            );
        }
        None => println!("Grid definition: absent"),
    }
    match &m.product {
        Some(p) => {
            println!("Product definition:");
            println!(
                "  discipline {} category {} number {}",
                m.indicator.discipline,
                option_u8(p.parameter_category),
                option_u8(p.parameter_number)
            );
            println!("  coordinate values {}", p.num_coord_values);
            println!(
                "  template {} template bytes {}",
                p.template_number,
                p.template.len()
            );
        }
        None => println!("Product definition: absent"),
    }
    match &m.representation {
        Some(r) => {
            println!("Data representation:");
            println!("  data points {}", r.num_points);
            match r.packing_name() {
                Some(name) => println!("  template {} ({})", r.template_number, name),
                None => println!("  template {} (unmapped)", r.template_number),
            }
            println!("  template bytes {}", r.template.len());
        }
        None => println!("Data representation: absent"),
    }
    match &m.bitmap {
        Some(b) => println!("Bit-map: {}", bitmap_text(b.indicator)),
        None => println!("Bit-map: absent"),
    }
    match m.data_length {
        Some(len) => println!("Data: {} B", len),
        None => println!("Data: absent"),
    }
}

fn option_u8(v: Option<u8>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "absent".to_string(),
    }
}

fn bitmap_text(indicator: u8) -> String {
    match indicator {
        255 => "implicit (no bit-map)".to_string(),
        0 => "explicit (bit-map follows)".to_string(),
        254 => "previously defined".to_string(),
        n => format!("indicator {}", n),
    }
}

fn note_text(note: &Grib2Note) -> String {
    match note {
        Grib2Note::Magic { bytes } => format!(
            "magic is {:02X} {:02X} {:02X} {:02X} — not GRIB",
            bytes[0], bytes[1], bytes[2], bytes[3]
        ),
        Grib2Note::Edition { edition } => format!("edition {} — not 2", edition),
        Grib2Note::Length { total, bytes } => {
            format!("message length {} exceeds the {} bytes present", total, bytes)
        }
        Grib2Note::SectionLength { number, off } => {
            format!("section {} at byte {} overruns the message", number, off)
        }
        Grib2Note::Terminator { off } => format!("terminator 7777 absent at byte {}", off),
        Grib2Note::EndAtByte { off } => format!("file ends at byte {}", off),
    }
}
