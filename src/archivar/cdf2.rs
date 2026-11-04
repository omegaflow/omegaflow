//! CDF 2.5/2.6 reader.
//!
//! CDF 3.0 lives in `cdf.rs`; the GFZ CHAMP products arrive as CDF 2.6 with
//! 32-bit descriptor offsets and a 4-byte record size, so the two layouts do
//! not share a parser. The offsets here follow the CDF 2.6 descriptor layout:
//! CDR size at 8, GDR at the CDR's own GDR word, VDR chain from GDR, VXR chain
//! from the VDR, data in VVR blocks.

pub const MAGIC_CDF2: [[u8; 4]; 2] = [[0xCD, 0xF2, 0x60, 0x02], [0xCE, 0xF2, 0x60, 0x02]];

pub const EPOCH_UNIX_OFFSET_S: f64 = 62_167_219_200.0;

const SECTION_VDR_ZVAR: u32 = 8;
const SECTION_VVR: u32 = 7;
const SECTION_CVVR: u32 = 13;

pub fn type_size(t: u32) -> Option<usize> {
    match t {
        1 | 11 | 41 | 51 | 52 => Some(1),
        2 | 12 => Some(2),
        4 | 14 | 21 | 44 => Some(4),
        8 | 22 | 31 | 32 | 45 => Some(8),
        _ => None,
    }
}

fn be32(b: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(o..o + 4)?.try_into().ok()?))
}

fn be32s(b: &[u8], o: usize) -> Option<i64> {
    Some(be32(b, o)? as i32 as i64)
}

#[derive(Clone, Debug)]
pub struct Cdf2Var {
    pub name: String,
    pub section: u32,
    pub data_type: u32,
    pub max_rec: usize,
    pub head_vxr: usize,
    pub num_elements: usize,
    pub dim_sizes: Vec<u32>,
}

