pub const MAGIC: [u8; 4] = *b"OSM2";
pub const FORMAT: &str = "osm_nodes";

const NANO: f64 = 1e-9;

struct Pb<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Pb<'a> {
    fn new(data: &'a [u8]) -> Self {
        Pb { data, pos: 0 }
    }

    fn eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn varint(&mut self) -> Option<u64> {
        let mut out = 0u64;
        let mut shift = 0u32;
        loop {
            let b = *self.data.get(self.pos)?;
            self.pos += 1;
            out |= ((b & 0x7f) as u64) << shift;
            if b & 0x80 == 0 {
                return Some(out);
            }
            shift += 7;
            if shift >= 64 {
                return None;
            }
        }
    }

    fn key(&mut self) -> Option<(u32, u8)> {
        let k = self.varint()?;
        Some(((k >> 3) as u32, (k & 7) as u8))
    }

    fn bytes(&mut self) -> Option<&'a [u8]> {
        let len = self.varint()? as usize;
        let s = self.data.get(self.pos..self.pos.checked_add(len)?)?;
        self.pos += len;
        Some(s)
    }

    fn skip(&mut self, wire: u8) -> Option<()> {
        match wire {
            0 => {
                self.varint()?;
            }
            1 => {
                self.pos = self.pos.checked_add(8)?;
            }
            2 => {
                let len = self.varint()? as usize;
                self.pos = self.pos.checked_add(len)?;
            }
            5 => {
                self.pos = self.pos.checked_add(4)?;
            }
            _ => return None,
        }
        if self.pos > self.data.len() {
            return None;
        }
        Some(())
    }
}

fn zigzag(v: u64) -> i64 {
    ((v >> 1) as i64) ^ -((v & 1) as i64)
}

fn packed_sint(p: &mut Pb) -> Option<Vec<i64>> {
    let raw = p.bytes()?;
    let mut q = Pb::new(raw);
    let mut out = Vec::new();
    while !q.eof() {
        out.push(zigzag(q.varint()?));
    }
    Some(out)
}

fn zlib_inflate(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 6 {
        return None;
    }
    let cmf = data[0];
    let flg = data[1];
    if cmf & 0x0f != 8 {
        return None;
    }
    if ((cmf as u16) << 8 | flg as u16) % 31 != 0 {
        return None;
    }
    let mut start = 2usize;
    if flg & 0x20 != 0 {
        start += 4;
    }
    let end = data.len().checked_sub(4)?;
    if start >= end {
        return None;
    }
    super::inflate::inflate(data.get(start..end)?)
}

