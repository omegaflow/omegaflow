// COM_CompMap_Dust-DL07-AvMaps_2048_R2.00.fits (IRSA Planck release_2):
// BINTABLE, NSIDE 2048, NESTED, TFIELDS 4 — columns
// AV_DL/AV_DL_UNC/AV_RQ/AV_RQ_UNC, all TFORM 'E' (f32), header BAD_DATA
// -1.63750E+30, R_V 3.1. Column three is read by default (--column): the
// native measured AV_RQ (mag) flows unmodified — no E(B-V) conversion, and
// AV_DL stays unread. Geometry: a single 2D foreground shell carried as
// --screen-pc distance (dist, never the redshift z), degrade to NSIDE 512
// by default (pixel 0.11 deg ≤ σ/2 of the GD-1 12 arcmin stream width).
use omegaflow::fits::FitsHeader;
use omegaflow::healpix::{galactic_to_icrs, pix2ang_nest};
use omegaflow::json::{parse_json, JsonVal};

const NSIDE_DEFAULT: i64 = 512;
const COLUMN_DEFAULT: usize = 3;

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
    width: usize,
    col_width: usize,
    col_off: usize,
    key: String,
    unit: String,
    bad: Option<f64>,
}

fn read_table(bytes: &[u8], column: usize) -> Option<(Table, String)> {
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
    let tfields = h.int("TFIELDS")? as usize;
    if column == 0 || column > tfields {
        eprintln!(
            "column {} reads nothing among TFIELDS {} — the map stays unread",
            column, tfields
        );
        return None;
    }
    let width = h.int("NAXIS1")? as usize;
    let tform = h
        .str_unescaped(&format!("TFORM{column}"))
        .unwrap_or_default();
    let col_width = if tform.contains('D') {
        8
    } else if tform.contains('E') {
        4
    } else {
        eprintln!(
            "TFORM{column} '{}' is neither 'E' nor 'D' — the column stays unread",
            tform
        );
        return None;
    };
    let col_off = (column - 1) * col_width;
    if col_off + col_width > width {
        eprintln!(
            "column {column} (offset {col_off}, width {col_width}) overruns NAXIS1 {width} — the column stays unread"
        );
        return None;
    }
    let ttype = h
        .str_unescaped(&format!("TTYPE{column}"))
        .unwrap_or_default();
    let ttype = ttype.trim();
    let tunit = h
        .str_unescaped(&format!("TUNIT{column}"))
        .unwrap_or_default();
    let tunit = tunit.trim();
    let bad = h.f64("BAD_DATA");
    if data_start + npix * width > bytes.len() {
        eprintln!("table exceeds the fetched bytes — the map stays unwritten");
        return None;
    }
    Some((
        Table {
            nside,
            npix,
            data_start,
            width,
            col_width,
            col_off,
            key: ttype.to_ascii_lowercase(),
            unit: tunit.to_string(),
            bad,
        },
        format!("{} {} {}", ttype, tunit, tform),
    ))
}

fn col_value(bytes: &[u8], table: &Table, row: usize) -> Option<f64> {
    let off = table.data_start + row * table.width + table.col_off;
    if table.col_width == 8 {
        let raw = bytes.get(off..off + 8)?;
        Some(f64::from_be_bytes(raw.try_into().ok()?))
    } else {
        let raw = bytes.get(off..off + 4)?;
        Some(f32::from_be_bytes(raw.try_into().ok()?) as f64)
    }
}

fn is_bad(v: f64, table: &Table) -> bool {
    let Some(b) = table.bad else {
        return false;
    };
    if table.col_width == 8 {
        v == b
    } else {
        (v as f32) == (b as f32)
    }
}

fn degrade(bytes: &[u8], table: &Table, nside_out: i64) -> Vec<Option<(f64, u64)>> {
    let ratio = table.nside / nside_out;
    let npix_out = (12 * nside_out * nside_out) as usize;
    let mut sum = vec![0.0f64; npix_out];
    let mut count = vec![0u64; npix_out];
    for r in 0..table.npix {
        let Some(v) = col_value(bytes, table, r) else {
            continue;
        };
        if !v.is_finite() {
            continue;
        }
        if is_bad(v, table) {
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

fn write_json(rows: &[(f64, f64, f64)], key: &str, dist_pc: f64, path: &str) -> bool {
    let mut out = String::with_capacity(rows.len() * 64 + 2);
    out.push('[');
    for (i, (ra, dec, a)) in rows.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"ra\":{},\"dec\":{},\"dist\":{},\"{}\":{}}}",
            ra, dec, dist_pc, key, a
        ));
    }
    out.push(']');
    if std::fs::write(path, out.as_bytes()).is_err() {
        eprintln!("write {path} returned void");
        return false;
    }
    true
}

