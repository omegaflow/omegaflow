use omegaflow::archivar::fits::FitsHeader;
use omegaflow::archivar::sha256::sha256_hex;
use omegaflow::cdn::upload_release;
use omegaflow::inflate::gunzip_stream;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};

const ASSET_MAGIC: [u8; 4] = [0xCF, 0x86, 0x05, 0x00];
const ASSET_HEADER_BYTES: usize = 16;
const RECORD_BYTES: usize = 56;
const MAG_BAND_G: usize = 0;
const MAG_BAND_R: usize = 1;
const MAG_BAND_I: usize = 2;
const MAG_BAND_Z: usize = 3;
const PCTL_MEDIAN: usize = 2;

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

#[derive(Clone)]
struct Column {
    name: String,
    code: char,
    repeat: usize,
    width: usize,
    tbcol: usize,
    tscal: Option<f64>,
    tzero: Option<f64>,
}

struct TableDesc {
    n_rows: usize,
    row_bytes: usize,
    columns: Vec<Column>,
    data_start: usize,
    complete: bool,
    available_rows: usize,
}

fn code_width(code: char) -> Option<usize> {
    match code {
        'E' => Some(4),
        'D' => Some(8),
        'J' => Some(4),
        'I' => Some(2),
        'K' => Some(8),
        'B' => Some(1),
        _ => None,
    }
}

fn parse_tform(tform: &str) -> Option<(char, usize)> {
    let inner = match tform.strip_suffix(')') {
        Some(i) => i.rsplit_once('(')?.0,
        None => tform,
    };
    let bytes = inner.as_bytes();
    let last = *bytes.last()?;
    let repeat: usize = if bytes.len() == 1 {
        1
    } else {
        std::str::from_utf8(&bytes[..bytes.len() - 1])
            .ok()?
            .parse()
            .ok()?
    };
    if last == b'P' || last == b'Q' {
        return None;
    }
    Some((last as char, repeat))
}

fn table_desc(buf: &[u8], hdu_start: usize) -> Option<TableDesc> {
    let (header, data_start) = FitsHeader::parse(buf, hdu_start)?;
    if header.value("XTENSION") != Some("'BINTABLE'") {
        return None;
    }
    let row_bytes = header.int("NAXIS1")? as usize;
    let n_rows = header.int("NAXIS2")? as usize;
    let heap_bytes = header.int("PCOUNT")? as usize;
    let tfields = header.int("TFIELDS")? as usize;
    let mut columns = Vec::new();
    let mut next_tbcol = 1usize;
    for i in 1..=tfields {
        let name = match header.str_unescaped(&format!("TTYPE{i}")) {
            Some(v) => v.trim().to_string(),
            None => return None,
        };
        let tform = match header.value(&format!("TFORM{i}")) {
            Some(v) => v.trim_matches('\'').trim().to_string(),
            None => return None,
        };
        let (code, repeat) = parse_tform(&tform)?;
        let width = code_width(code)? * repeat;
        let tbcol = match header.int(&format!("TBCOL{i}")) {
            Some(t) if t > 0 => t as usize,
            _ => next_tbcol,
        };
        if tbcol == 0 || tbcol + width > row_bytes + 1 {
            return None;
        }
        next_tbcol = tbcol + width;
        let tscal = header.f64(&format!("TSCAL{i}"));
        let tzero = header.f64(&format!("TZERO{i}"));
        columns.push(Column {
            name,
            code,
            repeat,
            width,
            tbcol,
            tscal,
            tzero,
        });
    }
    if columns.is_empty() || row_bytes == 0 {
        return None;
    }
    let table_end = data_start + row_bytes * n_rows + heap_bytes;
    let complete = table_end <= buf.len();
    let available_rows = if complete {
        n_rows
    } else {
        buf.len().saturating_sub(data_start) / row_bytes
    };
    Some(TableDesc {
        n_rows,
        row_bytes,
        columns,
        data_start,
        complete,
        available_rows,
    })
}

