use omegaflow::fits::FitsHeader;
use omegaflow::healpix::{galactic_to_icrs, pix2ang_nest};
use omegaflow::json::{JsonVal, parse_json};

const Z_CMB: f64 = 1100.0;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

struct Table {
    nside: i64,
    npix: usize,
    data_start: usize,
    t_width: usize,
    width: usize,
}

fn read_table(bytes: &[u8]) -> Option<(Table, String)> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (h, data_start) = FitsHeader::parse(bytes, off)?;
    let nside = h.int("NSIDE")?;
    let npix = h.int("NAXIS2")? as usize;
    if 12 * nside * nside != npix as i64 {
        eprintln!(
            "NAXIS2 {} != 12*NSIDE^2 ({}): the table shape stays unread",
            npix, nside
        );
        return None;
    }
    let ordering = h.str_unescaped("ORDERING").unwrap_or_default();
    if !ordering.trim().eq_ignore_ascii_case("NESTED") {
        eprintln!(
            "ORDERING '{}': the compiler reads NESTED only — the map stays unwritten",
            ordering
        );
        return None;
    }
    let width = h.int("NAXIS1")? as usize;
    let tform = h.str_unescaped("TFORM1").unwrap_or_default();
    let t_width = if tform.contains('D') { 8 } else { 4 };
    if width < t_width {
        eprintln!(
            "NAXIS1 {} < column width {}: the I column stays unread",
            width, t_width
        );
        return None;
    }
    let ttype = h.str_unescaped("TTYPE1").unwrap_or_default();
    let tunit = h.str_unescaped("TUNIT1").unwrap_or_default();
    if data_start + npix * width > bytes.len() {
        eprintln!("table exceeds the fetched bytes — the map stays unwritten");
        return None;
    }
    Some((
        Table {
            nside,
            npix,
            data_start,
            t_width,
            width,
        },
        format!("{} {} {}", ttype, tunit, tform),
    ))
}

fn col_value(bytes: &[u8], table: &Table, row: usize, width: usize) -> Option<f64> {
    let off = table.data_start + row * width;
    if table.t_width == 8 {
        let raw = bytes.get(off..off + 8)?;
        Some(f64::from_be_bytes(raw.try_into().ok()?))
    } else {
        let raw = bytes.get(off..off + 4)?;
        Some(f32::from_be_bytes(raw.try_into().ok()?) as f64)
    }
}

fn degrade(bytes: &[u8], table: &Table, width: usize, nside_out: i64) -> Vec<Option<(f64, u64)>> {
    let ratio = table.nside / nside_out;
    let npix_out = (12 * nside_out * nside_out) as usize;
    let mut sum = vec![0.0f64; npix_out];
    let mut count = vec![0u64; npix_out];
    for r in 0..table.npix {
        let Some(v) = col_value(bytes, table, r, width) else {
            continue;
        };
        if !v.is_finite() {
            continue;
        }
        let c = (r as i64 / (ratio * ratio)) as usize;
        sum[c] += v;
        count[c] += 1;
    }
    (0..npix_out)
        .map(|c| {
            if count[c] == 0 {
                None
            } else {
                Some((sum[c] / count[c] as f64, count[c]))
            }
        })
        .collect()
}