fn screen_pc_of(args: &[String]) -> Result<f64, String> {
    let Some(v) = arg_value(args, "--screen-pc") else {
        return Err(
            "no --screen-pc — the screen distance is never silent, no fabricated shell — refused"
                .into(),
        );
    };
    let pc: f64 = v
        .parse()
        .map_err(|_| format!("--screen-pc '{v}' carries no number — refused"))?;
    if !pc.is_finite() || pc <= 0.0 {
        return Err(format!(
            "--screen-pc '{v}' carries no plausible distance — refused"
        ));
    }
    Ok(pc)
}

fn nside_of(args: &[String]) -> Result<i64, String> {
    match arg_value(args, "--nside") {
        None => Ok(NSIDE_DEFAULT),
        Some(v) => v
            .parse()
            .map_err(|_| format!("--nside '{v}' carries no number — refused")),
    }
}

fn column_of(args: &[String]) -> Result<usize, String> {
    match arg_value(args, "--column") {
        None => Ok(COLUMN_DEFAULT),
        Some(v) => {
            let n: usize = v
                .parse()
                .map_err(|_| format!("--column '{v}' carries no number — refused"))?;
            if n == 0 {
                return Err("--column 0 reads nothing — refused".into());
            }
            Ok(n)
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let Some(input) = arg_value(args, "--input") else {
        return Err(
            "usage: dust_map_compiler --input <fits> [--column n] [--nside 512] \
             --screen-pc <pc> [--out path] [--mask-run --binding <file>] [--ci-mode] — refused"
                .into(),
        );
    };
    let nside_out = nside_of(args)?;
    let column = column_of(args)?;
    let screen_pc = screen_pc_of(args)?;
    let ci_mode = has_flag(args, "--ci-mode");
    if has_flag(args, "--mask-run") {
        let binding = arg_value(args, "--binding").ok_or(
            "--mask-run without --binding <file> — the science run refuses without a committed binding file — refused"
        )?;
        if !std::path::Path::new(&binding).exists() {
            return Err(format!(
                "--binding {binding} does not exist on disk — the science run refuses without its committed binding file"
            ));
        }
    }

    let bytes = std::fs::read(&input).map_err(|e| format!("read {input} returned void: {e}"))?;
    let (table, col) = read_table(&bytes, column).ok_or("the table stayed unread — refused")?;
    if nside_out <= 0 || nside_out > table.nside || table.nside % nside_out != 0 {
        return Err(format!(
            "--nside {nside_out} is not a power-of-two divisor of NSIDE {} — the map stays unwritten",
            table.nside
        ));
    }
    let out = arg_value(args, "--out")
        .unwrap_or_else(|| format!("planck_dust_{}_n{}.json", table.key, nside_out));
    eprintln!(
        "NSIDE {} ({} pixels), column {col}, ORDERING NESTED, screen {screen_pc} pc, degrade to NSIDE {}",
        table.nside, table.npix, nside_out
    );

    let coarse = degrade(&bytes, &table, nside_out);
    let mut rows: Vec<(f64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    let mut asum = 0.0f64;
    let mut amin = f64::INFINITY;
    let mut amax = f64::NEG_INFINITY;
    for (c, cell) in coarse.iter().enumerate() {
        let Some((a, _)) = cell else {
            skipped += 1;
            continue;
        };
        let Some((theta, phi)) = pix2ang_nest(nside_out, c as i64) else {
            skipped += 1;
            continue;
        };
        let (ra, dec) = galactic_to_icrs(theta, phi);
        rows.push((ra, dec, *a));
        asum += *a;
        if *a < amin {
            amin = *a;
        }
        if *a > amax {
            amax = *a;
        }
    }
    if rows.is_empty() {
        return Err("no rows — the map stays unwritten (0 honored)".into());
    }
    let n = rows.len() as f64;
    eprintln!(
        "{} rows, {} empty cells skipped, {} mean {:.6} {}, min {:.6} {}, max {:.6} {}",
        rows.len(),
        skipped,
        table.key,
        asum / n,
        table.unit,
        amin,
        table.unit,
        amax,
        table.unit
    );
    if !write_json(&rows, &table.key, screen_pc, &out) {
        return Err(format!("{out}: the map stayed unwritten"));
    }
    match std::fs::read_to_string(&out)
        .ok()
        .and_then(|s| parse_json(&s))
    {
        Some(JsonVal::Arr(arr)) => {
            eprintln!("{out}: {} rows roundtrip-parse", arr.len());
        }
        _ => return Err(format!("{out}: roundtrip parse void")),
    }
    if ci_mode {
        let _ = omegaflow::cdn::upload_release("irsa.ipac.caltech.edu", &out);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(msg) = run(&args) {
        eprintln!("dust_map_compiler: {msg}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW_BYTES: usize = 16;

    fn card(kw: &str, val: &str) -> String {
        format!("{:<8}= {:<70}", kw, val)
    }

    fn synth_map_col(
        nside: i64,
        values: &[f64],
        ordering: &str,
        bad: Option<f64>,
        col: usize,
    ) -> Vec<u8> {
        let npix = 12 * nside * nside;
        assert_eq!(values.len(), npix as usize);
        assert!(col >= 1 && col <= 4);
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
        let bad_card = match bad {
            Some(b) => card("BAD_DATA", &format!("{:E}", b)),
            None => String::new(),
        };
        let table = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            card("XTENSION", "'BINTABLE'"),
            card("BITPIX", "8"),
            card("NAXIS", "2"),
            card("NAXIS1", "16"),
            card("NAXIS2", &npix.to_string()),
            card("TFIELDS", "4"),
            card("NSIDE", &nside.to_string()),
            card("ORDERING", &format!("'{}'", ordering)),
            card("TTYPE1", "'AV_DL'"),
            card("TFORM1", "'1E'"),
            card("TTYPE2", "'AV_DL_UNC'"),
            card("TFORM2", "'1E'"),
            card("TTYPE3", "'AV_RQ'"),
            card("TFORM3", "'1E'"),
            card("TUNIT3", "'mag'"),
            bad_card
        );
        let mut thdr = table.into_bytes();
        thdr.extend(card("END", "").into_bytes());
        while thdr.len() % 2880 != 0 {
            thdr.push(b' ');
        }
        buf.extend_from_slice(&thdr);
        let off = (col - 1) * 4;
        for v in values {
            let mut row = [0u8; ROW_BYTES];
            row[off..off + 4].copy_from_slice(&(*v as f32).to_be_bytes());
            buf.extend_from_slice(&row);
        }
        buf
    }

    fn synth_map(nside: i64, values: &[f64], ordering: &str) -> Vec<u8> {
        synth_map_col(nside, values, ordering, None, 3)
    }

    fn synth_map_bad(nside: i64, values: &[f64], ordering: &str, bad: Option<f64>) -> Vec<u8> {
        synth_map_col(nside, values, ordering, bad, 3)
    }

    #[test]
    fn degrade_uniform_map() {
        let nside_in = 4;
        let npix = (12 * nside_in * nside_in) as usize;
        let bytes = synth_map(nside_in, &vec![1.0; npix], "NESTED");
        let table = read_table(&bytes, 3).unwrap().0;
        assert_eq!(table.key, "av_rq");
        assert_eq!(table.unit, "mag");
        assert_eq!(table.col_off, 8);
        let coarse = degrade(&bytes, &table, 1);
        assert_eq!(coarse.len(), 12);
        for c in &coarse {
            let (a, n) = c.unwrap();
            assert!((a - 1.0).abs() < 1e-12);
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
        let table = read_table(&bytes, 3).unwrap().0;
        let coarse = degrade(&bytes, &table, 1);
        assert_eq!(coarse[0].unwrap().1, 15);
        assert!((coarse[0].unwrap().0 - 1.0).abs() < 1e-12);
        for c in coarse.iter().skip(1) {
            assert_eq!(c.unwrap().1, 16);
        }
    }

    #[test]
    fn degrade_skips_bad_data_sentinel() {
        let nside_in = 4;
        let npix = (12 * nside_in * nside_in) as usize;
        let mut vals = vec![1.0; npix];
        vals[0] = -1.6375e30;
        let bytes = synth_map_bad(nside_in, &vals, "NESTED", Some(-1.6375e30));
        let table = read_table(&bytes, 3).unwrap().0;
        assert!(table.bad.is_some());
        let coarse = degrade(&bytes, &table, 1);
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
        let table = read_table(&bytes, 3).unwrap().0;
        let coarse = degrade(&bytes, &table, 1);
        assert!((coarse[0].unwrap().0 + 1.0).abs() < 1e-12);
        assert_eq!(coarse[0].unwrap().1, 16);
        assert!((coarse[1].unwrap().0 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn read_table_refuses_ring() {
        let nside = 1;
        let bytes = synth_map(nside, &vec![1.0; 12], "RING");
        assert!(read_table(&bytes, 3).is_none());
    }

    #[test]
    fn select_column_one_reads_av_dl() {
        let nside = 1;
        let bytes = synth_map_col(nside, &vec![1.0; 12], "NESTED", None, 1);
        let table = read_table(&bytes, 1).unwrap().0;
        assert_eq!(table.key, "av_dl");
        assert_eq!(table.col_off, 0);
        let coarse = degrade(&bytes, &table, 1);
        for c in &coarse {
            let (a, n) = c.unwrap();
            assert!((a - 1.0).abs() < 1e-12);
            assert_eq!(n, 1);
        }
    }

    #[test]
    fn read_table_refuses_column_over_tfields() {
        let nside = 1;
        let bytes = synth_map(nside, &vec![1.0; 12], "NESTED");
        assert!(read_table(&bytes, 5).is_none());
    }

    #[test]
    fn missing_screen_pc_refuses() {
        let args = vec!["--input".to_string(), "unused.fits".to_string()];
        let err = screen_pc_of(&args).unwrap_err();
        assert!(err.contains("refused"));
    }
}