fn find_hdu(buf: &[u8]) -> Option<(usize, TableDesc)> {
    let (_, off) = FitsHeader::parse(buf, 0)?;
    let mut hdu = off;
    for _ in 0..8 {
        if let Some(t) = table_desc(buf, hdu) {
            return Some((hdu, t));
        }
        match FitsHeader::parse(buf, hdu) {
            Some((_, next)) if next > hdu => hdu = next,
            _ => return None,
        }
    }
    None
}

fn column_named<'a>(t: &'a TableDesc, name: &str) -> Option<&'a Column> {
    t.columns.iter().find(|c| c.name.eq_ignore_ascii_case(name))
}

fn cell_elem(row: &[u8], col: &Column, k: usize) -> Option<f64> {
    if k >= col.repeat {
        return None;
    }
    let elem = code_width(col.code)?;
    let off = col.tbcol - 1 + k * elem;
    let raw = row.get(off..off + elem)?;
    let v = match col.code {
        'D' => f64::from_be_bytes(raw.try_into().ok()?),
        'E' => f32::from_be_bytes(raw[..4].try_into().ok()?) as f64,
        'J' => i32::from_be_bytes(raw[..4].try_into().ok()?) as f64,
        'I' => i16::from_be_bytes(raw[..2].try_into().ok()?) as f64,
        'K' => i64::from_be_bytes(raw.try_into().ok()?) as f64,
        'B' => raw[0] as f64,
        _ => return None,
    };
    let scaled = match col.tscal {
        Some(s) => v * s,
        None => v,
    };
    Some(match col.tzero {
        Some(z) => scaled + z,
        None => scaled,
    })
}

fn cell_array(row: &[u8], col: &Column) -> Option<Vec<f64>> {
    (0..col.repeat).map(|k| cell_elem(row, col, k)).collect()
}

fn plausible(v: f64) -> Option<f64> {
    if v.is_finite() && v > 0.0 {
        Some(v)
    } else {
        None
    }
}

fn plausible_ra(v: f64) -> Option<f64> {
    if v.is_finite() && (0.0..360.0).contains(&v) {
        Some(v)
    } else {
        None
    }
}

fn plausible_dec(v: f64) -> Option<f64> {
    if v.is_finite() && (-90.0..=90.0).contains(&v) {
        Some(v)
    } else {
        None
    }
}

fn encode_record(
    ra: f64,
    dec: f64,
    plx: Option<f64>,
    mag_g: f64,
    mag_r: Option<f64>,
    mag_i: Option<f64>,
    mag_z: Option<f64>,
    dist50: Option<f64>,
    ext50: Option<f64>,
    rv50: Option<f64>,
    logt50: Option<f64>,
    mini50: Option<f64>,
    out: &mut Vec<u8>,
) {
    out.extend_from_slice(&ra.to_le_bytes());
    out.extend_from_slice(&dec.to_le_bytes());
    for v in [
        plx, mag_r, mag_i, mag_z, dist50, ext50, rv50, logt50, mini50,
    ] {
        match v {
            Some(x) => out.extend_from_slice(&(x as f32).to_le_bytes()),
            None => out.extend_from_slice(&0.0f32.to_le_bytes()),
        }
    }
    out.extend_from_slice(&(mag_g as f32).to_le_bytes());
}

fn decode_gz(path: &str) -> Result<(Vec<u8>, bool), String> {
    let src = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut collected: Vec<u8> = Vec::new();
    let result = gunzip_stream(src, |chunk| {
        collected.extend_from_slice(chunk);
    });
    match result {
        Ok(total) => {
            eprintln!("{path}: gunzip {total} bytes");
            Ok((collected, true))
        }
        Err(e) => {
            if collected.len() >= 2880 {
                eprintln!(
                    "{path}: gunzip ended early ({e}) — {} bytes decoded before the end, parsing the decoded prefix",
                    collected.len()
                );
                Ok((collected, false))
            } else {
                Err(format!("{path}: {e}"))
            }
        }
    }
}

