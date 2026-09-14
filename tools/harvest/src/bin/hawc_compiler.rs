use omegaflow::cdn::upload_release;
use omegaflow::skymap::{
    HEADER_LEN, KIND_GAMMA, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header,
    write_header,
};
use std::io::{BufWriter, Write};

struct Source {
    ra: Option<f64>,
    dec: Option<f64>,
    best_radius: Option<f64>,
    flux: Option<f64>,
}

fn leading_spaces(line: &str) -> usize {
    line.bytes().take_while(|&b| b == b' ').count()
}

fn key_value(trimmed: &str) -> Option<(&str, &str)> {
    let (k, v) = trimmed.split_once(':')?;
    Some((k.trim(), v.trim()))
}

fn parse_yaml(input: &str) -> Vec<Source> {
    let mut sources: Vec<Source> = Vec::new();
    let mut current: Option<Source> = None;
    let mut pending_radius: Option<f64> = None;
    for raw in input.lines() {
        let line = raw.trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with("- name:") {
            if let Some(s) = current.take() {
                sources.push(s);
            }
            current = Some(Source {
                ra: None,
                dec: None,
                best_radius: None,
                flux: None,
            });
            pending_radius = None;
            continue;
        }
        let indent = leading_spaces(line);
        let Some((k, v)) = key_value(trimmed) else {
            continue;
        };
        if indent <= 6 {
            match k {
                "RA" => {
                    if let (Some(s), Ok(r)) = (current.as_mut(), v.parse::<f64>()) {
                        s.ra = Some(r);
                    }
                }
                "Dec" => {
                    if let (Some(s), Ok(d)) = (current.as_mut(), v.parse::<f64>()) {
                        s.dec = Some(d);
                    }
                }
                _ => {}
            }
        } else {
            let k = k.strip_prefix("- ").unwrap_or(k);
            match k {
                "tested radius" | "assumed radius" => match v.parse::<f64>() {
                    Ok(r) if r.is_finite() && r >= 0.0 => pending_radius = Some(r),
                    _ => pending_radius = None,
                },
                "flux" => {
                    if let (Some(s), Ok(f)) = (current.as_mut(), v.parse::<f64>()) {
                        if f.is_finite() && f > 0.0 {
                            let r = pending_radius.unwrap_or(f64::MAX);
                            let take = s.best_radius.map_or(true, |br| r < br);
                            if take {
                                s.best_radius = Some(r);
                                s.flux = Some(f);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if let Some(s) = current.take() {
        sources.push(s);
    }
    sources
}

fn records_of(sources: Vec<Source>) -> Vec<SkymapRecord> {
    let mut out = Vec::new();
    for s in sources {
        let ra = match s.ra {
            Some(r) if r.is_finite() && (0.0..360.0).contains(&r) => r,
            _ => continue,
        };
        let dec = match s.dec {
            Some(d) if d.is_finite() && (-90.0..=90.0).contains(&d) => d,
            _ => continue,
        };
        let flux = match s.flux {
            Some(f) if f.is_finite() && f > 0.0 => f,
            _ => continue,
        };
        let (order, ipix) = match SkymapRecord::pixel_of(ra, dec) {
            Some(x) => x,
            None => continue,
        };
        out.push(SkymapRecord {
            order,
            kind: KIND_GAMMA,
            ipix,
            ra_deg: ra as f32,
            dec_deg: dec as f32,
            value: flux as f32,
        });
    }
    out
}

fn gather(input: &str) -> Result<Vec<SkymapRecord>, String> {
    let text = std::fs::read_to_string(input).map_err(|e| format!("read {input}: {e}"))?;
    let sources = parse_yaml(&text);
    let total = sources.len();
    let records = records_of(sources);
    if records.is_empty() {
        return Err(format!(
            "{input}: no source with ra/dec and a fiducial flux — the asset stays unwritten (0 honored)"
        ));
    }
    eprintln!(
        "hawc: {} sources, {} rows skipped",
        records.len(),
        total - records.len()
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
        "last source: ra {:.4} dec {:.4} value {:.3e} kind {}",
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
    let usage = "usage: hawc_compiler --input <catalog.yaml> --out <sky1.sky1> [--ci-mode]";
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
            eprintln!("hawc_compiler: {e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = write_asset(&records, &out_path) {
        eprintln!("hawc_compiler: {e}");
        std::process::exit(1);
    }
    if let Err(e) = verify_asset(&out_path, &records) {
        eprintln!("hawc_compiler: {e}");
        std::process::exit(1);
    }
    let bytes = HEADER_LEN + records.len() * REC_BYTES;
    eprintln!(
        "hawc: {} sources, {} B -> {}",
        records.len(),
        bytes,
        out_path
    );
    if ci_mode && !upload_release("data.hawc-observatory.org", &out_path) {
        eprintln!("upload: {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_SOURCES: &str = "\
   - name: 2HWC J0534+220
     RA: 83.6279
     Dec: 22.0243
     TS: 11015.971849
     flux measurements:
        - tested radius: 0.0
          flux: 1.84708e-13
          flux uncertainty: 2.377e-15
          index: -2.58468
          index uncertainty: 0.0114
        - tested radius: 2.0
          flux: 9.99999e-13
          index: -2.0
   - name: 2HWC J0700+143
     RA: 105.117
     Dec: 14.3235
     flux measurements:
        - tested radius: 1.0
          flux: 1.37969e-14
          flux uncertainty: 4.22204e-15
          index: -2.1672
";

    const THREE_SOURCE: &str = "\
   - name: 3HWC J0534+220
     RA: 83.6279
     Dec: 22.0243
     flux measurements:
        - assumed radius: 0.0
          flux: 2.34204e-13
          flux statistical uncertainty up: 1.40459e-15
          index: -2.57949
          index statistical uncertainty up: 0.00465904
";

    #[test]
    fn parses_2hwc_sources_with_fiducial_flux() {
        let records = records_of(parse_yaml(TWO_SOURCES));
        assert_eq!(records.len(), 2);
        assert!((records[0].ra_deg as f64 - 83.6279).abs() < 1e-3);
        assert!((records[0].dec_deg as f64 - 22.0243).abs() < 1e-3);
        assert!((records[0].value as f64 - 1.84708e-13).abs() < 1e-16);
        assert_eq!(records[0].kind, KIND_GAMMA);
        assert!((records[1].value as f64 - 1.37969e-14).abs() < 1e-17);
    }

    #[test]
    fn parses_3hwc_assumed_radius_flavor() {
        let records = records_of(parse_yaml(THREE_SOURCE));
        assert_eq!(records.len(), 1);
        assert!((records[0].value as f64 - 2.34204e-13).abs() < 1e-16);
    }

    #[test]
    fn picks_the_smallest_radius_flux() {
        let yaml = "\
   - name: 2HWC J0631+169
     RA: 97.998
     Dec: 16.9968
     flux measurements:
        - tested radius: 2.0
          flux: 4.8704e-14
        - tested radius: 0.0
          flux: 6.7138e-15
";
        let records = records_of(parse_yaml(yaml));
        assert_eq!(records.len(), 1);
        assert!((records[0].value as f64 - 6.7138e-15).abs() < 1e-18);
    }

    #[test]
    fn drops_source_with_absent_flux() {
        let yaml = "   - name: 2HWC J0000+000\n     RA: 0.0\n     Dec: 0.0\n";
        assert!(records_of(parse_yaml(yaml)).is_empty());
    }

    #[test]
    fn skymap_asset_roundtrips() {
        let records = records_of(parse_yaml(TWO_SOURCES));
        let path = std::env::temp_dir().join("hawc_2hwc_test.sky1");
        let p = path.to_str().unwrap();
        write_asset(&records, p).unwrap();
        verify_asset(p, &records).unwrap();
        let expect = HEADER_LEN + records.len() * REC_BYTES;
        assert_eq!(std::fs::metadata(p).unwrap().len() as usize, expect);
        let _ = std::fs::remove_file(p);
        assert!(records.iter().all(|r| r.kind == KIND_GAMMA));
    }
}
