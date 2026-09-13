#[derive(Debug, PartialEq)]
pub enum SqliteValue {
    Null,
    Int(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

pub struct SqliteDb {
    bytes: Vec<u8>,
    page_size: usize,
    page_count: u32,
}

impl SqliteDb {
    pub fn from_bytes(bytes: Vec<u8>) -> Option<SqliteDb> {
        if bytes.len() < 100 {
            return None;
        }
        if &bytes[0..16] != b"SQLite format 3\0" {
            return None;
        }
        let raw = u16::from_be_bytes([bytes[16], bytes[17]]) as usize;
        let page_size = if raw == 1 { 65536 } else { raw };
        if page_size == 0 || page_size & (page_size - 1) != 0 {
            return None;
        }
        let page_count = u32::from_be_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);
        Some(SqliteDb {
            bytes,
            page_size,
            page_count,
        })
    }

    pub fn table_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for row in self.master_rows() {
            if row.len() < 2 {
                continue;
            }
            let type_col = match &row[0] {
                SqliteValue::Text(t) => t.as_str(),
                _ => continue,
            };
            if type_col != "table" {
                continue;
            }
            if let SqliteValue::Text(name) = &row[1] {
                names.push(name.clone());
            }
        }
        names
    }

    pub fn read_table(&self, name: &str) -> Option<Vec<Vec<SqliteValue>>> {
        let root = self.find_root(name)?;
        let mut pages = Vec::new();
        self.collect_leaf_pages(root, &mut pages)?;
        let mut rows = Vec::new();
        for page in pages {
            rows.extend(leaf_rows(self.page(page)?, base_of(page))?);
        }
        Some(rows)
    }

    fn page(&self, num: u32) -> Option<&[u8]> {
        if num == 0 || num > self.page_count {
            return None;
        }
        let start = (num as usize - 1) * self.page_size;
        self.bytes.get(start..start + self.page_size)
    }

    fn master_rows(&self) -> Vec<Vec<SqliteValue>> {
        let mut pages = Vec::new();
        if self.collect_leaf_pages(1, &mut pages).is_none() {
            return Vec::new();
        }
        let mut rows = Vec::new();
        for page in pages {
            if let Some(slice) = self.page(page) {
                if let Some(cells) = leaf_rows(slice, base_of(page)) {
                    rows.extend(cells);
                }
            }
        }
        rows
    }

    fn find_root(&self, name: &str) -> Option<u32> {
        for row in self.master_rows() {
            if row.len() < 4 {
                continue;
            }
            let type_col = match &row[0] {
                SqliteValue::Text(t) => t.as_str(),
                _ => continue,
            };
            if type_col != "table" {
                continue;
            }
            let name_col = match &row[1] {
                SqliteValue::Text(n) => n.as_str(),
                _ => continue,
            };
            if name_col != name {
                continue;
            }
            if let SqliteValue::Int(root) = row[3] {
                if root > 0 {
                    return Some(root as u32);
                }
            }
        }
        None
    }

    fn collect_leaf_pages(&self, num: u32, out: &mut Vec<u32>) -> Option<()> {
        let page = self.page(num)?;
        if page.is_empty() {
            return None;
        }
        let base = base_of(num);
        match page[base] {
            0x0d => {
                out.push(num);
                Some(())
            }
            0x05 => {
                let cell_count = read_u16_be(page, base + 3)? as usize;
                let mut children = Vec::with_capacity(cell_count + 1);
                for i in 0..cell_count {
                    let ptr = read_u16_be(page, base + 12 + i * 2)? as usize;
                    children.push(read_u32_be(page, ptr)?);
                }
                let right = read_u32_be(page, base + 8)?;
                if right != 0 {
                    children.push(right);
                }
                for child in children {
                    self.collect_leaf_pages(child, out)?;
                }
                Some(())
            }
            _ => None,
        }
    }
}

pub fn read_sqlite(path: &str) -> Option<SqliteDb> {
    SqliteDb::from_bytes(std::fs::read(path).ok()?)
}

fn base_of(num: u32) -> usize {
    if num == 1 {
        100
    } else {
        0
    }
}

fn leaf_rows(page: &[u8], base: usize) -> Option<Vec<Vec<SqliteValue>>> {
    if page.len() < base + 8 || page[base] != 0x0d {
        return None;
    }
    let cell_count = read_u16_be(page, base + 3)? as usize;
    let mut rows = Vec::with_capacity(cell_count);
    for i in 0..cell_count {
        let ptr = read_u16_be(page, base + 8 + i * 2)? as usize;
        rows.push(decode_cell(page, ptr)?);
    }
    Some(rows)
}

fn decode_cell(page: &[u8], ptr: usize) -> Option<Vec<SqliteValue>> {
    let (payload_len, adv1) = read_varint(page, ptr)?;
    if payload_len < 0 {
        return None;
    }
    let payload_len = payload_len as usize;
    let (_, adv2) = read_varint(page, ptr + adv1)?;
    let off = ptr + adv1 + adv2;
    let payload = page.get(off..off + payload_len)?;
    decode_record(payload)
}