fn write_json(rows: &[(f64, f64, f64)], path: &str) -> bool {
    let mut out = String::with_capacity(rows.len() * 64 + 2);
    out.push('[');
    for (i, (ra, dec, t)) in rows.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"ra\":{},\"dec\":{},\"z\":{},\"T\":{}}}",
            ra, dec, Z_CMB, t
        ));
    }
    out.push(']');
    if std::fs::write(path, out.as_bytes()).is_err() {
        eprintln!("write {path} returned void");
        return false;
    }
    true
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(input) = arg_value(&args, "--input") else {
        eprintln!(
            "usage: cmb_planck_compiler --input <fits> [--nside 64] [--out path] [--ci-mode]"
        );
        std::process::exit(1);
    };
    let nside_out: i64 = arg_value(&args, "--nside")
        .and_then(|v| v.parse().ok())
        .unwrap_or(64);
    let out = arg_value(&args, "--out")
        .unwrap_or_else(|| format!("cmb_planck_smica_n{}.json", nside_out));
    let ci_mode = has_flag(&args, "--ci-mode");

    let bytes = match std::fs::read(&input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {input} returned void: {e}");
            std::process::exit(1);
        }
    };
    let Some((table, col)) = read_table(&bytes) else {
        std::process::exit(1);
    };
    if nside_out <= 0 || nside_out > table.nside || table.nside % nside_out != 0 {
        eprintln!(
            "--nside {} is not a power-of-two divisor of NSIDE {} — the map stays unwritten",
            nside_out, table.nside
        );
        std::process::exit(1);
    }
    eprintln!(
        "NSIDE {} ({} pixels), column {}, ORDERING NESTED, degrade to NSIDE {}",
        table.nside, table.npix, col, nside_out
    );

    let coarse = degrade(&bytes, &table, table.width, nside_out);
    let mut rows: Vec<(f64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    let mut tsum = 0.0f64;
    let mut tmin = f64::INFINITY;
    let mut tmax = f64::NEG_INFINITY;
    for (c, cell) in coarse.iter().enumerate() {
        let Some((t, n)) = cell else {
            skipped += 1;
            continue;
        };
        let Some((theta, phi)) = pix2ang_nest(nside_out, c as i64) else {
            skipped += 1;
            continue;
        };
        let (ra, dec) = galactic_to_icrs(theta, phi);
        rows.push((ra, dec, *t));
        tsum += *t;
        if *t < tmin {
            tmin = *t;
        }
        if *t > tmax {
            tmax = *t;
        }
        let _ = n;
    }
    eprintln!(
        "{} rows, {} empty cells skipped, T mean {:.6} K, min {:.6} K, max {:.6} K",
        rows.len(),
        skipped,
        tsum / rows.len().max(1) as f64,
        tmin,
        tmax
    );
    if rows.is_empty() {
        eprintln!("no rows — the map stays unwritten (0 honored)");
        std::process::exit(1);
    }
    if !write_json(&rows, &out) {
        std::process::exit(1);
    }
    match std::fs::read_to_string(&out)
        .ok()
        .and_then(|s| parse_json(&s))
    {
        Some(JsonVal::Arr(arr)) => {
            eprintln!("{out}: {} rows roundtrip-parse", arr.len());
        }
        _ => {
            eprintln!("{out}: roundtrip parse void");
            std::process::exit(1);
        }
    }
    if ci_mode {
        let _ = omegaflow::cdn::upload_release("irsa.ipac.caltech.edu", &out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW_BYTES: usize = 40;

    fn card(kw: &str, val: &str) -> String {
        format!("{:<8}= {:<70}", kw, val)
    }

    fn synth_map(nside: i64, values: &[f64], ordering: &str) -> Vec<u8> {
        let npix = 12 * nside * nside;
        assert_eq!(values.len(), npix as usize);
        let mut buf: Vec<u8> = Vec::new();
        let primary = format!(
            "{}{}{}",
            card("SIMPLE", "T"),
            card("BITPIX", "8"),
            card("NAXIS", "0")
        );
        let mut hdr = primary.into_bytes();
        hdr.extend(card("END", "").into_bytes());
        while hdr.len() % 2880 != 0 {
            hdr.push(b' ');
        }
        buf.extend_from_slice(&hdr);
        let table = format!(
            "{}{}{}{}{}{}{}{}{}",
            card("XTENSION", "'BINTABLE'"),
            card("BITPIX", "8"),
            card("NAXIS", "2"),
            card("NAXIS1", "40"),
            card("NAXIS2", &npix.to_string()),
            card("NSIDE", &nside.to_string()),
            card("ORDERING", &format!("'{}'", ordering)),
            card("TTYPE1", "'I_STOKES'"),
            card("TFORM1", "'1E'")
        );
        let mut thdr = table.into_bytes();
        thdr.extend(card("END", "").into_bytes());
        while thdr.len() % 2880 != 0 {
            thdr.push(b' ');
        }
        buf.extend_from_slice(&thdr);
        for v in values {
            let mut row = [0u8; ROW_BYTES];
            row[0..4].copy_from_slice(&(*v as f32).to_be_bytes());
            buf.extend_from_slice(&row);
        }
        buf
    }

    #[test]
    fn degrade_uniform_map() {
        let nside_in = 4;
        let npix = (12 * nside_in * nside_in) as usize;
        let bytes = synth_map(nside_in, &vec![1.0; npix], "NESTED");
        let table = read_table(&bytes).unwrap().0;
        let coarse = degrade(&bytes, &table, ROW_BYTES, 1);
        assert_eq!(coarse.len(), 12);
        for c in &coarse {
            let (t, n) = c.unwrap();
            assert!((t - 1.0).abs() < 1e-12);
            assert_eq!(n, 16);
        }
    }

    #[test]
    fn degrade_skips_unseen() {
        let nside_in = 4;
        let npix = (12 * nside_in * nside_in) as usize;
        let mut vals = vec![1.0; npix];
        vals[0] = f64::NAN;
        let bytes = synth_map(nside_in, &vals, "NESTED");
        let table = read_table(&bytes).unwrap().0;
        let coarse = degrade(&bytes, &table, ROW_BYTES, 1);
        assert_eq!(coarse[0].unwrap().1, 15);
        assert!((coarse[0].unwrap().0 - 1.0).abs() < 1e-12);
        for c in coarse.iter().skip(1) {
            assert_eq!(c.unwrap().1, 16);
        }
    }

    #[test]
    fn degrade_keeps_signed_fluctuations() {
        let nside_in = 4;
        let npix = (12 * nside_in * nside_in) as usize;
        let mut vals = vec![1.0; npix];
        for v in vals.iter_mut().take(16) {
            *v = -1.0;
        }
        let bytes = synth_map(nside_in, &vals, "NESTED");
        let table = read_table(&bytes).unwrap().0;
        let coarse = degrade(&bytes, &table, ROW_BYTES, 1);
        assert!((coarse[0].unwrap().0 + 1.0).abs() < 1e-12);
        assert_eq!(coarse[0].unwrap().1, 16);
        assert!((coarse[1].unwrap().0 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn read_table_refuses_ring() {
        let nside = 1;
        let bytes = synth_map(nside, &vec![1.0; 12], "RING");
        assert!(read_table(&bytes).is_none());
    }
}
