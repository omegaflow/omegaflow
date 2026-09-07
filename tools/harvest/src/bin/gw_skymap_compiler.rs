use omegaflow::cdn::upload_asset;
use omegaflow::zeuge::{magic_identity, FeldIdentitaet, ZeugeArt};
use omegaflow::fits::{FitsHeader, FitsTable};
use omegaflow::healpix::pix2ang_nest;
use omegaflow::skymap::{
    decode_rec, encode_rec, parse_header, write_header, SkymapRecord, HEADER_LEN, KIND_GRAVITY,
    REC_BYTES,
};
use std::io::{BufWriter, Write};

const PI4: f64 = 4.0 * std::f64::consts::PI;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn uniq_order_ipix(uniq: u64) -> Option<(u8, u32)> {
    if uniq < 4 {
        return None;
    }
    let mut order: u32 = 0;
    let mut base: u64 = 4;
    loop {
        let next = base.checked_mul(4)?;
        if uniq < next {
            break;
        }
        base = next;
        order += 1;
        if order > 29 {
            return None;
        }
    }
    let npix_order = 12u64.checked_mul(1u64 << (2 * order))?;
    let ipix = uniq - base;
    if ipix >= npix_order {
        return None;
    }
    if ipix > u32::MAX as u64 {
        return None;
    }
    Some((order as u8, ipix as u32))
}

#[derive(Debug)]
struct Report {
    rows: u64,
    decoded: u64,
    uniq_invalid: u64,
    cell_unread: u64,
    density_invalid: u64,
    density_negative: u64,
    overflow: u64,
    dup_pixel: u64,
    integral: f64,
    object: String,
    ordering: String,
    coordsys: String,
}