fn decode_record(payload: &[u8]) -> Option<Vec<SqliteValue>> {
    let (header_len, mut cursor) = read_varint(payload, 0)?;
    let header_len = header_len as usize;
    if header_len == 0 || header_len > payload.len() {
        return None;
    }
    let mut serial = Vec::new();
    while cursor < header_len {
        let (st, adv) = read_varint(payload, cursor)?;
        serial.push(st);
        cursor += adv;
    }
    let mut data = header_len;
    let mut values = Vec::with_capacity(serial.len());
    for st in serial {
        let (value, size) = decode_value(payload, data, st)?;
        values.push(value);
        data += size;
    }
    Some(values)
}

fn decode_value(payload: &[u8], data: usize, st: i64) -> Option<(SqliteValue, usize)> {
    match st {
        0 => Some((SqliteValue::Null, 0)),
        1 => Some((SqliteValue::Int(read_i8(payload, data)?), 1)),
        2 => Some((SqliteValue::Int(read_i16(payload, data)?), 2)),
        3 => Some((SqliteValue::Int(read_i24(payload, data)?), 3)),
        4 => Some((SqliteValue::Int(read_i32(payload, data)?), 4)),
        5 => Some((SqliteValue::Int(read_i48(payload, data)?), 6)),
        6 => Some((SqliteValue::Int(read_i64(payload, data)?), 8)),
        7 => Some((SqliteValue::Real(read_f64(payload, data)?), 8)),
        8 => Some((SqliteValue::Int(0), 0)),
        9 => Some((SqliteValue::Int(1), 0)),
        10 | 11 => None,
        _ => {
            let len = ((st - 12) / 2) as usize;
            let bytes = payload.get(data..data + len)?;
            let value = if st % 2 == 0 {
                SqliteValue::Blob(bytes.to_vec())
            } else {
                SqliteValue::Text(std::str::from_utf8(bytes).ok()?.to_string())
            };
            Some((value, len))
        }
    }
}

fn read_varint(b: &[u8], off: usize) -> Option<(i64, usize)> {
    let mut val: i64 = 0;
    for i in 0..9 {
        let byte = *b.get(off + i)?;
        if i == 8 {
            val = (val << 8) | i64::from(byte);
            return Some((val, 9));
        }
        val = (val << 7) | i64::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return Some((val, i + 1));
        }
    }
    None
}

fn read_u16_be(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*b.get(off)?, *b.get(off + 1)?]))
}

fn read_u32_be(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes([
        *b.get(off)?,
        *b.get(off + 1)?,
        *b.get(off + 2)?,
        *b.get(off + 3)?,
    ]))
}

fn read_u24(b: &[u8], off: usize) -> Option<u32> {
    Some(
        (u32::from(*b.get(off)?) << 16)
            | (u32::from(*b.get(off + 1)?) << 8)
            | u32::from(*b.get(off + 2)?),
    )
}

fn read_u48(b: &[u8], off: usize) -> Option<u64> {
    let mut v = 0u64;
    for i in 0..6 {
        v = (v << 8) | u64::from(*b.get(off + i)?);
    }
    Some(v)
}

fn read_i8(b: &[u8], off: usize) -> Option<i64> {
    Some(i64::from(*b.get(off)? as i8))
}

fn read_i16(b: &[u8], off: usize) -> Option<i64> {
    Some(i64::from(i16::from_be_bytes([
        *b.get(off)?,
        *b.get(off + 1)?,
    ])))
}

fn read_i24(b: &[u8], off: usize) -> Option<i64> {
    let raw = read_u24(b, off)?;
    let sign = if raw & 0x0080_0000 != 0 {
        0xff00_0000
    } else {
        0
    };
    Some(i64::from((raw | sign) as i32))
}

fn read_i32(b: &[u8], off: usize) -> Option<i64> {
    Some(i64::from(i32::from_be_bytes([
        *b.get(off)?,
        *b.get(off + 1)?,
        *b.get(off + 2)?,
        *b.get(off + 3)?,
    ])))
}

fn read_i48(b: &[u8], off: usize) -> Option<i64> {
    let raw = read_u48(b, off)?;
    let sign = if raw & 0x0000_8000_0000_0000 != 0 {
        0xffff_0000_0000_0000
    } else {
        0
    };
    Some((raw | sign) as i64)
}

fn read_i64(b: &[u8], off: usize) -> Option<i64> {
    let mut arr = [0u8; 8];
    arr.copy_from_slice(b.get(off..off + 8)?);
    Some(i64::from_be_bytes(arr))
}

