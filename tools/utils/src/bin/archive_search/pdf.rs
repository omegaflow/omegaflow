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
    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

pub struct PdfImage {
    pub filter: String,
    pub width: u32,
    pub height: u32,
    pub colorspace: String,
    pub data: Vec<u8>,
}

pub fn pdf_images(bytes: &[u8]) -> Vec<PdfImage> {
    if !is_pdf(bytes) {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let Some(rel) = find(&bytes[i..], b"<<") else {
            break;
        };
        let ds = i + rel;
        let Some(de) = dict_end(bytes, ds) else {
            break;
        };
        let dict = &bytes[ds..de];
        if dict_names(dict, "Subtype").first().map(String::as_str) == Some("Image")
            && let Some((raw, next)) = stream_slice(bytes, de)
            && let Some(img) = decode_image(dict, raw)
        {
            out.push(img);
            i = next;
            continue;
        }
        i = de;
    }
    out
}

fn dict_end(data: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    let mut depth = 0i32;
    while i + 1 < data.len() {
        if data[i] == b'<' && data[i + 1] == b'<' {
            depth += 1;
            i += 2;
            continue;
        }
        if data[i] == b'>' && data[i + 1] == b'>' {
            depth -= 1;
            i += 2;
            if depth == 0 {
                return Some(i);
            }
            continue;
        }
        i += 1;
    }
    None
}

fn stream_slice(bytes: &[u8], after: usize) -> Option<(&[u8], usize)> {
    let mut i = after;
    while i < bytes.len() && is_ws(bytes[i]) {
        i += 1;
    }
    if bytes.get(i..i + b"stream".len()) != Some(b"stream") {
        return None;
    }
    i += b"stream".len();
    if bytes.get(i) == Some(&b'\r') {
        i += 1;
    }
    if bytes.get(i) == Some(&b'\n') {
        i += 1;
    }
    let erel = find(&bytes[i..], b"endstream")?;
    let end = i + erel;
    Some((&bytes[i..end], end + b"endstream".len()))
}

fn skip_ws(data: &[u8], mut i: usize) -> usize {
    while i < data.len() && is_ws(data[i]) {
        i += 1;
    }
    i
}

