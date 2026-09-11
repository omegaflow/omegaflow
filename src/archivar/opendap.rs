#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DapType {
    Byte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Float32,
    Float64,
    Str,
    Url,
}

impl DapType {
    fn from_keyword(kw: &str) -> Option<DapType> {
        match kw {
            "Byte" => Some(DapType::Byte),
            "Int16" => Some(DapType::Int16),
            "UInt16" => Some(DapType::UInt16),
            "Int32" => Some(DapType::Int32),
            "UInt32" => Some(DapType::UInt32),
            "Float32" => Some(DapType::Float32),
            "Float64" => Some(DapType::Float64),
            "String" => Some(DapType::Str),
            "Url" => Some(DapType::Url),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DapDim {
    pub name: String,
    pub len: u32,
}

#[derive(Clone, Debug)]
pub struct DapAttr {
    pub name: String,
    pub dap_type: DapType,
    pub raw: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct DapVarDecl {
    pub name: String,
    pub dap_type: DapType,
    pub dims: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct DapSchema {
    pub name: String,
    pub dims: Vec<DapDim>,
    pub vars: Vec<DapVarDecl>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DapData {
    Byte(Vec<u8>),
    Int16(Vec<i16>),
    UInt16(Vec<u16>),
    Int32(Vec<i32>),
    UInt32(Vec<u32>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
    Str(Vec<String>),
    Url(Vec<String>),
}

#[derive(Clone, Debug)]
pub struct DapVar {
    pub name: String,
    pub dap_type: DapType,
    pub dim_ids: Vec<usize>,
    pub attrs: Vec<DapAttr>,
    pub data: DapData,
}

#[derive(Clone, Debug)]
pub struct DapFile {
    pub name: String,
    pub dims: Vec<DapDim>,
    pub vars: Vec<DapVar>,
    pub gattrs: Vec<DapAttr>,
}

#[derive(Debug)]
pub struct DapAttrs {
    pub global: Vec<DapAttr>,
    pub per_var: Vec<(String, Vec<DapAttr>)>,
}

#[derive(Debug)]
pub enum DapNote {
    Keyword { word: String },
    Shape { var: String },
    Attr { name: String },
    EndAtByte { off: usize },
    CountMismatch { var: String, want: u64, got: u32 },
    Sequence { name: String },
}

fn be_u32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn be_u64(b: &[u8]) -> u64 {
    u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

fn tokenize(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        } else if matches!(ch, '{' | '}' | '[' | ']' | ';' | '=') {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            out.push(ch.to_string());
        } else {
            cur.push(ch);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

struct DdsCursor {
    toks: Vec<String>,
    pos: usize,
    dims: Vec<DapDim>,
    vars: Vec<DapVarDecl>,
}

impl DdsCursor {
    fn peek(&self) -> Option<&str> {
        self.toks.get(self.pos).map(|s| s.as_str())
    }

    fn take_word(&mut self) -> Option<String> {
        let t = self.toks.get(self.pos)?.clone();
        self.pos += 1;
        Some(t)
    }

    fn take_if(&mut self, s: &str) -> bool {
        if self.toks.get(self.pos).map(|x| x.as_str()) == Some(s) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, s: &str) -> Result<(), DapNote> {
        if self.take_if(s) {
            Ok(())
        } else {
            Err(DapNote::Keyword {
                word: s.to_string(),
            })
        }
    }

    fn register_dim(&mut self, name: &str, len: u32) {
        if !self.dims.iter().any(|d| d.name == name) {
            self.dims.push(DapDim {
                name: name.to_string(),
                len,
            });
        }
    }

    fn parse_decls(&mut self) -> Result<(), DapNote> {
        loop {
            match self.peek() {
                None | Some("}") => return Ok(()),
                _ => self.parse_decl()?,
            }
        }
    }

    fn parse_decl(&mut self) -> Result<(), DapNote> {
        let kw = self.take_word().ok_or(DapNote::Keyword {
            word: "".to_string(),
        })?;
        match kw.as_str() {
            "Grid" | "Structure" => {
                self.expect("{")?;
                loop {
                    match self.peek() {
                        Some("}") => {
                            self.take_word().ok_or(DapNote::Keyword {
                                word: "}".to_string(),
                            })?;
                            break;
                        }
                        Some("ARRAY:") | Some("MAPS:") => {
                            self.take_word().ok_or(DapNote::Keyword {
                                word: "}".to_string(),
                            })?;
                        }
                        Some("ARRAY") | Some("MAPS") => {
                            self.take_word().ok_or(DapNote::Keyword {
                                word: "}".to_string(),
                            })?;
                            let _ = self.take_if(":");
                        }
                        _ => self.parse_decl()?,
                    }
                }
                let _ = self.take_word();
                let _ = self.take_if(";");
                Ok(())
            }
            "Sequence" => {
                let mut depth = 0usize;
                loop {
                    match self.peek() {
                        Some("{") => {
                            self.take_word().ok_or(DapNote::Keyword {
                                word: "}".to_string(),
                            })?;
                            depth += 1;
                        }
                        Some("}") => {
                            self.take_word().ok_or(DapNote::Keyword {
                                word: "}".to_string(),
                            })?;
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        Some(_) => {
                            self.take_word().ok_or(DapNote::Keyword {
                                word: "}".to_string(),
                            })?;
                        }
                        None => {
                            return Err(DapNote::Keyword {
                                word: "}".to_string(),
                            })
                        }
                    }
                }
                let name = match self.take_word() {
                    Some(n) => n,
                    None => String::new(),
                };
                let _ = self.take_if(";");
                Err(DapNote::Sequence { name })
            }
            _ => {
                let dap_type = DapType::from_keyword(&kw)
                    .ok_or_else(|| DapNote::Keyword { word: kw.clone() })?;
                let name = self
                    .take_word()
                    .ok_or(DapNote::Keyword { word: kw.clone() })?;
                let mut dims = Vec::new();
                while self.peek() == Some("[") {
                    self.take_word()
                        .ok_or(DapNote::Shape { var: name.clone() })?;
                    let dim_name = self
                        .take_word()
                        .ok_or(DapNote::Shape { var: name.clone() })?;
                    self.expect("=")?;
                    let len_tok = self
                        .take_word()
                        .ok_or(DapNote::Shape { var: name.clone() })?;
                    let len: u32 = len_tok
                        .parse()
                        .map_err(|_| DapNote::Shape { var: name.clone() })?;
                    self.expect("]")?;
                    self.register_dim(&dim_name, len);
                    dims.push(dim_name);
                }
                self.expect(";")?;
                self.vars.push(DapVarDecl {
                    name,
                    dap_type,
                    dims,
                });
                Ok(())
            }
        }
    }
}

pub fn parse_dds(text: &str) -> Result<DapSchema, DapNote> {
    let mut cur = DdsCursor {
        toks: tokenize(text),
        pos: 0,
        dims: Vec::new(),
        vars: Vec::new(),
    };
    cur.expect("Dataset")?;
    cur.expect("{")?;
    cur.parse_decls()?;
    cur.expect("}")?;
    let name = match cur.take_word() {
        Some(n) => n,
        None => String::new(),
    };
    let _ = cur.take_if(";");
    Ok(DapSchema {
        name,
        dims: cur.dims,
        vars: cur.vars,
    })
}

fn tokenize_das(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                let mut s = String::new();
                while let Some(c) = chars.next() {
                    if c == '\\' {
                        if let Some(&n) = chars.peek() {
                            s.push(n);
                            chars.next();
                        }
                    } else if c == '"' {
                        break;
                    } else {
                        s.push(c);
                    }
                }
                if !s.is_empty() {
                    out.push(s);
                }
            }
            '{' | '}' | ';' => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                out.push(ch.to_string());
            }
            c if c.is_whitespace() || c == ',' => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

struct DasCursor {
    toks: Vec<String>,
    pos: usize,
}

impl DasCursor {
    fn peek(&self) -> Option<&str> {
        self.toks.get(self.pos).map(|s| s.as_str())
    }

    fn next_is_brace(&self) -> bool {
        self.toks.get(self.pos + 1).map(|s| s.as_str()) == Some("{")
    }

    fn take_word(&mut self) -> Option<String> {
        let t = self.toks.get(self.pos)?.clone();
        self.pos += 1;
        Some(t)
    }

    fn take_if(&mut self, s: &str) -> bool {
        if self.toks.get(self.pos).map(|x| x.as_str()) == Some(s) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, s: &str) -> Result<(), DapNote> {
        if self.take_if(s) {
            Ok(())
        } else {
            Err(DapNote::Keyword {
                word: s.to_string(),
            })
        }
    }

    fn parse_entries(
        &mut self,
        into: &mut Vec<DapAttr>,
        per_var: &mut Vec<(String, Vec<DapAttr>)>,
    ) -> Result<(), DapNote> {
        loop {
            match self.peek() {
                None | Some("}") => return Ok(()),
                Some(_) if self.next_is_brace() => {
                    let cname = self.take_word().ok_or(DapNote::Keyword {
                        word: "}".to_string(),
                    })?;
                    self.expect("{")?;
                    let mut attrs = Vec::new();
                    self.parse_entries(&mut attrs, per_var)?;
                    self.expect("}")?;
                    if cname == "NC_GLOBAL" {
                        into.extend(attrs);
                    } else {
                        per_var.push((cname, attrs));
                    }
                }
                Some(_) => {
                    let t = self.take_word().ok_or(DapNote::Keyword {
                        word: "}".to_string(),
                    })?;
                    let dap_type = DapType::from_keyword(&t)
                        .ok_or_else(|| DapNote::Keyword { word: t.clone() })?;
                    let name = self.take_word().ok_or(DapNote::Keyword { word: t })?;
                    let raw = self.parse_attr_values(dap_type, &name)?;
                    self.expect(";")?;
                    into.push(DapAttr {
                        name,
                        dap_type,
                        raw,
                    });
                }
            }
        }
    }

    fn parse_attr_values(&mut self, dap_type: DapType, name: &str) -> Result<Vec<u8>, DapNote> {
        let mut raw = Vec::new();
        if self.take_if("{") {
            loop {
                match self.peek() {
                    None => {
                        return Err(DapNote::Attr {
                            name: name.to_string(),
                        })
                    }
                    Some("}") => {
                        self.take_word().ok_or(DapNote::Attr {
                            name: name.to_string(),
                        })?;
                        break;
                    }
                    Some(_) => {
                        let tok = self.take_word().ok_or(DapNote::Attr {
                            name: name.to_string(),
                        })?;
                        encode_attr(&mut raw, dap_type, &tok, name)?;
                    }
                }
            }
        } else {
            loop {
                match self.peek() {
                    None | Some(";") => break,
                    Some(_) => {
                        let tok = self.take_word().ok_or(DapNote::Attr {
                            name: name.to_string(),
                        })?;
                        encode_attr(&mut raw, dap_type, &tok, name)?;
                    }
                }
            }
        }
        Ok(raw)
    }
}

fn encode_attr(raw: &mut Vec<u8>, dap_type: DapType, tok: &str, name: &str) -> Result<(), DapNote> {
    match dap_type {
        DapType::Str | DapType::Url => {
            if raw.is_empty() {
                raw.extend_from_slice(tok.as_bytes());
            }
        }
        _ => {
            let v: f64 = tok.parse().map_err(|_| DapNote::Attr {
                name: name.to_string(),
            })?;
            raw.extend_from_slice(&v.to_bits().to_be_bytes());
        }
    }
    Ok(())
}

pub fn parse_das(text: &str) -> Result<DapAttrs, DapNote> {
    let mut cur = DasCursor {
        toks: tokenize_das(text),
        pos: 0,
    };
    cur.expect("Attributes")?;
    cur.expect("{")?;
    let mut global = Vec::new();
    let mut per_var = Vec::new();
    cur.parse_entries(&mut global, &mut per_var)?;
    Ok(DapAttrs { global, per_var })
}

struct Xdr<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Xdr<'a> {
    fn new(buf: &'a [u8]) -> Xdr<'a> {
        Xdr { buf, pos: 0 }
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

    fn align4(&mut self) {
        self.pos = (self.pos + 3) & !3;
    }
}

fn shape_len(schema: &DapSchema, v: &DapVarDecl) -> u64 {
    if v.dims.is_empty() {
        return 1;
    }
    let mut n = 1u64;
    for d in &v.dims {
        match schema.dims.iter().find(|x| x.name == *d) {
            Some(dd) => n = n.saturating_mul(dd.len as u64),
            None => return 0,
        }
    }
    n
}

fn cardinal_count(cur: &mut Xdr, total: u64, var: &str) -> Result<u32, DapNote> {
    let c1 = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })?;
    let c2 = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })?;
    if c1 as u64 != total || c2 != c1 {
        return Err(DapNote::CountMismatch {
            var: var.to_string(),
            want: total,
            got: c1,
        });
    }
    Ok(c1)
}

fn decode_var(cur: &mut Xdr, v: &DapVarDecl, schema: &DapSchema) -> Result<DapData, DapNote> {
    let total = shape_len(schema, v);
    let is_array = !v.dims.is_empty();
    match v.dap_type {
        DapType::Byte => {
            if is_array {
                let n = cardinal_count(cur, total, &v.name)? as usize;
                let raw = cur.take(n).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                cur.align4();
                Ok(DapData::Byte(raw.to_vec()))
            } else {
                let b = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })? as u8;
                Ok(DapData::Byte(vec![b]))
            }
        }
        DapType::Int16 => {
            if is_array {
                let n = cardinal_count(cur, total, &v.name)? as usize;
                let raw = cur.take(n * 4).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::Int16(
                    raw.chunks_exact(4)
                        .map(|c| i32::from_be_bytes([c[0], c[1], c[2], c[3]]) as i16)
                        .collect(),
                ))
            } else {
                let b = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })? as i32 as i16;
                Ok(DapData::Int16(vec![b]))
            }
        }
        DapType::UInt16 => {
            if is_array {
                let n = cardinal_count(cur, total, &v.name)? as usize;
                let raw = cur.take(n * 4).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::UInt16(
                    raw.chunks_exact(4)
                        .map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]) as u16)
                        .collect(),
                ))
            } else {
                let b = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })? as u16;
                Ok(DapData::UInt16(vec![b]))
            }
        }
        DapType::Int32 => {
            if is_array {
                let n = cardinal_count(cur, total, &v.name)? as usize;
                let raw = cur.take(n * 4).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::Int32(
                    raw.chunks_exact(4)
                        .map(|c| i32::from_be_bytes([c[0], c[1], c[2], c[3]]))
                        .collect(),
                ))
            } else {
                let b = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })? as i32;
                Ok(DapData::Int32(vec![b]))
            }
        }
        DapType::UInt32 => {
            if is_array {
                let n = cardinal_count(cur, total, &v.name)? as usize;
                let raw = cur.take(n * 4).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::UInt32(raw.chunks_exact(4).map(be_u32).collect()))
            } else {
                let b = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::UInt32(vec![b]))
            }
        }
        DapType::Float32 => {
            if is_array {
                let n = cardinal_count(cur, total, &v.name)? as usize;
                let raw = cur.take(n * 4).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::Float32(
                    raw.chunks_exact(4)
                        .map(|c| f32::from_bits(be_u32(c)))
                        .collect(),
                ))
            } else {
                let raw = cur.take(4).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::Float32(vec![f32::from_bits(be_u32(raw))]))
            }
        }
        DapType::Float64 => {
            if is_array {
                let n = cardinal_count(cur, total, &v.name)? as usize;
                let raw = cur.take(n * 8).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::Float64(
                    raw.chunks_exact(8)
                        .map(|c| f64::from_bits(be_u64(c)))
                        .collect(),
                ))
            } else {
                let raw = cur.take(8).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                Ok(DapData::Float64(vec![f64::from_bits(be_u64(raw))]))
            }
        }
        DapType::Str | DapType::Url => {
            let mut out = Vec::new();
            if is_array {
                let n = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })? as usize;
                for _ in 0..n {
                    let len = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })? as usize;
                    let s = cur.take(len).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                    cur.align4();
                    out.push(String::from_utf8_lossy(s).into_owned());
                }
            } else {
                let len = cur.u32().ok_or(DapNote::EndAtByte { off: cur.pos })? as usize;
                let s = cur.take(len).ok_or(DapNote::EndAtByte { off: cur.pos })?;
                cur.align4();
                out.push(String::from_utf8_lossy(s).into_owned());
            }
            if v.dap_type == DapType::Str {
                Ok(DapData::Str(out))
            } else {
                Ok(DapData::Url(out))
            }
        }
    }
}

