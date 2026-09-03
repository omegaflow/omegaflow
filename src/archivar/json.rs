use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum JsonVal {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<JsonVal>),
    Obj(HashMap<String, JsonVal>),
}

pub fn parse_json(s: &str) -> Option<JsonVal> {
    let bytes = s.as_bytes();
    let start = (0..bytes.len()).find(|&i| bytes[i] == b'{' || bytes[i] == b'[')?;
    let mut p = JsonParser {
        chars: bytes,
        pos: start,
    };
    p.skip_ws();
    p.parse_value()
}

struct JsonParser<'a> {
    chars: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn skip_ws(&mut self) {
        while self.pos < self.chars.len() && (self.chars[self.pos] as char).is_whitespace() {
            self.pos += 1;
        }
    }
    fn parse_value(&mut self) -> Option<JsonVal> {
        self.skip_ws();
        if self.pos >= self.chars.len() {
            return None;
        }
        match self.chars[self.pos] {
            b'{' => self.parse_obj(),
            b'[' => self.parse_arr(),
            b'"' => self.parse_str().map(JsonVal::Str),
            b't' => {
                if self.chars[self.pos..].starts_with(b"true") {
                    self.pos += 4;
                    Some(JsonVal::Bool(true))
                } else {
                    None
                }
            }
            b'f' => {
                if self.chars[self.pos..].starts_with(b"false") {
                    self.pos += 5;
                    Some(JsonVal::Bool(false))
                } else {
                    None
                }
            }
            b'n' => {
                if self.chars[self.pos..].starts_with(b"null") {
                    self.pos += 4;
                    Some(JsonVal::Null)
                } else if self.chars[self.pos..].starts_with(b"nan") {
                    self.pos += 3;
                    Some(JsonVal::Num(f64::NAN))
                } else {
                    None
                }
            }
            b'N' => {
                if self.chars[self.pos..].starts_with(b"NaN") {
                    self.pos += 3;
                    Some(JsonVal::Num(f64::NAN))
                } else {
                    None
                }
            }
            b'I' => {
                if self.chars[self.pos..].starts_with(b"Infinity") {
                    self.pos += 8;
                    Some(JsonVal::Num(f64::INFINITY))
                } else {
                    None
                }
            }
            b'i' => {
                if self.chars[self.pos..].starts_with(b"inf") {
                    self.pos += 3;
                    Some(JsonVal::Num(f64::INFINITY))
                } else {
                    None
                }
            }
            b'-' if self.chars[self.pos..].starts_with(b"-Infinity") => {
                self.pos += 9;
                Some(JsonVal::Num(f64::NEG_INFINITY))
            }
            b'-' if self.chars[self.pos..].starts_with(b"-inf") => {
                self.pos += 4;
                Some(JsonVal::Num(f64::NEG_INFINITY))
            }
            _ => self.parse_num(),
        }
    }
    fn parse_obj(&mut self) -> Option<JsonVal> {
        self.pos += 1;
        self.skip_ws();
        let mut map = HashMap::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b'}' {
            self.pos += 1;
            return Some(JsonVal::Obj(map));
        }
        loop {
            self.skip_ws();
            let key = self.parse_str()?;
            self.skip_ws();
            if self.pos >= self.chars.len() || self.chars[self.pos] != b':' {
                return None;
            }
            self.pos += 1;
            let val = self.parse_value()?;
            map.insert(key, val);
            self.skip_ws();
            if self.pos >= self.chars.len() {
                return None;
            }
            match self.chars[self.pos] {
                b',' => {
                    self.pos += 1;
                }
                b'}' => {
                    self.pos += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(JsonVal::Obj(map))
    }
    fn parse_arr(&mut self) -> Option<JsonVal> {
        self.pos += 1;
        self.skip_ws();
        let mut arr = Vec::new();
        if self.pos < self.chars.len() && self.chars[self.pos] == b']' {
            self.pos += 1;
            return Some(JsonVal::Arr(arr));
        }
        loop {
            let val = self.parse_value()?;
            arr.push(val);
            self.skip_ws();
            if self.pos >= self.chars.len() {
                return None;
            }
            match self.chars[self.pos] {
                b',' => {
                    self.pos += 1;
                }
                b']' => {
                    self.pos += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(JsonVal::Arr(arr))
    }
    fn parse_str(&mut self) -> Option<String> {
        if self.pos >= self.chars.len() || self.chars[self.pos] != b'"' {
            return None;
        }
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == b'\\' && self.pos + 1 < self.chars.len() {
                self.pos += 1;
                match self.chars[self.pos] {
                    b'"' => {
                        s.push('"');
                        self.pos += 1;
                    }
                    b'\\' => {
                        s.push('\\');
                        self.pos += 1;
                    }
                    b'/' => {
                        s.push('/');
                        self.pos += 1;
                    }
                    b'n' => {
                        s.push('\n');
                        self.pos += 1;
                    }
                    b't' => {
                        s.push('\t');
                        self.pos += 1;
                    }
                    b'r' => {
                        s.push('\r');
                        self.pos += 1;
                    }
                    b'u' => {
                        self.pos += 1;
                        if self.pos + 4 <= self.chars.len() {
                            if let Ok(hex) =
                                std::str::from_utf8(&self.chars[self.pos..self.pos + 4])
                            {
                                if let Ok(cp) = u32::from_str_radix(hex, 16) {
                                    self.pos += 4;
                                    if (0xD800..=0xDBFF).contains(&cp)
                                        && self.pos + 6 <= self.chars.len()
                                        && self.chars[self.pos] == b'\\'
                                        && self.chars[self.pos + 1] == b'u'
                                    {
                                        if let Ok(hex_lo) = std::str::from_utf8(
                                            &self.chars[self.pos + 2..self.pos + 6],
                                        ) {
                                            if let Ok(lo) = u32::from_str_radix(hex_lo, 16) {
                                                if (0xDC00..=0xDFFF).contains(&lo) {
                                                    self.pos += 6;
                                                    let combined = 0x10000
                                                        + ((cp - 0xD800) << 10)
                                                        + (lo - 0xDC00);
                                                    if let Some(ch) = char::from_u32(combined) {
                                                        s.push(ch);
                                                    }
                                                    continue;
                                                }
                                            }
                                        }
                                    } else if let Some(ch) = char::from_u32(cp) {
                                        s.push(ch);
                                    }
                                } else {
                                    self.pos += 4;
                                }
                            } else {
                                self.pos += 4;
                            }
                        }
                    }
                    _ => {
                        self.pos += 1;
                    }
                }
            } else if c == b'"' {
                self.pos += 1;
                return Some(s);
            } else {
                let run_start = self.pos;
                while self.pos < self.chars.len()
                    && self.chars[self.pos] != b'"'
                    && self.chars[self.pos] != b'\\'
                {
                    self.pos += 1;
                }
                match std::str::from_utf8(&self.chars[run_start..self.pos]) {
                    Ok(t) => s.push_str(t),
                    Err(_) => {
                        s.push_str(&String::from_utf8_lossy(&self.chars[run_start..self.pos]))
                    }
                }
            }
        }
        None
    }
    fn parse_num(&mut self) -> Option<JsonVal> {
        let start = self.pos;
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c.is_ascii_digit() || c == b'-' || c == b'+' || c == b'.' || c == b'e' || c == b'E' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let s = std::str::from_utf8(&self.chars[start..self.pos]).ok()?;
        s.parse::<f64>().ok().map(JsonVal::Num)
    }
}

pub fn scalar_of(v: &JsonVal) -> Option<f64> {
    match v {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        JsonVal::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        _ => None,
    }
}

pub fn jpath_val<'a>(json: &'a JsonVal, path: &str) -> Option<&'a JsonVal> {
    if path.is_empty() || path == "." {
        return Some(json);
    }
    let mut current = json;
    for part in path.split('.') {
        if let JsonVal::Obj(map) = current {
            current = map.get(part)?;
        } else if let JsonVal::Arr(arr) = current {
            let raw_idx: i64 = part.parse().ok()?;
            let len = arr.len() as i64;
            let idx = if raw_idx < 0 {
                let actual = len + raw_idx;
                if actual < 0 {
                    return None;
                }
                actual as usize
            } else {
                raw_idx as usize
            };
            current = arr.get(idx)?;
        } else {
            return None;
        }
    }
    Some(current)
}

pub fn jnum(json: &JsonVal, key: &str) -> Option<f64> {
    if key.contains('.') {
        return jpath_val(json, key).and_then(scalar_of);
    }
    match json {
        JsonVal::Obj(map) => map.get(key).and_then(scalar_of),
        _ => None,
    }
}

pub fn jstr(json: &JsonVal, key: &str) -> Option<String> {
    if key.contains('.') {
        return jpath_val(json, key).and_then(|v| match v {
            JsonVal::Str(s) => Some(s.clone()),
            _ => None,
        });
    }
    match json {
        JsonVal::Obj(map) => match map.get(key) {
            Some(JsonVal::Str(s)) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn jpath(json: &JsonVal, path: &str) -> Option<f64> {
    if path == "." || path.is_empty() {
        return scalar_of(json);
    }
    jpath_val(json, path).and_then(scalar_of)
}

pub fn json_num(val: &JsonVal) -> Option<f64> {
    match val {
        JsonVal::Num(n) => Some(*n),
        JsonVal::Str(s) => s.parse().ok(),
        _ => None,
    }
}
