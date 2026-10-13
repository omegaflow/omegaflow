use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::gfc::{
    FUNC_GFC, FUNC_GFCT, SLOT_BIN_WIDTH, SLOT_CHANNEL, SLOT_EPOCH, SLOT_FREQ, SLOT_KERNEL,
    SLOT_PRESENCE, SLOT_TTL, SLOT_VAL, channel_of, channels, write_bin,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::archivar::{embedded_lsk, parse_iso_tdb};
use omegaflow::cdn::upload_release;
use omegaflow::force::{force_id_of, kernel_id_for_force};
use std::path::Path;

const NETLOC: &str = "icgem.gfz.de";
const FORMAT: &str = "gfc";
const COMPILER: &str = "tools/harvest/src/bin/gfc_compiler.rs";
const DEFAULT_URL: &str = "https://icgem.gfz.de/getmodel/gfc/8e01a2aba4f291ac0638b79a390155713b70fdc9b266981023c756839c11c9b8/XGM2019e.gfc";
const OUT_PATH: &str = "data/icgem.gfz.de/gfc.bin";

#[derive(Clone, Debug, PartialEq)]
struct Header {
    modelname: String,
    max_degree: u32,
    norm: Option<String>,
    earth_gravity_constant: f64,
    radius: f64,
    epoch: Option<String>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn line_tokens(line: &[u8]) -> Vec<&str> {
    match std::str::from_utf8(line) {
        Ok(s) => s.split_whitespace().collect(),
        Err(_) => Vec::new(),
    }
}

fn header_of(bytes: &[u8]) -> Option<(Header, usize)> {
    let mut modelname: Option<String> = None;
    let mut max_degree: Option<u32> = None;
    let mut norm: Option<String> = None;
    let mut gravity_constant: Option<f64> = None;
    let mut radius: Option<f64> = None;
    let mut epoch: Option<String> = None;
    let mut in_head = false;
    let mut end_line = None;
    for (i, raw) in bytes.split(|&b| b == b'\n').enumerate() {
        let Some(line) = std::str::from_utf8(raw).ok().map(str::trim) else {
            continue;
        };
        if line.starts_with("begin_of_head") {
            in_head = true;
            continue;
        }
        if line.starts_with("end_of_head") {
            end_line = Some(i);
            break;
        }
        if !in_head {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(key), Some(value)) = (it.next(), it.next()) else {
            continue;
        };
        match key {
            "modelname" => modelname = Some(value.to_string()),
            "max_degree" => max_degree = value.parse::<u32>().ok(),
            "norm" => norm = Some(value.to_string()),
            "earth_gravity_constant" => gravity_constant = value.parse::<f64>().ok(),
            "radius" => radius = value.parse::<f64>().ok(),
            "epoch" | "ref_epoch" => epoch = Some(value.to_string()),
            _ => {}
        }
    }
    let header = Header {
        modelname: modelname?,
        max_degree: max_degree?,
        norm,
        earth_gravity_constant: gravity_constant?,
        radius: radius?,
        epoch,
    };
    Some((header, end_line?))
}

fn epoch_of(header_epoch: &Option<String>, lsk: &omegaflow::archivar::LeapSeconds) -> Option<f64> {
    header_epoch.as_deref().and_then(|v| parse_iso_tdb(v, lsk))
}

fn push_coefficient(
    records: &mut Vec<[f64; 26]>,
    functional: u32,
    n: f64,
    m: f64,
    c: f64,
    s: f64,
    epoch: f64,
    ttl: f64,
    kernel: u8,
) {
    for (sine, value) in [(false, c), (true, s)] {
        let Some(comp) = channel_of(functional, sine) else {
            continue;
        };
        if !value.is_finite() {
            continue;
        }
        let mut r = [0.0f64; 26];
        r[SLOT_VAL] = value;
        r[SLOT_EPOCH] = epoch;
        r[SLOT_TTL] = ttl;
        r[SLOT_KERNEL] = kernel as f64;
        r[SLOT_CHANNEL] = comp as f64;
        r[SLOT_FREQ] = n;
        r[SLOT_BIN_WIDTH] = m;
        r[SLOT_PRESENCE] = 1.0;
        records.push(r);
    }
}

fn compile(
    bytes: &[u8],
    ttl: f64,
    max_degree: Option<u32>,
    out: &str,
    origin: &str,
    ci_mode: bool,
) -> Result<(), String> {
    let (header, end_line) = header_of(bytes).ok_or_else(|| {
        "the ICGEM header (begin_of_head/end_of_head) stays unread — the asset stays unwritten"
            .to_string()
    })?;
    let lsk = embedded_lsk().ok_or("the embedded naif0012 table stays unread")?;
    let epoch = epoch_of(&header.epoch, &lsk)
        .ok_or_else(|| "the ICGEM header carries no epoch — the asset stays unwritten".to_string())?;
    let cap = max_degree.unwrap_or(header.max_degree);
    let channel_ref =
        force_id_of("gravity").ok_or("gravity carries no force id — the asset stays unwritten")?;
    let kernel = kernel_id_for_force(channel_ref)
        .ok_or("gravity carries no kernel id — the asset stays unwritten")?;

    let mut records: Vec<[f64; 26]> = Vec::new();
    let mut lines_seen = 0usize;
    let mut lines_parsed = 0usize;
    let mut lines_capped = 0usize;
    let mut lines_ignored = 0usize;
    for (i, raw) in bytes.split(|&b| b == b'\n').enumerate() {
        if i <= end_line {
            continue;
        }
        let tokens = line_tokens(raw);
        if tokens.len() < 5 {
            continue;
        }
        let functional = match tokens[0] {
            "gfc" => FUNC_GFC,
            "gfct" => FUNC_GFCT,
            _ => {
                if tokens[0].bytes().all(|b| b.is_ascii_alphabetic()) {
                    lines_ignored += 1;
                }
                continue;
            }
        };
        lines_seen += 1;
        let (Ok(n), Ok(m), Ok(c), Ok(s)) = (
            tokens[1].parse::<f64>(),
            tokens[2].parse::<f64>(),
            tokens[3].parse::<f64>(),
            tokens[4].parse::<f64>(),
        ) else {
            continue;
        };
        if n > cap as f64 {
            lines_capped += 1;
            continue;
        }
        push_coefficient(&mut records, functional, n, m, c, s, epoch, ttl, kernel);
        lines_parsed += 1;
    }

    if records.is_empty() {
        return Err(
            "no gfc/gfct coefficient survived — the asset stays unwritten (0 honored)".to_string(),
        );
    }

    let bytes_out = write_bin(&records).ok_or("framing returned void")?;
    match omegaflow::archivar::gfc::parse_records(&bytes_out) {
        Some(parsed) if parsed == records => {}
        _ => {
            return Err(format!(
                "{out}: the frame roundtrip stays void — the asset stays unverified"
            ));
        }
    }

    if let Some(parent) = Path::new(out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(out, &bytes_out).map_err(|e| format!("write {out} void: {e}"))?;
    let sha = sha256_hex(&bytes_out);

    eprintln!(
        "{}: modelname {}, d/o {} (cap {}), norm {}, GM {:e}, radius {} m",
        origin,
        header.modelname,
        header.max_degree,
        cap,
        header.norm.as_deref().unwrap_or("absent"),
        header.earth_gravity_constant,
        header.radius
    );
    eprintln!(
        "{out}: {lines_parsed} coefficient line(s) -> {} record(s) (of {lines_seen} read, {lines_capped} beyond the cap, {lines_ignored} other functional(s) skipped), {} B, sha256 {sha}",
        records.len(),
        bytes_out.len()
    );

    let name = Path::new(out)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("{out}: carries no file name"))?;
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{name}");
    println!("terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007");
    println!("format {FORMAT}");
    println!("origin {origin}");
    println!("compiler {COMPILER}");
    println!("sha256 {sha}");
    println!("at earth");
    println!("ttl {}", ttl as u64);
    for c in channels() {
        println!(
            "field {} {} inverse-square gravity {} {} 0.0 0.0",
            c.name, c.name, c.unit, ttl as u64
        );
    }

    if ci_mode && !upload_release(NETLOC, out) {
        return Err(format!("{out}: CDN upload returned void"));
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "usage: gfc_compiler [--input <x.gfc>|--url <url>] [--out <file.bin>] --ttl <secs> [--max-degree <n>] [--ci-mode]"
        );
        return Ok(());
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let ttl = match arg_value(args, "--ttl").and_then(|v| v.parse::<f64>().ok()) {
        Some(t) if t.is_finite() && t > 0.0 => t,
        _ => {
            return Err(
                "the model's ttl is declared per source — pass --ttl <secs> (a ttl <= 0 drops every record)"
                    .to_string(),
            );
        }
    };
    let max_degree = match arg_value(args, "--max-degree") {
        Some(v) => Some(
            v.parse::<u32>()
                .map_err(|e| format!("--max-degree {v}: {e}"))?,
        ),
        None => None,
    };
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => OUT_PATH.to_string(),
    };
    let (bytes, origin) = match arg_value(args, "--input") {
        Some(path) => (
            std::fs::read(&path).map_err(|e| format!("read {path} void: {e}"))?,
            path,
        ),
        None => {
            let url = match arg_value(args, "--url") {
                Some(v) => v,
                None => DEFAULT_URL.to_string(),
            };
            let bytes =
                fetch_raw_bytes(&url).ok_or_else(|| format!("{url}: fetch returned void"))?;
            (bytes, url)
        }
    };
    compile(&bytes, ttl, max_degree, &out, &origin, ci_mode)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("gfc_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBE: &str = "begin_of_head =====\n\
product_type gravity_field\n\
modelname XGM2019e\n\
earth_gravity_constant 3.986004415E+14\n\
radius 6378136.3\n\
max_degree 2\n\
errors formal\n\
norm fully_normalized\n\
key L M C S sigmaC sigmaS\n\
end_of_head =====\n\
gfc 0 0 1.000000000000000E+00 0.000000000000000E+00 0.0 0.0\n\
gfc 2 2 2.439370331756250E-06 -1.400307649958125E-06 0.0 0.0\n";

    #[test]
    fn header_reads_the_mandatory_keys() {
        let (header, _) = header_of(PROBE.as_bytes()).expect("the probe header parses");
        assert_eq!(header.modelname, "XGM2019e");
        assert_eq!(header.max_degree, 2);
        assert_eq!(header.norm.as_deref(), Some("fully_normalized"));
        assert_eq!(header.earth_gravity_constant, 3.986004415E14);
        assert!((header.radius - 6378136.3).abs() < 1e-6);
    }

    #[test]
    fn header_of_finds_the_end_of_head_and_the_coefficients() {
        let (header, end_line) = header_of(PROBE.as_bytes()).expect("the probe header parses");
        assert_eq!(header.max_degree, 2);
        assert_eq!(end_line, 9);
    }

    #[test]
    fn coefficients_fold_onto_the_gfc_and_gfct_channels() {
        let bytes = PROBE.as_bytes();
        let (header, end_line) = header_of(bytes).unwrap();
        let mut records: Vec<[f64; 26]> = Vec::new();
        for (i, raw) in bytes.split(|&b| b == b'\n').enumerate() {
            if i <= end_line {
                continue;
            }
            let tokens = line_tokens(raw);
            if tokens.len() < 5 || tokens[0] != "gfc" {
                continue;
            }
            let n: f64 = tokens[1].parse().unwrap();
            let m: f64 = tokens[2].parse().unwrap();
            let c: f64 = tokens[3].parse().unwrap();
            let s: f64 = tokens[4].parse().unwrap();
            push_coefficient(
                &mut records,
                FUNC_GFC,
                n,
                m,
                c,
                s,
                0.0,
                header.max_degree as f64,
                0,
            );
        }
        assert_eq!(records.len(), 4);
        assert_eq!(records[0][SLOT_CHANNEL], 1.0);
        assert_eq!(records[0][SLOT_VAL], 1.0);
        assert_eq!(records[1][SLOT_CHANNEL], 2.0);
        assert_eq!(records[2][SLOT_CHANNEL], 1.0);
        assert!((records[2][SLOT_VAL] - 2.439370331756250E-06).abs() < 1e-18);
        assert_eq!(records[3][SLOT_CHANNEL], 2.0);
        assert!((records[3][SLOT_VAL] + 1.400307649958125E-06).abs() < 1e-18);
    }

    #[test]
    fn gfct_maps_to_its_own_channels() {
        assert_eq!(channel_of(FUNC_GFCT, false), Some(3));
        assert_eq!(channel_of(FUNC_GFCT, true), Some(4));
    }
}
