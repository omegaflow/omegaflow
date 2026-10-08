use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut input: Option<String> = None;
    let mut lang = String::from("eng");
    let mut out: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--lang" => {
                i += 1;
                match args.get(i) {
                    Some(v) => lang = v.clone(),
                    None => usage("--lang needs a code"),
                }
            }
            "--out" => {
                i += 1;
                match args.get(i) {
                    Some(v) => out = Some(v.clone()),
                    None => usage("--out needs a directory"),
                }
            }
            _ if input.is_none() => input = Some(args[i].clone()),
            other => usage(&format!("unknown argument: {other}")),
        }
        i += 1;
    }

    let Some(input) = input else {
        usage("a file is needed");
    };

    if !command_present("tesseract") {
        println!("pending — tesseract absent from PATH");
        std::process::exit(2);
    }

    let lower = input.to_lowercase();
    if lower.ends_with(".pdf") {
        if !command_present("pdftoppm") {
            println!("pending — pdftoppm absent from PATH");
            std::process::exit(2);
        }
        let outdir = match out {
            Some(d) => PathBuf::from(d),
            None => fresh_temp_dir(),
        };
        if let Err(e) = fs::create_dir_all(&outdir) {
            println!("pending — outdir does not open: {} ({e})", outdir.display());
            std::process::exit(2);
        }
        let prefix = outdir.join("page");
        match Command::new("pdftoppm")
            .arg("-r")
            .arg("300")
            .arg("-png")
            .arg(&input)
            .arg(&prefix)
            .status()
        {
            Ok(s) if s.success() => {}
            Ok(s) => {
                println!("pending — pdftoppm exit {}", exit_code(s));
                std::process::exit(2);
            }
            Err(e) => {
                println!("pending — pdftoppm does not run: {e}");
                std::process::exit(2);
            }
        }
        let mut pages: Vec<PathBuf> = match fs::read_dir(&outdir) {
            Ok(rd) => rd
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("page") && n.ends_with(".png"))
                        .unwrap_or(false)
                })
                .collect(),
            Err(e) => {
                println!("pending — outdir does not read: {e}");
                std::process::exit(2);
            }
        };
        pages.sort();
        if pages.is_empty() {
            println!("pending — pdftoppm wrote no page-*.png");
            std::process::exit(2);
        }
        for (n, page) in pages.iter().enumerate() {
            println!("--- page {} ---", n + 1);
            ocr_file(page, &lang);
        }
    } else {
        ocr_file(Path::new(&input), &lang);
    }
}

fn ocr_file(path: &Path, lang: &str) {
    match Command::new("tesseract")
        .arg(path)
        .arg("stdout")
        .arg("-l")
        .arg(lang)
        .output()
    {
        Ok(o) if o.status.success() => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
        }
        Ok(o) => {
            println!("pending — tesseract exit {}", exit_code(o.status));
        }
        Err(e) => {
            println!("pending — tesseract does not run: {e}");
        }
    }
}

fn command_present(name: &str) -> bool {
    match std::env::var_os("PATH") {
        Some(path) => std::env::split_paths(&path).any(|dir| dir.join(name).is_file()),
        None => false,
    }
}

fn fresh_temp_dir() -> PathBuf {
    let nanos = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_nanos().to_string(),
        Err(_) => String::from("before-epoch"),
    };
    std::env::temp_dir().join(format!("ocr_reader-{}-{}", std::process::id(), nanos))
}

fn exit_code(status: ExitStatus) -> String {
    match status.code() {
        Some(c) => c.to_string(),
        None => String::from("signal"),
    }
}

fn usage(detail: &str) -> ! {
    eprintln!("usage: ocr_reader <file.pdf|image> [--lang <code>] [--out <dir>]");
    eprintln!("{detail}");
    std::process::exit(2);
}
