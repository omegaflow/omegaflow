use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_img::{
    ImgMeta, ImgRaster, band_means, decode_raster, pack, parse_image, parse_label,
};
use omegaflow::archivar::pds3_table::odl_kv;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const NETLOC: &str = "pds-geosciences.wustl.edu";
const COMPILER: &str = "tools/harvest/src/bin/pds3_bip_radar_compiler.rs";
const FORMAT: &str = "pds3_img";
const STORAGE: &str = "SAMPLE_INTERLEAVED";

const BIP_SAMPLE_LABEL: &str = "PDS_VERSION_ID = PDS3
RECORD_TYPE = FIXED_LENGTH
RECORD_BYTES = 16
LINES = 2
LINE_SAMPLES = 1
BANDS = 2
SAMPLE_TYPE = PC_REAL
SAMPLE_BITS = 32
BAND_STORAGE_TYPE = SAMPLE_INTERLEAVED
BAND_NAME = (\"REAL COMPONENT\", \"IMAGINARY COMPONENT\")
END
";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fetch_or_read(spec: &str) -> Option<Vec<u8>> {
    if spec.starts_with("http://") || spec.starts_with("https://") {
        fetch_raw_bytes(spec)
    } else {
        std::fs::read(spec).ok()
    }
}

fn storage_type(label: &str) -> Option<String> {
    odl_kv(label)
        .into_iter()
        .find(|(k, _)| k == "BAND_STORAGE_TYPE")
        .map(|(_, v)| v.to_ascii_uppercase())
}

fn sample_interleaved_extent(meta: &ImgMeta) -> Result<usize, String> {
    let lines = meta.lines.ok_or("LINES absent — the raster stays unread")?;
    let samples = meta
        .line_samples
        .ok_or("LINE_SAMPLES absent — the raster stays unread")?;
    let bands = meta.bands.ok_or("BANDS absent — the raster stays unread")?;
    let bits = meta
        .sample_bits
        .ok_or("SAMPLE_BITS absent — the raster stays unread")?;
    if lines == 0 || samples == 0 || bands == 0 {
        return Err("LINES/LINE_SAMPLES/BANDS carry zero — the raster stays unread".to_string());
    }
    if bits == 0 || !bits.is_multiple_of(8) {
        return Err(format!(
            "SAMPLE_BITS {bits} not byte-aligned — the raster stays unread"
        ));
    }
    let width = (bits / 8) as usize;
    let prefix = meta.line_prefix_bytes.unwrap_or(0usize);
    let offset = meta.offset.unwrap_or(0usize);
    let row = samples
        .checked_mul(bands)
        .and_then(|v| v.checked_mul(width))
        .and_then(|v| v.checked_add(prefix))
        .ok_or("SAMPLE_INTERLEAVED row exceeds addressable memory")?;
    let extent = lines
        .checked_mul(row)
        .ok_or("SAMPLE_INTERLEAVED extent exceeds addressable memory")?;
    offset
        .checked_add(extent)
        .ok_or_else(|| "SAMPLE_INTERLEAVED extent exceeds addressable memory".to_string())
}

fn asset_name(img_spec: &str) -> String {
    let base = img_spec.rsplit('/').next().unwrap_or(img_spec);
    let last = base.split_once('?').map(|(h, _)| h).unwrap_or(base);
    let stem = last.split('.').next().unwrap_or(last);
    format!("pds3_bip_{}.bin", stem.to_ascii_lowercase())
}

fn print_register_lines(
    asset: &str,
    label_spec: &str,
    img_spec: &str,
    sha: &str,
    raster: &ImgRaster,
) {
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{asset}");
    println!("terms PD https://pds-geosciences.wustl.edu/");
    println!("format {FORMAT}");
    println!("origin {label_spec}");
    println!("origin {img_spec}");
    println!("compiler {COMPILER}");
    println!("sha256 {sha}");
    println!("at venus");
    println!("ttl 604800");
    for name in raster.band_names.iter().filter(|n| !n.is_empty()) {
        println!("field \"{name}\"");
    }
}