fn next_blob<'a>(data: &'a [u8], pos: &mut usize) -> Option<(String, &'a [u8])> {
    let hlen = u32::from_be_bytes(data.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
    let hstart = *pos + 4;
    let hend = hstart.checked_add(hlen)?;
    let header = data.get(hstart..hend)?;
    let mut p = Pb::new(header);
    let mut btype: Option<String> = None;
    let mut datasize: Option<usize> = None;
    while !p.eof() {
        let (field, wire) = p.key()?;
        match field {
            1 => btype = Some(std::str::from_utf8(p.bytes()?).ok()?.to_string()),
            3 => datasize = Some(p.varint()? as usize),
            _ => p.skip(wire)?,
        }
    }
    let datasize = datasize?;
    let blob = data.get(hend..hend.checked_add(datasize)?)?;
    *pos = hend + datasize;
    Some((btype?, blob))
}

fn blob_payload(blob: &[u8]) -> Option<Vec<u8>> {
    let mut p = Pb::new(blob);
    let mut raw: Option<&[u8]> = None;
    let mut zlib: Option<&[u8]> = None;
    while !p.eof() {
        let (field, wire) = p.key()?;
        match field {
            1 => raw = Some(p.bytes()?),
            2 => {
                p.varint()?;
            }
            3 => zlib = Some(p.bytes()?),
            _ => p.skip(wire)?,
        }
    }
    if let Some(r) = raw {
        return Some(r.to_vec());
    }
    zlib.and_then(zlib_inflate)
}

fn coordinate(lat: i64, lon: i64, gran: i64, lat_off: i64, lon_off: i64) -> (f64, f64) {
    let la = ((gran as i128) * (lat as i128) + (lat_off as i128)) as f64 * NANO;
    let lo = ((gran as i128) * (lon as i128) + (lon_off as i128)) as f64 * NANO;
    (la, lo)
}

fn dense_nodes(
    dense: &[u8],
    gran: i64,
    lat_off: i64,
    lon_off: i64,
    out: &mut Vec<(f64, f64)>,
) -> Option<()> {
    let mut p = Pb::new(dense);
    let mut lats: Vec<i64> = Vec::new();
    let mut lons: Vec<i64> = Vec::new();
    while !p.eof() {
        let (field, wire) = p.key()?;
        match field {
            8 => lats = packed_sint(&mut p)?,
            9 => lons = packed_sint(&mut p)?,
            _ => p.skip(wire)?,
        }
    }
    if lats.len() != lons.len() {
        return None;
    }
    let mut lat = 0i64;
    let mut lon = 0i64;
    for (&dlat, &dlon) in lats.iter().zip(lons.iter()) {
        lat = lat.wrapping_add(dlat);
        lon = lon.wrapping_add(dlon);
        out.push(coordinate(lat, lon, gran, lat_off, lon_off));
    }
    Some(())
}

fn node_coordinate(node: &[u8], gran: i64, lat_off: i64, lon_off: i64) -> Option<(f64, f64)> {
    let mut p = Pb::new(node);
    let mut lat: Option<i64> = None;
    let mut lon: Option<i64> = None;
    while !p.eof() {
        let (field, wire) = p.key()?;
        match field {
            8 => lat = Some(zigzag(p.varint()?)),
            9 => lon = Some(zigzag(p.varint()?)),
            _ => p.skip(wire)?,
        }
    }
    Some(coordinate(lat?, lon?, gran, lat_off, lon_off))
}

fn primitive_group(
    group: &[u8],
    gran: i64,
    lat_off: i64,
    lon_off: i64,
    out: &mut Vec<(f64, f64)>,
) -> Option<()> {
    let mut p = Pb::new(group);
    while !p.eof() {
        let (field, wire) = p.key()?;
        match field {
            1 => {
                let node = p.bytes()?;
                if let Some(c) = node_coordinate(node, gran, lat_off, lon_off) {
                    out.push(c);
                }
            }
            2 => {
                let dense = p.bytes()?;
                dense_nodes(dense, gran, lat_off, lon_off, out)?;
            }
            _ => p.skip(wire)?,
        }
    }
    Some(())
}

fn primitive_block(block: &[u8], out: &mut Vec<(f64, f64)>) -> Option<()> {
    let mut p = Pb::new(block);
    let mut gran = 100i64;
    let mut lat_off = 0i64;
    let mut lon_off = 0i64;
    let mut groups: Vec<&[u8]> = Vec::new();
    while !p.eof() {
        let (field, wire) = p.key()?;
        match field {
            2 => groups.push(p.bytes()?),
            17 => gran = p.varint()? as i64,
            19 => lat_off = p.varint()? as i64,
            20 => lon_off = p.varint()? as i64,
            _ => p.skip(wire)?,
        }
    }
    for group in groups {
        primitive_group(group, gran, lat_off, lon_off, out)?;
    }
    Some(())
}

pub fn parse_nodes(bytes: &[u8]) -> Option<Vec<(f64, f64)>> {
    let mut pos = 0usize;
    let mut out = Vec::new();
    let mut saw_data = false;
    while pos < bytes.len() {
        let (btype, blob) = next_blob(bytes, &mut pos)?;
        if btype != "OSMData" {
            continue;
        }
        saw_data = true;
        let payload = blob_payload(blob)?;
        primitive_block(&payload, &mut out)?;
    }
    if !saw_data {
        return None;
    }
    Some(out)
}

pub fn parse_timestamp(bytes: &[u8]) -> Option<f64> {
    let mut pos = 0usize;
    while pos < bytes.len() {
        let (btype, blob) = next_blob(bytes, &mut pos)?;
        if btype != "OSMHeader" {
            continue;
        }
        let payload = blob_payload(blob)?;
        let mut p = Pb::new(&payload);
        while !p.eof() {
            let (field, wire) = p.key()?;
            if field == 32 {
                return Some(p.varint()? as f64);
            }
            p.skip(wire)?;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn varint(mut v: u64, out: &mut Vec<u8>) {
        while v >= 0x80 {
            out.push((v as u8) | 0x80);
            v >>= 7;
        }
        out.push(v as u8);
    }

    fn key(field: u32, wire: u8, out: &mut Vec<u8>) {
        varint(((field as u64) << 3) | wire as u64, out);
    }

    fn packed(field: u32, vals: &[i64], out: &mut Vec<u8>) {
        key(field, 2, out);
        let mut body = Vec::new();
        for &v in vals {
            let z = ((v << 1) ^ (v >> 63)) as u64;
            varint(z, &mut body);
        }
        varint(body.len() as u64, out);
        out.extend_from_slice(&body);
    }

    fn adler32(data: &[u8]) -> u32 {
        let mut a = 1u32;
        let mut b = 0u32;
        for &byte in data {
            a = (a + byte as u32) % 65521;
            b = (b + a) % 65521;
        }
        (b << 16) | a
    }

    fn zlib_store(data: &[u8]) -> Vec<u8> {
        let mut out = vec![0x78, 0x01];
        out.push(0x01);
        let len = data.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(data);
        out.extend_from_slice(&adler32(data).to_be_bytes());
        out
    }

    fn frame(btype: &str, blob: &[u8]) -> Vec<u8> {
        let mut hdr = Vec::new();
        key(1, 2, &mut hdr);
        varint(btype.len() as u64, &mut hdr);
        hdr.extend_from_slice(btype.as_bytes());
        key(3, 0, &mut hdr);
        varint(blob.len() as u64, &mut hdr);
        let mut out = Vec::new();
        out.extend_from_slice(&(hdr.len() as u32).to_be_bytes());
        out.extend_from_slice(&hdr);
        out.extend_from_slice(blob);
        out
    }

    fn blob_raw(payload: &[u8]) -> Vec<u8> {
        let mut b = Vec::new();
        key(2, 0, &mut b);
        varint(payload.len() as u64, &mut b);
        key(1, 2, &mut b);
        varint(payload.len() as u64, &mut b);
        b.extend_from_slice(payload);
        b
    }

    fn blob_zlib(payload: &[u8]) -> Vec<u8> {
        let z = zlib_store(payload);
        let mut b = Vec::new();
        key(2, 0, &mut b);
        varint(payload.len() as u64, &mut b);
        key(3, 2, &mut b);
        varint(z.len() as u64, &mut b);
        b.extend_from_slice(&z);
        b
    }

    fn dense_block() -> Vec<u8> {
        let mut dense = Vec::new();
        packed(1, &[1, 2], &mut dense);
        packed(8, &[470_000_000, 1_000_000], &mut dense);
        packed(9, &[70_000_000, 1_000_000], &mut dense);

        let mut group = Vec::new();
        key(2, 2, &mut group);
        varint(dense.len() as u64, &mut group);
        group.extend_from_slice(&dense);

        let mut block = Vec::new();
        key(1, 2, &mut block);
        varint(0, &mut block);
        key(2, 2, &mut block);
        varint(group.len() as u64, &mut block);
        block.extend_from_slice(&group);
        block
    }

    fn dense_pbf(blob: fn(&[u8]) -> Vec<u8>) -> Vec<u8> {
        let mut out = frame("OSMData", &blob(&dense_block()));
        let mut header = Vec::new();
        key(32, 0, &mut header);
        varint(1_700_000_000, &mut header);
        let mut header_stream = frame("OSMHeader", &blob(&header));
        header_stream.extend_from_slice(&out);
        out = header_stream;
        out
    }

    #[test]
    fn parses_dense_nodes_in_order() {
        for blob in [blob_raw as fn(&[u8]) -> Vec<u8>, blob_zlib] {
            let bytes = dense_pbf(blob);
            let nodes = parse_nodes(&bytes).unwrap();
            assert_eq!(nodes.len(), 2);
            assert!((nodes[0].0 - 47.0).abs() < 1e-9);
            assert!((nodes[0].1 - 7.0).abs() < 1e-9);
            assert!((nodes[1].0 - 47.1).abs() < 1e-9);
            assert!((nodes[1].1 - 7.1).abs() < 1e-9);
        }
    }

    #[test]
    fn reads_the_header_timestamp() {
        let bytes = dense_pbf(blob_raw);
        assert_eq!(parse_timestamp(&bytes), Some(1_700_000_000.0));
    }

    #[test]
    fn absence_is_none() {
        assert!(parse_nodes(b"").is_none());
        assert!(parse_nodes(b"not a pbf").is_none());
        assert!(parse_timestamp(b"not a pbf").is_none());
    }
}
