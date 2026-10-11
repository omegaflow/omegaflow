use omegaflow::fits::{FitsHeader, FitsTable};
use omegaflow::healpix::{ang2pix_nest, galactic_to_icrs, pix2ang_nest, pix2ang_ring};
use omegaflow::json::{JsonVal, parse_json};

const Z_CMB: f64 = 1100.0;

const SPT_TRANSFER_BOUND_S: u64 = 6 * 3600;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn unit_factor(u: &str) -> Option<f64> {
    let s = u.trim();
    if s.starts_with("uK") || s.starts_with("\u{b5}K") || s.starts_with("microK") {
        Some(1e-6)
    } else if s.starts_with("mK") {
        Some(1e-3)
    } else if s.starts_with('K') {
        Some(1.0)
    } else {
        None
    }
}

struct Table {
    nside: i64,
    npix: usize,
    data_start: usize,
    t_width: usize,
    width: usize,
    unit: f64,
}

struct PartialTable {
    nside: i64,
    fits: FitsTable,
    pixel_idx: usize,
    temp_idx: usize,
    unit: f64,
}

enum Map {
    Full(Table, String),
    Partial(PartialTable, String),
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
    let ordering = match h.str_unescaped("ORDERING") {
        Some(o) => o,
        None => {
            eprintln!("ORDERING absent: the map stays unwritten");
            return None;
        }
    };
    if !ordering.trim().eq_ignore_ascii_case("NESTED") {
        eprintln!(
            "ORDERING '{}': the compiler reads NESTED only — the map stays unwritten",
            ordering
        );
        return None;
    }
    let width = h.int("NAXIS1")? as usize;
    let tform = match h.str_unescaped("TFORM1") {
        Some(t) => t,
        None => {
            eprintln!("TFORM1 absent: the I column stays unread");
            return None;
        }
    };
    let t_width = if tform.contains('D') { 8 } else { 4 };
    if width < t_width {
        eprintln!(
            "NAXIS1 {} < column width {}: the I column stays unread",
            width, t_width
        );
        return None;
    }
    let ttype = h.str_unescaped("TTYPE1");
    let unit_text = match h
        .str_unescaped("TUNIT1")
        .or_else(|| h.str_unescaped("BUNIT"))
    {
        Some(u) => u,
        None => {
            eprintln!("TUNIT1 and BUNIT absent: the unit stays unread");
            return None;
        }
    };
    let unit = match unit_factor(&unit_text) {
        Some(u) => u,
        None => {
            eprintln!(
                "TUNIT1/BUNIT '{}' unknown: the unit stays unread",
                unit_text
            );
            return None;
        }
    };
    if data_start + npix * width > bytes.len() {
        eprintln!("table exceeds the fetched bytes — the map stays unwritten");
        return None;
    }
    let label = match ttype {
        Some(t) => format!("{} {} {}", t, unit_text, tform),
        None => format!("{} {}", unit_text, tform),
    };
    Some((
        Table {
            nside,
            npix,
            data_start,
            t_width,
            width,
            unit,
        },
        label,
    ))
}

