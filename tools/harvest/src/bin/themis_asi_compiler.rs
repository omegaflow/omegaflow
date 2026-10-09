use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdf::{CdfFile, type_size, value_present};
use omegaflow::cdn::upload_release;
use std::collections::HashMap;

const NETLOC: &str = "themis.ssl.berkeley.edu";
const FORMAT: &str = "themis_asi";
const COMPILER: &str = "tools/harvest/src/bin/themis_asi_compiler.rs";
const URL: &str = "https://themis.ssl.berkeley.edu/data/themis/thg/l1/asi/fsim/2022/01/thg_l1_ast_fsim_20220131_v01.cdf";
const IMAGE_VAR: &str = "thg_ast_fsim";
const TIME_VAR: &str = "thg_ast_fsim_time";

const MAGIC: [u8; 4] = *b"TASI";
const HEADER_BYTES: usize = 12;
const PIXEL_BYTES: usize = 2;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn type_label(t: u32) -> &'static str {
    match t {
        omegaflow::cdf::TYPE_INT1 => "int1",
        omegaflow::cdf::TYPE_INT2 => "int2",
        omegaflow::cdf::TYPE_INT4 => "int4",
        omegaflow::cdf::TYPE_INT8 => "int8",
        omegaflow::cdf::TYPE_UINT1 => "uint1",
        omegaflow::cdf::TYPE_UINT2 => "uint2",
        omegaflow::cdf::TYPE_UINT4 => "uint4",
        omegaflow::cdf::TYPE_REAL4 => "real4",
        omegaflow::cdf::TYPE_REAL8 => "real8",
        omegaflow::cdf::TYPE_EPOCH => "epoch",
        omegaflow::cdf::TYPE_EPOCH16 => "epoch16",
        omegaflow::cdf::TYPE_TT2000 => "tt2000",
        omegaflow::cdf::TYPE_BYTE => "byte",
        omegaflow::cdf::TYPE_FLOAT => "float",
        omegaflow::cdf::TYPE_DOUBLE => "double",
        omegaflow::cdf::TYPE_CHAR => "char",
        omegaflow::cdf::TYPE_UCHAR => "uchar",
        _ => "unknown",
    }
}

fn list_vars(file: &CdfFile) {
    for v in &file.vars {
        let size = match v.data_type {
            omegaflow::cdf::TYPE_EPOCH16 => Some(16),
            t => type_size(t),
        };
        let size = match size {
            Some(s) => s.to_string(),
            None => "absent".to_string(),
        };
        eprintln!(
            "var {} type {} ({}) bytes {} dims {:?} vary {:?} record_vary {} compressed {}",
            v.name,
            v.data_type,
            type_label(v.data_type),
            size,
            v.dim_sizes,
            v.dim_vary,
            v.record_vary,
            v.compressed
        );
    }
}

