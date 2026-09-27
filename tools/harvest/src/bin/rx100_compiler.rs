use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::rx100::{COMP_LUMINANCE, exif_exposure, luminance, parse_bin, write_bin};
use omegaflow::cdn::upload_release;

const NETLOC: &str = "sony-camera-remote";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn run(args: &[String]) -> Result<(), String> {
    let lsk = embedded_lsk().ok_or_else(|| {
        "the embedded naif0012.tls stays unread — the date→TDB step is unavailable".to_string()
    })?;
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let jpeg = arg_value(args, "--jpeg").ok_or_else(|| {
        format!("{NETLOC}: --jpeg <path> names the capture — no record written without it")
    })?;
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => format!("data/{NETLOC}/rx100_luminance.bin"),
    };

    let jpeg_bytes = std::fs::read(&jpeg).map_err(|e| format!("{jpeg}: read void: {e}"))?;
    let (n, t, s, epoch) = exif_exposure(&jpeg_bytes).ok_or_else(|| {
        format!("{jpeg}: EXIF exposure absent — the luminance record is absent (0 honored)")
    })?;
    let lv = luminance(n, t, s).ok_or_else(|| {
        "scene luminance void — absent or implausible exposure (0 honored)".to_string()
    })?;
    let tdb = lsk
        .unix_to_tdb(epoch)
        .ok_or_else(|| "capture time→TDB reads void — the bin stays unwritten".to_string())?;

    let records = vec![(tdb, lv, COMP_LUMINANCE)];
    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&out, &bytes).map_err(|e| format!("write {out} returned void: {e}"))?;
    match parse_bin(&bytes) {
        Some(parsed) if parsed.len() == records.len() => {
            eprintln!(
                "{out}: {} luminance record at {lv:.3} cd/m2, roundtrip parses",
                parsed.len()
            );
            Ok(())
        }
        Some(parsed) => Err(format!(
            "{out}: {} parsed vs {} written — the asset stays unverified",
            parsed.len(),
            records.len()
        )),
        None => Err(format!(
            "{out}: roundtrip parse void — the asset stays unverified"
        )),
    }?;
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: CDN upload did not reach the release"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("rx100_compiler: {msg}");
        std::process::exit(2);
    }
}
