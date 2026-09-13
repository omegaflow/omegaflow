use omegaflow::inflate::inflate;

pub fn pdf_text(bytes: &[u8]) -> Option<String> {
    if !is_pdf(bytes) {
        return None;
    }
    let mut pages: Vec<String> = Vec::new();
    let mut search = 0usize;
    while search < bytes.len() {
        let Some(rel) = find(&bytes[search..], b"stream") else {
            break;
        };
        let kw = search + rel;
        if kw > 0 && bytes[kw - 1].is_ascii_alphanumeric() {
            search = kw + b"stream".len();
            continue;
        }
        let mut start = kw + b"stream".len();
        if bytes.get(start) == Some(&b'\r') {
            start += 1;
        }
        if bytes.get(start) == Some(&b'\n') {
            start += 1;
        }
        let Some(erel) = find(&bytes[start..], b"endstream") else {
            break;
        };
        let end = start + erel;
        let raw = &bytes[start..end];

        let dict_start = rfind(&bytes[..kw], b"<<").unwrap_or(kw);
        let dict = &bytes[dict_start..kw];
        let flate = find(dict, b"FlateDecode").is_some();
        let filtered = find(dict, b"/Filter").is_some();

        let decoded = if flate {
            if raw.len() >= 2 && (u16::from_be_bytes([raw[0], raw[1]]) % 31) == 0 {
                inflate(&raw[2..])
            } else {
                inflate(raw)
            }
        } else if filtered {
            None
        } else {
            Some(raw.to_vec())
        };

        if let Some(content) = decoded
            && let Some(text) = extract_text(&content)
        {
            pages.push(text);
        }
        search = end + b"endstream".len();
    }
    let joined = pages.join("\n");
    if joined.is_empty() { None } else { Some(joined) }
}

fn is_pdf(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(1024)];
    find(head, b"%PDF").is_some()
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).find(|&i| &haystack[i..i + needle.len()] == needle)
}

fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).rev().find(|&i| &haystack[i..i + needle.len()] == needle)
}

fn is_ws(b: u8) -> bool {
    matches!(b, 0x00 | 0x09 | 0x0a | 0x0c | 0x0d | 0x20)
}

fn read_operator(data: &[u8], mut i: usize) -> (&[u8], usize) {
    while i < data.len() && is_ws(data[i]) {
        i += 1;
    }
    let start = i;
    if i < data.len() && (data[i] == b'\'' || data[i] == b'"') {
        return (&data[i..i + 1], i + 1);
    }
    while i < data.len() && data[i].is_ascii_alphabetic() {
        i += 1;
    }
    (&data[start..i], i)
}

