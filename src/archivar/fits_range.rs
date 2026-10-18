
use std::collections::HashMap;

use super::cdn::{PartsManifest, parse_parts_manifest};
use super::fits::{FitsHeader, FitsTable, FitsValue};
use super::range::fetch_range;
use super::sha256::sha256_hex;

const BLOCK: u64 = 2880;

pub trait ByteSource {
    fn read_range(&mut self, offset: u64, len: u64) -> Option<Vec<u8>>;
}

pub struct HttpRangeSource {
    url: String,
    headers: Vec<(String, String)>,
}

impl HttpRangeSource {
    pub fn new(url: &str, headers: &[(String, String)]) -> Self {
        Self {
            url: url.to_string(),
            headers: headers.to_vec(),
        }
    }
}

impl ByteSource for HttpRangeSource {
    fn read_range(&mut self, offset: u64, len: u64) -> Option<Vec<u8>> {
        let bytes = fetch_range(&self.url, offset, len, &self.headers)?;
        if bytes.len() as u64 == len {
            Some(bytes)
        } else if offset == 0 && bytes.len() as u64 > len {
            Some(bytes[..len as usize].to_vec())
        } else {
            None
        }
    }
}

pub struct PartsSource {
    base: String,
    headers: Vec<(String, String)>,
    manifest: PartsManifest,
    offsets: Vec<u64>,
    cache: HashMap<usize, Vec<u8>>,
    fetched_shards: usize,
}

impl PartsSource {
    pub fn new(base: &str, headers: &[(String, String)], manifest: PartsManifest) -> Self {
        let mut offsets = Vec::with_capacity(manifest.parts.len());
        let mut acc = 0u64;
        for (_, _, len) in &manifest.parts {
            offsets.push(acc);
            acc += *len;
        }
        Self {
            base: base.trim_end_matches('/').to_string(),
            headers: headers.to_vec(),
            manifest,
            offsets,
            cache: HashMap::new(),
            fetched_shards: 0,
        }
    }

    #[cfg(test)]
    fn with_shards(
        base: &str,
        headers: &[(String, String)],
        manifest: PartsManifest,
        shards: Vec<(usize, Vec<u8>)>,
    ) -> Self {
        let mut source = Self::new(base, headers, manifest);
        for (idx, bytes) in shards {
            if source.verify(idx, &bytes) {
                source.cache.insert(idx, bytes);
            }
        }
        source
    }

    pub fn fetched_shards(&self) -> usize {
        self.fetched_shards
    }

    fn verify(&self, idx: usize, bytes: &[u8]) -> bool {
        let Some((_, sha, len)) = self.manifest.parts.get(idx) else {
            return false;
        };
        bytes.len() as u64 == *len && sha256_hex(bytes) == *sha
    }

    fn shard(&mut self, idx: usize) -> Option<&[u8]> {
        if !self.cache.contains_key(&idx) {
            let (name, _, len) = self.manifest.parts.get(idx)?.clone();
            let body = fetch_range(&format!("{}/{}", self.base, name), 0, len, &self.headers)?;
            if !self.verify(idx, &body) {
                return None;
            }
            self.fetched_shards += 1;
            self.cache.insert(idx, body);
        }
        self.cache.get(&idx).map(|v| v.as_slice())
    }
}

impl ByteSource for PartsSource {
    fn read_range(&mut self, offset: u64, len: u64) -> Option<Vec<u8>> {
        if len == 0 {
            return Some(Vec::new());
        }
        let end = offset.checked_add(len)?;
        let mut idx = self.offsets.iter().rposition(|start| *start <= offset)?;
        let mut out = Vec::with_capacity(len as usize);
        while (out.len() as u64) < len {
            let start = *self.offsets.get(idx)?;
            let shard_len = self.manifest.parts.get(idx)?.2;
            let shard_end = start + shard_len;
            let cursor = offset + out.len() as u64;
            if cursor >= end {
                break;
            }
            if cursor >= shard_end {
                idx += 1;
                continue;
            }
            let local = (cursor - start) as usize;
            let body = self.shard(idx)?;
            let available = body.len().saturating_sub(local);
            let take = available.min((end - cursor) as usize);
            if take == 0 {
                return None;
            }
            out.extend_from_slice(body.get(local..local + take)?);
            idx += 1;
        }
        (out.len() as u64 == len).then_some(out)
    }
}

pub fn parts_source(url: &str, headers: &[(String, String)]) -> Option<PartsSource> {
    let manifest_text = super::fetch::fetch_raw(&format!("{url}.parts"), None, &[])?;
    let manifest = parse_parts_manifest(&manifest_text)?;
    let base = url.rsplit_once('/').map(|(dir, _)| dir)?;
    Some(PartsSource::new(base, headers, manifest))
}

struct HeaderRead {
    header: FitsHeader,
    buf: Vec<u8>,
    data_start: u64,
}

