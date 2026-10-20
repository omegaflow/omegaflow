use std::collections::HashMap;

pub const MAGIC: [u8; 4] = [0xcd, 0xf3, 0x00, 0x01];

pub const SECTION_CDR: u32 = 1;
pub const SECTION_GDR: u32 = 2;
pub const SECTION_RVDR: u32 = 3;
pub const SECTION_ADR: u32 = 4;
pub const SECTION_VXR: u32 = 6;
pub const SECTION_VD_ZVAR: u32 = 8;
pub const SECTION_VVR: u32 = 7;
pub const SECTION_ZVDR: u32 = 9;
pub const SECTION_CVVR: u32 = 13;

pub const TYPE_INT1: u32 = 1;
pub const TYPE_INT2: u32 = 2;
pub const TYPE_INT4: u32 = 4;
pub const TYPE_INT8: u32 = 8;
pub const TYPE_UINT1: u32 = 11;
pub const TYPE_UINT2: u32 = 12;
pub const TYPE_UINT4: u32 = 14;
pub const TYPE_REAL4: u32 = 21;
pub const TYPE_REAL8: u32 = 22;
pub const TYPE_EPOCH: u32 = 31;
pub const TYPE_EPOCH16: u32 = 32;
pub const TYPE_TT2000: u32 = 33;
pub const TYPE_BYTE: u32 = 41;
pub const TYPE_FLOAT: u32 = 44;
pub const TYPE_DOUBLE: u32 = 45;
pub const TYPE_CHAR: u32 = 51;
pub const TYPE_UCHAR: u32 = 52;

pub const CTYPE_NONE: u32 = 0;
pub const CTYPE_RLE: u32 = 1;
pub const CTYPE_HUFF: u32 = 2;
pub const CTYPE_AHUFF: u32 = 3;
pub const CTYPE_GZIP: u32 = 5;

pub const FILL_F64: f64 = -1.0e31;

pub fn value_present(v: f64) -> bool {
    v.is_finite() && v.abs() < 1.0e30
}

pub const EPOCH_UNIX_OFFSET_S: f64 = 62_167_219_200.0;
pub const TT2000_UNIX_OFFSET_S: f64 = 946_727_935.816;

pub enum CdfNote {
    NotCdf {
        magic: [u8; 4],
    },
    Version(u32),
    WholeFileCompressed,
    Encoding(u32),
    MultiFormat,
    EndAtByte {
        off: usize,
    },
    VdrSection(u32),
    DataType(u32),
    SparseRecord,
    BlockKind(u32),
    VxrSection(u32),
    CprSection(u32),
    CompressionType(u32),
    DecompressVoid {
        off: usize,
    },
    RecordMismatch {
        var: usize,
        bytes: usize,
        per: usize,
    },
    AbsentVariable(String),
    NoRecords,
}

impl std::fmt::Debug for CdfNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CdfNote::NotCdf { magic } => write!(f, "magic {:02x?} is not CDF3", magic),
            CdfNote::Version(v) => write!(f, "CDF version {v} is not 3"),
            CdfNote::WholeFileCompressed => {
                write!(f, "the file carries whole-file compression — unread")
            }
            CdfNote::Encoding(e) => write!(f, "encoding {e} is not host/network"),
            CdfNote::MultiFormat => write!(f, "multi-format CDF — unread"),
            CdfNote::EndAtByte { off } => write!(f, "file ends at byte {off}"),
            CdfNote::VdrSection(s) => write!(f, "VDR section type {s} is not a zVariable"),
            CdfNote::DataType(t) => write!(f, "data type {t} is unknown"),
            CdfNote::SparseRecord => write!(f, "sparse records — unread"),
            CdfNote::BlockKind(k) => write!(f, "data block kind {k} is not VVR/CVVR"),
            CdfNote::VxrSection(s) => write!(f, "VXR section type {s} is unknown"),
            CdfNote::CprSection(s) => write!(f, "CPR section type {s} is unknown"),
            CdfNote::CompressionType(t) => write!(f, "compression type {t} is a parser gap"),
            CdfNote::DecompressVoid { off } => {
                write!(f, "the compressed block at {off} does not decompress")
            }
            CdfNote::RecordMismatch { var, bytes, per } => write!(
                f,
                "variable {var}: {} data bytes do not split into records of {}",
                bytes, per
            ),
            CdfNote::AbsentVariable(name) => write!(f, "variable {name} is absent"),
            CdfNote::NoRecords => write!(f, "the variable carries no records"),
        }
    }
}

pub struct CdfBlock {
    pub start: u32,
    pub end: u32,
    pub offset: u64,
    pub kind: u32,
}

pub struct CdfVar {
    pub name: String,
    pub var_num: u32,
    pub data_type: u32,
    pub num_elements: u32,
    pub record_vary: bool,
    pub pad: bool,
    pub compressed: bool,
    pub compression_type: Option<u32>,
    pub sparse: u32,
    pub dim_sizes: Vec<u32>,
    pub dim_vary: Vec<u32>,
    pub head_vxr: u64,
    pub max_rec: u32,
    pub pad_value: Vec<f64>,
}