fn write_bin(frames: &[(f64, Vec<u16>)], pixels: usize) -> Option<Vec<u8>> {
    let count = u32::try_from(frames.len()).ok()?;
    let pixels_u32 = u32::try_from(pixels).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + frames.len() * (8 + pixels * PIXEL_BYTES));
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&pixels_u32.to_le_bytes());
    for (epoch, values) in frames {
        if !epoch.is_finite() || values.len() != pixels {
            return None;
        }
        out.extend_from_slice(&epoch.to_le_bytes());
        for v in values {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    Some(out)
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<(f64, Vec<u16>)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let pixels = u32::from_le_bytes(bytes[8..12].try_into().ok()?) as usize;
    let per = 8 + pixels.checked_mul(PIXEL_BYTES)?;
    if bytes.len() != HEADER_BYTES + count.checked_mul(per)? {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let epoch = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        if !epoch.is_finite() {
            return None;
        }
        let mut values = Vec::with_capacity(pixels);
        for j in 0..pixels {
            let at = off + 8 + j * PIXEL_BYTES;
            values.push(u16::from_le_bytes(
                bytes.get(at..at + PIXEL_BYTES)?.try_into().ok()?,
            ));
        }
        off += per;
        out.push((epoch, values));
    }
    Some(out)
}

fn image_frames(file: &CdfFile, bytes: &[u8]) -> Result<Vec<(f64, Vec<u16>)>, String> {
    let image = file
        .var(IMAGE_VAR)
        .ok_or_else(|| format!("{IMAGE_VAR} absent — the bin stays unwritten (0 honored)"))?;
    let time_var = file
        .var(TIME_VAR)
        .ok_or_else(|| format!("{TIME_VAR} absent — the bin stays unwritten (0 honored)"))?;
    let pixels = file.num_values(image);
    if pixels == 0 {
        return Err("the image carries no values per record".to_string());
    }
    let times: HashMap<u32, f64> = file
        .var_records(bytes, time_var)
        .map_err(|n| format!("{TIME_VAR}: {n:?}"))?
        .into_iter()
        .filter_map(|(rec, vals)| vals.first().copied().map(|t| (rec, t)))
        .collect();
    let records = file
        .var_records(bytes, image)
        .map_err(|n| format!("{IMAGE_VAR}: {n:?}"))?;
    let mut frames = Vec::with_capacity(records.len());
    let mut skipped = 0usize;
    for (rec, values) in records {
        let (Some(&epoch), true) = (times.get(&rec), values.len() == pixels) else {
            skipped += 1;
            continue;
        };
        if !value_present(epoch) || !values.iter().all(|v| value_present(*v)) {
            skipped += 1;
            continue;
        }
        let mut frame = Vec::with_capacity(pixels);
        let mut fits = true;
        for v in &values {
            let rounded = v.round();
            if *v != rounded || !(0.0..=65535.0).contains(&rounded) {
                fits = false;
                break;
            }
            frame.push(rounded as u16);
        }
        if !fits {
            skipped += 1;
            continue;
        }
        frames.push((epoch, frame));
    }
    frames.sort_by(|a, b| a.0.total_cmp(&b.0));
    if frames.is_empty() {
        return Err("no finite frame — the bin stays unwritten (0 honored)".to_string());
    }
    eprintln!(
        "{IMAGE_VAR}: {} frames ({} skipped), {pixels} px/frame",
        frames.len(),
        skipped
    );
    Ok(frames)
}

fn emit(bytes: &[u8], out: &str, source: Option<&str>) {
    let label = source.unwrap_or(out);
    let file = match CdfFile::parse(bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{label}: {note:?}");
            std::process::exit(1);
        }
    };
    eprintln!(
        "{label}: CDF v{}.{}.{} encoding {} little {} vars {}",
        file.version.0,
        file.version.1,
        file.version.2,
        file.encoding,
        file.little_endian,
        file.vars.len()
    );
    list_vars(&file);

    let frames = match image_frames(&file, bytes) {
        Ok(f) => f,
        Err(msg) => {
            eprintln!("{label}: {msg}");
            std::process::exit(1);
        }
    };
    let pixels = frames[0].1.len();
    let Some(bin) = write_bin(&frames, pixels) else {
        eprintln!("a frame is not finite — the bin stays unwritten");
        std::process::exit(1);
    };
    match parse_bin(&bin) {
        Some(parsed) if parsed == frames => {}
        _ => {
            eprintln!("the roundtrip does not read back — the bin stays unwritten");
            std::process::exit(1);
        }
    }
    if let Some(parent) = std::path::Path::new(out).parent() {
        if !parent.as_os_str().is_empty() && std::fs::create_dir_all(parent).is_err() {
            eprintln!("create {} void", parent.display());
            std::process::exit(1);
        }
    }
    if std::fs::write(out, &bin).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    eprintln!(
        "{out}: {} frames x {} px, {} B, roundtrip parses",
        frames.len(),
        pixels,
        bin.len()
    );
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    if let Some(src) = source {
        println!("origin {src}");
    }
    println!("compiler {COMPILER}");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
}

fn compile_file(path: &str, out: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    emit(&bytes, out, None);
}