fn data_bytes(header: &FitsHeader) -> Option<u64> {
    let bitpix = header.int("BITPIX")?.unsigned_abs() / 8;
    let naxis = header.int("NAXIS")? as u64;
    let mut axes: u64 = if naxis == 0 { 0 } else { 1 };
    for i in 1..=naxis {
        axes = axes.checked_mul(header.int(&format!("NAXIS{i}"))? as u64)?;
    }
    let gcount = header.int("GCOUNT").unwrap_or(1) as u64;
    let pcount = match header.int("PCOUNT") {
        Some(v) => v as u64,
        None => 0,
    };
    Some(bitpix.checked_mul(axes)?.checked_mul(gcount)? + pcount)
}

pub struct FitsTableSlice {
    table: FitsTable,
    buf: Vec<u8>,
    first_row: usize,
    rows: usize,
}

impl FitsTableSlice {
    pub fn first_row(&self) -> usize {
        self.first_row
    }

    pub fn rows(&self) -> Vec<Vec<FitsValue>> {
        (0..self.rows)
            .filter_map(|i| self.table.row(&self.buf, i))
            .collect()
    }

    pub fn n_rows(&self) -> usize {
        self.rows
    }
}

pub struct FitsRangeReader<S> {
    source: S,
}

impl<S: ByteSource> FitsRangeReader<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }

    pub fn into_source(self) -> S {
        self.source
    }

    fn read_header(&mut self, start: u64) -> Option<HeaderRead> {
        let mut buf = Vec::new();
        for block in 0..64u64 {
            let bytes = self.source.read_range(start + block * BLOCK, BLOCK)?;
            if bytes.len() as u64 != BLOCK {
                return None;
            }
            buf.extend_from_slice(&bytes);
            if let Some((header, data_start)) = FitsHeader::parse(&buf, 0) {
                return Some(HeaderRead {
                    header,
                    buf,
                    data_start: start + data_start as u64,
                });
            }
        }
        None
    }

    fn next_hdu(_start: u64, data_start: u64, header: &FitsHeader) -> Option<u64> {
        let end = data_start.checked_add(data_bytes(header)?)?;
        Some(end.div_ceil(BLOCK) * BLOCK)
    }

    pub fn read_table_rows(
        &mut self,
        hdu: usize,
        start_row: usize,
        count: usize,
    ) -> Option<FitsTableSlice> {
        let mut off = 0u64;
        for i in 0..=hdu {
            let head = self.read_header(off)?;
            if i == hdu {
                if head.header.value("XTENSION") != Some("'BINTABLE'") {
                    return None;
                }
                let (table, _) = FitsTable::parse_lenient(&head.buf, 0)?;
                if start_row.checked_add(count)? > table.n_rows {
                    return None;
                }
                let row_off = head.data_start + (start_row as u64) * (table.row_bytes as u64);
                let row_len = (count as u64) * (table.row_bytes as u64);
                let rows = if row_len == 0 {
                    Vec::new()
                } else {
                    self.source.read_range(row_off, row_len)?
                };
                if rows.len() as u64 != row_len {
                    return None;
                }
                let mut buf = head.buf;
                buf.extend_from_slice(&rows);
                let (table, _) = FitsTable::parse_lenient(&buf, 0)?;
                return Some(FitsTableSlice {
                    table,
                    buf,
                    first_row: start_row,
                    rows: count,
                });
            }
            off = Self::next_hdu(off, head.data_start, &head.header)?;
        }
        None
    }

    pub fn hdu_count(&mut self) -> Option<usize> {
        let mut off = 0u64;
        let mut count = 0usize;
        while count < 4096 {
            let Some(head) = self.read_header(off) else {
                break;
            };
            count += 1;
            match Self::next_hdu(off, head.data_start, &head.header) {
                Some(next) if next > off => off = next,
                _ => break,
            }
        }
        (count > 0).then_some(count)
    }
}

