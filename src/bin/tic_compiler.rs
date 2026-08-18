use omegaflow::cdn::upload_asset;
use omegaflow::inflate::gunzip;

// TIC v8.2 binary record (little-endian), fixed stride — one star identification:
//   offset size  field
//   0      8     id: u64        TIC ID
//   8      8     ra_deg: f64    ICRS RA, degrees
//   16     8     dec_deg: f64   ICRS Dec, degrees
//   24     4     pm_ra_masyr: f32  proper motion RA, mas/yr
//   28     4     pm_de_masyr: f32  proper motion Dec, mas/yr
//   32     4     plx_mas: f32      parallax, mas (0.0 = absent, 0 honored)
//   36     4     tmag: f32         TESS magnitude (0.0 = absent)
//   40     4     dist_pc: f32      distance, pc (0.0 = absent)
const TIC_RECORD_STRIDE: usize = 44;

const COL_ID: usize = 0;
const COL_RA: usize = 13;
const COL_DEC: usize = 14;
const COL_PMRA: usize = 16;
const COL_PMDEC: usize = 18;
const COL_PLX: usize = 21;
const COL_TMAG: usize = 60;
const COL_DIST: usize = 79;

fn cell_f64(cells: &[&str], idx: usize) -> Option<f64> {
    let s = cells.get(idx)?.trim();
    if s.is_empty() {
        return None;
    }
    s.parse::<f64>().ok()
}

fn cell_f32(cells: &[&str], idx: usize) -> f32 {
    cell_f64(cells, idx).map(|v| v as f32).unwrap_or(0.0)
}

fn record_bytes(line: &str) -> Option<Vec<u8>> {
    let cells: Vec<&str> = line.split(',').collect();
    if cells.len() < 22 {
        return None;
    }
    let id: u64 = cells.get(COL_ID)?.trim().parse().ok()?;
    let ra = cell_f64(&cells, COL_RA)?;
    let dec = cell_f64(&cells, COL_DEC)?;
    if !ra.is_finite() || !dec.is_finite() || ra < 0.0 || ra >= 360.0 || dec.abs() > 90.0 {
        return None;
    }
    let pm_ra = cell_f32(&cells, COL_PMRA);
    let pm_de = cell_f32(&cells, COL_PMDEC);
    let plx = cell_f32(&cells, COL_PLX);
    let tmag = cell_f32(&cells, COL_TMAG);
    let d = cell_f32(&cells, COL_DIST);
    let dist_pc = if d > 0.0 {
        d
    } else if plx > 0.0 {
        1000.0 / plx
    } else {
        0.0
    };

    let mut out = Vec::with_capacity(TIC_RECORD_STRIDE);
    out.extend_from_slice(&id.to_le_bytes());
    out.extend_from_slice(&ra.to_le_bytes());
    out.extend_from_slice(&dec.to_le_bytes());
    out.extend_from_slice(&pm_ra.to_le_bytes());
    out.extend_from_slice(&pm_de.to_le_bytes());
    out.extend_from_slice(&plx.to_le_bytes());
    out.extend_from_slice(&tmag.to_le_bytes());
    out.extend_from_slice(&dist_pc.to_le_bytes());
    Some(out)
}

fn compile_catalog(input: &str, out_path: &str) -> usize {
    let packed = std::fs::read(input).unwrap_or_else(|e| panic!("read {}: {}", input, e));
    let bytes = gunzip(&packed).unwrap_or_else(|| panic!("gunzip {}", input));
    let text = String::from_utf8_lossy(&bytes);
    let mut buf = Vec::new();
    let mut written = 0usize;
    let mut skipped = 0usize;
    for line in text.split('\n') {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        match record_bytes(line) {
            Some(rec) => {
                buf.extend_from_slice(&rec);
                written += 1;
            }
            None => {
                skipped += 1;
            }
        }
    }
    std::fs::write(out_path, &buf).unwrap_or_else(|e| panic!("write {}: {}", out_path, e));
    eprintln!(
        "tic: {} records, {} skipped, {} B -> {}",
        written,
        skipped,
        buf.len(),
        out_path
    );
    written
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("usage: tic_compiler --input <tic_dec_band.csv.gz> --out <tic.bin> [--ci-mode]");
        std::process::exit(1);
    }
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    let input = match input {
        Some(p) => p,
        None => {
            eprintln!("--input absent");
            std::process::exit(1);
        }
    };
    let out_path = match out {
        Some(p) => p,
        None => {
            eprintln!("--out absent");
            std::process::exit(1);
        }
    };
    compile_catalog(&input, &out_path);
    if ci_mode && !upload_asset(&out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_with(cols: &[(usize, &str)]) -> String {
        let mut cells = vec![""; 125];
        for &(i, v) in cols {
            cells[i] = v;
        }
        cells.join(",")
    }

    #[test]
    fn record_parses_full_row() {
        let line = line_with(&[
            (COL_ID, "1669554699"),
            (1, "20190415"),
            (COL_RA, "277.607570911876"),
            (COL_DEC, "0.012240"),
            (COL_PMRA, "12.3"),
            (COL_PMDEC, "-4.5"),
            (COL_PLX, "2.5"),
            (COL_TMAG, "11.2"),
            (COL_DIST, "400.0"),
        ]);
        let rec = record_bytes(&line).unwrap();
        assert_eq!(rec.len(), TIC_RECORD_STRIDE);
        let id = u64::from_le_bytes(rec[0..8].try_into().unwrap());
        assert_eq!(id, 1669554699);
        let ra = f64::from_le_bytes(rec[8..16].try_into().unwrap());
        assert!((ra - 277.607570911876).abs() < 1e-9);
        let tmag = f32::from_le_bytes(rec[36..40].try_into().unwrap());
        assert!((tmag - 11.2).abs() < 1e-3);
        let dist = f32::from_le_bytes(rec[40..44].try_into().unwrap());
        assert!((dist - 400.0).abs() < 1e-3);
    }

    #[test]
    fn record_honors_zero_parallax_and_missing_dist() {
        let line = line_with(&[
            (COL_ID, "42"),
            (COL_RA, "10.0"),
            (COL_DEC, "0.0"),
            (COL_PLX, "0"),
            (COL_TMAG, "0"),
        ]);
        let rec = record_bytes(&line).unwrap();
        let plx = f32::from_le_bytes(rec[32..36].try_into().unwrap());
        assert_eq!(plx, 0.0);
        let dist = f32::from_le_bytes(rec[40..44].try_into().unwrap());
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn record_derives_dist_from_plx_when_d_absent() {
        let line = line_with(&[
            (COL_ID, "42"),
            (COL_RA, "10.0"),
            (COL_DEC, "0.0"),
            (COL_PLX, "500.0"),
        ]);
        let rec = record_bytes(&line).unwrap();
        let dist = f32::from_le_bytes(rec[40..44].try_into().unwrap());
        assert!((dist - 2.0).abs() < 1e-3);
    }

    #[test]
    fn record_void_on_empty_ra() {
        let line = line_with(&[(COL_ID, "42"), (COL_DEC, "0.0")]);
        assert!(record_bytes(&line).is_none());
    }
}