pub fn decode_dods(schema: &DapSchema, data: &[u8]) -> Result<Vec<DapData>, DapNote> {
    let mut cur = Xdr::new(data);
    let mut out = Vec::with_capacity(schema.vars.len());
    for v in &schema.vars {
        out.push(decode_var(&mut cur, v, schema)?);
    }
    Ok(out)
}

fn strip_header(bytes: &[u8]) -> &[u8] {
    let needle = b"\nData:\n";
    if let Some(pos) = bytes.windows(needle.len()).position(|w| w == needle) {
        return &bytes[pos + needle.len()..];
    }
    let head = b"Data:\n";
    if bytes.starts_with(head) {
        return &bytes[head.len()..];
    }
    bytes
}

fn lookup_attrs(attrs: &DapAttrs, name: &str) -> Vec<DapAttr> {
    for (k, v) in &attrs.per_var {
        let suffix = k.strip_suffix(name).map_or(false, |p| p.ends_with('.'));
        if k == name || suffix {
            return v.clone();
        }
    }
    Vec::new()
}

pub fn decode(dds_text: &str, das_text: &str, dods_bytes: &[u8]) -> Result<DapFile, DapNote> {
    let schema = parse_dds(dds_text)?;
    let attrs = parse_das(das_text)?;
    let data = strip_header(dods_bytes);
    let values = decode_dods(&schema, data)?;
    let mut vars = Vec::with_capacity(schema.vars.len());
    for (decl, value) in schema.vars.into_iter().zip(values.into_iter()) {
        let mut dim_ids = Vec::with_capacity(decl.dims.len());
        for dn in &decl.dims {
            match schema.dims.iter().position(|d| &d.name == dn) {
                Some(i) => dim_ids.push(i),
                None => {
                    return Err(DapNote::Shape {
                        var: decl.name.clone(),
                    })
                }
            }
        }
        let vattrs = lookup_attrs(&attrs, &decl.name);
        vars.push(DapVar {
            name: decl.name,
            dap_type: decl.dap_type,
            dim_ids,
            attrs: vattrs,
            data: value,
        });
    }
    Ok(DapFile {
        name: schema.name,
        dims: schema.dims,
        vars,
        gattrs: attrs.global,
    })
}

