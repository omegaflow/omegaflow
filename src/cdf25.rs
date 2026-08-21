use crate::cdf::{decode_record, type_size, TYPE_EPOCH16};

pub const MAGIC25: [u8; 8] = [0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff, 0xff];
pub const MAGIC26: [u8; 8] = [0xcd, 0xf2, 0x60, 0x02, 0x00, 0x00, 0xff, 0xff];

const RECORD_GDR: u32 = 2;
const RECORD_VDR: u32 = 3;
const RECORD_VXR: u32 = 6;
const RECORD_VVR: u32 = 7;

pub enum Cdf25Note {
    NotCdf25 { magic: [u8; 8] },
    Encoding(u32),
    Release(u32),
    MultiFormat,
    EndAtByte { off: usize },
    RecordKind { off: usize, kind: u32 },
    DataType(u32),
    SparseRecord,
    Compressed,
    NoRecords,
    VxrSection(u32),
}

impl std::fmt::Debug for Cdf25Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cdf25Note::NotCdf25 { magic } => write!(f, "magic {:02x?} is not CDF 2.5/2.6", magic),
            Cdf25Note::Encoding(e) => write!(f, "encoding {e} is not network (1)"),
            Cdf25Note::Release(r) => {
                write!(f, "release {r} is pre-2.5 — the VDR layout is a parser gap")
            }
            Cdf25Note::MultiFormat => write!(f, "multi-format CDF — unread"),
            Cdf25Note::EndAtByte { off } => write!(f, "file ends at byte {off}"),
            Cdf25Note::RecordKind { off, kind } => {
                write!(f, "the record at {off} carries kind {kind}")
            }
            Cdf25Note::DataType(t) => write!(f, "data type {t} is unknown"),
            Cdf25Note::SparseRecord => write!(f, "sparse records — unread"),
            Cdf25Note::Compressed => write!(f, "the variable carries compression — unread"),
            Cdf25Note::NoRecords => write!(f, "the variable carries no records"),
            Cdf25Note::VxrSection(s) => write!(f, "VXR section type {s} is unknown"),
        }
    }
}

pub struct Cdf25Var {
    pub name: String,
    pub var_num: u32,
    pub data_type: u32,
    pub num_elements: u32,
    pub max_rec: i32,
    pub sparse: u32,
    pub compressed: bool,
    pub head_vxr: u32,
    pub dim_vary: Vec<u32>,
}

pub struct Cdf25File {
    pub version: (u32, u32, u32),
    pub encoding: u32,
    pub vars: Vec<Cdf25Var>,
    pub rdim_sizes: Vec<u32>,
}

