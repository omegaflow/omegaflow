use omegaflow::archivar::tiff::parse_tiff;
use std::process::Command;

const NETLOC: &str = "jeodpp.jrc.ec.europa.eu";
const URL: &str = "https://jeodpp.jrc.ec.europa.eu/ftp/jrc-opendata/GHSL/GHS_BUILT_S_GLOBE_R2023A/GHS_BUILT_S_E2020_GLOBE_R2023A_4326_3ss/V1-0/GHS_BUILT_S_E2020_GLOBE_R2023A_4326_3ss_V1_0.zip";

const SIG_LOCAL: [u8; 4] = *b"PK\x03\x04";
const SIG_CENTRAL: [u8; 4] = *b"PK\x01\x02";
const SIG_EOCD: [u8; 4] = *b"PK\x05\x06";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn curl_range(url: &str, range: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .arg("-sS")
        .arg("--max-time")
        .arg("60")
        .arg("-r")
        .arg(range)
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: range {range} reads no bytes — {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    if out.stdout.is_empty() {
        return Err(format!("{url}: range {range} carries no bytes"));
    }
    Ok(out.stdout)
}

fn le16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

fn le32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn eocd(tail: &[u8]) -> Result<(usize, u32, u32), String> {
    let mut found = None;
    if tail.len() >= 4 {
        let mut i = tail.len() - 4;
        loop {
            if tail.get(i..i + 4) == Some(&SIG_EOCD[..]) {
                found = Some(i);
                break;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    let i = found.ok_or("the archive tail carries no EOCD signature")?;
    let entries = le16(tail, i + 10).ok_or("the EOCD carries no entry count")? as usize;
    let cd_size = le32(tail, i + 12).ok_or("the EOCD carries no central-directory size")?;
    let cd_off = le32(tail, i + 16).ok_or("the EOCD carries no central-directory offset")?;
    Ok((entries, cd_off, cd_size))
}

struct ZipMember {
    name: String,
    method: u16,
    compressed: u32,
    uncompressed: u32,
    local_offset: u32,
}

fn zip_members(cd: &[u8], expected: usize) -> Result<Vec<ZipMember>, String> {
    let mut out = Vec::new();
    let mut off = 0usize;
    while off + 46 <= cd.len() {
        if cd.get(off..off + 4) != Some(&SIG_CENTRAL[..]) {
            break;
        }
        let method = le16(cd, off + 10).ok_or("a central-directory entry ends early")?;
        let compressed = le32(cd, off + 20).ok_or("a central-directory entry ends early")?;
        let uncompressed = le32(cd, off + 24).ok_or("a central-directory entry ends early")?;
        let name_len = le16(cd, off + 28).ok_or("a central-directory entry ends early")? as usize;
        let extra_len = le16(cd, off + 30).ok_or("a central-directory entry ends early")? as usize;
        let comment_len =
            le16(cd, off + 32).ok_or("a central-directory entry ends early")? as usize;
        let local_offset = le32(cd, off + 42).ok_or("a central-directory entry ends early")?;
        let name_bytes = cd
            .get(off + 46..off + 46 + name_len)
            .ok_or("a central-directory name ends early")?;
        out.push(ZipMember {
            name: String::from_utf8_lossy(name_bytes).to_string(),
            method,
            compressed,
            uncompressed,
            local_offset,
        });
        off += 46 + name_len + extra_len + comment_len;
    }
    if out.len() != expected {
        return Err(format!(
            "the central directory reads {} members against the EOCD count {expected}",
            out.len()
        ));
    }
    Ok(out)
}

fn payload_start(url: &str, member: &ZipMember) -> Result<usize, String> {
    let end = member.local_offset.saturating_add(255);
    let head = curl_range(url, &format!("{}-{end}", member.local_offset))?;
    if head.get(0..4) != Some(&SIG_LOCAL[..]) {
        return Err(format!(
            "member '{}' carries no local header at {}",
            member.name, member.local_offset
        ));
    }
    let name_len = le16(&head, 26).ok_or("a local header ends early")? as usize;
    let extra_len = le16(&head, 28).ok_or("a local header ends early")? as usize;
    Ok(member.local_offset as usize + 30 + name_len + extra_len)
}

const MISSING_DESIGN: &str = "missing design: how a raster cell value attaches to a coordinate — the Archivar Sample wants a position and a val, so the decision (cell centre as position with the built-up fraction as val, or a query-time lookup grid) is not yet made; until then no source line and no asset is written";

fn probe(url: &str, bbox: Option<&str>) -> Result<(), String> {
    if let Some(b) = bbox {
        eprintln!("requested subset {b}: GHS_BUILT_S is a 4326 3ss global raster");
    }
    let tail = curl_range(url, "-65557")?;
    let (entries, cd_off, cd_size) = eocd(&tail)?;
    let cd_end = cd_off.saturating_add(cd_size.saturating_sub(1));
    let cd = curl_range(url, &format!("{cd_off}-{cd_end}"))?;
    let members = zip_members(&cd, entries)?;
    for m in &members {
        eprintln!(
            "{NETLOC}: member '{}' method {} compressed {} uncompressed {} at {}",
            m.name, m.method, m.compressed, m.uncompressed, m.local_offset
        );
    }
    let tif = members
        .iter()
        .find(|m| m.name.ends_with(".tif"))
        .ok_or("the archive carries no .tif member")?;
    let start = payload_start(url, tif)?;
    if tif.method != 0 {
        return Err(format!(
            "{url}\n  the GeoTIFF sits in zip member '{}' (method {} deflate) as one non-seekable stream from byte {start} — no HTTP range reaches the raster, so a bounded subset stalls at the wrapper and the raster stays unparsed. STOP. {MISSING_DESIGN}",
            tif.name, tif.method
        ));
    }
    Err(format!(
        "{url}\n  the GeoTIFF is stored (method 0) at byte {start}, but a COG tile index is not read here — a per-tile range subset is pending. STOP. {MISSING_DESIGN}"
    ))
}

fn parse_small_tile(path: &str) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {path} returned void: {e}"))?;
    let img = parse_tiff(&bytes).ok_or_else(|| format!("{path}: the TIFF arm reads no raster"))?;
    eprintln!(
        "{path}: {}x{} raster, {} bps x {}, compression {}, {} band(s)",
        img.width,
        img.height,
        img.bits_per_sample
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join("/"),
        img.samples_per_pixel,
        img.compression,
        img.samples_per_pixel
    );
    match &img.geo {
        Some(g) => eprintln!(
            "{path}: geo x0={} y0={} dx={} dy={}",
            g.x0, g.y0, g.dx, g.dy
        ),
        None => eprintln!("{path}: the raster carries no geotransform"),
    }
    let n = img.pixels.len().min(8);
    eprintln!("{path}: first {n} pixel bytes {:?}", &img.pixels[..n]);
    Err(format!(
        "{path}: GHS_BUILT_S E2020 is a static raster — no time axis, so no `epoch value` text is emitted; no covering (lat, lon, value) catalog arm exists, so no per-cell catalog is emitted. STOP. {MISSING_DESIGN}"
    ))
}

fn run(args: &[String]) -> Result<(), String> {
    if let Some(path) = arg_value(args, "--tif") {
        return parse_small_tile(&path);
    }
    let bbox = arg_value(args, "--bbox");
    probe(URL, bbox.as_deref())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("ghsl_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eocd_bytes(entries: u16, cd_size: u32, cd_off: u32) -> Vec<u8> {
        let mut t = Vec::new();
        t.extend_from_slice(&SIG_EOCD);
        t.extend_from_slice(&0u16.to_le_bytes());
        t.extend_from_slice(&0u16.to_le_bytes());
        t.extend_from_slice(&entries.to_le_bytes());
        t.extend_from_slice(&entries.to_le_bytes());
        t.extend_from_slice(&cd_size.to_le_bytes());
        t.extend_from_slice(&cd_off.to_le_bytes());
        t.extend_from_slice(&0u16.to_le_bytes());
        t
    }

    #[test]
    fn eocd_reads_the_trailer() {
        let t = eocd_bytes(3, 336, 12345);
        let (entries, cd_off, cd_size) = eocd(&t).unwrap();
        assert_eq!(entries, 3);
        assert_eq!(cd_off, 12345);
        assert_eq!(cd_size, 336);
    }

    #[test]
    fn zip_member_reads_name_and_method() {
        let mut cd = Vec::new();
        cd.extend_from_slice(&SIG_CENTRAL);
        cd.extend_from_slice(&[0u8; 6]);
        cd.extend_from_slice(&8u16.to_le_bytes());
        cd.extend_from_slice(&[0u8; 4]);
        cd.extend_from_slice(&[0u8; 4]);
        cd.extend_from_slice(&100u32.to_le_bytes());
        cd.extend_from_slice(&200u32.to_le_bytes());
        cd.extend_from_slice(&5u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&0u16.to_le_bytes());
        cd.extend_from_slice(&[0u8; 8]);
        cd.extend_from_slice(&7u32.to_le_bytes());
        cd.extend_from_slice(b"x.tif");
        let m = zip_members(&cd, 1).unwrap();
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].name, "x.tif");
        assert_eq!(m[0].method, 8);
        assert_eq!(m[0].compressed, 100);
        assert_eq!(m[0].uncompressed, 200);
        assert_eq!(m[0].local_offset, 7);
    }

    #[test]
    fn eocd_refuses_a_tail_without_signature() {
        assert!(eocd(b"no archive here").is_err());
    }
}