fn compile_map(input: &[u8], out: &mut Vec<u8>) -> Result<Report, String> {
    let (primary, first_off) =
        FitsHeader::parse(input, 0).ok_or("the primary FITS header stayed unread")?;
    let mut off = first_off;
    let (table_header, hdu_start) = loop {
        let (h, next) = FitsHeader::parse(input, off).ok_or("a FITS header stayed unread")?;
        if h.value("XTENSION") == Some("'BINTABLE'") {
            break (h, off);
        }
        off = next;
    };
    let (table, _) =
        FitsTable::parse(input, hdu_start).ok_or("the BINTABLE header stayed unread")?;

    let ordering = table_header
        .str_unescaped("ORDERING")
        .or_else(|| primary.str_unescaped("ORDERING"))
        .ok_or("the ORDERING card is absent from the header")?
        .trim()
        .to_string();
    if ordering != "NUNIQ" {
        return Err(format!(
            "ORDERING = {ordering}, the compiler reads NUNIQ multiorder maps"
        ));
    }
    let coordsys = match table_header
        .str_unescaped("COORDSYS")
        .or_else(|| primary.str_unescaped("COORDSYS"))
    {
        Some(s) => s.trim().to_string(),
        None => String::new(),
    };
    if !coordsys.is_empty() && coordsys != "C" {
        return Err(format!(
            "COORDSYS = {coordsys}, the map is not celestial (C)"
        ));
    }
    let object = match table_header
        .str_unescaped("OBJECT")
        .or_else(|| primary.str_unescaped("OBJECT"))
    {
        Some(s) => s.trim().to_string(),
        None => String::new(),
    };
    let indxschm = match table_header
        .str_unescaped("INDXSCHM")
        .or_else(|| primary.str_unescaped("INDXSCHM"))
    {
        Some(s) => s.trim().to_string(),
        None => String::new(),
    };
    if !indxschm.is_empty() && indxschm != "EXPLICIT" {
        return Err(format!(
            "INDXSCHM = {indxschm}, the compiler reads explicit UNIQ rows"
        ));
    }

    let uniq_col = table
        .column("UNIQ")
        .ok_or("the UNIQ column is absent from the table")?;
    if !matches!(uniq_col.code, 'K' | 'J') {
        return Err(format!(
            "UNIQ column code {} is not an integer type",
            uniq_col.code
        ));
    }
    let prob_col = table
        .column("PROBDENSITY")
        .ok_or("the PROBDENSITY column is absent from the table")?;
    if !matches!(prob_col.code, 'D' | 'E') {
        return Err(format!(
            "PROBDENSITY column code {} is not a floating type",
            prob_col.code
        ));
    }

    let mut report = Report {
        rows: table.n_rows as u64,
        decoded: 0,
        uniq_invalid: 0,
        cell_unread: 0,
        density_invalid: 0,
        density_negative: 0,
        overflow: 0,
        dup_pixel: 0,
        integral: 0.0,
        object,
        ordering,
        coordsys,
    };
    let mut seen = std::collections::HashSet::new();
    let mut records: Vec<SkymapRecord> = Vec::new();
    for i in 0..table.n_rows {
        let uniq = match table.cell_i64(input, i, uniq_col) {
            Some(u) if u >= 0 => u as u64,
            _ => {
                report.cell_unread += 1;
                continue;
            }
        };
        let Some((order, ipix)) = uniq_order_ipix(uniq) else {
            report.uniq_invalid += 1;
            continue;
        };
        let density = match table.cell_f64(input, i, prob_col) {
            Some(d) => d,
            None => {
                report.cell_unread += 1;
                continue;
            }
        };
        if !density.is_finite() {
            report.density_invalid += 1;
            continue;
        }
        if density < 0.0 {
            report.density_negative += 1;
            continue;
        }
        let value = density as f32;
        if !value.is_finite() {
            report.overflow += 1;
            continue;
        }
        let key = ((order as u64) << 32) | (ipix as u64);
        if !seen.insert(key) {
            report.dup_pixel += 1;
        }
        let nside = 1i64 << order;
        let Some((theta, phi)) = pix2ang_nest(nside, ipix as i64) else {
            report.cell_unread += 1;
            continue;
        };
        let dec_deg = 90.0 - theta.to_degrees();
        let ra_deg = phi.to_degrees();
        let omega = PI4 / (12.0 * 4u32.pow(order as u32) as f64);
        report.integral += density * omega;
        records.push(SkymapRecord {
            order,
            kind: KIND_GRAVITY,
            ipix,
            ra_deg: ra_deg as f32,
            dec_deg: dec_deg as f32,
            value,
        });
        report.decoded += 1;
    }

    if records.is_empty() {
        return Err("no decoded row — the asset stays unwritten (0 honored)".into());
    }

    let mut hbuf = Vec::with_capacity(HEADER_LEN);
    write_header(&mut hbuf, records.len() as u64);
    out.extend_from_slice(&hbuf);
    let mut rec = [0u8; REC_BYTES];
    for r in &records {
        encode_rec(&mut rec, r);
        out.extend_from_slice(&rec);
    }
    Ok(report)
}

fn witness_s2_direction_identity(magic: [u8; 4]) -> Result<(), String> {
    match magic_identity(magic) {
        Some(FeldIdentitaet::Zeuge(ZeugeArt::S2Richtung)) => {
            eprintln!(
                "{} reads as an s2-direction witness record",
                String::from_utf8_lossy(&magic)
            );
            Ok(())
        }
        Some(other) => Err(format!(
            "{} reads {:?}, not s2-direction — the asset stays unwritten",
            String::from_utf8_lossy(&magic),
            other
        )),
        None => Err(format!(
            "{} reads no identity — the asset stays unwritten",
            String::from_utf8_lossy(&magic)
        )),
    }
}

