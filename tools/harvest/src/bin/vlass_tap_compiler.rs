use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;

const CDN_TAG: &str = "ws-uv.canfar.net";

const BASE: &str =
    "https://ws-uv.canfar.net/youcat/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=";
const SRC_QUERY: &str = "SELECT+TOP+5000+RA_Source,DEC_Source,Total_flux_source,E_Total_flux_source,Peak_flux_source+FROM+cirada.VLASS_Source+WHERE+Total_flux_source+IS+NOT+NULL";
const CMP_QUERY: &str = "SELECT+TOP+5000+RA,DEC,Total_flux,E_Total_flux,Peak_flux,E_Peak_flux+FROM+cirada.VLASS_Component+WHERE+Total_flux+IS+NOT+NULL";

const MAGIC_SRC: [u8; 4] = *b"VLST";
const MAGIC_CMP: [u8; 4] = *b"VLCT";
const VLASS_FREQ_HZ: f64 = 3.0e9;
const HEADER_BYTES: usize = 16;
const SRC_REC_BYTES: usize = 40;
const CMP_REC_BYTES: usize = 48;

#[derive(Clone, Copy, Debug)]
struct SourceRec {
    ra_deg: f64,
    dec_deg: f64,
    total_mjy: f64,
    e_total_mjy: f64,
    peak_mjy: f64,
}

#[derive(Clone, Copy, Debug)]
struct CompRec {
    ra_deg: f64,
    dec_deg: f64,
    total_mjy: f64,
    e_total_mjy: f64,
    peak_mjy: f64,
    e_peak_mjy: f64,
}

fn csv_fields(line: &str) -> Vec<&str> {
    line.trim_end_matches('\r')
        .split(',')
        .map(str::trim)
        .collect()
}

fn col_idx(header: &[&str], name: &str) -> Option<usize> {
    header.iter().position(|h| *h == name)
}

fn f64_at(fields: &[&str], i: usize) -> Option<f64> {
    fields.get(i)?.parse::<f64>().ok()
}

fn write_src(records: &[SourceRec]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * SRC_REC_BYTES);
    out.extend_from_slice(&MAGIC_SRC);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    out.extend_from_slice(&VLASS_FREQ_HZ.to_le_bytes());
    for r in records {
        out.extend_from_slice(&r.ra_deg.to_le_bytes());
        out.extend_from_slice(&r.dec_deg.to_le_bytes());
        out.extend_from_slice(&r.total_mjy.to_le_bytes());
        out.extend_from_slice(&r.e_total_mjy.to_le_bytes());
        out.extend_from_slice(&r.peak_mjy.to_le_bytes());
    }
    out
}

fn read_src(data: &[u8]) -> Option<Vec<SourceRec>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC_SRC {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * SRC_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * SRC_REC_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(data.get(o..o + 8)?.try_into().ok()?))
        };
        let ra_deg = f64_at(base)?;
        let dec_deg = f64_at(base + 8)?;
        let total_mjy = f64_at(base + 16)?;
        let e_total_mjy = f64_at(base + 24)?;
        let peak_mjy = f64_at(base + 32)?;
        if !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !total_mjy.is_finite()
            || !e_total_mjy.is_finite()
            || !peak_mjy.is_finite()
        {
            return None;
        }
        out.push(SourceRec {
            ra_deg,
            dec_deg,
            total_mjy,
            e_total_mjy,
            peak_mjy,
        });
    }
    Some(out)
}

fn write_cmp(records: &[CompRec]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_BYTES + records.len() * CMP_REC_BYTES);
    out.extend_from_slice(&MAGIC_CMP);
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    out.extend_from_slice(&VLASS_FREQ_HZ.to_le_bytes());
    for r in records {
        out.extend_from_slice(&r.ra_deg.to_le_bytes());
        out.extend_from_slice(&r.dec_deg.to_le_bytes());
        out.extend_from_slice(&r.total_mjy.to_le_bytes());
        out.extend_from_slice(&r.e_total_mjy.to_le_bytes());
        out.extend_from_slice(&r.peak_mjy.to_le_bytes());
        out.extend_from_slice(&r.e_peak_mjy.to_le_bytes());
    }
    out
}