fn be_u32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn be_i32(b: &[u8], off: usize) -> Option<i32> {
    Some(i32::from_be_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn trim_name(raw: &[u8]) -> String {
    let end = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
    String::from_utf8_lossy(&raw[..end]).into_owned()
}

pub struct Cdf25Block {
    pub start: u32,
    pub end: u32,
    pub offset: u32,
    pub kind: u32,
}

fn walk_vxrs(
    bytes: &[u8],
    offset: u32,
    out: &mut Vec<Cdf25Block>,
    depth: usize,
) -> Result<(), Cdf25Note> {
    if depth > 32 {
        return Err(Cdf25Note::VxrSection(0));
    }
    let off = offset as usize;
    let size = be_u32(bytes, off).ok_or(Cdf25Note::EndAtByte { off })? as usize;
    let rec = bytes
        .get(off..off + size)
        .ok_or(Cdf25Note::EndAtByte { off })?;
    let section = be_u32(rec, 4).ok_or(Cdf25Note::EndAtByte { off })?;
    if section != RECORD_VXR {
        return Err(Cdf25Note::VxrSection(section));
    }
    let next = be_u32(rec, 8).ok_or(Cdf25Note::EndAtByte { off })?;
    let num_ent = be_u32(rec, 12).ok_or(Cdf25Note::EndAtByte { off })? as usize;
    let used = be_u32(rec, 16).ok_or(Cdf25Note::EndAtByte { off })? as usize;
    for i in 0..used {
        let start = be_u32(rec, 20 + 4 * i).ok_or(Cdf25Note::EndAtByte { off })?;
        let end = be_u32(rec, 20 + 4 * num_ent + 4 * i).ok_or(Cdf25Note::EndAtByte { off })?;
        let rec_off = be_u32(rec, 20 + 8 * num_ent + 4 * i).ok_or(Cdf25Note::EndAtByte { off })?;
        let target = rec_off as usize;
        let kind = be_u32(bytes, target + 4).ok_or(Cdf25Note::EndAtByte { off: target })?;
        if kind == RECORD_VXR {
            walk_vxrs(bytes, rec_off, out, depth + 1)?;
        } else {
            out.push(Cdf25Block {
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

impl Cdf25File {
    pub fn parse(bytes: &[u8]) -> Result<Cdf25File, Cdf25Note> {
        let magic: [u8; 8] = bytes
            .get(0..8)
            .ok_or(Cdf25Note::EndAtByte { off: 0 })?
            .try_into()
            .ok()
            .unwrap();
        if magic != MAGIC25 && magic != MAGIC26 {
            return Err(Cdf25Note::NotCdf25 { magic });
        }
        let version = be_u32(bytes, 20).ok_or(Cdf25Note::EndAtByte { off: 20 })?;
        if version != 2 {
            return Err(Cdf25Note::NotCdf25 { magic });
        }
        let release = be_u32(bytes, 24).ok_or(Cdf25Note::EndAtByte { off: 24 })?;
        if release < 5 {
            return Err(Cdf25Note::Release(release));
        }
        let encoding = be_u32(bytes, 28).ok_or(Cdf25Note::EndAtByte { off: 28 })?;
        if encoding != 1 {
            return Err(Cdf25Note::Encoding(encoding));
        }
        let flag = be_u32(bytes, 32).ok_or(Cdf25Note::EndAtByte { off: 32 })?;
        if flag & 0x0002 == 0 {
            return Err(Cdf25Note::MultiFormat);
        }
        let increment = be_u32(bytes, 44).ok_or(Cdf25Note::EndAtByte { off: 44 })?;
        let gdr_off = be_u32(bytes, 16).ok_or(Cdf25Note::EndAtByte { off: 16 })? as usize;
        let gdr_size =
            be_u32(bytes, gdr_off).ok_or(Cdf25Note::EndAtByte { off: gdr_off })? as usize;
        let gdr = bytes
            .get(gdr_off + 4..gdr_off + gdr_size)
            .ok_or(Cdf25Note::EndAtByte { off: gdr_off })?;
        let kind = be_u32(gdr, 0).ok_or(Cdf25Note::EndAtByte { off: gdr_off })?;
        if kind != RECORD_GDR {
            return Err(Cdf25Note::RecordKind { off: gdr_off, kind });
        }
        let first_vdr = be_u32(gdr, 4).ok_or(Cdf25Note::EndAtByte { off: gdr_off })?;
        let num_vars = be_u32(gdr, 20).ok_or(Cdf25Note::EndAtByte { off: gdr_off })?;
        let num_rdim = be_u32(gdr, 32).ok_or(Cdf25Note::EndAtByte { off: gdr_off })? as usize;
        let mut rdim_sizes = Vec::with_capacity(num_rdim);
        for i in 0..num_rdim {
            rdim_sizes.push(be_u32(gdr, 56 + 4 * i).ok_or(Cdf25Note::EndAtByte { off: gdr_off })?);
        }
        let mut vars = Vec::with_capacity(num_vars as usize);
        let mut next = first_vdr;
        let mut guard = 0usize;
        while next != 0 && guard < num_vars as usize + 1 {
            guard += 1;
            let off = next as usize;
            let size = be_u32(bytes, off).ok_or(Cdf25Note::EndAtByte { off })? as usize;
            let rec = bytes
                .get(off + 4..off + size)
                .ok_or(Cdf25Note::EndAtByte { off })?;
            let section = be_u32(rec, 0).ok_or(Cdf25Note::EndAtByte { off })?;
            if section != RECORD_VDR {
                return Err(Cdf25Note::RecordKind { off, kind: section });
            }
            let next_vdr = be_u32(rec, 4).ok_or(Cdf25Note::EndAtByte { off })?;
            let data_type = be_u32(rec, 8).ok_or(Cdf25Note::EndAtByte { off })?;
            if type_size(data_type).is_none() && data_type != 51 && data_type != 52 {
                return Err(Cdf25Note::DataType(data_type));
            }
            let max_rec = be_i32(rec, 12).ok_or(Cdf25Note::EndAtByte { off })?;
            let head_vxr = be_u32(rec, 16).ok_or(Cdf25Note::EndAtByte { off })?;
            let flags = be_u32(rec, 24).ok_or(Cdf25Note::EndAtByte { off })?;
            let sparse = be_u32(rec, 28).ok_or(Cdf25Note::EndAtByte { off })?;
            let num_elements = be_u32(rec, 44).ok_or(Cdf25Note::EndAtByte { off })?;
            let var_num = be_u32(rec, 48).ok_or(Cdf25Note::EndAtByte { off })?;
            let name = trim_name(rec.get(60..124).ok_or(Cdf25Note::EndAtByte { off })?);
            let mut dim_vary = Vec::with_capacity(num_rdim);
            for i in 0..num_rdim {
                dim_vary.push(be_u32(rec, 124 + 4 * i).ok_or(Cdf25Note::EndAtByte { off })?);
            }
            vars.push(Cdf25Var {
                name,
                var_num,
                data_type,
                num_elements,
                max_rec,
                sparse,
                compressed: flags & 0x0004 != 0,
                head_vxr,
                dim_vary,
            });
            next = next_vdr;
        }
        Ok(Cdf25File {
            version: (version, release, increment),
            encoding,
            vars,
            rdim_sizes,
        })
    }

    pub fn var(&self, name: &str) -> Option<&Cdf25Var> {
        self.vars.iter().find(|v| v.name == name)
    }

    pub fn num_values(&self, var: &Cdf25Var) -> usize {
        let mut values = var.num_elements as usize;
        for (k, dim) in self.rdim_sizes.iter().enumerate() {
            if var.dim_vary.get(k).copied().unwrap_or(0) != 0 {
                values *= *dim as usize;
            }
        }
        values
    }

    pub fn blocks(&self, bytes: &[u8], var: &Cdf25Var) -> Result<Vec<Cdf25Block>, Cdf25Note> {
        if var.head_vxr == 0 {
            return Err(Cdf25Note::NoRecords);
        }
        if var.compressed {
            return Err(Cdf25Note::Compressed);
        }
        if var.sparse != 0 {
            return Err(Cdf25Note::SparseRecord);
        }
        let mut out = Vec::new();
        walk_vxrs(bytes, var.head_vxr, &mut out, 0)?;
        Ok(out)
    }

    pub fn var_records(&self, bytes: &[u8], var: &Cdf25Var) -> Result<Vec<Vec<f64>>, Cdf25Note> {
        let blocks = self.blocks(bytes, var)?;
        let num_values = self.num_values(var);
        let n_records = if var.max_rec < 0 {
            0
        } else {
            var.max_rec as usize + 1
        };
        let mut out = Vec::with_capacity(n_records);
        for b in &blocks {
            if b.kind != RECORD_VVR {
                return Err(Cdf25Note::RecordKind {
                    off: b.offset as usize,
                    kind: b.kind,
                });
            }
            let off = b.offset as usize;
            let size = be_u32(bytes, off).ok_or(Cdf25Note::EndAtByte { off })? as usize;
            let data = bytes
                .get(off + 8..off + size)
                .ok_or(Cdf25Note::EndAtByte { off })?;
            let mut record = b.start;
            let mut cursor = 0usize;
            while record <= b.end && record < n_records as u32 {
                let per = if var.data_type == TYPE_EPOCH16 {
                    num_values * 16
                } else {
                    num_values
                        * type_size(var.data_type).ok_or(Cdf25Note::DataType(var.data_type))?
                };
                let chunk = data.get(cursor..cursor + per).ok_or(Cdf25Note::EndAtByte {
                    off: off + 8 + cursor,
                })?;
                cursor += per;
                let values = decode_record(chunk, var.data_type, num_values, false)
                    .ok_or(Cdf25Note::DataType(var.data_type))?;
                out.push(values);
                record += 1;
            }
        }
        if out.len() < n_records {
            return Err(Cdf25Note::NoRecords);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1.0e-9 * a.abs().max(b.abs()).max(1.0)
    }

    #[test]
    fn rejects_foreign_bytes() {
        assert!(matches!(
            Cdf25File::parse(b"not a cdf"),
            Err(Cdf25Note::NotCdf25 { .. })
        ));
    }

    #[test]
    fn parses_wind_orbit_pre_2021_when_present() {
        let path = "/tmp/opencode/wi_or_pre_20210101.cdf";
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = Cdf25File::parse(&bytes).unwrap();
        assert_eq!(file.version, (2, 5, 22));
        assert_eq!(file.encoding, 1);
        assert_eq!(file.vars.len(), 21);
        assert_eq!(file.rdim_sizes, vec![3]);
        let epoch = file.var("Epoch").unwrap();
        assert_eq!(epoch.data_type, 31);
        assert_eq!(file.num_values(epoch), 1);
        let e_records = file.var_records(&bytes, epoch).unwrap();
        assert_eq!(e_records.len(), 144);
        assert!(near(e_records[0][0], 1609459200.0));
        assert!(near(e_records[143][0], 1609545000.0));
        let pos = file.var("GCI_POS").unwrap();
        assert_eq!(file.num_values(pos), 3);
        let p_records = file.var_records(&bytes, pos).unwrap();
        assert_eq!(p_records.len(), 144);
        assert!(near(p_records[0][0], 858616.9445));
        assert!(near(p_records[0][1], -1234654.0691));
        assert!(near(p_records[0][2], -491726.1634));
        let vel = file.var("GCI_VEL").unwrap();
        let v_records = file.var_records(&bytes, vel).unwrap();
        assert!(near(v_records[0][0], 0.187276));
        assert!(near(v_records[0][1], 0.042758));
        assert!(near(v_records[0][2], 0.049476));
    }

    #[test]
    fn parses_wind_orbit_pre_1994_26_when_present() {
        let path = "/tmp/opencode/wi_or_pre_19941110_v02.cdf";
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = Cdf25File::parse(&bytes).unwrap();
        assert_eq!(file.version, (2, 5, 22));
        assert_eq!(file.vars.len(), 11);
        let epoch = file.var("Epoch").unwrap();
        let e_records = file.var_records(&bytes, epoch).unwrap();
        assert_eq!(e_records.len(), 144);
        assert!(near(e_records[0][0], 784426038.0));
        let pos = file.var("GCI_POS").unwrap();
        let p_records = file.var_records(&bytes, pos).unwrap();
        assert!(near(p_records[0][0], -404822.66679));
        assert!(near(p_records[0][1], -270033.44126999995));
        assert!(near(p_records[0][2], -136094.33625));
        let vel = file.var("GCI_VEL").unwrap();
        let v_records = file.var_records(&bytes, vel).unwrap();
        assert!(near(v_records[0][0], 0.1974009));
        assert!(near(v_records[0][1], -0.059630199999999994));
        assert!(near(v_records[0][2], -0.0335771));
    }

    #[test]
    fn parses_wind_orbit_def_1994_when_present() {
        let path = "/tmp/opencode/wi_or_def_19941110_v02.cdf";
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let file = Cdf25File::parse(&bytes).unwrap();
        assert_eq!(file.vars.len(), 10);
        let epoch = file.var("Epoch").unwrap();
        let e_records = file.var_records(&bytes, epoch).unwrap();
        assert_eq!(e_records.len(), 144);
        let pos = file.var("GCI_POS").unwrap();
        let p_records = file.var_records(&bytes, pos).unwrap();
        assert_eq!(p_records.len(), 144);
        assert_eq!(p_records[0].len(), 3);
        let vel = file.var("GCI_VEL").unwrap();
        let v_records = file.var_records(&bytes, vel).unwrap();
        assert_eq!(v_records.len(), 144);
    }

    #[test]
    fn rejects_truncated_gdr() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&MAGIC25);
        bytes.extend_from_slice(&[0u8; 400]);
        bytes[16] = 0xff;
        bytes[23] = 2;
        bytes[27] = 5;
        bytes[31] = 1;
        bytes[35] = 2;
        assert!(matches!(
            Cdf25File::parse(&bytes),
            Err(Cdf25Note::EndAtByte { .. })
        ));
    }

    #[test]
    fn rejects_wrong_encoding() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&MAGIC25);
        bytes.extend_from_slice(&[0u8; 400]);
        bytes[23] = 2;
        bytes[27] = 5;
        bytes[35] = 2;
        bytes[31] = 6;
        assert!(matches!(
            Cdf25File::parse(&bytes),
            Err(Cdf25Note::Encoding(6))
        ));
    }
}
