use omegaflow::archivar::fetch_raw_bytes;
use omegaflow::archivar::fugin::{FuginPixel, parse_fugin_cube};
use omegaflow::cdn::upload_release;
use omegaflow::skymap::{
    HEADER_LEN, KIND_GENERIC, REC_BYTES, SkymapRecord, decode_rec, encode_rec, parse_header,
    write_header,
};
use std::io::{BufWriter, Write};

const CDN_TAG: &str = "jvo.nao.ac.jp";

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn record_of(pixel: &FuginPixel) -> Option<SkymapRecord> {
    if !pixel.moment0_k_ms.is_finite() {
        return None;
    }
    let (order, ipix) = SkymapRecord::pixel_of(pixel.ra_deg, pixel.dec_deg)?;
    let value = pixel.moment0_k_ms as f32;
    if !value.is_finite() {
        return None;
    }
    Some(SkymapRecord {
        order,
        kind: KIND_GENERIC,
        ipix,
        ra_deg: pixel.ra_deg as f32,
        dec_deg: pixel.dec_deg as f32,
        value,
    })
}

fn records_of(pixels: &[FuginPixel]) -> Vec<SkymapRecord> {
    pixels.iter().filter_map(record_of).collect()
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
    let out = match arg_value(&args, "--out") {
        Some(v) => v,
        None => {
            eprintln!("fugin_skymap_compiler: --out <path> absent — the output path is never silent");
            std::process::exit(1);
        }
    };
    let input = arg_value(&args, "--input");
    let url = arg_value(&args, "--url");
    let bytes = match (input, url) {
        (Some(_), Some(_)) => {
            eprintln!(
                "fugin_skymap_compiler: --input and --url both present — exactly one source is the contract"
            );
            std::process::exit(1);
        }
        (Some(path), None) => match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("fugin_skymap_compiler: read {path}: {e} — the cube stays unread");
                std::process::exit(1);
            }
        },
        (None, Some(route)) => match fetch_raw_bytes(&route, 604800) {
            Some(b) => b,
            None => {
                eprintln!("fugin_skymap_compiler: fetch void ({route})");
                std::process::exit(1);
            }
        },
        (None, None) => {
            eprintln!("fugin_skymap_compiler: --input <file> or --url <https> absent — refused");
            std::process::exit(1);
        }
    };
    let Some(pixels) = parse_fugin_cube(&bytes) else {
        eprintln!(
            "fugin_skymap_compiler: the payload carries no NAXIS=3 VRAD cube (no moment-0 integral over the velocity axis) — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    };
    let records = records_of(&pixels);
    if records.is_empty() {
        eprintln!(
            "fugin_skymap_compiler: no finite moment-0 pixel projects to a healpix cell — the asset stays unwritten (0 honored)"
        );
        std::process::exit(1);
    }
    if let Err(e) = write_asset(&records, &out) {
        eprintln!("fugin_skymap_compiler: {e}");
        std::process::exit(1);
    }
    if let Err(e) = verify_asset(&out, &records) {
        eprintln!("fugin_skymap_compiler: {e}");
        std::process::exit(1);
    }
    let bytes_written = HEADER_LEN + records.len() * REC_BYTES;
    eprintln!(
        "fugin: {} native FITS pixels -> {out}: {} records, {} B — value = moment-0 (K m/s) per native pixel, kind generic",
        pixels.len(),
        records.len(),
        bytes_written
    );
    if ci_mode && !upload_release(CDN_TAG, &out) {
        eprintln!("fugin_skymap_compiler: upload {out} did not reach the CDN");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fits_card(kw: &str, value: &str) -> [u8; 80] {
        let mut card = [b' '; 80];
        let k = kw.as_bytes();
        card[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        card[8] = b'=';
        let v = value.as_bytes();
        card[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        card
    }

    fn fugin_cube_fixture_with_negative() -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(&fits_card("SIMPLE", "T"));
        header.extend_from_slice(&fits_card("BITPIX", "-32"));
        header.extend_from_slice(&fits_card("NAXIS", "3"));
        header.extend_from_slice(&fits_card("NAXIS1", "2"));
        header.extend_from_slice(&fits_card("NAXIS2", "2"));
        header.extend_from_slice(&fits_card("NAXIS3", "2"));
        header.extend_from_slice(&fits_card("CTYPE1", "'GLON-SFL'"));
        header.extend_from_slice(&fits_card("CTYPE2", "'GLAT-SFL'"));
        header.extend_from_slice(&fits_card("CTYPE3", "'VRAD'"));
        header.extend_from_slice(&fits_card("CRVAL1", "20.0"));
        header.extend_from_slice(&fits_card("CRVAL2", "0.0"));
        header.extend_from_slice(&fits_card("CRPIX1", "1.0"));
        header.extend_from_slice(&fits_card("CRPIX2", "1.0"));
        header.extend_from_slice(&fits_card("CDELT1", "0.5"));
        header.extend_from_slice(&fits_card("CDELT2", "0.5"));
        header.extend_from_slice(&fits_card("CDELT3", "100.0"));
        header.extend_from_slice(&fits_card("CUNIT3", "'m/s'"));
        header.extend_from_slice(&fits_card("END", ""));
        while !header.len().is_multiple_of(2880) {
            header.extend_from_slice(&[b' '; 80]);
        }
        buf.extend_from_slice(&header);
        let (nx, ny, nz) = (2usize, 2usize, 2usize);
        let mut data = vec![f32::NAN; nx * ny * nz];
        let mut set = |x: usize, y: usize, z: usize, v: f32| {
            data[z * (ny * nx) + y * nx + x] = v;
        };
        set(0, 0, 0, 4.0);
        set(0, 0, 1, 5.0);
        set(1, 1, 0, -1.0);
        set(1, 1, 1, -2.0);
        for v in data {
            buf.extend_from_slice(&v.to_be_bytes());
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn signed_moment0_pixel_survives() {
        let pixels = vec![
            FuginPixel {
                ra_deg: 30.0,
                dec_deg: 10.0,
                moment0_k_ms: -300.0,
            },
            FuginPixel {
                ra_deg: 31.0,
                dec_deg: 11.0,
                moment0_k_ms: 900.0,
            },
        ];
        let records = records_of(&pixels);
        assert_eq!(records.len(), 2, "a negative moment-0 pixel is a real measurement");
        let neg = records
            .iter()
            .find(|r| r.value < 0.0)
            .expect("the negative pixel stays a record, never dropped by a positive gate");
        assert!((neg.value as f64 + 300.0).abs() < 1e-3);
        let pos = records
            .iter()
            .find(|r| r.value > 0.0)
            .expect("the positive pixel stays a record");
        assert!((pos.value as f64 - 900.0).abs() < 1e-3);
    }

    #[test]
    fn negative_moment0_cube_survives_end_to_end() {
        let buf = fugin_cube_fixture_with_negative();
        let pixels = parse_fugin_cube(&buf).expect("2x2x2 VRAD cube parses");
        assert_eq!(pixels.len(), 2, "all-NaN pixels stay absent, two carry finite channels");
        let records = records_of(&pixels);
        assert_eq!(records.len(), 2);
        assert!(
            records.iter().any(|r| r.value < 0.0),
            "the negative moment-0 pixel is a real measurement, never fabricated away"
        );
    }

    #[test]
    fn non_finite_moment0_pixel_is_skipped() {
        let pixels = vec![
            FuginPixel {
                ra_deg: 30.0,
                dec_deg: 10.0,
                moment0_k_ms: f64::NAN,
            },
            FuginPixel {
                ra_deg: 31.0,
                dec_deg: 11.0,
                moment0_k_ms: 1.0,
            },
        ];
        let records = records_of(&pixels);
        assert_eq!(records.len(), 1);
        assert!((records[0].value as f64 - 1.0).abs() < 1e-3);
    }

    #[test]
    fn record_roundtrips_through_encode_decode() {
        let pixels = vec![FuginPixel {
            ra_deg: 148.8746,
            dec_deg: 2.5208,
            moment0_k_ms: -42.0,
        }];
        let records = records_of(&pixels);
        assert_eq!(records.len(), 1);
        let mut b = [0u8; REC_BYTES];
        encode_rec(&mut b, &records[0]);
        let back = decode_rec(&b).unwrap();
        assert_eq!(back.order, records[0].order);
        assert_eq!(back.kind, KIND_GENERIC);
        assert_eq!(back.ipix, records[0].ipix);
        assert_eq!(back.ra_deg, records[0].ra_deg);
        assert_eq!(back.dec_deg, records[0].dec_deg);
        assert_eq!(back.value, records[0].value);
    }

    #[test]
    fn pixel_of_applies_to_native_centers_not_aggregated() {
        let pixels = vec![
            FuginPixel {
                ra_deg: 20.0,
                dec_deg: 30.0,
                moment0_k_ms: 1.0,
            },
            FuginPixel {
                ra_deg: 20.4,
                dec_deg: 30.4,
                moment0_k_ms: 2.0,
            },
        ];
        let records = records_of(&pixels);
        assert_eq!(
            records.len(),
            2,
            "one native FITS pixel = one record, never a healpix cell bin aggregate"
        );
        for (p, r) in pixels.iter().zip(records.iter()) {
            assert!(
                (r.ra_deg as f64 - p.ra_deg).abs() < 1e-3,
                "ra preserved through pixel_of"
            );
            assert!(
                (r.dec_deg as f64 - p.dec_deg).abs() < 1e-3,
                "dec preserved through pixel_of"
            );
        }
    }
}