fn read_cmp(data: &[u8]) -> Option<Vec<CompRec>> {
    if data.len() < HEADER_BYTES || data[0..4] != MAGIC_CMP {
        return None;
    }
    let count = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() != HEADER_BYTES + count * CMP_REC_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let base = HEADER_BYTES + i * CMP_REC_BYTES;
        let f64_at = |o: usize| -> Option<f64> {
            Some(f64::from_le_bytes(data.get(o..o + 8)?.try_into().ok()?))
        };
        let ra_deg = f64_at(base)?;
        let dec_deg = f64_at(base + 8)?;
        let total_mjy = f64_at(base + 16)?;
        let e_total_mjy = f64_at(base + 24)?;
        let peak_mjy = f64_at(base + 32)?;
        let e_peak_mjy = f64_at(base + 40)?;
        if !ra_deg.is_finite()
            || !dec_deg.is_finite()
            || !total_mjy.is_finite()
            || !e_total_mjy.is_finite()
            || !peak_mjy.is_finite()
            || !e_peak_mjy.is_finite()
        {
            return None;
        }
        out.push(CompRec {
            ra_deg,
            dec_deg,
            total_mjy,
            e_total_mjy,
            peak_mjy,
            e_peak_mjy,
        });
    }
    Some(out)
}

fn gather_source(text: &str) -> Option<Vec<SourceRec>> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = csv_fields(lines.next()?);
    let ra = col_idx(&header, "RA_Source")?;
    let dec = col_idx(&header, "DEC_Source")?;
    let total = col_idx(&header, "Total_flux_source")?;
    let e_total = col_idx(&header, "E_Total_flux_source")?;
    let peak = col_idx(&header, "Peak_flux_source")?;

    let mut records = Vec::new();
    let mut skipped = 0usize;
    for line in lines {
        let fields = csv_fields(line);
        let (Some(r), Some(d), Some(t), Some(et), Some(p)) = (
            f64_at(&fields, ra),
            f64_at(&fields, dec),
            f64_at(&fields, total),
            f64_at(&fields, e_total),
            f64_at(&fields, peak),
        ) else {
            skipped += 1;
            continue;
        };
        if !(0.0..360.0).contains(&r)
            || !(-90.0..=90.0).contains(&d)
            || !(t > 0.0)
            || !(et > 0.0)
            || !(p > 0.0)
        {
            skipped += 1;
            continue;
        }
        records.push(SourceRec {
            ra_deg: r,
            dec_deg: d,
            total_mjy: t,
            e_total_mjy: et,
            peak_mjy: p,
        });
    }
    if records.is_empty() {
        eprintln!("vlass tap source: no valid rows — the asset stays unwritten (0 honored)");
        return None;
    }
    eprintln!(
        "vlass tap source: {} rows, {} skipped",
        records.len(),
        skipped
    );
    Some(records)
}

