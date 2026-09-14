use omegaflow::cdn::upload_release;
use omegaflow::zeuge::{FeldIdentitaet, ZeugeArt, magic_identity};
use std::io::{Read, Seek, SeekFrom, Write};
use std::process::Command;

const NETLOC: &str = "www.sciencebase.gov";
const TAR_URL: &str = "https://www.sciencebase.gov/catalog/file/get/5aa1b00ee4b0b1c392e86467?f=__disk__d5%2F91%2F39%2Fd591399bf4f249ab49ffec8a366e5070fe96e0ba";
const MAGIC: [u8; 4] = *b"SLB2";
const REC_BYTES: usize = 24;

const REGIONS: [&str; 27] = [
    "alu", "cal", "cam", "car", "cas", "cot", "hal", "hel", "him", "hin", "izu", "ker", "kur",
    "mak", "man", "mue", "pam", "phi", "png", "puy", "ryu", "sam", "sco", "sol", "sul", "sum",
    "van",
];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn witness_gestalt_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt)) => {
            eprintln!(
                "{} reads as a gestalt witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not gestalt — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn download(url: &str, path: &str) -> Result<(), String> {
    let out = Command::new("curl")
        .arg("-sSfL")
        .arg("--retry")
        .arg("2")
        .arg("--max-time")
        .arg("3600")
        .arg("-o")
        .arg(path)
        .arg(url)
        .output()
        .map_err(|e| format!("curl {url} returned void: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{url}: {} — the volume stays unfetched",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let sz = std::fs::metadata(path)
        .map_err(|e| format!("metadata {path} returned void: {e}"))?
        .len();
    if sz == 0 {
        return Err(format!("{url}: the volume carries no bytes"));
    }
    eprintln!("{url}: {sz} B -> {path}");
    Ok(())
}

fn tar_octal(field: &[u8]) -> Option<usize> {
    let mut v = 0usize;
    let mut any = false;
    for &b in field {
        if b == 0 || b == b' ' {
            break;
        }
        if !(b'0'..=b'7').contains(&b) {
            return None;
        }
        v = v * 8 + (b - b'0') as usize;
        any = true;
    }
    if any { Some(v) } else { None }
}

fn tar_name(header: &[u8]) -> String {
    let end = header[..100].iter().position(|&b| b == 0).unwrap_or(100);
    String::from_utf8_lossy(&header[..end]).to_string()
}

fn is_slab_dep_member(name: &str, region: &str) -> bool {
    name.ends_with(".xyz")
        && name
            .rsplit('/')
            .next()
            .is_some_and(|f| f.contains(&format!("{region}_slab2_dep_")))
}

fn collect_xyz(tar: &[u8], regions: &[&str]) -> Vec<Vec<u8>> {
    let mut found = vec![Vec::new(); regions.len()];
    let mut off = 0usize;
    let mut long_name: Option<String> = None;
    while off + 512 <= tar.len() {
        let header = &tar[off..off + 512];
        if header.iter().all(|&b| b == 0) {
            break;
        }
        let name = match &long_name {
            Some(n) => n.clone(),
            None => tar_name(header),
        };
        let Some(size) = tar_octal(&header[124..136]) else {
            break;
        };
        let typeflag = header[156];
        let data_off = off + 512;
        let data_end = data_off.saturating_add(size);
        if typeflag == b'L' {
            long_name = tar.get(data_off..data_end).map(|d| {
                let e = d.iter().position(|&b| b == 0).unwrap_or(d.len());
                String::from_utf8_lossy(&d[..e]).to_string()
            });
        } else {
            long_name = None;
            if typeflag == b'0' || typeflag == 0 {
                for (i, region) in regions.iter().enumerate() {
                    if is_slab_dep_member(&name, region) {
                        if let Some(data) = tar.get(data_off..data_end) {
                            found[i] = data.to_vec();
                        }
                    }
                }
            }
        }
        off = data_end + ((512 - size % 512) % 512);
    }
    found
}

fn parse_xyz(data: &[u8]) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    for line in data.split(|&b| b == b'\n') {
        let mut parts = line.split(|&b| b == b',');
        let lon = parts.next().and_then(|p| std::str::from_utf8(p).ok());
        let lat = parts.next().and_then(|p| std::str::from_utf8(p).ok());
        let dep = parts.next().and_then(|p| std::str::from_utf8(p).ok());
        let (Some(lon), Some(lat), Some(dep)) = (lon, lat, dep) else {
            continue;
        };
        let (Ok(lon), Ok(lat), Ok(dep)) = (
            lon.trim().parse::<f64>(),
            lat.trim().parse::<f64>(),
            dep.trim().parse::<f64>(),
        ) else {
            continue;
        };
        if !lon.is_finite() || !lat.is_finite() || !dep.is_finite() {
            continue;
        }
        if dep <= 0.0 {
            continue;
        }
        out.push((lat, lon, -dep * 1000.0));
    }
    out
}

fn write_asset(path: &str, records: &[(f64, f64, f64)]) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
    }
    let mut f =
        std::fs::File::create(path).map_err(|e| format!("create {path} returned void: {e}"))?;
    f.write_all(&MAGIC)
        .map_err(|e| format!("write {path} returned void: {e}"))?;
    f.write_all(&(records.len() as u32).to_le_bytes())
        .map_err(|e| format!("write {path} returned void: {e}"))?;
    for (lat, lon, elev) in records {
        f.write_all(&lat.to_le_bytes())
            .and_then(|_| f.write_all(&lon.to_le_bytes()))
            .and_then(|_| f.write_all(&elev.to_le_bytes()))
            .map_err(|e| format!("write {path} returned void: {e}"))?;
    }
    let _ = f.flush();
    Ok(())
}

