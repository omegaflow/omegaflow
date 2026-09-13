use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsHeader, FitsTable};

const URL: &str = "https://ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca/files/vault/cirada/continuum/vlass_data/sources_se.fits";
const CDN_TAG: &str = "ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca";

const MAGIC: [u8; 4] = *b"VLAS";
const VLASS_FREQ_HZ: f64 = 3.0e9;
const HEADER_BYTES: usize = 16;
const REC_BYTES: usize = 32;

#[derive(Clone, Copy, Debug)]
struct VlassSource {
    ra_deg: f64,
    dec_deg: f64,
    flux_mjy: f64,
    e_flux_mjy: f64,
}

fn write_bin(records: &[VlassSource]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * REC_BYTES);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    out.extend_from_slice(&VLASS_FREQ_HZ.to_le_bytes());
    for r in records {
        out.extend_from_slice(&r.ra_deg.to_le_bytes());
        out.extend_from_slice(&r.dec_deg.to_le_bytes());
        out.extend_from_slice(&r.flux_mjy.to_le_bytes());
        out.extend_from_slice(&r.e_flux_mjy.to_le_bytes());
    }
    out
}

fn read_bin(data: &[u8]) -> Option<Vec<VlassSource>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * REC_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(data.get(o..o + 8)?.try_into().ok()?))
        };
        let ra_deg = f64_at(base)?;
        let dec_deg = f64_at(base + 8)?;
        let flux_mjy = f64_at(base + 16)?;
        let e_flux_mjy = f64_at(base + 24)?;
        if !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !flux_mjy.is_finite()
            || !e_flux_mjy.is_finite()
        {
            return None;
        }
        out.push(VlassSource {
            ra_deg,
            dec_deg,
            flux_mjy,
            e_flux_mjy,
        });
    }
    Some(out)
}

fn gather(bytes: &[u8]) -> Option<Vec<VlassSource>> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (table, _next) = FitsTable::parse(bytes, off)?;
    let ra = table.column("RA")?;
    let dec = table.column("DEC")?;
    let flux = table.column("Flux")?;
    let e_flux = table.column("E_Flux")?;

    eprintln!(
        "vlass: {} rows, {} columns",
        table.n_rows,
        table.columns.len()
    );
    for c in &table.columns {
        eprintln!(
            "  column {:<14} code {} repeat {} width {} tbcol {}",
            c.name, c.code, c.repeat, c.width, c.tbcol
        );
    }

    let mut records = Vec::with_capacity(table.n_rows);
    let mut skipped = 0usize;
    for row in 0..table.n_rows {
        let (Some(r), Some(d), Some(f), Some(e)) = (
            table.cell_f64(bytes, row, ra),
            table.cell_f64(bytes, row, dec),
            table.cell_f64(bytes, row, flux),
            table.cell_f64(bytes, row, e_flux),
        ) else {
            skipped += 1;
            continue;
        };
        if !(0.0..360.0).contains(&r) || !(-90.0..=90.0).contains(&d) || !(f > 0.0) || !(e > 0.0) {
            skipped += 1;
            continue;
        }
        records.push(VlassSource {
            ra_deg: r,
            dec_deg: d,
            flux_mjy: f,
            e_flux_mjy: e,
        });
    }
    if records.is_empty() {
        eprintln!("vlass: no valid source — the asset stays unwritten (0 honored)");
        return None;
    }
    eprintln!("vlass: {} sources, {} rows skipped", records.len(), skipped);
    Some(records)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: vlass_compiler [--input <sources_se.fits>] --out <vlass_sources_se.bin> [--ci-mode]";
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
    let out_path = match out_path {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };
    let bytes = match input {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("vlass_compiler: read {path}: {e}");
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(URL, 604800) {
            Some(b) => b,
            None => {
                eprintln!("vlass_compiler: fetch void ({URL})");
                std::process::exit(1);
            }
        },
    };
    let records = match gather(&bytes) {
        Some(r) => r,
        None => std::process::exit(1),
    };
    let bin = write_bin(&records);
    if std::fs::write(&out_path, &bin).is_err() {
        eprintln!("vlass_compiler: write {out_path} void");
        std::process::exit(1);
    }
    match read_bin(&bin) {
        Some(parsed) => {
            let mut fmax = 0.0f64;
            for r in &parsed {
                if r.flux_mjy > fmax {
                    fmax = r.flux_mjy;
                }
            }
            eprintln!(
                "vlass: {} sources, flux_mjy bis {fmax:.3e}, {} B -> {out_path} (roundtrip parses)",
                parsed.len(),
                bin.len()
            );
        }
        None => {
            eprintln!(
                "vlass_compiler: {out_path}: roundtrip parse void — the asset stays unverified"
            );
            std::process::exit(1);
        }
    }
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("vlass_compiler: upload {} did not reach the CDN", out_path);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<VlassSource> {
        vec![
            VlassSource {
                ra_deg: 120.0817,
                dec_deg: 2.3533,
                flux_mjy: 25.7,
                e_flux_mjy: 0.4,
            },
            VlassSource {
                ra_deg: 314.9519,
                dec_deg: -0.1425,
                flux_mjy: 399.2,
                e_flux_mjy: 1.1,
            },
        ]
    }

    #[test]
    fn bin_roundtrip() {
        let srcs = sample();
        let bytes = write_bin(&srcs);
        let parsed = read_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, srcs[0].ra_deg);
        assert_eq!(parsed[0].flux_mjy, srcs[0].flux_mjy);
        assert_eq!(parsed[1].dec_deg, srcs[1].dec_deg);
        assert_eq!(parsed[1].e_flux_mjy, srcs[1].e_flux_mjy);
    }

    #[test]
    fn bin_rejects_bad_magic() {
        assert!(read_bin(b"XXXX").is_none());
        let mut bad = write_bin(&sample());
        bad[0] = b'X';
        assert!(read_bin(&bad).is_none());
    }

    #[test]
    fn bin_rejects_truncation() {
        let bytes = write_bin(&sample());
        assert!(read_bin(&bytes[..bytes.len() - 1]).is_none());
    }
}