pub struct CdfFile {
    pub version: (u32, u32, u32),
    pub encoding: u32,
    pub little_endian: bool,
    pub majority: u32,
    pub vars: Vec<CdfVar>,
    pub num_att: u32,
    pub eof: u64,
}

fn be_u32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn be_u64(b: &[u8], off: usize) -> Option<u64> {
    Some(u64::from_be_bytes(b.get(off..off + 8)?.try_into().ok()?))
}

fn f64_at(b: &[u8], off: usize, little: bool) -> Option<f64> {
    let r = b.get(off..off + 8)?.try_into().ok()?;
    Some(if little {
        f64::from_le_bytes(r)
    } else {
        f64::from_be_bytes(r)
    })
}

fn f32_at(b: &[u8], off: usize, little: bool) -> Option<f32> {
    let r = b.get(off..off + 4)?.try_into().ok()?;
    Some(if little {
        f32::from_le_bytes(r)
    } else {
        f32::from_be_bytes(r)
    })
}

fn i64_at(b: &[u8], off: usize, little: bool) -> Option<i64> {
    let r = b.get(off..off + 8)?.try_into().ok()?;
    Some(if little {
        i64::from_le_bytes(r)
    } else {
        i64::from_be_bytes(r)
    })
}

fn trim_name(raw: &[u8]) -> String {
    let end = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
    String::from_utf8_lossy(&raw[..end]).into_owned()
}

pub fn type_size(t: u32) -> Option<usize> {
    match t {
        TYPE_INT1 | TYPE_UINT1 | TYPE_BYTE => Some(1),
        TYPE_INT2 | TYPE_UINT2 => Some(2),
        TYPE_INT4 | TYPE_UINT4 | TYPE_REAL4 | TYPE_FLOAT => Some(4),
        TYPE_INT8 | TYPE_REAL8 | TYPE_DOUBLE | TYPE_EPOCH | TYPE_TT2000 => Some(8),
        TYPE_EPOCH16 => Some(16),
        _ => None,
    }
}

pub fn is_epoch_type(t: u32) -> bool {
    matches!(t, TYPE_EPOCH | TYPE_EPOCH16 | TYPE_TT2000)
}

pub fn cdf_epoch_unix(ms: f64) -> f64 {
    ms / 1000.0 - EPOCH_UNIX_OFFSET_S
}

pub fn cdf_epoch16_unix(sec: f64, ps: f64) -> f64 {
    sec + ps * 1.0e-12 - EPOCH_UNIX_OFFSET_S
}

pub fn tt2000_unix(ns: i64) -> f64 {
    ns as f64 * 1.0e-9 + TT2000_UNIX_OFFSET_S
}

fn decode_value(b: &[u8], off: usize, t: u32, little: bool) -> Option<f64> {
    match t {
        TYPE_INT1 | TYPE_BYTE => Some(*b.get(off)? as i8 as f64),
        TYPE_UINT1 => Some(*b.get(off)? as f64),
        TYPE_INT2 => {
            let r = b.get(off..off + 2)?.try_into().ok()?;
            Some(if little {
                i16::from_le_bytes(r) as f64
            } else {
                i16::from_be_bytes(r) as f64
            })
        }
        TYPE_UINT2 => {
            let r = b.get(off..off + 2)?.try_into().ok()?;
            Some(if little {
                u16::from_le_bytes(r) as f64
            } else {
                u16::from_be_bytes(r) as f64
            })
        }
        TYPE_INT4 => {
            let r = b.get(off..off + 4)?.try_into().ok()?;
            Some(if little {
                i32::from_le_bytes(r) as f64
            } else {
                i32::from_be_bytes(r) as f64
            })
        }
        TYPE_UINT4 => {
            let r = b.get(off..off + 4)?.try_into().ok()?;
            Some(if little {
                u32::from_le_bytes(r) as f64
            } else {
                u32::from_be_bytes(r) as f64
            })
        }
        TYPE_INT8 | TYPE_TT2000 => Some(i64_at(b, off, little)? as f64),
        TYPE_REAL4 | TYPE_FLOAT => Some(f32_at(b, off, little)? as f64),
        TYPE_REAL8 | TYPE_DOUBLE | TYPE_EPOCH => Some(f64_at(b, off, little)?),
        _ => None,
    }
}

pub fn decode_record(b: &[u8], t: u32, num_values: usize, little: bool) -> Option<Vec<f64>> {
    if t == TYPE_EPOCH16 {
        let mut out = Vec::with_capacity(num_values);
        for i in 0..num_values {
            let sec = f64_at(b, i * 16, little)?;
            let ps = f64_at(b, i * 16 + 8, little)?;
            out.push(cdf_epoch16_unix(sec, ps));
        }
        return Some(out);
    }
    if t == TYPE_EPOCH {
        let mut out = Vec::with_capacity(num_values);
        for i in 0..num_values {
            out.push(cdf_epoch_unix(f64_at(b, i * 8, little)?));
        }
        return Some(out);
    }
    if t == TYPE_TT2000 {
        let mut out = Vec::with_capacity(num_values);
        for i in 0..num_values {
            out.push(tt2000_unix(i64_at(b, i * 8, little)?));
        }
        return Some(out);
    }
    let ts = type_size(t)?;
    let mut out = Vec::with_capacity(num_values);
    for i in 0..num_values {
        out.push(decode_value(b, i * ts, t, little)?);
    }
    Some(out)
}

