// netCDF-3-Reader (CDF-1 + CDF-2), std-only, Big-Endian (XDR).
// Der Struktur-Atom für SPHEREx/DESI/GLODAP (siehe TODO.md, Struktur-Reader).
// CDF-5 bleibt pending, netCDF-4/HDF5 ist ein eigener Atom. Kein HDF5.
//
// Format nach der Classic-Spec (Unidata, Appendix B):
//   magic CDF\x01 (u32-Offsets) | CDF\x02 (u64-Offsets),
//   numrecs (u32, 0xFFFFFFFF = STREAMING), dim-/gatt-/var-Listen,
//   Namen und Attributwerte auf 4 Byte gepaddet, Offsets = Byte-Offsets,
//   Daten row-major, Record-Variablen interleaved am Dateiende.
//   nc_type: 1 BYTE, 2 CHAR, 3 SHORT, 4 INT, 5 FLOAT, 6 DOUBLE.
//   _FillValue bleibt als Attribut sichtbar (0 honored — nie ersetzt).

const MAGIC: [u8; 3] = [b'C', b'D', b'F'];
const STREAMING: u32 = 0xFFFF_FFFF;
const TAG_DIMENSION: u32 = 0x0A;
const TAG_VARIABLE: u32 = 0x0B;
const TAG_ATTRIBUTE: u32 = 0x0C;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NetcdfFormat {
    Cdf1,
    Cdf2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NetcdfType {
    Byte,
    Char,
    Short,
    Int,
    Float,
    Double,
}

impl NetcdfType {
    pub fn from_tag(tag: u32) -> Option<NetcdfType> {
        match tag {
            1 => Some(NetcdfType::Byte),
            2 => Some(NetcdfType::Char),
            3 => Some(NetcdfType::Short),
            4 => Some(NetcdfType::Int),
            5 => Some(NetcdfType::Float),
            6 => Some(NetcdfType::Double),
            _ => None,
        }
    }

    pub fn size(self) -> usize {
        match self {
            NetcdfType::Byte | NetcdfType::Char => 1,
            NetcdfType::Short => 2,
            NetcdfType::Int | NetcdfType::Float => 4,
            NetcdfType::Double => 8,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            NetcdfType::Byte => "byte",
            NetcdfType::Char => "char",
            NetcdfType::Short => "short",
            NetcdfType::Int => "int",
            NetcdfType::Float => "float",
            NetcdfType::Double => "double",
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetcdfDim {
    pub name: String,
    pub len: u32,
}

#[derive(Clone, Debug)]
pub struct NetcdfAttr {
    pub name: String,
    pub nc_type: NetcdfType,
    pub raw: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct NetcdfVar {
    pub name: String,
    pub dim_ids: Vec<usize>,
    pub attrs: Vec<NetcdfAttr>,
    pub nc_type: NetcdfType,
    pub vsize: u32,
    pub begin: u64,
}

#[derive(Clone, Debug)]
pub struct NetcdfFile {
    pub format: NetcdfFormat,
    pub numrecs: Option<u32>,
    pub dims: Vec<NetcdfDim>,
    pub gattrs: Vec<NetcdfAttr>,
    pub vars: Vec<NetcdfVar>,
}

#[derive(Clone, Debug)]
pub enum NetcdfNote {
    Magie { bytes: [u8; 4] },
    Cdf5,
    EndeBeiByte { off: usize },
    Typ { tag: u32, off: usize },
    Tag { tag: u32, off: usize },
    DimId { id: u32 },
    AnzahlOffen,
    KeineVariable { name: String },
    Slab { var: String },
}

fn be_u32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn be_u64(b: &[u8]) -> u64 {
    u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

struct Kur<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Kur<'a> {
    fn new(buf: &'a [u8]) -> Kur<'a> {
        Kur { buf, pos: 0 }
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.pos + n <= self.buf.len() {
            let s = &self.buf[self.pos..self.pos + n];
            self.pos += n;
            Some(s)
        } else {
            None
        }
    }

    fn u32(&mut self) -> Option<u32> {
        Some(be_u32(self.take(4)?))
    }

    fn u64(&mut self) -> Option<u64> {
        Some(be_u64(self.take(8)?))
    }

    fn align4(&mut self) -> bool {
        let next = (self.pos + 3) & !3;
        if next <= self.buf.len() {
            self.pos = next;
            true
        } else {
            false
        }
    }

    fn name(&mut self) -> Result<String, NetcdfNote> {
        let len = self
            .u32()
            .ok_or(NetcdfNote::EndeBeiByte { off: self.pos })? as usize;
        let bytes = self
            .take(len)
            .ok_or(NetcdfNote::EndeBeiByte { off: self.pos })?;
        if !self.align4() {
            return Err(NetcdfNote::EndeBeiByte { off: self.pos });
        }
        Ok(String::from_utf8_lossy(bytes).into_owned())
    }
}

fn attr(cur: &mut Kur) -> Result<NetcdfAttr, NetcdfNote> {
    let name = cur.name()?;
    let off = cur.pos;
    let tag = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off })?;
    let nc_type = NetcdfType::from_tag(tag).ok_or(NetcdfNote::Typ { tag, off })?;
    let nelems = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })? as usize;
    let raw_len = nc_type
        .size()
        .checked_mul(nelems)
        .ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })?;
    let raw = cur
        .take(raw_len)
        .ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })?
        .to_vec();
    if !cur.align4() {
        return Err(NetcdfNote::EndeBeiByte { off: cur.pos });
    }
    Ok(NetcdfAttr { name, nc_type, raw })
}

