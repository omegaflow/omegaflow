use omegaflow::archivar::embedded_lsk;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage, FitsTable};
use omegaflow::lsk::days_from_civil;

const NETLOC: &str = "vso.stanford.edu";
const FORMAT: &str = "iris";

const MAGIC: [u8; 4] = *b"IRIS";
const HEADER_BYTES: usize = 8;
const RECORD_BYTES: usize = 20;

#[derive(Clone, Debug, PartialEq)]
struct IrisRecord {
    t: f64,
    value: f64,
    comp: u32,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn iso_unix(s: &str) -> Option<f64> {
    let t = s.trim();
    let (date, time) = t.split_once('T')?;
    let mut dparts = date.split('-');
    let year: i64 = dparts.next()?.parse().ok()?;
    let month: i64 = dparts.next()?.parse().ok()?;
    let day: i64 = dparts.next()?.parse().ok()?;
    let time = time.strip_suffix('Z').unwrap_or(time);
    let mut tparts = time.split(':');
    let hour: f64 = tparts.next()?.parse().ok()?;
    let minute: f64 = tparts.next()?.parse().ok()?;
    let sec: f64 = tparts.next().unwrap_or("0").parse().ok()?;
    let days = days_from_civil(year, month, day)?;
    Some(days as f64 * 86400.0 + hour * 3600.0 + minute * 60.0 + sec)
}

fn header_time(hdr: &FitsHeader) -> Option<f64> {
    for key in ["T_OBS", "DATE_OBS", "DATE-OBS"] {
        if let Some(s) = hdr.str_unescaped(key) {
            if let Some(unix) = iso_unix(&s) {
                return Some(unix);
            }
        }
    }
    None
}

fn find_image(bytes: &[u8]) -> Option<(FitsImage, FitsHeader)> {
    let mut off = 0usize;
    while off + 80 <= bytes.len() {
        let (hdr, _) = FitsHeader::parse(bytes, off)?;
        if hdr.value("XTENSION") == Some("'BINTABLE'") {
            let (_, next) = FitsTable::parse(bytes, off)?;
            off = next;
            continue;
        }
        let (img, next) = FitsImage::parse(bytes, off)?;
        if img.dims[0] > 0 && img.dims[1] > 0 {
            return Some((img, hdr));
        }
        off = next;
    }
    None
}

fn build_records(img: &FitsImage, buf: &[u8], t: f64) -> Vec<IrisRecord> {
    let mut out = Vec::new();
    for z in 0..img.dims[2] {
        for y in 0..img.dims[1] {
            for x in 0..img.dims[0] {
                if let Some(v) = img.value_f64(buf, [x, y, z]) {
                    if v.is_finite() {
                        out.push(IrisRecord {
                            t,
                            value: v,
                            comp: z as u32,
                        });
                    }
                }
            }
        }
    }
    out
}

fn write_bin(records: &[IrisRecord]) -> Option<Vec<u8>> {
    let count = u32::try_from(records.len()).ok()?;
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * RECORD_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for r in records {
        if !r.t.is_finite() || !r.value.is_finite() {
            return None;
        }
        out.extend_from_slice(&r.t.to_le_bytes());
        out.extend_from_slice(&r.value.to_le_bytes());
        out.extend_from_slice(&r.comp.to_le_bytes());
    }
    Some(out)
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<IrisRecord>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(RECORD_BYTES)? {
        return None;
    }
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let value = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let comp = u32::from_le_bytes(bytes.get(off + 16..off + 20)?.try_into().ok()?);
        if !t.is_finite() || !value.is_finite() {
            return None;
        }
        out.push(IrisRecord { t, value, comp });
        off += RECORD_BYTES;
    }
    Some(out)
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let input = arg_value(args, "--input")
        .ok_or_else(|| "--input <fits> absent — the file stays unread".to_string())?;
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => format!("data/{NETLOC}/{FORMAT}.bin"),
    };

    let bytes = std::fs::read(&input).map_err(|e| format!("{input}: read void ({e})"))?;
    let (img, img_hdr) = find_image(&bytes)
        .ok_or_else(|| format!("{input}: carries no readable image array — refused"))?;
    let primary = FitsHeader::parse(&bytes, 0).map(|(h, _)| h);
    let unix = primary
        .as_ref()
        .and_then(header_time)
        .or_else(|| header_time(&img_hdr))
        .ok_or_else(|| {
            format!(
                "{input}: no T_OBS/DATE_OBS reads — the observation time stays unnamed, refused"
            )
        })?;
    let lsk = embedded_lsk()
        .ok_or_else(|| "the embedded naif0012.tls stays unread — no TDB clock".to_string())?;
    let t = lsk
        .unix_to_tdb(unix)
        .ok_or_else(|| format!("{input}: the observation time stays untranslated to TDB"))?;

    let records = build_records(&img, &bytes, t);
    if records.is_empty() {
        return Err(format!(
            "{input}: no finite pixel left the array — the bin stays unwritten (0 honored)"
        ));
    }
    let bin = write_bin(&records)
        .ok_or_else(|| "a held pixel is not finite — the bin stays unwritten".to_string())?;
    match parse_bin(&bin) {
        Some(parsed) if parsed == records => {}
        _ => return Err("the roundtrip does not read back — the bin stays unwritten".to_string()),
    }

    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{FORMAT}.bin");
    println!("origin {input}");
    println!("compiler tools/harvest/src/bin/iris_compiler.rs");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} samples, {} B, roundtrip parses",
        records.len(),
        bin.len()
    );

    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let records = vec![
        IrisRecord {
            t: 100.0,
            value: 25.5,
            comp: 0,
        },
        IrisRecord {
            t: 100.0,
            value: 9.75,
            comp: 1,
        },
    ];
    let Some(bin) = write_bin(&records) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    if bin.len() != HEADER_BYTES + 2 * RECORD_BYTES {
        eprintln!("selftest: the bin does not carry the measured record stride");
        std::process::exit(1);
    }
    if parse_bin(&bin) != Some(records.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    eprintln!("iris_compiler: selftest passes (t/value/comp flat bin, raw pixels)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: iris_compiler --input <iris_level2.fits> [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("iris_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_bin_roundtrips_the_measured_stride() {
        let records = vec![
            IrisRecord {
                t: 100.0,
                value: 25.5,
                comp: 0,
            },
            IrisRecord {
                t: 100.0,
                value: 9.75,
                comp: 1,
            },
        ];
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(bin.len(), HEADER_BYTES + 2 * RECORD_BYTES);
        assert_eq!(parse_bin(&bin), Some(records));
    }

    #[test]
    fn parse_bin_refuses_a_truncated_asset() {
        let records = vec![IrisRecord {
            t: 1.0,
            value: 2.0,
            comp: 3,
        }];
        let bin = write_bin(&records).expect("finite records encode");
        assert_eq!(parse_bin(&bin[..bin.len() - 1]), None);
    }
}
