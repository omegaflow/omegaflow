use omegaflow::archivar::dl3::{Dl3Event, parse_events, reduce_grid};
use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::{gunzip, tar_members};
use omegaflow::skymap::{
    HEADER_LEN, KIND_GAMMA, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header,
    write_header,
};
use std::io::{BufWriter, Write};

const HESS_TAR_URL: &str = "https://hess-experiment.eu/wp-content/uploads/2025/12/hess_dl3_dr1.tar";
const HESS_NETLOC: &str = "hess-experiment.eu";
const MAGIC_NETLOC: &str = "opendata.magic.pic.es";

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], key: &str) -> bool {
    args.iter().any(|a| a == key)
}

fn is_fits(b: &[u8]) -> bool {
    b.starts_with(b"SIMPLE")
}

fn is_gzip(b: &[u8]) -> bool {
    b.starts_with(&[0x1f, 0x8b])
}

fn is_tar(b: &[u8]) -> bool {
    b.get(257..262) == Some(b"ustar")
}

fn parse_grid(s: &str) -> Option<(usize, usize)> {
    let (lon, lat) = s.split_once('x')?;
    let lon = lon.parse::<usize>().ok()?;
    let lat = lat.parse::<usize>().ok()?;
    if lon == 0 || lat == 0 {
        return None;
    }
    Some((lon, lat))
}

fn events_from_payload(bytes: &[u8]) -> Option<Vec<Dl3Event>> {
    if is_fits(bytes) {
        return parse_events(bytes);
    }
    if is_gzip(bytes) {
        return events_from_payload(&gunzip(bytes)?);
    }
    if is_tar(bytes) {
        let members = tar_members(bytes)?;
        let mut all: Vec<Dl3Event> = Vec::new();
        for m in members {
            if let Some(ev) = events_from_payload(&bytes[m.start..m.end]) {
                all.extend(ev);
            }
        }
        if !all.is_empty() {
            return Some(all);
        }
    }
    None
}