fn attr_list(cur: &mut Kur) -> Result<Vec<NetcdfAttr>, NetcdfNote> {
    let off = cur.pos;
    let tag = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off })?;
    let nelems = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })? as usize;
    match tag {
        0 => Ok(Vec::new()),
        TAG_ATTRIBUTE => {
            let mut out = Vec::with_capacity(nelems.min(1024));
            for _ in 0..nelems {
                out.push(attr(cur)?);
            }
            Ok(out)
        }
        _ => Err(NetcdfNote::Tag { tag, off }),
    }
}

impl NetcdfFile {
    pub fn parse(bytes: &[u8]) -> Result<NetcdfFile, NetcdfNote> {
        if bytes.len() < 4 {
            return Err(NetcdfNote::EndeBeiByte { off: bytes.len() });
        }
        let mut mag = [0u8; 4];
        mag.copy_from_slice(&bytes[0..4]);
        let format = if mag[0..3] == MAGIC {
            match mag[3] {
                1 => NetcdfFormat::Cdf1,
                2 => NetcdfFormat::Cdf2,
                5 => return Err(NetcdfNote::Cdf5),
                _ => return Err(NetcdfNote::Magie { bytes: mag }),
            }
        } else {
            return Err(NetcdfNote::Magie { bytes: mag });
        };

        let mut cur = Kur::new(bytes);
        cur.take(4);
        let numrecs_raw = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })?;
        let numrecs = if numrecs_raw == STREAMING {
            None
        } else {
            Some(numrecs_raw)
        };

        let dims = dim_list(&mut cur)?;
        let gattrs = attr_list(&mut cur)?;
        let vars = var_list(&mut cur, format)?;

        Ok(NetcdfFile {
            format,
            numrecs,
            dims,
            gattrs,
            vars,
        })
    }

    pub fn var(&self, name: &str) -> Option<&NetcdfVar> {
        self.vars.iter().find(|v| v.name == name)
    }

    pub fn var_index(&self, name: &str) -> Option<usize> {
        self.vars.iter().position(|v| v.name == name)
    }

    pub fn gattr(&self, name: &str) -> Option<&NetcdfAttr> {
        self.gattrs.iter().find(|a| a.name == name)
    }

    pub fn record_var(&self, var: &NetcdfVar) -> bool {
        match var.dim_ids.first() {
            Some(&id) => self.dims.get(id).map_or(false, |d| d.len == 0),
            None => false,
        }
    }

    pub fn var_shape(&self, var: &NetcdfVar) -> Result<Vec<u64>, NetcdfNote> {
        let mut sh = Vec::with_capacity(var.dim_ids.len());
        for (i, &id) in var.dim_ids.iter().enumerate() {
            let d = self
                .dims
                .get(id)
                .ok_or(NetcdfNote::DimId { id: id as u32 })?;
            let len = d.len as u64;
            if len == 0 && i == 0 {
                sh.push(self.numrecs.ok_or(NetcdfNote::AnzahlOffen)? as u64);
            } else {
                sh.push(len);
            }
        }
        Ok(sh)
    }

    pub fn read_var(
        &self,
        file: &[u8],
        name: &str,
        start: &[usize],
        count: &[usize],
    ) -> Result<Vec<u8>, NetcdfNote> {
        let idx = self
            .var_index(name)
            .ok_or_else(|| NetcdfNote::KeineVariable {
                name: name.to_string(),
            })?;
        self.read_slab(file, idx, start, count)
    }

    pub fn read_var_index(
        &self,
        file: &[u8],
        idx: usize,
        start: &[usize],
        count: &[usize],
    ) -> Result<Vec<u8>, NetcdfNote> {
        self.read_slab(file, idx, start, count)
    }

    fn read_slab(
        &self,
        file: &[u8],
        idx: usize,
        start: &[usize],
        count: &[usize],
    ) -> Result<Vec<u8>, NetcdfNote> {
        let v = self.vars.get(idx).ok_or(NetcdfNote::KeineVariable {
            name: idx.to_string(),
        })?;
        let rank = v.dim_ids.len();
        if start.len() != rank || count.len() != rank {
            return Err(NetcdfNote::Slab {
                var: v.name.clone(),
            });
        }
        let shape = self.var_shape(v)?;
        for a in 0..rank {
            let end = (start[a] as u64)
                .checked_add(count[a] as u64)
                .ok_or_else(|| NetcdfNote::Slab {
                    var: v.name.clone(),
                })?;
            if end > shape[a] {
                return Err(NetcdfNote::Slab {
                    var: v.name.clone(),
                });
            }
        }

        let tsize = v.nc_type.size() as u64;
        let total: u64 = count.iter().map(|&c| c as u64).product();
        if total == 0 {
            return Ok(Vec::new());
        }
        let total_bytes = total.checked_mul(tsize).ok_or_else(|| NetcdfNote::Slab {
            var: v.name.clone(),
        })?;
        let mut out = Vec::with_capacity(total_bytes.min(file.len() as u64) as usize);

        let mut prod = vec![0u64; rank + 1];
        prod[rank] = 1;
        for i in (0..rank).rev() {
            prod[i] = prod[i + 1].saturating_mul(shape[i]);
        }

        let is_rec = self.record_var(v);
        let recsize = if is_rec {
            Some(self.record_size())
        } else {
            None
        };

        let mut coord = start.to_vec();
        loop {
            let mut within: u64 = 0;
            if is_rec {
                for i in 1..rank {
                    within += coord[i] as u64 * prod[i + 1];
                }
            } else {
                for i in 0..rank {
                    within += coord[i] as u64 * prod[i + 1];
                }
            }
            let off = match recsize {
                Some(rs) => v
                    .begin
                    .checked_add(coord[0] as u64 * rs)
                    .and_then(|x| x.checked_add(within * tsize)),
                None => v.begin.checked_add(within * tsize),
            }
            .ok_or_else(|| NetcdfNote::Slab {
                var: v.name.clone(),
            })?;

            let off = off as usize;
            let end = off + tsize as usize;
            if end > file.len() {
                return Err(NetcdfNote::EndeBeiByte { off: file.len() });
            }
            out.extend_from_slice(&file[off..end]);

            let mut carry = true;
            let mut a = rank;
            while carry && a > 0 {
                a -= 1;
                coord[a] += 1;
                if coord[a] < start[a] + count[a] {
                    carry = false;
                } else {
                    coord[a] = start[a];
                }
            }
            if carry {
                break;
            }
        }
        Ok(out)
    }

    fn nonrecord_elems(&self, var: &NetcdfVar) -> u64 {
        let mut e = 1u64;
        for &id in &var.dim_ids[1..] {
            if let Some(d) = self.dims.get(id) {
                e = e.saturating_mul(d.len as u64);
            }
        }
        e
    }

    fn record_size(&self) -> u64 {
        let mut single = 0u64;
        for v in &self.vars {
            if self.record_var(v) {
                single += 1;
            }
        }
        let mut recsize = 0u64;
        for v in &self.vars {
            if !self.record_var(v) {
                continue;
            }
            let raw = self.nonrecord_elems(v) * v.nc_type.size() as u64;
            let stride = if single == 1
                && matches!(
                    v.nc_type,
                    NetcdfType::Byte | NetcdfType::Char | NetcdfType::Short
                ) {
                raw
            } else {
                (raw + 3) & !3
            };
            recsize += stride;
        }
        recsize
    }

    pub fn values_i8(&self, file: &[u8], name: &str) -> Option<Vec<i8>> {
        let idx = self.var_index(name)?;
        if self.vars[idx].nc_type != NetcdfType::Byte {
            return None;
        }
        let raw = self.full_slab(file, idx)?;
        Some(raw.into_iter().map(|b| b as i8).collect())
    }

    pub fn values_i16(&self, file: &[u8], name: &str) -> Option<Vec<i16>> {
        let idx = self.var_index(name)?;
        if self.vars[idx].nc_type != NetcdfType::Short {
            return None;
        }
        let raw = self.full_slab(file, idx)?;
        Some(
            raw.chunks_exact(2)
                .map(|c| i16::from_be_bytes([c[0], c[1]]))
                .collect(),
        )
    }

    pub fn values_i32(&self, file: &[u8], name: &str) -> Option<Vec<i32>> {
        let idx = self.var_index(name)?;
        if self.vars[idx].nc_type != NetcdfType::Int {
            return None;
        }
        let raw = self.full_slab(file, idx)?;
        Some(
            raw.chunks_exact(4)
                .map(|c| i32::from_be_bytes([c[0], c[1], c[2], c[3]]))
                .collect(),
        )
    }

    pub fn values_f32(&self, file: &[u8], name: &str) -> Option<Vec<f32>> {
        let idx = self.var_index(name)?;
        if self.vars[idx].nc_type != NetcdfType::Float {
            return None;
        }
        let raw = self.full_slab(file, idx)?;
        Some(
            raw.chunks_exact(4)
                .map(|c| f32::from_bits(be_u32(c)))
                .collect(),
        )
    }

    pub fn values_f64(&self, file: &[u8], name: &str) -> Option<Vec<f64>> {
        let idx = self.var_index(name)?;
        if self.vars[idx].nc_type != NetcdfType::Double {
            return None;
        }
        let raw = self.full_slab(file, idx)?;
        Some(
            raw.chunks_exact(8)
                .map(|c| f64::from_bits(be_u64(c)))
                .collect(),
        )
    }

    pub fn values_text(&self, file: &[u8], name: &str) -> Option<String> {
        let idx = self.var_index(name)?;
        if self.vars[idx].nc_type != NetcdfType::Char {
            return None;
        }
        let raw = self.full_slab(file, idx)?;
        let s = String::from_utf8_lossy(&raw).into_owned();
        Some(s.trim_end_matches('\0').to_string())
    }

    fn full_slab(&self, file: &[u8], idx: usize) -> Option<Vec<u8>> {
        let v = self.vars.get(idx)?;
        let shape = self.var_shape(v).ok()?;
        let rank = v.dim_ids.len();
        let start = vec![0usize; rank];
        let count: Vec<usize> = shape.into_iter().map(|s| s as usize).collect();
        self.read_slab(file, idx, &start, &count).ok()
    }

    pub fn attr_text(&self, attr: &NetcdfAttr) -> Option<String> {
        if attr.nc_type != NetcdfType::Char {
            return None;
        }
        let s = String::from_utf8_lossy(&attr.raw).into_owned();
        Some(s.trim_end_matches('\0').to_string())
    }

    pub fn attr_num(&self, attr: &NetcdfAttr) -> Option<f64> {
        let r = &attr.raw;
        match attr.nc_type {
            NetcdfType::Byte => r.first().map(|&b| b as i8 as f64),
            NetcdfType::Char => None,
            NetcdfType::Short => Some(i16::from_be_bytes([*r.get(0)?, *r.get(1)?]) as f64),
            NetcdfType::Int => {
                Some(i32::from_be_bytes([*r.get(0)?, *r.get(1)?, *r.get(2)?, *r.get(3)?]) as f64)
            }
            NetcdfType::Float => Some(f32::from_bits(be_u32(&r.get(0..4)?)) as f64),
            NetcdfType::Double => Some(f64::from_bits(be_u64(&r.get(0..8)?))),
        }
    }
}

