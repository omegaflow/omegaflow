use omegaflow::opendap::{DapNote, DapType, decode, parse_das, parse_dds};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: opendap_reader <file.dds> [--das <file.das>] [--dods <file.dods>]");
        eprintln!("       opendap_reader --url <base_url>");
        return;
    }

    let (dds_text, attrs_text, dods_bytes) = if args[0] == "--url" {
        let base = args.get(1).map(|s| s.as_str()).unwrap_or("");
        let dds = fetch(&format!("{}.dds", base.trim_end_matches(['.', '/'])));
        let attr_bytes = fetch(&format!("{}.das", base.trim_end_matches(['.', '/'])));
        let dods = fetch(&format!("{}.dods", base.trim_end_matches(['.', '/'])));
        let dds_text = match dds {
            Some(b) => String::from_utf8_lossy(&b).into_owned(),
            None => {
                println!("dds fetch without answer: {}.dds", base);
                return;
            }
        };
        let attrs_text = match attr_bytes {
            Some(b) => String::from_utf8_lossy(&b).into_owned(),
            None => {
                println!("attribute fetch without answer: {}.das", base);
                return;
            }
        };
        (dds_text, attrs_text, dods)
    } else {
        let dds = read_file(&args[0]);
        let attr_bytes = match arg_value(&args, "--das") {
            Some(p) => read_file(&p),
            None => None,
        };
        let dods = match arg_value(&args, "--dods") {
            Some(p) => read_file(&p),
            None => None,
        };
        let dds_text = match dds {
            Some(b) => String::from_utf8_lossy(&b).into_owned(),
            None => {
                println!("dds file does not open: {}", args[0]);
                return;
            }
        };
        let attrs_text = match attr_bytes {
            Some(b) => String::from_utf8_lossy(&b).into_owned(),
            None => {
                println!("dds parsed without a --das attribute file");
                String::new()
            }
        };
        (dds_text, attrs_text, dods)
    };

    let schema = match parse_dds(&dds_text) {
        Ok(s) => s,
        Err(note) => {
            println!("{}", note_text(&note));
            return;
        }
    };

    print_schema(&schema);

    let attrs = parse_das(&attrs_text);

    match &dods_bytes {
        Some(bytes) => match decode(&dds_text, &attrs_text, bytes) {
            Ok(file) => data_vars(&file),
            Err(note) => println!("{}", note_text(&note)),
        },
        None => {}
    }

    match attrs {
        Ok(a) => print_attrs(&a),
        Err(note) => {
            if !attrs_text.is_empty() {
                println!("attrs: {}", note_text(&note));
            }
        }
    }
}

fn read_file(path: &str) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
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

fn dap_type_name(t: &DapType) -> &'static str {
    match t {
        DapType::Byte => "Byte",
        DapType::Int16 => "Int16",
        DapType::UInt16 => "UInt16",
        DapType::Int32 => "Int32",
        DapType::UInt32 => "UInt32",
        DapType::Float32 => "Float32",
        DapType::Float64 => "Float64",
        DapType::Str => "String",
        DapType::Url => "Url",
    }
}

fn print_schema(schema: &omegaflow::opendap::DapSchema) {
    println!("dataset: {}", schema.name);
    println!("dimensions:");
    for d in &schema.dims {
        println!("  {} = {}", d.name, d.len);
    }
    println!("variables:");
    for v in &schema.vars {
        let shape = if v.dims.is_empty() {
            "scalar".to_string()
        } else {
            v.dims.join(",")
        };
        println!("  {}: {} ({})", v.name, dap_type_name(&v.dap_type), shape);
    }
}

fn print_attrs(attrs: &omegaflow::opendap::DapAttrs) {
    println!("global attributes:");
    for a in &attrs.global {
        println!("  {}: {}", a.name, attr_value(a));
    }
    for (var, list) in &attrs.per_var {
        println!("{} attributes:", var);
        for a in list {
            println!("  {}: {}", a.name, attr_value(a));
        }
    }
}

fn attr_value(a: &omegaflow::opendap::DapAttr) -> String {
    match a.dap_type {
        DapType::Str | DapType::Url => String::from_utf8_lossy(&a.raw)
            .trim_matches('"')
            .to_string(),
        _ => {
            let mut nums = Vec::new();
            for c in a.raw.chunks_exact(8) {
                let mut b = [0u8; 8];
                b.copy_from_slice(c);
                nums.push(f64::from_be_bytes(b).to_string());
            }
            nums.join(", ")
        }
    }
}

fn data_vars(file: &omegaflow::opendap::DapFile) {
    println!("decoded variables:");
    for v in &file.vars {
        let shape = file
            .var_shape(v)
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("  {}: {} ({})", v.name, dap_type_name(&v.dap_type), shape);
    }
}

fn note_text(note: &DapNote) -> String {
    match note {
        DapNote::Keyword { word } => format!("keyword {} unread", word),
        DapNote::Shape { var } => format!("shape of {} points into the void", var),
        DapNote::Attr { name } => format!("attribute {} unread", name),
        DapNote::EndAtByte { off } => format!("text ends at byte {}", off),
        DapNote::CountMismatch { var, want, got } => {
            format!("{} declares {} values, carries {}", var, want, got)
        }
        DapNote::Sequence { name } => match name {
            Some(n) => format!("sequence {} unread", n),
            None => "sequence without a name unread".to_string(),
        },
    }
}
