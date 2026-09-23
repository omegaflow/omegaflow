use std::collections::{HashMap, HashSet};

use omegaflow::inflate::inflate;

pub fn pdf_text(bytes: &[u8]) -> Option<String> {
    if !is_pdf(bytes) {
        return None;
    }
    let pdf = Pdf::parse(bytes);
    let mut page_nums: Vec<u32> = Vec::new();
    let mut seen: HashSet<u32> = HashSet::new();
    for &num in pdf.objects.keys() {
        let Some(dict) = pdf.object_dict(num) else {
            continue;
        };
        if dict_names(dict, "Type").iter().any(|n| n == "Catalog") {
            if let Some(pages_root) = dict_ref(dict, "Pages") {
                pdf.collect_pages(pages_root, &mut page_nums, &mut seen, 0);
            }
        }
    }
    for &num in pdf.objects.keys() {
        if seen.contains(&num) {
            continue;
        }
        let Some(dict) = pdf.object_dict(num) else {
            continue;
        };
        if dict_names(dict, "Type").iter().any(|n| n == "Page") {
            seen.insert(num);
            page_nums.push(num);
        }
    }
    let mut pages: Vec<String> = Vec::new();
    for &num in &page_nums {
        let Some(dict) = pdf.object_dict(num) else {
            continue;
        };
        if let Some(text) = extract_page(&pdf, dict) {
            pages.push(text);
        }
    }
    let joined = pages.join("\n");
    if !joined.is_empty() {
        return Some(joined);
    }
    if page_nums.is_empty() {
        return legacy_streams(bytes);
    }
    None
}

fn legacy_streams(bytes: &[u8]) -> Option<String> {
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
            inflate_zlib(raw)
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

struct Pdf<'a> {
    bytes: &'a [u8],
    objects: HashMap<u32, ObjectLoc>,
    objstms: HashMap<u32, Vec<u8>>,
    objstm_entries: HashMap<u32, Vec<(u32, usize)>>,
}

enum ObjectLoc {
    File { dict_pos: usize },
    ObjStm { owner: u32, index: usize },
}

