use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::pds3_img::{
    ImgRaster, band_means, decode_envi, decode_raster, pack, parse_envi_hdr, parse_image,
    parse_label,
};
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;

const MINIRF_NETLOC: &str = "pds-geosciences.wustl.edu";
const M3_NETLOC: &str = "pds-imaging.jpl.nasa.gov";
const MINIRF_SAMPLE_LABEL: &str = "https://pds-geosciences.wustl.edu/lunar/ch1-orb-l-mrffr-1-pdr-v1/ch1mrf_0xxx/data/sar/00700_00799/level1/fsb_00720_1cd_xhu_84n209_v1.lbl";
const MINIRF_SAMPLE_IMG: &str = "https://pds-geosciences.wustl.edu/lunar/ch1-orb-l-mrffr-1-pdr-v1/ch1mrf_0xxx/data/sar/00700_00799/level1/fsb_00720_1cd_xhu_84n209_v1.img";
const M3_SAMPLE_HDR: &str = "https://pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0003/DATA/20081118_20090214/200811/L1B/M3G20081118T222604_V03_LOC.HDR";
const M3_SAMPLE_IMG: &str = "https://pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0003/DATA/20081118_20090214/200811/L1B/M3G20081118T222604_V03_LOC.IMG";

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

fn asset_name(img_spec: &str) -> String {
    let base = img_spec.rsplit('/').next().unwrap_or(img_spec);
    let last = base.split_once('?').map(|(h, _)| h).unwrap_or(base);
    let stem = last.split('.').next().unwrap_or(last);
    format!("pds3_img_{}.bin", stem.to_ascii_lowercase())
}

fn print_register_lines(asset: &str, netloc: &str) {
    println!("url https://github.com/omegaflow/sources/releases/download/{netloc}/{asset}");
    println!("format pds3_img");
    println!("ttl 604800");
    println!();
}

fn print_inventory(asset: &str, raster: &ImgRaster, note: &str) {
    let names: Vec<&str> = raster
        .band_names
        .iter()
        .filter(|n| !n.is_empty())
        .map(|n| n.as_str())
        .collect();
    let names = if names.is_empty() {
        String::from("unnamed")
    } else {
        names.join(",")
    };
    let present = raster.values.iter().filter(|v| v.is_some()).count();
    let means = band_means(raster).map(|m| m.len());
    eprintln!(
        "{asset}: {note}, {} band(s) x {} line(s) x {} sample(s), bands [{names}], {present} present sample(s), {means:?} band mean(s)",
        raster.bands, raster.lines, raster.samples
    );
}

fn write_asset(
    asset: &str,
    raster: &ImgRaster,
    netloc: &str,
    pair_mode: bool,
    out_dir: Option<&str>,
    ci_mode: bool,
) -> Option<()> {
    let out_path = match (pair_mode, out_dir) {
        (true, Some(f)) => f.to_string(),
        (true, None) => format!("data/{netloc}/pds3_img/{asset}"),
        (false, Some(d)) => format!("{}/{asset}", d.trim_end_matches('/')),
        (false, None) => format!("data/{netloc}/pds3_img/{asset}"),
    };
    let bin = pack(raster);
    let Some(parsed) = parse_image(&bin) else {
        eprintln!("{asset}: packed read void — the image stays unverified (0 honored)");
        return None;
    };
    if parsed != *raster {
        eprintln!("{asset}: roundtrip void — the image stays unverified (0 honored)");
        return None;
    }
    if let Some(parent) = std::path::Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("write {out_path} returned void");
        return None;
    }
    eprintln!(
        "{out_path}: {} byte(s), sha256 {}, roundtrip holds",
        bin.len(),
        sha256_hex(&bin)
    );
    print_register_lines(asset, netloc);
    if ci_mode && !upload_release(netloc, &out_path) {
        eprintln!("{asset}: CDN upload returned void");
        return None;
    }
    Some(())
}