fn decode_gz_to_file(path: &str, out_path: &str) -> Result<(u64, bool), String> {
    let src = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let dst = std::fs::File::create(out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut w = BufWriter::with_capacity(1 << 20, dst);
    let mut io_err: Option<String> = None;
    let result = gunzip_stream(src, |chunk| {
        if io_err.is_none()
            && let Err(e) = w.write_all(chunk)
        {
            io_err = Some(format!("write {out_path} returned void: {e}"));
        }
    });
    if let Some(e) = io_err {
        return Err(e);
    }
    let _ = w.flush();
    let _ = w.into_inner();
    match result {
        Ok(total) => Ok((total, true)),
        Err(e) => {
            let written = std::fs::metadata(out_path).ok().map(|m| m.len());
            match written {
                Some(w) if w >= 2880 => {
                    eprintln!(
                        "{path}: gunzip ended early ({e}) — {w} bytes decoded before the end"
                    );
                    Ok((w, false))
                }
                _ => Err(format!("{path}: {e}")),
            }
        }
    }
}

fn inspect(args: &[String]) -> Result<(), String> {
    let Some(input) = arg_value(args, "--inspect") else {
        return Err("--inspect <file.fits[.gz]>: the path is never silent — refused".into());
    };
    let limit = match arg_value(args, "--limit").and_then(|v| v.parse::<usize>().ok()) {
        Some(v) => v,
        None => 3,
    };
    let (buf, complete) = decode_gz(&input)?;
    let Some((hdu, t)) = find_hdu(&buf) else {
        return Err(format!(
            "{input}: no BINTABLE HDU parsed (decoded {} bytes, complete {complete})",
            buf.len()
        ));
    };
    eprintln!(
        "HDU {hdu}: BINTABLE {} rows claimed, row {} bytes, {} columns, data at {}, decoded complete {}, {} rows available",
        t.n_rows,
        t.row_bytes,
        t.columns.len(),
        t.data_start,
        complete,
        t.available_rows
    );
    for c in &t.columns {
        eprintln!(
            "  col {} code {} repeat {} width {} tbcol {} scal {:?} zero {:?}",
            c.name, c.code, c.repeat, c.width, c.tbcol, c.tscal, c.tzero
        );
    }
    let ra = column_named(&t, "ra");
    let dec = column_named(&t, "dec");
    let plx = column_named(&t, "parallax");
    let mag = column_named(&t, "mag");
    let dist = column_named(&t, "dist");
    let extinction = column_named(&t, "extinction");
    let rv = column_named(&t, "rv");
    let logt = column_named(&t, "logt");
    let mini = column_named(&t, "mini");
    eprintln!(
        "resolved: ra {:?} dec {:?} plx {:?} mag {:?} dist {:?} extinction {:?} rv {:?} logt {:?} mini {:?}",
        ra.map(|c| c.name.as_str()),
        dec.map(|c| c.name.as_str()),
        plx.map(|c| c.name.as_str()),
        mag.map(|c| c.name.as_str()),
        dist.map(|c| c.name.as_str()),
        extinction.map(|c| c.name.as_str()),
        rv.map(|c| c.name.as_str()),
        logt.map(|c| c.name.as_str()),
        mini.map(|c| c.name.as_str())
    );
    let mut band_finite = [0u64; 13];
    let mut band_sum = [0.0f64; 13];
    let mut printed = 0usize;
    let mut gated = 0usize;
    let mut skipped = 0usize;
    for row_idx in 0..t.available_rows {
        let row_start = t.data_start + row_idx * t.row_bytes;
        let Some(row) = buf.get(row_start..row_start + t.row_bytes) else {
            break;
        };
        let cell = |c: &Column| cell_elem(row, c, 0);
        let (Some(ra_v), Some(dec_v)) = (
            ra.and_then(cell).and_then(plausible_ra),
            dec.and_then(cell).and_then(plausible_dec),
        ) else {
            skipped += 1;
            continue;
        };
        let Some(mags) = mag.and_then(|c| cell_array(row, c)) else {
            skipped += 1;
            continue;
        };
        let Some(mag_g) = mags.get(MAG_BAND_G).copied().and_then(plausible) else {
            skipped += 1;
            continue;
        };
        gated += 1;
        for (k, m) in mags.iter().enumerate().take(13) {
            if m.is_finite() && *m > 0.0 {
                band_finite[k] += 1;
                band_sum[k] += *m;
            }
        }
        if printed < limit {
            eprintln!(
                "  row {row_idx}: ra {ra_v:.6} dec {dec_v:.6} g {mag_g:.4} r {:?} i {:?} z {:?} plx {:?} dist50 {:?} logt50 {:?} mini50 {:?}",
                mags.get(MAG_BAND_R).copied().and_then(plausible),
                mags.get(MAG_BAND_I).copied().and_then(plausible),
                mags.get(MAG_BAND_Z).copied().and_then(plausible),
                plx.and_then(cell).and_then(plausible),
                dist.and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
                    .and_then(plausible),
                logt.and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
                    .and_then(plausible),
                mini.and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
                    .and_then(plausible)
            );
            printed += 1;
        }
    }
    let mut bandline = String::from("band finite-fraction and mean mag (per index):");
    for k in 0..13 {
        let frac = if gated > 0 {
            band_finite[k] as f64 / gated as f64
        } else {
            0.0
        };
        let mean = if band_finite[k] > 0 {
            band_sum[k] / band_finite[k] as f64
        } else {
            0.0
        };
        bandline.push_str(&format!(" [{k}]={frac:.2}/{mean:.1}",));
    }
    eprintln!("{bandline}");
    eprintln!(
        "sample: {gated} gated rows, {skipped} skipped over {} available",
        t.available_rows
    );
    let _ = extinction;
    let _ = rv;
    Ok(())
}

struct Census {
    rows: u64,
    skipped: u64,
    present_r: u64,
    present_i: u64,
    present_z: u64,
    present_plx: u64,
    present_dist: u64,
    present_ext: u64,
    present_rv: u64,
    present_logt: u64,
    present_mini: u64,
    sum_g: f64,
}

fn compile_rows(buf: &[u8], census: &mut Census, out: &mut std::fs::File) -> Result<(), String> {
    let Some((_, t)) = find_hdu(buf) else {
        return Err("no BINTABLE HDU parsed — the asset stays unwritten".into());
    };
    if !t.complete {
        eprintln!(
            "partial table: compiling the {} available rows of {} claimed",
            t.available_rows, t.n_rows
        );
    }
    let ra = column_named(&t, "ra");
    let dec = column_named(&t, "dec");
    let plx = column_named(&t, "parallax");
    let mag = column_named(&t, "mag");
    let dist = column_named(&t, "dist");
    let extinction = column_named(&t, "extinction");
    let rv = column_named(&t, "rv");
    let logt = column_named(&t, "logt");
    let mini = column_named(&t, "mini");
    let (Some(ra), Some(dec), Some(mag)) = (ra, dec, mag) else {
        let names: Vec<&str> = t.columns.iter().map(|c| c.name.as_str()).collect();
        return Err(format!(
            "column arm absent: ra/dec/mag unresolved — the table names: {names:?}"
        ));
    };
    let mut rec = Vec::with_capacity(RECORD_BYTES);
    for row_idx in 0..t.available_rows {
        let row_start = t.data_start + row_idx * t.row_bytes;
        let Some(row) = buf.get(row_start..row_start + t.row_bytes) else {
            break;
        };
        let cell = |c: &Column| cell_elem(row, c, 0);
        let (Some(ra_v), Some(dec_v)) = (
            ra.cell_value(row).and_then(plausible_ra),
            dec.cell_value(row).and_then(plausible_dec),
        ) else {
            census.skipped += 1;
            continue;
        };
        let Some(mags) = cell_array(row, mag) else {
            census.skipped += 1;
            continue;
        };
        let Some(mag_g) = mags.get(MAG_BAND_G).copied().and_then(plausible) else {
            census.skipped += 1;
            continue;
        };
        let mag_r = mags.get(MAG_BAND_R).copied().and_then(plausible);
        let mag_i = mags.get(MAG_BAND_I).copied().and_then(plausible);
        let mag_z = mags.get(MAG_BAND_Z).copied().and_then(plausible);
        let plx_v = plx.and_then(cell).and_then(plausible);
        let dist_v = dist
            .and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
            .and_then(plausible);
        let ext_v = extinction
            .and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
            .and_then(plausible);
        let rv_v = rv
            .and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
            .and_then(plausible);
        let logt_v = logt
            .and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
            .and_then(plausible);
        let mini_v = mini
            .and_then(|c| cell_elem(row, c, PCTL_MEDIAN))
            .and_then(plausible);
        census.rows += 1;
        census.sum_g += mag_g;
        if mag_r.is_some() {
            census.present_r += 1;
        }
        if mag_i.is_some() {
            census.present_i += 1;
        }
        if mag_z.is_some() {
            census.present_z += 1;
        }
        if plx_v.is_some() {
            census.present_plx += 1;
        }
        if dist_v.is_some() {
            census.present_dist += 1;
        }
        if ext_v.is_some() {
            census.present_ext += 1;
        }
        if rv_v.is_some() {
            census.present_rv += 1;
        }
        if logt_v.is_some() {
            census.present_logt += 1;
        }
        if mini_v.is_some() {
            census.present_mini += 1;
        }
        rec.clear();
        encode_record(
            ra_v, dec_v, plx_v, mag_g, mag_r, mag_i, mag_z, dist_v, ext_v, rv_v, logt_v, mini_v,
            &mut rec,
        );
        out.write_all(&rec)
            .map_err(|e| format!("write records returned void: {e}"))?;
    }
    Ok(())
}

impl Column {
    fn cell_value(&self, row: &[u8]) -> Option<f64> {
        cell_elem(row, self, 0)
    }
}

fn compile(args: &[String]) -> Result<(), String> {
    let Some(out_path) = arg_value(args, "--out") else {
        return Err(
            "--out <decaps_dr2_stars.bin>: the asset path is never silent — refused".into(),
        );
    };
    let ci_mode = has(args, "--ci-mode");
    let mut census = Census {
        rows: 0,
        skipped: 0,
        present_r: 0,
        present_i: 0,
        present_z: 0,
        present_plx: 0,
        present_dist: 0,
        present_ext: 0,
        present_rv: 0,
        present_logt: 0,
        present_mini: 0,
        sum_g: 0.0,
    };
    let mut out = std::fs::File::create(&out_path)
        .map_err(|e| format!("create {out_path} returned void: {e}"))?;
    let mut header = Vec::with_capacity(ASSET_HEADER_BYTES);
    header.extend_from_slice(&ASSET_MAGIC);
    header.extend_from_slice(&0u32.to_le_bytes());
    header.extend_from_slice(&(RECORD_BYTES as u32).to_le_bytes());
    header.extend_from_slice(&0u32.to_le_bytes());
    out.write_all(&header)
        .map_err(|e| format!("write {out_path} header returned void: {e}"))?;

    if let Some(dir) = arg_value(args, "--input-dir") {
        let mut members: Vec<String> = std::fs::read_dir(&dir)
            .map_err(|e| format!("read_dir {dir} returned void: {e}"))?
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().to_string())
            .filter(|p| p.ends_with(".fits.gz"))
            .collect();
        members.sort();
        if members.is_empty() {
            return Err(format!(
                "{dir}: no .fits.gz members — the asset stays unwritten"
            ));
        }
        for (k, member) in members.iter().enumerate() {
            let fits_tmp = format!("/tmp/opencode/decaps_member_{}.fits", k);
            let (_, complete) = decode_gz_to_file(member, &fits_tmp)?;
            if !complete {
                eprintln!("{member}: partial stream — the asset stays partial");
            }
            let head_bytes = read_head(&fits_tmp)?;
            compile_rows(&head_bytes, &mut census, &mut out)?;
            let _ = std::fs::remove_file(&fits_tmp);
            eprintln!("{member}: {} records so far", census.rows);
        }
    } else if let Some(input) = arg_value(args, "--input") {
        let (buf, _) = decode_gz(&input)?;
        compile_rows(&buf, &mut census, &mut out)?;
    } else {
        return Err("--input <file.fits[.gz]> or --input-dir <dir> absent — refused".into());
    }
    let _ = out.flush();

    if census.rows == 0 {
        return Err(format!(
            "{out_path}: no record survived the gate — the asset stays unwritten (0 honored)"
        ));
    }
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .open(&out_path)
        .map_err(|e| format!("open {out_path} returned void: {e}"))?;
    f.seek(SeekFrom::Start(4))
        .map_err(|e| format!("seek {out_path} returned void: {e}"))?;
    f.write_all(&(census.rows as u32).to_le_bytes())
        .map_err(|e| format!("write {out_path} count returned void: {e}"))?;

    let bytes = std::fs::read(&out_path).map_err(|e| format!("read {out_path}: {e}"))?;
    let expect = ASSET_HEADER_BYTES + census.rows as usize * RECORD_BYTES;
    if bytes.len() != expect {
        return Err(format!(
            "{out_path}: {} bytes written, {} expected — the asset stays unwritten",
            bytes.len(),
            expect
        ));
    }
    if &bytes[..4] != ASSET_MAGIC {
        return Err(format!(
            "{out_path}: the header reads void — the asset stays unwritten"
        ));
    }
    let mean_g = census.sum_g / census.rows as f64;
    eprintln!(
        "{out_path}: {} records, {} bytes, sha256 {} — gated {} skipped, bands r {} i {} z {} plx {} dist50 {} ext50 {} rv50 {} logt50 {} mini50 {}, mean g {:.4}",
        census.rows,
        bytes.len(),
        sha256_hex(&bytes),
        census.skipped,
        census.present_r,
        census.present_i,
        census.present_z,
        census.present_plx,
        census.present_dist,
        census.present_ext,
        census.present_rv,
        census.present_logt,
        census.present_mini,
        mean_g
    );
    if ci_mode && !upload_release("dataverse.harvard.edu", &out_path) {
        return Err(format!("{out_path}: CDN upload returned void"));
    }
    Ok(())
}