fn walk_vxrs(
    bytes: &[u8],
    offset: u64,
    out: &mut Vec<CdfBlock>,
    depth: usize,
) -> Result<(), CdfNote> {
    if depth > 32 {
        return Err(CdfNote::VxrSection(0));
    }
    let off = offset as usize;
    let size = be_u64(bytes, off).ok_or(CdfNote::EndAtByte { off })? as usize;
    let rec = bytes
        .get(off..off + size)
        .ok_or(CdfNote::EndAtByte { off })?;
    let section = be_u32(rec, 8).ok_or(CdfNote::EndAtByte { off })?;
    if section != SECTION_VXR {
        return Err(CdfNote::VxrSection(section));
    }
    let next = be_u64(rec, 12).ok_or(CdfNote::EndAtByte { off })?;
    let num_ent = be_u32(rec, 20).ok_or(CdfNote::EndAtByte { off })? as usize;
    let used = be_u32(rec, 24).ok_or(CdfNote::EndAtByte { off })? as usize;
    for i in 0..used {
        let start = be_u32(rec, 28 + 4 * i).ok_or(CdfNote::EndAtByte { off })?;
        let end = be_u32(rec, 28 + 4 * num_ent + 4 * i).ok_or(CdfNote::EndAtByte { off })?;
        let rec_off = be_u64(rec, 28 + 8 * num_ent + 8 * i).ok_or(CdfNote::EndAtByte { off })?;
        let target = rec_off as usize;
        let kind = be_u32(bytes, target + 8).ok_or(CdfNote::EndAtByte { off: target })?;
        if kind == SECTION_VXR {
            walk_vxrs(bytes, rec_off, out, depth + 1)?;
        } else {
            out.push(CdfBlock {
                start,
                end,
                offset: rec_off,
                kind,
            });
        }
    }
    if next != 0 {
        walk_vxrs(bytes, next, out, depth + 1)?;
    }
    Ok(())
}

