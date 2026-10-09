use omegaflow::archivar::range::fetch_range;
use omegaflow::fits::FitsHeader;
use omegaflow::healpix::{galactic_to_icrs, pix2ang_nest};
use omegaflow::json::{JsonVal, parse_json};
use std::io::{Cursor, Read, Seek, SeekFrom};

const ACT_URL: &str = "https://lambda.gsfc.nasa.gov/data/act/maps/published/act-planck_dr4dr6_coadd_AA_daynight_f150_map_srcfree_healpix.fits";
const HDR_WINDOW: usize = 2880 * 8;
const ROW_CHUNK: usize = 2048;
const Z_CMB: f64 = 1100.0;
const BINTABLE_FIRST_COLUMN_OFFSET: usize = 0;

const JPLL: [i64; 12] = [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7];

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn isqrt(v: i64) -> i64 {
    if v <= 0 {
        return 0;
    }
    let mut r = (v as f64).sqrt() as i64;
    while (r + 1) * (r + 1) <= v {
        r += 1;
    }
    while r * r > v {
        r -= 1;
    }
    r
}

fn special_div(a: i64, b: i64) -> i64 {
    let t = if a >= (b << 1) { 1 } else { 0 };
    let a2 = a - t * (b << 1);
    (t << 1) + if a2 >= b { 1 } else { 0 }
}

fn spread_bits(v: i64, order: u32) -> i64 {
    let mut r = 0i64;
    let mut i = 0u32;
    while i < order {
        r |= ((v >> i) & 1) << (2 * i);
        i += 1;
    }
    r
}

fn is_pow2(n: i64) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

fn ring_to_nest(nside: i64, pix: i64) -> Option<i64> {
    if nside <= 0 || !is_pow2(nside) {
        return None;
    }
    let npix = 12i64.checked_mul(nside)?.checked_mul(nside)?;
    if pix < 0 || pix >= npix {
        return None;
    }
    let order = nside.trailing_zeros();
    let ncap = 2 * nside * (nside - 1);
    let nl2 = 2 * nside;
    let (iring, iphi, kshift, nr, face_num) = if pix < ncap {
        let ring = (1 + isqrt(1 + 2 * pix)) >> 1;
        let j = (pix + 1) - 2 * ring * (ring - 1);
        (ring, j, 0i64, ring, special_div(j - 1, ring))
    } else if pix < npix - ncap {
        let ip = pix - ncap;
        let tmp = ip >> (order + 2);
        let ring = tmp + nside;
        let j = ip - tmp * 4 * nside + 1;
        let shift = (ring + nside) & 1;
        let ire = tmp + 1;
        let irm = nl2 + 1 - tmp;
        let ifm = (j - (ire >> 1) + nside - 1) >> order;
        let ifp = (j - (irm >> 1) + nside - 1) >> order;
        let face = if ifp == ifm {
            ifp | 4
        } else if ifp < ifm {
            ifp
        } else {
            ifm + 8
        };
        (ring, j, shift, nside, face)
    } else {
        let ip = npix - pix;
        let ring = (1 + isqrt(2 * ip - 1)) >> 1;
        let j = 4 * ring + 1 - (ip - 2 * ring * (ring - 1));
        let face = special_div(j - 1, ring) + 8;
        let north_iring = 2 * nl2 - ring;
        (north_iring, j, 0i64, ring, face)
    };
    if !(0..12).contains(&face_num) {
        return None;
    }
    let irt = iring - ((2 + (face_num >> 2)) * nside) + 1;
    let mut ipt = 2 * iphi - JPLL[face_num as usize] * nr - kshift - 1;
    if ipt >= nl2 {
        ipt -= 8 * nside;
    }
    let ix = (ipt - irt) >> 1;
    let iy = (-ipt - irt) >> 1;
    if ix < 0 || ix >= nside || iy < 0 || iy >= nside {
        return None;
    }
    Some((face_num << (2 * order)) + spread_bits(ix, order) + (spread_bits(iy, order) << 1))
}

fn pix2ring(nside: i64, pix: i64) -> Option<i64> {
    let npix = 12i64.checked_mul(nside)?.checked_mul(nside)?;
    if pix < 0 || pix >= npix {
        return None;
    }
    let ncap = 2 * nside * (nside - 1);
    if pix < ncap {
        Some((1 + isqrt(1 + 2 * pix)) >> 1)
    } else if pix < npix - ncap {
        Some((pix - ncap) / (4 * nside) + nside)
    } else {
        Some(4 * nside - ((1 + isqrt(2 * (npix - pix) - 1)) >> 1))
    }
}

