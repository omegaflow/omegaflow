use omegaflow::archivar::fetch_raw;
use omegaflow::cdn::upload_release;

const MAGIC: &[u8; 4] = b"EMTF";
const FIELDS: usize = 9;
const NETLOC: &str = "data.earthscope.org";
const OUT_PATH: &str = "emtf_usarray.bin";
const Z_MT_TO_SI: f64 = 4.0 * std::f64::consts::PI * 1e-4;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn write_bin(records: &[[f64; FIELDS]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + records.len() * FIELDS * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for r in records {
        for v in r {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }
    out
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<[f64; FIELDS]>> {
    if bytes.get(0..4)? != MAGIC {
        return None;
    }
    let n = u32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?) as usize;
    if bytes.len() != 8 + n * FIELDS * 8 {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut r = [0.0f64; FIELDS];
        for (j, slot) in r.iter_mut().enumerate() {
            let off = 8 + i * FIELDS * 8 + j * 8;
            *slot = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        }
        out.push(r);
    }
    Some(out)
}

fn field_value(text: &str) -> Option<f64> {
    let v: f64 = text.trim().parse().ok()?;
    v.is_finite().then_some(v)
}

fn complex_value(text: &str) -> Option<(f64, f64)> {
    let mut it = text.split_whitespace();
    let re = field_value(it.next()?)?;
    let im = field_value(it.next()?)?;
    Some((re, im))
}

fn attr_value(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = tag[start..].find('"')? + start;
    Some(tag[start..end].to_string())
}

fn element<'a>(scope: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = scope.find(open)?;
    let after = &scope[start..];
    let end = after.find(close)?;
    Some(&after[..end])
}

fn tagged_complex(scope: &str, name: &str) -> Option<(f64, f64)> {
    let needle = format!("name=\"{name}\"");
    let at = scope.find(&needle)?;
    let rest = &scope[at..];
    let gt = rest.find('>')?;
    let text = &rest[gt + 1..];
    let lt = text.find('<')?;
    complex_value(&text[..lt])
}

fn period_blocks(body: &str) -> Vec<&str> {
    let mut blocks = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("<Period") {
        let after = &rest[start..];
        let Some(end) = after.find("</Period>") else {
            break;
        };
        blocks.push(&after[..end]);
        rest = &after[end + "</Period>".len()..];
    }
    blocks
}

fn parse_emtf(body: &str) -> Vec<[f64; FIELDS]> {
    let mut records = Vec::new();
    for block in period_blocks(body) {
        let Some(period) = attr_value(block, "value").and_then(|v| field_value(&v)) else {
            continue;
        };
        if !(period > 0.0) {
            continue;
        }
        let Some(z) = element(block, "<Z ", "</Z>") else {
            continue;
        };
        let (Some(zxx), Some(zxy), Some(zyx), Some(zyy)) = (
            tagged_complex(z, "Zxx"),
            tagged_complex(z, "Zxy"),
            tagged_complex(z, "Zyx"),
            tagged_complex(z, "Zyy"),
        ) else {
            continue;
        };
        records.push([
            period,
            zxx.0 * Z_MT_TO_SI,
            zxx.1 * Z_MT_TO_SI,
            zxy.0 * Z_MT_TO_SI,
            zxy.1 * Z_MT_TO_SI,
            zyx.0 * Z_MT_TO_SI,
            zyx.1 * Z_MT_TO_SI,
            zyy.0 * Z_MT_TO_SI,
            zyy.1 * Z_MT_TO_SI,
        ]);
    }
    records
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => OUT_PATH.to_string(),
    };

    let body = if let Some(path) =
        arg_value(&args, "--file").or_else(|| arg_value(&args, "--input"))
    {
        match std::fs::read_to_string(&path) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("{path}: the file stays unread");
                std::process::exit(1);
            }
        }
    } else if let Some(url) = arg_value(&args, "--url") {
        match fetch_raw(&url, None, &[]) {
            Some(b) => b,
            None => {
                eprintln!("{url} fetch void — the bin stays unwritten (0 honored)");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "usage: emtf_compiler --url <emtf.xml> | --file <emtf.xml> [--out <bin>] [--ci-mode]"
        );
        std::process::exit(2);
    };

    let mut records = parse_emtf(&body);
    records.sort_by(|a, b| a[0].total_cmp(&b[0]));
    eprintln!("EMTF: {} periods", records.len());
    if records.is_empty() {
        eprintln!("no records — the bin stays unwritten (0 honored)");
        std::process::exit(1);
    }
    let bytes = write_bin(&records);
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("create {} void: {e}", parent.display());
                std::process::exit(1);
            }
        }
    }
    if std::fs::write(&out, &bytes).is_err() {
        eprintln!("write {out} returned void");
        std::process::exit(1);
    }
    match parse_bin(&bytes) {
        Some(parsed) => {
            eprintln!(
                "{out}: {} periods, roundtrip parses ({} B)",
                parsed.len(),
                bytes.len()
            );
        }
        None => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(NETLOC, &out) {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<EM_TF>
  <Data count="2">
    <Period value="10.0" units="secs">
      <Z type="complex" size="2  2" units="[mV/km]/[nT]">
        <value name="Zxx" output="Ex" input="Hx">1.0 2.0</value>
        <value name="Zxy" output="Ex" input="Hy">3.0 4.0</value>
        <value name="Zyx" output="Ey" input="Hx">5.0 6.0</value>
        <value name="Zyy" output="Ey" input="Hy">7.0 8.0</value>
      </Z>
      <Z.VAR type="real" size="2  2">
        <value name="Zxx" output="Ex" input="Hx">0.1</value>
      </Z.VAR>
    </Period>
    <Period value="20.0" units="secs">
      <Z type="complex" size="2  2" units="[mV/km]/[nT]">
        <value name="Zxx" output="Ex" input="Hx">9.0 10.0</value>
        <value name="Zxy" output="Ex" input="Hy">11.0 12.0</value>
        <value name="Zyx" output="Ey" input="Hx">13.0 14.0</value>
      </Z>
    </Period>
  </Data>
</EM_TF>"#;

    #[test]
    fn parses_complete_period_and_skips_the_incomplete() {
        let records = parse_emtf(SAMPLE);

        assert_eq!(records.len(), 1);
        let c = Z_MT_TO_SI;
        assert_eq!(
            records[0],
            [
                10.0,
                1.0 * c,
                2.0 * c,
                3.0 * c,
                4.0 * c,
                5.0 * c,
                6.0 * c,
                7.0 * c,
                8.0 * c
            ]
        );
    }

    #[test]
    fn bin_roundtrip_preserves_records() {
        let records = parse_emtf(SAMPLE);
        let bytes = write_bin(&records);
        assert_eq!(bytes.len(), 8 + records.len() * FIELDS * 8);
        assert_eq!(parse_bin(&bytes).as_deref(), Some(records.as_slice()));
        assert!(parse_bin(b"XXXX").is_none());
        let short = &bytes[..bytes.len() - 1];
        assert!(parse_bin(short).is_none());
    }
}
