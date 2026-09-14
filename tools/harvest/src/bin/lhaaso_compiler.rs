use omegaflow::cdn::upload_release;
use omegaflow::skymap::{
    HEADER_LEN, KIND_GAMMA, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header,
    write_header,
};
use std::io::{BufWriter, Write};

fn parse_f64(cell: &str) -> Option<f64> {
    let v: f64 = cell.trim().parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn col_index(header: &str, name: &str) -> Option<usize> {
    header.split(',').map(|c| c.trim()).position(|c| c == name)
}

fn gather(input: &str) -> Result<Vec<SkymapRecord>, String> {
    let text = std::fs::read_to_string(input).map_err(|e| format!("read {input}: {e}"))?;
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| format!("{input}: header stays unread"))?;
    let (Some(ra_i), Some(dec_i), Some(n0_i)) = (
        col_index(header, "Ra"),
        col_index(header, "Dec"),
        col_index(header, "N0"),
    ) else {
        return Err(format!("{input}: Ra/Dec/N0 column absent from the header"));
    };
    let mut records = Vec::new();
    let mut skipped = 0usize;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split(',').collect();
        let (Some(ra), Some(dec), Some(n0)) = (
            cells.get(ra_i).and_then(|c| parse_f64(c)),
            cells.get(dec_i).and_then(|c| parse_f64(c)),
            cells.get(n0_i).and_then(|c| parse_f64(c)),
        ) else {
            skipped += 1;
            continue;
        };
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) || !(n0 > 0.0) {
            skipped += 1;
            continue;
        }
        let (order, ipix) = match SkymapRecord::pixel_of(ra, dec) {
            Some(x) => x,
            None => {
                skipped += 1;
                continue;
            }
        };
        records.push(SkymapRecord {
            order,
            kind: KIND_GAMMA,
            ipix,
            ra_deg: ra as f32,
            dec_deg: dec as f32,
            value: n0 as f32,
        });
    }
    if records.is_empty() {
        return Err(format!(
            "{input}: no valid source — the asset stays unwritten (0 honored)"
        ));
    }
    eprintln!(
        "lhaaso: {} sources, {} rows skipped",
        records.len(),
        skipped
    );
    Ok(records)
}

fn write_asset(records: &[SkymapRecord], out_path: &str) -> Result<usize, String> {
    let file = std::fs::File::create(out_path).map_err(|e| format!("create {out_path}: {e}"))?;
    let mut out = BufWriter::new(file);
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, records.len() as u64);
    out.write_all(&hbuf)
        .map_err(|e| format!("write {out_path}: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for r in records {
        encode_rec(&mut rec, r);
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path}: {e}"))?;
    }
    out.flush().map_err(|e| format!("flush {out_path}: {e}"))?;
    let expect = HEADER_LEN + records.len() * REC_BYTES;
    let actual = std::fs::metadata(out_path)
        .map_err(|e| format!("stat {out_path}: {e}"))?
        .len() as usize;
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }
    Ok(expect)
}

fn verify_asset(out_path: &str, records: &[SkymapRecord]) -> Result<(), String> {
    let bytes = std::fs::read(out_path).map_err(|e| format!("read {out_path}: {e}"))?;
    let n = parse_header(&bytes).ok_or_else(|| format!("{out_path}: header stays unread"))?;
    if n != records.len() as u64 {
        return Err(format!("{out_path}: {n} rows, {} expected", records.len()));
    }
    let last_off = HEADER_LEN + (records.len() - 1) * REC_BYTES;
    let last = decode_rec(&bytes[last_off..last_off + REC_BYTES])
        .ok_or_else(|| format!("{out_path}: last record stays unread"))?;
    eprintln!(
        "last source: ra {:.4} dec {:.4} value {:.4} kind {}",
        last.ra_deg, last.dec_deg, last.value, last.kind
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut input: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut ci_mode = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--out" => {
                out_path = args.get(i + 1).cloned();
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    let usage = "usage: lhaaso_compiler --input <table.csv> --out <lhaaso_sky1.sky1> [--ci-mode]";
    let input = match input {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };
    let out_path = match out_path {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };
    let records = match gather(&input) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("lhaaso_compiler: {e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = write_asset(&records, &out_path) {
        eprintln!("lhaaso_compiler: {e}");
        std::process::exit(1);
    }
    if let Err(e) = verify_asset(&out_path, &records) {
        eprintln!("lhaaso_compiler: {e}");
        std::process::exit(1);
    }
    let bytes = HEADER_LEN + records.len() * REC_BYTES;
    eprintln!(
        "lhaaso: {} sources, {} B -> {}",
        records.len(),
        bytes,
        out_path
    );
    if ci_mode && !upload_release("casdc.china-vo.org", &out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_f64_accepts_finite_and_refuses_blank() {
        assert_eq!(parse_f64("1.86"), Some(1.86));
        assert_eq!(parse_f64(" 57 "), Some(57.0));
        assert_eq!(parse_f64(""), None);
        assert_eq!(parse_f64("nan"), None);
    }

    #[test]
    fn col_index_finds_trimmed_header() {
        let header = "Source name, components, Ra, Dec, N0, index";
        assert_eq!(col_index(header, "Ra"), Some(2));
        assert_eq!(col_index(header, "Dec"), Some(3));
        assert_eq!(col_index(header, "N0"), Some(4));
        assert_eq!(col_index(header, "absent"), None);
    }
}
