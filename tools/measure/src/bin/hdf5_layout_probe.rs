use omegaflow::archivar::hdf5::{Hdf5File, Hdf5Layout};

fn run(path: &str) -> (usize, usize) {
    let Ok(buf) = std::fs::read(path) else {
        println!("{path}: unreadable");
        return (0, 0);
    };
    let Ok(file) = Hdf5File::parse(&buf) else {
        println!("{path}: not HDF5");
        return (0, 0);
    };
    let Ok(root) = file.root() else {
        println!("{path}: no root group");
        return (0, 0);
    };
    let names: Vec<String> = root
        .links
        .iter()
        .filter(|l| l.addr != u64::MAX)
        .map(|l| l.name.clone())
        .collect();
    let mut chunked = 0usize;
    let mut deflated = 0usize;
    for name in names {
        let Ok(obj) = file.resolve(&name) else {
            continue;
        };
        let Some(ds) = &obj.dataspace else {
            continue;
        };
        let shape: Vec<String> = ds.dims.iter().map(|d| d.to_string()).collect();
        let layout = match &obj.layout {
            Some(Hdf5Layout::Contiguous { .. }) => "contiguous".to_string(),
            Some(Hdf5Layout::Compact { .. }) => "compact".to_string(),
            Some(Hdf5Layout::Chunked { chunk_dims, .. }) => {
                let c: Vec<String> = chunk_dims.iter().map(|d| d.to_string()).collect();
                format!("chunked [{}]", c.join("x"))
            }
            None => "absent".to_string(),
        };
        let filters: Vec<String> = obj.filters.iter().map(|f| f.id.to_string()).collect();
        let has_deflate = obj.filters.iter().any(|f| f.id == 1);
        if matches!(obj.layout, Some(Hdf5Layout::Chunked { .. })) {
            chunked += 1;
        }
        if has_deflate {
            deflated += 1;
        }
        let read = match file.read_dataset(&name) {
            Ok(v) => v.len().to_string(),
            Err(_) => "unread".to_string(),
        };
        println!(
            "{path}: {name} [{}/{}] {layout} filters [{}] bytes {read}",
            shape.join("x"),
            ds.dims.len(),
            filters.join(",")
        );
    }
    println!("{path}: {chunked} chunked, {deflated} deflated");
    (chunked, deflated)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: hdf5_layout_probe <file.h5|.nc> [...]");
        std::process::exit(2);
    }
    let mut chunked = 0usize;
    let mut deflated = 0usize;
    for a in &args {
        let (c, d) = run(a);
        chunked += c;
        deflated += d;
    }
    println!("total: {chunked} chunked datasets, {deflated} deflated");
    std::process::exit(if chunked > 0 && deflated > 0 { 0 } else { 1 });
}