fn compile_url(url: &str, out: &str) {
    let Some(bytes) = fetch_raw_bytes(url) else {
        eprintln!("{url}: fetch void — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    };
    emit(&bytes, out, Some(url));
}

fn probe(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("{path}: the file stays unread");
        std::process::exit(1);
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("{path}: {note:?}");
            std::process::exit(1);
        }
    };
    eprintln!(
        "{path}: CDF v{}.{}.{} encoding {} little {} vars {}",
        file.version.0,
        file.version.1,
        file.version.2,
        file.encoding,
        file.little_endian,
        file.vars.len()
    );
    list_vars(&file);
    match image_frames(&file, &bytes) {
        Ok(frames) => {
            let pixels = frames.first().map(|r| r.1.len()).map(|p| p.to_string());
            let first = frames.first().map(|r| r.0.to_string());
            let last = frames.last().map(|r| r.0.to_string());
            eprintln!(
                "probe: {} frames, {} px/frame, t [{}, {}]",
                frames.len(),
                pixels.as_deref().unwrap_or("absent"),
                first.as_deref().unwrap_or("absent"),
                last.as_deref().unwrap_or("absent"),
            );
        }
        Err(msg) => eprintln!("probe: {msg}"),
    }
}

fn selftest() {
    let Some(bytes) = fetch_raw_bytes(URL) else {
        eprintln!("selftest: {URL} fetch void");
        std::process::exit(1);
    };
    let file = match CdfFile::parse(&bytes) {
        Ok(f) => f,
        Err(note) => {
            eprintln!("selftest: parse void: {note:?}");
            std::process::exit(1);
        }
    };
    eprintln!(
        "selftest: {} bytes, {} vars, little {}",
        bytes.len(),
        file.vars.len(),
        file.little_endian
    );
    let frames = match image_frames(&file, &bytes) {
        Ok(f) => f,
        Err(msg) => {
            eprintln!("selftest: {msg}");
            std::process::exit(1);
        }
    };
    let sample: Vec<(f64, Vec<u16>)> = frames.into_iter().take(4).collect();
    let pixels = sample[0].1.len();
    let Some(bin) = write_bin(&sample, pixels) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != HEADER_BYTES + sample.len() * (8 + pixels * PIXEL_BYTES) {
        eprintln!("selftest: the bin does not carry the measured stride");
        std::process::exit(1);
    }
    if parse_bin(&bin).as_deref() != Some(sample.as_slice()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    eprintln!(
        "themis_asi_compiler: selftest passes ({} frames x {pixels} px, CDF3 read)",
        sample.len()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    if let Some(path) = arg_value(&args, "--probe") {
        probe(&path);
        return;
    }
    let out = match arg_value(&args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };
    if let Some(url) = arg_value(&args, "--url") {
        compile_url(&url, &out);
    } else if let Some(path) = arg_value(&args, "--file").or_else(|| arg_value(&args, "--input")) {
        compile_file(&path, &out);
    } else if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: themis_asi_compiler [--url <cdf> | --file <cdf>] [--out <bin>] [--ci-mode] | --probe <cdf> | --selftest"
        );
        std::process::exit(2);
    } else {
        compile_url(URL, &out);
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrips_the_measured_stride() {
        let frames = vec![
            (1_700_000_000.0, vec![1u16, 2, 3, 4]),
            (1_700_000_003.0, vec![5u16, 6, 7, 8]),
        ];
        let bin = write_bin(&frames, 4).expect("frames encode");
        assert_eq!(bin.len(), HEADER_BYTES + 2 * (8 + 4 * PIXEL_BYTES));
        assert_eq!(parse_bin(&bin), Some(frames));
    }

    #[test]
    fn parse_bin_refuses_foreign_magic_and_short_body() {
        assert!(parse_bin(b"TASI").is_none());
        assert!(parse_bin(b"XXXX").is_none());
        let good = write_bin(&[(1.0, vec![0u16, 0])], 2).expect("a frame encodes");
        assert!(parse_bin(&good[..good.len() - 1]).is_none());
    }

    #[test]
    fn write_bin_refuses_a_mismatched_or_non_finite_frame() {
        assert!(write_bin(&[(f64::NAN, vec![1u16])], 1).is_none());
        assert!(write_bin(&[(1.0, vec![1u16, 2])], 1).is_none());
    }

    #[test]
    fn netcdf_magic_is_not_cdf3() {
        assert_ne!(omegaflow::cdf::MAGIC, *b"CDF\x01");
    }
}