fn ring2z(nside: i64, ring: i64) -> f64 {
    let npix = (12 * nside * nside) as f64;
    let fact2 = 4.0 / npix;
    let fact1 = (2 * nside) as f64 * fact2;
    if ring < nside {
        1.0 - (ring * ring) as f64 * fact2
    } else if ring <= 3 * nside {
        (2 * nside - ring) as f64 * fact1
    } else {
        let r = 4 * nside - ring;
        (r * r) as f64 * fact2 - 1.0
    }
}

fn read_window<R: Read>(reader: &mut R, want: usize) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; want];
    let mut filled = 0usize;
    while filled < want {
        let n = reader.read(&mut buf[filled..]).ok()?;
        if n == 0 {
            break;
        }
        filled += n;
    }
    buf.truncate(filled);
    if buf.is_empty() { None } else { Some(buf) }
}

fn bintable_header(bytes: &[u8]) -> Option<(FitsHeader, usize)> {
    let (first, first_end) = FitsHeader::parse(bytes, 0)?;
    if first.value("XTENSION") == Some("'BINTABLE'") {
        return Some((first, first_end));
    }
    let (second, data_start) = FitsHeader::parse(bytes, first_end)?;
    Some((second, data_start))
}

fn parse_tform(tform: &str) -> Option<(char, usize)> {
    let t = tform.trim();
    let code = t.chars().last()?;
    let head = &t[..t.len() - code.len_utf8()];
    let repeat = if head.is_empty() {
        1
    } else {
        head.parse().ok()?
    };
    Some((code, repeat))
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

struct RingLayout {
    nside: i64,
    data_start: u64,
    row_bytes: usize,
    n_rows: usize,
    col_offset: usize,
    col_repeat: usize,
    elem: usize,
    code: char,
    unit: f64,
    coordsys: String,
}

fn ring_layout(h: &FitsHeader, data_start: usize) -> Option<RingLayout> {
    if h.value("XTENSION") != Some("'BINTABLE'") {
        eprintln!("XTENSION is not 'BINTABLE': the table stays unread");
        return None;
    }
    let ordering = match h.str_unescaped("ORDERING") {
        Some(s) => s.trim().to_string(),
        None => {
            eprintln!("ORDERING absent: the ordering stays unread");
            return None;
        }
    };
    if !ordering.eq_ignore_ascii_case("RING") {
        eprintln!("ORDERING '{ordering}': the RING arm reads RING only");
        return None;
    }
    let coordsys = match h.str_unescaped("COORDSYS") {
        Some(s) => s.trim().to_string(),
        None => {
            eprintln!("COORDSYS absent: the frame stays unread");
            return None;
        }
    };
    if coordsys != "C" && coordsys != "G" {
        eprintln!("COORDSYS '{coordsys}': the frame stays unread");
        return None;
    }
    let unit_text = match h
        .str_unescaped("TUNIT1")
        .or_else(|| h.str_unescaped("BUNIT"))
    {
        Some(s) => s,
        None => {
            eprintln!("TUNIT1 and BUNIT absent: the unit stays unread");
            return None;
        }
    };
    let unit = match unit_factor(&unit_text) {
        Some(u) => u,
        None => {
            eprintln!("unit '{unit_text}': the conversion stays unread");
            return None;
        }
    };
    let nside = match h.int("NSIDE") {
        Some(v) => v,
        None => {
            eprintln!("NSIDE absent: the pixel count stays unread");
            return None;
        }
    };
    if nside <= 0 || !is_pow2(nside) {
        eprintln!("NSIDE {nside}: the NEST ordering needs a power of two");
        return None;
    }
    let tform = match h.str_unescaped("TFORM1") {
        Some(s) => s,
        None => {
            eprintln!("TFORM1 absent: the column stays unread");
            return None;
        }
    };
    let (code, col_repeat) = match parse_tform(&tform) {
        Some(t) => t,
        None => {
            eprintln!("TFORM1 '{tform}': the column stays unread");
            return None;
        }
    };
    let elem = match code {
        'E' => 4,
        'D' => 8,
        'J' => 4,
        'I' => 2,
        'K' => 8,
        'B' => 1,
        _ => {
            eprintln!("TFORM1 '{tform}': the format code stays unread");
            return None;
        }
    };
    let row_bytes = match h.int("NAXIS1") {
        Some(v) if v > 0 => v as usize,
        _ => {
            eprintln!("NAXIS1 absent or zero: the row stays unread");
            return None;
        }
    };
    let n_rows = match h.int("NAXIS2") {
        Some(v) if v > 0 => v as usize,
        _ => {
            eprintln!("NAXIS2 absent or zero: the rows stay unread");
            return None;
        }
    };
    let npix = 12 * nside * nside;
    if (n_rows as i64).checked_mul(col_repeat as i64) != Some(npix) {
        eprintln!(
            "NAXIS2 {} x TFORM1 repeat {} != 12*NSIDE^2 {}: the table shape stays unread",
            n_rows, col_repeat, npix
        );
        return None;
    }
    let col_offset = match h.int("TBCOL1") {
        Some(t) if t > 0 => (t - 1) as usize,
        _ => BINTABLE_FIRST_COLUMN_OFFSET,
    };
    if col_offset + col_repeat * elem > row_bytes {
        eprintln!(
            "column 1 offsets {} bytes past NAXIS1 {}: the row stays unread",
            col_offset + col_repeat * elem,
            row_bytes
        );
        return None;
    }
    Some(RingLayout {
        nside,
        data_start: data_start as u64,
        row_bytes,
        n_rows,
        col_offset,
        col_repeat,
        elem,
        code,
        unit,
        coordsys,
    })
}

fn value_at(bytes: &[u8], off: usize, code: char, elem: usize) -> Option<f64> {
    let raw = bytes.get(off..off + elem)?;
    match code {
        'E' => Some(f32::from_be_bytes(raw[..4].try_into().ok()?) as f64),
        'D' => Some(f64::from_be_bytes(raw.try_into().ok()?)),
        'J' => Some(i32::from_be_bytes(raw[..4].try_into().ok()?) as f64),
        'I' => Some(i16::from_be_bytes(raw[..2].try_into().ok()?) as f64),
        'K' => Some(i64::from_be_bytes(raw.try_into().ok()?) as f64),
        'B' => Some(raw[0] as f64),
        _ => None,
    }
}

struct MapStats {
    rows: usize,
    skipped: usize,
    tmin: Option<f64>,
    tmax: Option<f64>,
    tmean: Option<f64>,
}

fn compile<R: Read + Seek>(
    reader: &mut R,
    nside_out: i64,
) -> Option<(Vec<(f64, f64, f64)>, MapStats)> {
    reader.seek(SeekFrom::Start(0)).ok()?;
    let window = read_window(reader, HDR_WINDOW)?;
    let (header, data_start) = bintable_header(&window)?;
    let layout = ring_layout(&header, data_start)?;
    if nside_out <= 0 || nside_out > layout.nside || layout.nside % nside_out != 0 {
        eprintln!(
            "--nside {} is not a power-of-two divisor of NSIDE {}: the map stays unwritten",
            nside_out, layout.nside
        );
        return None;
    }
    let ratio = layout.nside / nside_out;
    let block = (ratio * ratio) as usize;
    let npix_out = (12 * nside_out * nside_out) as usize;
    let mut sum = vec![0.0f64; npix_out];
    let mut count = vec![0u64; npix_out];

    reader.seek(SeekFrom::Start(layout.data_start)).ok()?;
    let mut buf = vec![0u8; layout.row_bytes * ROW_CHUNK];
    let mut row0 = 0usize;
    while row0 < layout.n_rows {
        let rows = ROW_CHUNK.min(layout.n_rows - row0);
        let want = rows * layout.row_bytes;
        let mut filled = 0usize;
        while filled < want {
            let n = reader.read(&mut buf[filled..want]).ok()?;
            if n == 0 {
                break;
            }
            filled += n;
        }
        if filled < want {
            eprintln!(
                "row {} of {}: {} of {} bytes read — the map stays unwritten",
                row0, layout.n_rows, filled, want
            );
            return None;
        }
        for r in 0..rows {
            let row_base = r * layout.row_bytes + layout.col_offset;
            for i in 0..layout.col_repeat {
                let p = ((row0 + r) * layout.col_repeat + i) as i64;
                let v = match value_at(&buf, row_base + i * layout.elem, layout.code, layout.elem) {
                    Some(v) => v,
                    None => continue,
                };
                if !v.is_finite() {
                    continue;
                }
                let q = match ring_to_nest(layout.nside, p) {
                    Some(q) => q,
                    None => continue,
                };
                let c = (q as usize) / block;
                if c < npix_out {
                    sum[c] += v;
                    count[c] += 1;
                }
            }
        }
        row0 += rows;
    }

    let mut rows_out: Vec<(f64, f64, f64)> = Vec::new();
    let mut skipped = 0usize;
    let mut tsum = 0.0f64;
    let mut tmin = f64::INFINITY;
    let mut tmax = f64::NEG_INFINITY;
    for c in 0..npix_out {
        if count[c] == 0 {
            skipped += 1;
            continue;
        }
        let (theta, phi) = match pix2ang_nest(nside_out, c as i64) {
            Some(t) => t,
            None => {
                skipped += 1;
                continue;
            }
        };
        let (ra, dec) = match layout.coordsys.as_str() {
            "C" => (
                phi.to_degrees().rem_euclid(360.0),
                90.0 - theta.to_degrees(),
            ),
            "G" => galactic_to_icrs(theta, phi),
            _ => {
                skipped += 1;
                continue;
            }
        };
        let t = (sum[c] / count[c] as f64) * layout.unit;
        rows_out.push((ra, dec, t));
        tsum += t;
        if t < tmin {
            tmin = t;
        }
        if t > tmax {
            tmax = t;
        }
    }
    let stats = if rows_out.is_empty() {
        MapStats {
            rows: 0,
            skipped,
            tmin: None,
            tmax: None,
            tmean: None,
        }
    } else {
        MapStats {
            rows: rows_out.len(),
            skipped,
            tmin: Some(tmin),
            tmax: Some(tmax),
            tmean: Some(tsum / rows_out.len() as f64),
        }
    };
    Some((rows_out, stats))
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

fn probe(url: &str) -> bool {
    let bytes = match fetch_range(url, 0, HDR_WINDOW as u64, &[]) {
        Some(b) => b,
        None => {
            eprintln!("header range returned void: {url}");
            return false;
        }
    };
    if bytes.len() < 2880 {
        eprintln!(
            "header window {} B is shorter than one FITS block",
            bytes.len()
        );
        return false;
    }
    let (h, _data) = match bintable_header(&bytes) {
        Some(v) => v,
        None => {
            eprintln!("no BINTABLE header within {} B", bytes.len());
            return false;
        }
    };
    for key in [
        "XTENSION", "NAXIS1", "NAXIS2", "NSIDE", "ORDERING", "COORDSYS", "TFORM1", "TTYPE1",
        "TUNIT1", "BUNIT", "EXTNAME",
    ] {
        match h.value(key) {
            Some(v) => eprintln!("{key:<9}= {v}"),
            None => eprintln!("{key:<9}= <absent>"),
        }
    }
    let ordering = match h.str_unescaped("ORDERING") {
        Some(s) => s.trim().to_string(),
        None => {
            eprintln!("ORDERING absent");
            return false;
        }
    };
    let coordsys = match h.str_unescaped("COORDSYS") {
        Some(s) => s.trim().to_string(),
        None => {
            eprintln!("COORDSYS absent");
            return false;
        }
    };
    let nside = match h.int("NSIDE") {
        Some(v) => v,
        None => {
            eprintln!("NSIDE absent");
            return false;
        }
    };
    let tform = match h.str_unescaped("TFORM1") {
        Some(s) => s,
        None => {
            eprintln!("TFORM1 absent");
            return false;
        }
    };
    let (code, repeat) = match parse_tform(&tform) {
        Some(t) => t,
        None => {
            eprintln!("TFORM1 '{tform}' unread");
            return false;
        }
    };
    let ring = ordering.eq_ignore_ascii_case("RING");
    let equatorial = coordsys == "C";
    let nside_ok = is_pow2(nside);
    let tform_ok = code == 'E' && repeat == 1024;
    eprintln!(
        "probe: ORDERING={ordering} RING={ring}; COORDSYS={coordsys} C={equatorial}; NSIDE={nside} power-of-two={nside_ok}; TFORM1='{tform}' code={code} repeat={repeat}"
    );
    ring && equatorial && nside_ok && tform_ok
}

fn card(kw: &str, val: &str) -> String {
    format!("{:<8}= {:<70}", kw, val)
}

fn pad_header(cards: String) -> Vec<u8> {
    let mut header = cards.into_bytes();
    while header.len() % 2880 != 0 {
        header.push(b' ');
    }
    header
}

fn synth_ring_table(
    nside: i64,
    values: &[f64],
    ordering: &str,
    coordsys: &str,
    unit: &str,
) -> Vec<u8> {
    let npix = (12 * nside * nside) as usize;
    assert_eq!(values.len(), npix);
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&pad_header(format!(
        "{}{}{}",
        card("SIMPLE", "T"),
        card("BITPIX", "8"),
        card("NAXIS", "0")
    )));
    buf.extend_from_slice(&pad_header(format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        card("XTENSION", "'BINTABLE'"),
        card("BITPIX", "8"),
        card("NAXIS", "2"),
        card("NAXIS1", "4"),
        card("NAXIS2", &npix.to_string()),
        card("PCOUNT", "0"),
        card("GCOUNT", "1"),
        card("TFIELDS", "1"),
        card("TTYPE1", "'TEMPERATURE'"),
        card("TFORM1", "'1E'"),
        card("TBCOL1", "1"),
        card("NSIDE", &nside.to_string()),
        card("ORDERING", &format!("'{}'", ordering)),
        card("COORDSYS", &format!("'{}'", coordsys))
    )));
    let mut tail = format!(
        "{}{}",
        card("BUNIT", &format!("'{}'", unit)),
        card("END", "")
    )
    .into_bytes();
    while tail.len() % 2880 != 0 {
        tail.push(b' ');
    }
    buf.extend_from_slice(&tail);
    for v in values {
        buf.extend_from_slice(&(*v as f32).to_be_bytes());
    }
    while buf.len() % 2880 != 0 {
        buf.push(0);
    }
    buf
}

