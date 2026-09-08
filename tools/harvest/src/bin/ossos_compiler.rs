use omegaflow::archivar::ossos::{encode_record, OssosRec, MJD_TO_JD};
use omegaflow::cdn::upload_release;

const CDN_TAG: &str = "cdsarc.cds.unistra.fr";
const RECORD_BYTES: usize = 268;

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
    let a_au = field_f64(line, 107, 118)?;
    let sigma_a = field_f64(line, 119, 129)?;
    let e = field_f64(line, 130, 139)?;
    let sigma_e = field_f64(line, 140, 149)?;
    let i_deg = field_f64(line, 150, 157)?;
    let sigma_i = field_f64(line, 158, 165)?;
    let node = field_f64(line, 166, 174)?;
    let peri = field_f64(line, 184, 192)?;
    let tperi_mjd = field_f64(line, 203, 213)?;
    let a_ok = a_au.is_finite() && a_au > 0.0;
    let e_ok = e.is_finite() && e >= 0.0 && e < 1.0;
    if !a_ok || !e_ok {
        return None;
    }
    let mut desig = [b' '; 7];
    desig.copy_from_slice(&line[261..268]);
    let rec = OssosRec {
        desig,
        a_au,
        e,
        i_deg,
        node_deg: node,
        peri_deg: peri,
        tperi_jd: tperi_mjd + MJD_TO_JD,
        sigma_a_au: sigma_a,
        sigma_e,
        sigma_i_deg: sigma_i,
    };
    let mut out = Vec::new();
    encode_record(&rec, &mut out);
    Some(out)
}

fn compile_catalog(input: &str, out_path: &str) -> (usize, usize) {
    let packed = match std::fs::read(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("ossos: read {input}: {e}");
            std::process::exit(1);
        }
    };
    let text = String::from_utf8_lossy(&packed);
    let mut buf = Vec::new();
    let mut written = 0usize;
    let mut skipped = 0usize;
    for line in text.lines() {
        match record_bytes(line.as_bytes()) {
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
            eprintln!("ossos: write {out_path}: {e}");
            std::process::exit(1);
        }
    }
    eprintln!(
        "ossos: {written} records, {skipped} skipped, {} B -> {out_path}",
        buf.len()
    );
    (written, skipped)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("usage: ossos_compiler --input <t3char.dat> --out <ossos_tno.bin> [--ci-mode]");
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
        eprintln!("ossos: upload {out_path} did not reach the CDN");
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
    fn record_rejects_a_short_line() {
        assert!(record_bytes(b"short").is_none());
    }
}