fn read_f64(b: &[u8], off: usize) -> Option<f64> {
    let mut arr = [0u8; 8];
    arr.copy_from_slice(b.get(off..off + 8)?);
    Some(f64::from_be_bytes(arr))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: usize = 512;

    fn varint_len(v: u64) -> usize {
        match v {
            0..=0x7f => 1,
            0x80..=0x3fff => 2,
            0x4000..=0x1fffff => 3,
            0x200000..=0xfffffff => 4,
            0x10000000..=0x7ffffffff => 5,
            0x800000000..=0x3ffffffffff => 6,
            0x40000000000..=0x1ffffffffffff => 7,
            0x2000000000000..=0xffffffffffffff => 8,
            _ => 9,
        }
    }

    fn put_varint(mut v: u64, out: &mut Vec<u8>) {
        if v < (1u64 << 56) {
            let n = varint_len(v);
            let mut tmp = [0u8; 8];
            for k in (0..n).rev() {
                let mut b = (v & 0x7f) as u8;
                if k < n - 1 {
                    b |= 0x80;
                }
                tmp[k] = b;
                v >>= 7;
            }
            out.extend_from_slice(&tmp[..n]);
        } else {
            let mut tmp = [0u8; 9];
            tmp[8] = (v & 0xff) as u8;
            v >>= 8;
            for k in (0..8).rev() {
                tmp[k] = ((v & 0x7f) as u8) | 0x80;
                v >>= 7;
            }
            out.extend_from_slice(&tmp);
        }
    }

    fn build_record(serial: &[u64], body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let header_len = 1 + serial.iter().map(|&s| varint_len(s)).sum::<usize>();
        put_varint(header_len as u64, &mut out);
        for &s in serial {
            put_varint(s, &mut out);
        }
        out.extend_from_slice(body);
        out
    }

    fn leaf_cell(rowid: u64, payload: &[u8]) -> Vec<u8> {
        let mut cell = Vec::new();
        put_varint(payload.len() as u64, &mut cell);
        put_varint(rowid, &mut cell);
        cell.extend_from_slice(payload);
        cell
    }

    fn leaf_page(cells: &[Vec<u8>], base: usize) -> Vec<u8> {
        let mut page = vec![0u8; PAGE];
        page[base] = 0x0d;
        let n = cells.len();
        page[base + 3] = (n >> 8) as u8;
        page[base + 4] = n as u8;
        let mut ptrs = vec![0usize; n];
        let mut end = PAGE;
        for i in 0..n {
            end -= cells[i].len();
            ptrs[i] = end;
        }
        page[base + 5] = (end >> 8) as u8;
        page[base + 6] = end as u8;
        for i in 0..n {
            page[base + 8 + i * 2] = (ptrs[i] >> 8) as u8;
            page[base + 8 + i * 2 + 1] = ptrs[i] as u8;
        }
        for i in 0..n {
            page[ptrs[i]..ptrs[i] + cells[i].len()].copy_from_slice(&cells[i]);
        }
        page
    }

    fn synthetic_db() -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(b"table");
        body.extend_from_slice(b"t");
        body.extend_from_slice(b"t");
        body.push(2u8);
        let master_record = build_record(&[23, 15, 15, 1, 0], &body);
        let master_cell = leaf_cell(1, &master_record);

        let mut tbody = Vec::new();
        tbody.push(1u8);
        tbody.extend_from_slice(b"x");
        tbody.extend_from_slice(&2.5f64.to_be_bytes());
        let t_record = build_record(&[1, 15, 7], &tbody);
        let t_cell = leaf_cell(1, &t_record);

        let mut file = vec![0u8; PAGE * 2];
        file[0..PAGE].copy_from_slice(&leaf_page(&[master_cell], 100));
        file[PAGE..PAGE * 2].copy_from_slice(&leaf_page(&[t_cell], 0));

        file[0..16].copy_from_slice(b"SQLite format 3\0");
        file[16] = 0x02;
        file[17] = 0x00;
        file[28..32].copy_from_slice(&2u32.to_be_bytes());
        file[44..48].copy_from_slice(&4u32.to_be_bytes());
        file[56..60].copy_from_slice(&1u32.to_be_bytes());
        file
    }

    #[test]
    fn reads_synthetic_sqlite_tables_and_rows() {
        let db = SqliteDb::from_bytes(synthetic_db()).unwrap();
        assert_eq!(db.table_names(), vec!["t".to_string()]);
        let rows = db.read_table("t").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0],
            vec![
                SqliteValue::Int(1),
                SqliteValue::Text("x".to_string()),
                SqliteValue::Real(2.5),
            ]
        );
    }

    #[test]
    fn read_sqlite_path_round_trips() {
        let path =
            std::env::temp_dir().join(format!("omegaflow_gpkg_{}.sqlite", std::process::id()));
        let _ = std::fs::remove_file(&path);
        std::fs::write(&path, synthetic_db()).unwrap();
        let db = read_sqlite(path.to_str().unwrap()).unwrap();
        assert_eq!(db.table_names(), vec!["t".to_string()]);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn rejects_non_sqlite_bytes() {
        assert!(SqliteDb::from_bytes(b"not a sqlite file".to_vec()).is_none());
        assert!(SqliteDb::from_bytes(Vec::new()).is_none());
    }
}