impl CdfFile {
    pub fn parse(bytes: &[u8]) -> Result<CdfFile, CdfNote> {
        let magic: [u8; 4] = bytes
            .get(0..4)
            .ok_or(CdfNote::EndAtByte { off: 0 })?
            .try_into()
            .ok()
            .unwrap();
        if magic != MAGIC {
            return Err(CdfNote::NotCdf { magic });
        }
        let whole = be_u32(bytes, 4).ok_or(CdfNote::EndAtByte { off: 4 })?;
        if whole != 0x0000ffff {
            return Err(CdfNote::WholeFileCompressed);
        }
        let cdr_size = be_u64(bytes, 8).ok_or(CdfNote::EndAtByte { off: 8 })? as usize;
        let cdr = bytes
            .get(16..8 + cdr_size)
            .ok_or(CdfNote::EndAtByte { off: 8 })?;
        let gdr_off = be_u64(cdr, 4).ok_or(CdfNote::EndAtByte { off: 8 })? as usize;
        let version = be_u32(cdr, 12).ok_or(CdfNote::EndAtByte { off: 8 })?;
        if version != 3 {
            return Err(CdfNote::Version(version));
        }
        let release = be_u32(cdr, 16).ok_or(CdfNote::EndAtByte { off: 8 })?;
        let encoding = be_u32(cdr, 20).ok_or(CdfNote::EndAtByte { off: 8 })?;
        let little = match encoding {
            1 => false,
            6 => true,
            other => return Err(CdfNote::Encoding(other)),
        };
        let flag = be_u32(cdr, 24).ok_or(CdfNote::EndAtByte { off: 8 })?;
        let single_file = flag & 0x0002 != 0;
        if !single_file {
            return Err(CdfNote::MultiFormat);
        }
        let majority = if flag & 0x0001 != 0 { 1 } else { 2 };
        let increment = be_u32(cdr, 36).ok_or(CdfNote::EndAtByte { off: 8 })?;

        let gdr_size = be_u64(bytes, gdr_off).ok_or(CdfNote::EndAtByte { off: gdr_off })? as usize;
        let gdr = bytes
            .get(gdr_off + 8..gdr_off + gdr_size)
            .ok_or(CdfNote::EndAtByte { off: gdr_off })?;
        let first_zvar = be_u64(gdr, 12).ok_or(CdfNote::EndAtByte { off: gdr_off })? as usize;
        let eof = be_u64(gdr, 28).ok_or(CdfNote::EndAtByte { off: gdr_off })?;
        let num_att = be_u32(gdr, 40).ok_or(CdfNote::EndAtByte { off: gdr_off })?;
        let num_zvar = be_u32(gdr, 52).ok_or(CdfNote::EndAtByte { off: gdr_off })?;

        let mut vars = Vec::with_capacity(num_zvar as usize);
        let mut next = first_zvar;
        let mut guard = 0usize;
        while next != 0 && next != u64::MAX as usize && guard < num_zvar as usize + 1 {
            guard += 1;
            let off = next;
            let size = be_u64(bytes, off).ok_or(CdfNote::EndAtByte { off })? as usize;
            let rec = bytes
                .get(off..off + size)
                .ok_or(CdfNote::EndAtByte { off })?;
            let section = be_u32(rec, 8).ok_or(CdfNote::EndAtByte { off })?;
            if section != SECTION_VD_ZVAR {
                return Err(CdfNote::VdrSection(section));
            }
            let next_vdr = be_u64(rec, 12).ok_or(CdfNote::EndAtByte { off })?;
            let data_type = be_u32(rec, 20).ok_or(CdfNote::EndAtByte { off })?;
            let max_rec = be_u32(rec, 24).ok_or(CdfNote::EndAtByte { off })?;
            let head_vxr = be_u64(rec, 28).ok_or(CdfNote::EndAtByte { off })?;
            let flags = be_u32(rec, 44).ok_or(CdfNote::EndAtByte { off })?;
            let sparse = be_u32(rec, 48).ok_or(CdfNote::EndAtByte { off })?;
            let num_elements = be_u32(rec, 64).ok_or(CdfNote::EndAtByte { off })?;
            let var_num = be_u32(rec, 68).ok_or(CdfNote::EndAtByte { off })?;
            let cpr = be_u64(rec, 72).ok_or(CdfNote::EndAtByte { off })?;
            let name = trim_name(rec.get(84..340).ok_or(CdfNote::EndAtByte { off })?);
            let num_dims = be_u32(rec, 340).ok_or(CdfNote::EndAtByte { off })? as usize;
            let mut dim_sizes = Vec::with_capacity(num_dims);
            let mut dim_vary = Vec::with_capacity(num_dims);
            for i in 0..num_dims {
                dim_sizes.push(be_u32(rec, 344 + 4 * i).ok_or(CdfNote::EndAtByte { off })?);
            }
            for i in 0..num_dims {
                dim_vary.push(
                    be_u32(rec, 344 + 4 * num_dims + 4 * i).ok_or(CdfNote::EndAtByte { off })?,
                );
            }
            let record_vary = flags & 0x0001 != 0;
            let pad = flags & 0x0002 != 0;
            let compressed = flags & 0x0004 != 0;
            let mut compression_type = None;
            let mut pad_value = Vec::new();
            let coff = 344 + 8 * num_dims;
            if compressed {
                let cpr_off = cpr as usize;
                let cpr_size =
                    be_u64(bytes, cpr_off).ok_or(CdfNote::EndAtByte { off: cpr_off })? as usize;
                let cpr_rec = bytes
                    .get(cpr_off..cpr_off + cpr_size)
                    .ok_or(CdfNote::EndAtByte { off: cpr_off })?;
                let ctype = be_u32(cpr_rec, 12).ok_or(CdfNote::EndAtByte { off: cpr_off })?;
                if !matches!(ctype, CTYPE_NONE | CTYPE_RLE | CTYPE_GZIP) {
                    return Err(CdfNote::CompressionType(ctype));
                }
                compression_type = Some(ctype);
            }
            if pad && let (Some(ts), true) = (type_size(data_type), data_type != TYPE_EPOCH16) {
                for i in 0..num_elements as usize {
                    pad_value.push(
                        decode_value(rec, coff + i * ts, data_type, little)
                            .ok_or(CdfNote::EndAtByte { off })?,
                    );
                }
            }
            vars.push(CdfVar {
                name,
                var_num,
                data_type,
                num_elements,
                record_vary,
                pad,
                compressed,
                compression_type,
                sparse,
                dim_sizes,
                dim_vary,
                head_vxr,
                max_rec,
                pad_value,
            });
            next = next_vdr as usize;
        }
        Ok(CdfFile {
            version: (version, release, increment),
            encoding,
            little_endian: little,
            majority,
            vars,
            num_att,
            eof,
        })
    }

    pub fn var(&self, name: &str) -> Option<&CdfVar> {
        self.vars.iter().find(|v| v.name == name)
    }

    pub fn blocks(&self, bytes: &[u8], var: &CdfVar) -> Result<Vec<CdfBlock>, CdfNote> {
        if var.head_vxr == 0 {
            return Err(CdfNote::NoRecords);
        }
        let mut out = Vec::new();
        walk_vxrs(bytes, var.head_vxr, &mut out, 0)?;
        Ok(out)
    }

    pub fn num_values(&self, var: &CdfVar) -> usize {
        let mut n = 1usize;
        for (i, v) in var.dim_vary.iter().enumerate() {
            if *v != 0 {
                n *= var.dim_sizes[i] as usize;
            }
        }
        n
    }

    pub fn block_data(&self, bytes: &[u8], block: &CdfBlock) -> Result<Vec<u8>, CdfNote> {
        let off = block.offset as usize;
        let size = be_u64(bytes, off).ok_or(CdfNote::EndAtByte { off })? as usize;
        let rec = bytes
            .get(off..off + size)
            .ok_or(CdfNote::EndAtByte { off })?;
        match block.kind {
            SECTION_VVR => Ok(rec.get(12..).ok_or(CdfNote::EndAtByte { off })?.to_vec()),
            SECTION_CVVR => {
                let cs = be_u64(rec, 16).ok_or(CdfNote::EndAtByte { off })? as usize;
                let gz = rec.get(24..24 + cs).ok_or(CdfNote::EndAtByte { off })?;
                crate::inflate::gunzip(gz).ok_or(CdfNote::DecompressVoid { off })
            }
            _ => Err(CdfNote::BlockKind(block.kind)),
        }
    }

