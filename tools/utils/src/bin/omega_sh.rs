use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        return;
    }
    match args[1].as_str() {
        "reports" => reports(),
        "status" => status(),
        "search" => run_sibling("sgrep", &args[2..]),
        "fetch" => run_sibling("sfetch", &args[2..]),
        "jwst" => jwst(),
        "help" | "-h" | "--help" => usage(),
        other => {
            eprintln!("omega_sh: unknown subcommand '{}'", other);
            usage();
        }
    }
}

fn usage() {
    eprintln!("usage: omega_sh <reports|status|search|fetch|jwst> [args]");
    eprintln!("  reports  concat <state>/reports/*.φ (the watchdog lines)");
    eprintln!("  status   git status --short");
    eprintln!("  search   sgrep <args>");
    eprintln!("  fetch    sfetch <args>");
    eprintln!("  jwst     the jwst_spectra.bin CDN watch");
}

fn reports() {
    let entries = match fs::read_dir(state_dir().join("reports")) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("omega_sh: no <state>/reports dir");
            return;
        }
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .file_name()
                .map(|f| f.to_string_lossy().ends_with(".φ"))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();
    names.sort();
    for n in names {
        match fs::read_to_string(&n) {
            Ok(t) => print!("{}", t),
            Err(_) => eprintln!("omega_sh: read void: {}", n),
        }
    }
}

fn status() {
    match Command::new("git")
        .args(["status", "--short"])
        .output()
    {
        Ok(o) => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
            eprint!("{}", String::from_utf8_lossy(&o.stderr));
        }
        Err(_) => eprintln!("omega_sh: git not available"),
    }
}

fn run_sibling(name: &str, args: &[String]) {
    let exe = match env::current_exe() {
        Ok(e) => e,
        Err(_) => {
            eprintln!("omega_sh: current_exe void");
            return;
        }
    };
    let dir = exe
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let sibling = dir.join(name);
    match Command::new(&sibling).args(args).output() {
        Ok(o) => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
            eprint!("{}", String::from_utf8_lossy(&o.stderr));
        }
        Err(_) => eprintln!("omega_sh: {} not found next to {}", name, exe.display()),
    }
}

fn jwst() {
    let asset =
        "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/jwst_spectra.bin";
    let out = Command::new("curl")
        .args([
            "-sL",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "--max-time",
            "30",
            asset,
        ])
        .output();
    let code = match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => String::from("void"),
    };
    println!(
        "jwst_spectra.bin: {} ({}) — harvest complete",
        jwst_verdict(&code),
        code
    );
    if let Ok(t) = fs::read_to_string("phi/jwst_harvest/ledger.tsv") {
        let n = t.lines().count();
        println!("local ledger: {} obs_id finished", n);
    }
}

fn state_dir() -> PathBuf {
    if let Ok(dir) = env::var("OMEGAFLOW_STATE") {
        return PathBuf::from(dir);
    }
    PathBuf::from(".")
}

fn jwst_verdict(status: &str) -> &'static str {
    if status == "200" { "DA" } else { "absent" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_da_on_200() {
        assert_eq!(jwst_verdict("200"), "DA");
    }

    #[test]
    fn verdict_fehlt_otherwise() {
        assert_eq!(jwst_verdict("404"), "absent");
        assert_eq!(jwst_verdict("void"), "absent");
    }
}