impl DapFile {
    pub fn var(&self, name: &str) -> Option<&DapVar> {
        self.vars.iter().find(|v| v.name == name)
    }

    pub fn var_shape(&self, var: &DapVar) -> Vec<u64> {
        var.dim_ids
            .iter()
            .map(|&i| self.dims.get(i).map_or(0, |d| d.len as u64))
            .collect()
    }

    pub fn values_numeric(&self, name: &str) -> Option<Vec<f64>> {
        let v = self.var(name)?;
        match &v.data {
            DapData::Byte(b) => Some(b.iter().map(|&x| x as f64).collect()),
            DapData::Int16(x) => Some(x.iter().map(|&x| x as f64).collect()),
            DapData::UInt16(x) => Some(x.iter().map(|&x| x as f64).collect()),
            DapData::Int32(x) => Some(x.iter().map(|&x| x as f64).collect()),
            DapData::UInt32(x) => Some(x.iter().map(|&x| x as f64).collect()),
            DapData::Float32(x) => Some(x.iter().map(|&x| x as f64).collect()),
            DapData::Float64(x) => Some(x.clone()),
            DapData::Str(_) | DapData::Url(_) => None,
        }
    }

    pub fn values_text(&self, name: &str) -> Option<String> {
        let v = self.var(name)?;
        match &v.data {
            DapData::Str(s) | DapData::Url(s) => s.first().cloned(),
            _ => None,
        }
    }