impl Cdf2Var {
    pub fn values_per_record(&self) -> usize {
        if self.dim_sizes.is_empty() {
            self.num_elements
        } else {
            self.dim_sizes.iter().map(|d| *d as usize).product::<usize>() * self.num_elements
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cdf2File {
    pub version: (u32, u32, u32),
    pub little_endian: bool,
    pub vars: Vec<Cdf2Var>,
}

fn decode_record(b: &[u8], t: u32, count: usize, little: bool) -> Option<Vec<f64>> {
    let size = type_size(t)?;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let off = i * size;
        let raw = b.get(off..off + size)?;
        let v = match (t, little) {
            (31, false) => f64::from_be_bytes(raw.try_into().ok()?),
            (31, true) => f64::from_le_bytes(raw.try_into().ok()?),
            (21, false) => f32::from_be_bytes(raw.try_into().ok()?) as f64,
            (21, true) => f32::from_le_bytes(raw.try_into().ok()?) as f64,
            (22, false) => f64::from_be_bytes(raw.try_into().ok()?),
            (22, true) => f64::from_le_bytes(raw.try_into().ok()?),
            (45, false) => f64::from_be_bytes(raw.try_into().ok()?),
            (45, true) => f64::from_le_bytes(raw.try_into().ok()?),
            (44, false) => f32::from_be_bytes(raw.try_into().ok()?) as f64,
            (44, true) => f32::from_le_bytes(raw.try_into().ok()?) as f64,
            (4, false) => i32::from_be_bytes(raw.try_into().ok()?) as f64,
            (4, true) => i32::from_le_bytes(raw.try_into().ok()?) as f64,
            (8, false) => i64::from_be_bytes(raw.try_into().ok()?) as f64,
            (8, true) => i64::from_le_bytes(raw.try_into().ok()?) as f64,
            (1, _) => raw[0] as i8 as f64,
            (11, _) | (41, _) => raw[0] as f64,
            (51, _) | (52, _) => raw[0] as f64,
            (2, false) => i16::from_be_bytes(raw.try_into().ok()?) as f64,
            (2, true) => i16::from_le_bytes(raw.try_into().ok()?) as f64,
            (12, false) => u16::from_be_bytes(raw.try_into().ok()?) as f64,
            (12, true) => u16::from_le_bytes(raw.try_into().ok()?) as f64,
            (14, false) => u32::from_be_bytes(raw.try_into().ok()?) as f64,
            (14, true) => u32::from_le_bytes(raw.try_into().ok()?) as f64,
            _ => return None,
        };
        out.push(v);
    }
    Some(out)
}

impl Cdf2File {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 16 || !MAGIC_CDF2.iter().any(|m| *m == bytes[0..4]) {
            return Err("magic is not a CDF 2.x magic".to_string());
        }
        let compressed = be32(bytes, 4).ok_or("no whole-file flag")?;
        if compressed != 0x0000_ffff {
            return Err("whole file is compressed".to_string());
        }
        let cdr_size = be32(bytes, 8).ok_or("no CDR size")? as usize;
        if cdr_size < 16 || 8 + cdr_size > bytes.len() {
            return Err("CDR size out of range".to_string());
        }
        let cdr = &bytes[12..8 + cdr_size];
        let gdr_off = be32(cdr, 4).ok_or("no GDR offset")? as usize;
        let version = be32(cdr, 8).ok_or("no version")?;
        let release = be32(cdr, 12).ok_or("no release")?;
        let increment = be32(cdr, 32).ok_or("no increment")?;
        if version != 2 {
            return Err(format!("CDF version {version} is not 2.x"));
        }
        let encoding = be32(cdr, 16).ok_or("no encoding")?;
        let little = match encoding {
            1 | 2 => false,
            3 | 6 => true,
            other => return Err(format!("encoding {other} not handled")),
        };

        let gdr_size = be32(bytes, gdr_off).ok_or("no GDR size")? as usize;
        if gdr_size < 40 || gdr_off + gdr_size > bytes.len() {
            return Err("GDR size out of range".to_string());
        }
        let gdr = &bytes[gdr_off + 4..gdr_off + gdr_size];
        let first_zvar = be32s(gdr, 8).ok_or("no first z variable")?;
        let num_zvar = be32s(gdr, 36).ok_or("no z variable count")?;

        let mut vars = Vec::new();
        let mut next = first_zvar;
        let mut guard = 0i64;
        while next > 0 && (next as usize) < bytes.len() && guard <= num_zvar {
            guard += 1;
            let off = next as usize;
            let size = be32(bytes, off).ok_or("no VDR size")? as usize;
            if size < 132 || off + size > bytes.len() {
                return Err(format!("VDR at {off} size {size} out of range"));
            }
            let v = &bytes[off + 4..off + size];
            let section = be32(v, 0).ok_or("no section")?;
            if section != SECTION_VDR_ZVAR {
                return Err(format!("section {section} is not a z variable"));
            }
            let next_vdr = be32s(v, 4).ok_or("no next VDR")?;
            let data_type = be32(v, 8).ok_or("no data type")?;
            let max_rec = be32(v, 12).ok_or("no max record")? as usize;
            let head_vxr = be32s(v, 16).ok_or("no VXR head")? as usize;
            let num_elements = be32(v, 44).ok_or("no element count")? as usize;
            let num_dims = be32(v, 124).ok_or("no dim count")? as usize;
            let mut dim_sizes = Vec::with_capacity(num_dims);
            for i in 0..num_dims {
                dim_sizes.push(be32(v, 128 + 4 * i).ok_or("no dim size")?);
            }
            let raw_name = v.get(60..124).ok_or("no name slot")?;
            let end = raw_name.iter().position(|b| *b == 0).unwrap_or(64);
            let name = String::from_utf8_lossy(&raw_name[..end]).trim().to_string();
            vars.push(Cdf2Var {
                name,
                section,
                data_type,
                max_rec,
                head_vxr,
                num_elements,
                dim_sizes,
            });
            next = next_vdr;
        }
        if vars.is_empty() {
            return Err("no z variable in the descriptor chain".to_string());
        }
        Ok(Self {
            version: (version, release, increment),
            little_endian: little,
            vars,
        })
    }

    pub fn var(&self, name: &str) -> Option<&Cdf2Var> {
        self.vars.iter().find(|v| v.name == name)
    }

    fn vxr_entries(
        &self,
        bytes: &[u8],
        pos: usize,
        out: &mut Vec<(usize, usize, usize)>,
    ) -> Result<(), String> {
        let size = be32(bytes, pos).ok_or("no VXR size")? as usize;
        if size < 16 || pos + size > bytes.len() {
            return Err(format!("VXR at {pos} out of range"));
        }
        let x = &bytes[pos + 4..pos + size];
        let next = be32s(x, 4).ok_or("no next VXR")?;
        let num_ent = be32(x, 8).ok_or("no VXR entry count")? as usize;
        let used = be32(x, 12).ok_or("no VXR used count")? as usize;
        if used > num_ent {
            return Err("VXR uses more entries than it holds".to_string());
        }
        for i in 0..used {
            let start = be32(x, 16 + 4 * i).ok_or("no VXR start")? as usize;
            let end = be32(x, 16 + 4 * num_ent + 4 * i).ok_or("no VXR end")? as usize;
            let rec = be32(x, 16 + 8 * num_ent + 4 * i).ok_or("no VXR record")? as usize;
            let kind = be32(bytes, rec + 4).ok_or("no VXR record type")?;
            if kind == 6 {
                self.vxr_entries(bytes, rec, out)?;
            } else {
                out.push((start, end, rec));
            }
        }
        if next > 0 {
            self.vxr_entries(bytes, next as usize, out)?;
        }
        Ok(())
    }

    /// Records as (record number, values). Absent records stay absent.
    pub fn var_records(&self, bytes: &[u8], var: &Cdf2Var) -> Result<Vec<(usize, Vec<f64>)>, String> {
        let mut entries = Vec::new();
        if var.head_vxr > 0 {
            self.vxr_entries(bytes, var.head_vxr, &mut entries)?;
        }
        let per_record = var.values_per_record();
        let type_bytes = type_size(var.data_type).ok_or("unknown data type")?;
        let record_bytes = type_bytes * per_record;
        let mut out = Vec::new();
        for (start, end, rec) in entries {
            let size = be32(bytes, rec).ok_or("no VVR size")? as usize;
            if rec + size > bytes.len() {
                return Err(format!("VVR at {rec} out of range"));
            }
            let block = &bytes[rec + 4..rec + size];
            let kind = be32(block, 0).ok_or("no VVR section")?;
            let data = match kind {
                SECTION_VVR => &block[4..],
                SECTION_CVVR => return Err("compressed variable record".to_string()),
                other => return Err(format!("VV section {other} not handled")),
            };
            for record in start..=end {
                let idx = record - start;
                let slice = data
                    .get(idx * record_bytes..(idx + 1) * record_bytes)
                    .ok_or("VVR carries fewer records than the index says")?;
                let values = decode_record(slice, var.data_type, per_record, self.little_endian)
                    .ok_or("record does not decode")?;
                out.push((record, values));
            }
        }
        out.sort_by_key(|(r, _)| *r);
        Ok(out)
    }

    /// Per-record time in Unix seconds from an EPOCH variable.
    pub fn epoch_seconds(&self, bytes: &[u8]) -> Result<Vec<(usize, f64)>, String> {
        let var = self.var("EPOCH").ok_or("no EPOCH variable")?;
        let records = self.var_records(bytes, var)?;
        Ok(records
            .into_iter()
            .filter_map(|(r, v)| {
                let first = *v.first()?;
                if !first.is_finite() || first <= 0.0 {
                    return None;
                }
                Some((r, first / 1000.0 - EPOCH_UNIX_OFFSET_S))
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_foreign_bytes() {
        assert!(Cdf2File::parse(b"").is_err());
        assert!(Cdf2File::parse(b"CDF30001").is_err());
        assert!(Cdf2File::parse(b"CEF26002\x00\x00\xff\xff").is_err());
    }

    #[test]
    fn epoch_offset_is_the_cdf_zero_epoch() {
        assert_eq!(EPOCH_UNIX_OFFSET_S, 62_167_219_200.0);
    }
}
