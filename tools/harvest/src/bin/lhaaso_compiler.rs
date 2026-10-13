use omegaflow::cdn::upload_release;
use omegaflow::skymap::{
    HEADER_LEN, KIND_GAMMA, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header,
    write_header,
};
use std::io::{BufWriter, Write};

const COMPONENT_WCDA: &str = "WCDA";
const COMPONENT_KM2A: &str = "KM2A";

fn parse_f64(cell: &str) -> Option<f64> {
    let v: f64 = cell.trim().parse().ok()?;
    if v.is_finite() { Some(v) } else { None }
}

fn n0_to_si(component: &str, n0: f64) -> Option<f64> {
    let cm2_per_m2 = 1.0e4_f64;
    let scale = match component {
        COMPONENT_WCDA => 1.0e-13,
        COMPONENT_KM2A => 1.0e-16,
        _ => return None,
    };
    let v = n0 * scale * cm2_per_m2;
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

fn col_index(header: &str, name: &str) -> Option<usize> {
    header
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .position(|c| c == name)
}

fn gather(input: &str, component: &str) -> Result<Vec<SkymapRecord>, String> {
    let text = std::fs::read_to_string(input).map_err(|e| format!("read {input}: {e}"))?;
    let mut lines = text
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'));
    let header = lines
        .next()
        .ok_or_else(|| format!("{input}: header stays unread"))?;
    let (Some(ra_i), Some(dec_i), Some(comp_i), Some(n0_i)) = (
        col_index(header, "Ra").or_else(|| col_index(header, "RAJ2000")),
        col_index(header, "Dec").or_else(|| col_index(header, "DEJ2000")),
        col_index(header, "Comp"),
        col_index(header, "N0"),
    ) else {
        return Err(format!(
            "{input}: Ra/Dec/Comp/N0 column absent from the header"
        ));
    };
    let mut records = Vec::new();
    let mut skipped = 0usize;
    let mut foreign = 0usize;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.trim().is_empty())
            .collect();
        if cells.get(comp_i).copied() != Some(component) {
            foreign += 1;
            continue;
        }
        let (Some(ra), Some(dec), Some(n0)) = (
            cells.get(ra_i).and_then(|c| parse_f64(c)),
            cells.get(dec_i).and_then(|c| parse_f64(c)),
            cells.get(n0_i).and_then(|c| parse_f64(c)),
        ) else {
            skipped += 1;
            continue;
        };
        if !(0.0..=360.0).contains(&ra) || !(-90.0..=90.0).contains(&dec) {
            skipped += 1;
            continue;
        }
        let Some(value) = n0_to_si(component, n0) else {
            skipped += 1;
            continue;
        };
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
            value: value as f32,
        });
    }
    if records.is_empty() {
        return Err(format!(
            "{input}: no {component} source — the asset stays unwritten (0 honored)"
        ));
    }
    eprintln!(
        "lhaaso {component}: {} sources, {} rows skipped, {} other-component rows",
        records.len(),
        skipped,
        foreign
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
        "last source: ra {:.4} dec {:.4} value {:.4e} kind {}",
        last.ra_deg, last.dec_deg, last.value, last.kind
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut input: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut component = COMPONENT_WCDA.to_string();
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
            "--component" => {
                if let Some(v) = args.get(i + 1) {
                    component = v.clone();
                }
                i += 1;
            }
            "--ci-mode" => ci_mode = true,
            _ => {}
        }
        i += 1;
    }
    if component != COMPONENT_WCDA && component != COMPONENT_KM2A {
        eprintln!(
            "lhaaso_compiler: component {component} is neither {COMPONENT_WCDA} nor {COMPONENT_KM2A}"
        );
        std::process::exit(2);
    }
    let usage = "usage: lhaaso_compiler --input <table.tsv> --out <lhaaso_sky1.sky1> [--component WCDA|KM2A] [--ci-mode]";
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
    let records = match gather(&input, &component) {
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
    if ci_mode && !upload_release("vizier.cds.unistra.fr", &out_path) {
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
        assert_eq!(col_index(header, "Ra"), Some(3));
        assert_eq!(col_index(header, "Dec"), Some(4));
        assert_eq!(col_index(header, "N0"), Some(5));
        assert_eq!(col_index(header, "absent"), None);
    }

    #[test]
    fn col_index_reads_the_vizier_asu_tsv_header() {
        let header = "RAJ2000\tDEJ2000\tComp\tN0";
        assert_eq!(col_index(header, "RAJ2000"), Some(0));
        assert_eq!(col_index(header, "DEJ2000"), Some(1));
        assert_eq!(col_index(header, "Comp"), Some(2));
        assert_eq!(col_index(header, "N0"), Some(3));
    }

    #[test]
    fn n0_to_si_scales_each_component_and_refuses_zero() {
        let w = n0_to_si("WCDA", 21.10).unwrap();
        assert!((w - 2.11e-8).abs() / 2.11e-8 < 1e-12);
        let k = n0_to_si("KM2A", 6.23).unwrap();
        assert!((k - 6.23e-12).abs() / 6.23e-12 < 1e-12);
        assert_eq!(n0_to_si("WCDA", 0.0), None);
        assert_eq!(n0_to_si("X", 1.0), None);
    }

    #[test]
    fn gather_keeps_only_the_named_component_scaled() {
        let text = "RAJ2000\tDEJ2000\tComp\tN0\n\
                    001.86\t+57.00\tKM2A\t0.33\n\
                    001.86\t+57.00\tWCDA\t1.00\n";
        let path = std::env::temp_dir().join("lhaaso_component_test.tsv");
        std::fs::write(&path, text).unwrap();
        let p = path.to_str().unwrap();
        let wcda = gather(p, COMPONENT_WCDA).unwrap();
        assert_eq!(wcda.len(), 1);
        let w = wcda[0].value as f64;
        assert!((w - 1.0e-9).abs() / 1.0e-9 < 1e-5);
        let km2a = gather(p, COMPONENT_KM2A).unwrap();
        assert_eq!(km2a.len(), 1);
        let k = km2a[0].value as f64;
        assert!((k - 3.3e-13).abs() / 3.3e-13 < 1e-5);
        let _ = std::fs::remove_file(p);
    }

    #[test]
    fn gather_refuses_a_header_without_comp() {
        let text = "RAJ2000\tDEJ2000\tN0\n001.86\t+57.00\t1.00\n";
        let path = std::env::temp_dir().join("lhaaso_no_comp_test.tsv");
        std::fs::write(&path, text).unwrap();
        let p = path.to_str().unwrap();
        assert!(gather(p, COMPONENT_WCDA).is_err());
        let _ = std::fs::remove_file(p);
    }
}