    pub fn attr_num(&self, attr: &DapAttr) -> Option<f64> {
        match attr.dap_type {
            DapType::Str | DapType::Url => None,
            _ => Some(f64::from_bits(be_u64(attr.raw.get(0..8)?))),
        }
    }

    pub fn attr_text(&self, attr: &DapAttr) -> Option<String> {
        match attr.dap_type {
            DapType::Str | DapType::Url => Some(String::from_utf8_lossy(&attr.raw).into_owned()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u32b(x: u32) -> Vec<u8> {
        x.to_be_bytes().to_vec()
    }
    fn f32b(x: f32) -> Vec<u8> {
        x.to_bits().to_be_bytes().to_vec()
    }
    fn f64b(x: f64) -> Vec<u8> {
        x.to_bits().to_be_bytes().to_vec()
    }

    #[test]
    fn parses_dds_scalar_and_array() {
        let dds = "Dataset {\n    Float32 lat[lat = 361];\n    Int32 n;\n} demo;\n";
        let s = parse_dds(dds).unwrap();
        assert_eq!(s.name, "demo");
        assert_eq!(s.dims.len(), 1);
        assert_eq!(s.dims[0].name, "lat");
        assert_eq!(s.dims[0].len, 361);
        assert_eq!(s.vars.len(), 2);
        assert_eq!(s.vars[0].name, "lat");
        assert_eq!(s.vars[0].dap_type, DapType::Float32);
        assert_eq!(s.vars[0].dims, vec!["lat".to_string()]);
        assert_eq!(s.vars[1].name, "n");
        assert_eq!(s.vars[1].dap_type, DapType::Int32);
        assert!(s.vars[1].dims.is_empty());
    }

    #[test]
    fn parses_grid_flattened() {
        let dds = "Dataset {\n Grid {\n  ARRAY:\n    Float32 g[a = 2][b = 3];\n  MAPS:\n    Float32 a[a = 2];\n    Float32 b[b = 3];\n } g;\n} demo;\n";
        let s = parse_dds(dds).unwrap();
        assert_eq!(s.vars.len(), 3);
        assert_eq!(s.vars[0].name, "g");
        assert_eq!(s.vars[0].dims, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(s.vars[1].name, "a");
        assert_eq!(s.vars[2].name, "b");
    }

    #[test]
    fn decodes_float32_array_and_int32_scalar() {
        let dds = "Dataset {\n    Float32 lat[lat = 4];\n    Int32 n;\n} demo;\n";
        let mut data = Vec::new();
        data.extend(u32b(4));
        data.extend(u32b(4));
        for v in [90.0f32, 89.5, 89.0, 88.5] {
            data.extend(f32b(v));
        }
        data.extend(u32b(42));
        let file = decode(dds, "Attributes {\n}\n", &data).unwrap();
        assert_eq!(
            file.values_numeric("lat").unwrap(),
            vec![90.0, 89.5, 89.0, 88.5]
        );
        assert_eq!(file.values_numeric("n").unwrap(), vec![42.0]);
    }

    #[test]
    fn decodes_string_array() {
        let dds = "Dataset {\n    String name[name = 2];\n} demo;\n";
        let mut data = Vec::new();
        data.extend(u32b(2));
        data.extend(u32b(2));
        data.extend_from_slice(b"ab");
        data.extend([0u8, 0]);
        data.extend(u32b(1));
        data.push(b'c');
        data.extend([0u8, 0, 0]);
        let file = decode(dds, "Attributes {\n}\n", &data).unwrap();
        assert_eq!(
            file.var("name").unwrap().data,
            DapData::Str(vec!["ab".to_string(), "c".to_string()])
        );
    }

    #[test]
    fn decodes_float64_scalar() {
        let dds = "Dataset {\n    Float64 t;\n} demo;\n";
        let mut data = Vec::new();
        data.extend(f64b(1.5));
        let file = decode(dds, "Attributes {\n}\n", &data).unwrap();
        assert_eq!(file.values_numeric("t").unwrap(), vec![1.5]);
    }

    #[test]
    fn strips_dds_header_from_dods() {
        let dds = "Dataset {\n    Float32 lat[lat = 2];\n} demo;\n";
        let mut data = Vec::new();
        data.extend_from_slice(b"Dataset {\n    Float32 lat[lat = 2];\n} demo;\n\nData:\n");
        data.extend(u32b(2));
        data.extend(u32b(2));
        data.extend(f32b(1.0));
        data.extend(f32b(2.0));
        let file = decode(dds, "Attributes {\n}\n", &data).unwrap();
        assert_eq!(file.values_numeric("lat").unwrap(), vec![1.0, 2.0]);
    }

    #[test]
    fn parses_das_attributes() {
        let das_text = "Attributes {\n    Float64 earth_radius 6371229.0;\n    lat {\n        String units \"degrees_north\";\n        Float32 _FillValue 1.0e20;\n    }\n}\n";
        let file = decode(
            "Dataset {\n    Float32 lat[lat = 1];\n} demo;\n",
            das_text,
            &[0u8, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
        )
        .unwrap();
        let g = file
            .gattrs
            .iter()
            .find(|a| a.name == "earth_radius")
            .unwrap();
        assert_eq!(file.attr_num(g), Some(6371229.0));
        let lat = file.var("lat").unwrap();
        let units = lat.attrs.iter().find(|a| a.name == "units").unwrap();
        assert_eq!(file.attr_text(units), Some("degrees_north".to_string()));
        let fill = lat.attrs.iter().find(|a| a.name == "_FillValue").unwrap();
        assert_eq!(file.attr_num(fill), Some(1.0e20));
    }

    #[test]
    fn count_mismatch_is_named() {
        let dds = "Dataset {\n    Float32 lat[lat = 3];\n} demo;\n";
        let mut data = Vec::new();
        data.extend(u32b(2));
        data.extend(u32b(2));
        data.extend(f32b(1.0));
        data.extend(f32b(2.0));
        assert!(matches!(
            decode(dds, "Attributes {\n}\n", &data),
            Err(DapNote::CountMismatch { .. })
        ));
    }

    #[test]
    fn parses_nc_global_and_multi_value_attrs() {
        let das_text = "Attributes {\n    lat {\n        String units \"degrees_north\";\n        Float32 missing_value NaN;\n        Int32 Grib2_Parameter 0, 14, 0;\n    }\n    NC_GLOBAL {\n        String Conventions \"CF-1.6\";\n    }\n}\n";
        let file = decode(
            "Dataset {\n    Float32 lat[lat = 1];\n} demo;\n",
            das_text,
            &[0u8, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
        )
        .unwrap();
        assert!(file.gattrs.iter().any(|a| a.name == "Conventions"));
        let lat = file.var("lat").unwrap();
        let missing = lat
            .attrs
            .iter()
            .find(|a| a.name == "missing_value")
            .unwrap();
        assert!(file.attr_num(missing).unwrap().is_nan());
        let param = lat
            .attrs
            .iter()
            .find(|a| a.name == "Grib2_Parameter")
            .unwrap();
        assert_eq!(file.attr_num(param), Some(0.0));
    }
}
