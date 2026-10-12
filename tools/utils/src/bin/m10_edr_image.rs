use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::inflate::{gunzip, tar_members};

const URL: &str = "https://ser.im-ldi.com/M10/MVM/mvm_0001.tgz";
const REC: usize = 968;
const LINES: usize = 700;
const PIXELS: usize = 832;
const IMAGE_BYTES: usize = 726 * REC;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

fn adler32(data: &[u8]) -> u32 {
    let mut a = 1u32;
    let mut b = 0u32;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    let chunks: Vec<&[u8]> = data.chunks(65535).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        out.push(if i + 1 == chunks.len() { 1 } else { 0 });
        let len = chunk.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn chunk(kind: &[u8; 4], body: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(body);
    let mut crc_input = Vec::with_capacity(4 + body.len());
    crc_input.extend_from_slice(kind);
    crc_input.extend_from_slice(body);
    out.extend_from_slice(&crc32(&crc_input).to_be_bytes());
}

fn png_gray8(width: usize, height: usize, raster: &[u8]) -> Vec<u8> {
    let mut raw = Vec::with_capacity(height * (1 + width));
    for row in 0..height {
        raw.push(0);
        raw.extend_from_slice(&raster[row * width..(row + 1) * width]);
    }
    let mut out = Vec::new();
    out.extend_from_slice(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&(width as u32).to_be_bytes());
    ihdr.extend_from_slice(&(height as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 0, 0, 0, 0]);
    chunk(b"IHDR", &ihdr, &mut out);
    chunk(b"IDAT", &zlib_stored(&raw), &mut out);
    chunk(b"IEND", &[], &mut out);
    out
}

fn raster_of(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() != IMAGE_BYTES {
        return None;
    }
    let mut raster = Vec::with_capacity(PIXELS * LINES);
    for r in 0..LINES {
        let base = REC * (r + 1);
        raster.extend_from_slice(bytes.get(base..base + PIXELS)?);
    }
    Some(raster)
}

fn run(args: &[String]) -> Result<(), String> {
    let out = match arg_value(args, "--out") {
        Some(v) => v,
        None => "m10_edr".to_string(),
    };
    let count = match arg_value(args, "--n") {
        Some(v) => v.parse::<usize>().map_err(|e| format!("--n {v} void: {e}"))?,
        None => 1,
    };

    let gz = match arg_value(args, "--in") {
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("read {path} returned void: {e}"))?
        }
        None => fetch_raw_bytes(URL).ok_or_else(|| format!("{URL}: fetch void"))?,
    };

    if gz.len() == IMAGE_BYTES {
        let raster = raster_of(&gz).ok_or_else(|| "raw image does not decode".to_string())?;
        let png = png_gray8(PIXELS, LINES, &raster);
        let path = format!("{out}_0000.png");
        std::fs::write(&path, &png).map_err(|e| format!("write {path} returned void: {e}"))?;
        eprintln!(
            "{path}: {}x{} gray8, {} B, sha256 {} (raw image)",
            PIXELS,
            LINES,
            png.len(),
            sha256_hex(&png)
        );
        return Ok(());
    }

    let tar = gunzip(&gz).ok_or_else(|| format!("{URL}: gzip stream does not decode"))?;
    let members =
        tar_members(&tar).ok_or_else(|| format!("{URL}: tar stream does not enumerate"))?;

    let mut written = 0usize;
    for m in &members {
        if written >= count {
            break;
        }
        if !m.name.contains("IMAGES/") || m.end.saturating_sub(m.start) != IMAGE_BYTES {
            continue;
        }
        let Some(data) = tar.get(m.start..m.end) else {
            continue;
        };
        let Some(raster) = raster_of(data) else {
            continue;
        };
        let png = png_gray8(PIXELS, LINES, &raster);
        let path = format!("{out}_{written:04}.png");
        std::fs::write(&path, &png).map_err(|e| format!("write {path} returned void: {e}"))?;
        eprintln!(
            "{path}: {}x{} gray8, {} B, sha256 {} (edr {})",
            PIXELS,
            LINES,
            png.len(),
            sha256_hex(&png),
            m.name
        );
        written += 1;
    }
    if written == 0 {
        return Err("no image extracted — nothing written (0 honored)".to_string());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("usage: m10_edr_image [--in <mvm.tgz>] [--out <stem>] [--n <count>]");
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("m10_edr_image: {msg}");
        std::process::exit(1);
    }
}
