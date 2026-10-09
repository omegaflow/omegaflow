use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsImage, FitsTable};

const NETLOC: &str = "cdsarc.cds.unistra.fr";
const FORMAT: &str = "ebhis_hpx_series";
const ASSET: &str = "ebhis_hpx190.bin";
const DEFAULT_URL: &str = "https://cdsarc.cds.unistra.fr/ftp/J/A+A/585/A41/hpx/HPX_190.fit";
const DEFAULT_OUT: &str = "data/cdsarc.cds.unistra.fr/ebhis_hpx190.bin";

const MAGIC: [u8; 4] = *b"EBH1";
const N_CHANNELS: usize = 945;
const MASK_BYTES: usize = N_CHANNELS.div_ceil(8);
const HEADER_BYTES: usize = 8;
const SERIES_FIXED_BYTES: usize = 24 + MASK_BYTES;

#[derive(Clone, Debug, PartialEq)]
struct EbhisSeries {
    hpx_index: u64,
    glon: f64,
    glat: f64,
    channels: Vec<Option<f64>>,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn find_table(bytes: &[u8]) -> Option<FitsTable> {
    let mut off = 0usize;
    while off + 80 <= bytes.len() {
        let (hdr, _) = FitsHeader::parse(bytes, off)?;
        if hdr.value("XTENSION") == Some("'BINTABLE'") {
            let (table, next) = FitsTable::parse(bytes, off)?;
            if hdr.str_unescaped("EXTNAME").as_deref() == Some("EBHIS-HPX") {
                return Some(table);
            }
            if next <= off {
                return None;
            }
            off = next;
            continue;
        }
        let (_, next) = FitsImage::parse(bytes, off)?;
        if next <= off {
            return None;
        }
        off = next;
    }
    None
}

fn read_series(table: &FitsTable, bytes: &[u8]) -> Option<Vec<EbhisSeries>> {
    let hpx_col = table.column("HPXINDEX")?;
    let glon_col = table.column("GLON")?;
    let glat_col = table.column("GLAT")?;
    let data_col = table.column("DATA")?;
    let wdata_col = table.column("WDATA")?;
    let mut out = Vec::new();
    for row in 0..table.n_rows {
        let Some(hpx) = table.cell_i64(bytes, row, hpx_col) else {
            continue;
        };
        if hpx < 0 {
            continue;
        }
        let Some(glon) = table.cell_f64(bytes, row, glon_col) else {
            continue;
        };
        let Some(glat) = table.cell_f64(bytes, row, glat_col) else {
            continue;
        };
        if !glon.is_finite() || !glat.is_finite() {
            continue;
        }
        let Some(data) = table.cell_array_f64(bytes, row, data_col) else {
            continue;
        };
        let Some(wdata) = table.cell_array_f64(bytes, row, wdata_col) else {
            continue;
        };
        if data.len() != N_CHANNELS || wdata.len() != N_CHANNELS {
            continue;
        }
        let channels: Vec<Option<f64>> = (0..N_CHANNELS)
            .map(|c| {
                let d = data[c];
                let w = wdata[c];
                if d.is_finite() && w.is_finite() && w > 0.0 {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();
        out.push(EbhisSeries {
            hpx_index: hpx as u64,
            glon,
            glat,
            channels,
        });
    }
    Some(out)
}

fn write_bin(series: &[EbhisSeries]) -> Option<Vec<u8>> {
    let count = u32::try_from(series.len()).ok()?;
    let mut out = Vec::new();
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&count.to_le_bytes());
    for s in series {
        if s.channels.len() != N_CHANNELS || !s.glon.is_finite() || !s.glat.is_finite() {
            return None;
        }
        out.extend_from_slice(&s.hpx_index.to_le_bytes());
        out.extend_from_slice(&s.glon.to_le_bytes());
        out.extend_from_slice(&s.glat.to_le_bytes());
        let mut mask = [0u8; MASK_BYTES];
        for (c, ch) in s.channels.iter().enumerate() {
            if let Some(v) = ch {
                if !v.is_finite() {
                    return None;
                }
                mask[c / 8] |= 1 << (c % 8);
            }
        }
        out.extend_from_slice(&mask);
        for ch in &s.channels {
            if let Some(v) = ch {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
    Some(out)
}

fn parse_bin(bytes: &[u8]) -> Option<Vec<EbhisSeries>> {
    if bytes.len() < HEADER_BYTES || bytes[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let mut off = HEADER_BYTES;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let hpx_index = u64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
        let glon = f64::from_le_bytes(bytes.get(off + 8..off + 16)?.try_into().ok()?);
        let glat = f64::from_le_bytes(bytes.get(off + 16..off + 24)?.try_into().ok()?);
        off += 24;
        let mask = bytes.get(off..off + MASK_BYTES)?;
        off += MASK_BYTES;
        if !glon.is_finite() || !glat.is_finite() {
            return None;
        }
        let mut channels = Vec::with_capacity(N_CHANNELS);
        for c in 0..N_CHANNELS {
            if (mask[c / 8] >> (c % 8)) & 1 == 1 {
                let v = f64::from_le_bytes(bytes.get(off..off + 8)?.try_into().ok()?);
                if !v.is_finite() {
                    return None;
                }
                off += 8;
                channels.push(Some(v));
            } else {
                channels.push(None);
            }
        }
        out.push(EbhisSeries {
            hpx_index,
            glon,
            glat,
            channels,
        });
    }
    if off != bytes.len() {
        return None;
    }
    Some(out)
}

fn sample_channels() -> Vec<Option<f64>> {
    (0..N_CHANNELS)
        .map(|c| match c {
            0 => Some(-12.5),
            7 => Some(0.0),
            16 => Some(41.25),
            944 => Some(3.5),
            _ => None,
        })
        .collect()
}

fn run(args: &[String]) -> Result<(), String> {
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let input = arg_value(args, "--input");
    let url = match arg_value(args, "--url") {
        Some(v) => v,
        None => DEFAULT_URL.to_string(),
    };
    let out = match arg_value(args, "--out") {
        Some(p) if !p.is_empty() => p,
        _ => DEFAULT_OUT.to_string(),
    };
    let bytes = match &input {
        Some(path) => std::fs::read(path).map_err(|e| format!("{path}: read void ({e})"))?,
        None => fetch_raw_bytes(&url)
            .ok_or_else(|| format!("{url}: fetch void — the asset stays unwritten"))?,
    };
    let table = find_table(&bytes).ok_or_else(|| {
        "the EBHIS-HPX BINTABLE stays unread — the asset stays unwritten".to_string()
    })?;
    let series = read_series(&table, &bytes).ok_or_else(|| {
        "the EBHIS-HPX columns stay unread — the asset stays unwritten".to_string()
    })?;
    if series.is_empty() {
        return Err(
            "no HPX pixel left the table — the asset stays unwritten (0 honored)".to_string(),
        );
    }
    let bin = write_bin(&series)
        .ok_or_else(|| "a held channel is not finite — the asset stays unwritten".to_string())?;
    match parse_bin(&bin) {
        Some(parsed) if parsed == series => {}
        _ => {
            return Err("the roundtrip does not read back — the asset stays unwritten".to_string());
        }
    }
    if let Some(parent) = std::path::Path::new(&out).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create {} void: {e}", parent.display()))?;
        }
    }
    std::fs::write(&out, &bin).map_err(|e| format!("write {out} void: {e}"))?;

    let total_channels = series.len() * N_CHANNELS;
    let present: usize = series
        .iter()
        .map(|s| s.channels.iter().filter(|c| c.is_some()).count())
        .sum();
    let origin = match input {
        Some(path) => path,
        None => url,
    };
    println!("url https://github.com/omegaflow/sources/releases/download/{NETLOC}/{ASSET}");
    println!("origin {origin}");
    println!("compiler tools/harvest/src/bin/ebhis_compiler.rs");
    println!("format {FORMAT}");
    println!("sha256 {}", sha256_hex(&bin));
    eprintln!(
        "{out}: {} series, {}/{} channels present, {} B, roundtrip parses",
        series.len(),
        present,
        total_channels,
        bin.len()
    );
    if ci_mode && !upload_release(NETLOC, &out) {
        return Err(format!("{out}: the CDN upload returned void"));
    }
    Ok(())
}

fn selftest() {
    let series = vec![
        EbhisSeries {
            hpx_index: 1_000_003,
            glon: 122.25,
            glat: 15.5,
            channels: sample_channels(),
        },
        EbhisSeries {
            hpx_index: 1_000_004,
            glon: 122.75,
            glat: 15.0,
            channels: (0..N_CHANNELS).map(|_| None).collect(),
        },
    ];
    let Some(bin) = write_bin(&series) else {
        eprintln!("selftest: write_bin void");
        std::process::exit(1);
    };
    let bin_bytes = HEADER_BYTES + 2 * SERIES_FIXED_BYTES + 4 * 8;
    if bin.len() != bin_bytes {
        eprintln!("selftest: the bin does not carry the measured stride");
        std::process::exit(1);
    }
    if parse_bin(&bin) != Some(series.clone()) {
        eprintln!("selftest: the roundtrip does not read back");
        std::process::exit(1);
    }
    if parse_bin(&bin[..bin.len() - 1]).is_some() {
        eprintln!("selftest: a truncated bin reads back");
        std::process::exit(1);
    }
    let mut foreign = bin.clone();
    foreign[0] = b'X';
    if parse_bin(&foreign).is_some() {
        eprintln!("selftest: a foreign magic reads back");
        std::process::exit(1);
    }
    eprintln!("ebhis_compiler: selftest passes (HPX pixel -> present-channel bin)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--selftest") {
        selftest();
        return;
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "usage: ebhis_compiler [--url {DEFAULT_URL}] [--input <HPX_190.fit>] [--out <file.bin>] [--ci-mode] | --selftest"
        );
        std::process::exit(2);
    }
    if let Err(msg) = run(&args) {
        eprintln!("ebhis_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_roundtrips_the_measured_stride() {
        let series = vec![EbhisSeries {
            hpx_index: 42,
            glon: 10.0,
            glat: -3.0,
            channels: sample_channels(),
        }];
        let bin = write_bin(&series).expect("finite series encode");
        assert_eq!(bin.len(), HEADER_BYTES + SERIES_FIXED_BYTES + 4 * 8);
        assert_eq!(parse_bin(&bin), Some(series));
    }

    #[test]
    fn parse_bin_refuses_a_truncated_asset() {
        let series = vec![EbhisSeries {
            hpx_index: 7,
            glon: 1.0,
            glat: 2.0,
            channels: sample_channels(),
        }];
        let bin = write_bin(&series).expect("finite series encode");
        assert_eq!(parse_bin(&bin[..bin.len() - 1]), None);
    }

    #[test]
    fn write_bin_refuses_a_mismatched_channel_count() {
        let series = vec![EbhisSeries {
            hpx_index: 7,
            glon: 1.0,
            glat: 2.0,
            channels: vec![Some(1.0)],
        }];
        assert_eq!(write_bin(&series), None);
    }
}