fn compile_raster(
    img_spec: &str,
    label_spec: &str,
    pair_mode: bool,
    out_dir: Option<&str>,
    ci_mode: bool,
) -> Option<String> {
    let Some(label_bytes) = fetch_or_read(label_spec) else {
        eprintln!("label fetch void ({label_spec})");
        return None;
    };
    let Ok(label_text) = std::str::from_utf8(&label_bytes) else {
        eprintln!("label not utf8 ({label_spec})");
        return None;
    };
    let Some(meta) = parse_label(label_text) else {
        eprintln!("label parse void ({label_spec})");
        return None;
    };
    if let Err(reject) = decode_raster(&[], &meta) {
        eprintln!(
            "{label_spec}: {} — the image stays unwritten (0 honored)",
            reject.reason()
        );
        return None;
    }
    let Some(img_bytes) = fetch_or_read(img_spec) else {
        eprintln!("image fetch void ({img_spec})");
        return None;
    };
    let raster = match decode_raster(&img_bytes, &meta) {
        Ok(r) => r,
        Err(reject) => {
            eprintln!(
                "{img_spec}: {} — the image stays unwritten (0 honored)",
                reject.reason()
            );
            return None;
        }
    };
    if raster.values.iter().all(|v| v.is_none()) {
        eprintln!("{img_spec}: no sample survived — the image stays unwritten (0 honored)");
        return None;
    }
    let asset = asset_name(img_spec);
    print_inventory(&asset, &raster, "pds3 raster");
    write_asset(&asset, &raster, MINIRF_NETLOC, pair_mode, out_dir, ci_mode)?;
    Some(asset)
}

fn compile_envi(
    img_spec: &str,
    hdr_spec: &str,
    pair_mode: bool,
    out_dir: Option<&str>,
    ci_mode: bool,
) -> Option<String> {
    let Some(hdr_bytes) = fetch_or_read(hdr_spec) else {
        eprintln!("header fetch void ({hdr_spec})");
        return None;
    };
    let Ok(hdr_text) = std::str::from_utf8(&hdr_bytes) else {
        eprintln!("header not utf8 ({hdr_spec})");
        return None;
    };
    let Some(meta) = parse_envi_hdr(hdr_text) else {
        eprintln!("header parse void ({hdr_spec})");
        return None;
    };
    if meta.envi_data_type.is_none() {
        eprintln!("{hdr_spec}: no ENVI data type — the cube stays unwritten (0 honored)");
        return None;
    }
    if let Err(reject) = decode_envi(&[], &meta) {
        eprintln!(
            "{hdr_spec}: {} — the cube stays unwritten (0 honored)",
            reject.reason()
        );
        return None;
    }
    let Some(img_bytes) = fetch_or_read(img_spec) else {
        eprintln!("image fetch void ({img_spec})");
        return None;
    };
    let raster: ImgRaster = match decode_envi(&img_bytes, &meta) {
        Ok(r) => r,
        Err(reject) => {
            eprintln!(
                "{img_spec}: {} — the cube stays unwritten (0 honored)",
                reject.reason()
            );
            return None;
        }
    };
    if raster.values.iter().all(|v| v.is_none()) {
        eprintln!("{img_spec}: no sample survived — the cube stays unwritten (0 honored)");
        return None;
    }
    let asset = asset_name(img_spec);
    print_inventory(&asset, &raster, "envi cube");
    write_asset(&asset, &raster, M3_NETLOC, pair_mode, out_dir, ci_mode)?;
    Some(asset)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out_arg = arg_value(&args, "--out");
    let img_arg = arg_value(&args, "--img");
    let label_arg = arg_value(&args, "--label");
    let hdr_arg = arg_value(&args, "--hdr");

    let mut written = 0usize;
    match (img_arg, label_arg, hdr_arg) {
        (Some(img), Some(label), None) => {
            if compile_raster(&img, &label, true, out_arg.as_deref(), ci_mode).is_some() {
                written += 1;
            }
        }
        (Some(img), None, Some(hdr)) => {
            if compile_envi(&img, &hdr, true, out_arg.as_deref(), ci_mode).is_some() {
                written += 1;
            }
        }
        (Some(_), Some(_), Some(_)) => {
            eprintln!("--label and --hdr are mutually exclusive");
            std::process::exit(2);
        }
        (Some(_), None, None) | (None, Some(_), _) | (None, None, Some(_)) => {
            eprintln!("--img needs exactly one companion: --label <pds3> or --hdr <envi>");
            std::process::exit(2);
        }
        (None, None, None) => {
            if compile_raster(
                MINIRF_SAMPLE_IMG,
                MINIRF_SAMPLE_LABEL,
                false,
                out_arg.as_deref(),
                ci_mode,
            )
            .is_some()
            {
                written += 1;
            }
            if compile_envi(
                M3_SAMPLE_IMG,
                M3_SAMPLE_HDR,
                false,
                out_arg.as_deref(),
                ci_mode,
            )
            .is_some()
            {
                written += 1;
            }
        }
    }
    if written == 0 {
        eprintln!("no image packed — nothing written (0 honored)");
        std::process::exit(1);
    }
    eprintln!("{written} image(s) packed");
}