fn verify_asset(path: &str, count: usize) -> Result<(), String> {
    let mut vf =
        std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut head = [0u8; 8];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {path} header returned void: {e}"))?;
    if head[0..4] != MAGIC {
        return Err(format!("{path}: the magic stays unread"));
    }
    let n = u32::from_le_bytes(head[4..8].try_into().map_err(|_| "count unread")?) as usize;
    if n != count {
        return Err(format!(
            "{path}: {n} records read, {count} written — the asset stays unwritten"
        ));
    }
    if count > 0 {
        vf.seek(SeekFrom::Start(8 + ((count - 1) * REC_BYTES) as u64))
            .map_err(|e| format!("seek {path} returned void: {e}"))?;
        let mut tail = [0u8; REC_BYTES];
        vf.read_exact(&mut tail)
            .map_err(|e| format!("read {path} tail returned void: {e}"))?;
        let elev = f64::from_le_bytes(tail[16..24].try_into().map_err(|_| "depth unread")?);
        if !elev.is_finite() {
            return Err(format!("{path}: the last slab depth stays unread"));
        }
    }
    eprintln!("{path}: {n} slab depth records, roundtrip verified");
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    witness_gestalt_identity(MAGIC)?;
    let out_path = match arg_value(args, "--out") {
        Some(p) => p,
        None => format!("data/{NETLOC}/slab2_depth.bin"),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let mut records: Vec<(f64, f64, f64)> = Vec::new();
    if let Some(file) = arg_value(args, "--file") {
        let bytes = std::fs::read(&file).map_err(|e| format!("read {file} returned void: {e}"))?;
        records = parse_xyz(&bytes);
        if records.is_empty() {
            return Err(format!(
                "{file}: no slab depth measured — the asset stays unwritten"
            ));
        }
    } else {
        let region_arg = arg_value(args, "--region");
        let regions: Vec<&str> = match region_arg.as_deref() {
            Some(code) => {
                if !REGIONS.contains(&code) {
                    return Err(format!(
                        "--region {code}: unknown slab region code — refused"
                    ));
                }
                vec![code]
            }
            None => REGIONS.to_vec(),
        };
        let tar_gz = format!("data/{NETLOC}/Slab2Distribute_Mar2018.tar.gz");
        if let Some(parent) = std::path::Path::new(&tar_gz).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} returned void: {e}", parent.display()))?;
        }
        if !std::path::Path::new(&tar_gz).exists() {
            download(TAR_URL, &tar_gz)?;
        }
        let gz = std::fs::read(&tar_gz).map_err(|e| format!("read {tar_gz} returned void: {e}"))?;
        let mut tar = Vec::new();
        omegaflow::inflate::gunzip_stream(&gz[..], |chunk| tar.extend_from_slice(chunk))
            .map_err(|e| format!("gunzip returned void: {e}"))?;
        let members = collect_xyz(&tar, &regions);
        for (i, region) in regions.iter().enumerate() {
            if members[i].is_empty() {
                return Err(format!(
                    "{region}: the depth grid member stayed unfound in the volume"
                ));
            }
            let recs = parse_xyz(&members[i]);
            eprintln!("{region}: {} slab depth records", recs.len());
            records.extend(recs);
        }
    }
    if records.is_empty() {
        return Err("no slab depth measured — the asset stays unwritten (0 honored)".into());
    }

    write_asset(&out_path, &records)?;
    verify_asset(&out_path, records.len())?;

    if ci_mode && !upload_release(NETLOC, &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("slab2_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tar_block(data: &[u8]) -> [u8; 512] {
        let mut b = [0u8; 512];
        let n = data.len().min(512);
        b[..n].copy_from_slice(&data[..n]);
        b
    }

    fn tar_header(name: &str, size: usize, typeflag: u8) -> [u8; 512] {
        let mut h = [0u8; 512];
        h[..name.len()].copy_from_slice(name.as_bytes());
        let sz = format!("{size:o}\0");
        h[124..124 + sz.len()].copy_from_slice(sz.as_bytes());
        h[156] = typeflag;
        h[257..263].copy_from_slice(b"ustar\0");
        h
    }

    fn build_tar(members: &[(&str, &str)]) -> Vec<u8> {
        let mut out = Vec::new();
        for (name, content) in members {
            out.extend_from_slice(&tar_header(name, content.len(), b'0'));
            out.extend_from_slice(&tar_block(content.as_bytes()));
        }
        out.extend_from_slice(&[0u8; 1024]);
        out
    }

    #[test]
    fn tar_name_reads_until_nul() {
        let mut h = [0u8; 512];
        h[..4].copy_from_slice(b"abcd");
        h[4] = 0;
        assert_eq!(tar_name(&h), "abcd");
    }

    #[test]
    fn tar_octal_parses_size_field() {
        let mut f = [b' '; 12];
        f[..3].copy_from_slice(b"700");
        assert_eq!(tar_octal(&f), Some(448));
    }

    #[test]
    fn collect_xyz_finds_dep_members_only() {
        let tar = build_tar(&[
            (
                "Slab2Distribute_Mar2018/Slab2_TXT/alu_slab2_dep_02.23.18.xyz",
                "161,68,10.5\n162,68,11.0\n",
            ),
            (
                "Slab2Distribute_Mar2018/Slab2_TXT/alu_slab2_dip_02.23.18.xyz",
                "161,68,25.0\n",
            ),
        ]);
        let members = collect_xyz(&tar, &["alu"]);
        assert_eq!(members.len(), 1);
        assert_eq!(
            String::from_utf8_lossy(&members[0]),
            "161,68,10.5\n162,68,11.0\n"
        );
    }

    #[test]
    fn parse_xyz_skips_nan_and_nonpositive_depth() {
        let data = b"161,68,NaN\n161.05,68,10.5\n161.1,68,0.0\n161.15,68,-3.0\n162,68,11.0\n";
        let recs = parse_xyz(data);
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0], (68.0, 161.05, -10500.0));
        assert_eq!(recs[1], (68.0, 162.0, -11000.0));
    }

    #[test]
    fn asset_roundtrip() {
        let recs = vec![(68.0, 161.05, -10500.0), (50.0, 10.0, -30000.0)];
        let path = "/tmp/opencode/slab2_roundtrip_test.bin";
        write_asset(path, &recs).unwrap();
        verify_asset(path, 2).unwrap();
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn verify_refuses_wrong_count() {
        let recs = vec![(68.0, 161.05, -10500.0)];
        let path = "/tmp/opencode/slab2_wrongcount_test.bin";
        write_asset(path, &recs).unwrap();
        assert!(verify_asset(path, 2).is_err());
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn region_code_membership() {
        assert!(REGIONS.contains(&"alu"));
        assert!(!REGIONS.contains(&"foo"));
    }
}
