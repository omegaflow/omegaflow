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

pub fn parse_json(input: &str) -> Option<JsonVal> {
    let mut p = Parser {
        b: input.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value()?;
    Some(v)
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && (self.b[self.i] as char).is_whitespace() {
            self.i += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }

    fn eat(&mut self, c: u8) -> bool {
        if self.peek() == Some(c) {
            self.i += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, c: u8) -> Option<()> {
        if self.eat(c) {
            Some(())
        } else {
            None
        }
    }

    fn value(&mut self) -> Option<JsonVal> {
        self.ws();
        match self.peek()? {
            b'{' => self.object(),
            b'[' => self.array(),
            b'"' => self.string().map(JsonVal::Str),
            b't' => self.literal(b"true", JsonVal::Bool(true)),
            b'f' => self.literal(b"false", JsonVal::Bool(false)),
            b'n' => self.literal(b"null", JsonVal::Null),
            _ => self.number(),
        }
    }

    fn literal(&mut self, w: &[u8], v: JsonVal) -> Option<JsonVal> {
        let end = self.i + w.len();
        if self.b.get(self.i..end)? == w {
            self.i = end;
            Some(v)
        } else {
            None
        }
    }

    fn object(&mut self) -> Option<JsonVal> {
        self.expect(b'{')?;
        let mut m = HashMap::new();
        self.ws();
        if self.eat(b'}') {
            return Some(JsonVal::Obj(m));
        }
        loop {
            self.ws();
            let key = self.string()?;
            self.ws();
            self.expect(b':')?;
            let val = self.value()?;
            m.insert(key, val);
            self.ws();
            if self.eat(b'}') {
                return Some(JsonVal::Obj(m));
            }
            self.expect(b',')?;
        }
    }

    fn array(&mut self) -> Option<JsonVal> {
        self.expect(b'[')?;
        let mut a = Vec::new();
        self.ws();
        if self.eat(b']') {
            return Some(JsonVal::Arr(a));
        }
        loop {
            let val = self.value()?;
            a.push(val);
            self.ws();
            if self.eat(b']') {
                return Some(JsonVal::Arr(a));
            }
            self.expect(b',')?;
        }
    }

    fn string(&mut self) -> Option<String> {
        self.expect(b'"')?;
        let mut s = String::new();
        loop {
            let c = self.peek()?;
            self.i += 1;
            match c {
                b'"' => return Some(s),
                b'\\' => {
                    let esc = self.peek()?;
                    self.i += 1;
                    match esc {
                        b'"' => s.push('"'),
                        b'\\' => s.push('\\'),
                        b'/' => s.push('/'),
                        b'n' => s.push('\n'),
                        b't' => s.push('\t'),
                        b'r' => s.push('\r'),
                        b'b' => s.push('\u{0008}'),
                        b'f' => s.push('\u{000C}'),
                        b'u' => self.unicode_escape(&mut s)?,
                        _ => {}
                    }
                }
                _ => {
                    let start = self.i - 1;
                    let mut j = self.i;
                    while j < self.b.len() {
                        let x = self.b[j];
                        if x == b'"' || x == b'\\' {
                            break;
                        }
                        j += 1;
                    }
                    s.push_str(std::str::from_utf8(&self.b[start..j]).ok()?);
                    self.i = j;
                }
            }
        }
    }

    fn unicode_escape(&mut self, s: &mut String) -> Option<()> {
        let hi = self.hex4()?;
        if (0xD800..=0xDBFF).contains(&hi) {
            if self.peek() == Some(b'\\') {
                self.i += 1;
                if self.peek() == Some(b'u') {
                    self.i += 1;
                    let lo = self.hex4()?;
                    if (0xDC00..=0xDFFF).contains(&lo) {
                        let cp = 0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00);
                        if let Some(ch) = char::from_u32(cp) {
                            s.push(ch);
                        }
                    }
                }
            }
        } else if let Some(ch) = char::from_u32(hi) {
            s.push(ch);
        }
        Some(())
    }

    fn hex4(&mut self) -> Option<u32> {
        let raw = self.b.get(self.i..self.i + 4)?;
        self.i += 4;
        u32::from_str_radix(std::str::from_utf8(raw).ok()?, 16).ok()
    }

    fn number(&mut self) -> Option<JsonVal> {
        let start = self.i;
        while self.i < self.b.len() {
            let c = self.b[self.i];
            if c.is_ascii_digit() || c == b'-' || c == b'+' || c == b'.' || c == b'e' || c == b'E' {
                self.i += 1;
            } else {
                break;
            }
        }
        if self.i == start {
            return None;
        }
        let t = std::str::from_utf8(&self.b[start..self.i]).ok()?;
        match t {
            "NaN" | "nan" => Some(JsonVal::Num(f64::NAN)),
            "Infinity" | "inf" => Some(JsonVal::Num(f64::INFINITY)),
            "-Infinity" | "-inf" => Some(JsonVal::Num(f64::NEG_INFINITY)),
            _ => t.parse::<f64>().ok().map(JsonVal::Num),
        }
    }
}

pub fn as_arr(v: &JsonVal) -> Option<&Vec<JsonVal>> {
    match v {
        JsonVal::Arr(a) => Some(a),
        _ => None,
    }
}

pub fn as_obj(v: &JsonVal) -> Option<&HashMap<String, JsonVal>> {
    match v {
        JsonVal::Obj(m) => Some(m),
        _ => None,
    }
}

pub fn get_str(o: &HashMap<String, JsonVal>, k: &str) -> Option<String> {
    match o.get(k) {
        Some(JsonVal::Str(s)) => Some(s.clone()),
        Some(JsonVal::Num(n)) => {
            if n.is_finite() {
                Some(format!("{}", n))
            } else {
                None
            }
        }
        _ => None,
    }
}
