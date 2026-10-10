use omegaflow::archivar::planetary_radar::{self, RadarRange};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use std::collections::BTreeSet;

const NETLOC: &str = "iaaras.ru";
const COMPILER: &str = "tools/harvest/src/bin/planetary_radar_compiler.rs";
const FORMAT: &str = "planetary_radar";

const WAYBACK: &str =
    "https://web.archive.org/web/20190218230236id_/http://iaaras.ru:80/media/observations/";

const SAMPLE: &str = concat!(
    "Venus radar observations from Eupatoria, Crimea\n",
    "\n",
    "   2   8   8  19621021.0956  UT2    337074800.000  120.000\n",
);

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn measured(records: Vec<RadarRange>) -> (Vec<RadarRange>, usize) {
    let total = records.len();
    let named: Vec<RadarRange> = records
        .into_iter()
        .filter(|r| r.epoch_utc.is_some() && r.roundtrip_us.is_some())
        .collect();
    let held = total - named.len();
    (named, held)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let input = arg_value(args, "--input")
        .ok_or_else(|| "--input <rad.txt> absent — the file stays unread".to_string())?;
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };

    let bytes = std::fs::read(&input).map_err(|e| format!("{input}: read void ({e})"))?;
    let records = planetary_radar::parse_rad(&bytes)
        .ok_or_else(|| format!("{input}: carries no radar ranging row — refused"))?;
    let (mut named, held) = measured(records);
    if held > 0 {
        eprintln!("{input}: {held} rows carry no finite epoch/round-trip time — held out, named");
    }
    if named.is_empty() {
        return Err(format!(
            "{input}: no row carries a finite epoch and round-trip time — the bin stays unwritten (0 honored)"
        ));
    }
    named.sort_by(|a, b| match (a.epoch_utc, b.epoch_utc) {
        (Some(x), Some(y)) => x.total_cmp(&y),
        _ => std::cmp::Ordering::Equal,
    });

    let bin = planetary_radar::write_bin(&named)
        .ok_or_else(|| "a held value is not finite — the bin stays unwritten".to_string())?;
    match planetary_radar::parse_bin(&bin) {
        Some(parsed) if parsed == named => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    let planets: BTreeSet<u8> = named.iter().map(|r| r.planet).collect();
    let stations: BTreeSet<u16> = named
        .iter()
        .flat_map(|r| [r.xmit_station, r.rcvr_station])
        .collect();
    let basename = std::path::Path::new(&input)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let origin = if basename.is_empty() {
        input.clone()
    } else {
        format!("{WAYBACK}{basename}")
    };

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} rows, planets {planets:?}, stations {stations:?}, {held} held, {} B, roundtrip parses",
        named.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let records = planetary_radar::parse_rad(SAMPLE.as_bytes());
    let Some(records) = records else {
        eprintln!("selftest: the sample radar file does not parse");
        std::process::exit(1);
    };
    if records.len() != 1 {
        eprintln!("selftest: the sample does not carry exactly one ranging row");
        std::process::exit(1);
    }
    let Some(bin) = planetary_radar::write_bin(&records) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != planetary_radar::HEADER_BYTES + planetary_radar::RECORD_BYTES {
        eprintln!("selftest: the bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if planetary_radar::parse_bin(&bin) != Some(records.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if planetary_radar::parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    eprintln!(
        "planetary_radar_compiler: selftest passes (ranging row → t/round-trip/sigma series)"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: planetary_radar_compiler --input <rad.txt> [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("planetary_radar_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_roundtrips_the_measured_stride() {
        let records = planetary_radar::parse_rad(SAMPLE.as_bytes()).expect("the sample parses");
        let bin = planetary_radar::write_bin(&records).expect("finite records encode");
        assert_eq!(
            bin.len(),
            planetary_radar::HEADER_BYTES + planetary_radar::RECORD_BYTES
        );
        assert_eq!(planetary_radar::parse_bin(&bin), Some(records));
    }

    #[test]
    fn parse_rad_refuses_foreign_bytes() {
        assert_eq!(planetary_radar::parse_rad(b"not a radar file"), None);
    }
}