fn read_literal(data: &[u8], start: usize) -> Option<(Vec<u8>, usize)> {
    let mut i = start + 1;
    let mut depth = 1usize;
    let mut out = Vec::new();
    while i < data.len() {
        match data[i] {
            b'\\' => {
                i += 1;
                let e = *data.get(i)?;
                match e {
                    b'n' => {
                        out.push(b'\n');
                        i += 1;
                    }
                    b'r' => {
                        out.push(b'\r');
                        i += 1;
                    }
                    b't' => {
                        out.push(b'\t');
                        i += 1;
                    }
                    b'b' => {
                        out.push(0x08);
                        i += 1;
                    }
                    b'f' => {
                        out.push(0x0c);
                        i += 1;
                    }
                    b'(' => {
                        out.push(b'(');
                        i += 1;
                    }
                    b')' => {
                        out.push(b')');
                        i += 1;
                    }
                    b'\\' => {
                        out.push(b'\\');
                        i += 1;
                    }
                    b'\r' => {
                        i += 1;
                        if data.get(i) == Some(&b'\n') {
                            i += 1;
                        }
                    }
                    b'\n' => {
                        i += 1;
                    }
                    b'0'..=b'7' => {
                        let mut val = 0u32;
                        let mut k = 0;
                        while k < 3 {
                            match data.get(i).copied() {
                                Some(c @ b'0'..=b'7') => {
                                    val = val * 8 + (c - b'0') as u32;
                                    i += 1;
                                    k += 1;
                                }
                                _ => break,
                            }
                        }
                        out.push(val as u8);
                    }
                    other => {
                        out.push(other);
                        i += 1;
                    }
                }
            }
            b'(' => {
                depth += 1;
                out.push(b'(');
                i += 1;
            }
            b')' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some((out, i));
                }
                out.push(b')');
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    None
}

fn read_array(data: &[u8], mut i: usize) -> (Vec<Vec<u8>>, usize) {
    i += 1;
    let mut out = Vec::new();
    while i < data.len() {
        match data[i] {
            b']' => {
                i += 1;
                break;
            }
            b'(' => match read_literal(data, i) {
                Some((s, ni)) => {
                    out.push(s);
                    i = ni;
                }
                None => break,
            },
            _ => i += 1,
        }
    }
    (out, i)
}

fn extract_text(content: &[u8]) -> Option<String> {
    let mut pieces: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < content.len() {
        match content[i] {
            b'(' => {
                let Some((s, ni)) = read_literal(content, i) else {
                    break;
                };
                let (op, _) = read_operator(content, ni);
                if op == b"Tj" || op == b"'" || op == b"\"" {
                    pieces.push(String::from_utf8_lossy(&s).into_owned());
                }
                i = ni;
            }
            b'[' => {
                let (strs, ni) = read_array(content, i);
                let (op, _) = read_operator(content, ni);
                if op == b"TJ" || op == b"Tj" {
                    for s in strs {
                        pieces.push(String::from_utf8_lossy(&s).into_owned());
                    }
                }
                i = ni;
            }
            b'%' => {
                while i < content.len() && content[i] != b'\n' {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    let text = pieces.join(" ");
    if text.is_empty() { None } else { Some(text) }
}

#[cfg(test)]
mod tests {
    use super::pdf_text;

    const FLATE_HELLO: [u8; 33] = [
        0x78, 0x01, 0x01, 0x16, 0x00, 0xe9, 0xff, b'B', b'T', b' ', b'(', b'H', b'e', b'l', b'l',
        b'o', b' ', b'W', b'o', b'r', b'l', b'd', b')', b' ', b'T', b'j', b' ', b'E', b'T', 0x4d,
        0x4b, 0x06, 0xbb,
    ];

    fn pdf_with_stream(dict: &str, body: &[u8]) -> Vec<u8> {
        let mut pdf = Vec::new();
        pdf.extend_from_slice(b"%PDF-1.4\n");
        pdf.extend_from_slice(b"1 0 obj\n");
        pdf.extend_from_slice(dict.as_bytes());
        pdf.extend_from_slice(b"\nstream\n");
        pdf.extend_from_slice(body);
        pdf.extend_from_slice(b"\nendstream\nendobj\n%%EOF\n");
        pdf
    }

    #[test]
    fn reads_uncompressed_text() {
        let pdf = pdf_with_stream("<< /Length 22 >>", b"BT (Hello World) Tj ET");
        assert_eq!(pdf_text(&pdf), Some("Hello World".to_string()));
    }

    #[test]
    fn reads_escaped_text() {
        let pdf = pdf_with_stream("<< /Length 16 >>", b"BT (He\\(llo\\)) Tj ET");
        assert_eq!(pdf_text(&pdf), Some("He(llo)".to_string()));
    }

    #[test]
    fn reads_flate_decoded_text() {
        let pdf = pdf_with_stream("<< /Length 33 /Filter /FlateDecode >>", &FLATE_HELLO);
        assert_eq!(pdf_text(&pdf), Some("Hello World".to_string()));
    }

    #[test]
    fn reads_array_text() {
        let pdf = pdf_with_stream("<< /Length 20 >>", b"BT [(Hel)(lo)] TJ ET");
        assert_eq!(pdf_text(&pdf), Some("Hel lo".to_string()));
    }

    #[test]
    fn rejects_non_pdf() {
        assert_eq!(pdf_text(b"not a pdf at all"), None);
    }

    #[test]
    fn rejects_pdf_without_text_operator() {
        let pdf = pdf_with_stream("<< /Length 5 >>", b"BT ET");
        assert_eq!(pdf_text(&pdf), None);
    }

    #[test]
    fn skips_unknown_filter() {
        let pdf = pdf_with_stream("<< /Length 22 /Filter /ASCII85Decode >>", b"BT (Hello World) Tj ET");
        assert_eq!(pdf_text(&pdf), None);
    }
}