fn gather_component(text: &str) -> Option<Vec<CompRec>> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = csv_fields(lines.next()?);
    let ra = col_idx(&header, "RA")?;
    let dec = col_idx(&header, "DEC")?;
    let total = col_idx(&header, "Total_flux")?;
    let e_total = col_idx(&header, "E_Total_flux")?;
    let peak = col_idx(&header, "Peak_flux")?;
    let e_peak = col_idx(&header, "E_Peak_flux")?;

    let mut records = Vec::new();
    let mut skipped = 0usize;
    for line in lines {
        let fields = csv_fields(line);
        let (Some(r), Some(d), Some(t), Some(et), Some(p), Some(ep)) = (
            f64_at(&fields, ra),
            f64_at(&fields, dec),
            f64_at(&fields, total),
            f64_at(&fields, e_total),
            f64_at(&fields, peak),
            f64_at(&fields, e_peak),
        ) else {
            skipped += 1;
            continue;
        };
        if !(0.0..360.0).contains(&r)
            || !(-90.0..=90.0).contains(&d)
            || !(t > 0.0)
            || !(et > 0.0)
            || !(p > 0.0)
            || !(ep > 0.0)
        {
            skipped += 1;
            continue;
        }
        records.push(CompRec {
            ra_deg: r,
            dec_deg: d,
            total_mjy: t,
            e_total_mjy: et,
            peak_mjy: p,
            e_peak_mjy: ep,
        });
    }
    if records.is_empty() {
        eprintln!("vlass tap component: no valid rows — the asset stays unwritten (0 honored)");
        return None;
    }
    eprintln!(
        "vlass tap component: {} rows, {} skipped",
        records.len(),
        skipped
    );
    Some(records)
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: vlass_tap_compiler [--kind source|component] [--input <file.csv>] [--url <route>] --out <bin> [--ci-mode]";
    let ci_mode = args.iter().any(|a| a == "--ci-mode");
    let kind = match arg_value(&args, "--kind") {
        Some(v) => v,
        None => "source".to_string(),
    };
    let out_path = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("{usage}");
            std::process::exit(1);
        }
    };
    let fetch_url = match arg_value(&args, "--url") {
        Some(v) => v,
        None => format!(
            "{BASE}{}",
            if kind == "component" {
                CMP_QUERY
            } else {
                SRC_QUERY
            }
        ),
    };
    let bytes = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("vlass_tap_compiler: read {path}: {e}");
                std::process::exit(1);
            }
        },
        None => match fetch_raw_bytes(&fetch_url, 604800) {
            Some(b) => b,
            None => {
                eprintln!("vlass_tap_compiler: fetch void ({fetch_url})");
                std::process::exit(1);
            }
        },
    };
    let text = String::from_utf8_lossy(&bytes);
    if kind == "component" {
        let records = match gather_component(&text) {
            Some(r) => r,
            None => std::process::exit(1),
        };
        let bin = write_cmp(&records);
        if std::fs::write(&out_path, &bin).is_err() {
            eprintln!("vlass_tap_compiler: write {out_path} void");
            std::process::exit(1);
        }
        match read_cmp(&bin) {
            Some(parsed) => eprintln!(
                "vlass tap component: {} records, {} B -> {out_path} (roundtrip parses)",
                parsed.len(),
                bin.len()
            ),
            None => {
                eprintln!(
                    "vlass_tap_compiler: {out_path}: roundtrip parse void — the asset stays unverified"
                );
                std::process::exit(1);
            }
        }
    } else {
        let records = match gather_source(&text) {
            Some(r) => r,
            None => std::process::exit(1),
        };
        let bin = write_src(&records);
        if std::fs::write(&out_path, &bin).is_err() {
            eprintln!("vlass_tap_compiler: write {out_path} void");
            std::process::exit(1);
        }
        match read_src(&bin) {
            Some(parsed) => eprintln!(
                "vlass tap source: {} records, {} B -> {out_path} (roundtrip parses)",
                parsed.len(),
                bin.len()
            ),
            None => {
                eprintln!(
                    "vlass_tap_compiler: {out_path}: roundtrip parse void — the asset stays unverified"
                );
                std::process::exit(1);
            }
        }
    }
    if ci_mode && !upload_release(CDN_TAG, &out_path) {
        eprintln!("vlass_tap_compiler: upload {out_path} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC_CSV: &str = "RA_Source,DEC_Source,Total_flux_source,E_Total_flux_source,Peak_flux_source\n\
4.2932750569E-4,19.04440022422189,3.90431875046,0.4866574168899999,3.5087945195100003\n\
8.989744425700002E-4,-4.04502551242393,45.73029894447001,0.53351766346,41.90222122304\n";

    const CMP_CSV: &str = "RA,DEC,Total_flux,E_Total_flux,Peak_flux,E_Peak_flux\n\
1.7675657877000002E-4,80.01266744553764,0.8186520025599999,0.31886555872000005,0.75266829367,0.16584417167999999\n\
2.2178515263E-4,44.16959551709726,5.166948378880001,1.27345866877,1.06869314872,0.220937236\n";

    #[test]
    fn src_roundtrip() {
        let recs = gather_source(SRC_CSV).expect("gather");
        assert_eq!(recs.len(), 2);
        let bytes = write_src(&recs);
        let parsed = read_src(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].ra_deg, recs[0].ra_deg);
        assert_eq!(parsed[1].peak_mjy, recs[1].peak_mjy);
    }

    #[test]
    fn src_rejects_bad_magic() {
        assert!(read_src(b"XXXX").is_none());
        let mut bad = write_src(&gather_source(SRC_CSV).unwrap());
        bad[0] = b'X';
        assert!(read_src(&bad).is_none());
    }

    #[test]
    fn src_rejects_truncation() {
        let bytes = write_src(&gather_source(SRC_CSV).unwrap());
        assert!(read_src(&bytes[..bytes.len() - 1]).is_none());
    }

    #[test]
    fn src_skips_zero_flux() {
        let csv = "RA_Source,DEC_Source,Total_flux_source,E_Total_flux_source,Peak_flux_source\n\
1.0,2.0,0.0,0.1,0.2\n\
3.0,4.0,5.0,0.1,0.2\n";
        let recs = gather_source(csv).expect("gather");
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].total_mjy, 5.0);
    }

    #[test]
    fn cmp_roundtrip() {
        let recs = gather_component(CMP_CSV).expect("gather");
        assert_eq!(recs.len(), 2);
        let bytes = write_cmp(&recs);
        let parsed = read_cmp(&bytes).expect("parse");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].dec_deg, recs[0].dec_deg);
        assert_eq!(parsed[1].e_peak_mjy, recs[1].e_peak_mjy);
    }

    #[test]
    fn cmp_rejects_source_magic() {
        assert!(read_cmp(&write_src(&gather_source(SRC_CSV).unwrap())).is_none());
        assert!(read_src(&write_cmp(&gather_component(CMP_CSV).unwrap())).is_none());
    }

    #[test]
    fn cmp_rejects_truncation() {
        let bytes = write_cmp(&gather_component(CMP_CSV).unwrap());
        assert!(read_cmp(&bytes[..bytes.len() - 1]).is_none());
    }
}
