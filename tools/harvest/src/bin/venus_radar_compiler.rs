use omegaflow::archivar::pds4_img;

const MAGIC: [u8; 4] = *b"VNRD";
const HEADER_BYTES: usize = 8;
const REC_BYTES: usize = 20;

fn write_series(rows: &[(f64, f64, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + rows.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for &(t, v, comp) in rows {
        out.extend_from_slice(&t.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
        out.extend_from_slice(&comp.to_le_bytes());
    }
    out
}

fn read_series(bytes: &[u8]) -> Option<Vec<(f64, f64, u32)>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    if bytes.len() != HEADER_BYTES + count.checked_mul(REC_BYTES)? {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    let mut off = HEADER_BYTES;
    for _ in 0..count {
        let t = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let v = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let comp = u32::from_le_bytes(bytes.get(off + 16..off + 20)?.try_into().ok()?);
        if !t.is_finite() || !v.is_finite() {
            return None;
        }
        out.push((t, v, comp));
        off += REC_BYTES;
    }
    Some(out)
}

fn usage() {
    eprintln!("usage: venus_radar_compiler <input.img> <output.bin>");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        usage();
        std::process::exit(2);
    }
    let (Some(input), Some(output)) = (args.first(), args.get(1)) else {
        eprintln!("venus_radar_compiler: <input.img> and <output.bin> are both required");
        usage();
        std::process::exit(2);
    };
    let bytes = match std::fs::read(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("venus_radar_compiler: read {input}: {e} — the tile stays unread");
            std::process::exit(2);
        }
    };
    let expected = pds4_img::LINE_COUNT * pds4_img::SAMPLE_COUNT * pds4_img::ELEMENT_BYTES;
    if bytes.len() != expected {
        eprintln!(
            "venus_radar_compiler: {input} carries {} byte(s), the declared Array_2D_Map needs {expected} — the tile stays unread",
            bytes.len()
        );
        std::process::exit(2);
    }
    let Some(rows) = pds4_img::parse_series(&bytes) else {
        eprintln!(
            "venus_radar_compiler: {input} carries no present line — the series stays unwritten (0 honored)"
        );
        std::process::exit(2);
    };
    let bin = write_series(&rows);
    let Some(back) = read_series(&bin) else {
        eprintln!(
            "venus_radar_compiler: the packed series does not read back — the asset stays unverified"
        );
        std::process::exit(1);
    };
    if let Some(parent) = std::path::Path::new(output).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    if std::fs::write(output, &bin).is_err() {
        eprintln!("venus_radar_compiler: write {output} void");
        std::process::exit(1);
    }
    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;
    for &(_, v, _) in &rows {
        if v < vmin {
            vmin = v;
        }
        if v > vmax {
            vmax = v;
        }
    }
    println!(
        "{output}: {} line mean(s), {:.4e}..{:.4e}, {} byte(s), roundtrip {} point(s)",
        rows.len(),
        vmin,
        vmax,
        bin.len(),
        back.len()
    );
}