fn run(args: &[String]) -> Result<(), String> {
    witness_s2_direction_identity(omegaflow::skymap::MAGIC)?;
    let input = match arg_value(args, "--input") {
        Some(v) => v,
        None => {
            return Err(
                "usage: gw_skymap_compiler --input <bayestar.multiorder.fits> --out <map> \
                 [--ci-mode] — refused"
                    .into(),
            )
        }
    };
    let out_path = match arg_value(args, "--out") {
        Some(v) => v,
        None => return Err("--out <map>: the asset path is never silent — refused".into()),
    };
    let ci_mode = args.iter().any(|a| a == "--ci-mode");

    let data = std::fs::read(&input).map_err(|e| format!("read {input} returned void: {e}"))?;
    let mut asset: Vec<u8> = Vec::with_capacity(HEADER_LEN + (data.len() / 40) * REC_BYTES);
    let report = compile_map(&data, &mut asset)?;

    let mut w = BufWriter::with_capacity(
        1 << 20,
        std::fs::File::create(&out_path)
            .map_err(|e| format!("create {out_path} returned void: {e}"))?,
    );
    w.write_all(&asset)
        .map_err(|e| format!("write {out_path} returned void: {e}"))?;
    let _ = w.flush();
    let _ = w.into_inner();

    let expect = HEADER_LEN as u64 + report.decoded * REC_BYTES as u64;
    let actual = std::fs::metadata(&out_path)
        .map_err(|e| format!("stat {out_path} returned void: {e}"))?
        .len();
    if actual != expect {
        return Err(format!(
            "{out_path}: {actual} bytes written, {expect} expected — the asset stays unwritten"
        ));
    }
    let mut vf = std::fs::File::open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    let mut head = [0u8; HEADER_LEN];
    use std::io::Read;
    vf.read_exact(&mut head)
        .map_err(|e| format!("read {out_path} header returned void: {e}"))?;
    let n_rows =
        parse_header(&head).ok_or_else(|| format!("{out_path}: the header stays unread"))?;
    let last_off = HEADER_LEN as u64 + (report.decoded - 1) * REC_BYTES as u64;
    use std::io::Seek;
    use std::io::SeekFrom;
    vf.seek(SeekFrom::Start(last_off))
        .map_err(|e| format!("seek {out_path} returned void: {e}"))?;
    let mut tail = vec![0u8; REC_BYTES];
    vf.read_exact(&mut tail)
        .map_err(|e| format!("read {out_path} tail returned void: {e}"))?;
    let last =
        decode_rec(&tail).ok_or_else(|| format!("{out_path}: the last record stays unread"))?;

    eprintln!(
        "{out_path}: object {} ordering {} coordsys {}, {} of {} rows decoded",
        report.object, report.ordering, report.coordsys, report.decoded, report.rows
    );
    eprintln!(
        "{} rows: {} density-sr^-1 per pixel of kind {} — header {} — roundtrip verified",
        report.decoded, report.rows, KIND_GRAVITY, n_rows
    );
    eprintln!(
        "last pixel: order {} ipix {} ra {:.4} dec {:.4} density {:.6}",
        last.order, last.ipix, last.ra_deg, last.dec_deg, last.value
    );
    eprintln!(
        "skipped: {} invalid-uniq, {} unreadable-cell, {} non-finite-density, {} negative-density, {} f32-overflow, {} duplicate-pixel",
        report.uniq_invalid,
        report.cell_unread,
        report.density_invalid,
        report.density_negative,
        report.overflow,
        report.dup_pixel
    );
    eprintln!(
        "sky integral of the decoded density: {:.6} sr^-1 * sr (bayestar marginal normalizes to 1)",
        report.integral
    );
    if ci_mode && !upload_asset(&out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("gw_skymap_compiler: {msg}");
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
        card[10..10 + v.len().min(68)].copy_from_slice(&v[..v.len().min(68)]);
        card
    }

    fn hdu_header(cards: &[(&str, &str)]) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        for (k, v) in cards {
            header.extend_from_slice(&pad_card(k, v));
        }
        header.extend_from_slice(&pad_card("END", ""));
        while header.len() % 2880 != 0 {
            header.extend_from_slice(&[b' '; 80]);
        }
        header
    }

    fn synthetic_map(rows: &[(u32, u32, f64)]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&hdu_header(&[
            ("SIMPLE", "T"),
            ("BITPIX", "8"),
            ("NAXIS", "0"),
        ]));
        let n_rows = rows.len().to_string();
        let ext = hdu_header(&[
            ("XTENSION", "'BINTABLE'"),
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "16"),
            ("NAXIS2", &n_rows),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "2"),
            ("TTYPE1", "'UNIQ'"),
            ("TFORM1", "K"),
            ("TTYPE2", "'PROBDENSITY'"),
            ("TFORM2", "D"),
            ("ORDERING", "'NUNIQ'"),
            ("INDXSCHM", "'EXPLICIT'"),
            ("PIXTYPE", "'HEALPIX'"),
            ("COORDSYS", "'C'"),
            ("MOC", "T"),
            ("OBJECT", "'SYNTEST'"),
        ]);
        buf.extend_from_slice(&ext);
        for &(order, ipix, density) in rows {
            let base = 4u64.pow(order + 1);
            buf.extend_from_slice(&(base + ipix as u64).to_be_bytes());
            buf.extend_from_slice(&density.to_be_bytes());
        }
        while buf.len() % 2880 != 0 {
            buf.push(0);
        }
        buf
    }

    #[test]
    fn uniq_decode_roundtrip_across_orders() {
        for order in 0..14u32 {
            for ipix in [0u64, 3, 11, 12 * 4u64.pow(order) - 1] {
                let base = 4u64.pow(order + 1);
                let uniq = base + ipix;
                let (o, p) = uniq_order_ipix(uniq).unwrap();
                assert_eq!(o as u32, order);
                assert_eq!(p as u64, ipix);
            }
        }
    }

    #[test]
    fn uniq_decode_refuses_out_of_range() {
        assert!(uniq_order_ipix(0).is_none());
        assert!(uniq_order_ipix(1).is_none());
        assert!(uniq_order_ipix(2).is_none());
        assert!(uniq_order_ipix(3).is_none());
        assert!(uniq_order_ipix(u64::MAX).is_none());
        assert!(uniq_order_ipix(4u64.pow(31)).is_none());
        assert!(uniq_order_ipix(u64::MAX / 2 + 1).is_none());
    }

    #[test]
    fn synthetic_nuniq_map_compiles_to_gravity_skymap() {
        let rows: Vec<(u32, u32, f64)> = vec![
            (1, 3, 1.0 / (PI4 / 48.0)),
            (2, 44, 0.5 / (PI4 / 192.0)),
            (3, 500, 0.25 / (PI4 / 768.0)),
        ];
        let omega_of = |order: u32| PI4 / (12.0 * 4u32.pow(order) as f64);
        let expect_integral: f64 = rows.iter().map(|&(o, _, d)| d * omega_of(o)).sum();
        let buf = synthetic_map(&rows);
        let mut asset = Vec::new();
        let report = compile_map(&buf, &mut asset).unwrap();
        assert_eq!(report.decoded, 3);
        assert_eq!(report.uniq_invalid, 0);
        assert_eq!(report.cell_unread, 0);
        assert!((report.integral - expect_integral).abs() < 1e-12);

        let n = parse_header(&asset).unwrap();
        assert_eq!(n, 3);
        assert_eq!(asset.len(), HEADER_LEN + 3 * REC_BYTES);
        let rec = decode_rec(&asset[HEADER_LEN..HEADER_LEN + REC_BYTES]).unwrap();
        assert_eq!(rec.kind, KIND_GRAVITY);
        assert_eq!(rec.order, 1);
        assert_eq!(rec.ipix, 3);
        assert!(rec.ra_deg.is_finite());
        assert!(rec.dec_deg.is_finite());
        let last = decode_rec(&asset[HEADER_LEN + 2 * REC_BYTES..]).unwrap();
        assert_eq!(last.order, 3);
        assert_eq!(last.ipix, 500);
        let want = (0.25 / (PI4 / 768.0)) as f32;
        assert!(((last.value - want).abs() / want) < 1e-6);
    }

    #[test]
    fn compile_refuses_ring_ordered_map() {
        let mut buf = synthetic_map(&[(1, 3, 1.0)]);
        let needle = b"'NUNIQ'";
        let pos = buf.windows(needle.len()).position(|w| w == needle).unwrap();
        buf[pos..pos + 7].copy_from_slice(b"'RING '");
        let mut asset = Vec::new();
        let err = compile_map(&buf, &mut asset).unwrap_err();
        assert!(err.contains("reads NUNIQ"));
        assert!(asset.is_empty());
    }
}