fn read_head(path: &str) -> Result<Vec<u8>, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {path} returned void: {e}"))?;
    let mut head = vec![0u8; 4 << 20];
    let n = f
        .read(&mut head)
        .map_err(|e| format!("read {path} returned void: {e}"))?;
    head.truncate(n);
    Ok(head)
}

fn header_cards(args: &[String]) -> Result<(), String> {
    let Some(input) = arg_value(args, "--header") else {
        return Err("--header <file.fits[.gz]>: the path is never silent — refused".into());
    };
    let (buf, complete) = decode_gz(&input)?;
    let Some((hdu, _)) = find_hdu(&buf) else {
        return Err(format!(
            "{input}: no BINTABLE HDU parsed (decoded {} bytes, complete {complete})",
            buf.len()
        ));
    };
    let mut n = 0usize;
    while hdu + n * 80 + 80 <= buf.len() {
        let card = &buf[hdu + n * 80..hdu + n * 80 + 80];
        let text = String::from_utf8_lossy(card).trim_end().to_string();
        eprintln!("  {text}");
        n += 1;
        let kw = String::from_utf8_lossy(&card[0..8]);
        if kw.trim() == "END" {
            break;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = if arg_value(&args, "--inspect").is_some() {
        inspect(&args)
    } else if arg_value(&args, "--header").is_some() {
        header_cards(&args)
    } else {
        compile(&args)
    };
    if let Err(msg) = result {
        eprintln!("decaps_dr2_compiler: {msg}");
        std::process::exit(1);
    }
}