fn fixture_check() -> bool {
    let values = vec![1000.0f64; 12];
    let ring = synth_ring_table(1, &values, "RING", "C", "uK");
    let mut cursor = Cursor::new(ring);
    let (rows, stats) = match compile(&mut cursor, 1) {
        Some(v) => v,
        None => {
            eprintln!("fixture: compile returned void");
            return false;
        }
    };
    let mut values_ok = rows.len() == 12 && stats.rows == 12 && stats.skipped == 0;
    for (_, _, t) in &rows {
        if (*t - 1e-3).abs() > 1e-12 {
            values_ok = false;
        }
    }
    let nested = synth_ring_table(1, &values, "NESTED", "C", "uK");
    let refused = match bintable_header(&nested) {
        Some((h, data_start)) => ring_layout(&h, data_start).is_none(),
        None => false,
    };
    eprintln!(
        "fixture: RING rows {} T=1e-3 ok={} NESTED refused={}",
        rows.len(),
        values_ok,
        refused
    );
    values_ok && refused
}

fn selftest() -> bool {
    let mut ok = true;
    for n in [1i64, 2, 4, 8, 16, 32] {
        let npix = 12 * n * n;
        let mut seen = vec![false; npix as usize];
        let mut perm = true;
        for p in 0..npix {
            match ring_to_nest(n, p) {
                Some(q) if q >= 0 && q < npix && !seen[q as usize] => seen[q as usize] = true,
                _ => {
                    perm = false;
                    break;
                }
            }
        }
        let full = seen.iter().all(|&b| b);
        let mut zmax = 0.0f64;
        for p in 0..npix {
            let q = match ring_to_nest(n, p) {
                Some(q) => q,
                None => continue,
            };
            let ring = match pix2ring(n, p) {
                Some(r) => r,
                None => continue,
            };
            let theta = match pix2ang_nest(n, q) {
                Some((t, _)) => t,
                None => continue,
            };
            let d = (ring2z(n, ring) - theta.cos()).abs();
            if d > zmax {
                zmax = d;
            }
        }
        let good = perm && full && zmax < 1e-6;
        eprintln!(
            "nside {n} npix {npix} permutation={perm} full={full} max|z-ringz|={zmax:.3e} {}",
            if good { "ok" } else { "VOID" }
        );
        if !good {
            ok = false;
        }
    }
    if ring_to_nest(1, -1).is_some() || ring_to_nest(1, 12).is_some() {
        eprintln!("out-of-range pixel accepted");
        ok = false;
    }
    if ring_to_nest(3, 0).is_some() {
        eprintln!("non-power-of-two nside accepted");
        ok = false;
    }
    ok
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if has_flag(&args, "--selftest") {
        if selftest() && fixture_check() {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }
    if has_flag(&args, "--probe") {
        let url = match arg_value(&args, "--url") {
            Some(u) => u,
            None => ACT_URL.to_string(),
        };
        if probe(&url) {
            std::process::exit(0);
        } else {
            std::process::exit(2);
        }
    }
    let input = match arg_value(&args, "--input") {
        Some(v) => v,
        None => {
            eprintln!(
                "usage: cmb_act_compiler --probe [--url <fits>] | --input <fits> [--nside 64] [--out path] [--ci-mode] | --selftest"
            );
            std::process::exit(1);
        }
    };
    let nside_out: i64 = match arg_value(&args, "--nside") {
        Some(v) => match v.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("--nside {v} is not an integer");
                std::process::exit(1);
            }
        },
        None => 64,
    };
    let out = match arg_value(&args, "--out") {
        Some(o) => o,
        None => format!("cmb_act_f150_n{nside_out}.json"),
    };
    let ci_mode = has_flag(&args, "--ci-mode");

    let mut file = match std::fs::File::open(&input) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("read {input} returned void: {e}");
            std::process::exit(1);
        }
    };
    let (rows, stats) = match compile(&mut file, nside_out) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    eprintln!(
        "{} rows, {} empty cells skipped, T mean {:?} K, min {:?} K, max {:?} K",
        stats.rows, stats.skipped, stats.tmean, stats.tmin, stats.tmax
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
        let _ = omegaflow::cdn::upload_release("lambda.gsfc.nasa.gov", &out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn ring_layout_reads_a_ring_table() {
        let nside = 1;
        let values = vec![1000.0f64; 12];
        let bytes = synth_ring_table(nside, &values, "RING", "C", "uK");
        let window = bytes.clone();
        let (h, data_start) = bintable_header(&window).unwrap();
        let layout = ring_layout(&h, data_start).unwrap();
        assert_eq!(layout.nside, nside);
        assert_eq!(layout.n_rows, 12);
        assert_eq!(layout.col_offset, 0);
        assert_eq!(layout.col_repeat, 1);
        assert_eq!(layout.elem, 4);
        assert_eq!(layout.coordsys, "C");
        assert!((layout.unit - 1e-6).abs() < 1e-30);
    }

    #[test]
    fn compile_converts_ring_field_uk_to_k() {
        let nside = 1;
        let values = vec![1000.0f64; 12];
        let bytes = synth_ring_table(nside, &values, "RING", "C", "uK");
        let mut cursor = Cursor::new(bytes);
        let (rows, stats) = compile(&mut cursor, 1).unwrap();
        assert_eq!(rows.len(), 12);
        assert_eq!(stats.rows, 12);
        assert_eq!(stats.skipped, 0);
        for (_, _, t) in &rows {
            assert!((*t - 1e-3).abs() < 1e-12, "uK 1000 -> K {}", t);
        }
    }

    #[test]
    fn compile_reads_mk_unit() {
        let nside = 1;
        let values = vec![1000.0f64; 12];
        let bytes = synth_ring_table(nside, &values, "RING", "C", "mK");
        let mut cursor = Cursor::new(bytes);
        let (rows, _) = compile(&mut cursor, 1).unwrap();
        for (_, _, t) in &rows {
            assert!((*t - 1.0).abs() < 1e-12, "mK 1000 -> K {}", t);
        }
    }

    #[test]
    fn ring_layout_refuses_nested() {
        let bytes = synth_ring_table(1, &vec![1.0; 12], "NESTED", "C", "uK");
        let (h, data_start) = bintable_header(&bytes).unwrap();
        assert!(ring_layout(&h, data_start).is_none());
    }

    #[test]
    fn ring_to_nest_is_a_permutation() {
        for n in [1i64, 2, 4, 8] {
            let npix = 12 * n * n;
            let mut seen = vec![false; npix as usize];
            for p in 0..npix {
                let q = ring_to_nest(n, p).unwrap();
                assert!(q >= 0 && q < npix);
                assert!(!seen[q as usize]);
                seen[q as usize] = true;
            }
            assert!(seen.iter().all(|&b| b));
        }
    }

    #[test]
    fn ring_to_nest_rejects_bad_input() {
        assert!(ring_to_nest(1, -1).is_none());
        assert!(ring_to_nest(1, 12).is_none());
        assert!(ring_to_nest(3, 0).is_none());
    }

    #[test]
    fn ring_to_nest_keeps_the_ring_latitude() {
        for n in [1i64, 2, 4] {
            let npix = 12 * n * n;
            for p in 0..npix {
                let q = ring_to_nest(n, p).unwrap();
                let ring = pix2ring(n, p).unwrap();
                let (theta, _) = pix2ang_nest(n, q).unwrap();
                assert!((ring2z(n, ring) - theta.cos()).abs() < 1e-9);
            }
        }
    }
}
