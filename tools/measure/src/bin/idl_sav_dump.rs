use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::exit;

const T_U8: i32 = 1;
const T_I16: i32 = 2;
const T_I32: i32 = 3;
const T_F32: i32 = 4;
const T_F64: i32 = 5;
const T_C8: i32 = 6;
const T_STR: i32 = 7;
const T_C16: i32 = 9;
const T_PTR: i32 = 10;
const T_OBJPTR: i32 = 11;
const T_U16: i32 = 12;
const T_U32: i32 = 13;
const T_I64: i32 = 14;
const T_U64: i32 = 15;

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Reader { buf, pos: 0 }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self
            .pos
            .checked_add(n)
            .ok_or_else(|| format!("offset overflow at {}", self.pos))?;
        if end > self.buf.len() {
            return Err(format!(
                "end of file: {} bytes requested at {}",
                n, self.pos
            ));
        }
        let slice = &self.buf[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn skip(&mut self, n: usize) -> Result<(), String> {
        self.take(n).map(|_| ())
    }

    fn align32(&mut self) {
        let rem = self.pos % 4;
        if rem != 0 {
            self.pos += 4 - rem;
        }
    }

    fn read_i32(&mut self) -> Result<i32, String> {
        let b = self.take(4)?;
        Ok(i32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn read_i64(&mut self) -> Result<i64, String> {
        let b = self.take(8)?;
        let mut a = [0u8; 8];
        a.copy_from_slice(b);
        Ok(i64::from_be_bytes(a))
    }

    fn read_u64(&mut self) -> Result<u64, String> {
        let b = self.take(8)?;
        let mut a = [0u8; 8];
        a.copy_from_slice(b);
        Ok(u64::from_be_bytes(a))
    }

    fn read_f32(&mut self) -> Result<f32, String> {
        let b = self.take(4)?;
        Ok(f32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn read_f64(&mut self) -> Result<f64, String> {
        let b = self.take(8)?;
        let mut a = [0u8; 8];
        a.copy_from_slice(b);
        Ok(f64::from_be_bytes(a))
    }

    fn read_i16_slot(&mut self) -> Result<i16, String> {
        let b = self.take(4)?;
        Ok(i16::from_be_bytes([b[2], b[3]]))
    }

    fn read_u16_slot(&mut self) -> Result<u16, String> {
        let b = self.take(4)?;
        Ok(u16::from_be_bytes([b[2], b[3]]))
    }

    fn read_byte_slot(&mut self) -> Result<u8, String> {
        let b = self.take(4)?;
        Ok(b[0])
    }

    fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_i32()?;
        if len < 0 {
            return Err(format!("string length {} at {}", len, self.pos));
        }
        if len == 0 {
            return Ok(String::new());
        }
        let b = self.take(len as usize)?;
        let s: String = b.iter().map(|&c| c as char).collect();
        self.align32();
        Ok(s)
    }

    fn read_string_data(&mut self) -> Result<String, String> {
        let len = self.read_i32()?;
        if len < 0 {
            return Err(format!("string length {} at {}", len, self.pos));
        }
        if len == 0 {
            return Ok(String::new());
        }
        let len2 = self.read_i32()?;
        if len2 < 0 {
            return Err(format!("string length {} at {}", len2, self.pos));
        }
        let b = self.take(len2 as usize)?;
        let s: String = b.iter().map(|&c| c as char).collect();
        self.align32();
        Ok(s)
    }
}

struct ArrayDesc {
    nbytes: i64,
    nelements: i64,
    dims: Vec<i64>,
}

struct TagDesc {
    name: String,
    typecode: i32,
    array: bool,
    structure: bool,
}

struct StructDesc {
    tagtable: Vec<TagDesc>,
    arrtable: Vec<(String, ArrayDesc)>,
    structtable: Vec<(String, usize)>,
}

struct NumArray {
    typecode: i32,
    dims: Vec<i64>,
    values: Vec<f64>,
}

struct StructData {
    rows: Vec<Vec<(String, Node)>>,
}

enum Node {
    U8(u8),
    I16(i16),
    I32(i32),
    F32(f32),
    F64(f64),
    U16(u16),
    U32(u32),
    I64(i64),
    U64(u64),
    Str(String),
    Ptr(i32),
    Array(NumArray),
    StrArray(Vec<String>),
    Struct(StructData),
    Null,
}

fn read_arraydesc(r: &mut Reader) -> Result<ArrayDesc, String> {
    let arrstart = r.read_i32()?;
    if arrstart == 8 {
        r.skip(4)?;
        let nbytes = r.read_i32()? as i64;
        let nelements = r.read_i32()? as i64;
        let _ndims = r.read_i32()?;
        r.skip(8)?;
        let nmax = r.read_i32()?;
        let mut dims = Vec::new();
        for _ in 0..nmax {
            dims.push(r.read_i32()? as i64);
        }
        Ok(ArrayDesc {
            nbytes,
            nelements,
            dims,
        })
    } else if arrstart == 18 {
        r.skip(8)?;
        let nbytes = r.read_u64()? as i64;
        let nelements = r.read_u64()? as i64;
        let _ndims = r.read_i32()?;
        r.skip(8)?;
        let mut dims = Vec::new();
        for _ in 0..8 {
            let v = r.read_i32()?;
            if v != 0 {
                return Err(format!("array descriptor fill {} at {}", v, r.pos));
            }
            dims.push(r.read_i32()? as i64);
        }
        Ok(ArrayDesc {
            nbytes,
            nelements,
            dims,
        })
    } else {
        Err(format!("array start {} at {}", arrstart, r.pos))
    }
}

fn read_structdesc(
    r: &mut Reader,
    arena: &mut Vec<StructDesc>,
    map: &mut HashMap<String, usize>,
) -> Result<usize, String> {
    let structstart = r.read_i32()?;
    if structstart != 9 {
        return Err(format!("structure start {} at {}", structstart, r.pos));
    }
    let name = r.read_string()?;
    let predef = r.read_i32()?;
    let ntags = r.read_i32()?;
    let _nbytes = r.read_i32()?;
    let predef_bit = predef & 1 != 0;
    let inherits = predef & 2 != 0;
    let is_super = predef & 4 != 0;
    if predef_bit {
        return map
            .get(&name)
            .copied()
            .ok_or_else(|| format!("predefined structure {} at {}", name, r.pos));
    }
    let mut raw = Vec::new();
    for _ in 0..ntags {
        let offset = r.read_i32()?;
        if offset == -1 {
            let _ = r.read_u64()?;
        }
        let typecode = r.read_i32()?;
        let flags = r.read_i32()?;
        raw.push((typecode, flags & 4 != 0, flags & 32 != 0));
    }
    let mut tagtable = Vec::new();
    for (typecode, array, structure) in raw {
        let tname = r.read_string()?;
        tagtable.push(TagDesc {
            name: tname,
            typecode,
            array,
            structure,
        });
    }
    let mut arrtable = Vec::new();
    for t in &tagtable {
        if t.array {
            arrtable.push((t.name.clone(), read_arraydesc(r)?));
        }
    }
    let mut structtable = Vec::new();
    for t in &tagtable {
        if t.structure {
            let idx = read_structdesc(r, arena, map)?;
            structtable.push((t.name.clone(), idx));
        }
    }
    if inherits || is_super {
        let _classname = r.read_string()?;
        let nsup = r.read_i32()?;
        for _ in 0..nsup {
            let _ = r.read_string()?;
        }
        for _ in 0..nsup {
            let _ = read_structdesc(r, arena, map)?;
        }
    }
    let idx = arena.len();
    arena.push(StructDesc {
        tagtable,
        arrtable,
        structtable,
    });
    map.insert(name, idx);
    Ok(idx)
}

fn read_data(r: &mut Reader, typecode: i32) -> Result<Node, String> {
    match typecode {
        T_U8 => {
            let count = r.read_i32()?;
            if count != 1 {
                return Err(format!("byte count {} at {}", count, r.pos));
            }
            Ok(Node::U8(r.read_byte_slot()?))
        }
        T_I16 => Ok(Node::I16(r.read_i16_slot()?)),
        T_I32 => Ok(Node::I32(r.read_i32()?)),
        T_F32 => Ok(Node::F32(r.read_f32()?)),
        T_F64 => Ok(Node::F64(r.read_f64()?)),
        T_C8 => {
            let _re = r.read_f32()?;
            let _im = r.read_f32()?;
            Ok(Node::Null)
        }
        T_STR => Ok(Node::Str(r.read_string_data()?)),
        T_U16 => Ok(Node::U16(r.read_u16_slot()?)),
        T_U32 => Ok(Node::U32(r.read_u32()?)),
        T_I64 => Ok(Node::I64(r.read_i64()?)),
        T_U64 => Ok(Node::U64(r.read_u64()?)),
        T_PTR | T_OBJPTR => Ok(Node::Ptr(r.read_i32()?)),
        other => Err(format!("type code {} at {}", other, r.pos)),
    }
}

fn read_array(r: &mut Reader, typecode: i32, ad: &ArrayDesc) -> Result<Node, String> {
    let n = ad.nelements as usize;
    let nb = ad.nbytes as usize;
    let node = match typecode {
        T_U8 => {
            let _stored = r.read_i32()?;
            let b = r.take(nb)?;
            let values: Vec<f64> = b.iter().map(|&x| x as f64).collect();
            Node::Array(NumArray {
                typecode,
                dims: ad.dims.clone(),
                values,
            })
        }
        T_I16 | T_U16 => {
            let raw = r.take(nb * 2)?;
            let mut values = Vec::with_capacity(n);
            for i in 0..n {
                let o = i * 4 + 2;
                let v = if typecode == T_I16 {
                    i16::from_be_bytes([raw[o], raw[o + 1]]) as f64
                } else {
                    u16::from_be_bytes([raw[o], raw[o + 1]]) as f64
                };
                values.push(v);
            }
            Node::Array(NumArray {
                typecode,
                dims: ad.dims.clone(),
                values,
            })
        }
        T_I32 | T_U32 => {
            let b = r.take(nb)?;
            let mut values = Vec::with_capacity(n);
            for i in 0..n {
                let o = i * 4;
                let v = if typecode == T_I32 {
                    i32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as f64
                } else {
                    u32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as f64
                };
                values.push(v);
            }
            Node::Array(NumArray {
                typecode,
                dims: ad.dims.clone(),
                values,
            })
        }
        T_F32 => {
            let b = r.take(nb)?;
            let mut values = Vec::with_capacity(n);
            for i in 0..n {
                let o = i * 4;
                values.push(f32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as f64);
            }
            Node::Array(NumArray {
                typecode,
                dims: ad.dims.clone(),
                values,
            })
        }
        T_F64 => {
            let b = r.take(nb)?;
            let mut values = Vec::with_capacity(n);
            for i in 0..n {
                let o = i * 8;
                let mut a = [0u8; 8];
                a.copy_from_slice(&b[o..o + 8]);
                values.push(f64::from_be_bytes(a));
            }
            Node::Array(NumArray {
                typecode,
                dims: ad.dims.clone(),
                values,
            })
        }
        T_I64 | T_U64 => {
            let b = r.take(nb)?;
            let mut values = Vec::with_capacity(n);
            for i in 0..n {
                let o = i * 8;
                let mut a = [0u8; 8];
                a.copy_from_slice(&b[o..o + 8]);
                let v = if typecode == T_I64 {
                    i64::from_be_bytes(a) as f64
                } else {
                    u64::from_be_bytes(a) as f64
                };
                values.push(v);
            }
            Node::Array(NumArray {
                typecode,
                dims: ad.dims.clone(),
                values,
            })
        }
        T_C8 | T_C16 => {
            let _ = r.take(nb)?;
            Node::Null
        }
        T_STR => {
            let mut values = Vec::with_capacity(n);
            for _ in 0..n {
                match read_data(r, T_STR)? {
                    Node::Str(s) => values.push(s),
                    _ => return Err(format!("string array element at {}", r.pos)),
                }
            }
            Node::StrArray(values)
        }
        T_PTR | T_OBJPTR => {
            let mut values = Vec::with_capacity(n);
            for _ in 0..n {
                values.push(r.read_i32()? as f64);
            }
            Node::Array(NumArray {
                typecode,
                dims: ad.dims.clone(),
                values,
            })
        }
        other => return Err(format!("array type code {} at {}", other, r.pos)),
    };
    r.align32();
    Ok(node)
}

fn read_structure(
    r: &mut Reader,
    array_desc: &ArrayDesc,
    sd: &StructDesc,
    arena: &[StructDesc],
) -> Result<StructData, String> {
    let nrows = array_desc.nelements as usize;
    let mut rows = Vec::with_capacity(nrows);
    for _ in 0..nrows {
        let mut fields = Vec::with_capacity(sd.tagtable.len());
        for tag in &sd.tagtable {
            let node = if tag.structure {
                let ad = sd
                    .arrtable
                    .iter()
                    .find(|(n, _)| n == &tag.name)
                    .map(|(_, a)| a)
                    .ok_or_else(|| format!("structure tag {} lacks array descriptor", tag.name))?;
                let idx = sd
                    .structtable
                    .iter()
                    .find(|(n, _)| n == &tag.name)
                    .map(|(_, i)| *i)
                    .ok_or_else(|| {
                        format!("structure tag {} lacks structure descriptor", tag.name)
                    })?;
                Node::Struct(read_structure(r, ad, &arena[idx], arena)?)
            } else if tag.array {
                let ad = sd
                    .arrtable
                    .iter()
                    .find(|(n, _)| n == &tag.name)
                    .map(|(_, a)| a)
                    .ok_or_else(|| format!("array tag {} lacks array descriptor", tag.name))?;
                read_array(r, tag.typecode, ad)?
            } else {
                read_data(r, tag.typecode)?
            };
            fields.push((tag.name.clone(), node));
        }
        rows.push(fields);
    }
    Ok(StructData { rows })
}

fn read_variable(
    r: &mut Reader,
    arena: &mut Vec<StructDesc>,
    map: &mut HashMap<String, usize>,
) -> Result<(String, Node), String> {
    let varname = r.read_string()?;
    let typecode = r.read_i32()?;
    let varflags = r.read_i32()?;
    if varflags & 2 != 0 {
        return Err(format!("system variable {} at {}", varname, r.pos));
    }
    let is_array = varflags & 4 != 0;
    let is_struct = varflags & 32 != 0;
    if typecode == 0 {
        return Ok((varname, Node::Null));
    }
    if is_struct {
        let ad = read_arraydesc(r)?;
        let idx = read_structdesc(r, arena, map)?;
        let varstart = r.read_i32()?;
        if varstart != 7 {
            return Err(format!("variable start {} at {}", varstart, r.pos));
        }
        let data = read_structure(r, &ad, &arena[idx], arena)?;
        Ok((varname, Node::Struct(data)))
    } else if is_array {
        let ad = read_arraydesc(r)?;
        let varstart = r.read_i32()?;
        if varstart != 7 {
            return Err(format!("variable start {} at {}", varstart, r.pos));
        }
        let data = read_array(r, typecode, &ad)?;
        Ok((varname, data))
    } else {
        let varstart = r.read_i32()?;
        if varstart != 7 {
            return Err(format!("variable start {} at {}", varstart, r.pos));
        }
        let data = read_data(r, typecode)?;
        Ok((varname, data))
    }
}

fn parse_sav(buf: &[u8]) -> Result<Vec<(String, Node)>, String> {
    let mut r = Reader::new(buf);
    let signature = r.take(2)?;
    if signature != b"SR" {
        return Err(format!("signature {:?} at 0", signature));
    }
    let recfmt = r.take(2)?;
    if recfmt != [0, 4] {
        return Err(format!("record format {:02x} {:02x}", recfmt[0], recfmt[1]));
    }
    let mut arena: Vec<StructDesc> = Vec::new();
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut vars = Vec::new();
    let mut ended = false;
    while r.pos < buf.len() {
        if r.pos + 16 > buf.len() {
            return Err(format!("record header at {} runs past end", r.pos));
        }
        let rec_start = r.pos;
        let rectype = r.read_i32()?;
        let lo = r.read_u32()? as u64;
        let hi = r.read_u32()? as u64;
        let nextrec = lo + (hi << 32);
        r.skip(4)?;
        if rectype == 6 {
            ended = true;
            break;
        }
        if rectype == 2 {
            let (name, node) = read_variable(&mut r, &mut arena, &mut map)?;
            vars.push((name, node));
        }
        if nextrec <= rec_start as u64 {
            return Err(format!("record at {} points to {}", rec_start, nextrec));
        }
        r.pos = nextrec as usize;
        if r.pos > buf.len() {
            return Err(format!(
                "record at {} points past end {}",
                rec_start,
                buf.len()
            ));
        }
    }
    if !ended {
        return Err("end marker absent".to_string());
    }
    Ok(vars)
}

fn emit<'a>(
    name: &str,
    node: &'a Node,
    elev: &mut Option<&'a NumArray>,
    azim: &mut Option<&'a NumArray>,
) {
    match node {
        Node::Struct(sd) => {
            for (i, row) in sd.rows.iter().enumerate() {
                for (tname, child) in row {
                    let label = if sd.rows.len() == 1 {
                        tname.clone()
                    } else {
                        format!("{}[{}]", tname, i)
                    };
                    emit(&label, child, elev, azim);
                }
            }
        }
        Node::Array(a) => {
            if name == "FULL_ELEVATION" {
                *elev = Some(a);
            }
            if name == "FULL_AZIMUTH" {
                *azim = Some(a);
            }
            println!(
                "{} = <array typecode {} dims {:?} n={}>",
                name,
                a.typecode,
                a.dims,
                a.values.len()
            );
            if is_value_array(name) {
                println!("{} = [{}]", name, format_values(a));
            }
        }
        Node::StrArray(v) => println!("{} = <string array n={}>", name, v.len()),
        Node::Null => println!("{} = null", name),
        Node::U8(v) => println!("{} = {}", name, v),
        Node::I16(v) => println!("{} = {}", name, v),
        Node::I32(v) => println!("{} = {}", name, v),
        Node::F32(v) => println!("{} = {}", name, v),
        Node::F64(v) => println!("{} = {}", name, v),
        Node::U16(v) => println!("{} = {}", name, v),
        Node::U32(v) => println!("{} = {}", name, v),
        Node::I64(v) => println!("{} = {}", name, v),
        Node::U64(v) => println!("{} = {}", name, v),
        Node::Str(v) => println!("{} = {}", name, v),
        Node::Ptr(v) => println!("{} = ptr({})", name, v),
    }
}

fn is_value_array(name: &str) -> bool {
    matches!(
        name,
        "OPTICAL_PROJECTION"
            | "OPTICAL_ORIENTATION"
            | "CCD_CENTER"
            | "IMG_FLIP"
            | "BYTSCL_VALUES"
            | "FULL_MAP_ALTITUDE"
            | "BIN_ELEVATION"
            | "BIN_AZIMUTH"
    )
}

fn format_values(a: &NumArray) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(a.values.len());
    for v in &a.values {
        if a.typecode == T_F32 {
            parts.push(format!("{}", *v as f32));
        } else {
            parts.push(format!("{}", v));
        }
    }
    parts.join(", ")
}

fn write_values(path: &str, a: &NumArray) -> Result<(), String> {
    let mut f = File::create(path).map_err(|e| format!("{}: {}", path, e))?;
    for v in &a.values {
        if a.typecode == T_F32 {
            writeln!(f, "{}", *v as f32).map_err(|e| format!("{}: {}", path, e))?;
        } else {
            writeln!(f, "{}", v).map_err(|e| format!("{}: {}", path, e))?;
        }
    }
    Ok(())
}

fn run(input: &str, out_elev: &str, out_azim: &str) -> Result<(), String> {
    let buf = fs::read(input).map_err(|e| format!("{}: {}", input, e))?;
    let vars = parse_sav(&buf)?;
    let mut elev: Option<&NumArray> = None;
    let mut azim: Option<&NumArray> = None;
    for (name, node) in &vars {
        emit(name, node, &mut elev, &mut azim);
    }
    let elev = elev.ok_or_else(|| "FULL_ELEVATION absent".to_string())?;
    let azim = azim.ok_or_else(|| "FULL_AZIMUTH absent".to_string())?;
    write_values(out_elev, elev)?;
    write_values(out_azim, azim)?;
    println!(
        "FULL_ELEVATION -> {} ({} values)",
        out_elev,
        elev.values.len()
    );
    println!(
        "FULL_AZIMUTH -> {} ({} values)",
        out_azim,
        azim.values.len()
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        Some(s) => s.clone(),
        None => "state/river/themis/kapu_20240909_skymap.sav".to_string(),
    };
    let out_elev = match args.get(2) {
        Some(s) => s.clone(),
        None => "state/river/themis/kapu_20240909_full_elevation.txt".to_string(),
    };
    let out_azim = match args.get(3) {
        Some(s) => s.clone(),
        None => "state/river/themis/kapu_20240909_full_azimuth.txt".to_string(),
    };
    if let Err(detail) = run(&input, &out_elev, &out_azim) {
        eprintln!("idl_sav_dump: {}", detail);
        exit(2);
    }
}
