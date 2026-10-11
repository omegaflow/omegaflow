use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::zip_members;
use omegaflow::lsk::days_from_civil;
use omegaflow::substorm::{SubstormOnset, parse_bin, write_bin};

const NETLOC: &str = "zenodo.org";
const FORMAT: &str = "substorm";
const URL: &str = "https://zenodo.org/api/records/22711679/files/CorrOnsFiles.zip/content";
const MEMBER: &str = "ftims.txt";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_ftims(text: &str) -> Vec<SubstormOnset> {
    let mut out = Vec::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split_whitespace().take(6).collect();
        if cols.len() < 6 {
            continue;
        }
        let mut v = [0i64; 6];
        let mut ok = true;
        for (i, c) in cols.iter().enumerate() {
            match c.parse::<i64>() {
                Ok(x) => v[i] = x,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let Some(days) = days_from_civil(v[0], v[1], v[2]) else {
            continue;
        };
        let t_unix = (days * 86400 + v[3] * 3600 + v[4] * 60 + v[5]) as f64;
        out.push(SubstormOnset {
            t_unix,
            mlt: None,
            mlat: None,
            glon: None,
            glat: None,
        });
    }
    out
}

fn member_text(zip: &[u8]) -> Option<String> {
    let mut text: Option<String> = None;
    zip_members(zip, |name, data| {
        if text.is_none() && name.ends_with(MEMBER) {
            text = Some(String::from_utf8_lossy(data).into_owned());
        }
    });
    text
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(args, "--out-bin") {
        Some(v) => v,
        None => "mcpherron.bin".to_string(),
    };

    let text = match arg_value(args, "--in") {
        Some(path) => {
            std::fs::read_to_string(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => {
            let bytes = fetch_raw_bytes(URL).ok_or_else(|| format!("{URL}: fetch void"))?;
            member_text(&bytes)
                .ok_or_else(|| format!("{URL}: zip carries no {MEMBER} member"))?
        }
    };

    let records = parse_ftims(&text);
    if records.is_empty() {
        return Err("no onsets parsed — the bin stays unwritten (0 honored)".to_string());
    }
    let bytes = write_bin(&records);
    if parse_bin(&bytes).is_none() {
        return Err(format!("{out}: roundtrip parse void"));
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    eprintln!(
        "{out}: {} onsets ({} B), format {FORMAT}, sha256 {}",
        records.len(),
        bytes.len(),
        sha256_hex(&bytes)
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: mcpherron_compiler [--in <ftims.txt>] [--out-bin <name>] [--ci-mode]");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("mcpherron_compiler: {msg}");
        std::process::exit(1);
    }
}