fn read_table_partial(bytes: &[u8]) -> Option<(PartialTable, String)> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (h, hdu_start) = FitsHeader::parse(bytes, off)?;
    let (fits, _) = FitsTable::parse(bytes, hdu_start)?;
    let nside = h.int("NSIDE")?;
    if nside <= 0 || (nside & (nside - 1)) != 0 {
        eprintln!("NSIDE {nside} is not a positive power of two — the map stays unwritten");
        return None;
    }
    let npix_full = 12 * nside * nside;
    if fits.n_rows == 0 {
        eprintln!("NAXIS2 0: no EXPLICIT rows — the map stays unwritten (0 honored)");
        return None;
    }
    if fits.n_rows as i64 > npix_full {
        eprintln!(
            "NAXIS2 {} exceeds 12*NSIDE^2 ({npix_full}): the table shape stays unread",
            fits.n_rows
        );
        return None;
    }
    let pixel_idx = fits.columns.iter().position(|c| c.name == "PIXEL")?;
    if !matches!(fits.columns[pixel_idx].code, 'J' | 'I' | 'K') {
        eprintln!("PIXEL column is not an integer type — the map stays unwritten");
        return None;
    }
    let temp_idx = fits.columns.iter().position(|c| c.name == "TEMPERATURE")?;
    if !matches!(fits.columns[temp_idx].code, 'D' | 'E') {
        eprintln!("TEMPERATURE column is not a floating type — the map stays unwritten");
        return None;
    }
    let unit_text = fits.columns[temp_idx]
        .unit
        .clone()
        .or_else(|| h.str_unescaped("BUNIT"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let unit = match unit_text.as_deref().and_then(unit_factor) {
        Some(u) => u,
        None => {
            eprintln!("TEMPERATURE TUNIT/BUNIT absent or unknown: the unit stays unread");
            return None;
        }
    };
    let label = format!(
        "PIXEL/TEMPERATURE {}",
        unit_text.as_deref().unwrap_or("(unit absent)")
    );
    Some((
        PartialTable {
            nside,
            fits,
            pixel_idx,
            temp_idx,
            unit,
        },
        label,
    ))
}

fn read_map(bytes: &[u8]) -> Option<Map> {
    let (_, off) = FitsHeader::parse(bytes, 0)?;
    let (h, hdu_start) = FitsHeader::parse(bytes, off)?;
    let ordering = h
        .str_unescaped("ORDERING")
        .map(|s| s.trim().to_ascii_uppercase());
    let indxschm = h
        .str_unescaped("INDXSCHM")
        .map(|s| s.trim().to_ascii_uppercase());
    let has_pixel = FitsTable::parse(bytes, hdu_start)
        .map(|(t, _)| t.column("PIXEL").is_some())
        .unwrap_or(false);
    let partial =
        ordering.as_deref() == Some("RING") || indxschm.as_deref() == Some("EXPLICIT") || has_pixel;
    if partial {
        read_table_partial(bytes).map(|(t, l)| Map::Partial(t, l))
    } else {
        read_table(bytes).map(|(t, l)| Map::Full(t, l))
    }
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
                Some((sum[c] / count[c] as f64 * table.unit, count[c]))
            }
        })
        .collect()
}

