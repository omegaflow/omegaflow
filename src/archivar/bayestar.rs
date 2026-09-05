use super::fits::FitsHeader;
use crate::mathematikerin::healpix::ang2pix_nest;

pub const BE19_MU0: f64 = 4.0;
pub const BE19_DMU: f64 = 0.125;
pub const BE19_BINS: usize = 120;

const MAGIC: [u8; 4] = *b"BE19";
const HEADER_LEN: usize = 31;
pub const ASSET_HEADER_LEN: usize = HEADER_LEN;
pub const REC_BYTES: usize = 496;

#[derive(Clone, Copy)]
pub struct Be19Col {
    pub off: usize,
    pub code: char,
    pub repeat: usize,
}

pub struct Be19Table {
    pub n_rows: u64,
    pub row_bytes: usize,
    pub data_start: u64,
    pub nside: Be19Col,
    pub pix: Be19Col,
    pub converged: Be19Col,
    pub dm_min: Be19Col,
    pub dm_max: Be19Col,
    pub n_good: Be19Col,
    pub best: Be19Col,
}

pub struct Be19Row {
    pub nside: u32,
    pub ipix: u32,
    pub converged: bool,
    pub dm_min: f32,
    pub dm_max: f32,
    pub n_good: u32,
    pub best_fit: [f32; BE19_BINS],
}

fn elem_width(code: char) -> Option<usize> {
    match code {
        'E' => Some(4),
        'D' => Some(8),
        'J' => Some(4),
        'I' => Some(2),
        'K' => Some(8),
        'B' => Some(1),
        'L' => Some(1),
        _ => None,
    }
}

fn tform_width(tform: &str) -> Option<(char, usize)> {
    let t = tform.trim();
    let bytes = t.as_bytes();
    let code = *bytes.last()? as char;
    let elem = elem_width(code)?;
    let repeat = if bytes.len() == 1 {
        1
    } else {
        std::str::from_utf8(&bytes[..bytes.len() - 1])
            .ok()?
            .trim()
            .parse::<usize>()
            .ok()?
    };
    Some((code, repeat * elem))
}

fn column_of(h: &FitsHeader, i: usize, off: usize) -> Option<(Be19Col, usize)> {
    let tform = h.str_unescaped(&format!("TFORM{i}"))?;
    let (code, width) = tform_width(&tform)?;
    let (col, next) = match h.int(&format!("TBCOL{i}")) {
        Some(t) if t > 0 => (
            Be19Col {
                off: t as usize - 1,
                code,
                repeat: width,
            },
            t as usize + width - 1,
        ),
        _ => (
            Be19Col {
                off,
                code,
                repeat: width,
            },
            off + width,
        ),
    };
    Some((col, next))
}

pub fn table_of(head: &[u8]) -> Option<Be19Table> {
    let (_, h1_off) = FitsHeader::parse(head, 0)?;
    let (h, data_start) = FitsHeader::parse(head, h1_off)?;
    if h.value("XTENSION") != Some("'BINTABLE'") {
        return None;
    }
    let row_bytes = h.int("NAXIS1")? as usize;
    let n_rows = h.int("NAXIS2")? as u64;
    let tfields = h.int("TFIELDS")? as usize;
    let mut cols = Vec::with_capacity(tfields);
    let mut off = 0usize;
    for i in 1..=tfields {
        let (col, next) = column_of(&h, i, off)?;
        cols.push(col);
        off = next;
    }
    if off > row_bytes {
        return None;
    }
    let find = |want: &str| {
        let mut c = None;
        for (k, col) in cols.iter().enumerate() {
            let ty = h.str_unescaped(&format!("TTYPE{}", k + 1));
            if ty.as_deref().map(str::trim) == Some(want) {
                c = Some(*col);
            }
        }
        c
    };
    Some(Be19Table {
        n_rows,
        row_bytes,
        data_start: data_start as u64,
        nside: find("NSIDE")?,
        pix: find("HEALPIX_INDEX")?,
        converged: find("CONVERGED")?,
        dm_min: find("DM_RELIABLE_MIN")?,
        dm_max: find("DM_RELIABLE_MAX")?,
        n_good: find("N_GOOD")?,
        best: find("BEST_FIT")?,
    })
}

fn be_i32(b: &[u8]) -> i32 {
    i32::from_be_bytes(b.try_into().expect("i32 slice"))
}

fn be_i64(b: &[u8]) -> i64 {
    i64::from_be_bytes(b.try_into().expect("i64 slice"))
}

fn be_f32(b: &[u8]) -> f32 {
    f32::from_be_bytes(b.try_into().expect("f32 slice"))
}

fn col_bytes<'a>(row: &'a [u8], c: &Be19Col) -> &'a [u8] {
    &row[c.off..c.off + c.repeat]
}

