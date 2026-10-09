use std::env;
use std::process::exit;

use omegaflow::archivar::openneuro_eeg::{channel_index, channel_series, parse_bin};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn render_series(series: &[f64]) -> String {
    let mut out = String::with_capacity(series.len() * 20);
    for value in series {
        out.push_str(&format!("{value}\n"));
    }
    out
}

fn run(args: &[String]) -> Result<String, String> {
    let bin = arg_value(args, "--bin").ok_or("--bin <path> required (see --help)")?;
    if args.iter().any(|a| a == "--help" || a == "-h") {
        return Ok(
            "eeg_channel_split — the compact EEGB bin -> one text series per named channel\n\
             \x20 eeg_channel_split --bin <path.bin> --channel <label|1-based index>\n\
             writes one physical value per line to stdout (feed te_pair_probe --a/--b).\n\
             an absent channel reads absent and exits 2, never a fabricated 0.\n"
                .to_string(),
        );
    }
    let selector = arg_value(args, "--channel")
        .ok_or("--channel <label|1-based index> required (see --help)")?;
    let bytes = std::fs::read(&bin).map_err(|e| format!("read {bin} returned void: {e}"))?;
    let eeg = parse_bin(&bytes).ok_or_else(|| format!("{bin}: no compact EEGB asset"))?;
    let index = channel_index(&eeg, &selector)
        .ok_or_else(|| format!("{bin}: channel '{selector}' is absent"))?;
    let series = channel_series(&eeg, &selector).ok_or_else(|| {
        format!(
            "{bin}: channel {} ('{selector}') carries no finite series",
            index + 1
        )
    })?;
    Ok(render_series(&series))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(text) => print!("{text}"),
        Err(msg) => {
            eprintln!("eeg_channel_split: {msg}");
            exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_series_writes_one_value_per_line() {
        assert_eq!(render_series(&[1.0, -2.5, 40.0]), "1\n-2.5\n40\n");
        assert_eq!(render_series(&[]), "");
    }

    #[test]
    fn arg_value_reads_the_named_flag() {
        let args = vec![
            "--bin".to_string(),
            "x.bin".to_string(),
            "--channel".to_string(),
            "EKG1".to_string(),
        ];
        assert_eq!(arg_value(&args, "--bin"), Some("x.bin".to_string()));
        assert_eq!(arg_value(&args, "--channel"), Some("EKG1".to_string()));
        assert_eq!(arg_value(&args, "--out"), None);
    }

    #[test]
    fn run_refuses_a_missing_bin_argument() {
        let args = vec!["--channel".to_string(), "EKG1".to_string()];
        assert!(run(&args).is_err());
    }

    #[test]
    fn run_refuses_foreign_bytes() {
        let dir = std::env::temp_dir().join("eeg_channel_split_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("foreign.bin");
        std::fs::write(&path, b"not a compact EEGB asset").unwrap();
        let args = vec![
            "--bin".to_string(),
            path.to_string_lossy().into_owned(),
            "--channel".to_string(),
            "EKG1".to_string(),
        ];
        assert!(run(&args).is_err());
        let _ = std::fs::remove_file(&path);
    }
}
