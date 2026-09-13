use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(HashMap<String, Json>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(map) => map.get(key),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_scalar_string(&self) -> Option<String> {
        match self {
            Json::Str(s) => Some(s.clone()),
            Json::Num(n) if n.is_finite() && n.fract() == 0.0 => Some(format!("{}", *n as i64)),
            _ => None,
        }
    }
}

pub fn parse(text: &str) -> Option<Json> {
    let mut p = Parser {
        b: text.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.b.len() {
        return None;
    }
    Some(v)
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn value(&mut self) -> Option<Json> {
        self.ws();
        match self.b.get(self.i)? {
            b'{' => self.object(),
            b'[' => self.array(),
            b'"' => self.string().map(Json::Str),
            b't' => {
                self.lit("true")?;
                Some(Json::Bool(true))
            }
            b'f' => {
                self.lit("false")?;
                Some(Json::Bool(false))
            }
            b'n' => {
                self.lit("null")?;
                Some(Json::Null)
            }
            _ => self.number(),
        }
    }

    fn lit(&mut self, s: &str) -> Option<()> {
        if self.b[self.i..].starts_with(s.as_bytes()) {
            self.i += s.len();
            Some(())
        } else {
            None
        }
    }

    fn object(&mut self) -> Option<Json> {
        self.i += 1;
        let mut map = HashMap::new();
        self.ws();
        if self.b.get(self.i) == Some(&b'}') {
            self.i += 1;
            return Some(Json::Obj(map));
        }
        loop {
            self.ws();
            let key = self.string()?;
            self.ws();
            if self.b.get(self.i) != Some(&b':') {
                return None;
            }
            self.i += 1;
            let val = self.value()?;
            map.insert(key, val);
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(Json::Obj(map))
    }

    fn array(&mut self) -> Option<Json> {
        self.i += 1;
        let mut out = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b']') {
            self.i += 1;
            return Some(Json::Arr(out));
        }
        loop {
            out.push(self.value()?);
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    break;
                }
                _ => return None,
            }
        }
        Some(Json::Arr(out))
    }

    fn string(&mut self) -> Option<String> {
        if self.b.get(self.i) != Some(&b'"') {
            return None;
        }
        self.i += 1;
        let mut out = String::new();
        loop {
            let c = *self.b.get(self.i)?;
            match c {
                b'"' => {
                    self.i += 1;
                    return Some(out);
                }
                b'\\' => {
                    self.i += 1;
                    let e = *self.b.get(self.i)?;
                    self.i += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let cp = self.hex4()?;
                            out.push(char::from_u32(cp as u32)?);
                        }
                        _ => return None,
                    }
                }
                _ => {
                    let len = utf8_len(c);
                    if len == 0 || self.i + len > self.b.len() {
                        return None;
                    }
                    out.push_str(std::str::from_utf8(&self.b[self.i..self.i + len]).ok()?);
                    self.i += len;
                }
            }
        }
    }

    fn hex4(&mut self) -> Option<u16> {
        if self.i + 4 > self.b.len() {
            return None;
        }
        let mut v: u16 = 0;
        for k in 0..4 {
            let d = (self.b[self.i + k] as char).to_digit(16)? as u16;
            v = v * 16 + d;
        }
        self.i += 4;
        Some(v)
    }

    fn number(&mut self) -> Option<Json> {
        let start = self.i;
        if self.b.get(self.i) == Some(&b'-') {
            self.i += 1;
        }
        while self.i < self.b.len()
            && (self.b[self.i].is_ascii_digit()
                || matches!(self.b[self.i], b'.' | b'e' | b'E' | b'+' | b'-'))
        {
            self.i += 1;
        }
        if self.i == start {
            return None;
        }
        let s = std::str::from_utf8(&self.b[start..self.i]).ok()?;
        s.parse::<f64>().ok().map(Json::Num)
    }
}

fn utf8_len(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b >> 5 == 0b110 {
        2
    } else if b >> 4 == 0b1110 {
        3
    } else if b >> 3 == 0b11110 {
        4
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nested_objects_and_arrays() {
        let v = parse(r#"{"a":[1,2,{"b":"x"}],"c":true}"#).unwrap();
        assert_eq!(v.get("c"), Some(&Json::Bool(true)));
        assert_eq!(v.get("a").and_then(|a| a.as_arr()).unwrap().len(), 3);
        let nested = &v.get("a").unwrap().as_arr().unwrap()[2];
        assert_eq!(nested.get("b").and_then(|b| b.as_str()), Some("x"));
    }

    #[test]
    fn parses_string_escapes_and_unicode() {
        let v = parse(r#""a\"b\n\u0041""#).unwrap();
        assert_eq!(v.as_str(), Some("a\"b\nA"));
    }

    #[test]
    fn renders_numeric_ids_and_strings() {
        let v = parse(r#"{"id":20210005208,"bib":"2021AJ....161..105P"}"#).unwrap();
        assert_eq!(
            v.get("id").and_then(|i| i.as_scalar_string()),
            Some("20210005208".to_string())
        );
        assert_eq!(
            v.get("bib").and_then(|i| i.as_scalar_string()),
            Some("2021AJ....161..105P".to_string())
        );
    }

    #[test]
    fn rejects_trailing_text_and_truncation() {
        assert!(parse("{} x").is_none());
        assert!(parse("{\"a\":").is_none());
        assert!(parse("").is_none());
    }
}