pub fn decode_row(row: &[u8], t: &Be19Table) -> Option<Be19Row> {
    if row.len() != t.row_bytes {
        return None;
    }
    let nside = be_i32(col_bytes(row, &t.nside));
    if nside <= 0 || nside & (nside - 1) != 0 {
        return None;
    }
    let ipix = be_i64(col_bytes(row, &t.pix));
    let npix = 12 * (nside as i64) * (nside as i64);
    if ipix < 0 || ipix >= npix {
        return None;
    }
    let conv = row[t.converged.off];
    let mut best_fit = [0.0f32; BE19_BINS];
    for (j, b) in best_fit.iter_mut().enumerate() {
        let raw = &row[t.best.off + 4 * j..t.best.off + 4 * (j + 1)];
        *b = be_f32(raw);
    }
    Some(Be19Row {
        nside: nside as u32,
        ipix: ipix as u32,
        converged: conv == 1 || conv == b'T',
        dm_min: be_f32(col_bytes(row, &t.dm_min)),
        dm_max: be_f32(col_bytes(row, &t.dm_max)),
        n_good: be_i32(col_bytes(row, &t.n_good)).max(0) as u32,
        best_fit,
    })
}

pub fn mu_of_r_pc(r_pc: f64) -> Option<f64> {
    if r_pc.is_finite() && r_pc > 0.0 {
        Some(5.0 * r_pc.log10() - 5.0)
    } else {
        None
    }
}

pub fn ebv_at(bf: &[f32; BE19_BINS], mu: f64) -> Option<f64> {
    if !mu.is_finite() {
        return None;
    }
    let e0 = BE19_MU0;
    let elast = e0 + BE19_DMU * (BE19_BINS as f64 - 1.0);
    if mu <= e0 {
        let a = 10f64.powf(0.2 * (mu - e0));
        return Some(bf[0] as f64 * a.max(0.0));
    }
    for j in 0..BE19_BINS - 1 {
        let ej = e0 + BE19_DMU * j as f64;
        if mu < ej + BE19_DMU {
            let t = ((mu - ej) / BE19_DMU).clamp(0.0, 1.0);
            return Some(bf[j] as f64 * (1.0 - t) + bf[j + 1] as f64 * t);
        }
    }
    if mu >= elast {
        return Some(bf[BE19_BINS - 1] as f64);
    }
    None
}

pub struct MapHeader {
    pub n_rows: u64,
    pub mu0: f64,
    pub dmu: f64,
    pub bins: u16,
}

pub fn write_header(out: &mut Vec<u8>, h: &MapHeader) {
    out.extend_from_slice(&MAGIC);
    out.push(1);
    out.extend_from_slice(&h.n_rows.to_le_bytes());
    out.extend_from_slice(&h.mu0.to_le_bytes());
    out.extend_from_slice(&h.dmu.to_le_bytes());
    out.extend_from_slice(&h.bins.to_le_bytes());
}

pub fn parse_header(b: &[u8]) -> Option<MapHeader> {
    if b.len() < HEADER_LEN || b[0..4] != MAGIC || b[4] != 1 {
        return None;
    }
    let n_rows = u64::from_le_bytes(b[5..13].try_into().ok()?);
    let mu0 = f64::from_le_bytes(b[13..21].try_into().ok()?);
    let dmu = f64::from_le_bytes(b[21..29].try_into().ok()?);
    let bins = u16::from_le_bytes(b[29..31].try_into().ok()?);
    Some(MapHeader {
        n_rows,
        mu0,
        dmu,
        bins,
    })
}

pub fn encode_rec(out: &mut [u8; REC_BYTES], r: &Be19Row) {
    out.fill(0);
    let order = r.nside.trailing_zeros() as u8;
    out[0] = order;
    out[1] = r.converged as u8;
    out[4..8].copy_from_slice(&r.ipix.to_le_bytes());
    out[8..12].copy_from_slice(&r.dm_min.to_le_bytes());
    out[12..16].copy_from_slice(&r.dm_max.to_le_bytes());
    for (j, b) in r.best_fit.iter().enumerate() {
        out[16 + 4 * j..20 + 4 * j].copy_from_slice(&b.to_le_bytes());
    }
}

pub fn decode_rec(b: &[u8]) -> Option<Be19Row> {
    if b.len() != REC_BYTES {
        return None;
    }
    let order = b[0];
    let nside = 1u32 << order;
    let mut best_fit = [0.0f32; BE19_BINS];
    for (j, v) in best_fit.iter_mut().enumerate() {
        let raw: [u8; 4] = b[16 + 4 * j..20 + 4 * j].try_into().ok()?;
        *v = f32::from_le_bytes(raw);
    }
    Some(Be19Row {
        nside,
        ipix: u32::from_le_bytes(b[4..8].try_into().ok()?),
        converged: b[1] == 1,
        dm_min: f32::from_le_bytes(b[8..12].try_into().ok()?),
        dm_max: f32::from_le_bytes(b[12..16].try_into().ok()?),
        n_good: 0,
        best_fit,
    })
}

