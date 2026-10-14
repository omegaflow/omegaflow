use omegaflow::cdf::CdfFile;
use omegaflow::cdf25::Cdf25File;
use omegaflow::inflate::gunzip;

const MAGIC3: [u8; 4] = [0xcd, 0xf3, 0x00, 0x01];
const UNCOMPRESSED: u32 = 0x0000_ffff;

fn be_u32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn be_u64(b: &[u8], off: usize) -> Option<u64> {
    Some(u64::from_be_bytes(b.get(off..off + 8)?.try_into().ok()?))
}

fn hex4(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn decompress_whole_file(bytes: &[u8]) -> Option<Vec<u8>> {
    let block = be_u64(bytes, 8)? as usize;
    let data_start = 8 + 32;
    let data_size = block.checked_sub(32)?;
    let cpr = be_u64(bytes, 20)? as usize;
    let cpr_block = be_u64(bytes, cpr)? as usize;
    let cpr_rec = bytes.get(cpr..cpr + cpr_block)?;
    let ctype = be_u32(cpr_rec, 12)?;
    if ctype != 5 {
        return None;
    }
    let stream = bytes.get(data_start..data_start + data_size)?;
    let inner = gunzip(stream)?;
    let mut out = Vec::with_capacity(8 + inner.len());
    out.extend_from_slice(&MAGIC3);
    out.extend_from_slice(&UNCOMPRESSED.to_be_bytes());
    out.extend_from_slice(&inner);
    Some(out)
}

fn report(bytes: &[u8], dump: Option<&str>) {
    let magic: Vec<u8> = bytes.iter().take(4).copied().collect();
    let Some(marker) = be_u32(bytes, 4) else {
        println!("magic: {} (file shorter than 8 bytes)", hex4(&magic));
        return;
    };
    if magic == MAGIC3 {
        println!("magic: {} (CDF 3.0)", hex4(&magic));
        println!("marker bytes 4..8: {marker:#010x}");
        let owned;
        let cdf = if marker == UNCOMPRESSED {
            bytes
        } else {
            println!("whole-file compression: marker != 0x0000ffff");
            match decompress_whole_file(bytes) {
                Some(b) => {
                    println!("decompressed CDF3 body: {} bytes", b.len() - 8);
                    owned = b;
                    &owned
                }
                None => {
                    println!("could not decompress the whole-file CDF");
                    return;
                }
            }
        };
        match CdfFile::parse(cdf) {
            Ok(file) => {
                println!(
                    "cdf: version {}.{}.{} encoding {} little_endian {} vars {}",
                    file.version.0,
                    file.version.1,
                    file.version.2,
                    file.encoding,
                    file.little_endian,
                    file.vars.len()
                );
                for v in &file.vars {
                    println!(
                        "{:<24} num_elements={:<4} data_type={:<3} compressed={} dim_sizes={:?} dim_vary={:?} num_values={}",
                        v.name,
                        v.num_elements,
                        v.data_type,
                        v.compressed,
                        v.dim_sizes,
                        v.dim_vary,
                        file.num_values(v)
                    );
                    let Ok(blocks) = file.blocks(cdf, v) else {
                        continue;
                    };
                    let Some(first) = blocks.first() else {
                        continue;
                    };
                    let Ok(data) = file.block_data(cdf, first) else {
                        continue;
                    };
                    let per =
                        omegaflow::cdf::type_size(v.data_type).unwrap_or(1) * file.num_values(v);
                    if per > 0 && data.len() >= per {
                        let Some(vals) = omegaflow::cdf::decode_record(
                            &data[..per],
                            v.data_type,
                            file.num_values(v),
                            file.little_endian,
                        ) else {
                            continue;
                        };
                        if dump.as_deref() == Some(v.name.as_str()) {
                            for (i, x) in vals.iter().enumerate() {
                                println!("    [{i}] {x}");
                            }
                        }
                        let head: Vec<String> =
                            vals.iter().take(4).map(|x| format!("{x:.6}")).collect();
                        let tail: Vec<String> = vals
                            .iter()
                            .rev()
                            .take(2)
                            .map(|x| format!("{x:.6}"))
                            .collect();
                        println!(
                            "    block0 start={} len={} head={:?} tail={:?}",
                            first.start,
                            vals.len(),
                            head,
                            tail
                        );
                    }
                }
            }
            Err(note) => println!("cdf3 parse: {note:?}"),
        }
        return;
    }
    println!("magic: {} (not CDF 3.0)", hex4(&magic));
    match Cdf25File::parse(bytes) {
        Ok(file) => {
            println!(
                "cdf25: version {}.{}.{} encoding {} vars {}",
                file.version.0,
                file.version.1,
                file.version.2,
                file.encoding,
                file.vars.len()
            );
            for v in &file.vars {
                println!(
                    "{:<24} num_elements={:<4} data_type={:<3} compressed={}",
                    v.name, v.num_elements, v.data_type, v.compressed
                );
            }
        }
        Err(note) => println!("cdf25 parse: {note:?}"),
    }
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: cdf_vars <file.cdf>");
            std::process::exit(2);
        }
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            std::process::exit(2);
        }
    };
    println!("path: {path}");
    println!("bytes: {}", bytes.len());
    let dump = std::env::args().nth(2);
    report(&bytes, dump.as_deref());
}
