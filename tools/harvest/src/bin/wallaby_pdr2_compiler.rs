use std::io::Write;

use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::wallaby::{
    KEYS, WallabySource, parse_bin, parse_json_catalogue, write_bin,
};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn read_source(path: &str) -> Result<String, String> {
    let bytes = if path.starts_with("http://") || path.starts_with("https://") {
        fetch_raw_bytes(path).ok_or_else(|| format!("{path}: fetch returned void"))?
    } else {
        std::fs::read(path).map_err(|e| format!("{path}: read returned void: {e}"))?
    };
    String::from_utf8(bytes).map_err(|_| format!("{path}: body carries no utf8 text"))
}

fn run(args: &[String]) -> Result<(), String> {
    let input =
        arg_value(args, "--in").ok_or("--in <json path|url>: the catalogue is never silent")?;
    let out_path = arg_value(args, "--out")
        .ok_or("--out <table path>: the table path is never silent — refused")?;
    let text = read_source(&input)?;
    let sources = parse_json_catalogue(&text)
        .ok_or_else(|| format!("{input}: JSON does not read as the WALLABY source catalogue"))?;
    let limit = match arg_value(args, "--limit") {
        Some(v) => Some(
            v.parse::<usize>()
                .map_err(|_| "--limit carries no usize".to_string())?,
        ),
        None => None,
    };
    let sources = match limit {
        Some(n) => sources.into_iter().take(n).collect::<Vec<WallabySource>>(),
        None => sources,
    };
    let total = sources.len();
    let sources: Vec<WallabySource> = sources
        .into_iter()
        .filter(|s| KEYS.iter().all(|key| s.value_by_key(key).is_some()))
        .collect();
    if sources.len() != total {
        eprintln!(
            "{} row(s) with absent cells stay unwritten (0 honored)",
            total - sources.len()
        );
    }
    let bytes = write_bin(&sources)
        .ok_or("the catalogue refuses its own rows — the table stays unwritten")?;
    if let Some(parent) = std::path::Path::new(&out_path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
    }
    let mut file = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let parsed = parse_bin(&bytes).ok_or("catalogue roundtrip parse void — table unverified")?;
    if parsed != sources {
        return Err("catalogue roundtrip differs from the written table — table unverified".into());
    }
    println!(
        "{out_path}: {} B, {} rows, sha256 {}",
        bytes.len(),
        parsed.len(),
        sha256_hex(&bytes)
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("wallaby_pdr2_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_reads_and_writes_the_seven_columns() {
        let text = r#"[
            {"ra":154.2297749022268,"dec":-48.8773454083931,"freq":1.407512971336861E9,
             "w20":1809946.485445322,"w50":1703609.096101342,"dist_h":39.22986672308076,
             "log_m_hi":10.239972898827773},
            {"ra":153.7,"dec":-43.6,"freq":1.4061E9,"w20":null,"w50":1.69E6,
             "dist_h":43.35,"log_m_hi":10.06}
        ]"#;
        let sources = parse_json_catalogue(text).unwrap();
        assert_eq!(sources.len(), 2);
        let bytes = write_bin(&sources).unwrap();
        assert_eq!(parse_bin(&bytes).unwrap(), sources);
        assert_eq!(KEYS.len(), 7);
    }
}
