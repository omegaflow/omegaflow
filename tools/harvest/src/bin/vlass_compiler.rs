use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::fits::{FitsColumn, FitsHeader, FitsTable};

const URL: &str = "https://ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca/files/vault/cirada/continuum/vlass_data/sources_se.fits";
const CDN_TAG: &str = "ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca";

const MAGIC: [u8; 4] = *b"VLAS";
const MAGIC_COMP: [u8; 4] = *b"VLAC";
const VLASS_FREQ_HZ: f64 = 3.0e9;
const HEADER_BYTES: usize = 16;
const REC_BYTES: usize = 32;
const COMP_REC_BYTES: usize = 72;

#[derive(Clone, Copy, Debug)]
struct VlassSource {
    ra_deg: f64,
    dec_deg: f64,
    flux_mjy: f64,
    e_flux_mjy: f64,
}

#[derive(Clone, Copy, Debug)]
struct VlassComponent {
    ra_deg: f64,
    dec_deg: f64,
    total_flux_mjy: f64,
    e_total_flux_mjy: f64,
    peak_flux_mjy: f64,
    e_peak_flux_mjy: f64,
    maj_arcsec: f64,
    min_arcsec: f64,
    pa_deg: f64,
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

fn write_component_bin(records: &[VlassComponent]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * COMP_REC_BYTES);
    out.extend_from_slice(&MAGIC_COMP);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    out.extend_from_slice(&VLASS_FREQ_HZ.to_le_bytes());
    for r in records {
        out.extend_from_slice(&r.ra_deg.to_le_bytes());
        out.extend_from_slice(&r.dec_deg.to_le_bytes());
        out.extend_from_slice(&r.total_flux_mjy.to_le_bytes());
        out.extend_from_slice(&r.e_total_flux_mjy.to_le_bytes());
        out.extend_from_slice(&r.peak_flux_mjy.to_le_bytes());
        out.extend_from_slice(&r.e_peak_flux_mjy.to_le_bytes());
        out.extend_from_slice(&r.maj_arcsec.to_le_bytes());
        out.extend_from_slice(&r.min_arcsec.to_le_bytes());
        out.extend_from_slice(&r.pa_deg.to_le_bytes());
    }
    out
}

fn read_component_bin(data: &[u8]) -> Option<Vec<VlassComponent>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC_COMP {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * COMP_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * COMP_REC_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(data.get(o..o + 8)?.try_into().ok()?))
        };
        let ra_deg = f64_at(base)?;
        let dec_deg = f64_at(base + 8)?;
        let total_flux_mjy = f64_at(base + 16)?;
        let e_total_flux_mjy = f64_at(base + 24)?;
        let peak_flux_mjy = f64_at(base + 32)?;
        let e_peak_flux_mjy = f64_at(base + 40)?;
        let maj_arcsec = f64_at(base + 48)?;
        let min_arcsec = f64_at(base + 56)?;
        let pa_deg = f64_at(base + 64)?;
        if !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !total_flux_mjy.is_finite()
            || !e_total_flux_mjy.is_finite()
            || !peak_flux_mjy.is_finite()
            || !e_peak_flux_mjy.is_finite()
            || !maj_arcsec.is_finite()
            || !min_arcsec.is_finite()
            || !pa_deg.is_finite()
        {
            return None;
        }
        out.push(VlassComponent {
            ra_deg,
            dec_deg,
            total_flux_mjy,
            e_total_flux_mjy,
            peak_flux_mjy,
            e_peak_flux_mjy,
            maj_arcsec,
            min_arcsec,
            pa_deg,
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

fn col<'a>(table: &'a FitsTable, names: &[&str]) -> Option<&'a FitsColumn> {
    names.iter().find_map(|n| table.column(*n))
}

