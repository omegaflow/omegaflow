use std::process::{Command, ExitStatus};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(file) = args.first() else {
        eprintln!("usage: meta_reader <file>");
        std::process::exit(2);
    };

    if !command_present("exiftool") {
        println!("pending — exiftool absent from PATH");
        std::process::exit(2);
    }

    match Command::new("exiftool")
        .arg("-j")
        .arg("-a")
        .arg("-g")
        .arg(file)
        .output()
    {
        Ok(o) if o.status.success() => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
        }
        Ok(o) => {
            println!("pending — exiftool exit {}", exit_code(o.status));
            std::process::exit(2);
        }
        Err(e) => {
            println!("pending — exiftool does not run: {e}");
            std::process::exit(2);
        }
    }
}

fn command_present(name: &str) -> bool {
    match std::env::var_os("PATH") {
        Some(path) => std::env::split_paths(&path).any(|dir| dir.join(name).is_file()),
        None => false,
    }
}

fn exit_code(status: ExitStatus) -> String {
    match status.code() {
        Some(c) => c.to_string(),
        None => String::from("signal"),
    }
}