pub struct MapQuery {
    pub orders: Vec<u8>,
    pub keys: Vec<Vec<u64>>,
}

pub fn build_index() -> MapQuery {
    MapQuery {
        orders: Vec::new(),
        keys: Vec::new(),
    }
}

pub fn index_add(q: &mut MapQuery, idx: u64, order: u8, ipix: u32) {
    let oi = match q.orders.iter().position(|&o| o == order) {
        Some(o) => o,
        None => {
            q.orders.push(order);
            q.keys.push(Vec::new());
            q.orders.len() - 1
        }
    };
    q.keys[oi].push(((ipix as u64) << 32) | idx);
}

pub fn index_sort(q: &mut MapQuery) {
    let mut perm: Vec<usize> = (0..q.orders.len()).collect();
    perm.sort_by_key(|&i| q.orders[i]);
    let orders: Vec<u8> = perm.iter().map(|&i| q.orders[i]).collect();
    let keys: Vec<Vec<u64>> = perm.iter().map(|&i| q.keys[i].clone()).collect();
    q.orders = orders;
    q.keys = keys;
    for arr in q.keys.iter_mut() {
        arr.sort_unstable();
    }
}

pub fn leaf_record(q: &MapQuery, theta: f64, phi: f64) -> Option<u64> {
    let mut best: Option<u64> = None;
    for (oi, &order) in q.orders.iter().enumerate() {
        let nside = 1i64 << order;
        let pix = ang2pix_nest(nside, theta, phi)?;
        let want = (pix as u64) << 32;
        let arr = &q.keys[oi];
        let pos = arr.partition_point(|&k| k < want);
        if pos < arr.len() && arr[pos] >> 32 == pix as u64 {
            best = Some(arr[pos] & 0xFFFF_FFFF);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dm_grid_spans_63pc_to_60kpc() {
        let r0 = 10f64.powf(BE19_MU0 / 5.0 + 1.0);
        let r1 = 10f64.powf((BE19_MU0 + BE19_DMU * 119.0) / 5.0 + 1.0);
        assert!((r0 - 63.1).abs() < 0.2);
        assert!((r1 - 5.96e4).abs() < 300.0);
    }

    #[test]
    fn ebv_truncated_matches_steps() {
        let mut bf = [0.0f32; BE19_BINS];
        for (j, v) in bf.iter_mut().enumerate() {
            *v = (j as f32 + 1.0) * 0.01;
        }
        assert!((ebv_at(&bf, 4.0).unwrap() - 0.01).abs() < 1e-7);
        assert!((ebv_at(&bf, 4.125).unwrap() - 0.02).abs() < 1e-6);
        let mid = ebv_at(&bf, 4.0 + 0.0625).unwrap();
        assert!((mid - 0.015).abs() < 1e-6);
        assert!((ebv_at(&bf, 30.0).unwrap() - 1.20).abs() < 1e-6);
        let near = ebv_at(&bf, 3.0).unwrap();
        assert!(near > 0.001 && near < 0.01);
    }

    #[test]
    fn encode_decode_roundtrip() {
        let mut bf = [0.0f32; BE19_BINS];
        bf[5] = 0.123;
        bf[119] = 0.5;
        let row = Be19Row {
            nside: 512,
            ipix: 123456,
            converged: true,
            dm_min: 4.5,
            dm_max: 13.25,
            n_good: 7,
            best_fit: bf,
        };
        let mut buf = [0u8; REC_BYTES];
        encode_rec(&mut buf, &row);
        let back = decode_rec(&buf).unwrap();
        assert_eq!(back.nside, 512);
        assert_eq!(back.ipix, 123456);
        assert!(back.converged);
        assert_eq!(back.dm_min, 4.5);
        assert_eq!(back.dm_max, 13.25);
        assert!((back.best_fit[5] - 0.123).abs() < 1e-7);
    }

    #[test]
    fn header_roundtrip() {
        let mut b = Vec::new();
        let h = MapHeader {
            n_rows: 5,
            mu0: 4.0,
            dmu: 0.125,
            bins: 120,
        };
        write_header(&mut b, &h);
        let back = parse_header(&b).unwrap();
        assert_eq!(back.n_rows, 5);
        assert_eq!(back.mu0, 4.0);
        assert_eq!(back.bins, 120);
    }
}
