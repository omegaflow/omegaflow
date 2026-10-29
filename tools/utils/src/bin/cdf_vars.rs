use omegaflow::archivar::cdf::CdfFile;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: cdf_vars <file.cdf>");
            std::process::exit(2);
        }
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cdf_vars: read {path}: {e}");
            std::process::exit(2);
        }
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("cdf_vars: {path} does not parse as CDF: {note:?}");
            std::process::exit(1);
        }
    };
    println!(
        "cdf_vars: {path} | version {}.{}.{} | {} variables | little_endian {}",
        file.version.0,
        file.version.1,
        file.version.2,
        file.vars.len(),
        file.little_endian
    );
    let wanted = match std::env::args().nth(2) {
        Some(name) => Some(name),
        None => std::env::args().nth(3),
    };
    if let Some(name) = &wanted {
        let Some(var) = file.var(name) else {
            eprintln!("cdf_vars: no variable named {name}");
            std::process::exit(2);
        };
        match file.var_records(&bytes, var) {
            Ok(records) => {
                println!(
                    "{name}: {} record(s), first number {:?}, last number {:?}",
                    records.len(),
                    records.first().map(|r| r.0),
                    records.last().map(|r| r.0)
                );
                for (i, (number, values)) in records.iter().take(3).enumerate() {
                    println!("  [{i}] rec {number}: {:?}", &values[..values.len().min(3)]);
                }
            }
            Err(note) => println!("{name}: {note:?}"),
        }
    }
    for var in &file.vars {
        let dims = if var.dim_sizes.is_empty() {
            "-".to_string()
        } else {
            var.dim_sizes
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join("x")
        };
        println!(
            "  {:<28} type {:>3} elements {:>2} dims {:<10} records {} sparse {} max_rec {} vdr {}",
            var.name,
            var.data_type,
            var.num_elements,
            dims,
            var.record_vary,
            var.sparse,
            var.max_rec,
            var.vdr_offset
        );
    }
}