fn gather_component(bytes: &[u8]) -> Option<Vec<VlassComponent>> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (table, _next) = FitsTable::parse(bytes, off)?;
    let ra = col(&table, &["RA", "Component_RA", "RA_Component"])?;
    let dec = col(&table, &["DEC", "Component_DEC", "DEC_Component"])?;
    let total = col(&table, &["Total_flux", "Total_flux_component", "Flux"])?;
    let e_total = col(
        &table,
        &["E_Total_flux", "E_Total_flux_component", "E_Flux"],
    )?;
    let peak = col(&table, &["Peak_flux", "Peak_flux_component"])?;
    let e_peak = col(&table, &["E_Peak_flux", "E_Peak_flux_component"])?;
    let maj = col(&table, &["Maj", "Maj_component"])?;
    let min = col(&table, &["Min", "Min_component"])?;
    let pa = col(&table, &["PA", "PA_component"])?;

    eprintln!(
        "vlass component: {} rows, {} columns",
        table.n_rows,
        table.columns.len()
    );

    let mut records = Vec::with_capacity(table.n_rows);
    let mut skipped = 0usize;
    for row in 0..table.n_rows {
        let (
            Some(r),
            Some(d),
            Some(f),
            Some(ef),
            Some(p),
            Some(ep),
            Some(maj_v),
            Some(min_v),
            Some(pa_v),
        ) = (
            table.cell_f64(bytes, row, ra),
            table.cell_f64(bytes, row, dec),
            table.cell_f64(bytes, row, total),
            table.cell_f64(bytes, row, e_total),
            table.cell_f64(bytes, row, peak),
            table.cell_f64(bytes, row, e_peak),
            table.cell_f64(bytes, row, maj),
            table.cell_f64(bytes, row, min),
            table.cell_f64(bytes, row, pa),
        )
        else {
            skipped += 1;
            continue;
        };
        if !(0.0..360.0).contains(&r)
            || !(-90.0..=90.0).contains(&d)
            || !(f > 0.0)
            || !(ef > 0.0)
            || !(p > 0.0)
            || !(ep > 0.0)
            || !(maj_v >= 0.0)
            || !(min_v >= 0.0)
        {
            skipped += 1;
            continue;
        }
        records.push(VlassComponent {
            ra_deg: r,
            dec_deg: d,
            total_flux_mjy: f,
            e_total_flux_mjy: ef,
            peak_flux_mjy: p,
            e_peak_flux_mjy: ep,
            maj_arcsec: maj_v,
            min_arcsec: min_v,
            pa_deg: pa_v,
        });
    }
    if records.is_empty() {
        eprintln!("vlass component: no valid component — the asset stays unwritten (0 honored)");
        return None;
    }
    eprintln!(
        "vlass component: {} components, {} rows skipped",
        records.len(),
        skipped
    );
    Some(records)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: vlass_compiler [--kind source|component] [--input <file.fits>] [--url <route>] --out <bin> [--ci-mode]";
    let mut input: Option<String> = None;
    let mut out_path: Option<String> = None;
    let mut url: Option<String> = None;
    let mut kind = String::from("source");
    let mut ci_mode = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--url" => {
                url = args.get(i + 1).cloned();
                i += 1;
            }
            "--kind" => {
                kind = match args.get(i + 1) {
                    Some(k) => k.clone(),
                    None => "source".to_string(),
                };
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
    let fetch_url = match url {
        Some(v) => v,
        None => URL.to_string(),
    };
    let bytes = match input {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("vlass_compiler: read {path}: {e}");
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(&fetch_url, 604800) {
            Some(b) => b,
            None => {
                eprintln!("vlass_compiler: fetch void ({fetch_url})");
                std::process::exit(1);
            }
        },
    };
    if kind == "component" {
        let records = match gather_component(&bytes) {
            Some(r) => r,
            None => std::process::exit(1),
        };
        let bin = write_component_bin(&records);
        if std::fs::write(&out_path, &bin).is_err() {
            eprintln!("vlass_compiler: write {out_path} void");
            std::process::exit(1);
        }
        match read_component_bin(&bin) {
            Some(parsed) => {
                let mut fmax = 0.0f64;
                for r in &parsed {
                    if r.total_flux_mjy > fmax {
                        fmax = r.total_flux_mjy;
                    }
                }
                eprintln!(
                    "vlass component: {} records, total_flux_mjy bis {fmax:.3e}, {} B -> {out_path} (roundtrip parses)",
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
    } else {
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

    fn sample_component() -> Vec<VlassComponent> {
        vec![
            VlassComponent {
                ra_deg: 120.0817,
                dec_deg: 2.3533,
                total_flux_mjy: 25.7,
                e_total_flux_mjy: 0.4,
                peak_flux_mjy: 20.1,
                e_peak_flux_mjy: 0.3,
                maj_arcsec: 1.5,
                min_arcsec: 0.8,
                pa_deg: 45.0,
            },
            VlassComponent {
                ra_deg: 314.9519,
                dec_deg: -0.1425,
                total_flux_mjy: 399.2,
                e_total_flux_mjy: 1.1,
                peak_flux_mjy: 350.0,
                e_peak_flux_mjy: 0.9,
                maj_arcsec: 0.0,
                min_arcsec: 0.0,
                pa_deg: 0.0,
            },
        ]
    }

    #[test]
    fn component_bin_roundtrip() {
        let comps = sample_component();
        let bytes = write_component_bin(&comps);
        let parsed = read_component_bin(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, comps[0].ra_deg);
        assert_eq!(parsed[0].peak_flux_mjy, comps[0].peak_flux_mjy);
        assert_eq!(parsed[1].pa_deg, comps[1].pa_deg);
        assert_eq!(parsed[1].maj_arcsec, 0.0);
    }

    #[test]
    fn component_bin_rejects_source_magic() {
        assert!(read_component_bin(&write_bin(&sample())).is_none());
        assert!(read_bin(&write_component_bin(&sample_component())).is_none());
    }

    #[test]
    fn component_bin_rejects_truncation() {
        let bytes = write_component_bin(&sample_component());
        assert!(read_component_bin(&bytes[..bytes.len() - 1]).is_none());
    }

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn vlass_fixture() -> Vec<u8> {
        let mut buf = Vec::new();
        let mut hdr: Vec<u8> = Vec::new();
        for (k, v) in [
            ("SIMPLE", "T"),
            ("BITPIX", "8"),
            ("NAXIS", "0"),
            ("END", ""),
        ] {
            hdr.extend_from_slice(&pad_card(k, v));
        }
        while hdr.len() % 2880 != 0 {
            hdr.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&hdr);

        let mut ext: Vec<u8> = Vec::new();
        for (k, v) in [
            ("XTENSION", "'BINTABLE'"),
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "40"),
            ("NAXIS2", "2"),
            ("PCOUNT", "10"),
            ("GCOUNT", "1"),
            ("TFIELDS", "5"),
            ("TTYPE1", "'RA'"),
            ("TFORM1", "D"),
            ("TBCOL1", "1"),
            ("TTYPE2", "'DEC'"),
            ("TFORM2", "D"),
            ("TBCOL2", "9"),
            ("TTYPE3", "'Flux'"),
            ("TFORM3", "D"),
            ("TBCOL3", "17"),
            ("TTYPE4", "'E_Flux'"),
            ("TFORM4", "D"),
            ("TBCOL4", "25"),
            ("TTYPE5", "'Name'"),
            ("TFORM5", "'1PA'"),
            ("TBCOL5", "33"),
            ("END", ""),
        ] {
            ext.extend_from_slice(&pad_card(k, v));
        }
        while ext.len() % 2880 != 0 {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);

        buf.extend_from_slice(&120.0817f64.to_be_bytes());
        buf.extend_from_slice(&2.3533f64.to_be_bytes());
        buf.extend_from_slice(&25.7f64.to_be_bytes());
        buf.extend_from_slice(&0.4f64.to_be_bytes());
        buf.extend_from_slice(&5u32.to_be_bytes());
        buf.extend_from_slice(&0u32.to_be_bytes());
        buf.extend_from_slice(&314.9519f64.to_be_bytes());
        buf.extend_from_slice(&(-0.1425f64).to_be_bytes());
        buf.extend_from_slice(&399.2f64.to_be_bytes());
        buf.extend_from_slice(&1.1f64.to_be_bytes());
        buf.extend_from_slice(&5u32.to_be_bytes());
        buf.extend_from_slice(&5u32.to_be_bytes());
        buf.extend_from_slice(b"J0001J0002");
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn gather_extracts_from_a_bintable_that_carries_a_varlen_column() {
        let buf = vlass_fixture();
        let (_, off) = FitsHeader::parse(&buf, 0).expect("primary header");
        let (table, _) = FitsTable::parse(&buf, off).expect("bintable");
        assert_eq!(table.n_rows, 2);
        assert_eq!(table.column("Name").map(|c| c.code), Some('P'));
        let records = gather(&buf).expect("gather");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].ra_deg, 120.0817);
        assert_eq!(records[0].dec_deg, 2.3533);
        assert_eq!(records[0].flux_mjy, 25.7);
        assert_eq!(records[0].e_flux_mjy, 0.4);
        assert_eq!(records[1].ra_deg, 314.9519);
        assert_eq!(records[1].dec_deg, -0.1425);
        assert_eq!(records[1].flux_mjy, 399.2);
        assert_eq!(records[1].e_flux_mjy, 1.1);
    }
}
