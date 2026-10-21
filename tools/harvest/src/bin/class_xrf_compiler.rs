use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::harvest_table::{Table, parse_bin, write_bin};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::process::Command;

const NETLOC: &str = "data.mendeley.com";
const FORMAT: &str = "class_xrf";
const MAGIC: [u8; 4] = *b"CLS1";
const DEFAULT_OUT: &str = "class_xrf.bin";
const ORIGIN_URL: &str = "https://data.mendeley.com/public-files/datasets/6mmd96j5bg/files/4e692227-06b1-4c81-87c4-fcba8cf25c24/file_downloaded";
const DEFAULT_MEMBER: &str = "data_repository/290520/ch2_cla_scd_all_time_added_20200529T104532257-20200529T104548257_t1.txt";
const DEFAULT_KEV_PER_CHANNEL: f64 = 0.0135;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_counts(text: &str, kev_per_channel: f64) -> Option<Table> {
    if !(kev_per_channel.is_finite() && kev_per_channel > 0.0) {
        return None;
    }
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        let (Some(channel), Some(counts)) = (fields.next(), fields.next()) else {
            continue;
        };
        let (Ok(channel), Ok(counts)) = (channel.parse::<f64>(), counts.parse::<f64>()) else {
            continue;
        };
        if !(channel.is_finite() && counts.is_finite()) || channel < 0.0 || counts < 0.0 {
            continue;
        }
        rows.push(vec![
            Some(channel),
            Some(channel * kev_per_channel * 1000.0),
            Some(counts),
        ]);
    }
    if rows.is_empty() {
        return None;
    }
    Some(Table {
        names: vec![
            "channel".to_string(),
            "energy_ev".to_string(),
            "counts".to_string(),
        ],
        rows,
    })
}

fn member_text(zip: &[u8], member: &str) -> Result<String, String> {
    let dir = std::env::temp_dir().join("class_xrf");
    std::fs::create_dir_all(&dir).map_err(|e| format!("create {} void: {e}", dir.display()))?;
    let path = dir.join("class.zip");
    std::fs::write(&path, zip).map_err(|e| format!("write {} void: {e}", path.display()))?;
    let out = Command::new("unzip")
        .arg("-p")
        .arg(&path)
        .arg(member)
        .output()
        .map_err(|e| format!("unzip did not run: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "unzip -p {} {member}: {}",
            path.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    String::from_utf8(out.stdout).map_err(|e| format!("{member}: not UTF-8 ({e})"))
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => DEFAULT_OUT.to_string(),
    };
    let member = match arg_value(args, "--member") {
        Some(v) => v,
        None => DEFAULT_MEMBER.to_string(),
    };
    let kev_per_channel = match arg_value(args, "--kev-per-channel") {
        Some(v) => v
            .parse::<f64>()
            .map_err(|_| format!("--kev-per-channel {v} is not a number"))?,
        None => DEFAULT_KEV_PER_CHANNEL,
    };
    let text = match (arg_value(args, "--local"), arg_value(args, "--zip")) {
        (Some(_), Some(_)) => {
            return Err(
                "--local and --zip both present — exactly one source is the contract".into(),
            );
        }
        (Some(path), None) => {
            std::fs::read_to_string(&path).map_err(|e| format!("{path}: read void ({e})"))?
        }
        (None, Some(zip)) => {
            let bytes = if zip.starts_with("http://") || zip.starts_with("https://") {
                fetch_raw_bytes(&zip).ok_or_else(|| format!("{zip}: fetch void"))?
            } else {
                std::fs::read(&zip).map_err(|e| format!("{zip}: read void ({e})"))?
            };
            member_text(&bytes, &member)?
        }
        (None, None) => {
            let bytes =
                fetch_raw_bytes(ORIGIN_URL).ok_or_else(|| format!("{ORIGIN_URL}: fetch void"))?;
            member_text(&bytes, &member)?
        }
    };
    let table = parse_counts(&text, kev_per_channel)
        .ok_or_else(|| format!("{member}: carries no channel/counts rows — refused"))?;
    let bin = write_bin(MAGIC, &table);
    match parse_bin(MAGIC, &bin) {
        Some(parsed) if parsed == table => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".into()),
    }
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {ORIGIN_URL}");
    println!("compiler tools/harvest/src/bin/class_xrf_compiler.rs");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} channels, {:.4} keV/channel, {} B, roundtrip parses",
        table.rows.len(),
        kev_per_channel,
        bin.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: class_xrf_compiler [--zip <url|path>] [--member <zip-path>] [--local <txt>] [--kev-per-channel <f64>] [--out <bin>] [--ci-mode]"
        );
        return;
    }
    if let Err(msg) = run(&args) {
        eprintln!("class_xrf_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_rows_become_channel_energy_and_counts() {
        let text = ";ENERGY= 0.0135 keV x CHANNEL\n 0 0.0\n 1 2.0\n 2 4.0\n";
        let table = parse_counts(text, 0.0135).expect("table parses");
        assert_eq!(table.names, ["channel", "energy_ev", "counts"]);
        assert_eq!(table.rows[1], [Some(1.0), Some(13.5), Some(2.0)]);
    }

    #[test]
    fn header_and_non_numeric_lines_stay_out() {
        let text = "CHANNEL COUNTS\nfoo bar\n3 5\n";
        let table = parse_counts(text, 0.0135).expect("table parses");
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn an_empty_body_is_refused() {
        assert!(parse_counts("CHANNEL COUNTS\n", 0.0135).is_none());
    }
}
