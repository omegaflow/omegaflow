use omegaflow::fits::FitsHeader;
use omegaflow::healpix::{galactic_to_icrs, pix2ang_nest};
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

fn tarball_member_from_bytes(url: &str, bytes: &[u8], want: Option<&str>) -> Option<Vec<u8>> {
    let tar = match omegaflow::archivar::bzip2::decompress(bytes) {
        Some(t) => t,
        None => {
            eprintln!("bzip2 decompress of {url} returned void: the tarball stays unread");
            return None;
        }
    };
    let members = match omegaflow::archivar::inflate::tar_members(&tar) {
        Some(m) => m,
        None => {
            eprintln!("tar_members of {url} returned void: the tarball stays unread");
            return None;
        }
    };
    let (member, data) = match select_fits_member(&members, &tar, want) {
        Some(v) => v,
        None => {
            eprintln!(
                "no .fits member among {} tar members from {url}: the map stays unwritten",
                members.len()
            );
            return None;
        }
    };
    eprintln!("tar member '{}': {} bytes -> FITS", member.name, data.len());
    Some(data.to_vec())
}

fn tarball_fits_member(url: &str, want: Option<&str>) -> Option<Vec<u8>> {
    let bytes = match omegaflow::archivar::fetch::fetch_raw_bytes_with(
        url,
        omegaflow::archivar::fetch::RetryPolicy::Transient,
        SPT_TRANSFER_BOUND_S,
    ) {
        Some(b) => b,
        None => {
            eprintln!("fetch {url} returned void: the tarball stays unread");
            return None;
        }
    };
    tarball_member_from_bytes(url, &bytes, want)
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
        (Some(url), _) => match tarball_fits_member(&url, member.as_deref()) {
            Some(b) => b,
            None => std::process::exit(1),
        },
        (None, Some(path)) => {
            let raw = match std::fs::read(&path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("read {path} returned void: {e}");
                    std::process::exit(1);
                }
            };
            match tarball_member_from_bytes(&path, &raw, member.as_deref()) {
                Some(b) => b,
                None => std::process::exit(1),
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
