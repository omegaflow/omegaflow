use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::hdf5::Hdf5File;

const MAGIC: &[u8; 4] = b"GWOS";
const FIELDS: usize = 3;
const NETLOC: &str = "gwosc.org";
const COMPILER: &str = "tools/harvest/src/bin/gwosc_compiler.rs";
const ORIGIN: &str = "https://gwosc.org/eventapi/json/GWTC-1-confident/GW150914/v3/H-H1_GWOSC_4KHZ_R1-1126259447-32.hdf5";
const COMP_GWOSC_STRAIN: f64 = 1.0;
const HDF5_MAGIC: [u8; 4] = [0x89, b'H', b'D', b'F'];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn dataset_paths(dataset: &str) -> Vec<String> {
    let mut out = vec![dataset.to_string()];
    if let Some((parent, _)) = dataset.rsplit_once('/') {
        if !parent.is_empty() {
            out.push(parent.to_string());
        }
    }
    out.push("strain".to_string());
    out.push(String::new());
    out
}

fn find_attr(file: &Hdf5File, paths: &[String], attr: &str) -> Option<f64> {
    paths.iter().find_map(|p| file.attr_f64(p, attr))
}

fn attr_int(file: &Hdf5File, name: &str, attr: &str) -> Option<f64> {
    let a = file.attribute(name, attr)?;
    let bytes = a.data.get(0..a.datatype.size)?;
    match (a.datatype.class, a.datatype.size) {
        (0, 8) => {
            let arr: [u8; 8] = bytes.try_into().ok()?;
            Some(if a.datatype.signed {
                i64::from_le_bytes(arr) as f64
            } else {
                u64::from_le_bytes(arr) as f64
            })
        }
        (0, 4) => {
            let arr: [u8; 4] = bytes.try_into().ok()?;
            Some(if a.datatype.signed {
                i32::from_le_bytes(arr) as f64
            } else {
                u32::from_le_bytes(arr) as f64
            })
        }
        _ => None,
    }
}

fn find_attr_int(file: &Hdf5File, paths: &[String], attr: &str) -> Option<f64> {
    paths.iter().find_map(|p| attr_int(file, p, attr))
}

fn scalar_dataset(file: &Hdf5File, name: &str) -> Option<f64> {
    file.read_f64_dataset(name)
        .ok()
        .and_then(|v| v.first().copied())
}

