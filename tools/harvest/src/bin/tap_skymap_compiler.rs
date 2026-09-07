use omegaflow::cdn::upload_asset;
use omegaflow::skymap::{
    decode_rec, encode_rec, parse_header, write_header, SkymapRecord, HEADER_LEN, KIND_GAMMA,
    KIND_GENERIC, REC_BYTES,
};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn parse_f64(cell: &str) -> Option<f64> {
    let t = cell.trim();
    let v: f64 = t.parse().ok()?;
    if v.is_finite() {
        Some(v)
    } else {
        None
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let input =
        match arg_value(args, "--input") {
            Some(v) => v,
            None => return Err(
                "usage: tap_skymap_compiler --input <csv> --ra <col> --dec <col> --value <col> \
                 [--kind <0|1|2|3>] --out <map> [--ci-mode] — refused"
                    .into(),
            ),
        };
    let ra_col = match arg_value(args, "--ra") {
        Some(v) => v,
        None => return Err("--ra <col>: the RA column name is never silent — refused".into()),
    };
    let dec_col = match arg_value(args, "--dec") {
        Some(v) => v,
        None => return Err("--dec <col>: the Dec column name is never silent — refused".into()),
    };
    let value_col = match arg_value(args, "--value") {
        Some(v) => v,
        None => {
            return Err("--value <col>: the scalar column name is never silent — refused".into())
        }
    };
    let kind = match arg_value(args, "--kind").and_then(|k| k.parse::<u8>().ok()) {
        Some(k) if k <= KIND_GAMMA => k,
        Some(_) => return Err("--kind out of range — refused".into()),
        None => KIND_GENERIC,
    };
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => return Err("--out <map>: the asset path is never silent — refused".into()),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let text =
        std::fs::read_to_string(&input).map_err(|e| format!("read {input} returned void: {e}"))?;
    let mut lines = text.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => return Err(format!("{input}: the CSV header stays unread")),
    };
    let cols: Vec<&str> = header.split(',').map(|c| c.trim()).collect();
    let col_idx = |name: &str| cols.iter().position(|c| *c == name);
    let (ira, idec, ival) = match (col_idx(&ra_col), col_idx(&dec_col), col_idx(&value_col)) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => return Err(format!("{input}: a named column is absent from the header")),
    };

    let mut records: Vec<SkymapRecord> = Vec::new();
    let mut census = [0u64; 4];
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split(',').collect();
        let (Some(ra), Some(dec), Some(value)) = (
            cells.get(ira).and_then(|c| parse_f64(c)),
            cells.get(idec).and_then(|c| parse_f64(c)),
            cells.get(ival).and_then(|c| parse_f64(c)),
        ) else {
            census[0] += 1;
            continue;
        };
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
            census[1] += 1;
            continue;
        }
        let (order, ipix) = match SkymapRecord::pixel_of(ra, dec) {
            Some(x) => x,
            None => {
                census[2] += 1;
                continue;
            }
        };
        records.push(SkymapRecord {
            order,
            kind,
            ipix,
            ra_deg: ra as f32,
            dec_deg: dec as f32,
            value: value as f32,
        });
    }
    if records.is_empty() {
        return Err("no decoded row — the asset stays unwritten (0 honored)".into());
    }

    let mut out = BufWriter::with_capacity(
        1 << 20,
        std::fs::File::create(&out_path)
            .map_err(|e| format!("create {out_path} returned void: {e}"))?,
    );
    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, records.len() as u64);
    out.write_all(&hbuf)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let mut rec = [0u8; REC_BYTES];
    for r in &records {
        encode_rec(&mut rec, r);
        out.write_all(&rec)
            .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    }
    let _ = out.flush();
    let _ = out.into_inner();

    let expect = HEADER_LEN as u64 + records.len() as u64 * REC_BYTES as u64;
    let actual = std::fs::metadata(&out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len();
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expected} expected — the asset stays unwritten",
            expected = expect
        ));
    }
    let mut vf = std::fs::File::open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    let n_rows =
        parse_header(&head).ok_or_else(|| format!("{out_path}: the header stays unread"))?;
    let last_off = HEADER_LEN as u64 + (records.len() as u64 - 1) * REC_BYTES as u64;
    vf.seek(SeekFrom::Start(last_off))
        .map_err(|e| format!("seek {out_path} returned void: {e}"))?;
    let mut tail = vec![0u8; REC_BYTES];
    vf.read_exact(&mut tail)
        .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
    let last =
        decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stays unread"))?;

    eprintln!(
        "{out_path}: {} events from {input}, header {} — roundtrip verified",
        records.len(),
        n_rows
    );
    eprintln!(
        "last event: ra {:.4} dec {:.4} value {:.4} kind {}",
        last.ra_deg, last.dec_deg, last.value, last.kind
    );
    eprintln!(
        "skipped: {} bad-cell, {} out-of-range, {} unplaceable",
        census[0], census[1], census[2]
    );
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("tap_skymap_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_f64_accepts_finite_and_refuses_blank() {
        assert_eq!(parse_f64("148.8746"), Some(148.8746));
        assert_eq!(parse_f64(" 2.5 "), Some(2.5));
        assert_eq!(parse_f64(""), None);
        assert_eq!(parse_f64("nan"), None);
    }
}