fn degrade_partial(bytes: &[u8], table: &PartialTable, nside_out: i64) -> Vec<Option<(f64, u64)>> {
    let npix_out = (12 * nside_out * nside_out) as usize;
    let mut sum = vec![0.0f64; npix_out];
    let mut count = vec![0u64; npix_out];
    let npix_in = 12 * table.nside * table.nside;
    let pixel = &table.fits.columns[table.pixel_idx];
    let temp = &table.fits.columns[table.temp_idx];
    for r in 0..table.fits.n_rows {
        let Some(p) = table.fits.cell_i64(bytes, r, pixel) else {
            continue;
        };
        if p < 0 || p >= npix_in {
            continue;
        }
        let Some(v) = table.fits.cell_f64(bytes, r, temp) else {
            continue;
        };
        if !v.is_finite() {
            continue;
        }
        let Some((theta, phi)) = pix2ang_ring(table.nside, p) else {
            continue;
        };
        let Some(c) = ang2pix_nest(nside_out, theta, phi) else {
            continue;
        };
        let ci = c as usize;
        sum[ci] += v;
        count[ci] += 1;
    }
    (0..npix_out)
        .map(|c| {
            if count[c] == 0 {
                None
            } else {
                Some((sum[c] / count[c] as f64 * table.unit, count[c]))
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

#[cfg(test)]
fn select_fits_member<'a>(
    members: &'a [omegaflow::archivar::inflate::TarMember],
    tar: &'a [u8],
    want: Option<&str>,
) -> Option<(&'a omegaflow::archivar::inflate::TarMember, &'a [u8])> {
    let member = match want {
        Some(name) => members.iter().find(|m| m.name == name)?,
        None => members
            .iter()
            .find(|m| m.name.to_ascii_lowercase().ends_with(".fits"))?,
    };
    let data = tar.get(member.start..member.end)?;
    Some((member, data))
}

fn stream_fits_member_to_file(
    path: &str,
    want: Option<&str>,
    out: &std::path::Path,
) -> Option<String> {
    use std::io::Write;
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open {path} returned void: {e}");
            return None;
        }
    };
    let exact = want.map(str::to_string);
    let mut want_fn = |name: &str| match &exact {
        Some(w) => name == w,
        None => name.to_ascii_lowercase().ends_with(".fits"),
    };
    let mut out_file: Option<std::fs::File> = None;
    let mut written = String::new();
    {
        let mut emit = |name: &str, part: Option<&[u8]>| match part {
            Some(d) => {
                if out_file.is_none() {
                    match std::fs::File::create(out) {
                        Ok(f) => {
                            written = name.to_string();
                            out_file = Some(f);
                        }
                        Err(e) => {
                            eprintln!("create {} returned void: {e}", out.display());
                        }
                    }
                }
                if let Some(f) = out_file.as_mut() {
                    if f.write_all(d).is_err() {
                        out_file = None;
                    }
                }
            }
            None => {
                if let Some(mut f) = out_file.take() {
                    let _ = f.flush();
                }
            }
        };
        let reader = std::io::BufReader::with_capacity(1 << 16, file);
        if let Err(e) =
            omegaflow::archivar::bzip2::bzip2_tar_stream(reader, &mut want_fn, &mut emit)
        {
            eprintln!("bzip2 tar stream of {path} returned void: {e}");
            return None;
        }
    }
    if written.is_empty() {
        None
    } else {
        Some(written)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let nside_out: i64 = arg_value(&args, "--nside")
        .and_then(|v| v.parse().ok())
        .unwrap_or(64);
    let spt_url = arg_value(&args, "--url");
    let tar_path = arg_value(&args, "--tar");
    let is_spt = spt_url.is_some() || tar_path.is_some();
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None if is_spt => format!("cmb_spt_d1_n{}.json", nside_out),
        None => format!("cmb_planck_smica_n{}.json", nside_out),
    };
    let ci_mode = has_flag(&args, "--ci-mode");
    let member = arg_value(&args, "--member");

    let bytes = match (spt_url, tar_path) {
        (Some(url), _) => {
            let temp_tar =
                std::env::temp_dir().join(format!("cmb_spt_url_{}.tar.bz2", std::process::id()));
            let Some(temp_tar_str) = temp_tar.to_str() else {
                eprintln!(
                    "temp tarball path {} is not UTF-8: the map stays unwritten",
                    temp_tar.display()
                );
                std::process::exit(1);
            };
            match omegaflow::archivar::fetch::fetch_raw_to_file_with(
                &url,
                omegaflow::archivar::fetch::RetryPolicy::Transient,
                SPT_TRANSFER_BOUND_S,
                &temp_tar,
            ) {
                Some(n) => eprintln!("fetch {url}: {n} bytes streamed to {}", temp_tar.display()),
                None => {
                    eprintln!("fetch {url} returned void: the tarball stays unread");
                    std::process::exit(1);
                }
            }
            let fits_tmp =
                std::env::temp_dir().join(format!("cmb_spt_url_{}.fits", std::process::id()));
            let downloaded =
                match stream_fits_member_to_file(temp_tar_str, member.as_deref(), &fits_tmp) {
                    Some(name) => {
                        eprintln!("tar member '{name}' streamed to {}", fits_tmp.display());
                        std::fs::read(&fits_tmp)
                    }
                    None => {
                        eprintln!("no .fits member streamed from {url}: the map stays unwritten");
                        let _ = std::fs::remove_file(&temp_tar);
                        std::process::exit(1);
                    }
                };
            let _ = std::fs::remove_file(&fits_tmp);
            let _ = std::fs::remove_file(&temp_tar);
            match downloaded {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("read {} returned void: {e}", fits_tmp.display());
                    std::process::exit(1);
                }
            }
        }
        (None, Some(path)) => {
            let tmp = std::env::temp_dir().join(format!("cmb_spt_d1_{}.fits", std::process::id()));
            match stream_fits_member_to_file(&path, member.as_deref(), &tmp) {
                Some(name) => {
                    eprintln!("tar member '{name}' streamed to {}", tmp.display());
                    match std::fs::read(&tmp) {
                        Ok(b) => {
                            let _ = std::fs::remove_file(&tmp);
                            b
                        }
                        Err(e) => {
                            eprintln!("read {} returned void: {e}", tmp.display());
                            std::process::exit(1);
                        }
                    }
                }
                None => {
                    eprintln!("no .fits member streamed from {path}: the map stays unwritten");
                    std::process::exit(1);
                }
            }
        }
        (None, None) => {
            let Some(input) = arg_value(&args, "--input") else {
                eprintln!(
                    "usage: cmb_planck_compiler (--input <fits> | --url <tar.bz2> | --tar <local.tar.bz2> [--member <name>]) [--nside 64] [--out path] [--ci-mode]"
                );
                std::process::exit(1);
            };
            match std::fs::read(&input) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("read {input} returned void: {e}");
                    std::process::exit(1);
                }
            }
        }
    };
    let Some(map) = read_map(&bytes) else {
        std::process::exit(1);
    };
    let coarse = match &map {
        Map::Full(table, col) => {
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
            degrade(&bytes, table, table.width, nside_out)
        }
        Map::Partial(table, col) => {
            if nside_out <= 0 || nside_out > table.nside || table.nside % nside_out != 0 {
                eprintln!(
                    "--nside {} is not a power-of-two divisor of NSIDE {} — the map stays unwritten",
                    nside_out, table.nside
                );
                std::process::exit(1);
            }
            eprintln!(
                "NSIDE {} ({} EXPLICIT rows), column {}, ORDERING RING, degrade to NSIDE {}",
                table.nside, table.fits.n_rows, col, nside_out
            );
            degrade_partial(&bytes, table, nside_out)
        }
    };
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
    if rows.is_empty() {
        eprintln!("no rows — the map stays unwritten (0 honored)");
        std::process::exit(1);
    }
    eprintln!(
        "{} rows, {} empty cells skipped, T mean {:.6} K, min {:.6} K, max {:.6} K",
        rows.len(),
        skipped,
        tsum / rows.len() as f64,
        tmin,
        tmax
    );
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
        let host = if is_spt {
            "lambda.gsfc.nasa.gov"
        } else {
            "irsa.ipac.caltech.edu"
        };
        let _ = omegaflow::cdn::upload_release(host, &out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW_BYTES: usize = 40;

    fn card(kw: &str, val: &str) -> String {
        format!("{:<8}= {:<70}", kw, val)
    }

    fn synth_map(nside: i64, values: &[f64], ordering: &str, unit: Option<&str>) -> Vec<u8> {
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
        let mut table = format!(
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
        if let Some(u) = unit {
            table.push_str(&card("TUNIT1", &format!("'{}'", u)));
        }
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

    const PARTIAL_ROW_BYTES: usize = 12;

    fn synth_partial(nside: i64, rows: &[(i64, f64)], unit: Option<&str>) -> Vec<u8> {
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
        let mut table = String::new();
        table.push_str(&card("XTENSION", "'BINTABLE'"));
        table.push_str(&card("BITPIX", "8"));
        table.push_str(&card("NAXIS", "2"));
        table.push_str(&card("NAXIS1", &PARTIAL_ROW_BYTES.to_string()));
        table.push_str(&card("NAXIS2", &rows.len().to_string()));
        table.push_str(&card("TFIELDS", "2"));
        table.push_str(&card("PIXTYPE", "'HEALPIX'"));
        table.push_str(&card("ORDERING", "'RING'"));
        table.push_str(&card("NSIDE", &nside.to_string()));
        table.push_str(&card("INDXSCHM", "'EXPLICIT'"));
        table.push_str(&card("TTYPE1", "'PIXEL'"));
        table.push_str(&card("TFORM1", "'1J'"));
        table.push_str(&card("TTYPE2", "'TEMPERATURE'"));
        table.push_str(&card("TFORM2", "'1D'"));
        if let Some(u) = unit {
            table.push_str(&card("TUNIT2", &format!("'{}'", u)));
        }
        let mut thdr = table.into_bytes();
        thdr.extend(card("END", "").into_bytes());
        while thdr.len() % 2880 != 0 {
            thdr.push(b' ');
        }
        buf.extend_from_slice(&thdr);
        for (p, v) in rows {
            let mut row = [0u8; PARTIAL_ROW_BYTES];
            row[0..4].copy_from_slice(&(*p as i32).to_be_bytes());
            row[4..12].copy_from_slice(&v.to_be_bytes());
            buf.extend_from_slice(&row);
        }
        buf
    }

    #[test]
    fn partial_ring_downsample_selects_coarse_nest_pixels() {
        let rows = [
            (0i64, 10.0f64),
            (4, 20.0),
            (8, 30.0),
            (12, 40.0),
            (47, 50.0),
        ];
        let bytes = synth_partial(2, &rows, Some("uK"));
        let (table, _) = read_table_partial(&bytes).unwrap();
        assert_eq!(table.nside, 2);
        let coarse = degrade_partial(&bytes, &table, 1);
        assert_eq!(coarse.len(), 12);
        let c0 = coarse[0].unwrap();
        assert!((c0.0 - 15e-6).abs() < 1e-18, "coarse 0 = {}", c0.0);
        assert_eq!(c0.1, 2);
        let c2 = coarse[2].unwrap();
        assert!((c2.0 - 30e-6).abs() < 1e-18, "coarse 2 = {}", c2.0);
        assert_eq!(c2.1, 1);
        let c4 = coarse[4].unwrap();
        assert!((c4.0 - 40e-6).abs() < 1e-18, "coarse 4 = {}", c4.0);
        assert_eq!(c4.1, 1);
        let c11 = coarse[11].unwrap();
        assert!((c11.0 - 50e-6).abs() < 1e-18, "coarse 11 = {}", c11.0);
        assert_eq!(c11.1, 1);
        assert!(coarse[1].is_none());
        assert!(coarse[3].is_none());
    }

    #[test]
    fn read_map_selects_partial_and_full() {
        let partial = synth_partial(2, &[(0i64, 1.0f64)], Some("uK"));
        assert!(matches!(read_map(&partial), Some(Map::Partial(_, _))));
        let nested = synth_map(2, &vec![1.0; 48], "NESTED", Some("K"));
        assert!(matches!(read_map(&nested), Some(Map::Full(_, _))));
    }

    #[test]
    fn degrade_uniform_map() {
        let nside_in = 4;
        let npix = (12 * nside_in * nside_in) as usize;
        let bytes = synth_map(nside_in, &vec![1.0; npix], "NESTED", Some("K"));
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
        let bytes = synth_map(nside_in, &vals, "NESTED", Some("K"));
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
        let bytes = synth_map(nside_in, &vals, "NESTED", Some("K"));
        let table = read_table(&bytes).unwrap().0;
        let coarse = degrade(&bytes, &table, ROW_BYTES, 1);
        assert!((coarse[0].unwrap().0 + 1.0).abs() < 1e-12);
        assert_eq!(coarse[0].unwrap().1, 16);
        assert!((coarse[1].unwrap().0 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn read_table_refuses_ring() {
        let nside = 1;
        let bytes = synth_map(nside, &vec![1.0; 12], "RING", Some("K"));
        assert!(read_table(&bytes).is_none());
    }

    #[test]
    fn unit_factor_reads_the_measured_token() {
        assert_eq!(unit_factor("mK, thermodynamic"), Some(1e-3));
        assert_eq!(unit_factor("uK"), Some(1e-6));
        assert_eq!(unit_factor("K"), Some(1.0));
        assert_eq!(unit_factor("Jy/sr"), None);
    }

    #[test]
    fn read_table_applies_mk_to_k() {
        let nside = 4;
        let npix = (12 * nside * nside) as usize;
        let bytes = synth_map(
            nside,
            &vec![1000.0; npix],
            "NESTED",
            Some("mK, thermodynamic"),
        );
        let (table, _) = read_table(&bytes).unwrap();
        assert!((table.unit - 1e-3).abs() < 1e-30);
        let coarse = degrade(&bytes, &table, ROW_BYTES, 1);
        for c in &coarse {
            let (t, _) = c.unwrap();
            assert!((t - 1.0).abs() < 1e-9, "mK 1000 -> K {}", t);
        }
    }

    #[test]
    fn read_table_leaves_a_missing_unit_pending() {
        let nside = 4;
        let npix = (12 * nside * nside) as usize;
        let bytes = synth_map(nside, &vec![1000.0; npix], "NESTED", None);
        assert!(read_table(&bytes).is_none());
    }

    fn tar_header(name: &str, size: usize) -> [u8; 512] {
        let mut h = [0u8; 512];
        h[..name.len()].copy_from_slice(name.as_bytes());
        let octal = format!("{size:011o}");
        h[124..135].copy_from_slice(octal.as_bytes());
        h[156] = b'0';
        h[257..262].copy_from_slice(b"ustar");
        h[148..156].fill(b' ');
        let sum: u32 = h.iter().map(|&b| b as u32).sum();
        let checksum = format!("{sum:06o}");
        h[148..154].copy_from_slice(checksum.as_bytes());
        h[154] = 0;
        h[155] = b' ';
        h
    }

    fn pad_block(out: &mut Vec<u8>) {
        out.resize(out.len().div_ceil(512) * 512, 0);
    }

    #[test]
    fn select_fits_member_reads_the_named_member() {
        let fits = synth_map(1, &vec![1.0; 12], "NESTED", Some("K"));
        let mut tar = Vec::new();
        tar.extend_from_slice(&tar_header("readme.txt", 2));
        tar.extend_from_slice(b"hi");
        pad_block(&mut tar);
        tar.extend_from_slice(&tar_header("map.fits", fits.len()));
        tar.extend_from_slice(&fits);
        pad_block(&mut tar);
        tar.extend_from_slice(&[0u8; 1024]);

        let members = omegaflow::archivar::inflate::tar_members(&tar).unwrap();
        let (member, data) = select_fits_member(&members, &tar, None).unwrap();
        assert_eq!(member.name, "map.fits");
        assert_eq!(data, &fits[..]);
        let (named, _) = select_fits_member(&members, &tar, Some("readme.txt")).unwrap();
        assert_eq!(named.name, "readme.txt");
        assert!(select_fits_member(&members, &tar, Some("absent.fits")).is_none());
    }
}