fn discover_strain(file: &Hdf5File) -> Option<String> {
    for candidate in ["strain/Strain", "Strain", "strain/strain"] {
        if file.dataset(candidate).is_ok() {
            return Some(candidate.to_string());
        }
    }
    let roots: Vec<String> = file.links_of("").iter().map(|l| l.name.clone()).collect();
    for name in &roots {
        if name.to_ascii_lowercase().contains("strain") && file.dataset(name).is_ok() {
            return Some(name.clone());
        }
    }
    for name in &roots {
        let children: Vec<String> = file.links_of(name).iter().map(|l| l.name.clone()).collect();
        for child in &children {
            let path = format!("{name}/{child}");
            if path.to_ascii_lowercase().contains("strain") && file.dataset(&path).is_ok() {
                return Some(path);
            }
        }
    }
    None
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    if bytes.len() != 8 + n * FIELDS * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut r = [0.0f64; FIELDS];
        for (j, slot) in r.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn emit_records(records: &mut Vec<[f64; FIELDS]>, out: &str) {
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    let bytes_out = write_bin(records);
    if std::fs::write(out, &bytes_out).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    let t_min = records.iter().map(|r| r[0]).fold(f64::INFINITY, f64::min);
    let t_max = records
        .iter()
        .map(|r| r[0])
        .fold(f64::NEG_INFINITY, f64::max);
    let v_min = records.iter().map(|r| r[1]).fold(f64::INFINITY, f64::min);
    let v_max = records
        .iter()
        .map(|r| r[1])
        .fold(f64::NEG_INFINITY, f64::max);
    eprintln!(
        "{out}: {} strain samples, t_gps [{t_min}, {t_max}], strain [{v_min}, {v_max}], {} B",
        records.len(),
        bytes_out.len()
    );
    match parse_bin(&bytes_out) {
        Some(parsed) if parsed == *records => {
            eprintln!("  roundtrip: {} records parse, identical", parsed.len());
        }
        Some(parsed) => {
            eprintln!(
                "  roundtrip: {} records parse but differ from the emitted set",
                parsed.len()
            );
            std::process::exit(1);
        }
        None => {
            eprintln!("  roundtrip parse void — the bin stays unverified");
            std::process::exit(1);
        }
    }
    let out_name = match std::path::Path::new(out).file_name() {
        Some(n) => n.to_string_lossy().into_owned(),
        None => out.to_string(),
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{out_name}");
    println!("origin {ORIGIN}");
    println!("compiler {COMPILER}");
    println!("sha256 {}", sha256_hex(&bytes_out));
    println!("format gwosc");
}

fn compile(input: &str, dataset_override: Option<&str>, out: &str) {
    let bytes = match std::fs::read(input) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("{input}: the file stays unread");
            std::process::exit(1);
        }
    };
    if bytes.get(0..4) != Some(&HDF5_MAGIC[..]) {
        eprintln!("{input}: carries no HDF5 signature — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let file = match Hdf5File::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{input}: HDF5 arm parses void: {note:?}");
            std::process::exit(1);
        }
    };
    let dataset = match dataset_override {
        Some(name) => {
            if file.dataset(name).is_err() {
                eprintln!("{input}: --dataset {name} does not resolve — the strain stays unread");
                std::process::exit(1);
            }
            name.to_string()
        }
        None => match discover_strain(&file) {
            Some(name) => name,
            None => {
                eprintln!(
                    "{input}: no strain dataset on the file — the name is absent, nothing fabricated"
                );
                std::process::exit(1);
            }
        },
    };
    let values = match file.read_f64_dataset(&dataset) {
        Ok(v) => v,
        Err(note) => {
            eprintln!("{input}: {dataset} stays unread: {note:?}");
            std::process::exit(1);
        }
    };
    let paths = dataset_paths(&dataset);
    let xstart = match find_attr(&file, &paths, "Xstart")
        .or_else(|| find_attr_int(&file, &paths, "Xstart"))
        .or_else(|| scalar_dataset(&file, "meta/GPSstart"))
        .or_else(|| find_attr(&file, &paths, "GPSstart"))
        .or_else(|| find_attr_int(&file, &paths, "GPSstart"))
    {
        Some(v) if v.is_finite() => v,
        _ => {
            eprintln!(
                "{input}: the strain time origin (Xstart/GPSstart) is absent — the bin stays unwritten"
            );
            std::process::exit(1);
        }
    };
    let xspacing = match find_attr(&file, &paths, "Xspacing").filter(|v| v.is_finite() && *v > 0.0)
    {
        Some(v) => v,
        _ => {
            let npoints = find_attr(&file, &paths, "Npoints")
                .or_else(|| find_attr_int(&file, &paths, "Npoints"));
            let duration = scalar_dataset(&file, "meta/Duration")
                .or_else(|| find_attr(&file, &paths, "Duration"))
                .or_else(|| find_attr_int(&file, &paths, "Duration"));
            match (npoints, duration) {
                (Some(n), Some(d)) if n.is_finite() && n > 0.0 && d.is_finite() && d > 0.0 => d / n,
                _ => {
                    eprintln!(
                        "{input}: the sample spacing (Xspacing) is absent and no Npoints/Duration pair derives it — the bin stays unwritten"
                    );
                    std::process::exit(1);
                }
            }
        }
    };
    let mut records: Vec<[f64; FIELDS]> = Vec::with_capacity(values.len());
    let mut absent = 0usize;
    for (i, value) in values.iter().enumerate() {
        if !value.is_finite() {
            absent += 1;
            continue;
        }
        let t = xstart + i as f64 * xspacing;
        records.push([t, *value, COMP_GWOSC_STRAIN]);
    }
    eprintln!(
        "{input}: {dataset} carries {} samples, t0 {xstart}, dt {xspacing}, {absent} absent",
        values.len()
    );
    emit_records(&mut records, out);
}

fn selftest() {
    let records = vec![
        [1_126_259_447.0, 0.0, COMP_GWOSC_STRAIN],
        [1_126_259_447.000_244, -1.5e-19, COMP_GWOSC_STRAIN],
    ];
    let bytes = write_bin(&records);
    match parse_bin(&bytes) {
        Some(parsed) if parsed == records => {
            println!(
                "gwosc_compiler selftest: {} records roundtrip",
                parsed.len()
            );
        }
        _ => {
            eprintln!("gwosc_compiler selftest: the packed records do not roundtrip");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => "gwosc_strain.bin".to_string(),
    };
    let input = match arg_value(&args, "--input") {
        Some(p) => p,
        None => {
            eprintln!(
                "usage: gwosc_compiler --input <hdf5> [--dataset <name>] [--out <file.bin>] [--ci-mode] | --selftest"
            );
            std::process::exit(2);
        }
    };
    let dataset = arg_value(&args, "--dataset");
    compile(&input, dataset.as_deref(), &out);
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = vec![
            [1_126_259_447.0, 0.0, COMP_GWOSC_STRAIN],
            [1_126_259_447.000_244, -1.5e-19, COMP_GWOSC_STRAIN],
        ];
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
    }

    #[test]
    fn parse_bin_rejects_foreign_magic_and_short_body() {
        assert!(parse_bin(b"XXXX\x00\x00\x00\x00").is_none());
        assert!(parse_bin(b"GWOS").is_none());
        let good = write_bin(&[[1.0, 2.0, COMP_GWOSC_STRAIN]]);
        let short = good[..good.len() - 1].to_vec();
        assert!(parse_bin(&short).is_none());
    }
}