fn records_of(events: &[Dl3Event], n_lon: usize, n_lat: usize) -> Vec<SkymapRecord> {
    let cells = reduce_grid(events, n_lon, n_lat);
    let mut out = Vec::new();
    for c in cells {
        if !(c.energy_tev.is_finite() && c.energy_tev > 0.0) {
            continue;
        }
        let value = c.energy_tev as f32;
        if !value.is_finite() {
            continue;
        }
        let Some((order, ipix)) = SkymapRecord::pixel_of(c.lon_deg, c.lat_deg) else {
            continue;
        };
        out.push(SkymapRecord {
            order,
            kind: KIND_GAMMA,
            ipix,
            ra_deg: c.lon_deg as f32,
            dec_deg: c.lat_deg as f32,
            value,
        });
    }
    out
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
        "last cell: ra {:.4} dec {:.4} value {:.3e} kind {}",
        last.ra_deg, last.dec_deg, last.value, last.kind
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ci_mode = has_flag(&args, "--ci-mode");
    let telescope = match arg_value(&args, "--telescope") {
        Some(t) => t,
        None => "hess".to_string(),
    };
    let netloc = match telescope.as_str() {
        "hess" => HESS_NETLOC,
        "magic" => MAGIC_NETLOC,
        _ => {
            eprintln!("--telescope {telescope} carries no netloc — hess | magic");
            std::process::exit(1);
        }
    };
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => format!("data/{netloc}/dl3_skymap.sky1"),
    };
    let grid = match arg_value(&args, "--grid") {
        Some(s) => match parse_grid(&s) {
            Some(g) => g,
            None => {
                eprintln!("--grid {s} not <lon>x<lat>");
                std::process::exit(1);
            }
        },
        None => (360, 180),
    };
    let bytes = match arg_value(&args, "--input") {
        Some(path) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {path} returned void: {e}");
                std::process::exit(1);
            }
        },
        None => {
            let url = match arg_value(&args, "--url") {
                Some(u) => u,
                None if telescope == "hess" => HESS_TAR_URL.to_string(),
                None => {
                    eprintln!(
                        "magic: --input <file.fits> or --url <https> required — the portal carries no single TAR"
                    );
                    std::process::exit(1);
                }
            };
            match fetch_raw_bytes(&url) {
                Some(b) => b,
                None => {
                    eprintln!("{url}: fetch void");
                    std::process::exit(1);
                }
            }
        }
    };
    let Some(events) = events_from_payload(&bytes) else {
        eprintln!("payload carries no DL3 EVENTS table — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    };
    let records = records_of(&events, grid.0, grid.1);
    if records.is_empty() {
        eprintln!("no cell with a positive summed energy — the asset stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if let Err(e) = write_asset(&records, &out) {
        eprintln!("dl3_skymap_compiler: {e}");
        std::process::exit(1);
    }
    if let Err(e) = verify_asset(&out, &records) {
        eprintln!("dl3_skymap_compiler: {e}");
        std::process::exit(1);
    }
    let bytes_written = HEADER_LEN + records.len() * REC_BYTES;
    eprintln!(
        "dl3: {telescope} -> {out}: {} events, {} non-empty {grid:?} cells, {} B — value = summed gamma-ray energy (TeV) per cell, kind gamma",
        events.len(),
        records.len(),
        bytes_written
    );
    if ci_mode && !upload_release(netloc, &out) {
        eprintln!("upload: {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pad_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn synth_events(rows: &[[f32; 3]]) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut primary: Vec<u8> = Vec::new();
        primary.extend_from_slice(&pad_card("SIMPLE", "T"));
        primary.extend_from_slice(&pad_card("BITPIX", "8"));
        primary.extend_from_slice(&pad_card("NAXIS", "0"));
        primary.extend_from_slice(&pad_card("END", ""));
        while !primary.len().is_multiple_of(2880) {
            primary.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&primary);
        let mut ext: Vec<u8> = Vec::new();
        ext.extend_from_slice(&pad_card("XTENSION", "'BINTABLE'"));
        ext.extend_from_slice(&pad_card("BITPIX", "8"));
        ext.extend_from_slice(&pad_card("NAXIS", "2"));
        ext.extend_from_slice(&pad_card("NAXIS1", "12"));
        ext.extend_from_slice(&pad_card("NAXIS2", &rows.len().to_string()));
        ext.extend_from_slice(&pad_card("PCOUNT", "0"));
        ext.extend_from_slice(&pad_card("GCOUNT", "1"));
        ext.extend_from_slice(&pad_card("TFIELDS", "3"));
        ext.extend_from_slice(&pad_card("EXTNAME", "'EVENTS'"));
        ext.extend_from_slice(&pad_card("HDUCLAS1", "'EVENTS'"));
        ext.extend_from_slice(&pad_card("TTYPE1", "'RA'"));
        ext.extend_from_slice(&pad_card("TFORM1", "E"));
        ext.extend_from_slice(&pad_card("TBCOL1", "1"));
        ext.extend_from_slice(&pad_card("TTYPE2", "'DEC'"));
        ext.extend_from_slice(&pad_card("TFORM2", "E"));
        ext.extend_from_slice(&pad_card("TBCOL2", "5"));
        ext.extend_from_slice(&pad_card("TTYPE3", "'ENERGY'"));
        ext.extend_from_slice(&pad_card("TFORM3", "E"));
        ext.extend_from_slice(&pad_card("TBCOL3", "9"));
        ext.extend_from_slice(&pad_card("END", ""));
        while !ext.len().is_multiple_of(2880) {
            ext.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&ext);
        for row in rows {
            for v in row {
                buf.extend_from_slice(&v.to_be_bytes());
            }
        }
        while !buf.len().is_multiple_of(2880) {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn container_magic_detected() {
        assert!(is_fits(b"SIMPLE  =                    T"));
        assert!(!is_fits(b"PK\x03\x04"));
        assert!(is_gzip(&[0x1f, 0x8b, 0x08]));
        assert!(!is_gzip(b"SIMPLE"));
        let mut tar = vec![0u8; 512];
        tar[257..262].copy_from_slice(b"ustar");
        assert!(is_tar(&tar));
        assert!(!is_tar(b"SIMPLE"));
    }

    #[test]
    fn grid_parser_reads_lon_x_lat() {
        assert_eq!(parse_grid("360x180"), Some((360, 180)));
        assert_eq!(parse_grid("360"), None);
        assert_eq!(parse_grid("0x10"), None);
        assert_eq!(parse_grid("axb"), None);
    }

    #[test]
    fn fits_payload_reduces_to_gamma_cells() {
        let buf = synth_events(&[[10.0, 0.0, 1.2], [10.0, 0.0, 0.8], [190.0, -45.0, 0.5]]);
        let events = events_from_payload(&buf).unwrap();
        assert_eq!(events.len(), 3);
        let records = records_of(&events, 36, 18);
        assert_eq!(records.len(), 2);
        assert!(records.iter().all(|r| r.kind == KIND_GAMMA));
        let first = records
            .iter()
            .find(|r| (r.value - 2.0).abs() < 1e-3)
            .unwrap();
        assert!((first.ra_deg as f64 - 15.0).abs() < 1e-3);
        assert!((first.dec_deg as f64 - 5.0).abs() < 1e-3);
        let second = records
            .iter()
            .find(|r| (r.value - 0.5).abs() < 1e-3)
            .unwrap();
        assert!((second.ra_deg as f64 - 195.0).abs() < 1e-3);
        assert!((second.dec_deg as f64 - -45.0).abs() < 1e-3);
    }

    #[test]
    fn absent_events_leave_the_payload_void() {
        let mut buf = vec![b'S'; 1000];
        buf.extend_from_slice(b"IMPLE  =                    T");
        assert!(events_from_payload(&buf).is_none());
    }

    #[test]
    fn non_positive_energy_cells_are_skipped() {
        let events = vec![Dl3Event {
            ra_deg: 10.0,
            dec_deg: 0.0,
            energy_tev: 0.0,
        }];
        assert!(records_of(&events, 36, 18).is_empty());
    }

    #[test]
    fn skymap_asset_roundtrips() {
        let events = vec![
            Dl3Event {
                ra_deg: 83.6331,
                dec_deg: 22.0145,
                energy_tev: 1.2,
            },
            Dl3Event {
                ra_deg: 83.6331,
                dec_deg: 22.0145,
                energy_tev: 0.8,
            },
        ];
        let records = records_of(&events, 360, 180);
        assert_eq!(records.len(), 1);
        let path = std::env::temp_dir().join("dl3_skymap_test.sky1");
        let p = path.to_str().unwrap();
        write_asset(&records, p).unwrap();
        verify_asset(p, &records).unwrap();
        let expect = HEADER_LEN + records.len() * REC_BYTES;
        assert_eq!(std::fs::metadata(p).unwrap().len() as usize, expect);
        let _ = std::fs::remove_file(p);
    }
}