fn compile(
    label_spec: &str,
    img_spec: &str,
    out: Option<&str>,
    ci_mode: bool,
) -> Result<(), String> {
    let label_bytes =
        fetch_or_read(label_spec).ok_or_else(|| format!("label fetch void ({label_spec})"))?;
    let label_text =
        std::str::from_utf8(&label_bytes).map_err(|_| format!("label not utf8 ({label_spec})"))?;
    let storage = storage_type(label_text).ok_or_else(|| {
        format!("{label_spec}: BAND_STORAGE_TYPE absent — the raster stays unread")
    })?;
    if storage != STORAGE {
        return Err(format!(
            "{label_spec}: BAND_STORAGE_TYPE {storage} is not {STORAGE} — this arm reads only sample-interleaved radar products"
        ));
    }
    let meta = parse_label(label_text)
        .ok_or_else(|| format!("{label_spec}: label parse void — the raster stays unread"))?;
    let img_bytes =
        fetch_or_read(img_spec).ok_or_else(|| format!("image fetch void ({img_spec})"))?;
    let extent = sample_interleaved_extent(&meta)?;
    if img_bytes.len() < extent {
        return Err(format!(
            "{img_spec}: label demands {extent} bytes, the file carries {} — the raster stays unread",
            img_bytes.len()
        ));
    }
    let raster =
        decode_raster(&img_bytes, &meta).map_err(|e| format!("{img_spec}: {}", e.reason()))?;
    if raster.values.iter().all(|v| v.is_none()) {
        return Err(format!(
            "{img_spec}: no sample survived — the image stays unwritten (0 honored)"
        ));
    }
    let band_count = band_means(&raster).map(|m| m.len());
    eprintln!(
        "{img_spec}: SAMPLE_INTERLEAVED radar, {} band(s) x {} line(s) x {} sample(s), bands {:?}, {band_count:?} band mean(s)",
        raster.bands, raster.lines, raster.samples, raster.band_names
    );
    let asset = asset_name(img_spec);
    let bin = pack(&raster);
    match parse_image(&bin) {
        Some(parsed) if parsed == raster => {}
        _ => {
            return Err(format!(
                "{asset}: packed roundtrip void — the image stays unverified (0 honored)"
            ));
        }
    }
    let out_path = match out {
        Some(p) if !p.is_empty() => p.to_string(),
        _ => format!("data/{NETLOC}/{asset}"),
    };
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out_path, &bin).map_err(|e| format!("write {out_path} void: {e}"))?;
    let sha = sha256_hex(&bin);
    eprintln!(
        "{out_path}: {} byte(s), sha256 {sha}, roundtrip holds",
        bin.len()
    );
    print_register_lines(&asset, label_spec, img_spec, &sha, &raster);
    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let meta = match parse_label(BIP_SAMPLE_LABEL) {
        Some(m) => m,
        None => {
            eprintln!("selftest: the sample label does not parse");
            std::process::exit(1);
        }
    };
    if storage_type(BIP_SAMPLE_LABEL).as_deref() != Some(STORAGE) {
        eprintln!("selftest: BAND_STORAGE_TYPE does not read back");
        std::process::exit(1);
    }
    let mut raw = Vec::new();
    for v in [1.0f32, 2.0, 3.0, 4.0] {
        raw.extend_from_slice(&v.to_le_bytes());
    }
    let raster = match decode_raster(&raw, &meta) {
        Ok(r) => r,
        Err(reject) => {
            eprintln!("selftest: decode void ({})", reject.reason());
            std::process::exit(1);
        }
    };
    let expected = vec![Some(1.0), Some(3.0), Some(2.0), Some(4.0)];
    if raster.values != expected {
        eprintln!(
            "selftest: the band-major order does not read back ({:?})",
            raster.values
        );
        std::process::exit(1);
    }
    let bin = pack(&raster);
    if parse_image(&bin) != Some(raster) {
        eprintln!("selftest: the packed roundtrip does not hold");
        std::process::exit(1);
    }
    eprintln!("pds3_bip_radar_compiler: selftest passes (SAMPLE_INTERLEAVED → band-major P3IM)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: pds3_bip_radar_compiler --label <pds3.lbl> --img <file.img|url> [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = arg_value(&args, "--out");
    let (Some(label), Some(img)) = (arg_value(&args, "--label"), arg_value(&args, "--img")) else {
        eprintln!("--label <pds3.lbl> and --img <file.img|url> are both required");
        std::process::exit(2);
    };
    if let Err(msg) = compile(&label, &img, out.as_deref(), ci_mode) {
        eprintln!("pds3_bip_radar_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_reads_sample_interleaved_to_band_major() {
        let meta = parse_label(BIP_SAMPLE_LABEL).expect("label parses");
        let mut raw = Vec::new();
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            raw.extend_from_slice(&v.to_le_bytes());
        }
        let raster = decode_raster(&raw, &meta).expect("decodes");
        assert_eq!(
            raster.values,
            vec![Some(1.0), Some(3.0), Some(2.0), Some(4.0)]
        );
    }

    #[test]
    fn sample_interleaved_extent_holds_the_full_sample_interleaved_row() {
        let meta = parse_label(BIP_SAMPLE_LABEL).expect("label parses");
        assert_eq!(sample_interleaved_extent(&meta), Ok(16));
    }

    #[test]
    fn storage_type_reads_the_label_directive() {
        assert_eq!(storage_type(BIP_SAMPLE_LABEL).as_deref(), Some(STORAGE));
        assert_eq!(
            storage_type("BAND_STORAGE_TYPE = BAND_SEQUENTIAL\n"),
            Some("BAND_SEQUENTIAL".to_string())
        );
    }
}