    pub fn var_records(&self, bytes: &[u8], var: &CdfVar) -> Result<Vec<(u32, Vec<f64>)>, CdfNote> {
        if var.sparse != 0 {
            return Err(CdfNote::SparseRecord);
        }
        if !var.record_vary {
            let mut out = Vec::new();
            for block in self.blocks(bytes, var)? {
                let data = self.block_data(bytes, &block)?;
                let per = match var.data_type {
                    TYPE_EPOCH16 => 16,
                    _ => type_size(var.data_type).ok_or(CdfNote::DataType(var.data_type))?,
                } * self.num_values(var);
                if data.len() % per != 0 {
                    return Err(CdfNote::RecordMismatch {
                        var: var.var_num as usize,
                        bytes: data.len(),
                        per,
                    });
                }
                for i in 0..data.len() / per {
                    out.push((
                        0,
                        decode_record(
                            &data[i * per..i * per + per],
                            var.data_type,
                            self.num_values(var),
                            self.little_endian,
                        )
                        .ok_or(CdfNote::DataType(var.data_type))?,
                    ));
                }
            }
            return Ok(out);
        }
        let ts = match var.data_type {
            TYPE_EPOCH16 => 16,
            _ => type_size(var.data_type).ok_or(CdfNote::DataType(var.data_type))?,
        };
        let per = ts * self.num_values(var);
        let mut out = Vec::new();
        for block in self.blocks(bytes, var)? {
            let data = self.block_data(bytes, &block)?;
            if data.len() % per != 0 {
                return Err(CdfNote::RecordMismatch {
                    var: var.var_num as usize,
                    bytes: data.len(),
                    per,
                });
            }
            let recs = data.len() / per;
            for i in 0..recs {
                let rec_num = block.start.saturating_add(i as u32);
                out.push((
                    rec_num,
                    decode_record(
                        &data[i * per..i * per + per],
                        var.data_type,
                        self.num_values(var),
                        self.little_endian,
                    )
                    .ok_or(CdfNote::DataType(var.data_type))?,
                ));
            }
        }
        Ok(out)
    }

    pub fn epoch_map(&self, bytes: &[u8], var: &CdfVar) -> Result<HashMap<u32, f64>, CdfNote> {
        if !is_epoch_type(var.data_type) {
            return Err(CdfNote::DataType(var.data_type));
        }
        let mut map = HashMap::new();
        for (rec_num, vals) in self.var_records(bytes, var)? {
            map.insert(rec_num, vals[0]);
        }
        Ok(map)
    }
}

pub fn decompress_whole_file(bytes: &[u8]) -> Option<Vec<u8>> {
    let block = be_u64(bytes, 8)? as usize;
    let data_start = 8usize + 32;
    let data_size = block.checked_sub(32)?;
    let cpr = be_u64(bytes, 20)? as usize;
    let cpr_block = be_u64(bytes, cpr)? as usize;
    let cpr_rec = bytes.get(cpr..cpr + cpr_block)?;
    let ctype = be_u32(cpr_rec, 12)?;
    if ctype != CTYPE_GZIP {
        return None;
    }
    let stream = bytes.get(data_start..data_start + data_size)?;
    let inner = crate::inflate::gunzip(stream)?;
    let mut out = Vec::with_capacity(8 + inner.len());
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&0x0000_ffffu32.to_be_bytes());
    out.extend_from_slice(&inner);
    Some(out)
}

fn cdf_epoch_var(file: &CdfFile) -> Option<&CdfVar> {
    file.var("Epoch")
        .or_else(|| file.vars.iter().find(|v| is_epoch_type(v.data_type)))
}