fn read_value(data: &[u8], start: usize) -> &[u8] {
    if start >= data.len() {
        return &[];
    }
    match data[start] {
        b'[' => {
            let mut depth = 0i32;
            let mut j = start;
            while j < data.len() {
                match data[j] {
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;
                        if depth == 0 {
                            return &data[start..=j];
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            &data[start..]
        }
        b'<' if data.get(start + 1) == Some(&b'<') => match dict_end(data, start) {
            Some(end) => &data[start..end],
            None => &data[start..],
        },
        b'/' => {
            let mut j = start + 1;
            while j < data.len()
                && !is_ws(data[j])
                && !matches!(data[j], b'/' | b'[' | b']' | b'<' | b'>' | b'(')
            {
                j += 1;
            }
            &data[start..j]
        }
        _ => {
            let mut j = start;
            while j < data.len()
                && !is_ws(data[j])
                && !matches!(data[j], b'/' | b'[' | b']' | b'<' | b'>' | b'(')
            {
                j += 1;
            }
            &data[start..j]
        }
    }
}

fn dict_value<'a>(dict: &'a [u8], key: &str) -> Option<&'a [u8]> {
    let kb = key.as_bytes();
    let mut i = 0;
    while i < dict.len() {
        if dict[i] != b'/' {
            i += 1;
            continue;
        }
        let ks = i + 1;
        let mut ke = ks;
        while ke < dict.len()
            && (dict[ke].is_ascii_alphanumeric() || dict[ke] == b'_' || dict[ke] == b'-')
        {
            ke += 1;
        }
        if &dict[ks..ke] == kb {
            return Some(read_value(dict, skip_ws(dict, ke)));
        }
        i = if ke > i { ke } else { i + 1 };
    }
    None
}

fn dict_int(dict: &[u8], key: &str) -> Option<u32> {
    let v = dict_value(dict, key)?;
    std::str::from_utf8(v).ok()?.trim().parse().ok()
}

fn dict_names(dict: &[u8], key: &str) -> Vec<String> {
    let Some(v) = dict_value(dict, key) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut i = 0;
    while i < v.len() {
        if v[i] == b'/' {
            let mut j = i + 1;
            while j < v.len()
                && !is_ws(v[j])
                && !matches!(v[j], b'/' | b'[' | b']' | b'<' | b'>')
            {
                j += 1;
            }
            out.push(String::from_utf8_lossy(&v[i + 1..j]).into_owned());
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

fn trim_eol(data: &[u8]) -> &[u8] {
    let mut end = data.len();
    while end > 0 && (data[end - 1] == b'\n' || data[end - 1] == b'\r') {
        end -= 1;
    }
    &data[..end]
}

fn decode_image(dict: &[u8], raw_full: &[u8]) -> Option<PdfImage> {
    let width = dict_int(dict, "Width")?;
    let height = dict_int(dict, "Height")?;
    if width == 0 || height == 0 {
        return None;
    }
    let colorspace = dict_names(dict, "ColorSpace").into_iter().next()?;
    let filter = dict_names(dict, "Filter").into_iter().next()?;
    let raw = match dict_int(dict, "Length") {
        Some(len) if (len as usize) <= raw_full.len() => &raw_full[..len as usize],
        _ => trim_eol(raw_full),
    };
    let data = match filter.as_str() {
        "DCTDecode" => {
            if raw.len() >= 4 && raw.starts_with(&[0xFF, 0xD8]) && raw.ends_with(&[0xFF, 0xD9]) {
                raw.to_vec()
            } else {
                return None;
            }
        }
        "JPXDecode" => {
            const JP2: [u8; 12] = [
                0x00, 0x00, 0x00, 0x0C, 0x6A, 0x50, 0x20, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
            ];
            if raw.starts_with(&JP2) {
                raw.to_vec()
            } else {
                return None;
            }
        }
        "FlateDecode" => {
            if dict_int(dict, "BitsPerComponent") != Some(8) {
                return None;
            }
            if let Some(predictor) = dict_int(dict, "Predictor")
                && predictor > 1
            {
                return None;
            }
            let color_type = match colorspace.as_str() {
                "DeviceGray" => 0u8,
                "DeviceRGB" => 2u8,
                _ => return None,
            };
            let decoded = if raw.len() >= 2 && (u16::from_be_bytes([raw[0], raw[1]]) % 31) == 0 {
                inflate(&raw[2..])?
            } else {
                inflate(raw)?
            };
            let channels = if color_type == 0 { 1usize } else { 3usize };
            let expected = width as usize * height as usize * channels;
            if decoded.len() != expected {
                return None;
            }
            png_wrap(width, height, color_type, &decoded)
        }
        _ => return None,
    };
    Some(PdfImage {
        filter,
        width,
        height,
        colorspace,
        data,
    })
}

fn png_wrap(width: u32, height: u32, color_type: u8, raster: &[u8]) -> Vec<u8> {
    let mut png = Vec::new();
    png.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, color_type, 0, 0, 0]);
    png_chunk(&mut png, b"IHDR", &ihdr);
    png_chunk(&mut png, b"IDAT", &zlib_stored(raster));
    png_chunk(&mut png, b"IEND", &[]);
    png
}

fn png_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let mut crc_in = Vec::with_capacity(4 + data.len());
    crc_in.extend_from_slice(kind);
    crc_in.extend_from_slice(data);
    out.extend_from_slice(&crc32(&crc_in).to_be_bytes());
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

fn adler32(data: &[u8]) -> u32 {
    let mut a = 1u32;
    let mut b = 0u32;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    if data.is_empty() {
        out.push(0x01);
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0xFFFFu16.to_le_bytes());
    } else {
        let mut chunks = data.chunks(65535).peekable();
        while let Some(chunk) = chunks.next() {
            let final_block = chunks.peek().is_none();
            out.push(if final_block { 0x01 } else { 0x00 });
            let len = chunk.len() as u16;
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(&(!len).to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
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
    (0..=haystack.len() - needle.len())
        .rev()
        .find(|&i| &haystack[i..i + needle.len()] == needle)
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
    use super::{pdf_images, pdf_text};

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
        let pdf = pdf_with_stream(
            "<< /Length 22 /Filter /ASCII85Decode >>",
            b"BT (Hello World) Tj ET",
        );
        assert_eq!(pdf_text(&pdf), None);
    }

    const JPEG_MIN: [u8; 6] = [0xFF, 0xD8, 0xFF, 0xE0, 0xFF, 0xD9];
    const FLATE_GRAY_1: [u8; 12] = [
        0x78, 0x01, 0x01, 0x01, 0x00, 0xFE, 0xFF, 0x41, 0x00, 0x42, 0x00, 0x42,
    ];

    fn be_u32(data: &[u8]) -> u32 {
        u32::from_be_bytes([data[0], data[1], data[2], data[3]])
    }

    #[test]
    fn lifts_embedded_jpeg() {
        let pdf = pdf_with_stream(
            "<< /Type /XObject /Subtype /Image /Width 2 /Height 2 /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode /Length 6 >>",
            &JPEG_MIN,
        );
        let images = pdf_images(&pdf);
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].filter, "DCTDecode");
        assert_eq!(images[0].width, 2);
        assert_eq!(images[0].height, 2);
        assert!(images[0].data.starts_with(&[0xFF, 0xD8]));
        assert!(images[0].data.ends_with(&[0xFF, 0xD9]));
    }

    #[test]
    fn wraps_flate_raster_in_png() {
        let pdf = pdf_with_stream(
            "<< /Type /XObject /Subtype /Image /Width 1 /Height 1 /ColorSpace /DeviceGray /BitsPerComponent 8 /Filter /FlateDecode /Length 12 >>",
            &FLATE_GRAY_1,
        );
        let images = pdf_images(&pdf);
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].filter, "FlateDecode");
        let png = &images[0].data;
        assert_eq!(&png[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
        assert_eq!(&png[12..16], b"IHDR");
        assert_eq!(be_u32(&png[16..20]), 1);
        assert_eq!(be_u32(&png[20..24]), 1);
        assert_eq!(png[24], 8);
        assert_eq!(png[25], 0);
    }

    #[test]
    fn pdf_without_image_lifts_nothing() {
        let pdf = pdf_with_stream("<< /Length 22 >>", b"BT (Hello World) Tj ET");
        assert!(pdf_images(&pdf).is_empty());
    }

    #[test]
    fn skips_unsupported_image_filter() {
        let pdf = pdf_with_stream(
            "<< /Type /XObject /Subtype /Image /Width 2 /Height 2 /ColorSpace /DeviceGray /BitsPerComponent 8 /Filter /LZWDecode /Length 3 >>",
            b"\x00\x01\x02",
        );
        assert!(pdf_images(&pdf).is_empty());
    }

    #[test]
    fn skips_image_missing_width() {
        let pdf = pdf_with_stream(
            "<< /Type /XObject /Subtype /Image /Height 2 /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode /Length 6 >>",
            &JPEG_MIN,
        );
        assert!(pdf_images(&pdf).is_empty());
    }

    #[test]
    fn skips_jpeg_without_end_marker() {
        let pdf = pdf_with_stream(
            "<< /Type /XObject /Subtype /Image /Width 2 /Height 2 /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode /Length 4 >>",
            &[0xFF, 0xD8, 0xFF, 0xE0],
        );
        assert!(pdf_images(&pdf).is_empty());
    }
}
