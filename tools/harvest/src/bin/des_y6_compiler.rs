use omegaflow::archivar::des_y6::{encode_record, DesY6Rec, AU_M, AU_YR_TO_M_S, DES_Y6_EPOCH_JD};
use omegaflow::cdn::upload_release;

const CDN_TAG: &str = "cdsarc.cds.unistra.fr";
const RECORD_BYTES: usize = 611;

fn field_f64(line: &[u8], lo: usize, hi: usize) -> Option<f64> {
    let s = std::str::from_utf8(line.get(lo..hi)?).ok()?;
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok()
}

fn record_bytes(line: &[u8]) -> Option<Vec<u8>> {
    if line.len() < RECORD_BYTES {
        return None;
    }
    let mut desig = [b' '; 12];
    desig.copy_from_slice(&line[0..12]);
    let x_au = field_f64(line, 243, 252)?;
    let y_au = field_f64(line, 253, 263)?;
    let z_au = field_f64(line, 264, 274)?;
    let vx = field_f64(line, 275, 284)?;
    let vy = field_f64(line, 285, 294)?;
    let vz = field_f64(line, 295, 304)?;
    let sigxx = field_f64(line, 305, 316)?;
    let sigyy = field_f64(line, 382, 393)?;
    let sigzz = field_f64(line, 446, 457)?;
    let var = sigxx + sigyy + sigzz;
    if !var.is_finite() || var < 0.0 {
        return None;
    }
    let sigma_au = var.sqrt();
    let rec = DesY6Rec {
        desig,
        epoch_jd: DES_Y6_EPOCH_JD,
        x_m: x_au * AU_M,
        y_m: y_au * AU_M,
        z_m: z_au * AU_M,
        vx_ms: vx * AU_YR_TO_M_S,
        vy_ms: vy * AU_YR_TO_M_S,
        vz_ms: vz * AU_YR_TO_M_S,
        sigma_m: sigma_au * AU_M,
    };
    let mut out = Vec::new();
    encode_record(&rec, &mut out);
    Some(out)
}

fn compile_catalog(input: &str, out_path: &str) -> (usize, usize) {
    let packed = match std::fs::read(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("des_y6: read {input}: {e}");
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&packed);
    let mut buf = Vec::new();
    let mut written = 0usize;
    let mut skipped = 0usize;
    for line in text.lines() {
        let b = line.as_bytes();
        match record_bytes(b) {
            Some(rec) => {
                buf.extend_from_slice(&rec);
                written += 1;
            }
            None => skipped += 1,
        }
    }
    match std::fs::write(out_path, &buf) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("des_y6: write {out_path}: {e}");
            std::process::exit(1);
        }
    }
    eprintln!(
        "des_y6: {written} records, {skipped} skipped, {} B -> {out_path}",
        buf.len()
    );
    (written, skipped)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "usage: des_y6_compiler --input <catalog.dat> --out <des_y6_tno.bin> [--ci-mode]"
        );
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
    let (written, _) = compile_catalog(&input, &out_path);
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("des_y6: upload {out_path} did not reach the CDN");
        std::process::exit(1);
    }
    if written == 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_parses_padded_fixed_width_number() {
        let line = b"  42.5  ";
        assert_eq!(field_f64(line, 0, 8), Some(42.5));
        assert_eq!(field_f64(line, 0, 2), None);
    }

    #[test]
    fn record_rejects_a_short_line() {
        assert!(record_bytes(b"short").is_none());
    }
}