fn dim_list(cur: &mut Kur) -> Result<Vec<NetcdfDim>, NetcdfNote> {
    let off = cur.pos;
    let tag = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off })?;
    let nelems = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })? as usize;
    match tag {
        0 => Ok(Vec::new()),
        TAG_DIMENSION => {
            let mut out = Vec::with_capacity(nelems.min(1024));
            for _ in 0..nelems {
                let name = cur.name()?;
                let len = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })?;
                out.push(NetcdfDim { name, len });
            }
            Ok(out)
        }
        _ => Err(NetcdfNote::Tag { tag, off }),
    }
}

fn var_list(cur: &mut Kur, format: NetcdfFormat) -> Result<Vec<NetcdfVar>, NetcdfNote> {
    let off = cur.pos;
    let tag = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off })?;
    let nelems = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })? as usize;
    match tag {
        0 => Ok(Vec::new()),
        TAG_VARIABLE => {
            let mut out = Vec::with_capacity(nelems.min(1024));
            for _ in 0..nelems {
                let name = cur.name()?;
                let rank = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })? as usize;
                let mut dim_ids = Vec::with_capacity(rank);
                for _ in 0..rank {
                    let id = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })? as usize;
                    dim_ids.push(id);
                }
                let attrs = attr_list(cur)?;
                let off = cur.pos;
                let type_tag = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off })?;
                let nc_type =
                    NetcdfType::from_tag(type_tag).ok_or(NetcdfNote::Typ { tag: type_tag, off })?;
                let vsize = cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })?;
                let begin = match format {
                    NetcdfFormat::Cdf1 => {
                        cur.u32().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })? as u64
                    }
                    NetcdfFormat::Cdf2 => {
                        cur.u64().ok_or(NetcdfNote::EndeBeiByte { off: cur.pos })?
                    }
                };
                out.push(NetcdfVar {
                    name,
                    dim_ids,
                    attrs,
                    nc_type,
                    vsize,
                    begin,
                });
            }
            Ok(out)
        }
        _ => Err(NetcdfNote::Tag { tag, off }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u32b(x: u32) -> Vec<u8> {
        x.to_be_bytes().to_vec()
    }
    fn u64b(x: u64) -> Vec<u8> {
        x.to_be_bytes().to_vec()
    }
    fn f64b(x: f64) -> Vec<u8> {
        x.to_bits().to_be_bytes().to_vec()
    }
    fn name(s: &str) -> Vec<u8> {
        let mut b = u32b(s.len() as u32);
        b.extend_from_slice(s.as_bytes());
        while b.len() % 4 != 0 {
            b.push(0);
        }
        b
    }

    fn absent(b: &mut Vec<u8>) {
        b.extend(u32b(0));
        b.extend(u32b(0));
    }

    #[test]
    fn empty_file() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0));
        absent(&mut b);
        absent(&mut b);
        absent(&mut b);
        assert_eq!(b.len(), 32);
        let f = NetcdfFile::parse(&b).unwrap();
        assert!(f.dims.is_empty());
        assert!(f.gattrs.is_empty());
        assert!(f.vars.is_empty());
    }

    #[test]
    fn tiny_reference_file() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0));
        b.extend(u32b(0x0A));
        b.extend(u32b(1));
        b.extend(name("dim"));
        b.extend(u32b(5));
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(1));
        b.extend(name("vx"));
        b.extend(u32b(1));
        b.extend(u32b(0));
        absent(&mut b);
        b.extend(u32b(3));
        b.extend(u32b(12));
        b.extend(u32b(0x50));
        for v in [3i16, 1, 4, 1, 5] {
            b.extend(v.to_be_bytes());
        }
        b.extend([0u8, 0]);
        assert_eq!(b.len(), 92);
        let f = NetcdfFile::parse(&b).unwrap();
        assert_eq!(f.format, NetcdfFormat::Cdf1);
        assert_eq!(f.numrecs, Some(0));
        assert_eq!(f.dims[0].name, "dim");
        assert_eq!(f.dims[0].len, 5);
        assert_eq!(f.vars[0].begin, 80);
        assert_eq!(f.values_i16(&b, "vx").unwrap(), vec![3, 1, 4, 1, 5]);
    }

    #[test]
    fn cdf2_double_var_with_attr() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x02]);
        b.extend(u32b(0));
        b.extend(u32b(0x0A));
        b.extend(u32b(1));
        b.extend(name("x"));
        b.extend(u32b(4));
        b.extend(u32b(0x0C));
        b.extend(u32b(1));
        b.extend(name("title"));
        b.extend(u32b(2));
        b.extend(u32b(4));
        b.extend(b"test".to_vec());
        b.extend(u32b(0x0B));
        b.extend(u32b(1));
        b.extend(name("a"));
        b.extend(u32b(1));
        b.extend(u32b(0));
        absent(&mut b);
        b.extend(u32b(6));
        b.extend(u32b(32));
        let slot = b.len();
        b.extend(u64b(0));
        let data = [1.5f64, -2.25, 0.0, 3.75];
        let begin = b.len() as u64;
        b[slot..slot + 8].copy_from_slice(&begin.to_be_bytes());
        for d in &data {
            b.extend(f64b(*d));
        }
        let f = NetcdfFile::parse(&b).unwrap();
        assert_eq!(f.format, NetcdfFormat::Cdf2);
        assert_eq!(f.vars[0].begin, begin);
        assert_eq!(f.values_f64(&b, "a").unwrap(), data.to_vec());
        let title = f.gattr("title").and_then(|a| f.attr_text(a));
        assert_eq!(title, Some("test".to_string()));
    }

    #[test]
    fn fill_value_attr_visible() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0));
        absent(&mut b);
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(1));
        b.extend(name("v"));
        b.extend(u32b(0));
        b.extend(u32b(0x0C));
        b.extend(u32b(1));
        b.extend(name("_FillValue"));
        b.extend(u32b(6));
        b.extend(u32b(1));
        b.extend(f64b(9.9692099683868690e36));
        b.extend(u32b(6));
        b.extend(u32b(8));
        let slot = b.len();
        b.extend(u32b(0));
        let begin = b.len() as u64;
        b[slot..slot + 4].copy_from_slice(&(begin as u32).to_be_bytes());
        b.extend(f64b(5.0));
        let f = NetcdfFile::parse(&b).unwrap();
        let a = &f.vars[0].attrs[0];
        assert_eq!(a.name, "_FillValue");
        assert_eq!(a.raw, f64b(9.9692099683868690e36));
        assert_eq!(f.attr_num(a), Some(9.9692099683868690e36));
        assert_eq!(f.values_f64(&b, "v").unwrap(), vec![5.0]);
    }

    #[test]
    fn scalar_var() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0));
        absent(&mut b);
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(1));
        b.extend(name("s"));
        b.extend(u32b(0));
        absent(&mut b);
        b.extend(u32b(6));
        b.extend(u32b(8));
        let slot = b.len();
        b.extend(u32b(0));
        let begin = b.len() as u64;
        b[slot..slot + 4].copy_from_slice(&(begin as u32).to_be_bytes());
        b.extend(f64b(42.0));
        let f = NetcdfFile::parse(&b).unwrap();
        assert_eq!(f.values_f64(&b, "s").unwrap(), vec![42.0]);
    }

    #[test]
    fn record_single_short_var_unpadded() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(2));
        b.extend(u32b(0x0A));
        b.extend(u32b(2));
        b.extend(name("rec"));
        b.extend(u32b(0));
        b.extend(name("n"));
        b.extend(u32b(3));
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(1));
        b.extend(name("r"));
        b.extend(u32b(2));
        b.extend(u32b(0));
        b.extend(u32b(1));
        absent(&mut b);
        b.extend(u32b(3));
        b.extend(u32b(8));
        let slot = b.len();
        b.extend(u32b(0));
        let begin = b.len() as u64;
        b[slot..slot + 4].copy_from_slice(&(begin as u32).to_be_bytes());
        for v in [1i16, 2, 3, 4, 5, 6] {
            b.extend(v.to_be_bytes());
        }
        let f = NetcdfFile::parse(&b).unwrap();
        assert_eq!(f.numrecs, Some(2));
        assert!(f.record_var(&f.vars[0]));
        assert_eq!(f.values_i16(&b, "r").unwrap(), vec![1, 2, 3, 4, 5, 6]);
        let slab = f.read_var(&b, "r", &[1, 0], &[1, 2]).unwrap();
        assert_eq!(slab, [4i16.to_be_bytes(), 5i16.to_be_bytes()].concat());
    }

    #[test]
    fn record_two_vars_interleaved() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(2));
        b.extend(u32b(0x0A));
        b.extend(u32b(2));
        b.extend(name("rec"));
        b.extend(u32b(0));
        b.extend(name("n"));
        b.extend(u32b(2));
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(2));
        b.extend(name("a"));
        b.extend(u32b(2));
        b.extend(u32b(0));
        b.extend(u32b(1));
        absent(&mut b);
        b.extend(u32b(6));
        b.extend(u32b(16));
        let slot_a = b.len();
        b.extend(u32b(0));
        b.extend(name("b"));
        b.extend(u32b(2));
        b.extend(u32b(0));
        b.extend(u32b(1));
        absent(&mut b);
        b.extend(u32b(3));
        b.extend(u32b(4));
        let slot_b = b.len();
        b.extend(u32b(0));
        let a = [10.0f64, 11.0, 12.0, 13.0];
        let bv = [100i16, 101, 102, 103];
        let data_start = b.len() as u64;
        b[slot_a..slot_a + 4].copy_from_slice(&(data_start as u32).to_be_bytes());
        b[slot_b..slot_b + 4].copy_from_slice(&((data_start + 16) as u32).to_be_bytes());
        for r in 0..2 {
            b.extend(f64b(a[r * 2]));
            b.extend(f64b(a[r * 2 + 1]));
            b.extend(bv[r * 2].to_be_bytes());
            b.extend(bv[r * 2 + 1].to_be_bytes());
        }
        let f = NetcdfFile::parse(&b).unwrap();
        assert_eq!(f.values_f64(&b, "a").unwrap(), a.to_vec());
        assert_eq!(f.values_i16(&b, "b").unwrap(), bv.to_vec());
    }

    #[test]
    fn cdf5_is_pending() {
        let b = [0x43, 0x44, 0x46, 0x05];
        assert!(matches!(NetcdfFile::parse(&b), Err(NetcdfNote::Cdf5)));
    }

    #[test]
    fn unknown_magic() {
        let b = [0x00u8, 0x01, 0x02, 0x03];
        assert!(matches!(
            NetcdfFile::parse(&b),
            Err(NetcdfNote::Magie { .. })
        ));
    }

    #[test]
    fn truncated() {
        let b = [0x43u8, 0x44, 0x46, 0x01, 0x00];
        assert!(matches!(
            NetcdfFile::parse(&b),
            Err(NetcdfNote::EndeBeiByte { .. })
        ));
    }

    #[test]
    fn streaming_numrecs_open() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0xFFFF_FFFF));
        b.extend(u32b(0x0A));
        b.extend(u32b(1));
        b.extend(name("rec"));
        b.extend(u32b(0));
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(1));
        b.extend(name("r"));
        b.extend(u32b(1));
        b.extend(u32b(0));
        absent(&mut b);
        b.extend(u32b(6));
        b.extend(u32b(8));
        b.extend(u32b(0));
        let f = NetcdfFile::parse(&b).unwrap();
        assert_eq!(f.numrecs, None);
        assert_eq!(f.values_f64(&b, "r"), None);
    }

    #[test]
    fn unknown_type_tag() {
        let mut b = Vec::new();
        b.extend([0x43, 0x44, 0x46, 0x01]);
        b.extend(u32b(0));
        absent(&mut b);
        absent(&mut b);
        b.extend(u32b(0x0B));
        b.extend(u32b(1));
        b.extend(name("v"));
        b.extend(u32b(0));
        absent(&mut b);
        b.extend(u32b(9));
        b.extend(u32b(4));
        b.extend(u32b(0));
        assert!(matches!(
            NetcdfFile::parse(&b),
            Err(NetcdfNote::Typ { tag: 9, .. })
        ));
    }
}