pub fn extract_witness_channels(
    src: &crate::archivar::SourceConfig,
    body: &str,
    lsk: &crate::archivar::LeapSeconds,
) -> Option<crate::archivar::ExtractResult> {
    use crate::archivar::{Channel, Extract, ExtractResult, FieldConfig};

    let bytes = std::fs::read(body).ok()?;
    if bytes.len() < 4 || bytes[..4] != MAGIC {
        return None;
    }
    let marker = be_u32(&bytes, 4)?;
    let owned;
    let file_bytes: &[u8] = if marker == 0x0000_ffff {
        &bytes
    } else {
        owned = decompress_whole_file(&bytes)?;
        &owned
    };
    let file = CdfFile::parse(file_bytes).ok()?;
    let Some(epoch_var) = cdf_epoch_var(&file) else {
        return Some(ExtractResult::Measurements(vec![]));
    };
    let Ok(epoch_map) = file.epoch_map(file_bytes, epoch_var) else {
        return Some(ExtractResult::Measurements(vec![]));
    };
    let mut channels: Vec<(Channel, FieldConfig)> = Vec::new();
    for ext in &src.extracts {
        let Extract::Field(fc) = ext else {
            continue;
        };
        let tags: Vec<&str> = fc.key.split('@').collect();
        let (value_key, axis_key, cal_key) = match tags.as_slice() {
            [value] => (*value, None, None),
            [value, axis] => (*value, Some(*axis), None),
            [value, axis, cal] => (*value, Some(*axis), Some(*cal)),
            _ => continue,
        };
        let Some(var) = file.var(value_key) else {
            continue;
        };
        let Ok(records) = file.var_records(file_bytes, var) else {
            continue;
        };
        let axis: Option<HashMap<u32, Vec<f64>>> = axis_key
            .and_then(|key| file.var(key))
            .and_then(|av| file.var_records(file_bytes, av).ok())
            .map(|rows| rows.into_iter().collect());
        let cal: Option<Vec<(f64, f64)>> = cal_key
            .and_then(|key| file.var(key))
            .and_then(|cv| file.var_records(file_bytes, cv).ok())
            .and_then(|rows| rows.into_iter().next())
            .map(|(_, vals)| pair_curve(&vals));
        push_cdf_field(
            fc,
            &records,
            axis.as_ref(),
            cal.as_deref(),
            &epoch_map,
            lsk,
            &mut channels,
        );
    }
    Some(ExtractResult::Measurements(channels))
}

pub const NT_PER_T: f64 = 1.0e9;

fn pair_curve(vals: &[f64]) -> Vec<(f64, f64)> {
    vals.as_chunks::<2>()
        .0
        .iter()
        .filter_map(|pair| {
            (pair[0].is_finite() && pair[1].is_finite() && pair[1] > 0.0)
                .then_some((pair[0], pair[1]))
        })
        .collect()
}

fn interp_curve(curve: &[(f64, f64)], x: f64) -> Option<f64> {
    let first = curve.first()?;
    let last = curve.last()?;
    if !x.is_finite() || x < first.0 || x > last.0 {
        return None;
    }
    for window in curve.windows(2) {
        let (x0, y0) = window[0];
        let (x1, y1) = window[1];
        if x <= x1 {
            if x1.partial_cmp(&x0) != Some(std::cmp::Ordering::Greater) {
                return None;
            }
            let y = y0 + (x - x0) / (x1 - x0) * (y1 - y0);
            return (y.is_finite() && y > 0.0).then_some(y);
        }
    }
    (last.1 > 0.0).then_some(last.1)
}

fn push_cdf_field(
    fc: &crate::archivar::FieldConfig,
    records: &[(u32, Vec<f64>)],
    axis: Option<&HashMap<u32, Vec<f64>>>,
    cal: Option<&[(f64, f64)]>,
    epoch_map: &HashMap<u32, f64>,
    lsk: &crate::archivar::LeapSeconds,
    out: &mut Vec<(crate::archivar::Channel, crate::archivar::FieldConfig)>,
) {
    use crate::archivar::{Channel, Position};

    for (rec_num, vals) in records {
        let Some(&t_unix) = epoch_map.get(rec_num) else {
            continue;
        };
        if !value_present(t_unix) {
            continue;
        }
        let Some(epoch) = lsk.unix_to_tdb(t_unix) else {
            continue;
        };
        let axis_vals = axis.and_then(|map| map.get(rec_num));
        for (bin, raw) in vals.iter().enumerate() {
            if !value_present(*raw) {
                continue;
            }
            let (freq, bin_width) = match axis_vals {
                Some(a) => match axis_at(a, bin) {
                    Some(pair) => pair,
                    None => continue,
                },
                None => {
                    if bin > 0 {
                        continue;
                    }
                    (0.0, 0.0)
                }
            };
            let value = match cal {
                Some(curve) => {
                    let Some(factor) = interp_curve(curve, freq) else {
                        continue;
                    };
                    let cal_nt = factor * NT_PER_T;
                    let denom = cal_nt * cal_nt;
                    if !(denom.is_finite() && denom > 0.0) {
                        continue;
                    }
                    let field = raw / denom;
                    if !value_present(field) {
                        continue;
                    }
                    field
                }
                None => *raw,
            };
            out.push((
                Channel {
                    z: 0.0,
                    freq,
                    bin_width,
                    epoch,
                    station_code: None,
                    position: Position::Source,
                    name: fc.name.clone(),
                    value,
                },
                fc.clone(),
            ));
        }
    }
}