impl<'a> Pdf<'a> {
    fn parse(bytes: &'a [u8]) -> Pdf<'a> {
        let mut objects: HashMap<u32, ObjectLoc> = HashMap::new();
        let mut i = 0usize;
        while i + 3 <= bytes.len() {
            if &bytes[i..i + 3] == b"obj" && (i == 0 || !bytes[i - 1].is_ascii_alphanumeric()) {
                let mut j = i;
                while j > 0 && is_ws(bytes[j - 1]) {
                    j -= 1;
                }
                let gen_end = j;
                while j > 0 && bytes[j - 1].is_ascii_digit() {
                    j -= 1;
                }
                if j == gen_end {
                    i += 3;
                    continue;
                }
                let mut k = j;
                while k > 0 && is_ws(bytes[k - 1]) {
                    k -= 1;
                }
                let num_end = k;
                while k > 0 && bytes[k - 1].is_ascii_digit() {
                    k -= 1;
                }
                if k == num_end {
                    i += 3;
                    continue;
                }
                if let Some(num) = std::str::from_utf8(&bytes[k..num_end])
                    .ok()
                    .and_then(|t| t.parse::<u32>().ok())
                {
                    objects
                        .entry(num)
                        .or_insert(ObjectLoc::File { dict_pos: i + 3 });
                }
                i += 3;
            } else {
                i += 1;
            }
        }
        let mut pdf = Pdf {
            bytes,
            objects,
            objstms: HashMap::new(),
            objstm_entries: HashMap::new(),
        };
        let nums: Vec<u32> = pdf.objects.keys().copied().collect();
        for num in nums {
            let Some(d) = pdf.object_dict(num) else {
                continue;
            };
            if !dict_names(d, "Type").iter().any(|n| n == "ObjStm") {
                continue;
            }
            let Some((dict, data)) = pdf.object_stream(num) else {
                continue;
            };
            let Some(decoded) = decode_stream(dict, data) else {
                continue;
            };
            let Some(n) = dict_int(dict, "N") else {
                continue;
            };
            let Some(first) = dict_int(dict, "First") else {
                continue;
            };
            pdf.register_objstm(num, n, first as usize, &decoded);
        }
        pdf
    }

    fn register_objstm(&mut self, owner: u32, n: u32, first: usize, data: &[u8]) {
        if first > data.len() {
            return;
        }
        let mut entries: Vec<(u32, usize)> = Vec::new();
        let mut i = 0usize;
        while i < first && entries.len() < n as usize {
            i = skip_ws(data, i);
            let Some((num, ni)) = parse_uint_at(data, i) else {
                break;
            };
            i = skip_ws(data, ni);
            let Some((off, _)) = parse_uint_at(data, i) else {
                break;
            };
            entries.push((num, first + off as usize));
        }
        for (idx, (num, _)) in entries.iter().enumerate() {
            self.objects
                .entry(*num)
                .or_insert(ObjectLoc::ObjStm { owner, index: idx });
        }
        self.objstm_entries.insert(owner, entries);
        self.objstms.insert(owner, data.to_vec());
    }

    fn object_slice(&self, num: u32) -> Option<&[u8]> {
        match self.objects.get(&num)? {
            ObjectLoc::File { dict_pos } => self.file_object_slice(*dict_pos),
            ObjectLoc::ObjStm { owner, index } => {
                let data = self.objstms.get(owner)?;
                let entries = self.objstm_entries.get(owner)?;
                let (_, off) = entries.get(*index)?;
                let end = entries.get(*index + 1).map(|e| e.1).unwrap_or(data.len());
                data.get(*off..end)
            }
        }
    }

    fn file_object_slice(&self, dict_pos: usize) -> Option<&[u8]> {
        let bytes = self.bytes;
        let s = skip_ws(bytes, dict_pos);
        if bytes.get(s..s + 2) != Some(b"<<") {
            let end = find(&bytes[dict_pos..], b"endobj")
                .map(|r| dict_pos + r)
                .unwrap_or(bytes.len());
            return Some(&bytes[dict_pos..end]);
        }
        let d_end = dict_end(bytes, s)?;
        let after = skip_ws(bytes, d_end);
        if bytes.get(after..after + 6) != Some(b"stream") {
            let end = find(&bytes[d_end..], b"endobj")
                .map(|r| d_end + r)
                .unwrap_or(bytes.len());
            return Some(&bytes[dict_pos..end]);
        }
        let mut ds = after + 6;
        if bytes.get(ds) == Some(&b'\r') {
            ds += 1;
        }
        if bytes.get(ds) == Some(&b'\n') {
            ds += 1;
        }
        let dict = &bytes[s..d_end];
        let end = match self.stream_length(dict, bytes.len().saturating_sub(ds)) {
            Some(l) => {
                let mut e = ds + l;
                if bytes.get(e) == Some(&b'\r') {
                    e += 1;
                }
                if bytes.get(e) == Some(&b'\n') {
                    e += 1;
                }
                if bytes.get(e..e + 9) == Some(b"endstream") {
                    e + 9
                } else {
                    ds + l
                }
            }
            None => find(&bytes[ds..], b"endstream")
                .map(|r| ds + r + 9)
                .unwrap_or(bytes.len()),
        };
        Some(&bytes[dict_pos..end])
    }

    fn object_dict(&self, num: u32) -> Option<&[u8]> {
        let slice = self.object_slice(num)?;
        let s = skip_ws(slice, 0);
        if slice.get(s..s + 2) != Some(b"<<") {
            return None;
        }
        let e = dict_end(slice, s)?;
        Some(&slice[s..e])
    }

    fn object_stream(&self, num: u32) -> Option<(&[u8], &[u8])> {
        let slice = self.object_slice(num)?;
        let s = skip_ws(slice, 0);
        if slice.get(s..s + 2) != Some(b"<<") {
            return None;
        }
        let d_end = dict_end(slice, s)?;
        let dict = &slice[s..d_end];
        let after = skip_ws(slice, d_end);
        if slice.get(after..after + 6) != Some(b"stream") {
            return None;
        }
        let mut ds = after + 6;
        if slice.get(ds) == Some(&b'\r') {
            ds += 1;
        }
        if slice.get(ds) == Some(&b'\n') {
            ds += 1;
        }
        let len = self.stream_length(dict, slice.len().saturating_sub(ds));
        let data = match len {
            Some(l) if ds + l <= slice.len() => &slice[ds..ds + l],
            _ => {
                let rel = find(&slice[ds..], b"endstream")?;
                &slice[ds..ds + rel]
            }
        };
        Some((dict, data))
    }

    fn stream_length(&self, dict: &[u8], max: usize) -> Option<usize> {
        if let Some(r) = dict_ref(dict, "Length") {
            if let Some(sl) = self.object_slice(r) {
                let s = skip_ws(sl, 0);
                if sl.get(s..s + 2) != Some(b"<<")
                    && let Some((n, _)) = parse_uint_at(&sl[s..], 0)
                {
                    return Some((n as usize).min(max));
                }
            }
            return None;
        }
        let v = dict_value(dict, "Length")?;
        let n = std::str::from_utf8(v)
            .ok()
            .and_then(|t| t.trim().parse::<u32>().ok())?;
        Some((n as usize).min(max))
    }

    fn page_fonts(&self, page_dict: &[u8]) -> HashMap<String, FontInfo> {
        let mut out = HashMap::new();
        let Some(res) = self.dict_or_ref(page_dict, "Resources") else {
            return out;
        };
        let Some(fonts_v) = dict_value(res, "Font") else {
            return out;
        };
        let Some(fonts) = dict_span(fonts_v) else {
            return out;
        };
        let mut i = 0usize;
        while i < fonts.len() {
            if fonts[i] != b'/' {
                i += 1;
                continue;
            }
            let s = i + 1;
            let mut j = s;
            while j < fonts.len() && (fonts[j].is_ascii_alphanumeric() || fonts[j] == b'_') {
                j += 1;
            }
            if j == s {
                i += 1;
                continue;
            }
            let name = String::from_utf8_lossy(&fonts[s..j]).into_owned();
            let vpos = skip_ws(fonts, j);
            let font_dict = if fonts.get(vpos..vpos + 2) == Some(b"<<") {
                dict_span(&fonts[vpos..])
            } else if let Some(r) = ref_at(fonts, vpos) {
                self.object_dict(r)
            } else {
                None
            };
            if let Some(fd) = font_dict {
                let cmap = self.font_to_unicode(fd);
                out.insert(
                    name,
                    FontInfo {
                        cmap,
                        kind: font_kind(fd),
                    },
                );
            }
            i = j;
        }
        out
    }

    fn collect_pages(&self, node: u32, out: &mut Vec<u32>, seen: &mut HashSet<u32>, depth: usize) {
        if depth > 64 || !seen.insert(node) {
            return;
        }
        let Some(d) = self.object_dict(node) else {
            return;
        };
        let types = dict_names(d, "Type");
        if !types.iter().any(|n| n == "Pages") {
            out.push(node);
            return;
        }
        let Some(kids) = dict_value(d, "Kids") else {
            return;
        };
        if !kids.starts_with(b"[") {
            return;
        }
        for k in refs_in_array(kids) {
            self.collect_pages(k, out, seen, depth + 1);
        }
    }

    fn dict_or_ref<'b>(&'a self, dict: &'b [u8], key: &str) -> Option<&'b [u8]>
    where
        'a: 'b,
    {
        let v = dict_value(dict, key)?;
        if v.starts_with(b"<<") {
            return dict_span(v);
        }
        let r = dict_ref(dict, key)?;
        self.object_dict(r)
    }

    fn font_to_unicode(&self, font_dict: &[u8]) -> Option<HashMap<Vec<u8>, Vec<u8>>> {
        let r = dict_ref(font_dict, "ToUnicode")?;
        let (dict, data) = self.object_stream(r)?;
        let decoded = decode_stream(dict, data)?;
        parse_cmap(&decoded)
    }
}

struct FontInfo {
    cmap: Option<HashMap<Vec<u8>, Vec<u8>>>,
    kind: FontKind,
}

enum FontKind {
    Cid,
    Simple(bool),
}

fn font_kind(font_dict: &[u8]) -> FontKind {
    let subtype = dict_names(font_dict, "Subtype").into_iter().next();
    match subtype.as_deref() {
        Some("Type0") => FontKind::Cid,
        Some("Type1") | Some("TrueType") | Some("MMType1") => {
            FontKind::Simple(simple_encoding_allows_ascii(font_dict))
        }
        _ => FontKind::Simple(false),
    }
}

fn simple_encoding_allows_ascii(font_dict: &[u8]) -> bool {
    if dict_value(font_dict, "Encoding").is_none() {
        return true;
    }
    let names = dict_names(font_dict, "Encoding");
    if names.iter().any(|n| n == "Differences") {
        return false;
    }
    names.iter().any(|n| {
        matches!(
            n.as_str(),
            "StandardEncoding"
                | "WinAnsiEncoding"
                | "MacRomanEncoding"
                | "MacExpertEncoding"
                | "PDFDocEncoding"
        )
    })
}

enum CmapDest {
    Start(Vec<u8>),
    List(Vec<Vec<u8>>),
}

fn extract_page(pdf: &Pdf, page_dict: &[u8]) -> Option<String> {
    let fonts = pdf.page_fonts(page_dict);
    let mut content: Vec<u8> = Vec::new();
    for r in contents_refs(page_dict) {
        let Some((dict, data)) = pdf.object_stream(r) else {
            continue;
        };
        let Some(decoded) = decode_stream(dict, data) else {
            continue;
        };
        content.extend_from_slice(&decoded);
    }
    let pieces = extract_content(&content, &fonts);
    if pieces.is_empty() {
        None
    } else {
        Some(pieces.join(" "))
    }
}

fn extract_content(content: &[u8], fonts: &HashMap<String, FontInfo>) -> Vec<String> {
    let default_font = FontInfo {
        cmap: None,
        kind: FontKind::Simple(true),
    };
    let mut pieces: Vec<String> = Vec::new();
    let mut i = 0usize;
    let mut in_text = false;
    let mut font: Option<&FontInfo> = None;
    while i < content.len() {
        match content[i] {
            b'(' => {
                let Some((s, ni)) = read_literal(content, i) else {
                    break;
                };
                let (op, ni2) = read_operator(content, ni);
                if op == b"Tj" || op == b"'" || op == b"\"" {
                    if in_text
                        && let Some(text) = decode_string(font.unwrap_or(&default_font), &s)
                        && plausible(&text)
                    {
                        pieces.push(text);
                    }
                }
                i = ni2;
            }
            b'[' => {
                let (strs, ni) = read_text_array(content, i);
                let (op, ni2) = read_operator(content, ni);
                if op == b"TJ" || op == b"Tj" {
                    if in_text {
                        for s in strs {
                            if let Some(text) = decode_string(font.unwrap_or(&default_font), &s)
                                && plausible(&text)
                            {
                                pieces.push(text);
                            }
                        }
                    }
                }
                i = ni2;
            }
            b'%' => {
                while i < content.len() && content[i] != b'\n' {
                    i += 1;
                }
            }
            b'<' => {
                if content.get(i + 1) == Some(&b'<') {
                    i = dict_end(content, i).unwrap_or(content.len());
                } else {
                    match hex_at(content, i) {
                        Some((bytes, ni)) => {
                            let (op, ni2) = read_operator(content, ni);
                            if op == b"Tj" || op == b"'" || op == b"\"" {
                                if in_text
                                    && let Some(text) =
                                        decode_string(font.unwrap_or(&default_font), &bytes)
                                    && plausible(&text)
                                {
                                    pieces.push(text);
                                }
                            }
                            i = ni2;
                        }
                        None => i = content.len(),
                    }
                }
            }
            b'/' => {
                let (name, ni) = read_name(content, i);
                let mut j = skip_ws(content, ni);
                while j < content.len()
                    && (content[j].is_ascii_digit()
                        || content[j] == b'.'
                        || content[j] == b'-'
                        || content[j] == b'+')
                {
                    j += 1;
                }
                let (op, ni2) = read_operator(content, skip_ws(content, j));
                if op == b"Tf" {
                    font = fonts.get(&name);
                    i = ni2;
                } else {
                    i = ni;
                }
            }
            _ => {
                if content[i].is_ascii_alphabetic() || content[i] == b'\'' || content[i] == b'"' {
                    let (op, ni) = read_operator(content, i);
                    if op == b"BT" {
                        in_text = true;
                    } else if op == b"ET" {
                        in_text = false;
                    }
                    i = ni;
                } else {
                    i += 1;
                }
            }
        }
    }
    pieces
}

fn extract_text(content: &[u8]) -> Option<String> {
    let fonts: HashMap<String, FontInfo> = HashMap::new();
    let pieces = extract_content(content, &fonts);
    if pieces.is_empty() {
        None
    } else {
        Some(pieces.join(" "))
    }
}

fn decode_string(font: &FontInfo, s: &[u8]) -> Option<String> {
    match font.kind {
        FontKind::Cid => {
            let cmap = font.cmap.as_ref()?;
            let bytes = cmap_decode(cmap, s)?;
            if let Some(text) = utf16be(&bytes) {
                return Some(text);
            }
            raw_text(&bytes)
        }
        FontKind::Simple(allow_ascii) => {
            if s.len() >= 2 && s[0] == 0xFE && s[1] == 0xFF {
                if let Some(text) = utf16be(&s[2..]) {
                    return Some(text);
                }
            }
            if let Some(cmap) = &font.cmap
                && let Some(bytes) = cmap_decode(cmap, s)
            {
                if let Some(text) = utf16be(&bytes) {
                    return Some(text);
                }
                if let Some(text) = raw_text(&bytes) {
                    return Some(text);
                }
            }
            if allow_ascii { raw_text(s) } else { None }
        }
    }
}

fn utf16be(data: &[u8]) -> Option<String> {
    if data.is_empty() || data.len() % 2 != 0 {
        return None;
    }
    let mut units = Vec::with_capacity(data.len() / 2);
    for pair in data.chunks(2) {
        let c = u16::from_be_bytes([pair[0], pair[1]]);
        if (0xD800..=0xDFFF).contains(&c) {
            return None;
        }
        units.push(c);
    }
    String::from_utf16(&units).ok()
}

fn raw_text(s: &[u8]) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    for &b in s {
        match b {
            0x20..=0x7e => out.push(b as char),
            b'\n' | b'\t' => out.push(b as char),
            _ => return None,
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn plausible(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    let mut bad = 0usize;
    let mut alpha = false;
    let mut total = 0usize;
    for c in trimmed.chars() {
        total += 1;
        if c == '\u{FFFD}' || (c.is_control() && c != '\n' && c != '\t') {
            bad += 1;
        } else if c.is_alphanumeric() {
            alpha = true;
        }
    }
    alpha && bad * 2 < total
}

fn cmap_decode(map: &HashMap<Vec<u8>, Vec<u8>>, s: &[u8]) -> Option<Vec<u8>> {
    let max_len = match map.keys().map(Vec::len).max() {
        Some(m) => m,
        None => return None,
    };
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < s.len() {
        let mut matched = false;
        for l in (1..=max_len.min(s.len() - i)).rev() {
            if let Some(dst) = map.get(&s[i..i + l]) {
                out.extend_from_slice(dst);
                i += l;
                matched = true;
                break;
            }
        }
        if !matched {
            return None;
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn parse_cmap(data: &[u8]) -> Option<HashMap<Vec<u8>, Vec<u8>>> {
    if find(data, b"begincmap").is_none() {
        return None;
    }
    let mut map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
    let mut i = 0usize;
    while i < data.len() {
        if !data[i].is_ascii_alphabetic() {
            i += 1;
            continue;
        }
        let (word, ni) = read_word(data, i);
        match word {
            "beginbfchar" => {
                let Some((count, _)) = prev_uint(data, i) else {
                    i = ni;
                    continue;
                };
                let mut pos = ni;
                for _ in 0..count {
                    let Some((src, s1)) = next_hex(data, pos) else {
                        break;
                    };
                    let Some((dst, s2)) = next_hex(data, s1) else {
                        break;
                    };
                    if !src.is_empty() && !dst.is_empty() {
                        map.insert(src, dst);
                    }
                    pos = s2;
                }
                i = pos;
            }
            "beginbfrange" => {
                let Some((count, _)) = prev_uint(data, i) else {
                    i = ni;
                    continue;
                };
                let mut pos = ni;
                for _ in 0..count {
                    let Some((lo, p1)) = next_hex(data, pos) else {
                        break;
                    };
                    let Some((hi, p2)) = next_hex(data, p1) else {
                        break;
                    };
                    if lo.len() != hi.len() {
                        pos = p2;
                        continue;
                    }
                    let Some((dst, p3)) = next_dest(data, p2) else {
                        break;
                    };
                    match dst {
                        CmapDest::Start(start) => {
                            if let (Some(lo_v), Some(hi_v), Some(st_v)) =
                                (bytes_to_u64(&lo), bytes_to_u64(&hi), bytes_to_u64(&start))
                            {
                                let count = hi_v.saturating_sub(lo_v).saturating_add(1);
                                for j in 0..count.min(65536) {
                                    if let (Some(code), Some(dest)) = (
                                        u64_to_bytes(lo_v + j, lo.len()),
                                        u64_to_bytes(st_v + j, start.len()),
                                    ) {
                                        map.insert(code, dest);
                                    }
                                }
                            }
                        }
                        CmapDest::List(list) => {
                            if let (Some(lo_v), Some(hi_v)) = (bytes_to_u64(&lo), bytes_to_u64(&hi))
                            {
                                let count = hi_v.saturating_sub(lo_v).saturating_add(1);
                                for (j, d) in list.iter().enumerate() {
                                    if (j as u64) >= count {
                                        break;
                                    }
                                    if let Some(code) = u64_to_bytes(lo_v + j as u64, lo.len()) {
                                        map.insert(code, d.clone());
                                    }
                                }
                            }
                        }
                    }
                    pos = p3;
                }
                i = pos;
            }
            _ => i = ni,
        }
    }
    if map.is_empty() { None } else { Some(map) }
}

fn read_word(data: &[u8], start: usize) -> (&str, usize) {
    let mut j = start;
    while j < data.len() && data[j].is_ascii_alphabetic() {
        j += 1;
    }
    (std::str::from_utf8(&data[start..j]).unwrap_or(""), j)
}

fn prev_uint(data: &[u8], start: usize) -> Option<(u32, usize)> {
    let mut j = start;
    while j > 0 && is_ws(data[j - 1]) {
        j -= 1;
    }
    let end = j;
    while j > 0 && data[j - 1].is_ascii_digit() {
        j -= 1;
    }
    if j == end {
        return None;
    }
    let v = std::str::from_utf8(&data[j..end]).ok()?.parse().ok()?;
    Some((v, j))
}

fn next_hex(data: &[u8], start: usize) -> Option<(Vec<u8>, usize)> {
    let mut i = start;
    while i < data.len() {
        if data[i] == b'<' {
            if data.get(i + 1) == Some(&b'<') {
                i = dict_end(data, i).unwrap_or(i + 2);
                continue;
            }
            return hex_at(data, i);
        }
        i += 1;
    }
    None
}

fn hex_at(data: &[u8], i: usize) -> Option<(Vec<u8>, usize)> {
    if data.get(i) != Some(&b'<') {
        return None;
    }
    let mut j = i + 1;
    while j < data.len() && data[j] != b'>' {
        j += 1;
    }
    if j >= data.len() {
        return None;
    }
    let mut bytes = Vec::new();
    let mut hi: Option<u8> = None;
    for &b in &data[i + 1..j] {
        if let Some(v) = hex_val(b) {
            match hi {
                None => hi = Some(v),
                Some(h) => {
                    bytes.push(h << 4 | v);
                    hi = None;
                }
            }
        }
    }
    if let Some(h) = hi {
        bytes.push(h << 4);
    }
    Some((bytes, j + 1))
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn next_dest(data: &[u8], start: usize) -> Option<(CmapDest, usize)> {
    let mut i = start;
    while i < data.len() {
        match data[i] {
            b'<' => {
                let (v, ni) = hex_at(data, i)?;
                return Some((CmapDest::Start(v), ni));
            }
            b'[' => {
                let mut list = Vec::new();
                let mut j = i + 1;
                while j < data.len() {
                    if data[j] == b'<' {
                        let (v, nj) = hex_at(data, j)?;
                        list.push(v);
                        j = nj;
                        continue;
                    }
                    if data[j] == b']' {
                        return Some((CmapDest::List(list), j + 1));
                    }
                    j += 1;
                }
                return None;
            }
            _ => i += 1,
        }
    }
    None
}

fn bytes_to_u64(b: &[u8]) -> Option<u64> {
    if b.is_empty() || b.len() > 8 {
        return None;
    }
    let mut v = 0u64;
    for &x in b {
        v = v << 8 | x as u64;
    }
    Some(v)
}

fn u64_to_bytes(v: u64, len: usize) -> Option<Vec<u8>> {
    if len == 0 || len > 8 {
        return None;
    }
    let mut out = Vec::with_capacity(len);
    for k in (0..len).rev() {
        out.push((v >> (8 * k)) as u8);
    }
    Some(out)
}

fn decode_stream(dict: &[u8], data: &[u8]) -> Option<Vec<u8>> {
    let filters = dict_names(dict, "Filter");
    if filters.is_empty() {
        return Some(data.to_vec());
    }
    let mut cur = data.to_vec();
    for filter in filters.iter().rev() {
        match filter.as_str() {
            "FlateDecode" => {
                let mut decoded = inflate_zlib(&cur)?;
                if let Some(p) = parms_int(dict, "Predictor")
                    && p > 1
                {
                    let colors = parms_int(dict, "Colors").unwrap_or(1);
                    let bpc = parms_int(dict, "BitsPerComponent").unwrap_or(8);
                    let columns = parms_int(dict, "Columns").unwrap_or(1);
                    decoded = apply_predictor(&decoded, p, colors, bpc, columns)?;
                }
                cur = decoded;
            }
            _ => return None,
        }
    }
    Some(cur)
}

fn inflate_zlib(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() >= 2 && (u16::from_be_bytes([data[0], data[1]]) % 31) == 0 {
        inflate(&data[2..])
    } else {
        inflate(data)
    }
}

fn parms_int(dict: &[u8], key: &str) -> Option<u32> {
    let v = dict_value(dict, "DecodeParms")?;
    let mut spans: Vec<&[u8]> = Vec::new();
    if v.starts_with(b"<<") {
        if let Some(e) = dict_end(v, 0) {
            spans.push(&v[..e]);
        }
    } else {
        let mut i = 0usize;
        while i + 1 < v.len() {
            if v.get(i..i + 2) == Some(b"<<") {
                if let Some(e) = dict_end(v, i) {
                    spans.push(&v[i..e]);
                    i = e;
                    continue;
                }
            }
            i += 1;
        }
    }
    for span in spans {
        if let Some(n) = dict_int(span, key) {
            return Some(n);
        }
    }
    None
}

fn apply_predictor(
    data: &[u8],
    predictor: u32,
    colors: u32,
    bpc: u32,
    columns: u32,
) -> Option<Vec<u8>> {
    match predictor {
        2 => tiff_predictor(data, colors, bpc, columns),
        10..=15 => png_predictor(data, colors, bpc, columns),
        _ => None,
    }
}

fn tiff_predictor(data: &[u8], colors: u32, bpc: u32, columns: u32) -> Option<Vec<u8>> {
    if bpc != 8 {
        return None;
    }
    let row = (colors * columns) as usize;
    if row == 0 || data.len() % row != 0 {
        return None;
    }
    let mut out = data.to_vec();
    for r in out.chunks_mut(row) {
        for j in 1..r.len() {
            r[j] = r[j].wrapping_add(r[j - 1]);
        }
    }
    Some(out)
}

fn png_predictor(data: &[u8], colors: u32, bpc: u32, columns: u32) -> Option<Vec<u8>> {
    if bpc != 8 && bpc != 16 {
        return None;
    }
    if colors == 0 || columns == 0 {
        return None;
    }
    let bits = (colors * bpc * columns) as usize;
    let row_len = (bits + 7) / 8;
    let bpp = (colors as usize * bpc as usize + 7) / 8;
    let stride = row_len + 1;
    if data.len() % stride != 0 {
        return None;
    }
    let rows = data.len() / stride;
    let mut out = Vec::with_capacity(rows * row_len);
    let mut prev = vec![0u8; row_len];
    for r in 0..rows {
        let ft = data[r * stride];
        let src = &data[r * stride + 1..(r + 1) * stride];
        let mut row = vec![0u8; row_len];
        for (j, &x) in src.iter().enumerate() {
            let a = if j >= bpp { row[j - bpp] as i32 } else { 0 };
            let b = prev[j] as i32;
            let c = if j >= bpp { prev[j - bpp] as i32 } else { 0 };
            let v = match ft {
                0 => x as i32,
                1 => x as i32 + a,
                2 => x as i32 + b,
                3 => x as i32 + (a + b) / 2,
                4 => x as i32 + paeth(a, b, c),
                _ => return None,
            };
            row[j] = (v & 0xFF) as u8;
        }
        out.extend_from_slice(&row);
        prev = row;
    }
    Some(out)
}

fn paeth(a: i32, b: i32, c: i32) -> i32 {
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

fn parse_uint_at(data: &[u8], start: usize) -> Option<(u32, usize)> {
    let mut j = start;
    while j < data.len() && data[j].is_ascii_digit() {
        j += 1;
    }
    if j == start {
        return None;
    }
    let v = std::str::from_utf8(&data[start..j]).ok()?.parse().ok()?;
    Some((v, j))
}

fn ref_at(data: &[u8], start: usize) -> Option<u32> {
    let (n, ni) = parse_uint_at(data, start)?;
    let j = skip_ws(data, ni);
    let (_, gj) = parse_uint_at(data, j)?;
    let k = skip_ws(data, gj);
    if data.get(k) == Some(&b'R') {
        Some(n)
    } else {
        None
    }
}

fn dict_ref(dict: &[u8], key: &str) -> Option<u32> {
    let kb = key.as_bytes();
    let mut i = 0usize;
    while i < dict.len() {
        if dict[i] != b'/' {
            i += 1;
            continue;
        }
        let ks = i + 1;
        let mut ke = ks;
        while ke < dict.len() && (dict[ke].is_ascii_alphanumeric() || dict[ke] == b'_') {
            ke += 1;
        }
        if &dict[ks..ke] == kb {
            return ref_at(dict, skip_ws(dict, ke));
        }
        i = if ke > i { ke } else { i + 1 };
    }
    None
}

fn refs_in_array(data: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < data.len() {
        if !data[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let s = i;
        while i < data.len() && data[i].is_ascii_digit() {
            i += 1;
        }
        let Some(n) = std::str::from_utf8(&data[s..i])
            .ok()
            .and_then(|t| t.parse::<u32>().ok())
        else {
            continue;
        };
        let Some((_, gj)) = parse_uint_at(data, skip_ws(data, i)) else {
            continue;
        };
        let k = skip_ws(data, gj);
        if data.get(k) == Some(&b'R') {
            out.push(n);
            i = k + 1;
        }
    }
    out
}

fn contents_refs(dict: &[u8]) -> Vec<u32> {
    let Some(v) = dict_value(dict, "Contents") else {
        return Vec::new();
    };
    if v.first() == Some(&b'[') {
        return refs_in_array(v);
    }
    if let Some(r) = dict_ref(dict, "Contents") {
        return vec![r];
    }
    Vec::new()
}

fn dict_span(data: &[u8]) -> Option<&[u8]> {
    if !data.starts_with(b"<<") {
        return None;
    }
    let e = dict_end(data, 0)?;
    Some(&data[..e])
}

fn read_name(data: &[u8], start: usize) -> (String, usize) {
    let mut j = start + 1;
    while j < data.len()
        && !is_ws(data[j])
        && !matches!(data[j], b'/' | b'[' | b']' | b'<' | b'>' | b'(' | b')')
    {
        j += 1;
    }
    (String::from_utf8_lossy(&data[start + 1..j]).into_owned(), j)
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
            while j < v.len() && !is_ws(v[j]) && !matches!(v[j], b'/' | b'[' | b']' | b'<' | b'>') {
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
    let channels = if color_type == 0 { 1usize } else { 3usize };
    let row = width as usize * channels;
    let mut filtered = Vec::with_capacity(raster.len() + height as usize);
    for scanline in raster.chunks(row) {
        filtered.push(0);
        filtered.extend_from_slice(scanline);
    }
    let mut png = Vec::new();
    png.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, color_type, 0, 0, 0]);
    png_chunk(&mut png, b"IHDR", &ihdr);
    png_chunk(&mut png, b"IDAT", &zlib_stored(&filtered));
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

fn read_text_array(data: &[u8], mut i: usize) -> (Vec<Vec<u8>>, usize) {
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
            b'<' => {
                if data.get(i + 1) == Some(&b'<') {
                    i = dict_end(data, i).unwrap_or(i + 2);
                    continue;
                }
                match hex_at(data, i) {
                    Some((s, ni)) => {
                        out.push(s);
                        i = ni;
                    }
                    None => break,
                }
            }
            _ => i += 1,
        }
    }
    (out, i)
}

#[cfg(test)]
mod tests {
    use super::{pdf_images, pdf_text, png_wrap, zlib_stored};
    use omegaflow::inflate::inflate;

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

    fn assemble_pdf(parts: &[&[u8]]) -> Vec<u8> {
        let mut pdf = Vec::new();
        pdf.extend_from_slice(b"%PDF-1.4\n");
        for p in parts {
            pdf.extend_from_slice(p);
        }
        pdf.extend_from_slice(b"%%EOF\n");
        pdf
    }

    fn obj(num: &str, dict: &str, body: &[u8]) -> Vec<u8> {
        let mut o = Vec::new();
        o.extend_from_slice(num.as_bytes());
        o.extend_from_slice(b" 0 obj\n");
        o.extend_from_slice(dict.as_bytes());
        o.extend_from_slice(b"\n");
        if body.is_empty() {
            o.extend_from_slice(b"endobj\n");
        } else {
            o.extend_from_slice(b"stream\n");
            o.extend_from_slice(body);
            o.extend_from_slice(b"\nendstream\nendobj\n");
        }
        o
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

    #[test]
    fn decodes_subset_font_via_tounicode_cmap() {
        let content = b"BT /F1 12 Tf (\x01\x02) Tj ET";
        let cmap = b"/CIDInit /ProcSet findresource begin\nbegincmap\n1 begincodespacerange\n<00> <FF>\nendcodespacerange\n2 beginbfchar\n<01> <0041>\n<02> <0042>\nendbfchar\nendcmap";
        let content_dict = format!("<< /Length {} >>", content.len());
        let cmap_dict = format!("<< /Length {} >>", cmap.len());
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 2 0 R /Resources << /Font << /F1 3 0 R >> >> >>",
                &[],
            ),
            &obj("2", &content_dict, content),
            &obj(
                "3",
                "<< /Type /Font /Subtype /Type0 /ToUnicode 4 0 R >>",
                &[],
            ),
            &obj("4", &cmap_dict, cmap),
        ]);
        assert_eq!(pdf_text(&pdf), Some("AB".to_string()));
    }

    #[test]
    fn decodes_bfrange_and_destination_array() {
        let content = b"BT /F1 12 Tf (\x01\x02\x03\x04\x05) Tj ET";
        let cmap = b"/CIDInit /ProcSet findresource begin\nbegincmap\n1 begincodespacerange\n<00> <FF>\nendcodespacerange\n1 beginbfchar\n<01> <0041>\nendbfchar\n2 beginbfrange\n<02> <03> <0042>\n<04> <05> [<0043> <0044>]\nendbfrange\nendcmap";
        let content_dict = format!("<< /Length {} >>", content.len());
        let cmap_dict = format!("<< /Length {} >>", cmap.len());
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 2 0 R /Resources << /Font << /F1 3 0 R >> >> >>",
                &[],
            ),
            &obj("2", &content_dict, content),
            &obj(
                "3",
                "<< /Type /Font /Subtype /Type0 /ToUnicode 4 0 R >>",
                &[],
            ),
            &obj("4", &cmap_dict, cmap),
        ]);
        assert_eq!(pdf_text(&pdf), Some("ABCCD".to_string()));
    }

    #[test]
    fn resolves_content_ref_inside_object_stream() {
        let contained = b"<< /Length 14 >>\nstream\nBT (Hi) Tj ET\nendstream";
        let mut body = Vec::new();
        body.extend_from_slice(b"9 0 ");
        body.extend_from_slice(contained);
        let stm = zlib_stored(&body);
        let objstm_dict = format!(
            "<< /Type /ObjStm /N 1 /First 4 /Filter /FlateDecode /Length {} >>",
            stm.len()
        );
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 9 0 R /Resources << >> >>",
                &[],
            ),
            &obj("5", &objstm_dict, &stm),
        ]);
        assert_eq!(pdf_text(&pdf), Some("Hi".to_string()));
    }

    #[test]
    fn decodes_flate_content_with_filter_array() {
        let content = zlib_stored(b"BT (Flate Array) Tj ET");
        let content_dict = format!("<< /Length {} /Filter [/FlateDecode] >>", content.len());
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 2 0 R /Resources << >> >>",
                &[],
            ),
            &obj("2", &content_dict, &content),
        ]);
        assert_eq!(pdf_text(&pdf), Some("Flate Array".to_string()));
    }

    #[test]
    fn image_only_page_yields_no_text() {
        let draw = b"q 12 0 0 12 0 0 cm /Im1 Do Q";
        let draw_dict = format!("<< /Length {} >>", draw.len());
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 2 0 R /Resources << /XObject << /Im1 3 0 R >> >> >>",
                &[],
            ),
            &obj("2", &draw_dict, draw),
            &obj(
                "3",
                "<< /Type /XObject /Subtype /Image /Width 1 /Height 1 /ColorSpace /DeviceGray /BitsPerComponent 8 /Filter /DCTDecode /Length 6 >>",
                &JPEG_MIN,
            ),
        ]);
        assert_eq!(pdf_text(&pdf), None);
    }

    #[test]
    fn binary_noise_content_yields_no_text() {
        let noise = b"BT (\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0b\x0c\x0d\x0e\x0f) Tj ET";
        let noise_dict = format!("<< /Length {} >>", noise.len());
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 2 0 R /Resources << >> >>",
                &[],
            ),
            &obj("2", &noise_dict, noise),
        ]);
        assert_eq!(pdf_text(&pdf), None);
    }

    #[test]
    fn type0_identity_h_without_tounicode_yields_none() {
        let content = b"BT /F1 12 Tf (GL) Tj ET";
        let content_dict = format!("<< /Length {} >>", content.len());
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 2 0 R /Resources << /Font << /F1 3 0 R >> >> >>",
                &[],
            ),
            &obj("2", &content_dict, content),
            &obj(
                "3",
                "<< /Type /Font /Subtype /Type0 /Encoding /Identity-H >>",
                &[],
            ),
        ]);
        assert_eq!(pdf_text(&pdf), None);
    }

    #[test]
    fn walks_page_tree_through_pages_kids() {
        let content = b"BT (Tree Text) Tj ET";
        let content_dict = format!("<< /Length {} >>", content.len());
        let pdf = assemble_pdf(&[
            &obj("1", "<< /Type /Catalog /Pages 2 0 R >>", &[]),
            &obj("2", "<< /Type /Pages /Kids [3 0 R] /Count 1 >>", &[]),
            &obj(
                "3",
                "<< /Parent 2 0 R /Contents 4 0 R /Resources << >> >>",
                &[],
            ),
            &obj("4", &content_dict, content),
        ]);
        assert_eq!(pdf_text(&pdf), Some("Tree Text".to_string()));
    }

    #[test]
    fn type1_winansi_without_tounicode_decodes_ascii() {
        let content = b"BT /F1 12 Tf (WinAnsi Text) Tj ET";
        let content_dict = format!("<< /Length {} >>", content.len());
        let pdf = assemble_pdf(&[
            &obj(
                "1",
                "<< /Type /Page /Contents 2 0 R /Resources << /Font << /F1 3 0 R >> >> >>",
                &[],
            ),
            &obj("2", &content_dict, content),
            &obj(
                "3",
                "<< /Type /Font /Subtype /Type1 /Encoding /WinAnsiEncoding >>",
                &[],
            ),
        ]);
        assert_eq!(pdf_text(&pdf), Some("WinAnsi Text".to_string()));
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
        assert_eq!(
            &png[0..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
        );
        assert_eq!(&png[12..16], b"IHDR");
        assert_eq!(be_u32(&png[16..20]), 1);
        assert_eq!(be_u32(&png[20..24]), 1);
        assert_eq!(png[24], 8);
        assert_eq!(png[25], 0);
    }

    #[test]
    fn png_idat_carries_one_filter_byte_per_scanline() {
        let raster = [0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66];
        let png = png_wrap(2, 1, 2, &raster);
        let mut idat = Vec::new();
        let mut off = 8usize;
        while off + 8 <= png.len() {
            let len = be_u32(&png[off..off + 4]) as usize;
            if &png[off + 4..off + 8] == b"IDAT" {
                idat.extend_from_slice(&png[off + 8..off + 8 + len]);
            }
            off += 12 + len;
        }
        let filtered = inflate(&idat[2..]).expect("zlib stream");
        assert_eq!(filtered, vec![0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
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
