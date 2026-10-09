use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::root::{RootFile, read_file};

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn load(source: &str) -> Option<Vec<u8>> {
    if source.starts_with("http://") || source.starts_with("https://") {
        fetch_raw_bytes(source)
    } else {
        std::fs::read(source).ok()
    }
}

fn report(file: &RootFile, source: &str) {
    let h = &file.header;
    println!("{source}");
    println!(
        "root version {} (big {})  fBEGIN {}  fEND {}  fSeekFree {}  fNbytesFree {}  nfree {}",
        h.version, h.big, h.begin, h.end, h.seek_free, h.nbytes_free, h.nfree
    );
    println!(
        "fNbytesName {}  fUnits {}  fCompress {}  fSeekInfo {}  fNbytesInfo {}  fUUID {}",
        h.nbytes_name,
        h.units,
        h.compress,
        h.seek_info,
        h.nbytes_info,
        h.uuid
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    );
    println!("keys {}", file.keys.len());
    for k in &file.keys {
        println!(
            "  {:<20} {:<16} bytes {:>8}  objlen {:>8}  offset {:>10}  pdir {:>6}  cycle {}",
            k.name, k.class, k.nbytes, k.objlen, k.seek_key, k.seek_pdir, k.cycle
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(source) = arg_value(&args, "--probe") else {
        eprintln!("--probe <url|file> required");
        std::process::exit(2);
    };
    let Some(bytes) = load(&source) else {
        eprintln!("{source}: fetch/read void");
        std::process::exit(2);
    };
    match read_file(&bytes) {
        Ok(file) => report(&file, &source),
        Err(gap) => {
            eprintln!("{source}: {gap}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_value_reads_following_token() {
        let args = vec!["--probe".to_string(), "x.root".to_string()];
        assert_eq!(arg_value(&args, "--probe"), Some("x.root".to_string()));
        assert_eq!(arg_value(&args, "--missing"), None);
    }
}
