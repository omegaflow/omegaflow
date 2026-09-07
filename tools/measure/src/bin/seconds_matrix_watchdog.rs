use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const RESTART_DELAY_S: u64 = 10;

fn iso_now() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}", d.as_secs()),
        Err(_) => "epoch-unknown".to_string(),
    }
}

fn append(log: &PathBuf, line: &str) {
    if let Some(dir) = log.parent() {
        let _ = fs::create_dir_all(dir);
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(log) {
        use std::io::Write;
        let _ = writeln!(f, "{line}");
    }
}

fn supervise(bin: &str, log: &PathBuf) {
    append(log, &format!("[watchdog] {} start {}", iso_now(), bin));
    let status = Command::new(bin).spawn().and_then(|mut child| child.wait());
    let line = match status {
        Ok(s) => format!("exit {s} — restart in {RESTART_DELAY_S} s"),
        Err(e) => format!("cannot start: {e}"),
    };
    append(log, &format!("[watchdog] {} {line}", iso_now()));
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let bin = match args.first() {
        Some(v) => v.clone(),
        None => match env::var("SOLAR_MATRIX_PROBE") {
            Ok(v) => v,
            Err(_) => "target/release/solar_seconds_matrix_probe".to_string(),
        },
    };
    let log = PathBuf::from("data/reports/solar_seconds_matrix.log");
    loop {
        supervise(&bin, &log);
        sleep(Duration::from_secs(RESTART_DELAY_S));
    }
}