pub fn fetch_table_rows(
    url: &str,
    headers: &[(String, String)],
    hdu: usize,
    start_row: usize,
    count: usize,
) -> Option<FitsTableSlice> {
    let mut direct = FitsRangeReader::new(HttpRangeSource::new(url, headers));
    if let Some(slice) = direct.read_table_rows(hdu, start_row, count) {
        return Some(slice);
    }
    let source = parts_source(url, headers)?;
    FitsRangeReader::new(source).read_table_rows(hdu, start_row, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(kw: &str, value: &str) -> [u8; 80] {
        let mut c = [b' '; 80];
        let k = kw.as_bytes();
        c[..k.len().min(8)].copy_from_slice(&k[..k.len().min(8)]);
        c[8] = b'=';
        let v = value.as_bytes();
        c[10..10 + v.len().min(20)].copy_from_slice(&v[..v.len().min(20)]);
        c
    }

    fn header_cards(cards: &[(&str, &str)]) -> Vec<u8> {
        let mut h = Vec::new();
        for (k, v) in cards {
            h.extend_from_slice(&card(k, v));
        }
        while !h.len().is_multiple_of(BLOCK as usize) {
            h.extend_from_slice(&[b' '; 80]);
        }
        h
    }

    fn fixture() -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&header_cards(&[
            ("SIMPLE", "T"),
            ("BITPIX", "8"),
            ("NAXIS", "0"),
            ("END", ""),
        ]));
        buf.extend_from_slice(&header_cards(&[
            ("XTENSION", "'BINTABLE'"),
            ("BITPIX", "8"),
            ("NAXIS", "2"),
            ("NAXIS1", "17"),
            ("NAXIS2", "3"),
            ("PCOUNT", "0"),
            ("GCOUNT", "1"),
            ("TFIELDS", "4"),
            ("TTYPE1", "'FLUX'"),
            ("TFORM1", "D"),
            ("TTYPE2", "'ID'"),
            ("TFORM2", "J"),
            ("TTYPE3", "'NAME'"),
            ("TFORM3", "4A"),
            ("TTYPE4", "'OK'"),
            ("TFORM4", "L"),
            ("END", ""),
        ]));
        for i in 0..3i32 {
            buf.extend_from_slice(&(40.0 + i as f64).to_be_bytes());
            buf.extend_from_slice(&(i + 1).to_be_bytes());
            buf.extend_from_slice(format!("R{i}  ").as_bytes());
            buf.push(if i == 1 { b'F' } else { b'T' });
        }
        while buf.len() % BLOCK as usize != 0 {
            buf.push(0);
        }
        buf
    }

    struct MemSource {
        bytes: Vec<u8>,
        log: Vec<(u64, u64)>,
    }

    impl MemSource {
        fn new(bytes: Vec<u8>) -> Self {
            Self {
                bytes,
                log: Vec::new(),
            }
        }
    }

    impl ByteSource for MemSource {
        fn read_range(&mut self, offset: u64, len: u64) -> Option<Vec<u8>> {
            self.log.push((offset, len));
            let start = offset as usize;
            let end = start.checked_add(len as usize)?;
            self.bytes.get(start..end).map(|s| s.to_vec())
        }
    }

    #[test]
    fn reads_only_the_requested_rows() {
        let file = fixture();
        let mut reader = FitsRangeReader::new(MemSource::new(file.clone()));
        let slice = reader.read_table_rows(1, 1, 2).expect("rows 1..3");
        assert_eq!(slice.first_row(), 1);
        assert_eq!(slice.n_rows(), 2);
        let rows = slice.rows();
        assert_eq!(rows[0][0], FitsValue::Float(41.0));
        assert_eq!(rows[1][2], FitsValue::Str("R2".into()));

        let (_, data_start) = FitsHeader::parse(&file, BLOCK as usize).unwrap();
        let row0 = data_start as u64;
        let selected = row0 + 17;
        let source = reader.into_source();
        assert!(
            !source.log.iter().any(|(off, len)| *off <= row0 && off + len > row0),
            "row 0 is never fetched: {:?}",
            source.log
        );
        assert!(
            source.log.iter().any(|(off, _)| *off == selected),
            "selected row span starts at row 1: {:?}",
            source.log
        );
        assert!(source.log.iter().map(|(_, l)| *l).sum::<u64>() < file.len() as u64);
    }

    #[test]
    fn sharded_source_verifies_and_slices_across_the_boundary() {
        let file = fixture();
        let (_, data_start) = FitsHeader::parse(&file, BLOCK as usize).unwrap();
        let cut = data_start + 20;
        let shards = vec![file[..cut].to_vec(), file[cut..].to_vec()];
        let manifest = PartsManifest {
            total_sha256: sha256_hex(&file),
            parts: shards
                .iter()
                .enumerate()
                .map(|(i, s)| (format!("asset.p{i:03}"), sha256_hex(s), s.len() as u64))
                .collect(),
        };
        let mut source = PartsSource::with_shards(
            "https://example.invalid/asset",
            &[],
            manifest,
            shards.into_iter().enumerate().collect(),
        );
        assert_eq!(
            source.read_range(cut as u64 - 5, 10).unwrap(),
            file[cut - 5..cut + 5]
        );
        assert_eq!(
            source.read_range(cut as u64 - 5, 10).unwrap(),
            file[cut - 5..cut + 5]
        );
    }

    #[test]
    fn sharded_source_rejects_a_tampered_shard() {
        let file = fixture();
        let cut = 100;
        let first = file[..cut].to_vec();
        let second = file[cut..].to_vec();
        let manifest = PartsManifest {
            total_sha256: sha256_hex(&file),
            parts: vec![
                ("asset.p000".into(), sha256_hex(&first), first.len() as u64),
                ("asset.p001".into(), sha256_hex(&second), second.len() as u64),
            ],
        };
        let mut tampered = first.clone();
        tampered[60] ^= 0xff;
        let mut source = PartsSource::with_shards(
            "https://example.invalid/asset",
            &[],
            manifest,
            vec![(0, tampered), (1, second)],
        );
        assert!(
            source.read_range(0, 10).is_none(),
            "the sha256 gate refuses a tampered shard"
        );
    }

    #[test]
    fn walking_hdus_uses_headers_only() {
        let file = fixture();
        let mut reader = FitsRangeReader::new(MemSource::new(file));
        assert_eq!(reader.hdu_count(), Some(2));
        let source = reader.into_source();
        assert!(
            source.log.iter().all(|(_, len)| *len == BLOCK),
            "HDU walk fetches 2880-byte header blocks only: {:?}",
            source.log
        );
    }
}