fn axis_at(axis: &[f64], bin: usize) -> Option<(f64, f64)> {
    let freq = *axis.get(bin)?;
    if !(freq.is_finite() && freq > 0.0) {
        return None;
    }
    let lo = bin.checked_sub(1).and_then(|i| axis.get(i).copied());
    let hi = axis.get(bin + 1).copied();
    let width = match (lo, hi) {
        (Some(a), Some(b)) => (b - a).abs() * 0.5,
        (Some(a), None) => (freq - a).abs(),
        (None, Some(b)) => (b - freq).abs(),
        (None, None) => 0.0,
    };
    Some((freq, width))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_anchors() {
        assert_eq!(cdf_epoch_unix(62_167_219_200_000.0), 0.0);
        assert_eq!(cdf_epoch_unix(63_113_904_000_000.0), 946_684_800.0);
        assert_eq!(cdf_epoch16_unix(62_167_219_200.0, 0.0), 0.0);
        assert_eq!(tt2000_unix(0), 946_727_935.816);
    }

    #[test]
    fn decode_scalars() {
        assert_eq!(decode_value(&[0, 0, 0, 5], 0, TYPE_INT4, false), Some(5.0));
        assert_eq!(
            decode_value(
                &[0x40, 0x09, 0x1e, 0xb8, 0x51, 0xeb, 0x85, 0x1f],
                0,
                TYPE_REAL8,
                false
            ),
            Some(f64::from_bits(0x4009_1eb8_51eb_851f))
        );
        assert_eq!(
            decode_value(&[0, 0, 0, 0, 0, 0, 0, 10], 0, TYPE_TT2000, false),
            Some(10.0)
        );
        assert_eq!(
            decode_value(&[10, 0, 0, 0, 0, 0, 0, 0], 0, TYPE_TT2000, true),
            Some(10.0)
        );
    }

    #[test]
    fn parses_lira_when_present() {
        let path = crate::temp_path("lira10s_20251231.cdf");
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = CdfFile::parse(&bytes).unwrap();
        assert!(!file.little_endian);
        assert!(file.vars.iter().any(|v| v.name == "Epoch"));
        assert!(file.vars.iter().any(|v| v.name == "EDC_SRF"));
        let edc = file.var("EDC_SRF").unwrap();
        let records = file.var_records(&bytes, edc).unwrap();
        assert!(records.len() > 8000);
        for (_, vals) in &records {
            assert_eq!(vals.len(), 3);
        }
    }

    #[test]
    fn parses_little_endian_when_present() {
        let path = crate::temp_path("lira10s_20221201.cdf");
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = CdfFile::parse(&bytes).unwrap();
        let edc = file.var("EDC_SRF").unwrap();
        let records = file.var_records(&bytes, edc).unwrap();
        assert!(records.len() > 8000);
        let real: Vec<f64> = records
            .iter()
            .flat_map(|(_, v)| v.iter().copied())
            .filter(|v| value_present(*v))
            .collect();
        assert!(!real.is_empty());
        for v in &real {
            assert!(v.abs() < 1.0e3);
        }
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(matches!(
            CdfFile::parse(b"not a cdf at all"),
            Err(CdfNote::NotCdf { .. })
        ));
    }

    #[test]
    fn parses_wind_wav_h1_2021_when_present() {
        let path = crate::temp_path("wav_h1_20210101.cdf");
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = CdfFile::parse(&bytes).unwrap();
        assert!(!file.little_endian);
        assert!(file.vars.iter().any(|v| v.name == "E_VOLTAGE_RAD2"));
        let epoch = file.var("Epoch").unwrap();
        let epoch_records = file.var_records(&bytes, epoch).unwrap();
        assert_eq!(epoch_records.len(), 1440);
        let first_unix = epoch_records[0].1[0];
        assert!(first_unix > 1.6e9 && first_unix < 1.7e9);
        let rad2 = file.var("E_VOLTAGE_RAD2").unwrap();
        let records = file.var_records(&bytes, rad2).unwrap();
        assert_eq!(records.len(), 1440);
        for (_, vals) in &records {
            assert_eq!(vals.len(), 256);
        }
    }

    #[test]
    fn parses_wind_wav_h1_1994_when_present() {
        let path = crate::temp_path("wav_h1_19941110.cdf");
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = CdfFile::parse(&bytes).unwrap();
        assert_eq!(file.version.0, 3);
        assert!(!file.little_endian);
        let rad1 = file.var("E_VOLTAGE_RAD1").unwrap();
        let records = file.var_records(&bytes, rad1).unwrap();
        assert_eq!(records.len(), 1440);
        for (_, vals) in &records {
            assert_eq!(vals.len(), 256);
        }
    }

    #[test]
    fn spectrum_expands_one_channel_per_bin_with_the_axis_frequency() {
        use crate::archivar::{Aperture, Channel, FieldConfig, LeapSeconds};

        fn field(key: &str) -> FieldConfig {
            FieldConfig {
                key: key.into(),
                name: "spectrum".into(),
                band_id: None,
                kernel: 0,
                channel_ref: Some(0),
                tau: 0.0,
                absorption: 0.0,
                advection: 0.0,
                unit: "1".into(),
                freq: 0.0,
                bin_width: 0.0,
                fold: None,
                aperture: Aperture::None,
            }
        }
        let lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let mut epoch_map = HashMap::new();
        epoch_map.insert(0u32, 1_000_000_000.0);
        let records = vec![(0u32, vec![1.0, 2.0, 3.0])];

        let mut axis = HashMap::new();
        axis.insert(0u32, vec![10.0, 30.0, 50.0]);
        let mut out: Vec<(Channel, FieldConfig)> = Vec::new();
        push_cdf_field(
            &field("ch1@freq_vlf"),
            &records,
            Some(&axis),
            None,
            &epoch_map,
            &lsk,
            &mut out,
        );
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].0.freq, 10.0);
        assert_eq!(out[1].0.freq, 30.0);
        assert_eq!(out[2].0.freq, 50.0);
        assert_eq!(out[1].0.bin_width, 20.0);
        assert_eq!(out[0].0.value, 1.0);
        assert_eq!(out[2].0.value, 3.0);

        let mut scalar: Vec<(Channel, FieldConfig)> = Vec::new();
        push_cdf_field(
            &field("x"),
            &records,
            None,
            None,
            &epoch_map,
            &lsk,
            &mut scalar,
        );
        assert_eq!(scalar.len(), 1);
        assert_eq!(scalar[0].0.freq, 0.0);
        assert_eq!(scalar[0].0.bin_width, 0.0);
    }

    #[test]
    fn calibrated_spectrum_divides_by_the_interpolated_curve() {
        use crate::archivar::{Aperture, Channel, FieldConfig, LeapSeconds};

        let fc = FieldConfig {
            key: "ch1@freq_vlf@cal".into(),
            name: "b_nt2_hz".into(),
            band_id: None,
            kernel: 0,
            channel_ref: Some(0),
            tau: 0.0,
            absorption: 0.0,
            advection: 0.0,
            unit: "nT^2/Hz".into(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
            aperture: Aperture::None,
        };
        let lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let mut epoch_map = HashMap::new();
        epoch_map.insert(0u32, 1_000_000_000.0);
        let records = vec![(0u32, vec![1.0, 4.0])];
        let mut axis = HashMap::new();
        axis.insert(0u32, vec![10.0, 20.0]);
        let curve = vec![(10.0, 1.0), (20.0, 2.0)];
        let mut out: Vec<(Channel, FieldConfig)> = Vec::new();
        push_cdf_field(
            &fc,
            &records,
            Some(&axis),
            Some(&curve),
            &epoch_map,
            &lsk,
            &mut out,
        );
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].0.freq, 10.0);
        let cal0 = 1.0_f64 * NT_PER_T;
        assert!((out[0].0.value - 1.0 / (cal0 * cal0)).abs() < 1e-30);
        let cal1 = 2.0_f64 * NT_PER_T;
        assert!((out[1].0.value - 4.0 / (cal1 * cal1)).abs() < 1e-30);

        let mut off = HashMap::new();
        off.insert(0u32, vec![5.0]); // below the curve support → flagged, not extrapolated
        let mut none: Vec<(Channel, FieldConfig)> = Vec::new();
        push_cdf_field(
            &fc,
            &records,
            Some(&off),
            Some(&curve),
            &epoch_map,
            &lsk,
            &mut none,
        );
        assert_eq!(none.len(), 0);
    }

    #[test]
    fn extracts_athabasca_channels_when_present() {
        use crate::archivar::{Aperture, Channel, FieldConfig, LeapSeconds};

        let path = crate::temp_path("ath.cdf");
        let Ok(bytes) = std::fs::read(path) else {
            return;
        };
        if bytes.len() < 4 || bytes[..4] != MAGIC {
            return;
        }
        let marker = be_u32(&bytes, 4).expect("marker");
        let owned;
        let file_bytes: &[u8] = if marker == 0x0000_ffff {
            &bytes
        } else {
            owned = decompress_whole_file(&bytes).expect("gunzip");
            &owned
        };
        let file = CdfFile::parse(file_bytes).expect("cdf3");
        let epoch_var = cdf_epoch_var(&file).expect("epoch");
        let epoch_map = file.epoch_map(file_bytes, epoch_var).expect("epochs");

        let ch1 = file.var("ch1").expect("ch1");
        let freq = file.var("freq_vlf").expect("freq_vlf");
        assert_eq!(file.num_values(freq), 1024);
        let ch1_records = file.var_records(file_bytes, ch1).expect("ch1 records");
        let axis: HashMap<u32, Vec<f64>> = file
            .var_records(file_bytes, freq)
            .expect("freq records")
            .into_iter()
            .collect();

        let fc = FieldConfig {
            key: "ch1@freq_vlf".into(),
            name: "athabasca_vlf_ns_v2_hz".into(),
            band_id: None,
            kernel: 0,
            channel_ref: Some(0),
            tau: 0.0,
            absorption: 0.0,
            advection: 0.0,
            unit: "V^2/Hz".into(),
            freq: 0.0,
            bin_width: 0.0,
            fold: None,
            aperture: Aperture::None,
        };
        let lsk = LeapSeconds {
            delta_t_a: 32.184,
            deltas: vec![(37.0, 1483228800.0)],
        };
        let mut out: Vec<(Channel, FieldConfig)> = Vec::new();
        push_cdf_field(
            &fc,
            &ch1_records[..1],
            Some(&axis),
            None,
            &epoch_map,
            &lsk,
            &mut out,
        );
        assert_eq!(out.len(), 1024);
        assert!(out.iter().all(|(c, _)| c.name == "athabasca_vlf_ns_v2_hz"));
        assert!((out[0].0.freq - 10.0).abs() < 1e-6, "first bin is 10 Hz");
        assert!(out.iter().all(|(c, _)| c.freq > 0.0 && c.bin_width > 0.0));
        assert!(out.iter().all(|(c, _)| c.epoch > 0.0));
    }
}
