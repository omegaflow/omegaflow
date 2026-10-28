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
            "  {:<28} type {:>3} elements {:>2} dims {:<10} records {} sparse {}",
            var.name, var.data_type, var.num_elements, dims, var.record_vary, var.sparse
        );
    }
}
